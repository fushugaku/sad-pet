//! Stable part identities shared by the browser and native hosts.
use crate::bus::MAX_BUSES;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    #[default]
    Granular,
    Drums,
    Synth,
    /// Sings a score; its voice runs through the chain like a Bus input.
    Vocal,
    /// Plays a kit of sample pads; the kit runs through the chain like a
    /// Bus input.
    Sampler,
    /// The effect chain of one Sampler pad: a bus fed by that pad's voices,
    /// whose output returns into its Sampler.
    PadFx,
    /// Processes the sends of other timbres and feeds Master.
    Bus,
    /// Always present: the sum of every timbre and bus, then the output.
    Master,
}
impl Kind {
    pub fn label(self) -> &'static str {
        match self {
            Kind::Granular => "Granular",
            Kind::Drums => "Drums",
            Kind::Synth => "Synth",
            Kind::Vocal => "Vocal",
            Kind::Sampler => "Sampler",
            Kind::PadFx => "Pad FX",
            Kind::Bus => "Bus",
            Kind::Master => "Master",
        }
    }
    /// Bus, Master and Pad FX take their input from other timbres, not MIDI
    /// or a file.
    pub fn routing(self) -> bool {
        matches!(self, Kind::Bus | Kind::Master | Kind::PadFx)
    }
    /// Plays notes from MIDI and the keyboard; a Vocal timbre sings its score.
    pub fn takes_midi(self) -> bool {
        !self.routing() && self != Kind::Vocal
    }
    /// Keeps its engine role when a preset loads into it.
    pub fn keeps_role(self) -> bool {
        self.routing() || matches!(self, Kind::Vocal | Kind::Sampler)
    }
    /// Pad FX timbres cannot be told from a Bus by their preset; their Meta
    /// names the Sampler they belong to.
    pub fn of(preset: &crate::preset::Preset) -> Self {
        if preset.vocal.is_some() {
            Kind::Vocal
        } else if preset.sampler.is_some() {
            Kind::Sampler
        } else if let Some(bus) = &preset.bus {
            if bus.master { Kind::Master } else { Kind::Bus }
        } else if preset.drums.is_some() {
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
            Kind::Bus | Kind::Master | Kind::Vocal | Kind::Sampler | Kind::PadFx => {
                // Everything off: the input passes at unity and costs nothing.
                preset.bus = Some(Box::new(crate::bus::Settings {
                    master: self == Kind::Master,
                    ..Default::default()
                }));
                preset.params.master_volume = 1.0;
                preset.params.mix = 0.5;
                preset.color_chain.clear();
                if self == Kind::Vocal {
                    preset.vocal = Some(Box::new(crate::vocal::Settings::new()));
                }
                if self == Kind::Sampler {
                    preset.sampler = Some(Box::new(crate::sampler::Settings::new()));
                }
            }
        }
        Some(preset)
    }
}
/// Where a timbre's signal enters the chain of Master or a Bus, in chain
/// order: its input (the whole chain), Pre-filter FX, Filters or Post-filter
/// FX. A later entry skips the sections before it.
pub const ENTRIES: usize = 4;
/// Master, then Bus 1–8.
pub const DESTINATIONS: usize = MAX_BUSES + 1;
/// The level at each entry, 0–1.
pub type Entries = [f32; ENTRIES];
/// Everything at the input, as before the mixer had entries.
pub const INPUT: Entries = [1., 0., 0., 0.];
/// One frame of what enters a chain after its input.
pub type Entered = [crate::Stereo; ENTRIES - 1];
fn at_inputs(entries: &[Entries; DESTINATIONS]) -> bool {
    entries.iter().all(|e| *e == INPUT)
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Mix {
    /// Level into Master (for Master itself, the output level).
    pub gain: f32,
    pub muted: bool,
    pub solo: bool,
    /// 0 = any channel, 1–16 = a specific incoming MIDI channel.
    pub channel: u8,
    /// Send levels to Bus 1–8; Bus and Master timbres do not send.
    pub sends: [f32; MAX_BUSES],
    /// Where this timbre enters Master (0) and Bus 1–8; a Bus has only
    /// Master's, and Master none.
    #[serde(skip_serializing_if = "at_inputs")]
    pub entries: [Entries; DESTINATIONS],
}
impl Default for Mix {
    fn default() -> Self {
        Self {
            gain: 1.,
            muted: false,
            solo: false,
            channel: 0,
            sends: [0.; MAX_BUSES],
            entries: [INPUT; DESTINATIONS],
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
        for send in &mut self.sends {
            *send = if send.is_finite() {
                send.clamp(0., 2.)
            } else {
                0.
            };
        }
        for level in self.entries.iter_mut().flatten() {
            *level = if level.is_finite() {
                level.clamp(0., 1.)
            } else {
                0.
            };
        }
    }
    /// Forgets the send to a removed Bus `bus` (1–8) and where it entered;
    /// true when anything changed.
    pub fn forget_bus(&mut self, bus: u8) -> bool {
        let bus = bus as usize;
        let changed = self.sends[bus - 1] != 0. || self.entries[bus] != INPUT;
        self.sends[bus - 1] = 0.;
        self.entries[bus] = INPUT;
        changed
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
    /// 1–8 for a Bus: the send slot other timbres address; 0 otherwise.
    pub bus: u8,
    /// For a Pad FX timbre: the id of its Sampler and the pad it processes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<u32>,
    #[serde(skip_serializing_if = "is_zero")]
    pub pad: u32,
    #[serde(flatten)]
    pub mix: Mix,
}
fn is_zero(value: &u32) -> bool {
    *value == 0
}
impl Default for Meta {
    fn default() -> Self {
        Self {
            id: 0,
            name: "Granular 1".into(),
            kind: Kind::Granular,
            bus: 0,
            owner: None,
            pad: 0,
            mix: Mix::default(),
        }
    }
}
impl Meta {
    pub fn sanitize(&mut self) {
        self.mix.sanitize();
        if self.kind.routing() {
            self.mix.sends = [0.; MAX_BUSES];
            self.mix.channel = 0;
            self.mix.solo = false;
            // A Bus enters Master; Master and Pad FX enter no chain.
            let master = if self.kind == Kind::Bus {
                self.mix.entries[0]
            } else {
                INPUT
            };
            self.mix.entries = [INPUT; DESTINATIONS];
            self.mix.entries[0] = master;
        }
        if self.kind != Kind::Bus || self.bus as usize > MAX_BUSES {
            self.bus = 0;
        }
        if self.kind != Kind::PadFx {
            self.owner = None;
            self.pad = 0;
        }
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
/// The smoothed send levels of one timbre, ramped like its level into Master.
#[derive(Clone, Copy, Debug)]
pub struct SendGains {
    gains: [f32; MAX_BUSES],
    /// Mute and Solo of the section sends, whose levels ramp in the engine.
    taps: f32,
    /// The entry levels into Master and each bus, ramping to `Mix::entries`.
    entries: [Entries; DESTINATIONS],
    /// Bit d set for each destination this block splits over its entries.
    split: u16,
}
impl Default for SendGains {
    fn default() -> Self {
        Self {
            gains: [0.; MAX_BUSES],
            taps: 0.,
            entries: [INPUT; DESTINATIONS],
            split: 0,
        }
    }
}
impl SendGains {
    /// Notes, once per block, the destinations whose entries are not all at
    /// the input or are still ramping; the others take the plain sum.
    pub fn plan(&mut self, levels: &[Entries; DESTINATIONS]) -> u16 {
        self.split = 0;
        for (d, (gains, levels)) in self.entries.iter().zip(levels).enumerate() {
            if *gains != INPUT || *levels != INPUT {
                self.split |= 1 << d;
            }
        }
        self.split
    }
    /// The destinations split in this block (`plan`).
    pub fn split(&self) -> u16 {
        self.split
    }
    /// Ramps the entries of destination `d` one frame towards `levels` and
    /// adds `frame` at each entry's gain: the input's to `input`, the
    /// others' to `entered`.
    #[inline]
    pub fn enter(
        &mut self,
        d: usize,
        levels: &Entries,
        frame: crate::Stereo,
        input: &mut crate::Stereo,
        entered: &mut Entered,
    ) {
        let gains = &mut self.entries[d];
        for (gain, &level) in gains.iter_mut().zip(levels) {
            let delta = level - *gain;
            if delta != 0.0 {
                *gain = if delta.abs() < 1e-4 {
                    level
                } else {
                    *gain + delta * 0.004
                };
            }
        }
        input[0] += frame[0] * gains[0];
        input[1] += frame[1] * gains[0];
        for (slot, &gain) in entered.iter_mut().zip(&gains[1..]) {
            slot[0] += frame[0] * gain;
            slot[1] += frame[1] * gain;
        }
    }
    /// Adds a timbre's frame of bus sends (`own`) to the bus inputs, split
    /// over the entries of the buses planned for it.
    #[inline]
    pub fn enter_buses(
        &mut self,
        levels: &[Entries; DESTINATIONS],
        own: &[crate::Stereo; MAX_BUSES],
        buses: &mut [crate::Stereo; MAX_BUSES],
        entered: &mut [Entered; DESTINATIONS],
    ) {
        for (b, (frame, bus)) in own.iter().zip(buses).enumerate() {
            if self.split & 1 << (b + 1) != 0 {
                self.enter(b + 1, &levels[b + 1], *frame, bus, &mut entered[b + 1]);
            } else {
                bus[0] += frame[0];
                bus[1] += frame[1];
            }
        }
    }
    /// The mixer send gains, ramping towards their levels.
    pub fn gains(&self) -> &[f32; MAX_BUSES] {
        &self.gains
    }
    /// Adds a timbre's output to the bus inputs; a silent send costs a compare.
    #[inline]
    pub fn send(
        &mut self,
        sends: &[f32; MAX_BUSES],
        audible: bool,
        frame: crate::Stereo,
        buses: &mut [crate::Stereo; MAX_BUSES],
    ) {
        for ((gain, &level), bus) in self.gains.iter_mut().zip(sends).zip(buses) {
            let target = if audible { level } else { 0.0 };
            if target == 0.0 && *gain == 0.0 {
                continue;
            }
            *gain = crate::dsp::fast::flush(*gain + (target - *gain) * 0.004);
            bus[0] += frame[0] * *gain;
            bus[1] += frame[1] * *gain;
        }
    }
    /// Adds what a timbre's sections sent (`Engine::bus_sends`).
    #[inline]
    pub fn send_taps(
        &mut self,
        audible: bool,
        taps: Option<&[crate::Stereo; MAX_BUSES]>,
        buses: &mut [crate::Stereo; MAX_BUSES],
    ) {
        let target = f32::from(audible);
        if self.taps != target {
            let delta = target - self.taps;
            self.taps = if delta.abs() < 1e-4 {
                target
            } else {
                self.taps + delta * 0.004
            };
        }
        let Some(taps) = taps.filter(|_| self.taps > 0.0) else {
            return;
        };
        for (bus, tap) in buses.iter_mut().zip(taps) {
            bus[0] += tap[0] * self.taps;
            bus[1] += tap[1] * self.taps;
        }
    }
}
/// Bit n set for each Bus n that a timbre can send to.
pub fn open_buses<'a>(timbres: impl IntoIterator<Item = &'a Meta>) -> u16 {
    timbres
        .into_iter()
        .filter(|m| m.kind == Kind::Bus)
        .fold(0, |open, m| open | 1 << m.bus)
}
/// The lowest Bus number (1–8) that no existing bus uses.
pub fn free_bus<'a>(timbres: impl IntoIterator<Item = &'a Meta>) -> Option<u8> {
    let used = open_buses(timbres);
    (1..=MAX_BUSES as u8).find(|b| used & (1 << b) == 0)
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Context {
    pub name: String,
    pub active: u32,
    pub revision: u64,
    pub timbres: Vec<Meta>,
}
