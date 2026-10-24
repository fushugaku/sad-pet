//! Splitter (SPLITTER.ZDL), after the recovered `splitter.rs` callback.
//!
//! Mono `0.5 (L + R)` into a one-pole low-pass `h0 += T (x − h0)` (c5 = T,
//! c6 = 1 − T); the high band is `c6 (x − h0)`. The callback's second stage
//! (c7/c8) and third band (c11) stay at their ROM values 1/0/0, so the output
//! is `Lo · h0 + Hi · high`, times Level, on both outputs.
//!
//! Editors decoded: Freq picks T from the 16-entry ROM table 0x800004c8
//! (80 Hz … 2.5 kHz as `1 − e^(−2π f/44.1k)`, recomputed for the host rate);
//! Hi/Lo set c10/c9 to 0 at 0, else `curve(3.98, 0.251, 51, v, linear)`
//! (a linear amplitude from −12 to +12 dB), ramped over 16 ms; Level c2 = v/100.
use super::common::{RAMP_SECONDS, TimedRamp, level};
use crate::{
    Effect, Values,
    catalog::*,
    zoom::{DEFAULT_STEP, ORIGINAL_RATE, Ramp, curve},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_splitter",
    name: "Splitter",
    category: Category::Eq,
    zdl: "SPLITTER.ZDL",
    description: "Two-band split with separate high and low levels",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Hi",
            max: 50,
            default: 19,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Lo",
            max: 50,
            default: 16,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Freq",
            max: 15,
            default: 2,
            display: Display::labels(&[
                "80Hz", "100Hz", "125Hz", "160Hz", "200Hz", "250Hz", "315Hz", "400Hz", "500Hz",
                "630Hz", "800Hz", "1.0kHz", "1.3kHz", "1.6kHz", "2.0kHz", "2.5kHz",
            ]),
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Level",
            max: 150,
            default: 95,
            display: Display::INT,
        },
    ],
};

/// ROM 0x800004c8: one-pole coefficients at 44.1 kHz.
const CROSSOVER: [f32; 16] = [
    0.011_360_168,
    0.014_179_707,
    0.017_693_043,
    0.022_590_16,
    0.028_156_757,
    0.035_069_942,
    0.043_982_506,
    0.055_510_044,
    0.068_889_14,
    0.085_989,
    0.107_856_27,
    0.132_875_92,
    0.169_003_01,
    0.203_491_69,
    0.247_015_95,
    0.297_641_75,
];

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(Splitter::new(sample_rate))
}

pub struct Splitter {
    sample_rate: f32,
    coefficient: f32,
    low: f32,
    high_gain: TimedRamp,
    low_gain: TimedRamp,
    level: Ramp,
}

impl Splitter {
    fn new(sample_rate: f32) -> Self {
        let mut effect = Self {
            sample_rate,
            coefficient: 0.0,
            low: 0.0,
            high_gain: TimedRamp::new(1.0, RAMP_SECONDS, sample_rate),
            low_gain: TimedRamp::new(1.0, RAMP_SECONDS, sample_rate),
            level: Ramp::new(1.0, DEFAULT_STEP, sample_rate),
        };
        effect.set(&crate::Model::Splitter.defaults(), 120.0);
        effect.high_gain.jump(effect.high_gain.target);
        effect.low_gain.jump(effect.low_gain.target);
        effect.level.jump(effect.level.target);
        effect
    }

    /// Hi/Lo editor: 0 mutes the band.
    fn band_gain(value: f32) -> f32 {
        if value < 0.5 {
            0.0
        } else {
            curve(3.98, 0.251, 51, value.min(50.0), 0)
        }
    }

    /// The table entry recomputed for the host rate: `(1 − T)^(44.1k/fs)`.
    fn crossover(index: f32, sample_rate: f32) -> f32 {
        let t = CROSSOVER[(index.round().max(0.0) as usize).min(CROSSOVER.len() - 1)];
        1.0 - (1.0 - t).powf(ORIGINAL_RATE / sample_rate)
    }
}

impl Effect for Splitter {
    fn set(&mut self, values: &Values, _bpm: f32) {
        self.high_gain.set(Self::band_gain(values[0]));
        self.low_gain.set(Self::band_gain(values[1]));
        self.coefficient = Self::crossover(values[2], self.sample_rate);
        self.level.set(level(values[3]));
    }

    #[inline]
    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let x = 0.5 * (left + right);
        self.low = crate::dsp::flush(self.low + (x - self.low) * self.coefficient);
        let high = (1.0 - self.coefficient) * (x - self.low);
        let y =
            (self.low_gain.tick() * self.low + self.high_gain.tick() * high) * self.level.tick();
        [y, y]
    }

    fn clear(&mut self) {
        self.low = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::super::common::test_util::{sine_gain, values};
    use super::*;

    #[test]
    fn crossover_table_is_80_hz_to_2_5_khz() {
        let hz = |t: f32| -(1.0 - t).ln() * ORIGINAL_RATE / std::f32::consts::TAU;
        assert!((hz(CROSSOVER[0]) - 80.0).abs() < 1.0);
        assert!((hz(CROSSOVER[11]) - 1000.0).abs() < 2.0);
        assert!((hz(CROSSOVER[15]) - 2480.0).abs() < 5.0);
        let at96 = Splitter::crossover(11.0, 96_000.0);
        let hz96 = -(1.0 - at96).ln() * 96_000.0 / std::f32::consts::TAU;
        assert!((hz96 - 1000.7).abs() < 1.0, "{hz96}");
    }

    #[test]
    fn bands_follow_hi_and_lo() {
        let fs = 48_000.0;
        let mut effect = Splitter::new(fs);
        effect.set(&values(&[0.0, 50.0, 0.0, 100.0]), 120.0);
        effect.high_gain.jump(0.0);
        effect.low_gain.jump(3.98);
        let low = sine_gain(&mut effect, fs, 20.0, 0.1);
        let high = sine_gain(&mut effect, fs, 8000.0, 0.1);
        assert!((low - 3.98).abs() < 0.3, "{low}");
        assert!(high < 0.05, "{high}");
        effect.set(&values(&[50.0, 0.0, 0.0, 100.0]), 120.0);
        effect.high_gain.jump(3.98);
        effect.low_gain.jump(0.0);
        let high = sine_gain(&mut effect, fs, 8000.0, 0.1);
        // The callback scales the high band by c6 = 1 − T.
        assert!(
            (high - 3.98 * (1.0 - effect.coefficient)).abs() < 0.1,
            "{high}"
        );
        // At the 2.5 kHz split the high band no longer reaches 50 Hz.
        effect.set(&values(&[50.0, 0.0, 15.0, 100.0]), 120.0);
        let low = sine_gain(&mut effect, fs, 50.0, 0.1);
        assert!(low < 0.15, "{low}");
    }
}
