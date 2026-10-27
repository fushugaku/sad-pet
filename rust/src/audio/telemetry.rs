//! Bounded, reusable DSP snapshots. UI allocation, locking and retirement happen
//! on the publisher; a slow reader never blocks audio or consumes its peaks.
use super::*;
use crate::{
    dsp::telemetry::RealtimeView,
    recording::{Feedback, Recorder},
};

pub(super) enum Applied {
    Program(u8),
    Preset {
        view: Box<Preset>,
        retired: Box<Preset>,
        reply: SyncSender<()>,
    },
    Parameters(Vec<crate::parameters::Change>, SyncSender<()>),
    Sequence(Vec<crate::sequence::Edit>, SyncSender<()>),
    Region(Result<Region, String>, SyncSender<Result<Region, String>>),
    Fades(Result<Fades, String>, SyncSender<Result<Fades, String>>),
    Recording {
        feedback: Feedback,
        result: Result<(), String>,
        reply: SyncSender<Result<crate::recording::View, String>>,
    },
    /// A replaced file, which may be the last reference to many MB of PCM,
    /// and the history buffer of a replaced live input.
    Retired(Arc<SampleBuffer>, Option<SampleBuffer>),
}
impl Applied {
    fn publish(self, state: &Mutex<State>) {
        let mut state = state.lock().unwrap();
        match self {
            Self::Program(value) => {
                let mut preset = Preset::factory(value as usize % 5);
                preset.sanitize();
                state.preset = preset;
            }
            Self::Preset {
                view,
                retired,
                reply,
            } => {
                state.preset = *view;
                let _ = reply.try_send(());
                drop(state);
                drop(retired);
            }
            Self::Parameters(changes, reply) => {
                crate::parameters::apply(&mut state.preset, &changes);
                let _ = reply.try_send(());
            }
            Self::Sequence(edits, reply) => {
                state.preset.sequencer.edit(&edits);
                let _ = reply.try_send(());
            }
            Self::Region(result, reply) => {
                if let Ok(region) = &result {
                    state.region = *region;
                }
                let _ = reply.try_send(result);
            }
            Self::Fades(result, reply) => {
                if let Ok(fades) = &result {
                    state.fades = *fades;
                }
                let _ = reply.try_send(result);
            }
            Self::Recording {
                feedback,
                result,
                reply,
            } => {
                state.recording = feedback.view();
                state.recording_take = feedback.take;
                let _ = reply.try_send(result.map(|_| state.recording.clone()));
            }
            Self::Retired(sample, live) => {
                drop(state);
                drop((sample, live));
            }
        }
    }
}

#[derive(Default)]
struct Snapshot {
    view: RealtimeView,
    params: crate::preset::Params,
    first_mix: Option<(u32, f32)>,
    performance: Option<crate::performance::RealtimeView>,
    steps: [crate::performance::Step; crate::performance::STEPS],
    sample: Option<Arc<SampleBuffer>>,
    recording: Option<Feedback>,
    region: Region,
    fades: Fades,
    center: f32,
    /// Drum and Synth engines show their live history, not the loaded file.
    history: Option<(f64, usize)>,
    cpu: f32,
    underruns: u64,
    device_failed: bool,
}
impl Snapshot {
    fn fill(
        &mut self,
        engine: &mut Engine,
        recorder: &Recorder,
        cpu: f32,
        underruns: u64,
        failed: bool,
    ) {
        debug_assert!(self.sample.is_none() && self.recording.is_none());
        self.sample = Some(engine.sample.clone());
        self.params = engine.preset.params;
        self.first_mix = engine
            .preset
            .color_chain
            .first()
            .map(|slot| (slot.id, slot.mix));
        self.performance = Some(engine.realtime_performance_view());
        self.steps = engine.preset.performance.steps;
        self.recording = Some(recorder.feedback());
        self.center = (engine.center() / engine.sample.len() as f64) as f32;
        self.region = engine.region;
        self.fades = engine.fades;
        self.cpu = cpu;
        self.underruns = underruns;
        self.device_failed = failed;
        engine.realtime_view(&mut self.view);
        self.history = engine
            .preset
            .history()
            .map(|_| (engine.source_duration(), engine.source_frames()));
        if self.history.is_some() {
            self.center = self.view.center as f32;
        }
    }
    fn publish(&mut self, state: &Mutex<State>) {
        let mut state = state.lock().unwrap();
        let v = &self.view;
        state.preset.params = self.params;
        if let (Some((id, mix)), Some(slot)) =
            (self.first_mix, state.preset.color_chain.first_mut())
            && slot.id == id
        {
            slot.mix = mix;
        }
        state.playing = v.playing;
        state.frozen = v.frozen;
        state.bypass = v.bypass;
        state.midi_gate = v.midi_gate;
        state.center = self.center;
        state.region = self.region;
        state.fades = self.fades;
        state.grains.clear();
        state.grains.extend_from_slice(&v.grains[..v.grain_count]);
        state.active_grains = v.grain_count;
        state.notes.clear();
        state.notes.extend_from_slice(&v.notes[..v.note_count]);
        state.preset.performance.chord.clear();
        state
            .preset
            .performance
            .chord
            .extend_from_slice(&v.chord[..v.chord_len]);
        state.preset.performance.steps = self.steps;
        state.performance = self.performance.take().unwrap().into_view();
        state.sequencer = v.sequencer;
        state.step_entry = v.step_entry;
        state.step_edits = v.step_edits;
        let recording = self.recording.take().unwrap();
        state.recording = recording.view();
        state.recording_take = recording.take;
        state.lfos = v.lfos;
        state.envs = v.envs;
        state.peak = v.peak;
        state.cpu_percent = self.cpu;
        state.matrix_values = v.matrix;
        state.knobs = crate::dsp::telemetry::knob_states_from_view(&state.preset, v);
        state.gain_reduction = v
            .gain_reduction
            .iter()
            .filter(|(id, _)| *id != 0)
            .copied()
            .collect();
        state.resynth = crate::resynth::Meter {
            bands: v.resynth_bands,
            frequencies: v.resynth_frequencies,
        };
        state.source_fx = SourceFxStatus {
            pending: v.source_fx_pending,
            error: v.source_fx_error.then_some("Could not prepare Source FX"),
        };
        state.underruns = self.underruns;
        let sample = self.sample.take().unwrap();
        if !Arc::ptr_eq(&state.sample, &sample) {
            state.sample_name = sample.name.clone();
            state.sample_id = sample.library_id.clone();
            state.sample_seconds = sample.duration();
            state.sample_frames = sample.len();
            state.waveform.clone_from(&sample.waveform);
            state.sample = sample;
        }
        if let Some((seconds, frames)) = self.history {
            state.sample_seconds = seconds;
            state.sample_frames = frames;
            state.waveform.clear();
            state
                .waveform
                .extend_from_slice(&v.waveform[..v.waveform_len]);
            state.showing_history = true;
        } else if state.showing_history {
            // Leaving a live history restores the file's own display.
            let file = state.sample.clone();
            state.sample_seconds = file.duration();
            state.sample_frames = file.len();
            state.waveform.clone_from(&file.waveform);
            state.showing_history = false;
        }
        if self.device_failed {
            state.audio_error = Some(
                "Audio device disconnected. Restart the application after reconnecting it.".into(),
            );
        }
    }
}
enum Event {
    Applied(Applied),
    Snapshot(Box<Snapshot>),
}

pub(super) struct Publisher {
    events: rtrb::Producer<Event>,
    available: rtrb::Consumer<Box<Snapshot>>,
    done: Arc<AtomicBool>,
    thread: thread::Thread,
}
impl Publisher {
    pub fn new(state: Arc<Mutex<State>>) -> (Self, thread::JoinHandle<()>) {
        let (events, mut pending) = rtrb::RingBuffer::new(256);
        let (mut returned, available) = rtrb::RingBuffer::new(3);
        for _ in 0..3 {
            assert!(returned.push(Box::<Snapshot>::default()).is_ok());
        }
        let done = Arc::new(AtomicBool::new(false));
        let finished = done.clone();
        let worker = thread::Builder::new()
            .name("evil-state".into())
            .spawn(move || {
                loop {
                    let stop = finished.load(Ordering::Acquire);
                    while let Ok(event) = pending.pop() {
                        match event {
                            Event::Applied(applied) => applied.publish(&state),
                            Event::Snapshot(mut snapshot) => {
                                snapshot.publish(&state);
                                // Its owning sample/recording references were retired here.
                                assert!(returned.push(snapshot).is_ok());
                            }
                        }
                    }
                    if stop {
                        break;
                    }
                    thread::park_timeout(Duration::from_millis(40));
                }
            })
            .expect("start state publisher");
        let handle = worker.thread().clone();
        (
            Self {
                events,
                available,
                done,
                thread: handle,
            },
            worker,
        )
    }
    pub fn full(&self) -> bool {
        self.events.slots() == 0
    }
    pub fn applied(&mut self, applied: Applied) {
        // The sole producer checks capacity before applying a command.
        assert!(self.events.push(Event::Applied(applied)).is_ok());
        self.thread.unpark();
    }
    pub fn publish(
        &mut self,
        engine: &mut Engine,
        recorder: &Recorder,
        cpu: f32,
        underruns: u64,
        failed: bool,
    ) -> bool {
        if self.full() {
            return false;
        }
        let Ok(mut snapshot) = self.available.pop() else {
            return false;
        };
        snapshot.fill(engine, recorder, cpu, underruns, failed);
        assert!(self.events.push(Event::Snapshot(snapshot)).is_ok());
        self.thread.unpark();
        true
    }
}
impl Drop for Publisher {
    fn drop(&mut self) {
        self.done.store(true, Ordering::Release);
        self.thread.unpark();
    }
}
