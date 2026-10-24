//! Helpers shared by the dynamics group: the Main coefficient glides the
//! editors use, first-order sections in the callbacks' sign convention, rate
//! conversion of per-sample time constants and cheap log/exp for detectors.
use crate::dsp::flush;
use crate::zoom::{self, ORIGINAL_RATE, Ramp};

/// `A6` = 0x44306666 of the editors' `context[7]` calls: the step calculator
/// c00cc94c divides the distance to the new value by 705.6 samples at
/// 44.1 kHz, so every such coefficient glides linearly over 16 ms.
pub(crate) const GLIDE_SAMPLES: f32 = 705.6;

/// Rounded, clamped index of a stepped parameter.
#[inline]
pub(crate) fn index(value: f32, max: usize) -> usize {
    if value.is_finite() {
        (value.round().max(0.0) as usize).min(max)
    } else {
        0
    }
}

/// Every Level editor of the group: Main curve(1.5, 0, 151, v, linear),
/// i.e. `v / 100`, set through the float setter with step 0x3a77c5f5.
#[inline]
pub(crate) fn level(value: f32) -> f32 {
    if value.is_finite() {
        zoom::curve(1.5, 0.0, 151, value.clamp(0.0, 150.0), 0)
    } else {
        1.0
    }
}

/// Level coefficient (c2) ramp of the original float setter.
pub(crate) fn level_ramp(value: f32, sample_rate: f32) -> Ramp {
    Ramp::new(level(value), zoom::DEFAULT_STEP, sample_rate)
}

/// Snap a [`Ramp`] to its target (after `clear` or on the first `set`).
pub(crate) fn settle(ramp: &mut Ramp) {
    ramp.jump(ramp.target);
}

/// A coefficient written through `context[7]`: it moves linearly to the
/// requested value in [`GLIDE_SAMPLES`] (16 ms), whatever the distance.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Glide {
    pub current: f32,
    pub target: f32,
    step: f32,
    length: f32,
}

impl Glide {
    pub fn new(value: f32, sample_rate: f32) -> Self {
        Self {
            current: value,
            target: value,
            step: 0.0,
            length: (GLIDE_SAMPLES * sample_rate / ORIGINAL_RATE).max(1.0),
        }
    }

    pub fn set(&mut self, target: f32) {
        if target != self.target {
            self.target = target;
            // Main keeps a relative minimum step of |desired| * 1e-6.
            self.step = ((target - self.current).abs() / self.length)
                .max(target.abs() * 1e-6)
                .max(f32::MIN_POSITIVE);
        }
    }

    pub fn snap(&mut self) {
        self.current = self.target;
    }

    #[inline]
    pub fn tick(&mut self) -> f32 {
        if self.current != self.target {
            let difference = self.target - self.current;
            if difference.abs() <= self.step {
                self.current = self.target;
            } else {
                self.current += self.step.copysign(difference);
            }
        }
        self.current
    }
}

/// `44_100 / sample_rate`: the exponent that keeps a per-sample time constant.
#[inline]
pub(crate) fn rate_exponent(sample_rate: f32) -> f64 {
    f64::from(ORIGINAL_RATE) / f64::from(sample_rate)
}

/// A per-sample decay factor `x *= d` of the 44.1 kHz original at `sample_rate`.
pub(crate) fn decay(factor: f32, sample_rate: f32) -> f32 {
    if factor <= 0.0 || factor >= 1.0 {
        return factor;
    }
    f64::from(factor).powf(rate_exponent(sample_rate)) as f32
}

/// A follower coefficient `y += c (x - y)` at `sample_rate`.
pub(crate) fn follow(coefficient: f32, sample_rate: f32) -> f32 {
    if coefficient <= 0.0 || coefficient >= 1.0 {
        return coefficient;
    }
    1.0 - decay(1.0 - coefficient, sample_rate)
}

/// A one-pole recursion `y = gain x + pole y1` at `sample_rate`, keeping its
/// DC gain `gain / (1 - pole)` and its time constant.
pub(crate) fn one_pole(gain: f32, pole: f32, sample_rate: f32) -> (f32, f32) {
    if pole <= 0.0 || pole >= 1.0 {
        return (gain, pole);
    }
    let scaled = decay(pole, sample_rate);
    let gain = f64::from(gain) * (1.0 - f64::from(scaled)) / (1.0 - f64::from(pole));
    (gain as f32, scaled)
}

/// Saturation to ±1 used by the DualComp detector (CMPLTSP clamps).
#[inline]
pub(crate) fn clip(x: f32) -> f32 {
    x.clamp(-1.0, 1.0)
}

/// First-order section in the callbacks' convention:
/// `y = b0 x + b1 x[-1] + a1 y[-1]` (feedback added, as the ROM words are).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct FirstOrder {
    pub b0: f32,
    pub b1: f32,
    pub a1: f32,
    x1: f32,
    y1: f32,
}

impl FirstOrder {
    pub const IDENTITY: Self = Self::new(1.0, 0.0, 0.0);

    pub const fn new(b0: f32, b1: f32, a1: f32) -> Self {
        Self {
            b0,
            b1,
            a1,
            x1: 0.0,
            y1: 0.0,
        }
    }

    /// ROM words of a 44.1 kHz section.
    pub fn from_words(words: [u32; 3]) -> Self {
        Self::new(
            f32::from_bits(words[0]),
            f32::from_bits(words[1]),
            f32::from_bits(words[2]),
        )
    }

    /// The same analog prototype at `sample_rate`: the bilinear transform of
    /// 44.1 kHz is inverted and applied again (like `Coefficients::resample`).
    pub fn resample(self, sample_rate: f32) -> Self {
        let r = 1.0 / rate_exponent(sample_rate);
        if (r - 1.0).abs() < 1e-9 {
            return self;
        }
        let (b0, b1, a1) = (f64::from(self.b0), f64::from(self.b1), f64::from(self.a1));
        // Analog numerator (b0 - b1) s + (b0 + b1) k, denominator
        // (1 + a1) s + (1 - a1) k, mapped back with k' = r k.
        let n0 = (b0 - b1) * r + (b0 + b1);
        let n1 = -(b0 - b1) * r + (b0 + b1);
        let d0 = (1.0 + a1) * r + (1.0 - a1);
        let d1 = -(1.0 + a1) * r + (1.0 - a1);
        Self::new((n0 / d0) as f32, (n1 / d0) as f32, (-d1 / d0) as f32)
    }

    fn tangent(sample_rate: f32, hz: f32) -> f64 {
        let hz = f64::from(hz).clamp(1.0, f64::from(sample_rate) * 0.49);
        (std::f64::consts::PI * hz / f64::from(sample_rate)).tan()
    }

    /// Bilinear first-order low-pass (the DualComp `lpf_tbl` design).
    pub fn lowpass(sample_rate: f32, hz: f32) -> Self {
        let k = Self::tangent(sample_rate, hz);
        let b = k / (1.0 + k);
        Self::new(b as f32, b as f32, ((1.0 - k) / (1.0 + k)) as f32)
    }

    /// Bilinear first-order high-pass (the DualComp `hpf_tbl` design).
    pub fn highpass(sample_rate: f32, hz: f32) -> Self {
        let k = Self::tangent(sample_rate, hz);
        let a = 1.0 / (1.0 + k);
        Self::new(a as f32, -a as f32, ((1.0 - k) / (1.0 + k)) as f32)
    }

    /// Main's filter designer c00bc2e0, type 8 with a boost: first-order high
    /// shelf, unity at DC and `gain_db` at Nyquist, corner `hz`.
    pub fn high_shelf(sample_rate: f32, hz: f32, gain_db: f32) -> Self {
        let k = Self::tangent(sample_rate, hz);
        let g = 10f64.powf(f64::from(gain_db.max(0.0)) / 20.0);
        let norm = 1.0 / (1.0 + k);
        Self::new(
            ((g + k) * norm) as f32,
            ((k - g) * norm) as f32,
            ((1.0 - k) * norm) as f32,
        )
    }

    /// Replace the coefficients, keeping the histories.
    pub fn set_coefficients(&mut self, other: Self) {
        self.b0 = other.b0;
        self.b1 = other.b1;
        self.a1 = other.a1;
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let y = self.b0 * x + self.b1 * self.x1 + self.a1 * self.y1;
        self.x1 = x;
        self.y1 = flush(y);
        y
    }

    pub fn clear(&mut self) {
        self.x1 = 0.0;
        self.y1 = 0.0;
    }

    /// Magnitude response at `hz` (tests).
    #[cfg(test)]
    pub fn magnitude(&self, sample_rate: f32, hz: f32) -> f32 {
        let w = std::f64::consts::TAU * f64::from(hz) / f64::from(sample_rate);
        let (b0, b1, a1) = (f64::from(self.b0), f64::from(self.b1), f64::from(self.a1));
        let nr = b0 + b1 * w.cos();
        let ni = -b1 * w.sin();
        let dr = 1.0 - a1 * w.cos();
        let di = a1 * w.sin();
        ((nr * nr + ni * ni) / (dr * dr + di * di)).sqrt() as f32
    }
}

/// log2 for positive normal inputs (max error ~1e-4); zero and denormals give
/// about -127, never NaN.
#[inline]
pub(crate) fn fast_log2(x: f32) -> f32 {
    let bits = x.max(1e-30).to_bits();
    let exponent = ((bits >> 23) & 0xff) as i32 - 127;
    let m = f32::from_bits((bits & 0x007f_ffff) | 0x3f80_0000);
    // log2(m) = 2/ln2 * atanh((m - 1)/(m + 1)), m in [1, 2).
    let s = (m - 1.0) / (m + 1.0);
    let s2 = s * s;
    let series = s * (2.885_39 + s2 * (0.961_797 + s2 * (0.577_078 + s2 * 0.412_198)));
    exponent as f32 + series
}

/// 2^x (relative error < 2e-5), clamped to ±126.
#[inline]
pub(crate) fn fast_exp2(x: f32) -> f32 {
    let x = x.clamp(-126.0, 126.0);
    let whole = x.floor();
    let f = x - whole;
    // Taylor polynomial of 2^f on [0, 1), relative error < 2e-5.
    let p = 1.0
        + f * (std::f32::consts::LN_2
            + f * (0.240_226_5
                + f * (0.055_504_11
                    + f * (0.009_618_129 + f * (0.001_333_356 + f * 0.000_154_035)))));
    p * f32::from_bits(((whole as i32 + 127) as u32) << 23)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn glide_takes_sixteen_milliseconds() {
        let mut glide = Glide::new(0.0, 48_000.0);
        glide.set(1.0);
        let mut samples = 0;
        while glide.tick() < 1.0 {
            samples += 1;
        }
        assert!((samples as f32 - 0.016 * 48_000.0).abs() < 2.0, "{samples}");
    }

    #[test]
    fn resampled_first_order_keeps_its_response() {
        let at44 = FirstOrder::highpass(44_100.0, 660.0);
        let at96 = at44.resample(96_000.0);
        let direct = FirstOrder::highpass(96_000.0, 660.0);
        for hz in [50.0, 660.0, 5000.0] {
            let a = at96.magnitude(96_000.0, hz);
            let b = direct.magnitude(96_000.0, hz);
            assert!((a - b).abs() < 0.01, "{hz}: {a} {b}");
        }
        assert_eq!(at44.resample(44_100.0), at44);
    }

    #[test]
    fn rescaled_time_constants_match_in_seconds() {
        // 0.999 per sample at 44.1 kHz decays to 1/e in ~1000 samples (22.7 ms).
        let d = decay(0.999, 96_000.0);
        let n = (-1.0 / f64::from(d).ln()) / 96_000.0;
        assert!((n - 0.022_66).abs() < 1e-4, "{n}");
        let (g, p) = one_pole(0.01, 0.99, 48_000.0);
        assert!((g / (1.0 - p) - 1.0).abs() < 1e-4);
    }

    #[test]
    fn fast_math_is_accurate() {
        for i in 1..2000 {
            let x = i as f32 * 0.37e-3;
            assert!((fast_log2(x) - x.log2()).abs() < 2e-4, "{x}");
            let y = -(i as f32) * 0.03;
            let e = fast_exp2(y);
            assert!((e - y.exp2()).abs() <= y.exp2() * 3e-5, "{y}");
        }
        assert!(fast_log2(0.0).is_finite());
        assert!(fast_exp2(-1000.0) < 1e-37);
    }

    #[test]
    fn shelf_has_unity_dc_and_boosted_top() {
        let shelf = FirstOrder::high_shelf(44_100.0, 4000.0, 12.0);
        assert!((shelf.magnitude(44_100.0, 1.0) - 1.0).abs() < 1e-3);
        assert!((gain_db(shelf.magnitude(44_100.0, 22_000.0)) - 12.0).abs() < 0.1);
    }

    fn gain_db(g: f32) -> f32 {
        20.0 * g.log10()
    }
}
