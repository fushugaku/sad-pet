use sad_pet::{
    Stereo,
    color::{ColorParameter as P, ColorSlot, MAX_COLOR_SLOTS},
    dsp::{
        Engine, SampleBuffer,
        color::{ColorProcessor, ColorRack},
    },
    midi::Message,
    preset::{Destination, Env, Preset, SaturatorType as Type},
};
use std::sync::Arc;

const TYPES: [Type; 7] = [
    Type::Distortion,
    Type::Reducer,
    Type::Crusher,
    Type::Flanger,
    Type::Chorus,
    Type::Binaural,
    Type::Compressor,
];
const PARAMETERS: [(Type, P); 28] = [
    (Type::Distortion, P::DistortionDrive),
    (Type::Distortion, P::DistortionTone),
    (Type::Distortion, P::DistortionBias),
    (Type::Reducer, P::ReducerRate),
    (Type::Reducer, P::ReducerJitter),
    (Type::Reducer, P::ReducerSmooth),
    (Type::Crusher, P::CrusherBits),
    (Type::Crusher, P::CrusherDrive),
    (Type::Crusher, P::CrusherDither),
    (Type::Flanger, P::FlangerRate),
    (Type::Flanger, P::FlangerDepth),
    (Type::Flanger, P::FlangerDelay),
    (Type::Flanger, P::FlangerFeedback),
    (Type::Flanger, P::FlangerStereo),
    (Type::Chorus, P::ChorusRate),
    (Type::Chorus, P::ChorusDepth),
    (Type::Chorus, P::ChorusDelay),
    (Type::Chorus, P::ChorusFeedback),
    (Type::Chorus, P::ChorusStereo),
    (Type::Binaural, P::BinauralDelay),
    (Type::Binaural, P::BinauralCrossfeed),
    (Type::Binaural, P::BinauralWidth),
    (Type::Compressor, P::CompressorThreshold),
    (Type::Compressor, P::CompressorRatio),
    (Type::Compressor, P::CompressorAttack),
    (Type::Compressor, P::CompressorRelease),
    (Type::Compressor, P::CompressorKnee),
    (Type::Compressor, P::CompressorMakeup),
];
fn slot(id: u32, kind: Type) -> ColorSlot {
    ColorSlot {
        id,
        kind,
        mix: 1.0,
        ..ColorSlot::default()
    }
}
fn input(i: usize, sr: u32) -> Stereo {
    let t = i as f32 / sr as f32 * std::f32::consts::TAU;
    [
        (t * 137.0).sin() * 0.3 + (t * 1711.0).sin() * 0.08,
        (t * 211.0).sin() * 0.24 + (t * 2333.0).sin() * 0.07,
    ]
}
fn audio(config: &[ColorSlot], sr: u32, frames: usize) -> Vec<Stereo> {
    let mut rack = ColorRack::new(sr);
    rack.configure(config);
    (0..frames).map(|i| rack.process(input(i, sr))).collect()
}
fn difference(a: &[Stereo], b: &[Stereo]) -> f32 {
    a.iter()
        .zip(b)
        .map(|(a, b)| (a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2))
        .sum::<f32>()
        / (a.len() * 2) as f32
}

#[test]
fn empty_chain_bypass_and_zero_mix_preserve_dry_samples_exactly() {
    for kind in TYPES {
        let mut s = slot(1, kind);
        for disabled in [false, true] {
            s.enabled = !disabled;
            s.mix = if disabled { 1.0 } else { 0.0 };
            let rendered = audio(&[s], 8000, 4000);
            for (i, frame) in rendered.iter().enumerate() {
                assert_eq!(*frame, input(i, 8000));
            }
        }
    }
    for (i, frame) in audio(&[], 8000, 4000).iter().enumerate() {
        assert_eq!(*frame, input(i, 8000));
    }
}

#[test]
fn every_algorithm_parameter_changes_rendered_audio() {
    for (kind, parameter) in PARAMETERS {
        let mut low = slot(1, kind);
        *low.parameter_mut(parameter) = 0.0;
        let mut high = low;
        *high.parameter_mut(parameter) = 1.0;
        let a = audio(&[low], 8000, 16000);
        let b = audio(&[high], 8000, 16000);
        assert!(
            difference(&a, &b) > 1e-9,
            "{kind:?} {parameter:?} has no audible DSP effect"
        );
    }
}

#[test]
fn chain_is_sequential_and_changing_order_changes_audio() {
    let distortion = slot(1, Type::Distortion);
    let mut crusher = slot(2, Type::Crusher);
    crusher.settings.crusher_bits = 0.08;
    let a = audio(&[distortion, crusher], 8000, 8000);
    let b = audio(&[crusher, distortion], 8000, 8000);
    assert!(difference(&a, &b) > 1e-5);
    let mut first = ColorProcessor::new(8000);
    let mut second = ColorProcessor::new(8000);
    first.configure(distortion);
    second.configure(crusher);
    for (i, frame) in a.iter().enumerate() {
        assert_eq!(*frame, second.process(first.process(input(i, 8000))));
    }
}

#[test]
fn repeated_effects_have_independent_buffers_settings_and_random_streams() {
    let a = slot(1, Type::Chorus);
    let mut b = slot(2, Type::Chorus);
    b.settings.chorus_delay = 0.9;
    b.settings.chorus_rate = 0.85;
    let result = audio(&[a, b], 8000, 8000);
    let mut first = ColorProcessor::new(8000);
    let mut second = ColorProcessor::new(8000);
    first.configure(a);
    second.configure(b);
    for (i, frame) in result.iter().enumerate() {
        assert_eq!(*frame, second.process(first.process(input(i, 8000))));
    }
    let mut a = slot(11, Type::Reducer);
    a.settings.reducer_jitter = 1.0;
    let mut b = a;
    b.id = 12;
    assert!(difference(&audio(&[a], 8000, 8000), &audio(&[b], 8000, 8000)) > 1e-4);
}

#[test]
fn insertion_move_and_removal_preserve_other_slots_delay_tails() {
    let chorus = slot(10, Type::Chorus);
    let bypass = ColorSlot {
        id: 20,
        enabled: false,
        ..slot(20, Type::Flanger)
    };
    let mut reference = ColorRack::new(8000);
    let mut rack = ColorRack::new(8000);
    reference.configure(&[chorus]);
    rack.configure(&[chorus]);
    for i in 0..2000 {
        assert_eq!(
            rack.process(input(i, 8000)),
            reference.process(input(i, 8000))
        );
    }
    for (round, config) in [
        [bypass, chorus].to_vec(),
        [chorus, bypass].to_vec(),
        [chorus].to_vec(),
    ]
    .iter()
    .enumerate()
    {
        rack.configure(config);
        for i in 2000 + round * 2000..4000 + round * 2000 {
            assert_eq!(
                rack.process(input(i, 8000)),
                reference.process(input(i, 8000))
            );
        }
    }
    rack.configure(&[]);
    rack.configure(&[chorus]);
    let mut fresh = ColorRack::new(8000);
    fresh.configure(&[chorus]);
    for i in 0..2000 {
        assert_eq!(rack.process(input(i, 8000)), fresh.process(input(i, 8000)));
    }
}

#[test]
fn asymmetric_distortion_removes_dc_and_binaural_width_can_make_mono() {
    let mut s = slot(1, Type::Distortion);
    s.settings.distortion_bias = 1.0;
    let mut fx = ColorProcessor::new(8000);
    fx.configure(s);
    for _ in 0..24000 {
        fx.process([0.2; 2]);
    }
    assert!(fx.process([0.2; 2]).iter().all(|v| v.abs() < 1e-5));
    let mut s = slot(1, Type::Binaural);
    s.settings.binaural_width = 0.0;
    assert!(audio(&[s], 8000, 8000).iter().all(|f| f[0] == f[1]));
}

#[test]
fn presets_save_chain_order_inactive_settings_and_target_ids_and_repair_bad_values() {
    let mut p = Preset {
        color_chain: vec![slot(11, Type::Chorus), slot(22, Type::Distortion)],
        ..Preset::default()
    };
    p.color_chain[0].settings.flanger_rate = 0.87;
    p.lfos[1].destination = Destination::Color {
        slot: 11,
        parameter: P::ChorusDepth,
    };
    p.envs[2].extra_destination = Destination::Color {
        slot: 22,
        parameter: P::DistortionBias,
    };
    p.sanitize();
    let serialized = serde_json::to_value(&p).unwrap();
    let restored: Preset = serde_json::from_value(serialized.clone()).unwrap();
    assert_eq!(serde_json::to_value(restored).unwrap(), serialized);
    p.color_chain = (0..12).map(|_| slot(1, Type::Flanger)).collect();
    p.color_chain[0].id = 0;
    p.color_chain[0].mix = f32::NAN;
    p.color_chain[0].settings.flanger_delay = f32::INFINITY;
    p.color_chain[1].settings.flanger_feedback = -3.0;
    p.sanitize();
    assert_eq!(p.color_chain.len(), MAX_COLOR_SLOTS);
    let mut ids: Vec<_> = p.color_chain.iter().map(|s| s.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), MAX_COLOR_SLOTS);
    assert!(!ids.contains(&0));
    assert_eq!(p.color_chain[0].mix, 0.5);
    assert_eq!(
        p.color_chain[0].settings.flanger_delay,
        ColorSlot::default().settings.flanger_delay
    );
    assert_eq!(p.color_chain[1].settings.flanger_feedback, 0.0);
    assert_eq!(p.lfos[1].destination, Destination::None);
    assert_eq!(p.envs[2].extra_destination, Destination::None);
}

#[test]
fn legacy_color_and_reverb_migrate_and_new_empty_chain_stays_empty() {
    for (kind, name) in TYPES.into_iter().zip([
        "distortion",
        "reducer",
        "crusher",
        "flanger",
        "chorus",
        "binaural",
        "compressor",
    ]) {
        let p: Preset = serde_json::from_value(
            serde_json::json!({"params":{"saturator_type":name,"saturation":0.73}}),
        )
        .unwrap();
        assert_eq!(p.color_chain.len(), 2);
        assert_eq!(p.color_chain[0], ColorSlot::legacy(kind, 0.73));
        assert_eq!(p.color_chain[1].kind, Type::Reverb);
    }
    let p: Preset = serde_json::from_str(
        r#"{"fx_chain_version":1,"params":{"saturation":0.8},"color_chain":[]}"#,
    )
    .unwrap();
    assert!(p.color_chain.is_empty());
}

#[test]
fn envelopes_and_lfos_follow_slot_ids_through_reordering_and_deleted_targets_are_removed() {
    let sample = Arc::new(
        SampleBuffer::new(
            "tone".into(),
            8000,
            (0..8000).map(|i| input(i, 8000)).collect(),
        )
        .unwrap(),
    );
    let mut p = Preset {
        color_chain: vec![
            slot(11, Type::Flanger),
            ColorSlot {
                mix: 0.3,
                ..slot(22, Type::Crusher)
            },
        ],
        ..Preset::default()
    };
    p.envs = [Env {
        attack: 0.0,
        decay: 0.0,
        sustain: 1.0,
        release: 0.0,
        ..Env::default()
    }; 3];
    p.envs[0].destination = Destination::Color {
        slot: 11,
        parameter: P::FlangerRate,
    };
    p.envs[0].depth = 0.1;
    p.envs[2].extra_destination = Destination::Color {
        slot: 22,
        parameter: P::Mix,
    };
    p.envs[2].extra_depth = 0.4;
    p.lfos[0].destination = Destination::Color {
        slot: 11,
        parameter: P::FlangerDelay,
    };
    p.lfos[0].depth = 0.2;
    let rate = p.color_chain[0].settings.flanger_rate;
    let delay = p.color_chain[0].settings.flanger_delay;
    let mut e = Engine::new(8000, sample, p);
    e.midi(Message::NoteOn {
        channel: 0,
        note: 60,
        velocity: 127,
    });
    for _ in 0..320 {
        e.process_frame();
    }
    assert!((e.modulated_color_slots()[0].settings.flanger_rate - rate - 0.1).abs() < 1e-5);
    assert!(e.modulated_color_slots()[0].settings.flanger_delay > delay);
    assert!((e.modulated_color_slots()[1].mix - 0.7).abs() < 1e-5);
    let mut p = e.preset.clone();
    p.color_chain.swap(0, 1);
    e.set_preset(p);
    assert_eq!(e.modulated_color_slots()[0].id, 22);
    assert!((e.modulated_params().saturation - 0.7).abs() < 1e-5);
    assert!((e.modulated_color_slots()[1].settings.flanger_rate - rate - 0.1).abs() < 1e-5);
    let mut p = e.preset.clone();
    p.color_chain.retain(|s| s.id != 11);
    e.set_preset(p);
    assert_eq!(e.preset.envs[0].destination, Destination::None);
    assert_eq!(e.preset.lfos[0].destination, Destination::None);
}

#[test]
fn full_rack_stays_finite_at_extremes_and_renders_independently_of_block_size() {
    for sr in [8000, 48000, 192000] {
        for extreme in [0.0, 1.0] {
            let slots: Vec<_> = (0..MAX_COLOR_SLOTS)
                .map(|i| {
                    let kind = TYPES[i % 6];
                    let mut s = slot(i as u32 + 1, kind);
                    for (_, parameter) in PARAMETERS {
                        *s.parameter_mut(parameter) = extreme;
                    }
                    s
                })
                .collect();
            let rendered = audio(&slots, sr, sr as usize / 2);
            assert!(
                rendered
                    .iter()
                    .flatten()
                    .all(|x| x.is_finite() && x.abs() < 100.0)
            );
        }
    }
    let mut a = slot(1, Type::Reducer);
    a.settings.reducer_jitter = 0.8;
    let mut b = slot(2, Type::Crusher);
    b.settings.crusher_dither = 0.7;
    let p = Preset {
        color_chain: vec![a, b, slot(3, Type::Chorus)],
        ..Preset::default()
    };
    let source = Arc::new(
        SampleBuffer::new(
            "tone".into(),
            8000,
            (0..8000).map(|i| input(i, 8000)).collect(),
        )
        .unwrap(),
    );
    let mut a = Engine::new(8000, source.clone(), p.clone());
    let mut b = Engine::new(8000, source, p);
    a.set_playing(true);
    b.set_playing(true);
    let mut expected = vec![[0.0; 2]; 8000];
    let mut actual = expected.clone();
    a.process(&mut expected);
    for chunk in actual.chunks_mut(71) {
        b.process(chunk);
    }
    assert_eq!(actual, expected);
}
