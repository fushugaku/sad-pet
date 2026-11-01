//! Control-side audio import. Keep WAV metadata intact; decode MP3 once to
//! float WAV before storing it or handing it to the instrument's WAV readers.
use std::{borrow::Cow, io::Cursor};
use symphonia::core::{
    audio::SampleBuffer,
    codecs::{CODEC_TYPE_MP3, DecoderOptions},
    errors::Error,
    formats::FormatOptions,
    io::MediaSourceStream,
    probe::Hint,
};

pub const MAX_BYTES: usize = 128 * 1024 * 1024;

pub fn to_wav(bytes: &[u8], max_bytes: usize) -> Result<Cow<'_, [u8]>, String> {
    let max_bytes = max_bytes.min(MAX_BYTES);
    let too_large = || format!("The audio exceeds {} MiB as WAV", max_bytes / (1024 * 1024));
    if bytes.len() > max_bytes {
        return Err(too_large());
    }
    if bytes.len() >= 12 && &bytes[..4] == b"RIFF" && &bytes[8..12] == b"WAVE" {
        return Ok(Cow::Borrowed(bytes));
    }
    let fail = |error| format!("MP3: {error}");
    let source = MediaSourceStream::new(Box::new(Cursor::new(bytes.to_vec())), Default::default());
    let options = FormatOptions {
        enable_gapless: true,
        ..Default::default()
    };
    let mut hint = Hint::new();
    hint.with_extension("mp3");
    let mut format = symphonia::default::get_probe()
        .format(&hint, source, &options, &Default::default())
        .map_err(|_| "Use a WAV or MP3 audio file".to_string())?
        .format;
    let track = format.default_track().ok_or("The MP3 has no audio track")?;
    if track.codec_params.codec != CODEC_TYPE_MP3 {
        return Err("Use a WAV or MP3 audio file".into());
    }
    let track_id = track.id;
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &DecoderOptions::default())
        .map_err(fail)?;
    let mut output = Cursor::new(Vec::new());
    let mut sink = Some(&mut output);
    let mut writer = None;
    let mut spec = None;
    let mut samples = None;
    let mut frames = 0usize;
    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(Error::IoError(error)) if error.kind() == std::io::ErrorKind::UnexpectedEof => {
                break;
            }
            Err(error) => return Err(fail(error)),
        };
        if packet.track_id() != track_id {
            continue;
        }
        let audio = decoder.decode(&packet).map_err(fail)?;
        let current = *audio.spec();
        let channels = current.channels.count();
        if !(1..=2).contains(&channels) || current.rate == 0 {
            return Err("Use a mono or stereo MP3 file".into());
        }
        if spec.is_some_and(|spec| spec != current) {
            return Err("The MP3 changes its sample rate or channel count".into());
        }
        // Bound the expanded PCM, rather than just the compressed input. The
        // float WAV header written by hound fits within these 80 bytes.
        frames += audio.frames();
        if frames > current.rate as usize * 30 * 60 {
            return Err("The MP3 file exceeds 30 minutes".into());
        }
        if frames > max_bytes.saturating_sub(80) / (channels * 4) {
            return Err(too_large());
        }
        if writer.is_none() {
            spec = Some(current);
            writer = Some(
                hound::WavWriter::new(
                    sink.take().unwrap(),
                    hound::WavSpec {
                        channels: channels as u16,
                        sample_rate: current.rate,
                        bits_per_sample: 32,
                        sample_format: hound::SampleFormat::Float,
                    },
                )
                .map_err(|e| e.to_string())?,
            );
        }
        let samples = samples
            .get_or_insert_with(|| SampleBuffer::<f32>::new(audio.capacity() as u64, current));
        samples.copy_interleaved_ref(audio);
        for &sample in samples.samples() {
            if !sample.is_finite() {
                return Err("The MP3 contains invalid audio samples".into());
            }
            writer
                .as_mut()
                .unwrap()
                .write_sample(sample)
                .map_err(|e| e.to_string())?;
        }
    }
    if frames == 0 {
        return Err("The MP3 file is empty or damaged".into());
    }
    writer.unwrap().finalize().map_err(|e| e.to_string())?;
    Ok(Cow::Owned(output.into_inner()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const MONO: &[u8] = include_bytes!("../tests/fixtures/mono-44100.mp3");
    const STEREO: &[u8] = include_bytes!("../tests/fixtures/stereo-32000.mp3");

    #[test]
    fn mp3_import_preserves_rate_channels_and_gapless_length() {
        for (source, rate, channels, frames) in [(MONO, 44100, 1, 16384), (STEREO, 32000, 2, 8000)]
        {
            let bytes = to_wav(source, MAX_BYTES).unwrap();
            let mut wav = hound::WavReader::new(Cursor::new(&*bytes)).unwrap();
            assert_eq!(wav.spec().sample_rate, rate);
            assert_eq!(wav.spec().channels, channels);
            assert_eq!(wav.spec().sample_format, hound::SampleFormat::Float);
            assert_eq!(wav.spec().bits_per_sample, 32);
            assert_eq!(
                wav.duration(),
                frames,
                "encoder delay/padding reached the instrument"
            );
            let samples: Vec<f32> = wav.samples().map(Result::unwrap).collect();
            assert!(samples.iter().all(|sample| sample.is_finite()));
            assert!(samples.iter().any(|sample| sample.abs() > 0.05));
            if channels == 2 {
                assert!(samples.chunks_exact(2).any(|f| (f[0] - f[1]).abs() > 0.05));
            }
            // The instrument's existing WAV reader accepts the converted audio.
            let (sample_rate, pcm) = crate::wav::frames(Cursor::new(&*bytes)).unwrap();
            assert_eq!(sample_rate, rate);
            assert_eq!(pcm.len(), frames as usize);
        }
    }

    #[test]
    fn mp3_expansion_is_bounded_and_bad_input_fails() {
        assert!(MONO.len() < 8192);
        assert!(to_wav(MONO, 8192).unwrap_err().contains("as WAV"));
        assert!(to_wav(b"ID3broken MP3", MAX_BYTES).is_err());
        assert!(to_wav(&[], MAX_BYTES).is_err());
    }

    #[test]
    fn wav_import_keeps_period_metadata_and_pcm_unchanged() {
        let asset = crate::wavetable::assets::Asset::from_cycles(
            "Original".into(),
            128,
            &(0..128)
                .map(|i| (i as f32 * std::f32::consts::TAU / 128.0).sin())
                .collect::<Vec<_>>(),
        )
        .unwrap();
        let original = crate::wavetable::import::encode(&asset).unwrap();
        let wav = to_wav(&original, MAX_BYTES).unwrap();
        assert!(matches!(wav, Cow::Borrowed(_)));
        assert_eq!(&*wav, &original);
        assert_eq!(
            crate::wavetable::import::inspect(&wav).unwrap().frame_size,
            Some(2048)
        );
    }

    #[test]
    fn decoded_mp3_feeds_periodic_import_and_wave_maker() {
        let wav = to_wav(MONO, MAX_BYTES).unwrap();
        let asset =
            crate::wavetable::import::decode(&wav, "MP3 table".into(), Some(2048), None).unwrap();
        assert_eq!(asset.frame_count, 8);
        let wav = to_wav(STEREO, MAX_BYTES).unwrap();
        let settings = crate::wavetable::maker::Settings {
            channel: Some(crate::wavetable::import::Channel::Mix),
            ..Default::default()
        };
        let asset = crate::wavetable::maker::make(&wav, "MP3 texture".into(), &settings).unwrap();
        assert_eq!(asset.frame_count, 64);
    }
}
