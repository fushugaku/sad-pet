//! Shared pieces of the dynamics/EQ group: the Main biquad designer every EQ
//! editor calls (`context[25]`, Main c00bcb20 → c00bc2e0), ROM coefficient
//! sets, the clip/ramp shaper of OptComp/GrayComp/OrangeLim, the compressor
//! gain computer, fast log/exp for per-sample detectors, duration ramps and the
//! common EQ chain (mono sum or stereo, Level, ±1 clamp).
use crate::{
    dsp::{Biquad, Coefficients},
    zoom::{DEFAULT_STEP, ORIGINAL_RATE, Ramp},
};

/// Filter types of the Main designer (`B8` of `context[25]`), from its jump
/// table at c00ef0f0. 2/4 and 3/5 share code.
// The whole designer is kept; this group only uses the shelf and peak types.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Lowpass1 = 0,
    Highpass1 = 1,
    Lowpass2 = 2,
    Highpass2 = 3,
    Bandpass = 6,
    Notch = 7,
    /// First-order high shelf; a cut moves the corner down by the cut's ratio.
    HighShelf1 = 8,
    /// First-order low shelf; a cut moves the corner up by the cut's ratio.
    LowShelf1 = 9,
    /// Zölzer peak/dip, symmetric in dB.
    Peak = 10,
}

/// Main c00bc2e0 (`A4`=out, `B4`=Hz, `A6`=gain dB, `A8`=Q, `B8`=type) at the
/// host rate: `K = tan(π f / fs)` and the original per-type formulas. The
/// original writes feedback terms with their sign flipped (`y += fb·y1`).
pub fn design(kind: Kind, sample_rate: f32, hz: f32, gain_db: f32, q: f32) -> Coefficients {
    let fs = f64::from(sample_rate);
    let tan = |hz: f64| (std::f64::consts::PI * hz.clamp(1.0, fs * 0.49) / fs).tan();
    let hz = f64::from(hz);
    let g = f64::from(gain_db);
    let q = f64::from(q).max(1e-3);
    let k = tan(hz);
    let k2 = k * k;
    // (b0, b1, b2, fb1, fb2)
    let (b0, b1, b2, fb1, fb2) = match kind {
        Kind::Lowpass1 => {
            let n = 1.0 / (1.0 + k);
            (k * n, k * n, 0.0, (1.0 - k) * n, 0.0)
        }
        Kind::Highpass1 => {
            let n = 1.0 / (1.0 + k);
            (n, -n, 0.0, (1.0 - k) * n, 0.0)
        }
        Kind::Lowpass2 | Kind::Highpass2 | Kind::Bandpass | Kind::Notch => {
            let n = 1.0 / (1.0 + k / q + k2);
            let fb1 = -2.0 * (k2 - 1.0) * n;
            let fb2 = -(1.0 - k / q + k2) * n;
            match kind {
                Kind::Lowpass2 => (k2 * n, 2.0 * k2 * n, k2 * n, fb1, fb2),
                Kind::Highpass2 => (n, -2.0 * n, n, fb1, fb2),
                Kind::Bandpass => (k / q * n, 0.0, -k / q * n, fb1, fb2),
                _ => ((1.0 + k2) * n, -fb1, (1.0 + k2) * n, fb1, fb2),
            }
        }
        Kind::HighShelf1 => {
            let gain = 10f64.powf(g / 20.0);
            let k = if g >= 0.0 { k } else { tan(hz * gain) };
            let n = 1.0 / (1.0 + k);
            ((gain + k) * n, (k - gain) * n, 0.0, (1.0 - k) * n, 0.0)
        }
        Kind::LowShelf1 => {
            let gain = 10f64.powf(g / 20.0);
            let k = if g >= 0.0 { k } else { tan(hz / gain) };
            let n = 1.0 / (1.0 + k);
            (
                (1.0 + k * gain) * n,
                (k * gain - 1.0) * n,
                0.0,
                (1.0 - k) * n,
                0.0,
            )
        }
        Kind::Peak => {
            let b1 = 2.0 * (k2 - 1.0);
            if g > 0.0 {
                let v = 10f64.powf(g / 20.0);
                let n = 1.0 / (1.0 + k / q + k2);
                let b1 = b1 * n;
                (
                    (1.0 + v * k / q + k2) * n,
                    b1,
                    (1.0 - v * k / q + k2) * n,
                    -b1,
                    -(1.0 - k / q + k2) * n,
                )
            } else {
                let v = 10f64.powf(-g / 20.0);
                let n = 1.0 / (1.0 + v * k / q + k2);
                let b1 = b1 * n;
                (
                    (1.0 + k / q + k2) * n,
                    b1,
                    (1.0 - k / q + k2) * n,
                    -b1,
                    -(1.0 - v * k / q + k2) * n,
                )
            }
        }
    };
    Coefficients {
        b0: b0 as f32,
        b1: b1 as f32,
        b2: b2 as f32,
        a1: -fb1 as f32,
        a2: -fb2 as f32,
    }
}

/// A ROM biquad in the original word order `b0, b1, b2, fb1, fb2`
/// (`y = b0 x + b1 x1 + b2 x2 + fb1 y1 + fb2 y2`), designed for 44.1 kHz and
/// re-derived for `sample_rate` through its analog prototype.
pub fn rom(words: [f32; 5], sample_rate: f32) -> Coefficients {
    Coefficients {
        b0: words[0],
        b1: words[1],
        b2: words[2],
        a1: -words[3],
        a2: -words[4],
    }
    .resample(ORIGINAL_RATE, sample_rate)
}

/// A per-sample multiplier recovered at 44.1 kHz (`y *= coefficient`) at the
/// host rate, keeping its time constant.
pub fn per_sample(coefficient: f32, sample_rate: f32) -> f32 {
    f64::from(coefficient).powf(f64::from(ORIGINAL_RATE) / f64::from(sample_rate)) as f32
}

/// A one-pole smoother `y = gain·x + pole·y` recovered at 44.1 kHz, at the host
/// rate with the same time constant and DC gain.
pub fn one_pole(gain: f32, pole: f32, sample_rate: f32) -> (f32, f32) {
    let new_pole = per_sample(pole, sample_rate);
    (gain * (1.0 - new_pole) / (1.0 - pole), new_pole)
}

#[inline]
pub fn clip1(x: f32) -> f32 {
    x.clamp(-1.0, 1.0)
}

/// log2 for positive finite `x`, max error ~2e-6.
#[inline]
pub fn fast_log2(x: f32) -> f32 {
    let bits = x.max(1e-30).to_bits();
    let mut exponent = ((bits >> 23) & 0xff) as i32 - 127;
    let mut mantissa = f32::from_bits((bits & 0x007f_ffff) | 0x3f80_0000);
    if mantissa > std::f32::consts::SQRT_2 {
        mantissa *= 0.5;
        exponent += 1;
    }
    // log2(m) = 2/ln2 · atanh(t), t = (m - 1)/(m + 1) in ±0.172.
    let t = (mantissa - 1.0) / (mantissa + 1.0);
    let t2 = t * t;
    let series = t * (2.885_39 + t2 * (0.961_797 + t2 * (0.577_078 + t2 * 0.412_198)));
    exponent as f32 + series
}

/// 2^x, relative error ~3e-6 over the f32 range.
#[inline]
pub fn fast_exp2(x: f32) -> f32 {
    let x = x.clamp(-126.0, 127.0);
    let whole = x.round();
    let f = x - whole;
    // 2^f for f in ±0.5.
    let p = 1.0
        + f * (std::f32::consts::LN_2
            + f * (0.240_226_5 + f * (0.055_504_1 + f * (0.009_618_1 + f * 0.001_333_4))));
    p * f32::from_bits(((whole as i32 + 127) as u32) << 23)
}

/// 20·log10(x).
#[inline]
pub fn fast_gain_to_db(x: f32) -> f32 {
    fast_log2(x) * 6.020_6
}

/// 10^(db/20).
#[inline]
pub fn fast_db_to_gain(db: f32) -> f32 {
    fast_exp2(db * 0.166_096_4)
}

/// The clip/ramp waveshaper the three compressors share: the input is scaled,
/// a direct term and a sign-dependent ramp and offset are each clipped to ±1,
/// and `output · (direct·|ramp − 1| + ramp·offset)` is returned. With the ROM
/// numbers this is a soft clipper (OrangeLim), an asymmetric saturator
/// (GrayComp) or a hard clipper with a cross-over knee (OptComp).
#[derive(Debug, Clone, Copy)]
pub struct Shaper {
    pub input: f32,
    pub direct: f32,
    /// Negative input: ramp slope, offset constant and offset slope.
    pub negative: [f32; 3],
    /// Positive input: ramp slope, offset constant and offset slope.
    pub positive: [f32; 3],
    pub output: f32,
}

impl Shaper {
    #[inline]
    pub fn process(&self, value: f32) -> f32 {
        let s = value * self.input;
        let [ramp, constant, slope] = if value < 0.0 {
            self.negative
        } else {
            self.positive
        };
        let direct = clip1(self.direct * s);
        let ramp = clip1(ramp * s);
        let offset = clip1(constant + s * slope);
        self.output * (direct * (ramp - 1.0).abs() + ramp * offset)
    }
}

/// Downward compressor curve of GrayComp/OrangeLim (coefficients 37..42):
/// above `threshold` the level is scaled by `slope` (1/ratio); inside the
/// optional knee `(knee_low, knee_high)` a parabola of `knee_gain` dB at the
/// width `knee_width` replaces it.
#[derive(Debug, Clone, Copy)]
pub struct GainCurve {
    pub threshold: f32,
    pub slope: f32,
    pub knee_low: f32,
    pub knee_high: f32,
    pub knee_width: f32,
    pub knee_gain: f32,
}

impl GainCurve {
    #[inline]
    pub fn gain_db(&self, level_db: f32) -> f32 {
        let difference = level_db - self.threshold;
        let mut gain = if level_db > self.threshold {
            difference * self.slope - difference
        } else {
            0.0
        };
        if self.knee_width != 0.0 && level_db < self.knee_high && level_db > self.knee_low {
            let q = (level_db - self.knee_low) / self.knee_width;
            gain = q * q * self.knee_gain;
        }
        gain
    }
}

/// The ramp of Main's `context[7]` setter: the coefficient moves linearly to
/// the new target within `duration` samples (16 ms in these editors).
#[derive(Debug, Clone, Copy)]
pub struct TimedRamp {
    pub current: f32,
    pub target: f32,
    step: f32,
    duration: f32,
}

impl TimedRamp {
    pub fn new(value: f32, seconds: f32, sample_rate: f32) -> Self {
        Self {
            current: value,
            target: value,
            step: 0.0,
            duration: (seconds * sample_rate).max(1.0),
        }
    }
    pub fn set(&mut self, target: f32) {
        if target != self.target {
            self.target = target;
            self.step = ((target - self.current).abs() / self.duration).max(target.abs() * 1e-6);
        }
    }
    pub fn jump(&mut self, value: f32) {
        self.current = value;
        self.target = value;
    }
    #[inline]
    pub fn tick(&mut self) -> f32 {
        let difference = self.target - self.current;
        if difference.abs() <= self.step {
            self.current = self.target;
        } else if difference > 0.0 {
            self.current += self.step;
        } else {
            self.current -= self.step;
        }
        self.current
    }
}

/// The editors' 16 ms ramp (`A6` = 705.6 samples at 44.1 kHz).
pub const RAMP_SECONDS: f32 = 0.016;

/// The Level editor of every model here: `curve(1.5, 0, 151, v, linear)`.
pub fn level(value: f32) -> f32 {
    value.clamp(0.0, 150.0) / 100.0
}

/// Linear interpolation of a breakpoint table at a fractional position.
pub fn interpolate(table: &[f32], position: f32) -> f32 {
    let last = table.len() - 1;
    let position = position.clamp(0.0, last as f32);
    let index = (position as usize).min(last.saturating_sub(1));
    let fraction = position - index as f32;
    table[index] + (table[(index + 1).min(last)] - table[index]) * fraction
}

/// The audio callback shared by every EQ of the group: mono models filter
/// `0.5 (L + R)`, stereo models each channel; the result is scaled by Level
/// (`c2`, ramped by the default editor step) and clamped to ±1 before it goes
/// to both outputs.
pub struct Equalizer<const N: usize> {
    left: [Biquad; N],
    right: [Biquad; N],
    stereo: bool,
    level: Ramp,
}

impl<const N: usize> Equalizer<N> {
    pub fn new(stereo: bool, sample_rate: f32) -> Self {
        Self {
            left: [Biquad::default(); N],
            right: [Biquad::default(); N],
            stereo,
            level: Ramp::new(1.0, DEFAULT_STEP, sample_rate),
        }
    }
    pub fn set_band(&mut self, band: usize, coefficients: Coefficients) {
        self.left[band].coefficients = coefficients;
        self.right[band].coefficients = coefficients;
    }
    #[cfg(test)]
    pub fn band(&self, band: usize) -> Coefficients {
        self.left[band].coefficients
    }
    pub fn set_level(&mut self, value: f32) {
        self.level.set(level(value));
    }
    pub fn jump_level(&mut self, value: f32) {
        self.level.jump(level(value));
    }
    /// Combined magnitude of all bands at `hz`.
    #[cfg(test)]
    pub fn magnitude(&self, sample_rate: f32, hz: f32) -> f32 {
        self.left
            .iter()
            .map(|band| band.coefficients.magnitude(sample_rate, hz))
            .product()
    }
    #[inline]
    pub fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let level = self.level.tick();
        if self.stereo {
            let mut l = left;
            let mut r = right;
            for (a, b) in self.left.iter_mut().zip(self.right.iter_mut()) {
                l = a.process(l);
                r = b.process(r);
            }
            [clip1(l * level), clip1(r * level)]
        } else {
            let mut x = 0.5 * (left + right);
            for band in &mut self.left {
                x = band.process(x);
            }
            let y = clip1(x * level);
            [y, y]
        }
    }
    pub fn clear(&mut self) {
        for band in self.left.iter_mut().chain(self.right.iter_mut()) {
            band.clear();
        }
    }
}

#[cfg(test)]
pub(crate) mod test_util {
    use crate::{Effect, Values};

    /// RMS gain of a sine through an effect after settling.
    pub fn sine_gain(effect: &mut dyn Effect, sample_rate: f32, hz: f32, amplitude: f32) -> f32 {
        let settle = (sample_rate * 0.3) as usize;
        let measure = (sample_rate * 0.2) as usize;
        let mut input = 0.0f64;
        let mut output = 0.0f64;
        for i in 0..settle + measure {
            let x = amplitude * (std::f32::consts::TAU * hz * i as f32 / sample_rate).sin();
            let y = effect.process([x, x]);
            if i >= settle {
                input += f64::from(x * x);
                output += f64::from(y[0] * y[0]);
            }
        }
        (output / input).sqrt() as f32
    }

    pub fn values(list: &[f32]) -> Values {
        let mut values = [0.0; crate::MAX_PARAMS];
        values[..list.len()].copy_from_slice(list);
        values
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsp::gain_to_db;

    #[test]
    fn fast_math_is_accurate() {
        for i in 1..2000 {
            let x = i as f32 * 0.37e-3;
            assert!((fast_log2(x) - x.log2()).abs() < 1e-4, "{x}");
        }
        for i in -400..400 {
            let x = i as f32 * 0.05;
            let exact = x.exp2();
            assert!(((fast_exp2(x) - exact) / exact).abs() < 2e-5, "{x}");
        }
    }

    #[test]
    fn designer_shapes_match_their_names() {
        let fs = 44_100.0;
        let peak = design(Kind::Peak, fs, 1000.0, 9.0, 1.0);
        assert!((gain_to_db(peak.magnitude(fs, 1000.0)) - 9.0).abs() < 0.01);
        assert!(gain_to_db(peak.magnitude(fs, 50.0)).abs() < 0.2);
        let dip = design(Kind::Peak, fs, 1000.0, -9.0, 1.0);
        assert!((gain_to_db(dip.magnitude(fs, 1000.0)) + 9.0).abs() < 0.01);
        let high = design(Kind::HighShelf1, fs, 2000.0, 6.0, 1.0);
        assert!((gain_to_db(high.magnitude(fs, 20_000.0)) - 6.0).abs() < 0.2);
        assert!(gain_to_db(high.magnitude(fs, 20.0)).abs() < 0.01);
        let low = design(Kind::LowShelf1, fs, 200.0, -10.0, 1.0);
        assert!((gain_to_db(low.magnitude(fs, 5.0)) + 10.0).abs() < 0.1);
        assert!(gain_to_db(low.magnitude(fs, 15_000.0)).abs() < 0.1);
        let lp = design(
            Kind::Lowpass2,
            fs,
            1000.0,
            0.0,
            std::f32::consts::FRAC_1_SQRT_2,
        );
        assert!((gain_to_db(lp.magnitude(fs, 1000.0)) + 3.01).abs() < 0.05);
        let hp = design(Kind::Highpass1, fs, 1000.0, 0.0, 1.0);
        assert!((gain_to_db(hp.magnitude(fs, 1000.0)) + 3.01).abs() < 0.05);
        let notch = design(Kind::Notch, fs, 1000.0, 0.0, 2.0);
        assert!(notch.magnitude(fs, 1000.0) < 1e-3);
        let band = design(Kind::Bandpass, fs, 1000.0, 0.0, 2.0);
        assert!((band.magnitude(fs, 1000.0) - 1.0).abs() < 1e-3);
    }

    #[test]
    fn designer_matches_the_geq_rom_shelf() {
        // GEQ.ZDL 0x80000440: the 12k band at +12 dB and −12 dB, 44.1 kHz.
        let boost = design(Kind::HighShelf1, 44_100.0, 12_000.0, 12.0, 1.0);
        assert!((boost.b0 - 2.386_840_8).abs() < 1e-4, "{boost:?}");
        assert!((boost.b1 + 1.317_272_1).abs() < 1e-4);
        assert!((boost.a1 - 0.069_568_89).abs() < 1e-4);
        let cut = design(Kind::HighShelf1, 44_100.0, 12_000.0, -12.0, 1.0);
        assert!((cut.b0 - 0.385_258_9).abs() < 1e-4, "{cut:?}");
        assert!((cut.b1 + 0.027_170_677).abs() < 1e-4);
        assert!((cut.a1 + 0.641_911_75).abs() < 1e-4);
    }

    #[test]
    fn timed_ramp_reaches_its_target_in_time() {
        let mut ramp = TimedRamp::new(0.0, 0.016, 48_000.0);
        ramp.set(1.0);
        let samples = (0.016 * 48_000.0) as usize;
        for _ in 0..samples - 2 {
            ramp.tick();
        }
        assert!(ramp.current < 1.0);
        for _ in 0..3 {
            ramp.tick();
        }
        assert_eq!(ramp.current, 1.0);
    }
}
