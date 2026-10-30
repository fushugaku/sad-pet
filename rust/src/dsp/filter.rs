//! Two independent stereo filters for each voice, before the shared color/reverb.
use super::{effects::Delay, fast};
use crate::{
    Stereo,
    preset::{FilterModel, FilterRouting, FilterType, Params},
};

pub(crate) struct Filter {
    sample_rate: f32,
    kind: FilterType,
    configured: bool,
    state: [[f32; 2]; 2],
    comb: Delay,
    cutoff: f32,
    resonance: f32,
    g: f32,
    k: f32,
    coefficients: [f32; 3],
    band_gain: f32,
    delay: f32,
    model: FilterModel,
    rustias: super::radias_filter::RustiasFilter,
    drive: f32,
    morph: f32,
}
impl Filter {
    pub(crate) fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate: sample_rate as f32,
            kind: FilterType::LowPass,
            configured: false,
            state: [[0.0; 2]; 2],
            comb: Delay::new((sample_rate as f32 * 0.1) as usize),
            cutoff: 1.0,
            resonance: 0.0,
            g: 1.0,
            k: 2.0,
            coefficients: [0.25; 3],
            band_gain: 2.0_f32.sqrt(),
            delay: 1.0,
            model: FilterModel::Legacy,
            rustias: super::radias_filter::RustiasFilter::new(sample_rate),
            drive: 0.0,
            morph: 0.0,
        }
    }
    pub(crate) fn clear(&mut self) {
        self.state = [[0.0; 2]; 2];
        self.comb.clear();
        self.rustias.clear();
    }
    pub(crate) fn configure(
        &mut self,
        model: FilterModel,
        kind: FilterType,
        cutoff: f32,
        resonance: f32,
        drive: f32,
        morph: f32,
    ) {
        let cutoff = cutoff.clamp(0.0, 1.0);
        let resonance = resonance.clamp(0.0, 1.0);
        if self.configured
            && self.kind == kind
            && self.cutoff == cutoff
            && self.resonance == resonance
            && self.model == model
            && self.drive == drive
            && self.morph == morph
        {
            return;
        }
        if self.kind != kind || self.model != model {
            self.clear();
        }
        self.kind = kind;
        self.model = model;
        self.drive = drive;
        self.morph = morph;
        self.configured = true;
        self.cutoff = cutoff.clamp(0.0, 1.0);
        self.resonance = resonance.clamp(0.0, 1.0);
        if model != FilterModel::Legacy {
            self.rustias
                .configure(model, kind, [cutoff, resonance, drive, morph]);
            return;
        }
        let hz = cutoff_hz(kind, self.cutoff);
        self.g = (std::f32::consts::PI * hz.min(self.sample_rate * 0.45) / self.sample_rate).tan();
        self.k = 2.0 - 1.94 * self.resonance;
        let a1 = 1.0 / (1.0 + self.g * (self.g + self.k));
        let a2 = self.g * a1;
        self.coefficients = [a1, a2, self.g * a2];
        self.band_gain = self.k.sqrt();
        self.delay = self.sample_rate / hz;
    }
    pub(crate) fn process(&mut self, frame: Stereo) -> Stereo {
        if self.model != FilterModel::Legacy {
            return self.rustias.process(frame);
        }
        if self.kind == FilterType::Comb {
            let delayed = self.comb.read(self.delay);
            let result = std::array::from_fn(|c| {
                fast::flush(fast::tanh(frame[c] + delayed[c] * self.resonance * 0.92))
            });
            self.comb.push(result);
            return result;
        }
        let k = self.k;
        let [a1, a2, a3] = self.coefficients;
        std::array::from_fn(|c| {
            let [ic1, ic2] = self.state[c];
            let v3 = frame[c] - ic2;
            let band = a1 * ic1 + a2 * v3;
            let low = ic2 + a2 * ic1 + a3 * v3;
            self.state[c] = [fast::flush(2.0 * band - ic1), fast::flush(2.0 * low - ic2)];
            if self.cutoff >= 0.999 && self.kind == FilterType::LowPass {
                frame[c]
            } else {
                match self.kind {
                    FilterType::LowPass => low,
                    FilterType::BandPass => band * self.band_gain,
                    FilterType::HighPass => frame[c] - k * band - low,
                    FilterType::Comb => unreachable!(),
                }
            }
        })
    }
}

/// Rustias uses the original analytic 30 Hz .. 6 kHz controller scale.
pub fn model_cutoff_hz(model: FilterModel, kind: FilterType, value: f32) -> f32 {
    if model == FilterModel::Legacy {
        cutoff_hz(kind, value)
    } else {
        30.0 * 200.0_f32.powf(value.clamp(0.0, 1.0))
    }
}

pub fn cutoff_hz(kind: FilterType, value: f32) -> f32 {
    let (base, span): (f32, f32) = if kind == FilterType::Comb {
        (30.0, 200.0)
    } else {
        (20.0, 1000.0)
    };
    base * span.powf(value.clamp(0.0, 1.0))
}

/// The existing envelopes plus the source voice's dedicated 303 envelope.
pub(super) fn envelopes(
    envs: [f32; crate::preset::MODULATORS],
    tb303: f32,
) -> [f32; crate::preset::MODULATORS + 1] {
    let mut values = [0.0; crate::preset::MODULATORS + 1];
    values[..crate::preset::MODULATORS].copy_from_slice(&envs);
    values[crate::preset::MODULATORS] = tb303;
    values
}

/// Cutoff modulation in logarithmic frequency space. C4 is the tracking pivot;
/// 100% tracking shifts the cutoff by one octave per octave played, including bend.
pub fn voice_cutoffs<const N: usize>(p: &Params, note: f32, envs: [f32; N]) -> [f32; 2] {
    [
        tracked_cutoff(
            p.filter_model,
            p.vcf_type,
            p.filter,
            p.filter_env.value(&envs),
            p.filter_env_depth,
            note,
            p.filter_keytrack,
        ),
        tracked_cutoff(
            p.filter2_model,
            p.vcf2_type,
            p.filter2,
            p.filter2_env.value(&envs),
            p.filter2_env_depth,
            note,
            p.filter2_keytrack,
        ),
    ]
}

/// One filter's cutoff with its envelope and key tracking applied.
pub fn tracked_cutoff(
    model: FilterModel,
    kind: FilterType,
    base: f32,
    envelope: f32,
    depth: f32,
    note: f32,
    tracking: f32,
) -> f32 {
    // ln(200) and ln(1000) once, from the same libm call as before, instead of
    // a logarithm per voice and filter on every control block.
    static SPAN_LN: std::sync::LazyLock<[f32; 2]> =
        std::sync::LazyLock::new(|| [200.0_f32.ln(), 1000.0_f32.ln()]);
    let span_ln = if model != FilterModel::Legacy || kind == FilterType::Comb {
        SPAN_LN[0]
    } else {
        SPAN_LN[1]
    };
    (base + envelope * depth + (note - 60.0) * tracking * std::f32::consts::LN_2 / (12.0 * span_ln))
        .clamp(0.0, 1.0)
}

/// Frequencies a response covers, spaced evenly in octaves from 20 Hz to 20 kHz.
pub const RESPONSE_POINTS: usize = 160;

/// One filter's magnitude response in dB, measured by ringing a quiet impulse
/// through the real filter at 48 kHz, so every model and Comb is exact at low
/// levels. Drive stays at zero: the curve shows the filter, not saturation.
pub fn response(
    model: FilterModel,
    kind: FilterType,
    cutoff: f32,
    resonance: f32,
    morph: f32,
) -> Vec<f32> {
    const RATE: u32 = 48_000;
    const LENGTH: usize = 8192;
    const LEVEL: f32 = 0.01;
    let mut filter = Filter::new(RATE);
    filter.configure(model, kind, cutoff, resonance, 0.0, morph);
    // The last quarter fades out, so a long resonant tail cut short does not
    // ripple the curve.
    let ring: Vec<f64> = (0..LENGTH)
        .map(|n| {
            let sample = f64::from(filter.process([if n == 0 { LEVEL } else { 0.0 }; 2])[0]);
            let tail = (n as f64 - LENGTH as f64 * 0.75) / (LENGTH as f64 * 0.25);
            let taper = if tail > 0.0 {
                0.5 + 0.5 * (std::f64::consts::PI * tail).cos()
            } else {
                1.0
            };
            sample * taper / f64::from(LEVEL)
        })
        .collect();
    (0..RESPONSE_POINTS)
        .map(|i| {
            let hz = 20.0 * 1000.0_f64.powf(i as f64 / (RESPONSE_POINTS - 1) as f64);
            let (sin, cos) = (std::f64::consts::TAU * hz / f64::from(RATE)).sin_cos();
            // Turn one phasor per sample instead of a sine and cosine each.
            let (mut re, mut im, mut turn) = (0.0, 0.0, (1.0, 0.0));
            for h in &ring {
                re += h * turn.0;
                im -= h * turn.1;
                turn = (turn.0 * cos - turn.1 * sin, turn.0 * sin + turn.1 * cos);
            }
            (20.0 * re.hypot(im).max(1e-9).log10()) as f32
        })
        .collect()
}

/// `{model, kind, cutoffs: [one or two], resonance, morph}` from the editor,
/// answered with `{"curves": [[dB; RESPONSE_POINTS], …]}` to a tenth of a dB.
pub fn response_request(data: &serde_json::Value) -> Result<serde_json::Value, String> {
    let model: FilterModel =
        serde_json::from_value(data["model"].clone()).map_err(|e| format!("Filter model: {e}"))?;
    let kind: FilterType =
        serde_json::from_value(data["kind"].clone()).map_err(|e| format!("Filter type: {e}"))?;
    let number = |value: &serde_json::Value| {
        value
            .as_f64()
            .filter(|v| v.is_finite())
            .map(|v| v as f32)
            .ok_or("Filter response values must be numbers")
    };
    let resonance = number(&data["resonance"])?;
    let morph = number(&data["morph"])?;
    let curves = data["cutoffs"]
        .as_array()
        .filter(|cutoffs| (1..=2).contains(&cutoffs.len()))
        .ok_or("Filter response needs one or two cutoffs")?
        .iter()
        .map(|cutoff| {
            Ok(response(model, kind, number(cutoff)?, resonance, morph)
                .into_iter()
                .map(|db| (db * 10.0).round() / 10.0)
                .collect::<Vec<_>>())
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(serde_json::json!({ "curves": curves }))
}

pub struct FilterBank {
    filters: [Filter; 2],
    routing: FilterRouting,
}
impl FilterBank {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            filters: std::array::from_fn(|_| Filter::new(sample_rate)),
            routing: FilterRouting::Single,
        }
    }
    pub fn clear(&mut self) {
        self.filters.iter_mut().for_each(Filter::clear);
    }
    pub fn configure<const N: usize>(&mut self, p: &Params, note: f32, envs: [f32; N]) {
        if self.routing != p.filter_routing {
            self.clear();
        }
        self.routing = p.filter_routing;
        let cutoffs = voice_cutoffs(p, note, envs);
        self.filters[0].configure(
            p.filter_model,
            p.vcf_type,
            cutoffs[0],
            p.filter_resonance,
            p.filter_drive,
            p.filter_morph,
        );
        self.filters[1].configure(
            p.filter2_model,
            p.vcf2_type,
            cutoffs[1],
            p.filter2_resonance,
            p.filter2_drive,
            p.filter2_morph,
        );
    }
    pub(crate) fn process(
        &mut self,
        frame: Stereo,
        taps: &mut super::taps::Runtime,
        gain: f32,
    ) -> Stereo {
        use crate::taps::Stage;
        let first = self.filters[0].process(frame);
        if taps.sends(Stage::Filter1) {
            taps.send(Stage::Filter1, first.map(|x| x * gain));
        }
        let first = taps.mix(Stage::Filter1, frame, first);
        match self.routing {
            FilterRouting::Single => first,
            FilterRouting::Serial | FilterRouting::Parallel => {
                let input = if self.routing == FilterRouting::Serial {
                    first
                } else {
                    frame
                };
                let second = self.filters[1].process(input);
                if taps.sends(Stage::Filter2) {
                    taps.send(Stage::Filter2, second.map(|x| x * gain));
                }
                let second = taps.mix(Stage::Filter2, input, second);
                if self.routing == FilterRouting::Serial {
                    second
                } else {
                    std::array::from_fn(|c| (first[c] + second[c]) * 0.5)
                }
            }
        }
    }
}
