//! Compact, independently addressed histories for MainPitch parties. A party
//! keeps one evolving history across overlapping note releases. The buffer is
//! prepared off audio; reassignment only changes the logical valid range.
use crate::Stereo;

const MAX_PARTIES: usize = super::VOICES - 1;

struct History {
    owner: u32,
    pcm: Vec<[u16; 2]>,
    first: u64,
    end: u64,
    sum: Stereo,
    weight: f32,
    feedback_sum: Stereo,
    feedback: Stereo,
    dc: [Stereo; 2],
}

impl History {
    fn new(frames: usize) -> Self {
        Self {
            owner: 0,
            pcm: vec![[0; 2]; frames],
            first: 0,
            end: 0,
            sum: [0.0; 2],
            weight: 0.0,
            feedback_sum: [0.0; 2],
            feedback: [0.0; 2],
            dc: [[0.0; 2]; 2],
        }
    }
    fn placeholder() -> Self {
        Self::new(0)
    }
    fn reset(&mut self, owner: u32) {
        self.owner = owner;
        self.first = 0;
        self.end = 0;
        self.sum = [0.0; 2];
        self.weight = 0.0;
        self.feedback_sum = [0.0; 2];
        self.feedback = [0.0; 2];
        self.dc = [[0.0; 2]; 2];
    }
    fn push(&mut self, time: u64) {
        if self.pcm.is_empty() || self.weight <= 0.0 {
            return;
        }
        if self.end != time {
            // A gap, seek or reused slot must never expose previous PCM.
            self.first = time;
            self.feedback = [0.0; 2];
            self.dc = [[0.0; 2]; 2];
        }
        let frame = std::array::from_fn(|c| encode(self.sum[c] / self.weight + self.feedback[c]));
        let length = self.pcm.len();
        self.pcm[(time % length as u64) as usize] = frame;
        self.end = time + 1;
        self.first = self
            .first
            .max(self.end.saturating_sub(self.pcm.len() as u64));
    }
    fn feed(&mut self, amount: f32, norm: f32) {
        if amount <= 0.0 || self.weight <= 0.0 {
            self.feedback = [0.0; 2];
            self.dc = [[0.0; 2]; 2];
            return;
        }
        for c in 0..2 {
            let x = self.feedback_sum[c] / self.weight * norm;
            let y = x - self.dc[0][c] + 0.995 * self.dc[1][c];
            self.dc[0][c] = x;
            self.dc[1][c] = super::fast::flush(y);
            self.feedback[c] = super::fast::tanh(y * amount.min(1.0) * 0.98);
        }
    }
    fn frame(&self, time: i64) -> Stereo {
        if time < 0 || (time as u64) < self.first || time as u64 >= self.end || self.pcm.is_empty()
        {
            return [0.0; 2];
        }
        self.pcm[time as usize % self.pcm.len()].map(decode)
    }
}

pub(super) struct SequenceHistories {
    rate: u32,
    slots: Vec<History>,
}

impl std::fmt::Debug for SequenceHistories {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SequenceHistories")
            .field("rate", &self.rate)
            .field("parties", &self.slots.len())
            .finish()
    }
}

impl SequenceHistories {
    /// Allocate only actual parties (at most the 32 sounding-note slots), with
    /// 32 seconds available for audio-rate changes to the history window.
    pub(super) fn new(rate: u32, parties: usize, previous: Option<&Self>) -> Self {
        let previous = previous.filter(|p| p.rate == rate);
        let count = parties
            .min(MAX_PARTIES)
            .max(previous.map_or(0, |p| p.slots.len()));
        let reused = previous.map_or(0, |p| p.slots.len().min(count));
        let slots = (0..count)
            .map(|i| {
                if i < reused {
                    History::placeholder()
                } else {
                    History::new(rate as usize * 32)
                }
            })
            .collect();
        Self { rate, slots }
    }

    /// The prepared placeholders take the existing buffers; only the old,
    /// empty placeholders retire with the exchanged preset.
    pub(super) fn inherit(&mut self, old: &mut Self) {
        if self.rate == old.rate {
            for (next, previous) in self.slots.iter_mut().zip(&mut old.slots) {
                if next.pcm.is_empty() {
                    std::mem::swap(next, previous);
                }
            }
        }
    }

    pub(super) fn begin_frame(&mut self) {
        for slot in &mut self.slots {
            slot.sum = [0.0; 2];
            slot.weight = 0.0;
            slot.feedback_sum = [0.0; 2];
        }
    }

    pub(super) fn ensure(&mut self, owner: u32, active: &[u32]) -> Option<usize> {
        if owner == 0 {
            return None;
        }
        if let Some(index) = self.slots.iter().position(|s| s.owner == owner) {
            return Some(index);
        }
        let index = self
            .slots
            .iter()
            .position(|s| s.owner == 0 || !active.contains(&s.owner))?;
        self.slots[index].reset(owner);
        Some(index)
    }

    pub(super) fn add(
        &mut self,
        owner: u32,
        active: &[u32],
        frame: Stereo,
        grains: Stereo,
        weight: f32,
    ) {
        if weight <= 0.0 {
            return;
        }
        if let Some(index) = self.ensure(owner, active) {
            let slot = &mut self.slots[index];
            for (sum, value) in slot.sum.iter_mut().zip(frame) {
                *sum += value * weight;
            }
            for (sum, value) in slot.feedback_sum.iter_mut().zip(grains) {
                *sum += value * weight;
            }
            slot.weight += weight;
        }
    }

    pub(super) fn finish_frame(&mut self, time: u64, frozen: bool, feedback: f32, norm: f32) {
        if !frozen {
            for slot in &mut self.slots {
                slot.push(time);
                slot.feed(feedback, norm);
            }
        }
    }

    pub(super) fn slot(&self, owner: u32) -> Option<usize> {
        self.slots.iter().position(|s| s.owner == owner)
    }

    pub(super) fn read(
        &self,
        slot: usize,
        position: f64,
        range: &std::ops::Range<usize>,
        epoch: u64,
        rate: f64,
        antialiasing: bool,
    ) -> Stereo {
        let Some(history) = self.slots.get(slot) else {
            return [0.0; 2];
        };
        let start = epoch as i64 + range.start as i64;
        let length = range.len() as i64;
        if length <= 0 {
            return [0.0; 2];
        }
        super::sample::read_compact(position + epoch as f64, rate, antialiasing, |time| {
            history.frame(start + (time - start).rem_euclid(length))
        })
    }
}

// IEEE binary16 retains a wide dynamic range at half the size of f32 and is
// converted without allocation or platform-specific floating-point support.
fn encode(value: f32) -> u16 {
    let value = if value.is_finite() {
        value.clamp(-16.0, 16.0)
    } else {
        0.0
    };
    let bits = value.to_bits();
    let sign = ((bits >> 16) & 0x8000) as u16;
    let exp = ((bits >> 23) & 0xff) as i32 - 127 + 15;
    let mantissa = bits & 0x7f_ffff;
    if exp <= 0 {
        if exp < -10 {
            return sign;
        }
        let shift = (14 - exp) as u32;
        return sign | (((mantissa | 0x80_0000) + (1 << (shift - 1))) >> shift) as u16;
    }
    if exp >= 31 {
        return sign | 0x7bff;
    }
    let rounded = (mantissa + 0x1000) >> 13;
    sign | (((exp as u32) << 10) + rounded) as u16
}

fn decode(value: u16) -> f32 {
    let sign = ((value as u32) & 0x8000) << 16;
    let exponent = (value >> 10) & 31;
    let mantissa = (value & 1023) as u32;
    if exponent == 0 {
        let x = mantissa as f32 * (1.0 / 16_777_216.0);
        return if sign != 0 { -x } else { x };
    }
    f32::from_bits(sign | (((exponent as u32) + 112) << 23) | (mantissa << 13))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compact_histories_isolate_parties_and_never_replay_reassigned_pcm() {
        let mut h = SequenceHistories::new(8_000, 2, None);
        h.begin_frame();
        h.add(1, &[1, 2], [0.25, -0.5], [0.0; 2], 1.0);
        h.add(2, &[1, 2], [-0.75, 1.0], [0.0; 2], 1.0);
        h.finish_frame(10, false, 0.0, 1.0);
        assert_eq!(h.slots[h.slot(1).unwrap()].frame(10), [0.25, -0.5]);
        assert_eq!(h.slots[h.slot(2).unwrap()].frame(10), [-0.75, 1.0]);
        let slot = h.ensure(3, &[2, 3]).unwrap();
        assert_eq!(h.slots[slot].frame(10), [0.0; 2]);
        h.begin_frame();
        h.add(3, &[2, 3], [0.125; 2], [0.0; 2], 1.0);
        h.finish_frame(12, false, 0.0, 1.0);
        assert_eq!(h.slots[slot].frame(12), [0.125; 2]);
        assert_eq!(h.slots[slot].frame(11), [0.0; 2]);
    }
}
