//! Minimal RIFF WAVE reading (PCM 8/16/24/32-bit and 32/64-bit float, any
//! channel count, folded to mono) and 16-bit/float writing. Voicebank
//! recordings come from many tools, so unknown chunks are skipped.

pub struct Audio {
    pub rate: u32,
    pub samples: Vec<f32>,
}

fn u16_at(b: &[u8], i: usize) -> Option<u16> {
    Some(u16::from_le_bytes(b.get(i..i + 2)?.try_into().ok()?))
}
fn u32_at(b: &[u8], i: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(i..i + 4)?.try_into().ok()?))
}

pub fn read(bytes: &[u8]) -> Result<Audio, String> {
    if bytes.get(0..4) != Some(b"RIFF") || bytes.get(8..12) != Some(b"WAVE") {
        return Err("not a WAV file".into());
    }
    let mut at = 12;
    let mut format: Option<(u16, u16, u32, u16)> = None;
    let mut data: Option<&[u8]> = None;
    while at + 8 <= bytes.len() {
        let id = &bytes[at..at + 4];
        let size = u32_at(bytes, at + 4).ok_or("truncated chunk")? as usize;
        let body = &bytes[at + 8..(at + 8 + size).min(bytes.len())];
        match id {
            b"fmt " => {
                let mut tag = u16_at(body, 0).ok_or("short fmt")?;
                let channels = u16_at(body, 2).ok_or("short fmt")?;
                let rate = u32_at(body, 4).ok_or("short fmt")?;
                let bits = u16_at(body, 14).ok_or("short fmt")?;
                if tag == 0xFFFE {
                    // WAVE_FORMAT_EXTENSIBLE: the sub-format GUID starts with the tag.
                    tag = u16_at(body, 24).unwrap_or(1);
                }
                format = Some((tag, channels, rate, bits));
            }
            b"data" => data = Some(body),
            _ => {}
        }
        at += 8 + size + (size & 1);
    }
    let (tag, channels, rate, bits) = format.ok_or("no fmt chunk")?;
    let data = data.ok_or("no data chunk")?;
    if channels == 0 || rate == 0 {
        return Err("invalid WAV format".into());
    }
    let width = (bits as usize).div_ceil(8);
    let frame = width * channels as usize;
    let decode = |s: &[u8]| -> f32 {
        match (tag, bits) {
            (3, 32) => f32::from_le_bytes([s[0], s[1], s[2], s[3]]),
            (3, 64) => f64::from_le_bytes(s[..8].try_into().unwrap_or([0; 8])) as f32,
            (1, 8) => (s[0] as f32 - 128.0) / 128.0,
            (1, 16) => i16::from_le_bytes([s[0], s[1]]) as f32 / 32_768.0,
            (1, 24) => (i32::from_le_bytes([0, s[0], s[1], s[2]]) >> 8) as f32 / 8_388_608.0,
            (1, 32) => i32::from_le_bytes([s[0], s[1], s[2], s[3]]) as f32 / 2_147_483_648.0,
            _ => 0.0,
        }
    };
    if !matches!(
        (tag, bits),
        (3, 32) | (3, 64) | (1, 8) | (1, 16) | (1, 24) | (1, 32)
    ) {
        return Err(format!("unsupported WAV encoding {tag}/{bits}-bit"));
    }
    let samples = data
        .chunks_exact(frame)
        .map(|f| f.chunks_exact(width).map(decode).sum::<f32>() / channels as f32)
        .collect();
    Ok(Audio { rate, samples })
}

/// 32-bit float mono WAV.
pub fn write_float(rate: u32, samples: &[f32]) -> Vec<u8> {
    let data_len = samples.len() * 4;
    let mut out = Vec::with_capacity(44 + data_len);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data_len as u32).to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    out.extend_from_slice(&3u16.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&rate.to_le_bytes());
    out.extend_from_slice(&(rate * 4).to_le_bytes());
    out.extend_from_slice(&4u16.to_le_bytes());
    out.extend_from_slice(&32u16.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&(data_len as u32).to_le_bytes());
    for s in samples {
        out.extend_from_slice(&s.to_le_bytes());
    }
    out
}

/// Resamples by cubic interpolation; enough for analysis input, where the
/// envelope does not depend on the rate.
pub fn resample(samples: &[f32], from: u32, to: u32) -> Vec<f32> {
    if from == to || samples.is_empty() {
        return samples.to_vec();
    }
    let ratio = from as f64 / to as f64;
    let len = (samples.len() as f64 / ratio).floor() as usize;
    let at = |i: isize| samples[i.clamp(0, samples.len() as isize - 1) as usize];
    (0..len)
        .map(|j| {
            let x = j as f64 * ratio;
            let i = x.floor() as isize;
            let t = (x - i as f64) as f32;
            let (y0, y1, y2, y3) = (at(i - 1), at(i), at(i + 1), at(i + 2));
            let a = -0.5 * y0 + 1.5 * y1 - 1.5 * y2 + 0.5 * y3;
            let b = y0 - 2.5 * y1 + 2.0 * y2 - 0.5 * y3;
            let c = -0.5 * y0 + 0.5 * y2;
            ((a * t + b) * t + c) * t + y1
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn float_round_trip() {
        let samples = [0.0, 0.5, -0.25, 1.0];
        let audio = read(&write_float(48_000, &samples)).unwrap();
        assert_eq!(audio.rate, 48_000);
        assert_eq!(audio.samples, samples);
    }
    #[test]
    fn stereo_16_bit_folds_to_mono() {
        let mut bytes = write_float(8000, &[]);
        // Rewrite as 16-bit stereo PCM with one frame.
        bytes[20..22].copy_from_slice(&1u16.to_le_bytes());
        bytes[22..24].copy_from_slice(&2u16.to_le_bytes());
        bytes[34..36].copy_from_slice(&16u16.to_le_bytes());
        bytes[40..44].copy_from_slice(&4u32.to_le_bytes());
        bytes.extend_from_slice(&16_384i16.to_le_bytes());
        bytes.extend_from_slice(&0i16.to_le_bytes());
        let audio = read(&bytes).unwrap();
        assert_eq!(audio.samples, [0.25]);
    }
}
