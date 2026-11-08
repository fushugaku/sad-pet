//! Eight sparse physical slots, rendered once into independent stems.
//! Shares wavetable lookup, controls, unison and anti-alias filters with legacy.
use super::*;
use crate::dsp::{
    SampleBuffer,
    interaction::{Input, PhaseSlew},
    radias_oscillator, radias_wavefolder,
    resynth::PcmLoop,
};
use crate::{
    interaction::SyncMode,
    oscillators::{self as model, Kind, SLOTS},
    resynth::{Model, Waveform},
};
use radias_synth_application::primary::PrimaryProgram;
use radias_synth_domain::{
    Sample,
    pitch::{PhaseIncrement, PitchCode},
    primary_oscillator::PrimaryOscillator,
    waveform::WaveformTable,
};
use std::ops::Range;

#[derive(Clone, Copy, Default)]
pub struct SlotFrame {
    pub audio: Stereo,
    pub generation: Stereo,
    pub output: Stereo,
    pub wrap: Option<f64>,
    pub latency: f64,
}
#[derive(Default)]
struct Native {
    controller: radias_oscillator::Controller,
    oscillator: PrimaryOscillator,
    folder: radias_wavefolder::Wavefolder,
    pcm: f64,
    slew: PhaseSlew,
    initialized: bool,
    correction: Stereo,
    transport_phase: f64,
}
struct MixedVoice {
    identities: [u32; SLOTS],
    generation: u32,
    note: u32,
    hz: f64,
    last_frame: u64,
    lanes: [[Lane; MAX_UNISON]; SLOTS],
    native: [Native; SLOTS],
    envelopes: [Ramp; SLOTS],
    frames: [[Ramp; MAX_UNISON]; SLOTS],
    slews: [[PhaseSlew; MAX_UNISON]; SLOTS],
    mix_reduction: interaction::Filter,
    bypass_reduction: interaction::Filter,
    bypass_quiet: u16,
    exported: u8,
    generation_reduction: [interaction::Filter; SLOTS],
    taps: [f32; SLOTS],
    edges: [f64; SLOTS],
    lowpass: [[f32; 3]; SLOTS],
    quiet: [u16; SLOTS],
    last: [Stereo; SLOTS],
    tails: [Stereo; SLOTS],
    transition_left: [u32; SLOTS],
    selected: [f32; SLOTS],
    selection_ready: bool,
}
impl Default for MixedVoice {
    fn default() -> Self {
        Self {
            identities: [0; SLOTS],
            generation: u32::MAX,
            note: u32::MAX,
            hz: 0.0,
            last_frame: 0,
            lanes: [[Lane::default(); MAX_UNISON]; SLOTS],
            native: std::array::from_fn(|_| Native::default()),
            envelopes: [Ramp::default(); SLOTS],
            frames: [[Ramp::default(); MAX_UNISON]; SLOTS],
            slews: [[PhaseSlew::default(); MAX_UNISON]; SLOTS],
            mix_reduction: interaction::Filter::default(),
            bypass_reduction: interaction::Filter::default(),
            bypass_quiet: u16::MAX,
            exported: 0,
            generation_reduction: [interaction::Filter::default(); SLOTS],
            taps: [0.0; SLOTS],
            edges: [-1.0; SLOTS],
            lowpass: [[0.0; 3]; SLOTS],
            quiet: [u16::MAX; SLOTS],
            last: [[0.0; 2]; SLOTS],
            tails: [[0.0; 2]; SLOTS],
            transition_left: [0; SLOTS],
            selected: [1.0; SLOTS],
            selection_ready: false,
        }
    }
}

pub struct Runtime {
    rate: f64,
    identities: [u32; SLOTS],
    kinds: [Kind; SLOTS],
    native_controls: [crate::resynth::Oscillator; SLOTS],
    programs: [PrimaryProgram; SLOTS],
    limits: [f64; SLOTS],
    alpha: [f32; SLOTS],
    dc: f32,
    root: u32,
    root_hz: f64,
    layers: [Layer; SLOTS],
    dependencies: [u8; SLOTS],
    reachability: [u8; 1 << SLOTS],
    voices: Box<[MixedVoice]>,
    banks: [Option<Arc<Bank>>; SLOTS],
    previous: [Option<Arc<Bank>>; SLOTS],
    blends: [f32; SLOTS],
    fade_left: [u32; SLOTS],
    tables: radias_oscillator::Tables,
    waveform: WaveformTable,
    kernel: &'static interaction::Kernel,
    motion: [Ramp; 4],
    xy: [Option<usize>; 4],
    weights: [f32; SLOTS],
    weight_ramps: [Ramp; SLOTS],
    transpose: f32,
    clock: u64,
    power: Ramp,
    input: Stereo,
    view: [PositionView; SLOTS],
}
impl Runtime {
    pub fn new(rate: u32, settings: &model::Settings, assets: &Pool) -> Result<Self, String> {
        Self::prepare(rate, settings, assets, None)
    }
    pub(crate) fn prepare(
        rate: u32,
        settings: &model::Settings,
        assets: &Pool,
        previous: Option<&Self>,
    ) -> Result<Self, String> {
        settings.validate()?;
        let rate = rate.clamp(8000, 384000) as f64;
        let mut banks = std::array::from_fn(|_| None);
        for (i, s) in settings.slots.iter().enumerate() {
            if s.id != 0 && s.kind == Kind::Wavetable {
                banks[i] = Some(Bank::resolve(&s.asset_id, assets)?);
            }
        }
        let controls = settings.controls();
        let native_controls = std::array::from_fn(|i| controls[i].native(settings.slots[i].kind));
        let mut runtime = Self {
            rate,
            identities: settings.slots.each_ref().map(|s| s.id),
            kinds: settings.slots.each_ref().map(|s| s.kind),
            native_controls,
            programs: native_controls.map(primary_program),
            limits: [rate; SLOTS],
            alpha: [0.0; SLOTS],
            dc: (-std::f64::consts::TAU * 5.0 / rate).exp() as f32,
            root: u32::MAX,
            root_hz: 440.0,
            layers: controls.map(|c| Layer::new(c.table())),
            dependencies: [0; SLOTS],
            reachability: std::array::from_fn(|mask| mask as u8),
            // inherit() exchanges the running voices before this bank can render.
            // Do not allocate a second complete bank just to discard it there.
            voices: if previous.is_some_and(|old| old.rate == rate) {
                Box::default()
            } else {
                (0..crate::dsp::VOICES)
                    .map(|_| MixedVoice::default())
                    .collect::<Vec<_>>()
                    .into_boxed_slice()
            },
            banks,
            previous: std::array::from_fn(|_| None),
            blends: [1.0; SLOTS],
            fade_left: [0; SLOTS],
            tables: radias_oscillator::Tables::new(),
            waveform: radias_oscillator::waveform_table(),
            kernel: interaction::Kernel::shared(),
            motion: std::array::from_fn(|i| Ramp::new(settings.motion.get(i))),
            xy: settings.xy_slots.map(|id| settings.index(id)),
            weights: [1.0; SLOTS],
            weight_ramps: [Ramp::new(1.0); SLOTS],
            transpose: 0.0,
            clock: 0,
            power: Ramp::new(1.0),
            input: [0.0; 2],
            view: controls.map(|c| PositionView {
                current: c.wavetable.position,
                min: c.wavetable.position,
                max: c.wavetable.position,
            }),
        };
        runtime.refresh_dependencies();
        Ok(runtime)
    }
    pub fn inherit(&mut self, old: &mut Self) {
        if self.rate != old.rate {
            return;
        }
        std::mem::swap(&mut self.voices, &mut old.voices);
        self.clock = old.clock;
        self.power = old.power;
        self.weight_ramps = old.weight_ramps;
        for i in 0..4 {
            let target = self.motion[i].value;
            self.motion[i] = old.motion[i];
            self.motion[i].target(target, (self.rate * 0.005) as u32);
        }
        for i in 0..SLOTS {
            if self.identities[i] != 0
                && self.identities[i] == old.identities[i]
                && self.kinds[i] == old.kinds[i]
            {
                let desired = self.layers[i].target;
                std::mem::swap(&mut self.layers[i], &mut old.layers[i]);
                self.layers[i].configure(desired, self.rate as f32);
                let same = match (&self.banks[i], &old.banks[i]) {
                    (Some(a), Some(b)) => a.content_hash == b.content_hash,
                    (None, None) => true,
                    _ => false,
                };
                if same {
                    self.previous[i] = old.previous[i].take();
                    self.blends[i] = old.blends[i];
                    self.fade_left[i] = old.fade_left[i];
                } else {
                    self.previous[i] = old.banks[i].take();
                    self.blends[i] = 0.0;
                    self.fade_left[i] = (self.rate * 0.020) as u32;
                }
            } else {
                for voice in self.voices.iter_mut() {
                    if old.identities[i] != 0 {
                        voice.tails[i] = voice.last[i];
                        voice.transition_left[i] = (self.rate * 0.005) as u32;
                    }
                    voice.identities[i] = 0;
                }
            }
        }
        self.refresh_dependencies();
    }
    pub fn configure(&mut self, controls: [model::Controls; SLOTS], transpose: f32) {
        self.transpose = transpose;
        for (i, c) in controls.into_iter().enumerate() {
            self.native_controls[i] = c.native(self.kinds[i]);
            self.programs[i] = primary_program(self.native_controls[i]);
            self.layers[i].configure(c.table(), self.rate as f32);
        }
        self.refresh_dependencies();
    }
    fn refresh_dependencies(&mut self) {
        let direct = std::array::from_fn::<_, SLOTS, _>(|i| {
            let c = self.layers[i].target;
            let source = if self.kinds[i] == Kind::Wavetable && c.fm_mode != 0 && c.fm_index > 0.0 {
                Some(c.fm_source as usize)
            } else if self.native_controls[i].mode == crate::resynth::Mode::Cross
                && self.native_controls[i].cross_source > 0
            {
                Some(self.native_controls[i].cross_source as usize - 1)
            } else {
                None
            };
            let mut mask = 0;
            if let Some(source) = source
                && source < SLOTS
                && self.identities[source] != 0
                && self.layers[source].target.enabled
            {
                mask |= 1 << source;
            }
            let source = c.sync_source.min(7) as usize;
            if c.sync && self.identities[source] != 0 && self.layers[source].target.enabled {
                mask |= 1 << source;
            }
            mask
        });
        if direct == self.dependencies {
            return;
        }
        self.dependencies = direct;
        let mut reachable = std::array::from_fn::<_, SLOTS, _>(|i| direct[i] | (1 << i));
        for via in 0..SLOTS {
            for i in 0..SLOTS {
                if reachable[i] & (1 << via) != 0 {
                    reachable[i] |= reachable[via];
                }
            }
        }
        // Any dynamic demand/level/operator seed now needs just one lookup.
        // Recheck effective targets on every configure, including modulation.
        for mask in 1usize..self.reachability.len() {
            let source = mask.trailing_zeros() as usize;
            self.reachability[mask] = self.reachability[mask & (mask - 1)] | reachable[source];
        }
    }
    pub fn configure_motion(&mut self, motion: crate::wavetable::motion::Motion) {
        for (i, ramp) in self.motion.iter_mut().enumerate() {
            ramp.target(motion.get(i), (self.rate * 0.005) as u32);
        }
    }
    pub fn begin_frame(&mut self, power: bool) {
        self.clock = self.clock.wrapping_add(1);
        self.power
            .target(f32::from(power), (self.rate * 0.005) as u32);
        let power = self.power.next();
        for ramp in &mut self.motion {
            ramp.next();
        }
        self.weights.fill(1.0);
        let count = self.xy.iter().filter(|x| x.is_some()).count();
        let w =
            crate::wavetable::motion::weights(count, self.motion[2].value, self.motion[3].value);
        let mut sums = [0.0; SLOTS];
        let mut assigned = 0u8;
        for (corner, slot) in self.xy.into_iter().flatten().enumerate() {
            sums[slot] += w[corner];
            assigned |= 1 << slot;
        }
        for (i, sum) in sums.into_iter().enumerate() {
            if assigned & (1 << i) != 0 {
                self.weights[i] = 1.0 + (sum - 1.0) * self.motion[1].value;
            }
        }
        for i in 0..SLOTS {
            self.weight_ramps[i].target(self.weights[i], (self.rate * 0.005) as u32);
            self.weights[i] = self.weight_ramps[i].next();
            self.layers[i].advance(self.transpose, power);
            self.blends[i] = 1.0 - self.fade_left[i] as f32 / (self.rate * 0.020).max(1.0) as f32;
            self.fade_left[i] = self.fade_left[i].saturating_sub(1);
            self.view[i] = PositionView {
                current: self.layers[i].ramps[0].value,
                min: 1.0,
                max: 0.0,
            };
        }
        let mut limits = [self.rate; SLOTS];
        for layer in &self.layers {
            if layer.target.fm_mode != 0 && layer.target.fm_source < 8 {
                let source = layer.target.fm_source as usize;
                limits[source] =
                    limits[source].min(self.rate / (1.0 + layer.ramps[11].value as f64 * 0.5));
            }
        }
        for (i, limit) in limits.into_iter().enumerate() {
            if limit != self.limits[i] {
                self.alpha[i] = if limit < self.rate {
                    (1.0 - (-std::f64::consts::TAU * limit * 0.45 * 3.32 / (self.rate * 8.0)).exp())
                        as f32
                } else {
                    0.0
                };
            }
        }
        self.limits = limits;
    }
    pub fn positions(&self) -> [PositionView; SLOTS] {
        self.view
    }
    pub fn set_input(&mut self, input: Stereo) {
        self.input = input;
    }
    pub fn retire_into(&mut self, retired: &mut [Option<Arc<Bank>>; SLOTS]) {
        for (i, dest) in retired.iter_mut().enumerate() {
            if self.fade_left[i] == 0 && self.blends[i] == 1.0 && dest.is_none() {
                *dest = self.previous[i].take();
            }
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn render(
        &mut self,
        index: usize,
        generation: u32,
        note: f32,
        envs: &[f32; crate::preset::MODULATORS],
        lfos: &[f32; crate::preset::MODULATORS],
        inputs: [Input; SLOTS],
        demand: u8,
        export: u8,
        capture: [f32; SLOTS],
        sample: &SampleBuffer,
        range: &Range<usize>,
        root: f32,
        transports: Option<&mut crate::dsp::phase_transport::Runtime>,
        stamp: u64,
    ) -> ([SlotFrame; SLOTS], Stereo, Stereo) {
        self.render_masked(
            index,
            generation,
            note,
            envs,
            lfos,
            inputs,
            demand,
            export,
            capture,
            sample,
            range,
            root,
            transports,
            stamp,
            u8::MAX,
        )
    }

    /// Select the slots heard for this voice without removing their signals
    /// from the interaction graph or their audio from other voices.
    #[allow(clippy::too_many_arguments)]
    pub fn render_masked(
        &mut self,
        index: usize,
        generation: u32,
        note: f32,
        envs: &[f32; crate::preset::MODULATORS],
        lfos: &[f32; crate::preset::MODULATORS],
        inputs: [Input; SLOTS],
        demand: u8,
        export: u8,
        capture: [f32; SLOTS],
        sample: &SampleBuffer,
        range: &Range<usize>,
        root: f32,
        transports: Option<&mut crate::dsp::phase_transport::Runtime>,
        stamp: u64,
        mask: u8,
    ) -> ([SlotFrame; SLOTS], Stereo, Stereo) {
        // Native-only patches do not need wavetable lookup views. A separate
        // specialization removes that large per-frame scratch array while
        // retaining the same internal clock, filters and interaction graph.
        if self
            .identities
            .iter()
            .zip(self.kinds)
            .any(|(&id, kind)| id != 0 && kind == Kind::Wavetable)
        {
            self.render_masked_inner::<true>(
                index, generation, note, envs, lfos, inputs, demand, export, capture, sample,
                range, root, transports, stamp, mask,
            )
        } else {
            self.render_masked_inner::<false>(
                index, generation, note, envs, lfos, inputs, demand, export, capture, sample,
                range, root, transports, stamp, mask,
            )
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_masked_inner<const TABLES: bool>(
        &mut self,
        index: usize,
        generation: u32,
        note: f32,
        envs: &[f32; crate::preset::MODULATORS],
        lfos: &[f32; crate::preset::MODULATORS],
        inputs: [Input; SLOTS],
        demand: u8,
        export: u8,
        capture: [f32; SLOTS],
        sample: &SampleBuffer,
        range: &Range<usize>,
        root: f32,
        mut transports: Option<&mut crate::dsp::phase_transport::Runtime>,
        stamp: u64,
        mask: u8,
    ) -> ([SlotFrame; SLOTS], Stereo, Stereo) {
        let mut result = [SlotFrame::default(); SLOTS];
        if index >= self.voices.len() || !note.is_finite() {
            return (result, [0.0; 2], [0.0; 2]);
        }
        let voice = &mut self.voices[index];
        if voice.generation != generation {
            voice.selection_ready = false;
            voice.taps.fill(0.0);
            voice.edges.fill(-1.0);
            voice.transition_left.fill(0);
            voice.tails.fill([0.0; 2]);
            voice.mix_reduction = interaction::Filter::default();
            voice.bypass_reduction = interaction::Filter::default();
            voice.bypass_quiet = u16::MAX;
        }
        let selection_step = 1.0 / (self.rate as f32 * 0.005).max(1.0);
        for (i, selected) in voice.selected.iter_mut().enumerate() {
            let target = f32::from(mask & (1 << i) != 0);
            if voice.selection_ready {
                *selected += (target - *selected).clamp(-selection_step, selection_step);
            } else {
                *selected = target;
            }
        }
        voice.selection_ready = true;
        if voice.note != note.to_bits() {
            voice.note = note.to_bits();
            voice.hz = 440.0 * 2.0f64.powf((note as f64 - 69.0) / 12.0);
        }
        for i in 0..SLOTS {
            if voice.generation != generation || voice.identities[i] != self.identities[i] {
                let c = self.layers[i].target;
                for (j, lane) in voice.lanes[i].iter_mut().enumerate() {
                    let phase = if c.free_phase && voice.identities[i] == self.identities[i] {
                        (lane.phase
                            + if lane.frequency.is_finite() {
                                lane.frequency
                            } else {
                                voice.hz
                            } * (self.clock - voice.last_frame) as f64
                                / self.rate)
                            .rem_euclid(1.0)
                    } else {
                        (c.phase as f64
                            + c.phase_random as f64
                                * random01(
                                    0x53504d4958
                                        ^ (generation as u64) << 32
                                        ^ (index as u64) << 16
                                        ^ (self.identities[i] as u64) << 8
                                        ^ j as u64,
                                ))
                        .fract()
                    };
                    *lane = Lane {
                        phase,
                        master: phase,
                        ..Default::default()
                    };
                }
                voice.native[i] = Native::default();
                voice.slews[i] = [PhaseSlew::default(); MAX_UNISON];
                voice.generation_reduction[i] = interaction::Filter::default();
                voice.identities[i] = self.identities[i];
                voice.edges[i] = -1.0;
                voice.lowpass[i] = [0.0; 3];
                voice.taps[i] = 0.0;
                voice.frames[i] = [Ramp::new(c.position); MAX_UNISON];
                voice.envelopes[i] = Ramp::new(
                    c.position_env
                        .checked_sub(1)
                        .and_then(|e| envs.get(e as usize))
                        .copied()
                        .unwrap_or(0.0),
                );
            }
        }
        voice.generation = generation;
        voice.last_frame = self.clock;
        let hard = inputs.iter().any(|i| i.playback.mode == SyncMode::Hard);
        let factor = 8; // One fixed profile: automation never changes rate or tap latency.
        let internal = self.rate * factor as f64;
        let limits = self.limits;
        let alpha = self.alpha;
        let mut positions = [[0.0; MAX_UNISON]; SLOTS];
        let mut norms = [1.0; SLOTS];
        let mut views: [[Option<interaction::Lookup<'_>>; MAX_UNISON]; SLOTS] =
            [[None; MAX_UNISON]; SLOTS];
        if TABLES {
            for i in 0..SLOTS {
                let layer = &self.layers[i];
                let Some(bank) = &self.banks[i] else {
                    continue;
                };
                norms[i] = layer
                    .lanes
                    .iter()
                    .map(|lane| lane.value)
                    .sum::<f32>()
                    .max(1.0);
                let env = layer
                    .target
                    .position_env
                    .checked_sub(1)
                    .and_then(|e| envs.get(e as usize))
                    .copied()
                    .unwrap_or(0.0);
                voice.envelopes[i].target(env, crate::dsp::CONTROL_INTERVAL as u32);
                let position = layer.ramps[0].value
                    + self.motion[0].value * layer.ramps[8].value
                    + (note - 60.0) / 60.0 * layer.ramps[9].value
                    + layer.ramps[7].value * voice.envelopes[i].next()
                    + layer.ramps[10].value
                        * layer
                            .target
                            .position_lfo
                            .checked_sub(1)
                            .and_then(|e| lfos.get(e as usize))
                            .copied()
                            .unwrap_or(0.0);
                for j in 0..MAX_UNISON {
                    let mut p = (position
                        + layer.position_offsets[j].value * layer.ramps[13].value)
                        .clamp(0.0, 1.0);
                    if layer.target.step {
                        let n = (bank.frames - 1).max(1) as f32;
                        voice.frames[i][j].target((p * n).round() / n, (self.rate * 0.002) as u32);
                        p = voice.frames[i][j].next();
                    }
                    positions[i][j] = p;
                    self.view[i].min = self.view[i].min.min(p);
                    self.view[i].max = self.view[i].max.max(p);
                    if layer.lanes[j].value == 0.0 {
                        continue;
                    }
                    let frequency = voice.hz * layer.ratios[j];
                    let source = layer.target.fm_source.min(7) as usize;
                    let fm = layer.target.fm_mode != 0 && self.identities[source] != 0;
                    let depth = layer.ramps[11].value as f64;
                    let band = limits[source] * 0.45;
                    let base = frequency
                        * if layer.target.sync {
                            layer.ramps[12].value as f64
                        } else {
                            1.0
                        };
                    let bound = base
                        + if fm && layer.target.fm_mode == 1 {
                            2.0 * depth * voice.hz * self.layers[source].tune + band
                        } else if fm {
                            (2.0 * depth + 1.0) * band
                        } else {
                            0.0
                        };
                    let selection =
                        bank.selection(bound, if hard { self.rate * 4.0 } else { internal });
                    views[i][j] = Some(interaction::Lookup::new(
                        bank,
                        self.previous[i].as_deref(),
                        self.blends[i],
                        p,
                        selection,
                    ));
                }
            }
        }
        if self.root != root.to_bits() {
            self.root = root.to_bits();
            self.root_hz = 440.0 * 2.0f64.powf((root as f64 - 69.0) / 12.0);
        }
        let pcm_loops: [Option<PcmLoop>; SLOTS] = std::array::from_fn(|i| {
            (self.native_controls[i].waveform == Waveform::Pcm)
                .then(|| PcmLoop::new(sample, range, self.native_controls[i]))
        });
        let pcm_rates = std::array::from_fn::<_, SLOTS, _>(|i| {
            voice.hz * self.layers[i].tune / self.root_hz * sample.sample_rate as f64 / internal
        });
        let pcm_mips = std::array::from_fn::<_, SLOTS, _>(|i| {
            if pcm_loops[i].is_some() {
                sample.mip_for_rate(pcm_rates[i], true)
            } else {
                crate::dsp::sample::Mip::flat(0)
            }
        });
        let codes = std::array::from_fn::<_, SLOTS, _>(|i| {
            let o = self.native_controls[i];
            PitchCode::new(
                ((note + self.transpose + o.transpose + o.fine * 0.01).clamp(0.0, 127.996) * 256.0)
                    .round()
                    .min(32767.0) as u16,
            )
            .unwrap()
        });
        let mut needed = demand;
        for i in 0..SLOTS {
            if self.layers[i].ramps[3].value > 0.0 && self.layers[i].operator.value > 0.0 {
                needed |= 1 << i;
            }
        }
        let needed = self.reachability[needed as usize];
        for i in 0..SLOTS {
            if export & (1 << i) != 0 && voice.exported & (1 << i) == 0 {
                voice.generation_reduction[i] = interaction::Filter::default();
            }
        }
        voice.exported = export;
        let has_bypass = (0..SLOTS)
            .any(|i| capture[i] < 1.0 && (self.identities[i] != 0 || voice.transition_left[i] > 0));
        voice.bypass_quiet = if has_bypass {
            0
        } else {
            voice.bypass_quiet.saturating_add(1)
        };
        for i in 0..SLOTS {
            if needed & (1 << i) != 0 {
                voice.quiet[i] = 0;
            } else {
                voice.quiet[i] = voice.quiet[i].saturating_add(1);
            }
        }
        for sub in 0..factor {
            let mut into = [0.0; 2];
            let mut bypass = [0.0; 2];
            let old_taps = voice.taps;
            let old_edges = voice.edges;
            voice.edges.fill(-1.0);
            for i in 0..SLOTS {
                let layer = &self.layers[i];
                if self.identities[i] == 0 || layer.operator.value == 0.0 || needed & (1 << i) == 0
                {
                    voice.taps[i] = 0.0;
                    if export & (1 << i) != 0 {
                        reduce(
                            &mut voice.generation_reduction[i],
                            [0.0; 2],
                            factor,
                            sub,
                            self.kernel,
                        );
                    }
                    if voice.transition_left[i] > 0 {
                        let t = 1.0
                            - voice.transition_left[i] as f32 / (self.rate as f32 * 0.005).max(1.0);
                        let fade = 1.0 - t * t * (3.0 - 2.0 * t);
                        for c in 0..2 {
                            let audio = voice.tails[i][c] * fade * voice.selected[i];
                            into[c] += audio * capture[i];
                            bypass[c] += audio * (1.0 - capture[i]);
                        }
                    }
                    continue;
                }
                let input = inputs[i];
                let mut raw = [0.0; 2];
                let mut heard = [0.0; 2];
                if TABLES && self.kinds[i] == Kind::Wavetable {
                    let norm = norms[i];
                    let reference = voice.lanes[i][0].phase;
                    for (j, view) in views[i].iter_mut().enumerate() {
                        if layer.lanes[j].value == 0.0 {
                            continue;
                        }
                        let lane = &mut voice.lanes[i][j];
                        let frequency = voice.hz * layer.ratios[j];
                        let high = ((self.rate * 0.45 - frequency) / (self.rate * 0.03))
                            .clamp(0.0, 1.0) as f32;
                        let source = layer.target.fm_source as usize;
                        let modulation = old_taps.get(source).copied().unwrap_or(0.0) as f64
                            * layer.ramps[11].value as f64;
                        let pm = if layer.target.fm_mode == 2 {
                            modulation / std::f64::consts::TAU
                        } else {
                            0.0
                        };
                        let mode_key = layer.target.fm_mode + 4 * source.min(7) as u8;
                        if lane.mode != mode_key {
                            lane.phase = (lane.phase + lane.phase_mod - pm).rem_euclid(1.0);
                            lane.mode = mode_key;
                        }
                        lane.phase_mod = pm;
                        let fm = if layer.target.fm_mode == 1 {
                            modulation * voice.hz * self.layers.get(source).map_or(1.0, |l| l.tune)
                        } else {
                            0.0
                        };
                        let mut command = input.playback;
                        command.phase = (command.phase + lane.phase - reference).rem_euclid(1.0);
                        if command.event_in(sub, factor).is_some() {
                            voice.slews[i][j].correct(lane.phase, command, internal);
                        }
                        let source = layer.target.sync_source.min(7) as usize;
                        let linked = layer.target.sync
                            && input.playback.mode == SyncMode::Off
                            && self.identities[source] != 0
                            && self.layers[source].target.enabled;
                        let master_step = frequency / internal;
                        let step = (frequency
                            * if linked {
                                layer.ramps[12].value as f64
                            } else {
                                1.0
                            }
                            + fm)
                            / internal
                            + voice.slews[i][j].next();
                        let event = if input.playback.mode == SyncMode::Hard {
                            input.playback.event_in(sub, factor).unwrap_or(-1.0)
                        } else if linked {
                            command.phase = (lane.phase - reference).rem_euclid(1.0);
                            if source == i {
                                interaction::routing::crossing(lane.master, master_step)
                            } else {
                                old_edges[source]
                            }
                        } else {
                            -1.0
                        };
                        let value = interaction::routing::value(
                            view.as_mut(),
                            &mut lane.correction,
                            lane.phase,
                            step,
                            event,
                            command.phase,
                            pm,
                        );
                        let edge = interaction::routing::crossing(lane.phase + pm, step);
                        if j == 0 && edge >= 0.0 && (event < 0.0 || edge <= event) {
                            result[i].wrap = Some((sub as f64 + edge) / factor as f64);
                        }
                        if event >= 0.0 && j == 0 {
                            let after = interaction::routing::crossing(
                                command.phase + pm,
                                step * (1.0 - event),
                            );
                            if after >= 0.0 {
                                result[i].wrap = Some(
                                    (sub as f64 + event + (1.0 - event) * after) / factor as f64,
                                );
                            }
                        }
                        lane.phase = if event >= 0.0 {
                            (command.phase + step * (1.0 - event)).rem_euclid(1.0)
                        } else {
                            (lane.phase + step).rem_euclid(1.0)
                        };
                        lane.master = (lane.master + master_step).rem_euclid(1.0);
                        lane.frequency = frequency;
                        for c in 0..2 {
                            raw[c] += value * high * layer.lanes[j].value / norm;
                            heard[c] += value * high * layer.gains[j][c] * self.weights[i];
                        }
                    }
                } else {
                    let native = &mut voice.native[i];
                    let o = self.native_controls[i];
                    let frequency = voice.hz * layer.tune;
                    let step = frequency / internal;
                    let transporting = o.model == Model::Radias
                        && matches!(
                            o.waveform,
                            Waveform::Noise | Waveform::AudioIn | Waveform::Formant
                        )
                        && (o.transport.enabled || input.playback.mode != SyncMode::Off)
                        && transports
                            .as_ref()
                            .is_some_and(|p| p.contains(i, crate::dsp::interaction::GLOBAL));
                    let phase = if transporting {
                        native.transport_phase
                    } else if let Some(pcm) = pcm_loops[i] {
                        native.pcm / pcm.length.max(1.0)
                    } else if o.model == Model::Wavefolder {
                        native.folder.phase
                    } else {
                        radias_oscillator::phase(&native.oscillator)
                    };
                    let event = input.playback.event_in(sub, factor);
                    if event.is_some() {
                        native.slew.correct(phase, input.playback, internal);
                    }
                    let correction = native.slew.next();
                    let initial = !native.initialized;
                    let sync =
                        (event.is_some() && input.playback.mode == SyncMode::Hard) || initial;
                    let reset_phase = if initial {
                        voice.lanes[i][0].phase
                    } else {
                        input.playback.phase
                    };
                    let code = codes[i];
                    let increment = PhaseIncrement(
                        ((step + if transporting { 0.0 } else { correction }).max(0.0)
                            * 4294967296.0)
                            .round()
                            .min(u32::MAX as f64) as u32,
                    );
                    if o.waveform == Waveform::AudioIn {
                        raw = self.input;
                    } else if o.waveform == Waveform::Pcm {
                        let pcm = pcm_loops[i].unwrap();
                        if initial {
                            native.pcm = reset_phase * pcm.length;
                        }
                        let rate = pcm_rates[i] + correction * pcm.length;
                        let reset = (!initial && sync)
                            .then(|| (event.unwrap_or(0.0), reset_phase * pcm.length));
                        let old = native.pcm;
                        let (value, end) = crate::dsp::interaction::read_phase(
                            &mut native.correction,
                            old,
                            rate,
                            reset,
                            |p| {
                                let mut p = p.rem_euclid(pcm.length.max(1.0));
                                pcm.read_at(&mut p, 0.0, pcm_mips[i], sample, range)
                            },
                        );
                        raw = value;
                        native.pcm = end.rem_euclid(pcm.length.max(1.0));
                        if reset.is_none() && old + rate >= pcm.length {
                            result[i].wrap =
                                Some((sub as f64 + (pcm.length - old) / rate) / factor as f64);
                        }
                    } else if o.model == Model::Wavefolder {
                        if initial {
                            native.folder.phase = reset_phase;
                        }
                        let controls = [
                            o.control1 as f64,
                            ((o.control2 as f64 * 127.0 - 64.0) / 64.0).clamp(-1.0, 1.0),
                        ];
                        let reset = (!initial && sync).then(|| (event.unwrap_or(0.0), reset_phase));
                        let value = if reset.is_some() || native.correction != [0.0; 2] {
                            native.folder.next_interacting(
                                step + correction,
                                o.waveform,
                                o.fold_shape,
                                controls,
                                reset,
                                &mut native.correction,
                            )
                        } else {
                            native.folder.next(
                                step + correction,
                                o.waveform,
                                o.fold_shape,
                                controls,
                                false,
                            )
                        };
                        if !sync && phase + step + correction >= 1.0 {
                            result[i].wrap = Some(
                                (sub as f64 + (1.0 - phase) / (step + correction)) / factor as f64,
                            );
                        }
                        raw = [value; 2];
                    } else {
                        let program = self.programs[i];
                        let frame = if self.clock & 3 == 3 && sub + 1 == factor {
                            self.clock
                        } else {
                            self.clock & !3
                        };
                        let x = native
                            .controller
                            .next_with_fractional_reset(
                                &mut native.oscillator,
                                program,
                                increment,
                                code,
                                (lfos[0] * 32767.0) as i16,
                                [0; 2],
                                Sample(
                                    (old_taps
                                        .get(o.cross_source.saturating_sub(1) as usize)
                                        .copied()
                                        .unwrap_or(0.0) as f64
                                        * i32::MAX as f64)
                                        .clamp(i32::MIN as f64, i32::MAX as f64)
                                        as i32,
                                ),
                                &self.tables,
                                &self.waveform,
                                frame,
                                index * SLOTS + i,
                                (sync && !transporting) || initial,
                                self.clock / 32,
                                radias_oscillator::reset_word(reset_phase),
                                if !initial && !transporting && sync {
                                    event
                                } else {
                                    None
                                },
                            )
                            .0 as f32
                            / 1073741824.0;
                        raw = [x * 0.5; 2];
                        if !sync && phase + step + correction >= 1.0 {
                            result[i].wrap = Some(
                                (sub as f64 + (1.0 - phase) / (step + correction)) / factor as f64,
                            );
                        }
                    }
                    if let Some(pool) = &mut transports {
                        if sub + 1 == factor {
                            pool.capture(i, index, raw, stamp);
                        }
                        if transporting {
                            if initial {
                                native.transport_phase = reset_phase;
                            }
                            let start = native.transport_phase;
                            let advance = step * o.transport.rate as f64 + correction;
                            let reset =
                                (!initial && sync).then(|| (event.unwrap_or(0.0), reset_phase));
                            let (value, end) = crate::dsp::interaction::read_phase(
                                &mut native.correction,
                                start,
                                advance,
                                reset,
                                |p| {
                                    pool.read(i, index, p, frequency, internal, o.waveform)
                                        .unwrap_or([0.0; 2])
                                },
                            );
                            raw = value;
                            native.transport_phase = end.rem_euclid(1.0);
                            result[i].wrap = if reset.is_none() && start + advance >= 1.0 {
                                Some((sub as f64 + (1.0 - start) / advance) / factor as f64)
                            } else {
                                None
                            };
                        }
                    }
                    native.initialized = true;
                    voice.lanes[i][0].phase = if transporting {
                        native.transport_phase
                    } else if let Some(pcm) = pcm_loops[i] {
                        native.pcm / pcm.length.max(1.0)
                    } else if o.model == Model::Wavefolder {
                        native.folder.phase
                    } else {
                        radias_oscillator::phase(&native.oscillator)
                    };
                    voice.lanes[i][0].frequency = frequency;
                    if frequency >= self.rate * 0.45 {
                        raw = [0.0; 2];
                    }
                    let pan = layer.ramps[4].value;
                    let level = layer.ramps[3].value * 0.5 * self.weights[i] * self.power.value;
                    heard = [
                        raw[0] * (2.0 * (1.0 - pan)).min(1.0) * level,
                        raw[1] * (2.0 * pan).min(1.0) * level,
                    ];
                }
                let mono = (raw[0] + raw[1]) * 0.5;
                let mut tap = mono.clamp(-2.0, 2.0);
                if limits[i] < self.rate {
                    for pole in &mut voice.lowpass[i] {
                        *pole += alpha[i] * (tap - *pole);
                        tap = *pole;
                    }
                } else {
                    voice.lowpass[i].fill(tap);
                }
                voice.taps[i] = if layer.target.enabled { tap } else { 0.0 };
                if let Some(at) = result[i].wrap {
                    let at = at * factor as f64 - sub as f64;
                    if (0.0..=1.0).contains(&at) {
                        voice.edges[i] = at;
                    }
                }
                let factor_ring = input.factor_at(sub, factor);
                // Pack two mono modulation stems in one shared stereo FIR.
                if export & (1 << i) != 0 {
                    reduce(
                        &mut voice.generation_reduction[i],
                        [mono, mono * factor_ring],
                        factor,
                        sub,
                        self.kernel,
                    );
                }
                let mut audio = heard.map(|x| x * factor_ring);
                if voice.transition_left[i] > 0 {
                    let t =
                        1.0 - voice.transition_left[i] as f32 / (self.rate as f32 * 0.005).max(1.0);
                    let mix = t * t * (3.0 - 2.0 * t);
                    for (sample, tail) in audio.iter_mut().zip(voice.tails[i]) {
                        *sample = tail * (1.0 - mix) + *sample * mix;
                    }
                }
                voice.last[i] = audio;
                for c in 0..2 {
                    into[c] += audio[c] * capture[i] * voice.selected[i];
                    bypass[c] += audio[c] * (1.0 - capture[i]) * voice.selected[i];
                }
            }
            reduce(&mut voice.mix_reduction, into, factor, sub, self.kernel);
            if voice.bypass_quiet < 128 {
                reduce(
                    &mut voice.bypass_reduction,
                    bypass,
                    factor,
                    sub,
                    self.kernel,
                );
            }
        }
        let dc = self.dc;
        let into = voice.mix_reduction.next_mixed(self.kernel, dc);
        let bypass = if voice.bypass_quiet < 128 {
            voice.bypass_reduction.next_mixed(self.kernel, dc)
        } else {
            voice.bypass_reduction.silence(dc)
        };
        for (i, slot) in result.iter_mut().enumerate() {
            slot.latency = 39.125;
            if export & (1 << i) != 0 {
                let modulation = voice.generation_reduction[i].next_mixed(self.kernel, dc);
                if self.identities[i] != 0
                    && self.layers[i].target.enabled
                    && self.power.target > 0.0
                {
                    slot.generation = [modulation[0]; 2];
                    slot.output = [modulation[1]; 2];
                } else {
                    slot.wrap = None;
                }
            }
            voice.transition_left[i] = voice.transition_left[i].saturating_sub(1);
        }
        (result, into, bypass)
    }
}

fn primary_program(o: crate::resynth::Oscillator) -> PrimaryProgram {
    PrimaryProgram {
        selection: if o.waveform.analog() {
            o.waveform.code() + o.mode.code() * 16
        } else {
            o.waveform.code()
        },
        control: radias_synth_domain::controller_primary::PrimaryControl {
            control1: (o.control1 * 127.0).round() as u8,
            control2: (o.control2 * 127.0).round() as u8,
            ..Default::default()
        },
    }
}

fn reduce(
    filter: &mut interaction::Filter,
    audio: Stereo,
    factor: usize,
    sub: usize,
    kernel: &interaction::Kernel,
) {
    if factor == 32 {
        let value = filter.pre(audio, kernel, sub % 4 == 3);
        if sub % 4 == 3 {
            filter.push(value);
        }
    } else {
        filter.push(audio);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_specialization_preserves_audio_taps_and_fractional_resets() {
        for kind in [Kind::Radias, Kind::Wavefolder] {
            for waveform in [
                Waveform::Saw,
                Waveform::Sine,
                Waveform::Triangle,
                Waveform::Noise,
                Waveform::Pcm,
            ] {
                let mut settings = model::Settings::default();
                settings.slots[0].kind = kind;
                settings.add(Kind::Radias).unwrap();
                settings.slots[0].controls.native.waveform = waveform;
                settings.slots[0].controls.native.control1 = 0.7;
                settings.slots[0].controls.native.control2 = 0.3;
                settings.slots[0].controls.transpose = -12.0;
                settings.slots[1].controls.transpose = 27.3;
                let mut native = Runtime::new(8000, &settings, &Pool::default()).unwrap();
                let mut full = Runtime::new(8000, &settings, &Pool::default()).unwrap();
                let sample = SampleBuffer::new(
                    "PCM".into(),
                    8000,
                    (0..128).map(|i| [(i as f32 * 0.1).sin(); 2]).collect(),
                )
                .unwrap();
                let envs = [0.3; crate::preset::MODULATORS];
                let mut audible = false;
                for stamp in 0..512 {
                    if stamp == 384 {
                        settings.slots[1].id = 0;
                        let mut next = Runtime::new(8000, &settings, &Pool::default()).unwrap();
                        next.inherit(&mut native);
                        native = next;
                        let mut next = Runtime::new(8000, &settings, &Pool::default()).unwrap();
                        next.inherit(&mut full);
                        full = next;
                    }
                    native.begin_frame(true);
                    full.begin_frame(true);
                    let mut inputs = [Input::default(); SLOTS];
                    inputs[1].ring = (stamp as f32 * 0.017).sin();
                    inputs[1].ring_start = inputs[1].ring * 0.9;
                    inputs[1].depth = 1.0;
                    if stamp % 11 == 0 {
                        inputs[1].playback = crate::dsp::interaction::PhaseCommand {
                            mode: SyncMode::Hard,
                            event: 0.371,
                            phase: 0.23,
                            strength: 1.0,
                            ..Default::default()
                        };
                    }
                    let generation = 1 + stamp / 256;
                    let note = 48.0 + (stamp / 64) as f32 * 0.7;
                    let (actual, mix, bypass) = native.render_masked(
                        0,
                        generation,
                        note,
                        &envs,
                        &envs,
                        inputs,
                        3,
                        3,
                        [0.5; SLOTS],
                        &sample,
                        &(0..128),
                        60.0,
                        None,
                        stamp as u64,
                        3,
                    );
                    let (expected, expected_mix, expected_bypass) = full
                        .render_masked_inner::<true>(
                            0,
                            generation,
                            note,
                            &envs,
                            &envs,
                            inputs,
                            3,
                            3,
                            [0.5; SLOTS],
                            &sample,
                            &(0..128),
                            60.0,
                            None,
                            stamp as u64,
                            3,
                        );
                    assert_eq!(mix.map(f32::to_bits), expected_mix.map(f32::to_bits));
                    audible |= mix.iter().any(|x| x.abs() > 1e-5);
                    assert_eq!(bypass.map(f32::to_bits), expected_bypass.map(f32::to_bits));
                    for (a, b) in actual.into_iter().zip(expected) {
                        assert_eq!(a.audio.map(f32::to_bits), b.audio.map(f32::to_bits));
                        assert_eq!(
                            a.generation.map(f32::to_bits),
                            b.generation.map(f32::to_bits)
                        );
                        assert_eq!(a.output.map(f32::to_bits), b.output.map(f32::to_bits));
                        assert_eq!(a.wrap.map(f64::to_bits), b.wrap.map(f64::to_bits));
                        assert_eq!(a.latency.to_bits(), b.latency.to_bits());
                    }
                }
                assert!(audible, "{kind:?} {waveform:?} did not render audio");
            }
        }
    }

    #[test]
    fn per_note_mask_selects_audio_but_not_fm_or_exported_taps() {
        let mut settings = model::Settings {
            xy_slots: [0; 4],
            ..Default::default()
        };
        settings.slots[0].kind = Kind::Wavetable;
        settings.add(Kind::Wavetable).unwrap();
        settings.slots[0].controls.wavetable.fm_mode = 1;
        settings.slots[0].controls.wavetable.fm_index = 0.8;
        settings.slots[0].controls.wavetable.fm_source = 1;
        settings.slots[1].controls.transpose = 7.0;
        let mut runtime = Runtime::new(8_000, &settings, &Pool::default()).unwrap();
        let sample = SampleBuffer::new("Unused".into(), 8_000, vec![[0.0; 2]; 128]).unwrap();
        let env = [0.0; crate::preset::MODULATORS];
        let mut energy = 0.0_f32;
        let mut tap_energy = 0.0_f32;
        for stamp in 0..1024 {
            runtime.begin_frame(true);
            let mut render = |index, mask| {
                runtime.render_masked(
                    index,
                    1,
                    60.0,
                    &env,
                    &env,
                    [Input::default(); SLOTS],
                    3,
                    3,
                    [0.5; SLOTS],
                    &sample,
                    &(0..128),
                    60.0,
                    None,
                    stamp,
                    mask,
                )
            };
            let (at, a, a_bypass) = render(0, 1);
            let (_, b, b_bypass) = render(1, 2);
            let (both_taps, both, both_bypass) = render(2, 3);
            let (_, silent, silent_bypass) = render(3, 0);
            for c in 0..2 {
                assert!((both[c] - (a[c] + b[c])).abs() < 2e-5);
                assert!((both_bypass[c] - (a_bypass[c] + b_bypass[c])).abs() < 2e-5);
                assert_eq!(silent[c], 0.0);
                assert_eq!(silent_bypass[c], 0.0);
                assert_eq!(at[1].generation[c], both_taps[1].generation[c]);
            }
            energy += a[0].abs();
            tap_energy += at[1].generation[0].abs();
        }
        assert!(energy > 1.0);
        assert!(
            tap_energy > 1.0,
            "an unselected modulation source lost its tap"
        );
        runtime.begin_frame(true);
        runtime.render_masked(
            0,
            1,
            60.0,
            &env,
            &env,
            [Input::default(); SLOTS],
            3,
            3,
            [0.5; SLOTS],
            &sample,
            &(0..128),
            60.0,
            None,
            1024,
            0,
        );
        assert!(runtime.voices[0].selected[0] > 0.97);
        assert_eq!(runtime.voices[1].selected[1], 1.0);
    }

    fn fixed_point_needed(runtime: &Runtime, mut needed: u8) -> u8 {
        for _ in 0..SLOTS {
            for i in 0..SLOTS {
                if needed & (1 << i) == 0 {
                    continue;
                }
                let c = runtime.layers[i].target;
                let source =
                    if runtime.kinds[i] == Kind::Wavetable && c.fm_mode != 0 && c.fm_index > 0.0 {
                        Some(c.fm_source as usize)
                    } else if runtime.native_controls[i].mode == crate::resynth::Mode::Cross
                        && runtime.native_controls[i].cross_source > 0
                    {
                        Some(runtime.native_controls[i].cross_source as usize - 1)
                    } else {
                        None
                    };
                if let Some(source) = source
                    && source < SLOTS
                    && runtime.identities[source] != 0
                    && runtime.layers[source].target.enabled
                {
                    needed |= 1 << source;
                }
                let source = c.sync_source.min(7) as usize;
                if c.sync
                    && runtime.identities[source] != 0
                    && runtime.layers[source].target.enabled
                {
                    needed |= 1 << source;
                }
            }
        }
        needed
    }

    #[test]
    fn dependency_transitions_preserve_fixed_point_reachability_and_audio() {
        let mut settings = model::Settings {
            xy_slots: [0; 4],
            ..Default::default()
        };
        settings.slots[0].kind = Kind::Wavetable;
        settings.add(Kind::Wavetable).unwrap();
        settings.add(Kind::Wavetable).unwrap();
        settings.add(Kind::Radias).unwrap();
        for i in 0..4 {
            let c = &mut settings.slots[i].controls;
            c.level = if i == 0 { 0.65 } else { 0.0 };
            c.transpose = i as f32 * 7.0;
            c.wavetable.unison = 3;
        }
        settings.slots[0].controls.wavetable.fm_mode = 1;
        settings.slots[0].controls.wavetable.fm_index = 0.8;
        settings.slots[0].controls.wavetable.fm_source = 1;
        settings.slots[1].controls.wavetable.fm_mode = 2;
        settings.slots[1].controls.wavetable.fm_index = 0.6;
        settings.slots[1].controls.wavetable.fm_source = 2;
        settings.slots[2].controls.wavetable.sync = true;
        settings.slots[2].controls.wavetable.sync_source = 3;
        settings.slots[3].controls.native.mode = crate::resynth::Mode::Cross;
        settings.slots[3].controls.native.cross_source = 1;
        let pool = Pool::default();
        let mut cached = Runtime::new(48000, &settings, &pool).unwrap();
        let mut reference = Runtime::new(48000, &settings, &pool).unwrap();
        assert_eq!(cached.reachability[1], 0b1111);
        let sample = SampleBuffer::new("Unused".into(), 48000, vec![[0.0; 2]; 128]).unwrap();
        let env = [0.0; crate::preset::MODULATORS];
        let mut audible = false;
        let mut silent_modulator_ran = false;
        for stamp in 0..2560u64 {
            let stage = stamp / 128;
            if stamp % 128 == 0 {
                match stage {
                    1 => settings.slots[0].controls.wavetable.fm_index = 0.0,
                    2 => {
                        settings.slots[0].controls.wavetable.fm_index = 0.8;
                        settings.slots[0].controls.wavetable.fm_mode = 0;
                    }
                    3 => settings.slots[0].controls.wavetable.fm_mode = 2,
                    4 => settings.slots[0].controls.wavetable.fm_source = 0,
                    5 => settings.slots[0].controls.wavetable.fm_source = 1,
                    6 => settings.slots[1].controls.enabled = false,
                    7 => settings.slots[1].controls.enabled = true,
                    8 => settings.slots[2].controls.wavetable.sync = false,
                    9 => {
                        settings.slots[2].controls.wavetable.sync = true;
                        settings.slots[2].controls.wavetable.sync_source = 2;
                    }
                    10 => settings.slots[2].controls.wavetable.sync_source = 3,
                    11 => settings.slots[3].controls.native.cross_source = 0,
                    12 => {
                        settings.slots[3].controls.native.cross_source = 1;
                        settings.slots[3].controls.native.mode = crate::resynth::Mode::Wave;
                    }
                    13 => settings.slots[0].controls.level = 0.0,
                    14 => {
                        settings.slots[0].controls.level = 0.65;
                        settings.slots[1].id = 0;
                    }
                    15 => settings.slots[1].id = 42,
                    16 => {
                        settings.slots[0].kind = Kind::Radias;
                        settings.slots[0].controls.native.mode = crate::resynth::Mode::Cross;
                        settings.slots[0].controls.native.cross_source = 4;
                    }
                    17 => settings.slots[0].kind = Kind::Wavetable,
                    _ => {}
                }
                if (14..=17).contains(&stage) {
                    let mut next = Runtime::new(48000, &settings, &pool).unwrap();
                    next.inherit(&mut cached);
                    cached = next;
                    let mut next = Runtime::new(48000, &settings, &pool).unwrap();
                    next.inherit(&mut reference);
                    reference = next;
                }
            }
            if stage == 18 {
                settings.slots[0].controls.wavetable.fm_index =
                    if stamp & 1 == 0 { 0.0 } else { 0.8 };
            } else if stage == 19 {
                settings.slots[0].controls.wavetable.fm_mode = (stamp % 3) as u8;
                settings.slots[0].controls.wavetable.fm_source = (stamp % 4) as u8;
            }
            // Effective controls can change at audio rate, without replacing a runtime.
            cached.configure(settings.controls(), 0.0);
            reference.configure(settings.controls(), 0.0);
            if stamp % 128 == 0 || stage >= 18 {
                let expected =
                    std::array::from_fn(|mask| fixed_point_needed(&reference, mask as u8));
                assert_eq!(cached.reachability, expected, "reachability at {stamp}");
                // Render the independent baseline dependency closure, not the cached one.
                reference.reachability = expected;
            }
            cached.begin_frame(true);
            reference.begin_frame(true);
            let demand = match stamp % 5 {
                0 => 0,
                n => 1 << (n - 1),
            };
            let render = |runtime: &mut Runtime| {
                runtime.render(
                    0,
                    1,
                    60.0,
                    &env,
                    &env,
                    [Input::default(); SLOTS],
                    demand,
                    u8::MAX,
                    [1.0; SLOTS],
                    &sample,
                    &(0..128),
                    60.0,
                    None,
                    stamp,
                )
            };
            let (actual, into, bypass) = render(&mut cached);
            let (expected, expected_into, expected_bypass) = render(&mut reference);
            assert_eq!(
                into.map(f32::to_bits),
                expected_into.map(f32::to_bits),
                "mix at {stamp}"
            );
            assert_eq!(bypass.map(f32::to_bits), expected_bypass.map(f32::to_bits));
            for (actual, expected) in actual.into_iter().zip(expected) {
                assert_eq!(
                    actual.audio.map(f32::to_bits),
                    expected.audio.map(f32::to_bits)
                );
                assert_eq!(
                    actual.generation.map(f32::to_bits),
                    expected.generation.map(f32::to_bits)
                );
                assert_eq!(
                    actual.output.map(f32::to_bits),
                    expected.output.map(f32::to_bits)
                );
                assert_eq!(
                    actual.wrap.map(f64::to_bits),
                    expected.wrap.map(f64::to_bits)
                );
                assert_eq!(actual.latency.to_bits(), expected.latency.to_bits());
            }
            assert_eq!(cached.voices[0].quiet, reference.voices[0].quiet);
            assert_eq!(
                cached.voices[0].taps.map(f32::to_bits),
                reference.voices[0].taps.map(f32::to_bits)
            );
            audible |= into.iter().any(|x| x.abs() > 1e-5);
            if stage == 0 {
                assert_eq!(cached.voices[0].quiet[1], 0);
                silent_modulator_ran |= cached.voices[0].taps[1].abs() > 1e-5;
            }
        }
        assert!(audible);
        assert!(silent_modulator_ran);
    }

    #[test]
    fn a_reused_voice_does_not_inherit_the_previous_notes_fm_taps() {
        let mut settings = model::Settings::default();
        settings.slots[0].kind = Kind::Wavetable;
        settings.add(Kind::Wavetable).unwrap();
        settings.slots[0].controls.wavetable.fm_source = 1;
        settings.slots[0].controls.wavetable.fm_mode = 1;
        settings.slots[0].controls.wavetable.fm_index = 0.8;
        settings.slots[1].controls.transpose = 7.0;
        let mut r = Runtime::new(48000, &settings, &Pool::default()).unwrap();
        let sample = SampleBuffer::new("Unused".into(), 48000, vec![[0.0; 2]; 128]).unwrap();
        let env = [0.0; crate::preset::MODULATORS];
        let inputs = [Input::default(); SLOTS];
        for stamp in 0..200 {
            r.begin_frame(true);
            r.render(
                0,
                1,
                60.0,
                &env,
                &env,
                inputs,
                0,
                0,
                [1.0; SLOTS],
                &sample,
                &(0..128),
                60.0,
                None,
                stamp,
            );
        }
        for stamp in 200..400 {
            r.begin_frame(true);
            let (_, reused, _) = r.render(
                0,
                2,
                60.0,
                &env,
                &env,
                inputs,
                0,
                0,
                [1.0; SLOTS],
                &sample,
                &(0..128),
                60.0,
                None,
                stamp,
            );
            let (_, fresh, _) = r.render(
                1,
                1,
                60.0,
                &env,
                &env,
                inputs,
                0,
                0,
                [1.0; SLOTS],
                &sample,
                &(0..128),
                60.0,
                None,
                stamp,
            );
            assert_eq!(reused, fresh);
        }
    }
}
