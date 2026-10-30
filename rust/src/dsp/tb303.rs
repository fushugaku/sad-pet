//! Circuit-derived mono bass voice. The diode ladder is a coupled nonlinear
//! nodal system, integrated implicitly; it is not four buffered Moog poles.
//! See docs/TB303.md for equations, component values, sources and approximations.
use super::{SampleBuffer, resynth::Decimator, synth};
use crate::{
    Stereo,
    midi::Message,
    tb303::{Controls, Settings, View},
};
use std::f64::consts::{LN_10, TAU};

const CAP_RATIO: f64 = 33.0 / 18.0; // C19/C18; C19, C24 and C26 are 33 nF.
const SLIDE_TAU: f64 = 100_000.0 * 0.22e-6; // R91, C35.

/// Bilinear (s + zero)/(s + pole), with an affine instantaneous output.
/// The latter lets the entire resonance loop participate in the implicit solve.
#[derive(Clone, Copy, Debug)]
struct Coupling {
    g: f64,
    scale: f64,
    state: f64,
}
impl Coupling {
    fn new(zero: f64, pole: f64, rate: f64) -> Self {
        Self {
            g: pole / (2.0 * rate),
            scale: zero / pole - 1.0,
            state: 0.0,
        }
    }
    fn affine(&self) -> (f64, f64) {
        (
            1.0 + self.scale * self.g / (1.0 + self.g),
            self.scale * self.state / (1.0 + self.g),
        )
    }
    fn process(&mut self, input: f64) -> f64 {
        let low = (self.g * input + self.state) / (1.0 + self.g);
        self.state = 2.0 * low - self.state;
        input + self.scale * low
    }
}

#[derive(Debug)]
struct Ladder {
    nodes: [f64; 4],
    previous: [f64; 4],
    forward: [Coupling; 6],
    feedback: [Coupling; 6],
    rate: f64,
}
impl Ladder {
    fn new(rate: f64) -> Self {
        // The coupling network's poles and zeros (rad/s), from Stinchcombe's
        // complete linear analysis. The core itself below remains nonlinear.
        let poles = [97.5, 38.5, 4.45, 578.1, 20.0, 7.41];
        let forward = [0.0, 0.0, 0.0, 109.9, 34.0, 7.41];
        let feedback = [0.0, 0.0, 0.0, 0.0, 46.5, 4.40];
        Self {
            nodes: [0.0; 4],
            previous: [0.0; 4],
            rate,
            forward: std::array::from_fn(|i| Coupling::new(forward[i], poles[i], rate)),
            feedback: std::array::from_fn(|i| Coupling::new(feedback[i], poles[i], rate)),
        }
    }
    fn process(&mut self, input: f64, cutoff: f64, resonance: f64) -> f64 {
        let input = self
            .forward
            .iter_mut()
            .fold(input * 1.06, |x, c| c.process(x));
        let (a, b) = self.feedback.iter().fold((1.0, 0.0), |(a, b), c| {
            let (next, offset) = c.affine();
            (next * a, next * b + offset)
        });
        let gain = 18.7 * resonance;
        // The geometric pole frequency is g * (C/C18)^(1/4).
        let g = TAU * cutoff / CAP_RATIO.powf(0.25);
        let h = g / (2.0 * self.rate);
        let z = std::array::from_fn::<_, 4, _>(|i| {
            self.nodes[i] + self.previous[i] / (2.0 * self.rate)
        });
        let mut x = self.nodes;
        for _ in 0..6 {
            let (flow, derivatives) = currents(x, input - gain * (a * x[3] + b));
            let f = flow;
            let [d0, d1, d2, d3, di] = derivatives;
            let jacobian = [
                [
                    -CAP_RATIO * d0,
                    CAP_RATIO * d0,
                    0.0,
                    -CAP_RATIO * di * gain * a,
                ],
                [d0, -d0 - d1, d1, 0.0],
                [0.0, d1, -d1 - d2, d2],
                [0.0, 0.0, d2, -d2 - d3],
            ];
            let residual = std::array::from_fn::<_, 4, _>(|i| x[i] - z[i] - h * f[i]);
            if residual.iter().all(|v| v.abs() < 1e-10) {
                break;
            }
            let mut matrix = [[0.0; 5]; 4];
            for i in 0..4 {
                for j in 0..4 {
                    matrix[i][j] = f64::from(i == j) - h * jacobian[i][j];
                }
                matrix[i][4] = -residual[i];
            }
            let delta = solve(matrix);
            for i in 0..4 {
                x[i] += delta[i].clamp(-1.0, 1.0);
            }
        }
        // Re-evaluate after the final Newton update, so the trapezoidal history
        // always describes the state actually committed.
        let f = currents(x, input - gain * (a * x[3] + b)).0;
        self.nodes = x;
        self.previous = f.map(|v| v * g);
        self.feedback.iter_mut().fold(x[3], |x, c| c.process(x));
        x[3]
    }
}

/// KCL on the four capacitor voltages in units of 2*thermal voltage. Currents
/// are normalized differential-pair currents; each diode connects two nodes.
fn currents(x: [f64; 4], input: f64) -> ([f64; 4], [f64; 5]) {
    let t = [
        (x[0] - x[1]).tanh(),
        (x[1] - x[2]).tanh(),
        (x[2] - x[3]).tanh(),
        x[3].tanh(),
        input.tanh(),
    ];
    (
        [
            CAP_RATIO * (t[4] - t[0]),
            t[0] - t[1],
            t[1] - t[2],
            t[2] - t[3],
        ],
        t.map(|v| 1.0 - v * v),
    )
}
fn solve(mut a: [[f64; 5]; 4]) -> [f64; 4] {
    for i in 0..4 {
        let pivot = (i..4)
            .max_by(|&l, &r| a[l][i].abs().total_cmp(&a[r][i].abs()))
            .unwrap();
        a.swap(i, pivot);
        let divisor = a[i][i];
        for value in &mut a[i][i..] {
            *value /= divisor;
        }
        let pivot_row = a[i];
        for row in a.iter_mut().skip(i + 1) {
            let factor = row[i];
            for (value, pivot) in row[i..].iter_mut().zip(&pivot_row[i..]) {
                *value -= factor * pivot;
            }
        }
    }
    let mut x = [0.0; 4];
    for i in (0..4).rev() {
        x[i] = a[i][4];
        for j in i + 1..4 {
            x[i] -= a[i][j] * x[j];
        }
    }
    x
}

/// D24, R46 (47k), the second resonance-pot section (100k), C13 (1uF),
/// and the 100k summing load. Charge is retained between accented notes.
#[derive(Debug, Default)]
struct AccentSweep {
    capacitor: f64,
}
impl AccentSweep {
    fn process(&mut self, source: f64, resonance: f64, rate: f64) -> f64 {
        let ra = 47_000.0 + resonance * 100_000.0;
        let rb = (1.0 - resonance) * 100_000.0;
        let load = 100_000.0;
        let off_wiper = self.capacitor * load / (rb + load);
        let on = source > off_wiper + 0.6;
        let (target, resistance) = if on {
            (
                (source - 0.6) * load / (ra + load),
                rb + ra * load / (ra + load),
            )
        } else {
            (0.0, rb + load)
        };
        self.capacitor +=
            (target - self.capacitor) * (1.0 - (-1.0 / (rate * resistance * 1e-6)).exp());
        if rb < 1.0 {
            self.capacitor
        } else if on {
            ((source - 0.6) / ra + self.capacitor / rb) / (1.0 / ra + 1.0 / rb + 1.0 / load)
        } else {
            self.capacitor * load / (rb + load)
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Note {
    channel: u8,
    note: u8,
    velocity: u8,
    owner: u32,
}

pub struct Runtime {
    rate: f64,
    oversample: usize,
    levels: usize,
    decimators: [Decimator; 5],
    filter: Ladder,
    sweep: AccentSweep,
    controls: Controls,
    phase: f64,
    pitch: f64,
    target_pitch: f64,
    gate: bool,
    accented: bool,
    sliding: bool,
    attack: u32,
    main_env: f64,
    volume_env: f64,
    accent_vca: f64,
    notes: [Note; 32],
    held: usize,
    bend: [f64; 16],
    beat: f64,
    running: bool,
    pattern_running: bool,
    step: Option<u64>,
    view: View,
    radias: synth::Mono,
    radias_sample: SampleBuffer,
    transpose: f32,
    lfo: f32,
    external_notes: bool,
}
impl std::fmt::Debug for Runtime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("303")
            .field("rate", &self.rate)
            .finish_non_exhaustive()
    }
}
impl Runtime {
    pub fn new(rate: u32, controls: Controls) -> Self {
        let rate = rate.clamp(8000, 384000) as f64;
        let mut levels = 0;
        while rate * ((1 << levels) as f64) < 176_400.0 && levels < 5 {
            levels += 1;
        }
        let oversample = 1 << levels;
        Self {
            rate,
            oversample,
            levels,
            decimators: std::array::from_fn(|_| Decimator::new()),
            filter: Ladder::new(rate * oversample as f64),
            sweep: AccentSweep::default(),
            controls,
            phase: 0.0,
            pitch: 36.0,
            target_pitch: 36.0,
            gate: false,
            accented: false,
            sliding: false,
            attack: 0,
            main_env: 0.0,
            volume_env: 0.0,
            accent_vca: 0.0,
            notes: [Note::default(); 32],
            held: 0,
            bend: [0.0; 16],
            beat: 0.0,
            running: false,
            pattern_running: false,
            step: None,
            view: View::default(),
            radias: {
                let mut bank = synth::Mono::new((rate * oversample as f64) as u32);
                bank.set_source(0.0, false);
                bank
            },
            // RADIAS waves here do not read PCM. Keep the shared carrier API's
            // sample dependency allocated once, away from the audio callback.
            radias_sample: SampleBuffer::new(String::new(), rate as u32, vec![[0.0; 2]])
                .expect("valid silent sample"),
            transpose: 0.0,
            lfo: 0.0,
            external_notes: false,
        }
    }
    pub fn inherit(&mut self, old: &mut Self) {
        if self.rate == old.rate {
            std::mem::swap(self, old);
        }
    }
    pub fn transport(&mut self, playing: bool) {
        if self.running == playing {
            return;
        }
        self.running = playing;
        self.step = None;
        if playing {
            self.beat = 0.0;
        } else if self.held == 0 {
            self.gate = false;
            self.accented = false;
            self.sliding = false;
            self.attack = 0;
        }
    }
    pub fn stop(&mut self) {
        self.held = 0;
        self.gate = false;
        self.accented = false;
        self.sliding = false;
        self.attack = 0;
        self.running = false;
        self.step = None;
    }
    pub fn seek(&mut self, beat: f64, restart: bool) {
        if restart || (self.beat - beat).abs() > 0.25 {
            self.step = None;
            if self.held == 0 {
                self.gate = false;
            }
        }
        self.beat = beat.max(0.0);
    }
    fn note(&mut self, note: u8, accent: bool, slide: bool) {
        self.target_pitch = f64::from(note);
        self.accented = accent;
        self.sliding = slide && self.gate;
        if !self.sliding {
            self.pitch = self.target_pitch;
            self.attack = (self.rate * self.oversample as f64 * 0.007).round() as u32;
        }
        self.gate = true;
    }
    pub fn legato(&mut self, channel: u8, note: u8, velocity: u8, owner: u32) {
        self.notes[..self.held]
            .iter_mut()
            .filter(|n| n.owner == owner)
            .for_each(|n| {
                n.note = note;
                n.channel = channel;
                n.velocity = velocity;
            });
        self.note(note.min(127), velocity >= 100, true);
    }
    pub fn midi(&mut self, message: Message, owner: u32) -> bool {
        match message {
            Message::NoteOn {
                channel,
                note,
                velocity: 1..=127,
            } if channel < 16 && note < 128 => {
                let Message::NoteOn { velocity, .. } = message else {
                    unreachable!()
                };
                self.remove_note(channel, note, owner);
                if self.held == self.notes.len() {
                    self.notes.copy_within(1.., 0);
                    self.held -= 1;
                }
                let slide = self.held > 0;
                self.notes[self.held] = Note {
                    channel,
                    note,
                    velocity,
                    owner,
                };
                self.held += 1;
                self.note(note, velocity >= 100, slide);
            }
            Message::NoteOff { channel, note, .. }
            | Message::NoteOn {
                channel,
                note,
                velocity: 0,
            } => {
                let active = self.held.checked_sub(1).is_some_and(|i| {
                    let n = self.notes[i];
                    n.channel == channel && n.note == note && n.owner == owner
                });
                self.remove_note(channel, note, owner);
                if active {
                    if self.held > 0 {
                        let n = self.notes[self.held - 1];
                        self.note(n.note, n.velocity >= 100, true);
                    } else {
                        self.gate = false;
                        self.sliding = false;
                    }
                }
            }
            Message::PitchBend { channel, value } if channel < 16 => {
                self.bend[channel as usize] = (value.min(16383) as f64 - 8192.0) / 8192.0 * 2.0
            }
            Message::Control {
                channel,
                controller: 120 | 123,
                ..
            } => {
                let mut i = 0;
                while i < self.held {
                    if self.notes[i].channel == channel {
                        let n = self.notes[i];
                        self.remove_note(n.channel, n.note, n.owner);
                    } else {
                        i += 1;
                    }
                }
                if self.held == 0 {
                    self.gate = false;
                    self.accented = false;
                }
            }
            Message::Reset => {
                self.held = 0;
                self.gate = false;
                self.accented = false;
                self.attack = 0;
                self.volume_env = 0.0;
                self.accent_vca = 0.0;
                self.bend.fill(0.0);
            }
            Message::Program { .. } => {}
            _ => return false,
        }
        true
    }
    fn remove_note(&mut self, channel: u8, note: u8, owner: u32) {
        if let Some(i) = self.notes[..self.held]
            .iter()
            .position(|n| n.channel == channel && n.note == note && n.owner == owner)
        {
            self.notes.copy_within(i + 1..self.held, i);
            self.held -= 1;
        }
    }
    pub fn view(&self) -> View {
        self.view
    }
    pub fn configure(
        &mut self,
        radias: &crate::synth::Settings,
        transpose: f32,
        lfo: f32,
        notes: bool,
    ) {
        self.radias
            .configure(radias, 0.0, &self.radias_sample, 60.0);
        self.transpose = transpose;
        self.lfo = lfo;
        self.external_notes = notes;
    }
    pub fn note_state(&self) -> Option<(u8, u8, u8)> {
        if self.held > 0 {
            let n = self.notes[self.held - 1];
            Some((n.channel, n.note, n.velocity))
        } else {
            self.gate.then_some((
                0,
                self.target_pitch as u8,
                if self.accented { 127 } else { 80 },
            ))
        }
    }
    pub fn frame(
        &mut self,
        settings: &Settings,
        controls: Controls,
        playing: bool,
        bpm: f32,
    ) -> Stereo {
        self.transport(playing);
        let pattern = playing && settings.pattern && !self.external_notes;
        if pattern != self.pattern_running {
            self.step = None;
            self.pattern_running = pattern;
            if self.held == 0 {
                self.gate = false;
            }
        }
        let serial = (self.beat * 4.0 + 1e-9).floor() as u64;
        let index = serial as usize % settings.length as usize;
        if pattern {
            if self.step != Some(serial) {
                let previous = (index + settings.length as usize - 1) % settings.length as usize;
                let tie = self.step == serial.checked_sub(1)
                    && settings.steps[previous].slide
                    && self.gate;
                let step = settings.steps[index];
                if self.held == 0 {
                    if step.gate {
                        self.note(step.note, step.accent, tie);
                    } else {
                        self.gate = false;
                        self.sliding = false;
                        self.accented = false;
                    }
                }
                self.step = Some(serial);
            }
            if self.held == 0
                && !settings.steps[index].slide
                && (self.beat * 4.0).fract() >= 7.0 / 12.0
            {
                self.gate = false;
            }
        }
        if playing {
            self.beat += f64::from(bpm) / (60.0 * self.rate);
        }
        let smoothing = (1.0 - (-1.0 / (self.rate * 0.002)).exp()) as f32;
        // Waveform also crossfades, although its stored control is a switch.
        for i in (0..crate::tb303::FIELDS.len()).filter(|&i| i != 6) {
            self.controls.set(
                i,
                self.controls.get(i) + (controls.get(i) - self.controls.get(i)) * smoothing,
            );
        }
        self.controls.waveform += (controls.waveform - self.controls.waveform) * smoothing;
        if self.sliding {
            self.pitch +=
                (self.target_pitch - self.pitch) * (1.0 - (-1.0 / (self.rate * SLIDE_TAU)).exp());
        } else {
            self.pitch = self.target_pitch;
        }
        let bend = if self.held > 0 {
            self.bend[self.notes[self.held - 1].channel as usize]
        } else {
            0.0
        };
        let pitch = self.pitch + bend + f64::from(self.transpose + self.controls.tuning);
        let frequency = (440.0
            * 2.0_f64.powf((pitch + f64::from(self.controls.transpose) - 69.0) / 12.0))
        .min(self.rate * 0.4);
        let rate = self.rate * self.oversample as f64;
        let dt = frequency / rate;
        let decay = if self.accented {
            0.2
        } else {
            0.2 * 12.5_f64.powf(self.controls.decay as f64)
        } / LN_10;
        let main_decay = (-1.0 / (rate * decay)).exp();
        let volume_decay = (-LN_10 / (rate * 3.5)).exp();
        let release = (-1.0 / (rate * 0.004)).exp();
        let charge = 1.0 - (-1.0 / (rate * 0.001)).exp();
        let accent_alpha = 1.0 - (-1.0 / (rate * 47_000.0 * 0.033e-6)).exp();
        let cutoff = 80.0 * 32.0_f64.powf(self.controls.cutoff as f64);
        let mut output = [0.0; 2];
        for sample in 0..self.oversample {
            if self.attack > 0 {
                self.main_env += (1.0 - self.main_env) * charge;
                if self.gate && self.attack as f64 <= rate * 0.003 {
                    self.volume_env += (1.0 - self.volume_env) * charge;
                }
                self.attack -= 1;
            } else {
                self.main_env *= main_decay;
                self.volume_env *= volume_decay;
            }
            if !self.gate {
                self.volume_env *= release;
            }
            let accent = if self.accented {
                self.main_env * self.controls.accent as f64
            } else {
                0.0
            };
            self.accent_vca += (accent - self.accent_vca) * accent_alpha;
            let sweep = self
                .sweep
                .process(accent * 4.0, self.controls.resonance as f64, rate);
            let hz = (cutoff
                * 2.0_f64.powf(
                    (0.25 + 5.0 * self.controls.env_mod as f64) * self.main_env + 1.5 * sweep,
                ))
            .clamp(20.0, 18_000.0_f64.min(rate * 0.15));
            let phase = self.phase;
            let saw = 2.0 * phase - 1.0 - blep(phase, dt);
            let square = (if phase < 0.5 { 1.0 } else { -1.0 }) + blep(phase, dt)
                - blep((phase + 0.5).fract(), dt);
            let osc = saw + (square - saw) * self.controls.waveform as f64;
            self.phase = (phase + dt).fract();
            let extra = if self.radias.active() {
                self.radias.set_source(osc as f32, phase + dt >= 1.0);
                self.radias.begin_frame(self.lfo);
                self.radias
                    .render(0, 0, pitch as f32, &self.radias_sample, &(0..1))[0]
                    as f64
            } else {
                0.0
            };
            let filtered = self.filter.process(
                osc * 0.35 * f64::from(self.controls.level) + extra,
                hz,
                self.controls.resonance as f64,
            );
            // OTA/VCA small-signal gain with smooth current saturation.
            let vca = self.volume_env + self.accent_vca * 0.7;
            let value = (filtered * 2.0).tanh() * vca * 2.0;
            let mut frame = [value as f32; 2];
            for level in 0..self.levels {
                frame = self.decimators[level].next(frame);
                if (sample + 1) % (1 << (level + 1)) != 0 {
                    break;
                }
            }
            output = frame;
        }
        self.view = View {
            step: pattern.then_some(index as u8),
            gate: self.gate,
            accent: self.accented,
            slide: self.sliding,
            note: (pitch + f64::from(self.controls.transpose)) as f32,
            envelope: self.main_env as f32,
            accent_sweep: self.sweep.capacitor as f32,
        };
        output
    }
}

fn blep(phase: f64, dt: f64) -> f64 {
    if phase < dt {
        let t = phase / dt;
        t + t - t * t - 1.0
    } else if phase > 1.0 - dt {
        let t = (phase - 1.0) / dt;
        t * t + t + t + 1.0
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn on(note: u8, velocity: u8) -> Message {
        Message::NoteOn {
            channel: 0,
            note,
            velocity,
        }
    }
    fn off(note: u8) -> Message {
        Message::NoteOff {
            channel: 0,
            note,
            velocity: 0,
        }
    }
    fn render(r: &mut Runtime, s: &Settings, n: usize, playing: bool) -> f64 {
        (0..n)
            .map(|_| {
                let x = r.frame(s, s.controls, playing, 120.0);
                assert!(x.iter().all(|x| x.is_finite() && x.abs() < 4.0));
                assert_eq!(x[0], x[1]);
                f64::from(x[0]).powi(2)
            })
            .sum()
    }

    #[test]
    fn slide_follows_the_original_rc_and_keeps_the_envelope() {
        for rate in [8000, 44100, 48000, 96000] {
            let s = Settings {
                pattern: false,
                ..Default::default()
            };
            let mut r = Runtime::new(rate, s.controls);
            r.midi(on(36, 80), 0);
            render(&mut r, &s, rate as usize / 10, false);
            let envelope = r.main_env;
            r.midi(on(48, 80), 0);
            assert!(r.sliding);
            assert_eq!(r.attack, 0);
            render(
                &mut r,
                &s,
                (rate as f64 * SLIDE_TAU).round() as usize,
                false,
            );
            assert!((r.pitch - (48.0 - 12.0 / std::f64::consts::E)).abs() < 0.02);
            assert!(r.main_env < envelope, "Slide retriggered the envelope");
            r.midi(off(48), 0);
            assert_eq!(r.target_pitch, 36.0);
            assert!(r.gate);
            r.midi(off(36), 0);
            assert!(!r.gate);
            assert!(render(&mut r, &s, rate as usize / 5, false) < 10.0);
        }
    }

    #[test]
    fn repeated_accents_retain_charge_and_accent_decay_is_short() {
        let rate = 48_000;
        let mut s = Settings {
            pattern: false,
            ..Default::default()
        };
        s.controls.decay = 1.0;
        s.controls.resonance = 1.0;
        s.controls.accent = 1.0;
        let mut r = Runtime::new(rate, s.controls);
        r.midi(on(36, 127), 0);
        render(&mut r, &s, rate as usize / 10, false);
        let first = r.sweep.capacitor;
        r.midi(off(36), 0);
        render(&mut r, &s, rate as usize / 100, false);
        r.midi(on(36, 127), 0);
        render(&mut r, &s, rate as usize / 10, false);
        assert!(
            r.sweep.capacitor > first * 1.05,
            "Accents lost capacitor memory"
        );
        assert!(
            r.main_env < 0.4,
            "Accented notes followed the long Decay knob"
        );
        let mut normal = Runtime::new(rate, s.controls);
        normal.midi(on(36, 80), 0);
        render(&mut normal, &s, rate as usize / 10, false);
        assert!(normal.main_env > 0.8);
    }

    #[test]
    fn pattern_rests_slides_stop_and_parameter_changes_keep_audio_finite() {
        for rate in [8000, 44100, 48000, 96000] {
            let mut s = Settings::default();
            let mut r = Runtime::new(rate, s.controls);
            assert_eq!(render(&mut r, &s, 64, false), 0.0);
            assert!(render(&mut r, &s, rate as usize / 8, true) > 0.001);
            for &(cutoff, resonance) in &[(0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 0.0)] {
                s.controls.cutoff = cutoff;
                s.controls.resonance = resonance;
                render(&mut r, &s, rate as usize / 4, true);
            }
            r.transport(false);
            render(&mut r, &s, rate as usize / 4, false);
            assert!(render(&mut r, &s, 512, false) < 1e-8);
            assert_eq!(r.view.step, None);
        }
    }

    // Small-signal circuit reference: the loaded RC ladder's characteristic
    // polynomial and the published coupling transfer, independent of the solver.
    type C = (f64, f64);
    fn mul(a: C, b: C) -> C {
        (a.0 * b.0 - a.1 * b.1, a.0 * b.1 + a.1 * b.0)
    }
    fn div(a: C, b: C) -> C {
        let d = b.0 * b.0 + b.1 * b.1;
        ((a.0 * b.0 + a.1 * b.1) / d, (a.1 * b.0 - a.0 * b.1) / d)
    }
    fn reference(hz: f64, cutoff: f64, resonance: f64) -> f64 {
        let w = TAU * hz;
        let p = w / (TAU * cutoff / CAP_RATIO.powf(0.25));
        let r = CAP_RATIO;
        let core = div(
            (r, 0.0),
            (
                p.powi(4) - (10.0 + 5.0 * r) * p * p + r,
                -(6.0 + r) * p.powi(3) + (4.0 + 6.0 * r) * p,
            ),
        );
        let poles = [97.5, 38.5, 4.45, 578.1, 20.0, 7.41];
        let f = [0.0, 0.0, 0.0, 109.9, 34.0, 7.41]
            .iter()
            .zip(poles)
            .fold((1.06, 0.0), |a, (&z, p)| mul(a, div((z, w), (p, w))));
        let b = [0.0, 0.0, 0.0, 0.0, 46.5, 4.4]
            .iter()
            .zip(poles)
            .fold((18.7 * resonance, 0.0), |a, (&z, p)| {
                mul(a, div((z, w), (p, w)))
            });
        let loop_gain = mul(core, b);
        let h = div(mul(core, f), (1.0 + loop_gain.0, loop_gain.1));
        h.0.hypot(h.1)
    }
    #[test]
    fn quiet_filter_matches_the_circuit_transfer_function() {
        let rate = 192_000;
        for resonance in [0.0, 0.7] {
            for hz in [125.0, 500.0, 1000.0, 2000.0] {
                let mut f = Ladder::new(rate as f64);
                let mut re = 0.0;
                let mut im = 0.0;
                for n in 0..rate {
                    let phase = TAU * hz * n as f64 / rate as f64;
                    let out = f.process(phase.sin() * 1e-5, 1000.0, resonance);
                    if n >= rate / 2 {
                        re += out * phase.cos();
                        im += out * phase.sin();
                    }
                }
                let measured = re.hypot(im) * 4.0 / rate as f64 / 1e-5;
                let expected = reference(hz, 1000.0, resonance);
                assert!(
                    (measured / expected - 1.0).abs() < 0.025,
                    "{hz}, {resonance}: {measured} vs {expected}"
                );
            }
        }
    }
}
