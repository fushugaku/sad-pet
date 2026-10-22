//! Reads zip archives (stored and deflated entries), as voicebanks, .mxl
//! scores and VOCALOID5 .vpr projects are shared.

/// The files of a zip archive (stored or deflated entries).
pub fn unzip(bytes: &[u8]) -> Result<crate::utau::MemoryFiles, String> {
    use std::io::Read;
    let u16_at = |i: usize| {
        bytes
            .get(i..i + 2)
            .map(|b| u16::from_le_bytes([b[0], b[1]]) as usize)
    };
    let u32_at = |i: usize| {
        bytes
            .get(i..i + 4)
            .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as usize)
    };
    let end = (0..bytes.len().saturating_sub(21))
        .rev()
        .find(|&i| u32_at(i) == Some(0x0605_4b50))
        .ok_or("Not a zip file")?;
    let count = u16_at(end + 10).ok_or("Damaged zip file")?;
    let mut at = u32_at(end + 16).ok_or("Damaged zip file")?;
    let mut files = crate::utau::MemoryFiles::default();
    for _ in 0..count {
        if u32_at(at) != Some(0x0201_4b50) {
            return Err("Damaged zip file".into());
        }
        let flags = u16_at(at + 8).unwrap_or(0);
        let method = u16_at(at + 10).unwrap_or(0);
        let size = u32_at(at + 20).ok_or("Damaged zip file")?;
        let name_length = u16_at(at + 28).unwrap_or(0);
        let extra = u16_at(at + 30).unwrap_or(0);
        let comment = u16_at(at + 32).unwrap_or(0);
        let local = u32_at(at + 42).ok_or("Damaged zip file")?;
        let raw = bytes
            .get(at + 46..at + 46 + name_length)
            .ok_or("Damaged zip file")?;
        // Japanese voicebanks are often zipped with Shift-JIS names.
        let name = if flags & 0x800 != 0 {
            String::from_utf8_lossy(raw).into_owned()
        } else {
            crate::utau::decode_text(raw)
        };
        at += 46 + name_length + extra + comment;
        if name.ends_with('/') {
            continue;
        }
        let start = local + 30 + u16_at(local + 26).unwrap_or(0) + u16_at(local + 28).unwrap_or(0);
        let data = bytes.get(start..start + size).ok_or("Damaged zip file")?;
        let content = match method {
            0 => data.to_vec(),
            8 => {
                let mut out = Vec::new();
                flate2::read::DeflateDecoder::new(data)
                    .take(256 << 20)
                    .read_to_end(&mut out)
                    .map_err(|e| format!("{name}: {e}"))?;
                out
            }
            _ => continue,
        };
        files.0.insert(name.replace('\\', "/"), content);
    }
    Ok(files)
}
