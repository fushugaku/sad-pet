//! Ambience (AMBIENCE.ZDL, `Fx_REV_tc_AMB`).
//!
//! Follows the recovered callback: three input biquads (two ROM peaking EQs,
//! one identity) and a ROM high-pass/low-pass pair feed a mono injection into
//! the ring; a pre-delay tap pair (PreD and PreD + 76 samples) enters a
//! cross-coupled early loop (4631 + 5160 samples, two-band damped with the
//! one-pole crossover C77, the tone shelf C80..82 and the ROM shelf C83..85)
//! whose two halves each run through six lossy all-passes (w = x + 0.42 d,
//! y = -0.42 x + 0.378 d); each diffused side feeds four two-band damped combs
//! (82.6/69.5/61.0/47.5 ms) read by four output taps. Each output is
//! 0.03 x pre-delay + taps (x0.84 left, x1 right) + diffused signal through the
//! ROM low peak (C102..106) and the decay high-pass (C107..111), scaled by the
//! Mix table and added to the input-EQ'd dry signal (Dry switch, C52).
//!
//! Editors decoded: `decay_Calc_set` (T60 = 3000 ms x (Decay/100)^1.7, comb
//! and early-loop gains `g^(d/T60)` with the low band at 0.78 T60, the
//! type-1 output high-pass at 300 - 290 (Decay/100)^1.6 Hz, the injection fade
//! below Decay 3), `tone_edit` (type-8 high shelf at 16 kHz, -15..-3.5 dB
//! table), `mix_edit` (11-point wet table, 0 at Mix 0), `predelay_edit`
//! (PreD x 44.1 samples, minimum 1), `KillDry_edit`, `onf_aft` (C3, C127,
//! all-pass gains) and `Level_edit` (fixed 1.0). Ramps follow the editors'
//! setters (16 ms for Mix/Dry, step 0.00189 per 44.1 kHz sample for gains).
//! Approximated: a PreD change crossfades the tap over 16 ms instead of the
//! original's mute/jump/reopen; the ROM filters are re-derived for the host
//! rate through their bilinear prototypes.
use super::fast::Ramp;
use super::fast::{Bank, Biquad};
use super::tc_common::{
    CombBank, Design, FirstOrder, Glide, Line, TapFade, decay_gain, design, diffuse, one_pole_rom,
    rom_biquad, samples, table,
};
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_ambience",
    name: "Ambience",
    category: Category::Reverb,
    zdl: "AMBIENCE.ZDL",
    description: "Room ambience: early reflections and a short diffuse tail",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "DECAY",
            max: 100,
            default: 70,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "TONE",
            max: 100,
            default: 80,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "MIX",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "PreD",
            max: 200,
            default: 29,
            display: Display::number(0.0, 1.0, 0, " ms"),
        },
        // values[4] is firmware parameter word 7
        ParamInfo {
            name: "Dry",
            max: 1,
            default: 1,
            display: Display::OFF_ON,
        },
    ],
};

/// C112..116 and C117..121 (`[b0, b1, b2, -a1, -a2]`); C122..126 is identity.
const INPUT_EQ: [[f32; 5]; 2] = [
    [0.988_557, -1.465_969, 0.602_334, 1.465_969, -0.590_891],
    [0.977_861, -0.175_855, 0.230_608, 0.175_855, -0.208_469],
];
/// C37..41 (first-order high-pass) and C42..46 (first-order low-pass).
const INPUT_HP: (f32, f32, f32) = (0.996_451, -0.996_451, 0.992_901);
const INPUT_LP: (f32, f32, f32) = (0.443_248, 0.013_194, 0.543_558);
/// C83..85, the fixed shelf after the tone section of every loop.
const LOOP_SHELF: (f32, f32, f32) = (0.997_368, -0.962_46, 0.959_827);
/// C102..106, the output low peak.
const OUTPUT_EQ: [f32; 5] = [1.003_685, -1.959_337, 0.957_212, 1.959_337, -0.960_897];
/// C77, the one-pole crossover of every damped loop.
const CROSSOVER: f32 = 0.129_666;
/// C86, comb input; C95/C96 left/right tap sums; C97 pre-delay to output.
const COMB_INPUT: f32 = 0.35;
const TAP_GAIN: [f32; 2] = [0.84, 1.0];
const DIRECT: f32 = 0.03;
/// C48/C49/C50 after `onf_aft`.
const DIFFUSION: (f32, f32, f32) = (0.42, -0.42, 0.378);

/// Ring offsets at 44.1 kHz (read minus write coefficient).
const EARLY: [f32; 2] = [4631.0, 5160.0];
const DIFFUSER: [f32; 6] = [1058.0, 869.0, 706.0, 568.0, 451.0, 355.0];
const COMB: [f32; 4] = [3644.0, 3066.0, 2692.0, 2096.0];
const OUTPUT_TAPS: [[f32; 4]; 2] = [
    [1572.0, 1249.0, 818.0, 540.0],
    [1648.0, 1325.0, 894.0, 617.0],
];
/// C6 reads 76 samples beyond C5.
const SECOND_TAP: f32 = 76.0;

/// `decay_Calc_set` table at 0x80000728: (loop ms, gain at T60).
const COMB_DECAY: [(f32, f32); 4] = [
    (82.64, 0.001_238_05),
    (69.52, 0.001_303_21),
    (61.04, 0.001_371_8),
    (47.52, 0.001_444),
];
const EARLY_DECAY: (f32, f32) = (222.0, 0.0005);
/// `mix_edit` table at 0x800006c8.
const MIX: [f32; 11] = [
    0.001, 0.045_073, 0.108_696, 0.199_338, 0.321_27, 0.479_107, 0.677_839, 0.922_842, 1.219_888,
    1.575_159, 1.995_262,
];
/// `tone_edit` table at 0x800006f8 (dB of the 16 kHz shelf).
const TONE: [f32; 11] = [
    -15.0, -14.0, -13.0, -12.0, -11.0, -10.0, -9.0, -8.0, -7.0, -6.0, -3.5,
];
const GAIN_STEP: f32 = 0.001_890_359;

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(Ambience::new(sample_rate))
}

struct Ambience {
    sample_rate: f32,
    input_eq: [[Biquad; 2]; 2],
    input_hp: [FirstOrder; 2],
    input_lp: [FirstOrder; 2],
    injection: Line,
    predelay: TapFade,
    second_tap: f32,
    early: Bank<2>,
    early_lp: [f32; 2],
    early_tone: [FirstOrder; 2],
    early_shelf: [FirstOrder; 2],
    /// Right chain in regions 0..6, left chain in 6..12.
    diffusers: Bank<12>,
    combs: CombBank<8>,
    output_taps: [[usize; 4]; 2],
    output_eq: [Biquad; 2],
    output_hp: [FirstOrder; 2],
    crossover: f32,
    wet: Glide,
    dry: Glide,
    input: Ramp,
    early_gain: [Ramp; 2],
    comb_low: [Ramp; 4],
    comb_high: [Ramp; 4],
    last: [f32; 4],
}

impl Ambience {
    fn new(sample_rate: f32) -> Self {
        let rom1 = |c: (f32, f32, f32)| FirstOrder::rom(c.0, c.1, c.2, sample_rate);
        let shelf = rom1(LOOP_SHELF);
        let max_predelay = samples(200.0 * 44.1 + SECOND_TAP + 2.0, sample_rate);
        let ramp = |v: f32| Ramp::new(v, GAIN_STEP, sample_rate);
        let mut effect = Self {
            sample_rate,
            input_eq: std::array::from_fn(|_| INPUT_EQ.map(|w| rom_biquad(w, sample_rate))),
            input_hp: [rom1(INPUT_HP); 2],
            input_lp: [rom1(INPUT_LP); 2],
            injection: Line::new(max_predelay),
            predelay: TapFade::new(1.0, sample_rate),
            second_tap: SECOND_TAP * sample_rate / 44_100.0,
            early: Bank::new(EARLY.map(|d| samples(d, sample_rate)), 0),
            early_lp: [0.0; 2],
            early_tone: [FirstOrder::IDENTITY; 2],
            early_shelf: [shelf; 2],
            diffusers: Bank::new(
                std::array::from_fn(|i| samples(DIFFUSER[i % 6], sample_rate)),
                0,
            ),
            combs: CombBank::new(
                Bank::new(
                    std::array::from_fn(|i| samples(COMB[i % 4], sample_rate)),
                    0,
                ),
                FirstOrder::IDENTITY,
                shelf,
            ),
            output_taps: OUTPUT_TAPS.map(|side| side.map(|d| samples(d, sample_rate))),
            output_eq: [rom_biquad(OUTPUT_EQ, sample_rate); 2],
            output_hp: [FirstOrder::IDENTITY; 2],
            crossover: one_pole_rom(CROSSOVER, sample_rate),
            wet: Glide::new(0.0, 0.016, sample_rate),
            dry: Glide::new(1.0, 0.016, sample_rate),
            input: ramp(1.0),
            early_gain: [ramp(0.0); 2],
            comb_low: [ramp(0.0); 4],
            comb_high: [ramp(0.0); 4],
            last: [f32::NAN; 4],
        };
        let mut defaults = [0.0; crate::MAX_PARAMS];
        for (value, param) in defaults.iter_mut().zip(INFO.params) {
            *value = f32::from(param.default);
        }
        effect.set(&defaults, 120.0);
        effect.settle();
        effect
    }

    /// Jump every ramp to its target (initial state, like the original init).
    fn settle(&mut self) {
        let jump = |r: &mut Ramp| r.jump(r.target);
        self.wet.current = self.wet_target();
        self.dry.current = self.dry_target();
        jump(&mut self.input);
        self.early_gain.iter_mut().for_each(jump);
        self.comb_low.iter_mut().for_each(jump);
        self.comb_high.iter_mut().for_each(jump);
        self.predelay.jump();
    }

    fn wet_target(&self) -> f32 {
        mix_gain(self.last[2])
    }

    fn dry_target(&self) -> f32 {
        if self.last[3] >= 0.5 { 1.0 } else { 0.0 }
    }

    /// `decay_Calc_set`.
    fn set_decay(&mut self, decay: f32) {
        let decay = decay.clamp(0.0, 100.0);
        let t = decay / 100.0;
        let hz = 300.0 - 290.0 * t.powf(1.6);
        let hp = design(Design::HighPass, hz, 0.0, self.sample_rate);
        self.output_hp.iter_mut().for_each(|f| f.set(hp));
        self.input.set(if decay > 3.0 {
            1.0
        } else {
            0.1 + 0.9 * (decay / 3.0).powi(3)
        });
        let t60 = 3000.0 * t.powf(1.7);
        let t60_low = 0.78 * t60;
        for (i, &(ms, g)) in COMB_DECAY.iter().enumerate() {
            self.comb_high[i].set(decay_gain(g, ms, t60));
            self.comb_low[i].set(decay_gain(g, ms, t60_low));
        }
        let (ms, g) = EARLY_DECAY;
        self.early_gain[0].set(decay_gain(g, ms, t60_low));
        self.early_gain[1].set(decay_gain(g, ms, t60));
    }

    /// `tone_edit`: the type-8 shelf of every loop (C80..82).
    fn set_tone(&mut self, tone: f32) {
        let gain = table(&TONE, 101, tone.clamp(0.0, 100.0));
        let shelf = design(Design::HighShelf, 16_000.0, gain, self.sample_rate);
        self.early_tone.iter_mut().for_each(|f| f.set(shelf));
        self.combs.set_tone(shelf);
    }
}

/// `mix_edit`: 0 at Mix 0, otherwise the 11-point table.
fn mix_gain(mix: f32) -> f32 {
    if mix <= 0.0 {
        0.0
    } else {
        table(&MIX, 101, mix.min(100.0))
    }
}

/// `predelay_edit`: PreD x 441 / 10 samples (integer division), one sample
/// at 0.
fn predelay_samples(ms: f32, sample_rate: f32) -> f32 {
    let at_44k = if ms <= 0.0 {
        1.0
    } else {
        (ms.min(200.0) * 44.1).floor().max(1.0)
    };
    at_44k * sample_rate / 44_100.0
}

impl Effect for Ambience {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let [decay, tone, mix, predelay, dry] =
            [values[0], values[1], values[2], values[3], values[4]];
        if decay != self.last[0] {
            self.last[0] = decay;
            self.set_decay(decay);
        }
        if tone != self.last[1] {
            self.last[1] = tone;
            self.set_tone(tone);
        }
        self.last[2] = mix;
        self.wet.set(mix_gain(mix));
        self.last[3] = dry;
        self.dry.set(if dry >= 0.5 { 1.0 } else { 0.0 });
        self.predelay
            .set(predelay_samples(predelay, self.sample_rate).max(1.0));
    }

    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        // Input EQ (also the dry path) and the injection filters.
        let mut eq = [0.0; 2];
        let mut injection = 0.0;
        for (c, &x) in frame.iter().enumerate() {
            let [first, second] = &mut self.input_eq[c];
            eq[c] = second.process(first.process(x));
            injection += self.input_lp[c].process(self.input_hp[c].process(eq[c]));
        }
        let injection = injection * self.input.tick();

        // Pre-delay taps: C5 (tap1) and C6 = C5 + 76 (tap0).
        self.predelay.tick();
        let pre1 = self.predelay.read(&self.injection, 0.0);
        let pre0 = self.predelay.read(&self.injection, self.second_tap);
        self.injection.write(injection);

        // Cross-coupled early loop.
        let gains = [self.early_gain[0].tick(), self.early_gain[1].tick()];
        let delayed = [self.early.out(0), self.early.out(1)];
        let mut early = [0.0; 2];
        for (side, &d) in delayed.iter().enumerate() {
            let lp = &mut self.early_lp[side];
            *lp = super::fast::flush(*lp + self.crossover * (d - *lp));
            let v = gains[0] * *lp + gains[1] * (d - *lp);
            early[side] = self.early_shelf[side].process(self.early_tone[side].process(v));
        }
        // early[0] (from C55 via 4631) + tap0 goes to C56; early[1] + tap1 to C55.
        let v139 = early[0] + pre0;
        let v137 = early[1] + pre1;
        self.early.write(0, v137);
        self.early.write(1, v139);
        self.early.advance();

        // Diffusers: v137 -> right chain, v139 -> left chain.
        let (g, c, h) = DIFFUSION;
        let mut diffused = [v137, v139];
        let mut stages = self.diffusers.frame();
        for (chain, x) in diffused.iter_mut().enumerate() {
            for stage in 0..6 {
                *x = diffuse(&mut stages, chain * 6 + stage, *x, g, c, h);
            }
        }
        self.diffusers.advance();

        // Output taps are read before this frame's comb writes.
        let mut sums = [0.0; 2];
        for (side, sum) in sums.iter_mut().enumerate() {
            for (k, &tap) in self.output_taps[side].iter().enumerate() {
                *sum += self.combs.lines.tap(side * 4 + k, tap);
            }
        }
        let low = [
            self.comb_low[0].tick(),
            self.comb_low[1].tick(),
            self.comb_low[2].tick(),
            self.comb_low[3].tick(),
        ];
        let high = [
            self.comb_high[0].tick(),
            self.comb_high[1].tick(),
            self.comb_high[2].tick(),
            self.comb_high[3].tick(),
        ];
        self.combs.run(
            std::array::from_fn(|i| COMB_INPUT * diffused[i / 4]),
            self.crossover,
            std::array::from_fn(|i| low[i % 4]),
            std::array::from_fn(|i| high[i % 4]),
        );

        let wet = self.wet.tick();
        let dry = self.dry.tick();
        let pre = [pre1, pre0];
        let mut out = [0.0; 2];
        for side in 0..2 {
            let x = DIRECT * pre[side] + TAP_GAIN[side] * sums[side] + diffused[side];
            let x = self.output_hp[side].process(self.output_eq[side].process(x));
            out[side] = wet * x + dry * eq[side];
        }
        out
    }

    fn clear(&mut self) {
        for side in 0..2 {
            self.input_eq[side].iter_mut().for_each(Biquad::clear);
            self.input_hp[side].clear();
            self.input_lp[side].clear();
            self.early_lp[side] = 0.0;
            self.early_tone[side].clear();
            self.early_shelf[side].clear();
            self.output_eq[side].clear();
            self.output_hp[side].clear();
        }
        self.injection.clear();
        self.early.clear();
        self.diffusers.clear();
        self.combs.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Values {
        let mut values = [0.0; crate::MAX_PARAMS];
        for (value, param) in values.iter_mut().zip(INFO.params) {
            *value = f32::from(param.default);
        }
        values
    }

    #[test]
    fn editor_tables_match_the_rom() {
        assert_eq!(mix_gain(0.0), 0.0);
        assert!((mix_gain(100.0) - 1.995_262).abs() < 1e-6);
        assert!((mix_gain(50.0) - 0.479_107).abs() < 1e-6);
        assert!((table(&TONE, 101, 80.0) + 7.0).abs() < 1e-6);
        assert!((predelay_samples(29.0, 44_100.0) - 1278.0).abs() < 0.01);
        assert_eq!(predelay_samples(0.0, 44_100.0), 1.0);
    }

    #[test]
    fn decay_sets_rt60_gains() {
        let mut effect = Ambience::new(44_100.0);
        let mut values = defaults();
        values[0] = 100.0;
        effect.set(&values, 120.0);
        effect.settle();
        // T60 = 3 s at Decay 100: the 82.64 ms comb loses 0.00124 over 3 s.
        let high = effect.comb_high[0].current;
        let expected = 0.001_238_05f32.powf(82.64 / 3000.0);
        assert!((high - expected).abs() < 1e-6);
        assert!(effect.comb_low[0].current < high);
        // Decay 0 opens nothing and fades the injection to 0.1.
        values[0] = 0.0;
        effect.set(&values, 120.0);
        effect.settle();
        assert_eq!(effect.comb_high[0].current, 0.0);
        assert!((effect.input.current - 0.1).abs() < 1e-6);
    }

    #[test]
    fn dry_passes_through_the_input_eq_and_wet_follows_mix() {
        let mut effect = Ambience::new(48_000.0);
        let mut values = defaults();
        values[2] = 0.0;
        effect.set(&values, 120.0);
        effect.settle();
        // Mix 0: only the EQ'd dry signal, close to unity for a 1 kHz tone.
        let mut peak = 0.0f32;
        for i in 0..48_000 {
            let x = (i as f32 * 1000.0 / 48_000.0 * std::f32::consts::TAU).sin() * 0.5;
            let y = effect.process([x, x]);
            if i > 24_000 {
                peak = peak.max(y[0].abs());
            }
        }
        assert!((0.4..0.6).contains(&peak), "{peak}");
        // Dry off, Mix 100: a decaying tail and no dry signal after the pre-delay.
        values[2] = 100.0;
        values[4] = 0.0;
        effect.clear();
        effect.set(&values, 120.0);
        effect.settle();
        let first = effect.process([1.0, 1.0]);
        assert!(first[0].abs() < 1e-3, "{first:?}");
        let mut energy_early = 0.0;
        let mut energy_late = 0.0;
        for i in 0..96_000 {
            let y = effect.process([0.0, 0.0]);
            let e = y[0] * y[0] + y[1] * y[1];
            if i < 24_000 {
                energy_early += e;
            } else if i > 72_000 {
                energy_late += e;
            }
            assert!(y[0].is_finite() && y[0].abs() < 4.0);
        }
        assert!(energy_early > 1e-3, "{energy_early}");
        assert!(
            energy_late < energy_early * 1e-2,
            "{energy_late} {energy_early}"
        );
    }

    #[test]
    fn predelay_moves_the_first_reflection() {
        let first_arrival = |ms: f32| {
            let mut effect = Ambience::new(44_100.0);
            let mut values = defaults();
            values[3] = ms;
            values[4] = 0.0;
            values[2] = 100.0;
            effect.set(&values, 120.0);
            effect.settle();
            effect.process([1.0, 1.0]);
            (1..20_000)
                .find(|_| effect.process([0.0, 0.0])[0].abs() > 1e-3)
                .unwrap()
        };
        let short = first_arrival(10.0);
        let long = first_arrival(110.0);
        let difference = long as f32 - short as f32;
        assert!((difference - 4410.0).abs() < 3.0, "{short} {long}");
    }

    #[test]
    fn stays_bounded_at_maximum_decay() {
        let mut effect = Ambience::new(96_000.0);
        let values = [100.0, 100.0, 100.0, 200.0, 1.0, 0.0, 0.0, 0.0, 0.0];
        effect.set(&values, 120.0);
        let mut peak = 0.0f32;
        for i in 0..96_000 * 3 {
            let x = if i < 96_000 {
                ((i * 7) % 13) as f32 / 13.0 - 0.5
            } else {
                0.0
            };
            let y = effect.process([x, -x]);
            peak = peak.max(y[0].abs()).max(y[1].abs());
        }
        assert!(peak.is_finite() && peak < 8.0, "{peak}");
    }
}
