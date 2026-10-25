//! StompDly (STOMPDLY.ZDL), from the recovered `Fx_DLY_StompDly`
//! (`stomp_delay.rs`) and the decoded editors.
//!
//! Callback: the half-sum passes a DC blocker [15..19]; the previous tap
//! times F.B [12] is hard-clipped to ±1 and DC-blocked [25..29]; their sum
//! passes the HiDMP biquad [20..24] into the ring. The read position is a Q13
//! offset that follows the Time target [11] through a one-pole (1.856e-4,
//! ~120 ms), so time changes glide; the ROM leaves its LFO [49..53] and the
//! three output biquads [30..44] inactive. Output: Mono mode sends
//! `mono + E.LVL·echo` to both sides (the input is summed to mono); Stereo
//! mode sends `E.LVL·echo` left and the dry mono sum right.
//!
//! Editors: Time is `v + 1` ms when Sync is Off, otherwise Sync selects a note
//! (limited to 4000 ms); live edits move only the target (glide). E.LVL `v/100`,
//! F.B `1.1·(v/100)²` (self-oscillates above ~95, bounded by the clip), HiDMP
//! one of ten second-order low-passes or flat (10). Changing Mode fades the
//! output out and back in over 16 ms each way.
use super::common::{self, Biquad, DelayLine, Fade, Ramp, f, flush, q31};
use crate::{Effect, Values, catalog::*, dsp::Coefficients, zoom};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_stomp_dly",
    name: "StompDly",
    category: Category::Delay,
    zdl: "STOMPDLY.ZDL",
    description: "Stompbox delay; high F.B self-oscillates",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "E.LVL",
            max: 120,
            default: 30,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "F.B",
            max: 100,
            default: 20,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Time",
            max: 599,
            default: 359,
            display: Display::number(1.0, 1.0, 0, " ms"),
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Sync",
            max: 15,
            default: 0,
            display: Display::labels(&common::SYNC_LABELS),
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Mode",
            max: 1,
            default: 0,
            display: Display::labels(&["Mono", "Stereo"]),
        },
        // values[5] is firmware parameter word 8
        ParamInfo {
            name: "HiDMP",
            max: 10,
            default: 5,
            display: Display::INT,
        },
    ],
};

const MAX_MS: f32 = 4000.0;

/// DC blocker used for the input [15..19] and the feedback [25..29].
const DC_BLOCKER: [u32; 5] = [0x3f7fa2c2, 0xbf7fa2c2, 0x00000000, 0x3f7f4585, 0x00000000];
/// Time glide [47] (pole [48]).
const GLIDE: u32 = 0x00061546;

/// ROM 0x80000488: HiDMP biquads copied to [20..24].
const HIDMP: [[u32; 5]; 11] = [
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
    [0x3f800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000],
];

pub struct StompDly {
    sample_rate: f32,
    line: DelayLine,
    input: Biquad,
    feedback_filter: Biquad,
    hidmp: [Coefficients; 11],
    record: Biquad,
    target: f32,
    position: f32,
    glide: f32,
    tap: f32,
    feedback: Ramp,
    effect_level: Ramp,
    output: Fade,
    ramp: f32,
    stereo: bool,
    pending_stereo: bool,
    hidmp_index: usize,
    started: bool,
}

impl StompDly {
    pub fn new(sample_rate: f32) -> Self {
        let hidmp = HIDMP.map(|w| common::rom_biquad(w, sample_rate));
        let dc = common::rom_biquad(DC_BLOCKER, sample_rate);
        Self {
            sample_rate,
            line: common::line_for_ms(MAX_MS, 2, sample_rate),
            input: Biquad::new(dc),
            feedback_filter: Biquad::new(dc),
            hidmp,
            record: Biquad::new(hidmp[5]),
            target: 1.0,
            position: 1.0,
            glide: common::smoothing_at(q31(GLIDE), sample_rate),
            tap: 0.0,
            feedback: common::setter(0.0, sample_rate),
            effect_level: common::setter(0.0, sample_rate),
            output: Fade::new(1.0),
            ramp: common::TAP_RAMP_44K * common::rate_ratio(sample_rate),
            stereo: false,
            pending_stereo: false,
            hidmp_index: 5,
            started: false,
        }
    }

    pub fn time_ms(time: f32, sync: f32, bpm: f32) -> f32 {
        let sync = common::index(sync, 15);
        if sync == 0 {
            common::param(time, 599.0) + 1.0
        } else {
            zoom::delay_note_ms(sync - 1, bpm, MAX_MS)
        }
    }

    pub fn feedback_gain(value: f32) -> f32 {
        zoom::curve(f(0x3f8ccccd), 0.0, 101, common::param(value, 100.0), 1)
    }
}

impl Effect for StompDly {
    fn set(&mut self, values: &Values, bpm: f32) {
        let effect_level = common::param(values[0], 120.0) / 100.0;
        let feedback = Self::feedback_gain(values[1]);
        let ms = Self::time_ms(values[2], values[3], bpm);
        let stereo = common::param(values[4], 1.0) >= 0.5;
        let hidmp = common::index(values[5], 10);
        self.target = common::ms_to_samples(ms, self.sample_rate).min(self.line.capacity() as f32);
        if hidmp != self.hidmp_index {
            self.hidmp_index = hidmp;
            self.record.coefficients = self.hidmp[hidmp];
        }
        self.feedback.set(feedback);
        self.effect_level.set(effect_level);
        if !self.started {
            self.started = true;
            self.position = self.target;
            self.feedback.jump(feedback);
            self.effect_level.jump(effect_level);
            self.stereo = stereo;
            self.pending_stereo = stereo;
        } else if stereo != self.pending_stereo {
            // mode_edit: fade the output out, switch, fade back in.
            self.pending_stereo = stereo;
            self.output.start(0.0, self.ramp);
        }
    }

    #[inline]
    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let feedback = self.feedback.tick();
        let effect_level = self.effect_level.tick();
        let mono = 0.5 * (left + right);
        let input = self.input.process(mono);
        let fed = self
            .feedback_filter
            .process((self.tap * feedback).clamp(-1.0, 1.0));
        self.line.write(self.record.process(input + fed));
        self.position += (self.target - self.position) * self.glide;
        self.tap = flush(self.line.read_linear(self.position));
        let echo = self.tap * effect_level;
        if self.stereo != self.pending_stereo && self.output.current == 0.0 {
            self.stereo = self.pending_stereo;
            self.output.start(1.0, self.ramp);
        }
        let gain = self.output.tick();
        if self.stereo {
            [gain * echo, gain * mono]
        } else {
            let mixed = gain * (mono + echo);
            [mixed, mixed]
        }
    }

    fn clear(&mut self) {
        self.line.clear();
        self.input.clear();
        self.feedback_filter.clear();
        self.record.clear();
        self.tap = 0.0;
        self.position = self.target;
        self.stereo = self.pending_stereo;
        self.output.jump(1.0);
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(StompDly::new(sample_rate))
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
    fn echo_time_and_mono_sum() {
        let sample_rate = 48_000.0;
        let mut effect = StompDly::new(sample_rate);
        // E.LVL 100, no feedback, Time 99 -> 100 ms, HiDMP flat.
        effect.set(
            &with(defaults(), &[(0, 100.0), (1, 0.0), (2, 99.0), (5, 10.0)]),
            120.0,
        );
        let out = render(&mut effect, 9600, |i| {
            if i == 0 { [1.0, 0.0] } else { [0.0; 2] }
        });
        assert_eq!(out[0], [0.5, 0.5]);
        let at = peak_index(&out, 0, 1);
        assert!(at.abs_diff(4800) <= 1, "{at}");
        assert!((out[at][0] - 0.5).abs() < 0.02, "{}", out[at][0]);
    }

    #[test]
    fn stereo_mode_splits_wet_and_dry() {
        let sample_rate = 44_100.0;
        let mut effect = StompDly::new(sample_rate);
        effect.set(
            &with(
                defaults(),
                &[(0, 100.0), (1, 0.0), (2, 9.0), (4, 1.0), (5, 10.0)],
            ),
            120.0,
        );
        let out = render(&mut effect, 1000, impulse);
        assert_eq!(out[0], [0.0, 1.0]);
        assert!(out[441][0] > 0.9 && out[441][1] == 0.0);
    }

    #[test]
    fn sync_selects_notes() {
        assert_eq!(StompDly::time_ms(359.0, 0.0, 120.0), 360.0);
        assert!((StompDly::time_ms(0.0, 7.0, 120.0) - 500.0).abs() < 1e-3);
        assert!((StompDly::time_ms(0.0, 15.0, 120.0) - 4000.0).abs() < 1e-2);
    }

    #[test]
    fn self_oscillation_is_bounded() {
        let mut effect = StompDly::new(44_100.0);
        assert!((StompDly::feedback_gain(100.0) - 1.1).abs() < 1e-6);
        effect.set(
            &with(defaults(), &[(0, 120.0), (1, 100.0), (2, 50.0), (5, 10.0)]),
            120.0,
        );
        let out = render(&mut effect, 441_000, |i| {
            if i < 2000 { [0.9, 0.9] } else { [0.0; 2] }
        });
        assert!(out.iter().all(|f| f[0].abs() < 2.0));
        // Still ringing after ten seconds.
        assert!(rms(&out, 0, 400_000) > 0.05);
    }
}
