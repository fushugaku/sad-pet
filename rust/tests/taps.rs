use sad_pet::{
    color::ColorSlot,
    dsp::{Engine, SampleBuffer},
    parameters,
    preset::{Preset, SaturatorType},
    taps::{Stage, Tap},
    timbres::Kind,
};
use std::sync::Arc;

fn engine(preset: Preset, open: u16) -> Engine {
    let mut e = Engine::new(48_000, Arc::new(SampleBuffer::demo(48_000)), preset);
    e.set_open_buses(open);
    e.set_playing(true);
    e
}
/// A Bus engine: one permanent voice at unity, so its stages are exact.
fn bus(edit: impl FnOnce(&mut Preset)) -> Engine {
    let mut preset = Kind::Bus.preset().unwrap();
    edit(&mut preset);
    engine(preset, 1 << 2)
}
fn granular(edit: impl FnOnce(&mut Preset)) -> Engine {
    let mut preset = Preset::default();
    preset.params.master_volume = 0.3;
    edit(&mut preset);
    engine(preset, 1 << 1)
}
fn input(n: usize) -> [f32; 2] {
    let t = n as f32 / 48_000.0;
    [
        (t * 220.0 * std::f32::consts::TAU).sin() * 0.3,
        (t * 3_100.0 * std::f32::consts::TAU).sin() * 0.2,
    ]
}
fn distortion(p: &mut Preset) {
    p.color_chain = vec![ColorSlot {
        id: 1,
        kind: SaturatorType::Distortion,
        mix: 1.0,
        ..Default::default()
    }];
}
fn send(e: &Engine, bus: usize) -> [f32; 2] {
    e.bus_sends().map_or([0.0; 2], |s| s[bus - 1])
}

#[test]
fn a_section_out_of_the_chain_passes_its_input_and_still_sends_its_output() {
    let mut dry = bus(|_| {});
    let mut wet = bus(distortion);
    let mut tapped = bus(|p| {
        distortion(p);
        p.taps.post_fx = Tap {
            chain: 0.0,
            send: 1.0,
            bus: 2,
        };
    });
    let mut half = bus(|p| {
        distortion(p);
        p.taps.post_fx = Tap {
            chain: 0.5,
            send: 0.5,
            bus: 2,
        };
    });
    let (mut sent, mut error, mut blend) = (0.0_f32, 0.0_f32, 0.0_f32);
    for n in 0..12_000 {
        let x = input(n);
        let (d, w) = (dry.process_input_frame(x), wet.process_input_frame(x));
        let (t, h) = (tapped.process_input_frame(x), half.process_input_frame(x));
        assert_eq!(send(&dry, 2), [0.0; 2]);
        if n < 2_000 {
            continue;
        }
        for c in 0..2 {
            sent += send(&tapped, 2)[c].abs();
            error = error
                .max((t[c] - d[c]).abs())
                .max((send(&tapped, 2)[c] - w[c]).abs())
                .max((send(&half, 2)[c] - w[c] * 0.5).abs());
            blend = blend.max((h[c] - (d[c] + w[c]) * 0.5).abs());
        }
    }
    assert!(sent > 100.0, "the bus hears the distortion ({sent})");
    assert!(
        error < 1e-5,
        "chain keeps the input, bus the output ({error})"
    );
    assert!(blend < 1e-5, "Chain 50% blends input and output ({blend})");
}

#[test]
fn a_send_without_its_bus_goes_nowhere_and_leaves_the_chain_alone() {
    let mut reference = bus(distortion);
    let mut closed = bus(|p| {
        distortion(p);
        p.taps.resynth = Tap {
            chain: 1.0,
            send: 1.0,
            bus: 5,
        };
    });
    for n in 0..4_000 {
        let x = input(n);
        assert_eq!(
            closed.process_input_frame(x).map(f32::to_bits),
            reference.process_input_frame(x).map(f32::to_bits)
        );
        assert!(closed.bus_sends().is_none());
    }
}

#[test]
fn every_section_of_a_bus_sends_what_it_hands_on() {
    // Bypassed sections pass and send their input. Individual switched-off
    // filters send nothing; the aggregate Filters stage still passes input.
    for stage in Stage::ALL {
        let mut e = bus(|p| {
            *p.taps.get_mut(stage) = Tap {
                chain: 1.0,
                send: 1.0,
                bus: 2,
            };
        });
        let mut error = 0.0_f32;
        for n in 0..6_000 {
            let x = input(n);
            let y = e.process_input_frame(x);
            if n > 2_000 {
                let expected = if matches!(stage, Stage::Filter1 | Stage::Filter2) {
                    [0.0; 2]
                } else {
                    y
                };
                for (sent, y) in send(&e, 2).into_iter().zip(expected) {
                    error = error.max((sent - y).abs());
                }
            }
        }
        assert!(error < 1e-5, "{stage:?} sends its output ({error})");
    }
}

#[test]
fn granular_without_grains_in_the_chain_sends_the_grains_and_plays_dry() {
    // No reverb: Dry/Wet ramps in from fully wet and its tail would linger.
    let mut dry = granular(|p| {
        p.params.mix = 0.0;
        p.color_chain.clear();
    });
    let mut tapped = granular(|p| {
        p.params.mix = 1.0;
        p.color_chain.clear();
        p.taps.grains = Tap {
            chain: 0.0,
            send: 1.0,
            bus: 1,
        };
    });
    // The grain engine alone: every later section left out of the chain.
    let mut grains = granular(|p| {
        p.params.mix = 1.0;
        p.taps.filters.chain = 0.0;
        p.taps.post_fx.chain = 0.0;
    });
    let (mut sent, mut error, mut heard) = (0.0_f32, 0.0_f32, 0.0_f32);
    for n in 0..30_000 {
        let (d, t, g) = (
            dry.process_frame(),
            tapped.process_frame(),
            grains.process_frame(),
        );
        if n < 12_000 {
            continue;
        }
        for c in 0..2 {
            sent += send(&tapped, 1)[c].abs();
            heard += g[c].abs();
            error = error
                .max((t[c] - d[c]).abs())
                .max((send(&tapped, 1)[c] - g[c]).abs());
        }
    }
    assert!(
        sent > 100.0 && heard > 100.0,
        "grains play ({sent}, {heard})"
    );
    assert!(error < 1e-5, "dry chain, grains on the bus ({error})");
}

#[test]
fn source_fx_out_of_the_chain_blend_back_to_the_file() {
    // Source FX of a file are rendered ahead; Chain 0 reads the file itself.
    let source = |p: &mut Preset| {
        p.source_color_chain = vec![ColorSlot {
            id: 7,
            kind: SaturatorType::Distortion,
            mix: 1.0,
            ..Default::default()
        }];
    };
    let mut plain = granular(|p| p.params.mix = 0.0);
    let render = |chain: f32| {
        let mut e = granular(|p| {
            p.params.mix = 0.0;
            source(p);
            p.taps.source.chain = chain;
        });
        e.prepare_source_effects().unwrap();
        assert!(e.processed_source.is_some() && !e.source_effects_pending());
        e
    };
    let (mut skipped, mut applied) = (render(0.0), render(1.0));
    let (mut error, mut effect) = (0.0_f32, 0.0_f32);
    for n in 0..20_000 {
        let (p, s, a) = (
            plain.process_frame(),
            skipped.process_frame(),
            applied.process_frame(),
        );
        if n > 12_000 {
            error = error.max((p[0] - s[0]).abs()).max((p[1] - s[1]).abs());
            effect = effect.max((p[0] - a[0]).abs());
        }
    }
    assert!(effect > 0.01, "the Source FX change the file ({effect})");
    assert!(
        error < 1e-5,
        "the file plays without its Source FX ({error})"
    );
}

#[test]
fn the_resynth_split_of_earlier_builds_becomes_its_tap() {
    let mut old = serde_json::to_value(Preset::default()).unwrap();
    old["resynth"]["send"] = 0.3.into();
    old["resynth"]["bus"] = 2.into();
    let preset: Preset = serde_json::from_value(old.clone()).unwrap();
    assert_eq!(preset.taps.resynth.bus, 2);
    assert!((preset.taps.resynth.send - 0.3).abs() < 1e-6);
    assert!((preset.taps.resynth.chain - 0.7).abs() < 1e-6);
    // Saved taps win over the old fields, which are no longer written.
    old["taps"] = serde_json::json!({"resynth": {"chain": 1.0, "send": 0.1, "bus": 4}});
    let preset: Preset = serde_json::from_value(old).unwrap();
    assert_eq!(preset.taps.resynth.bus, 4);
    let saved = serde_json::to_value(&preset).unwrap();
    assert!(saved["resynth"].get("send").is_none());
    assert_eq!(saved["taps"]["resynth"]["bus"], 4);
    assert!(
        serde_json::to_value(Preset::default())
            .unwrap()
            .get("taps")
            .is_none()
    );
}

#[test]
fn tap_controls_append_after_the_drum_note_shifts() {
    let ids: Vec<_> = parameters::specs().iter().map(|s| s.id.as_str()).collect();
    let start = ids.iter().position(|&id| id == "tap_source_chain").unwrap();
    assert_eq!(
        ids[start - 3..start],
        ["drum7_note_shift", "resynth_send", "resynth_bus"]
    );
    // Controls added later (LFO/ENV routes, macros) follow the taps.
    assert_eq!(
        ids[start..start + 16],
        [
            "tap_source_chain",
            "tap_grains_chain",
            "tap_resynth_chain",
            "tap_pre_fx_chain",
            "tap_filters_chain",
            "tap_post_fx_chain",
            "tap_source_send",
            "tap_source_bus",
            "tap_grains_send",
            "tap_grains_bus",
            "tap_pre_fx_send",
            "tap_pre_fx_bus",
            "tap_filters_send",
            "tap_filters_bus",
            "tap_post_fx_send",
            "tap_post_fx_bus",
        ]
    );
    assert_eq!(ids[start + 16], "lfo1_depth2");
    let mut preset = Preset::default();
    let changes = parameters::parse(&serde_json::json!({
        "tap_filters_chain": 0.25, "tap_filters_send": 0.5, "tap_filters_bus": 3,
        "resynth_send": 0.75, "resynth_bus": 9,
    }))
    .unwrap();
    parameters::apply(&mut preset, &changes);
    assert_eq!(
        preset.taps.filters,
        Tap {
            chain: 0.25,
            send: 0.5,
            bus: 3
        }
    );
    assert_eq!(preset.taps.resynth.send, 0.75);
    assert_eq!(preset.taps.resynth.bus, 8, "clamped to Bus 8");
}
