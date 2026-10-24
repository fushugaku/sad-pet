//! Pieces shared by the delay_mod models.
use crate::dsp::DelayLine;

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
        let y = crate::dsp::flush(self.b0 * x + self.b1 * self.x1 + self.a1 * self.y1);
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
