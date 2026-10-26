//! Advanced grain mode: grains are dealt to four groups, each with its own
//! pitch, length, placement, level, filter and two envelopes that run over
//! every grain's own life. A group's amp envelope is its grains' window,
//! curved by its shape. A group's controls that the engine also has move
//! from the engine's value, so untouched groups play as the engine does.
use serde::{Deserialize, Serialize};

pub const GROUPS: usize = 4;
pub const NAMES: [&str; GROUPS] = ["A", "B", "C", "D"];
/// A group's pitch offset reaches this many semitones either way.
pub const PITCH_RANGE: f32 = 24.0;
/// A group's grains are up to this many octaves shorter or longer.
pub const SIZE_OCTAVES: [f32; 2] = [-3.0, 2.0];

/// The pitches a group's grains snap to, as intervals above the note they
/// play. Host values index `ALL`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Scale {
    #[default]
    Off,
    Chromatic,
    Major,
    Minor,
    Dorian,
    Mixolydian,
    HarmonicMinor,
    PentatonicMajor,
    PentatonicMinor,
    WholeTone,
    Fifths,
    Octaves,
    /// The notes held, in any octave.
    Chord,
}
impl Scale {
    pub const ALL: [Self; 13] = [
        Self::Off,
        Self::Chromatic,
        Self::Major,
        Self::Minor,
        Self::Dorian,
        Self::Mixolydian,
        Self::HarmonicMinor,
        Self::PentatonicMajor,
        Self::PentatonicMinor,
        Self::WholeTone,
        Self::Fifths,
        Self::Octaves,
        Self::Chord,
    ];
    /// Bit n set for each interval of n semitones in an octave; None for
    /// Off and for Chord, whose intervals come from the notes held.
    pub fn intervals(self) -> Option<u16> {
        let steps: &[u8] = match self {
            Self::Off | Self::Chord => return None,
            Self::Chromatic => &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            Self::Major => &[0, 2, 4, 5, 7, 9, 11],
            Self::Minor => &[0, 2, 3, 5, 7, 8, 10],
            Self::Dorian => &[0, 2, 3, 5, 7, 9, 10],
            Self::Mixolydian => &[0, 2, 4, 5, 7, 9, 10],
            Self::HarmonicMinor => &[0, 2, 3, 5, 7, 8, 11],
            Self::PentatonicMajor => &[0, 2, 4, 7, 9],
            Self::PentatonicMinor => &[0, 3, 5, 7, 10],
            Self::WholeTone => &[0, 2, 4, 6, 8, 10],
            Self::Fifths => &[0, 7],
            Self::Octaves => &[0],
        };
        Some(steps.iter().fold(0, |mask, &step| mask | 1 << step))
    }
}

/// `semitones` moved to the nearest interval in `intervals` (bit n for n
/// semitones above the note, in every octave). An empty set leaves it.
pub fn quantize(semitones: f32, intervals: u16) -> f32 {
    let intervals = intervals & 0xfff;
    if intervals == 0 || !semitones.is_finite() {
        return semitones;
    }
    let octave = (semitones / 12.0).floor();
    let mut best = semitones;
    let mut distance = f32::INFINITY;
    // The nearest interval may sit in the octave below or above.
    for o in [octave - 1.0, octave, octave + 1.0] {
        for step in (0..12).filter(|s| intervals & 1 << s != 0) {
            let candidate = o * 12.0 + step as f32;
            let d = (candidate - semitones).abs();
            if d < distance {
                distance = d;
                best = candidate;
            }
        }
    }
    best
}
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
    /// The engine's grain window: up over the first half, down over the second.
    fn default() -> Self {
        Self {
            attack: 0.5,
            decay: 0.0,
            sustain: 1.0,
            release: 0.5,
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
    /// How the amp envelope's segments bend, from straight lines to the
    /// window's half-cosine, in place of the engine's SHAPE.
    pub shape: f32,
    pub filter_type: FilterType,
    pub cutoff: f32,
    pub resonance: f32,
    /// How far the filter envelope moves the cutoff, either way.
    pub env_depth: f32,
    /// The grains' window.
    pub amp: Envelope,
    pub filter_env: Envelope,
    /// Where the group's grains go: CHAIN keeps them in the timbre, SEND
    /// feeds them to a Bus whatever CHAIN is.
    pub output: crate::taps::Tap,
    /// Octaves the grains are longer (or, below zero, shorter) than the
    /// engine's SIZE makes them.
    pub size: f32,
    /// Moves the grains from the engine's POSITION, as a share of the source.
    pub position: f32,
    /// Chance that a grain plays the other way to the engine's direction.
    pub reverse: f32,
    /// Added to the engine's SPREAD.
    pub spread: f32,
    /// Each grain's own random offsets: up to this many semitones either
    /// way, this much of the cutoff scale either way, and this much quieter.
    pub random_pitch: f32,
    pub random_cutoff: f32,
    pub random_level: f32,
    /// What the grains' pitch offset from their note snaps to.
    pub scale: Scale,
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
            output: Default::default(),
            size: 0.0,
            position: 0.0,
            reverse: 0.0,
            spread: 0.0,
            random_pitch: 0.0,
            random_cutoff: 0.0,
            random_level: 0.0,
            scale: Scale::Off,
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
        self.output.sanitize();
        let [shorter, longer] = SIZE_OCTAVES;
        crate::preset::finite_clamp(&mut self.size, shorter, longer, 0.0);
        crate::preset::finite_clamp(&mut self.position, -1.0, 1.0, 0.0);
        crate::preset::finite_clamp(&mut self.reverse, 0.0, 1.0, 0.0);
        crate::preset::finite_clamp(&mut self.spread, -1.0, 1.0, 0.0);
        crate::preset::finite_clamp(&mut self.random_pitch, 0.0, PITCH_RANGE, 0.0);
        crate::preset::finite_clamp(&mut self.random_cutoff, 0.0, 1.0, 0.0);
        crate::preset::finite_clamp(&mut self.random_level, 0.0, 1.0, 0.0);
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
/// Each group's output, appended after every group's FIELDS in host order;
/// get and set number them on from `FIELDS.len()`.
pub const OUTPUT_FIELDS: [(&str, &str, f32, f32); 3] = [
    ("chain", "chain", 0.0, 1.0),
    ("send", "bus send", 0.0, 1.0),
    ("bus", "bus", 0.0, crate::bus::MAX_BUSES as f32),
];
/// Each group's placement, randomness and scale, appended after every
/// group's output; get and set number them on from the outputs'.
pub const PLAY_FIELDS: [(&str, &str, f32, f32); 8] = [
    ("size", "size", SIZE_OCTAVES[0], SIZE_OCTAVES[1]),
    ("position", "position", -1.0, 1.0),
    ("reverse", "reverse", 0.0, 1.0),
    ("spread", "spread", -1.0, 1.0),
    ("random_pitch", "random pitch", 0.0, PITCH_RANGE),
    ("random_cutoff", "random cutoff", 0.0, 1.0),
    ("random_level", "random level", 0.0, 1.0),
    ("scale", "scale", 0.0, (Scale::ALL.len() - 1) as f32),
];
/// The first get/set field of PLAY_FIELDS.
pub const PLAY_FIELD: usize = FIELDS.len() + OUTPUT_FIELDS.len();
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
        15 => group.filter_env.release,
        16 => group.output.chain,
        17 => group.output.send,
        18 => group.output.bus as f32,
        19 => group.size,
        20 => group.position,
        21 => group.reverse,
        22 => group.spread,
        23 => group.random_pitch,
        24 => group.random_cutoff,
        25 => group.random_level,
        _ => group.scale as u8 as f32,
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
        15 => group.filter_env.release = value,
        16 => group.output.chain = value,
        17 => group.output.send = value,
        18 => group.output.bus = value.round() as u8,
        19 => group.size = value,
        20 => group.position = value,
        21 => group.reverse = value,
        22 => group.spread = value,
        23 => group.random_pitch = value,
        24 => group.random_cutoff = value,
        25 => group.random_level = value,
        _ => group.scale = Scale::ALL[(value.round().max(0.0) as usize).min(Scale::ALL.len() - 1)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pitches_snap_to_the_nearest_interval_in_any_octave() {
        let minor = Scale::Minor.intervals().unwrap();
        assert_eq!(quantize(0.4, minor), 0.0);
        assert_eq!(quantize(3.6, minor), 3.0);
        assert_eq!(quantize(6.4, minor), 7.0);
        assert_eq!(quantize(11.2, minor), 12.0);
        assert_eq!(quantize(-1.4, minor), -2.0);
        assert_eq!(quantize(-13.0, Scale::Octaves.intervals().unwrap()), -12.0);
        assert_eq!(quantize(5.3, 0), 5.3);
        for scale in Scale::ALL.into_iter().filter_map(Scale::intervals) {
            for tenth in -300..300 {
                let snapped = quantize(tenth as f32 * 0.1, scale);
                let step = snapped.rem_euclid(12.0) as u16;
                assert!(scale & 1 << step != 0 && snapped.fract() == 0.0);
            }
        }
    }
}
