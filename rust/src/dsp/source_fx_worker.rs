//! Cancellable source preparation. The callback only exchanges fixed requests
//! and Arc handles; the worker builds FIR mips and reclaims old caches.
use crate::{
    color::ColorSlot,
    dsp::{
        Engine, Region, SampleBuffer,
        source_fx::{self, ProcessedSource},
    },
};
use rtrb::{Consumer, Producer, RingBuffer};
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    thread::{self, JoinHandle},
    time::Duration,
};

#[derive(Clone, Copy, PartialEq)]
struct Key {
    sample: usize,
    region: Region,
    slot: Option<ColorSlot>,
    bpm: f32,
    root: f32,
}
impl Key {
    fn new(e: &Engine) -> Self {
        let slot = if e.live_input.is_none() {
            e.preset
                .source_color_chain
                .first()
                .copied()
                .filter(|s| s.enabled && s.mix > 0.0)
        } else {
            None
        };
        let recipe = crate::dsp::source_fx::Recipe::new(
            e.region,
            slot,
            e.preset.params.bpm,
            e.preset.params.sample_root_note,
        );
        Self {
            sample: Arc::as_ptr(&e.sample) as usize,
            region: e.region,
            slot,
            bpm: recipe.bpm,
            root: recipe.root,
        }
    }
}
struct Job {
    id: u64,
    key: Key,
    sample: Arc<SampleBuffer>,
}
struct Completed {
    id: u64,
    key: Key,
    error: bool,
    cache: Option<Arc<ProcessedSource>>,
}

pub struct Renderer {
    requests: Producer<Job>,
    results: Consumer<Completed>,
    retired: Producer<Completed>,
    latest: Arc<AtomicU64>,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
    key: Option<Key>,
    sequence: u64,
    submitted: bool,
}
impl Renderer {
    pub fn new(e: &Engine) -> Self {
        let (requests, mut jobs) = RingBuffer::<Job>::new(4);
        let (mut publish, results) = RingBuffer::<Completed>::new(4);
        let (retired, mut garbage) = RingBuffer::<Completed>::new(8);
        let latest = Arc::new(AtomicU64::new(0));
        let stop = Arc::new(AtomicBool::new(false));
        let revision = latest.clone();
        let stopped = stop.clone();
        let thread = thread::Builder::new()
            .name("SAD PET source FX".into())
            .spawn(move || {
                let mut pending = None;
                while !stopped.load(Ordering::Acquire) {
                    while garbage.pop().is_ok() {}
                    let mut job = None;
                    while let Ok(next) = jobs.pop() {
                        job = Some(next);
                    }
                    if let Some(job) = job {
                        let result = source_fx::prepare(
                            job.sample,
                            job.key.region,
                            job.key.slot,
                            job.key.bpm,
                            job.key.root,
                            || {
                                stopped.load(Ordering::Acquire)
                                    || revision.load(Ordering::Acquire) != job.id
                            },
                        );
                        if revision.load(Ordering::Acquire) == job.id {
                            pending = Some(match result {
                                Ok(cache) => Completed {
                                    id: job.id,
                                    key: job.key,
                                    error: false,
                                    cache,
                                },
                                Err(_) => Completed {
                                    id: job.id,
                                    key: job.key,
                                    error: true,
                                    cache: None,
                                },
                            });
                        }
                    }
                    if publish.slots() > 0
                        && let Some(result) = pending.take()
                    {
                        let _ = publish.push(result);
                    }
                    thread::sleep(Duration::from_millis(4));
                }
            })
            .expect("source FX worker");
        Self {
            requests,
            results,
            retired,
            latest,
            stop,
            thread: Some(thread),
            key: if e.source_effects_pending() {
                None
            } else {
                Some(Key::new(e))
            },
            sequence: 0,
            submitted: !e.source_effects_pending(),
        }
    }
    pub fn update(&mut self, e: &mut Engine) {
        let key = Key::new(e);
        if self.key != Some(key) {
            self.sequence = self.sequence.wrapping_add(1);
            self.latest.store(self.sequence, Ordering::Release);
            self.key = Some(key);
            self.submitted = false;
            e.source_fx_error = false;
        }
        if !self.submitted && self.requests.slots() > 0 {
            if e.source_effects_pending() {
                let result = self.requests.push(Job {
                    id: self.sequence,
                    key,
                    sample: e.sample.clone(),
                });
                debug_assert!(result.is_ok());
            }
            self.submitted = true;
        }
        while self.retired.slots() > 0 {
            let Ok(mut result) = self.results.pop() else {
                break;
            };
            if result.id == self.sequence && result.key == key {
                if result.error {
                    e.source_fx_error = true;
                } else {
                    e.exchange_processed_source(&mut result.cache);
                }
            }
            let result = self.retired.push(result);
            debug_assert!(result.is_ok());
        }
    }
}
impl Drop for Renderer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Release);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_full_request_queue_still_invalidates_older_results() {
        let mut e = Engine::new(8000, Arc::new(SampleBuffer::demo(8000)), Default::default());
        let old = Key::new(&e);
        let (mut requests, _receiver) = RingBuffer::new(1);
        requests
            .push(Job {
                id: 1,
                key: old,
                sample: e.sample.clone(),
            })
            .ok()
            .unwrap();
        let (mut publish, results) = RingBuffer::new(1);
        publish
            .push(Completed {
                id: 1,
                key: old,
                error: true,
                cache: None,
            })
            .ok()
            .unwrap();
        let (retired, _garbage) = RingBuffer::new(1);
        let mut renderer = Renderer {
            requests,
            results,
            retired,
            latest: Arc::new(AtomicU64::new(1)),
            stop: Arc::new(AtomicBool::new(false)),
            thread: None,
            key: Some(old),
            sequence: 1,
            submitted: true,
        };
        e.preset.source_color_chain = vec![ColorSlot {
            mix: 1.0,
            ..Default::default()
        }];
        renderer.update(&mut e);
        assert_eq!(renderer.sequence, 2);
        assert!(!renderer.submitted);
        assert!(!e.source_fx_error);
        assert_eq!(renderer.latest.load(Ordering::Acquire), 2);
    }
}
