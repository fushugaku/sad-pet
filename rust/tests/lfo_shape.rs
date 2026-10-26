use metropolix_core::sequencer::field;
use sad_pet::{
    dsp::{Engine, SampleBuffer, modulation::Oscillator},
    matrix::LfoParameter,
    midi::Message,
    parameters,
    preset::{Destination, Lfo, LfoReset, Preset, Waveform, lfo_seconds},
    sequence::{Target, Track},
};
use std::sync::Arc;

const SR: f32 = 1000.0;

fn engine(p: Preset) -> Engine {
    Engine::new(8000, Arc::new(SampleBuffer::demo(8000)), p)
}
/// One LFO's output over `n` samples at 1 kHz.
fn run(lfo: &Lfo, n: usize) -> Vec<f32> {
    let mut o = Oscillator::new(1);
    (0..n)
        .map(|_| o.next_sample(lfo, SR, 120.0, 0.0, None))
        .collect()
}
fn lfo(waveform: Waveform) -> Lfo {
    Lfo {
        waveform,
        rate_hz: 1.0,
        ..Lfo::default()
    }
}
/// A preset whose grains stay quiet, with LFO 1 as given.
fn preset(first: Lfo) -> Preset {
    let mut p = Preset::default();
    p.params.grains = 0.0;
    p.lfos[0] = first;
    p
}
fn frames(e: &mut Engine, n: usize) -> Vec<f32> {
    (0..n)
        .map(|_| {
            assert!(e.process_frame().into_iter().all(f32::is_finite));
            e.lfo_values[0]
        })
        .collect()
}

#[test]
fn plain_settings_keep_the_waves_bit_for_bit() {
    for waveform in [
        Waveform::Sine,
        Waveform::Triangle,
        Waveform::Saw,
        Waveform::Square,
    ] {
        let values = run(&lfo(waveform), 1500);
        let mut phase = 0.0f64;
        for (i, &v) in values.iter().enumerate() {
            let p = phase as f32;
            let old = match waveform {
                Waveform::Sine => sad_pet::tables::sine(phase),
                Waveform::Triangle => 1.0 - 4.0 * (p - 0.5).abs(),
                Waveform::Saw => p * 2.0 - 1.0,
                _ if p < 0.5 => 1.0,
                _ => -1.0,
            };
            assert_eq!(v.to_bits(), old.to_bits(), "{waveform:?} sample {i}");
            phase += 1.0 / 1000.0;
            if phase >= 1.0 {
                phase = phase.rem_euclid(1.0);
            }
        }
    }
    // Presets saved before the new controls load them neutral.
    let old: Lfo =
        serde_json::from_value(serde_json::json!({"waveform":"saw","rate_hz":2.0})).unwrap();
    assert_eq!(
        (old.shape, old.phase, old.smooth, old.delay, old.fade),
        (0.0, 0.0, 0.0, 0.0, 0.0)
    );
    assert!(!old.unipolar && !old.one_shot && old.reset == LfoReset::Free);
}

#[test]
fn shape_skews_triangles_into_saws_widens_squares_and_bends_ramps() {
    let up = run(
        &Lfo {
            shape: 1.0,
            ..lfo(Waveform::Triangle)
        },
        1000,
    );
    let down = run(
        &Lfo {
            shape: -1.0,
            ..lfo(Waveform::Triangle)
        },
        1000,
    );
    for i in [0, 250, 500, 750, 999] {
        let p = i as f32 / 1000.0;
        assert!((up[i] - (p * 2.0 - 1.0)).abs() < 1e-4, "up {i}: {}", up[i]);
        assert!(
            (down[i] - (1.0 - p * 2.0)).abs() < 1e-4,
            "down {i}: {}",
            down[i]
        );
    }
    // The skewed sine rises to its peak at 0.8 of the turn a quarter on, and
    // stays smooth; the plain one peaks at a quarter.
    let sine = run(
        &Lfo {
            shape: 0.6,
            ..lfo(Waveform::Sine)
        },
        1000,
    );
    let peak = (0..1000)
        .max_by(|&a, &b| sine[a].total_cmp(&sine[b]))
        .unwrap();
    assert!((548..=552).contains(&peak), "{peak}");
    assert!(sine.windows(2).all(|w| (w[1] - w[0]).abs() < 0.03));

    let wide = run(
        &Lfo {
            shape: 0.5,
            ..lfo(Waveform::Square)
        },
        1000,
    );
    let high = wide.iter().filter(|&&v| v > 0.0).count();
    assert!((744..=746).contains(&high), "{high}");

    let bent = run(
        &Lfo {
            shape: 1.0,
            ..lfo(Waveform::Saw)
        },
        1000,
    );
    assert!(
        bent.windows(2).all(|w| w[1] >= w[0]),
        "a bent saw still rises"
    );
    assert!((bent[500] - (2.0 * 0.5 / (0.5 + 0.5 / 16.0) - 1.0)).abs() < 1e-4);
    let random = run(
        &Lfo {
            shape: -1.0,
            ..lfo(Waveform::Random)
        },
        5000,
    );
    let plain = run(&lfo(Waveform::Random), 5000);
    assert!(
        random
            .iter()
            .zip(&plain)
            .all(|(a, b)| a.abs() <= b.abs() + 1e-6)
    );
    assert!(
        random
            .iter()
            .zip(&plain)
            .any(|(a, b)| a.abs() < b.abs() - 0.1)
    );
}

#[test]
fn phase_polarity_and_smoothing() {
    let shifted = run(
        &Lfo {
            phase: 0.25,
            ..lfo(Waveform::Sine)
        },
        1,
    );
    assert!((shifted[0] - 1.0).abs() < 1e-3, "{}", shifted[0]);

    let uni = run(
        &Lfo {
            unipolar: true,
            ..lfo(Waveform::Sine)
        },
        1000,
    );
    let (lo, hi) = uni
        .iter()
        .fold((f32::MAX, f32::MIN), |(lo, hi), &v| (lo.min(v), hi.max(v)));
    assert!(
        lo > -1e-3 && lo < 0.01 && hi > 0.99 && hi < 1.001,
        "{lo}..{hi}"
    );

    let hard = run(&lfo(Waveform::Square), 3000);
    let soft = run(
        &Lfo {
            smooth: 1.0,
            ..lfo(Waveform::Square)
        },
        3000,
    );
    let step = |v: &[f32]| {
        v.windows(2)
            .map(|w| (w[1] - w[0]).abs())
            .fold(0.0, f32::max)
    };
    assert_eq!(step(&hard), 2.0);
    assert!(step(&soft) < 0.02, "{}", step(&soft));
    // Smoothed, it still swings most of the way.
    let tail = &soft[2000..];
    assert!(tail.iter().cloned().fold(f32::MIN, f32::max) > 0.7);
    assert!(tail.iter().cloned().fold(f32::MAX, f32::min) < -0.7);
}

#[test]
fn delay_and_fade_follow_a_reset_and_one_shots_hold_their_end() {
    let delay = (0.1f32 / 10.0).cbrt();
    let fade = (0.2f32 / 10.0).cbrt();
    assert!((lfo_seconds(delay) - 0.1).abs() < 1e-4);
    let l = Lfo {
        delay,
        fade,
        reset: LfoReset::Note,
        ..lfo(Waveform::Square)
    };
    let mut o = Oscillator::new(1);
    // Never reset: full depth from the start, as before.
    assert_eq!(o.next_sample(&l, SR, 120.0, 0.0, None), 1.0);
    o.reset();
    let out: Vec<f32> = (0..400)
        .map(|_| o.next_sample(&l, SR, 120.0, 0.0, None))
        .collect();
    assert!(out[..99].iter().all(|&v| v == 0.0));
    assert!((out[200] - 0.5).abs() < 0.02, "{}", out[200]);
    assert_eq!(out[350], 1.0, "a reset cycle starts after its delay");

    let shot = Lfo {
        one_shot: true,
        ..lfo(Waveform::Saw)
    };
    let mut o = Oscillator::new(1);
    let out: Vec<f32> = (0..2500)
        .map(|_| o.next_sample(&shot, SR, 120.0, 0.0, None))
        .collect();
    assert!((out[500]).abs() < 0.01);
    assert!(out[1100..].iter().all(|&v| v == 1.0), "{}", out[1100]);
    // Back to Loop, a finished one-shot runs on from its start.
    let looped = Lfo {
        one_shot: false,
        ..shot
    };
    assert_eq!(o.next_sample(&looped, SR, 120.0, 0.0, None), -1.0);
    assert!(o.next_sample(&looped, SR, 120.0, 0.0, None) > -1.0);
    o.reset();
    assert_eq!(o.next_sample(&shot, SR, 120.0, 0.0, None), -1.0);
}

#[test]
fn note_resets_restart_the_cycle_and_the_first_note_restarts_a_free_fade() {
    let mut e = engine(preset(Lfo {
        reset: LfoReset::Note,
        rate_hz: 0.25,
        ..lfo(Waveform::Saw)
    }));
    let before = frames(&mut e, 8000);
    assert!((before[7999] - -0.5).abs() < 0.01, "{}", before[7999]);
    e.midi(Message::NoteOn {
        channel: 0,
        note: 60,
        velocity: 100,
    });
    let after = frames(&mut e, 2);
    assert!(after[0] < -0.99, "{}", after[0]);

    // A free LFO keeps its cycle; the first note restarts its fade, a second
    // one played over it does not.
    let mut e = engine(preset(Lfo {
        fade: 0.5,
        ..lfo(Waveform::Square)
    }));
    assert_eq!(frames(&mut e, 10)[9], 1.0);
    e.midi(Message::NoteOn {
        channel: 0,
        note: 60,
        velocity: 100,
    });
    let faded = frames(&mut e, 4000);
    assert!(faded[100].abs() < 0.02, "{}", faded[100]);
    e.midi(Message::NoteOn {
        channel: 0,
        note: 64,
        velocity: 100,
    });
    let held = frames(&mut e, 2);
    assert!(held[1].abs() > faded[3999].abs() - 0.01);
}

#[test]
fn tempo_resets_follow_the_song_position_at_synced_and_hz_rates() {
    let mut p = preset(Lfo {
        reset: LfoReset::Tempo,
        sync: true,
        division: 4.0,
        ..lfo(Waveform::Saw)
    });
    p.lfos[1] = Lfo {
        reset: LfoReset::Tempo,
        rate_hz: 1.0,
        ..lfo(Waveform::Saw)
    };
    let mut e = engine(p);
    frames(&mut e, 1234);
    // Beat 2 of a one-bar cycle is half way; 1.5 beats at 120 BPM are 0.75 s.
    e.host_transport(120.0, Some(2.0), false, false);
    frames(&mut e, 1);
    assert!(e.lfo_values[0].abs() < 1e-3, "{}", e.lfo_values[0]);
    e.host_transport(120.0, Some(1.5), false, false);
    frames(&mut e, 1);
    assert!((e.lfo_values[1] - 0.5).abs() < 1e-3, "{}", e.lfo_values[1]);
    // A stopped host repeats its playhead: the position counts on.
    for _ in 0..4 {
        e.host_transport(120.0, Some(1.5), false, false);
        frames(&mut e, 2000);
    }
    let expected = ((1.5 * 0.5 + 8001.0 / 8000.0) % 1.0) * 2.0 - 1.0;
    assert!(
        (e.lfo_values[1] - expected as f32).abs() < 1e-2,
        "{}",
        e.lfo_values[1]
    );
    // Starting the transport starts the song at its top.
    e.set_playing(true);
    frames(&mut e, 1);
    assert!(e.lfo_values[0] < -0.99);
}

#[test]
fn sequencer_gates_reset_lfos_through_a_reset_lane_or_steps_set_to_one() {
    let peak = |lane: Target, destination: Option<&str>, value: f32| {
        let mut p = preset(Lfo {
            reset: LfoReset::Sequencer,
            rate_hz: 0.5,
            ..lfo(Waveform::Saw)
        });
        p.sequencer.enabled = true;
        let mut t = Track {
            target: lane,
            ..Track::default()
        };
        t.config[field::GATE] = 10;
        t.destinations[0] = destination.map(Into::into);
        for s in &mut t.steps {
            s.destination1 = value;
        }
        p.sequencer.tracks = vec![t];
        let mut e = engine(p);
        e.set_playing(true);
        frames(&mut e, 16000).into_iter().fold(f32::MIN, f32::max)
    };
    let lane = Target::Parameter {
        parameter: "lfo1_reset".into(),
    };
    assert!(
        peak(lane, None, 0.0) < -0.5,
        "a reset lane resets on its gates"
    );
    assert!(peak(Target::Notes, Some("lfo1_reset"), 1.0) < -0.5);
    assert!(
        peak(Target::Notes, Some("lfo1_reset"), 0.0) > 0.9,
        "steps at 0"
    );
    assert!(
        peak(Target::Notes, Some("lfo2_reset"), 1.0) > 0.9,
        "another LFO"
    );
    assert_eq!(sad_pet::sequence::lfo_reset("lfo8_reset"), Some(7));
    assert_eq!(sad_pet::sequence::lfo_reset("lfo9_reset"), None);
    assert!(
        parameters::specs()
            .iter()
            .all(|s| !s.id.ends_with("_reset"))
    );
}

#[test]
fn new_controls_append_after_every_released_host_id_and_take_modulation() {
    let specs = parameters::specs();
    // A Rack's Mixer controls followed them.
    let end = specs
        .iter()
        .position(|s| s.id == "rack_sample_level")
        .unwrap();
    let ids: Vec<_> = specs[end - 40..end].iter().map(|s| s.id.as_str()).collect();
    assert_eq!(
        ids[..5],
        [
            "lfo1_shape",
            "lfo1_phase",
            "lfo1_smooth",
            "lfo1_delay",
            "lfo1_fade"
        ]
    );
    assert_eq!(ids[39], "lfo8_fade");
    assert!(specs[end - 41].id.starts_with("grain_group4_"));

    let mut p = preset(lfo(Waveform::Saw));
    specs
        .iter()
        .find(|s| s.id == "lfo1_shape")
        .unwrap()
        .set(&mut p, -0.4);
    assert_eq!(p.lfos[0].shape, -0.4);
    p.lfos[1] = Lfo {
        waveform: Waveform::Square,
        depth: 0.5,
        destination: Destination::Lfo {
            index: 0,
            parameter: LfoParameter::Phase,
        },
        ..lfo(Waveform::Square)
    };
    let mut e = engine(p);
    frames(&mut e, 64);
    assert!(
        (e.modulated_lfos()[0].phase - 0.25).abs() < 1e-5,
        "{}",
        e.modulated_lfos()[0].phase
    );
}

#[test]
fn unipolar_routes_swing_up_from_the_control_over_the_same_span() {
    let offsets = |unipolar: bool| {
        let mut p = preset(Lfo {
            unipolar,
            depth: 0.5,
            destination: Destination::Mix,
            rate_hz: 1.0,
            ..lfo(Waveform::Square)
        });
        p.params.mix = 0.3;
        let mut e = engine(p);
        frames(&mut e, 400);
        let high = e.modulated_params().mix - 0.3;
        frames(&mut e, 4000);
        let low = e.modulated_params().mix - 0.3;
        (high, low)
    };
    let (high, low) = offsets(false);
    assert!(
        (high - 0.25).abs() < 1e-5 && (low + 0.25).abs() < 1e-5,
        "{high} {low}"
    );
    let (high, low) = offsets(true);
    assert!(
        (high - 0.5).abs() < 1e-5 && low.abs() < 1e-5,
        "{high} {low}"
    );
}

#[test]
fn smoothing_keeps_its_look_across_rates() {
    // Relative to the cycle: a slow and a fast smoothed square look alike,
    // a sixteenth of a cycle after the fall.
    let at = |rate: f32| {
        let l = Lfo {
            smooth: 0.5,
            rate_hz: rate,
            ..lfo(Waveform::Square)
        };
        let cycle = (1000.0 / rate) as usize;
        run(&l, cycle * 4)[cycle * 3 + cycle / 2 + cycle / 16]
    };
    // A sample's shift of the edge at the faster rate is all that differs.
    assert!((at(1.0) - at(2.0)).abs() < 0.03, "{} {}", at(1.0), at(2.0));
}
