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
        let sample = audio.process_frame(input);
        assert!(sample.iter().all(|x| x.is_finite()));
        peak = peak.max(sample[0].abs());
    }
    audio.end(frames);
    peak
}

#[test]
fn new_plugin_starts_with_glasses5_and_restoration_replaces_it() {
    for effect in [false, true] {
        let mut control = Controller::new_plugin(effect);
        let state = call(&mut control, "/api/state", Value::Null);
        assert_eq!(state["preset"]["name"], "glasses5");
        assert_eq!(state["preset_selection"]["id"], "factory:5");
        assert_eq!(state["preset_selection"]["dirty"], false);
        assert_eq!(state["file_source"]["sample_id"], "factory:dusty-keys:v1");
        assert_eq!(state["file_source"]["sample_seconds"], 12.0);
        assert_eq!(control.fades.fade_in, 1.0);
        assert_eq!(control.fades.fade_out, 1.0);
        assert_eq!(control.preset.pre_color_chain.len(), 2);
        assert_eq!(control.preset.color_chain.len(), 5);
        assert_eq!(control.preset.resynth.count, 4);
        assert_eq!(control.preset.sequencer.tracks.len(), 2);
        assert!(control.source == if effect { Source::Grains } else { Source::File });
        assert_eq!(
            call(&mut control, "/api/presets", Value::Null)[5]["name"],
            "glasses5"
        );

        let precise = json!(control.preset);
        let mut normalized: Vec<f32> = parameters::specs()
            .iter()
            .map(|spec| {
                let value = spec.get(&control.preset);
                let normalized = (value - spec.min) / (spec.max - spec.min);
                spec.min + normalized * (spec.max - spec.min)
            })
            .collect();
        control.set_values(&normalized);
        assert!(!control.selection.dirty);
        assert_eq!(json!(control.preset), precise);
        let volume = parameters::specs()
            .iter()
            .position(|spec| spec.id == "master_volume")
            .unwrap();
        normalized[volume] = 0.4;
        control.set_values(&normalized);
        assert!(control.selection.dirty);
        assert_eq!(control.preset.params.master_volume, 0.4);

        // Existing projects remain authoritative, including their source and fades.
        let mut previous = Controller::new(effect);
        previous.preset.name = "Saved project".into();
        let saved = previous.snapshot().unwrap();
        control.restore_saved(saved).unwrap();
        assert_eq!(control.preset.name, "Saved project");
        assert_eq!(control.sample.name, "Glass sequence");
        assert_eq!(control.fades.fade_in, 0.0);

        // Factory recall carries the original source, not the previous sample.
        call(
            &mut control,
            "/api/preset-library/factory",
            json!({"index":5}),
        );
        assert_eq!(control.sample.name, "Dusty keys");
        assert_eq!(control.preset.name, "glasses5");
        assert_eq!(control.fades.fade_out, 1.0);
    }
}

#[test]
fn neuropolis_tracks_follow_daw_transport_survive_project_recall_and_edit_without_heap_work() {
    use sad_pet::sequence::{Target, Track};
    let mut control = Controller::new(false);
    let mut preset = control.preset.clone();
    preset.sequencer.enabled = true;
    preset.sequencer.tracks = (1..=129)
        .map(|id| {
            let mut t = Track {
                id,
                target: if id == 1 {
                    Target::Notes
                } else {
                    Target::Oscillator { index: 3 }
                },
                ..Track::default()
            };
            t.config[3] = if id % 2 == 0 { 2 } else { 1 };
            t.steps[1].pitch = 72;
            t.destinations = [
                Some("filter".into()),
                Some("resynth_osc4_control1".into()),
                Some("fx1_mix".into()),
            ];
            t.steps[7].destination2 = 0.13;
            t
        })
        .collect();
    call(&mut control, "/api/preset", json!(preset));
    let mut audio = control.prepare(48000);
    let values = control.values();
    no_heap(|| {
        audio.begin(&values, 120.0, Some(0.0), false, 128);
        render(&mut audio, 128, [0.0; 2]);
        assert!(!audio.engine.playing);
        audio.begin(&values, 120.0, Some(0.0), true, 8192);
        assert!(render(&mut audio, 8192, [0.0; 2]) > 0.001);
    });
    assert_eq!(audio.engine.sequence_view().tracks[0].step, 1);
    assert_eq!(audio.engine.sequence_view().tracks[1].step, 0);
    call(
        &mut control,
        "/api/sequencer/edit",
        json!([{"track":1,"stage":7,"field":0,"value":79},{"track":2,"field":3,"value":65},
            {"track":1,"stage":7,"field":16,"value":-0.23},{"track":2,"stage":7,"field":18,"value":0.14}]),
    );
    no_heap(|| {
        audio.begin(&values, 120.0, Some(8192.0 / 24000.0), true, 128);
        render(&mut audio, 128, [0.0; 2]);
    });
    assert_eq!(audio.engine.sequence_view().tracks[0].step, 1);
    let mut changed = control.preset.clone();
    changed.sequencer.tracks.remove(4);
    call(&mut control, "/api/preset", json!(changed));
    no_heap(|| {
        audio.begin(&values, 160.0, Some(3.137), true, 128);
        render(&mut audio, 128, [0.0; 2]);
    });
    call(&mut control, "/api/sequencer/view", json!({"offset":80}));
    no_heap(|| {
        audio.begin(
            &values,
            160.0,
            Some(3.137 + 128.0 * 160.0 / (60.0 * 48000.0)),
            true,
            2048,
        );
        render(&mut audio, 2048, [0.0; 2]);
    });
    control.drain();
    let state = control.state();
    assert_eq!(state["sequencer"]["total"], 128);
    assert_eq!(state["sequencer"]["offset"], 80);
    let saved = control.save().unwrap();
    let mut restored = Controller::new(false);
    restored.restore(&saved).unwrap();
    assert_eq!(restored.preset.sequencer.tracks.len(), 128);
    assert_eq!(restored.preset.sequencer.tracks[0].steps[7].pitch, 79);
    assert_eq!(
        restored.preset.sequencer.tracks[0].steps[7].destination1,
        -0.23
    );
    assert_eq!(
        restored.preset.sequencer.tracks[0].steps[7].destination2,
        0.13
    );
    assert_eq!(
        restored.preset.sequencer.tracks[1].steps[7].destination3,
        0.14
    );
    assert_eq!(
        restored.preset.sequencer.tracks[0].destinations[2].as_deref(),
        Some("fx1_mix")
    );
    let mut other = restored.prepare(48000);
    let restored_values = restored.values();
    no_heap(|| {
        other.begin(&restored_values, 120.0, Some(0.0), true, 128);
        render(&mut other, 128, [0.0; 2]);
        audio.begin(&values, 120.0, Some(4.0), false, 128);
        render(&mut audio, 128, [0.0; 2]);
    });
    assert!(audio.engine.held_notes().is_empty());
    assert!(!audio.engine.sequence_view().tracks[0].gate);
}

#[test]
fn four_resynth_layers_automation_and_project_recall_are_realtime_safe() {
    let mut control = Controller::new(true);
    let mut preset = control.preset.clone();
    preset.resynth.enabled = true;
    preset.resynth.count = 4;
    for (i, o) in preset.resynth.oscillators.iter_mut().enumerate() {
        o.waveform = sad_pet::resynth::Waveform::ALL[i];
        o.mode = sad_pet::resynth::Mode::ALL[i];
        o.control1 = 0.7;
        o.control2 = 0.4;
        if i == 1 {
            o.model = sad_pet::resynth::Model::Wavefolder;
            o.fold_shape = sad_pet::resynth::FoldShape::Sine;
        }
        o.interaction = sad_pet::resynth::Interaction::RingSync;
        o.interaction_source = [1, 4, 0, 2][i];
    }
    call(&mut control, "/api/preset", json!(preset));
    let mut audio = control.prepare(48000);
    let mut values = control.values();
    let spec = parameters::specs()
        .iter()
        .position(|s| s.id == "resynth_osc2_transpose")
        .unwrap();
    let wave = parameters::specs()
        .iter()
        .position(|s| s.id == "resynth_osc4_waveform")
        .unwrap();
    let mode = parameters::specs()
        .iter()
        .position(|s| s.id == "resynth_osc4_mode")
        .unwrap();
    call(
        &mut control,
        "/api/transport",
        json!({"playing":true,"midi_gate":false}),
    );
    for block in 0..8 {
        values[spec] = block as f32;
        if block == 4 {
            values[wave] = 7.0;
            values[mode] = 0.0;
        }
        control.set_values(&values);
        no_heap(|| {
            audio.begin(&values, 120.0, None, false, 128);
            render(&mut audio, 128, [0.3, -0.2]);
        });
    }
    control.drain();
    let saved = control.save().unwrap();
    let mut restored = Controller::new(true);
    restored.restore(&saved).unwrap();
    assert_eq!(restored.preset.resynth, control.preset.resynth);
    assert_eq!(control.preset.resynth.oscillators[1].transpose, 7.0);
    assert_eq!(
        control.state()["resynth"]["bands"]
            .as_array()
            .unwrap()
            .len(),
        16
    );
}

#[test]
fn one_bar_loop_host_controls_and_meter_are_realtime_safe_and_saved() {
    let mut control = Controller::new(false);
    call(
        &mut control,
        "/api/parameters",
        json!({"loop_one_bar":1,"sample_root_note":48}),
    );
    let mut audio = control.prepare(8000);
    let values = control.values();
    let ids = parameters::specs();
    assert_eq!(ids[1475].id, "sample_root_note");
    assert_eq!(ids[1476].id, "loop_one_bar");
    let region = Region {
        start: 0.25,
        end: 0.75,
    };
    audio.engine.set_region(region).unwrap();
    no_heap(|| {
        unsafe { sad_audio_time_signature(&mut audio, 6, 8) };
        audio.begin(&values, 120.0, None, true, 3000);
        audio.engine.midi(Message::NoteOn {
            channel: 0,
            note: 48,
            velocity: 100,
        });
        render(&mut audio, 3000, [0.0; 2]);
    });
    let frame = audio.engine.center() / audio.engine.sample.len() as f64;
    assert!((frame - 0.375).abs() < 1e-6);
    let saved = control.save().unwrap();
    let mut restored = Controller::new(false);
    restored.restore(&saved).unwrap();
    assert!(restored.preset.params.loop_one_bar);
    assert_eq!(restored.preset.params.sample_root_note, 48.0);
}

#[test]
fn source_fx_cache_updates_and_host_automation_never_allocate_on_audio() {
    let mut control = Controller::new(false);
    let mut p = control.preset.clone();
    p.params.loop_one_bar = true;
    p.source_color_chain = vec![sad_pet::color::ColorSlot {
        id: 40,
        mix: 1.0,
        ..Default::default()
    }];
    call(&mut control, "/api/preset", json!(p));
    let mut audio = control.prepare(8000);
    assert!(audio.engine.processed_source.is_some());
    audio.engine.midi(Message::NoteOn {
        channel: 0,
        note: 72,
        velocity: 100,
    });
    let mut values = control.values();
    let index = parameters::specs()
        .iter()
        .position(|p| p.id == "source_fx1_distortion_drive")
        .unwrap();
    values[index] = 0.9;
    no_heap(|| {
        audio.begin(&values, 120.0, None, false, 128);
        render(&mut audio, 128, [0.0; 2]);
    });
    let until = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while audio.engine.source_effects_pending() && std::time::Instant::now() < until {
        std::thread::sleep(std::time::Duration::from_millis(10));
        no_heap(|| {
            audio.begin(&values, 120.0, None, false, 128);
            render(&mut audio, 128, [0.0; 2]);
        });
    }
    assert!(!audio.engine.source_effects_pending());
    assert_eq!(audio.engine.held_notes(), vec![72]);
    assert_eq!(
        audio
            .engine
            .processed_source
            .as_ref()
            .unwrap()
            .slot
            .settings
            .distortion_drive,
        0.9
    );
    control.set_values(&values);
    let saved = control.save().unwrap();
    let mut other = Controller::new(false);
    other.restore(&saved).unwrap();
    let restored = other.prepare(8000);
    assert_eq!(
        restored.engine.preset.source_color_chain[0]
            .settings
            .distortion_drive,
        0.9
    );
    let mut effect = Controller::new(true);
    effect.preset.source_color_chain = p.source_color_chain;
    let mut live = effect.prepare(8000);
    let values = effect.values();
    no_heap(|| {
        live.begin(&values, 120.0, None, false, 128);
        render(&mut live, 128, [0.2; 2]);
    });
}

struct PresetDirectory(std::path::PathBuf);
impl PresetDirectory {
    fn new() -> Self {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let root = std::env::temp_dir().join(format!(
            "sad-pet-test-{}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));
        Self(root)
    }
    fn control(&self, effect: bool) -> Controller {
        let mut control = Controller::new(effect);
        control.library = preset_library::Library::at(self.0.clone());
        control
    }
}
impl Drop for PresetDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn disk_presets_restore_sound_sample_region_and_host_values_in_another_instance() {
    let folder = PresetDirectory::new();
    let mut control = folder.control(false);
    let _audio = control.prepare(48000);
    let frames: Vec<Stereo> = (0..1600)
        .map(|n| [(n as f32 * 0.09).sin() * 0.4; 2])
        .collect();
    call(
        &mut control,
        "/api/wav",
        json!({"base64":BASE64.encode(wav::bytes(8000, &frames).unwrap()),"name":"Own sample.wav","id":"my-sample"}),
    );
    call(&mut control, "/api/region", json!({"start":0.2,"end":0.8}));
    call(
        &mut control,
        "/api/fades",
        json!({"fade_in":0.01,"fade_out":0.02}),
    );
    let mut patch = control.preset.clone();
    patch.pre_color_chain = vec![sad_pet::color::ColorSlot {
        id: 40,
        kind: sad_pet::preset::SaturatorType::Waveshaper,
        ..Default::default()
    }];
    patch.pre_color_chain[0].settings.waveshaper_type =
        sad_pet::dsp::radias_waveshaper::WaveshaperMode::MultiSine;
    call(&mut control, "/api/preset", json!(patch));
    let mut values = control.values();
    let position = parameters::specs()
        .iter()
        .position(|s| s.id == "position")
        .unwrap();
    values[position] = 0.37;
    control.set_values(&values);
    let saved = call(
        &mut control,
        "/api/preset-library/save",
        json!({"name":"Моя атмосфера"}),
    );
    let id = saved["id"].as_str().unwrap();
    let bytes = std::fs::read(folder.0.join(format!("{id}.sadpet"))).unwrap();
    assert_eq!(&bytes[..8], b"SADPET\x01\0");
    assert!(!control.selection.dirty);
    let mut other = folder.control(false);
    let mut audio = other.prepare(48000);
    assert_eq!(
        call(&mut other, "/api/preset-library", Value::Null)
            .as_array()
            .unwrap()
            .len(),
        1
    );
    call(&mut other, "/api/preset-library/load", json!({"id":id}));
    assert_eq!(other.preset.name, "Моя атмосфера");
    assert_eq!(other.sample.frames(), frames);
    assert_eq!(other.region.start, 0.2);
    assert_eq!(other.fades.fade_out, 0.02);
    assert_eq!(other.preset.params.position, 0.37);
    assert_eq!(
        other.preset.pre_color_chain[0].settings.waveshaper_type,
        sad_pet::dsp::radias_waveshaper::WaveshaperMode::MultiSine
    );
    let values = other.values();
    no_heap(|| {
        audio.begin(&values, 120.0, None, false, 128);
        render(&mut audio, 128, [0.0; 2]);
    });
    let state = other.save().unwrap();
    let mut recalled = folder.control(false);
    recalled.restore(&state).unwrap();
    assert_eq!(recalled.selection.id, id);
    assert!(!recalled.selection.dirty);
}

#[test]
fn preset_overwrite_import_export_and_invalid_files_preserve_existing_patches() {
    let folder = PresetDirectory::new();
    let mut control = folder.control(false);
    let _audio = control.prepare(48000);
    let saved = call(
        &mut control,
        "/api/preset-library/save",
        json!({"name":"Texture"}),
    );
    let id = saved["id"].as_str().unwrap();
    let path = folder.0.join(format!("{id}.sadpet"));
    let initial = std::fs::read(&path).unwrap();
    for data in [
        json!({"name":""}),
        json!({"name":"texture"}),
        json!({"name":"X","id":"../../evil"}),
    ] {
        assert!(
            control
                .request(&json!({"path":"/api/preset-library/save","method":"POST","data":data}))
                .is_err()
        );
        assert_eq!(std::fs::read(&path).unwrap(), initial);
    }
    control.preset.params.position = 0.61;
    call(
        &mut control,
        "/api/preset-library/save",
        json!({"id":id,"name":"Texture renamed"}),
    );
    assert_eq!(
        call(&mut control, "/api/preset-library", Value::Null)
            .as_array()
            .unwrap()
            .len(),
        1
    );
    let exported = call(&mut control, "/api/preset-library/export", json!({"id":id}));
    assert!(exported["filename"].as_str().unwrap().ends_with(".sadpet"));
    let bytes = BASE64.decode(exported["binary"].as_str().unwrap()).unwrap();
    let original_name = control.preset.name.clone();
    for bytes in [&b"garbage"[..], &bytes[..bytes.len() - 4]] {
        assert!(control.request(&json!({"path":"/api/preset-library/import","method":"POST","data":{"base64":BASE64.encode(bytes)}})).is_err());
        assert_eq!(control.preset.name, original_name);
    }
    let imported = call(
        &mut control,
        "/api/preset-library/import",
        json!({"base64":BASE64.encode(&bytes)}),
    );
    assert_ne!(imported["id"], saved["id"]);
    assert_eq!(imported["name"], "Texture renamed (2)");
    call(&mut control, "/api/preset-library/delete", json!({"id":id}));
    assert!(!path.exists());
    assert_eq!(
        std::fs::read_dir(folder.0.join("Trash")).unwrap().count(),
        1
    );
}

#[test]
fn legacy_preset_migration_is_idempotent_and_fx_patches_can_load_in_the_instrument() {
    let folder = PresetDirectory::new();
    let mut control = folder.control(true);
    let _audio = control.prepare(48000);
    let legacy = json!({"base64":BASE64.encode(serde_json::to_vec(&control.preset).unwrap()),"import_key":"legacy:preset-1"});
    let first = call(&mut control, "/api/preset-library/import", legacy.clone());
    let again = call(&mut control, "/api/preset-library/import", legacy.clone());
    assert_eq!(first["id"], again["id"]);
    call(
        &mut control,
        "/api/preset-library/save",
        json!({"id":first["id"],"name":"Changed"}),
    );
    assert_eq!(
        call(&mut control, "/api/preset-library/import", legacy)["id"],
        first["id"]
    );
    let mut instrument = folder.control(false);
    let _other = instrument.prepare(44100);
    call(
        &mut instrument,
        "/api/preset-library/load",
        json!({"id":first["id"]}),
    );
    assert!(instrument.source == Source::File);
    assert!(instrument.midi_gate);
    assert!(!instrument.playing);
    let mut old_host_state: Value = serde_json::from_slice(&instrument.save().unwrap()).unwrap();
    old_host_state.as_object_mut().unwrap().remove("selection");
    instrument
        .restore(&serde_json::to_vec(&old_host_state).unwrap())
        .unwrap();
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
fn numeric_editor_updates_preserve_routes_host_values_and_playing_notes_without_heap_work() {
    let mut control = Controller::new(false);
    let mut audio = control.prepare(48000);
    let mut preset = control.preset.clone();
    preset.performance.mode = Mode::Sequencer;
    preset.color_chain = vec![sad_pet::color::ColorSlot::default()];
    preset.pre_color_chain = vec![sad_pet::color::ColorSlot {
        id: 2,
        kind: sad_pet::preset::SaturatorType::Waveshaper,
        ..Default::default()
    }];
    preset.matrix = vec![sad_pet::matrix::Route::default()];
    call(&mut control, "/api/preset", json!(preset));
    let mut values = control.values();
    audio.begin(&values, 120.0, Some(0.0), true, 128);
    render(&mut audio, 128, [0.0; 2]);
    control.drain();
    let matrix = audio.engine.preset.matrix.as_ptr();
    let effects = audio.engine.preset.color_chain.as_ptr();
    let chord = audio.engine.preset.performance.chord.as_ptr();
    let notes = audio.engine.held_notes();
    let volume = parameters::specs()
        .iter()
        .position(|p| p.id == "master_volume")
        .unwrap();
    // A concurrent DAW edit must survive a knob update from an older UI snapshot.
    values[volume] = 0.21;
    control.set_values(&values);
    let route = control.preset.matrix[0];
    for i in 1..=100 {
        let position = i as f32 / 100.0;
        call(
            &mut control,
            "/api/parameters",
            json!({
                "position":position,"env1_attack":0.12,"lfo2_depth":-0.4,
                "fx1_mix":0.3,"pre_fx1_waveshaper_depth":0.7,
            }),
        );
        assert_eq!(
            control.commands.slots(),
            QUEUE,
            "Numeric host controls enqueued a full preset"
        );
        values = control.values();
        no_heap(|| audio.begin(&values, 120.0, Some(128.0 * 2.0 / 48000.0), true, 128));
        assert_eq!(
            audio.engine.held_notes(),
            notes,
            "A knob released playing notes"
        );
        assert_eq!(audio.engine.preset.params.position, position);
        assert_eq!(audio.engine.preset.params.master_volume, 0.21);
        assert_eq!(audio.engine.preset.matrix.as_ptr(), matrix);
        assert_eq!(audio.engine.preset.color_chain.as_ptr(), effects);
        assert_eq!(audio.engine.preset.performance.chord.as_ptr(), chord);
        assert_eq!(audio.engine.preset.matrix[0], route);
        assert_eq!(audio.engine.preset.envs[0].attack, 0.12);
        assert_eq!(
            audio.engine.modulated_pre_color_slots()[0]
                .settings
                .waveshaper_depth,
            0.7
        );
    }
    call(
        &mut control,
        "/api/parameters",
        json!({"matrix1_depth":-0.75}),
    );
    values = control.values();
    no_heap(|| {
        audio.begin(&values, 120.0, Some(128.0 * 2.0 / 48000.0), true, 128);
        render(&mut audio, 128, [0.0; 2]);
    });
    assert_eq!(audio.engine.preset.matrix[0].depth, -0.75);
    assert_eq!(audio.engine.preset.matrix.as_ptr(), matrix);
    assert!(control.selection.dirty);
    let before = control.preset.params.position;
    assert!(
        control
            .request(&json!({"path":"/api/parameters","method":"POST",
        "data":{"position":0.9,"unknown_control":0.4}}))
            .is_err()
    );
    assert_eq!(
        control.preset.params.position, before,
        "Invalid messages applied partial changes"
    );
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

#[test]
fn drum_timbres_mix_independently_save_all_parts_and_edit_without_audio_allocations() {
    let mut control = Controller::new(false);
    let context = call(&mut control, "/api/timbres/add", json!({"kind":"drums"}));
    assert_eq!(context["active"], 1);
    call(
        &mut control,
        "/api/timbres/update",
        json!({"id":0,"muted":true}),
    );
    call(
        &mut control,
        "/api/drums/edit",
        json!([
            {"track":0,"step":0,"field":0,"value":0.9},
            {"track":0,"step":127,"field":0,"value":0.8},
            {"track":0,"step":null,"field":0,"value":128}
        ]),
    );
    call(
        &mut control,
        "/api/parameters",
        json!({"drum1_0":63.0,"drum1_12":6200.0}),
    );
    let mut audio = control.prepare(48000);
    let values = control.values();
    no_heap(|| {
        audio.begin(&values, 120., Some(0.), true, 8192);
        assert!(render(&mut audio, 8192, [0.; 2]) > 0.001);
    });
    let state = call(&mut control, "/api/state", Value::Null);
    assert_eq!(state["timbres"]["timbres"].as_array().unwrap().len(), 2);
    assert_eq!(state["preset"]["drums"]["tracks"][0]["length"], 128);
    assert_eq!(state["preset"]["drums"]["params"][0][0], 63.0);
    call(
        &mut control,
        "/api/drums/edit",
        json!([{ "track":0,"step":2,"field":3,"value":4 }]),
    );
    call(&mut control, "/api/parameters", json!({"drum1_0":81.0}));
    no_heap(|| {
        audio.begin(&values, 120., Some(8192. / 24000.), true, 4096);
        render(&mut audio, 4096, [0.; 2]);
    });
    let saved = control.save().unwrap();
    let mut restored = Controller::new(false);
    restored.restore(&saved).unwrap();
    let state = call(&mut restored, "/api/state", Value::Null);
    assert_eq!(state["timbres"]["active"], 1);
    assert_eq!(
        state["preset"]["drums"]["tracks"][0]["steps"][127]["velocity"],
        json!(0.8_f32)
    );
    assert_eq!(
        state["preset"]["drums"]["tracks"][0]["steps"][2]["ratchet"],
        4
    );
    assert_eq!(state["preset"]["drums"]["params"][0][0], 81.0);
    call(&mut restored, "/api/timbres/select", json!({"id":0}));
    assert!(call(&mut restored, "/api/state", Value::Null)["preset"]["drums"].is_null());
}

#[test]
fn new_and_removed_timbres_are_adopted_and_retired_away_from_audio() {
    let mut control = Controller::new(false);
    control.meta.mix.muted = true;
    let mut audio = control.prepare(8000);
    let values = control.values();
    call(&mut control, "/api/timbres/add", json!({"kind":"drums"}));
    no_heap(|| {
        audio.begin(&values, 120., None, false, 256);
        render(&mut audio, 256, [0.; 2]);
    });
    assert_eq!(audio.parts.as_ref().unwrap().audio.meta.id, 1);
    call(
        &mut control,
        "/api/timbres/update",
        json!({"id":1,"channel":10}),
    );
    no_heap(|| {
        audio.begin(&values, 120., None, false, 4096);
        audio.midi(Message::NoteOn {
            channel: 0,
            note: 36,
            velocity: 127,
        });
        assert_eq!(render(&mut audio, 4096, [0.; 2]), 0.);
        audio.midi(Message::NoteOn {
            channel: 9,
            note: 36,
            velocity: 127,
        });
        assert!(render(&mut audio, 4096, [0.; 2]) > 0.001);
    });
    call(&mut control, "/api/timbres/remove", json!({"id":1}));
    no_heap(|| {
        audio.begin(&values, 120., None, false, 256);
        render(&mut audio, 256, [0.; 2]);
    });
    assert!(audio.parts.is_none());
    control.drain();
}

#[test]
fn factory_recall_replaces_the_scene_and_a_drum_route_toggle_does_not_rebuild_audio() {
    let mut control = Controller::new(false);
    call(&mut control, "/api/timbres/add", json!({"kind":"drums"}));
    let mut audio = control.prepare(8000);
    let values = control.values();
    call(&mut control, "/api/parameters", json!({"drum_granular":1}));
    no_heap(|| {
        audio.begin(&values, 120., None, true, 1024);
        render(&mut audio, 1024, [0.; 2]);
    });
    assert!(
        !audio
            .parts
            .as_ref()
            .unwrap()
            .audio
            .engine
            .live_input
            .as_ref()
            .unwrap()
            .effects_only
    );
    call(
        &mut control,
        "/api/preset-library/factory",
        json!({"index":0}),
    );
    let recalled = control.values();
    no_heap(|| {
        audio.begin(&recalled, 120., None, false, 1024);
    });
    // Parameter collection itself belongs to Controller; never do that on audio.
    assert!(audio.parts.is_none());
    assert_eq!(
        call(&mut control, "/api/state", Value::Null)["timbres"]["active"],
        0
    );
}

#[test]
fn drum_audition_rings_out_while_the_daw_transport_is_stopped() {
    let mut control = Controller::new(false);
    control.meta.mix.muted = true;
    call(&mut control, "/api/timbres/add", json!({"kind":"drums"}));
    let mut audio = control.prepare(8000);
    let values = control.values();
    call(
        &mut control,
        "/api/drums/trigger",
        json!({"track":0,"velocity":1}),
    );
    no_heap(|| {
        audio.begin(&values, 120., Some(0.), false, 128);
        assert!(render(&mut audio, 128, [0.; 2]) > 0.001);
        audio.begin(&values, 120., Some(0.), false, 2048);
        assert!(render(&mut audio, 2048, [0.; 2]) > 0.001);
    });
    assert!(
        !audio
            .parts
            .as_ref()
            .unwrap()
            .audio
            .engine
            .drum_view()
            .playing
    );
}
