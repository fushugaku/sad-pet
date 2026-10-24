//! Shared building blocks. Everything here allocates only in `new`.
use std::f32::consts::{PI, TAU};

/// Flush denormals and non-finite values that would otherwise poison feedback.
#[inline]
pub fn flush(x: f32) -> f32 {
    if x.abs() < 1e-20 || !x.is_finite() {
        0.0
    } else {
        x
    }
}

#[inline]
pub fn db_to_gain(db: f32) -> f32 {
    10f32.powf(db * 0.05)
}

#[inline]
pub fn gain_to_db(gain: f32) -> f32 {
    20.0 * gain.max(1e-12).log10()
}

/// Padé tanh, accurate to ~1e-3 and exactly ±1 beyond ±3.
#[inline]
pub fn tanh(x: f32) -> f32 {
    if x <= -3.0 {
        -1.0
    } else if x >= 3.0 {
        1.0
    } else {
        let x2 = x * x;
        x * (27.0 + x2) / (27.0 + 9.0 * x2)
    }
}

/// Sine of a phase in turns (any real value), max error ~1e-4.
#[inline]
pub fn sin_turns(phase: f32) -> f32 {
    let x = phase - phase.floor();
    // Map to [-0.5, 0.5) and use an odd polynomial on the half period.
    let x = if x >= 0.5 { x - 1.0 } else { x };
    let y = x * 2.0; // [-1, 1): sin(pi * y)
    let ay = y.abs();
    let s = 4.0 * ay * (1.0 - ay); // parabola for sin(pi*|y|)
    let s = s * (0.775 + 0.225 * s); // refinement
    if y < 0.0 { -s } else { s }
}

/// Cosine of a phase in turns.
#[inline]
pub fn cos_turns(phase: f32) -> f32 {
    sin_turns(phase + 0.25)
}

/// Seconds to a one-pole coefficient: `y += (x - y) * coefficient`.
#[inline]
pub fn one_pole_coefficient(seconds: f32, sample_rate: f32) -> f32 {
    if seconds <= 0.0 {
        1.0
    } else {
        1.0 - (-1.0 / (seconds * sample_rate)).exp()
    }
}

/// Cutoff in Hz to a one-pole low-pass coefficient.
#[inline]
pub fn one_pole_hz(hz: f32, sample_rate: f32) -> f32 {
    1.0 - (-TAU * hz.clamp(0.0, sample_rate * 0.49) / sample_rate).exp()
}

/// Deterministic white noise (xorshift32).
#[derive(Debug, Clone)]
pub struct Noise(u32);
impl Noise {
    pub fn new(seed: u32) -> Self {
        Self(seed.max(1))
    }
    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;
        x
    }
    /// Uniform in [-1, 1).
    #[inline]
    pub fn bipolar(&mut self) -> f32 {
        (self.next_u32() as i32) as f32 * (1.0 / 2_147_483_648.0)
    }
    /// Uniform in [0, 1).
    #[inline]
    pub fn unit(&mut self) -> f32 {
        (self.next_u32() >> 8) as f32 * (1.0 / 16_777_216.0)
    }
}

/// One-pole low-pass, also usable as a parameter smoother.
#[derive(Debug, Clone, Copy, Default)]
pub struct OnePole {
    pub coefficient: f32,
    pub state: f32,
}
impl OnePole {
    pub fn new(coefficient: f32) -> Self {
        Self {
            coefficient,
            state: 0.0,
        }
    }
    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        self.state = flush(self.state + (x - self.state) * self.coefficient);
        self.state
    }
    /// High-pass output of the same pole.
    #[inline]
    pub fn highpass(&mut self, x: f32) -> f32 {
        x - self.process(x)
    }
    pub fn clear(&mut self) {
        self.state = 0.0;
    }
}

/// Exponential parameter smoother with a fixed time constant.
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
            coefficient: one_pole_coefficient(seconds, sample_rate),
        }
    }
    pub fn set(&mut self, target: f32) {
        self.target = target;
    }
    pub fn jump(&mut self, value: f32) {
        self.current = value;
        self.target = value;
    }
    #[inline]
    pub fn tick(&mut self) -> f32 {
        self.current += (self.target - self.current) * self.coefficient;
        if (self.target - self.current).abs() < 1e-7 {
            self.current = self.target;
        }
        self.current
    }
}

/// Biquad coefficients, normalized so that a0 = 1:
/// `y = b0 x + b1 x1 + b2 x2 - a1 y1 - a2 y2`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coefficients {
    pub b0: f32,
    pub b1: f32,
    pub b2: f32,
    pub a1: f32,
    pub a2: f32,
}

impl Default for Coefficients {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Coefficients {
    pub const IDENTITY: Self = Self {
        b0: 1.0,
        b1: 0.0,
        b2: 0.0,
        a1: 0.0,
        a2: 0.0,
    };

    fn normalize(b0: f64, b1: f64, b2: f64, a0: f64, a1: f64, a2: f64) -> Self {
        Self {
            b0: (b0 / a0) as f32,
            b1: (b1 / a0) as f32,
            b2: (b2 / a0) as f32,
            a1: (a1 / a0) as f32,
            a2: (a2 / a0) as f32,
        }
    }

    fn omega(sample_rate: f32, hz: f32) -> (f64, f64) {
        let hz = (hz as f64).clamp(1.0, sample_rate as f64 * 0.4999);
        let w = std::f64::consts::TAU * hz / sample_rate as f64;
        (w.sin(), w.cos())
    }

    /// RBJ cookbook designs.
    pub fn lowpass(sample_rate: f32, hz: f32, q: f32) -> Self {
        let (s, c) = Self::omega(sample_rate, hz);
        let alpha = s / (2.0 * q.max(1e-3) as f64);
        Self::normalize(
            (1.0 - c) / 2.0,
            1.0 - c,
            (1.0 - c) / 2.0,
            1.0 + alpha,
            -2.0 * c,
            1.0 - alpha,
        )
    }
    pub fn highpass(sample_rate: f32, hz: f32, q: f32) -> Self {
        let (s, c) = Self::omega(sample_rate, hz);
        let alpha = s / (2.0 * q.max(1e-3) as f64);
        Self::normalize(
            (1.0 + c) / 2.0,
            -(1.0 + c),
            (1.0 + c) / 2.0,
            1.0 + alpha,
            -2.0 * c,
            1.0 - alpha,
        )
    }
    /// Constant 0 dB peak gain band-pass.
    pub fn bandpass(sample_rate: f32, hz: f32, q: f32) -> Self {
        let (s, c) = Self::omega(sample_rate, hz);
        let alpha = s / (2.0 * q.max(1e-3) as f64);
        Self::normalize(alpha, 0.0, -alpha, 1.0 + alpha, -2.0 * c, 1.0 - alpha)
    }
    pub fn notch(sample_rate: f32, hz: f32, q: f32) -> Self {
        let (s, c) = Self::omega(sample_rate, hz);
        let alpha = s / (2.0 * q.max(1e-3) as f64);
        Self::normalize(1.0, -2.0 * c, 1.0, 1.0 + alpha, -2.0 * c, 1.0 - alpha)
    }
    pub fn allpass(sample_rate: f32, hz: f32, q: f32) -> Self {
        let (s, c) = Self::omega(sample_rate, hz);
        let alpha = s / (2.0 * q.max(1e-3) as f64);
        Self::normalize(
            1.0 - alpha,
            -2.0 * c,
            1.0 + alpha,
            1.0 + alpha,
            -2.0 * c,
            1.0 - alpha,
        )
    }
    pub fn peaking(sample_rate: f32, hz: f32, q: f32, gain_db: f32) -> Self {
        let (s, c) = Self::omega(sample_rate, hz);
        let a = 10f64.powf(gain_db as f64 / 40.0);
        let alpha = s / (2.0 * q.max(1e-3) as f64);
        Self::normalize(
            1.0 + alpha * a,
            -2.0 * c,
            1.0 - alpha * a,
            1.0 + alpha / a,
            -2.0 * c,
            1.0 - alpha / a,
        )
    }
    /// Shelves with slope parameter S = 1 when `q` is `FRAC_1_SQRT_2`.
    pub fn low_shelf(sample_rate: f32, hz: f32, q: f32, gain_db: f32) -> Self {
        let (s, c) = Self::omega(sample_rate, hz);
        let a = 10f64.powf(gain_db as f64 / 40.0);
        let alpha = s / (2.0 * q.max(1e-3) as f64);
        let k = 2.0 * a.sqrt() * alpha;
        Self::normalize(
            a * ((a + 1.0) - (a - 1.0) * c + k),
            2.0 * a * ((a - 1.0) - (a + 1.0) * c),
            a * ((a + 1.0) - (a - 1.0) * c - k),
            (a + 1.0) + (a - 1.0) * c + k,
            -2.0 * ((a - 1.0) + (a + 1.0) * c),
            (a + 1.0) + (a - 1.0) * c - k,
        )
    }
    pub fn high_shelf(sample_rate: f32, hz: f32, q: f32, gain_db: f32) -> Self {
        let (s, c) = Self::omega(sample_rate, hz);
        let a = 10f64.powf(gain_db as f64 / 40.0);
        let alpha = s / (2.0 * q.max(1e-3) as f64);
        let k = 2.0 * a.sqrt() * alpha;
        Self::normalize(
            a * ((a + 1.0) + (a - 1.0) * c + k),
            -2.0 * a * ((a - 1.0) + (a + 1.0) * c),
            a * ((a + 1.0) + (a - 1.0) * c - k),
            (a + 1.0) - (a - 1.0) * c + k,
            2.0 * ((a - 1.0) - (a + 1.0) * c),
            (a + 1.0) - (a - 1.0) * c - k,
        )
    }
    /// First-order low-pass (bilinear) in biquad form.
    pub fn lowpass1(sample_rate: f32, hz: f32) -> Self {
        let k = (PI * hz.clamp(1.0, sample_rate * 0.4999) / sample_rate).tan() as f64;
        Self::normalize(k, k, 0.0, k + 1.0, k - 1.0, 0.0)
    }
    /// First-order high-pass (bilinear) in biquad form.
    pub fn highpass1(sample_rate: f32, hz: f32) -> Self {
        let k = (PI * hz.clamp(1.0, sample_rate * 0.4999) / sample_rate).tan() as f64;
        Self::normalize(1.0, -1.0, 0.0, k + 1.0, k - 1.0, 0.0)
    }
    /// First-order all-pass with its 90° point at `hz`.
    pub fn allpass1(sample_rate: f32, hz: f32) -> Self {
        let k = (PI * hz.clamp(1.0, sample_rate * 0.4999) / sample_rate).tan() as f64;
        let a = (k - 1.0) / (k + 1.0);
        Self::normalize(a, 1.0, 0.0, 1.0, a, 0.0)
    }

    /// Re-derive a filter designed for `from_rate` (e.g. a 44.1 kHz ROM table)
    /// at `to_rate`, keeping its analog prototype: the bilinear transform is
    /// inverted with the prewarp of `from_rate` and applied again at `to_rate`.
    /// Use it for ROM coefficient sets whose analog shape is not obvious.
    pub fn resample(self, from_rate: f32, to_rate: f32) -> Self {
        if (from_rate - to_rate).abs() < 0.5 {
            return self;
        }
        let r = to_rate as f64 / from_rate as f64;
        if self.b2 == 0.0 && self.a2 == 0.0 {
            // First order stays first order: (1 + u) instead of (1 + u)^2.
            let s = |c0: f64, c1: f64| [c0 + c1, (c0 - c1) * r];
            let (b, a) = (s(self.b0 as f64, self.b1 as f64), s(1.0, self.a1 as f64));
            let z = |p: [f64; 2]| [p[0] + p[1], p[0] - p[1]];
            let (b, a) = (z(b), z(a));
            return Self::normalize(b[0], b[1], 0.0, a[0], a[1], 0.0);
        }
        // z = (1 + s/k)/(1 - s/k), k = 2 fs. Build H(s) numerator/denominator
        // from H(z), then map back with the new k. Coefficients in z^-1.
        let (b, a) = (
            [self.b0 as f64, self.b1 as f64, self.b2 as f64],
            [1.0, self.a1 as f64, self.a2 as f64],
        );
        // H(z) with z^-1 = (1 - s/k1)/(1 + s/k1): multiply num/den by (1+s/k1)^2.
        let to_s = |c: [f64; 3]| -> [f64; 3] {
            // c0 (1+u)^2 + c1 (1-u)(1+u) + c2 (1-u)^2 in powers of u = s/k1
            [
                c[0] + c[1] + c[2],
                2.0 * c[0] - 2.0 * c[2],
                c[0] - c[1] + c[2],
            ]
        };
        let (bs, as_) = (to_s(b), to_s(a));
        // Polynomials in u1 = s/k1; with k1 = 2 from_rate and k2 = 2 to_rate,
        // u1 = u2 * (k2/k1).
        let scale = |p: [f64; 3]| [p[0], p[1] * r, p[2] * r * r];
        let (bs, as_) = (scale(bs), scale(as_));
        // Back to z: u2 = (1 - z^-1)/(1 + z^-1); multiply by (1 + z^-1)^2.
        let to_z = |p: [f64; 3]| -> [f64; 3] {
            [
                p[0] + p[1] + p[2],
                2.0 * p[0] - 2.0 * p[2],
                p[0] - p[1] + p[2],
            ]
        };
        let (bz, az) = (to_z(bs), to_z(as_));
        Self::normalize(bz[0], bz[1], bz[2], az[0], az[1], az[2])
    }

    /// Magnitude response at `hz` (for tests and UI).
    pub fn magnitude(&self, sample_rate: f32, hz: f32) -> f32 {
        let w = std::f64::consts::TAU * hz as f64 / sample_rate as f64;
        let (c1, s1, c2, s2) = (w.cos(), w.sin(), (2.0 * w).cos(), (2.0 * w).sin());
        let nr = self.b0 as f64 + self.b1 as f64 * c1 + self.b2 as f64 * c2;
        let ni = -(self.b1 as f64 * s1 + self.b2 as f64 * s2);
        let dr = 1.0 + self.a1 as f64 * c1 + self.a2 as f64 * c2;
        let di = -(self.a1 as f64 * s1 + self.a2 as f64 * s2);
        ((nr * nr + ni * ni) / (dr * dr + di * di)).sqrt() as f32
    }
}

/// Transposed direct form II biquad.
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
    #[inline]
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

/// Ring buffer with a power-of-two length. `read(d)` returns the sample written
/// `d` samples before the most recent `write` (d = 0 is the latest sample).
#[derive(Debug, Clone)]
pub struct DelayLine {
    buffer: Vec<f32>,
    mask: usize,
    write: usize,
}
impl DelayLine {
    /// Holds at least `max_delay` samples of history plus interpolation room.
    pub fn new(max_delay: usize) -> Self {
        let length = (max_delay + 4).next_power_of_two();
        Self {
            buffer: vec![0.0; length],
            mask: length - 1,
            write: 0,
        }
    }
    /// For a maximum delay in seconds at `sample_rate`.
    pub fn with_seconds(seconds: f32, sample_rate: f32) -> Self {
        Self::new((seconds * sample_rate).ceil() as usize + 1)
    }
    pub fn capacity(&self) -> usize {
        self.mask - 3
    }
    #[inline]
    pub fn write(&mut self, x: f32) {
        self.write = (self.write + 1) & self.mask;
        self.buffer[self.write] = x;
    }
    #[inline]
    pub fn read(&self, delay: usize) -> f32 {
        self.buffer[self.write.wrapping_sub(delay) & self.mask]
    }
    /// Linear interpolation, `delay` clamped to `[0, capacity]`.
    #[inline]
    pub fn read_linear(&self, delay: f32) -> f32 {
        let delay = delay.clamp(0.0, self.capacity() as f32);
        let whole = delay as usize;
        let fraction = delay - whole as f32;
        let a = self.read(whole);
        let b = self.read(whole + 1);
        a + (b - a) * fraction
    }
    /// 4-point Hermite interpolation, `delay` clamped to `[1, capacity - 2]`.
    #[inline]
    pub fn read_cubic(&self, delay: f32) -> f32 {
        let delay = delay.clamp(1.0, self.capacity() as f32 - 2.0);
        let whole = delay as usize;
        let t = delay - whole as f32;
        let xm1 = self.read(whole - 1);
        let x0 = self.read(whole);
        let x1 = self.read(whole + 1);
        let x2 = self.read(whole + 2);
        let c1 = 0.5 * (x1 - xm1);
        let c2 = xm1 - 2.5 * x0 + 2.0 * x1 - 0.5 * x2;
        let c3 = 0.5 * (x2 - xm1) + 1.5 * (x0 - x1);
        ((c3 * t + c2) * t + c1) * t + x0
    }
    pub fn clear(&mut self) {
        self.buffer.fill(0.0);
    }
}

/// Schroeder all-pass diffuser on its own delay line.
#[derive(Debug, Clone)]
pub struct Allpass {
    line: DelayLine,
    pub delay: usize,
    pub gain: f32,
}
impl Allpass {
    pub fn new(delay: usize, gain: f32) -> Self {
        Self {
            line: DelayLine::new(delay.max(1)),
            delay: delay.max(1),
            gain,
        }
    }
    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let delayed = self.line.read(self.delay - 1);
        let v = flush(x + self.gain * delayed);
        self.line.write(v);
        delayed - self.gain * v
    }
    pub fn clear(&mut self) {
        self.line.clear();
    }
}

/// Phase accumulator in turns, for LFOs.
#[derive(Debug, Clone, Copy, Default)]
pub struct Phasor {
    pub phase: f32,
    pub increment: f32,
}
impl Phasor {
    pub fn set_hz(&mut self, hz: f32, sample_rate: f32) {
        self.increment = hz / sample_rate;
    }
    #[inline]
    pub fn tick(&mut self) -> f32 {
        let phase = self.phase;
        self.phase += self.increment;
        if self.phase >= 1.0 {
            self.phase -= self.phase.floor();
        }
        phase
    }
    #[inline]
    pub fn sine(&mut self) -> f32 {
        sin_turns(self.tick())
    }
    /// Triangle in [-1, 1], 0 at phase 0 rising.
    #[inline]
    pub fn triangle(&mut self) -> f32 {
        let p = self.tick();
        let t = 4.0 * p;
        if p < 0.25 {
            t
        } else if p < 0.75 {
            2.0 - t
        } else {
            t - 4.0
        }
    }
}

/// Peak envelope follower with separate attack and release times.
#[derive(Debug, Clone, Copy)]
pub struct Envelope {
    pub value: f32,
    attack: f32,
    release: f32,
}
impl Envelope {
    pub fn new(attack_seconds: f32, release_seconds: f32, sample_rate: f32) -> Self {
        Self {
            value: 0.0,
            attack: one_pole_coefficient(attack_seconds, sample_rate),
            release: one_pole_coefficient(release_seconds, sample_rate),
        }
    }
    pub fn set_times(&mut self, attack_seconds: f32, release_seconds: f32, sample_rate: f32) {
        self.attack = one_pole_coefficient(attack_seconds, sample_rate);
        self.release = one_pole_coefficient(release_seconds, sample_rate);
    }
    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let x = x.abs();
        let coefficient = if x > self.value {
            self.attack
        } else {
            self.release
        };
        self.value = flush(self.value + (x - self.value) * coefficient);
        self.value
    }
    pub fn clear(&mut self) {
        self.value = 0.0;
    }
}

/// Equal-power crossfade gains for a mix in [0, 1]: (dry, wet).
#[inline]
pub fn equal_power(mix: f32) -> (f32, f32) {
    let mix = mix.clamp(0.0, 1.0);
    (cos_turns(mix * 0.25), sin_turns(mix * 0.25))
}

/// Scale a sample count from the original 44.1 kHz rate.
#[inline]
pub fn scale_samples(samples_at_44k: f32, sample_rate: f32) -> f32 {
    samples_at_44k * sample_rate / crate::zoom::ORIGINAL_RATE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sine_approximation_is_close() {
        for i in 0..1000 {
            let p = i as f32 / 1000.0 * 3.0 - 1.0;
            assert!((sin_turns(p) - (p * TAU).sin()).abs() < 2e-3, "{p}");
        }
    }

    #[test]
    fn resampled_filters_keep_their_corner() {
        let at44 = Coefficients::lowpass(44_100.0, 1000.0, 0.707);
        let at48 = at44.resample(44_100.0, 48_000.0);
        let direct = Coefficients::lowpass(48_000.0, 1000.0, 0.707);
        for hz in [100.0, 1000.0, 5000.0] {
            let a = at48.magnitude(48_000.0, hz);
            let b = direct.magnitude(48_000.0, hz);
            assert!((gain_to_db(a) - gain_to_db(b)).abs() < 0.2, "{hz}: {a} {b}");
        }
    }

    #[test]
    fn delay_line_reads_back() {
        let mut line = DelayLine::new(10);
        for i in 0..20 {
            line.write(i as f32);
        }
        assert_eq!(line.read(0), 19.0);
        assert_eq!(line.read(5), 14.0);
        assert!((line.read_linear(2.5) - 16.5).abs() < 1e-6);
        assert!((line.read_cubic(4.0) - 15.0).abs() < 1e-6);
    }
}
