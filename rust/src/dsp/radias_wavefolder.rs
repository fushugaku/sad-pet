//! Rustias OSC 4 Wavefolder, circuit/expanded.rs at 91cae3f.
//! Keep its f64, four internal steps, smoothing, five transfer functions and
//! DC blocker together. Every Resynth carrier owns independent history.
use crate::resynth::{FoldShape, Waveform};

#[derive(Default)]
pub(super) struct Wavefolder {
    pub phase: f64,
    output: f64,
    dc_input: f64,
    dc_output: f64,
    depth: Option<f64>,
    bias: Option<f64>,
}
impl Wavefolder {
    pub fn next(
        &mut self,
        increment: f64,
        wave: Waveform,
        shape: FoldShape,
        controls: [f64; 2],
        sync: bool,
    ) -> f32 {
        if sync {
            self.phase = 0.0;
        }
        let depth = self.depth.get_or_insert(controls[0]);
        *depth += (controls[0] - *depth) * 0.02;
        let depth = *depth;
        let bias = self.bias.get_or_insert(controls[1]);
        *bias += (controls[1] - *bias) * 0.02;
        let bias = *bias;
        let mut sum = 0.0;
        for _ in 0..4 {
            self.phase = (self.phase + increment * 0.25).fract();
            let x = match wave {
                Waveform::Saw => self.phase * 2.0 - 1.0,
                Waveform::Pulse => {
                    if self.phase < 0.5 {
                        1.0
                    } else {
                        -1.0
                    }
                }
                Waveform::Triangle => 1.0 - 4.0 * (self.phase - 0.5).abs(),
                _ => super::fast::sin(self.phase * std::f64::consts::TAU),
            };
            let driven = x * (1.0 + depth * 11.0) + bias * 2.0;
            // y - floor(y) is rem_euclid(y, 1.0) without an fmod call, for every
            // finite y up to the sign of a zero result, which `- 0.5` and abs()
            // then discard.
            let triangle = |v: f64| {
                let y = (v + 1.0) * 0.25;
                1.0 - 4.0 * ((y - y.floor()) - 0.5).abs()
            };
            let folded = match shape {
                FoldShape::Triangle => triangle(driven),
                FoldShape::Sine => super::fast::sin(driven * std::f64::consts::FRAC_PI_2),
                FoldShape::Soft => soft(triangle(driven) * 1.5),
                FoldShape::Asymmetric => triangle(driven + depth),
                FoldShape::Rectify => driven.abs().rem_euclid(2.0) - 1.0,
            };
            self.output += (folded - self.output) * 0.6;
            sum += self.output * 0.25;
        }
        let dc = sum - self.dc_input + 0.9995 * self.dc_output;
        self.dc_input = sum;
        self.dc_output = dc;
        dc.clamp(-1.0, 1.0) as f32
    }
    /// `next` for two carriers of one layer at once: each lane runs the same
    /// operations in the same order, two values per vector instruction.
    pub fn next_pair(
        pair: [&mut Self; 2],
        increments: [f64; 2],
        wave: Waveform,
        shape: FoldShape,
        controls: [f64; 2],
        syncs: [bool; 2],
    ) -> [f32; 2] {
        let [a, b] = pair;
        let mut depth = [0.0; 2];
        let mut bias = [0.0; 2];
        for (lane, folder) in [&mut *a, &mut *b].into_iter().enumerate() {
            if syncs[lane] {
                folder.phase = 0.0;
            }
            let d = folder.depth.get_or_insert(controls[0]);
            *d += (controls[0] - *d) * 0.02;
            depth[lane] = *d;
            let s = folder.bias.get_or_insert(controls[1]);
            *s += (controls[1] - *s) * 0.02;
            bias[lane] = *s;
        }
        let depth = Lanes::new(depth);
        let gain = Lanes::splat(1.0) + depth * Lanes::splat(11.0);
        let offset = Lanes::new(bias) * Lanes::splat(2.0);
        let step = Lanes::new(increments) * Lanes::splat(0.25);
        let mut phase = Lanes::new([a.phase, b.phase]);
        let mut output = Lanes::new([a.output, b.output]);
        let mut sum = Lanes::splat(0.0);
        let triangle = |v: Lanes| {
            let y = (v + Lanes::splat(1.0)) * Lanes::splat(0.25);
            Lanes::splat(1.0) - Lanes::splat(4.0) * ((y - y.floor()) - Lanes::splat(0.5)).abs()
        };
        for _ in 0..4 {
            phase = (phase + step).fract();
            let x = match wave {
                Waveform::Saw => phase * Lanes::splat(2.0) - Lanes::splat(1.0),
                Waveform::Pulse => Lanes::select_less(
                    phase,
                    Lanes::splat(0.5),
                    Lanes::splat(1.0),
                    Lanes::splat(-1.0),
                ),
                Waveform::Triangle => {
                    Lanes::splat(1.0) - Lanes::splat(4.0) * (phase - Lanes::splat(0.5)).abs()
                }
                _ => (phase * Lanes::splat(std::f64::consts::TAU)).map(super::fast::sin),
            };
            let driven = x * gain + offset;
            let folded = match shape {
                FoldShape::Triangle => triangle(driven),
                FoldShape::Sine => {
                    (driven * Lanes::splat(std::f64::consts::FRAC_PI_2)).map(super::fast::sin)
                }
                FoldShape::Soft => {
                    let x = (triangle(driven) * Lanes::splat(1.5)).clamp(-3.0, 3.0);
                    x * (Lanes::splat(27.0) + x * x)
                        / (Lanes::splat(27.0) + Lanes::splat(9.0) * x * x)
                }
                FoldShape::Asymmetric => triangle(driven + depth),
                FoldShape::Rectify => driven.map(|v| v.abs().rem_euclid(2.0) - 1.0),
            };
            output = output + (folded - output) * Lanes::splat(0.6);
            sum = sum + output * Lanes::splat(0.25);
        }
        let [phase, output, sum] = [phase.get(), output.get(), sum.get()];
        let mut result = [0.0; 2];
        for (lane, folder) in [a, b].into_iter().enumerate() {
            folder.phase = phase[lane];
            folder.output = output[lane];
            let dc = sum[lane] - folder.dc_input + 0.9995 * folder.dc_output;
            folder.dc_input = sum[lane];
            folder.dc_output = dc;
            result[lane] = dc.clamp(-1.0, 1.0) as f32;
        }
        result
    }
}

/// Two f64 lanes. On aarch64 one NEON register: its add, subtract, multiply,
/// divide, abs, trunc, floor, min and max round exactly as the scalar
/// instructions do, so each lane equals the scalar result bit for bit.
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
struct Lanes(std::arch::aarch64::float64x2_t);
#[cfg(target_arch = "aarch64")]
impl Lanes {
    #[inline(always)]
    fn new(values: [f64; 2]) -> Self {
        // SAFETY: NEON is part of every aarch64 target.
        unsafe { Self(std::arch::aarch64::vld1q_f64(values.as_ptr())) }
    }
    #[inline(always)]
    fn get(self) -> [f64; 2] {
        let mut values = [0.0; 2];
        unsafe { std::arch::aarch64::vst1q_f64(values.as_mut_ptr(), self.0) };
        values
    }
    #[inline(always)]
    fn splat(value: f64) -> Self {
        unsafe { Self(std::arch::aarch64::vdupq_n_f64(value)) }
    }
    #[inline(always)]
    fn abs(self) -> Self {
        unsafe { Self(std::arch::aarch64::vabsq_f64(self.0)) }
    }
    #[inline(always)]
    fn floor(self) -> Self {
        unsafe { Self(std::arch::aarch64::vrndmq_f64(self.0)) }
    }
    /// `x - x.trunc()`, as f64::fract.
    #[inline(always)]
    fn fract(self) -> Self {
        self - unsafe { Self(std::arch::aarch64::vrndq_f64(self.0)) }
    }
    /// f64::clamp for finite values and either zero: FMAX/FMIN keep -0.0
    /// against nonzero bounds as the comparisons do.
    #[inline(always)]
    fn clamp(self, low: f64, high: f64) -> Self {
        use std::arch::aarch64::*;
        unsafe {
            Self(vminq_f64(
                vmaxq_f64(self.0, vdupq_n_f64(low)),
                vdupq_n_f64(high),
            ))
        }
    }
    /// `if x < limit { less } else { other }` per lane.
    #[inline(always)]
    fn select_less(x: Self, limit: Self, less: Self, other: Self) -> Self {
        use std::arch::aarch64::*;
        unsafe { Self(vbslq_f64(vcltq_f64(x.0, limit.0), less.0, other.0)) }
    }
}
#[cfg(target_arch = "aarch64")]
macro_rules! lanes_operator {
    ($trait:ident, $method:ident, $intrinsic:ident) => {
        impl std::ops::$trait for Lanes {
            type Output = Self;
            #[inline(always)]
            fn $method(self, other: Self) -> Self {
                unsafe { Self(std::arch::aarch64::$intrinsic(self.0, other.0)) }
            }
        }
    };
}
#[cfg(target_arch = "aarch64")]
lanes_operator!(Add, add, vaddq_f64);
#[cfg(target_arch = "aarch64")]
lanes_operator!(Sub, sub, vsubq_f64);
#[cfg(target_arch = "aarch64")]
lanes_operator!(Mul, mul, vmulq_f64);
#[cfg(target_arch = "aarch64")]
lanes_operator!(Div, div, vdivq_f64);

/// Two f64 lanes elsewhere: the scalar operations, lane by lane.
#[cfg(not(target_arch = "aarch64"))]
#[derive(Clone, Copy)]
struct Lanes([f64; 2]);
#[cfg(not(target_arch = "aarch64"))]
impl Lanes {
    #[inline(always)]
    fn new(values: [f64; 2]) -> Self {
        Self(values)
    }
    #[inline(always)]
    fn get(self) -> [f64; 2] {
        self.0
    }
    #[inline(always)]
    fn splat(value: f64) -> Self {
        Self([value; 2])
    }
    #[inline(always)]
    fn abs(self) -> Self {
        self.map(f64::abs)
    }
    #[inline(always)]
    fn floor(self) -> Self {
        self.map(f64::floor)
    }
    #[inline(always)]
    fn fract(self) -> Self {
        self.map(f64::fract)
    }
    #[inline(always)]
    fn clamp(self, low: f64, high: f64) -> Self {
        self.map(|x| x.clamp(low, high))
    }
    #[inline(always)]
    fn select_less(x: Self, limit: Self, less: Self, other: Self) -> Self {
        Self(std::array::from_fn(|i| {
            if x.0[i] < limit.0[i] {
                less.0[i]
            } else {
                other.0[i]
            }
        }))
    }
}
#[cfg(not(target_arch = "aarch64"))]
macro_rules! lanes_operator {
    ($trait:ident, $method:ident, $op:tt) => {
        impl std::ops::$trait for Lanes {
            type Output = Self;
            #[inline(always)]
            fn $method(self, other: Self) -> Self {
                Self([self.0[0] $op other.0[0], self.0[1] $op other.0[1]])
            }
        }
    };
}
#[cfg(not(target_arch = "aarch64"))]
lanes_operator!(Add, add, +);
#[cfg(not(target_arch = "aarch64"))]
lanes_operator!(Sub, sub, -);
#[cfg(not(target_arch = "aarch64"))]
lanes_operator!(Mul, mul, *);
#[cfg(not(target_arch = "aarch64"))]
lanes_operator!(Div, div, /);
impl Lanes {
    /// A scalar function on each lane.
    #[inline(always)]
    fn map(self, f: impl Fn(f64) -> f64) -> Self {
        let [a, b] = self.get();
        Self::new([f(a), f(b)])
    }
}
fn soft(x: f64) -> f64 {
    let x = x.clamp(-3.0, 3.0);
    x * (27.0 + x * x) / (27.0 + 9.0 * x * x)
}

#[cfg(test)]
mod tests {
    #[test]
    fn floor_triangle_matches_the_euclidean_remainder() {
        let original = |v: f64| 1.0 - 4.0 * (((v + 1.0) * 0.25).rem_euclid(1.0) - 0.5).abs();
        let floor = |v: f64| {
            let y = (v + 1.0) * 0.25;
            1.0 - 4.0 * ((y - y.floor()) - 0.5).abs()
        };
        let mut seed = 0x9e37_79b9_7f4a_7c15_u64;
        let mut values = vec![
            0.0, -0.0, -1.0, 1.0, 3.0, -5.0, -1e-300, 1e-300, 1e15, -1e15,
        ];
        for _ in 0..200_000 {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            values.push(((seed >> 11) as f64 / (1_u64 << 53) as f64 - 0.5) * 64.0);
        }
        for v in values {
            assert_eq!(floor(v).to_bits(), original(v).to_bits(), "{v}");
        }
    }
}
