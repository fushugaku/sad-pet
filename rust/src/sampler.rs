//! Sampler timbres play a kit of pads in the manner of an MPC. Every pad has
//! zones (velocity layers or key groups) that point into samples, each with
//! its own start/end, loop, crossfade, fades, tuning and level; the pad adds
//! the trigger mode, choke group, amp envelope, filter and pitch envelope.
//! The kit's voices feed the timbre's chain the way a Bus input does, so the
//! chain starts off and the kit passes through unchanged.
//!
//! A pad may send its voices through its own Pad FX timbre (Granular,
//! Filters, Resynth, effect racks). Bouncing renders every note of a pad
//! through that chain into plain samples, which the pad then plays instead.
use serde::{Deserialize, Serialize};

pub mod bounce;
pub mod edit;

/// Eight banks of sixteen pads, as on the MPC.
pub const MAX_PADS: usize = 128;
pub const BANK_PADS: usize = 16;
/// A bounce makes one zone per rendered note.
pub const MAX_ZONES: usize = 128;
pub const MAX_VOICES: usize = 64;
/// Mute groups 1–16; 0 is none.
pub const MAX_CHOKE: u8 = 16;
/// The first pad plays C1 (MIDI 36), the General MIDI kick.
pub const FIRST_NOTE: u8 = 36;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Trigger {
    /// Plays to the end of the sample whatever the note length.
    #[default]
    OneShot,
    /// Releases when the note ends.
    NoteOn,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LoopMode {
    #[default]
    Off,
    /// Loop start to the end, forever (until released).
    Forward,
    /// Back and forth between the loop start and the end.
    Alternate,
    /// Loops while the note is held, then plays on to the end.
    Sustain,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LayerMode {
    /// Every zone whose key and velocity range match plays together.
    #[default]
    Velocity,
    /// Matching zones take turns.
    Cycle,
    /// One matching zone at random.
    Random,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterKind {
    #[default]
    Off,
    Lowpass,
    Highpass,
    Bandpass,
    Notch,
}

fn finite(value: f32, lo: f32, hi: f32, default: f32) -> f32 {
    if value.is_finite() {
        value.clamp(lo, hi)
    } else {
        default
    }
}

fn keys(range: [u8; 2], lo: u8) -> [u8; 2] {
    let a = range[0].clamp(lo, 127);
    let b = range[1].clamp(lo, 127);
    [a.min(b), a.max(b)]
}

/// One sample region of a pad: a velocity layer, a key group or both.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Zone {
    /// The pool id of the sample, as the library stores it.
    pub sample: String,
    /// First frame played.
    pub start: u32,
    /// Frame after the last one played; 0 is the end of the sample.
    pub end: u32,
    pub loop_mode: LoopMode,
    /// The loop runs from here to `end`.
    pub loop_start: u32,
    /// Milliseconds of audio before the loop start blended into the loop's
    /// end, so a forward loop joins without a click.
    pub crossfade: f32,
    pub reverse: bool,
    /// Milliseconds at the start and end of the region.
    pub fade_in: f32,
    pub fade_out: f32,
    /// Shape of each fade: −1 fast (logarithmic), 0 linear, 1 slow (exponential).
    pub fade_curve: [f32; 2],
    /// The note at which the sample plays at its own pitch.
    pub root: u8,
    /// Semitones and cents.
    pub tune: f32,
    pub fine: f32,
    /// Decibels.
    pub gain: f32,
    /// −1 left … 1 right.
    pub pan: f32,
    /// Lowest and highest note, and velocity, that play this zone.
    pub keys: [u8; 2],
    pub velocities: [u8; 2],
}
impl Default for Zone {
    fn default() -> Self {
        Self {
            sample: String::new(),
            start: 0,
            end: 0,
            loop_mode: LoopMode::Off,
            loop_start: 0,
            crossfade: 0.0,
            reverse: false,
            fade_in: 0.0,
            fade_out: 0.0,
            fade_curve: [0.0; 2],
            root: 60,
            tune: 0.0,
            fine: 0.0,
            gain: 0.0,
            pan: 0.0,
            keys: [0, 127],
            velocities: [1, 127],
        }
    }
}
impl Zone {
    pub fn new(sample: &str) -> Self {
        Self {
            sample: sample.into(),
            ..Self::default()
        }
    }
    pub fn sanitize(&mut self) {
        self.sample = sample_id(&self.sample);
        if self.end != 0 && self.end <= self.start {
            self.end = self.start.saturating_add(1);
        }
        self.crossfade = finite(self.crossfade, 0.0, 10_000.0, 0.0);
        self.fade_in = finite(self.fade_in, 0.0, 60_000.0, 0.0);
        self.fade_out = finite(self.fade_out, 0.0, 60_000.0, 0.0);
        for curve in &mut self.fade_curve {
            *curve = finite(*curve, -1.0, 1.0, 0.0);
        }
        self.root = self.root.min(127);
        self.tune = finite(self.tune, -48.0, 48.0, 0.0);
        self.fine = finite(self.fine, -100.0, 100.0, 0.0);
        self.gain = finite(self.gain, -60.0, 12.0, 0.0);
        self.pan = finite(self.pan, -1.0, 1.0, 0.0);
        self.keys = keys(self.keys, 0);
        self.velocities = keys(self.velocities, 1);
    }
    /// Plays for this note and velocity (1–127).
    pub fn matches(&self, note: u8, velocity: u8) -> bool {
        (self.keys[0]..=self.keys[1]).contains(&note)
            && (self.velocities[0]..=self.velocities[1]).contains(&velocity.max(1))
    }
}

/// A sample pool id: a library id or a bounce id, printable ASCII.
pub fn sample_id(id: &str) -> String {
    id.chars()
        .filter(|c| c.is_ascii_graphic())
        .take(160)
        .collect()
}

/// Times in milliseconds; sustain 0–1.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Envelope {
    pub attack: f32,
    pub hold: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
}
impl Default for Envelope {
    fn default() -> Self {
        Self {
            attack: 0.0,
            hold: 0.0,
            decay: 300.0,
            sustain: 1.0,
            release: 30.0,
        }
    }
}
impl Envelope {
    fn sanitize(&mut self) {
        self.attack = finite(self.attack, 0.0, 30_000.0, 0.0);
        self.hold = finite(self.hold, 0.0, 30_000.0, 0.0);
        self.decay = finite(self.decay, 1.0, 30_000.0, 300.0);
        self.sustain = finite(self.sustain, 0.0, 1.0, 1.0);
        self.release = finite(self.release, 1.0, 30_000.0, 30.0);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Filter {
    pub kind: FilterKind,
    /// Hertz.
    pub cutoff: f32,
    /// 0–1; near 1 it rings.
    pub resonance: f32,
    /// Octaves the attack/decay envelope moves the cutoff, −8…8.
    pub envelope: f32,
    pub attack: f32,
    pub decay: f32,
    /// 0–1: how far the cutoff follows the note from C4.
    pub keytrack: f32,
    /// Octaves the cutoff falls for the softest note, 0–8.
    pub velocity: f32,
}
impl Default for Filter {
    fn default() -> Self {
        Self {
            kind: FilterKind::Off,
            cutoff: 8_000.0,
            resonance: 0.1,
            envelope: 0.0,
            attack: 0.0,
            decay: 300.0,
            keytrack: 0.0,
            velocity: 0.0,
        }
    }
}
impl Filter {
    fn sanitize(&mut self) {
        self.cutoff = finite(self.cutoff, 20.0, 20_000.0, 8_000.0);
        self.resonance = finite(self.resonance, 0.0, 1.0, 0.1);
        self.envelope = finite(self.envelope, -8.0, 8.0, 0.0);
        self.attack = finite(self.attack, 0.0, 30_000.0, 0.0);
        self.decay = finite(self.decay, 1.0, 30_000.0, 300.0);
        self.keytrack = finite(self.keytrack, 0.0, 1.0, 0.0);
        self.velocity = finite(self.velocity, 0.0, 8.0, 0.0);
    }
}

/// A pitch offset that decays back to the note, as on drum pads.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PitchEnvelope {
    /// Semitones at the start of the note, −48…48.
    pub amount: f32,
    /// Milliseconds to fall to the note.
    pub decay: f32,
}
impl Default for PitchEnvelope {
    fn default() -> Self {
        Self {
            amount: 0.0,
            decay: 100.0,
        }
    }
}

/// How the note's velocity shapes it, each 0–1.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Velocity {
    /// 0 every note at full level, 1 level follows velocity.
    pub level: f32,
    /// Softer notes start later by up to this share of the region.
    pub start: f32,
    /// Softer notes start the amp attack slower, up to 4× its time.
    pub attack: f32,
}
impl Default for Velocity {
    fn default() -> Self {
        Self {
            level: 1.0,
            start: 0.0,
            attack: 0.0,
        }
    }
}

/// How a pad was bounced, so a new bounce can start from the same choices.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BounceOptions {
    /// Lowest and highest note rendered, every `step` semitones.
    pub low: u8,
    pub high: u8,
    pub step: u8,
    pub velocity: u8,
    /// Seconds the note is held before its release.
    pub hold: f32,
    /// Most seconds kept after the release, while anything still sounds.
    pub tail: f32,
}
impl Default for BounceOptions {
    fn default() -> Self {
        Self {
            low: 36,
            high: 84,
            step: 3,
            velocity: 127,
            hold: 1.0,
            tail: 4.0,
        }
    }
}
impl BounceOptions {
    pub fn sanitize(&mut self) {
        let [low, high] = keys([self.low, self.high], 0);
        self.low = low;
        self.high = high;
        self.step = self.step.clamp(1, 12);
        self.velocity = self.velocity.clamp(1, 127);
        self.hold = finite(self.hold, 0.01, 30.0, 1.0);
        self.tail = finite(self.tail, 0.0, 30.0, 4.0);
    }
    /// The notes a bounce renders, each covering up to `step` keys.
    pub fn notes(&self) -> Vec<u8> {
        (self.low..=self.high).step_by(self.step as usize).collect()
    }
}

/// The rendered notes of a bounced pad, played instead of its zones.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Bounced {
    pub zones: Vec<Zone>,
    pub options: BounceOptions,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Pad {
    /// Stable identity: sequencer tracks and Pad FX timbres name it.
    pub id: u32,
    pub name: String,
    /// Lowest and highest MIDI note that plays the pad.
    pub keys: [u8; 2],
    /// Notes change the pitch from each zone's root; otherwise every note
    /// plays the zone at its root.
    pub keytrack: bool,
    pub trigger: Trigger,
    /// A new note cuts the pad's sounding notes.
    pub mono: bool,
    /// Mute group: a note cuts the other pads of the same group.
    pub choke: u8,
    pub layers: LayerMode,
    pub zones: Vec<Zone>,
    pub tune: f32,
    pub fine: f32,
    pub gain: f32,
    pub pan: f32,
    pub envelope: Envelope,
    pub filter: Filter,
    pub pitch: PitchEnvelope,
    pub velocity: Velocity,
    pub muted: bool,
    pub solo: bool,
    pub bounce: Option<Box<Bounced>>,
}
impl Default for Pad {
    fn default() -> Self {
        Self {
            id: 1,
            name: String::new(),
            keys: [FIRST_NOTE; 2],
            keytrack: false,
            trigger: Trigger::OneShot,
            mono: false,
            choke: 0,
            layers: LayerMode::Velocity,
            zones: Vec::new(),
            tune: 0.0,
            fine: 0.0,
            gain: 0.0,
            pan: 0.0,
            envelope: Envelope::default(),
            filter: Filter::default(),
            pitch: PitchEnvelope::default(),
            velocity: Velocity::default(),
            muted: false,
            solo: false,
            bounce: None,
        }
    }
}
impl Pad {
    pub fn new(id: u32, note: u8) -> Self {
        Self {
            id,
            keys: [note; 2],
            ..Self::default()
        }
    }
    pub fn sanitize(&mut self) {
        self.name = self
            .name
            .trim()
            .chars()
            .filter(|c| !c.is_control())
            .take(80)
            .collect();
        self.keys = keys(self.keys, 0);
        self.choke = self.choke.min(MAX_CHOKE);
        self.zones.truncate(MAX_ZONES);
        for zone in &mut self.zones {
            zone.sanitize();
        }
        self.tune = finite(self.tune, -48.0, 48.0, 0.0);
        self.fine = finite(self.fine, -100.0, 100.0, 0.0);
        self.gain = finite(self.gain, -60.0, 12.0, 0.0);
        self.pan = finite(self.pan, -1.0, 1.0, 0.0);
        self.envelope.sanitize();
        self.filter.sanitize();
        self.pitch.amount = finite(self.pitch.amount, -48.0, 48.0, 0.0);
        self.pitch.decay = finite(self.pitch.decay, 1.0, 10_000.0, 100.0);
        self.velocity.level = finite(self.velocity.level, 0.0, 1.0, 1.0);
        self.velocity.start = finite(self.velocity.start, 0.0, 1.0, 0.0);
        self.velocity.attack = finite(self.velocity.attack, 0.0, 1.0, 0.0);
        if let Some(bounce) = &mut self.bounce {
            bounce.zones.truncate(MAX_ZONES);
            for zone in &mut bounce.zones {
                zone.sanitize();
            }
            bounce.options.sanitize();
            if bounce.zones.is_empty() {
                self.bounce = None;
            }
        }
    }
    /// The zones that play: the bounce's, once the pad is bounced.
    pub fn playing_zones(&self) -> &[Zone] {
        self.bounce.as_ref().map_or(&self.zones, |b| &b.zones)
    }
    /// Every sample the pad needs, its bounce included.
    pub fn samples(&self) -> impl Iterator<Item = &str> {
        self.zones
            .iter()
            .chain(self.bounce.iter().flat_map(|b| &b.zones))
            .map(|z| z.sample.as_str())
            .filter(|s| !s.is_empty())
    }
}

/// What a Sampler timbre saves with its preset.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub pads: Vec<Pad>,
    /// Voices sounding at once, 1–64.
    pub polyphony: u8,
    /// The pad the editor shows; it keeps its place across sessions.
    pub selected: u32,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            pads: Vec::new(),
            polyphony: 32,
            selected: 1,
        }
    }
}
impl Settings {
    /// Bank A: sixteen empty pads on C1 and up.
    pub fn new() -> Self {
        Self {
            pads: (0..BANK_PADS as u32)
                .map(|i| Pad::new(i + 1, FIRST_NOTE + i as u8))
                .collect(),
            ..Self::default()
        }
    }
    pub fn sanitize(&mut self) {
        self.pads.truncate(MAX_PADS);
        self.polyphony = self.polyphony.clamp(1, MAX_VOICES as u8);
        let mut ids = std::collections::BTreeSet::new();
        let mut next = self
            .pads
            .iter()
            .map(|p| p.id)
            .max()
            .unwrap_or(0)
            .saturating_add(1);
        for pad in &mut self.pads {
            pad.sanitize();
            if pad.id == 0 || !ids.insert(pad.id) {
                while ids.contains(&next) || next == 0 {
                    next = next.wrapping_add(1);
                }
                pad.id = next;
                ids.insert(next);
            }
        }
        if !self.pads.iter().any(|p| p.id == self.selected) {
            self.selected = self.pads.first().map_or(0, |p| p.id);
        }
    }
    pub fn pad(&self, id: u32) -> Option<&Pad> {
        self.pads.iter().find(|p| p.id == id)
    }
    pub fn pad_mut(&mut self, id: u32) -> Option<&mut Pad> {
        self.pads.iter_mut().find(|p| p.id == id)
    }
    /// Every sample id the kit plays, once each.
    pub fn samples(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .pads
            .iter()
            .flat_map(Pad::samples)
            .map(str::to_string)
            .collect();
        ids.sort();
        ids.dedup();
        ids
    }
    /// The lowest pad id no pad uses yet.
    pub fn free_id(&self) -> u32 {
        (1..).find(|id| self.pad(*id).is_none()).unwrap_or(u32::MAX)
    }
}

/// What the editor shows of a kit while it plays.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct View {
    /// Each sounding voice: pad id, zone index and the frame it plays.
    pub voices: Vec<(u32, usize, f64)>,
    /// Notes played per pad id, counting up.
    pub hits: Vec<(u32, u32)>,
    /// The pool ids the engine holds.
    pub loaded: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_kits_have_a_bank_of_pads_on_consecutive_notes() {
        let mut kit = Settings::new();
        kit.sanitize();
        assert_eq!(kit.pads.len(), 16);
        assert_eq!(kit.pads[0].keys, [36, 36]);
        assert_eq!(kit.pads[15].keys, [51, 51]);
        assert_eq!(kit.selected, 1);
    }
    #[test]
    fn sanitize_repairs_identities_ranges_and_values() {
        let mut kit = Settings::new();
        kit.pads[1].id = 1;
        kit.pads[2].keys = [90, 20];
        kit.pads[3].zones.push(Zone {
            sample: "a b\u{7}c".into(),
            start: 100,
            end: 50,
            velocities: [0, 0],
            gain: f32::NAN,
            ..Zone::default()
        });
        kit.polyphony = 200;
        kit.sanitize();
        let ids: std::collections::BTreeSet<_> = kit.pads.iter().map(|p| p.id).collect();
        assert_eq!(ids.len(), 16);
        assert_eq!(kit.pads[2].keys, [20, 90]);
        let zone = &kit.pads[3].zones[0];
        assert_eq!(zone.sample, "abc");
        assert_eq!(zone.end, 101);
        assert_eq!(zone.velocities, [1, 1]);
        assert_eq!(zone.gain, 0.0);
        assert_eq!(kit.polyphony, 64);
    }
    #[test]
    fn bounce_notes_cover_the_range_in_steps() {
        let options = BounceOptions {
            low: 36,
            high: 48,
            step: 5,
            ..Default::default()
        };
        assert_eq!(options.notes(), vec![36, 41, 46]);
    }
}
