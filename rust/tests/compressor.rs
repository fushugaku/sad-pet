use sad_pet::{
    color::{ColorSettings, ColorSlot},
    dsp::color::{ColorProcessor, ColorRack},
    preset::{Preset, SaturatorType},
};

fn slot(id: u32) -> ColorSlot {
    ColorSlot {
        id,
        kind: SaturatorType::Compressor,
        mix: 1.0,
        settings: ColorSettings {
            compressor_threshold: 0.6, // -24 dBFS
            compressor_ratio: 4.0_f32.ln() / 20.0_f32.ln(),
            compressor_attack: 0.0,
            compressor_release: 0.0,
            compressor_knee: 0.0,
            ..ColorSettings::default()
        },
        ..ColorSlot::default()
    }
}

#[test]
fn threshold_ratio_and_stereo_link_match_the_expected_static_curve() {
    for rate in [8000, 44100, 48000, 96000] {
        for db in [-40.0_f32, -24.0, -12.0, 0.0, 6.0] {
            let amplitude = 10.0_f32.powf(db / 20.0);
            let expected_db = if db <= -24.0 {
                db
            } else {
                -24.0 + (db + 24.0) / 4.0
            };
            let expected = 10.0_f32.powf(expected_db / 20.0);
            let mut fx = ColorProcessor::new(rate);
            fx.configure(slot(1));
            for _ in 0..rate / 10 {
                fx.process([-amplitude, amplitude * 0.25]);
            }
            let out = fx.process([-amplitude, amplitude * 0.25]);
            assert!(
                (out[0] + expected).abs() < 2e-5,
                "{rate} Hz, {db} dB: {out:?}"
            );
            assert!((out[1] / out[0] + 0.25).abs() < 1e-6);
        }
    }
}

#[test]
fn attack_preserves_transients_and_release_recovers_gradually_in_real_time() {
    for rate in [44100, 48000, 96000] {
        let mut s = slot(1);
        s.settings.compressor_attack = 1.0; // 100 ms
        s.settings.compressor_release = 0.0; // 10 ms
        let mut fx = ColorProcessor::new(rate);
        fx.configure(s);
        let initial = fx.process([1.0; 2])[0];
        for _ in 1..rate / 10 {
            fx.process([1.0; 2]);
        }
        let after_attack = fx.process([1.0; 2])[0];
        assert!(initial > 0.99);
        assert!((0.25..0.29).contains(&after_attack));
        for _ in 0..rate {
            fx.process([1.0; 2]);
        }
        let compressed = fx.process([0.01; 2])[0];
        for _ in 1..rate / 100 {
            fx.process([0.01; 2]);
        }
        let recovered = fx.process([0.01; 2])[0];
        assert!(compressed < 0.002);
        assert!((0.0045..0.0049).contains(&recovered));
        for _ in 0..rate / 5 {
            fx.process([0.01; 2]);
        }
        assert!((fx.process([0.01; 2])[0] - 0.01).abs() < 1e-6);
    }
}

#[test]
fn knee_blends_near_threshold_and_makeup_and_parallel_mix_work() {
    let mut hard = ColorProcessor::new(48000);
    let mut soft = ColorProcessor::new(48000);
    let s = slot(1);
    hard.configure(s);
    let mut knee = s;
    knee.settings.compressor_knee = 0.5;
    soft.configure(knee);
    let input = [10.0_f32.powf(-24.0 / 20.0); 2];
    for _ in 0..4800 {
        hard.process(input);
        soft.process(input);
    }
    assert!((hard.process(input)[0] - input[0]).abs() < 1e-6);
    assert!(soft.process(input)[0] < input[0] * 0.9);

    let mut wet = ColorProcessor::new(48000);
    let mut blend = ColorProcessor::new(48000);
    wet.configure(s);
    let mut mixed = s;
    mixed.mix = 0.25;
    blend.configure(mixed);
    for _ in 0..4800 {
        let wet_out = wet.process([0.5; 2])[0];
        let blended = blend.process([0.5; 2])[0];
        assert!((blended - (0.375 + wet_out * 0.25)).abs() < 1e-6);
    }
    let mut makeup = s;
    makeup.settings.compressor_ratio = 0.0; // 1:1, no reduction
    makeup.settings.compressor_makeup = 0.25; // +6 dB
    wet.clear();
    wet.configure(makeup);
    assert!((wet.process([0.1; 2])[0] - 0.199526).abs() < 1e-6);
}

#[test]
fn independent_compressors_stack_and_keep_their_envelopes_when_reordered() {
    let a = slot(11);
    let mut b = slot(22);
    b.settings.compressor_attack = 1.0;
    b.settings.compressor_release = 1.0;
    b.settings.compressor_makeup = 0.2;
    let mut rack = ColorRack::new(48000);
    let mut first = ColorProcessor::new(48000);
    let mut second = ColorProcessor::new(48000);
    rack.configure(&[a, b]);
    first.configure(a);
    second.configure(b);
    for _ in 0..4800 {
        assert_eq!(
            rack.process([0.8, 0.2]),
            second.process(first.process([0.8, 0.2]))
        );
    }
    rack.configure(&[b, a]);
    for _ in 0..4800 {
        assert_eq!(
            rack.process([0.8, 0.2]),
            first.process(second.process([0.8, 0.2]))
        );
    }
    rack.clear();
    first.clear();
    second.clear();
    assert_eq!(
        rack.process([0.8, 0.2]),
        first.process(second.process([0.8, 0.2]))
    );
}

#[test]
fn compressor_presets_roundtrip_and_missing_or_invalid_settings_are_repaired() {
    let mut p = Preset {
        color_chain: vec![slot(11), slot(22)],
        ..Preset::default()
    };
    p.sanitize();
    let saved = serde_json::to_value(&p).unwrap();
    let restored: Preset = serde_json::from_value(saved.clone()).unwrap();
    assert_eq!(saved, serde_json::to_value(restored).unwrap());
    let legacy: ColorSlot = serde_json::from_str(r#"{"kind":"compressor","settings":{}}"#).unwrap();
    assert_eq!(legacy.settings, ColorSettings::default());
    let mut invalid = slot(1);
    for parameter in invalid.control_parameters() {
        *invalid.parameter_mut(*parameter) = f32::NAN;
    }
    invalid.sanitize();
    assert_eq!(invalid.settings, ColorSettings::default());
    assert!(invalid.mix.is_finite());
}
