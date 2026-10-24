//! Building blocks shared by the reverb_hd group: the offset-addressed ring
//! the original callbacks use, first-order sections in the firmware's
//! `y = b0 x + b1 x1 + f1 y1` form, Main's filter designer (`context[25]`,
//! c00bc2e0), `TBL_TO_VAL` and the `Calc_*_Decay` comb gain.
use crate::dsp::{Coefficients, flush};
use crate::zoom::ORIGINAL_RATE;
use std::f64::consts::PI;

/// Scale a 44.1 kHz sample offset to the instance rate.
#[inline]
pub fn scale(offset: u32, sample_rate: f32) -> usize {
    (f64::from(offset) * f64::from(sample_rate) / f64::from(ORIGINAL_RATE)).round() as usize
}

/// The single circular buffer of the original callbacks. Every frame the
/// cursor moves down by one sample; a value written at offset `w` is read back
/// at offset `r` exactly `r - w` frames later, so each pair of neighbouring
/// write offsets delimits one delay line.
#[derive(Debug, Clone)]
pub struct Ring {
    buffer: Vec<f32>,
    mask: usize,
    position: usize,
}

impl Ring {
    /// Room for offsets `0..=span` (plus interpolation room).
    pub fn new(span: usize) -> Self {
        let length = (span + 4).next_power_of_two();
        Self {
            buffer: vec![0.0; length],
            mask: length - 1,
            position: 0,
        }
    }
    #[inline]
    pub fn advance(&mut self) {
        self.position = self.position.wrapping_sub(1) & self.mask;
    }
    #[inline]
    pub fn read(&self, offset: usize) -> f32 {
        self.buffer[(self.position + offset) & self.mask]
    }
    /// Linear interpolation between `offset` and `offset + 1`.
    #[inline]
    pub fn read_frac(&self, offset: f32) -> f32 {
        let whole = offset as usize;
        let fraction = offset - whole as f32;
        let a = self.read(whole);
        let b = self.read(whole + 1);
        a + (b - a) * fraction
    }
    #[inline]
    pub fn write(&mut self, offset: usize, value: f32) {
        let index = (self.position + offset) & self.mask;
        self.buffer[index] = value;
    }
    pub fn clear(&mut self) {
        self.buffer.fill(0.0);
    }
}

/// First-order section `y = b0 x + b1 x[n-1] + f1 y[n-1]` (the ROM layout,
/// with `b2 = f2 = 0`).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct FirstOrder {
    pub b0: f32,
    pub b1: f32,
    pub f1: f32,
}

impl FirstOrder {
    pub const fn new(b0: f32, b1: f32, f1: f32) -> Self {
        Self { b0, b1, f1 }
    }

    /// Re-derive a 44.1 kHz ROM section at `sample_rate`, keeping its analog
    /// prototype (inverse bilinear transform, then forward at the new rate).
    pub fn resample(self, sample_rate: f32) -> Self {
        let r = f64::from(sample_rate) / f64::from(ORIGINAL_RATE);
        if (r - 1.0).abs() < 1e-6 {
            return self;
        }
        let (b0, b1, f1) = (f64::from(self.b0), f64::from(self.b1), f64::from(self.f1));
        // H(u) = ((b0 + b1) + (b0 - b1) u) / ((1 - f1) + (1 + f1) u), u = s / 2fs0.
        let (n0, n1) = (b0 + b1, (b0 - b1) * r);
        let (d0, d1) = (1.0 - f1, (1.0 + f1) * r);
        let (nz0, nz1) = (n0 + n1, n0 - n1);
        let (dz0, dz1) = (d0 + d1, d0 - d1);
        Self {
            b0: (nz0 / dz0) as f32,
            b1: (nz1 / dz0) as f32,
            f1: (-dz1 / dz0) as f32,
        }
    }

    /// Magnitude at `hz` (tests).
    #[cfg(test)]
    pub fn magnitude(&self, sample_rate: f32, hz: f32) -> f32 {
        Coefficients {
            b0: self.b0,
            b1: self.b1,
            b2: 0.0,
            a1: -self.f1,
            a2: 0.0,
        }
        .magnitude(sample_rate, hz)
    }
}

/// Running state of one first-order section.
#[derive(Debug, Clone, Copy, Default)]
pub struct FirstOrderState {
    x1: f32,
    y1: f32,
}

impl FirstOrderState {
    #[inline]
    pub fn process(&mut self, c: &FirstOrder, x: f32) -> f32 {
        let y = flush(c.b0 * x + c.b1 * self.x1 + c.f1 * self.y1);
        self.x1 = x;
        self.y1 = y;
        y
    }
    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

/// A ROM biquad row `[b0, b1, b2, f1, f2]` (feedback signs as stored) to the
/// shared `Coefficients`.
pub fn rom_biquad(row: [f32; 5]) -> Coefficients {
    Coefficients {
        b0: row[0],
        b1: row[1],
        b2: row[2],
        a1: -row[3],
        a2: -row[4],
    }
}

/// The kinds of Main's filter designer (`context[25]`, `B8`). The whole
/// designer is kept; not every kind is used by this group.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Design {
    /// 0: first-order low-pass.
    LowPass1,
    /// 1: first-order high-pass.
    HighPass1,
    /// 2, 4: second-order low-pass.
    LowPass2,
    /// 3, 5: second-order high-pass.
    HighPass2,
    /// 6: band-pass, 0 dB peak.
    BandPass,
    /// 7: notch.
    Notch,
    /// 8: first-order high shelf (a cut moves the corner down by its ratio).
    HighShelf1,
    /// 10: peaking EQ.
    Peaking,
}

/// Main c00bc2e0: `K = tan(pi f / fs)` bilinear designs. Returns
/// `Coefficients` (with `a = -f`).
pub fn design(kind: Design, sample_rate: f32, hz: f32, q: f32, gain_db: f32) -> Coefficients {
    let fs = f64::from(sample_rate);
    let hz = f64::from(hz).clamp(1.0, fs * 0.499);
    let k = (PI * hz / fs).tan();
    let q = f64::from(q).max(1e-3);
    let k2 = k * k;
    let make = |b0: f64, b1: f64, b2: f64, a1: f64, a2: f64| Coefficients {
        b0: b0 as f32,
        b1: b1 as f32,
        b2: b2 as f32,
        a1: a1 as f32,
        a2: a2 as f32,
    };
    match kind {
        Design::LowPass1 => {
            let n = 1.0 / (1.0 + k);
            make(k * n, k * n, 0.0, -(1.0 - k) * n, 0.0)
        }
        Design::HighPass1 => {
            let n = 1.0 / (1.0 + k);
            make(n, -n, 0.0, -(1.0 - k) * n, 0.0)
        }
        Design::LowPass2 => {
            let d = 1.0 / (1.0 + k / q + k2);
            make(
                k2 * d,
                2.0 * k2 * d,
                k2 * d,
                2.0 * (k2 - 1.0) * d,
                (1.0 - k / q + k2) * d,
            )
        }
        Design::HighPass2 => {
            let d = 1.0 / (1.0 + k / q + k2);
            make(d, -2.0 * d, d, 2.0 * (k2 - 1.0) * d, (1.0 - k / q + k2) * d)
        }
        Design::BandPass => {
            let d = 1.0 / (1.0 + k / q + k2);
            make(
                k / q * d,
                0.0,
                -k / q * d,
                2.0 * (k2 - 1.0) * d,
                (1.0 - k / q + k2) * d,
            )
        }
        Design::Notch => {
            let d = 1.0 / (1.0 + k / q + k2);
            make(
                (1.0 + k2) * d,
                2.0 * (k2 - 1.0) * d,
                (1.0 + k2) * d,
                2.0 * (k2 - 1.0) * d,
                (1.0 - k / q + k2) * d,
            )
        }
        Design::HighShelf1 => {
            // A cut moves the corner down by the cut's ratio (`tan(pi f g / fs)`).
            let g = 10f64.powf(f64::from(gain_db) / 20.0);
            let kp = if g >= 1.0 {
                k
            } else {
                (PI * (hz * g).max(1.0) / fs).tan()
            };
            let n = 1.0 / (1.0 + kp);
            make((g + kp) * n, (kp - g) * n, 0.0, -(1.0 - kp) * n, 0.0)
        }
        Design::Peaking => {
            let v = 10f64.powf(f64::from(gain_db).abs() / 20.0);
            let (num, den) = if gain_db >= 0.0 {
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

/// A first-order design as a `FirstOrder` section.
pub fn design1(kind: Design, sample_rate: f32, hz: f32, gain_db: f32) -> FirstOrder {
    let c = design(kind, sample_rate, hz, 0.707, gain_db);
    FirstOrder::new(c.b0, c.b1, -c.a1)
}

/// `TBL_TO_VAL` of the ZDL modules: linear interpolation of an `n`-point
/// table spread over `0..count-1`. Values past `count - 1` read on into
/// `next` (the word that follows the table in ROM), as the original does.
pub fn tbl(table: &[f32], count: u32, value: f32, next: f32) -> f32 {
    let n = table.len();
    let last = (count - 1) as f32;
    let value = value.max(0.0);
    if value <= 0.0 {
        return table[0];
    }
    if (value - last).abs() < 1e-6 {
        return table[n - 1];
    }
    let position = value * (n - 1) as f32 / last;
    let index = (position as usize).min(n - 1);
    let fraction = position - index as f32;
    let a = table[index];
    let b = if index + 1 < n {
        table[index + 1]
    } else {
        next
    };
    a + (b - a) * fraction
}

/// `Calc_*_Decay`: the feedback gain of a comb of `delay_ms` reaching
/// `base` (0.01 = -40 dB, 0.001 = -60 dB) after `time_ms`.
#[inline]
pub fn comb_gain(base: f32, delay_ms: f32, time_ms: f32) -> f32 {
    if time_ms <= 0.0 {
        0.0
    } else {
        base.powf(delay_ms / time_ms)
    }
}

/// Linear ramps advanced once per control block (the float setter of Main
/// moves `step` per 44.1 kHz sample).
#[derive(Debug, Clone)]
pub struct Ramps<const N: usize> {
    pub current: [f32; N],
    pub target: [f32; N],
    step: f32,
}

impl<const N: usize> Ramps<N> {
    /// `step_at_44k` per 44.1 kHz sample, advanced every `block` samples.
    pub fn new(step_at_44k: f32, block: usize, sample_rate: f32) -> Self {
        Self {
            current: [0.0; N],
            target: [0.0; N],
            step: step_at_44k * ORIGINAL_RATE / sample_rate * block as f32,
        }
    }
    pub fn jump(&mut self) {
        self.current = self.target;
    }
    /// Move every value one block towards its target. Returns true while any
    /// value is still moving.
    #[inline]
    pub fn tick(&mut self) -> bool {
        let mut moving = false;
        for (current, &target) in self.current.iter_mut().zip(&self.target) {
            let difference = target - *current;
            if difference.abs() <= self.step {
                *current = target;
            } else {
                *current += self.step.copysign(difference);
                moving = true;
            }
        }
        moving
    }
}

/// Control block length for ramps and slow modulators.
pub const BLOCK: usize = 16;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring_delays_by_offset_difference() {
        let mut ring = Ring::new(100);
        for n in 0..50 {
            ring.advance();
            let read = ring.read(17);
            ring.write(5, n as f32);
            if n >= 12 {
                assert_eq!(read, (n - 12) as f32);
            }
        }
    }

    #[test]
    fn first_order_resample_keeps_shape() {
        // HD Hall lane shelf at HiDMP 70 (ROM-sweep values).
        let rom = FirstOrder::new(0.682_738_7, -0.512_021_4, 0.829_282_5);
        let at48 = rom.resample(48_000.0);
        for hz in [100.0, 1000.0, 5000.0] {
            let a = rom.magnitude(44_100.0, hz);
            let b = at48.magnitude(48_000.0, hz);
            assert!((a - b).abs() < 0.01, "{hz}: {a} {b}");
        }
    }

    #[test]
    fn designer_matches_rom_sweeps() {
        // Chamber Tone 73: second-order low-pass, round(1000 + 4000 * 0.73^0.6) Hz, Q 0.9.
        let c = design(Design::LowPass2, 44_100.0, 4312.0, 0.9, 0.0);
        assert!((c.b0 - 0.069_250_67).abs() < 2e-5, "{c:?}");
        assert!((-c.a1 - 1.237_873).abs() < 2e-4, "{c:?}");
        // HD Hall HiDMP 70: high shelf, 2000 Hz, -3.7 dB.
        let s = design1(Design::HighShelf1, 44_100.0, 2000.0, -3.7);
        assert!((s.b0 - 0.682_738_7).abs() < 2e-3, "{s:?}");
        assert!((s.f1 - 0.829_282_5).abs() < 2e-3, "{s:?}");
    }

    #[test]
    fn tbl_interpolates_like_the_rom_helper() {
        let table = [0.0, 10.0, 20.0];
        assert_eq!(tbl(&table, 101, 25.0, 0.0), 5.0);
        assert_eq!(tbl(&table, 101, 100.0, 0.0), 20.0);
        // Shimmer's mix editor passes count 100: value 100 reads past the end.
        assert!((tbl(&[1.0, 0.5], 100, 100.0, 0.0) - (0.5 - 0.5 * (1.0 / 99.0))).abs() < 1e-6);
    }
}
