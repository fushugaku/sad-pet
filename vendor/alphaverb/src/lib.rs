//! Nightverb 1.02 statically translated to Rust. The firmware's own parameter
//! callbacks, delay network, interpolation, filters and dynamics run here.
#[path = "audio_generated/mod.rs"]
mod generated;
mod machine;
pub mod parameters;
mod userops;
use machine::Machine;

pub const SAMPLE_RATE: u32 = 48_000;
pub const MAX_FRAMES: usize = 512;
const INPUT: u32 = 0x20018000;
const OUTPUT: u32 = 0x20019000;

pub struct Nightverb {
    cpu: Machine,
    values: [f32; 22],
    io: Box<[f32; MAX_FRAMES * 2]>,
    bypass: bool,
}

impl Nightverb {
    pub fn new() -> Result<Self, u32> {
        let mut cpu = Machine::new();
        cpu.invoke(0x6002ab80, &[])?;
        let mut values = [0f32; 22];
        for (i, p) in parameters::PARAMETERS.iter().enumerate() {
            values[i] = p.default_normalized;
        }
        Ok(Self {
            cpu,
            values,
            io: Box::new([0.; MAX_FRAMES * 2]),
            bypass: false,
        })
    }
    pub fn set_parameter(&mut self, id: usize, normalized: f32) -> Result<(), u32> {
        let Some(p) = parameters::PARAMETERS.get(id) else {
            return Err(0xe0000001);
        };
        if !normalized.is_finite() {
            return Err(0xe0000002);
        }
        let v = normalized.clamp(0., 1.);
        // 0x6003005c is VSUB + VFMA. Preserve its single-rounding map.
        let actual = (p.max - p.min).mul_add(v, p.min);
        self.cpu.put_float(0x20000080 + id as u32 * 0x48 + 0x20, v);
        self.cpu.put_float(0x20000080 + id as u32 * 0x48 + 0x28, v);
        self.cpu.set_s0(actual);
        self.cpu.invoke(p.callback, &[0x200006b0])?;
        self.values[id] = v;
        Ok(())
    }
    pub fn value(&self, id: usize) -> Option<f32> {
        self.values.get(id).copied()
    }
    pub fn set_bypass(&mut self, bypass: bool) {
        self.bypass = bypass;
    }
    pub fn clear(&mut self) -> Result<(), u32> {
        self.cpu.reset();
        self.cpu.invoke(0x6002ab80, &[])?;
        for id in 0..22 {
            self.set_parameter(id, self.values[id])?;
        }
        Ok(())
    }
    pub fn process(&mut self, interleaved: &mut [f32]) -> Result<(), u32> {
        if interleaved.len() % 2 != 0 || interleaved.len() > MAX_FRAMES * 2 {
            return Err(0xe0000003);
        }
        self.io[..interleaved.len()].copy_from_slice(interleaved);
        self.process_io(interleaved.len() / 2)?;
        interleaved.copy_from_slice(&self.io[..interleaved.len()]);
        Ok(())
    }
    fn process_io(&mut self, frames: usize) -> Result<(), u32> {
        if frames > MAX_FRAMES {
            return Err(0xe0000003);
        }
        if frames == 0 {
            return Ok(());
        }
        for i in 0..frames * 2 {
            let value = if self.io[i].is_finite() {
                self.io[i].clamp(-1., 0.99999994)
            } else {
                0.
            };
            let q31 = (value as f64 * 2147483648.) as i32;
            self.cpu.store(INPUT + i as u32 * 4, 4, q31 as u32 as u64);
        }
        self.cpu.invoke(0x200, &[INPUT, OUTPUT, frames as u32])?;
        if !self.bypass {
            // The desktop unit's dry/wet crossfade is an external analogue VCA.
            // Replace that hardware stage with a linear host crossfade.
            let wet = self.values[20];
            for i in 0..frames * 2 {
                let processed =
                    (self.cpu.load(OUTPUT + i as u32 * 4, 4) as u32 as i32) as f32 / 2147483648.;
                self.io[i] = self.io[i] * (1. - wet) + processed * wet;
            }
        }
        Ok(())
    }
    /// Raw Q31 firmware processing, before the host's analogue-mixer replacement.
    pub fn process_q31(&mut self, input: &[i32], output: &mut [i32]) -> Result<(), u32> {
        if input.len() % 2 != 0 || input.len() > MAX_FRAMES * 2 || input.len() != output.len() {
            return Err(0xe0000003);
        }
        for (i, value) in input.iter().enumerate() {
            self.cpu
                .store(INPUT + i as u32 * 4, 4, *value as u32 as u64);
        }
        self.cpu
            .invoke(0x200, &[INPUT, OUTPUT, (input.len() / 2) as u32])?;
        for (i, value) in output.iter_mut().enumerate() {
            *value = self.cpu.load(OUTPUT + i as u32 * 4, 4) as u32 as i32;
        }
        Ok(())
    }
    pub fn memory_used(&self) -> usize {
        self.cpu.heap_used()
    }
    pub fn state_bytes(&self) -> &[u8] {
        self.cpu.ram()
    }
    pub fn state_float(&self, address: u32) -> f32 {
        self.cpu.float(address)
    }
}
