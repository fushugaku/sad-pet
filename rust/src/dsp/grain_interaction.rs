//! Bounded resampling state for phase-addressed grain readheads and Ring.
//! Ordinary grains keep their original, bit-identical read and DSP path.
use super::{MAX_GRAINS, interaction::Input, resynth::Decimator};
use crate::Stereo;

const FACTOR: usize = 4;
struct Reducer {
    stages: [Decimator; 2],
}
impl Reducer {
    fn new() -> Self {
        Self {
            stages: std::array::from_fn(|_| Decimator::new()),
        }
    }
    fn clear(&mut self) {
        for s in &mut self.stages {
            s.clear();
        }
    }
    fn push(&mut self, mut frame: Stereo, sub: usize) -> Stereo {
        frame = self.stages[0].next(frame);
        if sub & 1 == 1 {
            frame = self.stages[1].next(frame);
        }
        frame
    }
}
struct State {
    read: Reducer,
    ring: Reducer,
    correction: Stereo,
    previous: Stereo,
    read_blend: f32,
    ring_blend: f32,
    read_started: bool,
    ring_started: bool,
}
impl State {
    fn new() -> Self {
        Self {
            read: Reducer::new(),
            ring: Reducer::new(),
            correction: [0.0; 2],
            previous: [0.0; 2],
            read_blend: 0.0,
            ring_blend: 0.0,
            read_started: false,
            ring_started: false,
        }
    }
    fn clear(&mut self) {
        self.read.clear();
        self.ring.clear();
        self.correction = [0.0; 2];
        self.previous = [0.0; 2];
        self.read_blend = 0.0;
        self.ring_blend = 0.0;
        self.read_started = false;
        self.ring_started = false;
    }
}
pub(super) struct Runtime {
    states: Box<[State]>,
    step: f32,
}
impl Runtime {
    pub fn new(rate: u32) -> Self {
        Self {
            states: (0..MAX_GRAINS)
                .map(|_| State::new())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            step: 1.0 / (rate as f32 * 0.005),
        }
    }
    pub fn reset(&mut self, slot: usize) {
        self.states[slot].clear();
    }
    pub fn reading(&self, slot: usize) -> bool {
        self.states[slot].read_started
    }
    /// `rate` already includes the finite Soft Sync phase trajectory. A hard
    /// reset changes only this trajectory, without touching the grain's life.
    pub fn read(
        &mut self,
        slot: usize,
        position: f64,
        rate: f64,
        reset: Option<(f64, f64)>,
        engaged: bool,
        mut read: impl FnMut(f64) -> Stereo,
    ) -> (Stereo, f64) {
        let state = &mut self.states[slot];
        let end = reset.map_or(position + rate, |(at, p)| p + rate * (1.0 - at));
        if !engaged && state.read_blend == 0.0 {
            if state.read_started {
                state.read.clear();
                state.correction = [0.0; 2];
                state.read_started = false;
            }
            return (read(position), end);
        }
        let dry = (state.read_blend < 1.0 || !engaged).then(|| read(position));
        if !state.read_started {
            for sub in 0..64 {
                state.read.push(dry.unwrap(), sub);
            }
            state.read_started = true;
        }
        let mut wet = [0.0; 2];
        for sub in 0..FACTOR {
            let time = sub as f64 / FACTOR as f64;
            let current = reset.map_or(position + rate * time, |(at, p)| {
                if time <= at {
                    position + rate * time
                } else {
                    p + rate * (time - at)
                }
            });
            let mut frame = if sub == 0 {
                dry.unwrap_or_else(|| read(current))
            } else {
                read(current)
            };
            for (sample, correction) in frame.iter_mut().zip(state.correction) {
                *sample += correction;
            }
            state.correction = [0.0; 2];
            if let Some((at, p)) = reset {
                let event = at.clamp(0.0, 1.0 - f64::EPSILON) * FACTOR as f64;
                if event.floor() as usize == sub {
                    let f = event.fract() as f32;
                    let left = 1.0 - f;
                    let before = read(position + rate * at);
                    let after = read(p);
                    let epsilon = rate / FACTOR as f64 / 1024.0;
                    let a = read(position + rate * at - epsilon);
                    let b = read(position + rate * at + epsilon);
                    let c = read(p - epsilon);
                    let d = read(p + epsilon);
                    for channel in 0..2 {
                        let jump = after[channel] - before[channel];
                        let slope = ((d[channel] - c[channel]) - (b[channel] - a[channel])) * 512.0;
                        frame[channel] +=
                            jump * 0.5 * left * left + slope * left * left * left / 6.0;
                        state.correction[channel] = -jump * 0.5 * f * f + slope * f * f * f / 6.0;
                    }
                }
            }
            wet = state.read.push(frame, sub);
        }
        let target = f32::from(engaged);
        state.read_blend += (target - state.read_blend).clamp(-self.step, self.step);
        if state.read_blend == 1.0 {
            return (wet, end);
        }
        let dry = dry.unwrap_or_else(|| read(position));
        (
            std::array::from_fn(|c| dry[c] + (wet[c] - dry[c]) * state.read_blend),
            end,
        )
    }
    /// Reconstruct both factors before multiplication and low-pass the
    /// sidebands before returning to the host sample rate.
    pub fn ring(&mut self, slot: usize, frame: Stereo, input: Input) -> Stereo {
        let state = &mut self.states[slot];
        let engaged = input.depth > 0.0;
        if !engaged && state.ring_blend == 0.0 {
            if state.ring_started {
                state.ring.clear();
                state.ring_started = false;
            }
            state.previous = frame;
            return frame;
        }
        if !state.ring_started {
            for sub in 0..64 {
                state.ring.push(frame, sub);
            }
            state.previous = frame;
            state.ring_started = true;
        }
        let mut output = frame;
        for sub in 0..FACTOR {
            let t = (sub + 1) as f32 / FACTOR as f32;
            let factor = input.factor_at(sub, FACTOR);
            let audio = std::array::from_fn(|c| {
                (state.previous[c] + (frame[c] - state.previous[c]) * t) * factor
            });
            output = state.ring.push(audio, sub);
        }
        state.previous = frame;
        let target = f32::from(engaged);
        state.ring_blend += (target - state.ring_blend).clamp(-self.step, self.step);
        std::array::from_fn(|c| frame[c] + (output[c] - frame[c]) * state.ring_blend)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fractional_reset_preserves_the_remainder_and_does_not_change_grain_life() {
        let mut r = Runtime::new(48000);
        let (_, position) = r.read(0, 100.0, 4.0, Some((0.75, 20.0)), true, |p| {
            [p as f32 / 100.0; 2]
        });
        assert_eq!(position, 21.0);
        for _ in 0..100 {
            let (audio, _) = r.read(0, 100.0, 4.0, Some((0.25, 20.0)), true, |p| {
                [p as f32 / 100.0; 2]
            });
            assert!(audio.iter().all(|v| v.is_finite()));
        }
    }
    #[test]
    fn settled_reads_share_substep_zero_and_off_retires_the_high_rate_path() {
        let mut r = Runtime::new(48000);
        for _ in 0..300 {
            r.read(0, 10.0, 1.0, None, true, |p| [p as f32; 2]);
        }
        let mut calls = 0;
        r.read(0, 10.0, 1.0, None, true, |p| {
            calls += 1;
            [p as f32; 2]
        });
        assert_eq!(calls, 4);
        for _ in 0..300 {
            r.read(0, 10.0, 1.0, None, false, |p| [p as f32; 2]);
        }
        calls = 0;
        let (frame, end) = r.read(0, 10.0, 1.0, None, false, |p| {
            calls += 1;
            [p as f32; 2]
        });
        assert_eq!((calls, frame, end), (1, [10.0; 2], 11.0));
        assert!(!r.reading(0));
    }
}
