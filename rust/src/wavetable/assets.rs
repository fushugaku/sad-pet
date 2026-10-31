//! Canonical float32 cycles; binary payloads are portable and hash-validated.
use super::{FRAME_SIZE, MAX_FRAMES};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use rustfft::{FftPlanner, num_complex::Complex64};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, sync::Arc};

const MAX_SAMPLES: usize = FRAME_SIZE * MAX_FRAMES;
const HEADER: &[u8; 8] = b"SAD-WT1\0";

#[derive(Clone, Debug, Serialize)]
pub struct Asset {
    pub id: String,
    pub content_hash: String,
    pub name: String,
    pub author: String,
    pub provenance: String,
    pub canonical_version: u32,
    pub frame_size: usize,
    pub frame_count: usize,
    #[serde(with = "samples_codec")]
    pub samples: Arc<[f32]>,
}

impl Asset {
    /// One or more known periods, never pitch-estimated or phase-aligned.
    pub fn from_cycles(name: String, frame_size: usize, samples: &[f32]) -> Result<Self, String> {
        if ![128, 256, 512, 1024, 2048, 4096].contains(&frame_size) {
            return Err("Choose a frame size of 128, 256, 512, 1024, 2048 or 4096 samples".into());
        }
        if samples.is_empty() || !samples.len().is_multiple_of(frame_size) {
            return Err("The wavetable must contain complete, non-empty periods".into());
        }
        let frames = samples.len() / frame_size;
        if frames > MAX_FRAMES {
            return Err("A wavetable can contain at most 256 frames".into());
        }
        if samples.iter().any(|v| !v.is_finite()) {
            return Err("The wavetable contains NaN or infinite samples".into());
        }
        let mut prepared = if frame_size == FRAME_SIZE {
            samples.iter().map(|&x| x as f64).collect()
        } else {
            resample_cycles(samples, frame_size)
        };
        let mut peak = 0.0_f64;
        for frame in prepared.chunks_exact_mut(FRAME_SIZE) {
            let mean = frame.iter().sum::<f64>() / FRAME_SIZE as f64;
            for x in frame {
                *x -= mean;
                peak = peak.max(x.abs());
            }
        }
        let gain = if peak > 1.0 { 1.0 / peak } else { 1.0 };
        let canonical: Vec<_> = prepared
            .iter()
            .map(|x| {
                let x = (x * gain) as f32;
                if x == 0.0 { 0.0 } else { x }
            })
            .collect();
        let content_hash = hash(frames, &canonical);
        Ok(Self {
            id: format!("user:{content_hash}"),
            content_hash,
            name: name.chars().take(160).collect(),
            author: String::new(),
            provenance: "Imported periodic waveform; relative frame phases preserved".into(),
            canonical_version: 1,
            frame_size: FRAME_SIZE,
            frame_count: frames,
            samples: canonical.into(),
        })
    }
    pub fn validate(&self) -> Result<(), String> {
        if self.canonical_version != 1 || self.frame_size != FRAME_SIZE {
            return Err("Unsupported canonical wavetable format".into());
        }
        if !(1..=MAX_FRAMES).contains(&self.frame_count)
            || self.samples.len() != self.frame_count * FRAME_SIZE
            || self
                .samples
                .iter()
                .any(|v| !v.is_finite() || v.abs() > 1.001 || v.to_bits() == 0x8000_0000)
        {
            return Err("Invalid canonical wavetable dimensions or samples".into());
        }
        if hash(self.frame_count, &self.samples) != self.content_hash {
            return Err("Wavetable content hash does not match its samples".into());
        }
        if self.id.starts_with("user:") && self.id != format!("user:{}", self.content_hash) {
            return Err("Wavetable identity does not match its content".into());
        }
        if !self.id.starts_with("user:")
            && super::factory::find(&self.id).is_none_or(|f| f.content_hash != self.content_hash)
        {
            return Err("Unknown factory wavetable identity".into());
        }
        Ok(())
    }
    pub fn preview(&self, points: usize) -> Vec<Vec<f32>> {
        let points = points.clamp(16, 256);
        self.samples
            .chunks_exact(FRAME_SIZE)
            .map(|frame| {
                (0..points)
                    .map(|i| frame[i * FRAME_SIZE / points])
                    .collect()
            })
            .collect()
    }
    pub fn canonical_bytes(&self) -> Vec<u8> {
        self.samples.iter().flat_map(|v| v.to_le_bytes()).collect()
    }
}

impl<'de> Deserialize<'de> for Asset {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Data {
            id: String,
            content_hash: String,
            name: String,
            #[serde(default)]
            author: String,
            #[serde(default)]
            provenance: String,
            canonical_version: u32,
            frame_size: usize,
            frame_count: usize,
            #[serde(with = "samples_codec")]
            samples: Arc<[f32]>,
        }
        let d = Data::deserialize(deserializer)?;
        let asset = Self {
            id: d.id,
            content_hash: d.content_hash,
            name: d.name,
            author: d.author,
            provenance: d.provenance,
            canonical_version: d.canonical_version,
            frame_size: d.frame_size,
            frame_count: d.frame_count,
            samples: d.samples,
        };
        asset.validate().map_err(serde::de::Error::custom)?;
        Ok(asset)
    }
}

pub fn hash(frames: usize, samples: &[f32]) -> String {
    let mut digest = Sha256::new();
    digest.update(HEADER);
    digest.update((FRAME_SIZE as u32).to_le_bytes());
    digest.update((frames as u32).to_le_bytes());
    let mut block = [0u8; 4096];
    for chunk in samples.chunks(1024) {
        for (bytes, sample) in block.chunks_exact_mut(4).zip(chunk) {
            bytes.copy_from_slice(&sample.to_le_bytes());
        }
        digest.update(&block[..chunk.len() * 4]);
    }
    format!("{:x}", digest.finalize())
}

/// Control-side deduplication. Runtime voices never consult this map.
#[derive(Clone, Debug, Default)]
pub struct Pool {
    assets: BTreeMap<String, Arc<Asset>>,
    directory: Option<Arc<std::path::PathBuf>>,
    temporary: Option<(u64, Arc<Asset>)>,
    preview_generation: u64,
}
impl Pool {
    pub fn preview(&mut self, token: u64, asset: Asset) -> Result<Arc<Asset>, String> {
        if token < self.preview_generation {
            return Err("This wavetable preview was superseded".into());
        }
        self.preview_generation = token;
        self.admit(&asset)?;
        let asset = Arc::new(asset);
        self.temporary = Some((token, asset.clone()));
        Ok(asset)
    }
    pub fn commit_preview(&mut self, token: u64, id: &str) -> Result<(Arc<Asset>, bool), String> {
        let asset = self
            .temporary
            .as_ref()
            .filter(|(t, a)| *t == token && a.id == id)
            .map(|(_, a)| a.clone())
            .ok_or("The preview changed; preview the table again before applying")?;
        self.admit(&asset)?;
        let existing = self.contains(id);
        self.persist(&asset)?;
        let asset = self.insert((*asset).clone())?;
        self.clear_preview(token);
        Ok((asset, existing))
    }
    pub fn clear_preview(&mut self, token: u64) {
        if self
            .temporary
            .as_ref()
            .is_some_and(|(generation, _)| *generation == token)
        {
            self.temporary = None;
        }
    }
    pub fn set_directory(&mut self, path: std::path::PathBuf) {
        self.directory = Some(Arc::new(path));
    }
    fn path(&self, id: &str) -> Option<std::path::PathBuf> {
        let hash = id.strip_prefix("user:")?;
        if hash.len() != 64
            || !hash
                .bytes()
                .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
        {
            return None;
        }
        Some(self.directory.as_ref()?.join(format!("{hash}.sadwt")))
    }
    pub fn contains(&self, id: &str) -> bool {
        self.assets.contains_key(id) || self.path(id).is_some_and(|p| p.is_file())
    }
    pub fn remove(&mut self, id: &str) {
        self.assets.remove(id);
    }
    pub fn admit(&self, asset: &Asset) -> Result<(), String> {
        let used: usize = self.assets.values().map(|a| a.samples.len() * 4).sum();
        if !self.assets.contains_key(&asset.id) && used + asset.samples.len() * 4 > 64 * 1024 * 1024
        {
            return Err(
                "The wavetable asset pool exceeds 64 MiB; the current sound was kept".into(),
            );
        }
        Ok(())
    }
    pub fn persist(&self, asset: &Asset) -> Result<(), String> {
        use std::io::Write;
        let Some(path) = self.path(&asset.id) else {
            return Ok(());
        };
        if path.is_file() {
            return Ok(());
        }
        let directory = path.parent().unwrap();
        std::fs::create_dir_all(directory)
            .map_err(|e| format!("Cannot open the wavetable library: {e}"))?;
        let mut encoded = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
        serde_json::to_writer(&mut encoded, asset).map_err(|e| e.to_string())?;
        let bytes = encoded.finish().map_err(|e| e.to_string())?;
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let temporary = directory.join(format!(
            ".{}-{}-{nonce}.tmp",
            asset.content_hash,
            std::process::id()
        ));
        let result = (|| {
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&temporary)
                .map_err(|e| e.to_string())?;
            file.write_all(&bytes).map_err(|e| e.to_string())?;
            std::fs::rename(&temporary, &path)
                .map_err(|e| format!("Cannot save the wavetable: {e}"))
        })();
        if result.is_err() {
            let _ = std::fs::remove_file(temporary);
        }
        result
    }
    fn read_file(path: &std::path::Path) -> Result<Arc<Asset>, String> {
        use std::io::Read;
        let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
        let mut decoder = flate2::read::GzDecoder::new(file).take(3 * 1024 * 1024 + 1);
        let mut bytes = Vec::new();
        decoder.read_to_end(&mut bytes).map_err(|e| e.to_string())?;
        if bytes.len() > 3 * 1024 * 1024 {
            return Err("The saved wavetable exceeds 3 MiB".into());
        }
        serde_json::from_slice(&bytes).map_err(|e| format!("Saved wavetable: {e}"))
    }
    pub fn disk_records(&self) -> Vec<serde_json::Value> {
        let Some(directory) = &self.directory else {
            return Vec::new();
        };
        let Ok(files) = std::fs::read_dir(&**directory) else {
            return Vec::new();
        };
        files
            .flatten()
            .filter_map(|entry| {
                let path = entry.path();
                if path.extension().is_none_or(|e| e != "sadwt") {
                    return None;
                }
                let asset = Self::read_file(&path).ok()?;
                if self.path(&asset.id).as_deref() != Some(path.as_path())
                    || self.assets.contains_key(&asset.id)
                {
                    return None;
                }
                Some(super::library::record(&asset))
            })
            .collect()
    }
    pub fn from_manifest(assets: &[Arc<Asset>]) -> Result<Self, String> {
        let mut pool = Self::default();
        for asset in assets {
            if !asset.id.starts_with("user:") {
                return Err("Factory wavetables are referenced by identity, not embedded".into());
            }
            pool.insert((**asset).clone())?;
        }
        Ok(pool)
    }
    pub fn merge(&mut self, other: &Self) -> Result<(), String> {
        let extra: usize = other
            .assets
            .iter()
            .filter(|(id, _)| !self.assets.contains_key(*id))
            .map(|(_, a)| a.samples.len() * 4)
            .sum();
        let used: usize = self.assets.values().map(|a| a.samples.len() * 4).sum();
        if used + extra > 64 * 1024 * 1024 {
            return Err(
                "The wavetable asset pool exceeds 64 MiB; the current sound was kept".into(),
            );
        }
        for (id, asset) in &other.assets {
            self.assets
                .entry(id.clone())
                .or_insert_with(|| Arc::clone(asset));
        }
        Ok(())
    }
    /// Only referenced user data travels with a sound. Both oscillators,
    /// including a disabled one, retain their exact asset when recalled.
    pub fn manifest(&self, settings: Option<&super::Settings>) -> Result<Vec<Arc<Asset>>, String> {
        let mut used = BTreeMap::new();
        if let Some(settings) = settings {
            settings.validate_version()?;
            for oscillator in settings.oscillators.iter().take(settings.count as usize) {
                let asset = self.resolve(&oscillator.asset_id)?;
                if asset.id.starts_with("user:") {
                    used.entry(asset.id.clone()).or_insert(asset);
                }
            }
        }
        Ok(used.into_values().collect())
    }
    pub fn insert(&mut self, asset: Asset) -> Result<Arc<Asset>, String> {
        asset.validate()?;
        self.admit(&asset)?;
        let entry = self
            .assets
            .entry(asset.id.clone())
            .or_insert_with(|| Arc::new(asset));
        Ok(Arc::clone(entry))
    }
    pub fn resolve(&self, id: &str) -> Result<Arc<Asset>, String> {
        if let Some(asset) = self.assets.get(id) {
            return Ok(Arc::clone(asset));
        }
        if let Some((_, asset)) = &self.temporary
            && asset.id == id
        {
            return Ok(asset.clone());
        }
        if let Some(path) = self.path(id) {
            let asset = Self::read_file(&path)?;
            if asset.id != id {
                return Err("Saved wavetable identity does not match its file".into());
            }
            return Ok(asset);
        }
        super::factory::find(id)
            .ok_or_else(|| format!("Wavetable is missing: {id}"))?
            .load()
    }
    pub fn user_assets(&self) -> impl Iterator<Item = &Arc<Asset>> {
        self.assets.values().filter(|a| a.id.starts_with("user:"))
    }
}

fn resample_cycles(samples: &[f32], frame_size: usize) -> Vec<f64> {
    let mut planner = FftPlanner::<f64>::new();
    let forward = planner.plan_fft_forward(frame_size);
    let inverse = planner.plan_fft_inverse(FRAME_SIZE);
    let mut scratch = vec![
        Complex64::default();
        forward
            .get_inplace_scratch_len()
            .max(inverse.get_inplace_scratch_len())
    ];
    let mut source = vec![Complex64::default(); frame_size];
    let mut target = vec![Complex64::default(); FRAME_SIZE];
    let mut result = Vec::with_capacity(samples.len() / frame_size * FRAME_SIZE);
    let last = (frame_size / 2).min(FRAME_SIZE / 2 - 1);
    for frame in samples.chunks_exact(frame_size) {
        for (to, &x) in source.iter_mut().zip(frame) {
            *to = Complex64::new(x as f64, 0.0);
        }
        forward.process_with_scratch(&mut source, &mut scratch);
        target.fill(Complex64::default());
        for h in 1..=last {
            let value = source[h] * if h == frame_size / 2 { 0.5 } else { 1.0 };
            target[h] = value;
            target[FRAME_SIZE - h] = value.conj();
        }
        inverse.process_with_scratch(&mut target, &mut scratch);
        result.extend(target.iter().map(|x| x.re / frame_size as f64));
    }
    result
}

mod samples_codec {
    use super::*;
    pub fn serialize<S: serde::Serializer>(samples: &Arc<[f32]>, s: S) -> Result<S::Ok, S::Error> {
        let bytes: Vec<_> = samples.iter().flat_map(|v| v.to_le_bytes()).collect();
        s.serialize_str(&STANDARD.encode(bytes))
    }
    pub fn deserialize<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Arc<[f32]>, D::Error> {
        let encoded = String::deserialize(d)?;
        if encoded.len() > (MAX_SAMPLES * 4).div_ceil(3) * 4 {
            return Err(serde::de::Error::custom("Wavetable payload exceeds 2 MiB"));
        }
        let bytes = STANDARD.decode(encoded).map_err(serde::de::Error::custom)?;
        if bytes.is_empty() || bytes.len() > MAX_SAMPLES * 4 || !bytes.len().is_multiple_of(4) {
            return Err(serde::de::Error::custom("Invalid wavetable payload length"));
        }
        Ok(bytes
            .chunks_exact(4)
            .map(|b| f32::from_le_bytes(b.try_into().unwrap()))
            .collect::<Vec<_>>()
            .into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn identity_preserves_frame_levels_and_is_portable() {
        let mut samples = vec![0.0; FRAME_SIZE * 2];
        for (i, x) in samples.iter_mut().enumerate() {
            *x = (i as f64 * std::f64::consts::TAU / FRAME_SIZE as f64).sin() as f32
                * if i < FRAME_SIZE { 0.8 } else { 0.2 }
                + 0.1;
        }
        let a = Asset::from_cycles("A".into(), FRAME_SIZE, &samples).unwrap();
        let b = Asset::from_cycles("Another name".into(), FRAME_SIZE, &samples).unwrap();
        assert_eq!(a.id, b.id);
        let bytes = serde_json::to_vec(&a).unwrap();
        let restored: Asset = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(a.samples, restored.samples);
        let peak = |x: &[f32]| x.iter().fold(0.0_f32, |a, x| a.max(x.abs()));
        assert!(
            (peak(&a.samples[..FRAME_SIZE]) / peak(&a.samples[FRAME_SIZE..]) - 4.0).abs() < 1e-5
        );
        let mut corrupted = serde_json::to_value(&a).unwrap();
        corrupted["content_hash"] = "bad".into();
        assert!(serde_json::from_value::<Asset>(corrupted).is_err());
    }
    #[test]
    fn periodic_resampling_preserves_pitch_phase_and_amplitude() {
        for n in [128, 256, 512, 1024, 4096] {
            let source: Vec<_> = (0..n)
                .map(|i| ((i as f64 / n as f64 + 0.13) * std::f64::consts::TAU).sin() as f32 * 0.7)
                .collect();
            let a = Asset::from_cycles("Sine".into(), n, &source).unwrap();
            for (i, &x) in a.samples.iter().enumerate() {
                let expected =
                    ((i as f64 / FRAME_SIZE as f64 + 0.13) * std::f64::consts::TAU).sin() * 0.7;
                assert!((x as f64 - expected).abs() < 2e-6);
            }
        }
        assert!(Asset::from_cycles("Bad".into(), 2048, &[f32::NAN; 2048]).is_err());
        assert!(Asset::from_cycles("Bad".into(), 2048, &[0.0; 2049]).is_err());
    }
}
