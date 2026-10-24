//! Delay (DELAY.ZDL). The shared DELAY/ANLGDLY callback (see `feedback.rs`)
//! with DELAY's ROM: identity input and feedback filters, so only the HiDMP
//! tone biquad colours the repeats (HiDMP 10 is ~20 kHz, 0 is ~250 Hz), and
//! F.B up to 0.95. Editors decoded completely; see `feedback.rs`.
use super::feedback::{FeedbackDelay, TIME_THRESHOLD, Voicing};
use crate::{Effect, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_delay",
    name: "Delay",
    category: Category::Delay,
    zdl: "DELAY.ZDL",
    description: "Digital delay up to 4 s with HiDMP and ping-pong",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Time",
            max: 4022,
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
            default: 30,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 70,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "HiDMP",
            max: 10,
            default: 10,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "P-P",
            max: 1,
            default: 0,
            display: Display::OFF_ON,
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
    ],
};

/// ROM 0x80000460: eleven first-order low-passes, ~250 Hz (0) to ~20 kHz (10).
const HIDMP: [[u32; 5]; 11] = [
    [0x3c8f5b79, 0x3c8f5b79, 0x00000000, 0x3f770a48, 0x00000000],
    [0x3d0cef57, 0x3d0cef57, 0x00000000, 0x3f6e6215, 0x00000000],
    [0x3d88689d, 0x3d88689d, 0x00000000, 0x3f5de5d9, 0x00000000],
    [0x3e00767d, 0x3e00767d, 0x00000000, 0x3f3fc4c2, 0x00000000],
    [0x3e67ff0a, 0x3e67ff0a, 0x00000000, 0x3f0c007b, 0x00000000],
    [0x3e8ad692, 0x3e8ad692, 0x00000000, 0x3eea52dc, 0x00000000],
    [0x3ea03bca, 0x3ea03bca, 0x00000000, 0x3ebf886c, 0x00000000],
    [0x3eb48431, 0x3eb48431, 0x00000000, 0x3e96f79f, 0x00000000],
    [0x3ec7f697, 0x3ec7f697, 0x00000000, 0x3e6025a5, 0x00000000],
    [0x3eed461d, 0x3eed461d, 0x00000000, 0x3d95cf1b, 0x00000000],
    [0x3f5f2ca7, 0x3f5f2ca7, 0x00000000, 0xbf3e594e, 0x00000000],
];

const VOICING: Voicing = Voicing {
    analog: None,
    hidmp: &HIDMP,
    // fb_edit: curve maximum 0x3f733333.
    feedback_max: 0.95,
};

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(FeedbackDelay::new(sample_rate, &VOICING))
}

#[cfg(test)]
mod tests {
    use super::super::common::testing::*;
    use super::*;
    use crate::Model;

    fn defaults() -> crate::Values {
        Model::from_key(INFO.key).unwrap().defaults()
    }

    #[test]
    fn first_echo_arrives_after_time_plus_one_ms() {
        for sample_rate in [44_100.0, 48_000.0, 96_000.0] {
            let mut effect = create(sample_rate);
            // Time 99 -> 100 ms, no feedback, Mix 80 -> wet 1.0, HiDMP 10.
            effect.set(&with(defaults(), &[(0, 99.0), (1, 0.0), (2, 80.0)]), 120.0);
            let out = render(effect.as_mut(), (sample_rate * 0.2) as usize, impulse);
            let expected = (0.1 * sample_rate) as usize;
            let at = peak_index(&out, 0, 1);
            assert!(
                at.abs_diff(expected) <= 1,
                "{sample_rate}: {at} vs {expected}"
            );
            assert!((out[at][0] - 1.0).abs() < 1e-3, "{}", out[at][0]);
            // Dry passes at unity.
            assert!((out[0][0] - 1.0).abs() < 1e-6);
        }
    }

    #[test]
    fn mix_curve_follows_the_editor() {
        assert_eq!(FeedbackDelay::mix_gain(0.0), 0.0);
        assert!((FeedbackDelay::mix_gain(40.0) - 0.25).abs() < 1e-6);
        assert!((FeedbackDelay::mix_gain(80.0) - 1.0).abs() < 1e-6);
        assert!((FeedbackDelay::mix_gain(100.0) - (1.0 + (20.0f32 / 19.0).powi(2))).abs() < 1e-5);
    }

    #[test]
    fn ping_pong_puts_the_half_time_tap_on_the_right() {
        let sample_rate = 48_000.0;
        let mut effect = create(sample_rate);
        effect.set(
            &with(defaults(), &[(0, 199.0), (1, 0.0), (2, 80.0), (4, 1.0)]),
            120.0,
        );
        let out = render(effect.as_mut(), 15_000, impulse);
        let left = peak_index(&out, 0, 1);
        let right = peak_index(&out, 1, 1);
        assert!(left.abs_diff(9600) <= 1, "{left}");
        assert!(right.abs_diff(4800) <= 1, "{right}");
    }

    #[test]
    fn tempo_sync_uses_the_note_time() {
        let sample_rate = 48_000.0;
        let mut effect = create(sample_rate);
        // 4000 + 6 = a quarter note: 500 ms at 120 BPM.
        effect.set(
            &with(defaults(), &[(0, 4006.0), (1, 0.0), (2, 80.0)]),
            120.0,
        );
        let out = render(effect.as_mut(), 30_000, impulse);
        assert!(peak_index(&out, 0, 1).abs_diff(24_000) <= 1);
    }

    #[test]
    fn time_edit_mutes_and_reopens_after_the_new_time() {
        let sample_rate = 48_000.0;
        let mut effect = create(sample_rate);
        effect.set(&with(defaults(), &[(0, 99.0), (1, 90.0)]), 120.0);
        render(effect.as_mut(), 48_000, sine(220.0, 0.5, sample_rate));
        effect.set(&with(defaults(), &[(0, 299.0), (1, 90.0)]), 120.0);
        // Silence in: after the 16 ms fade the wet path stays closed for 301 ms.
        let out = render(effect.as_mut(), 20_000, |_| [0.0, 0.0]);
        assert!(out[2_000..14_000].iter().all(|f| f[0].abs() < 1e-6));
    }

    #[test]
    fn maximum_feedback_decays() {
        let sample_rate = 44_100.0;
        let mut effect = create(sample_rate);
        effect.set(
            &with(defaults(), &[(0, 9.0), (1, 100.0), (2, 100.0)]),
            120.0,
        );
        let out = render(effect.as_mut(), 441_000, |i| {
            if i < 4410 { [0.8, 0.8] } else { [0.0, 0.0] }
        });
        assert!(out.iter().all(|f| f[0].abs() < 16.0 && f[1].abs() < 16.0));
        assert!(rms(&out, 0, 400_000) < 1e-3);
    }
}
