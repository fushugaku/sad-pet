use sad_pet::{
    dsp::{Engine, SampleBuffer, sample::LiveInput},
    preset::Preset,
};
use std::sync::Arc;

fn engine(preset: Preset) -> Engine {
    let sample = SampleBuffer::new("Original file".into(), 8_000, vec![[0.5; 2]; 800]).unwrap();
    Engine::new(8_000, Arc::new(sample), preset)
}

#[test]
fn live_effects_use_input_instead_of_the_loaded_file() {
    let mut engine = engine(Preset::default());
    engine.set_live_input(true, true);
    engine.set_playing(true);
    for _ in 0..4_000 {
        assert_eq!(engine.process_input_frame([0.0; 2]), [0.0; 2]);
    }
    let mut peak = [0.0_f32; 2];
    for i in 0..8_000 {
        let x = (i as f32 * 0.1).sin() * 0.2;
        let output = engine.process_input_frame([x, -x]);
        assert!(output.iter().all(|x| x.is_finite()));
        for c in 0..2 {
            peak[c] = peak[c].max(output[c].abs());
        }
    }
    assert!(peak.iter().all(|x| *x > 0.05));
    assert_eq!(engine.active_grains(), 0);
    assert_eq!(engine.sample.name, "Original file");
    engine.set_live_input(false, false);
    assert_eq!(engine.source_sample().name, "Original file");
    assert_eq!(engine.source_sample().duration(), 0.1);
    engine.set_live_input(true, true);
    engine.set_playing(true);
    for _ in 0..1_000 {
        engine.process_input_frame([0.2; 2]);
    }
    engine.set_playing(false);
    engine.set_live_input(false, false);
    for _ in 0..4_000 {
        assert_eq!(
            engine.process_frame(),
            [0.0; 2],
            "stopping input must not play the old file's release tail"
        );
    }
}

#[test]
fn live_grains_read_the_captured_buffer_and_freeze_it() {
    let mut engine = engine(Preset::default());
    engine.set_live_input(true, false);
    engine.set_playing(true);
    let mut peak = 0.0_f32;
    for i in 0..16_000 {
        let x = (i as f32 * 0.1).sin() * 0.3;
        let output = engine.process_input_frame([x; 2]);
        if i > 8_000 {
            peak = peak.max(output[0].abs());
        }
    }
    assert!(peak > 0.01, "captured audio must reach the grains");
    assert!(engine.active_grains() > 0);
    let captured = engine.live_input.as_ref().unwrap().capture();
    engine.set_frozen(true);
    for _ in 0..8_000 {
        engine.process_input_frame([0.0; 2]);
    }
    assert_eq!(captured, engine.live_input.as_ref().unwrap().capture());
    engine.set_frozen(false);
    engine.process_input_frame([0.0; 2]);
    assert_ne!(
        captured.len(),
        engine.live_input.as_ref().unwrap().capture().len()
    );
    let replacement =
        Arc::new(SampleBuffer::new("New file".into(), 8_000, vec![[0.0; 2]; 8]).unwrap());
    engine.set_sample(replacement.clone());
    assert!(engine.live_input.is_none());
    assert!(Arc::ptr_eq(&engine.sample, &replacement));
}

#[test]
fn input_capture_is_chronological_after_wrap_and_ignores_nonfinite_values() {
    let mut input = LiveInput::new(8_000, true);
    assert!(input.capture().is_empty());
    for i in 0..72_000 {
        input.push([i as f32 / 72_000.0, -(i as f32) / 72_000.0], false);
    }
    let captured = input.capture();
    assert_eq!(input.seconds(), 8.0);
    assert_eq!(captured.len(), 64_000);
    assert_eq!(captured[0], [8_000.0 / 72_000.0, -8_000.0 / 72_000.0]);
    assert_eq!(
        captured[63_999],
        [71_999.0 / 72_000.0, -71_999.0 / 72_000.0]
    );
    input.push([f32::NAN, f32::INFINITY], false);
    assert_eq!(input.capture().last(), Some(&[0.0; 2]));
    assert!(
        input
            .sample
            .prepared_levels()
            .iter()
            .flatten()
            .flatten()
            .all(|x| x.is_finite())
    );
}

#[test]
fn live_input_passes_through_the_filter() {
    fn energy(cutoff: f32) -> f32 {
        let mut preset = Preset::default();
        preset.params.filter = cutoff;
        preset.params.filter_env_depth = 0.0;
        let mut engine = engine(preset);
        engine.set_live_input(true, true);
        engine.set_playing(true);
        let mut sum = 0.0;
        for i in 0..8_000 {
            let x = (i as f32 * std::f32::consts::TAU * 1_000.0 / 8_000.0).sin() * 0.2;
            let output = engine.process_input_frame([x; 2]);
            if i > 4_000 {
                sum += output[0].powi(2);
            }
        }
        sum
    }
    assert!(energy(0.2) < energy(1.0) * 0.01);
}
