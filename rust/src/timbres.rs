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
            Kind::Bus => "Bus",
            Kind::Master => "Master",
        }
    }
    /// Bus and Master take their input from other timbres, not MIDI or a file.
    pub fn routing(self) -> bool {
        matches!(self, Kind::Bus | Kind::Master)
    }
    /// Plays notes from MIDI and the keyboard; a Vocal timbre sings its score.
    pub fn takes_midi(self) -> bool {
        !self.routing() && self != Kind::Vocal
    }
    /// Keeps its engine role when a preset loads into it.
    pub fn keeps_role(self) -> bool {
        self.routing() || self == Kind::Vocal
    }
    pub fn of(preset: &crate::preset::Preset) -> Self {
        if preset.vocal.is_some() {
            Kind::Vocal
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
            Kind::Bus | Kind::Master | Kind::Vocal => {
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
            }
        }
        Some(preset)
    }
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
}
impl Default for Mix {
    fn default() -> Self {
        Self {
            gain: 1.,
            muted: false,
            solo: false,
            channel: 0,
            sends: [0.; MAX_BUSES],
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
    #[serde(flatten)]
    pub mix: Mix,
}
impl Default for Meta {
    fn default() -> Self {
        Self {
            id: 0,
            name: "Granular 1".into(),
            kind: Kind::Granular,
            bus: 0,
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
        }
        if self.kind != Kind::Bus || self.bus as usize > MAX_BUSES {
            self.bus = 0;
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
#[derive(Clone, Copy, Debug, Default)]
pub struct SendGains(pub [f32; MAX_BUSES]);
impl SendGains {
    /// Adds a timbre's output to the bus inputs; a silent send costs a compare.
    #[inline]
    pub fn send(
        &mut self,
        sends: &[f32; MAX_BUSES],
        audible: bool,
        frame: crate::Stereo,
        buses: &mut [crate::Stereo; MAX_BUSES],
    ) {
        for ((gain, &level), bus) in self.0.iter_mut().zip(sends).zip(buses) {
            let target = if audible { level } else { 0.0 };
            if target == 0.0 && *gain == 0.0 {
                continue;
            }
            *gain = crate::dsp::fast::flush(*gain + (target - *gain) * 0.004);
            bus[0] += frame[0] * *gain;
            bus[1] += frame[1] * *gain;
        }
    }
}
/// The lowest Bus number (1–8) that no existing bus uses.
pub fn free_bus<'a>(timbres: impl IntoIterator<Item = &'a Meta>) -> Option<u8> {
    let used = timbres
        .into_iter()
        .filter(|m| m.kind == Kind::Bus)
        .fold(0_u16, |used, m| used | 1 << m.bus);
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
