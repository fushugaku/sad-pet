//! MonoSynth (MONOSYN.ZDL): pitch-tracking monophonic synth voice.
//!
//! Callback: recovered `mono_synth.rs`. The input runs through the same
//! detector chain as Octave (c5..c16). Decaying peak trackers (c25) and a
//! sign-change test give one trigger per period; the samples since the last
//! trigger (`age`, below c36 = 2000 samples) set the oscillator step
//! `2^31 / (c37 · age)` (c37 = 0.5 for Sine/Tri: the input pitch; 1.0 for
//! the saws: an octave below), glided by the Speed one-pole (c28/c29). A
//! peak envelope (c30) opens a hysteresis gate (above c34 = -40 dB, closed
//! below c35 = -60 dB), smoothed by c21/c22, which sets the voice amplitude.
//! The Q31 oscillator mixes a saw (c31), a triangle (c32) and a soft
//! parabolic sine (c33); two first-order stages per wave (c38..c43) and the
//! Tone one-pole (c44/c45) follow; `out = c2 (Dry in + Synth voice)`.
//!
//! Editors decoded: Wave copies c31..c33, c37, c38..c43 from the wave tables
//! (with the 16 ms c3 mute around it); Tone = `context[21]` from pole 0.9311
//! to 0.473; Speed = Q31 `context[22]` from 0x7f000000 to 0x7ff80000 (c29,
//! c28 = 1 - c29); Synth/Dry linear with 16 ms ramps; Level c2. Rates and
//! filters are rescaled from 44.1 kHz. The synth sits at the ROM's fixed
//! level (about -22 dBFS for Sine), as on the pedal.
use super::{
    common::{FADE_SECONDS, TimedRamp, decay, out_level, percent, rom_first_order, step},
    octave::detector_chain,
    rom::{
        MONO_SYNTH_FIRST, MONO_SYNTH_PERIOD, MONO_SYNTH_ROM, MONO_SYNTH_SECOND, MONO_SYNTH_WAVE, f,
        q31,
    },
};
use crate::{
    Effect, Values,
    catalog::*,
    dsp::{Biquad, flush},
    zoom::{self, DEFAULT_STEP, ORIGINAL_RATE, Ramp},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_mono_synth",
    name: "MonoSynth",
    category: Category::Synth,
    zdl: "MONOSYN.ZDL",
    description: "Guitar synth: a wave that tracks the played note",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Synth",
            max: 100,
            default: 40,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Dry",
            max: 100,
            default: 100,
            display: Display::INT,
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
            name: "Wave",
            max: 3,
            default: 2,
            display: Display::labels(&["Sine", "Tri", "SawUp", "SawDn"]),
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Tone",
            max: 10,
            default: 8,
            display: Display::INT,
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "Speed",
            max: 100,
            default: 0,
            display: Display::INT,
        },
    ],
};

struct MonoSynth {
    sample_rate: f32,
    detector: [Biquad; 4],
    peak_decay: f32,
    envelope_decay: f32,
    gate_gain: f32,
    gate_on: f32,
    gate_off: f32,
    max_age: f32,
    peaks: [f32; 2],
    seen: [bool; 2],
    negative: bool,
    envelope: f32,
    gate: f32,
    gate_smooth: f32,
    age: f32,
    step: f64,
    increment: f64,
    glide: f64,
    phase: u32,
    wave: Option<usize>,
    weights: [f32; 3],
    period_scale: f32,
    stages: [Biquad; 2],
    pending_wave: Option<usize>,
    tone_pole: f32,
    smoothed: f32,
    mute: TimedRamp,
    synth: TimedRamp,
    dry: TimedRamp,
    level: Ramp,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let rom = |i: usize| f(MONO_SYNTH_ROM[i]);
    let fade = |value| TimedRamp::new(value, FADE_SECONDS, sample_rate);
    Box::new(MonoSynth {
        sample_rate,
        detector: detector_chain(&MONO_SYNTH_ROM, sample_rate),
        peak_decay: decay(rom(25), sample_rate),
        envelope_decay: decay(rom(30), sample_rate),
        gate_gain: 1.0 - decay(rom(22), sample_rate),
        gate_on: rom(34),
        gate_off: rom(35),
        max_age: rom(36) * sample_rate / ORIGINAL_RATE,
        peaks: [0.0; 2],
        seen: [false; 2],
        negative: false,
        envelope: 0.0,
        gate: 0.0,
        gate_smooth: 0.0,
        age: 0.0,
        step: 0.0,
        increment: 0.0,
        glide: 0.99,
        phase: 0,
        wave: None,
        weights: [0.0; 3],
        period_scale: 1.0,
        stages: [Biquad::default(); 2],
        pending_wave: None,
        tone_pole: 0.0,
        smoothed: 0.0,
        mute: fade(1.0),
        synth: fade(0.4),
        dry: fade(1.0),
        level: Ramp::new(1.0, DEFAULT_STEP, sample_rate),
    })
}

impl MonoSynth {
    fn apply_wave(&mut self, wave: usize) {
        let fs = self.sample_rate;
        self.weights = std::array::from_fn(|i| q31(MONO_SYNTH_WAVE[3 * wave + i]));
        self.period_scale = f(MONO_SYNTH_PERIOD[wave]);
        let first = &MONO_SYNTH_FIRST[3 * wave..3 * wave + 3];
        let second = &MONO_SYNTH_SECOND[3 * wave..3 * wave + 3];
        self.stages[0].coefficients = rom_first_order(first, ORIGINAL_RATE, fs);
        self.stages[1].coefficients = rom_first_order(second, ORIGINAL_RATE, fs);
        self.wave = Some(wave);
    }

    /// Saw (c31), folded triangle (c32) and parabolic sine (c33), saturated.
    #[inline]
    fn oscillator(&self) -> f32 {
        let p = self.phase as i32 as f32 * (1.0 / 2_147_483_648.0);
        let triangle = p.abs() - 0.5;
        let curved = triangle * (1.0 - triangle.abs());
        let [saw, folded, sine] = self.weights;
        (saw * p + folded * (2.0 * triangle) + sine * (4.0 * curved).clamp(-1.0, 1.0))
            .clamp(-1.0, 1.0)
    }
}

impl Effect for MonoSynth {
    fn set(&mut self, values: &Values, _bpm: f32) {
        self.synth.set(percent(values[0]));
        self.dry.set(percent(values[1]));
        self.level.set(out_level(values[2]));
        let wave = step(values[3], 3);
        if self.wave.is_none() {
            self.apply_wave(wave);
        } else if self.wave != Some(wave) {
            self.pending_wave = Some(wave);
            self.mute.set(0.0);
        }
        let pole = zoom::curve(0.473, 0.9311, 11, step(values[4], 10) as f32, 0);
        self.tone_pole = decay(pole, self.sample_rate);
        let speed = zoom::curve(
            0.999_755_86,
            0.992_187_5,
            101,
            values[5].clamp(0.0, 100.0),
            0,
        );
        self.glide = f64::from(decay(speed, self.sample_rate));
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        if let Some(wave) = self.pending_wave
            && self.mute.current <= 0.0
        {
            self.pending_wave = None;
            self.apply_wave(wave);
            self.mute.set(1.0);
        }
        self.phase = self.phase.wrapping_add(self.increment as i64 as u32);
        let oscillator = self.oscillator();
        let mean = 0.5 * (left + right);
        let signal = self
            .detector
            .iter_mut()
            .fold(mean, |x, stage| stage.process(x));
        self.age += 1.0;
        let positive_peak = signal > self.peaks[0];
        let negative_peak = signal < self.peaks[1];
        let negative = signal < 0.0;
        let edge = negative != self.negative;
        self.negative = negative;
        self.seen[0] |= positive_peak;
        self.seen[1] |= negative_peak;
        let magnitude = signal.abs();
        self.envelope = if magnitude > self.envelope {
            magnitude
        } else {
            flush(self.envelope * self.envelope_decay)
        };
        let below = self.envelope < self.gate_off;
        if self.envelope > self.gate_on {
            self.gate = 1.0;
        }
        if below {
            self.gate = 0.0;
        }
        let trigger = self.seen[0] && self.seen[1] && edge && !below;
        self.peaks[0] = if positive_peak {
            signal
        } else {
            self.peaks[0] * self.peak_decay
        };
        self.peaks[1] = if negative_peak {
            signal
        } else {
            self.peaks[1] * self.peak_decay
        };
        if below {
            self.seen = [false; 2];
        }
        self.gate_smooth =
            flush(self.gate_smooth + self.gate_gain * (self.gate - self.gate_smooth));
        self.increment = self.increment * self.glide + (1.0 - self.glide) * self.step;
        if trigger {
            if self.age < self.max_age {
                let step = 2_147_483_648.0 / f64::from(self.period_scale * self.age);
                self.step = step.min(2_147_483_647.0);
            }
            self.age = 0.0;
            self.seen = [false; 2];
        }
        let mute = self.mute.tick();
        let driven = self.gate_smooth * oscillator * mute;
        let first = self.stages[0].process(driven);
        let shaped = self.stages[1].process(first);
        self.smoothed = flush(shaped + self.tone_pole * (self.smoothed - shaped));
        let wet = self.smoothed * self.synth.tick();
        let (dry, level) = (self.dry.tick(), self.level.tick());
        [level * (dry * left + wet), level * (dry * right + wet)]
    }

    fn clear(&mut self) {
        self.detector.iter_mut().for_each(Biquad::clear);
        self.stages.iter_mut().for_each(Biquad::clear);
        self.peaks = [0.0; 2];
        self.seen = [false; 2];
        self.negative = false;
        self.envelope = 0.0;
        self.gate = 0.0;
        self.gate_smooth = 0.0;
        self.age = 0.0;
        self.step = 0.0;
        self.increment = 0.0;
        self.phase = 0;
        self.smoothed = 0.0;
        if let Some(wave) = self.pending_wave.take() {
            self.apply_wave(wave);
        }
        self.mute.jump(1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fx::pitch::two_tap::tests::centroid;

    fn synth(wave: f32, hz: f32, sample_rate: f32) -> Vec<f32> {
        let mut effect = create(sample_rate);
        let mut values = crate::Model::MonoSynth.defaults();
        values[0] = 100.0;
        values[1] = 0.0;
        values[3] = wave;
        values[4] = 10.0;
        effect.set(&values, 120.0);
        (0..sample_rate as usize)
            .map(|i| {
                let p = i as f32 * hz / sample_rate * std::f32::consts::TAU;
                effect.process([0.3 * p.sin(), 0.3 * p.sin()])[0]
            })
            .collect()
    }

    #[test]
    fn sine_tracks_the_input_pitch() {
        let out = synth(0.0, 196.0, 48_000.0);
        let found = centroid(&out[24_000..], 48_000.0, 176.0, 216.0);
        assert!((found - 196.0).abs() < 2.0, "{found}");
        let peak = out[24_000..].iter().fold(0.0f32, |a, &x| a.max(x.abs()));
        assert!(peak > 0.03 && peak < 0.2, "{peak}");
    }

    #[test]
    fn saws_play_an_octave_below() {
        let out = synth(2.0, 220.0, 44_100.0);
        let found = centroid(&out[22_050..], 44_100.0, 100.0, 120.0);
        assert!((found - 110.0).abs() < 2.0, "{found}");
    }

    #[test]
    fn silence_closes_the_gate() {
        let mut effect = create(44_100.0);
        effect.set(&crate::Model::MonoSynth.defaults(), 120.0);
        for i in 0..44_100 {
            let x = if i < 22_050 {
                0.3 * (i as f32 * 0.03).sin()
            } else {
                0.0
            };
            effect.process([x, x]);
        }
        let out = effect.process([0.0, 0.0]);
        assert!(out[0].abs() < 1e-3, "{out:?}");
    }
}
