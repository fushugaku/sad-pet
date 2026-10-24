//! DynaDelay (DYNADLY.ZDL): a mono feedback delay whose echo level follows
//! the playing dynamics: negative Sense brings the echo up as you play
//! harder, positive Sense ducks it while you play.
//!
//! Follows the recovered callback `Fx_DLY_DynaDelay`:
//! - Detector: the mono input passes a first-order high-pass (c5..7, ~80 Hz)
//!   and a +6 dB first-order treble shelf (c8..10), is scaled by c11 (the
//!   Sense table) and limited to 1; a peak holder decays by c12 per sample
//!   (~45 ms), a one-pole smoother c13/c14 (DC gain 52.7, ~93 ms) follows
//!   it, and `n = min(smooth, 1)` gives `gain = min(1.5 · max(n² − 0.125,
//!   0), 1)` (c15..18, c21).
//! - Delay: `(L + R)/2 + c20 · tap[n − 1]` is written and read `c32`
//!   samples back. The Q31 modulator (c26..31) and its fractional read are
//!   silent with the ROM values no editor changes, so they are left out, as
//!   are the unused dynamic feedback (c19) and dry (c24) terms.
//! - Output: `out = c2 · (x + (c23 + c22 · gain) · tap)` on both channels.
//!
//! Editors decoded (`init` copies the 42-word ROM block 0x80000564):
//! - `time_edit`: Time + 1 ms or a synced note (≤ 2000 ms) → `c32 =
//!   ⌊441·ms/10⌋` samples; the tap is muted (c3) and reopened by `onf_aft`
//!   after the new time, replaced here by a 16 ms crossfade.
//! - `sens_edit`: c11 from the 20-entry table at 0x80000740 (1.0 at −10
//!   down to 0.125 at ±1, up to 1.995 at +10), then `amount_edit`.
//! - `amount_edit`: Mix/100 goes to c22 with c23 = 0 for Sense −10..−1, or
//!   to c23 with c22 = −Mix/100 for +1..+10 (echo = Mix · (1 − gain)).
//! - `fb_edit`: c20 = 0.95 · F.B/100. `outLv_edit`: c2 = Level/100.
//!
//! Approximations: the detector listens to the effect's own input (the
//! original reads Main's dry input buffer, which is the same signal when
//! DynaDelay is first in the chain); the time-constant coefficients and the
//! two first-order filters are re-derived for the host rate.
use super::common::{DelayLine, FadeTap, FirstOrder, Ramp, flush};
use crate::{Effect, Values, catalog::*, zoom};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_dyna_delay",
    name: "DynaDelay",
    category: Category::Delay,
    zdl: "DYNADLY.ZDL",
    description: "Delay whose echo follows or ducks under your playing",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Time",
            max: 2014,
            default: 359,
            display: Display::DelayTime {
                threshold: 1999,
                offset: 1,
            },
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Sense",
            max: 19,
            default: 5,
            display: Display::labels(&[
                "-10", "-9", "-8", "-7", "-6", "-5", "-4", "-3", "-2", "-1", "1", "2", "3", "4",
                "5", "6", "7", "8", "9", "10",
            ]),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 80,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "F.B",
            max: 100,
            default: 30,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
    ],
};

const RATE: f32 = zoom::ORIGINAL_RATE;
const TIME_THRESHOLD: u16 = 1999;
const MAX_MS: f32 = 2000.0;
const FEEDBACK_MAX: f32 = 0.95;
const FADE_SECONDS: f32 = 705.6 / RATE;
/// The `context[8]` steps the editors pass: 0x3af7c5ec (c11, c20) and the
/// common 0x3a77c5f5 (c2, c22, c23).
const FAST_STEP: f32 = 2.0 * zoom::DEFAULT_STEP;

/// 0x80000740: the Sense detector gain c11, indexed by Sense (−10..−1, 1..10).
const SENSE_TABLE: [u32; 20] = [
    0x3f800000, 0x3f4b4000, 0x3f218000, 0x3f004000, 0x3ecbc000, 0x3ea1c000, 0x3e808000, 0x3e4c0000,
    0x3e220000, 0x3e008000, 0x3e008000, 0x3e2f0000, 0x3e6e0000, 0x3ea1c000, 0x3edc0000, 0x3f158000,
    0x3f4b4000, 0x3f8a3000, 0x3fbbe000, 0x3fff6000,
];
/// First positive Sense index (label "1"): from here the echo ducks.
const DUCKING: usize = 10;

/// ROM detector filters, `[b0, b1, a1]` with `y = b0 x + b1 x1 + a1 y1`.
const HIGH_PASS: [u32; 3] = [0x3f7e8c9f, 0xbf7e8c9f, 0x3f7d193e];
const SHELF: [u32; 3] = [0x3ffe7e12, 0xbffcae80, 0x3f7c60dc];
/// c12 (peak decay), c13/c14 (smoother), c17 (curve offset), c18 (curve gain).
const PEAK_DECAY: f32 = f32::from_bits(0x3f7fdf40);
const SMOOTH_INPUT: f32 = f32::from_bits(0x3c517439);
const SMOOTH_POLE: f32 = f32::from_bits(0x3f7ff017);
const CURVE_OFFSET: f32 = 0.125;
const CURVE_GAIN: f32 = 1.5;

fn first_order(words: [u32; 3], sample_rate: f32) -> FirstOrder {
    let [b0, b1, a1] = words.map(f32::from_bits);
    let (b0, b1, a1) = FirstOrder::resampled(b0, b1, a1, RATE, sample_rate);
    FirstOrder::new(b0, b1, a1)
}

/// The envelope detector of the callback's third pass.
#[derive(Debug, Clone)]
struct Detector {
    high_pass: FirstOrder,
    shelf: FirstOrder,
    peak: f32,
    smooth: f32,
    decay: f32,
    smooth_input: f32,
    smooth_pole: f32,
}

impl Detector {
    fn new(sample_rate: f32) -> Self {
        let ratio = RATE / sample_rate;
        let smooth_pole = SMOOTH_POLE.powf(ratio);
        Self {
            high_pass: first_order(HIGH_PASS, sample_rate),
            shelf: first_order(SHELF, sample_rate),
            peak: 0.0,
            smooth: 0.0,
            decay: PEAK_DECAY.powf(ratio),
            // Keep the smoother's DC gain (52.7) at the new pole.
            smooth_input: SMOOTH_INPUT * (1.0 - smooth_pole) / (1.0 - SMOOTH_POLE),
            smooth_pole,
        }
    }

    /// The dynamic gain 0..1 for one mono sample; `sense` is c11.
    #[inline]
    fn tick(&mut self, x: f32, sense: f32) -> f32 {
        let filtered = self.shelf.process(self.high_pass.process(x));
        let magnitude = (filtered * sense).abs().min(1.0);
        self.peak = flush((self.peak * self.decay).max(magnitude));
        self.smooth = flush(self.smooth_pole * self.smooth + self.smooth_input * self.peak);
        let normalized = self.smooth.min(1.0);
        let curve = (normalized * normalized - 1.0 - CURVE_OFFSET).max(-1.0);
        ((1.0 + curve) * CURVE_GAIN).min(1.0)
    }

    fn clear(&mut self) {
        self.high_pass.clear();
        self.shelf.clear();
        self.peak = 0.0;
        self.smooth = 0.0;
    }
}

struct DynaDelay {
    sample_rate: f32,
    line: DelayLine,
    tap: FadeTap,
    feedback_sample: f32,
    detector: Detector,
    sense: Ramp,
    feedback: Ramp,
    /// c22 (follows the gain) and c23 (fixed part) of the wet gain.
    dynamic: Ramp,
    fixed: Ramp,
    level: Ramp,
    configured: bool,
}

impl DynaDelay {
    fn new(sample_rate: f32) -> Self {
        let delay = 360.0 * 0.001 * sample_rate;
        Self {
            sample_rate,
            line: DelayLine::with_seconds(MAX_MS * 0.001 + 0.01, sample_rate),
            tap: FadeTap::new(delay, FADE_SECONDS, sample_rate),
            feedback_sample: 0.0,
            detector: Detector::new(sample_rate),
            sense: Ramp::new(1.0, FAST_STEP, sample_rate),
            feedback: Ramp::new(0.0, FAST_STEP, sample_rate),
            dynamic: Ramp::new(0.0, zoom::DEFAULT_STEP, sample_rate),
            fixed: Ramp::new(0.0, zoom::DEFAULT_STEP, sample_rate),
            level: Ramp::new(1.0, zoom::DEFAULT_STEP, sample_rate),
            configured: false,
        }
    }

    fn time_ms(value: f32, bpm: f32) -> f32 {
        zoom::delay_time_ms(value, TIME_THRESHOLD, bpm, MAX_MS)
    }
}

impl Effect for DynaDelay {
    fn set(&mut self, values: &Values, bpm: f32) {
        let [time, sense, mix, feedback, level, ..] = *values;
        let ms = Self::time_ms(time, bpm);
        self.tap.set(ms * 0.001 * self.sample_rate);
        let sense = sense.round().clamp(0.0, 19.0) as usize;
        self.sense.set(f32::from_bits(SENSE_TABLE[sense]));
        let mix = mix.clamp(0.0, 100.0) * 0.01;
        if sense < DUCKING {
            self.fixed.set(0.0);
            self.dynamic.set(mix);
        } else {
            self.fixed.set(mix);
            self.dynamic.set(-mix);
        }
        self.feedback
            .set(FEEDBACK_MAX * feedback.clamp(0.0, 100.0) * 0.01);
        self.level.set(level.clamp(0.0, 150.0) * 0.01);
        if !self.configured {
            self.configured = true;
            self.tap.jump(self.tap.target());
            for ramp in [
                &mut self.sense,
                &mut self.feedback,
                &mut self.dynamic,
                &mut self.fixed,
                &mut self.level,
            ] {
                ramp.jump(ramp.target);
            }
        }
    }

    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        let mono = 0.5 * (frame[0] + frame[1]);
        let gain = self.detector.tick(mono, self.sense.tick());

        let feedback = self.feedback.tick();
        self.line
            .write(flush(feedback * self.feedback_sample + mono));
        let tap = self.tap.read(&self.line);
        self.feedback_sample = tap;

        let wet = (self.fixed.tick() + self.dynamic.tick() * gain) * tap;
        let level = self.level.tick();
        [level * (frame[0] + wet), level * (frame[1] + wet)]
    }

    fn clear(&mut self) {
        self.line.clear();
        self.tap.jump(self.tap.target());
        self.feedback_sample = 0.0;
        self.detector.clear();
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(DynaDelay::new(sample_rate))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Model;

    fn values(edits: &[(usize, f32)]) -> Values {
        let mut values = Model::from_key("mooz_dyna_delay").unwrap().defaults();
        for &(index, value) in edits {
            values[index] = value;
        }
        values
    }

    /// First wet sample after a quiet impulse (Sense +10 keeps the echo open).
    fn echo_onset(sample_rate: f32, edits: &[(usize, f32)], bpm: f32) -> usize {
        let mut effect = DynaDelay::new(sample_rate);
        effect.set(&values(edits), bpm);
        for i in 0..(2.5 * sample_rate) as usize {
            let x = if i == 0 { 1e-3 } else { 0.0 };
            let y = effect.process([x, x])[0] - x;
            if y.abs() > 1e-5 {
                return i;
            }
        }
        usize::MAX
    }

    #[test]
    fn echo_lands_at_time_plus_one_ms() {
        for sample_rate in [44_100.0, 48_000.0, 96_000.0] {
            let onset = echo_onset(sample_rate, &[(1, 19.0), (3, 0.0)], 120.0);
            let expected = 0.36 * sample_rate;
            assert!(
                (onset as f32 - expected).abs() <= 1.0,
                "{sample_rate}: {onset}"
            );
        }
    }

    #[test]
    fn synced_time_follows_tempo() {
        // 2000 + 6 is a quarter note: 500 ms at 120 BPM.
        let onset = echo_onset(48_000.0, &[(0, 2006.0), (1, 19.0), (3, 0.0)], 120.0);
        assert!((onset as f32 - 24_000.0).abs() <= 1.0, "{onset}");
    }

    /// Wet RMS while a steady sine plays (dry removed), relative to the input.
    fn wet_while_playing(sense: f32, amplitude: f32) -> f32 {
        let mut effect = DynaDelay::new(48_000.0);
        effect.set(
            &values(&[(0, 99.0), (1, sense), (2, 100.0), (3, 0.0)]),
            120.0,
        );
        let mut energy = 0.0f64;
        for i in 0..48_000 {
            let x = amplitude * (i as f32 * std::f32::consts::TAU * 440.0 / 48_000.0).sin();
            let y = effect.process([x, x])[0] - x;
            if i >= 24_000 {
                energy += f64::from(y * y);
            }
        }
        (energy / 24_000.0).sqrt() as f32 / amplitude
    }

    #[test]
    fn negative_sense_follows_and_positive_sense_ducks() {
        // A 0.5 sine is loud enough to open (−10) or close (+10) the echo fully.
        let follow = wet_while_playing(0.0, 0.5);
        let duck = wet_while_playing(19.0, 0.5);
        assert!(follow > 0.6, "{follow}");
        assert!(duck < 0.02, "{duck}");
        // The detector saturates early (smoother gain 52.7): at −40 dB −10
        // still opens the echo while −1 leaves it closed.
        let sensitive = wet_while_playing(0.0, 0.01);
        let insensitive = wet_while_playing(9.0, 0.01);
        assert!(
            sensitive > 0.5 && insensitive < 0.05,
            "{sensitive} {insensitive}"
        );
    }

    #[test]
    fn ducked_echo_returns_after_playing_stops() {
        let mut effect = DynaDelay::new(48_000.0);
        effect.set(
            &values(&[(0, 999.0), (1, 19.0), (2, 100.0), (3, 0.0)]),
            120.0,
        );
        let mut late = 0.0f32;
        for i in 0..48_000 * 2 {
            let x = if i < 14_400 {
                0.5 * (i as f32 * std::f32::consts::TAU * 440.0 / 48_000.0).sin()
            } else {
                0.0
            };
            let y = effect.process([x, x])[0] - x;
            // The echo of the playing arrives 1 s later, after the detector
            // has released (~0.4 s after the input stops).
            if (55_000..62_000).contains(&i) {
                late = late.max(y.abs());
            }
        }
        assert!(late > 0.05, "{late}");
    }

    #[test]
    fn detector_time_constants_follow_the_host_rate() {
        for sample_rate in [44_100.0, 96_000.0] {
            let detector = Detector::new(sample_rate);
            let dc = detector.smooth_input / (1.0 - detector.smooth_pole);
            assert!((dc - 52.66).abs() < 0.1, "{dc}");
            // The peak holder halves in the same time at every rate.
            let half = (0.5f32.ln() / detector.decay.ln()) / sample_rate;
            assert!((half - 0.0314).abs() < 0.001, "{half}");
        }
    }

    #[test]
    fn sense_table_and_labels_line_up() {
        let table = SENSE_TABLE.map(f32::from_bits);
        assert!((table[0] - 1.0).abs() < 1e-6);
        assert!((table[9] - table[10]).abs() < 1e-6);
        assert!((table[19] - 1.995_117_2).abs() < 1e-6);
        if let Display::Labels { labels } = INFO.params[1].display {
            assert_eq!(labels.len(), 20);
            assert_eq!((labels[DUCKING - 1], labels[DUCKING]), ("-1", "1"));
        } else {
            panic!("Sense labels");
        }
    }

    #[test]
    fn feedback_at_maximum_stays_bounded() {
        for sense in [0.0, 19.0] {
            let mut effect = DynaDelay::new(48_000.0);
            effect.set(
                &values(&[(0, 10.0), (1, sense), (2, 100.0), (3, 100.0), (4, 150.0)]),
                120.0,
            );
            let mut peak = 0.0f32;
            for i in 0..48_000 * 6 {
                let x = if i < 48_000 {
                    (i as f32 * 0.07).sin()
                } else {
                    0.0
                };
                let [l, r] = effect.process([x, x]);
                assert!(l.is_finite() && r.is_finite());
                peak = peak.max(l.abs()).max(r.abs());
            }
            assert!(peak < 16.0, "{sense}: {peak}");
        }
    }

    #[test]
    fn defaults_sound_near_unity() {
        let mut effect = DynaDelay::new(48_000.0);
        effect.set(&values(&[]), 120.0);
        let (mut input, mut output) = (0.0f64, 0.0f64);
        for i in 0..48_000 * 3 {
            let x = 0.3 * (i as f32 * std::f32::consts::TAU * 220.0 / 48_000.0).sin();
            let [l, r] = effect.process([x, x]);
            input += f64::from(x * x);
            output += f64::from(0.5 * (l * l + r * r));
        }
        let ratio = (output / input).sqrt();
        assert!((0.7..2.5).contains(&ratio), "{ratio}");
    }
}
