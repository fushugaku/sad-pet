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
        let mut fingerprint = 0xcbf2_9ce4_8422_2325_u64;
        for frame in sample.frames() {
            for value in frame {
                // Bandlimited rate conversion may overshoot an original PCM
                // peak. Keep float headroom instead of clipping the recording.
                assert!(
                    value.is_finite()
                        && value.abs() <= if track.asset.is_some() { 1.5 } else { 0.85 },
                    "{} has invalid resampled amplitude {value}",
                    track.id
                );
                energy += (*value as f64).powi(2);
                // Rotate/XOR alone cancels identical repeated bars to zero.
                fingerprint = (fingerprint ^ value.to_bits() as u64).wrapping_mul(0x0100_0000_01b3);
            }
        }
        assert!(
            (energy / (sample.len() * 2) as f64).sqrt()
                > if track.asset.is_some() { 0.0001 } else { 0.02 },
            "{} is silent",
            track.name
        );
        assert!(
            fingerprints.insert(fingerprint),
            "{} duplicates another track's audio",
            track.id
        );
        if index > 0 && index < 10 && track.id != "harvest-dawn-loop" {
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
    assert_eq!(samples::TRACKS.len(), 414);
    for (index, track) in samples::TRACKS.iter().enumerate().take(158).skip(10) {
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
fn categorized_expansion_adds_128_sources_without_moving_original_entries() {
    let original_ids = [
        "glass-sequence",
        "warm-chords",
        "analog-bass",
        "drum-machine",
        "dusty-keys",
        "vowel-choir",
        "metallic-textures",
        "air-and-rain",
        "matrix-pov",
        "harvest-dawn-loop",
        "lofi-beat",
        "breakbeat",
        "four-on-floor",
        "electro-percussion",
        "dub-chords",
        "acid-line",
        "fm-marimba",
        "kalimba-garden",
        "nylon-harmonics",
        "music-box",
        "tape-piano",
        "string-orbit",
        "shimmer-pad",
        "dark-drone",
        "singing-bowl",
        "resonant-bells",
        "circuit-glitches",
        "space-transmission",
        "ocean-swell",
        "vinyl-dust",
    ];
    assert_eq!(
        samples::TRACKS[..30]
            .iter()
            .map(|t| t.id)
            .collect::<Vec<_>>(),
        original_ids
    );
    let mut categories = std::collections::BTreeMap::new();
    let mut names = HashSet::new();
    for track in samples::TRACKS {
        assert!(
            names.insert(track.name),
            "duplicate display name: {}",
            track.name
        );
        assert!(!track.category.is_empty());
        let json = serde_json::to_value(track).unwrap();
        assert_eq!(json["category"], track.category);
    }
    for track in &samples::TRACKS[30..158] {
        *categories.entry(track.category).or_insert(0) += 1;
    }
    assert_eq!(categories.len(), 8);
    assert!(categories.values().all(|&count| count == 16));
    assert_eq!(
        samples::TRACKS.iter().filter(|t| t.asset.is_some()).count(),
        256
    );
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
fn harvest_loop_preserves_the_opening_and_wraps_its_room_tail() {
    let index = samples::TRACKS
        .iter()
        .position(|track| track.id == "harvest-dawn-loop")
        .unwrap();
    assert!(samples::get("harvest-dawn-excerpt").is_none());
    assert!(samples::get("harvest-dawn-cover").is_none());
    for rate in [44_100, 48_000] {
        let sample = samples::generate(index, rate);
        assert!((sample.duration() - 64.0 * 60.0 / 90.0).abs() < 1.0 / rate as f64);
        // Check the attack of A4, D4, F4, E4 at the written quarter/eighth
        // positions, rather than accepting the old unrelated flute phrase.
        let correlation = |center: f64, note: u8| {
            let frequency = 440.0 * 2.0_f64.powf((note as f64 - 69.0) / 12.0);
            let start = ((center - 0.06) * rate as f64).round() as isize;
            let length = (0.12 * rate as f64) as usize;
            let (mut real, mut imag) = (0.0, 0.0);
            for i in 0..length {
                let phase = std::f64::consts::TAU * frequency * i as f64 / rate as f64;
                let window = 0.5 - 0.5 * (std::f64::consts::TAU * i as f64 / length as f64).cos();
                let pos = (start + i as isize).rem_euclid(sample.len() as isize) as usize;
                let value = sample.frames()[pos][0] as f64 * window;
                real += value * phase.cos();
                imag += value * phase.sin();
            }
            real * real + imag * imag
        };
        for (beat, note) in [(0.0, 69), (1.0, 62), (2.0, 65), (2.5, 64)] {
            let onset = beat * 60.0 / 90.0;
            let expected = correlation(onset + 0.14, note);
            let before = correlation(onset - 0.10, note);
            assert!(
                expected > before * 3.0,
                "opening note {note} at beat {beat} / {rate}Hz: {expected} <= {before}"
            );
        }
        for channel in 0..2 {
            let frames = sample.frames();
            let seam = (frames[0][channel] - frames.last().unwrap()[channel]).abs();
            let neighborhood = frames[..2048]
                .windows(2)
                .chain(frames[frames.len() - 2048..].windows(2))
                .map(|w| (w[1][channel] - w[0][channel]).abs())
                .fold(0.0_f32, f32::max);
            assert!(
                seam <= neighborhood * 1.05 + 1e-6,
                "loop seam: {seam} / {neighborhood}"
            );
            let tail = frames[frames.len() - 2048..]
                .iter()
                .map(|f| (f[channel] as f64).powi(2))
                .sum::<f64>()
                / 2048.0;
            assert!(
                tail > 1e-5,
                "retain the room tail instead of fading to silence"
            );
        }
    }
}
