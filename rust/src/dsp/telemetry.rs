//! Read-only control feedback. Reuse the DSP's destination arithmetic, without
//! running processors or changing audio state. Called at UI publication time.
use super::{Engine, Modulated, Modulator, filter};
use crate::{
    matrix::Source,
    preset::{Destination, MODULATOR_ROUTES},
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A fixed-size audio-thread snapshot serialized with only its active slots.
#[derive(Clone, Copy)]
pub struct FilterSlotsView {
    slots: [crate::preset::FilterSlot; crate::preset::MAX_FILTER_SLOTS],
    len: usize,
}

impl Serialize for FilterSlotsView {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.slots[..self.len.min(self.slots.len())].serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for FilterSlotsView {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Slots;
        impl<'de> serde::de::Visitor<'de> for Slots {
            type Value = FilterSlotsView;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    formatter,
                    "up to {} filter slots",
                    crate::preset::MAX_FILTER_SLOTS
                )
            }

            fn visit_seq<A: serde::de::SeqAccess<'de>>(
                self,
                mut sequence: A,
            ) -> Result<Self::Value, A::Error> {
                let mut view = FilterSlotsView {
                    slots: [crate::preset::FilterSlot::default(); crate::preset::MAX_FILTER_SLOTS],
                    len: 0,
                };
                while let Some(slot) = sequence.next_element()? {
                    if view.len == view.slots.len() {
                        return Err(serde::de::Error::invalid_length(view.len + 1, &self));
                    }
                    view.slots[view.len] = slot;
                    view.len += 1;
                }
                Ok(view)
            }
        }
        deserializer.deserialize_seq(Slots)
    }
}

/// Fixed-size feedback for hosts. Publishing it does not allocate or lock.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct RealtimeView {
    pub sequencer: crate::sequence::View,
    #[serde(default)]
    pub drums: crate::drums::View,
    #[serde(default)]
    pub drum_controls: crate::drums::Controls,
    pub peak: [f32; 2],
    pub input_peak: [f32; 2],
    pub input_seconds: f64,
    pub center: f64,
    pub bar_beats: f64,
    pub playing: bool,
    pub midi_gate: bool,
    pub frozen: bool,
    pub bypass: bool,
    pub lfos: [f32; crate::preset::MODULATORS],
    pub envs: [f32; crate::preset::MODULATORS],
    #[serde(default)]
    pub modulated_resynth: crate::resynth::Settings,
    #[serde(default)]
    pub modulated_synth: crate::synth::Settings,
    #[serde(default)]
    pub modulated_wavetable: [crate::wavetable::Controls; crate::wavetable::OSCILLATORS],
    #[serde(default)]
    pub modulated_wavetable_motion: crate::wavetable::motion::Motion,
    #[serde(default)]
    pub wavetable_positions: [super::wavetable::PositionView; crate::wavetable::OSCILLATORS],
    #[serde(default)]
    pub oscillator_positions: [super::wavetable::PositionView; crate::oscillators::SLOTS],
    #[serde(default)]
    pub modulated_oscillators: [crate::oscillators::Controls; crate::oscillators::SLOTS],
    #[serde(default)]
    pub modulated_oscillator_motion: crate::wavetable::motion::Motion,
    #[serde(default)]
    pub modulated_interactions: crate::interaction::Controls,
    #[serde(default)]
    pub dropped_grain_births: u64,
    #[serde(default)]
    pub modulated_lfos: [crate::preset::Lfo; crate::preset::MODULATORS],
    #[serde(default)]
    pub modulated_envs: [crate::preset::Env; crate::preset::MODULATORS],
    pub matrix: [f32; crate::matrix::MAX_ROUTES],
    /// Held notes, one per note voice.
    pub notes: [u8; super::VOICES - 1],
    pub note_count: usize,
    #[serde(skip, default = "empty_grains")]
    pub grains: [super::GrainView; super::MAX_GRAINS],
    pub grain_count: usize,
    #[serde(skip, default = "empty_waveform")]
    pub waveform: [[f32; 2]; 640],
    pub waveform_len: usize,
    pub params: crate::preset::Params,
    /// Omitted when the current controls match the saved preset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modulated_filter_slots: Option<FilterSlotsView>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modulated_grain_groups: Option<crate::grain_groups::Settings>,
    pub filter_cutoffs: [f32; 2],
    pub resynth_bands: [f32; crate::resynth::BANDS],
    pub resynth_frequencies: [f32; crate::resynth::BANDS],
    pub source_fx_pending: bool,
    pub source_fx_error: bool,
    pub source_gain_reduction: (u32, f32),
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_color: Option<crate::color::ColorSlot>,
    #[serde(skip)]
    pub color: [crate::color::ColorSlot; crate::color::MAX_COLOR_SLOTS],
    pub color_len: usize,
    #[serde(skip)]
    pub pre_color: [crate::color::ColorSlot; crate::color::MAX_COLOR_SLOTS],
    pub pre_color_len: usize,
    pub gain_reduction: [(u32, f32); crate::color::MAX_COLOR_SLOTS * 2],
    pub step: Option<usize>,
    pub arp_index: usize,
    pub gate: bool,
    pub beat: f64,
    pub chord: [u8; 32],
    pub chord_len: usize,
    pub step_entry: Option<usize>,
    pub step_edits: [Option<crate::performance::Step>; 16],
    /// A Rack's source chains (`rack::STRIP_SOURCES`), loudest since the
    /// last view.
    #[serde(default)]
    pub rack_peaks: [f32; crate::rack::STRIPS],
    /// A Rack's EQs as modulated, by number less one. A browser snapshot
    /// carries those the rack holds beside the view, as it does color slots.
    #[serde(skip)]
    pub modulated_rack_eqs: [crate::eq::Equalizer; crate::eq::MAX_EQS],
    #[serde(default)]
    pub master_limiter: super::limiter::Meter,
    #[serde(default)]
    pub tb303: crate::tb303::View,
    #[serde(default)]
    pub modulated_tb303: crate::tb303::Controls,
    #[serde(default)]
    pub modulated_tb303_radias: crate::synth::Settings,
}

impl Default for RealtimeView {
    fn default() -> Self {
        Self {
            sequencer: crate::sequence::View::default(),
            drums: crate::drums::View::default(),
            drum_controls: crate::drums::Controls::default(),
            peak: [0.0; 2],
            input_peak: [0.0; 2],
            input_seconds: 0.0,
            center: 0.0,
            bar_beats: 4.0,
            playing: false,
            midi_gate: true,
            frozen: false,
            bypass: false,
            lfos: [0.0; crate::preset::MODULATORS],
            envs: [0.0; crate::preset::MODULATORS],
            modulated_resynth: Default::default(),
            modulated_synth: Default::default(),
            modulated_wavetable: crate::wavetable::default_controls(),
            modulated_wavetable_motion: Default::default(),
            wavetable_positions: Default::default(),
            oscillator_positions: Default::default(),
            modulated_oscillators: [crate::oscillators::Controls::default();
                crate::oscillators::SLOTS],
            modulated_oscillator_motion: Default::default(),
            modulated_interactions: Default::default(),
            dropped_grain_births: 0,
            modulated_lfos: [crate::preset::Lfo::default(); crate::preset::MODULATORS],
            modulated_envs: [crate::preset::Env::default(); crate::preset::MODULATORS],
            matrix: [0.0; crate::matrix::MAX_ROUTES],
            notes: [0; super::VOICES - 1],
            note_count: 0,
            grains: [super::GrainView::default(); super::MAX_GRAINS],
            grain_count: 0,
            waveform: [[0.0; 2]; 640],
            waveform_len: 0,
            params: crate::preset::Params::default(),
            modulated_filter_slots: None,
            modulated_grain_groups: None,
            filter_cutoffs: [1.0; 2],
            resynth_bands: [0.0; crate::resynth::BANDS],
            resynth_frequencies: [0.0; crate::resynth::BANDS],
            source_fx_pending: false,
            source_fx_error: false,
            source_gain_reduction: (0, 0.0),
            source_color: None,
            color: [crate::color::ColorSlot::default(); crate::color::MAX_COLOR_SLOTS],
            color_len: 0,
            pre_color: [crate::color::ColorSlot::default(); crate::color::MAX_COLOR_SLOTS],
            pre_color_len: 0,
            gain_reduction: [(0, 0.0); crate::color::MAX_COLOR_SLOTS * 2],
            step: None,
            arp_index: 0,
            gate: false,
            beat: 0.0,
            chord: [0; 32],
            chord_len: 0,
            step_entry: None,
            step_edits: [None; 16],
            rack_peaks: [0.0; crate::rack::STRIPS],
            modulated_rack_eqs: Default::default(),
            master_limiter: Default::default(),
            tb303: Default::default(),
            modulated_tb303: Default::default(),
            modulated_tb303_radias: Default::default(),
        }
    }
}

fn empty_waveform() -> [[f32; 2]; 640] {
    [[0.0; 2]; 640]
}

fn empty_grains() -> [super::GrainView; super::MAX_GRAINS] {
    [super::GrainView::default(); super::MAX_GRAINS]
}

#[derive(Clone, Copy, Debug, Serialize)]
pub struct KnobState {
    pub base: f32,
    pub current: f32,
    pub min: f32,
    pub max: f32,
    pub modulated: bool,
}
impl KnobState {
    fn new(base: f32, current: f32, min: f32, max: f32) -> Self {
        let min = min.min(current).min(base);
        let max = max.max(current).max(base);
        Self {
            base,
            current,
            min,
            max,
            modulated: max - min > 0.00001,
        }
    }
}

fn source_range(preset: &crate::preset::Preset, source: Source) -> [f32; 2] {
    match source {
        _ if source.lfo().is_some_and(|i| preset.lfos[i].unipolar) => [0.0, 1.0],
        Source::Lfo1
        | Source::Lfo2
        | Source::Lfo3
        | Source::Lfo4
        | Source::Lfo5
        | Source::Lfo6
        | Source::Lfo7
        | Source::Lfo8
        | Source::KeyTrack
        | Source::PitchBend
        | Source::RandomNote => [-1.0, 1.0],
        _ => [0.0, 1.0],
    }
}
fn product_range(a: [f32; 2], b: [f32; 2]) -> [f32; 2] {
    let values = [a[0] * b[0], a[0] * b[1], a[1] * b[0], a[1] * b[1]];
    [
        values.into_iter().fold(f32::INFINITY, f32::min),
        values.into_iter().fold(f32::NEG_INFINITY, f32::max),
    ]
}
fn apply(low: &mut Modulated, high: &mut Modulated, destination: Destination, range: [f32; 2]) {
    low.route(destination, range[0]);
    high.route(destination, range[1]);
}

impl Engine {
    pub fn realtime_view(&mut self, view: &mut RealtimeView) {
        view.drums = self.drum_view();
        view.drum_controls = self.modulated.drums;
        view.sequencer = self
            .preset_runtime
            .sequence
            .as_ref()
            .map_or_else(crate::sequence::View::default, |r| r.view());
        view.peak = self.peak;
        self.peak = [0.0; 2];
        view.master_limiter = self.master_meter();
        view.tb303 = self.tb303_view();
        view.modulated_tb303 = self.modulated.tb303;
        view.modulated_tb303_radias = self.modulated.tb303_radias;
        self.limiter.clear_peak();
        view.input_peak = self.live_input.as_ref().map_or([0.0; 2], |i| i.peak);
        view.input_seconds = self.live_input.as_ref().map_or(0.0, |i| i.seconds());
        view.center = self.source_position(self.center());
        view.bar_beats = self.bar_beats();
        view.playing = self.playing;
        view.midi_gate = self.midi_gate;
        view.frozen = self.frozen;
        view.bypass = self.bypass;
        view.lfos = self.lfo_values;
        view.envs = self.env_values;
        view.matrix = self.matrix_values;
        view.params = self.modulated.params;
        view.modulated_filter_slots = if self.modulated.filter_slots[..self.modulated.filter_len]
            != self.preset.filter_slots[..]
        {
            Some(FilterSlotsView {
                slots: self.modulated.filter_slots,
                len: self.modulated.filter_len,
            })
        } else {
            None
        };
        view.modulated_grain_groups = (self.modulated.grain_groups != self.preset.grain_groups)
            .then_some(self.modulated.grain_groups);
        view.modulated_resynth = self.modulated.resynth;
        view.modulated_synth = self.modulated.synth;
        view.modulated_wavetable = self.modulated.wavetable;
        view.modulated_wavetable_motion = self.modulated.wavetable_motion;
        view.oscillator_positions = self
            .oscillator_engine
            .mixed
            .as_ref()
            .map_or([Default::default(); crate::oscillators::SLOTS], |m| {
                m.positions()
            });
        view.modulated_oscillators = self.modulated.oscillator_controls;
        view.modulated_oscillator_motion = self.modulated.oscillator_motion;
        view.modulated_interactions = self.modulated.interaction_controls;
        view.dropped_grain_births = self.oscillator_engine.graph.dropped_births;
        view.wavetable_positions = self
            .oscillator_engine
            .wavetable
            .as_ref()
            .map_or_else(Default::default, |w| w.positions());
        view.modulated_lfos = self.modulated.lfos;
        view.modulated_envs = self.modulated.envs;
        view.modulated_rack_eqs = self.modulated.rack_eqs;
        view.filter_cutoffs = self.current_filter_cutoffs();
        view.resynth_bands = self.oscillator_engine.resynth.carrier_levels();
        view.resynth_frequencies = self.oscillator_engine.resynth.frequencies();
        view.source_fx_pending = self.source_effects_pending();
        view.source_fx_error = self.source_fx_error;
        view.source_gain_reduction = self
            .preset
            .source_color_chain
            .first()
            .map_or((0, 0.0), |s| {
                (
                    s.id,
                    if self.live_input.is_some() {
                        self.source_color.gain_reduction_db()
                    } else {
                        0.0
                    },
                )
            });
        view.color = self.modulated.color_slots;
        view.color_len = self.modulated.color_len;
        view.pre_color = self.modulated.pre_color_slots;
        view.pre_color_len = self.modulated.pre_color_len;
        view.source_color = self.modulated.source_color;
        let split = crate::color::MAX_COLOR_SLOTS;
        view.gain_reduction[..split].copy_from_slice(&self.color_rack.realtime_gain_reduction());
        view.gain_reduction[split..]
            .copy_from_slice(&self.pre_color_rack.realtime_gain_reduction());
        if !self.playing || self.bypass {
            view.gain_reduction.iter_mut().for_each(|(_, db)| *db = 0.0);
        }
        view.note_count = 0;
        for voice in self.voices[1..].iter().filter(|v| v.held || v.sustained) {
            view.notes[view.note_count] = voice.note;
            view.note_count += 1;
        }
        if let Some((_, note, _)) = self
            .oscillator_engine
            .tb303
            .as_ref()
            .and_then(|b| b.note_state())
            && view.note_count < view.notes.len()
            && !view.notes[..view.note_count].contains(&note)
        {
            view.notes[view.note_count] = note;
            view.note_count += 1;
        }
        view.grain_count = 0;
        for slot in self.live_grain_slots() {
            view.grains[view.grain_count] = self.grain_view(slot);
            view.grain_count += 1;
        }
        let waveform = &self.shown_sample().waveform;
        view.waveform_len = waveform.len().min(view.waveform.len());
        view.waveform[..view.waveform_len].copy_from_slice(&waveform[..view.waveform_len]);
        (view.step, view.arp_index, view.gate, view.beat) = self.performance.realtime_position();
        let chord = &self.preset.performance.chord;
        view.chord_len = chord.len().min(32);
        view.chord[..view.chord_len].copy_from_slice(&chord[..view.chord_len]);
        view.step_entry = self.step_entry;
        view.step_edits = self.step_edits;
        view.rack_peaks = self.rack_peaks().unwrap_or_default();
        self.clear_rack_peaks();
        if let Some(input) = &mut self.live_input {
            input.peak = [0.0; 2];
        }
    }

    pub fn gain_reduction(&self) -> BTreeMap<u32, f32> {
        let mut meters = self.color_rack.gain_reduction();
        meters.extend(self.pre_color_rack.gain_reduction());
        if !self.playing || self.bypass {
            meters.values_mut().for_each(|value| *value = 0.0);
        }
        meters
    }

    pub fn knob_states(&self) -> BTreeMap<String, KnobState> {
        ControlFeedback {
            preset: &self.preset,
        }
        .knob_states_at(&self.modulated, self.current_filter_cutoffs())
    }

    pub fn knob_states_from_view(&self, view: &RealtimeView) -> BTreeMap<String, KnobState> {
        knob_states_from_view(&self.preset, view)
    }

    fn current_filter_cutoffs(&self) -> [f32; 2] {
        let m = self.modulated.params;
        let voice = self
            .voices
            .iter()
            .enumerate()
            .find(|(_, v)| {
                v.active() && v.note == self.source_note && v.channel == self.source_channel
            })
            .or_else(|| self.voices.iter().enumerate().find(|(_, v)| v.active()));
        let (note, envs, bass_env) = voice.map_or(
            (
                60.0,
                [0.0; crate::preset::MODULATORS],
                self.tb303_view().envelope,
            ),
            |(index, v)| {
                (
                    v.pitch() + self.bend[v.channel as usize] * m.midi_pitchbend_range,
                    v.env_values,
                    self.voice_tb303_envelope(index),
                )
            },
        );
        filter::voice_cutoffs(&m, note, filter::envelopes(envs, bass_env))
    }
}

struct ControlFeedback<'a> {
    preset: &'a crate::preset::Preset,
}
impl ControlFeedback<'_> {
    fn control_limits(&self, source_realtime: bool) -> (Modulated, Modulated) {
        let mut low = Modulated::from_preset(self.preset);
        if !source_realtime {
            low.source_color = None;
        }
        let mut high = low;
        // Equal and opposite routes from the same source cancel before bounds.
        let mut terms: Vec<(Destination, Source, f32)> = Vec::new();
        for route in self
            .preset
            .matrix
            .iter()
            .filter(|r| r.enabled && r.depth != 0.0 && r.destination != Destination::None)
        {
            if let Some(term) = terms
                .iter_mut()
                .find(|(d, s, _)| *d == route.destination && *s == route.source)
            {
                term.2 += route.depth;
            } else {
                terms.push((route.destination, route.source, route.depth));
            }
        }
        let mut ranges: Vec<(Destination, [f32; 2])> = Vec::new();
        // A macro reaches anywhere its knob can turn it.
        let macros = self.preset.macros.iter().flat_map(|m| {
            m.targets
                .iter()
                .filter(|t| t.depth != 0.0 && t.destination != Destination::None)
                .map(|t| (t.destination, product_range([0.0, 1.0], [t.depth; 2])))
        });
        for (destination, delta) in terms
            .into_iter()
            .map(|(d, source, depth)| {
                (
                    d,
                    product_range(source_range(self.preset, source), [depth; 2]),
                )
            })
            .chain(macros)
        {
            if let Some((_, range)) = ranges.iter_mut().find(|(d, _)| *d == destination) {
                range[0] += delta[0];
                range[1] += delta[1];
            } else {
                ranges.push((destination, delta));
            }
        }
        for (destination, range) in ranges {
            apply(&mut low, &mut high, destination, range);
        }
        // Match update_modulation(): routes apply in modulator order, each
        // over the depth range the routes before it left.
        for modulator in low.modulator_order() {
            let source = match modulator {
                Modulator::Lfo(i) if low.lfos[i].unipolar => [0.0, 1.0],
                Modulator::Lfo(_) => [-0.5, 0.5],
                Modulator::Env(_) => [0.0, 1.0],
            };
            for r in 0..MODULATOR_ROUTES {
                let (destination, from) = low.modulator_routes(modulator)[r];
                let to = high.modulator_routes(modulator)[r].1;
                let range = product_range(source, [from, to]);
                apply(&mut low, &mut high, destination, range);
            }
        }
        (low, high)
    }
    fn knob_states_at(
        &self,
        modulated: &Modulated,
        current: [f32; 2],
    ) -> BTreeMap<String, KnobState> {
        let (low, high) = self.control_limits(modulated.source_color.is_some());
        let p = self.preset.params;
        let m = modulated.params;
        let mut result = BTreeMap::new();
        macro_rules! parameter {($($field:ident),* $(,)?)=>{$(
            result.insert(stringify!($field).into(),KnobState::new(p.$field,m.$field,low.params.$field,high.params.$field));
        )*};}
        parameter!(
            position,
            grains,
            grain_size,
            pitch,
            grain_shape,
            cloud,
            detune,
            stereo_spread,
            filter_resonance,
            filter_env_depth,
            filter_keytrack,
            filter_drive,
            filter_morph,
            filter2_resonance,
            filter2_env_depth,
            filter2_keytrack,
            filter2_drive,
            filter2_morph,
            mix,
            master_volume
        );

        // Cutoff includes each voice's dedicated envelope and keyboard tracking.
        // A single marker follows the latest sounding note, as do note sources.
        let mut lo = low.params;
        let hi = high.params;
        // Negative notes give the minimum at maximum nonnegative key tracking.
        lo.filter_keytrack = hi.filter_keytrack;
        lo.filter2_keytrack = hi.filter2_keytrack;
        let below = filter::voice_cutoffs(
            &lo,
            -hi.midi_pitchbend_range,
            [0.0; crate::preset::MODULATORS + 1],
        );
        let below_env = filter::voice_cutoffs(
            &lo,
            -hi.midi_pitchbend_range,
            [1.0; crate::preset::MODULATORS + 1],
        );
        let above = filter::voice_cutoffs(
            &hi,
            127.0 + hi.midi_pitchbend_range,
            [0.0; crate::preset::MODULATORS + 1],
        );
        let above_env = filter::voice_cutoffs(
            &hi,
            127.0 + hi.midi_pitchbend_range,
            [1.0; crate::preset::MODULATORS + 1],
        );
        for (i, key, base) in [(0, "filter", p.filter), (1, "filter2", p.filter2)] {
            result.insert(
                key.into(),
                KnobState::new(
                    base,
                    current[i],
                    below[i].min(below_env[i]),
                    above[i].max(above_env[i]),
                ),
            );
        }
        for (slots, current_slots, low_slots, high_slots) in [
            (
                self.preset.color_chain.as_slice(),
                modulated.color_slots.as_slice(),
                low.color_slots.as_slice(),
                high.color_slots.as_slice(),
            ),
            (
                self.preset.pre_color_chain.as_slice(),
                modulated.pre_color_slots.as_slice(),
                low.pre_color_slots.as_slice(),
                high.pre_color_slots.as_slice(),
            ),
            (
                self.preset.source_color_chain.as_slice(),
                modulated.source_color.as_slice(),
                low.source_color.as_slice(),
                high.source_color.as_slice(),
            ),
        ] {
            for (i, slot) in slots.iter().enumerate() {
                for &parameter in slot.control_parameters() {
                    let mut base = *slot;
                    let mut current = current_slots.get(i).copied().unwrap_or(*slot);
                    let mut min = low_slots.get(i).copied().unwrap_or(*slot);
                    let mut max = high_slots.get(i).copied().unwrap_or(*slot);
                    let key = format!("color:{}:{}", slot.id, parameter.key());
                    let state = KnobState::new(
                        *base.parameter_mut(parameter),
                        *current.parameter_mut(parameter),
                        *min.parameter_mut(parameter),
                        *max.parameter_mut(parameter),
                    );
                    result.insert(key, state);
                }
            }
        }
        let base_frame = Modulated::from_preset(self.preset);
        // A Rack's EQs report only for those in it.
        let equalizers: Vec<String> = self
            .preset
            .rack
            .iter()
            .flat_map(|r| &r.equalizers)
            .map(|eq| format!("rack_eq{}_", eq.id))
            .collect();
        for spec in crate::parameters::specs().iter().filter(|s| {
            s.id.starts_with("resynth_")
                || s.id.starts_with("lfo")
                || s.id.starts_with("env")
                || (self.preset.drums.is_some() && s.id.starts_with("drum"))
                || (self.preset.synth.is_some() && s.id.starts_with("synth_"))
                || (self.preset.wavetable.is_some() && s.id.starts_with("wt_"))
                || (self.preset.oscillators.is_some() && s.id.starts_with("osc"))
                || (s.id.starts_with("filter") && s.id.contains("_ws_"))
                || (s.id.starts_with("grain_group") && s.id.contains("_waveshaper_"))
                || s.id
                    .strip_prefix("interaction")
                    .and_then(|tail| tail.split('_').next()?.parse::<usize>().ok())
                    .and_then(|n| n.checked_sub(1))
                    .and_then(|n| self.preset.interactions.routes.get(n))
                    .is_some_and(|r| {
                        r.source != crate::interaction::Node::None
                            || r.target != crate::interaction::Node::None
                    })
                || (self.preset.tb303.is_some() && s.id.starts_with("tb303_"))
                || equalizers.iter().any(|prefix| s.id.starts_with(prefix))
        }) {
            if let (Some(base), Some(current), Some(min), Some(max)) = (
                spec.frame_get(&base_frame),
                spec.frame_get(modulated),
                spec.frame_get(&low),
                spec.frame_get(&high),
            ) {
                result
                    .entry(spec.id.clone())
                    .or_insert_with(|| KnobState::new(base, current, min, max));
            }
        }
        result
    }
}
pub fn knob_states_from_view(
    preset: &crate::preset::Preset,
    view: &RealtimeView,
) -> BTreeMap<String, KnobState> {
    let mut m = Modulated::from_preset(preset);
    m.params = view.params;
    if let Some(slots) = view.modulated_filter_slots {
        m.filter_slots = slots.slots;
        m.filter_len = slots.len;
    }
    if let Some(groups) = view.modulated_grain_groups {
        m.grain_groups = groups;
    }
    m.color_slots = view.color;
    m.pre_color_slots = view.pre_color;
    m.source_color = view.source_color;
    m.resynth = view.modulated_resynth;
    m.synth = view.modulated_synth;
    m.wavetable = view.modulated_wavetable;
    m.wavetable_motion = view.modulated_wavetable_motion;
    m.oscillator_controls = view.modulated_oscillators;
    m.oscillator_motion = view.modulated_oscillator_motion;
    m.interaction_controls = view.modulated_interactions;
    m.tb303 = view.modulated_tb303;
    m.tb303_radias = view.modulated_tb303_radias;
    m.lfos = view.modulated_lfos;
    m.envs = view.modulated_envs;
    m.drums = view.drum_controls;
    m.drum_shifts = view.drums.shifts.map(|s| s as f32);
    m.rack_eqs = view.modulated_rack_eqs;
    ControlFeedback { preset }.knob_states_at(&m, view.filter_cutoffs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        dsp::SampleBuffer,
        macros::{Macro, Target},
        parameters,
        preset::{FilterSlot, Preset},
    };
    use std::sync::Arc;

    fn control(id: &str) -> Destination {
        let index = parameters::specs()
            .iter()
            .position(|spec| spec.id == id)
            .unwrap();
        Destination::Control {
            index: u16::try_from(index).unwrap(),
        }
    }

    fn close(actual: f32, expected: f32) {
        assert!((actual - expected).abs() < 1e-5, "{actual} != {expected}");
    }

    #[test]
    fn shaper_modulation_feedback_survives_realtime_view_and_serialization() {
        let mut preset = Preset::default();
        preset.params.filter_shapers[0].before.depth = 0.2;
        let mut slot = FilterSlot {
            id: 77,
            ..FilterSlot::default()
        };
        slot.shapers.after.options.bias = 0.3;
        preset.filter_slots.push(slot);
        preset.grain_groups.groups[2].waveshaper_options.input = 0.25;
        let cases = [
            ("filter1_ws_before_depth", 0.4, 0.2, 0.4, 0.2, 0.6),
            ("filter3_ws_after_bias", -0.2, 0.3, 0.2, 0.1, 0.3),
            ("grain_group3_waveshaper_input", 0.6, 0.25, 0.55, 0.25, 0.85),
        ];
        preset.macros.push(Macro {
            value: 0.5,
            targets: cases
                .iter()
                .map(|(id, depth, ..)| Target {
                    destination: control(id),
                    depth: *depth,
                })
                .collect(),
            ..Macro::default()
        });
        let mut engine = Engine::new(8_000, Arc::new(SampleBuffer::demo(8_000)), preset);
        let direct = engine.knob_states();
        let mut view = RealtimeView::default();
        engine.realtime_view(&mut view);
        let packet = serde_json::to_value(&view).unwrap();
        assert_eq!(
            packet["modulated_filter_slots"].as_array().unwrap().len(),
            1
        );
        let decoded: RealtimeView = serde_json::from_value(packet.clone()).unwrap();
        let published = engine.knob_states_from_view(&decoded);
        for (id, _, base, current, min, max) in cases {
            for state in [&direct[id], &published[id]] {
                close(state.base, base);
                close(state.current, current);
                close(state.min, min);
                close(state.max, max);
                assert!(state.modulated, "{id}");
            }
        }
        assert!(!direct.contains_key("filter4_ws_before_depth"));

        // Older snapshots omit the new fields and fall back to saved values.
        let mut old_packet = packet;
        let old = old_packet.as_object_mut().unwrap();
        old.remove("modulated_filter_slots");
        old.remove("modulated_grain_groups");
        let old: RealtimeView = serde_json::from_value(old_packet).unwrap();
        let old = engine.knob_states_from_view(&old);
        close(old["filter3_ws_after_bias"].current, 0.3);
        close(old["grain_group3_waveshaper_input"].current, 0.25);

        // Reusing the view must not retain an earlier modulated snapshot.
        engine.preset.macros[0].value = 0.0;
        engine.refresh_parameters();
        engine.realtime_view(&mut view);
        assert!(view.modulated_filter_slots.is_none());
        assert!(view.modulated_grain_groups.is_none());
        let packet = serde_json::to_value(&view).unwrap();
        assert!(packet.get("modulated_filter_slots").is_none());
        assert!(packet.get("modulated_grain_groups").is_none());
        let decoded: RealtimeView = serde_json::from_value(packet).unwrap();
        let published = engine.knob_states_from_view(&decoded);
        close(published["filter3_ws_after_bias"].current, 0.3);
        close(published["grain_group3_waveshaper_input"].current, 0.25);
    }
}
