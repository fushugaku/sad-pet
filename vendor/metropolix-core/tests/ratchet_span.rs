use metropolix_core::sequencer::{CLOCK_DIVISION_QUARTER, GateMode, Sequencer, field as f};

fn engine(span: u8, pulses: u8) -> Sequencer {
    let mut e = Sequencer::default();
    for lane in &mut e.mods {
        lane.config[9] = 1;
    }
    let step = &mut e.tracks[0].steps[0];
    step.pulses = pulses;
    step.ratchet = 4;
    step.ratchet_span = span;
    e
}
fn onsets(e: &mut Sequencer, ticks: usize) -> Vec<usize> {
    (0..ticks).filter(|_| e.tick() & 1 != 0).collect()
}

#[test]
fn auto_keeps_legacy_multi_single_and_stretched_schedules() {
    let mut e = engine(0, 4);
    assert_eq!(
        onsets(&mut e, 96),
        (0..16).map(|i| i * 6).collect::<Vec<_>>()
    );
    let mut e = engine(0, 4);
    e.tracks[0].steps[0].gate = GateMode::Single;
    assert_eq!(onsets(&mut e, 96), [0, 6, 12, 18]);
    let mut e = engine(0, 4);
    e.tracks[0].steps[0].gate = GateMode::Single;
    e.set_track(0, f::STRETCH as u32, 1);
    assert_eq!(onsets(&mut e, 96), [0, 24, 48, 72]);
    e.set_track(0, f::RATCHET_MODE as u32, 1);
    e.reset();
    assert_eq!(onsets(&mut e, 96), [0, 6, 12, 18]);
}

#[test]
fn explicit_span_sets_one_burst_without_moving_stage_or_pulses() {
    for mode in [GateMode::Single, GateMode::Multiple] {
        let mut e = engine(2, 4);
        e.tracks[0].steps[0].gate = mode;
        let mut pulses = Vec::new();
        let mut ons = Vec::new();
        for tick in 0..96 {
            if e.tick() & 1 != 0 {
                ons.push(tick);
            }
            if e.tracks[0].pulse {
                pulses.push(tick);
            }
            assert_eq!(e.stage(0), Some(0));
        }
        assert_eq!(ons, [0, 12, 24, 36]);
        assert_eq!(pulses, [0, 24, 48, 72]);
        e.tick();
        assert_eq!(e.stage(0), Some(1));
    }
}

#[test]
fn explicit_span_overrides_legacy_gate_grouping() {
    for ratchet_mode in 0..=2 {
        let mut e = engine(2, 8);
        e.set_track(0, f::STRETCH as u32, 1);
        e.set_track(0, f::PULSE_DIV as u32, 3);
        e.set_track(0, f::RATCHET_MODE as u32, ratchet_mode);
        assert_eq!(onsets(&mut e, 192), [0, 12, 24, 36]);
    }
}

#[test]
fn span_sums_swung_pulses_and_clamps_to_effective_stage_length() {
    let mut e = engine(3, 4);
    e.set_track(0, f::SWING as u32, 60);
    // 28 + 20 + 28 ticks; four onsets over that window.
    assert_eq!(onsets(&mut e, 96), [0, 19, 38, 57]);
    let mut e = engine(8, 2);
    assert_eq!(onsets(&mut e, 48), [0, 12, 24, 36]);
    let mut e = engine(8, 8);
    e.set_track(0, f::PULSE_OVERRIDE as u32, 2);
    assert_eq!(onsets(&mut e, 48), [0, 12, 24, 36]);
}

#[test]
fn span_and_count_edits_are_latched_until_next_stage_visit() {
    let mut e = engine(3, 4);
    e.set_track(0, f::LENGTH as u32, 1);
    assert_eq!(onsets(&mut e, 10), [0]);
    e.tracks[0].steps[0].ratchet_span = 1;
    e.tracks[0].steps[0].ratchet = 2;
    assert_eq!(onsets(&mut e, 86), [8, 26, 44]);
    assert_eq!(onsets(&mut e, 96), [0, 12]);
}

#[test]
fn shortened_stage_cancels_pending_burst_and_releases_its_gate() {
    let mut e = engine(8, 8);
    e.tracks[0].steps[1].gate = GateMode::Rest;
    assert_eq!(onsets(&mut e, 10), [0]);
    e.tracks[0].steps[0].pulses = 1;
    assert_eq!(onsets(&mut e, 14), []);
    assert!(e.gate(0));
    assert_eq!(e.tick() & 3, 2);
    assert!(!e.gate(0));
    assert_eq!(onsets(&mut e, 23), []);
}

#[test]
fn reset_and_live_skip_discard_pending_repeats() {
    let mut e = engine(4, 4);
    assert_eq!(onsets(&mut e, 10), [0]);
    e.tracks[0].steps[0].skip = true;
    assert_eq!(onsets(&mut e, 86), []);
    e.tracks[0].steps[0].skip = false;
    e.reset();
    assert_eq!(onsets(&mut e, 96), [0, 24, 48, 72]);
}

#[test]
fn rest_hold_gate_hold_acid_hold_and_single_repeat_keep_their_meaning() {
    let mut e = engine(4, 4);
    e.tracks[0].steps[0].gate = GateMode::Rest;
    assert_eq!(onsets(&mut e, 96), []);
    let mut e = engine(4, 4);
    e.tracks[0].steps[0].gate = GateMode::Hold;
    assert_eq!(onsets(&mut e, 96), [0]);
    assert!(e.gate(0));
    let mut e = engine(4, 4);
    e.tracks[0].steps[0].gate_override = 100;
    assert_eq!(onsets(&mut e, 96), [0]);
    assert!(e.gate(0));
    let mut e = engine(4, 4);
    e.set_track(0, f::SLIDE_TYPE as u32, 2);
    e.tracks[0].steps[0].slide = true;
    assert_eq!(onsets(&mut e, 96), [0]);
    assert!(e.gate(0));
    let mut e = engine(4, 4);
    e.tracks[0].steps[0].ratchet = 1;
    assert_eq!(onsets(&mut e, 96), [0, 24, 48, 72]);
}

#[test]
fn probability_mute_and_accumulator_use_actual_burst_events() {
    let mut e = engine(4, 4);
    e.tracks[0].steps[0].probability = 0;
    assert_eq!(onsets(&mut e, 96), []);
    let mut e = engine(4, 4);
    e.tracks[0].steps[0].accumulate = 1;
    e.tracks[0].steps[0].accumulate_trigger = 2;
    e.set_track(0, f::ACCUM_SCOPE as u32, 1);
    assert_eq!(onsets(&mut e, 96), [0, 24, 48, 72]);
    assert_eq!(e.accumulator(0), 4);
    e.reset();
    e.set_track(0, f::MUTE as u32, 1);
    assert_eq!(onsets(&mut e, 96), []);
    assert_eq!(e.accumulator(0), 0);
}

#[test]
fn very_short_span_never_spills_across_stage_boundary() {
    let mut e = engine(1, 1);
    e.set_track(0, f::DIVISION as u32, CLOCK_DIVISION_QUARTER);
    e.tracks[0].steps[0].ratchet = 8;
    e.tracks[0].steps[1].gate = GateMode::Rest;
    assert_eq!(onsets(&mut e, 12), [0, 1, 2, 3, 4, 5]);
    assert!(!e.gate(0));
}

#[test]
fn span_setter_preserves_range_and_auto_default() {
    let mut e = Sequencer::default();
    assert_eq!(e.tracks[0].steps[0].ratchet_span, 0);
    assert!(e.set_step(0, 0, 15, 8));
    assert!(!e.set_step(0, 0, 15, 9));
    assert!(!e.set_step(0, 0, 15, -1));
    assert_eq!(e.tracks[0].steps[0].ratchet_span, 8);
}
