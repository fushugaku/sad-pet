//! Host memory and arithmetic for the statically translated firmware.
//! Register bytes intentionally overlap (s0/s1 are the halves of d0).
use std::cell::Cell;

pub const RETURN: u32 = 0xfffffff0;
/// User-operation ids of the translated blocks (see `userops::name`).
const SET_ISA_MODE: u32 = 62;
const VECTOR_SIGNED_TO_FLOAT: u32 = 219;
const VECTOR_FLOAT_TO_SIGNED_FIXED: u32 = 223;

/// The firmware's VFMA: a fused multiply-add, rounded once. Targets with a
/// hardware FMA (every aarch64 CPU, x86 built with `+fma`) run the instruction
/// itself; the others (wasm32, baseline x86) use the emulations below instead
/// of the software `fmaf`/`fma` routines. Every path returns the unique
/// correctly rounded value, so the samples are identical.
pub trait FastFma {
    fn fast_mul_add(self, y: Self, z: Self) -> Self;
}
impl FastFma for f32 {
    #[inline(always)]
    fn fast_mul_add(self, y: f32, z: f32) -> f32 {
        #[cfg(any(target_arch = "aarch64", target_feature = "fma"))]
        return self.mul_add(y, z);
        #[cfg(not(any(target_arch = "aarch64", target_feature = "fma")))]
        fmaf32(self, y, z)
    }
}
impl FastFma for f64 {
    #[inline(always)]
    fn fast_mul_add(self, y: f64, z: f64) -> f64 {
        #[cfg(any(target_arch = "aarch64", target_feature = "fma"))]
        return self.mul_add(y, z);
        #[cfg(not(any(target_arch = "aarch64", target_feature = "fma")))]
        fma64(self, y, z)
    }
}

/// Correctly rounded float32 fused multiply-add from float64 operations. The
/// float64 product of two float32 values is exact, so one float64 addition
/// and the float32 conversion round twice. That is still the correctly
/// rounded result unless the addition was inexact and its result lies on a
/// float32 rounding boundary: a halfway case of a normal float32 (musl's
/// test), or any float32-subnormal result, whose rounding bit lies higher.
/// Those sums are first rounded to odd with the exact error of the addition,
/// after which the float32 conversion rounds once (Boldo and Melquiond).
#[cfg_attr(
    all(not(test), any(target_arch = "aarch64", target_feature = "fma")),
    allow(dead_code)
)]
#[inline(always)]
pub fn fmaf32(x: f32, y: f32, z: f32) -> f32 {
    let xy = x as f64 * y as f64;
    let addend = z as f64;
    let sum = xy + addend;
    let bits = sum.to_bits();
    let exponent = (bits >> 52) & 0x7ff;
    // Biased exponent 1023 - 126: the sum is at least the smallest normal
    // float32. 0x7ff: NaN or infinity, which only infinite operands produce.
    if ((bits & 0x1fff_ffff) != 0x1000_0000 && exponent >= 1023 - 126) || exponent == 0x7ff {
        return sum as f32;
    }
    // Knuth's exact sum: xy + addend == sum + error.
    let virtual_addend = sum - xy;
    let error = (xy - (sum - virtual_addend)) + (addend - virtual_addend);
    let odd = if error != 0.0 && bits & 1 == 0 {
        if error > 0.0 {
            sum.next_up()
        } else {
            sum.next_down()
        }
    } else {
        sum
    };
    odd as f32
}

/// Correctly rounded float64 fused multiply-add from plain float64 operations,
/// for targets without a hardware FMA (their `fma` routine multiplies 128-bit
/// integers). Boldo and Melquiond's emulated FMA: the exact product and the
/// exact sum with the addend are split into two floats each, their low parts
/// are added with rounding to odd, and one final rounding to nearest gives the
/// correctly rounded result. The proof needs every intermediate value to stay
/// normal, so operands outside a wide exponent window (and NaN/infinity) take
/// the library routine; a zero factor or addend has an exact product or an
/// exact sum and needs no fusion at all. Every path returns the unique
/// correctly rounded value.
/// (SuperEcho's routine: AlphaVerb's delay-line reads run one VFMA.F64 each.)
#[cfg_attr(
    all(not(test), any(target_arch = "aarch64", target_feature = "fma")),
    allow(dead_code)
)]
#[inline(always)]
pub fn fma64(a: f64, b: f64, c: f64) -> f64 {
    #[inline(always)]
    fn moderate(x: f64) -> bool {
        // Biased exponents 1023 ± 480: normal, |x| within 2^±480.
        ((x.to_bits() >> 52) & 0x7ff).wrapping_sub(1023 - 480) <= 960
    }
    if moderate(a) && moderate(b) {
        if moderate(c) {
            // Dekker's exact product: a * b == ph + pl.
            const SPLIT: f64 = 134_217_729.0; // 2^27 + 1
            let t = SPLIT * a;
            let ah = t - (t - a);
            let al = a - ah;
            let t = SPLIT * b;
            let bh = t - (t - b);
            let bl = b - bh;
            let ph = a * b;
            let pl = ((ah * bh - ph) + ah * bl + al * bh) + al * bl;
            // Knuth's exact sum: c + ph == sh + sl.
            let sh = c + ph;
            let z = sh - c;
            let sl = (c - (sh - z)) + (ph - z);
            // sl + pl rounded to odd: the nearest float, nudged to the odd
            // neighbour on the side of the exact sum when it was rounded.
            let v = sl + pl;
            let z = v - sl;
            let e = (sl - (v - z)) + (pl - z);
            let v = if e != 0.0 && v.to_bits() & 1 == 0 {
                if e > 0.0 {
                    v.next_up()
                } else {
                    v.next_down()
                }
            } else {
                v
            };
            return sh + v;
        }
        if c == 0.0 {
            // A normal product of moderate size is not zero: adding a signed
            // zero leaves its rounding unchanged.
            return a * b + c;
        }
    } else if (a == 0.0 && b.is_finite()) || (b == 0.0 && a.is_finite()) {
        // The product is an exact signed zero, so one rounding remains.
        return a * b + c;
    }
    a.mul_add(b, c)
}

const FLASH: &[u8; FLASH_SIZE] = include_bytes!("../assets/flash.bin");
const ITCM_SIZE: usize = 0x10000;
const DTCM_SIZE: usize = 0x20000;
const OCRAM_SIZE: usize = 0x80000;
const FLASH_SIZE: usize = 0x3d500;
const HEAP_SIZE: usize = 0x100_0000;

/// A zeroed region on the heap (a boxed array literal would be built on the stack).
fn region<const N: usize>() -> Box<[u8; N]> {
    vec![0; N].into_boxed_slice().try_into().unwrap()
}

/// The offset of `size` bytes at `address` in the region at `base`, if they
/// lie inside it. One unsigned comparison: an address below the base wraps.
#[inline(always)]
fn offset(address: u32, base: u32, len: usize, size: usize) -> Option<usize> {
    let off = address.wrapping_sub(base) as usize;
    (off <= len - size).then_some(off)
}

pub struct Machine {
    #[cfg(test)]
    pub reference_dispatch: bool,
    regs: [u8; 4096],
    itcm: Box<[u8; ITCM_SIZE]>,
    dtcm: Box<[u8; DTCM_SIZE]>,
    ocram: Box<[u8; OCRAM_SIZE]>,
    heap: Box<[u8; HEAP_SIZE]>,
    heap_cursor: usize,
    pub fault: Cell<Option<u32>>,
    irq: bool,
}

pub fn signed(v: u64, size: usize) -> i64 {
    let shift = 64 - size * 8;
    ((v << shift) as i64) >> shift
}

pub fn signed_overflow(a: u64, b: u64, size: usize, subtract: bool) -> u64 {
    let mask = 1u64 << (size * 8 - 1);
    let result = if subtract {
        a.wrapping_sub(b)
    } else {
        a.wrapping_add(b)
    };
    (((a ^ result) & if subtract { a ^ b } else { !(a ^ b) } & mask) != 0) as u64
}

impl Machine {
    pub fn new() -> Self {
        let mut m = Self {
            #[cfg(test)]
            reference_dispatch: false,
            regs: [0; 4096],
            itcm: region(),
            dtcm: region(),
            ocram: region(),
            heap: region(),
            heap_cursor: 0,
            fault: Cell::new(None),
            irq: true,
        };
        m.restore_images();
        m
    }
    fn restore_images(&mut self) {
        self.itcm[..13440].copy_from_slice(include_bytes!("../assets/itcm.bin"));
        self.dtcm[..0x2740].copy_from_slice(include_bytes!("../assets/dtcm.bin"));
        self.dtcm[0x2740..0x2e54].copy_from_slice(include_bytes!("../assets/dtcm2.bin"));
        self.w(0, 1, 1);
    }
    /// Reuse the existing allocations, including in the AudioWorklet thread.
    pub fn reset(&mut self) {
        self.regs.fill(0);
        self.itcm.fill(0);
        self.dtcm.fill(0);
        self.ocram.fill(0);
        self.heap.fill(0);
        self.heap_cursor = 0;
        self.fault.set(None);
        self.irq = true;
        self.restore_images();
    }
    #[inline(always)]
    pub fn r(&self, off: usize, size: usize) -> u64 {
        let mut bytes = [0u8; 8];
        bytes[..size].copy_from_slice(&self.regs[off..off + size]);
        u64::from_le_bytes(bytes)
    }
    #[inline(always)]
    pub fn w(&mut self, off: usize, size: usize, value: u64) {
        self.regs[off..off + size].copy_from_slice(&value.to_le_bytes()[..size]);
    }
    pub fn fail(&self, address: u32) {
        if self.fault.get().is_none() {
            self.fault.set(Some(address));
        }
    }
    /// Regions are disjoint and none ends where another begins, so an access
    /// that crosses a region's end matches no region and faults, as before.
    /// The delay lines (heap) and the stack and state (DTCM) are tested first.
    #[inline(always)]
    pub fn load(&self, address: u32, size: usize) -> u64 {
        let data: &[u8] = if let Some(off) = offset(address, 0x80000000, HEAP_SIZE, size) {
            &self.heap[off..off + size]
        } else if let Some(off) = offset(address, 0x20000000, DTCM_SIZE, size) {
            &self.dtcm[off..off + size]
        } else if let Some(off) = offset(address, 0x60020000, FLASH_SIZE, size) {
            &FLASH[off..off + size]
        } else if let Some(off) = offset(address, 0, ITCM_SIZE, size) {
            &self.itcm[off..off + size]
        } else if let Some(off) = offset(address, 0x20200000, OCRAM_SIZE, size) {
            &self.ocram[off..off + size]
        } else {
            self.fail(address);
            return 0;
        };
        let mut bytes = [0u8; 8];
        bytes[..size].copy_from_slice(data);
        u64::from_le_bytes(bytes)
    }
    #[inline(always)]
    pub fn store(&mut self, address: u32, size: usize, value: u64) {
        let data: &mut [u8] = if let Some(off) = offset(address, 0x80000000, HEAP_SIZE, size) {
            &mut self.heap[off..off + size]
        } else if let Some(off) = offset(address, 0x20000000, DTCM_SIZE, size) {
            &mut self.dtcm[off..off + size]
        } else if let Some(off) = offset(address, 0, ITCM_SIZE, size) {
            &mut self.itcm[off..off + size]
        } else if let Some(off) = offset(address, 0x20200000, OCRAM_SIZE, size) {
            &mut self.ocram[off..off + size]
        } else {
            self.fail(address);
            return;
        };
        data.copy_from_slice(&value.to_le_bytes()[..size]);
    }
    pub fn float(&self, address: u32) -> f32 {
        f32::from_bits(self.load(address, 4) as u32)
    }
    pub fn put_float(&mut self, address: u32, value: f32) {
        self.store(address, 4, value.to_bits() as u64);
    }
    pub fn set_s0(&mut self, value: f32) {
        self.w(768, 4, value.to_bits() as u64);
    }
    pub fn invoke(&mut self, entry: u32, args: &[u32]) -> Result<(), u32> {
        self.fault.set(None);
        self.w(84, 4, 0x2001e000);
        self.w(88, 4, (RETURN | 1) as u64);
        for (n, arg) in args.iter().take(4).enumerate() {
            self.w(32 + n * 4, 4, *arg as u64);
        }
        let mut pc = entry & !1;
        for _ in 0..4_000_000 {
            if pc == RETURN {
                return self.fault.get().map_or(Ok(()), Err);
            }
            #[cfg(test)]
            let fused = !self.reference_dispatch;
            #[cfg(not(test))]
            let fused = true;
            match pc {
                0x60029898 if fused => pc = crate::fused::b_60029898(self),
                0x60029dcc if fused => pc = crate::fused::b_60029dcc(self),
                // Replace only platform allocation; DSP/math execute translated Rust.
                0x600414c0 => {
                    let size = self.r(32, 4) as usize;
                    let next = (self.heap_cursor + size + 15) & !15;
                    if next > self.heap.len() {
                        return Err(0x80000000 + self.heap_cursor as u32);
                    }
                    self.w(32, 4, 0x80000000 + self.heap_cursor as u64);
                    self.heap_cursor = next;
                    pc = self.r(88, 4) as u32 & !1;
                }
                0x600414d0 => {
                    pc = self.r(88, 4) as u32 & !1;
                }
                _ => {
                    pc = match crate::fused::ITCM.get((pc >> 1) as usize) {
                        Some(Some(block)) if fused => block(self),
                        _ => crate::generated::step(self, pc),
                    }
                }
            }
            if let Some(address) = self.fault.get() {
                return Err(address);
            }
        }
        Err(pc)
    }
    /// The translated blocks name three user operations: the ISA-mode write of
    /// every interworking branch (`bx`, `blx`, `pop {pc}`) and the two VCVT
    /// conversions. Matching their ids inline removes the no-op from every
    /// return and the name lookup from every conversion; any other operation
    /// keeps the named table.
    #[inline(always)]
    pub fn userop(
        &mut self,
        id: u32,
        a: u64,
        b: u64,
        d: u64,
        input_size: usize,
        output_size: usize,
    ) -> u64 {
        match id {
            SET_ISA_MODE => 0,
            VECTOR_SIGNED_TO_FLOAT => {
                let value = signed(a, input_size);
                if output_size == 8 {
                    (value as f64).to_bits()
                } else {
                    (value as f32).to_bits() as u64
                }
            }
            VECTOR_FLOAT_TO_SIGNED_FIXED => {
                let value = f32::from_bits(a as u32) as f64 * 2f64.powi(d as i32);
                if b == 32 {
                    (value as i32 as u32) as u64
                } else {
                    value as i64 as u64
                }
            }
            _ => self.userop_named(id, a, output_size),
        }
    }
    #[cold]
    #[inline(never)]
    fn userop_named(&mut self, id: u32, a: u64, output_size: usize) -> u64 {
        // Names/IDs are generated from the same Ghidra language as the code.
        match crate::userops::name(id) {
            "DataSynchronizationBarrier"
            | "InstructionSynchronizationBarrier"
            | "DataMemoryBarrier"
            | "ExclusiveAccess" => 0,
            "hasExclusiveAccess" | "isCurrentModePrivileged" => 1,
            "enableIRQinterrupts" => {
                self.irq = true;
                0
            }
            "disableIRQinterrupts" => {
                self.irq = false;
                0
            }
            "isIRQinterruptsEnabled" => self.irq as u64,
            "setMainStackPointer" => {
                self.w(84, 4, a);
                0
            }
            "VectorUnsignedToFloat" => {
                if output_size == 8 {
                    (a as f64).to_bits()
                } else {
                    (a as f32).to_bits() as u64
                }
            }
            _ => {
                self.fail(0xf0000000 | id);
                0
            }
        }
    }
    pub fn ram(&self) -> &[u8] {
        &self.dtcm[..]
    }
    pub fn heap_used(&self) -> usize {
        self.heap_cursor
    }
    #[cfg(test)]
    pub fn assert_same_state(&self, other: &Self) {
        assert!(self.regs == other.regs, "registers changed");
        assert!(self.itcm == other.itcm, "ITCM changed");
        assert!(self.dtcm == other.dtcm, "DTCM changed");
        assert!(self.ocram == other.ocram, "OCRAM changed");
        assert!(self.heap == other.heap, "delay histories changed");
        assert_eq!(self.heap_cursor, other.heap_cursor);
        assert_eq!(self.fault.get(), other.fault.get());
        assert_eq!(self.irq, other.irq);
    }
}

#[cfg(test)]
mod userop_tests {
    #[test]
    fn inline_userop_ids_name_the_translated_operations() {
        assert_eq!(crate::userops::name(super::SET_ISA_MODE), "setISAMode");
        assert_eq!(
            crate::userops::name(super::VECTOR_SIGNED_TO_FLOAT),
            "VectorSignedToFloat"
        );
        assert_eq!(
            crate::userops::name(super::VECTOR_FLOAT_TO_SIGNED_FIXED),
            "VectorFloatToSignedFixed"
        );
    }
}

#[cfg(test)]
mod fma_tests {
    use super::{fma64, fmaf32};

    struct Rng(u64);
    impl Rng {
        fn next(&mut self) -> u64 {
            // xorshift64*
            self.0 ^= self.0 >> 12;
            self.0 ^= self.0 << 25;
            self.0 ^= self.0 >> 27;
            self.0.wrapping_mul(0x2545_f491_4f6c_dd1d)
        }
        /// Random sign and mantissa, biased exponent uniform in `exponents`.
        fn float(&mut self, exponents: std::ops::RangeInclusive<u64>, sparse: bool) -> f64 {
            let bits = self.next();
            let span = exponents.end() - exponents.start() + 1;
            let exponent = exponents.start() + (self.next() % span);
            let mut mantissa = bits & 0x000f_ffff_ffff_ffff;
            if sparse {
                // Few significant bits: exact products and halfway sums.
                mantissa &= self.next() & self.next() & self.next();
            }
            f64::from_bits((bits & (1 << 63)) | (exponent << 52) | mantissa)
        }
    }

    fn check(a: f64, b: f64, c: f64) {
        let want = a.mul_add(b, c);
        let got = fma64(a, b, c);
        assert!(
            want.to_bits() == got.to_bits() || (want.is_nan() && got.is_nan()),
            "fma({a:e}, {b:e}, {c:e}) = {want:e} ({:#018x}), emulated {got:e} ({:#018x})",
            want.to_bits(),
            got.to_bits()
        );
    }

    #[test]
    fn fma64_matches_the_fused_multiply_add_bit_for_bit() {
        let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
        let specials = [
            0.0,
            -0.0,
            1.0,
            -1.0,
            0.5,
            f64::MIN_POSITIVE,
            -f64::MIN_POSITIVE,
            f64::from_bits(1),
            f64::MAX,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NAN,
            1.0 + f64::EPSILON,
            3.0,
            1e-300,
            1e300,
        ];
        for &a in &specials {
            for &b in &specials {
                for &c in &specials {
                    check(a, b, c);
                }
            }
        }
        for _ in 0..3_000_000 {
            // Uniform within the emulated window, independent exponents.
            check(
                rng.float(543..=1503, false),
                rng.float(543..=1503, false),
                rng.float(543..=1503, false),
            );
            // Sparse mantissas near one exponent: ties and exact sums.
            let e = 900 + rng.next() % 300;
            let a = rng.float(e..=e + 3, true);
            let b = rng.float(e..=e + 3, true);
            let c = rng.float(2 * e - 1023 - 4..=2 * e - 1023 + 6, true);
            check(a, b, c);
            // Cancellation: the addend is the rounded product or a neighbour.
            let a = rng.float(800..=1200, false);
            let b = rng.float(800..=1200, false);
            let p = -(a * b);
            check(a, b, p);
            check(a, b, p.next_up());
            check(a, b, p.next_down());
            check(a, b, p * 0.5);
            check(a, b, p * 2.0);
            // Anything at all, including subnormals and infinities.
            check(
                rng.float(0..=2047, false),
                rng.float(0..=2047, false),
                rng.float(0..=2047, false),
            );
            // Zero factors and addends against every magnitude.
            let x = rng.float(0..=2047, false);
            let zero = if rng.next() & 1 == 0 { 0.0 } else { -0.0 };
            check(zero, x, rng.float(0..=2047, false));
            check(x, zero, rng.float(0..=2047, false));
            check(x, rng.float(0..=2047, false), zero);
        }
    }

    fn check32(a: f32, b: f32, c: f32) {
        let want = a.mul_add(b, c);
        let got = fmaf32(a, b, c);
        assert!(
            want.to_bits() == got.to_bits() || (want.is_nan() && got.is_nan()),
            "fmaf({a:e}, {b:e}, {c:e}) = {want:e} ({:#010x}), emulated {got:e} ({:#010x})",
            want.to_bits(),
            got.to_bits()
        );
    }

    #[test]
    fn fmaf32_matches_the_fused_multiply_add_bit_for_bit() {
        // A product just below 2^-150, half a subnormal step: the float64 sum
        // rounds onto the float32 midpoint, which a plain conversion would
        // round to even instead of down.
        assert_eq!(
            fmaf32(
                f32::from_bits(0x1a00_0001),
                f32::from_bits(0x19ff_fffe),
                f32::from_bits(0x0040_0001)
            )
            .to_bits(),
            0x0040_0001
        );
        let mut rng = Rng(0x2545_f491_4f6c_dd1d);
        let specials = [
            0.0,
            -0.0,
            1.0,
            -1.0,
            0.5,
            f32::MIN_POSITIVE,
            -f32::MIN_POSITIVE,
            f32::from_bits(1),
            f32::from_bits(0x8000_0001),
            f32::MAX,
            f32::INFINITY,
            f32::NEG_INFINITY,
            f32::NAN,
            1.0 + f32::EPSILON,
            3.0,
            1e-30,
            1e30,
        ];
        for &a in &specials {
            for &b in &specials {
                for &c in &specials {
                    check32(a, b, c);
                }
            }
        }
        let float = |bits: u64, exponent: u64| {
            f32::from_bits((bits as u32 & 0x807f_ffff) | ((exponent as u32 & 0xff) << 23))
        };
        for _ in 0..2_000_000 {
            // Anything at all.
            check32(
                f32::from_bits(rng.next() as u32),
                f32::from_bits(rng.next() as u32),
                f32::from_bits(rng.next() as u32),
            );
            // Products around the subnormal range, subnormal or tiny addends.
            let e = 20 + rng.next() % 100;
            let a = float(rng.next(), e);
            let b = float(rng.next(), 104 + rng.next() % 40 - e.min(104));
            check32(a, b, float(rng.next(), rng.next() % 3));
            // (1 + k) (1 - k) = 1 - k^2 just below a power of two, half a
            // subnormal step or a halfway case against an odd addend.
            let k = 1 + rng.next() % 1024;
            let ea = 1 + rng.next() % 200;
            let eb = 104 + rng.next() % 40 - ea.min(104);
            let a = f32::from_bits(((ea as u32) << 23) | k as u32);
            let b = f32::from_bits(((eb as u32 - 1) << 23) | (0x80_0000 - 2 * k as u32));
            let c = f32::from_bits((rng.next() as u32 & 0x80ff_ffff) | 1);
            check32(a, b, c);
            check32(-a, b, c);
            // Sparse mantissas: exact sums and halfway cases of normal results.
            let e = 60 + rng.next() % 130;
            let mut sparse = || rng.next() & rng.next() & rng.next();
            let (a, b, c) = (sparse(), sparse(), sparse());
            let (eb, ec) = (127 + rng.next() % 8, e + rng.next() % 40);
            check32(float(a, e), float(b, eb), float(c, ec));
            // Cancellation against the rounded product and its neighbours.
            let a = float(rng.next(), 64 + rng.next() % 128);
            let b = float(rng.next(), 64 + rng.next() % 128);
            let p = -(a * b);
            check32(a, b, p);
            check32(a, b, p.next_up());
            check32(a, b, p.next_down());
        }
    }
}
