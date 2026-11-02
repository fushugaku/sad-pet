//! Control surfaces retain references to the original editors, not copies of
//! their sound. Their sequencer lanes are compiled into each destination engine.
use crate::{
    preset::Preset,
    sequence::{Program, Target},
    timbres::{Kind, Meta},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub items: Vec<Item>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Item {
    pub id: String,
    pub timbre: u32,
    pub label: String,
    pub selector: String,
    pub parameter: Option<String>,
    pub module: bool,
    pub view: serde_json::Value,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Settings {
    pub fn sanitize(&mut self) {
        self.items
            .retain(|i| !i.id.is_empty() && !i.selector.is_empty());
        for item in &mut self.items {
            for value in [&mut item.x, &mut item.y, &mut item.width, &mut item.height] {
                *value = if value.is_finite() {
                    value.clamp(0., 100_000.)
                } else {
                    0.
                };
            }
        }
    }
}

/// A separate clock for each Control, shared by all its lanes at a destination.
/// Prepared on the control thread; its audio-side replacement retires off-thread.
#[derive(Debug)]
pub struct Input {
    pub source: u32,
    pub bpm: f32,
    pub muted: bool,
    pub program: Program,
    pub(crate) sums: Vec<(usize, f32)>,
}

pub fn destination(id: &str) -> Option<(u32, &str)> {
    let (timbre, parameter) = id.strip_prefix("control:")?.split_once(':')?;
    Some((timbre.parse().ok()?, parameter))
}

pub fn inputs<'a>(target: u32, parts: impl Iterator<Item = (&'a Meta, &'a Preset)>) -> Vec<Input> {
    parts
        .filter(|(m, p)| m.kind == Kind::Control && p.control.is_some())
        .filter_map(|(meta, preset)| {
            let mut program = preset.sequencer.clone();
            let mut routed = false;
            let mut map = |id: &str| -> Option<String> {
                let (timbre, parameter) = destination(id)?;
                if timbre != target {
                    return None;
                }
                if !crate::parameters::specs()
                    .iter()
                    .any(|s| s.id == parameter && s.can_sequence())
                {
                    return None;
                }
                routed = true;
                Some(parameter.into())
            };
            for track in &mut program.tracks {
                let parameter = match &track.target {
                    Target::Parameter { parameter } => map(parameter),
                    _ => None,
                };
                track.target = Target::Parameter {
                    parameter: parameter.unwrap_or_default(),
                };
                for destination in &mut track.destinations {
                    *destination = destination.as_deref().and_then(&mut map);
                }
                // A Control adds offsets; it never replaces the parent's base.
                track.absolute = false;
            }
            if !routed {
                return None;
            }
            program.sanitize();
            Some(Input {
                source: meta.id,
                bpm: preset.params.bpm,
                muted: meta.mix.muted,
                program,
                sums: Vec::with_capacity(crate::parameters::specs().len()),
            })
        })
        .collect()
}
