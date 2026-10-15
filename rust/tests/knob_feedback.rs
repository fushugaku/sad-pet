use sad_pet::{
    color::{ColorParameter as C, ColorSlot},
    dsp::{Engine, SampleBuffer},
    matrix::{LfoParameter, Parameter, Route, Source},
    midi::Message,
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
fn route(source: Source, destination: D, depth: f32) -> Route {
    Route {
        source,
        destination,
        depth,
        enabled: true,
    }
}
fn advance(e: &mut Engine, n: usize) {
    for _ in 0..n {
        e.process_frame();
    }
}
fn close(a: f32, b: f32) {
    assert!((a - b).abs() < 1e-5, "{a} != {b}");
}

#[test]
fn matrix_lfo_and_envelope_ranges_combine_and_clip_in_parameter_space() {
    let mut p = Preset::default();
    p.params.pitch = 0.5;
    p.lfos[0].destination = D::Pitch;
    p.lfos[0].depth = 0.2;
    p.envs[1].destination = D::Pitch;
    p.envs[1].depth = 0.3;
    p.matrix = vec![route(Source::RandomNote, D::Pitch, 0.05)];
    let mut e = Engine::new(8000, sample(), p);
    e.set_playing(true);
    for _ in 0..100 {
        advance(&mut e, 128);
        let state = e.knob_states()["pitch"];
        close(state.min, 0.35);
        close(state.max, 0.95);
        close(state.current, e.modulated_params().pitch);
        close(state.base, 0.5);
        assert!(state.modulated && state.current >= state.min && state.current <= state.max);
    }
    let mut p = e.preset.clone();
    p.params.pitch = 0.95;
    p.matrix[0].depth = 1.0;
    e.set_preset(p);
    let s = e.knob_states()["pitch"];
    close(s.min, 0.0);
    close(s.max, 1.0);
}

#[test]
fn correlated_matrix_routes_cancel_and_negative_unipolar_ranges_are_one_sided() {
    let mut p = Preset {
        matrix: vec![
            route(Source::Lfo1, D::Pitch, 0.5),
            route(Source::Lfo1, D::Pitch, -0.5),
        ],
        ..Preset::default()
    };
    let mut e = Engine::new(8000, sample(), p.clone());
    advance(&mut e, 64);
    let s = e.knob_states()["pitch"];
    assert!(!s.modulated);
    close(s.min, 0.5);
    close(s.max, 0.5);
    p.matrix = vec![route(Source::Velocity, D::Pitch, -0.2)];
    e.set_preset(p);
    let s = e.knob_states()["pitch"];
    close(s.min, 0.3);
    close(s.max, 0.5);
    e.midi(Message::NoteOn {
        channel: 0,
        note: 60,
        velocity: 127,
    });
    advance(&mut e, 64);
    close(e.knob_states()["pitch"].current, 0.3);
}

#[test]
fn every_visible_effect_knob_has_a_stable_key_and_correct_mapping() {
    let kinds = [
        Type::Distortion,
        Type::Reducer,
        Type::Crusher,
        Type::Flanger,
        Type::Chorus,
        Type::Binaural,
        Type::Reverb,
        Type::LcrDelay,
        Type::ModDelay,
        Type::LcrDelay,
        Type::ModDelay,
    ];
    let p = Preset {
        color_chain: kinds
            .iter()
            .enumerate()
            .map(|(i, &kind)| {
                let mut s = ColorSlot {
                    id: i as u32 + 11,
                    kind,
                    ..ColorSlot::default()
                };
                if i >= 9 {
                    s.settings.lcr_sync = false;
                    s.settings.mod_delay_sync = false;
                }
                s
            })
            .collect(),
        ..Preset::default()
    };
    let mut e = Engine::new(8000, sample(), p);
    for slot in e.preset.color_chain.clone() {
        for &parameter in slot.control_parameters() {
            assert_eq!(
                serde_json::to_value(parameter).unwrap().as_str().unwrap(),
                parameter.key()
            );
            let mut p = e.preset.clone();
            p.matrix = vec![route(
                Source::Lfo1,
                D::Color {
                    slot: slot.id,
                    parameter,
                },
                0.25,
            )];
            e.set_preset(p);
            advance(&mut e, 64);
            let key = format!("color:{}:{}", slot.id, parameter.key());
            let s = e.knob_states()[&key];
            let mut base = slot;
            close(s.min, (*base.parameter_mut(parameter) - 0.25).max(0.0));
            close(s.max, (*base.parameter_mut(parameter) + 0.25).min(1.0));
            let mut actual = *e
                .modulated_color_slots()
                .iter()
                .find(|s| s.id == slot.id)
                .unwrap();
            close(s.current, *actual.parameter_mut(parameter));
        }
    }
}

#[test]
fn filter_feedback_includes_dedicated_envelopes_and_keyboard_tracking() {
    let mut p = Preset::default();
    p.params.filter = 0.4;
    p.params.filter_env_depth = -0.25;
    p.envs[1] = Env {
        attack: 0.0,
        decay: 0.0,
        sustain: 1.0,
        ..Env::default()
    };
    let mut e = Engine::new(8000, sample(), p);
    e.midi(Message::NoteOn {
        channel: 0,
        note: 60,
        velocity: 127,
    });
    advance(&mut e, 800);
    let s = e.knob_states()["filter"];
    close(s.min, 0.15);
    close(s.max, 0.4);
    close(s.current, e.voice_filter_cutoffs(60).unwrap()[0]);
    close(s.current, 0.15);
    let mut p = e.preset.clone();
    p.params.filter_env_depth = 0.0;
    p.params.filter_keytrack = 1.0;
    e.set_preset(p);
    e.midi(Message::NoteOn {
        channel: 0,
        note: 84,
        velocity: 127,
    });
    advance(&mut e, 64);
    let s = e.knob_states()["filter"];
    close(s.current, e.voice_filter_cutoffs(84).unwrap()[0]);
    close(
        s.current,
        0.4 + 2.0 * std::f32::consts::LN_2 / 1000.0_f32.ln(),
    );
    assert!(s.min < 0.1 && s.max > 0.9);
}

#[test]
fn generator_depth_modulation_expands_the_destination_range() {
    let mut p = Preset::default();
    p.params.cloud = 0.5;
    p.lfos[0].destination = D::Cloud;
    p.lfos[0].depth = 0.0;
    p.lfos[0].waveform = Waveform::Square;
    p.matrix = vec![route(
        Source::Expression,
        D::Lfo {
            index: 0,
            parameter: LfoParameter::Depth,
        },
        0.5,
    )];
    let mut e = Engine::new(8000, sample(), p);
    advance(&mut e, 64);
    let s = e.knob_states()["cloud"];
    close(s.min, 0.0);
    close(s.max, 1.0);
    close(s.current, 1.0);
    let mut p = e.preset.clone();
    p.matrix[0].enabled = false;
    e.set_preset(p);
    assert!(!e.knob_states()["cloud"].modulated);
}

#[test]
fn moving_effects_keeps_feedback_ids_and_removing_them_removes_keys() {
    let mut p = Preset {
        color_chain: vec![
            ColorSlot {
                id: 11,
                kind: Type::Reverb,
                ..ColorSlot::default()
            },
            ColorSlot {
                id: 22,
                kind: Type::ModDelay,
                ..ColorSlot::default()
            },
        ],
        ..Preset::default()
    };
    p.matrix = vec![route(
        Source::Env2,
        D::Color {
            slot: 22,
            parameter: C::ModDelayDepth,
        },
        0.3,
    )];
    let mut e = Engine::new(8000, sample(), p);
    let before = e.knob_states()["color:22:mod_delay_depth"];
    let mut p = e.preset.clone();
    p.color_chain.swap(0, 1);
    e.set_preset(p);
    let after = e.knob_states()["color:22:mod_delay_depth"];
    close(before.min, after.min);
    close(before.max, after.max);
    let mut p = e.preset.clone();
    p.color_chain.remove(0);
    e.set_preset(p);
    assert!(!e.knob_states().contains_key("color:22:mod_delay_depth"));
}

#[test]
fn reading_feedback_does_not_change_audio_presets_or_random_generators() {
    let p = Preset {
        matrix: vec![
            route(Source::RandomNote, D::Pitch, 0.1),
            route(
                Source::Lfo1,
                D::Parameter {
                    parameter: Parameter::FilterEnvDepth,
                },
                0.2,
            ),
        ],
        ..Preset::default()
    };
    let mut a = Engine::new(8000, sample(), p.clone());
    let mut b = Engine::new(8000, sample(), p);
    a.set_playing(true);
    b.set_playing(true);
    for i in 0..16000 {
        if i % 128 == 0 {
            let _ = a.knob_states();
        }
        assert_eq!(a.process_frame(), b.process_frame());
    }
    assert_eq!(
        serde_json::to_value(&a.preset).unwrap(),
        serde_json::to_value(&b.preset).unwrap()
    );
}
