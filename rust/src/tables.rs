//! Original little-endian f32 tables extracted from the supplied 1.63 image.
//! Addresses and hashes are recorded in assets/manifest.json.
// Decoded once at compile time, bit for bit, instead of on every read.
static PITCH: [f32; 256] = decode(include_bytes!("../assets/pitch.f32"));
static FINE_PITCH: [f32; 257] = decode(include_bytes!("../assets/fine_pitch.f32"));
static WINDOW: [f32; 4097] = decode(include_bytes!("../assets/grain_window.f32"));
static SINE: [f32; 256] = decode(include_bytes!("../assets/sine.f32"));

const fn decode<const N: usize>(bytes: &[u8]) -> [f32; N] {
    assert!(bytes.len() == N * 4, "table length");
    let mut table = [0.0; N];
    let mut i = 0;
    while i < N {
        let b = [
            bytes[i * 4],
            bytes[i * 4 + 1],
            bytes[i * 4 + 2],
            bytes[i * 4 + 3],
        ];
        table[i] = f32::from_le_bytes(b);
        i += 1;
    }
    table
}

fn lookup<const N: usize>(table: &[f32; N], position: f32) -> f32 {
    let end = N - 1;
    let p = position.clamp(0.0, end as f32);
    let i = p as usize;
    let a = table[i];
    if i == end {
        a
    } else {
        a + (table[i + 1] - a) * (p - i as f32)
    }
}

/// Coarse semitone LUT times an 8-bit fractional-semitone LUT, as at c003f610.
pub fn pitch_ratio(semitones: f32) -> f64 {
    let semitones = if semitones.is_finite() {
        semitones.clamp(-64.0, 64.0)
    } else {
        0.0
    };
    let whole = semitones.floor();
    let fraction = ((semitones - whole) * 256.0) as usize;
    (PITCH[(whole as i32 + 128) as usize] * FINE_PITCH[fraction.min(255)]) as f64
}

/// c003840c: a mirrored ramp mixed with the original half-cosine window LUT.
pub fn grain_window(phase: f32, shape: f32) -> f32 {
    if !(0.0..=2.0).contains(&phase) {
        return 0.0;
    }
    let triangle = if phase <= 1.0 { phase } else { 2.0 - phase };
    triangle + shape.clamp(0.0, 1.0) * (lookup(&WINDOW, triangle * 4096.0) - triangle)
}

pub fn sine(phase: f64) -> f32 {
    lookup(
        &SINE,
        crate::dsp::sample::wrap_position(phase, 1.0) as f32 * 255.0,
    )
}

/// The normalization approximation in c003ea3c (one Newton step).
pub fn inverse_sqrt(x: f32) -> f32 {
    let mut y = f32::from_bits(0x5f37_59df_u32.wrapping_sub(x.to_bits() >> 1));
    y *= 1.5 - 0.5 * x * y * y;
    y
}

/// Bipolar size/direction curve in c003ea3c, converted from 48 kHz samples to seconds.
pub fn grain_duration(size: f32) -> f32 {
    0.010 + 2.0 * (2.0 * size.clamp(0.0, 1.0) - 1.0).abs().powi(3)
}

pub fn detune_range(knob: f32) -> f32 {
    let x = knob.clamp(0.0, 1.0) * 2.0 - 1.0;
    if x > 0.05 {
        ((x - 0.05) / 0.95).powi(3) * 12.0
    } else if x < -0.05 {
        ((x + 0.05) / 0.95).powi(3) * 12.0
    } else {
        0.0
    }
}

/// Linear stereo pan gains from the grain initializer in c003ea3c.
pub fn pan(p: f32) -> [f32; 2] {
    let p = p.clamp(0.0, 1.0);
    [
        if p < 0.5 { 1.0 } else { 2.0 * (1.0 - p) },
        if p < 0.5 { 2.0 * p } else { 1.0 },
    ]
}

/// State transition and XSH-RR output recovered from c003ea3c.
#[derive(Clone, Debug)]
pub struct Random(pub u64);
impl Default for Random {
    fn default() -> Self {
        Self(0x853c_49e6_748f_ea9b)
    }
}
impl Random {
    pub fn next_u32(&mut self) -> u32 {
        self.0 = self
            .0
            .wrapping_mul(0x5851_f42d_4c95_7f2d)
            .wrapping_add(0xda3e_39cb_94b9_5bdb);
        let shifted = (((self.0 >> 18) ^ self.0) >> 27) as u32;
        shifted.rotate_right((self.0 >> 59) as u32)
    }
    pub fn unit(&mut self) -> f32 {
        f32::from_bits(0x3f80_0000 | (self.next_u32() >> 9)) - 1.0
    }
    pub fn bipolar(&mut self) -> f32 {
        self.unit() * 2.0 - 1.0
    }
}
