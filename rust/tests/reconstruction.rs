use sad_pet::{
    dsp::{Engine, SampleBuffer},
    firmware::{Image, ImageError},
    midi::{Message, Parser},
    preset::{FilterType, Params, Preset, ReverbType, SaturatorType},
    tables, wav,
};
use std::{io::Cursor, path::Path, sync::Arc};

fn sine(sr: u32, hz: f64) -> Arc<SampleBuffer> {
    let frames = (0..sr)
        .map(|i| {
            let x = (i as f64 / sr as f64 * hz * std::f64::consts::TAU).sin() as f32 * 0.25;
            [x, x]
        })
        .collect();
    Arc::new(SampleBuffer::new("test tone".into(), sr, frames).unwrap())
}
fn rms(values: impl Iterator<Item = f32>) -> f64 {
    let (sum, n) = values.fold((0.0_f64, 0_usize), |(s, n), x| {
        (s + (x as f64).powi(2), n + 1)
    });
    (sum / n.max(1) as f64).sqrt()
}

#[test]
#[ignore = "requires the seven original .fw fixtures in the workspace root"]
fn supplied_images_and_extracted_tables_match() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    for v in [27, 29, 42, 48, 57, 59, 63] {
        let bytes = std::fs::read(root.join(format!("evilpet1p{v}.fw"))).unwrap();
        let image = Image::parse(&bytes).unwrap();
        assert_eq!(image.flags, 0x80000000);
        assert_eq!(image.entry, 0x2ffe0000);
    }
    let bytes = std::fs::read(root.join("evilpet1p63.fw")).unwrap();
    let image = Image::parse(&bytes).unwrap();
    assert_eq!(image.payload.len(), 789296);
    assert_eq!(image.checksum, 0x0562df49);
    for (file, offset, n) in [
        ("pitch", 0xa9408, 256),
        ("fine_pitch", 0xa9000, 257),
        ("grain_window", 0xaeca0, 4097),
        ("sine", 0xb7db0, 256),
    ] {
        assert_eq!(
            std::fs::read(root.join(format!("rust/assets/{file}.f32"))).unwrap(),
            image.payload[offset..offset + n * 4]
        );
    }
    let mut damaged = bytes.clone();
    *damaged.last_mut().unwrap() ^= 1;
    assert!(matches!(Image::parse(&damaged), Err(ImageError::Checksum)));
    damaged = bytes.clone();
    damaged[0x64] |= 2;
    assert!(matches!(Image::parse(&damaged), Err(ImageError::Encrypted)));
    assert!(matches!(
        Image::parse(&bytes[..100]),
        Err(ImageError::Truncated)
    ));
    assert!(matches!(
        Image::parse(&bytes[..bytes.len() - 1]),
        Err(ImageError::Length)
    ));
}

#[test]
fn recovered_pitch_and_grain_curves_have_expected_musical_values() {
    assert_eq!(tables::pitch_ratio(0.0), 1.0);
    assert_eq!(tables::pitch_ratio(12.0), 2.0);
    assert_eq!(tables::pitch_ratio(-12.0), 0.5);
    for i in -640..640 {
        let semis = i as f32 * 0.1;
        let expected = 2.0_f64.powf(semis as f64 / 12.0);
        assert!((tables::pitch_ratio(semis) / expected - 1.0).abs() < 0.0003);
    }
    for i in 0..=100 {
        let phase = i as f32 * 0.02;
        assert!(
            (tables::grain_window(phase, 1.0) - (0.5 - 0.5 * (std::f32::consts::PI * phase).cos()))
                .abs()
                < 0.000001
        );
        assert!(
            (tables::grain_window(phase, 0.4) - tables::grain_window(2.0 - phase, 0.4)).abs()
                < 0.000001
        );
    }
    assert_eq!(tables::grain_window(0.0, 1.0), 0.0);
    assert_eq!(tables::grain_window(1.0, 1.0), 1.0);
    assert_eq!(tables::grain_window(2.0, 1.0), 0.0);
    assert_eq!(tables::grain_window(0.25, 0.0), 0.25);
    assert!((tables::grain_duration(0.5) - 0.01).abs() < 1e-6);
    assert!((tables::grain_duration(0.0) - 2.01).abs() < 1e-6);
    assert!((tables::grain_duration(0.17) - tables::grain_duration(0.83)).abs() < 1e-6);
    assert_eq!(tables::detune_range(0.5), 0.0);
    assert_eq!(tables::detune_range(0.0), -12.0);
    assert_eq!(tables::detune_range(1.0), 12.0);
    assert_eq!(tables::pan(0.0), [1.0, 0.0]);
    assert_eq!(tables::pan(0.5), [1.0, 1.0]);
    assert_eq!(tables::pan(1.0), [0.0, 1.0]);
}

#[test]
fn recovered_random_sequence_is_stable() {
    let mut rng = tables::Random::default();
    for expected in [
        0x027c6003, 0xcb07bbf3, 0xf98befee, 0x1cd777e3, 0xa4e29590, 0x661e4b6d, 0x093b9e0e,
        0xb7e9851d,
    ] {
        assert_eq!(rng.next_u32(), expected);
    }
    for _ in 0..10000 {
        assert!((0.0..1.0).contains(&rng.unit()));
    }
    for x in [1.0_f32, 2.0, 5.0, 64.0, 128.0] {
        assert!((tables::inverse_sqrt(x) * x.sqrt() - 1.0).abs() < 0.002);
    }
}

#[test]
fn interpolation_preserves_long_recording_fractional_positions_and_wraps() {
    let data = (0..16)
        .map(|x| [x as f32 / 16.0, x as f32 / 16.0])
        .collect();
    let sample = SampleBuffer::new("short periodic".into(), 48000, data).unwrap();
    let expected = sample.read(3.125, 1.0, false);
    assert_eq!(
        sample.read((1_u64 << 26) as f64 + 3.125, 1.0, false),
        expected
    );
    assert_eq!(sample.read(-12.875, 1.0, false), expected);
    assert_eq!(sample.read(19.125, -1.0, false), expected);
}

#[test]
fn antialiasing_rejects_high_frequencies_in_both_directions() {
    let sample = sine(48000, 18000.0);
    for rate in [3.0_f64, -3.0, 7.0, -7.0] {
        let raw = rms((0..4096).map(|i| sample.read(i as f64 * rate, rate, false)[0]));
        let filtered = rms((0..4096).map(|i| sample.read(i as f64 * rate, rate, true)[0]));
        assert!(raw > 0.1);
        assert!(filtered < raw * 0.01, "rate {rate}: {filtered}/{raw}");
    }
}

#[test]
fn midi_running_status_realtime_and_sysex_do_not_corrupt_notes() {
    let mut p = Parser::default();
    let events = [
        0x90, 60, 0xf8, 100, 64, 90, 60, 0, 0xf0, 1, 2, 0xf8, 3, 0xf7, 64, 0, 0x81, 70, 30,
    ]
    .into_iter()
    .filter_map(|b| p.feed(b))
    .collect::<Vec<_>>();
    assert_eq!(
        events,
        vec![
            Message::Clock,
            Message::NoteOn {
                channel: 0,
                note: 60,
                velocity: 100
            },
            Message::NoteOn {
                channel: 0,
                note: 64,
                velocity: 90
            },
            Message::NoteOff {
                channel: 0,
                note: 60,
                velocity: 0
            },
            Message::Clock,
            Message::NoteOff {
                channel: 1,
                note: 70,
                velocity: 30
            }
        ]
    );
    let mut p = Parser::default();
    let messages = [0xe2, 0, 64, 127, 127]
        .into_iter()
        .filter_map(|b| p.feed(b))
        .collect::<Vec<_>>();
    assert_eq!(
        messages,
        vec![
            Message::PitchBend {
                channel: 2,
                value: 8192
            },
            Message::PitchBend {
                channel: 2,
                value: 16383
            }
        ]
    );
}

#[test]
fn midi_chords_sustain_and_channel_note_offs_are_independent() {
    let mut e = Engine::new(48000, sine(48000, 220.0), Preset::default());
    for (channel, note) in [(0, 60), (0, 64), (1, 60)] {
        e.midi(Message::NoteOn {
            channel,
            note,
            velocity: 100,
        });
    }
    assert_eq!(e.held_notes().len(), 3);
    e.midi(Message::Control {
        channel: 0,
        controller: 64,
        value: 127,
    });
    e.midi(Message::NoteOff {
        channel: 0,
        note: 60,
        velocity: 0,
    });
    assert_eq!(e.held_notes().len(), 3);
    e.midi(Message::Control {
        channel: 0,
        controller: 64,
        value: 0,
    });
    assert_eq!(e.held_notes().len(), 2);
    e.midi(Message::NoteOn {
        channel: 1,
        note: 60,
        velocity: 0,
    });
    assert_eq!(e.held_notes(), vec![64]);
    e.midi(Message::Control {
        channel: 0,
        controller: 123,
        value: 0,
    });
    assert!(e.held_notes().is_empty());
}

#[test]
fn engine_is_deterministic_across_audio_block_sizes() {
    let sample = sine(44100, 440.0);
    let preset = Preset::builtin(4);
    let mut a = Engine::new(48000, sample.clone(), preset.clone());
    let mut b = Engine::new(48000, sample, preset);
    a.set_playing(true);
    b.set_playing(true);
    let mut expected = vec![[0.0; 2]; 4800];
    a.process(&mut expected);
    let mut actual = vec![[0.0; 2]; 4800];
    for block in actual.chunks_mut(73) {
        b.process(block);
    }
    assert_eq!(actual, expected);
    assert!(actual.iter().flatten().any(|x| x.abs() > 0.01));
}

#[test]
fn zero_density_is_silent_and_bypass_preserves_the_source_gain() {
    let sample = sine(48000, 400.0);
    let mut preset = Preset::default();
    preset.params.grains = 0.0;
    preset.params.min_grains = 0.0;
    preset.params.reverb = 0.0;
    preset.color_chain[1].mix = 0.0;
    preset.params.master_volume = 1.0;
    let mut e = Engine::new(48000, sample.clone(), preset.clone());
    e.set_playing(true);
    for _ in 0..12000 {
        assert_eq!(e.process_frame(), [0.0; 2]);
    }
    assert_eq!(e.active_grains(), 0);
    let mut e = Engine::new(48000, sample.clone(), preset);
    e.bypass = true;
    e.set_playing(true);
    for i in 0..16000 {
        let out = e.process_frame();
        if i > 14000 {
            let expected = sample.read(i as f64, 1.0, true);
            assert!((out[0] - expected[0]).abs() < 0.00001);
        }
    }
}

#[test]
fn all_color_modes_remain_finite_at_extreme_settings() {
    let sample = sine(8000, 440.0);
    for color in [
        SaturatorType::Distortion,
        SaturatorType::Reducer,
        SaturatorType::Crusher,
        SaturatorType::Flanger,
        SaturatorType::Chorus,
        SaturatorType::Binaural,
    ] {
        for reverb in [ReverbType::PlateL, ReverbType::Reverse] {
            let mut preset = Preset::default();
            preset.params.saturator_type = color;
            preset.params.reverb_type = reverb;
            preset.params.vcf_type = FilterType::Comb;
            preset.params.saturation = 1.0;
            preset.color_chain = vec![sad_pet::color::ColorSlot::legacy(color, 1.0)];
            preset.params.reverb = 1.0;
            preset.params.shimmer = 1.0;
            preset.params.diffuse = 1.0;
            preset.params.reverb_decay = 1.0;
            preset.params.filter_resonance = 1.0;
            preset.params.grains = 1.0;
            preset.params.grain_size = 0.99;
            preset
                .color_chain
                .push(sad_pet::color::ColorSlot::legacy_reverb(2, &preset.params));
            let mut e = Engine::new(8000, sample.clone(), preset);
            e.set_playing(true);
            for _ in 0..8000 {
                for x in e.process_frame() {
                    assert!(x.is_finite() && x.abs() <= 1.0);
                }
            }
            assert!(e.active_grains() <= 128);
        }
    }
}

#[test]
fn wav_pcm_and_float_roundtrip_handle_both_channels() {
    let frames = vec![[0.25, -0.5], [0.0, 0.75], [-0.2, 0.1]];
    let bytes = wav::bytes(44100, &frames).unwrap();
    let sample = wav::decode(Cursor::new(bytes), "float".into()).unwrap();
    assert_eq!(sample.sample_rate, 44100);
    assert_eq!(sample.len(), 3);
    assert_eq!(sample.read(1.0, 1.0, false), frames[1]);
    let mut buffer = Cursor::new(Vec::new());
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 48000,
        bits_per_sample: 24,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::new(&mut buffer, spec).unwrap();
    writer.write_sample::<i32>(4194304).unwrap();
    writer.write_sample::<i32>(-4194304).unwrap();
    writer.finalize().unwrap();
    buffer.set_position(0);
    let sample = wav::decode(buffer, "pcm24".into()).unwrap();
    assert_eq!(sample.read(0.0, 1.0, false), [0.5; 2]);
    assert_eq!(sample.read(1.0, 1.0, false), [-0.5; 2]);
    assert!(wav::decode(Cursor::new(b"garbage"), "bad".into()).is_err());
}

#[test]
fn malformed_parameter_values_are_repaired_before_processing() {
    let mut params = Params {
        grain_size: f32::NAN,
        pitch: f32::INFINITY,
        reverb: 40.0,
        master_volume: -1.0,
        bpm: f32::NAN,
        ..Params::default()
    };
    params.sanitize();
    assert_eq!(params.grain_size, Params::default().grain_size);
    assert_eq!(params.pitch, 0.5);
    assert_eq!(params.reverb, 1.0);
    assert_eq!(params.master_volume, 0.0);
    assert_eq!(params.bpm, 120.0);
    for i in 0..5 {
        let p = Preset::builtin(i);
        let json = serde_json::to_string(&p).unwrap();
        let restored: Preset = serde_json::from_str(&json).unwrap();
        assert_eq!(serde_json::to_string(&restored).unwrap(), json);
    }
}
