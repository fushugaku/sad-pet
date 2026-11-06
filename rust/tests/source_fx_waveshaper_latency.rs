use sad_pet::{
    Stereo,
    color::{ColorSlot, WaveshaperMode},
    dsp::{Region, SampleBuffer, color::ColorProcessor, source_fx, waveshaper::Processor},
    preset::SaturatorType,
    waveshaper::Quality,
};
use std::{cell::Cell, sync::Arc};

fn sample(rate: u32, length: usize) -> Arc<SampleBuffer> {
    let mut frames: Vec<Stereo> = (0..length)
        .map(|i| [0.3 * (i as f32 * 0.17).sin(), 0.2 * (i as f32 * 0.11).cos()])
        .collect();
    frames[0] = [0.73, -0.41];
    frames[length - 1] = [-0.57, 0.38];
    Arc::new(SampleBuffer::new("stereo edges".into(), rate, frames).unwrap())
}

fn slot(quality: Quality, mix: f32) -> ColorSlot {
    let mut slot = ColorSlot::legacy(SaturatorType::Waveshaper, mix);
    slot.settings.waveshaper_type = WaveshaperMode::HardClip;
    slot.settings.waveshaper_depth = 0.8;
    slot.settings.waveshaper_options.quality = quality;
    slot
}

#[test]
fn clean_source_cache_preserves_the_file_grid_at_both_edges() {
    for rate in [8_000, 44_100, 48_000, 96_000] {
        for quality in [Quality::Clean2x, Quality::Clean4x] {
            // A one-frame source and sources shorter than the FIR delay must
            // also flush all of their samples onto the original grid.
            for length in [1, 7, 31, 127, 1024] {
                let original = sample(rate, length);
                // Exactly zero Mix is intentionally uncached. A tiny positive
                // Mix exercises the prepared path while being effectively dry.
                let processed = source_fx::prepare(
                    original.clone(),
                    Region::default(),
                    Some(slot(quality, 1e-8)),
                    120.0,
                    60.0,
                    || false,
                )
                .unwrap()
                .unwrap();
                assert_eq!(processed.range, 0..length);
                assert_eq!(processed.sample.sample_rate, rate);
                assert_eq!(processed.sample.frames().len(), length);
                for (i, (actual, expected)) in processed
                    .sample
                    .frames()
                    .iter()
                    .zip(original.frames())
                    .enumerate()
                {
                    for channel in 0..2 {
                        assert!(
                            (actual[channel] - expected[channel]).abs() < 1e-6,
                            "{quality:?} {rate} Hz, length {length}, frame {i}, channel {channel}: {} != {}",
                            actual[channel],
                            expected[channel],
                        );
                    }
                }
            }
            let original = sample(rate, 64);
            assert!(
                source_fx::prepare(
                    original,
                    Region::default(),
                    Some(slot(quality, 0.0)),
                    120.0,
                    60.0,
                    || false,
                )
                .unwrap()
                .is_none(),
                "a fully dry source must keep using the original file"
            );
        }
    }
}

#[test]
fn clean_selected_region_matches_a_flushed_realtime_processor() {
    for quality in [Quality::Clean2x, Quality::Clean4x] {
        let rate = 48_000;
        let original = sample(rate, 1024);
        let region = Region {
            start: 0.125,
            end: 0.875,
        };
        let range = region.bounds(original.len()).unwrap();
        let effect = slot(quality, 0.65);
        let mut latency_probe = Processor::new(rate);
        latency_probe.configure(
            effect.settings.waveshaper_type,
            effect.settings.waveshaper_depth,
            effect.settings.waveshaper_options,
        );
        let latency = latency_probe.latency_samples();
        assert!(latency > 0);
        let mut realtime = ColorProcessor::new(rate);
        realtime.set_pitch(60.0);
        realtime.configure_tempo(effect, 120.0);
        let expected: Vec<_> = original.frames()[range.clone()]
            .iter()
            .copied()
            .chain(std::iter::repeat_n([0.0; 2], latency))
            .map(|frame| realtime.process(frame))
            .skip(latency)
            .collect();
        let processed = source_fx::prepare(original, region, Some(effect), 120.0, 60.0, || false)
            .unwrap()
            .unwrap();
        assert_eq!(processed.range, range);
        assert_eq!(processed.sample.frames(), expected);
    }
}

#[test]
fn cancelling_during_the_flush_discards_the_cache() {
    let original = sample(48_000, 4096);
    let checks = Cell::new(0);
    let result = source_fx::prepare(
        original,
        Region::default(),
        Some(slot(Quality::Clean4x, 0.5)),
        120.0,
        60.0,
        || {
            checks.set(checks.get() + 1);
            checks.get() == 2
        },
    );
    assert!(result.is_err());
    assert_eq!(checks.get(), 2);
}
