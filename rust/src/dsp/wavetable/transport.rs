//! Ready-bank transport for separate WASM memories. The worklet receives
//! bounded chunks into one arena reserved before live audio, with no FFT,
//! large JSON payload or growing wave buffer on the receiving side.
use super::{Bank, COMPILER_REVISION, KERNEL_REVISION, Kernel, LAYOUT_REVISION, bank_layout};
use serde::{Deserialize, Serialize};
use std::{
    cell::Cell,
    ops::Deref,
    sync::{Arc, Mutex, OnceLock},
};

pub const ARENA_BYTES: usize = 64 * 1024 * 1024;
const PAGE_SAMPLES: usize = 16_384;
const PAGES: usize = ARENA_BYTES / (PAGE_SAMPLES * 4);
pub const CHUNK_BYTES: usize = 32_768;
thread_local! { static REALTIME: Cell<bool> = const { Cell::new(false) }; }
pub(super) fn is_realtime() -> bool {
    REALTIME.with(Cell::get)
}
pub fn enter_realtime() {
    reserve_realtime_banks();
    REALTIME.with(|v| v.set(true));
}

struct Arena {
    pointer: usize,
    pages: Mutex<[bool; PAGES]>,
}
// The backing allocation is leaked for the engine process's lifetime. A live
// Segment owns disjoint pages. Only its unpublished BankStage may write them;
// after finish, Arc<Bank> gives shared immutable reads until Segment::drop.
// Allocation metadata is locked only by staging/retirement, never by lookup.
impl Arena {
    fn shared() -> &'static Self {
        static ARENA: OnceLock<Arena> = OnceLock::new();
        ARENA.get_or_init(|| {
            let samples = Box::leak(vec![0.0_f32; ARENA_BYTES / 4].into_boxed_slice());
            Arena {
                pointer: samples.as_mut_ptr() as usize,
                pages: Mutex::new([false; PAGES]),
            }
        })
    }
    fn allocate(&'static self, length: usize) -> Result<Segment, String> {
        let count = length.div_ceil(PAGE_SAMPLES);
        if count == 0 || count > PAGES {
            return Err("The wavetable bank exceeds the audio arena".into());
        }
        let mut pages = self
            .pages
            .lock()
            .map_err(|_| "Wavetable arena unavailable")?;
        let mut run = 0;
        for i in 0..PAGES {
            run = if pages[i] { 0 } else { run + 1 };
            if run == count {
                let start = i + 1 - count;
                pages[start..start + count].fill(true);
                return Ok(Segment {
                    arena: self,
                    start,
                    count,
                    length,
                });
            }
        }
        Err("Wavetable audio memory is full; the current sound was kept".into())
    }
}
pub struct Segment {
    arena: &'static Arena,
    start: usize,
    count: usize,
    length: usize,
}
impl Segment {
    fn pointer(&self) -> *mut f32 {
        // Page arithmetic stays inside the fixed allocation by allocate().
        unsafe { (self.arena.pointer as *mut f32).add(self.start * PAGE_SAMPLES) }
    }
}
impl Drop for Segment {
    fn drop(&mut self) {
        if let Ok(mut pages) = self.arena.pages.lock() {
            pages[self.start..self.start + self.count].fill(false);
        }
    }
}
pub(super) enum Storage {
    Owned(Box<[f32]>),
    Arena(Segment),
}
impl Deref for Storage {
    type Target = [f32];
    #[inline]
    fn deref(&self) -> &[f32] {
        match self {
            Self::Owned(data) => data,
            Self::Arena(segment) => unsafe {
                std::slice::from_raw_parts(segment.pointer(), segment.length)
            },
        }
    }
}

pub fn reserve_realtime_banks() {
    let _ = Arena::shared();
    let _ = Kernel::shared();
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BankHeader {
    pub layout_revision: u32,
    pub compiler_revision: u32,
    pub kernel_revision: u32,
    pub content_hash: String,
    pub frames: usize,
}
impl BankHeader {
    pub fn bytes(&self) -> Result<usize, String> {
        if self.layout_revision != LAYOUT_REVISION
            || self.compiler_revision != COMPILER_REVISION
            || self.kernel_revision != KERNEL_REVISION
            || !(1..=crate::wavetable::MAX_FRAMES).contains(&self.frames)
            || self.content_hash.len() != 64
            || !self
                .content_hash
                .bytes()
                .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
        {
            return Err("Unsupported or invalid prepared wavetable bank".into());
        }
        Ok(bank_layout(self.frames).1 * 4)
    }
}
impl Bank {
    pub fn transport_header(&self) -> BankHeader {
        BankHeader {
            layout_revision: LAYOUT_REVISION,
            compiler_revision: COMPILER_REVISION,
            kernel_revision: KERNEL_REVISION,
            content_hash: self.content_hash.clone(),
            frames: self.frames,
        }
    }
    /// Worker/control only. Receivers do not deserialize these floats as JSON.
    pub fn transport_bytes(&self) -> Vec<u8> {
        self.data
            .iter()
            .flat_map(|sample| sample.to_le_bytes())
            .collect()
    }
}
pub struct BankStage {
    header: BankHeader,
    segment: Segment,
    written: usize,
}
impl BankStage {
    pub fn begin(header: BankHeader) -> Result<Self, String> {
        let bytes = header.bytes()?;
        Ok(Self {
            header,
            segment: Arena::shared().allocate(bytes / 4)?,
            written: 0,
        })
    }
    pub fn append(&mut self, bytes: &[u8]) -> Result<bool, String> {
        if bytes.len() > CHUNK_BYTES
            || !bytes.len().is_multiple_of(4)
            || self.written + bytes.len() / 4 > self.segment.length
        {
            return Err("Invalid wavetable transfer chunk".into());
        }
        // No reader can reach a Segment until this stage has received every
        // sample and finish() has published its immutable Bank.
        for (i, bytes) in bytes.chunks_exact(4).enumerate() {
            let value = f32::from_le_bytes(bytes.try_into().unwrap());
            if !value.is_finite() || value.abs() > 64.0 {
                return Err("Invalid prepared wavetable sample".into());
            }
            unsafe {
                self.segment.pointer().add(self.written + i).write(value);
            }
        }
        self.written += bytes.len() / 4;
        Ok(self.written == self.segment.length)
    }
    pub fn finish(self) -> Result<Arc<Bank>, String> {
        if self.written != self.segment.length {
            return Err("Incomplete wavetable transfer".into());
        }
        let bank = Arc::new(Bank {
            content_hash: self.header.content_hash,
            frames: self.header.frames,
            levels: bank_layout(self.header.frames).0,
            data: Storage::Arena(self.segment),
            kernel: Kernel::shared(),
        });
        Bank::remember(&bank)?;
        Ok(bank)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wavetable::{FRAME_SIZE, assets::Asset};
    #[test]
    fn bounded_staging_is_sample_identical_and_cancel_releases_pages() {
        reserve_realtime_banks();
        let wave: Vec<_> = (0..FRAME_SIZE * 2)
            .map(|i| (i as f64 * std::f64::consts::TAU / FRAME_SIZE as f64).sin() as f32 * 0.7)
            .collect();
        let asset = Asset::from_cycles("Transfer".into(), FRAME_SIZE, &wave).unwrap();
        let original = Bank::compile(&asset).unwrap();
        let bytes = original.transport_bytes();
        let mut stage = BankStage::begin(original.transport_header()).unwrap();
        assert!(stage.append(&vec![0; CHUNK_BYTES + 4]).is_err());
        for chunk in bytes.chunks(CHUNK_BYTES) {
            stage.append(chunk).unwrap();
        }
        let ready = stage.finish().unwrap();
        assert_eq!(&*original.data, &*ready.data);
        for i in 0..1000 {
            let mut cancelled = BankStage::begin(original.transport_header()).unwrap();
            cancelled.append(&bytes[..CHUNK_BYTES]).unwrap();
            if i % 100 == 0 {
                assert_eq!(
                    ready.read(0.25, 0.5, ready.selection(440.0, 48000.0)),
                    original.read(0.25, 0.5, original.selection(440.0, 48000.0))
                );
            }
        }
        let mut invalid = original.transport_header();
        invalid.frames = 257;
        assert!(BankStage::begin(invalid).is_err());
        let mut stage = BankStage::begin(original.transport_header()).unwrap();
        assert!(stage.append(&f32::NAN.to_le_bytes()).is_err());
        assert!(stage.finish().is_err());
    }
}
