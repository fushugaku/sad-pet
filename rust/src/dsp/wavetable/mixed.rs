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
        Ok(Self {
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
            voices: (0..crate::dsp::VOICES)
                .map(|_| MixedVoice::default())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
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
        })
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
    }
    pub fn configure(&mut self, controls: [model::Controls; SLOTS], transpose: f32) {
        self.transpose = transpose;
        for (i, c) in controls.into_iter().enumerate() {
            self.native_controls[i] = c.native(self.kinds[i]);
            self.programs[i] = primary_program(self.native_controls[i]);
            self.layers[i].configure(c.table(), self.rate as f32);
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
        mut transports: Option<&mut crate::dsp::phase_transport::Runtime>,
        stamp: u64,
    ) -> ([SlotFrame; SLOTS], Stereo, Stereo) {
        let mut result = [SlotFrame::default(); SLOTS];
        if index >= self.voices.len() || !note.is_finite() {
            return (result, [0.0; 2], [0.0; 2]);
        }
        let voice = &mut self.voices[index];
        if voice.generation != generation {
            voice.taps.fill(0.0);
            voice.edges.fill(-1.0);
            voice.transition_left.fill(0);
            voice.tails.fill([0.0; 2]);
            voice.mix_reduction = interaction::Filter::default();
            voice.bypass_reduction = interaction::Filter::default();
            voice.bypass_quiet = u16::MAX;
        }
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
        let mut views: [[Option<interaction::Lookup<'_>>; MAX_UNISON]; SLOTS] =
            [[None; MAX_UNISON]; SLOTS];
        for i in 0..SLOTS {
            let layer = &self.layers[i];
            let Some(bank) = &self.banks[i] else {
                continue;
            };
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
                let mut p = (position + layer.position_offsets[j].value * layer.ramps[13].value)
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
        for _ in 0..SLOTS {
            for i in 0..SLOTS {
                if needed & (1 << i) == 0 {
                    continue;
                }
                let c = self.layers[i].target;
                let source =
                    if self.kinds[i] == Kind::Wavetable && c.fm_mode != 0 && c.fm_index > 0.0 {
                        Some(c.fm_source as usize)
                    } else if self.native_controls[i].mode == crate::resynth::Mode::Cross
                        && self.native_controls[i].cross_source > 0
                    {
                        Some(self.native_controls[i].cross_source as usize - 1)
                    } else {
                        None
                    };
                if let Some(source) = source
                    && source < SLOTS
                    && self.identities[source] != 0
                    && self.layers[source].target.enabled
                {
                    needed |= 1 << source;
                }
                let sync_source = c.sync_source.min(7) as usize;
                if c.sync
                    && self.identities[sync_source] != 0
                    && self.layers[sync_source].target.enabled
                {
                    needed |= 1 << sync_source;
                }
            }
        }
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
                            let audio = voice.tails[i][c] * fade;
                            into[c] += audio * capture[i];
                            bypass[c] += audio * (1.0 - capture[i]);
                        }
                    }
                    continue;
                }
                let input = inputs[i];
                let mut raw = [0.0; 2];
                let mut heard = [0.0; 2];
                if self.kinds[i] == Kind::Wavetable {
                    let norm = layer.lanes.iter().map(|l| l.value).sum::<f32>().max(1.0);
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
                    into[c] += audio[c] * capture[i];
                    bypass[c] += audio[c] * (1.0 - capture[i]);
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
