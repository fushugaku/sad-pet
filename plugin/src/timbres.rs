//! Control-thread part creation, linked audio ownership and scene recall.
use super::*;
use sad_pet::{
    bus::MAX_BUSES,
    timbres::{Context, DESTINATIONS, Entered, Kind, Meta, Mix, SendGains},
};

/// Adds a Master when the scene has none (new instances and older projects).
pub(super) fn ensure_master_in(
    parts: &mut Vec<Controller>,
    head: &mut Option<Box<PartAudio>>,
    root: &Controller,
    rate: u32,
) {
    if parts.iter().any(|p| p.meta.kind == Kind::Master) {
        return;
    }
    let id = parts
        .iter()
        .map(|p| p.meta.id)
        .max()
        .unwrap_or(0)
        .max(root.next_timbre_id.saturating_sub(1))
        .saturating_add(1);
    let mut master = root.routing_part(Kind::Master, id, 0);
    *head = Some(Box::new(PartAudio {
        audio: master.prepare(rate),
        next: head.take(),
    }));
    parts.push(master);
}
/// A Bus or Master keeps its role when a preset loads into it; any other
/// timbre ignores bus settings.
pub(super) fn keep_role(preset: &mut Preset, current: &Preset, kind: Kind) {
    if kind.keeps_role() {
        if preset.bus.is_none() {
            preset.bus = current.bus.clone();
        }
        if let Some(bus) = &mut preset.bus {
            bus.master = kind == Kind::Master;
        }
    } else {
        preset.bus = None;
    }
    // A Vocal timbre keeps its score, and a Rack preset brings that of its
    // Vocal module; nothing else gains one.
    if kind == Kind::Vocal {
        if preset.vocal.is_none() {
            preset.vocal = current.vocal.clone();
        }
    } else if preset.rack.is_none() {
        preset.vocal = None;
    }
}

pub(super) struct PartAudio {
    pub audio: Audio,
    pub next: Option<Box<PartAudio>>,
}

/// Frames a part renders at a time before the parts are summed.
pub(super) const CHUNK: usize = 128;

/// One part's chunk: its input, its level-scaled output and its bus sends,
/// one slot per frame and bus, with the buses it wrote as a mask. A Bus or
/// Master also takes what enters its chain after the input while `entering`.
pub(super) struct Chunk {
    pub input: [Stereo; CHUNK],
    pub out: [Stereo; CHUNK],
    pub sends: [[Stereo; MAX_BUSES]; CHUNK],
    pub mask: u8,
    pub entered: [Entered; CHUNK],
    pub entering: bool,
}
impl Chunk {
    pub(super) fn new() -> Box<Self> {
        Box::new(Self {
            input: [[0.; 2]; CHUNK],
            out: [[0.; 2]; CHUNK],
            sends: [[[0.; 2]; MAX_BUSES]; CHUNK],
            mask: 0,
            entered: [Default::default(); CHUNK],
            entering: false,
        })
    }
}

/// Chunks shorter than this render on the audio thread alone: per-frame
/// calls would cost more to hand out than they take to render.
const MIN_PARALLEL: usize = 16;

/// One chunk's jobs for the workers, without allocating.
pub(super) struct Jobs {
    jobs: [Option<workers::Job>; workers::MAX_JOBS],
    len: usize,
}
impl Default for Jobs {
    fn default() -> Self {
        Self {
            jobs: [None; workers::MAX_JOBS],
            len: 0,
        }
    }
}
impl Jobs {
    /// False when full: the caller renders that part itself.
    fn push(&mut self, job: workers::Job) -> bool {
        if self.len == workers::MAX_JOBS {
            return false;
        }
        self.jobs[self.len] = Some(job);
        self.len += 1;
        true
    }
    fn len(&self) -> usize {
        self.len
    }
    fn is_empty(&self) -> bool {
        self.len == 0
    }
    fn as_slice(&self) -> &[Option<workers::Job>] {
        &self.jobs[..self.len]
    }
}

/// The fields of one part that rendering a chunk touches, and nothing else.
pub(super) struct Render<'a> {
    pub engine: &'a mut Engine,
    pub chunk: &'a mut Chunk,
    pub vocal: &'a mut Option<Box<VocalAudio>>,
    pub sends: &'a mut SendGains,
    pub gain: &'a mut f32,
    pub audible: bool,
    pub mix: &'a Mix,
}
impl Render<'_> {
    fn source(self, input: &[Stereo], buses: bool) {
        let n = input.len();
        match self.vocal {
            Some(vocal) => {
                for frame in &mut self.chunk.input[..n] {
                    *frame = [vocal.next(); 2];
                }
            }
            None => self.chunk.input[..n].copy_from_slice(input),
        }
        self.render(n, buses);
    }
    /// A Bus or Master renders the input already in its chunk, and a Bus
    /// sends on to the buses after it (`sends`).
    fn routing(self, n: usize, sends: bool) {
        self.render(n, sends);
    }
    fn render(self, n: usize, buses: bool) {
        let chunk = &mut *self.chunk;
        // A bus slot is written while its send gain is non-zero or ramping;
        // a send that is zero and stays zero is skipped, as before. So is a
        // bus that no section of this part is routed to.
        chunk.mask = 0;
        let taps = if buses { self.engine.tap_buses() } else { 0 };
        if buses {
            for (bus, (&gain, &level)) in self.sends.gains().iter().zip(&self.mix.sends).enumerate()
            {
                if gain != 0.0 || (self.audible && level != 0.0) {
                    chunk.mask |= 1 << bus;
                }
            }
            chunk.mask |= taps;
            if chunk.mask != 0 {
                for sends in &mut chunk.sends[..n] {
                    *sends = [[0.; 2]; MAX_BUSES];
                }
            }
        }
        if taps == 0 && !chunk.entering {
            self.engine
                .process_input_block(&chunk.input[..n], &mut chunk.out[..n]);
        } else {
            // Section sends and later entries change from frame to frame:
            // take each frame's as it renders, the way the block renders
            // anyway.
            for (((input, out), sends), entered) in chunk.input[..n]
                .iter()
                .zip(&mut chunk.out[..n])
                .zip(&mut chunk.sends[..n])
                .zip(&chunk.entered[..n])
            {
                if chunk.entering {
                    self.engine.enter(*entered);
                }
                *out = self.engine.process_input_frame(*input);
                if taps != 0 {
                    self.sends
                        .send_taps(self.audible, self.engine.bus_sends(), sends);
                }
            }
        }
        let target = if self.audible { self.mix.gain } else { 0. };
        for (out, sends) in chunk.out[..n].iter_mut().zip(&mut chunk.sends[..n]) {
            *self.gain += (target - *self.gain) * 0.004;
            if chunk.mask != 0 {
                self.sends.send(&self.mix.sends, self.audible, *out, sends);
            }
            *out = out.map(|v| v * *self.gain);
        }
    }
}

/// Adds one part's chunk to the Master input (`output`) and, with `buses`, to
/// the bus inputs; a destination it enters after the input gets the shares
/// of its other entries in `entered`. The scene's own part comes `first` and
/// starts the Master input. Returns the destinations it split.
#[allow(clippy::too_many_arguments)]
fn add_part(
    chunk: &Chunk,
    gains: &mut SendGains,
    mix: &Mix,
    buses: bool,
    first: bool,
    output: &mut [Stereo],
    sends: &mut [[Stereo; MAX_BUSES]],
    entered: &mut [[Entered; CHUNK]; DESTINATIONS],
) -> u16 {
    let n = output.len();
    let split = gains.plan(&mix.entries);
    if split & 1 != 0 {
        if first {
            output.fill([0.; 2]);
        }
        for ((out, frame), slot) in output.iter_mut().zip(&chunk.out[..n]).zip(&mut entered[0]) {
            gains.enter(0, &mix.entries[0], *frame, out, slot);
        }
    } else if first {
        output.copy_from_slice(&chunk.out[..n]);
    } else {
        for (out, frame) in output.iter_mut().zip(&chunk.out[..n]) {
            out[0] += frame[0];
            out[1] += frame[1];
        }
    }
    if buses {
        let routed = (split >> 1) as u8;
        let plain = chunk.mask & !routed;
        if plain != 0 {
            for (sum, part) in sends.iter_mut().zip(&chunk.sends[..n]) {
                add_sends(sum, part, plain);
            }
        }
        let mut pending = routed;
        while pending != 0 {
            let bus = pending.trailing_zeros() as usize;
            pending &= pending - 1;
            // A silent send still ramps its entries.
            let sent = chunk.mask & 1 << bus != 0;
            for ((sum, part), slot) in sends
                .iter_mut()
                .zip(&chunk.sends[..n])
                .zip(&mut entered[bus + 1])
            {
                let frame = if sent { part[bus] } else { [0.; 2] };
                gains.enter(bus + 1, &mix.entries[bus + 1], frame, &mut sum[bus], slot);
            }
        }
    }
    split
}

/// Adds one part's send slots of a frame to the bus inputs, for the buses
/// in `mask`. A slot starts from zero, so adding it later is the same sum.
#[inline]
fn add_sends(sum: &mut [Stereo; MAX_BUSES], part: &[Stereo; MAX_BUSES], mut mask: u8) {
    while mask != 0 {
        let bus = mask.trailing_zeros() as usize;
        mask &= mask - 1;
        sum[bus][0] += part[bus][0];
        sum[bus][1] += part[bus][1];
    }
}

pub(super) fn remove_part(
    head: &mut Option<Box<PartAudio>>,
    id: u32,
    retired: &mut Option<Box<PartAudio>>,
) {
    let mut cursor = head;
    loop {
        match cursor {
            Some(p) if p.audio.meta.id != id => cursor = &mut cursor.as_mut().unwrap().next,
            Some(_) => {
                let mut part = cursor.take().unwrap();
                *cursor = part.next.take();
                *retired = Some(part);
                return;
            }
            None => return,
        }
    }
}

impl Controller {
    /// A Bus or Master starts with everything off.
    pub(super) fn routing_part(&self, kind: Kind, id: u32, bus: u8) -> Controller {
        let mut part = self.new_part();
        part.meta = Meta {
            id,
            kind,
            bus,
            name: if kind == Kind::Bus {
                format!("Bus {bus}")
            } else {
                kind.label().into()
            },
            mix: Mix::default(),
            ..Meta::default()
        };
        part.preset = kind.preset().unwrap();
        part.preset.sanitize();
        part.source = Source::File;
        part.playing = self.playing;
        part.midi_gate = false;
        part.sync = self.sync;
        part.rate = self.rate;
        part
    }
    /// Every scene has exactly one Master, prepared with the other parts.
    pub(super) fn ensure_master(&mut self) {
        if !self.parts.iter().any(|p| p.meta.kind == Kind::Master) {
            let id = self.next_timbre_id;
            self.next_timbre_id = id.saturating_add(1);
            let master = self.routing_part(Kind::Master, id, 0);
            self.parts.push(master);
        }
    }
    pub(super) fn context(&self) -> Context {
        let mut context = Context {
            name: self.preset.name.clone(),
            active: self.active,
            revision: self.scene_revision,
            timbres: std::iter::once(self.meta.clone())
                .chain(self.parts.iter().map(|p| p.meta.clone()))
                .collect(),
        };
        context.note_feeds(&self.bus_feeds());
        context
    }
    /// Which buses each Bus feeds, loops left out.
    fn bus_feeds(&self) -> sad_pet::timbres::Feeds {
        sad_pet::timbres::bus_feeds(
            std::iter::once(self)
                .chain(&self.parts)
                .map(|p| (&p.meta, &p.preset)),
        )
    }
    pub(super) fn prepare_parts(&mut self, rate: u32) -> Option<Box<PartAudio>> {
        let mut head = None;
        for part in self.parts.iter_mut().rev() {
            head = Some(Box::new(PartAudio {
                audio: part.prepare(rate),
                next: head,
            }));
        }
        head
    }
    pub(super) fn restore_parts(
        &self,
        states: Vec<SavedState>,
    ) -> Result<(Vec<Controller>, Option<Box<PartAudio>>), String> {
        let mut controls = Vec::with_capacity(states.len());
        let mut ids = std::collections::BTreeSet::new();
        for state in states {
            if state.meta.id == 0 || !ids.insert(state.meta.id) || !state.timbres.is_empty() {
                return Err("Invalid timbre identities".into());
            }
            let mut control = self.new_part();
            control.meta = state.meta.clone();
            control.rate = self.rate;
            control.restore_saved(state)?;
            controls.push(control);
        }
        let mut head = None;
        for control in controls.iter_mut().rev() {
            head = Some(Box::new(PartAudio {
                audio: control.prepare(self.rate),
                next: head,
            }));
        }
        Ok((controls, head))
    }
    pub(super) fn scene_request(&mut self, request: &Value) -> Result<Value, String> {
        self.drain();
        let path = request["path"].as_str().ok_or("Missing command")?;
        let data = &request["data"];
        if path == "/api/sample-preview" && request["method"] == "POST" {
            return self.preview_request(data);
        }
        if self.standalone {
            let post = request["method"] == "POST";
            super::audio_library::check_editor_request(path, post, data)?;
            if path.starts_with("/api/library") {
                return self.library_request(path, post, data);
            }
        }
        if path.starts_with("/api/effect-presets") {
            return self.effect_presets_request(path, request["method"] == "POST", data);
        }
        if path.starts_with("/api/instruments") {
            return self.instruments_request(path, request["method"] == "POST", data);
        }
        if let Some(result) = self.vocal_scene_request(path, request["method"] == "POST", data) {
            return result;
        }
        if path.starts_with("/api/timbres") {
            match path {
                "/api/timbres" => {}
                "/api/timbres/add" => {
                    let kind: Kind =
                        serde_json::from_value(data["kind"].clone()).map_err(|e| e.to_string())?;
                    if kind == Kind::Master {
                        return Err("The scene always has one Master".into());
                    }
                    if matches!(kind, Kind::Sampler | Kind::PadFx) {
                        return Err("Sampler timbres play in the browser version for now".into());
                    }
                    let id = self.next_timbre_id;
                    let next_id = id.checked_add(1).ok_or("Timbre identity overflow")?;
                    let count = self.parts.iter().filter(|p| p.meta.kind == kind).count()
                        + usize::from(kind == Kind::Granular)
                        + 1;
                    let mut part = if kind == Kind::Bus {
                        let bus = sad_pet::timbres::free_bus(
                            std::iter::once(&self.meta).chain(self.parts.iter().map(|p| &p.meta)),
                        )
                        .ok_or(format!("Use at most {MAX_BUSES} buses"))?;
                        self.routing_part(kind, id, bus)
                    } else {
                        let mut part = self.new_part();
                        part.meta = Meta {
                            id,
                            kind,
                            name: format!("{} {count}", kind.label()),
                            ..Meta::default()
                        };
                        part.source = Source::File;
                        part.playing = self.playing;
                        part.midi_gate = self.midi_gate;
                        part.sync = self.sync;
                        if let Some(preset) = kind.preset() {
                            part.preset = preset;
                            part.preset.sanitize();
                            // Drums run free with the transport; a synth waits for
                            // notes, and so does a Rack, whose drums play anyway.
                            part.midi_gate = matches!(kind, Kind::Synth | Kind::Rack);
                        }
                        part
                    };
                    let audio = part.prepare(self.rate);
                    self.enqueue(Command::AddPart(Some(Box::new(PartAudio {
                        audio,
                        next: None,
                    }))))?;
                    // Master stays the last tab; new timbres go before it.
                    let at = self
                        .parts
                        .iter()
                        .position(|p| p.meta.kind == Kind::Master)
                        .unwrap_or(self.parts.len());
                    self.parts.insert(at, part);
                    self.next_timbre_id = next_id;
                    self.active = id;
                    self.selection.dirty = true;
                }
                "/api/timbres/select" => {
                    let id = data["id"].as_u64().ok_or("Choose a timbre")? as u32;
                    if id != 0 && !self.parts.iter().any(|p| p.meta.id == id) {
                        return Err("Unknown timbre".into());
                    }
                    self.active = id;
                }
                "/api/timbres/update" => {
                    let id = data["id"].as_u64().ok_or("Choose a timbre")? as u32;
                    let mut meta = if id == 0 {
                        self.meta.clone()
                    } else {
                        self.parts
                            .iter()
                            .find(|p| p.meta.id == id)
                            .ok_or("Unknown timbre")?
                            .meta
                            .clone()
                    };
                    let mut value = serde_json::to_value(&meta).unwrap();
                    for key in [
                        "name", "gain", "muted", "solo", "channel", "sends", "entries",
                    ] {
                        if let Some(v) = data.get(key) {
                            value[key] = v.clone();
                        }
                    }
                    meta = serde_json::from_value(value).map_err(|e| e.to_string())?;
                    meta.sanitize();
                    // A bus refuses a send that would feed back into itself.
                    if meta.kind == Kind::Bus {
                        sad_pet::timbres::refuse_loops(&mut meta.mix, &self.bus_feeds(), meta.bus);
                    }
                    self.enqueue(Command::PartMix(id, meta.mix))?;
                    if id == 0 {
                        self.meta = meta;
                    } else {
                        self.parts
                            .iter_mut()
                            .find(|p| p.meta.id == id)
                            .unwrap()
                            .meta = meta;
                    }
                    self.selection.dirty = true;
                }
                "/api/timbres/remove" => {
                    let id = data["id"].as_u64().ok_or("Choose a timbre")? as u32;
                    if id == 0 {
                        return Err("Keep the first timbre for DAW automation".into());
                    }
                    let removed = self
                        .parts
                        .iter()
                        .find(|p| p.meta.id == id)
                        .ok_or("Unknown timbre")?
                        .meta
                        .clone();
                    if removed.kind == Kind::Master {
                        return Err("The scene always has one Master".into());
                    }
                    if removed.kind == Kind::Bus && removed.bus > 0 {
                        // Nothing keeps sending to the removed bus number.
                        if self.meta.mix.forget_bus(removed.bus) {
                            self.enqueue(Command::PartMix(0, self.meta.mix))?;
                        }
                        for i in 0..self.parts.len() {
                            if self.parts[i].meta.mix.forget_bus(removed.bus) {
                                let (id, mix) = (self.parts[i].meta.id, self.parts[i].meta.mix);
                                self.enqueue(Command::PartMix(id, mix))?;
                            }
                        }
                    }
                    self.enqueue(Command::RemovePart(id, None))?;
                    if removed.kind == Kind::Vocal {
                        self.voice_service().forget(id);
                    }
                    self.parts.retain(|p| p.meta.id != id);
                    if self.active == id {
                        self.active = 0;
                    }
                    self.selection.dirty = true;
                }
                _ => return Err("Unknown timbre command".into()),
            }
            self.offer_workers();
            self.scene_revision = self.scene_revision.wrapping_add(1);
            return Ok(json!(self.context()));
        }
        if path == "/api/scene" {
            if request["method"] == "POST" {
                let saved: SavedState =
                    serde_json::from_value(data.clone()).map_err(|e| e.to_string())?;
                self.restore_saved(saved)?;
                return Ok(json!({"ok":true}));
            }
            return serde_json::to_value(self.snapshot()?).map_err(|e| e.to_string());
        }
        if path.starts_with("/api/preset-library") {
            return self.request_part(request);
        }
        if path == "/api/transport" && request["method"] == "POST" {
            // Play and DAW sync belong to the scene; Freeze/Bypass are per timbre.
            let mut shared = serde_json::Map::new();
            for key in ["playing", "sync", "clear_hold"] {
                if let Some(v) = data.get(key) {
                    shared.insert(key.into(), v.clone());
                }
            }
            if !shared.is_empty() {
                let global = json!({"path":path,"method":"POST","data":shared});
                self.request_part(&global)?;
                for part in &mut self.parts {
                    part.request_part(&global)?;
                }
            }
        }
        let mut result = if self.active == 0 {
            self.request_part(request)?
        } else {
            self.parts
                .iter_mut()
                .find(|p| p.meta.id == self.active)
                .ok_or("Unknown timbre")?
                .request_part(request)?
        };
        let (route, query) = path.split_once('?').unwrap_or((path, ""));
        if route == "/api/state" {
            result["timbres"] = json!(self.context());
            result["preset_selection"] = json!(self.selection);
            result["midi_controls"] = json!(self.controls.changes());
            self.sent.answer(&mut result, query);
        } else if request["method"] == "POST"
            && !matches!(
                path,
                "/api/note" | "/api/drums/trigger" | "/api/sequencer/view"
            )
        {
            self.selection.dirty = true;
        }
        Ok(result)
    }
}

impl Audio {
    pub(super) fn set_part_mix(&mut self, id: u32, mix: Mix) {
        if self.meta.id == id {
            if self.meta.mix.channel != mix.channel && self.engine.preset.drums.is_none() {
                self.engine.midi(Message::Reset);
            }
            self.meta.mix = mix;
        } else {
            let mut part = self.parts.as_mut();
            while let Some(p) = part {
                p.audio.set_part_mix(id, mix);
                part = p.next.as_mut();
            }
        }
    }
    /// One part's level-scaled output for a chunk, plus its sends into its
    /// own bus slots. Every part renders its whole chunk before the parts
    /// are summed, so one engine's state stays in cache for the chunk; the
    /// per-frame arithmetic, and so the rendered audio, is unchanged.
    pub(super) fn source_chunk(&mut self, input: &[Stereo], buses: bool) {
        self.render().source(input, buses);
    }
    /// A Bus or Master renders the input already in its chunk buffer.
    pub(super) fn source_routing_chunk(&mut self, n: usize) {
        let sends = self.meta.kind == Kind::Bus;
        self.render().routing(n, sends);
    }
    fn render(&mut self) -> Render<'_> {
        Render {
            engine: &mut self.engine,
            chunk: &mut self.chunk,
            vocal: &mut self.vocal,
            sends: &mut self.sends,
            gain: &mut self.smooth_part_gain,
            audible: self.audible,
            mix: &self.meta.mix,
        }
    }
    /// Timbres sum into Master and send to buses; buses then feed the buses
    /// after them and Master, whose engine renders the output. With no buses
    /// and Master at its defaults, the output is the plain sum of the
    /// timbres as before.
    pub(super) fn process_frame(&mut self, input: Stereo) -> Stereo {
        let mut output = [[0.; 2]];
        self.process_block(&[input], &mut output);
        output[0]
    }
    /// `output` frames from the same number of `input` frames, a chunk of up
    /// to `CHUNK` frames at a time, each chunk part by part.
    pub(super) fn process_block(&mut self, input: &[Stereo], output: &mut [Stereo]) {
        for (input, output) in input.chunks(CHUNK).zip(output.chunks_mut(CHUNK)) {
            self.process_chunk(input, output);
        }
    }
    fn process_chunk(&mut self, input: &[Stereo], output: &mut [Stereo]) {
        let n = input.len();
        let buses = self.buses;
        let parallel = n >= MIN_PARALLEL;
        {
            // Field by field: the workers own the other parts' audio while
            // this thread renders the scene's own part.
            let Audio {
                parts,
                pool,
                engine,
                chunk,
                vocal,
                sends,
                smooth_part_gain,
                audible,
                meta,
                ..
            } = self;
            let pool = pool.as_deref_mut().filter(|_| parallel);
            let mut jobs = Jobs::default();
            let mut part = parts.as_mut();
            while let Some(p) = part {
                if !p.audio.meta.kind.routing() {
                    let job = workers::Job::Timbre {
                        audio: &mut p.audio,
                        input: input.as_ptr(),
                        n,
                        buses,
                    };
                    if pool.is_none() || !jobs.push(job) {
                        p.audio.source_chunk(input, buses);
                    }
                }
                part = p.next.as_mut();
            }
            let render = Render {
                engine,
                chunk,
                vocal,
                sends,
                gain: smooth_part_gain,
                audible: *audible,
                mix: &meta.mix,
            };
            let own = move || render.source(input, buses);
            match pool {
                Some(pool) if !jobs.is_empty() => pool.run(jobs.as_slice(), own),
                _ => own(),
            }
        }
        // The frame-wise order of the sums is kept: this part, then every
        // timbre in list order, into the output and into each bus input.
        // Destinations a timbre enters after their input (bit 0 Master,
        // bit b Bus b) also get the other entries' shares.
        let mut sends = [[[0.; 2]; MAX_BUSES]; CHUNK];
        let mut entered = [[Entered::default(); CHUNK]; DESTINATIONS];
        let mut split = add_part(
            &self.chunk,
            &mut self.sends,
            &self.meta.mix,
            buses,
            true,
            output,
            &mut sends[..n],
            &mut entered,
        );
        let mut part = self.parts.as_mut();
        while let Some(p) = part {
            if !p.audio.meta.kind.routing() {
                let audio = &mut p.audio;
                split |= add_part(
                    &audio.chunk,
                    &mut audio.sends,
                    &audio.meta.mix,
                    buses,
                    false,
                    output,
                    &mut sends[..n],
                    &mut entered,
                );
            }
            part = p.next.as_mut();
        }
        if buses {
            // The buses of one wave read only sends that are complete, so they
            // render together; they then sum in order, and send to the buses
            // of later waves.
            let order = self.bus_order;
            for wave in order.waves() {
                let in_wave = |bus: u8| wave.contains(&bus);
                let Audio { parts, pool, .. } = self;
                let pool = pool.as_deref_mut().filter(|_| parallel);
                let mut jobs = Jobs::default();
                let mut part = parts.as_mut();
                while let Some(p) = part {
                    if p.audio.meta.kind == Kind::Bus && in_wave(p.audio.meta.bus) {
                        let bus = p.audio.meta.bus as usize - 1;
                        let chunk = &mut *p.audio.chunk;
                        for (input, sends) in chunk.input[..n].iter_mut().zip(&sends[..n]) {
                            *input = sends[bus];
                        }
                        chunk.entering = split & 1 << (bus + 1) != 0;
                        if chunk.entering {
                            chunk.entered[..n].copy_from_slice(&entered[bus + 1][..n]);
                        }
                        let job = workers::Job::Routing {
                            audio: &mut p.audio,
                            n,
                        };
                        if pool.is_none() || !jobs.push(job) {
                            p.audio.source_routing_chunk(n);
                        }
                    }
                    part = p.next.as_mut();
                }
                match pool {
                    Some(pool) if jobs.len() > 1 => pool.run(jobs.as_slice(), || {}),
                    // SAFETY: nothing else borrows the parts behind these jobs.
                    _ => jobs
                        .as_slice()
                        .iter()
                        .flatten()
                        .for_each(|job| unsafe { job.run() }),
                }
                for &bus in wave {
                    let mut part = self.parts.as_mut();
                    while let Some(p) = part {
                        if p.audio.meta.kind == Kind::Bus && p.audio.meta.bus == bus {
                            let audio = &mut p.audio;
                            split |= add_part(
                                &audio.chunk,
                                &mut audio.sends,
                                &audio.meta.mix,
                                true,
                                false,
                                output,
                                &mut sends[..n],
                                &mut entered,
                            );
                            break;
                        }
                        part = p.next.as_mut();
                    }
                }
            }
        }
        let mut part = self.parts.as_mut();
        while let Some(p) = part {
            if p.audio.meta.kind == Kind::Master {
                let chunk = &mut *p.audio.chunk;
                chunk.input[..n].copy_from_slice(output);
                chunk.entering = split & 1 != 0;
                if chunk.entering {
                    chunk.entered[..n].copy_from_slice(&entered[0][..n]);
                }
                p.audio.source_routing_chunk(n);
                output.copy_from_slice(&p.audio.chunk.out[..n]);
                break;
            }
            part = p.next.as_mut();
        }
        // A sound being previewed plays on top of the scene's output.
        if let Some(preview) = &mut self.preview {
            for out in output.iter_mut() {
                let frame = preview.next();
                *out = [out[0] + frame[0], out[1] + frame[1]];
            }
        }
    }
    pub(super) fn midi(&mut self, message: Message) {
        if self.meta.kind.takes_midi() && self.meta.mix.accepts(message) {
            self.engine.midi(message);
        }
        let mut part = self.parts.as_mut();
        while let Some(p) = part {
            p.audio.midi(message);
            part = p.next.as_mut();
        }
    }
    pub(super) fn time_signature(&mut self, numerator: u32, denominator: u32) {
        self.engine.set_time_signature(numerator, denominator);
        let mut part = self.parts.as_mut();
        while let Some(p) = part {
            p.audio.time_signature(numerator, denominator);
            part = p.next.as_mut();
        }
    }
}
