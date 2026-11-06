#[cfg(test)]
mod bar_loop_tests;
pub mod bus;
pub mod color;
pub mod compressor;
pub mod delay;
pub mod drums;
pub mod effects;
pub mod eq;
pub mod fast;
mod feedback;
pub mod filter;
pub mod filter_slots;
#[cfg(test)]
mod grain_group_tests;
mod grain_groups;
mod grain_interaction;
mod imported_rate;
pub(crate) mod interaction;
pub mod limiter;
#[cfg(test)]
mod main_pitch_tests;
pub mod modulation;
pub mod oscillators;
pub mod phase_transport;
mod prepared_preset;
pub use prepared_preset::{PreparedPreset, PresetPreparation};
mod rack;
mod radias_filter;
mod radias_oscillator;
mod radias_wavefolder;
pub mod radias_waveshaper;
pub mod resynth;
pub mod sample;
pub mod sampler;
pub(crate) mod sequence;
mod sequence_histories;
mod sequence_sources;
pub mod source_fx;
#[cfg(all(feature = "source-worker", not(target_arch = "wasm32")))]
pub mod source_fx_worker;
pub mod synth;
mod taps;
pub mod tb303;
pub mod telemetry;
pub mod waveshaper;
pub mod wavetable;

use crate::grain_groups as grain_groups_settings;
use crate::{
    Stereo,
    color::{ColorSlot, MAX_COLOR_SLOTS},
    matrix::{self, Source},
    midi::Message,
    performance::{Mode, Performance},
    preset::{
        Destination, Env, FilterPosition, FilterSlot, Lfo, LfoReset, MAX_FILTER_SLOTS, Params,
        Preset,
    },
    tables::{self, Random},
    taps::Stage,
};
use color::ColorRack;
use filter::FilterBank;
use filter_slots::Instance;
use modulation::{Envelope, Oscillator};
pub use sample::{Fades, Region, SampleBuffer};
use sample::{LiveInput, Mip};
use serde::Serialize;
use std::sync::Arc;

pub const MAX_GRAINS: usize = 128;
const _: () = assert!(
    MAX_GRAINS == u128::BITS as usize,
    "one occupancy bit per grain slot"
);
/// How grains read between frames, a CPU choice of the device rather than of
/// a preset. High weighs grains that step faster than their mip level's
/// frames with the wide kernels; Eco reads every grain with the flat kernel,
/// as before them, so pitched-up grains cost no more but fold back again.
/// Grains take it when they start.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GrainQuality {
    #[default]
    High,
    Eco,
}
static ECO_GRAINS: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Set how every engine in this process reads the grains it starts next.
pub fn set_grain_quality(quality: GrainQuality) {
    ECO_GRAINS.store(
        quality == GrainQuality::Eco,
        std::sync::atomic::Ordering::Relaxed,
    );
}
pub fn grain_quality() -> GrainQuality {
    if ECO_GRAINS.load(std::sync::atomic::Ordering::Relaxed) {
        GrainQuality::Eco
    } else {
        GrainQuality::High
    }
}

/// Voice 0 is the free voice; notes take the other 32, so long releases
/// overlap under fast sequences before a tail is stolen.
const VOICES: usize = 33;
const _: () = assert!(VOICES <= u64::BITS as usize, "one sounding bit per voice");
const CONTROL_INTERVAL: u64 = 32;
/// Sampler pads auditioned from the editor own their notes under this bit.
const AUDITION: u32 = 1 << 31;

#[derive(Clone, Copy, Debug, Default)]
struct GroupClock {
    phase: f64,
    slew: interaction::PhaseSlew,
}

#[derive(Clone, Copy, Debug, Default)]
struct Grain {
    position: f64,
    anchor: f64,
    playback_slew: interaction::PhaseSlew,
    playback_phase: f64,
    birth_time: f64,
    rate: f64,
    mip: Mip,
    age: u32,
    duration: u32,
    shape: f32,
    pan: Stereo,
    voice: usize,
    /// A history-granulated MainPitch party; None retains the legacy/file path.
    history: Option<u8>,
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
    /// Main-pitch lanes retain fractional pitch independently of MIDI identity.
    main_pitch: bool,
    pitch: f32,
    oscillator_mask: u32,
    granular: bool,
    /// ENV-off lanes sustain independently of any particular ENV 1 mode.
    env_gate: bool,
    drone_gain: f32,
    drone_release: f32,
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
    /// How far a played-in cloud has faded in (0..1) for each group and,
    /// last, the ungrouped grains, while `Engine::onsets` has this voice's
    /// bit, and how much further each frame.
    onset: [f32; crate::grain_groups::GROUPS + 1],
    onset_step: [f32; crate::grain_groups::GROUPS + 1],
}
impl Voice {
    fn pitch(&self) -> f32 {
        if self.main_pitch {
            self.pitch
        } else {
            self.note as f32
        }
    }
    fn oscillator_mask(&self) -> u32 {
        if self.main_pitch {
            self.oscillator_mask
        } else {
            (1 << 25) - 1
        }
    }
    fn granular(&self) -> bool {
        !self.main_pitch || self.granular
    }
    fn on(&mut self, count: usize) {
        self.generation = self.generation.wrapping_add(1);
        self.gated = false;
        self.loop_phase = 0.0;
        self.value = 0.0;
        for (i, env) in self.envelopes.iter_mut().enumerate() {
            if i < count {
                env.on();
            } else {
                env.kill();
            }
            // Filters configured before the next frame read the level a
            // retrigger continues from: zero would dip a still-open cutoff.
            self.env_values[i] = env.level();
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
        self.drone_gain = 0.0;
    }
    fn active(&self) -> bool {
        self.envelopes.iter().any(Envelope::active)
            || (self.main_pitch && !self.env_gate && (self.held || self.drone_gain > 0.0))
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
    pub(crate) source_color: Option<ColorSlot>,
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
    pub(crate) wavetable: [crate::wavetable::Controls; crate::wavetable::OSCILLATORS],
    pub(crate) wavetable_motion: crate::wavetable::motion::Motion,
    pub(crate) oscillator_controls: [crate::oscillators::Controls; crate::oscillators::SLOTS],
    pub(crate) oscillator_ids: [u32; crate::oscillators::SLOTS],
    pub(crate) oscillator_motion: crate::wavetable::motion::Motion,
    pub(crate) interaction_controls: crate::interaction::Controls,
    pub(crate) tb303: crate::tb303::Controls,
    pub(crate) tb303_radias: crate::synth::Settings,
    /// A modulation route wrote into a color slot of this frame. Clean frames
    /// carry the preset's own slots, so the racks need no reconfiguration.
    pub(crate) color_dirty: bool,
    pub(crate) source_dirty: bool,
    pub(crate) filter_slots: [FilterSlot; MAX_FILTER_SLOTS],
    pub(crate) filter_len: usize,
    pub(crate) grain_groups: crate::grain_groups::Settings,
    /// A Rack's Mixer strips.
    pub(crate) rack_strips: crate::rack::Strips,
    /// A Rack's EQs, by number less one.
    pub(crate) rack_eqs: [crate::eq::Equalizer; crate::eq::MAX_EQS],
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
        self.source_color = preset.source_color_chain.first().copied();
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
        self.oscillator_controls = preset.oscillators.as_deref().map_or_else(
            || {
                std::array::from_fn(|_| crate::oscillators::Controls {
                    enabled: false,
                    ..Default::default()
                })
            },
            crate::oscillators::Settings::controls,
        );
        self.oscillator_ids = preset
            .oscillators
            .as_deref()
            .map_or([0; crate::oscillators::SLOTS], |s| {
                s.slots.each_ref().map(|o| o.id)
            });
        self.oscillator_motion = preset
            .oscillators
            .as_deref()
            .map_or(Default::default(), |s| s.motion);
        self.interaction_controls =
            crate::interaction::Controls::from_settings(&preset.interactions);
        self.wavetable = preset.wavetable.as_deref().map_or_else(
            crate::wavetable::default_controls,
            crate::wavetable::Settings::controls,
        );
        self.wavetable_motion = preset
            .wavetable
            .as_ref()
            .map_or_else(Default::default, |s| s.motion);
        self.tb303 = preset
            .tb303
            .as_ref()
            .map_or_else(Default::default, |s| s.controls);
        self.tb303_radias = preset.tb303.as_ref().map_or_else(
            || crate::synth::Settings {
                count: 0,
                ..Default::default()
            },
            |s| s.radias(),
        );
        self.color_dirty = false;
        self.source_dirty = false;
        self.filter_len = preset.filter_slots.len();
        self.filter_slots[..self.filter_len].copy_from_slice(&preset.filter_slots);
        self.grain_groups = preset.grain_groups;
        self.rack_strips = preset
            .rack
            .as_ref()
            .map_or_else(Default::default, |r| r.strips);
        // Only EQs in the rack play, so others keep what they had.
        if let Some(rack) = preset.rack.as_deref()
            && !rack.equalizers.is_empty()
        {
            self.rack_eqs = rack.equalizer_bank();
        }
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
                    L::Division => (&mut l.division, 0.0625, 128.0, true),
                    L::Depth2 => (&mut l.depth2, -1.0, 1.0, false),
                    L::Depth3 => (&mut l.depth3, -1.0, 1.0, false),
                    L::Shape => (&mut l.shape, -1.0, 1.0, false),
                    L::Phase => (&mut l.phase, 0.0, 1.0, false),
                    L::Smooth => (&mut l.smooth, 0.0, 1.0, false),
                    L::Delay => (&mut l.delay, 0.0, 1.0, false),
                    L::Fade => (&mut l.fade, 0.0, 1.0, false),
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
                let effect =
                    if let Some(effect) = self.source_color.as_mut().filter(|s| s.id == slot) {
                        self.source_dirty = true;
                        effect
                    } else {
                        let Some(effect) = self.color_slots[..self.color_len]
                            .iter_mut()
                            .chain(self.pre_color_slots[..self.pre_color_len].iter_mut())
                            .find(|s| s.id == slot)
                        else {
                            return;
                        };
                        self.color_dirty = true;
                        effect
                    };
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
    /// Canonical control-side assets; the audio path reads prepared banks only.
    pub wavetable_pool: crate::wavetable::assets::Pool,
    preset_runtime: prepared_preset::Resources,
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
    control_inputs: Vec<crate::control::Input>,
    grains: [Grain; MAX_GRAINS],
    grain_interaction: Box<grain_interaction::Runtime>,
    /// Occupied grain slots, bit n for slot n. Visiting set bits from the lowest
    /// keeps the order of a full slot scan, so voices still sum in that order.
    live_grains: u128,
    /// Grains a wide kernel reads, live or not, and their frames this sample.
    wide_grains: u128,
    wide_frames: [Stereo; MAX_GRAINS],
    /// Voices whose played-in grain cloud is still fading in.
    onsets: u64,
    /// Each grain's group, envelopes and filter in advanced mode.
    group_grains: Box<grain_groups::Runtime>,
    group_clocks: [GroupClock; crate::grain_groups::GROUPS],
    group_sync_mask: u128,
    /// The grains' own output on its way back to later grains.
    grain_feedback: feedback::Feedback,
    voices: Box<[Voice; VOICES]>,
    oscillators: [Oscillator; crate::preset::MODULATORS],
    /// Beats since the transport started, or where the host's playhead is;
    /// it keeps counting while stopped. Tempo LFOs read their cycle from it.
    song_beat: f64,
    /// The host's last playhead, adopted only when it moves.
    host_beat: Option<f64>,
    pub lfo_values: [f32; crate::preset::MODULATORS],
    // Fixed reserve allows scope automation without allocations on note-on.
    // Only enabled polyphonic slots are advanced for sounding voices.
    voice_oscillators: Box<[[Oscillator; crate::preset::MODULATORS]; VOICES]>,
    voice_lfo_values: [[f32; crate::preset::MODULATORS]; VOICES],
    pub env_values: [f32; crate::preset::MODULATORS],
    color_rack: Box<ColorRack>,
    pre_color_rack: Box<ColorRack>,
    source_color: color::ColorProcessor,
    oscillator_engine: oscillators::OscillatorEngine,
    /// Each section's Chain share and bus send.
    taps: taps::Runtime,
    /// What enters a Bus or Master chain after its input in the next frame.
    entered: Option<crate::timbres::Entered>,
    /// What a Rack's Pad FX timbres returned, for its Sampler's next frame.
    pad_return: Stereo,
    drums: Option<Box<drums::Runtime>>,
    /// Present while the preset is a Sampler kit: its pads replace the input.
    sampler: Option<Box<sampler::Runtime>>,
    /// Present while the preset is a Rack: its modules' order and Mixer.
    rack: Option<rack::Runtime>,
    /// A Rack's EQ modules, by number.
    equalizers: Box<eq::Bank>,
    pub processed_source: Option<Arc<source_fx::ProcessedSource>>,
    previous_source: Option<Arc<source_fx::ProcessedSource>>,
    source_fade_remaining: u32,
    pub source_fx_error: bool,
    filters: Box<[FilterBank; VOICES]>,
    /// Banks of the added filters, in preset slot order.
    filter_banks: Vec<Box<filter_slots::Bank>>,
    /// Enabled added filters per position, refreshed at control rate.
    filter_masks: [u16; filter_slots::POSITIONS],
    /// Added filters after individual effects in FX A and B.
    filter_fx_masks: filter_slots::FxMasks,
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
    /// Voices that may have an active envelope: a superset of the voices
    /// `Voice::active` finds. Only `Envelope::on` makes an envelope active,
    /// and only `start_voice` calls it, so a voice joins here there and
    /// leaves once a check finds every envelope idle.
    maybe_active: u64,
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
    /// (preset revision, bpm bits, root note bits, live slot) the source FX last saw.
    source_key: Option<(u64, u32, u32, bool)>,
    source_configured_dirty: bool,
    color_revision: u64,
    applied_shaper_pitch: u32,
    overlap: f32,
    smooth_grain_norm: f32,
    grain_norm_smoothing: f32,
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
    limiter: limiter::Limiter,
}

/// Constructs the engine's independent resource banks in bounded stages.
/// No audio or event state advances until the completed engine is installed.
pub struct EnginePreparation {
    sample_rate: u32,
    stage: u8,
    grain_interaction: Option<Box<grain_interaction::Runtime>>,
    group_grains: Option<Box<grain_groups::Runtime>>,
    voices: Option<Box<[Voice; VOICES]>>,
    voice_oscillators: Option<Box<[[Oscillator; crate::preset::MODULATORS]; VOICES]>>,
    color_rack: Option<Box<ColorRack>>,
    pre_color_rack: Option<Box<ColorRack>>,
    resynth: Option<Box<resynth::Resynth>>,
    graph: Option<Box<interaction::Runtime>>,
    equalizers: Option<Box<eq::Bank>>,
    filters: Option<Box<[FilterBank; VOICES]>>,
    modulated: Option<Box<Modulated>>,
    spare_modulated: Option<Box<Modulated>>,
}
impl EnginePreparation {
    pub fn new(sample_rate: u32) -> Self {
        fast::warm_up();
        sample::prepare_interpolation();
        Self {
            sample_rate: sample_rate.clamp(8_000, 384_000),
            stage: 0,
            grain_interaction: None,
            group_grains: None,
            voices: None,
            voice_oscillators: None,
            color_rack: None,
            pre_color_rack: None,
            resynth: None,
            graph: None,
            equalizers: None,
            filters: None,
            modulated: None,
            spare_modulated: None,
        }
    }
    /// Prepares one independent bank, never the whole engine in one call.
    pub fn step(&mut self) -> bool {
        let sr = self.sample_rate;
        match self.stage {
            0 => self.grain_interaction = Some(Box::new(grain_interaction::Runtime::new(sr))),
            1 => self.group_grains = Some(Box::new(grain_groups::Runtime::new(sr))),
            2 => self.voices = Some(Box::new([Voice::default(); VOICES])),
            3 => {
                self.voice_oscillators = Some(Box::new(std::array::from_fn(|v| {
                    std::array::from_fn(|i| {
                        Oscillator::new(0x504f4c59 ^ (v as u64) << 16 ^ i as u64)
                    })
                })));
            }
            4 => self.color_rack = Some(Box::new(ColorRack::new(sr))),
            5 => self.pre_color_rack = Some(Box::new(ColorRack::new(sr))),
            6 => self.resynth = Some(Box::new(resynth::Resynth::new(sr))),
            7 => self.graph = Some(Box::new(interaction::Runtime::new(sr))),
            8 => self.equalizers = Some(Box::new(eq::Bank::new(sr))),
            9 => {
                self.filters = Some(
                    (0..VOICES)
                        .map(|_| FilterBank::new(sr))
                        .collect::<Vec<_>>()
                        .into_boxed_slice()
                        .try_into()
                        .unwrap_or_else(|_| unreachable!()),
                );
            }
            10 => self.modulated = Some(Box::new(Modulated::default())),
            11 => self.spare_modulated = Some(Box::new(Modulated::default())),
            _ => return true,
        }
        self.stage += 1;
        self.stage == 12
    }
    /// Applying the supplied preset may prepare its own resources; live hosts
    /// finish with Init, then prepare their instrument separately before commit.
    pub fn finish(self, sample: Arc<SampleBuffer>, preset: Preset) -> Option<Engine> {
        (self.stage == 12)
            .then(|| Engine::from_preparation(self, sample, preset, Default::default(), None))
    }
}

impl Engine {
    pub fn new(sample_rate: u32, sample: Arc<SampleBuffer>, preset: Preset) -> Self {
        let mut prepared = EnginePreparation::new(sample_rate);
        while !prepared.step() {}
        prepared.finish(sample, preset).unwrap()
    }

    /// Construct from a document and an explicit canonical asset owner.
    pub fn new_with_assets(
        sample_rate: u32,
        sample: Arc<SampleBuffer>,
        preset: Preset,
        assets: crate::wavetable::assets::Pool,
    ) -> Self {
        let mut preparation = EnginePreparation::new(sample_rate);
        while !preparation.step() {}
        Self::from_preparation(preparation, sample, preset, assets, None)
    }

    /// A host has already prepared this update, including its external kit.
    /// Canonical originals remain with the host; set `wavetable_pool` explicitly
    /// if this engine will also resolve assets for later control-side edits.
    pub fn from_prepared(sample: Arc<SampleBuffer>, preset: PreparedPreset) -> Self {
        let mut preparation = EnginePreparation::new(preset.sample_rate());
        while !preparation.step() {}
        Self::from_preparation(
            preparation,
            sample,
            Preset::default(),
            Default::default(),
            Some(preset),
        )
    }

    fn from_preparation(
        prepared: EnginePreparation,
        sample: Arc<SampleBuffer>,
        preset: Preset,
        assets: crate::wavetable::assets::Pool,
        update: Option<PreparedPreset>,
    ) -> Self {
        let sr = prepared.sample_rate;
        let mut engine = Self {
            sample_rate: sr,
            preset: Preset::default(),
            wavetable_pool: assets,
            preset_runtime: Default::default(),
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
            control_inputs: Vec::new(),
            grains: [Grain::default(); MAX_GRAINS],
            grain_interaction: prepared.grain_interaction.unwrap(),
            live_grains: 0,
            wide_grains: 0,
            wide_frames: [[0.0; 2]; MAX_GRAINS],
            onsets: 0,
            group_grains: prepared.group_grains.unwrap(),
            group_clocks: [GroupClock::default(); crate::grain_groups::GROUPS],
            group_sync_mask: 0,
            grain_feedback: feedback::Feedback::new(),
            voices: prepared.voices.unwrap(),
            oscillators: std::array::from_fn(|i| {
                Oscillator::new(0x853c49e6748fea9b_u64.wrapping_add(i as u64 * 0x12345679))
            }),
            song_beat: 0.0,
            host_beat: None,
            lfo_values: [0.0; crate::preset::MODULATORS],
            voice_oscillators: prepared.voice_oscillators.unwrap(),
            voice_lfo_values: [[0.0; crate::preset::MODULATORS]; VOICES],
            env_values: [0.0; crate::preset::MODULATORS],
            color_rack: prepared.color_rack.unwrap(),
            pre_color_rack: prepared.pre_color_rack.unwrap(),
            source_color: color::ColorProcessor::new(sr),
            oscillator_engine: oscillators::OscillatorEngine {
                synth: None,
                wavetable: None,
                resynth: prepared.resynth.unwrap(),
                tb303: None,
                graph: prepared.graph.unwrap(),
                mixed: None,
                phase_adapters: None,
            },
            taps: taps::Runtime::new(),
            entered: None,
            pad_return: [0.0; 2],
            drums: None,
            sampler: None,
            rack: None,
            equalizers: prepared.equalizers.unwrap(),
            processed_source: None,
            previous_source: None,
            source_fade_remaining: 0,
            source_fx_error: false,
            filters: prepared.filters.unwrap(),
            filter_banks: Vec::new(),
            filter_masks: [0; filter_slots::POSITIONS],
            filter_fx_masks: [[0; MAX_COLOR_SLOTS]; 2],
            random: Random::default(),
            frame: 0,
            cursor: 0.0,
            bar_beats: 4.0,
            frozen_center: 0.0,
            last_position: 0.0,
            grain_clock: 1.0,
            next_voice: 0,
            sounding: 0,
            maybe_active: 0,
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
            modulated: prepared.modulated.unwrap(),
            spare_modulated: prepared.spare_modulated,
            rack_key: None,
            rack_configured_dirty: false,
            source_key: None,
            source_configured_dirty: false,
            color_revision: 0,
            applied_shaper_pitch: 60.0_f32.to_bits(),
            overlap: 1.0,
            smooth_grain_norm: 1.0,
            grain_norm_smoothing: 1.0 - (-1.0 / (sr as f32 * 0.002)).exp(),
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
            limiter: limiter::Limiter::new(sr),
        };
        if let Some(mut update) = update {
            engine.exchange_preset(&mut update);
        } else {
            engine.set_preset(preset);
        }
        // Idle voices need their initial coefficients even when the first preset
        // has the same envelopes as Modulated::default(). Later refreshes only
        // configure envelopes whose values actually changed.
        for voice in engine.voices.iter_mut() {
            for (i, envelope) in voice.envelopes.iter_mut().enumerate() {
                envelope.configure_tempo(
                    sr as f32,
                    &engine.modulated.envs[i],
                    engine.modulated.params.bpm,
                );
            }
        }
        engine
    }

    /// Prepare against this engine's active banks. The returned update must be
    /// exchanged with this engine before rendering resumes; queued hosts instead
    /// use PreparedPreset::prepare with independent resources.
    pub fn prepare_preset_update(&mut self, preset: Preset) -> Result<PreparedPreset, String> {
        let assets = self.wavetable_pool.clone();
        let mut preparation = PresetPreparation::new(preset, self.sample_rate);
        while !preparation.step(&assets, Some(self))? {}
        preparation.finish()
    }

    /// Synchronous control/offline convenience. Replaced resources drop here,
    /// outside rendering; realtime owners exchange a PreparedPreset themselves.
    pub fn set_preset(&mut self, preset: Preset) {
        if let Ok(mut prepared) = self.prepare_preset_update(preset) {
            self.exchange_preset(&mut prepared);
        }
    }

    pub fn wavetable_motion(&self) -> crate::wavetable::motion::Motion {
        self.modulated.wavetable_motion
    }
    pub fn wavetable_positions(&self) -> [wavetable::PositionView; crate::wavetable::OSCILLATORS] {
        self.oscillator_engine
            .wavetable
            .as_ref()
            .map_or_else(Default::default, |runtime| runtime.positions())
    }
    pub fn oscillator_positions(&self) -> [wavetable::PositionView; crate::oscillators::SLOTS] {
        self.oscillator_engine
            .mixed
            .as_ref()
            .map_or([Default::default(); crate::oscillators::SLOTS], |r| {
                r.positions()
            })
    }
    pub fn oscillator_motion(&self) -> crate::wavetable::motion::Motion {
        self.modulated.oscillator_motion
    }
    pub fn retire_oscillator_banks(
        &mut self,
        retired: &mut [Option<Arc<wavetable::Bank>>; crate::oscillators::SLOTS],
    ) {
        if let Some(runtime) = &mut self.oscillator_engine.mixed {
            runtime.retire_into(retired);
        }
    }

    /// Move completed table crossfades into a control-side retirement slot.
    /// The caller releases these handles between render calls.
    pub fn retire_wavetable_banks(
        &mut self,
        retired: &mut [Option<Arc<wavetable::Bank>>; crate::wavetable::OSCILLATORS],
    ) {
        if let Some(runtime) = &mut self.oscillator_engine.wavetable {
            runtime.retire_into(retired);
        }
    }

    /// Install a finalized update without allocating or destroying buffers.
    /// The same envelope receives every old resource for off-thread retirement.
    /// An incompatible format or already retired envelope is left untouched.
    pub fn exchange_preset(&mut self, prepared: &mut PreparedPreset) -> bool {
        if !prepared.ready || prepared.sample_rate != self.sample_rate {
            return false;
        }
        let PreparedPreset {
            preset,
            resources: runtime,
            ready,
            ..
        } = prepared;
        for slot in 0..crate::oscillators::SLOTS {
            let before = self
                .preset
                .oscillators
                .as_ref()
                .map(|s| (s.slots[slot].id, s.slots[slot].kind));
            let after = preset
                .oscillators
                .as_ref()
                .map(|s| (s.slots[slot].id, s.slots[slot].kind));
            if before != after {
                self.oscillator_engine.graph.invalidate_node(slot);
            }
        }
        // A live input stays while the next preset can play it: a Granular
        // timbre or a Rack whose Sample it stands in for.
        let plays_input = |p: &Preset| {
            p.history().is_none()
                && p.bus.is_none()
                && p.drums.is_none()
                && p.synth.is_none()
                && p.rack.as_ref().is_none_or(|r| r.takes_input())
        };
        let input = self.live_input.is_some() && plays_input(&self.preset);
        let keep_input = input && plays_input(preset);
        // A Rack's history waits aside while the sources change, so no Drum
        // or Synth runtime takes it; it stays if the next Rack still reads one.
        // An input is never one: where it cannot play on, it retires.
        let mut rack_history = if self.preset.rack.is_some() && !input {
            self.live_input.take()
        } else {
            None
        };
        // A bus history is never freed here: it retires with the outgoing
        // preset, whose bus_runtime is empty while that preset is installed.
        let was_bus = self.preset.bus.is_some();
        if was_bus && preset.bus.is_none() {
            std::mem::swap(&mut self.live_input, &mut self.preset_runtime.bus_runtime.0);
        }
        // A retiring synth restores the input it replaced before drums install.
        if runtime.synth_runtime.0.is_none()
            && let Some(old) = &mut self.oscillator_engine.synth
        {
            std::mem::swap(&mut self.live_input, &mut old.live);
        }
        if let Some(next) = &mut runtime.drum_runtime.0 {
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
        std::mem::swap(&mut self.drums, &mut runtime.drum_runtime.0);
        if let Some(next) = &mut runtime.synth_runtime.0 {
            if let Some(old) = &mut self.oscillator_engine.synth {
                next.inherit(old);
            } else {
                std::mem::swap(&mut self.live_input, &mut next.live);
            }
        }
        std::mem::swap(
            &mut self.oscillator_engine.synth,
            &mut runtime.synth_runtime.0,
        );
        if let (Some(next), Some(old)) = (
            &mut runtime.wavetable_runtime.0,
            &mut self.oscillator_engine.wavetable,
        ) {
            next.inherit(old);
            if self.sounding == 0 {
                next.finish_silent_crossfades();
            }
        }
        std::mem::swap(
            &mut self.oscillator_engine.wavetable,
            &mut runtime.wavetable_runtime.0,
        );
        if let (Some(next), Some(old)) = (
            &mut runtime.oscillator_runtime.0,
            &mut self.oscillator_engine.mixed,
        ) {
            next.inherit(old);
        }
        std::mem::swap(
            &mut self.oscillator_engine.mixed,
            &mut runtime.oscillator_runtime.0,
        );
        if let (Some(next), Some(old)) = (
            &mut runtime.phase_runtime.0,
            &mut self.oscillator_engine.phase_adapters,
        ) {
            next.inherit(old);
        }
        std::mem::swap(
            &mut self.oscillator_engine.phase_adapters,
            &mut runtime.phase_runtime.0,
        );
        if let (Some(next), Some(old)) = (
            &mut runtime.tb303_runtime.0,
            &mut self.oscillator_engine.tb303,
        ) {
            next.inherit(old);
        }
        std::mem::swap(
            &mut self.oscillator_engine.tb303,
            &mut runtime.tb303_runtime.0,
        );
        if let (Some(next), Some(old)) = (&mut runtime.sequence, &mut self.preset_runtime.sequence)
        {
            next.inherit(old);
        }
        if let (Some(next), Some(old)) = (
            &mut runtime.sequence_sources,
            &mut self.preset_runtime.sequence_sources,
        ) {
            next.inherit(old);
        }
        if let (Some(next), Some(old)) = (
            &mut runtime.sequence_histories,
            &mut self.preset_runtime.sequence_histories,
        ) {
            next.inherit(old);
        }
        for voice in self.voices.iter_mut() {
            if voice.owner != 0
                && (!preset.sequencer.enabled
                    || !runtime.sequence.as_ref().is_some_and(|r| {
                        if voice.main_pitch {
                            r.main_pitch_owner(voice.owner)
                        } else {
                            r.retained_note(voice.owner)
                        }
                    }))
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
            .detach_imported(&mut self.preset_runtime.imported_effects);
        self.pre_color_rack
            .detach_imported(&mut self.preset_runtime.imported_effects);
        self.source_color
            .detach_imported(&mut self.preset_runtime.imported_effects);
        runtime.imported_effects.inherit_matching(
            &mut self.preset_runtime.imported_effects,
            preset
                .color_chain
                .iter()
                .chain(&preset.pre_color_chain)
                .chain(&preset.source_color_chain),
        );
        self.color_rack
            .attach_imported(&preset.color_chain, &mut runtime.imported_effects);
        self.pre_color_rack
            .attach_imported(&preset.pre_color_chain, &mut runtime.imported_effects);
        if let Some(slot) = preset.source_color_chain.first() {
            self.source_color
                .attach_imported(*slot, &mut runtime.imported_effects);
        }
        filter_slots::inherit(&mut runtime.filter_banks.0, &mut self.filter_banks);
        std::mem::swap(&mut self.filter_banks, &mut runtime.filter_banks.0);
        if preset.rack.is_some() && keep_input {
            // The input plays on in the next Rack; what the old one held retires.
            runtime.rack_runtime.0 = rack_history.take();
        } else if preset.rack.is_some() {
            let prepared = runtime.rack_runtime.0.take();
            let (kept, retired) = if rack_history.is_some() && preset.history().is_some() {
                // The running history stays; the prepared one retires unused.
                (rack_history.take(), prepared)
            } else {
                (prepared, rack_history.take())
            };
            // What the previous kind recorded into retires instead.
            let previous = std::mem::replace(&mut self.live_input, kept);
            debug_assert!(retired.is_none() || previous.is_none());
            runtime.rack_runtime.0 = retired.or(previous);
        } else {
            runtime.rack_runtime.0 = rack_history.take();
        }
        // It retires with the outgoing preset.
        std::mem::swap(
            &mut self.preset_runtime.rack_runtime.0,
            &mut runtime.rack_runtime.0,
        );
        self.rack = preset
            .rack
            .as_deref()
            .map(|settings| rack::Runtime::new(settings, self.rack.as_ref()));
        self.equalizers
            .activate(self.rack.as_ref().map_or(0, |r| r.equalizers));
        if let Some(bus) = &preset.bus {
            // Granular staying on keeps the running history and its audio.
            if !(bus.granular && self.bus_history()) {
                std::mem::swap(&mut self.live_input, &mut runtime.bus_runtime.0);
            }
            std::mem::swap(
                &mut self.preset_runtime.bus_runtime.0,
                &mut runtime.bus_runtime.0,
            );
        }
        std::mem::swap(&mut self.preset, preset);
        std::mem::swap(&mut self.preset_runtime, runtime);
        // A kit's runtime prepared off the audio thread takes over its
        // voices; the one it replaces, or a kit's last, retires with the
        // outgoing preset, whose slot was emptied when it was installed.
        match self.preset_runtime.sampler_runtime.0.take() {
            Some(mut next) => {
                if let Some(old) = &mut self.sampler {
                    next.inherit(old);
                }
                runtime.sampler_runtime.0 = self.sampler.replace(next);
            }
            None if self.preset.sampler.is_none() => {
                runtime.sampler_runtime.0 = self.sampler.take();
            }
            None => self.sync_sampler(),
        }
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
        *ready = false;
        true
    }
    pub fn set_sample(&mut self, sample: Arc<SampleBuffer>) {
        if self.preset.bus.is_none() && self.takes_input() {
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
        if self.preset.rack.is_some() {
            return Ok(());
        }
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
    /// Whether the file plays through a Source FX cache. A live input has no
    /// file, and a Rack's Insert FX runs live wherever it stands.
    pub fn reads_source_cache(&self) -> bool {
        self.live_input.is_none() && self.preset.rack.is_none()
    }
    pub fn source_effects_pending(&self) -> bool {
        if !self.reads_source_cache() || self.source_fx_error {
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
        // A Rack keeps the history its Grain engine records; one whose Sample
        // an input can stand in for takes the input as a Granular timbre does.
        if self.takes_input() {
            std::mem::swap(&mut self.live_input, live_input);
        }
        self.reset_source();
    }
    fn reset_source(&mut self) {
        self.oscillator_engine.graph.clear();
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
        self.oscillator_engine.resynth.clear();
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
            for input in &mut self.control_inputs {
                input.bpm = bpm;
            }
        }
        // A playing host moves its playhead; a stopped one leaves the song
        // position counting on from there.
        if let Some(beat) = beat.filter(|b| b.is_finite()) {
            if let Some(bass) = &mut self.oscillator_engine.tb303 {
                bass.seek(beat, restart);
            }
            if restart || self.host_beat != Some(beat) {
                self.song_beat = beat.max(0.0);
            }
            self.host_beat = Some(beat);
        }
        if restart {
            for input in &mut self.control_inputs {
                if let Some(runtime) = &mut input.runtime {
                    runtime.seek(beat.filter(|b| b.is_finite()).unwrap_or(0.0));
                }
            }
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
            if sequencer && let Some(runtime) = &mut self.preset_runtime.sequence {
                runtime.seek(beat.filter(|b| b.is_finite()).unwrap_or(0.0));
            }
            if let Some(beat) = beat.filter(|b| b.is_finite()) {
                self.performance.seek(beat);
                for (oscillator, lfo) in self.oscillators.iter_mut().zip(self.preset.lfos) {
                    if lfo.sync && lfo.reset == LfoReset::Free {
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
            && self.preset.sequencer.tracks.iter().any(|t| {
                matches!(
                    t.target,
                    crate::sequence::Target::Notes | crate::sequence::Target::MainPitch
                )
            })
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
        let first = !self
            .voices
            .iter()
            .enumerate()
            .any(|(i, v)| i != index && (v.held || v.sustained));
        for (oscillator, lfo) in self.oscillators.iter_mut().zip(&self.preset.lfos) {
            match lfo.reset {
                LfoReset::Note => oscillator.reset(),
                LfoReset::Free | LfoReset::Tempo if first && lfo.one_shot => oscillator.reset(),
                LfoReset::Free | LfoReset::Tempo if first => oscillator.restart_fade(),
                _ => {}
            }
        }
        for (i, lfo) in self
            .modulated
            .lfos
            .iter()
            .enumerate()
            .take(self.modulated.lfo_count)
        {
            if lfo.reset == LfoReset::Note {
                self.voice_oscillators[index][i].reset();
            } else {
                self.voice_oscillators[index][i] = self.oscillators[i].clone();
            }
        }
        self.voices[index].on(self.preset.env_count as usize);
        self.sounding |= 1 << index;
        self.maybe_active |= 1 << index;
        self.prefill_grains(index);
    }
    /// Voice 0 is the note held while playing without keys or note tracks.
    fn start_free_voice(&mut self) {
        self.voices[0].main_pitch = false;
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
    /// A Main pitch lane owns one held voice; released tails keep their last
    /// pitch and routing until their own envelopes finish.
    fn sync_main_pitch_voices(&mut self, events: bool) {
        let count = self.preset_runtime.sequence.as_ref().map_or(0, |r| r.len());
        for lane in 0..count {
            let Some(state) = self
                .preset_runtime
                .sequence
                .as_ref()
                .and_then(|r| r.main_pitch(lane))
            else {
                continue;
            };
            let current = (1..VOICES).find(|&i| {
                self.voices[i].main_pitch && self.voices[i].owner == state.id && self.voices[i].held
            });
            if (!state.open || (events && state.trigger))
                && let Some(i) = current
            {
                self.voices[i].held = false;
                self.voices[i].off();
            }
            if !state.open {
                continue;
            }
            let current = current.filter(|_| !(events && state.trigger));
            let pitch = self.modulated.params.sample_root_note + state.offset;
            let index = current.unwrap_or_else(|| {
                let index = (1..VOICES)
                    .find(|&i| !self.voices[i].active())
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
                self.kill_voice_grains(index);
                self.clear_voice_filters(index);
                let v = &mut self.voices[index];
                v.owner = state.id;
                v.main_pitch = true;
                v.note = pitch.round().clamp(0.0, 127.0) as u8;
                v.pitch = pitch;
                v.oscillator_mask = state.oscillators;
                v.granular = state.granular;
                v.env_gate = state.env_gate;
                v.drone_gain = 0.0;
                v.channel = 0;
                v.velocity = state.velocity as f32 / 127.0;
                v.held = true;
                v.sustained = false;
                self.start_voice(index);
                self.mod_note_on(0, pitch.round().clamp(0.0, 127.0) as u8, state.velocity);
                self.grain_clock = 1.0;
                index
            });
            let v = &mut self.voices[index];
            if v.env_gate && !state.env_gate {
                v.drone_gain = if v.velocity > 0.0 {
                    v.value / v.velocity
                } else {
                    0.0
                };
            }
            v.env_gate = state.env_gate;
            v.drone_release =
                (-9.0 / (self.modulated.envs[0].release_seconds() * self.sample_rate as f32)).exp();
            v.pitch = pitch;
            v.note = pitch.round().clamp(0.0, 127.0) as u8;
            v.oscillator_mask = state.oscillators;
            v.granular = state.granular;
            v.velocity = state.velocity as f32 / 127.0;
            self.configure_voice_filter(index);
        }
        if let Some(histories) = &mut self.preset_runtime.sequence_histories {
            let owners: [u32; VOICES] = std::array::from_fn(|i| {
                let v = self.voices[i];
                if v.main_pitch && (v.held || v.active()) {
                    v.owner
                } else {
                    0
                }
            });
            for &owner in &owners {
                if owner != 0 {
                    histories.ensure(owner, &owners);
                }
            }
        }
    }
    pub fn apply_parameters(&mut self, changes: &[crate::parameters::Change]) {
        if crate::parameters::apply(&mut self.preset, changes) {
            self.refresh_parameters();
        }
    }
    /// Prepared elsewhere; retain each Control's clock across edits, and return
    /// the old allocation to the caller for retirement off the audio thread.
    pub fn has_control_inputs(&self) -> bool {
        !self.control_inputs.is_empty()
    }

    pub fn exchange_control_inputs(&mut self, next: &mut Vec<crate::control::Input>) {
        for input in next.iter_mut() {
            if let Some(old) = self
                .control_inputs
                .iter_mut()
                .find(|i| i.source == input.source)
            {
                if let (Some(runtime), Some(previous)) = (&mut input.runtime, &mut old.runtime) {
                    runtime.inherit(previous);
                }
            } else if let Some(runtime) = &mut input.runtime {
                runtime.start(input.program.enabled && self.playing && self.sequence_running);
            }
        }
        std::mem::swap(&mut self.control_inputs, next);
        self.invalidate_racks();
        self.update_modulation();
    }

    pub fn apply_sequence_edits(&mut self, edits: &[crate::sequence::Edit]) {
        if let Some(runtime) = &mut self.preset_runtime.sequence {
            runtime.edit_program(&mut self.preset.sequencer, edits);
        } else {
            self.preset.sequencer.edit(edits);
        }
        for edit in edits.iter().filter(|e| {
            e.stage.is_none() && (matches!(e.field, 47 | 56) || (e.field == 8 && e.value >= 0.5))
        }) {
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
            if let Some(runtime) = &mut self.preset_runtime.sequence {
                runtime.restart();
            }
        }
        self.invalidate_racks();
        self.update_modulation();
        self.refresh_sequence_free_voice();
    }
    pub fn apply_control_sequence_edits(&mut self, source: u32, edits: &[crate::sequence::Edit]) {
        let Some(input) = self.control_inputs.iter_mut().find(|i| i.source == source) else {
            return;
        };
        for &edit in edits {
            let edit = if edit.stage.is_none() && edit.field == 44 {
                crate::sequence::Edit { value: 0., ..edit }
            } else {
                edit
            };
            if let Some(runtime) = &mut input.runtime {
                runtime.edit_program(&mut input.program, std::slice::from_ref(&edit));
            } else {
                input.program.edit(std::slice::from_ref(&edit));
            }
        }
        if edits.iter().any(|e| e.track == 0 && e.field == 1)
            && let Some(runtime) = &mut input.runtime
        {
            runtime.restart();
        }
        self.invalidate_racks();
        self.update_modulation();
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
        for input in &mut self.control_inputs {
            if let Some(runtime) = &mut input.runtime {
                runtime.start(running && input.program.enabled && self.playing);
            }
        }
        if let Some(drums) = &mut self.drums {
            if running {
                drums.engine.play();
            } else {
                drums.engine.stop();
            }
        }
        self.release_sequence_notes();
        if let Some(runtime) = &mut self.preset_runtime.sequence {
            runtime.start(running && self.preset.sequencer.enabled);
        }
        self.invalidate_racks();
        self.update_modulation();
        self.refresh_sequence_free_voice();
    }
    pub fn set_sequence_view(&mut self, offset: usize) {
        if let Some(runtime) = &mut self.preset_runtime.sequence {
            runtime.view_offset = offset.min(self.preset.sequencer.tracks.len().saturating_sub(1));
        }
    }
    pub fn sequence_view(&self) -> crate::sequence::View {
        self.preset_runtime
            .sequence
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
        if self.preset.history().is_some() || !self.takes_input() {
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
                && !self.oscillator_engine.resynth.active()
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
        if !self.preset.bus.as_ref().is_some_and(|b| b.master) {
            for (peak, value) in self.peak.iter_mut().zip(output) {
                *peak = peak.max(value.abs());
            }
        }
        self.frame += 1;
        output
    }
    /// Hosts call this once, after the Master fader and every return/preview.
    /// Intermediate buses retain headroom; the Master meter reads the actual
    /// output rather than the signal before the host's fader.
    pub fn protect_master(&mut self, input: Stereo) -> Stereo {
        let Some(bus) = self.preset.bus.as_ref().filter(|b| b.master) else {
            return input;
        };
        let output = self.limiter.process(input, bus.limiter);
        for (peak, value) in self.peak.iter_mut().zip(output) {
            *peak = peak.max(value.abs());
        }
        output
    }
    pub fn master_meter(&self) -> limiter::Meter {
        self.limiter.meter()
    }
    pub fn source_sample(&self) -> &SampleBuffer {
        self.live_input
            .as_ref()
            .map_or(&self.sample, |input| &input.sample)
    }
    /// A Rack with a Sample shows its file, even while its Grain engine
    /// reads a history of some chain, unless a live input plays instead.
    fn shows_file(&self) -> bool {
        self.preset
            .rack
            .as_ref()
            .is_some_and(|rack| rack.has(crate::rack::Module::Sample) && !self.rack_input())
    }
    /// Whether a live input can play here: in any timbre but a Rack, and in a
    /// Rack whose Sample it can stand in for (`rack::Settings::takes_input`).
    pub fn takes_input(&self) -> bool {
        self.preset.rack.as_ref().is_none_or(|r| r.takes_input())
    }
    /// A Rack's Sample plays a live input now.
    pub(crate) fn rack_input(&self) -> bool {
        self.live_input.is_some() && self.preset.rack.as_ref().is_some_and(|r| r.takes_input())
    }
    /// What a host shows as the source: `source_sample` but for a Rack's file.
    pub fn shown_sample(&self) -> &SampleBuffer {
        if self.shows_file() {
            &self.sample
        } else {
            self.source_sample()
        }
    }
    pub fn shown_frames(&self) -> usize {
        if self.shows_file() {
            self.sample.len()
        } else {
            self.source_frames()
        }
    }
    pub fn shown_duration(&self) -> f64 {
        if self.shows_file() {
            self.sample.duration()
        } else {
            self.source_duration()
        }
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
        self.onsets = 0;
        self.grain_clock = 1.0;
        self.group_clocks = [GroupClock::default(); 4];
        self.group_grains.reset();
        self.grain_feedback.reset();
    }
    pub fn set_playing(&mut self, playing: bool) {
        if let Some(bass) = &mut self.oscillator_engine.tb303 {
            if playing {
                bass.transport(true);
            } else {
                bass.stop();
            }
        }
        if playing && !self.playing {
            self.song_beat = 0.0;
        }
        self.playing = playing;
        for input in &mut self.control_inputs {
            if let Some(runtime) = &mut input.runtime {
                runtime.start(input.program.enabled && playing && self.sequence_running);
            }
        }
        if let Some(drums) = &mut self.drums {
            if playing && self.sequence_running {
                drums.engine.play();
            } else {
                drums.engine.stop();
            }
        }
        if let Some(runtime) = &mut self.preset_runtime.sequence {
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
            let margin = self.history_margin();
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
    /// Leave enough history for a grain to advance without overtaking the
    /// capture head, a group's longer grains included. Position scans
    /// backwards through the history.
    fn history_margin(&self) -> f64 {
        let length = self.source_range().len() as f64;
        let p = &self.modulated.params;
        let groups = &self.modulated.grain_groups;
        let longest = groups
            .groups
            .iter()
            .filter(|g| groups.advanced && g.share > 0.0 && g.size > 0.0)
            .map(|g| g.size)
            .fold(0.0, f32::max);
        let seconds = if longest > 0.0 {
            (self.grain_seconds(p.grain_size) * longest.exp2()).min(self.grain_seconds(1.0))
        } else {
            self.grain_seconds(p.grain_size)
        };
        (seconds as f64 * self.sample_rate as f64 * 2.0 + self.cloud_width() * 0.5 + 4.0)
            .min(length * 0.9)
    }
    /// How wide CLOUD scatters grains, in source frames: its share of the
    /// source, or in Time mode its span of time, never wider than the source.
    fn cloud_width(&self) -> f64 {
        let p = &self.modulated.params;
        let length = self.source_range().len() as f64;
        if p.cloud_time {
            let seconds = crate::preset::CLOUD_SECONDS * p.cloud * p.cloud;
            (seconds as f64 * self.source_sample().sample_rate as f64).min(length)
        } else {
            p.cloud as f64 * length
        }
    }
    /// A group's grain center: its POSITION moves the engine's, around a
    /// file and, within a history, no nearer the head than the engine goes.
    fn layer_center(&self, voice: usize, offset: f32) -> f64 {
        let center = self.voice_center(voice);
        if offset == 0.0 {
            return center;
        }
        let range = self.source_range();
        let length = range.len() as f64;
        let shift = if self.live_input.is_some() {
            let position = self.modulated.params.position;
            let target = (position + offset).clamp(0.0, 1.0);
            -((target - position) as f64) * (length - self.history_margin())
        } else {
            offset as f64 * length
        };
        range.start as f64 + (center - range.start as f64 + shift).rem_euclid(length)
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
        // Its next note fades in afresh, or not at all.
        self.onsets &= !(1 << voice);
        for slot in self.live_grain_slots() {
            if self.grains[slot].voice == voice {
                self.live_grains &= !(1 << slot);
            }
        }
    }
    pub fn held_notes(&self) -> Vec<u8> {
        let mut notes: Vec<u8> = self.voices[1..]
            .iter()
            .filter(|v| v.held || v.sustained)
            .map(|v| v.note)
            .collect();
        if let Some((_, note, _)) = self
            .oscillator_engine
            .tb303
            .as_ref()
            .and_then(|b| b.note_state())
            && !notes.contains(&note)
        {
            notes.push(note);
        }
        notes
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
            // In a Rack the notes play its Synth and Sample; the drums play
            // their pattern.
            if let Message::NoteOn {
                note,
                velocity: 1..=127,
                ..
            } = message
                && self.preset.rack.is_none()
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
        let pattern_owns_303 = owner != 0 && self.preset.tb303.as_ref().is_some_and(|s| s.pattern);
        if !pattern_owns_303
            && let Some(bass) = &mut self.oscillator_engine.tb303
            && bass.midi(message, owner)
        {
            if let Message::PitchBend { channel, value } = message
                && channel < 16
            {
                self.bend[channel as usize] = (value.min(16383) as f32 - 8192.0) / 8192.0;
            }
            if self.rack.is_none() {
                return;
            }
        }
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
                v.main_pitch = false;
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
            Message::Program { value, .. } => self.set_preset(Preset::factory(value as usize % 5)),
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
                    .oscillator_engine
                    .tb303
                    .as_ref()
                    .is_some_and(|b| b.note_state().is_some())
                {
                    return 1.0;
                }
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
        // File Source FX is baked off-thread; only live inputs and Rack inserts
        // take control-rate modulation, without invalidating the file cache.
        if self.reads_source_cache() && self.preset.bus.is_none() {
            m.source_color = None;
        }
        if !self.control_inputs.is_empty() && self.sequence_running {
            // All Control clocks and the parent's lanes contribute before a
            // single clamp. Opposing offsets cancel even near a range's ends.
            let mut sums = std::mem::take(&mut self.control_inputs[0].sums);
            sums.clear();
            if self.preset.sequencer.enabled
                && let Some(runtime) = &self.preset_runtime.sequence
            {
                runtime.accumulate(&m, &mut sums);
            }
            for input in &self.control_inputs {
                if input.program.enabled
                    && !input.muted
                    && let Some(runtime) = &input.runtime
                {
                    runtime.accumulate(&m, &mut sums);
                }
            }
            for &(index, delta) in &sums {
                let spec = &crate::parameters::specs()[index];
                if let Some(base) = spec.frame_get(&m) {
                    spec.frame_set(&mut m, base + delta);
                }
            }
            if self.preset.sequencer.enabled
                && let Some(runtime) = &self.preset_runtime.sequence
            {
                runtime.apply_gates(&mut m);
            }
            self.control_inputs[0].sums = sums;
        } else if self.preset.sequencer.enabled
            && self.sequence_running
            && let Some(runtime) = &self.preset_runtime.sequence
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
                // A unipolar LFO swings 0..1 over the same span as -0.5..0.5.
                Modulator::Lfo(i) if m.lfos[i].unipolar => self.lfo_values[i],
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
            m.source_color.is_some(),
        );
        if m.source_dirty || self.source_configured_dirty || self.source_key != Some(source_key) {
            self.source_color.set_pitch(m.params.sample_root_note);
            self.source_color.configure_tempo(
                m.source_color
                    .or_else(|| self.preset.source_color_chain.first().copied())
                    .unwrap_or(ColorSlot {
                        enabled: false,
                        ..ColorSlot::default()
                    }),
                m.params.bpm,
            );
            self.source_key = Some(source_key);
            self.source_configured_dirty = m.source_dirty;
        }
        let phase_capable = self.phase_capabilities(&m);
        self.oscillator_engine.graph.configure_routes(
            &m.interaction_controls.routes[..m.interaction_controls.count],
            self.preset.oscillators.as_deref(),
            phase_capable,
        );
        self.configure_resynth_interactions();
        self.oscillator_engine.resynth.configure(
            m.resynth,
            &self.sample,
            m.params.sample_root_note,
        );
        // Outside advanced mode the grain groups keep their taps closed.
        let group_taps = if m.grain_groups.advanced {
            m.grain_groups.groups.map(|g| g.output)
        } else {
            Default::default()
        };
        self.taps.configure_filters(&m.filter_slots[..m.filter_len]);
        self.taps.configure(&m.taps, group_taps);
        self.grain_feedback.tidy(m.params.grain_feedback);
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
        if let Some(synth) = &mut self.oscillator_engine.synth {
            synth.configure(&m.synth, tune, &self.sample, m.params.sample_root_note);
        }
        if let Some(wavetable) = &mut self.oscillator_engine.wavetable {
            wavetable.configure(m.wavetable, tune);
            wavetable.configure_motion(m.wavetable_motion);
        }
        if let Some(mixed) = &mut self.oscillator_engine.mixed {
            mixed.configure(m.oscillator_controls, tune);
            mixed.configure_motion(m.oscillator_motion);
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
            if *env != self.modulated.envs[i]
                || (env.wave.enabled && m.params.bpm != self.modulated.params.bpm)
            {
                for voice in self.voices.iter_mut() {
                    voice.envelopes[i].configure_tempo(self.sample_rate as f32, env, m.params.bpm);
                }
            }
        }
        let slots = &m.filter_slots[..m.filter_len];
        self.filter_masks = filter_slots::masks(slots, &self.filter_banks);
        self.filter_fx_masks = filter_slots::fx_masks(
            slots,
            self.filter_masks[FilterPosition::AfterFx as usize],
            [
                &m.pre_color_slots[..m.pre_color_len],
                &m.color_slots[..m.color_len],
            ],
        );
        let last_note = self.source_note as f32
            + self.bend[self.source_channel as usize] * m.params.midi_pitchbend_range;
        let bass_env = self.tb303_view().envelope;
        let shared_envs = filter::envelopes(self.env_values, bass_env);
        // Any added filter of a Rack may filter a chain as one.
        filter_slots::configure_shared(
            slots,
            &mut self.filter_banks,
            last_note,
            shared_envs,
            self.rack.is_some(),
        );
        let bus = self.preset.bus.is_some();
        let chain_filters = self.rack.is_some_and(|r| !r.filters_per_note);
        if chain_filters {
            // Without notes of its own, a chain's filters follow each
            // envelope's loudest voice and the last note, as added ones do.
            self.filters[0].configure(&m.params, last_note, shared_envs);
        }
        for (i, voice) in self.voices.iter().enumerate() {
            if chain_filters || (!voice.active() && !(bus && i == 0)) {
                continue;
            }
            let note =
                voice.pitch() + self.bend[voice.channel as usize] * m.params.midi_pitchbend_range;
            let envs = filter::envelopes(voice.env_values, self.voice_tb303_envelope(i));
            self.filters[i].configure(&m.params, note, envs);
            filter_slots::configure_voice(slots, &mut self.filter_banks, i, note, envs);
        }
        if let Some(rack) = &mut self.rack {
            rack.retarget(&m.rack_strips);
            self.equalizers.retarget(&m.rack_eqs);
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
        self.sync_main_pitch_voices(false);
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
        let mut result = crate::resynth::Meter {
            bands: self.oscillator_engine.resynth.carrier_levels(),
            frequencies: self.oscillator_engine.resynth.frequencies(),
        };
        if self.main_pitch_sequence()
            && let Some(sources) = &self.preset_runtime.sequence_sources
        {
            for (index, voice) in self.voices.iter().enumerate() {
                if !voice.active() {
                    continue;
                }
                if let Some(meter) = sources.resynth_meter(index) {
                    for band in 0..result.bands.len() {
                        if meter.bands[band] > result.bands[band] {
                            result.bands[band] = meter.bands[band];
                            result.frequencies[band] = meter.frequencies[band];
                        }
                    }
                }
            }
        }
        result
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
        let note = voice.pitch()
            + self.bend[voice.channel as usize] * self.modulated.params.midi_pitchbend_range;
        let envs = filter::envelopes(voice.env_values, self.voice_tb303_envelope(index));
        if !self.rack.is_some_and(|r| !r.filters_per_note) {
            self.filters[index].configure(&self.modulated.params, note, envs);
        }
        filter_slots::configure_voice(
            &self.modulated.filter_slots[..self.modulated.filter_len],
            &mut self.filter_banks,
            index,
            note,
            envs,
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
            .enumerate()
            .find(|(_, voice)| voice.active() && voice.note == note)
            .map(|(index, voice)| {
                let pitch = voice.pitch()
                    + self.bend[voice.channel as usize]
                        * self.modulated.params.midi_pitchbend_range;
                filter::voice_cutoffs(
                    &self.modulated.params,
                    pitch,
                    filter::envelopes(voice.env_values, self.voice_tb303_envelope(index)),
                )
            })
    }

    /// Synth grains read the mixed history after every voice's VCA: they
    /// belong to no note and keep its tails while playing. So do a Rack's
    /// that read their chain rather than the file, transport or not.
    fn synth_grains(&self) -> bool {
        match &self.rack {
            Some(rack) => !rack.reads_file,
            None => self.oscillator_engine.synth.is_some(),
        }
    }

    /// A generated-note history is already pitched; a shared drum, sample or
    /// vocal stream still needs the party's pitch on its grain playback.
    fn individual_history(&self) -> bool {
        self.rack
            .as_ref()
            .map_or(self.oscillator_engine.synth.is_some(), |rack| {
                rack.history_pitched
            })
    }

    fn main_pitch_sequence(&self) -> bool {
        (self.preset.sequencer.enabled
            && self.sequence_running
            && self
                .preset
                .sequencer
                .tracks
                .iter()
                .any(|t| t.target == crate::sequence::Target::MainPitch))
            || self.voices.iter().any(|v| v.main_pitch && v.active())
    }

    fn grain_voice(&mut self, synth: bool) -> Option<usize> {
        if synth && !self.main_pitch_sequence() {
            return (self.playing || self.rack.is_some()).then_some(0);
        }
        let bus = self.preset.bus.is_some();
        let legacy = synth && self.voices.iter().any(|v| !v.main_pitch && v.active());
        let voice = (0..VOICES)
            .map(|i| (i + self.next_voice) % VOICES)
            .find(|&i| {
                let v = &self.voices[i];
                (synth && i == 0 && legacy)
                    || ((v.active() || (bus && i == 0)) && v.granular() && (!synth || v.main_pitch))
            })?;
        self.next_voice = (voice + 1) % VOICES;
        Some(voice)
    }

    /// History grains may be shared post-VCA grains or a Main pitch party's
    /// own pitched/gated cloud. File grains always follow their voice's VCA.
    fn mixed_grains(&self, frames: &[Stereo; VOICES], history: bool) -> Stereo {
        let mut output = [0.0; 2];
        for (i, frame) in frames.iter().enumerate() {
            let weight = if history && !self.voices[i].main_pitch {
                f32::from(i == 0)
            } else {
                self.voices[i].value.max(0.0)
            };
            for c in 0..2 {
                output[c] += frame[c] * weight;
            }
        }
        output
    }

    /// Capture each party before its VCA. Notes of the same party share their
    /// evolving tape; weighting and normalizing their overlap keeps the VCA
    /// from being applied twice when the grains are played.
    fn capture_sequence_histories(
        &mut self,
        frames: &[Stereo; VOICES],
        grains: &[Stereo; VOICES],
        norm: f32,
    ) {
        let Some(live) = &self.live_input else {
            return;
        };
        let time = live.range_epoch() + live.range().end as u64;
        let Some(time) = time.checked_sub(1) else {
            return;
        };
        let owners: [u32; VOICES] = std::array::from_fn(|i| {
            let v = self.voices[i];
            if v.main_pitch && (v.held || v.active()) {
                v.owner
            } else {
                0
            }
        });
        let Some(histories) = &mut self.preset_runtime.sequence_histories else {
            return;
        };
        histories.begin_frame();
        for i in bits(self.sounding) {
            let voice = self.voices[i];
            if voice.main_pitch && voice.granular {
                histories.add(
                    voice.owner,
                    &owners,
                    frames[i],
                    grains[i],
                    voice.value.max(0.0),
                );
            }
        }
        histories.finish_frame(
            time,
            self.frozen,
            self.modulated.params.grain_feedback,
            norm,
        );
    }

    fn spawn(&mut self) {
        // Lowest free slot, as the previous linear scan chose.
        if (!self.live_grains).trailing_zeros() as usize == MAX_GRAINS {
            return;
        }
        let synth = self.synth_grains();
        let Some(voice) = self.grain_voice(synth) else {
            return;
        };
        self.spawn_grain(voice, synth, 0);
    }

    fn spawn_group(&mut self, group: usize, forced: bool, time: f64) {
        let synth = self.synth_grains();
        let Some(voice) = self.grain_voice(synth) else {
            return;
        };
        self.spawn_grain_group(voice, synth, 0, Some(group), forced, time);
    }
    fn schedule_grains(&mut self, enabled: bool) {
        self.grain_clock += self.grain_step;
        let clock_routes = self.modulated.interaction_controls.routes
            [..self.modulated.interaction_controls.count]
            .iter()
            .any(|r| r.needs_group_clock());
        if (self.preset.grain_scheduler != crate::interaction::Scheduler::Independent
            && !clock_routes)
            || !self.modulated.grain_groups.advanced
        {
            if enabled && self.grain_density > 0.0 && self.grain_clock >= 1.0 {
                self.grain_clock = self.grain_clock.fract();
                self.spawn();
            }
            return;
        }
        let groups = self.modulated.grain_groups.groups;
        let total = groups.iter().map(|g| g.share).sum::<f32>();
        for (i, group) in groups.iter().enumerate() {
            if !enabled || self.grain_density <= 0.0 || group.share <= 0.0 || total <= 0.0 {
                continue;
            }
            let command = self
                .oscillator_engine
                .graph
                .input(25 + i, interaction::GLOBAL)
                .clock;
            let clock = &mut self.group_clocks[i];
            clock
                .slew
                .correct(clock.phase, command, self.sample_rate as f64);
            let step = self.grain_step * (group.share / total) as f64 + clock.slew.next();
            let hard = command.mode == crate::interaction::SyncMode::Hard
                && command.event >= 0.0
                && command.strength > 0.0;
            let old = clock.phase;
            clock.phase = if hard {
                (command.phase + step * (1.0 - command.event)).rem_euclid(1.0)
            } else {
                (old + step).rem_euclid(1.0)
            };
            let wrap = !hard && old + step >= 1.0;
            if wrap {
                let at = ((1.0 - old) / step).clamp(0.0, 1.0);
                self.oscillator_engine.graph.event(
                    25 + i,
                    interaction::GLOBAL,
                    crate::interaction::Event::GroupClockWrap,
                    at,
                    false,
                );
            }
            if hard || wrap {
                let at = if hard {
                    command.event
                } else {
                    ((1.0 - old) / step).clamp(0.0, 1.0)
                };
                self.spawn_group(i, hard, at);
            }
        }
    }

    /// Start a grain of `voice` as it would be `age` frames after it began.
    fn spawn_grain(&mut self, voice: usize, synth: bool, age: u32) {
        self.spawn_grain_group(voice, synth, age, None, false, 0.0);
    }
    fn spawn_grain_group(
        &mut self,
        voice: usize,
        synth: bool,
        age: u32,
        chosen: Option<usize>,
        forced: bool,
        birth_time: f64,
    ) {
        if !self.voices[voice].granular() {
            return;
        }
        let slot = (!self.live_grains).trailing_zeros() as usize;
        if slot == MAX_GRAINS {
            if chosen.is_some() {
                self.oscillator_engine.graph.dropped_births += 1;
            }
            return;
        }
        // Since it began, the grain moved at its rate and the centre it
        // started from at the cursor's.
        let drift_speed = if age > 0 { self.center_speed() } else { 0.0 };
        let groups = &self.modulated.grain_groups;
        let group = if groups.advanced {
            let Some(index) = chosen.or_else(|| self.group_grains.deal(groups)) else {
                return;
            };
            // Explicit Clock/prefill births obey the same allocation gate as
            // the dealer, including when every group is Off.
            if groups.groups[index].share <= 0.0 {
                return;
            }
            Some((index, groups.groups[index]))
        } else {
            None
        };
        // A group at 100% ATTACK grows from none: it plays no grains in.
        if age > 0 && group.is_some_and(|(_, g)| g.attack >= 1.0) {
            return;
        }
        let p = &self.modulated.params;
        let note = self.voices[voice];
        let individual = synth && note.main_pitch && self.individual_history();
        let bend = if synth && (voice == 0 || individual) {
            0.0
        } else {
            self.bend[note.channel as usize] * p.midi_pitchbend_range
        };
        // A Synth's or drum kit's history is already tuned.
        let (tune_semis, tune_cents) = if synth || (self.drums.is_some() && self.rack.is_none()) {
            (0.0, 0.0)
        } else {
            (p.sample_tune_semis, p.sample_tune_cents)
        };
        let note_semis = if voice == 0 || individual {
            0.0
        } else {
            note.pitch() - p.sample_root_note
        };
        let semis = match group {
            None => {
                (p.pitch * 2.0 - 1.0) * p.pitch_range
                    + note_semis
                    + tune_semis
                    + tune_cents * 0.01
                    + bend
                    + self.random.unit() * tables::detune_range(p.detune)
            }
            Some((_, g)) => {
                // A group's offset from its note, random pitch and DETUNE
                // included, snaps to its scale before the note's transposition.
                let mut offset = (p.pitch * 2.0 - 1.0) * p.pitch_range
                    + g.pitch
                    + self.random.unit() * tables::detune_range(p.detune);
                if g.random_pitch > 0.0 {
                    offset += self.group_grains.random.bipolar() * g.random_pitch;
                }
                let intervals = match g.scale {
                    grain_groups_settings::Scale::Chord => Some(self.held_intervals(voice)),
                    scale => scale.intervals(),
                };
                if let Some(intervals) = intervals {
                    offset = grain_groups_settings::quantize(offset, intervals);
                }
                offset + note_semis + tune_semis + tune_cents * 0.01 + bend
            }
        };
        let range = self.source_range();
        let length = range.len() as f64;
        let forward = p.grain_size >= 0.5;
        // A group plays some of its grains the other way.
        let forward = match group {
            Some((_, g)) if g.reverse > 0.0 && self.group_grains.random.unit() < g.reverse => {
                !forward
            }
            _ => forward,
        };
        let rate = tables::pitch_ratio(semis) * self.source_sample().sample_rate as f64
            / self.sample_rate as f64
            * if forward { 1.0 } else { -1.0 };
        let seconds = self.layer_seconds(group.map(|(_, g)| g.size));
        let duration = (seconds * self.sample_rate as f32).round().max(4.0) as u32;
        if age >= duration {
            return;
        }
        let center = match group {
            Some((_, g)) => self.layer_center(voice, g.position),
            None => self.voice_center(voice),
        };
        let scatter = self.random.bipolar() as f64;
        let mut offset = center - range.start as f64
            + if p.cloud_time {
                scatter * self.cloud_width() * 0.5
            } else {
                scatter * p.cloud as f64 * length * 0.5
            };
        if age > 0 {
            offset += (rate - drift_speed) * age as f64;
        }
        let position = range.start as f64 + offset.rem_euclid(length);
        let spread = group.map_or(p.stereo_spread, |(_, g)| {
            (p.stereo_spread + g.spread).clamp(0.0, 1.0)
        });
        let pan = (self.modulated.pan + self.random.bipolar() * spread * 0.5).clamp(0.0, 1.0);
        let shape = group.map_or(p.grain_shape, |(_, g)| g.shape);
        match group {
            Some((index, g)) => {
                // The pitch the grain plays, which its waveshaper follows: a
                // Synth's history and a bus play at the latest note.
                let base = if voice == 0 {
                    self.source_note as f32
                } else {
                    p.sample_root_note
                };
                self.group_grains
                    .start(slot, index, &g, duration, base + semis)
            }
            None => self.group_grains.release(slot),
        }
        let history = if individual {
            let Some(index) = self
                .preset_runtime
                .sequence_histories
                .as_ref()
                .and_then(|h| h.slot(note.owner))
            else {
                return;
            };
            Some(index as u8)
        } else {
            None
        };
        self.live_grains |= 1 << slot;
        let mip = match grain_quality() {
            GrainQuality::High => self.source_sample().mip_for_rate(rate, true),
            GrainQuality::Eco => Mip::flat(self.source_sample().level_for_rate(rate, true)),
        };
        if mip.is_wide() {
            self.wide_grains |= 1 << slot;
        } else {
            self.wide_grains &= !(1 << slot);
        }
        self.grain_interaction.reset(slot);
        self.grains[slot] = Grain {
            position,
            anchor: range.start as f64
                + (position - rate * age as f64 - range.start as f64).rem_euclid(length),
            playback_slew: Default::default(),
            playback_phase: age as f64 / duration.max(1) as f64,
            birth_time,
            rate,
            mip,
            age,
            duration,
            shape: shape.clamp(0.0, 1.0),
            pan: tables::pan(pan),
            voice,
            history,
        };
        if let Some((group, _)) = group {
            self.oscillator_engine.graph.event(
                25 + group,
                interaction::GLOBAL,
                crate::interaction::Event::GrainLaunch,
                birth_time,
                forced,
            );
        }
    }

    /// How far the grains' centre moves each frame: the cursor's step, a bar
    /// loop's or the history head's, none while frozen or stopped on a file.
    fn center_speed(&self) -> f64 {
        if self.frozen || (self.live_input.is_none() && !self.playing) {
            0.0
        } else if self.bar_loop_active() {
            self.file_range.len() as f64 * self.bar_phase_step
        } else {
            self.source_sample().sample_rate as f64 / self.sample_rate as f64
        }
    }

    /// A grain's seconds at the engine's SIZE, made `octaves` longer for a
    /// group's SIZE but never longer than the engine's longest.
    fn layer_seconds(&self, octaves: Option<f32>) -> f32 {
        let size = self.modulated.params.grain_size;
        match octaves {
            Some(octaves) if octaves != 0.0 => {
                (self.grain_seconds(size) * octaves.exp2()).min(self.grain_seconds(1.0))
            }
            _ => self.grain_seconds(size),
        }
    }

    /// Below 100% ATTACK a new note starts with the grains it would hold
    /// after playing a while, its share of the cloud dealt at the grain rate,
    /// and fades in over ATTACK × 40% of a grain, which near 100% takes as
    /// long as a cloud growing from nothing: at 0% the cloud sounds whole at
    /// once and ENV1 alone shapes the attack. At 100% grains build up from
    /// none, as they always did. Played in, the cloud skips the level bump a
    /// growing one has while all its grains are young. In advanced mode each
    /// group does so at its own ATTACK, over its own grains' length.
    fn prefill_grains(&mut self, voice: usize) {
        const UNGROUPED: usize = crate::grain_groups::GROUPS;
        let mut attacks = [1.0_f32; UNGROUPED + 1];
        let mut seconds = [0.0_f32; UNGROUPED + 1];
        let groups = &self.modulated.grain_groups;
        if groups.advanced {
            for (k, g) in groups.groups.iter().enumerate() {
                if g.share > 0.0 {
                    attacks[k] = g.attack.max(0.0);
                    seconds[k] = self.layer_seconds(Some(g.size));
                }
            }
        } else {
            attacks[UNGROUPED] = self.modulated.params.grain_attack.max(0.0);
            seconds[UNGROUPED] = self.layer_seconds(None);
        }
        if attacks.iter().all(|&a| a >= 1.0)
            || !self.voices[voice].granular()
            || self.grain_density <= 0.0
            || self.synth_grains()
            || self.preset.bus.is_some()
            || self.live_input.as_ref().is_some_and(|i| i.effects_only)
            // A sounding voice keeps the grains it has.
            || self.live_grain_slots().any(|slot| self.grains[slot].voice == voice)
        {
            return;
        }
        let sr = self.sample_rate as f64;
        // The round robin deals each sounding voice its share of the grains.
        let voices = self.voices.iter().filter(|v| v.active()).count().max(1);
        let interval = voices as f64 * sr / self.grain_density as f64;
        let longest = (0..=UNGROUPED)
            .filter(|&k| attacks[k] < 1.0)
            .map(|k| seconds[k])
            .fold(0.0, f32::max) as f64
            * sr;
        // The newest grain, at age 0, is the one the grain clock spawns next.
        // Older ones than their own length died already and are left out.
        let empty = self.live_grains == 0;
        let mut n = 1;
        while n as f64 * interval < longest && self.live_grains != u128::MAX {
            // Whole refresh steps, so a group's filters tune on their first frame.
            let age = (n as f64 * interval) as u32 / grain_groups::REFRESH * grain_groups::REFRESH;
            self.spawn_grain(voice, false, age);
            n += 1;
        }
        // The overlap level follows at once, or the cloud would enter loud.
        self.overlap = self.overlap.max(self.active_grains() as f32);
        if empty {
            // There is no older cloud to declick. Start a prefilled cloud at
            // its own level, without a gain swell during the note's attack.
            self.smooth_grain_norm = self.grain_norm_target(false);
        }
        let v = &mut self.voices[voice];
        let mut fading = false;
        for k in 0..=UNGROUPED {
            let frames = if attacks[k] < 1.0 {
                attacks[k] as f64 * seconds[k] as f64 * sr * 0.4
            } else {
                0.0
            };
            if frames >= 1.0 {
                v.onset[k] = 0.0;
                v.onset_step[k] = (1.0 / frames) as f32;
                fading = true;
            } else {
                v.onset[k] = 1.0;
            }
        }
        if fading {
            self.onsets |= 1 << voice;
        } else {
            self.onsets &= !(1 << voice);
        }
    }

    /// The held notes as intervals above `voice`'s note (above the lowest
    /// held note for the free voice), bit n for n semitones; the note itself
    /// alone when nothing is held.
    fn held_intervals(&self, voice: usize) -> u16 {
        let held = self.voices[1..].iter().filter(|v| v.held || v.sustained);
        let base = if voice == 0 {
            held.clone().map(|v| v.note).min()
        } else {
            Some(self.voices[voice].note)
        };
        let Some(base) = base else {
            return 1;
        };
        held.fold(1, |mask, v| {
            mask | 1 << (v.note as i32 - base as i32).rem_euclid(12)
        })
    }

    /// Sequencer steps and arpeggiator notes for this frame.
    fn tick_events(&mut self) {
        let mut control_tick = false;
        for input in &mut self.control_inputs {
            if let Some(runtime) = &mut input.runtime {
                control_tick |= runtime.tick(
                    self.sample_rate,
                    input.bpm,
                    input.program.enabled && self.sequence_running,
                );
            }
        }
        if control_tick {
            self.update_modulation();
        }
        let tick = self.preset_runtime.sequence.as_mut().is_some_and(|r| {
            r.tick(
                self.sample_rate,
                self.preset.params.bpm,
                self.preset.sequencer.enabled && self.sequence_running,
            )
        });
        if tick {
            let count = self.preset_runtime.sequence.as_ref().map_or(0, |r| r.len());
            for index in 0..count {
                let (id, actions) = self
                    .preset_runtime
                    .sequence
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
                            if !self.preset.tb303.as_ref().is_some_and(|s| s.pattern)
                                && let Some(bass) = &mut self.oscillator_engine.tb303
                            {
                                bass.legato(channel, note, velocity, id);
                                if self.rack.is_none() {
                                    continue;
                                }
                            }
                            // A kit has no voice to bend: the slide replays the pad.
                            if let (Some(sampler), Some(settings)) =
                                (&mut self.sampler, &self.preset.sampler)
                            {
                                sampler.release_owner(settings, id);
                            }
                            // A Rack's voices slide while its kit replays.
                            let slides = self.sampler.is_none() || self.rack.is_some();
                            if let Some(voice) = self
                                .voices
                                .iter_mut()
                                .find(|v| v.owner == id && v.held && slides)
                            {
                                voice.note = note;
                                voice.channel = channel;
                                voice.velocity = velocity as f32 / 127.0;
                                self.source_note = note;
                                self.source_channel = channel;
                                if self.sampler.is_some() {
                                    self.sampler_midi(
                                        Message::NoteOn {
                                            channel,
                                            note,
                                            velocity,
                                        },
                                        id,
                                    );
                                }
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
            let resets = self
                .preset_runtime
                .sequence
                .as_ref()
                .map_or(0, |r| r.lfo_resets());
            for (i, (oscillator, lfo)) in self
                .oscillators
                .iter_mut()
                .zip(&self.preset.lfos)
                .enumerate()
            {
                if resets & 1 << i != 0 && lfo.reset == LfoReset::Sequencer {
                    oscillator.reset();
                    for voice in self.voice_oscillators.iter_mut() {
                        voice[i].reset();
                    }
                }
            }
            self.sync_main_pitch_voices(true);
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
        // A Rack's drums sound without notes or transport.
        if !self.playing && self.rack.is_none() {
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
    /// the message is used up; pitch bend and controllers continue. In a
    /// Rack the notes and the pedal go on to its voices too: only a pad's
    /// own track is the kit's alone.
    fn sampler_midi(&mut self, message: Message, owner: u32) -> bool {
        let (Some(sampler), Some(settings)) = (&mut self.sampler, &self.preset.sampler) else {
            return false;
        };
        let alone = self.rack.is_none();
        match message {
            Message::NoteOn {
                channel,
                note,
                velocity,
            } if channel < 16 && note < 128 && velocity < 128 => {
                let sustain = owner == 0 && self.sustain[channel as usize];
                if velocity == 0 {
                    // In a Rack it comes back as a note off.
                    if alone {
                        sampler.note_off(settings, channel, note, owner, sustain);
                    }
                    return alone;
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
                if !alone && pad.is_none() {
                    // The voice that starts next notes the modulation source.
                    return false;
                }
                self.mod_note_on(channel, note, velocity);
                true
            }
            Message::NoteOff { channel, note, .. } if channel < 16 => {
                let sustain = owner == 0 && self.sustain[channel as usize];
                sampler.note_off(settings, channel, note, owner, sustain);
                alone
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
                alone
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
    /// Installs a kit runtime prepared off the audio thread, for a changed
    /// pool or Pad FX routes: the sounding voices carry over, and the
    /// replaced runtime comes back in `runtime` for the caller to free. A
    /// preset without a kit leaves it there unused.
    pub fn exchange_sampler(&mut self, runtime: &mut Option<Box<sampler::Runtime>>) {
        if self.preset.sampler.is_none() {
            return;
        }
        if let (Some(next), Some(old)) = (runtime.as_deref_mut(), self.sampler.as_deref_mut()) {
            next.inherit(old);
        }
        std::mem::swap(&mut self.sampler, runtime);
    }
    /// `sampler_view` without allocating: the pool ids stay with the host.
    pub fn sampler_snapshot(&self, view: &mut sampler::Snapshot) {
        match &self.sampler {
            Some(sampler) => sampler.snapshot(view),
            None => {
                view.voice_count = 0;
                view.pad_count = 0;
            }
        }
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

    pub fn tb303_view(&self) -> crate::tb303::View {
        let legacy = self
            .oscillator_engine
            .tb303
            .as_ref()
            .map_or_else(Default::default, |b| b.view());
        if legacy.gate {
            return legacy;
        }
        self.preset_runtime
            .sequence_sources
            .as_ref()
            .and_then(|sources| {
                self.voices
                    .iter()
                    .enumerate()
                    .filter(|(_, v)| v.main_pitch && v.active())
                    .filter_map(|(index, v)| sources.tb303_view(index).map(|view| (v.value, view)))
                    .max_by(|a, b| a.0.total_cmp(&b.0))
                    .map(|(_, view)| view)
            })
            .unwrap_or(legacy)
    }

    /// Each sequence party has its own 303 filter-envelope state.
    fn voice_tb303_envelope(&self, index: usize) -> f32 {
        if self.voices[index].main_pitch
            && let Some(view) = self
                .preset_runtime
                .sequence_sources
                .as_ref()
                .and_then(|sources| sources.tb303_view(index))
        {
            return view.envelope;
        }
        self.oscillator_engine
            .tb303
            .as_ref()
            .map_or(0.0, |bass| bass.view().envelope)
    }

    fn render_main_tb303(&mut self, index: usize) -> Stereo {
        let v = self.voices[index];
        let Some(settings) = self.preset.tb303.as_deref() else {
            return [0.0; 2];
        };
        let Some(sources) = &mut self.preset_runtime.sequence_sources else {
            return [0.0; 2];
        };
        let graph = &mut self.oscillator_engine.graph;
        if v.main_pitch {
            graph.prepare_sequence_voice(index, v.generation);
        } else {
            graph.prepare_voice(index, v.generation);
        }
        let demand = ((graph.demanded | graph.targets) >> 20) as u8 & 31;
        sources.configure_tb303_interactions(
            index,
            v.generation,
            graph.input(24, index),
            std::array::from_fn(|i| graph.input(20 + i, index)),
            demand,
        );
        sources.set_tb303_drone(index, v.main_pitch && !v.env_gate);
        let output = sources.render_tb303(
            index,
            v.generation,
            settings,
            self.modulated.tb303,
            &self.modulated.tb303_radias,
            v.pitch()
                + self.modulated.params.sample_tune_semis
                + self.modulated.params.sample_tune_cents * 0.01,
            v.held,
            (v.velocity * 127.0).round() as u8,
            ((v.oscillator_mask() >> 20) & 31) as u8,
            self.voice_lfo_values[index][0],
            self.modulated.params.bpm,
        );
        if demand != 0
            && let Some((vco, layers)) = sources.tb303_frames(index)
        {
            for (node, frame) in std::iter::once((24, vco))
                .chain(layers.into_iter().enumerate().map(|(i, f)| (20 + i, f)))
            {
                graph.publish(node, index, frame.generation, frame.output, v.value);
                if let Some(at) = frame.wrap {
                    graph.natural_event(node, index, at, frame.latency);
                }
            }
        }
        output
    }

    fn render_main_resynth(
        &mut self,
        index: usize,
        dry: Stereo,
        source: Stereo,
        grains: Stereo,
        source_pitched: bool,
    ) -> Stereo {
        let v = self.voices[index];
        let Some(sources) = &mut self.preset_runtime.sequence_sources else {
            return dry;
        };
        let masks = self.filter_masks;
        let graph = &mut self.oscillator_engine.graph;
        if v.main_pitch {
            graph.prepare_sequence_voice(index, v.generation);
        } else {
            graph.prepare_voice(index, v.generation);
        }
        let demand = ((graph.demanded | graph.targets) >> 16) as u8 & 15;
        sources.configure_resynth_interactions(
            index,
            v.generation,
            std::array::from_fn(|i| graph.input(16 + i, index)),
            demand,
        );
        let processed = sources.render_resynth(
            index,
            v.generation,
            self.modulated.resynth,
            if source_pitched || self.modulated.resynth.input == crate::resynth::Input::Grains {
                0.0
            } else {
                v.pitch()
                    + self.bend[v.channel as usize] * self.modulated.params.midi_pitchbend_range
                    - self.modulated.params.sample_root_note
            },
            ((v.oscillator_mask() >> 16) & 15) as u8,
            dry,
            source,
            grains,
            self.voice_lfo_values[index][0],
            1.0,
            &self.sample,
            &self.file_range,
            self.modulated.params.sample_root_note,
            self.playing,
            [
                masks[FilterPosition::AfterOsc1 as usize],
                masks[FilterPosition::AfterOsc2 as usize],
                masks[FilterPosition::AfterOsc3 as usize],
                masks[FilterPosition::AfterOsc4 as usize],
            ],
            &mut self.filter_banks,
            Some((&mut self.taps, v.value)),
        );
        if demand != 0
            && let Some(layers) = sources.resynth_frames(index)
        {
            for (layer, frames) in layers.iter().enumerate() {
                let mut generation = [0.0; 2];
                let mut output = [0.0; 2];
                for frame in frames {
                    for c in 0..2 {
                        generation[c] += frame.generation[c] / crate::resynth::BANDS as f32;
                        output[c] += frame.output[c] / crate::resynth::BANDS as f32;
                    }
                    if let Some(at) = frame.wrap {
                        graph.natural_event(16 + layer, index, at, frame.latency);
                    }
                }
                graph.publish(16 + layer, index, generation, output, v.value);
            }
        }
        let processed = filter_slots::route_chain(
            &mut self.filter_banks,
            masks[FilterPosition::AfterResynth as usize],
            processed,
            Instance::Voice(index),
            Some((&mut self.taps, v.value)),
        );
        if self.taps.sends(Stage::Resynth) {
            self.taps
                .send(Stage::Resynth, processed.map(|x| x * v.value));
        }
        let latency = filter_slots::latency_samples(
            &self.filter_banks,
            masks[FilterPosition::AfterResynth as usize],
            Instance::Voice(index),
        );
        let aligned = self.filters[index].align_resynth_input(dry, latency);
        self.taps.mix(Stage::Resynth, aligned, processed)
    }

    fn render_tb303(&mut self) -> Stereo {
        let sequenced = self.main_pitch_sequence();
        if let (Some(bass), Some(settings)) =
            (&mut self.oscillator_engine.tb303, &self.preset.tb303)
        {
            let graph = &mut self.oscillator_engine.graph;
            let enabled = self.preset.rack.as_ref().is_none_or(|r| {
                r.modules
                    .iter()
                    .any(|s| s.module == crate::rack::Module::Tb303 && s.enabled)
            });
            let mask = if enabled {
                ((graph.demanded | graph.targets) >> 20) as u8 & 31
            } else {
                0
            };
            bass.configure_interactions(
                if enabled {
                    graph.input(24, interaction::GLOBAL)
                } else {
                    Default::default()
                },
                std::array::from_fn(|i| {
                    if enabled {
                        graph.input(20 + i, interaction::GLOBAL)
                    } else {
                        Default::default()
                    }
                }),
                mask,
            );
            bass.configure(
                &self.modulated.tb303_radias,
                self.modulated.params.sample_tune_semis
                    + self.modulated.params.sample_tune_cents * 0.01,
                self.lfo_values[0],
            );
            let output = bass.frame_interacting(
                settings,
                self.modulated.tb303,
                self.playing && self.sequence_running && !sequenced,
                self.preset.params.bpm,
                self.oscillator_engine.phase_adapters.as_deref_mut(),
                self.frame,
            );
            if mask != 0 {
                let (vco, layers) = bass.interaction_frames();
                for (node, frame) in std::iter::once((24, vco))
                    .chain(layers.into_iter().enumerate().map(|(i, f)| (20 + i, f)))
                {
                    graph.publish(
                        node,
                        interaction::GLOBAL,
                        frame.generation,
                        frame.output,
                        1.0,
                    );
                    if let Some(at) = frame.wrap {
                        graph.natural_event(node, interaction::GLOBAL, at, frame.latency);
                    }
                }
            }
            output
        } else {
            [0.0; 2]
        }
    }
    fn phase_capabilities(&self, m: &Modulated) -> u32 {
        use crate::resynth::Waveform;
        let slots = self.preset.oscillators.as_deref();
        let resets = m.interaction_controls.routes[..m.interaction_controls.count]
            .iter()
            .filter(|r| {
                r.enabled
                    && r.sync != crate::interaction::SyncMode::Off
                    && r.scope != crate::interaction::Scope::Clock
            })
            .filter_map(|r| r.target.resolve(slots))
            .fold(0u32, |mask, node| mask | (1 << node));
        let native = |node: usize, o: crate::resynth::Oscillator| {
            o.waveform.analog()
                || o.waveform == Waveform::Pcm
                || o.transport.enabled
                || resets & (1 << node) != 0
        };
        let mut mask = 1u32 << 24;
        for i in 0..crate::oscillators::SLOTS {
            if let Some(s) = slots
                && s.slots[i].id != 0
                && (s.slots[i].kind != crate::oscillators::Kind::Radias
                    || native(i, m.oscillator_controls[i].native))
            {
                mask |= 1 << i;
            }
        }
        for i in 0..4 {
            if native(8 + i, m.synth.oscillators[i]) {
                mask |= 1 << (8 + i);
            }
            mask |= 1 << (12 + i);
            if native(16 + i, m.resynth.oscillators[i]) {
                mask |= 1 << (16 + i);
            }
            if native(20 + i, m.tb303_radias.oscillators[i]) {
                mask |= 1 << (20 + i);
            }
        }
        mask
    }
    fn render_synth_voice(&mut self, index: usize) -> Stereo {
        let voice = &self.voices[index];
        let note = voice.pitch()
            + self.bend[voice.channel as usize] * self.modulated.params.midi_pitchbend_range;
        let owner = &mut self.oscillator_engine;
        let Some(synth) = &mut owner.synth else {
            return [0.0; 2];
        };
        if voice.main_pitch {
            owner.graph.prepare_sequence_voice(index, voice.generation);
        } else {
            owner.graph.prepare_voice(index, voice.generation);
        }
        let demand = ((owner.graph.demanded | owner.graph.targets) >> 8) as u8 & 15;
        let inputs =
            (demand != 0).then(|| std::array::from_fn(|slot| owner.graph.input(8 + slot, index)));
        let transport = owner
            .phase_adapters
            .as_deref_mut()
            .map(|p| (p, 8, index, self.frame));
        let output = synth.render_interacting_masked(
            index,
            voice.generation,
            note,
            &self.sample,
            &self.file_range,
            inputs,
            demand,
            transport,
            ((voice.oscillator_mask() >> 8) & 15) as u8,
        );
        if demand != 0 {
            for (slot, frame) in synth.interaction_frames().into_iter().enumerate() {
                owner
                    .graph
                    .publish(8 + slot, index, frame.generation, frame.output, voice.value);
                if let Some(at) = frame.wrap {
                    owner
                        .graph
                        .natural_event(8 + slot, index, at, frame.latency);
                }
            }
        }
        output
    }
    fn render_wavetable_voice(&mut self, index: usize) -> Stereo {
        let voice = &self.voices[index];
        let note = voice.pitch()
            + self.bend[voice.channel as usize] * self.modulated.params.midi_pitchbend_range;
        let owner = &mut self.oscillator_engine;
        if voice.main_pitch {
            owner.graph.prepare_sequence_voice(index, voice.generation);
        } else {
            owner.graph.prepare_voice(index, voice.generation);
        }
        let Some(wavetable) = &mut owner.wavetable else {
            return [0.0; 2];
        };
        let demand = if wavetable.powered() {
            ((owner.graph.demanded | owner.graph.targets) >> 12) as u8 & 15
        } else {
            0
        };
        let external = (demand != 0).then(|| {
            (
                std::array::from_fn(|s| owner.graph.input(12 + s, index)),
                demand,
            )
        });
        let output = wavetable.render_interacting_masked(
            index,
            voice.generation,
            note,
            &voice.env_values,
            &self.voice_lfo_values[index],
            external,
            ((voice.oscillator_mask() >> 12) & 15) as u8,
        );
        if demand != 0 {
            for (slot, frame) in wavetable.interaction_frames(index).into_iter().enumerate() {
                owner.graph.publish(
                    12 + slot,
                    index,
                    frame.generation,
                    frame.output,
                    voice.value,
                );
                if let Some(at) = frame.wrap {
                    owner
                        .graph
                        .natural_event(12 + slot, index, at, frame.latency);
                }
            }
        }
        output
    }
    fn configure_resynth_interactions(&mut self) {
        let owner = &mut self.oscillator_engine;
        if (owner.graph.demanded | owner.graph.targets) & (15 << 16) == 0 {
            owner
                .resynth
                .configure_interactions(Default::default(), Default::default());
            return;
        }
        let inputs = std::array::from_fn(|slot| {
            std::array::from_fn(|band| owner.graph.input(16 + slot, VOICES + band))
        });
        let demand = std::array::from_fn(|slot| {
            if self.preset.resynth.enabled && owner.graph.needed(16 + slot) {
                u16::MAX
            } else {
                0
            }
        });
        owner.resynth.configure_interactions(inputs, demand);
    }
    fn publish_resynth_interactions(&mut self) {
        let owner = &mut self.oscillator_engine;
        for (slot, frames) in owner.resynth.interaction_frames().iter().enumerate() {
            if !owner.graph.needed(16 + slot) {
                continue;
            }
            for (band, frame) in frames.iter().enumerate() {
                owner.graph.publish(
                    16 + slot,
                    VOICES + band,
                    frame.generation,
                    frame.output,
                    1.0 / crate::resynth::BANDS as f32,
                );
                if let Some(at) = frame.wrap {
                    owner
                        .graph
                        .natural_event(16 + slot, VOICES + band, at, frame.latency);
                }
            }
        }
    }
    fn track_tb303_note(&mut self) {
        if self.voices[1..].iter().any(|v| v.held || v.sustained) {
            return;
        }
        if let Some((channel, note, velocity)) = self
            .oscillator_engine
            .tb303
            .as_ref()
            .and_then(|b| b.note_state())
            && (self.source_channel != channel
                || self.source_note != note
                || self.source_velocity != velocity as f32 / 127.0)
        {
            self.mod_note_on(channel, note, velocity);
        }
    }

    pub fn process_input_frame(&mut self, input: Stereo) -> Stereo {
        if let Some(mixed) = &mut self.oscillator_engine.mixed {
            mixed.set_input(input);
        }
        self.oscillator_engine.graph.begin_frame();
        self.group_sync_mask = self.live_grains;
        if (self.oscillator_engine.graph.demanded | self.oscillator_engine.graph.targets)
            & (15 << 16)
            != 0
        {
            self.configure_resynth_interactions();
        }
        let output = self.process_input_frame_inner(input);
        if let Some(pool) = &mut self.oscillator_engine.phase_adapters {
            pool.finish_frame();
        }
        self.publish_resynth_interactions();
        self.oscillator_engine.graph.finish_frame();
        output
    }
    fn process_input_frame_inner(&mut self, input: Stereo) -> Stereo {
        if self.rack.is_some() {
            // A Rack's sources are its own; the input is its Vocal's voice.
            return self.process_rack_frame(input);
        }
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
        } else if let Some(synth) = &self.oscillator_engine.synth {
            synth.output()
        } else {
            input
        };
        self.sync_history_window();
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
            // Grain feedback enters the history the grains read, as Clouds does.
            let fed = self.grain_feedback.output;
            let recorded = if fed == [0.0; 2] {
                source_input
            } else {
                std::array::from_fn(|c| source_input[c] + fed[c])
            };
            live.push_with_meter(recorded, input, self.frozen);
            if windowed {
                let shift = (live.range_epoch() - epoch) as f64;
                if shift != 0.0 {
                    self.frozen_center -= shift;
                    for slot in self.live_grain_slots() {
                        self.grains[slot].position -= shift;
                        self.grains[slot].anchor -= shift;
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
            let mip = self.sample.mip_for_rate(source_rate, true);
            let processed = source_fx::read(
                &self.sample,
                &self.processed_source,
                &self.previous_source,
                self.source_fade_remaining,
                self.cursor,
                mip,
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
                    .read_region_mip(self.cursor, mip, &self.file_range)
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
        let (mut loudest_voice, mut voice_sum) = self.advance_modulators(sr);
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
        filter_slots::set_voice_gain(&mut self.filter_banks, loudest_voice);
        self.schedule_grains(!effects_only);
        let synth_grains = self.oscillator_engine.synth.is_some();
        let (voice_frames, group_sent) = self.render_grains(synth_grains);
        let file_source = self.live_input.is_none();
        let group_sends = self.taps.group_sends();
        let norm = self.grain_norm(effects_only);
        let input_gain = self.input_gain;
        let feedback = self.modulated.params.grain_feedback;
        if feedback > 0.0 || self.grain_feedback.output != [0.0; 2] {
            // The grains as the notes' VCA weighs them, at the overlap level.
            let mut grains = [0.0; 2];
            if synth_grains && self.main_pitch_sequence() {
                // MainPitch histories feed themselves, never the legacy tape.
                grains = voice_frames[0];
            } else if synth_grains {
                grains = self.mixed_grains(&voice_frames, true);
            } else {
                for i in bits(self.sounding | u64::from(bus.is_some())) {
                    let weight = self.voices[i].value.max(0.0);
                    for c in 0..2 {
                        grains[c] += voice_frames[i][c] * weight;
                    }
                }
            }
            self.grain_feedback.feed(grains.map(|x| x * norm), feedback);
            if file_source && feedback > 0.0 && !self.frozen {
                let center = self.center();
                self.grain_feedback.write(center);
            }
        }
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
        let grain_sends = self.oscillator_engine.synth.is_none() && self.taps.sends(Stage::Grains);
        let mut grain_send = [0.0; 2];
        let mut mixed_frames = [[0.0; 2]; VOICES];
        let mut before_fx = [0.0; 2];
        let mut grain_tap = [0.0; 2];
        let masks = self.filter_masks;
        // Voices outside the sounding set have no gain, except a bus's voice.
        let mixing = self.sounding | u64::from(bus.is_some());
        if self.oscillator_engine.synth.is_some() {
            // Oscillators replace each voice's source and grains before its
            // filter; the grains of voice slot 0 follow the VCA instead.
            self.oscillator_engine
                .synth
                .as_mut()
                .unwrap()
                .begin_frame(self.lfo_values[0]);
            for i in bits(mixing) {
                let gain = self.voices[i].value;
                if gain <= 0.0 {
                    continue;
                }
                let frame = self.render_synth_voice(i);
                // Filters placed after the voice source shape its oscillators.
                let frame = filter_slots::route_chain(
                    &mut self.filter_banks,
                    masks[FilterPosition::AfterGrains as usize],
                    frame,
                    Instance::Voice(i),
                    Some((&mut self.taps, input_gain * gain)),
                );
                mixed_frames[i] = frame.map(|x| x * input_gain);
                for (c, total) in before_fx.iter_mut().enumerate() {
                    *total += mixed_frames[i][c] * gain;
                }
            }
            grain_tap = before_fx;
        }
        for i in bits(mixing) {
            if self.voices[i].value <= 0.0 || self.oscillator_engine.synth.is_some() {
                continue;
            }
            let frame = filter_slots::route_chain(
                &mut self.filter_banks,
                masks[FilterPosition::AfterGrains as usize],
                voice_frames[i],
                Instance::Voice(i),
                Some((&mut self.taps, norm * input_gain * self.voices[i].value)),
            );
            let dry = if self.voices[i].granular() {
                chain_dry
            } else {
                0.0
            };
            mixed_frames[i] =
                std::array::from_fn(|c| (source[c] * dry + frame[c] * chain_grain) * input_gain);
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
        let main_pitch = self.main_pitch_sequence();
        if main_pitch {
            before_fx = [0.0; 2];
            for i in bits(mixing) {
                if self.voices[i].value > 0.0 {
                    let grains = voice_frames[i].map(|x| x * norm * input_gain);
                    let own_source = if self.oscillator_engine.synth.is_some() {
                        mixed_frames[i]
                    } else {
                        source.map(|x| x * input_gain)
                    };
                    mixed_frames[i] = self.render_main_resynth(
                        i,
                        mixed_frames[i],
                        own_source,
                        grains,
                        self.oscillator_engine.synth.is_some(),
                    );
                }
                for c in 0..2 {
                    before_fx[c] += mixed_frames[i][c] * self.voices[i].value;
                }
            }
            if voice_sum > 0.0 {
                before_fx = before_fx.map(|x| x / voice_sum);
            }
        }
        let resynth_active = !main_pitch && self.oscillator_engine.resynth.active();
        let resynth_output = if main_pitch {
            before_fx
        } else {
            self.oscillator_engine.resynth.process_interacting(
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
                Some((&mut self.taps, voice_sum)),
                self.oscillator_engine.phase_adapters.as_deref_mut(),
                self.frame,
            )
        };
        let after_resynth = if main_pitch {
            0
        } else {
            masks[FilterPosition::AfterResynth as usize]
        };
        let resynth_output = filter_slots::route_chain(
            &mut self.filter_banks,
            after_resynth,
            resynth_output,
            Instance::Shared,
            Some((&mut self.taps, voice_sum)),
        );
        // The shared stages reach every voice and are weighted by each, so
        // what they send follows the notes' VCA as their return does.
        if !main_pitch && self.taps.sends(Stage::Resynth) {
            self.taps
                .send(Stage::Resynth, resynth_output.map(|x| x * voice_sum));
        }
        let resynth_latency =
            filter_slots::latency_samples(&self.filter_banks, after_resynth, Instance::Shared);
        let resynth_output = if main_pitch {
            resynth_output
        } else {
            let aligned = self.filters[0].align_resynth_input(before_fx, resynth_latency);
            self.taps.mix(Stage::Resynth, aligned, resynth_output)
        };
        let resynth_output = match entered {
            Some(e) => std::array::from_fn(|c| resynth_output[c] + e[0][c] * input_gain),
            None => resynth_output,
        };
        let pre_output = self.pre_color_rack.process_with_filters(
            resynth_output,
            1.0,
            &self.filter_fx_masks[0],
            &mut self.filter_banks,
            &mut self.taps,
            voice_sum,
        );
        if self.taps.sends(Stage::PreFx) {
            self.taps
                .send(Stage::PreFx, pre_output.map(|x| x * voice_sum));
        }
        let pre_output = self.taps.mix(
            Stage::PreFx,
            self.pre_color_rack.aligned_input(),
            pre_output,
        );
        let shared_latency = self.pre_color_rack.latency_samples() + resynth_latency;
        let aligned_before_fx = self
            .pre_color_rack
            .align_reference(before_fx, resynth_latency);
        let correction = if self.pre_color_rack.is_active()
            || self.filter_fx_masks[0].iter().any(|&mask| mask != 0)
            || resynth_active
            || after_resynth != 0
            || entered.is_some()
        {
            // A bus has one voice, so what enters at Filters reaches it here.
            let filters = entered.map_or([0.0; 2], |e| e[1].map(|x| x * input_gain));
            std::array::from_fn(|c| pre_output[c] - aligned_before_fx[c] + filters[c])
        } else {
            [0.0; 2]
        };
        let mut filtered = [0.0; 2];
        let mut grain_bypass = [0.0; 2];
        let mut private_dry = [0.0; 2];
        let mut private_frames = [[0.0; 2]; VOICES];
        let filter_sends = self.taps.sends(Stage::Filters);
        let mut filter_send = [0.0; 2];
        for i in bits(mixing) {
            let mixed = self.filters[i].align_source_input(mixed_frames[i], shared_latency);
            if self.voices[i].value <= 0.0 {
                continue;
            }
            let unfiltered = std::array::from_fn(|c| mixed[c] + correction[c]);
            let voice = filter_slots::route_chain(
                &mut self.filter_banks,
                masks[FilterPosition::BeforeFilters as usize],
                unfiltered,
                Instance::Voice(i),
                Some((&mut self.taps, self.voices[i].value)),
            );
            let voice = if bus.is_some_and(|b| !b.filters) {
                voice
            } else {
                self.filters[i].process_tapped(voice, &mut self.taps, self.voices[i].value)
            };
            let voice = filter_slots::route_chain(
                &mut self.filter_banks,
                masks[FilterPosition::AfterFilters as usize],
                voice,
                Instance::Voice(i),
                Some((&mut self.taps, self.voices[i].value)),
            );
            if filter_sends {
                for c in 0..2 {
                    filter_send[c] += voice[c] * self.voices[i].value;
                }
            }
            let extra_latency = filter_slots::latency_samples(
                &self.filter_banks,
                masks[FilterPosition::BeforeFilters as usize]
                    | masks[FilterPosition::AfterFilters as usize],
                Instance::Voice(i),
            );
            let aligned = self.filters[i].align_chain_input(unfiltered, extra_latency);
            let voice = self.taps.mix(Stage::Filters, aligned, voice);
            if self.voices[i].main_pitch && self.voices[i].granular {
                private_frames[i] = voice;
            }
            for c in 0..2 {
                filtered[c] += voice[c] * self.voices[i].value;
                if self.voices[i].main_pitch && !self.voices[i].granular {
                    grain_bypass[c] += voice[c] * self.voices[i].value;
                } else if self.voices[i].main_pitch {
                    private_dry[c] += voice[c] * self.voices[i].value;
                }
            }
        }
        if filter_sends {
            self.taps.send(Stage::Filters, filter_send);
        }
        let history_grains = self.mixed_grains(&voice_frames, true);
        if self.oscillator_engine.synth.is_some() && main_pitch {
            self.capture_sequence_histories(&private_frames, &voice_frames, norm);
        }
        if let Some(synth) = &mut self.oscillator_engine.synth {
            // The shared tape only records legacy/MIDI voices. MainPitch
            // parties own separate tapes and must never enter each other's.
            synth.set_output(std::array::from_fn(|c| {
                filtered[c] - grain_bypass[c] - private_dry[c]
            }));
            // The history's source path, one frame later, is the dry signal.
            let dry = if self.live_input.is_some() {
                std::array::from_fn(|c| source[c] + grain_bypass[c] + private_dry[c])
            } else {
                filtered
            };
            // A Synth's grain engine follows its filters.
            let grains = if effects_only {
                dry
            } else {
                std::array::from_fn(|c| {
                    grain_bypass[c]
                        + (dry[c] - grain_bypass[c]) * (1.0 - self.smooth_mix)
                        + history_grains[c] * norm * self.smooth_mix
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
        let processed = self.color_rack.process_with_filters(
            filtered,
            loudest_voice,
            &self.filter_fx_masks[1],
            &mut self.filter_banks,
            &mut self.taps,
            1.0,
        );
        if self.taps.sends(Stage::PostFx) {
            self.taps.send(Stage::PostFx, processed);
        }
        let processed = self.taps.mix(Stage::PostFx, filtered, processed);
        // While bypassed, the source plays clean: the file or input without
        // Source FX where those are rendered ahead or chained.
        let bypass_source = self.bypass.then(|| {
            if self.processed_source.is_some()
                || self.previous_source.is_some()
                || !self.preset.source_color_chain.is_empty()
            {
                if self.live_input.is_some() || bus.is_some() {
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
                            v * self.fades.gain(
                                self.cursor,
                                self.sample.sample_rate,
                                &self.file_range,
                            )
                        })
                }
            } else {
                source
            }
        });
        self.finish_frame(processed, bypass_source, input_gain, source_rate)
    }

    /// A windowed history follows its length setting; grains and the frozen
    /// centre keep their place in the audio that stays.
    #[inline(always)]
    fn sync_history_window(&mut self) {
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
                    grain.anchor = remap(grain.anchor);
                }
            }
        }
    }
    /// One frame of every LFO and of each sounding voice's envelopes: the
    /// loudest voice's ENV 1 level and the sum of all of them.
    #[inline(always)]
    fn advance_modulators(&mut self, sr: f32) -> (f32, f32) {
        self.lfo_values.fill(0.0);
        let bpm = self.modulated.params.bpm;
        for index in bits(self.sounding) {
            self.voice_lfo_values[index].fill(0.0);
        }
        for i in 0..self.modulated.lfo_count {
            let lfo = &self.modulated.lfos[i];
            let position = (lfo.reset == LfoReset::Tempo).then(|| {
                if lfo.sync {
                    self.song_beat / f64::from(lfo.division)
                } else {
                    self.song_beat * 60.0 / f64::from(bpm) * f64::from(lfo.rate_hz)
                }
            });
            self.lfo_values[i] =
                self.oscillators[i].next_sample(lfo, sr, bpm, self.follower.min(1.0), position);
            for index in bits(self.sounding) {
                self.voice_lfo_values[index][i] = if lfo.poly {
                    self.voice_oscillators[index][i].next_sample(
                        lfo,
                        sr,
                        bpm,
                        self.follower.min(1.0),
                        position,
                    )
                } else {
                    self.lfo_values[i]
                };
            }
        }
        self.song_beat += f64::from(bpm) / (60.0 * f64::from(sr));
        self.env_values = [0.0; crate::preset::MODULATORS];
        let mut loudest_voice = 0.0_f32;
        let mut voice_sum = 0.0_f32;
        // A voice leaves the set on the frame its envelopes end at zero, so
        // the others keep the zero levels and gain that frame left.
        for index in bits(self.sounding) {
            let v = &mut self.voices[index];
            let mut active = false;
            let mut amp = 0.0;
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
                    amp = value;
                }
            }
            if v.main_pitch && !v.env_gate {
                if v.held {
                    // Smooth the constant level at onsets and live mode edits.
                    let step = (200.0 / sr).min(1.0);
                    v.drone_gain += (1.0 - v.drone_gain) * step;
                } else {
                    v.drone_gain *= v.drone_release;
                    if v.drone_gain < 0.0001 {
                        v.drone_gain = 0.0;
                    }
                }
                active |= v.held || v.drone_gain > 0.0;
                amp = v.drone_gain;
            }
            v.value = amp * v.velocity;
            loudest_voice = loudest_voice.max(v.value);
            voice_sum += v.value;
            if !active {
                self.sounding &= !(1 << index);
            }
        }
        (loudest_voice, voice_sum)
    }
    /// Every live grain's frame for this sample, summed per voice, and what
    /// each advanced group sends to its bus. `follow_mix` grains read a mixed
    /// history in voice slot 0, after the notes' VCA, as a Synth's do.
    #[inline(always)]
    fn render_grains(
        &mut self,
        follow_mix: bool,
    ) -> ([Stereo; VOICES], [Stereo; crate::grain_groups::GROUPS]) {
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
        // level directly, as read_region_mip would decide for every grain.
        let fading = file_source && self.fades != Fades::default();
        let antialiasing = self.preset.params.antialiasing;
        let whole_file = !source_effects && range.start == 0 && range.end == sample.len();
        let source_chain = self.taps.chain(Stage::Source);
        let top_level = sample.active_levels() - 1;
        // Grains that step faster than their level's frames read the whole
        // file with a wide kernel here, in a loop of their own: inlined in the
        // loop below, those kernels slow every grain's read by a third.
        let group_graph = self.modulated.grain_groups.advanced
            && self.oscillator_engine.graph.active()
            && (self.oscillator_engine.graph.demanded | self.oscillator_engine.graph.targets)
                & (15 << 25)
                != 0;
        let group_inputs = std::array::from_fn::<_, 4, _>(|i| {
            self.oscillator_engine
                .graph
                .input(25 + i, interaction::GLOBAL)
        });
        let mut wide = if antialiasing && whole_file {
            self.live_grains & self.wide_grains
        } else {
            0
        };
        let mut candidates = wide;
        while candidates != 0 {
            let slot = candidates.trailing_zeros() as usize;
            candidates &= candidates - 1;
            let command = if group_graph && self.group_sync_mask & (1 << slot) != 0 {
                self.group_grains
                    .group(slot)
                    .map(|g| group_inputs[g as usize].playback.mode)
            } else {
                None
            };
            if self.grains[slot].history.is_some()
                || self.grain_interaction.reading(slot)
                || self.grains[slot].playback_slew.is_active()
                || command.is_some_and(|m| m != crate::interaction::SyncMode::Off)
            {
                wide &= !(1 << slot);
            }
        }
        if wide != 0 {
            let mut pending = wide;
            let grains = &self.grains;
            let reads = std::iter::from_fn(move || {
                (pending != 0).then(|| {
                    let slot = pending.trailing_zeros() as usize;
                    pending &= pending - 1;
                    (slot, grains[slot].position, grains[slot].mip)
                })
            });
            sample.read_mips(reads, &mut self.wide_frames);
        }
        let groups = &self.modulated.grain_groups;
        let histories = self.preset_runtime.sequence_histories.as_deref();
        let history_epoch = self
            .live_input
            .as_ref()
            .map_or(0, |live| live.range_epoch());
        // Each group's grains in the chain, and what it sends to its bus.
        let group_chains = self.taps.group_chains();
        let group_sends = self.taps.group_sends();
        let mut group_sent = [[0.0; 2]; crate::grain_groups::GROUPS];
        let mut group_generation = [[0.0; 2]; 4];
        let mut group_output = [[0.0; 2]; 4];
        let synth_grains = follow_mix;
        // A file's grains also read what feedback overdubbed.
        let overdub = file_source && self.grain_feedback.readable();
        // Played-in clouds of new notes fade in on a smoothstep.
        let onsets = self.onsets;
        let fade = |x: f32| {
            let x = x.min(1.0);
            x * x * (3.0 - 2.0 * x)
        };
        let mut pending = self.live_grains;
        while pending != 0 {
            let slot = pending.trailing_zeros() as usize;
            pending &= pending - 1;
            let grain = &mut self.grains[slot];
            let group = self.group_grains.group(slot).filter(|_| group_graph);
            let command = if self.group_sync_mask & (1 << slot) != 0 {
                group.map_or(Default::default(), |g| group_inputs[g as usize].playback)
            } else {
                Default::default()
            };
            let span = grain.rate * grain.duration as f64;
            grain
                .playback_slew
                .correct(grain.playback_phase, command, self.sample_rate as f64);
            let birth = if grain.age == 0 {
                grain.birth_time
            } else {
                0.0
            };
            let rate = (grain.rate + grain.playback_slew.next() * span) * (1.0 - birth);
            let hard = command.mode == crate::interaction::SyncMode::Hard
                && command.event >= 0.0
                && command.strength > 0.0;
            let reset = hard.then(|| {
                grain.playback_slew.clear();
                let anchor = if self.live_input.is_some() {
                    grain
                        .anchor
                        .clamp(range.start as f64, range.end.saturating_sub(1) as f64)
                } else {
                    grain.anchor
                };
                (command.event, anchor + command.phase * span)
            });
            let phase_read = command.mode != crate::interaction::SyncMode::Off
                || grain.playback_slew.is_active();
            let mip = if antialiasing {
                if phase_read {
                    sample.mip_for_rate(rate / 4.0, true)
                } else {
                    grain.mip
                }
            } else {
                Mip::default()
            };
            let read = |position: f64| {
                let position = range.start as f64
                    + sample::wrap_position(position - range.start as f64, range.len() as f64);
                if let Some(history) = grain.history {
                    return histories.map_or([0.0; 2], |h| {
                        h.read(
                            history as usize,
                            position,
                            &range,
                            history_epoch,
                            rate,
                            antialiasing,
                        )
                    });
                }
                let mut frame = if wide & (1 << slot) != 0 {
                    self.wide_frames[slot]
                } else if whole_file && !mip.is_wide() {
                    sample.read_level(position, mip.level.min(top_level))
                } else if source_effects {
                    let processed = source_fx::read(
                        &self.sample,
                        &self.processed_source,
                        &self.previous_source,
                        self.source_fade_remaining,
                        position,
                        mip,
                        &range,
                    );
                    if source_chain < 1.0 {
                        let raw = sample.read_region_mip(position, mip, &range);
                        std::array::from_fn(|c| raw[c] + (processed[c] - raw[c]) * source_chain)
                    } else {
                        processed
                    }
                } else {
                    sample.read_region_mip(position, mip, &range)
                };
                if fading {
                    let gain = self.fades.gain(position, sample.sample_rate, &range);
                    frame = frame.map(|v| v * gain);
                }
                if overdub {
                    let fed = self.grain_feedback.read(position);
                    frame = std::array::from_fn(|c| frame[c] + fed[c]);
                }
                frame
            };
            let (mut frame, next_position) =
                self.grain_interaction
                    .read(slot, grain.position, rate, reset, phase_read, read);
            if birth > 0.0 {
                frame = frame.map(|v| v * (1.0 - birth) as f32);
            }
            if span != 0.0 {
                grain.playback_phase = if hard {
                    command.phase + rate / span * (1.0 - command.event)
                } else {
                    grain.playback_phase + rate / span
                };
                grain.playback_phase = grain.playback_phase.rem_euclid(1.0);
            }
            // A group's grains take its amp envelope in place of the window.
            let grouped = if group_graph {
                self.group_grains
                    .process_unleveled(slot, grain.age, frame, groups)
            } else if groups.advanced {
                self.group_grains.process(slot, grain.age, frame, groups)
            } else {
                None
            };
            if let Some((group, shaped)) = grouped {
                let shaped = if onsets & 1 << grain.voice != 0 {
                    let gain = fade(self.voices[grain.voice].onset[group]);
                    shaped.map(|x| x * gain)
                } else {
                    shaped
                };
                let mut panned: Stereo = std::array::from_fn(|c| shaped[c] * grain.pan[c]);
                if group_graph {
                    let weight = if synth_grains && !self.voices[grain.voice].main_pitch {
                        1.0
                    } else {
                        self.voices[grain.voice].value.max(0.0)
                    };
                    let ringed = self
                        .grain_interaction
                        .ring(slot, panned, group_inputs[group]);
                    for c in 0..2 {
                        group_generation[group][c] += panned[c] * weight;
                        group_output[group][c] += ringed[c] * weight;
                        panned[c] = ringed[c] * groups.groups[group].level;
                    }
                }
                let chain = group_chains[group];
                for c in 0..2 {
                    voice_frames[grain.voice][c] += panned[c] * chain;
                }
                if group_sends & 1 << group != 0 {
                    // Sends follow the notes' VCA as the chain does; a
                    // Synth's grains already read its voices after theirs.
                    let weight = if synth_grains && !self.voices[grain.voice].main_pitch {
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
                let mut amplitude = tables::grain_window(phase, grain.shape);
                if onsets & 1 << grain.voice != 0 {
                    amplitude *= fade(self.voices[grain.voice].onset[crate::grain_groups::GROUPS]);
                }
                for c in 0..2 {
                    voice_frames[grain.voice][c] += frame[c] * amplitude * grain.pan[c];
                }
            }
            grain.position = range.start as f64
                + sample::wrap_position(next_position - range.start as f64, range.len() as f64);
            grain.age += 1;
            if grain.age >= grain.duration {
                self.live_grains &= !(1 << slot);
            }
        }
        for i in bits(onsets) {
            let voice = &mut self.voices[i];
            let mut done = true;
            for (onset, step) in voice.onset.iter_mut().zip(voice.onset_step) {
                if *onset < 1.0 {
                    *onset += step;
                    done &= *onset >= 1.0;
                }
            }
            if done {
                self.onsets &= !(1 << i);
            }
        }
        if group_graph {
            for group in 0..4 {
                self.oscillator_engine.graph.publish(
                    25 + group,
                    interaction::GLOBAL,
                    group_generation[group].map(|v| v * self.smooth_grain_norm),
                    group_output[group].map(|v| v * self.smooth_grain_norm),
                    1.0,
                );
            }
        }
        if self.oscillator_engine.graph.needed(29) {
            let mut wet = [0.0; 2];
            for (i, frame) in voice_frames.iter().enumerate() {
                let weight = if synth_grains && !self.voices[i].main_pitch {
                    f32::from(i == 0)
                } else {
                    self.voices[i].value.max(0.0)
                };
                for c in 0..2 {
                    wet[c] += frame[c] * weight;
                }
            }
            let wet = wet.map(|v| v * self.smooth_grain_norm);
            self.oscillator_engine
                .graph
                .publish(29, interaction::GLOBAL, wet, wet, 1.0);
        }
        (voice_frames, group_sent)
    }
    /// Output level, the sends' share of it, soft clipping and the meter,
    /// then the file cursor and bar loops move on. `bypass_source` replaces
    /// the processed frame while bypassed.
    #[inline(always)]
    fn finish_frame(
        &mut self,
        processed: Stereo,
        bypass_source: Option<Stereo>,
        input_gain: f32,
        source_rate: f64,
    ) -> Stereo {
        let volume = if self.bypass {
            self.preset.params.master_volume
        } else {
            self.modulated.params.master_volume
        };
        let gain = volume * self.output_gain;
        self.smooth_gain = fast::flush(self.smooth_gain + (gain - self.smooth_gain) * 0.002);
        self.smooth_bypass_gate = fast::flush(
            self.smooth_bypass_gate
                + ((if self.playing || self.preset.bus.is_some() {
                    1.0
                } else {
                    0.0
                }) - self.smooth_bypass_gate)
                    * 0.002,
        );
        self.taps.finish(self.smooth_gain, self.bypass);
        let output = std::array::from_fn(|c| {
            let value = (match bypass_source {
                Some(source) => source[c] * input_gain * self.smooth_bypass_gate,
                None => processed[c],
            }) * self.smooth_gain;
            if !value.is_finite() {
                0.0
            } else if self.preset.bus.is_none() && value.abs() > 0.98 {
                // Instrument protection must not clip a summed bus before
                // its fader. Hosts protect the complete Master output once.
                value.signum() * (0.98 + 0.02 * fast::tanh((value.abs() - 0.98) / 0.02))
            } else {
                value
            }
        });
        if !self.preset.bus.as_ref().is_some_and(|b| b.master) {
            for (c, &value) in output.iter().enumerate() {
                self.peak[c] = self.peak[c].max(value.abs());
            }
        }
        if self.playing {
            let bar_loop = self.bar_loop_active();
            let phase_step = self.bar_phase_step;
            if bar_loop && !self.frozen {
                for index in bits(self.maybe_active) {
                    let voice = &mut self.voices[index];
                    if voice.active() {
                        voice.loop_phase =
                            sample::wrap_position(voice.loop_phase + phase_step, 1.0);
                    } else {
                        self.maybe_active &= !(1 << index);
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

    fn grain_norm_target(&self, effects_only: bool) -> f32 {
        if !effects_only && self.overlap > 2.0 {
            tables::inverse_sqrt(self.overlap - 1.0)
        } else {
            1.0
        }
    }
    /// Grain births/deaths change the control-rate overlap, but the level
    /// of the already sounding grains must move continuously, sample by sample.
    fn grain_norm(&mut self, effects_only: bool) -> f32 {
        let target = self.grain_norm_target(effects_only);
        self.smooth_grain_norm += (target - self.smooth_grain_norm) * self.grain_norm_smoothing;
        self.smooth_grain_norm
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
