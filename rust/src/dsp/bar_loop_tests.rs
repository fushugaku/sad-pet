use super::*;

fn engine() -> Engine {
    let sample = Arc::new(
        SampleBuffer::new(
            "Loop".into(),
            12000,
            (0..48000)
                .map(|i| [(i as f32 * std::f32::consts::TAU * 120.0 / 12000.0).sin() * 0.2; 2])
                .collect(),
        )
        .unwrap(),
    );
    let mut preset = Preset::default();
    preset.params.loop_one_bar = true;
    preset.params.cloud = 0.0;
    preset.params.detune = 0.5;
    preset.params.stereo_spread = 0.0;
    preset.params.grains = 0.8;
    preset.params.filter_env_depth = 0.0;
    preset.color_chain.clear();
    for env in &mut preset.envs {
        env.attack = 0.0;
        env.decay = 0.0;
        env.sustain = 1.0;
    }
    let mut engine = Engine::new(8000, sample, preset);
    engine
        .set_region(Region {
            start: 0.125,
            end: 0.875,
        })
        .unwrap();
    engine.set_midi_gate(true);
    engine
}
fn note(engine: &mut Engine, note: u8) -> usize {
    engine.midi(Message::NoteOn {
        channel: 0,
        note,
        velocity: 100,
    });
    engine
        .voices
        .iter()
        .position(|v| v.held && v.note == note)
        .unwrap()
}
fn advance(engine: &mut Engine, frames: usize) {
    for _ in 0..frames {
        assert!(engine.process_frame().iter().all(|v| v.is_finite()));
    }
}
fn phase(engine: &Engine, voice: usize, expected: f64) {
    let error = (engine.voices[voice].loop_phase - expected + 0.5).rem_euclid(1.0) - 0.5;
    assert!(
        error.abs() < 1e-9,
        "Expected phase {expected}, got {}",
        engine.voices[voice].loop_phase
    );
}

#[test]
fn every_note_and_pitch_traverses_the_whole_selection_in_one_bar() {
    for midi_note in [12, 48, 60, 84, 120] {
        let mut e = engine();
        let voice = note(&mut e, midi_note);
        for (pitch, expected) in [(0.2, 0.25), (0.5, 0.5), (0.8, 0.75), (0.1, 0.0)] {
            e.apply_parameters(
                &crate::parameters::parse(&serde_json::json!({"pitch":pitch})).unwrap(),
            );
            advance(&mut e, 4000);
            phase(&e, voice, expected);
            let position = (e.center() - e.file_range.start as f64) / e.file_range.len() as f64;
            let error = (position - expected + 0.5).rem_euclid(1.0) - 0.5;
            assert!(error.abs() < 1e-9);
            assert!(
                e.grains
                    .iter()
                    .filter(|g| g.active)
                    .all(|g| e.file_range.contains(&(g.position as usize)))
            );
        }
    }
}

#[test]
fn overlapping_notes_start_at_the_selection_and_retrigger_independently() {
    let mut e = engine();
    let low = note(&mut e, 60);
    advance(&mut e, 4000);
    let high = note(&mut e, 72);
    phase(&e, high, 0.0);
    phase(&e, low, 0.25);
    advance(&mut e, 4000);
    phase(&e, high, 0.25);
    phase(&e, low, 0.5);
    assert_eq!(note(&mut e, 72), high);
    phase(&e, high, 0.0);
    phase(&e, low, 0.5);
    advance(&mut e, 8000);
    phase(&e, low, 0.0);
    phase(&e, high, 0.5);
}

#[test]
fn meter_and_tempo_changes_keep_phase_and_use_quarter_note_beats() {
    let mut e = engine();
    e.set_time_signature(6, 8);
    let voice = note(&mut e, 60);
    advance(&mut e, 6000);
    phase(&e, voice, 0.5);
    e.host_transport(240.0, None, false);
    e.refresh_parameters();
    phase(&e, voice, 0.5);
    advance(&mut e, 3000);
    phase(&e, voice, 0.0);
    e.set_time_signature(0, 0);
    assert_eq!(e.bar_beats(), 4.0);
}

#[test]
fn root_pitch_and_bend_change_grain_pitch_without_retiming_the_loop() {
    let mut e = engine();
    e.apply_parameters(
        &crate::parameters::parse(&serde_json::json!({"sample_root_note":72})).unwrap(),
    );
    let voice = note(&mut e, 72);
    advance(&mut e, 1000);
    assert!(
        e.grains
            .iter()
            .any(|g| g.active && (g.rate - 1.5).abs() < 1e-6)
    );
    e.apply_parameters(&crate::parameters::parse(&serde_json::json!({"pitch":2.0/3.0})).unwrap());
    e.midi(Message::PitchBend {
        channel: 0,
        value: 12288,
    });
    advance(&mut e, 3000);
    phase(&e, voice, 0.25);
    let expected = 1.5 * 2.0_f64.powf(13.0 / 12.0);
    assert!(
        e.grains
            .iter()
            .any(|g| g.active && (g.rate - expected).abs() < 1e-4)
    );
}

#[test]
fn changing_the_selection_preserves_phase_and_disabling_restores_original_speed() {
    let mut e = engine();
    let voice = note(&mut e, 60);
    advance(&mut e, 4000);
    e.set_region(Region {
        start: 0.25,
        end: 0.5,
    })
    .unwrap();
    phase(&e, voice, 0.25);
    advance(&mut e, 4000);
    phase(&e, voice, 0.5);
    e.apply_parameters(&crate::parameters::parse(&serde_json::json!({"loop_one_bar":0})).unwrap());
    assert_eq!(e.held_notes(), vec![60]);
    let start = e.center();
    advance(&mut e, 1000);
    assert!((e.center() - start - 1500.0).abs() < 1e-6);
}

#[test]
fn rendered_pitch_changes_while_the_loop_duration_stays_the_same() {
    let mut crossings = Vec::new();
    for midi_note in [60, 72] {
        let mut e = engine();
        let voice = note(&mut e, midi_note);
        advance(&mut e, 4000);
        let mut previous = 0.0;
        let mut count = 0;
        let mut peak = 0.0_f32;
        for _ in 0..8000 {
            let value = e.process_frame()[0];
            peak = peak.max(value.abs());
            if previous <= 0.0 && value > 0.0 {
                count += 1;
            }
            previous = value;
        }
        assert!(peak > 0.001);
        phase(&e, voice, 0.75);
        crossings.push(count);
    }
    assert!(
        (1.8..2.2).contains(&(crossings[1] as f64 / crossings[0] as f64)),
        "Crossings: {crossings:?}"
    );
}
