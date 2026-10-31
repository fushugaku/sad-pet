//! Four polynomial readers with coefficients retained in SIMD registers.
//! Gather/transpose only on a cell change, never on a cache hit. This is the
//! same degree-seven fit and non-fused Estrin evaluation as Reader. Heard
//! mips with identical grids can share one polynomial after f64 coefficient
//! mixing; operator taps retain their original mip arithmetic.
use super::*;
use core::arch::wasm32::*;

pub(in crate::dsp::wavetable) struct Batch4<'a, 'c> {
    low: Cells<'c>,
    high: Option<Cells<'c>>,
    blend: v128,
    common_high: bool,
    _bank: core::marker::PhantomData<&'a Bank>,
}
struct Cells<'c> {
    frames: [Polynomial; 4],
    cache: &'c mut CellCache,
    lengths: [v128; 2],
    lengths_scalar: [f64; 4],
}
// Persistent cache contains numbers and identity keys, never dereferenceable
// pointers or bank borrows. The full content hash prevents allocator reuse
// from mistaking a newly loaded bank for the retired one.
#[derive(Clone, Copy)]
pub(in crate::dsp::wavetable) struct Cache {
    identity: [u8; 64],
    low: CellCache,
    high: CellCache,
}
#[derive(Clone, Copy)]
struct CellCache {
    keys: [CellKey; 4],
    coefficients: [v128; 8],
    cells: v128,
}
type CellKey = (usize, usize, u32, u64, usize, usize, u32, u32);
impl Default for Cache {
    fn default() -> Self {
        let empty = CellCache {
            keys: [(0, 0, 0, 0, 0, 0, 0, 0); 4],
            coefficients: [f32x4_splat(0.0); 8],
            cells: i32x4_splat(-1),
        };
        Self {
            identity: [0; 64],
            low: empty,
            high: empty,
        }
    }
}
#[derive(Clone, Copy)]
struct Polynomial {
    from: *const f32,
    into: *const f32,
    mix: f32,
    mip: Option<(*const f32, *const f32, f32, f32)>,
}
impl<'c> Cells<'c> {
    fn new(
        frames: [&Frame; 4],
        mip: Option<([&Frame; 4], [f32; 4])>,
        cache: &'c mut CellCache,
    ) -> Option<Self> {
        if frames.iter().any(|f| f.polynomial.is_none()) {
            return None;
        }
        for (i, f) in frames.iter().enumerate() {
            let (a, b) = f.polynomial.unwrap();
            let m = mip.map(|(h, blend)| (h[i], blend[i]));
            let (ma, mb) = m
                .and_then(|(h, _)| h.polynomial)
                .unwrap_or((core::ptr::null(), core::ptr::null()));
            let key = (
                a as usize,
                b as usize,
                f.mix.to_bits(),
                f.length.to_bits(),
                ma as usize,
                mb as usize,
                m.map_or(0, |(h, _)| h.mix.to_bits()),
                m.map_or(0, |(_, b)| b.to_bits()),
            );
            if key != cache.keys[i] {
                cache.keys[i] = key;
                cache.cells = match i {
                    0 => i32x4_replace_lane::<0>(cache.cells, -1),
                    1 => i32x4_replace_lane::<1>(cache.cells, -1),
                    2 => i32x4_replace_lane::<2>(cache.cells, -1),
                    _ => i32x4_replace_lane::<3>(cache.cells, -1),
                };
            }
        }
        Some(Self {
            frames: std::array::from_fn(|i| {
                let f = frames[i];
                let (from, into) = f.polynomial.unwrap();
                Polynomial {
                    from,
                    into,
                    mix: f.mix,
                    mip: mip.and_then(|(h, blend)| {
                        if blend[i] == 0.0 {
                            return None;
                        }
                        h[i].polynomial.map(|(a, b)| (a, b, h[i].mix, blend[i]))
                    }),
                }
            }),
            cache,
            lengths: [
                f64x2(frames[0].length, frames[1].length),
                f64x2(frames[2].length, frames[3].length),
            ],
            lengths_scalar: frames.map(|f| f.length),
        })
    }
    #[cold]
    #[inline(never)]
    fn update(&mut self, lane: usize, index: usize) {
        let f = &self.frames[lane];
        let (a, b) = (f.from, f.into);
        for (i, c) in self.cache.coefficients.iter_mut().enumerate() {
            // The parent Reader borrows a validated immutable bank. Each
            // normalized phase indexes one of length cells of eight floats.
            let mut x = unsafe {
                let x = *a.add(index * 8 + i);
                x + (*b.add(index * 8 + i) - x) * f.mix
            };
            if let Some((a, b, mix, blend)) = f.mip {
                let high = unsafe {
                    let a = *a.add(index * 8 + i);
                    a + (*b.add(index * 8 + i) - a) * mix
                };
                // Mips on the same grid are polynomials of the same variable.
                // Blend their coefficients in f64 once per cell. The heard
                // signal differs only by f32 rounding; FM taps stay strict.
                x = (x as f64 + (high as f64 - x as f64) * blend as f64) as f32;
            }
            *c = match lane {
                0 => f32x4_replace_lane::<0>(*c, x),
                1 => f32x4_replace_lane::<1>(*c, x),
                2 => f32x4_replace_lane::<2>(*c, x),
                _ => f32x4_replace_lane::<3>(*c, x),
            };
        }
    }
    #[inline(always)]
    fn position(&self, phase: [v128; 2]) -> (v128, v128) {
        let a = f64x2_mul(phase[0], self.lengths[0]);
        let b = f64x2_mul(phase[1], self.lengths[1]);
        let ia = i32x4_trunc_sat_f64x2_zero(a);
        let ib = i32x4_trunc_sat_f64x2_zero(b);
        let cells = i32x4_shuffle::<0, 1, 4, 5>(ia, ib);
        let ta = f32x4_demote_f64x2_zero(f64x2_sub(a, f64x2_convert_low_i32x4(ia)));
        let tb = f32x4_demote_f64x2_zero(f64x2_sub(b, f64x2_convert_low_i32x4(ib)));
        (cells, i32x4_shuffle::<0, 1, 4, 5>(ta, tb))
    }
    #[inline(always)]
    fn read_at(&mut self, (cells, t): (v128, v128)) -> v128 {
        let changed = i32x4_ne(cells, self.cache.cells);
        if v128_any_true(changed) {
            let mask = i32x4_bitmask(changed);
            if mask & 1 != 0 {
                self.update(0, i32x4_extract_lane::<0>(cells) as usize);
            }
            if mask & 2 != 0 {
                self.update(1, i32x4_extract_lane::<1>(cells) as usize);
            }
            if mask & 4 != 0 {
                self.update(2, i32x4_extract_lane::<2>(cells) as usize);
            }
            if mask & 8 != 0 {
                self.update(3, i32x4_extract_lane::<3>(cells) as usize);
            }
            self.cache.cells = cells;
        }
        let t2 = f32x4_mul(t, t);
        let t4 = f32x4_mul(t2, t2);
        let c = &self.cache.coefficients;
        let p0 = f32x4_add(c[0], f32x4_mul(c[1], t));
        let p1 = f32x4_add(c[2], f32x4_mul(c[3], t));
        let p2 = f32x4_add(c[4], f32x4_mul(c[5], t));
        let p3 = f32x4_add(c[6], f32x4_mul(c[7], t));
        f32x4_add(
            f32x4_add(p0, f32x4_mul(p1, t2)),
            f32x4_mul(f32x4_add(p2, f32x4_mul(p3, t2)), t4),
        )
    }
}
impl<'a, 'c> Batch4<'a, 'c> {
    pub(in crate::dsp::wavetable) fn new(
        readers: [&Reader<'a>; 4],
        cache: &'c mut Cache,
        merge_mips: bool,
    ) -> Option<Self> {
        if readers[1..]
            .iter()
            .any(|r| !core::ptr::eq(r.bank, readers[0].bank))
        {
            return None;
        }
        let identity: &[u8; 64] = readers[0].bank.content_hash.as_bytes().try_into().ok()?;
        if &cache.identity != identity {
            cache.identity = *identity;
            cache.low.cells = i32x4_splat(-1);
            cache.high.cells = i32x4_splat(-1);
        }
        // Duplicating low for a lane without a high mip gives a zero high
        // difference; that lane's blend is zero. No extra bank data is made.
        let highs = readers.map(|r| r.high.as_ref().unwrap_or(&r.low));
        let merge = merge_mips
            && readers
                .iter()
                .enumerate()
                .all(|(i, r)| r.low.length == highs[i].length && highs[i].polynomial.is_some());
        let low = Cells::new(
            readers.map(|r| &r.low),
            merge.then(|| (highs, readers.map(|r| r.blend))),
            &mut cache.low,
        )?;
        let high = if !merge && readers.iter().any(|r| r.high.is_some()) {
            Some(Cells::new(
                readers.map(|r| r.high.as_ref().unwrap_or(&r.low)),
                None,
                &mut cache.high,
            )?)
        } else {
            None
        };
        Some(Self {
            common_high: high
                .as_ref()
                .is_none_or(|h| h.lengths_scalar == low.lengths_scalar),
            low,
            high,
            blend: f32x4(
                readers[0].blend,
                readers[1].blend,
                readers[2].blend,
                readers[3].blend,
            ),
            _bank: core::marker::PhantomData,
        })
    }
    pub(in crate::dsp::wavetable) fn same_grid(&self, other: &Self) -> bool {
        self.low.lengths_scalar == other.low.lengths_scalar
    }
    #[inline(always)]
    pub(in crate::dsp::wavetable) fn sample(&mut self, phase: [v128; 2]) -> v128 {
        let position = self.low.position(phase);
        self.read_at(phase, position)
    }
    #[inline(always)]
    fn read_at(&mut self, phase: [v128; 2], position: (v128, v128)) -> v128 {
        // The caller normalizes finite f64 phases once for heard/operator
        // reads. Equal grids also share cell/fraction calculation.
        let low = self.low.read_at(position);
        if let Some(high) = &mut self.high {
            let p = if self.common_high {
                position
            } else {
                high.position(phase)
            };
            f32x4_add(low, f32x4_mul(f32x4_sub(high.read_at(p), low), self.blend))
        } else {
            low
        }
    }
    #[inline(always)]
    pub(in crate::dsp::wavetable) fn pair(
        heard: &mut Self,
        tapped: Option<&mut Self>,
        phase: [v128; 2],
        common: bool,
    ) -> [v128; 2] {
        let position = heard.low.position(phase);
        let a = heard.read_at(phase, position);
        let b = if let Some(tapped) = tapped {
            let p = if common {
                position
            } else {
                tapped.low.position(phase)
            };
            tapped.read_at(phase, p)
        } else {
            f32x4_splat(0.0)
        };
        [a, b]
    }
}
