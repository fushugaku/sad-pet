//! Tremolo (TREMOLO.ZDL), from the recovered `Fx_MOD_Tremolo` callback
//! (`tremolo.rs`) and the recovered editors (`tremolo_parameters.rs`).
//!
//! Callback, per sample: the LFO phase advances by the signed rate c5; the
//! wave is the phase itself (a saw, Wave UP/DWN) or `2|phase| − 1` (TRI),
//! multiplied by `16 × c9` (the clip table: 1×..10×) and saturated, so higher
//! wave numbers approach a square. The gain `1 − Depth × (0.5 + 0.5 wave)`
//! (c10..13) passes a one-pole smoother (c14/c15: 1/128 for saws, 0.264 for the
//! triangle) and multiplies the stereo mean; the pre/post filters c16..23 are
//! identities. Level (c2) scales the mono result sent to both outputs.
//!
//! Editors: Depth → c12 = −d (integer ramp 0x3deb7d per sample). Rate 0 mutes
//! the depth (ramp over 706 samples), 1..50 → `context[22]` 0xbe37..0xedc5b
//! square law over 50 steps (0.5..10.4 Hz), above 50 tempo notes; the rate is
//! negated except for the DWN group. Wave: group = value / 10 sets the saw
//! flag c8 and the smoother, value % 10 ramps c9 over 706 samples; a group
//! change crossfades the output to the dry mean and back (705.6 samples each,
//! modelled as one crossfade). Level 0..1.5. Everything follows the recovered
//! editors.
use super::common::{
    Lfo, Ramp, TREMOLO_CLIP, TREMOLO_WAVES, curve, guard, increment_hz, one_pole_at, rate_hz,
    stepped,
};
use crate::zoom::DEFAULT_STEP;
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_tremolo",
    name: "Tremolo",
    category: Category::Modulation,
    zdl: "TREMOLO.ZDL",
    description: "Tremolo with saw, reverse saw and clipped triangle waves",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Depth",
            max: 100,
            default: 80,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Rate",
            max: 78,
            default: 33,
            display: Display::sync_rate(50, 0.0),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Level",
            max: 150,
            default: 130,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Wave",
            max: 29,
            default: 21,
            display: Display::labels(&TREMOLO_WAVES),
        },
    ],
};

/// Q31 integer ramp 0x003deb7d per 44.1 kHz sample.
pub(super) const DEPTH_STEP: f32 = 4_057_981.0 / 2_147_483_648.0;
/// Integer ramps by duration (`context[9]`, 706 samples) and float (705.6).
pub(super) const DURATION: f32 = 706.0;
/// `Fx_MOD_Tremolo_LPF_tbl`: smoother input coefficient (saw, triangle).
const SMOOTH_SAW: f32 = 1.0 / 128.0;
const SMOOTH_TRIANGLE: f32 = 0.263_55;

/// Free rate as an original increment; `count` is 50 (Tremolo, AutoPan) or
/// 51 (DuoTrem).
pub(super) fn rate_increment(value: u32, count: u32) -> f32 {
    curve(973_915.0, 48_695.0, count, value, 1)
}

/// One tremolo LFO and gain path, shared with DuoTrem.
pub(super) struct Voice {
    sample_rate: f32,
    lfo: Lfo,
    hz: f32,
    group: u32,
    clip: Ramp,
    depth: Ramp,
    smooth_saw: f32,
    smooth_triangle: f32,
    memory: f32,
    /// DuoTrem's form: gain `(1 − d) + d × window` with the rate sign flipped.
    duo: bool,
    muted: bool,
}

impl Voice {
    pub fn new(sample_rate: f32, duo: bool) -> Self {
        Self {
            sample_rate,
            lfo: Lfo::default(),
            hz: 0.0,
            group: 2,
            clip: Ramp::with_duration(16.0 * TREMOLO_CLIP[5], DURATION, sample_rate),
            depth: if duo {
                Ramp::with_duration(0.0, DURATION, sample_rate)
            } else {
                Ramp::with_step(0.0, DEPTH_STEP, sample_rate)
            },
            smooth_saw: one_pole_at(SMOOTH_SAW, sample_rate),
            smooth_triangle: one_pole_at(SMOOTH_TRIANGLE, sample_rate),
            memory: 1.0,
            duo,
            muted: false,
        }
    }

    /// Rate value (0 mutes), threshold 50, curve count.
    pub fn set_rate(&mut self, rate: f32, bpm: f32, count: u32) {
        self.muted = rate < 0.5;
        self.hz = if self.muted {
            0.0
        } else {
            rate_hz(rate, 50, bpm, |v| {
                increment_hz(stepped(v.max(1.0), |n| rate_increment(n, count)))
            })
        };
        self.apply_rate();
    }

    fn apply_rate(&mut self) {
        // UP and TRI run the phase backwards in Tremolo, DWN in DuoTrem.
        let negative = (self.group == 1) == self.duo;
        let hz = if negative { -self.hz } else { self.hz };
        self.lfo.increment = super::common::hz_increment(hz.abs(), self.sample_rate);
        if negative {
            self.lfo.increment = self.lfo.increment.wrapping_neg();
        }
    }

    /// Depth 0..1 (already through the editor curve).
    pub fn set_depth(&mut self, depth: f32) {
        self.depth.set(if self.muted { 0.0 } else { depth });
    }

    /// Returns true when the wave group differs from the running one.
    pub fn group_changes(&self, wave: f32) -> bool {
        (wave.round().clamp(0.0, 29.0) as u32 / 10) != self.group
    }

    pub fn set_wave(&mut self, wave: f32, immediate: bool) {
        let wave = wave.round().clamp(0.0, 29.0) as u32;
        if immediate {
            self.group = wave / 10;
            self.apply_rate();
        }
        self.clip.set(16.0 * TREMOLO_CLIP[(wave % 10) as usize]);
    }

    pub fn settle(&mut self) {
        self.clip.jump(self.clip.target);
        self.depth.jump(self.depth.target);
    }

    #[inline]
    pub fn gain(&mut self) -> f32 {
        let s = self.lfo.tick();
        let wave = if self.group == 2 {
            2.0 * s.abs() - 1.0
        } else {
            s
        };
        let window = 0.5 + 0.5 * (wave * self.clip.tick()).clamp(-1.0, 1.0);
        let depth = self.depth.tick();
        let shaped = if self.duo {
            1.0 - depth + depth * window
        } else {
            1.0 - depth * window
        };
        let coefficient = if self.group == 2 {
            self.smooth_triangle
        } else {
            self.smooth_saw
        };
        self.memory += coefficient * (shaped - self.memory);
        self.memory
    }

    pub fn clear(&mut self) {
        self.lfo.phase = 0;
        self.memory = 1.0;
    }
}

struct Tremolo {
    voice: Voice,
    blend: Ramp,
    pending_wave: f32,
    level: Ramp,
    first: bool,
    cache: [f32; 4],
    bpm: f32,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(Tremolo {
        voice: Voice::new(sample_rate, false),
        blend: Ramp::with_duration(1.0, 705.6, sample_rate),
        pending_wave: 21.0,
        level: Ramp::with_step(1.3, DEFAULT_STEP, sample_rate),
        first: true,
        cache: [f32::NAN; 4],
        bpm: 0.0,
    })
}

impl Effect for Tremolo {
    fn set(&mut self, values: &Values, bpm: f32) {
        let [depth, rate, level, wave] = [values[0], values[1], values[2], values[3]];
        self.pending_wave = wave;
        let regroup = self.voice.group_changes(wave);
        if self.first {
            self.voice.set_wave(wave, true);
        } else if regroup {
            self.blend.set(0.0);
        } else {
            self.voice.set_wave(wave, false);
        }
        if rate != self.cache[1] || bpm != self.bpm {
            self.voice.set_rate(rate, bpm, 50);
            self.bpm = bpm;
        }
        self.voice.set_depth(depth.clamp(0.0, 100.0) * 0.01);
        self.level.set(level.clamp(0.0, 150.0) * 0.01);
        if self.first {
            self.first = false;
            self.voice.settle();
            self.level.jump(self.level.target);
        }
        self.cache = [depth, rate, level, wave];
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let blend = self.blend.tick();
        if blend == 0.0 && self.blend.target == 0.0 {
            self.voice.set_wave(self.pending_wave, true);
            self.blend.set(1.0);
        }
        let mean = 0.5 * (left + right);
        let gain = self.voice.gain();
        let wet = blend * gain * mean + (1.0 - blend) * mean;
        let out = guard(wet * self.level.tick());
        [out, out]
    }

    fn clear(&mut self) {
        self.voice.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fx::modulation::common::testing::crossings;

    fn gains(values: [f32; 4], seconds: f32) -> Vec<f32> {
        let mut effect = create(48_000.0);
        let mut v = [0.0; crate::MAX_PARAMS];
        v[..4].copy_from_slice(&values);
        effect.set(&v, 120.0);
        (0..(seconds * 48_000.0) as usize)
            .map(|_| effect.process([1.0, 1.0])[0])
            .collect()
    }

    #[test]
    fn rate_curve_matches_the_editor() {
        assert!((increment_hz(rate_increment(1, 50)) - 0.5).abs() < 0.01);
        assert!((increment_hz(rate_increment(49, 50)) - 10.0).abs() < 0.01);
    }

    #[test]
    fn depth_and_level_set_the_gain_range() {
        // Depth 80, Level 100, TRI 0 at Rate 25 (~3 Hz).
        let out = gains([80.0, 25.0, 100.0, 20.0], 2.0);
        let tail = &out[48_000..];
        let max = tail.iter().copied().fold(f32::MIN, f32::max);
        let min = tail.iter().copied().fold(f32::MAX, f32::min);
        assert!(
            (max - 1.0).abs() < 0.02 && (min - 0.2).abs() < 0.03,
            "{min} {max}"
        );
        let hz = crossings(tail) as f32;
        let expected = increment_hz(rate_increment(25, 50));
        assert!((hz - expected).abs() <= 1.0, "{hz} {expected}");
    }

    #[test]
    fn tempo_sync_follows_the_note() {
        // Rate 58 = index 7 = quarter note: 2 Hz at 120 BPM.
        let out = gains([100.0, 58.0, 100.0, 20.0], 3.0);
        let hz = crossings(&out[48_000..]) as f32 / 2.0;
        assert!((hz - 2.0).abs() <= 0.5, "{hz}");
    }

    #[test]
    fn rate_zero_mutes_the_modulation() {
        let out = gains([100.0, 0.0, 100.0, 20.0], 0.5);
        assert!(out[20_000..].iter().all(|&g| (g - 1.0).abs() < 1e-3));
    }
}
