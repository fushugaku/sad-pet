use sad_pet::{
    dsp::{Engine, SampleBuffer},
    midi::Message,
    performance::{Direction, Mode, Performance, Settings},
    preset::Preset,
};
use std::sync::Arc;

fn on(note: u8) -> Message {
    Message::NoteOn {
        channel: 0,
        note,
        velocity: 100,
    }
}
fn off(note: u8) -> Message {
    Message::NoteOff {
        channel: 0,
        note,
        velocity: 0,
    }
}
fn events(p: &mut Performance, frames: usize) -> Vec<(usize, Message)> {
    (0..frames)
        .flat_map(|frame| {
            p.tick(1000, 60.0)
                .into_iter()
                .flatten()
                .map(move |event| (frame, event))
        })
        .collect()
}
fn notes(events: &[(usize, Message)]) -> Vec<(usize, u8)> {
    events
        .iter()
        .filter_map(|&(frame, event)| match event {
            Message::NoteOn { note, .. } => Some((frame, note)),
            _ => None,
        })
        .collect()
}

#[test]
fn arpeggiator_uses_sample_clock_and_releases_each_gate() {
    let mut p = Performance::new(Settings {
        mode: Mode::Arp,
        arp_gate: 0.5,
        ..Settings::default()
    });
    p.start();
    assert_eq!(
        events(&mut p, 751),
        vec![
            (0, on(60)),
            (125, off(60)),
            (250, on(64)),
            (375, off(64)),
            (500, on(67)),
            (625, off(67)),
            (750, on(60)),
        ]
    );
}

#[test]
fn arpeggiator_orders_octaves_and_bounces_without_repeating_endpoints() {
    for (direction, octaves, expected) in [
        (Direction::Up, 2, vec![60, 64, 67, 72, 76, 79, 60]),
        (Direction::Down, 2, vec![79, 76, 72, 67, 64, 60, 79]),
        (Direction::UpDown, 1, vec![60, 64, 67, 64, 60, 64, 67]),
        (Direction::AsPlayed, 1, vec![67, 60, 64, 67, 60, 64, 67]),
    ] {
        let mut p = Performance::new(Settings {
            mode: Mode::Arp,
            direction,
            octaves,
            chord: vec![67, 60, 64],
            ..Settings::default()
        });
        p.start();
        assert_eq!(
            notes(&events(&mut p, 1501))
                .into_iter()
                .map(|(_, n)| n)
                .collect::<Vec<_>>(),
            expected
        );
    }
}

#[test]
fn random_arp_restarts_deterministically_and_single_note_bounce_is_valid() {
    let mut p = Performance::new(Settings {
        mode: Mode::Arp,
        direction: Direction::Random,
        octaves: 4,
        ..Settings::default()
    });
    p.start();
    let first = events(&mut p, 2000);
    p.stop();
    p.start();
    assert_eq!(events(&mut p, 2000), first);
    let mut p = Performance::new(Settings {
        mode: Mode::Arp,
        direction: Direction::UpDown,
        chord: vec![127],
        octaves: 4,
        ..Settings::default()
    });
    p.start();
    assert!(notes(&events(&mut p, 2000)).iter().all(|(_, n)| *n == 127));
}

#[test]
fn arp_hold_latches_a_chord_replaces_it_and_releases_when_disabled() {
    let mut settings = Settings {
        mode: Mode::Arp,
        ..Settings::default()
    };
    let mut p = Performance::new(settings.clone());
    p.start();
    p.input(on(72));
    p.input(on(76));
    p.input(off(72));
    p.input(off(76));
    assert_eq!(p.view().chord, vec![72, 76]);
    assert_eq!(notes(&events(&mut p, 251)), vec![(0, 72), (250, 76)]);
    p.input(on(79));
    assert_eq!(p.view().chord, vec![79]);
    p.input(off(79));
    settings.chord = p.stored_chord().to_vec();
    settings.hold = false;
    p.configure(&settings);
    assert!(p.view().chord.is_empty());
    assert!(p.stored_chord().is_empty());
    assert_eq!(events(&mut p, 1), vec![(0, off(76))]);
    p.input(on(62));
    assert_eq!(p.view().chord, vec![62]);
    p.input(off(62));
    assert!(p.view().chord.is_empty());
}

#[test]
fn note_hold_toggles_and_clear_drops_both_chord_and_physical_keys() {
    let mut p = Performance::new(Settings {
        mode: Mode::Hold,
        ..Settings::default()
    });
    assert_eq!(p.input(on(60)), Some([Some(on(60)), None]));
    assert_eq!(p.input(off(60)), Some([None; 2]));
    assert_eq!(p.view().chord, vec![60]);
    p.input(on(64));
    p.input(off(64));
    assert_eq!(p.input(on(60)), Some([Some(off(60)), None]));
    assert_eq!(p.view().chord, vec![64]);
    p.clear();
    assert!(p.stored_chord().is_empty());
    assert!(p.view().chord.is_empty());
    assert_eq!(p.input(on(64)), Some([Some(on(64)), None]));
}

#[test]
fn hold_disabled_cannot_restore_a_stale_chord_when_another_control_changes() {
    let mut settings = Settings {
        mode: Mode::Arp,
        hold: false,
        ..Settings::default()
    };
    let mut p = Performance::new(settings.clone());
    p.start();
    assert!(p.view().chord.is_empty());
    assert!(events(&mut p, 251).is_empty());
    p.input(on(62));
    // A delayed UI update may still contain an old latched chord.
    settings.direction = Direction::Down;
    p.configure(&settings);
    assert_eq!(p.view().chord, vec![62]);
    assert_eq!(p.stored_chord(), [62]);
    p.input(off(62));
    settings.octaves = 3;
    p.configure(&settings);
    assert!(p.view().chord.is_empty());
    assert!(p.stored_chord().is_empty());
    let mut hold = Performance::new(Settings {
        mode: Mode::Hold,
        ..Settings::default()
    });
    hold.clear();
    let (_, len) = hold.hold_start();
    assert_eq!(len, 0);
}

#[test]
fn sequencer_respects_rests_length_velocity_and_gate() {
    let mut settings = Settings {
        mode: Mode::Sequencer,
        length: 3,
        sequence_division: 0.5,
        sequence_gate: 0.5,
        ..Settings::default()
    };
    settings.steps[1].enabled = false;
    settings.steps[2].velocity = 77;
    let mut p = Performance::new(settings);
    p.start();
    assert_eq!(p.input(on(90)), Some([None; 2]));
    assert_eq!(
        events(&mut p, 1501),
        vec![
            (0, on(60)),
            (250, off(60)),
            (
                1000,
                Message::NoteOn {
                    channel: 0,
                    note: 67,
                    velocity: 77
                }
            ),
            (1250, off(67)),
            (1500, on(60))
        ]
    );
    assert_eq!(p.view().step, Some(0));
}

#[test]
fn sequencer_starts_and_releases_whole_chords_on_the_same_sample() {
    let mut settings = Settings {
        mode: Mode::Sequencer,
        length: 3,
        sequence_division: 0.5,
        sequence_gate: 0.5,
        ..Settings::default()
    };
    settings.steps[0].notes = [60, 64, 67].into();
    settings.steps[1].notes = [62, 65, 69].into();
    settings.steps[2].notes = [].into();
    let mut p = Performance::new(settings);
    p.start();
    assert_eq!(
        events(&mut p, 1501),
        vec![
            (0, on(60)),
            (0, on(64)),
            (0, on(67)),
            (250, off(60)),
            (250, off(64)),
            (250, off(67)),
            (500, on(62)),
            (500, on(65)),
            (500, on(69)),
            (750, off(62)),
            (750, off(65)),
            (750, off(69)),
            (1500, on(60)),
            (1500, on(64)),
            (1500, on(67)),
        ]
    );
}

#[test]
fn changing_step_rate_releases_all_eight_previous_notes_before_retriggering() {
    let mut settings = Settings {
        mode: Mode::Sequencer,
        length: 2,
        ..Settings::default()
    };
    let first = [48, 52, 55, 59, 60, 64, 67, 71];
    let second = [50, 53, 57, 60, 62, 65, 69, 72];
    settings.steps[0].notes = first.into();
    settings.steps[1].notes = second.into();
    let mut p = Performance::new(settings.clone());
    p.start();
    assert_eq!(
        events(&mut p, 1),
        first.into_iter().map(|n| (0, on(n))).collect::<Vec<_>>()
    );
    settings.sequence_division = 0.0625;
    p.configure(&settings);
    assert_eq!(
        events(&mut p, 1),
        first
            .into_iter()
            .map(|n| (0, off(n)))
            .chain(second.into_iter().map(|n| (0, on(n))))
            .collect::<Vec<_>>()
    );
    let tail = events(&mut p, 50);
    for note in second {
        assert!(tail.iter().any(|(_, message)| *message == off(note)));
    }
    assert!(!p.view().gate);
}

#[test]
fn chord_steps_round_trip_and_legacy_single_notes_still_load() {
    use sad_pet::performance::{Step, StepNotes};
    let legacy: Step =
        serde_json::from_str(r#"{"note":73,"velocity":86,"enabled":false}"#).unwrap();
    assert_eq!(legacy.notes.as_slice(), &[73]);
    assert_eq!((legacy.velocity, legacy.enabled), (86, false));
    let chord: Step =
        serde_json::from_str(r#"{"notes":[60,64,67,60,72,76,79,84,88,91,255]}"#).unwrap();
    assert_eq!(chord.notes.as_slice(), &[60, 64, 67, 72, 76, 79, 84, 88]);
    let json = serde_json::to_string(&chord).unwrap();
    assert_eq!(serde_json::from_str::<Step>(&json).unwrap(), chord);
    let rest: Step = serde_json::from_str(r#"{"notes":[],"note":60}"#).unwrap();
    assert!(rest.notes.as_slice().is_empty());
    let mut edge: StepNotes = [0, 4, 127].into();
    edge.transpose(12);
    assert_eq!(edge.as_slice(), &[12, 16, 127]);
    edge.transpose(127);
    assert_eq!(edge.as_slice(), &[127]);
}

#[test]
fn sequencer_arpeggiates_each_steps_chord_and_rest_cuts_a_note_between_pulses() {
    let mut settings = Settings {
        mode: Mode::SequencerArp,
        length: 3,
        sequence_division: 0.5,
        arp_division: 0.25,
        ..Settings::default()
    };
    settings.steps[0].notes = [60, 64, 67].into();
    settings.steps[1].notes = [62, 65, 69].into();
    settings.steps[2].enabled = false;
    let mut p = Performance::new(settings);
    p.start();
    assert_eq!(
        notes(&events(&mut p, 1501)),
        vec![(0, 60), (250, 64), (500, 62), (750, 65), (1500, 60)]
    );
    let mut settings = Settings {
        mode: Mode::SequencerArp,
        length: 2,
        sequence_division: 0.0625,
        arp_division: 1.0,
        arp_gate: 0.95,
        ..Settings::default()
    };
    settings.steps[1].enabled = false;
    let mut p = Performance::new(settings);
    p.start();
    assert_eq!(events(&mut p, 64), vec![(0, on(60)), (63, off(60))]);
}

fn engine(preset: Preset) -> Engine {
    let frames = (0..8000)
        .map(|i| {
            let v = (i as f32 / 8000.0 * 220.0 * std::f32::consts::TAU).sin() * 0.2;
            [v, v]
        })
        .collect();
    Engine::new(
        8000,
        Arc::new(SampleBuffer::new("tone".into(), 8000, frames).unwrap()),
        preset,
    )
}

#[test]
fn hold_preset_restores_all_notes_and_keyboard_does_not_add_a_default_note() {
    let mut preset = Preset::default();
    preset.performance.mode = Mode::Hold;
    preset.performance.chord = vec![62, 65, 69];
    let mut e = engine(preset.clone());
    // Updating an unrelated knob before Play must retain the saved held chord.
    preset.params.cloud = 0.4;
    e.set_preset(preset);
    e.set_playing(true);
    assert_eq!(e.held_notes(), vec![62, 65, 69]);
    e.set_playing(false);
    assert!(e.held_notes().is_empty());
    e.set_playing(true);
    assert_eq!(e.held_notes(), vec![62, 65, 69]);
    e.set_playing(false);
    e.midi(on(71));
    e.midi(off(71));
    assert_eq!(e.held_notes(), vec![71]);
    assert_eq!(e.preset.performance.chord, vec![71]);
    e.clear_hold();
    e.set_playing(false);
    e.midi(on(74));
    e.midi(off(74));
    assert_eq!(e.held_notes(), vec![74]);
    e.midi(on(74));
    assert!(e.held_notes().is_empty());
}

#[test]
fn mode_changes_and_restarts_release_old_voices() {
    let mut preset = Preset::default();
    preset.performance.mode = Mode::Arp;
    let mut e = engine(preset.clone());
    e.set_playing(true);
    e.process_frame();
    assert_eq!(e.held_notes(), vec![60]);
    e.set_playing(true);
    assert!(e.held_notes().is_empty());
    e.process_frame();
    assert_eq!(e.held_notes(), vec![60]);
    preset.performance.mode = Mode::Sequencer;
    preset.performance.steps[0].notes = [72].into();
    e.set_preset(preset.clone());
    assert!(e.held_notes().is_empty());
    e.process_frame();
    assert_eq!(e.held_notes(), vec![72]);
    preset.performance.mode = Mode::Keys;
    e.set_preset(preset);
    assert!(e.held_notes().is_empty());
    e.set_playing(false);
    assert!(e.held_notes().is_empty());
}

#[test]
fn new_modes_produce_identical_audio_for_different_block_sizes_and_save_in_presets() {
    for mode in [Mode::Hold, Mode::Arp, Mode::Sequencer, Mode::SequencerArp] {
        let mut preset = Preset::default();
        preset.performance.mode = mode;
        preset.performance.steps[0].notes = [60, 64, 67].into();
        preset.performance.steps[2].notes = [62, 65, 69].into();
        preset.performance.steps[1].enabled = false;
        preset.performance.sequence_division = 0.25;
        let json = serde_json::to_string(&preset).unwrap();
        let restored: Preset = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.performance, preset.performance);
        let mut a = engine(preset);
        let mut b = engine(restored);
        a.set_playing(true);
        b.set_playing(true);
        let mut expected = vec![[0.0; 2]; 8000];
        a.process(&mut expected);
        let mut actual = vec![[0.0; 2]; 8000];
        for block in actual.chunks_mut(73) {
            b.process(block);
        }
        assert_eq!(actual, expected, "{mode:?}");
        assert!(actual.iter().flatten().any(|x| x.abs() > 0.01), "{mode:?}");
        assert!(actual.iter().flatten().all(|x| x.is_finite()), "{mode:?}");
    }
    let mut old = serde_json::to_value(Preset::default()).unwrap();
    old.as_object_mut().unwrap().remove("performance");
    let restored: Preset = serde_json::from_value(old).unwrap();
    assert_eq!(restored.performance.mode, Mode::Keys);
}
