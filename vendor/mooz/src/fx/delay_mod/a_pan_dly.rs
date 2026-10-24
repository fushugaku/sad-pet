//! A-Pan DLY (APANDLY.ZDL), `Fx_DLY_APanDLY`: the module's own AutoPan
//! (`Fx_SFX_AutoPan`) and stereo delay (`Fx_DLY_StereoDly`) in the order
//! Link selects (c7: 0 = "P-D" pan first, 1 = "D-P" delay first).
//!
//! - Delay, per channel: `c14 · previous tap + c13 · c23 · input` is
//!   written; the tap is read with linear interpolation at the smoothed
//!   offset s26, which moves towards c8 = `⌊441 · ms / 10⌋ << 14` through a
//!   Q31 one-pole (c12 = 0.999814) stepped twice per frame (once per
//!   channel), so a time change glides like tape (τ ≈ 61 ms). F.B is
//!   `c14 = F.B / 100` (`fb_edit`), up to 1.
//! - AutoPan: a Q31 phase s25 (start −0.5) advances by c17. Its triangle
//!   `|s25| − ½` is bent into a parabolic sine by c20 (±½, the Width side),
//!   multiplied by `16 · c22` and clipped to ±1 (Clip: c22 = 1/16 .. 1, square
//!   curve, `clip_edit`), giving `pan = c21 · (−½ + clipped / 2)`; the
//!   quarter-shifted triangle, ×4 and clipped (c19 = ¼), gives the amplitude
//!   envelope `1 + c18 · (−½ + clipped / 2)` (Depth: c18 = 1/256 .. 1,
//!   square curve, `depth_edit`). Weights: L = `(1 + pan) · env`,
//!   R = `(1 − c21 − pan) · env`.
//! - Rate is tied to the delay time (`time_edit`): one pan cycle takes
//!   ¼, ½, ¾ (Rate 0..2) or `Rate − 2` (Rate 3..52) delay times.
//! - Width (`width_edit`): "L50" .. "0" .. "R50"; the distance `n` from the
//!   centre sets the pan depth `c21 = 1 − (1 − 1/256) · (1 − n/50)²` (0 at the
//!   centre) and the side sets the direction c20 = ∓½.
//! - Mix (`mix_edit`): 0..50 raises the wet `c6 = Mix / 50` with the dry
//!   `c5 = 1`; 51..100 keeps the wet at 1 and lowers the dry to 0.
//!   Output `c2 · (c5 · dry + c6 · wet)`, Level `c2 = Level / 100`.
//!
//! Deviations: the ROM starts c20 at +1 and `width_edit` only rewrites it
//! when Width crosses the centre, so the pedal's very first L-side setting
//! pans with a doubled (pre-clipped) curve in the R direction; this port
//! always uses ∓½ for the L/R side. The tap mute around non-knob time
//! edits is not reproduced (time always glides). A soft limit above ±2 in
//! the loop keeps F.B 100 bounded. Integer ramps are 16 ms glides.
use super::common::Glide;
use crate::{
    Effect, Values,
    catalog::*,
    dsp::{DelayLine, flush, scale_samples},
    zoom::{self, DEFAULT_STEP, Ramp},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_a_pan_dly",
    name: "A-Pan DLY",
    category: Category::Delay,
    zdl: "APANDLY.ZDL",
    description: "Stereo delay with an auto-panner tied to the echo time",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Time",
            max: 2014,
            default: 222,
            display: Display::DelayTime {
                threshold: 1999,
                offset: 1,
            },
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "F.B",
            max: 100,
            default: 87,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 53,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Rate",
            max: 52,
            default: 26,
            display: Display::labels(&RATE_LABELS),
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Width",
            max: 100,
            default: 0,
            display: Display::labels(&WIDTH_LABELS),
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "Depth",
            max: 10,
            default: 7,
            display: Display::INT,
        },
        // values[6] is firmware parameter word 8
        ParamInfo {
            name: "Clip",
            max: 10,
            default: 0,
            display: Display::INT,
        },
        // values[7] is firmware parameter word 9
        ParamInfo {
            name: "Link",
            max: 1,
            default: 1,
            display: Display::labels(&["P-D", "D-P"]),
        },
        // values[8] is firmware parameter word 10
        ParamInfo {
            name: "Level",
            max: 200,
            default: 100,
            display: Display::INT,
        },
    ],
};

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(APanDly::new(sample_rate))
}

const TIME_THRESHOLD: u16 = 1999;
const MAX_MS: f32 = 2000.0;
/// c12: the delay-offset smoother, applied twice per frame.
const OFFSET_POLE: f32 = 0.999_814_36;
/// c19: the envelope triangle gain (×16 → ×4).
const ENVELOPE_SHAPE: f32 = 0.25;
/// Main's integer curve minimum for Depth and Width (0x007fffff).
const CURVE_FLOOR: f32 = 1.0 / 256.0;
/// Loop level above which the port (not the original) soft-limits.
const LOOP_LIMIT: f32 = 2.0;

/// The editor's delay in samples at 44.1 kHz: `⌊441 · ms / 10⌋`.
fn delay_samples_44k(time: f32, bpm: f32) -> f32 {
    let ms = if zoom::is_synced(time, TIME_THRESHOLD) {
        zoom::delay_note_ms(zoom::sync_index(time, TIME_THRESHOLD), bpm, MAX_MS).floor()
    } else {
        time.clamp(0.0, f32::from(TIME_THRESHOLD)) + 1.0
    };
    (441.0 * ms.max(1.0) / 10.0).floor()
}

/// Pan cycles per delay time for a Rate value (`time_edit`).
fn delay_times_per_cycle(rate: f32) -> f32 {
    let rate = rate.round().clamp(0.0, 52.0);
    if rate < 3.0 {
        0.25 * (rate + 1.0)
    } else {
        rate - 2.0
    }
}

/// Main's integer curve, shape 1 (square), from `start` at 0 to `end` at `count - 1`.
fn square_curve(start: f32, end: f32, count: f32, value: f32) -> f32 {
    let t = (value / (count - 1.0)).clamp(0.0, 1.0);
    start + (end - start) * t * t
}

/// Width → (pan depth c21, direction c20).
fn width_coefficients(width: f32) -> (f32, f32) {
    let width = width.clamp(0.0, 100.0);
    if (width - 50.0).abs() < 1e-3 {
        return (0.0, 0.5);
    }
    let (distance, side) = if width < 50.0 {
        (width, -0.5)
    } else {
        (100.0 - width, 0.5)
    };
    (square_curve(1.0, CURVE_FLOOR, 51.0, distance), side)
}

/// Mix → (dry c5, wet c6).
fn mix_coefficients(mix: f32) -> (f32, f32) {
    let mix = mix.clamp(0.0, 100.0);
    if mix < 51.0 {
        (1.0, mix / 50.0)
    } else {
        (1.0 - (mix - 51.0) / 49.0, 1.0)
    }
}

/// Fold a Q31-style phase in [-1, 1) to `|p| - 1/2`.
#[inline]
fn triangle(phase: f32) -> f32 {
    phase.abs() - 0.5
}

/// Wrap into [-1, 1).
#[inline]
fn wrap(phase: f32) -> f32 {
    if phase >= 1.0 { phase - 2.0 } else { phase }
}

#[inline]
fn soft_limit(x: f32) -> f32 {
    let magnitude = x.abs();
    if magnitude <= LOOP_LIMIT {
        x
    } else {
        let over = magnitude - LOOP_LIMIT;
        (LOOP_LIMIT + over / (1.0 + over)).copysign(x)
    }
}

struct APanDly {
    sample_rate: f32,
    lines: [DelayLine; 2],
    taps: [f32; 2],
    offset: f32,
    target: f32,
    pole: f32,
    max_offset: f32,
    /// Pan phase in [-1, 1) (the Q31 s25), increment per host sample.
    phase: f32,
    increment: f32,
    feedback: Ramp,
    level: Ramp,
    dry: Glide,
    wet: Glide,
    depth: Glide,
    clip: Glide,
    pan_depth: Glide,
    side: Glide,
    /// Link crossfade: 0 = pan first, 1 = delay first.
    order: Glide,
    previous: Option<(Values, f32)>,
}

impl APanDly {
    fn new(sample_rate: f32) -> Self {
        let lines = [
            DelayLine::with_seconds(MAX_MS / 1000.0 + 0.01, sample_rate),
            DelayLine::with_seconds(MAX_MS / 1000.0 + 0.01, sample_rate),
        ];
        let max_offset = lines[0].capacity() as f32 - 2.0;
        let mut effect = Self {
            sample_rate,
            lines,
            taps: [0.0; 2],
            offset: 1.0,
            target: 1.0,
            pole: OFFSET_POLE.powf(zoom::ORIGINAL_RATE / sample_rate),
            max_offset,
            phase: -0.5,
            increment: 0.0,
            feedback: Ramp::new(0.0, DEFAULT_STEP, sample_rate),
            level: Ramp::new(1.0, DEFAULT_STEP, sample_rate),
            dry: Glide::new(1.0, sample_rate),
            wet: Glide::new(0.0, sample_rate),
            depth: Glide::new(0.0, sample_rate),
            clip: Glide::new(1.0 / 16.0, sample_rate),
            pan_depth: Glide::new(0.0, sample_rate),
            side: Glide::new(-0.5, sample_rate),
            order: Glide::new(1.0, sample_rate),
            previous: None,
        };
        let defaults = crate::Model::from_key(INFO.key)
            .map(|model| model.defaults())
            .unwrap_or([0.0; crate::MAX_PARAMS]);
        effect.apply(&defaults, 120.0, true);
        effect
    }

    fn apply(&mut self, values: &Values, bpm: f32, jump: bool) {
        let previous = self.previous;
        let changed = |i: usize| previous.is_none_or(|(old, _)| old[i] != values[i]);
        let bpm_changed = previous.is_none_or(|(_, old)| old != bpm);
        let glide = |glide: &mut Glide, target: f32| {
            if jump {
                glide.jump(target);
            } else {
                glide.set(target);
            }
        };

        if changed(0) || changed(3) || bpm_changed {
            let samples = delay_samples_44k(values[0], bpm);
            self.target = scale_samples(samples, self.sample_rate).clamp(1.0, self.max_offset);
            if jump {
                self.offset = self.target;
            }
            // One pan cycle per `delay_times_per_cycle` delay times.
            let cycle_seconds = samples / zoom::ORIGINAL_RATE * delay_times_per_cycle(values[3]);
            self.increment = 2.0 / (cycle_seconds * self.sample_rate);
        }
        if changed(1) {
            let feedback = zoom::curve(1.0, 0.0, 101, values[1].clamp(0.0, 100.0), 0);
            if jump {
                self.feedback.jump(feedback);
            } else {
                self.feedback.set(feedback);
            }
        }
        if changed(2) {
            let (dry, wet) = mix_coefficients(values[2]);
            glide(&mut self.dry, dry);
            glide(&mut self.wet, wet);
        }
        if changed(4) {
            let (depth, side) = width_coefficients(values[4]);
            glide(&mut self.pan_depth, depth);
            glide(&mut self.side, side);
        }
        if changed(5) {
            glide(
                &mut self.depth,
                square_curve(CURVE_FLOOR, 1.0, 11.0, values[5]),
            );
        }
        if changed(6) {
            glide(
                &mut self.clip,
                square_curve(1.0 / 16.0, 1.0, 11.0, values[6]),
            );
        }
        if changed(7) {
            glide(&mut self.order, if values[7] >= 0.5 { 1.0 } else { 0.0 });
        }
        if changed(8) {
            let level = zoom::curve(2.0, 0.0, 201, values[8].clamp(0.0, 200.0), 0);
            if jump {
                self.level.jump(level);
            } else {
                self.level.set(level);
            }
        }
        self.previous = Some((*values, bpm));
    }

    /// `Fx_SFX_AutoPan` weights for this frame.
    #[inline]
    fn pan_weights(&mut self) -> [f32; 2] {
        self.phase = wrap(self.phase + self.increment);
        let centered = triangle(self.phase);
        let curved = 8.0 * self.side.tick() * centered * (centered.abs() - 1.0);
        let clipped = (16.0 * self.clip.tick() * curved).clamp(-1.0, 1.0);
        let pan_depth = self.pan_depth.tick();
        let pan = pan_depth * (-0.5 + 0.5 * clipped);
        let quarter = triangle(wrap(self.phase + 0.5));
        let shaped = (16.0 * ENVELOPE_SHAPE * quarter).clamp(-1.0, 1.0);
        let envelope = 1.0 + self.depth.tick() * (-0.5 + 0.5 * shaped);
        [(1.0 + pan) * envelope, (1.0 - pan_depth - pan) * envelope]
    }

    /// `Fx_DLY_StereoDly` for one frame.
    #[inline]
    fn delay(&mut self, input: [f32; 2]) -> [f32; 2] {
        let feedback = self.feedback.tick();
        let mut out = [0.0; 2];
        for channel in 0..2 {
            self.offset = self.target + (self.offset - self.target) * self.pole;
            let write = soft_limit(feedback * self.taps[channel] + input[channel]);
            self.lines[channel].write(flush(write));
            let tap = self.lines[channel].read_linear(self.offset);
            self.taps[channel] = tap;
            out[channel] = tap;
        }
        out
    }
}

impl Effect for APanDly {
    fn set(&mut self, values: &Values, bpm: f32) {
        self.apply(values, bpm, false);
    }

    #[inline]
    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        let weights = self.pan_weights();
        let order = self.order.tick();
        let wet = if order >= 1.0 {
            let delayed = self.delay(frame);
            [delayed[0] * weights[0], delayed[1] * weights[1]]
        } else if order <= 0.0 {
            self.delay([frame[0] * weights[0], frame[1] * weights[1]])
        } else {
            // A Link change: the original mutes c23/c24 for 16 ms around it.
            let mute = (2.0 * order - 1.0).abs();
            let input = if order < 0.5 {
                [frame[0] * weights[0], frame[1] * weights[1]]
            } else {
                frame
            };
            let delayed = self.delay([input[0] * mute, input[1] * mute]);
            if order < 0.5 {
                [delayed[0] * mute, delayed[1] * mute]
            } else {
                [
                    delayed[0] * weights[0] * mute,
                    delayed[1] * weights[1] * mute,
                ]
            }
        };
        let dry = self.dry.tick();
        let wet_gain = self.wet.tick();
        let level = self.level.tick();
        [
            level * (dry * frame[0] + wet_gain * wet[0]),
            level * (dry * frame[1] + wet_gain * wet[1]),
        ]
    }

    fn clear(&mut self) {
        for line in &mut self.lines {
            line.clear();
        }
        self.taps = [0.0; 2];
        self.offset = self.target;
        self.phase = -0.5;
    }
}

const RATE_LABELS: [&str; 53] = [
    "1/4", "1/2", "3/4", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
    "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30",
    "31", "32", "33", "34", "35", "36", "37", "38", "39", "40", "41", "42", "43", "44", "45", "46",
    "47", "48", "49", "50",
];
const WIDTH_LABELS: [&str; 101] = [
    "L50", "L49", "L48", "L47", "L46", "L45", "L44", "L43", "L42", "L41", "L40", "L39", "L38",
    "L37", "L36", "L35", "L34", "L33", "L32", "L31", "L30", "L29", "L28", "L27", "L26", "L25",
    "L24", "L23", "L22", "L21", "L20", "L19", "L18", "L17", "L16", "L15", "L14", "L13", "L12",
    "L11", "L10", "L9", "L8", "L7", "L6", "L5", "L4", "L3", "L2", "L1", "0", "R1", "R2", "R3",
    "R4", "R5", "R6", "R7", "R8", "R9", "R10", "R11", "R12", "R13", "R14", "R15", "R16", "R17",
    "R18", "R19", "R20", "R21", "R22", "R23", "R24", "R25", "R26", "R27", "R28", "R29", "R30",
    "R31", "R32", "R33", "R34", "R35", "R36", "R37", "R38", "R39", "R40", "R41", "R42", "R43",
    "R44", "R45", "R46", "R47", "R48", "R49", "R50",
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Model;

    fn defaults() -> Values {
        Model::from_key(INFO.key).unwrap().defaults()
    }

    #[test]
    fn editor_mappings() {
        assert_eq!(delay_samples_44k(222.0, 120.0), 9834.0);
        assert_eq!(delay_samples_44k(1999.0, 120.0), 88_200.0);
        // 1/4 at 120 BPM.
        assert_eq!(delay_samples_44k(2006.0, 120.0), 22_050.0);
        assert_eq!(delay_times_per_cycle(0.0), 0.25);
        assert_eq!(delay_times_per_cycle(2.0), 0.75);
        assert_eq!(delay_times_per_cycle(26.0), 24.0);
        assert_eq!(width_coefficients(50.0).0, 0.0);
        assert_eq!(width_coefficients(0.0), (1.0, -0.5));
        assert_eq!(width_coefficients(100.0), (1.0, 0.5));
        assert_eq!(mix_coefficients(53.0).1, 1.0);
        assert_eq!(mix_coefficients(25.0), (1.0, 0.5));
        assert_eq!(RATE_LABELS[3], "1");
        assert_eq!(WIDTH_LABELS[50], "0");
    }

    #[test]
    fn echo_lands_at_the_time() {
        for sample_rate in [44_100.0, 96_000.0] {
            let mut effect = APanDly::new(sample_rate);
            let mut values = defaults();
            values[0] = 99.0; // 100 ms
            values[1] = 0.0;
            values[2] = 50.0;
            values[4] = 50.0; // no panning
            values[5] = 0.0;
            effect.apply(&values, 120.0, true);
            effect.depth.jump(0.0);
            let expected = scale_samples(4410.0, sample_rate).round() as usize;
            let mut peak = (0, 0.0f32);
            for i in 1..expected * 2 {
                let x = if i == 1 { 1.0 } else { 0.0 };
                let [l, _] = effect.process([x, x]);
                if i > 1 && l.abs() > peak.1 {
                    peak = (i - 1, l.abs());
                }
            }
            assert!(peak.0.abs_diff(expected) <= 1, "{sample_rate}: {peak:?}");
            assert!((peak.1 - 1.0).abs() < 0.01, "{peak:?}");
        }
    }

    #[test]
    fn pan_follows_the_delay_time() {
        let sample_rate = 48_000.0;
        let mut effect = APanDly::new(sample_rate);
        let mut values = defaults();
        values[0] = 499.0; // 500 ms
        values[3] = 3.0; // one cycle per delay time
        values[4] = 0.0;
        values[5] = 0.0;
        effect.apply(&values, 120.0, true);
        // Phase advances 2 per cycle: one 500 ms cycle.
        let period = 2.0 / effect.increment / sample_rate;
        assert!((period - 0.5).abs() < 1e-3, "{period}");
        // Full width swings each side between 0 and 1.
        let (mut low, mut high) = (f32::MAX, f32::MIN);
        for _ in 0..24_000 {
            let [l, _] = effect.pan_weights();
            low = low.min(l);
            high = high.max(l);
        }
        assert!(low < 0.05 && high > 0.95, "{low} {high}");
    }

    #[test]
    fn feedback_at_maximum_stays_bounded() {
        let mut effect = create(48_000.0);
        let mut values = defaults();
        values[0] = 0.0;
        values[1] = 100.0;
        values[8] = 200.0;
        effect.set(&values, 120.0);
        let mut noise = crate::dsp::Noise::new(1);
        let mut peak = 0.0f32;
        for _ in 0..48_000 * 5 {
            let x = noise.bipolar();
            let [l, r] = effect.process([x, x]);
            peak = peak.max(l.abs()).max(r.abs());
        }
        assert!(peak.is_finite() && peak < 16.0, "{peak}");
    }
}
