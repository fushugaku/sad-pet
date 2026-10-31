//! Four operators with explicit FM and sync sources. A one-internal-sample
//! tap delay makes every connection, including feedback cycles, causal and
//! independent of A/B/C/D iteration order. Old paired patches keep their path.
use super::*;
#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
mod simd;
#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
#[derive(Clone, Copy, Default)]
struct Mips {
    heard_key: [u64; 2],
    tapped_key: [u64; 2],
    heard: super::super::Selection,
    tapped: super::super::Selection,
}

#[derive(Clone, Copy)]
pub(in crate::dsp::wavetable) struct Graph {
    filter: Filter,
    pub active: bool,
    pub mix: super::super::Ramp,
    taps: [f32; OSCILLATORS],
    edges: [[f64; MAX_UNISON]; OSCILLATORS],
    corrections: [[f32; MAX_UNISON]; OSCILLATORS],
    lowpass: [[f32; 8]; OSCILLATORS],
    alpha: [f32; OSCILLATORS],
    keys: [u64; OSCILLATORS],
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    lookup_cache: [super::super::lookup::BatchCache; OSCILLATORS * 2],
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    mips: [[Mips; MAX_UNISON]; OSCILLATORS],
}
impl Default for Graph {
    fn default() -> Self {
        Self {
            filter: Filter::default(),
            active: false,
            mix: super::super::Ramp::default(),
            taps: [0.0; OSCILLATORS],
            edges: [[-1.0; MAX_UNISON]; OSCILLATORS],
            corrections: [[0.0; MAX_UNISON]; OSCILLATORS],
            lowpass: [[0.0; 8]; OSCILLATORS],
            alpha: [0.0; OSCILLATORS],
            keys: [0; OSCILLATORS],
            #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
            lookup_cache: [super::super::lookup::BatchCache::default(); OSCILLATORS * 2],
            #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
            mips: [[Mips::default(); MAX_UNISON]; OSCILLATORS],
        }
    }
}
impl Graph {
    pub(in crate::dsp::wavetable) fn reset(&mut self) {
        *self = Self::default();
    }
}

#[inline]
fn crossing(phase: f64, step: f64) -> f64 {
    let next = phase + step;
    if phase > 0.0 && phase < 1.0 && next > 0.0 && next < 1.0 {
        return -1.0;
    }
    if step > 0.0 && next.floor() > phase.floor() {
        ((next.floor() - phase) / step).clamp(0.0, 1.0)
    } else if step < 0.0 && next.ceil() < phase.ceil() {
        ((next.ceil() - phase) / step).clamp(0.0, 1.0)
    } else {
        -1.0
    }
}
// Reset correction is rare. Keep its six extra table reads out of the
// ordinary high-rate sample path, which can then be fully inlined.
#[cold]
#[inline(never)]
fn discontinuity(view: &mut Lookup<'_>, before: f64, reset: f64, pm: f64, step: f64) -> (f32, f32) {
    let jump = view.read(reset + pm) - view.read(before + pm);
    let delta = 1e-4;
    let slope = ((view.read(reset + pm + delta) - view.read(reset + pm - delta))
        - (view.read(before + pm + delta) - view.read(before + pm - delta)))
        / (2.0 * delta as f32)
        * step as f32;
    (jump, slope)
}
#[inline(always)]
fn value(
    view: Option<&mut Lookup<'_>>,
    correction: &mut f32,
    phase: f64,
    step: f64,
    event: f64,
    reset: f64,
    pm: f64,
) -> f32 {
    let Some(view) = view else {
        *correction = 0.0;
        return 0.0;
    };
    let mut sample = view.read(phase + pm) + *correction;
    *correction = 0.0;
    if event >= 0.0 {
        let (jump, slope) = discontinuity(view, phase + step * event, reset, pm, step);
        let f = event as f32;
        let left = 1.0 - f;
        sample += jump * 0.5 * left * left + slope * left * left * left / 6.0;
        *correction = -jump * 0.5 * f * f + slope * f * f * f / 6.0;
    }
    sample
}

#[allow(clippy::too_many_arguments)]
pub(in crate::dsp::wavetable) fn render(
    state: &mut Graph,
    engaged: bool,
    lanes: &mut [[Lane; MAX_UNISON]; OSCILLATORS],
    layers: &[Layer; OSCILLATORS],
    banks: &[Option<Arc<Bank>>; OSCILLATORS],
    previous: &[Option<Arc<Bank>>; OSCILLATORS],
    blends: &[f32; OSCILLATORS],
    positions: &[[f32; MAX_UNISON]; OSCILLATORS],
    weights: &[f32; OSCILLATORS],
    hz: f64,
    rate: f64,
    kernel: &Kernel,
    dc: f32,
) -> (Stereo, f32) {
    let fm = std::array::from_fn::<_, OSCILLATORS, _>(|i| {
        let s = layers[i].target.fm_source as usize;
        layers[i].target.fm_mode != 0
            && layers[i].ramps[11].value > 0.0
            && banks.get(s).is_some_and(Option::is_some)
            && layers[s].operator.value > 0.0
    });
    let sync = std::array::from_fn::<_, OSCILLATORS, _>(|i| {
        layers[i].target.sync
            && banks
                .get(layers[i].target.sync_source as usize)
                .is_some_and(Option::is_some)
    });
    let enabled = engaged && (0..OSCILLATORS).any(|i| banks[i].is_some() && (fm[i] || sync[i]));
    state.active = true;
    state.mix.target(f32::from(enabled), (rate * 0.020) as u32);
    let mix = state.mix.next();
    if !enabled && mix == 0.0 {
        state.active = false;
        return ([0.0; 2], 0.0);
    }
    let synced = sync.iter().any(|&x| x);
    let factor = if synced { 32 } else { OVERSAMPLE };
    let internal = rate * factor as f64;
    let counts = std::array::from_fn::<_, OSCILLATORS, _>(|i| {
        layers[i]
            .lanes
            .iter()
            .rposition(|r| r.value > 0.0)
            .map_or(1, |j| j + 1)
    });
    let centers = std::array::from_fn::<_, OSCILLATORS, _>(|i| hz * layers[i].tune);
    let depth = std::array::from_fn::<_, OSCILLATORS, _>(|i| {
        if fm[i] {
            layers[i].ramps[11].value * layers[layers[i].target.fm_source as usize].operator.value
        } else {
            0.0
        }
    });
    let mut needed = [false; OSCILLATORS];
    let mut limits = [rate; OSCILLATORS];
    for i in 0..OSCILLATORS {
        if fm[i] {
            let s = layers[i].target.fm_source as usize;
            needed[s] = true;
            limits[s] = limits[s].min(rate / (1.0 + depth[i] as f64 * 0.5));
        }
    }
    for (i, limit) in limits.iter().enumerate() {
        let key = (limit / internal).to_bits();
        if state.keys[i] != key {
            state.keys[i] = key;
            state.alpha[i] =
                (1.0 - (-std::f64::consts::TAU * limit * 0.45 * 3.32 / internal).exp()) as f32;
        }
    }
    let mut heard: [[Option<Lookup<'_>>; MAX_UNISON]; OSCILLATORS] =
        [[None; MAX_UNISON]; OSCILLATORS];
    let mut tapped = heard;
    let mut steps = [[0.0_f64; MAX_UNISON]; OSCILLATORS];
    let mut masters = steps;
    let mut gains = [[[0.0; 2]; MAX_UNISON]; OSCILLATORS];
    let mut tap_gains = [[0.0; MAX_UNISON]; OSCILLATORS];
    for i in 0..OSCILLATORS {
        let norm = layers[i]
            .lanes
            .iter()
            .map(|r| r.value)
            .sum::<f32>()
            .max(1.0);
        for id in 0..MAX_UNISON {
            let f = hz * layers[i].ratios[id];
            lanes[i][id].frequency = f;
            masters[i][id] = f / internal;
            let base = f * if sync[i] {
                layers[i].ramps[12].value as f64
            } else {
                1.0
            };
            steps[i][id] = base / internal;
            let high = ((rate * 0.45 - f) / (rate * 0.03)).clamp(0.0, 1.0) as f32;
            gains[i][id] = layers[i].gains[id].map(|g| g * high * weights[i]);
            tap_gains[i][id] = layers[i].lanes[id].value / norm
                * ((limits[i] * 0.45 - f) / (limits[i] * 0.03)).clamp(0.0, 1.0) as f32;
            if layers[i].lanes[id].value == 0.0 {
                continue;
            }
            if let Some(bank) = banks[i].as_deref() {
                let s = layers[i].target.fm_source as usize;
                let band = if fm[i] { limits[s] * 0.45 } else { 0.0 };
                let bound = base
                    + if fm[i] && layers[i].target.fm_mode == 1 {
                        2.0 * depth[i] as f64 * centers[s] + band
                    } else if fm[i] {
                        (2.0 * depth[i] as f64 + 1.0) * band
                    } else {
                        0.0
                    };
                if gains[i][id] != [0.0; 2] {
                    let budget = if sync[i] { rate * 4.0 } else { internal };
                    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
                    let selection = {
                        let cached = &mut state.mips[i][id];
                        let key = [bound.to_bits(), budget.to_bits()];
                        if key != cached.heard_key {
                            cached.heard_key = key;
                            cached.heard = bank.selection(bound, budget);
                        }
                        cached.heard
                    };
                    #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
                    let selection = bank.selection(bound, budget);
                    heard[i][id] = Some(Lookup::new(
                        bank,
                        previous[i].as_deref(),
                        blends[i],
                        positions[i][id],
                        selection,
                    ));
                }
                if needed[i] {
                    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
                    let selection = {
                        let cached = &mut state.mips[i][id];
                        let key = [bound.to_bits(), limits[i].to_bits()];
                        if key != cached.tapped_key {
                            cached.tapped_key = key;
                            cached.tapped = bank.selection(bound, limits[i]);
                        }
                        cached.tapped
                    };
                    #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
                    let selection = bank.selection(bound, limits[i]);
                    tapped[i][id] = Some(Lookup::new(
                        bank,
                        previous[i].as_deref(),
                        blends[i],
                        positions[i][id],
                        selection,
                    ));
                }
            }
        }
    }
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    let batched = counts == [4; OSCILLATORS]
        && simd::render(
            state,
            lanes,
            layers,
            &mut heard,
            &mut tapped,
            &steps,
            &masters,
            &gains,
            &tap_gains,
            &centers,
            &depth,
            &fm,
            &sync,
            &needed,
            factor,
            internal,
            kernel,
        );
    #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
    let batched = false;
    for sub in 0..if batched { 0 } else { factor } {
        let old_taps = state.taps;
        let old_edges = state.edges;
        let mut new_taps = [0.0; OSCILLATORS];
        let mut frame = [0.0; 2];
        for i in 0..OSCILLATORS {
            if banks[i].is_none() {
                continue;
            }
            let source = layers[i].target.fm_source as usize;
            let clock = layers[i].target.sync_source as usize;
            let mode = if fm[i] { layers[i].target.fm_mode } else { 0 };
            let mode_key = mode + 4 * source.min(3) as u8;
            let modulation = if fm[i] {
                old_taps[source].clamp(-2.0, 2.0) as f64 * depth[i] as f64
            } else {
                0.0
            };
            let fm_step = if mode == 1 {
                modulation * centers[source] / internal
            } else {
                0.0
            };
            let pm = if mode == 2 {
                modulation / std::f64::consts::TAU
            } else {
                0.0
            };
            for lane in lanes[i].iter_mut().take(counts[i]) {
                if lane.mode != mode_key {
                    lane.phase = wrap(lane.phase + lane.phase_mod - pm);
                    lane.mode = mode_key;
                }
                lane.phase_mod = pm;
            }
            for id in 0..counts[i] {
                let lane = &mut lanes[i][id];
                let event = if !sync[i] {
                    -1.0
                } else if clock == i {
                    if lane.master + masters[i][id] >= 1.0 {
                        ((1.0 - lane.master) / masters[i][id]).clamp(0.0, 1.0)
                    } else {
                        -1.0
                    }
                } else {
                    old_edges[clock][id % counts[clock]]
                };
                let step = steps[i][id] + fm_step;
                let phase = lane.phase;
                let reset = layers[i].target.phase as f64;
                if needed[i] {
                    let tap = value(
                        tapped[i][id].as_mut(),
                        &mut state.corrections[i][id],
                        phase,
                        step,
                        event,
                        reset,
                        pm,
                    );
                    new_taps[i] += tap * tap_gains[i][id];
                }
                let value = value(
                    heard[i][id].as_mut(),
                    &mut lane.correction,
                    phase,
                    step,
                    event,
                    reset,
                    pm,
                );
                // Heard and operator taps share one phase and clock advance.
                let mut edge = crossing(phase + pm, step);
                if event >= 0.0 {
                    let f = event.clamp(0.0, 1.0);
                    lane.phase = wrap(reset + step * (1.0 - f));
                    // Resets themselves are not outgoing clock pulses.
                    edge = if edge >= 0.0 && edge <= f { edge } else { -1.0 };
                    let after = crossing(reset + pm, step * (1.0 - f));
                    if after >= 0.0 {
                        edge = f + (1.0 - f) * after;
                    }
                } else {
                    lane.phase = wrap(phase + step);
                }
                lane.master = wrap(lane.master + masters[i][id]);
                state.edges[i][id] = edge;
                for c in 0..2 {
                    frame[c] += value * gains[i][id][c];
                }
            }
            if needed[i] && (fm[i] || sync[i]) {
                let mut value = new_taps[i].clamp(-2.0, 2.0);
                for pole in &mut state.lowpass[i] {
                    *pole += state.alpha[i] * (value - *pole);
                    value = *pole;
                }
                new_taps[i] = value;
            } else {
                state.lowpass[i].fill(new_taps[i]);
            }
        }
        state.taps = new_taps;
        if synced {
            let value = state.filter.pre(frame, kernel, sub % 4 == 3);
            if sub % 4 == 3 {
                state.filter.push(value);
            }
        } else {
            state.filter.push(frame);
        }
    }
    // Inaudible unison lanes keep a free base clock without high-rate table work.
    for i in 0..OSCILLATORS {
        for id in counts[i]..MAX_UNISON {
            let lane = &mut lanes[i][id];
            lane.master = wrap(lane.master + masters[i][id] * factor as f64);
            lane.phase = wrap(lane.phase + steps[i][id] * factor as f64);
            lane.correction = 0.0;
        }
    }
    (state.filter.next(kernel, dc), mix)
}
