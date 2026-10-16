use super::*;
use std::{
    alloc::{GlobalAlloc, Layout, System},
    cell::Cell,
};

struct Allocator;
thread_local! {
    static TRACK: Cell<bool> = const { Cell::new(false) };
    static HEAP_OPERATIONS: Cell<usize> = const { Cell::new(0) };
}
fn count() {
    let _ = TRACK.try_with(|track| {
        if track.get() {
            let _ = HEAP_OPERATIONS.try_with(|n| n.set(n.get() + 1));
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

fn no_heap(f: impl FnOnce()) {
    HEAP_OPERATIONS.with(|n| n.set(0));
    TRACK.with(|t| t.set(true));
    f();
    TRACK.with(|t| t.set(false));
    assert_eq!(
        HEAP_OPERATIONS.with(Cell::get),
        0,
        "audio allocated or freed heap memory"
    );
}
fn call(control: &mut Controller, path: &str, data: Value) -> Value {
    control
        .request(&json!({"path":path,"method":if data.is_null(){"GET"}else{"POST"},"data":data}))
        .unwrap()
}
fn render(audio: &mut Audio, frames: usize, input: Stereo) -> f32 {
    let mut peak = 0.0_f32;
    for _ in 0..frames {
        let sample = audio.engine.process_input_frame(input);
        assert!(sample.iter().all(|x| x.is_finite()));
        peak = peak.max(sample[0].abs());
    }
    audio.end(frames);
    peak
}

#[test]
fn instrument_is_silent_until_midi_and_instances_are_independent() {
    let mut a = Controller::new(false);
    let mut audio = a.prepare(48000);
    let values = a.values();
    let mut b = Controller::new(false);
    let mut other = b.prepare(48000);
    let other_values = b.values();
    audio.begin(&values, 120.0, None, false, 4096);
    other.begin(&other_values, 120.0, None, false, 4096);
    assert_eq!(render(&mut audio, 4096, [0.0; 2]), 0.0);
    no_heap(|| {
        audio.engine.midi(Message::NoteOn {
            channel: 0,
            note: 60,
            velocity: 110,
        });
        assert!(render(&mut audio, 4096, [0.0; 2]) > 0.001);
        assert_eq!(render(&mut other, 4096, [0.0; 2]), 0.0);
        audio.engine.midi(Message::NoteOff {
            channel: 0,
            note: 60,
            velocity: 0,
        });
    });
}

#[test]
fn effect_processes_audio_and_granular_source_is_prepared_off_thread() {
    let mut control = Controller::new(true);
    let mut audio = control.prepare(44100);
    let values = control.values();
    assert!(!audio.engine.live_input.as_ref().unwrap().effects_only);
    audio.begin(&values, 120.0, None, false, 44100);
    no_heap(|| assert!(render(&mut audio, 44100, [0.2; 2]) > 0.01));
    call(&mut control, "/api/input", json!({"mode":"effects"}));
    audio.begin(&values, 120.0, None, false, 4096);
    no_heap(|| assert!(render(&mut audio, 4096, [0.2; 2]) > 0.01));
    call(&mut control, "/api/input", json!({"mode":"grains"}));
    no_heap(|| {
        audio.begin(&values, 140.0, Some(0.0), true, 4096);
        render(&mut audio, 4096, [0.2; 2]);
    });
    assert!(!audio.engine.live_input.as_ref().unwrap().effects_only);
    call(&mut control, "/api/input", json!({"active":false}));
    no_heap(|| {
        audio.begin(&values, 140.0, Some(0.2), true, 128);
        render(&mut audio, 128, [0.0; 2]);
    });
    assert!(audio.engine.live_input.is_none());
    call(&mut control, "/api/input", json!({"active":true}));
    no_heap(|| audio.begin(&values, 140.0, None, false, 128));
    assert!(!audio.engine.live_input.as_ref().unwrap().effects_only);
    control.drain();
}

#[test]
fn both_fx_racks_waveshaper_changes_automation_and_feedback_are_realtime_safe() {
    use sad_pet::{
        color::ColorSlot, dsp::radias_waveshaper::WaveshaperMode, preset::SaturatorType,
    };
    let mut control = Controller::new(true);
    let mut audio = control.prepare(48000);
    let pre_depth = parameters::specs()
        .iter()
        .position(|p| p.id == "pre_fx1_waveshaper_depth")
        .unwrap();
    let mut preset = control.preset.clone();
    preset.pre_color_chain = vec![ColorSlot {
        id: 40,
        kind: SaturatorType::Waveshaper,
        ..ColorSlot::default()
    }];
    preset.color_chain = vec![ColorSlot {
        id: 41,
        kind: SaturatorType::Compressor,
        ..ColorSlot::default()
    }];
    for mode in WaveshaperMode::ALL {
        preset.pre_color_chain[0].settings.waveshaper_type = mode;
        call(&mut control, "/api/preset", json!(preset));
        let mut values = control.values();
        values[pre_depth] = 0.73;
        no_heap(|| {
            audio.begin(&values, 120.0, Some(1.0), true, 2048);
            render(&mut audio, 2048, [0.2, -0.1]);
        });
        let state = call(&mut control, "/api/state", Value::Null);
        assert!(state["knobs"]["color:40:waveshaper_depth"]["current"].is_number());
        assert!(
            (audio.engine.modulated_pre_color_slots()[0]
                .settings
                .waveshaper_depth
                - 0.73)
                .abs()
                < 1e-5
        );
    }
}

#[test]
fn preset_mode_changes_midi_capture_automation_and_seek_do_not_allocate() {
    let mut control = Controller::new(false);
    let mut audio = control.prepare(48000);
    let mut values = control.values();
    audio.begin(&values, 120.0, Some(0.0), false, 128);
    for mode in [
        Mode::Hold,
        Mode::Arp,
        Mode::Sequencer,
        Mode::SequencerArp,
        Mode::Keys,
    ] {
        let mut preset = control.preset.clone();
        preset.performance.mode = mode;
        preset.performance.hold = true;
        call(&mut control, "/api/preset", json!(preset));
        values = control.values();
        no_heap(|| {
            audio.begin(&values, 139.0, Some(8.0), true, 4096);
            for note in 40..72 {
                audio.engine.midi(Message::NoteOn {
                    channel: 0,
                    note,
                    velocity: 100,
                });
            }
            render(&mut audio, 4096, [0.0; 2]);
            audio.begin(&values, 139.0, Some(32.0), true, 4096);
            render(&mut audio, 4096, [0.0; 2]);
            for note in 40..72 {
                audio.engine.midi(Message::NoteOff {
                    channel: 0,
                    note,
                    velocity: 0,
                });
            }
            audio.begin(&values, 139.0, Some(32.2), false, 128);
            render(&mut audio, 128, [0.0; 2]);
        });
        control.drain();
    }
    let position = parameters::specs()
        .iter()
        .position(|p| p.id == "position")
        .unwrap();
    values[position] = 0.42;
    no_heap(|| audio.begin(&values, 128.0, None, false, 31));
    assert_eq!(audio.engine.preset.params.position, 0.42);
}

#[test]
fn custom_sample_full_preset_and_selection_survive_project_recall() {
    let mut control = Controller::new(false);
    let _audio = control.prepare(48000);
    let frames: Vec<Stereo> = (0..128)
        .map(|i| [(i as f32 * 0.13).sin() * 0.4; 2])
        .collect();
    let bytes = wav::bytes(32000, &frames).unwrap();
    call(
        &mut control,
        "/api/wav",
        json!({"base64":BASE64.encode(bytes),"name":"Portable.wav","id":"custom:portable"}),
    );
    call(
        &mut control,
        "/api/region",
        json!({"start":0.25,"end":0.75}),
    );
    call(
        &mut control,
        "/api/fades",
        json!({"fade_in":0.001,"fade_out":0.002}),
    );
    let mut preset = control.preset.clone();
    preset.name = "Saved project".into();
    preset.params.position = 0.37;
    preset.lfos[2].depth = 0.6;
    preset.color_chain.swap(0, 1);
    call(&mut control, "/api/preset", json!(preset));
    let saved = control.save().unwrap();
    let mut restored = Controller::new(false);
    let mut audio = restored.prepare(96000);
    restored.restore(&saved).unwrap();
    let values = restored.values();
    no_heap(|| {
        audio.begin(&values, 120.0, None, false, 128);
        render(&mut audio, 128, [0.0; 2]);
    });
    assert_eq!(audio.engine.sample.name, "Portable.wav");
    assert_eq!(audio.engine.sample.sample_rate, 32000);
    assert_eq!(audio.engine.sample.frames(), frames);
    assert_eq!(audio.engine.region, control.region);
    assert_eq!(audio.engine.fades, control.fades);
    assert_eq!(audio.engine.preset.name, "Saved project");
    assert_eq!(audio.engine.preset.params.position, 0.37);
    assert_eq!(audio.engine.preset.lfos[2].depth, 0.6);
    assert_eq!(
        audio.engine.preset.color_chain[0].id,
        control.preset.color_chain[0].id
    );
    assert!(restored.restore(b"invalid").is_err());
    assert_eq!(restored.sample.name, "Portable.wav");
}

#[test]
fn host_sync_uses_bpm_and_restarts_sequence_at_the_daw_position() {
    let mut control = Controller::new(false);
    let mut audio = control.prepare(48000);
    let mut preset = control.preset.clone();
    preset.performance.mode = Mode::Sequencer;
    call(&mut control, "/api/preset", json!(preset));
    let values = control.values();
    no_heap(|| {
        audio.begin(&values, 150.0, Some(9.0), true, 2048);
        render(&mut audio, 2048, [0.0; 2]);
    });
    assert_eq!(audio.engine.preset.params.bpm, 150.0);
    assert_eq!(audio.view.step, Some(9));
    no_heap(|| {
        audio.begin(&values, 90.0, Some(2.0), true, 2048);
        render(&mut audio, 2048, [0.0; 2]);
    });
    assert_eq!(audio.view.step, Some(2));
    no_heap(|| audio.begin(&values, 90.0, Some(2.064), false, 128));
    assert!(!audio.engine.playing);
}

#[test]
fn queue_backpressure_never_frees_or_blocks_on_the_audio_thread() {
    let mut control = Controller::new(false);
    let mut audio = control.prepare(48000);
    let values = control.values();
    for _ in 0..QUEUE {
        control
            .enqueue(Command::Preset(Box::new(control.preset.clone())))
            .unwrap();
    }
    no_heap(|| audio.begin(&values, 120.0, None, false, 128));
    // Fill the command queue while keeping the retired ring full.
    for revision in 0..QUEUE {
        control
            .commands
            .push(Update {
                revision: revision as u64,
                command: Command::Region(Region::default()),
            })
            .unwrap_or_else(|_| panic!("queue unexpectedly full"));
    }
    no_heap(|| {
        audio.begin(&values, 120.0, None, false, 128);
        render(&mut audio, 128, [0.0; 2]);
    });
    control.drain();
    no_heap(|| audio.begin(&values, 120.0, None, false, 128));
}
