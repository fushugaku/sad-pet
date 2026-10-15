//! Original factory audio. Events and their tails wrap around the buffer so
//! the rhythmic loops keep their full timing and ambience across the seam.
use super::{Track, hz, sine};
use crate::{Stereo, dsp::SampleBuffer, tables::Random};
use std::f64::consts::TAU;

struct Bank {
    frames: Vec<Stereo>,
    rate: u32,
    beat: f64,
    random: Random,
}

#[derive(Clone, Copy)]
enum Tone {
    Keys,
    Marimba,
    Kalimba,
    Nylon,
    Box,
    Piano,
    Bell,
    Bowl,
}

impl Bank {
    fn new(track: &Track, rate: u32) -> Self {
        let seed = track.id.bytes().fold(0x853c_49e6_748f_ea9b_u64, |a, b| {
            a.rotate_left(5) ^ u64::from(b)
        });
        Self {
            frames: vec![[0.0; 2]; (track.seconds * rate as f64).round() as usize],
            rate,
            beat: 60.0 / track.bpm.unwrap_or(80) as f64,
            random: Random(seed),
        }
    }

    fn event(
        &mut self,
        start: f64,
        duration: f64,
        pan: f64,
        mut signal: impl FnMut(f64, &mut Random) -> f64,
    ) {
        let length = self.frames.len();
        let start = (start * self.rate as f64).round() as usize % length;
        let count = (duration * self.rate as f64).ceil() as usize;
        let gains = [(1.0 - pan).sqrt() as f32, pan.sqrt() as f32];
        for i in 0..count {
            let age = i as f64 / self.rate as f64;
            // End even a very quiet tail smoothly before wrapping the event.
            let release = ((duration - age) / 0.008).clamp(0.0, 1.0);
            let value = (signal(age, &mut self.random) * release) as f32;
            let frame = &mut self.frames[(start + i) % length];
            for channel in 0..2 {
                frame[channel] += value * gains[channel];
            }
        }
    }

    fn kick(&mut self, beat: f64, level: f64, deep: bool) {
        let base = if deep { 43.0 } else { 55.0 };
        self.event(beat * self.beat, 0.6, 0.5, |t, _| {
            let phase = base * t + 140.0 / 35.0 * (1.0 - (-35.0 * t).exp());
            let body = sine(phase) * (-t / 0.14).exp();
            let click = sine(t * 1800.0) * (-t * 700.0).exp() * 0.16;
            (body + click) * (1.0 - (-t * 1800.0).exp()) * level
        });
    }

    fn snare(&mut self, beat: f64, level: f64, clap: bool) {
        let mut low = 0.0;
        let coefficient = 1.0 - (-TAU * 1800.0 / self.rate as f64).exp();
        self.event(beat * self.beat, 0.38, 0.5, |t, random| {
            let noise = random.bipolar() as f64;
            low += (noise - low) * coefficient;
            let high = noise - low;
            let burst = if clap && t < 0.035 {
                (-(t % 0.011) * 180.0).exp()
            } else {
                (-t * 20.0).exp()
            };
            (high * burst * 0.8 + sine(t * 178.0) * (-t * 35.0).exp() * 0.25)
                * (1.0 - (-t * 2500.0).exp())
                * level
        });
    }

    fn hat(&mut self, beat: f64, level: f64, open: bool, pan: f64) {
        let decay = if open { 0.105 } else { 0.023 };
        let nyquist = self.rate as f64 * 0.45;
        let mut previous = 0.0;
        self.event(beat * self.beat, decay * 7.0, pan, |t, random| {
            let noise = random.bipolar() as f64;
            let high = noise - previous;
            previous = noise;
            let metal: f64 = [3211.0, 4567.0, 6317.0]
                .iter()
                .filter(|f| **f < nyquist)
                .map(|f| sine(t * f))
                .sum();
            (high * 0.65 + metal * 0.12) * (-t / decay).exp() * (1.0 - (-t * 4000.0).exp()) * level
        });
    }

    fn note(&mut self, beat: f64, note: u8, tone: Tone, decay: f64, pan: f64, level: f64) {
        let frequency = hz(note);
        let partials: &[(f64, f64, f64)] = match tone {
            Tone::Keys => &[(1.0, 1.0, 1.0), (2.002, 0.28, 2.8), (4.006, 0.12, 5.0)],
            Tone::Marimba => &[(1.0, 1.0, 1.0), (3.99, 0.35, 4.0), (9.98, 0.10, 9.0)],
            Tone::Kalimba => &[(1.0, 1.0, 1.0), (2.76, 0.36, 2.0), (5.41, 0.16, 3.5)],
            Tone::Nylon => &[
                (1.0, 0.8, 1.0),
                (2.0, 0.55, 1.5),
                (3.0, 0.30, 2.2),
                (4.0, 0.13, 3.0),
                (5.0, 0.07, 5.0),
                (7.0, 0.03, 8.0),
            ],
            Tone::Box => &[(1.0, 1.0, 1.0), (2.756, 0.5, 2.5), (5.404, 0.21, 4.0)],
            Tone::Piano => &[
                (1.0, 1.0, 1.0),
                (2.001, 0.36, 1.7),
                (3.006, 0.16, 2.7),
                (4.012, 0.08, 4.0),
                (6.03, 0.025, 6.0),
            ],
            Tone::Bell => &[
                (1.0, 1.0, 1.0),
                (2.71, 0.48, 1.6),
                (4.08, 0.27, 2.4),
                (6.8, 0.12, 3.0),
            ],
            Tone::Bowl => &[
                (1.0, 1.0, 1.0),
                (2.01, 0.45, 0.8),
                (2.91, 0.24, 1.2),
                (4.17, 0.13, 1.8),
            ],
        };
        let partials: Vec<_> = partials
            .iter()
            .copied()
            .filter(|(ratio, _, _)| frequency * ratio < self.rate as f64 * 0.44)
            .collect();
        let attack = if matches!(tone, Tone::Bowl) {
            0.015
        } else {
            0.002
        };
        self.event(beat * self.beat, (decay * 8.0).min(18.0), pan, |t, _| {
            let flutter = if matches!(tone, Tone::Piano | Tone::Keys) {
                sine(t * 0.7) * 0.004 + sine(t * 6.2) * 0.0007
            } else {
                0.0
            };
            let phase = t * frequency + flutter;
            let value = if matches!(tone, Tone::Marimba) {
                let index = 2.8 * (-t / (decay * 0.12)).exp();
                sine(phase + sine(phase * 3.99) * index) * (-t / decay).exp()
            } else {
                let mut sum = 0.0;
                for &(ratio, amplitude, damping) in &partials {
                    let beating = if matches!(tone, Tone::Bowl) {
                        0.78 + 0.22 * sine(t * 0.43 * ratio)
                    } else {
                        1.0
                    };
                    sum += sine(phase * ratio) * amplitude * beating * (-t * damping / decay).exp();
                }
                sum
            };
            value * (1.0 - (-t / attack).exp()) * level
        });
    }

    fn bass(&mut self, beat: f64, note: u8, length: f64, accent: bool, previous: u8) {
        let frequency = hz(note);
        let glide = hz(previous) - frequency;
        let rate = self.rate as f64;
        let mut phase = 0.0;
        let (mut low, mut band) = (0.0, 0.0);
        self.event(beat * self.beat, length, 0.5, |t, _| {
            let freq = frequency + glide * (-t * 55.0).exp();
            let dt = freq / rate;
            phase = (phase + dt).fract();
            let mut saw = phase * 2.0 - 1.0;
            // PolyBLEP keeps the swept saw useful as a source for pitching/grains.
            if phase < dt {
                let x = phase / dt;
                saw -= x + x - x * x - 1.0;
            } else if phase > 1.0 - dt {
                let x = (phase - 1.0) / dt;
                saw -= x * x + x + x + 1.0;
            }
            let cutoff =
                (180.0 + if accent { 2600.0 } else { 1100.0 } * (-t * 11.0).exp()).min(rate * 0.12);
            let f = 2.0 * (std::f64::consts::PI * cutoff / rate).sin();
            // Two substeps keep the resonant state-variable filter stable.
            for _ in 0..2 {
                low += f * 0.5 * band;
                let high = (saw * 0.5 - low - 0.24 * band).tanh();
                band += f * 0.5 * high;
            }
            let env = (1.0 - (-t * 700.0).exp())
                * (-t * 4.0).exp()
                * ((length - t) / 0.018).clamp(0.0, 1.0);
            (low * 2.0).tanh() * env * if accent { 0.8 } else { 0.6 }
        });
    }

    fn pad(&mut self, beat: f64, notes: &[u8], seconds: f64, bright: bool) {
        for &note in notes {
            for (detune, pan) in [(0.998, 0.2), (1.002, 0.8)] {
                let freq = hz(note) * detune;
                let nyquist = self.rate as f64 * 0.44;
                self.event(beat * self.beat, seconds, pan, |t, _| {
                    let env = (t / 1.2).min(1.0) * ((seconds - t) / 2.0).clamp(0.0, 1.0);
                    let mut value = 0.0;
                    for harmonic in 1..=if bright { 7 } else { 5 } {
                        let h = harmonic as f64;
                        if freq * h < nyquist {
                            value += sine(t * freq * h + sine(t * 0.18) * 0.025)
                                / h.powf(if bright { 1.8 } else { 1.3 });
                        }
                    }
                    value * env * (0.8 + 0.2 * sine(t * 0.31)) * 0.13
                });
            }
        }
    }

    fn echo(&mut self, seconds: f64, gain: f32, repeats: usize) {
        let source = self.frames.clone();
        let length = source.len();
        for repeat in 1..=repeats {
            let offset = (seconds * repeat as f64 * self.rate as f64).round() as usize;
            let gain = gain.powi(repeat as i32);
            for (i, frame) in source.iter().enumerate() {
                let dest = &mut self.frames[(i + offset) % length];
                for channel in 0..2 {
                    dest[channel] += frame[if repeat % 2 == 1 {
                        1 - channel
                    } else {
                        channel
                    }] * gain;
                }
            }
        }
    }

    fn noise_bed(&mut self, ocean: bool) {
        let length = self.frames.len();
        let noise: Vec<f64> = (0..length).map(|_| self.random.bipolar() as f64).collect();
        let mut filtered = vec![0.0; length];
        let (mut low, mut deep) = (0.0, 0.0);
        let coefficient =
            1.0 - (-TAU * if ocean { 1700.0 } else { 450.0 } / self.rate as f64).exp();
        let deep_coefficient = 1.0 - (-TAU * 85.0 / self.rate as f64).exp();
        // Warm the filters with the same circular noise before retaining output.
        for pass in 0..2 {
            for (i, &value) in noise.iter().enumerate() {
                low += (value - low) * coefficient;
                deep += (value - deep) * deep_coefficient;
                if pass == 1 {
                    filtered[i] = low - deep * 0.7;
                }
            }
        }
        let seconds = length as f64 / self.rate as f64;
        for (i, frame) in self.frames.iter_mut().enumerate() {
            let t = i as f64 / self.rate as f64;
            let swell = if ocean {
                (0.5 + 0.5 * sine(3.0 * t / seconds)).powi(2) * 0.8 + 0.18
            } else {
                0.06
            };
            frame[0] += (filtered[i] * swell) as f32;
            frame[1] += (filtered[(i + length / 3) % length] * swell) as f32;
        }
    }

    fn finish(mut self, track: &Track) -> SampleBuffer {
        // Remove DC without altering a seamless loop's boundary or duration.
        let mut mean = [0.0; 2];
        for frame in &self.frames {
            for channel in 0..2 {
                mean[channel] += frame[channel] as f64;
            }
        }
        for value in &mut mean {
            *value /= self.frames.len() as f64;
        }
        let mut peak = 0.001_f64;
        for frame in &self.frames {
            for channel in 0..2 {
                peak = peak.max((frame[channel] as f64 - mean[channel]).abs());
            }
        }
        let gain = 0.78 / peak;
        for frame in &mut self.frames {
            for channel in 0..2 {
                frame[channel] = ((frame[channel] as f64 - mean[channel]) * gain) as f32;
            }
        }
        let mut sample = SampleBuffer::new(track.name.into(), self.rate, self.frames)
            .expect("valid original factory audio");
        sample.library_id = Some(format!("factory:{}:v1", track.id));
        sample
    }
}

pub(super) fn generate(track: &Track, rate: u32) -> SampleBuffer {
    let mut bank = Bank::new(track, rate);
    match track.id {
        "lofi-beat" => {
            for bar in 0..4 {
                let b = bar as f64 * 4.0;
                for offset in [0.0, 1.75, 2.5] {
                    bank.kick(b + offset, 0.85, true);
                }
                for offset in [1.0, 3.0] {
                    bank.snare(b + offset + 0.025, 0.38, false);
                }
                for step in 0..8 {
                    bank.hat(
                        b + step as f64 * 0.5 + if step % 2 == 1 { 0.09 } else { 0.0 },
                        if step % 2 == 0 { 0.16 } else { 0.09 },
                        false,
                        0.62,
                    );
                }
                for note in [48_i16, 55, 58, 62].map(|n| (n + [0, 5, 3, -2][bar]) as u8) {
                    bank.note(b + 0.08, note, Tone::Keys, 1.0, 0.4, 0.12);
                }
            }
        }
        "breakbeat" => {
            for bar in 0..4 {
                let b = bar as f64 * 4.0;
                for offset in [0.0, 1.5, 2.75, 3.5] {
                    bank.kick(b + offset, 0.85, false);
                }
                for offset in [1.0, 3.0] {
                    bank.snare(b + offset, 0.7, false);
                }
                for offset in [0.75, 2.5, 3.75] {
                    bank.snare(b + offset, 0.14, false);
                }
                for step in 0..16 {
                    bank.hat(
                        b + step as f64 * 0.25,
                        if step % 2 == 0 { 0.16 } else { 0.07 },
                        step == 14,
                        if step % 2 == 0 { 0.3 } else { 0.7 },
                    );
                }
            }
        }
        "four-on-floor" => {
            for beat in 0..16 {
                bank.kick(beat as f64, 1.0, true);
                bank.hat(beat as f64 + 0.5, 0.20, true, 0.65);
                bank.hat(beat as f64 + 0.75, 0.07, false, 0.3);
                if beat % 2 == 1 {
                    bank.snare(beat as f64, 0.5, true);
                }
            }
        }
        "electro-percussion" => {
            for step in 0..32 {
                let b = step as f64 * 0.5;
                bank.hat(
                    b,
                    if step % 3 == 0 { 0.22 } else { 0.08 },
                    false,
                    if step % 2 == 0 { 0.2 } else { 0.8 },
                );
                if step % 3 == 0 {
                    let freq = [130.0, 185.0, 240.0][step / 3 % 3];
                    bank.event(b * bank.beat, 0.5, 0.25, |t, _| {
                        sine(freq * t + 1.8 * (1.0 - (-t * 25.0).exp()))
                            * (-t * 13.0).exp()
                            * (1.0 - (-t * 1000.0).exp())
                            * 0.5
                    });
                }
                if step % 4 == 2 {
                    bank.event(b * bank.beat, 0.15, 0.75, |t, _| {
                        (sine(t * 540.0) + sine(t * 815.0) * 0.6)
                            * (-t * 55.0).exp()
                            * (1.0 - (-t * 3000.0).exp())
                            * 0.3
                    });
                }
            }
            bank.echo(bank.beat * 0.75, 0.20, 2);
        }
        "dub-chords" => {
            for bar in 0..4 {
                for offset in [0.5, 1.5, 2.75, 3.5] {
                    for note in [48_u8, 55, 58, 62].map(|n| n + [0, 3, 5, 2][bar]) {
                        bank.note(
                            bar as f64 * 4.0 + offset,
                            note,
                            Tone::Keys,
                            0.14,
                            0.35,
                            0.32,
                        );
                    }
                }
            }
            bank.echo(bank.beat * 0.75, 0.52, 5);
        }
        "acid-line" => {
            let notes = [
                36, 36, 48, 39, 43, 36, 46, 43, 34, 46, 41, 39, 36, 43, 39, 34,
            ];
            for step in 0..32 {
                bank.bass(
                    step as f64 * 0.5,
                    notes[step % 16],
                    bank.beat * 0.46,
                    step % 5 == 0 || step % 8 == 6,
                    notes[(step + 15) % 16],
                );
            }
        }
        "fm-marimba" => {
            let notes = [
                60, 67, 70, 74, 63, 72, 67, 77, 58, 65, 70, 72, 62, 69, 65, 74,
            ];
            for step in 0..32 {
                bank.note(
                    step as f64 * 0.5,
                    notes[step % 16],
                    Tone::Marimba,
                    0.35,
                    if step % 2 == 0 { 0.25 } else { 0.75 },
                    if step % 4 == 0 { 0.5 } else { 0.33 },
                );
            }
            bank.echo(bank.beat * 0.75, 0.22, 3);
        }
        "kalimba-garden" => {
            let notes = [60, 67, 72, 64, 69, 76, 67, 71];
            for step in 0..24 {
                bank.note(
                    step as f64 * 2.0 / 3.0,
                    notes[step % 8],
                    Tone::Kalimba,
                    0.65,
                    if step % 2 == 0 { 0.2 } else { 0.8 },
                    0.45,
                );
            }
            for step in 0..8 {
                bank.note(
                    step as f64 * 2.0 + 0.5,
                    48 + [0, 7, 5, 9][step % 4],
                    Tone::Kalimba,
                    0.85,
                    0.5,
                    0.22,
                );
            }
            bank.echo(bank.beat * 0.5, 0.16, 2);
        }
        "nylon-harmonics" => {
            let notes = [
                48, 55, 60, 64, 67, 64, 60, 55, 45, 52, 57, 60, 64, 60, 57, 52,
            ];
            for step in 0..32 {
                bank.note(
                    step as f64 * 0.5,
                    notes[step % 16],
                    Tone::Nylon,
                    0.65,
                    0.35 + (step % 3) as f64 * 0.15,
                    0.4,
                );
                if step % 4 == 2 {
                    bank.note(
                        step as f64 * 0.5,
                        notes[step % 16] + 12,
                        Tone::Nylon,
                        0.9,
                        0.7,
                        0.12,
                    );
                }
            }
            bank.echo(bank.beat * 0.33, 0.12, 2);
        }
        "music-box" => {
            let notes = [
                84, 79, 87, 82, 91, 84, 79, 86, 82, 89, 77, 84, 87, 79, 82, 86,
            ];
            for (step, note) in notes.into_iter().enumerate() {
                bank.note(
                    step as f64,
                    note,
                    Tone::Box,
                    0.65,
                    if step % 2 == 0 { 0.3 } else { 0.7 },
                    0.45,
                );
            }
            bank.echo(bank.beat * 0.5, 0.18, 3);
        }
        "tape-piano" => {
            let chords = [
                [48, 55, 62, 63],
                [44, 51, 58, 60],
                [46, 53, 60, 65],
                [43, 50, 57, 58],
            ];
            for (bar, notes) in chords.into_iter().enumerate() {
                for (n, note) in notes.into_iter().enumerate() {
                    bank.note(
                        bar as f64 * 4.0 + n as f64 * 0.025,
                        note,
                        Tone::Piano,
                        1.2,
                        0.25 + n as f64 * 0.16,
                        0.3,
                    );
                    bank.note(
                        bar as f64 * 4.0 + 2.5,
                        note + 12,
                        Tone::Piano,
                        0.6,
                        0.7,
                        0.07,
                    );
                }
            }
            bank.echo(bank.beat * 1.5, 0.14, 3);
        }
        "string-orbit" | "shimmer-pad" => {
            let bright = track.id == "shimmer-pad";
            let chords = [
                [48, 55, 62, 65],
                [45, 52, 59, 64],
                [41, 48, 55, 60],
                [43, 50, 57, 62],
            ];
            for (bar, notes) in chords.into_iter().enumerate() {
                bank.pad(bar as f64 * 4.0, &notes, bank.beat * 6.0, bright);
                if bright {
                    for note in notes {
                        bank.note(
                            bar as f64 * 4.0 + 1.5,
                            note + 24,
                            Tone::Bell,
                            1.0,
                            0.7,
                            0.035,
                        );
                    }
                }
            }
            bank.echo(bank.beat * 0.75, if bright { 0.33 } else { 0.14 }, 4);
        }
        "dark-drone" => {
            bank.noise_bed(false);
            let seconds = track.seconds;
            for (channel, frame_channel) in [0, 1].into_iter().enumerate() {
                for (i, frame) in bank.frames.iter_mut().enumerate() {
                    let phase = i as f64 / rate as f64 / seconds;
                    let base = sine(phase * 587.0) * 0.32 + sine(phase * 294.0) * 0.24;
                    let high =
                        sine(phase * (1175 + channel) as f64 + sine(phase * 2.0) * 0.15) * 0.1;
                    frame[frame_channel] += ((base + high) * (0.7 + sine(phase) * 0.25)) as f32;
                }
            }
        }
        "singing-bowl" => {
            bank.note(0.0, 57, Tone::Bowl, 3.4, 0.35, 0.6);
            bank.note(8.0, 64, Tone::Bowl, 2.6, 0.7, 0.18);
            bank.echo(0.127, 0.25, 4);
        }
        "resonant-bells" => {
            for (step, note) in [60, 79, 67, 74, 63, 82, 70, 77].into_iter().enumerate() {
                bank.note(
                    step as f64 * 2.0,
                    note,
                    Tone::Bell,
                    1.4,
                    if step % 2 == 0 { 0.2 } else { 0.8 },
                    0.4,
                );
                bank.note(
                    step as f64 * 2.0 + 0.75,
                    note + 12,
                    Tone::Bell,
                    0.6,
                    0.5,
                    0.08,
                );
            }
            bank.echo(bank.beat * 0.75, 0.24, 4);
        }
        "circuit-glitches" => {
            for step in 0..64 {
                if step % 7 == 4 || step % 11 == 3 {
                    continue;
                }
                let freq = hz([48, 60, 67, 75, 79, 84, 55][step % 7]);
                let dur = if step % 5 == 0 { 0.11 } else { 0.035 };
                bank.event(
                    step as f64 * bank.beat * 0.25,
                    dur,
                    if step % 2 == 0 { 0.2 } else { 0.8 },
                    |t, random| {
                        let age = (t * 8000.0).floor() / 8000.0;
                        let tone = sine(age * freq + sine(age * freq * 1.5) * 0.7);
                        let value = if step % 3 == 0 {
                            tone * 0.6 + random.bipolar() as f64 * 0.15
                        } else {
                            tone
                        };
                        (value * 12.0).round() / 12.0
                            * (1.0 - (-t * 3000.0).exp())
                            * (-t / (dur * 0.3)).exp()
                            * 0.45
                    },
                );
            }
            bank.echo(bank.beat * 0.75, 0.27, 3);
        }
        "space-transmission" => {
            bank.noise_bed(false);
            for event in 0..18 {
                let freq = [320.0, 470.0, 830.0, 1270.0, 1930.0][event % 5];
                let duration = [0.18, 0.3, 0.65][event % 3];
                bank.event(
                    event as f64 * 2.0 / 3.0,
                    duration,
                    0.2 + (event % 4) as f64 * 0.2,
                    |t, _| {
                        let window =
                            (t / 0.008).min(1.0) * ((duration - t) / 0.035).clamp(0.0, 1.0);
                        sine(t * freq + t * t * 950.0 + sine(t * 31.0) * 0.2)
                            * (0.55 + 0.45 * sine(t * 17.0))
                            * window
                            * 0.3
                    },
                );
            }
            bank.echo(0.375, 0.38, 4);
        }
        "ocean-swell" => bank.noise_bed(true),
        "vinyl-dust" => {
            bank.noise_bed(false);
            for event in 0..140 {
                let start = bank.random.unit() as f64 * track.seconds;
                let pan = 0.1 + bank.random.unit() as f64 * 0.8;
                let duration = 0.002 + bank.random.unit() as f64 * 0.007;
                let level = if event % 17 == 0 { 0.28 } else { 0.025 };
                bank.event(start, duration, pan, |t, random| {
                    random.bipolar() as f64 * (t / 0.0003).min(1.0) * (-t * 900.0).exp() * level
                });
            }
            for (i, frame) in bank.frames.iter_mut().enumerate() {
                let t = i as f64 / rate as f64;
                let hum = sine(t * 50.0) * 0.002 + sine(t * 100.0) * 0.0006;
                frame[0] += hum as f32;
                frame[1] += hum as f32;
            }
        }
        _ => unreachable!("unknown original factory track"),
    }
    bank.finish(track)
}
