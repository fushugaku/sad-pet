//! Grain feedback: the grains' own output, softly limited and without DC,
//! comes back for later grains to read. A timbre with a live history writes
//! it into that history; a file's grains read it from an overdub ring that
//! follows the file positions and is written where the grains are centered.
use super::fast;
use crate::Stereo;

/// Frames of the overdub ring, about five seconds at 48 kHz. Positions
/// further apart in a longer file share its frames.
const RING: usize = 1 << 18;
/// Frames of the ring cleared per call once feedback stops or the source
/// changes: a full clear takes sixteen control blocks.
const CLEAR_STEP: usize = RING / 16;
/// Loop gain at FEEDBACK 100%, just short of unity.
const MAX_GAIN: f32 = 0.98;

pub(crate) struct Feedback {
    ring: Box<[Stereo]>,
    /// The ring holds overdubs worth reading.
    written: bool,
    /// Clearing has reached this frame; None once the ring is clear.
    clearing: Option<usize>,
    last_write: Option<usize>,
    /// DC blocker state: previous input and output.
    dc: [Stereo; 2],
    /// This frame's feedback, which a live history takes in the next.
    pub(crate) output: Stereo,
}
impl Feedback {
    pub(crate) fn new() -> Self {
        Self {
            ring: vec![[0.0; 2]; RING].into_boxed_slice(),
            written: false,
            clearing: None,
            last_write: None,
            dc: [[0.0; 2]; 2],
            output: [0.0; 2],
        }
    }
    /// Forget everything fed back, as a new source starts.
    pub(crate) fn reset(&mut self) {
        self.output = [0.0; 2];
        self.dc = [[0.0; 2]; 2];
        self.last_write = None;
        if self.written || self.clearing.is_some() {
            self.written = false;
            self.clearing = Some(0);
        }
    }
    /// Some feedback is running or still sounding: worth reading.
    #[inline]
    pub(crate) fn readable(&self) -> bool {
        self.written
    }
    /// The grains' output at `amount` (0..=1), limited and without DC.
    pub(crate) fn feed(&mut self, grains: Stereo, amount: f32) -> Stereo {
        if amount <= 0.0 {
            if self.output != [0.0; 2] || self.dc != [[0.0; 2]; 2] {
                self.output = [0.0; 2];
                self.dc = [[0.0; 2]; 2];
            }
            return self.output;
        }
        let gain = amount.min(1.0) * MAX_GAIN;
        for (c, &x) in grains.iter().enumerate() {
            let x = if x.is_finite() { x } else { 0.0 };
            let y = x - self.dc[0][c] + 0.995 * self.dc[1][c];
            self.dc[0][c] = x;
            self.dc[1][c] = fast::flush(y);
            self.output[c] = fast::tanh(y * gain);
        }
        self.output
    }
    /// Overdubs this frame's feedback at a file position, filling frames the
    /// position skipped since the last write.
    pub(crate) fn write(&mut self, position: f64) {
        if self.clearing.is_some() || !position.is_finite() {
            return;
        }
        let index = position.max(0.0) as usize % RING;
        let gap = self
            .last_write
            .map_or(0, |last| (index + RING - last) % RING);
        let from = if (1..=8).contains(&gap) {
            self.last_write.unwrap() + 1
        } else {
            index
        };
        for i in from..=from + (index + RING - from) % RING {
            self.ring[i % RING] = self.output;
        }
        self.last_write = Some(index);
        self.written = true;
    }
    /// What was overdubbed at a file position, between frames.
    #[inline]
    pub(crate) fn read(&self, position: f64) -> Stereo {
        let whole = position.max(0.0).floor();
        let i = whole as usize % RING;
        let t = (position - whole) as f32;
        let (a, b) = (self.ring[i], self.ring[(i + 1) % RING]);
        [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t]
    }
    /// Once feedback is off and nothing is left to hear, clears the ring a
    /// step at a time; call once per control block.
    pub(crate) fn tidy(&mut self, amount: f32) {
        if amount <= 0.0 && self.written {
            self.written = false;
            self.clearing = Some(0);
            self.last_write = None;
        }
        if let Some(from) = self.clearing {
            let to = (from + CLEAR_STEP).min(RING);
            self.ring[from..to].fill([0.0; 2]);
            self.clearing = (to < RING).then_some(to);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overdubs_land_where_written_and_clear_once_feedback_stops() {
        let mut feedback = Feedback::new();
        assert!(!feedback.readable());
        let mut fed = Vec::new();
        for n in 0..10_000 {
            // A loud square wave: limited, and kept apart from its DC.
            let x = if n % 64 < 32 { 4.0 } else { -2.0 };
            let out = feedback.feed([x, -x], 1.0);
            assert!(out[0].abs() < 1.0, "limited");
            feedback.write(1_000.0 + n as f64);
            fed.push(out);
        }
        assert!(feedback.readable());
        assert_eq!(feedback.read(6_000.0), fed[5_000]);
        let mean = fed[5_000..].iter().map(|f| f[0]).sum::<f32>() / 5_000.0;
        assert!(mean.abs() < 0.05, "no DC builds up: {mean}");
        // Skipped positions are filled with the same frame.
        feedback.feed([0.5, 0.5], 1.0);
        feedback.write(20_000.0);
        feedback.feed([0.5, 0.5], 1.0);
        feedback.write(20_004.0);
        assert_eq!(feedback.read(20_002.0), feedback.read(20_004.0));
        for _ in 0..16 {
            feedback.tidy(0.0);
        }
        assert!(!feedback.readable());
        assert!(feedback.ring.iter().all(|f| *f == [0.0; 2]));
        assert_eq!(feedback.feed([1.0; 2], 0.0), [0.0; 2]);
    }
}
