//! Main 2.10 services the original parameter editors call, as plain f32 math:
//! the parameter curve (context[21]), tempo-sync tables (context[23]/[24])
//! and the float setter's linear ramp (context[8]).

/// Rate the original DSP ran at. Delay lengths in samples, ROM filter
/// coefficients and per-sample increments recovered from the firmware assume it.
pub const ORIGINAL_RATE: f32 = 44_100.0;

/// Main c00ca380 (`context[21]`): `A4`=maximum, `B4`=minimum, `A6`=count,
/// `A8`=selected, `B6`=shape. Normalizes `selected / (count - 1)` and applies
/// 0=linear, 1=square, 2=two slopes, 3=sqrt, 4=atan, 5=power 1.41, 6=cube.
pub fn curve(maximum: f32, minimum: f32, count: u32, selected: f32, shape: u8) -> f32 {
    let difference = maximum - minimum;
    let t = selected / count.saturating_sub(1).max(1) as f32;
    let shaped = match shape {
        0 => t,
        1 => t * t,
        2 => {
            if t > 0.5 {
                0.65 + (t - 0.5) * 0.7
            } else {
                1.3 * t
            }
        }
        3 => t.max(0.0).sqrt(),
        4 => t.atan() * 1.273_239_5,
        5 => t.max(0.0).powf(1.41),
        6 => t * t * t,
        _ => return 0.0,
    };
    minimum + difference * shaped
}

/// Main c00e9a90 delay-time divisions (`scale, numerator, denominator`); a
/// quarter note is scale 100. Index 0 is a sixteenth note, 6 a quarter note,
/// 8.. are 2..64 quarter notes.
pub const DELAY_NOTES: [(u16, u16, u16); 71] = {
    let mut table = [(100, 1, 1); 71];
    table[0] = (25, 1, 1);
    table[1] = (50, 2, 3);
    table[2] = (25, 3, 2);
    table[3] = (50, 1, 1);
    table[4] = (100, 2, 3);
    table[5] = (50, 3, 2);
    table[6] = (100, 1, 1);
    table[7] = (100, 3, 2);
    let mut index = 8;
    while index < 71 {
        table[index] = (100, index as u16 - 6, 1);
        index += 1;
    }
    table
};

pub const DELAY_NOTE_LABELS: [&str; 71] = [
    "1/16", "1/8T", "1/16.", "1/8", "1/4T", "1/8.", "1/4", "1/4.", "1/4×2", "1/4×3", "1/4×4",
    "1/4×5", "1/4×6", "1/4×7", "1/4×8", "1/4×9", "1/4×10", "1/4×11", "1/4×12", "1/4×13", "1/4×14",
    "1/4×15", "1/4×16", "1/4×17", "1/4×18", "1/4×19", "1/4×20", "1/4×21", "1/4×22", "1/4×23",
    "1/4×24", "1/4×25", "1/4×26", "1/4×27", "1/4×28", "1/4×29", "1/4×30", "1/4×31", "1/4×32",
    "1/4×33", "1/4×34", "1/4×35", "1/4×36", "1/4×37", "1/4×38", "1/4×39", "1/4×40", "1/4×41",
    "1/4×42", "1/4×43", "1/4×44", "1/4×45", "1/4×46", "1/4×47", "1/4×48", "1/4×49", "1/4×50",
    "1/4×51", "1/4×52", "1/4×53", "1/4×54", "1/4×55", "1/4×56", "1/4×57", "1/4×58", "1/4×59",
    "1/4×60", "1/4×61", "1/4×62", "1/4×63", "1/4×64",
];

/// Main c00ead60 LFO divisions; a quarter note is scale 1000. Index 0 is a
/// 32nd note, 7 a quarter note, 9.. are 2..20 quarter notes.
pub const RATE_NOTES: [(u16, u16, u16); 28] = {
    let mut table = [(1000, 1, 1); 28];
    table[0] = (125, 1, 1);
    table[1] = (250, 1, 1);
    table[2] = (500, 2, 3);
    table[3] = (250, 3, 2);
    table[4] = (500, 1, 1);
    table[5] = (1000, 2, 3);
    table[6] = (500, 3, 2);
    table[7] = (1000, 1, 1);
    table[8] = (1000, 3, 2);
    let mut index = 9;
    while index < 28 {
        table[index] = (1000, index as u16 - 7, 1);
        index += 1;
    }
    table
};

pub const RATE_NOTE_LABELS: [&str; 28] = [
    "1/32", "1/16", "1/8T", "1/16.", "1/8", "1/4T", "1/8.", "1/4", "1/4.", "1/4×2", "1/4×3",
    "1/4×4", "1/4×5", "1/4×6", "1/4×7", "1/4×8", "1/4×9", "1/4×10", "1/4×11", "1/4×12", "1/4×13",
    "1/4×14", "1/4×15", "1/4×16", "1/4×17", "1/4×18", "1/4×19", "1/4×20",
];

/// Main c00d7bc0 (`context[24]`): a synced delay in milliseconds. Like the
/// original, a duration longer than `maximum_ms` is halved until it fits.
pub fn delay_note_ms(index: usize, bpm: f32, maximum_ms: f32) -> f32 {
    let (scale, numerator, denominator) = DELAY_NOTES[index.min(DELAY_NOTES.len() - 1)];
    let quarter = 60_000.0 / bpm.clamp(20.0, 400.0);
    let mut ms =
        quarter * f32::from(scale) * f32::from(numerator) / (100.0 * f32::from(denominator));
    while ms > maximum_ms && ms > 0.0 {
        ms *= 0.5;
    }
    ms
}

/// Main c00d7c60 (`context[23]`): LFO frequency in Hz for a synced rate.
pub fn rate_note_hz(index: usize, bpm: f32) -> f32 {
    let (scale, numerator, denominator) = RATE_NOTES[index.min(RATE_NOTES.len() - 1)];
    let beats = f32::from(scale) * f32::from(numerator) / (1000.0 * f32::from(denominator));
    bpm.clamp(20.0, 400.0) / (60.0 * beats)
}

/// A delay-time parameter with a tempo tail: `0..=threshold` is `value + 1` ms,
/// above it the tempo notes, limited to `maximum_ms`.
pub fn delay_time_ms(value: f32, threshold: u16, bpm: f32, maximum_ms: f32) -> f32 {
    let value = value.max(0.0);
    if value <= f32::from(threshold) {
        (value + 1.0).min(maximum_ms)
    } else {
        let index = (value.round() as usize).saturating_sub(usize::from(threshold) + 1);
        delay_note_ms(index, bpm, maximum_ms)
    }
}

/// True when a parameter with a tempo tail selects a note.
pub fn is_synced(value: f32, threshold: u16) -> bool {
    value.round() > f32::from(threshold)
}

/// The note index of a synced value (`value - threshold - 1`).
pub fn sync_index(value: f32, threshold: u16) -> usize {
    (value.round().max(0.0) as usize).saturating_sub(usize::from(threshold) + 1)
}

/// The float setter of Main (`context[8]`: `A4`=target, `B4`=value, `A6`=step):
/// a coefficient moves linearly by `step` per 44.1 kHz sample until it reaches
/// the requested value. `step` is rescaled to the instance rate.
#[derive(Debug, Clone, Copy)]
pub struct Ramp {
    pub current: f32,
    pub target: f32,
    step: f32,
}

impl Ramp {
    pub fn new(value: f32, step_at_44k: f32, sample_rate: f32) -> Self {
        Self {
            current: value,
            target: value,
            step: step_at_44k * ORIGINAL_RATE / sample_rate,
        }
    }
    pub fn set(&mut self, target: f32) {
        self.target = target;
    }
    pub fn jump(&mut self, value: f32) {
        self.current = value;
        self.target = value;
    }
    #[inline]
    pub fn tick(&mut self) -> f32 {
        let difference = self.target - self.current;
        if difference.abs() <= self.step * 1.1 {
            self.current = self.target;
        } else if difference > 0.0 {
            self.current += self.step;
        } else {
            self.current -= self.step;
        }
        self.current
    }
}

/// The common editor step 0x3a77c5f5.
pub const DEFAULT_STEP: f32 = f32::from_bits(0x3a77_c5f5);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tempo_tables_follow_the_main_divisions() {
        assert!((delay_note_ms(6, 120.0, 4000.0) - 500.0).abs() < 1e-3);
        assert!((delay_note_ms(0, 120.0, 4000.0) - 125.0).abs() < 1e-3);
        assert!((delay_note_ms(22, 120.0, 4000.0) - 4000.0).abs() < 1e-2);
        assert!((delay_note_ms(70, 120.0, 4000.0) - 4000.0).abs() < 1e-2);
        assert!((delay_note_ms(9, 120.0, 1000.0) - 750.0).abs() < 1e-2);
        assert!((rate_note_hz(7, 120.0) - 2.0).abs() < 1e-4);
        assert!((rate_note_hz(0, 120.0) - 16.0).abs() < 1e-3);
        assert!((delay_time_ms(3999.0, 3999, 120.0, 4000.0) - 4000.0).abs() < 1e-3);
        assert!((curve(1.5, 0.0, 151, 100.0, 0) - 1.0).abs() < 1e-6);
    }
}
