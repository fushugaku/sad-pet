#[cfg(test)]
mod bar_loop_tests;
pub mod color;
pub mod compressor;
pub mod delay;
pub mod effects;
pub mod fast;
pub mod filter;
pub mod filter_slots;
mod grain_groups;
mod imported_rate;
pub mod modulation;
mod radias_filter;
mod radias_oscillator;
mod radias_wavefolder;
pub mod radias_waveshaper;
pub mod resynth;
pub mod sample;
pub mod sampler;
pub(crate) mod sequence;
pub mod source_fx;
#[cfg(all(feature = "source-worker", not(target_arch = "wasm32")))]
pub mod source_fx_worker;
pub mod synth;
mod taps;
pub mod telemetry;

use crate::{
    Stereo,
    color::{ColorSlot, MAX_COLOR_SLOTS},
    matrix::{self, Source},
    midi::Message,
    performance::{Mode, Performance},
    preset::{Destination, Env, FilterPosition, FilterSlot, Lfo, MAX_FILTER_SLOTS, Params, Preset},
    tables::{self, Random},
    taps::Stage,
};
use color::ColorRack;
use filter::FilterBank;
use filter_slots::Instance;
use modulation::{Envelope, Oscillator};
use sample::LiveInput;
pub use sample::{Fades, Region, SampleBuffer};
use serde::Serialize;
use std::sync::Arc;

pub const MAX_GRAINS: usize = 128;
const _: () = assert!(
    MAX_GRAINS == u128::BITS as usize,
    "one occupancy bit per grain slot"
);
/// Voice 0 is the free voice; notes take the other 32, so long releases
/// overlap under fast sequences before a tail is stolen.
const VOICES: usize = 33;
const _: () = assert!(VOICES <= u64::BITS as usize, "one sounding bit per voice");
const CONTROL_INTERVAL: u64 = 32;
/// Sampler pads auditioned from the editor own their notes under this bit.
const AUDITION: u32 = 1 << 31;

#[derive(Clone, Copy, Debug, Default)]
struct Grain {
    position: f64,
    rate: f64,
    read_level: usize,
    age: u32,
    duration: u32,
    shape: f32,
    pan: Stereo,
    voice: usize,
}

/// Set bits of `mask`, lowest first.
fn bits(mut mask: u64) -> impl Iterator<Item = usize> {
    std::iter::from_fn(move || {
        (mask != 0).then(|| {
            let bit = mask.trailing_zeros() as usize;
            mask &= mask - 1;
            bit
        })
    })
}

#[derive(Clone, Copy, Debug, Default)]
struct Voice {
    owner: u32,
    envelopes: [Envelope; crate::preset::MODULATORS],
    note: u8,
    channel: u8,
    velocity: f32,
    held: bool,
    sustained: bool,
    /// Still held, but a Main pitch track closed its ENV gate between steps.
    gated: bool,
    value: f32,
    env_values: [f32; crate::preset::MODULATORS],
    loop_phase: f64,
    /// Counts note-ons, so Synth engine carriers restart with each note.
    generation: u32,
}
impl Voice {
    fn on(&mut self, count: usize) {
        self.generation = self.generation.wrapping_add(1);
        self.gated = false;
        self.loop_phase = 0.0;
        self.env_values = [0.0; crate::preset::MODULATORS];
        self.value = 0.0;
        for (i, env) in self.envelopes.iter_mut().enumerate() {
            if i < count {
                env.on();
            } else {
                env.kill();
            }
        }
    }
    fn off(&mut self) {
        for env in &mut self.envelopes {
            env.off();
        }
    }
    fn kill(&mut self) {
        for env in &mut self.envelopes {
            env.kill();
        }
    }
    fn active(&self) -> bool {
        self.envelopes.iter().any(Envelope::active)
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, serde::Deserialize)]
pub struct GrainView {
    pub position: f32,
    pub age: f32,
    pub pan: f32,
    /// 1–4 for the advanced mode group the grain joined, 0 outside it.
    #[serde(default)]
    pub group: u8,
}

#[derive(Clone, Copy, Default)]
pub(crate) struct Modulated {
    pub(crate) params: Params,
    pub(crate) pan: f32,
    pub(crate) color_slots: [ColorSlot; MAX_COLOR_SLOTS],
    pub(crate) color_len: usize,
    pub(crate) pre_color_slots: [ColorSlot; MAX_COLOR_SLOTS],
    pub(crate) pre_color_len: usize,
    pub(crate) lfos: [Lfo; crate::preset::MODULATORS],
    pub(crate) envs: [Env; crate::preset::MODULATORS],
    pub(crate) lfo_count: usize,
    pub(crate) env_count: usize,
    pub(crate) performance: crate::performance::Modulation,
    pub(crate) resynth: crate::resynth::Settings,
    pub(crate) taps: crate::taps::Taps,
    pub(crate) drums: crate::drums::Controls,
    /// Each drum track's pattern note shift, in steps.
    pub(crate) drum_shifts: [f32; crate::drums::TRACKS],
    pub(crate) synth: crate::synth::Settings,
    /// A modulation route wrote into a color slot of this frame. Clean frames
    /// carry the preset's own slots, so the racks need no reconfiguration.
    pub(crate) color_dirty: bool,
    pub(crate) filter_slots: [FilterSlot; MAX_FILTER_SLOTS],
    pub(crate) filter_len: usize,
    pub(crate) grain_groups: crate::grain_groups::Settings,
}
/// An LFO or ENV of a frame, by index.
#[derive(Clone, Copy)]
pub(crate) enum Modulator {
    Lfo(usize),
    Env(usize),
}
impl Modulated {
    fn from_preset(preset: &Preset) -> Self {
        let mut result = Self::default();
        result.write_from_preset(preset);
        result
    }
    /// Rebuild from the preset without a fresh frame. Only the active color
    /// slots are copied: every reader slices by `color_len`/`pre_color_len`,
    /// so slots beyond them are never observed.
    fn write_from_preset(&mut self, preset: &Preset) {
        self.params = preset.params;
        self.pan = 0.5;
        self.color_len = preset.color_chain.len();
        self.color_slots[..self.color_len].copy_from_slice(&preset.color_chain);
        self.pre_color_len = preset.pre_color_chain.len();
        self.pre_color_slots[..self.pre_color_len].copy_from_slice(&preset.pre_color_chain);
        self.lfos = preset.lfos;
        self.envs = preset.envs;
        self.lfo_count = preset.lfo_count as usize;
        self.env_count = preset.env_count as usize;
        self.performance = crate::performance::Modulation::from_settings(&preset.performance);
        self.resynth = preset.resynth;
        self.taps = preset.taps;
        self.drums = preset
            .drums
            .as_ref()
            .map_or_else(Default::default, |d| d.params);
        self.drum_shifts = [0.0; crate::drums::TRACKS];
        if let Some(d) = &preset.drums {
            for (to, track) in self.drum_shifts.iter_mut().zip(&d.tracks) {
                *to = track.shift as f32;
            }
        }
        self.synth = preset.synth.as_deref().copied().unwrap_or_default();
        self.color_dirty = false;
        self.filter_len = preset.filter_slots.len();
        self.filter_slots[..self.filter_len].copy_from_slice(&preset.filter_slots);
        self.grain_groups = preset.grain_groups;
    }
    /// An LFO's or ENV's routes, each at its depth as modulated so far.
    pub(crate) fn modulator_routes(
        &self,
        modulator: Modulator,
    ) -> [(Destination, f32); crate::preset::MODULATOR_ROUTES] {
        match modulator {
            Modulator::Lfo(i) => self.lfos[i].routes(),
            Modulator::Env(i) => self.envs[i].routes(),
        }
    }
    /// The LFOs, then the ENVs, in the order their routes apply: each after
    /// every one that routes into its depths, so its routes carry that
    /// modulation. Where such routes loop, the first pending one goes first.
    pub(crate) fn modulator_order(&self) -> impl Iterator<Item = Modulator> + use<> {
        use crate::preset::MODULATORS as N;
        use matrix::{EnvParameter as E, LfoParameter as L};
        let node = |n: usize| {
            if n < N {
                Modulator::Lfo(n)
            } else {
                Modulator::Env(n - N)
            }
        };
        // Bit n for node n: LFO i is node i, ENV i is node N + i.
        let active =
            ((1u16 << self.lfo_count.min(N)) - 1) | (((1u16 << self.env_count.min(N)) - 1) << N);
        let mut inputs = [0u16; 2 * N];
        for source in (0..2 * N).filter(|&n| active & (1 << n) != 0) {
            for (destination, _) in self.modulator_routes(node(source)) {
                let target = match destination {
                    Destination::Lfo {
                        index,
                        parameter: L::Depth | L::Depth2 | L::Depth3,
                    } if (index as usize) < N => index as usize,
                    Destination::Env {
                        index,
                        parameter: E::Depth | E::ExtraDepth | E::Depth3,
                    } if (index as usize) < N => N + index as usize,
                    _ => continue,
                };
                if target != source {
                    inputs[target] |= 1 << source;
                }
            }
        }
        let mut pending = active;
        std::iter::from_fn(move || {
            if pending == 0 {
                return None;
            }
            let next = (0..2 * N)
                .find(|&n| pending & (1 << n) != 0 && inputs[n] & pending == 0)
                .unwrap_or(pending.trailing_zeros() as usize);
            pending &= !(1 << next);
            Some(node(next))
        })
    }
    fn route(&mut self, destination: Destination, value: f32) {
        if value == 0.0 {
            return;
        }
        if let Destination::Control { index } = destination {
            if let Some(spec) = crate::parameters::specs().get(index as usize)
                && spec.can_sequence()
            {
                spec.frame_modulate(self, value);
            }
            return;
        }
        use matrix::{
            EnvParameter as E, LfoParameter as L, Parameter as P, PerformanceParameter as F,
        };
        let variable = match destination {
            Destination::FilterSlot { slot, parameter } => self.filter_slots[..self.filter_len]
                .iter_mut()
                .find(|s| s.id == slot)
                .map(|s| {
                    let (target, min, max) = s.parameter_mut(parameter);
                    (target, min, max, false)
                }),
            Destination::Parameter { parameter } => {
                let p = &mut self.params;
                Some(match parameter {
                    P::FilterEnvDepth => (&mut p.filter_env_depth, -1.0, 1.0, false),
                    P::FilterKeytrack => (&mut p.filter_keytrack, 0.0, 1.0, false),
                    P::Filter2EnvDepth => (&mut p.filter2_env_depth, -1.0, 1.0, false),
                    P::Filter2Keytrack => (&mut p.filter2_keytrack, 0.0, 1.0, false),
                    P::FilterDrive => (&mut p.filter_drive, 0.0, 1.0, false),
                    P::FilterMorph => (&mut p.filter_morph, 0.0, 1.0, false),
                    P::Filter2Drive => (&mut p.filter2_drive, 0.0, 1.0, false),
                    P::Filter2Morph => (&mut p.filter2_morph, 0.0, 1.0, false),
                    P::Tempo => (&mut p.bpm, 20.0, 300.0, false),
                    P::InputGain => (&mut p.in_gain, -20.0, 20.0, false),
                    P::OutputGain => (&mut p.out_gain, -12.0, 12.0, false),
                    P::PitchRange => (&mut p.pitch_range, 12.0, 48.0, false),
                    P::SampleTuneSemis => (&mut p.sample_tune_semis, -48.0, 48.0, false),
                    P::SampleTuneCents => (&mut p.sample_tune_cents, -100.0, 100.0, false),
                    P::PitchBendRange => (&mut p.midi_pitchbend_range, 0.0, 48.0, false),
                    P::MinGrains => (&mut p.min_grains, 0.0, 1.0, false),
                })
            }
            Destination::Env { index, parameter }
                if (index as usize) < crate::preset::MODULATORS =>
            {
                let e = &mut self.envs[index as usize];
                Some(match parameter {
                    E::Attack => (&mut e.attack, 0.0, 1.0, false),
                    E::Decay => (&mut e.decay, 0.0, 1.0, false),
                    E::Sustain => (&mut e.sustain, 0.0, 1.0, false),
                    E::Release => (&mut e.release, 0.0, 1.0, false),
                    E::Depth => (&mut e.depth, -1.0, 1.0, false),
                    E::ExtraDepth => (&mut e.extra_depth, -1.0, 1.0, false),
                    E::Depth3 => (&mut e.depth3, -1.0, 1.0, false),
                })
            }
            Destination::Lfo { index, parameter }
                if (index as usize) < crate::preset::MODULATORS =>
            {
                let l = &mut self.lfos[index as usize];
                Some(match parameter {
                    L::Rate => (&mut l.rate_hz, 0.01, 40.0, true),
                    L::Depth => (&mut l.depth, -1.0, 1.0, false),
                    L::Division => (&mut l.division, 0.0625, 32.0, true),
                    L::Depth2 => (&mut l.depth2, -1.0, 1.0, false),
                    L::Depth3 => (&mut l.depth3, -1.0, 1.0, false),
                })
            }
            Destination::Performance { parameter } => {
                let p = &mut self.performance;
                Some(match parameter {
                    F::ArpRate => (&mut p.arp_rate, 0.0625, 8.0, true),
                    F::ArpGate => (&mut p.arp_gate, 0.05, 0.95, false),
                    F::ArpOctaves => (&mut p.arp_octaves, 1.0, 4.0, false),
                    F::SequenceRate => (&mut p.sequence_rate, 0.0625, 16.0, true),
                    F::SequenceGate => (&mut p.sequence_gate, 0.05, 0.95, false),
                    F::SequenceLength => (&mut p.sequence_length, 1.0, 16.0, false),
                })
            }
            Destination::Step { index, parameter } if index < 16 => {
                let step = &mut self.performance.steps[index as usize];
                match parameter {
                    matrix::StepParameter::Note => {
                        step.notes.transpose((value * 127.0).round() as i16);
                    }
                    matrix::StepParameter::Velocity => {
                        step.velocity = (step.velocity as f32 + value * 126.0)
                            .round()
                            .clamp(1.0, 127.0) as u8
                    }
                }
                return;
            }
            _ => None,
        };
        if let Some((target, min, max, logarithmic)) = variable {
            if logarithmic {
                matrix::add_log(target, value, min, max);
            } else {
                matrix::add(target, value, min, max);
            }
            return;
        }
        if let Some(parameter) = crate::color::legacy_reverb_parameter(destination)
            && let Some(slot) = self.color_slots[..self.color_len]
                .iter_mut()
                .find(|s| s.kind == crate::preset::SaturatorType::Reverb)
        {
            let target = slot.parameter_mut(parameter);
            *target = (*target + value).clamp(0.0, 1.0);
            self.color_dirty = true;
            crate::color::mirror_reverb(&mut self.params, &self.color_slots[..self.color_len]);
            if self.color_len > 0 {
                self.params.saturation = self.color_slots[0].mix;
            }
            return;
        }
        let p = &mut self.params;
        let target = match destination {
            Destination::None => return,
            Destination::Position => &mut p.position,
            Destination::Pitch => &mut p.pitch,
            Destination::GrainSize => &mut p.grain_size,
            Destination::Grains => &mut p.grains,
            Destination::Filter => &mut p.filter,
            Destination::Mix => &mut p.mix,
            Destination::Pan => &mut self.pan,
            Destination::Reverb => &mut p.reverb,
            Destination::GrainShape => &mut p.grain_shape,
            Destination::Detune => &mut p.detune,
            Destination::Cloud => &mut p.cloud,
            Destination::StereoSpread => &mut p.stereo_spread,
            Destination::FilterResonance => &mut p.filter_resonance,
            Destination::Filter2 => &mut p.filter2,
            Destination::Filter2Resonance => &mut p.filter2_resonance,
            Destination::Saturation => {
                if self.color_len > 0 {
                    self.color_dirty = true;
                    &mut self.color_slots[0].mix
                } else {
                    &mut p.saturation
                }
            }
            Destination::Color { slot, parameter } => {
                let Some(effect) = self.color_slots[..self.color_len]
                    .iter_mut()
                    .chain(self.pre_color_slots[..self.pre_color_len].iter_mut())
                    .find(|s| s.id == slot)
                else {
                    return;
                };
                self.color_dirty = true;
                if parameter == crate::color::ColorParameter::LcrRatio {
                    effect.settings.lcr_ratio = crate::radias_delay::modulate_lcr_ratio(
                        effect.settings.lcr_sync,
                        effect.settings.lcr_ratio,
                        value,
                    );
                    return;
                }
                effect.parameter_mut(effect.playing_time(parameter))
            }
            Destination::ReverbDecay => &mut p.reverb_decay,
            Destination::ReverbTone => &mut p.reverb_tone,
            Destination::Predelay => &mut p.predelay,
            Destination::Shimmer => &mut p.shimmer,
            Destination::Diffuse => &mut p.diffuse,
            Destination::Volume => &mut p.master_volume,
            _ => return,
        };
        *target = (*target + value).clamp(0.0, 1.0);
        if self.color_len > 0 {
            p.saturation = self.color_slots[0].mix;
        }
        crate::color::mirror_reverb(p, &self.color_slots[..self.color_len]);
    }
}

/// The large members (per-voice state, color racks, Resynth) are boxed, so an
/// engine stays small however many voices it has: debug builds copy it on the
/// stack at every by-value move, and tests hold several on a 2 MB thread.
pub struct Engine {
    pub sample_rate: u32,
    pub preset: Preset,
    pub sample: Arc<SampleBuffer>,
    pub live_input: Option<LiveInput>,
    pub region: Region,
    pub fades: Fades,
    file_range: std::ops::Range<usize>,
    pub playing: bool,
    pub frozen: bool,
    pub bypass: bool,
    pub midi_gate: bool,
    pub step_entry: Option<usize>,
    pub step_edits: [Option<crate::performance::Step>; 16],
    captured_keys: [u128; 16],
    performance: Performance,
    sequence_running: bool,
    grains: [Grain; MAX_GRAINS],
    /// Occupied grain slots, bit n for slot n. Visiting set bits from the lowest
    /// keeps the order of a full slot scan, so voices still sum in that order.
    live_grains: u128,
    /// Each grain's group, envelopes and filter in advanced mode.
    group_grains: Box<grain_groups::Runtime>,
    voices: Box<[Voice; VOICES]>,
    oscillators: [Oscillator; crate::preset::MODULATORS],
    pub lfo_values: [f32; crate::preset::MODULATORS],
    pub env_values: [f32; crate::preset::MODULATORS],
    color_rack: Box<ColorRack>,
    pre_color_rack: Box<ColorRack>,
    source_color: color::ColorProcessor,
    resynth: Box<resynth::Resynth>,
    /// Each section's Chain share and bus send.
    taps: taps::Runtime,
    /// What enters a Bus or Master chain after its input in the next frame.
    entered: Option<crate::timbres::Entered>,
    drums: Option<Box<crate::drums::Runtime>>,
    synth: Option<Box<synth::Runtime>>,
    /// Present while the preset is a Sampler kit: its pads replace the input.
    sampler: Option<Box<sampler::Runtime>>,
    pub processed_source: Option<Arc<source_fx::ProcessedSource>>,
    previous_source: Option<Arc<source_fx::ProcessedSource>>,
    source_fade_remaining: u32,
    pub source_fx_error: bool,
    filters: Box<[FilterBank; VOICES]>,
    /// Banks of the added filters, in preset slot order.
    filter_banks: Vec<Box<filter_slots::Bank>>,
    /// Enabled added filters per position, refreshed at control rate.
    filter_masks: [u16; filter_slots::POSITIONS],
    random: Random,
    frame: u64,
    cursor: f64,
    bar_beats: f64,
    frozen_center: f64,
    last_position: f32,
    grain_clock: f64,
    next_voice: usize,
    /// Voices whose envelopes may still run, bit n for voice n. The frame
    /// loop advances only these, so idle voices cost nothing per frame.
    sounding: u64,
    bend: [f32; 16],
    sustain: [bool; 16],
    follower: f32,
    source_note: u8,
    source_channel: u8,
    source_velocity: f32,
    source_random: f32,
    mod_random: Random,
    wheel: [f32; 16],
    expression: [f32; 16],
    pressure: [f32; 16],
    poly_pressure: [[f32; 128]; 16],
    pub matrix_values: [f32; matrix::MAX_ROUTES],
    modulated: Box<Modulated>,
    /// The previous frame, rebuilt in place on the next control block instead
    /// of constructing and copying a new frame.
    spare_modulated: Option<Box<Modulated>>,
    /// (preset revision, bpm bits, shaper pitch bits) the racks last saw.
    rack_key: Option<(u64, u32, u32)>,
    rack_configured_dirty: bool,
    /// (preset revision, bpm bits, sample root note bits) the source FX last saw.
    source_key: Option<(u64, u32, u32)>,
    color_revision: u64,
    applied_shaper_pitch: u32,
    overlap: f32,
    smooth_mix: f32,
    smooth_gain: f32,
    smooth_bypass_gate: f32,
    input_gain: f32,
    output_gain: f32,
    gain_key: Option<[u32; 2]>,
    grain_density: f32,
    grain_step: f64,
    bar_phase_step: f64,
    pub peak: Stereo,
}

impl Engine {
    pub fn new(sample_rate: u32, sample: Arc<SampleBuffer>, preset: Preset) -> Self {
        fast::warm_up();
        let sr = sample_rate.clamp(8_000, 384_000);
        let mut engine = Self {
            sample_rate: sr,
            preset: Preset::default(),
            region: Region::default(),
            fades: Fades::default(),
            file_range: 0..sample.len(),
            sample,
            live_input: None,
            playing: false,
            frozen: false,
            bypass: false,
            midi_gate: false,
            step_entry: None,
            step_edits: [None; 16],
            captured_keys: [0; 16],
            performance: Performance::default(),
            sequence_running: true,
            grains: [Grain::default(); MAX_GRAINS],
            live_grains: 0,
            group_grains: Box::new(grain_groups::Runtime::new(sr)),
            voices: Box::new([Voice::default(); VOICES]),
            oscillators: std::array::from_fn(|i| {
                Oscillator::new(0x853c49e6748fea9b_u64.wrapping_add(i as u64 * 0x12345679))
            }),
            lfo_values: [0.0; crate::preset::MODULATORS],
            env_values: [0.0; crate::preset::MODULATORS],
            color_rack: Box::new(ColorRack::new(sr)),
            pre_color_rack: Box::new(ColorRack::new(sr)),
            source_color: color::ColorProcessor::new(sr),
            resynth: Box::new(resynth::Resynth::new(sr)),
            taps: taps::Runtime::new(),
            entered: None,
            drums: None,
            synth: None,
            sampler: None,
            processed_source: None,
            previous_source: None,
            source_fade_remaining: 0,
            source_fx_error: false,
            filters: Box::new(std::array::from_fn(|_| FilterBank::new(sr))),
            filter_banks: Vec::new(),
            filter_masks: [0; filter_slots::POSITIONS],
            random: Random::default(),
            frame: 0,
            cursor: 0.0,
            bar_beats: 4.0,
            frozen_center: 0.0,
            last_position: 0.0,
            grain_clock: 1.0,
            next_voice: 0,
            sounding: 0,
            bend: [0.0; 16],
            sustain: [false; 16],
            follower: 0.0,
            source_note: 60,
            source_channel: 0,
            source_velocity: 0.0,
            source_random: 0.0,
            mod_random: Random(0x4d41545249585f31),
            wheel: [0.0; 16],
            expression: [1.0; 16],
            pressure: [0.0; 16],
            poly_pressure: [[0.0; 128]; 16],
            matrix_values: [0.0; matrix::MAX_ROUTES],
            modulated: Box::new(Modulated::default()),
            spare_modulated: Some(Box::new(Modulated::default())),
            rack_key: None,
            rack_configured_dirty: false,
            source_key: None,
            color_revision: 0,
            applied_shaper_pitch: 60.0_f32.to_bits(),
            overlap: 1.0,
            smooth_mix: 1.0,
            smooth_gain: 0.0,
            smooth_bypass_gate: 0.0,
            input_gain: 1.0,
            output_gain: 1.0,
            gain_key: None,
            grain_density: 0.0,
            grain_step: 0.0,
            bar_phase_step: 0.0,
            peak: [0.0; 2],
        };
        engine.set_preset(preset);
        // Idle voices need their initial coefficients even when the first preset
        // has the same envelopes as Modulated::default(). Later refreshes only
        // configure envelopes whose values actually changed.
        for voice in engine.voices.iter_mut() {
            for (i, envelope) in voice.envelopes.iter_mut().enumerate() {
                envelope.configure(sr as f32, &engine.modulated.envs[i]);
            }
        }
        engine
    }

    pub fn set_preset(&mut self, mut preset: Preset) {
        preset.sanitize();
        preset.prepare_effects_reusing(self.sample_rate, &self.preset);
        preset
            .drum_runtime
            .prepare(preset.drums.as_deref(), self.sample_rate);
        preset
            .synth_runtime
            .prepare(preset.synth.as_deref(), self.sample_rate);
        preset.bus_runtime.prepare_reusing(
            preset.bus.as_deref(),
            self.sample_rate,
            self.bus_history(),
        );
        preset.filter_banks.prepare_reusing(
            &preset.filter_slots,
            self.sample_rate,
            &mut self.filter_banks,
        );
        self.exchange_preset(&mut preset);
    }

    /// Exchange a sanitized preset without allocating or destroying its buffers.
    /// Structural FX/drum edits must use `prepared_clone_for(sample_rate)`.
    /// The caller retires the previous preset away from the audio callback.
    pub fn exchange_preset(&mut self, preset: &mut Preset) {
        // A bus history is never freed here: it retires with the outgoing
        // preset, whose bus_runtime is empty while that preset is installed.
        let was_bus = self.preset.bus.is_some();
        if was_bus && preset.bus.is_none() {
            std::mem::swap(&mut self.live_input, &mut self.preset.bus_runtime.0);
        }
        // A retiring synth restores the input it replaced before drums install.
        if preset.synth_runtime.0.is_none()
            && let Some(old) = &mut self.synth
        {
            std::mem::swap(&mut self.live_input, &mut old.live);
        }
        if let Some(next) = &mut preset.drum_runtime.0 {
            if let Some(old) = &mut self.drums {
                next.engine.take_audio(&mut old.engine);
                next.engine.inherit_clock(&old.engine);
                std::mem::swap(&mut next.live, &mut old.live);
            } else {
                std::mem::swap(&mut self.live_input, &mut next.live);
            }
        } else if let Some(old) = &mut self.drums {
            std::mem::swap(&mut self.live_input, &mut old.live);
        }
        std::mem::swap(&mut self.drums, &mut preset.drum_runtime.0);
        if let Some(next) = &mut preset.synth_runtime.0 {
            if let Some(old) = &mut self.synth {
                next.inherit(old);
            } else {
                std::mem::swap(&mut self.live_input, &mut next.live);
            }
        }
        std::mem::swap(&mut self.synth, &mut preset.synth_runtime.0);
        if let (Some(next), Some(old)) = (
            &mut preset.sequencer.runtime,
            &mut self.preset.sequencer.runtime,
        ) {
            next.inherit(old);
        }
        for voice in self.voices.iter_mut() {
            if voice.owner != 0
                && (!preset.sequencer.enabled
                    || !preset
                        .sequencer
                        .runtime
                        .as_ref()
                        .is_some_and(|r| r.retained_note(voice.owner)))
            {
                voice.held = false;
                voice.sustained = false;
                voice.off();
            }
        }
        if preset.performance.mode != self.performance.mode() {
            self.step_edits.fill(None);
        }
        for (i, pending) in self.step_edits.iter_mut().enumerate() {
            if let Some(step) = *pending {
                if preset.performance.steps[i] == step {
                    *pending = None;
                } else {
                    preset.performance.steps[i] = step;
                }
            }
        }
        let hold_chord_changed = preset.performance.mode == Mode::Hold
            && self.preset.performance.chord != preset.performance.chord;
        let mode_changed = self.performance.configure_prepared(&preset.performance);
        if mode_changed {
            self.step_entry = None;
        }
        self.color_rack
            .detach_imported(&mut self.preset.imported_effects);
        self.pre_color_rack
            .detach_imported(&mut self.preset.imported_effects);
        self.source_color
            .detach_imported(&mut self.preset.imported_effects);
        preset.imported_effects.inherit_matching(
            &mut self.preset.imported_effects,
            preset
                .color_chain
                .iter()
                .chain(&preset.pre_color_chain)
                .chain(&preset.source_color_chain),
        );
        self.color_rack
            .attach_imported(&preset.color_chain, &mut preset.imported_effects);
        self.pre_color_rack
            .attach_imported(&preset.pre_color_chain, &mut preset.imported_effects);
        if let Some(slot) = preset.source_color_chain.first() {
            self.source_color
                .attach_imported(*slot, &mut preset.imported_effects);
        }
        filter_slots::inherit(&mut preset.filter_banks.0, &mut self.filter_banks);
        std::mem::swap(&mut self.filter_banks, &mut preset.filter_banks.0);
        if let Some(bus) = &preset.bus {
            // Granular staying on keeps the running history and its audio.
            if !(bus.granular && self.bus_history()) {
                std::mem::swap(&mut self.live_input, &mut preset.bus_runtime.0);
            }
            std::mem::swap(&mut self.preset.bus_runtime.0, &mut preset.bus_runtime.0);
        }
        std::mem::swap(&mut self.preset, preset);
        self.sync_sampler();
        if !was_bus && self.preset.bus.is_some() {
            // The input passes at unity from the first frame, without a fade-in.
            self.voices[0].note = 60;
            self.smooth_gain = self.preset.params.master_volume
                * 10.0_f32.powf(self.preset.params.out_gain * 0.05);
        }
        self.invalidate_racks();
        self.sync_stored_chord();
        self.update_modulation();
        if mode_changed || hold_chord_changed {
            self.release_notes();
            self.midi_gate = self.performance.mode() != Mode::Keys;
            if self.playing {
                self.set_playing(true);
            }
        }
        self.refresh_sequence_free_voice();
    }
    pub fn set_sample(&mut self, sample: Arc<SampleBuffer>) {
        if self.preset.bus.is_none() {
            self.live_input = None;
        }
        self.sample = sample;
        self.reset_source();
    }
    /// Swap a prepared file cache without resetting held notes or loop phases.
    /// Return the older allocation to the caller for retirement off the callback.
    pub fn exchange_processed_source(
        &mut self,
        prepared: &mut Option<Arc<source_fx::ProcessedSource>>,
    ) {
        self.source_fx_error = false;
        if self.processed_source.is_none() && prepared.is_none() {
            return;
        }
        std::mem::swap(&mut self.processed_source, prepared);
        std::mem::swap(&mut self.previous_source, prepared);
        self.source_fade_remaining = if self.playing { 128 } else { 0 };
    }
    pub fn prepare_source_effects(&mut self) -> Result<(), String> {
        let mut prepared = source_fx::prepare(
            self.sample.clone(),
            self.region,
            self.preset.source_color_chain.first().copied(),
            self.preset.params.bpm,
            self.preset.params.sample_root_note,
            || false,
        )?;
        self.exchange_processed_source(&mut prepared);
        self.source_fade_remaining = 0;
        Ok(())
    }
    /// File-load/control boundary only: retires the previous file's cache pool.
    pub fn clear_processed_sources(&mut self) {
        self.processed_source = None;
        self.previous_source = None;
        self.source_fade_remaining = 0;
    }
    pub fn source_effects_pending(&self) -> bool {
        if self.live_input.is_some() || self.source_fx_error {
            return false;
        }
        let slot = self
            .preset
            .source_color_chain
            .first()
            .copied()
            .filter(|s| s.enabled && s.mix > 0.0);
        let recipe = source_fx::Recipe::from_engine(self);
        match (&self.processed_source, slot) {
            (Some(cache), Some(slot)) => {
                !(Arc::ptr_eq(&cache.original, &self.sample)
                    && cache.range == self.file_range
                    && cache.slot == slot
                    && cache.bpm == recipe.bpm
                    && cache.root == recipe.root)
            }
            (None, None) => false,
            _ => true,
        }
    }
    /// Swap prepared file/live sources, returning their old allocations to the caller.
    pub fn exchange_source(
        &mut self,
        sample: &mut Arc<SampleBuffer>,
        live_input: &mut Option<LiveInput>,
    ) {
        std::mem::swap(&mut self.sample, sample);
        std::mem::swap(&mut self.live_input, live_input);
        self.reset_source();
    }
    fn reset_source(&mut self) {
        self.region = Region::default();
        self.fades = Fades::default();
        self.file_range = 0..self.sample.len();
        self.cursor = 0.0;
        self.voices.iter_mut().for_each(|v| v.loop_phase = 0.0);
        self.frozen_center = 0.0;
        self.reset_grains();
        self.color_rack.clear();
        self.pre_color_rack.clear();
        self.source_color.clear();
        self.invalidate_racks();
        self.resynth.clear();
        self.clear_filters();
    }

    /// Tempo is measured in quarter notes; e.g. 6/8 is three quarter notes per bar.
    pub fn set_time_signature(&mut self, numerator: u32, denominator: u32) {
        self.bar_beats = if (1..=128).contains(&numerator)
            && denominator.is_power_of_two()
            && denominator <= 128
        {
            numerator as f64 * 4.0 / denominator as f64
        } else {
            4.0
        };
        self.bar_phase_step =
            self.modulated.params.bpm as f64 / (60.0 * self.sample_rate as f64 * self.bar_beats);
    }
    pub fn bar_beats(&self) -> f64 {
        self.bar_beats
    }
    fn bar_loop_active(&self) -> bool {
        self.modulated.params.loop_one_bar && self.live_input.is_none()
    }
    fn voice_center(&self, voice: usize) -> f64 {
        if self.bar_loop_active() && !self.frozen {
            let range = &self.file_range;
            range.start as f64
                + (self.voices[voice].loop_phase + self.modulated.params.position as f64)
                    .rem_euclid(1.0)
                    * range.len() as f64
        } else {
            self.center()
        }
    }

    /// `restart` moves the parts that read the beat (drums, Hold/Arp and synced
    /// LFOs) to `beat`. Neuropolis counts ticks from its own start instead and
    /// keeps its playheads across preset changes: `sequencer` restarts it too.
    pub fn host_transport(&mut self, bpm: f32, beat: Option<f64>, restart: bool, sequencer: bool) {
        if bpm.is_finite() && (20.0..=300.0).contains(&bpm) {
            self.preset.params.bpm = bpm;
        }
        if restart {
            if let Some(drums) = &mut self.drums {
                drums
                    .engine
                    .seek_clock(beat.filter(|b| b.is_finite()).unwrap_or(0.));
            }
            if self.performance.mode() != Mode::Keys {
                self.release_notes();
            } else if sequencer {
                self.release_sequence_notes();
            }
            if sequencer && let Some(runtime) = &mut self.preset.sequencer.runtime {
                runtime.seek(beat.filter(|b| b.is_finite()).unwrap_or(0.0));
            }
            if let Some(beat) = beat.filter(|b| b.is_finite()) {
                self.performance.seek(beat);
                for (oscillator, lfo) in self.oscillators.iter_mut().zip(self.preset.lfos) {
                    if lfo.sync {
                        oscillator.phase = (beat / lfo.division as f64).rem_euclid(1.0);
                    }
                }
            }
        }
    }

    /// Refresh numeric host automation without rebuilding the preset or effects.
    pub fn refresh_parameters(&mut self) {
        self.performance.refresh_timing(&self.preset.performance);
        self.invalidate_racks();
        self.update_modulation();
    }
    /// The preset or a rack history changed outside modulation: the next
    /// control block must present every slot to its processor again.
    fn invalidate_racks(&mut self) {
        self.color_revision = self.color_revision.wrapping_add(1);
    }
    fn sequence_has_notes(&self) -> bool {
        self.sequence_running
            && self.preset.sequencer.enabled
            && self
                .preset
                .sequencer
                .tracks
                .iter()
                .any(|t| t.target == crate::sequence::Target::Notes && t.config[8] == 0)
    }
    fn sequence_gates_envelopes(&self) -> bool {
        self.sequence_running
            && self.preset.sequencer.enabled
            && self.preset.sequencer.tracks.iter().any(|t| {
                t.target == crate::sequence::Target::MainPitch && t.env_gate && t.config[8] == 0
            })
    }
    fn refresh_sequence_free_voice(&mut self) {
        if self.preset.bus.is_some() {
            // A bus voice is permanent; the transport neither starts nor ends it.
            return;
        }
        if self.playing && !self.midi_gate && self.performance.mode() == Mode::Keys {
            if self.sequence_has_notes() {
                self.voices[0].held = false;
                self.voices[0].off();
            } else if !self.voices[0].held
                || (self.voices[0].gated && !self.sequence_gates_envelopes())
            {
                self.start_free_voice();
            }
        }
    }
    /// Starts a voice's envelopes and lets the frame loop advance them.
    fn start_voice(&mut self, index: usize) {
        self.voices[index].on(self.preset.env_count as usize);
        self.sounding |= 1 << index;
    }
    /// Voice 0 is the note held while playing without keys or note tracks.
    fn start_free_voice(&mut self) {
        self.voices[0].velocity = 1.0;
        self.voices[0].note = 60;
        self.voices[0].held = true;
        // ENV gates retrigger a voice that may still sound: keep its filters.
        if !self.voices[0].active() {
            self.clear_voice_filters(0);
        }
        self.start_voice(0);
        self.mod_note_on(0, 60, 127);
        self.configure_voice_filter(0);
    }
    pub fn apply_parameters(&mut self, changes: &[crate::parameters::Change]) {
        if crate::parameters::apply(&mut self.preset, changes) {
            self.refresh_parameters();
        }
    }
    pub fn apply_sequence_edits(&mut self, edits: &[crate::sequence::Edit]) {
        self.preset.sequencer.edit(edits);
        for edit in edits
            .iter()
            .filter(|e| e.stage.is_none() && (e.field == 47 || (e.field == 8 && e.value >= 0.5)))
        {
            for voice in self.voices.iter_mut() {
                if voice.owner == edit.track {
                    voice.held = false;
                    voice.sustained = false;
                    voice.off();
                }
            }
            if let (Some(sampler), Some(settings)) = (&mut self.sampler, &self.preset.sampler) {
                sampler.release_owner(settings, edit.track);
            }
        }
        if !self.preset.sequencer.enabled || edits.iter().any(|e| e.track == 0 && e.field == 1) {
            self.release_sequence_notes();
            if let Some(runtime) = &mut self.preset.sequencer.runtime {
                runtime.restart();
            }
        }
        self.invalidate_racks();
        self.update_modulation();
        self.refresh_sequence_free_voice();
    }
    fn release_sequence_notes(&mut self) {
        for voice in self.voices.iter_mut() {
            if voice.owner != 0 {
                voice.held = false;
                voice.sustained = false;
                voice.off();
            }
        }
        if let (Some(sampler), Some(settings)) = (&mut self.sampler, &self.preset.sampler) {
            sampler.release_sequenced(settings);
        }
    }
    /// DAW transport stops the sequencer while live input and keyboard audition
    /// keep their normal audio path. Standalone playback owns this clock itself.
    pub fn set_sequence_transport(&mut self, running: bool) {
        if self.sequence_running == running {
            return;
        }
        self.sequence_running = running;
        if let Some(drums) = &mut self.drums {
            if running {
                drums.engine.play();
            } else {
                drums.engine.stop();
            }
        }
        self.release_sequence_notes();
        if let Some(runtime) = &mut self.preset.sequencer.runtime {
            runtime.start(running && self.preset.sequencer.enabled);
        }
        self.invalidate_racks();
        self.update_modulation();
        self.refresh_sequence_free_voice();
    }
    pub fn set_sequence_view(&mut self, offset: usize) {
        if let Some(runtime) = &mut self.preset.sequencer.runtime {
            runtime.view_offset = offset.min(self.preset.sequencer.tracks.len().saturating_sub(1));
        }
    }
    pub fn sequence_view(&self) -> crate::sequence::View {
        self.preset
            .sequencer
            .runtime
            .as_ref()
            .map_or_else(crate::sequence::View::default, |r| r.view())
    }
    pub fn set_region(&mut self, region: Region) -> Result<(), String> {
        if self.live_input.is_some() {
            return Err("Load a recording or file to select a region".into());
        }
        let range = region.bounds(self.sample.len())?;
        if range == self.file_range {
            return Ok(());
        }
        self.region = Region {
            start: range.start as f64 / self.sample.len() as f64,
            end: range.end as f64 / self.sample.len() as f64,
        };
        self.cursor = (self.cursor.max(range.start as f64) - range.start as f64)
            .rem_euclid(range.len() as f64)
            + range.start as f64;
        self.frozen_center = (self.frozen_center.max(range.start as f64) - range.start as f64)
            .rem_euclid(range.len() as f64)
            + range.start as f64;
        self.file_range = range;
        self.reset_grains();
        self.color_rack.clear();
        self.pre_color_rack.clear();
        self.source_color.clear();
        self.invalidate_racks();
        self.clear_filters();
        Ok(())
    }
    fn source_range(&self) -> std::ops::Range<usize> {
        self.live_input.as_ref().map_or_else(
            || self.file_range.clone(),
            |input| {
                if self.preset.history().is_some() {
                    input.range()
                } else {
                    0..input.sample.len()
                }
            },
        )
    }
    pub fn set_fades(&mut self, fades: Fades) -> Result<(), String> {
        if self.live_input.is_some() {
            return Err("Load a recording or file to set fades".into());
        }
        self.fades = fades.validate()?;
        Ok(())
    }
    pub fn set_live_input(&mut self, enabled: bool, effects_only: bool) {
        if self.preset.history().is_some() {
            // Drum and Synth timbres record their own output.
            return;
        }
        if enabled {
            if let Some(input) = &mut self.live_input {
                input.effects_only = effects_only;
            } else {
                self.live_input = Some(LiveInput::new(self.sample_rate, effects_only));
            }
        } else {
            self.live_input = None;
        }
        self.frozen = false;
        self.cursor = self.file_range.start as f64;
        self.reset_grains();
        self.color_rack.clear();
        self.pre_color_rack.clear();
        self.source_color.clear();
        self.invalidate_racks();
        self.clear_filters();
        if !self.playing {
            for voice in self.voices.iter_mut() {
                voice.kill();
            }
            self.smooth_bypass_gate = 0.0;
            self.peak = [0.0; 2];
        }
    }
    /// A Bus or Master whose Granular history is allocated and running.
    fn bus_history(&self) -> bool {
        self.preset.bus.as_ref().is_some_and(|b| b.granular) && self.live_input.is_some()
    }
    /// A Bus or Master with nothing switched on passes its input through.
    fn bus_passthrough(&self) -> bool {
        self.preset.bus.as_ref().is_some_and(|bus| {
            !bus.filters
                && !self.bus_history()
                && !self.bypass
                && !self.resynth.active()
                && !self.taps.active()
                && !self.pre_color_rack.is_active()
                && !self.color_rack.is_active()
                && self.preset.source_color_chain.is_empty()
                && self.filter_masks.iter().all(|&mask| mask == 0)
        })
    }
    /// Input times In/Out gain and Volume, without any processing or clipping.
    /// At the default unity levels this is the input itself, bit for bit.
    fn pass_bus(&mut self, input: Stereo) -> Stereo {
        let gain = self.modulated.params.master_volume * self.output_gain;
        self.smooth_gain = fast::flush(self.smooth_gain + (gain - self.smooth_gain) * 0.002);
        let output = input.map(|x| x * self.input_gain * self.smooth_gain);
        for (peak, value) in self.peak.iter_mut().zip(output) {
            *peak = peak.max(value.abs());
        }
        self.frame += 1;
        output
    }
    pub fn source_sample(&self) -> &SampleBuffer {
        self.live_input
            .as_ref()
            .map_or(&self.sample, |input| &input.sample)
    }
    pub fn source_frames(&self) -> usize {
        self.preset.history().map_or_else(
            || self.source_sample().len(),
            |history| history.buffer_frames(self.sample_rate),
        )
    }
    pub fn source_duration(&self) -> f64 {
        if self.preset.history().is_some() {
            self.source_frames() as f64 / self.sample_rate as f64
        } else {
            self.source_sample().duration()
        }
    }
    pub fn source_position(&self, position: f64) -> f64 {
        if self.preset.history().is_some()
            && let Some(input) = &self.live_input
        {
            input.display_position(position)
        } else {
            position.rem_euclid(self.source_sample().len() as f64)
                / self.source_sample().len() as f64
        }
    }
    fn grain_seconds(&self, size: f32) -> f32 {
        self.preset.history().map_or_else(
            || tables::grain_duration(size),
            |history| history.grain_seconds(size),
        )
    }
    pub fn reset_grains(&mut self) {
        self.live_grains = 0;
        self.grain_clock = 1.0;
        self.group_grains.reset();
    }
    pub fn set_playing(&mut self, playing: bool) {
        self.playing = playing;
        if let Some(drums) = &mut self.drums {
            if playing && self.sequence_running {
                drums.engine.play();
            } else {
                drums.engine.stop();
            }
        }
        if let Some(runtime) = &mut self.preset.sequencer.runtime {
            runtime.start(self.preset.sequencer.enabled && playing && self.sequence_running);
        }
        if playing {
            if self.performance.mode() != Mode::Keys {
                self.release_notes();
                self.sustain.fill(false);
            }
            self.performance.start();
            if self.performance.mode() != Mode::Keys {
                self.midi_gate = true;
            }
            if self.performance.mode() == Mode::Hold {
                let (keys, len) = self.performance.hold_start();
                for key in &keys[..len] {
                    self.apply_midi(Message::NoteOn {
                        channel: key.channel,
                        note: key.note,
                        velocity: key.velocity,
                    });
                }
                self.sync_stored_chord();
            } else if self.performance.mode() == Mode::Keys
                && !self.midi_gate
                && !self.sequence_has_notes()
            {
                self.voices[0].velocity = 1.0;
                self.voices[0].note = 60;
                self.voices[0].held = true;
                self.clear_voice_filters(0);
                self.start_voice(0);
                self.mod_note_on(0, 60, 127);
                self.configure_voice_filter(0);
            }
            self.grain_clock = 1.0;
        } else {
            self.performance.stop();
            self.release_notes();
        }
    }
    pub fn set_frozen(&mut self, frozen: bool) {
        if frozen && !self.frozen {
            self.frozen_center = self.center();
        }
        self.frozen = frozen;
    }
    pub fn set_midi_gate(&mut self, enabled: bool) {
        self.midi_gate = enabled;
        self.sustain.fill(false);
        for voice in self.voices.iter_mut() {
            voice.held = false;
            voice.sustained = false;
            voice.off();
        }
        if !enabled
            && self.playing
            && self.performance.mode() == Mode::Keys
            && !self.sequence_has_notes()
        {
            self.voices[0].note = 60;
            self.voices[0].velocity = 1.0;
            self.voices[0].held = true;
            self.clear_voice_filters(0);
            self.start_voice(0);
            self.mod_note_on(0, 60, 127);
            self.configure_voice_filter(0);
        }
    }
    pub fn center(&self) -> f64 {
        if self.frozen {
            self.frozen_center
        } else if let Some(input) = &self.live_input {
            let range = self.source_range();
            let length = range.len() as f64;
            let p = &self.modulated.params;
            // Leave enough history for a grain to advance without overtaking
            // the capture head. Position scans backwards through the history.
            let margin = (self.grain_seconds(p.grain_size) as f64 * self.sample_rate as f64 * 2.0
                + p.cloud as f64 * length * 0.5
                + 4.0)
                .min(length * 0.9);
            let head = if self.preset.history().is_some() {
                range.end as f64
            } else {
                input.head() as f64
            };
            range.start as f64
                + (head - range.start as f64 - margin - p.position as f64 * (length - margin))
                    .rem_euclid(length)
        } else if self.bar_loop_active()
            && let Some(voice) = self.voices.iter().enumerate().position(|(i, v)| {
                (i != 0 || !self.midi_gate)
                    && v.active()
                    && v.note == self.source_note
                    && v.channel == self.source_channel
            })
        {
            self.voice_center(voice)
        } else {
            let range = &self.file_range;
            range.start as f64
                + (self.cursor - range.start as f64
                    + self.modulated.params.position as f64 * range.len() as f64)
                    .rem_euclid(range.len() as f64)
        }
    }
    pub fn active_grains(&self) -> usize {
        self.live_grains.count_ones() as usize
    }
    /// Live slots in ascending order.
    fn live_grain_slots(&self) -> impl Iterator<Item = usize> + use<> {
        let mut pending = self.live_grains;
        std::iter::from_fn(move || {
            let slot = (pending != 0).then(|| pending.trailing_zeros() as usize)?;
            pending &= pending - 1;
            Some(slot)
        })
    }
    #[cfg(test)]
    fn live_grain_list(&self) -> impl Iterator<Item = &Grain> {
        self.live_grain_slots().map(|slot| &self.grains[slot])
    }
    fn kill_voice_grains(&mut self, voice: usize) {
        for slot in self.live_grain_slots() {
            if self.grains[slot].voice == voice {
                self.live_grains &= !(1 << slot);
            }
        }
    }
    pub fn held_notes(&self) -> Vec<u8> {
        self.voices[1..]
            .iter()
            .filter(|v| v.held || v.sustained)
            .map(|v| v.note)
            .collect()
    }
    pub fn grain_views(&self) -> Vec<GrainView> {
        self.live_grain_slots()
            .map(|slot| self.grain_view(slot))
            .collect()
    }
    fn grain_view(&self, slot: usize) -> GrainView {
        let g = &self.grains[slot];
        GrainView {
            position: self.source_position(g.position) as f32,
            age: g.age as f32 / g.duration as f32,
            pan: (g.pan[1] - g.pan[0] + 1.0) * 0.5,
            group: self.group_grains.group(slot).map_or(0, |group| group + 1),
        }
    }

    fn release_notes(&mut self) {
        for voice in self.voices.iter_mut() {
            voice.held = false;
            voice.sustained = false;
            voice.off();
        }
        if let Some(sampler) = &mut self.sampler {
            sampler.release_all();
        }
    }
    fn sync_stored_chord(&mut self) {
        self.preset.performance.chord.clear();
        self.preset
            .performance
            .chord
            .extend_from_slice(self.performance.stored_chord());
    }
    pub fn performance_view(&self) -> crate::performance::View {
        self.performance.view()
    }
    pub fn realtime_performance_view(&self) -> crate::performance::RealtimeView {
        self.performance.realtime_view()
    }
    pub fn clear_hold(&mut self) {
        self.performance.clear();
        self.sustain.fill(false);
        self.release_notes();
        self.sync_stored_chord();
    }

    pub fn set_step_entry(&mut self, step: Option<usize>, reset: bool) {
        if reset {
            self.step_edits.fill(None);
        }
        self.step_entry = step.filter(|&i| {
            i < crate::performance::STEPS
                && matches!(
                    self.performance.mode(),
                    Mode::Sequencer | Mode::SequencerArp
                )
        });
    }

    pub fn clear_step_notes(&mut self) {
        if let Some(index) = self.step_entry {
            self.preset.performance.steps[index].notes = Default::default();
            self.step_edits[index] = Some(self.preset.performance.steps[index]);
            self.performance
                .configure_prepared(&self.preset.performance);
        }
    }

    fn capture_step_note(&mut self, message: Message) -> bool {
        let (channel, note, velocity) = match message {
            Message::NoteOn {
                channel,
                note,
                velocity,
            } if channel < 16 && note < 128 && velocity < 128 => (channel, note, velocity),
            Message::NoteOff { channel, note, .. } if channel < 16 && note < 128 => {
                (channel, note, 0)
            }
            _ => return false,
        };
        let mask = 1_u128 << note;
        let captured = &mut self.captured_keys[channel as usize];
        if velocity == 0 {
            let consumed = *captured & mask != 0;
            *captured &= !mask;
            return consumed;
        }
        if *captured & mask != 0 {
            return true;
        }
        let Some(index) = self.step_entry else {
            return false;
        };
        let step = &mut self.preset.performance.steps[index];
        step.notes.toggle(note);
        step.velocity = velocity;
        step.enabled = true;
        self.step_edits[index] = Some(*step);
        self.performance
            .configure_prepared(&self.preset.performance);
        *captured |= mask;
        true
    }

    pub fn midi(&mut self, message: Message) {
        if (self.preset.synth.is_some() || self.preset.sampler.is_some())
            && matches!(message, Message::Program { .. })
        {
            // Synth patches are recalled by the host, as scenes and drums are.
            return;
        }
        if self.preset.drums.is_some() {
            // Scene and drum patches are recalled by the host, not the legacy sample-preset bank.
            if matches!(message, Message::Program { .. }) {
                return;
            }
            if let Message::NoteOn {
                note,
                velocity: 1..=127,
                ..
            } = message
            {
                if let Message::NoteOn { velocity, .. } = message {
                    let track = self
                        .preset
                        .drums
                        .as_ref()
                        .unwrap()
                        .tracks
                        .iter()
                        .position(|t| t.midi_note == note);
                    if let Some(track) = track {
                        self.trigger_drum(track, velocity as f32 / 127.);
                    }
                }
                return;
            }
            if matches!(
                message,
                Message::Reset
                    | Message::Control {
                        controller: 120 | 123,
                        ..
                    }
            ) && let Some(drums) = &mut self.drums
            {
                drums.engine.panic();
            }
        }
        if self.capture_step_note(message) {
            return;
        }
        match message {
            Message::Control {
                channel,
                controller: 120 | 123,
                ..
            } if channel < 16 => self.captured_keys[channel as usize] = 0,
            Message::Reset => {
                self.captured_keys.fill(0);
                self.step_entry = None;
            }
            _ => {}
        }
        if self.performance.mode() == Mode::Hold
            && !self.playing
            && matches!(
                message,
                Message::NoteOn {
                    channel: 0..=15,
                    note: 0..=127,
                    velocity: 1..=127
                }
            )
        {
            self.release_notes();
            self.sustain.fill(false);
            self.midi_gate = true;
            self.playing = true;
            self.performance.clear();
            self.performance.start();
        }
        if let Some(actions) = self.performance.input(message) {
            if matches!(
                message,
                Message::NoteOn {
                    velocity: 1..=127,
                    ..
                }
            ) && !self.playing
            {
                self.set_playing(true);
            }
            for action in actions.into_iter().flatten() {
                self.apply_midi(action);
            }
            self.sync_stored_chord();
            return;
        }
        if let Message::Control {
            channel,
            controller: 120 | 123,
            ..
        } = message
        {
            self.performance.clear_channel(channel);
            self.sync_stored_chord();
        }
        if message == Message::Reset {
            self.clear_hold();
        }
        self.apply_midi(message);
    }

    fn apply_midi(&mut self, message: Message) {
        self.apply_owned_midi(message, 0);
    }
    fn apply_owned_midi(&mut self, message: Message, owner: u32) {
        if matches!(message, Message::Control { .. } | Message::Program { .. }) {
            self.invalidate_racks();
        }
        if self.sampler.is_some() && self.sampler_midi(message, owner) {
            return;
        }
        match message {
            Message::NoteOn {
                channel,
                note,
                velocity,
            } => {
                if channel > 15 || note > 127 || velocity > 127 {
                    return;
                }
                if velocity == 0 {
                    self.apply_owned_midi(
                        Message::NoteOff {
                            channel,
                            note,
                            velocity: 0,
                        },
                        owner,
                    );
                    return;
                }
                if !self.midi_gate {
                    if owner == 0 {
                        self.set_midi_gate(true);
                    } else {
                        self.voices[0].held = false;
                        self.voices[0].off();
                    }
                }
                self.playing = true;
                let index = (1..VOICES)
                    .find(|&i| {
                        self.voices[i].held
                            && self.voices[i].owner == owner
                            && self.voices[i].note == note
                            && self.voices[i].channel == channel
                    })
                    .or_else(|| (1..VOICES).find(|&i| !self.voices[i].active()))
                    // Steal the quietest voice, released ones first.
                    .unwrap_or_else(|| {
                        (1..VOICES)
                            .min_by(|&a, &b| {
                                let (a, b) = (&self.voices[a], &self.voices[b]);
                                (a.held || a.sustained)
                                    .cmp(&(b.held || b.sustained))
                                    .then(a.value.total_cmp(&b.value))
                            })
                            .unwrap()
                    });
                self.next_voice = index;
                // A sounding voice keeps its grains and filter state: its
                // envelope attacks again from the level it has reached.
                if !self.voices[index].active() {
                    self.kill_voice_grains(index);
                    self.clear_voice_filters(index);
                }
                let v = &mut self.voices[index];
                v.note = note;
                v.owner = owner;
                v.channel = channel;
                v.velocity = velocity as f32 / 127.0;
                v.held = true;
                v.sustained = false;
                self.start_voice(index);
                self.mod_note_on(channel, note, velocity);
                self.configure_voice_filter(index);
                self.grain_clock = 1.0;
            }
            Message::NoteOff { channel, note, .. } => {
                if channel > 15 {
                    return;
                }
                for v in &mut self.voices[1..] {
                    if v.note == note && v.channel == channel && v.owner == owner {
                        v.held = false;
                        v.sustained = owner == 0 && self.sustain[channel as usize];
                        if !v.sustained {
                            v.off();
                        }
                    }
                }
            }
            Message::PitchBend { channel, value } if channel < 16 => {
                self.bend[channel as usize] = (value.min(16383) as f32 - 8192.0) / 8192.0
            }
            Message::Pressure { channel, value } if channel < 16 => {
                self.pressure[channel as usize] = value.min(127) as f32 / 127.0;
            }
            Message::PolyPressure {
                channel,
                note,
                value,
            } if channel < 16 && note < 128 => {
                self.poly_pressure[channel as usize][note as usize] = value.min(127) as f32 / 127.0;
            }
            Message::Control {
                channel,
                controller: 121,
                ..
            } if channel < 16 => {
                self.wheel[channel as usize] = 0.0;
                self.expression[channel as usize] = 1.0;
                self.pressure[channel as usize] = 0.0;
                self.poly_pressure[channel as usize].fill(0.0);
                self.bend[channel as usize] = 0.0;
                self.apply_midi(Message::Control {
                    channel,
                    controller: 64,
                    value: 0,
                });
            }
            Message::Control {
                channel,
                controller: 64,
                value,
            } if channel < 16 => {
                self.sustain[channel as usize] = value >= 64;
                if value < 64 {
                    for v in &mut self.voices[1..] {
                        if v.channel == channel && !v.held {
                            v.sustained = false;
                            v.off();
                        }
                    }
                }
            }
            Message::Control {
                channel,
                controller: 120,
                ..
            } if channel < 16 => {
                for i in 0..VOICES {
                    let v = &mut self.voices[i];
                    if v.channel == channel {
                        v.kill();
                        self.filters[i].clear();
                        for bank in &mut self.filter_banks {
                            bank.clear_voice(i);
                        }
                        v.held = false;
                        v.sustained = false;
                        self.kill_voice_grains(i);
                    }
                }
                self.color_rack.clear();
                self.pre_color_rack.clear();
                self.source_color.clear();
                self.invalidate_racks();
            }
            Message::Control {
                channel,
                controller: 123,
                ..
            } if channel < 16 => {
                for v in self.voices.iter_mut() {
                    if v.channel == channel {
                        v.held = false;
                        v.sustained = false;
                        v.off();
                    }
                }
            }
            Message::Control {
                channel,
                controller,
                value,
            } => {
                let unit = value.min(127) as f32 / 127.0;
                if channel < 16 {
                    if controller == 1 {
                        self.wheel[channel as usize] = unit;
                    }
                    if controller == 11 {
                        self.expression[channel as usize] = unit;
                    }
                }
                match controller {
                    1 => {
                        if !self.preset.matrix.iter().any(|r| {
                            r.enabled
                                && r.depth != 0.0
                                && r.source == Source::ModWheel
                                && r.destination != Destination::None
                        }) {
                            self.preset.params.cloud = unit;
                        }
                    }
                    39 => {
                        self.preset.params.saturation = unit;
                        if let Some(slot) = self.preset.color_chain.first_mut() {
                            slot.mix = unit;
                        }
                        crate::color::mirror_reverb(
                            &mut self.preset.params,
                            &self.preset.color_chain,
                        );
                    }
                    58 => self.set_playing(value >= 64),
                    59 => self.bypass = value >= 64,
                    70 => self.preset.params.filter = unit,
                    7 => self.preset.params.master_volume = unit,
                    _ => {}
                }
            }
            Message::Program { value, .. } => self.set_preset(Preset::builtin(value as usize % 5)),
            Message::Start | Message::Continue => self.set_playing(true),
            Message::Stop => self.set_playing(false),
            Message::Reset => {
                self.set_playing(false);
                self.reset_grains();
                self.sustain.fill(false);
                self.bend.fill(0.0);
                self.wheel.fill(0.0);
                self.expression.fill(1.0);
                self.pressure.fill(0.0);
                self.poly_pressure.fill([0.0; 128]);
                self.source_velocity = 0.0;
                self.source_random = 0.0;
                self.source_note = 60;
                self.source_channel = 0;
                self.mod_random = Random(0x4d41545249585f31);
                self.color_rack.clear();
                self.pre_color_rack.clear();
                self.source_color.clear();
                self.invalidate_racks();
                self.clear_filters();
            }
            _ => {}
        }
    }

    fn mod_note_on(&mut self, channel: u8, note: u8, velocity: u8) {
        self.source_channel = channel;
        self.source_note = note;
        self.source_velocity = velocity as f32 / 127.0;
        self.source_random = self.mod_random.bipolar();
        self.poly_pressure[channel as usize][note as usize] = 0.0;
    }
    pub fn modulation_source(&self, source: Source) -> f32 {
        let channel = self.source_channel as usize;
        match source {
            Source::Lfo1 => self.lfo_values[0],
            Source::Lfo2 => self.lfo_values[1],
            Source::Lfo3 => self.lfo_values[2],
            Source::Lfo4 => self.lfo_values[3],
            Source::Lfo5 => self.lfo_values[4],
            Source::Lfo6 => self.lfo_values[5],
            Source::Lfo7 => self.lfo_values[6],
            Source::Lfo8 => self.lfo_values[7],
            Source::Env1 => self.env_values[0],
            Source::Env2 => self.env_values[1],
            Source::Env3 => self.env_values[2],
            Source::Env4 => self.env_values[3],
            Source::Env5 => self.env_values[4],
            Source::Env6 => self.env_values[5],
            Source::Env7 => self.env_values[6],
            Source::Env8 => self.env_values[7],
            Source::Velocity => self.source_velocity,
            Source::KeyTrack => ((self.source_note as f32 - 60.0) / 48.0).clamp(-1.0, 1.0),
            Source::ModWheel => self.wheel[channel],
            Source::PitchBend => self.bend[channel],
            Source::Aftertouch => {
                self.pressure[channel].max(self.poly_pressure[channel][self.source_note as usize])
            }
            Source::Expression => self.expression[channel],
            Source::RandomNote => self.source_random,
            Source::Gate => {
                if self
                    .voices
                    .iter()
                    .any(|v| (v.held && !v.gated) || v.sustained)
                {
                    1.0
                } else {
                    0.0
                }
            }
            Source::Follower => self.follower.min(1.0),
        }
    }
    fn update_modulation(&mut self) {
        let mut m = self.spare_modulated.take().expect("spare modulation frame");
        m.write_from_preset(&self.preset);
        if self.preset.sequencer.enabled
            && self.sequence_running
            && let Some(runtime) = &self.preset.sequencer.runtime
        {
            runtime.apply(&mut m);
        }
        if m.params.loop_one_bar != self.modulated.params.loop_one_bar {
            self.voices.iter_mut().for_each(|v| v.loop_phase = 0.0);
            self.cursor = self.file_range.start as f64;
            self.reset_grains();
        }
        self.matrix_values.fill(0.0);
        let mut targets = [Destination::None; matrix::MAX_ROUTES];
        let mut amounts = [0.0; matrix::MAX_ROUTES];
        let mut target_count = 0;
        for (i, route) in self.preset.matrix.iter().enumerate() {
            if route.enabled {
                let value = self.modulation_source(route.source) * route.depth;
                self.matrix_values[i] = value;
                let target = targets[..target_count]
                    .iter()
                    .position(|d| *d == route.destination)
                    .unwrap_or_else(|| {
                        targets[target_count] = route.destination;
                        target_count += 1;
                        target_count - 1
                    });
                amounts[target] += value;
            }
        }
        for i in 0..target_count {
            m.route(targets[i], amounts[i]);
        }
        // Macros act with the matrix, so they can set LFO and ENV depths too.
        for knob in &self.preset.macros {
            for target in &knob.targets {
                m.route(target.destination, knob.value * target.depth);
            }
        }
        // Each route reads its depth as it applies, after every LFO or ENV
        // that routes into that depth, so one can open another's routes.
        for modulator in m.modulator_order() {
            let source = match modulator {
                Modulator::Lfo(i) => self.lfo_values[i] * 0.5,
                Modulator::Env(i) => self.env_values[i],
            };
            for r in 0..crate::preset::MODULATOR_ROUTES {
                let (destination, depth) = m.modulator_routes(modulator)[r];
                m.route(destination, source * depth);
            }
        }
        if self.frozen && (m.params.position - self.last_position).abs() > 0.00001 {
            let range = self.source_range();
            let length = range.len() as f64;
            self.frozen_center = range.start as f64
                + (self.frozen_center - range.start as f64
                    + (m.params.position - self.last_position) as f64 * length)
                    .rem_euclid(length);
        }
        self.last_position = m.params.position;
        crate::color::mirror_reverb(&mut m.params, &m.color_slots[..m.color_len]);
        // Racks reconfigure only when their inputs can differ from the previous
        // call: a slot modulated now or last time, a preset edit, a tempo
        // change, or a shaper pitch they received since. Otherwise every
        // processor would compare its whole slot and return unchanged.
        let rack_key = (
            self.color_revision,
            m.params.bpm.to_bits(),
            self.applied_shaper_pitch,
        );
        if m.color_dirty || self.rack_configured_dirty || self.rack_key != Some(rack_key) {
            self.color_rack
                .configure_tempo(&m.color_slots[..m.color_len], m.params.bpm);
            self.pre_color_rack
                .configure_tempo(&m.pre_color_slots[..m.pre_color_len], m.params.bpm);
            self.rack_key = Some(rack_key);
            self.rack_configured_dirty = m.color_dirty;
        }
        let source_key = (
            self.color_revision,
            m.params.bpm.to_bits(),
            m.params.sample_root_note.to_bits(),
        );
        if self.source_key != Some(source_key) {
            self.source_color.set_pitch(m.params.sample_root_note);
            self.source_color.configure_tempo(
                self.preset
                    .source_color_chain
                    .first()
                    .copied()
                    .unwrap_or(ColorSlot {
                        enabled: false,
                        ..ColorSlot::default()
                    }),
                m.params.bpm,
            );
            self.source_key = Some(source_key);
        }
        self.resynth
            .configure(m.resynth, &self.sample, m.params.sample_root_note);
        // Outside advanced mode the grain groups keep their taps closed.
        let group_taps = if m.grain_groups.advanced {
            m.grain_groups.groups.map(|g| g.output)
        } else {
            Default::default()
        };
        self.taps.configure(&m.taps, group_taps);
        let shaper_note = self.source_note as f32
            + self.bend[self.source_channel as usize] * m.params.midi_pitchbend_range
            + (m.params.pitch * 2.0 - 1.0) * m.params.pitch_range
            + m.params.sample_tune_semis
            + m.params.sample_tune_cents * 0.01;
        self.pre_color_rack.set_pitch(shaper_note);
        self.color_rack.set_pitch(shaper_note);
        self.applied_shaper_pitch = shaper_note.to_bits();
        self.performance.modulate(m.performance);
        // Pitch is a Grain engine control: it leaves the drum and synth voices
        // alone, so switching Granular off restores their own pitch. Sample
        // tune is the voices' own: the history the grains read carries it, so
        // neither Granular nor its Chain moves the tuning.
        let tune = m.params.sample_tune_semis + m.params.sample_tune_cents * 0.01;
        if let (Some(drums), Some(settings)) = (&mut self.drums, &self.preset.drums) {
            drums.configure(settings, m.drums, m.params.bpm, tune);
            drums.shift(settings, m.drum_shifts);
        }
        if let Some(synth) = &mut self.synth {
            synth.configure(&m.synth, tune, &self.sample, m.params.sample_root_note);
        }
        for (i, env) in m.envs.iter().enumerate() {
            if i >= m.env_count {
                if i < self.modulated.env_count {
                    for voice in self.voices.iter_mut() {
                        voice.envelopes[i].kill();
                        voice.env_values[i] = 0.0;
                    }
                }
                continue;
            }
            if *env != self.modulated.envs[i] {
                for voice in self.voices.iter_mut() {
                    voice.envelopes[i].configure(self.sample_rate as f32, env);
                }
            }
        }
        let slots = &m.filter_slots[..m.filter_len];
        self.filter_masks = filter_slots::masks(slots, &self.filter_banks);
        let last_note = self.source_note as f32
            + self.bend[self.source_channel as usize] * m.params.midi_pitchbend_range;
        filter_slots::configure_shared(slots, &mut self.filter_banks, last_note, self.env_values);
        let bus = self.preset.bus.is_some();
        for (i, voice) in self.voices.iter().enumerate() {
            if !voice.envelopes[0].active() && !(bus && i == 0) {
                continue;
            }
            let note = voice.note as f32
                + self.bend[voice.channel as usize] * m.params.midi_pitchbend_range;
            self.filters[i].configure(&m.params, note, voice.env_values);
            filter_slots::configure_voice(slots, &mut self.filter_banks, i, note, voice.env_values);
        }
        let params = m.params;
        self.spare_modulated = Some(std::mem::replace(&mut self.modulated, m));
        // Keep the control clock and exact powf expression; unchanged dB values
        // can reuse their result without changing modulation or smoothing.
        let gain_key = [params.in_gain.to_bits(), params.out_gain.to_bits()];
        if self.gain_key.map(|key| key[0]) != Some(gain_key[0]) {
            self.input_gain = 10.0_f32.powf(params.in_gain * 0.05);
        }
        if self.gain_key.map(|key| key[1]) != Some(gain_key[1]) {
            self.output_gain = 10.0_f32.powf(params.out_gain * 0.05);
        }
        self.gain_key = Some(gain_key);
        self.grain_density =
            params.min_grains + (MAX_GRAINS as f32 - params.min_grains) * params.grains.powi(3);
        self.grain_step = self.grain_density as f64 / self.sample_rate as f64;
        self.bar_phase_step = params.bpm as f64 / (60.0 * self.sample_rate as f64 * self.bar_beats);
        let count = self.active_grains() as f32;
        self.overlap = fast::flush(
            self.overlap + (count - self.overlap) * if count > self.overlap { 0.9 } else { 0.2 },
        );
    }
    /// Buses (bit n for Bus n) that this timbre's sections may send to. A
    /// host without buses leaves this empty and nothing is sent.
    pub fn set_open_buses(&mut self, open: u16) {
        self.taps.set_open_buses(open);
    }
    /// What enters this Bus or Master chain in the next frame after its
    /// input: at Pre-filter FX, Filters and Post-filter FX, each at the
    /// chain's input gain. Other timbres ignore it.
    pub fn enter(&mut self, entered: crate::timbres::Entered) {
        self.entered = Some(entered);
    }
    /// What the sections sent into each bus in the last frame, at the
    /// timbre's volume; None when nothing was sent.
    pub fn bus_sends(&self) -> Option<&[Stereo; crate::bus::MAX_BUSES]> {
        self.taps.output()
    }
    /// The buses (bit b for Bus b + 1) that the sections may send to in the
    /// coming frames: those they are routed to and those still fading out.
    pub fn tap_buses(&self) -> u8 {
        self.taps.buses()
    }
    pub fn resynth_meter(&self) -> crate::resynth::Meter {
        crate::resynth::Meter {
            bands: self.resynth.levels(),
            frequencies: self.resynth.frequencies(),
        }
    }
    pub fn modulated_params(&self) -> Params {
        self.modulated.params
    }
    pub fn modulated_color_slots(&self) -> &[ColorSlot] {
        &self.modulated.color_slots[..self.modulated.color_len]
    }
    pub fn modulated_pre_color_slots(&self) -> &[ColorSlot] {
        &self.modulated.pre_color_slots[..self.modulated.pre_color_len]
    }
    pub fn modulated_envs(&self) -> [Env; crate::preset::MODULATORS] {
        self.modulated.envs
    }
    pub fn modulated_lfos(&self) -> [Lfo; crate::preset::MODULATORS] {
        self.modulated.lfos
    }

    fn configure_voice_filter(&mut self, index: usize) {
        let voice = &self.voices[index];
        let note = voice.note as f32
            + self.bend[voice.channel as usize] * self.modulated.params.midi_pitchbend_range;
        self.filters[index].configure(&self.modulated.params, note, voice.env_values);
        filter_slots::configure_voice(
            &self.modulated.filter_slots[..self.modulated.filter_len],
            &mut self.filter_banks,
            index,
            note,
            voice.env_values,
        );
    }
    fn clear_voice_filters(&mut self, voice: usize) {
        self.filters[voice].clear();
        for bank in &mut self.filter_banks {
            bank.clear_voice(voice);
        }
    }
    fn clear_filters(&mut self) {
        self.filters.iter_mut().for_each(FilterBank::clear);
        for bank in &mut self.filter_banks {
            bank.clear();
        }
    }

    pub fn voice_filter_cutoffs(&self, note: u8) -> Option<[f32; 2]> {
        self.voices
            .iter()
            .find(|voice| voice.active() && voice.note == note)
            .map(|voice| {
                let pitch = note as f32
                    + self.bend[voice.channel as usize]
                        * self.modulated.params.midi_pitchbend_range;
                filter::voice_cutoffs(&self.modulated.params, pitch, voice.env_values)
            })
    }

    fn spawn(&mut self) {
        // Lowest free slot, as the previous linear scan chose.
        let slot = (!self.live_grains).trailing_zeros() as usize;
        if slot == MAX_GRAINS {
            return;
        }
        let synth = self.synth.is_some();
        let voice = if synth {
            // Synth grains read the mixed history after every voice's VCA:
            // they belong to no note and keep its tails while playing.
            if !self.playing {
                return;
            }
            0
        } else {
            let bus = self.preset.bus.is_some();
            let Some(voice) = (0..VOICES)
                .map(|i| (i + self.next_voice) % VOICES)
                .find(|&i| self.voices[i].envelopes[0].active() || (bus && i == 0))
            else {
                return;
            };
            self.next_voice = (voice + 1) % VOICES;
            voice
        };
        let groups = &self.modulated.grain_groups;
        let group = if groups.advanced {
            let Some(index) = self.group_grains.deal(groups) else {
                return;
            };
            Some((index, groups.groups[index]))
        } else {
            None
        };
        let p = &self.modulated.params;
        let note = self.voices[voice];
        let bend = if synth {
            0.0
        } else {
            self.bend[note.channel as usize] * p.midi_pitchbend_range
        };
        // A Synth's or drum kit's history is already tuned.
        let (tune_semis, tune_cents) = if synth || self.drums.is_some() {
            (0.0, 0.0)
        } else {
            (p.sample_tune_semis, p.sample_tune_cents)
        };
        let semis = (p.pitch * 2.0 - 1.0) * p.pitch_range
            + if voice == 0 {
                0.0
            } else {
                note.note as f32 - p.sample_root_note
            }
            + tune_semis
            + tune_cents * 0.01
            + bend
            + group.map_or(0.0, |(_, g)| g.pitch)
            + self.random.unit() * tables::detune_range(p.detune);
        let range = self.source_range();
        let length = range.len() as f64;
        let rate = tables::pitch_ratio(semis) * self.source_sample().sample_rate as f64
            / self.sample_rate as f64
            * if p.grain_size < 0.5 { -1.0 } else { 1.0 };
        let duration = (self.grain_seconds(p.grain_size) * self.sample_rate as f32)
            .round()
            .max(4.0) as u32;
        let position = range.start as f64
            + (self.voice_center(voice) - range.start as f64
                + self.random.bipolar() as f64 * p.cloud as f64 * length * 0.5)
                .rem_euclid(length);
        let pan =
            (self.modulated.pan + self.random.bipolar() * p.stereo_spread * 0.5).clamp(0.0, 1.0);
        let shape = group.map_or(p.grain_shape, |(_, g)| g.shape);
        match group {
            Some((index, g)) => self.group_grains.start(slot, index, &g, duration),
            None => self.group_grains.release(slot),
        }
        self.live_grains |= 1 << slot;
        self.grains[slot] = Grain {
            position,
            rate,
            read_level: self.source_sample().level_for_rate(rate, true),
            age: 0,
            duration,
            shape: shape.clamp(0.0, 1.0),
            pan: tables::pan(pan),
            voice,
        };
    }

    /// Sequencer steps and arpeggiator notes for this frame.
    fn tick_events(&mut self) {
        let tick = self.preset.sequencer.runtime.as_mut().is_some_and(|r| {
            r.tick(
                self.sample_rate,
                self.preset.params.bpm,
                self.preset.sequencer.enabled && self.sequence_running,
            )
        });
        if tick {
            let count = self
                .preset
                .sequencer
                .runtime
                .as_ref()
                .map_or(0, |r| r.len());
            for index in 0..count {
                let (id, actions) = self
                    .preset
                    .sequencer
                    .runtime
                    .as_mut()
                    .unwrap()
                    .actions(index);
                for action in actions.into_iter().flatten() {
                    match action {
                        sequence::Action::Midi(message) => self.apply_owned_midi(message, id),
                        sequence::Action::Legato {
                            channel,
                            note,
                            velocity,
                        } => {
                            // A kit has no voice to bend: the slide replays the pad.
                            if let (Some(sampler), Some(settings)) =
                                (&mut self.sampler, &self.preset.sampler)
                            {
                                sampler.release_owner(settings, id);
                            }
                            if let Some(voice) = self
                                .voices
                                .iter_mut()
                                .find(|v| v.owner == id && v.held && self.sampler.is_none())
                            {
                                voice.note = note;
                                voice.channel = channel;
                                voice.velocity = velocity as f32 / 127.0;
                                self.source_note = note;
                                self.source_channel = channel;
                            } else {
                                self.apply_owned_midi(
                                    Message::NoteOn {
                                        channel,
                                        note,
                                        velocity,
                                    },
                                    id,
                                );
                            }
                        }
                    }
                }
            }
            // Main pitch steps can gate the free voice's ENV like notes.
            if let Some((trigger, open)) = self
                .preset
                .sequencer
                .runtime
                .as_ref()
                .and_then(|r| r.env_gate())
                && self.voices[0].held
            {
                if trigger {
                    self.start_free_voice();
                    self.grain_clock = 1.0;
                } else if !open && !self.voices[0].gated {
                    self.voices[0].off();
                    self.voices[0].gated = true;
                }
            }
            self.update_modulation();
        }
        if self.performance.ticking() {
            for action in self
                .performance
                .tick(self.sample_rate, self.modulated.params.bpm)
                .into_iter()
                .flatten()
            {
                self.apply_midi(action);
            }
        }
    }

    pub fn process_frame(&mut self) -> Stereo {
        self.process_input_frame([0.0; 2])
    }

    pub fn apply_drum_edits(&mut self, edits: &[crate::drums::Edit]) {
        if let Some(settings) = &mut self.preset.drums {
            settings.edit(edits);
            if let Some(runtime) = &mut self.drums {
                runtime.sync_tracks(settings);
            }
        }
    }
    pub fn trigger_drum(&mut self, track: usize, velocity: f32) {
        if let Some(drums) = &mut self.drums {
            drums.engine.trigger(track, velocity.clamp(0., 1.), 0);
        }
        if !self.playing {
            self.midi_gate = false;
            // Audition does not start the drum pattern.
            let gate = self.drums.as_ref().is_some_and(|d| d.engine.playing);
            self.set_playing(true);
            if !gate && let Some(drums) = &mut self.drums {
                drums.engine.stop();
            }
        }
    }
    fn render_sampler(&mut self) -> Stereo {
        let (Some(sampler), Some(settings)) = (&mut self.sampler, &self.preset.sampler) else {
            return [0.0; 2];
        };
        let range = self.preset.params.midi_pitchbend_range;
        let bend: [f32; 16] = std::array::from_fn(|c| self.bend[c] * range);
        sampler.render(settings, &bend)
    }
    fn sync_sampler(&mut self) {
        match &self.preset.sampler {
            Some(settings) => {
                let rate = self.sample_rate;
                self.sampler
                    .get_or_insert_with(|| Box::new(sampler::Runtime::new(rate)))
                    .configure(settings);
            }
            None => self.sampler = None,
        }
    }
    /// Notes, the sustain pedal and panics of a Sampler kit. Returns whether
    /// the message is used up; pitch bend and controllers continue.
    fn sampler_midi(&mut self, message: Message, owner: u32) -> bool {
        let (Some(sampler), Some(settings)) = (&mut self.sampler, &self.preset.sampler) else {
            return false;
        };
        match message {
            Message::NoteOn {
                channel,
                note,
                velocity,
            } if channel < 16 && note < 128 && velocity < 128 => {
                let sustain = owner == 0 && self.sustain[channel as usize];
                if velocity == 0 {
                    sampler.note_off(settings, channel, note, owner, sustain);
                    return true;
                }
                let pad = self
                    .preset
                    .sequencer
                    .tracks
                    .iter()
                    .find(|t| owner != 0 && t.id == owner)
                    .and_then(|t| match t.target {
                        crate::sequence::Target::Pad { pad } => Some(pad),
                        _ => None,
                    });
                match pad {
                    // A pad's own track plays it whatever its keys.
                    Some(pad) => {
                        if let Some(index) = sampler.pad_index(pad) {
                            sampler.trigger(settings, index, channel, note, velocity, owner);
                        }
                    }
                    None => sampler.note_on(settings, channel, note, velocity, owner),
                }
                self.mod_note_on(channel, note, velocity);
                true
            }
            Message::NoteOff { channel, note, .. } if channel < 16 => {
                let sustain = owner == 0 && self.sustain[channel as usize];
                sampler.note_off(settings, channel, note, owner, sustain);
                true
            }
            Message::Control {
                channel,
                controller: 64,
                value,
            } if channel < 16 => {
                self.sustain[channel as usize] = value >= 64;
                if value < 64 {
                    sampler.release_sustained(settings, channel);
                }
                true
            }
            Message::Control {
                controller: 120, ..
            }
            | Message::Reset => {
                sampler.panic();
                false
            }
            Message::Control {
                controller: 123, ..
            } => {
                sampler.release_all();
                false
            }
            _ => false,
        }
    }
    /// Replaces a kit's settings without rebuilding the preset or effects.
    pub fn set_sampler(&mut self, settings: crate::sampler::Settings) {
        if self.preset.sampler.is_some() {
            self.preset.sampler = Some(Box::new(settings));
            self.sync_sampler();
        }
    }
    /// The kit's samples by pool id; call `refresh_sampler` after a change.
    pub fn sampler_pool(&self) -> Option<&sampler::Pool> {
        self.sampler.as_ref().map(|s| &s.pool)
    }
    pub fn sampler_pool_mut(&mut self) -> Option<&mut sampler::Pool> {
        self.sampler.as_mut().map(|s| &mut s.pool)
    }
    pub fn refresh_sampler(&mut self) {
        self.sync_sampler();
    }
    /// The pads whose voices go to their Pad FX timbre rather than the mix.
    pub fn set_sampler_routes(&mut self, routes: &[u32]) {
        if let (Some(sampler), Some(settings)) = (&mut self.sampler, &self.preset.sampler) {
            sampler.set_routes(routes, settings);
        }
    }
    /// The last frame of a routed pad's voices.
    pub fn sampler_send(&self, pad: u32) -> Stereo {
        self.sampler.as_ref().map_or([0.0; 2], |s| s.send(pad))
    }
    pub fn sampler_idle(&self) -> bool {
        self.sampler.as_ref().is_none_or(|s| s.idle())
    }
    /// Plays one pad, as a click on it does: `note` defaults to its lowest key.
    pub fn trigger_pad(&mut self, pad: u32, note: Option<u8>, velocity: u8) {
        if let (Some(sampler), Some(settings)) = (&mut self.sampler, &self.preset.sampler)
            && let Some(index) = sampler.pad_index(pad)
        {
            let note = note.unwrap_or(settings.pads[index].keys[0]).min(127);
            sampler.trigger(
                settings,
                index,
                0,
                note,
                velocity.clamp(1, 127),
                AUDITION | pad,
            );
        }
    }
    pub fn release_pad(&mut self, pad: u32) {
        if let (Some(sampler), Some(settings)) = (&mut self.sampler, &self.preset.sampler) {
            sampler.release_owner(settings, AUDITION | pad);
        }
    }
    pub fn sampler_view(&self) -> crate::sampler::View {
        let Some(sampler) = &self.sampler else {
            return Default::default();
        };
        crate::sampler::View {
            voices: sampler.positions(),
            hits: sampler
                .pad_ids()
                .zip(sampler.hits.iter().copied())
                .collect(),
            loaded: sampler.pool.keys().cloned().collect(),
        }
    }
    pub fn drum_view(&self) -> crate::drums::View {
        self.drums
            .as_ref()
            .map_or_else(Default::default, |d| crate::drums::View {
                positions: d.engine.positions,
                levels: d.engine.meters,
                playing: d.engine.playing,
                shifts: d.shifts,
            })
    }

    pub fn process_input_frame(&mut self, input: Stereo) -> Stereo {
        self.taps.begin_frame();
        let input = if self.sampler.is_some() {
            // A kit plays its sequencer even while its chain passes through.
            if self.playing {
                self.tick_events();
            }
            let kit = self.render_sampler();
            [input[0] + kit[0], input[1] + kit[1]]
        } else {
            input
        };
        let bus = self.preset.bus.as_deref().copied();
        let mut entered = self.entered.take().filter(|_| bus.is_some());
        // With nothing to skip, what enters later is the same as input.
        let input = match entered {
            Some(e) if self.bypass || self.bus_passthrough() => {
                entered = None;
                std::array::from_fn(|c| input[c] + e[0][c] + e[1][c] + e[2][c])
            }
            _ => input,
        };
        if bus.is_some() && self.bus_passthrough() {
            // A bus with nothing switched on still runs its sequencer.
            if self.playing && self.sampler.is_none() {
                self.tick_events();
            }
            return self.pass_bus(input);
        }
        let input = if let Some(drums) = &mut self.drums {
            drums.frame()
        } else if let Some(synth) = &self.synth {
            synth.output()
        } else {
            input
        };
        if let (Some(history), Some(live)) = (self.preset.history(), &mut self.live_input) {
            live.effects_only = !history.granular;
            if live.set_window_seconds(history.buffer_seconds) {
                let range = live.range();
                let capacity = live.sample.len() as f64;
                let remap = |position: f64| {
                    let mut position = position.rem_euclid(capacity);
                    if position < range.start as f64 {
                        position += capacity;
                    }
                    range.start as f64
                        + (position - range.start as f64).rem_euclid(range.len() as f64)
                };
                self.frozen_center = remap(self.frozen_center);
                for slot in self.live_grain_slots() {
                    let grain = &mut self.grains[slot];
                    grain.position = remap(grain.position);
                }
            }
        }
        let effects_only = if bus.is_some() {
            // Without a history (Granular off) a bus processes its input only.
            self.live_input.as_ref().is_none_or(|i| i.effects_only)
        } else {
            self.live_input.as_ref().is_some_and(|i| i.effects_only)
        };
        let source_input = if self.live_input.is_some() || bus.is_some() {
            let clean = input.map(|x| {
                if x.is_finite() {
                    x.clamp(-16.0, 16.0)
                } else {
                    0.0
                }
            });
            let processed = self.source_color.process(clean);
            if self.taps.sends(Stage::Source) {
                let gain = self.input_gain;
                self.taps.send(Stage::Source, processed.map(|x| x * gain));
            }
            self.taps.mix(Stage::Source, clean, processed)
        } else {
            input
        };
        if let Some(live) = &mut self.live_input {
            let windowed = self.preset.history().is_some();
            let epoch = if windowed { live.range_epoch() } else { 0 };
            live.push_with_meter(source_input, input, self.frozen);
            if windowed {
                let shift = (live.range_epoch() - epoch) as f64;
                if shift != 0.0 {
                    self.frozen_center -= shift;
                    for slot in self.live_grain_slots() {
                        self.grains[slot].position -= shift;
                    }
                }
            }
        }
        if self.playing && self.sampler.is_none() {
            self.tick_events();
        }
        let sr = self.sample_rate as f32;
        let source_rate = self.source_sample().sample_rate as f64 / self.sample_rate as f64;
        let source = if self.live_input.is_some() || bus.is_some() {
            source_input.map(|x| {
                if x.is_finite() {
                    x.clamp(-16.0, 16.0)
                } else {
                    0.0
                }
            })
        } else {
            let fade = self
                .fades
                .gain(self.cursor, self.sample.sample_rate, &self.file_range);
            let level = self.sample.level_for_rate(source_rate, true);
            let processed = source_fx::read(
                &self.sample,
                &self.processed_source,
                &self.previous_source,
                self.source_fade_remaining,
                self.cursor,
                level,
                &self.file_range,
            )
            .map(|v| v * fade);
            if self.taps.sends(Stage::Source) {
                let gain = self.input_gain;
                self.taps.send(Stage::Source, processed.map(|x| x * gain));
            }
            if self.taps.chain(Stage::Source) < 1.0
                && (self.processed_source.is_some() || self.source_fade_remaining != 0)
            {
                // Source FX are rendered ahead into a copy of the file; leaving
                // them out of the chain blends back to the file itself.
                let raw = self
                    .sample
                    .read_region_level(self.cursor, level, &self.file_range)
                    .map(|v| v * fade);
                self.taps.mix(Stage::Source, raw, processed)
            } else {
                processed
            }
        };
        let level = source[0].abs().max(source[1].abs());
        self.follower = fast::flush(
            self.follower
                + (level - self.follower)
                    * if level > self.follower {
                        0.006
                    } else {
                        0.00015
                    },
        );
        self.lfo_values.fill(0.0);
        for i in 0..self.modulated.lfo_count {
            self.lfo_values[i] = self.oscillators[i].next_sample(
                &self.modulated.lfos[i],
                sr,
                self.modulated.params.bpm,
                self.follower.min(1.0),
            );
        }
        self.env_values = [0.0; crate::preset::MODULATORS];
        let mut loudest_voice = 0.0_f32;
        let mut voice_sum = 0.0_f32;
        // A voice leaves the set on the frame its envelopes end at zero, so
        // the others keep the zero levels and gain that frame left.
        for index in bits(self.sounding) {
            let v = &mut self.voices[index];
            let mut active = false;
            for (i, env) in v
                .envelopes
                .iter_mut()
                .take(self.modulated.env_count)
                .enumerate()
            {
                let value = env.next_sample();
                active |= env.active();
                v.env_values[i] = value;
                self.env_values[i] = self.env_values[i].max(value);
                if i == 0 {
                    v.value = value * v.velocity;
                    loudest_voice = loudest_voice.max(v.value);
                    voice_sum += v.value;
                }
            }
            if !active {
                self.sounding &= !(1 << index);
            }
        }
        if bus.is_some() {
            // A bus has one permanent voice at unity: no notes, no ENV1 VCA.
            for (i, v) in self.voices.iter_mut().enumerate() {
                v.value = if i == 0 { 1.0 } else { 0.0 };
            }
            loudest_voice = 1.0;
            voice_sum = 1.0;
        }
        if self.frame.is_multiple_of(CONTROL_INTERVAL) {
            self.update_modulation();
        }
        let density = self.grain_density;
        self.grain_clock += self.grain_step;
        if !effects_only && density > 0.0 && self.grain_clock >= 1.0 {
            self.grain_clock = self.grain_clock.fract();
            self.spawn();
        }
        let mut voice_frames = [[0.0; 2]; VOICES];
        let range = self.source_range();
        let file_source = self.live_input.is_none();
        let source_effects =
            file_source && (self.processed_source.is_some() || self.source_fade_remaining != 0);
        let sample = self
            .live_input
            .as_ref()
            .map_or(&*self.sample, |i| &i.sample);
        // Loop invariants, hoisted out of the per-grain path: default fades
        // return a gain of exactly 1.0, and a whole-file region reads the mip
        // level directly, as read_region_level would decide for every grain.
        let fading = file_source && self.fades != Fades::default();
        let antialiasing = self.preset.params.antialiasing;
        let whole_file = !source_effects && range.start == 0 && range.end == sample.len();
        let source_chain = self.taps.chain(Stage::Source);
        let top_level = sample.active_levels() - 1;
        let groups = &self.modulated.grain_groups;
        // Each group's grains in the chain, and what it sends to its bus.
        let group_chains = self.taps.group_chains();
        let group_sends = self.taps.group_sends();
        let mut group_sent = [[0.0; 2]; crate::grain_groups::GROUPS];
        let synth_grains = self.synth.is_some();
        let mut pending = self.live_grains;
        while pending != 0 {
            let slot = pending.trailing_zeros() as usize;
            pending &= pending - 1;
            let grain = &mut self.grains[slot];
            let level = if antialiasing { grain.read_level } else { 0 };
            let mut frame = if whole_file {
                sample.read_level(grain.position, level.min(top_level))
            } else if source_effects {
                let processed = source_fx::read(
                    &self.sample,
                    &self.processed_source,
                    &self.previous_source,
                    self.source_fade_remaining,
                    grain.position,
                    level,
                    &range,
                );
                if source_chain < 1.0 {
                    let raw = sample.read_region_level(grain.position, level, &range);
                    std::array::from_fn(|c| raw[c] + (processed[c] - raw[c]) * source_chain)
                } else {
                    processed
                }
            } else {
                sample.read_region_level(grain.position, level, &range)
            };
            if fading {
                let gain = self.fades.gain(grain.position, sample.sample_rate, &range);
                frame = frame.map(|v| v * gain);
            }
            // A group's grains take its amp envelope in place of the window.
            if groups.advanced
                && let Some((group, shaped)) =
                    self.group_grains.process(slot, grain.age, frame, groups)
            {
                let panned: Stereo = std::array::from_fn(|c| shaped[c] * grain.pan[c]);
                let chain = group_chains[group];
                for c in 0..2 {
                    voice_frames[grain.voice][c] += panned[c] * chain;
                }
                if group_sends & 1 << group != 0 {
                    // Sends follow the notes' VCA as the chain does; a
                    // Synth's grains already read its voices after theirs.
                    let weight = if synth_grains {
                        1.0
                    } else {
                        self.voices[grain.voice].value.max(0.0)
                    };
                    for c in 0..2 {
                        group_sent[group][c] += panned[c] * weight;
                    }
                }
            } else {
                let phase = 2.0 * grain.age as f32 / (grain.duration - 1) as f32;
                let amplitude = tables::grain_window(phase, grain.shape);
                for c in 0..2 {
                    voice_frames[grain.voice][c] += frame[c] * amplitude * grain.pan[c];
                }
            }
            grain.position = range.start as f64
                + sample::wrap_position(
                    grain.position + grain.rate - range.start as f64,
                    range.len() as f64,
                );
            grain.age += 1;
            if grain.age >= grain.duration {
                self.live_grains &= !(1 << slot);
            }
        }
        let norm = if !effects_only && self.overlap > 2.0 {
            tables::inverse_sqrt(self.overlap - 1.0)
        } else {
            1.0
        };
        let input_gain = self.input_gain;
        // A group's send carries its grains before Dry/Wet and the added
        // filters, at the engine's overlap level.
        for (group, sent) in group_sent.iter().enumerate() {
            if group_sends & 1 << group != 0 {
                self.taps
                    .send_group(group, sent.map(|x| x * norm * input_gain));
            }
        }
        self.smooth_mix =
            fast::flush(self.smooth_mix + (self.modulated.params.mix - self.smooth_mix) * 0.002);
        // The source is shared by all notes. Distribute it across their filter
        // banks without multiplying its level when a chord adds more voices.
        let dry_scale = if voice_sum > 0.0 {
            loudest_voice / voice_sum
        } else {
            0.0
        };
        let (dry_amount, grain_amount) = if effects_only {
            (dry_scale, 0.0)
        } else {
            (dry_scale * (1.0 - self.smooth_mix), norm * self.smooth_mix)
        };
        // Without the grain engine in the chain, voices get the source as
        // Dry/Wet would at fully dry; the bus still hears the blend.
        let grain_chain = self.taps.chain(Stage::Grains);
        let (chain_dry, chain_grain) = if grain_chain == 1.0 {
            (dry_amount, grain_amount)
        } else {
            (
                dry_scale + (dry_amount - dry_scale) * grain_chain,
                grain_amount * grain_chain,
            )
        };
        let grain_sends = self.synth.is_none() && self.taps.sends(Stage::Grains);
        let mut grain_send = [0.0; 2];
        let mut mixed_frames = [[0.0; 2]; VOICES];
        let mut before_fx = [0.0; 2];
        let mut grain_tap = [0.0; 2];
        let masks = self.filter_masks;
        // Voices outside the sounding set have no gain, except a bus's voice.
        let mixing = self.sounding | u64::from(bus.is_some());
        if let Some(synth) = &mut self.synth {
            // Oscillators replace each voice's source and grains before its
            // filter; the grains of voice slot 0 follow the VCA instead.
            synth.begin_frame(self.lfo_values[0]);
            for i in bits(mixing) {
                let voice = &self.voices[i];
                if voice.value <= 0.0 {
                    continue;
                }
                let note = voice.note as f32
                    + self.bend[voice.channel as usize]
                        * self.modulated.params.midi_pitchbend_range;
                let frame = synth.render(i, voice.generation, note, &self.sample, &self.file_range);
                // Filters placed after the voice source shape its oscillators.
                let frame = filter_slots::chain(
                    &mut self.filter_banks,
                    masks[FilterPosition::AfterGrains as usize],
                    frame,
                    Instance::Voice(i),
                );
                mixed_frames[i] = frame.map(|x| x * input_gain);
                for (c, total) in before_fx.iter_mut().enumerate() {
                    *total += mixed_frames[i][c] * voice.value;
                }
            }
            grain_tap = before_fx;
        }
        for i in bits(mixing) {
            if self.voices[i].value <= 0.0 || self.synth.is_some() {
                continue;
            }
            let frame = filter_slots::chain(
                &mut self.filter_banks,
                masks[FilterPosition::AfterGrains as usize],
                voice_frames[i],
                Instance::Voice(i),
            );
            mixed_frames[i] = std::array::from_fn(|c| {
                (source[c] * chain_dry + frame[c] * chain_grain) * input_gain
            });
            for (c, total) in before_fx.iter_mut().enumerate() {
                *total += mixed_frames[i][c] * self.voices[i].value;
                grain_tap[c] += frame[c] * norm * input_gain * self.voices[i].value;
            }
            if grain_sends {
                for (c, total) in grain_send.iter_mut().enumerate() {
                    *total += (source[c] * dry_amount + frame[c] * grain_amount)
                        * input_gain
                        * self.voices[i].value;
                }
            }
        }
        if grain_sends {
            self.taps.send(Stage::Grains, grain_send);
        }
        // One stereo pre-filter rack, with its own tails and delay clock. The
        // shared return enters each note's filter before its VCA. Normalizing
        // the send/return by voice weight preserves per-note envelopes and the
        // dry voice/filter separation; an empty or bypassed rack is transparent.
        if voice_sum > 0.0 {
            before_fx.iter_mut().for_each(|v| *v /= voice_sum);
            grain_tap.iter_mut().for_each(|v| *v /= voice_sum);
        }
        let resynth_active = self.resynth.active();
        let resynth_output = self.resynth.process_with_filters(
            before_fx,
            source.map(|x| x * input_gain),
            grain_tap,
            self.lfo_values[0],
            dry_scale,
            &self.sample,
            &self.file_range,
            self.playing,
            [
                masks[FilterPosition::AfterOsc1 as usize],
                masks[FilterPosition::AfterOsc2 as usize],
                masks[FilterPosition::AfterOsc3 as usize],
                masks[FilterPosition::AfterOsc4 as usize],
            ],
            &mut self.filter_banks,
        );
        let after_resynth = masks[FilterPosition::AfterResynth as usize];
        let resynth_output = filter_slots::chain(
            &mut self.filter_banks,
            after_resynth,
            resynth_output,
            Instance::Shared,
        );
        // The shared stages reach every voice and are weighted by each, so
        // what they send follows the notes' VCA as their return does.
        if self.taps.sends(Stage::Resynth) {
            self.taps
                .send(Stage::Resynth, resynth_output.map(|x| x * voice_sum));
        }
        let resynth_output = self.taps.mix(Stage::Resynth, before_fx, resynth_output);
        let resynth_output = match entered {
            Some(e) => std::array::from_fn(|c| resynth_output[c] + e[0][c] * input_gain),
            None => resynth_output,
        };
        let pre_output = self.pre_color_rack.process(resynth_output);
        if self.taps.sends(Stage::PreFx) {
            self.taps
                .send(Stage::PreFx, pre_output.map(|x| x * voice_sum));
        }
        let pre_output = self.taps.mix(Stage::PreFx, resynth_output, pre_output);
        let correction = if self.pre_color_rack.is_active()
            || resynth_active
            || after_resynth != 0
            || entered.is_some()
        {
            // A bus has one voice, so what enters at Filters reaches it here.
            let filters = entered.map_or([0.0; 2], |e| e[1].map(|x| x * input_gain));
            std::array::from_fn(|c| pre_output[c] - before_fx[c] + filters[c])
        } else {
            [0.0; 2]
        };
        let mut filtered = [0.0; 2];
        let filter_sends = self.taps.sends(Stage::Filters);
        let mut filter_send = [0.0; 2];
        for i in bits(mixing) {
            let mixed = mixed_frames[i];
            if self.voices[i].value <= 0.0 {
                continue;
            }
            let unfiltered = std::array::from_fn(|c| mixed[c] + correction[c]);
            let voice = filter_slots::chain(
                &mut self.filter_banks,
                masks[FilterPosition::BeforeFilters as usize],
                unfiltered,
                Instance::Voice(i),
            );
            let voice = if bus.is_some_and(|b| !b.filters) {
                voice
            } else {
                self.filters[i].process(voice)
            };
            let voice = filter_slots::chain(
                &mut self.filter_banks,
                masks[FilterPosition::AfterFilters as usize],
                voice,
                Instance::Voice(i),
            );
            if filter_sends {
                for c in 0..2 {
                    filter_send[c] += voice[c] * self.voices[i].value;
                }
            }
            let voice = self.taps.mix(Stage::Filters, unfiltered, voice);
            for c in 0..2 {
                filtered[c] += voice[c] * self.voices[i].value;
            }
        }
        if filter_sends {
            self.taps.send(Stage::Filters, filter_send);
        }
        if let Some(synth) = &mut self.synth {
            synth.set_output(filtered);
            // The history's source path, one frame later, is the dry signal.
            let dry = if self.live_input.is_some() {
                source
            } else {
                filtered
            };
            // A Synth's grain engine follows its filters.
            let grains = if effects_only {
                dry
            } else {
                std::array::from_fn(|c| {
                    dry[c] * (1.0 - self.smooth_mix) + voice_frames[0][c] * norm * self.smooth_mix
                })
            };
            if self.taps.sends(Stage::Grains) {
                self.taps.send(Stage::Grains, grains);
            }
            filtered = self.taps.mix(Stage::Grains, dry, grains);
        }
        let filtered = match entered {
            Some(e) => std::array::from_fn(|c| filtered[c] + e[2][c] * input_gain),
            None => filtered,
        };
        let processed = self
            .color_rack
            .process_with_voice_gain(filtered, loudest_voice);
        let processed = filter_slots::chain(
            &mut self.filter_banks,
            masks[FilterPosition::AfterFx as usize],
            processed,
            Instance::Shared,
        );
        if self.taps.sends(Stage::PostFx) {
            self.taps.send(Stage::PostFx, processed);
        }
        let processed = self.taps.mix(Stage::PostFx, filtered, processed);
        let volume = if self.bypass {
            self.preset.params.master_volume
        } else {
            self.modulated.params.master_volume
        };
        let gain = volume * self.output_gain;
        self.smooth_gain = fast::flush(self.smooth_gain + (gain - self.smooth_gain) * 0.002);
        self.smooth_bypass_gate = fast::flush(
            self.smooth_bypass_gate
                + ((if self.playing || bus.is_some() {
                    1.0
                } else {
                    0.0
                }) - self.smooth_bypass_gate)
                    * 0.002,
        );
        self.taps.finish(self.smooth_gain, self.bypass);
        let bypass_source = if self.bypass
            && (self.processed_source.is_some()
                || self.previous_source.is_some()
                || !self.preset.source_color_chain.is_empty())
        {
            if self.live_input.is_some() {
                input.map(|x| {
                    if x.is_finite() {
                        x.clamp(-16.0, 16.0)
                    } else {
                        0.0
                    }
                })
            } else {
                self.sample
                    .read_region(self.cursor, source_rate, true, &self.file_range)
                    .map(|v| {
                        v * self
                            .fades
                            .gain(self.cursor, self.sample.sample_rate, &self.file_range)
                    })
            }
        } else {
            source
        };
        let output = std::array::from_fn(|c| {
            let value = (if self.bypass {
                bypass_source[c] * input_gain * self.smooth_bypass_gate
            } else {
                processed[c]
            }) * self.smooth_gain;
            if !value.is_finite() {
                0.0
            } else if value.abs() > 0.98 {
                value.signum() * (0.98 + 0.02 * fast::tanh((value.abs() - 0.98) / 0.02))
            } else {
                value
            }
        });
        for (c, &value) in output.iter().enumerate() {
            self.peak[c] = self.peak[c].max(value.abs());
        }
        if self.playing {
            let bar_loop = self.bar_loop_active();
            let phase_step = self.bar_phase_step;
            if bar_loop && !self.frozen {
                for voice in self.voices.iter_mut() {
                    if voice.active() {
                        voice.loop_phase =
                            sample::wrap_position(voice.loop_phase + phase_step, 1.0);
                    }
                }
            }
            let advance = if bar_loop {
                if self.frozen {
                    0.0
                } else {
                    self.file_range.len() as f64 * phase_step
                }
            } else {
                source_rate
            };
            self.cursor = self.file_range.start as f64
                + sample::wrap_position(
                    self.cursor + advance - self.file_range.start as f64,
                    self.file_range.len() as f64,
                );
        }
        self.frame += 1;
        self.source_fade_remaining = self.source_fade_remaining.saturating_sub(1);
        output
    }

    pub fn process(&mut self, output: &mut [Stereo]) {
        for frame in output {
            *frame = self.process_frame();
        }
    }

    /// One block of input frames to the same number of output frames: the
    /// per-frame path, frame after frame, so one engine's state stays in cache
    /// for the whole block instead of alternating with other engines' states.
    pub fn process_input_block(&mut self, input: &[Stereo], output: &mut [Stereo]) {
        for (input, output) in input.iter().zip(output) {
            *output = self.process_input_frame(*input);
        }
    }
}
