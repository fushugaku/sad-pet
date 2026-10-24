//! Building blocks shared by the reverb group: the Main-style tap ring with a
//! decrementing cursor, the firmware's first-order sections (with their
//! bilinear re-derivation for other sample rates) and the two coefficient
//! setters the editors use (fixed step per sample, or a fixed duration).
use crate::{dsp::Coefficients, zoom::ORIGINAL_RATE};

/// `crate::dsp::flush` as one integer compare: keeps `x` exactly when
/// `1e-20 <= |x| < inf`, zero otherwise (tiny, infinite or NaN), the same
/// results bit for bit.
#[inline(always)]
pub fn flush(x: f32) -> f32 {
    const LOW: u32 = 1e-20f32.to_bits();
    const SPAN: u32 = f32::INFINITY.to_bits() - LOW;
    if (x.to_bits() & 0x7fff_ffff).wrapping_sub(LOW) < SPAN {
        x
    } else {
        0.0
    }
}

/// A ROM float word.
#[inline]
pub const fn f(bits: u32) -> f32 {
    f32::from_bits(bits)
}

/// Step 0x3af7c5ec per 44.1 kHz sample: decay, pre-delay duck and shape setters.
pub const STEP_FAST: f32 = f(0x3af7_c5ec);
/// Duration 0x44306666 (705.6 samples, 16 ms) of the `context[7]` setter.
pub const SETTER_SAMPLES: f32 = f(0x4430_6666);

/// A 44.1 kHz sample count at `rate`, rounded, at least one.
#[inline]
pub fn scale(samples: f32, rate: f32) -> usize {
    ((samples * rate / ORIGINAL_RATE).round() as usize).max(1)
}

/// Milliseconds to samples at `rate`.
#[inline]
pub fn ms(milliseconds: f32, rate: f32) -> usize {
    (milliseconds * rate * 0.001).round() as usize
}

/// Main's delay memory as the callbacks use it: one ring, taps read and
/// written at fixed offsets from a cursor that steps back one word per frame.
/// A tap read at `base + d` before the frame's writes returns what was
/// written at `base` exactly `d` frames ago.
pub struct Ring {
    buffer: Vec<f32>,
    mask: usize,
    position: usize,
}

impl Ring {
    /// Holds offsets `0..length`.
    pub fn new(length: usize) -> Self {
        let size = (length + 2).next_power_of_two();
        Self {
            buffer: vec![0.0; size],
            mask: size - 1,
            position: 0,
        }
    }
    #[inline(always)]
    pub fn read(&self, offset: usize) -> f32 {
        let at = self.position.wrapping_add(offset) & self.mask;
        debug_assert!(at < self.buffer.len());
        // SAFETY: `buffer.len() == mask + 1` (set in `new`, never resized).
        unsafe { *self.buffer.get_unchecked(at) }
    }
    #[inline(always)]
    pub fn write(&mut self, offset: usize, x: f32) {
        let at = self.position.wrapping_add(offset) & self.mask;
        debug_assert!(at < self.buffer.len());
        // SAFETY: `buffer.len() == mask + 1` (set in `new`, never resized).
        unsafe { *self.buffer.get_unchecked_mut(at) = flush(x) };
    }
    /// The frame's writes with the cursor, mask and buffer held in locals
    /// (stores into the buffer can't make the compiler reload them).
    #[inline(always)]
    pub fn writer(&mut self) -> RingWriter<'_> {
        RingWriter {
            buffer: &mut self.buffer,
            mask: self.mask,
            position: self.position,
        }
    }
    #[inline(always)]
    pub fn advance(&mut self) {
        self.position = self.position.wrapping_sub(1) & self.mask;
    }
    pub fn clear(&mut self) {
        self.buffer.fill(0.0);
    }
}

/// [`Ring::writer`]: the same `write`/`write_flushed` on local copies.
pub struct RingWriter<'a> {
    buffer: &'a mut [f32],
    mask: usize,
    position: usize,
}

impl RingWriter<'_> {
    #[inline(always)]
    pub fn write(&mut self, offset: usize, x: f32) {
        self.write_flushed_unchecked(offset, flush(x));
    }
    /// `write` for a value that is already flushed (flushing is idempotent).
    #[inline(always)]
    pub fn write_flushed(&mut self, offset: usize, x: f32) {
        debug_assert_eq!(x.to_bits(), flush(x).to_bits());
        self.write_flushed_unchecked(offset, x);
    }
    #[inline(always)]
    fn write_flushed_unchecked(&mut self, offset: usize, x: f32) {
        let at = self.position.wrapping_add(offset) & self.mask;
        debug_assert!(at < self.buffer.len());
        // SAFETY: `buffer.len() == mask + 1` (set in `Ring::new`, never resized).
        unsafe { *self.buffer.get_unchecked_mut(at) = x };
    }
}

/// Lays segments (each the longest delay read from it) out back to back in a
/// ring, one spare word between them, and returns their base offsets and the
/// ring length.
pub fn layout<const N: usize>(lengths: [usize; N]) -> ([usize; N], usize) {
    let mut bases = [0; N];
    let mut next = 0;
    for (base, length) in bases.iter_mut().zip(lengths) {
        *base = next;
        next += length + 2;
    }
    (bases, next)
}

/// Places a callback's ring at the host rate. Every ROM read belongs to the
/// segment of the nearest write offset at or below it; segments keep their
/// tap delays scaled from 44.1 kHz (at least `min_lengths`) and are laid out
/// back to back. Returns the write offsets, the read offsets and the length.
pub fn place<const W: usize, const R: usize>(
    writes: &[u32; W],
    reads: &[u32; R],
    min_lengths: [usize; W],
    rate: f32,
) -> ([usize; W], [usize; R], usize) {
    let mut lengths = min_lengths;
    let mut segment = [0usize; R];
    let mut delay = [0usize; R];
    for j in 0..R {
        let s = (0..W)
            .filter(|&s| writes[s] <= reads[j])
            .max_by_key(|&s| writes[s])
            .unwrap_or(0);
        segment[j] = s;
        delay[j] = scale((reads[j] - writes[s]) as f32, rate);
        lengths[s] = lengths[s].max(delay[j]);
    }
    let mut order: [usize; W] = std::array::from_fn(|i| i);
    order.sort_by_key(|&s| writes[s]);
    let mut bases = [0usize; W];
    let mut next = 0;
    for &s in &order {
        bases[s] = next;
        next += lengths[s] + 2;
    }
    let read = std::array::from_fn(|j| bases[segment[j]] + delay[j]);
    (bases, read, next)
}

/// The callbacks' first-order section `y = b0 x + b1 x[n-1] + a y[n-1]`.
#[derive(Debug, Clone, Copy, Default)]
pub struct FirstOrder {
    pub b0: f32,
    pub b1: f32,
    pub a: f32,
    x1: f32,
    y1: f32,
}

impl FirstOrder {
    #[cfg(test)]
    pub fn new(b0: f32, b1: f32, a: f32) -> Self {
        Self {
            b0,
            b1,
            a,
            x1: 0.0,
            y1: 0.0,
        }
    }

    /// The ROM section designed for `from` Hz re-derived at `to` Hz through its
    /// analog prototype (inverse bilinear transform, then forward again).
    pub fn resampled(b0: f32, b1: f32, a: f32, from: f32, to: f32) -> Self {
        let mut section = Self::default();
        section.set_resampled(b0, b1, a, from, to);
        section
    }

    pub fn set_resampled(&mut self, b0: f32, b1: f32, a: f32, from: f32, to: f32) {
        let (b0, b1, a) = (f64::from(b0), f64::from(b1), f64::from(a));
        let r = f64::from(to) / f64::from(from);
        let (sum, difference) = (b0 + b1, b0 - b1);
        let (n0, n1) = (sum + r * difference, sum - r * difference);
        let (d0, d1) = ((1.0 - a) + r * (1.0 + a), (1.0 - a) - r * (1.0 + a));
        self.b0 = (n0 / d0) as f32;
        self.b1 = (n1 / d0) as f32;
        self.a = (-d1 / d0) as f32;
    }

    /// Bilinear first-order low-pass (`highpass = false`) or high-pass with
    /// the 3 dB point at `hz`, as Main's designer `c00bc2e0` types 0 and 1.
    pub fn design(&mut self, highpass: bool, hz: f32, rate: f32) {
        let k =
            (std::f64::consts::PI * f64::from(hz.clamp(1.0, rate * 0.499)) / f64::from(rate)).tan();
        let norm = 1.0 / (1.0 + k);
        if highpass {
            self.b0 = norm as f32;
            self.b1 = -norm as f32;
        } else {
            self.b0 = (k * norm) as f32;
            self.b1 = (k * norm) as f32;
        }
        self.a = ((1.0 - k) * norm) as f32;
    }

    #[inline(always)]
    pub fn process(&mut self, x: f32) -> f32 {
        let y = flush(self.b0 * x + self.b1 * self.x1 + self.a * self.y1);
        self.x1 = x;
        self.y1 = y;
        y
    }

    /// Magnitude at `hz` (tests).
    #[cfg(test)]
    pub fn magnitude(&self, rate: f32, hz: f32) -> f32 {
        Coefficients {
            b0: self.b0,
            b1: self.b1,
            b2: 0.0,
            a1: -self.a,
            a2: 0.0,
        }
        .magnitude(rate, hz)
    }

    pub fn clear(&mut self) {
        self.x1 = 0.0;
        self.y1 = 0.0;
    }
}

/// `crate::dsp::Biquad` (transposed direct form II, same arithmetic) with
/// the group's single-compare [`flush`].
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

/// A ROM biquad `y = b0 x + b1 x1 + b2 x2 + c3 y1 + c4 y2` re-derived at `rate`.
pub fn rom_biquad(words: [f32; 5], rate: f32) -> Coefficients {
    Coefficients {
        b0: words[0],
        b1: words[1],
        b2: words[2],
        a1: -words[3],
        a2: -words[4],
    }
    .resample(ORIGINAL_RATE, rate)
}

/// A one-pole smoother `y = (1 - p) x + p y` at 44.1 kHz carried to `rate`
/// with the same time constant: returns `(gain, pole)`, keeping the DC gain
/// `gain / (1 - pole)` of the original pair.
pub fn one_pole_at(gain: f32, pole: f32, rate: f32) -> (f32, f32) {
    let pole_rate = f64::from(pole).powf(f64::from(ORIGINAL_RATE / rate));
    let dc = f64::from(gain) / (1.0 - f64::from(pole));
    ((dc * (1.0 - pole_rate)) as f32, pole_rate as f32)
}

/// A bank of coefficients moved by Main's float setter: linearly by a step
/// per sample until they reach their targets. Only ticks while one moves.
pub struct Glides<const N: usize> {
    pub value: [f32; N],
    target: [f32; N],
    step: [f32; N],
    moving: bool,
    rate_ratio: f32,
}

impl<const N: usize> Glides<N> {
    pub fn new(value: [f32; N], rate: f32) -> Self {
        Self {
            value,
            target: value,
            step: [0.0; N],
            moving: false,
            rate_ratio: ORIGINAL_RATE / rate,
        }
    }

    /// `context[8]`: move by `step_at_44k` per 44.1 kHz sample.
    pub fn to(&mut self, index: usize, target: f32, step_at_44k: f32) {
        self.target[index] = target;
        self.step[index] = step_at_44k * self.rate_ratio;
        self.moving |= self.value[index] != target;
    }

    /// `context[7]`: arrive after `samples_at_44k` 44.1 kHz samples.
    pub fn over(&mut self, index: usize, target: f32, samples_at_44k: f32) {
        let distance = (target - self.value[index]).abs();
        let step = (distance / samples_at_44k).max(target.abs() * 1e-6);
        self.to(index, target, step);
    }

    pub fn jump(&mut self, index: usize, value: f32) {
        self.value[index] = value;
        self.target[index] = value;
    }

    #[cfg(test)]
    pub fn settled(&self, index: usize) -> bool {
        self.value[index] == self.target[index]
    }

    #[inline]
    pub fn tick(&mut self) {
        if !self.moving {
            return;
        }
        let mut moving = false;
        for ((value, &target), &step) in self.value.iter_mut().zip(&self.target).zip(&self.step) {
            let difference = target - *value;
            if difference.abs() <= step * 1.1 {
                *value = target;
            } else {
                *value += step.copysign(difference);
                moving = true;
            }
        }
        self.moving = moving;
    }
}

/// A stepped parameter in pedal units.
#[inline]
pub fn stepped(value: f32, max: u16) -> usize {
    (value.round().max(0.0) as usize).min(usize::from(max))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring_delay_is_offset_minus_write_offset() {
        let mut ring = Ring::new(100);
        let mut seen = None;
        for n in 0..50 {
            if ring.read(17) == 3.0 && seen.is_none() {
                seen = Some(n);
            }
            ring.write(5, if n == 10 { 3.0 } else { 0.0 });
            ring.advance();
        }
        assert_eq!(seen, Some(22));
    }

    #[test]
    fn first_order_resample_keeps_the_corner() {
        // Hall tone1 entry 0: a 500 Hz one-pole low-pass at 44.1 kHz.
        let rom = FirstOrder::new(f(0x3d0c_ef57), f(0x3d0c_ef57), f(0x3f6e_6215));
        let at96 = FirstOrder::resampled(rom.b0, rom.b1, rom.a, 44_100.0, 96_000.0);
        for hz in [100.0, 500.0, 2000.0] {
            let a = rom.magnitude(44_100.0, hz);
            let b = at96.magnitude(96_000.0, hz);
            assert!((a - b).abs() < 0.02, "{hz}: {a} {b}");
        }
        let same = FirstOrder::resampled(rom.b0, rom.b1, rom.a, 44_100.0, 44_100.0);
        assert!((same.a - rom.a).abs() < 1e-6 && (same.b0 - rom.b0).abs() < 1e-6);
    }

    #[test]
    fn glides_follow_the_setters() {
        let mut glides = Glides::new([0.0, 1.0], 44_100.0);
        glides.over(0, 1.0, SETTER_SAMPLES);
        glides.to(1, 0.0, STEP_FAST);
        let mut n = 0;
        while !(glides.settled(0) && glides.settled(1)) {
            glides.tick();
            n += 1;
        }
        assert!((704..=708).contains(&n), "{n}");
        let mut glides = Glides::new([1.0], 44_100.0);
        glides.to(0, 0.0, STEP_FAST);
        let mut n = 0;
        while !glides.settled(0) {
            glides.tick();
            n += 1;
        }
        assert!((525..=531).contains(&n), "{n}");
    }
}
