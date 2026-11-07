//! Two independent stereo filters for each voice, before the shared color/reverb.
use super::{effects::Delay, fast};
use crate::{
    Stereo,
    preset::{FilterModel, FilterRouting, FilterType, MAX_FILTER_SLOTS, Params},
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

// A filter has two shaper stages. The two main filters and at most
// MAX_FILTER_SLOTS added filters can all appear in the same chain.
const FILTER_LATENCY: usize = 2 * super::waveshaper::MAX_LATENCY_SAMPLES;
pub(crate) const CHAIN_DELAY_CAPACITY: usize = (MAX_FILTER_SLOTS + 2) * FILTER_LATENCY + 1;
pub(crate) const RESYNTH_DELAY_CAPACITY: usize = MAX_FILTER_SLOTS * FILTER_LATENCY + 1;
// A shared filter slot can be after Resynth OR after an FX, not both.
pub(crate) const SOURCE_DELAY_CAPACITY: usize = crate::color::MAX_COLOR_SLOTS
    * super::waveshaper::MAX_LATENCY_SAMPLES
    + MAX_FILTER_SLOTS * FILTER_LATENCY
    + 1;

/// Small fixed delay for matching the dry path and parallel filter branches.
/// Its storage is prepared with the filter, never allocated while rendering.
pub(crate) struct FrameDelay<const N: usize = 128> {
    frames: [Stereo; N],
    cursor: usize,
}
impl<const N: usize> FrameDelay<N> {
    pub(crate) fn new() -> Self {
        Self {
            frames: [[0.0; 2]; N],
            cursor: 0,
        }
    }
    pub(crate) fn clear(&mut self) {
        self.frames.fill([0.0; 2]);
        self.cursor = 0;
    }
    pub(crate) fn process(&mut self, frame: Stereo, delay: usize) -> Stereo {
        debug_assert!(delay < self.frames.len());
        self.frames[self.cursor] = frame;
        let delay = delay.min(N - 1);
        let read = if self.cursor >= delay {
            self.cursor - delay
        } else {
            self.cursor + N - delay
        };
        let output = self.frames[read];
        self.cursor += 1;
        if self.cursor == N {
            self.cursor = 0;
        }
        output
    }
}

/// A main/added filter and its two independent shapers. Grain filters retain
/// the smaller raw Filter and do not pay for these extra states.
pub(crate) struct ShapedFilter {
    filter: Filter,
    before: super::waveshaper::StageProcessor,
    after: super::waveshaper::StageProcessor,
    dry: FrameDelay,
    aligned_input: Stereo,
}
impl ShapedFilter {
    pub(crate) fn new(rate: u32) -> Self {
        Self {
            filter: Filter::new(rate),
            before: super::waveshaper::StageProcessor::new(rate),
            after: super::waveshaper::StageProcessor::new(rate),
            dry: FrameDelay::new(),
            aligned_input: [0.0; 2],
        }
    }
    pub(crate) fn clear(&mut self) {
        self.filter.clear();
        self.before.clear();
        self.after.clear();
        self.dry.clear();
        self.aligned_input = [0.0; 2];
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
        self.filter
            .configure(model, kind, cutoff, resonance, drive, morph);
    }
    pub(crate) fn configure_shapers(
        &mut self,
        shapers: crate::waveshaper::FilterShapers,
        note: f32,
        envelopes: &[f32],
    ) {
        self.before.configure(shapers.before, note);
        self.after.configure(shapers.after, note);
        self.before.set_envelopes(envelopes);
        self.after.set_envelopes(envelopes);
    }
    pub(crate) fn latency_samples(&self) -> usize {
        self.before.latency_samples() + self.after.latency_samples()
    }
    pub(crate) fn aligned_input(&self) -> Stereo {
        self.aligned_input
    }
    pub(crate) fn process_with_voice_gain(&mut self, frame: Stereo, gain: f32) -> Stereo {
        let latency = self.latency_samples();
        self.aligned_input = self.dry.process(frame, latency);
        let frame = self.before.process_with_voice_gain(frame, gain);
        let frame = self.filter.process(frame);
        self.after.process_with_voice_gain(frame, gain)
    }
}

struct FilterBankState {
    filters: [ShapedFilter; 2],
    routing: FilterRouting,
    order: [usize; 2],
    parallel: [FrameDelay; 2],
    chain_dry: Box<FrameDelay<CHAIN_DELAY_CAPACITY>>,
    source_dry: Box<FrameDelay<SOURCE_DELAY_CAPACITY>>,
    resynth_dry: Box<FrameDelay<RESYNTH_DELAY_CAPACITY>>,
}
impl FilterBankState {
    fn new(sample_rate: u32) -> Self {
        Self {
            filters: std::array::from_fn(|_| ShapedFilter::new(sample_rate)),
            routing: FilterRouting::Single,
            order: [0, 1],
            parallel: std::array::from_fn(|_| FrameDelay::new()),
            chain_dry: Box::new(FrameDelay::new()),
            source_dry: Box::new(FrameDelay::new()),
            resynth_dry: Box::new(FrameDelay::new()),
        }
    }
    pub fn clear(&mut self) {
        self.filters.iter_mut().for_each(ShapedFilter::clear);
        self.parallel.iter_mut().for_each(FrameDelay::clear);
        self.chain_dry.clear();
        self.source_dry.clear();
        self.resynth_dry.clear();
    }
    pub fn configure<const N: usize>(&mut self, p: &Params, note: f32, envs: [f32; N]) {
        if self.routing != p.filter_routing {
            self.clear();
        }
        self.routing = p.filter_routing;
        self.order = if p.filter_order == [1, 0] {
            [1, 0]
        } else {
            [0, 1]
        };
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
        for (filter, shapers) in self.filters.iter_mut().zip(p.filter_shapers) {
            filter.configure_shapers(shapers, note, &envs);
        }
    }
    pub fn process(&mut self, frame: Stereo) -> Stereo {
        self.route(frame, None, 1.0, 1.0)
    }
    pub(crate) fn latency_samples(&self) -> usize {
        match self.routing {
            FilterRouting::Single => self.filters[0].latency_samples(),
            FilterRouting::Serial => self.filters.iter().map(ShapedFilter::latency_samples).sum(),
            FilterRouting::Parallel => self
                .filters
                .iter()
                .map(ShapedFilter::latency_samples)
                .max()
                .unwrap_or(0),
        }
    }
    pub(crate) fn align_chain_input(&mut self, frame: Stereo, extra_latency: usize) -> Stereo {
        let latency = self.latency_samples() + extra_latency;
        self.chain_dry.process(frame, latency)
    }
    pub(crate) fn align_source_input(&mut self, frame: Stereo, latency: usize) -> Stereo {
        self.source_dry.process(frame, latency)
    }
    pub(crate) fn align_resynth_input(&mut self, frame: Stereo, latency: usize) -> Stereo {
        self.resynth_dry.process(frame, latency)
    }
    pub(crate) fn process_tapped(
        &mut self,
        frame: Stereo,
        taps: &mut super::taps::Runtime,
        gain: f32,
    ) -> Stereo {
        self.route(frame, Some(taps), gain, 1.0)
    }
    /// Shared Rack filters have already summed their note VCAs. Only their
    /// generated Sub component follows this separate note activity signal.
    pub(crate) fn process_tapped_with_voice_gain(
        &mut self,
        frame: Stereo,
        taps: &mut super::taps::Runtime,
        gain: f32,
        voice_gain: f32,
    ) -> Stereo {
        self.route(frame, Some(taps), gain, voice_gain)
    }
    fn route(
        &mut self,
        frame: Stereo,
        mut taps: Option<&mut super::taps::Runtime>,
        gain: f32,
        voice_gain: f32,
    ) -> Stereo {
        use crate::taps::Stage;
        let count = if self.routing == FilterRouting::Single {
            1
        } else {
            2
        };
        let order = if count == 1 { [0, 1] } else { self.order };
        let mut serial = frame;
        let mut branches = [[0.0; 2]; 2];
        let common_latency = self
            .filters
            .iter()
            .map(ShapedFilter::latency_samples)
            .max()
            .unwrap_or(0);
        for &index in &order[..count] {
            let input = if self.routing == FilterRouting::Serial {
                serial
            } else {
                frame
            };
            let output = self.filters[index].process_with_voice_gain(input, voice_gain);
            let output = if let Some(taps) = taps.as_deref_mut() {
                let stage = if index == 0 {
                    Stage::Filter1
                } else {
                    Stage::Filter2
                };
                if taps.sends(stage) {
                    taps.send(stage, output.map(|x| x * gain));
                }
                taps.mix(stage, self.filters[index].aligned_input(), output)
            } else {
                output
            };
            serial = output;
            if self.routing == FilterRouting::Parallel {
                let delay = common_latency - self.filters[index].latency_samples();
                branches[index] = self.parallel[index].process(output, delay);
            }
        }
        if self.routing == FilterRouting::Parallel {
            std::array::from_fn(|c| (branches[0][c] + branches[1][c]) * 0.5)
        } else {
            serial
        }
    }
}

/// A lightweight holder for optional filter DSP state. Voice arrays may keep
/// an inactive bank where their filter output is masked; its alignment paths
/// pass samples through without allocating delay histories.
pub struct FilterBank {
    state: Option<Box<FilterBankState>>,
}

impl FilterBank {
    /// Construct a fully prepared bank. Use this before publishing a voice or
    /// Bus role that may process filters on the audio callback.
    pub fn new(sample_rate: u32) -> Self {
        Self {
            state: Some(Box::new(FilterBankState::new(sample_rate))),
        }
    }

    /// Construct an empty holder for a role whose filter output is unused.
    pub(crate) fn inactive() -> Self {
        Self { state: None }
    }

    pub(crate) fn is_prepared(&self) -> bool {
        self.state.is_some()
    }

    /// Swap a prepared bank into this holder without allocating or dropping
    /// either side. If this holder was active, its previous state moves into
    /// `prepared` and remains available for the inverse role transition.
    pub(crate) fn attach_prepared(&mut self, prepared: &mut Self) {
        debug_assert!(prepared.is_prepared(), "source FilterBank must be prepared");
        std::mem::swap(&mut self.state, &mut prepared.state);
    }

    pub fn clear(&mut self) {
        if let Some(state) = self.state.as_mut() {
            state.clear();
        }
    }

    pub fn configure<const N: usize>(&mut self, p: &Params, note: f32, envs: [f32; N]) {
        if let Some(state) = self.state.as_mut() {
            state.configure(p, note, envs);
        }
    }

    pub fn process(&mut self, frame: Stereo) -> Stereo {
        match self.state.as_mut() {
            Some(state) => state.process(frame),
            None => {
                debug_assert!(
                    frame == [0.0; 2],
                    "inactive FilterBank received nonzero audio"
                );
                frame
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn latency_samples(&self) -> usize {
        self.state
            .as_ref()
            .map_or(0, |state| state.latency_samples())
    }

    pub(crate) fn align_chain_input(&mut self, frame: Stereo, extra_latency: usize) -> Stereo {
        self.state
            .as_mut()
            .map_or(frame, |state| state.align_chain_input(frame, extra_latency))
    }

    pub(crate) fn align_source_input(&mut self, frame: Stereo, latency: usize) -> Stereo {
        self.state
            .as_mut()
            .map_or(frame, |state| state.align_source_input(frame, latency))
    }

    pub(crate) fn align_resynth_input(&mut self, frame: Stereo, latency: usize) -> Stereo {
        self.state
            .as_mut()
            .map_or(frame, |state| state.align_resynth_input(frame, latency))
    }

    pub(crate) fn process_tapped(
        &mut self,
        frame: Stereo,
        taps: &mut super::taps::Runtime,
        gain: f32,
    ) -> Stereo {
        match self.state.as_mut() {
            Some(state) => state.process_tapped(frame, taps, gain),
            None => {
                debug_assert!(
                    frame == [0.0; 2],
                    "inactive FilterBank received nonzero tapped audio"
                );
                frame
            }
        }
    }

    pub(crate) fn process_tapped_with_voice_gain(
        &mut self,
        frame: Stereo,
        taps: &mut super::taps::Runtime,
        gain: f32,
        voice_gain: f32,
    ) -> Stereo {
        match self.state.as_mut() {
            Some(state) => state.process_tapped_with_voice_gain(frame, taps, gain, voice_gain),
            None => {
                debug_assert!(
                    frame == [0.0; 2],
                    "inactive FilterBank received nonzero tapped audio"
                );
                frame
            }
        }
    }
}

#[cfg(test)]
mod delay_tests {
    use super::*;
    use crate::waveshaper::{FilterShapers, Quality, Stage};

    // Exercise zero delay, the last available sample and changes in delay
    // across multiple wraps against the original, unbounded input timeline.
    fn check_ring<const N: usize>() {
        let mut ring = FrameDelay::<N>::new();
        let frame = |i: usize| [i as f32 + 1.0, -(i as f32 + 1.0)];
        for i in 0..N * 3 {
            let delay = [0, 1, N / 2, N - 1][i % 4];
            let expected = i.checked_sub(delay).map_or([0.0; 2], frame);
            assert_eq!(
                ring.process(frame(i), delay),
                expected,
                "N={N}, i={i}, delay={delay}"
            );
        }
        ring.clear();
        assert_eq!(ring.process([1.0, -1.0], N - 1), [0.0; 2]);
        assert_eq!(ring.process([2.0, -2.0], 0), [2.0, -2.0]);
    }

    #[test]
    fn latency_rings_cover_their_full_range_across_wraps() {
        check_ring::<{ super::super::waveshaper::MAX_LATENCY_SAMPLES + 1 }>();
        check_ring::<CHAIN_DELAY_CAPACITY>();
        check_ring::<SOURCE_DELAY_CAPACITY>();
        check_ring::<RESYNTH_DELAY_CAPACITY>();
    }

    #[test]
    fn inactive_bank_is_small_and_alignment_calls_are_passthrough() {
        let mut bank = FilterBank::inactive();
        assert!(!bank.is_prepared());
        assert_eq!(
            std::mem::size_of::<FilterBank>(),
            std::mem::size_of::<Option<Box<FilterBankState>>>()
        );
        let frame = [0.25, -0.5];
        assert_eq!(bank.align_chain_input(frame, 96), frame);
        assert_eq!(bank.align_source_input(frame, 3), frame);
        assert_eq!(bank.align_resynth_input(frame, 7), frame);
        assert_eq!(bank.latency_samples(), 0);
        bank.configure(&Params::default(), 60.0, [0.0; 8]);
        bank.clear();
        assert!(!bank.is_prepared());
    }

    #[test]
    fn prepared_bank_swap_preserves_both_full_states_without_replacement() {
        let stage = Stage {
            enabled: true,
            mix: 0.0,
            options: crate::waveshaper::Options {
                quality: Quality::Clean2x,
                ..Default::default()
            },
            ..Stage::default()
        };
        let params = Params {
            filter_shapers: [FilterShapers {
                before: stage,
                after: stage,
            }; 2],
            ..Params::default()
        };
        let mut active = FilterBank::new(48_000);
        active.configure(&params, 60.0, [0.0; 8]);
        let old_latency = active.latency_samples();
        assert!(old_latency > 0);

        let mut prepared = FilterBank::new(48_000);
        assert_eq!(prepared.latency_samples(), 0);
        active.attach_prepared(&mut prepared);
        assert_eq!(active.latency_samples(), 0);
        assert_eq!(prepared.latency_samples(), old_latency);
        assert!(active.is_prepared() && prepared.is_prepared());
    }

    #[test]
    fn filter_bank_aligns_the_maximum_configured_chains() {
        let mut stage = Stage {
            enabled: true,
            mix: 0.0,
            ..Stage::default()
        };
        stage.options.quality = Quality::Clean4x;
        let params = Params {
            filter_routing: FilterRouting::Serial,
            filter_shapers: [FilterShapers {
                before: stage,
                after: stage,
            }; 2],
            ..Params::default()
        };
        let mut bank = FilterBank::new(8_000);
        bank.configure(&params, 60.0, [0.0; 8]);
        let extra = MAX_FILTER_SLOTS * FILTER_LATENCY;
        assert_eq!(bank.latency_samples() + extra, CHAIN_DELAY_CAPACITY - 1);
        for i in 0..SOURCE_DELAY_CAPACITY {
            let input = if i == 0 { [1.0, -0.5] } else { [0.0; 2] };
            for (output, latency) in [
                (
                    bank.align_chain_input(input, extra),
                    CHAIN_DELAY_CAPACITY - 1,
                ),
                (
                    bank.align_source_input(input, SOURCE_DELAY_CAPACITY - 1),
                    SOURCE_DELAY_CAPACITY - 1,
                ),
                (
                    bank.align_resynth_input(input, RESYNTH_DELAY_CAPACITY - 1),
                    RESYNTH_DELAY_CAPACITY - 1,
                ),
            ] {
                assert_eq!(
                    output,
                    if i == latency { [1.0, -0.5] } else { [0.0; 2] },
                    "frame {i}, latency {latency}"
                );
            }
        }
    }
}
