use sad_pet::{
    dsp::{Engine, SampleBuffer},
    midi::Message,
    performance::Mode,
    preset::Preset,
};
use std::sync::Arc;

fn engine(mode: Mode) -> Engine {
    let mut preset = Preset::default();
    preset.performance.mode = mode;
    Engine::new(
        8000,
        Arc::new(SampleBuffer::new("silence".into(), 8000, vec![[0.0; 2]; 8000]).unwrap()),
        preset,
    )
}
fn on(channel: u8, note: u8, velocity: u8) -> Message {
    Message::NoteOn {
        channel,
        note,
        velocity,
    }
}
fn off(channel: u8, note: u8) -> Message {
    Message::NoteOff {
        channel,
        note,
        velocity: 0,
    }
}

#[test]
fn a_played_note_fills_and_enables_only_the_chosen_step_without_starting_transport() {
    let mut e = engine(Mode::Sequencer);
    e.preset.performance.steps[4].enabled = false;
    let before = e.preset.performance.steps;
    e.set_step_entry(Some(4), false);
    e.clear_step_notes();
    e.midi(on(15, 127, 83));
    assert_eq!(e.step_entry, Some(4));
    assert!(!e.playing);
    assert!(e.held_notes().is_empty());
    let step = e.preset.performance.steps[4];
    assert_eq!(step.notes.as_slice(), &[127]);
    assert_eq!((step.velocity, step.enabled), (83, true));
    for (i, old) in before.into_iter().enumerate() {
        if i != 4 {
            assert_eq!(e.preset.performance.steps[i], old);
        }
    }
    e.midi(off(15, 127));
    assert!(!e.playing);
    assert_eq!(e.step_edits[4], Some(step));
}

#[test]
fn capture_consumes_the_matching_note_off_without_releasing_an_arp_note() {
    let mut e = engine(Mode::SequencerArp);
    e.set_playing(true);
    e.process_frame();
    assert_eq!(e.held_notes(), vec![60]);
    let chord = e.performance_view().chord;
    let beat = e.performance_view().beat;
    e.set_step_entry(Some(3), false);
    e.midi(on(0, 60, 100));
    e.midi(off(0, 60));
    assert_eq!(e.held_notes(), vec![60]);
    assert_eq!(e.performance_view().chord, chord);
    assert_eq!(e.performance_view().beat, beat);
    e.process_frame();
    assert!(e.performance_view().beat > beat);
    // Capture stays selected so multiple notes can be entered across octaves.
    e.midi(on(0, 67, 100));
    e.midi(off(0, 67));
    assert_eq!(
        e.preset.performance.steps[3].notes.as_slice(),
        &[72, 60, 67]
    );
    assert_eq!(e.performance_view().chord, chord);
    e.set_step_entry(None, false);
    e.midi(on(0, 69, 100));
    assert_eq!(
        e.preset.performance.steps[3].notes.as_slice(),
        &[72, 60, 67]
    );
}

#[test]
fn velocity_zero_and_invalid_midi_do_not_consume_the_selected_step() {
    let mut e = engine(Mode::Sequencer);
    let before = e.preset.performance.steps;
    e.set_step_entry(Some(0), false);
    for message in [
        off(0, 60),
        on(0, 60, 0),
        on(16, 60, 100),
        on(0, 128, 100),
        on(0, 60, 128),
    ] {
        e.midi(message);
    }
    assert_eq!(e.step_entry, Some(0));
    assert_eq!(e.preset.performance.steps, before);
    e.midi(on(0, 0, 1));
    assert_eq!(e.preset.performance.steps[0].notes.as_slice(), &[60, 0]);
    assert_eq!(e.step_entry, Some(0));
}

#[test]
fn cancel_and_mode_changes_restore_normal_keyboard_behavior() {
    let mut e = engine(Mode::Sequencer);
    e.set_step_entry(Some(16), false);
    assert_eq!(e.step_entry, None);
    e.set_step_entry(Some(2), false);
    e.set_step_entry(None, false);
    let before = e.preset.performance.steps;
    e.midi(on(0, 75, 100));
    assert_eq!(e.preset.performance.steps, before);
    assert!(e.playing);
    e.set_step_entry(Some(2), false);
    let mut p = e.preset.clone();
    p.performance.mode = Mode::Keys;
    e.set_preset(p);
    assert_eq!(e.step_entry, None);
    e.set_step_entry(Some(0), false);
    assert_eq!(e.step_entry, None);
    e.midi(on(0, 64, 100));
    assert_eq!(e.held_notes(), vec![64]);
    e.midi(off(0, 64));
    assert!(e.held_notes().is_empty());
}

#[test]
fn stale_parameter_updates_cannot_erase_notes_captured_from_midi() {
    let mut e = engine(Mode::Sequencer);
    let mut stale = e.preset.clone();
    for (step, note) in [(0, 24), (8, 108), (15, 127)] {
        e.set_step_entry(Some(step), false);
        e.clear_step_notes();
        e.midi(on(0, note, 77));
        e.midi(off(0, note));
    }
    stale.params.cloud = 0.9;
    e.set_preset(stale);
    for (step, note) in [(0, 24), (8, 108), (15, 127)] {
        assert_eq!(e.preset.performance.steps[step].notes.as_slice(), &[note]);
    }
    assert_eq!(e.preset.params.cloud, 0.9);
    let echo = e.preset.clone();
    e.set_preset(echo);
    assert!(e.step_edits.iter().all(Option::is_none));
    let mut changed = e.preset.clone();
    changed.performance.steps[0].notes = [60].into();
    e.set_preset(changed);
    assert_eq!(e.preset.performance.steps[0].notes.as_slice(), &[60]);
}

#[test]
fn preset_load_can_replace_pending_edits_and_capture_state_is_not_saved() {
    let mut e = engine(Mode::Sequencer);
    let original = e.preset.clone();
    e.set_step_entry(Some(1), false);
    e.clear_step_notes();
    e.midi(on(0, 96, 95));
    let serialized = serde_json::to_string(&e.preset).unwrap();
    let saved: Preset = serde_json::from_str(&serialized).unwrap();
    assert_eq!(saved.performance.steps[1].notes.as_slice(), &[96]);
    assert!(!serialized.contains("step_entry"));
    e.set_step_entry(None, true);
    e.set_preset(original.clone());
    assert_eq!(e.preset.performance.steps, original.performance.steps);
    e.set_step_entry(Some(0), false);
    e.midi(Message::Reset);
    assert_eq!(e.step_entry, None);
    assert!(!e.playing);
}

#[test]
fn chord_entry_toggles_notes_limits_polyphony_and_consumes_releases_after_done() {
    let mut e = engine(Mode::Sequencer);
    e.set_step_entry(Some(0), false);
    e.clear_step_notes();
    for note in [48, 52, 55, 59, 62, 65, 69, 72, 76] {
        e.midi(on(0, note, 90));
        e.midi(off(0, note));
    }
    assert_eq!(
        e.preset.performance.steps[0].notes.as_slice(),
        &[48, 52, 55, 59, 62, 65, 69, 72]
    );
    e.midi(on(0, 55, 100));
    // A repeated NoteOn without its release must not toggle a second time.
    e.midi(on(0, 55, 100));
    assert!(!e.preset.performance.steps[0].notes.as_slice().contains(&55));
    e.midi(off(0, 55));
    e.midi(on(0, 76, 100));
    e.set_step_entry(None, false);
    e.midi(off(0, 76));
    assert!(!e.playing);
    assert_eq!(
        e.preset.performance.steps[0].notes.as_slice(),
        &[48, 52, 59, 62, 65, 69, 72, 76]
    );
}
