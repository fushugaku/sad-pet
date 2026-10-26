use sad_pet::{
    color::ColorSlot,
    dsp::{Engine, SampleBuffer},
    midi::Message,
    preset::{Preset, SaturatorType},
    rack::{Module, Settings, Slot},
    timbres::Kind,
};
use std::sync::Arc;

const RATE: u32 = 48_000;

fn slots(modules: &[Module]) -> Vec<Slot> {
    modules
        .iter()
        .map(|&module| Slot {
            module,
            enabled: true,
        })
        .collect()
}
fn rack_preset(modules: &[Module], edit: impl FnOnce(&mut Preset)) -> Preset {
    let mut preset = Kind::Rack.preset().unwrap();
    preset.rack.as_mut().unwrap().modules = slots(modules);
    preset.color_chain.clear();
    edit(&mut preset);
    preset.sanitize();
    preset
}
fn start(preset: Preset) -> Engine {
    let mut engine = Engine::new(RATE, Arc::new(SampleBuffer::demo(RATE)), preset);
    engine.set_midi_gate(true);
    engine.set_playing(true);
    engine
}
fn note(engine: &mut Engine, note: u8, on: bool) {
    engine.midi(if on {
        Message::NoteOn {
            channel: 0,
            note,
            velocity: 100,
        }
    } else {
        Message::NoteOff {
            channel: 0,
            note,
            velocity: 0,
        }
    });
}
fn render(engine: &mut Engine, frames: usize) -> Vec<[f32; 2]> {
    (0..frames).map(|_| engine.process_frame()).collect()
}
fn peak(frames: &[[f32; 2]]) -> f32 {
    frames
        .iter()
        .flatten()
        .fold(0.0_f32, |peak, x| peak.max(x.abs()))
}
fn bits(frames: &[[f32; 2]]) -> Vec<[u32; 2]> {
    frames.iter().map(|f| f.map(f32::to_bits)).collect()
}
/// Notes for both engines at the same frames: a chord, a release, a new note.
fn play(engine: &mut Engine) -> Vec<[f32; 2]> {
    let mut out = render(engine, 2_000);
    note(engine, 60, true);
    note(engine, 64, true);
    out.extend(render(engine, 9_000));
    note(engine, 60, false);
    out.extend(render(engine, 4_000));
    note(engine, 67, true);
    out.extend(render(engine, 6_000));
    note(engine, 64, false);
    note(engine, 67, false);
    out.extend(render(engine, 8_000));
    out
}
fn sound(edit: impl Fn(&mut Preset)) -> impl Fn(&mut Preset) {
    move |p: &mut Preset| {
        p.params.grains = 0.7;
        p.params.grain_size = 0.62;
        p.params.cloud = 0.3;
        p.params.mix = 0.65;
        p.params.filter = 0.45;
        p.params.filter_resonance = 0.4;
        p.params.filter_env_depth = 0.6;
        p.color_chain = vec![ColorSlot {
            id: 1,
            kind: SaturatorType::Distortion,
            mix: 0.6,
            ..ColorSlot::default()
        }];
        edit(p);
    }
}

#[test]
fn sample_grains_filters_and_fx_play_a_granular_timbre_bit_for_bit() {
    let edit = sound(|_| {});
    let mut granular = Preset::default();
    edit(&mut granular);
    let rack = rack_preset(
        &[
            Module::Sample,
            Module::Grains,
            Module::Filters,
            Module::PostFx,
        ],
        &edit,
    );
    assert!(rack.rack.as_ref().unwrap().grains_read_file());
    let expected = play(&mut start(granular));
    let rendered = play(&mut start(rack));
    assert!(peak(&expected) > 0.01);
    assert_eq!(bits(&rendered), bits(&expected));
}

#[test]
fn a_synth_over_its_filters_plays_the_synth_engine() {
    let edit = sound(|p| p.color_chain.clear());
    let mut synth = Kind::Synth.preset().unwrap();
    edit(&mut synth);
    synth.sanitize();
    let rack = rack_preset(&[Module::Synth, Module::Filters], |p| {
        edit(p);
        p.synth = synth.synth.clone();
    });
    // Both output levels settle first, so the frame between them is all
    // that differs.
    let settled = |preset| {
        let mut engine = start(preset);
        render(&mut engine, 24_000);
        play(&mut engine)
    };
    let expected = settled(synth);
    let rendered = settled(rack);
    assert!(peak(&expected) > 0.01);
    // A Synth timbre hears its voices through its history, a frame later.
    assert_eq!(bits(&rendered[..rendered.len() - 1]), bits(&expected[1..]));
}

fn drum_rack(modules: &[Module], edit: impl FnOnce(&mut Preset)) -> Engine {
    start(rack_preset(modules, |p| {
        let drums = p.drums.get_or_insert_with(Default::default);
        for step in [0, 4, 8, 12] {
            drums.tracks[0].steps[step].velocity = 1.0;
        }
        edit(p);
    }))
}

#[test]
fn drums_play_their_pattern_without_notes_and_keys_play_the_synth() {
    let mut engine = drum_rack(&[Module::Drums, Module::Synth, Module::Mixer], |p| {
        p.rack.as_mut().unwrap().strips.synth.muted = true
    });
    let drums = render(&mut engine, RATE as usize / 2);
    assert!(peak(&drums) > 0.05, "the pattern plays");
    assert!(engine.rack_peaks().unwrap()[2] > 0.05);
    engine.clear_rack_peaks();
    // Muted, the Synth strip keeps the keys out; the kick's note does not
    // play the drums.
    let mut silent = drum_rack(&[Module::Drums, Module::Synth, Module::Mixer], |p| {
        let rack = p.rack.as_mut().unwrap();
        rack.strips.synth.muted = true;
        rack.strips.drums.muted = true;
    });
    note(&mut silent, 36, true);
    assert_eq!(peak(&render(&mut silent, RATE as usize / 2)), 0.0);
    // The Synth strip plays the keys.
    let mut keys = drum_rack(&[Module::Drums, Module::Synth, Module::Mixer], |p| {
        p.rack.as_mut().unwrap().strips.drums.muted = true;
    });
    assert_eq!(peak(&render(&mut keys, 4_000)), 0.0);
    note(&mut keys, 48, true);
    assert!(peak(&render(&mut keys, 8_000)) > 0.01);
    let peaks = keys.rack_peaks().unwrap();
    keys.clear_rack_peaks();
    assert!(peaks[1] > 0.01 && peaks[2] == 0.0, "{peaks:?}");
}

#[test]
fn the_mixer_balances_its_strips_and_solo_keeps_one_chain() {
    let level = |edit: fn(&mut Settings)| {
        let mut engine = drum_rack(&[Module::Drums, Module::Mixer], |p| {
            edit(p.rack.as_mut().unwrap())
        });
        // Long enough for the strip gains to settle where they start.
        let frames = render(&mut engine, RATE as usize / 2);
        let left = frames.iter().fold(0f32, |p, f| p.max(f[0].abs()));
        let right = frames.iter().fold(0f32, |p, f| p.max(f[1].abs()));
        (left, right)
    };
    let (left, right) = level(|_| {});
    assert!(left > 0.05 && right > 0.05);
    let (half_left, _) = level(|r| r.strips.drums.level = 0.5);
    assert!((half_left / left - 0.5).abs() < 0.01, "{half_left} {left}");
    let (hard_left, hard_right) = level(|r| r.strips.drums.pan = 0.0);
    assert!(hard_left > 0.05 && hard_right == 0.0);
    // Soloing a chain that is not in the Mixer silences none.
    let (solo_other, _) = level(|r| r.strips.synth.solo = true);
    assert_eq!(solo_other, left);
}

#[test]
fn modules_run_in_the_order_they_stand() {
    let render_order = |modules: &[Module]| {
        let mut engine = start(rack_preset(
            modules,
            sound(|p| {
                p.color_chain[0].mix = 1.0;
                p.params.filter = 0.2;
            }),
        ));
        play(&mut engine)
    };
    let filtered_then_driven = render_order(&[
        Module::Synth,
        Module::Filters,
        Module::Mixer,
        Module::PostFx,
    ]);
    let driven_then_filtered = render_order(&[
        Module::Synth,
        Module::Mixer,
        Module::PostFx,
        Module::Filters,
    ]);
    assert!(peak(&filtered_then_driven) > 0.01 && peak(&driven_then_filtered) > 0.01);
    assert_ne!(bits(&filtered_then_driven), bits(&driven_then_filtered));
}

#[test]
fn modules_switched_off_pass_the_chain_and_sources_off_are_silent() {
    let render_with = |edit: fn(&mut Settings)| {
        let mut engine = start(rack_preset(
            &[Module::Synth, Module::Filters, Module::PostFx],
            sound(move |p| edit(p.rack.as_mut().unwrap())),
        ));
        play(&mut engine)
    };
    let without = render_with(|r| r.modules.retain(|s| s.module == Module::Synth));
    let off = render_with(|r| {
        for slot in &mut r.modules[1..] {
            slot.enabled = false;
        }
    });
    assert!(peak(&without) > 0.01);
    assert_eq!(bits(&off), bits(&without));
    let silent = render_with(|r| r.modules[0].enabled = false);
    assert_eq!(peak(&silent), 0.0);
}

#[test]
fn a_grain_engine_under_the_mixer_granulates_the_chains_it_hears() {
    let mut engine = drum_rack(&[Module::Drums, Module::Mixer, Module::Grains], |p| {
        p.params.grains = 0.9;
        p.params.mix = 1.0;
    });
    assert!(
        engine.live_input.is_some(),
        "the Grain engine keeps a history"
    );
    let frames = render(&mut engine, RATE as usize);
    assert!(engine.active_grains() > 0);
    assert!(peak(&frames) > 0.01);
    // Right under the Sample, the grains read the file and need no history.
    let file = start(rack_preset(&[Module::Sample, Module::Grains], |_| {}));
    assert!(file.live_input.is_none());
}

#[test]
fn rack_edits_keep_the_grain_history_and_a_kind_change_retires_it() {
    let modules = [Module::Drums, Module::Mixer, Module::Grains];
    let mut engine = drum_rack(&modules, |_| {});
    render(&mut engine, 4_000);
    let history = engine.live_input.as_ref().unwrap().sample.prepared_levels()[0].as_ptr();
    // Another module keeps the running history.
    let mut edited = engine.preset.clone();
    edited.rack.as_mut().unwrap().modules =
        slots(&[Module::Drums, Module::Mixer, Module::PreFx, Module::Grains]);
    engine.set_preset(edited);
    assert_eq!(
        engine.live_input.as_ref().unwrap().sample.prepared_levels()[0].as_ptr(),
        history
    );
    render(&mut engine, 1_000);
    // Grains moved under a Sample read the file instead.
    let mut file = engine.preset.clone();
    file.rack.as_mut().unwrap().modules = slots(&[Module::Sample, Module::Grains]);
    engine.set_preset(file);
    assert!(engine.live_input.is_none());
    render(&mut engine, 1_000);
    // A Synth timbre records its own history.
    engine.set_preset(Kind::Synth.preset().unwrap());
    assert!(engine.live_input.is_some());
    render(&mut engine, 1_000);
    engine.set_preset(rack_preset(&modules, |_| {}));
    assert!(engine.live_input.is_some());
    render(&mut engine, 1_000);
    engine.set_preset(Kind::Granular.preset().unwrap_or_default());
    assert!(engine.live_input.is_none());
}

#[test]
fn a_rack_preset_round_trips_and_keeps_only_its_sources_settings() {
    let preset = rack_preset(&[Module::Synth, Module::Mixer], |p| {
        p.drums = Some(Default::default());
        p.rack.as_mut().unwrap().strips.synth.level = 0.25;
    });
    assert!(preset.drums.is_none() && preset.synth.is_some());
    assert_eq!(Kind::of(&preset), Kind::Rack);
    let json = serde_json::to_string(&preset).unwrap();
    let back: Preset = serde_json::from_str(&json).unwrap();
    assert_eq!(back.rack, preset.rack);
    assert_eq!(Kind::of(&back), Kind::Rack);
    // Other presets serialize without a rack.
    assert!(
        !serde_json::to_string(&Preset::default())
            .unwrap()
            .contains("\"rack\"")
    );
}

#[test]
fn strip_controls_are_host_parameters_that_ramp_and_modulate() {
    let id = |name: &str| {
        sad_pet::parameters::specs()
            .iter()
            .position(|s| s.id == name)
            .unwrap()
    };
    let mut engine = drum_rack(&[Module::Drums, Module::Mixer], |_| {});
    render(&mut engine, RATE as usize / 2);
    let loud = peak(&render(&mut engine, RATE as usize / 2));
    engine.apply_parameters(
        &sad_pet::parameters::parse(&serde_json::json!({"rack_drums_level": 0.25})).unwrap(),
    );
    assert_eq!(
        engine.preset.rack.as_ref().unwrap().strips.drums.level,
        0.25
    );
    // The fader ramps: the next frames move by small steps, never jump.
    let ramp = render(&mut engine, 64);
    for pair in ramp.windows(2) {
        assert!((pair[1][0] - pair[0][0]).abs() < 0.5);
    }
    // The next kick, a whole beat later, plays at the new level.
    render(&mut engine, RATE as usize / 4);
    let quiet = peak(&render(&mut engine, RATE as usize / 2));
    assert!((quiet / loud - 0.25).abs() < 0.02, "{quiet} {loud}");
    // An LFO moves the strip like any control.
    let mut modulated = drum_rack(&[Module::Drums, Module::Mixer], |p| {
        p.lfos[0].rate_hz = 0.5;
        p.lfos[0].destination = sad_pet::preset::Destination::Control {
            index: id("rack_drums_level") as u16,
        };
        p.lfos[0].depth = 1.0;
    });
    let swing: Vec<f32> = (0..4)
        .map(|_| peak(&render(&mut modulated, RATE as usize / 4)))
        .collect();
    let (low, high) = swing
        .iter()
        .fold((f32::MAX, 0f32), |(l, h), &x| (l.min(x), h.max(x)));
    assert!(high > low * 1.5, "{swing:?}");
    assert!(sad_pet::parameters::specs()[id("rack_buffer_seconds")].max == 32.0);
}
