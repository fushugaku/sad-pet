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
    WaveshaperDepth => waveshaper_depth,
    SuperEchoDirty => super_echo_dirty,
    SuperEchoOrbit => super_echo_orbit,
    SuperEchoReverb => super_echo_reverb,
    SuperEchoTime => super_echo_time,
    SuperEchoCutoff => super_echo_cutoff,
    SuperEchoModSpeed => super_echo_mod_speed,
    SuperEchoModDepth => super_echo_mod_depth,
    SuperEchoPitch => super_echo_pitch,
    SuperEchoShimmer => super_echo_shimmer,
    SuperEchoFeedback => super_echo_feedback,
    SuperEchoResonance => super_echo_resonance,
    SuperEchoPingPong => super_echo_ping_pong,
    SuperEchoCompression => super_echo_compression,
    SuperEchoShimmerMode => super_echo_shimmer_mode,
    SuperEchoFilterMode => super_echo_filter_mode,
    SuperEchoDub => super_echo_dub,
    SuperEchoDubMix => super_echo_dub_mix,
    SuperEchoShimmerStyle => super_echo_shimmer_style,
    SuperEchoFreeze => super_echo_freeze,
    SuperEchoSync => super_echo_sync,
    SuperEchoNote => super_echo_note,
    AlphaVerbDirty => alpha_verb_dirty,
    AlphaVerbPredelay => alpha_verb_predelay,
    AlphaVerbEarlyLate => alpha_verb_early_late,
    AlphaVerbSize => alpha_verb_size,
    AlphaVerbShape => alpha_verb_shape,
    AlphaVerbLowDamp => alpha_verb_low_damp,
    AlphaVerbHighDamp => alpha_verb_high_damp,
    AlphaVerbBass => alpha_verb_bass,
    AlphaVerbTreble => alpha_verb_treble,
    AlphaVerbFeedback => alpha_verb_feedback,
    AlphaVerbSpin => alpha_verb_spin,
    AlphaVerbStereo => alpha_verb_stereo,
    AlphaVerbShimmer => alpha_verb_shimmer,
    AlphaVerbDynamics => alpha_verb_dynamics,
    AlphaVerbCompThreshold => alpha_verb_comp_threshold,
    AlphaVerbCompRatio => alpha_verb_comp_ratio,
    AlphaVerbCompRelease => alpha_verb_comp_release,
    AlphaVerbGateThreshold => alpha_verb_gate_threshold,
    AlphaVerbGateHold => alpha_verb_gate_hold,
    AlphaVerbGateRelease => alpha_verb_gate_release,
    AlphaVerbFreeze => alpha_verb_freeze,
    MoozP1 => mooz_p1,
    MoozP2 => mooz_p2,
    MoozP3 => mooz_p3,
    MoozP4 => mooz_p4,
    MoozP5 => mooz_p5,
    MoozP6 => mooz_p6,
    MoozP7 => mooz_p7,
    MoozP8 => mooz_p8,
    MoozP9 => mooz_p9,
}

impl ColorParameter {
    /// Parameter count in the released 0.8 host layout. New IDs append later.
    pub const LEGACY_COUNT: usize = 59;
    /// Generic controls of the mooz models, in each model's catalog order.
    pub const MOOZ: [Self; sad_pet_mooz::MAX_PARAMS] = [
        Self::MoozP1,
        Self::MoozP2,
        Self::MoozP3,
        Self::MoozP4,
        Self::MoozP5,
        Self::MoozP6,
        Self::MoozP7,
        Self::MoozP8,
        Self::MoozP9,
    ];
    /// SuperEcho and AlphaVerb controls, appended after the 0.8 layout.
    pub fn is_imported(self) -> bool {
        self as usize >= Self::LEGACY_COUNT && !self.is_mooz()
    }
    /// Mooz controls append after every other host ID.
    pub fn is_mooz(self) -> bool {
        self as usize >= Self::MoozP1 as usize
    }
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
    pub lcr_smooth: bool,
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
    pub mod_delay_smooth: bool,
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
    pub waveshaper_type: crate::dsp::radias_waveshaper::WaveshaperMode,
    pub waveshaper_depth: f32,
    pub super_echo_dirty: f32,
    pub super_echo_orbit: f32,
    pub super_echo_reverb: f32,
    pub super_echo_time: f32,
    pub super_echo_cutoff: f32,
    pub super_echo_mod_speed: f32,
    pub super_echo_mod_depth: f32,
    pub super_echo_pitch: f32,
    pub super_echo_shimmer: f32,
    pub super_echo_feedback: f32,
    pub super_echo_resonance: f32,
    pub super_echo_ping_pong: f32,
    pub super_echo_compression: f32,
    pub super_echo_shimmer_mode: f32,
    pub super_echo_filter_mode: f32,
    pub super_echo_dub: f32,
    pub super_echo_dub_mix: f32,
    pub super_echo_shimmer_style: f32,
    pub super_echo_freeze: f32,
    pub super_echo_sync: f32,
    pub super_echo_note: f32,
    pub alpha_verb_dirty: f32,
    pub alpha_verb_predelay: f32,
    pub alpha_verb_early_late: f32,
    pub alpha_verb_size: f32,
    pub alpha_verb_shape: f32,
    pub alpha_verb_low_damp: f32,
    pub alpha_verb_high_damp: f32,
    pub alpha_verb_bass: f32,
    pub alpha_verb_treble: f32,
    pub alpha_verb_feedback: f32,
    pub alpha_verb_spin: f32,
    pub alpha_verb_stereo: f32,
    pub alpha_verb_shimmer: f32,
    pub alpha_verb_dynamics: f32,
    pub alpha_verb_comp_threshold: f32,
    pub alpha_verb_comp_ratio: f32,
    pub alpha_verb_comp_release: f32,
    pub alpha_verb_gate_threshold: f32,
    pub alpha_verb_gate_hold: f32,
    pub alpha_verb_gate_release: f32,
    pub alpha_verb_freeze: f32,
    /// Normalized values of the slot's mooz model, in catalog order.
    pub mooz_p1: f32,
    pub mooz_p2: f32,
    pub mooz_p3: f32,
    pub mooz_p4: f32,
    pub mooz_p5: f32,
    pub mooz_p6: f32,
    pub mooz_p7: f32,
    pub mooz_p8: f32,
    pub mooz_p9: f32,
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
            lcr_smooth: false,
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
            mod_delay_smooth: false,
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
            waveshaper_type: crate::dsp::radias_waveshaper::WaveshaperMode::Drive,
            waveshaper_depth: 64.0 / 127.0,
            super_echo_dirty: 0.0,
            super_echo_orbit: 0.2,
            super_echo_reverb: 0.0,
            super_echo_time: 0.8,
            super_echo_cutoff: 63.0 / 127.0,
            super_echo_mod_speed: 0.25,
            super_echo_mod_depth: 0.25,
            super_echo_pitch: 1.0,
            super_echo_shimmer: 0.0,
            super_echo_feedback: 0.0,
            super_echo_resonance: 0.5,
            super_echo_ping_pong: 0.5,
            super_echo_compression: 0.0,
            super_echo_shimmer_mode: 0.0,
            super_echo_filter_mode: 0.0,
            super_echo_dub: 0.0,
            super_echo_dub_mix: 0.5,
            super_echo_shimmer_style: 0.0,
            super_echo_freeze: 0.0,
            super_echo_sync: 0.0,
            super_echo_note: 0.5,
            alpha_verb_dirty: 0.0,
            alpha_verb_predelay: 0.0,
            alpha_verb_early_late: 0.5,
            alpha_verb_size: 0.5,
            alpha_verb_shape: 0.5,
            alpha_verb_low_damp: 0.0,
            alpha_verb_high_damp: 0.0,
            alpha_verb_bass: 0.5,
            alpha_verb_treble: 0.5,
            alpha_verb_feedback: 0.5,
            alpha_verb_spin: 0.5,
            alpha_verb_stereo: 0.5,
            alpha_verb_shimmer: 0.0,
            alpha_verb_dynamics: 0.0,
            alpha_verb_comp_threshold: 0.8125,
            alpha_verb_comp_ratio: 0.5,
            alpha_verb_comp_release: sad_pet_alphaverb::parameters::PARAMETERS[16]
                .default_normalized,
            alpha_verb_gate_threshold: 0.8125,
            alpha_verb_gate_hold: sad_pet_alphaverb::parameters::PARAMETERS[18].default_normalized,
            alpha_verb_gate_release: 0.0,
            alpha_verb_freeze: 0.0,
            mooz_p1: 0.0,
            mooz_p2: 0.0,
            mooz_p3: 0.0,
            mooz_p4: 0.0,
            mooz_p5: 0.0,
            mooz_p6: 0.0,
            mooz_p7: 0.0,
            mooz_p8: 0.0,
            mooz_p9: 0.0,
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
        const MOOZ_CONTROLS: [&[ColorParameter]; sad_pet_mooz::MAX_PARAMS + 1] = [
            &[P::Mix],
            &[P::MoozP1, P::Mix],
            &[P::MoozP1, P::MoozP2, P::Mix],
            &[P::MoozP1, P::MoozP2, P::MoozP3, P::Mix],
            &[P::MoozP1, P::MoozP2, P::MoozP3, P::MoozP4, P::Mix],
            &[
                P::MoozP1,
                P::MoozP2,
                P::MoozP3,
                P::MoozP4,
                P::MoozP5,
                P::Mix,
            ],
            &[
                P::MoozP1,
                P::MoozP2,
                P::MoozP3,
                P::MoozP4,
                P::MoozP5,
                P::MoozP6,
                P::Mix,
            ],
            &[
                P::MoozP1,
                P::MoozP2,
                P::MoozP3,
                P::MoozP4,
                P::MoozP5,
                P::MoozP6,
                P::MoozP7,
                P::Mix,
            ],
            &[
                P::MoozP1,
                P::MoozP2,
                P::MoozP3,
                P::MoozP4,
                P::MoozP5,
                P::MoozP6,
                P::MoozP7,
                P::MoozP8,
                P::Mix,
            ],
            &[
                P::MoozP1,
                P::MoozP2,
                P::MoozP3,
                P::MoozP4,
                P::MoozP5,
                P::MoozP6,
                P::MoozP7,
                P::MoozP8,
                P::MoozP9,
                P::Mix,
            ],
        ];
        match self.kind {
            SaturatorType::Mooz(model) => MOOZ_CONTROLS[model.info().params.len()],
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
            SaturatorType::Waveshaper => &[P::WaveshaperDepth, P::Mix],
            SaturatorType::SuperEcho => &[
                P::SuperEchoDirty,
                P::SuperEchoOrbit,
                P::SuperEchoReverb,
                P::SuperEchoTime,
                P::SuperEchoCutoff,
                P::SuperEchoModSpeed,
                P::SuperEchoModDepth,
                P::SuperEchoPitch,
                P::SuperEchoShimmer,
                P::SuperEchoFeedback,
                P::SuperEchoResonance,
                P::SuperEchoPingPong,
                P::SuperEchoCompression,
                P::SuperEchoShimmerMode,
                P::SuperEchoFilterMode,
                P::SuperEchoDub,
                P::SuperEchoDubMix,
                P::SuperEchoShimmerStyle,
                P::SuperEchoFreeze,
                P::SuperEchoSync,
                P::SuperEchoNote,
                P::Mix,
            ],
            SaturatorType::AlphaVerb => &[
                P::AlphaVerbDirty,
                P::AlphaVerbPredelay,
                P::AlphaVerbEarlyLate,
                P::AlphaVerbSize,
                P::AlphaVerbShape,
                P::AlphaVerbLowDamp,
                P::AlphaVerbHighDamp,
                P::AlphaVerbBass,
                P::AlphaVerbTreble,
                P::AlphaVerbFeedback,
                P::AlphaVerbSpin,
                P::AlphaVerbStereo,
                P::AlphaVerbShimmer,
                P::AlphaVerbDynamics,
                P::AlphaVerbCompThreshold,
                P::AlphaVerbCompRatio,
                P::AlphaVerbCompRelease,
                P::AlphaVerbGateThreshold,
                P::AlphaVerbGateHold,
                P::AlphaVerbGateRelease,
                P::AlphaVerbFreeze,
                P::Mix,
            ],
        }
    }
    pub fn super_echo_values(&self) -> [f32; 20] {
        let s = self.settings;
        [
            s.super_echo_dirty,
            s.super_echo_orbit,
            s.super_echo_reverb,
            s.super_echo_time,
            s.super_echo_cutoff,
            s.super_echo_mod_speed,
            s.super_echo_mod_depth,
            s.super_echo_pitch,
            s.super_echo_shimmer,
            s.super_echo_feedback,
            s.super_echo_resonance,
            s.super_echo_ping_pong,
            s.super_echo_compression,
            s.super_echo_shimmer_mode,
            s.super_echo_filter_mode,
            s.super_echo_dub,
            s.super_echo_dub_mix,
            s.super_echo_shimmer_style,
            1.0,
            s.super_echo_freeze,
        ]
    }
    pub fn alpha_verb_values(&self) -> [f32; 22] {
        let s = self.settings;
        [
            s.alpha_verb_dirty,
            s.alpha_verb_predelay,
            s.alpha_verb_early_late,
            s.alpha_verb_size,
            s.alpha_verb_shape,
            s.alpha_verb_low_damp,
            s.alpha_verb_high_damp,
            s.alpha_verb_bass,
            s.alpha_verb_treble,
            s.alpha_verb_feedback,
            s.alpha_verb_spin,
            s.alpha_verb_stereo,
            s.alpha_verb_shimmer,
            s.alpha_verb_dynamics,
            s.alpha_verb_comp_threshold,
            s.alpha_verb_comp_ratio,
            s.alpha_verb_comp_release,
            s.alpha_verb_gate_threshold,
            s.alpha_verb_gate_hold,
            s.alpha_verb_gate_release,
            1.0,
            s.alpha_verb_freeze,
        ]
    }
    pub fn mooz_values(&self) -> [f32; sad_pet_mooz::MAX_PARAMS] {
        let s = self.settings;
        [
            s.mooz_p1, s.mooz_p2, s.mooz_p3, s.mooz_p4, s.mooz_p5, s.mooz_p6, s.mooz_p7, s.mooz_p8,
            s.mooz_p9,
        ]
    }
    /// A fresh slot of a mooz model at its original defaults. The model has
    /// its own Mix and Level, so the slot passes it through fully wet.
    pub fn mooz(id: u32, model: sad_pet_mooz::Model) -> Self {
        let mut slot = Self {
            id,
            kind: SaturatorType::Mooz(model),
            mix: 1.0,
            ..Self::default()
        };
        slot.set_mooz_defaults();
        slot
    }
    pub fn set_mooz_defaults(&mut self) {
        let defaults = self.kind.mooz().map(|m| m.normalized_defaults());
        let defaults = defaults.unwrap_or([0.0; sad_pet_mooz::MAX_PARAMS]);
        for (parameter, value) in ColorParameter::MOOZ.into_iter().zip(defaults) {
            *self.parameter_mut(parameter) = value;
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
            super_echo_dirty,
            super_echo_orbit,
            super_echo_reverb,
            super_echo_time,
            super_echo_cutoff,
            super_echo_mod_speed,
            super_echo_mod_depth,
            super_echo_pitch,
            super_echo_shimmer,
            super_echo_feedback,
            super_echo_resonance,
            super_echo_ping_pong,
            super_echo_compression,
            super_echo_shimmer_mode,
            super_echo_filter_mode,
            super_echo_dub,
            super_echo_dub_mix,
            super_echo_shimmer_style,
            super_echo_freeze,
            super_echo_sync,
            super_echo_note,
            alpha_verb_dirty,
            alpha_verb_predelay,
            alpha_verb_early_late,
            alpha_verb_size,
            alpha_verb_shape,
            alpha_verb_low_damp,
            alpha_verb_high_damp,
            alpha_verb_bass,
            alpha_verb_treble,
            alpha_verb_feedback,
            alpha_verb_spin,
            alpha_verb_stereo,
            alpha_verb_shimmer,
            alpha_verb_dynamics,
            alpha_verb_comp_threshold,
            alpha_verb_comp_ratio,
            alpha_verb_comp_release,
            alpha_verb_gate_threshold,
            alpha_verb_gate_hold,
            alpha_verb_gate_release,
            alpha_verb_freeze,
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
            compressor_makeup,
            waveshaper_depth,
            mooz_p1,
            mooz_p2,
            mooz_p3,
            mooz_p4,
            mooz_p5,
            mooz_p6,
            mooz_p7,
            mooz_p8,
            mooz_p9
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

/// What the UI needs to build mooz cards: every model's catalog entry, the menu
/// sections and the tempo-note labels used by synced parameters.
pub fn mooz_catalog() -> serde_json::Value {
    // Plain integer displays are the common case; the UI prints those itself.
    let plain = serde_json::to_value(sad_pet_mooz::Display::INT).expect("display");
    let models: Vec<_> = sad_pet_mooz::Model::ALL
        .iter()
        .map(|model| {
            let info = model.info();
            let params: Vec<_> = info
                .params
                .iter()
                .map(|param| {
                    let mut entry = serde_json::json!({
                        "name": param.name,
                        "max": param.max,
                        "default": param.default,
                    });
                    let display = serde_json::to_value(param.display).expect("display");
                    if display != plain {
                        entry["display"] = display;
                    }
                    entry
                })
                .collect();
            serde_json::json!({
                "key": info.key,
                "name": info.name,
                "category": info.category,
                "description": info.description,
                "params": params,
            })
        })
        .collect();
    let categories: Vec<_> = sad_pet_mooz::Category::ALL
        .iter()
        .map(|category| serde_json::json!({"key": category, "label": category.label()}))
        .collect();
    serde_json::json!({
        "models": models,
        "categories": categories,
        "delay_notes": &sad_pet_mooz::zoom::DELAY_NOTE_LABELS[..],
        "rate_notes": &sad_pet_mooz::zoom::RATE_NOTE_LABELS[..],
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn mooz_catalog_lists_every_model_compactly() {
        let catalog = super::mooz_catalog();
        assert_eq!(catalog["models"].as_array().unwrap().len(), 137);
        assert_eq!(catalog["delay_notes"].as_array().unwrap().len(), 71);
        let size = serde_json::to_string(&catalog).unwrap().len();
        assert!(size < 128 * 1024, "{size} bytes");
    }
}
