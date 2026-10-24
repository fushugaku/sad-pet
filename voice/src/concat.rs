//! Concatenative synthesis from a voicebank, VOCALOID-style. A dynamic
//! program picks the units that cover the phrase's phonemes with the fewest
//! and least audible joins, preferring units recorded near the sung pitch.
//! Units overlap on a shared phoneme and cross-fade there in the spectral
//! domain, so a join is a smooth change of envelope rather than a splice.
//! Vowels stretch over their steady part (looping it back and forth for long
//! notes); consonants keep their recorded speed.
use crate::bank::{Unit, Voicebank};
use crate::phoneme::{Class, Phoneme};
use crate::spectrum::{ENV_POINTS, FLOOR_DB, FRAME_SECONDS, Frame, grid_position};
use crate::timing::Segment;

const FRAME: f64 = FRAME_SECONDS as f64;
/// How incomplete a candidate's last consonant is, 0 (as long as the voice
/// sings it) to 1 (only the start of it): Kumi's VC units end a stop or an
/// affricate just after its closure, so «ч» from «ач» before «к» is silent.
fn stub(bank: &Voicebank, c: &Candidate) -> f32 {
    let Some(u) = c.unit else { return 0.0 };
    let unit = &bank.units[u as usize];
    let k = c.first + (c.b - c.a);
    let p = unit.phonemes[k];
    if p.is_vowel() || p.is_pause() {
        return 0.0;
    }
    let (a, b) = unit.span(k);
    let span = (b - a) as f32 * FRAME_SECONDS;
    let full = bank
        .duration(p)
        .unwrap_or_else(|| crate::utau::default_duration(p));
    (1.0 - span / (0.6 * full)).clamp(0.0, 1.0)
}

/// Cross-fade half-width at a join between two phonemes (a hard join).
const EDGE: f64 = 0.012;
/// Candidates kept per phrase range, by their own cost.
const KEEP: usize = 6;

/// How far a vowel's middle moves toward the voice's typical shape of it
/// (0 keeps each take as recorded). Rebuilding recordings of three UTAU
/// banks from the rest of their bank, 0.7 cut the envelope error most (5.1
/// to 4.3 dB) but blurred the formants: their contrast fell from 8.4 to
/// 7.3 dB in Kumi, under the recordings' own. 0.3 keeps it (7.9 dB) and still
/// cuts the error to 4.7 dB.
const MEAN_PULL: f32 = 0.3;

/// One phoneme of the phrase as synthesis sees it.
#[derive(Clone, Debug)]
struct Item {
    /// The bank phoneme matched against units; pauses become `sil`.
    phoneme: Phoneme,
    start: f64,
    end: f64,
    pause: bool,
    /// The phrase's note whose syllable this is.
    note: Option<usize>,
}

/// The units a note's syllable is joined from, and others that could sing it.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize)]
pub struct NoteUnits {
    pub used: Vec<String>,
    pub options: Vec<String>,
}

/// A note's chosen unit outweighs every other way to sing its syllable.
const PREFERRED: f32 = 4.0;

#[derive(Clone, Copy, Debug)]
struct Candidate {
    unit: Option<u32>,
    /// First unit phoneme used.
    first: usize,
    /// Phrase range a..=b.
    a: usize,
    b: usize,
    cost: f32,
}

fn items(bank: &Voicebank, segments: &[Segment]) -> Vec<Item> {
    let mut out: Vec<Item> = Vec::new();
    for s in segments {
        let pause = s.phoneme.is_pause() || s.phoneme.class() == Class::Closure;
        let phoneme = if pause {
            Phoneme::SIL
        } else {
            bank.substitute(s.phoneme)
        };
        if pause && out.last().is_some_and(|i| i.pause) {
            out.last_mut().unwrap().end = s.end;
            continue;
        }
        out.push(Item {
            phoneme,
            start: s.start,
            end: s.end,
            pause,
            note: s.note,
        });
    }
    out
}

fn target_pitch(pitch: &[f32], start: f64, end: f64) -> f32 {
    let a = ((start / FRAME) as usize).min(pitch.len().saturating_sub(1));
    let b = ((end / FRAME) as usize).clamp(a + 1, pitch.len().max(a + 1));
    let slice = &pitch[a..b.min(pitch.len())];
    if slice.is_empty() {
        return 60.0;
    }
    let mut v = slice.to_vec();
    v.sort_by(f32::total_cmp);
    v[v.len() / 2]
}

/// `prefer[n]`: the unit alias the phrase's note `n` asks for ("" for any).
fn candidates(bank: &Voicebank, items: &[Item], pitch: &[f32], prefer: &[&str]) -> Vec<Candidate> {
    let mut out = Vec::new();
    // Phoneme → (unit, index) occurrences.
    let mut occurrences: std::collections::HashMap<Phoneme, Vec<(u32, usize)>> =
        std::collections::HashMap::new();
    for (u, unit) in bank.units.iter().enumerate() {
        for (k, &p) in unit.phonemes.iter().enumerate() {
            occurrences.entry(p).or_default().push((u as u32, k));
        }
    }
    for a in 0..items.len() {
        if items[a].pause {
            // Plain silence: a unit with a recorded onset or ending is better.
            out.push(Candidate {
                unit: None,
                first: 0,
                a,
                b: a,
                cost: 0.6,
            });
        }
        let mut by_range: std::collections::HashMap<usize, Vec<(Candidate, f32)>> =
            std::collections::HashMap::new();
        for &(u, k) in occurrences
            .get(&items[a].phoneme)
            .map_or(&[][..], Vec::as_slice)
        {
            let unit = &bank.units[u as usize];
            let mut len = 0;
            while k + len < unit.phonemes.len()
                && a + len < items.len()
                && unit.phonemes[k + len] == items[a + len].phoneme
            {
                len += 1;
            }
            // Every prefix of the match is a candidate.
            for l in 1..=len {
                let b = a + l - 1;
                let target_left = a.checked_sub(1).map_or(Phoneme::SIL, |i| items[i].phoneme);
                let target_right = items.get(b + 1).map_or(Phoneme::SIL, |i| i.phoneme);
                let cut_left = k
                    .checked_sub(1)
                    .map_or(0.0, |i| cut_cost(unit.phonemes[i], target_left));
                let cut_right = unit
                    .phonemes
                    .get(k + l)
                    .map_or(0.0, |&p| cut_cost(p, target_right));
                // A unit of only pauses adds nothing.
                if unit.phonemes[k..k + l].iter().all(|p| p.is_pause()) {
                    continue;
                }
                let main = (k..k + l)
                    .rev()
                    .find(|&i| unit.phonemes[i].is_vowel())
                    .unwrap_or(k + l - 1);
                let target =
                    target_pitch(pitch, items[a + main - k].start, items[a + main - k].end);
                let semitones = (unit.pitch - target).abs();
                let unwanted = items[a..=b].iter().any(|item| {
                    item.note
                        .and_then(|n| prefer.get(n))
                        .is_some_and(|alias| !alias.is_empty() && *alias != unit.alias)
                });
                let cost = 0.3
                    + cut_left
                    + cut_right
                    + if unit.variant { 0.5 } else { 0.0 }
                    + if unwanted { PREFERRED } else { 0.0 };
                by_range.entry(b).or_default().push((
                    Candidate {
                        unit: Some(u),
                        first: k,
                        a,
                        b,
                        cost,
                    },
                    semitones,
                ));
            }
        }
        for (_, mut list) in by_range {
            // Pitch only ranks alternatives: a voice recorded at one pitch
            // must not make every unit look worse than plain silence.
            let nearest = list.iter().map(|c| c.1).fold(f32::INFINITY, f32::min);
            for (c, semitones) in &mut list {
                let off = *semitones - nearest;
                c.cost += 0.06 * (off - 1.0).max(0.0) + 0.08 * (off - 6.0).max(0.0);
            }
            list.sort_by(|x, y| x.0.cost.total_cmp(&y.0.cost));
            list.truncate(KEEP);
            out.extend(list.into_iter().map(|(c, _)| c));
        }
    }
    out
}

/// Mean difference (dB) of two envelopes' shapes over 100 Hz – 5 kHz. Their
/// levels are matched first: placement evens out loudness, so only a change
/// of timbre makes a join audible.
fn spectral_distance(x: &Frame, y: &Frame) -> f32 {
    let lo = grid_position(100.0) as usize;
    let hi = grid_position(5000.0) as usize;
    let count = (hi - lo + 1) as f32;
    let offset = (lo..=hi).map(|g| x.env[g] - y.env[g]).sum::<f32>() / count;
    (lo..=hi)
        .map(|g| (x.env[g] - y.env[g] - offset).abs())
        .sum::<f32>()
        / count
}

fn middle_frame(bank: &Voicebank, unit: &Unit, k: usize) -> Frame {
    let (a, b) = unit.span(k);
    bank.frame(((a + b) / 2) as usize)
}

fn join_cost(bank: &Voicebank, items: &[Item], p: &Candidate, c: &Candidate) -> Option<f32> {
    match (p.unit, c.unit) {
        (Some(pu), Some(cu)) if p.b == c.a => {
            // Overlap on the shared phoneme: cheap in vowels, costly in bursts.
            let (pu, cu) = (&bank.units[pu as usize], &bank.units[cu as usize]);
            let class = items[c.a].phoneme.class().join_cost();
            let x = middle_frame(bank, pu, p.first + (p.b - p.a));
            let y = middle_frame(bank, cu, c.first);
            Some(class + 0.03 * spectral_distance(&x, &y))
        }
        (_, _) if p.b == c.a => None,
        (Some(_), Some(_)) if p.b + 1 == c.a => {
            // Where the cut falls decides how much it is heard: in a stop's
            // closure, near silence, hardly; between a consonant's release
            // and its vowel, the transition is lost.
            // A consonant cut short before the join is a stub of itself.
            let (left, right) = (items[p.b].phoneme.class(), items[c.a].phoneme.class());
            let place = match (left, right) {
                (_, Class::Stop | Class::Affricate) => 0.6,
                (Class::Stop | Class::Affricate | Class::Fricative, Class::Vowel) => 2.2,
                _ => 1.4,
            };
            Some(place + 1.5 * stub(bank, p))
        }
        // Next to a pause, a cut is a natural onset or ending.
        (_, _) if p.b + 1 == c.a => Some(0.25),
        _ => None,
    }
}

/// The cheapest chain of candidates covering every item.
fn select(bank: &Voicebank, items: &[Item], cands: &[Candidate]) -> Vec<Candidate> {
    let n = items.len();
    let mut ending: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (i, c) in cands.iter().enumerate() {
        ending[c.b].push(i);
    }
    let mut order: Vec<usize> = (0..cands.len()).collect();
    order.sort_by_key(|&i| (cands[i].a, cands[i].b));
    let mut best = vec![f32::INFINITY; cands.len()];
    let mut back = vec![usize::MAX; cands.len()];
    for &i in &order {
        let c = &cands[i];
        if c.a == 0 {
            best[i] = c.cost;
            continue;
        }
        let mut predecessors: Vec<usize> = ending[c.a].clone();
        predecessors.extend(&ending[c.a - 1]);
        for p in predecessors {
            if !best[p].is_finite() || p == i {
                continue;
            }
            if let Some(j) = join_cost(bank, items, &cands[p], c) {
                let total = best[p] + j + c.cost;
                if total < best[i] {
                    best[i] = total;
                    back[i] = p;
                }
            }
        }
    }
    let end = ending.get(n.wrapping_sub(1)).and_then(|list| {
        list.iter()
            .copied()
            .filter(|&i| best[i].is_finite())
            .min_by(|&x, &y| best[x].total_cmp(&best[y]))
    });
    let Some(mut i) = end else { return Vec::new() };
    let mut chain = vec![cands[i]];
    while back[i] != usize::MAX {
        i = back[i];
        chain.push(cands[i]);
    }
    chain.reverse();
    chain
}

/// Maps target time to a fractional source frame.
#[derive(Clone, Debug)]
struct Placement {
    t0: f64,
    t1: f64,
    s0: f64,
    s1: f64,
    /// Steady region that may stretch or loop.
    stable: Option<(f64, f64)>,
    /// Weight ramps: up over fade_in, down over fade_out (seconds).
    fade_in: (f64, f64),
    fade_out: (f64, f64),
    /// Limits for extrapolating past the span at a hard join.
    clamp: (f64, f64),
    gain_db: f32,
    /// For a vowel: the voice's typical shape of it, and the source frames
    /// the vowel spans (the pull toward the shape fades in from its edges).
    shape: Option<(Vec<f32>, f64, f64)>,
}

impl Placement {
    fn weight(&self, t: f64) -> f64 {
        if t < self.fade_in.0 || t > self.fade_out.1 {
            return 0.0;
        }
        // A ramp of no length is no fade: inside the span, full weight.
        let ramp = |x: f64, a: f64, b: f64| {
            if b <= a {
                1.0
            } else {
                ((x - a) / (b - a)).clamp(0.0, 1.0)
            }
        };
        let up = ramp(t, self.fade_in.0, self.fade_in.1);
        let down = if self.fade_out.1 <= self.fade_out.0 {
            1.0
        } else {
            1.0 - ramp(t, self.fade_out.0, self.fade_out.1)
        };
        let w = up.min(down);
        // Equal-power-ish raised cosine.
        0.5 - 0.5 * (std::f64::consts::PI * w).cos()
    }

    fn source(&self, t: f64) -> f64 {
        let span_t = (self.t1 - self.t0).max(1e-9);
        let linear = |t: f64| self.s0 + (t - self.t0) / span_t * (self.s1 - self.s0);
        let s = match self.stable {
            Some((r0, r1)) if r1 - r0 >= 2.0 => {
                let head = (r0 - self.s0).max(0.0) * FRAME;
                let tail = (self.s1 - r1).max(0.0) * FRAME;
                let middle = span_t - head - tail;
                let natural = (r1 - r0) * FRAME;
                if middle <= 0.0 {
                    linear(t)
                } else if t < self.t0 + head {
                    self.s0 + (t - self.t0) / FRAME
                } else if t > self.t1 - tail {
                    r1 + (t - (self.t1 - tail)) / FRAME
                } else if middle <= natural * 2.0 {
                    r0 + (t - self.t0 - head) / middle * (r1 - r0)
                } else {
                    // Back and forth over the steady part, an odd number of
                    // passes so the loop ends where the tail begins.
                    let passes = ((middle / natural).round() as i64 | 1).max(1) as f64;
                    let phase = (t - self.t0 - head) / (middle / passes);
                    let n = phase.floor();
                    let f = phase - n;
                    let eased = 0.5 - 0.5 * (std::f64::consts::PI * f).cos();
                    if (n as i64) % 2 == 0 {
                        r0 + (r1 - r0) * eased
                    } else {
                        r1 - (r1 - r0) * eased
                    }
                }
            }
            _ => linear(t),
        };
        s.clamp(self.clamp.0, self.clamp.1)
    }
}

fn place(bank: &Voicebank, items: &[Item], chain: &[Candidate]) -> Vec<Placement> {
    let mut out = Vec::new();
    let median = bank.median_level();
    // The whole voice sings at a common level; each unit moves most of the
    // way to the voice's own median, smoothing uneven recordings.
    let voice_gain = crate::spectrum::SINGING_LEVEL - median;
    for (ci, c) in chain.iter().enumerate() {
        let Some(u) = c.unit else { continue };
        let unit = &bank.units[u as usize];
        let previous = ci.checked_sub(1).map(|i| &chain[i]);
        let next = chain.get(ci + 1);
        let unit_gain = if unit.level > FLOOR_DB + 10.0 {
            ((median - unit.level) * 0.7).clamp(-12.0, 12.0)
        } else {
            0.0
        };
        let gain_db = (voice_gain + unit_gain).clamp(-36.0, 36.0);
        for (j, item) in items.iter().enumerate().take(c.b + 1).skip(c.a) {
            let k = c.first + (j - c.a);
            let (s0, s1) = unit.span(k);
            let (s0, s1) = (s0 as f64, s1 as f64);
            let (t0, t1) = (item.start, item.end);
            let length = t1 - t0;
            let natural = (s1 - s0) * FRAME;
            let phoneme = unit.phonemes[k];
            let last = k + 1 == unit.phonemes.len();
            let stable = if last && unit.stable[1] > unit.stable[0] {
                Some((unit.stable[0] as f64, unit.stable[1] as f64))
            } else if phoneme.class().sustainable() && !phoneme.is_pause() {
                let q = (s1 - s0) * 0.25;
                Some((s0 + q, s1 - q))
            } else {
                None
            };
            let incoming = j == c.a && previous.is_some_and(|p| p.b == j && p.unit.is_some());
            let outgoing = j == c.b && next.is_some_and(|n| n.a == j && n.unit.is_some());
            let hard_in = j == c.a && previous.is_some_and(|p| p.b + 1 == j)
                || j == c.a && previous.is_none();
            let hard_out =
                j == c.b && next.is_some_and(|n| n.a == j + 1) || j == c.b && next.is_none();
            let mut p = Placement {
                t0,
                t1,
                s0,
                s1,
                stable,
                fade_in: (t0, t0),
                fade_out: (t1, t1),
                clamp: (unit.start() as f64, unit.end().saturating_sub(1) as f64),
                gain_db,
                shape: None,
            };
            if phoneme.is_vowel() {
                p.shape = bank.vowel_shape(phoneme, unit.pitch).map(|s| (s, s0, s1));
            }
            if phoneme.is_pause() {
                // A recorded lead-in or tail: keep at most 150 ms of it, faded.
                let keep = natural.min(0.15).min(length);
                if k == 0 {
                    p.t0 = t1 - keep;
                    p.s0 = s1 - keep / FRAME;
                    p.fade_in = (p.t0, p.t0 + keep * 0.6);
                } else {
                    p.t1 = t0 + keep;
                    p.s1 = s0 + keep / FRAME;
                    p.fade_out = (p.t1 - keep * 0.6, p.t1);
                }
                p.stable = None;
                out.push(p);
                continue;
            }
            if incoming {
                // The incoming unit sings the end of the shared phoneme.
                let own = natural.min(length * 0.6).max(FRAME * 2.0).min(length);
                p.t0 = t1 - own;
                p.stable = None;
                p.fade_in = (p.t0, p.t0 + own * 0.5);
            }
            if outgoing {
                // The outgoing unit sings the start and fades under the next.
                let next_unit = &bank.units[next.unwrap().unit.unwrap() as usize];
                let (n0, n1) = next_unit.span(next.unwrap().first);
                let own_next = ((n1 - n0) as f64 * FRAME)
                    .min(length * 0.6)
                    .max(FRAME * 2.0)
                    .min(length);
                let fade_start = t1 - own_next;
                let fade_end = t1 - own_next * 0.5;
                p.t1 = fade_end;
                p.fade_out = (fade_start, fade_end);
                if stable.is_none() || !phoneme.class().sustainable() {
                    // A consonant keeps its recorded speed.
                    p.t1 = (p.t0 + natural).min(fade_end).max(p.t0 + FRAME);
                    p.fade_out = (p.t1 - (fade_end - fade_start).min(p.t1 - p.t0), p.t1);
                }
            }
            if !phoneme.class().sustainable() && !incoming && !outgoing {
                // A stop or an affricate keeps its recorded speed, its
                // release at the end of its time: squeezed, a burst of one
                // frame would be skipped; given more time, it holds its
                // closure.
                p.stable = None;
                p.s0 = s1 - (t1 - t0) / FRAME;
            }
            if hard_in && !incoming {
                p.t0 -= EDGE;
                p.s0 -= EDGE / FRAME;
                p.fade_in = (p.t0, p.t0 + 2.0 * EDGE);
            }
            if hard_out && !outgoing {
                p.t1 += EDGE;
                p.s1 += EDGE / FRAME;
                p.fade_out = (p.t1 - 2.0 * EDGE, p.t1);
            }
            out.push(p);
        }
    }
    out
}

fn breath(bank: &Voicebank, progress: f64) -> Frame {
    let mut f = Frame::SILENT;
    let color = bank.color();
    let fade = (std::f64::consts::PI * progress).sin().max(0.0) as f32;
    let gain = 20.0 * fade.max(1e-4).log10();
    for (g, e) in f.env.iter_mut().enumerate() {
        let hz = crate::spectrum::grid_hz(g);
        // Breath is the voice's color, quieter, with less low end.
        let shape = if hz < 800.0 {
            -12.0 * (1.0 - hz / 800.0)
        } else {
            0.0
        };
        *e = (color.get(g).copied().unwrap_or(-90.0) - 28.0 + shape + gain).max(FLOOR_DB);
    }
    f
}

/// Semitones a voiced source frame lies off its 100 ms neighbourhood.
fn micro_pitch(bank: &Voicebank, position: f64, f0: f32) -> f32 {
    let center = position.round() as isize;
    let (mut sum, mut count) = (0.0f32, 0u32);
    for i in center - 10..=center + 10 {
        if i < 0 {
            continue;
        }
        let f = bank.f0(i as usize);
        if f > 0.0 {
            sum += f.log2();
            count += 1;
        }
    }
    if count < 5 {
        return 0.0;
    }
    12.0 * (f0.log2() - sum / count as f32)
}

/// Frames for laid-out phonemes from a voicebank. `pitch` is MIDI per frame;
/// `modulation` (0–1) keeps that share of the recording's own pitch detail;
/// `prefer[n]` names a unit the phrase's note `n` asks for ("" for any).
pub fn frames(
    bank: &Voicebank,
    segments: &[Segment],
    pitch: &[f32],
    modulation: f32,
    prefer: &[&str],
) -> Vec<Frame> {
    let count = pitch.len();
    let mut out = vec![Frame::SILENT; count];
    let items = items(bank, segments);
    if items.is_empty() || bank.units.is_empty() {
        return out;
    }
    let cands = candidates(bank, &items, pitch, prefer);
    let chain = select(bank, &items, &cands);
    let placements = place(bank, &items, &chain);
    // Index placements by frame range for speed.
    for (i, frame) in out.iter_mut().enumerate() {
        let t = i as f64 * FRAME;
        let mut weight = 0.0f64;
        let mut env = [0.0f64; ENV_POINTS];
        let mut ap = [0.0f64; crate::spectrum::AP_BANDS];
        let mut voiced = 0.0f64;
        let mut detail = 0.0f64;
        let mut detail_weight = 0.0f64;
        for p in &placements {
            let w = p.weight(t);
            if w <= 0.0 {
                continue;
            }
            let s = p.source(t);
            let f = bank.frame_at(s as f32);
            let mut own = f.env;
            for v in own.iter_mut() {
                *v += p.gain_db;
            }
            if let Some((shape, a, b)) = p.shape.as_ref().filter(|_| f.voiced()) {
                // Takes of a vowel differ; pull its middle toward the voice's
                // typical color so consecutive units match.
                let edge = ((s - a).min(b - s) / ((b - a) * 0.25).max(1.0)).clamp(0.0, 1.0);
                let pull = MEAN_PULL * (edge * edge * (3.0 - 2.0 * edge)) as f32;
                let level = crate::spectrum::color_level(&f.env);
                for ((o, v), m) in own.iter_mut().zip(&f.env).zip(shape) {
                    *o += pull * (m - (v - level));
                }
            }
            // Mixed as power: a fade into a closure is not a dB average
            // falling 20 dB below both.
            for (e, v) in env.iter_mut().zip(&own) {
                *e += w * 10f64.powf(*v as f64 / 10.0);
            }
            for (a, v) in ap.iter_mut().zip(&f.ap) {
                *a += w * *v as f64;
            }
            if f.voiced() {
                voiced += w;
                // Only the recording's fast pitch detail (jitter, the dip at a
                // voiced consonant) carries over, not its slow intonation.
                let semitones = micro_pitch(bank, s, f.f0);
                detail += w * semitones.clamp(-1.0, 1.0) as f64;
                detail_weight += w;
            }
            weight += w;
        }
        if weight <= 1e-6 {
            continue;
        }
        let presence = weight.min(1.0);
        let fade_db = 20.0 * presence.max(1e-4).log10();
        for (o, e) in frame.env.iter_mut().zip(&env) {
            *o = (10.0 * (e / weight).max(1e-30).log10() + fade_db).max(FLOOR_DB as f64) as f32;
        }
        for (o, a) in frame.ap.iter_mut().zip(&ap) {
            *o = (a / weight).clamp(0.0, 1.0) as f32;
        }
        if voiced / weight >= 0.5 {
            let micro = if detail_weight > 0.0 {
                detail / detail_weight
            } else {
                0.0
            } * modulation as f64;
            let midi = pitch[i] as f64 + micro;
            frame.f0 = (440.0 * 2f64.powf((midi - 69.0) / 12.0)) as f32;
        } else {
            frame.f0 = 0.0;
        }
    }
    // Breaths in pauses: from the timing's own breath segments.
    for s in segments.iter().filter(|s| s.phoneme == Phoneme::BREATH) {
        let a = (s.start / FRAME) as usize;
        let b = ((s.end / FRAME) as usize).min(count);
        for (i, frame) in out.iter_mut().enumerate().take(b).skip(a) {
            let progress = (i - a) as f64 / (b - a).max(1) as f64;
            if frame.level() < -150.0 {
                *frame = breath(bank, progress);
            }
        }
    }
    out
}

/// What cutting a unit short of its recorded `neighbour` costs, next to the
/// phrase's `target` phoneme there. Cutting off a recorded silence only
/// loses the context; cutting into a phoneme loses its transition, more so
/// the less the neighbour is like the phrase's: «у» from «лю» serves «рю»
/// better than «у» from «ду».
fn cut_cost(neighbour: Phoneme, target: Phoneme) -> f32 {
    if neighbour.is_pause() {
        return 0.25;
    }
    0.35 + 0.7 * (1.0 - (-neighbour.distance(target) / 2.5).exp())
}

/// A chosen unit's alias, with the phonemes used when it is cut.
fn label(bank: &Voicebank, c: &Candidate) -> String {
    let Some(u) = c.unit else {
        return "·".into();
    };
    let unit = &bank.units[u as usize];
    let used = &unit.phonemes[c.first..c.first + (c.b - c.a) + 1];
    if used.len() == unit.phonemes.len() {
        unit.alias.clone()
    } else {
        let symbols: Vec<_> = used.iter().map(|p| p.symbol()).collect();
        format!("{}[{}]", unit.alias, symbols.join(" "))
    }
}

/// Which unit aliases a phrase would use (for tests and the editor).
pub fn describe(bank: &Voicebank, segments: &[Segment], pitch: &[f32]) -> Vec<String> {
    let items = items(bank, segments);
    let cands = candidates(bank, &items, pitch, &[]);
    select(bank, &items, &cands)
        .iter()
        .map(|c| label(bank, c))
        .collect()
}

/// For each of the phrase's `notes`: the units its syllable is joined from,
/// and the units that could sing its main sound instead, best first.
pub fn note_units(
    bank: &Voicebank,
    segments: &[Segment],
    pitch: &[f32],
    prefer: &[&str],
    notes: usize,
) -> Vec<NoteUnits> {
    let mut out = vec![NoteUnits::default(); notes];
    let items = items(bank, segments);
    if items.is_empty() || bank.units.is_empty() {
        return out;
    }
    let cands = candidates(bank, &items, pitch, prefer);
    let chain = select(bank, &items, &cands);
    for (n, units) in out.iter_mut().enumerate() {
        let mine: Vec<usize> = (0..items.len())
            .filter(|&i| items[i].note == Some(n) && !items[i].pause)
            .collect();
        let Some(&last) = mine.last() else { continue };
        for c in chain.iter().filter(|c| c.unit.is_some()) {
            let name = label(bank, c);
            if mine.iter().any(|&i| (c.a..=c.b).contains(&i)) && !units.used.contains(&name) {
                units.used.push(name);
            }
        }
        // The syllable's vowel, else its last sound.
        let main = mine
            .iter()
            .copied()
            .find(|&i| items[i].phoneme.is_vowel())
            .unwrap_or(last);
        let mut options: Vec<(f32, &str)> = cands
            .iter()
            .filter(|c| (c.a..=c.b).contains(&main))
            .filter_map(|c| {
                c.unit
                    .map(|u| (c.cost, bank.units[u as usize].alias.as_str()))
            })
            .collect();
        options.sort_by(|x, y| x.0.total_cmp(&y.0));
        for (_, alias) in options {
            if units.options.len() < 16 && !units.options.iter().any(|o| o == alias) {
                units.options.push(alias.to_string());
            }
        }
        if let Some(alias) = prefer.get(n).filter(|a| !a.is_empty())
            && !units.options.iter().any(|o| o == alias)
        {
            units.options.insert(0, alias.to_string());
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bank::Info;
    use crate::phoneme::Language;
    use crate::timing::Role;

    /// A bank whose frames encode (unit, phoneme) in f0 so mapping is visible.
    fn bank(aliases: &[(&str, &str)]) -> Voicebank {
        let mut bank = Voicebank::new(Info {
            language: Language::Ja,
            rate: 44_100,
            ..Default::default()
        });
        for (alias, phonemes) in aliases {
            let phonemes = Phoneme::parse_list(Language::Ja, phonemes).unwrap();
            let mut bounds = Vec::new();
            for p in &phonemes {
                let mut f = Frame::SILENT;
                f.f0 = if p.voiced() && !p.is_pause() {
                    220.0
                } else {
                    0.0
                };
                f.env = [-90.0; ENV_POINTS];
                let len = if p.is_vowel() { 40 } else { 14 };
                bounds.push(bank.push_frames(&vec![f; len]));
            }
            bounds.push(bank.frame_count() as u32);
            let last = *bounds.last().unwrap();
            bank.add_unit(Unit {
                alias: alias.to_string(),
                phonemes,
                fixed: bounds[0],
                stable: [last - 30, last - 5],
                bounds,
                pitch: 57.0,
                level: -80.0,
                variant: false,
                source: String::new(),
            });
        }
        bank.finish();
        bank
    }

    fn segments(list: &[(&str, f64, f64)]) -> Vec<Segment> {
        list.iter()
            .map(|&(p, a, b)| Segment {
                phoneme: Phoneme::parse(Language::Ja, p).unwrap(),
                start: a,
                end: b,
                role: Role::Nucleus,
                note: Some(0),
            })
            .collect()
    }

    #[test]
    fn vcv_banks_overlap_on_vowels() {
        let b = bank(&[
            ("- ka", "sil k a"),
            ("a ki", "a k i"),
            ("i -", "i sil"),
            ("ka", "k a"),
            ("ki", "k i"),
        ]);
        let s = segments(&[
            ("sil", 0.0, 0.3),
            ("k", 0.3, 0.37),
            ("a", 0.37, 0.8),
            ("k", 0.8, 0.87),
            ("i", 0.87, 1.3),
            ("sil", 1.3, 1.5),
        ]);
        let pitch = vec![57.0; 300];
        assert_eq!(describe(&b, &s, &pitch), ["- ka", "a ki", "i -"]);
    }

    #[test]
    fn a_note_can_ask_for_another_unit() {
        let b = bank(&[
            ("- ka", "sil k a"),
            ("a ki", "a k i"),
            ("i -", "i sil"),
            ("ka", "k a"),
            ("ki", "k i"),
        ]);
        let s = segments(&[
            ("sil", 0.0, 0.3),
            ("k", 0.3, 0.37),
            ("a", 0.37, 0.8),
            ("sil", 0.8, 1.0),
        ]);
        let pitch = vec![57.0; 200];
        let auto = note_units(&b, &s, &pitch, &[""], 1);
        assert_eq!(auto[0].used, ["- ka"]);
        assert!(
            auto[0].options.iter().any(|o| o == "ka"),
            "{:?}",
            auto[0].options
        );
        let chosen = note_units(&b, &s, &pitch, &["ka"], 1);
        assert_eq!(chosen[0].used, ["ka"]);
        assert!(frames(&b, &s, &pitch, 0.0, &["ka"])[110].level() > -100.0);
    }

    #[test]
    fn a_cut_after_a_release_costs_more_than_one_in_a_closure() {
        // "o d' i": joining at the stop's closure keeps the d'→i release.
        let ru = |p| Phoneme::parse(Language::Ru, p).unwrap();
        let item = |p| Item {
            phoneme: ru(p),
            start: 0.0,
            end: 0.1,
            pause: false,
            note: None,
        };
        let items = [item("o"), item("d'"), item("i")];
        let b = bank(&[("ka", "k a"), ("ki", "k i")]);
        let unit = |a, b| Candidate {
            unit: Some(0),
            first: 0,
            a,
            b,
            cost: 0.0,
        };
        let in_closure = join_cost(&b, &items, &unit(0, 0), &unit(1, 2)).unwrap();
        let after_release = join_cost(&b, &items, &unit(0, 1), &unit(2, 2)).unwrap();
        assert!(
            in_closure < after_release,
            "{in_closure} vs {after_release}"
        );
    }

    #[test]
    fn cv_banks_join_at_boundaries() {
        let b = bank(&[("ka", "k a"), ("ki", "k i")]);
        let s = segments(&[
            ("sil", 0.0, 0.3),
            ("k", 0.3, 0.37),
            ("a", 0.37, 0.8),
            ("k", 0.8, 0.87),
            ("i", 0.87, 1.3),
            ("sil", 1.3, 1.5),
        ]);
        let pitch = vec![57.0; 300];
        assert_eq!(describe(&b, &s, &pitch), ["·", "ka", "ki", "·"]);
        let f = frames(&b, &s, &pitch, 0.0, &[]);
        // Silent before the phrase, voiced in the vowels at the sung pitch.
        assert_eq!(f[10].f0, 0.0);
        // The consonant inside "ka" sounds too (it once had no weight).
        assert!(f[67].level() > -150.0, "{}", f[67].level());
        assert!((f[120].f0 - 220.0).abs() < 0.5);
        assert!(f[120].level() > -100.0);
        assert!(f[290].level() < -150.0);
    }

    #[test]
    fn missing_sounds_use_parts_of_units() {
        // No unit has "s" + "t"; parts of others cover it.
        let b = bank(&[("sa", "s a"), ("to", "t o"), ("a s", "a s")]);
        let s = segments(&[
            ("sil", 0.0, 0.2),
            ("s", 0.2, 0.3),
            ("t", 0.3, 0.37),
            ("o", 0.37, 0.8),
            ("sil", 0.8, 1.0),
        ]);
        let pitch = vec![57.0; 200];
        let used = describe(&b, &s, &pitch);
        assert!(used.contains(&"to".to_string()), "{used:?}");
        assert!(used.iter().any(|u| u.contains("[s]")), "{used:?}");
    }

    #[test]
    fn long_vowels_loop_their_steady_part() {
        let b = bank(&[("ka", "k a")]);
        let s = segments(&[
            ("sil", 0.0, 0.1),
            ("k", 0.1, 0.17),
            ("a", 0.17, 3.0),
            ("sil", 3.0, 3.2),
        ]);
        let pitch = vec![57.0; 640];
        let f = frames(&b, &s, &pitch, 0.0, &[]);
        let voiced = f.iter().filter(|f| f.voiced()).count();
        assert!(voiced > 540, "{voiced}");
    }
}
