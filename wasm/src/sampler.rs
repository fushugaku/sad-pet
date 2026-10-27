//! Sampler timbres in the browser. The worklet keeps each kit's sample pool
//! next to its engine and runs every pad's Pad FX timbre between the pad's
//! voices and the kit; the utility worker prepares samples, makes the
//! destructive edits and analysis of the Sample editor and bounces pads on
//! its own copy of the scene, off the audio thread.
use super::*;
use sad_pet::{
    sampler::{
        self, BounceOptions, Settings,
        edit::{self, Audio, Operation},
    },
    timbres::{Kind, Meta, SendGains},
};

/// "A01" for the first pad of bank A.
fn pad_label(index: usize) -> String {
    let bank = (b'A' + (index / sampler::BANK_PADS).min(25) as u8) as char;
    format!("{bank}{:02}", index % sampler::BANK_PADS + 1)
}

/// A Sampler timbre, or a Rack with a Sampler module: it has a kit, a pool
/// and Pad FX timbres.
pub(super) fn plays_kit(part: &super::timbres::Part) -> bool {
    matches!(part.meta.kind, Kind::Sampler | Kind::Rack)
        && part.host.engine.preset.sampler.is_some()
}

impl super::timbres::Registry {
    /// Each kit, its Pad FX timbres and their pads: (owner, chain, pad).
    pub fn chain_links(&self) -> Vec<(usize, usize, u32)> {
        let mut links = Vec::new();
        for (c, part) in self.parts.iter().enumerate() {
            if part.meta.kind != Kind::PadFx {
                continue;
            }
            if let Some(o) = self
                .parts
                .iter()
                .position(|p| Some(p.meta.id) == part.meta.owner && plays_kit(p))
            {
                links.push((o, c, part.meta.pad));
            }
        }
        links
    }
    /// Pads with a Pad FX timbre send their voices to it, unless bounced.
    pub fn sync_sampler_routes(&mut self) {
        let links = self.chain_links();
        for (o, part) in self.parts.iter_mut().enumerate() {
            if plays_kit(part) {
                let routes: Vec<u32> = links.iter().filter(|l| l.0 == o).map(|l| l.2).collect();
                part.host.engine.set_sampler_routes(&routes);
            }
        }
    }
    /// Drops Pad FX timbres whose Sampler or pad is gone, and duplicates.
    /// Returns whether any went.
    pub fn prune_chains(&mut self) -> bool {
        let mut seen = std::collections::BTreeSet::new();
        let keep: Vec<bool> = self
            .parts
            .iter()
            .map(|part| {
                if part.meta.kind != Kind::PadFx {
                    return true;
                }
                let owner = self
                    .parts
                    .iter()
                    .find(|p| Some(p.meta.id) == part.meta.owner && plays_kit(p));
                owner.is_some_and(|o| {
                    o.host
                        .engine
                        .preset
                        .sampler
                        .as_ref()
                        .is_some_and(|s| s.pad(part.meta.pad).is_some())
                }) && seen.insert((part.meta.owner, part.meta.pad))
            })
            .collect();
        if keep.iter().all(|k| *k) {
            return false;
        }
        let active = self.parts[self.selected].meta.id;
        let mut i = 0;
        self.parts.retain(|_| {
            i += 1;
            keep[i - 1]
        });
        self.selected = self
            .parts
            .iter()
            .position(|p| p.meta.id == active)
            .unwrap_or(0);
        self.target = None;
        true
    }
    /// The Pad FX timbre of a pad of the selected Sampler, made if needed.
    fn open_chain(&mut self, pad: u32) -> Result<(), String> {
        let index = self.index();
        let owner = &self.parts[index];
        let settings = owner
            .host
            .engine
            .preset
            .sampler
            .as_deref()
            .filter(|_| plays_kit(owner))
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
        let owner_id = owner.meta.id;
        if let Some(existing) = self.parts.iter().position(|p| {
            p.meta.kind == Kind::PadFx && p.meta.owner == Some(owner_id) && p.meta.pad == pad
        }) {
            self.selected = existing;
            return Ok(());
        }
        let id = self.next_id;
        self.next_id = id.checked_add(1).ok_or("Timbre identity overflow")?;
        let rate = self.parts[0].host.rate;
        let mut host = Host::new(rate);
        host.engine.set_preset(Kind::PadFx.preset().unwrap());
        host.engine
            .set_playing(self.parts[index].host.engine.playing);
        host.resize_snapshot();
        // After its Sampler and the Sampler's other chains.
        let at = (index + 1..self.parts.len())
            .find(|&i| self.parts[i].meta.owner != Some(owner_id))
            .unwrap_or(self.parts.len());
        self.parts.insert(
            at,
            super::timbres::Part {
                meta: Meta {
                    id,
                    name,
                    kind: Kind::PadFx,
                    owner: Some(owner_id),
                    pad,
                    ..Meta::default()
                },
                host,
                gain: 1.,
                sends: SendGains::default(),
                vocal: None,
            },
        );
        self.selected = at;
        Ok(())
    }

    pub fn sampler_command(&mut self, path: &str, data: &Value) -> Result<Value, String> {
        let index = self.index();
        if path == "/api/sampler/chain" {
            let pad = data["pad"].as_u64().ok_or("Choose a pad")? as u32;
            self.open_chain(pad)?;
            self.target = None;
            self.revision = self.revision.wrapping_add(1);
            self.refresh_context();
            return Ok(json!(self.context()));
        }
        let part = &mut self.parts[index];
        let sampler = |part: &super::timbres::Part| {
            if plays_kit(part) {
                Ok(())
            } else {
                Err("Select a Sampler timbre".to_string())
            }
        };
        let result = match path {
            "/api/sampler/settings" => {
                sampler(part)?;
                let mut settings: Settings =
                    serde_json::from_value(data["settings"].clone()).map_err(|e| e.to_string())?;
                settings.sanitize();
                part.host.engine.set_sampler(settings);
                part.host.config_revision = part.host.config_revision.wrapping_add(1);
                part.host.resize_snapshot();
                let settings = json!(part.host.engine.preset.sampler);
                if self.prune_chains() {
                    self.revision = self.revision.wrapping_add(1);
                    self.refresh_context();
                }
                self.sync_sampler_routes();
                json!({ "ok": true, "settings": settings })
            }
            "/api/sampler/trigger" => {
                sampler(part)?;
                let pad = data["pad"].as_u64().ok_or("Choose a pad")? as u32;
                if data["off"] == true {
                    part.host.engine.release_pad(pad);
                } else {
                    let note = data["note"].as_u64().map(|n| n.min(127) as u8);
                    let velocity = data["velocity"].as_u64().unwrap_or(110).clamp(1, 127) as u8;
                    part.host.engine.trigger_pad(pad, note, velocity);
                }
                json!({ "ok": true })
            }
            "/api/sampler/state" => {
                let view = part.host.engine.sampler_view();
                let id = part.meta.id;
                let chains: Vec<Value> = self
                    .parts
                    .iter()
                    .filter(|p| p.meta.kind == Kind::PadFx && p.meta.owner == Some(id))
                    .map(|p| json!({ "pad": p.meta.pad, "id": p.meta.id, "name": p.meta.name }))
                    .collect();
                json!({ "view": view, "chains": chains, "rate": self.parts[0].host.rate,
                    "playing": self.parts[index].host.engine.playing })
            }
            "/api/sampler/forget" => {
                let ids: Vec<String> =
                    serde_json::from_value(data["ids"].clone()).map_err(|e| e.to_string())?;
                if let Some(pool) = part.host.engine.sampler_pool_mut() {
                    for id in ids {
                        pool.remove(&id);
                    }
                }
                part.host.engine.refresh_sampler();
                json!({ "ok": true })
            }
            _ => return Err("Unknown sampler command".into()),
        };
        Ok(result)
    }
}

/// Adds a prepared sample to the pool of Sampler `part`: `id_length` bytes
/// of pool id, then the SADPCM01 transfer of `sad_sampler_prepare`.
#[unsafe(no_mangle)]
pub extern "C" fn sad_sampler_sample(part: u32, id_length: usize) -> u32 {
    registry(|r| {
        let index = r.index();
        let input = std::mem::take(&mut r.parts[index].host.input);
        let result = (|| {
            let id = sampler::sample_id(
                std::str::from_utf8(input.get(..id_length).ok_or("Missing sample id")?)
                    .map_err(|e| e.to_string())?,
            );
            if id.is_empty() {
                return Err("Missing sample id".to_string());
            }
            let sample = super::unpack(&input[id_length..])?;
            let target = r
                .parts
                .iter_mut()
                .find(|p| p.meta.id == part && plays_kit(p))
                .ok_or("Unknown Sampler timbre")?;
            let pool = target
                .host
                .engine
                .sampler_pool_mut()
                .ok_or("Not a Sampler timbre")?;
            let frames = sample.len();
            pool.insert(id.clone(), Arc::new(sample));
            target.host.engine.refresh_sampler();
            Ok(serde_json::to_vec(&json!({ "id": id, "frames": frames })).unwrap())
        })();
        r.parts[index].host.finish(result)
    })
}

fn decode(bytes: &[u8]) -> Result<Audio, String> {
    let (rate, frames) = wav::frames(Cursor::new(bytes))?;
    if frames.is_empty() {
        return Err("The audio file is empty".into());
    }
    Ok(Audio::new(rate, frames))
}
/// A JSON header of `header` bytes, then the audio.
fn split(input: &[u8], header: usize) -> Result<(Value, &[u8]), String> {
    let head = input.get(..header).ok_or("Missing header")?;
    Ok((
        serde_json::from_slice(head).map_err(|e| e.to_string())?,
        &input[header..],
    ))
}

/// A WAV of any rate and bit depth as a playable, prepared sample.
#[unsafe(no_mangle)]
pub extern "C" fn sad_sampler_prepare() -> u32 {
    host(|h| {
        let result = wav::decode(Cursor::new(&h.input), "Sample".into()).map(|s| super::pack(&s));
        h.finish(result)
    })
}

/// One destructive edit: a JSON `Operation` (or `{op: "paste", at, mix,
/// clip}` with the clip's byte length) and the WAV, then the clip's WAV.
#[unsafe(no_mangle)]
pub extern "C" fn sad_sampler_process(header: usize) -> u32 {
    host(|h| {
        let result = (|| {
            let (request, bytes) = split(&h.input, header)?;
            let audio = if request["op"] == "paste" {
                let clip = request["clip"].as_u64().ok_or("Missing clip")? as usize;
                let split = bytes.len().checked_sub(clip).ok_or("Incomplete clip")?;
                let target = decode(&bytes[..split])?;
                let clip = decode(&bytes[split..])?;
                target.paste(
                    &clip,
                    request["at"].as_u64().unwrap_or(0) as u32,
                    request["mix"] == true,
                )?
            } else {
                let operation: Operation =
                    serde_json::from_value(request).map_err(|e| e.to_string())?;
                decode(bytes)?.apply(operation)?
            };
            wav::bytes(audio.rate, &audio.frames)
        })();
        h.finish(result)
    })
}

/// Slices, the root note or a zero crossing of a WAV, as JSON.
#[unsafe(no_mangle)]
pub extern "C" fn sad_sampler_analyze(header: usize) -> u32 {
    host(|h| {
        let result = (|| {
            let (request, bytes) = split(&h.input, header)?;
            let audio = decode(bytes)?;
            let number = |key: &str, default: f64| request[key].as_f64().unwrap_or(default);
            let value = match request["kind"].as_str().unwrap_or("") {
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
                    json!({ "rate": audio.rate, "frames": audio.frames.len(), "peak": audio.peak() })
                }
                _ => return Err("Unknown analysis".to_string()),
            };
            serde_json::to_vec(&value).map_err(|e| e.to_string())
        })();
        h.finish(result)
    })
}

/// Renders one note of a pad on the worker's copy of the scene:
/// `{part, pad, note, options}`. The output is a WAV.
#[unsafe(no_mangle)]
pub extern "C" fn sad_sampler_bounce() -> u32 {
    registry(|r| {
        let index = r.index();
        let result = (|| {
            let request: Value =
                serde_json::from_slice(&r.parts[index].host.input).map_err(|e| e.to_string())?;
            let part = request["part"].as_u64().ok_or("Choose a Sampler timbre")? as u32;
            let pad = request["pad"].as_u64().ok_or("Choose a pad")? as u32;
            let note = request["note"].as_u64().ok_or("Choose a note")?.min(127) as u8;
            let mut options: BounceOptions =
                serde_json::from_value(request["options"].clone()).unwrap_or_default();
            options.sanitize();
            let owner = r
                .parts
                .iter()
                .find(|p| p.meta.id == part && plays_kit(p))
                .ok_or("Unknown Sampler timbre")?;
            let settings = owner
                .host
                .engine
                .preset
                .sampler
                .as_deref()
                .ok_or("Not a kit")?;
            let pool = owner.host.engine.sampler_pool().ok_or("Not a kit")?;
            let chain = r
                .parts
                .iter()
                .find(|p| {
                    p.meta.kind == Kind::PadFx && p.meta.owner == Some(part) && p.meta.pad == pad
                })
                .map(|p| &p.host.engine.preset);
            let rate = r.parts[0].host.rate;
            let frames = sampler::bounce::note(rate, settings, pool, pad, chain, &options, note)?;
            wav::bytes(rate, &frames)
        })();
        r.parts[index].host.finish(result)
    })
}

/// The WAV of a pool sample of Sampler `part`; the input is its id.
#[unsafe(no_mangle)]
pub extern "C" fn sad_sampler_wav(part: u32) -> u32 {
    registry(|r| {
        let index = r.index();
        let result = (|| {
            let id = String::from_utf8_lossy(&r.parts[index].host.input).into_owned();
            let sample = r
                .parts
                .iter()
                .find(|p| p.meta.id == part)
                .and_then(|p| p.host.engine.sampler_pool())
                .and_then(|pool| pool.get(&id))
                .ok_or("The kit has no such sample")?;
            wav::bytes(sample.sample_rate, sample.frames())
        })();
        r.parts[index].host.finish(result)
    })
}
