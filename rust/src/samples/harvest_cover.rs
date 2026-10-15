//! Harvest Dawn: the recognizable harp introduction, arranged as a periodic loop.
//! Composition: Jeremy Soule, Oblivion (2006). Opening pitches/rhythm checked
//! against Piano Letter Notes and Harry Murrell's Gametabs transcription.
//! See assets/samples/SOURCES.txt. No soundtrack recording is sampled.
use super::{Track, hz};
use crate::{Stereo, dsp::SampleBuffer, tables::Random};
use std::f64::consts::PI;

pub(super) const BPM: u32 = 90;
pub(super) const SECONDS: f64 = 64.0 * 60.0 / BPM as f64;

// Eight-bar opening. Beat positions and note lengths are quarter-note units;
// the second pass changes orchestration and dynamics, not the melody's rhythm.
const MELODY: &[(f64, f64, u8)] = &[
    (0.0, 1.0, 69),
    (1.0, 1.0, 62),
    (2.0, 0.5, 65),
    (2.5, 0.5, 64),
    (3.0, 1.0, 65),
    (4.0, 1.0, 67),
    (5.0, 2.0, 62),
    (7.0, 1.0, 62),
    (8.0, 1.0, 60),
    (9.0, 1.0, 57),
    (10.0, 1.0, 57),
    (11.0, 1.0, 53),
    (12.0, 4.0, 55),
    (16.0, 1.0, 69),
    (17.0, 1.0, 62),
    (18.0, 0.5, 65),
    (18.5, 0.5, 64),
    (19.0, 1.0, 65),
    (20.0, 0.5, 67),
    (20.5, 0.5, 69),
    (21.0, 2.0, 65),
    (23.0, 1.0, 64),
    (24.0, 1.0, 62),
    (25.0, 1.0, 58),
    (26.0, 2.0, 60),
    (28.0, 4.0, 62),
];

struct Arrangement {
    frames: Vec<Stereo>,
    rate: u32,
}

// Offline oscillator recurrence avoids hundreds of millions of sin/exp calls
// when the expanded file is first generated on a phone. Each strike still has
// independent modes, stiffness, damping and paired strings.
struct Oscillator {
    sine: f64,
    cosine: f64,
    step_sine: f64,
    step_cosine: f64,
}

impl Oscillator {
    fn new(frequency: f64, rate: u32) -> Self {
        let step = 2.0 * PI * frequency / rate as f64;
        Self {
            sine: 0.0,
            cosine: 1.0,
            step_sine: step.sin(),
            step_cosine: step.cos(),
        }
    }
    fn tick(&mut self) -> f64 {
        let value = self.sine;
        let next = self.sine * self.step_cosine + self.cosine * self.step_sine;
        self.cosine = self.cosine * self.step_cosine - self.sine * self.step_sine;
        self.sine = next;
        value
    }
}

impl Arrangement {
    fn voice(&mut self, start: f64, duration: f64, pan: f64, mut signal: impl FnMut(f64) -> f64) {
        let size = self.frames.len();
        let start =
            ((start * self.rate as f64).round() as isize).rem_euclid(size as isize) as usize;
        let length = (duration * self.rate as f64).ceil() as usize;
        let gains = [(1.0 - pan).sqrt() as f32, pan.sqrt() as f32];
        for i in 0..length {
            let t = i as f64 / self.rate as f64;
            let release = ((duration - t) / 0.012).clamp(0.0, 1.0);
            let value = (signal(t) * release) as f32;
            let frame = &mut self.frames[(start + i) % size];
            for (channel, gain) in gains.into_iter().enumerate() {
                frame[channel] += value * gain;
            }
        }
    }

    fn harp(&mut self, beat: f64, gate: f64, note: u8, level: f64, seed: u64) {
        let frequency = hz(note);
        let decay = 1.65 * (220.0 / frequency).powf(0.22);
        let pan = (0.42 + (note as f64 - 62.0) / 150.0).clamp(0.2, 0.72);
        let mut partials: Vec<_> = (1..=14)
            .filter_map(|h| {
                let h = h as f64;
                // Pluck-position notches, string stiffness and faster damping of
                // upper modes give a plucked envelope rather than an organ tone.
                let ratio = h * (1.0 + 0.000025 * h * h).sqrt();
                (frequency * ratio < self.rate as f64 * 0.44).then_some((
                    Oscillator::new(frequency * ratio, self.rate),
                    Oscillator::new(frequency * ratio * 1.0009, self.rate),
                    (PI * h * 0.24).sin() / h.powf(1.35),
                    (-(1.0 + 0.12 * (h - 1.0).powf(1.5)) / decay / self.rate as f64).exp(),
                ))
            })
            .collect();
        let mut random = Random(0x853c_49e6_748f_ea9b ^ seed);
        let attack_step = (-1.0 / (self.rate as f64 * 0.0015)).exp();
        let finger_step = (-1.0 / (self.rate as f64 * 0.004)).exp();
        let damping_step = (-1.0 / (self.rate as f64 * 1.5)).exp();
        let (mut attack, mut finger_env, mut damping) = (1.0, 1.0, 1.0);
        self.voice(beat * 60.0 / BPM as f64, gate + 5.5, pan, |t| {
            let mut tone = 0.0;
            for (string, coupled, amplitude, decay_step) in &mut partials {
                tone += (string.tick() + 0.22 * coupled.tick()) * *amplitude;
                *amplitude *= *decay_step;
            }
            let finger = random.bipolar() as f64 * finger_env * 0.018;
            let value = (tone + finger) * (1.0 - attack) * damping * level;
            attack *= attack_step;
            finger_env *= finger_step;
            if t > gate + 0.12 {
                damping *= damping_step;
            }
            value
        });
    }

    fn strings(&mut self, beat: f64, gate: f64, notes: &[u8], level: f64) {
        for &note in notes {
            for (detune, pan) in [(0.9992, 0.16), (1.0008, 0.84)] {
                let frequency = hz(note) * detune;
                let mut oscillators =
                    [1.0, 2.0, 3.0, 4.0].map(|h| Oscillator::new(frequency * h, self.rate));
                let attack_step = (-1.0 / (self.rate as f64 * 0.48)).exp();
                let release_step = (-1.0 / (self.rate as f64 * 0.5)).exp();
                let (mut attack, mut release) = (1.0, 1.0);
                self.voice(beat * 60.0 / BPM as f64, gate + 2.0, pan, |t| {
                    let env = (1.0 - attack) * release;
                    let tone: f64 = oscillators
                        .iter_mut()
                        .zip([1.0, 0.19, 0.08, 0.035])
                        .map(|(oscillator, gain)| oscillator.tick() * gain)
                        .sum();
                    attack *= attack_step;
                    if t > gate {
                        release *= release_step;
                    }
                    tone * env * level
                });
            }
        }
    }

    fn room(&mut self) {
        struct Comb {
            buffer: Vec<f64>,
            position: usize,
            low: f64,
            feedback: f64,
        }
        let mut combs: Vec<_> = [0.0311, 0.0377, 0.0419, 0.0473, 0.0539, 0.0617]
            .into_iter()
            .map(|seconds| Comb {
                buffer: vec![0.0; (seconds * self.rate as f64).round() as usize],
                position: 0,
                low: 0.0,
                feedback: 10.0_f64.powf(-3.0 * seconds / 2.0),
            })
            .collect();
        let dry = self.frames.clone();
        // Prime the room with one complete previous period, retaining the
        // ending's decay at the start without fading the whole loop to silence.
        for pass in 0..2 {
            for (i, frame) in dry.iter().enumerate() {
                let input = (frame[0] + frame[1]) as f64 * 0.5;
                let mut wet = [0.0; 2];
                for (n, comb) in combs.iter_mut().enumerate() {
                    let value = comb.buffer[comb.position];
                    comb.low += (value - comb.low) * 0.22;
                    comb.buffer[comb.position] = input + comb.low * comb.feedback;
                    comb.position = (comb.position + 1) % comb.buffer.len();
                    wet[n % 2] += value;
                    wet[1 - n % 2] += value * 0.18;
                }
                if pass == 1 {
                    self.frames[i] = [
                        frame[0] + wet[0] as f32 * 0.065,
                        frame[1] + wet[1] as f32 * 0.065,
                    ];
                }
            }
        }
    }
}

pub(super) fn generate(track: &Track, rate: u32) -> SampleBuffer {
    let beat_seconds = 60.0 / BPM as f64;
    let mut arrangement = Arrangement {
        frames: vec![[0.0; 2]; (track.seconds * rate as f64).round() as usize],
        rate,
    };
    for pass in 0..2 {
        let offset = pass as f64 * 32.0;
        for (n, &(beat, duration, note)) in MELODY.iter().enumerate() {
            let accent = if beat % 4.0 == 0.0 { 1.04 } else { 0.92 };
            let human = if n == 0 {
                0.0
            } else {
                ((n * 7 % 5) as f64 - 2.0) * 0.003
            };
            arrangement.harp(
                offset + beat + human,
                duration * beat_seconds,
                note,
                0.54 * accent * if pass == 0 { 1.0 } else { 0.94 },
                (pass * 101 + n) as u64,
            );
        }
        // Descending bass/pedal movement of the introduction. Chamber voicings
        // remain behind the harp; the second pass opens their stereo texture.
        for (beat, duration, notes) in [
            (0.0, 2.0, [38, 50, 57, 62]),
            (2.0, 2.0, [36, 48, 55, 60]),
            (4.0, 4.0, [34, 46, 53, 58]),
            (8.0, 4.0, [33, 45, 53, 57]),
            (12.0, 4.0, [31, 43, 50, 58]),
            (16.0, 2.0, [38, 50, 57, 62]),
            (18.0, 2.0, [36, 48, 55, 60]),
            (20.0, 3.0, [34, 46, 53, 58]),
            (23.0, 1.0, [33, 45, 52, 57]),
            (24.0, 2.0, [34, 46, 53, 58]),
            (26.0, 1.0, [31, 43, 50, 55]),
            (27.0, 1.0, [33, 45, 52, 57]),
            (28.0, 4.0, [38, 50, 53, 57]),
        ] {
            arrangement.harp(
                offset + beat,
                duration * beat_seconds,
                notes[1],
                0.16,
                (beat as u64 + pass as u64 * 77) ^ 0xabba,
            );
            arrangement.strings(
                offset + beat,
                duration * beat_seconds,
                &notes,
                if pass == 0 { 0.010 } else { 0.014 },
            );
            if pass == 1 && duration >= 3.0 {
                arrangement.harp(
                    offset + beat + 1.5,
                    beat_seconds * 2.0,
                    notes[2] + 12,
                    0.055,
                    beat as u64 ^ 0xa441,
                );
            }
        }
    }
    arrangement.room();
    let mut mean = [0.0; 2];
    for frame in &arrangement.frames {
        for c in 0..2 {
            mean[c] += frame[c] as f64;
        }
    }
    for value in &mut mean {
        *value /= arrangement.frames.len() as f64;
    }
    let peak = arrangement.frames.iter().fold(0.001_f64, |peak, frame| {
        peak.max((frame[0] as f64 - mean[0]).abs())
            .max((frame[1] as f64 - mean[1]).abs())
    });
    for frame in &mut arrangement.frames {
        for c in 0..2 {
            frame[c] = ((frame[c] as f64 - mean[c]) * 0.78 / peak) as f32;
        }
    }
    let mut sample = SampleBuffer::new(track.name.into(), rate, arrangement.frames)
        .expect("valid periodic Harvest Dawn cover");
    sample.library_id = Some(format!("factory:{}:v1", track.id));
    sample
}
