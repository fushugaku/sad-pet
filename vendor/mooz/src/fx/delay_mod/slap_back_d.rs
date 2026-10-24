//! SlapBackD (SLAPBKD.ZDL), `Fx_DLY_SlapBackDelay`: a stereo slapback
//! delay with one 2 s ring per channel.
//!
//! Per channel, as in the callback: the input passes the c10..14 high-cut
//! shelf (−3 dB at the top, unity at DC); the feedback word (the tap
//! output of the previous 8-frame block) is scaled by F.B (`c9 = 0.993 ·
//! F.B / 100`, `fb_edit`), hard-clipped to ±1 and DC-blocked (c25..29);
//! `feedback + c5 · input` is written. Two read heads (c45/c46 left,
//! c47/c48 right) crossfade linearly over 0.4 s whenever the time changes
//! (c50 ramps by ±c51 = 1/17640 per sample; `time_edit` writes the new
//! offset into the idle head and waits for a running fade to finish). The
//! tap passes the c30..34 ~2 kHz peaking section; that is both the wet
//! signal and the next feedback word.
//!
//! `CalcDelayTime`: TIME 0..299 is `TIME + 1` ms; 300 is "Sync", a quarter
//! note (SubDv 0) or a dotted eighth (SubDv ≠ 0) at most 1999 ms. Unsynced,
//! SubDv ≠ 0 takes `3/4` of the time. Offsets are `441 · ms / 10` samples;
//! SubDv 2 ("P-P") gives the right head `4/3` of the left one.
//!
//! Output (`WetMix_edit`, `KillDry_edit`, `OUTMODE_edit`): `c2 · (c8 · dry +
//! c7 · wet)` with FxLVL `c7 = FxLVL / 100` and DRY `c8 ∈ {0, 1}`; Mode MONO
//! (c53 = 0) averages both sides, dry included.
//!
//! Deviations: the 8-frame feedback block delay is kept as 8 samples scaled
//! to the host rate; ROM filters are re-derived for the host rate; a Mode
//! change crossfades over 16 ms instead of muting c54 around the switch.
use super::common::{FadeTap, FirstOrder};
use crate::{
    Effect, Values,
    catalog::*,
    dsp::{Biquad, Coefficients, DelayLine, flush, scale_samples},
    zoom::{self, DEFAULT_STEP, Ramp},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_slap_back_d",
    name: "SlapBackD",
    category: Category::Delay,
    zdl: "SLAPBKD.ZDL",
    description: "Short stereo slapback delay with ping-pong mode",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "TIME",
            max: 300,
            default: 98,
            display: Display::labels(&TIME_LABELS),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "SubDv",
            max: 2,
            default: 0,
            display: Display::labels(&["1/4", "1/8.", "P-P"]),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "F.B",
            max: 100,
            default: 29,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "FxLVL",
            max: 100,
            default: 40,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "DRY",
            max: 1,
            default: 1,
            display: Display::OFF_ON,
        },
        // values[5] is firmware parameter word 8
        ParamInfo {
            name: "Mode",
            max: 1,
            default: 0,
            display: Display::labels(&["MONO", "STR"]),
        },
    ],
};

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(SlapBackD::new(sample_rate))
}

/// `*0x800004e8`: TIME values above it select "Sync".
const TIME_THRESHOLD: u16 = 299;
/// The synced note's limit (`context[24]` maximum 0x7cf).
const SYNC_MAX_MS: f32 = 1999.0;
/// 88200-word rings.
const RING_SECONDS: f32 = 2.0;
/// c51 = ±0x386dc5b8 per sample: a 17640-sample (0.4 s) crossfade.
const FADE_SECONDS: f32 = 0.4;
/// The feedback words are one 8-frame block old.
const BLOCK: f32 = 8.0;
/// c5 ramps by 0x3ab9c278 per sample when the effect switches on.
const MODE_FADE: f32 = 705.6;

/// ROM (0x80000620) filters at 44.1 kHz, `y = b0 x + b1 x1 + b2 x2 + a1 y1 + a2 y2`.
const INPUT_SHELF: (f32, f32, f32) = (
    f32::from_bits(0x3f448d0e), // c10
    f32::from_bits(0xbeb7521f), // c11
    f32::from_bits(0x3f171c02), // c13
);
const DC_BLOCK: (f32, f32, f32) = (
    f32::from_bits(0x3f7fa2c2), // c25
    f32::from_bits(0xbf7fa2c2), // c26
    f32::from_bits(0x3f7f4585), // c28
);
/// c30..34.
const PEAK: [f32; 5] = [
    f32::from_bits(0x3f819597),
    f32::from_bits(0xbfe58b82),
    f32::from_bits(0x3f5b3639),
    f32::from_bits(0x3fe58b82),
    f32::from_bits(0xbf5e6167),
];
const FEEDBACK_MAX: f32 = f32::from_bits(0x3f7e353f);

/// `CalcDelayTime` → (left, right) offsets in samples at 44.1 kHz.
fn delay_samples_44k(time: f32, subdivision: usize, bpm: f32) -> (f32, f32) {
    let ms = if zoom::is_synced(time, TIME_THRESHOLD) {
        let note = if subdivision == 0 { 6 } else { 5 };
        zoom::delay_note_ms(note, bpm, SYNC_MAX_MS).floor()
    } else {
        let ms = time.clamp(0.0, f32::from(TIME_THRESHOLD)) + 1.0;
        if subdivision == 0 {
            ms
        } else {
            (3.0 * ms / 4.0).floor()
        }
    }
    .max(1.0);
    let mut left = (441.0 * ms / 10.0).floor();
    if ms == SYNC_MAX_MS {
        left -= 1.0;
    }
    let right = if subdivision == 2 {
        (4.0 * left / 3.0).floor()
    } else {
        left
    };
    (left, right)
}

/// A short fixed delay for the block-old feedback words.
struct BlockDelay {
    buffer: [f32; 32],
    length: usize,
    index: usize,
}

impl BlockDelay {
    fn new(length: usize) -> Self {
        Self {
            buffer: [0.0; 32],
            length: length.clamp(1, 32),
            index: 0,
        }
    }
    /// Returns the word pushed `length` calls ago.
    #[inline]
    fn exchange(&mut self, x: f32) -> f32 {
        let y = self.buffer[self.index];
        self.buffer[self.index] = x;
        self.index += 1;
        if self.index == self.length {
            self.index = 0;
        }
        y
    }
}

struct Channel {
    shelf: FirstOrder,
    dc_block: FirstOrder,
    peak: Biquad,
    line: DelayLine,
    tap: FadeTap,
    feedback: BlockDelay,
    pending: f32,
}

impl Channel {
    fn new(sample_rate: f32, shelf: FirstOrder, dc_block: FirstOrder, peak: Coefficients) -> Self {
        Self {
            shelf,
            dc_block,
            peak: Biquad::new(peak),
            line: DelayLine::with_seconds(RING_SECONDS, sample_rate),
            tap: FadeTap::new(1.0, FADE_SECONDS, sample_rate),
            feedback: BlockDelay::new(scale_samples(BLOCK, sample_rate).round() as usize),
            pending: 0.0,
        }
    }

    #[inline]
    fn process(&mut self, x: f32, feedback_gain: f32, input_gain: f32) -> f32 {
        let input = self.shelf.process(x);
        let old = self.feedback.exchange(self.pending);
        let clipped = (feedback_gain * old).clamp(-1.0, 1.0);
        let feedback = self.dc_block.process(clipped);
        self.line.write(flush(feedback + input_gain * input));
        let wet = self.peak.process(self.tap.read(&self.line));
        self.pending = wet;
        wet
    }

    fn clear(&mut self) {
        self.shelf.clear();
        self.dc_block.clear();
        self.peak.clear();
        self.line.clear();
        self.tap.jump(self.tap.target());
        self.feedback.buffer = [0.0; 32];
        self.pending = 0.0;
    }
}

struct SlapBackD {
    sample_rate: f32,
    channels: [Channel; 2],
    feedback: Ramp,
    wet: Ramp,
    dry: Ramp,
    stereo: Ramp,
    input: f32,
    max_delay: f32,
    previous: Option<(Values, f32)>,
}

impl SlapBackD {
    fn new(sample_rate: f32) -> Self {
        let rate = zoom::ORIGINAL_RATE;
        let shelf = FirstOrder::resampled(
            INPUT_SHELF.0,
            INPUT_SHELF.1,
            INPUT_SHELF.2,
            rate,
            sample_rate,
        );
        let dc = FirstOrder::resampled(DC_BLOCK.0, DC_BLOCK.1, DC_BLOCK.2, rate, sample_rate);
        let peak = Coefficients {
            b0: PEAK[0],
            b1: PEAK[1],
            b2: PEAK[2],
            a1: -PEAK[3],
            a2: -PEAK[4],
        }
        .resample(rate, sample_rate);
        let channel = || {
            Channel::new(
                sample_rate,
                FirstOrder::new(shelf.0, shelf.1, shelf.2),
                FirstOrder::new(dc.0, dc.1, dc.2),
                peak,
            )
        };
        let mut effect = Self {
            sample_rate,
            channels: [channel(), channel()],
            feedback: Ramp::new(0.0, DEFAULT_STEP, sample_rate),
            wet: Ramp::new(0.0, DEFAULT_STEP, sample_rate),
            dry: Ramp::new(1.0, DEFAULT_STEP, sample_rate),
            stereo: Ramp::new(0.0, 1.0 / MODE_FADE, sample_rate),
            input: 1.0,
            max_delay: 0.0,
            previous: None,
        };
        effect.max_delay = effect.channels[0].line.capacity() as f32 - 2.0;
        let defaults = crate::Model::from_key(INFO.key)
            .map(|model| model.defaults())
            .unwrap_or([0.0; crate::MAX_PARAMS]);
        effect.apply(&defaults, 120.0, true);
        effect
    }

    fn apply(&mut self, values: &Values, bpm: f32, jump: bool) {
        let previous = self.previous;
        let changed = |i: usize| previous.is_none_or(|(old, _)| old[i] != values[i]);
        let bpm_changed = previous.is_none_or(|(_, old)| old != bpm);
        let set = |ramp: &mut Ramp, target: f32| {
            if jump {
                ramp.jump(target);
            } else {
                ramp.set(target);
            }
        };

        if changed(0) || changed(1) || bpm_changed {
            let subdivision = values[1].round().clamp(0.0, 2.0) as usize;
            let (left, right) = delay_samples_44k(values[0], subdivision, bpm);
            for (channel, samples) in self.channels.iter_mut().zip([left, right]) {
                let delay = scale_samples(samples, self.sample_rate)
                    .round()
                    .clamp(1.0, self.max_delay);
                if jump {
                    channel.tap.jump(delay);
                } else {
                    channel.tap.set(delay);
                }
            }
        }
        if changed(2) {
            let feedback = zoom::curve(FEEDBACK_MAX, 0.0, 101, values[2].clamp(0.0, 100.0), 0);
            set(&mut self.feedback, feedback);
        }
        if changed(3) {
            let wet = zoom::curve(1.0, 0.0, 101, values[3].clamp(0.0, 100.0), 0);
            set(&mut self.wet, wet);
        }
        if changed(4) {
            set(&mut self.dry, if values[4] >= 0.5 { 1.0 } else { 0.0 });
        }
        if changed(5) {
            set(&mut self.stereo, if values[5] >= 0.5 { 1.0 } else { 0.0 });
        }
        self.previous = Some((*values, bpm));
    }
}

impl Effect for SlapBackD {
    fn set(&mut self, values: &Values, bpm: f32) {
        self.apply(values, bpm, false);
    }

    #[inline]
    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let feedback = self.feedback.tick();
        let wet_gain = self.wet.tick();
        let dry_gain = self.dry.tick();
        let stereo = self.stereo.tick();
        let wet_left = self.channels[0].process(left, feedback, self.input);
        let wet_right = self.channels[1].process(right, feedback, self.input);
        let out_left = dry_gain * left + wet_gain * wet_left;
        let out_right = dry_gain * right + wet_gain * wet_right;
        let mono = 0.5 * (out_left + out_right);
        [
            mono + (out_left - mono) * stereo,
            mono + (out_right - mono) * stereo,
        ]
    }

    fn clear(&mut self) {
        for channel in &mut self.channels {
            channel.clear();
        }
    }
}

const TIME_LABELS: [&str; 301] = [
    "1 ms", "2 ms", "3 ms", "4 ms", "5 ms", "6 ms", "7 ms", "8 ms", "9 ms", "10 ms", "11 ms",
    "12 ms", "13 ms", "14 ms", "15 ms", "16 ms", "17 ms", "18 ms", "19 ms", "20 ms", "21 ms",
    "22 ms", "23 ms", "24 ms", "25 ms", "26 ms", "27 ms", "28 ms", "29 ms", "30 ms", "31 ms",
    "32 ms", "33 ms", "34 ms", "35 ms", "36 ms", "37 ms", "38 ms", "39 ms", "40 ms", "41 ms",
    "42 ms", "43 ms", "44 ms", "45 ms", "46 ms", "47 ms", "48 ms", "49 ms", "50 ms", "51 ms",
    "52 ms", "53 ms", "54 ms", "55 ms", "56 ms", "57 ms", "58 ms", "59 ms", "60 ms", "61 ms",
    "62 ms", "63 ms", "64 ms", "65 ms", "66 ms", "67 ms", "68 ms", "69 ms", "70 ms", "71 ms",
    "72 ms", "73 ms", "74 ms", "75 ms", "76 ms", "77 ms", "78 ms", "79 ms", "80 ms", "81 ms",
    "82 ms", "83 ms", "84 ms", "85 ms", "86 ms", "87 ms", "88 ms", "89 ms", "90 ms", "91 ms",
    "92 ms", "93 ms", "94 ms", "95 ms", "96 ms", "97 ms", "98 ms", "99 ms", "100 ms", "101 ms",
    "102 ms", "103 ms", "104 ms", "105 ms", "106 ms", "107 ms", "108 ms", "109 ms", "110 ms",
    "111 ms", "112 ms", "113 ms", "114 ms", "115 ms", "116 ms", "117 ms", "118 ms", "119 ms",
    "120 ms", "121 ms", "122 ms", "123 ms", "124 ms", "125 ms", "126 ms", "127 ms", "128 ms",
    "129 ms", "130 ms", "131 ms", "132 ms", "133 ms", "134 ms", "135 ms", "136 ms", "137 ms",
    "138 ms", "139 ms", "140 ms", "141 ms", "142 ms", "143 ms", "144 ms", "145 ms", "146 ms",
    "147 ms", "148 ms", "149 ms", "150 ms", "151 ms", "152 ms", "153 ms", "154 ms", "155 ms",
    "156 ms", "157 ms", "158 ms", "159 ms", "160 ms", "161 ms", "162 ms", "163 ms", "164 ms",
    "165 ms", "166 ms", "167 ms", "168 ms", "169 ms", "170 ms", "171 ms", "172 ms", "173 ms",
    "174 ms", "175 ms", "176 ms", "177 ms", "178 ms", "179 ms", "180 ms", "181 ms", "182 ms",
    "183 ms", "184 ms", "185 ms", "186 ms", "187 ms", "188 ms", "189 ms", "190 ms", "191 ms",
    "192 ms", "193 ms", "194 ms", "195 ms", "196 ms", "197 ms", "198 ms", "199 ms", "200 ms",
    "201 ms", "202 ms", "203 ms", "204 ms", "205 ms", "206 ms", "207 ms", "208 ms", "209 ms",
    "210 ms", "211 ms", "212 ms", "213 ms", "214 ms", "215 ms", "216 ms", "217 ms", "218 ms",
    "219 ms", "220 ms", "221 ms", "222 ms", "223 ms", "224 ms", "225 ms", "226 ms", "227 ms",
    "228 ms", "229 ms", "230 ms", "231 ms", "232 ms", "233 ms", "234 ms", "235 ms", "236 ms",
    "237 ms", "238 ms", "239 ms", "240 ms", "241 ms", "242 ms", "243 ms", "244 ms", "245 ms",
    "246 ms", "247 ms", "248 ms", "249 ms", "250 ms", "251 ms", "252 ms", "253 ms", "254 ms",
    "255 ms", "256 ms", "257 ms", "258 ms", "259 ms", "260 ms", "261 ms", "262 ms", "263 ms",
    "264 ms", "265 ms", "266 ms", "267 ms", "268 ms", "269 ms", "270 ms", "271 ms", "272 ms",
    "273 ms", "274 ms", "275 ms", "276 ms", "277 ms", "278 ms", "279 ms", "280 ms", "281 ms",
    "282 ms", "283 ms", "284 ms", "285 ms", "286 ms", "287 ms", "288 ms", "289 ms", "290 ms",
    "291 ms", "292 ms", "293 ms", "294 ms", "295 ms", "296 ms", "297 ms", "298 ms", "299 ms",
    "300 ms", "Sync",
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Model;

    fn defaults() -> Values {
        Model::from_key(INFO.key).unwrap().defaults()
    }

    #[test]
    fn calc_delay_time_follows_the_editor() {
        assert_eq!(delay_samples_44k(98.0, 0, 120.0), (4365.0, 4365.0));
        // SubDv 1: 3/4 of 100 ms = 75 ms on both sides.
        assert_eq!(delay_samples_44k(99.0, 1, 120.0), (3307.0, 3307.0));
        // P-P: 75 ms left, 4/3 of it right.
        assert_eq!(delay_samples_44k(99.0, 2, 120.0), (3307.0, 4409.0));
        // Sync: a quarter (500 ms) or a dotted eighth (375 ms) at 120 BPM.
        assert_eq!(delay_samples_44k(300.0, 0, 120.0), (22_050.0, 22_050.0));
        assert_eq!(delay_samples_44k(300.0, 1, 120.0), (16_537.0, 16_537.0));
    }

    #[test]
    fn the_echo_lands_at_the_time() {
        for sample_rate in [44_100.0, 96_000.0] {
            let mut effect = SlapBackD::new(sample_rate);
            let mut values = defaults();
            values[0] = 99.0; // 100 ms
            values[2] = 0.0;
            values[3] = 100.0;
            values[4] = 0.0;
            values[5] = 1.0;
            effect.apply(&values, 120.0, true);
            let expected = scale_samples(4410.0, sample_rate).round() as usize;
            let mut peak = (0, 0.0f32);
            for i in 0..expected * 2 {
                let x = if i == 0 { 1.0 } else { 0.0 };
                let [l, _] = effect.process([x, 0.0]);
                if l.abs() > peak.1 {
                    peak = (i, l.abs());
                }
            }
            assert!(peak.0.abs_diff(expected) <= 1, "{sample_rate}: {peak:?}");
        }
    }

    #[test]
    fn rom_filters_have_their_shapes() {
        let peak = Coefficients {
            b0: PEAK[0],
            b1: PEAK[1],
            b2: PEAK[2],
            a1: -PEAK[3],
            a2: -PEAK[4],
        };
        assert!((peak.magnitude(44_100.0, 10.0) - 1.0).abs() < 0.01);
        assert!(peak.magnitude(44_100.0, 2000.0) > 1.1);
        let at96 = peak.resample(44_100.0, 96_000.0);
        assert!((at96.magnitude(96_000.0, 2000.0) - peak.magnitude(44_100.0, 2000.0)).abs() < 0.05);
        let shelf_dc = (INPUT_SHELF.0 + INPUT_SHELF.1) / (1.0 - INPUT_SHELF.2);
        assert!((shelf_dc - 1.0).abs() < 1e-3);
    }

    #[test]
    fn mono_mode_sums_and_stereo_keeps_sides() {
        let mut effect = SlapBackD::new(48_000.0);
        let mut values = defaults();
        values[5] = 0.0;
        effect.apply(&values, 120.0, true);
        let [l, r] = effect.process([1.0, 0.0]);
        assert!((l - r).abs() < 1e-6 && (l - 0.5).abs() < 1e-6);
        values[5] = 1.0;
        effect.apply(&values, 120.0, true);
        let [l, r] = effect.process([1.0, 0.0]);
        assert!((l - 1.0).abs() < 1e-6 && r.abs() < 1e-6);
    }

    #[test]
    fn maximum_feedback_stays_bounded_and_rings() {
        let mut effect = SlapBackD::new(48_000.0);
        let mut values = defaults();
        values[0] = 20.0;
        values[2] = 100.0;
        values[3] = 100.0;
        effect.apply(&values, 120.0, true);
        let mut noise = crate::dsp::Noise::new(9);
        let mut peak = 0.0f32;
        for i in 0..48_000 * 10 {
            let x = if i < 4800 { noise.bipolar() } else { 0.0 };
            let [l, r] = effect.process([x, x]);
            peak = peak.max(l.abs()).max(r.abs());
        }
        assert!(peak.is_finite() && peak < 4.0, "{peak}");
    }
}
