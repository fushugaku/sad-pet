//! Neuropolis programs: dynamic track count, stable track IDs and scalar edits.
//! All clocks, play orders, gates and pulse decisions use the pinned Rust core.
use metropolix_core::sequencer::{TRACK_RANGES, field};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const STAGES: usize = 16;
pub const VIEW_TRACKS: usize = 64;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Target {
    #[default]
    MainPitch,
    Oscillator {
        index: u8,
    },
    Parameter {
        parameter: String,
    },
    Notes,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Stage {
    pub pitch: u8,
    pub pulses: u8,
    pub gate: u8,
    pub ratchet: u8,
    pub probability: u8,
    pub slide: bool,
    pub skip: bool,
    pub accumulate: i16,
    pub pitch_override: i16,
    pub gate_override: i16,
    pub probability_target: u8,
    pub accumulate_trigger: u8,
    pub cv: i32,
    pub cv_ramp: u8,
    pub cv_gate: bool,
    pub velocity: u8,
}
impl Default for Stage {
    fn default() -> Self {
        Self {
            pitch: 60,
            pulses: 1,
            gate: 2,
            ratchet: 1,
            probability: 100,
            slide: false,
            skip: false,
            accumulate: 0,
            pitch_override: -1,
            gate_override: -1,
            probability_target: 0,
            accumulate_trigger: 0,
            cv: 0,
            cv_ramp: 0,
            cv_gate: false,
            velocity: 100,
        }
    }
}
impl Stage {
    pub fn set(&mut self, field: u8, value: f32) {
        let n = value.round() as i32;
        match field {
            0 => self.pitch = n.clamp(0, 127) as u8,
            1 => self.pulses = n.clamp(1, 8) as u8,
            2 => self.gate = n.clamp(0, 3) as u8,
            3 => self.ratchet = n.clamp(1, 8) as u8,
            4 => self.probability = n.clamp(0, 100) as u8,
            5 => self.slide = value >= 0.5,
            6 => self.skip = value >= 0.5,
            7 => self.accumulate = n.clamp(-28, 28) as i16,
            8 => self.pitch_override = n.clamp(-1, 127) as i16,
            9 => self.gate_override = n.clamp(-1, 100) as i16,
            10 => self.probability_target = n.clamp(0, 1) as u8,
            11 => self.accumulate_trigger = n.clamp(0, 2) as u8,
            12 => self.cv = n.clamp(-5000, 5000),
            13 => self.cv_ramp = n.clamp(0, 2) as u8,
            14 => self.cv_gate = value >= 0.5,
            15 => self.velocity = n.clamp(0, 127) as u8,
            _ => {}
        }
    }
    pub fn get(self, field: u8) -> f32 {
        match field {
            0 => self.pitch as f32,
            1 => self.pulses as f32,
            2 => self.gate as f32,
            3 => self.ratchet as f32,
            4 => self.probability as f32,
            5 => u8::from(self.slide) as f32,
            6 => u8::from(self.skip) as f32,
            7 => self.accumulate as f32,
            8 => self.pitch_override as f32,
            9 => self.gate_override as f32,
            10 => self.probability_target as f32,
            11 => self.accumulate_trigger as f32,
            12 => self.cv as f32,
            13 => self.cv_ramp as f32,
            14 => u8::from(self.cv_gate) as f32,
            _ => self.velocity as f32,
        }
    }
}

pub fn default_config() -> [i32; field::COUNT] {
    // Neuropolis default_config at 872a28c; SAD PET starts at full track velocity.
    [
        0, 16, 0, 1, 0, 0, 0, 55, 0, 0, 12, 50, 30, 0, 0, 0, 1, 100, 12, 1, 1, 0, 4, 0, 0, 0, 0, 0,
        1, 1, 100, 0, 0, 0, -1, -1, -1, 0, 36, 0, 0, 127, 16,
    ]
}
mod config_words {
    use super::*;
    pub fn serialize<S: serde::Serializer>(
        config: &[i32; field::COUNT],
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        config.as_slice().serialize(serializer)
    }
    pub fn deserialize<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<[i32; field::COUNT], D::Error> {
        let words = Vec::<i32>::deserialize(deserializer)?;
        let mut config = default_config();
        for (target, value) in config.iter_mut().zip(words) {
            *target = value;
        }
        Ok(config)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Track {
    pub id: u32,
    pub name: String,
    pub target: Target,
    pub amount: f32,
    pub absolute: bool,
    pub gate_output: bool,
    pub root_note: u8,
    /// Main pitch also transposes the selected Resynth layers. Oscillator
    /// lanes add their own -24..24 semitone offset to that shared pitch.
    pub pitch_oscillators: [bool; 4],
    #[serde(with = "config_words")]
    pub config: [i32; field::COUNT],
    pub steps: [Stage; STAGES],
}
impl Default for Track {
    fn default() -> Self {
        Self {
            id: 1,
            name: "Track 1".into(),
            target: Target::MainPitch,
            amount: 1.0,
            absolute: false,
            gate_output: false,
            root_note: 60,
            pitch_oscillators: [false; 4],
            config: default_config(),
            steps: [Stage::default(); STAGES],
        }
    }
}
impl Track {
    pub fn edit(&mut self, edit: Edit) {
        if let Some(stage) = edit.stage {
            if let Some(s) = self.steps.get_mut(stage as usize) {
                s.set(edit.field, edit.value);
                if matches!(self.target, Target::Oscillator { .. }) {
                    s.pitch = s.pitch.clamp(36, 84);
                    if s.pitch_override >= 0 {
                        s.pitch_override = s.pitch_override.clamp(36, 84);
                    }
                }
            }
        } else if let Some((lo, hi)) = TRACK_RANGES.get(edit.field as usize) {
            self.config[edit.field as usize] = (edit.value.round() as i32).clamp(*lo, *hi);
        } else {
            match edit.field {
                43 => self.amount = edit.value.clamp(-1.0, 1.0),
                44 => self.absolute = edit.value >= 0.5,
                45 => self.gate_output = edit.value >= 0.5,
                46 if !matches!(self.target, Target::Oscillator { .. }) => {
                    self.root_note = edit.value.round().clamp(0.0, 127.0) as u8;
                }
                49..=52 => self.pitch_oscillators[(edit.field - 49) as usize] = edit.value >= 0.5,
                _ => {}
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Program {
    pub enabled: bool,
    pub tracks: Vec<Track>,
    #[serde(skip)]
    pub(crate) runtime: Option<Box<crate::dsp::sequence::Runtime>>,
}
impl Default for Program {
    fn default() -> Self {
        Self {
            enabled: false,
            tracks: vec![Track::default()],
            runtime: None,
        }
    }
}
impl Clone for Program {
    fn clone(&self) -> Self {
        Self {
            enabled: self.enabled,
            tracks: self.tracks.clone(),
            runtime: None,
        }
    }
}
impl Program {
    pub fn migrate(old: &mut crate::performance::Settings) -> Self {
        use crate::performance::Mode;
        if !matches!(old.mode, Mode::Sequencer | Mode::SequencerArp) {
            return Self::default();
        }
        let voices = old
            .steps
            .iter()
            .map(|s| s.notes.as_slice().len())
            .max()
            .unwrap_or(1)
            .max(1);
        let division = old.sequence_division * 4.0;
        let code = if division <= 0.25 {
            66
        } else if division <= 0.5 {
            65
        } else {
            division.round().clamp(1.0, 64.0) as i32
        };
        let tracks = (0..voices)
            .map(|voice| {
                let mut t = Track {
                    id: voice as u32 + 1,
                    name: format!("Notes {}", voice + 1),
                    target: Target::Notes,
                    ..Track::default()
                };
                t.config[field::LENGTH] = old.length as i32;
                t.config[field::DIVISION] = code;
                t.config[field::GATE] = (old.sequence_gate * 100.0).round() as i32;
                for (to, from) in t.steps.iter_mut().zip(old.steps) {
                    if let Some(note) = from.notes.as_slice().get(voice) {
                        to.pitch = *note;
                    } else {
                        to.gate = 0;
                    }
                    if !from.enabled {
                        to.gate = 0;
                    }
                    to.velocity = from.velocity;
                }
                t
            })
            .collect();
        old.mode = Mode::Keys;
        Self {
            enabled: true,
            tracks,
            runtime: None,
        }
    }
    /// Only the small numeric edit crosses into the audio owner.
    pub fn edit(&mut self, edits: &[Edit]) {
        for &edit in edits {
            if edit.track == 0 {
                if edit.field == 0 {
                    self.enabled = edit.value >= 0.5;
                }
            } else if let Some(index) = self
                .runtime
                .as_ref()
                .and_then(|r| r.index(edit.track))
                .or_else(|| self.tracks.iter().position(|t| t.id == edit.track))
            {
                self.tracks[index].edit(edit);
                if let Some(runtime) = &mut self.runtime {
                    runtime.edit(index, &self.tracks[index], edit);
                }
            }
        }
    }
    pub fn sanitize(&mut self) {
        let mut ids = BTreeSet::new();
        let mut next = self
            .tracks
            .iter()
            .map(|t| t.id)
            .max()
            .unwrap_or(0)
            .saturating_add(1)
            .max(1);
        for track in &mut self.tracks {
            if track.id == 0 || !ids.insert(track.id) {
                while ids.contains(&next) {
                    next = next.wrapping_add(1).max(1);
                }
                track.id = next;
                ids.insert(next);
                next = next.wrapping_add(1).max(1);
            }
            if !track.amount.is_finite() {
                track.amount = 1.0;
            }
            track.amount = track.amount.clamp(-1.0, 1.0);
            track.root_note = track.root_note.min(127);
            if let Target::Oscillator { index } = &mut track.target {
                *index = (*index).min(3);
                // Keep older root-relative patterns at the same offset while
                // giving every oscillator lane a consistent signed editor.
                let shift = 60 - i16::from(track.root_note);
                track.config[field::ROOT] =
                    (track.config[field::ROOT] + i32::from(shift)).rem_euclid(12);
                for step in &mut track.steps {
                    step.pitch = (i16::from(step.pitch) + shift).clamp(36, 84) as u8;
                    if step.pitch_override >= 0 {
                        step.pitch_override = (step.pitch_override + shift).clamp(36, 84);
                    }
                }
                track.root_note = 60;
            }
            for (value, (lo, hi)) in track.config.iter_mut().zip(TRACK_RANGES) {
                *value = (*value).clamp(lo, hi);
            }
            for step in &mut track.steps {
                for field in 0..16 {
                    step.set(field, step.get(field));
                }
            }
        }
        if self.runtime.is_none() {
            self.runtime = Some(Box::new(crate::dsp::sequence::Runtime::new(&self.tracks)));
        }
    }
}

/// Metadata is built on the control thread. Source FX is rendered ahead of
/// playback and is deliberately absent from realtime destinations.
pub fn catalog() -> serde_json::Value {
    serde_json::json!({
        "track":Track::default(),
        "orders":["Linear","Ping Pong","Pendulum","Odd / Even","Jump by 2","Jump by 3","Jump by 4","Jump by 5","Hopscotch","Arp 3 − 1","Arp 4 − 2","Arp 4 − 1","Converge","Diverge","Growing","Shrinking","Pedal Point","Brownian","Shuffle","Quasi Random","Random"],
        "scales":metropolix_core::scale::FACTORY_SCALES.iter().map(|s|s.name).collect::<Vec<_>>(),
        "ranges":TRACK_RANGES.as_slice(),
        "parameters":crate::parameters::specs().iter().filter(|s|s.can_sequence()).collect::<Vec<_>>(),
        "modulation":crate::parameters::specs().iter().enumerate().filter(|(_,s)|s.can_sequence()).map(|(index,s)|serde_json::json!({"index":index,"id":s.id,"name":s.name})).collect::<Vec<_>>(),
    })
}

/// Track zero edits transport: field 0 = enabled, field 1 = restart. All other
/// IDs address an existing track; stage fields follow Neuropolis's stable ABI.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Edit {
    pub track: u32,
    #[serde(default)]
    pub stage: Option<u8>,
    pub field: u8,
    pub value: f32,
}
pub fn parse_edits(value: &serde_json::Value) -> Result<Vec<Edit>, String> {
    let edits: Vec<Edit> = serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
    if edits.iter().any(|e| {
        !e.value.is_finite()
            || e.stage.is_some_and(|s| s >= 16)
            || e.field > if e.stage.is_some() { 15 } else { 52 }
            || (e.track == 0 && (e.stage.is_some() || e.field > 1))
    }) {
        return Err("Invalid sequencer control".into());
    }
    Ok(edits)
}

#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize)]
pub struct TrackView {
    pub id: u32,
    pub step: u8,
    pub gate: bool,
    pub pulse: bool,
    pub pitch: f32,
    pub cv: f32,
    pub value: f32,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct View {
    pub offset: usize,
    pub total: usize,
    pub count: usize,
    #[serde(with = "view_tracks")]
    pub tracks: [TrackView; VIEW_TRACKS],
}
impl Default for View {
    fn default() -> Self {
        Self {
            offset: 0,
            total: 0,
            count: 0,
            tracks: [TrackView::default(); VIEW_TRACKS],
        }
    }
}
mod view_tracks {
    use super::*;
    pub fn serialize<S: serde::Serializer>(
        v: &[TrackView; VIEW_TRACKS],
        s: S,
    ) -> Result<S::Ok, S::Error> {
        v.as_slice().serialize(s)
    }
    pub fn deserialize<'de, D: serde::Deserializer<'de>>(
        d: D,
    ) -> Result<[TrackView; VIEW_TRACKS], D::Error> {
        let v = Vec::<TrackView>::deserialize(d)?;
        let mut out = [TrackView::default(); VIEW_TRACKS];
        for (to, from) in out.iter_mut().zip(v) {
            *to = from;
        }
        Ok(out)
    }
}
