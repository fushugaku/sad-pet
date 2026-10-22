use crate::{
    audio::{Command, Runtime, State, Transport},
    dsp::Engine,
    midi,
    preset::Preset,
    wav,
};
use std::{
    io::{Cursor, Read},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
        mpsc::SyncSender,
    },
    thread,
    time::Duration,
};
use tiny_http::{Header, Method, Request, Response, Server};

type ApiResult = Result<(Vec<u8>, &'static str), String>;
const MAX_BODY: u64 = 128 * 1024 * 1024;

fn header(name: &str, value: &str) -> Header {
    Header::from_bytes(name, value).expect("valid fixed HTTP header")
}

fn response(request: Request, status: u16, bytes: Vec<u8>, mime: &'static str) {
    let mut response = Response::from_data(bytes)
        .with_status_code(status)
        .with_header(header("Content-Type", mime))
        .with_header(header("Cache-Control", "no-store"))
        .with_header(header("X-Content-Type-Options", "nosniff"));
    if mime == "audio/wav" {
        response = response.with_header(header(
            "Content-Disposition",
            "attachment; filename=sad-pet-render.wav",
        ));
    }
    if mime == "audio/mpeg" {
        response = response.with_header(header(
            "Content-Disposition",
            "attachment; filename=sad-pet-recording.mp3",
        ));
    }
    if mime.starts_with("text/html") {
        response=response.with_header(header("Content-Security-Policy","default-src 'self'; script-src 'self'; style-src 'self'; connect-src 'self'; img-src 'self' data:; object-src 'none'; base-uri 'none'; frame-ancestors 'none'"));
    }
    let _ = request.respond(response);
}

fn send(sender: &SyncSender<Command>, command: Command) -> Result<(), String> {
    sender
        .try_send(command)
        .map_err(|e| format!("Audio command queue: {e}"))
}
fn json<T: serde::Serialize>(value: &T) -> ApiResult {
    Ok((
        serde_json::to_vec(value).map_err(|e| e.to_string())?,
        "application/json",
    ))
}

fn decode_name(name: &str) -> String {
    let mut result = Vec::new();
    let mut bytes = name.as_bytes().iter().copied();
    while let Some(byte) = bytes.next() {
        if byte == b'%' {
            if let (Some(a), Some(b)) = (bytes.next(), bytes.next())
                && let (Some(a), Some(b)) = ((a as char).to_digit(16), (b as char).to_digit(16))
            {
                result.push((a * 16 + b) as u8);
            }
        } else {
            result.push(byte);
        }
    }
    String::from_utf8_lossy(&result)
        .chars()
        .filter(|c| !c.is_control())
        .take(150)
        .collect()
}

fn handle(
    mut request: Request,
    commands: SyncSender<Command>,
    state: Arc<Mutex<State>>,
    connection: Arc<Mutex<Option<midi::Connection>>>,
    port: u16,
    running: Arc<AtomicBool>,
) {
    let host = request
        .headers()
        .iter()
        .find(|h| h.field.equiv("Host"))
        .map(|h| h.value.as_str())
        .unwrap_or("");
    let allowed_host = host == format!("127.0.0.1:{port}") || host == format!("localhost:{port}");
    let origin = request
        .headers()
        .iter()
        .find(|h| h.field.equiv("Origin"))
        .map(|h| h.value.as_str());
    let allowed_origin = origin.is_none_or(|o| {
        o == format!("http://127.0.0.1:{port}") || o == format!("http://localhost:{port}")
    });
    if !allowed_host || !allowed_origin {
        response(
            request,
            403,
            b"Local origin required".to_vec(),
            "text/plain",
        );
        return;
    }
    let url = request.url().to_owned();
    let method = request.method().clone();
    let path = url.split('?').next().unwrap_or("/");
    let result: ApiResult = (|| {
        if method == Method::Get {
            return match path {
                "/" => Ok((
                    include_bytes!("../web/index.html").to_vec(),
                    "text/html; charset=utf-8",
                )),
                "/style.css" => Ok((
                    include_bytes!("../web/style.css").to_vec(),
                    "text/css; charset=utf-8",
                )),
                "/app.js" => Ok((
                    include_bytes!("../web/app.js").to_vec(),
                    "text/javascript; charset=utf-8",
                )),
                "/library.js" => Ok((
                    include_bytes!("../web/library.js").to_vec(),
                    "text/javascript; charset=utf-8",
                )),
                "/selection.js" => Ok((
                    include_bytes!("../web/selection.js").to_vec(),
                    "application/javascript",
                )),
                "/presets.js" => Ok((
                    include_bytes!("../web/presets.js").to_vec(),
                    "application/javascript",
                )),
                "/sequence.js" => Ok((
                    include_bytes!("../web/sequence.js").to_vec(),
                    "application/javascript",
                )),
                "/panels.js" => Ok((
                    include_bytes!("../web/panels.js").to_vec(),
                    "application/javascript",
                )),
                "/vocal.js" => Ok((
                    include_bytes!("../web/vocal.js").to_vec(),
                    "application/javascript",
                )),
                "/vocal.css" => Ok((
                    include_bytes!("../web/vocal.css").to_vec(),
                    "text/css; charset=utf-8",
                )),
                "/api/sequencer/catalog" => json(&crate::sequence::catalog()),
                "/api/drums/catalog" => json(&crate::drums::catalog()),
                "/favicon.ico" => Ok((Vec::new(), "image/x-icon")),
                "/api/state" => json(&*state.lock().map_err(|e| e.to_string())?),
                "/api/preset" => json(&state.lock().map_err(|e| e.to_string())?.preset),
                "/api/presets" => json(&(0..5).map(Preset::builtin).collect::<Vec<_>>()),
                "/api/samples" => json(&crate::samples::TRACKS),
                "/api/sample.wav" => {
                    let sample = state.lock().map_err(|e| e.to_string())?.sample.clone();
                    Ok((
                        wav::bytes(sample.sample_rate, sample.frames())?,
                        "audio/wav",
                    ))
                }
                "/api/recording.wav" | "/api/recording.mp3" => {
                    let take = state
                        .lock()
                        .map_err(|e| e.to_string())?
                        .recording_take
                        .clone()
                        .ok_or("There is no completed recording yet.")?;
                    if url.split_once("?id=").is_some_and(|(_, id)| id != take.id) {
                        return Err("This recording is no longer in the engine. Open its saved copy in Library.".into());
                    }
                    if path.ends_with(".mp3") {
                        Ok((
                            crate::recording::mp3(take.sample_rate, &take.frames)?,
                            "audio/mpeg",
                        ))
                    } else {
                        Ok((wav::bytes(take.sample_rate, &take.frames)?, "audio/wav"))
                    }
                }
                "/api/effect-presets/factory" => json(&crate::effect_presets::factory()),
                "/api/color-defaults" => json(
                    &serde_json::json!({"max_slots": crate::color::MAX_COLOR_SLOTS, "slot": crate::color::ColorSlot::default(), "delay_tables": crate::radias_delay::display_tables()}),
                ),
                "/api/midi" => json(
                    &serde_json::json!({"ports":midi::ports()?,"connected":connection.lock().map_err(|e|e.to_string())?.as_ref().map(|c|&c.name)}),
                ),
                _ => {
                    if let Some(id) = path
                        .strip_prefix("/api/samples/")
                        .and_then(|s| s.strip_suffix(".wav"))
                    {
                        let sample = crate::samples::get(id).ok_or("Route not found")?;
                        Ok((
                            wav::bytes(sample.sample_rate, sample.frames())?,
                            "audio/wav",
                        ))
                    } else {
                        Err("Route not found".into())
                    }
                }
            };
        }
        if method != Method::Post {
            return Err("Method not allowed".into());
        }
        if request
            .body_length()
            .is_some_and(|len| len as u64 > MAX_BODY)
        {
            return Err("File exceeds 128 MB".into());
        }
        let mut bytes = Vec::new();
        request
            .as_reader()
            .take(MAX_BODY + 1)
            .read_to_end(&mut bytes)
            .map_err(|e| e.to_string())?;
        if bytes.len() as u64 > MAX_BODY {
            return Err("File exceeds 128 MB".into());
        }
        match path {
            "/api/quit" => {
                running.store(false, Ordering::Relaxed);
                json(&serde_json::json!({"ok":true}))
            }
            "/api/parameters" => {
                let data =
                    serde_json::from_slice(&bytes).map_err(|e| format!("Parameters: {e}"))?;
                let changes = crate::parameters::parse(&data)?;
                let (reply, result) = std::sync::mpsc::sync_channel(1);
                send(&commands, Command::Parameters(changes, reply))?;
                result
                    .recv_timeout(Duration::from_secs(5))
                    .map_err(|e| format!("Parameters: {e}"))?;
                json(&serde_json::json!({"ok":true}))
            }
            "/api/sequencer/edit" => {
                let value =
                    serde_json::from_slice(&bytes).map_err(|e| format!("Sequencer: {e}"))?;
                let edits = crate::sequence::parse_edits(&value)?;
                let (reply, result) = std::sync::mpsc::sync_channel(1);
                send(&commands, Command::Sequence(edits, reply))?;
                result
                    .recv_timeout(Duration::from_secs(5))
                    .map_err(|e| e.to_string())?;
                json(&serde_json::json!({"ok":true}))
            }
            "/api/sequencer/view" => {
                let value: serde_json::Value =
                    serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
                send(
                    &commands,
                    Command::SequenceView(value["offset"].as_u64().unwrap_or(0) as usize),
                )?;
                json(&serde_json::json!({"ok":true}))
            }
            "/api/preset" => {
                let value: serde_json::Value =
                    serde_json::from_slice(&bytes).map_err(|e| format!("Preset: {e}"))?;
                if value.get("format").and_then(|v| v.as_str()) != Some("evil-pet-rust-v1") {
                    return Err("Use an SAD PET preset (.sad.json); original .evl presets require a separate compatibility pass".into());
                }
                let mut preset: Preset =
                    serde_json::from_value(value).map_err(|e| format!("Preset: {e}"))?;
                // The desktop app has one engine and no buses or voices to feed one.
                preset.bus = None;
                preset.vocal = None;
                preset.sanitize();
                let rate = state.lock().map_err(|e| e.to_string())?.sample_rate;
                let (reply, result) = std::sync::mpsc::sync_channel(1);
                send(
                    &commands,
                    Command::preset(preset.prepared_clone_for(rate), reply),
                )?;
                result
                    .recv_timeout(Duration::from_secs(5))
                    .map_err(|e| format!("Preset: {e}"))?;
                json(&preset)
            }
            "/api/transport" => {
                let t: Transport = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
                send(&commands, Command::Transport(t))?;
                json(&serde_json::json!({"ok":true}))
            }
            "/api/recording" => {
                #[derive(serde::Deserialize)]
                struct Recording {
                    active: bool,
                }
                let value: Recording = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
                let (reply, result) = std::sync::mpsc::sync_channel(1);
                send(&commands, Command::Recording(value.active, reply))?;
                let view = result
                    .recv_timeout(Duration::from_secs(5))
                    .map_err(|e| format!("Recording: {e}"))??;
                json(&view)
            }
            "/api/wav" => {
                let name = request
                    .headers()
                    .iter()
                    .find(|h| h.field.equiv("X-File-Name"))
                    .map(|h| h.value.as_str().to_owned())
                    .unwrap_or_else(|| "Sample.wav".into());
                let mut sample = wav::decode(Cursor::new(bytes), decode_name(&name))?;
                sample.library_id = request
                    .headers()
                    .iter()
                    .find(|h| h.field.equiv("X-Library-Id"))
                    .map(|h| h.value.as_str())
                    .filter(|id| {
                        id.len() <= 160 && id.is_ascii() && !id.chars().any(char::is_control)
                    })
                    .map(str::to_owned);
                let result = serde_json::json!({"name":sample.name,"id":sample.library_id,"seconds":sample.duration()});
                let sample = Arc::new(sample);
                send(&commands, Command::Sample(sample))?;
                json(&result)
            }
            "/api/region" => {
                let region: crate::dsp::Region =
                    serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
                let (reply, result) = std::sync::mpsc::sync_channel(1);
                send(&commands, Command::Region(region, reply))?;
                let region = result
                    .recv_timeout(Duration::from_secs(5))
                    .map_err(|e| e.to_string())??;
                json(&region)
            }
            "/api/trim" => {
                let value: serde_json::Value =
                    serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
                let region: crate::dsp::Region =
                    serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
                let sample = state.lock().map_err(|e| e.to_string())?.sample.clone();
                if value
                    .get("sample_id")
                    .is_some_and(|id| id.as_str() != sample.library_id.as_deref())
                {
                    return Err("The audio source changed; select the region again".into());
                }
                let fades = serde_json::from_value(
                    value
                        .get("fades")
                        .cloned()
                        .unwrap_or_else(|| serde_json::json!({})),
                )
                .map_err(|e| e.to_string())?;
                Ok((sample.trim_wav_with_fades(region, fades)?, "audio/wav"))
            }
            "/api/fades" => {
                let fades: crate::dsp::Fades =
                    serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
                let (reply, result) = std::sync::mpsc::sync_channel(1);
                send(&commands, Command::Fades(fades, reply))?;
                let fades = result
                    .recv_timeout(Duration::from_secs(5))
                    .map_err(|e| e.to_string())??;
                json(&fades)
            }
            "/api/demo" => {
                send(
                    &commands,
                    Command::Sample(crate::samples::get("glass-sequence").unwrap()),
                )?;
                json(&serde_json::json!({"ok":true}))
            }
            "/api/note" => {
                let message: midi::Message =
                    serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
                send(&commands, Command::Midi(message))?;
                json(&serde_json::json!({"ok":true}))
            }
            "/api/step-entry" => {
                #[derive(serde::Deserialize)]
                struct Entry {
                    step: Option<usize>,
                    #[serde(default)]
                    reset: bool,
                    #[serde(default)]
                    clear: bool,
                }
                let entry: Entry = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
                if entry.step.is_some_and(|s| s >= crate::performance::STEPS) {
                    return Err("Step must be between 0 and 15".into());
                }
                send(
                    &commands,
                    Command::StepEntry(entry.step, entry.reset, entry.clear),
                )?;
                json(&serde_json::json!({"ok":true}))
            }
            "/api/midi" => {
                let value: serde_json::Value =
                    serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
                let mut current = connection.lock().map_err(|e| e.to_string())?;
                *current = None;
                send(
                    &commands,
                    Command::Transport(Transport {
                        midi_gate: Some(false),
                        ..Transport::default()
                    }),
                )?;
                if let Some(id) = value.get("port").and_then(|v| v.as_u64()) {
                    *current = Some(midi::Connection::open(id as usize, commands.clone())?);
                    send(
                        &commands,
                        Command::Transport(Transport {
                            midi_gate: Some(true),
                            ..Transport::default()
                        }),
                    )?;
                }
                json(&serde_json::json!({"connected":current.as_ref().map(|c|&c.name)}))
            }
            "/api/render" => {
                let value: serde_json::Value =
                    serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
                let seconds = value
                    .get("seconds")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(10.0);
                if !seconds.is_finite() || !(0.1..=120.0).contains(&seconds) {
                    return Err("Render duration must be 0.1–120 seconds".into());
                }
                let snapshot = state.lock().map_err(|e| e.to_string())?.clone();
                let frames = render(&snapshot, seconds);
                Ok((wav::bytes(snapshot.sample_rate, &frames)?, "audio/wav"))
            }
            _ => Err("Route not found".into()),
        }
    })();
    match result {
        Ok((bytes, mime)) => response(request, 200, bytes, mime),
        Err(error) => {
            let status = if error == "Route not found" { 404 } else { 400 };
            response(
                request,
                status,
                serde_json::to_vec(&serde_json::json!({"error":error})).unwrap(),
                "application/json",
            );
        }
    }
}

pub fn render(snapshot: &crate::audio::State, seconds: f64) -> Vec<crate::Stereo> {
    let mut engine = Engine::new(
        snapshot.sample_rate,
        snapshot.sample.clone(),
        snapshot.preset.clone(),
    );
    let _ = engine.set_region(snapshot.region);
    let _ = engine.set_fades(snapshot.fades);
    engine.bypass = snapshot.bypass;
    engine.set_frozen(snapshot.frozen);
    engine.set_playing(true);
    let mut frames = vec![[0.0; 2]; (seconds * snapshot.sample_rate as f64).round() as usize];
    engine.process(&mut frames);
    frames
}

pub fn serve(port: u16, audio: bool, open: bool) -> Result<(), String> {
    let server = Server::http(("127.0.0.1", port))
        .map_err(|e| format!("Cannot open local panel port {port}: {e}"))?;
    let runtime = Runtime::new(
        crate::samples::get("glass-sequence").unwrap(),
        Preset::default(),
        audio,
    );
    if let Some(error) = &runtime.state.lock().unwrap().audio_error {
        eprintln!("Audio: {error}");
    }
    let connection = Arc::new(Mutex::new(None));
    let url = format!("http://127.0.0.1:{port}");
    println!("SAD PET · {url}\nSpace: play/pause · A–K: notes · Ctrl+C: quit");
    if open {
        open_browser(&url);
    }
    let running = Arc::new(AtomicBool::new(true));
    let signal = running.clone();
    ctrlc::set_handler(move || {
        signal.store(false, Ordering::Relaxed);
    })
    .map_err(|e| e.to_string())?;
    while running.load(Ordering::Relaxed) {
        if let Some(request) = server
            .recv_timeout(Duration::from_millis(250))
            .map_err(|e| e.to_string())?
        {
            let commands = runtime.commands.clone();
            let state = runtime.state.clone();
            let connection = connection.clone();
            let running = running.clone();
            thread::spawn(move || handle(request, commands, state, connection, port, running));
        }
    }
    Ok(())
}

fn open_browser(url: &str) {
    #[cfg(target_os = "macos")]
    let result = std::process::Command::new("open").arg(url).spawn();
    #[cfg(target_os = "windows")]
    let result = std::process::Command::new("cmd")
        .args(["/C", "start", "", url])
        .spawn();
    #[cfg(all(unix, not(target_os = "macos")))]
    let result = std::process::Command::new("xdg-open").arg(url).spawn();
    let _ = result;
}
