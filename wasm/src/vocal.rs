//! Vocal timbres in the browser. The worklet's registry keeps each Vocal
//! part's rendered clips and mixes them at the scene's song clock; a
//! dedicated Worker runs the `sad_vocal_*` ABI below, which plans and renders
//! phrases and imports voicebanks. Clip keys fit in 53 bits, so JavaScript
//! numbers carry them exactly.
use super::*;
use sad_pet::vocal::{self, Clip, Job, Plan, Renderer, Song, voice};
use std::collections::BTreeMap;

/// The worklet's state for one Vocal part.
pub(super) struct VocalPart {
    pub song: Song,
    pub buffer: Box<[f32; BLOCK]>,
    /// The settings last sent to the render worker.
    pub sent: Option<vocal::Settings>,
}
impl VocalPart {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            song: Song::default(),
            buffer: Box::new([0.0; BLOCK]),
            sent: None,
        })
    }
}

impl super::timbres::Registry {
    /// Creates or drops each part's vocal state and queues a render job for
    /// every score the worker has not seen.
    pub fn sync_vocal(&mut self) {
        let rate = self.parts[0].host.rate;
        let mut present = Vec::new();
        let mut jobs = Vec::new();
        for part in &mut self.parts {
            let settings = part
                .host
                .engine
                .preset
                .vocal
                .as_deref()
                .filter(|_| part.meta.kind == sad_pet::timbres::Kind::Vocal);
            let Some(settings) = settings else {
                part.vocal = None;
                continue;
            };
            present.push(part.meta.id);
            // A new part has no clips yet: the worker sends all of them again.
            let fresh = part.vocal.is_none();
            let state = part.vocal.get_or_insert_with(VocalPart::new);
            if state.sent.as_ref() != Some(settings) || fresh {
                state.sent = Some(settings.clone());
                jobs.push(
                    json!({ "type": "job", "id": part.meta.id, "settings": settings,
                    "rate": rate, "fresh": fresh }),
                );
            }
        }
        for id in &self.vocal_ids {
            if !present.contains(id) {
                jobs.push(json!({ "type": "forget", "id": id }));
            }
        }
        self.vocal_ids = present;
        self.vocal_jobs.extend(jobs);
    }

    pub fn vocal_command(&mut self, path: &str, data: &Value) -> Result<Value, String> {
        let rate = self.parts[0].host.rate as f64;
        let index = self.index();
        let result = match path {
            "/api/vocal/score" => {
                let part = &mut self.parts[index];
                let settings = part
                    .host
                    .engine
                    .preset
                    .vocal
                    .as_deref_mut()
                    .ok_or("Select a Vocal timbre")?;
                let mut score: voice::score::Score =
                    serde_json::from_value(data["score"].clone()).map_err(|e| e.to_string())?;
                score.sanitize();
                settings.score = score;
                let reply = json!({ "ok": true, "score": settings.score });
                part.host.config_revision = part.host.config_revision.wrapping_add(1);
                part.host.resize_snapshot();
                reply
            }
            "/api/vocal/transport" => {
                if let Some(start) = data["start"].as_f64() {
                    self.clock.start = (start.max(0.0) * rate) as u64;
                    if !self.clock.playing {
                        self.clock.position = self.clock.start;
                    }
                }
                if let Some(playing) = data["playing"].as_bool() {
                    self.clock.play(playing);
                }
                json!({ "ok": true })
            }
            "/api/vocal/state" => {
                let clips = self.parts[index]
                    .vocal
                    .as_ref()
                    .map_or(0, |v| v.song.clips.len());
                json!({
                    "playing": self.clock.playing,
                    "position": self.clock.position as f64 / rate,
                    "start": self.clock.start as f64 / rate,
                    "clips": clips,
                    "rate": rate,
                })
            }
            _ => return Err("Unknown vocal command".into()),
        };
        self.sync_vocal();
        Ok(result)
    }
}

/// Render-worker messages queued by the registry, as a JSON array.
#[unsafe(no_mangle)]
pub extern "C" fn sad_vocal_jobs() -> u32 {
    registry(|r| {
        let jobs = std::mem::take(&mut r.vocal_jobs);
        let index = r.index();
        r.parts[index]
            .host
            .finish(serde_json::to_vec(&jobs).map_err(|e| e.to_string()))
    })
}

/// A clip being uploaded by the worklet.
struct Staged {
    part: u32,
    key: u64,
    start: u64,
    samples: Vec<f32>,
}
thread_local! {
    static STAGED: RefCell<Option<Staged>> = const { RefCell::new(None) };
    static PLAN: RefCell<Vec<f64>> = const { RefCell::new(Vec::new()) };
    /// Clips for the next offline scene render, by part id.
    static RENDER_CLIPS: RefCell<Vec<(u32, Clip)>> = const { RefCell::new(Vec::new()) };
}

/// Room for one rendered clip of the next offline scene render (Render WAV):
/// the utility worker has no voicebanks, so the vocal worker's clips come in.
#[unsafe(no_mangle)]
pub extern "C" fn sad_render_clip(part: u32, start: f64, length: usize) -> *mut f32 {
    let mut samples: Arc<[f32]> = vec![0.0; length.min(64 << 20)].into();
    let pointer = Arc::get_mut(&mut samples).map_or(std::ptr::null_mut(), |s| s.as_mut_ptr());
    RENDER_CLIPS.with(|c| {
        c.borrow_mut().push((
            part,
            Clip {
                key: 0,
                start: start as u64,
                samples,
            },
        ))
    });
    pointer
}

impl super::timbres::Registry {
    /// Gives an offline render's Vocal parts the uploaded clips and plays
    /// the song from its start.
    pub fn adopt_render_clips(&mut self) {
        let clips = RENDER_CLIPS.with(|c| std::mem::take(&mut *c.borrow_mut()));
        self.sync_vocal();
        self.vocal_jobs.clear();
        for part in &mut self.parts {
            if let Some(vocal) = &mut part.vocal {
                vocal.song = Song::new(
                    clips
                        .iter()
                        .filter(|(id, _)| *id == part.meta.id)
                        .map(|(_, c)| c.clone())
                        .collect(),
                );
            }
        }
        self.clock = vocal::Clock {
            playing: true,
            position: 0,
            start: 0,
        };
    }
}

/// Every rendered clip of every part: `[{id, clips: [[key, start, length]]}]`.
#[unsafe(no_mangle)]
pub extern "C" fn sad_vocal_songs() -> u32 {
    worker(|w| {
        let songs: Vec<Value> = w
            .jobs
            .iter()
            .map(|(id, (_, plan))| {
                let song = w.renderer.song(plan);
                json!({ "id": id, "clips": song.clips.iter()
                    .map(|c| [c.key as f64, c.start as f64, c.samples.len() as f64]).collect::<Vec<_>>() })
            })
            .collect();
        let result = serde_json::to_vec(&songs).map_err(|e| e.to_string());
        reply(w, result)
    })
}

/// Scratch for a plan of `count` clips: `[key, start, length]` triples.
#[unsafe(no_mangle)]
pub extern "C" fn sad_vocal_plan(count: usize) -> *mut f64 {
    PLAN.with(|p| {
        let mut p = p.borrow_mut();
        p.clear();
        p.resize(count.min(1 << 20) * 3, 0.0);
        p.as_mut_ptr()
    })
}
/// Keeps the part's clips that are in the plan (moving them if their start
/// changed) and drops the rest. Returns how many planned clips are missing.
#[unsafe(no_mangle)]
pub extern "C" fn sad_vocal_plan_commit(part: u32, count: usize) -> usize {
    let plan: Vec<(u64, u64)> = PLAN.with(|p| {
        p.borrow()
            .as_chunks::<3>()
            .0
            .iter()
            .take(count)
            .map(|t| (t[0] as u64, t[1] as u64))
            .collect()
    });
    registry(|r| {
        let Some(state) = r
            .parts
            .iter_mut()
            .find(|p| p.meta.id == part)
            .and_then(|p| p.vocal.as_mut())
        else {
            return count;
        };
        let mut clips: Vec<Clip> = Vec::with_capacity(plan.len());
        for &(key, start) in &plan {
            if let Some(clip) = state.song.clips.iter().find(|c| c.key == key) {
                clips.push(Clip {
                    key,
                    start,
                    samples: clip.samples.clone(),
                });
            }
        }
        let missing = plan.len() - clips.len();
        state.song = Song::new(clips);
        missing
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_vocal_clip_begin(part: u32, key: f64, start: f64, length: usize) -> *mut f32 {
    STAGED.with(|s| {
        let mut staged = Staged {
            part,
            key: key as u64,
            start: start as u64,
            samples: vec![0.0; length.min(64 << 20)],
        };
        let pointer = staged.samples.as_mut_ptr();
        *s.borrow_mut() = Some(staged);
        pointer
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_vocal_clip_commit() -> u32 {
    let Some(staged) = STAGED.with(|s| s.borrow_mut().take()) else {
        return 0;
    };
    registry(|r| {
        let Some(state) = r
            .parts
            .iter_mut()
            .find(|p| p.meta.id == staged.part)
            .and_then(|p| p.vocal.as_mut())
        else {
            return 0;
        };
        let mut clips = std::mem::take(&mut state.song.clips);
        clips.retain(|c| c.key != staged.key);
        clips.push(Clip {
            key: staged.key,
            start: staged.start,
            samples: staged.samples.into(),
        });
        state.song = Song::new(clips);
        1
    })
}

// --- The render worker ------------------------------------------------------

#[derive(Default)]
struct Worker {
    rate: u32,
    input: Vec<u8>,
    output: Vec<u8>,
    renderer: Renderer,
    jobs: BTreeMap<u32, (Job, Plan)>,
    /// Keys each part has already received, so only new clips are sent.
    delivered: BTreeMap<u32, std::collections::HashSet<u64>>,
    import: Option<Import>,
}
struct Import {
    files: voice::utau::MemoryFiles,
    options: voice::utau::ImportOptions,
}

thread_local! { static WORKER: RefCell<Option<Worker>> = const { RefCell::new(None) }; }
fn worker<T>(f: impl FnOnce(&mut Worker) -> T) -> T {
    WORKER.with(|w| f(w.borrow_mut().get_or_insert_with(Worker::default)))
}
fn reply(w: &mut Worker, result: Result<Vec<u8>, String>) -> u32 {
    match result {
        Ok(bytes) => {
            w.output = bytes;
            200
        }
        Err(error) => {
            w.output = serde_json::to_vec(&json!({ "error": error })).unwrap_or_default();
            400
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sad_vocal_init(rate: u32) {
    worker(|w| w.rate = rate.clamp(8_000, 384_000));
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_vocal_input(length: usize) -> *mut u8 {
    worker(|w| {
        if length > MAX_INPUT {
            return std::ptr::null_mut();
        }
        w.input.resize(length, 0);
        w.input.as_mut_ptr()
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_vocal_output() -> *const u8 {
    worker(|w| w.output.as_ptr())
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_vocal_output_len() -> usize {
    worker(|w| w.output.len())
}

fn worker_command(w: &mut Worker, request: Value) -> Result<Value, String> {
    match request["type"].as_str().unwrap_or("") {
        "job" => {
            let id = request["id"].as_u64().ok_or("Missing timbre")? as u32;
            let mut settings: vocal::Settings =
                serde_json::from_value(request["settings"].clone()).map_err(|e| e.to_string())?;
            settings.sanitize();
            let job = Job {
                settings,
                rate: request["rate"].as_u64().map_or(w.rate, |r| r as u32),
                host_bpm: request["host_bpm"].as_f64().map(|b| b as f32),
            };
            let plan = w.renderer.plan(&job);
            w.jobs.insert(id, (job, plan));
            if request["fresh"] == true {
                w.delivered.remove(&id);
            }
            let plans: Vec<&Plan> = w.jobs.values().map(|(_, p)| p).collect();
            w.renderer.retain(&plans);
            Ok(json!({ "ok": true }))
        }
        "forget" => {
            let id = request["id"].as_u64().ok_or("Missing timbre")? as u32;
            w.jobs.remove(&id);
            w.delivered.remove(&id);
            Ok(json!({ "ok": true }))
        }
        // The worklet lost its clips (a new scene or audio context).
        "reset" => {
            w.delivered.clear();
            Ok(json!({ "ok": true }))
        }
        "voice-remove" => {
            let id = request["id"].as_str().ok_or("Missing voice")?;
            w.renderer.remove_voice(id);
            replan(w);
            Ok(json!({ "ok": true }))
        }
        "voices" => {
            Ok(json!(
            w.renderer
                .voices()
                .map(|(id, b)| json!({ "id": id, "name": b.info.name, "language": b.info.language,
                    "units": b.units.len(), "author": b.info.author }))
                .collect::<Vec<_>>()
        ))
        }
        "progress" => Ok(json!(
            w.jobs
                .iter()
                .map(|(id, (job, plan))| (id.to_string(), json!(w.renderer.progress(job, plan))))
                .collect::<serde_json::Map<_, _>>()
        )),
        "phonemes" => {
            // Lyrics as phonemes, for the editor's phoneme row.
            let language = voice::Language::parse(request["language"].as_str().unwrap_or("ja"))
                .unwrap_or_default();
            let lyrics: Vec<String> =
                serde_json::from_value(request["lyrics"].clone()).unwrap_or_default();
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
        "import-score" => {
            use base64::Engine as _;
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(request["base64"].as_str().unwrap_or(""))
                .map_err(|e| e.to_string())?;
            let language = request["language"]
                .as_str()
                .and_then(voice::Language::parse)
                .unwrap_or_default();
            let score =
                voice::import::score(request["name"].as_str().unwrap_or(""), &bytes, language)?;
            Ok(json!({ "score": score }))
        }
        "import-begin" => {
            w.import = Some(Import {
                files: Default::default(),
                options: voice::utau::ImportOptions {
                    language: request["language"]
                        .as_str()
                        .and_then(voice::Language::parse),
                    name: request["name"].as_str().map(str::to_string),
                },
            });
            Ok(json!({ "ok": true }))
        }
        _ => Err("Unknown vocal command".into()),
    }
}

fn replan(w: &mut Worker) {
    let ids: Vec<u32> = w.jobs.keys().copied().collect();
    for id in ids {
        let job = w.jobs[&id].0.clone();
        let plan = w.renderer.plan(&job);
        w.jobs.insert(id, (job, plan));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sad_vocal_command() -> u32 {
    worker(|w| {
        let result = serde_json::from_slice::<Value>(&w.input)
            .map_err(|e| e.to_string())
            .and_then(|v| worker_command(w, v))
            .and_then(|v| serde_json::to_vec(&v).map_err(|e| e.to_string()));
        reply(w, result)
    })
}

/// Loads a `.sadvoice` from the input; `id_length` bytes of id come first.
#[unsafe(no_mangle)]
pub extern "C" fn sad_vocal_voice(id_length: usize) -> u32 {
    worker(|w| {
        let input = std::mem::take(&mut w.input);
        let result = (|| {
            let id = std::str::from_utf8(input.get(..id_length).ok_or("Missing voice id")?)
                .map_err(|e| e.to_string())?
                .to_string();
            let bank = voice::bank::Voicebank::from_bytes(&input[id_length..])?;
            let info = json!({ "id": id, "name": bank.info.name, "language": bank.info.language,
                "units": bank.units.len(), "author": bank.info.author });
            w.renderer.add_voice(&id, Arc::new(bank));
            replan(w);
            serde_json::to_vec(&info).map_err(|e| e.to_string())
        })();
        reply(w, result)
    })
}

/// Adds one voicebank file to the import; `path_length` bytes of path first.
#[unsafe(no_mangle)]
pub extern "C" fn sad_vocal_import_file(path_length: usize) -> u32 {
    worker(|w| {
        let input = std::mem::take(&mut w.input);
        let result = (|| {
            let import = w.import.as_mut().ok_or("Start an import first")?;
            let path = String::from_utf8_lossy(input.get(..path_length).ok_or("Missing path")?)
                .replace('\\', "/");
            import.files.0.insert(path, input[path_length..].to_vec());
            Ok(b"{\"ok\":true}".to_vec())
        })();
        reply(w, result)
    })
}

/// Analyzes the collected files; the output is the `.sadvoice` file.
#[unsafe(no_mangle)]
pub extern "C" fn sad_vocal_import_finish() -> u32 {
    worker(|w| {
        let result = (|| {
            let import = w.import.take().ok_or("Start an import first")?;
            let bank = voice::utau::import(&import.files, &import.options, &mut |_, _| {})?;
            Ok(bank.to_bytes())
        })();
        reply(w, result)
    })
}

/// Renders the next missing phrase of any part, nearest `near` seconds first.
/// Returns 1 when something was rendered, 0 when everything is up to date.
#[unsafe(no_mangle)]
pub extern "C" fn sad_vocal_step(near: f64) -> u32 {
    worker(|w| {
        for (job, plan) in w.jobs.values() {
            let near = (near.max(0.0) * plan.rate as f64) as u64;
            if w.renderer.render_next(job, plan, near) {
                return 1;
            }
        }
        0
    })
}

/// Clips to send: per part, the plan as `[key, start, length]` and the keys
/// not delivered yet. Marks those keys delivered.
#[unsafe(no_mangle)]
pub extern "C" fn sad_vocal_updates() -> u32 {
    worker(|w| {
        let mut updates = Vec::new();
        for (&id, (_, plan)) in &w.jobs {
            let song = w.renderer.song(plan);
            let delivered = w.delivered.entry(id).or_default();
            let fresh: Vec<u64> = song
                .clips
                .iter()
                .map(|c| c.key)
                .filter(|k| !delivered.contains(k))
                .collect();
            let planned: Vec<u64> = plan.phrases.iter().map(|p| p.key).collect();
            // Forget delivered keys that left the plan; they may come back.
            delivered.retain(|k| planned.contains(k));
            delivered.extend(fresh.iter().copied());
            updates.push(json!({
                "id": id,
                "plan": song.clips.iter().map(|c| [c.key as f64, c.start as f64, c.samples.len() as f64]).collect::<Vec<_>>(),
                "fresh": fresh.iter().map(|&k| k as f64).collect::<Vec<_>>(),
            }));
        }
        let result = serde_json::to_vec(&updates).map_err(|e| e.to_string());
        reply(w, result)
    })
}

/// Copies one rendered clip into the output (raw little-endian f32).
#[unsafe(no_mangle)]
pub extern "C" fn sad_vocal_clip(key: f64) -> u32 {
    worker(|w| {
        let result = w
            .renderer
            .cached(key as u64)
            .map(|samples| samples.iter().flat_map(|s| s.to_le_bytes()).collect())
            .ok_or_else(|| "Unknown clip".to_string());
        reply(w, result)
    })
}
