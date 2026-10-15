use sad_pet::{
    Stereo,
    color::ColorSlot,
    dsp::{Engine, SampleBuffer},
    midi::Message,
    preset::{Env, FilterRouting, Preset, SaturatorType},
};
use std::sync::Arc;

const RATE: u32 = 8_000;
fn source(i: usize) -> Stereo {
    let phase = i as f32 * std::f32::consts::TAU / RATE as f32;
    [(phase * 1000.0).sin() * 0.15, (phase * 1300.0).sin() * 0.12]
}
fn sample(scale: f32) -> Arc<SampleBuffer> {
    Arc::new(
        SampleBuffer::new(
            "Mix test".into(),
            RATE,
            (0..RATE as usize)
                .map(|i| source(i).map(|v| v * scale))
                .collect(),
        )
        .unwrap(),
    )
}
fn preset() -> Preset {
    let mut p = Preset::default();
    p.params.mix = 0.0;
    p.params.grains = 0.0;
    p.params.min_grains = 0.0;
    p.params.filter = 1.0;
    p.params.filter2 = 1.0;
    p.params.filter_env_depth = 0.0;
    p.params.filter2_env_depth = 0.0;
    p.params.master_volume = 0.5;
    p.color_chain.clear();
    p.envs = [Env {
        attack: 0.0,
        decay: 0.0,
        sustain: 1.0,
        release: 0.0,
        ..Env::default()
    }; 3];
    p
}
fn render(p: Preset, scale: f32) -> Vec<Stereo> {
    let mut engine = Engine::new(RATE, sample(scale), p);
    engine.set_playing(true);
    (0..RATE * 2).map(|_| engine.process_frame()).collect()
}
fn energy(frames: &[Stereo]) -> f32 {
    frames.iter().flatten().map(|v| v * v).sum::<f32>() / frames.len() as f32
}
fn difference(a: &[Stereo], b: &[Stereo]) -> f32 {
    a.iter()
        .zip(b)
        .map(|(a, b)| (a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2))
        .sum::<f32>()
        / a.len() as f32
}

#[test]
fn fully_dry_audio_uses_both_filters_in_every_routing_for_files_and_microphone() {
    for mode in [None, Some(true), Some(false)] {
        for routing in [
            FilterRouting::Single,
            FilterRouting::Serial,
            FilterRouting::Parallel,
        ] {
            let run = |closed| {
                let mut p = preset();
                p.params.filter_routing = routing;
                if closed {
                    p.params.filter = if routing == FilterRouting::Serial {
                        1.0
                    } else {
                        0.15
                    };
                    p.params.filter2 = 0.15;
                }
                let mut e = Engine::new(RATE, sample(1.0), p);
                if let Some(effects_only) = mode {
                    e.set_live_input(true, effects_only);
                }
                e.set_playing(true);
                let frames: Vec<_> = (0..RATE as usize)
                    .map(|i| e.process_input_frame(source(i)))
                    .collect();
                energy(&frames[4000..])
            };
            let open = run(false);
            assert!(open > 0.001);
            assert!(
                run(true) < open * 0.001,
                "Dry escaped {routing:?}, input mode {mode:?}"
            );
        }
    }
}

#[test]
fn fully_dry_audio_reaches_every_effect_and_their_order_matters() {
    let base = render(preset(), 1.0);
    let types = [
        SaturatorType::Distortion,
        SaturatorType::Reducer,
        SaturatorType::Crusher,
        SaturatorType::Flanger,
        SaturatorType::Chorus,
        SaturatorType::Binaural,
        SaturatorType::Reverb,
        SaturatorType::LcrDelay,
        SaturatorType::ModDelay,
    ];
    for kind in types {
        let mut p = preset();
        p.color_chain = vec![ColorSlot {
            kind,
            mix: 1.0,
            ..ColorSlot::default()
        }];
        let output = render(p, 1.0);
        assert!(output.iter().flatten().all(|v| v.is_finite()));
        assert!(
            difference(&base[4000..], &output[4000..]) > 1e-7,
            "Dry escaped {kind:?}"
        );
    }
    let mut p = preset();
    let distortion = ColorSlot {
        kind: SaturatorType::Distortion,
        mix: 1.0,
        ..ColorSlot::default()
    };
    let crusher = ColorSlot {
        id: 2,
        kind: SaturatorType::Crusher,
        mix: 1.0,
        ..ColorSlot::default()
    };
    p.color_chain = vec![distortion, crusher];
    let first = render(p.clone(), 1.0);
    p.color_chain.reverse();
    assert!(difference(&first[4000..], &render(p, 1.0)[4000..]) > 1e-7);
}

#[test]
fn blend_happens_before_nonlinear_effects_and_dry_level_does_not_multiply_with_chords() {
    let mut p = preset();
    p.color_chain = vec![ColorSlot {
        mix: 1.0,
        ..ColorSlot::default()
    }];
    let half_source = render(p.clone(), 0.5);
    p.params.mix = 0.5;
    let half_blend = render(p, 1.0);
    assert!(
        difference(&half_source[8000..], &half_blend[8000..]) < 1e-8,
        "with no grains, 50% blend must halve the source before distortion"
    );

    let chord = |notes: &[u8]| {
        let mut e = Engine::new(RATE, sample(1.0), preset());
        e.set_midi_gate(true);
        e.set_playing(true);
        for &note in notes {
            e.midi(Message::NoteOn {
                channel: 0,
                note,
                velocity: 127,
            });
        }
        let mut frames = vec![[0.0; 2]; 8000];
        e.process(&mut frames);
        frames
    };
    assert!(difference(&chord(&[60])[4000..], &chord(&[60, 64, 67, 72])[4000..]) < 1e-12);
}

#[test]
fn reverb_and_delay_tails_remain_audible_at_fully_dry_after_note_release() {
    for kind in [
        SaturatorType::Reverb,
        SaturatorType::LcrDelay,
        SaturatorType::ModDelay,
    ] {
        let mut p = preset();
        p.color_chain = vec![ColorSlot {
            kind,
            mix: 1.0,
            ..ColorSlot::default()
        }];
        let mut e = Engine::new(RATE, sample(1.0), p);
        e.set_playing(true);
        for _ in 0..8000 {
            e.process_frame();
        }
        e.set_playing(false);
        let frames: Vec<_> = (0..4000).map(|_| e.process_frame()).collect();
        assert!(energy(&frames[100..]) > 1e-8, "Dry muted {kind:?} tail");
    }
}

#[test]
fn effects_only_microphone_always_uses_the_same_filter_and_fx_path() {
    let run = |mix| {
        let mut p = preset();
        p.params.mix = mix;
        p.params.filter = 0.6;
        p.color_chain = vec![ColorSlot {
            mix: 1.0,
            ..ColorSlot::default()
        }];
        let mut e = Engine::new(RATE, sample(1.0), p);
        e.set_live_input(true, true);
        e.set_playing(true);
        (0..8000)
            .map(|i| e.process_input_frame(source(i)))
            .collect::<Vec<_>>()
    };
    assert_eq!(run(0.0), run(0.5));
    assert_eq!(run(0.0), run(1.0));

    let run_bypass = |mut p: Preset| {
        p.params.master_volume = 0.5;
        let mut e = Engine::new(RATE, sample(1.0), p);
        e.bypass = true;
        e.set_playing(true);
        (0..8000).map(|_| e.process_frame()).collect::<Vec<_>>()
    };
    let reference = run_bypass(preset());
    let mut altered = preset();
    altered.params.mix = 1.0;
    altered.params.filter = 0.0;
    altered.color_chain = vec![ColorSlot {
        mix: 1.0,
        ..ColorSlot::default()
    }];
    assert_eq!(reference, run_bypass(altered));
}
