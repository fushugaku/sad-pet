//! Bounded periodic WAV interchange. An audio sample is never silently
//! interpreted as a table: its period comes from metadata or the user's choice.
use super::{FRAME_SIZE, MAX_FRAMES, assets::Asset};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

pub const MAX_BYTES: usize = 16 * 1024 * 1024;
pub const PERIODS: [usize; 6] = [128, 256, 512, 1024, 2048, 4096];

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Channel {
    Left,
    Right,
    Mix,
}
#[derive(Clone, Debug, Serialize)]
pub struct Info {
    pub channels: u16,
    pub sample_rate: u32,
    pub bits_per_sample: u16,
    pub samples_per_channel: usize,
    pub frame_size: Option<usize>,
    pub frame_count: Option<usize>,
}

/// Read every RIFF boundary before handing the stream to the PCM decoder.
/// `clm ` supports the documented Serum-style `<!>period ...` convention;
/// other metadata never guesses a period from an arbitrary sample's length.
pub fn inspect(bytes: &[u8]) -> Result<Info, String> {
    if bytes.len() > MAX_BYTES {
        return Err("The wavetable WAV exceeds 16 MiB".into());
    }
    if bytes.len() < 12 || &bytes[..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
        return Err("Choose a PCM or float WAV file".into());
    }
    let end = 8usize
        .checked_add(u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize)
        .filter(|&end| end >= 12 && end <= bytes.len())
        .ok_or("The WAV RIFF length is incomplete")?;
    let mut offset = 12;
    let (mut fmt, mut data, mut period) = (false, false, None);
    while offset < end {
        let header = bytes
            .get(offset..offset + 8)
            .filter(|_| offset + 8 <= end)
            .ok_or("Incomplete WAV chunk header")?;
        let length = u32::from_le_bytes(header[4..8].try_into().unwrap()) as usize;
        let start = offset + 8;
        let stop = start
            .checked_add(length)
            .filter(|&n| n <= end)
            .ok_or("A WAV chunk extends beyond the RIFF data")?;
        let chunk = &bytes[start..stop];
        match &header[..4] {
            b"fmt " => {
                if fmt {
                    return Err("The WAV has duplicate format chunks".into());
                }
                fmt = true;
            }
            b"data" => {
                if data {
                    return Err("The WAV has duplicate audio chunks".into());
                }
                data = true;
            }
            b"clm " => {
                if length > 4096 {
                    return Err("The wavetable metadata is too large".into());
                }
                if let Ok(text) = std::str::from_utf8(chunk)
                    && let Some(text) = text.trim_end_matches('\0').strip_prefix("<!>")
                {
                    if period.is_some() {
                        return Err("The WAV has conflicting period metadata".into());
                    }
                    let digits: String = text.chars().take_while(char::is_ascii_digit).collect();
                    let n = digits
                        .parse::<usize>()
                        .map_err(|_| "Invalid wavetable period metadata")?;
                    if !PERIODS.contains(&n) {
                        return Err(
                            "Unsupported wavetable period metadata; use 128–4096 samples".into(),
                        );
                    }
                    period = Some(n);
                }
            }
            _ => {}
        }
        offset = stop
            .checked_add(length & 1)
            .filter(|&n| n <= end)
            .ok_or("The WAV chunk padding is incomplete")?;
    }
    if !fmt || !data {
        return Err("The WAV is missing its format or audio chunk".into());
    }
    let reader = hound::WavReader::new(Cursor::new(bytes)).map_err(|e| format!("WAV: {e}"))?;
    let spec = reader.spec();
    if !(1..=2).contains(&spec.channels) {
        return Err("Use a mono or stereo WAV".into());
    }
    if spec.sample_rate == 0 {
        return Err("The WAV sample rate is zero".into());
    }
    if !matches!(
        (spec.sample_format, spec.bits_per_sample),
        (hound::SampleFormat::Int, 16 | 24 | 32) | (hound::SampleFormat::Float, 32)
    ) {
        return Err("Use PCM 16/24/32-bit or float32 WAV".into());
    }
    let samples = reader.duration() as usize;
    if samples == 0 {
        return Err("The wavetable WAV is empty".into());
    }
    let frames = period.map(|p| dimensions(samples, p)).transpose()?;
    Ok(Info {
        channels: spec.channels,
        sample_rate: spec.sample_rate,
        bits_per_sample: spec.bits_per_sample,
        samples_per_channel: samples,
        frame_size: period,
        frame_count: frames,
    })
}
fn dimensions(samples: usize, period: usize) -> Result<usize, String> {
    if !PERIODS.contains(&period) {
        return Err("Choose a supported wavetable frame size".into());
    }
    if !samples.is_multiple_of(period) {
        return Err("The WAV ends with an incomplete wavetable frame".into());
    }
    let frames = samples / period;
    if !(1..=MAX_FRAMES).contains(&frames) {
        return Err("Use a wavetable with 1–256 frames; no frames were discarded".into());
    }
    Ok(frames)
}
pub fn decode(
    bytes: &[u8],
    name: String,
    frame_size: Option<usize>,
    channel: Option<Channel>,
) -> Result<Asset, String> {
    let info = inspect(bytes)?;
    let period = frame_size
        .or(info.frame_size)
        .ok_or("Choose the WAV's frame size before importing")?;
    dimensions(info.samples_per_channel, period)?;
    if info.channels == 2 && channel.is_none() {
        return Err("Choose Left, Right or Mix for the stereo WAV".into());
    }
    let mut reader = hound::WavReader::new(Cursor::new(bytes)).map_err(|e| format!("WAV: {e}"))?;
    let samples: Vec<f32> = match reader.spec().sample_format {
        hound::SampleFormat::Float => reader
            .samples::<f32>()
            .map(|v| v.map_err(|e| e.to_string()))
            .collect::<Result<_, _>>()?,
        hound::SampleFormat::Int => {
            let gain = 2.0_f64.powi(-(info.bits_per_sample as i32 - 1));
            reader
                .samples::<i32>()
                .map(|v| {
                    v.map(|x| (x as f64 * gain) as f32)
                        .map_err(|e| e.to_string())
                })
                .collect::<Result<_, _>>()?
        }
    };
    if samples.len() != info.samples_per_channel * info.channels as usize
        || samples.iter().any(|x| !x.is_finite())
    {
        return Err("The WAV has incomplete or non-finite audio samples".into());
    }
    let mono = if info.channels == 1 {
        samples
    } else {
        samples
            .chunks_exact(2)
            .map(|p| match channel.unwrap() {
                Channel::Left => p[0],
                Channel::Right => p[1],
                Channel::Mix => (p[0] as f64 * 0.5 + p[1] as f64 * 0.5) as f32,
            })
            .collect()
    };
    Asset::from_cycles(name, period, &mono)
}

/// Canonical mono float32 periods with an explicit period chunk. Sample rate
/// is only a WAV container field; playback pitch is determined by oscillator Hz.
pub fn encode(asset: &Asset) -> Result<Vec<u8>, String> {
    asset.validate()?;
    let mut cursor = Cursor::new(Vec::new());
    let mut writer = hound::WavWriter::new(
        &mut cursor,
        hound::WavSpec {
            channels: 1,
            sample_rate: 48_000,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        },
    )
    .map_err(|e| e.to_string())?;
    for &sample in asset.samples.iter() {
        writer.write_sample(sample).map_err(|e| e.to_string())?;
    }
    writer.finalize().map_err(|e| e.to_string())?;
    let mut bytes = cursor.into_inner();
    let metadata = format!("<!>{FRAME_SIZE} 1 0 0 0 0 0 SAD PET");
    bytes.extend_from_slice(b"clm ");
    bytes.extend_from_slice(&(metadata.len() as u32).to_le_bytes());
    bytes.extend_from_slice(metadata.as_bytes());
    if metadata.len() & 1 != 0 {
        bytes.push(0);
    }
    let length = (bytes.len() - 8) as u32;
    bytes[4..8].copy_from_slice(&length.to_le_bytes());
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn wav(bits: u16, float: bool, channels: u16, frames: usize) -> Vec<u8> {
        let mut c = Cursor::new(Vec::new());
        let mut w = hound::WavWriter::new(
            &mut c,
            hound::WavSpec {
                channels,
                sample_rate: 44_100,
                bits_per_sample: bits,
                sample_format: if float {
                    hound::SampleFormat::Float
                } else {
                    hound::SampleFormat::Int
                },
            },
        )
        .unwrap();
        for i in 0..frames * 128 {
            for channel in 0..channels {
                let value = (i as f64 * std::f64::consts::TAU / 128.0).sin()
                    * if channel == 0 { 0.7 } else { 0.2 };
                if float {
                    w.write_sample(value as f32).unwrap();
                } else {
                    w.write_sample((value * (1u64 << (bits - 1)) as f64) as i32)
                        .unwrap();
                }
            }
        }
        w.finalize().unwrap();
        c.into_inner()
    }
    #[test]
    fn formats_stereo_choice_and_portable_export() {
        for (bits, float) in [(16, false), (24, false), (32, false), (32, true)] {
            let bytes = wav(bits, float, 2, 2);
            let info = inspect(&bytes).unwrap();
            assert_eq!(info.samples_per_channel, 256);
            assert!(decode(&bytes, "A".into(), Some(128), None).is_err());
            let left = decode(&bytes, "Left".into(), Some(128), Some(Channel::Left)).unwrap();
            let right = decode(&bytes, "Right".into(), Some(128), Some(Channel::Right)).unwrap();
            let mix = decode(&bytes, "Mix".into(), Some(128), Some(Channel::Mix)).unwrap();
            let peak = |a: &Asset| a.samples.iter().map(|v| v.abs()).fold(0.0_f32, f32::max);
            assert!((peak(&left) - 0.7).abs() < 5e-5);
            assert!((peak(&right) - 0.2).abs() < 5e-5);
            assert!((peak(&mix) - 0.45).abs() < 5e-5);
            let exported = encode(&left).unwrap();
            assert_eq!(inspect(&exported).unwrap().frame_size, Some(FRAME_SIZE));
            let recalled = decode(&exported, "Recalled".into(), None, None).unwrap();
            assert!(
                recalled
                    .samples
                    .iter()
                    .zip(left.samples.iter())
                    .all(|(a, b)| (a - b).abs() < 1e-6)
            );
        }
    }
    #[test]
    fn reject_ambiguous_corrupt_partial_oversized_and_nonfinite() {
        let bytes = wav(32, true, 1, 1);
        assert!(decode(&bytes, "A".into(), None, None).is_err());
        assert!(decode(&bytes, "A".into(), Some(256), None).is_err());
        assert!(decode(&wav(16, false, 1, 257), "A".into(), Some(128), None).is_err());
        assert!(inspect(&vec![0; MAX_BYTES + 1]).is_err());
        for cut in [0, 4, 11, 20, bytes.len() - 1] {
            assert!(inspect(&bytes[..cut]).is_err());
        }
        let mut invalid = bytes.clone();
        invalid[16..20].copy_from_slice(&u32::MAX.to_le_bytes());
        assert!(inspect(&invalid).is_err());
        let mut nan = bytes.clone();
        let at = nan.windows(4).position(|p| p == b"data").unwrap() + 8;
        nan[at..at + 4].copy_from_slice(&f32::NAN.to_le_bytes());
        assert!(decode(&nan, "A".into(), Some(128), None).is_err());
    }
}
