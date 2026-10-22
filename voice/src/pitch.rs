//! The pitch a singer actually sings: smooth transitions between notes with a
//! little overshoot and preparation, scoops into phrases and falls out of
//! them, delayed vibrato, slow drift, the per-note attack and release
//! effects, and the user's pitch-bend curve on top.
use crate::score::{Attack, Effect, Release, Style};
use crate::spectrum::{FRAME_SECONDS, Rng};

#[derive(Clone, Copy, Debug)]
pub struct PitchNote {
    pub start: f64,
    pub end: f64,
    /// MIDI note, transposition included.
    pub key: f32,
    pub vibrato_depth: f32,
    pub vibrato_rate: f32,
    pub vibrato_delay: f32,
    pub attack: Effect<Attack>,
    pub release: Effect<Release>,
}

/// Notes closer than this sing legato into each other.
const LEGATO_GAP: f64 = 0.06;

fn smoothstep(x: f64) -> f64 {
    let x = x.clamp(0.0, 1.0);
    x * x * (3.0 - 2.0 * x)
}

/// A hump over `0..width` that peaks at 1 in its middle.
fn bump(t: f64, width: f64) -> f64 {
    if t < 0.0 {
        0.0
    } else {
        let x = t / width;
        x * (1.0 - x).max(0.0) * 4.0
    }
}

/// Smooth noise: a few slow sinusoids with random phases.
struct Wander {
    parts: [(f64, f64, f64); 3],
}
impl Wander {
    fn new(rng: &mut Rng, base: f64) -> Self {
        let mut part = |f: f64| {
            (
                f * (1.0 + 0.3 * rng.uniform() as f64),
                std::f64::consts::TAU * rng.uniform() as f64,
                0.6 + 0.4 * rng.uniform().abs() as f64,
            )
        };
        Self {
            parts: [part(base), part(base * 2.3), part(base * 4.1)],
        }
    }
    fn at(&self, t: f64) -> f64 {
        let (sum, weight) = self
            .parts
            .iter()
            .fold((0.0, 0.0), |(s, w), &(f, phase, a)| {
                (s + a * (std::f64::consts::TAU * f * t + phase).sin(), w + a)
            });
        sum / weight
    }
}

/// MIDI pitch per frame for `frames` frames from the phrase origin.
/// `bend(t)` adds cents (the PIT/PBS curve), `portamento(t)` is POR (0–127).
pub fn contour(
    notes: &[PitchNote],
    frames: usize,
    style: &Style,
    bend: &dyn Fn(f64) -> f32,
    portamento: &dyn Fn(f64) -> f32,
    seed: u64,
) -> Vec<f32> {
    let mut out = vec![60.0f32; frames];
    if notes.is_empty() {
        return out;
    }
    let mut rng = Rng::new(seed);
    let drift = Wander::new(&mut rng, 0.35);
    let vibrato_wander = Wander::new(&mut rng, 0.5);
    let robot = style.robot;

    for (i, value) in out.iter_mut().enumerate() {
        let t = i as f64 * FRAME_SECONDS as f64;
        // The note sounding now, or the nearer one in a rest.
        let index = match notes.iter().position(|n| t < n.end) {
            Some(k) if t >= notes[k].start || k == 0 => k,
            Some(k) => {
                let gap_mid = (notes[k - 1].end + notes[k].start) * 0.5;
                if t < gap_mid { k - 1 } else { k }
            }
            None => notes.len() - 1,
        };
        let mut pitch = notes[0].key as f64;
        // Transitions between consecutive notes accumulate from the first key.
        for k in 1..notes.len() {
            let (a, b) = (&notes[k - 1], &notes[k]);
            let interval = (b.key - a.key) as f64;
            let legato = b.start - a.end < LEGATO_GAP;
            let boundary = if legato {
                // POR above 64 moves the transition earlier.
                b.start - (portamento(b.start) as f64 - 64.0) / 64.0 * 0.04
            } else {
                (a.end + b.start) * 0.5
            };
            let duration = if robot {
                0.02
            } else {
                style.transition as f64 * (0.7 + 0.3 * (interval.abs() / 12.0).min(1.0))
            };
            let x = (t - boundary) / duration + 0.5;
            if x <= 0.0 && (!legato || robot) {
                break;
            }
            let mut shift = interval * smoothstep(x);
            if legato && !robot && interval != 0.0 {
                let after = t - (boundary + duration * 0.5);
                shift += interval * style.overshoot as f64 * 0.12 * bump(after, 0.16);
                let before = (boundary - duration * 0.5) - t;
                shift -= interval * style.preparation as f64 * 0.06 * bump(before, 0.1);
            }
            pitch += shift;
            if x <= 0.0 {
                break;
            }
        }
        let note = &notes[index];
        let mut cents = 0.0f64;
        if !robot {
            // Scoop into a note after a rest, fall at the end before one.
            let after_rest = index == 0 || note.start - notes[index - 1].end >= LEGATO_GAP;
            let before_rest =
                index + 1 == notes.len() || notes[index + 1].start - note.end >= LEGATO_GAP;
            let since = t - note.start;
            let left = note.end - t;
            if after_rest && since >= -0.2 {
                cents -= style.scoop as f64 * (1.0 - smoothstep(since / 0.12 + 0.3));
            }
            if before_rest && left < 0.15 {
                cents -= style.fall as f64 * smoothstep(1.0 - left / 0.15);
            }
            // Attack and release effects.
            let a = note.attack.amount as f64;
            match note.attack.kind {
                Attack::Scoop => cents -= 150.0 * a * (1.0 - smoothstep(since / 0.15)),
                Attack::Bend => cents += 150.0 * a * (1.0 - smoothstep(since / 0.15)),
                Attack::Turn => {
                    cents -= 120.0 * a * bump(since, 0.18);
                }
                _ => {}
            }
            let r = note.release.amount as f64;
            let tail = smoothstep(1.0 - left / 0.25);
            match note.release.kind {
                Release::Fall => cents -= 300.0 * r * tail * tail,
                Release::Rise => cents += 200.0 * r * tail * tail,
                _ => {}
            }
            // Vibrato after its delay, fading in over 0.25 s.
            let length = note.end - note.start;
            if note.vibrato_depth > 0.0 && length >= style.vibrato_min_length as f64 {
                let start = note.start + note.vibrato_delay as f64;
                if t > start && t < note.end {
                    let fade = smoothstep((t - start) / 0.25);
                    let swell = if note.release.kind == Release::Swell {
                        1.0 + r * smoothstep((t - start) / (note.end - start).max(0.01))
                    } else {
                        1.0
                    };
                    let rate = note.vibrato_rate as f64 * (1.0 + 0.04 * vibrato_wander.at(t));
                    let depth =
                        note.vibrato_depth as f64 * (1.0 + 0.15 * vibrato_wander.at(t * 1.7));
                    cents +=
                        depth * fade * swell * (std::f64::consts::TAU * rate * (t - start)).sin();
                }
            }
            cents += style.drift as f64 * drift.at(t);
        }
        cents += bend(t) as f64;
        *value = (pitch + cents / 100.0) as f32;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    fn note(start: f64, end: f64, key: f32) -> PitchNote {
        PitchNote {
            start,
            end,
            key,
            vibrato_depth: 0.0,
            vibrato_rate: 5.5,
            vibrato_delay: 0.3,
            attack: Effect::default(),
            release: Effect::default(),
        }
    }
    fn at(c: &[f32], t: f64) -> f32 {
        c[(t / FRAME_SECONDS as f64).round() as usize]
    }
    #[test]
    fn robot_steps_exactly() {
        let style = Style::presets().into_iter().find(|s| s.robot).unwrap();
        let notes = [note(0.0, 0.5, 60.0), note(0.5, 1.0, 67.0)];
        let c = contour(&notes, 200, &style, &|_| 0.0, &|_| 64.0, 1);
        assert_eq!(at(&c, 0.25), 60.0);
        assert_eq!(at(&c, 0.75), 67.0);
    }
    #[test]
    fn natural_glides_and_overshoots() {
        let style = Style {
            drift: 0.0,
            scoop: 0.0,
            fall: 0.0,
            ..Style::default()
        };
        let notes = [note(0.0, 0.5, 60.0), note(0.5, 1.2, 64.0)];
        let c = contour(&notes, 240, &style, &|_| 0.0, &|_| 64.0, 1);
        assert!((at(&c, 0.2) - 60.0).abs() < 0.01);
        let mid = at(&c, 0.5);
        assert!(mid > 61.0 && mid < 63.0, "{mid}");
        let peak = c[100..140].iter().copied().fold(0.0, f32::max);
        assert!(peak > 64.1, "no overshoot: {peak}");
        assert!((at(&c, 1.0) - 64.0).abs() < 0.05);
    }
    #[test]
    fn vibrato_after_delay_and_bend() {
        let style = Style {
            drift: 0.0,
            scoop: 0.0,
            fall: 0.0,
            ..Style::default()
        };
        let mut n = note(0.0, 2.0, 60.0);
        n.vibrato_depth = 50.0;
        let c = contour(
            &[n],
            400,
            &style,
            &|t| if t > 1.5 { 100.0 } else { 0.0 },
            &|_| 64.0,
            1,
        );
        assert!((at(&c, 0.2) - 60.0).abs() < 0.01);
        let wobble = c[140..290]
            .iter()
            .fold((f32::MAX, f32::MIN), |(lo, hi), &v| (lo.min(v), hi.max(v)));
        assert!(wobble.1 - wobble.0 > 0.7, "{wobble:?}");
        assert!(at(&c, 1.7) > 60.4);
    }
}
