//! Fixed-point products used by original DSP algorithms, without machine state.

#[inline]
pub fn saturate(value: i64) -> i32 {
    value.clamp(i32::MIN as i64, i32::MAX as i64) as i32
}

#[inline]
pub fn high_product(left: i16, right: i16) -> i64 {
    if left == i16::MIN && right == i16::MIN {
        i32::MAX as i64
    } else {
        left as i64 * right as i64 * 2
    }
}

#[inline]
pub fn multiply_q15(sample: i32, gain: i16) -> i64 {
    let low = (sample as u32 & 65535) as i64 * gain as i64 * 2;
    high_product((sample >> 16) as i16, gain) + (low >> 16)
}
