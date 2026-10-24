//! GrayComp (GRAYCOMP.ZDL), after the recovered `gray_comp.rs` callback.
//!
//! Mono `0.5 (L + R)` → prefilter biquads (ROM c47..61) → asymmetric clip/ramp
//! saturator (c62..69; its input gain follows SUSTN) → five shaping biquads
//! (c70..94) → three detector-path biquads (c5..19) → ×1.585 (c20) → gain →
//! three output biquads (c96..110, input scaled by c2) on both outputs.
//!
//! The gain comes from an instant-attack peak (decay c33) feeding a slow
//! envelope (release c36 below +6 dBFS, else c35), 20·log10 of it, a hard knee
//! at −18 dB (c37) with slope c38 = 1/ratio, 10^(dB/20) and the one-pole
//! smoother c44/c45; the detector sees the previous sample's scaled signal
//! (c32 = 0: feedforward), as in the callback.
//!
//! The callback reads the saturator output through an 8-word block buffer, i.e.
//! eight samples later; that pure latency is dropped here.
//!
//! The SUSTN/OUT editor is decoded: with `i = v / 10` and the remainder
//! interpolated, SUSTN sets `c62 = 0.45 · 10^(T1a/20)` (ROM 0x800007e0, −12.6 ..
//! +8.2 dB), the slope `c38 = T1b` (ratio 2:1 … ∞) and a makeup factor
//! (0x80000870) that multiplies the OUT table (0x80000840) into `c2`; all three
//! ramp over 16 ms. Log/exp use fast approximations.
use super::common::{
    GainCurve, RAMP_SECONDS, Shaper, TimedRamp, fast_db_to_gain, fast_gain_to_db, interpolate,
    one_pole, per_sample, rom,
};
use crate::{Effect, Values, catalog::*, dsp::Biquad};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_gray_comp",
    name: "GrayComp",
    category: Category::Dynamics,
    zdl: "GRAYCOMP.ZDL",
    description: "Sustaining compressor with asymmetric saturation",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "SUSTN",
            max: 100,
            default: 63,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "OUT",
            max: 100,
            default: 88,
            display: Display::INT,
        },
    ],
};

/// ROM 0x800002d4: prefilters (c47..61, the middle one is identity).
const PREFILTER: [[f32; 5]; 2] = [
    [
        1.009_221,
        -1.932_086_5,
        0.935_484_23,
        1.932_086_5,
        -0.944_705_2,
    ],
    [
        1.340_947_4,
        -1.866_907_2,
        0.685_048_76,
        1.482_937_2,
        -0.609_306,
    ],
];
/// c70..94 (c95 = 1).
const SHAPING: [[f32; 5]; 5] = [
    [
        1.007_322_2,
        -1.980_668_8,
        0.973_861_34,
        1.980_668_8,
        -0.981_183_6,
    ],
    [1.008_762_1, -0.977_090_84, 0.0, 0.985_852_96, 0.0],
    [
        1.036_576_4,
        -1.800_458_7,
        0.786_099_5,
        1.800_458_7,
        -0.822_675_9,
    ],
    [
        1.003_648_9,
        -1.931_878_2,
        0.936_543_46,
        1.931_878_2,
        -0.940_192_3,
    ],
    [0.849_750_16, 0.121_332_47, 0.0, 0.028_917_35, 0.0],
];
/// c5..19.
const DETECTOR: [[f32; 5]; 3] = [
    [0.999_288_14, -0.999_288_14, 0.0, 0.998_576_3, 0.0],
    [
        1.003_758_2,
        -1.956_943_5,
        0.956_367_6,
        1.956_943_5,
        -0.960_125_8,
    ],
    [
        1.031_543_9,
        -1.618_432_6,
        0.724_803_75,
        1.618_432_6,
        -0.756_347_66,
    ],
];
/// c96..110.
const OUTPUT: [[f32; 5]; 3] = [
    [0.996_502_04, -0.982_404_23, 0.0, 0.978_906_3, 0.0],
    [1.409_525_2, -1.409_525_2, 0.0, 0.995_734_8, 0.0],
    [
        0.926_259_04,
        -1.312_957_8,
        0.568_759_74,
        1.312_957_8,
        -0.495_018_75,
    ],
];
/// c20.
const DETECTOR_GAIN: f32 = 1.584_893_2;
/// c33..36: peak decay, release switch level, fast and slow release.
const PEAK_DECAY: f32 = 0.998_956_26;
const RELEASE_SWITCH: f32 = 1.995_262_3;
const FAST_RELEASE: f32 = 0.998_956_26;
const SLOW_RELEASE: f32 = 0.999_974_25;
/// c44/c45.
const SMOOTH: (f32, f32) = (0.005_522_820_6, 0.994_477_15);
/// T1 at 0x800007e0: saturator input gain in dB and compressor slope.
const DRIVE_DB: [f32; 12] = [
    -12.6, -11.88, -10.92, -9.0, -1.62, 1.68, 4.56, 6.0, 7.2, 7.2, 8.16, 8.16,
];
const SLOPE: [f32; 12] = [
    0.5,
    0.5,
    0.25,
    0.166_666_67,
    0.111_111_11,
    0.071_428_575,
    0.025,
    0.0125,
    0.000_333_333_3,
    1e-5,
    1e-5,
    1e-5,
];
/// 0x80000870: makeup by SUSTN; 0x80000840: OUT.
const MAKEUP: [f32; 12] = [
    0.670_481_86,
    0.626_592_04,
    0.781_054_8,
    0.803_532,
    0.792_293_37,
    0.792_293_37,
    0.792_293_37,
    0.848_486_36,
    0.848_486_36,
    0.837_247_8,
    0.883_867_2,
    0.883_867_2,
];
const OUT: [f32; 12] = [
    0.001_351_191,
    0.024_027_953,
    0.240_279_53,
    0.479_420_66,
    0.603_554_84,
    0.759_830_53,
    0.872_402_2,
    1.001_651_8,
    1.150_05,
    1.320_434_2,
    1.516_061_3,
    1.516_061_3,
];

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(GrayComp::new(sample_rate))
}

pub struct GrayComp {
    prefilter: [Biquad; 2],
    shaping: [Biquad; 5],
    detector: [Biquad; 3],
    output: [Biquad; 3],
    peak_decay: f32,
    fast_release: f32,
    slow_release: f32,
    smooth_gain: f32,
    smooth_pole: f32,
    curve: GainCurve,
    drive: TimedRamp,
    slope: TimedRamp,
    level: TimedRamp,
    previous: f32,
    peak: f32,
    envelope: f32,
    gain: f32,
}

impl GrayComp {
    fn new(sample_rate: f32) -> Self {
        let (smooth_gain, smooth_pole) = one_pole(SMOOTH.0, SMOOTH.1, sample_rate);
        let mut effect = Self {
            prefilter: PREFILTER.map(|w| Biquad::new(rom(w, sample_rate))),
            shaping: SHAPING.map(|w| Biquad::new(rom(w, sample_rate))),
            detector: DETECTOR.map(|w| Biquad::new(rom(w, sample_rate))),
            output: OUTPUT.map(|w| Biquad::new(rom(w, sample_rate))),
            peak_decay: per_sample(PEAK_DECAY, sample_rate),
            fast_release: per_sample(FAST_RELEASE, sample_rate),
            slow_release: per_sample(SLOW_RELEASE, sample_rate),
            smooth_gain,
            smooth_pole,
            curve: GainCurve {
                threshold: -18.0,
                slope: 1e-5,
                knee_low: -18.0,
                knee_high: -18.0,
                knee_width: 0.0,
                knee_gain: 0.0,
            },
            drive: TimedRamp::new(0.45, RAMP_SECONDS, sample_rate),
            slope: TimedRamp::new(1e-5, RAMP_SECONDS, sample_rate),
            level: TimedRamp::new(0.0, RAMP_SECONDS, sample_rate),
            previous: 0.0,
            peak: 0.0,
            envelope: 0.0,
            gain: 1.0,
        };
        effect.set(&crate::Model::GrayComp.defaults(), 120.0);
        for ramp in [&mut effect.drive, &mut effect.slope, &mut effect.level] {
            ramp.jump(ramp.target);
        }
        effect
    }

    /// `(saturator gain c62, slope c38, output c2)` for SUSTN and OUT.
    fn targets(sustain: f32, out: f32) -> (f32, f32, f32) {
        let s = sustain.clamp(0.0, 100.0) / 10.0;
        let o = out.clamp(0.0, 100.0) / 10.0;
        let drive = 0.45 * 10f32.powf(interpolate(&DRIVE_DB, s) / 20.0);
        (
            drive,
            interpolate(&SLOPE, s),
            interpolate(&MAKEUP, s) * interpolate(&OUT, o),
        )
    }

    fn shaper(input: f32) -> Shaper {
        Shaper {
            input,
            direct: 2.247_579,
            negative: [-5.116_298_7, -0.413_888_9, 0.13],
            positive: [2.842_388_2, 0.745, 0.13],
            output: 1.0,
        }
    }
}

impl Effect for GrayComp {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let (drive, slope, level) = Self::targets(values[0], values[1]);
        self.drive.set(drive);
        self.slope.set(slope);
        self.level.set(level);
    }

    #[inline]
    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let mut x = 0.5 * (left + right);
        for section in &mut self.prefilter {
            x = section.process(x);
        }
        x = Self::shaper(self.drive.tick()).process(x);
        for section in &mut self.shaping {
            x = section.process(x);
        }
        for section in &mut self.detector {
            x = section.process(x);
        }
        // Envelope of the previous sample's detector signal.
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
        self.curve.slope = self.slope.tick();
        let gain = fast_db_to_gain(self.curve.gain_db(fast_gain_to_db(self.envelope)));
        self.gain = self.smooth_gain * gain + self.smooth_pole * self.gain;
        let scaled = DETECTOR_GAIN * x;
        self.previous = scaled;
        let mut y = scaled * self.gain * self.level.tick();
        for section in &mut self.output {
            y = section.process(y);
        }
        [y, y]
    }

    fn clear(&mut self) {
        for section in self
            .prefilter
            .iter_mut()
            .chain(self.shaping.iter_mut())
            .chain(self.detector.iter_mut())
            .chain(self.output.iter_mut())
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
    use super::super::common::test_util::{sine_gain, values};
    use super::*;
    use crate::dsp::gain_to_db;

    #[test]
    fn editor_tables_interpolate_by_tens() {
        let (drive, slope, level) = GrayComp::targets(63.0, 88.0);
        assert!((gain_to_db(drive / 0.45) - (4.56 + 0.3 * 1.44)).abs() < 1e-3);
        assert!((slope - (0.025 - 0.3 * 0.0125)).abs() < 1e-6);
        let makeup = 0.792_293_37 + 0.3 * (0.848_486_36 - 0.792_293_37);
        let out = 1.150_05 + 0.8 * (1.320_434_2 - 1.150_05);
        assert!((level - makeup * out).abs() < 1e-5);
        let (_, top, _) = GrayComp::targets(100.0, 100.0);
        assert!((top - 1e-5).abs() < 1e-9);
    }

    #[test]
    fn sustain_raises_compression() {
        let fs = 48_000.0;
        let swing = |sustain: f32| {
            let mut effect = GrayComp::new(fs);
            effect.set(&values(&[sustain, 50.0]), 120.0);
            effect.drive.jump(effect.drive.target);
            effect.slope.jump(effect.slope.target);
            effect.level.jump(effect.level.target);
            let quiet = sine_gain(&mut effect, fs, 500.0, 0.02);
            let loud = sine_gain(&mut effect, fs, 500.0, 0.4);
            gain_to_db(loud * 0.4) - gain_to_db(quiet * 0.02)
        };
        let light = swing(0.0);
        let heavy = swing(100.0);
        // 26 dB more input gives less output change with more sustain.
        assert!(heavy < light - 3.0, "light {light} heavy {heavy}");
        assert!(heavy < 12.0, "{heavy}");
    }

    #[test]
    fn defaults_are_finite_and_bounded() {
        for fs in [44_100.0, 96_000.0] {
            let mut effect = GrayComp::new(fs);
            let gain = sine_gain(&mut effect, fs, 220.0, 0.3);
            assert!(gain.is_finite() && gain > 0.05 && gain < 4.0, "{gain}");
        }
    }
}
