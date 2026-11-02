use sad_pet::{
    dsp::{Engine, SampleBuffer},
    interaction::{self, Event, Node, Route, Scope, SyncMode},
    midi::Message,
    oscillators::{Kind, Settings},
    preset::Preset,
    rack::{Module, Slot},
};
use std::sync::Arc;
use std::{
    alloc::{GlobalAlloc, Layout, System},
    cell::Cell,
};
struct Allocator;
thread_local! {static TRACK:Cell<bool>=const{Cell::new(false)};static HEAP:Cell<usize>=const{Cell::new(0)};}
fn heap() {
    let _ = TRACK.try_with(|t| {
        if t.get() {
            let _ = HEAP.try_with(|n| n.set(n.get() + 1));
        }
    });
}
unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        heap();
        unsafe { System.alloc(l) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        heap();
        unsafe { System.dealloc(p, l) }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        heap();
        unsafe { System.realloc(p, l, n) }
    }
}
#[global_allocator]
static ALLOCATOR: Allocator = Allocator;
fn no_heap(f: impl FnOnce()) {
    HEAP.with(|n| n.set(0));
    TRACK.with(|t| t.set(true));
    f();
    TRACK.with(|t| t.set(false));
    assert_eq!(HEAP.with(Cell::get), 0, "heap operation in audio callback");
}

fn sample() -> Arc<SampleBuffer> {
    Arc::new(SampleBuffer::demo(48000))
}
fn patch() -> Preset {
    let mut p = sad_pet::timbres::Kind::Oscillators.preset().unwrap();
    p.params.filter = 1.0;
    p.params.filter2 = 1.0;
    p.color_chain.clear();
    p.pre_color_chain.clear();
    p.params.in_gain = 0.0;
    p.params.out_gain = 0.0;
    p.params.master_volume = 0.5;
    p.envs[0].attack = 0.0;
    p.envs[0].decay = 0.0;
    p.envs[0].sustain = 1.0;
    p.envs[0].release = 0.1;
    p
}
fn play(p: Preset) -> Engine {
    let mut e = Engine::new(48000, sample(), p);
    e.set_midi_gate(true);
    e.set_playing(true);
    e.midi(Message::NoteOn {
        channel: 0,
        note: 60,
        velocity: 100,
    });
    e
}
fn audio(e: &mut Engine, n: usize) -> Vec<f32> {
    (0..n).map(|_| e.process_frame()[0]).collect()
}
fn energy(a: &[f32]) -> f64 {
    a.iter().map(|&x| (x as f64).powi(2)).sum::<f64>() / a.len() as f64
}

#[test]
fn eight_wavetables_are_real_slots_and_reorder_keeps_the_held_note() {
    let mut p = patch();
    let settings = p.oscillators.as_mut().unwrap();
    settings.slots[0].kind = Kind::Wavetable;
    for i in 1..8 {
        let id = settings.add(Kind::Wavetable).unwrap();
        assert_eq!(settings.index(id), Some(i));
    }
    for (i, s) in settings.slots.iter_mut().enumerate() {
        s.controls.transpose = i as f32 * 2.0;
        s.controls.level = 0.1;
    }
    let mut actual = play(p.clone());
    let mut reference = play(p);
    let before = audio(&mut actual, 1024);
    audio(&mut reference, 1024);
    assert!(energy(&before) > 1e-6);
    let mut next = actual.preset.clone();
    next.oscillators.as_mut().unwrap().move_slot(8, -1).unwrap();
    let mut next = next.prepared_clone_for(48000);
    actual.exchange_preset(&mut next);
    let a = audio(&mut actual, 1024);
    let b = audio(&mut reference, 1024);
    assert_eq!(
        a, b,
        "display order must not restart or retune a held voice"
    );
    for physical in 0..8 {
        let mut p = patch();
        let mut s = Settings::default();
        s.slots[0].id = 0;
        s.slots[physical] = sad_pet::oscillators::Slot {
            id: physical as u32 + 1,
            kind: Kind::Wavetable,
            ..Default::default()
        };
        p.oscillators = Some(Box::new(s));
        assert!(
            energy(&audio(&mut play(p), 1024)) > 1e-6,
            "WT slot {physical} is silent"
        );
    }
}

#[test]
fn selecting_no_capture_sources_keeps_the_whole_bank_audible_at_full_wet() {
    let mut direct = patch();
    direct.params.mix = 1.0;
    let mut granular = direct.clone();
    granular.rack.as_mut().unwrap().modules.insert(
        1,
        Slot {
            module: Module::Grains,
            ..Default::default()
        },
    );
    granular.grain_capture = interaction::Capture {
        mode: interaction::CaptureMode::Osc,
        slots: 0,
    };
    let a = audio(&mut play(direct), 4096);
    let b = audio(&mut play(granular), 4096);
    assert_eq!(
        a, b,
        "unselected oscillators bypass the entire granular branch"
    );
}

#[test]
fn cross_type_ring_changes_only_the_target_and_survives_scene_json() {
    let mut p = patch();
    let s = p.oscillators.as_mut().unwrap();
    let id = s.add(Kind::Wavetable).unwrap();
    s.slots[1].controls.transpose = 7.0;
    s.slots[1].controls.level = 0.0;
    let base = audio(&mut play(p.clone()), 2048);
    p.interactions.routes.push(Route {
        source: Node::Osc { id },
        target: Node::Osc { id: 1 },
        ring: 1.0,
        ..Default::default()
    });
    let wire = serde_json::to_vec(&p).unwrap();
    let restored: Preset = serde_json::from_slice(&wire).unwrap();
    assert_eq!(p.interactions, restored.interactions);
    let ring = audio(&mut play(restored), 2048);
    assert!(
        energy(&ring) > 1e-8,
        "zero-level modulation source must still run"
    );
    assert_ne!(base, ring);
}

#[test]
fn individual_groups_ring_each_other_without_chain_or_send() {
    let mut p = Preset::default();
    p.color_chain.clear();
    p.grain_groups.advanced = true;
    p.params.grains = 0.8;
    p.grain_groups.groups[0].attack = 1.0;
    p.grain_groups.groups[1].output.chain = 0.0;
    p.grain_groups.groups[1].output.send = 0.0;
    p.grain_groups.groups[1].level = 0.0;
    p.grain_groups.groups[1].pitch = 7.0;
    for i in 2..4 {
        p.grain_groups.groups[i].share = 0.0;
    }
    p.interactions.routes.push(Route {
        source: Node::Group { index: 1 },
        target: Node::Group { index: 0 },
        ring: 1.0,
        ..Default::default()
    });
    let mut e = play(p);
    let out = audio(&mut e, 12000);
    assert!(out.iter().all(|x| x.is_finite()));
    assert!(energy(&out[6000..]) > 1e-10);
}

#[test]
fn group_playback_and_clock_hard_sync_have_distinct_lifecycles() {
    let mut p = Preset::default();
    p.color_chain.clear();
    p.grain_groups.advanced = true;
    p.params.grains = 0.4;
    p.interactions.routes.push(Route {
        source: Node::Group { index: 0 },
        target: Node::Group { index: 1 },
        sync: SyncMode::Hard,
        event: Event::AudioZeroCross,
        scope: Scope::Playback,
        ..Default::default()
    });
    let mut playback = play(p.clone());
    let a = audio(&mut playback, 8192);
    p.grain_scheduler = interaction::Scheduler::Independent;
    p.interactions.routes[0].scope = Scope::Clock;
    let mut clock = play(p);
    let b = audio(&mut clock, 8192);
    assert!(a.iter().chain(&b).all(|x| x.is_finite()));
    assert_ne!(a, b);
}

#[test]
fn every_mixed_type_pair_rings_soft_and_hard_syncs_in_both_directions() {
    let tone = Arc::new(
        SampleBuffer::new(
            "Pair oracle".into(),
            48000,
            (0..4096)
                .map(|i| {
                    let phase = i as f64 / 96.0 * std::f64::consts::TAU;
                    [(phase.sin() * 0.7 + (phase * 3.0).sin() * 0.15) as f32; 2]
                })
                .collect(),
        )
        .unwrap(),
    );
    let render = |p: Preset| {
        let mut e = Engine::new(48000, tone.clone(), p);
        e.set_midi_gate(true);
        e.set_playing(true);
        e.midi(Message::NoteOn {
            channel: 0,
            note: 60,
            velocity: 100,
        });
        audio(&mut e, 1536)
    };
    for target in [Kind::Radias, Kind::Wavefolder, Kind::Pcm, Kind::Wavetable] {
        for source in [Kind::Radias, Kind::Wavefolder, Kind::Pcm, Kind::Wavetable] {
            let mut p = patch();
            let s = p.oscillators.as_mut().unwrap();
            s.slots[0].kind = target;
            s.slots[0].controls.transpose = 7.0;
            s.slots[0].controls.native.control1 = 0.95;
            let id = s.add(source).unwrap();
            s.slots[1].controls.level = 0.0;
            s.slots[1].controls.native.control1 = 0.95;
            let base = render(p.clone());
            assert!(energy(&base) > 1e-8, "silent {target:?}");
            for (ring, sync) in [
                (1.0, SyncMode::Off),
                (0.0, SyncMode::Soft),
                (0.0, SyncMode::Hard),
                (1.0, SyncMode::Soft),
                (1.0, SyncMode::Hard),
            ] {
                p.interactions.routes = vec![Route {
                    source: Node::Osc { id },
                    target: Node::Osc { id: 1 },
                    ring,
                    sync,
                    ..Default::default()
                }];
                let out = render(p.clone());
                assert!(out.iter().all(|v| v.is_finite()));
                assert_ne!(
                    out, base,
                    "{source:?} -> {target:?}, Ring {ring}, {sync:?} did nothing"
                );
                assert!(
                    energy(&out[512..]) > 1e-12,
                    "{source:?} -> {target:?} became silent"
                );
            }
        }
    }
}

#[test]
fn prepared_topology_noise_windows_cycles_groups_and_scalar_changes_do_not_touch_the_heap() {
    let mut p = patch();
    let s = p.oscillators.as_mut().unwrap();
    s.slots[0].controls.native.waveform = sad_pet::resynth::Waveform::Noise;
    let id = s.add(Kind::Wavetable).unwrap();
    s.slots[1].controls.level = 0.2;
    p.rack.as_mut().unwrap().modules.insert(
        1,
        Slot {
            module: Module::Grains,
            ..Default::default()
        },
    );
    p.grain_groups.advanced = true;
    p.params.grains = 0.65;
    p.interactions.routes = vec![
        Route {
            source: Node::Osc { id },
            target: Node::Osc { id: 1 },
            sync: SyncMode::Hard,
            ring: 0.5,
            ..Default::default()
        },
        Route {
            source: Node::Osc { id: 1 },
            target: Node::Osc { id },
            sync: SyncMode::Soft,
            ring: 0.3,
            ..Default::default()
        },
        Route {
            source: Node::Osc { id: 1 },
            target: Node::Group { index: 0 },
            sync: SyncMode::Hard,
            scope: Scope::Both,
            event: Event::AudioZeroCross,
            ..Default::default()
        },
        Route {
            source: Node::Group { index: 0 },
            target: Node::Group { index: 1 },
            ring: 0.5,
            ..Default::default()
        },
    ];
    let mut e = play(p);
    audio(&mut e, 1024);
    let mut next = e.preset.clone();
    next.oscillators
        .as_mut()
        .unwrap()
        .move_slot(id, -1)
        .unwrap();
    let mut prepared = next.prepared_clone_for(48000);
    let changes =
        sad_pet::parameters::parse(&serde_json::json!({"interaction1_ring":0.75})).unwrap();
    no_heap(|| {
        e.exchange_preset(&mut prepared);
        e.apply_parameters(&changes);
        e.midi(Message::NoteOn {
            channel: 0,
            note: 67,
            velocity: 100,
        });
        for _ in 0..4096 {
            assert!(e.process_frame().iter().all(|v| v.is_finite()));
        }
        e.midi(Message::NoteOff {
            channel: 0,
            note: 67,
            velocity: 0,
        });
    });
}

#[test]
fn unselected_oscillators_cannot_enter_capture_through_an_intermediate_resynth() {
    let mut direct = patch();
    direct.params.mix = 1.0;
    let bank = direct.oscillators.as_mut().unwrap();
    bank.slots[0].controls.level = 0.0;
    bank.add(Kind::Radias).unwrap();
    let mut routed = direct.clone();
    routed.params.grains = 0.8;
    let modules = &mut routed.rack.as_mut().unwrap().modules;
    modules.insert(
        1,
        Slot {
            module: Module::Resynth,
            ..Default::default()
        },
    );
    modules.insert(
        2,
        Slot {
            module: Module::Grains,
            ..Default::default()
        },
    );
    routed.resynth.enabled = true;
    routed.resynth.mix = 1.0;
    routed.resynth.threshold = 0.0;
    routed.grain_capture = interaction::Capture {
        mode: interaction::CaptureMode::Osc,
        slots: 1,
    };
    let a = audio(&mut play(direct), 12000);
    let b = audio(&mut play(routed), 12000);
    assert_eq!(
        a, b,
        "Resynth heard an oscillator outside the selected capture branch"
    );
}

#[test]
fn default_auto_sync_events_work_from_oscillators_to_groups_and_back() {
    let mut p = patch();
    p.params.grains = 0.8;
    p.params.pitch = 0.8;
    p.rack.as_mut().unwrap().modules.insert(
        1,
        Slot {
            module: Module::Grains,
            ..Default::default()
        },
    );
    p.grain_groups.advanced = true;
    p.grain_scheduler = interaction::Scheduler::Independent;
    for group in &mut p.grain_groups.groups {
        group.share = 0.0;
    }
    p.grain_groups.groups[0].share = 1.0;
    let plain = audio(&mut play(p.clone()), 12000);
    p.interactions.routes = vec![Route {
        source: Node::Osc { id: 1 },
        target: Node::Group { index: 0 },
        sync: SyncMode::Hard,
        ..Default::default()
    }];
    let into_group = audio(&mut play(p.clone()), 12000);
    assert_ne!(plain, into_group);
    p.interactions.routes = vec![Route {
        source: Node::Group { index: 0 },
        target: Node::Osc { id: 1 },
        sync: SyncMode::Hard,
        ..Default::default()
    }];
    let from_group = audio(&mut play(p), 12000);
    assert_ne!(plain, from_group);
    assert!(into_group.iter().chain(&from_group).all(|v| v.is_finite()));
}

#[test]
fn disabled_sync_on_noise_has_a_prepared_transport_for_daw_automation() {
    let mut p = patch();
    let bank = p.oscillators.as_mut().unwrap();
    bank.slots[0].controls.native.waveform = sad_pet::resynth::Waveform::Noise;
    let id = bank.add(Kind::Wavetable).unwrap();
    p.interactions.routes.push(Route {
        source: Node::Osc { id },
        target: Node::Osc { id: 1 },
        ..Default::default()
    });
    let mut e = play(p);
    audio(&mut e, 512);
    let change = sad_pet::parameters::parse(&serde_json::json!({"interaction1_sync":2})).unwrap();
    no_heap(|| {
        e.apply_parameters(&change);
        for _ in 0..4096 {
            assert!(e.process_frame().iter().all(|v| v.is_finite()));
        }
    });
}
