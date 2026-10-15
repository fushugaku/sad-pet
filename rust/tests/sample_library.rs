use sad_pet::{samples, wav};
use std::{collections::HashSet, io::Cursor, sync::Arc};

#[test]
fn every_track_has_unique_identity_valid_audio_and_exact_duration() {
    let mut identities = HashSet::new();
    let mut fingerprints = HashSet::new();
    for (index, track) in samples::TRACKS.iter().enumerate() {
        assert!(identities.insert(track.id));
        let sample = samples::generate(index, 8_000);
        assert_eq!(sample.name, track.name);
        assert_eq!(sample.library_id, Some(format!("factory:{}:v1", track.id)));
        assert_eq!(sample.len(), (track.seconds * 8_000.0).round() as usize);
        assert!((sample.duration() - track.seconds).abs() < 1.0 / 8_000.0);
        let mut energy = 0.0;
        let mut fingerprint = 0_u64;
        for frame in sample.frames() {
            for value in frame {
                assert!(value.is_finite() && value.abs() <= 0.85);
                energy += (*value as f64).powi(2);
                fingerprint = fingerprint.rotate_left(7) ^ value.to_bits() as u64;
            }
        }
        assert!(
            (energy / (sample.len() * 2) as f64).sqrt() > 0.02,
            "{} is silent",
            track.name
        );
        assert!(
            fingerprints.insert(fingerprint),
            "tracks must contain different audio"
        );
        if index > 0 && index < 10 {
            assert_eq!(sample.frames()[0], [0.0; 2]);
            assert!(
                sample
                    .frames()
                    .last()
                    .unwrap()
                    .iter()
                    .all(|v| v.abs() < 1e-6)
            );
        }
    }
}

#[test]
fn expanded_bank_preserves_four_bar_timing_and_loop_seams() {
    assert_eq!(samples::TRACKS.len(), 30);
    for (index, track) in samples::TRACKS.iter().enumerate().skip(10) {
        if let Some(bpm) = track.bpm {
            assert!((track.seconds * bpm as f64 / 60.0 - 16.0).abs() < 1e-10);
        }
        let sample = samples::generate(index, 48_000);
        for channel in 0..2 {
            let frames = sample.frames();
            let mean: f64 =
                frames.iter().map(|f| f[channel] as f64).sum::<f64>() / frames.len() as f64;
            assert!(mean.abs() < 1e-6, "{} has DC offset", track.id);
            let seam = (frames[0][channel] - frames.last().unwrap()[channel]).abs();
            let max_delta = frames
                .windows(2)
                .map(|f| (f[1][channel] - f[0][channel]).abs())
                .fold(0.0_f32, f32::max);
            assert!(
                seam <= max_delta * 1.05 + 1e-6,
                "{} clicks at the seam: {seam} > {max_delta}",
                track.id
            );
        }
    }
}

#[test]
fn generated_tracks_are_reproducible_and_float_wav_export_is_lossless() {
    for index in 0..samples::TRACKS.len() {
        let sample = samples::generate(index, 8_000);
        let repeat = samples::generate(index, 8_000);
        assert_eq!(sample.frames(), repeat.frames());
        let bytes = wav::bytes(sample.sample_rate, sample.frames()).unwrap();
        let decoded = wav::decode(Cursor::new(bytes), sample.name.clone()).unwrap();
        assert_eq!(decoded.sample_rate, sample.sample_rate);
        assert_eq!(decoded.frames(), sample.frames());
    }
}

#[test]
fn native_bank_reuses_samples_and_glass_stays_compatible() {
    let sample = samples::get("glass-sequence").unwrap();
    let same = samples::get("glass-sequence").unwrap();
    assert!(Arc::ptr_eq(&sample, &same));
    assert_eq!(sample.sample_rate, 48_000);
    let original = sad_pet::dsp::SampleBuffer::demo(48_000);
    assert_eq!(sample.frames(), original.frames());
    assert!(samples::get("missing-track").is_none());
}

#[test]
fn harvest_cover_renders_the_short_melody_at_both_audio_rates() {
    let index = samples::TRACKS
        .iter()
        .position(|track| track.id == "harvest-dawn-cover")
        .unwrap();
    assert!(samples::get("harvest-dawn-excerpt").is_none());
    for rate in [44_100, 48_000] {
        let sample = samples::generate(index, rate);
        assert_eq!(sample.duration(), 8.0);
        assert_eq!(sample.frames()[0], [0.0; 2]);
        assert_eq!(*sample.frames().last().unwrap(), [0.0; 2]);
        for (center, note) in [
            (0.6, 74),
            (2.2, 69),
            (4.5, 74),
            (5.2, 76),
            (6.3, 77),
            (7.7, 81),
        ] {
            let correlation = |note: f64| {
                let frequency = 440.0 * 2.0_f64.powf((note - 69.0) / 12.0);
                let start = ((center - 0.08) * rate as f64) as usize;
                let length = (0.16 * rate as f64) as usize;
                let (mut real, mut imag) = (0.0, 0.0);
                for i in 0..length {
                    let phase = std::f64::consts::TAU * frequency * i as f64 / rate as f64;
                    let window =
                        0.5 - 0.5 * (std::f64::consts::TAU * i as f64 / length as f64).cos();
                    let value = sample.frames()[start + i][0] as f64 * window;
                    real += value * phase.cos();
                    imag += value * phase.sin();
                }
                real * real + imag * imag
            };
            let expected = correlation(note as f64);
            let neighbor = correlation(note as f64 - 1.0).max(correlation(note as f64 + 1.0));
            assert!(
                expected > neighbor * 10.0,
                "cover melody at {center}s / {rate}Hz"
            );
        }
    }
}
