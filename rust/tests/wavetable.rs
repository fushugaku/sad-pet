use sad_pet::{
    dsp::{Engine, SampleBuffer, telemetry::RealtimeView, wavetable::Runtime},
    midi::Message,
    preset::Preset,
    rack::Module,
    timbres::Kind,
    wavetable::{
        FRAME_SIZE, Settings,
        assets::{Asset, Pool},
    },
};
use std::{
    alloc::{GlobalAlloc, Layout, System},
    cell::Cell,
    sync::Arc,
};

struct Allocator;
thread_local! {
    static TRACK: Cell<bool> = const { Cell::new(false) };
    static HEAP: Cell<usize> = const { Cell::new(0) };
}
fn count() {
    let _ = TRACK.try_with(|t| {
        if t.get() {
            let _ = HEAP.try_with(|n| n.set(n.get() + 1));
        }
    });
}
unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        count();
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        count();
        unsafe { System.dealloc(pointer, layout) }
    }
    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        count();
        unsafe { System.realloc(pointer, layout, size) }
    }
}
#[global_allocator]
static ALLOCATOR: Allocator = Allocator;
fn no_heap(f: impl FnOnce()) {
    HEAP.with(|n| n.set(0));
    TRACK.with(|t| t.set(true));
    f();
    TRACK.with(|t| t.set(false));
    assert_eq!(
        HEAP.with(Cell::get),
        0,
        "allocation or retirement in the audio path"
    );
}
fn sample() -> Arc<SampleBuffer> {
    Arc::new(SampleBuffer::new("Unused".into(), 48_000, vec![[0.0; 2]; 128]).unwrap())
}
fn engine() -> Engine {
    let mut engine = Engine::new(48_000, sample(), Kind::Wavetable.preset().unwrap());
    engine.set_midi_gate(true);
    engine.set_playing(true);
    engine
}
fn note(engine: &mut Engine, pitch: u8) {
    engine.midi(Message::NoteOn {
        channel: 0,
        note: pitch,
        velocity: 100,
    });
}
fn user_sound() -> (Settings, Pool) {
    let samples: Vec<_> = (0..FRAME_SIZE * 2)
        .map(|i| {
            (i as f64 * std::f64::consts::TAU / FRAME_SIZE as f64).sin() as f32
                * if i < FRAME_SIZE { 0.8 } else { 0.2 }
        })
        .collect();
    let asset = Asset::from_cycles("Independent attacks".into(), FRAME_SIZE, &samples).unwrap();
    let mut settings = Settings::default();
    for oscillator in &mut settings.oscillators {
        oscillator.asset_id = asset.id.clone();
    }
    let mut pool = Pool::default();
    pool.insert(asset).unwrap();
    (settings, pool)
}

#[test]
fn released_host_parameters_keep_the_entire_prefix() {
    let before: serde_json::Value = serde_json::from_str(include_str!(
        "fixtures/wavetable/host_parameters_before.json"
    ))
    .unwrap();
    // Compare the actual serde wire format; Value::from(f32) widens its
    // mantissa while direct JSON serialization uses the f32 representation.
    let current: serde_json::Value =
        serde_json::from_slice(&serde_json::to_vec(sad_pet::parameters::specs()).unwrap()).unwrap();
    let length = before.as_array().unwrap().len();
    for index in 0..length {
        assert_eq!(
            current[index], before[index],
            "changed released host slot {index}"
        );
    }
    assert_eq!(current[length]["id"], "wt_a_enabled");
    let ab: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/wavetable/ab_parameters_before.json")).unwrap();
    for (offset, before) in ab.as_array().unwrap().iter().enumerate() {
        assert_eq!(
            &current[length + offset],
            before,
            "changed A/B host slot {offset}"
        );
    }
    assert_eq!(current[length + 30]["id"], "wt_c_enabled");
    let thirdwave: serde_json::Value = serde_json::from_str(include_str!(
        "fixtures/wavetable/thirdwave_parameters_before.json"
    ))
    .unwrap();
    let prefix = thirdwave.as_array().unwrap();
    assert_eq!(&current.as_array().unwrap()[..prefix.len()], prefix);
    assert_eq!(current[prefix.len()]["id"], "env1_wave_on");
    let all = current.as_array().unwrap();
    let prior: serde_json::Value = serde_json::from_reader(flate2::read::GzDecoder::new(
        &include_bytes!("fixtures/wavetable/optimization_parameters_before.json.gz")[..],
    ))
    .unwrap();
    let prior = prior.as_array().unwrap();
    assert_eq!(&all[..prior.len()], prior);
    assert_eq!(all[prior.len()]["id"], "wt_a_fm_source");
    for (slot, letter) in all[prior.len() - 4..prior.len()]
        .iter()
        .zip(["a", "b", "c", "d"])
    {
        assert_eq!(slot["id"], format!("wt_{letter}_voice_shift"));
        assert_eq!(slot["default"], 0.0);
    }
}

#[test]
fn every_fm_and_sync_source_pair_and_feedback_ring_runs_without_audio_heap_work() {
    let pool = Pool::default();
    for sync in [false, true] {
        for carrier in 0..4 {
            for source in 0..4 {
                let mut s = Settings::default();
                s.count = 4;
                for o in &mut s.oscillators {
                    o.controls.enabled = true;
                    o.controls.level = 0.0;
                    o.controls.unison = 2;
                }
                s.oscillators[source].controls.transpose = 12.0;
                let c = &mut s.oscillators[carrier].controls;
                c.level = 0.7;
                c.fm_mode = if sync { 0 } else { 1 };
                c.fm_index = 1.5;
                c.fm_source = source as u8;
                c.sync = sync;
                c.sync_source = source as u8;
                c.sync_ratio = 3.25;
                let mut r = Runtime::new(48000, &s, &pool).unwrap();
                let mut peak = 0.0_f32;
                no_heap(|| {
                    for _ in 0..2400 {
                        r.begin_frame(true);
                        for x in r.render(0, 1, 60.0, &[0.0; 8]) {
                            assert!(x.is_finite());
                            peak = peak.max(x.abs());
                        }
                    }
                });
                assert!(
                    peak > 1e-5,
                    "silent route {carrier} <- {source}, sync {sync}"
                );
            }
        }
    }
    let mut s = Settings::default();
    s.count = 4;
    for (i, o) in s.oscillators.iter_mut().enumerate() {
        let c = &mut o.controls;
        c.enabled = true;
        c.level = 0.07;
        c.unison = 4;
        c.fm_mode = 1;
        c.fm_index = 2.0;
        c.fm_source = ((i + 1) % 4) as u8;
        c.sync = true;
        c.sync_source = ((i + 3) % 4) as u8;
        c.sync_ratio = 2.75;
        c.voice_shift = 0.08;
    }
    let mut r = Runtime::new(48000, &s, &pool).unwrap();
    let mut changed = s.controls();
    changed[0].fm_source = 2;
    changed[2].sync_source = 1;
    no_heap(|| {
        for n in 0..2400 {
            if n == 1200 {
                r.configure(changed, 0.0);
            }
            r.begin_frame(true);
            for voice in 0..8 {
                assert!(
                    r.render(voice, 1, 48.0 + voice as f32 * 2.0, &[0.0; 8])
                        .iter()
                        .all(|x| x.is_finite())
                );
            }
        }
    });
    let restored: Settings = serde_json::from_slice(&serde_json::to_vec(&s).unwrap()).unwrap();
    assert_eq!(s, restored);
    let mut old = serde_json::to_value(&s).unwrap();
    old["engine_revision"] = 2.into();
    for o in old["oscillators"].as_array_mut().unwrap() {
        o.as_object_mut().unwrap().remove("fm_source");
        o.as_object_mut().unwrap().remove("sync_source");
    }
    let migrated: Settings = serde_json::from_value(old).unwrap();
    for (i, o) in migrated.oscillators.iter().enumerate() {
        assert_eq!(o.controls.fm_source as usize, i ^ 1);
        assert_eq!(o.controls.sync_source as usize, i);
    }
}

#[test]
fn unison_voice_shift_selects_distinct_frames_and_steps_each_voice() {
    // The three frames contain different Fourier partials. Three coherent
    // unison voices at 0/50/100% must sound all three, rather than detuning
    // three copies of the middle frame. Projections are independent of lookup.
    let samples = (1..=3)
        .flat_map(|harmonic| {
            (0..FRAME_SIZE).map(move |n| {
                (std::f64::consts::TAU * harmonic as f64 * n as f64 / FRAME_SIZE as f64).sin()
                    as f32
                    * 0.6
            })
        })
        .collect::<Vec<_>>();
    let asset =
        Asset::from_cycles("Unison Fourier positions".into(), FRAME_SIZE, &samples).unwrap();
    let mut pool = Pool::default();
    let id = pool.insert(asset).unwrap().id.clone();
    let mut settings = Settings::default();
    settings.oscillators[0].asset_id = id;
    let c = &mut settings.oscillators[0].controls;
    c.position = 0.5;
    c.unison = 3;
    c.detune = 0.0;
    c.spread = 0.0;
    for (shift, step, expected) in [
        (0.0, false, [0.0, 0.3, 0.0]),
        (0.5, false, [0.1, 0.1, 0.1]),
        (-0.5, false, [0.1, 0.1, 0.1]),
        (0.25, true, [0.0, 0.2, 0.1]),
    ] {
        settings.oscillators[0].controls.voice_shift = shift;
        settings.oscillators[0].controls.step = step;
        let mut runtime = Runtime::new(48000, &settings, &pool).unwrap();
        let mut output = vec![0.0_f32; 48000];
        no_heap(|| {
            for _ in 0..2400 {
                runtime.begin_frame(true);
                runtime.render(0, 1, 69.0, &[0.0; 8]);
            }
            for value in &mut output {
                runtime.begin_frame(true);
                *value = runtime.render(0, 1, 69.0, &[0.0; 8])[0];
            }
        });
        for (i, expected) in expected.into_iter().enumerate() {
            let (mut re, mut im) = (0.0, 0.0);
            for (n, &value) in output.iter().enumerate() {
                let phase = std::f64::consts::TAU * 440.0 * (i + 1) as f64 * n as f64 / 48000.0;
                re += value as f64 * phase.cos();
                im += value as f64 * phase.sin();
            }
            let amplitude = re.hypot(im) * 2.0 / output.len() as f64;
            assert!(
                (amplitude - expected).abs() < 2e-4,
                "shift {shift}, Step {step}, harmonic {}: {amplitude} != {expected}",
                i + 1
            );
        }
        let view = runtime.positions()[0];
        assert!((0.0..=1.0).contains(&view.min) && (view.min..=1.0).contains(&view.max));
        if shift.abs() == 0.5 {
            assert!(view.min < 0.001 && view.max > 0.999);
        }
    }
    settings.oscillators[0].controls.unison = 1;
    settings.oscillators[0].controls.step = false;
    settings.oscillators[0].controls.voice_shift = 0.0;
    let mut plain = Runtime::new(48000, &settings, &pool).unwrap();
    settings.oscillators[0].controls.voice_shift = 1.0;
    let mut shifted = Runtime::new(48000, &settings, &pool).unwrap();
    no_heap(|| {
        for _ in 0..2000 {
            plain.begin_frame(true);
            shifted.begin_frame(true);
            assert_eq!(
                plain.render(0, 1, 69.0, &[0.0; 8]),
                shifted.render(0, 1, 69.0, &[0.0; 8])
            );
        }
    });
}

#[test]
fn creation_is_a_rack_and_factory_recall_needs_no_sample() {
    let preset = Kind::Wavetable.preset().unwrap();
    assert_eq!(Kind::of(&preset), Kind::Rack);
    let modules: Vec<_> = preset
        .rack
        .as_ref()
        .unwrap()
        .modules
        .iter()
        .map(|s| s.module)
        .collect();
    assert_eq!(
        modules,
        vec![
            Module::Wavetable,
            Module::Filters,
            Module::Mixer,
            Module::PostFx
        ]
    );
    let restored: Preset = serde_json::from_slice(&serde_json::to_vec(&preset).unwrap()).unwrap();
    assert_eq!(restored.wavetable, preset.wavetable);
    let mut engine = engine();
    note(&mut engine, 60);
    let mut peak = 0.0_f32;
    no_heap(|| {
        for _ in 0..4800 {
            for x in engine.process_frame() {
                assert!(x.is_finite());
                peak = peak.max(x.abs());
            }
        }
    });
    assert!(peak > 0.01);
    assert!(engine.rack_peaks().unwrap()[6] > 0.01);
}

#[test]
fn four_waves_follow_the_mixer_and_feed_the_racks_grain_history() {
    let make = |modules: &[Module], muted: bool| {
        let mut p = Kind::Wavetable.preset().unwrap();
        p.wavetable.as_mut().unwrap().count = 4;
        for o in &mut p.wavetable.as_mut().unwrap().oscillators {
            o.controls.enabled = true;
            o.controls.level = 0.25;
        }
        p.rack.as_mut().unwrap().modules = modules
            .iter()
            .map(|&module| sad_pet::rack::Slot {
                module,
                enabled: true,
                id: 0,
            })
            .collect();
        p.rack.as_mut().unwrap().strips.wavetable.muted = muted;
        if modules.contains(&Module::Grains) {
            p.params.mix = 1.0;
            p.params.grains = 0.7;
            p.params.grain_size = 0.5;
            p.params.cloud = 0.25;
        }
        let mut e = Engine::new(48_000, sample(), p);
        e.set_midi_gate(true);
        e.set_playing(true);
        note(&mut e, 60);
        e
    };
    let mut before = make(&[Module::Wavetable, Module::Mixer], true);
    let mut after = make(&[Module::Mixer, Module::Wavetable], true);
    let mut grains = make(&[Module::Wavetable, Module::Grains, Module::Mixer], false);
    let mut peaks = [0.0_f32; 3];
    no_heap(|| {
        for _ in 0..24_000 {
            for (i, e) in [&mut before, &mut after, &mut grains]
                .into_iter()
                .enumerate()
            {
                for x in e.process_frame() {
                    assert!(x.is_finite());
                    peaks[i] = peaks[i].max(x.abs());
                }
            }
        }
    });
    assert!(peaks[0] < 1e-6, "muted WT strip leaked: {}", peaks[0]);
    assert!(
        peaks[1] > 0.01,
        "WT below the Mixer must follow the existing bypass-strip rule"
    );
    assert!(
        peaks[2] > 0.001,
        "Grains did not read the WT chain; its original Sample is silent"
    );
}

#[test]
fn the_engine_applies_the_note_vca_exactly_once() {
    let make = |sustain| {
        let mut p = Kind::Wavetable.preset().unwrap();
        p.envs[0].attack = 0.0;
        p.envs[0].decay = 0.0;
        p.envs[0].sustain = sustain;
        p.params.adsr_attack = 0.0;
        p.params.adsr_decay = 0.0;
        p.params.adsr_sustain = sustain;
        let mut e = Engine::new(48_000, sample(), p);
        e.set_midi_gate(true);
        e.set_playing(true);
        note(&mut e, 60);
        e
    };
    let (mut full, mut half) = (make(1.0), make(0.5));
    let (mut a, mut b) = (0.0_f64, 0.0_f64);
    no_heap(|| {
        for i in 0..4096 {
            let x = full.process_frame();
            let y = half.process_frame();
            if i > 512 {
                a += x[0] as f64 * x[0] as f64;
                b += y[0] as f64 * y[0] as f64;
            }
        }
    });
    assert!(
        ((b / a).sqrt() - 0.5).abs() < 0.002,
        "VCA ratio {}",
        (b / a).sqrt()
    );
}

#[test]
fn four_waves_add_remove_and_recall_while_notes_keep_playing_without_heap() {
    let mut e = engine();
    note(&mut e, 60);
    let mut p = e.preset.clone();
    for i in 1..4 {
        p.wavetable
            .as_mut()
            .unwrap()
            .edit_layers("add", None)
            .unwrap();
        p.wavetable.as_mut().unwrap().oscillators[i]
            .controls
            .transpose = i as f32 * 7.0;
        let mut prepared = p.prepared_clone_for(48_000);
        no_heap(|| {
            e.exchange_preset(&mut prepared);
            for _ in 0..1024 {
                assert!(e.process_frame().into_iter().all(f32::is_finite));
            }
        });
        assert_eq!(e.preset.wavetable.as_ref().unwrap().count, i as u8 + 1);
    }
    assert!(
        p.wavetable
            .as_mut()
            .unwrap()
            .edit_layers("add", None)
            .is_err()
    );
    let controls=sad_pet::parameters::parse(&serde_json::json!({"wt_c_position":0.7,"wt_d_position":0.4,"wt_d_fine":-8.0,"wt_c_unison":4})).unwrap();
    no_heap(|| {
        e.apply_parameters(&controls);
        for _ in 0..1024 {
            assert!(e.process_frame().into_iter().all(f32::is_finite));
        }
    });
    assert_eq!(
        e.preset.wavetable.as_ref().unwrap().oscillators[3]
            .controls
            .fine,
        -8.0
    );
    p = e.preset.clone();
    p.wavetable
        .as_mut()
        .unwrap()
        .edit_layers("remove", Some(1))
        .unwrap();
    let mut prepared = p.prepared_clone_for(48_000);
    no_heap(|| {
        e.exchange_preset(&mut prepared);
        for _ in 0..2048 {
            assert!(e.process_frame().into_iter().all(f32::is_finite));
        }
    });
    assert_eq!(e.preset.wavetable.as_ref().unwrap().count, 3);
    assert_eq!(
        e.preset.wavetable.as_ref().unwrap().oscillators[1]
            .controls
            .transpose,
        14.0
    );
    let recalled: Preset = serde_json::from_slice(&serde_json::to_vec(&e.preset).unwrap()).unwrap();
    assert_eq!(recalled.wavetable, e.preset.wavetable);
    assert!(!recalled.wavetable.as_ref().unwrap().controls()[3].enabled);
}

#[test]
fn local_position_envelopes_do_not_follow_the_last_note() {
    let (mut settings, pool) = user_sound();
    settings.oscillators[0].controls.position = 0.1;
    settings.oscillators[0].controls.position_env = 2;
    settings.oscillators[0].controls.position_env_depth = 0.75;
    let mut runtime = Runtime::new(48_000, &settings, &pool).unwrap();
    assert!(
        (runtime.positions()[0].current - 0.1).abs() < 1e-6,
        "idle preview must show the selected Position"
    );
    let mut first = [0.0; 8];
    first[1] = 0.2;
    let mut second = [0.0; 8];
    second[1] = 0.8;
    runtime.begin_frame(true);
    runtime.render(1, 1, 60.0, &first);
    runtime.render(2, 1, 60.0, &second);
    let view = runtime.positions()[0];
    assert!((view.min - 0.25).abs() < 1e-6);
    assert!((view.max - 0.7).abs() < 1e-6);
    first[1] = 0.9;
    for _ in 0..64 {
        runtime.begin_frame(true);
        runtime.render(1, 1, 60.0, &first);
        runtime.render(2, 1, 60.0, &second);
    }
    let view = runtime.positions()[0];
    assert!((view.current - 0.7).abs() < 1e-6);
    assert!((view.max - 0.775).abs() < 1e-6);
}

#[test]
fn compatible_updates_and_sparse_controls_keep_phases_without_heap() {
    let (settings, pool) = user_sound();
    let mut a = Runtime::new(48_000, &settings, &pool).unwrap();
    let mut b = Runtime::new(48_000, &settings, &pool).unwrap();
    let mut replacement = Runtime::new(48_000, &settings, &pool).unwrap();
    let mut retired = std::array::from_fn(|_| None);
    no_heap(|| {
        for _ in 0..500 {
            a.begin_frame(true);
            b.begin_frame(true);
            assert_eq!(
                a.render(1, 1, 60.0, &[0.0; 8]),
                b.render(1, 1, 60.0, &[0.0; 8])
            );
        }
        replacement.inherit(&mut a);
        std::mem::swap(&mut a, &mut replacement);
        for _ in 0..2000 {
            a.begin_frame(true);
            b.begin_frame(true);
            assert_eq!(
                a.render(1, 1, 60.0, &[0.0; 8]),
                b.render(1, 1, 60.0, &[0.0; 8])
            );
        }
        a.retire_into(&mut retired);
    });
    let mut engine = engine();
    note(&mut engine, 60);
    let changes=sad_pet::parameters::parse(&serde_json::json!({"wt_a_position":0.7,"wt_a_detune":17.0,"wt_a_unison":4.0,"wt_a_env":2.0,"wt_a_env_depth":0.4})).unwrap();
    no_heap(|| {
        engine.apply_parameters(&changes);
        for _ in 0..2000 {
            assert!(engine.process_frame().iter().all(|x| x.is_finite()));
        }
        let mut view = RealtimeView::default();
        engine.realtime_view(&mut view);
    });
    assert_eq!(
        engine.preset.wavetable.as_ref().unwrap().oscillators[0]
            .controls
            .unison,
        4
    );
}

#[test]
fn coherent_unison_level_does_not_jump_while_its_count_changes() {
    let (mut settings, pool) = user_sound();
    settings.oscillators[0].controls.detune = 0.0;
    settings.oscillators[0].controls.spread = 0.0;
    let mut single = Runtime::new(48_000, &settings, &pool).unwrap();
    let mut changing = Runtime::new(48_000, &settings, &pool).unwrap();
    for frame in 0..3000 {
        if frame == 500 {
            settings.oscillators[0].controls.unison = 8;
            changing.configure(settings.controls(), 0.0);
        }
        if frame == 2000 {
            settings.oscillators[0].controls.unison = 1;
            changing.configure(settings.controls(), 0.0);
        }
        single.begin_frame(true);
        changing.begin_frame(true);
        let a = single.render(1, 1, 60.0, &[0.0; 8]);
        let b = changing.render(1, 1, 60.0, &[0.0; 8]);
        assert!((a[0] - b[0]).abs() < 2e-6, "frame {frame}: {a:?} != {b:?}");
    }
}

#[test]
fn prepared_replacement_retains_the_voice_and_removed_sources_retire_off_audio() {
    let mut engine = engine();
    note(&mut engine, 60);
    for _ in 0..500 {
        engine.process_frame();
    }
    let mut preset = engine.preset.clone();
    preset.wavetable.as_mut().unwrap().oscillators[0].asset_id = "factory:wt-vowel-sweep:v1".into();
    let mut prepared = preset.prepared_clone_for(48_000);
    assert!(prepared.wavetable_runtime.1.is_none());
    no_heap(|| {
        engine.exchange_preset(&mut prepared);
        for _ in 0..2000 {
            assert!(engine.process_frame().iter().all(|x| x.is_finite()));
        }
    });
    let mut removed = engine.preset.clone();
    removed
        .rack
        .as_mut()
        .unwrap()
        .modules
        .retain(|s| s.module != Module::Wavetable);
    let mut removed = removed.prepared_clone_for(48_000);
    no_heap(|| {
        engine.exchange_preset(&mut removed);
        for _ in 0..64 {
            engine.process_frame();
        }
    });
    assert!(engine.preset.wavetable.is_none());
}

#[test]
fn surf_keys_step_and_xy_are_per_note_and_do_not_allocate() {
    let (mut s, pool) = user_sound();
    s.count = 4;
    for (i, o) in s.oscillators.iter_mut().enumerate() {
        o.controls.enabled = true;
        o.controls.position = 0.5;
        o.controls.surf_depth = if i % 2 == 0 { 1.0 } else { -1.0 };
        o.controls.key_waves = 0.5;
    }
    let mut runtime = Runtime::new(48_000, &s, &pool).unwrap();
    let mut motion = sad_pet::wavetable::motion::Motion {
        surf: 0.2,
        ..Default::default()
    };
    no_heap(|| {
        runtime.configure_motion(motion);
        for _ in 0..1000 {
            runtime.begin_frame(true);
            runtime.render(1, 1, 60.0, &[0.0; 8]);
            runtime.render(2, 1, 84.0, &[0.0; 8]);
        }
    });
    let p = runtime.positions();
    assert!((p[0].min - 0.7).abs() < 1e-5);
    assert!((p[0].max - 0.9).abs() < 1e-5);
    assert!((p[1].min - 0.3).abs() < 1e-5);
    let mut c = s.controls();
    c[0].step = true;
    no_heap(|| {
        runtime.configure(c, 0.0);
        for _ in 0..2000 {
            runtime.begin_frame(true);
            runtime.render(1, 1, 60.0, &[0.0; 8]);
        }
    });
    assert!((runtime.positions()[0].current - 1.0).abs() < 1e-5);
    motion.xy_on = true;
    motion.x = 0.8;
    motion.y = 0.6;
    no_heap(|| {
        runtime.configure_motion(motion);
        for _ in 0..1000 {
            runtime.begin_frame(true);
            assert!(
                runtime
                    .render(1, 1, 60.0, &[0.0; 8])
                    .iter()
                    .all(|v| v.is_finite())
            );
        }
    });
}

#[test]
fn shared_wave_env_and_poly_lfo_follow_each_held_note() {
    let mut p = Kind::Wavetable.preset().unwrap();
    p.wavetable.as_mut().unwrap().oscillators[0]
        .controls
        .position_env = 2;
    p.wavetable.as_mut().unwrap().oscillators[0]
        .controls
        .position_env_depth = 1.0;
    p.envs[1].wave = sad_pet::envelope::Wave {
        enabled: true,
        times: [1.0; 6],
        levels: [1.0, 0.0, 1.0, 0.0, 1.0, 0.0],
        loop_on: true,
        ..Default::default()
    };
    let mut e = Engine::new(48_000, sample(), p);
    e.set_midi_gate(true);
    e.set_playing(true);
    no_heap(|| {
        note(&mut e, 60);
        for _ in 0..9600 {
            e.process_frame();
        }
        note(&mut e, 72);
        for _ in 0..4800 {
            e.process_frame();
        }
    });
    let view = e.wavetable_positions()[0];
    assert!((view.max - view.min - 0.2).abs() < 0.02, "{view:?}");
    let changes=sad_pet::parameters::parse(&serde_json::json!({"env2_wave_on":0,"wt_a_env_depth":0,"wt_a_lfo":1,"wt_a_lfo_depth":0.5,"wt_a_position":0.5,"lfo1_poly":1})).unwrap();
    no_heap(|| e.apply_parameters(&changes));
    // Per-note phases reset separately through the common LFO controller.
    let mut next = e.preset.clone();
    next.lfos[0].waveform = sad_pet::preset::Waveform::Saw;
    next.lfos[0].rate_hz = 1.0;
    next.lfos[0].reset = sad_pet::preset::LfoReset::Note;
    let mut prepared = next.prepared_clone_for(48_000);
    no_heap(|| {
        e.exchange_preset(&mut prepared);
    });
    no_heap(|| {
        note(&mut e, 48);
        for _ in 0..9600 {
            e.process_frame();
        }
        note(&mut e, 55);
        for _ in 0..4800 {
            e.process_frame();
        }
    });
    let view = e.wavetable_positions()[0];
    assert!(view.max - view.min > 0.15, "{view:?}");
    let recalled: Preset = serde_json::from_slice(&serde_json::to_vec(&e.preset).unwrap()).unwrap();
    assert!(recalled.lfos[0].poly);
    assert_eq!(recalled.envs[1].wave, e.preset.envs[1].wave);
}

#[test]
fn live_fm_pm_sync_and_topology_changes_remain_finite_without_heap() {
    let (mut s, pool) = user_sound();
    s.count = 4;
    for o in &mut s.oscillators {
        o.controls.enabled = true;
        o.controls.level = 0.3;
    }
    s.oscillators[0].controls.fm_mode = 1;
    s.oscillators[0].controls.fm_index = 2.0;
    s.oscillators[2].controls.sync = true;
    s.oscillators[2].controls.sync_ratio = 2.75;
    let mut runtime = Runtime::new(48_000, &s, &pool).unwrap();
    let mut peak = 0.0_f32;
    no_heap(|| {
        for mode in [1, 2, 0] {
            let mut controls = s.controls();
            controls[0].fm_mode = mode;
            controls[2].sync_ratio = 3.25;
            runtime.configure(controls, 0.0);
            for i in 0..4000 {
                runtime.begin_frame(true);
                let y = runtime.render(1, if i < 2000 { 1 } else { 2 }, 69.0, &[0.0; 8]);
                assert!(y.iter().all(|v| v.is_finite()));
                peak = peak.max(y[0].abs());
            }
        }
    });
    assert!(peak > 0.01 && peak < 1.5, "{peak}");
}

#[test]
fn zero_depth_and_unavailable_operator_keep_the_clean_phase_running() {
    for count in [1, 2, 4] {
        let mut clean = Settings::default();
        clean.count = count;
        for o in clean.oscillators.iter_mut().take(count as usize) {
            o.controls.enabled = true;
            o.controls.level = 0.1;
        }
        let mut configured = clean.clone();
        if count == 4 {
            configured.oscillators[1].controls.fm_mode = 2;
            configured.oscillators[1].controls.fm_index = 0.0;
            configured.oscillators[1].controls.fm_source = 3;
        } else {
            configured.oscillators[0].controls.fm_mode = 1;
            configured.oscillators[0].controls.fm_index = 2.0;
            configured.oscillators[0].controls.fm_source = 3;
            configured.oscillators[0].controls.sync = true;
            configured.oscillators[0].controls.sync_source = 3;
        }
        let pool = Pool::default();
        let mut a = Runtime::new(48000, &clean, &pool).unwrap();
        let mut b = Runtime::new(48000, &configured, &pool).unwrap();
        no_heap(|| {
            for _ in 0..5000 {
                a.begin_frame(true);
                b.begin_frame(true);
                assert_eq!(
                    a.render(1, 1, 60.0, &[0.0; 8]),
                    b.render(1, 1, 60.0, &[0.0; 8]),
                    "inactive interaction changed the phase trajectory, count={count}"
                );
            }
        });
    }
    let mut clean = Settings::default();
    clean.count = 4;
    for o in &mut clean.oscillators {
        o.controls.enabled = true;
        o.controls.level = 0.1;
    }
    clean.oscillators[3].controls.enabled = false;
    let mut configured = clean.clone();
    configured.oscillators[0].controls.fm_mode = 1;
    configured.oscillators[0].controls.fm_index = 2.0;
    configured.oscillators[0].controls.fm_source = 3;
    let pool = Pool::default();
    let mut a = Runtime::new(48000, &clean, &pool).unwrap();
    let mut b = Runtime::new(48000, &configured, &pool).unwrap();
    no_heap(|| {
        for _ in 0..5000 {
            a.begin_frame(true);
            b.begin_frame(true);
            assert_eq!(
                a.render(1, 1, 60.0, &[0.0; 8]),
                b.render(1, 1, 60.0, &[0.0; 8]),
                "a disabled D operator was replaced by B"
            );
        }
    });
}

#[test]
fn routed_feedback_is_independent_of_operator_order_and_sources_change_the_sound() {
    let mut s = Settings::default();
    s.count = 4;
    for (i, o) in s.oscillators.iter_mut().enumerate() {
        o.controls.enabled = true;
        o.controls.level = 0.1;
        o.controls.transpose = [0.0, 12.0, 7.0, 19.0][i];
        o.controls.position = 0.2 + i as f32 * 0.15;
        o.controls.fm_mode = (i % 2 + 1) as u8;
        o.controls.fm_index = 0.7;
        o.controls.fm_source = ((i + 3) % 4) as u8;
        o.controls.sync = true;
        o.controls.sync_ratio = 2.75;
        o.controls.sync_source = ((i + 1) % 4) as u8;
    }
    let pool = Pool::default();
    let render = |settings: &Settings| {
        let mut runtime = Runtime::new(48000, settings, &pool).unwrap();
        let mut output = vec![0.0; 6000];
        no_heap(|| {
            for x in &mut output {
                runtime.begin_frame(true);
                *x = runtime.render(1, 1, 60.0, &[0.0; 8])[0];
                assert!(x.is_finite());
            }
        });
        output
    };
    let reference = render(&s);
    assert!(
        reference[1200..].iter().map(|v| v * v).sum::<f32>() / 4800.0 > 1e-6,
        "reciprocal sync lost its sustained sound after the transition"
    );
    let mut reordered = s.clone();
    let permutation = [3, 1, 2, 0];
    for i in 0..4 {
        reordered.oscillators[i] = s.oscillators[permutation[i]].clone();
        let c = &mut reordered.oscillators[i].controls;
        c.fm_source = permutation[c.fm_source as usize] as u8;
        c.sync_source = permutation[c.sync_source as usize] as u8;
    }
    let moved = render(&reordered);
    let startup = reference[..1200]
        .iter()
        .zip(&moved[..1200])
        .map(|(a, b)| (a - b).abs())
        .fold(0.0_f32, f32::max);
    let worst = reference[1200..]
        .iter()
        .zip(&moved[1200..])
        .map(|(a, b)| (a - b).abs())
        .fold(0.0_f32, f32::max);
    assert!(
        worst < 1e-6,
        "operator order changed feedback by {worst}, startup {startup}"
    );
    for sync in [false, true] {
        let mut a = s.clone();
        for o in &mut a.oscillators {
            o.controls.fm_mode = 0;
            o.controls.sync = false;
            o.controls.level = 0.0;
        }
        let carrier = &mut a.oscillators[1].controls;
        carrier.level = 0.2;
        carrier.fm_mode = if sync { 0 } else { 1 };
        carrier.sync = sync;
        carrier.fm_source = 0;
        carrier.sync_source = 0;
        let mut b = a.clone();
        b.oscillators[1].controls.fm_source = 3;
        b.oscillators[1].controls.sync_source = 3;
        let aa = render(&a);
        let bb = render(&b);
        let difference = aa
            .iter()
            .zip(&bb)
            .map(|(a, b)| (a - b) * (a - b))
            .sum::<f32>()
            / aa.len() as f32;
        assert!(
            difference.sqrt() > 1e-3,
            "source selection has no audible effect, sync={sync}"
        );
    }
}

#[test]
fn shared_capture_is_bounded_and_never_allocates_in_its_writer() {
    let (shared, mut writer) = sad_pet::capture::Shared::prepare(48000);
    no_heap(|| {
        shared.arm(8000, 0.05, false).unwrap();
        for n in 0..1000 {
            writer.push([0.9; 2], [n as f32 / 1000.0, 0.0]);
        }
    });
    assert_eq!(shared.view()["frames"], 400);
    assert_eq!(shared.view()["ready"], true);
    let (rate, frames) = shared.snapshot().unwrap();
    assert_eq!(rate, 8000);
    assert_eq!(frames.len(), 400);
    assert_eq!(frames[399], [0.399, 0.0]);
    let generation = shared.view()["generation"].as_u64().unwrap() as u32;
    let mut chunk = [0_u8; 32768];
    no_heap(|| {
        assert_eq!(shared.read_chunk(generation, 0, &mut chunk).unwrap(), 3200);
        assert!(shared.read_chunk(generation + 1, 0, &mut chunk).is_err());
    });
    no_heap(|| {
        shared.arm(48000, 0.1, true).unwrap();
        for _ in 0..120 {
            writer.push([0.25, -0.25], [0.0; 2]);
        }
        shared.stop();
        writer.push([0.0; 2], [0.0; 2]);
    });
    assert_eq!(shared.snapshot().unwrap().1, vec![[0.25, -0.25]; 120]);
}

#[test]
fn split_layers_and_velocity_ranges_keep_note_off_and_sustain_routing() {
    use sad_pet::timbres::Mix;
    let bass = Mix {
        key_high: 59,
        ..Mix::default()
    };
    let lead = Mix {
        key_low: 60,
        velocity_low: 64,
        channel: 2,
        ..Mix::default()
    };
    let on = |channel, note, velocity| Message::NoteOn {
        channel,
        note,
        velocity,
    };
    assert!(bass.accepts(on(0, 59, 100)));
    assert!(!bass.accepts(on(0, 60, 100)));
    assert!(lead.accepts(on(1, 60, 100)));
    assert!(!lead.accepts(on(0, 60, 100)));
    assert!(!lead.accepts(on(1, 60, 63)));
    assert!(lead.accepts(Message::NoteOff {
        channel: 1,
        note: 40,
        velocity: 0
    }));
    assert!(lead.accepts(on(1, 40, 0)));
    assert!(lead.accepts(Message::Control {
        channel: 1,
        controller: 64,
        value: 0
    }));
    let old: Mix = serde_json::from_str("{}").unwrap();
    assert!(old.accepts(on(0, 0, 1)));
    assert!(old.accepts(on(15, 127, 127)));
    let restored: Mix = serde_json::from_value(serde_json::to_value(lead).unwrap()).unwrap();
    assert_eq!(restored.key_low, 60);
    assert_eq!(restored.velocity_low, 64);
}

#[test]
fn motion_patterns_recall_and_filter_recipes_use_shared_rack_components() {
    let mut p = Kind::Wavetable.preset().unwrap();
    p.wavetable.as_mut().unwrap().oscillators[0]
        .controls
        .position = 0.5;
    let patterns = sad_pet::sequence::motion::factory();
    assert_eq!(patterns.len(), 8);
    p.sequencer.patterns = patterns[..2].to_vec();
    p.sequencer.tracks = patterns[0].tracks.clone();
    p.sequencer.enabled = true;
    let recalled: Preset = serde_json::from_value(serde_json::to_value(&p).unwrap()).unwrap();
    assert_eq!(recalled.sequencer.patterns.len(), 2);
    assert_eq!(recalled.sequencer.patterns[0].name, "Slow Surf");
    let mut e = Engine::new(48000, sample(), recalled);
    e.set_playing(true);
    e.set_midi_gate(true);
    note(&mut e, 60);
    let mut low = 1.0_f32;
    let mut high = 0.0_f32;
    no_heap(|| {
        for i in 0..96000 {
            e.process_frame();
            if i > 4800 {
                let p = e.wavetable_positions()[0].current;
                low = low.min(p);
                high = high.max(p);
            }
        }
    });
    assert!(
        high - low > 0.05,
        "motion did not move position: {low}..{high}"
    );
    for recipe in sad_pet::filter_presets::factory() {
        let mut p = Kind::Wavetable.preset().unwrap();
        let mut params = serde_json::to_value(p.params).unwrap();
        for (key, value) in recipe["params"].as_object().unwrap() {
            params[key] = value.clone();
        }
        p.params = serde_json::from_value(params).unwrap();
        let mut e = Engine::new(48000, sample(), p);
        e.set_midi_gate(true);
        e.set_playing(true);
        note(&mut e, 60);
        let mut energy = 0.0;
        no_heap(|| {
            for _ in 0..4800 {
                let x = e.process_frame();
                assert!(x.iter().all(|v| v.is_finite()));
                energy += x[0] * x[0];
            }
        });
        assert!(energy > 1e-6, "silent recipe {}", recipe["name"]);
    }
}
