//! Portable desktop effects. These replace, rather than translate, unresolved
//! NEON kernels; the original reverb impulse responses are not reproduced.
use super::fast;
use crate::{
    Stereo,
    preset::{Params, ReverbType},
};

pub struct Delay {
    frames: Vec<Stereo>,
    cursor: usize,
}
impl Delay {
    pub fn new(frames: usize) -> Self {
        Self {
            frames: vec![[0.0; 2]; frames.max(4)],
            cursor: 0,
        }
    }
    pub(crate) fn empty() -> Self {
        Self {
            frames: Vec::new(),
            cursor: 0,
        }
    }
    pub(crate) fn has_storage(&self) -> bool {
        !self.frames.is_empty()
    }
    pub(crate) fn take_storage(&mut self) -> Vec<Stereo> {
        std::mem::take(&mut self.frames)
    }
    pub(crate) fn install_storage(&mut self, frames: Vec<Stereo>) {
        debug_assert!(self.frames.is_empty());
        self.frames = frames;
    }
    #[inline(always)]
    pub fn push(&mut self, frame: Stereo) {
        self.frames[self.cursor] = frame;
        // The cursor is always inside the buffer: wrapping by comparison is the
        // same as the modulo it replaces, without an integer division per frame.
        self.cursor += 1;
        if self.cursor == self.frames.len() {
            self.cursor = 0;
        }
    }
    #[inline(always)]
    pub fn read(&self, samples: f32) -> Stereo {
        let len = self.frames.len();
        // The fractional delay splits into its ceiling and the distance to it.
        // Previously this subtracted `samples` from the cursor in float64 and
        // took the floor and fraction of the result; both were exact there
        // (the span of bits stays under 53), so this integer index is that
        // floor and `f` is that fraction: `whole - samples` is exact by
        // Sterbenz's lemma, as `whole / 2 <= samples <= whole`.
        let samples = samples.clamp(1.0, (len - 2) as f32);
        let whole = samples.ceil();
        let f = whole - samples;
        let i = self.cursor + len - whole as usize;
        let i = if i >= len { i - len } else { i };
        let a = self.frames[i];
        let b = self.frames[if i + 1 == len { 0 } else { i + 1 }];
        [a[0] + (b[0] - a[0]) * f, a[1] + (b[1] - a[1]) * f]
    }
    pub fn clear(&mut self) {
        self.frames.fill([0.0; 2]);
        self.cursor = 0;
    }
}

pub(crate) type Storage = [Vec<Stereo>; 13];

pub struct Effects {
    sample_rate: f32,
    predelay: Delay,
    tank: [Delay; 8],
    tank_damping: [f32; 8],
    feedback: [f32; 8],
    lengths: [f32; 8],
    diffuser: [Delay; 2],
    reverse: Delay,
    reverse_phase: usize,
    shifter: Delay,
    shifter_phase: f64,
    tone: f32,
    predelay_samples: f32,
    /// Consecutive frames that wrote only zeros into every line and left the
    /// tank filters at zero.
    quiet: usize,
    /// The longest line. After this many quiet frames every line holds only
    /// zeros, so where its cursor stands no longer matters.
    horizon: usize,
    /// The type those quiet frames ran with.
    quiet_type: ReverbType,
}

impl Effects {
    pub fn new(sample_rate: u32) -> Self {
        Self::create(sample_rate, true)
    }
    pub(crate) fn unallocated(sample_rate: u32) -> Self {
        Self::create(sample_rate, false)
    }
    fn create(sample_rate: u32, allocate: bool) -> Self {
        let sr = sample_rate as f32;
        let delay = |frames| {
            if allocate {
                Delay::new(frames)
            } else {
                Delay::empty()
            }
        };
        Self {
            sample_rate: sr,
            predelay: delay((sr * 0.25) as usize),
            tank: std::array::from_fn(|_| delay((sr * 0.18) as usize)),
            tank_damping: [0.0; 8],
            feedback: [0.8; 8],
            lengths: [1000.0; 8],
            diffuser: [delay((sr * 0.013) as usize), delay((sr * 0.019) as usize)],
            reverse: delay((sr * 0.6) as usize + 4),
            reverse_phase: 0,
            shifter: delay((sr * 0.1) as usize + 8),
            shifter_phase: 0.0,
            tone: 0.5,
            predelay_samples: 1.0,
            quiet: 0,
            horizon: [
                (sr * 0.25) as usize,
                (sr * 0.18) as usize,
                (sr * 0.013) as usize,
                (sr * 0.019) as usize,
                (sr * 0.6) as usize + 4,
                (sr * 0.1) as usize + 8,
            ]
            .into_iter()
            .max()
            .unwrap_or(0)
            .max(4),
            quiet_type: ReverbType::PlateS,
        }
    }

    fn delays(&mut self) -> impl Iterator<Item = &mut Delay> {
        std::iter::once(&mut self.predelay)
            .chain(self.tank.iter_mut())
            .chain(self.diffuser.iter_mut())
            .chain(std::iter::once(&mut self.reverse))
            .chain(std::iter::once(&mut self.shifter))
    }
    pub(crate) fn has_storage(&self) -> bool {
        self.predelay.has_storage()
    }
    pub(crate) fn take_storage(&mut self) -> Storage {
        let mut storage = std::array::from_fn(|_| Vec::new());
        for (delay, frames) in self.delays().zip(storage.iter_mut()) {
            std::mem::swap(&mut delay.frames, frames);
        }
        storage
    }
    pub(crate) fn install_storage(&mut self, mut storage: Storage) {
        // Lines from another slot may still ring.
        self.quiet = 0;
        for (delay, frames) in self.delays().zip(storage.iter_mut()) {
            debug_assert!(delay.frames.is_empty());
            std::mem::swap(&mut delay.frames, frames);
        }
    }

    pub fn configure(&mut self, p: &Params) {
        let scale = match p.reverb_type {
            ReverbType::PlateS => 0.65,
            ReverbType::PlateL => 1.45,
            ReverbType::Room => 0.4,
            ReverbType::Reverse => 1.0,
        };
        let ms = [29.7, 37.1, 41.1, 43.7, 53.3, 59.3, 61.7, 71.9];
        let decay = 0.25 + p.reverb_decay.powi(2) * 14.0;
        for (i, ms) in ms.iter().enumerate() {
            self.lengths[i] = self.sample_rate * ms * 0.001 * scale;
            self.feedback[i] = 10.0_f32.powf(-3.0 * self.lengths[i] / (self.sample_rate * decay));
        }
        self.tone = 0.03 + 0.9 * p.reverb_tone.powi(2);
        self.predelay_samples = (p.predelay.powi(2) * 0.2 * self.sample_rate).max(1.0);
    }

    pub fn reverb(&mut self, frame: Stereo, p: &Params, wet: f32) -> Stereo {
        // The reverse line is written only in Reverse: a type change may
        // read lines the quiet frames did not fill with zeros.
        if p.reverb_type != self.quiet_type {
            self.quiet_type = p.reverb_type;
            self.quiet = 0;
        }
        let zero = frame[0] == 0.0 && frame[1] == 0.0;
        if zero && self.quiet >= self.horizon {
            return self.sleep(frame, p, wet);
        }
        self.predelay.push(frame);
        let mut written = !zero;
        let mut input = self.predelay.read(self.predelay_samples);
        if p.reverb_type == ReverbType::Reverse {
            written |= input[0] != 0.0 || input[1] != 0.0;
            self.reverse.push(input);
            let window = (self.sample_rate * 0.22) as usize;
            let phase = self.reverse_phase as f32 / window as f32;
            let other = (phase + 0.5).fract();
            let a = self.reverse.read((self.reverse_phase * 2 + 1) as f32);
            let b = self
                .reverse
                .read((other * window as f32 * 2.0 + 1.0).min(self.sample_rate * 0.55));
            let wa = fast::sin2_pi(phase);
            input = std::array::from_fn(|i| a[i] * wa + b[i] * (1.0 - wa));
            self.reverse_phase = (self.reverse_phase + 1) % window;
        }
        for i in 0..2 {
            let old =
                self.diffuser[i].read(self.sample_rate * (if i == 0 { 0.0097 } else { 0.0151 }));
            let gain = p.diffuse * 0.7;
            let next: Stereo = std::array::from_fn(|c| fast::flush(input[c] + old[c] * gain));
            written |= next[0] != 0.0 || next[1] != 0.0;
            self.diffuser[i].push(next);
            input = std::array::from_fn(|c| old[c] - next[c] * gain);
        }
        written |= input[0] != 0.0 || input[1] != 0.0;
        self.shifter.push(input);
        let shift_window = self.sample_rate * 0.06;
        let phase = self.shifter_phase as f32;
        let other = (phase + 0.5).fract();
        let a = self.shifter.read(1.0 + (1.0 - phase) * shift_window);
        let b = self.shifter.read(1.0 + (1.0 - other) * shift_window);
        let wa = fast::sin2_pi(phase);
        self.shifter_phase =
            super::sample::wrap_position(self.shifter_phase + 1.0 / shift_window as f64, 1.0);
        input = std::array::from_fn(|c| {
            input[c] * (1.0 - p.shimmer * 0.5) + (a[c] * wa + b[c] * (1.0 - wa)) * p.shimmer * 0.5
        });
        let mut taps = [0.0_f32; 8];
        for (i, tap) in taps.iter_mut().enumerate() {
            let x = self.tank[i].read(self.lengths[i])[0];
            self.tank_damping[i] =
                fast::flush(self.tank_damping[i] + self.tone * (x - self.tank_damping[i]));
            *tap = self.tank_damping[i];
        }
        let reflection = taps.iter().sum::<f32>() * 0.25;
        for (i, &tap) in taps.iter().enumerate() {
            let value = input[i % 2] * 0.22 + (tap - reflection) * self.feedback[i];
            let clean = if value.abs() < 1e-20 {
                0.0
            } else {
                value.clamp(-8.0, 8.0)
            };
            written |= clean != 0.0;
            self.tank[i].push([clean, clean]);
        }
        written |= self.tank_damping.iter().any(|&d| d != 0.0);
        self.quiet = if written { 0 } else { self.quiet + 1 };
        let tail = [
            (taps[0] + taps[2] - taps[4] + taps[6]) * 0.5,
            (taps[1] + taps[3] - taps[5] + taps[7]) * 0.5,
        ];
        std::array::from_fn(|i| frame[i] * (1.0 - wet * 0.4) + tail[i] * wet * 2.0)
    }

    /// A zero frame into a reverb whose lines hold only zeros: the frame the
    /// full path renders, without reading or writing the lines.
    ///
    /// The tank holds +0.0 (writes below 1e-20 store 0.0) and its filters
    /// are +0.0 (flushed), so every tap and the tail are +0.0, exactly as
    /// reading the lines would give; a zero of either sign in the lines
    /// before the tank only ever meets a non-zero value or a flush on its way
    /// in. The modulation phases advance as they do there.
    fn sleep(&mut self, frame: Stereo, p: &Params, wet: f32) -> Stereo {
        if p.reverb_type == ReverbType::Reverse {
            let window = (self.sample_rate * 0.22) as usize;
            self.reverse_phase = (self.reverse_phase + 1) % window;
        }
        let shift_window = self.sample_rate * 0.06;
        self.shifter_phase =
            super::sample::wrap_position(self.shifter_phase + 1.0 / shift_window as f64, 1.0);
        let tail = [0.0_f32; 2];
        std::array::from_fn(|i| frame[i] * (1.0 - wet * 0.4) + tail[i] * wet * 2.0)
    }

    pub fn clear(&mut self) {
        self.tank_damping = [0.0; 8];
        self.reverse_phase = 0;
        self.shifter_phase = 0.0;
        self.quiet = 0;
        self.predelay.clear();
        self.reverse.clear();
        self.shifter.clear();
        self.tank.iter_mut().for_each(Delay::clear);
        self.diffuser.iter_mut().for_each(Delay::clear);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tables::Random;

    #[test]
    fn a_sleeping_reverb_renders_the_frames_of_the_full_path() {
        let rate = 8000;
        let mut sleeping = Effects::new(rate);
        let mut awake = Effects::new(rate);
        let mut random = Random::default();
        let mut p = Params {
            reverb_decay: 0.0,
            reverb_tone: 0.4,
            predelay: 0.3,
            shimmer: 0.6,
            diffuse: 0.8,
            ..Params::default()
        };
        let types = [
            ReverbType::PlateS,
            ReverbType::Reverse,
            ReverbType::Room,
            ReverbType::PlateL,
            ReverbType::Reverse,
        ];
        let mut slept = 0;
        for (round, kind) in types.into_iter().enumerate() {
            p.reverb_decay = round as f32 * 0.05;
            for n in 0..rate as usize * 6 {
                // A type change in the middle of the silence, a burst at its start.
                if n == rate as usize * 4 {
                    p.reverb_type = kind;
                }
                if n % 4000 == 0 {
                    sleeping.configure(&p);
                    awake.configure(&p);
                }
                let frame = if n < 300 {
                    [random.bipolar() * 0.5, if n % 7 == 0 { -0.0 } else { random.bipolar() }]
                } else if n % 2 == 0 {
                    [-0.0, 0.0]
                } else {
                    [0.0; 2]
                };
                if sleeping.quiet >= sleeping.horizon && frame == [0.0; 2] {
                    slept += 1;
                }
                awake.quiet = 0;
                let a = sleeping.reverb(frame, &p, 1.0);
                let b = awake.reverb(frame, &p, 1.0);
                assert_eq!(
                    a.map(f32::to_bits),
                    b.map(f32::to_bits),
                    "round {round}, frame {n}"
                );
            }
        }
        assert!(slept > rate as usize * 5, "slept for only {slept} frames");
    }
}
