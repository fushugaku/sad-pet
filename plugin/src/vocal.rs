//! Native singing: one render thread per plug-in instance plans and renders
//! the phrases of every Vocal timbre, closest to the playhead first, and
//! posts each finished song through the timbre's mailbox; the audio thread
//! never waits for it. Voicebanks live as `.sadvoice` files in the Voices
//! folder beside the User presets.
use sad_pet::vocal::{self, Job, Mailbox, Plan, Progress, Renderer, voice};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::Ordering;
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;

/// The voicebanks that come with SAD PET (`vocal::PACKS`), built in.
const BUNDLED: [(&str, &[u8]); 3] = [
    (
        "kasane-teto",
        include_bytes!("../../voice/packs/kasane-teto.sadvoice"),
    ),
    (
        "kasane-teto-en",
        include_bytes!("../../voice/packs/kasane-teto-en.sadvoice"),
    ),
    (
        "kumi-ru",
        include_bytes!("../../voice/packs/kumi-ru.sadvoice"),
    ),
];

fn bundled(id: &str) -> Option<&'static [u8]> {
    BUNDLED.iter().find(|(i, _)| *i == id).map(|(_, b)| *b)
}

/// A voicebank's file contents: built in, or from the Voices folder.
fn voice_bytes(id: &str) -> Result<std::borrow::Cow<'static, [u8]>, String> {
    if let Some(bytes) = bundled(id) {
        return Ok(bytes.into());
    }
    if id.is_empty() || id.contains(['/', '\\', '.']) {
        return Err("Choose a voicebank".into());
    }
    let folder = voices_folder().ok_or("No folder for voicebanks")?;
    std::fs::read(folder.join(format!("{id}.sadvoice")))
        .map(Into::into)
        .map_err(|e| format!("Cannot read the voicebank: {e}"))
}

/// The folder that holds imported voicebanks.
pub fn voices_folder() -> Option<PathBuf> {
    std::env::var_os("SAD_PET_VOICE_DIR")
        .map(PathBuf::from)
        .or_else(|| {
            super::preset_library::Library::standard()
                .root()
                .ok()
                .and_then(|root| root.parent().map(|p| p.join("Voices")))
        })
}

struct Entry {
    job: Job,
    mailbox: Arc<Mailbox>,
    /// Bumped by every submit, so the thread re-plans.
    generation: u64,
}

#[derive(Default)]
struct State {
    entries: BTreeMap<u32, Entry>,
    /// Progress of the plan of one generation of each timbre's job.
    progress: BTreeMap<u32, (u64, Progress)>,
    stop: bool,
    /// Voicebanks to drop from the renderer (removed files).
    forgotten: Vec<String>,
}

struct Shared {
    state: Mutex<State>,
    wake: Condvar,
}

pub struct Service {
    shared: Arc<Shared>,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl Service {
    pub fn new() -> Self {
        let shared = Arc::new(Shared {
            state: Mutex::new(State::default()),
            wake: Condvar::new(),
        });
        let worker = shared.clone();
        let thread = std::thread::Builder::new()
            .name("SAD PET vocal".into())
            .spawn(move || run(worker))
            .ok();
        Self { shared, thread }
    }
    /// Renders (or re-renders) a timbre's score into its mailbox.
    pub fn submit(&self, id: u32, settings: &vocal::Settings, rate: u32, mailbox: &Arc<Mailbox>) {
        if let Ok(mut state) = self.shared.state.lock() {
            let generation = state.entries.get(&id).map_or(0, |e| e.generation) + 1;
            state.entries.insert(
                id,
                Entry {
                    job: Job {
                        settings: settings.clone(),
                        rate,
                        host_bpm: None,
                    },
                    mailbox: mailbox.clone(),
                    generation,
                },
            );
        }
        self.shared.wake.notify_one();
    }
    pub fn forget(&self, id: u32) {
        if let Ok(mut state) = self.shared.state.lock() {
            state.entries.remove(&id);
            state.progress.remove(&id);
        }
    }
    /// Progress of the latest submitted score; nothing while it is unplanned.
    pub fn progress(&self, id: u32) -> Progress {
        self.shared
            .state
            .lock()
            .ok()
            .and_then(|s| {
                let (generation, progress) = s.progress.get(&id).copied()?;
                (s.entries.get(&id)?.generation == generation).then_some(progress)
            })
            .unwrap_or_default()
    }
    /// A voicebank file changed: every score using it renders again.
    pub fn voice_changed(&self, id: &str) {
        if let Ok(mut state) = self.shared.state.lock() {
            state.forgotten.push(id.to_string());
            for entry in state.entries.values_mut() {
                entry.generation += 1;
            }
        }
        self.shared.wake.notify_one();
    }
}
impl Default for Service {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for Service {
    fn drop(&mut self) {
        if let Ok(mut state) = self.shared.state.lock() {
            state.stop = true;
        }
        self.shared.wake.notify_all();
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

fn load_voice(renderer: &mut Renderer, id: &str) {
    if id.is_empty() || renderer.has_voice(id) {
        return;
    }
    if let Ok(bytes) = voice_bytes(id)
        && let Ok(bank) = voice::bank::Voicebank::from_bytes(&bytes)
    {
        renderer.add_voice(id, Arc::new(bank));
    }
}

/// The host tempo a score renders at: the DAW's, when it follows the host.
fn host_bpm(job: &Job, mailbox: &Mailbox) -> Option<f32> {
    let bpm = f32::from_bits(mailbox.host_bpm.load(Ordering::Relaxed));
    (job.settings.score.follow_host && bpm > 0.0).then_some(bpm)
}

fn run(shared: Arc<Shared>) {
    let mut renderer = Renderer::new();
    // id → (generation planned, plan, job)
    let mut plans: BTreeMap<u32, (u64, Plan, Job)> = BTreeMap::new();
    loop {
        let (work, forgotten) = {
            let Ok(mut state) = shared.state.lock() else {
                return;
            };
            if state.stop {
                return;
            }
            let forgotten = std::mem::take(&mut state.forgotten);
            let work: Vec<(u32, u64, Job, Arc<Mailbox>)> = state
                .entries
                .iter()
                .map(|(&id, e)| (id, e.generation, e.job.clone(), e.mailbox.clone()))
                .collect();
            (work, forgotten)
        };
        for id in forgotten {
            renderer.remove_voice(&id);
        }
        plans.retain(|id, _| work.iter().any(|w| w.0 == *id));
        let mut rendered = false;
        for (id, generation, mut job, mailbox) in work {
            mailbox.collect();
            job.host_bpm = host_bpm(&job, &mailbox);
            let stale = plans
                .get(&id)
                .is_none_or(|(g, _, planned)| *g != generation || planned.host_bpm != job.host_bpm);
            if stale {
                load_voice(&mut renderer, &job.settings.score.voice);
                let plan = renderer.plan(&job);
                mailbox.post(renderer.song(&plan));
                plans.insert(id, (generation, plan, job.clone()));
            }
            let (planned, plan, job) = &plans[&id];
            let near = mailbox.position.load(Ordering::Relaxed);
            if renderer.render_next(plan, near) {
                mailbox.post(renderer.song(plan));
                rendered = true;
            }
            if let Ok(mut state) = shared.state.lock() {
                let progress = renderer.progress(job, plan);
                state.progress.insert(id, (*planned, progress));
            }
            if rendered {
                // One phrase per pass, so new edits and other timbres are seen.
                break;
            }
        }
        let current: Vec<&Plan> = plans.values().map(|(_, p, _)| p).collect();
        renderer.retain(&current);
        if !rendered {
            let Ok(state) = shared.state.lock() else {
                return;
            };
            if state.stop {
                return;
            }
            // Wake for new work, or now and then to follow the host tempo.
            let _ = shared.wake.wait_timeout(state, Duration::from_millis(150));
        }
    }
}

impl super::Controller {
    /// Scene-wide Vocal requests: the song clock, voicebanks and phonemes.
    /// Score edits and the timbre's state go to the selected part instead.
    pub(super) fn vocal_scene_request(
        &mut self,
        path: &str,
        post: bool,
        data: &serde_json::Value,
    ) -> Option<Result<serde_json::Value, String>> {
        use base64::Engine as _;
        use serde_json::json;
        let base64 = base64::engine::general_purpose::STANDARD;
        let result = match (path, post) {
            ("/api/vocal/transport", true) => {
                let start = data["start"]
                    .as_f64()
                    .map(|s| (s.max(0.0) * self.rate as f64) as u64);
                let playing = data["playing"].as_bool();
                self.enqueue(super::Command::VocalTransport { playing, start })
                    .map(|_| json!({"ok":true}))
            }
            ("/api/vocal/voices", _) => {
                let mut list = list_voices();
                list.push(json!({"id":"","name":"Synthetic (no voicebank)","language":null,"builtin":true}));
                Ok(json!(list))
            }
            ("/api/vocal/phonemes", _) => {
                let language = voice::Language::parse(data["language"].as_str().unwrap_or("ja"))
                    .unwrap_or_default();
                let lyrics: Vec<String> =
                    serde_json::from_value(data["lyrics"].clone()).unwrap_or_default();
                let lyric: Vec<voice::g2p::Lyric> = lyrics
                    .iter()
                    .map(|t| voice::g2p::Lyric {
                        text: t,
                        phonemes: None,
                        language,
                    })
                    .collect();
                let default = match language {
                    voice::Language::Ja => "ら",
                    voice::Language::En => "la",
                    voice::Language::Ru => "ла",
                };
                Ok(json!(
                    voice::g2p::assign(&lyric, default, None)
                        .iter()
                        .map(|t| match t {
                            voice::g2p::NoteText::Hold => "-".to_string(),
                            voice::g2p::NoteText::Syllables(s) => s
                                .iter()
                                .flat_map(|s| s.phonemes.iter().map(|p| p.symbol()))
                                .collect::<Vec<_>>()
                                .join(" "),
                        })
                        .collect::<Vec<_>>()
                ))
            }
            ("/api/vocal/import", true) => (|| {
                let options = voice::utau::ImportOptions {
                    language: data["language"].as_str().and_then(voice::Language::parse),
                    name: data["name"].as_str().map(str::to_string),
                };
                let decode = |v: &serde_json::Value| -> Result<Vec<u8>, String> {
                    base64
                        .decode(v.as_str().unwrap_or(""))
                        .map_err(|e| e.to_string())
                };
                let bank = if let Some(path) = data["path"].as_str() {
                    import_path(Path::new(path), &options)?
                } else if data.get("voice").is_some() {
                    voice::bank::Voicebank::from_bytes(&decode(&data["voice"])?)?
                } else if data.get("zip").is_some() {
                    voice::utau::import(
                        &voice::zip::unzip(&decode(&data["zip"])?)?,
                        &options,
                        &mut |_, _| {},
                    )?
                } else {
                    let mut files = voice::utau::MemoryFiles::default();
                    for file in data["files"].as_array().ok_or("Choose a voicebank")? {
                        let path = file["path"].as_str().ok_or("Missing file name")?;
                        files
                            .0
                            .insert(path.replace('\\', "/"), decode(&file["base64"])?);
                    }
                    voice::utau::import(&files, &options, &mut |_, _| {})?
                };
                let info = store(&bank, options.name.as_deref())?;
                self.voice_service()
                    .voice_changed(info["id"].as_str().unwrap_or(""));
                Ok(info)
            })(),
            ("/api/vocal/voice/remove", true) => (|| {
                let id = data["id"].as_str().ok_or("Choose a voicebank")?;
                if id.is_empty() || bundled(id).is_some() || id.contains(['/', '\\', '.']) {
                    return Err("Voices that come with SAD PET cannot be removed".into());
                }
                let folder = voices_folder().ok_or("No folder for voicebanks")?;
                std::fs::remove_file(folder.join(format!("{id}.sadvoice")))
                    .map_err(|e| format!("Cannot remove the voicebank: {e}"))?;
                self.voice_service().voice_changed(id);
                Ok(json!({"ok":true}))
            })(),
            ("/api/vocal/export", true) => (|| {
                let id = data["id"].as_str().ok_or("Choose a voicebank")?;
                let bytes = voice_bytes(id)?;
                match data["path"].as_str() {
                    Some(path) => super::audio_library::write_file(Path::new(path), &bytes)
                        .map(|_| json!({"ok":true})),
                    None => Ok(
                        json!({"binary": base64.encode(bytes), "mime": "application/octet-stream"}),
                    ),
                }
            })(),
            ("/api/vocal/import-score", true) => (|| {
                let language = data["language"]
                    .as_str()
                    .and_then(voice::Language::parse)
                    .unwrap_or_default();
                let (name, bytes) = match data["path"].as_str() {
                    Some(path) => (
                        path.to_string(),
                        std::fs::read(path).map_err(|e| format!("Cannot read {path}: {e}"))?,
                    ),
                    None => (
                        data["name"].as_str().unwrap_or("score").to_string(),
                        base64
                            .decode(data["base64"].as_str().unwrap_or(""))
                            .map_err(|e| e.to_string())?,
                    ),
                };
                let score = voice::import::score(&name, &bytes, language)?;
                Ok(json!({ "score": score }))
            })(),
            ("/api/vocal/progress", _) => {
                let service = self.voice_service();
                let mut progress = serde_json::Map::new();
                for part in std::iter::once(&*self).chain(self.parts.iter()) {
                    if part.vocal.is_some() {
                        progress.insert(
                            part.meta.id.to_string(),
                            json!(service.progress(part.meta.id)),
                        );
                    }
                }
                Ok(json!({"progress": progress, "error": null}))
            }
            _ => return None,
        };
        Some(result)
    }
}

/// Lists the voicebanks that come with SAD PET, then the Voices folder:
/// `{id, name, language, author}`, with `bundled` on the former.
pub fn list_voices() -> Vec<serde_json::Value> {
    let mut out: Vec<serde_json::Value> = BUNDLED
        .iter()
        .filter_map(|(id, bytes)| {
            let info = voice::bank::Voicebank::read_info(bytes)?;
            Some(
                serde_json::json!({ "id": id, "name": info.name, "language": info.language,
                "author": info.author, "bundled": true }),
            )
        })
        .collect();
    out.extend(imported_voices());
    out
}

fn imported_voices() -> Vec<serde_json::Value> {
    let Some(folder) = voices_folder() else {
        return Vec::new();
    };
    let Ok(entries) = std::fs::read_dir(&folder) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_none_or(|e| e != "sadvoice") {
            continue;
        }
        let Some(id) = path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .filter(|id| bundled(id).is_none())
        else {
            continue;
        };
        if let Some(info) = read_info(&path) {
            out.push(
                serde_json::json!({ "id": id, "name": info.name, "language": info.language,
                "author": info.author }),
            );
        }
    }
    out.sort_by(|a, b| a["name"].as_str().cmp(&b["name"].as_str()));
    out
}

/// Reads only the header of a voicebank file.
fn read_info(path: &Path) -> Option<voice::bank::Info> {
    use std::io::Read;
    let mut file = std::fs::File::open(path).ok()?;
    let mut head = [0u8; 16];
    file.read_exact(&mut head).ok()?;
    if &head[..8] != b"SADVOICE" {
        return None;
    }
    let length = u32::from_le_bytes(head[12..16].try_into().ok()?) as usize;
    let mut bytes = head.to_vec();
    bytes.resize(16 + length.min(64 << 20), 0);
    file.read_exact(&mut bytes[16..]).ok()?;
    voice::bank::Voicebank::read_info(&bytes)
}

/// An id for a new voicebank from its name, unique in the Voices folder.
pub fn new_id(name: &str, folder: &Path) -> String {
    let base: String = name
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    let base = if base.is_empty() {
        "voice".to_string()
    } else {
        base.chars().take(40).collect()
    };
    let mut id = base.clone();
    let mut n = 2;
    while folder.join(format!("{id}.sadvoice")).exists() || bundled(&id).is_some() {
        id = format!("{base}-{n}");
        n += 1;
    }
    id
}

/// Saves a voicebank into the Voices folder and returns its listing entry.
pub fn store(
    bank: &voice::bank::Voicebank,
    name: Option<&str>,
) -> Result<serde_json::Value, String> {
    let folder = voices_folder().ok_or("No folder for voicebanks")?;
    std::fs::create_dir_all(&folder)
        .map_err(|e| format!("Cannot create the Voices folder: {e}"))?;
    let id = new_id(name.unwrap_or(&bank.info.name), &folder);
    super::audio_library::write_file(&folder.join(format!("{id}.sadvoice")), &bank.to_bytes())?;
    Ok(
        serde_json::json!({ "id": id, "name": bank.info.name, "language": bank.info.language,
        "units": bank.units.len(), "author": bank.info.author }),
    )
}

/// Imports a voicebank from a folder (UTAU), a `.zip` of one, or a `.sadvoice`.
pub fn import_path(
    path: &Path,
    options: &voice::utau::ImportOptions,
) -> Result<voice::bank::Voicebank, String> {
    if path.is_dir() {
        return voice::utau::import(
            &voice::utau::Directory(path.to_path_buf()),
            options,
            &mut |_, _| {},
        );
    }
    let bytes = std::fs::read(path).map_err(|e| format!("Cannot read {}: {e}", path.display()))?;
    if bytes.starts_with(b"SADVOICE") {
        return voice::bank::Voicebank::from_bytes(&bytes);
    }
    if bytes.starts_with(b"PK") {
        let files = voice::zip::unzip(&bytes)?;
        return voice::utau::import(&files, options, &mut |_, _| {});
    }
    Err("Choose a UTAU voicebank folder, a .zip of one, or a .sadvoice file".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_into_the_mailbox_off_the_audio_thread() {
        let service = Service::new();
        let mailbox = Mailbox::new();
        let mut settings = vocal::Settings::default();
        settings.score.notes.push(voice::score::Note {
            id: 1,
            start: 0,
            length: 480,
            key: 62,
            lyric: "a".into(),
            ..Default::default()
        });
        service.submit(7, &settings, 16_000, &mailbox);
        let mut track = vocal::Track::new(mailbox.clone());
        let started = std::time::Instant::now();
        loop {
            track.update();
            if !track.song.clips.is_empty() {
                break;
            }
            assert!(
                started.elapsed() < Duration::from_secs(20),
                "nothing rendered"
            );
            std::thread::sleep(Duration::from_millis(5));
        }
        let mut out = vec![0.0; 16_000];
        track.fill(0, &mut out);
        assert!(out.iter().any(|s| s.abs() > 0.01));
        assert_eq!(service.progress(7).phrases, 1);
        service.forget(7);
    }

    #[test]
    fn the_voices_that_come_with_sad_pet_are_listed_and_load() {
        for ((id, bytes), (pack, language)) in BUNDLED.iter().zip(vocal::PACKS) {
            assert_eq!(*id, pack);
            let info = voice::bank::Voicebank::read_info(bytes).unwrap();
            assert_eq!(info.language, language, "{id}");
            assert!(!info.author.is_empty(), "{id} credits its author");
        }
        let listed = list_voices();
        assert_eq!(listed[0]["id"], "kasane-teto");
        assert_eq!(listed[0]["bundled"], true);
        let mut renderer = Renderer::new();
        load_voice(&mut renderer, "kumi-ru");
        assert!(renderer.has_voice("kumi-ru"));
        assert_eq!(
            vocal::Settings::new().score.voice,
            "kasane-teto",
            "a new Vocal timbre sings with a voice that comes with SAD PET"
        );
    }

    #[test]
    fn zip_entries_inflate() {
        use std::io::Write;
        // A one-file zip with a deflated entry, built by hand.
        let content = b"a.wav=- a,0,10,-100,5,0\n".repeat(4);
        let mut deflated = Vec::new();
        let mut encoder =
            flate2::write::DeflateEncoder::new(&mut deflated, flate2::Compression::default());
        encoder.write_all(&content).unwrap();
        encoder.finish().unwrap();
        let name = b"bank/oto.ini";
        let mut zip = Vec::new();
        let local = zip.len() as u32;
        zip.extend_from_slice(&0x0403_4b50u32.to_le_bytes());
        zip.extend_from_slice(&[20, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        zip.extend_from_slice(&(deflated.len() as u32).to_le_bytes());
        zip.extend_from_slice(&(content.len() as u32).to_le_bytes());
        zip.extend_from_slice(&(name.len() as u16).to_le_bytes());
        zip.extend_from_slice(&0u16.to_le_bytes());
        zip.extend_from_slice(name);
        zip.extend_from_slice(&deflated);
        let central = zip.len() as u32;
        zip.extend_from_slice(&0x0201_4b50u32.to_le_bytes());
        zip.extend_from_slice(&[20, 0, 20, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        zip.extend_from_slice(&(deflated.len() as u32).to_le_bytes());
        zip.extend_from_slice(&(content.len() as u32).to_le_bytes());
        zip.extend_from_slice(&(name.len() as u16).to_le_bytes());
        zip.extend_from_slice(&[0; 12]);
        zip.extend_from_slice(&local.to_le_bytes());
        zip.extend_from_slice(name);
        let size = zip.len() as u32 - central;
        zip.extend_from_slice(&0x0605_4b50u32.to_le_bytes());
        zip.extend_from_slice(&[0, 0, 0, 0, 1, 0, 1, 0]);
        zip.extend_from_slice(&size.to_le_bytes());
        zip.extend_from_slice(&central.to_le_bytes());
        zip.extend_from_slice(&[0, 0]);
        let files = voice::zip::unzip(&zip).unwrap();
        assert_eq!(files.0.get("bank/oto.ini").unwrap(), &content);
    }
}
