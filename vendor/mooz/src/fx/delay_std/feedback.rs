//! The audio callback shared bit-for-bit by DELAY.ZDL and ANLGDLY.ZDL
//! (`Fx_DLY_Delay` / `Fx_DLY_AnalogDly`, 7800..7c40, recovered as
//! `feedback_delay.rs`) together with their editors, which differ only in
//! the ROM image (input/feedback filters, HiDMP table) and the F.B maximum.
//!
//! Callback, per frame (coefficient indices of the original in brackets):
//! the stereo half-sum passes a one-pole [12,13] and a biquad [14..18]; it is
//! mixed with the previous tone output (feedback [8], input [7]), filtered by a
//! biquad [19..23] and written to the ring. Two integer taps [29,30] are
//! weighted by their gates [5,6]; the left output takes the first tap, the
//! right one crossfades to the second tap with [11] (ping-pong). Wet [9], dry
//! [10] and Level [2] form the output; the tone biquad [24..28] (HiDMP) filters
//! the first tap into the next feedback.
//!
//! Editors decoded from the disassembly: Time `ms·44.1` samples (sync notes
//! limited to 4000 ms); P-P halves the second tap and ramps [11] with step
//! `0x3af7c5ec`; on a live Time/P-P edit the taps fade out in 16 ms
//! (`tapmuteMute`), the offsets change and `TapOpen` reopens the first tap
//! after `ms + 1` ms, `ppTapOpen` the second after `ms/2 + 1` ms. F.B is linear
//! up to 0.95 (Delay) or 0.65 (AnalogDly); Mix is `(v/80)²` up to 80 and
//! `1 + ((v-80)/19)²` above (Main's square curve with count 20); Level `v/100`;
//! HiDMP copies one of eleven ROM biquads. ROM biquads are re-derived for the
//! host rate. The Main volume coefficient [4] and on/off [0] are taken as 1.
use super::common::{self, Gate};
use crate::{
    Effect, Values,
    dsp::{Biquad, Coefficients, DelayLine, flush},
    zoom::{self, Ramp},
};

/// Time parameter: 0..=3999 are 1..4000 ms, above are tempo notes.
pub const TIME_THRESHOLD: u16 = 3999;
const MAX_MS: f32 = 4000.0;

/// The model-specific part of the ROM image and editors.
pub struct Voicing {
    /// Input one-pole [12,13], input biquad [14..18] and feedback biquad
    /// [19..23]; `None` where the ROM leaves them as identities (Delay).
    pub analog: Option<AnalogRom>,
    /// HiDMP 0..10: the tone biquad [24..28].
    pub hidmp: &'static [[u32; 5]; 11],
    /// Main curve maximum of the F.B editor.
    pub feedback_max: f32,
}

pub struct AnalogRom {
    /// `first = first·[13] + mono·[12]`.
    pub pole: u32,
    pub input: [u32; 5],
    pub feedback: [u32; 5],
}

struct AnalogStages {
    pole: f32,
    first: f32,
    input: Biquad,
    feedback: Biquad,
}

pub struct FeedbackDelay {
    sample_rate: f32,
    line: DelayLine,
    analog: Option<AnalogStages>,
    hidmp: [Coefficients; 11],
    tone: Biquad,
    feedback_max: f32,
    feedback: Ramp,
    mix: Ramp,
    level: Ramp,
    pingpong: Ramp,
    gates: [Gate; 2],
    offsets: [usize; 2],
    pending: [usize; 2],
    reopen: [Option<u32>; 2],
    muting: bool,
    feedback_sample: f32,
    time_ms: f32,
    pingpong_on: bool,
    hidmp_index: usize,
    started: bool,
}

impl FeedbackDelay {
    pub fn new(sample_rate: f32, voicing: &Voicing) -> Self {
        let analog = voicing.analog.as_ref().map(|rom| AnalogStages {
            pole: common::pole_at(common::f(rom.pole), sample_rate),
            first: 0.0,
            input: Biquad::new(common::rom_biquad(rom.input, sample_rate)),
            feedback: Biquad::new(common::rom_biquad(rom.feedback, sample_rate)),
        });
        let hidmp = voicing
            .hidmp
            .map(|words| common::rom_biquad(words, sample_rate));
        Self {
            sample_rate,
            line: common::line_for_ms(MAX_MS, 2, sample_rate),
            analog,
            hidmp,
            tone: Biquad::new(hidmp[10]),
            feedback_max: voicing.feedback_max,
            feedback: common::setter(0.0, sample_rate),
            mix: common::setter(0.0, sample_rate),
            level: common::setter(1.0, sample_rate),
            // pp_edit: `context[8]` with step 0x3af7c5ec.
            pingpong: Ramp::new(0.0, common::f(0x3af7_c5ec), sample_rate),
            gates: [Gate::new(sample_rate, true), Gate::new(sample_rate, false)],
            offsets: [0; 2],
            pending: [0; 2],
            reopen: [None; 2],
            muting: false,
            feedback_sample: 0.0,
            time_ms: 0.0,
            pingpong_on: false,
            hidmp_index: 10,
            started: false,
        }
    }

    /// Wet gain [9] of the Mix editor.
    pub fn mix_gain(mix: f32) -> f32 {
        let mix = common::param(mix, 100.0);
        if mix <= 80.0 {
            (mix / 80.0) * (mix / 80.0)
        } else {
            let t = (mix - 80.0) / 19.0;
            1.0 + t * t
        }
    }

    fn apply_offsets(&mut self) {
        self.offsets = self.pending;
        self.muting = false;
        for (gate, reopen) in self.gates.iter_mut().zip(self.reopen) {
            match reopen {
                Some(samples) => gate.open_after(samples),
                None => gate.set_open(false),
            }
        }
    }

    #[inline]
    fn ramps_settle(&mut self) -> (f32, f32, f32, f32) {
        (
            self.feedback.tick(),
            self.mix.tick(),
            self.level.tick(),
            self.pingpong.tick(),
        )
    }
}

impl Effect for FeedbackDelay {
    fn set(&mut self, values: &Values, bpm: f32) {
        let feedback = self.feedback_max * common::param(values[1], 100.0) / 100.0;
        let mix = Self::mix_gain(values[2]);
        let level = common::param(values[5], 150.0) / 100.0;
        let hidmp = common::index(values[3], 10);
        let pingpong = common::param(values[4], 1.0) >= 0.5;
        let time = common::param(values[0], 4022.0).round();
        let ms = zoom::delay_time_ms(time, TIME_THRESHOLD, bpm, MAX_MS);

        if hidmp != self.hidmp_index {
            self.hidmp_index = hidmp;
            self.tone.coefficients = self.hidmp[hidmp];
        }
        self.feedback.set(feedback);
        self.mix.set(mix);
        self.level.set(level);
        self.pingpong.set(if pingpong { 1.0 } else { 0.0 });

        let first = common::tap_samples(ms, self.sample_rate, &self.line);
        let second = if pingpong { first / 2 } else { first };
        let changed = !self.started || ms != self.time_ms || pingpong != self.pingpong_on;
        if changed {
            self.time_ms = ms;
            self.pingpong_on = pingpong;
            self.pending = [first, second];
            let open = |ms: f32| common::ms_to_samples(ms, self.sample_rate) as u32;
            self.reopen = [
                Some(open(ms + 1.0)),
                pingpong.then(|| open((ms * 0.5).floor() + 1.0)),
            ];
            if self.started {
                // A live edit: tapmuteMute, new offsets, timers reopen the taps.
                self.muting = true;
                for gate in &mut self.gates {
                    gate.mute();
                }
            } else {
                // Initialisation writes everything directly.
                self.offsets = self.pending;
                self.gates[0].set_open(true);
                self.gates[1].set_open(pingpong);
                self.feedback.jump(feedback);
                self.mix.jump(mix);
                self.level.jump(level);
                self.pingpong.jump(if pingpong { 1.0 } else { 0.0 });
                self.started = true;
            }
        }
    }

    #[inline]
    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let (feedback, mix, level, pingpong) = self.ramps_settle();
        let mono = 0.5 * (left + right);
        let input = match &mut self.analog {
            Some(stages) => {
                stages.first = flush(mono + (stages.first - mono) * stages.pole);
                stages.input.process(stages.first)
            }
            None => mono,
        };
        let fed = self.feedback_sample * feedback + input;
        let stored = match &mut self.analog {
            Some(stages) => stages.feedback.process(fed),
            None => fed,
        };
        self.line.write(stored);
        if self.muting && self.gates[0].closed() && self.gates[1].closed() {
            self.apply_offsets();
        }
        let first = self.gates[0].tick() * self.line.read(self.offsets[0]);
        let second = self.gates[1].tick() * self.line.read(self.offsets[1]);
        self.feedback_sample = flush(self.tone.process(first));
        let crossed = first + (second - first) * pingpong;
        [
            level * (mix * first + left),
            level * (mix * crossed + right),
        ]
    }

    fn clear(&mut self) {
        self.line.clear();
        if let Some(stages) = &mut self.analog {
            stages.first = 0.0;
            stages.input.clear();
            stages.feedback.clear();
        }
        self.tone.clear();
        self.feedback_sample = 0.0;
        if self.muting {
            self.apply_offsets();
        }
        for (gate, reopen) in self.gates.iter_mut().zip(self.reopen) {
            gate.set_open(reopen.is_some());
        }
    }
}
