//! A short, independently synthesized arrangement of Harvest Dawn.
//! Composition: Jeremy Soule, The Elder Scrolls IV: Oblivion (2006).
//! Only the eight-second phrase previously used by the bank is covered.
//! The original soundtrack recording is not used by this renderer.
use super::{Track, hz, sine};
use crate::{Stereo, dsp::SampleBuffer, tables::Random};

struct Arrangement {
    frames: Vec<Stereo>,
    rate: u32,
}

impl Arrangement {
    fn voice(&mut self, start: f64, duration: f64, pan: f64, mut signal: impl FnMut(f64) -> f64) {
        let start = (start * self.rate as f64).round() as usize;
        let length = (duration * self.rate as f64).ceil() as usize;
        let count = length.min(self.frames.len().saturating_sub(start));
        let gains = [(1.0 - pan).sqrt() as f32, pan.sqrt() as f32];
        for i in 0..count {
            let t = i as f64 / self.rate as f64;
            let release = ((duration - t) / 0.02).clamp(0.0, 1.0);
            let value = (signal(t) * release) as f32;
            for (channel, gain) in gains.into_iter().enumerate() {
                self.frames[start + i][channel] += value * gain;
            }
        }
    }

    fn flute(&mut self, start: f64, gate: f64, note: u8, level: f64) {
        let frequency = hz(note);
        let rate = self.rate as f64;
        let mut random = Random::default();
        let (mut phase, mut breath) = (0.0, 0.0);
        self.voice(start, gate + 0.14, 0.56, |t| {
            let vibrato = (t / 0.35).clamp(0.0, 1.0) * sine(t * 5.1) * 0.0018;
            phase += frequency * (1.0 + vibrato) / rate;
            breath += (random.bipolar() as f64 - breath) * 0.12;
            let env = (1.0 - (-t / 0.035).exp())
                * if t > gate {
                    (-(t - gate) / 0.045).exp()
                } else {
                    1.0
                };
            let tone = sine(phase) + sine(phase * 2.0) * 0.12 + sine(phase * 3.0) * 0.035;
            (tone + breath * 0.018) * env * level
        });
    }

    fn piano(&mut self, start: f64, note: u8, level: f64, pan: f64) {
        let frequency = hz(note);
        self.voice(start, 3.2, pan, |t| {
            let attack = 1.0 - (-t / 0.004).exp();
            let phase = t * frequency;
            let tone = sine(phase) * (-t / 0.95).exp()
                + sine(phase * 2.001) * 0.22 * (-t / 0.38).exp()
                + sine(phase * 3.006) * 0.08 * (-t / 0.22).exp()
                + sine(phase * 4.012) * 0.025 * (-t / 0.12).exp();
            tone * attack * level
        });
    }

    fn strings(&mut self, start: f64, gate: f64, note: u8, level: f64) {
        for (detune, pan) in [(0.9988, 0.22), (1.0012, 0.78)] {
            let frequency = hz(note) * detune;
            self.voice(start, gate + 0.65, pan, |t| {
                let env = (1.0 - (-t / 0.3).exp())
                    * if t > gate {
                        (-(t - gate) / 0.2).exp()
                    } else {
                        1.0
                    };
                let phase = t * frequency + sine(t * 4.4) * 0.003;
                (sine(phase) + sine(phase * 2.0) * 0.18 + sine(phase * 3.0) * 0.07) * env * level
            });
        }
    }
}

pub(super) fn generate(track: &Track, rate: u32) -> SampleBuffer {
    let mut arrangement = Arrangement {
        frames: vec![[0.0; 2]; (track.seconds * rate as f64).round() as usize],
        rate,
    };
    // Melodic transcription of the existing short reference, with a new
    // accompaniment and timbres. This is a cover, not an original composition.
    for (start, gate, note, level) in [
        (0.0, 1.69, 74, 0.36),
        (1.90, 2.08, 69, 0.33),
        (4.16, 0.68, 74, 0.34),
        (4.98, 0.51, 76, 0.37),
        (5.64, 1.44, 77, 0.35),
        (7.18, 0.22, 79, 0.31),
        (7.47, 0.53, 81, 0.34),
    ] {
        arrangement.flute(start, gate, note, level);
    }
    for (start, gate, notes) in [
        (0.0, 2.45, [43, 55, 58, 62, 65]),
        (2.50, 2.35, [38, 50, 57, 62, 64]),
        (4.98, 3.02, [41, 53, 58, 62, 65]),
    ] {
        for (n, note) in notes.into_iter().enumerate() {
            arrangement.piano(
                start + n as f64 * 0.027,
                note,
                if n == 0 { 0.14 } else { 0.065 },
                0.25 + n as f64 * 0.1,
            );
            arrangement.strings(start, gate, note, if n == 0 { 0.025 } else { 0.018 });
        }
        for (n, note) in notes[2..].iter().enumerate() {
            arrangement.piano(start + 1.18 + n as f64 * 0.09, *note + 12, 0.028, 0.65);
        }
    }
    // A few early reflections provide space without sampling an external room
    // or changing the phrase length.
    let dry = arrangement.frames.clone();
    for (seconds, gain) in [(0.047, 0.14), (0.089, 0.10), (0.173, 0.065), (0.257, 0.04)] {
        let delay = (seconds * rate as f64).round() as usize;
        for i in delay..arrangement.frames.len() {
            arrangement.frames[i][0] += dry[i - delay][1] * gain;
            arrangement.frames[i][1] += dry[i - delay][0] * gain;
        }
    }
    let last = arrangement.frames.len() - 1;
    for (i, frame) in arrangement.frames.iter_mut().enumerate() {
        let fade = (i as f64 / (rate as f64 * 0.012)).min(1.0)
            * ((last - i) as f64 / (rate as f64 * 0.06)).min(1.0);
        for value in frame {
            *value *= fade as f32;
        }
    }
    let peak = arrangement
        .frames
        .iter()
        .flatten()
        .fold(0.001_f32, |peak, value| peak.max(value.abs()));
    for frame in &mut arrangement.frames {
        for value in frame {
            *value *= 0.78 / peak;
        }
    }
    let mut sample = SampleBuffer::new(track.name.into(), rate, arrangement.frames)
        .expect("valid synthesized Harvest Dawn cover");
    sample.library_id = Some(format!("factory:{}:v1", track.id));
    sample
}
