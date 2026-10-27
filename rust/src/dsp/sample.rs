use crate::{Stereo, tables::Random};
use std::{f64::consts::PI, sync::LazyLock};

/// Frames the interpolator weighs: base-5 ..= base+6.
const TAPS: usize = 12;
const HALF: i64 = TAPS as i64 / 2;
/// Kernel rows per frame; a read takes the nearest one (±1/2048 frame).
const PHASES: usize = 1024;
const ROWS: usize = PHASES + 1;

/// Kaiser-windowed sinc (β = 5) for every 1/PHASES of a frame, each row at
/// unity gain. Grains read between frames almost always: at 44.1 kHz this
/// stays within 0.1 dB to 16 kHz (0.4 dB at 18 kHz) and images tones below
/// 13 kHz 60 dB down, where the cubic it replaces lost 1.8 dB at 16 kHz and
/// 2.4 dB at 18 kHz and imaged them at -18 dB. Sixteen taps kept 0.3 dB more
/// at 18 kHz but took WASM 15% longer to render 128 grains.
static KERNEL: LazyLock<Box<[[f32; TAPS]]>> = LazyLock::new(|| {
    const BETA: f64 = 5.0;
    let i0 = |x: f64| {
        let (mut sum, mut term) = (1.0, 1.0);
        for k in 1..40 {
            term *= (x * 0.5 / k as f64).powi(2);
            sum += term;
        }
        sum
    };
    (0..=PHASES)
        .map(|row| {
            let mut weights = [0.0; TAPS];
            if row % PHASES == 0 {
                // Whole frames read exactly the frame.
                weights[(HALF - 1) as usize + row / PHASES] = 1.0;
                return weights;
            }
            let t = row as f64 / PHASES as f64;
            let taps: [f64; TAPS] = std::array::from_fn(|k| {
                let x = k as f64 - (HALF - 1) as f64 - t;
                let ratio = x / HALF as f64;
                let window = i0(BETA * (1.0 - ratio * ratio).max(0.0).sqrt()) / i0(BETA);
                (PI * x).sin() / (PI * x) * window
            });
            let sum: f64 = taps.iter().sum();
            for (weight, tap) in weights.iter_mut().zip(taps) {
                *weight = (tap / sum) as f32;
            }
            weights
        })
        .collect()
});

/// Reads that step more than one frame of their level fold what lies above
/// the new Nyquist back down. Up to 2^(1/8) frames a read all of it lands
/// above 20 kHz, and the flat kernel reads on. Each faster band of steps, an
/// eighth of an octave wide, has the flat kernel stretched by its middle
/// step: the cutoff falls with the step, and the taps grow with it so the
/// transition stays about as narrow at the output as the flat kernel's.
const WIDE_KERNELS: usize = 7;
/// The steps where the flat kernel's band and each wide one end, 2^(k/8).
const BAND_TOPS: [f64; WIDE_KERNELS + 1] = [
    1.090_507_732_665_257_7,
    1.189_207_115_002_721,
    1.296_839_554_651_009_6,
    std::f64::consts::SQRT_2,
    1.542_210_825_407_940_8,
    1.681_792_830_507_429,
    1.834_008_086_409_342_6,
    2.0,
];
/// Kernels 1..=3 have 16 taps, 4 and 5 have 20, 6 and 7 have 24.
struct Wide {
    taps16: Box<[[f32; 16]]>,
    taps20: Box<[[f32; 20]]>,
    taps24: Box<[[f32; 24]]>,
}
static WIDE: LazyLock<Wide> = LazyLock::new(|| {
    // The middle of band k, 2^((k + 1/2)/8).
    let stretch = |k: usize| BAND_TOPS[k - 1] * 1.044_273_782_427_413_8;
    Wide {
        taps16: (1..=3).flat_map(|k| wide_kernel(stretch(k))).collect(),
        taps20: (4..=5).flat_map(|k| wide_kernel(stretch(k))).collect(),
        taps24: (6..=7).flat_map(|k| wide_kernel(stretch(k))).collect(),
    }
});

/// ROWS rows of an N-tap Kaiser-windowed sinc (β = 5) with its cutoff at
/// 1/`stretch` of Nyquist, each at unity gain.
fn wide_kernel<const N: usize>(stretch: f64) -> Vec<[f32; N]> {
    const BETA: f64 = 5.0;
    let half = (N / 2) as f64;
    // I0's series; past twenty terms they are below an f64's precision.
    let inverse_squares: [f64; 19] = std::array::from_fn(|k| 1.0 / ((k + 1) * (k + 1)) as f64);
    let i0 = |x: f64| {
        let quarter = x * x * 0.25;
        let (mut sum, mut term) = (1.0, 1.0);
        for inverse in inverse_squares {
            term *= quarter * inverse;
            sum += term;
        }
        sum
    };
    let i0_beta = i0(BETA);
    let mut rows = vec![[0.0; N]; ROWS];
    // Row PHASES - r is row r backwards.
    for row in 0..=PHASES / 2 {
        let t = row as f64 / PHASES as f64;
        let taps: [f64; N] = std::array::from_fn(|k| {
            let x = k as f64 - (half - 1.0) - t;
            let ratio = x / half;
            let window = i0(BETA * (1.0 - ratio * ratio).max(0.0).sqrt()) / i0_beta;
            let y = PI * x / stretch;
            if y == 0.0 {
                window
            } else {
                y.sin() / y * window
            }
        });
        let sum: f64 = taps.iter().sum();
        for (k, tap) in taps.into_iter().enumerate() {
            let weight = (tap / sum) as f32;
            rows[row][k] = weight;
            rows[PHASES - row][N - 1 - k] = weight;
        }
    }
    rows
}

/// Build the kernels off the audio thread.
pub fn prepare_interpolation() {
    LazyLock::force(&KERNEL);
    LazyLock::force(&WIDE);
}

/// The kernel for a read that steps `step` frames of its level.
fn kernel_for_step(step: f32) -> u8 {
    if step.is_nan() {
        return 0;
    }
    BAND_TOPS
        .iter()
        .position(|&top| step as f64 <= top)
        .unwrap_or(WIDE_KERNELS) as u8
}

/// The mip level a read takes its frames from, and the kernel that reads it.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct Mip {
    pub(crate) level: usize,
    /// Frames of the level each read steps; 0 where the rate is not followed.
    step: f32,
    /// 0 is the flat kernel, 1..=WIDE_KERNELS read ever faster steps.
    kernel: u8,
}
impl Mip {
    fn new(level: usize, step: f32) -> Self {
        Self {
            level,
            step,
            kernel: kernel_for_step(step),
        }
    }
    /// The flat kernel on `level`, wherever the read lands.
    pub(crate) fn flat(level: usize) -> Self {
        Self {
            level,
            step: 0.0,
            kernel: 0,
        }
    }
    /// Whether a wide kernel reads it.
    pub(crate) fn is_wide(self) -> bool {
        self.kernel != 0
    }
    /// The next level down, which the same read steps twice as fast through.
    fn finer(self) -> Self {
        Self::new(self.level - 1, self.step * 2.0)
    }
    #[inline(always)]
    fn at_most(self, top: usize) -> Self {
        if self.level > top {
            self.down_to(top)
        } else {
            self
        }
    }
    #[cold]
    fn down_to(mut self, top: usize) -> Self {
        while self.level > top {
            self = self.finer();
        }
        self
    }
    /// The frames the kernel weighs on each side.
    fn half(self) -> i64 {
        match self.kernel {
            0 => HALF,
            1..=3 => 8,
            4 | 5 => 10,
            _ => 12,
        }
    }
}

/// The frame `fraction` past `base` in `range` of `data`, read by `kernel`.
/// Inlined where the kernel is known, the other kernels compile away: in the
/// grain loop their code would slow the flat kernel's reads by a third.
/// MONO weighs the left channel alone, for files whose channels are equal.
#[inline(always)]
fn read_kernel<const MONO: bool>(
    data: &[Stereo],
    base: i64,
    fraction: f64,
    range: &std::ops::Range<usize>,
    kernel: u8,
) -> Stereo {
    let rows = |offset: u8| offset as usize * ROWS..(offset as usize + 1) * ROWS;
    match kernel {
        0 => interpolated::<TAPS, MONO>(data, base, fraction, range, &KERNEL[..]),
        1..=3 => {
            interpolated::<16, MONO>(data, base, fraction, range, &WIDE.taps16[rows(kernel - 1)])
        }
        4 | 5 => {
            interpolated::<20, MONO>(data, base, fraction, range, &WIDE.taps20[rows(kernel - 4)])
        }
        _ => interpolated::<24, MONO>(data, base, fraction, range, &WIDE.taps24[rows(kernel - 6)]),
    }
}

/// The frame `fraction` (0..1) past `frames[N / 2 - 1]`.
#[inline(always)]
fn interpolate<const N: usize, const MONO: bool>(
    frames: &[Stereo; N],
    fraction: f64,
    rows: &[[f32; N]],
) -> Stereo {
    weigh::<N, MONO>(frames, &rows[(fraction * PHASES as f64 + 0.5) as usize])
}

/// `frames` weighed by `row`. Four partial sums a channel keep the
/// multiply-adds independent: tap i adds into sum i % 4, then
/// (s0 + s1) + (s2 + s3). SIMD lanes are those sums, multiplied and added as
/// the scalar code does them, so every target and build renders the same
/// bits. MONO leaves out the right channel, whose sums would take the same
/// steps to the same bits.
#[cfg(not(any(
    target_arch = "aarch64",
    target_arch = "x86_64",
    all(target_arch = "wasm32", target_feature = "simd128")
)))]
#[inline(always)]
fn weigh<const N: usize, const MONO: bool>(frames: &[Stereo; N], row: &[f32; N]) -> Stereo {
    let mut sums = [[0.0_f32; 4]; 2];
    for i in (0..N).step_by(4) {
        for j in 0..4 {
            sums[0][j] += row[i + j] * frames[i + j][0];
            if !MONO {
                sums[1][j] += row[i + j] * frames[i + j][1];
            }
        }
    }
    let [left, right] = sums.map(|s| (s[0] + s[1]) + (s[2] + s[3]));
    [left, if MONO { left } else { right }]
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn weigh<const N: usize, const MONO: bool>(frames: &[Stereo; N], row: &[f32; N]) -> Stereo {
    use std::arch::aarch64::*;
    // SAFETY: NEON is part of every aarch64 target, and each load reads taps
    // i..i + 4 of N, a multiple of four.
    unsafe {
        let (mut left, mut right) = (vdupq_n_f32(0.0), vdupq_n_f32(0.0));
        for i in (0..N).step_by(4) {
            // Four frames, split into their left and right channels.
            let channels = vld2q_f32(frames.as_ptr().add(i).cast());
            let weights = vld1q_f32(row.as_ptr().add(i));
            left = vaddq_f32(left, vmulq_f32(weights, channels.0));
            if !MONO {
                right = vaddq_f32(right, vmulq_f32(weights, channels.1));
            }
        }
        // [s0 + s1, s2 + s3] of each channel, then their sum.
        let halves = vpaddq_f32(left, right);
        let sums = vpaddq_f32(halves, halves);
        let left = vgetq_lane_f32(sums, 0);
        [left, if MONO { left } else { vgetq_lane_f32(sums, 1) }]
    }
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
fn weigh<const N: usize, const MONO: bool>(frames: &[Stereo; N], row: &[f32; N]) -> Stereo {
    use std::arch::x86_64::*;
    // SAFETY: SSE is part of every x86_64 target, and each load reads taps
    // i..i + 4 of N, a multiple of four.
    unsafe {
        let (mut left, mut right) = (_mm_setzero_ps(), _mm_setzero_ps());
        for i in (0..N).step_by(4) {
            let first = _mm_loadu_ps(frames.as_ptr().add(i).cast());
            let second = _mm_loadu_ps(frames.as_ptr().add(i + 2).cast());
            let weights = _mm_loadu_ps(row.as_ptr().add(i));
            left = _mm_add_ps(
                left,
                _mm_mul_ps(weights, _mm_shuffle_ps::<0x88>(first, second)),
            );
            if !MONO {
                let rights = _mm_shuffle_ps::<0xdd>(first, second);
                right = _mm_add_ps(right, _mm_mul_ps(weights, rights));
            }
        }
        let sum = |v: __m128| {
            let s: [f32; 4] = std::mem::transmute(v);
            (s[0] + s[1]) + (s[2] + s[3])
        };
        let left = sum(left);
        [left, if MONO { left } else { sum(right) }]
    }
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
#[inline(always)]
fn weigh<const N: usize, const MONO: bool>(frames: &[Stereo; N], row: &[f32; N]) -> Stereo {
    use std::arch::wasm32::*;
    // SAFETY: each load reads taps i..i + 4 of N, a multiple of four.
    unsafe {
        let (mut left, mut right) = (f32x4_splat(0.0), f32x4_splat(0.0));
        for i in (0..N).step_by(4) {
            let first = v128_load(frames.as_ptr().add(i).cast());
            let second = v128_load(frames.as_ptr().add(i + 2).cast());
            let weights = v128_load(row.as_ptr().add(i).cast());
            let lefts = i32x4_shuffle::<0, 2, 4, 6>(first, second);
            left = f32x4_add(left, f32x4_mul(weights, lefts));
            if !MONO {
                let rights = i32x4_shuffle::<1, 3, 5, 7>(first, second);
                right = f32x4_add(right, f32x4_mul(weights, rights));
            }
        }
        let sum = |v: v128| {
            (f32x4_extract_lane::<0>(v) + f32x4_extract_lane::<1>(v))
                + (f32x4_extract_lane::<2>(v) + f32x4_extract_lane::<3>(v))
        };
        let left = sum(left);
        [left, if MONO { left } else { sum(right) }]
    }
}

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
    /// Every frame of every level has the same bits left and right, so a
    /// read can weigh one channel. Never set for a live history.
    mono: bool,
    pub waveform: Vec<[f32; 2]>,
}

/// Whether `frames` have the same bits left and right.
fn mono(frames: &[Stereo]) -> bool {
    frames.iter().all(|f| f[0].to_bits() == f[1].to_bits())
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
    /// What a Rack's Grain engine hears of its chain, with the drum window.
    pub fn for_rack(sample_rate: u32, seconds: f32) -> Self {
        let mut input = Self::with_capacity(
            sample_rate,
            false,
            crate::rack::MAX_BUFFER_SECONDS as usize,
            seconds,
        );
        input.sample.name = "Rack".into();
        input
    }
    fn with_capacity(sample_rate: u32, effects_only: bool, seconds: usize, window: f32) -> Self {
        let capacity = sample_rate as usize * seconds;
        let mut length = capacity;
        let mut levels = Vec::with_capacity(7);
        for _ in 0..7 {
            levels.push(vec![[0.0; 2]; length]);
            length = length.div_ceil(2);
        }
        let mut sample =
            SampleBuffer::from_prepared("Microphone".into(), None, sample_rate, levels)
                .expect("valid input buffer");
        sample.mono = false;
        Self {
            sample,
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
                mono: false,
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
            mono: true,
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
        self.mono = true;
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
        self.mono &= mono(frames);
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
            // The mips of a mono file are filtered alike on both channels.
            mono: mono(&levels[0]),
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
            mono: levels.iter().all(|level| mono(level)),
            levels,
            waveform,
        })
    }

    pub fn read(&self, position: f64, rate: f64, antialiasing: bool) -> Stereo {
        self.read_mip(position, self.mip_for_rate(rate, antialiasing))
    }

    pub(crate) fn level_for_rate(&self, rate: f64, antialiasing: bool) -> usize {
        if antialiasing && rate.abs() >= 2.0 {
            (rate.abs().log2().floor() as usize).min(self.active_levels - 1)
        } else {
            0
        }
    }

    /// The level and kernel that read at `rate` without folding back.
    pub(crate) fn mip_for_rate(&self, rate: f64, antialiasing: bool) -> Mip {
        let level = self.level_for_rate(rate, antialiasing);
        if antialiasing {
            Mip::new(level, (rate.abs() * level_scale(level)) as f32)
        } else {
            Mip::flat(level)
        }
    }

    /// Read one mip level over the whole file with the flat kernel; `level`
    /// must be an active level.
    #[inline(always)]
    pub(crate) fn read_level(&self, position: f64, level: usize) -> Stereo {
        self.read_whole::<false>(position, level, 0)
    }

    /// Read `mip` over the whole file; levels above the active ones read the
    /// top one, stepping faster.
    #[inline(always)]
    pub(crate) fn read_mip(&self, position: f64, mip: Mip) -> Stereo {
        let mip = mip.at_most(self.active_levels - 1);
        if self.mono {
            self.read_whole::<true>(position, mip.level, mip.kernel)
        } else {
            self.read_whole::<false>(position, mip.level, mip.kernel)
        }
    }

    /// Read each whole-file grain `(slot, position, mip)` into `frames[slot]`:
    /// the grains a wide kernel reads, before the grain loop, so that loop
    /// keeps the flat kernel's read alone.
    #[inline(never)]
    pub(crate) fn read_mips(
        &self,
        reads: impl Iterator<Item = (usize, f64, Mip)>,
        frames: &mut [Stereo],
    ) {
        let top = self.active_levels - 1;
        for (slot, position, mip) in reads {
            let mip = mip.at_most(top);
            frames[slot] = if self.mono {
                self.read_whole::<true>(position, mip.level, mip.kernel)
            } else {
                self.read_whole::<false>(position, mip.level, mip.kernel)
            };
        }
    }

    #[inline(always)]
    fn read_whole<const MONO: bool>(&self, position: f64, level: usize, kernel: u8) -> Stereo {
        if !position.is_finite() {
            return [0.0; 2];
        }
        let data = &self.levels[level];
        // Keep indices in f64: f32 loses sub-sample precision on long recordings.
        // Scaling by 2^-level is exact, like the division by 2^level it replaces.
        let p = wrap_position(position, self.len() as f64) * level_scale(level);
        let base = p.floor();
        read_kernel::<MONO>(data, base as i64, p - base, &(0..data.len()), kernel)
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
        self.read_region_mip(position, self.mip_for_rate(rate, antialiasing), range)
    }

    pub(crate) fn read_region_level(
        &self,
        position: f64,
        level: usize,
        range: &std::ops::Range<usize>,
    ) -> Stereo {
        self.read_region_mip(position, Mip::flat(level), range)
    }

    pub(crate) fn read_region_mip(
        &self,
        position: f64,
        mip: Mip,
        range: &std::ops::Range<usize>,
    ) -> Stereo {
        if self.mono {
            self.read_region_as::<true>(position, mip, range)
        } else {
            self.read_region_as::<false>(position, mip, range)
        }
    }

    #[inline(always)]
    fn read_region_as<const MONO: bool>(
        &self,
        position: f64,
        mip: Mip,
        range: &std::ops::Range<usize>,
    ) -> Stereo {
        let mut mip = mip.at_most(self.active_levels - 1);
        if range.start == 0 && range.end == self.len() {
            return self.read_whole::<MONO>(position, mip.level, mip.kernel);
        }
        if !position.is_finite() {
            return [0.0; 2];
        }
        let p =
            range.start as f64 + wrap_position(position - range.start as f64, range.len() as f64);
        while mip.level > 0 {
            let scale = (1_usize << mip.level) as f64;
            // Multiplying by 2^-level is the same exact value as the division.
            let base = (p * level_scale(mip.level)).floor();
            let support = 16.0 * (scale - 1.0);
            let half = mip.half() as f64;
            if (base - (half - 1.0)) * scale - support >= range.start as f64
                && (base + half) * scale + support < range.end as f64
            {
                return self.read_whole::<MONO>(p, mip.level, mip.kernel);
            }
            // A finer level, stepped through twice as fast.
            mip = mip.finer();
        }
        let base = p.floor();
        read_kernel::<MONO>(self.frames(), base as i64, p - base, range, mip.kernel)
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

/// The frame `fraction` past `base`, from the frames around it in `range`.
#[inline(always)]
fn interpolated<const N: usize, const MONO: bool>(
    data: &[Stereo],
    base: i64,
    fraction: f64,
    range: &std::ops::Range<usize>,
    rows: &[[f32; N]],
) -> Stereo {
    let half = N as i64 / 2;
    let first = base - (half - 1);
    // Most grain reads are inside the region. Boundary reads retain the exact
    // wrap used for tiny selections, negative positions and reverse grains.
    if first >= range.start as i64 && base + half < range.end as i64 {
        let i = first as usize;
        // One bounds check for the taps instead of one per frame.
        if let Some(frames) = data.get(i..i + N) {
            return interpolate::<N, MONO>(frames.try_into().unwrap(), fraction, rows);
        }
    }
    let frames = std::array::from_fn(|i| {
        data[(range.start
            + (first + i as i64 - range.start as i64).rem_euclid(range.len() as i64) as usize)
            % data.len()]
    });
    interpolate::<N, MONO>(&frames, fraction, rows)
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
