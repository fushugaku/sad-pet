use crate::{
    Stereo,
    dsp::{Engine, GrainView, SampleBuffer},
    midi::Message,
    preset::Preset,
};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use serde::{Deserialize, Serialize};
use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU64, Ordering},
        mpsc::{self, SyncSender},
    },
    thread,
    time::{Duration, Instant},
};

#[derive(Clone, Copy, Debug, Default, Deserialize)]
pub struct Transport {
    pub playing: Option<bool>,
    pub frozen: Option<bool>,
    pub bypass: Option<bool>,
    pub midi_gate: Option<bool>,
    pub clear_hold: Option<bool>,
}

pub enum Command {
    Preset(Box<Preset>),
    Sample(Arc<SampleBuffer>),
    Transport(Transport),
    Midi(Message),
    StepEntry(Option<usize>, bool, bool),
    Recording(bool, SyncSender<Result<crate::recording::View, String>>),
    Shutdown,
}

#[derive(Clone, Serialize)]
pub struct State {
    pub version: &'static str,
    pub instance_id: String,
    pub sample_rate: u32,
    pub audio_device: String,
    pub audio_error: Option<String>,
    pub audio_enabled: bool,
    pub playing: bool,
    pub frozen: bool,
    pub bypass: bool,
    pub midi_gate: bool,
    pub preset: Preset,
    pub sample_name: String,
    pub sample_id: Option<String>,
    pub sample_seconds: f64,
    pub waveform: Vec<[f32; 2]>,
    pub center: f32,
    pub grains: Vec<GrainView>,
    pub active_grains: usize,
    pub notes: Vec<u8>,
    pub step_entry: Option<usize>,
    pub step_edits: [Option<crate::performance::Step>; 16],
    pub recording: crate::recording::View,
    pub performance: crate::performance::View,
    pub lfos: [f32; 3],
    pub envs: [f32; 3],
    pub matrix_values: [f32; crate::matrix::MAX_ROUTES],
    pub knobs: std::collections::BTreeMap<String, crate::dsp::telemetry::KnobState>,
    pub peak: Stereo,
    pub cpu_percent: f32,
    pub underruns: u64,
    #[serde(skip)]
    pub sample: Arc<SampleBuffer>,
    #[serde(skip)]
    pub recording_take: Option<Arc<crate::recording::Take>>,
}

pub struct Runtime {
    pub commands: SyncSender<Command>,
    pub state: Arc<Mutex<State>>,
    running: Arc<AtomicBool>,
    worker: Option<thread::JoinHandle<()>>,
    stream: Option<cpal::Stream>,
}

fn output_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    mut consumer: rtrb::Consumer<Stereo>,
    underruns: Arc<AtomicU64>,
    failed: Arc<AtomicBool>,
) -> Result<cpal::Stream, String>
where
    T: cpal::SizedSample + cpal::FromSample<f32>,
{
    let channels = config.channels as usize;
    device
        .build_output_stream(
            config,
            move |output: &mut [T], _| {
                let mut missing = 0;
                for frame in output.chunks_mut(channels) {
                    let source = consumer.pop().unwrap_or_else(|_| {
                        missing += 1;
                        [0.0; 2]
                    });
                    for (c, sample) in frame.iter_mut().enumerate() {
                        let value = if channels == 1 {
                            (source[0] + source[1]) * 0.5
                        } else if c < 2 {
                            source[c]
                        } else {
                            0.0
                        };
                        *sample = T::from_sample(value);
                    }
                }
                if missing > 0 {
                    underruns.fetch_add(1, Ordering::Relaxed);
                }
            },
            move |_| {
                failed.store(true, Ordering::Relaxed);
            },
            None,
        )
        .map_err(|e| e.to_string())
}

impl Runtime {
    pub fn new(sample: Arc<SampleBuffer>, preset: Preset, audio: bool) -> Self {
        let (mut producer, consumer) = rtrb::RingBuffer::new(2048);
        let underruns = Arc::new(AtomicU64::new(0));
        let failed = Arc::new(AtomicBool::new(false));
        let mut sr = 48_000;
        let mut device_name = String::new();
        let mut error = None;
        let stream = if audio {
            let result = (|| {
                let device = cpal::default_host()
                    .default_output_device()
                    .ok_or("No audio output device")?;
                device_name = device.name().unwrap_or_else(|_| "Default output".into());
                let supported = device.default_output_config().map_err(|e| e.to_string())?;
                sr = supported.sample_rate().0;
                if !(8_000..=384_000).contains(&sr) {
                    return Err("Unsupported device sample rate".into());
                }
                let config = supported.config();
                match supported.sample_format() {
                    cpal::SampleFormat::F32 => output_stream::<f32>(
                        &device,
                        &config,
                        consumer,
                        underruns.clone(),
                        failed.clone(),
                    ),
                    cpal::SampleFormat::I16 => output_stream::<i16>(
                        &device,
                        &config,
                        consumer,
                        underruns.clone(),
                        failed.clone(),
                    ),
                    cpal::SampleFormat::U16 => output_stream::<u16>(
                        &device,
                        &config,
                        consumer,
                        underruns.clone(),
                        failed.clone(),
                    ),
                    other => Err(format!("Unsupported device sample format {other}")),
                }
            })();
            match result {
                Ok(stream) => Some(stream),
                Err(e) => {
                    error = Some(e);
                    None
                }
            }
        } else {
            None
        };
        let audio_enabled = stream.is_some();
        let device_active = Arc::new(AtomicBool::new(audio_enabled));
        let worker_device = device_active.clone();
        let state = Arc::new(Mutex::new(State {
            version: env!("CARGO_PKG_VERSION"),
            instance_id: format!(
                "{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos()
            ),
            sample_rate: sr,
            audio_device: device_name,
            audio_error: error,
            audio_enabled,
            playing: false,
            frozen: false,
            bypass: false,
            midi_gate: false,
            preset: preset.clone(),
            sample_name: sample.name.clone(),
            sample_id: sample.library_id.clone(),
            sample_seconds: sample.duration(),
            waveform: sample.waveform.clone(),
            center: 0.0,
            grains: Vec::new(),
            active_grains: 0,
            notes: Vec::new(),
            step_entry: None,
            step_edits: [None; 16],
            recording: Default::default(),
            recording_take: None,
            performance: crate::performance::Performance::new(preset.performance.clone()).view(),
            lfos: [0.0; 3],
            envs: [0.0; 3],
            matrix_values: [0.0; crate::matrix::MAX_ROUTES],
            knobs: Default::default(),
            peak: [0.0; 2],
            cpu_percent: 0.0,
            underruns: 0,
            sample: sample.clone(),
        }));
        let running = Arc::new(AtomicBool::new(true));
        let (commands, receiver) = mpsc::sync_channel(256);
        let worker_running = running.clone();
        let worker_state = state.clone();
        let worker=thread::Builder::new().name("evil-dsp".into()).spawn(move || {
            let mut engine=Engine::new(sr,sample,preset);
            let mut recorder=crate::recording::Recorder::new(sr);
            let mut frames=[[0.0;2];128];
            let mut last_publish=Instant::now(); let mut cpu=0.0;
            let mut deadline=Instant::now();
            while worker_running.load(Ordering::Relaxed) {
                for _ in 0..64 {
                    let Ok(command)=receiver.try_recv() else {break;};
                    match command {
                        Command::Preset(preset)=>engine.set_preset(*preset),
                        Command::Sample(sample)=>engine.set_sample(sample),
                        Command::Transport(t)=> {
                            if let Some(v)=t.midi_gate {engine.set_midi_gate(v);}
                            if let Some(v)=t.playing {engine.set_playing(v);}
                            if let Some(v)=t.frozen {engine.set_frozen(v);}
                            if let Some(v)=t.bypass {engine.bypass=v;}
                            if t.clear_hold==Some(true) {engine.clear_hold();}
                        }
                        Command::Midi(message)=>engine.midi(message),
                        Command::StepEntry(step,reset,clear)=> {
                            engine.set_step_entry(step,reset);
                            if clear {engine.clear_step_notes();}
                        },
                        Command::Recording(active,reply)=> {
                            let result = if active {recorder.start()} else {recorder.stop().map(|_|())};
                            if let Ok(mut state)=worker_state.lock() {
                                state.recording=recorder.view();
                                state.recording_take=recorder.take();
                            }
                            let _=reply.try_send(result.map(|_|recorder.view()));
                        },
                        Command::Shutdown=>{worker_running.store(false,Ordering::Relaxed);break;}
                    }
                }
                let device_active=worker_device.load(Ordering::Relaxed);
                if device_active && producer.slots()<frames.len() {thread::sleep(Duration::from_micros(700));continue;}
                let start=Instant::now();
                engine.process(&mut frames);
                recorder.capture(&frames);
                let load=start.elapsed().as_secs_f32()/(frames.len() as f32/sr as f32)*100.0;
                cpu+=(load-cpu)*0.05;
                if device_active {for &frame in &frames {let _=producer.push(frame);}}
                else {
                    deadline+=Duration::from_secs_f64(frames.len() as f64/sr as f64);
                    if let Some(wait)=deadline.checked_duration_since(Instant::now()) {thread::sleep(wait);} else {deadline=Instant::now();}
                }
                if last_publish.elapsed()>=Duration::from_millis(40) {
                    if let Ok(mut state)=worker_state.lock() {
                        state.playing=engine.playing;state.frozen=engine.frozen;state.bypass=engine.bypass;state.midi_gate=engine.midi_gate;
                        state.preset=engine.preset.clone();state.center=(engine.center()/engine.sample.len() as f64) as f32;
                        state.grains=engine.grain_views();state.active_grains=engine.active_grains();state.notes=engine.held_notes();
                        state.performance=engine.performance_view();
                        state.step_entry=engine.step_entry;
                        state.step_edits=engine.step_edits;
                        state.recording=recorder.view();state.recording_take=recorder.take();
                        state.lfos=engine.lfo_values;state.envs=engine.env_values;state.peak=engine.peak;state.cpu_percent=cpu;
                        state.matrix_values=engine.matrix_values;
                        state.knobs=engine.knob_states();
                        state.underruns=underruns.load(Ordering::Relaxed);
                        if !Arc::ptr_eq(&state.sample,&engine.sample) {
                            state.sample_name=engine.sample.name.clone();state.sample_id=engine.sample.library_id.clone();state.sample_seconds=engine.sample.duration();state.waveform=engine.sample.waveform.clone();state.sample=engine.sample.clone();
                        }
                        if failed.load(Ordering::Relaxed) {state.audio_error=Some("Audio device disconnected. Restart the application after reconnecting it.".into());}
                    }
                    engine.peak=[0.0;2];last_publish=Instant::now();
                }
            }
        }).expect("start DSP worker");
        let runtime = Self {
            commands,
            state,
            running,
            worker: Some(worker),
            stream,
        };
        if let Some(stream) = &runtime.stream {
            // Pre-fill the queue before the device starts requesting samples.
            thread::sleep(Duration::from_millis(45));
            if let Err(error) = stream.play() {
                let mut state = runtime.state.lock().unwrap();
                state.audio_error = Some(error.to_string());
                state.audio_enabled = false;
                device_active.store(false, Ordering::Relaxed);
            }
        }
        runtime
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        self.stream.take();
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}
