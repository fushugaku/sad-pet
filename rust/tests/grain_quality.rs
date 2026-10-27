use sad_pet::{
    dsp::{self, Engine, GrainQuality, Region, SampleBuffer},
    preset::Preset,
};
use std::{f64::consts::TAU, sync::Arc};

const RATE: u32 = 48_000;

/// Tones of `hz` at a quarter of full scale each, in a 48 kHz file.
fn tones(hz: &[f64]) -> Arc<SampleBuffer> {
    let frames = (0..RATE as usize * 2)
        .map(|i| {
            let t = i as f64 / RATE as f64;
            let x = hz.iter().map(|f| (t * f * TAU).sin()).sum::<f64>() as f32 * 0.25;
            [x, x]
        })
        .collect();
    Arc::new(SampleBuffer::new("tones".into(), RATE, frames).unwrap())
}

/// The default preset's grains at `semis`, from 0.5 to 1 s.
fn render(hz: &[f64], semis: f32, region: bool) -> Vec<f32> {
    let mut preset = Preset::default();
    preset.params.reverb = 0.0;
    preset.color_chain.clear();
    preset.params.pitch = 0.5 + semis / 72.0;
    let mut engine = Engine::new(RATE, tones(hz), preset);
    if region {
        engine
            .set_region(Region {
                start: 0.1,
                end: 0.9,
            })
            .unwrap();
    }
    engine.set_playing(true);
    for _ in 0..RATE / 2 {
        engine.process_frame();
    }
    (0..RATE / 2)
        .map(|_| {
            let frame = engine.process_frame();
            (frame[0] + frame[1]) * 0.5
        })
        .collect()
}

fn db(a: &[f32], b: &[f32]) -> f64 {
    let rms =
        |x: &[f32]| (x.iter().map(|&v| (v as f64).powi(2)).sum::<f64>() / x.len() as f64).sqrt();
    20.0 * (rms(a) / rms(b)).log10()
}

#[test]
fn eco_grains_skip_the_wide_kernels() {
    // One test: the choice holds for the whole process.
    assert_eq!(dsp::grain_quality(), GrainQuality::High);
    let high_before = render(&[3_000.0, 18_000.0], 7.0, false);
    for region in [false, true] {
        // 22-23.4 kHz a fifth up folds to 13-15 kHz.
        let folded = [22_000.0, 22_700.0, 23_400.0];
        let played = [5_000.0, 6_000.0, 7_000.0];
        dsp::set_grain_quality(GrainQuality::High);
        let high = db(&render(&folded, 7.0, region), &render(&played, 7.0, region));
        let high_unpitched = render(&[3_000.0, 18_000.0], 0.0, region);
        dsp::set_grain_quality(GrainQuality::Eco);
        assert_eq!(dsp::grain_quality(), GrainQuality::Eco);
        let eco = db(&render(&folded, 7.0, region), &render(&played, 7.0, region));
        assert!(
            high < -45.0,
            "region {region}: High folds back at {high:.1} dB"
        );
        assert!(
            eco > -20.0,
            "region {region}: Eco folds back at {eco:.1} dB"
        );
        // Grains that step one frame a read take the flat kernel either way.
        assert_eq!(render(&[3_000.0, 18_000.0], 0.0, region), high_unpitched);
    }
    dsp::set_grain_quality(GrainQuality::High);
    assert_eq!(render(&[3_000.0, 18_000.0], 7.0, false), high_before);
}
