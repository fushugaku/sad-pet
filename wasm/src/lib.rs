//! A small, dependency-free WebAssembly ABI shared by the audio worklet and
//! utility worker. All synthesis uses the same Engine as the desktop build.
#[cfg(test)]
mod allocation_tests;
mod clock;
mod sampler;
mod source_fx;
mod telemetry;
mod timbres;
mod vocal;
use sad_pet::{
    Stereo,
    dsp::{Engine, SampleBuffer},
    midi::Message,
    preset::Preset,
    wav,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{cell::RefCell, io::Cursor, sync::Arc};

const MAX_INPUT: usize = 300 * 1024 * 1024;
const BLOCK: usize = 4096;

struct Host {
    context: sad_pet::timbres::Context,
    engine: Box<Engine>,
    midi: sad_pet::midi::Parser,
    rate: u32,
    input: Vec<u8>,
    output: Vec<u8>,
    frames: Box<[Stereo; BLOCK]>,
    audio_input: Box<[Stereo; BLOCK]>,
    status: u32,
    view: sad_pet::dsp::telemetry::RealtimeView,
    snapshot_buffer: Vec<u8>,
    sequence_edits: Vec<(u64, sad_pet::sequence::Edit)>,
    drum_edits: Vec<(u64, sad_pet::drums::Edit)>,
    parameter_edits: Vec<(u64, String, f32)>,
    edit_revision: u64,
    sent_edit_revisions: [Option<(u32, u64)>; 3],
    config_revision: u64,
    sent_revision: Option<u64>,
    snapshot_state: Value,
    snapshot_sequence: u32,
    unacked_peaks: [Option<(u32, Stereo, Stereo)>; 3],
    source_stage: Option<source_fx::Stage>,
    retired_source: Option<Arc<sad_pet::dsp::source_fx::ProcessedSource>>,
    source_pool: Vec<Arc<sad_pet::dsp::source_fx::ProcessedSource>>,
    source_upload: Box<[Stereo; BLOCK]>,
    file_generation: u32,
}

impl Host {
    fn new(rate: u32) -> Self {
        sad_pet::parameters::specs();
        let sample = SampleBuffer::new("Loading…".into(), rate, vec![[0.0; 2]; 2]).unwrap();
        Self {
            context: sad_pet::timbres::Context::default(),
            engine: Box::new(Engine::new(rate, Arc::new(sample), Preset::default())),
            midi: sad_pet::midi::Parser::default(),
            rate,
            input: Vec::new(),
            output: Vec::new(),
            frames: Box::new([[0.0; 2]; BLOCK]),
            audio_input: Box::new([[0.0; 2]; BLOCK]),
            status: 200,
            view: sad_pet::dsp::telemetry::RealtimeView::default(),
            snapshot_buffer: vec![0; 131072],
            sequence_edits: Vec::new(),
            drum_edits: Vec::new(),
            parameter_edits: Vec::new(),
            edit_revision: 0,
            sent_edit_revisions: [None; 3],
            config_revision: 0,
            sent_revision: None,
            snapshot_state: Value::Null,
            snapshot_sequence: 0,
            unacked_peaks: [None; 3],
            source_stage: None,
            retired_source: None,
            source_pool: Vec::with_capacity(3),
            source_upload: Box::new([[0.0; 2]; BLOCK]),
            file_generation: 0,
        }
    }

    fn resize_snapshot(&mut self) {
        let size = rmp_serde::to_vec_named(&self.engine.preset).map_or(0, |v| v.len()) + 131072;
        self.snapshot_buffer.resize(size, 0);
    }
    fn state(&self) -> Value {
        let e = &self.engine;
        let sample = e.shown_sample();
        let mut state = json!({
            "version": env!("CARGO_PKG_VERSION"), "instance_id": "browser",
            "sample_rate": self.rate, "audio_device": "Web Audio", "audio_error": null,
            "audio_enabled": false, "playing": e.playing, "frozen": e.frozen,
            "bypass": e.bypass, "midi_gate": e.midi_gate, "preset": e.preset,
            "sample_name": if e.preset.rack.is_some() { &sample.name } else if e.preset.drums.is_some() { "Drum machine" } else if e.preset.synth.is_some() { sad_pet::dsp::synth::NAME } else { &sample.name }, "sample_id": sample.library_id,
            "sample_seconds": e.shown_duration(), "waveform": sample.waveform,
            "sample_frames": e.shown_frames(),
            "region": e.region,
            "fades": e.fades,
            "center": e.source_position(e.center()), "grains": e.grain_views(),
            "input": {"active": e.live_input.is_some(),
                "seconds": e.live_input.as_ref().map_or(0.0, |i| i.seconds()),
                "peak": e.live_input.as_ref().map_or([0.0; 2], |i| i.peak)},
            "active_grains": e.active_grains(), "notes": e.held_notes(),
            "step_entry": e.step_entry, "step_edits": e.step_edits,
            "recording": {"active": false, "id": null, "seconds": 0, "available": false, "error": null},
            "performance": e.performance_view(), "lfos": e.lfo_values, "envs": e.env_values,
            "sequencer": e.sequence_view(),
            "matrix_values": e.matrix_values, "knobs": e.knob_states(), "peak": e.peak,
            "gain_reduction": e.gain_reduction(),
            "resynth": e.resynth_meter(), "drums":e.drum_view(),
            "cpu_percent": 0, "underruns": 0,
            "source_fx":{"pending":e.source_effects_pending(),"recipe":sad_pet::dsp::source_fx::Recipe::from_engine(e)},
        });
        state["startup_preset_index"] = json!(sad_pet::startup::INDEX);
        state["source_generation"] = json!(self.file_generation);
        state["rack_peaks"] = json!(e.rack_peaks().unwrap_or_default());
        if self.engine.preset.drums.is_some() && self.engine.preset.rack.is_none() {
            state["playing"] = json!(self.engine.drum_view().playing);
        }
        state
    }

    fn command(&mut self, request: Value) -> Result<Value, String> {
        let path = request["path"].as_str().ok_or("Missing command")?;
        let data = &request["data"];
        let before = self.engine.preset.params;
        let midi = matches!(path, "/api/note" | "/api/midi-bytes");
        if !data.is_null()
            && !midi
            && !matches!(
                path,
                "/api/state"
                    | "/api/transport"
                    | "/api/step-entry"
                    | "/api/sequencer/edit"
                    | "/api/sequencer/view"
                    | "/api/parameters"
                    | "/api/drums/edit"
                    | "/api/drums/trigger"
            )
        {
            self.config_revision = self.config_revision.wrapping_add(1);
        }
        let result = match path {
            "/api/state" => {
                let state = self.state();
                self.engine.peak = [0.0; 2];
                self.engine.clear_rack_peaks();
                if let Some(input) = &mut self.engine.live_input {
                    input.peak = [0.0; 2];
                }
                Ok(state)
            }
            "/api/preset" if data.is_null() => Ok(json!(self.engine.preset)),
            "/api/preset" => {
                if data["format"].as_str() != Some("evil-pet-rust-v1") {
                    return Err("Use an SAD PET preset (.sad.json)".into());
                }
                let mut preset: Preset =
                    serde_json::from_value(data.clone()).map_err(|e| e.to_string())?;
                preset.sanitize();
                self.engine.set_preset(preset);
                let size = rmp_serde::to_vec_named(&self.engine.preset)
                    .map_err(|e| e.to_string())?
                    .len()
                    + 131072;
                if self.snapshot_buffer.len() < size {
                    self.snapshot_buffer.resize(size, 0);
                }
                self.sequence_edits.clear();
                self.drum_edits.clear();
                self.parameter_edits.clear();
                // Reserve the control-message high-water mark while preparing a
                // patch. First-time metadata must not grow memory after a scalar edit.
                let _ = serde_json::to_vec(&self.state());
                Ok(json!(self.engine.preset))
            }
            "/api/parameters" => {
                let changes = sad_pet::parameters::parse(data)?;
                self.engine.apply_parameters(&changes);
                self.edit_revision = self.edit_revision.wrapping_add(1);
                for (key, value) in data.as_object().unwrap() {
                    let value = value.as_f64().unwrap() as f32;
                    if let Some(entry) = self.parameter_edits.iter_mut().find(|(_, k, _)| k == key)
                    {
                        *entry = (self.edit_revision, key.clone(), value);
                    } else {
                        self.parameter_edits
                            .push((self.edit_revision, key.clone(), value));
                    }
                }
                Ok(json!({"ok":true}))
            }
            "/api/sequencer/catalog" => Ok(sad_pet::sequence::catalog()),
            "/api/drums/catalog" => Ok(sad_pet::drums::catalog()),
            "/api/filter/response" => sad_pet::dsp::filter::response_request(data),
            "/api/drums/edit" => {
                let edits = sad_pet::drums::parse_edits(data)?;
                self.engine.apply_drum_edits(&edits);
                self.edit_revision = self.edit_revision.wrapping_add(1);
                for edit in edits {
                    if let Some(entry) = self.drum_edits.iter_mut().find(|(_, e)| {
                        e.track == edit.track && e.step == edit.step && e.field == edit.field
                    }) {
                        *entry = (self.edit_revision, edit);
                    } else {
                        self.drum_edits.push((self.edit_revision, edit));
                    }
                }
                Ok(json!({"ok":true}))
            }
            "/api/drums/trigger" => {
                self.engine.trigger_drum(
                    data["track"].as_u64().unwrap_or(0).min(6) as usize,
                    data["velocity"].as_f64().unwrap_or(1.) as f32,
                );
                Ok(json!({"ok":true}))
            }
            "/api/sequencer/view" => {
                self.engine
                    .set_sequence_view(data["offset"].as_u64().unwrap_or(0) as usize);
                Ok(json!({"ok":true}))
            }
            "/api/sequencer/edit" => {
                let edits = sad_pet::sequence::parse_edits(data)?;
                self.engine.apply_sequence_edits(&edits);
                self.edit_revision = self.edit_revision.wrapping_add(1);
                for edit in edits {
                    if let Some(entry) = self.sequence_edits.iter_mut().find(|(_, e)| {
                        e.track == edit.track && e.stage == edit.stage && e.field == edit.field
                    }) {
                        *entry = (self.edit_revision, edit);
                    } else {
                        self.sequence_edits.push((self.edit_revision, edit));
                    }
                }
                Ok(json!({"ok":true}))
            }
            "/api/presets" => Ok(json!(
                (0..=sad_pet::startup::INDEX)
                    .map(Preset::factory)
                    .collect::<Vec<_>>()
            )),
            "/api/startup" => Ok(sad_pet::startup::configuration()),
            "/api/samples" => Ok(json!(sad_pet::samples::TRACKS)),
            "/api/effect-presets/factory" => Ok(json!(sad_pet::effect_presets::factory())),
            "/api/color-defaults" => Ok(json!({
                "max_slots": sad_pet::color::MAX_COLOR_SLOTS,
                "slot": sad_pet::color::ColorSlot::default(),
                "delay_tables": sad_pet::radias_delay::display_tables(),
                "mooz": sad_pet::color::mooz_catalog(),
            })),
            "/api/transport" => {
                if let Some(v) = data["midi_gate"].as_bool() {
                    self.engine.set_midi_gate(v);
                }
                if let Some(v) = data["playing"].as_bool() {
                    self.engine.set_playing(v);
                }
                if let Some(v) = data["frozen"].as_bool() {
                    self.engine.set_frozen(v);
                }
                if let Some(v) = data["bypass"].as_bool() {
                    self.engine.bypass = v;
                }
                if data["clear_hold"] == true {
                    self.engine.clear_hold();
                }
                Ok(json!({"ok": true}))
            }
            "/api/input" => {
                self.engine
                    .set_live_input(data["active"] == true, data["mode"] != "grains");
                Ok(json!({"ok": true}))
            }
            "/api/region" => {
                let region = serde_json::from_value(data.clone()).map_err(|e| e.to_string())?;
                self.engine.set_region(region)?;
                Ok(json!(self.engine.region))
            }
            "/api/fades" => {
                let fades = serde_json::from_value(data.clone()).map_err(|e| e.to_string())?;
                self.engine.set_fades(fades)?;
                Ok(json!(self.engine.fades))
            }
            "/api/note" => {
                let message: Message =
                    serde_json::from_value(data.clone()).map_err(|e| e.to_string())?;
                self.engine.midi(message);
                Ok(json!({"ok": true}))
            }
            "/api/midi-bytes" => {
                // Preserve running status and SysEx framing between packets.
                for byte in data.as_array().ok_or("Invalid MIDI packet")? {
                    let byte = byte
                        .as_u64()
                        .filter(|v| *v <= 255)
                        .ok_or("Invalid MIDI byte")? as u8;
                    if let Some(message) = self.midi.feed(byte) {
                        self.engine.midi(message);
                    }
                }
                Ok(json!({"ok": true}))
            }
            "/api/step-entry" => {
                let step = data["step"].as_u64().map(|v| v as usize);
                if step.is_some_and(|s| s >= 16) {
                    return Err("Step must be between 0 and 15".into());
                }
                self.engine.set_step_entry(step, data["reset"] == true);
                if data["clear"] == true {
                    self.engine.clear_step_notes();
                }
                Ok(json!({"ok": true}))
            }
            "/sample-info" => Ok(
                json!({"name": self.engine.sample.name, "id": self.engine.sample.library_id, "seconds": self.engine.sample.duration()}),
            ),
            _ => Err(format!("Unknown command: {path}")),
        };
        if midi
            && (before != self.engine.preset.params
                || data["kind"] == "program"
                || data.as_array().is_some_and(|bytes| {
                    bytes
                        .iter()
                        .any(|v| v.as_u64().is_some_and(|b| (0xc0..=0xcf).contains(&b)))
                }))
        {
            self.config_revision = self.config_revision.wrapping_add(1);
        }
        if matches!(
            path,
            "/api/sequencer/edit" | "/api/parameters" | "/api/drums/edit"
        ) {
            let capacity = 131072
                + self.sequence_edits.len() * 80
                + self.drum_edits.len() * 80
                + self
                    .parameter_edits
                    .iter()
                    .map(|(_, key, _)| key.len() + 32)
                    .sum::<usize>();
            if self.snapshot_buffer.len() < capacity {
                self.snapshot_buffer.resize(capacity, 0);
            }
        }
        result
    }

    fn finish(&mut self, result: Result<Vec<u8>, String>) -> u32 {
        match result {
            Ok(bytes) => {
                self.output = bytes;
                self.status = 200;
            }
            Err(error) => {
                self.output = serde_json::to_vec(&json!({"error": error})).unwrap();
                self.status = 400;
            }
        }
        self.status
    }
}

#[derive(Serialize, Deserialize)]
struct Prepared {
    name: String,
    library_id: Option<String>,
    sample_rate: u32,
    lengths: Vec<usize>,
}

fn pack(sample: &SampleBuffer) -> Vec<u8> {
    let header = serde_json::to_vec(&Prepared {
        name: sample.name.clone(),
        library_id: sample.library_id.clone(),
        sample_rate: sample.sample_rate,
        lengths: sample.prepared_levels().iter().map(Vec::len).collect(),
    })
    .unwrap();
    let mut bytes = Vec::with_capacity(
        12 + header.len()
            + sample
                .prepared_levels()
                .iter()
                .map(|l| l.len() * 8)
                .sum::<usize>(),
    );
    bytes.extend_from_slice(b"SADPCM01");
    bytes.extend_from_slice(&(header.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&header);
    for value in sample.prepared_levels().iter().flatten().flatten() {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
    bytes
}

fn unpack(bytes: &[u8]) -> Result<SampleBuffer, String> {
    if bytes.len() < 12 || &bytes[..8] != b"SADPCM01" {
        return Err("Invalid prepared sample".into());
    }
    let n = u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize;
    let header: Prepared =
        serde_json::from_slice(bytes.get(12..12 + n).ok_or("Incomplete sample header")?)
            .map_err(|e| e.to_string())?;
    if header.lengths.is_empty() || header.lengths.len() > 7 {
        return Err("Invalid sample levels".into());
    }
    let mut tail = &bytes[12 + n..];
    let mut levels = Vec::new();
    for length in header.lengths {
        let count = length.checked_mul(8).ok_or("Sample too large")?;
        let block = tail.get(..count).ok_or("Incomplete sample levels")?;
        levels.push(
            block
                .as_chunks::<8>()
                .0
                .iter()
                .map(|b| {
                    [
                        f32::from_le_bytes(b[..4].try_into().unwrap()),
                        f32::from_le_bytes(b[4..].try_into().unwrap()),
                    ]
                })
                .collect(),
        );
        tail = &tail[count..];
    }
    if !tail.is_empty() {
        return Err("Unexpected sample data".into());
    }
    SampleBuffer::from_prepared(header.name, header.library_id, header.sample_rate, levels)
}

thread_local! { static HOST: RefCell<Option<timbres::Registry>> = const { RefCell::new(None) }; }
fn registry<T>(f: impl FnOnce(&mut timbres::Registry) -> T) -> T {
    HOST.with(|h| f(h.borrow_mut().as_mut().expect("Call sad_init first")))
}
fn host<T>(f: impl FnOnce(&mut Host) -> T) -> T {
    registry(|r| {
        let index = r.index();
        f(&mut r.parts[index].host)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn sad_init(rate: u32) {
    HOST.with(|h| *h.borrow_mut() = Some(timbres::Registry::new(rate.clamp(8_000, 384_000))));
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_input(length: usize) -> *mut u8 {
    host(|h| {
        if length > MAX_INPUT {
            return std::ptr::null_mut();
        }
        h.input.resize(length, 0);
        h.input.as_mut_ptr()
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_output() -> *const u8 {
    host(|h| h.output.as_ptr())
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_output_len() -> usize {
    host(|h| h.output.len())
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_command() -> u32 {
    registry(|r| {
        let index = r.index();
        let result = serde_json::from_slice::<Value>(&r.parts[index].host.input)
            .map_err(|e| e.to_string())
            .and_then(|v| r.command(v))
            .and_then(|v| serde_json::to_vec(&v).map_err(|e| e.to_string()));
        let index = r.index();
        r.parts[index].host.finish(result)
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_process(count: usize) -> *const Stereo {
    registry(|r| r.process(count))
}
/// A MIDI clock message (Clock, Start, Continue, Stop or Song Position)
/// sounding at `frame` on `sad_clock_frame`'s count.
#[unsafe(no_mangle)]
pub extern "C" fn sad_clock_event(frame: f64, status: u32, data1: u32, data2: u32) {
    let message = match status {
        0xf8 => Message::Clock,
        0xfa => Message::Start,
        0xfb => Message::Continue,
        0xfc => Message::Stop,
        0xf2 => Message::SongPosition {
            sixteenths: ((data2 & 0x7f) << 7 | (data1 & 0x7f)) as u16,
        },
        _ => return,
    };
    registry(|r| r.clock_event(frame, message));
}
/// Frames rendered so far: the first frame of the next block.
#[unsafe(no_mangle)]
pub extern "C" fn sad_clock_frame() -> f64 {
    registry(|r| r.midi_clock.frame() as f64)
}
/// Changes when the scene starts, stops or moves.
#[unsafe(no_mangle)]
pub extern "C" fn sad_clock_epoch() -> u32 {
    registry(|r| r.midi_clock.epoch())
}
/// The clock's state as `clock::VIEW` numbers.
#[unsafe(no_mangle)]
pub extern "C" fn sad_clock_view() -> *const f64 {
    registry(|r| r.midi_clock.view())
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_audio_input() -> *mut Stereo {
    registry(|r| r.parts[0].host.audio_input.as_mut_ptr())
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_input_wav() -> u32 {
    host(|h| {
        let result = h
            .engine
            .live_input
            .as_ref()
            .ok_or_else(|| "Microphone is off".to_string())
            .and_then(|input| {
                let frames = input.capture();
                if frames.is_empty() {
                    return Err("The input buffer is empty".into());
                }
                wav::bytes(h.rate, &frames)
            });
        h.finish(result)
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_factory(index: usize) -> u32 {
    host(|h| {
        let result = sad_pet::samples::TRACKS
            .get(index)
            .ok_or_else(|| "Unknown track".to_string())
            .and_then(|track| {
                if track.asset.is_some() {
                    return Err("Load this recording from its catalog asset".to_string());
                }
                // IndexedDB retains the exported WAV. Keeping every generated
                // sample and its mipmaps in the worker would duplicate the whole
                // expanded bank in mobile RAM throughout the browser session.
                let sample = sad_pet::samples::generate(index, 48_000);
                wav::bytes(sample.sample_rate, sample.frames())
            });
        h.finish(result)
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_prepare() -> u32 {
    host(|h| {
        let result = wav::decode(Cursor::new(&h.input), "Sample.wav".into()).map(|sample| {
            let bytes = pack(&sample);
            h.engine.set_sample(Arc::new(sample));
            bytes
        });
        h.finish(result)
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_load() -> u32 {
    host(|h| {
        let result = unpack(&h.input).map(|sample| {
            h.engine.set_sample(Arc::new(sample));
            h.reset_source_buffers();
            h.config_revision = h.config_revision.wrapping_add(1);
            serde_json::to_vec(&h.command(json!({"path": "/sample-info"})).unwrap()).unwrap()
        });
        h.finish(result)
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_sample_name() -> u32 {
    host(|h| {
        let result = serde_json::from_slice::<Value>(&h.input)
            .map_err(|e| e.to_string())
            .map(|v| {
                let sample =
                    Arc::get_mut(&mut h.engine.sample).expect("worker sample has one owner");
                sample.name = v["name"]
                    .as_str()
                    .unwrap_or("Sample.wav")
                    .chars()
                    .filter(|c| !c.is_control())
                    .take(150)
                    .collect();
                sample.library_id = v["id"]
                    .as_str()
                    .filter(|id| {
                        id.len() <= 160 && id.is_ascii() && !id.chars().any(char::is_control)
                    })
                    .map(str::to_owned);
                serde_json::to_vec(&json!({"ok": true})).unwrap()
            });
        h.finish(result)
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_pack() -> u32 {
    host(|h| h.finish(Ok(pack(&h.engine.sample))))
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_sample_wav() -> u32 {
    host(|h| {
        h.finish(wav::bytes(
            h.engine.sample.sample_rate,
            h.engine.sample.frames(),
        ))
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_trim() -> u32 {
    host(|h| {
        let result = serde_json::from_slice::<Value>(&h.input)
            .map_err(|e| e.to_string())
            .and_then(|value| {
                if value
                    .get("sample_id")
                    .is_some_and(|id| id.as_str() != h.engine.sample.library_id.as_deref())
                {
                    return Err("The audio source changed; select the region again".into());
                }
                let fades = serde_json::from_value(
                    value.get("fades").cloned().unwrap_or_else(|| json!({})),
                )
                .map_err(|e| e.to_string())?;
                let region = serde_json::from_value(value).map_err(|e| e.to_string())?;
                h.engine.sample.trim_wav_with_fades(region, fades)
            });
        h.finish(result)
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_render() -> u32 {
    host(|h| {
        let result = (|| {
            let value: Value = serde_json::from_slice(&h.input).map_err(|e| e.to_string())?;
            let seconds = value["seconds"].as_f64().ok_or("Missing render duration")?;
            if !seconds.is_finite() || !(0.1..=120.0).contains(&seconds) {
                return Err("Render duration must be 0.1–120 seconds".into());
            }
            let mut preset: Preset =
                serde_json::from_value(value["preset"].clone()).map_err(|e| e.to_string())?;
            preset.sanitize();
            let mut engine = Engine::new(h.rate, h.engine.sample.clone(), preset);
            if let Some(region) = value.get("region") {
                engine.set_region(
                    serde_json::from_value(region.clone()).map_err(|e| e.to_string())?,
                )?;
            }
            engine.bypass = value["bypass"] == true;
            if let Some(fades) = value.get("fades") {
                engine
                    .set_fades(serde_json::from_value(fades.clone()).map_err(|e| e.to_string())?)?;
            }
            engine.set_frozen(value["frozen"] == true);
            engine.prepare_source_effects()?;
            engine.set_playing(true);
            let mut frames = vec![[0.0; 2]; (seconds * h.rate as f64).round() as usize];
            engine.process(&mut frames);
            wav::bytes(h.rate, &frames)
        })();
        h.finish(result)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn prepared_transfer_preserves_all_read_rates() {
        let sample = SampleBuffer::demo(8_000);
        let copy = unpack(&pack(&sample)).unwrap();
        assert_eq!(copy.waveform, sample.waveform);
        for rate in [0.5, 1.0, 2.5, 8.0, -4.0] {
            for position in [-12.3, 0.0, 100.75, 5000.1] {
                assert_eq!(
                    copy.read(position, rate, true),
                    sample.read(position, rate, true)
                );
            }
        }
    }
    #[test]
    fn rejects_incomplete_or_invalid_prepared_samples() {
        let sample = SampleBuffer::demo(8_000);
        let mut bytes = pack(&sample);
        assert!(unpack(&bytes[..bytes.len() - 1]).is_err());
        bytes.push(0);
        assert!(unpack(&bytes).is_err());
        assert!(unpack(b"not a sample").is_err());
        assert!(
            SampleBuffer::from_prepared("bad".into(), None, 48_000, vec![vec![[f32::NAN, 0.0]]])
                .is_err()
        );
    }
}
