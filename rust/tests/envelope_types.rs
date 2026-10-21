use radias_synth_domain::q_envelope::TIME_TICKS;
use sad_pet::{
    dsp::{Engine, SampleBuffer, modulation::Envelope},
    midi::Message,
    parameters,
    preset::{Env, EnvMode, Preset},
};
use std::{
    alloc::{GlobalAlloc, Layout, System},
    cell::Cell,
    sync::Arc,
};

struct Allocator;
thread_local! {static WATCH:Cell<bool>=const{Cell::new(false)};static EVENTS:Cell<usize>=const{Cell::new(0)};}
fn event() {
    let _ = WATCH.try_with(|w| {
        if w.get() {
            let _ = EVENTS.try_with(|n| n.set(n.get() + 1));
        }
    });
}
unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        event();
        unsafe { System.alloc(l) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        event();
        unsafe { System.dealloc(p, l) }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        event();
        unsafe { System.realloc(p, l, n) }
    }
}
#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

const RATE: f32 = 48_000.0;

/// A 0..1 control that maps exactly onto the Rustias 0..=127 value.
fn control(value: u8) -> f32 {
    f32::from(value) / 127.0
}
/// Rustias Q time in samples at `RATE`.
fn q_samples(value: u8) -> usize {
    TIME_TICKS[value as usize] as usize * 24
}
fn q_level(value: u8) -> f32 {
    (u32::from(value) * 32766 / 127) as f32 / 32766.0
}
fn radias_level(value: u8) -> f32 {
    f32::from((u16::from(value) << 8) | (u16::from(value) << 1)) / 32766.0
}
fn envelope(p: &Env) -> Envelope {
    let mut e = Envelope::default();
    e.configure(RATE, p);
    e
}
fn run(e: &mut Envelope, samples: usize) -> Vec<f32> {
    (0..samples).map(|_| e.next_sample()).collect()
}
fn peak(values: &[f32]) -> f32 {
    values.iter().copied().fold(0.0, f32::max)
}

#[test]
fn old_presets_load_the_original_envelope_and_types_roundtrip() {
    let mut value = serde_json::to_value(Preset::default()).unwrap();
    for env in value["envs"].as_array_mut().unwrap() {
        let env = env.as_object_mut().unwrap();
        for key in ["mode", "attack_level", "decay2", "sustain2", "curve"] {
            assert!(env.remove(key).is_some());
        }
    }
    let p: Preset = serde_json::from_value(value).unwrap();
    assert!(p.envs.iter().all(|e| e.mode == EnvMode::Legacy));
    assert_eq!(p.envs[0], Preset::default().envs[0]);

    let mut p = Preset::default();
    p.envs[2] = Env {
        mode: EnvMode::QLoopS1s2,
        attack_level: 0.7,
        decay2: 0.25,
        sustain2: 0.125,
        curve: 6,
        ..p.envs[2]
    };
    let json = serde_json::to_value(&p).unwrap();
    assert_eq!(json["envs"][2]["mode"], "q_loop_s1s2");
    assert_eq!(json["envs"][0]["mode"], "legacy");
    let copy: Preset = serde_json::from_value(json).unwrap();
    assert_eq!(copy.envs, p.envs);

    let mut broken = p.clone();
    broken.envs[1].curve = 99;
    broken.envs[1].sustain2 = f32::NAN;
    broken.sanitize();
    assert_eq!(broken.envs[1].curve, 7);
    assert_eq!(broken.envs[1].sustain2, Env::default().sustain2);
}

#[test]
fn every_rustias_mode_has_a_stable_name() {
    let names: Vec<_> = EnvMode::ALL
        .iter()
        .map(|m| serde_json::to_value(m).unwrap())
        .collect();
    assert_eq!(
        names,
        [
            "legacy",
            "radias",
            "q_adsr",
            "q_ads1ds2r",
            "q_one_shot",
            "q_loop_s1s2",
            "q_loop_all"
        ]
    );
    assert_eq!(EnvMode::Legacy.rustias_mode(), None);
    assert_eq!(EnvMode::Radias.rustias_mode(), Some(0));
    assert_eq!(EnvMode::QLoopAll.rustias_mode(), Some(5));
}

#[test]
fn type_controls_append_after_every_released_host_parameter() {
    let specs = parameters::specs();
    let start = specs
        .iter()
        .position(|s| s.id == "drum_buffer_seconds")
        .unwrap()
        + 1;
    for i in 0..8 {
        let ids: Vec<_> = specs[start + i * 5..start + i * 5 + 5]
            .iter()
            .map(|s| s.id.as_str())
            .collect();
        let n = i + 1;
        assert_eq!(
            ids,
            [
                format!("env{n}_mode"),
                format!("env{n}_attack_level"),
                format!("env{n}_decay2"),
                format!("env{n}_sustain2"),
                format!("env{n}_curve"),
            ]
        );
    }
    assert_eq!(specs.len(), start + 40);
    assert_eq!(specs[start].max, 6.0);
    assert_eq!(specs[start].default, 0.0);
    assert_eq!(specs[start + 4].default, 1.0);

    let mut p = Preset::default();
    let changes = parameters::parse(&serde_json::json!({
        "env3_mode": 4.4, "env3_attack_level": 0.5, "env3_decay2": 2.0,
        "env3_sustain2": 0.25, "env3_curve": 6.6, "env8_mode": 99
    }))
    .unwrap();
    assert!(parameters::apply(&mut p, &changes));
    assert_eq!(p.envs[2].mode, EnvMode::QOneShot);
    assert_eq!(p.envs[2].attack_level, 0.5);
    assert_eq!(p.envs[2].decay2, 1.0);
    assert_eq!(p.envs[2].sustain2, 0.25);
    assert_eq!(p.envs[2].curve, 7);
    assert_eq!(p.envs[7].mode, EnvMode::QLoopAll);
}

#[test]
fn q_adsr_attacks_instantly_holds_sustain_and_releases_on_time() {
    let p = Env {
        mode: EnvMode::QAdsr,
        attack: 0.0,
        decay: control(0),
        sustain: control(64),
        release: control(40),
        ..Env::default()
    };
    let mut e = envelope(&p);
    assert!(!e.active());
    assert_eq!(e.next_sample(), 0.0);
    e.on();
    let start = run(&mut e, 2400);
    // Zero Attack and Decay reach Sustain at once; only the gain smoothing remains.
    assert!(start[0] > 0.0 && start[0] < 0.1);
    assert!(start.windows(2).all(|w| w[0] <= w[1]));
    assert_eq!(*start.last().unwrap(), q_level(64));
    assert_eq!(run(&mut e, 4800).last(), Some(&q_level(64)));
    e.off();
    let release = run(&mut e, q_samples(40) + 960);
    assert!(release.windows(2).all(|w| w[0] >= w[1]));
    // The Q release starts fast: 1 - 0.6³ of the level is gone after 40%.
    assert!(release[q_samples(40) * 2 / 5] < q_level(64) / 2.0);
    assert!(release[q_samples(40) / 2] > 0.0);
    assert_eq!(*release.last().unwrap(), 0.0);
    assert!(!e.active());
}

#[test]
fn q_attack_is_linear_and_decay_has_a_fast_start() {
    let p = Env {
        mode: EnvMode::QAdsr,
        attack: control(70),
        decay: control(70),
        sustain: 0.0,
        ..Env::default()
    };
    let mut e = envelope(&p);
    e.on();
    let attack = q_samples(70);
    let values = run(&mut e, attack * 2);
    let quarter = values[attack / 4];
    assert!((quarter - 0.25).abs() < 0.02, "{quarter}");
    assert!(peak(&values) > 0.99);
    let decay_quarter = values[attack + attack / 4];
    // 1 - (1 - 0.25)^3 of the drop happens in the first quarter.
    assert!((decay_quarter - 0.42).abs() < 0.03, "{decay_quarter}");
}

#[test]
fn ads1ds2r_peaks_at_attack_level_and_holds_sustain_two() {
    let p = Env {
        mode: EnvMode::QAds1ds2r,
        attack: control(30),
        decay: control(30),
        sustain: control(90),
        release: control(30),
        attack_level: control(70),
        decay2: control(30),
        sustain2: control(24),
        ..Env::default()
    };
    let mut e = envelope(&p);
    e.on();
    let values = run(&mut e, 4 * q_samples(30) + 4800);
    assert!(
        (peak(&values) - q_level(90)).abs() < 0.03,
        "{}",
        peak(&values)
    );
    // Attack goes to the Attack level, then Decay rises to Sustain 1.
    assert!(values[q_samples(30)] < q_level(80));
    assert_eq!(*values.last().unwrap(), q_level(24));
    e.off();
    run(&mut e, q_samples(30) + 2400);
    assert!(!e.active());
}

#[test]
fn one_shot_completes_after_a_short_gate_and_ends_while_held() {
    let p = Env {
        mode: EnvMode::QOneShot,
        attack: control(70),
        decay: control(70),
        sustain: control(100),
        release: control(70),
        ..Env::default()
    };
    let mut held = envelope(&p);
    held.on();
    let mut released = held;
    run(&mut held, 10);
    run(&mut released, 10);
    released.off();
    let length = 3 * q_samples(70) + 2400;
    let a = run(&mut held, length);
    let b = run(&mut released, length);
    assert_eq!(a, b);
    assert!(peak(&a) > 0.97, "{}", peak(&a));
    assert!(!held.active());
    assert_eq!(*a.last().unwrap(), 0.0);
}

#[test]
fn loops_cycle_while_held_and_stop_on_note_off() {
    for (mode, floor) in [(EnvMode::QLoopS1s2, q_level(24)), (EnvMode::QLoopAll, 0.0)] {
        let p = Env {
            mode,
            attack: control(40),
            decay: control(40),
            sustain: control(100),
            release: control(40),
            attack_level: 1.0,
            decay2: control(40),
            sustain2: control(24),
            ..Env::default()
        };
        let mut e = envelope(&p);
        e.on();
        run(&mut e, q_samples(40) * 2);
        let values = run(&mut e, RATE as usize);
        let rises = values
            .windows(2)
            .filter(|w| w[0] < 0.5 && w[1] >= 0.5)
            .count();
        assert!(rises > 5, "{mode:?} {rises}");
        assert!(e.active());
        assert!(values.iter().all(|&v| v >= floor - 0.01), "{mode:?}");
        if mode == EnvMode::QLoopS1s2 {
            assert!(peak(&values) < q_level(100) + 0.01);
        } else {
            assert!(peak(&values) > 0.85, "{}", peak(&values));
        }
        e.off();
        run(&mut e, 2 * q_samples(40) + 4800);
        assert!(!e.active(), "{mode:?}");
    }
}

#[test]
fn radias_type_runs_the_rustias_controller_curves() {
    let p = Env {
        mode: EnvMode::Radias,
        attack: 0.0,
        decay: control(64),
        sustain: control(40),
        release: control(40),
        ..Env::default()
    };
    // RADIAS segment times: 3 ms · 2000^(value / 127).
    let seconds = |value: u8| 0.003 * 2000.0f32.powf(f32::from(value) / 127.0);
    let attack = (seconds(0) * RATE) as usize;
    let decay = (seconds(64) * RATE) as usize;
    let mut levels = Vec::new();
    for curve in [0, 6] {
        let mut e = envelope(&Env { curve, ..p });
        e.on();
        let values = run(&mut e, attack + decay + 4800);
        assert!(peak(&values) > 0.9, "{curve} {}", peak(&values));
        levels.push(values[attack + decay / 3]);
        // RADIAS segments end within one step of their 0..=32766 target.
        assert!((values.last().unwrap() - radias_level(40)).abs() < 2.0 / 32766.0);
        e.off();
        let release = run(&mut e, (seconds(40) * RATE) as usize + 4800);
        assert_eq!(*release.last().unwrap(), 0.0);
        assert!(!e.active());
    }
    // Cubic decays slowly at first; inverted cubic falls fast at first.
    assert!(levels[0] > 0.9 && levels[1] < 0.6, "{levels:?}");
}

#[test]
fn rustias_times_do_not_depend_on_the_sample_rate() {
    let p = Env {
        mode: EnvMode::QAdsr,
        attack: control(60),
        decay: control(60),
        sustain: control(50),
        ..Env::default()
    };
    let half = TIME_TICKS[60] as f32 / 2000.0 / 2.0;
    let mut values = Vec::new();
    for rate in [44_100.0, 48_000.0, 96_000.0] {
        let mut e = Envelope::default();
        e.configure(rate, &p);
        e.on();
        let v: Vec<_> = (0..(half * rate) as usize)
            .map(|_| e.next_sample())
            .collect();
        values.push(*v.last().unwrap());
    }
    assert!(values.iter().all(|v| (v - 0.5).abs() < 0.02), "{values:?}");
}

#[test]
fn changing_the_type_continues_from_the_current_level() {
    let legacy = Env {
        attack: 0.0,
        decay: 0.0,
        sustain: 0.8,
        release: 0.5,
        ..Env::default()
    };
    let mut e = envelope(&legacy);
    e.on();
    let before = *run(&mut e, 4800).last().unwrap();
    assert!((before - 0.8).abs() < 1e-4);
    let q = Env {
        mode: EnvMode::QAdsr,
        attack: control(40),
        decay: control(127),
        ..legacy
    };
    e.configure(RATE, &q);
    let after = run(&mut e, q_samples(40) + 960);
    assert!((after[0] - before).abs() < 0.01);
    assert!(after.windows(2).all(|w| w[1] >= w[0] - 1e-5));
    assert!(peak(&after) > 0.99);
    e.off();
    e.configure(
        RATE,
        &Env {
            mode: EnvMode::Radias,
            ..q
        },
    );
    let released = run(&mut e, 4);
    assert!(released[0] > 0.98);
    let radias = run(&mut e, RATE as usize * 3);
    assert_eq!(*radias.last().unwrap(), 0.0);
    assert!(!e.active());
    // Back to the original envelope: the next note is the legacy ADSR again.
    e.configure(RATE, &legacy);
    e.on();
    let mut fresh = envelope(&legacy);
    fresh.on();
    assert_eq!(run(&mut e, 400), run(&mut fresh, 400));
}

#[test]
fn rustias_types_render_and_switch_without_audio_allocations() {
    let mut p = Preset {
        env_count: 8,
        ..Preset::default()
    };
    p.params.grains = 0.75;
    for (env, mode) in p
        .envs
        .iter_mut()
        .zip(EnvMode::ALL.into_iter().cycle().skip(1))
    {
        env.mode = mode;
        env.attack = 0.1;
        env.decay = 0.2;
        env.release = 0.1;
    }
    let mut engine = Engine::new(48_000, Arc::new(SampleBuffer::demo(48_000)), p);
    let changes = parameters::parse(&serde_json::json!({
        "env1_mode": 6, "env2_curve": 0, "env3_decay2": 0.9, "env4_mode": 0, "env5_attack": 0.0
    }))
    .unwrap();
    let on = Message::NoteOn {
        channel: 0,
        note: 60,
        velocity: 100,
    };
    let off = Message::NoteOff {
        channel: 0,
        note: 60,
        velocity: 0,
    };
    EVENTS.with(|n| n.set(0));
    WATCH.with(|w| w.set(true));
    engine.midi(on);
    for i in 0..48_000 {
        if i == 9_000 {
            engine.apply_parameters(&changes);
        }
        if i == 30_000 {
            engine.midi(off);
        }
        assert!(engine.process_frame().into_iter().all(f32::is_finite));
    }
    WATCH.with(|w| w.set(false));
    assert_eq!(EVENTS.with(Cell::get), 0);
    assert_eq!(engine.preset.envs[0].mode, EnvMode::QLoopAll);
    assert_eq!(engine.preset.envs[3].mode, EnvMode::Legacy);
}

#[test]
fn a_one_shot_amp_envelope_ends_the_note_while_the_key_is_held() {
    let mut p = Preset::default();
    p.params.grains = 0.75;
    p.envs[0] = Env {
        mode: EnvMode::QOneShot,
        attack: control(20),
        decay: control(40),
        sustain: 1.0,
        release: control(40),
        ..Env::default()
    };
    let mut engine = Engine::new(48_000, Arc::new(SampleBuffer::demo(48_000)), p);
    engine.midi(Message::NoteOn {
        channel: 0,
        note: 60,
        velocity: 127,
    });
    let mut peak = 0.0f32;
    for _ in 0..2400 {
        engine.process_frame();
        peak = peak.max(engine.env_values[0]);
    }
    assert!(peak > 0.99);
    for _ in 0..24_000 {
        engine.process_frame();
    }
    assert_eq!(engine.env_values[0], 0.0);
}
