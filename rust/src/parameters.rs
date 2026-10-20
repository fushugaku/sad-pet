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
    },
    Lfo(usize, usize),
    Env(usize, usize),
    Fx(bool, usize, ColorParameter),
    SourceFx(ColorParameter),
    Performance(usize),
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
    pub fn get(&self, preset: &Preset) -> f32 {
        match self.target {
            Target::SourceFx(field) => *preset
                .source_color_chain
                .first()
                .copied()
                .unwrap_or_default()
                .parameter_mut(field),
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
                spec.default = spec.get(&preset);
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
                spec.default = spec.get(&preset);
                result.push(spec);
            }
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
                if field == ColorParameter::WaveshaperDepth {
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
                    if !pre && field != ColorParameter::WaveshaperDepth {
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
            },
        });
        for &field in ColorParameter::ALL {
            result.push(Spec {
                id: format!("source_fx1_{}", field.key()),
                name: format!("Source FX {}", field.key().replace('_', " ")),
                min: 0.0,
                max: 1.0,
                default: *ColorSlot::default().parameter_mut(field),
                target: Target::SourceFx(field),
            });
        }
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
