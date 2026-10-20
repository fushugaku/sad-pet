//! Rustias filter adapter at 91cae3f. The original fixed-point Filter 1,
//! Filter 2 and Comb come directly from radias-synth-domain; the expanded
//! f64 kernels live in models.rs. All delay memory is prepared off-thread.
mod models;
use std::cell::RefCell;

#[derive(Clone, Copy)]
struct Prewarped {
    coefficient: i32,
    rate: f64,
    cutoff: f64,
    gain: f64,
}
thread_local! {
    // The upstream pure-coefficient cache, extended with the host rate key.
    // No voice history, allocation, locks or approximation in this cache.
    static PREWARPED: RefCell<[Option<Prewarped>; 256]> = const { RefCell::new([None;256]) };
}
fn prewarped(coefficient: i32, rate: f64) -> Prewarped {
    PREWARPED.with_borrow_mut(|cache| {
        let index = (coefficient as u32).wrapping_mul(0x9e37_79b1) >> 24;
        let slot = &mut cache[index as usize];
        if let Some(value) = *slot
            && value.coefficient == coefficient
            && value.rate == rate
        {
            return value;
        }
        let cutoff = ((coefficient as f64 / 2147483648.0)
            .clamp(0.0001, 0.99)
            .asin()
            * rate
            / std::f64::consts::PI)
            .clamp(12.0, 20_000.0_f64.min(rate * 0.45));
        let value = Prewarped {
            coefficient,
            rate,
            cutoff,
            gain: (std::f64::consts::PI * cutoff / (rate * 2.0)).tan(),
        };
        *slot = Some(value);
        value
    })
}

use crate::{
    Stereo,
    preset::{FilterModel, FilterType},
};
use radias_synth_domain::{
    Sample,
    comb::Comb,
    controller_comb::{CombControlTables, CombResonanceControl},
    controller_filter::ControllerFilterTables,
    filter::{FilterCoefficients, ResonantFilter},
    filter_control::{self, FilterMixTable},
    filter_routing::{Filter2, Filter2Coefficients, Filter2Output},
};

pub(super) struct RustiasFilter {
    rate: f64,
    frequencies: ControllerFilterTables,
    mix: FilterMixTable,
    comb_tables: CombControlTables,
    first: [ResonantFilter; 2],
    second: [Filter2; 2],
    comb: Box<[Comb; 2]>,
    models: [models::Model; 2],
    coefficients: FilterCoefficients,
    comb_delay: u32,
    comb_feedback: i32,
    model: FilterModel,
    kind: FilterType,
    cutoff: f64,
    resonance: f64,
    drive: f64,
    morph: f64,
    gain: f64,
    control_key: Option<(i32, u64)>,
}

impl RustiasFilter {
    pub fn new(sample_rate: u32) -> Self {
        let rate = sample_rate as f64;
        Self {
            rate,
            // Firmware-free analytic tables from standalone_tables.rs. Bound
            // the original fixed-point integrator to its supported rate ratio.
            frequencies: ControllerFilterTables {
                frequency: std::array::from_fn(|i| {
                    let hz = (30.0 * 200.0_f64.powf((i as f64 - 36.0) / 127.0))
                        .clamp(5.0, 10_000.0)
                        .min(rate * (10_000.0 / 48_000.0));
                    ((std::f64::consts::PI * hz / rate).sin() * 2147483647.0) as u32
                }),
                key_depth: [0; 128], // SAD PET composes per-voice tracking before this adapter.
            },
            mix: FilterMixTable {
                weights: std::array::from_fn(|r| {
                    std::array::from_fn(|i| {
                        let points = [
                            [32767, 0, 0, 0, 0],
                            [0, 0, 0, 32767, 0],
                            [0, 0, 0, 0, 32767],
                            [0, 32767, 0, 0, 0],
                            [0, 0, 32767, 0, 0],
                        ];
                        let x = i as f64 / 32.0;
                        let a = (x.floor() as usize).min(3);
                        let f = x - a as f64;
                        ((1.0 - f) * points[a][r] as f64 + f * points[a + 1][r] as f64).round()
                            as i16
                    })
                }),
            },
            comb_tables: CombControlTables {
                delays: std::array::from_fn(|i| {
                    ((rate / (30.0 * 200.0_f64.powf(i as f64 / 127.0))).clamp(2.0, 4000.0)
                        * 65536.0) as u32
                }),
                feedback: std::array::from_fn(|i| (i as f64 / 127.0 * 0.98 * 2147483647.0) as u32),
            },
            first: [ResonantFilter::default(); 2],
            second: [Filter2::default(); 2],
            comb: Box::new(std::array::from_fn(|_| Comb {
                feedback: Default::default(),
                delay: Default::default(),
            })),
            models: std::array::from_fn(|_| models::Model::new(rate)),
            coefficients: FilterCoefficients {
                input_gain: 24576,
                feedback: 0,
                integrator_gain: 0,
                post_gain: 32767,
                post_feedback: 0,
                mix: [32767, 0, 0, 0, 0],
            },
            comb_delay: 0,
            comb_feedback: 0,
            model: FilterModel::Radias,
            kind: FilterType::LowPass,
            cutoff: 30.0,
            resonance: 0.0,
            drive: 0.0,
            morph: 0.0,
            gain: 0.0,
            control_key: None,
        }
    }

    pub fn clear(&mut self) {
        self.first = [ResonantFilter::default(); 2];
        self.second = [Filter2::default(); 2];
        self.models = std::array::from_fn(|_| models::Model::new(self.rate));
        // Only Comb can have written its memory. Do not zero unused delay
        // buffers on every polyphonic note in a different filter model.
        if self.model == FilterModel::Radias2 && self.kind == FilterType::Comb {
            for comb in self.comb.iter_mut() {
                comb.delay.samples.fill(0);
                comb.delay.pending.fill(0);
                comb.delay.read_samples.fill(0);
                comb.delay.fraction = 0;
                comb.delay.write_cursor_bytes = 0;
                comb.delay.group_phase = 0;
                comb.feedback = Default::default();
            }
        }
    }

    pub fn configure(&mut self, model: FilterModel, kind: FilterType, values: [f32; 4]) {
        if self.model != model || (model == FilterModel::Radias2 && self.kind != kind) {
            self.clear();
        }
        self.model = model;
        self.kind = kind;
        let [cutoff, resonance, drive, morph] = values.map(|v| f64::from(v.clamp(0.0, 1.0)));
        let code = (cutoff * 127.0 * 256.0).round() as i32;
        let key = (code, resonance.to_bits());
        if self.control_key != Some(key) {
            let frequency = self.frequencies.frequency(code) as i32;
            let c = filter_control::compile(
                frequency,
                (resonance * 0.95 * 2147483647.0) as i32,
                0x4000_0000,
            );
            self.coefficients.feedback = c.feedback;
            self.coefficients.integrator_gain = c.integrator_gain;
            self.coefficients.post_gain = c.post_gain;
            self.coefficients.post_feedback = c.post_feedback;
            self.comb_delay = self.comb_tables.delay(code);
            self.comb_feedback = self.comb_tables.compile_feedback(
                code,
                CombResonanceControl {
                    resonance: (resonance * 127.0).round() as u8,
                    ..Default::default()
                },
            ) as i32;
            self.control_key = Some(key);
        }
        self.coefficients.mix = self.mix.weights((morph * 127.0).round() as u16 * 256);
        if !matches!(model, FilterModel::Radias | FilterModel::Radias2) {
            let prewarped = prewarped(self.coefficients.integrator_gain, self.rate);
            self.cutoff = prewarped.cutoff;
            self.gain = prewarped.gain;
        }
        self.resonance = resonance;
        self.drive = drive;
        self.morph = morph;
    }

    pub fn process(&mut self, frame: Stereo) -> Stereo {
        use FilterModel::{Radias, Radias2};
        std::array::from_fn(|channel| {
            let input = f64::from(frame[channel]);
            if !matches!(self.model, Radias | Radias2) {
                return self.models[channel]
                    .sample(
                        input,
                        self.model as u8 - 1,
                        self.cutoff,
                        self.resonance,
                        self.drive,
                        self.morph,
                        self.gain,
                    )
                    .clamp(-1.0, 1.0) as f32;
            }
            let input = if self.drive == 0.0 {
                input
            } else {
                models::soft(input * (1.0 + self.drive * 11.0))
            };
            let input = Sample((input.clamp(-1.0, 1.0) * 2147483647.0) as i32);
            let output = if self.model == Radias {
                self.first[channel].next_sample(input, self.coefficients)
            } else if self.kind == FilterType::Comb {
                self.comb[channel].sample(input, self.comb_feedback, self.comb_delay)
            } else {
                let c = self.coefficients;
                self.second[channel].sample(
                    input,
                    Filter2Coefficients {
                        input_gain: c.input_gain,
                        feedback: c.feedback,
                        integrator_gain: c.integrator_gain,
                        output: match self.kind {
                            FilterType::LowPass => Filter2Output::LowPass,
                            FilterType::BandPass => Filter2Output::BandPass,
                            FilterType::HighPass => Filter2Output::HighPass,
                            FilterType::Comb => unreachable!(),
                        },
                    },
                )
            };
            (output.0 as f64 / 2147483648.0) as f32
        })
    }
}
