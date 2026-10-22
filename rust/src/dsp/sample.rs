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
        let p = wrap_position(position - range.start as f64, range.len() as f64);
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

/// Input history allocated once when the input is enabled. Drum captures reserve
/// 32 seconds, with a shorter read window that can change without heap work.
/// The mip levels are updated incrementally rather than rebuilding a sample on
/// the audio callback. The original file remains available in Engine::sample.
pub struct LiveInput {
    pub sample: SampleBuffer,
    pub effects_only: bool,
    pub peak: Stereo,
    write: usize,
    captured: usize,
    total: u64,
    window: usize,
    waveform_rebuild: Option<(u64, u64)>,
    lowpass: [[Stereo; 2]; 6],
}

impl LiveInput {
    pub fn new(sample_rate: u32, effects_only: bool) -> Self {
        Self::with_capacity(sample_rate, effects_only, 8, 8.0)
    }
    /// A Bus or Master input history, with a window of at most eight seconds.
    pub fn for_bus(sample_rate: u32, seconds: f32) -> Self {
        let mut input = Self::with_capacity(
            sample_rate,
            false,
            crate::bus::MAX_BUFFER_SECONDS as usize,
            seconds,
        );
        input.sample.name = "Bus input".into();
        input
    }
    pub fn for_drums(sample_rate: u32, effects_only: bool, seconds: f32) -> Self {
        Self::with_capacity(sample_rate, effects_only, 32, seconds)
    }
    fn with_capacity(sample_rate: u32, effects_only: bool, seconds: usize, window: f32) -> Self {
        let capacity = sample_rate as usize * seconds;
        let mut length = capacity;
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
            total: capacity as u64,
            window: ((window.clamp(0.125, seconds as f32) * sample_rate as f32).round() as usize)
                .clamp(1, capacity),
            waveform_rebuild: None,
            lowpass: [[[0.0; 2]; 2]; 6],
        }
    }
    pub fn window_frames(&self) -> usize {
        self.window
    }
    pub fn range_epoch(&self) -> u64 {
        (self.total - self.window as u64) / self.sample.len() as u64 * self.sample.len() as u64
    }
    pub fn range(&self) -> std::ops::Range<usize> {
        let start = ((self.total - self.window as u64) % self.sample.len() as u64) as usize;
        start..start + self.window
    }
    pub fn display_position(&self, position: f64) -> f64 {
        (position + self.range_epoch() as f64).rem_euclid(self.window as f64) / self.window as f64
    }
    pub fn set_window_seconds(&mut self, seconds: f32) -> bool {
        let frames = ((seconds.clamp(0.125, 32.0) * self.sample.sample_rate as f32).round()
            as usize)
            .clamp(1, self.sample.len());
        if frames == self.window {
            return false;
        }
        self.window = frames;
        self.sample.waveform.fill([0.; 2]);
        self.waveform_rebuild = Some((self.total - self.captured.min(frames) as u64, self.total));
        true
    }
    // Rebuild the new view in bounded work, including while capture is frozen.
    // The retained PCM and antialiasing levels are never rebuilt or cleared.
    fn rebuild_waveform(&mut self) {
        let Some((mut at, end)) = self.waveform_rebuild else {
            return;
        };
        at = at.max(self.total - self.window as u64);
        let until = (at + 16).min(end);
        for index in at..until {
            let frame = self.sample.levels[0][(index % self.sample.len() as u64) as usize];
            let value = (frame[0] + frame[1]) * 0.5;
            let bin =
                (index % self.window as u64) as usize * self.sample.waveform.len() / self.window;
            self.sample.waveform[bin][0] = self.sample.waveform[bin][0].min(value);
            self.sample.waveform[bin][1] = self.sample.waveform[bin][1].max(value);
        }
        self.waveform_rebuild = (until < end).then_some((until, end));
    }

    pub fn push(&mut self, frame: Stereo, frozen: bool) {
        self.push_with_meter(frame, frame, frozen);
    }
    pub fn push_with_meter(&mut self, frame: Stereo, meter: Stereo, frozen: bool) {
        self.rebuild_waveform();
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
        let phase = (self.total % self.window as u64) as usize;
        let bin = phase * self.sample.waveform.len() / self.window;
        let value = (frame[0] + frame[1]) * 0.5;
        if phase == 0 || (phase - 1) * self.sample.waveform.len() / self.window != bin {
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
        self.total += 1;
    }

    pub fn head(&self) -> usize {
        self.write
    }
    pub fn seconds(&self) -> f64 {
        self.captured.min(self.window) as f64 / self.sample.sample_rate as f64
    }

    /// Return only captured frames, oldest first, including after a ring wrap.
    pub fn capture(&self) -> Vec<Stereo> {
        let count = self.captured.min(self.window);
        let start = self.total - count as u64;
        (0..count)
            .map(|i| self.sample.frames()[((start + i as u64) % self.sample.len() as u64) as usize])
            .collect()
    }
}

/// Build a live history between render quanta. Only initialized, complete input
/// buffers can be committed to an engine; no synthesis state is advanced here.
pub struct LiveInputPreparation {
    sample_rate: u32,
    effects_only: bool,
    levels: Vec<Vec<Stereo>>,
    level: usize,
    length: usize,
}
impl LiveInputPreparation {
    pub fn new(sample_rate: u32, effects_only: bool) -> Self {
        assert!(
            (8_000..=384_000).contains(&sample_rate),
            "valid input sample rate"
        );
        Self {
            sample_rate,
            effects_only,
            levels: Vec::with_capacity(7),
            level: 0,
            length: sample_rate as usize * 8,
        }
    }
    /// Reserve at most one level or initialize at most 256 KiB in one step.
    pub fn step(&mut self) -> bool {
        if self.level == 7 {
            return true;
        }
        if self.level == self.levels.len() {
            self.levels.push(Vec::with_capacity(self.length));
            return false;
        }
        let level = &mut self.levels[self.level];
        level.resize((level.len() + 32_768).min(self.length), [0.0; 2]);
        if level.len() == self.length {
            self.level += 1;
            self.length = self.length.div_ceil(2);
        }
        self.level == 7
    }
    pub fn finish(self) -> Option<LiveInput> {
        // Eight seconds, all of them visible, as LiveInput::new.
        let capacity = self.sample_rate as usize * 8;
        (self.level == 7).then(|| LiveInput {
            sample: SampleBuffer {
                name: "Microphone".into(),
                library_id: None,
                sample_rate: self.sample_rate,
                levels: self.levels,
                active_levels: 7,
                waveform: vec![[0.0; 2]; capacity.min(640)],
            },
            effects_only: self.effects_only,
            peak: [0.0; 2],
            write: 0,
            captured: 0,
            total: capacity as u64,
            window: capacity,
            waveform_rebuild: None,
            lowpass: [[[0.0; 2]; 2]; 6],
        })
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
    /// Number of readable mip levels, including the original PCM.
    pub(crate) fn active_levels(&self) -> usize {
        self.active_levels
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

    /// Read one mip level over the whole file; `level` must be an active level.
    #[inline(always)]
    pub(crate) fn read_level(&self, position: f64, level: usize) -> Stereo {
        if !position.is_finite() {
            return [0.0; 2];
        }
        let data = &self.levels[level];
        // Keep indices in f64: f32 loses sub-sample precision on long recordings.
        // Scaling by 2^-level is exact, like the division by 2^level it replaces.
        let p = wrap_position(position, self.len() as f64) * level_scale(level);
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
        let p =
            range.start as f64 + wrap_position(position - range.start as f64, range.len() as f64);
        while level > 0 {
            let scale = (1_usize << level) as f64;
            // Multiplying by 2^-level is the same exact value as the division.
            let base = (p * level_scale(level)).floor();
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

/// 2^-level as an exact float64, for levels below 64.
#[inline(always)]
pub(crate) fn level_scale(level: usize) -> f64 {
    f64::from_bits((1023 - level as u64) << 52)
}

#[inline(always)]
fn interpolation_frames(data: &[Stereo], base: i64, range: &std::ops::Range<usize>) -> [Stereo; 4] {
    // Most grain reads are inside the region. Boundary reads retain the exact
    // wrap used for tiny selections, negative positions and reverse grains.
    if base > range.start as i64 && base + 2 < range.end as i64 {
        let i = base as usize;
        // One bounds check for the four frames instead of one per frame.
        if let Some(frames) = data.get(i - 1..i + 3) {
            return [frames[0], frames[1], frames[2], frames[3]];
        }
        let i = i % data.len();
        if i > 0 && i + 2 < data.len() {
            return [data[i - 1], data[i], data[i + 1], data[i + 2]];
        }
    }
    std::array::from_fn(|i| {
        data[(range.start
            + (base + i as i64 - 1 - range.start as i64).rem_euclid(range.len() as i64) as usize)
            % data.len()]
    })
}

/// `rem_euclid` without its `fmod` call when the position is less than one
/// length outside the range. Both shortcuts are the exact operations
/// `rem_euclid` performs there: the remainder of |x| < length is x itself, and
/// a negative one gets length added once. -length itself stays on the slow
/// path, where the result is -0.0.
#[inline]
pub(crate) fn wrap_position(position: f64, length: f64) -> f64 {
    if position >= 0.0 && position < length {
        return position;
    }
    if position < 0.0 && position > -length {
        return position + length;
    }
    position.rem_euclid(length)
}

#[cfg(test)]
mod preparation_tests {
    use super::*;
    #[test]
    fn bounded_input_preparation_matches_the_original_history_and_mips() {
        for rate in [8000, 44_100, 96_000] {
            let mut pending = LiveInputPreparation::new(rate, false);
            loop {
                let before: usize = pending.levels.iter().map(Vec::len).sum();
                let ready = pending.step();
                let after: usize = pending.levels.iter().map(Vec::len).sum();
                assert!(after - before <= 32_768);
                if ready {
                    break;
                }
            }
            let mut actual = pending.finish().unwrap();
            let mut reference = LiveInput::new(rate, false);
            assert_eq!(actual.sample.levels, reference.sample.levels);
            assert_eq!(actual.sample.waveform, reference.sample.waveform);
            for i in 0..1024 {
                let frame = [(i as f32 * 0.13).sin(), (i as f32 * 0.17).cos()];
                actual.push(frame, i % 41 == 0);
                reference.push(frame, i % 41 == 0);
            }
            assert_eq!(actual.capture(), reference.capture());
            assert_eq!(actual.sample.levels, reference.sample.levels);
            assert_eq!(actual.sample.waveform, reference.sample.waveform);
            assert_eq!(actual.peak, reference.peak);
            assert_eq!(actual.lowpass, reference.lowpass);
        }
    }
}

#[cfg(test)]
mod wrap_tests {
    #[test]
    fn wrap_position_is_rem_euclid_bit_for_bit() {
        let mut seed = 0x2545_f491_4f6c_dd1d_u64;
        for length in [1.0, 4.0, 3.0e-3, 44_100.0, 352_800.0, 1e9 + 0.5] {
            let below = |x: f64| f64::from_bits(x.to_bits() - 1);
            let mut positions = vec![
                0.0,
                -0.0,
                length,
                -length,
                below(length),
                -below(length),
                length * 2.0,
                -length * 2.0,
                -1e-300,
                1e-300,
                f64::NAN,
                f64::INFINITY,
                f64::NEG_INFINITY,
            ];
            for _ in 0..10_000 {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                let unit = (seed >> 11) as f64 / (1_u64 << 53) as f64;
                positions.push((unit * 6.0 - 3.0) * length);
            }
            for x in positions {
                assert_eq!(
                    super::wrap_position(x, length).to_bits(),
                    x.rem_euclid(length).to_bits(),
                    "{x} wrapped by {length}"
                );
            }
        }
    }
}
