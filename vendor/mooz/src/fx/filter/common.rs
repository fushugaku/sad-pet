//! Pieces shared by the filter group: rate conversion of the 44.1 kHz ROM
//! coefficients, first-order detector sections, the Chamberlin SVF frequency
//! map, the c3 "duck" the editors use while they swap a coefficient set, and
//! the pedal's Sense labels.
use crate::zoom::ORIGINAL_RATE;

/// GetString_AwaSens: Sense 0..19 is shown as -10..-1, 1..10 (the negative
/// half sweeps down, the positive half up).
pub const SENSE_LABELS: [&str; 20] = [
    "-10", "-9", "-8", "-7", "-6", "-5", "-4", "-3", "-2", "-1", "1", "2", "3", "4", "5", "6", "7",
    "8", "9", "10",
];

/// A raw firmware coefficient word as f32.
#[inline]
pub const fn w(bits: u32) -> f32 {
    f32::from_bits(bits)
}

/// A stepped pedal value (table index) from a possibly fractional value.
#[inline]
pub fn step(value: f32, max: usize) -> usize {
    if value.is_finite() {
        (value.round().max(0.0) as usize).min(max)
    } else {
        0
    }
}

/// A continuous pedal value clamped to `0..=max`.
#[inline]
pub fn clamp_value(value: f32, max: f32) -> f32 {
    if value.is_finite() {
        value.clamp(0.0, max)
    } else {
        0.0
    }
}

/// A per-sample decay factor of the original rate rescaled to `sample_rate`
/// (same time constant).
pub fn decay(factor: f32, sample_rate: f32) -> f32 {
    if factor <= 0.0 {
        factor
    } else {
        factor.powf(ORIGINAL_RATE / sample_rate)
    }
}

/// `y = a y + b x` at 44.1 kHz to the same time constant and DC gain at
/// `sample_rate`. Returns `(b, a)`.
pub fn one_pole(b: f32, a: f32, sample_rate: f32) -> (f32, f32) {
    let a2 = decay(a, sample_rate);
    if (1.0 - a).abs() < 1e-9 {
        (b, a2)
    } else {
        (b * (1.0 - a2) / (1.0 - a), a2)
    }
}

/// A first-order section `y = b0 x + b1 x[n-1] + a y[n-1]`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FirstOrder {
    pub b0: f32,
    pub b1: f32,
    pub a: f32,
}

impl FirstOrder {
    /// A section designed for 44.1 kHz re-derived for `sample_rate` through
    /// its analog prototype (bilinear transform inverted and reapplied).
    pub fn rom(b0: f32, b1: f32, a: f32, sample_rate: f32) -> Self {
        let r = f64::from(sample_rate) / f64::from(ORIGINAL_RATE);
        if (r - 1.0).abs() < 1e-6 {
            return Self { b0, b1, a };
        }
        let (b0, b1, p) = (f64::from(b0), f64::from(b1), f64::from(a));
        // H(z) = (b0 + b1 z^-1) / (1 - p z^-1), z^-1 = (1 - u)/(1 + u).
        let (n0, n1) = (b0 + b1, b0 - b1);
        let (d0, d1) = (1.0 - p, 1.0 + p);
        // u = s / 2fs scales by r; map back with u = (1 - z^-1)/(1 + z^-1).
        let num = [n0 + n1 * r, n0 - n1 * r];
        let den = [d0 + d1 * r, d0 - d1 * r];
        Self {
            b0: (num[0] / den[0]) as f32,
            b1: (num[1] / den[0]) as f32,
            a: (-den[1] / den[0]) as f32,
        }
    }

    /// Magnitude at `hz` (tests).
    #[cfg(test)]
    pub fn magnitude(&self, sample_rate: f32, hz: f32) -> f32 {
        let wv = std::f64::consts::TAU * f64::from(hz) / f64::from(sample_rate);
        let (c, s) = (wv.cos(), wv.sin());
        let nr = f64::from(self.b0) + f64::from(self.b1) * c;
        let ni = -f64::from(self.b1) * s;
        let dr = 1.0 - f64::from(self.a) * c;
        let di = f64::from(self.a) * s;
        ((nr * nr + ni * ni) / (dr * dr + di * di)).sqrt() as f32
    }
}

/// Maps a Chamberlin SVF frequency coefficient computed with the original
/// 44.1 kHz numbers (`f = 2 sin(pi fc / 44100)`) to the coefficient with the
/// same `fc` at the instance rate. A 129-point table over `f` in `[0, 2]`.
#[derive(Debug, Clone)]
pub struct SvfFrequency {
    table: [f32; Self::POINTS + 1],
}

impl SvfFrequency {
    const POINTS: usize = 128;

    pub fn new(sample_rate: f32) -> Self {
        let k = f64::from(ORIGINAL_RATE) / f64::from(sample_rate);
        let mut table = [0.0; Self::POINTS + 1];
        for (i, value) in table.iter_mut().enumerate() {
            let f = 2.0 * i as f64 / Self::POINTS as f64;
            let theta = (f * 0.5).min(1.0).asin();
            *value = (2.0 * (theta * k).min(std::f64::consts::FRAC_PI_2).sin()) as f32;
        }
        Self { table }
    }

    #[inline]
    pub fn map(&self, f: f32) -> f32 {
        let t = (f * (Self::POINTS as f32 * 0.5)).clamp(0.0, Self::POINTS as f32 - 1e-3);
        let i = t as usize;
        let a = self.table[i];
        a + (self.table[i + 1] - a) * (t - i as f32)
    }
}

/// The c3 fade the original editors run around a coefficient-set swap
/// (`context[7]` ramps of 705.6 samples = 16 ms): the output ducks to zero,
/// the new set is applied, and it fades back in.
#[derive(Debug, Clone, Copy)]
pub struct Duck {
    pub gain: f32,
    step: f32,
    falling: bool,
}

impl Duck {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            gain: 1.0,
            step: 1.0 / (705.6 * sample_rate / ORIGINAL_RATE),
            falling: false,
        }
    }

    pub fn start(&mut self) {
        self.falling = true;
    }

    pub fn is_falling(&self) -> bool {
        self.falling
    }

    pub fn reset(&mut self) {
        self.gain = 1.0;
        self.falling = false;
    }

    /// Advances one sample; true on the sample the gain reaches zero, when
    /// the caller applies its pending coefficient set.
    #[inline]
    pub fn tick(&mut self) -> bool {
        if self.falling {
            self.gain -= self.step;
            if self.gain <= 0.0 {
                self.gain = 0.0;
                self.falling = false;
                return true;
            }
        } else if self.gain < 1.0 {
            self.gain = (self.gain + self.step).min(1.0);
        }
        false
    }
}

/// Main's timed setter `context[7]` (c00cc94c): the value moves linearly to
/// the target in `A6` samples at 44.1 kHz (the editors pass 705.6, 16 ms),
/// with a step of at least `1e-6 |target|`.
#[derive(Debug, Clone, Copy)]
pub struct Glide {
    pub current: f32,
    target: f32,
    step: f32,
    samples: f32,
}

impl Glide {
    pub fn new(value: f32, samples_at_44k: f32, sample_rate: f32) -> Self {
        Self {
            current: value,
            target: value,
            step: 0.0,
            samples: (samples_at_44k * sample_rate / ORIGINAL_RATE).max(1.0),
        }
    }

    pub fn set(&mut self, target: f32) {
        self.target = target;
        self.step = ((self.current - target).abs() / self.samples).max(1e-6 * target.abs());
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

/// `context[7]` ramp length used by every editor here: 16 ms.
pub const GLIDE_16MS: f32 = 705.6;

/// The two Main ramp steps the editors pass to the float setter.
pub const STEP_DEFAULT: f32 = crate::zoom::DEFAULT_STEP; // 0x3a77c5f5
pub const STEP_FAST: f32 = w(0x3af7_c5ec);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_order_keeps_its_response_across_rates() {
        let at44 = FirstOrder {
            b0: w(0x3f7c_2496),
            b1: w(0xbf78_9347),
            a: w(0x3f74_b7dc),
        };
        let at96 = FirstOrder::rom(at44.b0, at44.b1, at44.a, 96_000.0);
        for hz in [50.0, 300.0, 1000.0, 5000.0] {
            let a = at44.magnitude(44_100.0, hz);
            let b = at96.magnitude(96_000.0, hz);
            assert!((a / b - 1.0).abs() < 0.05, "{hz}: {a} {b}");
        }
        assert_eq!(FirstOrder::rom(0.5, 0.25, 0.1, 44_100.0).b1, 0.25);
    }

    #[test]
    fn svf_frequency_map_keeps_the_cutoff() {
        let map = SvfFrequency::new(96_000.0);
        for fc in [100.0f32, 1000.0, 5000.0] {
            let f44 = 2.0 * (std::f32::consts::PI * fc / 44_100.0).sin();
            let f96 = 2.0 * (std::f32::consts::PI * fc / 96_000.0).sin();
            assert!((map.map(f44) / f96 - 1.0).abs() < 0.01, "{fc}");
        }
        let same = SvfFrequency::new(44_100.0);
        assert!((same.map(0.3) - 0.3).abs() < 1e-5);
    }

    #[test]
    fn duck_reaches_zero_once_and_recovers() {
        let mut duck = Duck::new(44_100.0);
        duck.start();
        let mut hits = 0;
        for _ in 0..2000 {
            if duck.tick() {
                hits += 1;
            }
        }
        assert_eq!(hits, 1);
        assert_eq!(duck.gain, 1.0);
    }

    #[test]
    fn one_pole_keeps_dc_gain() {
        let (b, a) = one_pole(w(0x3a83_126f), w(0x3f7f_be77), 96_000.0);
        assert!((b / (1.0 - a) - 1.0).abs() < 1e-3);
    }
}
