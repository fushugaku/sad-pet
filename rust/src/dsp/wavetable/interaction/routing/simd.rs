//! U4 fast path. Keep feedback sums and reset arithmetic in their original
//! order; vectorize independent table reads, f64 phases and operator lowpasses.
use super::*;
use crate::dsp::wavetable::lookup::Batch4;
use core::arch::wasm32::*;

fn batch<'a, 'c>(
    views: &[Option<Lookup<'a>>; MAX_UNISON],
    cache: &'c mut crate::dsp::wavetable::lookup::BatchCache,
    merge: bool,
) -> Option<Batch4<'a, 'c>> {
    let mut readers = [&views[0].as_ref()?.current; 4];
    for i in 0..4 {
        let view = views[i].as_ref()?;
        if view.previous.is_some() {
            return None;
        }
        readers[i] = &view.current;
    }
    Batch4::new(readers, cache, merge)
}
#[inline(always)]
fn pack(x: &[f64; MAX_UNISON]) -> [v128; 2] {
    [f64x2(x[0], x[1]), f64x2(x[2], x[3])]
}
#[inline(always)]
fn floats(x: v128) -> [f32; 4] {
    [
        f32x4_extract_lane::<0>(x),
        f32x4_extract_lane::<1>(x),
        f32x4_extract_lane::<2>(x),
        f32x4_extract_lane::<3>(x),
    ]
}
#[inline(always)]
fn doubles(x: [v128; 2]) -> [f64; 4] {
    [
        f64x2_extract_lane::<0>(x[0]),
        f64x2_extract_lane::<1>(x[0]),
        f64x2_extract_lane::<0>(x[1]),
        f64x2_extract_lane::<1>(x[1]),
    ]
}
#[inline(always)]
fn sum(initial: f32, x: v128) -> f32 {
    let x = floats(x);
    (((initial + x[0]) + x[1]) + x[2]) + x[3]
}
#[inline(always)]
fn wrapped(p: v128) -> v128 {
    f64x2_sub(p, f64x2_floor(p))
}
#[cold]
#[inline(never)]
fn boundaries(phase: v128, step: v128) -> v128 {
    f64x2(
        crossing(
            f64x2_extract_lane::<0>(phase),
            f64x2_extract_lane::<0>(step),
        ),
        crossing(
            f64x2_extract_lane::<1>(phase),
            f64x2_extract_lane::<1>(step),
        ),
    )
}
#[inline(always)]
fn edges(phase: v128, step: v128) -> v128 {
    let next = f64x2_add(phase, step);
    if i64x2_bitmask(f64x2_gt(step, f64x2_splat(0.0))) == 3 {
        if i64x2_bitmask(f64x2_gt(f64x2_floor(next), f64x2_floor(phase))) == 0 {
            return f64x2_splat(-1.0);
        }
        return boundaries(phase, step);
    }
    let safe = v128_and(
        v128_and(
            f64x2_gt(phase, f64x2_splat(0.0)),
            f64x2_lt(phase, f64x2_splat(1.0)),
        ),
        v128_and(
            f64x2_gt(next, f64x2_splat(0.0)),
            f64x2_lt(next, f64x2_splat(1.0)),
        ),
    );
    if i64x2_bitmask(safe) == 3 {
        f64x2_splat(-1.0)
    } else {
        boundaries(phase, step)
    }
}
#[inline(always)]
fn forward_edges(phase: v128, step: v128, before: v128, after: v128) -> v128 {
    if v128_any_true(f64x2_gt(after, before)) {
        boundaries(phase, step)
    } else {
        f64x2_splat(-1.0)
    }
}
#[inline(always)]
fn self_events(master: v128, step: v128, next: v128) -> v128 {
    let mask = f64x2_ge(next, f64x2_splat(1.0));
    if !v128_any_true(mask) {
        return f64x2_splat(-1.0);
    }
    v128_bitselect(
        f64x2_min(
            f64x2_splat(1.0),
            f64x2_max(
                f64x2_splat(0.0),
                f64x2_div(f64x2_sub(f64x2_splat(1.0), master), step),
            ),
        ),
        f64x2_splat(-1.0),
        mask,
    )
}
#[cold]
#[inline(never)]
#[allow(clippy::too_many_arguments)]
fn resets(
    event: [v128; 2],
    phase: [v128; 2],
    step: [v128; 2],
    pm: f64,
    reset: f64,
    next: &mut [v128; 2],
    edge: &mut [v128; 2],
    heard: &mut [Option<Lookup<'_>>; MAX_UNISON],
    tapped: &mut [Option<Lookup<'_>>; MAX_UNISON],
    audible: &mut v128,
    operator: &mut v128,
    correction: &mut v128,
    tap_correction: &mut v128,
    needed: bool,
) {
    let (events, phases, steps) = (doubles(event), doubles(phase), doubles(step));
    let (mut ns, mut es) = (doubles(*next), doubles(*edge));
    let (mut a, mut t) = (floats(*audible), floats(*operator));
    let (mut ac, mut tc) = ([0.0; 4], [0.0; 4]);
    for id in 0..4 {
        if events[id] < 0.0 {
            continue;
        }
        let event = events[id];
        let f = event as f32;
        let left = 1.0 - f;
        let (jump, slope) = discontinuity(
            heard[id].as_mut().unwrap(),
            phases[id] + steps[id] * event,
            reset,
            pm,
            steps[id],
        );
        a[id] += jump * 0.5 * left * left + slope * left * left * left / 6.0;
        ac[id] = -jump * 0.5 * f * f + slope * f * f * f / 6.0;
        if needed {
            let (jump, slope) = discontinuity(
                tapped[id].as_mut().unwrap(),
                phases[id] + steps[id] * event,
                reset,
                pm,
                steps[id],
            );
            t[id] += jump * 0.5 * left * left + slope * left * left * left / 6.0;
            tc[id] = -jump * 0.5 * f * f + slope * f * f * f / 6.0;
        }
        let f = event.clamp(0.0, 1.0);
        ns[id] = wrap(reset + steps[id] * (1.0 - f));
        es[id] = if es[id] >= 0.0 && es[id] <= f {
            es[id]
        } else {
            -1.0
        };
        let after = crossing(reset + pm, steps[id] * (1.0 - f));
        if after >= 0.0 {
            es[id] = f + (1.0 - f) * after;
        }
    }
    *next = [f64x2(ns[0], ns[1]), f64x2(ns[2], ns[3])];
    *edge = [f64x2(es[0], es[1]), f64x2(es[2], es[3])];
    *audible = f32x4(a[0], a[1], a[2], a[3]);
    *operator = f32x4(t[0], t[1], t[2], t[3]);
    *correction = f32x4(ac[0], ac[1], ac[2], ac[3]);
    *tap_correction = f32x4(tc[0], tc[1], tc[2], tc[3]);
}

#[allow(clippy::too_many_arguments)]
#[inline(never)]
pub(super) fn render(
    state: &mut Graph,
    lanes: &mut [[Lane; MAX_UNISON]; OSCILLATORS],
    layers: &[Layer; OSCILLATORS],
    heard: &mut [[Option<Lookup<'_>>; MAX_UNISON]; OSCILLATORS],
    tapped: &mut [[Option<Lookup<'_>>; MAX_UNISON]; OSCILLATORS],
    steps: &[[f64; MAX_UNISON]; OSCILLATORS],
    masters: &[[f64; MAX_UNISON]; OSCILLATORS],
    gains: &[[Stereo; MAX_UNISON]; OSCILLATORS],
    tap_gains: &[[f32; MAX_UNISON]; OSCILLATORS],
    centers: &[f64; OSCILLATORS],
    depth: &[f32; OSCILLATORS],
    fm: &[bool; OSCILLATORS],
    sync: &[bool; OSCILLATORS],
    needed: &[bool; OSCILLATORS],
    factor: usize,
    internal: f64,
    kernel: &Kernel,
) -> bool {
    let full = needed.iter().all(|&x| x)
        && (0..OSCILLATORS).all(|i| {
            fm[i]
                && sync[i]
                && layers[i].target.fm_mode == 1
                && layers[i].target.sync_source as usize != i
        });
    if full {
        render_inner::<true>(
            state, lanes, layers, heard, tapped, steps, masters, gains, tap_gains, centers, depth,
            fm, sync, needed, factor, internal, kernel,
        )
    } else {
        render_inner::<false>(
            state, lanes, layers, heard, tapped, steps, masters, gains, tap_gains, centers, depth,
            fm, sync, needed, factor, internal, kernel,
        )
    }
}

#[allow(clippy::too_many_arguments)]
#[inline(never)]
fn render_inner<const FULL: bool>(
    state: &mut Graph,
    lanes: &mut [[Lane; MAX_UNISON]; OSCILLATORS],
    layers: &[Layer; OSCILLATORS],
    heard: &mut [[Option<Lookup<'_>>; MAX_UNISON]; OSCILLATORS],
    tapped: &mut [[Option<Lookup<'_>>; MAX_UNISON]; OSCILLATORS],
    steps: &[[f64; MAX_UNISON]; OSCILLATORS],
    masters: &[[f64; MAX_UNISON]; OSCILLATORS],
    gains: &[[Stereo; MAX_UNISON]; OSCILLATORS],
    tap_gains: &[[f32; MAX_UNISON]; OSCILLATORS],
    centers: &[f64; OSCILLATORS],
    depth: &[f32; OSCILLATORS],
    fm: &[bool; OSCILLATORS],
    sync: &[bool; OSCILLATORS],
    needed: &[bool; OSCILLATORS],
    factor: usize,
    internal: f64,
    kernel: &Kernel,
) -> bool {
    // A transition, muted operator or large sinc mip uses the general path.
    // Eligibility finishes before touching DSP state; no render allocations.
    let mut caches = state.lookup_cache.iter_mut();
    let mut audible = std::array::from_fn::<_, OSCILLATORS, _>(|i| {
        batch(&heard[i], caches.next().unwrap(), true)
    });
    let mut operators = std::array::from_fn::<_, OSCILLATORS, _>(|i| {
        batch(&tapped[i], caches.next().unwrap(), false)
    });
    if audible.iter().any(Option::is_none)
        || (0..OSCILLATORS).any(|i| needed[i] && operators[i].is_none())
    {
        return false;
    }
    let common = std::array::from_fn::<_, OSCILLATORS, _>(|i| {
        operators[i]
            .as_ref()
            .is_some_and(|o| audible[i].as_ref().unwrap().same_grid(o))
    });
    let modes = std::array::from_fn::<_, OSCILLATORS, _>(|i| {
        if FULL {
            1
        } else if fm[i] {
            layers[i].target.fm_mode
        } else {
            0
        }
    });
    let fm_rates = std::array::from_fn::<_, OSCILLATORS, _>(|i| {
        if fm[i] {
            centers[layers[i].target.fm_source as usize] / internal
        } else {
            0.0
        }
    });
    for i in 0..OSCILLATORS {
        let source = layers[i].target.fm_source as usize;
        let key = modes[i] + 4 * source.min(3) as u8;
        let initial_pm = if modes[i] == 2 {
            (state.taps[source].clamp(-2.0, 2.0) as f64 * depth[i] as f64) / std::f64::consts::TAU
        } else {
            0.0
        };
        for lane in &mut lanes[i][..4] {
            if lane.mode != key {
                lane.phase = wrap(lane.phase + lane.phase_mod - initial_pm);
                lane.mode = key;
            }
        }
    }
    let mut phases = std::array::from_fn::<_, OSCILLATORS, _>(|i| {
        [
            f64x2(lanes[i][0].phase, lanes[i][1].phase),
            f64x2(lanes[i][2].phase, lanes[i][3].phase),
        ]
    });
    let mut master = std::array::from_fn::<_, OSCILLATORS, _>(|i| {
        [
            f64x2(lanes[i][0].master, lanes[i][1].master),
            f64x2(lanes[i][2].master, lanes[i][3].master),
        ]
    });
    let min_step = steps.map(|s| s[..4].iter().copied().fold(f64::INFINITY, f64::min));
    let steps = steps.map(|s| pack(&s));
    let masters = masters.map(|s| pack(&s));
    let mut edge = state.edges.map(|s| pack(&s));
    let mut corrections = lanes.map(|l| {
        f32x4(
            l[0].correction,
            l[1].correction,
            l[2].correction,
            l[3].correction,
        )
    });
    let mut tap_corrections = state.corrections.map(|c| f32x4(c[0], c[1], c[2], c[3]));
    let g = gains.map(|g| {
        [
            f32x4(g[0][0], g[1][0], g[2][0], g[3][0]),
            f32x4(g[0][1], g[1][1], g[2][1], g[3][1]),
        ]
    });
    let tg = tap_gains.map(|g| f32x4(g[0], g[1], g[2], g[3]));
    let mut poles = std::array::from_fn::<_, 8, _>(|p| {
        f32x4(
            state.lowpass[0][p],
            state.lowpass[1][p],
            state.lowpass[2][p],
            state.lowpass[3][p],
        )
    });
    let alpha = f32x4(
        state.alpha[0],
        state.alpha[1],
        state.alpha[2],
        state.alpha[3],
    );
    let mask = i32x4(
        -i32::from(needed[0] && (fm[0] || sync[0])),
        -i32::from(needed[1] && (fm[1] || sync[1])),
        -i32::from(needed[2] && (fm[2] || sync[2])),
        -i32::from(needed[3] && (fm[3] || sync[3])),
    );
    let mut last_pm = [0.0; OSCILLATORS];
    for sub in 0..factor {
        let old_taps = state.taps;
        let old_edges = edge;
        let mut new_taps = [0.0; OSCILLATORS];
        let mut frame = [0.0; 2];
        macro_rules! operator {
            ($i:literal) => {{
                const I: usize = $i;
                let i = I;
                let source = layers[i].target.fm_source as usize;
                let clock = layers[i].target.sync_source as usize;
                let mode = modes[i];
                let modulation = if FULL || fm[i] {
                    old_taps[source].clamp(-2.0, 2.0) as f64 * depth[i] as f64
                } else {
                    0.0
                };
                let fm_step = if mode == 1 {
                    modulation * fm_rates[i]
                } else {
                    0.0
                };
                let pm = if mode == 2 {
                    modulation / std::f64::consts::TAU
                } else {
                    0.0
                };
                last_pm[i] = pm;
                let read_phase = if FULL {
                    phases[i]
                } else {
                    phases[i].map(|p| f64x2_add(p, f64x2_splat(pm)))
                };
                let read_floor = read_phase.map(|v| f64x2_floor(v));
                let normalized = std::array::from_fn(|j| {
                    let q = f64x2_sub(read_phase[j], read_floor[j]);
                    v128_bitselect(f64x2_splat(0.0), q, f64x2_ge(q, f64x2_splat(1.0)))
                });
                let step = steps[i].map(|s| f64x2_add(s, f64x2_splat(fm_step)));
                let values = Batch4::pair(
                    audible[i].as_mut().unwrap(),
                    if FULL || needed[i] {
                        operators[i].as_mut()
                    } else {
                        None
                    },
                    normalized,
                    common[i],
                );
                let mut a = f32x4_add(values[0], corrections[i]);
                let mut t = if FULL || needed[i] {
                    f32x4_add(values[1], tap_corrections[i])
                } else {
                    f32x4_splat(0.0)
                };
                corrections[i] = f32x4_splat(0.0);
                tap_corrections[i] = f32x4_splat(0.0);
                let raw_next = [
                    f64x2_add(phases[i][0], step[0]),
                    f64x2_add(phases[i][1], step[1]),
                ];
                let next_floor = raw_next.map(|v| f64x2_floor(v));
                let mut next = [
                    f64x2_sub(raw_next[0], next_floor[0]),
                    f64x2_sub(raw_next[1], next_floor[1]),
                ];
                edge[i] = if fm_step > -min_step[i] {
                    let end = if pm == 0.0 {
                        next_floor
                    } else {
                        [
                            f64x2_floor(f64x2_add(read_phase[0], step[0])),
                            f64x2_floor(f64x2_add(read_phase[1], step[1])),
                        ]
                    };
                    [
                        forward_edges(read_phase[0], step[0], read_floor[0], end[0]),
                        forward_edges(read_phase[1], step[1], read_floor[1], end[1]),
                    ]
                } else {
                    [edges(read_phase[0], step[0]), edges(read_phase[1], step[1])]
                };
                let master_next = [
                    f64x2_add(master[i][0], masters[i][0]),
                    f64x2_add(master[i][1], masters[i][1]),
                ];
                if FULL || sync[i] {
                    let event = if !FULL && clock == i {
                        std::array::from_fn(|j| {
                            self_events(master[i][j], masters[i][j], master_next[j])
                        })
                    } else {
                        old_edges[clock]
                    };
                    if v128_any_true(v128_or(
                        f64x2_ge(event[0], f64x2_splat(0.0)),
                        f64x2_ge(event[1], f64x2_splat(0.0)),
                    )) {
                        resets(
                            event,
                            phases[i],
                            step,
                            pm,
                            layers[i].target.phase as f64,
                            &mut next,
                            &mut edge[i],
                            &mut heard[i],
                            &mut tapped[i],
                            &mut a,
                            &mut t,
                            &mut corrections[i],
                            &mut tap_corrections[i],
                            needed[i],
                        );
                    }
                }
                phases[i] = next;
                master[i] = master_next.map(wrapped);
                if FULL || needed[i] {
                    new_taps[i] = sum(0.0, f32x4_mul(t, tg[i]));
                }
                frame[0] = sum(frame[0], f32x4_mul(a, g[i][0]));
                frame[1] = sum(frame[1], f32x4_mul(a, g[i][1]));
            }};
        }
        operator!(0);
        operator!(1);
        operator!(2);
        operator!(3);
        let raw = f32x4(new_taps[0], new_taps[1], new_taps[2], new_taps[3]);
        let mut v = f32x4_min(f32x4_splat(2.0), f32x4_max(f32x4_splat(-2.0), raw));
        for pole in &mut poles {
            let updated = f32x4_add(*pole, f32x4_mul(alpha, f32x4_sub(v, *pole)));
            *pole = if FULL {
                updated
            } else {
                v128_bitselect(updated, raw, mask)
            };
            v = *pole;
        }
        state.taps = floats(v);
        if factor == 32 {
            let value = state.filter.pre(frame, kernel, sub % 4 == 3);
            if sub % 4 == 3 {
                state.filter.push(value);
            }
        } else {
            state.filter.push(frame);
        }
    }
    for (p, pole) in poles.into_iter().enumerate() {
        for (i, value) in floats(pole).into_iter().enumerate() {
            state.lowpass[i][p] = value;
        }
    }
    for i in 0..OSCILLATORS {
        let p = doubles(phases[i]);
        let m = doubles(master[i]);
        let e = doubles(edge[i]);
        let c = floats(corrections[i]);
        let tc = floats(tap_corrections[i]);
        for id in 0..4 {
            lanes[i][id].phase = p[id];
            lanes[i][id].master = m[id];
            lanes[i][id].correction = c[id];
            lanes[i][id].phase_mod = last_pm[i];
            state.corrections[i][id] = tc[id];
            state.edges[i][id] = e[id];
        }
    }
    true
}
