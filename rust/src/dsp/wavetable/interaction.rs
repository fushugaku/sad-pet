//! B→A / D→C, signed linear FM and distinct phase modulation. The coupled
//! pair runs at 8×, followed by a 641-tap Kaiser lowpass before decimation.
//! Sync corrects both the value and slope discontinuity at its fractional
//! reset time (polyBLEP / polyBLAMP), rather than resetting a 1× lookup.
use super::{Bank, Lane, Layer, MAX_UNISON, OSCILLATORS, Stereo};
use std::sync::{Arc, OnceLock};
#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
mod pair_simd;
mod routing;
pub(super) use routing::{Graph, render as render_routed};

pub(super) const OVERSAMPLE: usize = 8;
const TAPS: usize = 641;
pub(super) struct Kernel {
    weights: [f32; TAPS],
    pre: [f32; 63],
}
impl Kernel {
    pub fn shared() -> &'static Self {
        static KERNEL: OnceLock<Kernel> = OnceLock::new();
        KERNEL.get_or_init(|| {
            let i0 = |x: f64| {
                let mut sum = 1.0;
                let mut term = 1.0;
                for k in 1..48 {
                    term *= x * x / (4.0 * (k * k) as f64);
                    sum += term;
                    if term < sum * 1e-16 {
                        break;
                    }
                }
                sum
            };
            let denom = i0(10.0);
            let mut weights = std::array::from_fn(|n| {
                let x = n as f64 - (TAPS - 1) as f64 / 2.0;
                let cutoff = 0.45 / OVERSAMPLE as f64;
                let sinc = if x == 0.0 {
                    2.0 * cutoff
                } else {
                    (std::f64::consts::TAU * cutoff * x).sin() / (std::f64::consts::PI * x)
                };
                let r = 2.0 * n as f64 / (TAPS - 1) as f64 - 1.0;
                (sinc * i0(10.0 * (1.0 - r * r).max(0.0).sqrt()) / denom) as f32
            });
            let sum: f32 = weights.iter().sum();
            for x in &mut weights {
                *x /= sum;
            }
            let mut pre = std::array::from_fn(|n| {
                let x = n as f64 - 31.0;
                let cutoff = 2.0 / 32.0;
                let sinc = if x == 0.0 {
                    2.0 * cutoff
                } else {
                    (std::f64::consts::TAU * cutoff * x).sin() / (std::f64::consts::PI * x)
                };
                let a = std::f64::consts::TAU * n as f64 / 62.0;
                (sinc * (0.42 - 0.5 * a.cos() + 0.08 * (2.0 * a).cos())) as f32
            });
            let sum: f32 = pre.iter().sum();
            for x in &mut pre {
                *x /= sum;
            }
            Kernel { weights, pre }
        })
    }
}
#[derive(Clone, Copy)]
struct Filter {
    history: [Stereo; TAPS * 2],
    head: usize,
    input: Stereo,
    output: Stereo,
    pre_history: [Stereo; 126],
    pre_head: usize,
}
impl Default for Filter {
    fn default() -> Self {
        Self {
            history: [[0.0; 2]; TAPS * 2],
            head: 0,
            input: [0.0; 2],
            output: [0.0; 2],
            pre_history: [[0.0; 2]; 126],
            pre_head: 0,
        }
    }
}
#[derive(Clone, Copy, Default)]
pub(super) struct Pair {
    filter: Filter,
    pub mix: super::Ramp,
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    lookup_cache: [super::lookup::BatchCache; 3],
}
impl Pair {
    pub(super) fn reset(&mut self) {
        self.filter = Filter::default();
        self.mix = super::Ramp::default();
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        {
            self.lookup_cache = [super::lookup::BatchCache::default(); 3];
        }
    }
}
// Four independent reduction chains have identical arithmetic order on
// native and WASM, while avoiding one long floating-point dependency chain.
#[inline]
fn dot(weights: &[f32], frames: &[Stereo]) -> Stereo {
    #[cfg(target_arch = "aarch64")]
    {
        use core::arch::aarch64::*;
        // Four independent chains per channel, in the scalar order below.
        // load2 deinterleaves four stereo frames; every load is bounded by
        // the complete four-element prefix of the shorter validated view.
        unsafe {
            let full = weights.len().min(frames.len()) / 4 * 4;
            let mut left = vdupq_n_f32(0.0);
            let mut right = left;
            for i in (0..full).step_by(4) {
                let w = vld1q_f32(weights.as_ptr().add(i));
                let x = vld2q_f32(frames.as_ptr().add(i).cast());
                left = vaddq_f32(left, vmulq_f32(w, x.0));
                right = vaddq_f32(right, vmulq_f32(w, x.1));
            }
            let mut l = [0.0; 4];
            let mut r = [0.0; 4];
            vst1q_f32(l.as_mut_ptr(), left);
            vst1q_f32(r.as_mut_ptr(), right);
            let mut sum = [(l[0] + l[1]) + (l[2] + l[3]), (r[0] + r[1]) + (r[2] + r[3])];
            for (&w, x) in weights[full..].iter().zip(&frames[full..]) {
                sum[0] += w * x[0];
                sum[1] += w * x[1];
            }
            sum
        }
    }
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    {
        use core::arch::wasm32::*;
        // Two stereo loads are deinterleaved into the original four chains.
        // Keep the reduction and remainder in exactly the scalar order.
        unsafe {
            let full = weights.len().min(frames.len()) / 4 * 4;
            let mut left = f32x4_splat(0.0);
            let mut right = left;
            for i in (0..full).step_by(4) {
                let w = v128_load(weights.as_ptr().add(i).cast());
                let x = v128_load(frames.as_ptr().add(i).cast());
                let y = v128_load(frames.as_ptr().add(i + 2).cast());
                left = f32x4_add(left, f32x4_mul(w, i32x4_shuffle::<0, 2, 4, 6>(x, y)));
                right = f32x4_add(right, f32x4_mul(w, i32x4_shuffle::<1, 3, 5, 7>(x, y)));
            }
            let mut l = [0.0_f32; 4];
            let mut r = l;
            v128_store(l.as_mut_ptr().cast(), left);
            v128_store(r.as_mut_ptr().cast(), right);
            let mut sum = [(l[0] + l[1]) + (l[2] + l[3]), (r[0] + r[1]) + (r[2] + r[3])];
            for (&w, x) in weights[full..].iter().zip(&frames[full..]) {
                sum[0] += w * x[0];
                sum[1] += w * x[1];
            }
            sum
        }
    }
    #[cfg(not(any(
        target_arch = "aarch64",
        all(target_arch = "wasm32", target_feature = "simd128")
    )))]
    {
        let mut sums = [[0.0_f32; 2]; 4];
        for (w, x) in weights.chunks_exact(4).zip(frames.chunks_exact(4)) {
            for j in 0..4 {
                sums[j][0] += w[j] * x[j][0];
                sums[j][1] += w[j] * x[j][1];
            }
        }
        let full = weights.len() / 4 * 4;
        let mut sum = [
            (sums[0][0] + sums[1][0]) + (sums[2][0] + sums[3][0]),
            (sums[0][1] + sums[1][1]) + (sums[2][1] + sums[3][1]),
        ];
        for (&w, x) in weights[full..].iter().zip(&frames[full..]) {
            sum[0] += w * x[0];
            sum[1] += w * x[1];
        }
        sum
    }
}
impl Filter {
    fn push(&mut self, x: Stereo) {
        self.history[self.head] = x;
        self.history[self.head + TAPS] = x;
        self.head = (self.head + 1) % TAPS;
    }
    fn pre(&mut self, frame: Stereo, kernel: &Kernel, read: bool) -> Stereo {
        self.pre_history[self.pre_head] = frame;
        self.pre_history[self.pre_head + 63] = frame;
        self.pre_head = (self.pre_head + 1) % 63;
        if read {
            dot(
                &kernel.pre,
                &self.pre_history[self.pre_head..self.pre_head + 63],
            )
        } else {
            [0.0; 2]
        }
    }
    fn next(&mut self, kernel: &Kernel, dc: f32) -> Stereo {
        let mut sum = dot(&kernel.weights, &self.history[self.head..self.head + TAPS]);
        for (c, value) in sum.iter_mut().enumerate() {
            let filtered = *value - self.input[c] + dc * self.output[c];
            self.input[c] = *value;
            self.output[c] = if filtered.abs() < 1e-20 {
                0.0
            } else {
                filtered
            };
            *value = self.output[c];
        }
        sum
    }
}

fn read(
    bank: &Bank,
    previous: Option<&Bank>,
    blend: f32,
    phase: f64,
    position: f32,
    selection: super::Selection,
) -> f32 {
    let new = bank.read(phase, position, selection);
    if blend < 1.0
        && let Some(old) = previous
    {
        let old = old.read(phase, position, selection);
        old + (new - old) * blend
    } else {
        new
    }
}
#[derive(Clone, Copy)]
struct Lookup<'a> {
    current: super::lookup::Reader<'a>,
    previous: Option<&'a Bank>,
    position: f32,
    selection: super::Selection,
    blend: f32,
}
impl<'a> Lookup<'a> {
    fn new(
        bank: &'a Bank,
        previous: Option<&'a Bank>,
        blend: f32,
        position: f32,
        selection: super::Selection,
    ) -> Self {
        Self {
            current: bank.reader(position, selection),
            previous: previous.filter(|_| blend < 1.0),
            position,
            selection,
            blend,
        }
    }
    #[inline(always)]
    fn read(&mut self, phase: f64) -> f32 {
        let value = self.current.read(phase);
        if let Some(previous) = self.previous {
            let old = previous.read(phase, self.position, self.selection);
            old + (value - old) * self.blend
        } else {
            value
        }
    }
}
#[cold]
#[inline(never)]
fn discontinuity(
    view: &mut Lookup<'_>,
    before: f64,
    after: f64,
    phase_mod: f64,
    step: f64,
) -> (f32, f32) {
    let jump = view.read(after + phase_mod) - view.read(before + phase_mod);
    let delta = 1e-4;
    let mut slope = |p: f64| {
        (view.read(p + phase_mod + delta) - view.read(p + phase_mod - delta)) / (2.0 * delta as f32)
            * step as f32
    };
    (jump, slope(after) - slope(before))
}
/// Advances one slave and master by one *internal* sample. Pre/post event
/// corrections share the exact event fraction, including noninteger ratios.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn oscillator(
    lane: &mut Lane,
    view: &mut Lookup<'_>,
    step: f64,
    master_step: f64,
    sync: bool,
    reset: f64,
    phase_mod: f64,
) -> f32 {
    let phase = lane.phase;
    let mut sample = view.read(phase + phase_mod) + lane.correction;
    lane.correction = 0.0;
    if sync && lane.master + master_step >= 1.0 {
        let fraction = ((1.0 - lane.master) / master_step).clamp(0.0, 1.0);
        let before = phase + step * fraction;
        let after = reset;
        let (jump, slope_jump) = discontinuity(view, before, after, phase_mod, step);
        let f = fraction as f32;
        let left = 1.0 - f;
        sample += jump * 0.5 * left * left + slope_jump * left * left * left / 6.0;
        lane.correction = -jump * 0.5 * f * f + slope_jump * f * f * f / 6.0;
        lane.phase = wrap(reset + step * (1.0 - fraction));
    } else {
        lane.phase = wrap(phase + step);
    }
    lane.master = wrap(lane.master + master_step);
    sample
}

#[inline]
fn wrap(x: f64) -> f64 {
    if !(0.0..1.0).contains(&x) {
        x - x.floor()
    } else {
        x
    }
}
fn silent(lane: &mut Lane, step: f64, master_step: f64, sync: bool, reset: f64) {
    if sync && lane.master + master_step >= 1.0 {
        let fraction = ((1.0 - lane.master) / master_step).clamp(0.0, 1.0);
        lane.phase = wrap(reset + step * (1.0 - fraction));
    } else {
        lane.phase = wrap(lane.phase + step);
    }
    lane.master = wrap(lane.master + master_step);
    lane.correction = 0.0;
}
#[allow(clippy::too_many_arguments)]
pub(super) fn render(
    pair_index: usize,
    state: &mut Pair,
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
) -> Option<Stereo> {
    let a = pair_index * 2;
    let b = a + 1;
    let carrier_sync = layers[a].target.sync && layers[a].target.sync_source as usize == a;
    let source_sync = layers[b].target.sync && layers[b].target.sync_source as usize == b;
    let fm = layers[a].target.fm_source as usize == b
        && layers[a].target.fm_mode != 0
        && layers[a].ramps[11].value > 0.0
        && banks[b].is_some()
        && layers[b].operator.value > 0.0;
    let enabled = fm || (carrier_sync && banks[a].is_some()) || (source_sync && banks[b].is_some());
    state.mix.target(f32::from(enabled), (rate * 0.020) as u32);
    let mix = state.mix.next();
    if mix == 0.0 && !enabled {
        return None;
    }
    let mut dry = [0.0; 2];
    for osc in a..=b {
        for (id, lane) in lanes[osc].iter_mut().enumerate() {
            lane.frequency = hz * layers[osc].ratios[id];
        }
        if mix == 1.0 {
            continue;
        }
        if let Some(bank) = &banks[osc] {
            for (id, lane) in lanes[osc].iter_mut().enumerate() {
                if layers[osc].gains[id] == [0.0; 2] {
                    continue;
                }
                let sample = read(
                    bank,
                    previous[osc].as_deref(),
                    blends[osc],
                    lane.phase,
                    positions[osc][id],
                    bank.selection(lane.frequency, rate),
                );
                let gain = ((rate * 0.45 - lane.frequency) / (rate * 0.03)).clamp(0.0, 1.0) as f32;
                for (c, value) in dry.iter_mut().enumerate() {
                    *value += sample * gain * layers[osc].gains[id][c] * weights[osc];
                }
            }
        }
    }
    let synced = carrier_sync || source_sync;
    let factor = if synced { 32 } else { OVERSAMPLE };
    let internal = rate * factor as f64;
    let sync_budget = rate * 4.0;
    let depth = layers[a].ramps[11].value * layers[b].operator.value;
    let source_hz = hz * layers[b].tune;
    let source_limit = rate / (1.0 + depth as f64 * 0.5);
    let source_norm = layers[b]
        .lanes
        .iter()
        .map(|r| r.value)
        .sum::<f32>()
        .max(1.0);
    let mod_band = if let Some(bank) = &banks[b] {
        let sel = bank.selection(source_hz, source_limit);
        bank.harmonic_weights(sel)
            .iter()
            .filter(|(_, w)| *w > 0.0)
            .map(|(h, _)| *h as f64 * source_hz * 1.06)
            .fold(0.0_f64, f64::max)
    } else {
        0.0
    };
    let source_selections = std::array::from_fn::<_, MAX_UNISON, _>(|id| {
        banks[b]
            .as_ref()
            .map(|bank| bank.selection(hz * layers[b].ratios[id], source_limit))
    });
    let slave_selections = std::array::from_fn::<_, MAX_UNISON, _>(|id| {
        banks[b].as_ref().map(|bank| {
            bank.selection(
                hz * layers[b].ratios[id]
                    * if source_sync {
                        layers[b].ramps[12].value as f64
                    } else {
                        1.0
                    },
                if source_sync { sync_budget } else { internal },
            )
        })
    });
    let carrier_selections = std::array::from_fn::<_, MAX_UNISON, _>(|id| {
        banks[a].as_ref().map(|bank| {
            let base = hz
                * layers[a].ratios[id]
                * if carrier_sync {
                    layers[a].ramps[12].value as f64
                } else {
                    1.0
                };
            let bound = base
                + if fm && layers[a].target.fm_mode == 1 {
                    2.0 * depth as f64 * source_hz + mod_band
                } else if fm {
                    2.0 * depth as f64 * mod_band + mod_band
                } else {
                    0.0
                };
            bank.selection(bound, if carrier_sync { sync_budget } else { internal })
        })
    });
    let mut source_views = std::array::from_fn::<_, MAX_UNISON, _>(|id| {
        if !fm || layers[b].lanes[id].value == 0.0 {
            return None;
        }
        banks[b].as_deref().map(|bank| {
            Lookup::new(
                bank,
                previous[b].as_deref(),
                blends[b],
                positions[b][id],
                source_selections[id].unwrap(),
            )
        })
    });
    let mut slave_views = std::array::from_fn::<_, MAX_UNISON, _>(|id| {
        if layers[b].lanes[id].value == 0.0 {
            return None;
        }
        banks[b].as_deref().map(|bank| {
            Lookup::new(
                bank,
                previous[b].as_deref(),
                blends[b],
                positions[b][id],
                slave_selections[id].unwrap(),
            )
        })
    });
    let mut carrier_views = std::array::from_fn::<_, MAX_UNISON, _>(|id| {
        if layers[a].gains[id] == [0.0; 2] {
            return None;
        }
        banks[a].as_deref().map(|bank| {
            Lookup::new(
                bank,
                previous[a].as_deref(),
                blends[a],
                positions[a][id],
                carrier_selections[id].unwrap(),
            )
        })
    });
    let source_count = layers[b]
        .lanes
        .iter()
        .rposition(|r| r.value != 0.0)
        .map_or(0, |i| i + 1);
    let source_master: [f64; MAX_UNISON] =
        std::array::from_fn(|id| lanes[b][id].frequency / internal);
    let source_steps = source_master.map(|step| {
        step * if source_sync {
            layers[b].ramps[12].value as f64
        } else {
            1.0
        }
    });
    let carrier_master: [f64; MAX_UNISON] =
        std::array::from_fn(|id| lanes[a][id].frequency / internal);
    let carrier_steps = carrier_master.map(|step| {
        step * if carrier_sync {
            layers[a].ramps[12].value as f64
        } else {
            1.0
        }
    });
    let source_gains: [Stereo; MAX_UNISON] = std::array::from_fn(|id| {
        let f = lanes[b][id].frequency;
        let high = ((rate * 0.45 - f) / (rate * 0.03)).clamp(0.0, 1.0) as f32;
        layers[b].gains[id].map(|g| g * high * weights[b])
    });
    let carrier_gains: [Stereo; MAX_UNISON] = std::array::from_fn(|id| {
        let f = lanes[a][id].frequency;
        let high = ((rate * 0.45 - f) / (rate * 0.03)).clamp(0.0, 1.0) as f32;
        layers[a].gains[id].map(|g| g * high * weights[a])
    });
    let source_scales: [f32; MAX_UNISON] = std::array::from_fn(|id| {
        let f = lanes[b][id].frequency;
        ((source_limit * 0.45 - f) / (source_limit * 0.03)).clamp(0.0, 1.0) as f32
            * layers[b].lanes[id].value
            / source_norm
    });
    let fm_step = source_hz / internal;
    let mode = if fm { layers[a].target.fm_mode } else { 0 };
    let (carriers, modulators) = lanes.split_at_mut(b);
    let carriers = &mut carriers[a];
    let modulators = &mut modulators[0];
    // An inaudible source has constant frequency across the internal frame.
    // Advance it once, preserving its master/reset trajectory. The only
    // difference from repeated additions is below f64 phase resolution.
    for (id, lane) in modulators.iter_mut().enumerate().skip(source_count) {
        let master_step = source_master[id] * factor as f64;
        let step = source_steps[id] * factor as f64;
        if source_sync && lane.master + master_step >= 1.0 {
            let events = (lane.master + master_step).floor();
            let fraction = ((events - lane.master) / master_step).clamp(0.0, 1.0);
            lane.phase = wrap(layers[b].target.phase as f64 + step * (1.0 - fraction));
            lane.master = wrap(lane.master + master_step);
            lane.correction = 0.0;
        } else {
            silent(
                lane,
                step,
                master_step,
                source_sync,
                layers[b].target.phase as f64,
            );
        }
    }
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    let batched = source_count == 4
        && carrier_views[4..].iter().all(Option::is_none)
        && pair_simd::render(
            &mut state.filter,
            &mut state.lookup_cache,
            carriers,
            modulators,
            &mut source_views,
            &mut slave_views,
            &mut carrier_views,
            source_master,
            source_steps,
            carrier_master,
            carrier_steps,
            source_gains,
            carrier_gains,
            source_scales,
            fm,
            mode,
            depth,
            fm_step,
            source_sync,
            carrier_sync,
            layers[b].target.phase as f64,
            layers[a].target.phase as f64,
            factor,
            kernel,
        );
    #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
    let batched = false;
    for subsample in 0..if batched { 0 } else { factor } {
        let mut frame = [0.0; 2];
        let mut modulation_sum = 0.0;
        if banks[b].is_some() {
            for (id, lane) in modulators.iter_mut().enumerate().take(source_count) {
                // FM taps the source's base wavetable, before its level, XY
                // and sync. Its spectral budget narrows smoothly with index.
                let modulation = source_views[id]
                    .as_mut()
                    .map_or(0.0, |view| view.read(lane.master));
                let sample = oscillator(
                    lane,
                    slave_views[id].as_mut().unwrap(),
                    source_steps[id],
                    source_master[id],
                    source_sync,
                    layers[b].target.phase as f64,
                    0.0,
                );
                if fm {
                    modulation_sum += modulation.clamp(-2.0, 2.0) * source_scales[id];
                }
                for (c, value) in frame.iter_mut().enumerate() {
                    *value += sample * source_gains[id][c];
                }
            }
        }
        if banks[a].is_some() {
            let modulation = modulation_sum as f64 * depth as f64;
            let phase = if mode == 2 {
                modulation / std::f64::consts::TAU
            } else {
                0.0
            };
            for lane in carriers.iter_mut() {
                if lane.mode != mode {
                    lane.phase = wrap(lane.phase + lane.phase_mod - phase);
                    lane.mode = mode;
                }
                lane.phase_mod = phase;
            }
            for (id, lane) in carriers.iter_mut().enumerate() {
                let step = carrier_steps[id] + if mode == 1 { modulation * fm_step } else { 0.0 };
                if layers[a].gains[id] == [0.0; 2] {
                    silent(
                        lane,
                        step,
                        carrier_master[id],
                        carrier_sync,
                        layers[a].target.phase as f64,
                    );
                    continue;
                }
                let sample = oscillator(
                    lane,
                    carrier_views[id].as_mut().unwrap(),
                    step,
                    carrier_master[id],
                    carrier_sync,
                    layers[a].target.phase as f64,
                    phase,
                );
                for (c, value) in frame.iter_mut().enumerate() {
                    *value += sample * carrier_gains[id][c];
                }
            }
        }
        if synced {
            let filtered = state.filter.pre(frame, kernel, subsample % 4 == 3);
            if subsample % 4 == 3 {
                state.filter.push(filtered);
            }
        } else {
            state.filter.push(frame);
        }
    }
    let filtered = state.filter.next(kernel, dc);
    Some(std::array::from_fn(|c| {
        dry[c] + (filtered[c] - dry[c]) * mix
    }))
}
