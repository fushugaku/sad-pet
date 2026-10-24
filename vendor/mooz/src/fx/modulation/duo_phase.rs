//! DuoPhase (DUOPHASE.ZDL), from the recovered `Fx_MOD_DuoPhase` callback
//! (`duo_phase.rs`, `duo_phase_parameters.rs`) and its decoded editors.
//!
//! Callback, per sample: two LFOs make `0.5 + 2 t (1 − |t|)` waves (t the
//! centred triangle), each smoothed by a one-pole of 7.14e-4 per sample. Bank
//! A and bank B are six first-order all-passes whose coefficient is
//! `offset + depth × wave` (c8/c7, c14/c13); each bank's input is
//! `0.7 × source + 0.95 × Reso × its last output` and its result is
//! `source + all-pass output` (c9, c15). Link: Seri feeds A from the mean and B
//! from A's result one sample later (B's result on both outputs), Para feeds
//! both from the mean and sums them × 0.7, STR runs A on the left input and B
//! on the right. RateB SyncA lets B follow wave A, RvrsA follows `1 − wave A`
//! (c18, c17). Level (c2) and the wet fade c3 scale the outputs; no dry path.
//!
//! Editors: RateA/B 0 → increment 0x22f2, 1..49 → `context[22]`
//! 0x45e6..0xb9af6 square law (as Phaser); RateA above 49 tempo notes, RateB
//! 50/51 = SyncA/RvrsA. ResoA/B 0..10 → 0.5..0.9 (ramp 705.6 samples).
//! DPT_A/B 0..99 → depth `(0.05 + 0.95 n/99) × (0.374 − 0.972)` from
//! `DuoPhase_DptOfst_tbl` with offset 0.972 (ramp 705.6). Link from `Link_tbl`
//! with a wet fade out/in. Level 0..1.5. All editors decoded; the 44.1 kHz
//! all-pass coefficients are re-warped to the host rate.
use super::common::{
    AllpassWarp, Lfo, Ramp, allpass, centered_triangle, guard, increment_hz, one_pole_at, rate_hz,
    stepped,
};
use super::phaser::rate_increment;
use crate::zoom::DEFAULT_STEP;
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_duo_phase",
    name: "DuoPhase",
    category: Category::Modulation,
    zdl: "DUOPHASE.ZDL",
    description: "Two phasers in series, parallel or stereo",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "RateA",
            max: 77,
            default: 46,
            display: Display::sync_rate(49, 1.0),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "RateB",
            max: 51,
            default: 5,
            display: Display::labels(&RATE_B_LABELS),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "ResoA",
            max: 10,
            default: 0,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "ResoB",
            max: 10,
            default: 6,
            display: Display::INT,
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "Link",
            max: 2,
            default: 0,
            display: Display::labels(&["Seri", "Para", "STR"]),
        },
        // values[6] is firmware parameter word 8
        ParamInfo {
            name: "DPT_A",
            max: 99,
            default: 36,
            display: Display::offset(1.0),
        },
        // values[7] is firmware parameter word 9
        ParamInfo {
            name: "DPT_B",
            max: 99,
            default: 62,
            display: Display::offset(1.0),
        },
    ],
};

/// `GetString_DuoRateB`: 1..50, then SyncA and RvrsA.
const RATE_B_LABELS: [&str; 52] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16", "17",
    "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30", "31", "32", "33",
    "34", "35", "36", "37", "38", "39", "40", "41", "42", "43", "44", "45", "46", "47", "48", "49",
    "50", "SyncA", "RvrsA",
];

/// `DuoPhase_DptOfst_tbl`: coefficient at rest and at full sweep.
const REST: f32 = 0.971_792_2;
const FULL: f32 = 0.374_037_6;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Link {
    Series,
    Parallel,
    Stereo,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Follow {
    Own,
    SyncA,
    ReverseA,
}

struct Bank {
    denormals: super::common::Denormals,
    stages: [f32; 6],
    last: f32,
    smoothed: f32,
    reso: Ramp,
    depth: Ramp,
}

impl Bank {
    fn new(sample_rate: f32) -> Self {
        Self {
            denormals: Default::default(),
            stages: [0.0; 6],
            last: 0.0,
            smoothed: 0.0,
            reso: Ramp::with_duration(0.5, 705.6, sample_rate),
            depth: Ramp::with_duration(0.0, 705.6, sample_rate),
        }
    }

    #[inline]
    fn process(&mut self, source: f32, weight: f32) -> f32 {
        let mut value = 0.95 * self.reso.tick() * self.last + 0.7 * source;
        for memory in &mut self.stages {
            value = allpass(memory, value, weight);
        }
        self.last = value;
        self.denormals.sweep(&mut self.stages);
        source + value
    }
}

struct DuoPhase {
    sample_rate: f32,
    warp: AllpassWarp,
    lfos: [Lfo; 2],
    smoothing: f32,
    banks: [Bank; 2],
    follow: Follow,
    link: Link,
    pending_link: Link,
    linked: f32,
    fade: Ramp,
    level: Ramp,
    first: bool,
    rates: [f32; 2],
    bpm: f32,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(DuoPhase {
        sample_rate,
        warp: AllpassWarp::new(sample_rate),
        lfos: [Lfo::default(); 2],
        smoothing: one_pole_at(7.138_252e-4, sample_rate),
        banks: [Bank::new(sample_rate), Bank::new(sample_rate)],
        follow: Follow::Own,
        link: Link::Series,
        pending_link: Link::Series,
        linked: 0.0,
        fade: Ramp::with_duration(1.0, 705.6, sample_rate),
        level: Ramp::with_step(1.0, DEFAULT_STEP, sample_rate),
        first: true,
        rates: [f32::NAN; 2],
        bpm: 0.0,
    })
}

#[inline]
fn wave(s: f32) -> f32 {
    let t = centered_triangle(s);
    0.5 + 2.0 * t * (1.0 - t.abs())
}

impl Effect for DuoPhase {
    fn set(&mut self, values: &Values, bpm: f32) {
        let [
            rate_a,
            rate_b,
            level,
            reso_a,
            reso_b,
            link,
            depth_a,
            depth_b,
            _,
        ] = *values;
        if [rate_a, rate_b] != self.rates || bpm != self.bpm {
            let hz = rate_hz(rate_a, 49, bpm, |v| {
                increment_hz(stepped(v, rate_increment))
            });
            self.lfos[0].set_hz(hz, self.sample_rate);
            let b = rate_b.clamp(0.0, 51.0);
            self.follow = match b.round() as u32 {
                50 => Follow::SyncA,
                51 => Follow::ReverseA,
                _ => Follow::Own,
            };
            let hz = increment_hz(stepped(b.min(49.0), rate_increment));
            self.lfos[1].set_hz(hz, self.sample_rate);
            self.rates = [rate_a, rate_b];
            self.bpm = bpm;
        }
        for (bank, (reso, depth)) in self
            .banks
            .iter_mut()
            .zip([(reso_a, depth_a), (reso_b, depth_b)])
        {
            bank.reso.set(0.5 + 0.04 * reso.clamp(0.0, 10.0));
            let t = depth.clamp(0.0, 99.0) / 99.0;
            bank.depth.set((0.05 + 0.95 * t) * (FULL - REST));
        }
        self.pending_link = match link.round() as i32 {
            i32::MIN..=0 => Link::Series,
            1 => Link::Parallel,
            _ => Link::Stereo,
        };
        self.level.set(level.clamp(0.0, 150.0) * 0.01);
        if self.first {
            self.first = false;
            self.link = self.pending_link;
            self.level.jump(self.level.target);
            for bank in &mut self.banks {
                bank.reso.jump(bank.reso.target);
                bank.depth.jump(bank.depth.target);
            }
        } else if self.pending_link != self.link {
            self.fade.set(0.0);
        }
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let fade = self.fade.tick();
        if fade == 0.0 && self.fade.target == 0.0 {
            self.link = self.pending_link;
            self.fade.set(1.0);
        }
        let wave_a = wave(self.lfos[0].tick());
        let wave_b = wave(self.lfos[1].tick());
        let selected_b = match self.follow {
            Follow::Own => wave_b,
            Follow::SyncA => wave_a,
            Follow::ReverseA => 1.0 - wave_a,
        };
        let [a, b] = &mut self.banks;
        a.smoothed += self.smoothing * (wave_a - a.smoothed);
        b.smoothed += self.smoothing * (selected_b - b.smoothed);
        let weight_a = self.warp.map(REST + a.depth.tick() * a.smoothed);
        let weight_b = self.warp.map(REST + b.depth.tick() * b.smoothed);
        let mean = 0.5 * (left + right);
        let (source_a, source_b) = match self.link {
            Link::Parallel => (mean, mean),
            Link::Series => (mean, self.linked),
            Link::Stereo => (left, right),
        };
        self.linked = a.process(source_a, weight_a);
        let second = b.process(source_b, weight_b);
        let (wet_left, wet_right) = match self.link {
            Link::Parallel => {
                let both = 0.7 * (second + self.linked);
                (both, both)
            }
            Link::Series => (second, second),
            Link::Stereo => (self.linked, second),
        };
        let gain = fade * self.level.tick();
        [guard(wet_left * gain), guard(wet_right * gain)]
    }

    fn clear(&mut self) {
        for bank in &mut self.banks {
            bank.stages = [0.0; 6];
            bank.last = 0.0;
            bank.smoothed = 0.0;
        }
        self.linked = 0.0;
        for lfo in &mut self.lfos {
            lfo.phase = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wave_spans_zero_to_one() {
        assert!((wave(0.0) - 0.0).abs() < 1e-6);
        assert!((wave(1.0) - 1.0).abs() < 1e-3);
        assert!((wave(-0.5) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn stereo_link_keeps_sides_apart() {
        let mut effect = create(48_000.0);
        effect.set(&[10.0, 10.0, 100.0, 5.0, 5.0, 2.0, 50.0, 50.0, 0.0], 120.0);
        let mut right_energy = 0.0;
        for i in 0..48_000 {
            let x = (i as f32 * 0.05).sin();
            let [_, r] = effect.process([x, 0.0]);
            right_energy += r * r;
        }
        assert!(right_energy < 1e-6, "{right_energy}");
    }

    #[test]
    fn defaults_have_a_moving_notch() {
        let mut effect = create(44_100.0);
        effect.set(&crate::Model::DuoPhase.defaults(), 120.0);
        let mut blocks = Vec::new();
        let mut energy = 0.0;
        for i in 0..441_000 {
            let x = (i as f32 * std::f32::consts::TAU * 1_000.0 / 44_100.0).sin() * 0.5;
            let [l, _] = effect.process([x, x]);
            energy += l * l;
            if i % 4_410 == 4_409 {
                blocks.push(energy);
                energy = 0.0;
            }
        }
        let max = blocks.iter().copied().fold(f32::MIN, f32::max);
        let min = blocks.iter().copied().fold(f32::MAX, f32::min);
        assert!(max > 2.0 * min, "{min} {max}");
    }
}
