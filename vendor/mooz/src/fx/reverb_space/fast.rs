//! Hot-path versions of the shared `crate::dsp` / `crate::zoom` blocks for
//! this group. Every one keeps the original arithmetic in the original order
//! (renders stay bit-identical); they only drop work: the ring index is masked
//! into a power-of-two buffer and used unchecked, the denormal flush is one
//! compare and a select, ramps return early once arrived, first-order
//! sections skip the always-zero second state, constant fractional taps are
//! split once, and a [`Bank`] keeps a reverb's parallel lines in one buffer
//! behind one cursor. Everything is forced inline.
use crate::dsp::Coefficients;
use std::sync::atomic::{AtomicUsize, Ordering};

static STAGGER: AtomicUsize = AtomicUsize::new(0);

/// `crate::dsp::flush` as one integer range check and a select: zero for
/// |x| < 1e-20, infinities and NaN. For the magnitude bits of a float,
/// `1e-20 <= |x| < inf` is `LO <= bits < INF` (non-negative floats order like
/// their bits; NaN bits lie above infinity's), one wrapping compare.
#[inline(always)]
pub fn flush(x: f32) -> f32 {
    const LO: u32 = 1e-20f32.to_bits();
    const INF: u32 = f32::INFINITY.to_bits();
    if (x.to_bits() & 0x7fff_ffff).wrapping_sub(LO) < INF - LO {
        x
    } else {
        0.0
    }
}

/// `crate::dsp::Biquad` (transposed direct form II, same operation order).
#[derive(Debug, Clone, Copy, Default)]
pub struct Biquad {
    pub coefficients: Coefficients,
    s1: f32,
    s2: f32,
}

impl Biquad {
    pub fn new(coefficients: Coefficients) -> Self {
        Self {
            coefficients,
            s1: 0.0,
            s2: 0.0,
        }
    }
    #[inline(always)]
    pub fn process(&mut self, x: f32) -> f32 {
        let c = &self.coefficients;
        let y = c.b0 * x + self.s1;
        self.s1 = flush(c.b1 * x - c.a1 * y + self.s2);
        self.s2 = flush(c.b2 * x - c.a2 * y);
        y
    }
    pub fn clear(&mut self) {
        self.s1 = 0.0;
        self.s2 = 0.0;
    }
}

/// `crate::dsp::DelayLine`: same length rule, cursor and interpolation, with
/// the masked index used unchecked (it is below the power-of-two length by
/// construction).
#[derive(Debug, Clone)]
pub struct DelayLine {
    buffer: Box<[f32]>,
    mask: usize,
    write: usize,
    /// `capacity() as f32`, the clamp of the interpolated reads.
    limit: f32,
}

impl DelayLine {
    pub fn new(max_delay: usize) -> Self {
        let length = (max_delay + 4).next_power_of_two();
        // The output only depends on positions relative to the cursor, so the
        // start can be anywhere: staggering it by a few cache lines per line
        // keeps the cursors of a model's lines (all advanced once per frame)
        // off the same cache set when the buffers are page aligned.
        let start = STAGGER.fetch_add(1, Ordering::Relaxed).wrapping_mul(7 * 16);
        Self {
            buffer: vec![0.0; length].into_boxed_slice(),
            mask: length - 1,
            write: start & (length - 1),
            limit: (length - 4) as f32,
        }
    }
    pub fn capacity(&self) -> usize {
        self.mask - 3
    }
    #[inline(always)]
    fn at(&self, index: usize) -> f32 {
        let index = index & self.mask;
        debug_assert!(index < self.buffer.len());
        // SAFETY: `buffer.len() == mask + 1` (a power of two) and the index is masked.
        unsafe { *self.buffer.get_unchecked(index) }
    }
    #[inline(always)]
    pub fn write(&mut self, x: f32) {
        self.write = (self.write + 1) & self.mask;
        debug_assert!(self.write < self.buffer.len());
        // SAFETY: as in `at`.
        unsafe { *self.buffer.get_unchecked_mut(self.write) = x };
    }
    #[inline(always)]
    pub fn read(&self, delay: usize) -> f32 {
        self.at(self.write.wrapping_sub(delay))
    }
    #[inline(always)]
    pub fn read_linear(&self, delay: f32) -> f32 {
        let delay = delay.clamp(0.0, self.limit);
        let whole = delay as usize;
        let fraction = delay - whole as f32;
        let a = self.read(whole);
        let b = self.read(whole + 1);
        a + (b - a) * fraction
    }
    /// The split of a constant `read_linear` delay, computed once.
    pub fn fixed(&self, delay: f32) -> Tap {
        let delay = delay.clamp(0.0, self.capacity() as f32);
        let whole = delay as usize;
        Tap {
            whole,
            fraction: delay - whole as f32,
        }
    }
    /// `read_linear` at a delay split by [`DelayLine::fixed`].
    #[inline(always)]
    pub fn read_fixed(&self, tap: Tap) -> f32 {
        let a = self.read(tap.whole);
        let b = self.read(tap.whole + 1);
        a + (b - a) * tap.fraction
    }
    pub fn clear(&mut self) {
        self.buffer.fill(0.0);
    }
}

/// A constant fractional delay of [`DelayLine::read_linear`], split once.
#[derive(Debug, Clone, Copy, Default)]
pub struct Tap {
    whole: usize,
    fraction: f32,
}

/// [`Biquad`] for first-order coefficients (`b2 = a2 = 0`): its second state
/// is then always `flush(0 x - 0 y) = 0`, and adding that zero to the first
/// state only turns -0 into +0 before the flush, so it is left out.
#[derive(Debug, Clone, Copy, Default)]
pub struct Biquad1 {
    b0: f32,
    b1: f32,
    a1: f32,
    s1: f32,
}

impl Biquad1 {
    pub fn new(c: Coefficients) -> Self {
        assert!(c.b2 == 0.0 && c.a2 == 0.0, "not a first-order section");
        Self {
            b0: c.b0,
            b1: c.b1,
            a1: c.a1,
            s1: 0.0,
        }
    }
    pub fn set(&mut self, c: Coefficients) {
        *self = Self {
            s1: self.s1,
            ..Self::new(c)
        };
    }
    #[inline(always)]
    pub fn process(&mut self, x: f32) -> f32 {
        let y = self.b0 * x + self.s1;
        self.s1 = flush(self.b1 * x - self.a1 * y);
        y
    }
    pub fn clear(&mut self) {
        self.s1 = 0.0;
    }
}

/// `crate::zoom::Ramp` with an early exit once it has arrived: with
/// `current == target` the original sets `current = target` again (the step
/// is never negative), so returning at once gives the same value.
#[derive(Debug, Clone, Copy)]
pub struct Ramp {
    pub current: f32,
    pub target: f32,
    step: f32,
}

impl Ramp {
    pub fn new(value: f32, step_at_44k: f32, sample_rate: f32) -> Self {
        Self {
            current: value,
            target: value,
            step: step_at_44k * crate::zoom::ORIGINAL_RATE / sample_rate,
        }
    }
    pub fn set(&mut self, target: f32) {
        self.target = target;
    }
    pub fn jump(&mut self, value: f32) {
        self.current = value;
        self.target = value;
    }
    #[inline(always)]
    pub fn tick(&mut self) -> f32 {
        if self.current.to_bits() == self.target.to_bits() {
            return self.current;
        }
        let difference = self.target - self.current;
        if difference.abs() <= self.step * 1.1 {
            self.current = self.target;
        } else if difference > 0.0 {
            self.current += self.step;
        } else {
            self.current -= self.step;
        }
        self.current
    }
}

/// `crate::dsp::Smoother` with an early exit once it has arrived: with
/// `current == target` the original adds `0 * coefficient` and then snaps to
/// the target, so returning the target at once gives the same value.
#[derive(Debug, Clone, Copy)]
pub struct Smoother {
    pub current: f32,
    pub target: f32,
    coefficient: f32,
}

impl Smoother {
    pub fn new(value: f32, seconds: f32, sample_rate: f32) -> Self {
        Self {
            current: value,
            target: value,
            coefficient: crate::dsp::one_pole_coefficient(seconds, sample_rate),
        }
    }
    pub fn set(&mut self, target: f32) {
        self.target = target;
    }
    pub fn jump(&mut self, value: f32) {
        self.current = value;
        self.target = value;
    }
    #[inline(always)]
    pub fn tick(&mut self) -> f32 {
        if self.current.to_bits() == self.target.to_bits() {
            return self.current;
        }
        self.current += (self.target - self.current) * self.coefficient;
        if (self.target - self.current).abs() < 1e-7 {
            self.current = self.target;
        }
        self.current
    }
}

/// `N` delay lines that share one write cursor (one buffer, one power-of-two
/// region per line, a cache line apart): region `i` holds `delay[i]` frames
/// (plus extra taps), is read before this frame's write and written once per
/// frame; [`Bank::advance`] moves the cursor after every region has been
/// written. A region is at least as long as the line it replaces, so every
/// integer read returns the same sample.
#[derive(Debug, Clone)]
pub struct Bank<const N: usize> {
    buffer: Box<[f32]>,
    stride: usize,
    mask: usize,
    cursor: usize,
    pub delay: [usize; N],
}

impl<const N: usize> Bank<N> {
    /// `delay[i]` frames per region, `longest` the longest read (taps).
    pub fn new(delay: [usize; N], longest: usize) -> Self {
        let longest = delay.iter().copied().fold(longest, usize::max);
        let length = (longest + 6).next_power_of_two();
        // One cache line between regions keeps the N cursors off the same set.
        let stride = length + 16;
        Self {
            buffer: vec![0.0; stride * N].into_boxed_slice(),
            stride,
            mask: length - 1,
            cursor: 0,
            delay: delay.map(|d| d.max(1)),
        }
    }
    #[inline(always)]
    fn at(&self, region: usize, back: usize) -> f32 {
        debug_assert!(region < N);
        let index = region * self.stride + (self.cursor.wrapping_sub(back) & self.mask);
        debug_assert!(index < self.buffer.len());
        // SAFETY: region < N and the masked offset < stride; the buffer holds
        // N * stride samples.
        unsafe { *self.buffer.get_unchecked(index) }
    }
    /// The sample written `back` frames before the latest write (the
    /// `DelayLine::read` argument).
    #[inline(always)]
    pub fn read(&self, region: usize, back: usize) -> f32 {
        self.at(region, back)
    }
    /// The sample written `delay[region]` frames ago.
    #[inline(always)]
    pub fn out(&self, region: usize) -> f32 {
        self.at(region, self.delay[region] - 1)
    }
    /// An extra tap `delay` frames back (`1 <= delay < region length`).
    #[inline(always)]
    pub fn tap(&self, region: usize, delay: usize) -> f32 {
        self.at(region, delay - 1)
    }
    /// This frame's write of `region` as is (no flush).
    #[inline(always)]
    pub fn write_raw(&mut self, region: usize, x: f32) {
        debug_assert!(region < N);
        let index = region * self.stride + (self.cursor.wrapping_add(1) & self.mask);
        debug_assert!(index < self.buffer.len());
        // SAFETY: as in `at`.
        unsafe { *self.buffer.get_unchecked_mut(index) = x };
    }
    #[inline(always)]
    pub fn write(&mut self, region: usize, x: f32) {
        self.write_raw(region, flush(x));
    }
    /// This frame's view with the cursor, mask and stride in locals (stores
    /// into the buffer then cannot force them to be reloaded); call
    /// [`Bank::advance`] once it is dropped.
    #[inline(always)]
    pub fn frame(&mut self) -> BankFrame<'_, N> {
        BankFrame {
            buffer: &mut self.buffer,
            delay: &self.delay,
            stride: self.stride,
            mask: self.mask,
            cursor: self.cursor,
            write: self.cursor.wrapping_add(1) & self.mask,
        }
    }
    /// Move the cursor once every region has been written this frame.
    #[inline(always)]
    pub fn advance(&mut self) {
        self.cursor = self.cursor.wrapping_add(1) & self.mask;
    }
    pub fn clear(&mut self) {
        self.buffer.fill(0.0);
    }
}

/// One frame of a [`Bank`], see [`Bank::frame`].
pub struct BankFrame<'a, const N: usize> {
    buffer: &'a mut [f32],
    delay: &'a [usize; N],
    stride: usize,
    mask: usize,
    cursor: usize,
    write: usize,
}

impl<const N: usize> BankFrame<'_, N> {
    #[inline(always)]
    fn at(&self, region: usize, back: usize) -> f32 {
        debug_assert!(region < N);
        let index = region * self.stride + (self.cursor.wrapping_sub(back) & self.mask);
        debug_assert!(index < self.buffer.len());
        // SAFETY: as in `Bank::at`.
        unsafe { *self.buffer.get_unchecked(index) }
    }
    /// [`Bank::out`].
    #[inline(always)]
    pub fn out(&self, region: usize) -> f32 {
        self.at(region, self.delay[region] - 1)
    }
    /// [`Bank::tap`].
    #[inline(always)]
    pub fn tap(&self, region: usize, delay: usize) -> f32 {
        self.at(region, delay - 1)
    }
    /// [`Bank::write_raw`].
    #[inline(always)]
    pub fn write_raw(&mut self, region: usize, x: f32) {
        debug_assert!(region < N);
        let index = region * self.stride + self.write;
        debug_assert!(index < self.buffer.len());
        // SAFETY: as in `Bank::at` (`write` is masked).
        unsafe { *self.buffer.get_unchecked_mut(index) = x };
    }
    /// [`Bank::write`].
    #[inline(always)]
    pub fn write(&mut self, region: usize, x: f32) {
        self.write_raw(region, flush(x));
    }
}
