//! Offline audio-to-table conversion. Cycle extraction keeps pitch periods;
//! spectral snapshots deliberately create periodic textures from arbitrary audio.
use super::{
    FRAME_SIZE, MAX_FRAMES,
    assets::Asset,
    import::{self, Channel},
};
use rustfft::{FftPlanner, num_complex::Complex32};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Method {
    #[default]
    Spectral,
    Cycles,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Settings {
    pub method: Method,
    pub frames: usize,
    pub start: f64,
    pub end: Option<f64>,
    pub root_note: Option<f32>,
    pub channel: Option<Channel>,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            method: Method::Spectral,
            frames: 64,
            start: 0.0,
            end: None,
            root_note: None,
            channel: None,
        }
    }
}

fn sample(data: &[f32], at: f64) -> f32 {
    let at = at.clamp(0.0, (data.len() - 1) as f64);
    let i = at as usize;
    let f = (at - i as f64) as f32;
    data[i] + (data[(i + 1).min(data.len() - 1)] - data[i]) * f
}
fn period(data: &[f32], center: usize, rate: u32) -> Option<f64> {
    let stride = (rate as usize / 12000).max(1);
    let fs = rate as f64 / stride as f64;
    let maximum =
        ((fs / 35.0) as usize).min(data.len().saturating_div(stride * 3).saturating_sub(1));
    let minimum = (fs / 1800.0).max(2.0) as usize;
    if maximum <= minimum {
        return None;
    }
    let length = maximum * 2;
    let start = center
        .saturating_sub(length * stride / 2)
        .min(data.len().saturating_sub((length + maximum) * stride));
    let mut signal = vec![0.0; length + maximum];
    for (i, x) in signal.iter_mut().enumerate() {
        let at = (start + i * stride).min(data.len() - 1);
        let stop = (at + stride).min(data.len());
        *x = data[at..stop].iter().sum::<f32>() / (stop - at) as f32;
    }
    let energy = signal.iter().map(|x| x * x).sum::<f32>();
    if energy < 1e-8 {
        return None;
    }
    let mut scores = vec![1.0; maximum + 1];
    let mut cumulative = 0.0;
    for tau in 1..=maximum {
        let difference = (0..length)
            .map(|n| {
                let d = signal[n] - signal[n + tau];
                d * d
            })
            .sum::<f32>();
        cumulative += difference;
        scores[tau] = difference * tau as f32 / cumulative.max(1e-20);
    }
    let mut tau = minimum;
    while tau < maximum {
        if scores[tau] < 0.15 {
            while tau < maximum && scores[tau + 1] < scores[tau] {
                tau += 1;
            }
            let left = scores[tau - 1] as f64;
            let right = scores[(tau + 1).min(maximum)] as f64;
            let mid = scores[tau] as f64;
            let curvature = left - 2.0 * mid + right;
            let correction = if curvature.abs() > 1e-12 {
                0.5 * (left - right) / curvature
            } else {
                0.0
            };
            return Some((tau as f64 + correction.clamp(-0.5, 0.5)) * stride as f64);
        }
        tau += 1;
    }
    None
}
fn crossing(data: &[f32], at: f64, radius: f64) -> Option<f64> {
    let first = (at - radius).max(0.0) as usize;
    let last = ((at + radius).ceil() as usize).min(data.len() - 1);
    (first..last)
        .filter(|&i| data[i] <= 0.0 && data[i + 1] > 0.0)
        .map(|i| i as f64 - data[i] as f64 / (data[i + 1] - data[i]) as f64)
        .min_by(|a, b| (a - at).abs().total_cmp(&(b - at).abs()))
}
pub fn make(bytes: &[u8], name: String, settings: &Settings) -> Result<Asset, String> {
    let info = import::inspect(bytes)?;
    if !(1..=MAX_FRAMES).contains(&settings.frames) {
        return Err("Choose 1–256 frames".into());
    }
    if info.channels == 2 && settings.channel.is_none() {
        return Err("Choose Left, Right or Mix for the stereo audio".into());
    }
    if !settings.start.is_finite()
        || settings.start < 0.0
        || settings
            .end
            .is_some_and(|v| !v.is_finite() || v <= settings.start)
    {
        return Err("Choose a valid audio range".into());
    }
    if settings
        .root_note
        .is_some_and(|n| !n.is_finite() || !(0.0..=127.0).contains(&n))
    {
        return Err("Choose a MIDI root note from 0 to 127".into());
    }
    let rate = info.sample_rate;
    let first = (settings.start * rate as f64).round() as usize;
    let last = settings.end.map_or(info.samples_per_channel, |v| {
        (v * rate as f64).round() as usize
    });
    if first >= last || last > info.samples_per_channel || last - first < 128 {
        return Err("Select at least 128 samples within the recording".into());
    }
    if last - first > rate as usize * 10 {
        return Err("Wave Maker processes at most ten seconds at a time; trim the range".into());
    }
    // Decode only the selected range into mono, rather than first expanding
    // a whole 16 MiB mono PCM file into a 64 MiB stereo allocation.
    let mut reader =
        hound::WavReader::new(std::io::Cursor::new(bytes)).map_err(|e| e.to_string())?;
    let spec = reader.spec();
    let channels = spec.channels as usize;
    let collect =
        |samples: &mut dyn Iterator<Item = Result<f32, String>>| -> Result<Vec<f32>, String> {
            let mut mono = Vec::with_capacity(last - first);
            for _ in first..last {
                let left = samples.next().ok_or("Incomplete audio frame")??;
                let right = if channels == 2 {
                    samples.next().ok_or("Incomplete audio frame")??
                } else {
                    left
                };
                if !left.is_finite() || !right.is_finite() {
                    return Err("The selected audio contains non-finite samples".into());
                }
                mono.push(match settings.channel.unwrap_or(Channel::Left) {
                    Channel::Left => left,
                    Channel::Right => right,
                    Channel::Mix => (left + right) * 0.5,
                });
            }
            Ok(mono)
        };
    let mono = if spec.sample_format == hound::SampleFormat::Float {
        collect(
            &mut reader
                .samples::<f32>()
                .skip(first * channels)
                .map(|v| v.map_err(|e| e.to_string())),
        )?
    } else {
        let scale = 2.0_f64.powi(spec.bits_per_sample as i32 - 1) as f32;
        collect(
            &mut reader
                .samples::<i32>()
                .skip(first * channels)
                .map(|v| v.map(|x| x as f32 / scale).map_err(|e| e.to_string())),
        )?
    };
    if mono.iter().map(|v| v.abs()).fold(0.0_f32, f32::max) < 1e-7 {
        return Err("The selected audio is silent".into());
    }
    let mut result = Vec::with_capacity(settings.frames * FRAME_SIZE);
    let mut planner = FftPlanner::<f32>::new();
    let forward = planner.plan_fft_forward(FRAME_SIZE);
    let inverse = planner.plan_fft_inverse(FRAME_SIZE);
    let mut spectrum = vec![Complex32::default(); FRAME_SIZE];
    let mut wave = spectrum.clone();
    for frame in 0..settings.frames {
        let position = if settings.frames == 1 {
            0.5
        } else {
            frame as f64 / (settings.frames - 1) as f64
        };
        match settings.method {
            Method::Spectral => {
                let start = (position * mono.len().saturating_sub(FRAME_SIZE) as f64) as usize;
                for (i, v) in spectrum.iter_mut().enumerate() {
                    let window = 0.5
                        - 0.5 * (std::f32::consts::TAU * i as f32 / (FRAME_SIZE - 1) as f32).cos();
                    let offset = if mono.len() < FRAME_SIZE {
                        (FRAME_SIZE - mono.len()) / 2
                    } else {
                        0
                    };
                    let value = if i >= offset {
                        mono.get(start + i - offset).copied().unwrap_or(0.0)
                    } else {
                        0.0
                    };
                    *v = Complex32::new(value * window, 0.0);
                }
                forward.process(&mut spectrum);
                wave.fill(Complex32::default());
                for h in 1..FRAME_SIZE / 2 {
                    let phase =
                        -std::f32::consts::PI * (h * (h - 1)) as f32 / (FRAME_SIZE / 2) as f32;
                    let c = Complex32::from_polar(spectrum[h].norm() * 2.0, phase);
                    wave[h] = c;
                    wave[FRAME_SIZE - h] = c.conj();
                }
                inverse.process(&mut wave);
                result.extend(wave.iter().map(|v| v.re / FRAME_SIZE as f32));
            }
            Method::Cycles => {
                let center = (position * (mono.len() - 1) as f64) as usize;
                let length = if let Some(root) = settings.root_note {
                    rate as f64 / (440.0 * 2.0_f64.powf((root as f64 - 69.0) / 12.0))
                } else {
                    period(&mono, center, rate).ok_or(
                        "No stable pitch found. Set the root note or use Spectral snapshots",
                    )?
                };
                if (mono.len() as f64) < length * 2.0 {
                    return Err("The selection needs at least two pitch periods".into());
                }
                let center = length * 0.5 + position * (mono.len() as f64 - length * 2.0);
                let from = crossing(&mono, center, length * 0.5).unwrap_or(center);
                let end = crossing(&mono, from + length, length * 0.2)
                    .unwrap_or(from + length)
                    .min((mono.len() - 1) as f64);
                let actual = end - from;
                if actual < 2.0 {
                    return Err("The selected pitch period is incomplete".into());
                }
                // Windowed sinc also lowpasses when a long period is reduced.
                let cutoff = (FRAME_SIZE as f64 / actual).min(1.0);
                for i in 0..FRAME_SIZE {
                    let at = from + i as f64 / FRAME_SIZE as f64 * actual;
                    let mut sum = 0.0;
                    let mut weight = 0.0;
                    for tap in -16..=16 {
                        let x = at.floor() + tap as f64;
                        let d = (at - x) * cutoff;
                        let sinc = if d.abs() < 1e-12 {
                            1.0
                        } else {
                            (std::f64::consts::PI * d).sin() / (std::f64::consts::PI * d)
                        };
                        let window =
                            (0.5 + 0.5 * (std::f64::consts::PI * (at - x) / 17.0).cos()).max(0.0);
                        let w = sinc * window;
                        sum += sample(&mono, x) as f64 * w;
                        weight += w;
                    }
                    result.push((sum / weight.max(1e-12)) as f32);
                }
            }
        }
    }
    let mut asset = Asset::from_cycles(name, FRAME_SIZE, &result)?;
    asset.provenance=match settings.method {Method::Cycles=>"Wave Maker: pitch-period extraction and phase alignment",Method::Spectral=>"Wave Maker: Hann spectral snapshots, coherent Schroeder phases; source timing and pitch are not retained"}.into();
    Ok(asset)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn audio() -> Vec<u8> {
        let frames = (0..48000)
            .map(|i| {
                let t = i as f32 / 48000.0;
                let a = (t * std::f32::consts::TAU * 220.0).sin() * 0.6;
                [
                    a,
                    a * 0.5 + (t * std::f32::consts::TAU * 660.0).sin() * t * 0.25,
                ]
            })
            .collect::<Vec<_>>();
        crate::wav::bytes(48000, &frames).unwrap()
    }
    #[test]
    fn arbitrary_audio_makes_bounded_spectral_tables_or_detected_cycles() {
        let bytes = audio();
        let mut s = Settings {
            frames: 8,
            channel: Some(Channel::Left),
            ..Default::default()
        };
        let spectrum = make(&bytes, "Spectrum".into(), &s).unwrap();
        assert_eq!(spectrum.frame_count, 8);
        spectrum.validate().unwrap();
        s.method = Method::Cycles;
        let cycles = make(&bytes, "Cycles".into(), &s).unwrap();
        cycles.validate().unwrap();
        let a = &cycles.samples[..FRAME_SIZE];
        let rms = (a.iter().map(|v| v * v).sum::<f32>() / FRAME_SIZE as f32).sqrt();
        assert!((rms - 0.6 / 2.0_f32.sqrt()).abs() < 0.02, "{rms}");
        let error = a
            .iter()
            .zip(&cycles.samples[FRAME_SIZE * 7..])
            .map(|(a, b)| (a - b).abs())
            .fold(0.0_f32, f32::max);
        assert!(error < 0.02, "phase-aligned cycle error {error}");
        s.frames = 257;
        assert!(make(&bytes, "Invalid".into(), &s).is_err());
        s.frames = 8;
        s.channel = None;
        assert!(make(&bytes, "Stereo choice".into(), &s).is_err());
    }
}
