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

/// Cutoff modulation in logarithmic frequency space. C4 is the tracking pivot;
/// 100% tracking shifts the cutoff by one octave per octave played, including bend.
pub fn voice_cutoffs<const N: usize>(p: &Params, note: f32, envs: [f32; N]) -> [f32; 2] {
    [
        tracked_cutoff(
            p.filter_model,
            p.vcf_type,
            p.filter,
            envs[p.filter_env.index()],
            p.filter_env_depth,
            note,
            p.filter_keytrack,
        ),
        tracked_cutoff(
            p.filter2_model,
            p.vcf2_type,
            p.filter2,
            envs[p.filter2_env.index()],
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
    pub fn process(&mut self, frame: Stereo) -> Stereo {
        let first = self.filters[0].process(frame);
        match self.routing {
            FilterRouting::Single => first,
            FilterRouting::Serial => self.filters[1].process(first),
            FilterRouting::Parallel => {
                let second = self.filters[1].process(frame);
                std::array::from_fn(|c| (first[c] + second[c]) * 0.5)
            }
        }
    }
}
