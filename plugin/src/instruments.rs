//! Instrument presets: one timbre's sound with its sample, filed by the user
//! in categories and shared by every native instance. Loading one replaces a
//! timbre's sound in place; the other timbres play on. No file work runs on Audio.
use super::preset_library::{name, timestamp, unique_id, valid_id};
use super::{BASE64, Command, Controller, MAX_BYTES, SavedState, Source, Transport};
use base64::Engine as _;
use flate2::{Compression, read::GzDecoder, write::GzEncoder};
use sad_pet::{
    dsp::SampleBuffer,
    timbres::{Kind, Meta},
    wav,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    fs::{self, File},
    io::{Cursor, Read},
    path::PathBuf,
    sync::Arc,
};

const MAGIC: &[u8; 8] = b"SADINS\x01\0";
const MAX_HEADER: usize = 4096;
const MAX_STATE: u64 = 256 * 1024 * 1024;
const WRONG_FILE: &str = "Use a SAD PET instrument preset (.sadinst).";

#[derive(Clone, Serialize, Deserialize)]
struct Record {
    id: String,
    name: String,
    /// Empty for presets in no category.
    #[serde(default)]
    category: String,
    kind: Kind,
    created_at: u64,
    modified_at: u64,
    #[serde(default = "gzip")]
    codec: String,
}
fn gzip() -> String {
    "gzip".into()
}
/// A category typed by the user; empty leaves the preset in no category.
fn category(value: &str) -> Result<String, String> {
    let category = value.trim();
    if category.chars().count() > 40 || category.chars().any(char::is_control) {
        return Err("Use a category name of up to 40 characters.".into());
    }
    Ok(category.into())
}
fn same(a: &str, b: &str) -> bool {
    a.to_lowercase() == b.to_lowercase()
}
fn read_header(input: &mut impl Read) -> Result<Record, String> {
    let mut magic = [0; 8];
    input.read_exact(&mut magic).map_err(|_| WRONG_FILE)?;
    if &magic != MAGIC {
        return Err(WRONG_FILE.into());
    }
    let mut size = [0; 4];
    input
        .read_exact(&mut size)
        .map_err(|_| "The instrument preset is incomplete.")?;
    let size = u32::from_le_bytes(size) as usize;
    if size == 0 || size > MAX_HEADER {
        return Err("The instrument preset header is invalid.".into());
    }
    let mut bytes = vec![0; size];
    input
        .read_exact(&mut bytes)
        .map_err(|_| "The instrument preset is incomplete.")?;
    let record: Record =
        serde_json::from_slice(&bytes).map_err(|_| "The instrument preset header is invalid.")?;
    name(&record.name)?;
    category(&record.category)?;
    if !valid_id(&record.id) || record.codec != "gzip" || record.kind.routing() {
        return Err("Unsupported instrument preset format.".into());
    }
    Ok(record)
}
/// The header and the compressed state after it.
fn split(bytes: &[u8]) -> Result<(Record, &[u8]), String> {
    if bytes.len() > MAX_BYTES {
        return Err("The instrument preset exceeds 128 MB.".into());
    }
    let mut input = Cursor::new(bytes);
    let record = read_header(&mut input)?;
    Ok((record, &bytes[input.position() as usize..]))
}
fn decode(bytes: &[u8]) -> Result<(Record, SavedState), String> {
    let (record, body) = split(bytes)?;
    let mut decoded = Vec::new();
    GzDecoder::new(body)
        .take(MAX_STATE + 1)
        .read_to_end(&mut decoded)
        .map_err(|_| "The instrument preset is damaged or incomplete.")?;
    if decoded.len() as u64 > MAX_STATE {
        return Err("The instrument preset is too large.".into());
    }
    let state = serde_json::from_slice(&decoded).map_err(|_| "The instrument data is invalid.")?;
    Ok((record, state))
}
/// `record`'s header before an already compressed state.
fn framed(record: &Record, body: &[u8]) -> Result<Vec<u8>, String> {
    let header = serde_json::to_vec(record).map_err(|e| e.to_string())?;
    if header.len() > MAX_HEADER {
        return Err("The instrument preset metadata is too large.".into());
    }
    let mut file = Vec::with_capacity(12 + header.len() + body.len());
    file.extend_from_slice(MAGIC);
    file.extend_from_slice(&(header.len() as u32).to_le_bytes());
    file.extend_from_slice(&header);
    file.extend_from_slice(body);
    if file.len() > MAX_BYTES {
        return Err("The instrument preset exceeds 128 MB.".into());
    }
    Ok(file)
}
fn encode(record: &Record, state: &SavedState) -> Result<Vec<u8>, String> {
    // Level 1: the sample is base64 text that higher levels barely shrink.
    let mut compressed = GzEncoder::new(Vec::new(), Compression::fast());
    serde_json::to_writer(&mut compressed, state).map_err(|e| e.to_string())?;
    framed(record, &compressed.finish().map_err(|e| e.to_string())?)
}
/// One playable timbre with its sample, ready to load: its engine type and
/// decoded sample.
fn check(state: &mut SavedState) -> Result<(Kind, SampleBuffer), String> {
    if state.version != 1 || state.preset.format != "evil-pet-rust-v1" || !state.timbres.is_empty()
    {
        return Err(WRONG_FILE.into());
    }
    state.preset.sanitize();
    let kind = Kind::of(&state.preset);
    if kind.routing() {
        return Err(WRONG_FILE.into());
    }
    let bytes = BASE64
        .decode(&state.sample_wav)
        .map_err(|_| "The instrument's sample is invalid.")?;
    if bytes.len() > MAX_BYTES {
        return Err("The instrument's sample exceeds 128 MB.".into());
    }
    let mut sample = wav::decode(Cursor::new(bytes), state.sample_name.clone())?;
    sample.library_id = state.sample_id.clone();
    state.region.bounds(sample.len())?;
    state.fades.validate()?;
    Ok((kind, sample))
}

impl Controller {
    fn instrument_root(&self) -> Result<PathBuf, String> {
        Ok(self.library.root()?.join("Instruments"))
    }
    fn instrument_path(&self, id: &str) -> Result<PathBuf, String> {
        if !valid_id(id) {
            return Err("Unknown instrument preset.".into());
        }
        Ok(self.instrument_root()?.join(format!("{id}.sadinst")))
    }
    /// Every instrument, by category and then name.
    fn instruments(&self) -> Result<Vec<Record>, String> {
        let root = self.instrument_root()?;
        if !root.exists() {
            return Ok(Vec::new());
        }
        let mut records = Vec::new();
        for entry in
            fs::read_dir(root).map_err(|e| format!("Cannot open the instrument library: {e}"))?
        {
            let path = entry.map_err(|e| e.to_string())?.path();
            if path.extension().is_none_or(|e| e != "sadinst") {
                continue;
            }
            if let Ok(mut file) = File::open(&path)
                && let Ok(record) = read_header(&mut file)
                && path.file_stem().and_then(|n| n.to_str()) == Some(&record.id)
            {
                records.push(record);
            }
        }
        records.sort_by(|a, b| {
            (a.category.to_lowercase(), a.name.to_lowercase(), &a.id).cmp(&(
                b.category.to_lowercase(),
                b.name.to_lowercase(),
                &b.id,
            ))
        });
        Ok(records)
    }
    fn instrument_bytes(&self, id: &str) -> Result<Vec<u8>, String> {
        let file = File::open(self.instrument_path(id)?)
            .map_err(|_| "This instrument is no longer in the library.")?;
        let mut bytes = Vec::new();
        file.take(MAX_BYTES as u64 + 1)
            .read_to_end(&mut bytes)
            .map_err(|e| e.to_string())?;
        if bytes.len() > MAX_BYTES {
            return Err("The instrument preset exceeds 128 MB.".into());
        }
        Ok(bytes)
    }
    fn write_instrument(&self, bytes: &[u8], id: &str) -> Result<(), String> {
        fs::create_dir_all(self.instrument_root()?)
            .map_err(|e| format!("Cannot create the instrument folder: {e}"))?;
        super::audio_library::write_file(&self.instrument_path(id)?, bytes)
    }
    /// Names are unique within a category.
    fn check_free(records: &[Record], record: &Record) -> Result<(), String> {
        if records.iter().any(|r| {
            r.id != record.id && same(&r.category, &record.category) && same(&r.name, &record.name)
        }) {
            return Err(if record.category.is_empty() {
                format!("An instrument named “{}” already exists.", record.name)
            } else {
                format!(
                    "“{}” already has an instrument named “{}”.",
                    record.category, record.name
                )
            });
        }
        Ok(())
    }
    /// The active timbre's sound, as an instrument preset keeps it.
    fn active_sound(&mut self) -> Result<SavedState, String> {
        let active = self.active;
        let mut state = if active == 0 {
            // The scene's own timbre, without the other parts.
            let parts = std::mem::take(&mut self.parts);
            let state = self.snapshot();
            self.parts = parts;
            state?
        } else {
            self.parts
                .iter_mut()
                .find(|p| p.meta.id == active)
                .ok_or("Unknown timbre")?
                .snapshot()?
        };
        if state.meta.kind.routing() {
            return Err(
                "Choose an instrument timbre to save. Bus and Master hold no instrument.".into(),
            );
        }
        state.meta = Meta {
            name: state.meta.name.clone(),
            kind: state.meta.kind,
            ..Meta::default()
        };
        state.timbres.clear();
        state.active_timbre = 0;
        state.source = Source::File;
        state.sync = true;
        state.frozen = false;
        state.bypass = false;
        state.playing = false;
        state.selection = Default::default();
        Ok(state)
    }
    /// Gives timbre `id` the sound in `state`, named `title`. It keeps its
    /// place, mix and transport; the scene's own timbre also keeps the
    /// scene's name.
    fn load_sound(
        &mut self,
        id: u32,
        mut state: SavedState,
        kind: Kind,
        sample: SampleBuffer,
        title: &str,
    ) -> Result<(), String> {
        if id == 0 {
            if self.meta.kind.routing() {
                return Err(
                    "Choose an instrument timbre, or load the instrument as a new timbre.".into(),
                );
            }
            // It never becomes a Vocal timbre, nor stops being one; a Rack's
            // Vocal module comes and goes with the preset below.
            if kind == Kind::Vocal || self.meta.kind == Kind::Vocal {
                return Err("Load Vocal instruments as a new timbre.".into());
            }
            state.preset.name = self.preset.name.clone();
            let kit = super::sampler::decode_kit(&state.kit_samples)?;
            let sample = Arc::new(sample);
            let source = self.prepared_source_for(
                sample.clone(),
                Source::File,
                state.region,
                state.fades,
                &state.preset,
            );
            // The scene routes the kit's pads to its Pad FX timbres after.
            self.enqueue(Command::Preset(super::sampler::prepared_with(
                &state.preset,
                &kit,
                &[],
                self.rate,
            )))?;
            self.enqueue(Command::Source(Box::new(source)))?;
            self.enqueue(Command::Transport(Transport {
                midi_gate: Some(state.midi_gate),
                frozen: Some(false),
                ..Default::default()
            }))?;
            self.meta.kind = kind;
            self.meta.name = title.into();
            self.meta.sanitize();
            self.preset = state.preset;
            self.kit = kit;
            self.pad_routes.clear();
            self.sample = sample;
            self.source = Source::File;
            self.region = state.region;
            self.fades = state.fades;
            self.frozen = false;
            self.midi_gate = state.midi_gate;
            return self.update_vocal();
        }
        let index = self
            .parts
            .iter()
            .position(|p| p.meta.id == id)
            .ok_or("Unknown timbre")?;
        let old = &self.parts[index];
        if old.meta.kind.routing() {
            return Err(
                "Choose an instrument timbre, or load the instrument as a new timbre.".into(),
            );
        }
        // A new part takes the sound, so a Vocal voice comes or goes with it.
        let mut part = self.new_part();
        part.meta = Meta {
            kind,
            name: title.into(),
            ..old.meta.clone()
        };
        part.meta.sanitize();
        part.rate = self.rate;
        state.meta = part.meta.clone();
        state.preset.name = part.meta.name.clone();
        state.source = Source::File;
        state.sync = self.sync;
        state.playing = self.playing;
        state.frozen = false;
        state.bypass = old.bypass;
        // A Vocal timbre's score, or a Rack's, leaves with it.
        let sang = old.vocal.is_some();
        part.restore_saved(state)?;
        if sang {
            self.voice_service().forget(id);
        }
        let mut audio = part.prepare(self.rate);
        // Its Pad FX timbres move into the new sound's audio.
        audio.chains = super::timbres::chain_audio(&mut self.parts, id, self.rate);
        self.enqueue(Command::RemovePart(id, None))?;
        self.enqueue(Command::AddPart(Some(Box::new(
            super::timbres::PartAudio { audio, next: None },
        ))))?;
        self.parts[index] = part;
        Ok(())
    }

    pub(super) fn instruments_request(
        &mut self,
        path: &str,
        post: bool,
        data: &Value,
    ) -> Result<Value, String> {
        let id = || {
            data["id"]
                .as_str()
                .filter(|id| valid_id(id))
                .ok_or("Choose an instrument preset.")
        };
        match (path, post) {
            ("/api/instruments", false) => Ok(json!(self.instruments()?)),
            ("/api/instruments/save", true) => {
                let records = self.instruments()?;
                let existing = match data["id"].as_str() {
                    Some(id) => Some(
                        records
                            .iter()
                            .find(|r| r.id == id)
                            .ok_or("This instrument was removed. Save it again.")?,
                    ),
                    None => None,
                };
                let state = self.active_sound()?;
                let record = Record {
                    id: existing.map_or_else(unique_id, |r| r.id.clone()),
                    name: name(data["name"].as_str().unwrap_or(""))?,
                    category: category(data["category"].as_str().unwrap_or(""))?,
                    kind: state.meta.kind,
                    created_at: existing.map_or_else(timestamp, |r| r.created_at),
                    modified_at: timestamp(),
                    codec: gzip(),
                };
                Self::check_free(&records, &record)?;
                self.write_instrument(&encode(&record, &state)?, &record.id)?;
                Ok(json!(record))
            }
            ("/api/instruments/load", true) => {
                let (record, mut state) = decode(&self.instrument_bytes(id()?)?)?;
                let (kind, sample) = check(&mut state)?;
                if data["new"] == true {
                    self.scene_request(&json!({
                        "path": "/api/timbres/add", "method": "POST", "data": {"kind": kind}
                    }))?;
                }
                self.load_sound(self.active, state, kind, sample, &record.name)?;
                self.selection.dirty = true;
                self.offer_workers();
                self.scene_revision = self.scene_revision.wrapping_add(1);
                Ok(json!(self.context()))
            }
            ("/api/instruments/edit", true) => {
                let records = self.instruments()?;
                let bytes = self.instrument_bytes(id()?)?;
                let (mut record, body) = split(&bytes)?;
                record.name = name(data["name"].as_str().unwrap_or(""))?;
                record.category = category(data["category"].as_str().unwrap_or(""))?;
                record.modified_at = timestamp();
                Self::check_free(&records, &record)?;
                self.write_instrument(&framed(&record, body)?, &record.id)?;
                Ok(json!(record))
            }
            ("/api/instruments/category", true) => {
                let from = data["from"].as_str().ok_or("Choose a category.")?;
                let to = category(data["to"].as_str().unwrap_or(""))?;
                let records = self.instruments()?;
                let moving: Vec<&Record> = records.iter().filter(|r| r.category == from).collect();
                let staying: Vec<Record> = records
                    .iter()
                    .filter(|r| r.category != from)
                    .cloned()
                    .collect();
                for record in &moving {
                    let mut moved = (*record).clone();
                    moved.category = to.clone();
                    Self::check_free(&staying, &moved)?;
                }
                for record in moving {
                    let bytes = self.instrument_bytes(&record.id)?;
                    let (mut moved, body) = split(&bytes)?;
                    moved.category = to.clone();
                    moved.modified_at = timestamp();
                    self.write_instrument(&framed(&moved, body)?, &moved.id)?;
                }
                Ok(json!({"ok":true}))
            }
            ("/api/instruments/delete", true) => {
                let id = id()?;
                let trash = self.instrument_root()?.join("Trash");
                fs::create_dir_all(&trash).map_err(|e| e.to_string())?;
                fs::rename(
                    self.instrument_path(id)?,
                    trash.join(format!("{id}-{}.sadinst", unique_id())),
                )
                .map_err(|e| format!("Cannot delete the instrument: {e}"))?;
                Ok(json!({"ok":true}))
            }
            ("/api/instruments/export", true) => {
                let bytes = self.instrument_bytes(id()?)?;
                let (record, _) = split(&bytes)?;
                if let Some(path) = data["path"].as_str() {
                    super::audio_library::write_file(path.as_ref(), &bytes)?;
                    return Ok(json!({"ok":true,"name":record.name}));
                }
                let title: String = record
                    .name
                    .chars()
                    .map(|c| {
                        if c.is_control() || "/\\:*?\"<>|".contains(c) {
                            '_'
                        } else {
                            c
                        }
                    })
                    .collect();
                Ok(json!({
                    "binary": BASE64.encode(bytes),
                    "mime": "application/x-sad-pet-instrument",
                    "filename": format!("{title}.sadinst"),
                }))
            }
            ("/api/instruments/import", true) => {
                let bytes = match data["path"].as_str() {
                    Some(path) => super::audio_library::read_file(path.as_ref())?,
                    None => BASE64
                        .decode(
                            data["base64"]
                                .as_str()
                                .ok_or("Choose an instrument file.")?,
                        )
                        .map_err(|_| "The instrument file is invalid.")?,
                };
                let (mut record, mut state) = decode(&bytes)?;
                let (kind, _) = check(&mut state)?;
                let records = self.instruments()?;
                // A copy of its own, under a name free in its category.
                record.id = unique_id();
                record.kind = kind;
                record.created_at = timestamp();
                record.modified_at = record.created_at;
                let title = record.name.clone();
                let mut suffix = 2;
                while Self::check_free(&records, &record).is_err() {
                    record.name =
                        format!("{} ({suffix})", title.chars().take(72).collect::<String>());
                    suffix += 1;
                }
                self.write_instrument(&encode(&record, &state)?, &record.id)?;
                Ok(json!(record))
            }
            _ => Err("Unknown instrument preset action.".into()),
        }
    }
}
