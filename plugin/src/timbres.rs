//! Control-thread part creation, linked audio ownership and scene recall.
use super::*;
use sad_pet::{
    bus::MAX_BUSES,
    timbres::{Context, Kind, Meta, Mix},
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
    // A Vocal timbre keeps its score; nothing else gains one.
    if kind == Kind::Vocal {
        if preset.vocal.is_none() {
            preset.vocal = current.vocal.clone();
        }
    } else {
        preset.vocal = None;
    }
}

pub(super) struct PartAudio {
    pub audio: Audio,
    pub next: Option<Box<PartAudio>>,
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
    fn context(&self) -> Context {
        Context {
            name: self.preset.name.clone(),
            active: self.active,
            revision: self.scene_revision,
            timbres: std::iter::once(self.meta.clone())
                .chain(self.parts.iter().map(|p| p.meta.clone()))
                .collect(),
        }
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
                            // Drums run free with the transport; a synth waits for notes.
                            part.midi_gate = kind == Kind::Synth;
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
                    for key in ["name", "gain", "muted", "solo", "channel", "sends"] {
                        if let Some(v) = data.get(key) {
                            value[key] = v.clone();
                        }
                    }
                    meta = serde_json::from_value(value).map_err(|e| e.to_string())?;
                    meta.sanitize();
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
                        let index = removed.bus as usize - 1;
                        if self.meta.mix.sends[index] != 0. {
                            self.meta.mix.sends[index] = 0.;
                            self.enqueue(Command::PartMix(0, self.meta.mix))?;
                        }
                        for i in 0..self.parts.len() {
                            if self.parts[i].meta.mix.sends[index] != 0. {
                                self.parts[i].meta.mix.sends[index] = 0.;
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
    /// One part's level-scaled output, plus its sends into the bus inputs.
    fn source_frame(
        &mut self,
        input: Stereo,
        sends: &mut [Stereo; MAX_BUSES],
        buses: bool,
    ) -> Stereo {
        let target = if self.audible { self.meta.mix.gain } else { 0. };
        self.smooth_part_gain += (target - self.smooth_part_gain) * 0.004;
        let input = match &mut self.vocal {
            Some(vocal) => [vocal.next(); 2],
            None => input,
        };
        let out = self.engine.process_input_frame(input);
        if buses {
            self.sends
                .send(&self.meta.mix.sends, self.audible, out, sends);
        }
        out.map(|v| v * self.smooth_part_gain)
    }
    /// Timbres sum into Master and send to buses; buses then feed Master,
    /// whose engine renders the output. With no buses and Master at its
    /// defaults, the output is the plain sum of the timbres as before.
    pub(super) fn process_frame(&mut self, input: Stereo) -> Stereo {
        let buses = self.buses;
        let mut sends = [[0.; 2]; MAX_BUSES];
        let mut output = self.source_frame(input, &mut sends, buses);
        let mut part = self.parts.as_mut();
        while let Some(p) = part {
            if !p.audio.meta.kind.routing() {
                let frame = p.audio.source_frame(input, &mut sends, buses);
                output[0] += frame[0];
                output[1] += frame[1];
            }
            part = p.next.as_mut();
        }
        let mut master = None;
        let mut part = self.parts.as_mut();
        while let Some(p) = part {
            match p.audio.meta.kind {
                Kind::Bus if buses && p.audio.meta.bus > 0 => {
                    let bus = sends[p.audio.meta.bus as usize - 1];
                    let frame = p.audio.source_frame(bus, &mut [[0.; 2]; MAX_BUSES], false);
                    output[0] += frame[0];
                    output[1] += frame[1];
                }
                Kind::Master => master = Some(&mut p.audio),
                _ => {}
            }
            part = p.next.as_mut();
        }
        match master {
            Some(master) => master.source_frame(output, &mut [[0.; 2]; MAX_BUSES], false),
            None => output,
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
