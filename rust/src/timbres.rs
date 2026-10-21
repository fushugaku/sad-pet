//! Stable part identities shared by the browser and native hosts.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    #[default]
    Granular,
    Drums,
    Synth,
}
impl Kind {
    pub fn label(self) -> &'static str {
        match self {
            Kind::Granular => "Granular",
            Kind::Drums => "Drums",
            Kind::Synth => "Synth",
        }
    }
    pub fn of(preset: &crate::preset::Preset) -> Self {
        if preset.drums.is_some() {
            Kind::Drums
        } else if preset.synth.is_some() {
            Kind::Synth
        } else {
            Kind::Granular
        }
    }
    /// Drum and Synth parts start from their own engine; a Granular part
    /// keeps its host's preset.
    pub fn preset(self) -> Option<crate::preset::Preset> {
        let mut preset = crate::preset::Preset {
            name: self.label().into(),
            ..Default::default()
        };
        match self {
            Kind::Granular => return None,
            Kind::Drums => preset.drums = Some(Box::default()),
            Kind::Synth => preset.synth = Some(Box::default()),
        }
        Some(preset)
    }
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Mix {
    pub gain: f32,
    pub muted: bool,
    pub solo: bool,
    /// 0 = any channel, 1–16 = a specific incoming MIDI channel.
    pub channel: u8,
}
impl Default for Mix {
    fn default() -> Self {
        Self {
            gain: 1.,
            muted: false,
            solo: false,
            channel: 0,
        }
    }
}
impl Mix {
    pub fn sanitize(&mut self) {
        self.gain = if self.gain.is_finite() {
            self.gain.clamp(0., 2.)
        } else {
            1.
        };
        self.channel = self.channel.min(16);
    }
    pub fn accepts(self, message: crate::midi::Message) -> bool {
        use crate::midi::Message::*;
        let channel = match message {
            NoteOn { channel, .. }
            | NoteOff { channel, .. }
            | Control { channel, .. }
            | PitchBend { channel, .. }
            | Pressure { channel, .. }
            | PolyPressure { channel, .. }
            | Program { channel, .. } => Some(channel),
            _ => None,
        };
        self.channel == 0 || channel.is_none_or(|c| c + 1 == self.channel)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Meta {
    pub id: u32,
    pub name: String,
    pub kind: Kind,
    #[serde(flatten)]
    pub mix: Mix,
}
impl Default for Meta {
    fn default() -> Self {
        Self {
            id: 0,
            name: "Granular 1".into(),
            kind: Kind::Granular,
            mix: Mix::default(),
        }
    }
}
impl Meta {
    pub fn sanitize(&mut self) {
        self.mix.sanitize();
        self.name = self
            .name
            .trim()
            .chars()
            .filter(|c| !c.is_control())
            .take(80)
            .collect();
        if self.name.is_empty() {
            self.name = self.kind.label().into();
        }
    }
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Context {
    pub name: String,
    pub active: u32,
    pub revision: u64,
    pub timbres: Vec<Meta>,
}
