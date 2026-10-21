use sad_pet::{
    Stereo,
    color::{ColorParameter as C, ColorSlot},
    dsp::{Engine, SampleBuffer},
    matrix::{EnvParameter, LfoParameter, Parameter, Route, Source, StepParameter},
    midi::Message,
    performance::Mode,
    preset::{Destination as D, Env, Preset, SaturatorType as Type, Waveform},
};
use std::sync::Arc;

fn sample() -> Arc<SampleBuffer> {
    Arc::new(
        SampleBuffer::new(
            "tone".into(),
            8000,
            (0..8000)
                .map(|i| {
                    let t = i as f32 / 8000.0 * std::f32::consts::TAU;
                    [(t * 173.0).sin() * 0.3, (t * 311.0).sin() * 0.2]
                })
                .collect(),
        )
        .unwrap(),
    )
}
fn engine(p: Preset) -> Engine {
    Engine::new(8000, sample(), p)
}
fn route(source: Source, destination: D, depth: f32) -> Route {
    Route {
        source,
        destination,
        depth,
        enabled: true,
    }
}
fn on(e: &mut Engine, channel: u8, note: u8, velocity: u8) {
    e.midi(Message::NoteOn {
        channel,
        note,
        velocity,
    });
}
fn advance(e: &mut Engine, n: usize) {
    for _ in 0..n {
        e.process_frame();
    }
}

#[test]
fn controller_sources_are_channel_aware_and_reset_without_sticking() {
    let mut e = engine(Preset::default());
    on(&mut e, 3, 84, 64);
    assert!((e.modulation_source(Source::Velocity) - 64.0 / 127.0).abs() < 1e-6);
    assert_eq!(e.modulation_source(Source::KeyTrack), 0.5);
    e.midi(Message::Control {
        channel: 2,
        controller: 1,
        value: 127,
    });
    assert_eq!(e.modulation_source(Source::ModWheel), 0.0);
    e.midi(Message::Control {
        channel: 3,
        controller: 1,
        value: 100,
    });
    e.midi(Message::Control {
        channel: 3,
        controller: 11,
        value: 80,
    });
    e.midi(Message::PitchBend {
        channel: 3,
        value: 0,
    });
    e.midi(Message::Pressure {
        channel: 3,
        value: 40,
    });
    e.midi(Message::PolyPressure {
        channel: 3,
        note: 84,
        value: 90,
    });
    assert_eq!(e.modulation_source(Source::ModWheel), 100.0 / 127.0);
    assert_eq!(e.modulation_source(Source::Expression), 80.0 / 127.0);
    assert_eq!(e.modulation_source(Source::PitchBend), -1.0);
    assert_eq!(e.modulation_source(Source::Aftertouch), 90.0 / 127.0);
    e.midi(Message::Control {
        channel: 3,
        controller: 64,
        value: 127,
    });
    e.midi(Message::NoteOff {
        channel: 3,
        note: 84,
        velocity: 0,
    });
    assert_eq!(e.modulation_source(Source::Gate), 1.0);
    e.midi(Message::Control {
        channel: 3,
        controller: 121,
        value: 0,
    });
    assert_eq!(e.modulation_source(Source::Gate), 0.0);
    assert_eq!(e.modulation_source(Source::Aftertouch), 0.0);
    assert_eq!(e.modulation_source(Source::ModWheel), 0.0);
    assert_eq!(e.modulation_source(Source::Expression), 1.0);
    assert_eq!(e.modulation_source(Source::PitchBend), 0.0);
}

#[test]
fn random_note_is_held_between_notes_and_reproducible() {
    let mut a = engine(Preset::default());
    let mut b = engine(Preset::default());
    let mut previous = 0.0;
    for note in [60, 60, 67, 72] {
        on(&mut a, 0, note, 100);
        on(&mut b, 0, note, 100);
        let value = a.modulation_source(Source::RandomNote);
        assert_eq!(value, b.modulation_source(Source::RandomNote));
        assert_ne!(value, previous);
        advance(&mut a, 1000);
        advance(&mut b, 1000);
        assert_eq!(a.modulation_source(Source::RandomNote), value);
        previous = value;
    }
}

#[test]
fn matrix_limits_validates_and_roundtrips_eight_rows() {
    let mut p = Preset {
        matrix: vec![route(Source::Expression, D::Filter, 0.2); 12],
        ..Preset::default()
    };
    p.matrix[0].depth = f32::NAN;
    p.matrix[1].destination = D::Env {
        index: 9,
        parameter: EnvParameter::Attack,
    };
    p.matrix[2].destination = D::Color {
        slot: 999,
        parameter: C::Mix,
    };
    p.sanitize();
    assert_eq!(p.matrix.len(), 8);
    assert_eq!(p.matrix[0].depth, 0.0);
    assert_eq!(p.matrix[1].destination, D::None);
    assert_eq!(p.matrix[2].destination, D::None);
    let json = serde_json::to_value(&p).unwrap();
    let mut restored: Preset = serde_json::from_value(json.clone()).unwrap();
    restored.sanitize();
    assert_eq!(serde_json::to_value(&restored).unwrap(), json);
    let mut old = serde_json::to_value(Preset::default()).unwrap();
    old.as_object_mut().unwrap().remove("matrix");
    assert!(
        serde_json::from_value::<Preset>(old)
            .unwrap()
            .matrix
            .is_empty()
    );
}

#[test]
fn equal_opposite_routes_cancel_before_clamping_and_bypass_restores_base() {
    let mut p = Preset::default();
    p.params.filter = 0.95;
    p.matrix = vec![
        route(Source::Expression, D::Filter, 0.5),
        route(Source::Expression, D::Filter, -0.5),
    ];
    let mut e = engine(p);
    advance(&mut e, 64);
    assert!((e.modulated_params().filter - 0.95).abs() < 1e-6);
    assert_eq!(e.preset.params.filter, 0.95);
    let mut p = e.preset.clone();
    p.matrix[1].enabled = false;
    e.set_preset(p);
    assert_eq!(e.modulated_params().filter, 1.0);
    let mut p = e.preset.clone();
    p.matrix[0].enabled = false;
    e.set_preset(p);
    assert_eq!(e.modulated_params().filter, 0.95);
}

#[test]
fn all_lfos_envelopes_and_follower_produce_live_source_values() {
    let mut p = Preset::default();
    for lfo in &mut p.lfos {
        lfo.waveform = Waveform::Square;
    }
    p.envs.fill(Env {
        attack: 0.0,
        decay: 0.0,
        sustain: 1.0,
        ..Env::default()
    });
    let sources = [
        Source::Lfo1,
        Source::Lfo2,
        Source::Lfo3,
        Source::Env1,
        Source::Env2,
        Source::Env3,
        Source::Follower,
        Source::Gate,
    ];
    p.matrix = sources.iter().map(|s| route(*s, D::Pan, 0.05)).collect();
    let mut e = engine(p);
    on(&mut e, 0, 60, 127);
    advance(&mut e, 800);
    for (i, source) in sources.iter().enumerate() {
        assert!(e.modulation_source(*source) > 0.05);
        assert!(e.matrix_values[i] > 0.0);
    }
    assert!(e.modulated_params().filter.is_finite());
}

#[test]
fn adsr_lfo_and_filter_control_targets_affect_the_running_engine() {
    let mut p = Preset::default();
    p.envs[0] = Env {
        attack: 0.0,
        decay: 0.0,
        sustain: 0.8,
        ..Env::default()
    };
    p.matrix = vec![
        route(
            Source::Expression,
            D::Env {
                index: 0,
                parameter: EnvParameter::Sustain,
            },
            -0.4,
        ),
        route(
            Source::Expression,
            D::Lfo {
                index: 1,
                parameter: LfoParameter::Rate,
            },
            0.25,
        ),
        route(
            Source::Expression,
            D::Parameter {
                parameter: Parameter::FilterEnvDepth,
            },
            -0.2,
        ),
        route(
            Source::Expression,
            D::Parameter {
                parameter: Parameter::Filter2Keytrack,
            },
            0.6,
        ),
    ];
    let mut e = engine(p);
    on(&mut e, 0, 60, 127);
    advance(&mut e, 1600);
    assert!((e.env_values[0] - 0.4).abs() < 0.001);
    assert!(e.modulated_lfos()[1].rate_hz > 1.0);
    assert!((e.modulated_params().filter_env_depth + 0.05).abs() < 1e-6);
    assert_eq!(e.modulated_params().filter2_keytrack, 0.6);
    // Editing an unrelated parameter must retain the effective ADSR coefficients.
    let mut p = e.preset.clone();
    p.params.cloud = 0.3;
    e.set_preset(p);
    advance(&mut e, 64);
    assert!((e.env_values[0] - 0.4).abs() < 0.001);
    assert_eq!(e.preset.envs[0].sustain, 0.8);
}

#[test]
fn every_parameter_family_is_bounded_and_applied_without_mutating_knobs() {
    let p = Preset {
        matrix: vec![
            route(
                Source::Expression,
                D::Parameter {
                    parameter: Parameter::Tempo,
                },
                1.0,
            ),
            route(
                Source::Expression,
                D::Parameter {
                    parameter: Parameter::InputGain,
                },
                1.0,
            ),
            route(
                Source::Expression,
                D::Parameter {
                    parameter: Parameter::OutputGain,
                },
                -1.0,
            ),
            route(
                Source::Expression,
                D::Parameter {
                    parameter: Parameter::SampleTuneSemis,
                },
                1.0,
            ),
            route(
                Source::Expression,
                D::Parameter {
                    parameter: Parameter::SampleTuneCents,
                },
                -1.0,
            ),
            route(
                Source::Expression,
                D::Lfo {
                    index: 0,
                    parameter: LfoParameter::Depth,
                },
                -1.0,
            ),
            route(
                Source::Expression,
                D::Env {
                    index: 2,
                    parameter: EnvParameter::Release,
                },
                1.0,
            ),
            route(
                Source::Expression,
                D::Parameter {
                    parameter: Parameter::PitchBendRange,
                },
                1.0,
            ),
        ],
        ..Preset::default()
    };
    let mut e = engine(p);
    advance(&mut e, 64);
    let m = e.modulated_params();
    assert_eq!(m.bpm, 300.0);
    assert_eq!(m.in_gain, 20.0);
    assert_eq!(m.out_gain, -12.0);
    assert_eq!(m.sample_tune_semis, 48.0);
    assert_eq!(m.sample_tune_cents, -100.0);
    assert_eq!(m.midi_pitchbend_range, 48.0);
    assert_eq!(e.modulated_lfos()[0].depth, -1.0);
    assert_eq!(e.modulated_envs()[2].release, 1.0);
    assert_eq!(e.preset.params.bpm, 120.0);
}

#[test]
fn effect_targets_follow_slot_ids_and_are_cleared_on_deletion() {
    let a = ColorSlot {
        id: 11,
        kind: Type::LcrDelay,
        ..ColorSlot::default()
    };
    let b = ColorSlot {
        id: 22,
        kind: Type::ModDelay,
        ..ColorSlot::default()
    };
    let mut p = Preset {
        color_chain: vec![a, b],
        ..Preset::default()
    };
    p.matrix = vec![
        route(
            Source::ModWheel,
            D::Color {
                slot: 11,
                parameter: C::LcrHighDamp,
            },
            0.7,
        ),
        route(
            Source::Expression,
            D::Color {
                slot: 22,
                parameter: C::ModDelayDepth,
            },
            -1.0,
        ),
    ];
    let mut e = engine(p);
    on(&mut e, 0, 60, 127);
    let cloud = e.preset.params.cloud;
    e.midi(Message::Control {
        channel: 0,
        controller: 1,
        value: 127,
    });
    advance(&mut e, 64);
    assert_eq!(e.modulated_color_slots()[0].settings.lcr_high_damp, 0.7);
    assert_eq!(e.modulated_color_slots()[1].settings.mod_delay_depth, 0.0);
    assert_eq!(e.preset.params.cloud, cloud);
    let mut p = e.preset.clone();
    p.color_chain.swap(0, 1);
    e.set_preset(p);
    assert_eq!(e.modulated_color_slots()[1].id, 11);
    assert_eq!(e.modulated_color_slots()[1].settings.lcr_high_damp, 0.7);
    let mut p = e.preset.clone();
    p.color_chain.pop();
    e.set_preset(p);
    assert_eq!(e.preset.matrix[0].destination, D::None);
}

#[test]
fn step_note_and_velocity_targets_reach_the_sample_clocked_sequencer() {
    let mut p = Preset::default();
    p.performance.mode = Mode::Sequencer;
    p.performance.steps[0].notes = [60, 64, 67].into();
    p.matrix = vec![
        route(
            Source::Expression,
            D::Step {
                index: 0,
                parameter: StepParameter::Note,
            },
            12.0 / 127.0,
        ),
        route(
            Source::Expression,
            D::Step {
                index: 0,
                parameter: StepParameter::Velocity,
            },
            -0.5,
        ),
    ];
    let mut e = engine(p);
    e.set_playing(true);
    advance(&mut e, 64);
    assert_eq!(e.held_notes(), vec![72, 76, 79]);
    assert_eq!(
        e.preset.performance.steps[0].notes.as_slice(),
        &[60, 64, 67]
    );
    assert!((e.modulation_source(Source::Velocity) - 37.0 / 127.0).abs() < 1e-6);
}

#[test]
fn random_matrix_and_delay_stack_render_identically_across_blocks() {
    let mut p = Preset::default();
    p.performance.mode = Mode::Sequencer;
    p.color_chain = vec![
        ColorSlot {
            id: 11,
            kind: Type::ModDelay,
            ..ColorSlot::default()
        },
        ColorSlot {
            id: 22,
            kind: Type::Reverb,
            ..ColorSlot::default()
        },
    ];
    p.matrix = vec![
        route(Source::RandomNote, D::Pitch, 0.1),
        route(
            Source::Lfo1,
            D::Color {
                slot: 11,
                parameter: C::ModDelayDepth,
            },
            0.2,
        ),
        route(
            Source::Env2,
            D::Color {
                slot: 22,
                parameter: C::ReverbTone,
            },
            -0.3,
        ),
    ];
    let mut a = engine(p.clone());
    let mut b = engine(p);
    a.set_playing(true);
    b.set_playing(true);
    let mut expected: Vec<Stereo> = vec![[0.0; 2]; 16000];
    let mut actual = expected.clone();
    a.process(&mut expected);
    for block in actual.chunks_mut(71) {
        b.process(block);
    }
    assert_eq!(actual, expected);
    assert!(actual.iter().flatten().any(|v| v.abs() > 1e-4));
}
