//! CLOUD scatters grains over a share of the whole source, or in Time mode
//! over a span of time that does not grow with the source.
use sad_pet::{
    dsp::{Engine, SampleBuffer},
    preset::Preset,
};
use std::sync::Arc;

/// How far the live grains sit from their mean, in seconds of the 8 s demo.
fn spread(time: bool, cloud: f32) -> f32 {
    let mut preset = Preset::default();
    // 128 grains a second of 140 ms each, so a snapshot holds about 17.
    preset.params.grains = 1.0;
    preset.params.grain_size = 0.7;
    preset.params.position = 0.5;
    preset.params.cloud = cloud;
    preset.params.cloud_time = time;
    let mut e = Engine::new(48_000, Arc::new(SampleBuffer::demo(48_000)), preset);
    e.set_playing(true);
    for _ in 0..24_000 {
        e.process_frame();
    }
    let seconds: Vec<f32> = e.grain_views().iter().map(|g| g.position * 8.0).collect();
    assert!(seconds.len() > 8, "{} grains", seconds.len());
    let mean = seconds.iter().sum::<f32>() / seconds.len() as f32;
    (seconds.iter().map(|s| (s - mean).powi(2)).sum::<f32>() / seconds.len() as f32).sqrt()
}

#[test]
fn time_mode_scatters_grains_over_a_span_of_time() {
    // 12% of the 8 s source spans 0.96 s; in Time mode 12% is
    // 4 s × 0.12² = 58 ms, so the cloud stays close to POSITION.
    let region = spread(false, 0.12);
    let timed = spread(true, 0.12);
    assert!(region > 0.2, "{region}");
    assert!(timed < 0.08 && region > timed * 3.0, "{timed} vs {region}");
    // Half the knob spans a quarter of 4 s, about what 12% of 8 s does.
    let half = spread(true, 0.5);
    assert!(
        half > region * 0.6 && half < region * 1.6,
        "{half} vs {region}"
    );
    // Older presets, without the field, keep the share of the source.
    let preset: Preset =
        serde_json::from_value(serde_json::json!({"params": {"cloud": 0.3}})).unwrap();
    assert!(!preset.params.cloud_time);
}
