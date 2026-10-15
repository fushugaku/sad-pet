//! STM32 v2 container validation, independent of audio and the desktop runtime.
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum ImageError {
    Truncated,
    Magic,
    Version(u32),
    Encrypted,
    Length,
    Checksum,
}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "STM32 image: {self:?}")
    }
}
impl std::error::Error for ImageError {}

#[derive(Debug)]
pub struct Image<'a> {
    pub payload: &'a [u8],
    pub entry: u32,
    pub flags: u32,
    pub checksum: u32,
}

impl<'a> Image<'a> {
    pub fn parse(bytes: &'a [u8]) -> Result<Self, ImageError> {
        if bytes.len() < 512 {
            return Err(ImageError::Truncated);
        }
        if &bytes[..4] != b"STM2" {
            return Err(ImageError::Magic);
        }
        let word = |o| u32::from_le_bytes(bytes[o..o + 4].try_into().unwrap());
        let version = word(0x48);
        if version != 0x0002_0000 {
            return Err(ImageError::Version(version));
        }
        let flags = word(0x64);
        if flags & 2 != 0 {
            return Err(ImageError::Encrypted);
        }
        let header = 128_usize
            .checked_add(word(0x68) as usize)
            .ok_or(ImageError::Length)?;
        if header != 512 || header.checked_add(word(0x4c) as usize) != Some(bytes.len()) {
            return Err(ImageError::Length);
        }
        let payload = &bytes[header..];
        let checksum = payload.iter().fold(0_u32, |s, &b| s.wrapping_add(b as u32));
        if checksum != word(0x44) {
            return Err(ImageError::Checksum);
        }
        Ok(Self {
            payload,
            checksum,
            flags,
            entry: word(0x50),
        })
    }
}
