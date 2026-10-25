//! Helpers shared by MangledSp and DualRev: the ZDL `TBL_TO_VAL` table
//! interpolation, the filter designs of Main's coefficient service
//! (`context[25]`, c00bc2e0), the parabolic LFO shape both callbacks compute in
//! Q31, and a glide for delay lengths that follow the Size knob.
use crate::dsp::Coefficients;
use std::f64::consts::PI;

/// `TBL_TO_VAL(count, value, table, len)`: linear interpolation of a float
/// table at `value * (len - 1) / (count - 1)`. The original reads one word past
/// the table when `value` exceeds `count - 1`; here the position is clamped.
pub fn table(count: u32, value: f32, table: &[f32]) -> f32 {
    let last = table.len() - 1;
    let position = (value.max(0.0) * last as f32 / (count.max(2) - 1) as f32).min(last as f32);
    let index = (position as usize).min(last.saturating_sub(1));
    let fraction = position - index as f32;
    if last == 0 {
        return table[0];
    }
    table[index] + (table[index + 1] - table[index]) * fraction
}

/// `K = tan(pi f / fs)` as the service computes it, with the corner kept below
/// Nyquist for any host rate.
fn prewarp(sample_rate: f32, hz: f32) -> f64 {
    let hz = f64::from(hz).clamp(1.0, f64::from(sample_rate) * 0.49);
    (PI * hz / f64::from(sample_rate)).tan()
}

/// The service stores `[b0, b1, b2, -a1, -a2]`; `Coefficients` keeps `a1, a2`.
fn stored(b0: f64, b1: f64, b2: f64, minus_a1: f64, minus_a2: f64) -> Coefficients {
    Coefficients {
        b0: b0 as f32,
        b1: b1 as f32,
        b2: b2 as f32,
        a1: -minus_a1 as f32,
        a2: -minus_a2 as f32,
    }
}

/// Type 2: second-order low-pass, `H(s) = 1 / (s^2 + s/Q + 1)` (bilinear, prewarped).
pub fn lowpass(sample_rate: f32, hz: f32, q: f32) -> Coefficients {
    let k = prewarp(sample_rate, hz);
    let kq = k / f64::from(q.max(1e-3));
    let k2 = k * k;
    let d = 1.0 / (1.0 + kq + k2);
    let b0 = k2 * d;
    stored(b0, 2.0 * b0, b0, 2.0 * (1.0 - k2) * d, -(1.0 - kq + k2) * d)
}

/// One-pole bilinear low-pass with unity DC gain (the ROM's first-order sets).
pub fn lowpass1(sample_rate: f32, hz: f32) -> Coefficients {
    let k = prewarp(sample_rate, hz);
    let d = 1.0 / (1.0 + k);
    stored(k * d, k * d, 0.0, (1.0 - k) * d, 0.0)
}

/// Type 9: first-order low shelf with gain `gain_db` below `hz` (the cut keeps
/// its zero at `hz` by moving the pole down to `hz / V`).
pub fn low_shelf1(sample_rate: f32, hz: f32, gain_db: f32) -> Coefficients {
    let v = 10f64.powf(f64::from(gain_db) / 20.0);
    let k = if gain_db < 0.0 {
        prewarp(sample_rate, (f64::from(hz) / v) as f32)
    } else {
        prewarp(sample_rate, hz)
    };
    let d = 1.0 / (1.0 + k);
    stored(
        (1.0 + k * v) * d,
        (k * v - 1.0) * d,
        0.0,
        (1.0 - k) * d,
        0.0,
    )
}

/// Type 8: first-order high shelf with gain `gain_db` above `hz` (a cut moves
/// the corner down to `hz * G`, mirroring type 9).
pub fn high_shelf1(sample_rate: f32, hz: f32, gain_db: f32) -> Coefficients {
    let g = 10f64.powf(f64::from(gain_db) / 20.0);
    let k = if gain_db < 0.0 {
        prewarp(sample_rate, (f64::from(hz) * g) as f32)
    } else {
        prewarp(sample_rate, hz)
    };
    let d = 1.0 / (1.0 + k);
    stored((g + k) * d, (k - g) * d, 0.0, (1.0 - k) * d, 0.0)
}

/// Type 10: peaking filter, `(s^2 + V s/Q + 1)/(s^2 + s/Q + 1)` for a boost and
/// the inverse for a cut, `V = 10^(|gain|/20)`.
pub fn peak(sample_rate: f32, hz: f32, q: f32, gain_db: f32) -> Coefficients {
    let k = prewarp(sample_rate, hz);
    let kq = k / f64::from(q.max(1e-3));
    let k2 = k * k;
    let v = 10f64.powf(f64::from(gain_db.abs()) / 20.0);
    let (num, den) = if gain_db > 0.0 { (v, 1.0) } else { (1.0, v) };
    let d = 1.0 / (1.0 + den * kq + k2);
    stored(
        (1.0 + num * kq + k2) * d,
        2.0 * (k2 - 1.0) * d,
        (1.0 - num * kq + k2) * d,
        -2.0 * (k2 - 1.0) * d,
        -(1.0 - den * kq + k2) * d,
    )
}

/// A first-order section `y = b0 x + b1 x1 - a1 y1` (types 8 and 9 and the
/// ROM one-pole low-passes), cheaper than a biquad in the feedback loops.
#[derive(Debug, Clone, Copy, Default)]
pub struct FirstOrder {
    b0: f32,
    b1: f32,
    a1: f32,
    x1: f32,
    y1: f32,
}

impl FirstOrder {
    pub fn set(&mut self, c: Coefficients) {
        self.b0 = c.b0;
        self.b1 = c.b1;
        self.a1 = c.a1;
    }
    #[inline(always)]
    pub fn process(&mut self, x: f32) -> f32 {
        let y = super::fast::flush(self.b0 * x + self.b1 * self.x1 - self.a1 * self.y1);
        self.x1 = x;
        self.y1 = y;
        y
    }
    pub fn clear(&mut self) {
        self.x1 = 0.0;
        self.y1 = 0.0;
    }
}

/// `N` [`FirstOrder`] sections with the same coefficients (one per comb).
#[derive(Debug, Clone, Copy)]
pub struct FirstOrderBank<const N: usize> {
    b0: f32,
    b1: f32,
    a1: f32,
    state: [(f32, f32); N],
}

impl<const N: usize> Default for FirstOrderBank<N> {
    fn default() -> Self {
        Self {
            b0: 0.0,
            b1: 0.0,
            a1: 0.0,
            state: [(0.0, 0.0); N],
        }
    }
}

impl<const N: usize> FirstOrderBank<N> {
    pub fn set(&mut self, c: Coefficients) {
        self.b0 = c.b0;
        self.b1 = c.b1;
        self.a1 = c.a1;
    }
    /// [`FirstOrder::process`] of section `i`.
    #[inline(always)]
    pub fn process(&mut self, i: usize, x: f32) -> f32 {
        let (x1, y1) = self.state[i];
        let y = super::fast::flush(self.b0 * x + self.b1 * x1 - self.a1 * y1);
        self.state[i] = (x, y);
        y
    }
    pub fn clear(&mut self) {
        self.state = [(0.0, 0.0); N];
    }
}

/// The LFO shape both callbacks evaluate on a Q31 phase `P` in [-1, 1):
/// `u = |P| - 1/2`, `0.5 + 2 u (1 - |u|)`, a parabolic raised cosine in
/// [0, 1] whose period is one wrap of the phase. `phase` is in turns.
#[inline(always)]
pub fn lfo_shape(phase: f32) -> f32 {
    let p = 2.0 * phase - 1.0;
    let u = p.abs() - 0.5;
    0.5 + 2.0 * u * (1.0 - u.abs())
}

/// Advance a phase in turns, wrapping at 1.
#[inline(always)]
pub fn advance(phase: &mut f32, increment: f32) {
    *phase += increment;
    if *phase >= 1.0 {
        *phase -= 1.0;
    }
}

/// Delay lengths (in host samples) that glide to new targets instead of
/// jumping, so Size can move without clicks. Each length moves linearly,
/// reaching its target in about 100 ms but never faster than half a sample
/// per sample (a fifth of an octave of pitch bend). While a glide runs the
/// owner reads its lines with interpolation; once every length has arrived it
/// is exact again and integer reads are used.
pub struct Glide<const N: usize> {
    pub current: [f32; N],
    target: [f32; N],
    step: [f32; N],
    glide_samples: f32,
    pub moving: bool,
}

impl<const N: usize> Glide<N> {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            current: [1.0; N],
            target: [1.0; N],
            step: [0.0; N],
            glide_samples: 0.1 * sample_rate,
            moving: false,
        }
    }
    /// Set the target lengths; `jump` lands on them immediately.
    pub fn set(&mut self, target: &[f32; N], jump: bool) {
        self.target = *target;
        if jump {
            self.current = *target;
            self.moving = false;
            return;
        }
        for i in 0..N {
            let distance = (self.target[i] - self.current[i]).abs();
            self.step[i] = (distance / self.glide_samples).clamp(1e-3, 0.5);
            self.moving |= distance > 0.0;
        }
    }
    /// One sample of glide; clears `moving` once every length has arrived.
    #[inline(always)]
    pub fn tick(&mut self) {
        let mut arrived = true;
        for ((current, &target), &step) in self.current.iter_mut().zip(&self.target).zip(&self.step)
        {
            let difference = target - *current;
            if difference.abs() <= step {
                *current = target;
            } else {
                arrived = false;
                *current += step.copysign(difference);
            }
        }
        self.moving = !arrived;
    }
}

/// Exponentially smoothed gains that stop costing anything once settled.
pub struct Gains<const N: usize> {
    pub value: [f32; N],
    target: [f32; N],
    coefficient: f32,
    settled: bool,
}

impl<const N: usize> Gains<N> {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            value: [0.0; N],
            target: [0.0; N],
            coefficient: crate::dsp::one_pole_coefficient(0.01, sample_rate),
            settled: true,
        }
    }
    pub fn set(&mut self, index: usize, target: f32, jump: bool) {
        self.target[index] = target;
        if jump {
            self.value[index] = target;
        } else if self.value[index] != target {
            self.settled = false;
        }
    }
    #[inline(always)]
    pub fn tick(&mut self) {
        if self.settled {
            return;
        }
        let mut settled = true;
        for (value, &target) in self.value.iter_mut().zip(&self.target) {
            let difference = target - *value;
            if difference.abs() <= 1e-6 * target.abs().max(1e-3) {
                *value = target;
            } else {
                settled = false;
                *value += difference * self.coefficient;
            }
        }
        self.settled = settled;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsp::gain_to_db;

    #[test]
    fn table_interpolates_like_tbl_to_val() {
        let t = [0.0, 0.375_837_4, 0.707_945_8, 0.944_060_9, 1.0];
        assert_eq!(table(101, 0.0, &t), 0.0);
        assert!((table(101, 50.0, &t) - 0.707_945_8).abs() < 1e-6);
        assert!((table(101, 100.0, &t) - 1.0).abs() < 1e-6);
        assert!((table(101, 12.5, &t) - 0.187_918_7).abs() < 1e-5);
        // count 100 reaches the last entry at 99 and is clamped above.
        assert!((table(100, 99.0, &t) - 1.0).abs() < 1e-6);
        assert!((table(100, 100.0, &t) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn lowpass_matches_the_rom_set_of_mangled_space() {
        // C99..C9D of MNGLD_SP.ZDL: the High editor's 3600 Hz, Q 0.75 low-pass.
        let c = lowpass(44_100.0, 3600.0, 0.75);
        assert!((c.b0 - 0.048_48).abs() < 1e-5, "{c:?}");
        assert!((c.b1 - 0.096_961).abs() < 1e-5);
        assert!((-c.a1 - 1.313_072).abs() < 1e-5);
        assert!((-c.a2 + 0.506_994).abs() < 1e-5);
    }

    #[test]
    fn shelves_and_peaks_reach_their_gain() {
        let shelf = low_shelf1(44_100.0, 50.0, -12.0);
        assert!((gain_to_db(shelf.magnitude(44_100.0, 2.0)) + 12.0).abs() < 0.2);
        assert!(gain_to_db(shelf.magnitude(44_100.0, 5000.0)).abs() < 0.1);
        let boost = low_shelf1(44_100.0, 130.0, 9.0);
        assert!((gain_to_db(boost.magnitude(44_100.0, 2.0)) - 9.0).abs() < 0.2);
        let top = high_shelf1(44_100.0, 3500.0, -6.5);
        assert!((gain_to_db(top.magnitude(44_100.0, 21_000.0)) + 6.5).abs() < 0.3);
        assert!(gain_to_db(top.magnitude(44_100.0, 20.0)).abs() < 0.05);
        // The ROM damping set C155..C159 of DUAL_REV.ZDL is this design: -2 dB,
        // K = 0.34053 at 44.1 kHz (5800 Hz before the cut's corner shift).
        let rom = high_shelf1(44_100.0, 5800.6, -2.0);
        assert!((rom.b0 - 0.846_573).abs() < 1e-4, "{rom:?}");
        assert!((rom.b1 + 0.338_536).abs() < 1e-4);
        assert!((-rom.a1 - 0.491_963).abs() < 1e-4);
        let bell = peak(48_000.0, 5200.0, 1.0, 8.0);
        assert!((gain_to_db(bell.magnitude(48_000.0, 5200.0)) - 8.0).abs() < 0.05);
        let cut = peak(48_000.0, 800.0, 1.0, -5.0);
        assert!((gain_to_db(cut.magnitude(48_000.0, 800.0)) + 5.0).abs() < 0.05);
        assert!(gain_to_db(cut.magnitude(48_000.0, 20.0)).abs() < 0.05);
    }

    #[test]
    fn lfo_shape_is_a_raised_cosine() {
        assert!(lfo_shape(0.5).abs() < 1e-6);
        assert!((lfo_shape(0.0) - 1.0).abs() < 1e-6);
        assert!((lfo_shape(0.25) - 0.5).abs() < 1e-6);
        for i in 0..100 {
            let p = i as f32 / 100.0;
            let ideal = 0.5 + 0.5 * (std::f32::consts::TAU * p).cos();
            assert!((lfo_shape(p) - ideal).abs() < 0.06, "{p}");
        }
    }
}
