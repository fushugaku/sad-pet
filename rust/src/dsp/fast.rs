//! Per-sample math without software libm calls. Cubic Hermite tables with
//! exact node derivatives stay within 1e-9 of libm, below float32 resolution,
//! so each call costs a few multiplies instead of a polynomial routine that
//! WebAssembly cannot delegate to hardware. `flush` keeps decaying IIR states
//! and smoothers out of the subnormal range, where x86 arithmetic slows by
//! two orders of magnitude; values below the threshold are hundreds of dB
//! under the noise floor of any output format.
use std::sync::LazyLock;

struct Table<const N: usize> {
    /// `[value, derivative * step]` at each node, including the final one.
    nodes: [[f32; 2]; N],
    scale: f32,
}
impl<const N: usize> Table<N> {
    fn new(range: f64, f: impl Fn(f64) -> (f64, f64)) -> Self {
        let segments = (N - 1) as f64;
        let step = range / segments;
        Self {
            nodes: std::array::from_fn(|i| {
                let (y, dy) = f(i as f64 * step);
                [y as f32, (dy * step) as f32]
            }),
            scale: (segments / range) as f32,
        }
    }
    /// Cubic Hermite interpolation for `0 <= x <= range`.
    #[inline(always)]
    fn eval(&self, x: f32) -> f32 {
        let p = x * self.scale;
        let i = (p as usize).min(N - 2);
        let t = p - i as f32;
        let [y0, d0] = self.nodes[i];
        let [y1, d1] = self.nodes[i + 1];
        let t2 = t * t;
        let t3 = t2 * t;
        (2.0 * t3 - 3.0 * t2 + 1.0) * y0
            + (t3 - 2.0 * t2 + t) * d0
            + (3.0 * t2 - 2.0 * t3) * y1
            + (t3 - t2) * d1
    }
}

/// tanh(9.5) rounds to 1.0 in float32.
const TANH_RANGE: f64 = 9.5;
static TANH: LazyLock<Table<1025>> = LazyLock::new(|| {
    Table::new(TANH_RANGE, |x| {
        let t = x.tanh();
        (t, 1.0 - t * t)
    })
});
static LOG2: LazyLock<Table<257>> = LazyLock::new(|| {
    Table::new(1.0, |x| {
        ((1.0 + x).log2(), 1.0 / ((1.0 + x) * std::f64::consts::LN_2))
    })
});
static EXP2: LazyLock<Table<257>> = LazyLock::new(|| {
    Table::new(1.0, |x| {
        let y = x.exp2();
        (y, y * std::f64::consts::LN_2)
    })
});
static COS: LazyLock<Table<257>> = LazyLock::new(|| {
    Table::new(1.0, |x| {
        let a = std::f64::consts::TAU * x;
        (a.cos(), -std::f64::consts::TAU * a.sin())
    })
});

/// Build every table before rendering starts.
pub fn warm_up() {
    LazyLock::force(&TANH);
    LazyLock::force(&LOG2);
    LazyLock::force(&EXP2);
    LazyLock::force(&COS);
}

#[inline]
pub fn tanh(x: f32) -> f32 {
    let magnitude = x.abs();
    if magnitude >= TANH_RANGE as f32 {
        return 1.0_f32.copysign(x);
    }
    TANH.eval(magnitude).copysign(x)
}

/// log2 of a positive normal float32.
#[inline]
pub fn log2(x: f32) -> f32 {
    let bits = x.to_bits();
    let exponent = ((bits >> 23) & 0xff) as i32 - 127;
    let mantissa = f32::from_bits((bits & 0x007f_ffff) | 0x3f80_0000) - 1.0;
    exponent as f32 + LOG2.eval(mantissa)
}

/// 2^x for x within the float32 exponent range.
#[inline]
pub fn exp2(x: f32) -> f32 {
    let x = x.clamp(-126.0, 127.0);
    let floor = x.floor();
    let scale = f32::from_bits(((floor as i32 + 127) as u32) << 23);
    EXP2.eval(x - floor) * scale
}

/// Level of a positive amplitude in decibels.
#[inline]
pub fn db(amplitude: f32) -> f32 {
    // Folded in f64 so the float32 constant is the correctly rounded value.
    log2(amplitude) * ((20.0 * std::f64::consts::LOG10_2) as f32)
}

/// Amplitude of a level in decibels.
#[inline]
pub fn gain_db(db: f32) -> f32 {
    exp2(db * ((std::f64::consts::LOG2_10 / 20.0) as f32))
}

/// sin²(π·phase) for a phase in 0..=1: the raised-cosine crossfade window.
#[inline]
pub fn sin2_pi(phase: f32) -> f32 {
    0.5 - 0.5 * COS.eval(phase.clamp(0.0, 1.0))
}

#[inline(always)]
pub fn flush(x: f32) -> f32 {
    if x.abs() < 1e-20 { 0.0 } else { x }
}

#[inline(always)]
pub fn flush64(x: f64) -> f64 {
    if x.abs() < 1e-200 { 0.0 } else { x }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tables_stay_within_float32_resolution_of_libm() {
        // References in f64: the float32 libm calls themselves round by an ulp.
        let mut worst = [0.0_f64; 4];
        for i in -40_000..=40_000 {
            let x = i as f32 * 0.0005;
            worst[0] = worst[0].max((tanh(x) as f64 - (x as f64).tanh()).abs());
            let p = 2.0_f32.powf(x * 0.01);
            worst[1] = worst[1].max((log2(p) as f64 - (p as f64).log2()).abs());
            let e = x * 0.01;
            let exact = (e as f64).exp2();
            worst[2] = worst[2].max(((exp2(e) as f64 - exact) / exact).abs());
        }
        for i in 0..=1000 {
            let phase = i as f32 / 1000.0;
            let exact = (std::f64::consts::PI * phase as f64).sin().powi(2);
            worst[3] = worst[3].max((sin2_pi(phase) as f64 - exact).abs());
        }
        // A few float32 ulps at most: the tables add nothing audible.
        assert!(worst[0] < 3e-7, "tanh error {}", worst[0]);
        assert!(worst[1] < 1e-6, "log2 error {}", worst[1]);
        assert!(worst[2] < 3e-7, "exp2 error {}", worst[2]);
        assert!(worst[3] < 3e-7, "sin2 error {}", worst[3]);
        assert_eq!(tanh(-0.0).to_bits(), (-0.0_f32).to_bits());
        assert_eq!(tanh(f32::INFINITY), 1.0);
        assert_eq!(tanh(-50.0), -1.0);
        assert!((db(1.0)).abs() < 1e-6);
        assert!((gain_db(-6.0) - 0.501_187_2).abs() < 1e-6);
        assert_eq!(flush(1e-25), 0.0);
        assert_eq!(flush(-1e-10), -1e-10);
        assert_eq!(flush64(1e-250), 0.0);
    }
}
