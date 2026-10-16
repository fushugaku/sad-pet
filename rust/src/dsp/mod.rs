pub mod color;
pub mod compressor;
pub mod delay;
pub mod effects;
pub mod filter;
pub mod modulation;
pub mod radias_waveshaper;
pub mod sample;
pub mod telemetry;

use crate::{
    Stereo,
    color::{ColorSlot, MAX_COLOR_SLOTS},
    matrix::{self, Source},
    midi::Message,
    performance::{Mode, Performance},
    preset::{Destination, Env, Lfo, Params, Preset},
    tables::{self, Random},
};
use color::ColorRack;
use filter::FilterBank;
use modulation::{Envelope, Oscillator};
use sample::LiveInput;
pub use sample::{Fades, Region, SampleBuffer};
use serde::Serialize;
use std::sync::Arc;

pub const MAX_GRAINS: usize = 128;
const VOICES: usize = 9;
const CONTROL_INTERVAL: u64 = 32;

#[derive(Clone, Copy, Debug, Default)]
struct Grain {
    active: bool,
    position: f64,
    rate: f64,
    age: u32,
    duration: u32,
    shape: f32,
    pan: Stereo,
    voice: usize,
}

#[derive(Clone, Copy, Debug, Default)]
struct Voice {
    envelopes: [Envelope; 3],
    note: u8,
    channel: u8,
    velocity: f32,
    held: bool,
    sustained: bool,
    value: f32,
    env_values: [f32; 3],
}
impl Voice {
    fn on(&mut self) {
        self.env_values = [0.0; 3];
        self.value = 0.0;
        for env in &mut self.envelopes {
            env.on();
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

#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct GrainView {
    pub position: f32,
    pub age: f32,
    pub pan: f32,
}

#[derive(Clone, Copy, Default)]
struct Modulated {
    params: Params,
    pan: f32,
    color_slots: [ColorSlot; MAX_COLOR_SLOTS],
    color_len: usize,
    pre_color_slots: [ColorSlot; MAX_COLOR_SLOTS],
    pre_color_len: usize,
    lfos: [Lfo; 3],
    envs: [Env; 3],
    performance: crate::performance::Modulation,
}
impl Modulated {
    fn from_preset(preset: &Preset) -> Self {
        let mut result = Self {
            params: preset.params,
            pan: 0.5,
            color_slots: [ColorSlot::default(); MAX_COLOR_SLOTS],
            color_len: preset.color_chain.len(),
            pre_color_slots: [ColorSlot::default(); MAX_COLOR_SLOTS],
            pre_color_len: preset.pre_color_chain.len(),
            lfos: preset.lfos,
            envs: preset.envs,
            performance: crate::performance::Modulation::from_settings(&preset.performance),
        };
        result.color_slots[..result.color_len].copy_from_slice(&preset.color_chain);
        result.pre_color_slots[..result.pre_color_len].copy_from_slice(&preset.pre_color_chain);
        result
    }
    fn route(&mut self, destination: Destination, value: f32) {
        if value == 0.0 {
            return;
        }
        use matrix::{
            EnvParameter as E, LfoParameter as L, Parameter as P, PerformanceParameter as F,
        };
        let variable = match destination {
            Destination::Parameter { parameter } => {
                let p = &mut self.params;
                Some(match parameter {
                    P::FilterEnvDepth => (&mut p.filter_env_depth, -1.0, 1.0, false),
                    P::FilterKeytrack => (&mut p.filter_keytrack, 0.0, 1.0, false),
                    P::Filter2EnvDepth => (&mut p.filter2_env_depth, -1.0, 1.0, false),
                    P::Filter2Keytrack => (&mut p.filter2_keytrack, 0.0, 1.0, false),
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
            Destination::Env { index, parameter } if index < 3 => {
                let e = &mut self.envs[index as usize];
                Some(match parameter {
                    E::Attack => (&mut e.attack, 0.0, 1.0, false),
                    E::Decay => (&mut e.decay, 0.0, 1.0, false),
                    E::Sustain => (&mut e.sustain, 0.0, 1.0, false),
                    E::Release => (&mut e.release, 0.0, 1.0, false),
                    E::Depth => (&mut e.depth, -1.0, 1.0, false),
                    E::ExtraDepth => (&mut e.extra_depth, -1.0, 1.0, false),
                })
            }
            Destination::Lfo { index, parameter } if index < 3 => {
                let l = &mut self.lfos[index as usize];
                Some(match parameter {
                    L::Rate => (&mut l.rate_hz, 0.01, 40.0, true),
                    L::Depth => (&mut l.depth, -1.0, 1.0, false),
                    L::Division => (&mut l.division, 0.0625, 32.0, true),
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
                effect.parameter_mut(parameter)
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
    grains: [Grain; MAX_GRAINS],
    voices: [Voice; VOICES],
    oscillators: [Oscillator; 3],
    pub lfo_values: [f32; 3],
    pub env_values: [f32; 3],
    color_rack: ColorRack,
    pre_color_rack: ColorRack,
    filters: [FilterBank; VOICES],
    random: Random,
    frame: u64,
    cursor: f64,
    frozen_center: f64,
    last_position: f32,
    grain_clock: f64,
    next_voice: usize,
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
    modulated: Modulated,
    overlap: f32,
    smooth_mix: f32,
    smooth_gain: f32,
    smooth_bypass_gate: f32,
    pub peak: Stereo,
}

impl Engine {
    pub fn new(sample_rate: u32, sample: Arc<SampleBuffer>, preset: Preset) -> Self {
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
            grains: [Grain::default(); MAX_GRAINS],
            voices: [Voice::default(); VOICES],
            oscillators: std::array::from_fn(|i| {
                Oscillator::new(0x853c49e6748fea9b_u64.wrapping_add(i as u64 * 0x12345679))
            }),
            lfo_values: [0.0; 3],
            env_values: [0.0; 3],
            color_rack: ColorRack::new(sr),
            pre_color_rack: ColorRack::new(sr),
            filters: std::array::from_fn(|_| FilterBank::new(sr)),
            random: Random::default(),
            frame: 0,
            cursor: 0.0,
            frozen_center: 0.0,
            last_position: 0.0,
            grain_clock: 1.0,
            next_voice: 0,
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
            modulated: Modulated::default(),
            overlap: 1.0,
            smooth_mix: 1.0,
            smooth_gain: 0.0,
            smooth_bypass_gate: 0.0,
            peak: [0.0; 2],
        };
        engine.set_preset(preset);
        // Idle voices need their initial coefficients even when the first preset
        // has the same envelopes as Modulated::default(). Later refreshes only
        // configure envelopes whose values actually changed.
        for voice in &mut engine.voices {
            for (i, envelope) in voice.envelopes.iter_mut().enumerate() {
                envelope.configure(sr as f32, &engine.modulated.envs[i]);
            }
        }
        engine
    }

    pub fn set_preset(&mut self, mut preset: Preset) {
        preset.sanitize();
        self.exchange_preset(&mut preset);
    }

    /// Exchange a sanitized preset without allocating or destroying its buffers.
    /// The caller retires the previous preset away from the audio callback.
    pub fn exchange_preset(&mut self, preset: &mut Preset) {
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
        std::mem::swap(&mut self.preset, preset);
        self.sync_stored_chord();
        self.update_modulation();
        if mode_changed || hold_chord_changed {
            self.release_notes();
            self.midi_gate = self.performance.mode() != Mode::Keys;
            if self.playing {
                self.set_playing(true);
            }
        }
    }
    pub fn set_sample(&mut self, sample: Arc<SampleBuffer>) {
        self.live_input = None;
        self.sample = sample;
        self.reset_source();
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
        self.frozen_center = 0.0;
        self.reset_grains();
        self.color_rack.clear();
        self.pre_color_rack.clear();
        self.filters.iter_mut().for_each(FilterBank::clear);
    }

    pub fn host_transport(&mut self, bpm: f32, beat: Option<f64>, restart: bool) {
        if bpm.is_finite() && (20.0..=300.0).contains(&bpm) {
            self.preset.params.bpm = bpm;
        }
        if restart {
            self.release_notes();
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
        self.update_modulation();
    }
    pub fn apply_parameters(&mut self, changes: &[crate::parameters::Change]) {
        if crate::parameters::apply(&mut self.preset, changes) {
            self.refresh_parameters();
        }
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
        self.filters.iter_mut().for_each(FilterBank::clear);
        Ok(())
    }
    fn source_range(&self) -> std::ops::Range<usize> {
        self.live_input
            .as_ref()
            .map_or_else(|| self.file_range.clone(), |input| 0..input.sample.len())
    }
    pub fn set_fades(&mut self, fades: Fades) -> Result<(), String> {
        if self.live_input.is_some() {
            return Err("Load a recording or file to set fades".into());
        }
        self.fades = fades.validate()?;
        Ok(())
    }
    pub fn set_live_input(&mut self, enabled: bool, effects_only: bool) {
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
        self.filters.iter_mut().for_each(FilterBank::clear);
        if !self.playing {
            for voice in &mut self.voices {
                voice.kill();
            }
            self.smooth_bypass_gate = 0.0;
            self.peak = [0.0; 2];
        }
    }
    pub fn source_sample(&self) -> &SampleBuffer {
        self.live_input
            .as_ref()
            .map_or(&self.sample, |input| &input.sample)
    }
    pub fn reset_grains(&mut self) {
        self.grains.fill(Grain::default());
        self.grain_clock = 1.0;
    }
    pub fn set_playing(&mut self, playing: bool) {
        self.playing = playing;
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
            } else if self.performance.mode() == Mode::Keys && !self.midi_gate {
                self.voices[0].velocity = 1.0;
                self.voices[0].note = 60;
                self.voices[0].held = true;
                self.filters[0].clear();
                self.voices[0].on();
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
        for voice in &mut self.voices {
            voice.held = false;
            voice.sustained = false;
            voice.off();
        }
        if !enabled && self.playing && self.performance.mode() == Mode::Keys {
            self.voices[0].note = 60;
            self.voices[0].velocity = 1.0;
            self.voices[0].held = true;
            self.filters[0].clear();
            self.voices[0].on();
            self.mod_note_on(0, 60, 127);
            self.configure_voice_filter(0);
        }
    }
    pub fn center(&self) -> f64 {
        if self.frozen {
            self.frozen_center
        } else if let Some(input) = &self.live_input {
            let length = input.sample.len() as f64;
            let p = &self.modulated.params;
            // Leave enough history for a grain to advance without overtaking
            // the capture head. Position scans backwards through the history.
            let margin =
                (tables::grain_duration(p.grain_size) as f64 * self.sample_rate as f64 * 2.0
                    + p.cloud as f64 * length * 0.5
                    + 4.0)
                    .min(length * 0.9);
            (input.head() as f64 - margin - p.position as f64 * (length - margin))
                .rem_euclid(length)
        } else {
            let range = &self.file_range;
            range.start as f64
                + (self.cursor - range.start as f64
                    + self.modulated.params.position as f64 * range.len() as f64)
                    .rem_euclid(range.len() as f64)
        }
    }
    pub fn active_grains(&self) -> usize {
        self.grains.iter().filter(|g| g.active).count()
    }
    pub fn held_notes(&self) -> Vec<u8> {
        self.voices[1..]
            .iter()
            .filter(|v| v.held || v.sustained)
            .map(|v| v.note)
            .collect()
    }
    pub fn grain_views(&self) -> Vec<GrainView> {
        self.grains
            .iter()
            .filter(|g| g.active)
            .map(|g| GrainView {
                position: (g.position.rem_euclid(self.source_sample().len() as f64)
                    / self.source_sample().len() as f64) as f32,
                age: g.age as f32 / g.duration as f32,
                pan: (g.pan[1] - g.pan[0] + 1.0) * 0.5,
            })
            .collect()
    }

    fn release_notes(&mut self) {
        for voice in &mut self.voices {
            voice.held = false;
            voice.sustained = false;
            voice.off();
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
                    self.apply_midi(Message::NoteOff {
                        channel,
                        note,
                        velocity: 0,
                    });
                    return;
                }
                if !self.midi_gate {
                    self.set_midi_gate(true);
                }
                self.playing = true;
                let index = (1..VOICES)
                    .find(|&i| {
                        self.voices[i].held
                            && self.voices[i].note == note
                            && self.voices[i].channel == channel
                    })
                    .or_else(|| (1..VOICES).find(|&i| !self.voices[i].active()))
                    .unwrap_or(1 + self.next_voice % (VOICES - 1));
                self.next_voice = index;
                for grain in &mut self.grains {
                    if grain.voice == index {
                        grain.active = false;
                    }
                }
                let v = &mut self.voices[index];
                self.filters[index].clear();
                v.note = note;
                v.channel = channel;
                v.velocity = velocity as f32 / 127.0;
                v.held = true;
                v.sustained = false;
                v.on();
                self.mod_note_on(channel, note, velocity);
                self.configure_voice_filter(index);
                self.grain_clock = 1.0;
            }
            Message::NoteOff { channel, note, .. } => {
                if channel > 15 {
                    return;
                }
                for v in &mut self.voices[1..] {
                    if v.note == note && v.channel == channel {
                        v.held = false;
                        v.sustained = self.sustain[channel as usize];
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
                for (i, v) in self.voices.iter_mut().enumerate() {
                    if v.channel == channel {
                        v.kill();
                        self.filters[i].clear();
                        v.held = false;
                        v.sustained = false;
                        for grain in &mut self.grains {
                            if grain.voice == i {
                                grain.active = false;
                            }
                        }
                    }
                }
                self.color_rack.clear();
                self.pre_color_rack.clear();
            }
            Message::Control {
                channel,
                controller: 123,
                ..
            } if channel < 16 => {
                for v in &mut self.voices {
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
                self.filters.iter_mut().for_each(FilterBank::clear);
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
            Source::Env1 => self.env_values[0],
            Source::Env2 => self.env_values[1],
            Source::Env3 => self.env_values[2],
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
                if self.voices.iter().any(|v| v.held || v.sustained) {
                    1.0
                } else {
                    0.0
                }
            }
            Source::Follower => self.follower.min(1.0),
        }
    }
    fn update_modulation(&mut self) {
        let mut m = Modulated::from_preset(&self.preset);
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
        for (i, lfo) in m.lfos.into_iter().enumerate() {
            m.route(lfo.destination, self.lfo_values[i] * lfo.depth * 0.5);
        }
        for (i, env) in m.envs.into_iter().enumerate() {
            m.route(env.destination, self.env_values[i] * env.depth);
            m.route(env.extra_destination, self.env_values[i] * env.extra_depth);
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
        self.color_rack
            .configure_tempo(&m.color_slots[..m.color_len], m.params.bpm);
        self.pre_color_rack
            .configure_tempo(&m.pre_color_slots[..m.pre_color_len], m.params.bpm);
        let shaper_note = self.source_note as f32
            + self.bend[self.source_channel as usize] * m.params.midi_pitchbend_range
            + (m.params.pitch * 2.0 - 1.0) * m.params.pitch_range
            + m.params.sample_tune_semis
            + m.params.sample_tune_cents * 0.01;
        self.pre_color_rack.set_pitch(shaper_note);
        self.color_rack.set_pitch(shaper_note);
        self.performance.modulate(m.performance);
        for (i, env) in m.envs.iter().enumerate() {
            if *env != self.modulated.envs[i] {
                for voice in &mut self.voices {
                    voice.envelopes[i].configure(self.sample_rate as f32, env);
                }
            }
        }
        for (i, voice) in self.voices.iter().enumerate() {
            if !voice.envelopes[0].active() {
                continue;
            }
            let note = voice.note as f32
                + self.bend[voice.channel as usize] * m.params.midi_pitchbend_range;
            self.filters[i].configure(&m.params, note, voice.env_values);
        }
        self.modulated = m;
        let count = self.active_grains() as f32;
        self.overlap += (count - self.overlap) * if count > self.overlap { 0.9 } else { 0.2 };
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
    pub fn modulated_envs(&self) -> [Env; 3] {
        self.modulated.envs
    }
    pub fn modulated_lfos(&self) -> [Lfo; 3] {
        self.modulated.lfos
    }

    fn configure_voice_filter(&mut self, index: usize) {
        let voice = &self.voices[index];
        let note = voice.note as f32
            + self.bend[voice.channel as usize] * self.modulated.params.midi_pitchbend_range;
        self.filters[index].configure(&self.modulated.params, note, voice.env_values);
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
        let Some(slot) = self.grains.iter().position(|g| !g.active) else {
            return;
        };
        let Some(voice) = (0..VOICES)
            .map(|i| (i + self.next_voice) % VOICES)
            .find(|&i| self.voices[i].envelopes[0].active())
        else {
            return;
        };
        self.next_voice = (voice + 1) % VOICES;
        let p = &self.modulated.params;
        let note = self.voices[voice];
        let bend = self.bend[note.channel as usize] * p.midi_pitchbend_range;
        let semis = (p.pitch * 2.0 - 1.0) * p.pitch_range
            + (note.note as f32 - 60.0)
            + p.sample_tune_semis
            + p.sample_tune_cents * 0.01
            + bend
            + self.random.unit() * tables::detune_range(p.detune);
        let range = self.source_range();
        let length = range.len() as f64;
        let rate = tables::pitch_ratio(semis) * self.source_sample().sample_rate as f64
            / self.sample_rate as f64
            * if p.grain_size < 0.5 { -1.0 } else { 1.0 };
        let duration = (tables::grain_duration(p.grain_size) * self.sample_rate as f32)
            .round()
            .max(4.0) as u32;
        let position = range.start as f64
            + (self.center() - range.start as f64
                + self.random.bipolar() as f64 * p.cloud as f64 * length * 0.5)
                .rem_euclid(length);
        let pan =
            (self.modulated.pan + self.random.bipolar() * p.stereo_spread * 0.5).clamp(0.0, 1.0);
        self.grains[slot] = Grain {
            active: true,
            position,
            rate,
            age: 0,
            duration,
            shape: p.grain_shape,
            pan: tables::pan(pan),
            voice,
        };
    }

    pub fn process_frame(&mut self) -> Stereo {
        self.process_input_frame([0.0; 2])
    }

    pub fn process_input_frame(&mut self, input: Stereo) -> Stereo {
        let effects_only = self.live_input.as_ref().is_some_and(|i| i.effects_only);
        if let Some(live) = &mut self.live_input {
            live.push(input, self.frozen);
        }
        if self.playing {
            for action in self
                .performance
                .tick(self.sample_rate, self.modulated.params.bpm)
                .into_iter()
                .flatten()
            {
                self.apply_midi(action);
            }
        }
        let sr = self.sample_rate as f32;
        let source_rate = self.source_sample().sample_rate as f64 / self.sample_rate as f64;
        let source = if self.live_input.is_some() {
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
        };
        let level = source[0].abs().max(source[1].abs());
        self.follower += (level - self.follower)
            * if level > self.follower {
                0.006
            } else {
                0.00015
            };
        for i in 0..3 {
            self.lfo_values[i] = self.oscillators[i].next_sample(
                &self.modulated.lfos[i],
                sr,
                self.modulated.params.bpm,
                self.follower.min(1.0),
            );
        }
        self.env_values = [0.0; 3];
        let mut loudest_voice = 0.0_f32;
        let mut voice_sum = 0.0_f32;
        for v in &mut self.voices {
            for (i, env) in v.envelopes.iter_mut().enumerate() {
                let value = env.next_sample();
                v.env_values[i] = value;
                self.env_values[i] = self.env_values[i].max(value);
                if i == 0 {
                    v.value = value * v.velocity;
                    loudest_voice = loudest_voice.max(v.value);
                    voice_sum += v.value;
                }
            }
        }
        if self.frame.is_multiple_of(CONTROL_INTERVAL) {
            self.update_modulation();
        }
        let density = self.modulated.params.min_grains
            + (MAX_GRAINS as f32 - self.modulated.params.min_grains)
                * self.modulated.params.grains.powi(3);
        self.grain_clock += density as f64 / self.sample_rate as f64;
        if !effects_only && density > 0.0 && self.grain_clock >= 1.0 {
            self.grain_clock = self.grain_clock.fract();
            self.spawn();
        }
        let mut voice_frames = [[0.0; 2]; VOICES];
        let range = self.source_range();
        let sample = self
            .live_input
            .as_ref()
            .map_or(&*self.sample, |i| &i.sample);
        for grain in &mut self.grains {
            if !grain.active {
                continue;
            }
            let phase = 2.0 * grain.age as f32 / (grain.duration - 1) as f32;
            let amplitude = tables::grain_window(phase, grain.shape);
            let mut frame = sample.read_region(
                grain.position,
                grain.rate,
                self.preset.params.antialiasing,
                &range,
            );
            if self.live_input.is_none() {
                let gain = self.fades.gain(grain.position, sample.sample_rate, &range);
                frame = frame.map(|v| v * gain);
            }
            for c in 0..2 {
                voice_frames[grain.voice][c] += frame[c] * amplitude * grain.pan[c];
            }
            grain.position = range.start as f64
                + (grain.position + grain.rate - range.start as f64).rem_euclid(range.len() as f64);
            grain.age += 1;
            grain.active = grain.age < grain.duration;
        }
        let norm = if !effects_only && self.overlap > 2.0 {
            tables::inverse_sqrt(self.overlap - 1.0)
        } else {
            1.0
        };
        let input_gain = 10.0_f32.powf(self.modulated.params.in_gain * 0.05);
        self.smooth_mix += (self.modulated.params.mix - self.smooth_mix) * 0.002;
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
        let mut mixed_frames = [[0.0; 2]; VOICES];
        let mut before_fx = [0.0; 2];
        for (i, frame) in voice_frames.into_iter().enumerate() {
            if self.voices[i].value <= 0.0 {
                continue;
            }
            mixed_frames[i] = std::array::from_fn(|c| {
                (source[c] * dry_amount + frame[c] * grain_amount) * input_gain
            });
            for (c, total) in before_fx.iter_mut().enumerate() {
                *total += mixed_frames[i][c] * self.voices[i].value;
            }
        }
        // One stereo pre-filter rack, with its own tails and delay clock. The
        // shared return enters each note's filter before its VCA. Normalizing
        // the send/return by voice weight preserves per-note envelopes and the
        // dry voice/filter separation; an empty or bypassed rack is transparent.
        if voice_sum > 0.0 {
            before_fx.iter_mut().for_each(|v| *v /= voice_sum);
        }
        let pre_output = self.pre_color_rack.process(before_fx);
        let correction = if self.pre_color_rack.is_active() {
            std::array::from_fn(|c| pre_output[c] - before_fx[c])
        } else {
            [0.0; 2]
        };
        let mut filtered = [0.0; 2];
        for (i, mixed) in mixed_frames.into_iter().enumerate() {
            if self.voices[i].value <= 0.0 {
                continue;
            }
            let voice = self.filters[i].process(std::array::from_fn(|c| mixed[c] + correction[c]));
            for c in 0..2 {
                filtered[c] += voice[c] * self.voices[i].value;
            }
        }
        let processed = self
            .color_rack
            .process_with_voice_gain(filtered, loudest_voice);
        let volume = if self.bypass {
            self.preset.params.master_volume
        } else {
            self.modulated.params.master_volume
        };
        let gain = volume * 10.0_f32.powf(self.modulated.params.out_gain * 0.05);
        self.smooth_gain += (gain - self.smooth_gain) * 0.002;
        self.smooth_bypass_gate +=
            ((if self.playing { 1.0 } else { 0.0 }) - self.smooth_bypass_gate) * 0.002;
        let output = std::array::from_fn(|c| {
            let value = (if self.bypass {
                source[c] * input_gain * self.smooth_bypass_gate
            } else {
                processed[c]
            }) * self.smooth_gain;
            if !value.is_finite() {
                0.0
            } else if value.abs() > 0.98 {
                value.signum() * (0.98 + 0.02 * ((value.abs() - 0.98) / 0.02).tanh())
            } else {
                value
            }
        });
        for (c, &value) in output.iter().enumerate() {
            self.peak[c] = self.peak[c].max(value.abs());
        }
        if self.playing {
            self.cursor = self.file_range.start as f64
                + (self.cursor + source_rate - self.file_range.start as f64)
                    .rem_euclid(self.file_range.len() as f64);
        }
        self.frame += 1;
        output
    }

    pub fn process(&mut self, output: &mut [Stereo]) {
        for frame in output {
            *frame = self.process_frame();
        }
    }
}
