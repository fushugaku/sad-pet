//! Persisted Color FX rack. Every slot owns all algorithm settings, so changing
//! its type and returning to it restores the previous knob positions.
use crate::preset::{Destination, Params, ReverbType, SaturatorType, finite_clamp};
use serde::{Deserialize, Serialize};

pub const MAX_COLOR_SLOTS: usize = 12;

// A single registry keeps persisted destinations, control keys and telemetry aligned.
macro_rules! color_parameters {
    ($($variant:ident => $field:ident),* $(,)?) => {
        #[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
        #[serde(rename_all = "snake_case")]
        pub enum ColorParameter { Mix, $($variant),* }
        impl ColorParameter {
            pub const ALL: &'static [Self] = &[Self::Mix, $(Self::$variant),*];
            pub fn key(self) -> &'static str {
                match self { Self::Mix => "mix", $(Self::$variant => stringify!($field)),* }
            }
        }
        impl ColorSlot {
            pub fn parameter_mut(&mut self, parameter: ColorParameter) -> &mut f32 {
                match parameter {
                    ColorParameter::Mix => &mut self.mix,
                    $(ColorParameter::$variant => &mut self.settings.$field),*
                }
            }
        }
    };
}
color_parameters! {
    DistortionDrive => distortion_drive,
    DistortionTone => distortion_tone,
    DistortionBias => distortion_bias,
    ReducerRate => reducer_rate,
    ReducerJitter => reducer_jitter,
    ReducerSmooth => reducer_smooth,
    CrusherBits => crusher_bits,
    CrusherDrive => crusher_drive,
    CrusherDither => crusher_dither,
    FlangerRate => flanger_rate,
    FlangerDepth => flanger_depth,
    FlangerDelay => flanger_delay,
    FlangerFeedback => flanger_feedback,
    FlangerStereo => flanger_stereo,
    ChorusRate => chorus_rate,
    ChorusDepth => chorus_depth,
    ChorusDelay => chorus_delay,
    ChorusFeedback => chorus_feedback,
    ChorusStereo => chorus_stereo,
    BinauralDelay => binaural_delay,
    BinauralCrossfeed => binaural_crossfeed,
    BinauralWidth => binaural_width,
    ReverbPredelay => reverb_predelay,
    ReverbDecay => reverb_decay,
    ReverbTone => reverb_tone,
    ReverbShimmer => reverb_shimmer,
    ReverbDiffuse => reverb_diffuse,
    LcrRatio => lcr_ratio,
    LcrLeftDelay => lcr_left_delay,
    LcrCenterDelay => lcr_center_delay,
    LcrRightDelay => lcr_right_delay,
    LcrLeftNote => lcr_left_note,
    LcrCenterNote => lcr_center_note,
    LcrRightNote => lcr_right_note,
    LcrLeftLevel => lcr_left_level,
    LcrCenterLevel => lcr_center_level,
    LcrRightLevel => lcr_right_level,
    LcrFeedback => lcr_feedback,
    LcrHighDamp => lcr_high_damp,
    LcrLowDamp => lcr_low_damp,
    LcrTrim => lcr_trim,
    LcrSpread => lcr_spread,
    ModDelayRatio => mod_delay_ratio,
    ModDelayLeftDelay => mod_delay_left_delay,
    ModDelayRightDelay => mod_delay_right_delay,
    ModDelayLeftNote => mod_delay_left_note,
    ModDelayRightNote => mod_delay_right_note,
    ModDelayFeedback => mod_delay_feedback,
    ModDelayDepth => mod_delay_depth,
    ModDelayRate => mod_delay_rate,
    ModDelaySpread => mod_delay_spread,
    CompressorThreshold => compressor_threshold,
    CompressorRatio => compressor_ratio,
    CompressorAttack => compressor_attack,
    CompressorRelease => compressor_release,
    CompressorKnee => compressor_knee,
    CompressorMakeup => compressor_makeup,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct ColorSettings {
    pub distortion_drive: f32,
    pub distortion_tone: f32,
    pub distortion_bias: f32,
    pub reducer_rate: f32,
    pub reducer_jitter: f32,
    pub reducer_smooth: f32,
    pub crusher_bits: f32,
    pub crusher_drive: f32,
    pub crusher_dither: f32,
    pub flanger_rate: f32,
    pub flanger_depth: f32,
    pub flanger_delay: f32,
    pub flanger_feedback: f32,
    pub flanger_stereo: f32,
    pub chorus_rate: f32,
    pub chorus_depth: f32,
    pub chorus_delay: f32,
    pub chorus_feedback: f32,
    pub chorus_stereo: f32,
    pub binaural_delay: f32,
    pub binaural_crossfeed: f32,
    pub binaural_width: f32,
    pub reverb_type: ReverbType,
    pub reverb_predelay: f32,
    pub reverb_decay: f32,
    pub reverb_tone: f32,
    pub reverb_shimmer: f32,
    pub reverb_diffuse: f32,
    pub lcr_sync: bool,
    pub lcr_ratio: f32,
    pub lcr_left_delay: f32,
    pub lcr_center_delay: f32,
    pub lcr_right_delay: f32,
    pub lcr_left_note: f32,
    pub lcr_center_note: f32,
    pub lcr_right_note: f32,
    pub lcr_left_level: f32,
    pub lcr_center_level: f32,
    pub lcr_right_level: f32,
    pub lcr_feedback: f32,
    pub lcr_high_damp: f32,
    pub lcr_low_damp: f32,
    pub lcr_trim: f32,
    pub lcr_spread: f32,
    pub mod_delay_sync: bool,
    pub mod_delay_ratio: f32,
    pub mod_delay_left_delay: f32,
    pub mod_delay_right_delay: f32,
    pub mod_delay_left_note: f32,
    pub mod_delay_right_note: f32,
    pub mod_delay_feedback: f32,
    pub mod_delay_depth: f32,
    pub mod_delay_rate: f32,
    pub mod_delay_spread: f32,
    pub compressor_threshold: f32,
    pub compressor_ratio: f32,
    pub compressor_attack: f32,
    pub compressor_release: f32,
    pub compressor_knee: f32,
    pub compressor_makeup: f32,
}
impl Default for ColorSettings {
    fn default() -> Self {
        Self {
            distortion_drive: 0.5,
            distortion_tone: 1.0,
            distortion_bias: 0.5,
            reducer_rate: 0.6,
            reducer_jitter: 0.0,
            reducer_smooth: 0.0,
            crusher_bits: 7.0 / 15.0,
            crusher_drive: 0.0,
            crusher_dither: 0.0,
            flanger_rate: 27.0_f32.ln() / 2000.0_f32.ln(),
            flanger_depth: 0.0025 / (0.003 * 0.99),
            flanger_delay: (0.003 - 0.0001) / 0.0099,
            flanger_feedback: 0.5 + 0.55 / 1.9,
            flanger_stereo: 0.5,
            chorus_rate: 27.0_f32.ln() / 2000.0_f32.ln(),
            chorus_depth: 0.6,
            chorus_delay: (0.020 - 0.005) / 0.035,
            chorus_feedback: 0.5 + 0.1 / 1.9,
            chorus_stereo: 0.5,
            binaural_delay: 0.4,
            binaural_crossfeed: 1.0 / 3.0,
            binaural_width: 0.5,
            reverb_type: ReverbType::PlateS,
            reverb_predelay: 0.0,
            reverb_decay: 0.45,
            reverb_tone: 0.65,
            reverb_shimmer: 0.0,
            reverb_diffuse: 0.5,
            lcr_sync: true,
            lcr_ratio: 64.0 / 127.0,
            lcr_left_delay: 69.0 / 127.0,
            lcr_center_delay: 89.0 / 127.0,
            lcr_right_delay: 59.0 / 127.0,
            lcr_left_note: 7.0 / 13.0,
            lcr_center_note: 8.0 / 13.0,
            lcr_right_note: 5.0 / 13.0,
            lcr_left_level: 100.0 / 127.0,
            lcr_center_level: 50.0 / 127.0,
            lcr_right_level: 100.0 / 127.0,
            lcr_feedback: 90.0 / 127.0,
            lcr_high_damp: 0.0,
            lcr_low_damp: 0.0,
            lcr_trim: 1.0,
            lcr_spread: 63.0 / 127.0,
            mod_delay_sync: true,
            mod_delay_ratio: 64.0 / 127.0,
            mod_delay_left_delay: 64.0 / 127.0,
            mod_delay_right_delay: 52.0 / 127.0,
            mod_delay_left_note: 7.0 / 13.0,
            mod_delay_right_note: 5.0 / 13.0,
            mod_delay_feedback: 90.0 / 127.0,
            mod_delay_depth: 30.0 / 127.0,
            mod_delay_rate: 34.0 / 127.0,
            mod_delay_spread: 1.0,
            compressor_threshold: 0.7, // -18 dBFS
            compressor_ratio: 4.0_f32.ln() / 20.0_f32.ln(),
            compressor_attack: 2.0 / 3.0,                       // 10 ms
            compressor_release: 12.0_f32.ln() / 200.0_f32.ln(), // 120 ms
            compressor_knee: 0.25,                              // 6 dB
            compressor_makeup: 0.0,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct ColorSlot {
    pub id: u32,
    pub kind: SaturatorType,
    pub enabled: bool,
    pub mix: f32,
    pub settings: ColorSettings,
}
impl Default for ColorSlot {
    fn default() -> Self {
        Self {
            id: 1,
            kind: SaturatorType::Distortion,
            enabled: true,
            mix: 0.5,
            settings: ColorSettings::default(),
        }
    }
}
impl ColorSlot {
    pub fn control_parameters(&self) -> &'static [ColorParameter] {
        use ColorParameter as P;
        match self.kind {
            SaturatorType::Distortion => &[
                P::DistortionDrive,
                P::DistortionTone,
                P::DistortionBias,
                P::Mix,
            ],
            SaturatorType::Reducer => &[P::ReducerRate, P::ReducerJitter, P::ReducerSmooth, P::Mix],
            SaturatorType::Crusher => &[P::CrusherBits, P::CrusherDrive, P::CrusherDither, P::Mix],
            SaturatorType::Flanger => &[
                P::FlangerRate,
                P::FlangerDepth,
                P::FlangerDelay,
                P::FlangerFeedback,
                P::FlangerStereo,
                P::Mix,
            ],
            SaturatorType::Chorus => &[
                P::ChorusRate,
                P::ChorusDepth,
                P::ChorusDelay,
                P::ChorusFeedback,
                P::ChorusStereo,
                P::Mix,
            ],
            SaturatorType::Binaural => &[
                P::BinauralDelay,
                P::BinauralCrossfeed,
                P::BinauralWidth,
                P::Mix,
            ],
            SaturatorType::Reverb => &[
                P::ReverbPredelay,
                P::ReverbDecay,
                P::ReverbTone,
                P::ReverbShimmer,
                P::ReverbDiffuse,
                P::Mix,
            ],
            SaturatorType::LcrDelay if self.settings.lcr_sync => &[
                P::LcrLeftNote,
                P::LcrCenterNote,
                P::LcrRightNote,
                P::LcrLeftLevel,
                P::LcrCenterLevel,
                P::LcrRightLevel,
                P::LcrRatio,
                P::LcrFeedback,
                P::LcrHighDamp,
                P::LcrLowDamp,
                P::LcrTrim,
                P::LcrSpread,
                P::Mix,
            ],
            SaturatorType::LcrDelay => &[
                P::LcrLeftDelay,
                P::LcrCenterDelay,
                P::LcrRightDelay,
                P::LcrLeftLevel,
                P::LcrCenterLevel,
                P::LcrRightLevel,
                P::LcrRatio,
                P::LcrFeedback,
                P::LcrHighDamp,
                P::LcrLowDamp,
                P::LcrTrim,
                P::LcrSpread,
                P::Mix,
            ],
            SaturatorType::ModDelay if self.settings.mod_delay_sync => &[
                P::ModDelayLeftNote,
                P::ModDelayRightNote,
                P::ModDelayRatio,
                P::ModDelayFeedback,
                P::ModDelayDepth,
                P::ModDelayRate,
                P::ModDelaySpread,
                P::Mix,
            ],
            SaturatorType::ModDelay => &[
                P::ModDelayLeftDelay,
                P::ModDelayRightDelay,
                P::ModDelayRatio,
                P::ModDelayFeedback,
                P::ModDelayDepth,
                P::ModDelayRate,
                P::ModDelaySpread,
                P::Mix,
            ],
            SaturatorType::Compressor => &[
                P::CompressorThreshold,
                P::CompressorRatio,
                P::CompressorAttack,
                P::CompressorRelease,
                P::CompressorKnee,
                P::CompressorMakeup,
                P::Mix,
            ],
        }
    }
    pub fn legacy_reverb(id: u32, params: &Params) -> Self {
        Self {
            id,
            kind: SaturatorType::Reverb,
            mix: params.reverb,
            settings: ColorSettings {
                reverb_type: params.reverb_type,
                reverb_predelay: params.predelay,
                reverb_decay: params.reverb_decay,
                reverb_tone: params.reverb_tone,
                reverb_shimmer: params.shimmer,
                reverb_diffuse: params.diffuse,
                ..ColorSettings::default()
            },
            ..Self::default()
        }
    }
    pub fn reverb_params(&self) -> Params {
        Params {
            reverb: self.mix,
            reverb_type: self.settings.reverb_type,
            predelay: self.settings.reverb_predelay,
            reverb_decay: self.settings.reverb_decay,
            reverb_tone: self.settings.reverb_tone,
            shimmer: self.settings.reverb_shimmer,
            diffuse: self.settings.reverb_diffuse,
            ..Params::default()
        }
    }
    pub fn legacy(kind: SaturatorType, amount: f32) -> Self {
        let amount = if amount.is_finite() {
            amount.clamp(0.0, 1.0)
        } else {
            0.0
        };
        let mut slot = Self {
            kind,
            mix: amount,
            ..Self::default()
        };
        match kind {
            SaturatorType::Distortion => slot.settings.distortion_drive = amount,
            SaturatorType::Reducer => {
                let period = 1.0 + (amount.powi(2) * 63.0).floor();
                slot.settings.reducer_rate = 1.0 - period.log2() / 10.0;
            }
            SaturatorType::Crusher => slot.settings.crusher_bits = (15.0 - amount * 13.0) / 15.0,
            _ => {}
        }
        slot
    }
    pub fn sanitize(&mut self) {
        finite_clamp(&mut self.mix, 0.0, 1.0, 0.5);
        let defaults = ColorSettings::default();
        macro_rules! unit { ($($field:ident),*) => { $(finite_clamp(&mut self.settings.$field, 0.0, 1.0, defaults.$field);)* }; }
        unit!(
            distortion_drive,
            distortion_tone,
            distortion_bias,
            reducer_rate,
            reducer_jitter,
            reducer_smooth,
            crusher_bits,
            crusher_drive,
            crusher_dither,
            flanger_rate,
            flanger_depth,
            flanger_delay,
            flanger_feedback,
            flanger_stereo,
            chorus_rate,
            chorus_depth,
            chorus_delay,
            chorus_feedback,
            chorus_stereo,
            binaural_delay,
            binaural_crossfeed,
            binaural_width,
            reverb_predelay,
            reverb_decay,
            reverb_tone,
            reverb_shimmer,
            reverb_diffuse,
            lcr_ratio,
            lcr_left_delay,
            lcr_center_delay,
            lcr_right_delay,
            lcr_left_note,
            lcr_center_note,
            lcr_right_note,
            lcr_left_level,
            lcr_center_level,
            lcr_right_level,
            lcr_feedback,
            lcr_high_damp,
            lcr_low_damp,
            lcr_trim,
            lcr_spread,
            mod_delay_ratio,
            mod_delay_left_delay,
            mod_delay_right_delay,
            mod_delay_left_note,
            mod_delay_right_note,
            mod_delay_feedback,
            mod_delay_depth,
            mod_delay_rate,
            mod_delay_spread,
            compressor_threshold,
            compressor_ratio,
            compressor_attack,
            compressor_release,
            compressor_knee,
            compressor_makeup
        );
    }
}

pub fn legacy_reverb_parameter(destination: Destination) -> Option<ColorParameter> {
    Some(match destination {
        Destination::Reverb => ColorParameter::Mix,
        Destination::Predelay => ColorParameter::ReverbPredelay,
        Destination::ReverbDecay => ColorParameter::ReverbDecay,
        Destination::ReverbTone => ColorParameter::ReverbTone,
        Destination::Shimmer => ColorParameter::ReverbShimmer,
        Destination::Diffuse => ColorParameter::ReverbDiffuse,
        _ => return None,
    })
}

pub fn mirror_reverb(params: &mut Params, slots: &[ColorSlot]) {
    if let Some(slot) = slots.iter().find(|s| s.kind == SaturatorType::Reverb) {
        let p = slot.reverb_params();
        params.reverb = p.reverb;
        params.reverb_type = p.reverb_type;
        params.predelay = p.predelay;
        params.reverb_decay = p.reverb_decay;
        params.reverb_tone = p.reverb_tone;
        params.shimmer = p.shimmer;
        params.diffuse = p.diffuse;
    } else {
        params.reverb = 0.0;
    }
}
