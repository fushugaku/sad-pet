use sad_pet::{
    dsp::{
        Engine, SampleBuffer,
        filter::{FilterBank, cutoff_hz, voice_cutoffs},
    },
    midi::Message,
    preset::{Destination, Env, EnvSource, FilterRouting, FilterType, Params, Preset},
};
use std::sync::Arc;

fn params() -> Params {
    Params {
        filter_env_depth: 0.0,
        filter2_env_depth: 0.0,
        filter_resonance: 0.0,
        filter2_resonance: 0.0,
        ..Params::default()
    }
}
fn response(p: Params, hz: f32) -> f32 {
    let mut bank = FilterBank::new(48000);
    bank.configure(&p, 60.0, [0.0; 3]);
    let mut energy = 0.0;
    for i in 0..12000 {
        let input = (i as f32 * std::f32::consts::TAU * hz / 48000.0).sin() * 0.1;
        let output = bank.process([input; 2]);
        assert!(output.iter().all(|x| x.is_finite()));
        if i >= 6000 {
            energy += output[0] * output[0];
        }
    }
    (energy / 6000.0).sqrt()
}

#[test]
fn single_uses_only_first_filter_and_parallel_preserves_open_level() {
    let mut p = params();
    let reference = response(p, 900.0);
    p.filter2 = 0.0;
    p.vcf2_type = FilterType::HighPass;
    assert_eq!(response(p, 900.0), reference);
    p.filter2 = 1.0;
    p.vcf2_type = FilterType::LowPass;
    for routing in [FilterRouting::Serial, FilterRouting::Parallel] {
        p.filter_routing = routing;
        assert!((response(p, 900.0) - reference).abs() < 1e-6);
    }
}

#[test]
fn serial_cascades_attenuation_and_parallel_is_average_of_independent_paths() {
    let mut p = params();
    p.filter = 0.5;
    p.filter2 = 0.5;
    let single = response(p, 4000.0);
    p.filter_routing = FilterRouting::Serial;
    assert!(response(p, 4000.0) < single * 0.1);
    p.filter_routing = FilterRouting::Parallel;
    assert!((response(p, 4000.0) - single).abs() < 1e-6);
    p.filter2 = 1.0;
    let mut parallel = FilterBank::new(48000);
    parallel.configure(&p, 60.0, [0.0; 3]);
    let mut first = FilterBank::new(48000);
    p.filter_routing = FilterRouting::Single;
    first.configure(&p, 60.0, [0.0; 3]);
    let mut second = FilterBank::new(48000);
    p.filter = 1.0;
    second.configure(&p, 60.0, [0.0; 3]);
    for i in 0..12000 {
        let input = [(i as f32 * std::f32::consts::TAU * 4000.0 / 48000.0).sin() * 0.1; 2];
        let a = first.process(input);
        let b = second.process(input);
        let mixed = parallel.process(input);
        for c in 0..2 {
            assert!((mixed[c] - (a[c] + b[c]) * 0.5).abs() < 1e-6);
        }
    }
}

#[test]
fn low_high_and_bandpass_have_expected_frequency_response() {
    let mut p = params();
    p.filter = 0.5;
    let low = response(p, 100.0);
    let high = response(p, 6000.0);
    assert!(low > high * 50.0);
    p.vcf_type = FilterType::HighPass;
    assert!(response(p, 6000.0) > response(p, 100.0) * 30.0);
    p.vcf_type = FilterType::BandPass;
    assert!(response(p, 630.0) > response(p, 50.0) * 4.0);
    assert!(response(p, 630.0) > response(p, 10000.0) * 4.0);
}

#[test]
fn each_filter_tracks_octaves_from_c4_even_for_comb() {
    for kind in [FilterType::LowPass, FilterType::Comb] {
        let mut p = params();
        p.vcf_type = kind;
        p.vcf2_type = kind;
        p.filter = 0.5;
        p.filter2 = 0.5;
        p.filter_keytrack = 1.0;
        p.filter2_keytrack = 0.5;
        let c4 = voice_cutoffs(&p, 60.0, [0.0; 3]);
        let c5 = voice_cutoffs(&p, 72.0, [0.0; 3]);
        let c3 = voice_cutoffs(&p, 48.0, [0.0; 3]);
        assert!((cutoff_hz(kind, c5[0]) / cutoff_hz(kind, c4[0]) - 2.0).abs() < 1e-5);
        assert!((cutoff_hz(kind, c3[0]) / cutoff_hz(kind, c4[0]) - 0.5).abs() < 1e-5);
        assert!((cutoff_hz(kind, c5[1]) / cutoff_hz(kind, c4[1]) - 2_f32.sqrt()).abs() < 1e-5);
        p.filter_keytrack = 0.0;
        assert_eq!(voice_cutoffs(&p, 127.0, [0.0; 3])[0], 0.5);
    }
}

#[test]
fn independent_envelope_sources_signed_depth_and_clamping() {
    let mut p = params();
    p.filter = 0.5;
    p.filter2 = 0.5;
    p.filter_env = EnvSource::Env1;
    p.filter2_env = EnvSource::Env3;
    p.filter_env_depth = 0.4;
    p.filter2_env_depth = -0.6;
    let values = voice_cutoffs(&p, 60.0, [0.5, 1.0, 0.25]);
    assert!((values[0] - 0.7).abs() < 1e-6);
    assert!((values[1] - 0.35).abs() < 1e-6);
    p.filter_env_depth = 1.0;
    p.filter2_env_depth = -1.0;
    assert_eq!(voice_cutoffs(&p, 60.0, [1.0; 3]), [1.0, 0.0]);
}

#[test]
fn all_filter_pairs_and_routings_are_finite_at_extreme_settings() {
    let types = [
        FilterType::LowPass,
        FilterType::BandPass,
        FilterType::HighPass,
        FilterType::Comb,
    ];
    for sample_rate in [8000, 48000, 192000] {
        for first in types {
            for second in types {
                let mut bank = FilterBank::new(sample_rate);
                let mut p = params();
                p.vcf_type = first;
                p.vcf2_type = second;
                p.filter_resonance = 1.0;
                p.filter2_resonance = 1.0;
                for routing in [
                    FilterRouting::Single,
                    FilterRouting::Serial,
                    FilterRouting::Parallel,
                ] {
                    p.filter_routing = routing;
                    for cutoff in [0.0, 0.5, 1.0] {
                        p.filter = cutoff;
                        p.filter2 = 1.0 - cutoff;
                        bank.configure(&p, 60.0, [1.0; 3]);
                        for i in 0..2048 {
                            let value = if i == 0 { 0.4 } else { 0.0 };
                            assert!(
                                bank.process([value, -value])
                                    .iter()
                                    .all(|x| x.is_finite() && x.abs() < 10.0)
                            );
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn legacy_presets_migrate_filter_env_without_doubling_or_losing_free_routes() {
    let old = serde_json::json!({
        "params": {"filter": 0.4, "filter_resonance": 0.7, "vcf_type": "high_pass"},
        "envs": [Env::default(), Env { destination: Destination::Filter, depth: -0.3, extra_destination: Destination::Pitch, extra_depth: 0.15, ..Env::default() }, Env::default()]
    });
    let p: Preset = serde_json::from_value(old).unwrap();
    assert_eq!(p.params.filter_routing, FilterRouting::Single);
    assert_eq!(p.params.filter, 0.4);
    assert_eq!(p.params.filter_resonance, 0.7);
    assert_eq!(p.params.vcf_type, FilterType::HighPass);
    assert_eq!(p.params.filter_env_depth, -0.3);
    assert_eq!(p.envs[1].destination, Destination::None);
    assert_eq!(p.envs[1].extra_destination, Destination::Pitch);
    assert_eq!(p.envs[1].extra_depth, 0.15);
    let old_no_env: Preset = serde_json::from_str(r#"{"params":{"filter":0.4}}"#).unwrap();
    assert_eq!(old_no_env.params.filter_env_depth, 0.0);
    let old_custom: Preset = serde_json::from_value(serde_json::json!({"params": {}, "envs": [Env::default(), Env {destination: Destination::Cloud, depth: 0.6, ..Env::default()}, Env::default()]})).unwrap();
    assert_eq!(old_custom.params.filter_env_depth, 0.0);
    assert_eq!(old_custom.envs[1].destination, Destination::Cloud);
}

#[test]
fn filter_settings_and_modulation_destinations_roundtrip_and_sanitize() {
    let mut p = Preset::default();
    p.params.filter_routing = FilterRouting::Parallel;
    p.params.filter2 = 0.46;
    p.params.filter2_env_depth = -0.74;
    p.params.filter2_keytrack = 0.9;
    p.params.filter2_env = EnvSource::Env3;
    p.params.vcf2_type = FilterType::Comb;
    p.lfos[0].destination = Destination::Filter2;
    p.envs[0].extra_destination = Destination::Filter2Resonance;
    let serialized = serde_json::to_value(&p).unwrap();
    let restored: Preset = serde_json::from_value(serialized.clone()).unwrap();
    assert_eq!(serde_json::to_value(restored).unwrap(), serialized);
    p.params.filter_env_depth = f32::NAN;
    p.params.filter2_env_depth = -5.0;
    p.params.filter2_keytrack = 4.0;
    p.params.filter2 = f32::INFINITY;
    p.sanitize();
    assert_eq!(p.params.filter_env_depth, 0.0);
    assert_eq!(p.params.filter2_env_depth, -1.0);
    assert_eq!(p.params.filter2_keytrack, 1.0);
    assert_eq!(p.params.filter2, 1.0);
}

#[test]
fn engine_filters_use_each_voices_note_envelope_and_channel_bend() {
    let sample = Arc::new(SampleBuffer::new("test".into(), 8000, vec![[0.1; 2]; 8000]).unwrap());
    let mut p = Preset {
        params: params(),
        ..Preset::default()
    };
    p.params.filter = 0.3;
    p.params.filter2 = 0.5;
    p.params.filter_keytrack = 1.0;
    p.params.filter2_env_depth = 0.3;
    p.envs[1].attack = 0.5;
    p.envs[1].sustain = 1.0;
    p.envs[2] = Env {
        attack: 0.0,
        decay: 0.0,
        sustain: 1.0,
        ..Env::default()
    };
    p.envs[2].destination = Destination::Filter2Resonance;
    p.envs[2].depth = 0.4;
    p.lfos[0].destination = Destination::Filter2;
    p.lfos[0].depth = 0.2;
    let mut engine = Engine::new(8000, sample, p);
    engine.midi(Message::NoteOn {
        channel: 0,
        note: 60,
        velocity: 100,
    });
    for _ in 0..800 {
        engine.process_frame();
    }
    engine.midi(Message::NoteOn {
        channel: 1,
        note: 72,
        velocity: 100,
    });
    for _ in 0..80 {
        engine.process_frame();
    }
    let old = engine.voice_filter_cutoffs(60).unwrap();
    let fresh = engine.voice_filter_cutoffs(72).unwrap();
    assert!(
        (cutoff_hz(FilterType::LowPass, fresh[0]) / cutoff_hz(FilterType::LowPass, old[0]) - 2.0)
            .abs()
            < 1e-5
    );
    assert!(old[1] > fresh[1] + 0.02);
    assert!(engine.modulated_params().filter2 > 0.5);
    assert!((engine.modulated_params().filter2_resonance - 0.4).abs() < 1e-5);
    engine.midi(Message::PitchBend {
        channel: 1,
        value: 0,
    });
    for _ in 0..32 {
        engine.process_frame();
    }
    let bent = engine.voice_filter_cutoffs(72).unwrap()[0];
    let expected = 2.0_f32.powf(-2.0 / 12.0);
    assert!(
        (cutoff_hz(FilterType::LowPass, bent) / cutoff_hz(FilterType::LowPass, fresh[0])
            - expected)
            .abs()
            < 1e-5
    );
    assert_eq!(engine.voice_filter_cutoffs(60).unwrap()[0], old[0]);
}
