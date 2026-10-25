use sad_pet::{
    dsp::{Engine, SampleBuffer},
    parameters,
    preset::{Destination, Lfo, Preset, Waveform},
};
use std::sync::Arc;

fn drum_engine(granular: bool) -> Engine {
    pattern_engine(granular, &[(0, 1.0), (4, 0.85)])
}
fn pattern_engine(granular: bool, steps: &[(usize, f32)]) -> Engine {
    let mut p = Preset {
        drums: Some(Box::default()),
        color_chain: vec![],
        ..Preset::default()
    };
    let drums = p.drums.as_mut().unwrap();
    drums.granular = granular;
    for &(step, velocity) in steps {
        drums.tracks[0].steps[step].velocity = velocity;
    }
    p.params.grains = 0.75;
    p.params.cloud = 0.0;
    p.params.grain_size = 0.72;
    p.params.pitch = 0.75;
    p.params.mix = 1.0;
    let mut engine = Engine::new(8000, Arc::new(SampleBuffer::demo(8000)), p);
    engine.set_midi_gate(false);
    engine.set_playing(true);
    engine
}

#[test]
fn drum_granular_routing_changes_audio_and_can_be_toggled_without_rebuilding_the_pattern() {
    let mut dry = drum_engine(false);
    let mut wet = drum_engine(true);
    let (mut energy, mut difference) = (0.0, 0.0);
    let mut grains = 0;
    for _ in 0..16000 {
        let a = dry.process_frame();
        let b = wet.process_frame();
        assert!(a.into_iter().chain(b).all(f32::is_finite));
        energy += b[0] * b[0];
        difference += (a[0] - b[0]).powi(2);
        grains = grains.max(wet.active_grains());
    }
    assert!(energy > 0.01, "The granular drum output was silent");
    assert!(
        difference > 0.01,
        "Granular routing still emitted direct drums"
    );
    assert!(grains > 0);
    let before = dry.drum_view().positions;
    dry.apply_parameters(&parameters::parse(&serde_json::json!({"drum_granular":1})).unwrap());
    assert_eq!(dry.drum_view().positions, before);
    assert!(dry.preset.drums.as_ref().unwrap().granular);
    for _ in 0..2000 {
        dry.process_frame();
    }
    assert!(dry.active_grains() > 0);
}

#[test]
fn grain_pitch_leaves_the_direct_drums_at_their_own_tuning() {
    // Pitch moved while Granular is on, then Granular switched off.
    let mut moved = drum_engine(true);
    moved.apply_parameters(
        &parameters::parse(&serde_json::json!({"pitch":0.9,"drum_granular":0})).unwrap(),
    );
    let mut reference = drum_engine(false);
    reference.apply_parameters(&parameters::parse(&serde_json::json!({"pitch":0.5})).unwrap());
    let mut energy = 0.0;
    for _ in 0..16000 {
        let (a, b) = (moved.process_frame(), reference.process_frame());
        energy += a[0] * a[0];
        assert!(
            (a[0] - b[0]).abs() < 1e-4,
            "Grain pitch transposed the direct drums"
        );
    }
    assert!(energy > 0.01);
}

fn shift(engine: &mut Engine, steps: i32) {
    engine.apply_parameters(
        &parameters::parse(&serde_json::json!({"drum1_note_shift": steps})).unwrap(),
    );
}
fn assert_same_audio(a: &mut Engine, b: &mut Engine, frames: usize) {
    let mut energy = 0.0;
    for frame in 0..frames {
        let (x, y) = (a.process_frame(), b.process_frame());
        energy += x[0] * x[0];
        assert_eq!(x, y, "The shifted pattern diverged at frame {frame}");
    }
    assert!(energy > 0.01, "The pattern was silent");
}

#[test]
fn pattern_note_shift_plays_the_steps_elsewhere_and_leaves_them_in_place() {
    // Steps 1 and 5 shifted right by 2 sound like steps 3 and 7.
    let mut shifted = drum_engine(false);
    shift(&mut shifted, 2);
    assert_same_audio(
        &mut shifted,
        &mut pattern_engine(false, &[(2, 1.0), (6, 0.85)]),
        16000,
    );
    assert_eq!(shifted.drum_view().shifts[0], 2);
    assert_eq!(shifted.drum_view().shifts[1..], [0; 6]);
    let track = &shifted.preset.drums.as_ref().unwrap().tracks[0];
    assert_eq!(track.shift, 2);
    assert_eq!(
        (track.steps[0].velocity, track.steps[2].velocity),
        (1.0, 0.0)
    );

    // A left shift wraps inside the track length.
    let mut shifted = drum_engine(false);
    shift(&mut shifted, -1);
    assert_same_audio(
        &mut shifted,
        &mut pattern_engine(false, &[(15, 1.0), (3, 0.85)]),
        16000,
    );

    // A step added while shifted is placed where it was set and plays shifted.
    let mut shifted = drum_engine(false);
    shift(&mut shifted, 2);
    let edit = [sad_pet::drums::Edit {
        track: 0,
        step: Some(8),
        field: 0,
        value: 0.7,
    }];
    shifted.apply_drum_edits(&edit);
    assert_eq!(
        shifted.preset.drums.as_ref().unwrap().tracks[0].steps[8].velocity,
        0.7
    );
    let mut reference = pattern_engine(false, &[(2, 1.0), (6, 0.85), (10, 0.7)]);
    assert_same_audio(&mut shifted, &mut reference, 16000);
}

#[test]
fn modulation_shifts_the_drum_pattern_without_moving_its_steps() {
    let mut engine = drum_engine(false);
    let index = parameters::specs()
        .iter()
        .position(|s| s.id == "drum1_note_shift")
        .unwrap() as u16;
    engine.preset.lfos[0] = Lfo {
        waveform: Waveform::Square,
        rate_hz: 2.0,
        depth: 0.25,
        destination: Destination::Control { index },
        ..Lfo::default()
    };
    let mut seen = std::collections::BTreeSet::new();
    for _ in 0..8000 {
        engine.process_frame();
        seen.insert(engine.drum_view().shifts[0]);
    }
    // Depth 0.25 swings ±4 of the ±16 steps.
    assert!(seen.contains(&-4) && seen.contains(&4), "{seen:?}");
    assert!(seen.iter().all(|s| s.abs() <= 4), "{seen:?}");
    let track = &engine.preset.drums.as_ref().unwrap().tracks[0];
    assert_eq!(track.shift, 0);
    assert_eq!(
        (track.steps[0].velocity, track.steps[4].velocity),
        (1.0, 0.85)
    );
}
