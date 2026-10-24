//! Pieces shared by the filter_seq models: conversion of the 44.1 kHz
//! per-sample coefficients to the host rate, Main's integer parameter curve,
//! the timed ramp (`context[7]`) and the mute → switch → unmute sequence the
//! editors run around structural changes, the clipped Chamberlin filter of
//! SeqFLTR/RndmFLTR/fCycle/SlowFLTR and the attack detector of
//! SlowFLTR/SlowATTCK.
use crate::{
    dsp::{Biquad, Coefficients, flush},
    zoom::{self, ORIGINAL_RATE},
};

/// 2^31, the Q31 scale.
pub const Q31: f64 = 2_147_483_648.0;
/// 2^32, one cycle of an integer phase accumulator.
pub const CYCLE: f64 = 4_294_967_296.0;

/// The float setter step 0x3af7c5ec used by several editors (`context[8]`).
pub const FAST_STEP: f32 = 1.887_99e-3;

/// Main c00ca160 (`context[22]`): the integer parameter curve, as a value in
/// the units of `maximum`/`minimum`. Like the original's SPINT, the shaped
/// fraction saturates at one (several editors pass a count one short).
pub fn int_curve(maximum: f64, minimum: f64, count: u32, selected: f32, shape: u8) -> f64 {
    let t = (f64::from(selected) / f64::from(count.saturating_sub(1).max(1))).max(0.0);
    let shaped = match shape {
        1 => t * t,
        3 => t.sqrt(),
        6 => t * t * t,
        _ => t,
    };
    minimum + (maximum - minimum) * shaped.min(1.0)
}

/// Hz of a phase increment that completes a cycle every 2^32 at 44.1 kHz.
pub fn q32_hz(increment: f64) -> f32 {
    (increment / CYCLE * f64::from(ORIGINAL_RATE)) as f32
}

/// A rate parameter with a tempo tail: `normal` up to `threshold`, the Main
/// LFO divisions (`context[23]`) above it.
pub fn sync_hz(value: f32, threshold: u16, bpm: f32, normal: impl Fn(f32) -> f32) -> f32 {
    if zoom::is_synced(value, threshold) {
        zoom::rate_note_hz(zoom::sync_index(value, threshold), bpm)
    } else {
        normal(value.clamp(0.0, f32::from(threshold)))
    }
}

/// Phase increment (2^32 per cycle) for `hz` at `sample_rate`.
pub fn increment(hz: f32, sample_rate: f32) -> u32 {
    (f64::from(hz) / f64::from(sample_rate) * CYCLE).clamp(0.0, CYCLE / 2.0) as u32
}

/// Ratio of the original rate to the host rate.
pub fn ratio(sample_rate: f32) -> f32 {
    ORIGINAL_RATE / sample_rate
}

/// A per-sample decay factor of the original at `sample_rate`.
pub fn pole(p: f32, sample_rate: f32) -> f32 {
    if p <= 0.0 {
        0.0
    } else {
        f64::from(p).powf(f64::from(ratio(sample_rate))) as f32
    }
}

/// `y = gain * x + p * y` at `sample_rate`, keeping its DC gain.
pub fn one_pole(gain: f32, p: f32, sample_rate: f32) -> (f32, f32) {
    let converted = pole(p, sample_rate);
    let dc = gain / (1.0 - p);
    (dc * (1.0 - converted), converted)
}

/// A first-order section `y = b0 x + b1 x1 + p y1` of the original, re-derived
/// for `sample_rate` from its analog prototype.
pub fn first_order(b0: f32, b1: f32, p: f32, sample_rate: f32) -> Biquad {
    Biquad::new(
        Coefficients {
            b0,
            b1,
            b2: 0.0,
            a1: -p,
            a2: 0.0,
        }
        .resample(ORIGINAL_RATE, sample_rate),
    )
}

/// The SVF frequency word `f = 2 sin(pi fc / fs)` of the original, for the
/// same cutoff at `sample_rate`.
pub fn svf_f(f44: f32, sample_rate: f32) -> f32 {
    let angle = (f64::from(f44) * 0.5).clamp(0.0, 1.0).asin() * f64::from(ratio(sample_rate));
    (2.0 * angle.min(std::f64::consts::FRAC_PI_2).sin()) as f32
}

const TUNING_POINTS: usize = 256;

/// [`svf_f`] as a table for per-sample use over `f44` in `0..1`.
#[derive(Debug, Clone)]
pub struct SvfTuning {
    table: [f32; TUNING_POINTS + 2],
}

impl SvfTuning {
    pub fn new(sample_rate: f32) -> Self {
        let mut table = [0.0; TUNING_POINTS + 2];
        for (i, value) in table.iter_mut().enumerate() {
            *value = svf_f(i as f32 / TUNING_POINTS as f32, sample_rate);
        }
        Self { table }
    }

    #[inline]
    pub fn map(&self, f44: f32) -> f32 {
        let x = f44.clamp(0.0, 1.0) * TUNING_POINTS as f32;
        let i = x as usize;
        let fraction = x - i as f32;
        let (a, b) = (self.table[i], self.table[i + 1]);
        a + (b - a) * fraction
    }
}

#[inline]
pub fn clip(x: f32) -> f32 {
    x.clamp(-1.0, 1.0)
}

/// The Chamberlin state-variable filter of the FLT modules, every node clipped
/// to ±1 as the callbacks do.
#[derive(Debug, Clone, Copy, Default)]
pub struct ClippedSvf {
    pub band: f32,
    pub low: f32,
}

impl ClippedSvf {
    /// Returns (high, band, low).
    #[inline]
    pub fn process(&mut self, input: f32, f: f32, damping: f32) -> (f32, f32, f32) {
        let high = clip(input - damping * self.band - self.low);
        let band = clip(self.band + f * high);
        let low = clip(self.low + f * band);
        self.band = flush(band);
        self.low = flush(low);
        (high, band, low)
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

/// Samples of the timed ramp Main's `context[7]` runs (A6 = 705.6 at 44.1 kHz).
pub fn ramp_samples(sample_rate: f32) -> f32 {
    705.6 / ratio(sample_rate)
}

/// Main c00cc94c (`context[7]`): reach the target linearly in 16 ms.
#[derive(Debug, Clone, Copy)]
pub struct TimedRamp {
    pub current: f32,
    target: f32,
    step: f32,
    samples: f32,
}

impl TimedRamp {
    pub fn new(value: f32, sample_rate: f32) -> Self {
        Self {
            current: value,
            target: value,
            step: 0.0,
            samples: ramp_samples(sample_rate),
        }
    }

    pub fn set(&mut self, target: f32) {
        if target != self.target {
            self.target = target;
            self.step = (target - self.current).abs() / self.samples;
        }
    }

    pub fn jump(&mut self, value: f32) {
        self.current = value;
        self.target = value;
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

/// The editors' sequence around a structural change (type, pattern, wave):
/// ramp the wet gain (`C[3]`) to zero in 16 ms, apply the change, ramp back
/// up through the scheduled `onf_aft` callback.
#[derive(Debug, Clone, Copy)]
pub struct SwitchGate {
    pub gain: f32,
    step: f32,
    closing: bool,
    opening: bool,
}

impl SwitchGate {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            gain: 1.0,
            step: 1.0 / ramp_samples(sample_rate),
            closing: false,
            opening: false,
        }
    }

    /// Start (or restart) the closing ramp.
    pub fn request(&mut self) {
        self.closing = true;
        self.opening = false;
    }

    /// Advance one sample; true exactly when the gain reached zero and the
    /// pending change must be applied.
    #[inline]
    pub fn tick(&mut self) -> bool {
        if self.closing {
            self.gain -= self.step;
            if self.gain <= 0.0 {
                self.gain = 0.0;
                self.closing = false;
                self.opening = true;
                return true;
            }
        } else if self.opening {
            self.gain += self.step;
            if self.gain >= 1.0 {
                self.gain = 1.0;
                self.opening = false;
            }
        }
        false
    }
}

/// `log2` for positive finite `x`, accurate to ~1e-7 (exact near 1).
#[inline]
pub fn log2(x: f32) -> f32 {
    let bits = x.to_bits();
    let mut exponent = ((bits >> 23) & 0xff) as i32 - 127;
    let mut m = f32::from_bits((bits & 0x007f_ffff) | 0x3f80_0000);
    if m > std::f32::consts::SQRT_2 {
        m *= 0.5;
        exponent += 1;
    }
    let t = (m - 1.0) / (m + 1.0);
    let t2 = t * t;
    let series = t * (2.885_39 + t2 * (0.961_796_7 + t2 * (0.577_078 + t2 * 0.412_198_6)));
    exponent as f32 + series
}

/// `2^y`, relative error ~1e-7 (exact near 0).
#[inline]
pub fn exp2(y: f32) -> f32 {
    let y = y.clamp(-126.0, 126.0);
    let whole = y.round_ties_even();
    let f = y - whole;
    let p = 1.0
        + f * (std::f32::consts::LN_2
            + f * (0.240_226_5
                + f * (0.055_504_11
                    + f * (0.009_618_129 + f * (0.001_333_355 + f * 1.540_353e-4)))));
    p * f32::from_bits(((whole as i32 + 127) as u32) << 23)
}

/// `x^e` for `x` in `0..=1` (Main's `pow`, c00d92c0, as the callbacks use it).
#[inline]
pub fn pow01(x: f32, e: f32) -> f32 {
    if x <= 0.0 {
        0.0
    } else {
        exp2(e * log2(x.min(1.0)))
    }
}

/// The attack detector of SlowFLTR and SlowATTCK (identical ROM constants):
/// 100 Hz high-pass, decaying peak, fast and slow followers; a rising attack
/// (fast above slow) restarts a linear timer that climbs to one in `Time`.
#[derive(Debug, Clone)]
pub struct AttackDetector {
    filter: Biquad,
    peak: f32,
    fast: f32,
    slow: f32,
    pub phase: f32,
    step: f32,
    peak_decay: f32,
    fast_gain: f32,
    fast_pole: f32,
    slow_gain: f32,
    slow_pole: f32,
    sample_rate: f32,
}

impl AttackDetector {
    pub fn new(sample_rate: f32) -> Self {
        // ROM c12..c16 (b0, b1, b2, -a1, -a2), c7 decay, c8 sensitivity,
        // c17/c18 fast and c19/c20 slow followers.
        let filter = first_order(
            f32::from_bits(0x3f7e_306e),
            f32::from_bits(0xbf7e_306e),
            f32::from_bits(0x3f7c_60dc),
            sample_rate,
        );
        let (fast_gain, fast_pole) = one_pole(
            f32::from_bits(0x3c8a_2869) * 0.75,
            f32::from_bits(0x3f7b_aebd),
            sample_rate,
        );
        let (slow_gain, slow_pole) = one_pole(
            f32::from_bits(0x3a82_f7aa),
            f32::from_bits(0x3f7f_be84),
            sample_rate,
        );
        Self {
            filter,
            peak: 0.0,
            fast: 0.0,
            slow: 0.0,
            phase: 0.0,
            step: 0.0,
            peak_decay: pole(f32::from_bits(0x3f7f_fc01), sample_rate),
            fast_gain,
            fast_pole,
            slow_gain,
            slow_pole,
            sample_rate,
        }
    }

    /// The timer climbs from 0 to 1 in `seconds` (`c9 = 1 / (T * 44100)`).
    pub fn set_time(&mut self, seconds: f32) {
        self.step = 1.0 / (seconds.max(1e-3) * self.sample_rate);
    }

    /// Feed one detector sample, return the timer (0..1).
    #[inline]
    pub fn process(&mut self, input: f32) -> f32 {
        let magnitude = self.filter.process(input).abs();
        let decayed = self.peak * self.peak_decay;
        self.peak = flush(if decayed < magnitude {
            magnitude
        } else {
            decayed
        });
        self.fast = flush(self.fast * self.fast_pole + self.fast_gain * self.peak);
        self.slow = flush(self.slow * self.slow_pole + self.slow_gain * self.peak);
        self.phase = if self.slow < self.fast {
            0.0
        } else {
            (self.phase + self.step).min(1.0)
        };
        self.phase
    }

    pub fn clear(&mut self) {
        self.filter.clear();
        self.peak = 0.0;
        self.fast = 0.0;
        self.slow = 0.0;
        self.phase = 0.0;
    }
}

/// Values of a model's parameters that changed since the last `set`.
#[derive(Debug, Clone, Copy)]
pub struct Changes {
    last: crate::Values,
    bpm: f32,
    first: bool,
}

impl Changes {
    pub fn new() -> Self {
        Self {
            last: [f32::NAN; crate::MAX_PARAMS],
            bpm: f32::NAN,
            first: true,
        }
    }

    /// Record `values`/`bpm`; returns a mask of changed indices, whether the
    /// tempo changed, and whether this is the first call.
    pub fn update(&mut self, values: &crate::Values, bpm: f32) -> (u16, bool, bool) {
        let mut mask = 0u16;
        for (i, (last, value)) in self.last.iter_mut().zip(values).enumerate() {
            if last.to_bits() != value.to_bits() {
                mask |= 1 << i;
                *last = *value;
            }
        }
        let tempo = self.bpm.to_bits() != bpm.to_bits();
        self.bpm = bpm;
        let first = self.first;
        self.first = false;
        (mask, tempo, first)
    }
}

impl Default for Changes {
    fn default() -> Self {
        Self::new()
    }
}

/// Round a discrete parameter into `0..=max`.
pub fn index(value: f32, max: usize) -> usize {
    (value.round().max(0.0) as usize).min(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fast_math_is_accurate() {
        for i in 1..2000 {
            let x = i as f32 / 2000.0;
            for e in [0.4f32, 1.0, 1.088_435, 2.5, 4.0] {
                let exact = (x as f64).powf(e as f64) as f32;
                let approx = pow01(x, e);
                assert!((approx - exact).abs() <= exact * 2e-6 + 1e-9, "{x} {e}");
            }
        }
        // Poles close to one keep their distance from one.
        let p = 0.999_3f32;
        let exact = 1.0 - (p as f64).powf(44_100.0 / 48_000.0);
        let approx = 1.0 - pow01(p, 44_100.0 / 48_000.0) as f64;
        assert!((approx - exact).abs() < exact * 2e-3);
    }

    #[test]
    fn svf_tuning_keeps_the_cutoff() {
        // 0x3e9166f5 is ~2 kHz at 44.1 kHz.
        let f44 = f32::from_bits(0x3e91_66f5);
        let hz = |f: f32, rate: f32| (f as f64 * 0.5).asin() * rate as f64 / std::f64::consts::PI;
        let at48 = svf_f(f44, 48_000.0);
        assert!((hz(f44, 44_100.0) - hz(at48, 48_000.0)).abs() < 0.01);
        assert!((hz(f44, 44_100.0) - 2000.0).abs() < 5.0);
        let table = SvfTuning::new(96_000.0);
        for i in 0..100 {
            let f = i as f32 / 100.0 * 0.8;
            assert!((table.map(f) - svf_f(f, 96_000.0)).abs() < 2e-5, "{f}");
        }
        let same = SvfTuning::new(44_100.0);
        assert!((same.map(0.3) - 0.3).abs() < 1e-6);
    }

    #[test]
    fn integer_curve_saturates_like_spint() {
        assert_eq!(int_curve(100.0, 0.0, 49, 48.0, 1), 100.0);
        assert_eq!(int_curve(100.0, 0.0, 49, 50.0, 1), 100.0);
        assert!((int_curve(100.0, 0.0, 101, 50.0, 1) - 25.0).abs() < 1e-9);
    }

    #[test]
    fn gate_closes_applies_and_opens() {
        let mut gate = SwitchGate::new(44_100.0);
        gate.request();
        let mut applied_at = None;
        for i in 0..2000 {
            if gate.tick() {
                applied_at = Some(i);
            }
        }
        let at = applied_at.unwrap();
        assert!((700..=710).contains(&at), "{at}");
        assert_eq!(gate.gain, 1.0);
    }
}
