use sad_pet::{
    dsp::{Engine, SampleBuffer},
    parameters,
    preset::Preset,
};
use std::sync::Arc;

fn drum_engine(granular: bool) -> Engine {
    let mut p = Preset {
        drums: Some(Box::default()),
        color_chain: vec![],
        ..Preset::default()
    };
    let drums = p.drums.as_mut().unwrap();
    drums.granular = granular;
    drums.tracks[0].steps[0].velocity = 1.0;
    drums.tracks[0].steps[4].velocity = 0.85;
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
        assert!((a[0] - b[0]).abs() < 1e-4, "Grain pitch transposed the direct drums");
    }
    assert!(energy > 0.01);
}
