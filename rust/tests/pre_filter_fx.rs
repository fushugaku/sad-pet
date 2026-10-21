use sad_pet::{
    Stereo,
    color::{ColorParameter, ColorSettings, ColorSlot},
    dsp::{
        Engine, SampleBuffer,
        radias_waveshaper::{RadiasWaveshaper, WaveshaperMode},
    },
    midi::Message,
    preset::{Destination, Env, Preset, SaturatorType},
};
use std::sync::Arc;

const RATE: u32 = 8000;
fn slot(id: u32, kind: SaturatorType) -> ColorSlot {
    ColorSlot {
        id,
        kind,
        mix: 1.0,
        ..ColorSlot::default()
    }
}
fn preset() -> Preset {
    let mut p = Preset::default();
    p.color_chain.clear();
    p.params.mix = 0.0;
    p.params.filter_env_depth = 0.0;
    p.params.filter_resonance = 0.0;
    p.params.master_volume = 0.65;
    p.envs.fill(Env {
        attack: 0.0,
        decay: 0.0,
        sustain: 1.0,
        release: 0.0,
        ..Env::default()
    });
    p
}
fn sample() -> Arc<SampleBuffer> {
    Arc::new(
        SampleBuffer::new(
            "FX routing".into(),
            RATE,
            (0..RATE)
                .map(|i| {
                    [(i as f32 * 1000.0 * std::f32::consts::TAU / RATE as f32).sin() * 0.75; 2]
                })
                .collect(),
        )
        .unwrap(),
    )
}
fn render(preset: Preset, notes: &[u8]) -> Vec<Stereo> {
    let mut engine = Engine::new(RATE, sample(), preset);
    if notes.is_empty() {
        engine.set_playing(true);
    }
    for &note in notes {
        engine.midi(Message::NoteOn {
            channel: 0,
            note,
            velocity: 127,
        });
    }
    (0..RATE * 2).map(|_| engine.process_frame()).collect()
}
fn difference(a: &[Stereo], b: &[Stereo]) -> f32 {
    a.iter()
        .zip(b)
        .flat_map(|(a, b)| [0, 1].map(|c| (a[c] - b[c]).powi(2)))
        .sum::<f32>()
        / a.len() as f32
}

#[test]
fn old_presets_and_bypassed_pre_racks_preserve_polyphonic_audio() {
    let p = preset();
    let mut value = serde_json::to_value(&p).unwrap();
    value.as_object_mut().unwrap().remove("pre_color_chain");
    let old: Preset = serde_json::from_value(value).unwrap();
    assert!(old.pre_color_chain.is_empty());
    for notes in [&[][..], &[60][..], &[48, 60, 64, 67][..]] {
        let base = render(old.clone(), notes);
        for mix in [0.0, 1.0] {
            let mut bypassed = old.clone();
            let mut fx = slot(23, SaturatorType::Waveshaper);
            fx.mix = mix;
            fx.enabled = mix == 0.0;
            bypassed.pre_color_chain = vec![fx];
            assert_eq!(base, render(bypassed, notes));
        }
    }
}

#[test]
fn pre_effects_are_before_the_filters_and_can_use_the_whole_catalog() {
    let kinds = [
        SaturatorType::Distortion,
        SaturatorType::Reducer,
        SaturatorType::Crusher,
        SaturatorType::Flanger,
        SaturatorType::Chorus,
        SaturatorType::Binaural,
        SaturatorType::Reverb,
        SaturatorType::LcrDelay,
        SaturatorType::ModDelay,
        SaturatorType::Compressor,
        SaturatorType::Waveshaper,
    ];
    let base = render(preset(), &[]);
    for kind in kinds {
        let mut p = preset();
        p.pre_color_chain = vec![slot(31, kind)];
        let output = render(p, &[]);
        assert!(output.iter().flatten().all(|v| v.is_finite()), "{kind:?}");
        assert!(
            difference(&base[8000..], &output[8000..]) > 1e-8,
            "Pre rack missed {kind:?}"
        );
    }
    let mut pre = preset();
    pre.params.filter = 0.57;
    pre.pre_color_chain = vec![slot(31, SaturatorType::Distortion)];
    pre.pre_color_chain[0].settings.distortion_drive = 1.0;
    let mut post = pre.clone();
    post.color_chain = std::mem::take(&mut post.pre_color_chain);
    let a = render(pre, &[]);
    let b = render(post, &[]);
    let harmonic = |frames: &[Stereo], hz: f32| {
        let (mut re, mut im) = (0.0_f64, 0.0_f64);
        for (i, frame) in frames.iter().enumerate() {
            let phase = std::f64::consts::TAU * hz as f64 * i as f64 / RATE as f64;
            re += frame[0] as f64 * phase.cos();
            im += frame[0] as f64 * phase.sin();
        }
        re.hypot(im)
    };
    let pre_ratio = harmonic(&a[8000..], 3000.0) / harmonic(&a[8000..], 1000.0);
    let post_ratio = harmonic(&b[8000..], 3000.0) / harmonic(&b[8000..], 1000.0);
    assert!(
        pre_ratio < post_ratio * 0.5,
        "filter must remove distortion harmonics: pre={pre_ratio}, post={post_ratio}"
    );
}

#[test]
fn pre_and_post_ids_modulation_meters_and_preset_recall_are_independent() {
    let mut p = preset();
    p.color_chain = vec![slot(7, SaturatorType::Compressor)];
    p.pre_color_chain = vec![
        slot(7, SaturatorType::Waveshaper),
        slot(0, SaturatorType::Compressor),
    ];
    p.sanitize();
    let id = p.pre_color_chain[0].id;
    assert_ne!(id, 7);
    assert_ne!(id, p.pre_color_chain[1].id);
    let mut restored: Preset = serde_json::from_slice(&serde_json::to_vec(&p).unwrap()).unwrap();
    restored.pre_color_chain[0].settings.waveshaper_type = WaveshaperMode::Pickup;
    restored.pre_color_chain[0].settings.waveshaper_depth = 0.3;
    restored.envs[0].destination = Destination::Color {
        slot: id,
        parameter: ColorParameter::WaveshaperDepth,
    };
    restored.envs[0].depth = 0.4;
    let mut engine = Engine::new(RATE, sample(), restored.clone());
    engine.midi(Message::NoteOn {
        channel: 0,
        note: 60,
        velocity: 100,
    });
    for _ in 0..1000 {
        engine.process_frame();
    }
    assert!(
        (engine.modulated_pre_color_slots()[0]
            .settings
            .waveshaper_depth
            - 0.7)
            .abs()
            < 1e-4
    );
    assert_eq!(
        engine.preset.pre_color_chain[0].settings.waveshaper_depth,
        0.3
    );
    assert_eq!(
        engine.preset.pre_color_chain[0].settings.waveshaper_type,
        WaveshaperMode::Pickup
    );
    assert!(engine.knob_states()[&format!("color:{id}:waveshaper_depth")].modulated);
    let meters = engine.gain_reduction();
    assert!(meters.contains_key(&7));
    assert!(meters.contains_key(&restored.pre_color_chain[1].id));
}

#[test]
fn all_radias_modes_retain_depth_and_stereo_state_at_host_sample_rates() {
    for rate in [8000, 44100, 48000, 96000] {
        for mode in WaveshaperMode::ALL {
            let run = |depth: f32, other: f32| {
                let mut ws = RadiasWaveshaper::new(rate);
                ws.configure(mode, depth);
                ws.set_pitch(69.0);
                (0..4096)
                    .map(|i| {
                        let frame = ws.process([(i as f32 * 0.12).sin() * 0.6, other]);
                        assert!(
                            frame.iter().all(|v| v.is_finite() && v.abs() <= 1.0),
                            "{mode:?}"
                        );
                        frame
                    })
                    .collect::<Vec<_>>()
            };
            let low = run(0.0, 0.0);
            let high = run(1.0, 0.0);
            assert!(
                difference(&low[1000..], &high[1000..]) > 1e-10,
                "Depth missed {mode:?} at {rate}"
            );
            let stereo = run(1.0, 0.4);
            assert!(
                high.iter().zip(stereo).all(|(a, b)| a[0] == b[0]),
                "Shared channel state in {mode:?}"
            );
        }
    }
    let settings = ColorSettings {
        waveshaper_depth: f32::NAN,
        ..ColorSettings::default()
    };
    let mut fx = ColorSlot {
        settings,
        ..slot(3, SaturatorType::Waveshaper)
    };
    fx.sanitize();
    assert_eq!(
        fx.settings.waveshaper_depth,
        ColorSettings::default().waveshaper_depth
    );
}

#[test]
fn post_sub_oscillators_follow_note_envelopes_and_stay_silent_without_notes() {
    for mode in [
        WaveshaperMode::SubSaw,
        WaveshaperMode::SubSquare,
        WaveshaperMode::SubTriangle,
        WaveshaperMode::SubSine,
    ] {
        let mut p = preset();
        let mut fx = slot(13, SaturatorType::Waveshaper);
        fx.settings.waveshaper_type = mode;
        fx.settings.waveshaper_depth = 1.0;
        p.color_chain = vec![fx];
        let mut engine = Engine::new(RATE, sample(), p);
        for _ in 0..1000 {
            assert_eq!(engine.process_frame(), [0.0; 2]);
        }
        engine.midi(Message::NoteOn {
            channel: 0,
            note: 60,
            velocity: 110,
        });
        assert!((0..1000).any(|_| engine.process_frame()[0].abs() > 0.001));
        engine.midi(Message::NoteOff {
            channel: 0,
            note: 60,
            velocity: 0,
        });
        for _ in 0..RATE {
            engine.process_frame();
        }
        for _ in 0..1000 {
            assert_eq!(engine.process_frame(), [0.0; 2], "{mode:?}");
        }
    }
}
