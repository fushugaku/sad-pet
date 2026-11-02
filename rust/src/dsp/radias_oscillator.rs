//! Rustias OSC1 adapter, based on Rustias circuit/primary.rs at 91cae3f.
//! Public fixed-point kernels and control compilers are pinned dependencies.
//! Generator arithmetic, coefficient transfers and phase initialization stay
//! in the shared synthesis crates; this adapter owns only a module's state.
use radias_synth_application::{
    noise::{NoiseTables, NoiseVoiceControl},
    primary::PrimaryProgram,
};
use radias_synth_domain::{
    Sample,
    bandlimit::BandwidthTable,
    control_slew::SlewWeights,
    noise::{FormantState, NoiseFrameSeeds},
    pitch::{PhaseIncrement, PitchCode},
    primary_oscillator::{PrimaryOscillator, PrimaryParameters},
    waveform::WaveformTable,
};

pub(crate) struct Tables {
    noise: NoiseTables,
    bandwidth: BandwidthTable,
}
impl Tables {
    pub fn new() -> Self {
        Self {
            noise: noise_tables(),
            bandwidth: BandwidthTable { gains: [0; 129] },
        }
    }
}
pub(crate) struct Controller {
    selection: u8,
    current: i16,
    phase_code: Option<u16>,
    noise: Option<NoiseVoiceControl>,
    inputs: Option<ControlInputs>,
    target: i16,
    parameters: Option<PrimaryParameters>,
    /// (configuration epoch, LFO) at the last full input comparison. Inputs
    /// change only with one of these, so an equal key means equal inputs.
    seen: Option<(u64, i16)>,
    reset_correction: f64,
}
// The plan owns immutable tables. A new plan/table identity invalidates the
// cache; oscillator history, feedback and four-frame slew never live here.
#[derive(Clone, Copy, PartialEq, Eq)]
struct ControlInputs {
    program: PrimaryProgram,
    increment: PhaseIncrement,
    code: PitchCode,
    lfo: i16,
    modulation: [i16; 2],
    tables: usize,
}
impl Default for Controller {
    fn default() -> Self {
        Self {
            selection: u8::MAX,
            current: 0,
            phase_code: None,
            noise: None,
            inputs: None,
            target: 0,
            parameters: None,
            seen: None,
            reset_correction: 0.0,
        }
    }
}
impl Controller {
    #[cfg(test)]
    #[allow(clippy::too_many_arguments)]
    #[inline(always)]
    pub fn next_with_sync(
        &mut self,
        oscillator: &mut PrimaryOscillator,
        program: PrimaryProgram,
        increment: PhaseIncrement,
        code: PitchCode,
        lfo: i16,
        modulation: [i16; 2],
        modulator: Sample,
        tables: &Tables,
        waveform: &WaveformTable,
        frame: u64,
        id: usize,
        sync: bool,
        epoch: u64,
    ) -> Sample {
        self.next_with_phase_reset(
            oscillator, program, increment, code, lfo, modulation, modulator, tables, waveform,
            frame, id, sync, epoch, 0,
        )
    }
    #[cfg(test)]
    #[allow(clippy::too_many_arguments)]
    #[inline(always)]
    pub fn next_with_phase_reset(
        &mut self,
        oscillator: &mut PrimaryOscillator,
        program: PrimaryProgram,
        increment: PhaseIncrement,
        code: PitchCode,
        lfo: i16,
        modulation: [i16; 2],
        modulator: Sample,
        tables: &Tables,
        waveform: &WaveformTable,
        frame: u64,
        id: usize,
        sync: bool,
        epoch: u64,
        reset_phase: u32,
    ) -> Sample {
        self.next_with_fractional_reset(
            oscillator,
            program,
            increment,
            code,
            lfo,
            modulation,
            modulator,
            tables,
            waveform,
            frame,
            id,
            sync,
            epoch,
            reset_phase,
            None,
        )
    }
    #[allow(clippy::too_many_arguments)]
    #[inline(always)]
    pub fn next_with_fractional_reset(
        &mut self,
        oscillator: &mut PrimaryOscillator,
        mut program: PrimaryProgram,
        increment: PhaseIncrement,
        code: PitchCode,
        lfo: i16,
        modulation: [i16; 2],
        modulator: Sample,
        tables: &Tables,
        waveform: &WaveformTable,
        frame: u64,
        id: usize,
        sync: bool,
        epoch: u64,
        reset_phase: u32,
        fraction: Option<f64>,
    ) -> Sample {
        let fresh = self.selection != program.selection;
        if fresh {
            self.selection = program.selection;
            self.current = 0;
            self.phase_code = None;
            *oscillator = PrimaryOscillator::default();
            oscillator.phase = NoiseFrameSeeds::from_inputs(id as i16 + 1, 0).primary[0];
            oscillator.initialize_waveform_phase(program.selection);
            oscillator.formant = FormantState {
                counter: tables.noise.counters.for_slot(id as u8),
                ..Default::default()
            };
            self.noise = tables.noise.compile(program, code, lfo, modulation);
            self.inputs = None;
            self.parameters = None;
            self.seen = None;
        }
        let inputs = (self.seen != Some((epoch, lfo))).then_some(ControlInputs {
            program,
            increment,
            code,
            lfo,
            modulation,
            tables: tables as *const Tables as usize,
        });
        self.seen = Some((epoch, lfo));
        let changed = inputs.is_some() && self.inputs != inputs;
        let weights = SlewWeights {
            target: 0x1d4,
            memory: 0x7e2d,
        };
        if let Some(noise) = &mut self.noise {
            // Controls use the same compiler and four-frame slewing as native OSC1.
            if changed {
                tables
                    .noise
                    .update_controls(noise, program.control, code, lfo, modulation);
                let coefficient = if program.selection == 4 {
                    tables.noise.noise.curve_scale(code)
                } else {
                    radias_synth_domain::noise_control::formant_frequency(increment)
                };
                noise.receive_pitch(increment, coefficient);
                self.inputs = inputs;
            }
            if fresh {
                noise.initialize_targets();
            }
            if frame & 3 == 3 {
                noise.advance(weights);
            }
            if sync {
                oscillator.phase = radias_synth_domain::Phase(reset_phase);
            }
            return oscillator.next_with_modulator_and_bias(
                waveform,
                noise.parameters(),
                Sample(0),
                noise.current()[1],
            );
        }
        if changed {
            self.target = program.waveform_control(lfo, modulation).unwrap_or(0);
        }
        let target = self.target;
        if frame & 3 == 3 {
            self.current = weights.word(self.current, target);
        }
        program.control.control1_modulation = modulation[0];
        program.control.control2_modulation = modulation[1];
        if changed {
            let mut parameters = program
                .compile_waveform(increment, tables.bandwidth.coefficient(increment))
                .unwrap();
            match &mut parameters {
                PrimaryParameters::Ramp(p) | PrimaryParameters::Pulse(p) => {
                    p.offset_target = target
                }
                PrimaryParameters::Sine(p) => p.control[1] = target,
                PrimaryParameters::Unison(p) => {
                    p.detune = target.max(0) as u16;
                    p.retune(increment);
                }
                PrimaryParameters::UnisonCarrier(p) => {
                    p.parameters.detune = target.max(0) as u16;
                    p.parameters.retune(increment);
                }
                PrimaryParameters::Vpm(p) => p.ratio = program.control.vpm_ratio(),
                PrimaryParameters::VpmCarrier(p) => p.modulator.ratio = program.control.vpm_ratio(),
                _ => {}
            }
            self.parameters = Some(parameters);
            self.inputs = inputs;
        }
        // Patch the slewed control in place: the field is rewritten before
        // every use, so the stored parameters stay equivalent to a fresh copy.
        let parameters = self.parameters.as_mut().unwrap();
        match parameters {
            PrimaryParameters::Triangle(p) => p.edge_gain = self.current,
            PrimaryParameters::Sine(p) => p.modulation_gain = self.current,
            PrimaryParameters::Cross(p) => p.modulation_gain = self.current,
            PrimaryParameters::CrossTriangle(p) => p.modulation_gain = self.current,
            PrimaryParameters::CrossSine(p) => p.modulation_gain = self.current,
            PrimaryParameters::Vpm(p) => p.modulation_gain = self.current,
            PrimaryParameters::VpmCarrier(p) => p.modulator.modulation_gain = self.current,
            PrimaryParameters::Noise(_) | PrimaryParameters::Formant(_) => unreachable!(),
            _ => {}
        }
        if program.selection & 48 == 32 {
            let code = program.control.phase_code();
            if self.phase_code != Some(code) {
                if let Some(phases) = radias_synth_domain::unison_pitch::unison_phases(
                    code,
                    program.selection & 3 == 2,
                ) {
                    oscillator.unison.phases = phases;
                    oscillator.phase = phases[0];
                }
                self.phase_code = Some(code);
            }
        }
        let before = *oscillator;
        if sync {
            // Shift the whole Unison bank together; preserve its phase spread.
            let reset_phase = reset_phase
                .wrapping_add(fraction.map_or(0, |f| (increment.0 as f64 * f).round() as u32));
            let base = oscillator.unison.phases[0].0;
            for phase in &mut oscillator.unison.phases {
                phase.0 = phase.0.wrapping_sub(base).wrapping_add(reset_phase);
            }
            oscillator.phase = radias_synth_domain::Phase(reset_phase);
            oscillator.modulated_phase =
                radias_synth_domain::Phase(reset_phase.wrapping_add(if program.selection == 3 {
                    0xc000_0000
                } else {
                    0
                }));
            oscillator.control_feedback = 0;
        }
        let sample = oscillator.next_with_modulator(waveform, *parameters, modulator);
        if let Some(at) = fraction.filter(|_| sync) {
            let mut old = before;
            let old = old.next_with_modulator(waveform, *parameters, modulator).0 as f64;
            let jump = sample.0 as f64 - old;
            let left = 1.0 - at;
            let value = old + self.reset_correction + jump * 0.5 * left * left;
            self.reset_correction = -jump * 0.5 * at * at;
            Sample(value.round().clamp(i32::MIN as f64, i32::MAX as f64) as i32)
        } else if self.reset_correction != 0.0 {
            let value = sample.0 as f64 + self.reset_correction;
            self.reset_correction = 0.0;
            Sample(value.round().clamp(i32::MIN as f64, i32::MAX as f64) as i32)
        } else {
            sample
        }
    }
}

/// The fixed-point kernels count down. The interaction graph uses forward
/// cycles for every backend, with phase zero at the same reset anchor.
pub(crate) fn phase(oscillator: &PrimaryOscillator) -> f64 {
    (-((oscillator.phase.0 as f64) / 4294967296.0)).rem_euclid(1.0)
}
pub(crate) fn phase_word(phase: f64) -> u32 {
    (phase.rem_euclid(1.0) * 4294967296.0).round() as u32
}
pub(crate) fn reset_word(phase: f64) -> u32 {
    phase_word(phase).wrapping_neg()
}

#[cfg(test)]
mod phase_tests {
    use super::*;
    #[test]
    fn fractional_reset_keeps_the_remainder_in_forward_phase_coordinates() {
        let mut controller = Controller::default();
        let mut oscillator = PrimaryOscillator::default();
        let tables = Tables::new();
        let wave = waveform_table();
        let increment = PhaseIncrement(0x01000000);
        let program = PrimaryProgram {
            selection: 0,
            ..Default::default()
        };
        let code = PitchCode::new(60 * 256).unwrap();
        controller.next_with_sync(
            &mut oscillator,
            program,
            increment,
            code,
            0,
            [0; 2],
            Sample(0),
            &tables,
            &wave,
            0,
            1,
            false,
            0,
        );
        controller.next_with_fractional_reset(
            &mut oscillator,
            program,
            increment,
            code,
            0,
            [0; 2],
            Sample(0),
            &tables,
            &wave,
            1,
            1,
            true,
            0,
            reset_word(0.25),
            Some(0.75),
        );
        let expected = 0.25 + (increment.0 as f64 / 4294967296.0) * 0.25;
        assert!((phase(&oscillator) - expected).abs() < 1e-8);
    }
}

// Analytic tables from Rustias standalone_tables.rs; no firmware/ROM data.
fn noise_tables() -> NoiseTables {
    use radias_synth_domain::{
        controller_noise::FormantCounterSeeds, noise_control::NoisePitchTable, pitch::PitchTable,
    };
    NoiseTables {
        pitch: PitchTable {
            notes: core::array::from_fn(|i| {
                (440.0 * 2.0f64.powf((i as f64 - 69.0) / 12.0) * 4294967296.0 / 48000.0).round()
                    as u32
            }),
            fractions: core::array::from_fn(|i| {
                ((2.0f64.powf(i as f64 / 1536.0) - 1.0) * 32768.0).round() as i16
            }),
        },
        noise: NoisePitchTable {
            curve_scales: core::array::from_fn(|i| {
                (16384.0 * 2.0f64.powf((60.0 - i as f64) / 24.0)).clamp(256.0, 32767.0) as i16
            }),
        },
        counters: FormantCounterSeeds {
            values: core::array::from_fn(|i| ((i as u32 * 2053 + 97) & 32767) as i16),
        },
    }
}
pub(crate) fn waveform_table() -> WaveformTable {
    WaveformTable {
        correction: [0; 129],
        shapers: radias_synth_domain::waveshaper::ShaperTables {
            sub_edges: core::array::from_fn(|i| 16384usize.checked_div(i).unwrap_or(32767) as i16),
        },
    }
}
