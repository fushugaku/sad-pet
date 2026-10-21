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
