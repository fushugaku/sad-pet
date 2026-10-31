//! Read ready audio through the existing 32 KiB scratch, off process().
use super::*;
#[unsafe(no_mangle)]
pub extern "C" fn sad_capture_chunk(generation: u32, offset: u32) -> i32 {
    registry(|r| {
        r.capture
            .shared()
            .read_chunk(generation, offset as usize, &mut r.wavetable_upload[..])
            .map_or(-1, |n| n as i32)
    })
}
