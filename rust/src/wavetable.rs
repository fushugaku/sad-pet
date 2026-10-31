//! Wavetable settings and portable, content-addressed periodic wave assets.
//! Decoding, hashing and compilation belong to the control/worker owner.
pub mod assets;
pub mod edit;
pub mod factory;
pub mod import;
pub mod library;
pub mod maker;
pub mod motion;
pub mod presets;

use serde::{Deserialize, Serialize};

pub const OSCILLATORS: usize = 4;
pub const MAX_UNISON: usize = 8;
pub const FRAME_SIZE: usize = 2048;
pub const MAX_FRAMES: usize = 256;
pub const SCHEMA_VERSION: u32 = 2;
pub const ENGINE_REVISION: u32 = 3;
pub const DEFAULT_ASSET: &str = "factory:wt-basic-morph:v1";

pub fn default_controls() -> [Controls; OSCILLATORS] {
    std::array::from_fn(|i| Controls {
        enabled: i == 0,
        fm_source: (i ^ 1) as u8,
        sync_source: i as u8,
        ..Controls::default()
    })
}

/// Only small numeric data enters the control-rate modulation frame.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Controls {
    pub enabled: bool,
    pub position: f32,
    pub transpose: f32,
    pub fine: f32,
    pub level: f32,
    pub pan: f32,
    pub phase: f32,
    pub phase_random: f32,
    pub unison: u8,
    pub detune: f32,
    pub spread: f32,
    /// Position step between adjacent unison voices, centred around Position.
    pub voice_shift: f32,
    /// 0 = off; 1..8 select the engine voice's existing ENV.
    pub position_env: u8,
    pub position_env_depth: f32,
    pub surf_depth: f32,
    pub step: bool,
    /// Position offset over five octaves, centred on MIDI C4 (60).
    pub key_waves: f32,
    /// False = retrigger, true = free-running oscillator phase.
    pub free_phase: bool,
    pub position_lfo: u8,
    pub position_lfo_depth: f32,
    /// 0 off, 1 linear FM (deviation = index * source Hz), 2 PM (radians).
    pub fm_mode: u8,
    pub fm_index: f32,
    pub sync: bool,
    pub sync_ratio: f32,
    /// 0..3 = A..D; 4 is the missing-field migration sentinel.
    pub fm_source: u8,
    pub sync_source: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Field {
    Enabled,
    Position,
    Transpose,
    Fine,
    Level,
    Pan,
    Phase,
    PhaseRandom,
    Unison,
    Detune,
    Spread,
    Env,
    EnvDepth,
    SurfDepth,
    Step,
    KeyWaves,
    FreePhase,
    Lfo,
    LfoDepth,
    FmMode,
    FmIndex,
    Sync,
    SyncRatio,
    VoiceShift,
    FmSource,
    SyncSource,
}
pub const FIELDS: [(Field, &str, &str, f32, f32); 13] = [
    (Field::Enabled, "enabled", "enabled", 0.0, 1.0),
    (Field::Position, "position", "position", 0.0, 1.0),
    (Field::Transpose, "transpose", "transpose", -48.0, 48.0),
    (Field::Fine, "fine", "fine tune", -100.0, 100.0),
    (Field::Level, "level", "level", 0.0, 1.0),
    (Field::Pan, "pan", "pan", 0.0, 1.0),
    (Field::Phase, "phase", "start phase", 0.0, 1.0),
    (Field::PhaseRandom, "phase_random", "random phase", 0.0, 1.0),
    (Field::Unison, "unison", "unison voices", 1.0, 8.0),
    (Field::Detune, "detune", "unison detune", 0.0, 100.0),
    (Field::Spread, "spread", "stereo spread", 0.0, 1.0),
    (Field::Env, "env", "position envelope", 0.0, 8.0),
    (
        Field::EnvDepth,
        "env_depth",
        "position envelope depth",
        -1.0,
        1.0,
    ),
];
/// Appended separately: never insert fields into the released A/B/C/D loops.
pub const MOTION_FIELDS: [(Field, &str, &str, f32, f32); 6] = [
    (Field::SurfDepth, "surf_depth", "Surf depth", -1.0, 1.0),
    (Field::Step, "step", "frame stepping", 0.0, 1.0),
    (Field::KeyWaves, "key_waves", "Key to Waves", -1.0, 1.0),
    (Field::FreePhase, "free_phase", "free phase", 0.0, 1.0),
    (Field::Lfo, "lfo", "position LFO", 0.0, 8.0),
    (
        Field::LfoDepth,
        "lfo_depth",
        "position LFO depth",
        -1.0,
        1.0,
    ),
];
pub const FM_FIELDS: [(Field, &str, &str, f32, f32); 2] = [
    (Field::FmMode, "fm_mode", "modulation mode", 0.0, 2.0),
    (Field::FmIndex, "fm_index", "modulation index", 0.0, 4.0),
];
pub const SYNC_FIELDS: [(Field, &str, &str, f32, f32); 2] = [
    (Field::Sync, "sync", "hard sync", 0.0, 1.0),
    (Field::SyncRatio, "sync_ratio", "sync slave ratio", 1.0, 8.0),
];
impl Default for Controls {
    fn default() -> Self {
        Self {
            enabled: true,
            position: 0.0,
            transpose: 0.0,
            fine: 0.0,
            level: 1.0,
            pan: 0.5,
            phase: 0.0,
            phase_random: 0.0,
            unison: 1,
            detune: 10.0,
            spread: 1.0,
            voice_shift: 0.0,
            position_env: 0,
            position_env_depth: 0.0,
            surf_depth: 1.0,
            step: false,
            key_waves: 0.0,
            free_phase: false,
            position_lfo: 0,
            position_lfo_depth: 0.0,
            fm_mode: 0,
            fm_index: 0.0,
            sync: false,
            sync_ratio: 1.0,
            fm_source: 4,
            sync_source: 4,
        }
    }
}
impl Controls {
    pub fn get(self, field: Field) -> f32 {
        match field {
            Field::Enabled => f32::from(self.enabled),
            Field::Position => self.position,
            Field::Transpose => self.transpose,
            Field::Fine => self.fine,
            Field::Level => self.level,
            Field::Pan => self.pan,
            Field::Phase => self.phase,
            Field::PhaseRandom => self.phase_random,
            Field::Unison => self.unison as f32,
            Field::Detune => self.detune,
            Field::Spread => self.spread,
            Field::Env => self.position_env as f32,
            Field::EnvDepth => self.position_env_depth,
            Field::SurfDepth => self.surf_depth,
            Field::Step => f32::from(self.step),
            Field::KeyWaves => self.key_waves,
            Field::FreePhase => f32::from(self.free_phase),
            Field::Lfo => self.position_lfo as f32,
            Field::LfoDepth => self.position_lfo_depth,
            Field::FmMode => self.fm_mode as f32,
            Field::FmIndex => self.fm_index,
            Field::Sync => f32::from(self.sync),
            Field::SyncRatio => self.sync_ratio,
            Field::VoiceShift => self.voice_shift,
            Field::FmSource => self.fm_source as f32,
            Field::SyncSource => self.sync_source as f32,
        }
    }
    pub fn set(&mut self, field: Field, value: f32) {
        if !value.is_finite() {
            return;
        }
        match field {
            Field::Enabled => self.enabled = value >= 0.5,
            Field::Position => self.position = value,
            Field::Transpose => self.transpose = value,
            Field::Fine => self.fine = value,
            Field::Level => self.level = value,
            Field::Pan => self.pan = value,
            Field::Phase => self.phase = value,
            Field::PhaseRandom => self.phase_random = value,
            Field::Unison => self.unison = value.round().clamp(1.0, 8.0) as u8,
            Field::Detune => self.detune = value,
            Field::Spread => self.spread = value,
            Field::Env => self.position_env = value.round().clamp(0.0, 8.0) as u8,
            Field::EnvDepth => self.position_env_depth = value,
            Field::SurfDepth => self.surf_depth = value,
            Field::Step => self.step = value >= 0.5,
            Field::KeyWaves => self.key_waves = value,
            Field::FreePhase => self.free_phase = value >= 0.5,
            Field::Lfo => self.position_lfo = value.round().clamp(0.0, 8.0) as u8,
            Field::LfoDepth => self.position_lfo_depth = value,
            Field::FmMode => self.fm_mode = value.round().clamp(0.0, 2.0) as u8,
            Field::FmIndex => self.fm_index = value,
            Field::Sync => self.sync = value >= 0.5,
            Field::SyncRatio => self.sync_ratio = value,
            Field::VoiceShift => self.voice_shift = value,
            Field::FmSource => self.fm_source = value.round().clamp(0.0, 3.0) as u8,
            Field::SyncSource => self.sync_source = value.round().clamp(0.0, 3.0) as u8,
        }
    }
    pub fn sanitize(&mut self) {
        use crate::preset::finite_clamp as clamp;
        clamp(&mut self.position, 0.0, 1.0, 0.0);
        clamp(&mut self.transpose, -48.0, 48.0, 0.0);
        clamp(&mut self.fine, -100.0, 100.0, 0.0);
        clamp(&mut self.level, 0.0, 1.0, 1.0);
        clamp(&mut self.pan, 0.0, 1.0, 0.5);
        clamp(&mut self.phase, 0.0, 1.0, 0.0);
        clamp(&mut self.phase_random, 0.0, 1.0, 0.0);
        clamp(&mut self.detune, 0.0, 100.0, 10.0);
        clamp(&mut self.spread, 0.0, 1.0, 1.0);
        clamp(&mut self.voice_shift, -1.0, 1.0, 0.0);
        clamp(&mut self.position_env_depth, -1.0, 1.0, 0.0);
        clamp(&mut self.surf_depth, -1.0, 1.0, 1.0);
        clamp(&mut self.key_waves, -1.0, 1.0, 0.0);
        clamp(&mut self.position_lfo_depth, -1.0, 1.0, 0.0);
        self.position_lfo = self.position_lfo.min(8);
        self.fm_mode = self.fm_mode.min(2);
        self.fm_source = self.fm_source.min(4);
        self.sync_source = self.sync_source.min(4);
        clamp(&mut self.fm_index, 0.0, 4.0, 0.0);
        clamp(&mut self.sync_ratio, 1.0, 8.0, 1.0);
        self.unison = self.unison.clamp(1, MAX_UNISON as u8);
        self.position_env = self.position_env.min(crate::preset::MODULATORS as u8);
    }
    pub fn resolve_sources(&mut self, index: usize) {
        if self.fm_source >= 4 {
            self.fm_source = (index ^ 1) as u8;
        }
        if self.sync_source >= 4 {
            self.sync_source = index as u8;
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Oscillator {
    pub asset_id: String,
    #[serde(flatten)]
    pub controls: Controls,
}
impl Default for Oscillator {
    fn default() -> Self {
        Self {
            asset_id: DEFAULT_ASSET.into(),
            controls: Controls::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Settings {
    pub schema_version: u32,
    pub engine_revision: u32,
    #[serde(with = "seed_codec")]
    pub seed: u64,
    pub count: u8,
    pub oscillators: [Oscillator; OSCILLATORS],
    pub motion: motion::Motion,
}

mod seed_codec {
    use serde::{Deserialize, Serialize};
    pub fn serialize<S: serde::Serializer>(seed: &u64, s: S) -> Result<S::Ok, S::Error> {
        seed.to_string().serialize(s)
    }
    pub fn deserialize<'de, D: serde::Deserializer<'de>>(d: D) -> Result<u64, D::Error> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Seed {
            Text(String),
            Number(u64),
        }
        match Seed::deserialize(d)? {
            Seed::Number(n) => Ok(n),
            Seed::Text(text) => text.parse().map_err(serde::de::Error::custom),
        }
    }
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            engine_revision: ENGINE_REVISION,
            seed: 0x5754_5341_4450_4554,
            count: 1,
            motion: motion::Motion::default(),
            oscillators: std::array::from_fn(|i| Oscillator {
                controls: default_controls()[i],
                ..Oscillator::default()
            }),
        }
    }
}
impl Settings {
    pub fn validate_version(&self) -> Result<(), String> {
        if self.schema_version != SCHEMA_VERSION || self.engine_revision != ENGINE_REVISION {
            return Err(format!(
                "Unsupported wavetable version {}/{}; update SAD PET to open this sound",
                self.schema_version, self.engine_revision
            ));
        }
        if !(1..=OSCILLATORS as u8).contains(&self.count) {
            return Err("Use one to four wavetable waves".into());
        }
        Ok(())
    }
    pub fn sanitize(&mut self) {
        self.motion.sanitize();
        self.count = self.count.clamp(1, OSCILLATORS as u8);
        for (i, oscillator) in self.oscillators.iter_mut().enumerate() {
            oscillator.controls.resolve_sources(i);
            oscillator.controls.sanitize();
            if oscillator.asset_id.is_empty() {
                oscillator.asset_id = DEFAULT_ASSET.into();
            }
        }
    }
    pub fn controls(&self) -> [Controls; OSCILLATORS] {
        std::array::from_fn(|i| {
            let mut c = self.oscillators[i].controls;
            c.resolve_sources(i);
            c.enabled &= i < self.count as usize;
            c
        })
    }
    pub fn edit_layers(&mut self, action: &str, index: Option<usize>) -> Result<(), String> {
        match action {
            "add" if (self.count as usize) < OSCILLATORS => {
                self.oscillators[self.count as usize] = Oscillator::default();
                self.oscillators[self.count as usize]
                    .controls
                    .resolve_sources(self.count as usize);
                self.oscillators[self.count as usize].controls.level = 0.65;
                self.count += 1;
            }
            "add" => return Err("A Wavetable module can hold at most four waves".into()),
            "remove" => {
                if self.count <= 1 {
                    return Err("Keep at least one wave in Wavetable".into());
                }
                let index = index
                    .filter(|i| *i < self.count as usize)
                    .ok_or("Choose the wave to remove")?;
                self.oscillators[index..self.count as usize].rotate_left(1);
                self.count -= 1;
                for i in 0..self.count as usize {
                    let c = &mut self.oscillators[i].controls;
                    if c.fm_source as usize == index {
                        c.fm_mode = 0;
                        c.fm_source = (i ^ 1) as u8;
                    } else if c.fm_source as usize > index && c.fm_source < 4 {
                        c.fm_source -= 1;
                    }
                    if c.sync_source as usize == index {
                        c.sync = false;
                        c.sync_source = i as u8;
                    } else if c.sync_source as usize > index && c.sync_source < 4 {
                        c.sync_source -= 1;
                    }
                }
                self.oscillators[self.count as usize] = Oscillator {
                    controls: Controls {
                        enabled: false,
                        ..default_controls()[self.count as usize]
                    },
                    ..Default::default()
                };
            }
            _ => return Err("Choose Add or Remove wave".into()),
        }
        Ok(())
    }
}

impl<'de> Deserialize<'de> for Settings {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(default)]
        struct Data {
            schema_version: u32,
            engine_revision: u32,
            #[serde(with = "seed_codec")]
            seed: u64,
            count: Option<u8>,
            oscillators: Option<Vec<Oscillator>>,
            motion: motion::Motion,
        }
        impl Default for Data {
            fn default() -> Self {
                let s = Settings::default();
                Self {
                    schema_version: s.schema_version,
                    engine_revision: s.engine_revision,
                    seed: s.seed,
                    count: None,
                    oscillators: None,
                    motion: motion::Motion::default(),
                }
            }
        }
        let d = Data::deserialize(d)?;
        let mut settings = Settings {
            schema_version: d.schema_version,
            engine_revision: d.engine_revision,
            seed: d.seed,
            motion: d.motion,
            ..Settings::default()
        };
        if let Some(oscillators) = d.oscillators {
            if oscillators.is_empty() || oscillators.len() > OSCILLATORS {
                return Err(serde::de::Error::custom("Use one to four wavetable waves"));
            }
            let count = d.count.unwrap_or(oscillators.len() as u8);
            if count == 0 || count as usize > oscillators.len() {
                return Err(serde::de::Error::custom("Missing wavetable wave settings"));
            }
            if d.schema_version == 1 && oscillators.len() > 2 {
                return Err(serde::de::Error::custom(
                    "Wavetable version 1 supports at most two waves",
                ));
            }
            settings.count = count;
            for (i, oscillator) in oscillators.into_iter().enumerate() {
                settings.oscillators[i] = oscillator;
            }
        } else {
            settings.count = d.count.unwrap_or(1);
        }
        if settings.count == 0 || settings.count as usize > OSCILLATORS {
            return Err(serde::de::Error::custom("Use one to four wavetable waves"));
        }
        for (i, o) in settings.oscillators.iter_mut().enumerate() {
            o.controls.resolve_sources(i);
        }
        if matches!(settings.engine_revision, 1 | 2) {
            settings.engine_revision = ENGINE_REVISION;
        }
        if settings.schema_version == 1 {
            settings.schema_version = SCHEMA_VERSION;
        }
        Ok(settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_to_four_waves_and_legacy_ab_patches_keep_their_settings() {
        let mut s = Settings::default();
        assert_eq!(s.count, 1);
        for _ in 0..3 {
            s.edit_layers("add", None).unwrap();
        }
        assert_eq!(s.count, 4);
        assert!(s.controls().iter().all(|c| c.enabled));
        assert!(s.edit_layers("add", None).is_err());
        s.oscillators[2].controls.transpose = 7.0;
        s.edit_layers("remove", Some(1)).unwrap();
        assert_eq!(s.count, 3);
        assert_eq!(s.oscillators[1].controls.transpose, 7.0);
        assert!(!s.controls()[3].enabled);
        let restored: Settings = serde_json::from_slice(&serde_json::to_vec(&s).unwrap()).unwrap();
        assert_eq!(s, restored);
        let mut old = serde_json::to_value(&s).unwrap();
        old["schema_version"] = 1.into();
        old.as_object_mut().unwrap().remove("count");
        old["oscillators"].as_array_mut().unwrap().truncate(2);
        let migrated: Settings = serde_json::from_value(old).unwrap();
        assert_eq!(migrated.count, 2);
        assert_eq!(migrated.schema_version, 2);
        assert_eq!(migrated.oscillators[..2], s.oscillators[..2]);
        let mut invalid = serde_json::to_value(&s).unwrap();
        invalid["count"] = 5.into();
        assert!(serde_json::from_value::<Settings>(invalid).is_err());
    }
}
