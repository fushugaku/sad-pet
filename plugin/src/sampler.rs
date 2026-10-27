//! Sampler kits in the native hosts. A kit's samples stay with its part's
//! controller; its audio plays them from a runtime built here and swapped in
//! at the start of a block, so the audio thread neither allocates nor frees a
//! sample. A pad's Pad FX timbre is a part of the scene whose audio lives in
//! its Sampler's, which feeds it the pad's voices frame by frame. The app
//! plays the kits of its scenes; their samples never cross its editor.
use super::*;
use sad_pet::{
    dsp::sampler::{Pool, Runtime},
    sampler::{self as kits, BounceOptions, edit},
    timbres::Kind,
};

/// One sample of a kit, as scenes save it in the browser too.
#[derive(Clone, Serialize, Deserialize)]
pub(super) struct KitSample {
    id: String,
    #[serde(default)]
    name: String,
    wav: String,
}

pub(super) fn encode_kit(pool: &Pool) -> Result<Vec<KitSample>, String> {
    pool.iter()
        .map(|(id, sample)| {
            Ok(KitSample {
                id: id.clone(),
                name: sample.name.clone(),
                wav: BASE64.encode(wav::bytes(sample.sample_rate, sample.frames())?),
            })
        })
        .collect()
}

pub(super) fn decode_kit(samples: &[KitSample]) -> Result<Pool, String> {
    let mut pool = Pool::new();
    for item in samples {
        let id = kits::sample_id(&item.id);
        if id.is_empty() {
            continue;
        }
        let name = if item.name.is_empty() {
            "Sample"
        } else {
            &item.name
        };
        let mut sample = wav::decode(Cursor::new(decode_bytes(&item.wav)?), name.into())?;
        sample.library_id = Some(id.clone());
        pool.insert(id, Arc::new(sample));
    }
    Ok(pool)
}

fn decode_bytes(text: &str) -> Result<Vec<u8>, String> {
    let bytes = BASE64
        .decode(text)
        .map_err(|e| format!("Kit sample: {e}"))?;
    if bytes.len() > MAX_BYTES {
        return Err("A kit sample exceeds 128 MB".into());
    }
    Ok(bytes)
}

/// A WAV for the Sample editor, at its own rate.
fn edit_audio(text: &Value) -> Result<edit::Audio, String> {
    let bytes = decode_bytes(text.as_str().ok_or("Missing audio")?)?;
    let (rate, frames) = wav::frames(Cursor::new(bytes))?;
    if frames.is_empty() {
        return Err("The audio file is empty".into());
    }
    Ok(edit::Audio::new(rate, frames))
}

fn binary(rate: u32, frames: &[Stereo]) -> Result<Value, String> {
    Ok(json!({"binary":BASE64.encode(wav::bytes(rate, frames)?),"mime":"audio/wav"}))
}

/// "A01" for the first pad of bank A.
fn pad_label(index: usize) -> String {
    let bank = (b'A' + (index / kits::BANK_PADS).min(25) as u8) as char;
    format!("{bank}{:02}", index % kits::BANK_PADS + 1)
}

/// `preset` ready for a part's audio, with a kit's runtime built from
/// `kit` and the pads `routes` sends to Pad FX timbres.
pub(super) fn prepared_with(preset: &Preset, kit: &Pool, routes: &[u32], rate: u32) -> Box<Preset> {
    let mut prepared = Box::new(preset.prepared_clone_for(rate));
    prepared
        .sampler_runtime
        .prepare(prepared.sampler.as_deref(), kit, routes, rate);
    prepared
}

impl Controller {
    /// A Sampler timbre, or a Rack with a Sampler module.
    pub(super) fn plays_kit(&self) -> bool {
        matches!(self.meta.kind, Kind::Sampler | Kind::Rack) && self.preset.sampler.is_some()
    }
    pub(super) fn kit_runtime(&self) -> Option<Box<Runtime>> {
        self.preset
            .sampler
            .as_deref()
            .map(|settings| Runtime::prepared(settings, &self.kit, &self.pad_routes, self.rate))
    }
    /// `preset` ready for this part's audio, its kit included.
    pub(super) fn prepared_preset(&self, preset: &Preset) -> Box<Preset> {
        prepared_with(preset, &self.kit, &self.pad_routes, self.rate)
    }
    /// Sends the kit's runtime for a changed pool or routes. Without a queue
    /// slot it waits for the next scene request.
    pub(super) fn update_kit(&mut self) {
        self.kit_pending = match self.kit_runtime() {
            Some(runtime) => self.enqueue(Command::Sampler(Some(runtime))).is_err(),
            None => false,
        };
    }
    /// A preset without a kit lets the kit's samples go.
    pub(super) fn forget_unused_kit(&mut self) {
        if self.preset.sampler.is_none() {
            self.kit.clear();
            self.pad_routes.clear();
        }
    }
    fn timbre(&self, id: u32) -> Option<&Controller> {
        if self.meta.id == id {
            return Some(self);
        }
        self.parts.iter().find(|p| p.meta.id == id)
    }
    fn timbre_mut(&mut self, id: u32) -> Option<&mut Controller> {
        if self.meta.id == id {
            return Some(self);
        }
        self.parts.iter_mut().find(|p| p.meta.id == id)
    }
    /// Drops Pad FX timbres whose kit or pad is gone, and duplicates, then
    /// routes each kit's pads to the chains that remain. What finds no queue
    /// slot waits for the next request.
    pub(super) fn sync_chains(&mut self) {
        let mut seen = std::collections::BTreeSet::new();
        let orphans: Vec<u32> = self
            .parts
            .iter()
            .filter(|p| p.meta.kind == Kind::PadFx)
            .filter(|p| {
                let kept = p
                    .meta
                    .owner
                    .and_then(|o| self.timbre(o))
                    .filter(|o| o.plays_kit())
                    .and_then(|o| o.preset.sampler.as_ref())
                    .is_some_and(|kit| kit.pad(p.meta.pad).is_some());
                !(kept && seen.insert((p.meta.owner, p.meta.pad)))
            })
            .map(|p| p.meta.id)
            .collect();
        for id in orphans {
            if self.enqueue(Command::RemovePart(id, None)).is_ok() {
                self.parts.retain(|p| p.meta.id != id);
                if self.active == id {
                    self.active = self.shown();
                }
                self.scene_revision = self.scene_revision.wrapping_add(1);
            }
        }
        let routes = |owner: u32| -> Vec<u32> {
            self.parts
                .iter()
                .filter(|p| p.meta.kind == Kind::PadFx && p.meta.owner == Some(owner))
                .map(|p| p.meta.pad)
                .collect()
        };
        let owners: Vec<(u32, Vec<u32>)> = std::iter::once(&*self)
            .chain(&self.parts)
            .filter(|p| p.plays_kit())
            .map(|p| (p.meta.id, routes(p.meta.id)))
            .collect();
        for (id, routes) in owners {
            let owner = self.timbre_mut(id).unwrap();
            if owner.pad_routes != routes || owner.kit_pending {
                owner.pad_routes = routes;
                owner.update_kit();
            }
        }
    }
    /// The Pad FX timbre of a pad of the selected kit, made if needed.
    fn open_chain(&mut self, pad: u32) -> Result<(), String> {
        let owner = self.active_part()?;
        let settings = owner
            .preset
            .sampler
            .as_deref()
            .filter(|_| owner.plays_kit())
            .ok_or("Select a Sampler timbre")?;
        let position = settings
            .pads
            .iter()
            .position(|p| p.id == pad)
            .ok_or("Unknown pad")?;
        let name = match settings.pads[position].name.as_str() {
            "" => format!("{} FX", pad_label(position)),
            name => format!("{name} FX"),
        };
        let (owner_id, playing) = (owner.meta.id, owner.playing);
        if let Some(existing) = self.parts.iter().find(|p| {
            p.meta.kind == Kind::PadFx && p.meta.owner == Some(owner_id) && p.meta.pad == pad
        }) {
            self.active = existing.meta.id;
            return Ok(());
        }
        let id = self.next_timbre_id;
        let next_id = id.checked_add(1).ok_or("Timbre identity overflow")?;
        let mut chain = self.routing_part(Kind::PadFx, id, 0);
        chain.meta.name = name;
        chain.meta.owner = Some(owner_id);
        chain.meta.pad = pad;
        chain.playing = playing;
        let audio = chain.prepare(self.rate);
        self.enqueue(Command::AddChain(
            owner_id,
            Some(Box::new(timbres::PartAudio { audio, next: None })),
        ))?;
        // After its Sampler and the Sampler's other chains.
        let start = self
            .parts
            .iter()
            .position(|p| p.meta.id == owner_id)
            .map_or(0, |i| i + 1);
        let at = (start..self.parts.len())
            .find(|&i| self.parts[i].meta.owner != Some(owner_id))
            .unwrap_or(self.parts.len());
        self.parts.insert(at, chain);
        self.next_timbre_id = next_id;
        self.active = id;
        Ok(())
    }

    /// Kit uploads, pad clicks and the Sample editor, as the browser answers
    /// them; recording takes stays a browser feature.
    pub(super) fn sampler_request(
        &mut self,
        path: &str,
        post: bool,
        data: &Value,
    ) -> Result<Value, String> {
        let ok = || json!({"ok":true});
        let rate = self.rate;
        match (path, post) {
            ("/api/sampler/chain", true) => {
                let pad = data["pad"].as_u64().ok_or("Choose a pad")? as u32;
                self.open_chain(pad)?;
                self.selection.dirty = true;
                self.offer_workers();
                self.scene_revision = self.scene_revision.wrapping_add(1);
                Ok(json!(self.context()))
            }
            ("/api/sampler/upload", true) => {
                let id = kits::sample_id(data["id"].as_str().unwrap_or(""));
                if id.is_empty() {
                    return Err("Missing sample id".into());
                }
                let part = data["part"].as_u64().ok_or("Unknown Sampler timbre")? as u32;
                let bytes = decode_bytes(data["wav"].as_str().ok_or("Missing audio")?)?;
                let sample = wav::decode(Cursor::new(bytes), "Sample".into())?;
                let frames = sample.len();
                let target = self
                    .timbre_mut(part)
                    .filter(|p| p.plays_kit())
                    .ok_or("Unknown Sampler timbre")?;
                target.kit.insert(id.clone(), Arc::new(sample));
                target.update_kit();
                self.selection.dirty = true;
                Ok(json!({"id":id,"frames":frames}))
            }
            ("/api/sampler/sample.wav", true) => {
                let part = data["part"].as_u64().unwrap_or(u64::MAX) as u32;
                let id = data["id"].as_str().unwrap_or("");
                let sample = self
                    .timbre(part)
                    .and_then(|p| p.kit.get(id))
                    .ok_or("The kit has no such sample")?;
                binary(sample.sample_rate, sample.frames())
            }
            ("/api/sampler/state", _) => {
                let part = self.active_part()?;
                part.drain();
                let loaded = if part.plays_kit() {
                    part.kit.keys().cloned().collect()
                } else {
                    Vec::new()
                };
                let (id, view, playing) =
                    (part.meta.id, part.kit_view.view(loaded), part.view.playing);
                let chains: Vec<Value> = self
                    .parts
                    .iter()
                    .filter(|p| p.meta.kind == Kind::PadFx && p.meta.owner == Some(id))
                    .map(|p| json!({"pad":p.meta.pad,"id":p.meta.id,"name":p.meta.name}))
                    .collect();
                Ok(json!({"view":view,"chains":chains,"rate":rate,"playing":playing}))
            }
            ("/api/sampler/trigger", true) => {
                let part = self.active_part()?;
                if !part.plays_kit() {
                    return Err("Select a Sampler timbre".into());
                }
                part.enqueue(Command::Pad {
                    pad: data["pad"].as_u64().ok_or("Choose a pad")? as u32,
                    note: data["note"].as_u64().map(|n| n.min(127) as u8),
                    velocity: data["velocity"].as_u64().unwrap_or(110).clamp(1, 127) as u8,
                    off: data["off"] == true,
                })?;
                Ok(ok())
            }
            ("/api/sampler/forget", true) => {
                let ids: Vec<String> =
                    serde_json::from_value(data["ids"].clone()).map_err(|e| e.to_string())?;
                let part = self.active_part()?;
                if !part.plays_kit() {
                    return Err("Select a Sampler timbre".into());
                }
                for id in ids {
                    part.kit.remove(&id);
                }
                part.update_kit();
                Ok(ok())
            }
            ("/api/sampler/process", true) => {
                let request = &data["request"];
                let audio = if request["op"] == "paste" {
                    edit_audio(&data["wav"])?.paste(
                        &edit_audio(&data["clip"])?,
                        request["at"].as_u64().unwrap_or(0) as u32,
                        request["mix"] == true,
                    )?
                } else {
                    let operation: edit::Operation =
                        serde_json::from_value(request.clone()).map_err(|e| e.to_string())?;
                    edit_audio(&data["wav"])?.apply(operation)?
                };
                binary(audio.rate, &audio.frames)
            }
            ("/api/sampler/analyze", true) => {
                let request = &data["request"];
                let audio = edit_audio(&data["wav"])?;
                let number = |key: &str, default: f64| request[key].as_f64().unwrap_or(default);
                Ok(match request["kind"].as_str().unwrap_or("") {
                    "transients" => json!(edit::transients(
                        &audio.frames,
                        audio.rate,
                        number("sensitivity", 0.5) as f32,
                        number("gap", 60.0) as f32,
                    )),
                    "divide" => json!(edit::divide(
                        audio.frames.len(),
                        number("count", 8.0) as usize
                    )),
                    "beats" => json!(edit::beats(
                        audio.frames.len(),
                        audio.rate,
                        number("bpm", 120.0) as f32,
                        number("beats", 1.0) as f32,
                    )),
                    "zero" => {
                        let at = number("at", 0.0).max(0.0) as usize;
                        let radius = (audio.rate as usize / 100).max(1);
                        json!(edit::zero_crossing(
                            &audio.frames,
                            at.min(audio.frames.len() - 1),
                            radius
                        ))
                    }
                    "root" => json!(edit::root(&audio.frames, audio.rate)),
                    "info" => {
                        json!({"rate":audio.rate,"frames":audio.frames.len(),"peak":audio.peak()})
                    }
                    _ => return Err("Unknown analysis".into()),
                })
            }
            ("/api/sampler/bounce", true) => {
                let part = data["part"].as_u64().ok_or("Choose a Sampler timbre")? as u32;
                let pad = data["pad"].as_u64().ok_or("Choose a pad")? as u32;
                let note = data["note"].as_u64().ok_or("Choose a note")?.min(127) as u8;
                let mut options: BounceOptions =
                    serde_json::from_value(data["options"].clone()).unwrap_or_default();
                options.sanitize();
                let owner = self
                    .timbre(part)
                    .filter(|p| p.plays_kit())
                    .ok_or("Unknown Sampler timbre")?;
                let settings = owner.preset.sampler.as_deref().ok_or("Not a kit")?;
                let chain = self
                    .parts
                    .iter()
                    .find(|p| {
                        p.meta.kind == Kind::PadFx
                            && p.meta.owner == Some(part)
                            && p.meta.pad == pad
                    })
                    .map(|p| &p.preset);
                let frames =
                    kits::bounce::note(rate, settings, &owner.kit, pad, chain, &options, note)?;
                binary(rate, &frames)
            }
            ("/api/sampler/record", false) => {
                Ok(json!({"active":false,"armed":false,"seconds":0,"peak":0,"source":null}))
            }
            ("/api/sampler/record", true) => {
                Err("Sampler takes record in the browser version for now.".into())
            }
            _ => Err("Unknown sampler command".into()),
        }
    }
}
