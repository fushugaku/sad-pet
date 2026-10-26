//! Advanced grain mode: grains are dealt to four groups, each with its own
//! pitch, window shape, level, filter and two envelopes that run over every
//! grain's own life.
use serde::{Deserialize, Serialize};

pub const GROUPS: usize = 4;
pub const NAMES: [&str; GROUPS] = ["A", "B", "C", "D"];
/// A group's pitch offset reaches this many semitones either way.
pub const PITCH_RANGE: f32 = 24.0;

/// The response of a group's filter. Host values index `ALL`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterType {
    #[default]
    LowPass,
    BandPass,
    HighPass,
}
impl FilterType {
    pub const ALL: [Self; 3] = [Self::LowPass, Self::BandPass, Self::HighPass];
}

/// An ADSR over one grain's life: attack, decay and release are shares of
/// the grain's length, scaled down together when they add up to more.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Envelope {
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
}
impl Default for Envelope {
    /// Flat: the grain keeps its window alone.
    fn default() -> Self {
        Self {
            attack: 0.0,
            decay: 0.0,
            sustain: 1.0,
            release: 0.0,
        }
    }
}
impl Envelope {
    fn filter_default() -> Self {
        Self {
            attack: 0.0,
            decay: 0.5,
            sustain: 0.0,
            release: 0.0,
        }
    }
    fn sanitize(&mut self, fallback: Self) {
        for (value, fallback) in [
            (&mut self.attack, fallback.attack),
            (&mut self.decay, fallback.decay),
            (&mut self.sustain, fallback.sustain),
            (&mut self.release, fallback.release),
        ] {
            crate::preset::finite_clamp(value, 0.0, 1.0, fallback);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Group {
    /// Share of the spawned grains this group takes; 0 switches it off.
    pub share: f32,
    pub level: f32,
    /// Semitones added to the engine's pitch.
    pub pitch: f32,
    /// Window shape, in place of the engine's SHAPE.
    pub shape: f32,
    pub filter_type: FilterType,
    pub cutoff: f32,
    pub resonance: f32,
    /// How far the filter envelope moves the cutoff, either way.
    pub env_depth: f32,
    pub amp: Envelope,
    pub filter_env: Envelope,
}
impl Default for Group {
    fn default() -> Self {
        Self {
            share: 1.0,
            level: 1.0,
            pitch: 0.0,
            shape: crate::preset::Params::default().grain_shape,
            filter_type: FilterType::LowPass,
            cutoff: 1.0,
            resonance: 0.1,
            env_depth: 0.0,
            amp: Envelope::default(),
            filter_env: Envelope::filter_default(),
        }
    }
}
impl Group {
    fn sanitize(&mut self) {
        let fallback = Self::default();
        crate::preset::finite_clamp(&mut self.share, 0.0, 1.0, fallback.share);
        crate::preset::finite_clamp(&mut self.level, 0.0, 1.0, fallback.level);
        crate::preset::finite_clamp(&mut self.pitch, -PITCH_RANGE, PITCH_RANGE, 0.0);
        crate::preset::finite_clamp(&mut self.shape, 0.0, 1.0, fallback.shape);
        crate::preset::finite_clamp(&mut self.cutoff, 0.0, 1.0, fallback.cutoff);
        crate::preset::finite_clamp(&mut self.resonance, 0.0, 1.0, fallback.resonance);
        crate::preset::finite_clamp(&mut self.env_depth, -1.0, 1.0, 0.0);
        self.amp.sanitize(fallback.amp);
        self.filter_env.sanitize(fallback.filter_env);
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// Off, every grain plays the engine's own controls and the groups wait.
    pub advanced: bool,
    pub groups: [Group; GROUPS],
}
impl Settings {
    /// Presets without advanced mode serialize as before.
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }
    pub fn sanitize(&mut self) {
        self.groups.iter_mut().for_each(Group::sanitize);
    }
}

/// Host controls of each group, in host order: (key, name, min, max).
pub const FIELDS: [(&str, &str, f32, f32); 16] = [
    ("share", "share", 0.0, 1.0),
    ("level", "level", 0.0, 1.0),
    ("pitch", "pitch", -PITCH_RANGE, PITCH_RANGE),
    ("shape", "shape", 0.0, 1.0),
    ("filter_type", "filter type", 0.0, 2.0),
    ("cutoff", "cutoff", 0.0, 1.0),
    ("resonance", "resonance", 0.0, 1.0),
    ("env_depth", "filter envelope", -1.0, 1.0),
    ("amp_attack", "amp attack", 0.0, 1.0),
    ("amp_decay", "amp decay", 0.0, 1.0),
    ("amp_sustain", "amp sustain", 0.0, 1.0),
    ("amp_release", "amp release", 0.0, 1.0),
    ("filter_attack", "filter attack", 0.0, 1.0),
    ("filter_decay", "filter decay", 0.0, 1.0),
    ("filter_sustain", "filter sustain", 0.0, 1.0),
    ("filter_release", "filter release", 0.0, 1.0),
];
pub fn get(group: &Group, field: usize) -> f32 {
    match field {
        0 => group.share,
        1 => group.level,
        2 => group.pitch,
        3 => group.shape,
        4 => group.filter_type as u8 as f32,
        5 => group.cutoff,
        6 => group.resonance,
        7 => group.env_depth,
        8 => group.amp.attack,
        9 => group.amp.decay,
        10 => group.amp.sustain,
        11 => group.amp.release,
        12 => group.filter_env.attack,
        13 => group.filter_env.decay,
        14 => group.filter_env.sustain,
        _ => group.filter_env.release,
    }
}
pub fn set(group: &mut Group, field: usize, value: f32) {
    match field {
        0 => group.share = value,
        1 => group.level = value,
        2 => group.pitch = value,
        3 => group.shape = value,
        4 => group.filter_type = FilterType::ALL[(value.round() as usize).min(2)],
        5 => group.cutoff = value,
        6 => group.resonance = value,
        7 => group.env_depth = value,
        8 => group.amp.attack = value,
        9 => group.amp.decay = value,
        10 => group.amp.sustain = value,
        11 => group.amp.release = value,
        12 => group.filter_env.attack = value,
        13 => group.filter_env.decay = value,
        14 => group.filter_env.sustain = value,
        _ => group.filter_env.release = value,
    }
}
