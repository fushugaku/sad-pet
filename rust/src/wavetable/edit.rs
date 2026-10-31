//! Explicit offline table edits. Clean playback and the canonical compiler
//! retain their original semantics; character processing creates a new asset.
use super::{FRAME_SIZE, MAX_FRAMES, assets::Asset};
use rustfft::{FftPlanner, num_complex::Complex32};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Harmonic {
    pub harmonic: usize,
    pub gain: f32,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Local {
    pub frame: usize,
    pub phase: f32,
    pub gains: Vec<Harmonic>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Settings {
    /// Source-frame indices in the desired order, including intentional repeats.
    pub order: Option<Vec<usize>>,
    pub reverse: bool,
    pub phase: f32,
    pub align: bool,
    pub bits: u8,
    pub phase_steps: usize,
    pub harmonics: usize,
    /// dB/octave about harmonic 8.
    pub tilt: f32,
    pub normalize: bool,
    /// None = all output frames, Some = one output frame.
    pub frame: Option<usize>,
    pub gains: Vec<Harmonic>,
    pub locals: Vec<Local>,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            order: None,
            reverse: false,
            phase: 0.0,
            align: false,
            bits: 24,
            phase_steps: FRAME_SIZE,
            harmonics: FRAME_SIZE / 2 - 1,
            tilt: 0.0,
            normalize: false,
            frame: None,
            gains: Vec::new(),
            locals: Vec::new(),
        }
    }
}
pub fn edit(source: &Asset, name: String, s: &Settings) -> Result<Asset, String> {
    source.validate()?;
    if !(4..=24).contains(&s.bits)
        || !s.phase.is_finite()
        || !(0.0..=1.0).contains(&s.phase)
        || !s.tilt.is_finite()
        || !(-12.0..=12.0).contains(&s.tilt)
        || !(1..FRAME_SIZE / 2).contains(&s.harmonics)
        || !s.phase_steps.is_power_of_two()
        || !(16..=FRAME_SIZE).contains(&s.phase_steps)
        || s.gains.len() > 1023
    {
        return Err("Invalid table edit settings".into());
    }
    if s.gains.iter().any(|g| {
        !(1..FRAME_SIZE / 2).contains(&g.harmonic)
            || !g.gain.is_finite()
            || !(0.0..=4.0).contains(&g.gain)
    }) {
        return Err("Harmonic gains must be finite, 0–4, for harmonics 1–1023".into());
    }
    let mut order = s
        .order
        .clone()
        .unwrap_or_else(|| (0..source.frame_count).collect());
    if order.is_empty()
        || order.len() > MAX_FRAMES
        || order.iter().any(|&n| n >= source.frame_count)
        || s.frame.is_some_and(|n| n >= order.len())
    {
        return Err("Choose 1–256 existing frames".into());
    }
    if s.locals.len() > MAX_FRAMES
        || s.locals.iter().map(|l| l.gains.len()).sum::<usize>() > 16384
        || s.locals.iter().any(|l| {
            l.frame >= order.len()
                || !l.phase.is_finite()
                || !(-1.0..=1.0).contains(&l.phase)
                || l.gains.iter().any(|g| {
                    !(1..FRAME_SIZE / 2).contains(&g.harmonic)
                        || !g.gain.is_finite()
                        || !(0.0..=4.0).contains(&g.gain)
                })
        })
    {
        return Err("Invalid per-frame edit".into());
    }
    let mut local_frames = [false; MAX_FRAMES];
    for l in &s.locals {
        if local_frames[l.frame] {
            return Err("Set each frame edit once".into());
        }
        local_frames[l.frame] = true;
        let mut used = [false; FRAME_SIZE / 2];
        for g in &l.gains {
            if used[g.harmonic] {
                return Err("Set each local harmonic once".into());
            }
            used[g.harmonic] = true;
        }
    }
    if s.reverse {
        order.reverse();
    }
    let identity = !s.reverse
        && !s.align
        && s.phase == 0.0
        && s.bits == 24
        && s.phase_steps == FRAME_SIZE
        && s.harmonics == FRAME_SIZE / 2 - 1
        && s.tilt == 0.0
        && !s.normalize
        && s.gains.is_empty()
        && s.locals.is_empty()
        && order.iter().copied().eq(0..source.frame_count);
    if identity {
        let mut copy = source.clone();
        copy.name = name;
        if copy.id.starts_with("factory:") {
            copy.id = format!("user:{}", copy.content_hash);
        }
        return Ok(copy);
    }
    let mut gains = [1.0_f32; FRAME_SIZE / 2];
    let mut seen = [false; FRAME_SIZE / 2];
    for g in &s.gains {
        if seen[g.harmonic] {
            return Err("Set each harmonic gain once".into());
        }
        seen[g.harmonic] = true;
        gains[g.harmonic] = g.gain;
    }
    let mut result = Vec::with_capacity(order.len() * FRAME_SIZE);
    let mut planner = FftPlanner::<f32>::new();
    let forward = planner.plan_fft_forward(FRAME_SIZE);
    let inverse = planner.plan_fft_inverse(FRAME_SIZE);
    let mut spectrum = vec![Complex32::default(); FRAME_SIZE];
    let mut anchor = spectrum.clone();
    let mut correlation = spectrum.clone();
    for (frame, &index) in order.iter().enumerate() {
        let input = &source.samples[index * FRAME_SIZE..(index + 1) * FRAME_SIZE];
        for (v, &x) in spectrum.iter_mut().zip(input) {
            *v = Complex32::new(x, 0.0);
        }
        forward.process(&mut spectrum);
        let selected = s.frame.is_none_or(|n| n == frame);
        let local = s.locals.iter().find(|l| l.frame == frame);
        let mut local_gains = [1.0_f32; FRAME_SIZE / 2];
        if let Some(local) = local {
            for g in &local.gains {
                local_gains[g.harmonic] = g.gain;
            }
        }
        let mut shift = if selected { s.phase } else { 0.0 };
        shift += local.map_or(0.0, |l| l.phase);
        if frame == 0 {
            anchor.copy_from_slice(&spectrum);
        } else if s.align {
            for ((c, &a), &b) in correlation.iter_mut().zip(&anchor).zip(&spectrum) {
                *c = a.conj() * b;
            }
            inverse.process(&mut correlation);
            let peak = correlation
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.re.total_cmp(&b.re))
                .unwrap()
                .0;
            let a = correlation[(peak + FRAME_SIZE - 1) % FRAME_SIZE].re;
            let b = correlation[peak].re;
            let c = correlation[(peak + 1) % FRAME_SIZE].re;
            let curve = a - 2.0 * b + c;
            let correction = if curve.abs() > 1e-12 {
                0.5 * (a - c) / curve
            } else {
                0.0
            };
            shift += (peak as f32 + correction.clamp(-0.5, 0.5)) / FRAME_SIZE as f32;
        }
        for h in 1..FRAME_SIZE / 2 {
            let tilt = if selected {
                10.0_f32.powf(s.tilt * (h as f32 / 8.0).log2() / 20.0)
            } else {
                1.0
            };
            let gain = (if selected { gains[h] } else { 1.0 }) * local_gains[h];
            let c = if h > s.harmonics && selected {
                Complex32::default()
            } else {
                spectrum[h]
                    * Complex32::from_polar(tilt * gain, std::f32::consts::TAU * h as f32 * shift)
            };
            spectrum[h] = c;
            spectrum[FRAME_SIZE - h] = c.conj();
        }
        spectrum[0] = Complex32::default();
        spectrum[FRAME_SIZE / 2] = Complex32::default();
        inverse.process(&mut spectrum);
        let scale = 2.0_f32.powi(s.bits as i32 - 1) - 1.0;
        for n in 0..FRAME_SIZE {
            let at = if selected {
                n * s.phase_steps / FRAME_SIZE * FRAME_SIZE / s.phase_steps
            } else {
                n
            };
            let mut sample = spectrum[at].re / FRAME_SIZE as f32;
            if s.bits < 24 && selected {
                sample = (sample * scale).round() / scale;
            }
            result.push(sample);
        }
    }
    if s.normalize {
        let peak = result.iter().map(|x| x.abs()).fold(0.0_f32, f32::max);
        if peak > 1e-8 {
            for x in &mut result {
                *x *= 0.95 / peak;
            }
        }
    }
    let mut asset = Asset::from_cycles(name, FRAME_SIZE, &result)?;
    asset.author = source.author.clone();
    asset.provenance = format!(
        "Offline edit of {}: explicit frame/phase/spectral/quantization settings",
        source.id
    );
    Ok(asset)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn source() -> Asset {
        let v = (0..FRAME_SIZE * 3)
            .map(|n| {
                let frame = n / FRAME_SIZE;
                let p = (n % FRAME_SIZE) as f32 / FRAME_SIZE as f32 * std::f32::consts::TAU;
                (p + frame as f32 * 0.7).sin() * 0.6 + (p * 3.0 + frame as f32 * 2.1).sin() * 0.15
            })
            .collect::<Vec<_>>();
        Asset::from_cycles("Source".into(), FRAME_SIZE, &v).unwrap()
    }
    #[test]
    fn identity_order_trim_and_character_are_content_addressed() {
        let a = source();
        assert_eq!(
            edit(&a, "Copy".into(), &Settings::default()).unwrap().id,
            a.id
        );
        let s = Settings {
            order: Some(vec![2, 0]),
            bits: 8,
            phase_steps: 128,
            normalize: true,
            ..Default::default()
        };
        let b = edit(&a, "Vintage".into(), &s).unwrap();
        assert_eq!(b.frame_count, 2);
        assert_ne!(a.id, b.id);
        b.validate().unwrap();
        assert_eq!(b.id, edit(&a, "Again".into(), &s).unwrap().id);
    }
    #[test]
    fn alignment_removes_cancellation_and_harmonic_edit_removes_the_partial() {
        let a = source();
        let b = edit(
            &a,
            "Aligned".into(),
            &Settings {
                align: true,
                ..Default::default()
            },
        )
        .unwrap();
        let f = &b.samples[..FRAME_SIZE];
        let last = &b.samples[FRAME_SIZE * 2..];
        let error = f
            .iter()
            .zip(last)
            .map(|(a, b)| (a - b).abs())
            .fold(0.0_f32, f32::max);
        assert!(error < 0.02, "{error}");
        let muted = edit(
            &a,
            "No fundamental".into(),
            &Settings {
                gains: vec![Harmonic {
                    harmonic: 1,
                    gain: 0.0,
                }],
                ..Default::default()
            },
        )
        .unwrap();
        let magnitude = muted.samples[..FRAME_SIZE]
            .iter()
            .enumerate()
            .map(|(n, x)| {
                Complex32::from_polar(*x, std::f32::consts::TAU * n as f32 / FRAME_SIZE as f32)
            })
            .sum::<Complex32>()
            .norm();
        assert!(magnitude < 0.001);
    }
}
