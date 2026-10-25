//! Pieces shared by the delay_mod models.
use crate::dsp::Coefficients;

/// `1e-20` as compared by [`crate::dsp::flush`].
const FLUSH_FLOOR: u32 = 1e-20f32.to_bits();
const INFINITY_BITS: u32 = 0x7f80_0000;

/// [`crate::dsp::flush`] with one integer compare: keeps `x` exactly when
/// `1e-20 <= |x| < inf` (the magnitude bits of non-negative floats order like
/// integers, and NaN sits above infinity), returns `0.0` otherwise.
#[inline(always)]
pub fn flush(x: f32) -> f32 {
    if (x.to_bits() & 0x7fff_ffff).wrapping_sub(FLUSH_FLOOR) < INFINITY_BITS - FLUSH_FLOOR {
        x
    } else {
        0.0
    }
}

/// [`crate::zoom::Ramp`] with the same values: the snap distance
/// `step · 1.1` is computed once, and a settled ramp (current and target the
/// same bits, where `tick` would store the target again) returns at once.
#[derive(Debug, Clone, Copy)]
pub struct Ramp {
    pub current: f32,
    pub target: f32,
    step: f32,
    snap: f32,
}

impl Ramp {
    pub fn new(value: f32, step_at_44k: f32, sample_rate: f32) -> Self {
        let step = step_at_44k * crate::zoom::ORIGINAL_RATE / sample_rate;
        Self {
            current: value,
            target: value,
            step,
            snap: step * 1.1,
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
        if difference.abs() <= self.snap {
            self.current = self.target;
        } else if difference > 0.0 {
            self.current += self.step;
        } else {
            self.current -= self.step;
        }
        self.current
    }
}

/// [`crate::dsp::Biquad`] (transposed direct form II) with the same
/// arithmetic, using the cheaper [`flush`].
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

/// [`crate::dsp::DelayLine`] with the same layout and results: a power-of-two
/// ring whose reads skip the bounds check (the index is always masked to the
/// buffer length) and whose interpolation limit is computed once.
#[derive(Debug, Clone)]
pub struct DelayLine {
    buffer: Box<[f32]>,
    mask: usize,
    write: usize,
    /// `capacity() as f32`, the `read_linear` clamp.
    limit: f32,
}

impl DelayLine {
    /// Holds at least `max_delay` samples of history plus interpolation room.
    pub fn new(max_delay: usize) -> Self {
        let length = (max_delay + 4).next_power_of_two();
        Self {
            buffer: vec![0.0; length].into_boxed_slice(),
            mask: length - 1,
            write: 0,
            limit: (length - 4) as f32,
        }
    }

    /// For a maximum delay in seconds at `sample_rate`.
    pub fn with_seconds(seconds: f32, sample_rate: f32) -> Self {
        Self::new((seconds * sample_rate).ceil() as usize + 1)
    }

    pub fn capacity(&self) -> usize {
        self.mask - 3
    }

    #[inline(always)]
    fn at(&self, index: usize) -> f32 {
        let index = index & self.mask;
        debug_assert!(index < self.buffer.len());
        // SAFETY: `buffer.len() == mask + 1` (a power of two), so a masked
        // index is always in range.
        unsafe { *self.buffer.get_unchecked(index) }
    }

    #[inline(always)]
    pub fn write(&mut self, x: f32) {
        self.write = (self.write + 1) & self.mask;
        debug_assert!(self.write < self.buffer.len());
        // SAFETY: as in `at`.
        unsafe { *self.buffer.get_unchecked_mut(self.write) = x };
    }

    /// The sample written `delay` frames before the latest one.
    #[inline(always)]
    pub fn read(&self, delay: usize) -> f32 {
        self.at(self.write.wrapping_sub(delay))
    }

    /// Linear interpolation, `delay` clamped to `[0, capacity]` exactly like
    /// `f32::clamp` (NaN passes through).
    #[inline(always)]
    pub fn read_linear(&self, delay: f32) -> f32 {
        let mut delay = delay;
        if delay < 0.0 {
            delay = 0.0;
        }
        if delay > self.limit {
            delay = self.limit;
        }
        let whole = delay as usize;
        let fraction = delay - whole as f32;
        let a = self.read(whole);
        let b = self.read(whole + 1);
        a + (b - a) * fraction
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0.0);
    }
}

/// A delay read head that moves to a new delay time by crossfading from the
/// old position to the new one. The originals mute the tap while a time edit
/// lands (`*_tapmuteMute`, reopened by `*_TapOpen` after the new time); a
/// short crossfade keeps the echoes and avoids the click when SAD PET moves
/// the time continuously. A request that arrives during a fade starts as soon
/// as the running fade ends, so the head never jumps.
#[derive(Debug, Clone, Copy)]
pub struct FadeTap {
    from: f32,
    to: f32,
    pending: f32,
    mix: f32,
    step: f32,
}

impl FadeTap {
    /// `delay` in samples (fractional allowed), crossfade length in seconds.
    pub fn new(delay: f32, fade_seconds: f32, sample_rate: f32) -> Self {
        Self {
            from: delay,
            to: delay,
            pending: delay,
            mix: 1.0,
            step: 1.0 / (fade_seconds * sample_rate).max(1.0),
        }
    }

    /// Request a new delay in samples.
    pub fn set(&mut self, delay: f32) {
        self.pending = delay;
    }

    /// Move immediately (used by `clear`).
    pub fn jump(&mut self, delay: f32) {
        self.from = delay;
        self.to = delay;
        self.pending = delay;
        self.mix = 1.0;
    }

    /// The delay the head is moving to.
    pub fn target(&self) -> f32 {
        self.pending
    }

    /// Read the line (after this frame's `write`): `delay` samples back.
    #[inline]
    pub fn read(&mut self, line: &DelayLine) -> f32 {
        if self.mix >= 1.0 {
            if self.pending == self.to {
                return line.read_linear(self.to);
            }
            self.from = self.to;
            self.to = self.pending;
            self.mix = 0.0;
        }
        self.mix = (self.mix + self.step).min(1.0);
        let a = line.read_linear(self.from);
        let b = line.read_linear(self.to);
        a + (b - a) * self.mix
    }
}

/// Main's `context[7]` setter (c00cc94c): the coefficient moves linearly to
/// its target in `705.6` samples at 44.1 kHz (16 ms), with a step of at least
/// `1e-6 · |target|`.
#[derive(Debug, Clone, Copy)]
pub struct Glide {
    pub current: f32,
    target: f32,
    step: f32,
    samples: f32,
}

impl Glide {
    pub fn new(value: f32, sample_rate: f32) -> Self {
        Self {
            current: value,
            target: value,
            step: 0.0,
            samples: (705.6 * sample_rate / crate::zoom::ORIGINAL_RATE).max(1.0),
        }
    }

    pub fn set(&mut self, target: f32) {
        if target != self.target {
            self.target = target;
            self.step = ((self.current - target).abs() / self.samples).max(1e-6 * target.abs());
        }
    }

    pub fn jump(&mut self, value: f32) {
        self.current = value;
        self.target = value;
    }

    #[inline]
    pub fn tick(&mut self) -> f32 {
        // Settled: the general path would store the target again.
        if self.current.to_bits() == self.target.to_bits() {
            return self.current;
        }
        let difference = self.target - self.current;
        if difference.abs() <= self.step {
            self.current = self.target;
        } else {
            self.current += self.step.copysign(difference);
        }
        self.current
    }
}

/// First-order section in the originals' sign convention:
/// `y = b0 x + b1 x[n-1] + a1 y[n-1]` (note `+ a1`).
#[derive(Debug, Clone, Copy, Default)]
pub struct FirstOrder {
    pub b0: f32,
    pub b1: f32,
    pub a1: f32,
    x1: f32,
    y1: f32,
}

impl FirstOrder {
    pub const fn new(b0: f32, b1: f32, a1: f32) -> Self {
        Self {
            b0,
            b1,
            a1,
            x1: 0.0,
            y1: 0.0,
        }
    }

    pub fn set(&mut self, b0: f32, b1: f32, a1: f32) {
        self.b0 = b0;
        self.b1 = b1;
        self.a1 = a1;
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let y = flush(self.b0 * x + self.b1 * self.x1 + self.a1 * self.y1);
        self.x1 = x;
        self.y1 = y;
        y
    }

    /// Re-derive a first-order section designed at `from_rate` (a ROM entry)
    /// for `to_rate`, keeping its analog prototype (the bilinear transform is
    /// inverted at `from_rate` and applied again at `to_rate`).
    pub fn resampled(b0: f32, b1: f32, a1: f32, from_rate: f32, to_rate: f32) -> (f32, f32, f32) {
        if (from_rate - to_rate).abs() < 0.5 {
            return (b0, b1, a1);
        }
        let (b0, b1, a1) = (f64::from(b0), f64::from(b1), f64::from(a1));
        let r = f64::from(to_rate) / f64::from(from_rate);
        let (n0, n1) = (b0 + b1, (b0 - b1) * r);
        let (d0, d1) = (1.0 - a1, (1.0 + a1) * r);
        let norm = d0 + d1;
        (
            ((n0 + n1) / norm) as f32,
            ((n0 - n1) / norm) as f32,
            (-(d0 - d1) / norm) as f32,
        )
    }

    pub fn clear(&mut self) {
        self.x1 = 0.0;
        self.y1 = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_flush_matches_dsp_flush() {
        let mut bits = 0u32;
        loop {
            let x = f32::from_bits(bits);
            assert_eq!(
                flush(x).to_bits(),
                crate::dsp::flush(x).to_bits(),
                "{bits:#x}"
            );
            let next = bits.wrapping_add(0x1357);
            if next < bits {
                break;
            }
            bits = next;
        }
        for x in [
            1e-20f32,
            -1e-20,
            f32::from_bits(1e-20f32.to_bits() - 1),
            f32::MAX,
            f32::INFINITY,
            f32::NEG_INFINITY,
            f32::NAN,
            0.0,
            -0.0,
        ] {
            assert_eq!(flush(x).to_bits(), crate::dsp::flush(x).to_bits(), "{x}");
        }
    }

    #[test]
    fn local_ramp_matches_zoom_ramp() {
        for sample_rate in [44_100.0, 48_000.0, 96_000.0] {
            let mut a = Ramp::new(0.3, crate::zoom::DEFAULT_STEP, sample_rate);
            let mut b = crate::zoom::Ramp::new(0.3, crate::zoom::DEFAULT_STEP, sample_rate);
            for i in 0..20_000 {
                if i % 3000 == 0 {
                    let target = [1.0, -0.0, 0.0, 0.25, 0.2501, -1.0, 0.3][i / 3000 % 7];
                    a.set(target);
                    b.set(target);
                }
                assert_eq!(a.tick().to_bits(), b.tick().to_bits(), "{i}");
                assert_eq!(a.current.to_bits(), b.current.to_bits());
            }
        }
    }

    #[test]
    fn local_delay_line_matches_dsp() {
        let mut a = DelayLine::new(37);
        let mut b = crate::dsp::DelayLine::new(37);
        for i in 0..500 {
            let x = (i as f32 * 0.37).fract() - 0.5;
            a.write(x);
            b.write(x);
            for d in [-3.0f32, 0.0, 0.25, 7.5, 33.9, 59.0, 80.0, f32::NAN] {
                assert_eq!(
                    a.read_linear(d).to_bits(),
                    b.read_linear(d).to_bits(),
                    "{d}"
                );
            }
            assert_eq!(a.read(5).to_bits(), b.read(5).to_bits());
        }
    }

    #[test]
    fn fade_tap_reaches_the_new_delay_without_a_jump() {
        let mut line = DelayLine::new(1000);
        let mut tap = FadeTap::new(100.0, 0.001, 48_000.0);
        // A ramp signal: the read value is (now - delay).
        let mut previous = 0.0;
        for i in 0..2000 {
            line.write(i as f32);
            if i == 500 {
                tap.set(300.0);
            }
            let y = tap.read(&line);
            if i > 200 {
                assert!((y - previous).abs() <= 6.0, "{i}: {previous} -> {y}");
            }
            previous = y;
        }
        assert!((previous - (1999.0 - 300.0)).abs() < 1e-3);
    }

    #[test]
    fn glide_takes_16_ms() {
        let mut glide = Glide::new(0.0, 48_000.0);
        glide.set(1.0);
        let samples = (705.6f32 * 48_000.0 / 44_100.0).ceil() as usize;
        for _ in 0..samples - 2 {
            assert!(glide.tick() < 1.0);
        }
        glide.tick();
        glide.tick();
        assert_eq!(glide.tick(), 1.0);
    }

    #[test]
    fn first_order_resample_keeps_dc_gain() {
        let (b0, b1, a1) =
            FirstOrder::resampled(0.024_358_96, 0.024_358_96, 0.931_184, 44_100.0, 96_000.0);
        let dc = (b0 + b1) / (1.0 - a1);
        assert!((dc - 0.708).abs() < 0.01, "{dc}");
    }
}
