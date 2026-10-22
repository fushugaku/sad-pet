//! The standalone app's audio library. WAV files are read, written and rendered
//! here; the editor receives names, durations and engine waveforms, never samples.
use super::{
    Controller, MAX_BYTES, Source,
    preset_library::{timestamp, unique_id, valid_id},
};
use sad_pet::{
    Stereo,
    dsp::{Fades, Region, SampleBuffer},
    wav,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs::{self, File, OpenOptions},
    io::{Cursor, Read, Write},
    path::{Path, PathBuf},
    sync::Arc,
};

const KINDS: [&str; 3] = ["audio", "render", "recording"];
const INDEX: &str = "library.json";
const MAX_RENDER_SECONDS: f64 = 600.0;

/// Refuses every editor request that would carry sample data across the
/// WebView boundary. The standalone editor uses the path-based routes below.
pub(super) fn check_editor_request(path: &str, post: bool, data: &Value) -> Result<(), String> {
    let path = path.split('?').next().unwrap_or(path);
    let carries_audio = match path {
        "/api/wav" | "/api/sample.wav" | "/api/trim" | "/api/scene" | "/api/render"
        | "/api/input.wav" => true,
        "/api/preset-library/import" | "/api/effect-presets/import" => {
            data.get("base64").is_some() || data["path"].as_str().is_none()
        }
        "/api/preset-library/export" | "/api/effect-presets/export" => {
            data["path"].as_str().is_none()
        }
        _ => path.starts_with("/api/samples/") || path.starts_with("/api/recording."),
    };
    if carries_audio {
        let action = if post { "Sending" } else { "Reading" };
        return Err(format!(
            "{action} {path} is handled natively in the SAD PET app."
        ));
    }
    Ok(())
}

pub(super) fn read_file(path: &Path) -> Result<Vec<u8>, String> {
    let file = File::open(path).map_err(|e| format!("Cannot open {}: {e}", label(path)))?;
    let mut bytes = Vec::new();
    file.take(MAX_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| format!("Cannot read {}: {e}", label(path)))?;
    if bytes.len() > MAX_BYTES {
        return Err(format!("{} exceeds 128 MB.", label(path)));
    }
    Ok(bytes)
}

/// Replaces the destination only once the new contents are complete.
pub(super) fn write_file(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let temporary = path.with_file_name(format!(".{}.{}.tmp", label(path), unique_id()));
    let write = || -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)?;
        file.write_all(bytes)?;
        file.sync_all()?;
        fs::rename(&temporary, path)
    };
    write().map_err(|e| {
        let _ = fs::remove_file(&temporary);
        format!("Cannot save {}: {e}", label(path))
    })
}

fn label(path: &Path) -> String {
    path.file_name().map_or_else(
        || path.display().to_string(),
        |n| n.to_string_lossy().into(),
    )
}

fn stem(name: &str) -> String {
    let lower = name.to_lowercase();
    if lower.ends_with(".wav") {
        name[..name.len() - 4].into()
    } else {
        name.into()
    }
}

/// Content fingerprint for re-imports of the same file (FNV-1a).
fn fingerprint(bytes: &[u8]) -> String {
    let hash = bytes.iter().fold(0xcbf2_9ce4_8422_2325_u64, |hash, &byte| {
        (hash ^ byte as u64).wrapping_mul(0x0100_0000_01b3)
    });
    format!("{hash:016x}-{}", bytes.len())
}

fn factory_track(id: &str) -> Option<&'static str> {
    let track = id.strip_prefix("factory:")?.strip_suffix(":v1")?;
    sad_pet::samples::TRACKS
        .iter()
        .find(|t| t.id == track)
        .map(|t| t.id)
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Entry {
    pub id: String,
    pub kind: String,
    pub name: String,
    pub display_name: String,
    pub seconds: f64,
    pub size: u64,
    pub added_at: u64,
    pub modified_at: u64,
    #[serde(default)]
    pub trashed_at: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recorded_source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trimmed_from: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<Region>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applied_fades: Option<Fades>,
}
impl Entry {
    /// Blank names are filled from the imported file when it is stored.
    fn new(kind: &str, name: &str, display_name: &str) -> Self {
        Self {
            id: String::new(),
            kind: kind.into(),
            name: name.trim().into(),
            display_name: display_name.trim().into(),
            seconds: 0.,
            size: 0,
            added_at: 0,
            modified_at: 0,
            trashed_at: None,
            hash: None,
            recorded_source: None,
            trimmed_from: None,
            region: None,
            applied_fades: None,
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
struct Index {
    version: u32,
    #[serde(default)]
    files: Vec<Entry>,
    /// Fade settings per library or factory id, restored when the file loads.
    #[serde(default)]
    fades: BTreeMap<String, Fades>,
}

pub(super) struct Files {
    root: Option<PathBuf>,
}
impl Files {
    #[cfg(test)]
    pub(super) fn at(root: PathBuf) -> Self {
        Self { root: Some(root) }
    }
    pub fn standard() -> Self {
        let root = std::env::var_os("SAD_PET_LIBRARY_DIR")
            .map(PathBuf::from)
            .or_else(|| {
                if cfg!(target_os = "windows") {
                    std::env::var_os("APPDATA").map(|p| PathBuf::from(p).join("SAD PET/Library"))
                } else if cfg!(target_os = "macos") {
                    std::env::var_os("HOME").map(|p| {
                        PathBuf::from(p).join("Library/Application Support/SAD PET/Library")
                    })
                } else {
                    std::env::var_os("XDG_DATA_HOME")
                        .map(PathBuf::from)
                        .or_else(|| {
                            std::env::var_os("HOME").map(|p| PathBuf::from(p).join(".local/share"))
                        })
                        .map(|p| p.join("sad-pet/library"))
                }
            });
        Self { root }
    }
    fn root(&self) -> Result<&Path, String> {
        self.root
            .as_deref()
            .ok_or_else(|| "The audio library folder is unavailable.".into())
    }
    fn audio_path(&self, id: &str) -> Result<PathBuf, String> {
        match id.split_once(':') {
            Some((kind, key)) if KINDS.contains(&kind) && valid_id(key) => {
                Ok(self.root()?.join("Audio").join(format!("{key}.wav")))
            }
            _ => Err("File is no longer in the library.".into()),
        }
    }
    fn index(&self) -> Result<Index, String> {
        let path = self.root()?.join(INDEX);
        if !path.exists() {
            return Ok(Index {
                version: 1,
                ..Index::default()
            });
        }
        let bytes = read_file(&path)?;
        serde_json::from_slice(&bytes).map_err(|_| "The audio library index is damaged.".into())
    }
    fn store(&self, index: &Index) -> Result<(), String> {
        fs::create_dir_all(self.root()?)
            .map_err(|e| format!("Cannot create the audio library: {e}"))?;
        let bytes = serde_json::to_vec(index).map_err(|e| e.to_string())?;
        write_file(&self.root()?.join(INDEX), &bytes)
    }
    pub fn list(&self) -> Result<(Vec<Entry>, BTreeMap<String, Fades>), String> {
        let index = self.index()?;
        let files = index
            .files
            .into_iter()
            .filter(|e| self.audio_path(&e.id).is_ok_and(|p| p.is_file()))
            .collect();
        Ok((files, index.fades))
    }
    /// Validates, stores and indexes a WAV file. Re-importing identical audio
    /// returns (and restores) the existing entry.
    fn add(&self, bytes: &[u8], mut entry: Entry) -> Result<Entry, String> {
        if bytes.len() < 12 || &bytes[..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
            return Err("Use a WAV audio file.".into());
        }
        if entry.name.is_empty() {
            entry.name = "Sample.wav".into();
        }
        if entry.display_name.is_empty() {
            entry.display_name = stem(&entry.name);
        }
        let sample = wav::decode(Cursor::new(bytes), entry.name.clone())?;
        let hash = fingerprint(bytes);
        let mut index = self.index()?;
        if entry.kind == "audio"
            && let Some(existing) = index.files.iter_mut().find(|e| {
                e.kind == "audio"
                    && e.hash.as_deref() == Some(&hash)
                    && self.audio_path(&e.id).is_ok_and(|p| p.is_file())
            })
        {
            existing.trashed_at = None;
            let existing = existing.clone();
            self.store(&index)?;
            return Ok(existing);
        }
        let key = unique_id();
        entry.id = format!("{}:{key}", entry.kind);
        let path = self.audio_path(&entry.id)?;
        fs::create_dir_all(path.parent().unwrap())
            .map_err(|e| format!("Cannot create the audio library: {e}"))?;
        write_file(&path, bytes)?;
        entry.seconds = sample.duration();
        entry.size = bytes.len() as u64;
        entry.added_at = timestamp();
        entry.modified_at = entry.added_at;
        entry.hash = Some(hash);
        index.files.push(entry.clone());
        if let Err(error) = self.store(&index) {
            let _ = fs::remove_file(path);
            return Err(error);
        }
        Ok(entry)
    }
    pub fn import(&self, path: &Path, mut entry: Entry) -> Result<Entry, String> {
        let bytes = read_file(path)?;
        if entry.name.is_empty() {
            entry.name = label(path);
        }
        self.add(&bytes, entry)
    }
    fn entry(&self, id: &str) -> Result<Entry, String> {
        self.index()?
            .files
            .into_iter()
            .find(|e| e.id == id)
            .ok_or_else(|| "File is no longer in the library.".into())
    }
    pub fn read(&self, id: &str) -> Result<SampleBuffer, String> {
        let entry = self.entry(id)?;
        let bytes = read_file(&self.audio_path(id)?)?;
        let mut sample = wav::decode(Cursor::new(bytes), entry.name)?;
        sample.library_id = Some(entry.id);
        Ok(sample)
    }
    pub fn trash(&self, id: &str, restore: bool) -> Result<(), String> {
        let mut index = self.index()?;
        let entry = index
            .files
            .iter_mut()
            .find(|e| e.id == id)
            .ok_or("File is no longer in the library.")?;
        entry.trashed_at = (!restore).then(timestamp);
        self.store(&index)
    }
    pub fn export(&self, id: &str, destination: &Path) -> Result<(), String> {
        self.entry(id)?;
        write_file(destination, &read_file(&self.audio_path(id)?)?)
    }
    pub fn fades(&self, id: &str) -> Result<Fades, String> {
        Ok(self.index()?.fades.get(id).copied().unwrap_or_default())
    }
    pub fn set_fades(&self, id: &str, fades: Fades) -> Result<(), String> {
        if factory_track(id).is_none() {
            self.entry(id)?;
        }
        let mut index = self.index()?;
        if fades == Fades::default() {
            index.fades.remove(id);
        } else {
            index.fades.insert(id.into(), fades.validate()?);
        }
        self.store(&index)
    }
}

impl Controller {
    /// The timbre the editor shows; file and region commands apply to it.
    fn active_part(&mut self) -> Result<&mut Controller, String> {
        if self.active == 0 {
            return Ok(self);
        }
        let active = self.active;
        self.parts
            .iter_mut()
            .find(|p| p.meta.id == active)
            .ok_or_else(|| "Unknown timbre".into())
    }

    fn file_part(&mut self) -> Result<&mut Controller, String> {
        let part = self.active_part()?;
        if part.meta.kind.routing() {
            return Err("A bus takes its input from the other timbres".into());
        }
        Ok(part)
    }

    /// An independent copy of the scene that plays from the start, as Play does.
    fn offline_copy(&self) -> Controller {
        let mut copy = Controller::new(self.effect);
        copy.standalone = self.standalone;
        copy.meta = self.meta.clone();
        copy.parts = self.parts.iter().map(Controller::offline_copy).collect();
        copy.active = self.active;
        copy.preset = self.preset.clone();
        copy.sample = self.sample.clone();
        copy.source = self.source;
        copy.region = self.region;
        copy.fades = self.fades;
        copy.sync = false;
        copy.frozen = self.frozen && self.source == Source::File;
        copy.bypass = self.bypass;
        copy.midi_gate = false;
        copy.playing = true;
        copy.rate = self.rate;
        copy
    }

    fn render_scene(&self, seconds: f64) -> Result<Vec<Stereo>, String> {
        if !(0.1..=MAX_RENDER_SECONDS).contains(&seconds) {
            return Err("Choose a render length of up to 10 minutes.".into());
        }
        let mut copy = self.offline_copy();
        let values = copy.values();
        let mut audio = copy.prepare(self.rate);
        let total = (seconds * self.rate as f64).round() as usize;
        let mut frames = Vec::with_capacity(total);
        while frames.len() < total {
            let block = (total - frames.len()).min(512);
            audio.begin(&values, 0., None, true, block);
            for _ in 0..block {
                frames.push(audio.process_frame([0.; 2]));
            }
            audio.end(block);
        }
        Ok(frames)
    }

    pub(super) fn library_request(
        &mut self,
        path: &str,
        post: bool,
        data: &Value,
    ) -> Result<Value, String> {
        let id = || data["id"].as_str().ok_or("Choose a file.");
        let text = |key: &str| data[key].as_str().unwrap_or_default();
        match (path, post) {
            ("/api/library", false) => {
                let (files, fades) = self.files.list()?;
                Ok(json!({"files":files,"fades":fades}))
            }
            ("/api/library/load", true) => {
                let id = id()?;
                let sample = match factory_track(id) {
                    Some(track) => sad_pet::samples::get(track).ok_or("Unknown factory sample")?,
                    None => Arc::new(self.files.read(id)?),
                };
                let fades = self.files.fades(id)?;
                let result = self.file_part()?.use_sample(sample, fades)?;
                self.selection.dirty = true;
                Ok(result)
            }
            ("/api/library/import", true) => {
                let paths: Vec<&str> = match data["paths"].as_array() {
                    Some(paths) => paths.iter().filter_map(Value::as_str).collect(),
                    None => data["path"].as_str().into_iter().collect(),
                };
                if paths.is_empty() {
                    return Err("Choose files to import.".into());
                }
                let (mut files, mut presets, mut errors) = (Vec::new(), Vec::new(), Vec::new());
                for path in paths {
                    let lower = path.to_lowercase();
                    let result = if lower.ends_with(".sadpet") || lower.ends_with(".json") {
                        self.preset_library_request(
                            "/api/preset-library/import",
                            true,
                            &json!({"path":path}),
                        )
                        .map(|record| presets.push(record))
                    } else {
                        self.files
                            .import(Path::new(path), Entry::new("audio", "", ""))
                            .map(|entry| files.push(entry))
                    };
                    if let Err(error) = result {
                        errors.push(format!("{}: {error}", label(Path::new(path))));
                    }
                }
                Ok(json!({"files":files,"presets":presets,"errors":errors}))
            }
            ("/api/library/trash", true) => {
                self.files.trash(id()?, data["restore"] == true)?;
                Ok(json!({"ok":true}))
            }
            ("/api/library/fades", true) => {
                let fades: Fades =
                    serde_json::from_value(data["fades"].clone()).map_err(|e| e.to_string())?;
                self.files.set_fades(id()?, fades)?;
                Ok(json!({"ok":true}))
            }
            ("/api/library/export", true) => {
                let destination = data["path"]
                    .as_str()
                    .ok_or("Choose where to save the file.")?;
                if data["format"] == "mp3" {
                    return Err("The app saves recordings as WAV.".into());
                }
                let id = id()?;
                match factory_track(id) {
                    Some(track) => {
                        let sample =
                            sad_pet::samples::get(track).ok_or("Unknown factory sample")?;
                        let bytes = wav::bytes(sample.sample_rate, sample.frames())?;
                        write_file(destination.as_ref(), &bytes)?;
                    }
                    None => self.files.export(id, destination.as_ref())?,
                }
                Ok(json!({"ok":true}))
            }
            ("/api/library/trim", true) => {
                let mut entry = Entry::new("audio", text("name"), text("displayName"));
                let bytes = {
                    let part = self.file_part()?;
                    if part.source != Source::File {
                        return Err("Load a file to select a region".into());
                    }
                    entry.trimmed_from = part.sample.library_id.clone();
                    wav::bytes(part.sample.sample_rate, &part.trimmed(data)?)?
                };
                entry.region = serde_json::from_value(data.clone()).ok();
                entry.applied_fades = serde_json::from_value(data["fades"].clone()).ok();
                Ok(json!(self.files.add(&bytes, entry)?))
            }
            ("/api/library/render", true) => {
                let seconds = data["seconds"].as_f64().ok_or("Choose a render length.")?;
                self.drain();
                let frames = self.render_scene(seconds)?;
                let entry = Entry::new("render", text("name"), text("displayName"));
                Ok(json!(
                    self.files.add(&wav::bytes(self.rate, &frames)?, entry)?
                ))
            }
            // Issued by the app's recorder with its own file; the editor cannot name one.
            ("/api/library/take", true) => {
                let file = data["path"]
                    .as_str()
                    .ok_or("The recording is unavailable.")?;
                let mut entry = Entry::new("recording", text("name"), text("displayName"));
                entry.recorded_source = Some(
                    if data["source"] == "input" {
                        "input"
                    } else {
                        "output"
                    }
                    .into(),
                );
                Ok(json!(self.files.import(Path::new(file), entry)?))
            }
            _ => Err("Unknown library action.".into()),
        }
    }
}
