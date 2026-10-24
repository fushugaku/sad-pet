//! OptComp (OPTCOMP.ZDL), after the recovered `optical_compressor.rs` callback.
//!
//! Mono: `0.5 (L + R)` → first-order input shelf (ROM c11..15) → Drive gain
//! (c21) → compressor gain → ×2 (c24) → two tone sections (c26..35, a high
//! shelf and a −20 dB de-emphasis) → clip/ramp shaper (c41..48, c69) → three
//! ROM output biquads (c49..63, re-emphasis and a low peak) → the Tone shelf
//! (c64..68) → Level (c2) on both outputs.
//!
//! Detector: instant-attack peak envelope of the drive-scaled signal (one
//! sample late, as in the callback) with the two-speed optical release (c5 above
//! 0.1, the slower ROM c6 below), floored at c10; gain `c8 · env^(c7 − 1)` (a
//! 3.7:1 ratio above −31.5 dBFS) smoothed by the one-pole c22/c23.
//!
//! Editors decoded: Drive `c21 = 10^((−20 + 20 (d/10)²)/20)` ramped over
//! 16 ms; Tone designs a first-order high shelf (Main type 8) at 5 kHz with
//! −18..0 dB for 0..50 or at 8 kHz with 0..+12 dB for 51..100; Level
//! `c2 = v/100`; the hidden Release editor (fixed value 42) sets
//! `c5 = 0.001^(1/(2537.4 ms))`. The per-sample `pow` uses fast log2/exp2.
use super::common::{
    Kind, RAMP_SECONDS, Shaper, TimedRamp, design, fast_exp2, fast_log2, level, one_pole,
    per_sample, rom,
};
use crate::{
    Effect, Values,
    catalog::*,
    dsp::Biquad,
    zoom::{DEFAULT_STEP, Ramp, curve},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_opt_comp",
    name: "OptComp",
    category: Category::Dynamics,
    zdl: "OPTCOMP.ZDL",
    description: "Optical compressor with soft drive and tone shelf",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Drive",
            max: 10,
            default: 7,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Tone",
            max: 100,
            default: 54,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Level",
            max: 150,
            default: 50,
            display: Display::INT,
        },
    ],
};

/// ROM 0x800002f0 words 11..15, 26..35 and 49..63.
const INPUT: [f32; 5] = [0.998_938_74, -0.996_806_14, 0.0, 0.995_744_94, 0.0];
const TONE: [[f32; 5]; 2] = [
    [1.344_236_4, -0.801_899_73, 0.0, 0.457_663_42, 0.0],
    [0.106_366_17, -0.092_219_13, 0.0, 0.985_852_96, 0.0],
];
const POST: [[f32; 5]; 3] = [
    [9.400_548, -9.267_337, 0.0, 0.866_788_45, 0.0],
    [1.417_604_8, -0.490_753_77, 0.0, 0.073_148_93, 0.0],
    [
        1.005_794_3,
        -1.971_108_3,
        0.966_114_5,
        1.971_108_3,
        -0.971_908_87,
    ],
];
const SHAPER: Shaper = Shaper {
    input: 8.0,
    direct: 0.629_462_7,
    negative: [1.144_477_6, 0.55, 0.0],
    positive: [-1.144_477_6, -0.55, 0.0],
    output: 0.5,
};
/// c7 (exponent), c8 (numerator), c9 (release switch), c10 (floor), c24.
const EXPONENT: f32 = 0.270_270_26;
const NUMERATOR: f32 = 0.070_794_575;
const RELEASE_SWITCH: f32 = 0.1;
const FLOOR: f32 = 0.026_550_584;
const MAKEUP: f32 = 2.0;
/// ROM c6, the slow release below the switch level.
const SLOW_RELEASE: f32 = 0.999_98;
/// Release editor: `curve(6000, 30, 101, 42, linear)` ms to fall by 60 dB.
const RELEASE_MS: f32 = 30.0 + (6000.0 - 30.0) * 0.42;

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(OptComp::new(sample_rate))
}

pub struct OptComp {
    sample_rate: f32,
    input: Biquad,
    tone: [Biquad; 2],
    post: [Biquad; 3],
    shelf: Biquad,
    fast_release: f32,
    slow_release: f32,
    smooth_gain: f32,
    smooth_pole: f32,
    floor_log2: f32,
    peak: f32,
    envelope: f32,
    gain: f32,
    drive: TimedRamp,
    level: Ramp,
    tone_value: f32,
}

impl OptComp {
    fn new(sample_rate: f32) -> Self {
        let (smooth_gain, smooth_pole) = one_pole(0.002, 0.998, sample_rate);
        let mut effect = Self {
            sample_rate,
            input: Biquad::new(rom(INPUT, sample_rate)),
            tone: TONE.map(|words| Biquad::new(rom(words, sample_rate))),
            post: POST.map(|words| Biquad::new(rom(words, sample_rate))),
            shelf: Biquad::default(),
            fast_release: (0.001f64.ln() / f64::from(RELEASE_MS * 0.001 * sample_rate)).exp()
                as f32,
            slow_release: per_sample(SLOW_RELEASE, sample_rate),
            smooth_gain,
            smooth_pole,
            floor_log2: FLOOR.log2(),
            peak: 0.0,
            envelope: FLOOR,
            gain: 1.0,
            drive: TimedRamp::new(0.0, RAMP_SECONDS, sample_rate),
            level: Ramp::new(0.0, DEFAULT_STEP, sample_rate),
            tone_value: f32::NAN,
        };
        let defaults = crate::Model::OptComp.defaults();
        effect.set(&defaults, 120.0);
        effect.drive.jump(effect.drive.target);
        effect.level.jump(effect.level.target);
        effect
    }

    /// Drive editor: `curve(0, −20, 11, d, square)` dB.
    fn drive_gain(drive: f32) -> f32 {
        let db = curve(0.0, -20.0, 11, drive.clamp(0.0, 10.0), 1);
        10f32.powf(db / 20.0)
    }

    /// Tone editor: the first-order high shelf of `c64..68`.
    fn tone_shelf(tone: f32, sample_rate: f32) -> crate::dsp::Coefficients {
        let tone = tone.clamp(0.0, 100.0);
        let (hz, db) = if tone < 51.0 {
            (5000.0, curve(0.0, -18.0, 51, tone, 0))
        } else {
            (8000.0, curve(12.0, 0.0, 51, tone - 50.0, 0))
        };
        design(Kind::HighShelf1, sample_rate, hz, db, 1.0)
    }
}

impl Effect for OptComp {
    fn set(&mut self, values: &Values, _bpm: f32) {
        self.drive.set(Self::drive_gain(values[0]));
        if values[1] != self.tone_value {
            self.tone_value = values[1];
            self.shelf.coefficients = Self::tone_shelf(values[1], self.sample_rate);
        }
        self.level.set(level(values[2]));
    }

    #[inline]
    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let filtered = self.input.process(0.5 * (left + right));
        // Envelope from the previous sample's peak.
        self.envelope = if self.envelope < self.peak {
            self.peak
        } else if self.envelope < RELEASE_SWITCH {
            self.envelope * self.slow_release
        } else {
            self.envelope * self.fast_release
        };
        self.envelope = self.envelope.max(FLOOR);
        let log2 = fast_log2(self.envelope).max(self.floor_log2);
        let target = NUMERATOR * fast_exp2((EXPONENT - 1.0) * log2);
        self.gain = self.smooth_gain * target + self.smooth_pole * self.gain;
        let scaled = self.drive.tick() * filtered;
        self.peak = scaled.abs();
        let mut x = MAKEUP * scaled * self.gain;
        for section in &mut self.tone {
            x = section.process(x);
        }
        x = SHAPER.process(x);
        for section in &mut self.post {
            x = section.process(x);
        }
        let y = self.shelf.process(x) * self.level.tick();
        [y, y]
    }

    fn clear(&mut self) {
        self.input.clear();
        for section in self.tone.iter_mut().chain(self.post.iter_mut()) {
            section.clear();
        }
        self.shelf.clear();
        self.peak = 0.0;
        self.envelope = FLOOR;
        self.gain = 1.0;
    }
}

#[cfg(test)]
mod tests {
    use super::super::common::test_util::{sine_gain, values};
    use super::*;
    use crate::dsp::gain_to_db;

    #[test]
    fn gain_curve_is_unity_at_the_floor() {
        // c8 = c10^(1 − c7): the gain is continuous at the threshold.
        let gain = NUMERATOR * FLOOR.powf(EXPONENT - 1.0);
        assert!((gain - 1.0).abs() < 1e-3, "{gain}");
    }

    #[test]
    fn release_follows_the_editor() {
        let effect = OptComp::new(44_100.0);
        assert!((effect.fast_release - 0.999_938_2).abs() < 2e-7);
        let at96 = OptComp::new(96_000.0);
        let seconds = |r: f32, fs: f32| (0.001f32).ln() / r.ln() / fs;
        assert!((seconds(at96.fast_release, 96_000.0) - 2.537).abs() < 0.01);
    }

    #[test]
    fn compresses_about_3_7_to_1() {
        let fs = 48_000.0;
        let mut effect = OptComp::new(fs);
        effect.set(&values(&[10.0, 50.0, 100.0]), 120.0);
        // Measure gain-reduction slope on the detector directly.
        let quiet = sine_gain(&mut effect, fs, 100.0, 0.05);
        effect.clear();
        let loud = sine_gain(&mut effect, fs, 100.0, 0.2);
        let slope = (gain_to_db(loud) - gain_to_db(quiet)) / gain_to_db(4.0);
        // Output rises by ~1/3.7 of the input change (−0.73 per dB).
        assert!((slope + 0.73).abs() < 0.12, "slope {slope}");
    }

    #[test]
    fn tone_moves_the_top_end() {
        let fs = 48_000.0;
        let dark = OptComp::tone_shelf(0.0, fs);
        let flat = OptComp::tone_shelf(50.0, fs);
        let bright = OptComp::tone_shelf(100.0, fs);
        assert!((gain_to_db(dark.magnitude(fs, 20_000.0)) + 17.0).abs() < 1.5);
        assert!(gain_to_db(flat.magnitude(fs, 10_000.0)).abs() < 1e-3);
        assert!((gain_to_db(bright.magnitude(fs, 20_000.0)) - 11.0).abs() < 1.5);
    }

    #[test]
    fn defaults_are_finite_and_bounded() {
        for fs in [44_100.0, 96_000.0] {
            let mut effect = OptComp::new(fs);
            let gain = sine_gain(&mut effect, fs, 220.0, 0.5);
            assert!(gain.is_finite() && gain > 0.05 && gain < 2.0, "{gain}");
        }
    }
}
