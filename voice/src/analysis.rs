//! Turns a recording into frames: F0 by YIN with a Viterbi path over its
//! candidates, the spectral envelope by CheapTrick (Morise 2015: an
//! F0-adaptive window, smoothing across one harmonic and cepstral liftering)
//! and aperiodicity from how well each band repeats one period later.
use crate::spectrum::{
    AP_BANDS, AP_EDGES, ENV_MAX_HZ, ENV_POINTS, FLOOR_DB, FRAME_SECONDS, Frame, Transform, grid_hz,
};
use rustfft::num_complex::Complex32;

#[derive(Clone, Copy, Debug)]
pub struct Options {
    pub f0_min: f32,
    pub f0_max: f32,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            f0_min: 70.0,
            f0_max: 1100.0,
        }
    }
}

/// Analyzes mono samples into one frame per 5 ms.
pub fn analyze(samples: &[f32], rate: u32, options: &Options) -> Vec<Frame> {
    let mut x = samples.to_vec();
    crate::vocoder::dc_block(&mut x, rate as f32);
    let f0 = track_f0(&x, rate, options);
    let mut envelope = Envelope::new(rate);
    let mut aperiodicity = Periodicity::new(rate);
    let hop = FRAME_SECONDS as f64 * rate as f64;
    let mut frames: Vec<Frame> = f0
        .iter()
        .enumerate()
        .map(|(i, &f0)| {
            let center = i as f64 * hop;
            // YIN's interpolated period is the more precise F0; the
            // periodicity search only aligns the bands it compares.
            let ap = if f0 > 0.0 {
                aperiodicity.measure(&x, center, f0).1
            } else {
                [1.0; AP_BANDS]
            };
            let env = envelope.measure(&x, center, if f0 > 0.0 { f0 } else { 500.0 });
            Frame { f0, env, ap }
        })
        .collect();
    smooth_aperiodicity(&mut frames);
    frames
}

/// A 25 ms moving average (in the log domain) within each voiced run: one
/// period-to-period comparison is a noisy estimate.
fn smooth_aperiodicity(frames: &mut [Frame]) {
    let logs: Vec<[f32; AP_BANDS]> = frames
        .iter()
        .map(|f| f.ap.map(|a| a.max(1e-4).ln()))
        .collect();
    for i in 0..frames.len() {
        if !frames[i].voiced() {
            continue;
        }
        let mut sum = [0.0f32; AP_BANDS];
        let mut count = 0.0;
        for j in i.saturating_sub(2)..(i + 3).min(frames.len()) {
            if frames[j].voiced() {
                for b in 0..AP_BANDS {
                    sum[b] += logs[j][b];
                }
                count += 1.0;
            }
        }
        frames[i].ap = sum.map(|s| (s / count).exp().clamp(0.001, 1.0));
    }
}

fn frame_count(len: usize, rate: u32) -> usize {
    (len as f64 / (FRAME_SECONDS as f64 * rate as f64)).floor() as usize + 1
}

/// Samples with zeros outside the signal.
#[inline]
fn at(x: &[f32], i: isize) -> f32 {
    if i < 0 {
        0.0
    } else {
        x.get(i as usize).copied().unwrap_or(0.0)
    }
}

// --- F0 ------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
struct Candidate {
    hz: f32,
    cost: f32,
}

/// F0 per frame in Hz, 0 where unvoiced.
pub fn track_f0(x: &[f32], rate: u32, options: &Options) -> Vec<f32> {
    let frames = frame_count(x.len(), rate);
    let hop = FRAME_SECONDS as f64 * rate as f64;
    let tau_min = ((rate as f32 / options.f0_max).floor() as usize).max(2);
    let tau_max = (rate as f32 / options.f0_min).ceil() as usize;
    // At least 25 ms, so breathy and noisy frames still average out.
    let w = tau_max.max((0.025 * rate as f32) as usize);
    let n = (2 * w + tau_max).next_power_of_two();
    let mut fft = Transform::new(n);
    let mut a = vec![Complex32::default(); n];
    let mut b = vec![Complex32::default(); n];
    let mut squares = vec![0.0f64; w + tau_max + 1];
    let mut cmnd = vec![1.0f32; tau_max + 2];
    // Normalized autocorrelation: the periodic share of the power at a lag.
    let mut nacf = vec![0.0f32; tau_max + 2];

    // Loudness gate relative to the loudest frame.
    let level: Vec<f32> = (0..frames)
        .map(|i| {
            let c = (i as f64 * hop) as isize;
            let half = (w / 2) as isize;
            let e: f32 = (c - half..c + half).map(|j| at(x, j).powi(2)).sum();
            10.0 * (e / w as f32 + 1e-20).log10()
        })
        .collect();
    let loudest = level.iter().copied().fold(-200.0f32, f32::max);

    let mut candidates: Vec<Vec<Candidate>> = Vec::with_capacity(frames);
    for (i, &frame_level) in level.iter().enumerate() {
        let start = (i as f64 * hop) as isize - ((w + tau_max) / 2) as isize;
        for (j, v) in b.iter_mut().enumerate() {
            *v = Complex32::new(
                if j < w + tau_max {
                    at(x, start + j as isize)
                } else {
                    0.0
                },
                0.0,
            );
        }
        for (j, v) in a.iter_mut().enumerate() {
            *v = if j < w { b[j] } else { Complex32::default() };
        }
        squares[0] = 0.0;
        for j in 0..w + tau_max {
            squares[j + 1] = squares[j] + (b[j].re as f64).powi(2);
        }
        fft.forward(&mut a);
        fft.forward(&mut b);
        for (av, bv) in a.iter_mut().zip(&b) {
            *av = av.conj() * bv;
        }
        fft.inverse(&mut a);
        let scale = 1.0 / n as f64;
        let energy_a = squares[w];
        let mut running = 0.0f64;
        cmnd[0] = 1.0;
        for tau in 1..=tau_max {
            let energy_b = squares[tau + w] - squares[tau];
            let d = (energy_a + energy_b - 2.0 * a[tau].re as f64 * scale).max(0.0);
            running += d;
            cmnd[tau] = if running > 0.0 {
                (d * tau as f64 / running) as f32
            } else {
                1.0
            };
            let product = (energy_a * energy_b).sqrt();
            nacf[tau] = if product > 0.0 {
                ((energy_a + energy_b - d) * 0.5 / product) as f32
            } else {
                0.0
            };
        }
        let mut found: Vec<Candidate> = Vec::new();
        for tau in tau_min.max(2)..tau_max {
            let (y0, y1, y2) = (cmnd[tau - 1], cmnd[tau], cmnd[tau + 1]);
            if y1 < y0 && y1 <= y2 && y1 < 0.6 {
                let denominator = y0 - 2.0 * y1 + y2;
                let offset = if denominator.abs() > 1e-9 {
                    (0.5 * (y0 - y2) / denominator).clamp(-0.5, 0.5)
                } else {
                    0.0
                };
                // YIN's dips find the candidates; how periodic the signal is
                // at that lag decides how likely each one is.
                let periodic = nacf[tau].max(nacf[tau - 1]).max(nacf[tau + 1]);
                found.push(Candidate {
                    hz: rate as f32 / (tau as f32 + offset),
                    cost: (1.0 - periodic).clamp(0.0, 1.0),
                });
            }
        }
        // YIN prefers the shortest period that is clearly periodic; longer
        // multiples of it pay a little per octave. This happens before the
        // list is cut, so a high voice keeps its fundamental among its many
        // equally periodic multiples.
        let floor = found.iter().map(|c| c.cost).fold(f32::INFINITY, f32::min);
        if let Some(top) = found
            .iter()
            .filter(|c| c.cost < floor + 0.15)
            .map(|c| c.hz)
            .fold(None, |m: Option<f32>, h| Some(m.map_or(h, |m| m.max(h))))
        {
            for c in &mut found {
                c.cost += 0.1 * (top / c.hz).log2().max(0.0);
            }
        }
        found.sort_by(|p, q| p.cost.total_cmp(&q.cost));
        found.truncate(6);
        if frame_level < loudest - 50.0 {
            for c in &mut found {
                c.cost += 1.0;
            }
        }
        candidates.push(found);
    }
    viterbi(&candidates)
}

fn viterbi(candidates: &[Vec<Candidate>]) -> Vec<f32> {
    const UNVOICED: f32 = 0.45;
    const SWITCH: f32 = 0.25;
    const PER_OCTAVE: f32 = 2.5;
    let frames = candidates.len();
    // State 0 is unvoiced; state k is candidate k - 1.
    let mut cost: Vec<Vec<f32>> = Vec::with_capacity(frames);
    let mut back: Vec<Vec<usize>> = Vec::with_capacity(frames);
    for (i, found) in candidates.iter().enumerate() {
        let mut here = vec![UNVOICED];
        here.extend(found.iter().map(|c| c.cost));
        let mut from = vec![0usize; here.len()];
        if i > 0 {
            let previous = &candidates[i - 1];
            let prev_cost = &cost[i - 1];
            for s in 0..here.len() {
                let mut best = (f32::INFINITY, 0);
                for (p, &before) in prev_cost.iter().enumerate() {
                    let transition = match (p, s) {
                        (0, 0) => 0.0,
                        (0, _) | (_, 0) => SWITCH,
                        (p, s) => PER_OCTAVE * (found[s - 1].hz / previous[p - 1].hz).log2().abs(),
                    };
                    let total = before + transition;
                    if total < best.0 {
                        best = (total, p);
                    }
                }
                here[s] += best.0;
                from[s] = best.1;
            }
        }
        cost.push(here);
        back.push(from);
    }
    let mut out = vec![0.0; frames];
    let Some(last) = cost.last() else { return out };
    let mut state = (0..last.len())
        .min_by(|&a, &b| last[a].total_cmp(&last[b]))
        .unwrap_or(0);
    for i in (0..frames).rev() {
        out[i] = if state == 0 {
            0.0
        } else {
            candidates[i][state - 1].hz
        };
        state = back[i][state];
    }
    // Voiced islands shorter than 15 ms are noise.
    let mut i = 0;
    while i < frames {
        if out[i] > 0.0 {
            let start = i;
            while i < frames && out[i] > 0.0 {
                i += 1;
            }
            if i - start < 3 {
                out[start..i].fill(0.0);
            }
        } else {
            i += 1;
        }
    }
    out
}

// --- Envelope ----------------------------------------------------------------

struct Envelope {
    rate: f32,
    fft: Transform,
    buffer: Vec<Complex32>,
    power: Vec<f32>,
    smoothed: Vec<f32>,
    cumulative: Vec<f64>,
    log: Vec<f32>,
}
impl Envelope {
    fn new(rate: u32) -> Self {
        let n = ((3.0 * rate as f32 / 70.0) as usize + 1).next_power_of_two();
        Self {
            rate: rate as f32,
            fft: Transform::new(n),
            buffer: vec![Complex32::default(); n],
            power: vec![0.0; n / 2 + 1],
            smoothed: vec![0.0; n / 2 + 1],
            cumulative: vec![0.0; n + 2],
            log: vec![0.0; n / 2 + 1],
        }
    }

    /// CheapTrick at one instant; returns the envelope in dB per Hz.
    fn measure(&mut self, x: &[f32], center: f64, f0: f32) -> [f32; ENV_POINTS] {
        let n = self.fft.n;
        let half_n = n / 2;
        let rate = self.rate;
        let f0 = f0.max(3.0 * rate / n as f32);
        let half = (1.5 * rate / f0).round() as isize;
        let c = center.round() as isize;
        // Hann window over three periods, unit energy; weighted mean removed.
        let mut energy = 0.0f32;
        let mut weighted = 0.0f32;
        let mut weights = 0.0f32;
        for m in -half..=half {
            let w = 0.5 + 0.5 * (std::f32::consts::PI * m as f32 / (half + 1) as f32).cos();
            energy += w * w;
            weighted += at(x, c + m) * w;
            weights += w;
        }
        let mean = weighted / weights;
        let norm = 1.0 / energy.sqrt();
        self.buffer.fill(Complex32::default());
        for m in -half..=half {
            let w = 0.5 + 0.5 * (std::f32::consts::PI * m as f32 / (half + 1) as f32).cos();
            self.buffer[(m + half) as usize] =
                Complex32::new((at(x, c + m) - mean) * w * norm, 0.0);
        }
        self.fft.forward(&mut self.buffer);
        for k in 0..=half_n {
            self.power[k] = self.buffer[k].norm_sqr();
        }
        let bin_hz = rate / n as f32;
        // Fold the power below F0 back onto itself (DC correction).
        let f0_bins = f0 / bin_hz;
        for k in 0..f0_bins.floor() as usize {
            let mirror = f0_bins - k as f32;
            let i = mirror.floor() as usize;
            let t = mirror - i as f32;
            let value = self.power[i.min(half_n)] * (1.0 - t) + self.power[(i + 1).min(half_n)] * t;
            self.smoothed[k] = self.power[k] + value;
        }
        for k in f0_bins.floor() as usize..=half_n {
            self.smoothed[k] = self.power[k];
        }
        // Box smoothing two thirds of a harmonic wide, on mirrored edges.
        let width = 2.0 * f0_bins / 3.0;
        let pad = width.ceil() as usize + 1;
        let get = |k: isize| -> f32 {
            let k = if k < 0 { -k } else { k };
            let k = if k as usize > half_n {
                2 * half_n as isize - k
            } else {
                k
            };
            self.smoothed[k.clamp(0, half_n as isize) as usize]
        };
        let total = half_n + 2 * pad + 1;
        if self.cumulative.len() < total + 1 {
            self.cumulative.resize(total + 1, 0.0);
        }
        self.cumulative[0] = 0.0;
        for j in 0..total {
            self.cumulative[j + 1] = self.cumulative[j] + get(j as isize - pad as isize) as f64;
        }
        let cumulative = &self.cumulative;
        let integral = |position: f32| -> f64 {
            let p = (position + pad as f32).clamp(0.0, total as f32);
            let i = (p.floor() as usize).min(total - 1);
            let t = (p - i as f32) as f64;
            cumulative[i] + (cumulative[i + 1] - cumulative[i]) * t
        };
        for k in 0..=half_n {
            let lo = k as f32 - width * 0.5 + 0.5;
            let hi = k as f32 + width * 0.5 + 0.5;
            let mean = (integral(hi) - integral(lo)) / width as f64;
            self.log[k] = (mean.max(1e-30) as f32).ln();
        }
        // Cepstral liftering: smoothing (sinc) and compensation (q1 = -0.15).
        for k in 0..=half_n {
            self.buffer[k] = Complex32::new(self.log[k], 0.0);
        }
        for k in half_n + 1..n {
            self.buffer[k] = self.buffer[n - k];
        }
        self.fft.inverse(&mut self.buffer);
        let q1 = -0.15f32;
        for m in 0..n {
            let q = if m <= half_n { m } else { n - m } as f32;
            let lifter = if q == 0.0 {
                1.0
            } else {
                let a = std::f32::consts::PI * f0 * q / rate;
                (a.sin() / a) * ((1.0 - 2.0 * q1) + 2.0 * q1 * (2.0 * a).cos())
            };
            self.buffer[m] *= lifter / n as f32;
        }
        self.fft.forward(&mut self.buffer);
        for k in 0..=half_n {
            self.log[k] = self.buffer[k].re;
        }
        to_grid(&self.log, rate, n)
    }
}

/// Natural-log power per sample on FFT bins → dB per Hz on the mel grid.
fn to_grid(log_power: &[f32], rate: f32, n: usize) -> [f32; ENV_POINTS] {
    let half_n = n / 2;
    let bin_hz = rate / n as f32;
    let nyquist = rate * 0.5;
    let to_db = 10.0 / std::f32::consts::LN_10;
    let per_hz = 10.0 * rate.log10();
    let mut env = [FLOOR_DB; ENV_POINTS];
    let mut last_valid = 0;
    for (g, value) in env.iter_mut().enumerate() {
        let hz = grid_hz(g);
        if hz > nyquist * 0.98 {
            break;
        }
        // Average over the grid cell, so high bands do not alias.
        let lo = if g == 0 {
            0.0
        } else {
            (grid_hz(g - 1) + hz) * 0.5
        };
        let hi = if g + 1 < ENV_POINTS {
            (grid_hz(g + 1) + hz) * 0.5
        } else {
            hz
        };
        let (k0, k1) = (
            (lo / bin_hz).ceil() as usize,
            ((hi / bin_hz).floor() as usize).min(half_n),
        );
        let ln = if k1 > k0 {
            log_power[k0..=k1].iter().sum::<f32>() / (k1 - k0 + 1) as f32
        } else {
            let p = hz / bin_hz;
            let i = (p.floor() as usize).min(half_n - 1);
            let t = p - i as f32;
            log_power[i] * (1.0 - t) + log_power[i + 1] * t
        };
        *value = (ln * to_db - per_hz).max(FLOOR_DB);
        last_valid = g;
    }
    // Above the recording's Nyquist the envelope falls away steeply.
    for g in last_valid + 1..ENV_POINTS {
        env[g] = (env[last_valid] - 30.0 - 10.0 * (g - last_valid) as f32).max(FLOOR_DB);
    }
    let _ = ENV_MAX_HZ;
    env
}

// --- Aperiodicity ----------------------------------------------------------

struct Periodicity {
    rate: f32,
    fft: Transform,
    first: Vec<Complex32>,
    second: Vec<Complex32>,
    band_of_bin: Vec<u8>,
}
impl Periodicity {
    fn new(rate: u32) -> Self {
        let n = ((3.0 * rate as f32 / 70.0) as usize + 1).next_power_of_two();
        let bin_hz = rate as f32 / n as f32;
        let band_of_bin = (0..=n / 2)
            .map(|k| {
                let hz = k as f32 * bin_hz;
                (0..AP_BANDS)
                    .find(|&b| hz < AP_EDGES[b + 1])
                    .unwrap_or(AP_BANDS - 1) as u8
            })
            .collect();
        Self {
            rate: rate as f32,
            fft: Transform::new(n),
            first: vec![Complex32::default(); n],
            second: vec![Complex32::default(); n],
            band_of_bin,
        }
    }

    fn segment(&self, x: &[f32], center: isize, half: isize, out: &mut [Complex32]) {
        out.fill(Complex32::default());
        for m in -half..=half {
            let w = 0.42
                + 0.5 * (std::f32::consts::PI * m as f32 / (half + 1) as f32).cos()
                + 0.08 * (std::f32::consts::TAU * m as f32 / (half + 1) as f32).cos();
            out[(m + half) as usize] = Complex32::new(at(x, center + m) * w, 0.0);
        }
    }

    /// Compares the waveform with itself one period later. Returns the
    /// refined F0 (the best-aligned period) and per-band noise share.
    fn measure(&mut self, x: &[f32], center: f64, f0: f32) -> (f32, [f32; AP_BANDS]) {
        let n = self.fft.n;
        let half_n = n / 2;
        let period = self.rate / f0;
        let rounded = period.round();
        let half = ((1.5 * period).round() as isize).min(half_n as isize - 1);
        let c1 = (center - period as f64 * 0.5).round() as isize;
        let c2 = c1 + rounded as isize;
        let mut first = std::mem::take(&mut self.first);
        let mut second = std::mem::take(&mut self.second);
        self.segment(x, c1, half, &mut first);
        self.segment(x, c2, half, &mut second);
        self.fft.forward(&mut first);
        self.fft.forward(&mut second);
        let omega = std::f32::consts::TAU / n as f32;
        let bin_hz = self.rate / n as f32;
        let top = (4000.0 / bin_hz) as usize;
        // Correlation of the two spectra when the second is shifted by delta samples.
        let correlate = |delta: f32, upto: usize| -> f32 {
            (1..upto.min(half_n))
                .map(|k| {
                    let phase = omega * k as f32 * delta;
                    let rotated = second[k] * Complex32::new(phase.cos(), phase.sin());
                    (first[k] * rotated.conj()).re
                })
                .sum()
        };
        // Search the residual period offset within ±2 % and refine by parabola.
        let span = (period * 0.02).max(0.6);
        let steps = 16;
        let mut best = (f32::NEG_INFINITY, 0.0f32);
        let mut values = Vec::with_capacity(steps + 1);
        for s in 0..=steps {
            let delta = (period - rounded) + span * (2.0 * s as f32 / steps as f32 - 1.0);
            let r = correlate(delta, top);
            values.push((delta, r));
            if r > best.0 {
                best = (r, delta);
            }
        }
        let mut delta = best.1;
        if let Some(i) = values.iter().position(|v| v.1 == best.0)
            && i > 0
            && i < steps
        {
            let (y0, y1, y2) = (values[i - 1].1, values[i].1, values[i + 1].1);
            let d = y0 - 2.0 * y1 + y2;
            if d.abs() > 1e-20 {
                let step = values[1].0 - values[0].0;
                delta += (0.5 * (y0 - y2) / d).clamp(-0.5, 0.5) * step;
            }
        }
        // Each band takes its own best alignment within the search span: a
        // slightly wrong period must not read as noise at high frequencies.
        let mut cross = [0.0f32; AP_BANDS];
        let mut p1 = [0.0f32; AP_BANDS];
        let mut p2 = [0.0f32; AP_BANDS];
        for k in 1..half_n {
            let b = self.band_of_bin[k] as usize;
            p1[b] += first[k].norm_sqr();
            p2[b] += second[k].norm_sqr();
        }
        let mut best_cross = [f32::NEG_INFINITY; AP_BANDS];
        let center_delta = delta;
        let fine = 24;
        for s in 0..=fine {
            let delta = center_delta + span * 0.5 * (2.0 * s as f32 / fine as f32 - 1.0);
            cross.fill(0.0);
            for k in 1..half_n {
                let b = self.band_of_bin[k] as usize;
                let phase = omega * k as f32 * delta;
                let rotated = second[k] * Complex32::new(phase.cos(), phase.sin());
                cross[b] += (first[k] * rotated.conj()).re;
            }
            for b in 0..AP_BANDS {
                best_cross[b] = best_cross[b].max(cross[b]);
            }
        }
        let nyquist = self.rate * 0.5;
        let mut ap = [1.0f32; AP_BANDS];
        for b in 0..AP_BANDS {
            if AP_EDGES[b] >= nyquist {
                ap[b] = ap[b.saturating_sub(1)];
                continue;
            }
            let den = (p1[b] * p2[b]).sqrt();
            let rho = if den > 0.0 {
                (best_cross[b] / den).clamp(0.0, 1.0)
            } else {
                0.0
            };
            ap[b] = (1.0 - rho).clamp(0.001, 1.0);
        }
        self.first = first;
        self.second = second;
        let refined = self.rate / (rounded + delta);
        (refined, ap)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spectrum::Rng;

    /// A glottal-like pulse train through two resonances, plus noise.
    fn voice(rate: u32, f0: impl Fn(f32) -> f32, seconds: f32, noise: f32) -> Vec<f32> {
        let len = (seconds * rate as f32) as usize;
        let mut rng = Rng::new(5);
        let mut phase = 0.0f32;
        let mut out = Vec::with_capacity(len);
        let resonators = [(700.0f32, 80.0f32), (1200.0, 100.0), (2600.0, 160.0)];
        let mut state = [[0.0f32; 2]; 3];
        for i in 0..len {
            let t = i as f32 / rate as f32;
            let f = f0(t);
            let mut excitation = 0.0;
            if f > 0.0 {
                phase += f / rate as f32;
                if phase >= 1.0 {
                    phase -= 1.0;
                    excitation = 1.0;
                }
            }
            excitation += noise * rng.normal() * 0.05;
            let mut y = excitation;
            for (r, s) in resonators.iter().zip(state.iter_mut()) {
                let radius = (-std::f32::consts::PI * r.1 / rate as f32).exp();
                let theta = std::f32::consts::TAU * r.0 / rate as f32;
                let out = y + 2.0 * radius * theta.cos() * s[0] - radius * radius * s[1];
                s[1] = s[0];
                s[0] = out;
                y = out * (1.0 - radius);
            }
            out.push(y * 0.5);
        }
        out
    }

    #[test]
    fn f0_follows_a_glide_and_finds_silence() {
        let rate = 44_100;
        let x = voice(
            rate,
            |t| {
                if t < 0.5 {
                    0.0
                } else {
                    150.0 + 200.0 * (t - 0.5)
                }
            },
            1.5,
            0.0,
        );
        let f0 = track_f0(&x, rate, &Options::default());
        let hop = FRAME_SECONDS;
        let mut errors = 0;
        for (i, &f) in f0.iter().enumerate() {
            let t = i as f32 * hop;
            if (0.55..1.45).contains(&t) {
                let truth = 150.0 + 200.0 * (t - 0.5);
                if (f / truth - 1.0).abs() > 0.01 {
                    errors += 1;
                }
            }
            if t < 0.45 {
                assert_eq!(f, 0.0, "voiced at {t}");
            }
        }
        assert!(errors < 4, "{errors} frames off");
    }

    #[test]
    fn high_voices_keep_their_fundamental() {
        let rate = 48_000;
        for f in [494.0f32, 700.0, 988.0] {
            let x = voice(rate, |_| f, 0.6, 0.0);
            let f0 = track_f0(&x, rate, &Options::default());
            let wrong = f0[10..100]
                .iter()
                .filter(|&&v| (v / f - 1.0).abs() > 0.02)
                .count();
            assert!(wrong == 0, "{f} Hz: {wrong} frames off");
        }
    }

    #[test]
    fn analysis_and_synthesis_round_trip() {
        let rate = 48_000;
        let x = voice(rate, |_| 220.0, 1.0, 0.0);
        let frames = analyze(&x, rate, &Options::default());
        let middle = &frames[50..150];
        for f in middle {
            assert!((f.f0 / 220.0 - 1.0).abs() < 0.003, "f0 {}", f.f0);
            assert!(f.ap[0] < 0.05 && f.ap[2] < 0.1, "ap {:?}", f.ap);
        }
        // The first formant peak stands above the valley between F2 and F3.
        let f = &middle[50];
        assert!(f.env_at(700.0) > f.env_at(1800.0) + 6.0);
        let mut vocoder = crate::vocoder::Vocoder::new(rate);
        let y = vocoder.render(&frames, &[], 9);
        let power =
            |s: &[f32]| 10.0 * (s.iter().map(|v| v * v).sum::<f32>() / s.len() as f32).log10();
        let (a, b) = (power(&x[12_000..36_000]), power(&y[12_000..36_000]));
        assert!((a - b).abs() < 1.5, "original {a} dB, resynthesized {b} dB");
    }

    #[test]
    fn noise_is_aperiodic() {
        let rate = 44_100;
        // Noise at a quarter of the pulse power: a fifth of every band is noise.
        // 210 Hz keeps the test pulses on whole samples, so they are exactly periodic.
        let x = voice(
            rate,
            |_| 210.0,
            0.6,
            (0.25 * 210.0 / rate as f32).sqrt() / 0.05,
        );
        let frames = analyze(&x, rate, &Options::default());
        let voiced: Vec<_> = frames.iter().filter(|f| f.voiced()).collect();
        assert!(voiced.len() > 60, "{} voiced frames", voiced.len());
        for band in [1, 3, 6] {
            let mean: f32 = voiced.iter().map(|f| f.ap[band]).sum::<f32>() / voiced.len() as f32;
            assert!(
                (0.1..0.4).contains(&mean),
                "band {band} aperiodicity {mean}"
            );
        }
    }
}
