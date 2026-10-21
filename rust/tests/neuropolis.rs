use metropolix_core::sequencer::{Sequencer, field};
use sad_pet::{
    dsp::{Engine, SampleBuffer, telemetry::RealtimeView},
    preset::Preset,
    sequence::{Edit, Stage, Target, Track},
};
use std::{
    alloc::{GlobalAlloc, Layout, System},
    cell::Cell,
    sync::Arc,
};

struct Allocator;
thread_local! {static TRACK:Cell<bool>=const{Cell::new(false)};static COUNT:Cell<usize>=const{Cell::new(0)};}
fn count() {
    let _ = TRACK.try_with(|t| {
        if t.get() {
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
    assert_eq!(COUNT.with(Cell::get), 0, "audio allocated or freed");
}
fn engine(p: Preset) -> Engine {
    Engine::new(8000, Arc::new(SampleBuffer::demo(8000)), p)
}
fn frames(e: &mut Engine, n: usize) {
    for _ in 0..n {
        let s = e.process_frame();
        assert!(s.into_iter().all(f32::is_finite));
    }
}
fn view(e: &mut Engine) -> RealtimeView {
    let mut v = RealtimeView::default();
    e.realtime_view(&mut v);
    v
}
fn track(id: u32, division: i32, target: Target) -> Track {
    let mut t = Track {
        id,
        target,
        ..Track::default()
    };
    t.config[field::DIVISION] = division;
    for (i, s) in t.steps.iter_mut().enumerate() {
        s.pitch = 60 + i as u8;
        s.cv = (i as i32 - 8) * 500;
    }
    t
}

#[test]
fn three_step_destinations_reach_filters_oscillators_and_fx_with_sparse_realtime_edits() {
    use sad_pet::{color::ColorSlot, preset::SaturatorType};
    let mut p = Preset::default();
    p.params.filter = 0.5;
    p.resynth.oscillators[3].control1 = 0.5;
    p.pre_color_chain = vec![ColorSlot {
        id: 71,
        kind: SaturatorType::LcrDelay,
        ..ColorSlot::default()
    }];
    p.sequencer.enabled = true;
    let t = &mut p.sequencer.tracks[0];
    t.target = Target::Notes;
    t.destinations = [
        Some("filter".into()),
        Some("resynth_osc4_control1".into()),
        Some("pre_fx1_lcr_ratio".into()),
    ];
    for s in &mut t.steps {
        s.destination1 = 0.2;
        s.destination2 = -0.3;
        s.destination3 = 0.1;
    }
    let saved = serde_json::to_value(&p).unwrap();
    let mut e = engine(serde_json::from_value(saved.clone()).unwrap());
    let mut feedback = RealtimeView::default();
    no_heap(|| {
        e.set_playing(true);
        frames(&mut e, 64);
        e.realtime_view(&mut feedback);
    });
    assert!((feedback.params.filter - 0.7).abs() < 1e-5);
    assert!((feedback.modulated_resynth.oscillators[3].control1 - 0.2).abs() < 1e-5);
    assert!((feedback.pre_color[0].settings.lcr_ratio - (64.0 / 127.0 + 0.1)).abs() < 1e-5);
    let stage = e.sequence_view().tracks[0].step;
    let edits = sad_pet::sequence::parse_edits(&serde_json::json!([
        {"track":1,"stage":0,"field":16,"value":-0.15},
        {"track":1,"stage":0,"field":17,"value":0.25},
        {"track":1,"stage":0,"field":18,"value":-0.2},
    ]))
    .unwrap();
    no_heap(|| {
        e.apply_sequence_edits(&edits);
        frames(&mut e, 64);
        e.realtime_view(&mut feedback);
    });
    assert_eq!(e.sequence_view().tracks[0].step, stage);
    assert!((feedback.params.filter - 0.35).abs() < 1e-5);
    assert!((feedback.modulated_resynth.oscillators[3].control1 - 0.75).abs() < 1e-5);
    assert!((feedback.pre_color[0].settings.lcr_ratio - (64.0 / 127.0 - 0.2)).abs() < 1e-5);
    assert_eq!(
        e.preset.params.filter, 0.5,
        "The route changed the saved base value"
    );
    let mut muted = e.preset.clone();
    muted.sequencer.tracks[0].config[field::MUTE] = 1;
    e.set_preset(muted);
    frames(&mut e, 64);
    assert_eq!(e.modulated_params().filter, 0.5);

    let mut old = saved;
    let t = old["sequencer"]["tracks"][0].as_object_mut().unwrap();
    t.remove("destinations");
    for s in t["steps"].as_array_mut().unwrap() {
        for key in ["destination1", "destination2", "destination3"] {
            s.as_object_mut().unwrap().remove(key);
        }
    }
    let old: Preset = serde_json::from_value(old).unwrap();
    assert_eq!(old.sequencer.tracks[0].destinations, [None, None, None]);
    assert_eq!(old.sequencer.tracks[0].steps[0].destination1, 0.0);
}

#[test]
fn real_neuropolis_orders_pulses_probability_and_independent_clocks_match_upstream() {
    for order in 0..21 {
        let mut p = Preset::default();
        p.sequencer.enabled = true;
        p.sequencer.tracks = vec![
            track(1, 1, Target::MainPitch),
            track(2, 65, Target::Oscillator { index: 3 }),
        ];
        for t in &mut p.sequencer.tracks {
            t.config[field::ORDER] = order;
            t.config[field::SWING] = 62;
            t.config[field::LENGTH] = 7;
            for (i, s) in t.steps.iter_mut().enumerate() {
                s.pulses = (i % 3 + 1) as u8;
                s.probability = 73;
                s.ratchet = (i % 2 + 1) as u8;
                s.skip = i == 2;
            }
        }
        let mut reference = Sequencer::default();
        reference.resize_tracks(2);
        // Disable unused upstream mod lanes, just as the integration does.
        for m in &mut reference.mods {
            m.config[9] = 1;
        }
        for (i, t) in p.sequencer.tracks.iter().enumerate() {
            for (f, v) in t.config.iter().enumerate() {
                reference.set_track(i, f as u32, *v);
            }
            for (s, step) in t.steps.iter().enumerate() {
                for f in 0..15 {
                    reference.set_step(i, s, f as u32, step.get(f) as i32);
                }
            }
        }
        reference.restart();
        let mut e = engine(p);
        e.set_playing(true);
        let mut ticks = 0;
        for sample in 0..16000 {
            // The first tick starts at sample zero; all subsequent ticks have
            // the same sample-clock boundary as the shared 96 PPQN transport.
            let due = (sample as f64 * 120.0 * 96.0 / (60.0 * 8000.0) + 1e-9).floor() as u64 + 1;
            while ticks < due {
                reference.tick();
                ticks += 1;
            }
            e.process_frame();
            if sample % 317 == 0 {
                let v = e.sequence_view();
                for i in 0..2 {
                    assert_eq!(
                        v.tracks[i].step,
                        reference.stage(i).unwrap(),
                        "order {order}, sample {sample}, track {i}"
                    );
                    assert_eq!(v.tracks[i].pitch, reference.cents(i) as f32 / 100.0);
                    assert_eq!(v.tracks[i].gate, reference.gate(i));
                }
            }
        }
    }
}

#[test]
fn destinations_change_working_sound_parameters_without_overwriting_the_patch() {
    let mut p = Preset::default();
    p.sequencer.enabled = true;
    p.resynth.enabled = true;
    p.resynth.count = 4;
    p.params.filter = 0.5;
    let mut main = track(1, 1, Target::MainPitch);
    main.steps.fill(Stage {
        pitch: 72,
        ..Stage::default()
    });
    let mut osc = track(2, 2, Target::Oscillator { index: 3 });
    osc.steps.fill(Stage {
        pitch: 67,
        ..Stage::default()
    });
    let mut param = track(
        3,
        65,
        Target::Parameter {
            parameter: "filter".into(),
        },
    );
    param.amount = 0.25;
    param.steps.fill(Stage {
        cv: -2500,
        ..Stage::default()
    });
    p.sequencer.tracks = vec![main, osc, param];
    let base = p.params.filter;
    let mut e = engine(p);
    e.set_playing(true);
    frames(&mut e, 50);
    let v = view(&mut e);
    assert_eq!(v.params.sample_tune_semis, 12.0);
    assert!((v.params.filter - (base - 0.125)).abs() < 1e-5);
    assert_eq!(e.preset.params.sample_tune_semis, 0.0);
    assert_eq!(e.preset.params.filter, base);
    e.apply_sequence_edits(&[Edit {
        track: 3,
        stage: Some(0),
        field: 12,
        value: 5000.0,
    }]);
    frames(&mut e, 100);
    assert!(view(&mut e).params.filter > base);
    let ticks = e.sequence_view();
    e.apply_sequence_edits(&[Edit {
        track: 1,
        stage: Some(7),
        field: 0,
        value: 48.0,
    }]);
    assert_eq!(ticks.tracks[0].step, e.sequence_view().tracks[0].step);
    e.apply_sequence_edits(&[Edit {
        track: 0,
        stage: None,
        field: 0,
        value: 0.0,
    }]);
    assert_eq!(view(&mut e).params.sample_tune_semis, 0.0);
}

#[test]
fn unlimited_tracks_recall_and_id_based_adoption_do_not_allocate_or_reset_other_lanes() {
    let mut p = Preset::default();
    p.sequencer.enabled = true;
    p.sequencer.tracks = (1..=257)
        .map(|id| track(id, if id % 2 == 0 { 2 } else { 1 }, Target::MainPitch))
        .collect();
    let mut e = engine(p);
    e.set_playing(true);
    frames(&mut e, 2101);
    e.set_sequence_view(193);
    let before = e.sequence_view();
    assert_eq!(before.total, 257);
    assert_eq!(before.count, 64);
    let edits = [
        Edit {
            track: 220,
            stage: Some(14),
            field: 12,
            value: 1700.0,
        },
        Edit {
            track: 220,
            stage: None,
            field: 11,
            value: 67.0,
        },
        Edit {
            track: 220,
            stage: None,
            field: 49,
            value: 1.0,
        },
        Edit {
            track: 220,
            stage: Some(0),
            field: 9,
            value: 28.0,
        },
    ];
    e.apply_sequence_edits(&edits);
    let mut next = e.preset.clone();
    next.sequencer.tracks.remove(0);
    next.sequencer.tracks.reverse();
    next.sanitize();
    no_heap(|| {
        e.apply_sequence_edits(&edits);
        e.exchange_preset(&mut next);
        frames(&mut e, 100);
        e.realtime_view(&mut RealtimeView::default());
    });
    e.set_sequence_view(0);
    let after = e.sequence_view();
    assert_eq!(after.total, 256);
    for old in before.tracks {
        if old.id >= 194 {
            let new = after.tracks.iter().find(|t| t.id == old.id).unwrap();
            assert_eq!(old.step, new.step, "track {} lost phase", old.id);
        }
    }
    let encoded = serde_json::to_vec(&e.preset).unwrap();
    let mut restored: Preset = serde_json::from_slice(&encoded).unwrap();
    restored.sanitize();
    assert_eq!(restored.sequencer.tracks.len(), 256);
    assert_eq!(
        restored
            .sequencer
            .tracks
            .iter()
            .find(|t| t.id == 220)
            .unwrap()
            .config[11],
        67
    );
    let restored_track = restored
        .sequencer
        .tracks
        .iter()
        .find(|t| t.id == 220)
        .unwrap();
    assert!(restored_track.pitch_oscillators[0]);
    assert_eq!(restored_track.steps[0].gate_override, 28);
}

#[test]
fn step_gate_length_uses_the_original_core_and_oscillator_presets_use_signed_offsets() {
    let mut p = Preset::default();
    p.sequencer.enabled = true;
    let mut t = Track {
        target: Target::Notes,
        ..Track::default()
    };
    t.steps[0].gate_override = 20;
    t.steps[1].gate_override = 80;
    p.sequencer.tracks = vec![t];
    let mut e = engine(p);
    e.set_playing(true);
    let mut gates = [0; 2];
    for sample in 0..2000 {
        e.process_frame();
        gates[sample / 1000] += usize::from(e.sequence_view().tracks[0].gate);
    }
    assert!(gates[0] > 150 && gates[0] < 260, "20% gate: {}", gates[0]);
    assert!(gates[1] > 750 && gates[1] < 860, "80% gate: {}", gates[1]);

    let mut value = serde_json::to_value(Preset::default()).unwrap();
    value["sequencer"]["tracks"][0]
        .as_object_mut()
        .unwrap()
        .remove("pitch_oscillators");
    value["sequencer"]["tracks"][0]["target"] = serde_json::json!({"kind":"oscillator","index":3});
    value["sequencer"]["tracks"][0]["root_note"] = serde_json::json!(48);
    value["sequencer"]["tracks"][0]["steps"][0]["pitch"] = serde_json::json!(55);
    let mut p: Preset = serde_json::from_value(value).unwrap();
    p.sanitize();
    let t = &p.sequencer.tracks[0];
    assert_eq!(t.pitch_oscillators, [false; 4]);
    assert_eq!(t.root_note, 60);
    assert_eq!(
        t.steps[0].pitch, 67,
        "An old +7 st offset changed on recall"
    );
    let encoded = serde_json::to_vec(&p).unwrap();
    let mut recalled: Preset = serde_json::from_slice(&encoded).unwrap();
    recalled.sanitize();
    assert_eq!(recalled.sequencer.tracks[0].steps[0].pitch, 67);
    let mut old = serde_json::to_value(Preset::default()).unwrap();
    old["sequencer"]["enabled"] = serde_json::json!(true);
    old["sequencer"]["tracks"][0]["target"] = serde_json::json!({"kind":"oscillator","index":0});
    old["sequencer"]["tracks"][0]["root_note"] = serde_json::json!(54);
    old["sequencer"]["tracks"][0]["steps"][0]["pitch"] = serde_json::json!(55);
    let mut p: Preset = serde_json::from_value(old).unwrap();
    p.sanitize();
    assert_eq!(p.sequencer.tracks[0].config[field::ROOT], 6);
    let mut e = engine(p);
    e.set_playing(true);
    frames(&mut e, 1);
    assert_eq!(
        e.sequence_view().tracks[0].value,
        1.0,
        "A non-C root changed the quantized oscillator offset"
    );
}

#[test]
fn note_tracks_trigger_envelopes_and_do_not_release_an_external_note_with_the_same_pitch() {
    let mut p = Preset::default();
    p.sequencer.enabled = true;
    p.sequencer.tracks = vec![Track {
        target: Target::Notes,
        ..Track::default()
    }];
    p.sequencer.tracks[0].config[field::GATE] = 10;
    let mut e = engine(p);
    e.set_midi_gate(true);
    e.midi(sad_pet::midi::Message::NoteOn {
        channel: 0,
        note: 60,
        velocity: 100,
    });
    frames(&mut e, 32);
    let v = view(&mut e);
    assert_eq!(v.note_count, 2);
    assert!(v.envs[0] > 0.0);
    frames(&mut e, 150);
    let v = view(&mut e);
    assert_eq!(&v.notes[..v.note_count], &[60]);
    let mut next = e.preset.prepared_clone();
    next.sequencer.tracks.clear();
    e.exchange_preset(&mut next);
    assert_eq!(e.held_notes(), vec![60]);
}

#[test]
fn old_chord_patterns_migrate_into_neuropolis_note_lanes() {
    let mut value = serde_json::to_value(Preset::default()).unwrap();
    value.as_object_mut().unwrap().remove("sequencer");
    value["performance"]["mode"] = serde_json::json!("sequencer");
    value["performance"]["steps"][0]["notes"] = serde_json::json!([60, 64, 67]);
    let p: Preset = serde_json::from_value(value).unwrap();
    assert!(p.sequencer.enabled);
    assert_eq!(p.sequencer.tracks.len(), 3);
    assert_eq!(p.performance.mode, sad_pet::performance::Mode::Keys);
    assert_eq!(p.sequencer.tracks[2].steps[0].pitch, 67);
    assert_eq!(p.sequencer.tracks[2].steps[1].gate, 0);
}

#[test]
fn opposite_tracks_cancel_before_clamping_and_unipolar_absolute_cv_uses_the_full_range() {
    let mut p = Preset::default();
    p.params.filter = 0.7;
    p.sequencer.enabled = true;
    let mut a = track(
        1,
        1,
        Target::Parameter {
            parameter: "filter".into(),
        },
    );
    a.steps.fill(Stage {
        cv: 5000,
        ..Stage::default()
    });
    let mut b = a.clone();
    b.id = 2;
    b.amount = -1.0;
    p.sequencer.tracks = vec![a, b];
    let mut e = engine(p);
    e.set_playing(true);
    frames(&mut e, 10);
    assert!((view(&mut e).params.filter - 0.7).abs() < 1e-6);
    e.apply_sequence_edits(&[
        Edit {
            track: 2,
            stage: None,
            field: 8,
            value: 1.0,
        },
        Edit {
            track: 1,
            stage: None,
            field: 44,
            value: 1.0,
        },
        Edit {
            track: 1,
            stage: None,
            field: 32,
            value: 1.0,
        },
        Edit {
            track: 1,
            stage: Some(0),
            field: 12,
            value: 0.0,
        },
    ]);
    frames(&mut e, 100);
    assert_eq!(view(&mut e).params.filter, 0.0);
}

#[test]
fn main_pitch_steps_can_gate_envelopes_like_notes() {
    use sad_pet::matrix::Source;
    let mut p = Preset::default();
    p.sequencer.enabled = true;
    // 1/16 at 120 BPM is 1000 frames at 8 kHz; the 55% gate closes near 550.
    p.sequencer.tracks = vec![track(1, 1, Target::MainPitch)];
    for env in &mut p.envs {
        env.release = 0.0;
    }
    let old = p.clone();
    p.sequencer.tracks[0].env_gate = true;
    let mut e = engine(p);
    e.set_playing(true);
    let mut starts = [0; 3];
    let mut closed = [0; 3];
    let mut previous = [0.0; 3];
    no_heap(|| {
        for frame in 0..4000 {
            e.process_frame();
            for env in 0..3 {
                let value = e.env_values[env];
                starts[env] += usize::from(previous[env] == 0.0 && value > 0.0);
                closed[env] += usize::from(frame % 1000 > 700 && value == 0.0);
                previous[env] = value;
            }
            let gate = e.modulation_source(Source::Gate);
            match frame % 1000 {
                0..500 => assert_eq!(gate, 1.0, "frame {frame}"),
                701.. => assert_eq!(gate, 0.0, "frame {frame}"),
                _ => {}
            }
        }
    });
    assert_eq!(starts, [4; 3], "ENV1-3 restart on every step");
    assert_eq!(closed, [4 * 299; 3], "ENV1-3 release after each gate");

    // Switching the gate off reopens the held voice without waiting a step.
    frames(&mut e, 800);
    assert_eq!(e.env_values[0], 0.0);
    let edit = |value| Edit {
        track: 1,
        stage: None,
        field: 53,
        value,
    };
    e.apply_sequence_edits(&[edit(0.0)]);
    assert!(!e.preset.sequencer.tracks[0].env_gate);
    frames(&mut e, 40);
    assert!(e.env_values[0] > 0.5);
    frames(&mut e, 1000);
    assert!(e.env_values[0] > 0.5, "The free voice was gated while off");
    e.apply_sequence_edits(&[edit(1.0)]);
    frames(&mut e, 1000);
    assert_eq!(e.env_values[0], 0.0, "The gate did not resume");

    // Without the option, Main pitch keeps the free voice droning as before.
    let mut e = engine(old);
    e.set_playing(true);
    frames(&mut e, 100);
    for _ in 0..4000 {
        e.process_frame();
        assert!(e.env_values[0] > 0.5);
        assert_eq!(e.modulation_source(Source::Gate), 1.0);
    }

    let mut value = serde_json::to_value(Preset::default()).unwrap();
    let saved = value["sequencer"]["tracks"][0].as_object_mut().unwrap();
    assert_eq!(saved.remove("env_gate"), Some(serde_json::json!(false)));
    let p: Preset = serde_json::from_value(value).unwrap();
    assert!(!p.sequencer.tracks[0].env_gate);
    let json = serde_json::json!([{"track":1,"field":53,"value":1}]);
    assert_eq!(sad_pet::sequence::parse_edits(&json).unwrap()[0].field, 53);
    let json = serde_json::json!([{"track":1,"field":54,"value":1}]);
    assert!(sad_pet::sequence::parse_edits(&json).is_err());
}
