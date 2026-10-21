//! Shared numeric controls. Host IDs and their order remain stable across builds.
use crate::{
    color::{ColorParameter, ColorSlot, MAX_COLOR_SLOTS},
    preset::Preset,
};
use serde::Serialize;
use std::sync::OnceLock;

#[derive(Clone, Copy, Debug)]
enum Address {
    Host(usize),
    MatrixDepth(usize),
}

/// Validated off-thread; applying a change only writes a numeric field.
#[derive(Clone, Copy, Debug)]
pub struct Change {
    address: Address,
    value: f32,
}

impl Change {
    pub fn host_index(self) -> Option<usize> {
        match self.address {
            Address::Host(index) => Some(index),
            Address::MatrixDepth(_) => None,
        }
    }

    fn apply(self, preset: &mut Preset) -> bool {
        match self.address {
            Address::Host(index) => {
                let spec = &specs()[index];
                let before = spec.get(preset);
                spec.set(preset, self.value);
                before != spec.get(preset)
            }
            Address::MatrixDepth(index) => {
                if let Some(route) = preset.matrix.get_mut(index) {
                    let changed = route.depth != self.value;
                    route.depth = self.value;
                    changed
                } else {
                    false
                }
            }
        }
    }
}

/// Decode a small {parameter_id: value} message before touching the audio owner.
/// The whole message is validated before any of its changes are applied.
pub fn parse(data: &serde_json::Value) -> Result<Vec<Change>, String> {
    let values = data.as_object().ok_or("Expected numeric parameters")?;
    if values.len() > specs().len() + crate::matrix::MAX_ROUTES {
        return Err("Too many parameter changes".into());
    }
    values
        .iter()
        .map(|(id, value)| {
            let value = value
                .as_f64()
                .filter(|v| v.is_finite() && (*v as f32).is_finite())
                .ok_or_else(|| format!("Invalid value for {id}"))? as f32;
            if let Some(index) = specs().iter().position(|spec| spec.id == *id) {
                let spec = &specs()[index];
                return Ok(Change {
                    address: Address::Host(index),
                    value: value.clamp(spec.min, spec.max),
                });
            }
            if let Some(index) = id
                .strip_prefix("matrix")
                .and_then(|id| id.strip_suffix("_depth"))
                .and_then(|n| n.parse::<usize>().ok())
                .and_then(|n| n.checked_sub(1))
                .filter(|&n| n < crate::matrix::MAX_ROUTES)
            {
                return Ok(Change {
                    address: Address::MatrixDepth(index),
                    value: value.clamp(-1.0, 1.0),
                });
            }
            Err(format!("Unknown parameter: {id}"))
        })
        .collect()
}

pub fn apply(preset: &mut Preset, changes: &[Change]) -> bool {
    let mut changed = false;
    for change in changes {
        changed |= change.apply(preset);
    }
    if changed {
        mirror(preset);
    }
    changed
}

#[derive(Clone, Copy)]
enum Target {
    Core {
        get: fn(&Preset) -> f32,
        set: fn(&mut Preset, f32),
        frame_get: fn(&crate::dsp::Modulated) -> f32,
        frame_set: fn(&mut crate::dsp::Modulated, f32),
    },
    Lfo(usize, usize),
    Env(usize, usize),
    Fx(bool, usize, ColorParameter),
    SourceFx(ColorParameter),
    Performance(usize),
    ResynthOsc(usize, usize),
    Drum(usize, usize),
    DrumGrain,
    DrumBuffer,
}

#[derive(Serialize)]
pub struct Spec {
    pub id: String,
    pub name: String,
    pub min: f32,
    pub max: f32,
    pub default: f32,
    #[serde(skip)]
    target: Target,
}

impl Spec {
    pub(crate) fn can_sequence(&self) -> bool {
        !matches!(self.target, Target::DrumGrain | Target::DrumBuffer)
            && self.id != "bpm"
            && !matches!(self.target, Target::SourceFx(_) | Target::Performance(3..))
    }
    pub(crate) fn frame_get(&self, m: &crate::dsp::Modulated) -> Option<f32> {
        Some(match self.target {
            Target::Core { frame_get, .. } => frame_get(m),
            Target::Drum(i, field) => m.drums.0[i][field],
            Target::SourceFx(_) | Target::DrumGrain | Target::DrumBuffer => return None,
            Target::ResynthOsc(i, field) => oscillator_get(m.resynth.oscillators[i], field),
            Target::Lfo(i, field) => match field {
                0 => m.lfos[i].rate_hz,
                1 => m.lfos[i].depth,
                _ => m.lfos[i].division,
            },
            Target::Env(i, field) => {
                let e = m.envs[i];
                match field {
                    0 => e.attack,
                    1 => e.decay,
                    2 => e.sustain,
                    3 => e.release,
                    4 => e.depth,
                    _ => e.extra_depth,
                }
            }
            Target::Fx(pre, i, field) => {
                let slots = if pre {
                    &m.pre_color_slots[..m.pre_color_len]
                } else {
                    &m.color_slots[..m.color_len]
                };
                *slots.get(i)?.to_owned().parameter_mut(field)
            }
            Target::Performance(field) => match field {
                0 => m.performance.arp_rate,
                1 => m.performance.arp_gate,
                2 => m.performance.arp_octaves,
                3 => m.performance.sequence_rate,
                4 => m.performance.sequence_gate,
                _ => m.performance.sequence_length,
            },
        })
    }
    pub(crate) fn frame_modulate(&self, m: &mut crate::dsp::Modulated, amount: f32) {
        if let Target::Fx(pre, index, ColorParameter::LcrRatio) = self.target {
            let slots = if pre {
                &mut m.pre_color_slots[..m.pre_color_len]
            } else {
                &mut m.color_slots[..m.color_len]
            };
            if let Some(slot) = slots.get_mut(index) {
                slot.settings.lcr_ratio = crate::radias_delay::modulate_lcr_ratio(
                    slot.settings.lcr_sync,
                    slot.settings.lcr_ratio,
                    amount,
                );
            }
        } else if let Some(base) = self.frame_get(m) {
            self.frame_set(m, base + amount * (self.max - self.min));
        }
    }
    pub(crate) fn frame_set(&self, m: &mut crate::dsp::Modulated, value: f32) {
        if !value.is_finite() {
            return;
        }
        let value = value.clamp(self.min, self.max);
        match self.target {
            Target::Core { frame_set, .. } => frame_set(m, value),
            Target::Drum(i, field) => {
                m.drums.0[i][field] =
                    sad_pet_dxm088::params::constrain(field, value).unwrap_or(self.default);
            }
            Target::SourceFx(_) | Target::DrumGrain | Target::DrumBuffer => {}
            Target::ResynthOsc(i, field) => {
                oscillator_set(&mut m.resynth.oscillators[i], field, value)
            }
            Target::Lfo(i, field) => {
                *match field {
                    0 => &mut m.lfos[i].rate_hz,
                    1 => &mut m.lfos[i].depth,
                    _ => &mut m.lfos[i].division,
                } = value
            }
            Target::Env(i, field) => {
                let e = &mut m.envs[i];
                *match field {
                    0 => &mut e.attack,
                    1 => &mut e.decay,
                    2 => &mut e.sustain,
                    3 => &mut e.release,
                    4 => &mut e.depth,
                    _ => &mut e.extra_depth,
                } = value;
            }
            Target::Fx(pre, i, field) => {
                let slots = if pre {
                    &mut m.pre_color_slots[..m.pre_color_len]
                } else {
                    &mut m.color_slots[..m.color_len]
                };
                if let Some(slot) = slots.get_mut(i) {
                    *slot.parameter_mut(field) = value;
                }
            }
            Target::Performance(field) => {
                *match field {
                    0 => &mut m.performance.arp_rate,
                    1 => &mut m.performance.arp_gate,
                    2 => &mut m.performance.arp_octaves,
                    3 => &mut m.performance.sequence_rate,
                    4 => &mut m.performance.sequence_gate,
                    _ => &mut m.performance.sequence_length,
                } = value
            }
        }
    }
    pub fn get(&self, preset: &Preset) -> f32 {
        match self.target {
            Target::DrumGrain => f32::from(preset.drums.as_ref().is_some_and(|d| d.granular)),
            Target::DrumBuffer => preset.drums.as_ref().map_or(8., |d| d.buffer_seconds),
            Target::Drum(i, field) => preset
                .drums
                .as_ref()
                .map_or(self.default, |d| d.params.0[i][field]),
            Target::SourceFx(field) => *preset
                .source_color_chain
                .first()
                .copied()
                .unwrap_or_default()
                .parameter_mut(field),
            Target::ResynthOsc(i, field) => {
                let o = preset.resynth.oscillators[i];
                match field {
                    0 => f32::from(o.enabled),
                    1 => o.waveform.code() as f32,
                    2 => o.mode.code() as f32,
                    3 => o.control1,
                    4 => o.control2,
                    5 => o.transpose,
                    6 => o.fine,
                    7 => o.level,
                    8 => o.pan,
                    9 => o.cross_source as f32,
                    10 => o.model as u8 as f32,
                    11 => o.fold_shape as u8 as f32,
                    12 => o.interaction as u8 as f32,
                    13 => o.interaction_source as f32,
                    _ => o.cross_input as u8 as f32,
                }
            }
            Target::Core { get, .. } => get(preset),
            Target::Lfo(i, field) => {
                let l = preset.lfos[i];
                match field {
                    0 => l.rate_hz,
                    1 => l.depth,
                    _ => l.division,
                }
            }
            Target::Env(i, field) => {
                let e = preset.envs[i];
                match field {
                    0 => e.attack,
                    1 => e.decay,
                    2 => e.sustain,
                    3 => e.release,
                    4 => e.depth,
                    _ => e.extra_depth,
                }
            }
            Target::Fx(pre, i, field) => *(if pre {
                &preset.pre_color_chain
            } else {
                &preset.color_chain
            })
            .get(i)
            .copied()
            .unwrap_or_default()
            .parameter_mut(field),
            Target::Performance(field) => {
                let p = &preset.performance;
                match field {
                    0 => p.arp_division,
                    1 => p.arp_gate,
                    2 => p.octaves as f32,
                    3 => p.sequence_division,
                    4 => p.sequence_gate,
                    _ => p.length as f32,
                }
            }
        }
    }
    pub fn set(&self, preset: &mut Preset, value: f32) {
        let value = if value.is_finite() {
            value.clamp(self.min, self.max)
        } else {
            self.default
        };
        match self.target {
            Target::DrumBuffer => {
                if let Some(d) = &mut preset.drums {
                    d.buffer_seconds = value;
                }
            }
            Target::DrumGrain => {
                if let Some(d) = &mut preset.drums {
                    d.granular = value >= 0.5;
                }
            }
            Target::Drum(i, field) => {
                if let Some(d) = &mut preset.drums {
                    d.params.0[i][field] =
                        sad_pet_dxm088::params::constrain(field, value).unwrap_or(self.default);
                }
            }
            Target::ResynthOsc(i, field) => {
                let o = &mut preset.resynth.oscillators[i];
                match field {
                    0 => o.enabled = value >= 0.5,
                    1 => o.waveform = crate::resynth::Waveform::ALL[value.round() as usize],
                    2 => o.mode = crate::resynth::Mode::ALL[value.round() as usize],
                    3 => o.control1 = value,
                    4 => o.control2 = value,
                    5 => o.transpose = value,
                    6 => o.fine = value,
                    7 => o.level = value,
                    8 => o.pan = value,
                    9 => o.cross_source = value.round() as u8,
                    10 => {
                        o.model = if value < 0.5 {
                            crate::resynth::Model::Radias
                        } else {
                            crate::resynth::Model::Wavefolder
                        }
                    }
                    11 => o.fold_shape = crate::resynth::FoldShape::ALL[value.round() as usize],
                    12 => o.interaction = crate::resynth::Interaction::ALL[value.round() as usize],
                    13 => o.interaction_source = value.round() as u8,
                    _ => o.cross_input = crate::resynth::CrossInput::ALL[value.round() as usize],
                }
            }
            Target::Core { set, .. } => set(preset, value),
            Target::Lfo(i, field) => {
                let l = &mut preset.lfos[i];
                *match field {
                    0 => &mut l.rate_hz,
                    1 => &mut l.depth,
                    _ => &mut l.division,
                } = value;
            }
            Target::Env(i, field) => {
                let e = &mut preset.envs[i];
                *match field {
                    0 => &mut e.attack,
                    1 => &mut e.decay,
                    2 => &mut e.sustain,
                    3 => &mut e.release,
                    4 => &mut e.depth,
                    _ => &mut e.extra_depth,
                } = value;
            }
            Target::Fx(pre, i, field) => {
                if let Some(slot) = (if pre {
                    &mut preset.pre_color_chain
                } else {
                    &mut preset.color_chain
                })
                .get_mut(i)
                {
                    *slot.parameter_mut(field) = value;
                }
            }
            Target::SourceFx(field) => {
                if let Some(slot) = preset.source_color_chain.first_mut() {
                    *slot.parameter_mut(field) = value;
                }
            }
            Target::Performance(field) => {
                let p = &mut preset.performance;
                match field {
                    0 => p.arp_division = value,
                    1 => p.arp_gate = value,
                    2 => p.octaves = value.round() as u8,
                    3 => p.sequence_division = value,
                    4 => p.sequence_gate = value,
                    _ => p.length = value.round() as u8,
                }
            }
        }
    }
}

fn oscillator_get(o: crate::resynth::Oscillator, field: usize) -> f32 {
    match field {
        0 => f32::from(o.enabled),
        1 => o.waveform.code() as f32,
        2 => o.mode.code() as f32,
        3 => o.control1,
        4 => o.control2,
        5 => o.transpose,
        6 => o.fine,
        7 => o.level,
        8 => o.pan,
        9 => o.cross_source as f32,
        10 => o.model as u8 as f32,
        11 => o.fold_shape as u8 as f32,
        12 => o.interaction as u8 as f32,
        13 => o.interaction_source as f32,
        _ => o.cross_input as u8 as f32,
    }
}
fn oscillator_set(o: &mut crate::resynth::Oscillator, field: usize, value: f32) {
    use crate::resynth::*;
    match field {
        0 => o.enabled = value >= 0.5,
        1 => o.waveform = Waveform::ALL[value.round() as usize],
        2 => o.mode = Mode::ALL[value.round() as usize],
        3 => o.control1 = value,
        4 => o.control2 = value,
        5 => o.transpose = value,
        6 => o.fine = value,
        7 => o.level = value,
        8 => o.pan = value,
        9 => o.cross_source = value.round() as u8,
        10 => {
            o.model = if value < 0.5 {
                Model::Radias
            } else {
                Model::Wavefolder
            }
        }
        11 => o.fold_shape = FoldShape::ALL[value.round() as usize],
        12 => o.interaction = Interaction::ALL[value.round() as usize],
        13 => o.interaction_source = value.round() as u8,
        _ => o.cross_input = CrossInput::ALL[value.round() as usize],
    }
}

fn append_modulators(result: &mut Vec<Spec>, preset: &Preset, i: usize) {
    for (field, (id, name, min, max)) in [
        ("rate", "rate", 0.01, 40.0),
        ("depth", "depth", -1.0, 1.0),
        ("division", "division", 0.0625, 32.0),
    ]
    .into_iter()
    .enumerate()
    {
        let mut spec = Spec {
            id: format!("lfo{}_{}", i + 1, id),
            name: format!("LFO {} {name}", i + 1),
            min,
            max,
            default: 0.0,
            target: Target::Lfo(i, field),
        };
        spec.default = spec.get(preset);
        result.push(spec);
    }
    for (field, id) in [
        "attack",
        "decay",
        "sustain",
        "release",
        "depth",
        "extra_depth",
    ]
    .into_iter()
    .enumerate()
    {
        let mut spec = Spec {
            id: format!("env{}_{}", i + 1, id),
            name: format!("ENV {} {}", i + 1, id.replace('_', " ")),
            min: if field >= 4 { -1.0 } else { 0.0 },
            max: 1.0,
            default: 0.0,
            target: Target::Env(i, field),
        };
        spec.default = spec.get(preset);
        result.push(spec);
    }
}

pub fn specs() -> &'static [Spec] {
    static SPECS: OnceLock<Vec<Spec>> = OnceLock::new();
    SPECS.get_or_init(|| {
        let preset = Preset::default();
        let mut result = Vec::new();
        macro_rules! core {
            ($field:ident, $name:literal, $min:expr, $max:expr) => {
                result.push(Spec {
                    id: stringify!($field).into(),
                    name: $name.into(),
                    min: $min,
                    max: $max,
                    default: preset.params.$field,
                    target: Target::Core {
                        get: |p| p.params.$field,
                        set: |p, v| p.params.$field = v,
                        frame_get: |m| m.params.$field,
                        frame_set: |m, v| m.params.$field = v,
                    },
                });
            };
        }
        core!(position, "Position", 0.0, 1.0);
        core!(grain_size, "Grain size", 0.0, 1.0);
        core!(grain_shape, "Grain shape", 0.0, 1.0);
        core!(grains, "Grain density", 0.0, 1.0);
        core!(pitch, "Pitch", 0.0, 1.0);
        core!(detune, "Detune", 0.0, 1.0);
        core!(cloud, "Cloud", 0.0, 1.0);
        core!(stereo_spread, "Stereo spread", 0.0, 1.0);
        core!(mix, "Dry / wet", 0.0, 1.0);
        core!(filter, "Filter 1 cutoff", 0.0, 1.0);
        core!(filter_resonance, "Filter 1 resonance", 0.0, 1.0);
        core!(filter_env_depth, "Filter 1 envelope", -1.0, 1.0);
        core!(filter_keytrack, "Filter 1 key tracking", 0.0, 1.0);
        core!(filter2, "Filter 2 cutoff", 0.0, 1.0);
        core!(filter2_resonance, "Filter 2 resonance", 0.0, 1.0);
        core!(filter2_env_depth, "Filter 2 envelope", -1.0, 1.0);
        core!(filter2_keytrack, "Filter 2 key tracking", 0.0, 1.0);
        core!(master_volume, "Volume", 0.0, 1.0);
        core!(in_gain, "Input gain", -20.0, 20.0);
        core!(out_gain, "Output gain", -12.0, 12.0);
        core!(sample_tune_semis, "Sample tune semitones", -48.0, 48.0);
        core!(sample_tune_cents, "Sample tune cents", -100.0, 100.0);
        core!(pitch_range, "Pitch range", 12.0, 48.0);
        core!(midi_pitchbend_range, "Pitch bend range", 0.0, 48.0);
        core!(min_grains, "Minimum grains", 0.0, 1.0);
        core!(bpm, "Tempo", 20.0, 300.0);
        for i in 0..3 {
            append_modulators(&mut result, &preset, i);
        }
        for (field, (id, name, min, max)) in [
            ("arp_rate", "Arp rate", 0.0625, 8.0),
            ("arp_gate", "Arp gate", 0.05, 0.95),
            ("arp_octaves", "Arp octaves", 1.0, 4.0),
            ("sequence_rate", "Sequence rate", 0.0625, 16.0),
            ("sequence_gate", "Sequence gate", 0.05, 0.95),
            ("sequence_length", "Sequence length", 1.0, 16.0),
        ]
        .into_iter()
        .enumerate()
        {
            let mut spec = Spec {
                id: id.into(),
                name: name.into(),
                min,
                max,
                default: 0.0,
                target: Target::Performance(field),
            };
            spec.default = spec.get(&preset);
            result.push(spec);
        }
        for i in 0..MAX_COLOR_SLOTS {
            for &field in ColorParameter::ALL {
                // Existing AU parameter order stays stable; append new controls below.
                if field == ColorParameter::WaveshaperDepth || field.is_imported() {
                    continue;
                }
                let default = *ColorSlot::default().parameter_mut(field);
                result.push(Spec {
                    id: format!("fx{}_{}", i + 1, field.key()),
                    name: format!("FX {} {}", i + 1, field.key().replace('_', " ")),
                    min: 0.0,
                    max: 1.0,
                    default: preset
                        .color_chain
                        .get(i)
                        .map_or(default, |s| *s.to_owned().parameter_mut(field)),
                    target: Target::Fx(false, i, field),
                });
            }
        }
        for pre in [false, true] {
            for i in 0..MAX_COLOR_SLOTS {
                for &field in ColorParameter::ALL {
                    if field.is_imported() || (!pre && field != ColorParameter::WaveshaperDepth) {
                        continue;
                    }
                    let mut spec = Spec {
                        id: format!(
                            "{}fx{}_{}",
                            if pre { "pre_" } else { "" },
                            i + 1,
                            field.key()
                        ),
                        name: format!(
                            "{}FX {} {}",
                            if pre { "Pre " } else { "" },
                            i + 1,
                            field.key().replace('_', " ")
                        ),
                        min: 0.0,
                        max: 1.0,
                        default: 0.0,
                        target: Target::Fx(pre, i, field),
                    };
                    spec.default = spec.get(&preset);
                    result.push(spec);
                }
            }
        }
        // Append controls: AU parameter indices and existing host IDs must not move.
        result.push(Spec {
            id: "sample_root_note".into(),
            name: "Sample root note".into(),
            min: 0.0,
            max: 127.0,
            default: 60.0,
            target: Target::Core {
                get: |p| p.params.sample_root_note,
                set: |p, v| p.params.sample_root_note = v.round(),
                frame_get: |m| m.params.sample_root_note,
                frame_set: |m, v| m.params.sample_root_note = v.round(),
            },
        });
        result.push(Spec {
            id: "loop_one_bar".into(),
            name: "Loop one bar".into(),
            min: 0.0,
            max: 1.0,
            default: 0.0,
            target: Target::Core {
                get: |p| f32::from(p.params.loop_one_bar),
                set: |p, v| p.params.loop_one_bar = v >= 0.5,
                frame_get: |m| f32::from(m.params.loop_one_bar),
                frame_set: |m, v| m.params.loop_one_bar = v >= 0.5,
            },
        });
        for &field in ColorParameter::ALL {
            if field.is_imported() {
                continue;
            }
            result.push(Spec {
                id: format!("source_fx1_{}", field.key()),
                name: format!("Source FX {}", field.key().replace('_', " ")),
                min: 0.0,
                max: 1.0,
                default: *ColorSlot::default().parameter_mut(field),
                target: Target::SourceFx(field),
            });
        }
        macro_rules! resynth {
            ($field:ident, $name:literal, $min:expr, $max:expr) => {
                result.push(Spec {
                    id: concat!("resynth_", stringify!($field)).into(),
                    name: concat!("Resynth ", $name).into(),
                    min: $min,
                    max: $max,
                    default: preset.resynth.$field,
                    target: Target::Core {
                        get: |p| p.resynth.$field,
                        set: |p, v| p.resynth.$field = v,
                        frame_get: |m| m.resynth.$field,
                        frame_set: |m, v| m.resynth.$field = v,
                    },
                });
            };
        }
        resynth!(mix, "mix", 0.0, 1.0);
        resynth!(sensitivity, "sensitivity", 0.0, 1.0);
        resynth!(attack, "attack", 0.0, 1.0);
        resynth!(release, "release", 0.0, 1.0);
        resynth!(threshold, "threshold", 0.0, 1.0);
        result.push(Spec {
            id: "resynth_enabled".into(),
            name: "Resynth enabled".into(),
            min: 0.0,
            max: 1.0,
            default: 0.0,
            target: Target::Core {
                get: |p| f32::from(p.resynth.enabled),
                set: |p, v| p.resynth.enabled = v >= 0.5,
                frame_get: |m| f32::from(m.resynth.enabled),
                frame_set: |m, v| m.resynth.enabled = v >= 0.5,
            },
        });
        result.push(Spec {
            id: "resynth_input".into(),
            name: "Resynth input".into(),
            min: 0.0,
            max: 1.0,
            default: 0.0,
            target: Target::Core {
                frame_get: |m| {
                    if m.resynth.input == crate::resynth::Input::Source {
                        0.0
                    } else {
                        1.0
                    }
                },
                frame_set: |m, v| {
                    m.resynth.input = if v < 0.5 {
                        crate::resynth::Input::Source
                    } else {
                        crate::resynth::Input::Grains
                    }
                },
                get: |p| {
                    if p.resynth.input == crate::resynth::Input::Source {
                        0.0
                    } else {
                        1.0
                    }
                },
                set: |p, v| {
                    p.resynth.input = if v < 0.5 {
                        crate::resynth::Input::Source
                    } else {
                        crate::resynth::Input::Grains
                    }
                },
            },
        });
        result.push(Spec {
            id: "resynth_count".into(),
            name: "Resynth oscillator count".into(),
            min: 1.0,
            max: 4.0,
            default: 1.0,
            target: Target::Core {
                get: |p| p.resynth.count as f32,
                set: |p, v| p.resynth.count = v.round() as u8,
                frame_get: |m| m.resynth.count as f32,
                frame_set: |m, v| m.resynth.count = v.round() as u8,
            },
        });
        for i in 0..crate::resynth::MAX_OSCILLATORS {
            for (field, (key, name, min, max)) in [
                ("enabled", "enabled", 0.0, 1.0),
                ("waveform", "waveform", 0.0, 7.0),
                ("mode", "mode", 0.0, 3.0),
                ("control1", "control 1", 0.0, 1.0),
                ("control2", "control 2", 0.0, 1.0),
                ("transpose", "transpose", -48.0, 48.0),
                ("fine", "fine tune", -100.0, 100.0),
                ("level", "level", 0.0, 1.0),
                ("pan", "pan", 0.0, 1.0),
                ("cross_source", "cross source", 0.0, 4.0),
            ]
            .into_iter()
            .enumerate()
            {
                let mut spec = Spec {
                    id: format!("resynth_osc{}_{}", i + 1, key),
                    name: format!("Resynth OSC {} {name}", i + 1),
                    min,
                    max,
                    default: 0.0,
                    target: Target::ResynthOsc(i, field),
                };
                spec.default = spec.get(&preset);
                result.push(spec);
            }
        }
        // Never insert fields into the released per-oscillator stride above.
        for i in 0..crate::resynth::MAX_OSCILLATORS {
            for (offset, (key, name, max)) in [
                ("model", "model", 1.0),
                ("fold_shape", "fold shape", 4.0),
                ("interaction", "Ring/Sync", 3.0),
                ("interaction_source", "Ring/Sync source", 5.0),
                ("cross_input", "Cross input", 2.0),
            ]
            .into_iter()
            .enumerate()
            {
                let mut spec = Spec {
                    id: format!("resynth_osc{}_{}", i + 1, key),
                    name: format!("Resynth OSC {} {name}", i + 1),
                    min: 0.0,
                    max,
                    default: 0.0,
                    target: Target::ResynthOsc(i, 10 + offset),
                };
                spec.default = spec.get(&preset);
                result.push(spec);
            }
        }
        // New filter controls append after every released host index.
        core!(filter_drive, "Filter 1 drive", 0.0, 1.0);
        core!(filter_morph, "Filter 1 morph", 0.0, 1.0);
        core!(filter2_drive, "Filter 2 drive", 0.0, 1.0);
        core!(filter2_morph, "Filter 2 morph", 0.0, 1.0);
        macro_rules! choice {
            ($field:ident, $name:literal, $kind:ident, $max:expr) => {
                result.push(Spec {
                    id: stringify!($field).into(),
                    name: $name.into(),
                    min: 0.0,
                    max: $max,
                    default: preset.params.$field as u8 as f32,
                    target: Target::Core {
                        get: |p| p.params.$field as u8 as f32,
                        set: |p, v| p.params.$field = crate::preset::$kind::ALL[v.round() as usize],
                        frame_get: |m| m.params.$field as u8 as f32,
                        frame_set: |m, v| {
                            m.params.$field = crate::preset::$kind::ALL[v.round() as usize]
                        },
                    },
                });
            };
        }
        choice!(filter_model, "Filter 1 model", FilterModel, 19.0);
        choice!(filter2_model, "Filter 2 model", FilterModel, 19.0);
        choice!(vcf_type, "Filter 1 response", FilterType, 3.0);
        choice!(vcf2_type, "Filter 2 response", FilterType, 3.0);
        // Imported effects append after all 0.8 IDs, including Source FX.
        for rack in 0..3 {
            let count = if rack == 2 { 1 } else { MAX_COLOR_SLOTS };
            for i in 0..count {
                for &field in &ColorParameter::ALL[ColorParameter::LEGACY_COUNT..] {
                    let prefix = ["", "pre_", "source_"][rack];
                    let mut spec = Spec {
                        id: format!("{prefix}fx{}_{}", i + 1, field.key()),
                        name: format!(
                            "{}FX {} {}",
                            ["", "Pre ", "Source "][rack],
                            i + 1,
                            field.key().replace('_', " ")
                        ),
                        min: 0.0,
                        max: 1.0,
                        default: 0.0,
                        target: if rack == 2 {
                            Target::SourceFx(field)
                        } else {
                            Target::Fx(rack == 1, i, field)
                        },
                    };
                    spec.default = spec.get(&preset);
                    result.push(spec);
                }
            }
        }
        for i in 3..crate::preset::MODULATORS {
            append_modulators(&mut result, &preset, i);
        }
        let drums = sad_pet_dxm088::params::kit(0);
        for (i, row) in drums.iter().enumerate() {
            for (field, default) in row.iter().enumerate() {
                result.push(Spec {
                    id: format!("drum{}_{}", i + 1, field),
                    name: format!(
                        "{} · {}",
                        crate::drums::NAMES[i],
                        crate::drums::PARAM_NAMES[field]
                    ),
                    min: sad_pet_dxm088::params::MIN[field],
                    max: sad_pet_dxm088::params::MAX[field],
                    default: *default,
                    target: Target::Drum(i, field),
                });
            }
        }
        result.push(Spec {
            id: "drum_granular".into(),
            name: "Drum granular processing".into(),
            min: 0.,
            max: 1.,
            default: 0.,
            target: Target::DrumGrain,
        });
        result.push(Spec {
            id: "drum_buffer_seconds".into(),
            name: "Drum granular buffer seconds".into(),
            min: 0.125,
            max: 32.,
            default: 8.,
            target: Target::DrumBuffer,
        });
        result
    })
}

pub fn mirror(preset: &mut Preset) {
    let p = &mut preset.params;
    p.adsr_attack = preset.envs[0].attack;
    p.adsr_decay = preset.envs[0].decay;
    p.adsr_sustain = preset.envs[0].sustain;
    p.adsr_release = preset.envs[0].release;
    p.saturation = preset.color_chain.first().map_or(0.0, |s| s.mix);
    crate::color::mirror_reverb(p, &preset.color_chain);
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn resynth_is_backward_compatible_and_appended_controls_roundtrip() {
        let mut old = serde_json::to_value(Preset::default()).unwrap();
        old.as_object_mut().unwrap().remove("resynth");
        let mut preset: Preset = serde_json::from_value(old).unwrap();
        assert_eq!(preset.resynth, crate::resynth::Settings::default());
        assert_eq!(specs()[1475].id, "sample_root_note");
        assert_eq!(specs()[1476].id, "loop_one_bar");
        assert_eq!(
            specs()[1477 + ColorParameter::LEGACY_COUNT].id,
            "resynth_mix"
        );
        let changes=parse(&json!({"resynth_enabled":1,"resynth_count":4,"resynth_input":1,"resynth_attack":0.31,"resynth_osc4_waveform":7,"resynth_osc4_transpose":-12,"resynth_osc4_control1":0.7,"resynth_osc3_mode":3,"resynth_osc3_cross_source":4})).unwrap();
        apply(&mut preset, &changes);
        preset.sanitize();
        assert_eq!(preset.resynth.count, 4);
        assert_eq!(
            preset.resynth.oscillators[3].waveform,
            crate::resynth::Waveform::Pcm
        );
        assert_eq!(
            preset.resynth.oscillators[2].mode,
            crate::resynth::Mode::Vpm
        );
        let restored: Preset =
            serde_json::from_slice(&serde_json::to_vec(&preset).unwrap()).unwrap();
        assert_eq!(restored.resynth, preset.resynth);
        let changes=parse(&json!({"resynth_count":99,"resynth_osc4_cross_source":999,"resynth_osc4_transpose":-900})).unwrap();
        apply(&mut preset, &changes);
        assert_eq!(preset.resynth.count, 4);
        assert_eq!(preset.resynth.oscillators[3].cross_source, 4);
        assert_eq!(preset.resynth.oscillators[3].transpose, -48.0);
    }
    #[test]
    fn osc4_interactions_append_after_all_released_resynth_ids() {
        let start = 1477 + ColorParameter::LEGACY_COUNT + 8;
        for i in 0..4 {
            assert_eq!(
                specs()[start + i * 10].id,
                format!("resynth_osc{}_enabled", i + 1)
            );
            assert_eq!(
                specs()[start + i * 10 + 9].id,
                format!("resynth_osc{}_cross_source", i + 1)
            );
            assert_eq!(specs()[start + i * 10 + 9].max, 4.0);
        }
        let mut p = Preset::default();
        let values = json!({"resynth_osc4_model":1,"resynth_osc4_fold_shape":4,"resynth_osc4_interaction":3,"resynth_osc4_interaction_source":5,"resynth_osc4_cross_input":2});
        apply(&mut p, &parse(&values).unwrap());
        p.sanitize();
        let restored: Preset = serde_json::from_slice(&serde_json::to_vec(&p).unwrap()).unwrap();
        assert_eq!(p.resynth, restored.resynth);
        assert_eq!(
            p.resynth.oscillators[3].model,
            crate::resynth::Model::Wavefolder
        );
        assert_eq!(
            p.resynth.oscillators[3].fold_shape,
            crate::resynth::FoldShape::Rectify
        );
        assert_eq!(
            p.resynth.oscillators[3].interaction,
            crate::resynth::Interaction::RingSync
        );
        let mut old = serde_json::to_value(&p).unwrap();
        for o in old["resynth"]["oscillators"].as_array_mut().unwrap() {
            for key in [
                "model",
                "fold_shape",
                "interaction",
                "interaction_source",
                "cross_input",
            ] {
                o.as_object_mut().unwrap().remove(key);
            }
        }
        let old: Preset = serde_json::from_value(old).unwrap();
        assert_eq!(
            old.resynth.oscillators[3].model,
            crate::resynth::Model::Radias
        );
        assert_eq!(
            old.resynth.oscillators[3].interaction,
            crate::resynth::Interaction::Off
        );
    }

    #[test]
    fn old_presets_keep_original_playback_and_new_loop_settings_roundtrip() {
        let mut old = serde_json::to_value(Preset::default()).unwrap();
        old["params"]
            .as_object_mut()
            .unwrap()
            .remove("loop_one_bar");
        old["params"]
            .as_object_mut()
            .unwrap()
            .remove("sample_root_note");
        let mut preset: Preset = serde_json::from_value(old).unwrap();
        assert!(!preset.params.loop_one_bar);
        assert_eq!(preset.params.sample_root_note, 60.0);
        apply(
            &mut preset,
            &parse(&json!({"loop_one_bar":1,"sample_root_note":48.4})).unwrap(),
        );
        let restored: Preset =
            serde_json::from_slice(&serde_json::to_vec(&preset).unwrap()).unwrap();
        assert!(restored.params.loop_one_bar);
        assert_eq!(restored.params.sample_root_note, 48.0);
    }

    #[test]
    fn numeric_batches_clamp_values_preserve_topology_and_mirror_legacy_fields() {
        let mut preset = Preset {
            color_chain: vec![ColorSlot::default()],
            pre_color_chain: vec![ColorSlot {
                id: 2,
                ..ColorSlot::default()
            }],
            matrix: vec![crate::matrix::Route::default()],
            ..Preset::default()
        };
        let topology = (
            preset.color_chain.as_ptr(),
            preset.pre_color_chain.as_ptr(),
            preset.matrix.as_ptr(),
        );
        let changes = parse(&json!({"position":2.0,"env1_attack":-1.0,"lfo3_depth":-2.0,
            "fx1_mix":0.4,"pre_fx1_waveshaper_depth":0.75,"matrix1_depth":-3.0,
            "sequence_length":4.7}))
        .unwrap();
        assert!(apply(&mut preset, &changes));
        assert_eq!(preset.params.position, 1.0);
        assert_eq!(preset.params.adsr_attack, 0.0);
        assert_eq!(preset.lfos[2].depth, -1.0);
        assert_eq!(preset.params.saturation, 0.4);
        assert_eq!(preset.pre_color_chain[0].settings.waveshaper_depth, 0.75);
        assert_eq!(preset.matrix[0].depth, -1.0);
        assert_eq!(preset.performance.length, 5);
        assert_eq!(
            (
                preset.color_chain.as_ptr(),
                preset.pre_color_chain.as_ptr(),
                preset.matrix.as_ptr()
            ),
            topology
        );
        assert!(
            !apply(&mut preset, &changes),
            "Unchanged values requested another refresh"
        );
    }

    #[test]
    fn numeric_messages_reject_unknown_non_numeric_and_out_of_range_addresses() {
        for bad in [
            json!([]),
            json!({"position":null}),
            json!({"position":"0.5"}),
            json!({"position":1e100}),
            json!({"matrix0_depth":0.2}),
            json!({"matrix9_depth":0.2}),
            json!({"position":0.4,"unknown":0.5}),
        ] {
            assert!(parse(&bad).is_err(), "Accepted invalid message: {bad}");
        }
    }
}
