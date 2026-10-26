//! Native plugin boundary. Only Audio is touched by the host's audio callback.
//! Controller prepares files/presets and reclaims retired allocations off-thread.
mod audio_library;
mod effect_presets;
mod instruments;
mod parameters;
mod preset_library;
mod preview;
mod timbres;
mod vocal;
mod workers;
use sad_pet::dsp::source_fx_worker as source_fx;

use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use rtrb::{Consumer, Producer, RingBuffer};
use sad_pet::{
    Stereo,
    dsp::{Engine, Fades, Region, SampleBuffer, sample::LiveInput, telemetry::RealtimeView},
    midi::{Controls, Message, Parser},
    performance::Mode,
    preset::Preset,
    wav,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{io::Cursor, sync::Arc};

const QUEUE: usize = 64;
const MAX_BYTES: usize = 128 * 1024 * 1024;

#[derive(Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Source {
    #[default]
    File,
    Effects,
    Grains,
}

struct PreparedSource {
    sample: Arc<SampleBuffer>,
    live: Option<LiveInput>,
    region: Region,
    fades: Fades,
    processed: Option<Arc<sad_pet::dsp::source_fx::ProcessedSource>>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct Transport {
    playing: Option<bool>,
    midi_gate: Option<bool>,
    frozen: Option<bool>,
    bypass: Option<bool>,
    sync: Option<bool>,
    clear_hold: bool,
}

enum Command {
    Preview(Option<Box<preview::Preview>>),
    PreviewStop,
    Preset(Box<Preset>),
    Parameters(Box<[parameters::Change]>),
    Sequence(Box<[sad_pet::sequence::Edit]>),
    SequenceView(usize),
    Drums(Box<[sad_pet::drums::Edit]>),
    DrumTrigger(usize, f32),
    AddPart(Option<Box<timbres::PartAudio>>),
    RemovePart(u32, Option<Box<timbres::PartAudio>>),
    PartMix(u32, sad_pet::timbres::Mix),
    Source(Box<PreparedSource>),
    Restore(Box<Restore>),
    Transport(Transport),
    Region(Region),
    Fades(Fades),
    Midi(Message),
    /// Play or stop the song clock that Vocal timbres follow without a host.
    VocalTransport {
        playing: Option<bool>,
        start: Option<u64>,
    },
    Step {
        index: Option<usize>,
        reset: bool,
        clear: bool,
    },
    /// Render workers, started on a control thread once a scene has two timbres.
    Workers(Option<Box<workers::Pool>>),
}

struct Restore {
    mix: sad_pet::timbres::Mix,
    parts: Option<Box<timbres::PartAudio>>,
    preset: Preset,
    source: PreparedSource,
    sync: bool,
    frozen: bool,
    bypass: bool,
    midi_gate: bool,
    playing: bool,
}

struct Update {
    revision: u64,
    command: Command,
}
struct Feedback {
    revision: u64,
    view: RealtimeView,
    bpm: f32,
    host_playing: bool,
    /// The song clock or the host's song position, for Vocal timbres.
    song_playing: bool,
    song_position: i64,
}

/// The editor polls /api/state about twelve times a second. The preset and
/// waveforms rarely change and most knobs are unmodulated, so an answer to
/// `?since=<state_token>` of the previous answer leaves out the unchanged
/// parts and lists only changed knobs in `knob_changes` (null = removed).
#[derive(Default)]
struct SentState {
    token: u64,
    preset: Value,
    waveform: Value,
    file_source: Value,
    knobs: serde_json::Map<String, Value>,
}
impl SentState {
    fn answer(&mut self, state: &mut Value, query: &str) {
        let since = query
            .split('&')
            .find_map(|pair| pair.strip_prefix("since="))
            .and_then(|token| token.parse::<u64>().ok());
        let delta = self.token != 0 && since == Some(self.token);
        let Some(state) = state.as_object_mut() else {
            return;
        };
        for (key, sent) in [
            ("preset", &mut self.preset),
            ("waveform", &mut self.waveform),
            ("file_source", &mut self.file_source),
        ] {
            let Some(value) = state.remove(key) else {
                continue;
            };
            if !delta || value != *sent {
                state.insert(key.into(), value.clone());
                *sent = value;
            }
        }
        if let Some(Value::Object(knobs)) = state.remove("knobs") {
            if delta {
                let mut changes = serde_json::Map::new();
                for (key, value) in &knobs {
                    if self.knobs.get(key) != Some(value) {
                        changes.insert(key.clone(), value.clone());
                    }
                }
                for key in self.knobs.keys().filter(|key| !knobs.contains_key(*key)) {
                    changes.insert(key.clone(), Value::Null);
                }
                state.insert("knob_changes".into(), Value::Object(changes));
            } else {
                state.insert("knobs".into(), Value::Object(knobs.clone()));
            }
            self.knobs = knobs;
        }
        self.token = self.token.wrapping_add(1).max(1);
        state.insert("state_token".into(), json!(self.token));
        state.insert("delta".into(), json!(delta));
    }
}

pub struct Controller {
    preview_generation: u64,
    meta: sad_pet::timbres::Meta,
    parts: Vec<Controller>,
    active: u32,
    scene_revision: u64,
    next_timbre_id: u32,
    commands: Producer<Update>,
    retired: Consumer<Update>,
    feedback: Consumer<Feedback>,
    preset: Preset,
    sample: Arc<SampleBuffer>,
    source: Source,
    region: Region,
    fades: Fades,
    sync: bool,
    frozen: bool,
    bypass: bool,
    midi_gate: bool,
    playing: bool,
    effect: bool,
    rate: u32,
    revision: u64,
    view: RealtimeView,
    host_bpm: f32,
    host_playing: bool,
    library: preset_library::Library,
    selection: preset_library::Selection,
    /// The standalone app: audio input is always available, the engine keeps
    /// its own transport and sample files never cross the editor boundary.
    standalone: bool,
    files: audio_library::Files,
    /// The scene's last /api/state answer, for delta answers.
    sent: SentState,
    /// Present while this part is a Vocal timbre.
    vocal: Option<Arc<sad_pet::vocal::Mailbox>>,
    /// The render thread, shared by every part and started on first use.
    voices: Arc<std::sync::OnceLock<vocal::Service>>,
    song_playing: bool,
    song_position: i64,
    /// The prepared audio has render workers, or a queued update brings them.
    workers_started: bool,
    /// Controllers moved on the host's MIDI input, for the editor's MIDI learn.
    controls: Arc<Controls>,
}

pub struct Audio {
    preview: Option<Box<preview::Preview>>,
    meta: sad_pet::timbres::Meta,
    parts: Option<Box<timbres::PartAudio>>,
    audible: bool,
    smooth_part_gain: f32,
    /// Smoothed Bus sends of this part's output.
    sends: sad_pet::timbres::SendGains,
    /// Set on the root part while any Bus exists, so sends are summed.
    buses: bool,
    /// On the root part: the order buses render in, so that a bus renders
    /// after every bus that feeds it.
    bus_order: sad_pet::timbres::BusOrder,
    engine: Box<Engine>,
    commands: Consumer<Update>,
    retired: Producer<Update>,
    feedback: Producer<Feedback>,
    parser: Parser,
    controls: Arc<Controls>,
    parameters: Vec<f32>,
    revision: u64,
    sync: bool,
    host_playing: bool,
    host_bpm: f32,
    expected_beat: Option<f64>,
    frames_since_feedback: usize,
    view: RealtimeView,
    source_renderer: source_fx::Renderer,
    vocal: Option<Box<VocalAudio>>,
    /// The scene's song clock, used when the host has no transport.
    clock: sad_pet::vocal::Clock,
    /// This block's song state: playing, position in frames.
    song: (bool, i64),
    /// The DAW's tempo map as heard so far: Vocal timbres render on it and
    /// read the song position with it.
    tempo: sad_pet::vocal::TempoMap,
    /// The host renders faster than real time (a bounce): Vocal timbres wait
    /// for their phrases.
    offline: bool,
    /// This part's render buffers for one chunk of the block.
    chunk: Box<timbres::Chunk>,
    /// The scene's render workers; only the scene's own part has them.
    pool: Option<Box<workers::Pool>>,
}

/// A Vocal part's audio side: rendered clips, read a chunk at a time.
struct VocalAudio {
    track: sad_pet::vocal::Track,
    buffer: [f32; 256],
    index: usize,
    position: i64,
    playing: bool,
    /// The version of the host tempo map the render thread was given.
    published: u64,
    /// An offline bounce stopped waiting for phrases (the render thread
    /// took too long); it plays what there is until the transport stops.
    gave_up: bool,
}
impl VocalAudio {
    fn new(mailbox: Arc<sad_pet::vocal::Mailbox>) -> Box<Self> {
        Box::new(Self {
            track: sad_pet::vocal::Track::new(mailbox),
            buffer: [0.0; 256],
            index: 256,
            position: 0,
            playing: false,
            published: 0,
            gave_up: false,
        })
    }
    /// Starts a block at a song position. With a host transport, `tempo` is
    /// the DAW's tempo map as heard so far; offline (a bounce), the block
    /// waits until every phrase is rendered.
    fn begin(
        &mut self,
        position: i64,
        playing: bool,
        tempo: Option<&sad_pet::vocal::TempoMap>,
        offline: bool,
    ) {
        self.position = position;
        self.playing = playing;
        self.index = self.buffer.len();
        self.track.update();
        if let (Some(map), Some(mailbox)) = (tempo, &self.track.mailbox)
            && self.published != map.version()
            && mailbox.publish_tempo(map)
        {
            self.published = map.version();
        }
        if !playing {
            self.gave_up = false;
        } else if offline && !self.gave_up {
            let started = std::time::Instant::now();
            while !self.track.complete() {
                if started.elapsed() > std::time::Duration::from_secs(30) {
                    self.gave_up = true;
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(2));
            }
        }
    }
    fn next(&mut self) -> f32 {
        if self.index == self.buffer.len() {
            if self.playing {
                self.track.fill(self.position, &mut self.buffer);
            } else {
                self.buffer.fill(0.0);
            }
            self.position += self.buffer.len() as i64;
            self.index = 0;
        }
        let sample = self.buffer[self.index];
        self.index += 1;
        sample
    }
}

#[derive(Serialize, Deserialize)]
struct SavedState {
    #[serde(default)]
    meta: sad_pet::timbres::Meta,
    #[serde(default)]
    timbres: Vec<SavedState>,
    #[serde(default)]
    active_timbre: u32,
    version: u32,
    preset: Preset,
    sample_name: String,
    sample_id: Option<String>,
    sample_wav: String,
    source: Source,
    region: Region,
    fades: Fades,
    sync: bool,
    frozen: bool,
    bypass: bool,
    midi_gate: bool,
    playing: bool,
    #[serde(default)]
    selection: preset_library::Selection,
}

impl Controller {
    /// A new DAW instance starts with the factory scene; newly added timbres
    /// still use the empty constructor below. Project restoration replaces it.
    fn new_plugin(effect: bool) -> Self {
        Self::new_scene(effect, false)
    }

    fn new_scene(effect: bool, standalone: bool) -> Self {
        let mut control = Self::new(effect);
        control.standalone = standalone;
        control.sync = !standalone;
        control.preset = sad_pet::startup::preset();
        control.preset.sanitize();
        control.sample = sad_pet::startup::sample();
        control.region = sad_pet::startup::REGION;
        control.fades = sad_pet::startup::FADES;
        control.selection.id = format!("factory:{}", sad_pet::startup::INDEX);
        control.ensure_master();
        control
    }

    fn new(effect: bool) -> Self {
        let (commands, _) = RingBuffer::new(QUEUE);
        let (_, retired) = RingBuffer::new(QUEUE);
        let (_, feedback) = RingBuffer::new(3);
        let mut preset = Preset::default();
        preset.sanitize();
        Self {
            meta: sad_pet::timbres::Meta::default(),
            parts: Vec::new(),
            active: 0,
            scene_revision: 0,
            next_timbre_id: 1,
            commands,
            retired,
            feedback,
            preset,
            sample: sad_pet::samples::get("glass-sequence").expect("bundled sample"),
            source: if effect { Source::Grains } else { Source::File },
            region: Region::default(),
            fades: Fades::default(),
            sync: true,
            frozen: false,
            bypass: false,
            midi_gate: !effect,
            playing: effect,
            effect,
            rate: 44_100,
            revision: 0,
            view: RealtimeView::default(),
            host_bpm: 120.0,
            host_playing: false,
            library: preset_library::Library::standard(),
            selection: preset_library::Selection {
                id: "factory:0".into(),
                dirty: false,
            },
            standalone: false,
            files: audio_library::Files::standard(),
            sent: SentState::default(),
            vocal: None,
            voices: Arc::new(std::sync::OnceLock::new()),
            song_playing: false,
            song_position: 0,
            preview_generation: 0,
            workers_started: false,
            controls: Arc::default(),
        }
    }

    /// A new timbre shares the scene's host mode.
    fn new_part(&self) -> Self {
        let mut part = Self::new(self.effect);
        part.standalone = self.standalone;
        part.sync = self.sync;
        part.voices = self.voices.clone();
        part
    }

    fn input_available(&self) -> bool {
        self.effect || self.standalone
    }

    fn input_name(&self) -> &'static str {
        if self.standalone {
            "Audio input"
        } else {
            "DAW input"
        }
    }

    fn drain(&mut self) {
        while self.retired.pop().is_ok() {}
        while let Ok(feedback) = self.feedback.pop() {
            self.view = feedback.view;
            self.host_bpm = feedback.bpm;
            self.host_playing = feedback.host_playing;
            self.song_playing = feedback.song_playing;
            self.song_position = feedback.song_position;
            if feedback.revision == self.revision {
                self.playing = self.view.playing;
                self.midi_gate = self.view.midi_gate;
                self.frozen = self.view.frozen;
                self.bypass = self.view.bypass;
                self.preset.performance.chord.clear();
                self.preset
                    .performance
                    .chord
                    .extend_from_slice(&self.view.chord[..self.view.chord_len]);
                for (i, step) in self.view.step_edits.iter().enumerate() {
                    if let Some(step) = step {
                        if self.preset.performance.steps[i] != *step {
                            self.selection.dirty = true;
                        }
                        self.preset.performance.steps[i] = *step;
                    }
                }
            }
        }
    }

    fn enqueue(&mut self, command: Command) -> Result<(), String> {
        self.drain();
        let revision = self.revision.wrapping_add(1);
        self.commands
            .push(Update { revision, command })
            .map_err(|_| "Plugin update queue is full. Try again.".to_string())?;
        self.revision = revision;
        Ok(())
    }

    fn prepared_source(
        &self,
        sample: Arc<SampleBuffer>,
        source: Source,
        region: Region,
        fades: Fades,
    ) -> PreparedSource {
        self.prepared_source_for(sample, source, region, fades, &self.preset)
    }
    fn prepared_source_for(
        &self,
        sample: Arc<SampleBuffer>,
        source: Source,
        region: Region,
        fades: Fades,
        preset: &Preset,
    ) -> PreparedSource {
        PreparedSource {
            processed: if source == Source::File {
                sad_pet::dsp::source_fx::prepare(
                    sample.clone(),
                    region,
                    preset.source_color_chain.first().copied(),
                    preset.params.bpm,
                    preset.params.sample_root_note,
                    || false,
                )
                .ok()
                .flatten()
            } else {
                None
            },
            sample,
            live: (preset.bus.is_none() && (source != Source::File || preset.history().is_some()))
                .then(|| {
                    let mut input = preset.history().map_or_else(
                        || LiveInput::new(self.rate, source == Source::Effects),
                        |h| LiveInput::for_drums(self.rate, !h.granular, h.buffer_seconds),
                    );
                    input.sample.name = if preset.drums.is_some() {
                        "Drum machine"
                    } else if preset.synth.is_some() {
                        sad_pet::dsp::synth::NAME
                    } else {
                        self.input_name()
                    }
                    .into();
                    input
                }),
            region,
            fades,
        }
    }

    fn prepare(&mut self, rate: u32) -> Audio {
        self.drain();
        // Workers queued for an earlier audio leave with its queue.
        self.workers_started = false;
        self.rate = rate.clamp(8_000, 384_000);
        let (commands, receiver) = RingBuffer::new(QUEUE);
        let (retire, retired) = RingBuffer::new(QUEUE);
        let (publish, feedback) = RingBuffer::new(3);
        self.commands = commands;
        self.retired = retired;
        self.feedback = feedback;
        let mut engine = Box::new(Engine::new(
            self.rate,
            self.sample.clone(),
            self.preset.clone(),
        ));
        engine.set_midi_gate(self.midi_gate);
        // A Bus or Master reads other timbres: it has no file source, and its
        // Granular history comes prepared with the preset.
        if self.preset.bus.is_none() {
            let mut source =
                self.prepared_source(self.sample.clone(), self.source, self.region, self.fades);
            engine.exchange_source(&mut source.sample, &mut source.live);
            if self.source == Source::File {
                let _ = engine.set_region(self.region);
                let _ = engine.set_fades(self.fades);
            }
            engine.exchange_processed_source(&mut source.processed);
        }
        engine.set_playing(
            self.playing
                && (self.effect || self.preset.performance.mode != Mode::Keys || !self.midi_gate),
        );
        engine.set_frozen(self.frozen && self.source == Source::File);
        engine.bypass = self.bypass;
        let source_renderer = source_fx::Renderer::new(&engine);
        let vocal = self.sync_vocal().map(VocalAudio::new);
        Audio {
            preview: None,
            meta: self.meta.clone(),
            parts: self.prepare_parts(self.rate),
            audible: !self.meta.mix.muted,
            smooth_part_gain: if self.meta.mix.muted {
                0.
            } else {
                self.meta.mix.gain
            },
            sends: Default::default(),
            buses: false,
            bus_order: Default::default(),
            engine,
            commands: receiver,
            retired: retire,
            feedback: publish,
            parser: Parser::default(),
            controls: self.controls.clone(),
            parameters: vec![f32::NAN; parameters::specs().len()],
            revision: self.revision,
            sync: self.sync,
            host_playing: false,
            host_bpm: 120.0,
            expected_beat: None,
            frames_since_feedback: 0,
            view: RealtimeView::default(),
            source_renderer,
            vocal,
            clock: Default::default(),
            song: (false, 0),
            tempo: sad_pet::vocal::TempoMap::new(),
            offline: false,
            chunk: timbres::Chunk::new(),
            pool: self.start_workers(),
        }
    }

    /// Timbres that render their own source: the scene's part and every
    /// part that is not a Bus or Master.
    fn rendered_timbres(&self) -> usize {
        1 + self.parts.iter().filter(|p| !p.meta.kind.routing()).count()
    }
    /// Workers for the scene once it renders two timbres or more; a part
    /// of the scene, or a scene of one timbre, renders on the audio thread.
    fn start_workers(&mut self) -> Option<Box<workers::Pool>> {
        if self.meta.id != 0 || self.workers_started || self.rendered_timbres() < 2 {
            return None;
        }
        let pool = workers::Pool::new().map(Box::new);
        self.workers_started = pool.is_some();
        pool
    }
    /// Hands workers to the running audio once the scene has grown to two
    /// timbres. Without a queue slot they follow with a later change.
    fn offer_workers(&mut self) {
        if let Some(pool) = self.start_workers()
            && self.enqueue(Command::Workers(Some(pool))).is_err()
        {
            self.workers_started = false;
        }
    }

    pub(crate) fn voice_service(&self) -> &vocal::Service {
        self.voices.get_or_init(vocal::Service::new)
    }
    /// Gives a Vocal part its mailbox and sends its score to the render
    /// thread; any other part drops both.
    fn sync_vocal(&mut self) -> Option<Arc<sad_pet::vocal::Mailbox>> {
        let vocal = self
            .preset
            .vocal
            .as_deref()
            .filter(|_| self.meta.kind == sad_pet::timbres::Kind::Vocal)
            .cloned();
        match vocal {
            Some(settings) => {
                let mailbox = self
                    .vocal
                    .get_or_insert_with(sad_pet::vocal::Mailbox::new)
                    .clone();
                self.voice_service()
                    .submit(self.meta.id, &settings, self.rate, &mailbox);
                Some(mailbox)
            }
            None => {
                if self.vocal.take().is_some() {
                    self.voice_service().forget(self.meta.id);
                }
                None
            }
        }
    }

    fn values(&self) -> Vec<f32> {
        parameters::specs()
            .iter()
            .map(|p| p.get(&self.preset))
            .collect()
    }
    fn set_values(&mut self, values: &[f32]) {
        for (spec, &value) in parameters::specs().iter().zip(values) {
            let previous = spec.get(&self.preset);
            // Hosts normalize to float32 and back; retain the saved value when
            // the difference is only that round trip's precision.
            if (value - previous).abs() <= 2.0 * f32::EPSILON * (spec.max - spec.min) {
                continue;
            }
            spec.set(&mut self.preset, value);
            if previous != spec.get(&self.preset) {
                self.selection.dirty = true;
            }
        }
        parameters::mirror(&mut self.preset);
    }

    fn state(&mut self) -> Value {
        self.drain();
        let mut displayed_preset = self.preset.clone();
        if self.sync {
            displayed_preset.params.bpm = self.host_bpm;
        }
        let view = &self.view;
        let live = self.source != Source::File;
        let drums = self.preset.drums.is_some();
        let synth = self.preset.synth.is_some();
        let buffered = live || drums || synth;
        let buffer_frames = self
            .preset
            .history()
            .map_or(self.rate as usize * 8, |h| h.buffer_frames(self.rate));
        let knobs = sad_pet::dsp::telemetry::knob_states_from_view(&self.preset, view);
        let mut reduction: std::collections::BTreeMap<_, _> = view
            .gain_reduction
            .iter()
            .filter(|(id, _)| *id != 0)
            .map(|(id, db)| (id.to_string(), *db))
            .collect();
        if view.source_gain_reduction.0 != 0 {
            reduction.insert(
                view.source_gain_reduction.0.to_string(),
                view.source_gain_reduction.1,
            );
        }
        let mut state = json!({
            "version": env!("CARGO_PKG_VERSION"), "instance_id": "plugin",
            "plugin": {"effect":self.effect,"standalone":self.standalone,"sync":self.sync,"host_bpm":self.host_bpm,"host_playing":self.host_playing,"bar_beats":view.bar_beats},
            "sample_rate":self.rate,"audio_device":if self.standalone { "Audio device" } else { "DAW" },"audio_enabled":true,"audio_error":null,
            "playing":self.playing,"midi_gate":self.midi_gate,"frozen":self.frozen,"bypass":self.bypass,
            "preset":displayed_preset,"sample_name":if drums { "Drum machine" } else if synth { sad_pet::dsp::synth::NAME } else if live { self.input_name() } else { &self.sample.name },
            "preset_selection":self.selection,
            "sample_id":if buffered { None } else { self.sample.library_id.as_ref() },
            "sample_seconds":if buffered { buffer_frames as f64 / self.rate as f64 } else { self.sample.duration() },
            "sample_frames":if buffered { buffer_frames } else { self.sample.len() },
            "file_source":{"sample_name":self.sample.name,"sample_id":self.sample.library_id,"sample_seconds":self.sample.duration(),"sample_frames":self.sample.len(),"waveform":self.sample.waveform},
            "waveform":if buffered { &view.waveform[..view.waveform_len] } else { &self.sample.waveform },
            "region":self.region,"fades":self.fades,"center":view.center,
            "grains":&view.grains[..view.grain_count],"active_grains":view.grain_count,
            "notes":&view.notes[..view.note_count],"step_entry":view.step_entry,"step_edits":view.step_edits,
            "input":{"active":live,"available":self.input_available(),"name":self.input_name(),"mode":if self.source==Source::Effects {"effects"} else {"grains"},"gain":1.0,"monitor":true,"pending":false,"devices":[],"seconds":view.input_seconds,"peak":view.input_peak},
            "recording":{"active":false,"available":false,"seconds":0,"error":null},
            "performance":{"mode":self.preset.performance.mode,"step":view.step,"arp_index":view.arp_index,"chord":&view.chord[..view.chord_len],"gate":view.gate,"beat":view.beat},
            "lfos":view.lfos,"envs":view.envs,"matrix_values":view.matrix,"knobs":knobs,
            "peak":view.peak,"gain_reduction":reduction,"cpu_percent":0.0,"underruns":0,
            "source_fx":{"pending":view.source_fx_pending,"error":if view.source_fx_error {Some("Could not prepare Source FX")}else{None}},
        });
        state["startup_preset_index"] = json!(sad_pet::startup::INDEX);
        state["resynth"] =
            json!({"bands":view.resynth_bands,"frequencies":view.resynth_frequencies});
        state["sequencer"] = json!(view.sequencer);
        state["drums"] = json!(view.drums);
        if self.preset.drums.is_some() {
            state["playing"] = json!(view.drums.playing);
        }
        state
    }

    fn snapshot(&mut self) -> Result<SavedState, String> {
        self.drain();
        let sample_wav = BASE64.encode(wav::bytes(self.sample.sample_rate, self.sample.frames())?);
        Ok(SavedState {
            meta: self.meta.clone(),
            timbres: self
                .parts
                .iter_mut()
                .map(Controller::snapshot)
                .collect::<Result<Vec<_>, _>>()?,
            active_timbre: self.active,
            version: 1,
            preset: self.preset.clone(),
            sample_name: self.sample.name.clone(),
            sample_id: self.sample.library_id.clone(),
            sample_wav,
            source: self.source,
            region: self.region,
            fades: self.fades,
            sync: self.sync,
            frozen: self.frozen && self.source == Source::File,
            bypass: self.bypass,
            midi_gate: self.midi_gate,
            playing: self.playing && !self.midi_gate,
            selection: self.selection.clone(),
        })
    }

    fn save(&mut self) -> Result<Vec<u8>, String> {
        serde_json::to_vec(&self.snapshot()?).map_err(|e| e.to_string())
    }

    fn restore(&mut self, bytes: &[u8]) -> Result<(), String> {
        let saved: SavedState =
            serde_json::from_slice(bytes).map_err(|e| format!("Plugin state: {e}"))?;
        self.restore_saved(saved)
    }

    fn restore_saved(&mut self, mut saved: SavedState) -> Result<(), String> {
        if saved.version != 1 {
            return Err("Unsupported SAD PET plugin state version".into());
        }
        if saved.source != Source::File && !self.input_available() {
            return Err("Use SAD PET FX for audio input".into());
        }
        // The standalone app has no host transport to follow.
        saved.sync &= !self.standalone;
        if saved.meta.id != self.meta.id {
            return Err("Invalid timbre identity".into());
        }
        saved.meta.sanitize();
        saved.meta.kind = sad_pet::timbres::Kind::of(&saved.preset);
        saved.preset.sanitize();
        let decoded = BASE64
            .decode(&saved.sample_wav)
            .map_err(|e| format!("Saved sample: {e}"))?;
        if decoded.len() > MAX_BYTES {
            return Err("Saved sample exceeds 128 MB".into());
        }
        let mut sample = wav::decode(Cursor::new(decoded), saved.sample_name)?;
        sample.library_id = saved.sample_id;
        saved.region.bounds(sample.len())?;
        saved.fades.validate()?;
        let sample = Arc::new(sample);
        let source = self.prepared_source_for(
            sample.clone(),
            saved.source,
            saved.region,
            saved.fades,
            &saved.preset,
        );
        let (mut parts, mut part_audio) = self.restore_parts(std::mem::take(&mut saved.timbres))?;
        // Restoration recurses into each part; only the scene owns a Master.
        if self.meta.id == 0 {
            timbres::ensure_master_in(&mut parts, &mut part_audio, self, self.rate);
        }
        self.enqueue(Command::Restore(Box::new(Restore {
            parts: part_audio,
            mix: saved.meta.mix,
            preset: saved.preset.prepared_clone_for(self.rate),
            source,
            sync: saved.sync,
            frozen: saved.frozen,
            bypass: saved.bypass,
            midi_gate: saved.midi_gate,
            playing: saved.playing,
        })))?;
        self.next_timbre_id = self.next_timbre_id.max(
            parts
                .iter()
                .map(|p| p.meta.id)
                .max()
                .unwrap_or(0)
                .saturating_add(1),
        );
        self.parts = parts;
        self.meta = saved.meta;
        self.active = if self.parts.iter().any(|c| c.meta.id == saved.active_timbre) {
            saved.active_timbre
        } else {
            0
        };
        self.scene_revision = self.scene_revision.wrapping_add(1);
        self.preset = saved.preset;
        self.sample = sample;
        self.source = saved.source;
        self.region = saved.region;
        self.fades = saved.fades;
        self.sync = saved.sync;
        self.frozen = saved.frozen;
        self.bypass = saved.bypass;
        self.midi_gate = saved.midi_gate;
        self.playing = saved.playing;
        self.selection = saved.selection;
        self.offer_workers();
        Ok(())
    }

    fn request(&mut self, request: &Value) -> Result<Value, String> {
        if self.meta.id == 0 {
            return self.scene_request(request);
        }
        self.request_part(request)
    }
    fn request_part(&mut self, request: &Value) -> Result<Value, String> {
        self.drain();
        let url = request["path"].as_str().ok_or("Missing request path")?;
        let path = url.split('?').next().unwrap_or(url);
        let data = &request["data"];
        let post = request["method"] == "POST";
        if post
            && self.meta.kind.keeps_role()
            && matches!(
                path,
                "/api/wav" | "/api/input" | "/api/region" | "/api/fades"
            )
        {
            return Err(if self.meta.kind == sad_pet::timbres::Kind::Vocal {
                "A Vocal timbre sings its score".into()
            } else {
                "A bus takes its input from the other timbres".into()
            });
        }
        let ok = || json!({"ok":true});
        let binary = |bytes: Vec<u8>| json!({"binary":BASE64.encode(bytes),"mime":"audio/wav"});
        if path.starts_with("/api/preset-library") {
            return self.preset_library_request(path, post, data);
        }
        let result = match (path, post) {
            ("/api/state", _) => Ok(self.state()),
            ("/api/preset", false) => Ok(json!(self.preset)),
            ("/api/presets", false) => Ok(json!(
                (0..=sad_pet::startup::INDEX)
                    .map(Preset::builtin)
                    .collect::<Vec<_>>()
            )),
            ("/api/color-defaults", false) => Ok(
                json!({"max_slots":sad_pet::color::MAX_COLOR_SLOTS,"slot":sad_pet::color::ColorSlot::default(),"delay_tables":sad_pet::radias_delay::display_tables(),"mooz":sad_pet::color::mooz_catalog()}),
            ),
            ("/api/samples", false) => Ok(json!(sad_pet::samples::TRACKS)),
            ("/api/sample.wav", false) => Ok(binary(wav::bytes(
                self.sample.sample_rate,
                self.sample.frames(),
            )?)),
            ("/api/midi", _) => {
                let name = if self.standalone {
                    "All MIDI inputs"
                } else {
                    "DAW MIDI"
                };
                Ok(json!({"ports":[{"id":0,"name":name}],"connected":name}))
            }
            ("/api/preset", true) => {
                if data["format"] != "evil-pet-rust-v1" {
                    return Err("Use a SAD PET preset (.sad.json)".into());
                }
                let mut preset: Preset =
                    serde_json::from_value(data.clone()).map_err(|e| e.to_string())?;
                timbres::keep_role(&mut preset, &self.preset, self.meta.kind);
                preset.sanitize();
                self.enqueue(Command::Preset(Box::new(
                    preset.prepared_clone_for(self.rate),
                )))?;
                // A loaded preset can change the timbre's engine type.
                self.meta.kind = sad_pet::timbres::Kind::of(&preset);
                self.preset = preset;
                self.sync_vocal();
                Ok(json!(self.preset))
            }
            ("/api/parameters", true) => {
                let changes = parameters::parse(data)?;
                // Host-backed values arrive through the existing atomic parameter
                // snapshot. Only non-host controls need an audio command.
                let extra: Box<[_]> = changes
                    .iter()
                    .copied()
                    .filter(|change| self.meta.id != 0 || change.host_index().is_none())
                    .collect();
                if !extra.is_empty() {
                    self.enqueue(Command::Parameters(extra))?;
                }
                if parameters::apply(&mut self.preset, &changes) {
                    self.selection.dirty = true;
                }
                Ok(ok())
            }
            ("/api/vocal/score", true) => {
                let settings = self
                    .preset
                    .vocal
                    .as_deref_mut()
                    .ok_or("Select a Vocal timbre")?;
                let mut score: sad_pet::vocal::voice::score::Score =
                    serde_json::from_value(data["score"].clone()).map_err(|e| e.to_string())?;
                score.sanitize();
                settings.score = score;
                let reply = json!({"ok":true,"score":settings.score});
                self.sync_vocal();
                self.selection.dirty = true;
                Ok(reply)
            }
            ("/api/vocal/units", _) => Ok(json!(
                self.voice_service()
                    .units(self.meta.id)
                    .into_iter()
                    .map(|(note, units)| (note.to_string(), json!(units)))
                    .collect::<serde_json::Map<_, _>>()
            )),
            ("/api/vocal/state", _) => {
                let progress = self.voice_service().progress(self.meta.id);
                // Where a looping score is in its pass.
                let host_tempo = self.vocal.as_ref().and_then(|m| m.tempo().0);
                let looping = self.preset.vocal.as_deref().and_then(|s| {
                    sad_pet::vocal::looping(&s.score, host_tempo.as_deref(), self.rate)
                });
                let song = sad_pet::vocal::Song {
                    looping,
                    ..Default::default()
                };
                let position = song.wrap(self.song_position);
                Ok(json!({
                    "playing": self.song_playing,
                    "position": position as f64 / self.rate as f64,
                    "clips": progress.rendered,
                    "rate": self.rate,
                    "progress": progress,
                    "host": self.sync && !self.standalone,
                }))
            }
            ("/api/sequencer/catalog", false) => Ok(sad_pet::sequence::catalog()),
            ("/api/drums/catalog", false) => Ok(sad_pet::drums::catalog()),
            ("/api/filter/response", true) => sad_pet::dsp::filter::response_request(data),
            ("/api/drums/edit", true) => {
                let edits = sad_pet::drums::parse_edits(data)?;
                if self.preset.drums.is_none() {
                    return Err("Choose a drum timbre".into());
                }
                self.enqueue(Command::Drums(edits.clone().into_boxed_slice()))?;
                self.preset.drums.as_mut().unwrap().edit(&edits);
                self.selection.dirty = true;
                Ok(ok())
            }
            ("/api/drums/trigger", true) => {
                self.enqueue(Command::DrumTrigger(
                    data["track"].as_u64().unwrap_or(0).min(6) as usize,
                    data["velocity"].as_f64().unwrap_or(1.) as f32,
                ))?;
                Ok(ok())
            }
            ("/api/sequencer/edit", true) => {
                let edits = sad_pet::sequence::parse_edits(data)?;
                self.enqueue(Command::Sequence(edits.clone().into_boxed_slice()))?;
                self.preset.sequencer.edit(&edits);
                self.selection.dirty = true;
                Ok(ok())
            }
            ("/api/sequencer/view", true) => {
                self.enqueue(Command::SequenceView(
                    data["offset"].as_u64().unwrap_or(0) as usize
                ))?;
                Ok(ok())
            }
            ("/api/transport", true) => {
                let transport: Transport =
                    serde_json::from_value(data.clone()).map_err(|e| e.to_string())?;
                let sync = transport.sync.map(|v| v && !self.standalone);
                self.enqueue(Command::Transport(Transport {
                    playing: transport.playing,
                    midi_gate: transport.midi_gate,
                    frozen: transport.frozen,
                    bypass: transport.bypass,
                    sync,
                    clear_hold: transport.clear_hold,
                }))?;
                if let Some(v) = transport.playing {
                    self.playing = v;
                }
                if let Some(v) = transport.midi_gate {
                    self.midi_gate = v;
                }
                if let Some(v) = transport.frozen {
                    self.frozen = v;
                }
                if let Some(v) = transport.bypass {
                    self.bypass = v;
                }
                if let Some(v) = sync {
                    self.sync = v;
                }
                Ok(ok())
            }
            ("/api/input", true) => {
                if !self.input_available() {
                    return Err("Use SAD PET FX for audio input".into());
                }
                let source = if data["active"] == false {
                    Source::File
                } else {
                    match data["mode"].as_str() {
                        Some("effects") => Source::Effects,
                        Some("grains") => Source::Grains,
                        _ => match self.source {
                            Source::File => Source::Grains,
                            source => source,
                        },
                    }
                };
                let prepared =
                    self.prepared_source(self.sample.clone(), source, self.region, self.fades);
                self.enqueue(Command::Source(Box::new(prepared)))?;
                self.source = source;
                self.frozen = false;
                Ok(ok())
            }
            ("/api/region", true) => {
                if self.source != Source::File {
                    return Err("Load a file to select a region".into());
                }
                let region: Region =
                    serde_json::from_value(data.clone()).map_err(|e| e.to_string())?;
                region.bounds(self.sample.len())?;
                self.enqueue(Command::Region(region))?;
                self.region = region;
                Ok(json!(region))
            }
            ("/api/fades", true) => {
                let fades: Fades =
                    serde_json::from_value(data.clone()).map_err(|e| e.to_string())?;
                fades.validate()?;
                self.enqueue(Command::Fades(fades))?;
                self.fades = fades;
                Ok(json!(fades))
            }
            ("/api/wav", true) => {
                let bytes = BASE64
                    .decode(data["base64"].as_str().ok_or("Missing audio file")?)
                    .map_err(|e| e.to_string())?;
                if bytes.len() > MAX_BYTES {
                    return Err("File exceeds 128 MB".into());
                }
                let mut sample = wav::decode(
                    Cursor::new(bytes),
                    data["name"].as_str().unwrap_or("Sample.wav").into(),
                )?;
                sample.library_id = data["id"].as_str().map(str::to_string);
                self.use_sample(Arc::new(sample), Fades::default())
            }
            ("/api/note", true) => {
                let message = serde_json::from_value(data.clone()).map_err(|e| e.to_string())?;
                self.enqueue(Command::Midi(message))?;
                Ok(ok())
            }
            ("/api/step-entry", true) => {
                let index = data["step"].as_u64().map(|n| n as usize);
                if index.is_some_and(|i| i >= 16) {
                    return Err("Step must be between 0 and 15".into());
                }
                self.enqueue(Command::Step {
                    index,
                    reset: data["reset"] == true,
                    clear: data["clear"] == true,
                })?;
                Ok(ok())
            }
            ("/api/trim", true) => {
                let frames = self.trimmed(data)?;
                Ok(binary(wav::bytes(self.sample.sample_rate, &frames)?))
            }
            _ if path.starts_with("/api/samples/") && path.ends_with(".wav") && !post => {
                let id = &path[13..path.len() - 4];
                let sample = sad_pet::samples::get(id).ok_or("Unknown factory sample")?;
                Ok(binary(wav::bytes(sample.sample_rate, sample.frames())?))
            }
            _ => Err(format!("Unsupported plugin request: {path}")),
        };
        if result.is_ok()
            && post
            && matches!(
                path,
                "/api/preset" | "/api/wav" | "/api/region" | "/api/fades" | "/api/input"
            )
        {
            self.selection.dirty = true;
        }
        result
    }

    /// Replaces this timbre's file source; the engine receives it on the next block.
    fn use_sample(&mut self, sample: Arc<SampleBuffer>, fades: Fades) -> Result<Value, String> {
        fades.validate()?;
        let source = self.prepared_source(sample.clone(), Source::File, Region::default(), fades);
        self.enqueue(Command::Source(Box::new(source)))?;
        self.sample = sample;
        self.source = Source::File;
        self.region = Region::default();
        self.fades = fades;
        self.frozen = false;
        Ok(
            json!({"name":self.sample.name,"seconds":self.sample.duration(),"sample_id":self.sample.library_id}),
        )
    }

    /// The selected region of the file with its fades applied.
    fn trimmed(&self, data: &Value) -> Result<Vec<Stereo>, String> {
        let region: Region = serde_json::from_value(data.clone()).map_err(|e| e.to_string())?;
        let fades: Fades =
            serde_json::from_value(data["fades"].clone()).map_err(|e| e.to_string())?;
        let range = region.bounds(self.sample.len())?;
        fades.validate()?;
        Ok(self.sample.frames()[range.clone()]
            .iter()
            .enumerate()
            .map(|(i, f)| {
                f.map(|v| v * fades.gain((range.start + i) as f64, self.sample.sample_rate, &range))
            })
            .collect())
    }
}

impl Audio {
    fn source(&mut self, source: &mut PreparedSource) {
        self.engine
            .exchange_source(&mut source.sample, &mut source.live);
        if self.engine.live_input.is_none() {
            let _ = self.engine.set_region(source.region);
            let _ = self.engine.set_fades(source.fades);
        }
        self.engine.set_frozen(false);
        self.engine.exchange_processed_source(&mut source.processed);
    }
    /// The longest lead-in of a Vocal part's first note, before 0.
    fn vocal_lead_in(&self) -> u64 {
        let mut lead_in = self.vocal.as_ref().map_or(0, |v| v.track.song.lead_in());
        let mut part = self.parts.as_ref();
        while let Some(p) = part {
            if let Some(v) = &p.audio.vocal {
                lead_in = lead_in.max(v.track.song.lead_in());
            }
            part = p.next.as_ref();
        }
        lead_in
    }
    fn begin(&mut self, values: &[f32], bpm: f32, beat: Option<f64>, playing: bool, frames: usize) {
        let sequence_was_enabled = self.engine.preset.sequencer.enabled;
        let mut preset_changed = false;
        let mut numeric_changed = false;
        while self.retired.slots() > 0 {
            let Ok(mut update) = self.commands.pop() else {
                break;
            };
            match &mut update.command {
                Command::Preview(preview) => std::mem::swap(&mut self.preview, preview),
                Command::PreviewStop => {
                    if let Some(preview) = &mut self.preview {
                        preview.stop();
                    }
                }
                Command::Drums(edits) => self.engine.apply_drum_edits(edits),
                Command::DrumTrigger(track, velocity) => {
                    self.engine.trigger_drum(*track, *velocity)
                }
                Command::AddPart(part) => {
                    if let Some(mut part) = part.take() {
                        part.next = self.parts.take();
                        self.parts = Some(part);
                    }
                }
                Command::RemovePart(id, retired) => {
                    timbres::remove_part(&mut self.parts, *id, retired)
                }
                Command::PartMix(id, mix) => self.set_part_mix(*id, *mix),
                Command::Preset(preset) => {
                    self.engine.exchange_preset(preset);
                    preset_changed = true;
                }
                Command::Parameters(changes) => {
                    numeric_changed |= parameters::apply(&mut self.engine.preset, changes);
                }
                Command::Sequence(edits) => self.engine.apply_sequence_edits(edits),
                Command::SequenceView(offset) => self.engine.set_sequence_view(*offset),
                Command::Source(source) => self.source(source),
                Command::Restore(state) => {
                    std::mem::swap(&mut self.parts, &mut state.parts);
                    self.meta.mix = state.mix;
                    self.engine.midi(Message::Reset);
                    self.engine.exchange_preset(&mut state.preset);
                    self.source(&mut state.source);
                    self.sync = state.sync;
                    self.engine.bypass = state.bypass;
                    self.engine.set_midi_gate(state.midi_gate);
                    self.engine.set_playing(state.playing);
                    self.engine.set_frozen(state.frozen);
                    preset_changed = true;
                }
                Command::Transport(t) => {
                    if let Some(v) = t.sync {
                        self.sync = v;
                        self.expected_beat = None;
                    }
                    if let Some(v) = t.midi_gate {
                        self.engine.set_midi_gate(v);
                    }
                    if let Some(v) = t.playing {
                        self.engine.set_playing(v);
                        let lead_in = self.vocal_lead_in();
                        self.clock.play(v, lead_in);
                    }
                    if let Some(v) = t.frozen {
                        self.engine.set_frozen(v);
                    }
                    if let Some(v) = t.bypass {
                        self.engine.bypass = v;
                    }
                    if t.clear_hold {
                        self.engine.clear_hold();
                    }
                }
                Command::Region(region) => {
                    let _ = self.engine.set_region(*region);
                }
                Command::Fades(fades) => {
                    let _ = self.engine.set_fades(*fades);
                }
                Command::Midi(message) => self.engine.midi(*message),
                Command::VocalTransport { playing, start } => {
                    if let Some(start) = start {
                        self.clock.start = *start;
                        if !self.clock.playing {
                            self.clock.position = *start as i64;
                        }
                    }
                    if let Some(playing) = playing {
                        let lead_in = self.vocal_lead_in();
                        self.clock.play(*playing, lead_in);
                    }
                }
                Command::Step {
                    index,
                    reset,
                    clear,
                } => {
                    self.engine.set_step_entry(*index, *reset);
                    if *clear {
                        self.engine.clear_step_notes();
                    }
                }
                // Any previous pool goes back with the update, so its
                // threads are joined on the control thread.
                Command::Workers(pool) => std::mem::swap(&mut self.pool, pool),
            }
            self.revision = update.revision;
            // Space was checked above; no allocation can be freed in this callback.
            let result = self.retired.push(update);
            debug_assert!(result.is_ok());
        }
        for ((spec, previous), &value) in parameters::specs()
            .iter()
            .zip(&mut self.parameters)
            .zip(values)
        {
            if preset_changed || value != *previous {
                let before = spec.get(&self.engine.preset);
                spec.set(&mut self.engine.preset, value);
                *previous = value;
                numeric_changed |= before != spec.get(&self.engine.preset);
            }
        }
        if numeric_changed {
            parameters::mirror(&mut self.engine.preset);
        }
        let bpm = if bpm.is_finite() && (20.0..=300.0).contains(&bpm) {
            bpm
        } else {
            self.engine.preset.params.bpm
        };
        let previous_bpm = self.engine.preset.params.bpm;
        if self.sync {
            let jumped = playing
                && beat
                    .zip(self.expected_beat)
                    .is_some_and(|(now, expected)| (now - expected).abs() > 0.02);
            let started = playing && !self.host_playing;
            let sequence_started = self.engine.preset.sequencer.enabled && !sequence_was_enabled;
            let drums = self.engine.preset.drums.is_some();
            let performance = self.engine.preset.performance.mode != Mode::Keys;
            if performance
                && (playing != self.host_playing
                    || preset_changed
                    || sequence_started
                    || self.engine.playing != playing)
            {
                self.engine.set_playing(playing);
            }
            self.engine.set_sequence_transport(playing);
            if (self.engine.preset.sequencer.enabled || drums) && playing && !self.engine.playing {
                self.engine.set_playing(true);
            }
            let moved = started || jumped || sequence_started;
            let sequencer = self.engine.preset.sequencer.enabled;
            self.engine.host_transport(
                bpm,
                beat,
                (performance || drums || sequencer) && (moved || preset_changed),
                sequencer && moved,
            );
        } else {
            self.engine.set_sequence_transport(true);
        }
        if numeric_changed || self.engine.preset.params.bpm != previous_bpm {
            self.engine.refresh_parameters();
        }
        self.host_bpm = bpm;
        self.host_playing = playing;
        self.expected_beat = beat.map(|b| {
            b + if playing {
                frames as f64 * bpm as f64 / (60.0 * self.engine.sample_rate as f64)
            } else {
                0.0
            }
        });
        self.source_renderer.update(&mut self.engine);
        // Vocal timbres play at the DAW's song position, or at the scene's
        // own clock in the app and while a host has no transport.
        let (song_playing, song_position, tempo) = match beat.filter(|_| self.sync) {
            Some(beat) => {
                self.tempo.observe(beat, bpm);
                let seconds = self.tempo.seconds(beat.max(0.0), bpm);
                let position = (seconds * self.engine.sample_rate as f64).round() as i64;
                (playing, position, Some(&self.tempo))
            }
            None => (self.clock.playing, self.clock.position, None),
        };
        self.song = (song_playing, song_position);
        if let Some(vocal) = &mut self.vocal {
            vocal.begin(song_position, song_playing, tempo, self.offline);
        }
        let mut part = self.parts.as_mut();
        let mut solo = self.meta.mix.solo;
        let mut open = 0;
        while let Some(p) = part {
            p.audio.begin(&[], bpm, beat, playing, frames);
            if let Some(vocal) = &mut p.audio.vocal {
                vocal.begin(song_position, song_playing, tempo, self.offline);
            }
            let kind = p.audio.meta.kind;
            solo |= p.audio.meta.mix.solo && !kind.routing();
            if kind == sad_pet::timbres::Kind::Bus {
                open |= 1_u16 << p.audio.meta.bus;
            }
            part = p.next.as_mut();
        }
        self.buses = open != 0;
        self.audible = !self.meta.mix.muted && (!solo || self.meta.mix.solo);
        // A bus feeds only the buses that render after it: no loop can form.
        let parts = || std::iter::successors(self.parts.as_deref(), |p| p.next.as_deref());
        let feeds =
            sad_pet::timbres::bus_feeds(parts().map(|p| (&p.audio.meta, &p.audio.engine.preset)));
        self.bus_order = sad_pet::timbres::BusOrder::new(
            parts()
                .filter(|p| p.audio.meta.kind == sad_pet::timbres::Kind::Bus)
                .map(|p| p.audio.meta.bus),
            &feeds,
        );
        let routes = |meta: &sad_pet::timbres::Meta| match meta.kind {
            sad_pet::timbres::Kind::Bus if meta.bus > 0 => {
                u16::from(feeds[meta.bus as usize - 1]) << 1
            }
            kind if kind.routing() => 0,
            _ => open,
        };
        self.engine.set_open_buses(routes(&self.meta));
        let mut part = self.parts.as_mut();
        while let Some(p) = part {
            let mix = p.audio.meta.mix;
            // Solo selects timbres; buses and Master keep processing them.
            p.audio.audible = !mix.muted && (p.audio.meta.kind.routing() || !solo || mix.solo);
            p.audio.engine.set_open_buses(routes(&p.audio.meta));
            part = p.next.as_mut();
        }
    }
    fn end(&mut self, frames: usize) {
        let mut part = self.parts.as_mut();
        while let Some(p) = part {
            p.audio.end(frames);
            part = p.next.as_mut();
        }
        self.clock.advance(frames);
        self.frames_since_feedback += frames;
        if self.frames_since_feedback >= self.engine.sample_rate as usize / 30
            && self.feedback.slots() > 0
        {
            self.engine.realtime_view(&mut self.view);
            let result = self.feedback.push(Feedback {
                revision: self.revision,
                view: self.view,
                bpm: self.host_bpm,
                host_playing: self.host_playing,
                song_playing: self.song.0,
                song_position: self.song.1,
            });
            debug_assert!(result.is_ok());
            self.frames_since_feedback = 0;
        }
    }
}

/// Owned bytes returned across the C ABI. Always release with sad_bytes_free.
#[repr(C)]
pub struct Bytes {
    pub data: *mut u8,
    pub len: usize,
}
impl Bytes {
    fn new(bytes: Vec<u8>) -> Self {
        let mut bytes = bytes.into_boxed_slice();
        let result = Self {
            data: bytes.as_mut_ptr(),
            len: bytes.len(),
        };
        std::mem::forget(bytes);
        result
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sad_parameters() -> Bytes {
    Bytes::new(serde_json::to_vec(parameters::specs()).unwrap_or_default())
}

#[unsafe(no_mangle)]
pub extern "C" fn sad_control_new(effect: bool) -> *mut Controller {
    Box::into_raw(Box::new(Controller::new_plugin(effect)))
}

/// The standalone app's scene: free transport, audio input and a disk library.
#[unsafe(no_mangle)]
pub extern "C" fn sad_control_new_standalone() -> *mut Controller {
    Box::into_raw(Box::new(Controller::new_scene(false, true)))
}

// The wrapper serializes Controller calls and calls prepare/free only while audio is stopped.
// Audio methods have exactly one owner: the DAW audio callback. Buffers are valid for each call.
/// # Safety
/// The pointer must be null or a live Controller from sad_control_new, used exclusively and freed once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sad_control_free(control: *mut Controller) {
    if !control.is_null() {
        drop(unsafe { Box::from_raw(control) });
    }
}
/// # Safety
/// The pointer must be null or a live Audio from sad_control_prepare. Stop all audio calls before freeing it.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sad_audio_free(audio: *mut Audio) {
    if !audio.is_null() {
        drop(unsafe { Box::from_raw(audio) });
    }
}
/// # Safety
/// The pointer and length must be an unchanged Bytes result from this library, released exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sad_bytes_free(data: *mut u8, len: usize) {
    if !data.is_null() {
        drop(unsafe { Box::from_raw(std::ptr::slice_from_raw_parts_mut(data, len)) });
    }
}
/// # Safety
/// Controller must be live and exclusively held. Stop and free its previous Audio before preparing again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sad_control_prepare(control: *mut Controller, rate: u32) -> *mut Audio {
    if control.is_null() {
        return std::ptr::null_mut();
    }
    Box::into_raw(Box::new(unsafe { &mut *control }.prepare(rate)))
}
/// # Safety
/// Controller must be exclusively held; values must address len readable floats for the call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sad_control_set_parameters(
    control: *mut Controller,
    values: *const f32,
    len: usize,
) {
    if control.is_null() || values.is_null() || len != parameters::specs().len() {
        return;
    }
    unsafe { &mut *control }.set_values(unsafe { std::slice::from_raw_parts(values, len) });
}
/// # Safety
/// Controller must be exclusively held; values must address len writable floats for the call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sad_control_get_parameters(
    control: *mut Controller,
    values: *mut f32,
    len: usize,
) {
    if control.is_null() || values.is_null() || len != parameters::specs().len() {
        return;
    }
    let current = unsafe { &*control }.values();
    unsafe { std::slice::from_raw_parts_mut(values, len) }.copy_from_slice(&current);
}
/// # Safety
/// Controller must be exclusively held; data must address len readable bytes for the call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sad_control_request(
    control: *mut Controller,
    data: *const u8,
    len: usize,
) -> Bytes {
    if control.is_null() || data.is_null() || len > MAX_BYTES * 2 {
        return Bytes::new(br#"{"status":400,"data":{"error":"Invalid request"}}"#.to_vec());
    }
    let control = unsafe { &mut *control };
    let result = serde_json::from_slice::<Value>(unsafe { std::slice::from_raw_parts(data, len) })
        .map_err(|e| e.to_string())
        .and_then(|v| control.request(&v));
    let envelope = match result {
        Ok(data) => json!({"status":200,"data":data}),
        Err(error) => json!({"status":400,"data":{"error":error}}),
    };
    Bytes::new(serde_json::to_vec(&envelope).unwrap_or_default())
}
/// The scene as the zlib stream hosts store; setStateInformation inflates it.
/// # Safety
/// Controller must be live and exclusively held. Release the returned bytes with sad_bytes_free.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sad_control_save(control: *mut Controller) -> Bytes {
    use std::io::Write as _;
    if control.is_null() {
        return Bytes::new(Vec::new());
    }
    let Ok(state) = unsafe { &mut *control }.save() else {
        return Bytes::new(Vec::new());
    };
    // Saved samples are base64 text that higher levels barely shrink, but
    // they take several times longer on the host's message thread.
    let mut compressed = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::fast());
    let result = compressed
        .write_all(&state)
        .and_then(|()| compressed.finish());
    Bytes::new(result.unwrap_or_default())
}
/// # Safety
/// Controller must be exclusively held; data must address len readable bytes for the call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sad_control_restore(
    control: *mut Controller,
    data: *const u8,
    len: usize,
) -> bool {
    if control.is_null() || data.is_null() || len > MAX_BYTES * 2 {
        return false;
    }
    unsafe { &mut *control }
        .restore(unsafe { std::slice::from_raw_parts(data, len) })
        .is_ok()
}
/// # Safety
/// Audio must be exclusively held by the render thread; values must address len readable floats.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sad_audio_begin(
    audio: *mut Audio,
    values: *const f32,
    len: usize,
    bpm: f32,
    beat: f64,
    has_beat: bool,
    playing: bool,
    frames: usize,
) {
    if audio.is_null() || values.is_null() || len != parameters::specs().len() {
        return;
    }
    unsafe { &mut *audio }.begin(
        unsafe { std::slice::from_raw_parts(values, len) },
        bpm,
        has_beat.then_some(beat),
        playing,
        frames,
    );
}
/// # Safety
/// Audio must be exclusively held by the host render thread.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sad_audio_time_signature(
    audio: *mut Audio,
    numerator: u32,
    denominator: u32,
) {
    if let Some(audio) = unsafe { audio.as_mut() } {
        audio.time_signature(numerator, denominator);
    }
}
/// # Safety
/// Audio must be exclusively held by the host render thread.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sad_audio_offline(audio: *mut Audio, offline: bool) {
    if let Some(audio) = unsafe { audio.as_mut() } {
        audio.offline = offline;
    }
}
/// # Safety
/// Audio must be exclusively held by the render thread; data must address len readable MIDI bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sad_audio_midi(audio: *mut Audio, data: *const u8, len: usize) {
    if audio.is_null() || data.is_null() {
        return;
    }
    let audio = unsafe { &mut *audio };
    for &byte in unsafe { std::slice::from_raw_parts(data, len) } {
        if let Some(message) = audio.parser.feed(byte) {
            audio.controls.record(message);
            audio.midi(message);
        }
    }
}
/// # Safety
/// Audio must be exclusively held. Non-null inputs and both outputs must address frames floats; each input may alias its corresponding output.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sad_audio_process(
    audio: *mut Audio,
    left: *const f32,
    right: *const f32,
    out_left: *mut f32,
    out_right: *mut f32,
    frames: usize,
) {
    if audio.is_null() || out_left.is_null() || out_right.is_null() {
        return;
    }
    let audio = unsafe { &mut *audio };
    // Each chunk's input is read before any of it is written; DAWs normally
    // supply in-place buffers.
    let mut input = [[0.0; 2]; timbres::CHUNK];
    let mut output = [[0.0; 2]; timbres::CHUNK];
    let mut start = 0;
    while start < frames {
        let n = (frames - start).min(timbres::CHUNK);
        for (i, frame) in input[..n].iter_mut().enumerate() {
            *frame = [
                if left.is_null() {
                    0.0
                } else {
                    unsafe { *left.add(start + i) }
                },
                if right.is_null() {
                    0.0
                } else {
                    unsafe { *right.add(start + i) }
                },
            ];
        }
        audio.process_block(&input[..n], &mut output[..n]);
        for (i, frame) in output[..n].iter().enumerate() {
            unsafe {
                *out_left.add(start + i) = frame[0];
                *out_right.add(start + i) = frame[1];
            }
        }
        start += n;
    }
}
/// # Safety
/// Audio must be live and exclusively held by the render thread.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sad_audio_end(audio: *mut Audio, frames: usize) {
    if !audio.is_null() {
        unsafe { &mut *audio }.end(frames);
    }
}

#[cfg(test)]
mod tests;
