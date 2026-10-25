//! TapeEcho (TAPEECHO.ZDL), from the recovered `Fx_DLY_TapeEcho` (`tape_echo.rs`)
//! and the decoded editors.
//!
//! Callback: the stereo half-sum passes a DC blocker [20..24]; with the
//! previous tap times F.B [17] it is filtered by the HiDMP biquad [25..29] and
//! written to the ring. The read position is a Q14 offset: the Time target
//! [16] glides through a one-pole (coefficient 1.856e-4, ~120 ms) and a slow
//! triangle (`|phase|`, 0.356 Hz, depth ~3.2 ms) adds tape wow. The
//! interpolated tap feeds back unfiltered (tone biquad [30..34] is the ROM
//! identity) and reaches the output through a ~150 Hz first-order high-pass
//! [35..37]; wet [10] and dry [11] mix both channels with the same mono echo.
//!
//! Editors: Time `ms·44.1` samples (sync notes limited to 2000 ms); a live
//! Time edit only moves the target, so the tape glides to the new time like the
//! original (no tap mute). F.B `0.95·(v/100)²`, Mix `0.8·(v/100)^1.41` (Main
//! curves 1 and 5), Level `v/100`, HiDMP copies one of eleven second-order
//! low-passes. All ROM filters are re-derived for the host rate; the modulation
//! keeps its rate in Hz and depth in milliseconds.
use super::common::{self, Biquad, DelayLine, Ramp, f, flush, q31};
use crate::{Effect, Values, catalog::*, dsp::Coefficients, zoom};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_tape_echo",
    name: "TapeEcho",
    category: Category::Delay,
    zdl: "TAPEECHO.ZDL",
    description: "Tape echo with wow and gliding time changes",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Time",
            max: 2014,
            default: 559,
            display: Display::DelayTime {
                threshold: TIME_THRESHOLD,
                offset: 1,
            },
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "F.B",
            max: 100,
            default: 64,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 56,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "HiDMP",
            max: 10,
            default: 5,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
    ],
};

const TIME_THRESHOLD: u16 = 1999;
const MAX_MS: f32 = 2000.0;

/// Input DC blocker [20..24] of ROM 0x800005d8.
const INPUT: [u32; 5] = [0x3f7fa2c2, 0xbf7fa2c2, 0x00000000, 0x3f7f4585, 0x00000000];
/// Output smoothing [35..37]: `y = c35 x + c36 x1 + c37 y1`.
const SMOOTHING: [u32; 5] = [0x3f7d4b15, 0xbf7d4b15, 0x00000000, 0x3f7a9629, 0x00000000];
/// Time glide [40] (its complement [41] is the pole), triangle increment [42]
/// and depth [43] in Q14 samples.
const GLIDE: u32 = 0x00061546;
const LFO_INCREMENT: u32 = 0x00008782;
const LFO_DEPTH: u32 = 0x00236c59;

/// ROM 0x80000428: the HiDMP biquads copied to [25..29].
const HIDMP: [[u32; 5]; 11] = [
    [0x3951057f, 0x39d105c0, 0x3951057f, 0x3ffb4e6a, 0xbf76d115],
    [0x3a4d4d00, 0x3acd4d00, 0x3a4d4d00, 0x3ff694c2, 0xbf6df6d2],
    [0x3b462cf8, 0x3bc62cfc, 0x3b462cf8, 0x3fed0e54, 0xbf5d355c],
    [0x3b98294c, 0x3c18294a, 0x3b98294c, 0x3fe843ce, 0xbf5548e6],
    [0x3c241012, 0x3ca41013, 0x3c241012, 0x3fdc3be5, 0xbf42b8cc],
    [0x3c8bf37f, 0x3d0bf37f, 0x3c8bf37f, 0x3fd02a61, 0xbf31d332],
    [0x3cd2207e, 0x3d52207e, 0x3cd2207e, 0x3fc4190e, 0xbf22762c],
    [0x3d119135, 0x3d919136, 0x3d119135, 0x3fb80f49, 0xbf1482df],
    [0x3d3ee1fa, 0x3dbee1fa, 0x3d3ee1fa, 0x3fac1273, 0xbf07dd64],
    [0x3d707fe0, 0x3df07fe0, 0x3d707fe0, 0x3fa02652, 0xbef8d936],
    [0x3daf7edc, 0x3e2f7edc, 0x3daf7edc, 0x3f888900, 0xbed1a2dc],
];

pub struct TapeEcho {
    sample_rate: f32,
    line: DelayLine,
    input: Biquad,
    hidmp: [Coefficients; 11],
    record: Biquad,
    smoothing: Biquad,
    feedback: Ramp,
    wet: Ramp,
    level: Ramp,
    target: f32,
    position: f32,
    glide: f32,
    phase: u32,
    increment: u32,
    depth: f32,
    feedback_sample: f32,
    hidmp_index: usize,
    started: bool,
}

impl TapeEcho {
    pub fn new(sample_rate: f32) -> Self {
        let ratio = common::rate_ratio(sample_rate);
        // |phase| / 2^31 times the Q31 depth, in Q14 samples.
        let depth = q31(LFO_DEPTH) * 2_147_483_648.0 / 16_384.0 * ratio;
        let hidmp = HIDMP.map(|words| common::rom_biquad(words, sample_rate));
        Self {
            sample_rate,
            line: common::line_for_ms(MAX_MS, depth.ceil() as usize + 2, sample_rate),
            input: Biquad::new(common::rom_biquad(INPUT, sample_rate)),
            hidmp,
            record: Biquad::new(hidmp[5]),
            smoothing: Biquad::new(common::rom_biquad(SMOOTHING, sample_rate)),
            feedback: common::setter(0.0, sample_rate),
            wet: common::setter(0.0, sample_rate),
            level: common::setter(1.0, sample_rate),
            target: 0.0,
            position: 0.0,
            glide: common::smoothing_at(q31(GLIDE), sample_rate),
            phase: 0,
            increment: (LFO_INCREMENT as f32 / ratio).round() as u32,
            depth,
            feedback_sample: 0.0,
            hidmp_index: 5,
            started: false,
        }
    }

    pub fn wet_gain(mix: f32) -> f32 {
        zoom::curve(0.8, 0.0, 101, common::param(mix, 100.0), 5)
    }

    pub fn feedback_gain(feedback: f32) -> f32 {
        zoom::curve(f(0x3f733333), 0.0, 101, common::param(feedback, 100.0), 1)
    }
}

impl Effect for TapeEcho {
    fn set(&mut self, values: &Values, bpm: f32) {
        let ms = zoom::delay_time_ms(
            common::param(values[0], 2014.0),
            TIME_THRESHOLD,
            bpm,
            MAX_MS,
        );
        self.target = common::ms_to_samples(ms, self.sample_rate);
        let feedback = Self::feedback_gain(values[1]);
        let wet = Self::wet_gain(values[2]);
        let level = common::param(values[4], 150.0) / 100.0;
        let hidmp = common::index(values[3], 10);
        if hidmp != self.hidmp_index {
            self.hidmp_index = hidmp;
            self.record.coefficients = self.hidmp[hidmp];
        }
        self.feedback.set(feedback);
        self.wet.set(wet);
        self.level.set(level);
        if !self.started {
            self.started = true;
            self.position = self.target;
            self.feedback.jump(feedback);
            self.wet.jump(wet);
            self.level.jump(level);
        }
    }

    #[inline]
    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let feedback = self.feedback.tick();
        let wet = self.wet.tick();
        let level = self.level.tick();
        let input = self.input.process(0.5 * (left + right));
        let stored = self.record.process(self.feedback_sample * feedback + input);
        self.line.write(stored);
        self.position += (self.target - self.position) * self.glide;
        self.phase = self.phase.wrapping_add(self.increment);
        let triangle = (self.phase as i32).unsigned_abs() as f32 * (1.0 / 2_147_483_648.0);
        let tap = self.line.read_linear(self.position + triangle * self.depth);
        self.feedback_sample = flush(tap);
        let echo = self.smoothing.process(tap) * wet;
        [level * (left + echo), level * (right + echo)]
    }

    fn clear(&mut self) {
        self.line.clear();
        self.input.clear();
        self.record.clear();
        self.smoothing.clear();
        self.feedback_sample = 0.0;
        self.position = self.target;
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(TapeEcho::new(sample_rate))
}

#[cfg(test)]
mod tests {
    use super::super::common::testing::*;
    use super::*;
    use crate::Model;

    fn defaults() -> Values {
        Model::from_key(INFO.key).unwrap().defaults()
    }

    #[test]
    fn echo_arrives_near_the_time_plus_wow() {
        for sample_rate in [44_100.0, 96_000.0] {
            let mut effect = TapeEcho::new(sample_rate);
            effect.set(
                &with(defaults(), &[(0, 199.0), (1, 0.0), (2, 100.0)]),
                120.0,
            );
            let out = render(&mut effect, (0.3 * sample_rate) as usize, impulse);
            let at = peak_index(&out, 0, 1) as f32;
            let expected = 0.2 * sample_rate;
            // The wow triangle starts at zero and adds at most 3.2 ms.
            assert!(
                at >= expected - 2.0 && at <= expected + 0.004 * sample_rate,
                "{at}"
            );
            assert_eq!(out[0][0], 1.0);
        }
    }

    #[test]
    fn time_changes_glide() {
        let sample_rate = 48_000.0;
        let mut effect = TapeEcho::new(sample_rate);
        effect.set(&with(defaults(), &[(0, 99.0)]), 120.0);
        effect.set(&with(defaults(), &[(0, 999.0)]), 120.0);
        let start = effect.position;
        render(&mut effect, (0.12 * sample_rate) as usize, |_| [0.0; 2]);
        let moved = (effect.position - start) / (effect.target - start);
        // A one-pole of ~122 ms: about 63% after its time constant.
        assert!(moved > 0.55 && moved < 0.7, "{moved}");
    }

    #[test]
    fn wow_rate_and_depth_follow_the_rom() {
        let effect = TapeEcho::new(48_000.0);
        let hz = effect.increment as f32 * 48_000.0 / 4_294_967_296.0;
        assert!((hz - 0.3562).abs() < 1e-3, "{hz}");
        assert!((effect.depth / 48.0 - 141.69 / 44.1).abs() < 0.01);
    }

    #[test]
    fn editor_curves() {
        assert!((TapeEcho::feedback_gain(100.0) - 0.95).abs() < 1e-6);
        assert!((TapeEcho::feedback_gain(50.0) - 0.2375).abs() < 1e-6);
        assert!((TapeEcho::wet_gain(100.0) - 0.8).abs() < 1e-6);
    }

    #[test]
    fn maximum_feedback_stays_bounded_and_decays() {
        let mut effect = TapeEcho::new(44_100.0);
        effect.set(
            &with(defaults(), &[(0, 20.0), (1, 100.0), (3, 10.0)]),
            120.0,
        );
        let out = render(&mut effect, 441_000, |i| {
            if i < 4410 { [0.9, 0.9] } else { [0.0; 2] }
        });
        assert!(out.iter().all(|f| f[0].abs() < 4.0));
        assert!(rms(&out, 0, 400_000) < 1e-3);
    }
}
