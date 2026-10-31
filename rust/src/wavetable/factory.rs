//! Built-in, original wavetable packs. No files or network are needed at runtime.
use super::assets::Asset;
use flate2::read::GzDecoder;
use serde::Serialize;
use std::{
    collections::BTreeMap,
    io::Read,
    sync::{Arc, Mutex, OnceLock, Weak},
};

#[derive(Debug, Serialize)]
pub struct Factory {
    pub id: &'static str,
    pub name: &'static str,
    pub pack: &'static str,
    pub pack_name: &'static str,
    pub frames: usize,
    pub content_hash: &'static str,
    #[serde(skip)]
    bytes: &'static [u8],
}
include!("factory_data.rs");

pub fn find(id: &str) -> Option<&'static Factory> {
    FACTORY.iter().find(|f| f.id == id)
}

impl Factory {
    pub fn load(&self) -> Result<Arc<Asset>, String> {
        static CACHE: OnceLock<Mutex<BTreeMap<&'static str, Weak<Asset>>>> = OnceLock::new();
        let cache = CACHE.get_or_init(Mutex::default);
        if let Some(asset) = cache
            .lock()
            .map_err(|_| "Wavetable cache is unavailable")?
            .get(self.id)
            .and_then(Weak::upgrade)
        {
            return Ok(asset);
        }
        let length = self.frames * super::FRAME_SIZE * 2;
        let mut bytes = Vec::with_capacity(length);
        GzDecoder::new(self.bytes)
            .take(length as u64 + 1)
            .read_to_end(&mut bytes)
            .map_err(|e| format!("Factory wavetable: {e}"))?;
        if bytes.len() != length {
            return Err(format!("Invalid factory wavetable: {}", self.name));
        }
        let samples: Vec<_> = bytes
            .chunks_exact(2)
            .map(|b| i16::from_le_bytes(b.try_into().unwrap()) as f32 / 32768.0)
            .collect();
        let mut asset = Asset::from_cycles(self.name.into(), super::FRAME_SIZE, &samples)?;
        if asset.content_hash != self.content_hash {
            return Err(format!(
                "Factory wavetable checksum mismatch: {}",
                self.name
            ));
        }
        asset.id = self.id.into();
        asset.author = "SAD PET".into();
        asset.provenance = format!("Original procedural content; {} v1", self.pack_name);
        let asset = Arc::new(asset);
        cache
            .lock()
            .map_err(|_| "Wavetable cache is unavailable")?
            .insert(self.id, Arc::downgrade(&asset));
        Ok(asset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_packs_are_installed_and_have_distinct_valid_waves() {
        assert_eq!(FACTORY.len(), 96);
        let original: serde_json::Value = serde_json::from_str(include_str!(
            "../../tests/fixtures/wavetable/factory_v1_before.json"
        ))
        .unwrap();
        for old in original.as_array().unwrap() {
            let current = find(old["id"].as_str().unwrap()).unwrap();
            assert_eq!(
                current.content_hash,
                old["content_hash"].as_str().unwrap(),
                "changed original factory audio: {}",
                current.id
            );
            assert_eq!(current.frames, old["frames"].as_u64().unwrap() as usize);
        }
        let mut packs = std::collections::BTreeSet::new();
        let mut hashes = std::collections::BTreeSet::new();
        for entry in FACTORY {
            packs.insert(entry.pack);
            assert!(
                hashes.insert(entry.content_hash),
                "duplicate content: {}",
                entry.name
            );
            let asset = entry.load().unwrap();
            asset.validate().unwrap();
            assert_eq!(asset.frame_count, 64);
            assert!(asset.samples.iter().any(|x| x.abs() > 0.1));
            assert_ne!(
                &asset.samples[..super::super::FRAME_SIZE],
                &asset.samples[asset.samples.len() - super::super::FRAME_SIZE..]
            );
        }
        assert_eq!(packs.len(), 13);
    }
}
