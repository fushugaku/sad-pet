//! LO-FI Dly (LOFIDLY.ZDL): a stereo delay whose input is sample-and-held,
//! bit-reduced and filtered, with vinyl noise riding on the repeats, after the
//! recovered `Fx_DLY_LoFiDelay` callback and the decoded editors.
//!
//! Per channel, as in the callback: `clip±1(F.B·tap)` → 20 Hz high-pass →
//! `+ input`; a sample-and-hold every SMPL samples (`_LoFi_SampleTbl`, 1/128…
//! 1/1 of 44.1 kHz, kept at that rate on any host rate); the held value ×3.46
//! is quantized (`_LoFi_BitsTbl` float steps for the half bits, otherwise
//! `_LoFi_BitShiftTbl` shifts of the saturating Q31 value) with the LFSR's low
//! bit randomly clearing the LSB, ×0.2865; BLEND crossfades clean and lo-fi;
//! the four DAMP biquads (shared `dampCoeTbl`) → 2 s delay. The wet taps feed
//! the vinyl section: the 32-bit Galois LFSR (taps 0xD0000001) makes hiss and
//! three sparse crackle/click streams; a peak follower of the wet (×70,
//! 0.9996 decay) drives the "dynamic" amount (rises 3e-5/sample above 0.1,
//! decays 0.9999); clicks pass a low-shelf, a 2× low-pass and a peak EQ, hiss
//! a 12 kHz low-pass; VINYL D:n scales the dynamic amount, S:n a static level
//! (`_LoFi_VinylTbl`, n²/100), added to L and subtracted from R. Then the FLT
//! set (seven ROM biquads of `Filter_tbl`) and `MixWet·wet + MixDry·dry`.
//!
//! Editors: TIME is `p + 2` ms (2–1900) or tempo notes, F.B `0.87·FB_tbl`,
//! BLEND `p/100`, DAMP/FLT copied with the editor's 16 ms mute around the
//! swap. ROM biquads are re-derived at the host rate. Deviations: a TIME change
//! glides (as the original does for tempo changes) instead of mute → jump →
//! reopen; the LFSR runs per host sample, so crackle density scales with the
//! sample rate. The unused delay LFO and grit section are omitted.
use super::{
    common::{
        Cascade, DelayLine, DurationRamp, Glide, MuteSwap, Q31, RAMP_SAMPLES, Settled, Stage,
        Stage1, clip1, flush, pole, rom, rom_set, table, time_ms,
    },
    tables::{DAMP, LOFI_FILTER},
    trem_delay::{FB_TBL, FEEDBACK_HP, MIX_DRY, MIX_WET},
};
use crate::{
    Effect, Values,
    catalog::*,
    zoom::{DEFAULT_STEP, Ramp, is_synced},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_lo_fi_dly",
    name: "LO-FI Dly",
    category: Category::Delay,
    zdl: "LOFIDLY.ZDL",
    description: "Bit-crushed, sample-reduced delay with vinyl noise",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "TIME",
            max: 1913,
            default: 248,
            display: Display::DelayTime {
                threshold: 1898,
                offset: 2,
            },
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "F.B",
            max: 100,
            default: 25,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "MIX",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "SMPL",
            max: 14,
            default: 5,
            display: Display::labels(&[
                "1/128", "1/64", "1/32", "1/24", "1/12", "1/10", "1/9", "1/8", "1/7", "1/6", "1/5",
                "1/4", "1/3", "1/2", "1/1",
            ]),
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "BITS",
            max: 20,
            default: 10,
            display: Display::labels(&[
                "4", "4.5", "5", "5.5", "6", "6.5", "7", "7.5", "8", "9", "10", "11", "12", "13",
                "14", "15", "16", "18", "20", "24", "32",
            ]),
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "BLEND",
            max: 100,
            default: 60,
            display: Display::INT,
        },
        // values[6] is firmware parameter word 8
        ParamInfo {
            name: "DAMP",
            max: 10,
            default: 0,
            display: Display::INT,
        },
        // values[7] is firmware parameter word 9
        ParamInfo {
            name: "FLT",
            max: 8,
            default: 2,
            display: Display::labels(&["OFF", "1", "2", "3", "4", "5", "6", "7", "8"]),
        },
        // values[8] is firmware parameter word 10
        ParamInfo {
            name: "VINYL",
            max: 18,
            default: 0,
            display: Display::labels(&[
                "OFF", "D:1", "D:2", "D:3", "D:4", "D:5", "D:6", "D:7", "D:8", "D:9", "S:1", "S:2",
                "S:3", "S:4", "S:5", "S:6", "S:7", "S:8", "S:9",
            ]),
        },
    ],
};

const MAX_MS: f32 = 1900.0;
const THRESHOLD: u16 = 1898;
const SAMPLE_HOLD: [f32; 15] = [
    128.0, 64.0, 32.0, 24.0, 12.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
];
const BIT_STEP: [f32; 21] = [
    0.0,
    0.088_388_4,
    0.0,
    0.044_194_2,
    0.0,
    0.022_097,
    0.0,
    0.011_048_6,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
];
const BIT_SHIFT: [u32; 21] = [
    28, 0, 27, 0, 26, 0, 25, 0, 24, 23, 22, 21, 20, 19, 18, 17, 16, 14, 12, 8, 0,
];
const CLICK_EQ: [[f32; 5]; 3] = [
    [
        1.018_24,
        -1.975_582_6,
        0.957_631_6,
        1.975_582_6,
        -0.975_871_4,
    ],
    [
        0.077_409_92,
        0.154_819_85,
        0.077_409_92,
        1.569_790_6,
        -0.724_978,
    ],
    [
        0.854_631_2,
        -1.569_052_7,
        0.720_172_2,
        1.569_052_7,
        -0.574_803_4,
    ],
];
const HISS_LP: [f32; 5] = [0.534_784_4, 0.534_784_4, 0.0, -0.069_568_89, 0.0];

/// The callback's quantizer of `scaled = held · 3.46`.
pub fn quantize(scaled: f32, step: f32, shift: u32, lfsr: i32) -> f32 {
    let mask = 0xffff_fffe_u32 | lfsr as u32;
    let magnitude = if step > 0.0 {
        let integer = ((scaled.abs() / step) as u32) & mask;
        step * integer as f32
    } else {
        let integer =
            (f64::from(scaled) * 2_147_483_648.0).clamp(-2_147_483_648.0, 2_147_483_647.0) as i64;
        let integer = integer.unsigned_abs().min(0x7fff_ffff) as u32;
        (((integer >> shift) & mask) << shift) as f32 * Q31
    };
    magnitude.copysign(scaled)
}

struct Channel {
    feedback_hp: Stage1,
    held: f32,
    /// `quantize(held · 3.46, step, shift, lfsr)` for LFSR bit 0 clear / set
    /// (the only LFSR bit the quantizer's mask keeps), updated with `held`
    /// and the BITS setting.
    levels: [f32; 2],
    damp: Cascade<4>,
    line: DelayLine,
    tap: f32,
    filter: Cascade<7>,
}

struct LoFiDly {
    sample_rate: f32,
    channels: [Channel; 2],
    time: Glide,
    lfsr: i32,
    hold_phase: f32,
    hold_samples: f32,
    hold_step: f32,
    step: f32,
    shift: u32,
    blend: Ramp,
    feedback: Ramp,
    wet: Ramp,
    dry: Ramp,
    damp_swap: MuteSwap,
    damp_pending: [[f32; 5]; 4],
    filter_swap: MuteSwap,
    filter_pending: [[f32; 5]; 7],
    // vinyl
    vinyl: (f32, f32),
    detector: f32,
    detector_decay: f32,
    amount: f32,
    amount_rise: f32,
    amount_decay: f32,
    click_eq: [Stage; 3],
    hiss_lp: Stage1,
    vinyl_gain: DurationRamp,
    last: [f32; 9],
    bpm: f32,
}

impl LoFiDly {
    fn new(sample_rate: f32) -> Self {
        let sr = sample_rate;
        let channel = || Channel {
            feedback_hp: Stage1::from(rom(FEEDBACK_HP, sr)),
            held: 0.0,
            levels: [0.0; 2],
            damp: Cascade::default(),
            line: DelayLine::with_seconds(MAX_MS * 0.001 + 0.01, sr),
            tap: 0.0,
            filter: Cascade::default(),
        };
        let mut effect = Self {
            sample_rate,
            channels: [channel(), channel()],
            time: Glide::standard(0.0, sr),
            lfsr: 1,
            hold_phase: 1e9,
            hold_samples: 1.0,
            hold_step: 44_100.0 / sr,
            step: 0.0,
            shift: 0,
            blend: Ramp::new(1.0, DEFAULT_STEP, sr),
            feedback: Ramp::new(0.0, DEFAULT_STEP, sr),
            wet: Ramp::new(0.0, DEFAULT_STEP, sr),
            dry: Ramp::new(1.0, DEFAULT_STEP, sr),
            damp_swap: MuteSwap::new(sr),
            damp_pending: [Stage::IDENTITY; 4],
            filter_swap: MuteSwap::new(sr),
            filter_pending: [Stage::IDENTITY; 7],
            vinyl: (0.0, 0.0),
            detector: 0.0,
            detector_decay: pole(0.999_6, sr),
            amount: 0.0,
            amount_rise: 3e-5 * 44_100.0 / sr,
            amount_decay: pole(0.999_9, sr),
            click_eq: CLICK_EQ.map(|w| Stage::from(rom(w, sr))),
            hiss_lp: Stage1::from(rom(HISS_LP, sr)),
            vinyl_gain: DurationRamp::new(0.0, RAMP_SAMPLES, sr),
            last: [f32::NAN; 9],
            bpm: 0.0,
        };
        effect.set(&crate::Model::LoFiDly.defaults(), 120.0);
        effect.time.value = effect.time.target;
        for ramp in [
            &mut effect.blend,
            &mut effect.feedback,
            &mut effect.wet,
            &mut effect.dry,
        ] {
            ramp.jump(ramp.target);
        }
        effect.vinyl_gain.jump(effect.vinyl_gain.target);
        effect
    }

    /// The quantized hold for both values of the LFSR's low bit.
    #[inline]
    fn levels(held: f32, step: f32, shift: u32) -> [f32; 2] {
        let scaled = held * 3.46;
        [
            quantize(scaled, step, shift, 0),
            quantize(scaled, step, shift, 1),
        ]
    }

    #[inline]
    fn vinyl_noise(&mut self, wet: [f32; 2]) -> f32 {
        let (dynamic, fixed) = self.vinyl;
        // detector and dynamic amount
        let candidate = 70.0 * wet[0].abs().max(wet[1].abs());
        self.detector = flush(self.detector.max(candidate) * self.detector_decay);
        if self.detector > 0.1 {
            if self.amount == 0.0 {
                self.amount = 0.2;
            }
            self.amount = (self.amount + self.amount_rise).min(1.0);
        } else {
            self.amount = flush(self.amount * self.amount_decay);
        }
        let r = self.lfsr;
        let band = |mask: u32, scale: u32| {
            if r as u32 & mask == mask {
                r as f32 * Q31 * (scale as f32 * Q31)
            } else {
                0.0
            }
        };
        let hiss = band(0xc00f_ff00, 0x2c04_ea4a)
            + band(0xfa94_1000, 0x2385_1eb8)
            + (r as u32 & 0x001f_ffff) as f32 * Q31;
        let mut click = band(0xc075_00f5, 0x37ff_ffff);
        for stage in &mut self.click_eq {
            click = stage.process(click);
        }
        let noise = click + self.hiss_lp.process(hiss);
        (fixed + dynamic * self.amount) * noise
    }
}

impl Effect for LoFiDly {
    fn set(&mut self, values: &Values, bpm: f32) {
        if values[..9] == self.last && bpm == self.bpm {
            return;
        }
        let sr = self.sample_rate;
        let first = self.last[0].is_nan();
        if values[0] != self.last[0] || (bpm != self.bpm && is_synced(values[0], THRESHOLD)) {
            let ms = time_ms(values[0], THRESHOLD, 2.0, MAX_MS, bpm);
            self.time.target = ms * 0.001 * sr;
        }
        self.feedback
            .set(0.87 * table(&FB_TBL, values[1].round().clamp(0.0, 100.0), 101));
        let mix = values[2].round().clamp(0.0, 100.0);
        self.wet.set(table(&MIX_WET, mix, 101));
        self.dry.set(table(&MIX_DRY, mix, 101));
        self.hold_samples = SAMPLE_HOLD[values[3].round().clamp(0.0, 14.0) as usize];
        let bits = values[4].round().clamp(0.0, 20.0) as usize;
        self.step = BIT_STEP[bits];
        self.shift = BIT_SHIFT[bits];
        for ch in &mut self.channels {
            ch.levels = Self::levels(ch.held, self.step, self.shift);
        }
        self.blend.set(values[5].round().clamp(0.0, 100.0) * 0.01);
        if values[6] != self.last[6] {
            let damp = values[6].round().clamp(0.0, 10.0) as usize;
            self.damp_pending = rom_set(&DAMP[damp], sr);
            if first {
                for ch in &mut self.channels {
                    ch.damp.load(&self.damp_pending);
                }
            } else {
                self.damp_swap.start();
            }
        }
        if values[7] != self.last[7] {
            let filter = values[7].round().clamp(0.0, 8.0) as usize;
            self.filter_pending = rom_set(&LOFI_FILTER[filter], sr);
            if first {
                for ch in &mut self.channels {
                    ch.filter.load(&self.filter_pending);
                }
            } else {
                self.filter_swap.start();
            }
        }
        let vinyl = values[8].round().clamp(0.0, 18.0) as usize;
        self.vinyl = match vinyl {
            0 => (0.0, 0.0),
            1..=9 => ((vinyl * vinyl) as f32 * 0.01, 0.0),
            _ => (0.0, ((vinyl - 9) * (vinyl - 9)) as f32 * 0.01),
        };
        self.vinyl_gain.set(if vinyl == 0 { 0.0 } else { 1.0 });
        self.last.copy_from_slice(&values[..9]);
        self.bpm = bpm;
    }

    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        // LFSR (asr + 0xD0000001)
        let old = self.lfsr;
        self.lfsr = (old >> 1)
            ^ if old & 1 != 0 {
                0xd000_0001_u32 as i32
            } else {
                0
            };
        self.hold_phase += self.hold_step;
        let hold = self.hold_phase >= self.hold_samples;
        if hold {
            self.hold_phase = (self.hold_phase - self.hold_samples).min(self.hold_samples);
        }
        let delay = self.time.tick();
        let feedback = self.feedback.step();
        let blend = self.blend.step();
        let (damp_gain, damp_swap) = self.damp_swap.tick();
        let (filter_gain, filter_swap) = self.filter_swap.tick();
        if damp_swap {
            for ch in &mut self.channels {
                ch.damp.load(&self.damp_pending);
            }
        }
        if filter_swap {
            for ch in &mut self.channels {
                ch.filter.load(&self.filter_pending);
            }
        }
        let mut wet = [0.0; 2];
        for (ch, channel) in self.channels.iter_mut().enumerate() {
            let combined = channel.feedback_hp.process(clip1(feedback * channel.tap)) + frame[ch];
            if hold {
                channel.held = combined;
                channel.levels = Self::levels(combined, self.step, self.shift);
            }
            let lofi = 0.286_522 * channel.levels[(old & 1) as usize];
            let blended = (1.0 - blend) * combined + blend * lofi;
            let damped = damp_gain * channel.damp.process(damp_gain * blended);
            channel.line.write(flush(damped));
            channel.tap = channel.line.read_linear(delay);
            wet[ch] = channel.tap;
        }
        let vinyl_gain = self.vinyl_gain.tick();
        let noise = if vinyl_gain > 0.0 {
            vinyl_gain * self.vinyl_noise(wet)
        } else {
            0.0
        };
        let feed = [wet[0] + noise, wet[1] - noise];
        let wet_gain = self.wet.step();
        let dry_gain = self.dry.step();
        let mut out = [0.0; 2];
        for (ch, channel) in self.channels.iter_mut().enumerate() {
            let filtered = filter_gain * channel.filter.process(feed[ch]);
            out[ch] = wet_gain * filtered + dry_gain * frame[ch];
        }
        out
    }

    fn clear(&mut self) {
        for ch in &mut self.channels {
            ch.feedback_hp.clear();
            ch.held = 0.0;
            ch.levels = [0.0; 2];
            ch.damp.clear();
            ch.line.clear();
            ch.tap = 0.0;
            ch.filter.clear();
        }
        self.detector = 0.0;
        self.amount = 0.0;
        for s in &mut self.click_eq {
            s.clear();
        }
        self.hiss_lp.clear();
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(LoFiDly::new(sample_rate))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quantizer_steps_and_saturates() {
        // 4 bits: steps of 2^-3 of the ±1 Q31 range, saturating at 1
        let q = quantize(0.3, 0.0, 28, -1);
        assert!((q - 0.25).abs() < 1e-6, "{q}");
        assert!((quantize(-3.0, 0.0, 28, -1) + 0.875).abs() < 1e-6);
        // 4.5 bits: float step 2^-3.5
        let q = quantize(0.2, 0.088_388_4, 0, -1);
        assert!((q - 2.0 * 0.088_388_4).abs() < 1e-6);
        // the LFSR low bit clears the LSB
        assert!((quantize(0.3, 0.0, 28, 0) - 0.25).abs() < 1e-6);
        assert!((quantize(0.4, 0.0, 28, 0) - 0.25).abs() < 1e-6);
    }

    #[test]
    fn sample_hold_keeps_the_44k_rate() {
        let mut effect = LoFiDly::new(88_200.0);
        let mut values = crate::Model::LoFiDly.defaults();
        values[3] = 13.0; // 1/2: hold 2 frames at 44.1 kHz = 4 frames here
        values[4] = 20.0;
        values[5] = 100.0;
        values[2] = 100.0;
        values[1] = 0.0;
        values[7] = 0.0;
        effect.set(&values, 120.0);
        let mut holds = 0_i32;
        for _ in 0..400 {
            effect.process([0.1, 0.1]);
            if effect.hold_phase < effect.hold_step {
                holds += 1;
            }
        }
        assert!((holds - 100_i32).abs() <= 2, "{holds}");
    }

    #[test]
    fn vinyl_and_max_feedback_stay_bounded() {
        let sr = 48_000.0;
        let mut effect = LoFiDly::new(sr);
        let mut values = crate::Model::LoFiDly.defaults();
        values[1] = 100.0;
        values[8] = 18.0;
        effect.set(&values, 120.0);
        let mut energy = 0.0;
        for i in 0..(sr as usize * 4) {
            let x = if i < 2000 { 0.5 } else { 0.0 };
            let y = effect.process([x, x]);
            assert!(y[0].is_finite() && y[0].abs() < 4.0);
            if i > sr as usize * 3 {
                energy += y[0] * y[0];
            }
        }
        assert!(energy > 0.0);
    }
}
