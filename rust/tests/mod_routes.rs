use sad_pet::{
    dsp::{Engine, SampleBuffer, telemetry::RealtimeView},
    macros::{Macro, Target},
    matrix::{LfoParameter, Route, Source},
    parameters,
    preset::{Destination, Env, Lfo, Preset, Waveform},
};
use serde_json::json;
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
fn target(id: &str) -> Destination {
    Destination::Control {
        index: index(id) as u16,
    }
}
fn index(id: &str) -> usize {
    parameters::specs().iter().position(|s| s.id == id).unwrap()
}
/// Renders without a single allocation on the audio path.
fn render(e: &mut Engine, n: usize) {
    EVENTS.with(|n| n.set(0));
    WATCH.with(|w| w.set(true));
    for _ in 0..n {
        assert!(e.process_frame().into_iter().all(f32::is_finite));
    }
    WATCH.with(|w| w.set(false));
    assert_eq!(EVENTS.with(Cell::get), 0, "the audio path allocated");
}
fn quiet() -> Preset {
    let mut p = Preset::default();
    p.params.grains = 0.0;
    p.color_chain.clear();
    p.params.cloud = 0.0;
    p.params.detune = 0.0;
    p.params.stereo_spread = 0.0;
    p
}

#[test]
fn every_lfo_and_envelope_drives_three_destinations_of_its_own() {
    let mut p = quiet();
    p.lfos[0] = Lfo {
        waveform: Waveform::Square,
        depth: 0.0,
        destination: Destination::Pitch,
        destination2: Destination::Cloud,
        depth2: 1.0,
        destination3: target("resynth_osc1_control1"),
        depth3: 0.5,
        ..Lfo::default()
    };
    p.envs[2] = Env {
        attack: 0.0,
        decay: 0.0,
        sustain: 1.0,
        destination: Destination::Detune,
        depth: 0.25,
        extra_destination: Destination::None,
        destination3: Destination::StereoSpread,
        depth3: 0.75,
        ..Env::default()
    };
    let mut e = Engine::new(8000, Arc::new(SampleBuffer::demo(8000)), p);
    e.set_playing(true);
    render(&mut e, 2000);
    assert_eq!(e.env_values[2], 1.0);
    let knobs = e.knob_states();
    // A square LFO at +1 adds half its depth; route 1 has none.
    assert_eq!(knobs["pitch"].current, knobs["pitch"].base);
    assert!(!knobs["pitch"].modulated);
    assert!((knobs["cloud"].current - 0.5).abs() < 1e-6);
    assert!((knobs["resynth_osc1_control1"].current - 0.25).abs() < 1e-6);
    assert!((knobs["detune"].current - 0.25).abs() < 1e-6);
    assert!((knobs["stereo_spread"].current - 0.75).abs() < 1e-6);
    // The rings span what each route can reach.
    assert_eq!((knobs["cloud"].min, knobs["cloud"].max), (0.0, 0.5));
    assert_eq!(
        (knobs["stereo_spread"].min, knobs["stereo_spread"].max),
        (0.0, 0.75)
    );
    let mut v = RealtimeView::default();
    e.realtime_view(&mut v);
    let host = e.knob_states_from_view(&v);
    assert_eq!(
        host["stereo_spread"].current,
        knobs["stereo_spread"].current
    );
    assert_eq!(host["cloud"].max, knobs["cloud"].max);
}

#[test]
fn route_depths_are_host_parameters_and_modulation_destinations() {
    let mut p = quiet();
    p.lfos[0] = Lfo {
        waveform: Waveform::Square,
        destination2: Destination::Cloud,
        depth2: 0.2,
        ..Lfo::default()
    };
    // A second square LFO opens route 2 of LFO 1 the rest of the way.
    p.lfos[1].waveform = Waveform::Square;
    p.matrix.push(Route {
        source: Source::Lfo2,
        destination: Destination::Lfo {
            index: 0,
            parameter: LfoParameter::Depth2,
        },
        depth: 0.4,
        enabled: true,
    });
    let mut e = Engine::new(8000, Arc::new(SampleBuffer::demo(8000)), p);
    e.set_playing(true);
    render(&mut e, 2000);
    let knobs = e.knob_states();
    assert!((knobs["lfo1_depth2"].current - 1.0).abs() < 1e-6);
    assert!((knobs["cloud"].current - 0.5).abs() < 1e-6);

    // An LFO route opens another LFO's route the same way.
    let mut p = quiet();
    p.lfos[1] = Lfo {
        waveform: Waveform::Square,
        destination: Destination::Lfo {
            index: 2,
            parameter: LfoParameter::Depth,
        },
        depth: 1.0,
        ..Lfo::default()
    };
    p.lfos[2] = Lfo {
        waveform: Waveform::Square,
        destination: Destination::Cloud,
        depth: 0.0,
        ..Lfo::default()
    };
    let mut e = Engine::new(8000, Arc::new(SampleBuffer::demo(8000)), p);
    e.set_playing(true);
    render(&mut e, 2000);
    let knobs = e.knob_states();
    assert!((knobs["lfo3_depth"].current - 1.0).abs() < 1e-6);
    assert!((knobs["cloud"].current - 0.5).abs() < 1e-6);
    assert_eq!((knobs["cloud"].min, knobs["cloud"].max), (0.0, 0.5));

    // Routes into an earlier LFO, from an ENV too, apply before its own.
    let mut p = quiet();
    p.lfos[0] = Lfo {
        waveform: Waveform::Square,
        destination: Destination::Cloud,
        depth: 0.0,
        destination2: Destination::StereoSpread,
        depth2: 0.0,
        ..Lfo::default()
    };
    p.lfos[2] = Lfo {
        waveform: Waveform::Square,
        destination: Destination::Lfo {
            index: 0,
            parameter: LfoParameter::Depth,
        },
        depth: 0.6,
        ..Lfo::default()
    };
    p.envs[1] = Env {
        attack: 0.0,
        decay: 0.0,
        sustain: 1.0,
        destination: Destination::Lfo {
            index: 0,
            parameter: LfoParameter::Depth2,
        },
        depth: 0.25,
        ..Env::default()
    };
    let mut e = Engine::new(8000, Arc::new(SampleBuffer::demo(8000)), p);
    e.set_playing(true);
    render(&mut e, 2000);
    assert_eq!(e.env_values[1], 1.0);
    let knobs = e.knob_states();
    assert!((knobs["cloud"].current - 0.3).abs() < 1e-6);
    assert!((knobs["stereo_spread"].current - 0.25).abs() < 1e-6);
    assert_eq!(knobs["stereo_spread"].max, 0.25);

    // Two LFOs opening each other still render.
    let mut p = e.preset.clone();
    p.lfos[1].destination = Destination::Lfo {
        index: 2,
        parameter: LfoParameter::Depth,
    };
    p.lfos[2].destination = Destination::Lfo {
        index: 1,
        parameter: LfoParameter::Depth,
    };
    e.set_preset(p);
    render(&mut e, 400);

    let mut p = Preset::default();
    let changes =
        parameters::parse(&json!({"lfo3_depth2":-0.5,"lfo8_depth3":2.0,"env5_depth3":0.3}))
            .unwrap();
    assert!(parameters::apply(&mut p, &changes));
    assert_eq!(p.lfos[2].depth2, -0.5);
    assert_eq!(p.lfos[7].depth3, 1.0);
    assert_eq!(p.envs[4].depth3, 0.3);
    // ENV 5 curve and type kept their own host IDs.
    assert_eq!(p.envs[4].curve, Env::default().curve);
    let specs = parameters::specs();
    assert_eq!(specs[index("env1_curve")].get(&p), 1.0);
    assert_eq!(specs[index("env5_depth3")].get(&p), 0.3);
}

#[test]
fn new_host_parameters_append_after_every_released_id() {
    let specs = parameters::specs();
    let last_tap = specs
        .iter()
        .position(|s| s.id == "tap_post_fx_bus")
        .unwrap();
    assert_eq!(specs[last_tap + 1].id, "lfo1_depth2");
    assert_eq!(specs[last_tap + 2].id, "lfo1_depth3");
    assert_eq!(specs[last_tap + 3].id, "env1_depth3");
    assert_eq!(specs[last_tap + 24].id, "env8_depth3");
    for i in 0..8 {
        assert_eq!(specs[last_tap + 25 + i].id, format!("macro{}", i + 1));
    }
    // Advanced grain mode and its groups follow the macros.
    assert_eq!(specs[last_tap + 33].id, "grain_advanced");
    // Macro knobs are sources: no route can target them.
    let catalog = sad_pet::sequence::catalog();
    let modulation = catalog["modulation"].as_array().unwrap();
    assert!(modulation.iter().any(|s| s["id"] == "lfo8_depth3"));
    assert!(!modulation.iter().any(|s| s["id"] == "macro1"));
}

#[test]
fn a_macro_turns_several_destinations_from_one_knob() {
    let mut p = quiet();
    p.params.filter = 0.5;
    p.macros.push(Macro {
        name: "Bloom".into(),
        value: 0.5,
        targets: vec![
            Target {
                destination: Destination::Cloud,
                depth: 1.0,
            },
            Target {
                destination: Destination::Filter,
                depth: -0.4,
            },
            Target {
                destination: target("resynth_osc2_level"),
                depth: 0.6,
            },
            Target {
                destination: Destination::Lfo {
                    index: 0,
                    parameter: LfoParameter::Depth,
                },
                depth: 1.0,
            },
        ],
    });
    p.lfos[0] = Lfo {
        waveform: Waveform::Square,
        destination: Destination::StereoSpread,
        ..Lfo::default()
    };
    let level = p.resynth.oscillators[1].level;
    let mut e = Engine::new(8000, Arc::new(SampleBuffer::demo(8000)), p);
    e.set_playing(true);
    render(&mut e, 2000);
    let knobs = e.knob_states();
    assert!((knobs["cloud"].current - 0.5).abs() < 1e-6);
    assert!((knobs["resynth_osc2_level"].current - (level + 0.3).min(1.0)).abs() < 1e-6);
    // The macro opens LFO 1 (depth −1…1) before the LFO routes itself.
    assert!((knobs["lfo1_depth"].current - 1.0).abs() < 1e-6);
    assert!((knobs["stereo_spread"].current - 0.5).abs() < 1e-6);
    assert_eq!((knobs["cloud"].min, knobs["cloud"].max), (0.0, 1.0));
    assert!(knobs["cloud"].modulated);
    assert_eq!(e.preset.params.cloud, 0.0, "modulation wrote the preset");

    // The host turns the knob under its own ID.
    let changes = parameters::parse(&json!({"macro1":1.0,"macro2":0.7})).unwrap();
    assert!(parameters::apply(&mut e.preset, &changes));
    assert_eq!(e.preset.macros[0].value, 1.0);
    assert_eq!(e.preset.macros.len(), 1, "a missing macro appeared");
    render(&mut e, 400);
    assert!((e.knob_states()["cloud"].current - 1.0).abs() < 1e-6);

    // The editor sets a target's depth without sending the whole preset.
    let changes = parameters::parse(&json!({"macro1_depth2":-3.0,"macro1_depth8":0.5})).unwrap();
    assert!(parameters::apply(&mut e.preset, &changes));
    assert_eq!(e.preset.macros[0].targets[1].depth, -1.0);
    assert_eq!(
        e.preset.macros[0].targets.len(),
        4,
        "a missing target appeared"
    );
    assert!(changes.iter().all(|c| c.host_index().is_none()));
    for bad in [
        "macro0_depth1",
        "macro9_depth1",
        "macro1_depth0",
        "macro1_depth9",
        "macro1_depth",
    ] {
        assert!(
            parameters::parse(&json!({ bad: 0.1 })).is_err(),
            "accepted {bad}"
        );
    }
}

#[test]
fn macros_serialize_only_when_present_and_sanitize_their_targets() {
    let value = serde_json::to_value(Preset::default()).unwrap();
    assert!(value.get("macros").is_none(), "old presets gained a field");
    let old: Preset = serde_json::from_value(value).unwrap();
    assert!(old.macros.is_empty());

    let mut p = Preset {
        macros: (0..10)
            .map(|i| Macro {
                name: "x".repeat(60),
                value: if i == 0 { f32::NAN } else { 3.0 },
                targets: (0..12)
                    .map(|_| Target {
                        destination: Destination::Color {
                            slot: 999,
                            parameter: sad_pet::color::ColorParameter::Mix,
                        },
                        depth: -4.0,
                    })
                    .collect(),
            })
            .collect(),
        ..Preset::default()
    };
    p.macros[1].targets[0].destination = Destination::Pitch;
    p.sanitize();
    assert_eq!(p.macros.len(), 8);
    assert_eq!(p.macros[0].value, 0.0);
    assert_eq!(p.macros[1].value, 1.0);
    assert_eq!(p.macros[1].name.chars().count(), 40);
    assert_eq!(p.macros[1].targets.len(), 8);
    assert_eq!(p.macros[1].targets[0].destination, Destination::Pitch);
    assert_eq!(p.macros[1].targets[0].depth, -1.0);
    // Routes to an effect slot that is not there are dropped.
    assert_eq!(p.macros[1].targets[1].destination, Destination::None);

    let copy: Preset = serde_json::from_slice(&serde_json::to_vec(&p).unwrap()).unwrap();
    assert_eq!(copy.macros, p.macros);
    let lfo = Lfo {
        destination3: Destination::Grains,
        depth3: -0.3,
        ..Lfo::default()
    };
    let env = Env {
        destination3: Destination::Pan,
        depth3: 0.6,
        ..Env::default()
    };
    let lfo_copy: Lfo = serde_json::from_value(serde_json::to_value(lfo).unwrap()).unwrap();
    let env_copy: Env = serde_json::from_value(serde_json::to_value(env).unwrap()).unwrap();
    assert_eq!(lfo_copy.routes(), lfo.routes());
    assert_eq!(env_copy, env);
    // Presets from before the third route keep theirs empty.
    let older: Env = serde_json::from_value(json!({"destination":"pitch","depth":0.2})).unwrap();
    assert_eq!(older.routes()[2], (Destination::None, 0.0));
}
