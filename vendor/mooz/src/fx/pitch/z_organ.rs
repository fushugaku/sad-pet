//! Z-Organ (Z_ORGAN.ZDL): organ tone from octave-up and octave-down
//! regenerating pitch shifters.
//!
//! Callback: recovered `z_organ.rs`, mono out. Mono → 50 Hz high-pass
//! (c40..c42) → 142-sample look-ahead delay; a per-block peak compressor
//! (−40 dB threshold, 1:20, +15 dB, `PacketComp_init`) with a smoothed gain
//! (c49/c50) drives the delayed signal, clipped to ±1. Two two-tap shifters
//! run at half rate (every second frame, like the original): Upper +1
//! octave (c9, 2029-sample sweep) through a 1.7 kHz low-pass loop filter,
//! Lower −1 octave (c10, 1083.5 samples) through a 100 Hz high-pass. Each
//! feeds back its own (sign-inverted, trapezoid-windowed) output scaled by
//! 0.25 / 0.45 (`Hrmncs`) and by an input envelope (621 Hz high-passed peak
//! ×501, c51..c60), so the octaves regenerate into organ-like stacks while
//! notes sound. Upper/Lower are smoothed by the 0.5 one-pole (c61/c62) and
//! mixed with the dry mono (c8); then the LPF (c63..c67) and HPF (c68..c72)
//! biquads and `c2`.
//!
//! Editors decoded: Mix_UP c23 = 0.53·Upper/100, Mix_Dn c24 = Lower/100,
//! Mix_DRY c8 = Dry/100 (16 ms `context[7]` ramps), PostHPF/PostLPF copy
//! `Z_ORGAN_HPF`/`Z_ORGAN_LPF` rows, Out_Lv c2. Half-rate filters are
//! re-derived from 22.05 kHz to half the host rate, the rest from 44.1 kHz.
use super::{
    common::{
        FADE_SECONDS, TimedRamp, decay, out_level, percent, rom_biquad, rom_first_order, samples,
        step, sweep, trapezoid,
    },
    rom::{Z_ORGAN_HPF, Z_ORGAN_LOWER_FILTER, Z_ORGAN_LPF, Z_ORGAN_ROM, Z_ORGAN_UPPER_FILTER, f},
};
use crate::{
    Effect, Values,
    catalog::*,
    dsp::{Biquad, DelayLine, flush},
    zoom::{DEFAULT_STEP, ORIGINAL_RATE, Ramp},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_z_organ",
    name: "Z-Organ",
    category: Category::Synth,
    zdl: "Z_ORGAN.ZDL",
    description: "Organ sound from octave up/down regeneration",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Upper",
            max: 100,
            default: 70,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Lower",
            max: 100,
            default: 80,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Dry",
            max: 100,
            default: 80,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "HPF",
            max: 10,
            default: 3,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "LPF",
            max: 10,
            default: 8,
            display: Display::INT,
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

/// The rate the harmonic shifters ran at.
const HALF_RATE: f32 = ORIGINAL_RATE / 2.0;
/// `PacketComp_init`: −40 dB threshold, slope 1/20, +15 dB make-up.
const THRESHOLD: f32 = 0.01;
const SLOPE: f32 = 0.05;
const MAKEUP: f32 = 5.623_413;
/// `Hrmncs`: loop feedback of the upper and lower shifters (c21, c22).
const FEEDBACK: [f32; 2] = [0.25, 0.45];

/// One half-rate regenerating octave shifter.
struct Harmonic {
    line: DelayLine,
    loop_filter: Biquad,
    phase: u32,
    increment: u32,
    base: f32,
    depth: f32,
    feedback: f32,
    wet: f32,
}

impl Harmonic {
    fn new(sample_rate: f32, bank: usize) -> Self {
        // Half-rate sample counts scale by (fs/2)/22050 = fs/44100.
        let depth = (Z_ORGAN_ROM[13 + bank] as f32) / 1_048_576.0;
        let words: &[u32] = if bank == 0 {
            &Z_ORGAN_UPPER_FILTER
        } else {
            &Z_ORGAN_LOWER_FILTER
        };
        let increment =
            Z_ORGAN_ROM[9 + bank] as i32 as f64 * f64::from(ORIGINAL_RATE / sample_rate);
        Self {
            line: DelayLine::new(samples(depth + 8.0, sample_rate).ceil() as usize),
            loop_filter: Biquad::new(rom_biquad(words, HALF_RATE, sample_rate * 0.5)),
            phase: 0,
            increment: increment.round() as i64 as u32,
            base: samples(2.0, sample_rate),
            depth: samples(depth, sample_rate),
            feedback: FEEDBACK[bank],
            wet: 0.0,
        }
    }

    #[inline]
    fn tick(&mut self, drive: f32, envelope: f32) {
        self.phase = self.phase.wrapping_add(self.increment);
        let mixed = envelope * (self.wet * self.feedback) + drive;
        self.line.write(self.loop_filter.process(mixed));
        let other = self.phase.wrapping_add(0x8000_0000);
        let a = self
            .line
            .read_linear(self.base + self.depth * sweep(self.phase));
        let b = self.line.read_linear(self.base + self.depth * sweep(other));
        // The window words are negative; the original keeps the sign.
        self.wet = flush(-(trapezoid(self.phase) * a + trapezoid(other) * b));
    }

    fn clear(&mut self) {
        self.line.clear();
        self.loop_filter.clear();
        self.phase = 0;
        self.wet = 0.0;
    }
}

struct ZOrgan {
    sample_rate: f32,
    input: Biquad,
    delay: DelayLine,
    look_ahead: usize,
    block_peak: f32,
    counter: u32,
    block_decay: f32,
    compressor: f32,
    normalization: f32,
    gain_smoothing: f32,
    gain: f32,
    detector: Biquad,
    peak_decay: f32,
    peak: f32,
    envelope_gain: f32,
    envelope: f32,
    harmonics: [Harmonic; 2],
    odd: bool,
    mix_pole: f32,
    mixes: [f32; 2],
    post: [Biquad; 2],
    filters: Option<(usize, usize)>,
    upper: TimedRamp,
    lower: TimedRamp,
    dry: TimedRamp,
    level: Ramp,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let rom = |i: usize| f(Z_ORGAN_ROM[i]);
    let fade = |value| TimedRamp::new(value, FADE_SECONDS, sample_rate);
    let look_ahead = samples(Z_ORGAN_ROM[44] as f32, sample_rate).round() as usize;
    Box::new(ZOrgan {
        sample_rate,
        input: Biquad::new(rom_first_order(
            &Z_ORGAN_ROM[40..43],
            ORIGINAL_RATE,
            sample_rate,
        )),
        delay: DelayLine::new(look_ahead + 2),
        look_ahead,
        block_peak: 0.0,
        counter: 0,
        block_decay: decay(rom(45), sample_rate),
        compressor: 0.0,
        normalization: 1.0,
        gain_smoothing: 1.0 - decay(rom(50), sample_rate),
        gain: 0.0,
        detector: Biquad::new(rom_first_order(
            &Z_ORGAN_ROM[51..54],
            ORIGINAL_RATE,
            sample_rate,
        )),
        peak_decay: decay(rom(58), sample_rate),
        peak: 0.0,
        envelope_gain: 1.0 - decay(rom(60), sample_rate),
        envelope: 0.0,
        harmonics: [Harmonic::new(sample_rate, 0), Harmonic::new(sample_rate, 1)],
        odd: false,
        mix_pole: decay(rom(62), sample_rate),
        mixes: [0.0; 2],
        post: [Biquad::default(); 2],
        filters: None,
        upper: fade(0.53 * 0.7),
        lower: fade(0.8),
        dry: fade(0.8),
        level: Ramp::new(1.0, DEFAULT_STEP, sample_rate),
    })
}

impl Effect for ZOrgan {
    fn set(&mut self, values: &Values, _bpm: f32) {
        self.upper.set(0.53 * percent(values[0]));
        self.lower.set(percent(values[1]));
        self.dry.set(percent(values[2]));
        let filters = (step(values[3], 10), step(values[4], 10));
        if self.filters != Some(filters) {
            let (hpf, lpf) = filters;
            let fs = self.sample_rate;
            self.post[0].coefficients =
                rom_biquad(&Z_ORGAN_LPF[5 * lpf..5 * lpf + 5], ORIGINAL_RATE, fs);
            self.post[1].coefficients =
                rom_biquad(&Z_ORGAN_HPF[5 * hpf..5 * hpf + 5], ORIGINAL_RATE, fs);
            self.filters = Some(filters);
        }
        self.level.set(out_level(values[5]));
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        if self.counter == 0 {
            // Packet gain: block peak hold, then the 1:20 curve above -40 dB.
            self.compressor = self.block_peak.max(self.compressor * self.block_decay);
            self.normalization = if self.compressor > THRESHOLD {
                (THRESHOLD + SLOPE * (self.compressor - THRESHOLD)) / self.compressor
            } else {
                1.0
            };
            self.block_peak = 0.0;
        }
        self.counter = (self.counter + 1) % 8;
        let mono = 0.5 * (left + right);
        let sample = self.input.process(mono);
        self.delay.write(sample);
        self.block_peak = self.block_peak.max(sample);
        let tap = self.delay.read(self.look_ahead);
        self.gain = flush(self.gain + self.gain_smoothing * (self.normalization - self.gain));
        let drive = (MAKEUP * tap * self.gain).clamp(-1.0, 1.0);
        let high = self.detector.process(mono);
        let magnitude = (f(Z_ORGAN_ROM[57]) * (2.0 * high).abs()).min(1.0);
        self.peak = magnitude.max(self.peak * self.peak_decay);
        self.envelope = flush(self.envelope + self.envelope_gain * (self.peak - self.envelope));
        if !self.odd {
            for harmonic in &mut self.harmonics {
                harmonic.tick(drive, self.envelope);
            }
        }
        self.odd = !self.odd;
        let p = self.mix_pole;
        let up = self.upper.tick() * self.harmonics[0].wet;
        let down = self.lower.tick() * self.harmonics[1].wet;
        self.mixes[0] = flush(up + p * (self.mixes[0] - up));
        self.mixes[1] = flush(down + p * (self.mixes[1] - down));
        let mixed = mono * self.dry.tick() + self.mixes[0] + self.mixes[1];
        let lowpassed = self.post[0].process(mixed);
        let filtered = self.post[1].process(lowpassed);
        let out = self.level.tick() * filtered;
        [out, out]
    }

    fn clear(&mut self) {
        self.input.clear();
        self.delay.clear();
        self.block_peak = 0.0;
        self.counter = 0;
        self.compressor = 0.0;
        self.normalization = 1.0;
        self.gain = 0.0;
        self.detector.clear();
        self.peak = 0.0;
        self.envelope = 0.0;
        self.harmonics.iter_mut().for_each(Harmonic::clear);
        self.odd = false;
        self.mixes = [0.0; 2];
        self.post.iter_mut().for_each(Biquad::clear);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fx::pitch::two_tap::tests::centroid;

    #[test]
    fn harmonics_are_an_octave_up_and_down() {
        for (bank, ratio) in [(0usize, 2.0f64), (1, 0.5)] {
            let depth = Z_ORGAN_ROM[13 + bank] as f64 / 1_048_576.0;
            let increment = Z_ORGAN_ROM[9 + bank] as i32 as f64;
            let found = 1.0 - depth * increment / 4_294_967_296.0;
            assert!((found - ratio).abs() < 1e-3, "{bank}: {found}");
        }
    }

    fn render(hz: f32, sample_rate: f32) -> Vec<f32> {
        let mut effect = create(sample_rate);
        let mut values = crate::Model::ZOrgan.defaults();
        values[2] = 0.0;
        values[3] = 0.0;
        values[4] = 10.0;
        effect.set(&values, 120.0);
        let n = sample_rate as usize;
        let out: Vec<f32> = (0..n)
            .map(|i| {
                let x = 0.3 * (i as f32 * hz / sample_rate * std::f32::consts::TAU).sin();
                effect.process([x, x])[0]
            })
            .collect();
        out[n / 2..].to_vec()
    }

    // Frequencies whose taps sit a whole number of cycles apart (half sweep
    // 1014.5 and 541.75 samples at 22.05 kHz), so splices add no offset.
    #[test]
    fn organ_adds_octaves_while_playing() {
        let tail = render(217.35, 48_000.0);
        let rms = (tail.iter().map(|x| x * x).sum::<f32>() / tail.len() as f32).sqrt();
        assert!(rms > 0.02 && rms < 2.0, "{rms}");
        let up = centroid(&tail, 48_000.0, 414.7, 454.7);
        assert!((up - 434.7).abs() < 4.0, "{up}");
        let tail = render(203.5, 44_100.0);
        let down = centroid(&tail, 44_100.0, 91.75, 111.75);
        assert!((down - 101.75).abs() < 3.0, "{down}");
    }
}
