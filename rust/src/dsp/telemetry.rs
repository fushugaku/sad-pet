//! Read-only control feedback. Reuse the DSP's destination arithmetic, without
//! running processors or changing audio state. Called at UI publication time.
use super::{Engine, Modulated, filter};
use crate::{matrix::Source, preset::Destination};
use serde::Serialize;
use std::collections::BTreeMap;

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
    pub fn gain_reduction(&self) -> BTreeMap<u32, f32> {
        let mut meters = self.color_rack.gain_reduction();
        if !self.playing || self.bypass {
            meters.values_mut().for_each(|value| *value = 0.0);
        }
        meters
    }

    fn control_limits(&self) -> (Modulated, Modulated) {
        let mut low = Modulated::from_preset(&self.preset);
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
        for (destination, source, depth) in terms {
            let delta = product_range(source_range(source), [depth; 2]);
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
        for (i, lfo) in self.preset.lfos.iter().enumerate() {
            let range = product_range([-0.5, 0.5], [low_lfos[i].depth, high_lfos[i].depth]);
            apply(&mut low, &mut high, lfo.destination, range);
        }
        let low_envs = low.envs;
        let high_envs = high.envs;
        for (i, env) in self.preset.envs.iter().enumerate() {
            for (destination, depths) in [
                (env.destination, [low_envs[i].depth, high_envs[i].depth]),
                (
                    env.extra_destination,
                    [low_envs[i].extra_depth, high_envs[i].extra_depth],
                ),
            ] {
                apply(
                    &mut low,
                    &mut high,
                    destination,
                    product_range([0.0, 1.0], depths),
                );
            }
        }
        (low, high)
    }
    pub fn knob_states(&self) -> BTreeMap<String, KnobState> {
        let (low, high) = self.control_limits();
        let p = self.preset.params;
        let m = self.modulated.params;
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
            filter2_resonance,
            filter2_env_depth,
            filter2_keytrack,
            mix,
            master_volume
        );

        // Cutoff includes each voice's dedicated envelope and keyboard tracking.
        // A single marker follows the latest sounding note, as do note sources.
        let voice = self
            .voices
            .iter()
            .find(|v| v.active() && v.note == self.source_note && v.channel == self.source_channel)
            .or_else(|| self.voices.iter().find(|v| v.active()));
        let (note, envs) = voice.map_or((60.0, [0.0; 3]), |v| {
            (
                v.note as f32 + self.bend[v.channel as usize] * m.midi_pitchbend_range,
                v.env_values,
            )
        });
        let current = filter::voice_cutoffs(&m, note, envs);
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
        for (i, slot) in self.preset.color_chain.iter().enumerate() {
            for &parameter in slot.control_parameters() {
                let mut base = *slot;
                let mut current = self.modulated.color_slots[i];
                let mut min = low.color_slots[i];
                let mut max = high.color_slots[i];
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
        result
    }
}
