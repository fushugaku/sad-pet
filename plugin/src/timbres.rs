//! Control-thread part creation, linked audio ownership and scene recall.
use super::*;
use sad_pet::timbres::{Context, Kind, Meta, Mix};

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
            let mut control = Controller::new(self.effect);
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
        if path.starts_with("/api/effect-presets") {
            return self.effect_presets_request(path, request["method"] == "POST", data);
        }
        if path.starts_with("/api/timbres") {
            match path {
                "/api/timbres" => {}
                "/api/timbres/add" => {
                    let kind: Kind =
                        serde_json::from_value(data["kind"].clone()).map_err(|e| e.to_string())?;
                    let id = self.next_timbre_id;
                    let next_id = id.checked_add(1).ok_or("Timbre identity overflow")?;
                    let count = self.parts.iter().filter(|p| p.meta.kind == kind).count()
                        + usize::from(kind == Kind::Granular)
                        + 1;
                    let mut part = Controller::new(self.effect);
                    part.meta = Meta {
                        id,
                        kind,
                        name: format!(
                            "{} {count}",
                            if kind == Kind::Drums {
                                "Drums"
                            } else {
                                "Granular"
                            }
                        ),
                        mix: Mix::default(),
                    };
                    part.source = Source::File;
                    part.playing = self.playing;
                    part.midi_gate = self.midi_gate;
                    part.sync = self.sync;
                    if kind == Kind::Drums {
                        part.preset.drums = Some(Box::default());
                        part.preset.name = "Drums".into();
                        part.midi_gate = false;
                    }
                    let audio = part.prepare(self.rate);
                    self.enqueue(Command::AddPart(Some(Box::new(PartAudio {
                        audio,
                        next: None,
                    }))))?;
                    self.parts.push(part);
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
                    for key in ["name", "gain", "muted", "solo", "channel"] {
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
                    if !self.parts.iter().any(|p| p.meta.id == id) {
                        return Err("Unknown timbre".into());
                    }
                    self.enqueue(Command::RemovePart(id, None))?;
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
        if path == "/api/state" {
            result["timbres"] = json!(self.context());
            result["preset_selection"] = json!(self.selection);
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
    pub(super) fn process_frame(&mut self, input: Stereo) -> Stereo {
        let target = if self.audible { self.meta.mix.gain } else { 0. };
        self.smooth_part_gain += (target - self.smooth_part_gain) * 0.004;
        let mut output = self
            .engine
            .process_input_frame(input)
            .map(|v| v * self.smooth_part_gain);
        let mut part = self.parts.as_mut();
        while let Some(p) = part {
            let frame = p.audio.process_frame(input);
            output[0] += frame[0];
            output[1] += frame[1];
            part = p.next.as_mut();
        }
        output
    }
    pub(super) fn midi(&mut self, message: Message) {
        if self.meta.mix.accepts(message) {
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
