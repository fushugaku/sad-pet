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

/// `x.sin()`, bit for bit. In the browser build std's sin is the libm port
/// of FreeBSD's sin, which calls its general argument reduction out of line;
/// `libm_sin` is that same arithmetic, inlined, for |x| < 2^20 * pi/2, which
/// covers every oscillator phase. Elsewhere this is the platform's sin.
#[inline(always)]
pub fn sin(x: f64) -> f64 {
    #[cfg(target_arch = "wasm32")]
    return libm_sin(x);
    #[cfg(not(target_arch = "wasm32"))]
    x.sin()
}

/// libm 0.2.16 `sin`: the same branches and operations in the same order.
#[cfg(any(target_arch = "wasm32", test))]
#[inline(always)]
fn libm_sin(x: f64) -> f64 {
    let ix = (x.to_bits() >> 32) as u32 & 0x7fffffff;
    if ix <= 0x3fe921fb {
        // |x| < 2^-26 returns x after only raising floating-point flags.
        if ix < 0x3e500000 {
            return x;
        }
        return libm::k_sin(x, 0.0, 0);
    }
    // Large, infinite and NaN arguments take std's own path.
    if ix >= 0x413921fb {
        return x.sin();
    }
    let (n, y0, y1) = libm::rem_pio2(x, ix);
    match n & 3 {
        0 => libm::k_sin(y0, y1, 1),
        1 => libm::k_cos(y0, y1),
        2 => -libm::k_sin(y0, y1, 1),
        _ => -libm::k_cos(y0, y1),
    }
}

/// The parts of libm 0.2.16 (k_sin.rs, k_cos.rs, rem_pio2.rs) `libm_sin` uses.
/// The constants are libm's literals, as written there.
#[cfg(any(target_arch = "wasm32", test))]
#[allow(clippy::excessive_precision, clippy::approx_constant)]
mod libm {
    const S1: f64 = -1.66666666666666324348e-01;
    const S2: f64 = 8.33333333332248946124e-03;
    const S3: f64 = -1.98412698298579493134e-04;
    const S4: f64 = 2.75573137070700676789e-06;
    const S5: f64 = -2.50507602534068634195e-08;
    const S6: f64 = 1.58969099521155010221e-10;
    #[inline(always)]
    pub(super) fn k_sin(x: f64, y: f64, iy: i32) -> f64 {
        let z = x * x;
        let w = z * z;
        let r = S2 + z * (S3 + z * S4) + z * w * (S5 + z * S6);
        let v = z * x;
        if iy == 0 {
            x + v * (S1 + z * r)
        } else {
            x - ((z * (0.5 * y - v * r) - y) - v * S1)
        }
    }

    const C1: f64 = 4.16666666666666019037e-02;
    const C2: f64 = -1.38888888888741095749e-03;
    const C3: f64 = 2.48015872894767294178e-05;
    const C4: f64 = -2.75573143513906633035e-07;
    const C5: f64 = 2.08757232129817482790e-09;
    const C6: f64 = -1.13596475577881948265e-11;
    #[inline(always)]
    pub(super) fn k_cos(x: f64, y: f64) -> f64 {
        let z = x * x;
        let w = z * z;
        let r = z * (C1 + z * (C2 + z * C3)) + w * w * (C4 + z * (C5 + z * C6));
        let hz = 0.5 * z;
        let w = 1.0 - hz;
        w + (((1.0 - w) - hz) + (z * r - x * y))
    }

    const TO_INT: f64 = 1.5 / 2.2204460492503131e-16;
    const INV_PIO2: f64 = 6.36619772367581382433e-01;
    const PIO2_1: f64 = 1.57079632673412561417e+00;
    const PIO2_1T: f64 = 6.07710050650619224932e-11;
    const PIO2_2: f64 = 6.07710050630396597660e-11;
    const PIO2_2T: f64 = 2.02226624879595063154e-21;
    const PIO2_3: f64 = 2.02226624871116645580e-21;
    const PIO2_3T: f64 = 8.47842766036889956997e-32;

    #[inline(always)]
    fn medium(x: f64, ix: u32) -> (i32, f64, f64) {
        let tmp = x * INV_PIO2 + TO_INT;
        let f_n = tmp - TO_INT;
        let n = f_n as i32;
        let mut r = x - f_n * PIO2_1;
        let mut w = f_n * PIO2_1T;
        let mut y0 = r - w;
        let ey = (y0.to_bits() >> 52) as i32 & 0x7ff;
        let ex = (ix >> 20) as i32;
        if ex - ey > 16 {
            let t = r;
            w = f_n * PIO2_2;
            r = t - w;
            w = f_n * PIO2_2T - ((t - r) - w);
            y0 = r - w;
            let ey = (y0.to_bits() >> 52) as i32 & 0x7ff;
            if ex - ey > 49 {
                let t = r;
                w = f_n * PIO2_3;
                r = t - w;
                w = f_n * PIO2_3T - ((t - r) - w);
                y0 = r - w;
            }
        }
        let y1 = (r - y0) - w;
        (n, y0, y1)
    }

    /// `rem_pio2` for pi/4 < |x| < 2^20 * pi/2 (`ix` is |x|'s high word).
    #[inline(always)]
    pub(super) fn rem_pio2(x: f64, ix: u32) -> (i32, f64, f64) {
        let positive = x.to_bits() >> 63 == 0;
        // Near k * pi/2 the fixed steps below cancel; medium handles those.
        let near = |k: f64| {
            let (z, t) = (k * PIO2_1, k * PIO2_1T);
            if positive {
                let z = x - z;
                let y0 = z - t;
                (y0, (z - y0) - t)
            } else {
                let z = x + z;
                let y0 = z + t;
                (y0, (z - y0) + t)
            }
        };
        let signed = |n: i32| if positive { n } else { -n };
        if ix <= 0x400f6a7a {
            if (ix & 0xfffff) == 0x921fb {
                return medium(x, ix);
            }
            if ix <= 0x4002d97c {
                let (y0, y1) = near(1.0);
                return (signed(1), y0, y1);
            }
            let (y0, y1) = near(2.0);
            return (signed(2), y0, y1);
        }
        if ix <= 0x401c463b {
            if ix <= 0x4015fdbc {
                if ix == 0x4012d97c {
                    return medium(x, ix);
                }
                let (y0, y1) = near(3.0);
                return (signed(3), y0, y1);
            }
            if ix == 0x401921fb {
                return medium(x, ix);
            }
            let (y0, y1) = near(4.0);
            return (signed(4), y0, y1);
        }
        medium(x, ix)
    }
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

    #[test]
    fn libm_sin_matches_the_libm_crate_bit_for_bit() {
        // Above 2^20 * pi/2 (and for inf/NaN) libm_sin calls std's sin, which
        // is this libm only in the browser build.
        let check = |x: f64| {
            if (x.to_bits() >> 32) as u32 & 0x7fffffff < 0x413921fb {
                assert_eq!(
                    libm_sin(x).to_bits(),
                    ::libm::sin(x).to_bits(),
                    "sin({x:e})"
                );
            }
        };
        // Every special case of the reduction: around each k * pi/2, the
        // branch limits, the tiny and the large arguments.
        let mut values = vec![0.0, -0.0, 1e-300, -1e-9, 1.5e-8, f64::MIN_POSITIVE, 5e-324];
        for k in -20..=20 {
            let center = k as f64 * std::f64::consts::FRAC_PI_2;
            for ulps in -300_i64..=300 {
                values.push(f64::from_bits((center.to_bits() as i64 + ulps) as u64));
            }
        }
        for high in [
            0x3e500000_u64,
            0x3fe921fb,
            0x4002d97c,
            0x400f6a7a,
            0x4012d97c,
            0x4015fdbc,
            0x401921fb,
            0x401c463b,
            0x413921fb,
        ] {
            for low in [0_u64, 1, 0x54442d18, 0xffffffff] {
                for sign in [0, 1_u64 << 63] {
                    values.push(f64::from_bits(sign | (high - 1) << 32 | low));
                    values.push(f64::from_bits(sign | high << 32 | low));
                    values.push(f64::from_bits(sign | (high + 1) << 32 | low));
                }
            }
        }
        let mut seed = 0x9e37_79b9_7f4a_7c15_u64;
        let mut random = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            (seed >> 11) as f64 / (1_u64 << 53) as f64
        };
        for _ in 0..2_000_000 {
            // Oscillator phases, then Sine folds of driven values up to +-16.
            values.push(random() * std::f64::consts::TAU);
            values.push((random() - 0.5) * 32.0 * std::f64::consts::FRAC_PI_2);
        }
        for x in values {
            check(x);
        }
    }
}
