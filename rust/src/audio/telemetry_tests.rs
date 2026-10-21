use super::telemetry::{Applied, Publisher};
use super::*;
use std::{
    alloc::{GlobalAlloc, Layout, System},
    cell::Cell,
};

struct Allocator;
thread_local! { static TRACK: Cell<bool> = const { Cell::new(false) }; static COUNT: Cell<usize> = const { Cell::new(0) }; }
fn count() {
    let _ = TRACK.try_with(|tracking| {
        if tracking.get() {
            let _ = COUNT.try_with(|n| n.set(n.get() + 1));
        }
    });
}
unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        count();
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        count();
        unsafe { System.dealloc(ptr, layout) }
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        count();
        unsafe { System.realloc(ptr, layout, size) }
    }
}
#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

fn barrier(publisher: &mut Publisher, region: Region) {
    let (reply, done) = mpsc::sync_channel(1);
    publisher.applied(Applied::Region(Ok(region), reply));
    done.recv_timeout(Duration::from_secs(2)).unwrap().unwrap();
}

#[test]
fn a_blocked_ui_does_not_allocate_stall_or_change_dsp_samples() {
    let sample = Arc::new(SampleBuffer::demo(8000));
    let mut preset = Preset::default();
    preset.performance.mode = crate::performance::Mode::Hold;
    preset.performance.chord = vec![48, 60, 67];
    let runtime = Runtime::new(sample.clone(), preset.clone(), false);
    let state = runtime.state.clone();
    drop(runtime);
    let (mut publisher, worker) = Publisher::new(state.clone());
    let mut engine = Engine::new(48_000, sample.clone(), preset.clone());
    let mut reference = Engine::new(48_000, sample, preset);
    engine.set_playing(true);
    reference.set_playing(true);
    let mut recorder = crate::recording::Recorder::new(48_000);
    recorder.start().unwrap();
    let locked = state.lock().unwrap();
    let mut published = 0;
    for block in 0..160 {
        if matches!(block, 32 | 36) {
            let message = if block == 32 {
                Message::NoteOn {
                    channel: 0,
                    note: 72,
                    velocity: 100,
                }
            } else {
                Message::NoteOff {
                    channel: 0,
                    note: 72,
                    velocity: 0,
                }
            };
            engine.midi(message);
            reference.midi(message);
        }
        if block == 16 {
            for controller in [1, 39, 70, 7] {
                let message = Message::Control {
                    channel: 0,
                    controller,
                    value: 91,
                };
                engine.midi(message);
                reference.midi(message);
            }
        }
        let mut actual = [[0.; 2]; 128];
        let mut expected = actual;
        engine.process(&mut actual);
        reference.process(&mut expected);
        assert_eq!(
            actual.map(|f| f.map(f32::to_bits)),
            expected.map(|f| f.map(f32::to_bits))
        );
        recorder.capture(&actual);
        COUNT.with(|n| n.set(0));
        TRACK.with(|t| t.set(true));
        let success = publisher.publish(&mut engine, &recorder, 12.3, 4, false);
        TRACK.with(|t| t.set(false));
        assert_eq!(
            COUNT.with(Cell::get),
            0,
            "DSP publication touched the allocator"
        );
        published += usize::from(success);
    }
    assert_eq!(
        published, 3,
        "A busy UI must use only the three reserved snapshots"
    );
    assert!(engine.peak.iter().any(|peak| *peak > 0.));
    drop(locked);
    barrier(&mut publisher, engine.region);
    let knobs = serde_json::to_value(engine.knob_states()).unwrap();
    let performance = serde_json::to_value(engine.performance_view()).unwrap();
    let peak = engine.peak;
    assert!(publisher.publish(&mut engine, &recorder, 12.3, 4, false));
    barrier(&mut publisher, engine.region);
    {
        let state = state.lock().unwrap();
        assert_eq!(serde_json::to_value(&state.knobs).unwrap(), knobs);
        assert_eq!(
            serde_json::to_value(&state.performance).unwrap(),
            performance
        );
        assert_eq!(state.peak, peak);
        assert_eq!(state.underruns, 4);
        assert_eq!(state.recording.seconds, recorder.view().seconds);
        assert_eq!(state.recording.id, recorder.view().id);
        assert_eq!(
            serde_json::to_value(&state.preset).unwrap(),
            serde_json::to_value(&engine.preset).unwrap()
        );
    }
    drop(publisher);
    worker.join().unwrap();
}

#[test]
fn native_command_replies_follow_the_published_metadata() {
    let runtime = Runtime::new(Arc::new(SampleBuffer::demo(8000)), Preset::default(), false);
    let preset = Preset {
        name: "Ordered publication".into(),
        ..Preset::builtin(2)
    };
    let (reply, done) = mpsc::sync_channel(1);
    assert!(
        runtime
            .commands
            .send(Command::preset(preset.prepared_clone_for(48_000), reply))
            .is_ok()
    );
    done.recv_timeout(Duration::from_secs(2)).unwrap();
    assert_eq!(runtime.state.lock().unwrap().preset.name, preset.name);
    let (reply, done) = mpsc::sync_channel(1);
    let changes =
        crate::parameters::parse(&serde_json::json!({"filter":0.37,"in_gain":-4.0})).unwrap();
    assert!(
        runtime
            .commands
            .send(Command::Parameters(changes, reply))
            .is_ok()
    );
    done.recv_timeout(Duration::from_secs(2)).unwrap();
    assert_eq!(runtime.state.lock().unwrap().preset.params.filter, 0.37);
    let (reply, done) = mpsc::sync_channel(1);
    let region = Region {
        start: 0.1,
        end: 0.6,
    };
    assert!(
        runtime
            .commands
            .send(Command::Region(region, reply))
            .is_ok()
    );
    assert_eq!(
        done.recv_timeout(Duration::from_secs(2)).unwrap().unwrap(),
        region
    );
    assert_eq!(runtime.state.lock().unwrap().region, region);
    for active in [true, false] {
        let (reply, done) = mpsc::sync_channel(1);
        assert!(
            runtime
                .commands
                .send(Command::Recording(active, reply))
                .is_ok()
        );
        let view = done.recv_timeout(Duration::from_secs(2)).unwrap().unwrap();
        assert_eq!(view.active, active);
        assert_eq!(runtime.state.lock().unwrap().recording.active, active);
    }
    assert!(runtime.state.lock().unwrap().recording_take.is_some());
    assert!(
        runtime
            .commands
            .send(Command::Midi(Message::Program {
                channel: 0,
                value: 3
            }))
            .is_ok()
    );
    let (reply, done) = mpsc::sync_channel(1);
    assert!(
        runtime
            .commands
            .send(Command::Region(region, reply))
            .is_ok()
    );
    done.recv_timeout(Duration::from_secs(2)).unwrap().unwrap();
    assert_eq!(
        runtime.state.lock().unwrap().preset.name,
        Preset::builtin(3).name
    );
    let drums = Preset {
        drums: Some(Box::default()),
        name: "Drums".into(),
        ..Preset::default()
    };
    let (reply, done) = mpsc::sync_channel(1);
    assert!(
        runtime
            .commands
            .send(Command::preset(drums.prepared_clone_for(48_000), reply))
            .is_ok()
    );
    done.recv_timeout(Duration::from_secs(2)).unwrap();
    assert!(
        runtime
            .commands
            .send(Command::Midi(Message::Program {
                channel: 0,
                value: 1
            }))
            .is_ok()
    );
    let (reply, done) = mpsc::sync_channel(1);
    assert!(
        runtime
            .commands
            .send(Command::Region(region, reply))
            .is_ok()
    );
    assert!(done.recv_timeout(Duration::from_secs(2)).unwrap().is_err());
    assert_eq!(runtime.state.lock().unwrap().preset.name, "Drums");
}
