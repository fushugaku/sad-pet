//! Spring63 (SPRING63.ZDL): a 1963-style spring reverb tank with "drip".
//!
//! Follows the recovered `Fx_REV_Spring63` callback (zoom-ms70-cdr
//! `spring63/samples.rs`) slot by slot:
//!
//! - Mono input `(L + R) / 2` -> first-order high-pass (C39..43, ~104 Hz) ->
//!   peaking boost (C44..48, +6 dB at 3 kHz) -> DWELL gain C49.
//! - Excitation detector: `|x| * 12` clipped to 1 with instant attack and a
//!   0.9995/sample release (S41, C150/C151), smoothed twice (C153..156) into the
//!   tension signal S55. A fast/slow follower pair (C160..164) restarts a
//!   48.8 ms window (the integer phase S45 counting by C165) on every attack.
//! - Drip: two short combs (feedback C52/C53 = 0.93/0.92) whose delay is
//!   `min(tension, 1) * 71.68` samples (Q31 C142 = 0.28, shift C143 = 23,
//!   fraction C147), the second 2 samples longer. Their mix (C55/C56) is gated by
//!   the smoothed fall of the tension over 452 samples (C152, C157/C158) while
//!   the window is open, then high-passed (C57..61) and low-passed (C62..66).
//! - Tank: four series all-pass diffusers (1631/1852/1340/1499 samples,
//!   g = 0.57, input gain C67 = 0.71), then four recirculating lines (3567,
//!   3253, 3027, 2317 samples) with split low/high damping (one-pole C74,
//!   low gains C85..88, high gains C81..84) and two first-order shelves
//!   (C75..77, C78..80), each tapped near its write point (1013/44/1016/44).
//!   Two more lines (4263 and 12789 samples, feedback C103 = 0.789 and
//!   C104 = 0.492 through the same shelves) are tapped 44 samples after the
//!   write and band-limited (300 Hz high-pass, 4 kHz low-pass, C105..114).
//! - Wet sum: 0.62 drip + 0.72 * (taps through the resonant 500 Hz high-pass
//!   C93..97 and the +12 dB high shelf C98..102) + line taps (C117/C118) ->
//!   500 Hz high-pass (C120..124) -> TONE low-pass (C125..129) -> wet gain C140.
//! - Dry stays stereo: per channel a 20 Hz low shelf (C130..134) and a 3 kHz
//!   high shelf (C135..139), dry gain C141; output `C2 * (dry + wet)`.
//!
//! Editors decoded from the disassembly: init/onf_aft (the fixed feedback and
//! tank gains above), `dwell_edit` (11-point dB table, 0 = off, 16 ms ramp),
//! `tone_edit` (11-point frequency table -> Main designer type 0, first-order
//! low-pass), `mixer_edit` (tables -> type 9 low shelf at 20 Hz, type 8 high
//! shelf at 3 kHz, wet dB table, dry 0 dB table, 16 ms ramps) and `level_edit`
//! (`LEVEL / 100`, float-setter ramp). The Main designer (c00bc2e0) works at
//! 44.1 kHz; its first-order sections and the ROM filters are moved to the host
//! rate by inverting their bilinear transform, delays and time constants are
//! rescaled.
//!
//! Approximations: the drip combs never read below one sample (at rest the
//! original's zero-length read aliases the whole 176400-word ring); Main's
//! on/off crossfade C0 and the shared control C4 are taken as 1.
use super::fast::{Bank, Biquad, DelayLine, Ramp, flush};
use crate::{Effect, Values, catalog::*, dsp::Coefficients, zoom::ORIGINAL_RATE};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_spring63",
    name: "Spring63",
    category: Category::Reverb,
    zdl: "SPRING63.ZDL",
    description: "Vintage 1963 spring reverb tank with drip",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "DWELL",
            max: 100,
            default: 35,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "MIXER",
            max: 100,
            default: 51,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "TONE",
            max: 100,
            default: 58,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "LEVEL",
            max: 150,
            default: 100,
            display: Display::INT,
        },
    ],
};

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(Spring63::new(sample_rate))
}

/// `dwell_edit` table at 0x800007f8: input drive in dB (DWELL 0 mutes).
const DWELL_DB: [f32; 11] = [
    -74.0, -6.0, -4.67, -3.33, -2.0, -0.67, 0.67, 2.0, 3.33, 4.67, 6.0,
];
/// `mixer_edit` table at 0x80000828: dry 20 Hz low-shelf gain in dB.
const MIXER_LOW_DB: [f32; 11] = [
    0.0,
    -0.917_21,
    -2.624_278,
    -4.234_78,
    -5.756_346,
    -6.943_773,
    -7.916_591,
    -8.876_696,
    -9.733_126,
    -10.489_875,
    -11.019_744,
];
/// `mixer_edit` table at 0x80000858: dry 3 kHz high-shelf gain in dB.
const MIXER_HIGH_DB: [f32; 11] = [
    0.0, -0.997_559, -2.351_86, -3.369_201, -3.999_635, -4.493_518, -4.984_491, -5.265_801,
    -5.583_105, -5.897_078, -6.0,
];
/// `mixer_edit` table at 0x800008b8: wet gain in dB (the dry table at
/// 0x80000888 is all zeros, so the dry gain C141 stays 1).
const MIXER_WET_DB: [f32; 11] = [
    -9.039_265, -6.990_144, -4.104_982, -1.773_207, 0.161_694, 1.533_803, 2.640_441, 3.733_486,
    4.600_354, 5.452_609, 6.0,
];
/// `tone_edit` table at 0x800008e8: wet low-pass corner in Hz.
const TONE_HZ: [f32; 11] = [
    100.0, 319.0, 976.0, 2071.0, 3604.0, 5575.0, 7984.0, 10831.0, 14116.0, 17839.0, 22000.0,
];

/// TBL_TO_VAL(101, value, table, 11): linear interpolation over 0..=100.
fn table(points: &[f32; 11], value: f32) -> f32 {
    let position = value.clamp(0.0, 100.0) * 0.1;
    let index = (position as usize).min(9);
    let fraction = position - index as f32;
    points[index] + (points[index + 1] - points[index]) * fraction
}

fn db_gain(db: f32) -> f32 {
    10f32.powf(db / 20.0)
}

/// First-order section `y = b0 x + b1 x1 + c y1`, the shape of every
/// first-order ROM filter and Main designer result (`c` is the stored, already
/// negated feedback word).
#[derive(Debug, Clone, Copy, Default)]
struct FirstOrder {
    b0: f32,
    b1: f32,
    c: f32,
    x1: f32,
    y1: f32,
}

impl FirstOrder {
    /// A section designed for 44.1 kHz, moved to `sample_rate` keeping its
    /// analog prototype (inverse bilinear transform, then forward again).
    fn at_rate(b0: f32, b1: f32, c: f32, sample_rate: f32) -> Self {
        let (b0, b1, a1) = (f64::from(b0), f64::from(b1), -f64::from(c));
        let r = f64::from(sample_rate) / f64::from(ORIGINAL_RATE);
        let (n0, n1) = (b0 + b1, (b0 - b1) * r);
        let (d0, d1) = (1.0 + a1, (1.0 - a1) * r);
        let norm = d0 + d1;
        Self {
            b0: ((n0 + n1) / norm) as f32,
            b1: ((n0 - n1) / norm) as f32,
            c: (-(d0 - d1) / norm) as f32,
            x1: 0.0,
            y1: 0.0,
        }
    }

    fn set(&mut self, other: Self) {
        self.b0 = other.b0;
        self.b1 = other.b1;
        self.c = other.c;
    }

    #[inline(always)]
    fn process(&mut self, x: f32) -> f32 {
        let y = flush(self.b0 * x + self.b1 * self.x1 + self.c * self.y1);
        self.x1 = x;
        self.y1 = y;
        y
    }

    fn clear(&mut self) {
        self.x1 = 0.0;
        self.y1 = 0.0;
    }

    #[cfg(test)]
    fn magnitude(&self, sample_rate: f32, hz: f32) -> f32 {
        Coefficients {
            b0: self.b0,
            b1: self.b1,
            b2: 0.0,
            a1: -self.c,
            a2: 0.0,
        }
        .magnitude(sample_rate, hz)
    }
}

/// Two first-order sections in series.
#[inline(always)]
fn cascade(sections: &mut [FirstOrder; 2], x: f32) -> f32 {
    let y = sections[0].process(x);
    sections[1].process(y)
}

/// Main c00bc2e0 (the context[25] filter designer) at 44.1 kHz, the
/// first-order types Spring63 uses: 0 low-pass, 8 high shelf, 9 low shelf.
/// Returns `(b0, b1, c)`.
fn design(kind: u8, hz: f32, gain_db: f32) -> (f32, f32, f32) {
    let v = db_gain(gain_db);
    let tan = |hz: f32| (std::f32::consts::PI * hz / ORIGINAL_RATE).tan();
    match kind {
        0 => {
            let k = tan(hz);
            (k / (1.0 + k), k / (1.0 + k), (1.0 - k) / (1.0 + k))
        }
        8 => {
            let k = if gain_db < 0.0 { tan(hz * v) } else { tan(hz) };
            (
                (v + k) / (1.0 + k),
                (k - v) / (1.0 + k),
                (1.0 - k) / (1.0 + k),
            )
        }
        _ => {
            let k = if gain_db < 0.0 { tan(hz / v) } else { tan(hz) };
            (
                (1.0 + k * v) / (1.0 + k),
                (k * v - 1.0) / (1.0 + k),
                (1.0 - k) / (1.0 + k),
            )
        }
    }
}

/// A ROM biquad (`[b0, b1, b2, -a1, -a2]` words) moved to the host rate.
fn rom_biquad(words: [f32; 5], sample_rate: f32) -> Biquad {
    Biquad::new(
        Coefficients {
            b0: words[0],
            b1: words[1],
            b2: words[2],
            a1: -words[3],
            a2: -words[4],
        }
        .resample(ORIGINAL_RATE, sample_rate),
    )
}

/// Leaky integrator `y = p y + g x` with a 44.1 kHz pole, rescaled to keep its
/// time constant and steady-state gain `g / (1 - p)`.
#[derive(Debug, Clone, Copy)]
struct Leaky {
    pole: f32,
    gain: f32,
    state: f32,
}

impl Leaky {
    fn new(pole: f32, gain: f32, sample_rate: f32) -> Self {
        let scaled = f64::from(pole).powf(f64::from(ORIGINAL_RATE / sample_rate));
        let gain = f64::from(gain) / (1.0 - f64::from(pole)) * (1.0 - scaled);
        Self {
            pole: scaled as f32,
            gain: gain as f32,
            state: 0.0,
        }
    }
    #[inline(always)]
    fn process(&mut self, x: f32) -> f32 {
        self.state = flush(self.pole * self.state + self.gain * x);
        self.state
    }
}

/// Linear ramp to a target over a fixed time (Main context[7], c00cc94c).
#[derive(Debug, Clone, Copy)]
struct TimedRamp {
    current: f32,
    target: f32,
    step: f32,
    remaining: u32,
    length: u32,
}

impl TimedRamp {
    fn new(value: f32, samples_at_44k: f32, sample_rate: f32) -> Self {
        Self {
            current: value,
            target: value,
            step: 0.0,
            remaining: 0,
            length: (samples_at_44k * sample_rate / ORIGINAL_RATE)
                .round()
                .max(1.0) as u32,
        }
    }
    fn set(&mut self, target: f32) {
        if target != self.target {
            self.target = target;
            self.step = (target - self.current) / self.length as f32;
            self.remaining = self.length;
        }
    }
    fn jump(&mut self, value: f32) {
        self.current = value;
        self.target = value;
        self.remaining = 0;
    }
    #[inline(always)]
    fn tick(&mut self) -> f32 {
        if self.remaining > 0 {
            self.remaining -= 1;
            self.current = if self.remaining == 0 {
                self.target
            } else {
                self.current + self.step
            };
        }
        self.current
    }
}

/// A delay line with its tap distances (in samples, as `DelayLine::read`
/// arguments: the original distance minus one, since reads precede the write).
struct Line {
    buffer: DelayLine,
    feedback_tap: usize,
    output_tap: usize,
}

/// The tap distances of a line kept in a [`Bank`] region.
#[derive(Debug, Clone, Copy)]
struct Taps {
    feedback_tap: usize,
    output_tap: usize,
}

impl Taps {
    fn new(feedback_at_44k: f32, output_at_44k: f32, sample_rate: f32) -> Self {
        Self {
            feedback_tap: samples(feedback_at_44k, sample_rate),
            output_tap: samples(output_at_44k, sample_rate),
        }
    }
    /// The longest read, in [`Bank::new`] terms.
    fn longest(&self) -> usize {
        self.feedback_tap.max(self.output_tap) + 1
    }
}

impl Line {
    fn new(feedback_at_44k: f32, output_at_44k: f32, sample_rate: f32) -> Self {
        let feedback_tap = samples(feedback_at_44k, sample_rate);
        let output_tap = samples(output_at_44k, sample_rate);
        Self {
            buffer: DelayLine::new(feedback_tap.max(output_tap) + 1),
            feedback_tap,
            output_tap,
        }
    }
}

/// An original read distance (44.1 kHz samples) as a `DelayLine::read` argument.
fn samples(at_44k: f32, sample_rate: f32) -> usize {
    ((at_44k * sample_rate / ORIGINAL_RATE).round() as usize).max(1) - 1
}

/// A recirculating tank line with split damping and the two loop shelves.
struct TankLine {
    line: Taps,
    low_gain: f32,
    high_gain: f32,
    damping: f32,
    lowpass: f32,
    shelf1: FirstOrder,
    shelf2: FirstOrder,
}

/// Tank delays at 44.1 kHz: (recirculation, output tap) for lines 1-4.
const TANK: [(f32, f32); 4] = [
    (3567.0, 1013.0),
    (3253.0, 44.0),
    (3027.0, 1016.0),
    (2317.0, 44.0),
];
/// onf_aft C85..88 (low) and C81..84 (high) feedback gains.
const TANK_LOW: [f32; 4] = [0.833_323_7, 0.847_891_6, 0.858_684_5, 0.890_740_2];
const TANK_HIGH: [f32; 4] = [0.872_187_1, 0.883_599_3, 0.892_020_9, 0.916_881_4];
/// Diffuser delays at 44.1 kHz (C10..C13 minus C29..C32).
const DIFFUSERS: [f32; 4] = [1631.0, 1852.0, 1340.0, 1499.0];
/// Drip comb modulation depth: Q31 0.28 shifted right by 23 = 71.68 samples.
const DRIP_DEPTH: f32 = 0.28 * 256.0;

struct Spring63 {
    sample_rate: f32,
    last: Values,
    // Input conditioning and detector.
    input_highpass: FirstOrder,
    input_peak: Biquad,
    envelope: f32,
    release: f32,
    tension_1: Leaky,
    tension: Leaky,
    fast: Leaky,
    slow: Leaky,
    window: f32,
    window_step: f32,
    // Drip.
    tension_line: DelayLine,
    tension_tap: usize,
    comb_a: DelayLine,
    comb_b: DelayLine,
    comb_a_read: f32,
    comb_b_read: f32,
    comb_delay: f32,
    comb_offset: f32,
    depth: f32,
    fall: Leaky,
    drip_highpass: Biquad,
    drip_lowpass: FirstOrder,
    // Tank.
    diffusers: [Taps; 4],
    diffuser_lines: Bank<4>,
    tank: [TankLine; 4],
    tank_lines: Bank<4>,
    long_a: Line,
    long_b: Line,
    long_a_shelves: [FirstOrder; 2],
    long_b_shelves: [FirstOrder; 2],
    tap_a_filters: [FirstOrder; 2],
    tap_b_filters: [FirstOrder; 2],
    taps_highpass: Biquad,
    taps_shelf: FirstOrder,
    wet_highpass: FirstOrder,
    tone: FirstOrder,
    // Dry path, per channel: 20 Hz low shelf then 3 kHz high shelf.
    dry_low: [FirstOrder; 2],
    dry_high: [FirstOrder; 2],
    // Gains.
    dwell: TimedRamp,
    wet: TimedRamp,
    level: Ramp,
}

impl Spring63 {
    fn new(sample_rate: f32) -> Self {
        let first = |b0: f32, b1: f32, c: f32| FirstOrder::at_rate(b0, b1, c, sample_rate);
        let shelf1 = first(0.999_513, -0.957_678, 0.957_191);
        let shelf2 = first(0.959_578, -0.404_795, 0.445_217);
        let tap_highpass = first(0.979_073, -0.979_073, 0.958_145);
        let tap_lowpass = first(0.226_559, 0.226_559, 0.546_882);
        let tank: [TankLine; 4] = std::array::from_fn(|i| TankLine {
            line: Taps::new(TANK[i].0, TANK[i].1, sample_rate),
            low_gain: TANK_LOW[i],
            high_gain: TANK_HIGH[i],
            damping: 0.0,
            lowpass: 0.0,
            shelf1,
            shelf2,
        });
        let diffusers: [Taps; 4] =
            std::array::from_fn(|i| Taps::new(DIFFUSERS[i], 1.0, sample_rate));
        let bank = |taps: [Taps; 4]| {
            let longest = taps.iter().map(Taps::longest).max().unwrap_or(1);
            Bank::new([1; 4], longest)
        };
        let mut effect = Self {
            sample_rate,
            last: [f32::NAN; crate::MAX_PARAMS],
            input_highpass: first(0.992_646, -0.992_646, 0.985_291),
            input_peak: rom_biquad(
                [1.093_457, -1.649_163, 0.718_74, 1.649_163, -0.812_197],
                sample_rate,
            ),
            envelope: 0.0,
            release: 0.9995f32.powf(ORIGINAL_RATE / sample_rate),
            tension_1: Leaky::new(0.9, 0.123_027, sample_rate),
            tension: Leaky::new(0.95, 0.05, sample_rate),
            fast: Leaky::new(0.93, 0.07, sample_rate),
            slow: Leaky::new(0.995, 0.01, sample_rate),
            window: 1.0,
            window_step: 998_166.0 / 2_147_483_648.0 * ORIGINAL_RATE / sample_rate,
            tension_line: DelayLine::new(samples(452.0, sample_rate) + 1),
            tension_tap: samples(452.0, sample_rate),
            comb_a: DelayLine::new((80.0 * sample_rate / ORIGINAL_RATE) as usize + 4),
            comb_b: DelayLine::new((80.0 * sample_rate / ORIGINAL_RATE) as usize + 4),
            comb_a_read: 0.0,
            comb_b_read: 0.0,
            comb_delay: 0.0,
            comb_offset: 2.0 * sample_rate / ORIGINAL_RATE,
            depth: DRIP_DEPTH * sample_rate / ORIGINAL_RATE,
            fall: Leaky::new(0.95, 0.05, sample_rate),
            drip_highpass: rom_biquad(
                [0.877_103, -1.754_206, 0.877_103, 1.718_109, -0.790_303],
                sample_rate,
            ),
            drip_lowpass: first(0.555_976, 0.555_976, -0.111_952),
            diffuser_lines: bank(diffusers),
            diffusers,
            tank_lines: bank(tank.each_ref().map(|line| line.line)),
            tank,
            long_a: Line::new(4263.0, 44.0, sample_rate),
            long_b: Line::new(12789.0, 44.0, sample_rate),
            long_a_shelves: [shelf1, shelf2],
            long_b_shelves: [shelf1, shelf2],
            tap_a_filters: [tap_highpass, tap_lowpass],
            tap_b_filters: [tap_highpass, tap_lowpass],
            taps_highpass: rom_biquad(
                [0.493_421, -0.986_841, 0.493_421, 1.967_268, -0.970_746],
                sample_rate,
            ),
            taps_shelf: first(2.056_804, -0.765_814, -0.290_99),
            wet_highpass: first(0.965_592, -0.965_592, 0.931_184),
            tone: FirstOrder::default(),
            dry_low: [FirstOrder::default(); 2],
            dry_high: [FirstOrder::default(); 2],
            dwell: TimedRamp::new(0.0, 705.6, sample_rate),
            wet: TimedRamp::new(0.0, 705.6, sample_rate),
            level: Ramp::new(0.0, 9.451_8e-4, sample_rate),
        };
        let one_pole = 1.0 - (1.0 - 0.344_299f64).powf(f64::from(ORIGINAL_RATE / sample_rate));
        for line in &mut effect.tank {
            line.damping = one_pole as f32;
        }
        let defaults = crate::Model::Spring63.defaults();
        effect.set(&defaults, 120.0);
        effect.dwell.jump(effect.dwell.target);
        effect.wet.jump(effect.wet.target);
        effect.level.jump(effect.level.target);
        effect
    }

    fn dwell_gain(value: f32) -> f32 {
        if value <= 0.0 {
            0.0
        } else {
            db_gain(table(&DWELL_DB, value))
        }
    }
}

impl Effect for Spring63 {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let rate = self.sample_rate;
        if values[0] != self.last[0] {
            self.dwell.set(Self::dwell_gain(values[0]));
        }
        if values[1] != self.last[1] {
            let mixer = values[1];
            let (b0, b1, c) = design(9, 20.0, table(&MIXER_LOW_DB, mixer));
            let low = FirstOrder::at_rate(b0, b1, c, rate);
            let (b0, b1, c) = design(8, 3000.0, table(&MIXER_HIGH_DB, mixer));
            let high = FirstOrder::at_rate(b0, b1, c, rate);
            for channel in 0..2 {
                self.dry_low[channel].set(low);
                self.dry_high[channel].set(high);
            }
            self.wet.set(db_gain(table(&MIXER_WET_DB, mixer)));
        }
        if values[2] != self.last[2] {
            let (b0, b1, c) = design(0, table(&TONE_HZ, values[2]), 0.0);
            self.tone.set(FirstOrder::at_rate(b0, b1, c, rate));
        }
        if values[3] != self.last[3] {
            self.level.set(values[3].clamp(0.0, 150.0) / 100.0);
        }
        self.last = *values;
    }

    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        // Reads first, as the original does before its block of ring writes.
        let tension_delayed = self.tension_line.read(self.tension_tap);
        let diffused: [f32; 4] =
            std::array::from_fn(|i| self.diffuser_lines.read(i, self.diffusers[i].feedback_tap));
        let tank_reads: [(f32, f32); 4] = std::array::from_fn(|i| {
            let taps = &self.tank[i].line;
            (
                self.tank_lines.read(i, taps.feedback_tap),
                self.tank_lines.read(i, taps.output_tap),
            )
        });
        let long_a = self.long_a.buffer.read(self.long_a.feedback_tap);
        let long_a_tap = self.long_a.buffer.read(self.long_a.output_tap);
        let long_b = self.long_b.buffer.read(self.long_b.feedback_tap);
        let long_b_tap = self.long_b.buffer.read(self.long_b.output_tap);

        // Input conditioning and excitation detector.
        let mono = 0.5 * (frame[0] + frame[1]);
        let shaped = self.input_peak.process(self.input_highpass.process(mono));
        let candidate = (shaped.abs() * 12.0).min(1.0);
        self.envelope = if self.envelope < candidate {
            candidate
        } else {
            flush(self.envelope * self.release)
        };
        let envelope = self.envelope;
        let tension_1 = self.tension_1.process(envelope);
        let tension = self.tension.process(tension_1);
        self.window += self.window_step;
        let open = self.window < 1.0;
        if !open {
            self.window = 1.0;
        }
        let driven = self.dwell.tick() * shaped;

        // Drip: tension-tuned combs, gated by the falling tension.
        let comb_a = driven + 0.93 * self.comb_a_read;
        let comb_b = driven + 0.92 * self.comb_b_read;
        let read_a = self.comb_a.read_linear((self.comb_delay - 1.0).max(0.0));
        let read_b = self
            .comb_b
            .read_linear((self.comb_delay + self.comb_offset - 1.0).max(0.0));
        self.comb_a_read = read_a;
        self.comb_b_read = read_b;
        let fall = tension_delayed - tension;
        let gated = if fall >= 0.007_079 && open { fall } else { 0.0 };
        let fall = self.fall.process(gated);
        let resonance = read_b * 0.47 + read_a;
        let drip = self
            .drip_lowpass
            .process(self.drip_highpass.process(fall * resonance));
        self.comb_delay = tension.min(1.0) * self.depth;
        let fast = self.fast.process(envelope);
        let slow = self.slow.process(envelope);
        if fast >= 1e-5 && slow < fast {
            self.window = 0.0;
        }

        // Diffusers: w = 0.71 x + 0.57 d, y = d - 0.57 w.
        let mut x = driven;
        let mut writes = [0.0f32; 4];
        for (i, d) in diffused.into_iter().enumerate() {
            let w = 0.71 * x + 0.57 * d;
            writes[i] = w;
            x = d - 0.57 * w;
        }
        let diffused_out = x;

        // Tank lines with split damping and loop shelves.
        let input = diffused_out + 0.33 * drip;
        let mut tap_sum = 0.0;
        let mut tank_lines = self.tank_lines.frame();
        for (i, (line, (recirculated, tap))) in self.tank.iter_mut().zip(tank_reads).enumerate() {
            // One-pole C74 split: low = LP(read), high = (1 - C74)(read - LP_prev).
            let low_previous = line.lowpass;
            let low = flush(low_previous + (recirculated - low_previous) * line.damping);
            let high = recirculated - low;
            line.lowpass = low;
            let x = input + line.low_gain * low + line.high_gain * high;
            let y = line.shelf2.process(line.shelf1.process(x));
            tank_lines.write_raw(i, y);
            tap_sum += tap;
        }
        self.tank_lines.advance();
        let long_a_fb = cascade(&mut self.long_a_shelves, long_a);
        let long_b_fb = cascade(&mut self.long_b_shelves, long_b);
        self.long_a
            .buffer
            .write(0.54 * drip + 0.789 * long_a_fb + diffused_out);
        self.long_b
            .buffer
            .write(0.54 * drip + 0.492 * long_b_fb + diffused_out);
        for (i, w) in writes.into_iter().enumerate() {
            self.diffuser_lines.write_raw(i, w);
        }
        self.diffuser_lines.advance();
        self.comb_a.write(comb_a);
        self.comb_b.write(comb_b);
        self.tension_line.write(tension);

        // Wet output.
        let taps = self
            .taps_shelf
            .process(self.taps_highpass.process(0.45 * tap_sum));
        let band_a = cascade(&mut self.tap_a_filters, long_a_tap);
        let band_b = cascade(&mut self.tap_b_filters, long_b_tap);
        let wet = 0.62 * drip + 0.72 * taps + 0.7 * band_b + band_a;
        let wet = self.tone.process(self.wet_highpass.process(wet)) * self.wet.tick();

        let level = self.level.tick();
        let mut out = [0.0; 2];
        for (channel, sample) in out.iter_mut().enumerate() {
            let dry = self.dry_high[channel].process(self.dry_low[channel].process(frame[channel]));
            *sample = level * (dry + wet);
        }
        out
    }

    fn clear(&mut self) {
        self.input_highpass.clear();
        self.input_peak.clear();
        self.envelope = 0.0;
        for leaky in [
            &mut self.tension_1,
            &mut self.tension,
            &mut self.fast,
            &mut self.slow,
            &mut self.fall,
        ] {
            leaky.state = 0.0;
        }
        self.window = 1.0;
        self.tension_line.clear();
        self.comb_a.clear();
        self.comb_b.clear();
        self.comb_a_read = 0.0;
        self.comb_b_read = 0.0;
        self.comb_delay = 0.0;
        self.drip_highpass.clear();
        self.drip_lowpass.clear();
        self.diffuser_lines.clear();
        self.tank_lines.clear();
        for line in &mut self.tank {
            line.shelf1.clear();
            line.shelf2.clear();
            line.lowpass = 0.0;
        }
        self.long_a.buffer.clear();
        self.long_b.buffer.clear();
        for filter in self
            .long_a_shelves
            .iter_mut()
            .chain(&mut self.long_b_shelves)
            .chain(&mut self.tap_a_filters)
            .chain(&mut self.tap_b_filters)
            .chain(&mut self.dry_low)
            .chain(&mut self.dry_high)
        {
            filter.clear();
        }
        self.taps_highpass.clear();
        self.taps_shelf.clear();
        self.wet_highpass.clear();
        self.tone.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Model, dsp::gain_to_db};

    fn values(dwell: f32, mixer: f32, tone: f32, level: f32) -> Values {
        let mut v = [0.0; crate::MAX_PARAMS];
        v[..4].copy_from_slice(&[dwell, mixer, tone, level]);
        v
    }

    #[test]
    fn tables_interpolate_like_tbl_to_val() {
        assert_eq!(table(&TONE_HZ, 0.0), 100.0);
        assert_eq!(table(&TONE_HZ, 100.0), 22000.0);
        assert!((table(&TONE_HZ, 58.0) - (5575.0 + 0.8 * (7984.0 - 5575.0))).abs() < 0.5);
        assert!((table(&DWELL_DB, 35.0) - (-2.665)).abs() < 1e-3);
        assert_eq!(Spring63::dwell_gain(0.0), 0.0);
        assert!((gain_to_db(Spring63::dwell_gain(100.0)) - 6.0).abs() < 1e-3);
    }

    #[test]
    fn designer_shelves_have_the_requested_gains() {
        let (b0, b1, c) = design(9, 20.0, -11.0);
        let low = FirstOrder::at_rate(b0, b1, c, 44_100.0);
        assert!((gain_to_db(low.magnitude(44_100.0, 1.0)) + 11.0).abs() < 0.1);
        assert!(gain_to_db(low.magnitude(44_100.0, 2000.0)).abs() < 0.1);
        let (b0, b1, c) = design(8, 3000.0, -6.0);
        let high = FirstOrder::at_rate(b0, b1, c, 44_100.0);
        assert!((gain_to_db(high.magnitude(44_100.0, 22_000.0)) + 6.0).abs() < 0.2);
        assert!(gain_to_db(high.magnitude(44_100.0, 50.0)).abs() < 0.1);
        let (b0, b1, c) = design(0, 1000.0, 0.0);
        let lowpass = FirstOrder::at_rate(b0, b1, c, 44_100.0);
        assert!((gain_to_db(lowpass.magnitude(44_100.0, 1000.0)) + 3.01).abs() < 0.05);
    }

    #[test]
    fn rom_sections_keep_their_shape_at_other_rates() {
        // The 104 Hz input high-pass (C39..43) at 44.1 and 96 kHz.
        let at44 = FirstOrder::at_rate(0.992_646, -0.992_646, 0.985_291, 44_100.0);
        let at96 = FirstOrder::at_rate(0.992_646, -0.992_646, 0.985_291, 96_000.0);
        for hz in [30.0, 104.0, 1000.0] {
            let a = gain_to_db(at44.magnitude(44_100.0, hz));
            let b = gain_to_db(at96.magnitude(96_000.0, hz));
            assert!((a - b).abs() < 0.05, "{hz}: {a} {b}");
        }
        assert!((gain_to_db(at44.magnitude(44_100.0, 104.0)) + 3.0).abs() < 0.1);
        // The 3 kHz +6 dB peak of the input stage.
        let peak = rom_biquad(
            [1.093_457, -1.649_163, 0.718_74, 1.649_163, -0.812_197],
            48_000.0,
        );
        let db = gain_to_db(peak.coefficients.magnitude(48_000.0, 3000.0));
        assert!((db - 6.0).abs() < 0.3, "{db}");
    }

    #[test]
    fn delays_follow_the_ring_offsets() {
        // Line 6 recirculates over C23 - C38 = 74100 - 61311 samples.
        assert_eq!(samples(12789.0, 44_100.0), 12788);
        let effect = Spring63::new(88_200.0);
        assert_eq!(effect.long_b.feedback_tap, 2 * 12789 - 1);
        assert_eq!(effect.tank[0].line.output_tap, 2 * 1013 - 1);
        assert_eq!(effect.diffusers[1].feedback_tap, 2 * 1852 - 1);
    }

    #[test]
    fn dry_path_is_near_unity_at_defaults_and_dwell_zero_is_dry_only() {
        let rate = 48_000.0;
        let mut effect = Spring63::new(rate);
        effect.set(&values(0.0, 51.0, 58.0, 100.0), 120.0);
        let mut effect_dry_only = Spring63::new(rate);
        effect_dry_only.set(&values(0.0, 51.0, 58.0, 100.0), 120.0);
        let mut peak = 0.0f32;
        for i in 0..(rate as usize) {
            let x = 0.5 * (std::f32::consts::TAU * 500.0 * i as f32 / rate).sin();
            let y = effect.process([x, x]);
            if i > rate as usize / 2 {
                peak = peak.max(y[0].abs());
            }
        }
        // Dry shelves at MIXER 51 leave 500 Hz within about 1 dB.
        assert!((gain_to_db(peak / 0.5)).abs() < 1.0, "{peak}");
        // With DWELL 0 and silence the wet path stays silent.
        for _ in 0..1000 {
            let y = effect_dry_only.process([0.0, 0.0]);
            assert_eq!(y, [0.0, 0.0]);
        }
    }

    #[test]
    fn reverb_tail_rings_and_then_decays() {
        let rate = 44_100.0;
        let mut effect = Spring63::new(rate);
        effect.set(&values(100.0, 100.0, 100.0, 100.0), 120.0);
        // A short burst, then silence.
        let mut energy = [0.0f32; 4];
        for i in 0..(rate as usize * 8) {
            let x = if i < 400 {
                0.5 * (std::f32::consts::TAU * 700.0 * i as f32 / rate).sin()
            } else {
                0.0
            };
            let y = effect.process([x, x]);
            assert!(y[0].is_finite() && y[0].abs() < 4.0, "{i} {}", y[0]);
            let second = i / (rate as usize * 2);
            energy[second] += y[0] * y[0];
        }
        assert!(energy[0] > 1e-2, "{energy:?}");
        assert!(energy[3] < energy[0] * 1e-4, "{energy:?}");
    }

    #[test]
    fn drip_window_opens_on_attacks_for_49_ms() {
        let rate = 48_000.0;
        let mut effect = Spring63::new(rate);
        effect.set(&Model::Spring63.defaults(), 120.0);
        for _ in 0..100 {
            effect.process([0.0; 2]);
        }
        assert!(effect.window >= 1.0);
        effect.process([0.5, 0.5]);
        assert_eq!(effect.window, 0.0);
        // The fast follower stays above the slow one for a few ms after the
        // attack and keeps restarting the window; it closes 48.8 ms after the
        // last restart.
        let (mut last_reset, mut closed) = (0, 0);
        for i in 1..(rate as usize / 5) {
            effect.process([0.0; 2]);
            if effect.window == 0.0 {
                last_reset = i;
            }
            if effect.window >= 1.0 && closed == 0 {
                closed = i;
            }
        }
        assert!(last_reset > 0 && last_reset < closed);
        let ms = (closed - last_reset) as f32 / rate * 1000.0;
        assert!((ms - 48.8).abs() < 0.2, "{ms}");
    }

    #[test]
    fn drip_comb_follows_the_tension() {
        let rate = 96_000.0;
        let mut effect = Spring63::new(rate);
        effect.set(&Model::Spring63.defaults(), 120.0);
        for i in 0..2000 {
            let x = if i % 2 == 0 { 0.9 } else { -0.9 };
            effect.process([x, x]);
        }
        // Saturated tension: 71.68 samples at 44.1 kHz, rescaled.
        let expected = 71.68 * rate / 44_100.0;
        assert!(
            (effect.comb_delay - expected).abs() < 0.5,
            "{}",
            effect.comb_delay
        );
    }
}
