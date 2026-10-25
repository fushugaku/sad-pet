//! StereoDly (STDELAY.ZDL), from the recovered `Fx_DLY_StereoDly`
//! (`stereo_delay.rs`) and the decoded editors.
//!
//! Callback: two independent rings of 2 s. Each channel feeds its own input
//! plus its previous tap times its F.B [12/13] into its ring; the tap is read
//! at an integer offset [40/41] and weighted by its gate [5/6] and the common
//! tap gain [3]. All input, feedback and output filters of the ROM image are
//! identities, so this is a clean dual delay. Output: `Level·(dry + wet·tap)`
//! per channel.
//!
//! Editors: TimeL/TimeR `ms·44.1` samples (sync notes limited to 2000 ms); a
//! live edit mutes that channel's tap in 16 ms, changes the offset and reopens
//! it `ms + 10` ms later. LchFB/RchFB `0.95·v/100`. The wet gains multiply two
//! Main power-1.41 curves: `[9] = (LchLv/100)^1.41·(Mix/100)^1.41` and likewise
//! [10] for the right channel. Level `v/100`.
use super::common::{self, DelayLine, Gate, Ramp, flush};
use crate::{Effect, Values, catalog::*, zoom};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_stereo_dly",
    name: "StereoDly",
    category: Category::Delay,
    zdl: "STDELAY.ZDL",
    description: "Two independent delays with their own time and feedback",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "TimeL",
            max: 2014,
            default: 164,
            display: Display::DelayTime {
                threshold: TIME_THRESHOLD,
                offset: 1,
            },
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "TimeR",
            max: 2014,
            default: 504,
            display: Display::DelayTime {
                threshold: TIME_THRESHOLD,
                offset: 1,
            },
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 100,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "LchFB",
            max: 100,
            default: 55,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "RchFB",
            max: 100,
            default: 37,
            display: Display::INT,
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
        // values[6] is firmware parameter word 8
        ParamInfo {
            name: "LchLv",
            max: 100,
            default: 100,
            display: Display::INT,
        },
        // values[7] is firmware parameter word 9
        ParamInfo {
            name: "RchLv",
            max: 100,
            default: 100,
            display: Display::INT,
        },
    ],
};

const TIME_THRESHOLD: u16 = 1999;
const MAX_MS: f32 = 2000.0;

struct Channel {
    line: DelayLine,
    gate: Gate,
    offset: usize,
    pending: usize,
    reopen: u32,
    muting: bool,
    time_ms: f32,
    feedback: Ramp,
    wet: Ramp,
    tap: f32,
}

impl Channel {
    fn new(sample_rate: f32) -> Self {
        Self {
            line: common::line_for_ms(MAX_MS, 2, sample_rate),
            gate: Gate::new(sample_rate, true),
            offset: 0,
            pending: 0,
            reopen: 0,
            muting: false,
            time_ms: -1.0,
            feedback: common::setter(0.0, sample_rate),
            wet: common::setter(0.0, sample_rate),
            tap: 0.0,
        }
    }

    fn set_time(&mut self, ms: f32, sample_rate: f32, live: bool) {
        if ms == self.time_ms {
            return;
        }
        self.time_ms = ms;
        self.pending = common::tap_samples(ms, sample_rate, &self.line);
        self.reopen = common::ms_to_samples(ms + 10.0, sample_rate) as u32;
        if live {
            self.muting = true;
            self.gate.mute();
        } else {
            self.offset = self.pending;
        }
    }

    #[inline]
    fn process(&mut self, input: f32) -> f32 {
        let feedback = self.feedback.tick();
        let wet = self.wet.tick();
        self.line.write(self.tap * feedback + input);
        if self.muting && self.gate.closed() {
            self.muting = false;
            self.offset = self.pending;
            self.gate.open_after(self.reopen);
        }
        self.tap = flush(self.gate.tick() * self.line.read(self.offset));
        self.tap * wet
    }

    fn clear(&mut self) {
        self.line.clear();
        self.tap = 0.0;
        if self.muting {
            self.muting = false;
            self.offset = self.pending;
        }
        self.gate.clear();
    }
}

pub struct StereoDly {
    sample_rate: f32,
    channels: [Channel; 2],
    level: Ramp,
    started: bool,
}

/// Main curve 5 (power 1.41) over 0..100.
fn power_curve(value: f32) -> f32 {
    zoom::curve(1.0, 0.0, 101, common::param(value, 100.0), 5)
}

impl Effect for StereoDly {
    fn set(&mut self, values: &Values, bpm: f32) {
        let live = self.started;
        let mix = power_curve(values[2]);
        let level = common::param(values[5], 150.0) / 100.0;
        self.level.set(level);
        for (channel, (time, (feedback, lv))) in self.channels.iter_mut().zip([
            (values[0], (values[3], values[6])),
            (values[1], (values[4], values[7])),
        ]) {
            let time = common::param(time, 2014.0).round();
            let ms = zoom::delay_time_ms(time, TIME_THRESHOLD, bpm, MAX_MS);
            channel.set_time(ms, self.sample_rate, live);
            let feedback = 0.95 * common::param(feedback, 100.0) / 100.0;
            let wet = power_curve(lv) * mix;
            channel.feedback.set(feedback);
            channel.wet.set(wet);
            if !live {
                channel.feedback.jump(feedback);
                channel.wet.jump(wet);
            }
        }
        if !live {
            self.level.jump(level);
            self.started = true;
        }
    }

    #[inline]
    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let level = self.level.tick();
        let wet_left = self.channels[0].process(left);
        let wet_right = self.channels[1].process(right);
        [level * (left + wet_left), level * (right + wet_right)]
    }

    fn clear(&mut self) {
        for channel in &mut self.channels {
            channel.clear();
        }
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(StereoDly {
        sample_rate,
        channels: [Channel::new(sample_rate), Channel::new(sample_rate)],
        level: common::setter(1.0, sample_rate),
        started: false,
    })
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
    fn channels_have_their_own_times() {
        let sample_rate = 48_000.0;
        let mut effect = create(sample_rate);
        effect.set(&with(defaults(), &[(3, 0.0), (4, 0.0)]), 120.0);
        let out = render(effect.as_mut(), 30_000, impulse);
        // 164 -> 165 ms, 504 -> 505 ms.
        assert_eq!(peak_index(&out, 0, 1), 7920);
        assert_eq!(peak_index(&out, 1, 1), 24_240);
        assert!((out[7920][0] - 1.0).abs() < 1e-6);
    }

    #[test]
    fn feedback_repeats_on_the_same_channel() {
        let sample_rate = 44_100.0;
        let mut effect = create(sample_rate);
        effect.set(&with(defaults(), &[(0, 99.0), (3, 50.0), (4, 0.0)]), 120.0);
        let out = render(effect.as_mut(), 10_000, impulse);
        // The previous frame's tap feeds back: repeats every T + 1 samples.
        let t = 4410;
        assert!(
            (out[2 * t + 1][0] - 0.475).abs() < 1e-4,
            "{}",
            out[2 * t + 1][0]
        );
        assert!(out[2 * t + 1][1].abs() < 1e-6);
    }

    #[test]
    fn levels_multiply_power_curves() {
        let sample_rate = 44_100.0;
        let mut effect = create(sample_rate);
        effect.set(
            &with(defaults(), &[(0, 9.0), (2, 50.0), (3, 0.0), (6, 50.0)]),
            120.0,
        );
        let out = render(effect.as_mut(), 1000, impulse);
        let expected = 0.5f32.powf(1.41) * 0.5f32.powf(1.41);
        assert!((out[441][0] - expected).abs() < 1e-4);
    }

    #[test]
    fn maximum_feedback_decays() {
        let mut effect = create(44_100.0);
        effect.set(
            &with(defaults(), &[(0, 0.0), (1, 0.0), (3, 100.0), (4, 100.0)]),
            120.0,
        );
        let out = render(effect.as_mut(), 441_000, |i| {
            if i < 100 { [1.0, 1.0] } else { [0.0; 2] }
        });
        assert!(out.iter().all(|f| f[0].abs() < 16.0));
        assert!(rms(&out, 0, 400_000) < 1e-3);
    }
}
