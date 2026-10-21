use super::*;
use std::{
    alloc::{GlobalAlloc, Layout, System},
    cell::Cell,
};
struct Allocator;
thread_local! {static TRACK:Cell<bool>=const{Cell::new(false)};static COUNT:Cell<usize>=const{Cell::new(0)};}
fn count() {
    let _ = TRACK.try_with(|v| {
        if v.get() {
            let _ = COUNT.try_with(|n| n.set(n.get() + 1));
        }
    });
}
unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        count();
        unsafe { System.alloc(l) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        count();
        unsafe { System.dealloc(p, l) }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        count();
        unsafe { System.realloc(p, l, n) }
    }
}
#[global_allocator]
static ALLOCATOR: Allocator = Allocator;
fn no_heap(f: impl FnOnce()) {
    COUNT.with(|n| n.set(0));
    TRACK.with(|t| t.set(true));
    f();
    TRACK.with(|t| t.set(false));
    assert_eq!(
        COUNT.with(Cell::get),
        0,
        "Audio-side snapshot allocated/freed"
    );
}

#[test]
fn telemetry_decoder_does_not_initialize_an_audio_registry() {
    HOST.with(|h| *h.borrow_mut() = None);
    telemetry::sad_telemetry_init(48_000);
    assert!(HOST.with(|h| h.borrow().is_none()));
}

#[test]
fn telemetry_deltas_keep_presets_and_waveforms_until_they_change() {
    let mut audio = Host::new(8000);
    let mut worker = Host::new(8000);
    let transfer = |audio: &mut Host, worker: &mut Host, delta| {
        let length = audio.publish_snapshot();
        assert!(length > 0);
        worker.input.clear();
        worker
            .input
            .extend_from_slice(&audio.snapshot_buffer[..length]);
        let result = worker.import_snapshot(delta).unwrap();
        audio.ack_snapshot(audio.snapshot_sequence, 1);
        result
    };
    let initial = transfer(&mut audio, &mut worker, true);
    assert!(initial["state"]["preset"].is_object());
    assert!(initial["state"]["waveform"].is_array());
    let tick = transfer(&mut audio, &mut worker, true);
    assert!(tick["state"].get("preset").is_none());
    assert!(tick["state"].get("waveform").is_none());
    audio
        .command(json!({"path":"/api/parameters","data":{"filter":0.41}}))
        .unwrap();
    let edited = transfer(&mut audio, &mut worker, true);
    assert_eq!(
        edited["state"]["preset"]["params"]["filter"],
        json!(0.41_f32)
    );
    let tick = transfer(&mut audio, &mut worker, true);
    assert!(tick["state"].get("preset").is_none());
    let full = transfer(&mut audio, &mut worker, false);
    assert_eq!(full["state"]["preset"], edited["state"]["preset"]);
    assert_eq!(full["state"]["waveform"], initial["state"]["waveform"]);
    // A rejected main-thread delta must trigger a complete resynchronization.
    let length = audio.publish_snapshot();
    assert!(length > 0);
    audio.ack_snapshot(audio.snapshot_sequence, 0);
    let reset = transfer(&mut audio, &mut worker, true);
    assert!(reset["state"]["preset"].is_object());
    assert!(reset["state"]["sample_name"].is_string());
    worker
        .command(json!({"path":"/api/preset","data":reset["state"]["preset"]}))
        .unwrap();
    assert!(
        worker
            .engine
            .process_frame()
            .into_iter()
            .all(f32::is_finite)
    );
}

#[test]
fn neuropolis_large_patches_publish_sparse_edits_without_heap_or_full_configuration() {
    use sad_pet::sequence::{Edit, Target, Track};
    let mut audio = Host::new(8000);
    audio.engine.set_sample(Arc::new(SampleBuffer::demo(8000)));
    let mut preset = audio.engine.preset.clone();
    preset.sequencer.enabled = true;
    preset.sequencer.tracks = (1..=257)
        .map(|id| Track {
            id,
            target: Target::MainPitch,
            ..Track::default()
        })
        .collect();
    audio
        .command(json!({"path":"/api/preset","data":preset}))
        .unwrap();
    assert!(audio.snapshot_buffer.len() > 131072);
    audio.engine.set_playing(true);
    let mut length = 0;
    no_heap(|| {
        for _ in 0..512 {
            audio.engine.process_frame();
        }
        length = audio.publish_snapshot();
    });
    assert!(length > 131072);
    let mut worker = Host::new(8000);
    worker
        .input
        .extend_from_slice(&audio.snapshot_buffer[..length]);
    let state = worker.import_snapshot(false).unwrap();
    assert_eq!(
        state["state"]["preset"]["sequencer"]["tracks"]
            .as_array()
            .unwrap()
            .len(),
        257
    );
    // Acknowledgement is a control message, outside the process callback.
    audio.unacked_peaks.fill(None);
    audio.sent_edit_revisions.fill(None);
    let revision = audio.config_revision;
    audio.command(json!({"path":"/api/sequencer/edit","data":[{"track":256,"stage":3,"field":0,"value":77}]})).unwrap();
    audio
        .command(json!({"path":"/api/parameters","data":{"filter":0.41}}))
        .unwrap();
    assert_eq!(
        revision, audio.config_revision,
        "A scalar drag resent the full program"
    );
    let edits = [Edit {
        track: 1,
        stage: Some(9),
        field: 0,
        value: 67.0,
    }];
    no_heap(|| {
        audio.engine.apply_sequence_edits(&edits);
        for _ in 0..512 {
            audio.engine.process_frame();
        }
        length = audio.publish_snapshot();
    });
    assert!(
        length > 0 && length < 40000,
        "Only feedback and sparse edits should cross back to the worker: {length}"
    );
    worker.input.clear();
    worker
        .input
        .extend_from_slice(&audio.snapshot_buffer[..length]);
    let state = worker.import_snapshot(false).unwrap();
    assert_eq!(
        state["state"]["preset"]["sequencer"]["tracks"][255]["steps"][3]["pitch"],
        77
    );
    assert!(
        (state["state"]["preset"]["params"]["filter"]
            .as_f64()
            .unwrap()
            - 0.41)
            .abs()
            < 1e-6
    );
}

#[test]
fn resynth_render_scalar_edits_and_binary_meters_have_no_heap_operations() {
    let mut audio = Host::new(48000);
    audio.engine.set_sample(Arc::new(SampleBuffer::demo(48000)));
    audio.engine.set_live_input(true, false);
    audio.engine.set_playing(true);
    audio.engine.preset.resynth.enabled = true;
    audio.engine.preset.resynth.count = 4;
    for (i, o) in audio
        .engine
        .preset
        .resynth
        .oscillators
        .iter_mut()
        .enumerate()
    {
        o.waveform = sad_pet::resynth::Waveform::ALL[i];
        o.mode = sad_pet::resynth::Mode::ALL[i];
        o.control1 = 0.6;
        o.control2 = 0.4;
    }
    audio.engine.preset.resynth.oscillators[3].waveform = sad_pet::resynth::Waveform::Pcm;
    audio.engine.preset.resynth.oscillators[3].mode = sad_pet::resynth::Mode::Wave;
    audio.engine.refresh_parameters();
    let changes = sad_pet::parameters::parse(
        &json!({"resynth_osc4_control1":0.8,"resynth_osc2_transpose":12,"resynth_mix":0.7,"resynth_osc2_model":1,"resynth_osc2_fold_shape":4,"resynth_osc2_interaction":3,"resynth_osc2_interaction_source":3,"resynth_osc1_interaction":3,"resynth_osc1_interaction_source":1}),
    )
    .unwrap();
    let mut n = 0;
    no_heap(|| {
        audio.engine.apply_parameters(&changes);
        for n in 0..4096 {
            audio
                .engine
                .process_input_frame([(n as f32 * 0.04).sin() * 0.2; 2]);
        }
        n = audio.publish_snapshot();
        assert!(n > 0);
    });
    let mut worker = Host::new(48000);
    worker.input.extend_from_slice(&audio.snapshot_buffer[..n]);
    let state = worker.import_snapshot(false).unwrap();
    assert_eq!(
        state["state"]["resynth"]["bands"].as_array().unwrap().len(),
        16
    );
    assert_eq!(state["state"]["preset"]["resynth"]["count"], 4);
}
#[test]
fn binary_publication_has_no_heap_and_worker_matches_original_state() {
    let mut audio = Host::new(8000);
    audio.engine.set_sample(Arc::new(SampleBuffer::demo(8000)));
    audio.engine.set_playing(true);
    audio.engine.midi(Message::NoteOn {
        channel: 0,
        note: 72,
        velocity: 100,
    });
    for _ in 0..1024 {
        audio.engine.process_frame();
    }
    let expected = audio.state();
    let mut n = 0;
    no_heap(|| n = audio.publish_snapshot());
    assert!(n > 0 && n < 131072);
    let mut worker = Host::new(8000);
    worker.input.extend_from_slice(&audio.snapshot_buffer[..n]);
    let result = worker.import_snapshot(false).unwrap();
    for key in [
        "preset",
        "waveform",
        "notes",
        "knobs",
        "grains",
        "peak",
        "performance",
    ] {
        assert_eq!(result["state"][key], expected[key], "{key}");
    }
    let revision = audio.config_revision;
    audio.command(json!({"path":"/api/note","data":{"kind":"note_on","channel":0,"note":67,"velocity":100}})).unwrap();
    assert_eq!(
        audio.config_revision, revision,
        "A note resent the whole configuration"
    );
    for _ in 0..2 {
        no_heap(|| {
            assert!(audio.publish_snapshot() > 0);
        });
    }
    audio.engine.peak = [0.91; 2];
    no_heap(|| assert_eq!(audio.publish_snapshot(), 0));
    assert_eq!(audio.engine.peak, [0.91; 2]);
}

#[test]
fn source_pcm_and_mips_upload_without_allocating_or_restarting_the_audio() {
    sad_init(8000);
    let recipe = host(|h| {
        h.engine.set_sample(Arc::new(SampleBuffer::demo(8000)));
        h.reset_source_buffers();
        h.file_generation = 1;
        h.engine
            .set_region(sad_pet::dsp::Region {
                start: 0.125,
                end: 0.375,
            })
            .unwrap();
        h.engine.preset.source_color_chain = vec![sad_pet::color::ColorSlot {
            id: 70,
            mix: 1.0,
            ..Default::default()
        }];
        sad_pet::dsp::source_fx::Recipe::from_engine(&h.engine)
    });
    let cache = host(|h| {
        sad_pet::dsp::source_fx::prepare(
            h.engine.sample.clone(),
            recipe.region,
            recipe.slot,
            recipe.bpm,
            recipe.root,
            || false,
        )
        .unwrap()
        .unwrap()
    });
    let header = Prepared {
        name: cache.sample.name.clone(),
        library_id: None,
        sample_rate: 8000,
        lengths: cache
            .sample
            .prepared_levels()
            .iter()
            .map(Vec::len)
            .collect(),
    };
    host(|h| {
        h.input =
            serde_json::to_vec(&json!({"recipe":recipe,"header":header,"generation":1})).unwrap()
    });
    assert_eq!(source_fx::sad_source_begin(), 200);
    let mut status = 0;
    for level in cache.sample.prepared_levels() {
        for chunk in level.chunks(BLOCK) {
            no_heap(|| {
                host(|h| h.source_upload[..chunk.len()].copy_from_slice(chunk));
                status = source_fx::sad_source_push(chunk.len());
            });
        }
    }
    assert_eq!(status, 1);
    host(|h| {
        let imported = &h.engine.processed_source.as_ref().unwrap().sample;
        assert_eq!(imported.prepared_levels(), cache.sample.prepared_levels());
        assert_eq!(imported.waveform, cache.sample.waveform);
        assert!(!h.engine.source_effects_pending());
    });
}

#[test]
fn drum_step_edits_and_granular_routing_publish_sparse_views_without_heap_work() {
    let mut host = Host::new(8000);
    let preset = Preset {
        drums: Some(Box::default()),
        ..Preset::default()
    };
    host.command(json!({"path":"/api/preset","data":preset}))
        .unwrap();
    host.engine.set_playing(true);
    let first = host.publish_snapshot();
    assert!(first > 0);
    host.ack_snapshot(host.snapshot_sequence, 1);
    let revision = host.config_revision;
    host.command(json!({"path":"/api/drums/edit","data":[{"track":0,"step":127,"field":0,"value":0.8},{"track":0,"step":null,"field":0,"value":128}]})).unwrap();
    host.command(json!({"path":"/api/parameters","data":{"drum_granular":1,"drum1_0":72,"drum_buffer_seconds":0.125}}))
        .unwrap();
    assert_eq!(
        host.config_revision, revision,
        "A drum gesture resent the whole patch"
    );
    let mut size = 0;
    no_heap(|| {
        for _ in 0..2048 {
            host.engine.process_frame();
        }
        size = host.publish_snapshot();
    });
    assert!(
        size > 0 && size < 40000,
        "Expected only sparse drum edits and fixed telemetry"
    );
    let mut worker = Host::new(8000);
    worker.engine.preset = host.engine.preset.clone();
    worker
        .input
        .extend_from_slice(&host.snapshot_buffer[..size]);
    let view = worker.import_snapshot(false).unwrap();
    assert_eq!(
        view["state"]["preset"]["drums"]["tracks"][0]["steps"][127]["velocity"],
        json!(0.8_f32)
    );
    assert_eq!(view["state"]["preset"]["drums"]["granular"], true);
    assert_eq!(view["state"]["sample_seconds"], 0.125);
    assert_eq!(view["state"]["sample_frames"], 1000);
    host.ack_snapshot(host.snapshot_sequence, 1);
    // A second sparse edit must update duration even when configuration is omitted.
    host.command(json!({"path":"/api/parameters","data":{"drum_buffer_seconds":16}}))
        .unwrap();
    no_heap(|| {
        for _ in 0..128 {
            host.engine.process_frame();
        }
        size = host.publish_snapshot();
    });
    worker.input.clear();
    worker
        .input
        .extend_from_slice(&host.snapshot_buffer[..size]);
    let view = worker.import_snapshot(false).unwrap();
    assert_eq!(view["state"]["sample_seconds"], 16.);
    assert_eq!(view["state"]["sample_frames"], 128000);
    assert_eq!(host.config_revision, revision);
}
