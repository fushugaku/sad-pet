use sad_pet::{
    Stereo,
    color::{ColorParameter as P, ColorSettings, ColorSlot, MAX_COLOR_SLOTS},
    dsp::{
        Engine, SampleBuffer,
        color::{ColorProcessor, ColorRack},
    },
    preset::{Destination, Preset, SaturatorType as Type},
    radias_delay,
};
use std::sync::Arc;

fn slot(id: u32, kind: Type) -> ColorSlot {
    ColorSlot {
        id,
        kind,
        mix: 1.0,
        ..ColorSlot::default()
    }
}
fn signal(i: usize, sr: u32) -> Stereo {
    let t = i as f32 / sr as f32 * std::f32::consts::TAU;
    [(t * 173.0).sin() * 0.25, (t * 311.0).sin() * 0.2]
}
fn audio(s: &[ColorSlot], sr: u32, n: usize) -> Vec<Stereo> {
    let mut rack = ColorRack::new(sr);
    rack.configure(s);
    (0..n).map(|i| rack.process(signal(i, sr))).collect()
}
fn difference(a: &[Stereo], b: &[Stereo]) -> f32 {
    a.iter()
        .zip(b)
        .map(|(a, b)| (a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2))
        .sum::<f32>()
        / (a.len() * 2) as f32
}

#[test]
fn radias_tables_and_tempo_place_three_impulses_at_the_expected_times() {
    assert!((radias_delay::lfo_hz(34.0 / 127.0) - 1.0).abs() < 1e-6);
    assert!(
        (radias_delay::seconds(true, 64.0 / 127.0, 0.0, 7.0 / 13.0, 120.0, false) - 0.375).abs()
            < 1e-6
    );
    assert_eq!(
        radias_delay::seconds(false, 1.0, 1.0, 0.0, 120.0, false),
        radias_delay::MAX_SECONDS
    );
    for (sr, bpm) in [(8000, 120.0), (48000, 240.0)] {
        let mut s = slot(1, Type::LcrDelay);
        s.settings.lcr_left_level = 1.0;
        s.settings.lcr_center_level = 1.0;
        s.settings.lcr_right_level = 1.0;
        s.settings.lcr_feedback = 0.0;
        s.settings.lcr_spread = 1.0;
        let mut p = ColorProcessor::new(sr);
        p.configure_tempo(s, bpm);
        let n = (sr as f32 * 60.0 / bpm) as usize + 4;
        let frames: Vec<_> = (0..n)
            .map(|i| p.process(if i == 0 { [1.0; 2] } else { [0.0; 2] }))
            .collect();
        let beat = sr as f32 * 60.0 / bpm;
        for (offset, expected) in [(0.75, [1.0, 0.0]), (1.0, [0.707, 0.707]), (0.5, [0.0, 1.0])] {
            let at = (beat * offset).round() as usize;
            for (c, expected) in expected.iter().enumerate() {
                assert!((frames[at][c] - expected).abs() < 1e-4);
            }
        }
        assert!(
            frames
                .iter()
                .enumerate()
                .filter(|(_, f)| f[0].abs() + f[1].abs() > 1e-4)
                .count()
                <= 3
        );
    }
}

#[test]
fn smooth_clock_keeps_note_anchors_and_fills_the_spaces_between_them() {
    for mod_delay in [false, true] {
        for bpm in [60.0, 120.0, 240.0] {
            for note in 0..14 {
                let value = note as f32 / 13.0;
                let stepped = radias_delay::seconds(true, 64.0 / 127.0, 0.0, value, bpm, mod_delay);
                let smooth =
                    radias_delay::smooth_seconds(true, 64.0 / 127.0, 0.0, value, bpm, mod_delay);
                assert!((stepped - smooth).abs() < 1e-6);
            }
        }
        let mut previous = 0.375;
        for step in 1..=100 {
            let note = (7.0 + step as f32 / 100.0) / 13.0;
            let time =
                radias_delay::smooth_seconds(true, 64.0 / 127.0, 0.0, note, 120.0, mod_delay);
            assert!(time > previous && time - previous < 0.0013);
            previous = time;
        }
        assert!((previous - 0.5).abs() < 1e-6);
        assert_eq!(
            radias_delay::smooth_seconds(false, 1.0, 1.0, 0.0, 120.0, mod_delay),
            radias_delay::MAX_SECONDS
        );
    }
}

#[test]
fn lcr_ratio_is_continuous_without_smooth_and_retains_original_table_anchors() {
    for (sync, table) in [
        (false, &radias_delay::FREE_RATIO),
        (true, &radias_delay::SYNC_RATIO),
    ] {
        let time = 50.0 / 127.0; // 150 ms; short enough to avoid buffer capping.
        let note = 8.0 / 13.0; // quarter note
        for (i, &ratio) in table.iter().enumerate() {
            let expected = if sync { 0.5 } else { 0.15 } * ratio as f32 / 1000.0;
            let actual = radias_delay::seconds(sync, i as f32 / 127.0, time, note, 120.0, false);
            assert!((actual - expected).abs() < 1e-6);
        }
        let left = if sync { 67 } else { 65 };
        let before = radias_delay::seconds(sync, left as f32 / 127.0, time, note, 120.0, false);
        let after =
            radias_delay::seconds(sync, (left + 1) as f32 / 127.0, time, note, 120.0, false);
        let mut previous = before;
        for step in 1..=100 {
            let ratio = (left as f32 + step as f32 / 100.0) / 127.0;
            let current = radias_delay::seconds(sync, ratio, time, note, 120.0, false);
            assert!(current > previous, "L/C/R ratio has a quantized dead zone");
            assert!((current - (before + (after - before) * step as f32 / 100.0)).abs() < 1e-6);
            previous = current;
        }
    }
}

#[test]
fn lcr_ratio_modulation_has_no_plateaus_and_is_finer_than_manual_steps() {
    for sync in [false, true] {
        let base = 64.0 / 127.0;
        let time = 50.0 / 127.0;
        let note = 8.0 / 13.0;
        let mut previous = 0.0;
        for i in 0..1000 {
            let offset = (i as f32 - 500.0) * 0.000001;
            let position = radias_delay::modulate_lcr_ratio(sync, base, offset);
            let seconds = radias_delay::seconds(sync, position, time, note, 120.0, false);
            if i > 0 {
                assert!(seconds > previous, "Small ratio modulation had a dead zone");
                assert!(seconds - previous < 0.00001);
            }
            previous = seconds;
        }
        assert_eq!(radias_delay::modulate_lcr_ratio(sync, base, 0.0), base);
    }
    // Both stable effect IDs and generic numeric destinations use the same mapping.
    for generic in [false, true] {
        let mut p = Preset {
            color_chain: vec![slot(1, Type::LcrDelay)],
            ..Preset::default()
        };
        p.params.grains = 0.0;
        p.lfos[0].rate_hz = 1.0;
        p.lfos[0].depth = 0.0002;
        p.lfos[0].destination = if generic {
            Destination::Control {
                index: sad_pet::parameters::specs()
                    .iter()
                    .position(|s| s.id == "fx1_lcr_ratio")
                    .unwrap() as u16,
            }
        } else {
            Destination::Color {
                slot: 1,
                parameter: P::LcrRatio,
            }
        };
        let mut e = Engine::new(8000, Arc::new(SampleBuffer::demo(8000)), p);
        e.set_playing(true);
        e.process_frame();
        let mut range = [f32::MAX, f32::MIN];
        for _ in 0..100 {
            for _ in 0..32 {
                e.process_frame();
            }
            let s = e.modulated_color_slots()[0].settings;
            let actual = radias_delay::seconds(true, s.lcr_ratio, 0.0, 8.0 / 13.0, 120.0, false);
            let expected = 0.5 * (1000.0 + e.lfo_values[0] * 0.0001 * 3875.0) / 1000.0;
            assert!((actual - expected).abs() < 0.000001);
            range[0] = range[0].min(actual);
            range[1] = range[1].max(actual);
        }
        assert!(
            range[1] - range[0] > 0.0001,
            "Low-depth LFO did not change ratio"
        );
        assert_eq!(e.preset.color_chain[0].settings.lcr_ratio, 64.0 / 127.0);
    }
}

#[test]
fn lcr_ratio_glides_without_smooth_and_does_not_clear_the_existing_tail() {
    for sr in [8000, 48000] {
        let mut s = slot(1, Type::LcrDelay);
        s.settings.lcr_smooth = false;
        s.settings.lcr_left_note = 5.0 / 13.0;
        s.settings.lcr_left_level = 1.0;
        s.settings.lcr_center_level = 0.0;
        s.settings.lcr_right_level = 0.0;
        s.settings.lcr_spread = 1.0;
        s.settings.lcr_feedback = 0.0;
        let mut p = ColorProcessor::new(sr);
        p.configure_tempo(s, 120.0);
        let input = |i: u32| [(i as f32 / sr as f32 * std::f32::consts::TAU * 5.0).sin() * 0.25; 2];
        let mut last = 0.0;
        for i in 0..sr * 2 {
            last = p.process(input(i))[0];
        }
        // 100% -> 200%; the previous tail must move gradually, even with Smooth off.
        s.settings.lcr_ratio = 97.0 / 127.0;
        p.configure_tempo(s, 120.0);
        let next = p.process(input(sr * 2))[0];
        assert!((next - last).abs() < 0.005);
        assert!(next.abs() > 0.2);
        let mut settled = next;
        for i in sr * 2 + 1..sr * 3 {
            settled = p.process(input(i))[0];
        }
        assert!((settled - input(sr * 3 - 1 - sr / 2)[0]).abs() < 1e-4);
    }
}

#[test]
fn smooth_delay_emits_an_echo_halfway_between_two_note_divisions() {
    for kind in [Type::LcrDelay, Type::ModDelay] {
        let mut s = slot(1, kind);
        s.settings.lcr_smooth = true;
        s.settings.mod_delay_smooth = true;
        s.settings.lcr_left_note = 7.5 / 13.0;
        s.settings.lcr_left_level = 1.0;
        s.settings.lcr_center_level = 0.0;
        s.settings.lcr_right_level = 0.0;
        s.settings.lcr_spread = 1.0;
        s.settings.lcr_feedback = 0.0;
        s.settings.mod_delay_left_note = 7.5 / 13.0;
        s.settings.mod_delay_depth = 0.0;
        s.settings.mod_delay_feedback = 0.0;
        for sr in [8000, 48000] {
            let mut p = ColorProcessor::new(sr);
            p.configure_tempo(s, 120.0);
            let at = (sr as f32 * 0.4375) as usize;
            for i in 0..=at + 2 {
                let frame = p.process(if i == 0 { [1.0; 2] } else { [0.0; 2] });
                assert!(
                    (frame[0] - if i == at { 1.0 } else { 0.0 }).abs() < 0.005,
                    "{kind:?} at {sr} Hz, frame {i}: {frame:?}"
                );
            }
        }
    }
}

#[test]
fn smooth_delay_glides_on_time_changes_without_clearing_the_tail() {
    for kind in [Type::LcrDelay, Type::ModDelay] {
        for sr in [8000, 48000] {
            let mut first_jumps = Vec::new();
            for smooth in [false, true] {
                let mut s = slot(1, kind);
                s.settings.lcr_smooth = smooth;
                s.settings.mod_delay_smooth = smooth;
                s.settings.lcr_left_note = 5.0 / 13.0;
                s.settings.mod_delay_left_note = 5.0 / 13.0;
                s.settings.lcr_left_level = 1.0;
                s.settings.lcr_center_level = 0.0;
                s.settings.lcr_right_level = 0.0;
                s.settings.lcr_spread = 1.0;
                s.settings.lcr_feedback = 0.0;
                s.settings.mod_delay_depth = 0.0;
                s.settings.mod_delay_feedback = 0.0;
                let mut p = ColorProcessor::new(sr);
                p.configure_tempo(s, 120.0);
                let input =
                    |i: u32| [(i as f32 / sr as f32 * std::f32::consts::TAU * 5.0).sin() * 0.25; 2];
                let mut last = 0.0;
                for i in 0..sr * 2 {
                    last = p.process(input(i))[0];
                }
                s.settings.lcr_left_note = 8.0 / 13.0;
                s.settings.mod_delay_left_note = 8.0 / 13.0;
                p.configure_tempo(s, 120.0);
                let next = p.process(input(sr * 2))[0];
                first_jumps.push((next - last).abs());
                if smooth {
                    assert!(
                        next.abs() > 0.2,
                        "The previous delay tail must survive reconfiguration"
                    );
                }
                let mut settled = next;
                for i in sr * 2 + 1..sr * 3 {
                    settled = p.process(input(i))[0];
                }
                let expected = input(sr * 3 - 1 - sr / 2)[0];
                assert!((settled - expected).abs() < 1e-4);
            }
            assert!(
                first_jumps[1] < first_jumps[0] * 0.1,
                "{kind:?} at {sr} Hz: {first_jumps:?}"
            );
        }
    }
}

#[test]
fn mod_delay_uses_mono_input_independent_times_and_audible_modulation() {
    let mut s = slot(1, Type::ModDelay);
    s.settings.mod_delay_sync = false;
    s.settings.mod_delay_depth = 0.0;
    s.settings.mod_delay_feedback = 0.0;
    s.settings.mod_delay_left_delay = 10.0 / 127.0;
    s.settings.mod_delay_right_delay = 18.0 / 127.0;
    let mut p = ColorProcessor::new(8000);
    p.configure(s);
    for i in 0..200 {
        let f = p.process(if i == 0 { [1.0; 2] } else { [0.0; 2] });
        assert_eq!(
            f,
            [
                if i == 80 { 1.0 } else { 0.0 },
                if i == 160 { 1.0 } else { 0.0 }
            ]
        );
    }
    p.clear();
    p.configure(s);
    for i in 0..400 {
        assert_eq!(
            p.process(if i == 0 { [1.0, -1.0] } else { [0.0; 2] }),
            [0.0; 2]
        );
    }
    let dry = audio(&[s], 8000, 16000);
    s.settings.mod_delay_depth = 1.0;
    assert!(difference(&dry, &audio(&[s], 8000, 16000)) > 1e-4);
}

#[test]
fn all_delay_controls_change_the_audio_in_their_clock_mode() {
    let params = [
        (Type::LcrDelay, P::LcrRatio),
        (Type::LcrDelay, P::LcrLeftDelay),
        (Type::LcrDelay, P::LcrCenterDelay),
        (Type::LcrDelay, P::LcrRightDelay),
        (Type::LcrDelay, P::LcrLeftNote),
        (Type::LcrDelay, P::LcrCenterNote),
        (Type::LcrDelay, P::LcrRightNote),
        (Type::LcrDelay, P::LcrLeftLevel),
        (Type::LcrDelay, P::LcrCenterLevel),
        (Type::LcrDelay, P::LcrRightLevel),
        (Type::LcrDelay, P::LcrFeedback),
        (Type::LcrDelay, P::LcrHighDamp),
        (Type::LcrDelay, P::LcrLowDamp),
        (Type::LcrDelay, P::LcrTrim),
        (Type::LcrDelay, P::LcrSpread),
        (Type::ModDelay, P::ModDelayRatio),
        (Type::ModDelay, P::ModDelayLeftDelay),
        (Type::ModDelay, P::ModDelayRightDelay),
        (Type::ModDelay, P::ModDelayLeftNote),
        (Type::ModDelay, P::ModDelayRightNote),
        (Type::ModDelay, P::ModDelayFeedback),
        (Type::ModDelay, P::ModDelayDepth),
        (Type::ModDelay, P::ModDelayRate),
        (Type::ModDelay, P::ModDelaySpread),
    ];
    for (kind, param) in params {
        let mut low = slot(1, kind);
        let sync = matches!(
            param,
            P::LcrLeftNote
                | P::LcrCenterNote
                | P::LcrRightNote
                | P::ModDelayLeftNote
                | P::ModDelayRightNote
        );
        low.settings.lcr_sync = sync;
        low.settings.mod_delay_sync = sync;
        *low.parameter_mut(param) = 0.0;
        let mut high = low;
        // -180 and +180 degrees are the same stereo LFO phase.
        *high.parameter_mut(param) = if param == P::ModDelaySpread { 0.5 } else { 1.0 };
        assert!(
            difference(&audio(&[low], 8000, 24000), &audio(&[high], 8000, 24000)) > 1e-9,
            "{kind:?} {param:?}"
        );
    }
}

#[test]
fn duplicate_delays_have_separate_tails_and_reordering_keeps_them() {
    let a = slot(11, Type::LcrDelay);
    let b = slot(22, Type::ModDelay);
    let mut rack = ColorRack::new(8000);
    rack.configure(&[a, b]);
    let mut first = ColorProcessor::new(8000);
    first.configure(a);
    let mut second = ColorProcessor::new(8000);
    second.configure(b);
    for i in 0..8000 {
        assert_eq!(
            rack.process(signal(i, 8000)),
            second.process(first.process(signal(i, 8000)))
        );
    }
    let mut disabled = a;
    disabled.id = 33;
    disabled.enabled = false;
    let mut expected = ColorRack::new(8000);
    expected.configure(&[a]);
    rack.clear();
    rack.configure(&[a]);
    for i in 0..4000 {
        assert_eq!(
            rack.process(signal(i, 8000)),
            expected.process(signal(i, 8000))
        );
    }
    rack.configure(&[disabled, a]);
    for _ in 0..4000 {
        assert_eq!(rack.process([0.0; 2]), expected.process([0.0; 2]));
    }
    rack.configure(&[]);
    rack.configure(&[a]);
    for _ in 0..8000 {
        assert_eq!(rack.process([0.0; 2]), [0.0; 2]);
    }
    let distortion = slot(44, Type::Distortion);
    assert!(
        difference(
            &audio(&[a, distortion], 8000, 16000),
            &audio(&[distortion, a], 8000, 16000)
        ) > 1e-6
    );
}

#[test]
fn bypass_mix_and_extreme_stacks_remain_finite_at_multiple_sample_rates() {
    for kind in [Type::LcrDelay, Type::ModDelay] {
        for enabled in [true, false] {
            let mut s = slot(1, kind);
            s.enabled = enabled;
            s.mix = if enabled { 0.0 } else { 1.0 };
            for (i, f) in audio(&[s], 8000, 8000).iter().enumerate() {
                assert_eq!(*f, signal(i, 8000));
            }
        }
    }
    for sr in [8000, 48000, 192000] {
        let slots: Vec<_> = (0..MAX_COLOR_SLOTS)
            .map(|i| {
                let mut s = slot(
                    i as u32 + 1,
                    if i % 2 == 0 {
                        Type::LcrDelay
                    } else {
                        Type::ModDelay
                    },
                );
                s.mix = 0.6;
                s.settings.lcr_sync = false;
                s.settings.mod_delay_sync = false;
                s.settings.lcr_left_delay = 0.0;
                s.settings.lcr_center_delay = 0.0;
                s.settings.lcr_right_delay = 0.0;
                s.settings.mod_delay_left_delay = 0.0;
                s.settings.mod_delay_right_delay = 0.0;
                s.settings.lcr_feedback = 1.0;
                s.settings.mod_delay_feedback = 1.0;
                s.settings.mod_delay_depth = 1.0;
                s.settings.mod_delay_rate = 1.0;
                s
            })
            .collect();
        assert!(
            audio(&slots, sr, sr as usize / 4)
                .iter()
                .flatten()
                .all(|v| v.is_finite() && v.abs() < 1000.0)
        );
    }
}

#[test]
fn clocks_settings_and_modulation_targets_roundtrip_without_changing_old_presets() {
    let mut p = Preset {
        color_chain: vec![slot(11, Type::LcrDelay), slot(22, Type::ModDelay)],
        ..Preset::default()
    };
    p.color_chain[0].settings.lcr_sync = false;
    p.color_chain[0].settings.lcr_smooth = true;
    p.color_chain[0].settings.lcr_left_delay = 0.2;
    p.color_chain[0].settings.lcr_left_note = 0.7;
    p.color_chain[1].settings.mod_delay_rate = 0.5;
    p.color_chain[1].settings.mod_delay_smooth = true;
    p.lfos[0].destination = Destination::Color {
        slot: 22,
        parameter: P::ModDelayDepth,
    };
    p.envs[2].destination = Destination::Color {
        slot: 11,
        parameter: P::LcrHighDamp,
    };
    p.sanitize();
    let value = serde_json::to_value(&p).unwrap();
    let mut restored: Preset = serde_json::from_value(value.clone()).unwrap();
    restored.sanitize();
    assert_eq!(serde_json::to_value(&restored).unwrap(), value);
    let defaults: ColorSettings = serde_json::from_str("{}").unwrap();
    assert_eq!(defaults, ColorSettings::default());
}

#[test]
fn stacked_delays_and_reverb_render_identically_across_audio_blocks() {
    let mut p = Preset {
        color_chain: vec![
            slot(11, Type::LcrDelay),
            slot(22, Type::Reverb),
            slot(33, Type::ModDelay),
        ],
        ..Preset::default()
    };
    p.color_chain[0].settings.lcr_smooth = true;
    p.color_chain[2].settings.mod_delay_smooth = true;
    p.lfos[0].destination = Destination::Color {
        slot: 11,
        parameter: P::LcrLeftNote,
    };
    p.lfos[0].depth = 0.12;
    p.envs[0].extra_destination = Destination::Color {
        slot: 33,
        parameter: P::ModDelayRightNote,
    };
    p.envs[0].extra_depth = 0.15;
    let sample = Arc::new(
        SampleBuffer::new(
            "tone".into(),
            8000,
            (0..8000).map(|i| signal(i, 8000)).collect(),
        )
        .unwrap(),
    );
    let mut a = Engine::new(8000, sample.clone(), p.clone());
    let mut b = Engine::new(8000, sample, p);
    a.set_playing(true);
    b.set_playing(true);
    let mut expected = vec![[0.0; 2]; 16000];
    let mut actual = expected.clone();
    a.process(&mut expected);
    for chunk in actual.chunks_mut(71) {
        b.process(chunk);
    }
    assert_eq!(actual, expected);
    assert!(actual.iter().flatten().any(|v| v.abs() > 1e-4));
}
