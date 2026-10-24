//! Places phonemes in time the way singers and VOCALOID do: the vowel of each
//! syllable starts exactly on its note, the consonants before it are sung
//! ahead of the beat, and a syllable's final consonants close the note. When
//! notes are too short, consonants shrink before the vowels do.
use crate::g2p::NoteText;
use crate::phoneme::{Class, Phoneme, Syllable};

#[derive(Clone, Debug)]
pub struct SungNote {
    /// Seconds from the phrase origin.
    pub start: f64,
    pub end: f64,
    pub text: NoteText,
    /// VEL, 0–127.
    pub velocity: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    Pause,
    Onset,
    Nucleus,
    Coda,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Segment {
    pub phoneme: Phoneme,
    pub start: f64,
    pub end: f64,
    pub role: Role,
    /// The note whose syllable this is.
    pub note: Option<usize>,
}
impl Segment {
    pub fn length(&self) -> f64 {
        self.end - self.start
    }
}

/// VEL 64 keeps the natural length; 127 halves it, 0 doubles it.
pub fn velocity_scale(velocity: u8) -> f64 {
    2f64.powf((64.0 - velocity as f64) / 64.0)
}

struct Slot {
    syllable: Syllable,
    note: usize,
    /// Where the vowel starts.
    anchor: f64,
    /// Where the syllable's time ends (note end, or the next syllable in the note).
    end: f64,
    scale: f64,
}

/// Lays out one phrase. `length(p)` is the natural length of a consonant in
/// seconds; `breath` the length of an auto breath before the phrase, if any.
/// The result covers `0..total` without gaps.
pub fn layout(
    notes: &[SungNote],
    length: &dyn Fn(Phoneme) -> f64,
    breath: Option<f64>,
    total: f64,
) -> Vec<Segment> {
    let mut slots: Vec<Slot> = Vec::new();
    for (i, note) in notes.iter().enumerate() {
        match &note.text {
            NoteText::Syllables(list) if !list.is_empty() => {
                let part = (note.end - note.start) / list.len() as f64;
                for (k, syllable) in list.iter().enumerate() {
                    slots.push(Slot {
                        syllable: syllable.clone(),
                        note: i,
                        anchor: note.start + part * k as f64,
                        end: note.start + part * (k + 1) as f64,
                        scale: velocity_scale(note.velocity),
                    });
                }
            }
            _ => {
                if let Some(last) = slots.last_mut() {
                    last.end = last.end.max(note.end);
                }
            }
        }
    }
    let mut out: Vec<Segment> = Vec::new();
    let push = |out: &mut Vec<Segment>, phoneme, start: f64, end: f64, role, note| {
        if end > start + 1e-6 {
            out.push(Segment {
                phoneme,
                start,
                end,
                role,
                note,
            });
        }
    };
    if slots.is_empty() {
        push(&mut out, Phoneme::SIL, 0.0, total, Role::Pause, None);
        return out;
    }
    let lengths = |list: &[Phoneme], scale: f64| -> Vec<f64> {
        list.iter()
            .map(|&p| {
                let natural = length(p);
                // Closures and pauses keep their time; consonants follow VEL.
                if matches!(p.class(), Class::Closure) {
                    natural
                } else {
                    natural * scale
                }
            })
            .collect()
    };

    // Onset of the first syllable, with an optional breath before it.
    let first = &slots[0];
    let mut onset = lengths(first.syllable.onset(), first.scale);
    let available = first.anchor.max(0.0);
    let wanted: f64 = onset.iter().sum();
    if wanted > available {
        let f = available / wanted.max(1e-9);
        onset.iter_mut().for_each(|l| *l *= f);
    }
    let mut cursor = first.anchor - onset.iter().sum::<f64>();
    if let Some(b) = breath {
        let end = cursor - 0.03;
        let start = (end - b).max(0.0);
        if end - start > 0.08 {
            push(&mut out, Phoneme::SIL, 0.0, start, Role::Pause, None);
            push(&mut out, Phoneme::BREATH, start, end, Role::Pause, None);
            push(&mut out, Phoneme::SIL, end, cursor, Role::Pause, None);
        } else {
            push(&mut out, Phoneme::SIL, 0.0, cursor, Role::Pause, None);
        }
    } else {
        push(&mut out, Phoneme::SIL, 0.0, cursor, Role::Pause, None);
    }
    for (&p, l) in first.syllable.onset().iter().zip(&onset) {
        push(
            &mut out,
            p,
            cursor,
            cursor + l,
            Role::Onset,
            Some(first.note),
        );
        cursor += l;
    }

    for s in 0..slots.len() {
        let slot = &slots[s];
        let vowel = slot.syllable.vowel().unwrap_or(Phoneme::SIL);
        let mut coda = lengths(slot.syllable.coda(), slot.scale);
        let next = slots.get(s + 1);
        let mut next_onset = next.map_or(Vec::new(), |n| lengths(n.syllable.onset(), n.scale));
        let next_anchor = next.map_or(slot.end, |n| n.anchor);
        let rest = next.is_none() || next_anchor - slot.end >= 0.03;
        // The vowel keeps at least this much of its syllable.
        let minimum = (0.3 * (slot.end - slot.anchor))
            .max(0.04)
            .min(slot.end - slot.anchor);
        let (coda_end, onset_start);
        if rest {
            let coda_len: f64 = coda.iter().sum();
            let room = (slot.end - slot.anchor - minimum).max(0.0);
            if coda_len > room {
                let f = (room / coda_len.max(1e-9)).max(0.35);
                coda.iter_mut().for_each(|l| *l *= f);
            }
            let onset_len: f64 = next_onset.iter().sum();
            let earliest = slot.end.min(next_anchor);
            let room = next_anchor - slot.anchor - minimum - coda.iter().sum::<f64>();
            if onset_len > room.max(0.0) && next.is_some() {
                let f = (room.max(0.0) / onset_len.max(1e-9)).max(0.35);
                next_onset.iter_mut().for_each(|l| *l *= f);
            }
            onset_start = next_anchor - next_onset.iter().sum::<f64>();
            coda_end = earliest.min(onset_start);
        } else {
            // Legato: the coda and the next onset share the time before the next vowel.
            let block: f64 = coda.iter().sum::<f64>() + next_onset.iter().sum::<f64>();
            let room = (next_anchor - slot.anchor - minimum).max(0.0);
            if block > room {
                let f = (room / block.max(1e-9)).max(0.35);
                coda.iter_mut().for_each(|l| *l *= f);
                next_onset.iter_mut().for_each(|l| *l *= f);
            }
            onset_start = next_anchor - next_onset.iter().sum::<f64>();
            coda_end = onset_start;
        }
        let coda_start = (coda_end - coda.iter().sum::<f64>()).max(cursor);
        push(
            &mut out,
            vowel,
            cursor,
            coda_start,
            Role::Nucleus,
            Some(slot.note),
        );
        cursor = coda_start;
        for (&p, l) in slot.syllable.coda().iter().zip(&coda) {
            let end = (cursor + l).min(coda_end.max(cursor));
            push(&mut out, p, cursor, end, Role::Coda, Some(slot.note));
            cursor = end;
        }
        if let Some(next) = next {
            let onset_start = onset_start.max(cursor);
            push(
                &mut out,
                Phoneme::SIL,
                cursor,
                onset_start,
                Role::Pause,
                None,
            );
            cursor = onset_start;
            let mut t = cursor;
            for (&p, l) in next.syllable.onset().iter().zip(&next_onset) {
                let end = (t + l).min(next.anchor.max(t));
                push(&mut out, p, t, end, Role::Onset, Some(next.note));
                t = end;
            }
            cursor = t.max(cursor);
        }
    }
    push(
        &mut out,
        Phoneme::SIL,
        cursor,
        total.max(cursor),
        Role::Pause,
        None,
    );
    // Merge neighbouring pauses of the same kind.
    let mut merged: Vec<Segment> = Vec::with_capacity(out.len());
    for seg in out {
        if let Some(last) = merged.last_mut()
            && last.phoneme == seg.phoneme
            && last.role == Role::Pause
            && seg.role == Role::Pause
        {
            last.end = seg.end;
            continue;
        }
        merged.push(seg);
    }
    release_stops(merged)
}

/// How long a released stop opens into its vowel, seconds.
const RELEASE: f64 = 0.035;

/// A Russian stop right before a nasal releases through the nose and is
/// barely heard (the «д» of «звёздный»).
/// Sung for clarity, it opens into a short vowel first: «ы», or «и» next to
/// a soft consonant. The time comes from the stop, down to 20 ms, then from
/// the sound after it.
fn release_stops(mut segments: Vec<Segment>) -> Vec<Segment> {
    let ru = crate::phoneme::Language::Ru;
    let mut i = 0;
    while i + 1 < segments.len() {
        let (stop, next) = (segments[i], segments[i + 1]);
        let muffled = next.phoneme.class() == Class::Nasal;
        let soft = stop.phoneme.info().soft || next.phoneme.info().soft;
        let vowel = if soft { "i" } else { "y" };
        let vowel = Phoneme::parse(ru, vowel).filter(|_| {
            stop.phoneme.language() == Some(ru) && stop.phoneme.class() == Class::Stop && muffled
        });
        if let Some(vowel) = vowel {
            let from_stop = (stop.length() - 0.02).clamp(0.0, RELEASE);
            let from_next = (RELEASE - from_stop).min((next.length() - 0.03).max(0.0));
            let start = stop.end - from_stop;
            let end = stop.end + from_next;
            if end - start >= 0.015 {
                segments[i].end = start;
                segments[i + 1].start = end;
                segments.insert(
                    i + 1,
                    Segment {
                        phoneme: vowel,
                        start,
                        end,
                        ..stop
                    },
                );
                i += 1;
            }
        }
        i += 1;
    }
    segments
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::g2p::{self, Lyric};
    use crate::phoneme::Language;

    fn notes(lyrics: &[&str], times: &[(f64, f64)]) -> Vec<SungNote> {
        let lyrics: Vec<Lyric> = lyrics
            .iter()
            .map(|t| Lyric {
                text: t,
                phonemes: None,
                language: Language::Ja,
            })
            .collect();
        g2p::assign(&lyrics, "ら", None)
            .into_iter()
            .zip(times)
            .map(|(text, &(start, end))| SungNote {
                start,
                end,
                text,
                velocity: 64,
            })
            .collect()
    }
    fn fixed(_: Phoneme) -> f64 {
        0.08
    }
    fn show(segments: &[Segment]) -> String {
        segments
            .iter()
            .map(|s| format!("{}@{:.2}", s.phoneme.symbol(), s.start))
            .collect::<Vec<_>>()
            .join(" ")
    }

    #[test]
    fn russian_stops_open_before_a_nasal() {
        let ru = |t: &'static str| Lyric {
            text: t,
            phonemes: None,
            language: Language::Ru,
        };
        let sung: Vec<SungNote> = g2p::assign(&[ru("звёзд-"), ru("ный")], "ла", None)
            .into_iter()
            .zip([(0.5, 1.0), (1.0, 1.5)])
            .map(|(text, (start, end))| SungNote {
                start,
                end,
                text,
                velocity: 64,
            })
            .collect();
        let s = show(&layout(&sung, &fixed, None, 2.0));
        assert!(
            s.contains("d@0.") && s.contains(" y@0.") && s.contains(" n@"),
            "{s}"
        );
        let d = s.find("d@").unwrap();
        assert!(
            s[d..].starts_with("d@") && s[d..].split(' ').nth(1).unwrap().starts_with("y@"),
            "{s}"
        );
    }

    #[test]
    fn vowels_start_on_the_beat() {
        let n = notes(&["さ", "く", "ら"], &[(0.5, 1.0), (1.0, 1.5), (1.5, 2.0)]);
        let s = layout(&n, &fixed, None, 2.5);
        assert_eq!(
            show(&s),
            "sil@0.00 s@0.42 a@0.50 k@0.92 u@1.00 r@1.42 a@1.50 sil@2.00"
        );
        // Contiguous cover of the whole phrase.
        assert_eq!(s[0].start, 0.0);
        assert_eq!(s.last().unwrap().end, 2.5);
        for w in s.windows(2) {
            assert!((w[0].end - w[1].start).abs() < 1e-9);
        }
    }

    #[test]
    fn rests_and_codas_and_breath() {
        let n = notes(&["かっ", "ん"], &[(0.6, 1.0), (1.4, 1.8)]);
        let s = layout(&n, &fixed, Some(0.3), 2.2);
        assert_eq!(
            show(&s),
            "sil@0.00 br@0.19 sil@0.49 k@0.52 a@0.60 cl@0.92 sil@1.00 N@1.40 sil@1.80"
        );
    }

    #[test]
    fn short_notes_shrink_consonants() {
        let n = notes(&["さ", "し"], &[(0.5, 0.6), (0.6, 0.7)]);
        let s = layout(&n, &|_| 0.2, None, 1.0);
        let vowel = s.iter().find(|x| x.phoneme.symbol() == "a").unwrap();
        assert!(vowel.length() >= 0.03, "{}", show(&s));
        let sh = s.iter().find(|x| x.phoneme.symbol() == "sh").unwrap();
        assert!(sh.length() < 0.1);
    }

    #[test]
    fn melisma_extends_the_vowel() {
        let n = notes(&["あ", "-", "い"], &[(0.2, 0.5), (0.5, 0.8), (0.8, 1.0)]);
        let s = layout(&n, &fixed, None, 1.2);
        assert_eq!(show(&s), "sil@0.00 a@0.20 i@0.80 sil@1.00");
    }
}
