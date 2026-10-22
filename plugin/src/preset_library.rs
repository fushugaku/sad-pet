//! User patches live on disk, shared by AU/VST3 instances. No file work runs on Audio.
use super::{BASE64, Command, Controller, MAX_BYTES, SavedState, Source};
use base64::Engine as _;
use flate2::{Compression, read::GzDecoder, write::GzEncoder};
use sad_pet::{preset::Preset, wav};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    fs::{self, File, OpenOptions},
    io::{Cursor, Read, Write},
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

const MAGIC: &[u8; 8] = b"SADPET\x01\0";
const MAX_HEADER: usize = 4096;
const MAX_STATE: u64 = 256 * 1024 * 1024;
static NONCE: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Default, Serialize, Deserialize)]
pub(super) struct Selection {
    pub id: String,
    pub dirty: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub(super) struct Record {
    pub id: String,
    pub name: String,
    pub created_at: u64,
    pub modified_at: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    import_key: Option<String>,
    #[serde(default = "gzip")]
    codec: String,
}
fn gzip() -> String {
    "gzip".into()
}
pub(super) fn timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
pub(super) fn unique_id() -> String {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!(
        "{:016x}{:08x}{:08x}",
        time as u64,
        std::process::id(),
        NONCE.fetch_add(1, Ordering::Relaxed) as u32
    )
}
pub(super) fn name(value: &str) -> Result<String, String> {
    let name = value.trim();
    if name.is_empty() {
        return Err("Enter a preset name.".into());
    }
    if name.chars().count() > 80 || name.chars().any(char::is_control) {
        return Err("Use a preset name of up to 80 characters.".into());
    }
    Ok(name.into())
}
pub(super) fn valid_id(id: &str) -> bool {
    id.len() == 32 && id.bytes().all(|c| c.is_ascii_hexdigit())
}
fn read_header(input: &mut impl Read) -> Result<Record, String> {
    let mut magic = [0; 8];
    input
        .read_exact(&mut magic)
        .map_err(|_| "The preset file is incomplete.")?;
    if &magic != MAGIC {
        return Err("Use a SAD PET preset (.sadpet).".into());
    }
    let mut size = [0; 4];
    input
        .read_exact(&mut size)
        .map_err(|_| "The preset file is incomplete.")?;
    let size = u32::from_le_bytes(size) as usize;
    if size == 0 || size > MAX_HEADER {
        return Err("The preset header is invalid.".into());
    }
    let mut bytes = vec![0; size];
    input
        .read_exact(&mut bytes)
        .map_err(|_| "The preset file is incomplete.")?;
    let record: Record =
        serde_json::from_slice(&bytes).map_err(|_| "The preset header is invalid.")?;
    name(&record.name)?;
    if !valid_id(&record.id) || record.codec != "gzip" {
        return Err("Unsupported preset format.".into());
    }
    Ok(record)
}
pub(super) fn decode(bytes: &[u8]) -> Result<(Record, SavedState), String> {
    if bytes.len() > MAX_BYTES {
        return Err("The preset file exceeds 128 MB.".into());
    }
    let mut input = Cursor::new(bytes);
    let record = read_header(&mut input)?;
    let mut decoded = Vec::new();
    GzDecoder::new(input)
        .take(MAX_STATE + 1)
        .read_to_end(&mut decoded)
        .map_err(|_| "The preset file is damaged or incomplete.")?;
    if decoded.len() as u64 > MAX_STATE {
        return Err("The preset is too large.".into());
    }
    let state = serde_json::from_slice(&decoded).map_err(|_| "The preset data is invalid.")?;
    Ok((record, state))
}
fn encode(record: &Record, state: &SavedState) -> Result<Vec<u8>, String> {
    let header = serde_json::to_vec(record).map_err(|e| e.to_string())?;
    if header.len() > MAX_HEADER {
        return Err("The preset metadata is too large.".into());
    }
    let mut file = Vec::new();
    file.extend_from_slice(MAGIC);
    file.extend_from_slice(&(header.len() as u32).to_le_bytes());
    file.extend_from_slice(&header);
    let mut compressed = GzEncoder::new(file, Compression::new(6));
    serde_json::to_writer(&mut compressed, state).map_err(|e| e.to_string())?;
    let file = compressed.finish().map_err(|e| e.to_string())?;
    if file.len() > MAX_BYTES {
        return Err("The preset file exceeds 128 MB.".into());
    }
    Ok(file)
}

pub(super) struct Library {
    root: Option<PathBuf>,
}
impl Library {
    #[cfg(test)]
    pub(super) fn at(root: PathBuf) -> Self {
        Self { root: Some(root) }
    }
    pub fn standard() -> Self {
        let root = std::env::var_os("SAD_PET_PRESET_DIR")
            .map(PathBuf::from)
            .or_else(|| {
                if cfg!(target_os = "windows") {
                    std::env::var_os("APPDATA").map(|p| PathBuf::from(p).join("SAD PET/Presets"))
                } else if cfg!(target_os = "macos") {
                    std::env::var_os("HOME").map(|p| {
                        PathBuf::from(p).join("Library/Application Support/SAD PET/Presets")
                    })
                } else {
                    std::env::var_os("XDG_DATA_HOME")
                        .map(PathBuf::from)
                        .or_else(|| {
                            std::env::var_os("HOME").map(|p| PathBuf::from(p).join(".local/share"))
                        })
                        .map(|p| p.join("sad-pet/presets"))
                }
            });
        Self { root }
    }
    pub(super) fn root(&self) -> Result<&std::path::Path, String> {
        self.root
            .as_deref()
            .ok_or_else(|| "The user preset folder is unavailable.".into())
    }
    fn path(&self, id: &str) -> Result<PathBuf, String> {
        if !valid_id(id) {
            return Err("Unknown user preset.".into());
        }
        Ok(self.root()?.join(format!("{id}.sadpet")))
    }
    pub fn list(&self) -> Result<Vec<Record>, String> {
        let root = self.root()?;
        if !root.exists() {
            return Ok(Vec::new());
        }
        let mut records = Vec::new();
        for entry in
            fs::read_dir(root).map_err(|e| format!("Cannot open the preset library: {e}"))?
        {
            let path = entry.map_err(|e| e.to_string())?.path();
            if path.extension().is_none_or(|e| e != "sadpet") {
                continue;
            }
            let mut file = File::open(&path).map_err(|e| e.to_string())?;
            if let Ok(record) = read_header(&mut file)
                && path.file_stem().and_then(|n| n.to_str()) == Some(&record.id)
            {
                records.push(record);
            }
        }
        records.sort_by(|a, b| {
            a.name
                .to_lowercase()
                .cmp(&b.name.to_lowercase())
                .then(a.id.cmp(&b.id))
        });
        Ok(records)
    }
    fn bytes(&self, id: &str) -> Result<Vec<u8>, String> {
        let file = File::open(self.path(id)?)
            .map_err(|_| "This preset is no longer in the library. Save it as a new preset.")?;
        let mut bytes = Vec::new();
        file.take(MAX_BYTES as u64 + 1)
            .read_to_end(&mut bytes)
            .map_err(|e| e.to_string())?;
        if bytes.len() > MAX_BYTES {
            return Err("The preset file exceeds 128 MB.".into());
        }
        Ok(bytes)
    }
    fn write(
        &self,
        title: &str,
        id: Option<&str>,
        state: &SavedState,
        import_key: Option<String>,
    ) -> Result<Record, String> {
        let title = name(title)?;
        let records = self.list()?;
        if records
            .iter()
            .any(|r| r.name.to_lowercase() == title.to_lowercase() && Some(r.id.as_str()) != id)
        {
            return Err(format!(
                "A preset named “{title}” already exists. Choose another name."
            ));
        }
        let existing = id
            .map(|id| {
                records
                    .iter()
                    .find(|r| r.id == id)
                    .ok_or("This preset was removed. Use Save as new.")
            })
            .transpose()?;
        let record = Record {
            id: existing.map_or_else(unique_id, |r| r.id.clone()),
            name: title,
            created_at: existing.map_or_else(timestamp, |r| r.created_at),
            modified_at: timestamp(),
            import_key: import_key.or_else(|| existing.and_then(|r| r.import_key.clone())),
            codec: gzip(),
        };
        let bytes = encode(&record, state)?;
        fs::create_dir_all(self.root()?)
            .map_err(|e| format!("Cannot create the preset folder: {e}"))?;
        let path = self.path(&record.id)?;
        let temporary = self
            .root()?
            .join(format!(".{}.{}.tmp", record.id, unique_id()));
        let write = || -> std::io::Result<()> {
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&temporary)?;
            file.write_all(&bytes)?;
            file.sync_all()?;
            fs::rename(&temporary, &path)
        };
        if let Err(error) = write() {
            let _ = fs::remove_file(temporary);
            return Err(format!("Cannot save the preset: {error}"));
        }
        Ok(record)
    }
    fn remove(&self, id: &str) -> Result<(), String> {
        let trash = self.root()?.join("Trash");
        fs::create_dir_all(&trash).map_err(|e| e.to_string())?;
        fs::rename(
            self.path(id)?,
            trash.join(format!("{id}-{}.sadpet", unique_id())),
        )
        .map_err(|e| format!("Cannot delete the preset: {e}"))
    }
}

fn validate(state: &SavedState) -> Result<(), String> {
    let mut ids = std::collections::BTreeSet::new();
    for part in &state.timbres {
        if part.meta.id == 0 || !ids.insert(part.meta.id) || !part.timbres.is_empty() {
            return Err("Invalid timbre identities".into());
        }
        validate(part)?;
    }
    if state.version != 1 || state.preset.format != "evil-pet-rust-v1" {
        return Err("Unsupported SAD PET preset version.".into());
    }
    let bytes = BASE64
        .decode(&state.sample_wav)
        .map_err(|_| "The saved sample is invalid.")?;
    if bytes.len() > MAX_BYTES {
        return Err("The saved sample exceeds 128 MB.".into());
    }
    let sample = wav::decode(Cursor::new(bytes), state.sample_name.clone())?;
    state.region.bounds(sample.len())?;
    state.fades.validate().map(|_| ())
}

impl Controller {
    pub(super) fn preset_library_request(
        &mut self,
        path: &str,
        post: bool,
        data: &Value,
    ) -> Result<Value, String> {
        let id = || data["id"].as_str().ok_or("Choose a user preset.");
        match (path, post) {
            ("/api/preset-library", false) => Ok(json!(self.library.list()?)),
            ("/api/preset-library/save", true) => {
                let title = name(data["name"].as_str().unwrap_or(""))?;
                let mut state = self.snapshot()?;
                state.preset.name = title.clone();
                let record = self
                    .library
                    .write(&title, data["id"].as_str(), &state, None)?;
                self.preset.name = title;
                self.selection = Selection {
                    id: record.id.clone(),
                    dirty: false,
                };
                Ok(json!(record))
            }
            ("/api/preset-library/load", true) => {
                let (record, mut state) = decode(&self.library.bytes(id()?)?)?;
                validate(&state)?;
                // Patch recall carries the sound and source; transport stays with this instance.
                state.preset.name = record.name;
                state.selection = Selection {
                    id: record.id,
                    dirty: false,
                };
                if !self.effect {
                    state.source = Source::File;
                    for part in &mut state.timbres {
                        part.source = Source::File;
                    }
                }
                state.frozen = false;
                state.bypass = self.bypass;
                state.midi_gate = self.midi_gate;
                state.playing = self.playing && !self.midi_gate;
                self.restore_saved(state)?;
                Ok(self.state())
            }
            ("/api/preset-library/factory", true) => {
                let index = data["index"]
                    .as_u64()
                    .filter(|n| *n <= sad_pet::startup::INDEX as u64)
                    .ok_or("Unknown factory preset.")?;
                let preset = Preset::builtin(index as usize);
                let startup = index as usize == sad_pet::startup::INDEX;
                let sample = if startup {
                    sad_pet::startup::sample()
                } else {
                    self.sample.clone()
                };
                let region = if startup {
                    sad_pet::startup::REGION
                } else {
                    self.region
                };
                let fades = if startup {
                    sad_pet::startup::FADES
                } else {
                    self.fades
                };
                let source =
                    self.prepared_source_for(sample.clone(), self.source, region, fades, &preset);
                // A scene from the library starts with only its Master.
                let mut parts = Vec::new();
                let mut part_audio = None;
                if self.meta.id == 0 {
                    super::timbres::ensure_master_in(&mut parts, &mut part_audio, self, self.rate);
                }
                self.enqueue(Command::Restore(Box::new(super::Restore {
                    parts: part_audio,
                    mix: Default::default(),
                    preset: preset.prepared_clone_for(self.rate),
                    source,
                    sync: self.sync,
                    frozen: self.frozen,
                    bypass: self.bypass,
                    midi_gate: self.midi_gate,
                    playing: self.playing,
                })))?;
                self.parts = parts;
                self.active = 0;
                self.meta = sad_pet::timbres::Meta::default();
                self.scene_revision = self.scene_revision.wrapping_add(1);
                self.preset = preset;
                self.sample = sample;
                self.region = region;
                self.fades = fades;
                self.selection = Selection {
                    id: format!("factory:{index}"),
                    dirty: false,
                };
                Ok(self.state())
            }
            ("/api/preset-library/delete", true) => {
                let id = id()?;
                self.library.remove(id)?;
                if self.selection.id == id {
                    self.selection = Selection {
                        id: String::new(),
                        dirty: true,
                    };
                }
                Ok(json!({"ok":true}))
            }
            ("/api/preset-library/export", true) => {
                let bytes = self.library.bytes(id()?)?;
                let (record, _) = decode(&bytes)?;
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
                Ok(
                    json!({"binary":BASE64.encode(bytes),"mime":"application/x-sad-pet-preset","filename":format!("{title}.sadpet")}),
                )
            }
            ("/api/preset-library/import", true) => {
                let bytes = BASE64
                    .decode(data["base64"].as_str().ok_or("Choose a preset file.")?)
                    .map_err(|_| "The preset file is invalid.")?;
                if bytes.len() > MAX_BYTES {
                    return Err("The preset file exceeds 128 MB.".into());
                }
                let key = data["import_key"].as_str().map(str::to_owned);
                let records = self.library.list()?;
                if let Some(key) = &key
                    && let Some(record) =
                        records.iter().find(|r| r.import_key.as_ref() == Some(key))
                {
                    return Ok(json!(record));
                }
                let mut state = if bytes.starts_with(MAGIC) {
                    decode(&bytes)?.1
                } else {
                    let value: Value = serde_json::from_slice(&bytes)
                        .map_err(|_| "Use a SAD PET preset (.sadpet or .sad.json).")?;
                    if value["format"] != "evil-pet-rust-v1" {
                        return Err("Use a SAD PET preset (.sadpet or .sad.json).".into());
                    }
                    let mut state = self.snapshot()?;
                    state.preset = serde_json::from_value(value).map_err(|e| e.to_string())?;
                    state.timbres.clear();
                    state.active_timbre = 0;
                    state.meta = sad_pet::timbres::Meta::default();
                    state
                };
                validate(&state)?;
                state.preset.sanitize();
                let title = name(&state.preset.name)?;
                let mut available = title.clone();
                let mut suffix = 2;
                while records
                    .iter()
                    .any(|r| r.name.to_lowercase() == available.to_lowercase())
                {
                    available =
                        format!("{} ({suffix})", title.chars().take(72).collect::<String>());
                    suffix += 1;
                }
                state.preset.name = available.clone();
                Ok(json!(self.library.write(&available, None, &state, key)?))
            }
            _ => Err("Unknown preset library action.".into()),
        }
    }
}
