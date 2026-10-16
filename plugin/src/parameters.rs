//! Stable host parameter IDs. FX automation addresses positions in the rack.
use sad_pet::{
    color::{ColorParameter, ColorSlot, MAX_COLOR_SLOTS},
    preset::Preset,
};
use serde::Serialize;
use std::sync::OnceLock;

#[derive(Clone, Copy)]
enum Target {
    Core {
        get: fn(&Preset) -> f32,
        set: fn(&mut Preset, f32),
    },
    Lfo(usize, usize),
    Env(usize, usize),
    Fx(usize, ColorParameter),
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
            Target::Fx(i, field) => *preset
                .color_chain
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
            Target::Fx(i, field) => {
                if let Some(slot) = preset.color_chain.get_mut(i) {
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
                    target: Target::Fx(i, field),
                });
            }
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
    sad_pet::color::mirror_reverb(p, &preset.color_chain);
}
