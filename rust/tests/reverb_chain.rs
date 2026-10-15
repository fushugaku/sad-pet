use sad_pet::{
    Stereo,
    color::{ColorParameter as P, ColorSlot, MAX_COLOR_SLOTS},
    dsp::{
        Engine, SampleBuffer,
        color::{ColorProcessor, ColorRack},
        effects::Effects,
    },
    midi::Message,
    preset::{Destination, Env, Params, Preset, ReverbType, SaturatorType as Type},
};
use std::sync::Arc;

const MODES: [ReverbType; 4] = [
    ReverbType::PlateS,
    ReverbType::PlateL,
    ReverbType::Room,
    ReverbType::Reverse,
];
fn reverb(id: u32, mode: ReverbType) -> ColorSlot {
    ColorSlot::legacy_reverb(
        id,
        &Params {
            reverb_type: mode,
            reverb: 0.7,
            shimmer: 0.25,
            ..Params::default()
        },
    )
}
fn input(i: usize, sr: u32) -> Stereo {
    if i >= sr as usize / 8 {
        return [0.0; 2];
    }
    let t = i as f32 / sr as f32 * std::f32::consts::TAU;
    [(t * 173.0).sin() * 0.3, (t * 311.0).sin() * 0.2]
}
fn audio(slots: &[ColorSlot], sr: u32, n: usize) -> Vec<Stereo> {
    let mut rack = ColorRack::new(sr);
    rack.configure(slots);
    (0..n).map(|i| rack.process(input(i, sr))).collect()
}
fn difference(a: &[Stereo], b: &[Stereo]) -> f32 {
    a.iter()
        .zip(b)
        .map(|(a, b)| (a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2))
        .sum::<f32>()
        / (a.len() * 2) as f32
}

#[test]
fn migration_keeps_eight_color_effects_then_reverb_settings_and_existing_routes() {
    let mut old = Preset {
        color_chain: (1..=8)
            .map(|id| ColorSlot {
                id,
                kind: Type::Chorus,
                ..ColorSlot::default()
            })
            .collect(),
        ..Preset::default()
    };
    old.params.reverb = 0.63;
    old.params.reverb_type = ReverbType::Reverse;
    old.params.predelay = 0.27;
    old.params.reverb_decay = 0.73;
    old.params.reverb_tone = 0.42;
    old.params.shimmer = 0.31;
    old.params.diffuse = 0.61;
    old.lfos[0].destination = Destination::ReverbDecay;
    old.lfos[1].destination = Destination::Color {
        slot: 9,
        parameter: P::ChorusRate,
    };
    old.envs[0].destination = Destination::Reverb;
    old.envs[2].extra_destination = Destination::Shimmer;
    let mut json = serde_json::to_value(&old).unwrap();
    json.as_object_mut().unwrap().remove("fx_chain_version");
    let mut p: Preset = serde_json::from_value(json).unwrap();
    p.sanitize();
    assert_eq!(p.color_chain.len(), 9);
    assert!(p.color_chain[..8].iter().all(|s| s.kind == Type::Chorus));
    let rv = p.color_chain[8];
    assert_eq!(rv, ColorSlot::legacy_reverb(rv.id, &old.params));
    assert_ne!(rv.id, 9); // The new reverb must not revive a dangling route.
    assert_eq!(p.lfos[1].destination, Destination::None);
    assert_eq!(
        p.lfos[0].destination,
        Destination::Color {
            slot: rv.id,
            parameter: P::ReverbDecay
        }
    );
    assert_eq!(
        p.envs[0].destination,
        Destination::Color {
            slot: rv.id,
            parameter: P::Mix
        }
    );
    assert_eq!(
        p.envs[2].extra_destination,
        Destination::Color {
            slot: rv.id,
            parameter: P::ReverbShimmer
        }
    );
    let serialized = serde_json::to_value(&p).unwrap();
    let restored: Preset = serde_json::from_value(serialized.clone()).unwrap();
    assert_eq!(serde_json::to_value(restored).unwrap(), serialized);
    let empty: Preset =
        serde_json::from_str(r#"{"fx_chain_version":1,"color_chain":[],"params":{"reverb":1}}"#)
            .unwrap();
    assert!(empty.color_chain.is_empty());
    let old_empty: Preset =
        serde_json::from_str(r#"{"color_chain":[],"params":{"reverb":0.4}}"#).unwrap();
    assert_eq!(old_empty.color_chain.len(), 1);
    assert_eq!(old_empty.color_chain[0].kind, Type::Reverb);
    assert_eq!(old_empty.color_chain[0].mix, 0.4);
}

#[test]
fn migrated_rack_matches_previous_color_then_shared_reverb_audio() {
    for mode in MODES {
        let params = Params {
            reverb_type: mode,
            reverb: 0.63,
            predelay: 0.17,
            shimmer: 0.32,
            diffuse: 0.6,
            ..Params::default()
        };
        let color = ColorSlot::legacy(Type::Distortion, 0.57);
        let rv = ColorSlot::legacy_reverb(2, &params);
        let mut rack = ColorRack::new(8000);
        rack.configure(&[color, rv]);
        let mut before = ColorProcessor::new(8000);
        before.configure(color);
        let mut legacy = Effects::new(8000);
        legacy.configure(&params);
        for i in 0..12000 {
            let original = legacy.reverb(before.process(input(i, 8000)), &params, params.reverb);
            let actual = rack.process(input(i, 8000));
            for c in 0..2 {
                assert!((actual[c] - original[c]).abs() < 2e-6, "{mode:?}");
            }
        }
    }
}

#[test]
fn repeated_reverbs_are_independent_and_order_relative_to_distortion_matters() {
    let a = reverb(11, ReverbType::Room);
    let mut b = reverb(22, ReverbType::Reverse);
    b.settings.reverb_decay = 0.8;
    b.settings.reverb_predelay = 0.4;
    let rendered = audio(&[a, b], 8000, 12000);
    let mut first = ColorProcessor::new(8000);
    first.configure(a);
    let mut second = ColorProcessor::new(8000);
    second.configure(b);
    for (i, frame) in rendered.iter().enumerate() {
        assert_eq!(*frame, second.process(first.process(input(i, 8000))));
    }
    assert!(difference(&rendered, &audio(&[a], 8000, 12000)) > 1e-7);
    let distortion = ColorSlot {
        id: 33,
        mix: 1.0,
        ..ColorSlot::default()
    };
    assert!(
        difference(
            &audio(&[a, distortion], 8000, 12000),
            &audio(&[distortion, a], 8000, 12000)
        ) > 1e-6
    );
}

#[test]
fn every_reverb_knob_and_algorithm_affects_sound_and_bypass_is_exact() {
    for parameter in [
        P::ReverbPredelay,
        P::ReverbDecay,
        P::ReverbTone,
        P::ReverbShimmer,
        P::ReverbDiffuse,
    ] {
        let mut a = reverb(1, ReverbType::PlateS);
        *a.parameter_mut(parameter) = 0.0;
        let mut b = a;
        *b.parameter_mut(parameter) = 1.0;
        assert!(
            difference(&audio(&[a], 8000, 16000), &audio(&[b], 8000, 16000)) > 1e-9,
            "{parameter:?}"
        );
    }
    let reference = audio(&[reverb(1, MODES[0])], 8000, 12000);
    for mode in MODES {
        let mut s = reverb(1, mode);
        if mode != MODES[0] {
            assert!(difference(&reference, &audio(&[s], 8000, 12000)) > 1e-7);
        }
        for bypass in [false, true] {
            s.enabled = !bypass;
            s.mix = if bypass { 1.0 } else { 0.0 };
            for (i, frame) in audio(&[s], 8000, 4000).iter().enumerate() {
                assert_eq!(*frame, input(i, 8000));
            }
        }
    }
}

#[test]
fn reordering_and_removing_other_slots_preserve_reverb_tails_and_removal_resets_its_state() {
    let rv = reverb(10, ReverbType::Reverse);
    let disabled = ColorSlot {
        id: 20,
        enabled: false,
        ..reverb(20, ReverbType::PlateL)
    };
    let mut reference = ColorRack::new(8000);
    reference.configure(&[rv]);
    let mut rack = ColorRack::new(8000);
    rack.configure(&[rv]);
    for i in 0..2000 {
        assert_eq!(
            rack.process(input(i, 8000)),
            reference.process(input(i, 8000))
        );
    }
    for (round, chain) in [vec![disabled, rv], vec![rv, disabled], vec![rv]]
        .iter()
        .enumerate()
    {
        rack.configure(chain);
        for i in 2000 + 2000 * round..4000 + 2000 * round {
            assert_eq!(
                rack.process(input(i, 8000)),
                reference.process(input(i, 8000))
            );
        }
    }
    rack.configure(&[]);
    rack.configure(&[rv]);
    let mut fresh = ColorRack::new(8000);
    fresh.configure(&[rv]);
    for i in 0..4000 {
        assert_eq!(rack.process(input(i, 8000)), fresh.process(input(i, 8000)));
    }
}

#[test]
fn per_reverb_envelopes_and_lfos_keep_their_targets_when_moved_or_deleted() {
    let sample = Arc::new(
        SampleBuffer::new(
            "tone".into(),
            8000,
            (0..8000).map(|i| input(i, 8000)).collect(),
        )
        .unwrap(),
    );
    let mut a = reverb(11, ReverbType::Room);
    a.settings.reverb_decay = 0.3;
    let mut b = reverb(22, ReverbType::PlateL);
    b.settings.reverb_tone = 0.4;
    b.mix = 0.6;
    let mut p = Preset {
        color_chain: vec![a, b],
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
        parameter: P::ReverbDecay,
    };
    p.envs[0].depth = 0.2;
    p.envs[1].extra_destination = Destination::Color {
        slot: 22,
        parameter: P::ReverbTone,
    };
    p.envs[1].extra_depth = -0.15;
    p.lfos[0].destination = Destination::Color {
        slot: 22,
        parameter: P::Mix,
    };
    p.lfos[0].depth = 0.1;
    let mut e = Engine::new(8000, sample, p);
    e.midi(Message::NoteOn {
        channel: 0,
        note: 60,
        velocity: 127,
    });
    for _ in 0..320 {
        e.process_frame();
    }
    assert!((e.modulated_color_slots()[0].settings.reverb_decay - 0.5).abs() < 1e-6);
    assert!((e.modulated_color_slots()[1].settings.reverb_tone - 0.25).abs() < 1e-6);
    assert!(e.modulated_color_slots()[1].mix > 0.6);
    let mut p = e.preset.clone();
    p.color_chain.swap(0, 1);
    e.set_preset(p);
    assert_eq!(e.modulated_color_slots()[0].id, 22);
    assert!(e.modulated_params().reverb > 0.6);
    assert!((e.modulated_color_slots()[1].settings.reverb_decay - 0.5).abs() < 1e-6);
    e.midi(Message::Control {
        channel: 0,
        controller: 39,
        value: 64,
    });
    assert_eq!(e.preset.params.reverb, e.preset.color_chain[0].mix);
    let mut p = e.preset.clone();
    p.color_chain.retain(|s| s.id != 11);
    e.set_preset(p);
    assert_eq!(e.preset.envs[0].destination, Destination::None);
    assert_eq!(
        e.preset.envs[1].extra_destination,
        Destination::Color {
            slot: 22,
            parameter: P::ReverbTone
        }
    );
}

#[test]
fn twelve_reverbs_are_finite_at_high_resonance_and_multiple_sample_rates() {
    for sr in [8000, 48000, 192000] {
        let slots: Vec<_> = (0..MAX_COLOR_SLOTS)
            .map(|i| {
                let mut s = reverb(i as u32 + 1, MODES[i % 4]);
                s.mix = 1.0;
                s.settings.reverb_decay = 1.0;
                s.settings.reverb_shimmer = 1.0;
                s.settings.reverb_diffuse = 1.0;
                s
            })
            .collect();
        assert!(
            audio(&slots, sr, sr as usize / 4)
                .iter()
                .flatten()
                .all(|v| v.is_finite() && v.abs() < 100.0)
        );
    }
}

#[test]
fn stacked_reverbs_render_identically_across_audio_block_sizes() {
    let p = Preset {
        color_chain: vec![
            reverb(11, ReverbType::Reverse),
            ColorSlot {
                id: 22,
                kind: Type::Flanger,
                ..ColorSlot::default()
            },
            reverb(33, ReverbType::PlateL),
        ],
        ..Preset::default()
    };
    let sample = Arc::new(
        SampleBuffer::new(
            "tone".into(),
            8000,
            (0..8000).map(|i| input(i, 8000)).collect(),
        )
        .unwrap(),
    );
    let mut a = Engine::new(8000, sample.clone(), p.clone());
    let mut b = Engine::new(8000, sample, p);
    a.set_playing(true);
    b.set_playing(true);
    let mut expected = vec![[0.0; 2]; 8000];
    let mut actual = expected.clone();
    a.process(&mut expected);
    for chunk in actual.chunks_mut(71) {
        b.process(chunk);
    }
    assert_eq!(actual, expected);
    assert!(actual.iter().flatten().any(|v| v.abs() > 1e-4));
}
