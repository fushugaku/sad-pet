//! The spectral representation shared by analysis, voicebanks and the
//! vocoder. A frame is the voice's power spectral envelope in dB per Hz on a
//! mel-spaced grid that does not depend on any sample rate, its aperiodicity
//! in eight bands, and F0. Envelopes from different voicebanks and the
//! built-in formant voice can therefore be mixed, warped and stored alike.
use rustfft::{Fft, FftPlanner, num_complex::Complex32};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Envelope grid points, mel-spaced from 0 Hz to `ENV_MAX_HZ`.
pub const ENV_POINTS: usize = 96;
pub const ENV_MAX_HZ: f32 = 24_000.0;
/// Aperiodicity band edges in Hz.
pub const AP_EDGES: [f32; 9] = [0., 500., 1000., 2000., 3000., 4500., 6500., 9500., 24000.];
pub const AP_BANDS: usize = AP_EDGES.len() - 1;
/// Analysis and synthesis frames are this far apart.
pub const FRAME_SECONDS: f32 = 0.005;
/// Below every real envelope: silence.
pub const FLOOR_DB: f32 = -210.0;
/// The vowel level (`Frame::level`) every voice sings at by default: about
/// -18 dBFS RMS, leaving room for loud Dynamics.
pub const SINGING_LEVEL: f32 = -54.5;

pub fn mel(hz: f32) -> f32 {
    1127.0 * (1.0 + hz / 700.0).ln()
}
pub fn mel_to_hz(mel: f32) -> f32 {
    700.0 * ((mel / 1127.0).exp() - 1.0)
}

/// Grid point `i` in Hz.
pub fn grid_hz(i: usize) -> f32 {
    mel_to_hz(i as f32 / (ENV_POINTS - 1) as f32 * mel(ENV_MAX_HZ))
}

/// Fractional grid position of a frequency.
pub fn grid_position(hz: f32) -> f32 {
    (mel(hz.clamp(0.0, ENV_MAX_HZ)) / mel(ENV_MAX_HZ) * (ENV_POINTS - 1) as f32)
        .clamp(0.0, (ENV_POINTS - 1) as f32)
}

/// Mean of an envelope over 150 Hz – 6 kHz, where a vowel's color lies;
/// subtracting it leaves the envelope's shape.
pub fn color_level(env: &[f32]) -> f32 {
    let lo = grid_position(150.0) as usize;
    let hi = grid_position(6000.0) as usize;
    env[lo..=hi].iter().sum::<f32>() / (hi - lo + 1) as f32
}

pub type Envelope = [f32; ENV_POINTS];
pub type Aperiodicity = [f32; AP_BANDS];

/// One 5 ms frame of a voice.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Frame {
    /// Hz; 0 when unvoiced.
    pub f0: f32,
    /// Power spectral density, dB per Hz, on the mel grid.
    #[serde(with = "serde_big_array")]
    pub env: Envelope,
    /// Share of the power that is noise, 0–1, per band.
    pub ap: Aperiodicity,
}
impl Default for Frame {
    fn default() -> Self {
        Self::SILENT
    }
}
impl Frame {
    pub const SILENT: Self = Self {
        f0: 0.0,
        env: [FLOOR_DB; ENV_POINTS],
        ap: [1.0; AP_BANDS],
    };
    pub fn voiced(&self) -> bool {
        self.f0 > 0.0
    }
    /// Envelope value at any frequency, interpolated on the grid.
    pub fn env_at(&self, hz: f32) -> f32 {
        sample_grid(&self.env, grid_position(hz))
    }
    /// Linear blend of two frames; voicing follows the heavier side.
    pub fn mix(&self, other: &Self, t: f32) -> Self {
        let mut out = *self;
        for (o, (a, b)) in out.env.iter_mut().zip(self.env.iter().zip(&other.env)) {
            *o = a + (b - a) * t;
        }
        for (o, (a, b)) in out.ap.iter_mut().zip(self.ap.iter().zip(&other.ap)) {
            *o = a + (b - a) * t;
        }
        out.f0 = match (self.voiced(), other.voiced()) {
            (true, true) => (self.f0.ln() + (other.f0.ln() - self.f0.ln()) * t).exp(),
            _ if t < 0.5 => self.f0,
            _ => other.f0,
        };
        out
    }
    /// Loudness proxy: mean envelope level in dB over 100 Hz – 5 kHz.
    pub fn level(&self) -> f32 {
        let lo = grid_position(100.0) as usize;
        let hi = grid_position(5000.0).ceil() as usize;
        let slice = &self.env[lo..=hi];
        // Power mean, so a strong formant counts more than a deep valley.
        let mean: f32 =
            slice.iter().map(|d| 10f32.powf(d / 10.0)).sum::<f32>() / slice.len() as f32;
        10.0 * mean.max(1e-30).log10()
    }
}

pub fn sample_grid(env: &Envelope, position: f32) -> f32 {
    let i = (position as usize).min(ENV_POINTS - 2);
    let t = position - i as f32;
    env[i] + (env[i + 1] - env[i]) * t
}

/// Aperiodicity at any frequency: dB-linear between band centers.
pub fn ap_at(ap: &Aperiodicity, hz: f32) -> f32 {
    let center = |b: usize| (AP_EDGES[b] + AP_EDGES[b + 1]) * 0.5;
    if hz <= center(0) {
        return ap[0];
    }
    for b in 0..AP_BANDS - 1 {
        let (c0, c1) = (center(b), center(b + 1));
        if hz <= c1 {
            let t = (hz - c0) / (c1 - c0);
            let (a, b) = (ap[b].max(1e-6).ln(), ap[b + 1].max(1e-6).ln());
            return (a + (b - a) * t).exp();
        }
    }
    ap[AP_BANDS - 1]
}

/// serde has no impl for arrays longer than 32.
mod serde_big_array {
    use super::{ENV_POINTS, Envelope};
    use serde::{Deserialize, Deserializer, Serializer, ser::SerializeTuple};
    pub fn serialize<S: Serializer>(v: &Envelope, s: S) -> Result<S::Ok, S::Error> {
        let mut t = s.serialize_tuple(ENV_POINTS)?;
        for x in v {
            t.serialize_element(x)?;
        }
        t.end()
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Envelope, D::Error> {
        let v = Vec::<f32>::deserialize(d)?;
        v.try_into()
            .map_err(|_| serde::de::Error::custom("envelope length"))
    }
}

/// A forward/inverse complex FFT pair of one size with its scratch space.
pub struct Transform {
    pub n: usize,
    forward: Arc<dyn Fft<f32>>,
    inverse: Arc<dyn Fft<f32>>,
    scratch: Vec<Complex32>,
}
impl Transform {
    pub fn new(n: usize) -> Self {
        let mut planner = FftPlanner::new();
        let forward = planner.plan_fft_forward(n);
        let inverse = planner.plan_fft_inverse(n);
        let len = forward
            .get_inplace_scratch_len()
            .max(inverse.get_inplace_scratch_len());
        Self {
            n,
            forward,
            inverse,
            scratch: vec![Complex32::default(); len],
        }
    }
    pub fn forward(&mut self, data: &mut [Complex32]) {
        self.forward.process_with_scratch(data, &mut self.scratch);
    }
    /// Unnormalized, as rustfft is: divide by `n` for a true inverse.
    pub fn inverse(&mut self, data: &mut [Complex32]) {
        self.inverse.process_with_scratch(data, &mut self.scratch);
    }
    /// Turns a log-amplitude spectrum (bins 0..=n/2, natural log) into the
    /// minimum-phase complex spectrum with that magnitude, in `out`.
    pub fn minimum_phase(&mut self, log_amplitude: &[f32], out: &mut [Complex32]) {
        let n = self.n;
        let half = n / 2;
        for k in 0..=half {
            out[k] = Complex32::new(log_amplitude[k], 0.0);
        }
        for k in half + 1..n {
            out[k] = out[n - k];
        }
        self.inverse(out);
        let scale = 1.0 / n as f32;
        // Fold the real cepstrum onto positive quefrencies.
        out[0] = Complex32::new(out[0].re * scale, 0.0);
        for value in &mut out[1..half] {
            *value = Complex32::new(value.re * 2.0 * scale, 0.0);
        }
        out[half] = Complex32::new(out[half].re * scale, 0.0);
        for value in &mut out[half + 1..] {
            *value = Complex32::default();
        }
        self.forward(out);
        for value in out.iter_mut() {
            let magnitude = value.re.exp();
            *value = Complex32::new(magnitude * value.im.cos(), magnitude * value.im.sin());
        }
    }
}

/// xorshift64*: fast, seedable and identical on every platform.
#[derive(Clone, Debug)]
pub struct Rng(u64);
impl Rng {
    pub fn new(seed: u64) -> Self {
        Self(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1)
    }
    pub fn next_u32(&mut self) -> u32 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        (self.0.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 32) as u32
    }
    /// Uniform in [-1, 1).
    pub fn uniform(&mut self) -> f32 {
        self.next_u32() as f32 * (2.0 / 4_294_967_296.0) - 1.0
    }
    /// Approximately normal with unit variance (sum of four uniforms).
    pub fn normal(&mut self) -> f32 {
        // Each uniform in [-1,1) has variance 1/3.
        (self.uniform() + self.uniform() + self.uniform() + self.uniform()) * 0.866_025_4
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn grid_is_monotonic_and_invertible() {
        assert_eq!(grid_hz(0), 0.0);
        assert!((grid_hz(ENV_POINTS - 1) - ENV_MAX_HZ).abs() < 1.0);
        for i in 1..ENV_POINTS {
            assert!(grid_hz(i) > grid_hz(i - 1));
            assert!((grid_position(grid_hz(i)) - i as f32).abs() < 1e-3);
        }
        // About 30 Hz resolution in the first formant region.
        assert!(grid_hz(10) < 400.0);
    }
    #[test]
    fn minimum_phase_keeps_magnitude() {
        let n = 512;
        let mut t = Transform::new(n);
        let log: Vec<f32> = (0..=n / 2)
            .map(|k| (-((k as f32 - 40.0) / 20.0).powi(2)).exp() * 3.0 - 2.0)
            .collect();
        let mut out = vec![Complex32::default(); n];
        t.minimum_phase(&log, &mut out);
        for k in 0..=n / 2 {
            assert!((out[k].norm().ln() - log[k]).abs() < 1e-3, "bin {k}");
        }
        // Causal: the impulse response is concentrated at the start.
        t.inverse(&mut out);
        let energy: f32 = out.iter().map(|c| c.re * c.re).sum();
        let early: f32 = out[..n / 4].iter().map(|c| c.re * c.re).sum();
        assert!(early / energy > 0.99);
    }
    #[test]
    fn noise_has_unit_variance() {
        let mut rng = Rng::new(7);
        let n = 100_000;
        let (mut sum, mut sq) = (0.0f64, 0.0f64);
        for _ in 0..n {
            let x = rng.normal() as f64;
            sum += x;
            sq += x * x;
        }
        let mean = sum / n as f64;
        let var = sq / n as f64 - mean * mean;
        assert!(
            mean.abs() < 0.02 && (var - 1.0).abs() < 0.03,
            "{mean} {var}"
        );
    }
}
