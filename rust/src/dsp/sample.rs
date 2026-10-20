use crate::{Stereo, tables::Random};
use std::f64::consts::PI;

/// Normalized, non-destructive playback/trim boundaries. End is exclusive.
#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Region {
    pub start: f64,
    pub end: f64,
}
impl Default for Region {
    fn default() -> Self {
        Self {
            start: 0.0,
            end: 1.0,
        }
    }
}
impl Region {
    pub fn bounds(self, length: usize) -> Result<std::ops::Range<usize>, String> {
        if !self.start.is_finite()
            || !self.end.is_finite()
            || self.start < 0.0
            || self.end > 1.0
            || self.start >= self.end
        {
            return Err("Choose a start before the end, within the audio file".into());
        }
        let start = (self.start * length as f64).round() as usize;
        let end = (self.end * length as f64).round() as usize;
        if start >= end || end > length {
            return Err("Select at least one audio frame".into());
        }
        Ok(start..end)
    }
}

#[derive(Clone, Copy, Debug, Default, serde::Deserialize, serde::Serialize, PartialEq)]
#[serde(default)]
pub struct Fades {
    pub fade_in: f64,
    pub fade_out: f64,
}
impl Fades {
    pub fn validate(self) -> Result<Self, String> {
        if [self.fade_in, self.fade_out]
            .iter()
            .any(|x| !x.is_finite() || !(0.0..=1800.0).contains(x))
        {
            return Err("Fade times must be between 0 and 1800 seconds".into());
        }
        Ok(self)
    }
    pub fn gain(self, position: f64, sample_rate: u32, range: &std::ops::Range<usize>) -> f32 {
        if self == Self::default() {
            return 1.0;
        }
        let length = range.len().saturating_sub(1) as f64;
        let p = (position - range.start as f64).rem_euclid(range.len() as f64);
        let ramp = |distance: f64, seconds: f64| {
            if seconds == 0.0 {
                return 1.0;
            }
            let frames = (seconds * sample_rate as f64)
                .round()
                .max(1.0)
                .min(length.max(1.0));
            let x = (distance / frames).clamp(0.0, 1.0);
            x * x * (3.0 - 2.0 * x)
        };
        (ramp(p, self.fade_in) * ramp(length - p, self.fade_out)) as f32
    }
}

#[derive(Debug)]
pub struct SampleBuffer {
    pub name: String,
    pub library_id: Option<String>,
    pub sample_rate: u32,
    levels: Vec<Vec<Stereo>>,
    active_levels: usize,
    pub waveform: Vec<[f32; 2]>,
}

/// Eight seconds of input history, allocated once when the input is enabled.
/// The mip levels are updated incrementally rather than rebuilding a sample on
/// the audio callback. The original file remains available in Engine::sample.
pub struct LiveInput {
    pub sample: SampleBuffer,
    pub effects_only: bool,
    pub peak: Stereo,
    write: usize,
    captured: usize,
    lowpass: [[Stereo; 2]; 6],
}

impl LiveInput {
    pub fn new(sample_rate: u32, effects_only: bool) -> Self {
        let mut length = sample_rate as usize * 8;
        let mut levels = Vec::with_capacity(7);
        for _ in 0..7 {
            levels.push(vec![[0.0; 2]; length]);
            length = length.div_ceil(2);
        }
        Self {
            sample: SampleBuffer::from_prepared("Microphone".into(), None, sample_rate, levels)
                .expect("valid input buffer"),
            effects_only,
            peak: [0.0; 2],
            write: 0,
            captured: 0,
            lowpass: [[[0.0; 2]; 2]; 6],
        }
    }

    pub fn push(&mut self, frame: Stereo, frozen: bool) {
        self.push_with_meter(frame, frame, frozen);
    }
    pub fn push_with_meter(&mut self, frame: Stereo, meter: Stereo, frozen: bool) {
        let mut frame = frame.map(|x| {
            if x.is_finite() {
                x.clamp(-16.0, 16.0)
            } else {
                0.0
            }
        });
        for (peak, value) in self.peak.iter_mut().zip(meter) {
            if value.is_finite() {
                *peak = peak.max(value.clamp(-16.0, 16.0).abs());
            }
        }
        if frozen {
            return;
        }
        let index = self.write;
        self.sample.levels[0][index] = frame;
        let bin = index * self.sample.waveform.len() / self.sample.len();
        let value = (frame[0] + frame[1]) * 0.5;
        if index == 0 || (index - 1) * self.sample.waveform.len() / self.sample.len() != bin {
            self.sample.waveform[bin] = [value; 2];
        } else {
            self.sample.waveform[bin][0] = self.sample.waveform[bin][0].min(value);
            self.sample.waveform[bin][1] = self.sample.waveform[bin][1].max(value);
        }
        let mut parent_index = index;
        for (level, filter) in self.lowpass.iter_mut().enumerate() {
            for (c, value) in frame.iter_mut().enumerate() {
                filter[0][c] += 0.6 * (*value - filter[0][c]);
                filter[1][c] += 0.6 * (filter[0][c] - filter[1][c]);
                *value = filter[1][c];
            }
            if !parent_index.is_multiple_of(2) {
                break;
            }
            parent_index /= 2;
            self.sample.levels[level + 1][parent_index] = frame;
        }
        self.write = (index + 1) % self.sample.len();
        self.captured = (self.captured + 1).min(self.sample.len());
    }

    pub fn head(&self) -> usize {
        self.write
    }
    pub fn seconds(&self) -> f64 {
        self.captured as f64 / self.sample.sample_rate as f64
    }

    /// Return only captured frames, oldest first, including after a ring wrap.
    pub fn capture(&self) -> Vec<Stereo> {
        let start = if self.captured == self.sample.len() {
            self.write
        } else {
            0
        };
        (0..self.captured)
            .map(|i| self.sample.frames()[(start + i) % self.sample.len()])
            .collect()
    }
}

impl SampleBuffer {
    /// Allocate a prepared import on a control-message boundary. PCM and mips
    /// can then be copied in bounded chunks between audio blocks.
    pub fn blank_prepared(
        name: String,
        sample_rate: u32,
        lengths: &[usize],
    ) -> Result<Self, String> {
        if !(8_000..=384_000).contains(&sample_rate)
            || lengths.is_empty()
            || lengths.len() > 7
            || lengths[0] == 0
            || lengths[0] > sample_rate as usize * 1800
            || lengths.windows(2).any(|p| p[1] != p[0].div_ceil(2))
        {
            return Err("Invalid prepared source".into());
        }
        Ok(Self {
            name,
            library_id: None,
            sample_rate,
            levels: lengths.iter().map(|n| Vec::with_capacity(*n)).collect(),
            active_levels: lengths.len(),
            waveform: vec![[0.0; 2]; lengths[0].min(640)],
        })
    }
    pub fn reset_prepared(&mut self, lengths: &[usize]) -> Result<(), &'static str> {
        if lengths.is_empty()
            || lengths.len() > self.levels.len()
            || lengths[0] == 0
            || lengths.windows(2).any(|p| p[1] != p[0].div_ceil(2))
            || lengths
                .iter()
                .zip(&self.levels)
                .any(|(n, l)| *n > l.capacity())
        {
            return Err("Prepared source exceeds reserved capacity");
        }
        for level in &mut self.levels {
            level.clear();
        }
        self.active_levels = lengths.len();
        self.waveform.resize(lengths[0].min(640), [0.0; 2]);
        Ok(())
    }
    pub fn write_prepared(
        &mut self,
        level: usize,
        offset: usize,
        frames: &[Stereo],
        total_frames: usize,
    ) -> Result<(), &'static str> {
        if level >= self.active_levels
            || offset != self.levels[level].len()
            || frames.len() > self.levels[level].capacity() - offset
            || frames
                .iter()
                .flatten()
                .any(|v| !v.is_finite() || v.abs() > 16.0)
        {
            return Err("Invalid prepared PCM");
        }
        self.levels[level].extend_from_slice(frames);
        if level == 0 {
            for (i, frame) in frames.iter().enumerate() {
                let index = offset + i;
                let bin = ((index + 1) * self.waveform.len() - 1) / total_frames;
                let value = (frame[0] + frame[1]) * 0.5;
                if index == 0 || (index * self.waveform.len() - 1) / total_frames != bin {
                    self.waveform[bin] = [1.0_f32.min(value), (-1.0_f32).max(value)];
                } else {
                    self.waveform[bin][0] = self.waveform[bin][0].min(value);
                    self.waveform[bin][1] = self.waveform[bin][1].max(value);
                }
            }
        }
        Ok(())
    }
    pub fn new(name: String, sample_rate: u32, frames: Vec<Stereo>) -> Result<Self, String> {
        Self::new_cancellable(name, sample_rate, frames, &|| false)
    }
    pub(crate) fn new_cancellable(
        name: String,
        sample_rate: u32,
        mut frames: Vec<Stereo>,
        cancelled: &impl Fn() -> bool,
    ) -> Result<Self, String> {
        if !(8_000..=384_000).contains(&sample_rate) {
            return Err("Unsupported sample rate (8–384 kHz)".into());
        }
        if frames.is_empty() {
            return Err("The audio file is empty".into());
        }
        for frame in &mut frames {
            for value in frame {
                *value = if value.is_finite() {
                    value.clamp(-16.0, 16.0)
                } else {
                    0.0
                };
            }
        }
        let bins = frames.len().min(640);
        let waveform = (0..bins)
            .map(|i| {
                let start = i * frames.len() / bins;
                let end = ((i + 1) * frames.len() / bins).max(start + 1);
                frames[start..end].iter().fold([1.0_f32, -1.0_f32], |m, x| {
                    let value = (x[0] + x[1]) * 0.5;
                    [m[0].min(value), m[1].max(value)]
                })
            })
            .collect();
        let mut levels = vec![frames];
        // Windowed-sinc lowpass before each 2:1 decimation. Constructed on load,
        // never on the audio callback. This replaces the unresolved NEON FIR.
        let mut kernel = [0.0_f64; 33];
        for (i, k) in kernel.iter_mut().enumerate() {
            let x = i as f64 - 16.0;
            let sinc = if x == 0.0 {
                0.45
            } else {
                (PI * 0.45 * x).sin() / (PI * x)
            };
            let window = 0.42 - 0.5 * (2.0 * PI * i as f64 / 32.0).cos()
                + 0.08 * (4.0 * PI * i as f64 / 32.0).cos();
            *k = sinc * window;
        }
        let sum: f64 = kernel.iter().sum();
        kernel.iter_mut().for_each(|x| *x /= sum);
        while levels.len() < 7 && levels.last().unwrap().len() >= 64 {
            let source = levels.last().unwrap();
            let len = source.len() as i64;
            let mut reduced = Vec::with_capacity(source.len().div_ceil(2));
            for center in (0..source.len()).step_by(2) {
                if center % 2048 == 0 && cancelled() {
                    return Err("Source FX preparation superseded".into());
                }
                let mut frame = [0.0_f64; 2];
                for (tap, &k) in kernel.iter().enumerate() {
                    let i = (center as i64 + tap as i64 - 16).rem_euclid(len) as usize;
                    for (c, value) in frame.iter_mut().enumerate() {
                        *value += source[i][c] as f64 * k;
                    }
                }
                reduced.push([frame[0] as f32, frame[1] as f32]);
            }
            levels.push(reduced);
        }
        Ok(Self {
            name,
            library_id: None,
            sample_rate,
            active_levels: levels.len(),
            levels,
            waveform,
        })
    }
    pub fn len(&self) -> usize {
        self.levels[0].len()
    }
    pub fn is_empty(&self) -> bool {
        self.levels[0].is_empty()
    }
    pub fn duration(&self) -> f64 {
        self.len() as f64 / self.sample_rate as f64
    }

    pub fn frames(&self) -> &[Stereo] {
        &self.levels[0]
    }

    /// Transfer already prepared antialiasing levels from a Web Worker to the
    /// audio worklet, so loading a sample never runs the FIR on the audio thread.
    pub fn prepared_levels(&self) -> &[Vec<Stereo>] {
        &self.levels[..self.active_levels]
    }

    pub fn from_prepared(
        name: String,
        library_id: Option<String>,
        sample_rate: u32,
        levels: Vec<Vec<Stereo>>,
    ) -> Result<Self, String> {
        if !(8_000..=384_000).contains(&sample_rate)
            || levels.is_empty()
            || levels.len() > 7
            || levels[0].is_empty()
            || levels[0].len() > sample_rate as usize * 30 * 60
            || levels
                .windows(2)
                .any(|l| l[1].len() != l[0].len().div_ceil(2))
            || levels
                .iter()
                .flatten()
                .flatten()
                .any(|x| !x.is_finite() || x.abs() > 16.0)
        {
            return Err("Invalid prepared sample".into());
        }
        let frames = &levels[0];
        let bins = frames.len().min(640);
        let waveform = (0..bins)
            .map(|i| {
                let start = i * frames.len() / bins;
                let end = ((i + 1) * frames.len() / bins).max(start + 1);
                frames[start..end].iter().fold([1.0_f32, -1.0_f32], |m, x| {
                    let value = (x[0] + x[1]) * 0.5;
                    [m[0].min(value), m[1].max(value)]
                })
            })
            .collect();
        Ok(Self {
            name,
            library_id,
            sample_rate,
            active_levels: levels.len(),
            levels,
            waveform,
        })
    }

    pub fn read(&self, position: f64, rate: f64, antialiasing: bool) -> Stereo {
        self.read_level(position, self.level_for_rate(rate, antialiasing))
    }

    pub(crate) fn level_for_rate(&self, rate: f64, antialiasing: bool) -> usize {
        if antialiasing && rate.abs() >= 2.0 {
            (rate.abs().log2().floor() as usize).min(self.active_levels - 1)
        } else {
            0
        }
    }

    fn read_level(&self, position: f64, level: usize) -> Stereo {
        if !position.is_finite() {
            return [0.0; 2];
        }
        let data = &self.levels[level];
        // Keep indices in f64: f32 loses sub-sample precision on long recordings.
        let p = position.rem_euclid(self.len() as f64) / (1_usize << level) as f64;
        let base = p.floor() as i64;
        let fraction = (p - p.floor()) as f32;
        let [a, b, c, d] = interpolation_frames(data, base, &(0..data.len()));
        std::array::from_fn(|i| {
            let c1 = 0.5 * (c[i] - a[i]);
            let c2 = a[i] - 2.5 * b[i] + 2.0 * c[i] - 0.5 * d[i];
            let c3 = 0.5 * (d[i] - a[i]) + 1.5 * (b[i] - c[i]);
            ((c3 * fraction + c2) * fraction + c1) * fraction + b[i]
        })
    }

    /// Wrap interpolation inside the selection, including backwards grains.
    /// A mip is safe only when its entire FIR footprint is inside the region;
    /// near a cut we fall back to finer levels so excluded audio cannot leak in.
    pub fn read_region(
        &self,
        position: f64,
        rate: f64,
        antialiasing: bool,
        range: &std::ops::Range<usize>,
    ) -> Stereo {
        self.read_region_level(position, self.level_for_rate(rate, antialiasing), range)
    }

    pub(crate) fn read_region_level(
        &self,
        position: f64,
        mut level: usize,
        range: &std::ops::Range<usize>,
    ) -> Stereo {
        level = level.min(self.active_levels - 1);
        if range.start == 0 && range.end == self.len() {
            return self.read_level(position, level);
        }
        if !position.is_finite() {
            return [0.0; 2];
        }
        let p = range.start as f64 + (position - range.start as f64).rem_euclid(range.len() as f64);
        while level > 0 {
            let scale = (1_usize << level) as f64;
            let base = (p / scale).floor();
            let support = 16.0 * (scale - 1.0);
            if (base - 1.0) * scale - support >= range.start as f64
                && (base + 2.0) * scale + support < range.end as f64
            {
                return self.read_level(p, level);
            }
            level -= 1;
        }
        let base = p.floor() as i64;
        let fraction = (p - p.floor()) as f32;
        let [a, b, c, d] = interpolation_frames(self.frames(), base, range);
        std::array::from_fn(|i| {
            let c1 = 0.5 * (c[i] - a[i]);
            let c2 = a[i] - 2.5 * b[i] + 2.0 * c[i] - 0.5 * d[i];
            let c3 = 0.5 * (d[i] - a[i]) + 1.5 * (b[i] - c[i]);
            ((c3 * fraction + c2) * fraction + c1) * fraction + b[i]
        })
    }

    pub fn trim_wav(&self, region: Region) -> Result<Vec<u8>, String> {
        self.trim_wav_with_fades(region, Fades::default())
    }
    pub fn trim_wav_with_fades(&self, region: Region, fades: Fades) -> Result<Vec<u8>, String> {
        let fades = fades.validate()?;
        let range = region.bounds(self.len())?;
        if fades == Fades::default() {
            return crate::wav::bytes(self.sample_rate, &self.frames()[range]);
        }
        let frames: Vec<_> = range
            .clone()
            .map(|i| self.frames()[i].map(|v| v * fades.gain(i as f64, self.sample_rate, &range)))
            .collect();
        crate::wav::bytes(self.sample_rate, &frames)
    }

    pub fn demo(sample_rate: u32) -> Self {
        let mut random = Random::default();
        let notes = [
            130.8128_f64,
            195.9977,
            164.8138,
            246.9417,
            261.6256,
            164.8138,
            195.9977,
            329.6276,
        ];
        let frames = (0..sample_rate as usize * 8)
            .map(|i| {
                let t = i as f64 / sample_rate as f64;
                let step = (t * 2.0) as usize;
                let age = t - step as f64 * 0.5;
                let frequency = notes[step % notes.len()];
                let pluck = (1.0 - (-age * 500.0).exp()) * (-age * 5.0).exp();
                let phase = t * frequency * 2.0 * PI;
                let tone = (phase.sin() + 0.3 * (phase * 2.001).sin() + 0.12 * (phase * 3.0).sin())
                    * pluck
                    * 0.28;
                let pad =
                    (t * 65.4064 * 2.0 * PI).sin() * 0.06 + (t * 97.9989 * 2.0 * PI).sin() * 0.04;
                let tick = random.bipolar() as f64 * (-age * 100.0).exp() * 0.06;
                [
                    (tone + pad + tick) as f32,
                    (tone * (0.88 + 0.12 * (t * 0.8).sin()) + pad - tick) as f32,
                ]
            })
            .collect();
        Self::new("Glass sequence".into(), sample_rate, frames).expect("valid built-in sample")
    }
}

#[inline]
fn interpolation_frames(data: &[Stereo], base: i64, range: &std::ops::Range<usize>) -> [Stereo; 4] {
    // Most grain reads are inside the region. Boundary reads retain the exact
    // wrap used for tiny selections, negative positions and reverse grains.
    if base > range.start as i64 && base + 2 < range.end as i64 {
        let i = base as usize;
        [data[i - 1], data[i], data[i + 1], data[i + 2]]
    } else {
        std::array::from_fn(|i| {
            data[range.start
                + (base + i as i64 - 1 - range.start as i64).rem_euclid(range.len() as i64)
                    as usize]
        })
    }
}
