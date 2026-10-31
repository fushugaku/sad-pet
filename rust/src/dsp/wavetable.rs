//! Bandlimited, multiframe wavetable banks. Preparation is never audio work.
//! Each lookup interpolates periodic samples, adjacent frames and safe mip levels.
mod interaction;
mod lookup;
mod polynomial;
mod transport;
use crate::wavetable::{
    FRAME_SIZE,
    assets::{Asset, Pool},
};
use rustfft::{FftPlanner, num_complex::Complex32};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex, OnceLock, Weak},
};
use transport::Storage;
pub use transport::{BankHeader, BankStage, enter_realtime, reserve_realtime_banks};

pub const HARMONICS: [usize; 10] = [1023, 511, 255, 127, 63, 31, 15, 7, 3, 1];
pub const COMPILER_REVISION: u32 = 2;
pub const LAYOUT_REVISION: u32 = 2;
pub const KERNEL_REVISION: u32 = 1;
const TAPS: usize = 16;
const GUARD: usize = TAPS / 2;
const KERNEL_PHASES: usize = 1024;

struct Kernel {
    phases: Box<[[f32; TAPS]]>,
}
impl Kernel {
    fn shared() -> &'static Self {
        static KERNEL: OnceLock<Kernel> = OnceLock::new();
        KERNEL.get_or_init(|| {
            let mut phases = Vec::with_capacity(KERNEL_PHASES + 1);
            for i in 0..=KERNEL_PHASES {
                let t = i as f64 / KERNEL_PHASES as f64;
                let mut weights = [0.0_f64; TAPS];
                for (tap, weight) in weights.iter_mut().enumerate() {
                    let distance = t - (tap as f64 - (GUARD - 1) as f64);
                    let sinc = if distance.abs() < 1e-12 {
                        1.0
                    } else {
                        let p = std::f64::consts::PI * distance;
                        p.sin() / p
                    };
                    let a = std::f64::consts::PI * distance / GUARD as f64;
                    *weight = sinc * (0.42 + 0.5 * a.cos() + 0.08 * (2.0 * a).cos());
                }
                let sum: f64 = weights.iter().sum();
                phases.push(weights.map(|v| (v / sum) as f32));
            }
            Self {
                phases: phases.into_boxed_slice(),
            }
        })
    }
    #[inline]
    fn position(&self, fraction: f64) -> (usize, f32) {
        let coordinate = fraction * KERNEL_PHASES as f64;
        let index = (coordinate as usize).min(KERNEL_PHASES - 1);
        (index, (coordinate - index as f64) as f32)
    }
}

#[derive(Clone, Copy, Debug)]
struct Level {
    harmonics: usize,
    length: usize,
    stride: usize,
    offset: usize,
    polynomial: usize,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Selection {
    low: usize,
    blend: f32,
}

pub struct Bank {
    pub content_hash: String,
    pub frames: usize,
    levels: [Level; HARMONICS.len()],
    data: Storage,
    kernel: &'static Kernel,
}
impl std::fmt::Debug for Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WavetableBank")
            .field("hash", &self.content_hash)
            .field("frames", &self.frames)
            .field("bytes", &self.bytes())
            .finish()
    }
}

type Cache = BTreeMap<String, Weak<Bank>>;
fn cache_key(hash: &str) -> String {
    format!("{COMPILER_REVISION}:{LAYOUT_REVISION}:{KERNEL_REVISION}:{hash}")
}
fn cache() -> &'static Mutex<Cache> {
    static CACHE: OnceLock<Mutex<Cache>> = OnceLock::new();
    CACHE.get_or_init(Mutex::default)
}
fn bank_layout(frames: usize) -> ([Level; HARMONICS.len()], usize) {
    let mut offset = 0;
    let mut levels = HARMONICS.map(|harmonics| {
        let length = (4 * (harmonics + 1)).max(64);
        let stride = length + 2 * GUARD;
        let level = Level {
            harmonics,
            length,
            stride,
            offset,
            polynomial: usize::MAX,
        };
        offset += stride * frames;
        level
    });
    // At most 4 MiB of additional, derived coefficients per bank. Prefer
    // smaller mips; a bounded sinc fallback handles the remaining levels.
    let mut added = 0;
    for level in levels.iter_mut().rev() {
        let count = level.length * 8 * frames;
        if added + count <= 1024 * 1024 {
            level.polynomial = offset;
            offset += count;
            added += count;
        }
    }
    (levels, offset)
}
impl Bank {
    /// Resolve and reuse a bank on a control/worker thread. Never called by render.
    pub fn resolve(id: &str, assets: &Pool) -> Result<Arc<Self>, String> {
        if let Some(hash) = id.strip_prefix("user:")
            && let Some(bank) = Self::cached(hash)?
        {
            return Ok(bank);
        }
        if let Some(factory) = crate::wavetable::factory::find(id)
            && let Some(bank) = Self::cached(factory.content_hash)?
        {
            return Ok(bank);
        }
        if transport::is_realtime() {
            return Err(
                "The wavetable bank has not finished loading; the current sound was kept".into(),
            );
        }
        let asset = assets.resolve(id)?;
        if let Some(bank) = Self::cached(&asset.content_hash)? {
            return Ok(bank);
        }
        let bank = Arc::new(Self::compile(&asset)?);
        Self::remember(&bank)?;
        Ok(bank)
    }
    pub fn remember(bank: &Arc<Self>) -> Result<(), String> {
        let mut cache = cache()
            .lock()
            .map_err(|_| "Wavetable bank cache is unavailable")?;
        // Dead entries retain no wave buffers, and their keys are bounded too.
        if cache.len() >= 64 {
            cache.retain(|_, bank| bank.strong_count() > 0);
        }
        cache.insert(cache_key(&bank.content_hash), Arc::downgrade(bank));
        Ok(())
    }
    pub fn cached(hash: &str) -> Result<Option<Arc<Self>>, String> {
        Ok(cache()
            .lock()
            .map_err(|_| "Wavetable bank cache is unavailable")?
            .get(&cache_key(hash))
            .and_then(Weak::upgrade))
    }
    pub fn compile(asset: &Asset) -> Result<Self, String> {
        asset.validate()?;
        let (levels, offset) = bank_layout(asset.frame_count);
        let mut data = vec![0.0; offset];
        let mut planner = FftPlanner::<f32>::new();
        let forward = planner.plan_fft_forward(FRAME_SIZE);
        let inverses: Vec<_> = levels
            .iter()
            .map(|l| planner.plan_fft_inverse(l.length))
            .collect();
        let scratch_len = inverses
            .iter()
            .map(|p| p.get_inplace_scratch_len())
            .chain([forward.get_inplace_scratch_len()])
            .max()
            .unwrap();
        let mut scratch = vec![Complex32::default(); scratch_len];
        let mut frequency = vec![Complex32::default(); FRAME_SIZE];
        let mut wave = vec![Complex32::default(); levels[0].length];
        for (frame_index, frame) in asset.samples.chunks_exact(FRAME_SIZE).enumerate() {
            for (to, &x) in frequency.iter_mut().zip(frame) {
                *to = Complex32::new(x, 0.0);
            }
            forward.process_with_scratch(&mut frequency, &mut scratch);
            for (level, inverse) in levels.iter().zip(&inverses) {
                let wave = &mut wave[..level.length];
                wave.fill(Complex32::default());
                for h in 1..=level.harmonics {
                    let value = frequency[h] / FRAME_SIZE as f32;
                    wave[h] = value;
                    wave[level.length - h] = value.conj();
                }
                inverse.process_with_scratch(wave, &mut scratch);
                let start = level.offset + frame_index * level.stride;
                let target = &mut data[start..start + level.stride];
                for (i, sample) in target.iter_mut().enumerate() {
                    *sample = wave[(i + level.length - GUARD) % level.length].re;
                }
            }
        }
        polynomial::prepare(&mut data, &levels, asset);
        Ok(Self {
            content_hash: asset.content_hash.clone(),
            frames: asset.frame_count,
            levels,
            data: Storage::Owned(data.into_boxed_slice()),
            kernel: Kernel::shared(),
        })
    }
    pub fn bytes(&self) -> usize {
        self.data.len() * size_of::<f32>()
    }
    /// The exact harmonic weights, for independent additive reference renders.
    pub fn harmonic_weights(&self, selection: Selection) -> [(usize, f64); 2] {
        [
            (
                self.levels[selection.low].harmonics,
                1.0 - selection.blend as f64,
            ),
            (
                self.levels[(selection.low + 1).min(self.levels.len() - 1)].harmonics,
                selection.blend as f64,
            ),
        ]
    }

    /// The richer mip has zero weight by its safe boundary, never afterwards.
    #[inline]
    pub fn selection(&self, hz: f64, sample_rate: f64) -> Selection {
        let hz = hz.abs();
        for (i, level) in self.levels.iter().enumerate() {
            let safe = sample_rate * 0.45 / level.harmonics as f64;
            if hz <= safe {
                let blend = if i + 1 < self.levels.len() {
                    ((hz / safe - 0.8) * 5.0).clamp(0.0, 1.0) as f32
                } else {
                    0.0
                };
                return Selection { low: i, blend };
            }
        }
        Selection {
            low: self.levels.len() - 1,
            blend: 0.0,
        }
    }
    #[inline]
    pub fn read(&self, phase: f64, position: f32, selection: Selection) -> f32 {
        if !phase.is_finite() || !position.is_finite() {
            return 0.0;
        }
        let phase = if (0.0..1.0).contains(&phase) {
            phase
        } else {
            phase.rem_euclid(1.0)
        };
        let frame = position.clamp(0.0, 1.0) * (self.frames - 1) as f32;
        let first = (frame as usize).min(self.frames - 1);
        let next = (first + 1).min(self.frames - 1);
        let fraction = frame - first as f32;
        let lower = self.read_level(&self.levels[selection.low], phase, first, next, fraction);
        if selection.blend == 0.0 {
            return lower;
        }
        let higher = self.read_level(
            &self.levels[(selection.low + 1).min(self.levels.len() - 1)],
            phase,
            first,
            next,
            fraction,
        );
        lower + (higher - lower) * selection.blend
    }
    /// Resolve frame and mip slices once per output frame, before the
    /// oversampled FM/sync loop. The ordinary Clean reader stays unchanged.
    fn reader(&self, position: f32, selection: Selection) -> lookup::Reader<'_> {
        lookup::Reader::new(self, position, selection)
    }
    #[inline]
    fn read_level(&self, level: &Level, phase: f64, first: usize, next: usize, mix: f32) -> f32 {
        let at = phase * level.length as f64;
        let index = at as usize;
        let (kernel, fraction) = self.kernel.position(at - index as f64);
        let a = level.offset + first * level.stride + index + 1;
        let b = level.offset + next * level.stride + index + 1;
        let from = &self.data[a..a + TAPS];
        let into = &self.data[b..b + TAPS];
        let weights = &self.kernel.phases[kernel];
        let next_weights = &self.kernel.phases[kernel + 1];
        let mut value = 0.0;
        for i in 0..TAPS {
            let wave = from[i] + (into[i] - from[i]) * mix;
            let weight = weights[i] + (next_weights[i] - weights[i]) * fraction;
            value += wave * weight;
        }
        value
    }
}

#[derive(Clone, Copy, Default)]
struct Ramp {
    value: f32,
    target: f32,
    step: f32,
    left: u32,
}
impl Ramp {
    fn new(value: f32) -> Self {
        Self {
            value,
            target: value,
            ..Self::default()
        }
    }
    fn target(&mut self, value: f32, samples: u32) {
        if value.to_bits() != self.target.to_bits() {
            self.target = value;
            self.left = samples.max(1);
            self.step = (value - self.value) / self.left as f32;
        }
    }
    fn next(&mut self) -> f32 {
        if self.left > 0 {
            self.left -= 1;
            self.value = if self.left == 0 {
                self.target
            } else {
                self.value + self.step
            };
        }
        self.value
    }
}

use crate::{
    Stereo,
    wavetable::{Controls, MAX_UNISON, OSCILLATORS, Settings},
};
const SMOOTH_FIELDS: usize = 14;
fn smooth_values(c: Controls) -> [f32; SMOOTH_FIELDS] {
    [
        c.position,
        c.transpose,
        c.fine,
        if c.enabled { c.level } else { 0.0 },
        c.pan,
        c.detune,
        c.spread,
        c.position_env_depth,
        c.surf_depth,
        c.key_waves,
        c.position_lfo_depth,
        c.fm_index,
        c.sync_ratio,
        c.voice_shift,
    ]
}

struct Layer {
    target: Controls,
    ramps: [Ramp; SMOOTH_FIELDS],
    lanes: [Ramp; MAX_UNISON],
    offsets: [Ramp; MAX_UNISON],
    position_offsets: [Ramp; MAX_UNISON],
    tune: f64,
    ratios: [f64; MAX_UNISON],
    gains: [Stereo; MAX_UNISON],
    pitch_key: [u32; 4],
    operator: Ramp,
}
impl Layer {
    fn new(controls: Controls) -> Self {
        Self {
            target: controls,
            ramps: smooth_values(controls).map(Ramp::new),
            lanes: std::array::from_fn(|i| Ramp::new(f32::from(i < controls.unison as usize))),
            offsets: std::array::from_fn(|i| {
                Ramp::new(if controls.unison > 1 && i < controls.unison as usize {
                    2.0 * i as f32 / (controls.unison - 1) as f32 - 1.0
                } else {
                    0.0
                })
            }),
            position_offsets: std::array::from_fn(|i| {
                Ramp::new(if i < controls.unison as usize {
                    i as f32 - (controls.unison - 1) as f32 * 0.5
                } else {
                    0.0
                })
            }),
            tune: 1.0,
            ratios: [1.0; MAX_UNISON],
            gains: [[0.0; 2]; MAX_UNISON],
            pitch_key: [u32::MAX; 4],
            operator: Ramp::new(f32::from(controls.enabled)),
        }
    }
    fn configure(&mut self, controls: Controls, rate: f32) {
        for (ramp, target) in self.ramps.iter_mut().zip(smooth_values(controls)) {
            ramp.target(target, (rate * 0.002).round() as u32);
        }
        for (i, ramp) in self.lanes.iter_mut().enumerate() {
            ramp.target(
                f32::from(i < controls.unison as usize),
                (rate * 0.005).round() as u32,
            );
            if i < controls.unison as usize {
                let offset = if controls.unison > 1 {
                    2.0 * i as f32 / (controls.unison - 1) as f32 - 1.0
                } else {
                    0.0
                };
                self.offsets[i].target(offset, (rate * 0.005).round() as u32);
                self.position_offsets[i].target(
                    i as f32 - (controls.unison - 1) as f32 * 0.5,
                    (rate * 0.005).round() as u32,
                );
            }
        }
        self.operator
            .target(f32::from(controls.enabled), (rate * 0.005) as u32);
        self.target = controls;
    }
    fn advance(&mut self, transpose: f32, power: f32) {
        self.operator.next();
        for ramp in &mut self.ramps {
            ramp.next();
        }
        for ramp in &mut self.position_offsets {
            ramp.next();
        }
        let coarse = self.ramps[1].value;
        let fine = self.ramps[2].value;
        let detune = self.ramps[5].value;
        let key = [
            coarse.to_bits(),
            fine.to_bits(),
            detune.to_bits(),
            transpose.to_bits(),
        ];
        // Common tune also moves through a shared ramp; compute once per layer,
        // never an exponential for every sample of every oscillator lane.
        let changed = key != self.pitch_key;
        if changed {
            self.tune =
                2.0_f64.powf((transpose as f64 + coarse as f64 + fine as f64 * 0.01) / 12.0);
        }
        for (i, ratio) in self.ratios.iter_mut().enumerate() {
            let old = self.offsets[i].value;
            let spread = self.offsets[i].next();
            if changed || spread != old {
                *ratio = self.tune * 2.0_f64.powf(spread as f64 * detune as f64 / 1200.0);
            }
        }
        self.pitch_key = key;
        let mut sum = 0.0;
        for lane in &mut self.lanes {
            sum += lane.next();
        }
        let norm = if sum > 0.0 {
            self.ramps[3].value * 0.5 * power / sum
        } else {
            0.0
        };
        for (i, gain) in self.gains.iter_mut().enumerate() {
            let offset = self.offsets[i].value;
            let pan = (self.ramps[4].value + offset * self.ramps[6].value * 0.5).clamp(0.0, 1.0);
            let level = self.lanes[i].value * norm;
            *gain = [
                (2.0 * (1.0 - pan)).min(1.0) * level,
                (2.0 * pan).min(1.0) * level,
            ];
        }
    }
}

#[derive(Clone, Copy)]
struct Lane {
    phase: f64,
    frequency: f64,
    selection: Selection,
    high_gain: f32,
    master: f64,
    correction: f32,
    mode: u8,
    phase_mod: f64,
}
impl Default for Lane {
    fn default() -> Self {
        Self {
            phase: 0.0,
            frequency: f64::NAN,
            selection: Selection::default(),
            high_gain: 1.0,
            master: 0.0,
            correction: 0.0,
            mode: 0,
            phase_mod: 0.0,
        }
    }
}
#[derive(Clone, Copy)]
struct Voice {
    lanes: [[Lane; MAX_UNISON]; OSCILLATORS],
    envelopes: [Ramp; OSCILLATORS],
    generation: u32,
    note: u32,
    hz: f64,
    frames: [[Ramp; MAX_UNISON]; OSCILLATORS],
    last_frame: u64,
    pairs: [interaction::Pair; 2],
    graph: interaction::Graph,
}
impl Default for Voice {
    fn default() -> Self {
        Self {
            lanes: [[Lane::default(); MAX_UNISON]; OSCILLATORS],
            envelopes: [Ramp::default(); OSCILLATORS],
            generation: u32::MAX,
            note: u32::MAX,
            hz: 0.0,
            frames: [[Ramp::default(); MAX_UNISON]; OSCILLATORS],
            last_frame: 0,
            pairs: [interaction::Pair::default(); 2],
            graph: interaction::Graph::default(),
        }
    }
}

/// Compact per-note Position feedback; it never feeds back into the preset.
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub struct PositionView {
    pub current: f32,
    pub min: f32,
    pub max: f32,
}
impl Default for PositionView {
    fn default() -> Self {
        Self {
            current: 0.0,
            min: 0.0,
            max: 0.0,
        }
    }
}

pub struct Runtime {
    rate: f64,
    count: usize,
    voices: Box<[Voice]>,
    banks: [Option<Arc<Bank>>; OSCILLATORS],
    previous: [Option<Arc<Bank>>; OSCILLATORS],
    fade_left: [u32; OSCILLATORS],
    fade_blend: [f32; OSCILLATORS],
    layers: [Layer; OSCILLATORS],
    transpose: Ramp,
    power: Ramp,
    seed: u64,
    view: [PositionView; OSCILLATORS],
    viewed: bool,
    motion: [Ramp; 4],
    clock: u64,
    weights: [f32; 4],
    interaction_kernel: &'static interaction::Kernel,
    dc: f32,
}
impl Runtime {
    pub fn new(rate: u32, settings: &Settings, assets: &Pool) -> Result<Self, String> {
        settings.validate_version()?;
        let mut banks = std::array::from_fn(|_| None);
        for (i, bank) in banks.iter_mut().enumerate().take(settings.count as usize) {
            *bank = Some(Bank::resolve(&settings.oscillators[i].asset_id, assets)?);
        }
        Ok(Self {
            rate: rate.clamp(8_000, 384_000) as f64,
            count: settings.count as usize,
            voices: (0..super::VOICES)
                .map(|_| Voice::default())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            banks,
            previous: std::array::from_fn(|_| None),
            fade_left: [0; OSCILLATORS],
            fade_blend: [1.0; OSCILLATORS],
            layers: settings.controls().map(|mut c| {
                c.sanitize();
                Layer::new(c)
            }),
            transpose: Ramp::new(0.0),
            power: Ramp::new(1.0),
            seed: settings.seed,
            view: [PositionView::default(); OSCILLATORS],
            viewed: false,
            motion: std::array::from_fn(|i| Ramp::new(settings.motion.get(i))),
            clock: 0,
            weights: [1.0; 4],
            interaction_kernel: interaction::Kernel::shared(),
            dc: (-std::f64::consts::TAU * 5.0 / rate as f64).exp() as f32,
        })
    }
    /// Exchanges only small states/handles. The caller retires `old` off audio.
    pub fn inherit(&mut self, old: &mut Self) {
        std::mem::swap(&mut self.voices, &mut old.voices);
        if self.rate != old.rate {
            for voice in self.voices.iter_mut() {
                for lanes in &mut voice.lanes {
                    for lane in lanes {
                        lane.frequency = f64::NAN;
                    }
                }
            }
        }
        let motion = self.motion.map(|r| r.target);
        self.motion = old.motion;
        for (r, target) in self.motion.iter_mut().zip(motion) {
            r.target(target, (self.rate * 0.005) as u32);
        }
        self.clock = old.clock;
        let desired = self.layers.each_ref().map(|l| l.target);
        if self.rate == old.rate {
            std::mem::swap(&mut self.layers, &mut old.layers);
            self.transpose = old.transpose;
            self.power = old.power;
        }
        for (i, &controls) in desired.iter().enumerate() {
            // A removed layer retains its old bank while its gain slews out.
            // The next control-side replacement reclaims this bounded handle.
            if self.banks[i].is_none() {
                self.banks[i] = old.banks[i].take();
            }
            let same = match (&self.banks[i], &old.banks[i]) {
                (Some(a), Some(b)) => a.content_hash == b.content_hash,
                _ => false,
            };
            if same {
                self.previous[i] = old.previous[i].take();
                self.fade_left[i] = old.fade_left[i];
                self.fade_blend[i] = old.fade_blend[i];
            } else {
                self.previous[i] = old.banks[i].take();
                self.fade_left[i] = (self.rate * 0.020).round() as u32;
                self.fade_blend[i] = 0.0;
            }
            self.layers[i].configure(controls, self.rate as f32);
        }
    }
    pub fn configure(&mut self, controls: [Controls; OSCILLATORS], transpose: f32) {
        let transpose = if transpose.is_finite() {
            transpose.clamp(-128.0, 128.0)
        } else {
            0.0
        };
        self.transpose
            .target(transpose, (self.rate * 0.002).round() as u32);
        for (i, (layer, mut controls)) in self.layers.iter_mut().zip(controls).enumerate() {
            controls.enabled &= i < self.count;
            let position = controls.position;
            controls.resolve_sources(i);
            controls.sanitize();
            // Shared Position offsets remain unclamped until the local ENV is
            // included, so opposite contributions can cancel at the edges.
            if position.is_finite() {
                controls.position = position;
            }
            layer.configure(controls, self.rate as f32);
        }
    }
    pub fn configure_motion(&mut self, mut motion: crate::wavetable::motion::Motion) {
        motion.sanitize();
        for (i, ramp) in self.motion.iter_mut().enumerate() {
            ramp.target(motion.get(i), (self.rate * 0.005) as u32);
        }
    }
    pub fn begin_frame(&mut self, powered: bool) {
        self.clock = self.clock.wrapping_add(1);
        for ramp in &mut self.motion {
            ramp.next();
        }
        let weights = crate::wavetable::motion::weights(
            self.count,
            self.motion[2].value,
            self.motion[3].value,
        );
        self.weights = weights.map(|w| 1.0 + (w - 1.0) * self.motion[1].value);
        self.power
            .target(f32::from(powered), (self.rate * 0.005).round() as u32);
        let power = self.power.next();
        let transpose = self.transpose.next();
        for layer in &mut self.layers {
            layer.advance(transpose, power);
        }
        for i in 0..OSCILLATORS {
            self.fade_blend[i] =
                1.0 - self.fade_left[i] as f32 / (self.rate * 0.020).round().max(1.0) as f32;
            self.fade_left[i] = self.fade_left[i].saturating_sub(1);
            let position = self.layers[i].ramps[0].value;
            self.view[i] = PositionView {
                current: position,
                min: position,
                max: position,
            };
        }
        self.viewed = false;
    }
    pub fn active(&self) -> bool {
        (self.power.value > 0.0 || self.power.target > 0.0)
            && self
                .layers
                .iter()
                .any(|l| l.ramps[3].value > 0.0 || l.ramps[3].target > 0.0)
    }
    pub fn positions(&self) -> [PositionView; OSCILLATORS] {
        if self.viewed {
            self.view
        } else {
            // No sounding note: feedback follows the selected/shared Position,
            // including before the worklet's first live audio frame.
            std::array::from_fn(|i| {
                let controls = self.layers[i].target;
                let position = controls.position + self.motion[0].target * controls.surf_depth;
                let at = |value: f32| {
                    let value = value.clamp(0.0, 1.0);
                    if controls.step
                        && let Some(bank) = &self.banks[i]
                    {
                        let count = (bank.frames - 1).max(1) as f32;
                        (value * count).round() / count
                    } else {
                        value
                    }
                };
                let radius = (controls.unison - 1) as f32 * 0.5 * controls.voice_shift.abs();
                PositionView {
                    current: at(position),
                    min: at(position - radius),
                    max: at(position + radius),
                }
            })
        }
    }
    /// Move finished banks into a preallocated retirement message. Dropping
    /// their last Arc belongs to the receiving control/worker owner.
    pub fn retire_into(&mut self, retired: &mut [Option<Arc<Bank>>; OSCILLATORS]) {
        for (i, slot) in retired.iter_mut().enumerate() {
            if self.fade_left[i] == 0 && self.fade_blend[i] == 1.0 && slot.is_none() {
                *slot = self.previous[i].take();
            }
        }
    }
    pub fn render(
        &mut self,
        index: usize,
        generation: u32,
        note: f32,
        envs: &[f32; crate::preset::MODULATORS],
    ) -> Stereo {
        self.render_modulated(
            index,
            generation,
            note,
            envs,
            &[0.0; crate::preset::MODULATORS],
        )
    }
    pub fn render_modulated(
        &mut self,
        index: usize,
        generation: u32,
        note: f32,
        envs: &[f32; crate::preset::MODULATORS],
        lfos: &[f32; crate::preset::MODULATORS],
    ) -> Stereo {
        if !note.is_finite() || note.abs() > 512.0 || index >= self.voices.len() {
            return [0.0; 2];
        }
        let voice = &mut self.voices[index];
        if voice.generation != generation {
            voice.generation = generation;
            voice.note = u32::MAX;
            for pair in &mut voice.pairs {
                pair.reset();
            }
            voice.graph.reset();
            for (osc, lanes) in voice.lanes.iter_mut().enumerate() {
                let c = self.layers[osc].target;
                for (lane, state) in lanes.iter_mut().enumerate() {
                    let key = self.seed
                        ^ (generation as u64) << 32
                        ^ (index as u64) << 16
                        ^ (osc as u64) << 8
                        ^ lane as u64;
                    let phase = if c.free_phase {
                        let frequency = if state.frequency.is_finite() {
                            state.frequency
                        } else {
                            440.0
                                * 2.0_f64.powf((note as f64 - 69.0) / 12.0)
                                * self.layers[osc].ratios[lane]
                        };
                        (state.phase
                            + frequency * (self.clock - voice.last_frame) as f64 / self.rate)
                            .rem_euclid(1.0)
                    } else {
                        (c.phase as f64 + c.phase_random as f64 * random01(key)).fract()
                    };
                    *state = Lane {
                        phase,
                        master: phase,
                        ..Lane::default()
                    };
                }
                let env = c
                    .position_env
                    .checked_sub(1)
                    .and_then(|i| envs.get(i as usize))
                    .copied()
                    .unwrap_or(0.0);
                voice.envelopes[osc] = Ramp::new(env);
                voice.frames[osc] = [Ramp::new(c.position); MAX_UNISON];
            }
        }
        if voice.note != note.to_bits() {
            voice.note = note.to_bits();
            voice.hz = 440.0 * 2.0_f64.powf((note as f64 - 69.0) / 12.0);
        }
        let mut positions = [[0.0; MAX_UNISON]; OSCILLATORS];
        for (osc, result) in positions.iter_mut().enumerate() {
            let layer = &self.layers[osc];
            let Some(bank) = &self.banks[osc] else {
                continue;
            };
            let env = layer
                .target
                .position_env
                .checked_sub(1)
                .and_then(|i| envs.get(i as usize))
                .copied()
                .unwrap_or(0.0);
            voice.envelopes[osc].target(env, super::CONTROL_INTERVAL as u32);
            let position = layer.ramps[0].value
                + self.motion[0].value * layer.ramps[8].value
                + (note - 60.0) / 60.0 * layer.ramps[9].value
                + layer.ramps[10].value
                    * layer
                        .target
                        .position_lfo
                        .checked_sub(1)
                        .and_then(|i| lfos.get(i as usize))
                        .copied()
                        .unwrap_or(0.0)
                + layer.ramps[7].value * voice.envelopes[osc].next();
            let frames = (bank.frames - 1).max(1) as f32;
            let center = position.clamp(0.0, 1.0);
            self.view[osc].current = if layer.target.step {
                (center * frames).round() / frames
            } else {
                center
            };
            let mut first = !self.viewed;
            for (id, result) in result.iter_mut().enumerate() {
                let mut shifted = (position
                    + layer.position_offsets[id].value * layer.ramps[13].value)
                    .clamp(0.0, 1.0);
                if layer.target.step {
                    voice.frames[osc][id].target(
                        (shifted * frames).round() / frames,
                        (self.rate * 0.002) as u32,
                    );
                    shifted = voice.frames[osc][id].next();
                } else {
                    voice.frames[osc][id] = Ramp::new(shifted);
                }
                *result = shifted;
                if layer.lanes[id].value > 0.0 {
                    if first {
                        self.view[osc].min = shifted;
                        self.view[osc].max = shifted;
                        first = false;
                    } else {
                        self.view[osc].min = self.view[osc].min.min(shifted);
                        self.view[osc].max = self.view[osc].max.max(shifted);
                    }
                }
            }
        }
        let legacy = |lanes: &mut [[Lane; MAX_UNISON]; OSCILLATORS],
                      pairs: &mut [interaction::Pair; 2]| {
            let mut output = [0.0; 2];
            let mut paired = false;
            for osc in 0..OSCILLATORS {
                if osc % 2 == 1 && paired {
                    continue;
                }
                if osc % 2 == 0 {
                    let frame = interaction::render(
                        osc / 2,
                        &mut pairs[osc / 2],
                        lanes,
                        &self.layers,
                        &self.banks,
                        &self.previous,
                        &self.fade_blend,
                        &positions,
                        &self.weights,
                        voice.hz,
                        self.rate,
                        self.interaction_kernel,
                        self.dc,
                    );
                    paired = frame.is_some();
                    if let Some(frame) = frame {
                        for c in 0..2 {
                            output[c] += frame[c];
                        }
                        continue;
                    }
                }
                let layer = &self.layers[osc];
                let Some(bank) = &self.banks[osc] else {
                    continue;
                };
                for (id, lane) in lanes[osc].iter_mut().enumerate() {
                    let position = positions[osc][id];
                    let gains = layer.gains[id];
                    let frequency = voice.hz * layer.ratios[id];
                    if frequency != lane.frequency {
                        lane.frequency = frequency;
                        lane.selection = bank.selection(frequency, self.rate);
                        lane.high_gain = ((self.rate * 0.45 - frequency) / (self.rate * 0.03))
                            .clamp(0.0, 1.0) as f32;
                    }
                    if gains != [0.0; 2] && lane.high_gain > 0.0 {
                        let mut sample = bank.read(lane.phase, position, lane.selection);
                        if self.fade_blend[osc] < 1.0
                            && let Some(previous) = &self.previous[osc]
                        {
                            let old = previous.read(lane.phase, position, lane.selection);
                            sample = old + (sample - old) * self.fade_blend[osc];
                        }
                        sample *= lane.high_gain;
                        for c in 0..2 {
                            output[c] += sample * gains[c] * self.weights[osc];
                        }
                    }
                    lane.phase += frequency / self.rate;
                    if lane.phase >= 1.0 {
                        lane.phase -= lane.phase.floor();
                    }
                    lane.master = lane.phase;
                    lane.correction = 0.0;
                }
            }
            output
        };
        let fm = std::array::from_fn::<_, OSCILLATORS, _>(|i| {
            let l = &self.layers[i];
            let s = l.target.fm_source as usize;
            self.banks[i].is_some()
                && l.target.fm_mode != 0
                && l.ramps[11].value > 0.0
                && self.banks.get(s).is_some_and(Option::is_some)
                && self.layers[s].operator.value > 0.0
        });
        let sync = std::array::from_fn::<_, OSCILLATORS, _>(|i| {
            let l = &self.layers[i];
            self.banks[i].is_some()
                && l.target.sync
                && self
                    .banks
                    .get(l.target.sync_source as usize)
                    .is_some_and(Option::is_some)
        });
        let custom = (0..OSCILLATORS).any(|i| {
            (fm[i] && (i % 2 == 1 || self.layers[i].target.fm_source as usize != (i ^ 1)))
                || (sync[i] && self.layers[i].target.sync_source as usize != i)
        });
        let output = if custom || voice.graph.active {
            let mut old_lanes = if voice.graph.mix.value < 1.0 || !custom {
                Some(voice.lanes)
            } else {
                None
            };
            let old = old_lanes
                .as_mut()
                .map_or([0.0; 2], |lanes| legacy(lanes, &mut voice.pairs));
            let (new, mix) = interaction::render_routed(
                &mut voice.graph,
                custom,
                &mut voice.lanes,
                &self.layers,
                &self.banks,
                &self.previous,
                &self.fade_blend,
                &positions,
                &self.weights,
                voice.hz,
                self.rate,
                self.interaction_kernel,
                self.dc,
            );
            // The graph retires at zero mix without advancing its clocks.
            // Keep the dry trajectory on that final transition sample too.
            if mix == 0.0
                && let Some(lanes) = old_lanes
            {
                voice.lanes = lanes;
            }
            std::array::from_fn(|c| old[c] + (new[c] - old[c]) * mix)
        } else {
            legacy(&mut voice.lanes, &mut voice.pairs)
        };
        self.viewed = true;
        voice.last_frame = self.clock;
        output
    }
}

fn random01(mut value: u64) -> f64 {
    value = value.wrapping_add(0x9e37_79b9_7f4a_7c15);
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^= value >> 31;
    (value >> 11) as f64 / (1u64 << 53) as f64
}

/// A short, bounded raw-table cue. It owns a prepared bank and never changes
/// the playing timbre's patch, notes, position or effects.
pub struct Audition {
    runtime: Box<Runtime>,
    at: u32,
    end: u32,
    rate: u32,
    controls: [Controls; OSCILLATORS],
}
impl Audition {
    pub fn new(id: &str, assets: &Pool, rate: u32) -> Result<Self, String> {
        let mut settings = Settings::default();
        for oscillator in &mut settings.oscillators {
            oscillator.asset_id = id.into();
        }
        Ok(Self {
            runtime: Box::new(Runtime::new(rate, &settings, assets)?),
            at: 0,
            end: (rate as f32 * 0.75) as u32,
            rate,
            controls: settings.controls(),
        })
    }
    pub fn stop(&mut self) {
        self.end = self.end.min(self.at + self.rate * 15 / 1000);
    }
    pub fn process_frame(&mut self) -> Stereo {
        if self.at >= self.end {
            return [0.0; 2];
        }
        if self.at.is_multiple_of(super::CONTROL_INTERVAL as u32) {
            self.controls[0].position = (self.at as f32 / (self.rate as f32 * 0.7)).min(1.0);
            self.runtime.configure(self.controls, 0.0);
        }
        self.runtime.begin_frame(true);
        let gain = (self.at as f32 / (self.rate as f32 * 0.008)).min(1.0)
            * ((self.end - self.at) as f32 / (self.rate as f32 * 0.020)).min(1.0)
            * 0.35;
        self.at += 1;
        self.runtime
            .render(1, 1, 60.0, &[0.0; crate::preset::MODULATORS])
            .map(|v| v * gain)
    }
}

#[derive(Default)]
pub struct Prepared(pub Option<Box<Runtime>>, pub Option<String>);
impl Clone for Prepared {
    fn clone(&self) -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for Prepared {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PreparedWavetable")
            .field("ready", &self.0.is_some())
            .finish()
    }
}
impl Prepared {
    pub fn prepare(
        &mut self,
        settings: Option<&Settings>,
        assets: &Pool,
        rate: u32,
    ) -> Result<(), String> {
        match settings
            .map(|s| Runtime::new(rate, s, assets).map(Box::new))
            .transpose()
        {
            Ok(runtime) => {
                self.0 = runtime;
                self.1 = None;
                Ok(())
            }
            Err(error) => {
                self.0 = None;
                self.1 = Some(error.clone());
                Err(error)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn sine_frames() -> Asset {
        let samples: Vec<_> = (0..FRAME_SIZE * 2)
            .map(|i| {
                ((i % FRAME_SIZE) as f64 * std::f64::consts::TAU / FRAME_SIZE as f64).sin() as f32
                    * if i < FRAME_SIZE { 0.8 } else { 0.2 }
            })
            .collect();
        Asset::from_cycles("Sine frames".into(), FRAME_SIZE, &samples).unwrap()
    }
    #[test]
    fn frames_and_mips_preserve_amplitude_phase_and_endpoints() {
        let bank = Bank::compile(&sine_frames()).unwrap();
        for low in 0..HARMONICS.len() {
            for position in [0.0, 0.23, 0.5, 1.0] {
                for i in 0..513 {
                    let phase = (i as f64 + 0.37) / 513.0;
                    let expected =
                        (phase * std::f64::consts::TAU).sin() as f32 * (0.8 - 0.6 * position);
                    let got = bank.read(phase, position, Selection { low, blend: 0.0 });
                    // This kernel has finite passband ripple. -80 dBFS bounds
                    // it independently of the looser full-spectrum Q02 gate.
                    assert!(
                        (got - expected).abs() < 1e-4,
                        "level {low}: {got} != {expected}"
                    );
                }
            }
        }
        assert!(bank.bytes() < 3 * 1024 * 1024);
        assert_eq!(bank.read(f64::NAN, 0.0, Selection::default()), 0.0);
    }
    #[test]
    fn level_changes_are_continuous_and_always_bandlimited() {
        let bank = Bank::compile(&sine_frames()).unwrap();
        for i in 0..HARMONICS.len() - 1 {
            let boundary = 48_000.0 * 0.45 / HARMONICS[i] as f64;
            let below = bank.selection(boundary * (1.0 - 1e-8), 48_000.0);
            let above = bank.selection(boundary * (1.0 + 1e-8), 48_000.0);
            assert_eq!(below.low, i);
            assert_eq!(above.low, i + 1);
            assert!(below.blend > 0.9999);
            for selection in [below, above] {
                assert!(
                    bank.levels[selection.low].harmonics as f64 * boundary <= 48_000.0 * 0.45001
                );
            }
            let a = bank.read(0.123, 0.4, below);
            let b = bank.read(0.123, 0.4, above);
            assert!((a - b).abs() < 2e-6);
        }
    }
    #[test]
    fn returning_to_paired_routes_retires_the_graph_after_its_crossfade() {
        let mut settings = Settings::default();
        settings.count = 4;
        settings.oscillators[0].controls.fm_mode = 1;
        settings.oscillators[0].controls.fm_index = 0.7;
        settings.oscillators[0].controls.fm_source = 3;
        settings.oscillators[3].controls.enabled = true;
        settings.oscillators[1].controls.enabled = true;
        let mut runtime = Runtime::new(48000, &settings, &Pool::default()).unwrap();
        for _ in 0..2000 {
            runtime.begin_frame(true);
            runtime.render(1, 1, 60.0, &[0.0; 8]);
        }
        assert!(runtime.voices.iter().any(|v| v.graph.active));
        settings.oscillators[0].controls.fm_source = 1;
        runtime.configure(settings.controls(), 0.0);
        for _ in 0..2000 {
            runtime.begin_frame(true);
            runtime.render(1, 1, 60.0, &[0.0; 8]);
        }
        assert!(
            runtime
                .voices
                .iter()
                .all(|v| !v.graph.active && v.graph.mix.value == 0.0)
        );
    }

    #[test]
    fn banks_are_shared_without_a_per_voice_copy() {
        let pool = Pool::default();
        let a = Bank::resolve(crate::wavetable::DEFAULT_ASSET, &pool).unwrap();
        let b = Bank::resolve(crate::wavetable::DEFAULT_ASSET, &pool).unwrap();
        assert!(Arc::ptr_eq(&a, &b));
    }
}
