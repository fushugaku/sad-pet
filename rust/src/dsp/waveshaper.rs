//! Extended waveshaper processor. Legacy mode stays on the original RADIAS
//! processor; clean modes run the native kernels between cascaded halfband
//! interpolators and decimators.

use super::radias_waveshaper::{
    self as radias, PhaseIncrement, RadiasWaveshaper,
    kernels::{ShaperCoefficients, ShaperTables, Waveshaper},
};
use crate::{
    Stereo,
    color::WaveshaperMode,
    waveshaper::{Curve, Options, Quality, Stage},
};
use std::cell::RefCell;

const FIR_TAPS: usize = 65;
const HALF_TAPS: usize = FIR_TAPS / 2;
const FIR_CENTER: usize = FIR_TAPS / 2;
pub(crate) const MAX_LATENCY_SAMPLES: usize = FIR_CENTER * 3 / 2;
const DELAY_CAPACITY: usize = MAX_LATENCY_SAMPLES + 1;
const SMOOTH_SECONDS: f32 = 0.004;
const LEVEL_MATCH_CACHE_SIZE: usize = 64;

// 65-tap Kaiser-windowed halfband FIR (beta=9), symmetric and normalized.
// Exact zero taps are retained so the two interpolation phases stay compact.
#[allow(clippy::excessive_precision)]
const HALF_BAND: [f32; FIR_TAPS] = [
    0.0,
    -2.5276330850e-5,
    0.0,
    9.5903945197e-5,
    0.0,
    -2.5250812310e-4,
    0.0,
    5.5308765522e-4,
    0.0,
    -1.0766606508e-3,
    0.0,
    1.9261057468e-3,
    0.0,
    -3.2317568666e-3,
    0.0,
    5.1579925225e-3,
    0.0,
    -7.9176846117e-3,
    0.0,
    1.1805179775e-2,
    0.0,
    -1.7273247806e-2,
    0.0,
    2.5123162123e-2,
    0.0,
    -3.7032866410e-2,
    0.0,
    5.7364864736e-2,
    0.0,
    -1.0221339714e-1,
    0.0,
    3.1699710143e-1,
    0.5,
    3.1699710143e-1,
    0.0,
    -1.0221339714e-1,
    0.0,
    5.7364864736e-2,
    0.0,
    -3.7032866410e-2,
    0.0,
    2.5123162123e-2,
    0.0,
    -1.7273247806e-2,
    0.0,
    1.1805179775e-2,
    0.0,
    -7.9176846117e-3,
    0.0,
    5.1579925225e-3,
    0.0,
    -3.2317568666e-3,
    0.0,
    1.9261057468e-3,
    0.0,
    -1.0766606508e-3,
    0.0,
    5.5308765522e-4,
    0.0,
    -2.5250812310e-4,
    0.0,
    9.5903945197e-5,
    0.0,
    -2.5276330850e-5,
    0.0,
];

/// Static transfer for curve previews and the clean curve layer.
pub fn curve_transfer(curve: Curve, x: f32, knee: f32, points: &[f32; 17]) -> f32 {
    let x = finite_or_zero(x).clamp(-1.0, 1.0);
    let knee = finite_or_zero(knee).clamp(0.0, 1.0);
    match curve {
        // RADIAS is the mode-dependent kernel before this optional static layer.
        Curve::Radias => x,
        Curve::SoftClip => {
            let threshold = 1.0 - 0.75 * knee;
            let magnitude = x.abs();
            if magnitude <= threshold {
                x
            } else {
                let u = ((magnitude - threshold) / (1.0 - threshold)).clamp(0.0, 1.0);
                let eased = u + u * u - u * u * u;
                x.signum() * (threshold + (1.0 - threshold) * eased)
            }
        }
        Curve::Tanh => {
            let gain = 0.15 + 5.85 * knee;
            (gain * x).tanh() / gain.tanh()
        }
        Curve::Diode => {
            let gain = 0.15 + 5.85 * knee;
            if x >= 0.0 {
                (gain * x).tanh() / gain.tanh()
            } else {
                let negative_gain = 0.55 * gain;
                (negative_gain * x).tanh() / negative_gain.tanh()
            }
        }
        Curve::Rectify => x.abs(),
        Curve::Custom => custom_transfer(x, points),
    }
}

/// Evaluate both selected curves with the model's morph control.
pub fn transfer(options: Options, x: f32) -> f32 {
    let options = options.sanitize();
    let a = curve_transfer(options.curve, x, options.knee, &options.points);
    let b = curve_transfer(options.curve_b, x, options.knee, &options.points);
    a + (b - a) * options.morph
}

fn custom_transfer(x: f32, points: &[f32; 17]) -> f32 {
    let position = (x.clamp(-1.0, 1.0) + 1.0) * 8.0;
    let index = (position.floor() as usize).min(15);
    let fraction = position - index as f32;
    let y0 = finite_or_zero(points[index]).clamp(-1.0, 1.0);
    let y1 = finite_or_zero(points[index + 1]).clamp(-1.0, 1.0);
    (y0 + (y1 - y0) * fraction).clamp(-1.0, 1.0)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct LevelMatchKey {
    mode: WaveshaperMode,
    depth_bits: u32,
    sample_rate: u32,
    curve: Curve,
    curve_b: Curve,
    morph_bits: u32,
    input_bits: u32,
    bias_bits: u32,
    knee_bits: u32,
    point_bits: [u32; 17],
}

impl LevelMatchKey {
    fn new(mode: WaveshaperMode, depth: f32, options: Options, sample_rate: u32) -> Self {
        Self {
            mode,
            depth_bits: depth.to_bits(),
            sample_rate,
            curve: options.curve,
            curve_b: options.curve_b,
            morph_bits: options.morph.to_bits(),
            input_bits: options.input.to_bits(),
            bias_bits: options.bias.to_bits(),
            knee_bits: options.knee.to_bits(),
            point_bits: std::array::from_fn(|index| options.points[index].to_bits()),
        }
    }

    fn slot(self) -> usize {
        let mut hash = 0xcbf29ce484222325_u64;
        let mut add = |word: u32| {
            hash ^= word as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        };
        add(mode_key(self.mode));
        add(self.depth_bits);
        add(self.sample_rate);
        add(curve_key(self.curve));
        add(curve_key(self.curve_b));
        add(self.morph_bits);
        add(self.input_bits);
        add(self.bias_bits);
        add(self.knee_bits);
        for point in self.point_bits {
            add(point);
        }
        (hash as usize) & (LEVEL_MATCH_CACHE_SIZE - 1)
    }
}

fn mode_key(mode: WaveshaperMode) -> u32 {
    match mode {
        WaveshaperMode::Drive => 0,
        WaveshaperMode::HardClip => 1,
        WaveshaperMode::Decimator => 2,
        WaveshaperMode::OctSaw => 3,
        WaveshaperMode::MultiTriangle => 4,
        WaveshaperMode::MultiSine => 5,
        WaveshaperMode::SubSaw => 6,
        WaveshaperMode::SubSquare => 7,
        WaveshaperMode::SubTriangle => 8,
        WaveshaperMode::SubSine => 9,
        WaveshaperMode::Pickup => 10,
        WaveshaperMode::LevelBoost => 11,
    }
}

fn curve_key(curve: Curve) -> u32 {
    match curve {
        Curve::Radias => 0,
        Curve::SoftClip => 1,
        Curve::Tanh => 2,
        Curve::Diode => 3,
        Curve::Rectify => 4,
        Curve::Custom => 5,
    }
}

#[derive(Clone, Copy)]
struct LevelMatchEntry {
    key: LevelMatchKey,
    gain: f32,
    valid: bool,
}

impl LevelMatchEntry {
    const EMPTY: Self = Self {
        key: LevelMatchKey {
            mode: WaveshaperMode::Drive,
            depth_bits: 0,
            sample_rate: 0,
            curve: Curve::Radias,
            curve_b: Curve::Radias,
            morph_bits: 0,
            input_bits: 0,
            bias_bits: 0,
            knee_bits: 0,
            point_bits: [0; 17],
        },
        gain: 1.0,
        valid: false,
    };
}

struct LevelMatchCache {
    entries: [LevelMatchEntry; LEVEL_MATCH_CACHE_SIZE],
    calibration_count: u64,
}

impl LevelMatchCache {
    const fn new() -> Self {
        Self {
            entries: [LevelMatchEntry::EMPTY; LEVEL_MATCH_CACHE_SIZE],
            calibration_count: 0,
        }
    }

    fn get(&self, key: LevelMatchKey) -> Option<f32> {
        let entry = self.entries[key.slot()];
        (entry.valid && entry.key == key).then_some(entry.gain)
    }

    fn insert(&mut self, key: LevelMatchKey, gain: f32) {
        self.entries[key.slot()] = LevelMatchEntry {
            key,
            gain,
            valid: true,
        };
        self.calibration_count = self.calibration_count.wrapping_add(1);
    }
}

thread_local! {
    // Const TLS has no per-thread heap initialization, and LevelMatchCache is
    // plain fixed-size data with no destructor or synchronization primitive.
    static LEVEL_MATCH_CACHE: RefCell<LevelMatchCache> = const {
        RefCell::new(LevelMatchCache::new())
    };
}

fn static_level_match(mode: WaveshaperMode, depth: f32, options: Options, sample_rate: u32) -> f32 {
    if !options.auto_gain {
        return 1.0;
    }
    let key = LevelMatchKey::new(mode, depth, options, sample_rate);
    LEVEL_MATCH_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some(gain) = cache.get(key) {
            return gain;
        }
        let gain = calculate_static_level_match(mode, depth, options, sample_rate);
        cache.insert(key, gain);
        gain
    })
}

fn calculate_static_level_match(
    mode: WaveshaperMode,
    depth: f32,
    options: Options,
    sample_rate: u32,
) -> f32 {
    // Estimate a fixed sine-reference RMS ratio through the native kernel and
    // optional curve. It is computed only when controls change; no signal
    // envelope or runtime input level feeds this gain, so it is not AGC.
    let mut reference = CleanRadias::new(sample_rate);
    reference.configure(mode, depth);
    reference.set_pitch(60.0);
    let input_gain = 10.0_f32.powf((options.input - 0.5) * 48.0 / 20.0);
    let bias = (options.bias - 0.5) * 0.5;
    let mut input_power = 0.0_f64;
    let mut output_power = 0.0_f64;
    let mut output_mean = 0.0_f64;
    const COUNT: usize = 1024;
    const WARM: usize = 128;
    for index in 0..COUNT {
        let input =
            0.2 * (std::f32::consts::TAU * 997.0 * index as f32 / sample_rate.max(1) as f32).sin();
        let driven = safe_audio(input * input_gain + bias);
        let base = reference.process_host([driven, driven], 1.0)[0];
        let output = transfer(options, base);
        if index >= WARM {
            input_power += (input as f64).powi(2);
            output_power += (output as f64).powi(2);
            output_mean += output as f64;
        }
    }
    let count = (COUNT - WARM) as f64;
    let variance = (output_power / count - (output_mean / count).powi(2)).max(1.0e-9);
    let input_rms = (input_power / count).sqrt();
    (input_rms / variance.sqrt()).clamp(0.1, 4.0) as f32
}

fn finite_or_zero(value: f32) -> f32 {
    if value.is_finite() { value } else { 0.0 }
}

#[inline]
fn external_envelope_gain(amount: f32, value: f32) -> f32 {
    2.0_f32.powf((amount - 0.5) * 4.0 * value)
}

#[inline]
fn safe_audio(value: f32) -> f32 {
    finite_or_zero(value).clamp(-8.0, 8.0)
}

#[derive(Clone, Copy)]
struct Up2 {
    history: [f32; HALF_TAPS],
    cursor: usize,
}

impl Default for Up2 {
    fn default() -> Self {
        Self {
            history: [0.0; HALF_TAPS],
            cursor: 0,
        }
    }
}

impl Up2 {
    #[inline]
    fn push(&mut self, sample: f32) -> [f32; 2] {
        self.history[self.cursor] = sample;
        self.cursor = (self.cursor + 1) % HALF_TAPS;

        // Even phase has only the halfband center tap. Odd phase uses the
        // remaining non-zero taps, with the ×2 zero-stuffing compensation.
        let even = self.lag(FIR_CENTER / 2);
        let mut odd = 0.0;
        for j in 0..HALF_TAPS {
            odd += (2.0 * HALF_BAND[2 * j + 1]) * self.lag(j);
        }
        [even, odd]
    }

    #[inline]
    fn lag(&self, lag: usize) -> f32 {
        let index = (self.cursor + HALF_TAPS - 1 - lag) % HALF_TAPS;
        self.history[index]
    }
}

#[derive(Clone, Copy)]
struct Down2 {
    history: [f32; FIR_TAPS],
    cursor: usize,
    phase: bool,
}

impl Default for Down2 {
    fn default() -> Self {
        Self {
            history: [0.0; FIR_TAPS],
            cursor: 0,
            phase: false,
        }
    }
}

impl Down2 {
    #[inline]
    fn push(&mut self, sample: f32) -> Option<f32> {
        self.history[self.cursor] = sample;
        self.cursor += 1;
        if self.cursor == FIR_TAPS {
            self.cursor = 0;
        }
        let output = if !self.phase {
            let mut sum = 0.0;
            // Halfband even taps are zero except the center. Keep the
            // nonzero additions in their original order so the FIR's PCM,
            // phase and latency stay unchanged.
            for lag in (1..FIR_CENTER).step_by(2) {
                let index = (self.cursor + FIR_TAPS - 1 - lag) % FIR_TAPS;
                sum += HALF_BAND[lag] * self.history[index];
            }
            let index = (self.cursor + FIR_TAPS - 1 - FIR_CENTER) % FIR_TAPS;
            sum += HALF_BAND[FIR_CENTER] * self.history[index];
            for lag in (FIR_CENTER + 1..FIR_TAPS).step_by(2) {
                let index = (self.cursor + FIR_TAPS - 1 - lag) % FIR_TAPS;
                sum += HALF_BAND[lag] * self.history[index];
            }
            Some(sum)
        } else {
            None
        };
        self.phase = !self.phase;
        output
    }
}

#[derive(Clone, Copy, Default)]
struct ChannelOversampler {
    up1: Up2,
    down1: Down2,
    up2: Up2,
    down2: Down2,
}

#[derive(Clone, Copy, Default)]
struct Oversampler {
    channels: [ChannelOversampler; 2],
}

impl Oversampler {
    fn clear(&mut self) {
        *self = Self::default();
    }

    #[inline]
    fn upsample(&mut self, frame: Stereo, factor: usize) -> [[f32; 4]; 2] {
        let mut output = [[0.0; 4]; 2];
        for channel in 0..2 {
            let pair = self.channels[channel].up1.push(frame[channel]);
            if factor == 2 {
                output[channel][0] = pair[0];
                output[channel][1] = pair[1];
            } else {
                for half_phase in 0..2 {
                    let quad = self.channels[channel].up2.push(pair[half_phase]);
                    output[channel][2 * half_phase] = quad[0];
                    output[channel][2 * half_phase + 1] = quad[1];
                }
            }
        }
        output
    }

    #[inline]
    fn downsample(&mut self, frame: &[[f32; 4]; 2], factor: usize) -> Stereo {
        let mut result = [0.0; 2];
        for channel in 0..2 {
            if factor == 2 {
                for &sample in frame[channel].iter().take(2) {
                    if let Some(value) = self.channels[channel].down1.push(sample) {
                        result[channel] = value;
                    }
                }
            } else {
                for &sample in frame[channel].iter().take(4) {
                    if let Some(mid) = self.channels[channel].down2.push(sample)
                        && let Some(value) = self.channels[channel].down1.push(mid)
                    {
                        result[channel] = value;
                    }
                }
            }
        }
        result
    }
}

/// The clean kernel adapter uses one state per host-time interpolation phase
/// for Drive/Pickup, while SubOSC advances one state at the oversampled rate.
struct CleanRadias {
    states: [Waveshaper; 8],
    tables: &'static ShaperTables,
    mode: WaveshaperMode,
    current: ShaperCoefficients,
    target: ShaperCoefficients,
    pitch: u16,
    host_increment: PhaseIncrement,
    high_increment: PhaseIncrement,
    sample_rate: f64,
    factor: usize,
    note: f32,
    frame: u8,
}

impl CleanRadias {
    fn new(sample_rate: u32) -> Self {
        let current = radias::coefficients(WaveshaperMode::Drive, 0.5, 60 * 256);
        let mut result = Self {
            states: [Waveshaper::default(); 8],
            tables: radias::shared_tables(),
            mode: WaveshaperMode::Drive,
            current,
            target: current,
            pitch: 60 * 256,
            host_increment: PhaseIncrement(0),
            high_increment: PhaseIncrement(0),
            sample_rate: sample_rate.max(1) as f64,
            factor: 4,
            note: 60.0,
            frame: 0,
        };
        result.set_pitch(60.0);
        result
    }

    fn configure(&mut self, mode: WaveshaperMode, depth: f32) {
        self.target = radias::coefficients(mode, depth, self.pitch);
        if self.mode != mode {
            self.mode = mode;
            self.clear();
        }
        if let (
            ShaperCoefficients::SubOscillator(current),
            ShaperCoefficients::SubOscillator(target),
        ) = (&mut self.current, self.target)
        {
            current.target_depth = target.target_depth;
        }
    }

    fn clear(&mut self) {
        self.states = [Waveshaper::default(); 8];
        self.current = self.target;
        if !matches!(self.current, ShaperCoefficients::Pickup(_)) {
            self.current.set_gain_current(0);
        }
        self.frame = 0;
    }

    fn set_pitch(&mut self, note: f32) {
        self.note = if note.is_finite() { note } else { 60.0 };
        let (pitch, host_increment) = radias::pitch(self.note, self.sample_rate);
        let (_, high_increment) = radias::pitch(self.note, self.sample_rate * self.factor as f64);
        self.pitch = pitch;
        self.host_increment = host_increment;
        self.high_increment = high_increment;
    }

    fn set_factor(&mut self, factor: usize) {
        if self.factor != factor {
            self.factor = factor;
            self.set_pitch(self.note);
        }
    }

    fn process_high(
        &mut self,
        frame: &[[f32; 4]; 2],
        factor: usize,
        voice_gain: f32,
    ) -> [[f32; 4]; 2] {
        let mut result = [[0.0; 4]; 2];
        for channel in 0..2 {
            for phase in 0..factor {
                let mut coefficients = self.current;
                if let ShaperCoefficients::SubOscillator(c) = &mut coefficients {
                    c.gain_current = (c.gain_current as f32 * voice_gain.clamp(0.0, 1.0))
                        .round()
                        .clamp(i16::MIN as f32, i16::MAX as f32)
                        as i16;
                }
                let state_phase = if matches!(self.current, ShaperCoefficients::SubOscillator(_)) {
                    0
                } else {
                    phase
                };
                result[channel][phase] = radias::shape(
                    &mut self.states[channel * 4 + state_phase],
                    frame[channel][phase],
                    coefficients,
                    (self.pitch, self.high_increment),
                    self.tables,
                );
            }
        }
        self.advance_host_control();
        result
    }

    fn process_host(&mut self, frame: Stereo, voice_gain: f32) -> Stereo {
        let mut coefficients = self.current;
        if let ShaperCoefficients::SubOscillator(c) = &mut coefficients {
            c.gain_current = (c.gain_current as f32 * voice_gain.clamp(0.0, 1.0))
                .round()
                .clamp(i16::MIN as f32, i16::MAX as f32) as i16;
        }
        let result = std::array::from_fn(|channel| {
            radias::shape(
                &mut self.states[channel * 4],
                frame[channel],
                coefficients,
                (self.pitch, self.host_increment),
                self.tables,
            )
        });
        self.advance_host_control();
        result
    }

    fn advance_host_control(&mut self) {
        if self.frame & 3 == 3 {
            let gain = self.current.gain_target(self.pitch);
            self.current
                .set_depth(slew(self.current.depth(), self.target.depth()));
            if let (Some(current), Some(target)) = (self.current.gain_current(), gain) {
                self.current.set_gain_current(slew(current, target));
            }
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

fn slew(current: i16, target: i16) -> i16 {
    let value = 2 * (target as i64 * 0x1d4 + current as i64 * 0x7e2d);
    (value.clamp(i32::MIN as i64, i32::MAX as i64) >> 16) as i16
}

// Derived controls change only when their smoothed input changes. Retain
// the existing per-sample slew without repeating exponentials at rest.
struct Coefficients {
    input_gain: f32,
    output_gain: f32,
    attack: f32,
    release: f32,
    pre_tone: f32,
    post_tone: f32,
    bass: f32,
    dc: f32,
}

impl Coefficients {
    fn new(options: Options, sample_rate: u32) -> Self {
        let mut result = Self {
            input_gain: 0.0,
            output_gain: 0.0,
            attack: 0.0,
            release: 0.0,
            pre_tone: 0.0,
            post_tone: 0.0,
            bass: 0.0,
            dc: (-std::f32::consts::TAU * 5.0 / sample_rate.max(1) as f32).exp(),
        };
        let previous = Options {
            input: f32::NAN,
            output: f32::NAN,
            attack: f32::NAN,
            release: f32::NAN,
            pre_tone: f32::NAN,
            post_tone: f32::NAN,
            bass_frequency: f32::NAN,
            ..options
        };
        result.update(&previous, &options, sample_rate);
        result
    }

    fn update(&mut self, previous: &Options, options: &Options, sample_rate: u32) {
        if previous.input != options.input {
            let input_db = (options.input - 0.5) * 48.0;
            self.input_gain = 10.0_f32.powf(input_db / 20.0);
        }
        if previous.output != options.output {
            self.output_gain = 10.0_f32.powf((options.output - 0.5) * 48.0 / 20.0);
        }
        if previous.attack != options.attack {
            let ms = 1.0 + options.attack * options.attack * 199.0;
            self.attack = (-1.0 / (ms * 0.001 * sample_rate as f32)).exp();
        }
        if previous.release != options.release {
            let ms = 10.0 + options.release * options.release * 990.0;
            self.release = (-1.0 / (ms * 0.001 * sample_rate as f32)).exp();
        }
        if previous.pre_tone != options.pre_tone {
            self.pre_tone = tone_coefficient(options.pre_tone, sample_rate);
        }
        if previous.post_tone != options.post_tone {
            self.post_tone = tone_coefficient(options.post_tone, sample_rate);
        }
        if previous.bass_frequency != options.bass_frequency {
            self.bass = bass_coefficient(options.bass_frequency, sample_rate);
        }
    }
}

/// Full per-voice processor. Its fixed FIR histories allocate nothing in
/// `process` or `configure`; the largest path has a 48-sample integer latency.
pub struct Processor {
    sample_rate: u32,
    shaper: RadiasWaveshaper,
    clean: CleanRadias,
    mode: WaveshaperMode,
    depth: f32,
    options: Options,
    target_options: Options,
    coefficients: Coefficients,
    external_envelope: f32,
    options_slew_left: usize,
    curve_from: [Curve; 2],
    curve_slew_left: usize,
    quality: Quality,
    oversampler: Oversampler,
    dry_delay: [Stereo; DELAY_CAPACITY],
    bass_delay: [Stereo; DELAY_CAPACITY],
    delay_cursor: usize,
    pre_tone_state: Stereo,
    post_tone_state: Stereo,
    bass_input_state: Stereo,
    bass_wet_state: Stereo,
    envelope_state: Stereo,
    dc_previous_input: Stereo,
    dc_previous_output: Stereo,
    auto_gain_current: f32,
    auto_gain_target: f32,
    auto_gain_slew_left: usize,
    mix_current: f32,
    mix_target: f32,
    mix_slew_left: usize,
    mix_initialized: bool,
    transition_from: Stereo,
    transition_left: usize,
    transition_total: usize,
    transition_from_latency: usize,
    last_wet: Stereo,
    has_processed: bool,
}

impl Processor {
    pub fn new(sample_rate: u32) -> Self {
        let sample_rate = sample_rate.max(1);
        Self {
            sample_rate,
            shaper: RadiasWaveshaper::new(sample_rate),
            clean: CleanRadias::new(sample_rate),
            mode: WaveshaperMode::Drive,
            depth: 0.5,
            options: Options::default(),
            target_options: Options::default(),
            coefficients: Coefficients::new(Options::default(), sample_rate),
            external_envelope: 0.0,
            options_slew_left: 0,
            curve_from: [Curve::Radias, Curve::SoftClip],
            curve_slew_left: 0,
            quality: Quality::Legacy,
            oversampler: Oversampler::default(),
            dry_delay: [[0.0; 2]; DELAY_CAPACITY],
            bass_delay: [[0.0; 2]; DELAY_CAPACITY],
            delay_cursor: 0,
            pre_tone_state: [0.0; 2],
            post_tone_state: [0.0; 2],
            bass_input_state: [0.0; 2],
            bass_wet_state: [0.0; 2],
            envelope_state: [0.0; 2],
            dc_previous_input: [0.0; 2],
            dc_previous_output: [0.0; 2],
            auto_gain_current: 1.0,
            auto_gain_target: 1.0,
            auto_gain_slew_left: 0,
            mix_current: 1.0,
            mix_target: 1.0,
            mix_slew_left: 0,
            mix_initialized: false,
            transition_from: [0.0; 2],
            transition_left: 0,
            transition_total: 0,
            transition_from_latency: 0,
            last_wet: [0.0; 2],
            has_processed: false,
        }
    }

    pub fn clear(&mut self) {
        self.shaper.clear();
        self.clean.clear();
        self.oversampler.clear();
        self.dry_delay = [[0.0; 2]; DELAY_CAPACITY];
        self.bass_delay = [[0.0; 2]; DELAY_CAPACITY];
        self.delay_cursor = 0;
        self.pre_tone_state = [0.0; 2];
        self.post_tone_state = [0.0; 2];
        self.bass_input_state = [0.0; 2];
        self.bass_wet_state = [0.0; 2];
        self.envelope_state = [0.0; 2];
        self.dc_previous_input = [0.0; 2];
        self.dc_previous_output = [0.0; 2];
        self.auto_gain_current = self.auto_gain_target;
        self.auto_gain_slew_left = 0;
        self.transition_left = 0;
        self.last_wet = [0.0; 2];
        self.has_processed = false;
        self.mix_current = 1.0;
        self.mix_target = 1.0;
        self.mix_slew_left = 0;
        self.mix_initialized = false;
    }

    pub fn configure(&mut self, mode: WaveshaperMode, depth: f32, options: Options) {
        let options = options.sanitize();
        let mode_changed = self.mode != mode;
        let quality_changed = self.quality != options.quality;
        let next_depth = if depth.is_finite() {
            depth.clamp(0.0, 1.0)
        } else {
            0.5
        };
        let depth_changed = self.depth != next_depth;
        let changed = mode_changed || quality_changed;
        if changed {
            self.transition_from = self.last_wet;
            self.transition_from_latency = quality_latency(self.quality);
            if self.has_processed {
                self.transition_total = self.transition_samples();
                self.transition_left = self.transition_total;
            } else {
                self.transition_total = 0;
                self.transition_left = 0;
            }
        }
        if self.target_options.curve != options.curve
            || self.target_options.curve_b != options.curve_b
        {
            self.curve_from = [self.options.curve, self.options.curve_b];
            self.curve_slew_left = self.transition_samples();
        }
        if self.target_options != options || mode_changed || depth_changed {
            self.options_slew_left = self.transition_samples();
            self.auto_gain_target = static_level_match(mode, next_depth, options, self.sample_rate);
            self.auto_gain_slew_left = self.transition_samples();
        }
        if self.options.envelope_source != options.envelope_source {
            self.external_envelope = 0.0;
        }
        // Envelope source is a structural selector. Publish it immediately;
        // the follower amount and all audio trims keep their normal slew.
        self.options.envelope_source = options.envelope_source;
        self.mode = mode;
        self.depth = next_depth;
        self.target_options = options;
        self.quality = options.quality;
        if mode_changed || quality_changed {
            self.oversampler.clear();
            self.clean.clear();
        }
        self.clean.set_factor(match self.quality {
            Quality::Legacy | Quality::Clean2x => 2,
            Quality::Clean4x => 4,
        });
        self.shaper.configure(mode, self.depth);
        self.clean.configure(mode, self.depth);
    }

    /// Start a fresh note with no filter, curve, FIR, envelope, or mode state
    /// inherited from the previous occupant of this processor.
    pub fn reset_for_note(
        &mut self,
        mode: WaveshaperMode,
        depth: f32,
        options: Options,
        note: f32,
    ) {
        let options = options.sanitize();
        let depth = if depth.is_finite() {
            depth.clamp(0.0, 1.0)
        } else {
            0.5
        };
        self.clear();
        self.mode = mode;
        self.depth = depth;
        self.options = options;
        self.target_options = options;
        self.coefficients = Coefficients::new(options, self.sample_rate);
        self.external_envelope = 0.0;
        self.options_slew_left = 0;
        self.curve_from = [options.curve, options.curve_b];
        self.curve_slew_left = 0;
        self.quality = options.quality;
        self.transition_from_latency = 0;
        self.transition_left = 0;
        self.transition_total = 0;
        self.transition_from = [0.0; 2];
        self.last_wet = [0.0; 2];
        self.has_processed = false;
        self.auto_gain_current = static_level_match(mode, depth, options, self.sample_rate);
        self.auto_gain_target = self.auto_gain_current;
        self.auto_gain_slew_left = 0;
        self.shaper = RadiasWaveshaper::new(self.sample_rate);
        self.shaper.configure(mode, depth);
        self.clean = CleanRadias::new(self.sample_rate);
        self.clean.set_factor(match options.quality {
            Quality::Legacy | Quality::Clean2x => 2,
            Quality::Clean4x => 4,
        });
        self.clean.configure(mode, depth);
        self.shaper.set_pitch(note);
        self.clean.set_pitch(note);
        self.mix_current = 1.0;
        self.mix_target = 1.0;
        self.mix_slew_left = 0;
        self.mix_initialized = false;
    }

    pub fn set_pitch(&mut self, note: f32) {
        self.shaper.set_pitch(note);
        self.clean.set_pitch(note);
    }

    /// Resolve the selected external envelope now and retain only one scalar.
    /// Call on the control update that already owns this processor's envelopes.
    pub fn set_envelopes(&mut self, envelopes: &[f32]) {
        self.external_envelope = self.options.envelope_source.map_or(0.0, |source| {
            finite_or_zero(source.value(envelopes)).clamp(0.0, 1.0)
        });
    }

    pub fn pitch_code(&self) -> u16 {
        self.shaper.pitch_code()
    }

    pub fn latency_samples(&self) -> usize {
        let target = quality_latency(self.quality);
        if self.transition_left > 0 {
            target.max(self.transition_from_latency)
        } else {
            target
        }
    }

    pub fn process(&mut self, frame: Stereo) -> Stereo {
        self.process_with_voice_gain(frame, 1.0)
    }

    pub fn process_with_voice_gain(&mut self, frame: Stereo, voice_gain: f32) -> Stereo {
        self.process_frame(frame, voice_gain).0
    }

    pub fn process_mixed(&mut self, frame: Stereo, mix: f32, voice_gain: f32) -> Stereo {
        let mix = if mix.is_finite() {
            mix.clamp(0.0, 1.0)
        } else {
            1.0
        };
        if !self.mix_initialized {
            self.mix_current = mix;
            self.mix_target = mix;
            self.mix_initialized = true;
        } else if mix != self.mix_target {
            self.mix_target = mix;
            self.mix_slew_left = self.transition_samples();
        }
        if self.mix_slew_left > 0 {
            self.mix_current += (self.mix_target - self.mix_current) / self.mix_slew_left as f32;
            self.mix_slew_left -= 1;
            if self.mix_slew_left == 0 {
                self.mix_current = self.mix_target;
            }
        }
        self.process_mixed_fixed(frame, self.mix_current, voice_gain)
    }

    fn process_mixed_fixed(&mut self, frame: Stereo, mix: f32, voice_gain: f32) -> Stereo {
        let (wet, dry) = self.process_frame(frame, voice_gain);
        blend(dry, wet, mix)
    }

    fn process_frame(&mut self, frame: Stereo, voice_gain: f32) -> (Stereo, Stereo) {
        self.advance_options();
        let delay = quality_latency(self.quality);
        // The dry branch retains the host's headroom. Apply safety limiting
        // only in the wet front end; Mix=0 must remain a true finite bypass.
        let safe_frame = frame.map(finite_or_zero);
        self.dry_delay[self.delay_cursor] = safe_frame;
        let delayed_dry_index = (self.delay_cursor + DELAY_CAPACITY - delay) % DELAY_CAPACITY;
        let delayed_dry = self.dry_delay[delayed_dry_index];

        if self.quality == Quality::Legacy
            && self.options.is_legacy_default()
            && self.target_options.is_legacy_default()
        {
            let mut wet = self.shaper.process_with_voice_gain(frame, voice_gain);
            wet = self.apply_transition(wet);
            self.delay_cursor = (self.delay_cursor + 1) % DELAY_CAPACITY;
            self.last_wet = wet;
            self.has_processed = true;
            return (wet, delayed_dry);
        }

        let front = self.apply_front_end(safe_frame);
        for (channel, front_sample) in front.into_iter().enumerate() {
            self.bass_input_state[channel] = one_pole(
                front_sample,
                self.bass_input_state[channel],
                self.coefficients.bass,
            );
        }
        self.bass_delay[self.delay_cursor] = self.bass_input_state;
        let delayed_bass_index = (self.delay_cursor + DELAY_CAPACITY - delay) % DELAY_CAPACITY;
        let delayed_bass = self.bass_delay[delayed_bass_index];

        let mut wet = if self.quality == Quality::Legacy {
            self.shaper.process_with_voice_gain(front, voice_gain)
        } else {
            self.process_clean(front, voice_gain)
        };

        if self.quality == Quality::Legacy {
            wet = self.apply_curve_host(wet);
        }
        wet.iter_mut()
            .for_each(|sample| *sample *= self.auto_gain_current);
        wet = self.apply_post_end(wet, delayed_bass);
        wet = self.apply_transition(wet);
        self.delay_cursor = (self.delay_cursor + 1) % DELAY_CAPACITY;
        self.last_wet = wet;
        self.has_processed = true;
        (wet, delayed_dry)
    }

    fn process_clean(&mut self, input: Stereo, voice_gain: f32) -> Stereo {
        let factor = match self.quality {
            Quality::Clean2x => 2,
            Quality::Clean4x => 4,
            Quality::Legacy => return self.shaper.process_with_voice_gain(input, voice_gain),
        };
        let is_host_clocked = self.mode == WaveshaperMode::Decimator;
        let source = if is_host_clocked {
            // The native decimator is a host-clocked effect; keep its update
            // count unchanged, then oversample the added static curve layer.
            self.clean.process_host(input, voice_gain)
        } else {
            input
        };
        let oversampled = self.oversampler.upsample(source, factor);
        let mut shaped = if is_host_clocked {
            oversampled
        } else {
            self.clean.process_high(&oversampled, factor, voice_gain)
        };

        let curve_mix = self.curve_mix();
        for channel in &mut shaped {
            for sample in channel.iter_mut().take(factor) {
                *sample = self.apply_curve_sample(*sample, curve_mix);
            }
        }
        self.finish_curve_frame();

        self.oversampler.downsample(&shaped, factor)
    }

    fn apply_curve_host(&mut self, frame: Stereo) -> Stereo {
        let mix = self.curve_mix();
        let result = std::array::from_fn(|channel| self.apply_curve_sample(frame[channel], mix));
        self.finish_curve_frame();
        result
    }

    fn curve_mix(&self) -> f32 {
        if self.curve_slew_left == 0 {
            1.0
        } else {
            1.0 - self.curve_slew_left as f32 / self.transition_samples().max(1) as f32
        }
    }

    fn apply_curve_sample(&self, x: f32, curve_mix: f32) -> f32 {
        let old_a = curve_transfer(
            self.curve_from[0],
            x,
            self.options.knee,
            &self.options.points,
        );
        let old_b = curve_transfer(
            self.curve_from[1],
            x,
            self.options.knee,
            &self.options.points,
        );
        let new_a = curve_transfer(
            self.target_options.curve,
            x,
            self.options.knee,
            &self.options.points,
        );
        let new_b = curve_transfer(
            self.target_options.curve_b,
            x,
            self.options.knee,
            &self.options.points,
        );
        let a = old_a + (new_a - old_a) * curve_mix;
        let b = old_b + (new_b - old_b) * curve_mix;
        a + (b - a) * self.options.morph
    }

    fn finish_curve_frame(&mut self) {
        if self.curve_slew_left > 0 {
            self.curve_slew_left -= 1;
            if self.curve_slew_left == 0 {
                self.curve_from = [self.target_options.curve, self.target_options.curve_b];
            }
        }
    }

    fn apply_front_end(&mut self, input: Stereo) -> Stereo {
        let options = self.options;
        let bias = (options.bias - 0.5) * 0.5;
        let external_gain = options
            .envelope_source
            .map(|_| external_envelope_gain(options.envelope, self.external_envelope));
        std::array::from_fn(|channel| {
            let raw = safe_audio(input[channel] * self.coefficients.input_gain + bias);
            let magnitude = raw.abs();
            let coefficient = if magnitude > self.envelope_state[channel] {
                self.coefficients.attack
            } else {
                self.coefficients.release
            };
            self.envelope_state[channel] = (magnitude
                + coefficient * (self.envelope_state[channel] - magnitude))
                .clamp(0.0, 8.0);
            let envelope_gain = if let Some(gain) = external_gain {
                gain
            } else {
                let envelope_octaves = (options.envelope - 0.5) * 4.0;
                2.0_f32.powf(envelope_octaves * (0.5 - self.envelope_state[channel]))
            };
            let driven = raw * envelope_gain;
            self.pre_tone_state[channel] = one_pole(
                driven,
                self.pre_tone_state[channel],
                self.coefficients.pre_tone,
            );
            self.pre_tone_state[channel]
                + (driven - self.pre_tone_state[channel]) * options.pre_tone
        })
    }

    fn apply_post_end(&mut self, wet: Stereo, delayed_bass: Stereo) -> Stereo {
        let options = self.options;
        let mut result = std::array::from_fn(|channel| {
            let sample = safe_audio(wet[channel]);
            self.post_tone_state[channel] = one_pole(
                sample,
                self.post_tone_state[channel],
                self.coefficients.post_tone,
            );
            let tone = self.post_tone_state[channel]
                + (sample - self.post_tone_state[channel]) * options.post_tone;
            let low = one_pole(tone, self.bass_wet_state[channel], self.coefficients.bass);
            self.bass_wet_state[channel] = low;
            let preserved = tone + options.bass * (delayed_bass[channel] - low);
            let dc = if options.dc_block {
                let value = preserved - self.dc_previous_input[channel]
                    + self.coefficients.dc * self.dc_previous_output[channel];
                self.dc_previous_input[channel] = preserved;
                self.dc_previous_output[channel] = value;
                value
            } else {
                self.dc_previous_input[channel] = preserved;
                self.dc_previous_output[channel] = preserved;
                preserved
            };
            dc * self.coefficients.output_gain
        });
        for sample in &mut result {
            if !sample.is_finite() {
                *sample = 0.0;
            }
        }
        result
    }

    fn apply_transition(&mut self, wet: Stereo) -> Stereo {
        if self.transition_left == 0 {
            return wet;
        }
        let mix = 1.0 - self.transition_left as f32 / self.transition_total.max(1) as f32;
        let output = blend(self.transition_from, wet, mix);
        self.transition_left -= 1;
        output
    }

    fn advance_options(&mut self) {
        if self.auto_gain_slew_left > 0 {
            self.auto_gain_current +=
                (self.auto_gain_target - self.auto_gain_current) / self.auto_gain_slew_left as f32;
            self.auto_gain_slew_left -= 1;
            if self.auto_gain_slew_left == 0 {
                self.auto_gain_current = self.auto_gain_target;
            }
        }
        if self.options_slew_left == 0 {
            return;
        }
        let left = self.options_slew_left as f32;
        let previous = self.options;
        let target = self.target_options;
        self.options.morph = step(self.options.morph, target.morph, left);
        self.options.input = step(self.options.input, target.input, left);
        self.options.output = step(self.options.output, target.output, left);
        self.options.knee = step(self.options.knee, target.knee, left);
        self.options.bias = step(self.options.bias, target.bias, left);
        self.options.pre_tone = step(self.options.pre_tone, target.pre_tone, left);
        self.options.post_tone = step(self.options.post_tone, target.post_tone, left);
        self.options.bass = step(self.options.bass, target.bass, left);
        self.options.bass_frequency =
            step(self.options.bass_frequency, target.bass_frequency, left);
        self.options.envelope = step(self.options.envelope, target.envelope, left);
        self.options.attack = step(self.options.attack, target.attack, left);
        self.options.release = step(self.options.release, target.release, left);
        for i in 0..17 {
            self.options.points[i] = step(self.options.points[i], target.points[i], left);
        }
        self.options.dc_block = target.dc_block;
        self.options.auto_gain = target.auto_gain;
        self.options.quality = target.quality;
        self.options_slew_left -= 1;
        if self.options_slew_left == 0 {
            self.options = target;
        }
        self.coefficients
            .update(&previous, &self.options, self.sample_rate);
    }

    fn transition_samples(&self) -> usize {
        ((self.sample_rate as f32 * SMOOTH_SECONDS).round() as usize).max(1)
    }
}

fn step(current: f32, target: f32, left: f32) -> f32 {
    current + (target - current) / left.max(1.0)
}

fn quality_latency(quality: Quality) -> usize {
    match quality {
        Quality::Legacy => 0,
        Quality::Clean2x => FIR_CENTER,
        Quality::Clean4x => MAX_LATENCY_SAMPLES,
    }
}

#[inline]
fn blend(dry: Stereo, wet: Stereo, mix: f32) -> Stereo {
    let mix = mix.clamp(0.0, 1.0);
    if mix <= 0.0 {
        dry
    } else if mix >= 1.0 {
        wet
    } else {
        // Preserve the released ColorProcessor's arithmetic as well as its
        // endpoints: reassociating this sum changes Legacy PCM by an ulp.
        std::array::from_fn(|channel| dry[channel] * (1.0 - mix) + mix * wet[channel])
    }
}

#[inline]
fn one_pole(input: f32, state: f32, coefficient: f32) -> f32 {
    state + coefficient * (input - state)
}

fn tone_coefficient(tone: f32, sample_rate: u32) -> f32 {
    let tone = tone.clamp(0.0, 1.0);
    let cutoff = 80.0 + tone * tone * 19_920.0;
    1.0 - (-std::f32::consts::TAU * cutoff / sample_rate.max(1) as f32).exp()
}

fn bass_coefficient(position: f32, sample_rate: u32) -> f32 {
    let frequency = 40.0 * 2.0_f32.powf(position.clamp(0.0, 1.0) * 4.0);
    1.0 - (-std::f32::consts::TAU * frequency / sample_rate.max(1) as f32).exp()
}

/// A per-filter shaper stage. Disabled stages return their input without
/// touching the processor; enable/disable and mix changes fade over 4 ms.
pub struct StageProcessor {
    processor: Processor,
    stage: Stage,
    enabled_mix: f32,
    enabled_target: f32,
    enabled_slew_left: usize,
    stage_mix: f32,
    stage_mix_target: f32,
    stage_mix_slew_left: usize,
    configured: bool,
}

impl StageProcessor {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            processor: Processor::new(sample_rate),
            stage: Stage::default(),
            enabled_mix: 0.0,
            enabled_target: 0.0,
            enabled_slew_left: 0,
            stage_mix: 1.0,
            stage_mix_target: 1.0,
            stage_mix_slew_left: 0,
            configured: false,
        }
    }

    pub fn configure(&mut self, stage: Stage, note: f32) {
        let stage = stage.sanitize();
        let next_enabled = if stage.enabled { 1.0 } else { 0.0 };
        if self.configured && self.stage.enabled != stage.enabled {
            self.enabled_target = next_enabled;
            self.enabled_slew_left =
                ((self.processor.sample_rate as f32 * SMOOTH_SECONDS).round() as usize).max(1);
        } else if !self.configured {
            self.enabled_mix = next_enabled;
            self.enabled_target = next_enabled;
        }
        if !self.configured {
            self.stage_mix = stage.mix;
            self.stage_mix_target = stage.mix;
        } else if self.stage_mix_target != stage.mix {
            self.stage_mix_target = stage.mix;
            self.stage_mix_slew_left =
                ((self.processor.sample_rate as f32 * SMOOTH_SECONDS).round() as usize).max(1);
        }
        if stage.enabled {
            let mut options = stage.options;
            if stage.keytrack != 0.0 && note.is_finite() && note != 60.0 {
                // The existing input trim spans 48 dB over [0, 1]. Keytrack
                // applies 6.0206 dB per octave around MIDI note 60 without
                // modifying the saved trim value.
                let octaves = (note - 60.0) / 12.0;
                options.input =
                    (options.input + stage.keytrack * octaves * (6.0206 / 48.0)).clamp(0.0, 1.0);
            }
            self.processor.configure(stage.mode, stage.depth, options);
            self.processor.set_pitch(note);
        }
        self.stage = stage;
        self.configured = true;
    }

    pub fn set_envelopes(&mut self, envelopes: &[f32]) {
        self.processor.set_envelopes(envelopes);
    }

    pub fn clear(&mut self) {
        self.processor.clear();
        self.enabled_mix = if self.stage.enabled { 1.0 } else { 0.0 };
        self.enabled_target = self.enabled_mix;
        self.enabled_slew_left = 0;
        self.stage_mix = self.stage.mix;
        self.stage_mix_target = self.stage.mix;
        self.stage_mix_slew_left = 0;
    }

    pub fn latency_samples(&self) -> usize {
        if self.stage.enabled || self.enabled_mix > 0.0 || self.enabled_slew_left > 0 {
            self.processor.latency_samples()
        } else {
            0
        }
    }

    pub fn process(&mut self, frame: Stereo) -> Stereo {
        self.process_with_voice_gain(frame, 1.0)
    }

    pub fn process_with_voice_gain(&mut self, frame: Stereo, voice_gain: f32) -> Stereo {
        if !self.stage.enabled && self.enabled_mix <= 0.0 && self.enabled_slew_left == 0 {
            return frame;
        }
        if self.enabled_slew_left > 0 {
            self.enabled_mix +=
                (self.enabled_target - self.enabled_mix) / self.enabled_slew_left as f32;
            self.enabled_slew_left -= 1;
            if self.enabled_slew_left == 0 {
                self.enabled_mix = self.enabled_target;
            }
        }
        if self.stage_mix_slew_left > 0 {
            self.stage_mix +=
                (self.stage_mix_target - self.stage_mix) / self.stage_mix_slew_left as f32;
            self.stage_mix_slew_left -= 1;
            if self.stage_mix_slew_left == 0 {
                self.stage_mix = self.stage_mix_target;
            }
        }
        self.processor.process_mixed_fixed(
            frame,
            (self.stage_mix * self.enabled_mix).clamp(0.0, 1.0),
            voice_gain,
        )
    }
}

// This import makes the model API available from the DSP module for callers
// that already organize stage settings beside their processor.
pub use crate::waveshaper::FilterShapers as FilterShaperSettings;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_transfer_interpolates_in_seventeen_even_steps_and_clamps_ends() {
        let points = std::array::from_fn(|index| index as f32 / 8.0 - 1.0);
        assert_eq!(curve_transfer(Curve::Custom, -2.0, 0.5, &points), -1.0);
        assert_eq!(curve_transfer(Curve::Custom, 2.0, 0.5, &points), 1.0);
        assert!((curve_transfer(Curve::Custom, -0.9375, 0.5, &points) + 0.9375).abs() < 1e-6);
    }

    #[test]
    fn filter_keytrack_is_centered_at_c4_and_updates_when_only_note_changes() {
        let stage = Stage {
            enabled: true,
            mode: WaveshaperMode::Decimator,
            depth: 0.0,
            keytrack: 1.0,
            ..Stage::default()
        };
        let mut processor = StageProcessor::new(48_000);
        processor.configure(stage, 60.0);
        assert_eq!(
            processor.processor.target_options.input,
            stage.options.input
        );
        assert_eq!(processor.stage.options.input, stage.options.input);

        let octave = 6.0206_f32 / 48.0;
        processor.configure(stage, 72.0);
        assert!(
            (processor.processor.target_options.input - (stage.options.input + octave)).abs()
                < 1e-7
        );
        assert_eq!(processor.stage.options.input, stage.options.input);
        processor.configure(stage, 48.0);
        assert!(
            (processor.processor.target_options.input - (stage.options.input - octave)).abs()
                < 1e-7
        );

        let mut untracked = StageProcessor::new(48_000);
        untracked.configure(
            Stage {
                keytrack: 0.0,
                ..stage
            },
            48.0,
        );
        let mut audio_delta = 0.0;
        for index in 0..512 {
            let sample = (std::f32::consts::TAU * index as f32 / 37.0).sin() * 0.18;
            let frame = [sample, -sample];
            let keyed = processor.process(frame);
            let plain = untracked.process(frame);
            audio_delta += (keyed[0] - plain[0]).abs();
        }
        assert_eq!(
            processor.processor.options.input,
            processor.processor.target_options.input
        );
        assert_eq!(
            untracked.processor.options.input,
            untracked.processor.target_options.input
        );
        assert!(
            audio_delta > 0.1,
            "keytrack did not change filter WS PCM: delta={audio_delta}"
        );
    }

    #[test]
    fn filter_waveshaper_keytrack_keeps_mix_zero_as_exact_dry_bypass() {
        let mut processor = StageProcessor::new(48_000);
        processor.configure(
            Stage {
                enabled: true,
                mode: WaveshaperMode::HardClip,
                mix: 0.0,
                keytrack: 1.0,
                ..Stage::default()
            },
            84.0,
        );
        for frame in [[0.125, -0.375], [0.75, 0.25], [-0.9, 0.1]] {
            assert_eq!(processor.process(frame), frame);
        }
    }

    #[test]
    fn shared_sub_oscillator_table_matches_the_original_integer_formula() {
        let tables = radias::shared_tables();
        for index in 0..tables.sub_edges.len() {
            let original = 16_384_usize.checked_div(index).unwrap_or(32_767) as i16;
            assert_eq!(tables.sub_edges[index], original, "table index {index}");
        }
        assert!(std::ptr::eq(tables, radias::shared_tables()));
    }

    #[test]
    fn transition_starts_at_the_previous_output_and_remains_finite() {
        let mut processor = Processor::new(48_000);
        processor.configure(WaveshaperMode::Drive, 0.65, Options::default());
        let mut previous = [0.0; 2];
        for index in 0..256 {
            let value = (index as f32 * 0.03).sin() * 0.6;
            previous = processor.process([value, -value]);
        }
        processor.configure(WaveshaperMode::HardClip, 0.9, Options::default());
        let first = processor.process([0.3, -0.3]);
        assert_eq!(first, previous);
        for _ in 0..256 {
            let output = processor.process([0.3, -0.3]);
            assert!(output.iter().all(|sample| sample.is_finite()));
        }
    }

    #[test]
    fn quality_latencies_are_integer_and_stage_state_fits_fixed_budget() {
        assert_eq!(quality_latency(Quality::Legacy), 0);
        assert_eq!(quality_latency(Quality::Clean2x), 32);
        assert_eq!(quality_latency(Quality::Clean4x), 48);
        let processor_size = std::mem::size_of::<Processor>();
        let size = std::mem::size_of::<StageProcessor>();
        eprintln!("Processor fixed state: {processor_size} bytes; StageProcessor: {size} bytes");
        assert!(
            processor_size < 4096,
            "unexpectedly large processor state: {processor_size} bytes"
        );
        assert!(
            size < 4096,
            "unexpectedly large fixed DSP state: {size} bytes"
        );
    }

    #[test]
    fn repeated_identical_processors_share_one_level_match_calibration() {
        let sample_rate = 47_891;
        let mut points = Options::default().points;
        points[3] = -0.231;
        let options = Options {
            curve: Curve::Custom,
            curve_b: Curve::Diode,
            morph: 0.371,
            input: 0.813,
            bias: 0.147,
            knee: 0.629,
            auto_gain: true,
            points,
            ..Options::default()
        };
        let before = LEVEL_MATCH_CACHE.with(|cache| cache.borrow().calibration_count);
        for _ in 0..64 {
            let mut processor = Processor::new(sample_rate);
            processor.configure(WaveshaperMode::Drive, 0.713, options);
        }
        let after = LEVEL_MATCH_CACHE.with(|cache| cache.borrow().calibration_count);
        assert_eq!(
            after - before,
            1,
            "identical settings recalibrated per processor"
        );
    }

    #[test]
    fn direct_map_collisions_cannot_return_a_different_key_gain() {
        let base_options = Options {
            auto_gain: true,
            input: 0.813,
            ..Options::default()
        };
        let key_a = LevelMatchKey::new(WaveshaperMode::Drive, 0.713, base_options, 47_891);
        let key_b = (1..10_000)
            .map(|step| {
                let options = Options {
                    morph: step as f32 / 10_000.0,
                    ..base_options
                };
                LevelMatchKey::new(WaveshaperMode::Drive, 0.713, options, 47_891)
            })
            .find(|candidate| *candidate != key_a && candidate.slot() == key_a.slot())
            .expect("find a distinct key with the same direct-map slot");

        let mut cache = LevelMatchCache::new();
        cache.insert(key_a, 0.25);
        assert_eq!(cache.get(key_b), None);
        cache.insert(key_b, 0.75);
        assert_eq!(cache.get(key_b), Some(0.75));
        assert_eq!(cache.get(key_a), None);
    }
}
