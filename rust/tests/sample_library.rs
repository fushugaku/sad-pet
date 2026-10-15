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
        if index > 0 {
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
