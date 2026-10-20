use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FilterType {
    #[default]
    LowPass,
    BandPass,
    HighPass,
    Comb,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FilterRouting {
    #[default]
    Single,
    Serial,
    Parallel,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EnvSource {
    Env1,
    #[default]
    Env2,
    Env3,
}
impl EnvSource {
    pub fn index(self) -> usize {
        match self {
            Self::Env1 => 0,
            Self::Env2 => 1,
            Self::Env3 => 2,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SaturatorType {
    #[default]
    Distortion,
    Reducer,
    Crusher,
    Flanger,
    Chorus,
    Binaural,
    Reverb,
    LcrDelay,
    ModDelay,
    Compressor,
    Waveshaper,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReverbType {
    #[default]
    PlateS,
    PlateL,
    Room,
    Reverse,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Waveform {
    #[default]
    Sine,
    Triangle,
    Saw,
    Square,
    Random,
    SmoothRandom,
    Envelope,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Destination {
    #[default]
    None,
    Position,
    Pitch,
    GrainSize,
    Grains,
    Filter,
    Mix,
    Pan,
    Reverb,
    GrainShape,
    Detune,
    Cloud,
    StereoSpread,
    FilterResonance,
    Filter2,
    Filter2Resonance,
    Saturation,
    ReverbDecay,
    ReverbTone,
    Predelay,
    Shimmer,
    Diffuse,
    Volume,
    Color {
        slot: u32,
        parameter: crate::color::ColorParameter,
    },
    Parameter {
        parameter: crate::matrix::Parameter,
    },
    Env {
        index: u8,
        parameter: crate::matrix::EnvParameter,
    },
    Lfo {
        index: u8,
        parameter: crate::matrix::LfoParameter,
    },
    Performance {
        parameter: crate::matrix::PerformanceParameter,
    },
    Step {
        index: u8,
        parameter: crate::matrix::StepParameter,
    },
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Lfo {
    pub waveform: Waveform,
    pub rate_hz: f32,
    pub depth: f32,
    pub destination: Destination,
    pub sync: bool,
    pub division: f32,
}
impl Default for Lfo {
    fn default() -> Self {
        Self {
            waveform: Waveform::Sine,
            rate_hz: 0.2,
            depth: 0.0,
            destination: Destination::None,
            sync: false,
            division: 1.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Env {
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
    pub destination: Destination,
    pub depth: f32,
    pub extra_destination: Destination,
    pub extra_depth: f32,
}
impl Default for Env {
    fn default() -> Self {
        Self {
            attack: 0.04,
            decay: 0.2,
            sustain: 0.8,
            release: 0.3,
            destination: Destination::None,
            depth: 0.0,
            extra_destination: Destination::None,
            extra_depth: 0.0,
        }
    }
}
impl Env {
    pub fn attack_seconds(&self) -> f32 {
        0.002 + self.attack.powi(3) * 8.0
    }
    pub fn decay_seconds(&self) -> f32 {
        0.005 + self.decay.powi(3) * 8.0
    }
    pub fn release_seconds(&self) -> f32 {
        0.005 + self.release.powi(3) * 12.0
    }
    fn sanitize(&mut self) {
        let fallback = Self::default();
        finite_clamp(&mut self.attack, 0.0, 1.0, fallback.attack);
        finite_clamp(&mut self.decay, 0.0, 1.0, fallback.decay);
        finite_clamp(&mut self.sustain, 0.0, 1.0, fallback.sustain);
        finite_clamp(&mut self.release, 0.0, 1.0, fallback.release);
        finite_clamp(&mut self.depth, -1.0, 1.0, 0.0);
        finite_clamp(&mut self.extra_depth, -1.0, 1.0, 0.0);
    }
}
fn default_envs() -> [Env; 3] {
    [
        Env::default(),
        Env {
            attack: 0.12,
            decay: 0.35,
            sustain: 0.35,
            release: 0.3,
            ..Env::default()
        },
        Env {
            attack: 0.3,
            decay: 0.3,
            sustain: 0.5,
            release: 0.4,
            ..Env::default()
        },
    ]
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Params {
    pub grain_size: f32,
    pub grain_shape: f32,
    pub position: f32,
    pub grains: f32,
    pub pitch: f32,
    pub detune: f32,
    pub cloud: f32,
    pub stereo_spread: f32,
    pub mix: f32,
    pub filter: f32,
    pub filter_resonance: f32,
    pub filter_env_depth: f32,
    pub filter_keytrack: f32,
    pub filter_env: EnvSource,
    pub filter2: f32,
    pub filter2_resonance: f32,
    pub filter2_env_depth: f32,
    pub filter2_keytrack: f32,
    pub filter2_env: EnvSource,
    pub filter_routing: FilterRouting,
    pub saturation: f32,
    pub reverb: f32,
    pub reverb_decay: f32,
    pub reverb_tone: f32,
    pub predelay: f32,
    pub shimmer: f32,
    pub diffuse: f32,
    pub adsr_attack: f32,
    pub adsr_decay: f32,
    pub adsr_sustain: f32,
    pub adsr_release: f32,
    pub master_volume: f32,
    pub in_gain: f32,
    pub out_gain: f32,
    pub pitch_range: f32,
    pub sample_tune_semis: f32,
    pub sample_tune_cents: f32,
    pub midi_pitchbend_range: f32,
    pub min_grains: f32,
    pub bpm: f32,
    pub loop_one_bar: bool,
    pub sample_root_note: f32,
    pub antialiasing: bool,
    pub vcf_type: FilterType,
    pub vcf2_type: FilterType,
    pub saturator_type: SaturatorType,
    pub reverb_type: ReverbType,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            grain_size: 0.69,
            grain_shape: 0.9,
            position: 0.0,
            grains: 0.48,
            pitch: 0.5,
            detune: 0.5,
            cloud: 0.12,
            stereo_spread: 0.7,
            mix: 1.0,
            filter: 1.0,
            filter_resonance: 0.12,
            filter_env_depth: 0.35,
            filter_keytrack: 0.0,
            filter_env: EnvSource::Env2,
            filter2: 1.0,
            filter2_resonance: 0.12,
            filter2_env_depth: 0.35,
            filter2_keytrack: 0.0,
            filter2_env: EnvSource::Env2,
            filter_routing: FilterRouting::Single,
            saturation: 0.0,
            reverb: 0.22,
            reverb_decay: 0.45,
            reverb_tone: 0.65,
            predelay: 0.0,
            shimmer: 0.0,
            diffuse: 0.5,
            adsr_attack: 0.04,
            adsr_decay: 0.2,
            adsr_sustain: 0.8,
            adsr_release: 0.3,
            master_volume: 0.65,
            in_gain: 0.0,
            out_gain: 0.0,
            pitch_range: 36.0,
            sample_tune_semis: 0.0,
            sample_tune_cents: 0.0,
            midi_pitchbend_range: 2.0,
            min_grains: 0.0,
            bpm: 120.0,
            loop_one_bar: false,
            sample_root_note: 60.0,
            antialiasing: true,
            vcf_type: FilterType::LowPass,
            vcf2_type: FilterType::LowPass,
            saturator_type: SaturatorType::Distortion,
            reverb_type: ReverbType::PlateS,
        }
    }
}

pub(crate) fn finite_clamp(value: &mut f32, min: f32, max: f32, fallback: f32) {
    *value = if value.is_finite() {
        value.clamp(min, max)
    } else {
        fallback
    };
}

impl Params {
    pub fn sanitize(&mut self) {
        macro_rules! unit { ($($f:ident),*) => { $(finite_clamp(&mut self.$f, 0.0, 1.0, Self::default().$f);)* }; }
        unit!(
            grain_size,
            grain_shape,
            position,
            grains,
            pitch,
            detune,
            cloud,
            stereo_spread,
            mix,
            filter,
            filter_resonance,
            filter_keytrack,
            filter2,
            filter2_resonance,
            filter2_keytrack,
            saturation,
            reverb,
            reverb_decay,
            reverb_tone,
            predelay,
            shimmer,
            diffuse,
            adsr_attack,
            adsr_decay,
            adsr_sustain,
            adsr_release,
            master_volume
        );
        finite_clamp(&mut self.filter_env_depth, -1.0, 1.0, 0.0);
        finite_clamp(&mut self.filter2_env_depth, -1.0, 1.0, 0.0);
        finite_clamp(&mut self.in_gain, -20.0, 20.0, 0.0);
        finite_clamp(&mut self.out_gain, -12.0, 12.0, 0.0);
        finite_clamp(&mut self.pitch_range, 12.0, 48.0, 36.0);
        finite_clamp(&mut self.sample_tune_semis, -48.0, 48.0, 0.0);
        finite_clamp(&mut self.sample_tune_cents, -100.0, 100.0, 0.0);
        finite_clamp(&mut self.midi_pitchbend_range, 0.0, 48.0, 2.0);
        finite_clamp(&mut self.min_grains, 0.0, 1.0, 0.0);
        finite_clamp(&mut self.bpm, 20.0, 300.0, 120.0);
        finite_clamp(&mut self.sample_root_note, 0.0, 127.0, 60.0);
        self.sample_root_note = self.sample_root_note.round();
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Preset {
    pub format: String,
    pub name: String,
    pub params: Params,
    pub lfos: [Lfo; 3],
    pub envs: [Env; 3],
    pub performance: crate::performance::Settings,
    pub resynth: crate::resynth::Settings,
    pub pre_color_chain: Vec<crate::color::ColorSlot>,
    pub source_color_chain: Vec<crate::color::ColorSlot>,
    pub color_chain: Vec<crate::color::ColorSlot>,
    pub fx_chain_version: u8,
    pub matrix: Vec<crate::matrix::Route>,
}
impl Default for Preset {
    fn default() -> Self {
        Self {
            // The format identifier stays stable for existing presets.
            format: "evil-pet-rust-v1".into(),
            name: "Init".into(),
            params: Params::default(),
            lfos: std::array::from_fn(|_| Lfo::default()),
            envs: default_envs(),
            performance: crate::performance::Settings::default(),
            resynth: crate::resynth::Settings::default(),
            pre_color_chain: Vec::new(),
            source_color_chain: Vec::new(),
            color_chain: vec![
                crate::color::ColorSlot::legacy(SaturatorType::Distortion, 0.0),
                crate::color::ColorSlot::legacy_reverb(2, &Params::default()),
            ],
            fx_chain_version: 1,
            matrix: Vec::new(),
        }
    }
}
impl<'de> Deserialize<'de> for Preset {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(default)]
        struct Data {
            format: String,
            name: String,
            params: Option<serde_json::Value>,
            lfos: [Lfo; 3],
            envs: Option<[Env; 3]>,
            performance: crate::performance::Settings,
            resynth: crate::resynth::Settings,
            pre_color_chain: Vec<crate::color::ColorSlot>,
            source_color_chain: Vec<crate::color::ColorSlot>,
            color_chain: Option<Vec<crate::color::ColorSlot>>,
            fx_chain_version: u8,
            matrix: Vec<crate::matrix::Route>,
        }
        impl Default for Data {
            fn default() -> Self {
                let p = Preset::default();
                Self {
                    format: p.format,
                    name: p.name,
                    params: None,
                    lfos: p.lfos,
                    envs: None,
                    performance: p.performance,
                    resynth: p.resynth,
                    pre_color_chain: Vec::new(),
                    source_color_chain: Vec::new(),
                    color_chain: None,
                    fx_chain_version: 0,
                    matrix: Vec::new(),
                }
            }
        }
        let d = Data::deserialize(deserializer)?;
        let legacy_filter = d.params.as_ref().is_some_and(|p| {
            p.get("filter_routing").is_none() && p.get("filter_env_depth").is_none()
        });
        let mut params: Params = match d.params {
            Some(value) => serde_json::from_value(value).map_err(serde::de::Error::custom)?,
            None => Params::default(),
        };
        let had_envs = d.envs.is_some();
        let mut envs = d.envs.unwrap_or_else(|| {
            let mut envs = default_envs();
            envs[0].attack = params.adsr_attack;
            envs[0].decay = params.adsr_decay;
            envs[0].sustain = params.adsr_sustain;
            envs[0].release = params.adsr_release;
            envs
        });
        if legacy_filter {
            // Move the old default ENV 2 → filter route into its dedicated control.
            // Presets predating envelopes keep an unmodulated filter.
            params.filter_env_depth = 0.0;
            if had_envs && envs[1].destination == Destination::Filter {
                params.filter_env_depth = envs[1].depth;
                envs[1].destination = Destination::None;
                envs[1].depth = 0.0;
            }
        }
        let mut color_chain = d.color_chain.unwrap_or_else(|| {
            vec![crate::color::ColorSlot::legacy(
                params.saturator_type,
                params.saturation,
            )]
        });
        let mut lfos = d.lfos;
        let mut matrix = d.matrix;
        if d.fx_chain_version == 0 && !color_chain.iter().any(|s| s.kind == SaturatorType::Reverb) {
            // v0.6 and earlier always process the shared reverb after the Color rack.
            color_chain.truncate(crate::color::MAX_COLOR_SLOTS - 1);
            let referenced = lfos
                .iter()
                .map(|l| l.destination)
                .chain(
                    envs.iter()
                        .flat_map(|e| [e.destination, e.extra_destination]),
                )
                .chain(matrix.iter().map(|r| r.destination));
            let mut reserved: Vec<_> = color_chain
                .iter()
                .chain(&d.pre_color_chain)
                .chain(&d.source_color_chain)
                .map(|s| s.id)
                .collect();
            reserved.extend(referenced.filter_map(|d| {
                if let Destination::Color { slot, .. } = d {
                    Some(slot)
                } else {
                    None
                }
            }));
            let id = (1..).find(|id| !reserved.contains(id)).unwrap();
            color_chain.push(crate::color::ColorSlot::legacy_reverb(id, &params));
            let migrate = |destination: &mut Destination| {
                if let Some(parameter) = crate::color::legacy_reverb_parameter(*destination) {
                    *destination = Destination::Color {
                        slot: id,
                        parameter,
                    };
                }
            };
            for lfo in &mut lfos {
                migrate(&mut lfo.destination);
            }
            for env in &mut envs {
                migrate(&mut env.destination);
                migrate(&mut env.extra_destination);
            }
            for route in &mut matrix {
                migrate(&mut route.destination);
            }
        }
        Ok(Self {
            format: d.format,
            name: d.name,
            params,
            lfos,
            envs,
            performance: d.performance,
            resynth: d.resynth,
            pre_color_chain: d.pre_color_chain,
            source_color_chain: d.source_color_chain,
            color_chain,
            fx_chain_version: 1,
            matrix,
        })
    }
}
impl Preset {
    pub fn sanitize(&mut self) {
        self.fx_chain_version = 1;
        self.params.sanitize();
        self.resynth.sanitize();
        self.matrix.truncate(crate::matrix::MAX_ROUTES);
        self.color_chain.truncate(crate::color::MAX_COLOR_SLOTS);
        self.pre_color_chain.truncate(crate::color::MAX_COLOR_SLOTS);
        self.source_color_chain.truncate(1);
        let mut reserved: Vec<u32> = self
            .color_chain
            .iter()
            .chain(&self.pre_color_chain)
            .chain(&self.source_color_chain)
            .map(|s| s.id)
            .collect();
        reserved.extend(
            self.lfos
                .iter()
                .map(|l| l.destination)
                .chain(
                    self.envs
                        .iter()
                        .flat_map(|e| [e.destination, e.extra_destination]),
                )
                .chain(self.matrix.iter().map(|r| r.destination))
                .filter_map(|d| {
                    if let Destination::Color { slot, .. } = d {
                        Some(slot)
                    } else {
                        None
                    }
                }),
        );
        let mut used = Vec::new();
        for slot in self
            .color_chain
            .iter_mut()
            .chain(&mut self.pre_color_chain)
            .chain(&mut self.source_color_chain)
        {
            if slot.id == 0 || used.contains(&slot.id) {
                slot.id = (1..).find(|id| !reserved.contains(id)).unwrap();
                reserved.push(slot.id);
            }
            used.push(slot.id);
            slot.sanitize();
        }
        self.params.saturation = self.color_chain.first().map_or(0.0, |s| s.mix);
        self.params.saturator_type = self
            .color_chain
            .first()
            .map_or(SaturatorType::Distortion, |s| s.kind);
        crate::color::mirror_reverb(&mut self.params, &self.color_chain);
        // Source FX for files is prepared off-thread, so it cannot be an
        // audio-rate modulation destination. Host automation uses its own IDs.
        let source_id = self.source_color_chain.first().map(|slot| slot.id);
        let validate = |route: &mut Destination| {
            if !crate::matrix::valid(*route) {
                *route = Destination::None;
            }
            if let Destination::Color { slot, .. } = *route
                && (!used.contains(&slot) || source_id == Some(slot))
            {
                *route = Destination::None;
            }
        };
        for route in &mut self.matrix {
            route.sanitize();
            validate(&mut route.destination);
        }
        for env in &mut self.envs {
            env.sanitize();
            validate(&mut env.destination);
            validate(&mut env.extra_destination);
        }
        self.params.adsr_attack = self.envs[0].attack;
        self.params.adsr_decay = self.envs[0].decay;
        self.params.adsr_sustain = self.envs[0].sustain;
        self.params.adsr_release = self.envs[0].release;
        self.performance.sanitize();
        // Hold/arp MIDI capture must never grow a vector on the audio thread.
        self.performance
            .chord
            .reserve(32 - self.performance.chord.len());
        self.name = self.name.chars().take(80).collect();
        for lfo in &mut self.lfos {
            validate(&mut lfo.destination);
            finite_clamp(&mut lfo.rate_hz, 0.01, 40.0, 0.2);
            finite_clamp(&mut lfo.depth, -1.0, 1.0, 0.0);
            finite_clamp(&mut lfo.division, 0.0625, 32.0, 1.0);
        }
    }

    pub fn builtin(index: usize) -> Self {
        let mut p = Self::default();
        match index {
            1 => {
                p.name = "Halo".into();
                p.params.grain_size = 0.86;
                p.params.grains = 0.6;
                p.params.pitch = 2.0 / 3.0;
                p.params.reverb = 0.62;
                p.params.shimmer = 0.35;
                p.params.reverb_decay = 0.8;
                p.params.filter = 0.82;
                p.lfos[0] = Lfo {
                    destination: Destination::Position,
                    rate_hz: 0.07,
                    depth: 0.32,
                    ..Lfo::default()
                };
            }
            2 => {
                p.name = "Reverse glass".into();
                p.params.grain_size = 0.14;
                p.params.pitch = 1.0 / 3.0;
                p.params.cloud = 0.3;
                p.params.reverb = 0.42;
                p.params.reverb_type = ReverbType::Reverse;
                p.params.grains = 0.46;
                p.params.stereo_spread = 1.0;
            }
            3 => {
                p.name = "Low orbit".into();
                p.params.grain_size = 0.79;
                p.params.pitch = 1.0 / 6.0;
                p.params.filter = 0.56;
                p.params.filter_resonance = 0.55;
                p.params.saturation = 0.22;
                p.params.reverb = 0.36;
                p.params.grains = 0.6;
                p.lfos[0] = Lfo {
                    destination: Destination::Filter,
                    waveform: Waveform::Triangle,
                    rate_hz: 0.11,
                    depth: 0.2,
                    ..Lfo::default()
                };
            }
            4 => {
                p.name = "Broken tape".into();
                p.params.grain_size = 0.62;
                p.params.detune = 0.7;
                p.params.saturation = 0.42;
                p.params.saturator_type = SaturatorType::Chorus;
                p.params.grain_shape = 0.3;
                p.params.grains = 0.4;
                p.params.reverb = 0.16;
                p.lfos[0] = Lfo {
                    destination: Destination::Pitch,
                    waveform: Waveform::SmoothRandom,
                    rate_hz: 2.1,
                    depth: 0.018,
                    ..Lfo::default()
                };
                p.lfos[1] = Lfo {
                    destination: Destination::Position,
                    rate_hz: 0.3,
                    depth: 0.15,
                    ..Lfo::default()
                };
            }
            _ => {}
        }
        p.color_chain = vec![
            crate::color::ColorSlot::legacy(p.params.saturator_type, p.params.saturation),
            crate::color::ColorSlot::legacy_reverb(2, &p.params),
        ];
        p
    }
}
