use crate::{Stereo, dsp::SampleBuffer};
use std::{
    io::{Cursor, Read, Seek, Write},
    path::Path,
};

pub fn decode<R: Read + Seek>(reader: R, name: String) -> Result<SampleBuffer, String> {
    let (rate, frames) = frames(reader)?;
    SampleBuffer::new(name, rate, frames)
}
/// The rate and stereo frames of a mono or stereo WAV, without preparing
/// the antialiasing levels a playable sample needs.
pub fn frames<R: Read + Seek>(reader: R) -> Result<(u32, Vec<Stereo>), String> {
    let mut wav = hound::WavReader::new(reader).map_err(|e| format!("WAV: {e}"))?;
    let spec = wav.spec();
    if !(1..=2).contains(&spec.channels) {
        return Err("Use a mono or stereo WAV file".into());
    }
    if wav.duration() > spec.sample_rate.saturating_mul(30 * 60) {
        return Err("The WAV file exceeds 30 minutes".into());
    }
    let samples: Vec<f32> = match spec.sample_format {
        hound::SampleFormat::Float => wav
            .samples::<f32>()
            .map(|s| s.map_err(|e| e.to_string()))
            .collect::<Result<_, _>>()?,
        hound::SampleFormat::Int => {
            if !(1..=32).contains(&spec.bits_per_sample) {
                return Err("Unsupported WAV bit depth".into());
            }
            let scale = 2.0_f64.powi(spec.bits_per_sample as i32 - 1) as f32;
            wav.samples::<i32>()
                .map(|s| s.map(|x| x as f32 / scale).map_err(|e| e.to_string()))
                .collect::<Result<_, _>>()?
        }
    };
    if !samples.len().is_multiple_of(spec.channels as usize) {
        return Err("Incomplete WAV frame".into());
    }
    let frames = if spec.channels == 1 {
        samples.into_iter().map(|s| [s, s]).collect()
    } else {
        samples.as_chunks::<2>().0.to_vec()
    };
    Ok((spec.sample_rate, frames))
}
pub fn read(path: &Path) -> Result<SampleBuffer, String> {
    decode(
        std::fs::File::open(path).map_err(|e| e.to_string())?,
        path.file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned(),
    )
}
pub fn encode<W: Write + Seek>(
    writer: W,
    sample_rate: u32,
    frames: &[Stereo],
) -> Result<(), String> {
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut wav = hound::WavWriter::new(writer, spec).map_err(|e| e.to_string())?;
    for frame in frames {
        for &sample in frame {
            wav.write_sample(sample).map_err(|e| e.to_string())?;
        }
    }
    wav.finalize().map_err(|e| e.to_string())
}
pub fn bytes(sample_rate: u32, frames: &[Stereo]) -> Result<Vec<u8>, String> {
    // Float WAV payload plus the RIFF/fmt/fact headers. Reserve once so large
    // browser exports do not transiently retain both sides of a growing buffer.
    let mut buffer = Cursor::new(Vec::with_capacity(frames.len() * 8 + 80));
    encode(&mut buffer, sample_rate, frames)?;
    Ok(buffer.into_inner())
}
