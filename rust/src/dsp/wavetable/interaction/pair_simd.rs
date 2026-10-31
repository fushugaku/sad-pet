//! Four-lane compatibility renderer: source tap still precedes source sync.
use super::*;
use crate::dsp::wavetable::lookup::{Batch4, BatchCache};
use core::arch::wasm32::*;

fn batch<'a, 'c>(
    views: &[Option<Lookup<'a>>; MAX_UNISON],
    cache: &'c mut BatchCache,
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
fn pack(x: [f64; 4]) -> [v128; 2] {
    [f64x2(x[0], x[1]), f64x2(x[2], x[3])]
}
#[inline(always)]
fn unpack(x: [v128; 2]) -> [f64; 4] {
    [
        f64x2_extract_lane::<0>(x[0]),
        f64x2_extract_lane::<1>(x[0]),
        f64x2_extract_lane::<0>(x[1]),
        f64x2_extract_lane::<1>(x[1]),
    ]
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
fn sum(start: f32, x: v128) -> f32 {
    let a = floats(x);
    (((start + a[0]) + a[1]) + a[2]) + a[3]
}
#[inline(always)]
fn wrapped(p: v128) -> v128 {
    f64x2_sub(p, f64x2_floor(p))
}
#[inline(always)]
fn normalized(p: [v128; 2]) -> [v128; 2] {
    p.map(|p| {
        let q = wrapped(p);
        v128_bitselect(f64x2_splat(0.0), q, f64x2_ge(q, f64x2_splat(1.0)))
    })
}
#[inline(always)]
fn events(master: [v128; 2], step: [v128; 2], sync: bool) -> [v128; 2] {
    if !sync {
        return [f64x2_splat(-1.0); 2];
    }
    std::array::from_fn(|j| {
        let mask = f64x2_ge(f64x2_add(master[j], step[j]), f64x2_splat(1.0));
        if !v128_any_true(mask) {
            return f64x2_splat(-1.0);
        }
        v128_bitselect(
            f64x2_min(
                f64x2_splat(1.0),
                f64x2_max(
                    f64x2_splat(0.0),
                    f64x2_div(f64x2_sub(f64x2_splat(1.0), master[j]), step[j]),
                ),
            ),
            f64x2_splat(-1.0),
            mask,
        )
    })
}
#[inline(always)]
fn has_event(e: [v128; 2]) -> bool {
    v128_any_true(v128_or(
        f64x2_ge(e[0], f64x2_splat(0.0)),
        f64x2_ge(e[1], f64x2_splat(0.0)),
    ))
}
#[cold]
#[inline(never)]
#[allow(clippy::too_many_arguments)]
fn resets(
    views: &mut [Option<Lookup<'_>>; MAX_UNISON],
    sample: &mut v128,
    correction: &mut v128,
    phase: [v128; 2],
    next: &mut [v128; 2],
    step: [v128; 2],
    event: [v128; 2],
    reset: f64,
    pm: f64,
) {
    let (p, s, e) = (unpack(phase), unpack(step), unpack(event));
    let mut n = unpack(*next);
    let mut a = floats(*sample);
    let mut c = [0.0; 4];
    for i in 0..4 {
        if e[i] < 0.0 {
            continue;
        }
        let f = e[i] as f32;
        let left = 1.0 - f;
        let (jump, slope) = discontinuity(
            views[i].as_mut().unwrap(),
            p[i] + s[i] * e[i],
            reset,
            pm,
            s[i],
        );
        a[i] += jump * 0.5 * left * left + slope * left * left * left / 6.0;
        c[i] = -jump * 0.5 * f * f + slope * f * f * f / 6.0;
        n[i] = wrap(reset + s[i] * (1.0 - e[i]));
    }
    *sample = f32x4(a[0], a[1], a[2], a[3]);
    *correction = f32x4(c[0], c[1], c[2], c[3]);
    *next = pack(n);
}
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn advance(
    phase: &mut [v128; 2],
    master: &mut [v128; 2],
    step: [v128; 2],
    master_step: [v128; 2],
    sync: bool,
    reset: f64,
    pm: f64,
    sample: v128,
    correction: &mut v128,
    views: &mut [Option<Lookup<'_>>; MAX_UNISON],
) -> v128 {
    let event = events(*master, master_step, sync);
    let mut next = [
        wrapped(f64x2_add(phase[0], step[0])),
        wrapped(f64x2_add(phase[1], step[1])),
    ];
    let mut sample = f32x4_add(sample, *correction);
    *correction = f32x4_splat(0.0);
    if has_event(event) {
        resets(
            views,
            &mut sample,
            correction,
            *phase,
            &mut next,
            step,
            event,
            reset,
            pm,
        );
    }
    *phase = next;
    *master = [
        wrapped(f64x2_add(master[0], master_step[0])),
        wrapped(f64x2_add(master[1], master_step[1])),
    ];
    sample
}
#[inline(always)]
fn silent4(
    phase: &mut [v128; 2],
    master: &mut [v128; 2],
    step: [v128; 2],
    master_step: [v128; 2],
    sync: bool,
    reset: f64,
) {
    let event = events(*master, master_step, sync);
    for j in 0..2 {
        let mut n = wrapped(f64x2_add(phase[j], step[j]));
        let mask = f64x2_ge(event[j], f64x2_splat(0.0));
        if v128_any_true(mask) {
            n = v128_bitselect(
                wrapped(f64x2_add(
                    f64x2_splat(reset),
                    f64x2_mul(step[j], f64x2_sub(f64x2_splat(1.0), event[j])),
                )),
                n,
                mask,
            );
        }
        phase[j] = n;
        master[j] = wrapped(f64x2_add(master[j], master_step[j]));
    }
}
#[allow(clippy::too_many_arguments)]
#[inline(never)]
pub(super) fn render(
    filter: &mut Filter,
    caches: &mut [BatchCache; 3],
    carriers: &mut [Lane; MAX_UNISON],
    modulators: &mut [Lane; MAX_UNISON],
    source_views: &mut [Option<Lookup<'_>>; MAX_UNISON],
    slave_views: &mut [Option<Lookup<'_>>; MAX_UNISON],
    carrier_views: &mut [Option<Lookup<'_>>; MAX_UNISON],
    source_master: [f64; MAX_UNISON],
    source_steps: [f64; MAX_UNISON],
    carrier_master: [f64; MAX_UNISON],
    carrier_steps: [f64; MAX_UNISON],
    source_gains: [Stereo; MAX_UNISON],
    carrier_gains: [Stereo; MAX_UNISON],
    source_scales: [f32; MAX_UNISON],
    fm: bool,
    mode: u8,
    depth: f32,
    fm_step: f64,
    source_sync: bool,
    carrier_sync: bool,
    source_reset: f64,
    carrier_reset: f64,
    factor: usize,
    kernel: &Kernel,
) -> bool {
    let [source_cache, slave_cache, carrier_cache] = caches;
    let mut source = if fm {
        batch(source_views, source_cache, false)
    } else {
        None
    };
    let (Some(mut slave), Some(mut carrier)) = (
        batch(slave_views, slave_cache, true),
        batch(carrier_views, carrier_cache, true),
    ) else {
        return false;
    };
    if fm && source.is_none() {
        return false;
    }
    let sm = pack(std::array::from_fn(|i| source_master[i]));
    let ss = pack(std::array::from_fn(|i| source_steps[i]));
    let cm = pack(std::array::from_fn(|i| carrier_master[i]));
    let cs = pack(std::array::from_fn(|i| carrier_steps[i]));
    let im = pack(std::array::from_fn(|i| carrier_master[i + 4]));
    let is = pack(std::array::from_fn(|i| carrier_steps[i + 4]));
    let mut sp = pack(std::array::from_fn(|i| modulators[i].phase));
    let mut master = pack(std::array::from_fn(|i| modulators[i].master));
    let mut cp = pack(std::array::from_fn(|i| carriers[i].phase));
    let mut clock = pack(std::array::from_fn(|i| carriers[i].master));
    let mut ip = pack(std::array::from_fn(|i| carriers[i + 4].phase));
    let mut ic = pack(std::array::from_fn(|i| carriers[i + 4].master));
    let mut sc = f32x4(
        modulators[0].correction,
        modulators[1].correction,
        modulators[2].correction,
        modulators[3].correction,
    );
    let mut cc = f32x4(
        carriers[0].correction,
        carriers[1].correction,
        carriers[2].correction,
        carriers[3].correction,
    );
    let sg = std::array::from_fn::<_, 2, _>(|c| {
        f32x4(
            source_gains[0][c],
            source_gains[1][c],
            source_gains[2][c],
            source_gains[3][c],
        )
    });
    let cg = std::array::from_fn::<_, 2, _>(|c| {
        f32x4(
            carrier_gains[0][c],
            carrier_gains[1][c],
            carrier_gains[2][c],
            carrier_gains[3][c],
        )
    });
    let scales = f32x4(
        source_scales[0],
        source_scales[1],
        source_scales[2],
        source_scales[3],
    );
    let mut last_pm = 0.0;
    for sub in 0..factor {
        let m = if let Some(source) = &mut source {
            source.sample(normalized(master))
        } else {
            f32x4_splat(0.0)
        };
        let s = slave.sample(normalized(sp));
        let s = advance(
            &mut sp,
            &mut master,
            ss,
            sm,
            source_sync,
            source_reset,
            0.0,
            s,
            &mut sc,
            slave_views,
        );
        let modulation_sum = if fm {
            sum(
                0.0,
                f32x4_mul(
                    f32x4_min(f32x4_splat(2.0), f32x4_max(f32x4_splat(-2.0), m)),
                    scales,
                ),
            )
        } else {
            0.0
        };
        let modulation = modulation_sum as f64 * depth as f64;
        let pm = if mode == 2 {
            modulation / std::f64::consts::TAU
        } else {
            0.0
        };
        last_pm = pm;
        if sub == 0 {
            for lane in carriers.iter_mut() {
                if lane.mode != mode {
                    lane.phase = wrap(lane.phase + lane.phase_mod - pm);
                    lane.mode = mode;
                }
            }
            cp = pack(std::array::from_fn(|i| carriers[i].phase));
            ip = pack(std::array::from_fn(|i| carriers[i + 4].phase));
        }
        let delta = if mode == 1 { modulation * fm_step } else { 0.0 };
        let step = cs.map(|s| f64x2_add(s, f64x2_splat(delta)));
        let read = cp.map(|p| f64x2_add(p, f64x2_splat(pm)));
        let c = carrier.sample(normalized(read));
        let carrier_sample = advance(
            &mut cp,
            &mut clock,
            step,
            cm,
            carrier_sync,
            carrier_reset,
            pm,
            c,
            &mut cc,
            carrier_views,
        );
        silent4(
            &mut ip,
            &mut ic,
            is.map(|s| f64x2_add(s, f64x2_splat(delta))),
            im,
            carrier_sync,
            carrier_reset,
        );
        let frame = std::array::from_fn(|c| {
            sum(
                sum(0.0, f32x4_mul(s, sg[c])),
                f32x4_mul(carrier_sample, cg[c]),
            )
        });
        if factor == 32 {
            let value = filter.pre(frame, kernel, sub % 4 == 3);
            if sub % 4 == 3 {
                filter.push(value);
            }
        } else {
            filter.push(frame);
        }
    }
    let (sp, sm, cp, cm, ip, im, sc, cc) = (
        unpack(sp),
        unpack(master),
        unpack(cp),
        unpack(clock),
        unpack(ip),
        unpack(ic),
        floats(sc),
        floats(cc),
    );
    for i in 0..4 {
        modulators[i].phase = sp[i];
        modulators[i].master = sm[i];
        modulators[i].correction = sc[i];
        carriers[i].phase = cp[i];
        carriers[i].master = cm[i];
        carriers[i].correction = cc[i];
        carriers[i + 4].phase = ip[i];
        carriers[i + 4].master = im[i];
        carriers[i + 4].correction = 0.0;
    }
    for lane in carriers {
        lane.phase_mod = last_pm;
    }
    true
}
