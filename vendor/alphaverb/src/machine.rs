//! Host memory and arithmetic for the statically translated firmware.
//! Register bytes intentionally overlap (s0/s1 are the halves of d0).
use std::cell::Cell;

pub const RETURN: u32 = 0xfffffff0;
const FLASH: &[u8] = include_bytes!("../assets/flash.bin");

pub struct Machine {
    #[cfg(test)]
    pub reference_dispatch: bool,
    regs: [u8; 4096],
    itcm: Vec<u8>,
    dtcm: Vec<u8>,
    ocram: Vec<u8>,
    heap: Vec<u8>,
    heap_cursor: usize,
    pub fault: Cell<Option<u32>>,
    irq: bool,
}

pub fn signed(v: u64, size: usize) -> i64 {
    let shift = 64 - size * 8;
    ((v << shift) as i64) >> shift
}

pub fn signed_overflow(a: u64, b: u64, size: usize, subtract: bool) -> u64 {
    let mask = 1u64 << (size * 8 - 1);
    let result = if subtract {
        a.wrapping_sub(b)
    } else {
        a.wrapping_add(b)
    };
    (((a ^ result) & if subtract { a ^ b } else { !(a ^ b) } & mask) != 0) as u64
}

impl Machine {
    pub fn new() -> Self {
        let mut m = Self {
            #[cfg(test)]
            reference_dispatch: false,
            regs: [0; 4096],
            itcm: vec![0; 65536],
            dtcm: vec![0; 0x20000],
            ocram: vec![0; 0x80000],
            heap: vec![0; 16 * 1024 * 1024],
            heap_cursor: 0,
            fault: Cell::new(None),
            irq: true,
        };
        m.restore_images();
        m
    }
    fn restore_images(&mut self) {
        self.itcm[..13440].copy_from_slice(include_bytes!("../assets/itcm.bin"));
        self.dtcm[..0x2740].copy_from_slice(include_bytes!("../assets/dtcm.bin"));
        self.dtcm[0x2740..0x2e54].copy_from_slice(include_bytes!("../assets/dtcm2.bin"));
        self.w(0, 1, 1);
    }
    /// Reuse the existing allocations, including in the AudioWorklet thread.
    pub fn reset(&mut self) {
        self.regs.fill(0);
        self.itcm.fill(0);
        self.dtcm.fill(0);
        self.ocram.fill(0);
        self.heap.fill(0);
        self.heap_cursor = 0;
        self.fault.set(None);
        self.irq = true;
        self.restore_images();
    }
    #[inline(always)]
    pub fn r(&self, off: usize, size: usize) -> u64 {
        let mut bytes = [0u8; 8];
        bytes[..size].copy_from_slice(&self.regs[off..off + size]);
        u64::from_le_bytes(bytes)
    }
    #[inline(always)]
    pub fn w(&mut self, off: usize, size: usize, value: u64) {
        self.regs[off..off + size].copy_from_slice(&value.to_le_bytes()[..size]);
    }
    pub fn fail(&self, address: u32) {
        if self.fault.get().is_none() {
            self.fault.set(Some(address));
        }
    }
    #[inline(always)]
    pub fn load(&self, address: u32, size: usize) -> u64 {
        let (data, off) = match address {
            0..=0xffff => (&self.itcm[..], address as usize),
            0x20000000..=0x2001ffff => (&self.dtcm[..], (address - 0x20000000) as usize),
            0x20200000..=0x2027ffff => (&self.ocram[..], (address - 0x20200000) as usize),
            0x60020000..=0x6005d4ff => (FLASH, (address - 0x60020000) as usize),
            0x80000000..=0x80ffffff => (&self.heap[..], (address - 0x80000000) as usize),
            _ => {
                self.fail(address);
                return 0;
            }
        };
        if off + size > data.len() {
            self.fail(address);
            return 0;
        }
        let mut bytes = [0u8; 8];
        bytes[..size].copy_from_slice(&data[off..off + size]);
        u64::from_le_bytes(bytes)
    }
    #[inline(always)]
    pub fn store(&mut self, address: u32, size: usize, value: u64) {
        let (data, off) = match address {
            0..=0xffff => (&mut self.itcm[..], address as usize),
            0x20000000..=0x2001ffff => (&mut self.dtcm[..], (address - 0x20000000) as usize),
            0x20200000..=0x2027ffff => (&mut self.ocram[..], (address - 0x20200000) as usize),
            0x80000000..=0x80ffffff => (&mut self.heap[..], (address - 0x80000000) as usize),
            _ => {
                self.fail(address);
                return;
            }
        };
        if off + size > data.len() {
            self.fail(address);
            return;
        }
        data[off..off + size].copy_from_slice(&value.to_le_bytes()[..size]);
    }
    pub fn float(&self, address: u32) -> f32 {
        f32::from_bits(self.load(address, 4) as u32)
    }
    pub fn put_float(&mut self, address: u32, value: f32) {
        self.store(address, 4, value.to_bits() as u64);
    }
    pub fn set_s0(&mut self, value: f32) {
        self.w(768, 4, value.to_bits() as u64);
    }
    pub fn invoke(&mut self, entry: u32, args: &[u32]) -> Result<(), u32> {
        self.fault.set(None);
        self.w(84, 4, 0x2001e000);
        self.w(88, 4, (RETURN | 1) as u64);
        for (n, arg) in args.iter().take(4).enumerate() {
            self.w(32 + n * 4, 4, *arg as u64);
        }
        let mut pc = entry & !1;
        for _ in 0..4_000_000 {
            if pc == RETURN {
                return self.fault.get().map_or(Ok(()), Err);
            }
            #[cfg(test)]
            let fused = !self.reference_dispatch;
            #[cfg(not(test))]
            let fused = true;
            match pc {
                0x60029898 if fused => pc = crate::fused::b_60029898(self),
                0x60029dcc if fused => pc = crate::fused::b_60029dcc(self),
                // Replace only platform allocation; DSP/math execute translated Rust.
                0x600414c0 => {
                    let size = self.r(32, 4) as usize;
                    let next = (self.heap_cursor + size + 15) & !15;
                    if next > self.heap.len() {
                        return Err(0x80000000 + self.heap_cursor as u32);
                    }
                    self.w(32, 4, 0x80000000 + self.heap_cursor as u64);
                    self.heap_cursor = next;
                    pc = self.r(88, 4) as u32 & !1;
                }
                0x600414d0 => {
                    pc = self.r(88, 4) as u32 & !1;
                }
                _ => pc = crate::generated::step(self, pc),
            }
            if let Some(address) = self.fault.get() {
                return Err(address);
            }
        }
        Err(pc)
    }
    pub fn userop(
        &mut self,
        id: u32,
        a: u64,
        b: u64,
        d: u64,
        input_size: usize,
        output_size: usize,
    ) -> u64 {
        match id {
            // Names/IDs are generated from the same Ghidra language as the code.
            _ => self.userop_named(id, a, b, d, input_size, output_size),
        }
    }
    fn userop_named(
        &mut self,
        id: u32,
        a: u64,
        b: u64,
        d: u64,
        input_size: usize,
        output_size: usize,
    ) -> u64 {
        match crate::userops::name(id) {
            "setISAMode"
            | "DataSynchronizationBarrier"
            | "InstructionSynchronizationBarrier"
            | "DataMemoryBarrier"
            | "ExclusiveAccess" => 0,
            "hasExclusiveAccess" | "isCurrentModePrivileged" => 1,
            "enableIRQinterrupts" => {
                self.irq = true;
                0
            }
            "disableIRQinterrupts" => {
                self.irq = false;
                0
            }
            "isIRQinterruptsEnabled" => self.irq as u64,
            "setMainStackPointer" => {
                self.w(84, 4, a);
                0
            }
            "VectorSignedToFloat" => {
                let value = signed(a, input_size);
                if output_size == 8 {
                    (value as f64).to_bits()
                } else {
                    (value as f32).to_bits() as u64
                }
            }
            "VectorUnsignedToFloat" => {
                if output_size == 8 {
                    (a as f64).to_bits()
                } else {
                    (a as f32).to_bits() as u64
                }
            }
            "VectorFloatToSignedFixed" => {
                let value = f32::from_bits(a as u32) as f64 * 2f64.powi(d as i32);
                if b == 32 {
                    (value as i32 as u32) as u64
                } else {
                    value as i64 as u64
                }
            }
            _ => {
                self.fail(0xf0000000 | id);
                0
            }
        }
    }
    pub fn ram(&self) -> &[u8] {
        &self.dtcm
    }
    pub fn heap_used(&self) -> usize {
        self.heap_cursor
    }
    #[cfg(test)]
    pub fn assert_same_state(&self, other: &Self) {
        assert!(self.regs == other.regs, "registers changed");
        assert!(self.itcm == other.itcm, "ITCM changed");
        assert!(self.dtcm == other.dtcm, "DTCM changed");
        assert!(self.ocram == other.ocram, "OCRAM changed");
        assert!(self.heap == other.heap, "delay histories changed");
        assert_eq!(self.heap_cursor, other.heap_cursor);
        assert_eq!(self.fault.get(), other.fault.get());
        assert_eq!(self.irq, other.irq);
    }
}
