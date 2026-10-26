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

#[test]
fn sample_preview_plays_without_changing_scene_and_stops_without_heap_work() {
    for rate in [44100, 48000] {
        let mut control = Controller::new(false);
        let mut audio = control.prepare(rate);
        let values = control.values();
        let saved = control.save().unwrap();
        let source = control.sample.clone();
        call(
            &mut control,
            "/api/sample-preview",
            json!({"id":"factory:glass-sequence:v1","generation":1}),
        );
        let mut peak = 0.0_f32;
        no_heap(|| {
            audio.begin(&values, 0.0, None, false, 4096);
            for _ in 0..4096 {
                for sample in audio.process_frame([0.0; 2]) {
                    peak = peak.max(sample.abs());
                }
            }
        });
        assert!(peak > 0.01);
        assert!(Arc::ptr_eq(&source, &control.sample));
        assert_eq!(saved, control.save().unwrap());
        call(&mut control, "/api/sample-preview", json!({"generation":3}));
        // A load that arrives after Stop with an older generation stays stopped.
        call(
            &mut control,
            "/api/sample-preview",
            json!({"id":"factory:dusty-keys:v1","generation":2}),
        );
        no_heap(|| {
            audio.begin(&values, 0.0, None, false, 2048);
            for i in 0..2048 {
                let frame = audio.process_frame([0.0; 2]);
                if i > 1024 {
                    assert_eq!(frame, [0.0; 2]);
                }
            }
        });
        assert_eq!(saved, control.save().unwrap());
    }
}

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
fn preset_edits_while_the_daw_plays_keep_neuropolis_on_the_host_grid() {
    use sad_pet::sequence::{Target, Track};
    fn play(audio: &mut Audio, values: &[f32], beat: &mut f64, blocks: usize) {
        for _ in 0..blocks {
            audio.begin(values, 120.0, Some(*beat), true, 128);
            render(audio, 128, [0.0; 2]);
            *beat += 128.0 * 120.0 / (60.0 * 48000.0);
        }
    }
    let mut control = Controller::new(false);
    let mut preset = control.preset.clone();
    preset.sequencer.enabled = true;
    preset.sequencer.tracks = (1..=2)
        .map(|id| Track {
            id,
            target: Target::Notes,
            ..Track::default()
        })
        .collect();
    call(&mut control, "/api/preset", json!(preset));
    let mut audio = control.prepare(48000);
    let values = control.values();
    let mut beat = 0.0;
    // 3.2 beats: the 13th sixteenth.
    play(&mut audio, &values, &mut beat, 600);
    assert_eq!(audio.engine.sequence_view().tracks[0].step, 12);
    // A rename and a new track rebuild the preset. Neither restarts the
    // pattern at the current beat; the new track joins the others.
    let mut changed = control.preset.clone();
    changed.sequencer.tracks[1].name = "Bass".into();
    let mut added = changed.sequencer.tracks[0].clone();
    added.id = 3;
    changed.sequencer.tracks.push(added);
    call(&mut control, "/api/preset", json!(changed));
    no_heap(|| play(&mut audio, &values, &mut beat, 20));
    let view = audio.engine.sequence_view();
    assert_eq!(view.count, 3);
    for t in &view.tracks[..3] {
        assert_eq!(t.step, 13, "track {} left the host grid", t.id);
    }
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
    // The recalled scene keeps only its Master besides the first timbre.
    let parts = audio.parts.as_ref().unwrap();
    assert_eq!(parts.audio.meta.kind, sad_pet::timbres::Kind::Master);
    assert!(parts.next.is_none());
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
    let state = call(&mut control, "/api/state", Value::Null);
    assert_eq!(state["sample_name"], "Drum machine");
    assert_eq!(state["sample_seconds"], 8.0);
    assert_eq!(state["sample_frames"], 64000);
    assert!(state["sample_id"].is_null());
    assert!(state["waveform"].as_array().unwrap().iter().any(|bin| {
        bin.as_array()
            .unwrap()
            .iter()
            .any(|value| value.as_f64().unwrap().abs() > 0.001)
    }));
    assert_ne!(state["waveform"], state["file_source"]["waveform"]);
}

#[test]
fn native_drum_buffer_choices_use_reserved_history_without_audio_heap_work() {
    let mut control = Controller::new(false);
    control.meta.mix.muted = true;
    call(&mut control, "/api/timbres/add", json!({"kind":"drums"}));
    let mut audio = control.prepare(8000);
    call(
        &mut control,
        "/api/drums/trigger",
        json!({"track":0,"velocity":1}),
    );
    for seconds in [0.125, 32., 0.5, 16., 1.] {
        call(
            &mut control,
            "/api/parameters",
            json!({"drum_buffer_seconds":seconds,"drum_granular":1}),
        );
        let values = control.values();
        no_heap(|| {
            audio.begin(&values, 120., Some(0.), false, 2048);
            render(&mut audio, 2048, [0.; 2]);
        });
        let state = call(&mut control, "/api/state", Value::Null);
        assert_eq!(state["sample_seconds"], seconds);
        assert_eq!(state["sample_frames"], (seconds * 8000.) as usize);
        let engine = &audio.parts.as_ref().unwrap().audio.engine;
        assert_eq!(
            engine.live_input.as_ref().unwrap().window_frames(),
            (seconds * 8000.) as usize
        );
        assert_eq!(engine.source_sample().len(), 32 * 8000);
    }
}

#[test]
fn synth_timbres_play_notes_switch_grains_and_save_without_audio_allocations() {
    let mut control = Controller::new(false);
    let context = call(&mut control, "/api/timbres/add", json!({"kind":"synth"}));
    assert_eq!(context["active"], 1);
    assert_eq!(context["timbres"][1]["kind"], "synth");
    assert_eq!(context["timbres"][1]["name"], "Synth 1");
    call(
        &mut control,
        "/api/timbres/update",
        json!({"id":0,"muted":true}),
    );
    call(
        &mut control,
        "/api/parameters",
        json!({"synth_count":2,"synth_osc2_waveform":2,"synth_osc2_transpose":7}),
    );
    let mut audio = control.prepare(48000);
    let values = control.values();
    no_heap(|| {
        audio.begin(&values, 120., None, false, 4096);
        assert!(render(&mut audio, 4096, [0.; 2]) < 1e-5);
        audio.midi(Message::NoteOn {
            channel: 0,
            note: 60,
            velocity: 100,
        });
        audio.begin(&values, 120., None, false, 8192);
        assert!(render(&mut audio, 8192, [0.; 2]) > 0.01);
    });
    call(
        &mut control,
        "/api/parameters",
        json!({"synth_granular":1,"synth_buffer_seconds":2,"grains":0.9,"position":0}),
    );
    no_heap(|| {
        audio.begin(&values, 120., None, false, 48000);
        assert!(render(&mut audio, 48000, [0.; 2]) > 0.01);
    });
    let state = call(&mut control, "/api/state", Value::Null);
    assert_eq!(state["sample_name"], "Synth engine");
    assert_eq!(state["sample_seconds"], 2.0);
    assert_eq!(state["preset"]["synth"]["granular"], true);
    assert_eq!(
        state["preset"]["synth"]["oscillators"][1]["waveform"],
        "triangle"
    );
    let saved = control.save().unwrap();
    let mut restored = Controller::new(false);
    restored.restore(&saved).unwrap();
    let state = call(&mut restored, "/api/state", Value::Null);
    assert_eq!(state["timbres"]["active"], 1);
    assert_eq!(state["timbres"]["timbres"][1]["kind"], "synth");
    assert_eq!(state["preset"]["synth"]["count"], 2);
    assert_eq!(state["preset"]["synth"]["oscillators"][1]["transpose"], 7.0);
    call(&mut restored, "/api/timbres/select", json!({"id":0}));
    assert!(call(&mut restored, "/api/state", Value::Null)["preset"]["synth"].is_null());
}

#[test]
fn rack_timbres_mix_drums_and_a_synth_and_rebuild_without_audio_allocations() {
    let mut control = Controller::new(false);
    let context = call(&mut control, "/api/timbres/add", json!({"kind":"rack"}));
    assert_eq!(context["timbres"][1]["kind"], "rack");
    assert_eq!(context["timbres"][1]["name"], "Rack 1");
    call(
        &mut control,
        "/api/timbres/update",
        json!({"id":0,"muted":true}),
    );
    // Without FX B, no reverb tail outlasts a muted strip.
    let mut preset = call(&mut control, "/api/preset", Value::Null);
    assert_eq!(preset["rack"]["modules"][4]["module"], "post_fx");
    preset["rack"]["modules"]
        .as_array_mut()
        .unwrap()
        .truncate(4);
    call(&mut control, "/api/preset", preset);
    call(
        &mut control,
        "/api/drums/edit",
        json!([0, 4, 8, 12].map(|step| json!({"track":0,"step":step,"field":0,"value":1}))),
    );
    let mut audio = control.prepare(48000);
    let values = control.values();
    // The drums play their pattern with the transport; no note is needed.
    no_heap(|| {
        audio.begin(&values, 120., Some(0.), true, 24000);
        assert!(render(&mut audio, 24000, [0.; 2]) > 0.01);
    });
    // Muting their strip leaves the keys to the synth.
    call(
        &mut control,
        "/api/parameters",
        json!({"rack_drums_mute":1}),
    );
    no_heap(|| {
        // The strip fades out rather than click, and the kick's tail with it.
        audio.begin(&values, 120., Some(1.), true, 12000);
        render(&mut audio, 12000, [0.; 2]);
        audio.begin(&values, 120., Some(1.5), true, 12000);
        assert!(render(&mut audio, 12000, [0.; 2]) < 1e-4);
        audio.midi(Message::NoteOn {
            channel: 0,
            note: 60,
            velocity: 100,
        });
        audio.begin(&values, 120., Some(2.), true, 12000);
        assert!(render(&mut audio, 12000, [0.; 2]) > 0.01);
    });
    // A Grain engine under the Mixer comes prepared with its history.
    let mut preset = call(&mut control, "/api/preset", Value::Null);
    preset["rack"]["modules"] = json!(
        ["drums", "synth", "filters", "mixer", "grains", "post_fx"]
            .map(|module| json!({"module":module,"enabled":true}))
    );
    call(&mut control, "/api/preset", preset);
    no_heap(|| {
        audio.begin(&values, 120., Some(2.5), true, 24000);
        assert!(render(&mut audio, 24000, [0.; 2]) > 0.01);
    });
    let state = call(&mut control, "/api/state", Value::Null);
    assert_eq!(state["sample_name"], "Rack");
    assert_eq!(state["preset"]["rack"]["modules"][4]["module"], "grains");
    assert_eq!(state["preset"]["rack"]["strips"]["drums"]["muted"], true);
    assert_eq!(state["rack_peaks"].as_array().unwrap().len(), 3);
    let saved = control.save().unwrap();
    let mut restored = Controller::new(false);
    restored.restore(&saved).unwrap();
    let state = call(&mut restored, "/api/state", Value::Null);
    assert_eq!(state["timbres"]["timbres"][1]["kind"], "rack");
    assert_eq!(
        state["preset"]["rack"]["modules"].as_array().unwrap().len(),
        6
    );
    assert!(state["preset"]["synth"].is_object() && state["preset"]["drums"].is_object());
}

#[test]
fn timbres_reach_master_directly_or_through_a_bus_without_audio_allocation() {
    let mut control = Controller::new_plugin(false);
    let timbres = call(&mut control, "/api/timbres", Value::Null)["timbres"].clone();
    let master = timbres
        .as_array()
        .unwrap()
        .iter()
        .find(|t| t["kind"] == "master")
        .expect("every scene has a Master")["id"]
        .as_u64()
        .unwrap();
    let error = control
        .request(&json!({"path":"/api/timbres/remove","method":"POST","data":{"id":master}}))
        .unwrap_err();
    assert!(error.contains("Master"), "{error}");
    let context = call(&mut control, "/api/timbres/add", json!({"kind":"bus"}));
    let bus = context["timbres"]
        .as_array()
        .unwrap()
        .iter()
        .find(|t| t["kind"] == "bus")
        .unwrap()
        .clone();
    assert_eq!(bus["bus"], 1);
    // The first timbre no longer feeds Master directly, only through Bus 1.
    let mut sends = [0.0; 8];
    sends[0] = 1.0;
    call(
        &mut control,
        "/api/timbres/update",
        json!({"id":0,"gain":0,"sends":sends}),
    );
    call(
        &mut control,
        "/api/transport",
        json!({"playing":true,"midi_gate":false}),
    );
    let mut audio = control.prepare(8000);
    let values = control.values();
    let mut peak = 0.0_f32;
    no_heap(|| {
        for _ in 0..8 {
            audio.begin(&values, 120., None, true, 1024);
            peak = peak.max(render(&mut audio, 1024, [0.; 2]));
        }
    });
    assert!(peak > 0.001, "the bus passes the timbre to Master ({peak})");
    // Without the send nothing reaches the output.
    let silent = [0.0_f32; 8];
    call(
        &mut control,
        "/api/timbres/update",
        json!({"id":0,"sends":silent}),
    );
    let mut tail = 0.0_f32;
    no_heap(|| {
        for block in 0..8 {
            audio.begin(&values, 120., None, true, 1024);
            let peak = render(&mut audio, 1024, [0.; 2]);
            if block >= 6 {
                tail = tail.max(peak);
            }
        }
    });
    assert!(tail < 1e-4, "a muted route still sounds ({tail})");
    // Removing the bus clears every send to its number.
    call(
        &mut control,
        "/api/timbres/update",
        json!({"id":0,"sends":sends}),
    );
    let context = call(&mut control, "/api/timbres/remove", json!({"id":bus["id"]}));
    assert_eq!(context["timbres"][0]["sends"][0], 0.0);
}

#[test]
fn a_bus_feeds_a_bus_listed_before_it_and_refuses_a_loop() {
    let mut control = Controller::new_plugin(false);
    let ids: Vec<Value> = (0..3)
        .map(|_| {
            let context = call(&mut control, "/api/timbres/add", json!({"kind":"bus"}));
            context["timbres"]
                .as_array()
                .unwrap()
                .iter()
                .rfind(|t| t["kind"] == "bus")
                .unwrap()["id"]
                .clone()
        })
        .collect();
    call(&mut control, "/api/timbres/select", json!({"id":0}));
    call(
        &mut control,
        "/api/transport",
        json!({"playing":true,"midi_gate":false}),
    );
    let send = |to: usize| {
        let mut sends = [0.0; 8];
        sends[to - 1] = 1.0;
        sends
    };
    // A fresh engine per case renders the same timbre from its start.
    let level = |control: &mut Controller| {
        let mut audio = control.prepare(8000);
        let values = control.values();
        let mut peak = 0.0_f32;
        no_heap(|| {
            for block in 0..8 {
                audio.begin(&values, 120., None, true, 1024);
                let block_peak = render(&mut audio, 1024, [0.; 2]);
                if block >= 4 {
                    peak = peak.max(block_peak);
                }
            }
        });
        peak
    };
    let direct = level(&mut control);
    assert!(direct > 0.01, "the timbre plays ({direct})");
    // The timbre reaches Master only through Bus 2, then Bus 1, which is
    // listed first: Bus 1 must render after Bus 2.
    call(
        &mut control,
        "/api/timbres/update",
        json!({"id":0,"gain":0,"sends":send(2)}),
    );
    call(
        &mut control,
        "/api/timbres/update",
        json!({"id":ids[1],"gain":0,"sends":send(1)}),
    );
    let context = call(&mut control, "/api/timbres", Value::Null);
    assert_eq!(context["timbres"][2]["feeds"], 0b1, "Bus 2 feeds Bus 1");
    let through = level(&mut control);
    assert!(
        (through - direct).abs() < direct * 0.01,
        "Bus 2 passes the timbre to Bus 1 ({through} vs {direct})"
    );
    // Bus 1 cannot send back to Bus 2, nor through Bus 3.
    let context = call(
        &mut control,
        "/api/timbres/update",
        json!({"id":ids[0],"sends":send(2)}),
    );
    assert_eq!(
        context["timbres"][1]["sends"][1], 0.0,
        "a loop was accepted"
    );
    call(
        &mut control,
        "/api/timbres/update",
        json!({"id":ids[2],"sends":send(2)}),
    );
    let context = call(
        &mut control,
        "/api/timbres/update",
        json!({"id":ids[0],"sends":send(3)}),
    );
    assert_eq!(
        context["timbres"][1]["sends"][2], 0.0,
        "a loop through Bus 3 was accepted"
    );
    // A loop made anyway by a section of Bus 1 never feeds back.
    call(&mut control, "/api/timbres/select", json!({"id":ids[0]}));
    let mut preset = call(&mut control, "/api/preset", Value::Null);
    preset["taps"]["post_fx"] = json!({"chain":1.0,"send":1.0,"bus":2});
    call(&mut control, "/api/preset", preset);
    let looped = level(&mut control);
    assert!(
        looped.is_finite() && looped < direct * 2.5,
        "the loop fed back ({looped} vs {direct})"
    );
    // Scenes keep sends between buses.
    let saved = control.save().unwrap();
    let mut restored = Controller::new_plugin(false);
    restored.restore(&saved).unwrap();
    let context = call(&mut restored, "/api/timbres", Value::Null);
    assert_eq!(context["timbres"][2]["sends"][0], 1.0);
}

#[test]
fn a_timbre_entering_a_bus_after_its_filters_skips_them() {
    let mut control = Controller::new_plugin(false);
    let context = call(&mut control, "/api/timbres/add", json!({"kind":"bus"}));
    let bus = context["timbres"]
        .as_array()
        .unwrap()
        .iter()
        .find(|t| t["kind"] == "bus")
        .unwrap()["id"]
        .clone();
    // Bus 1 runs its filters, nearly closed.
    call(&mut control, "/api/timbres/select", json!({"id":bus}));
    let mut preset = call(&mut control, "/api/preset", Value::Null);
    preset["bus"]["filters"] = json!(true);
    preset["params"]["filter"] = json!(0.05);
    call(&mut control, "/api/preset", preset);
    call(&mut control, "/api/timbres/select", json!({"id":0}));
    call(
        &mut control,
        "/api/transport",
        json!({"playing":true,"midi_gate":false}),
    );
    // A fresh engine per case renders the same timbre from its start.
    let output = |control: &mut Controller, mix: Value| {
        call(control, "/api/timbres/update", mix);
        let mut audio = control.prepare(8000);
        let values = control.values();
        let mut output = vec![[0.0_f32; 2]; 8 * 1024];
        no_heap(|| {
            for block in output.chunks_mut(1024) {
                audio.begin(&values, 120., None, true, 1024);
                audio.process_block(&[[0.0; 2]; 1024], block);
                audio.end(1024);
            }
        });
        output
    };
    let direct = output(&mut control, json!({"id":0,"gain":1}));
    let mut sends = [0.0; 8];
    sends[0] = 1.0;
    let mut entries = [[1.0_f32, 0.0, 0.0, 0.0]; 9];
    let filtered = output(
        &mut control,
        json!({"id":0,"gain":0,"sends":sends,"entries":entries}),
    );
    entries[1] = [0.0, 0.0, 0.0, 1.0];
    let post = output(&mut control, json!({"id":0,"entries":entries}));
    // Past the entry and send ramps, compare the second half.
    let difference = |a: &[Stereo], b: &[Stereo]| {
        a[4096..]
            .iter()
            .zip(&b[4096..])
            .flat_map(|(a, b)| [(a[0] - b[0]).abs(), (a[1] - b[1]).abs()])
            .fold(0.0_f32, f32::max)
    };
    let level = direct[4096..]
        .iter()
        .fold(0.0_f32, |p, s| p.max(s[0].abs()));
    assert!(level > 0.01, "the timbre plays ({level})");
    let skipped = difference(&post, &direct);
    assert!(
        skipped < level * 1e-3,
        "Post-filter FX entry skips the filters ({skipped})"
    );
    let changed = difference(&filtered, &direct);
    assert!(
        changed > level * 0.1,
        "the input runs the filters ({changed})"
    );
    // Scenes keep the entries; removing the bus resets them.
    let saved = control.save().unwrap();
    let mut restored = Controller::new_plugin(false);
    restored.restore(&saved).unwrap();
    let context = call(&mut restored, "/api/timbres", Value::Null);
    assert_eq!(
        context["timbres"][0]["entries"][1],
        json!([0.0, 0.0, 0.0, 1.0])
    );
    let context = call(&mut control, "/api/timbres/remove", json!({"id":bus}));
    assert!(
        context["timbres"][0]["entries"].is_null(),
        "{}",
        context["timbres"][0]
    );
}

#[test]
fn sections_send_to_a_bus_and_leave_the_chain_as_their_chain_share_says() {
    let mut control = Controller::new_plugin(false);
    let context = call(&mut control, "/api/timbres/add", json!({"kind":"bus"}));
    let bus = context["timbres"]
        .as_array()
        .unwrap()
        .iter()
        .find(|t| t["kind"] == "bus")
        .unwrap()["id"]
        .clone();
    call(&mut control, "/api/timbres/select", json!({"id":0}));
    // Fully wet, one voice, no effect tails, and the filters left out of
    // the chain: the timbre plays what Resynth hands on.
    let mut preset = control.preset.clone();
    preset.sequencer.enabled = false;
    preset.color_chain.clear();
    preset.pre_color_chain.clear();
    preset.resynth.enabled = true;
    preset.resynth.mix = 1.0;
    preset.taps.resynth.bus = 1;
    preset.taps.filters.chain = 0.0;
    call(&mut control, "/api/preset", json!(preset));
    call(
        &mut control,
        "/api/transport",
        json!({"playing":true,"midi_gate":false}),
    );
    // A fresh engine per case, so no tail carries over between them.
    let level = |control: &mut Controller, changes: Value| {
        call(control, "/api/parameters", changes);
        let mut audio = control.prepare(8000);
        let values = control.values();
        let mut peak = 0.0_f32;
        no_heap(|| {
            for block in 0..16 {
                audio.begin(&values, 120., None, true, 1024);
                let block_peak = render(&mut audio, 1024, [0.; 2]);
                if block >= 8 {
                    peak = peak.max(block_peak);
                }
            }
        });
        peak
    };
    let kept = level(&mut control, json!({"resynth_send":0}));
    assert!(kept > 0.01, "Resynth plays in the timbre ({kept})");
    let dry = level(&mut control, json!({"resynth_enabled":0}));
    assert!(
        (dry - kept).abs() > kept * 0.05,
        "Resynth changes the timbre ({dry} vs {kept})"
    );
    call(
        &mut control,
        "/api/timbres/update",
        json!({"id":0,"gain":0}),
    );
    let through_bus = level(&mut control, json!({"resynth_enabled":1,"resynth_send":1}));
    assert!(
        (through_bus - kept).abs() < kept * 0.01,
        "the bus passes the Resynth output to Master ({through_bus} vs {kept})"
    );
    call(
        &mut control,
        "/api/timbres/update",
        json!({"id":0,"muted":true}),
    );
    let muted = level(&mut control, json!({}));
    assert!(muted < kept * 0.001, "Mute silences the sends ({muted})");
    call(
        &mut control,
        "/api/timbres/update",
        json!({"id":0,"gain":1,"muted":false}),
    );
    call(
        &mut control,
        "/api/timbres/update",
        json!({"id":bus,"muted":true}),
    );
    // Out of the chain, Resynth only feeds the bus: the timbre plays dry.
    let left = level(&mut control, json!({"tap_resynth_chain":0}));
    assert!(
        (left - dry).abs() < dry * 0.01,
        "the timbre plays without Resynth ({left} vs {dry})"
    );
    // Without its bus, the send goes nowhere and the chain is unchanged.
    call(&mut control, "/api/timbres/remove", json!({"id":bus}));
    let missing = level(&mut control, json!({"tap_resynth_chain":1}));
    assert!(
        (missing - kept).abs() < kept * 0.01,
        "a missing bus leaves the timbre as it was ({missing} vs {kept})"
    );
}

#[test]
fn projects_with_master_and_buses_survive_repeated_save_and_restore() {
    let mut control = Controller::new_plugin(false);
    call(&mut control, "/api/timbres/add", json!({"kind":"bus"}));
    call(&mut control, "/api/timbres/add", json!({"kind":"drums"}));
    for round in 0..3 {
        let saved = control.save().unwrap();
        let mut restored = Controller::new_plugin(false);
        restored
            .restore(&saved)
            .unwrap_or_else(|e| panic!("round {round}: {e}"));
        let kinds: Vec<String> = call(&mut restored, "/api/timbres", Value::Null)["timbres"]
            .as_array()
            .unwrap()
            .iter()
            .map(|t| t["kind"].as_str().unwrap().to_owned())
            .collect();
        assert_eq!(
            kinds,
            ["granular", "bus", "drums", "master"],
            "round {round}"
        );
        // Parts never nest their own timbres, and only the scene has a Master.
        assert!(
            restored.parts.iter().all(|p| p.parts.is_empty()),
            "round {round}"
        );
        control = restored;
    }
}

#[test]
fn user_preset_recall_survives_daw_state_with_a_master() {
    // The native host test's path: a factory instance with its Master saves a
    // user preset, recalls it, and a new instance restores the DAW state.
    let folder = PresetDirectory::new();
    let mut control = Controller::new_plugin(false);
    control.library = preset_library::Library::at(folder.0.clone());
    let _audio = control.prepare(48000);
    let saved = call(
        &mut control,
        "/api/preset-library/save",
        json!({"name":"Native patch"}),
    );
    let id = saved["id"].as_str().unwrap().to_owned();
    call(&mut control, "/api/preset-library/load", json!({"id":id}));
    let state = control.save().unwrap();
    let mut restored = Controller::new_plugin(false);
    restored.library = preset_library::Library::at(folder.0.clone());
    restored.restore(&state).unwrap();
    let state = call(&mut restored, "/api/state", Value::Null);
    assert_eq!(state["preset_selection"]["id"], id);
    let kinds: Vec<&str> = state["timbres"]["timbres"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t["kind"].as_str().unwrap())
        .collect();
    assert_eq!(kinds, ["granular", "master"]);
}

impl PresetDirectory {
    /// The standalone scene with its preset and audio libraries in this folder.
    fn standalone(&self) -> Controller {
        let mut control = Controller::new_scene(false, true);
        control.library = preset_library::Library::at(self.0.join("Presets"));
        control.files = audio_library::Files::at(self.0.join("Library"));
        control
    }
}

/// No response the standalone editor receives may hold sample data.
fn without_audio(value: Value) -> Value {
    let text = value.to_string();
    for key in ["\"binary\"", "\"base64\"", "\"sample_wav\""] {
        assert!(!text.contains(key), "{key} reached the editor: {text:.200}");
    }
    value
}

fn sine_wav(path: &std::path::Path, frames: usize) {
    let frames: Vec<Stereo> = (0..frames)
        .map(|n| [(n as f32 * 0.05).sin() * 0.5; 2])
        .collect();
    std::fs::write(path, sad_pet::wav::bytes(48000, &frames).unwrap()).unwrap();
}

#[test]
fn standalone_editor_can_neither_send_nor_receive_sample_data() {
    let folder = PresetDirectory::new();
    let mut control = folder.standalone();
    let _audio = control.prepare(48000);
    let wav = BASE64.encode(sad_pet::wav::bytes(48000, &[[0.1; 2]; 64]).unwrap());
    for (path, data) in [
        ("/api/sample.wav", Value::Null),
        ("/api/samples/glass-sequence.wav", Value::Null),
        ("/api/scene", Value::Null),
        ("/api/wav", json!({"base64":wav,"name":"x.wav"})),
        ("/api/trim", json!({"start":0.0,"end":1.0,"fades":{}})),
        ("/api/render", json!({"seconds":1})),
        ("/api/preset-library/import", json!({"base64":wav})),
        ("/api/preset-library/export", json!({"id":"0".repeat(32)})),
        ("/api/effect-presets/export", json!({"id":"0".repeat(32)})),
    ] {
        let method = if data.is_null() { "GET" } else { "POST" };
        let result = control.request(&json!({"path":path,"method":method,"data":data}));
        assert!(
            result.as_ref().is_err_and(|e| e.contains("natively")),
            "{path} was not refused"
        );
    }
    let state = without_audio(call(&mut control, "/api/state", Value::Null));
    assert_eq!(state["plugin"]["standalone"], true);
    assert_eq!(state["input"]["available"], true);
    without_audio(call(&mut control, "/api/samples", Value::Null));
    without_audio(call(&mut control, "/api/library", Value::Null));
    // The plug-ins keep their existing editor protocol.
    let mut plugin = folder.control(false);
    let _plugin_audio = plugin.prepare(48000);
    assert!(call(&mut plugin, "/api/sample.wav", Value::Null)["binary"].is_string());
}

#[test]
fn standalone_keeps_its_own_transport_even_for_daw_scenes() {
    let folder = PresetDirectory::new();
    let mut control = folder.standalone();
    let mut audio = control.prepare(48000);
    call(&mut control, "/api/transport", json!({"sync":true}));
    assert_eq!(
        call(&mut control, "/api/state", Value::Null)["plugin"]["sync"],
        false
    );
    let mut daw = folder.control(false);
    daw.sync = true;
    let saved = daw.save().unwrap();
    control.restore(&saved).unwrap();
    assert!(!control.sync);
    // Without a DAW the sequencer runs once Play is pressed.
    call(
        &mut control,
        "/api/transport",
        json!({"playing":true,"midi_gate":false}),
    );
    let values = control.values();
    audio.begin(&values, 0.0, None, false, 4800);
    let peak = render(&mut audio, 4800, [0.0; 2]);
    assert!(
        peak > 0.0,
        "the standalone scene stayed silent while playing"
    );
}

/// What the editor's state merge does with a delta answer.
fn merge_state(previous: &Value, delta: &Value) -> Value {
    let mut merged = delta.clone();
    for key in ["preset", "waveform", "file_source"] {
        if merged.get(key).is_none() {
            merged[key] = previous[key].clone();
        }
    }
    let mut knobs = previous["knobs"].as_object().unwrap().clone();
    for (key, value) in merged["knob_changes"].as_object().unwrap() {
        if value.is_null() {
            knobs.remove(key);
        } else {
            knobs.insert(key.clone(), value.clone());
        }
    }
    merged["knobs"] = Value::Object(knobs);
    merged.as_object_mut().unwrap().remove("knob_changes");
    merged
}

#[test]
fn state_polls_since_the_last_answer_leave_out_unchanged_parts() {
    let mut control = Controller::new_scene(false, true);
    let mut audio = control.prepare(48000);
    call(
        &mut control,
        "/api/transport",
        json!({"playing":true,"midi_gate":false}),
    );
    render(&mut audio, 4800, [0.0; 2]);
    let full = call(&mut control, "/api/state", Value::Null);
    assert_eq!(full["delta"], false);
    assert!(full["preset"].is_object() && full["knobs"].is_object());
    assert!(full["waveform"].as_array().is_some_and(|w| !w.is_empty()));
    let token = full["state_token"].as_u64().unwrap();

    render(&mut audio, 4800, [0.0; 2]);
    let delta = call(
        &mut control,
        &format!("/api/state?since={token}"),
        Value::Null,
    );
    assert_eq!(delta["delta"], true);
    for key in ["preset", "waveform", "file_source", "knobs"] {
        assert!(
            delta.get(key).is_none(),
            "an unchanged {key} was sent again"
        );
    }
    assert!(delta["knob_changes"].is_object());
    assert!(delta["input"].is_object() && delta["timbres"].is_object());
    let merged = merge_state(&full, &delta);
    let next = delta["state_token"].as_u64().unwrap();
    assert_ne!(next, token);
    // A complete answer to the same moment matches the merged one.
    let complete = call(&mut control, "/api/state", Value::Null);
    for key in [
        "preset",
        "waveform",
        "file_source",
        "knobs",
        "grains",
        "center",
    ] {
        assert_eq!(merged[key], complete[key], "{key} differs after merging");
    }

    // A stale token gets a complete answer; changes are sent again.
    let stale = call(
        &mut control,
        &format!("/api/state?since={next}"),
        Value::Null,
    );
    assert_eq!(stale["delta"], false);
    assert!(stale["preset"].is_object() && stale["knobs"].is_object());
    let token = stale["state_token"].as_u64().unwrap();
    call(&mut control, "/api/parameters", json!({"loop_one_bar":0}));
    let edited = call(
        &mut control,
        &format!("/api/state?since={token}"),
        Value::Null,
    );
    assert_eq!(edited["delta"], true);
    assert_eq!(edited["preset"]["params"]["loop_one_bar"], false);
    assert!(edited.get("waveform").is_none());
}

#[test]
fn host_state_is_a_zlib_stream_of_the_saved_scene() {
    use std::io::Read as _;
    let mut control = Controller::new_scene(false, true);
    let _audio = control.prepare(48000);
    let bytes = unsafe { sad_control_save(&mut control) };
    let stream = unsafe { std::slice::from_raw_parts(bytes.data, bytes.len) }.to_vec();
    unsafe { sad_bytes_free(bytes.data, bytes.len) };
    let mut state = Vec::new();
    flate2::read::ZlibDecoder::new(stream.as_slice())
        .read_to_end(&mut state)
        .unwrap();
    assert!(stream.len() < state.len());
    assert_eq!(state, control.save().unwrap());
    let mut recalled = Controller::new_scene(false, true);
    let _recalled_audio = recalled.prepare(48000);
    assert!(unsafe { sad_control_restore(&mut recalled, state.as_ptr(), state.len()) });
    assert_eq!(recalled.preset.name, control.preset.name);
}

#[test]
fn standalone_library_imports_loads_trims_exports_and_trashes_files_on_disk() {
    let folder = PresetDirectory::new();
    std::fs::create_dir_all(&folder.0).unwrap();
    let source = folder.0.join("Harp take.wav");
    sine_wav(&source, 48000);
    let mut control = folder.standalone();
    let _audio = control.prepare(48000);
    let path = source.to_str().unwrap();
    let imported = without_audio(call(
        &mut control,
        "/api/library/import",
        json!({"paths":[path, folder.0.join("missing.wav").to_str().unwrap()]}),
    ));
    let entry = &imported["files"][0];
    assert_eq!(entry["displayName"], "Harp take");
    assert_eq!(entry["seconds"], 1.0);
    assert_eq!(imported["errors"].as_array().unwrap().len(), 1);
    let id = entry["id"].as_str().unwrap().to_string();
    // The same audio imports once.
    let again = call(&mut control, "/api/library/import", json!({"paths":[path]}));
    assert_eq!(again["files"][0]["id"], id.as_str());

    let fades = json!({"fade_in":0.05,"fade_out":0.1});
    call(
        &mut control,
        "/api/library/fades",
        json!({"id":id,"fades":fades}),
    );
    let loaded = without_audio(call(&mut control, "/api/library/load", json!({"id":id})));
    assert_eq!(loaded["sample_id"], id.as_str());
    let state = without_audio(call(&mut control, "/api/state", Value::Null));
    assert_eq!(state["sample_id"], id.as_str());
    assert_eq!(state["fades"], fades);

    let trim = without_audio(call(
        &mut control,
        "/api/library/trim",
        json!({"start":0.25,"end":0.75,"fades":{"fade_in":0.0,"fade_out":0.0},"name":"Harp take-trim.wav","displayName":"Harp middle"}),
    ));
    assert_eq!(trim["trimmedFrom"], id.as_str());
    assert!((trim["seconds"].as_f64().unwrap() - 0.5).abs() < 0.001);

    let copy = folder.0.join("Exported.wav");
    call(
        &mut control,
        "/api/library/export",
        json!({"id":id,"path":copy.to_str().unwrap()}),
    );
    assert_eq!(
        std::fs::read(&copy).unwrap(),
        std::fs::read(&source).unwrap()
    );
    let factory = folder.0.join("Glass.wav");
    call(
        &mut control,
        "/api/library/export",
        json!({"id":"factory:glass-sequence:v1","path":factory.to_str().unwrap()}),
    );
    assert!(sad_pet::wav::read(&factory).unwrap().len() > 48000);

    call(&mut control, "/api/library/trash", json!({"id":id}));
    let listed = call(&mut control, "/api/library", Value::Null);
    let trashed = listed["files"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["id"] == id.as_str())
        .unwrap();
    assert!(trashed["trashedAt"].is_u64());
    call(
        &mut control,
        "/api/library/trash",
        json!({"id":id,"restore":true}),
    );
    assert!(
        call(&mut control, "/api/library", Value::Null)["files"]
            .as_array()
            .unwrap()
            .iter()
            .all(|f| f["trashedAt"].is_null())
    );
    let factory = call(
        &mut control,
        "/api/library/load",
        json!({"id":"factory:glass-sequence:v1"}),
    );
    assert_eq!(factory["sample_id"], "factory:glass-sequence:v1");
    assert!(
        control
            .request(
                &json!({"path":"/api/library/load","method":"POST","data":{"id":"audio:../../etc"}})
            )
            .is_err()
    );
}

#[test]
fn standalone_renders_the_scene_and_keeps_recordings_in_the_library() {
    let folder = PresetDirectory::new();
    std::fs::create_dir_all(&folder.0).unwrap();
    let mut control = folder.standalone();
    let _audio = control.prepare(48000);
    let render = without_audio(call(
        &mut control,
        "/api/library/render",
        json!({"seconds":1.0,"name":"sad-pet-render.wav","displayName":"glasses5 · 1s"}),
    ));
    assert_eq!(render["kind"], "render");
    assert_eq!(render["seconds"], 1.0);
    let id = render["id"].as_str().unwrap().to_string();
    let exported = folder.0.join("render.wav");
    call(
        &mut control,
        "/api/library/export",
        json!({"id":id,"path":exported.to_str().unwrap()}),
    );
    let rendered = sad_pet::wav::read(&exported).unwrap();
    let peak = rendered
        .frames()
        .iter()
        .flatten()
        .fold(0.0_f32, |p, v| p.max(v.abs()));
    assert!(peak > 0.001, "the offline render was silent");

    let take = folder.0.join("take.wav");
    sine_wav(&take, 24000);
    let recording = without_audio(call(
        &mut control,
        "/api/library/take",
        json!({"path":take.to_str().unwrap(),"source":"input","name":"sad-pet-microphone.wav","displayName":"Microphone"}),
    ));
    assert_eq!(recording["kind"], "recording");
    assert_eq!(recording["recordedSource"], "input");
    assert_eq!(recording["seconds"], 0.5);
    assert_eq!(
        call(&mut control, "/api/library", Value::Null)["files"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
}

fn wait_for_vocal(control: &mut Controller) {
    let started = std::time::Instant::now();
    loop {
        let state = call(control, "/api/vocal/state", Value::Null);
        let progress = &state["progress"];
        if progress["phrases"].as_u64() > Some(0) && progress["phrases"] == progress["rendered"] {
            return;
        }
        assert!(
            started.elapsed() < std::time::Duration::from_secs(30),
            "vocal phrases did not render: {state}"
        );
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

#[test]
fn vocal_timbres_sing_at_the_daw_position_and_survive_project_recall() {
    let voices = PresetDirectory::new();
    // SAFETY: tests that read the variable set the same value.
    unsafe { std::env::set_var("SAD_PET_VOICE_DIR", &voices.0) };
    let mut control = Controller::new(false);
    control.meta.mix.muted = true;
    let mut audio = control.prepare(8000);
    let values = control.values();
    let context = call(&mut control, "/api/timbres/add", json!({"kind":"vocal"}));
    let id = context["timbres"]
        .as_array()
        .unwrap()
        .iter()
        .find(|t| t["kind"] == "vocal")
        .unwrap()["id"]
        .as_u64()
        .unwrap();
    call(&mut control, "/api/timbres/select", json!({ "id": id }));
    let score = json!({"score":{"language":"ja","style":{"auto_breath":false},"notes":[
        {"id":1,"start":480,"length":480,"key":64,"lyric":"ら"},
        {"id":2,"start":960,"length":480,"key":67,"lyric":"ら"}]}});
    assert_eq!(
        call(&mut control, "/api/vocal/score", score.clone())["score"]["notes"][1]["key"],
        67
    );
    wait_for_vocal(&mut control);
    // The DAW plays from beat 1 (0.5 s at 120 BPM): the first note sings.
    no_heap(|| {
        audio.begin(&values, 120., Some(0.0), false, 256);
        render(&mut audio, 256, [0.; 2]);
        audio.begin(&values, 120., Some(1.0), true, 2048);
        assert!(render(&mut audio, 2048, [0.; 2]) > 0.01);
        // Before the phrase, and while the transport is stopped: silence.
        audio.begin(&values, 120., Some(0.0), true, 1024);
        assert!(render(&mut audio, 1024, [0.; 2]) < 1e-3);
        audio.begin(&values, 120., Some(1.0), false, 1024);
        assert!(render(&mut audio, 1024, [0.; 2]) < 1e-3);
    });
    // A Vocal timbre does not take a file or MIDI.
    assert!(
        control
            .request(&json!({"path":"/api/region","method":"POST","data":{"start":0,"end":1}}))
            .is_err()
    );

    // Project recall renders the saved score again in a new instance.
    let saved = control.save().unwrap();
    let mut recalled = Controller::new(false);
    recalled.restore(&saved).unwrap();
    let mut recalled_audio = recalled.prepare(8000);
    call(&mut recalled, "/api/timbres/select", json!({ "id": id }));
    wait_for_vocal(&mut recalled);
    recalled_audio.begin(&values, 120., Some(1.0), true, 2048);
    let _ = render(&mut recalled_audio, 1, [0.; 2]);
    recalled_audio.begin(&values, 120., Some(1.0), true, 2048);
    assert!(render(&mut recalled_audio, 2048, [0.; 2]) > 0.01);

    // An offline bounce waits for a phrase the edit just added.
    let added = json!({"score":{"language":"ja","style":{"auto_breath":false},"notes":[
        {"id":1,"start":480,"length":480,"key":64,"lyric":"ら"},
        {"id":2,"start":960,"length":480,"key":67,"lyric":"ら"},
        {"id":3,"start":3840,"length":480,"key":69,"lyric":"ら"}]}});
    call(&mut control, "/api/vocal/score", added);
    audio.offline = true;
    no_heap(|| {
        audio.begin(&values, 120., Some(8.0), true, 2048);
        assert!(
            render(&mut audio, 2048, [0.; 2]) > 0.01,
            "the bounce sings the new note"
        );
    });
    audio.offline = false;

    // The app's own song clock plays without a host transport.
    call(
        &mut control,
        "/api/vocal/transport",
        json!({"start": 0.5, "playing": true}),
    );
    audio.begin(&values, 0., None, false, 2048);
    assert!(render(&mut audio, 2048, [0.; 2]) > 0.01);
    control.drain();
    recalled.drain();
}

#[test]
fn host_midi_controllers_reach_the_editor_for_midi_learn_without_heap_work() {
    let mut control = Controller::new(false);
    let mut audio = control.prepare(48000);
    // The editor shows another timbre; the scene still reports the host input.
    call(&mut control, "/api/timbres/add", json!({"kind":"synth"}));
    let bytes = [
        0xb2, 74, 10, 0xb2, 74, 0xf8, 101, 0x90, 60, 100, 0xb0, 1, 64,
    ];
    no_heap(|| unsafe { sad_audio_midi(&mut audio, bytes.as_ptr(), bytes.len()) });
    let state = call(&mut control, "/api/state", Value::Null);
    assert_eq!(state["midi_controls"], json!([[0, 1, 64], [2, 74, 101]]));
    let state = call(&mut control, "/api/state", Value::Null);
    assert_eq!(state["midi_controls"], json!([]));
}

#[test]
fn instrument_presets_file_one_timbre_in_categories_and_load_it_in_place() {
    let folder = PresetDirectory::new();
    let mut control = folder.control(false);
    let mut audio = control.prepare(48000);
    let values = control.values();
    let request = |control: &mut Controller, path: &str, data: Value| {
        control.request(&json!({"path":path,"method":"POST","data":data}))
    };
    // The scene's own Granular timbre plays a sample of its own.
    let frames: Vec<Stereo> = (0..4800)
        .map(|n| [(n as f32 * 0.07).sin() * 0.4; 2])
        .collect();
    call(
        &mut control,
        "/api/wav",
        json!({"base64":BASE64.encode(wav::bytes(8000, &frames).unwrap()),"name":"Glass.wav"}),
    );
    call(&mut control, "/api/region", json!({"start":0.2,"end":0.8}));
    let glass = call(
        &mut control,
        "/api/instruments/save",
        json!({"name":"Glass","category":""}),
    );
    assert_eq!(glass["kind"], "granular");

    // A Synth timbre at half level on channel 3.
    call(&mut control, "/api/timbres/add", json!({"kind":"synth"}));
    call(
        &mut control,
        "/api/parameters",
        json!({"synth_count":2,"synth_osc2_transpose":7}),
    );
    call(
        &mut control,
        "/api/timbres/update",
        json!({"id":1,"gain":0.5,"channel":3}),
    );
    let lead = call(
        &mut control,
        "/api/instruments/save",
        json!({"name":"Wide Lead","category":"Leads"}),
    );
    assert_eq!([&lead["kind"], &lead["category"]], ["synth", "Leads"]);
    // Names are unique within a category, not across them.
    let taken = request(
        &mut control,
        "/api/instruments/save",
        json!({"name":"wide lead","category":"Leads"}),
    );
    assert!(taken.is_err_and(|e| e.contains("Leads")));
    let pad = call(
        &mut control,
        "/api/instruments/save",
        json!({"name":"Wide Lead","category":"Pads"}),
    );
    let replaced = call(
        &mut control,
        "/api/instruments/save",
        json!({"id":lead["id"],"name":"Wide Lead","category":"Leads"}),
    );
    assert_eq!(replaced["id"], lead["id"]);
    assert_eq!(replaced["created_at"], lead["created_at"]);
    let list = call(&mut control, "/api/instruments", Value::Null);
    let names: Vec<_> = list
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            format!(
                "{}/{}",
                r["category"].as_str().unwrap(),
                r["name"].as_str().unwrap()
            )
        })
        .collect();
    assert_eq!(names, ["/Glass", "Leads/Wide Lead", "Pads/Wide Lead"]);
    assert!(
        list.to_string().len() < 2000,
        "the list holds no sample data"
    );

    // Glass into the Synth timbre: it becomes Granular in its place, with its
    // level, channel and the scene's other timbres untouched, and the audio
    // thread swaps it without heap work.
    let context = call(
        &mut control,
        "/api/instruments/load",
        json!({"id":glass["id"]}),
    );
    let part = &context["timbres"][1];
    assert_eq!(
        [
            &part["id"],
            &part["kind"],
            &part["name"],
            &part["gain"],
            &part["channel"]
        ],
        [
            &json!(1),
            &json!("granular"),
            &json!("Glass"),
            &json!(0.5),
            &json!(3)
        ]
    );
    assert_eq!(context["timbres"][0]["kind"], "granular");
    let state = call(&mut control, "/api/state", Value::Null);
    assert_eq!(state["sample_name"], "Glass.wav");
    assert_eq!(state["region"]["start"], 0.2);
    assert_eq!(state["preset_selection"]["dirty"], true);
    call(
        &mut control,
        "/api/timbres/update",
        json!({"id":0,"muted":true}),
    );
    call(
        &mut control,
        "/api/transport",
        json!({"playing":true,"midi_gate":false}),
    );
    no_heap(|| {
        audio.begin(&values, 120., None, false, 512);
        render(&mut audio, 512, [0.; 2]);
    });
    audio.begin(&values, 120., None, false, 24000);
    assert!(
        render(&mut audio, 24000, [0.; 2]) > 0.001,
        "the loaded timbre plays"
    );
    call(&mut control, "/api/transport", json!({"playing":false}));

    // The Synth sound into the scene's own timbre: the scene keeps its name.
    call(&mut control, "/api/timbres/select", json!({"id":0}));
    let scene = control.preset.name.clone();
    let context = call(
        &mut control,
        "/api/instruments/load",
        json!({"id":lead["id"]}),
    );
    assert_eq!(context["name"], scene);
    assert_eq!(
        [
            &context["timbres"][0]["kind"],
            &context["timbres"][0]["name"]
        ],
        ["synth", "Wide Lead"]
    );
    let state = call(&mut control, "/api/state", Value::Null);
    assert_eq!(state["preset"]["synth"]["count"], 2);
    assert_eq!(state["preset"]["synth"]["oscillators"][1]["transpose"], 7.0);
    no_heap(|| {
        audio.begin(&values, 120., None, false, 512);
        render(&mut audio, 512, [0.; 2]);
    });

    // As a new timbre: added before Master and selected.
    let context = call(
        &mut control,
        "/api/instruments/load",
        json!({"id":glass["id"],"new":true}),
    );
    let kinds: Vec<_> = context["timbres"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t["kind"].as_str().unwrap())
        .collect();
    assert_eq!(kinds, ["synth", "granular", "granular"]);
    let added = context["active"].clone();
    assert_eq!(
        context["timbres"]
            .as_array()
            .unwrap()
            .iter()
            .find(|t| t["id"] == added)
            .unwrap()["name"],
        "Glass"
    );
    // A project saved now recalls every timbre's new sound.
    let mut recalled = Controller::new(false);
    recalled.restore(&control.save().unwrap()).unwrap();
    call(&mut recalled, "/api/timbres/select", json!({"id":1}));
    assert_eq!(
        call(&mut recalled, "/api/state", Value::Null)["sample_name"],
        "Glass.wav"
    );

    // A Bus holds no instrument.
    let context = call(&mut control, "/api/timbres/add", json!({"kind":"bus"}));
    let bus = context["timbres"]
        .as_array()
        .unwrap()
        .iter()
        .find(|t| t["id"] == context["active"])
        .unwrap();
    assert_eq!(bus["kind"], "bus");
    assert!(request(&mut control, "/api/instruments/save", json!({"name":"Out"})).is_err());
    assert!(
        request(
            &mut control,
            "/api/instruments/load",
            json!({"id":lead["id"]})
        )
        .is_err()
    );

    // Renaming, moving and whole categories keep names unique per category.
    assert!(
        request(
            &mut control,
            "/api/instruments/category",
            json!({"from":"Leads","to":"Pads"})
        )
        .is_err_and(|e| e.contains("Wide Lead"))
    );
    call(
        &mut control,
        "/api/instruments/edit",
        json!({"id":lead["id"],"name":"Lead 2","category":"Leads"}),
    );
    call(
        &mut control,
        "/api/instruments/category",
        json!({"from":"Leads","to":"Pads"}),
    );
    let list = call(&mut control, "/api/instruments", Value::Null);
    assert!(
        list.as_array()
            .unwrap()
            .iter()
            .all(|r| r["category"] != "Leads")
    );
    let moved = list
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["id"] == lead["id"])
        .unwrap();
    assert_eq!([&moved["name"], &moved["category"]], ["Lead 2", "Pads"]);
    // The moved preset still loads.
    call(&mut control, "/api/timbres/select", json!({"id":1}));
    call(
        &mut control,
        "/api/instruments/load",
        json!({"id":lead["id"]}),
    );

    // Export and import: a copy of its own, renamed if its category has one.
    let exported = call(
        &mut control,
        "/api/instruments/export",
        json!({"id":pad["id"]}),
    );
    assert_eq!(exported["filename"], "Wide Lead.sadinst");
    let imported = call(
        &mut control,
        "/api/instruments/import",
        json!({"base64":exported["binary"]}),
    );
    assert_ne!(imported["id"], pad["id"]);
    assert_eq!(
        [&imported["name"], &imported["category"]],
        ["Wide Lead (2)", "Pads"]
    );
    assert!(
        request(
            &mut control,
            "/api/instruments/import",
            json!({"base64":BASE64.encode(b"nope")})
        )
        .is_err_and(|e| e.contains(".sadinst"))
    );
    call(
        &mut control,
        "/api/instruments/delete",
        json!({"id":imported["id"]}),
    );
    assert_eq!(
        call(&mut control, "/api/instruments", Value::Null)
            .as_array()
            .unwrap()
            .len(),
        3
    );
    assert!(
        folder
            .0
            .join("Instruments/Trash")
            .read_dir()
            .unwrap()
            .next()
            .is_some()
    );
    assert!(
        request(
            &mut control,
            "/api/instruments/load",
            json!({"id":imported["id"]})
        )
        .is_err()
    );

    // The standalone editor sends and receives no instrument files.
    let mut standalone = folder.standalone();
    let _standalone_audio = standalone.prepare(48000);
    for (path, data) in [
        (
            "/api/instruments/import",
            json!({"base64":exported["binary"]}),
        ),
        ("/api/instruments/export", json!({"id":pad["id"]})),
    ] {
        assert!(
            request(&mut standalone, path, data).is_err_and(|e| e.contains("natively")),
            "{path} was not refused"
        );
    }
    without_audio(call(&mut standalone, "/api/instruments", Value::Null));
}
