//! MultiTapD (MLTTPDLY.ZDL), from the recovered `Fx_DLY_MultiTapD`
//! (`multi_tap_delay.rs`), its Tone table (`multi_tap_delay_parameters.rs`) and
//! the decoded Time/Pattern/sequencer editors.
//!
//! Callback: the half-sum passes a ~300 Hz first-order high-pass [5..7] and,
//! with the last tap times the pattern feedback [48], enters one ring. Eight
//! taps at `T/16 · pattern[k]` (T = Time in samples) are summed into the left
//! and right outputs with per-tap gains [31..38]/[39..46], scaled by the tap
//! gain [3], filtered by the first-order Tone section [11..13] per channel and
//! mixed: `Level·(dry + wet·tone)`. (The callback's second feedback bank
//! [15..18]/[27..30] and second write [49..53] are inert in the ROM image.)
//!
//! Editors: Time `ms·44.1` samples (sync notes limited to 3000 ms). A Time or
//! Pattern edit mutes [3] in 16 ms and zeroes the tap gains and feedback; then
//! `qTimeSet`/`onf_aft` open the taps one by one, each when its echo of new
//! material arrives (`(ms/16 + 1)·Δpattern` ms apart), with 16 ms ramps to the
//! pattern's ROM gains, and the pattern feedback after the last tap. Mix is
//! `v/100`, Level `v/100`, Tone one of eleven first-order sections (darker
//! 0..6, flat 7, thinner and brighter 8..10). Pattern ROM tables at
//! 0x800006d8 (positions), 0x80000358 (gains) and 0x80000ac0 (feedback).
use super::common::{self, Biquad, DelayLine, Fade, Ramp, f, flush};
use crate::{Effect, Values, catalog::*, dsp::Coefficients, zoom};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_multi_tap_d",
    name: "MultiTapD",
    category: Category::Delay,
    zdl: "MLTTPDLY.ZDL",
    description: "Eight-tap rhythmic delay with stereo tap patterns",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Time",
            max: 3018,
            default: 2999,
            display: Display::DelayTime {
                threshold: TIME_THRESHOLD,
                offset: 1,
            },
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "PTTRN",
            max: 7,
            default: 1,
            display: Display::offset(1.0),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 20,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Tone",
            max: 10,
            default: 10,
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

const TIME_THRESHOLD: u16 = 2999;
const MAX_MS: f32 = 3000.0;

/// Input high-pass [5..7] of ROM 0x800007f8.
const INPUT: [u32; 5] = [0x3f7aa482, 0xbf7aa482, 0x00000000, 0x3f754904, 0x00000000];

/// Tap positions in sixteenths of Time; word 0 is the start of the sequence.
const PATTERNS: [[u8; 9]; 8] = [
    [0, 2, 4, 6, 8, 10, 12, 14, 16],
    [0, 2, 4, 6, 8, 10, 12, 14, 16],
    [0, 2, 4, 6, 8, 10, 12, 14, 16],
    [0, 2, 4, 6, 8, 10, 12, 14, 16],
    [0, 2, 4, 6, 8, 10, 12, 14, 16],
    [0, 2, 4, 6, 8, 10, 12, 14, 16],
    [0, 2, 3, 4, 6, 7, 8, 12, 16],
    [0, 1, 3, 4, 5, 7, 8, 12, 16],
];

/// Left/right gain of each tap.
const GAINS: [[u32; 16]; 8] = [
    [
        0x3f570996, 0x3e75c28f, 0x3e75c28f, 0x3f570996, 0x3f6b83cf, 0x3df5c28f, 0x3df5c28f,
        0x3f6b83cf, 0x3df5c28f, 0x3df5c28f, 0x3f0104d5, 0x3e13721d, 0x3e13721d, 0x3f0104d5,
        0x3f1998f2, 0x3f1998f2,
    ],
    [
        0x3e6970f8, 0x3ddd1f60, 0x3e13721d, 0x3e9ba494, 0x3eeb8130, 0x3d75c28f, 0x3d936f7e,
        0x3f0d4f37, 0x3ece6d9c, 0x3edcc63f, 0x3f498549, 0x3c9d4952, 0x3cb0c88a, 0x3f62b607,
        0x3f1998f2, 0x3f1998f2,
    ],
    [
        0x3ee97636, 0x3e5d29dc, 0x3ec63dc5, 0x3c9d4952, 0x3ee97636, 0x3e5d29dc, 0x3f1998f2,
        0x3f1998f2, 0x3e5d29dc, 0x3ee97636, 0x3c9d4952, 0x3ec63dc5, 0x3e5d29dc, 0x3ee97636,
        0x3f1998f2, 0x3f1998f2,
    ],
    [
        0x3e5d29dc, 0x3ee97636, 0x3c9d4952, 0x3ec63dc5, 0x3e5d29dc, 0x3ee97636, 0x3f1998f2,
        0x3f1998f2, 0x3ee97636, 0x3e5d29dc, 0x3ec63dc5, 0x3c9d4952, 0x3ee97636, 0x3e5d29dc,
        0x3f1998f2, 0x3f1998f2,
    ],
    [
        0x3f77ce31, 0x3d4486ad, 0x3f5b2181, 0x3e5d29dc, 0x3f3e7579, 0x3ec49907, 0x3f25e354,
        0x3f07290b, 0x3f0104d5, 0x3f29fb40, 0x3eb851ec, 0x3f428d65, 0x3e449907, 0x3f5f396d,
        0x3d4486ad, 0x3f77ce31,
    ],
    [
        0x3d4486ad, 0x3f77ce31, 0x3e5d29dc, 0x3f5b2181, 0x3ec49907, 0x3f3e7579, 0x3f07290b,
        0x3f25e354, 0x3f29fb40, 0x3f0104d5, 0x3f428d65, 0x3eb851ec, 0x3f5f396d, 0x3e449907,
        0x3f77ce31, 0x3d4486ad,
    ],
    [
        0x3f800000, 0x00000000, 0x00000000, 0x3f800000, 0x3f1998f2, 0x3f1998f2, 0x3f800000,
        0x00000000, 0x00000000, 0x3f800000, 0x3f1998f2, 0x3f1998f2, 0x3f1998f2, 0x3f1998f2,
        0x3f1998f2, 0x3f1998f2,
    ],
    [
        0x3f800000, 0x00000000, 0x00000000, 0x3f800000, 0x3f1998f2, 0x3f1998f2, 0x3f800000,
        0x00000000, 0x00000000, 0x3f800000, 0x3f1998f2, 0x3f1998f2, 0x3f1998f2, 0x3f1998f2,
        0x3f1998f2, 0x3f1998f2,
    ],
];

/// Feedback from the last tap, per pattern.
const FEEDBACK: [u32; 8] = [
    0x00000000, 0x3d800000, 0x00000000, 0x3e000000, 0x00000000, 0x3e000000, 0x00000000, 0x3dc00000,
];

/// `TONE_COEFFICIENTS` (ROM 0x80000a38): `b0 b1 a1` of a first-order section.
const TONE: [[u32; 3]; 11] = [
    [0x3cca7f62, 0x3cca7f62, 0x3f770a48],
    [0x3d316d33, 0x3d316d33, 0x3f6e6215],
    [0x3d990d92, 0x3d990d92, 0x3f5de5d9],
    [0x3e00767d, 0x3e00767d, 0x3f3fc4c2],
    [0x3e4fd2ef, 0x3e4fd2ef, 0x3f181689],
    [0x3e8ad692, 0x3e8ad692, 0x3eea52dc],
    [0x3ec7f697, 0x3ec7f697, 0x3e6025a5],
    [0x3f7fba0c, 0xbf7fba0c, 0x3f7f7417],
    [0x3f7b8524, 0xbf7b8524, 0x3f770a48],
    [0x3fab0bd7, 0xbfab0bd7, 0x3f645dab],
    [0x3fdf5a9c, 0xbfdf5a9c, 0x3f3fc4c2],
];

pub struct MultiTapD {
    sample_rate: f32,
    line: DelayLine,
    input: Biquad,
    tones: [Coefficients; 11],
    post: [Biquad; 2],
    positions: [usize; 8],
    pending: [usize; 8],
    left_gains: [f32; 8],
    right_gains: [f32; 8],
    fades: [[Fade; 2]; 8],
    feedback: Fade,
    tap: Fade,
    /// Samples during which a fade may still be moving.
    fading: u32,
    ramp: f32,
    muting: bool,
    /// Next tap to open and the samples until it opens.
    step: usize,
    countdown: u32,
    sixteenth_ms: u32,
    pattern: usize,
    wet: Ramp,
    level: Ramp,
    key: (u32, usize),
    tone_index: usize,
    started: bool,
}

impl MultiTapD {
    pub fn new(sample_rate: f32) -> Self {
        let tones = TONE.map(|[b0, b1, a1]| common::rom_biquad([b0, b1, 0, a1, 0], sample_rate));
        Self {
            sample_rate,
            line: common::line_for_ms(MAX_MS, 2, sample_rate),
            input: Biquad::new(common::rom_biquad(INPUT, sample_rate)),
            tones,
            post: [Biquad::new(tones[10]); 2],
            positions: [1; 8],
            pending: [1; 8],
            left_gains: [0.0; 8],
            right_gains: [0.0; 8],
            fades: [[Fade::new(0.0); 2]; 8],
            feedback: Fade::new(0.0),
            tap: Fade::new(0.0),
            fading: 0,
            ramp: common::TAP_RAMP_44K * common::rate_ratio(sample_rate),
            muting: false,
            step: 8,
            countdown: 0,
            sixteenth_ms: 1,
            pattern: 1,
            wet: common::setter(0.0, sample_rate),
            level: common::setter(1.0, sample_rate),
            key: (u32::MAX, usize::MAX),
            tone_index: 10,
            started: false,
        }
    }

    /// Delay before tap `step` opens, counted from the previous one.
    fn interval(&self, step: usize) -> u32 {
        let pattern = &PATTERNS[self.pattern];
        let sixteenths = u32::from(pattern[step + 1] - pattern[step]);
        common::ms_to_samples((self.sixteenth_ms * sixteenths) as f32, self.sample_rate) as u32
    }

    fn mute(&mut self) {
        self.tap.start(0.0, self.ramp);
        self.feedback.jump(0.0);
        for pair in &mut self.fades {
            pair[0].jump(0.0);
            pair[1].jump(0.0);
        }
        self.left_gains = [0.0; 8];
        self.right_gains = [0.0; 8];
        self.step = 8;
        self.countdown = 0;
        self.muting = true;
        self.fading = self.ramp as u32 + 2;
    }

    /// `onf_aft`: open tap `step` (and the tap gain, feedback at the ends).
    fn open_step(&mut self) {
        let step = self.step;
        let gains = &GAINS[self.pattern];
        if step == 0 {
            self.tap.start(1.0, self.ramp);
        }
        self.fades[step][0].start(f(gains[2 * step]), self.ramp);
        self.fades[step][1].start(f(gains[2 * step + 1]), self.ramp);
        if step == 7 {
            self.feedback.start(f(FEEDBACK[self.pattern]), self.ramp);
        }
        self.fading = self.ramp as u32 + 2;
        self.step += 1;
        if self.step < 8 {
            self.countdown = self.interval(self.step).max(1);
        }
    }

    #[inline]
    fn advance_sequence(&mut self) {
        if self.muting && self.tap.idle() {
            self.muting = false;
            self.positions = self.pending;
            self.step = 0;
            self.countdown = self.interval(0).max(1);
        }
        if self.step < 8 {
            self.countdown -= 1;
            if self.countdown == 0 {
                self.open_step();
            }
        }
        if self.fading > 0 {
            self.fading -= 1;
            self.tap.tick();
            self.feedback.tick();
            for (k, pair) in self.fades.iter_mut().enumerate() {
                self.left_gains[k] = pair[0].tick();
                self.right_gains[k] = pair[1].tick();
            }
        }
    }
}

impl Effect for MultiTapD {
    fn set(&mut self, values: &Values, bpm: f32) {
        let time = common::param(values[0], 3018.0).round();
        let ms = zoom::delay_time_ms(time, TIME_THRESHOLD, bpm, MAX_MS);
        let pattern = common::index(values[1], 7);
        let tone = common::index(values[3], 10);
        let wet = common::param(values[2], 100.0) / 100.0;
        let level = common::param(values[4], 150.0) / 100.0;
        if tone != self.tone_index {
            self.tone_index = tone;
            for post in &mut self.post {
                post.coefficients = self.tones[tone];
            }
        }
        self.wet.set(wet);
        self.level.set(level);
        let key = (ms.to_bits(), pattern);
        if key != self.key {
            self.key = key;
            self.pattern = pattern;
            // time_edit: positions are `(T >> 4) · pattern`, T = ms·44.1 samples.
            let unit = (common::ms_to_samples(ms, self.sample_rate) / 16.0).floor() as usize;
            let capacity = self.line.capacity();
            for (k, position) in self.pending.iter_mut().enumerate() {
                *position = (unit * usize::from(PATTERNS[pattern][k + 1])).clamp(1, capacity);
            }
            self.sixteenth_ms = (ms as u32 >> 4) + 1;
            self.mute();
        }
        if !self.started {
            self.started = true;
            self.wet.jump(wet);
            self.level.jump(level);
        }
    }

    #[inline]
    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        self.advance_sequence();
        let wet = self.wet.tick();
        let level = self.level.tick();
        let input = self.input.process(0.5 * (left + right));
        // The last tap of the previous frame: age `positions[7] + 1` before this write.
        let last = self.line.read(self.positions[7]);
        self.line.write(flush(last * self.feedback.current + input));
        let (mut sum_left, mut sum_right) = (0.0, 0.0);
        for k in 0..8 {
            let x = self.line.read(self.positions[k]);
            sum_left += self.left_gains[k] * x;
            sum_right += self.right_gains[k] * x;
        }
        let gain = self.tap.current;
        let post_left = self.post[0].process(sum_left * gain);
        let post_right = self.post[1].process(sum_right * gain);
        [
            level * (left + post_left * wet),
            level * (right + post_right * wet),
        ]
    }

    fn clear(&mut self) {
        self.line.clear();
        self.input.clear();
        for post in &mut self.post {
            post.clear();
        }
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(MultiTapD::new(sample_rate))
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
    fn taps_follow_the_pattern_and_open_in_sequence() {
        let sample_rate = 48_000.0;
        let mut effect = MultiTapD::new(sample_rate);
        // Time 1599 -> 1600 ms: sixteenth = 100 ms; pattern 7: 1 3 4 5 7 8 12 16.
        effect.set(
            &with(defaults(), &[(0, 1599.0), (1, 7.0), (2, 100.0), (3, 7.0)]),
            120.0,
        );
        let expected: Vec<usize> = [1, 3, 4, 5, 7, 8, 12, 16]
            .iter()
            .map(|s| s * 4800)
            .collect();
        assert_eq!(effect.pending.to_vec(), expected);
        // Before the first tap opens nothing comes out of the wet path.
        let out = render(&mut effect, 4000, impulse);
        assert!(out[1..].iter().all(|f| f[0].abs() < 1e-6));
        render(&mut effect, 80_000, |_| [0.0; 2]);
        assert_eq!(effect.step, 8);
        assert!((effect.left_gains[0] - 1.0).abs() < 1e-6);
        assert!((effect.feedback.current - 0.09375).abs() < 1e-6);
    }

    #[test]
    fn opened_taps_place_echoes_by_pattern() {
        let sample_rate = 44_100.0;
        let mut effect = MultiTapD::new(sample_rate);
        // Time 159 -> 160 ms: sixteenth 441 samples (rounded down: 7056 >> 4).
        effect.set(
            &with(defaults(), &[(0, 159.0), (1, 6.0), (2, 100.0), (3, 7.0)]),
            120.0,
        );
        render(&mut effect, 20_000, |_| [0.0; 2]);
        let out = render(&mut effect, 8000, impulse);
        // Pattern 6, tap 0 at 2/16: left gain 1, right 0.
        let at = 2 * 441;
        assert!(out[at][0] > 0.9, "{}", out[at][0]);
        assert!(out[at][1].abs() < 0.01);
        // Tap 1 at 3/16 goes right.
        assert!(out[3 * 441][1] > 0.9);
    }

    #[test]
    fn tone_table_is_unity_or_brighter_at_high_frequencies() {
        let tones = TONE.map(|[b0, b1, a1]| common::rom_biquad([b0, b1, 0, a1, 0], 44_100.0));
        assert!((tones[6].magnitude(44_100.0, 20.0) - 1.0).abs() < 0.01);
        assert!(tones[0].magnitude(44_100.0, 5000.0) < 0.1);
        assert!((tones[10].magnitude(44_100.0, 15_000.0) - 2.0).abs() < 0.1);
    }

    #[test]
    fn feedback_decays() {
        let mut effect = MultiTapD::new(44_100.0);
        effect.set(&with(defaults(), &[(0, 99.0), (1, 3.0), (2, 100.0)]), 120.0);
        let out = render(&mut effect, 220_500, |i| {
            if i < 2000 { [0.8, 0.8] } else { [0.0; 2] }
        });
        assert!(out.iter().all(|f| f[0].abs() < 16.0));
        assert!(rms(&out, 0, 200_000) < 1e-4);
    }
}
