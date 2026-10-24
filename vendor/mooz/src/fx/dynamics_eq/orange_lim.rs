//! OrangeLim (ORANGELM.ZDL), after the recovered `orange_limiter.rs` callback.
//!
//! Mono `0.5 (L + R)` → three detector/drive biquads (ROM c5..19: DC block,
//! low peak, +8 dB treble shelf) → ×1.585 (c20) → smoothed compressor gain
//! → ×2.11 (c46) → three biquads (c47..61) → symmetric soft clipper
//! `2u − u|u|` (c62..69) → three biquads (c70..84) → ×0.569 (c95) → Level
//! (c2 = 1.0 from the Level editor's fixed 100) on both outputs.
//!
//! The gain computer follows the callback: instant-attack peak of the previous
//! sample's drive signal (decay c33), an envelope with release c36 below
//! −15 dBFS (c34) and c35 above, threshold −27 dB with a 15:1 slope (c37/c38)
//! and the quadratic soft knee from −43.5 to −10.5 dB (c39..42), then 10^(dB/20)
//! smoothed by c44/c45. There are no user parameters; all numbers are the ROM's.
//! Log/exp use fast approximations.
use super::common::{
    GainCurve, Shaper, fast_db_to_gain, fast_gain_to_db, one_pole, per_sample, rom,
};
use crate::{Effect, Values, catalog::*, dsp::Biquad};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_orange_lim",
    name: "OrangeLim",
    category: Category::Dynamics,
    zdl: "ORANGELM.ZDL",
    description: "Squeezing limiter into a soft clipper",
    params: &[],
};

/// ROM 0x80000344 c5..19.
const DETECTOR: [[f32; 5]; 3] = [
    [0.998_577_3, -0.998_577_3, 0.0, 0.997_154_53, 0.0],
    [
        1.012_412,
        -1.971_395_1,
        0.959_495_54,
        1.971_395_1,
        -0.971_907_5,
    ],
    [2.322_217_2, -2.071_313_4, 0.0, 0.749_096_04, 0.0],
];
/// c47..61.
const PRE: [[f32; 5]; 3] = [
    [
        1.006_649_5,
        -1.956_943_5,
        0.953_476_37,
        1.956_943_5,
        -0.960_125_8,
    ],
    [
        1.012_207_4,
        -1.933_730_7,
        0.928_610_5,
        1.933_730_7,
        -0.940_817_9,
    ],
    [
        1.093_189_7,
        -0.630_842_27,
        0.416_833_22,
        0.630_842_27,
        -0.510_023,
    ],
];
/// c70..84.
const POST: [[f32; 5]; 3] = [
    [
        1.001_713_9,
        -1.971_395_1,
        0.970_193_6,
        1.971_395_1,
        -0.971_907_5,
    ],
    [
        0.971_173_6,
        -1.584_649_1,
        0.748_511_73,
        1.584_649_1,
        -0.719_685_3,
    ],
    [0.965_966_3, -0.408_121_38, 0.0, 0.374_087_7, 0.0],
];
const DRIVE: f32 = 1.584_893_2;
const PEAK_DECAY: f32 = 0.998_956_26;
const RELEASE_SWITCH: f32 = 0.177_827_94;
const FAST_RELEASE: f32 = 0.998_956_26;
const SLOW_RELEASE: f32 = 0.999_772_13;
const SMOOTH: (f32, f32) = (0.004_989_833_6, 0.995_010_14);
const INTO_CLIPPER: f32 = 2.113_489_2;
const OUTPUT: f32 = 0.568_852_9;
const CURVE: GainCurve = GainCurve {
    threshold: -27.0,
    slope: 0.066_666_67,
    knee_low: -43.5,
    knee_high: -10.5,
    knee_width: 33.0,
    knee_gain: -15.4,
};
const CLIPPER: Shaper = Shaper {
    input: 1.0,
    direct: 1.158_697_4,
    negative: [-1.158_697_4, -1.0, 0.0],
    positive: [1.158_697_4, 1.0, 0.0],
    output: 1.0,
};

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(OrangeLim::new(sample_rate))
}

pub struct OrangeLim {
    detector: [Biquad; 3],
    pre: [Biquad; 3],
    post: [Biquad; 3],
    peak_decay: f32,
    fast_release: f32,
    slow_release: f32,
    smooth_gain: f32,
    smooth_pole: f32,
    previous: f32,
    peak: f32,
    envelope: f32,
    gain: f32,
}

impl OrangeLim {
    fn new(sample_rate: f32) -> Self {
        let (smooth_gain, smooth_pole) = one_pole(SMOOTH.0, SMOOTH.1, sample_rate);
        Self {
            detector: DETECTOR.map(|w| Biquad::new(rom(w, sample_rate))),
            pre: PRE.map(|w| Biquad::new(rom(w, sample_rate))),
            post: POST.map(|w| Biquad::new(rom(w, sample_rate))),
            peak_decay: per_sample(PEAK_DECAY, sample_rate),
            fast_release: per_sample(FAST_RELEASE, sample_rate),
            slow_release: per_sample(SLOW_RELEASE, sample_rate),
            smooth_gain,
            smooth_pole,
            previous: 0.0,
            peak: 0.0,
            envelope: 0.0,
            gain: 1.0,
        }
    }
}

impl Effect for OrangeLim {
    fn set(&mut self, _values: &Values, _bpm: f32) {}

    #[inline]
    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let mut x = 0.5 * (left + right);
        for section in &mut self.detector {
            x = section.process(x);
        }
        let magnitude = self.previous.abs();
        self.peak = if self.peak < magnitude {
            magnitude
        } else {
            self.peak * self.peak_decay
        };
        self.envelope = if self.envelope < self.peak {
            self.peak
        } else if self.peak < RELEASE_SWITCH {
            self.envelope * self.slow_release
        } else {
            self.envelope * self.fast_release
        };
        let gain = fast_db_to_gain(CURVE.gain_db(fast_gain_to_db(self.envelope)));
        self.gain = self.smooth_pole * self.gain + self.smooth_gain * gain;
        let drive = x * DRIVE;
        self.previous = drive;
        let mut y = INTO_CLIPPER * self.gain * drive;
        for section in &mut self.pre {
            y = section.process(y);
        }
        y = CLIPPER.process(y);
        for section in &mut self.post {
            y = section.process(y);
        }
        let y = y * OUTPUT;
        [y, y]
    }

    fn clear(&mut self) {
        for section in self
            .detector
            .iter_mut()
            .chain(self.pre.iter_mut())
            .chain(self.post.iter_mut())
        {
            section.clear();
        }
        self.previous = 0.0;
        self.peak = 0.0;
        self.envelope = 0.0;
        self.gain = 1.0;
    }
}

#[cfg(test)]
mod tests {
    use super::super::common::test_util::sine_gain;
    use super::*;
    use crate::dsp::gain_to_db;

    #[test]
    fn clipper_is_two_u_minus_u_squared() {
        for i in -20..=20 {
            let x = i as f32 * 0.05;
            let u = (x * 1.158_697_4).clamp(-1.0, 1.0);
            let expected = 2.0 * u - u * u.abs();
            assert!((CLIPPER.process(x) - expected).abs() < 1e-6, "{x}");
        }
    }

    #[test]
    fn knee_is_continuous() {
        let inside = CURVE.gain_db(-10.501);
        let outside = CURVE.gain_db(-10.499);
        assert!((inside - outside).abs() < 0.01, "{inside} {outside}");
        assert!(CURVE.gain_db(-50.0) == 0.0);
    }

    #[test]
    fn limits_level_changes() {
        let fs = 48_000.0;
        let mut effect = OrangeLim::new(fs);
        let quiet = sine_gain(&mut effect, fs, 400.0, 0.01);
        effect.clear();
        let loud = sine_gain(&mut effect, fs, 400.0, 0.5);
        let change = gain_to_db(loud * 0.5) - gain_to_db(quiet * 0.01);
        // 34 dB more input gives a much smaller output change.
        assert!(change < 16.0, "{change}");
        assert!(loud.is_finite() && loud * 0.5 < 1.5);
    }
}
