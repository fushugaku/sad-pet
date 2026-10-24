//! Render workers: the timbres of one chunk render on several cores.
//!
//! The audio thread publishes a list of jobs (one per timbre or bus), renders
//! its own part, helps with the remaining jobs and then waits for the rest.
//! Every job touches only its own part, and the mix is summed afterwards on
//! the audio thread in list order, so the output is the same bit for bit as
//! rendering the parts one after another.
//!
//! Nothing here allocates, locks or frees memory while audio runs. Workers
//! spin briefly after a chunk, so the next chunk of the same host block finds
//! them awake, and park otherwise. Each job runs with the audio thread's
//! floating-point control register, so a host's flush-to-zero setting gives
//! the same results on every core.
use super::{Audio, Stereo};
use std::{
    cell::UnsafeCell,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering},
    },
    thread,
    time::{Duration, Instant},
};

/// Jobs one chunk can publish; further parts render on the audio thread.
pub(super) const MAX_JOBS: usize = 64;
const MAX_WORKERS: usize = 7;
/// How long an idle worker spins before it parks.
const SPIN: Duration = Duration::from_micros(60);

#[derive(Clone, Copy)]
pub(super) enum Job {
    /// A timbre renders its chunk from `input` and writes its sends.
    Timbre {
        audio: *mut Audio,
        input: *const Stereo,
        n: usize,
        buses: bool,
    },
    /// A bus or Master renders the input already in its chunk buffer.
    Routing { audio: *mut Audio, n: usize },
}
impl Job {
    /// # Safety
    /// The audio pointer is live and no other thread touches that part until
    /// the job is done; `input` addresses `n` frames for that time.
    pub(super) unsafe fn run(self) {
        match self {
            Job::Timbre {
                audio,
                input,
                n,
                buses,
            } => unsafe {
                (*audio).source_chunk(std::slice::from_raw_parts(input, n), buses);
            },
            Job::Routing { audio, n } => unsafe { (*audio).source_routing_chunk(n) },
        }
    }
}

struct Shared {
    jobs: UnsafeCell<[Option<Job>; MAX_JOBS]>,
    /// epoch << 32 | count << 16 | next job. One word, so a worker that
    /// wakes late can never claim a job of a later epoch with an old count.
    claim: AtomicU64,
    done: AtomicU32,
    /// The audio thread's FPCR (aarch64) or MXCSR (x86_64) for this epoch.
    fp: AtomicU64,
    /// Bit n while worker n is parked.
    parked: AtomicU32,
    quit: AtomicBool,
}
// SAFETY: `jobs` is written only by the audio thread while no epoch is open
// (every claimed job of the previous one is done) and read by workers only
// for a job they claimed in the open epoch, after the Release store of
// `claim` that published it. The parts behind the pointers are owned by the
// audio thread, which hands each to exactly one claimant per epoch.
unsafe impl Sync for Shared {}
unsafe impl Send for Shared {}

const fn pack(epoch: u32, count: usize, next: usize) -> u64 {
    (epoch as u64) << 32 | (count as u64) << 16 | next as u64
}
const fn unpack(value: u64) -> (u32, usize, usize) {
    (
        (value >> 32) as u32,
        ((value >> 16) & 0xffff) as usize,
        (value & 0xffff) as usize,
    )
}

impl Shared {
    /// The next unclaimed job of `epoch`, if any.
    fn claim(&self, epoch: u32) -> Option<usize> {
        let mut value = self.claim.load(Ordering::Acquire);
        loop {
            let (e, count, next) = unpack(value);
            if e != epoch || next >= count {
                return None;
            }
            match self.claim.compare_exchange_weak(
                value,
                pack(e, count, next + 1),
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return Some(next),
                Err(current) => value = current,
            }
        }
    }
    /// Runs claimed jobs of `epoch` until none is left.
    fn work(&self, epoch: u32) {
        while let Some(index) = self.claim(epoch) {
            // SAFETY: the claim above gives this thread job `index` alone.
            let job = unsafe { (*self.jobs.get())[index] };
            if let Some(job) = job {
                // A panic would leave the audio thread waiting for this job;
                // abort, as a panic across the plug-in's C ABI does.
                let result =
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { job.run() }));
                if result.is_err() {
                    std::process::abort();
                }
            }
            self.done.fetch_add(1, Ordering::Release);
        }
    }
}

pub(super) struct Pool {
    shared: Arc<Shared>,
    threads: Vec<thread::JoinHandle<()>>,
    epoch: u32,
}
impl Pool {
    /// Starts the workers on a control thread; `None` on one core or when
    /// no thread can start.
    pub(super) fn new() -> Option<Self> {
        let cores = thread::available_parallelism().map_or(1, |n| n.get());
        let workers = cores.saturating_sub(1).min(MAX_WORKERS);
        if workers == 0 {
            return None;
        }
        let shared = Arc::new(Shared {
            jobs: UnsafeCell::new([None; MAX_JOBS]),
            claim: AtomicU64::new(pack(0, 0, 0)),
            done: AtomicU32::new(0),
            fp: AtomicU64::new(0),
            parked: AtomicU32::new(0),
            quit: AtomicBool::new(false),
        });
        let mut threads = Vec::with_capacity(workers);
        for index in 0..workers {
            let shared = shared.clone();
            let thread = thread::Builder::new()
                .name(format!("SAD PET render {}", index + 1))
                .spawn(move || worker(&shared, index));
            match thread {
                Ok(thread) => threads.push(thread),
                Err(_) => break,
            }
        }
        if threads.is_empty() {
            return None;
        }
        Some(Self {
            shared,
            threads,
            epoch: 0,
        })
    }
    /// Publishes `jobs`, runs `own` on this thread, helps with the jobs and
    /// returns once every job is done.
    pub(super) fn run(&mut self, jobs: &[Option<Job>], own: impl FnOnce()) {
        let count = jobs.len().min(MAX_JOBS);
        let shared = &*self.shared;
        // SAFETY: no epoch is open, so no worker reads the jobs now.
        unsafe {
            let slots = &mut *shared.jobs.get();
            slots[..count].copy_from_slice(&jobs[..count]);
        }
        shared.done.store(0, Ordering::Relaxed);
        shared.fp.store(read_fp(), Ordering::Relaxed);
        self.epoch = self.epoch.wrapping_add(1);
        let epoch = self.epoch;
        // SeqCst with the workers' parked flags: either a worker sees this
        // epoch before it parks, or this thread sees it parked and wakes it.
        shared.claim.store(pack(epoch, count, 0), Ordering::SeqCst);
        let mut parked = shared.parked.load(Ordering::SeqCst);
        let mut wake = count;
        while parked != 0 && wake > 0 {
            let index = parked.trailing_zeros() as usize;
            parked &= parked - 1;
            if let Some(thread) = self.threads.get(index) {
                thread.thread().unpark();
            }
            wake -= 1;
        }
        own();
        shared.work(epoch);
        let mut spins = 0_u32;
        while (shared.done.load(Ordering::Acquire) as usize) < count {
            spins = spins.wrapping_add(1);
            if spins < 1 << 14 {
                std::hint::spin_loop();
            } else {
                thread::yield_now();
            }
        }
        // Close the epoch: a late worker finds no job left to claim.
        shared
            .claim
            .store(pack(epoch, count, count), Ordering::Release);
    }
}
impl Drop for Pool {
    fn drop(&mut self) {
        self.shared.quit.store(true, Ordering::SeqCst);
        for thread in &self.threads {
            thread.thread().unpark();
        }
        for thread in self.threads.drain(..) {
            let _ = thread.join();
        }
    }
}

fn worker(shared: &Shared, index: usize) {
    promote();
    let bit = 1_u32 << index;
    let mut seen = 0_u32;
    loop {
        let started = Instant::now();
        let mut spins = 0_u32;
        let epoch = loop {
            if shared.quit.load(Ordering::Acquire) {
                return;
            }
            let (epoch, _, _) = unpack(shared.claim.load(Ordering::Acquire));
            if epoch != seen {
                break epoch;
            }
            spins = spins.wrapping_add(1);
            if !spins.is_multiple_of(64) || started.elapsed() < SPIN {
                std::hint::spin_loop();
                continue;
            }
            shared.parked.fetch_or(bit, Ordering::SeqCst);
            let (epoch, _, _) = unpack(shared.claim.load(Ordering::SeqCst));
            if epoch == seen && !shared.quit.load(Ordering::SeqCst) {
                thread::park();
            }
            shared.parked.fetch_and(!bit, Ordering::SeqCst);
        };
        seen = epoch;
        write_fp(shared.fp.load(Ordering::Relaxed));
        shared.work(epoch);
    }
}

#[cfg(target_arch = "aarch64")]
fn read_fp() -> u64 {
    let value: u64;
    // SAFETY: reads this thread's floating-point control register.
    unsafe {
        core::arch::asm!("mrs {}, fpcr", out(reg) value, options(nomem, nostack, preserves_flags))
    };
    value
}
#[cfg(target_arch = "aarch64")]
fn write_fp(value: u64) {
    // SAFETY: rounding, flush-to-zero and default-NaN modes of this thread
    // only; the value came from another thread's FPCR.
    unsafe {
        core::arch::asm!("msr fpcr, {}", in(reg) value, options(nomem, nostack, preserves_flags))
    };
}
#[cfg(target_arch = "x86_64")]
fn read_fp() -> u64 {
    let mut value = 0_u32;
    // SAFETY: stores this thread's MXCSR into a local.
    unsafe {
        core::arch::asm!("stmxcsr [{}]", in(reg) &mut value, options(nostack, preserves_flags))
    };
    value as u64
}
#[cfg(target_arch = "x86_64")]
fn write_fp(value: u64) {
    let value = value as u32;
    // SAFETY: loads a valid MXCSR, read from another thread, into this one.
    unsafe { core::arch::asm!("ldmxcsr [{}]", in(reg) &value, options(nostack, preserves_flags)) };
}
#[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
fn read_fp() -> u64 {
    0
}
#[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
fn write_fp(_: u64) {}

/// Real-time scheduling, as the host gives its audio thread. A worker that
/// is descheduled while it holds a job would make the audio thread wait.
#[cfg(target_os = "macos")]
fn promote() {
    #[repr(C)]
    struct Timebase {
        numer: u32,
        denom: u32,
    }
    #[repr(C)]
    struct TimeConstraint {
        period: u32,
        computation: u32,
        constraint: u32,
        preemptible: u32,
    }
    const THREAD_TIME_CONSTRAINT_POLICY: u32 = 2;
    unsafe extern "C" {
        fn mach_timebase_info(info: *mut Timebase) -> i32;
        fn pthread_self() -> usize;
        fn pthread_mach_thread_np(thread: usize) -> u32;
        fn thread_policy_set(
            thread: u32,
            flavor: u32,
            policy: *const TimeConstraint,
            count: u32,
        ) -> i32;
    }
    let mut timebase = Timebase { numer: 0, denom: 0 };
    // SAFETY: fills the struct it is given.
    if unsafe { mach_timebase_info(&mut timebase) } != 0 || timebase.numer == 0 {
        return;
    }
    let ticks = |seconds: f64| {
        (seconds * 1e9 * timebase.denom as f64 / timebase.numer as f64).round() as u32
    };
    // A 256-frame block at 48 kHz: the thread asks for half of each period.
    let period = 256.0 / 48_000.0;
    let policy = TimeConstraint {
        period: ticks(period),
        computation: ticks(period * 0.5),
        constraint: ticks(period),
        preemptible: 1,
    };
    // SAFETY: this thread's own port and a policy of the flavor's size.
    unsafe {
        thread_policy_set(
            pthread_mach_thread_np(pthread_self()),
            THREAD_TIME_CONSTRAINT_POLICY,
            &policy,
            (size_of::<TimeConstraint>() / size_of::<u32>()) as u32,
        );
    }
}
#[cfg(windows)]
fn promote() {
    const THREAD_PRIORITY_TIME_CRITICAL: i32 = 15;
    unsafe extern "system" {
        fn GetCurrentThread() -> isize;
        fn SetThreadPriority(thread: isize, priority: i32) -> i32;
    }
    // SAFETY: the pseudo-handle of the calling thread.
    unsafe {
        SetThreadPriority(GetCurrentThread(), THREAD_PRIORITY_TIME_CRITICAL);
    }
}
#[cfg(not(any(target_os = "macos", windows)))]
fn promote() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn claims_never_cross_epochs() {
        let value = pack(7, 3, 1);
        assert_eq!(unpack(value), (7, 3, 1));
        let shared = Shared {
            jobs: UnsafeCell::new([None; MAX_JOBS]),
            claim: AtomicU64::new(pack(7, 3, 1)),
            done: AtomicU32::new(0),
            fp: AtomicU64::new(0),
            parked: AtomicU32::new(0),
            quit: AtomicBool::new(false),
        };
        assert_eq!(shared.claim(6), None);
        assert_eq!(shared.claim(7), Some(1));
        assert_eq!(shared.claim(7), Some(2));
        assert_eq!(shared.claim(7), None);
    }
    #[test]
    fn workers_copy_the_audio_threads_float_mode() {
        let value = read_fp();
        write_fp(value);
        assert_eq!(read_fp(), value);
    }
}
