//! One WASM memory, independently running parts and a fixed audio mix buffer.
use super::*;
use sad_pet::{
    bus::MAX_BUSES,
    timbres::{Context, DESTINATIONS, Entered, Kind, Meta, SendGains},
};

pub(super) struct Part {
    pub meta: Meta,
    pub host: Host,
    pub gain: f32,
    pub sends: SendGains,
    /// Present while the part is a Vocal timbre.
    pub vocal: Option<Box<super::vocal::VocalPart>>,
}
impl Part {
    /// A Bus or Master starts with everything off. It keeps the host's tiny
    /// placeholder sample, so scenes do not store the file again for it.
    fn routing(rate: u32, kind: Kind, id: u32, bus: u8) -> Self {
        let mut host = Host::new(rate);
        host.engine.set_preset(kind.preset().unwrap());
        host.resize_snapshot();
        let name = match kind {
            Kind::Bus => format!("Bus {bus}"),
            _ => kind.label().into(),
        };
        Self {
            meta: Meta {
                id,
                name,
                kind,
                bus,
                ..Meta::default()
            },
            host,
            gain: 1.,
            sends: SendGains::default(),
            vocal: None,
        }
    }
}
struct Preparing {
    kind: Kind,
    /// What the page asked for: an instrument kind arrives as a Rack.
    instrument: Kind,
    stage: u8,
    host: Option<Host>,
    preset: Option<Preset>,
    live: Option<sad_pet::dsp::sample::LiveInputPreparation>,
}
pub(super) struct Registry {
    preparing: Option<Preparing>,
    pub parts: Vec<Part>,
    pub selected: usize,
    pub target: Option<usize>,
    pub revision: u64,
    pub next_id: u32,
    pub sequence: u32,
    pub snapshot_part: usize,
    pub unacked: [Option<(u32, u32, u32)>; 3],
    /// The song clock that Vocal timbres play at.
    pub clock: sad_pet::vocal::Clock,
    /// Render-worker messages waiting for the worklet to post them.
    pub vocal_jobs: Vec<Value>,
    pub vocal_ids: Vec<u32>,
    /// The MIDI clock the scene follows, and its beat for clock out.
    pub midi_clock: super::clock::MidiClock,
}
impl Registry {
    pub fn new(rate: u32) -> Self {
        let mut r = Self {
            preparing: None,
            parts: vec![Part {
                meta: Meta::default(),
                host: Host::new(rate),
                gain: 1.,
                sends: SendGains::default(),
                vocal: None,
            }],
            selected: 0,
            target: None,
            revision: 0,
            next_id: 1,
            sequence: 0,
            snapshot_part: 0,
            unacked: [None; 3],
            clock: Default::default(),
            vocal_jobs: Vec::new(),
            vocal_ids: Vec::new(),
            midi_clock: super::clock::MidiClock::new(rate),
        };
        r.ensure_master();
        r.refresh_context();
        r
    }
    /// Every scene has exactly one Master, including scenes saved before it.
    fn ensure_master(&mut self) {
        if !self.parts.iter().any(|p| p.meta.kind == Kind::Master) {
            let rate = self.parts[0].host.rate;
            let id = self.next_id;
            self.next_id = id.saturating_add(1);
            self.parts.push(Part::routing(rate, Kind::Master, id, 0));
        }
    }
    /// Bus numbers are unique and valid after loading or adopting a scene.
    fn number_buses(&mut self) {
        for i in 0..self.parts.len() {
            if self.parts[i].meta.kind != Kind::Bus {
                continue;
            }
            let bus = self.parts[i].meta.bus;
            let taken = bus == 0
                || self.parts[..i]
                    .iter()
                    .any(|p| p.meta.kind == Kind::Bus && p.meta.bus == bus);
            if taken {
                let metas: Vec<Meta> = self.parts[..i].iter().map(|p| p.meta.clone()).collect();
                self.parts[i].meta.bus = sad_pet::timbres::free_bus(&metas).unwrap_or(0);
            }
        }
    }
    pub fn index(&self) -> usize {
        self.target.unwrap_or(self.selected)
    }
    pub fn context(&self) -> Context {
        let mut context = Context {
            name: self.parts[0].host.engine.preset.name.clone(),
            active: self.parts[self.selected].meta.id,
            revision: self.revision,
            timbres: self.parts.iter().map(|p| p.meta.clone()).collect(),
        };
        context.note_feeds(&self.bus_feeds());
        context
    }
    /// Which buses each Bus feeds, loops left out.
    fn bus_feeds(&self) -> sad_pet::timbres::Feeds {
        sad_pet::timbres::bus_feeds(self.parts.iter().map(|p| (&p.meta, &p.host.engine.preset)))
    }
    pub fn refresh_context(&mut self) {
        // An emptied first timbre does not show: another one is open.
        if self.parts[self.selected].meta.kind == Kind::Empty {
            self.selected = self.shown();
        }
        let context = self.context();
        for p in &mut self.parts {
            p.host.context = context.clone();
        }
        self.sync_sampler_routes();
    }
    pub fn adopt_context(&mut self, context: Context) -> Result<(), String> {
        if context.timbres.is_empty() || context.revision < self.revision {
            return Ok(());
        }
        let rate = self.parts[0].host.rate;
        for meta in &context.timbres {
            if !self.parts.iter().any(|p| p.meta.id == meta.id) {
                let mut host = Host::new(rate);
                if !meta.kind.keeps_role() {
                    host.engine
                        .set_sample(self.parts[0].host.engine.sample.clone());
                }
                host.file_generation = self.parts[0].host.file_generation;
                self.parts.push(Part {
                    meta: meta.clone(),
                    host,
                    gain: meta.mix.gain,
                    sends: SendGains::default(),
                    vocal: None,
                });
            } else {
                self.parts
                    .iter_mut()
                    .find(|p| p.meta.id == meta.id)
                    .unwrap()
                    .meta = meta.clone();
            }
        }
        self.parts
            .retain(|p| context.timbres.iter().any(|m| m.id == p.meta.id));
        self.selected = self
            .parts
            .iter()
            .position(|p| p.meta.id == context.active)
            .ok_or("Unknown timbre")?;
        self.next_id = self.next_id.max(
            self.parts
                .iter()
                .map(|p| p.meta.id)
                .max()
                .unwrap_or(0)
                .saturating_add(1),
        );
        self.revision = context.revision;
        self.refresh_context();
        Ok(())
    }
    /// The timbre to open when the open one goes: the first instrument, else
    /// the first bus, else Master.
    fn shown(&self) -> usize {
        let kind = |i: &usize| self.parts[*i].meta.kind;
        (0..self.parts.len())
            .find(|i| !kind(i).routing() && kind(i) != Kind::Empty)
            .or_else(|| (0..self.parts.len()).find(|i| kind(i) == Kind::Bus))
            .or_else(|| (0..self.parts.len()).find(|i| kind(i) == Kind::Master))
            .unwrap_or(0)
    }
    /// The first timbre goes: its place stays, empty and silent, as the
    /// scene's anchor (its preset holds the scene's name), until the next
    /// instrument fills it. Its Pad FX timbres go with it.
    fn empty_first(&mut self) -> Result<(), String> {
        if self.parts[0].meta.kind == Kind::Empty {
            return Err("Unknown timbre".into());
        }
        let host = &mut self.parts[0].host;
        let mut preset = Kind::Empty.preset().unwrap();
        preset.name = host.engine.preset.name.clone();
        host.engine.set_preset(preset);
        host.config_revision = host.config_revision.wrapping_add(1);
        self.parts[0].meta = Meta {
            id: 0,
            kind: Kind::Empty,
            name: Kind::Empty.label().into(),
            ..Meta::default()
        };
        self.parts[0].vocal = None;
        self.parts.retain(|p| p.meta.owner != Some(0));
        self.selected = self.shown();
        Ok(())
    }
    /// A new timbre of `kind`: an instrument is a Rack of its engine's
    /// modules, or with `classic` that engine's own timbre, as scenes saved
    /// before Racks hold.
    fn begin_part(&mut self, kind: Kind, classic: bool) -> Result<(), String> {
        if self.preparing.is_some() {
            return Err("A timbre is already being prepared".into());
        }
        if matches!(kind, Kind::Master | Kind::Empty) {
            return Err("The scene always has one Master".into());
        }
        if kind == Kind::Bus
            && sad_pet::timbres::free_bus(self.parts.iter().map(|p| &p.meta)).is_none()
        {
            return Err(format!("Use at most {MAX_BUSES} buses"));
        }
        self.next_id
            .checked_add(1)
            .ok_or("Timbre identity overflow")?;
        // Every instrument is built as a Rack of its engine's modules.
        let instrument = kind;
        let kind = if !classic && kind.rack_instrument().is_some() {
            Kind::Rack
        } else {
            kind
        };
        self.preparing = Some(Preparing {
            kind,
            instrument,
            stage: 0,
            host: None,
            preset: None,
            live: None,
        });
        Ok(())
    }
    fn prepare_part(&mut self) -> Result<bool, String> {
        let mut pending = self
            .preparing
            .take()
            .ok_or("No timbre preparation is pending")?;
        let rate = self.parts[0].host.rate;
        match pending.stage {
            0 => pending.host = Some(Host::new(rate)),
            1 if pending.kind != Kind::Granular => {
                let mut preset = if pending.kind == Kind::Rack {
                    pending.instrument.rack_instrument()
                } else {
                    None
                }
                .or_else(|| pending.kind.preset())
                .unwrap();
                preset.sanitize();
                preset.prepare_effects_reusing(rate, &pending.host.as_ref().unwrap().engine.preset);
                pending.preset = Some(preset);
            }
            2 if pending.kind != Kind::Granular => {
                let preset = pending.preset.as_mut().unwrap();
                preset
                    .drum_runtime
                    .prepare_engine(preset.drums.as_deref(), rate);
                preset
                    .synth_runtime
                    .prepare_engine(preset.synth.as_deref(), rate);
            }
            3 if pending.kind != Kind::Granular && !pending.kind.keeps_role() => {
                let preset = pending.preset.as_mut().unwrap();
                // A Rack keeps a history only for a Grain engine that reads one.
                let Some(history) = preset.history() else {
                    pending.stage += 1;
                    self.preparing = Some(pending);
                    return Ok(false);
                };
                let input = pending.live.get_or_insert_with(|| {
                    sad_pet::dsp::sample::LiveInputPreparation::new(rate, !history.granular)
                });
                if !input.step() {
                    self.preparing = Some(pending);
                    return Ok(false);
                }
                let mut input = pending.live.take().unwrap().finish().unwrap();
                if preset.rack.is_some() {
                    input.sample.name = "Rack".into();
                    preset.rack_runtime.0 = Some(input);
                } else if let Some(drums) = &mut preset.drum_runtime.0 {
                    input.sample.name = "Drum machine".into();
                    drums.live = Some(input);
                } else if let Some(synth) = &mut preset.synth_runtime.0 {
                    input.sample.name = sad_pet::dsp::synth::NAME.into();
                    synth.live = Some(input);
                }
            }
            4 if pending.kind != Kind::Granular => {
                let size = rmp_serde::to_vec_named(pending.preset.as_ref().unwrap())
                    .map_or(0, |bytes| bytes.len())
                    + 131072;
                pending
                    .host
                    .as_mut()
                    .unwrap()
                    .snapshot_buffer
                    .resize(size, 0);
            }
            _ => {
                let mut host = pending.host.take().unwrap();
                let primary = &self.parts[0].host;
                if !pending.kind.keeps_role() {
                    host.engine.set_sample(primary.engine.sample.clone());
                }
                host.engine.set_midi_gate(primary.engine.midi_gate);
                host.file_generation = primary.file_generation;
                let playing = primary.engine.playing;
                // A synth waits for notes, and so does a Rack, whose drums
                // play their pattern anyway; drums run free with the
                // transport, and a Granular instrument keeps the host's way.
                let gate = (pending.instrument != Kind::Granular)
                    .then(|| pending.instrument.waits_for_notes());
                let instrument = pending.instrument;
                let label = instrument.label();
                let name = |parts: &[Part]| {
                    let count = parts
                        .iter()
                        .filter(|p| p.meta.name.starts_with(label))
                        .count()
                        + 1;
                    format!("{label} {count}")
                };
                // The emptied first timbre takes the next instrument.
                if self.parts[0].meta.kind == Kind::Empty && pending.kind != Kind::Bus {
                    let name = name(&self.parts);
                    let first = &mut self.parts[0];
                    if let Some(mut preset) = pending.preset.take() {
                        // Its preset keeps holding the scene's name.
                        preset.name = first.host.engine.preset.name.clone();
                        first.host.engine.exchange_preset(&mut preset);
                    }
                    if let Some(gate) = gate {
                        first.host.engine.set_midi_gate(gate);
                    }
                    first.host.engine.set_playing(playing);
                    first.host.resize_snapshot();
                    first.host.config_revision = first.host.config_revision.wrapping_add(1);
                    first.meta = Meta {
                        id: 0,
                        kind: pending.kind,
                        name,
                        ..Meta::default()
                    };
                    first.gain = 1.;
                    first.sends = SendGains::default();
                    self.selected = 0;
                    self.target = None;
                    self.revision = self.revision.wrapping_add(1);
                    self.refresh_context();
                    return Ok(true);
                }
                if let Some(mut preset) = pending.preset.take() {
                    host.engine.exchange_preset(&mut preset);
                }
                if let Some(gate) = gate {
                    host.engine.set_midi_gate(gate);
                }
                host.engine.set_playing(playing);
                let kind = pending.kind;
                let bus = if kind == Kind::Bus {
                    sad_pet::timbres::free_bus(self.parts.iter().map(|p| &p.meta))
                        .ok_or("No free bus")?
                } else {
                    0
                };
                let id = self.next_id;
                self.next_id = id.checked_add(1).ok_or("Timbre identity overflow")?;
                // Master stays the last tab; new timbres go before it.
                let at = self
                    .parts
                    .iter()
                    .position(|p| p.meta.kind == Kind::Master)
                    .unwrap_or(self.parts.len());
                self.parts.insert(
                    at,
                    Part {
                        meta: Meta {
                            id,
                            kind,
                            bus,
                            name: if kind == Kind::Bus {
                                format!("Bus {bus}")
                            } else {
                                name(&self.parts)
                            },
                            ..Meta::default()
                        },
                        host,
                        gain: 1.,
                        sends: SendGains::default(),
                        vocal: None,
                    },
                );
                self.selected = at;
                self.target = None;
                self.revision = self.revision.wrapping_add(1);
                self.refresh_context();
                return Ok(true);
            }
        }
        pending.stage += 1;
        self.preparing = Some(pending);
        Ok(false)
    }
    pub fn command(&mut self, request: Value) -> Result<Value, String> {
        let path = request["path"].as_str().ok_or("Missing command")?;
        let data = &request["data"];
        if path.starts_with("/api/timbres") {
            match path {
                "/api/timbres" => {}
                "/api/timbres/context" => {
                    self.adopt_context(
                        serde_json::from_value(data.clone()).map_err(|e| e.to_string())?,
                    )?;
                    return Ok(json!(self.context()));
                }
                "/api/timbres/add" => {
                    let kind =
                        serde_json::from_value(data["kind"].clone()).map_err(|e| e.to_string())?;
                    self.begin_part(kind, data["classic"].as_bool().unwrap_or(false))?;
                    while !self.prepare_part()? {}
                    self.sync_vocal();
                    return Ok(json!(self.context()));
                }
                "/api/timbres/reset" => {
                    self.parts.retain(|p| p.meta.id == 0);
                    // An emptied first timbre comes back as the default one.
                    if self.parts[0].meta.kind == Kind::Empty {
                        let host = &mut self.parts[0].host;
                        let name = host.engine.preset.name.clone();
                        host.engine.set_preset(Preset {
                            name,
                            ..Preset::default()
                        });
                    }
                    self.selected = 0;
                    self.parts[0].meta = Meta::default();
                    self.parts[0].gain = 1.;
                    self.parts[0].sends = SendGains::default();
                    self.ensure_master();
                }
                "/api/timbres/select" => {
                    let id = data["id"].as_u64().ok_or("Choose a timbre")? as u32;
                    self.selected = self
                        .parts
                        .iter()
                        .position(|p| p.meta.id == id && p.meta.kind != Kind::Empty)
                        .ok_or("Unknown timbre")?;
                    self.parts[self.selected].host.sent_revision = None;
                }
                "/api/timbres/update" => {
                    let id = data["id"].as_u64().ok_or("Choose a timbre")? as u32;
                    // A bus refuses a send that would feed back into itself.
                    let feeds = self.bus_feeds();
                    let part = self
                        .parts
                        .iter()
                        .find(|p| p.meta.id == id)
                        .ok_or("Unknown timbre")?;
                    let mut value = json!(part.meta);
                    for key in [
                        "name", "gain", "muted", "solo", "channel", "sends", "entries",
                    ] {
                        if let Some(v) = data.get(key) {
                            value[key] = v.clone();
                        }
                    }
                    let mut meta: Meta =
                        serde_json::from_value(value).map_err(|e| e.to_string())?;
                    meta.sanitize();
                    if meta.kind == Kind::Bus {
                        sad_pet::timbres::refuse_loops(&mut meta.mix, &feeds, meta.bus);
                    }
                    let part = self
                        .parts
                        .iter_mut()
                        .find(|p| p.meta.id == id)
                        .ok_or("Unknown timbre")?;
                    if meta.mix.channel != part.meta.mix.channel
                        && part.host.engine.preset.drums.is_none()
                    {
                        part.host.engine.midi(Message::Reset);
                    }
                    part.meta = meta;
                }
                "/api/timbres/remove" => {
                    let id = data["id"].as_u64().ok_or("Choose a timbre")? as u32;
                    let removed = self.parts.iter().find(|p| p.meta.id == id);
                    if removed.is_some_and(|p| p.meta.kind == Kind::Master) {
                        return Err("The scene always has one Master".into());
                    }
                    // Nothing keeps sending to a removed bus or its number.
                    if let Some(bus) = removed
                        .filter(|p| p.meta.kind == Kind::Bus)
                        .map(|p| p.meta.bus)
                    {
                        for part in &mut self.parts {
                            part.meta.mix.forget_bus(bus);
                        }
                    }
                    if id == 0 {
                        self.empty_first()?;
                    } else {
                        let active = self.parts[self.selected].meta.id;
                        // A Sampler's Pad FX timbres go with it.
                        self.parts
                            .retain(|p| p.meta.id != id && p.meta.owner != Some(id));
                        self.selected = self
                            .parts
                            .iter()
                            .position(|p| p.meta.id == active)
                            .unwrap_or_else(|| self.shown());
                    }
                }
                _ => return Err("Unknown timbre command".into()),
            }
            self.target = None;
            self.revision = self.revision.wrapping_add(1);
            self.refresh_context();
            self.sync_vocal();
            return Ok(json!(self.context()));
        }
        if path == "/api/scene/name" {
            let name = data["name"].as_str().ok_or("Enter a preset name")?.trim();
            if name.is_empty() || name.chars().count() > 80 || name.chars().any(char::is_control) {
                return Err("Use a preset name of up to 80 characters".into());
            }
            self.parts[0].host.engine.preset.name = name.into();
            self.parts[0].host.config_revision = self.parts[0].host.config_revision.wrapping_add(1);
            self.refresh_context();
            return Ok(json!({"ok":true}));
        }
        if path == "/api/clock" {
            return self.clock_command(data);
        }
        if path == "/api/midi-bytes" && !data.is_null() {
            // Each parser keeps running status across packets; filtering follows decoding.
            for part in &mut self.parts {
                if !part.meta.kind.takes_midi() {
                    continue;
                }
                let before = part.host.engine.preset.params;
                let mut program = false;
                for byte in data.as_array().ok_or("Invalid MIDI packet")? {
                    let byte = byte
                        .as_u64()
                        .filter(|v| *v <= 255)
                        .ok_or("Invalid MIDI byte")? as u8;
                    if let Some(message) = part.host.midi.feed(byte)
                        && part.meta.mix.accepts(message)
                    {
                        program |= matches!(message, Message::Program { .. });
                        part.host.engine.midi(message);
                    }
                }
                if before != part.host.engine.preset.params || program {
                    part.host.config_revision = part.host.config_revision.wrapping_add(1);
                    part.host.resize_snapshot();
                    if program {
                        part.host.parameter_edits.clear();
                        part.host.sequence_edits.clear();
                        part.host.drum_edits.clear();
                    }
                }
            }
            return Ok(json!({"ok":true}));
        }
        if path.starts_with("/api/vocal/") {
            return self.vocal_command(path, data);
        }
        if path.starts_with("/api/sampler/") {
            return self.sampler_command(path, data);
        }
        if path == "/api/transport" && !data.is_null() {
            if let Some(playing) = data["playing"].as_bool() {
                let lead_in = self.vocal_lead_in();
                self.clock.play(playing, lead_in);
            }
            let mut shared = serde_json::Map::new();
            for key in ["playing", "midi_gate", "clear_hold"] {
                if let Some(v) = data.get(key) {
                    shared.insert(key.into(), v.clone());
                }
            }
            if !shared.is_empty() {
                for part in &mut self.parts {
                    part.host.command(json!({"path":path,"data":shared}))?;
                }
            }
        }
        let index = self.index();
        let state = path == "/api/state";
        let preset_changed = path == "/api/preset" && !data.is_null();
        let mut request = request;
        if preset_changed && let Some(data) = request["data"].as_object_mut() {
            // A Bus or Master keeps its role when a preset loads into it;
            // any other timbre ignores bus settings.
            let engine = &self.parts[index].host.engine;
            let kind = self.parts[index].meta.kind;
            if let Some(bus) = engine.preset.bus.as_deref().filter(|_| kind.keeps_role()) {
                let mut settings = data.get("bus").cloned().unwrap_or(Value::Null);
                if settings.is_null() {
                    settings = json!(bus);
                }
                settings["master"] = json!(bus.master);
                data.insert("bus".into(), settings);
                data.remove("drums");
                data.remove("synth");
            } else {
                data.remove("bus");
            }
            // A Sampler keeps its kit and a Vocal timbre its score; a Rack
            // preset brings those of its modules, or keeps the timbre's own
            // through an edit of its modules. Nothing else gains one.
            let rack = data.get("rack").is_some_and(|r| !r.is_null());
            match engine
                .preset
                .sampler
                .as_deref()
                .filter(|_| kind == Kind::Sampler || rack)
            {
                Some(kit) => {
                    if data.get("sampler").is_none_or(Value::is_null) {
                        data.insert("sampler".into(), json!(kit));
                    }
                }
                None if !rack => {
                    data.remove("sampler");
                }
                None => {}
            }
            match engine
                .preset
                .vocal
                .as_deref()
                .filter(|_| kind == Kind::Vocal || rack)
            {
                Some(vocal) => {
                    if data.get("vocal").is_none_or(Value::is_null) {
                        data.insert("vocal".into(), json!(vocal));
                    }
                }
                None if !rack => {
                    data.remove("vocal");
                }
                None => {}
            }
        }
        let mut result = self.parts[index].host.command(request)?;
        if preset_changed {
            // A loaded preset can change the part's engine type; a Pad FX
            // timbre stays one.
            let part = &mut self.parts[index];
            if part.meta.kind != Kind::PadFx {
                part.meta.kind = Kind::of(&part.host.engine.preset);
            }
            self.prune_chains();
            self.refresh_context();
        }
        if state {
            result["timbres"] = json!(self.context());
        }
        self.sync_vocal();
        Ok(result)
    }
    /// Timbres sum into Master and send to buses; buses then feed the buses
    /// after them and Master, whose engine renders the output. With no buses
    /// and Master at its defaults, the output is the plain sum of the
    /// timbres as before.
    pub fn process(&mut self, count: usize) -> *const Stereo {
        let solo = self
            .parts
            .iter()
            .any(|p| p.meta.mix.solo && !p.meta.kind.routing());
        let open = sad_pet::timbres::open_buses(self.parts.iter().map(|p| &p.meta));
        let buses = open != 0;
        // A bus renders after every bus that feeds it, so a send never
        // reaches a bus that already rendered the frame: no loop can form.
        let feeds = self.bus_feeds();
        let order = sad_pet::timbres::BusOrder::new(
            self.parts
                .iter()
                .filter(|p| p.meta.kind == Kind::Bus)
                .map(|p| p.meta.bus),
            &feeds,
        );
        let mut at = [0; MAX_BUSES];
        for (i, part) in self.parts.iter_mut().enumerate() {
            let open = match part.meta.kind {
                Kind::Bus if part.meta.bus > 0 => {
                    at[part.meta.bus as usize - 1] = i;
                    u16::from(feeds[part.meta.bus as usize - 1]) << 1
                }
                kind if kind.routing() => 0,
                _ => open,
            };
            part.host.engine.set_open_buses(open);
        }
        let master = self.parts.iter().position(|p| p.meta.kind == Kind::Master);
        // Destinations (0 = Master, b = Bus b) that some timbre enters after
        // their input; the others take the plain sum as before.
        let split = self
            .parts
            .iter_mut()
            .fold(0, |split, p| split | p.sends.plan(&p.meta.mix.entries));
        let gain = |part: &mut Part, audible: bool| {
            let target = if audible { part.meta.mix.gain } else { 0. };
            part.gain = sad_pet::dsp::fast::flush(part.gain + (target - part.gain) * 0.004);
            part.gain
        };
        let count = count.min(BLOCK);
        self.clock_block(count);
        // A pad's chain runs on the pad's voices of the previous frame and
        // returns into its Sampler; a bounced pad needs no chain.
        let links: Vec<(usize, usize, u32)> = self
            .chain_links()
            .into_iter()
            .filter(|&(o, _, pad)| {
                self.parts[o]
                    .host
                    .engine
                    .preset
                    .sampler
                    .as_ref()
                    .and_then(|s| s.pad(pad))
                    .is_some_and(|p| p.bounce.is_none())
            })
            .collect();
        for part in &mut self.parts {
            if let Some(vocal) = &mut part.vocal {
                let out = &mut vocal.buffer[..count];
                if self.clock.playing {
                    vocal.song.fill(self.clock.position, out);
                } else {
                    out.fill(0.0);
                }
            }
        }
        self.clock.advance(count);
        let mut entered: [Entered; DESTINATIONS] = Default::default();
        for i in 0..count {
            let input = self.parts[0].host.audio_input[i];
            let mut frame = [0.; 2];
            let mut sends = [[0.; 2]; MAX_BUSES];
            if split != 0 {
                entered = Default::default();
            }
            for p in 0..self.parts.len() {
                if self.parts[p].meta.kind.routing() {
                    continue;
                }
                let mut returned = [0.; 2];
                let kit = matches!(self.parts[p].meta.kind, Kind::Sampler | Kind::Rack);
                if kit {
                    for &(_, c, pad) in links.iter().filter(|l| l.0 == p) {
                        let send = self.parts[p].host.engine.sampler_send(pad);
                        let chain = &mut self.parts[c];
                        let level = gain(chain, !chain.meta.mix.muted);
                        let out = chain.host.engine.process_input_frame(send);
                        returned[0] += out[0] * level;
                        returned[1] += out[1] * level;
                    }
                }
                let part = &mut self.parts[p];
                let audible = !part.meta.mix.muted && (!solo || part.meta.mix.solo);
                let level = gain(part, audible);
                let source = match &part.vocal {
                    Some(vocal) => [vocal.buffer[i]; 2],
                    None if part.meta.kind == Kind::Sampler => returned,
                    None => input,
                };
                if kit && part.meta.kind == Kind::Rack {
                    // A Rack's input is its voice; the pads return beside it.
                    part.host.engine.return_pads(returned);
                }
                let out = part.host.engine.process_input_frame(source);
                let Part {
                    meta,
                    host,
                    sends: gains,
                    ..
                } = part;
                let mix = &meta.mix;
                if gains.split() & 1 == 0 {
                    for c in 0..2 {
                        frame[c] += out[c] * level;
                    }
                } else {
                    let out = out.map(|v| v * level);
                    gains.enter(0, &mix.entries[0], out, &mut frame, &mut entered[0]);
                }
                if buses {
                    if gains.split() >> 1 == 0 {
                        gains.send(&mix.sends, audible, out, &mut sends);
                        gains.send_taps(audible, host.engine.bus_sends(), &mut sends);
                    } else {
                        let mut own = [[0.; 2]; MAX_BUSES];
                        gains.send(&mix.sends, audible, out, &mut own);
                        gains.send_taps(audible, host.engine.bus_sends(), &mut own);
                        gains.enter_buses(&mix.entries, &own, &mut sends, &mut entered);
                    }
                }
            }
            if buses {
                for &bus in order.buses() {
                    let bus = bus as usize;
                    let part = &mut self.parts[at[bus - 1]];
                    let audible = !part.meta.mix.muted;
                    let level = gain(part, audible);
                    if split & 1 << bus != 0 {
                        part.host.engine.enter(entered[bus]);
                    }
                    let out = part.host.engine.process_input_frame(sends[bus - 1]);
                    let Part {
                        meta,
                        host,
                        sends: gains,
                        ..
                    } = part;
                    let mix = &meta.mix;
                    if gains.split() & 1 == 0 {
                        for c in 0..2 {
                            frame[c] += out[c] * level;
                        }
                    } else {
                        let out = out.map(|v| v * level);
                        gains.enter(0, &mix.entries[0], out, &mut frame, &mut entered[0]);
                    }
                    // Into the buses after it; `order` put them later.
                    if gains.split() >> 1 == 0 {
                        gains.send(&mix.sends, audible, out, &mut sends);
                        gains.send_taps(audible, host.engine.bus_sends(), &mut sends);
                    } else {
                        let mut own = [[0.; 2]; MAX_BUSES];
                        gains.send(&mix.sends, audible, out, &mut own);
                        gains.send_taps(audible, host.engine.bus_sends(), &mut own);
                        gains.enter_buses(&mix.entries, &own, &mut sends, &mut entered);
                    }
                }
            }
            if let Some(m) = master {
                let part = &mut self.parts[m];
                let level = gain(part, !part.meta.mix.muted);
                if split & 1 != 0 {
                    part.host.engine.enter(entered[0]);
                }
                frame = part
                    .host
                    .engine
                    .process_input_frame(frame)
                    .map(|v| v * level);
            }
            self.parts[0].host.frames[i] = frame;
        }
        self.midi_clock.advance(count);
        self.parts[0].host.frames.as_ptr()
    }
}

#[derive(Serialize, Deserialize)]
struct PartState {
    generation: u32,
    processed: Option<sad_pet::dsp::source_fx::Recipe>,
    meta: Meta,
    preset: Preset,
    region: sad_pet::dsp::Region,
    fades: sad_pet::dsp::Fades,
    playing: bool,
    midi_gate: bool,
    frozen: bool,
    bypass: bool,
    live: bool,
    effects_only: bool,
    /// A Sampler's pool ids, whose samples follow its source in this order.
    #[serde(default)]
    pool: Vec<String>,
}
#[derive(Serialize, Deserialize)]
struct Header {
    context: Context,
    parts: Vec<PartState>,
}
/// Reads one length-prefixed packed sample from the front of `tail`.
fn take_sample(tail: &mut &[u8], missing: &str) -> Result<SampleBuffer, String> {
    let length = u32::from_le_bytes(tail.get(..4).ok_or(missing)?.try_into().unwrap()) as usize;
    let sample = super::unpack(tail.get(4..4 + length).ok_or(missing)?)?;
    *tail = &tail[4 + length..];
    Ok(sample)
}
fn put_sample(bytes: &mut Vec<u8>, sample: &SampleBuffer) {
    let sample = super::pack(sample);
    bytes.extend_from_slice(&(sample.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&sample);
}
/// `magic`, the header's length and the header.
fn framed(magic: &[u8; 8], header: &impl Serialize) -> Result<Vec<u8>, String> {
    let header = serde_json::to_vec(header).map_err(|e| e.to_string())?;
    let mut bytes = magic.to_vec();
    bytes.extend_from_slice(&(header.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&header);
    Ok(bytes)
}
/// The header after `magic` and the bytes that follow it.
fn unframed<'a, T: serde::de::DeserializeOwned>(
    bytes: &'a [u8],
    magic: &[u8; 8],
) -> Result<(T, &'a [u8]), String> {
    if bytes.len() < 12 || &bytes[..8] != magic {
        return Err("Invalid scene".into());
    }
    let size = u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize;
    let header = serde_json::from_slice(bytes.get(12..12 + size).ok_or("Incomplete scene")?)
        .map_err(|e| e.to_string())?;
    Ok((header, &bytes[12 + size..]))
}
/// One part packed alone, for an instrument preset loading into a timbre.
const PART: &[u8; 8] = b"SADPRT01";
impl Part {
    /// The ready Source FX cache its file plays through. One left over from
    /// before a live input or a Rack took over is not part of the sound.
    fn source_cache(&self) -> Option<&Arc<sad_pet::dsp::source_fx::ProcessedSource>> {
        let e = &self.host.engine;
        e.processed_source
            .as_ref()
            .filter(|_| e.reads_source_cache() && !e.source_effects_pending())
    }
    fn packed_state(&self) -> PartState {
        let e = &self.host.engine;
        PartState {
            generation: self.host.file_generation,
            processed: self
                .source_cache()
                .map(|_| sad_pet::dsp::source_fx::Recipe::from_engine(e)),
            meta: self.meta.clone(),
            preset: e.preset.clone(),
            region: e.region,
            fades: e.fades,
            playing: e.playing,
            midi_gate: e.midi_gate,
            frozen: e.frozen,
            bypass: e.bypass,
            live: e.live_input.is_some(),
            effects_only: e.live_input.as_ref().is_some_and(|i| i.effects_only),
            pool: e
                .sampler_pool()
                .map_or_else(Vec::new, |pool| pool.keys().cloned().collect()),
        }
    }
    /// Its sample, Source FX cache and kit, in the order `unpack` reads them.
    fn pack_audio(&self, bytes: &mut Vec<u8>) {
        let e = &self.host.engine;
        put_sample(bytes, &e.sample);
        if let Some(cache) = self.source_cache() {
            put_sample(bytes, &cache.sample);
        }
        for sample in e.sampler_pool().into_iter().flat_map(|p| p.values()) {
            put_sample(bytes, sample);
        }
    }
    fn unpack(state: PartState, tail: &mut &[u8], rate: u32) -> Result<Self, String> {
        let sample = take_sample(tail, "Incomplete scene audio")?;
        let mut host = Host::new(rate);
        host.engine.set_sample(Arc::new(sample));
        host.engine.set_preset(state.preset);
        if host.engine.preset.history().is_none() {
            host.engine.set_live_input(state.live, state.effects_only);
        }
        if host.engine.live_input.is_none() {
            host.engine.set_region(state.region)?;
            host.engine.set_fades(state.fades)?;
        }
        host.engine.set_midi_gate(state.midi_gate);
        host.engine.set_playing(state.playing);
        host.engine.set_frozen(state.frozen);
        host.engine.bypass = state.bypass;
        host.file_generation = state.generation;
        if let Some(recipe) = state.processed {
            let sample = take_sample(tail, "Incomplete Source FX")?;
            let range = recipe.region.bounds(host.engine.sample.len())?;
            if sample.len() != range.len() || host.engine.live_input.is_some() {
                return Err("Invalid Source FX cache".into());
            }
            let mut cache = Some(Arc::new(sad_pet::dsp::source_fx::ProcessedSource {
                original: host.engine.sample.clone(),
                sample,
                range,
                slot: recipe.slot.ok_or("Missing Source FX")?,
                bpm: recipe.bpm,
                root: recipe.root,
            }));
            host.engine.exchange_processed_source(&mut cache);
        }
        for id in state.pool {
            let sample = take_sample(tail, "Incomplete kit")?;
            if let Some(pool) = host.engine.sampler_pool_mut() {
                pool.insert(id, Arc::new(sample));
            }
        }
        host.engine.refresh_sampler();
        host.reset_source_buffers();
        host.resize_snapshot();
        Ok(Part {
            meta: state.meta,
            host,
            gain: 1.,
            sends: SendGains::default(),
            vocal: None,
        })
    }
}
impl Registry {
    pub fn pack(&self) -> Result<Vec<u8>, String> {
        let header = Header {
            context: self.context(),
            parts: self.parts.iter().map(Part::packed_state).collect(),
        };
        let mut bytes = framed(b"SADSCN01", &header)?;
        for part in &self.parts {
            part.pack_audio(&mut bytes);
        }
        Ok(bytes)
    }
    pub fn unpack(&mut self, bytes: &[u8], advance: bool) -> Result<(), String> {
        let (header, mut tail): (Header, _) = unframed(bytes, b"SADSCN01")?;
        if header.parts.is_empty() {
            return Err("The scene has no timbres".into());
        }
        let rate = self.parts[0].host.rate;
        let mut parts = Vec::new();
        let mut ids = std::collections::BTreeSet::new();
        for state in header.parts {
            if !ids.insert(state.meta.id) {
                return Err("Duplicate timbre identity".into());
            }
            parts.push(Part::unpack(state, &mut tail, rate)?);
        }
        if !tail.is_empty() || parts[0].meta.id != 0 {
            return Err("Invalid scene layout".into());
        }
        self.next_id = self.next_id.max(
            parts
                .iter()
                .map(|p| p.meta.id)
                .max()
                .unwrap_or(0)
                .saturating_add(1),
        );
        self.parts = parts;
        self.selected = self
            .parts
            .iter()
            .position(|p| p.meta.id == header.context.active)
            .unwrap_or(0);
        self.target = None;
        self.revision = if advance {
            self.revision.max(header.context.revision).wrapping_add(1)
        } else {
            header.context.revision
        };
        self.unacked.fill(None);
        self.number_buses();
        self.ensure_master();
        self.prune_chains();
        self.refresh_context();
        self.sync_vocal();
        Ok(())
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sad_timbre_target(id: u32) -> u32 {
    registry(|r| {
        let previous = r.parts[r.index()].meta.id;
        if let Some(index) = r.parts.iter().position(|p| p.meta.id == id) {
            r.target = Some(index);
            previous
        } else {
            u32::MAX
        }
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_scene_pack() -> u32 {
    registry(|r| {
        let result = r.pack();
        let index = r.index();
        r.parts[index].host.finish(result)
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_scene_load() -> u32 {
    registry(|r| {
        let index = r.index();
        let bytes = std::mem::take(&mut r.parts[index].host.input);
        let result = r
            .unpack(&bytes, true)
            .and_then(|_| serde_json::to_vec(&json!({"ok":true})).map_err(|e| e.to_string()));
        let index = r.index();
        r.parts[index].host.finish(result)
    })
}

impl Part {
    /// One timbre's saved state, as scenes and instrument presets store it.
    fn document(&self) -> Result<Value, String> {
        use base64::Engine as _;
        let e = &self.host.engine;
        let kit: Vec<Value> = e
            .sampler_pool()
            .into_iter()
            .flatten()
            .map(|(id, s)| {
                Ok(json!({"id":id,"name":s.name,"wav":base64::engine::general_purpose::STANDARD.encode(wav::bytes(s.sample_rate,s.frames())?)}))
            })
            .collect::<Result<_, String>>()?;
        Ok(
            json!({"version":1,"meta":self.meta,"kit_samples":kit,"preset":e.preset,"sample_name":e.sample.name,"sample_id":e.sample.library_id,"sample_wav":base64::engine::general_purpose::STANDARD.encode(wav::bytes(e.sample.sample_rate,e.sample.frames())?),"source":if e.preset.history().is_some() || e.live_input.is_none() {"file"} else if e.live_input.as_ref().is_some_and(|i|i.effects_only) {"effects"} else {"grains"},"region":e.region,"fades":e.fades,"sync":true,"frozen":false,"bypass":e.bypass,"midi_gate":e.midi_gate,"playing":false,"timbres":[],"active_timbre":0}),
        )
    }
    /// A part from one timbre's saved state, as `meta` (its engine type
    /// follows the preset). It starts stopped.
    fn from_document(state: &Value, mut meta: Meta, rate: u32) -> Result<Self, String> {
        use base64::Engine as _;
        if state["version"] != 1 || state["timbres"].as_array().is_some_and(|a| !a.is_empty()) {
            return Err("Invalid timbre state".into());
        }
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(state["sample_wav"].as_str().ok_or("Missing saved audio")?)
            .map_err(|e| e.to_string())?;
        if bytes.len() > 128 * 1024 * 1024 {
            return Err("Saved sample exceeds 128 MB".into());
        }
        let mut sample = wav::decode(
            Cursor::new(bytes),
            state["sample_name"].as_str().unwrap_or("Sample.wav").into(),
        )?;
        sample.library_id = state["sample_id"].as_str().map(str::to_string);
        let preset: Preset =
            serde_json::from_value(state["preset"].clone()).map_err(|e| e.to_string())?;
        if (meta.kind != Kind::PadFx || meta.owner.is_none()) && meta.kind != Kind::Empty {
            meta.kind = Kind::of(&preset);
        }
        meta.sanitize();
        let mut host = Host::new(rate);
        host.engine = Box::new(Engine::new(rate, Arc::new(sample), preset));
        for item in state["kit_samples"].as_array().into_iter().flatten() {
            let id = sad_pet::sampler::sample_id(item["id"].as_str().unwrap_or(""));
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(item["wav"].as_str().ok_or("Missing kit audio")?)
                .map_err(|e| e.to_string())?;
            let mut sample = wav::decode(
                Cursor::new(bytes),
                item["name"].as_str().unwrap_or("Sample").into(),
            )?;
            sample.library_id = Some(id.clone());
            if let Some(pool) = host.engine.sampler_pool_mut() {
                pool.insert(id, Arc::new(sample));
            }
        }
        host.engine.refresh_sampler();
        if host.engine.preset.history().is_none() {
            host.engine.set_region(
                serde_json::from_value(state["region"].clone()).map_err(|e| e.to_string())?,
            )?;
            host.engine.set_fades(
                serde_json::from_value(state.get("fades").cloned().unwrap_or_else(|| json!({})))
                    .map_err(|e| e.to_string())?,
            )?;
            host.engine.prepare_source_effects()?;
        }
        host.engine
            .set_midi_gate(state["midi_gate"].as_bool().unwrap_or(true));
        host.engine.set_playing(false);
        host.engine.bypass = state["bypass"] == true;
        host.reset_source_buffers();
        host.resize_snapshot();
        Ok(Part {
            meta,
            host,
            gain: 1.,
            sends: SendGains::default(),
            vocal: None,
        })
    }
}
impl Registry {
    fn document(&self) -> Result<Value, String> {
        let mut states = self
            .parts
            .iter()
            .map(Part::document)
            .collect::<Result<Vec<_>, _>>()?;
        let mut root = states.remove(0);
        root["timbres"] = json!(states);
        root["active_timbre"] = json!(self.parts[self.selected].meta.id);
        Ok(root)
    }
    fn prepare_document(&mut self, mut value: Value) -> Result<(), String> {
        if value["version"] != 1 {
            return Err("Unsupported scene version".into());
        }
        let others = value["timbres"].as_array().cloned().unwrap_or_default();
        let active = value["active_timbre"].as_u64().unwrap_or(0) as u32;
        value["timbres"] = json!([]);
        let mut states = vec![value];
        states.extend(others);
        let rate = self.parts[0].host.rate;
        let mut parts = Vec::new();
        let mut ids = std::collections::BTreeSet::new();
        for (i, state) in states.into_iter().enumerate() {
            let mut meta: Meta =
                serde_json::from_value(state.get("meta").cloned().unwrap_or_else(|| {
                    json!(Meta {
                        id: i as u32,
                        ..Meta::default()
                    })
                }))
                .map_err(|e| e.to_string())?;
            if (i == 0 && meta.id != 0) || !ids.insert(meta.id) {
                return Err("Invalid timbre identities".into());
            }
            meta.sanitize();
            parts.push(Part::from_document(&state, meta, rate)?);
        }
        self.next_id = self.next_id.max(
            parts
                .iter()
                .map(|p| p.meta.id)
                .max()
                .unwrap_or(0)
                .saturating_add(1),
        );
        self.parts = parts;
        self.selected = self
            .parts
            .iter()
            .position(|p| p.meta.id == active)
            .unwrap_or(0);
        self.target = None;
        self.revision = self.revision.wrapping_add(1);
        self.number_buses();
        self.ensure_master();
        self.prune_chains();
        self.refresh_context();
        Ok(())
    }
}
/// Instrument presets: one timbre's sound in place of another's.
impl Registry {
    /// Builds timbre `id`'s new sound from one timbre's saved state (an
    /// instrument preset), named `name`, in this copy of the scene and
    /// returns it packed for the audio thread's `load_instrument`.
    pub fn prepare_instrument(&mut self, data: &Value) -> Result<Vec<u8>, String> {
        let id = data["id"].as_u64().ok_or("Choose a timbre")? as u32;
        let index = self
            .parts
            .iter()
            .position(|p| p.meta.id == id)
            .ok_or("Unknown timbre")?;
        if self.parts[index].meta.kind.routing() {
            return Err("Choose an instrument timbre".into());
        }
        let rate = self.parts[index].host.rate;
        let mut part = Part::from_document(&data["state"], self.parts[index].meta.clone(), rate)?;
        if part.meta.kind.routing() {
            return Err("Use an instrument preset".into());
        }
        part.meta.name = data["name"].as_str().unwrap_or_default().into();
        part.meta.sanitize();
        part.host.file_generation = data["generation"]
            .as_u64()
            .map_or(part.host.file_generation, |g| g as u32);
        let mut bytes = framed(PART, &part.packed_state())?;
        part.pack_audio(&mut bytes);
        // Source FX for this timbre now read its new sample here.
        self.parts[index] = part;
        Ok(bytes)
    }
    /// Swaps in a part from `prepare_instrument`. The timbre keeps its place, mix
    /// and transport, and the scene's other parts play on untouched.
    pub fn load_instrument(&mut self, bytes: &[u8]) -> Result<(), String> {
        let (state, mut tail): (PartState, _) = unframed(bytes, PART)?;
        let index = self
            .parts
            .iter()
            .position(|p| p.meta.id == state.meta.id)
            .ok_or("The timbre was removed")?;
        if self.parts[index].meta.kind.routing() {
            return Err("Choose an instrument timbre".into());
        }
        let rate = self.parts[index].host.rate;
        let mut part = Part::unpack(state, &mut tail, rate)?;
        if !tail.is_empty() {
            return Err("Invalid timbre layout".into());
        }
        let old = &mut self.parts[index];
        let (kind, name) = (part.meta.kind, std::mem::take(&mut part.meta.name));
        part.meta = old.meta.clone();
        part.meta.kind = kind;
        part.meta.name = name;
        part.meta.sanitize();
        // The scene's own timbre carries the scene's name.
        part.host.engine.preset.name = if part.meta.id == 0 {
            old.host.engine.preset.name.clone()
        } else {
            part.meta.name.clone()
        };
        part.host.engine.set_playing(old.host.engine.playing);
        part.host.engine.bypass = old.host.engine.bypass;
        part.gain = old.gain;
        part.sends = std::mem::take(&mut old.sends);
        self.parts[index] = part;
        self.target = None;
        self.revision = self.revision.wrapping_add(1);
        self.prune_chains();
        self.refresh_context();
        self.sync_vocal();
        Ok(())
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_part_document() -> u32 {
    registry(|r| {
        let i = r.index();
        let result = r.parts[r.selected]
            .document()
            .and_then(|v| serde_json::to_vec(&v).map_err(|e| e.to_string()));
        r.parts[i].host.finish(result)
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_instrument_prepare() -> u32 {
    registry(|r| {
        let i = r.index();
        let result = serde_json::from_slice::<Value>(&r.parts[i].host.input)
            .map_err(|e| e.to_string())
            .and_then(|v| r.prepare_instrument(&v));
        let i = r.index();
        r.parts[i].host.finish(result)
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_instrument_load() -> u32 {
    registry(|r| {
        let index = r.index();
        let bytes = std::mem::take(&mut r.parts[index].host.input);
        let result = r
            .load_instrument(&bytes)
            .and_then(|_| serde_json::to_vec(&r.context()).map_err(|e| e.to_string()));
        let index = r.index();
        r.parts[index].host.finish(result)
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_scene_document() -> u32 {
    registry(|r| {
        let result = r
            .document()
            .and_then(|v| serde_json::to_vec(&v).map_err(|e| e.to_string()));
        let i = r.index();
        r.parts[i].host.finish(result)
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_scene_prepare() -> u32 {
    registry(|r| {
        let i = r.index();
        let result = serde_json::from_slice::<Value>(&r.parts[i].host.input)
            .map_err(|e| e.to_string())
            .and_then(|v| r.prepare_document(v))
            .and_then(|_| r.pack());
        let i = r.index();
        r.parts[i].host.finish(result)
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_scene_render(seconds: f64) -> u32 {
    registry(|r| {
        let result = (|| {
            if !seconds.is_finite() || !(0.1..=120.).contains(&seconds) {
                return Err("Render duration must be 0.1–120 seconds".into());
            }
            for part in &mut r.parts {
                part.host.engine.set_midi_gate(false);
                part.host.engine.set_playing(true);
            }
            let rate = r.parts[0].host.rate;
            let mut frames = vec![[0.; 2]; (seconds * rate as f64).round() as usize];
            for chunk in frames.chunks_mut(BLOCK) {
                r.process(chunk.len());
                chunk.copy_from_slice(&r.parts[0].host.frames[..chunk.len()]);
            }
            wav::bytes(rate, &frames)
        })();
        let i = r.index();
        r.parts[i].host.finish(result)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn sad_scene_adopt() -> u32 {
    registry(|r| {
        let i = r.index();
        let bytes = std::mem::take(&mut r.parts[i].host.input);
        let result = r
            .unpack(&bytes, false)
            .and_then(|_| serde_json::to_vec(&json!({"ok":true})).map_err(|e| e.to_string()));
        let i = r.index();
        r.parts[i].host.finish(result)
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_scene_render_document() -> u32 {
    registry(|r| {
        let i = r.index();
        let result = (|| {
            let data: Value =
                serde_json::from_slice(&r.parts[i].host.input).map_err(|e| e.to_string())?;
            let seconds = data["seconds"].as_f64().ok_or("Missing render duration")?;
            if !seconds.is_finite() || !(0.1..=120.).contains(&seconds) {
                return Err("Render duration must be 0.1–120 seconds".into());
            }
            let rate = r.parts[0].host.rate;
            let mut renderer = Registry::new(rate);
            renderer.prepare_document(data["scene"].clone())?;
            renderer.adopt_render_clips();
            for part in &mut renderer.parts {
                part.host.engine.set_midi_gate(false);
                part.host.engine.set_playing(true);
            }
            let mut frames = vec![[0.; 2]; (seconds * rate as f64).round() as usize];
            for chunk in frames.chunks_mut(BLOCK) {
                renderer.process(chunk.len());
                chunk.copy_from_slice(&renderer.parts[0].host.frames[..chunk.len()]);
            }
            wav::bytes(rate, &frames)
        })();
        r.parts[i].host.finish(result)
    })
}

/// Preparation is called once per worklet quantum; existing parts keep rendering.
#[unsafe(no_mangle)]
pub extern "C" fn sad_timbre_prepare_begin() -> u32 {
    registry(|r| {
        let index = r.index();
        let result = serde_json::from_slice::<Value>(&r.parts[index].host.input)
            .map_err(|e| e.to_string())
            .and_then(|v| {
                let kind = serde_json::from_value(v["kind"].clone()).map_err(|e| e.to_string())?;
                r.begin_part(kind, v["classic"].as_bool().unwrap_or(false))
            })
            .map(|_| b"{}".to_vec());
        r.parts[index].host.finish(result)
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_timbre_prepare_step() -> u32 {
    registry(|r| {
        let result = r.prepare_part().and_then(|done| {
            if done {
                r.sync_vocal();
            }
            serde_json::to_vec(&json!({"done":done,"context":done.then(||r.context())}))
                .map_err(|e| e.to_string())
        });
        let index = r.index();
        r.parts[index].host.finish(result)
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_timbre_prepare_cancel() {
    registry(|r| r.preparing = None);
}
