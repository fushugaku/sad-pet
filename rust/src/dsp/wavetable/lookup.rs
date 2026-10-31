//! Prepared reads retain Clean's 16-tap phase kernel. Small mips use cached
//! polynomial cells fitted to that kernel; large mips keep the full sinc.
//! Large-mip sinc reductions use SIMD where available.
use super::{Bank, Kernel, Selection, TAPS};
#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
mod simd;
#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
pub(in crate::dsp::wavetable) use simd::Batch4;
#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
pub(in crate::dsp::wavetable) use simd::Cache as BatchCache;

#[derive(Clone, Copy)]
struct Frame {
    from: *const f32,
    into: *const f32,
    length: f64,
    mix: f32,
    polynomial: Option<(*const f32, *const f32)>,
    cell: usize,
    coefficients: [f32; 8],
}
#[derive(Clone, Copy)]
pub(super) struct Reader<'a> {
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    bank: &'a Bank,
    low: Frame,
    high: Option<Frame>,
    blend: f32,
    kernel: &'static Kernel,
    _bank: core::marker::PhantomData<&'a Bank>,
}
impl<'a> Reader<'a> {
    pub(super) fn new(bank: &'a Bank, position: f32, selection: Selection) -> Self {
        let frame = position.clamp(0.0, 1.0) * (bank.frames - 1) as f32;
        let first = (frame as usize).min(bank.frames - 1);
        let next = (first + 1).min(bank.frames - 1);
        let mix = frame - first as f32;
        let at = |index: usize| {
            let level = bank.levels[index];
            let start = level.offset + first * level.stride;
            let end = level.offset + next * level.stride;
            Frame {
                from: bank.data[start..start + level.stride].as_ptr(),
                into: bank.data[end..end + level.stride].as_ptr(),
                length: level.length as f64,
                mix,
                cell: usize::MAX,
                coefficients: [0.0; 8],
                polynomial: if level.polynomial == usize::MAX {
                    None
                } else {
                    let stride = level.length * 8;
                    let a = level.polynomial + first * stride;
                    let b = level.polynomial + next * stride;
                    Some((
                        bank.data[a..a + stride].as_ptr(),
                        bank.data[b..b + stride].as_ptr(),
                    ))
                },
            }
        };
        Self {
            #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
            bank,
            low: at(selection.low),
            high: (selection.blend != 0.0)
                .then(|| at((selection.low + 1).min(bank.levels.len() - 1))),
            blend: selection.blend,
            kernel: bank.kernel,
            _bank: core::marker::PhantomData,
        }
    }
    #[inline(always)]
    pub(super) fn read(&mut self, phase: f64) -> f32 {
        if !phase.is_finite() {
            return 0.0;
        }
        let phase = if (0.0..1.0).contains(&phase) {
            phase
        } else {
            let p = phase - phase.floor();
            if p >= 1.0 { 0.0 } else { p }
        };
        let low = self.low.read(phase, self.kernel);
        if let Some(high) = &mut self.high {
            low + (high.read(phase, self.kernel) - low) * self.blend
        } else {
            low
        }
    }
}
impl Frame {
    #[inline(always)]
    fn read(&mut self, phase: f64, kernel: &Kernel) -> f32 {
        let at = phase * self.length;
        let index = at as usize;
        if let Some((a, b)) = self.polynomial {
            let t = (at - index as f64) as f32;
            if index != self.cell {
                self.coefficients = std::array::from_fn(|i| {
                    // Reader owns a borrow of the immutable bank. The phase
                    // is finite and normalized, so index < length; each cell
                    // has exactly eight validated coefficients.
                    unsafe {
                        let a = *a.add(index * 8 + i);
                        a + (*b.add(index * 8 + i) - a) * self.mix
                    }
                });
                self.cell = index;
            }
            let c = &self.coefficients;
            let t2 = t * t;
            let t4 = t2 * t2;
            let p0 = madd(c[1], t, c[0]);
            let p1 = madd(c[3], t, c[2]);
            let p2 = madd(c[5], t, c[4]);
            let p3 = madd(c[7], t, c[6]);
            return madd(madd(p3, t2, p2), t4, madd(p1, t2, p0));
        }
        let (k, t) = kernel.position(at - index as f64);
        // Every bank has TAPS/2 periodic guards on either side. These exact
        // TAPS-long views prove all SIMD loads below are in bounds.
        interpolate(
            // The normalized index and the bank's periodic guards keep both
            // sixteen-float views in the bank borrowed by Reader.
            unsafe { &*self.from.add(index + 1).cast::<[f32; TAPS]>() },
            unsafe { &*self.into.add(index + 1).cast::<[f32; TAPS]>() },
            &kernel.phases[k],
            &kernel.phases[k + 1],
            self.mix,
            t,
        )
    }
}

#[inline(always)]
fn madd(a: f32, b: f32, c: f32) -> f32 {
    #[cfg(target_arch = "aarch64")]
    {
        a.mul_add(b, c)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        a * b + c
    }
}

#[inline(always)]
fn interpolate(
    a: &[f32; TAPS],
    b: &[f32; TAPS],
    w: &[f32; TAPS],
    v: &[f32; TAPS],
    mix: f32,
    t: f32,
) -> f32 {
    let sums: [f32; 4];
    #[cfg(target_arch = "aarch64")]
    {
        use core::arch::aarch64::*;
        // AArch64 guarantees NEON. All pointers reference sixteen valid floats;
        // vld1q permits unaligned loads. Deliberately no fused multiply-add.
        unsafe {
            let (m, f) = (vdupq_n_f32(mix), vdupq_n_f32(t));
            let mut sum = vdupq_n_f32(0.0);
            for i in (0..TAPS).step_by(4) {
                let (aa, bb, ww, vv) = (
                    vld1q_f32(a.as_ptr().add(i)),
                    vld1q_f32(b.as_ptr().add(i)),
                    vld1q_f32(w.as_ptr().add(i)),
                    vld1q_f32(v.as_ptr().add(i)),
                );
                let wave = vaddq_f32(aa, vmulq_f32(vsubq_f32(bb, aa), m));
                let weight = vaddq_f32(ww, vmulq_f32(vsubq_f32(vv, ww), f));
                sum = vaddq_f32(sum, vmulq_f32(wave, weight));
            }
            let mut out = [0.0; 4];
            vst1q_f32(out.as_mut_ptr(), sum);
            sums = out;
        }
    }
    #[cfg(target_arch = "x86_64")]
    {
        use core::arch::x86_64::*;
        // SSE is part of x86-64's baseline; loadu handles periodic guard offsets.
        unsafe {
            let (m, f) = (_mm_set1_ps(mix), _mm_set1_ps(t));
            let mut sum = _mm_setzero_ps();
            for i in (0..TAPS).step_by(4) {
                let (aa, bb, ww, vv) = (
                    _mm_loadu_ps(a.as_ptr().add(i)),
                    _mm_loadu_ps(b.as_ptr().add(i)),
                    _mm_loadu_ps(w.as_ptr().add(i)),
                    _mm_loadu_ps(v.as_ptr().add(i)),
                );
                let wave = _mm_add_ps(aa, _mm_mul_ps(_mm_sub_ps(bb, aa), m));
                let weight = _mm_add_ps(ww, _mm_mul_ps(_mm_sub_ps(vv, ww), f));
                sum = _mm_add_ps(sum, _mm_mul_ps(wave, weight));
            }
            let mut out = [0.0; 4];
            _mm_storeu_ps(out.as_mut_ptr(), sum);
            sums = out;
        }
    }
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    {
        use core::arch::wasm32::*;
        unsafe {
            let (m, f) = (f32x4_splat(mix), f32x4_splat(t));
            let mut sum = f32x4_splat(0.0);
            for i in (0..TAPS).step_by(4) {
                let (aa, bb, ww, vv) = (
                    v128_load(a.as_ptr().add(i).cast()),
                    v128_load(b.as_ptr().add(i).cast()),
                    v128_load(w.as_ptr().add(i).cast()),
                    v128_load(v.as_ptr().add(i).cast()),
                );
                let wave = f32x4_add(aa, f32x4_mul(f32x4_sub(bb, aa), m));
                let weight = f32x4_add(ww, f32x4_mul(f32x4_sub(vv, ww), f));
                sum = f32x4_add(sum, f32x4_mul(wave, weight));
            }
            let mut out = [0.0; 4];
            v128_store(out.as_mut_ptr().cast(), sum);
            sums = out;
        }
    }
    #[cfg(not(any(
        target_arch = "aarch64",
        target_arch = "x86_64",
        all(target_arch = "wasm32", target_feature = "simd128")
    )))]
    {
        let mut out = [0.0; 4];
        for i in 0..TAPS {
            let wave = a[i] + (b[i] - a[i]) * mix;
            let weight = w[i] + (v[i] - w[i]) * t;
            out[i % 4] += wave * weight;
        }
        sums = out;
    }
    (sums[0] + sums[1]) + (sums[2] + sums[3])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn prepared_reader_keeps_full_sinc_kernel_frames_mips_and_periodic_edges() {
        let asset = crate::wavetable::assets::Asset::from_cycles(
            "Lookup comparison".into(),
            2048,
            &(0..4096)
                .map(|i| {
                    let p = (i % 2048) as f64 / 2048.0 * std::f64::consts::TAU;
                    ((p.sin() + 0.2 * (p * 63.0 + 0.3).sin()) * if i < 2048 { 0.5 } else { 0.2 })
                        as f32
                })
                .collect::<Vec<_>>(),
        )
        .unwrap();
        let bank = Bank::compile(&asset).unwrap();
        let mut worst = 0.0_f32;
        for low in 0..bank.levels.len() {
            for blend in [0.0, 0.3, 1.0] {
                for position in [0.0, 0.123, 0.5, 1.0] {
                    let selection = Selection { low, blend };
                    let mut reader = Reader::new(&bank, position, selection);
                    for n in 0..1000 {
                        let phase = n as f64 * 0.014321 - 2.314;
                        let expected = bank.read(phase, position, selection);
                        let actual = reader.read(phase);
                        worst = worst.max((actual - expected).abs());
                    }
                    for phase in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                        assert_eq!(reader.read(phase), 0.0);
                    }
                    for phase in [f64::MAX, -f64::MAX, -1.0, 0.0, 1.0] {
                        assert!(reader.read(phase).is_finite());
                    }
                    assert!(reader.read(-f64::EPSILON * 0.01).is_finite());
                }
            }
        }
        assert!(worst < 1e-6, "prepared lookup changed by {worst}");
    }
}
