//! Read-only control feedback. Reuse the DSP's destination arithmetic, without
//! running processors or changing audio state. Called at UI publication time.
use super::{Engine, Modulated, filter};
use crate::{matrix::Source, preset::Destination};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

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
    pub filter_cutoffs: [f32; 2],
    pub resynth_bands: [f32; crate::resynth::BANDS],
    pub resynth_frequencies: [f32; crate::resynth::BANDS],
    pub source_fx_pending: bool,
    pub source_fx_error: bool,
    pub source_gain_reduction: (u32, f32),
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
            filter_cutoffs: [1.0; 2],
            resynth_bands: [0.0; crate::resynth::BANDS],
            resynth_frequencies: [0.0; crate::resynth::BANDS],
            source_fx_pending: false,
            source_fx_error: false,
            source_gain_reduction: (0, 0.0),
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

fn source_range(source: Source) -> [f32; 2] {
    match source {
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
            .preset
            .sequencer
            .runtime
            .as_ref()
            .map_or_else(crate::sequence::View::default, |r| r.view());
        view.peak = self.peak;
        self.peak = [0.0; 2];
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
        view.modulated_resynth = self.modulated.resynth;
        view.modulated_synth = self.modulated.synth;
        view.modulated_lfos = self.modulated.lfos;
        view.modulated_envs = self.modulated.envs;
        view.filter_cutoffs = self.current_filter_cutoffs();
        view.resynth_bands = self.resynth.levels();
        view.resynth_frequencies = self.resynth.frequencies();
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
        view.grain_count = 0;
        for grain in self.live_grain_list() {
            view.grains[view.grain_count] = super::GrainView {
                position: self.source_position(grain.position) as f32,
                age: grain.age as f32 / grain.duration as f32,
                pan: (grain.pan[1] - grain.pan[0] + 1.0) * 0.5,
            };
            view.grain_count += 1;
        }
        let waveform = &self.source_sample().waveform;
        view.waveform_len = waveform.len().min(view.waveform.len());
        view.waveform[..view.waveform_len].copy_from_slice(&waveform[..view.waveform_len]);
        (view.step, view.arp_index, view.gate, view.beat) = self.performance.realtime_position();
        let chord = &self.preset.performance.chord;
        view.chord_len = chord.len().min(32);
        view.chord[..view.chord_len].copy_from_slice(&chord[..view.chord_len]);
        view.step_entry = self.step_entry;
        view.step_edits = self.step_edits;
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
            .find(|v| v.active() && v.note == self.source_note && v.channel == self.source_channel)
            .or_else(|| self.voices.iter().find(|v| v.active()));
        let (note, envs) = voice.map_or((60.0, [0.0; crate::preset::MODULATORS]), |v| {
            (
                v.note as f32 + self.bend[v.channel as usize] * m.midi_pitchbend_range,
                v.env_values,
            )
        });
        filter::voice_cutoffs(&m, note, envs)
    }
}

struct ControlFeedback<'a> {
    preset: &'a crate::preset::Preset,
}
impl ControlFeedback<'_> {
    fn control_limits(&self) -> (Modulated, Modulated) {
        let mut low = Modulated::from_preset(self.preset);
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
            .map(|(d, source, depth)| (d, product_range(source_range(source), [depth; 2])))
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
        // Match the order and snapshot boundaries in update_modulation().
        let low_lfos = low.lfos;
        let high_lfos = high.lfos;
        for (i, lfo) in self
            .preset
            .lfos
            .iter()
            .take(self.preset.lfo_count as usize)
            .enumerate()
        {
            let (lows, highs) = (low_lfos[i].routes(), high_lfos[i].routes());
            for (r, (destination, _)) in lfo.routes().into_iter().enumerate() {
                let range = product_range([-0.5, 0.5], [lows[r].1, highs[r].1]);
                apply(&mut low, &mut high, destination, range);
            }
        }
        let low_envs = low.envs;
        let high_envs = high.envs;
        for (i, env) in self
            .preset
            .envs
            .iter()
            .take(self.preset.env_count as usize)
            .enumerate()
        {
            let (lows, highs) = (low_envs[i].routes(), high_envs[i].routes());
            for (r, (destination, _)) in env.routes().into_iter().enumerate() {
                let range = product_range([0.0, 1.0], [lows[r].1, highs[r].1]);
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
        let (low, high) = self.control_limits();
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
        let below = filter::voice_cutoffs(&lo, -hi.midi_pitchbend_range, [0.0; 3]);
        let below_env = filter::voice_cutoffs(&lo, -hi.midi_pitchbend_range, [1.0; 3]);
        let above = filter::voice_cutoffs(&hi, 127.0 + hi.midi_pitchbend_range, [0.0; 3]);
        let above_env = filter::voice_cutoffs(&hi, 127.0 + hi.midi_pitchbend_range, [1.0; 3]);
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
                &self.preset.color_chain,
                &modulated.color_slots,
                &low.color_slots,
                &high.color_slots,
            ),
            (
                &self.preset.pre_color_chain,
                &modulated.pre_color_slots,
                &low.pre_color_slots,
                &high.pre_color_slots,
            ),
        ] {
            for (i, slot) in slots.iter().enumerate() {
                for &parameter in slot.control_parameters() {
                    let mut base = *slot;
                    let mut current = current_slots[i];
                    let mut min = low_slots[i];
                    let mut max = high_slots[i];
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
        for slot in &self.preset.source_color_chain {
            for &parameter in slot.control_parameters() {
                let base = *slot.to_owned().parameter_mut(parameter);
                result.insert(
                    format!("color:{}:{}", slot.id, parameter.key()),
                    KnobState::new(base, base, base, base),
                );
            }
        }
        let base_frame = Modulated::from_preset(self.preset);
        for spec in crate::parameters::specs().iter().filter(|s| {
            s.id.starts_with("resynth_")
                || s.id.starts_with("lfo")
                || s.id.starts_with("env")
                || (self.preset.drums.is_some() && s.id.starts_with("drum"))
                || (self.preset.synth.is_some() && s.id.starts_with("synth_"))
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
    m.color_slots = view.color;
    m.pre_color_slots = view.pre_color;
    m.resynth = view.modulated_resynth;
    m.synth = view.modulated_synth;
    m.lfos = view.modulated_lfos;
    m.envs = view.modulated_envs;
    m.drums = view.drum_controls;
    m.drum_shifts = view.drums.shifts.map(|s| s as f32);
    ControlFeedback { preset }.knob_states_at(&m, view.filter_cutoffs)
}
