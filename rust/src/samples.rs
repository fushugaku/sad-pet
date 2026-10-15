//! Original, deterministic source material for the local sample library.
use crate::{Stereo, dsp::SampleBuffer, tables::Random};
use serde::Serialize;
use std::{
    f64::consts::TAU,
    sync::{Arc, OnceLock},
};

#[derive(Clone, Copy, Serialize)]
pub struct Track {
    pub id: &'static str,
    pub name: &'static str,
    pub character: &'static str,
    pub seconds: u32,
    pub bpm: Option<u32>,
}

pub const TRACKS: [Track; 8] = [
    Track {
        id: "glass-sequence",
        name: "Glass sequence",
        character: "Glass plucks",
        seconds: 8,
        bpm: Some(120),
    },
    Track {
        id: "warm-chords",
        name: "Warm chords",
        character: "Slow analog pad",
        seconds: 16,
        bpm: Some(60),
    },
    Track {
        id: "analog-bass",
        name: "Analog bass",
        character: "Pulsing bass line",
        seconds: 8,
        bpm: Some(120),
    },
    Track {
        id: "drum-machine",
        name: "Drum machine",
        character: "Kick, snare & hats",
        seconds: 8,
        bpm: Some(120),
    },
    Track {
        id: "dusty-keys",
        name: "Dusty keys",
        character: "Soft electric keys",
        seconds: 12,
        bpm: Some(80),
    },
    Track {
        id: "vowel-choir",
        name: "Vowel choir",
        character: "Synthetic vocal phrases",
        seconds: 8,
        bpm: Some(120),
    },
    Track {
        id: "metallic-textures",
        name: "Metallic textures",
        character: "Inharmonic struck tones",
        seconds: 12,
        bpm: Some(80),
    },
    Track {
        id: "air-and-rain",
        name: "Air & rain",
        character: "Noise, droplets & air",
        seconds: 12,
        bpm: None,
    },
];

static CACHE: [OnceLock<Arc<SampleBuffer>>; TRACKS.len()] =
    [const { OnceLock::new() }; TRACKS.len()];

pub fn get(id: &str) -> Option<Arc<SampleBuffer>> {
    let index = TRACKS.iter().position(|track| track.id == id)?;
    Some(
        CACHE[index]
            .get_or_init(|| Arc::new(generate(index, 48_000)))
            .clone(),
    )
}

fn hz(note: u8) -> f64 {
    440.0 * 2.0_f64.powf((note as f64 - 69.0) / 12.0)
}
fn sine(cycles: f64) -> f64 {
    (TAU * cycles).sin()
}
fn envelope(age: f64, attack: f64, decay: f64) -> f64 {
    (1.0 - (-age / attack).exp()) * (-age / decay).exp()
}

/// A variable rate is useful to verify the bank without allocating all 48 kHz files.
pub fn generate(index: usize, sample_rate: u32) -> SampleBuffer {
    let track = &TRACKS[index];
    if index == 0 {
        let mut sample = SampleBuffer::demo(sample_rate);
        sample.library_id = Some(format!("factory:{}:v1", track.id));
        return sample;
    }
    let mut random = Random::default();
    let mut low_noise = 0.0;
    let length = track.seconds as usize * sample_rate as usize;
    let mut frames: Vec<Stereo> = (0..length)
        .map(|i| {
            let t = i as f64 / sample_rate as f64;
            let noise = random.bipolar() as f64;
            low_noise += (noise - low_noise) * (1.0 - (-TAU * 850.0 / sample_rate as f64).exp());
            let high_noise = noise - low_noise;
            let mut frame = [0.0; 2];
            match index {
                1 => {
                    let chords = [
                        [48, 55, 60, 63],
                        [44, 51, 58, 60],
                        [51, 58, 63, 67],
                        [46, 53, 60, 65],
                    ];
                    let step = (t / 4.0) as usize;
                    let age = t % 4.0;
                    let env = (1.0 - (-age * 2.5).exp()) * ((4.0 - age) / 0.8).min(1.0);
                    for (n, note) in chords[step % 4].iter().enumerate() {
                        let frequency = hz(*note);
                        for (channel, value) in frame.iter_mut().enumerate() {
                            let detune = if channel == 0 { 0.9984 } else { 1.0016 };
                            let p = t * frequency * detune;
                            *value += (sine(p)
                                + 0.22 * sine(p * 2.0)
                                + 0.08 * sine(p * 3.0 + n as f64 * 0.2))
                                * env
                                * 0.13;
                        }
                    }
                }
                2 => {
                    let notes = [
                        36, 36, 43, 36, 39, 36, 46, 43, 32, 32, 39, 34, 36, 43, 39, 34,
                    ];
                    let step = (t * 2.0) as usize;
                    let age = t % 0.5;
                    let frequency = hz(notes[step % 16]);
                    let env = envelope(age, 0.004, 0.20) * ((0.44 - age) / 0.08).clamp(0.0, 1.0);
                    let mut tone = sine(age * frequency * 0.5) * 0.35;
                    for harmonic in 1..=8 {
                        let h = harmonic as f64;
                        tone += sine(age * frequency * h) / h * (-age * (h - 1.0) * 8.0).exp();
                    }
                    frame = [tone * env * 0.5, tone * env * 0.5];
                }
                3 => {
                    let beat = (t * 2.0) as usize;
                    let age = t % 0.5;
                    let eighth = (t * 4.0) as usize;
                    let hat_age = t % 0.25;
                    let kick = if beat.is_multiple_of(2) || beat % 16 == 15 {
                        sine(52.0 * age + 115.0 / 28.0 * (1.0 - (-28.0 * age).exp()))
                            * envelope(age, 0.001, 0.13)
                    } else {
                        0.0
                    };
                    let snare = if beat % 2 == 1 {
                        (high_noise * 0.7 + sine(age * 185.0) * 0.3) * envelope(age, 0.001, 0.075)
                    } else {
                        0.0
                    };
                    let hat = high_noise
                        * envelope(hat_age, 0.0005, if eighth % 8 == 7 { 0.12 } else { 0.024 })
                        * 0.20;
                    frame = [
                        kick * 0.72 + snare * 0.45 + hat * 0.7,
                        kick * 0.72 + snare * 0.45 + hat,
                    ];
                }
                4 => {
                    let notes = [
                        60, 67, 70, 63, 65, 72, 68, 63, 58, 65, 68, 62, 60, 67, 63, 58,
                    ];
                    let step = (t / 0.75) as usize;
                    for offset in 0..=3.min(step) {
                        let event = step - offset;
                        let age = t - event as f64 * 0.75;
                        let frequency = hz(notes[event % 16]);
                        let env = envelope(age, 0.003, 0.7);
                        let p = age * frequency + sine(t * 0.7) * 0.003;
                        let tone = sine(p)
                            + 0.32 * sine(p * 2.002) * (-age * 3.0).exp()
                            + 0.14 * sine(p * 3.997) * (-age * 6.0).exp();
                        let pan = if event.is_multiple_of(2) { 0.3 } else { 0.7 };
                        frame[0] += tone * env * (1.0 - pan) * 0.7;
                        frame[1] += tone * env * pan * 0.7;
                    }
                    frame[0] += low_noise * 0.005;
                    frame[1] -= low_noise * 0.005;
                }
                5 => {
                    let step = (t * 2.0) as usize;
                    let age = t % 0.5;
                    let notes = [48, 55, 51, 58, 48, 51, 55, 63];
                    let frequency = hz(notes[step % 8]);
                    let vowels = [
                        [650.0, 1100.0, 2500.0],
                        [400.0, 1800.0, 2600.0],
                        [320.0, 800.0, 2200.0],
                        [800.0, 1400.0, 2800.0],
                    ];
                    let formants = vowels[(step / 4) % 4];
                    let env = envelope(age, 0.025, 0.55) * ((0.5 - age) / 0.09).min(1.0);
                    for harmonic in 1..=20 {
                        let h = harmonic as f64;
                        let f = frequency * h;
                        if f >= sample_rate as f64 * 0.45 {
                            break;
                        }
                        let weight: f64 = formants
                            .iter()
                            .map(|center| (-((f - center) / 180.0).powi(2)).exp())
                            .sum();
                        for (channel, value) in frame.iter_mut().enumerate() {
                            let detune = if channel == 0 { 0.999 } else { 1.001 };
                            *value += sine(t * f * detune + sine(t * 5.0) * 0.02) * (0.08 + weight)
                                / h.sqrt()
                                * env
                                * 0.15;
                        }
                    }
                    for value in &mut frame {
                        *value += high_noise * env * 0.012;
                    }
                }
                6 => {
                    let notes = [48, 55, 58, 51, 60, 53, 58, 55];
                    let step = (t / 1.5) as usize;
                    for offset in 0..=3.min(step) {
                        let event = step - offset;
                        let age = t - event as f64 * 1.5;
                        let frequency = hz(notes[event % 8]);
                        for (h, ratio) in [1.0, 1.471, 2.091, 2.563, 3.821, 5.419, 7.137]
                            .iter()
                            .enumerate()
                        {
                            if frequency * ratio >= sample_rate as f64 * 0.45 {
                                continue;
                            }
                            let tone = sine(age * frequency * ratio)
                                * envelope(age, 0.001, 2.2 / (1.0 + h as f64 * 0.3))
                                / (1.0 + h as f64 * 0.5);
                            let pan = 0.2 + (h % 3) as f64 * 0.3;
                            frame[0] += tone * (1.0 - pan) * 0.25;
                            frame[1] += tone * pan * 0.25;
                        }
                    }
                }
                7 => {
                    let swell = 0.45 + 0.2 * sine(t / 9.0);
                    let age = t % 0.375;
                    let drop =
                        sine(age * (1100.0 - 240.0 * age)) * envelope(age, 0.0008, 0.025) * 0.16;
                    frame = [
                        low_noise * swell + high_noise * 0.025 + drop,
                        low_noise * swell * 0.7 - high_noise * 0.04
                            + drop * 0.4
                            + sine(t * 73.416) * 0.025,
                    ];
                }
                _ => unreachable!(),
            }
            let fade = (t / 0.025).min(1.0)
                * ((track.seconds as f64 - t - 1.0 / sample_rate as f64) / 0.04).clamp(0.0, 1.0);
            frame.map(|value| (value * fade) as f32)
        })
        .collect();
    let peak = frames
        .iter()
        .flatten()
        .fold(0.0_f32, |peak, value| peak.max(value.abs()));
    let gain = 0.78 / peak.max(0.001);
    for frame in &mut frames {
        for value in frame {
            *value *= gain;
        }
    }
    let mut sample =
        SampleBuffer::new(track.name.into(), sample_rate, frames).expect("valid built-in track");
    sample.library_id = Some(format!("factory:{}:v1", track.id));
    sample
}
