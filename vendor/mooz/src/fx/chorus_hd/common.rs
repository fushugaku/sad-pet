//! Pieces shared by the chorus_hd models: Main's filter designer (c00bc2e0,
//! reached through `context[25]`/`context[52]`), the ZDL-local `TBL_TO_VAL`
//! interpolator, ROM biquads, the Q31 triangle/polynomial LFO of the
//! SuperCho family and a few small filters.
use crate::dsp::{Biquad, Coefficients, flush};
use crate::zoom::ORIGINAL_RATE;

/// Filter types of Main c00bc2e0 (jump table at c00ef0f0). `B8` selects the
/// type; the editors pass the frequency in `B4`, the gain in dB in `A6` and
/// Q in `A8`. All are bilinear designs with `K = tan(pi f / fs)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// Type 0: first-order low-pass.
    LowPass1,
    /// Types 2/4: second-order low-pass.
    LowPass2,
    /// Types 3/5: second-order high-pass.
    HighPass2,
    /// Type 8: first-order high shelf, unity at DC and `gain` at Nyquist.
    /// Cuts move the pole down to `f * G` so boost and cut mirror each other.
    HighShelf1,
    /// Type 10: peaking EQ (the symmetric boost/cut form).
    Peaking,
}

/// Main c00bc2e0 at the instance rate.
pub fn design(kind: Kind, sample_rate: f32, hz: f32, gain_db: f32, q: f32) -> Coefficients {
    let fs = f64::from(sample_rate);
    let k_at = |f: f64| (std::f64::consts::PI * f.clamp(1.0, fs * 0.4999) / fs).tan();
    let hz = f64::from(hz);
    let q = f64::from(q).max(1e-3);
    let gain = f64::from(gain_db);
    let make = |b0: f64, b1: f64, b2: f64, a1: f64, a2: f64| Coefficients {
        b0: b0 as f32,
        b1: b1 as f32,
        b2: b2 as f32,
        a1: a1 as f32,
        a2: a2 as f32,
    };
    match kind {
        Kind::LowPass1 => {
            let k = k_at(hz);
            let d = 1.0 / (1.0 + k);
            make(k * d, k * d, 0.0, (k - 1.0) * d, 0.0)
        }
        Kind::LowPass2 | Kind::HighPass2 => {
            let k = k_at(hz);
            let k2 = k * k;
            let d = 1.0 / (1.0 + k / q + k2);
            let a1 = 2.0 * (k2 - 1.0) * d;
            let a2 = (1.0 - k / q + k2) * d;
            if kind == Kind::LowPass2 {
                make(k2 * d, 2.0 * k2 * d, k2 * d, a1, a2)
            } else {
                make(d, -2.0 * d, d, a1, a2)
            }
        }
        Kind::HighShelf1 => {
            let g = 10f64.powf(gain / 20.0);
            let k = if gain < 0.0 { k_at(hz * g) } else { k_at(hz) };
            let d = 1.0 / (1.0 + k);
            make((g + k) * d, (k - g) * d, 0.0, (k - 1.0) * d, 0.0)
        }
        Kind::Peaking => {
            let k = k_at(hz);
            let k2 = k * k;
            let v = 10f64.powf(gain.abs() / 20.0);
            let (num, den) = if gain > 0.0 {
                (v * k / q, k / q)
            } else {
                (k / q, v * k / q)
            };
            let d = 1.0 / (1.0 + den + k2);
            make(
                (1.0 + num + k2) * d,
                2.0 * (k2 - 1.0) * d,
                (1.0 - num + k2) * d,
                2.0 * (k2 - 1.0) * d,
                (1.0 - den + k2) * d,
            )
        }
    }
}

/// A ROM coefficient set in the callback's form
/// `y = b0 x + b1 x1 + b2 x2 + y1 * p1 + y2 * p2` (the feedback words are
/// stored with the sign the DSP adds), re-derived for `sample_rate`.
pub fn rom(b0: f32, b1: f32, b2: f32, p1: f32, p2: f32, sample_rate: f32) -> Coefficients {
    Coefficients {
        b0,
        b1,
        b2,
        a1: -p1,
        a2: -p2,
    }
    .resample(ORIGINAL_RATE, sample_rate)
}

/// The ZDL-local `TBL_TO_VAL(count, value, table, n)`: `n` breakpoints spread
/// evenly over `0..=maximum`, linear in between (fractional values allowed).
pub fn table(points: &[f32], value: f32, maximum: f32) -> f32 {
    let last = points.len() - 1;
    let position = (value / maximum).clamp(0.0, 1.0) * last as f32;
    let index = (position as usize).min(last.saturating_sub(1));
    let fraction = position - index as f32;
    if last == 0 {
        points[0]
    } else {
        points[index] + (points[index + 1] - points[index]) * fraction
    }
}

/// A 44.1 kHz Q32 phase increment rescaled to the instance rate.
pub fn rescale_increment(increment_at_44k: f32, sample_rate: f32) -> u32 {
    (f64::from(increment_at_44k) * f64::from(ORIGINAL_RATE) / f64::from(sample_rate))
        .clamp(0.0, f64::from(u32::MAX >> 1)) as u32
}

/// A per-sample one-pole coefficient of the original rate at the instance rate.
pub fn rescale_pole(coefficient_at_44k: f32, sample_rate: f32) -> f32 {
    let keep = 1.0 - f64::from(coefficient_at_44k);
    (1.0 - keep.powf(f64::from(ORIGINAL_RATE) / f64::from(sample_rate))) as f32
}

/// Q31 magnitude of a phase word as a fraction: a triangle 0 -> 1 -> 0 over a turn.
#[inline]
pub fn fold(phase: u32) -> f32 {
    (phase as i32).unsigned_abs() as f32 * (1.0 / 2_147_483_648.0)
}

/// Biquad applied to a mono signal; thin wrapper so models can swap coefficients.
#[derive(Debug, Clone, Copy, Default)]
pub struct Filter {
    pub biquad: Biquad,
}
impl Filter {
    pub fn set(&mut self, coefficients: Coefficients) {
        self.biquad.coefficients = coefficients;
    }
    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        self.biquad.process(x)
    }
    pub fn clear(&mut self) {
        self.biquad.clear();
    }
}

/// `y += (x - y) * k`, the one-pole filters the callbacks build from two words.
#[derive(Debug, Clone, Copy, Default)]
pub struct Lag {
    pub y: f32,
    pub k: f32,
}
impl Lag {
    pub fn new(k: f32) -> Self {
        Self { y: 0.0, k }
    }
    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        self.y = flush(self.y + (x - self.y) * self.k);
        self.y
    }
}

/// The SuperCho-family modulation (SuperCho, CE-Cho5, ANA234Cho, the chorus
/// stage of SilkyCho): a Q32 phase, the polynomial
/// `wave = 1 - scale * (linear |p| + square p^2)` of the folded phase, and two
/// one-pole smoothed targets, so that `delay = center + amplitude * wave`.
#[derive(Debug, Clone)]
pub struct BbdLfo {
    phase: u32,
    increment: u32,
    linear: f32,
    square: f32,
    /// c55 (c50 in ANA234): set by the rate editor.
    pub scale: f32,
    center: f32,
    center_target: f32,
    amplitude: f32,
    amplitude_target: f32,
    smoothing: f32,
    samples_per_ms: f32,
    fresh: bool,
}

impl BbdLfo {
    /// `linear`/`square` are the ROM polynomial words; the target smoothing
    /// is the ROM pair 0x00061546 / 0x7ff9eab8 per 44.1 kHz sample.
    pub fn new(sample_rate: f32, linear: f32, square: f32) -> Self {
        Self {
            phase: 0,
            increment: 0,
            linear,
            square,
            scale: 1.0,
            center: 0.0,
            center_target: 0.0,
            amplitude: 0.0,
            amplitude_target: 0.0,
            smoothing: rescale_pole(398_662.0 / 2_147_483_648.0, sample_rate),
            samples_per_ms: sample_rate / 1000.0,
            fresh: true,
        }
    }

    /// Phase increment in 44.1 kHz Q32 units (what the rate editors store).
    pub fn set_increment(&mut self, increment_at_44k: f32, sample_rate: f32) {
        self.increment = rescale_increment(increment_at_44k, sample_rate);
    }

    pub fn set_targets(&mut self, center_ms: f32, amplitude_ms: f32) {
        self.center_target = center_ms * self.samples_per_ms;
        self.amplitude_target = amplitude_ms * self.samples_per_ms;
        if self.fresh {
            self.fresh = false;
            self.center = self.center_target;
            self.amplitude = self.amplitude_target;
        }
    }

    /// Delay in samples at the instance rate.
    #[inline]
    pub fn tick(&mut self) -> f32 {
        self.phase = self.phase.wrapping_add(self.increment);
        self.amplitude += (self.amplitude_target - self.amplitude) * self.smoothing;
        self.center += (self.center_target - self.center) * self.smoothing;
        let m = fold(self.phase);
        let wave = 1.0 - (m * self.linear + m * m * self.square) * self.scale;
        wave * self.amplitude + self.center
    }

    pub fn clear(&mut self) {
        self.phase = 0;
        self.center = self.center_target;
        self.amplitude = self.amplitude_target;
    }
}

/// Linear ramp toward a target in a fixed number of instance samples, like
/// Main's `context[7]` (705.6 samples at 44.1 kHz in every editor here).
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
        let difference = self.target - self.current;
        if difference.abs() <= self.step {
            self.current = self.target;
        } else {
            self.current += self.step.copysign(difference);
        }
        self.current
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsp::gain_to_db;

    #[test]
    fn designer_matches_the_rom_sets_it_produced() {
        // SuperCho c35..c39: type 2 at 10 kHz, Q 0.78.
        let lp = design(Kind::LowPass2, 44_100.0, 10_000.0, 0.0, 0.78);
        assert!((lp.b0 - 0.261_436_67).abs() < 1e-5, "{lp:?}");
        assert!((lp.a1 + 0.178_092_06).abs() < 1e-5, "{lp:?}");
        assert!((lp.a2 - 0.223_838_76).abs() < 1e-5, "{lp:?}");
        // SuperCho c40..c44 ROM: type 0 at 16 kHz.
        let lp1 = design(Kind::LowPass1, 44_100.0, 16_000.0, 0.0, 0.78);
        assert!((lp1.b0 - 0.685_016_3).abs() < 1e-4, "{lp1:?}");
        // ANA234 c52..c56: a -3 dB high shelf at 3 kHz.
        let shelf = design(Kind::HighShelf1, 44_100.0, 3000.0, -3.0, 1.0);
        assert!((shelf.b0 - 0.746_582_5).abs() < 2e-3, "{shelf:?}");
        assert!((shelf.a1 + 0.735_413_8).abs() < 2e-3, "{shelf:?}");
    }

    #[test]
    fn shelf_and_peak_have_the_requested_gains() {
        let shelf = design(Kind::HighShelf1, 48_000.0, 1000.0, -6.0, 1.0);
        assert!(gain_to_db(shelf.magnitude(48_000.0, 20.0)).abs() < 0.1);
        assert!((gain_to_db(shelf.magnitude(48_000.0, 20_000.0)) + 6.0).abs() < 0.3);
        let peak = design(Kind::Peaking, 48_000.0, 140.0, 1.25, 1.0);
        assert!((gain_to_db(peak.magnitude(48_000.0, 140.0)) - 1.25).abs() < 0.05);
        let cut = design(Kind::Peaking, 48_000.0, 140.0, -4.0, 1.0);
        assert!((gain_to_db(cut.magnitude(48_000.0, 140.0)) + 4.0).abs() < 0.05);
    }

    #[test]
    fn table_interpolates_like_tbl_to_val() {
        let points = [0.24, 1.0, 2.6, 3.0, 3.6];
        assert_eq!(table(&points, 0.0, 100.0), 0.24);
        assert_eq!(table(&points, 100.0, 100.0), 3.6);
        assert!((table(&points, 62.5, 100.0) - 2.8).abs() < 1e-6);
    }
}
