//! From a score to audio, one phrase at a time. A phrase is a run of notes
//! between rests; each renders on its own, so an edit re-renders only the
//! phrases it touches, and a host can render ahead of the playhead while
//! earlier phrases already play. Every phrase has a key that changes exactly
//! when its sound would.
use crate::bank::Voicebank;
use crate::g2p::{self, Lyric, NoteText, en::Dictionary};
use crate::phoneme::Phoneme;
use crate::pitch::{self, PitchNote};
use crate::score::{Param, Score, Timeline};
use crate::spectrum::FRAME_SECONDS;
use crate::timing::{self, Role, SungNote};
use crate::vocoder::{Pulse, Vocoder};
use crate::{concat, formant, params, utau};
use std::sync::Arc;

/// Rests at least this long split phrases.
pub const PHRASE_GAP: f64 = 0.35;
/// Audio before the first note (breath and consonants) and after the last.
pub const LEAD: f64 = 0.6;
pub const TAIL: f64 = 0.3;
/// The synthetic voice's period and level jitter (`vocoder::Pulse`): about
/// a singer's own, ±0.4 % in period.
const JITTER: f32 = 0.3;

#[derive(Clone, Debug)]
pub enum Voice {
    /// The built-in formant voice.
    Formant,
    Bank(Arc<Voicebank>),
}
impl Voice {
    fn consonant_length(&self, p: Phoneme) -> f64 {
        match self {
            // Sung consonants are short; a synthetic one is all hiss.
            Voice::Formant => utau::default_duration(p) as f64 * 0.7,
            Voice::Bank(bank) => {
                bank.duration(bank.substitute(p))
                    .unwrap_or_else(|| utau::default_duration(p)) as f64
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Phrase {
    pub key: u64,
    /// Song seconds where the phrase's audio starts and ends.
    pub start: f64,
    pub end: f64,
    /// Indices into the score's notes.
    pub notes: Vec<usize>,
    texts: Vec<NoteText>,
    /// A breath may precede it.
    breath: bool,
}

struct Fnv(u64);
impl Fnv {
    fn new() -> Self {
        Self(0xcbf2_9ce4_8422_2325)
    }
    fn bytes(&mut self, data: &[u8]) {
        for &b in data {
            self.0 ^= b as u64;
            self.0 = self.0.wrapping_mul(0x0100_0000_01b3);
        }
    }
    fn json(&mut self, value: &impl serde::Serialize) {
        self.bytes(&serde_json::to_vec(value).unwrap_or_default());
    }
}

/// Splits the score into phrases. Overlapping notes are cut at the next
/// note's start: a voice sings one note at a time.
pub fn plan(
    score: &Score,
    timeline: &Timeline,
    voice_id: &str,
    rate: u32,
    dictionary: Option<&Dictionary>,
) -> Vec<Phrase> {
    let notes = &score.notes;
    if notes.is_empty() {
        return Vec::new();
    }
    let default = score.default_lyric();
    let lyrics: Vec<Lyric> = notes
        .iter()
        .map(|n| Lyric {
            text: &n.lyric,
            phonemes: n.phonemes.as_deref(),
            language: n.language.unwrap_or(score.language),
        })
        .collect();
    let texts = g2p::assign(&lyrics, default, dictionary);
    let seconds: Vec<(f64, f64)> = (0..notes.len())
        .map(|i| {
            let start = timeline.seconds(notes[i].start as f64);
            let mut end = timeline.seconds(notes[i].end() as f64);
            if let Some(next) = notes.get(i + 1) {
                end = end
                    .min(timeline.seconds(next.start as f64))
                    .max(start + 0.01);
            }
            (start, end)
        })
        .collect();
    // A looping score's first phrase in the loop follows the last one of
    // the pass before, and may start before the loop does: its lead-in
    // sounds at that pass's end.
    let looping = (score.loop_ticks > 0).then(|| {
        (
            timeline.seconds(score.loop_start as f64),
            timeline.seconds(score.loop_ticks as f64),
        )
    });
    let wrapped_end = looping.and_then(|(a, b)| seconds.last().map(|s| s.1 - (b - a)));
    let mut phrases: Vec<Phrase> = Vec::new();
    let mut current: Vec<usize> = Vec::new();
    let flush = |current: &mut Vec<usize>, phrases: &mut Vec<Phrase>| {
        if current.is_empty() {
            return;
        }
        let first = current[0];
        let last = *current.last().unwrap();
        let previous_end = phrases
            .last()
            .map(|p| p.notes.last().map_or(0.0, |&n| seconds[n].1))
            .or(wrapped_end);
        let breath = previous_end.is_none_or(|e| seconds[first].0 - e >= 0.5);
        // A first note's consonants may come before 0: playback from the
        // start begins early enough for them (`Song::lead_in`).
        let earliest = match looping {
            Some((a, b)) if seconds[first].0 >= a => a - (b - a),
            _ => f64::NEG_INFINITY,
        };
        let start = (seconds[first].0 - LEAD)
            .max(earliest)
            .min(seconds[first].0);
        let end = seconds[last].1 + TAIL;
        let mut hash = Fnv::new();
        hash.json(&(
            voice_id,
            rate,
            score.transpose,
            &score.style,
            score.language,
        ));
        hash.json(&(start, end, breath));
        for &n in current.iter() {
            hash.json(&notes[n]);
            hash.json(&seconds[n]);
            hash.bytes(format!("{:?}", texts[n]).as_bytes());
        }
        // Curve points inside the phrase, plus the ones bracketing it.
        let (t0, t1) = (timeline.tick(start), timeline.tick(end));
        for (param, curve) in &score.curves {
            let a = curve
                .partition_point(|p| (p.tick as f64) < t0)
                .saturating_sub(1);
            let b = (curve.partition_point(|p| (p.tick as f64) <= t1) + 1).min(curve.len());
            hash.json(&(param, &curve[a..b]));
        }
        phrases.push(Phrase {
            // 53 bits: exact in JavaScript numbers.
            key: hash.0 & ((1 << 53) - 1),
            start,
            end,
            notes: current.clone(),
            texts: current.iter().map(|&n| texts[n].clone()).collect(),
            breath,
        });
        current.clear();
    };
    for i in 0..notes.len() {
        if let Some(&last) = current.last() {
            // A melisma or a "+" syllable never starts a new phrase.
            let continues = matches!(texts[i], NoteText::Hold);
            // The loop's first note starts a phrase of its own.
            let enters_loop =
                looping.is_some_and(|(a, _)| seconds[last].0 < a && seconds[i].0 >= a);
            if (seconds[i].0 - seconds[last].1 >= PHRASE_GAP && !continues) || enters_loop {
                flush(&mut current, &mut phrases);
            }
        }
        current.push(i);
    }
    flush(&mut current, &mut phrases);
    phrases
}

/// A phrase laid out for synthesis: its notes' times (seconds from the
/// phrase start), the phonemes' timing and the pitch per frame.
struct Layout<'a> {
    notes: Vec<&'a crate::score::Note>,
    times: Vec<(f64, f64)>,
    segments: Vec<crate::timing::Segment>,
    contour: Vec<f32>,
}

fn layout<'a>(score: &'a Score, timeline: &Timeline, phrase: &Phrase, voice: &Voice) -> Layout<'a> {
    let origin = phrase.start;
    let total = phrase.end - phrase.start;
    let frames = (total / FRAME_SECONDS as f64).ceil() as usize + 1;
    let notes: Vec<&crate::score::Note> = phrase.notes.iter().map(|&i| &score.notes[i]).collect();
    let times: Vec<(f64, f64)> = phrase
        .notes
        .iter()
        .enumerate()
        .map(|(k, &i)| {
            let n = &score.notes[i];
            let start = timeline.seconds(n.start as f64) - origin;
            let mut end = timeline.seconds(n.end() as f64) - origin;
            if let Some(&next) = phrase.notes.get(k + 1) {
                end = end.min(timeline.seconds(score.notes[next].start as f64) - origin);
            }
            (start, end.max(start + 0.01))
        })
        .collect();
    let sung: Vec<SungNote> = notes
        .iter()
        .zip(&times)
        .zip(&phrase.texts)
        .map(|((n, &(start, end)), text)| SungNote {
            start,
            end,
            text: text.clone(),
            velocity: n.velocity,
        })
        .collect();
    let style = &score.style;
    let breath = (phrase.breath && style.auto_breath && style.breath_level > 0.0)
        .then_some(0.22 + 0.2 * style.breath_level.clamp(0.0, 1.0) as f64);
    let segments = timing::layout(&sung, &|p| voice.consonant_length(p), breath, total);

    let pitch_notes: Vec<PitchNote> = notes
        .iter()
        .zip(&times)
        .map(|(n, &(start, end))| PitchNote {
            start,
            end,
            key: n.key as f32 + score.transpose as f32,
            vibrato_depth: n.vibrato.depth.unwrap_or(style.vibrato_depth),
            vibrato_rate: n.vibrato.rate.unwrap_or(style.vibrato_rate),
            vibrato_delay: n.vibrato.delay.unwrap_or(style.vibrato_delay),
            attack: n.attack,
            release: n.release,
        })
        .collect();
    let tick_at = |t: f64| timeline.tick(t + origin) as f32;
    let bend = |t: f64| {
        let tick = tick_at(t);
        score.param(Param::PitchBend, tick) / 8192.0
            * score.param(Param::PitchBendSensitivity, tick)
            * 100.0
    };
    let portamento = |t: f64| score.param(Param::PortamentoTiming, tick_at(t));
    let contour = pitch::contour(&pitch_notes, frames, style, &bend, &portamento, phrase.key);
    Layout {
        notes,
        times,
        segments,
        contour,
    }
}

/// For each note of the phrase (as score indices): the voicebank units its
/// syllable is joined from and others that could sing it. Empty for the
/// built-in voice.
pub fn units(
    score: &Score,
    timeline: &Timeline,
    phrase: &Phrase,
    voice: &Voice,
) -> Vec<(usize, concat::NoteUnits)> {
    let Voice::Bank(bank) = voice else {
        return Vec::new();
    };
    let l = layout(score, timeline, phrase, voice);
    let prefer: Vec<&str> = l.notes.iter().map(|n| n.unit.as_str()).collect();
    let found = concat::note_units(bank, &l.segments, &l.contour, &prefer, l.notes.len());
    phrase.notes.iter().copied().zip(found).collect()
}

/// Renders one phrase at `rate`; the samples start at `phrase.start`.
pub fn render(
    score: &Score,
    timeline: &Timeline,
    phrase: &Phrase,
    voice: &Voice,
    rate: u32,
) -> Vec<f32> {
    let origin = phrase.start;
    let total = phrase.end - phrase.start;
    let Layout {
        notes,
        times,
        segments,
        contour,
    } = layout(score, timeline, phrase, voice);
    let style = &score.style;
    let tick_at = |t: f64| timeline.tick(t + origin) as f32;
    let mut voice_frames = match voice {
        Voice::Formant => formant::frames(&segments, &contour),
        Voice::Bank(bank) => {
            let prefer: Vec<&str> = notes.iter().map(|n| n.unit.as_str()).collect();
            concat::frames(bank, &segments, &contour, 0.25, &prefer)
        }
    };
    // Parameters, note effects and pulse controls per frame.
    let mut pulses = vec![Pulse::default(); voice_frames.len()];
    let mut note_at = 0usize;
    let mut seg_at = 0usize;
    for (i, (frame, pulse)) in voice_frames.iter_mut().zip(pulses.iter_mut()).enumerate() {
        let t = i as f64 * FRAME_SECONDS as f64;
        while note_at + 1 < times.len() && t >= times[note_at + 1].0 - 0.05 {
            note_at += 1;
        }
        while seg_at + 1 < segments.len() && t >= segments[seg_at].end {
            seg_at += 1;
        }
        let mut values = params::Values::get(score, tick_at(t));
        let n = notes[note_at];
        let (start, end) = times[note_at];
        let (db, breath_add, bright) = params::effects(
            (n.attack.kind, n.attack.amount),
            (n.release.kind, n.release.amount),
            t - start,
            end - t,
        );
        values.breathiness = (values.breathiness + breath_add).min(127.0);
        values.brightness = (values.brightness + bright).min(127.0);
        // Pauses keep the voice's own breath; parameters shape the singing.
        if segments[seg_at].role != Role::Pause {
            *pulse = params::apply(frame, &values, style.robot);
            if matches!(voice, Voice::Formant) && !style.robot {
                // Recordings carry their own period and level wander; the
                // synthetic voice needs it added.
                pulse.jitter = pulse.jitter.max(JITTER);
            }
            if db != 0.0 {
                for e in frame.env.iter_mut() {
                    *e += db;
                }
            }
        }
    }
    let mut vocoder = Vocoder::new(rate);
    let mut samples = vocoder.render(&voice_frames, &pulses, phrase.key);
    let len = ((total * rate as f64).round() as usize).min(samples.len());
    samples.truncate(len);
    // Short fades at the phrase edges.
    let fade = (0.005 * rate as f64) as usize;
    let n = samples.len();
    for k in 0..fade.min(n / 2) {
        let g = k as f32 / fade as f32;
        samples[k] *= g;
        samples[n - 1 - k] *= g;
    }
    samples
}

/// Renders the whole score into one buffer from 0 s (for files and tests).
pub fn render_song(
    score: &Score,
    voice: &Voice,
    voice_id: &str,
    rate: u32,
    dictionary: Option<&Dictionary>,
) -> Vec<f32> {
    let timeline = score.timeline(None);
    let phrases = plan(score, &timeline, voice_id, rate, dictionary);
    let end = phrases.iter().map(|p| p.end).fold(0.0, f64::max);
    let mut out = vec![0.0f32; (end * rate as f64).ceil() as usize + 1];
    for phrase in &phrases {
        let audio = render(score, &timeline, phrase, voice, rate);
        // A looping score's lead-in before 0 has no pass before it here.
        let offset = (phrase.start * rate as f64).round() as i64;
        let skip = (-offset).max(0) as usize;
        for (o, s) in out[offset.max(0) as usize..]
            .iter_mut()
            .zip(audio.iter().skip(skip))
        {
            *o += s;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::phoneme::Language;
    use crate::score::{Note, TICKS_PER_BEAT};

    fn song(language: Language, lyrics: &[&str]) -> Score {
        let mut score = Score {
            language,
            ..Default::default()
        };
        for (i, lyric) in lyrics.iter().enumerate() {
            score.notes.push(Note {
                id: i as u32 + 1,
                start: TICKS_PER_BEAT * (2 + i as u32),
                length: TICKS_PER_BEAT,
                key: 60 + (i as u8 % 5),
                lyric: lyric.to_string(),
                ..Default::default()
            });
        }
        score
    }

    #[test]
    fn phrases_split_at_rests_and_keys_follow_edits() {
        let mut score = song(Language::Ja, &["さ", "く", "ら", "さ"]);
        score.notes[3].start += TICKS_PER_BEAT * 2;
        let timeline = score.timeline(None);
        let a = plan(&score, &timeline, "", 48_000, None);
        assert_eq!(a.len(), 2);
        assert_eq!(a[0].notes, [0, 1, 2]);
        score.notes[3].lyric = "ら".into();
        let b = plan(&score, &timeline, "", 48_000, None);
        assert_eq!(a[0].key, b[0].key);
        assert_ne!(a[1].key, b[1].key);
        // Host tempo moves everything.
        let host = [crate::score::Tempo { tick: 0, bpm: 90.0 }];
        let c = plan(&score, &score.timeline(Some(&host)), "", 48_000, None);
        assert_ne!(a[0].key, c[0].key);
    }

    #[test]
    fn a_first_note_sings_its_consonant_before_zero() {
        let mut score = song(Language::Ru, &["ла", "ла"]);
        score.notes[0].start = 0;
        let timeline = score.timeline(None);
        let once = plan(&score, &timeline, "", 48_000, None);
        assert!(once[0].start < -0.3, "{}", once[0].start);
        score.loop_ticks = TICKS_PER_BEAT * 8;
        let looped = plan(&score, &timeline, "", 48_000, None);
        assert!(looped[0].start < -0.3, "{}", looped[0].start);
        let audio = render(&score, &timeline, &looped[0], &Voice::Formant, 48_000);
        let before: f32 = audio[..(-looped[0].start * 48_000.0) as usize]
            .iter()
            .map(|s| s * s)
            .sum();
        assert!(before > 1e-3, "the «л» sounds before the beat: {before}");
    }

    #[test]
    fn the_loop_start_begins_a_phrase_and_its_lead_in_may_reach_back() {
        let mut score = song(Language::Ru, &["ла", "ла", "ла"]);
        // Notes at beats 2, 3 and 4; the loop goes from beat 3 to beat 6.
        score.loop_start = TICKS_PER_BEAT * 3;
        score.loop_ticks = TICKS_PER_BEAT * 6;
        let timeline = score.timeline(None);
        let phrases = plan(&score, &timeline, "", 48_000, None);
        assert_eq!(phrases.len(), 2, "the intro note sings apart");
        assert_eq!(phrases[1].notes, [1, 2]);
        assert!(phrases[1].start < timeline.seconds(score.loop_start as f64));
    }

    #[test]
    fn the_built_in_voice_sings_every_language() {
        let rate = 48_000;
        for (language, lyrics) in [
            (Language::Ja, vec!["き", "ら", "き", "ら"]),
            (Language::En, vec!["twin-", "kle", "lit-", "tle"]),
            (Language::Ru, vec!["зве", "зда", "го", "рит"]),
        ] {
            let score = song(language, &lyrics);
            let audio = render_song(&score, &Voice::Formant, "", rate, None);
            assert!(audio.iter().all(|s| s.is_finite()));
            // Voice where the notes are, silence well before them.
            let rms = |a: f64, b: f64| {
                let s = &audio[(a * rate as f64) as usize..(b * rate as f64) as usize];
                (s.iter().map(|x| x * x).sum::<f32>() / s.len() as f32).sqrt()
            };
            let singing = rms(1.1, 2.9);
            assert!(singing > 0.02 && singing < 0.5, "{language:?}: {singing}");
            assert!(rms(0.0, 0.3) < 1e-4);
            let peak = audio.iter().fold(0.0f32, |m, s| m.max(s.abs()));
            assert!(peak < 1.0, "{language:?} clips: {peak}");
        }
    }
}
