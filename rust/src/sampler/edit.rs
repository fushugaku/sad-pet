//! Destructive sample edits and analysis, as in the MPC's Sample Edit mode.
//! Every edit takes the whole sample and returns a new one; the editor keeps
//! the original in the library, so edits never lose audio. Positions are
//! frames; an `end` of 0 is the end of the sample.
use crate::{Stereo, dsp::SampleBuffer};
use serde::{Deserialize, Serialize};
use std::ops::Range;

/// A sample being edited: frames at a rate.
#[derive(Clone, Debug, PartialEq)]
pub struct Audio {
    pub rate: u32,
    pub frames: Vec<Stereo>,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum Operation {
    /// Keeps only the region.
    Crop {
        start: u32,
        end: u32,
    },
    Delete {
        start: u32,
        end: u32,
    },
    Silence {
        start: u32,
        end: u32,
    },
    InsertSilence {
        at: u32,
        ms: f32,
    },
    /// Scales the region so its peak reaches `peak` dBFS.
    Normalize {
        start: u32,
        end: u32,
        peak: f32,
    },
    Gain {
        start: u32,
        end: u32,
        db: f32,
    },
    Reverse {
        start: u32,
        end: u32,
    },
    FadeIn {
        start: u32,
        end: u32,
        curve: f32,
    },
    FadeOut {
        start: u32,
        end: u32,
        curve: f32,
    },
    /// Blends `ms` of audio before the loop start into the end of the loop,
    /// so a plain forward loop from `loop_start` to `end` joins smoothly.
    LoopCrossfade {
        loop_start: u32,
        end: u32,
        ms: f32,
    },
    RemoveDc {
        start: u32,
        end: u32,
    },
    Invert {
        start: u32,
        end: u32,
    },
    Mono,
    Swap,
    /// Semitones; `keep_length` shifts the pitch without changing timing,
    /// otherwise it is varispeed, like a tape or the MPC's Pitch Shift off.
    Pitch {
        semitones: f32,
        keep_length: bool,
    },
    /// New length over old, keeping the pitch.
    Stretch {
        factor: f32,
    },
    /// Converts to a new sample rate, keeping pitch and duration.
    Resample {
        rate: u32,
    },
    /// Bit depth and a sample-and-hold rate in hertz, as old samplers.
    Crush {
        bits: u8,
        rate: f32,
    },
    /// Removes leading and trailing audio below `threshold` dBFS.
    TrimSilence {
        threshold: f32,
    },
}

fn span(len: usize, start: u32, end: u32) -> Result<Range<usize>, String> {
    let end = if end == 0 {
        len
    } else {
        (end as usize).min(len)
    };
    let start = start as usize;
    if start >= end {
        return Err("Select a region of at least one frame".into());
    }
    Ok(start..end)
}

fn shape(x: f32, curve: f32) -> f32 {
    let x = x.clamp(0.0, 1.0);
    if curve == 0.0 || x == 0.0 {
        x
    } else {
        x.powf(4.0_f32.powf(curve.clamp(-1.0, 1.0)))
    }
}

impl Audio {
    pub fn new(rate: u32, frames: Vec<Stereo>) -> Self {
        Self { rate, frames }
    }
    pub fn from_sample(sample: &SampleBuffer) -> Self {
        Self::new(sample.sample_rate, sample.frames().to_vec())
    }
    pub fn apply(&self, operation: Operation) -> Result<Audio, String> {
        let len = self.frames.len();
        let mut frames = self.frames.clone();
        let mut rate = self.rate;
        match operation {
            Operation::Crop { start, end } => {
                frames = frames[span(len, start, end)?].to_vec();
            }
            Operation::Delete { start, end } => {
                frames.drain(span(len, start, end)?);
            }
            Operation::Silence { start, end } => {
                frames[span(len, start, end)?].fill([0.0; 2]);
            }
            Operation::InsertSilence { at, ms } => {
                let count = (ms.clamp(0.0, 600_000.0) * 0.001 * rate as f32) as usize;
                let at = (at as usize).min(len);
                frames.splice(at..at, std::iter::repeat_n([0.0; 2], count));
            }
            Operation::Normalize { start, end, peak } => {
                let range = span(len, start, end)?;
                let loudest = frames[range.clone()]
                    .iter()
                    .flatten()
                    .fold(0.0_f32, |m, v| m.max(v.abs()));
                if loudest > 0.0 {
                    let gain = 10.0_f32.powf(peak.clamp(-60.0, 6.0) / 20.0) / loudest;
                    frames[range].iter_mut().flatten().for_each(|v| *v *= gain);
                }
            }
            Operation::Gain { start, end, db } => {
                let gain = 10.0_f32.powf(db.clamp(-96.0, 48.0) / 20.0);
                frames[span(len, start, end)?]
                    .iter_mut()
                    .flatten()
                    .for_each(|v| *v *= gain);
            }
            Operation::Reverse { start, end } => frames[span(len, start, end)?].reverse(),
            Operation::FadeIn { start, end, curve } | Operation::FadeOut { start, end, curve } => {
                let range = span(len, start, end)?;
                let count = range.len().max(2) - 1;
                let rising = matches!(operation, Operation::FadeIn { .. });
                for (i, frame) in frames[range].iter_mut().enumerate() {
                    let x = i as f32 / count as f32;
                    let gain = shape(if rising { x } else { 1.0 - x }, curve);
                    frame.iter_mut().for_each(|v| *v *= gain);
                }
            }
            Operation::LoopCrossfade {
                loop_start,
                end,
                ms,
            } => {
                let range = span(len, loop_start, end)?;
                let count = ((ms.max(0.0) * 0.001 * rate as f32) as usize)
                    .min(range.start)
                    .min(range.len());
                for i in 0..count {
                    // The loop's last frame becomes the one before its start.
                    let t = (i + 1) as f32 / count as f32;
                    let from = frames[range.start - count + i];
                    let to = &mut frames[range.end - count + i];
                    for c in 0..2 {
                        to[c] = to[c] * (1.0 - t).sqrt() + from[c] * t.sqrt();
                    }
                }
            }
            Operation::RemoveDc { start, end } => {
                let range = span(len, start, end)?;
                let n = range.len() as f64;
                let mean: [f64; 2] = std::array::from_fn(|c| {
                    frames[range.clone()]
                        .iter()
                        .map(|f| f[c] as f64)
                        .sum::<f64>()
                        / n
                });
                for frame in &mut frames[range] {
                    for c in 0..2 {
                        frame[c] -= mean[c] as f32;
                    }
                }
            }
            Operation::Invert { start, end } => frames[span(len, start, end)?]
                .iter_mut()
                .flatten()
                .for_each(|v| *v = -*v),
            Operation::Mono => {
                for frame in &mut frames {
                    let m = (frame[0] + frame[1]) * 0.5;
                    *frame = [m; 2];
                }
            }
            Operation::Swap => frames.iter_mut().for_each(|f| f.swap(0, 1)),
            Operation::Pitch {
                semitones,
                keep_length,
            } => {
                let ratio = 2.0_f64.powf(semitones.clamp(-48.0, 48.0) as f64 / 12.0);
                let source = if keep_length {
                    stretch(&frames, rate, ratio)
                } else {
                    frames
                };
                frames = varispeed(&source, rate, ratio)?;
            }
            Operation::Stretch { factor } => {
                frames = stretch(&frames, rate, (factor as f64).clamp(0.125, 8.0));
            }
            Operation::Resample { rate: target } => {
                if !(8_000..=384_000).contains(&target) {
                    return Err("Choose a rate between 8 and 384 kHz".into());
                }
                frames = varispeed(&frames, rate, rate as f64 / target as f64)?;
                rate = target;
            }
            Operation::Crush { bits, rate: hold } => {
                let steps = 2.0_f32.powi(bits.clamp(1, 24) as i32 - 1);
                let every = (rate as f32 / hold.clamp(100.0, rate as f32)).max(1.0);
                let mut held = [0.0; 2];
                let mut phase = every;
                for frame in &mut frames {
                    phase += 1.0;
                    if phase >= every {
                        phase -= every;
                        held = frame.map(|v| (v * steps).round() / steps);
                    }
                    *frame = held;
                }
            }
            Operation::TrimSilence { threshold } => {
                let floor = 10.0_f32.powf(threshold.clamp(-120.0, 0.0) / 20.0);
                let loud = |f: &Stereo| f[0].abs().max(f[1].abs()) > floor;
                let first = frames.iter().position(loud);
                let last = frames.iter().rposition(loud);
                if let (Some(first), Some(last)) = (first, last) {
                    frames = frames[first..=last].to_vec();
                } else {
                    return Err("The whole sample is below the threshold".into());
                }
            }
        }
        if frames.is_empty() {
            return Err("The edit would leave no audio".into());
        }
        Ok(Audio { rate, frames })
    }
    /// Inserts `clip` at `at`, or mixes it in from there, extending the
    /// sample if it runs past the end. A clip at another rate is converted.
    pub fn paste(&self, clip: &Audio, at: u32, mix: bool) -> Result<Audio, String> {
        let clip = if clip.rate == self.rate {
            clip.frames.clone()
        } else {
            varispeed(&clip.frames, clip.rate, clip.rate as f64 / self.rate as f64)?
        };
        let at = (at as usize).min(self.frames.len());
        let mut frames = self.frames.clone();
        if mix {
            if frames.len() < at + clip.len() {
                frames.resize(at + clip.len(), [0.0; 2]);
            }
            for (to, from) in frames[at..].iter_mut().zip(&clip) {
                to[0] += from[0];
                to[1] += from[1];
            }
        } else {
            frames.splice(at..at, clip);
        }
        Ok(Audio {
            rate: self.rate,
            frames,
        })
    }
    pub fn peak(&self) -> f32 {
        self.frames
            .iter()
            .flatten()
            .fold(0.0_f32, |m, v| m.max(v.abs()))
    }
}

/// Reads `frames` at `ratio` frames per output frame, from antialiasing mips.
fn varispeed(frames: &[Stereo], rate: u32, ratio: f64) -> Result<Vec<Stereo>, String> {
    let sample = SampleBuffer::new(String::new(), rate, frames.to_vec())?;
    let count = ((frames.len() as f64 / ratio).round() as usize).max(1);
    Ok((0..count)
        .map(|i| {
            let position = i as f64 * ratio;
            // The last frame does not wrap around to the first.
            if position > (frames.len() - 1) as f64 {
                frames[frames.len() - 1]
            } else {
                sample.read(position, ratio, true)
            }
        })
        .collect())
}

/// WSOLA: overlapping windows of the input, each shifted by up to a quarter
/// window to the position that best continues the previous one, so the pitch
/// stays while the length changes by `factor`.
pub fn stretch(frames: &[Stereo], rate: u32, factor: f64) -> Vec<Stereo> {
    if frames.len() < 16 || (factor - 1.0).abs() < 1e-6 {
        return frames.to_vec();
    }
    let window = ((rate as f64 * 0.04) as usize).clamp(64, 8192) & !1;
    let hop = window / 2;
    let tolerance = window / 4;
    let input_hop = hop as f64 / factor;
    let length = (frames.len() as f64 * factor).round() as usize;
    let mut out = vec![[0.0_f32; 2]; length + window];
    let mut weight = vec![0.0_f32; length + window];
    let hann: Vec<f32> = (0..window)
        .map(|i| 0.5 - 0.5 * (std::f32::consts::TAU * i as f32 / window as f32).cos())
        .collect();
    let mono = |i: usize| frames.get(i).map_or(0.0, |f| f[0] + f[1]);
    let mut previous = 0_usize;
    let mut k = 0;
    while k * hop < length {
        let nominal = (k as f64 * input_hop) as isize;
        let chosen = if k == 0 {
            0
        } else {
            // Compare candidates with what naturally follows the last window.
            let natural = previous + hop;
            let lo = (nominal - tolerance as isize).max(0) as usize;
            let hi = ((nominal + tolerance as isize).max(0) as usize)
                .min(frames.len().saturating_sub(1));
            let mut best = (f32::MIN, lo.min(hi));
            let mut candidate = lo;
            while candidate <= hi {
                let mut score = 0.0;
                for i in (0..hop).step_by(2) {
                    score += mono(natural + i) * mono(candidate + i);
                }
                if score > best.0 {
                    best = (score, candidate);
                }
                candidate += 2;
            }
            best.1
        };
        let at = k * hop;
        for i in 0..window {
            if let Some(frame) = frames.get(chosen + i) {
                let w = hann[i];
                out[at + i][0] += frame[0] * w;
                out[at + i][1] += frame[1] * w;
            }
            weight[at + i] += hann[i];
        }
        previous = chosen;
        k += 1;
    }
    out.truncate(length.max(1));
    for (frame, w) in out.iter_mut().zip(weight) {
        if w > 1e-3 {
            frame[0] /= w;
            frame[1] /= w;
        }
    }
    out
}

fn mono(frames: &[Stereo]) -> Vec<f32> {
    frames.iter().map(|f| (f[0] + f[1]) * 0.5).collect()
}

/// The nearest frame within `radius` of `at` where the signal crosses zero.
pub fn zero_crossing(frames: &[Stereo], at: usize, radius: usize) -> usize {
    let signal = |i: usize| (frames[i][0] + frames[i][1]) * 0.5;
    let crosses =
        |i: usize| i > 0 && i < frames.len() && (signal(i - 1) <= 0.0) != (signal(i) <= 0.0);
    for d in 0..=radius {
        if at >= d && crosses(at - d) {
            return at - d;
        }
        if crosses(at + d) {
            return at + d;
        }
    }
    at.min(frames.len().saturating_sub(1))
}

/// Slice starts at transients. `sensitivity` 0–1 finds more as it rises;
/// slices are at least `gap_ms` apart. The first slice starts at 0.
pub fn transients(frames: &[Stereo], rate: u32, sensitivity: f32, gap_ms: f32) -> Vec<u32> {
    let signal = mono(frames);
    let hop = (rate as usize / 200).max(16);
    // A first difference favours the clicks of attacks over sustained lows.
    let energies: Vec<f32> = signal
        .chunks(hop)
        .enumerate()
        .map(|(k, chunk)| {
            let base = k * hop;
            chunk
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    let previous = if base + i == 0 {
                        0.0
                    } else {
                        signal[base + i - 1]
                    };
                    let d = v - previous;
                    d * d + v * v * 0.1
                })
                .sum::<f32>()
                / chunk.len() as f32
        })
        .collect();
    let loudest = energies.iter().fold(0.0_f32, |m, e| m.max(*e));
    if loudest <= 0.0 {
        return vec![0];
    }
    let logs: Vec<f32> = energies.iter().map(|e| (e + loudest * 1e-7).ln()).collect();
    let threshold = 3.0 - 2.6 * sensitivity.clamp(0.0, 1.0);
    let floor = loudest * 10.0_f32.powf((-50.0 + 30.0 * (1.0 - sensitivity)) / 10.0);
    let gap = ((gap_ms.max(5.0) * 0.001 * rate as f32) as usize / hop).max(1);
    let strength: Vec<f32> = (0..logs.len())
        .map(|k| {
            let from = k.saturating_sub(4);
            if k == from {
                return 0.0;
            }
            let mean = logs[from..k].iter().sum::<f32>() / (k - from) as f32;
            (logs[k] - mean).max(0.0)
        })
        .collect();
    let mut slices = vec![0_u32];
    let mut last = 0_usize;
    for k in 1..strength.len() {
        let peak = strength[k] >= strength[k - 1]
            && strength.get(k + 1).is_none_or(|next| strength[k] >= *next);
        if peak && strength[k] > threshold && energies[k] > floor && k - last >= gap {
            // Back off to the quiet just before the attack, at a zero crossing.
            let at = (k * hop).saturating_sub(hop / 2);
            let at = zero_crossing(frames, at, hop / 2);
            if at as u32 > *slices.last().unwrap() {
                slices.push(at as u32);
                last = k;
            }
        }
    }
    slices
}

/// `count` equal slices.
pub fn divide(length: usize, count: usize) -> Vec<u32> {
    let count = count.clamp(1, crate::sampler::MAX_PADS);
    (0..count).map(|i| (i * length / count) as u32).collect()
}

/// Slices of `beats` beats each at `bpm`.
pub fn beats(length: usize, rate: u32, bpm: f32, beats: f32) -> Vec<u32> {
    let every =
        rate as f64 * 60.0 / bpm.clamp(20.0, 999.0) as f64 * beats.clamp(0.0625, 64.0) as f64;
    let mut slices = Vec::new();
    let mut at = 0.0_f64;
    while (at as usize) < length && slices.len() < crate::sampler::MAX_PADS {
        slices.push(at.round() as u32);
        at += every;
    }
    slices
}

/// The note of the sample's pitch (MIDI, fractional), by YIN on its loudest
/// stretch after the attack; `None` for unpitched audio.
pub fn root(frames: &[Stereo], rate: u32) -> Option<f32> {
    let signal = mono(frames);
    let window = ((rate as usize) / 12).clamp(1024, 8192);
    if signal.len() < window * 2 {
        return analyse(&signal, rate);
    }
    let skip = (rate as usize / 50).min(signal.len() - window * 2);
    let (mut best, mut loudest) = (skip, 0.0);
    let mut at = skip;
    while at + window * 2 <= signal.len() {
        let energy: f32 = signal[at..at + window].iter().map(|v| v * v).sum();
        if energy > loudest {
            loudest = energy;
            best = at;
        }
        at += window / 2;
    }
    analyse(&signal[best..best + window * 2], rate)
}

fn analyse(signal: &[f32], rate: u32) -> Option<f32> {
    let min_lag = (rate as usize / 2000).max(2);
    let max_lag = (rate as usize / 40).min(signal.len() / 2);
    if max_lag <= min_lag + 2 {
        return None;
    }
    let n = signal.len() - max_lag;
    let mut diff = vec![0.0_f32; max_lag + 1];
    for (lag, d) in diff.iter_mut().enumerate().skip(1) {
        *d = (0..n).map(|i| (signal[i] - signal[i + lag]).powi(2)).sum();
    }
    // Cumulative mean normalized difference.
    let mut cmnd = vec![1.0_f32; max_lag + 1];
    let mut sum = 0.0;
    for lag in 1..=max_lag {
        sum += diff[lag];
        cmnd[lag] = if sum > 0.0 {
            diff[lag] * lag as f32 / sum
        } else {
            1.0
        };
    }
    let mut lag = (min_lag..max_lag).find(|&l| cmnd[l] < 0.15 && cmnd[l] <= cmnd[l + 1]);
    if lag.is_none() {
        let (l, v) = (min_lag..max_lag)
            .map(|l| (l, cmnd[l]))
            .min_by(|a, b| a.1.total_cmp(&b.1))?;
        if v > 0.35 {
            return None;
        }
        lag = Some(l);
    }
    let l = lag?;
    let (a, b, c) = (cmnd[l - 1], cmnd[l], cmnd[l + 1]);
    let shift = if a - 2.0 * b + c != 0.0 {
        0.5 * (a - c) / (a - 2.0 * b + c)
    } else {
        0.0
    };
    let frequency = rate as f32 / (l as f32 + shift.clamp(-1.0, 1.0));
    Some(69.0 + 12.0 * (frequency / 440.0).log2())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn tone(rate: u32, hz: f32, seconds: f32) -> Audio {
        Audio::new(
            rate,
            (0..(rate as f32 * seconds) as usize)
                .map(|i| [(std::f32::consts::TAU * hz * i as f32 / rate as f32).sin() * 0.5; 2])
                .collect(),
        )
    }
    #[test]
    fn region_edits_change_only_their_region() {
        let audio = Audio::new(8_000, (0..100).map(|i| [i as f32 / 100.0; 2]).collect());
        let crop = audio.apply(Operation::Crop { start: 10, end: 20 }).unwrap();
        assert_eq!(crop.frames.len(), 10);
        assert_eq!(crop.frames[0], [0.1; 2]);
        let cut = audio
            .apply(Operation::Delete { start: 10, end: 20 })
            .unwrap();
        assert_eq!(cut.frames.len(), 90);
        assert_eq!(cut.frames[10], [0.2; 2]);
        let quiet = audio
            .apply(Operation::Silence { start: 0, end: 50 })
            .unwrap();
        assert_eq!(quiet.frames[49], [0.0; 2]);
        assert_eq!(quiet.frames[50], [0.5; 2]);
        let back = audio
            .apply(Operation::Reverse { start: 0, end: 0 })
            .unwrap();
        assert_eq!(back.frames[0], [0.99; 2]);
        let loud = audio
            .apply(Operation::Normalize {
                start: 0,
                end: 0,
                peak: 0.0,
            })
            .unwrap();
        assert!((loud.peak() - 1.0).abs() < 1e-6);
        let longer = audio
            .apply(Operation::InsertSilence { at: 50, ms: 1.0 })
            .unwrap();
        assert_eq!(longer.frames.len(), 108);
        assert!(audio.apply(Operation::Crop { start: 50, end: 50 }).is_err());
    }
    #[test]
    fn fades_reach_silence_at_their_edge() {
        let audio = Audio::new(8_000, vec![[1.0; 2]; 101]);
        let fade = audio
            .apply(Operation::FadeOut {
                start: 0,
                end: 0,
                curve: 0.0,
            })
            .unwrap();
        assert_eq!(fade.frames[0], [1.0; 2]);
        assert!((fade.frames[50][0] - 0.5).abs() < 1e-6);
        assert_eq!(fade.frames[100], [0.0; 2]);
        let slow = audio
            .apply(Operation::FadeIn {
                start: 0,
                end: 0,
                curve: 1.0,
            })
            .unwrap();
        assert!(slow.frames[50][0] < 0.1);
    }
    #[test]
    fn varispeed_halves_length_an_octave_up() {
        let audio = tone(48_000, 440.0, 1.0);
        let up = audio
            .apply(Operation::Pitch {
                semitones: 12.0,
                keep_length: false,
            })
            .unwrap();
        assert_eq!(up.frames.len(), 24_000);
        let note = root(&up.frames, 48_000).unwrap();
        assert!((note - 81.0).abs() < 0.2, "{note}");
    }
    #[test]
    fn stretching_keeps_the_pitch() {
        let audio = tone(48_000, 220.0, 1.0);
        let long = audio.apply(Operation::Stretch { factor: 1.5 }).unwrap();
        assert_eq!(long.frames.len(), 72_000);
        let note = root(&long.frames, 48_000).unwrap();
        assert!((note - 57.0).abs() < 0.3, "{note}");
        let shifted = audio
            .apply(Operation::Pitch {
                semitones: 7.0,
                keep_length: true,
            })
            .unwrap();
        assert!((shifted.frames.len() as i64 - 48_000).abs() < 4);
        let note = root(&shifted.frames, 48_000).unwrap();
        assert!((note - 64.0).abs() < 0.3, "{note}");
    }
    #[test]
    fn resampling_keeps_duration() {
        let audio = tone(48_000, 1_000.0, 0.5);
        let low = audio.apply(Operation::Resample { rate: 22_050 }).unwrap();
        assert_eq!(low.rate, 22_050);
        assert_eq!(low.frames.len(), 11_025);
        let note = root(&low.frames, 22_050).unwrap();
        assert!((note - root(&audio.frames, 48_000).unwrap()).abs() < 0.1);
    }
    #[test]
    fn transients_find_each_hit() {
        let rate = 48_000;
        let mut frames = vec![[0.0; 2]; rate as usize * 2];
        for hit in [0.25, 0.75, 1.5] {
            let at = (hit * rate as f32) as usize;
            for i in 0..2_000 {
                let v = (i as f32 * 0.3).sin() * (-(i as f32) / 400.0).exp();
                frames[at + i] = [v; 2];
            }
        }
        let slices = transients(&frames, rate, 0.5, 50.0);
        assert_eq!(slices.len(), 4, "{slices:?}");
        for (slice, hit) in slices[1..].iter().zip([0.25, 0.75, 1.5]) {
            let at = hit * rate as f32;
            assert!((*slice as f32 - at).abs() < 300.0, "{slice} {at}");
        }
    }
    #[test]
    fn grids_and_divisions() {
        assert_eq!(divide(100, 4), vec![0, 25, 50, 75]);
        assert_eq!(
            beats(96_000, 48_000, 120.0, 1.0),
            vec![0, 24_000, 48_000, 72_000]
        );
    }
    #[test]
    fn loop_crossfade_ends_where_the_loop_starts() {
        let audio = Audio::new(1_000, (0..1_000).map(|i| [i as f32; 2]).collect());
        let faded = audio
            .apply(Operation::LoopCrossfade {
                loop_start: 500,
                end: 0,
                ms: 100.0,
            })
            .unwrap();
        assert_eq!(faded.frames[999], audio.frames[499]);
        assert_eq!(faded.frames[800], audio.frames[800]);
    }
    #[test]
    fn paste_inserts_or_mixes() {
        let audio = Audio::new(8_000, vec![[1.0; 2]; 10]);
        let clip = Audio::new(8_000, vec![[0.5; 2]; 4]);
        assert_eq!(audio.paste(&clip, 2, false).unwrap().frames.len(), 14);
        let mixed = audio.paste(&clip, 8, true).unwrap();
        assert_eq!(mixed.frames.len(), 12);
        assert_eq!(mixed.frames[8], [1.5; 2]);
        assert_eq!(mixed.frames[11], [0.5; 2]);
    }
}
