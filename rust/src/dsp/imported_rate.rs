//! Streaming conversion for the original 48 kHz DSP. Based on KekLocator / Dayverb.
use crate::{Stereo, imported_fx::Core};
use std::{
    collections::BTreeMap,
    f64::consts::PI,
    sync::{Arc, Mutex, OnceLock, Weak},
};
const TAPS: usize = 96;
const HALF: usize = TAPS / 2;
const PHASES: usize = 256;
const HISTORY: usize = 256;
fn padding(rate: f64) -> usize {
    if rate == 48000. {
        0
    } else {
        ((rate / 48000.).ceil() as usize + 2).max(4)
    }
}
struct Resampler {
    history: [[f32; 2]; HISTORY],
    count: i64,
    next: f64,
    step: f64,
    coefficients: std::sync::Arc<[[f32; TAPS]]>,
}
impl Resampler {
    fn coefficients(input: f64, output: f64) -> std::sync::Arc<[[f32; TAPS]]> {
        let cutoff = (output / input).min(1.) * 0.94;
        let mut coefficients = vec![[0.; TAPS]; PHASES + 1];
        for (phase, row) in coefficients.iter_mut().enumerate() {
            let fraction = phase as f64 / PHASES as f64;
            let mut sum = 0.;
            for (tap, c) in row.iter_mut().enumerate() {
                let x = tap as f64 - (HALF - 1) as f64 - fraction;
                let w = if x.abs() <= HALF as f64 {
                    0.35875
                        + 0.48829 * (PI * x / HALF as f64).cos()
                        + 0.14128 * (2. * PI * x / HALF as f64).cos()
                        + 0.01168 * (3. * PI * x / HALF as f64).cos()
                } else {
                    0.
                };
                let sinc = if x.abs() < 1e-12 {
                    cutoff
                } else {
                    (PI * x * cutoff).sin() / (PI * x)
                };
                *c = (w * sinc) as f32;
                sum += *c as f64;
            }
            for c in row {
                *c = (*c as f64 / sum) as f32;
            }
        }
        coefficients.into()
    }
    fn new(step: f64, coefficients: std::sync::Arc<[[f32; TAPS]]>) -> Self {
        Self {
            history: [[0.0; 2]; HISTORY],
            count: 0,
            next: -(HALF as f64),
            step,
            coefficients,
        }
    }
    fn clear(&mut self) {
        self.history.fill([0.0; 2]);
        self.count = 0;
        self.next = -(HALF as f64);
    }
    fn push(&mut self, s: [f32; 2]) {
        self.history[self.count as usize % HISTORY] = s;
        self.count += 1;
    }
    fn pop(&mut self) -> Option<[f32; 2]> {
        if self.next + HALF as f64 > (self.count - 1) as f64 + 1e-9 {
            return None;
        }
        let center = self.next.floor() as i64;
        let phase = (self.next - center as f64) * PHASES as f64;
        let p = (phase.floor() as usize).min(PHASES - 1);
        let f = phase - p as f64;
        let mut sum = [0.; 2];
        for tap in 0..TAPS {
            let i = center + tap as i64 - (HALF - 1) as i64;
            if i < 0 || i >= self.count || self.count - i > HISTORY as i64 {
                continue;
            }
            let c = self.coefficients[p][tap] as f64
                + (self.coefficients[p + 1][tap] as f64 - self.coefficients[p][tap] as f64) * f;
            let s = self.history[i as usize % HISTORY];
            sum[0] += s[0] as f64 * c;
            sum[1] += s[1] as f64 * c;
        }
        self.next += self.step;
        Some([sum[0] as f32, sum[1] as f32])
    }
}

struct Kernels {
    input: Arc<[[f32; TAPS]]>,
    output: Arc<[[f32; TAPS]]>,
}
fn kernels(rate: u32) -> Arc<Kernels> {
    static CACHE: OnceLock<Mutex<BTreeMap<u32, Weak<Kernels>>>> = OnceLock::new();
    let mut cache = CACHE.get_or_init(Default::default).lock().unwrap();
    if let Some(existing) = cache.get(&rate).and_then(Weak::upgrade) {
        return existing;
    }
    let result = Arc::new(Kernels {
        input: Resampler::coefficients(rate as f64, 48000.0),
        output: Resampler::coefficients(48000.0, rate as f64),
    });
    cache.insert(rate, Arc::downgrade(&result));
    result
}
pub struct Conversion {
    input: Resampler,
    output: Resampler,
    queue: [Stereo; 64],
    read: usize,
    len: usize,
    rate: u32,
    // Retain the cache owner. No mutex or reference count updates in process().
    _kernels: Arc<Kernels>,
}
impl Conversion {
    pub fn new(rate: u32) -> Self {
        let kernels = kernels(rate);
        Self {
            input: Resampler::new(rate as f64 / 48000.0, kernels.input.clone()),
            output: Resampler::new(48000.0 / rate as f64, kernels.output.clone()),
            queue: [[0.0; 2]; 64],
            read: 0,
            len: padding(rate as f64),
            rate,
            _kernels: kernels,
        }
    }
    pub fn clear(&mut self) {
        self.input.clear();
        self.output.clear();
        self.queue.fill([0.0; 2]);
        self.read = 0;
        self.len = padding(self.rate as f64);
    }
    pub fn process(&mut self, core: &mut Core, frame: Stereo) -> Stereo {
        self.input.push(frame);
        while let Some(input) = self.input.pop() {
            self.output.push(core.process(input));
            while let Some(output) = self.output.pop() {
                debug_assert!(self.len < self.queue.len());
                if self.len < self.queue.len() {
                    self.queue[(self.read + self.len) % 64] = output;
                    self.len += 1;
                }
            }
        }
        debug_assert!(self.len > 0);
        if self.len == 0 {
            return [0.0; 2];
        }
        let output = self.queue[self.read];
        self.read = (self.read + 1) % 64;
        self.len -= 1;
        output
    }
}

/// The original host rate needs neither resampling storage nor filter tables.
pub enum RateBridge {
    Direct,
    Converted(Box<Conversion>),
}
impl RateBridge {
    pub fn new(rate: u32) -> Self {
        if rate == 48000 {
            Self::Direct
        } else {
            Self::Converted(Box::new(Conversion::new(rate)))
        }
    }
    pub fn clear(&mut self) {
        if let Self::Converted(c) = self {
            c.clear();
        }
    }
    pub fn process(&mut self, core: &mut Core, frame: Stereo) -> Stereo {
        match self {
            Self::Direct => core.process(frame),
            Self::Converted(c) => c.process(core, frame),
        }
    }
}
