//! Rt Closet (RTCLOSET.ZDL): a Leslie-style rotary speaker with a drive
//! stage, rebuilt from the recovered callback `Fx_SFX_Rt_Closet`
//! (zoom-ms70-cdr `rotary_closet`) and its editors.
//!
//! Signal flow (per 44.1 kHz sample in the original, ROM filters re-derived
//! for the instance rate):
//!
//! ```text
//! (L+R)/2 -> low peak c81 (+9 dB at 50 Hz) -> soft clip (Drive) -> 50 Hz HPF c90
//!   -> 200 Hz dip c95 = x
//! drum: x -> SVF (f 0.035, q 10, states clamped to ±1) -> clamp(10 hp + 19.5 bp + 19 lp)
//!   -> 2 kHz LPF c11 -> SVF (f 0.035, q 1, clamped) band-pass -> 2 x 500 Hz LPF c21
//!   -> ring 0 and "direct"
//! horn: x -> 800 Hz HPF c34 -> +8 dB low shelf c39 (= horn direct)
//!   -> +2 dB peak c44 -> 100 Hz notch-ish c49 -> ring 1
//! ```
//!
//! Each rotor reads its ring at a "near" tap `252 + 54 k` and a "far" tap
//! `340 - 35 k` samples (linear interpolation), `k` the folded rotor phase
//! (squared for the drum), offset by -1/16 turn (drum) / +1/16 turn (horn).
//! Amplitude modulation: drum `0.4375 + 0.5625 min(1.25 m^4, 1)` on the
//! phase and its quadrature (c109/c110 smoothing is an identity); horn
//! `0.46875 + 0.53125 m^2` times 0.6 (c60/c61).
//!
//! ```text
//! drum L = am_drum(q) (direct + 0.2 (far + c104 near))      c105 = 0.2
//! drum R = am_drum(p) (direct + 0.2 (c104 far + near))
//! horn L = tone(am_horn(p) near + c104 am_horn(q) far + 0.15 direct)
//! horn R = tone(c104 am_horn(p) near + am_horn(q) far + 0.15 direct)
//! out = Level (Bal_drum drum + Bal_horn horn)
//! ```
//!
//! Rotors: the drum increment ramps linearly by ±2 per sample to 69564
//! (0.71 Hz) or 500000 (5.1 Hz), snapping when within 4/3 of a step (about
//! 4.9 s up); the horn's speed word follows 623296 / 5194208 through a
//! one-pole (k = 2^-15, ~0.74 s) and its phase turns backwards by a
//! eighth of it per sample (0.80 / 6.67 Hz).
//!
//! Editors decoded: `rate_edit` (Mode: targets above and the ramp sign; jumps
//! to the targets while bypassed), `bal_edit` (drum `0.61 (1 - Bal/100)`, horn
//! `0.61 * 1.2506 Bal/100`, 16 ms glides), `drive_edit` (clip gain
//! `4 + 11 (Drive/100)^2`, output `0.6 - 0.002 gain`, ramped), `outlv_edit`
//! (`Level/100`), and the fixed `tone_edit` (c62..c66: 3 kHz Q 0.7 low-pass),
//! `phaMix_edit` (c104 = 0.2) and `dryMix_edit` (c67 = 0.15).
//! `width_edit` and `depth_edit` write c76/c77/c102/c103, which the callback
//! never reads. Identity stages (input one-pole c78..c80) are left out.
use super::common::{Filter, Glide, Kind, design, fold, rescale_pole, rom};
use crate::dsp::DelayLine;
use crate::zoom::{ORIGINAL_RATE, Ramp, curve};
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_rt_closet",
    name: "Rt Closet",
    category: Category::Modulation,
    zdl: "RTCLOSET.ZDL",
    description: "Rotary speaker with drum and horn rotors and drive",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Bal",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Mode",
            max: 1,
            default: 0,
            display: Display::labels(&["Slow", "Fast"]),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Drive",
            max: 100,
            default: 20,
            display: Display::INT,
        },
    ],
};

/// The editors' 0x3af7c5ec step (per 44.1 kHz sample).
const DRIVE_STEP: f32 = 0.001_890_359;
const DRUM_TARGETS: [f32; 2] = [69_564.0, 500_000.0];
/// Where the Q31 one-pole really settles for the editor words 623296 /
/// 5194208 (its products truncate toward minus infinity).
const HORN_TARGETS: [f32; 2] = [622_592.0, 5_177_344.0];
/// c57 (Q31 0xba0cd9e9) and c56 (0x6c000000) over the 25-bit tap scale (c70).
const FAR_SWING: f32 = -1_173_562_903.0 / 33_554_432.0;
const NEAR_SWING: f32 = 54.0;
const FAR_BASE: f32 = 340.0;
const NEAR_BASE: f32 = 252.0;
const PHASE_MIX: f32 = 0.2;
const DRUM_TAPS: f32 = 0.2;
const HORN_DIRECT: f32 = 0.15;

#[inline]
fn clamp1(x: f32) -> f32 {
    x.clamp(-1.0, 1.0)
}

/// Chamberlin state-variable filter whose three outputs saturate at ±1.
#[derive(Debug, Clone, Copy)]
struct Svf {
    f: f32,
    q: f32,
    band: f32,
    low: f32,
}

impl Svf {
    /// `f_at_44k` is the ROM frequency word `2 sin(pi fc / 44100)`.
    fn new(f_at_44k: f32, q: f32, sample_rate: f32) -> Self {
        let hz =
            (f64::from(f_at_44k) * 0.5).asin() * f64::from(ORIGINAL_RATE) / std::f64::consts::PI;
        let f = 2.0 * (std::f64::consts::PI * hz / f64::from(sample_rate)).sin();
        Self {
            f: f as f32,
            q,
            band: 0.0,
            low: 0.0,
        }
    }

    #[inline]
    fn tick(&mut self, x: f32) -> (f32, f32, f32) {
        let high = clamp1(x - self.q * self.band - self.low);
        self.band = clamp1(self.band + self.f * high);
        self.low = clamp1(self.low + self.f * self.band);
        (high, self.band, self.low)
    }

    fn clear(&mut self) {
        self.band = 0.0;
        self.low = 0.0;
    }
}

/// `soft_clip` of the callback: knee at `threshold`, `t + d / (1 + (d/(1-t))^2)`
/// above it, flat `(1 + t) / 2` from 1 up (c87/c88 = 0.2 for both signs).
#[inline]
fn soft_clip(x: f32, gain: f32, out: f32) -> f32 {
    const T: f32 = 0.2;
    let value = x.abs() * gain;
    let shaped = if value >= 1.0 {
        0.5 * (1.0 + T)
    } else if value > T {
        let d = value - T;
        let q = d / (1.0 - T);
        T + d / (1.0 + q * q)
    } else {
        value
    };
    (shaped * out).copysign(x)
}

/// 0.4375 + 0.5625 min(1.25 m^4, 1): the drum's amplitude shape.
#[inline]
fn drum_am(phase: u32) -> f32 {
    let m = fold(phase);
    let m2 = m * m;
    0.4375 + 0.5625 * (1.25 * m2 * m2).min(1.0)
}

/// 0.46875 + 0.53125 m^2: the horn's amplitude shape.
#[inline]
fn horn_am(phase: u32) -> f32 {
    let m = fold(phase);
    0.46875 + 0.53125 * m * m
}

struct RtCloset {
    /// Instance samples per original sample.
    scale: f32,
    drum_phase: u32,
    drum_increment: f32,
    drum_target: f32,
    drum_accel: f32,
    horn_phase: u32,
    horn_speed: f32,
    horn_target: f32,
    horn_smoothing: f32,
    pre: Filter,
    post: [Filter; 2],
    gain: Ramp,
    clip_out: Ramp,
    svf_drive: Svf,
    drum_lowpass: Filter,
    svf_band: Svf,
    crossover: [Filter; 2],
    horn_filters: [Filter; 4],
    lines: [DelayLine; 2],
    tone: [Filter; 2],
    level: Ramp,
    drum_level: Glide,
    horn_level: Glide,
    last: Values,
    fresh: bool,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(RtCloset::new(sample_rate))
}

impl RtCloset {
    fn new(sample_rate: f32) -> Self {
        let r = |b0, b1, b2, p1, p2| {
            let mut f = Filter::default();
            f.set(rom(b0, b1, b2, p1, p2, sample_rate));
            f
        };
        let tone = {
            let mut f = Filter::default();
            f.set(design(Kind::LowPass2, sample_rate, 3000.0, 0.0, 0.7));
            f
        };
        let lowpass2 = r(
            0.001_206_819,
            0.002_413_636,
            0.001_206_819,
            1.898_409_8,
            -0.903_237_1,
        );
        let mut effect = RtCloset {
            scale: sample_rate / ORIGINAL_RATE,
            drum_phase: 0,
            drum_increment: 0.0,
            drum_target: 0.0,
            drum_accel: 0.0,
            horn_phase: 0,
            horn_speed: 0.0,
            horn_target: 0.0,
            horn_smoothing: rescale_pole(65_536.0 / 2_147_483_648.0, sample_rate),
            pre: r(
                1.009_660_8,
                -1.990_984_1,
                0.981_403_5,
                1.990_984_1,
                -0.991_064_3,
            ),
            post: [
                r(
                    0.992_902,
                    -1.985_804_1,
                    0.992_902,
                    1.985_754_6,
                    -0.985_853_4,
                ),
                r(
                    0.996_376_4,
                    -1.963_965_9,
                    0.968_387_1,
                    1.963_965_9,
                    -0.964_763_5,
                ),
            ],
            gain: Ramp::new(4.0, DRIVE_STEP, sample_rate),
            clip_out: Ramp::new(0.6, DRIVE_STEP, sample_rate),
            svf_drive: Svf::new(0.035, 10.0, sample_rate),
            drum_lowpass: r(0.125_452, 0.125_452, 0.0, 0.749_096, 0.0),
            svf_band: Svf::new(0.035, 1.0, sample_rate),
            crossover: [lowpass2, lowpass2],
            horn_filters: [
                r(0.946_027_2, -0.946_027_2, 0.0, 0.892_054_3, 0.0),
                r(1.172_672_5, -0.598_907_5, 0.0, 0.771_58, 0.0),
                r(
                    1.017_205_1,
                    -1.862_368_2,
                    0.849_898_8,
                    1.862_368_2,
                    -0.867_103_9,
                ),
                r(
                    0.992_339_6,
                    -1.982_158_3,
                    0.990_133_05,
                    1.982_158_3,
                    -0.982_472_7,
                ),
            ],
            lines: [
                DelayLine::with_seconds(0.01, sample_rate),
                DelayLine::with_seconds(0.01, sample_rate),
            ],
            tone: [tone, tone],
            level: Ramp::new(0.0, crate::zoom::DEFAULT_STEP, sample_rate),
            drum_level: Glide::new(0.0, 705.6, sample_rate),
            horn_level: Glide::new(0.0, 705.6, sample_rate),
            last: [f32::NAN; crate::MAX_PARAMS],
            fresh: true,
        };
        effect.set(&crate::Model::RtCloset.defaults(), 120.0);
        effect
    }
}

impl Effect for RtCloset {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let jump = self.fresh;
        self.fresh = false;
        if values[0] != self.last[0] {
            let bal = values[0].clamp(0.0, 100.0);
            let horn = 0.61 * curve(1.250_6, 0.0, 101, bal, 0);
            let drum = 0.61 * curve(0.0, 1.0, 101, bal, 0);
            if jump {
                self.horn_level.jump(horn);
                self.drum_level.jump(drum);
            } else {
                self.horn_level.set(horn);
                self.drum_level.set(drum);
            }
        }
        if values[1] != self.last[1] {
            let fast = usize::from(values[1].round() >= 1.0);
            let r = 1.0 / self.scale;
            self.drum_target = DRUM_TARGETS[fast] * r;
            self.drum_accel = if fast == 1 { 2.0 } else { -2.0 } * r * r;
            self.horn_target = HORN_TARGETS[fast];
            if jump {
                self.drum_increment = self.drum_target;
                self.horn_speed = self.horn_target;
            }
        }
        if values[2] != self.last[2] {
            let level = curve(1.5, 0.0, 151, values[2], 0);
            if jump {
                self.level.jump(level);
            } else {
                self.level.set(level);
            }
        }
        if values[3] != self.last[3] {
            let gain = curve(15.0, 4.0, 101, values[3], 1);
            let out = 0.6 - 0.002 * gain;
            if jump {
                self.gain.jump(gain);
                self.clip_out.jump(out);
            } else {
                self.gain.set(gain);
                self.clip_out.set(out);
            }
        }
        self.last = *values;
    }

    fn process(&mut self, [l, r]: [f32; 2]) -> [f32; 2] {
        // Rotors.
        let next = self.drum_increment + self.drum_accel;
        self.drum_increment = if self.drum_accel.abs() > 0.75 * (next - self.drum_target).abs() {
            self.drum_target
        } else {
            next
        };
        self.drum_phase = self
            .drum_phase
            .wrapping_add(self.drum_increment.round() as i32 as u32);
        self.horn_speed += (self.horn_target - self.horn_speed) * self.horn_smoothing;
        let horn_step = (self.horn_speed * 0.125 / self.scale).round() as i32 as u32;
        self.horn_phase = self.horn_phase.wrapping_sub(horn_step);

        // Shared front end.
        let mono = (l + r) * 0.5;
        let x = self.pre.process(mono);
        let x = soft_clip(x, self.gain.tick(), self.clip_out.tick());
        let [post_a, post_b] = &mut self.post;
        let x = post_b.process(post_a.process(x));

        // Drum branch.
        let (high, band, low) = self.svf_drive.tick(x);
        let drum = clamp1(10.0 * high + 19.5 * band + 19.0 * low);
        let drum = self.drum_lowpass.process(drum);
        let (_, band, _) = self.svf_band.tick(drum);
        let [lp_a, lp_b] = &mut self.crossover;
        let drum = lp_b.process(lp_a.process(clamp1(band)));
        self.lines[0].write(drum);
        let k = fold(self.drum_phase.wrapping_sub(0x1000_0000));
        let k = k * k;
        let near = self.lines[0].read_linear((NEAR_BASE + NEAR_SWING * k) * self.scale);
        let far = self.lines[0].read_linear((FAR_BASE + FAR_SWING * k) * self.scale);
        let am_p = drum_am(self.drum_phase);
        let am_q = drum_am(self.drum_phase.wrapping_add(0x4000_0000));
        let drum_l = am_q * (drum + DRUM_TAPS * (far + PHASE_MIX * near));
        let drum_r = am_p * (drum + DRUM_TAPS * (PHASE_MIX * far + near));

        // Horn branch.
        let [hp, shelf, peak, dip] = &mut self.horn_filters;
        let direct = shelf.process(hp.process(x));
        self.lines[1].write(dip.process(peak.process(direct)));
        let k = fold(self.horn_phase.wrapping_add(0x1000_0000));
        let near = self.lines[1].read_linear((NEAR_BASE + NEAR_SWING * k) * self.scale);
        let far = self.lines[1].read_linear((FAR_BASE + FAR_SWING * k) * self.scale);
        let near = 0.6 * horn_am(self.horn_phase) * near;
        let far = 0.6 * horn_am(self.horn_phase.wrapping_add(0xc000_0000)) * far;
        let direct = HORN_DIRECT * direct;
        let horn_l = self.tone[0].process(near + PHASE_MIX * far + direct);
        let horn_r = self.tone[1].process(PHASE_MIX * near + far + direct);

        let level = self.level.tick();
        let drum_level = self.drum_level.tick() * level;
        let horn_level = self.horn_level.tick() * level;
        [
            drum_level * drum_l + horn_level * horn_l,
            drum_level * drum_r + horn_level * horn_r,
        ]
    }

    fn clear(&mut self) {
        self.pre.clear();
        self.post.iter_mut().for_each(Filter::clear);
        self.svf_drive.clear();
        self.drum_lowpass.clear();
        self.svf_band.clear();
        self.crossover.iter_mut().for_each(Filter::clear);
        self.horn_filters.iter_mut().for_each(Filter::clear);
        self.lines.iter_mut().for_each(DelayLine::clear);
        self.tone.iter_mut().for_each(Filter::clear);
        self.drum_phase = 0;
        self.horn_phase = 0;
        self.drum_increment = self.drum_target;
        self.horn_speed = self.horn_target;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Model;

    #[test]
    fn rotor_speeds_match_the_rate_editor() {
        let hz = |word: f32| word * 44_100.0 / 4_294_967_296.0;
        assert!((hz(DRUM_TARGETS[0]) - 0.714).abs() < 0.002);
        assert!((hz(DRUM_TARGETS[1]) - 5.134).abs() < 0.005);
        assert!((hz(HORN_TARGETS[0] / 8.0) - 0.799).abs() < 0.002);
        assert!((hz(HORN_TARGETS[1] / 8.0) - 6.645).abs() < 0.005);
    }

    #[test]
    fn drum_spins_up_linearly_in_about_five_seconds() {
        let mut effect = RtCloset::new(48_000.0);
        let mut values = Model::RtCloset.defaults();
        values[1] = 1.0;
        effect.set(&values, 120.0);
        let start = effect.drum_increment;
        for _ in 0..48_000 * 2 {
            effect.process([0.0; 2]);
        }
        let halfway = effect.drum_increment;
        for _ in 0..48_000 * 4 {
            effect.process([0.0; 2]);
        }
        assert!(halfway > start && halfway < effect.drum_target);
        assert_eq!(effect.drum_increment, effect.drum_target);
        let seconds = (effect.drum_target - start) / effect.drum_accel / 48_000.0;
        assert!((seconds - 4.88).abs() < 0.05, "{seconds}");
    }

    #[test]
    fn soft_clip_knee_is_continuous() {
        let below = soft_clip(0.999_99, 1.0, 1.0);
        assert!((below - 0.6).abs() < 1e-4, "{below}");
        assert_eq!(soft_clip(0.1, 1.0, 1.0), 0.1);
        assert!((soft_clip(-2.0, 1.0, 0.5) + 0.3).abs() < 1e-6);
    }

    #[test]
    fn svf_rescales_its_frequency_word() {
        let at44 = Svf::new(0.035, 10.0, 44_100.0);
        assert!((at44.f - 0.035).abs() < 1e-6);
        let at96 = Svf::new(0.035, 10.0, 96_000.0);
        assert!((at96.f - 0.035 * 44_100.0 / 96_000.0).abs() < 1e-4);
    }
}
