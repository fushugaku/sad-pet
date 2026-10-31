use serde::{Deserialize, Serialize};

pub const MODULATORS: usize = 8;

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FilterType {
    #[default]
    LowPass,
    BandPass,
    HighPass,
    Comb,
}
impl FilterType {
    pub const ALL: [Self; 4] = [Self::LowPass, Self::BandPass, Self::HighPass, Self::Comb];
}

/// Separate from the released four-value FilterType so old automation and
/// presets keep their original SVF/Comb sound. Rustias model order is stable.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FilterModel {
    #[default]
    Legacy,
    Radias,
    LadderLp24,
    Ms20Lp12,
    Ms20Hp12,
    ProLp24,
    Sem,
    MultiFilter,
    LadderHp24,
    LadderBp24,
    MPolyLp24,
    LinearLp12,
    LinearLp24,
    LinearHp12,
    LinearHp24,
    LinearBp12,
    LinearBp24,
    LinearNotch12,
    LinearNotch24,
    Radias2,
}
impl FilterModel {
    pub const ALL: [Self; 20] = [
        Self::Legacy,
        Self::Radias,
        Self::LadderLp24,
        Self::Ms20Lp12,
        Self::Ms20Hp12,
        Self::ProLp24,
        Self::Sem,
        Self::MultiFilter,
        Self::LadderHp24,
        Self::LadderBp24,
        Self::MPolyLp24,
        Self::LinearLp12,
        Self::LinearLp24,
        Self::LinearHp12,
        Self::LinearHp24,
        Self::LinearBp12,
        Self::LinearBp24,
        Self::LinearNotch12,
        Self::LinearNotch24,
        Self::Radias2,
    ];
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
    Tb303,
}
impl EnvSource {
    pub fn index(self) -> usize {
        match self {
            Self::Env1 => 0,
            Self::Env2 => 1,
            Self::Env3 => 2,
            Self::Tb303 => MODULATORS,
        }
    }
    pub fn value(self, envs: &[f32]) -> f32 {
        envs.get(self.index()).copied().unwrap_or(0.0)
    }
}

/// Filters 3 and up, added by the user after Filter 1 and 2.
pub const MAX_FILTER_SLOTS: usize = 16;

/// Where an added filter sits. Positions inside a note's path run one filter
/// per voice with its own envelope and key tracking; the others are shared and
/// follow the loudest voice's envelope and the last note.
/// Filters after a Resynth oscillator follow its sync/ring taps, so they shape
/// only what that oscillator adds to the output.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FilterPosition {
    AfterGrains,
    AfterOsc1,
    AfterOsc2,
    AfterOsc3,
    AfterOsc4,
    AfterResynth,
    BeforeFilters,
    #[default]
    AfterFilters,
    AfterFx,
}
impl FilterPosition {
    pub const ALL: [Self; 9] = [
        Self::AfterGrains,
        Self::AfterOsc1,
        Self::AfterOsc2,
        Self::AfterOsc3,
        Self::AfterOsc4,
        Self::AfterResynth,
        Self::BeforeFilters,
        Self::AfterFilters,
        Self::AfterFx,
    ];
    /// One filter per voice, with the note's envelope and key tracking.
    pub fn per_voice(self) -> bool {
        matches!(
            self,
            Self::AfterGrains | Self::BeforeFilters | Self::AfterFilters
        )
    }
    /// The Resynth oscillator (0..4) whose output this position follows.
    pub fn oscillator(self) -> Option<usize> {
        match self {
            Self::AfterOsc1 => Some(0),
            Self::AfterOsc2 => Some(1),
            Self::AfterOsc3 => Some(2),
            Self::AfterOsc4 => Some(3),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct FilterSlot {
    pub id: u32,
    pub enabled: bool,
    pub position: FilterPosition,
    pub model: FilterModel,
    pub kind: FilterType,
    pub cutoff: f32,
    pub resonance: f32,
    pub drive: f32,
    pub morph: f32,
    pub env: EnvSource,
    pub env_depth: f32,
    pub keytrack: f32,
    pub output: crate::taps::Tap,
}
impl Default for FilterSlot {
    fn default() -> Self {
        Self {
            id: 0,
            enabled: true,
            position: FilterPosition::default(),
            model: FilterModel::Legacy,
            kind: FilterType::LowPass,
            cutoff: 0.7,
            resonance: 0.12,
            drive: 0.0,
            morph: 0.0,
            env: EnvSource::default(),
            env_depth: 0.0,
            keytrack: 0.0,
            output: crate::taps::Tap::default(),
        }
    }
}
impl FilterSlot {
    pub fn sanitize(&mut self) {
        let d = Self::default();
        finite_clamp(&mut self.cutoff, 0.0, 1.0, d.cutoff);
        finite_clamp(&mut self.resonance, 0.0, 1.0, d.resonance);
        finite_clamp(&mut self.drive, 0.0, 1.0, d.drive);
        finite_clamp(&mut self.morph, 0.0, 1.0, d.morph);
        finite_clamp(&mut self.env_depth, -1.0, 1.0, d.env_depth);
        finite_clamp(&mut self.keytrack, 0.0, 1.0, d.keytrack);
        self.output.sanitize();
    }
    /// A modulatable control with its range.
    pub fn parameter_mut(&mut self, parameter: FilterSlotParameter) -> (&mut f32, f32, f32) {
        match parameter {
            FilterSlotParameter::Cutoff => (&mut self.cutoff, 0.0, 1.0),
            FilterSlotParameter::Resonance => (&mut self.resonance, 0.0, 1.0),
            FilterSlotParameter::Drive => (&mut self.drive, 0.0, 1.0),
            FilterSlotParameter::Morph => (&mut self.morph, 0.0, 1.0),
            FilterSlotParameter::EnvDepth => (&mut self.env_depth, -1.0, 1.0),
            FilterSlotParameter::Keytrack => (&mut self.keytrack, 0.0, 1.0),
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FilterSlotParameter {
    Cutoff,
    Resonance,
    Drive,
    Morph,
    EnvDepth,
    Keytrack,
}
impl FilterSlotParameter {
    pub const ALL: [Self; 6] = [
        Self::Cutoff,
        Self::Resonance,
        Self::Drive,
        Self::Morph,
        Self::EnvDepth,
        Self::Keytrack,
    ];
    pub fn key(self) -> &'static str {
        match self {
            Self::Cutoff => "cutoff",
            Self::Resonance => "resonance",
            Self::Drive => "drive",
            Self::Morph => "morph",
            Self::EnvDepth => "env_depth",
            Self::Keytrack => "keytrack",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
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
    SuperEcho,
    AlphaVerb,
    /// The MS-70CDR effect set, persisted as its `mooz_<name>` key.
    Mooz(sad_pet_mooz::Model),
}
impl SaturatorType {
    const BUILT_IN: [(Self, &'static str); 13] = [
        (Self::Distortion, "distortion"),
        (Self::Reducer, "reducer"),
        (Self::Crusher, "crusher"),
        (Self::Flanger, "flanger"),
        (Self::Chorus, "chorus"),
        (Self::Binaural, "binaural"),
        (Self::Reverb, "reverb"),
        (Self::LcrDelay, "lcr_delay"),
        (Self::ModDelay, "mod_delay"),
        (Self::Compressor, "compressor"),
        (Self::Waveshaper, "waveshaper"),
        (Self::SuperEcho, "super_echo"),
        (Self::AlphaVerb, "alpha_verb"),
    ];
    pub fn key(self) -> &'static str {
        match self {
            Self::Mooz(model) => model.key(),
            kind => {
                Self::BUILT_IN
                    .iter()
                    .find(|(k, _)| *k == kind)
                    .expect("built-in kind")
                    .1
            }
        }
    }
    pub fn from_key(key: &str) -> Option<Self> {
        Self::BUILT_IN
            .iter()
            .find(|(_, k)| *k == key)
            .map(|(kind, _)| *kind)
            .or_else(|| sad_pet_mooz::Model::from_key(key).map(Self::Mooz))
    }
    pub fn mooz(self) -> Option<sad_pet_mooz::Model> {
        match self {
            Self::Mooz(model) => Some(model),
            _ => None,
        }
    }
}
impl Serialize for SaturatorType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.key())
    }
}
impl<'de> Deserialize<'de> for SaturatorType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let key = std::borrow::Cow::<'de, str>::deserialize(deserializer)?;
        Self::from_key(&key).ok_or_else(|| {
            serde::de::Error::unknown_variant(&key, &["distortion", "…", "alpha_verb", "mooz_*"])
        })
    }
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
    /// Stable index in the append-only numeric parameter catalog.
    Control {
        index: u16,
    },
    /// An added filter, addressed by its stable slot id.
    FilterSlot {
        slot: u32,
        parameter: FilterSlotParameter,
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
    /// Beats per cycle under `sync`: a 1/16 note up to 32 bars.
    pub division: f32,
    /// Routes 2 and 3; route 1 is `destination` and `depth`.
    pub destination2: Destination,
    pub depth2: f32,
    pub destination3: Destination,
    pub depth3: f32,
    /// -1..=1, zero is the plain wave: Sine and Triangle symmetry, Square
    /// pulse width, and the curve of the others.
    pub shape: f32,
    /// Start of the cycle, 0..1 of a turn. Random waves and the follower
    /// have no phase.
    pub phase: f32,
    /// 0..=1: rounds the wave off over up to a quarter cycle.
    pub smooth: f32,
    /// Swing 0..1 instead of -1..1.
    pub unipolar: bool,
    pub reset: LfoReset,
    /// Play one cycle from each reset, then hold its last value.
    pub one_shot: bool,
    /// 0..=1 controls: silence after a reset, then the fade-in to full
    /// depth. `lfo_seconds` maps them.
    pub delay: f32,
    pub fade: f32,
    /// Per-note consumers use their own phase; common Rack controls remain mono.
    pub poly: bool,
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
            destination2: Destination::None,
            depth2: 0.0,
            destination3: Destination::None,
            depth3: 0.0,
            shape: 0.0,
            phase: 0.0,
            smooth: 0.0,
            unipolar: false,
            reset: LfoReset::Free,
            one_shot: false,
            delay: 0.0,
            fade: 0.0,
            poly: false,
        }
    }
}
/// What starts an LFO's cycle over. Free and Tempo LFOs restart their delay
/// and fade (and a one-shot cycle) on a note played after every key was let go.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LfoReset {
    /// Runs on its own; a synced one starts in place when the transport does.
    #[default]
    Free,
    /// Every note.
    Note,
    /// The song position: the same point of the cycle at the same beat.
    Tempo,
    /// Neuropolis steps routed to the LFO's reset.
    Sequencer,
}
/// Seconds of an LFO delay or fade control.
pub fn lfo_seconds(control: f32) -> f32 {
    control * control * control * 10.0
}
/// Each LFO and envelope routes to this many destinations of its own,
/// besides any matrix routes that use it as a source.
pub const MODULATOR_ROUTES: usize = 3;
impl Lfo {
    pub fn routes(&self) -> [(Destination, f32); MODULATOR_ROUTES] {
        [
            (self.destination, self.depth),
            (self.destination2, self.depth2),
            (self.destination3, self.depth3),
        ]
    }
    pub fn destinations_mut(&mut self) -> [&mut Destination; MODULATOR_ROUTES] {
        [
            &mut self.destination,
            &mut self.destination2,
            &mut self.destination3,
        ]
    }
}

/// Envelope generator type. `Legacy` is the original SAD PET ADSR; the others
/// are the Rustias EG types (RADIAS controller ADSR and the Q modes).
/// Host values index `ALL`, so the order is stable.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EnvMode {
    #[default]
    Legacy,
    Radias,
    QAdsr,
    QAds1ds2r,
    QOneShot,
    QLoopS1s2,
    QLoopAll,
}
impl EnvMode {
    pub const ALL: [Self; 7] = [
        Self::Legacy,
        Self::Radias,
        Self::QAdsr,
        Self::QAds1ds2r,
        Self::QOneShot,
        Self::QLoopS1s2,
        Self::QLoopAll,
    ];
    /// Rustias `q_envelope::Settings::mode`: 0 RADIAS, 1..=5 the Q modes.
    pub fn rustias_mode(self) -> Option<u8> {
        match self {
            Self::Legacy => None,
            mode => Some(mode as u8 - 1),
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
    /// Presets without a type keep the original SAD PET envelope.
    pub mode: EnvMode,
    /// Q ADS1DS2R and loop modes: Attack level, Decay 2 time, Sustain 2 level.
    pub attack_level: f32,
    pub decay2: f32,
    pub sustain2: f32,
    /// RADIAS decay/release curve, 0..=7.
    pub curve: u8,
    /// Route 3; routes 1 and 2 are `destination` and `extra_destination`.
    pub destination3: Destination,
    pub depth3: f32,
    #[serde(skip_serializing_if = "crate::envelope::Wave::is_default")]
    pub wave: crate::envelope::Wave,
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
            mode: EnvMode::Legacy,
            // Rustias defaults: 127, 64, 64 and curve 1.
            attack_level: 1.0,
            decay2: 0.5,
            sustain2: 0.5,
            curve: 1,
            destination3: Destination::None,
            depth3: 0.0,
            wave: crate::envelope::Wave::default(),
        }
    }
}
impl Env {
    pub fn routes(&self) -> [(Destination, f32); MODULATOR_ROUTES] {
        [
            (self.destination, self.depth),
            (self.extra_destination, self.extra_depth),
            (self.destination3, self.depth3),
        ]
    }
    pub fn destinations_mut(&mut self) -> [&mut Destination; MODULATOR_ROUTES] {
        [
            &mut self.destination,
            &mut self.extra_destination,
            &mut self.destination3,
        ]
    }
    /// A 0..1 control as the 0..=127 value the Rustias EG types read.
    pub fn rustias_control(value: f32) -> u8 {
        (value.clamp(0.0, 1.0) * 127.0).round() as u8
    }
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
        finite_clamp(&mut self.depth3, -1.0, 1.0, 0.0);
        finite_clamp(&mut self.attack_level, 0.0, 1.0, fallback.attack_level);
        finite_clamp(&mut self.decay2, 0.0, 1.0, fallback.decay2);
        finite_clamp(&mut self.sustain2, 0.0, 1.0, fallback.sustain2);
        self.curve = self.curve.min(7);
        self.wave.sanitize();
    }
}
fn default_envs() -> [Env; crate::preset::MODULATORS] {
    std::array::from_fn(|i| match i {
        1 => Env {
            attack: 0.12,
            decay: 0.35,
            sustain: 0.35,
            release: 0.3,
            ..Env::default()
        },
        2 => Env {
            attack: 0.3,
            decay: 0.3,
            sustain: 0.5,
            release: 0.4,
            ..Env::default()
        },
        _ => Env::default(),
    })
}

/// The span CLOUD at 100% scatters grains over in Time mode; the knob
/// follows a square law, so 50% is a quarter of it.
pub const CLOUD_SECONDS: f32 = 4.0;

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
    pub filter_model: FilterModel,
    pub filter_drive: f32,
    pub filter_morph: f32,
    pub filter_env_depth: f32,
    pub filter_keytrack: f32,
    pub filter_env: EnvSource,
    pub filter2: f32,
    pub filter2_resonance: f32,
    pub filter2_model: FilterModel,
    pub filter2_drive: f32,
    pub filter2_morph: f32,
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
    /// How much of the grains' own output they read again.
    pub grain_feedback: f32,
    /// How much of the grain cloud's build-up a note waits through: at 1 its
    /// grains start from none, at 0 it starts with a played-in cloud.
    pub grain_attack: f32,
    /// CLOUD scatters grains over a span of time, up to `CLOUD_SECONDS`,
    /// rather than over a share of the whole source.
    pub cloud_time: bool,
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
            filter_model: FilterModel::Legacy,
            filter_drive: 0.0,
            filter_morph: 0.0,
            filter_env_depth: 0.35,
            filter_keytrack: 0.0,
            filter_env: EnvSource::Env2,
            filter2: 1.0,
            filter2_resonance: 0.12,
            filter2_model: FilterModel::Legacy,
            filter2_drive: 0.0,
            filter2_morph: 0.0,
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
            grain_feedback: 0.0,
            grain_attack: 1.0,
            cloud_time: false,
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
            filter_drive,
            filter_morph,
            filter_keytrack,
            filter2,
            filter2_resonance,
            filter2_drive,
            filter2_morph,
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
            master_volume,
            grain_feedback,
            grain_attack
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
    pub lfos: [Lfo; crate::preset::MODULATORS],
    pub envs: [Env; crate::preset::MODULATORS],
    pub lfo_count: u8,
    pub env_count: u8,
    pub performance: crate::performance::Settings,
    pub sequencer: crate::sequence::Program,
    pub resynth: crate::resynth::Settings,
    /// Where each section's output goes: on down the chain and to a Bus.
    #[serde(skip_serializing_if = "crate::taps::Taps::is_default")]
    pub taps: crate::taps::Taps,
    /// Advanced grain mode and its groups.
    #[serde(skip_serializing_if = "crate::grain_groups::Settings::is_default")]
    pub grain_groups: crate::grain_groups::Settings,
    pub drums: Option<Box<crate::drums::Settings>>,
    #[serde(skip)]
    pub drum_runtime: crate::drums::Prepared,
    /// Present only in Synth engine timbres; other presets serialize unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synth: Option<Box<crate::synth::Settings>>,
    #[serde(skip)]
    pub synth_runtime: crate::dsp::synth::Prepared,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wavetable: Option<Box<crate::wavetable::Settings>>,
    #[serde(skip)]
    pub wavetable_runtime: crate::dsp::wavetable::Prepared,
    /// A control owner's canonical assets; portable host formats store their
    /// deduplicated manifest separately from numeric preset settings.
    #[serde(skip)]
    pub wavetable_pool: crate::wavetable::assets::Pool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tb303: Option<Box<crate::tb303::Settings>>,
    #[serde(skip)]
    pub tb303_runtime: crate::tb303::Prepared,
    /// Present only in Bus and Master timbres; other presets serialize unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bus: Option<Box<crate::bus::Settings>>,
    #[serde(skip)]
    pub bus_runtime: crate::bus::Prepared,
    /// Present in Vocal timbres, which also carry Bus settings: their engine
    /// processes the sung voice the way a bus processes its input; and in a
    /// Rack with a Vocal module.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocal: Option<Box<crate::vocal::Settings>>,
    /// Present in Sampler timbres, which also carry Bus settings: their engine
    /// processes the kit the way a bus processes its input; and in a Rack
    /// with a Sampler module.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampler: Option<Box<crate::sampler::Settings>>,
    /// A native host's kit runtime, built with the pool it holds.
    #[serde(skip)]
    pub sampler_runtime: crate::dsp::sampler::Prepared,
    /// Present only in Rack timbres, which may also hold Drum, Synth, Sampler
    /// and Vocal settings for their modules; other presets serialize unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rack: Option<Box<crate::rack::Settings>>,
    #[serde(skip)]
    pub rack_runtime: crate::rack::Prepared,
    pub pre_color_chain: Vec<crate::color::ColorSlot>,
    pub source_color_chain: Vec<crate::color::ColorSlot>,
    pub color_chain: Vec<crate::color::ColorSlot>,
    pub fx_chain_version: u8,
    #[serde(skip)]
    pub imported_effects: crate::imported_fx::PreparedEffects,
    pub matrix: Vec<crate::matrix::Route>,
    /// Knobs of the user's own, each turning several destinations at once.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub macros: Vec<crate::macros::Macro>,
    /// Filters 3 and up, in processing order within each position.
    pub filter_slots: Vec<FilterSlot>,
    #[serde(skip)]
    pub filter_banks: crate::dsp::filter_slots::Prepared,
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
            lfo_count: 3,
            env_count: 3,
            performance: crate::performance::Settings::default(),
            sequencer: crate::sequence::Program::default(),
            resynth: crate::resynth::Settings::default(),
            taps: Default::default(),
            grain_groups: Default::default(),
            drums: None,
            drum_runtime: Default::default(),
            synth: None,
            synth_runtime: Default::default(),
            wavetable: None,
            wavetable_runtime: Default::default(),
            wavetable_pool: Default::default(),
            tb303: None,
            tb303_runtime: Default::default(),
            bus: None,
            bus_runtime: Default::default(),
            vocal: None,
            sampler: None,
            sampler_runtime: Default::default(),
            rack: None,
            rack_runtime: Default::default(),
            pre_color_chain: Vec::new(),
            source_color_chain: Vec::new(),
            color_chain: vec![
                crate::color::ColorSlot::legacy(SaturatorType::Distortion, 0.0),
                crate::color::ColorSlot::legacy_reverb(2, &Params::default()),
            ],
            fx_chain_version: 1,
            imported_effects: Default::default(),
            matrix: Vec::new(),
            macros: Vec::new(),
            filter_slots: Vec::new(),
            filter_banks: Default::default(),
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
            lfos: Vec<Lfo>,
            envs: Option<Vec<Env>>,
            lfo_count: u8,
            env_count: u8,
            performance: crate::performance::Settings,
            sequencer: Option<crate::sequence::Program>,
            resynth: Option<serde_json::Value>,
            taps: Option<crate::taps::Taps>,
            grain_groups: crate::grain_groups::Settings,
            drums: Option<Box<crate::drums::Settings>>,
            synth: Option<Box<crate::synth::Settings>>,
            wavetable: Option<Box<crate::wavetable::Settings>>,
            tb303: Option<Box<crate::tb303::Settings>>,
            bus: Option<Box<crate::bus::Settings>>,
            vocal: Option<Box<crate::vocal::Settings>>,
            sampler: Option<Box<crate::sampler::Settings>>,
            rack: Option<Box<crate::rack::Settings>>,
            pre_color_chain: Vec<crate::color::ColorSlot>,
            source_color_chain: Vec<crate::color::ColorSlot>,
            color_chain: Option<Vec<crate::color::ColorSlot>>,
            fx_chain_version: u8,
            matrix: Vec<crate::matrix::Route>,
            macros: Vec<crate::macros::Macro>,
            filter_slots: Vec<FilterSlot>,
        }
        impl Default for Data {
            fn default() -> Self {
                let p = Preset::default();
                Self {
                    format: p.format,
                    name: p.name,
                    params: None,
                    lfos: p.lfos.to_vec(),
                    envs: None,
                    lfo_count: p.lfo_count,
                    env_count: p.env_count,
                    performance: p.performance,
                    sequencer: None,
                    resynth: None,
                    taps: None,
                    grain_groups: Default::default(),
                    drums: None,
                    synth: None,
                    wavetable: None,
                    tb303: None,
                    bus: None,
                    vocal: None,
                    sampler: None,
                    rack: None,
                    pre_color_chain: Vec::new(),
                    source_color_chain: Vec::new(),
                    color_chain: None,
                    fx_chain_version: 0,
                    matrix: Vec::new(),
                    macros: Vec::new(),
                    filter_slots: Vec::new(),
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
        let mut envs = d.envs.map_or_else(
            || {
                let mut envs = default_envs();
                envs[0].attack = params.adsr_attack;
                envs[0].decay = params.adsr_decay;
                envs[0].sustain = params.adsr_sustain;
                envs[0].release = params.adsr_release;
                envs
            },
            |values| {
                let mut envs = default_envs();
                for (to, from) in envs.iter_mut().zip(values) {
                    *to = from;
                }
                envs
            },
        );
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
        let mut lfos = [Lfo::default(); MODULATORS];
        for (to, from) in lfos.iter_mut().zip(d.lfos) {
            *to = from;
        }
        let mut matrix = d.matrix;
        if d.fx_chain_version == 0 && !color_chain.iter().any(|s| s.kind == SaturatorType::Reverb) {
            // v0.6 and earlier always process the shared reverb after the Color rack.
            color_chain.truncate(crate::color::MAX_COLOR_SLOTS - 1);
            let referenced = lfos
                .iter()
                .flat_map(|l| l.routes().map(|(d, _)| d))
                .chain(envs.iter().flat_map(|e| e.routes().map(|(d, _)| d)))
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
            for destination in lfos
                .iter_mut()
                .flat_map(Lfo::destinations_mut)
                .chain(envs.iter_mut().flat_map(Env::destinations_mut))
            {
                migrate(destination);
            }
            for route in &mut matrix {
                migrate(&mut route.destination);
            }
        }
        let mut taps = d.taps.unwrap_or_default();
        let resynth = match d.resynth {
            Some(value) => {
                // Earlier builds split a share of the wet Resynth off to a bus:
                // its timbre and bus shares become the Resynth tap's Chain/Send.
                let send = value.get("send").and_then(serde_json::Value::as_f64);
                let bus = value.get("bus").and_then(serde_json::Value::as_u64);
                if let (None, Some(send), Some(bus @ 1..=8)) = (d.taps, send, bus)
                    && send > 0.0
                {
                    taps.resynth = crate::taps::Tap {
                        chain: 1.0 - send as f32,
                        send: send as f32,
                        bus: bus as u8,
                    };
                }
                serde_json::from_value(value).map_err(serde::de::Error::custom)?
            }
            None => crate::resynth::Settings::default(),
        };
        let mut performance = d.performance;
        let sequencer = d
            .sequencer
            .unwrap_or_else(|| crate::sequence::Program::migrate(&mut performance));
        Ok(Self {
            format: d.format,
            name: d.name,
            params,
            lfos,
            envs,
            lfo_count: d.lfo_count,
            env_count: d.env_count,
            performance,
            sequencer,
            resynth,
            taps,
            grain_groups: d.grain_groups,
            drums: d.drums,
            drum_runtime: Default::default(),
            synth: d.synth,
            synth_runtime: Default::default(),
            wavetable: d.wavetable,
            wavetable_runtime: Default::default(),
            wavetable_pool: Default::default(),
            tb303: d.tb303,
            tb303_runtime: Default::default(),
            bus: d.bus,
            bus_runtime: Default::default(),
            vocal: d.vocal,
            sampler: d.sampler,
            sampler_runtime: Default::default(),
            rack: d.rack,
            rack_runtime: Default::default(),
            pre_color_chain: d.pre_color_chain,
            source_color_chain: d.source_color_chain,
            color_chain,
            fx_chain_version: 1,
            imported_effects: Default::default(),
            matrix,
            macros: d.macros,
            filter_slots: d.filter_slots,
            filter_banks: Default::default(),
        })
    }
}
impl Preset {
    /// Prepare the audio owner's copy on the control thread.
    pub fn prepared_clone(&self) -> Self {
        self.prepared_clone_for(self.imported_effects.sample_rate().unwrap_or(48_000))
    }
    pub fn prepared_clone_for(&self, rate: u32) -> Self {
        let mut preset = self.clone();
        preset.sanitize();
        let _ = preset.wavetable_runtime.prepare(
            preset.wavetable.as_deref(),
            &preset.wavetable_pool,
            rate,
        );
        preset.tb303_runtime.prepare(preset.tb303.as_deref(), rate);
        preset.prepare_effects(rate);
        if preset.rack.is_some() {
            // A Rack's sources hand their sound down its modules, so only its
            // Grain engine keeps a history.
            preset
                .drum_runtime
                .prepare_engine(preset.drums.as_deref(), rate);
            preset
                .synth_runtime
                .prepare_engine(preset.synth.as_deref(), rate);
        } else {
            preset.drum_runtime.prepare(preset.drums.as_deref(), rate);
            preset.synth_runtime.prepare(preset.synth.as_deref(), rate);
        }
        preset.bus_runtime.prepare(preset.bus.as_deref(), rate);
        preset.rack_runtime.prepare(preset.rack.as_deref(), rate);
        preset
    }
    /// Drum and Synth timbres keep their own output in a windowed live history.
    /// A Rack's Grain engine keeps one when it does not read the file.
    pub fn history(&self) -> Option<crate::synth::History> {
        if let Some(rack) = &self.rack {
            rack.history()
        } else if let Some(bus) = &self.bus {
            Some(bus.history())
        } else if let Some(drums) = &self.drums {
            Some(crate::synth::History {
                granular: drums.granular,
                buffer_seconds: drums.buffer_seconds,
            })
        } else {
            self.synth.as_ref().map(|synth| synth.history())
        }
    }
    pub fn prepare_imported_effects(&mut self) {
        self.prepare_effects(48_000);
    }
    pub fn prepare_effects(&mut self, rate: u32) {
        self.imported_effects.prepare_racks(
            [
                &self.color_chain,
                &self.pre_color_chain,
                &self.source_color_chain,
            ],
            rate,
        );
        self.filter_banks.prepare(&self.filter_slots, rate);
    }
    /// Synchronous owners know exactly which histories are still available.
    /// Queued native updates use prepared_clone_for instead, so a future slot
    /// change cannot invalidate their prepared resources.
    pub fn prepare_effects_reusing(&mut self, rate: u32, previous: &Self) {
        self.imported_effects.prepare_racks_reusing(
            [
                &self.color_chain,
                &self.pre_color_chain,
                &self.source_color_chain,
            ],
            rate,
            &previous.imported_effects,
            [
                &previous.color_chain,
                &previous.pre_color_chain,
                &previous.source_color_chain,
            ],
        );
    }
    /// The engine settings of every other kind: one source at most.
    fn sanitize_engine(&mut self) {
        if let Some(bass) = &mut self.tb303 {
            bass.sanitize();
            self.bus.get_or_insert_with(Default::default).master = false;
            self.vocal = None;
            self.sampler = None;
        }
        if let Some(vocal) = &mut self.vocal {
            // A Vocal timbre's engine is a bus fed by its voice, never Master.
            vocal.sanitize();
            let bus = self.bus.get_or_insert_with(Default::default);
            bus.master = false;
            self.sampler = None;
        }
        if let Some(sampler) = &mut self.sampler {
            // So is a Sampler's, fed by its kit.
            sampler.sanitize();
            let bus = self.bus.get_or_insert_with(Default::default);
            bus.master = false;
        }
        if let Some(bus) = &mut self.bus {
            // A bus takes its sound from other timbres, never its own engine.
            bus.sanitize();
            self.drums = None;
            self.synth = None;
        }
        if let Some(drums) = &mut self.drums {
            drums.sanitize();
            self.synth = None;
        }
        if let Some(synth) = &mut self.synth {
            synth.sanitize();
        }
    }
    pub fn sanitize(&mut self) {
        // Migrate the first 303 projects to a real source chain in a Rack.
        if self.tb303.is_some() && self.rack.is_none() {
            use crate::rack::{Module, Slot};
            let mut modules = vec![Module::Tb303];
            if !self.source_color_chain.is_empty() {
                modules.push(Module::SourceFx);
            }
            if self.bus.as_ref().is_some_and(|b| b.granular) {
                modules.push(Module::Grains);
            }
            if self.resynth.enabled {
                modules.push(Module::Resynth);
            }
            if !self.pre_color_chain.is_empty() {
                modules.push(Module::PreFx);
            }
            if self.bus.as_ref().is_some_and(|b| b.filters) || !self.filter_slots.is_empty() {
                modules.push(Module::Filters);
            }
            if !self.color_chain.is_empty() {
                modules.push(Module::PostFx);
            }
            self.rack = Some(Box::new(crate::rack::Settings {
                modules: modules
                    .into_iter()
                    .map(|module| Slot {
                        module,
                        ..Default::default()
                    })
                    .collect(),
                buffer_seconds: self.bus.as_ref().map_or(4.0, |b| b.buffer_seconds),
                ..Default::default()
            }));
        }
        self.fx_chain_version = 1;
        self.params.sanitize();
        self.lfo_count = self.lfo_count.clamp(1, MODULATORS as u8);
        self.env_count = self.env_count.clamp(1, MODULATORS as u8);
        self.resynth.sanitize();
        self.taps.sanitize();
        self.grain_groups.sanitize();
        if let Some(rack) = &mut self.rack {
            // A Rack holds the settings of the sources it has, and only those.
            rack.sanitize();
            self.bus = None;
            if rack.has(crate::rack::Module::Tb303) {
                self.tb303.get_or_insert_with(Default::default).sanitize();
            } else {
                self.tb303 = None;
            }
            if rack.has(crate::rack::Module::Vocal) {
                self.vocal
                    .get_or_insert_with(|| Box::new(crate::vocal::Settings::new()))
                    .sanitize();
            } else {
                self.vocal = None;
            }
            if rack.has(crate::rack::Module::Sampler) {
                self.sampler
                    .get_or_insert_with(|| Box::new(crate::sampler::Settings::new()))
                    .sanitize();
            } else {
                self.sampler = None;
            }
            if rack.has(crate::rack::Module::Drums) {
                self.drums.get_or_insert_with(Default::default).sanitize();
            } else {
                self.drums = None;
            }
            if rack.has(crate::rack::Module::Synth) {
                self.synth.get_or_insert_with(Default::default).sanitize();
            } else {
                self.synth = None;
            }
            if rack.has(crate::rack::Module::Wavetable) {
                self.wavetable
                    .get_or_insert_with(Default::default)
                    .sanitize();
            } else {
                self.wavetable = None;
            }
        } else {
            self.wavetable = None;
            self.sanitize_engine();
        }
        self.sequencer.sanitize();
        self.matrix.truncate(crate::matrix::MAX_ROUTES);
        self.color_chain.truncate(crate::color::MAX_COLOR_SLOTS);
        self.pre_color_chain.truncate(crate::color::MAX_COLOR_SLOTS);
        self.source_color_chain.truncate(1);
        self.filter_slots.truncate(MAX_FILTER_SLOTS);
        let mut filter_ids: Vec<u32> = Vec::new();
        for slot in &mut self.filter_slots {
            if slot.id == 0 || filter_ids.contains(&slot.id) {
                slot.id = (1..).find(|id| !filter_ids.contains(id)).unwrap();
            }
            filter_ids.push(slot.id);
            slot.sanitize();
        }
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
                .flat_map(|l| l.routes().map(|(d, _)| d))
                .chain(self.envs.iter().flat_map(|e| e.routes().map(|(d, _)| d)))
                .chain(self.matrix.iter().map(|r| r.destination))
                .chain(
                    self.macros
                        .iter()
                        .flat_map(|m| m.targets.iter().map(|t| t.destination)),
                )
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
            if let Destination::FilterSlot { slot, .. } = *route
                && !filter_ids.contains(&slot)
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
            env.destinations_mut().into_iter().for_each(&validate);
        }
        self.macros.truncate(crate::macros::MAX_MACROS);
        for m in &mut self.macros {
            m.sanitize();
            for target in &mut m.targets {
                validate(&mut target.destination);
            }
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
            lfo.destinations_mut().into_iter().for_each(&validate);
            finite_clamp(&mut lfo.rate_hz, 0.01, 40.0, 0.2);
            finite_clamp(&mut lfo.depth, -1.0, 1.0, 0.0);
            finite_clamp(&mut lfo.depth2, -1.0, 1.0, 0.0);
            finite_clamp(&mut lfo.depth3, -1.0, 1.0, 0.0);
            finite_clamp(&mut lfo.division, 0.0625, 128.0, 1.0);
            finite_clamp(&mut lfo.shape, -1.0, 1.0, 0.0);
            finite_clamp(&mut lfo.phase, 0.0, 1.0, 0.0);
            finite_clamp(&mut lfo.smooth, 0.0, 1.0, 0.0);
            finite_clamp(&mut lfo.delay, 0.0, 1.0, 0.0);
            finite_clamp(&mut lfo.fade, 0.0, 1.0, 0.0);
        }
    }

    /// Factory preset `index` as the library offers it: a Rack of the
    /// modules its engine runs (`as_rack`), else the preset as it is.
    pub fn factory(index: usize) -> Self {
        let preset = Self::builtin(index);
        preset.as_rack().unwrap_or(preset)
    }
    /// This Granular preset as a Rack that plays it the same: the Sample, its
    /// Grain engine, Resynth when it is on, FX A when it holds effects, then
    /// Filters and FX B, every setting kept. None for any other engine, and
    /// for Source FX, which a Granular timbre renders into its file ahead.
    pub fn as_rack(&self) -> Option<Self> {
        use crate::rack::{Module, Slot};
        let classic = self.rack.is_none()
            && self.drums.is_none()
            && self.synth.is_none()
            && self.bus.is_none()
            && self.sampler.is_none()
            && self.vocal.is_none();
        if !classic || self.source_color_chain.iter().any(|s| s.enabled) {
            return None;
        }
        let mut modules = vec![Module::Sample, Module::Grains];
        if self.resynth.enabled {
            modules.push(Module::Resynth);
        }
        if !self.pre_color_chain.is_empty() {
            modules.push(Module::PreFx);
        }
        modules.extend([Module::Filters, Module::PostFx]);
        let mut preset = self.clone();
        preset.source_color_chain.clear();
        preset.rack = Some(Box::new(crate::rack::Settings {
            modules: modules
                .into_iter()
                .map(|module| Slot {
                    module,
                    enabled: true,
                    id: 0,
                })
                .collect(),
            ..Default::default()
        }));
        preset.sanitize();
        Some(preset)
    }

    pub fn builtin(index: usize) -> Self {
        if index == crate::startup::INDEX {
            return crate::startup::preset();
        }
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
