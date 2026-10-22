//! Source-filter synthesis in the manner of WORLD: every glottal pulse is a
//! minimum-phase impulse response of the spectral envelope, split into a
//! periodic part and a noise part by the aperiodicity. Unvoiced frames are
//! noise grains at a fixed rate. The envelope is a power spectral density, so
//! the output level does not depend on the pitch or the sample rate: a frame
//! sung an octave higher has twice the pulses at half the energy each.
use crate::spectrum::{
    AP_BANDS, AP_EDGES, ENV_MAX_HZ, ENV_POINTS, FRAME_SECONDS, Frame, Rng, Transform,
    grid_position, sample_grid,
};
use rustfft::num_complex::Complex32;

/// Noise grains per second where the voice is unvoiced.
const UNVOICED_RATE: f32 = 400.0;

/// Per-frame performance controls that act on single pulses.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Pulse {
    /// 0–1: alternating pulse level and spacing, a rough subharmonic growl.
    pub growl: f32,
    /// 0–1: random period and level variation of a natural voice.
    pub jitter: f32,
}

pub struct Vocoder {
    rate: f32,
    fft: Transform,
    /// Envelope grid position of every bin up to Nyquist; NaN above the grid.
    env_position: Vec<f32>,
    /// Aperiodicity interpolation per bin: lower band index and weight.
    ap_weight: Vec<(u8, f32)>,
    log_amplitude: Vec<f32>,
    spectrum: Vec<Complex32>,
    noise: Vec<Complex32>,
    db_to_ln_amplitude: f32,
}

impl Vocoder {
    pub fn new(rate: u32) -> Self {
        let n = if rate > 50_000 { 4096 } else { 2048 };
        let rate_f = rate as f32;
        let half = n / 2;
        let env_position = (0..=half)
            .map(|k| {
                let hz = k as f32 * rate_f / n as f32;
                if hz <= ENV_MAX_HZ {
                    grid_position(hz)
                } else {
                    f32::NAN
                }
            })
            .collect();
        let center = |b: usize| (AP_EDGES[b] + AP_EDGES[b + 1]) * 0.5;
        let ap_weight = (0..=half)
            .map(|k| {
                let hz = k as f32 * rate_f / n as f32;
                if hz <= center(0) {
                    return (0, 0.0);
                }
                for b in 0..AP_BANDS - 1 {
                    if hz <= center(b + 1) {
                        return (b as u8, (hz - center(b)) / (center(b + 1) - center(b)));
                    }
                }
                ((AP_BANDS - 2) as u8, 1.0)
            })
            .collect();
        Self {
            rate: rate_f,
            fft: Transform::new(n),
            env_position,
            ap_weight,
            log_amplitude: vec![0.0; half + 1],
            spectrum: vec![Complex32::default(); n],
            noise: vec![Complex32::default(); n],
            // dB per Hz → natural-log amplitude per sample: ln(sqrt(P·rate)).
            db_to_ln_amplitude: std::f32::consts::LN_10 / 20.0,
        }
    }

    pub fn rate(&self) -> u32 {
        self.rate as u32
    }

    /// Renders frames spaced `FRAME_SECONDS` apart. `pulses` (same length as
    /// `frames`, or empty) adds growl and jitter. The result has exactly
    /// `frames.len()` frames of samples; tails past the end are dropped.
    pub fn render(&mut self, frames: &[Frame], pulses: &[Pulse], seed: u64) -> Vec<f32> {
        let hop = FRAME_SECONDS as f64 * self.rate as f64;
        let len = (frames.len() as f64 * hop).round() as usize;
        let mut out = vec![0.0f32; len + self.fft.n];
        let mut rng = Rng::new(seed);
        let mut t = 0.0f64;
        let mut count = 0u32;
        while (t as usize) < len {
            let position = t / hop;
            let frame = interpolate(frames, position);
            let pulse = interpolate_pulse(pulses, position);
            let voiced = frame.voiced();
            let mut period = if voiced {
                self.rate / frame.f0
            } else {
                self.rate / UNVOICED_RATE
            };
            let mut amplitude = 1.0;
            if voiced {
                let odd = if count % 2 == 1 { 1.0 } else { -1.0 };
                period *= 1.0 + pulse.growl * 0.1 * odd;
                amplitude *= 1.0 - pulse.growl * 0.4 * (0.5 + 0.5 * odd);
                // Jitter and shimmer only as asked: a recording's own
                // irregularity is already in its aperiodicity.
                period *= 1.0 + pulse.jitter * 0.012 * rng.uniform();
                amplitude *= 1.0 + pulse.jitter * 0.12 * rng.uniform();
            }
            let period = period.max(2.0);
            self.pulse(&frame, t, period, amplitude, voiced, &mut rng, &mut out);
            t += period as f64;
            count = count.wrapping_add(1);
        }
        out.truncate(len);
        dc_block(&mut out, self.rate);
        out
    }

    #[allow(clippy::too_many_arguments)]
    fn pulse(
        &mut self,
        frame: &Frame,
        t: f64,
        period: f32,
        amplitude: f32,
        voiced: bool,
        rng: &mut Rng,
        out: &mut [f32],
    ) {
        let n = self.fft.n;
        let half = n / 2;
        let rate_db = 10.0 * self.rate.log10();
        let top = frame.env[ENV_POINTS - 1];
        for (k, value) in self.log_amplitude.iter_mut().enumerate() {
            let position = self.env_position[k];
            let db = if position.is_nan() {
                top - 60.0
            } else {
                sample_grid(&frame.env, position)
            };
            *value = (db + rate_db) * self.db_to_ln_amplitude;
        }
        self.fft
            .minimum_phase(&self.log_amplitude, &mut self.spectrum);

        // White noise for the length of this pulse, unit variance.
        let length = (period.round() as usize).clamp(1, n);
        for (i, value) in self.noise.iter_mut().enumerate() {
            *value = Complex32::new(if i < length { rng.normal() } else { 0.0 }, 0.0);
        }
        self.fft.forward(&mut self.noise);

        let start = t.floor();
        let fraction = (t - start) as f32;
        let periodic_gain = if voiced {
            (period).sqrt() * amplitude
        } else {
            0.0
        };
        let omega = std::f32::consts::TAU / n as f32;
        for k in 0..=half {
            let ap = if voiced { self.ap(frame, k) } else { 1.0 };
            let delay = -omega * k as f32 * fraction;
            let periodic =
                Complex32::new(delay.cos(), delay.sin()) * (periodic_gain * (1.0 - ap).sqrt());
            let noise = self.noise[k] * ap.sqrt();
            self.spectrum[k] *= periodic + noise;
        }
        self.spectrum[half].im = 0.0;
        self.spectrum[0].im = 0.0;
        for k in half + 1..n {
            self.spectrum[k] = self.spectrum[n - k].conj();
        }
        self.fft.inverse(&mut self.spectrum);
        let scale = 1.0 / n as f32;
        let start = start as usize;
        if let Some(target) = out.get_mut(start..start + n) {
            for (o, s) in target.iter_mut().zip(&self.spectrum) {
                *o += s.re * scale;
            }
        } else if start < out.len() {
            for (o, s) in out[start..].iter_mut().zip(&self.spectrum) {
                *o += s.re * scale;
            }
        }
    }

    #[inline]
    fn ap(&self, frame: &Frame, k: usize) -> f32 {
        let (band, weight) = self.ap_weight[k];
        let band = band as usize;
        let a = frame.ap[band].clamp(1e-5, 1.0);
        let b = frame.ap[(band + 1).min(AP_BANDS - 1)].clamp(1e-5, 1.0);
        if weight <= 0.0 {
            a
        } else {
            (a.ln() + (b.ln() - a.ln()) * weight).exp()
        }
    }
}

/// Frame at a fractional index; voicing changes take the nearer frame.
pub fn interpolate(frames: &[Frame], position: f64) -> Frame {
    if frames.is_empty() {
        return Frame::SILENT;
    }
    let i = (position.floor() as usize).min(frames.len() - 1);
    let j = (i + 1).min(frames.len() - 1);
    let t = (position - i as f64).clamp(0.0, 1.0) as f32;
    let (a, b) = (&frames[i], &frames[j]);
    if a.voiced() != b.voiced() {
        // Keep the spectrum smooth but switch voicing at the midpoint.
        let mut mixed = a.mix(b, t);
        mixed.f0 = if t < 0.5 { a.f0 } else { b.f0 };
        return mixed;
    }
    a.mix(b, t)
}

fn interpolate_pulse(pulses: &[Pulse], position: f64) -> Pulse {
    if pulses.is_empty() {
        return Pulse::default();
    }
    let i = (position.floor() as usize).min(pulses.len() - 1);
    let j = (i + 1).min(pulses.len() - 1);
    let t = (position - i as f64).clamp(0.0, 1.0) as f32;
    Pulse {
        growl: pulses[i].growl + (pulses[j].growl - pulses[i].growl) * t,
        jitter: pulses[i].jitter + (pulses[j].jitter - pulses[i].jitter) * t,
    }
}

/// One-pole high-pass at 20 Hz: pulses with energy at DC leave an offset.
pub fn dc_block(samples: &mut [f32], rate: f32) {
    let r = 1.0 - std::f32::consts::TAU * 20.0 / rate;
    let (mut x1, mut y1) = (0.0f32, 0.0f32);
    for s in samples {
        let y = *s - x1 + r * y1;
        x1 = *s;
        y1 = y;
        *s = y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flat(f0: f32, db: f32, ap: f32) -> Frame {
        Frame {
            f0,
            env: [db; ENV_POINTS],
            ap: [ap; AP_BANDS],
        }
    }
    fn power_db(x: &[f32]) -> f32 {
        let p: f64 = x.iter().map(|&s| (s as f64).powi(2)).sum::<f64>() / x.len() as f64;
        10.0 * p.log10() as f32
    }

    #[test]
    fn level_matches_the_envelope_for_noise_and_pulses() {
        for rate in [44_100u32, 48_000, 96_000] {
            let mut v = Vocoder::new(rate);
            // A flat PSD of -100 dB/Hz over 0..rate/2 has power -100 + 10·log10(rate/2)
            // ... in one-sided terms; the envelope is two-sided per Hz, so the
            // power is -100 + 10·log10(rate) with the band edge correction.
            let expect = -100.0 + 10.0 * (rate.min(48_000) as f32).log10();
            for (f0, ap) in [(0.0, 1.0), (220.0, 0.0), (110.0, 0.0), (440.0, 0.5)] {
                let frames = vec![flat(f0, -100.0, ap); 400];
                let out = v.render(&frames, &[], 1);
                let got = power_db(&out[out.len() / 4..]);
                // Above 24 kHz the grid rolls off, so 96 kHz keeps the 48 kHz power.
                assert!(
                    (got - expect).abs() < 1.0,
                    "rate {rate} f0 {f0} ap {ap}: {got} vs {expect}"
                );
            }
        }
    }

    #[test]
    fn pitch_is_the_frame_f0() {
        let rate = 48_000;
        let mut v = Vocoder::new(rate);
        let frames = vec![flat(200.0, -100.0, 0.0); 200];
        let out = v.render(&frames, &[], 3);
        // Autocorrelation peak at the period.
        let x = &out[4000..];
        let r = |lag: usize| -> f32 {
            x[..x.len() - 400]
                .iter()
                .zip(&x[lag..])
                .map(|(a, b)| a * b)
                .sum()
        };
        let best = (150..400).max_by(|&a, &b| r(a).total_cmp(&r(b))).unwrap();
        assert!((best as i32 - 240).abs() <= 1, "{best}");
    }
}
