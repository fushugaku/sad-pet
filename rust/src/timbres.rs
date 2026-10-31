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
    Tb303,
    /// Sings a score; its voice runs through the chain like a Bus input.
    Vocal,
    /// Plays a kit of sample pads; the kit runs through the chain like a
    /// Bus input.
    Sampler,
    /// The effect chain of one Sampler pad: a bus fed by that pad's voices,
    /// whose output returns into its Sampler.
    PadFx,
    /// Built from modules in any order: sources, processors and a Mixer.
    Rack,
    /// Processes the sends of other timbres and feeds Master.
    Bus,
    /// Always present: the sum of every timbre and bus, then the output.
    Master,
    /// What the first timbre leaves when it is removed. The scene keeps its
    /// place, where DAW automation and the host's own engine stay, but it
    /// sounds nothing, takes no MIDI and does not show; the next instrument
    /// added fills it.
    Empty,
    /// Creation shortcut; its saved sound is a Rack with a Wavetable source.
    Wavetable,
}
impl Kind {
    pub fn label(self) -> &'static str {
        match self {
            Kind::Granular => "Granular",
            Kind::Drums => "Drums",
            Kind::Synth => "Synth",
            Kind::Tb303 => "303 Bass",
            Kind::Vocal => "Vocal",
            Kind::Sampler => "Sampler",
            Kind::PadFx => "Pad FX",
            Kind::Rack => "Rack",
            Kind::Bus => "Bus",
            Kind::Master => "Master",
            Kind::Empty => "Empty",
            Kind::Wavetable => "Wavetable",
        }
    }
    /// Bus, Master and Pad FX take their input from other timbres, not MIDI
    /// or a file.
    pub fn routing(self) -> bool {
        matches!(self, Kind::Bus | Kind::Master | Kind::PadFx)
    }
    /// Plays notes from MIDI and the keyboard; a Vocal timbre sings its score.
    pub fn takes_midi(self) -> bool {
        !self.routing() && !matches!(self, Kind::Vocal | Kind::Empty)
    }
    /// A new instrument waits for notes rather than playing its voice 0 with
    /// the transport (a Granular one keeps its host's choice).
    pub fn waits_for_notes(self) -> bool {
        matches!(
            self,
            Kind::Synth | Kind::Rack | Kind::Tb303 | Kind::Wavetable
        )
    }
    /// A new instrument of this kind, built as a Rack that plays as a timbre
    /// of the kind does: the modules its engine runs, in their order. None
    /// for a Rack, which starts from its own default, and for what is not an
    /// instrument.
    pub fn rack_instrument(self) -> Option<crate::preset::Preset> {
        use crate::rack::{Module, Slot};
        let modules: &[Module] = match self {
            Kind::Granular => &[
                Module::Sample,
                Module::Grains,
                Module::Filters,
                Module::PostFx,
            ],
            Kind::Drums => &[Module::Drums, Module::Filters, Module::PostFx],
            Kind::Synth => &[Module::Synth, Module::Filters, Module::PostFx],
            Kind::Sampler => &[Module::Sampler],
            Kind::Vocal => &[Module::Vocal],
            Kind::Tb303 => &[Module::Tb303],
            Kind::Wavetable => &[
                Module::Wavetable,
                Module::Filters,
                Module::Mixer,
                Module::PostFx,
            ],
            _ => return None,
        };
        let mut preset = crate::preset::Preset {
            name: self.label().into(),
            ..Default::default()
        };
        if self == Kind::Drums {
            // A Drum timbre plays its pattern through voice 0's envelope,
            // held at sustain; a Rack's drums have no envelope.
            preset.params.master_volume *= preset.params.adsr_sustain;
        }
        if matches!(self, Kind::Sampler | Kind::Vocal | Kind::Tb303) {
            // As those timbres pass their kit or voice: at unity, no effects.
            preset.params.master_volume = 1.0;
            preset.params.mix = 0.5;
            preset.color_chain.clear();
        }
        if self == Kind::Tb303 {
            preset.sequencer.enabled = false;
        }
        if self == Kind::Wavetable {
            preset.color_chain.clear();
            preset.params.filter = 1.0;
            preset.params.filter2 = 1.0;
            preset.params.filter_env_depth = 0.0;
            preset.params.filter2_env_depth = 0.0;
            preset.sequencer.enabled = false;
        }
        preset.rack = Some(Box::new(crate::rack::Settings {
            modules: modules
                .iter()
                .map(|&module| Slot {
                    module,
                    enabled: self != Kind::Wavetable || module != Module::Filters,
                    id: 0,
                })
                .collect(),
            ..Default::default()
        }));
        // Each source module's settings start from that engine's own.
        preset.sanitize();
        Some(preset)
    }
    /// Keeps its engine role when a preset loads into it.
    pub fn keeps_role(self) -> bool {
        self.routing() || matches!(self, Kind::Vocal | Kind::Sampler)
    }
    /// Pad FX timbres cannot be told from a Bus by their preset; their Meta
    /// names the Sampler they belong to.
    pub fn of(preset: &crate::preset::Preset) -> Self {
        if preset.rack.is_some() {
            Kind::Rack
        } else if preset.tb303.is_some() {
            Kind::Tb303
        } else if preset.vocal.is_some() {
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
            Kind::Tb303 | Kind::Wavetable => {
                return self.rack_instrument();
            }
            Kind::Rack => {
                preset.rack = Some(Box::default());
                preset.sanitize();
            }
            Kind::Empty => {
                // A Rack without modules: silent, and it costs next to nothing.
                preset.rack = Some(Box::new(crate::rack::Settings {
                    modules: Vec::new(),
                    ..Default::default()
                }));
                preset.color_chain.clear();
                preset.sanitize();
            }
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
    #[serde(skip_serializing_if = "is_zero")]
    pub key_low: u8,
    #[serde(skip_serializing_if = "midi_max")]
    pub key_high: u8,
    #[serde(skip_serializing_if = "velocity_min")]
    pub velocity_low: u8,
    #[serde(skip_serializing_if = "midi_max")]
    pub velocity_high: u8,
    /// Send levels to Bus 1–8; a Bus sends to other buses, Master and Pad
    /// FX do not send.
    pub sends: [f32; MAX_BUSES],
    /// Where this timbre enters Master (0) and Bus 1–8; a Bus enters no
    /// chain of its own, and Master none.
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
            key_low: 0,
            key_high: 127,
            velocity_low: 1,
            velocity_high: 127,
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
        self.key_low = self.key_low.min(127);
        self.key_high = self.key_high.clamp(self.key_low, 127);
        self.velocity_low = self.velocity_low.clamp(1, 127);
        self.velocity_high = self.velocity_high.clamp(self.velocity_low, 127);
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
    pub fn midi_changed(self, other: Self) -> bool {
        (
            self.channel,
            self.key_low,
            self.key_high,
            self.velocity_low,
            self.velocity_high,
        ) != (
            other.channel,
            other.key_low,
            other.key_high,
            other.velocity_low,
            other.velocity_high,
        )
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
        let channel_ok = self.channel == 0 || channel.is_none_or(|c| c + 1 == self.channel);
        // Note-off (including velocity-zero NoteOn) always reaches previously
        // accepted notes. CC/sustain/clock are never lost at a split boundary.
        channel_ok
            && match message {
                NoteOn { note, velocity, .. } if velocity > 0 => {
                    (self.key_low..=self.key_high).contains(&note)
                        && (self.velocity_low..=self.velocity_high).contains(&velocity)
                }
                _ => true,
            }
    }
}
fn midi_max(value: &u8) -> bool {
    *value == 127
}
fn velocity_min(value: &u8) -> bool {
    *value == 1
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
    /// For a Bus: the buses it feeds (bit b − 1 for Bus b) by its sends and
    /// its sections, as the host last routed them; the page offers no
    /// route that would close a loop.
    #[serde(skip_serializing_if = "is_zero")]
    pub feeds: u8,
    #[serde(flatten)]
    pub mix: Mix,
}
fn is_zero<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
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
            feeds: 0,
            mix: Mix::default(),
        }
    }
}
impl Meta {
    pub fn sanitize(&mut self) {
        self.mix.sanitize();
        if self.kind != Kind::Bus || self.bus as usize > MAX_BUSES {
            self.bus = 0;
        }
        if self.kind.routing() {
            self.mix.channel = 0;
            self.mix.solo = false;
        }
        if self.kind == Kind::Bus {
            // A Bus sends to and enters the other buses, never its own.
            if self.bus > 0 {
                self.mix.forget_bus(self.bus);
            }
        } else {
            self.feeds = 0;
            if self.kind.routing() {
                // Master and Pad FX send nowhere and enter no chain.
                self.mix.sends = [0.; MAX_BUSES];
                self.mix.entries = [INPUT; DESTINATIONS];
            }
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
/// For each Bus b (index b − 1), the buses it feeds: bit t − 1 for Bus t.
pub type Feeds = [u8; MAX_BUSES];

/// The buses a timbre's sections are routed to (bit b − 1 for Bus b),
/// whatever their send levels: modulation can open a send at any time.
pub fn tapped(preset: &crate::preset::Preset) -> u8 {
    // Outside advanced mode the grain groups keep their taps closed.
    let groups = &preset.grain_groups;
    let groups = groups
        .groups
        .iter()
        .filter(|_| groups.advanced)
        .map(|g| &g.output);
    crate::taps::Stage::ALL
        .iter()
        .map(|&stage| preset.taps.get(stage))
        .chain(groups)
        .chain(preset.filter_slots.iter().map(|slot| &slot.output))
        .filter(|tap| (1..=MAX_BUSES as u8).contains(&tap.bus))
        .fold(0, |buses, tap| buses | 1 << (tap.bus - 1))
}
/// The routes between buses that make no loop: each Bus feeds the buses it
/// sends to above zero and those its sections are routed to (`tapped`).
/// A route that would close a loop is left out, the later bus's first; the
/// page refuses such routes, so only automation or a loaded preset makes one.
pub fn bus_feeds<'a>(
    buses: impl IntoIterator<Item = (&'a Meta, &'a crate::preset::Preset)>,
) -> Feeds {
    let mut wanted = [0; MAX_BUSES];
    let mut open = 0;
    for (meta, preset) in buses {
        if meta.kind != Kind::Bus || meta.bus == 0 {
            continue;
        }
        let b = meta.bus as usize - 1;
        open |= 1 << b;
        wanted[b] = meta
            .mix
            .sends
            .iter()
            .enumerate()
            .filter(|(_, level)| **level > 0.)
            .fold(tapped(preset), |feeds, (t, _)| feeds | 1 << t);
    }
    let mut feeds = [0; MAX_BUSES];
    for from in 0..MAX_BUSES {
        let mut pending = wanted[from] & open;
        while pending != 0 {
            let to = pending.trailing_zeros() as usize;
            pending &= pending - 1;
            if !reaches(&feeds, to, from) {
                feeds[from] |= 1 << to;
            }
        }
    }
    feeds
}
/// Whether Bus index `from` is Bus index `to` or feeds it, directly or
/// through other buses.
fn reaches(feeds: &Feeds, from: usize, to: usize) -> bool {
    let mut seen = 1_u8 << from;
    let mut frontier = seen;
    while frontier != 0 && seen & 1 << to == 0 {
        let mut next = 0;
        while frontier != 0 {
            next |= feeds[frontier.trailing_zeros() as usize];
            frontier &= frontier - 1;
        }
        frontier = next & !seen;
        seen |= next;
    }
    seen & 1 << to != 0
}
/// Whether a route from Bus `from` to Bus `to` (1–8) would close a loop:
/// `to` is `from` or already feeds it.
pub fn closes_loop(feeds: &Feeds, from: u8, to: u8) -> bool {
    let valid = 1..=MAX_BUSES as u8;
    !valid.contains(&from)
        || !valid.contains(&to)
        || reaches(feeds, to as usize - 1, from as usize - 1)
}
/// Drops the sends of Bus `from` that would close a loop through `feeds`;
/// true when any was dropped.
pub fn refuse_loops(mix: &mut Mix, feeds: &Feeds, from: u8) -> bool {
    let mut refused = false;
    for (t, level) in mix.sends.iter_mut().enumerate() {
        if *level > 0. && closes_loop(feeds, from, t as u8 + 1) {
            *level = 0.;
            refused = true;
        }
    }
    refused
}
/// The order buses render in: waves, each holding the buses that only
/// buses of earlier waves feed, in list order. Without routes between
/// buses it is one wave in list order, as before buses fed buses.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BusOrder {
    /// Bus numbers 1–8, wave after wave.
    buses: [u8; MAX_BUSES],
    /// Where each wave ends in `buses`.
    ends: [u8; MAX_BUSES],
    len: u8,
    waves: u8,
}
impl BusOrder {
    /// `listed` holds the bus numbers in list order.
    pub fn new(listed: impl IntoIterator<Item = u8>, feeds: &Feeds) -> Self {
        let mut order = Self::default();
        let mut buses = [0; MAX_BUSES];
        let mut count = 0;
        let mut open = 0_u8;
        for bus in listed {
            let valid = (1..=MAX_BUSES as u8).contains(&bus);
            if valid && open & 1 << (bus - 1) == 0 {
                open |= 1 << (bus - 1);
                buses[count] = bus;
                count += 1;
            }
        }
        let mut done = 0_u8;
        while (order.len as usize) < count {
            let before = done;
            for &bus in &buses[..count] {
                let b = bus as usize - 1;
                if done & 1 << b != 0 {
                    continue;
                }
                // Waits while a bus not yet rendered feeds it; a loop, which
                // `bus_feeds` never leaves, breaks into list order.
                let waiting = (0..MAX_BUSES)
                    .any(|x| x != b && open & !before & 1 << x != 0 && feeds[x] & 1 << b != 0);
                if !waiting {
                    order.buses[order.len as usize] = bus;
                    order.len += 1;
                    done |= 1 << b;
                }
            }
            if done == before {
                for &bus in &buses[..count] {
                    if done & 1 << (bus - 1) == 0 {
                        order.buses[order.len as usize] = bus;
                        order.len += 1;
                        done |= 1 << (bus - 1);
                    }
                }
            }
            order.ends[order.waves as usize] = order.len;
            order.waves += 1;
        }
        order
    }
    /// Every bus, wave after wave.
    pub fn buses(&self) -> &[u8] {
        &self.buses[..self.len as usize]
    }
    /// The waves in order; the buses of one wave can render together.
    pub fn waves(&self) -> impl Iterator<Item = &[u8]> {
        let ends = &self.ends[..self.waves as usize];
        ends.iter().scan(0, |start, &end| {
            let wave = &self.buses[*start..end as usize];
            *start = end as usize;
            Some(wave)
        })
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
impl Context {
    /// Notes on every Bus which buses it feeds.
    pub fn note_feeds(&mut self, feeds: &Feeds) {
        for meta in &mut self.timbres {
            meta.feeds = match meta.kind {
                Kind::Bus if meta.bus > 0 => feeds[meta.bus as usize - 1],
                _ => 0,
            };
        }
    }
}
