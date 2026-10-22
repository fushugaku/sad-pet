//! Memory and ABI adapters for the statically translated firmware.
//! Instructions themselves are Rust statements in `generated`, not interpreted bytes.
use std::{cell::Cell, fmt};

pub const RETURN_PC: u32 = 0xffff_fff0;

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
pub const FLASH_BASE: u32 = 0x6002_0000;
pub const ROM: &[u8; FLASH_SIZE] = include_bytes!("../firmware/echolocator-1.05.bin");
const ITCM_SIZE: usize = 0x3500;
const DTCM_SIZE: usize = 0x20000;
const OCRAM_SIZE: usize = 0x80000;
const FLASH_SIZE: usize = 0x3d000;
const HEAP_SIZE: usize = 32 * 1024 * 1024;

/// A zeroed region on the heap (a boxed array literal would be built on the stack).
fn region<const N: usize>() -> Box<[u8; N]> {
    vec![0; N].into_boxed_slice().try_into().unwrap()
}

/// The offset of `size` bytes at `address` in the region at `base`, if they
/// lie inside it. For a constant width this is one unsigned comparison: an
/// address below the base wraps around.
#[inline(always)]
fn offset(address: u32, base: u32, len: usize, size: usize) -> Option<usize> {
    let off = address.wrapping_sub(base) as usize;
    (off < len && size <= len - off).then_some(off)
}
pub type Result<T> = std::result::Result<T, Fault>;

#[derive(Debug, Clone)]
pub struct Fault {
    pub pc: u32,
    pub message: String,
}
impl fmt::Display for Fault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:08x}: {}", self.pc, self.message)
    }
}
impl std::error::Error for Fault {}

// Keep formatting/allocation for invalid firmware addresses out of the hot,
// inlined memory adapter (and out of every generated DSP load/store).
#[cold]
#[inline(never)]
fn unmapped_memory(pc: u32, address: u32, size: usize, operation: &str) -> Fault {
    Fault {
        pc,
        message: format!("{operation} unmapped address 0x{address:08x} ({size} bytes)"),
    }
}

#[cold]
#[inline(never)]
fn crossing_memory(pc: u32, address: u32, operation: &str) -> Fault {
    Fault {
        pc,
        message: format!("{operation} crosses region at 0x{address:08x}"),
    }
}

pub struct Machine {
    /// Tests compare the fused ITCM dispatch against the page dispatcher.
    #[cfg(test)]
    pub reference_dispatch: bool,
    pub registers: [u8; 2048],
    pub itcm: Box<[u8; ITCM_SIZE]>,
    pub dtcm: Box<[u8; DTCM_SIZE]>,
    pub ocram: Box<[u8; OCRAM_SIZE]>,
    pub heap: Box<[u8; HEAP_SIZE]>,
    pub pc: u32,
    pub ticks: u64,
    pub budget: u64,
    pub heap_cursor: usize,
    pub exclusive: u32,
    pub irq_enabled: bool,
    /// A fault raised inside a fused ITCM block, which carries on with zero
    /// loads and dropped stores until the dispatcher reads the cell.
    faulted: Cell<bool>,
    pending_fault: Cell<Option<Fault>>,
}

impl Default for Machine {
    fn default() -> Self {
        Self::new()
    }
}
impl Machine {
    pub fn new() -> Self {
        let mut dtcm = region::<DTCM_SIZE>();
        dtcm[..0x1f60].copy_from_slice(&ROM[0x37398..0x392f8]);
        dtcm[0x1f60..0x2674].copy_from_slice(&ROM[0x3c7f8..0x3cf0c]);
        Self {
            #[cfg(test)]
            reference_dispatch: false,
            registers: [0; 2048],
            itcm: ROM[0x392f8..0x3c7f8].try_into().map(Box::new).unwrap(),
            dtcm,
            ocram: region(),
            heap: region(),
            pc: 0,
            ticks: 0,
            budget: 0,
            heap_cursor: 0,
            exclusive: u32::MAX,
            irq_enabled: true,
            faulted: Cell::new(false),
            pending_fault: Cell::new(None),
        }
    }
    /// Reset the translated machine without replacing its allocated buffers.
    pub fn reset(&mut self) {
        self.registers.fill(0);
        self.itcm.copy_from_slice(&ROM[0x392f8..0x3c7f8]);
        self.dtcm.fill(0);
        self.dtcm[..0x1f60].copy_from_slice(&ROM[0x37398..0x392f8]);
        self.dtcm[0x1f60..0x2674].copy_from_slice(&ROM[0x3c7f8..0x3cf0c]);
        self.ocram.fill(0);
        self.heap[..self.heap_cursor].fill(0);
        self.pc = 0;
        self.ticks = 0;
        self.budget = 0;
        self.heap_cursor = 0;
        self.exclusive = u32::MAX;
        self.irq_enabled = true;
        self.faulted.set(false);
        self.pending_fault.set(None);
    }
    pub fn fault(&self, message: impl Into<String>) -> Fault {
        Fault {
            pc: self.pc,
            message: message.into(),
        }
    }
    /// Park the first fault of a fused block for the dispatcher.
    #[cold]
    #[inline(never)]
    pub fn fail(&self, fault: Fault) {
        if !self.faulted.replace(true) {
            self.pending_fault.set(Some(fault));
        }
    }
    /// A fused block has faulted since the last `take_fault`.
    #[inline(always)]
    pub fn faulted(&self) -> bool {
        self.faulted.get()
    }
    /// The parked fault, clearing the cell.
    #[cold]
    #[inline(never)]
    pub fn take_fault(&mut self) -> Fault {
        self.faulted.set(false);
        self.pending_fault
            .take()
            .unwrap_or_else(|| self.fault("fused block faulted"))
    }
    /// Continue inside ITCM from a fused block, in tail position. On wasm32
    /// with tail calls this is `return_call_indirect`: the firmware routine
    /// runs block to block without the dispatch loop, which only sees program
    /// counters outside ITCM, `RETURN_PC`, or a parked fault. Backward edges
    /// pass here, so the tick budget is still enforced on every loop.
    #[inline(always)]
    pub fn jump(&mut self, pc: u32) -> u32 {
        if self.faulted.get() {
            return pc;
        }
        if self.ticks >= self.budget {
            self.fail(self.fault("instruction budget exceeded"));
            return pc;
        }
        crate::fused::fused(self, pc).unwrap_or(pc)
    }
    /// `load` for fused ITCM blocks: an unmapped address parks the fault and
    /// reads as zero, so the block needs no early exit.
    #[inline(always)]
    pub fn load_itcm(&self, address: u32, size: usize) -> u64 {
        match self.region(address, size) {
            Ok(b) => match size {
                1 => b[0] as u64,
                2 => u16::from_le_bytes(b.try_into().unwrap()) as u64,
                4 => u32::from_le_bytes(b.try_into().unwrap()) as u64,
                8 => u64::from_le_bytes(b.try_into().unwrap()),
                _ => {
                    self.fail(self.fault("invalid load width"));
                    0
                }
            },
            Err(fault) => {
                self.fail(fault);
                0
            }
        }
    }
    /// `store` for fused ITCM blocks: an unmapped address parks the fault and
    /// drops the write.
    #[inline(always)]
    pub fn store_itcm(&mut self, address: u32, size: usize, value: u64) {
        match self.region_mut(address, size) {
            Ok(b) => b.copy_from_slice(&value.to_le_bytes()[..size]),
            Err(fault) => self.fail(fault),
        }
    }
    #[inline(always)]
    pub fn reg(&self, off: usize, size: usize) -> u64 {
        assert!(size <= self.registers.len() && off <= self.registers.len() - size);
        // SAFETY: the span is checked above. Generated constant offsets remove this
        // check during optimization, while callers of the public API remain checked.
        unsafe {
            let p = self.registers.as_ptr().add(off);
            match size {
                1 => *p as u64,
                2 => u16::from_le(std::ptr::read_unaligned(p.cast())) as u64,
                4 => u32::from_le(std::ptr::read_unaligned(p.cast())) as u64,
                8 => u64::from_le(std::ptr::read_unaligned(p.cast())),
                _ => unreachable!(),
            }
        }
    }
    #[inline(always)]
    pub fn set_reg(&mut self, off: usize, size: usize, value: u64) {
        assert!(size <= self.registers.len() && off <= self.registers.len() - size);
        // SAFETY: the complete unaligned write is inside the checked register span.
        unsafe {
            let p = self.registers.as_mut_ptr().add(off);
            match size {
                1 => *p = value as u8,
                2 => std::ptr::write_unaligned(p.cast(), (value as u16).to_le()),
                4 => std::ptr::write_unaligned(p.cast(), (value as u32).to_le()),
                8 => std::ptr::write_unaligned(p.cast(), value.to_le()),
                _ => unreachable!(),
            }
        }
    }
    // DSP loads have constant widths and often constant addresses. Keeping the
    // checked region selection beside each load lets LLVM remove redundant
    // mapping/width checks instead of calling this adapter for every sample.
    // Each region is one comparison, the delay lines (heap) and the stack and
    // state (DTCM) first. Regions are disjoint and none ends where another
    // begins, so an access that crosses a region's end matches none of them.
    #[inline(always)]
    fn region(&self, address: u32, size: usize) -> Result<&[u8]> {
        if let Some(off) = offset(address, 0x80000000, HEAP_SIZE, size) {
            return Ok(&self.heap[off..off + size]);
        }
        if let Some(off) = offset(address, 0x20000000, DTCM_SIZE, size) {
            return Ok(&self.dtcm[off..off + size]);
        }
        if let Some(off) = offset(address, FLASH_BASE, FLASH_SIZE, size) {
            return Ok(&ROM[off..off + size]);
        }
        if let Some(off) = offset(address, 0, ITCM_SIZE, size) {
            return Ok(&self.itcm[off..off + size]);
        }
        if let Some(off) = offset(address, 0x20200000, OCRAM_SIZE, size) {
            return Ok(&self.ocram[off..off + size]);
        }
        Err(self.region_fault(address, size, "read"))
    }
    #[inline(always)]
    fn region_mut(&mut self, address: u32, size: usize) -> Result<&mut [u8]> {
        if let Some(off) = offset(address, 0x80000000, HEAP_SIZE, size) {
            return Ok(&mut self.heap[off..off + size]);
        }
        if let Some(off) = offset(address, 0x20000000, DTCM_SIZE, size) {
            return Ok(&mut self.dtcm[off..off + size]);
        }
        if let Some(off) = offset(address, 0, ITCM_SIZE, size) {
            return Ok(&mut self.itcm[off..off + size]);
        }
        if let Some(off) = offset(address, 0x20200000, OCRAM_SIZE, size) {
            return Ok(&mut self.ocram[off..off + size]);
        }
        Err(self.region_fault(address, size, "write"))
    }
    /// An access outside every region: it starts in none (unmapped) or runs
    /// past the end of the one it starts in. Flash is read-only.
    #[cold]
    #[inline(never)]
    fn region_fault(&self, address: u32, size: usize, operation: &str) -> Fault {
        let mapped = matches!(
            address,
            0x00000000..=0x000034ff | 0x20000000..=0x2001ffff | 0x20200000..=0x2027ffff
                | 0x80000000..=0x81ffffff
        ) || (operation == "read" && (0x60020000..=0x6005cfff).contains(&address));
        if mapped {
            crossing_memory(self.pc, address, operation)
        } else {
            unmapped_memory(self.pc, address, size, operation)
        }
    }
    #[inline(always)]
    pub fn load(&self, address: u32, size: usize) -> Result<u64> {
        let b = self.region(address, size)?;
        Ok(match size {
            1 => b[0] as u64,
            2 => u16::from_le_bytes(b.try_into().unwrap()) as u64,
            4 => u32::from_le_bytes(b.try_into().unwrap()) as u64,
            8 => u64::from_le_bytes(b.try_into().unwrap()),
            _ => return Err(self.fault("invalid load width")),
        })
    }
    #[inline(always)]
    pub fn store(&mut self, address: u32, size: usize, value: u64) -> Result<()> {
        self.region_mut(address, size)?
            .copy_from_slice(&value.to_le_bytes()[..size]);
        Ok(())
    }
    pub fn read_f32(&self, address: u32) -> Result<f32> {
        Ok(f32::from_bits(self.load(address, 4)? as u32))
    }
    pub fn write_f32(&mut self, address: u32, value: f32) -> Result<()> {
        self.store(address, 4, value.to_bits() as u64)
    }
    pub fn allocate(&mut self, bytes: usize) -> Result<u32> {
        let start = (self.heap_cursor + 15) & !15;
        if start
            .checked_add(bytes)
            .is_none_or(|end| end > self.heap.len())
        {
            return Err(self.fault(format!(
                "firmware heap exhausted: allocation {bytes}, used {start}"
            )));
        }
        // Allocation tracing from the standalone project is intentionally absent:
        // environment lookups can allocate on Windows, including during reset.
        self.heap[start..start + bytes].fill(0);
        self.heap_cursor = start + bytes;
        Ok(0x80000000 + start as u32)
    }
    pub fn invoke(
        &mut self,
        entry: u32,
        args: &[u32],
        float_arg: Option<f32>,
        budget: u64,
    ) -> Result<u32> {
        for (i, &v) in args.iter().take(4).enumerate() {
            self.set_reg(0x20 + i * 4, 4, v as u64);
        }
        if let Some(v) = float_arg {
            self.set_reg(0x300, 4, v.to_bits() as u64);
        }
        self.set_reg(0x54, 4, 0x2001fff0);
        self.set_reg(0x58, 4, (RETURN_PC | 1) as u64);
        self.pc = entry & !1;
        self.budget = self.ticks.saturating_add(budget);
        while self.pc != RETURN_PC {
            if self.ticks >= self.budget {
                return Err(self.fault("instruction budget exceeded"));
            }
            #[cfg(test)]
            let fused = !self.reference_dispatch;
            #[cfg(not(test))]
            let fused = true;
            // No host call lives in a fused page, so its blocks step straight
            // through the fused tables.
            let next = if fused {
                crate::fused::fused(self, self.pc)
            } else {
                None
            };
            if let Some(next) = next {
                self.pc = next;
                if self.faulted.get() {
                    return Err(self.take_fault());
                }
                continue;
            }
            if !self.host_call()? {
                self.pc = if fused {
                    crate::fused::step(self, self.pc)?
                } else {
                    crate::generated::run_block(self, self.pc)?
                };
            }
        }
        Ok(self.reg(0x20, 4) as u32)
    }
    fn host_call(&mut self) -> Result<bool> {
        match self.pc {
            // Runtime alloc/free are replaced by a private, zeroed browser heap.
            0x600413e0 => {
                let p = self.allocate(self.reg(0x20, 4) as usize)?;
                self.set_reg(0x20, 4, p as u64);
            }
            0x600413f0 => {}
            // The analogue DAC controls external VCAs; web audio uses digital mixing.
            0x60030af4 => {}
            // These are exact byte operations, independent of ARM alignment paths.
            0x60041868 => {
                let dest = self.reg(0x20, 4) as u32;
                let byte = self.reg(0x24, 4) as u8;
                let len = self.reg(0x28, 4) as usize;
                self.region_mut(dest, len)?.fill(byte);
            }
            0x6002047c => {
                let dest = self.reg(0x20, 4) as u32;
                let source = self.reg(0x24, 4) as u32;
                let len = self.reg(0x28, 4) as usize;
                let temp = self.region(source, len)?.to_vec();
                self.region_mut(dest, len)?.copy_from_slice(&temp);
            }
            _ => return Ok(false),
        }
        self.pc = (self.reg(0x58, 4) as u32) & !1;
        self.ticks += 1;
        Ok(true)
    }
}

#[inline(always)]
pub fn mask(size: usize) -> u64 {
    if size == 8 {
        u64::MAX
    } else {
        (1u64 << (size * 8)) - 1
    }
}
#[inline(always)]
pub fn signed(value: u64, size: usize) -> i64 {
    ((value << (64 - size * 8)) as i64) >> (64 - size * 8)
}
#[inline(always)]
pub fn shl(value: u64, amount: u64) -> u64 {
    if amount >= 64 {
        0
    } else {
        value << amount
    }
}
#[inline(always)]
pub fn shr(value: u64, amount: u64) -> u64 {
    if amount >= 64 {
        0
    } else {
        value >> amount
    }
}
#[inline(always)]
pub fn sar(value: u64, amount: u64, size: usize) -> u64 {
    (signed(value, size) >> (amount.min(63))) as u64
}
#[inline(always)]
pub fn carry(a: u64, b: u64, size: usize) -> u64 {
    ((a as u128 + b as u128) > (mask(size) as u128)) as u64
}
#[inline(always)]
pub fn scarry(a: u64, b: u64, size: usize) -> u64 {
    let a = signed(a, size) as i128;
    let b = signed(b, size) as i128;
    let v = a + b;
    (v < -(1i128 << (size * 8 - 1)) || v > (1i128 << (size * 8 - 1)) - 1) as u64
}
#[inline(always)]
pub fn sborrow(a: u64, b: u64, size: usize) -> u64 {
    let a = signed(a, size) as i128;
    let b = signed(b, size) as i128;
    let v = a - b;
    (v < -(1i128 << (size * 8 - 1)) || v > (1i128 << (size * 8 - 1)) - 1) as u64
}
#[inline(always)]
pub fn udiv(a: u64, b: u64) -> u64 {
    if b == 0 {
        0
    } else {
        a / b
    }
}
#[inline(always)]
pub fn sdiv(a: u64, b: u64, size: usize) -> u64 {
    let a = signed(a, size);
    let b = signed(b, size);
    if b == 0 {
        0
    } else {
        a.wrapping_div(b) as u64
    }
}
#[inline(always)]
pub fn float_to_fixed(bits: u64, width: usize, fraction: u32) -> u64 {
    let v = (f32::from_bits(bits as u32) as f64) * 2f64.powi(fraction as i32);
    if width == 32 {
        (v as i32) as u32 as u64
    } else {
        (v as i16) as u16 as u64
    }
}

#[cfg(test)]
mod dispatch_tests {
    #[test]
    fn host_calls_are_never_fused() {
        for pc in [0x600413e0, 0x600413f0, 0x60030af4, 0x60041868, 0x6002047c] {
            assert!(!crate::fused::is_fused(pc), "{pc:#x}");
        }
        assert!(crate::fused::is_fused(0x208));
        assert!(crate::fused::is_fused(0x60040a38));
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
