//! Bounded capture of the shared input/output. The sole audio Writer never
//! allocates, locks, grows or retires a buffer. A control reader can copy only
//! after the writer publishes Ready; arming and reading are mutually exclusive.
use crate::Stereo;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde_json::{Value, json};
use std::{
    cell::UnsafeCell,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU8, AtomicU32, AtomicUsize, Ordering},
    },
};
const IDLE: u8 = 0;
const ARMED: u8 = 1;
const WRITING: u8 = 2;
const READY: u8 = 3;
const READING: u8 = 4;
pub struct Shared {
    buffer: UnsafeCell<Box<[Stereo]>>,
    capacity: usize,
    phase: AtomicU8,
    stop: AtomicBool,
    input: AtomicBool,
    frames: AtomicUsize,
    limit: AtomicUsize,
    rate: AtomicU32,
    generation: AtomicU32,
}
// Exactly one non-Clone Writer exists. It writes only in WRITING; snapshot
// claims READY→READING with acquire, and arming cannot replace a READING take.
unsafe impl Sync for Shared {}
pub struct Writer {
    shared: Arc<Shared>,
    position: usize,
}
impl Shared {
    pub fn prepare(rate: u32) -> (Arc<Self>, Writer) {
        let rate = rate.clamp(8000, 384000);
        let capacity = rate as usize * 2;
        let shared = Arc::new(Self {
            buffer: UnsafeCell::new(vec![[0.0; 2]; capacity].into_boxed_slice()),
            capacity,
            phase: AtomicU8::new(IDLE),
            stop: AtomicBool::new(false),
            input: AtomicBool::new(false),
            frames: AtomicUsize::new(0),
            limit: AtomicUsize::new(0),
            rate: AtomicU32::new(rate),
            generation: AtomicU32::new(0),
        });
        (
            shared.clone(),
            Writer {
                shared,
                position: 0,
            },
        )
    }
    pub fn arm(&self, rate: u32, seconds: f64, input: bool) -> Result<(), String> {
        if !(8000..=384000).contains(&rate)
            || !seconds.is_finite()
            || !(0.05..=2.0).contains(&seconds)
        {
            return Err("Capture lasts 0.05–2 seconds".into());
        }
        if (rate as f64 * seconds).round() as usize > self.capacity {
            return Err("Capture buffer was prepared for a different sample rate".into());
        }
        let phase = self.phase.load(Ordering::Acquire);
        if ![IDLE, READY].contains(&phase)
            || self
                .phase
                .compare_exchange(phase, READING, Ordering::AcqRel, Ordering::Acquire)
                .is_err()
        {
            return Err("Capture is already running or being read".into());
        }
        self.generation.fetch_add(1, Ordering::Relaxed);
        self.frames.store(0, Ordering::Relaxed);
        self.limit
            .store((rate as f64 * seconds).round() as usize, Ordering::Relaxed);
        self.rate.store(rate, Ordering::Relaxed);
        self.input.store(input, Ordering::Relaxed);
        self.stop.store(false, Ordering::Relaxed);
        self.phase.store(ARMED, Ordering::Release);
        Ok(())
    }
    pub fn stop(&self) {
        self.stop.store(true, Ordering::Release);
    }
    pub fn view(&self) -> Value {
        let phase = self.phase.load(Ordering::Acquire);
        let rate = self.rate.load(Ordering::Relaxed);
        let frames = self.frames.load(Ordering::Relaxed);
        json!({"active":([ARMED,WRITING].contains(&phase)),"ready":phase==READY,"frames":frames,"seconds":frames as f64/rate as f64,"sample_rate":rate,"source":if self.input.load(Ordering::Relaxed){"input"}else{"output"},"max_seconds":2,"generation":self.generation.load(Ordering::Relaxed)})
    }
    /// Bounded, allocation-free read for the worklet's control messages.
    pub fn read_chunk(
        &self,
        generation: u32,
        offset: usize,
        out: &mut [u8],
    ) -> Result<usize, &'static str> {
        self.phase
            .compare_exchange(READY, READING, Ordering::AcqRel, Ordering::Acquire)
            .map_err(|_| "Capture is not ready")?;
        let size = self.frames.load(Ordering::Relaxed) * 8;
        if generation != self.generation.load(Ordering::Relaxed)
            || offset > size
            || !offset.is_multiple_of(8)
            || !out.len().is_multiple_of(8)
        {
            self.phase.store(READY, Ordering::Release);
            return Err("Capture was superseded or offset is invalid");
        }
        let bytes = (size - offset).min(out.len());
        let frames = unsafe { &(&*self.buffer.get())[offset / 8..(offset + bytes) / 8] };
        for (to, &sample) in out[..bytes]
            .chunks_exact_mut(4)
            .zip(frames.iter().flatten())
        {
            to.copy_from_slice(&sample.to_le_bytes());
        }
        self.phase.store(READY, Ordering::Release);
        Ok(bytes)
    }
    pub fn snapshot(&self) -> Result<(u32, Vec<Stereo>), String> {
        self.phase
            .compare_exchange(READY, READING, Ordering::AcqRel, Ordering::Acquire)
            .map_err(|_| "Capture is not ready")?;
        let frames = self.frames.load(Ordering::Relaxed);
        let rate = self.rate.load(Ordering::Relaxed);
        // The unique writer has stopped before its release-store of READY.
        let copy = unsafe { (&*self.buffer.get())[..frames].to_vec() };
        self.phase.store(READY, Ordering::Release);
        if copy.is_empty() {
            return Err("No audio was captured".into());
        }
        Ok((rate, copy))
    }
    pub fn command(
        &self,
        path: &str,
        data: &Value,
        rate: u32,
        input_available: bool,
    ) -> Result<Value, String> {
        match path {
            "/api/capture/start" => {
                let input = data["source"] == "input";
                if input && !input_available {
                    return Err("Connect an audio input before capturing it".into());
                }
                self.arm(rate, data["seconds"].as_f64().unwrap_or(2.0), input)?;
                Ok(self.view())
            }
            "/api/capture/stop" => {
                self.stop();
                Ok(self.view())
            }
            "/api/capture/status" => Ok(self.view()),
            "/api/capture/wav" => {
                let (rate, frames) = self.snapshot()?;
                Ok(
                    json!({"wav":STANDARD.encode(crate::wav::bytes(rate,&frames)?),"sample_rate":rate,"frames":frames.len()}),
                )
            }
            _ => Err("Unknown capture action".into()),
        }
    }
}
impl Writer {
    pub fn shared(&self) -> &Shared {
        &self.shared
    }
    #[inline]
    pub fn push(&mut self, input: Stereo, output: Stereo) {
        let mut phase = self.shared.phase.load(Ordering::Acquire);
        if phase == ARMED {
            self.position = 0;
            self.shared.phase.store(WRITING, Ordering::Release);
            phase = WRITING;
        }
        if phase != WRITING {
            return;
        }
        if self.shared.stop.load(Ordering::Acquire) {
            self.shared.phase.store(READY, Ordering::Release);
            return;
        }
        let frame = if self.shared.input.load(Ordering::Relaxed) {
            input
        } else {
            output
        };
        // Exclusive Writer; control can access the buffer only after Ready.
        unsafe {
            (&mut *self.shared.buffer.get())[self.position] =
                frame.map(|v| if v.is_finite() { v } else { 0.0 });
        }
        self.position += 1;
        self.shared.frames.store(self.position, Ordering::Relaxed);
        if self.position >= self.shared.limit.load(Ordering::Relaxed) {
            self.shared.phase.store(READY, Ordering::Release);
        }
    }
}
