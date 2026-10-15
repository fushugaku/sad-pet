//! Sample-clocked note generation shared by realtime playback and WAV rendering.
use crate::{midi::Message, tables::Random};
use serde::{Deserialize, Serialize};

pub const STEPS: usize = 16;
/// The audio engine has eight polyphonic voices.
pub const MAX_STEP_NOTES: usize = 8;
const MAX_CHORD: usize = 32;
pub type Actions = [Option<Message>; 2];
pub type TickActions = [Option<Message>; MAX_STEP_NOTES * 2];

/// A compact chord that can be copied in the audio loop without allocating.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StepNotes {
    notes: [u8; MAX_STEP_NOTES],
    len: u8,
}
impl StepNotes {
    pub fn as_slice(&self) -> &[u8] {
        &self.notes[..self.len as usize]
    }
    pub fn toggle(&mut self, note: u8) {
        if let Some(index) = self.as_slice().iter().position(|&n| n == note) {
            self.notes.copy_within(index + 1..self.len as usize, index);
            self.len -= 1;
            self.notes[self.len as usize] = 0;
        } else if note <= 127 && (self.len as usize) < MAX_STEP_NOTES {
            self.notes[self.len as usize] = note;
            self.len += 1;
        }
    }
    pub fn transpose(&mut self, semitones: i16) {
        *self = self
            .as_slice()
            .iter()
            .map(|&note| (note as i16 + semitones).clamp(0, 127) as u8)
            .collect();
    }
}
impl FromIterator<u8> for StepNotes {
    fn from_iter<T: IntoIterator<Item = u8>>(notes: T) -> Self {
        let mut result = Self::default();
        for note in notes {
            if note <= 127 && !result.as_slice().contains(&note) {
                if result.len as usize == MAX_STEP_NOTES {
                    break;
                }
                result.toggle(note);
            }
        }
        result
    }
}
impl<const N: usize> From<[u8; N]> for StepNotes {
    fn from(notes: [u8; N]) -> Self {
        notes.into_iter().collect()
    }
}
impl Serialize for StepNotes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_slice().serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for StepNotes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Vec::<u8>::deserialize(deserializer)?.into_iter().collect())
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    #[default]
    Keys,
    Hold,
    Arp,
    Sequencer,
    SequencerArp,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    #[default]
    Up,
    Down,
    UpDown,
    Random,
    AsPlayed,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(from = "LegacyStep")]
pub struct Step {
    pub notes: StepNotes,
    pub velocity: u8,
    pub enabled: bool,
}
impl Default for Step {
    fn default() -> Self {
        Self {
            notes: [60].into(),
            velocity: 100,
            enabled: true,
        }
    }
}

#[derive(Deserialize)]
#[serde(default)]
struct LegacyStep {
    notes: Option<StepNotes>,
    note: u8,
    velocity: u8,
    enabled: bool,
}
impl Default for LegacyStep {
    fn default() -> Self {
        Self {
            notes: None,
            note: 60,
            velocity: 100,
            enabled: true,
        }
    }
}
impl From<LegacyStep> for Step {
    fn from(step: LegacyStep) -> Self {
        Self {
            notes: step.notes.unwrap_or_else(|| [step.note.min(127)].into()),
            velocity: step.velocity,
            enabled: step.enabled,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub mode: Mode,
    pub hold: bool,
    pub direction: Direction,
    pub octaves: u8,
    /// Quarter-note beats per arpeggiator pulse and sequence step.
    pub arp_division: f32,
    pub arp_gate: f32,
    pub sequence_division: f32,
    pub sequence_gate: f32,
    pub length: u8,
    pub steps: [Step; STEPS],
    pub chord: Vec<u8>,
}
impl Default for Settings {
    fn default() -> Self {
        let notes = [
            60, 64, 67, 72, 67, 64, 62, 65, 69, 74, 69, 65, 59, 62, 67, 71,
        ];
        Self {
            mode: Mode::Keys,
            hold: true,
            direction: Direction::Up,
            octaves: 1,
            arp_division: 0.25,
            arp_gate: 0.65,
            sequence_division: 1.0,
            sequence_gate: 0.75,
            length: 16,
            steps: std::array::from_fn(|i| Step {
                notes: [notes[i]].into(),
                ..Step::default()
            }),
            chord: vec![60, 64, 67],
        }
    }
}
impl Settings {
    pub fn sanitize(&mut self) {
        self.octaves = self.octaves.clamp(1, 4);
        self.length = self.length.clamp(1, 16);
        for (value, min, max, fallback) in [
            (&mut self.arp_division, 0.0625, 8.0, 0.25),
            (&mut self.sequence_division, 0.0625, 16.0, 1.0),
            (&mut self.arp_gate, 0.05, 0.95, 0.65),
            (&mut self.sequence_gate, 0.05, 0.95, 0.75),
        ] {
            *value = if value.is_finite() {
                value.clamp(min, max)
            } else {
                fallback
            };
        }
        for step in &mut self.steps {
            step.velocity = step.velocity.clamp(1, 127);
        }
        self.chord.truncate(MAX_CHORD);
        self.chord.retain(|&n| n <= 127);
        let mut seen = [false; 128];
        self.chord.retain(|&n| {
            let unique = !seen[n as usize];
            seen[n as usize] = true;
            unique
        });
    }
}

/// Numeric performance controls copied without allocating in the DSP loop.
#[derive(Clone, Copy)]
pub struct Modulation {
    pub arp_rate: f32,
    pub arp_gate: f32,
    pub arp_octaves: f32,
    pub sequence_rate: f32,
    pub sequence_gate: f32,
    pub sequence_length: f32,
    pub steps: [Step; STEPS],
}
impl Default for Modulation {
    fn default() -> Self {
        Self::from_settings(&Settings::default())
    }
}
impl Modulation {
    pub fn from_settings(s: &Settings) -> Self {
        Self {
            arp_rate: s.arp_division,
            arp_gate: s.arp_gate,
            arp_octaves: s.octaves as f32,
            sequence_rate: s.sequence_division,
            sequence_gate: s.sequence_gate,
            sequence_length: s.length as f32,
            steps: s.steps,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Key {
    pub channel: u8,
    pub note: u8,
    pub velocity: u8,
}
impl Key {
    fn on(self) -> Message {
        Message::NoteOn {
            channel: self.channel,
            note: self.note,
            velocity: self.velocity,
        }
    }
    fn off(self) -> Message {
        Message::NoteOff {
            channel: self.channel,
            note: self.note,
            velocity: 0,
        }
    }
}

#[derive(Clone, Copy)]
struct Notes {
    keys: [Key; MAX_CHORD],
    len: usize,
}
impl Default for Notes {
    fn default() -> Self {
        Self {
            keys: [Key::default(); MAX_CHORD],
            len: 0,
        }
    }
}
impl Notes {
    fn index(&self, key: Key) -> Option<usize> {
        (0..self.len)
            .find(|&i| self.keys[i].channel == key.channel && self.keys[i].note == key.note)
    }
    fn add(&mut self, key: Key) {
        if let Some(i) = self.index(key) {
            self.keys[i] = key;
        } else if self.len < MAX_CHORD {
            self.keys[self.len] = key;
            self.len += 1;
        }
    }
    fn remove(&mut self, key: Key) {
        if let Some(i) = self.index(key) {
            self.keys.copy_within(i + 1..self.len, i);
            self.len -= 1;
        }
    }
    fn clear(&mut self) {
        self.len = 0;
    }
    fn from_notes(notes: &[u8]) -> Self {
        let mut result = Self::default();
        for &note in notes {
            result.add(Key {
                channel: 0,
                note,
                velocity: 100,
            });
        }
        result
    }
    fn from_step(step: Step) -> Self {
        let mut result = Self::from_notes(step.notes.as_slice());
        for key in &mut result.keys[..result.len] {
            key.velocity = step.velocity;
        }
        result
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct View {
    pub mode: Mode,
    pub step: Option<usize>,
    pub arp_index: usize,
    pub chord: Vec<u8>,
    pub gate: bool,
    pub beat: f64,
}

pub struct Performance {
    settings: Settings,
    physical: Notes,
    latched: Notes,
    active: Notes,
    beat: f64,
    next_arp: f64,
    next_step: f64,
    off_at: f64,
    arp_counter: usize,
    step_counter: usize,
    current_step: Option<usize>,
    last_arp_index: usize,
    running: bool,
    random: Random,
}
impl Default for Performance {
    fn default() -> Self {
        Self::new(Settings::default())
    }
}
impl Performance {
    pub fn new(mut settings: Settings) -> Self {
        settings.sanitize();
        let latched = if settings.mode == Mode::Hold
            || (matches!(settings.mode, Mode::Arp | Mode::SequencerArp) && !settings.hold)
        {
            Notes::default()
        } else {
            Notes::from_notes(&settings.chord)
        };
        if matches!(settings.mode, Mode::Arp | Mode::SequencerArp) && !settings.hold {
            settings.chord.clear();
        }
        Self {
            settings,
            physical: Notes::default(),
            latched,
            active: Notes::default(),
            beat: 0.0,
            next_arp: 0.0,
            next_step: 0.0,
            off_at: 0.0,
            arp_counter: 0,
            step_counter: 0,
            current_step: None,
            last_arp_index: 0,
            running: false,
            random: Random::default(),
        }
    }
    pub fn mode(&self) -> Mode {
        self.settings.mode
    }
    pub fn modulate(&mut self, values: Modulation) {
        self.settings.arp_division = values.arp_rate;
        self.settings.arp_gate = values.arp_gate;
        self.settings.octaves = values.arp_octaves.round() as u8;
        self.settings.sequence_division = values.sequence_rate;
        self.settings.sequence_gate = values.sequence_gate;
        self.settings.length = values.sequence_length.round() as u8;
        self.settings.steps = values.steps;
    }
    pub fn configure(&mut self, settings: &Settings) -> bool {
        let mut settings = settings.clone();
        settings.sanitize();
        let settings = &settings;
        let mode_changed = self.settings.mode != settings.mode;
        if mode_changed {
            *self = Self::new(settings.clone());
            return true;
        }
        if self.settings.chord != settings.chord {
            self.latched = Notes::from_notes(&settings.chord);
            self.arp_counter = 0;
        }
        if self.settings.arp_division != settings.arp_division {
            self.next_arp = self.beat;
        }
        if self.settings.sequence_division != settings.sequence_division {
            self.next_step = self.beat;
        }
        let release_latch = !settings.hold && matches!(self.mode(), Mode::Arp | Mode::SequencerArp);
        if release_latch {
            self.latched = self.physical;
        }
        self.settings = settings.clone();
        if release_latch {
            self.store_chord();
        }
        false
    }
    pub fn start(&mut self) {
        self.beat = 0.0;
        self.next_arp = 0.0;
        self.next_step = 0.0;
        self.arp_counter = 0;
        self.step_counter = 0;
        self.current_step = None;
        self.active.clear();
        self.running = true;
        self.random = Random::default();
    }
    pub fn stop(&mut self) {
        self.running = false;
        self.active.clear();
        self.current_step = None;
    }
    pub fn clear(&mut self) {
        self.latched.clear();
        self.physical.clear();
        self.active.clear();
        self.settings.chord.clear();
    }
    pub fn clear_channel(&mut self, channel: u8) {
        let keys = self.latched;
        for &key in &keys.keys[..keys.len] {
            if key.channel == channel {
                self.latched.remove(key);
                self.physical.remove(key);
            }
        }
        let active = self.active;
        for &key in &active.keys[..active.len] {
            if key.channel == channel {
                self.active.remove(key);
            }
        }
        self.store_chord();
    }
    fn store_chord(&mut self) {
        self.settings.chord.clear();
        self.settings
            .chord
            .extend(self.latched.keys[..self.latched.len].iter().map(|k| k.note));
    }
    pub fn stored_chord(&self) -> &[u8] {
        &self.settings.chord
    }
    pub fn hold_start(&mut self) -> ([Key; MAX_CHORD], usize) {
        if self.latched.len == 0 {
            self.latched = Notes::from_notes(&self.settings.chord);
        }
        self.store_chord();
        (self.latched.keys, self.latched.len)
    }
    pub fn view(&self) -> View {
        View {
            mode: self.mode(),
            step: self.current_step,
            arp_index: self.last_arp_index,
            chord: self.latched.keys[..self.latched.len]
                .iter()
                .map(|k| k.note)
                .collect(),
            gate: self.active.len > 0
                || (self.running && self.mode() == Mode::Hold && self.latched.len > 0),
            beat: self.beat,
        }
    }

    /// Returns None for input that should go directly to the polyphonic engine.
    pub fn input(&mut self, message: Message) -> Option<Actions> {
        if self.mode() == Mode::Keys {
            return None;
        }
        let (key, on) = match message {
            Message::NoteOn {
                channel,
                note,
                velocity,
            } if channel < 16 && note < 128 && velocity < 128 => (
                Key {
                    channel,
                    note,
                    velocity: velocity.max(1),
                },
                velocity != 0,
            ),
            Message::NoteOff { channel, note, .. } if channel < 16 && note < 128 => (
                Key {
                    channel,
                    note,
                    velocity: 0,
                },
                false,
            ),
            _ => return None,
        };
        if matches!(self.mode(), Mode::Sequencer | Mode::SequencerArp) {
            return Some([None; 2]);
        }
        let mut actions = [None; 2];
        if on {
            let fresh = self.physical.len == 0;
            if self.mode() == Mode::Hold {
                if self.latched.index(key).is_some() {
                    self.latched.remove(key);
                    actions[0] = Some(key.off());
                } else {
                    self.latched.add(key);
                    actions[0] = Some(key.on());
                }
            } else {
                if fresh {
                    self.latched.clear();
                    self.arp_counter = 0;
                    self.next_arp = self.beat;
                }
                self.latched.add(key);
            }
            self.physical.add(key);
        } else {
            self.physical.remove(key);
            if self.mode() != Mode::Hold && !self.settings.hold {
                self.latched.remove(key);
            }
        }
        self.store_chord();
        Some(actions)
    }

    fn choose_arp(&mut self, mut notes: Notes) -> Option<Key> {
        if notes.len == 0 {
            return None;
        }
        if self.settings.direction != Direction::AsPlayed {
            notes.keys[..notes.len].sort_unstable_by_key(|k| (k.note, k.channel));
        }
        let n = notes.len * self.settings.octaves as usize;
        let index = match self.settings.direction {
            Direction::Up | Direction::AsPlayed => self.arp_counter % n,
            Direction::Down => n - 1 - self.arp_counter % n,
            Direction::UpDown => {
                let period = (2 * n).saturating_sub(2).max(1);
                let i = self.arp_counter % period;
                if i < n { i } else { period - i }
            }
            Direction::Random => self.random.next_u32() as usize % n,
        };
        self.arp_counter = self.arp_counter.wrapping_add(1);
        self.last_arp_index = index;
        let mut key = notes.keys[index % notes.len];
        key.note = (key.note as usize + 12 * (index / notes.len)).min(127) as u8;
        Some(key)
    }
    fn release(&mut self, actions: &mut TickActions) {
        let offset = actions.iter().take_while(|action| action.is_some()).count();
        for (i, key) in self.active.keys[..self.active.len].iter().enumerate() {
            actions[offset + i] = Some(key.off());
        }
        self.active.clear();
    }
    fn trigger(&mut self, notes: Notes, gate: f64, actions: &mut TickActions) {
        self.release(actions);
        let offset = actions.iter().take_while(|action| action.is_some()).count();
        for (i, key) in notes.keys[..notes.len].iter().enumerate() {
            actions[offset + i] = Some(key.on());
        }
        self.active = notes;
        if notes.len > 0 {
            self.off_at = self.beat + gate;
        }
    }

    pub fn tick(&mut self, sample_rate: u32, bpm: f32) -> TickActions {
        let mut actions = [None; MAX_STEP_NOTES * 2];
        if !self.running || matches!(self.mode(), Mode::Keys | Mode::Hold) {
            return actions;
        }
        if self.active.len > 0 && self.beat + 1e-10 >= self.off_at {
            self.release(&mut actions);
        }
        let sequence = matches!(self.mode(), Mode::Sequencer | Mode::SequencerArp);
        if sequence && self.beat + 1e-10 >= self.next_step {
            let i = self.step_counter % self.settings.length as usize;
            self.current_step = Some(i);
            self.step_counter = self.step_counter.wrapping_add(1);
            self.next_step += self.settings.sequence_division as f64;
            if self.mode() == Mode::SequencerArp {
                self.arp_counter = 0;
                self.next_arp = self.beat;
            }
            if self.mode() == Mode::Sequencer {
                let step = self.settings.steps[i];
                self.trigger(
                    if step.enabled {
                        Notes::from_step(step)
                    } else {
                        Notes::default()
                    },
                    self.settings.sequence_division as f64 * self.settings.sequence_gate as f64,
                    &mut actions,
                );
            }
        }
        if matches!(self.mode(), Mode::Arp | Mode::SequencerArp) {
            let step = self.current_step.map(|i| self.settings.steps[i]);
            let notes = if self.mode() == Mode::Arp {
                self.latched
            } else {
                step.filter(|s| s.enabled)
                    .map_or_else(Notes::default, Notes::from_step)
            };
            if notes.len == 0 && self.active.len > 0 {
                self.release(&mut actions);
            }
            if self.beat + 1e-10 >= self.next_arp {
                self.next_arp += self.settings.arp_division as f64;
                let mut pulse = Notes::default();
                if let Some(key) = self.choose_arp(notes) {
                    pulse.add(key);
                }
                self.trigger(
                    pulse,
                    self.settings.arp_division as f64 * self.settings.arp_gate as f64,
                    &mut actions,
                );
            }
        }
        self.beat += bpm as f64 / (60.0 * sample_rate as f64);
        actions
    }
}
