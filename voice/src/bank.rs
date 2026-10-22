//! Voicebanks: analyzed frames of the recordings plus units, the labelled
//! stretches of them (diphones, CV and VCV syllables, sustained vowels) that
//! synthesis cuts and joins. Frames are kept quantized, as on disk: 0.75 dB
//! envelope steps and 0.25 dB aperiodicity steps are far below audibility
//! and make a bank about a quarter of its float size.
use crate::phoneme::{Class, Language, Phoneme};
use crate::spectrum::{AP_BANDS, ENV_POINTS, FLOOR_DB, Frame};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const MAGIC: &[u8; 8] = b"SADVOICE";
const VERSION: u32 = 1;
const ENV_STEP: f32 = 0.75;
const AP_STEP_DB: f32 = 0.25;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Info {
    pub name: String,
    pub author: String,
    pub language: Language,
    /// Sample rate of the analyzed recordings.
    pub rate: u32,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Unit {
    pub alias: String,
    pub phonemes: Vec<Phoneme>,
    /// Frame index where each phoneme starts, plus the end: `phonemes.len() + 1` entries.
    pub bounds: Vec<u32>,
    /// End of the part that must not be stretched (UTAU's consonant area).
    pub fixed: u32,
    /// A steady stretch of the last phoneme that may be looped; empty if none.
    pub stable: [u32; 2],
    /// Median pitch of the voiced frames, as a MIDI note.
    pub pitch: f32,
    /// Median level of the voiced frames in dB.
    pub level: f32,
}
impl Unit {
    pub fn start(&self) -> u32 {
        self.bounds[0]
    }
    pub fn end(&self) -> u32 {
        *self.bounds.last().unwrap_or(&0)
    }
    /// Frames of phoneme `i`.
    pub fn span(&self, i: usize) -> (u32, u32) {
        (self.bounds[i], self.bounds[i + 1])
    }
}

#[derive(Serialize, Deserialize)]
struct Header {
    info: Info,
    units: Vec<Unit>,
    frames: u32,
}

#[derive(Clone, Debug, Default)]
pub struct Voicebank {
    pub info: Info,
    pub units: Vec<Unit>,
    f0: Vec<f32>,
    env: Vec<u8>,
    ap: Vec<u8>,
    by_sequence: HashMap<Vec<Phoneme>, Vec<u32>>,
    inventory: Vec<Phoneme>,
    durations: HashMap<Phoneme, f32>,
    median_level: f32,
    /// Mean envelope of the voiced frames: the voice's overall color.
    color: Vec<f32>,
}

fn quantize_env(db: f32) -> u8 {
    ((db - FLOOR_DB) / ENV_STEP).round().clamp(0.0, 255.0) as u8
}
fn quantize_ap(a: f32) -> u8 {
    (-10.0 * a.clamp(1e-6, 1.0).log10() / AP_STEP_DB)
        .round()
        .clamp(0.0, 240.0) as u8
}

impl Voicebank {
    pub fn new(info: Info) -> Self {
        Self {
            info,
            ..Default::default()
        }
    }
    pub fn frame_count(&self) -> usize {
        self.f0.len()
    }
    /// Appends frames and returns the index of the first.
    pub fn push_frames(&mut self, frames: &[Frame]) -> u32 {
        let start = self.f0.len() as u32;
        for f in frames {
            self.f0.push(f.f0);
            self.env.extend(f.env.iter().map(|&d| quantize_env(d)));
            self.ap.extend(f.ap.iter().map(|&a| quantize_ap(a)));
        }
        start
    }
    pub fn frame(&self, index: usize) -> Frame {
        let index = index.min(self.f0.len().saturating_sub(1));
        if self.f0.is_empty() {
            return Frame::SILENT;
        }
        let mut frame = Frame {
            f0: self.f0[index],
            ..Frame::SILENT
        };
        let env = &self.env[index * ENV_POINTS..(index + 1) * ENV_POINTS];
        for (o, &q) in frame.env.iter_mut().zip(env) {
            *o = FLOOR_DB + q as f32 * ENV_STEP;
        }
        let ap = &self.ap[index * AP_BANDS..(index + 1) * AP_BANDS];
        for (o, &q) in frame.ap.iter_mut().zip(ap) {
            *o = 10f32.powf(-(q as f32) * AP_STEP_DB / 10.0);
        }
        frame
    }
    /// Frame at a fractional index, interpolated.
    pub fn frame_at(&self, position: f32) -> Frame {
        let i = position.floor().max(0.0) as usize;
        let t = position - i as f32;
        let a = self.frame(i);
        if t <= 0.0 {
            return a;
        }
        a.mix(&self.frame(i + 1), t)
    }
    pub fn f0(&self, index: usize) -> f32 {
        self.f0.get(index).copied().unwrap_or(0.0)
    }

    pub fn add_unit(&mut self, unit: Unit) {
        self.units.push(unit);
    }

    /// Builds the lookup tables; call after the units are complete.
    pub fn finish(&mut self) {
        self.by_sequence.clear();
        let mut seen = std::collections::BTreeSet::new();
        let mut lengths: HashMap<Phoneme, Vec<f32>> = HashMap::new();
        for (i, unit) in self.units.iter().enumerate() {
            self.by_sequence
                .entry(unit.phonemes.clone())
                .or_default()
                .push(i as u32);
            for (k, &p) in unit.phonemes.iter().enumerate() {
                seen.insert(p);
                // A consonant fully inside a unit, between two other phonemes,
                // shows its natural length.
                if !p.is_vowel() && !p.is_pause() && k > 0 && k + 1 < unit.phonemes.len() {
                    let (a, b) = unit.span(k);
                    lengths
                        .entry(p)
                        .or_default()
                        .push((b - a) as f32 * crate::spectrum::FRAME_SECONDS);
                }
            }
        }
        self.inventory = seen.into_iter().collect();
        let mut levels: Vec<f32> = self
            .units
            .iter()
            .map(|u| u.level)
            .filter(|l| *l > FLOOR_DB + 10.0)
            .collect();
        levels.sort_by(f32::total_cmp);
        self.median_level = levels.get(levels.len() / 2).copied().unwrap_or(-80.0);
        let mut color = vec![0.0f64; ENV_POINTS];
        let mut count = 0usize;
        let step = (self.f0.len() / 2000).max(1);
        for i in (0..self.f0.len()).step_by(step) {
            if self.f0[i] > 0.0 {
                let f = self.frame(i);
                if f.level() > self.median_level - 10.0 {
                    for (c, e) in color.iter_mut().zip(&f.env) {
                        *c += *e as f64;
                    }
                    count += 1;
                }
            }
        }
        self.color = if count > 0 {
            color.iter().map(|c| (c / count as f64) as f32).collect()
        } else {
            vec![-90.0; ENV_POINTS]
        };
        self.durations = lengths
            .into_iter()
            .map(|(p, mut v)| {
                v.sort_by(f32::total_cmp);
                (p, v[v.len() / 2])
            })
            .collect();
    }

    pub fn units_for(&self, phonemes: &[Phoneme]) -> &[u32] {
        self.by_sequence.get(phonemes).map_or(&[], Vec::as_slice)
    }
    pub fn inventory(&self) -> &[Phoneme] {
        &self.inventory
    }
    pub fn has(&self, p: Phoneme) -> bool {
        self.inventory.binary_search(&p).is_ok()
    }
    pub fn median_level(&self) -> f32 {
        self.median_level
    }
    /// Mean voiced envelope (dB per Hz on the grid).
    pub fn color(&self) -> &[f32] {
        &self.color
    }
    /// The voice's own length of a consonant, if it was measured.
    pub fn duration(&self, p: Phoneme) -> Option<f32> {
        self.durations.get(&p).copied()
    }
    /// The bank phoneme closest to `p`, for languages or sounds it lacks.
    pub fn substitute(&self, p: Phoneme) -> Phoneme {
        if p.is_pause() || p.class() == Class::Closure || self.has(p) || self.inventory.is_empty() {
            return p;
        }
        *self
            .inventory
            .iter()
            .filter(|q| !q.is_pause())
            .min_by(|a, b| p.distance(**a).total_cmp(&p.distance(**b)))
            .unwrap_or(&p)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let header = serde_json::to_vec(&Header {
            info: self.info.clone(),
            units: self.units.clone(),
            frames: self.f0.len() as u32,
        })
        .expect("voicebank header");
        let mut out =
            Vec::with_capacity(16 + header.len() + self.f0.len() * (4 + ENV_POINTS + AP_BANDS));
        out.extend_from_slice(MAGIC);
        out.extend_from_slice(&VERSION.to_le_bytes());
        out.extend_from_slice(&(header.len() as u32).to_le_bytes());
        out.extend_from_slice(&header);
        for f in &self.f0 {
            out.extend_from_slice(&f.to_le_bytes());
        }
        out.extend_from_slice(&self.env);
        out.extend_from_slice(&self.ap);
        out
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.get(..8) != Some(MAGIC) {
            return Err("not a SAD PET voicebank".into());
        }
        let word = |i: usize| -> Result<u32, String> {
            Ok(u32::from_le_bytes(
                bytes
                    .get(i..i + 4)
                    .ok_or("truncated voicebank")?
                    .try_into()
                    .unwrap(),
            ))
        };
        if word(8)? != VERSION {
            return Err("unsupported voicebank version".into());
        }
        let header_len = word(12)? as usize;
        let header: Header = serde_json::from_slice(
            bytes
                .get(16..16 + header_len)
                .ok_or("truncated voicebank")?,
        )
        .map_err(|e| format!("voicebank header: {e}"))?;
        let n = header.frames as usize;
        let mut at = 16 + header_len;
        let f0_bytes = bytes.get(at..at + n * 4).ok_or("truncated frames")?;
        let f0 = f0_bytes
            .chunks_exact(4)
            .map(|c| f32::from_le_bytes(c.try_into().unwrap()))
            .collect();
        at += n * 4;
        let env = bytes
            .get(at..at + n * ENV_POINTS)
            .ok_or("truncated envelopes")?
            .to_vec();
        at += n * ENV_POINTS;
        let ap = bytes
            .get(at..at + n * AP_BANDS)
            .ok_or("truncated aperiodicity")?
            .to_vec();
        for unit in &header.units {
            let valid = unit.bounds.len() == unit.phonemes.len() + 1
                && unit.bounds.windows(2).all(|w| w[0] <= w[1])
                && unit.end() as usize <= n;
            if !valid {
                return Err(format!("unit {} is out of range", unit.alias));
            }
        }
        let mut bank = Self {
            info: header.info,
            units: header.units,
            f0,
            env,
            ap,
            ..Default::default()
        };
        bank.finish();
        Ok(bank)
    }

    /// Median voiced level and pitch of a frame range.
    pub fn measure(&self, start: u32, end: u32) -> (f32, f32) {
        let mut pitches = Vec::new();
        let mut levels = Vec::new();
        for i in start..end {
            let f = self.frame(i as usize);
            if f.voiced() {
                pitches.push(69.0 + 12.0 * (f.f0 / 440.0).log2());
                levels.push(f.level());
            }
        }
        let median = |v: &mut Vec<f32>| {
            if v.is_empty() {
                return None;
            }
            v.sort_by(f32::total_cmp);
            Some(v[v.len() / 2])
        };
        (
            median(&mut pitches).unwrap_or(60.0),
            median(&mut levels).unwrap_or(FLOOR_DB),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bytes_round_trip_within_quantization() {
        let mut bank = Voicebank::new(Info {
            name: "Test".into(),
            language: Language::Ja,
            rate: 44_100,
            ..Default::default()
        });
        let mut frame = Frame::SILENT;
        frame.f0 = 220.0;
        for (i, e) in frame.env.iter_mut().enumerate() {
            *e = -100.0 - i as f32 * 0.4;
        }
        frame.ap = [0.001, 0.01, 0.05, 0.1, 0.2, 0.4, 0.7, 1.0];
        let start = bank.push_frames(&[frame; 10]);
        let a = Phoneme::parse(Language::Ja, "a").unwrap();
        bank.add_unit(Unit {
            alias: "a".into(),
            phonemes: vec![a],
            bounds: vec![start, start + 10],
            fixed: start,
            stable: [start + 2, start + 8],
            pitch: 57.0,
            level: -90.0,
        });
        bank.finish();
        let back = Voicebank::from_bytes(&bank.to_bytes()).unwrap();
        assert_eq!(back.units, bank.units);
        assert_eq!(back.units_for(&[a]), &[0]);
        let f = back.frame(3);
        assert_eq!(f.f0, 220.0);
        for (x, y) in f.env.iter().zip(&frame.env) {
            assert!((x - y).abs() <= ENV_STEP / 2.0 + 1e-3);
        }
        for (x, y) in f.ap.iter().zip(&frame.ap) {
            assert!((10.0 * (x / y).log10()).abs() <= AP_STEP_DB / 2.0 + 1e-3);
        }
    }
    #[test]
    fn substitutes_the_closest_phoneme() {
        let mut bank = Voicebank::new(Info::default());
        let ja = |s| Phoneme::parse(Language::Ja, s).unwrap();
        bank.push_frames(&[Frame::SILENT; 4]);
        for (s, i) in [("a", 0), ("i", 1), ("u", 2), ("k", 3)] {
            bank.add_unit(Unit {
                alias: s.into(),
                phonemes: vec![ja(s)],
                bounds: vec![i, i + 1],
                fixed: i,
                stable: [0, 0],
                pitch: 60.0,
                level: -90.0,
            });
        }
        bank.finish();
        let ru = |s| Phoneme::parse(Language::Ru, s).unwrap();
        // Ы is unrounded and central: Japanese u (ɯ) is the nearer vowel.
        assert_eq!(bank.substitute(ru("y")), ja("u"));
        assert_eq!(bank.substitute(ru("k'")), ja("k"));
        assert_eq!(bank.substitute(ru("a")), ja("a"));
    }
}
