//! SilkyCho (SILKYCHO.ZDL): two detuned voices plus a chorus.
//!
//! Callback `Fx_MOD_SilkyCho`, front stage (per sample):
//! - `mono = (L + R) / 2` is written to ring 0.
//! - Two pitch shifters ("Lo" c12..c19, "Hi" c38..c45), each a sawtooth
//!   phase read by two taps half a turn apart: delay
//!   `PreD + (p/2 + 1/2) * 722.5` samples, window
//!   `min(1, 1.613 (1 - min(|p| + 0.07, 1)))` (silent near the jump).
//!   The increments come from the 51-entry cent table (LoPit/HiPit - 25 cents).
//! - Lo voice: one-pole low-pass at ~800 Hz (c26/c27), then the low-pass half
//!   of the c5 one-pole crossover (~800 Hz), times LoMix.
//!   Hi voice: one-pole low-pass at ~1 kHz (c52/c53), then the high-pass half
//!   of the same crossover, times HiMix.
//! - `front = mono + lo + hi` (c23 = 1).
//!
//! Chorus stage: `front` -> ring 1 -> linear tap at `132 samples + center +
//! amplitude * wave` (the SuperCho LFO and rate/depth editors, polynomial
//! `0.6 |p| + 0.4 p^2`) -> ROM 51 Hz high-pass -> ROM 10 kHz low-pass
//! (Q 0.78) -> ChMix. The identity input/feedback/write biquads and the zero
//! feedback gain c71 are left out.
//!
//! Output: `wet = tone(chorus + lo + hi)` (first-order low-pass, table
//! 400..16000 Hz) added to the untouched stereo input.
//!
//! Editors: `Lo_mix_edit`/`Hi_mix_edit` `curve(1, 0, shape 2)` over 705.6
//! samples; `chmix_edit` `ChMix / 100`; `predly_edit` `44.1 * PreD + 2`
//! samples for both voices (smoothed here instead of the original jump);
//! `Lo_cent_edit`/`Hi_cent_edit` table lookups; `rate_edit`, `depth_edit` as
//! SuperCho's; `tone_edit` designer type 0. The 2-sample pipeline delay of
//! the crossfade windows and the startup fade (s27/s37) are not modelled.
use super::common::BbdLfo;
use super::common::{Filter, Glide, Kind, Lag, design, rescale_pole, rom, table};
use super::super_cho::{depth_ms, rate_words};
use crate::dsp::{DelayLine, Smoother};
use crate::zoom::{DEFAULT_STEP, ORIGINAL_RATE, Ramp, curve};
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_silky_cho",
    name: "SilkyCho",
    category: Category::Modulation,
    zdl: "SILKYCHO.ZDL",
    description: "Chorus blended with two slightly detuned voices",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "LoMix",
            max: 100,
            default: 38,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "HiMix",
            max: 100,
            default: 100,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "ChMix",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "LoPit",
            max: 50,
            default: 31,
            display: Display::offset(-25.0),
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "HiPit",
            max: 50,
            default: 33,
            display: Display::offset(-25.0),
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "PreD",
            max: 50,
            default: 27,
            display: Display::number(0.0, 1.0, 0, " ms"),
        },
        // values[6] is firmware parameter word 8
        ParamInfo {
            name: "Rate",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[7] is firmware parameter word 9
        ParamInfo {
            name: "Depth",
            max: 100,
            default: 46,
            display: Display::INT,
        },
        // values[8] is firmware parameter word 10
        ParamInfo {
            name: "Tone",
            max: 100,
            default: 50,
            display: Display::INT,
        },
    ],
};

/// SILKYCHO 0x800007c8: Q32 detune increments at 44.1 kHz for -25..+25 cents.
const CENT_INCREMENT: [f32; 51] = [
    85285.0, 81898.0, 78508.0, 75116.0, 71722.0, 68327.0, 64929.0, 61530.0, 58128.0, 54724.0,
    51319.0, 47912.0, 44502.0, 41091.0, 37677.0, 34262.0, 30845.0, 27425.0, 24004.0, 20581.0,
    17156.0, 13729.0, 10299.0, 6868.0, 3435.0, 0.0, -3437.0, -6876.0, -10317.0, -13760.0, -17205.0,
    -20652.0, -24101.0, -27552.0, -31006.0, -34461.0, -37918.0, -41377.0, -44838.0, -48301.0,
    -51766.0, -55233.0, -58702.0, -62173.0, -65646.0, -69121.0, -72598.0, -76077.0, -79558.0,
    -83041.0, -86526.0,
];
/// SILKYCHO 0x80000898: tone low-pass corners.
const TONE_HZ: [f32; 11] = [
    400.0, 800.0, 1000.0, 2000.0, 3000.0, 4800.0, 6000.0, 8000.0, 10000.0, 12000.0, 16000.0,
];
/// c15 = 0x5a5119cc as Q31 over the Q21 tap scale: the sawtooth sweep.
const SWEEP: f32 = 1_515_264_460.0 / 2_097_152.0;

/// One pitch shifter: a sawtooth phase and its two windowed taps.
#[derive(Debug, Clone, Copy, Default)]
struct Detune {
    phase: u32,
    increment: u32,
}

impl Detune {
    #[inline]
    fn window(x: f32) -> f32 {
        (1.612_903_1 * (1.0 - (x.abs() + 0.07).min(1.0))).min(1.0)
    }

    #[inline]
    fn process(&mut self, line: &DelayLine, base: f32, sweep: f32) -> f32 {
        self.phase = self.phase.wrapping_add(self.increment);
        let mut sum = 0.0;
        for offset in [0, 0x8000_0000u32] {
            let x = self.phase.wrapping_add(offset) as i32 as f32 * (1.0 / 2_147_483_648.0);
            let delay = base + (0.5 * x + 0.5) * sweep;
            sum += line.read_linear(delay) * Self::window(x);
        }
        sum
    }
}

struct SilkyCho {
    sample_rate: f32,
    scale: f32,
    front_line: DelayLine,
    chorus_line: DelayLine,
    voices: [Detune; 2],
    predelay: Smoother,
    voice_lowpass: [Lag; 2],
    crossover: [Lag; 2],
    mixes: [Glide; 2],
    lfo: BbdLfo,
    chorus_filters: [Filter; 2],
    chorus_mix: Ramp,
    tone: Filter,
    last: Values,
    fresh: bool,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let scale = sample_rate / ORIGINAL_RATE;
    let mut chorus_filters = [Filter::default(); 2];
    chorus_filters[0].set(rom(0.996_38, -0.996_38, 0.0, 0.992_76, 0.0, sample_rate));
    chorus_filters[1].set(rom(
        0.261_436_67,
        0.522_873_3,
        0.261_436_67,
        0.178_092_06,
        -0.223_838_76,
        sample_rate,
    ));
    let crossover = rescale_pole(1.0 - 0.892_143_85, sample_rate);
    let mut effect = SilkyCho {
        sample_rate,
        scale,
        front_line: DelayLine::with_seconds(0.07, sample_rate),
        chorus_line: DelayLine::with_seconds(0.013, sample_rate),
        voices: [Detune::default(); 2],
        predelay: Smoother::new(0.0, 0.03, sample_rate),
        voice_lowpass: [
            Lag::new(rescale_pole(0.107_856_12, sample_rate)),
            Lag::new(rescale_pole(0.132_875_76, sample_rate)),
        ],
        crossover: [Lag::new(crossover), Lag::new(crossover)],
        mixes: [Glide::new(0.0, 705.6, sample_rate); 2],
        lfo: BbdLfo::new(sample_rate, 0.6, 0.4),
        chorus_filters,
        chorus_mix: Ramp::new(0.0, DEFAULT_STEP, sample_rate),
        tone: Filter::default(),
        last: [f32::NAN; crate::MAX_PARAMS],
        fresh: true,
    };
    effect.set(&crate::Model::SilkyCho.defaults(), 120.0);
    Box::new(effect)
}

impl Effect for SilkyCho {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let jump = self.fresh;
        self.fresh = false;
        for (voice, index) in [(0, 0), (1, 1)] {
            if values[index] != self.last[index] {
                let mix = curve(1.0, 0.0, 101, values[index], 2);
                if jump {
                    self.mixes[voice].jump(mix);
                } else {
                    self.mixes[voice].set(mix);
                }
            }
        }
        if values[2] != self.last[2] {
            let mix = curve(1.2, 0.0, 121, values[2], 0);
            if jump {
                self.chorus_mix.jump(mix);
            } else {
                self.chorus_mix.set(mix);
            }
        }
        for (voice, index) in [(0, 3), (1, 4)] {
            if values[index] != self.last[index] {
                let increment = table(&CENT_INCREMENT, values[index], 50.0) / self.scale;
                self.voices[voice].increment = (increment.round() as i32) as u32;
            }
        }
        if values[5] != self.last[5] {
            let samples = (44.1 * values[5].clamp(0.0, 50.0) + 2.0) * self.scale;
            if jump {
                self.predelay.jump(samples);
            } else {
                self.predelay.set(samples);
            }
        }
        if values[6] != self.last[6] {
            let (increment, scale) = rate_words(values[6]);
            self.lfo.set_increment(increment, self.sample_rate);
            self.lfo.scale = scale;
        }
        if values[7] != self.last[7] {
            let (center, amplitude) = depth_ms(values[7]);
            self.lfo.set_targets(center, amplitude);
        }
        if values[8] != self.last[8] {
            let hz = table(&TONE_HZ, values[8], 100.0);
            self.tone
                .set(design(Kind::LowPass1, self.sample_rate, hz, 0.0, 0.78));
        }
        self.last = *values;
    }

    fn process(&mut self, [l, r]: [f32; 2]) -> [f32; 2] {
        let mono = (l + r) * 0.5;
        self.front_line.write(mono);
        let base = self.predelay.tick();
        let sweep = SWEEP * self.scale;
        let low = self.voices[0].process(&self.front_line, base, sweep);
        let high = self.voices[1].process(&self.front_line, base, sweep);
        let low = self.crossover[0].process(self.voice_lowpass[0].process(low));
        let high = self.voice_lowpass[1].process(high);
        let high = high - self.crossover[1].process(high);
        let voices = low * self.mixes[0].tick() + high * self.mixes[1].tick();
        self.chorus_line.write(mono + voices);
        let delay = 132.0 * self.scale + self.lfo.tick();
        let [hp, lp] = &mut self.chorus_filters;
        let chorus = lp.process(hp.process(self.chorus_line.read_linear(delay)));
        let wet = self.tone.process(chorus * self.chorus_mix.tick() + voices);
        [l + wet, r + wet]
    }

    fn clear(&mut self) {
        self.front_line.clear();
        self.chorus_line.clear();
        self.voices.iter_mut().for_each(|v| v.phase = 0);
        self.voice_lowpass.iter_mut().for_each(|f| f.y = 0.0);
        self.crossover.iter_mut().for_each(|f| f.y = 0.0);
        self.lfo.clear();
        self.chorus_filters.iter_mut().for_each(Filter::clear);
        self.tone.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cent_table_gives_the_labelled_detune() {
        // Delay slope of a voice = sweep * increment / 2^32 per sample.
        let cents = |v: f32| {
            let slope = SWEEP as f64 * table(&CENT_INCREMENT, v, 50.0) as f64 / 4_294_967_296.0;
            1200.0 * (1.0 - slope).log2()
        };
        assert!((cents(0.0) + 25.0).abs() < 0.1, "{}", cents(0.0));
        assert!(cents(25.0).abs() < 1e-9);
        assert!((cents(50.0) - 25.0).abs() < 0.1, "{}", cents(50.0));
        assert!((cents(31.0) - 6.0).abs() < 0.1, "{}", cents(31.0));
    }

    #[test]
    fn windows_meet_in_the_middle_and_vanish_at_the_jump() {
        assert_eq!(Detune::window(0.0), 1.0);
        assert!(Detune::window(0.95) < 1e-6);
        assert!(Detune::window(-0.99) < 1e-6);
        let sum = Detune::window(0.5) + Detune::window(-0.5);
        assert!((sum - 1.387).abs() < 0.01, "{sum}");
    }

    #[test]
    fn dry_passes_untouched_with_everything_off() {
        let mut effect = create(48_000.0);
        let mut values = crate::Model::SilkyCho.defaults();
        values[0] = 0.0;
        values[1] = 0.0;
        values[2] = 0.0;
        effect.set(&values, 120.0);
        for i in 0..4800 {
            let x = (i as f32 * 0.01).sin() * 0.5;
            let out = effect.process([x, -x]);
            assert!((out[0] - x).abs() < 1e-6 && (out[1] + x).abs() < 1e-6);
        }
    }
}
