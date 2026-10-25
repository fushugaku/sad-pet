//! Pieces shared by SpaceHole (SPC_HOLE.ZDL) and TremoloRv (TREM_REV.ZDL): the
//! modules' `TBL_TO_VAL` table interpolation, `EQ_Calc_2Pole_Shelv`, the
//! parabolic LFO both callbacks bend a Q31 triangle with, and the tap-mute
//! fader their editors put around delay changes.
use super::fast::DelayLine;
use crate::dsp::Coefficients;

/// `TBL_TO_VAL(count, value, table, len)`: linear interpolation of a float
/// table at `value * (len - 1) / (count - 1)`.
pub fn table(table: &[f32], count: u32, value: f32) -> f32 {
    let last = table.len().saturating_sub(1);
    if last == 0 {
        return table.first().copied().unwrap_or(0.0);
    }
    let position =
        (value * last as f32 / count.saturating_sub(1).max(1) as f32).clamp(0.0, last as f32);
    let index = (position as usize).min(last - 1);
    let fraction = position - index as f32;
    table[index] + (table[index + 1] - table[index]) * fraction
}

/// `EQ_Calc_2Pole_Shelv(out, hz, gain_db, slope, type)`: the RBJ shelf with
/// shelf slope `S` (`alpha = sin(w0)/2 * sqrt((A + 1/A)(1/S - 1) + 2)`);
/// type 12 is the low shelf, 11 the high shelf. The firmware designs it at
/// 44.1 kHz; here it is designed at the instance rate.
pub fn shelf(sample_rate: f32, hz: f32, gain_db: f32, slope: f32, high: bool) -> Coefficients {
    let a = 10f32.powf(gain_db / 40.0);
    let x = ((a + 1.0 / a) * (1.0 / slope - 1.0) + 2.0).max(1e-4);
    let q = 1.0 / x.sqrt();
    if high {
        Coefficients::high_shelf(sample_rate, hz, q, gain_db)
    } else {
        Coefficients::low_shelf(sample_rate, hz, q, gain_db)
    }
}

/// `|p|` of the Q31 phase word `p` (one cycle = 2^32), with the phase given in
/// turns: 0 at turn 0, 1 at half a turn.
#[inline]
pub fn triangle(turns: f32) -> f32 {
    let u = turns - turns.floor();
    1.0 - (2.0 * u - 1.0).abs()
}

/// The callbacks' LFO: the triangle bent by `0.5 + 2 t (1 - |t|)`,
/// `t = triangle - 0.5`, a sine-like wave from 0 to 1.
#[inline]
pub fn parabolic(turns: f32) -> f32 {
    let t = triangle(turns) - 0.5;
    0.5 + 2.0 * t * (1.0 - t.abs())
}

/// Advance a phase in turns, wrapping to [0, 1).
#[inline]
pub fn advance(phase: &mut f32, increment: f32) {
    *phase += increment;
    if *phase >= 1.0 {
        *phase -= phase.floor();
    }
}

/// A delay in 44.1 kHz samples at the instance rate, at least one sample.
#[inline]
pub fn samples(at_44k: f32, sample_rate: f32) -> usize {
    (crate::dsp::scale_samples(at_44k, sample_rate).round() as usize).max(1)
}

/// The firmware's ring delay `R - W = delay` (>= 1 frame), read before this
/// frame's write.
#[inline]
pub fn tap(line: &DelayLine, delay: usize) -> f32 {
    line.read(delay - 1)
}

/// Fractional version of [`tap`].
#[inline]
pub fn tap_linear(line: &DelayLine, delay: f32) -> f32 {
    line.read_linear(delay - 1.0)
}

/// Main's `context[7]` setter: move linearly to a new value over a fixed
/// duration (given in 44.1 kHz samples, e.g. 705.6 = 16 ms).
#[derive(Debug, Clone, Copy)]
pub struct Glide {
    pub current: f32,
    target: f32,
    step: f32,
    samples: f32,
}

impl Glide {
    pub fn new(value: f32, duration_at_44k: f32, sample_rate: f32) -> Self {
        Self {
            current: value,
            target: value,
            step: 0.0,
            samples: (duration_at_44k * sample_rate / crate::zoom::ORIGINAL_RATE).max(1.0),
        }
    }

    pub fn set(&mut self, target: f32) {
        if target != self.target {
            self.target = target;
            self.step = ((target - self.current).abs() / self.samples).max(target.abs() * 1e-6);
        }
    }

    pub fn jump(&mut self, value: f32) {
        self.current = value;
        self.target = value;
    }

    #[inline(always)]
    pub fn tick(&mut self) -> f32 {
        if self.current != self.target {
            let difference = self.target - self.current;
            if difference.abs() <= self.step {
                self.current = self.target;
            } else {
                self.current += self.step.copysign(difference);
            }
        }
        self.current
    }
}

/// The editors' tap mute: ramp a gain to zero, let the caller switch delay
/// lengths while it is silent (optionally after holding silence), then ramp
/// back to one. Requests during a cycle are folded into the next switch.
#[derive(Debug, Clone, Copy)]
pub struct TapMute {
    pub gain: f32,
    close_step: f32,
    open_step: f32,
    hold: u32,
    holding: u32,
    hold_next: bool,
    closing: bool,
    pending: bool,
}

impl TapMute {
    /// Ramp lengths in 44.1 kHz samples (the editors' context[7] durations).
    pub fn new(close_at_44k: f32, open_at_44k: f32, sample_rate: f32) -> Self {
        let rate = sample_rate / crate::zoom::ORIGINAL_RATE;
        Self {
            gain: 1.0,
            close_step: 1.0 / (close_at_44k * rate).max(1.0),
            open_step: 1.0 / (open_at_44k * rate).max(1.0),
            hold: 0,
            holding: 0,
            hold_next: false,
            closing: false,
            pending: false,
        }
    }

    /// Silence to hold before switching, for requests made with `hold = true`.
    pub fn set_hold_samples(&mut self, samples: u32) {
        self.hold = samples;
    }

    pub fn request(&mut self, hold: bool) {
        self.pending = true;
        self.hold_next |= hold;
    }

    pub fn busy(&self) -> bool {
        self.pending || self.closing || self.holding > 0 || self.gain < 1.0
    }

    /// Finish any cycle at once (used by `clear`).
    pub fn reset(&mut self) {
        self.gain = 1.0;
        self.closing = false;
        self.pending = false;
        self.holding = 0;
        self.hold_next = false;
    }

    /// One sample. Returns true when the caller should apply its new delays.
    #[inline(always)]
    pub fn tick(&mut self) -> bool {
        if self.holding > 0 {
            self.holding -= 1;
            return self.holding == 0;
        }
        if self.pending && !self.closing {
            self.closing = true;
        }
        if self.closing {
            self.gain -= self.close_step;
            if self.gain <= 0.0 {
                self.gain = 0.0;
                self.closing = false;
                self.pending = false;
                if self.hold_next && self.hold > 0 {
                    self.hold_next = false;
                    self.holding = self.hold;
                    return false;
                }
                self.hold_next = false;
                return true;
            }
        } else if self.gain < 1.0 {
            self.gain = (self.gain + self.open_step).min(1.0);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_interpolates_like_tbl_to_val() {
        let t = [0.0, 0.2, 0.4, 0.6, 0.8, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
        assert_eq!(table(&t, 101, 0.0), 0.0);
        assert!((table(&t, 101, 40.0) - 0.8).abs() < 1e-6);
        assert!((table(&t, 101, 45.0) - 0.9).abs() < 1e-6);
        assert_eq!(table(&t, 101, 100.0), 1.0);
        // count 100 spreads 0..99 over the same table
        assert!((table(&t, 100, 99.0) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn parabolic_lfo_spans_zero_to_one() {
        assert!(parabolic(0.0).abs() < 1e-6);
        assert!((parabolic(0.5) - 1.0).abs() < 1e-6);
        assert!((parabolic(0.25) - 0.5).abs() < 1e-6);
        assert!((triangle(0.75) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn shelves_reach_their_gain() {
        let sr = 48_000.0;
        let low = shelf(sr, 350.0, 12.0, 1.5, false);
        assert!((crate::dsp::gain_to_db(low.magnitude(sr, 20.0)) - 12.0).abs() < 0.3);
        assert!(crate::dsp::gain_to_db(low.magnitude(sr, 10_000.0)).abs() < 0.3);
        let high = shelf(sr, 3200.0, -19.0, 1.6, true);
        assert!((crate::dsp::gain_to_db(high.magnitude(sr, 20_000.0)) + 19.0).abs() < 1.0);
        assert!(crate::dsp::gain_to_db(high.magnitude(sr, 50.0)).abs() < 0.3);
    }

    #[test]
    fn tap_mute_switches_while_silent() {
        let mut mute = TapMute::new(10.0, 10.0, 44_100.0);
        mute.request(false);
        let mut switched_at = None;
        for i in 0..40 {
            if mute.tick() {
                assert_eq!(mute.gain, 0.0);
                switched_at = Some(i);
            }
        }
        assert_eq!(switched_at, Some(9));
        assert_eq!(mute.gain, 1.0);
        mute.set_hold_samples(5);
        mute.request(true);
        let switches = (0..40).filter(|_| mute.tick()).count();
        assert_eq!(switches, 1);
    }
}
