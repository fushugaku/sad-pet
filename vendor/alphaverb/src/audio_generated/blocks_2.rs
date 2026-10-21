#![allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut,
    unused_comparisons
)]
use crate::machine::{signed, signed_overflow, FastFma, Machine};
pub fn b_1cea(c: &mut Machine) -> u32 {
    {
        // 0x00001cea ldr r1,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001cec cmp r0,r1
        let mut t1a6e00 = 0u64;
        let value = (c.r(36, 4) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), c.r(36, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(32, 4)).wrapping_sub(c.r(36, 4))) & 0xffffffffu64;
        let value = (signed(((t1a6e00 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a6e00 >> 0) & 0xffffffffu64) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x00001cee bgt.w 0x0000310c
        let mut t183c00 = 0u64;
        let mut t183d00 = 0u64;
        let mut t183f00 = 0u64;
        t183c00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t183d00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t183f00 = (((((t183c00 >> 0) & 0xffu64) != 0) & (((t183d00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t183f00 >> 0) & 0xffu64) != 0 {
            return 0x310cu32;
        }
    }
    0x1cf2
}
pub fn b_1cf2(c: &mut Machine) -> u32 {
    {
        // 0x00001cf2 ldr r1,[0x00001df0]
        let value = c.load(0x1df0, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001cf4 vmov.f32 s10,0x3f000000
        let value = 1056964608u64;
        c.w(808, 4, value);
    }
    {
        // 0x00001cf8 vldr.32 s14,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00001cfc vldr.32 s11,[r1,#-0x4]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x00001d00 vldr.32 s15,[r1,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00001d04 vsub.f32 s12,s14,s11
        let value = (f32::from_bits(c.r(824, 4) as u32) - f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00001d08 vldr.32 s9,[r1,#-0x8]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(804, 4, value);
    }
    {
        // 0x00001d0c vsub.f32 s11,s11,s15
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00001d10 vsub.f32 s9,s9,s14
        let value = (f32::from_bits(c.r(804, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(804, 4, value);
    }
    {
        // 0x00001d14 vmov.f32 s15,s12
        let value = c.r(816, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00001d18 vfma.f32 s15,s11,s10
        let value = f32::from_bits(c.r(812, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001d1c vadd.f32 s12,s12,s15
        let value = (f32::from_bits(c.r(816, 4) as u32) + f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00001d20 vfma.f32 s12,s9,s10
        let value = f32::from_bits(c.r(804, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(816, 4) as u32),
            )
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00001d24 vadd.f32 s15,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001d28 vfnms.f32 s15,s12,s13
        let mut t49a00 = 0u64;
        let mut t49b00 = 0u64;
        t49a00 = ((-f32::from_bits(c.r(828, 4) as u32)).to_bits() as u64) & 0xffffffffu64;
        t49b00 = ((f32::from_bits(c.r(816, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64)
            & 0xffffffffu64;
        let value = (f32::from_bits(((t49a00 >> 0) & 0xffffffffu64) as u32)
            + f32::from_bits(((t49b00 >> 0) & 0xffffffffu64) as u32))
        .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001d2c vmul.f32 s15,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001d30 vfma.f32 s15,s11,s10
        let value = f32::from_bits(c.r(812, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001d34 vfma.f32 s14,s13,s15
        let value = f32::from_bits(c.r(820, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001d38 vadd.f32 s15,s14,s23
        let value = (f32::from_bits(c.r(824, 4) as u32) + f32::from_bits(c.r(860, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    0x1d3c
}
pub fn b_1d3c(c: &mut Machine) -> u32 {
    {
        // 0x00001d3c ldr r1,[0x00001df8]
        let value = c.load(0x1df8, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001d3e vmov.f32 s9,0x3f800000
        let value = 1065353216u64;
        c.w(804, 4, value);
    }
    {
        // 0x00001d42 vldr.32 s14,[pc,#0xb8]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((7490u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(184u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00001d46 vmov.f32 s10,0x3f000000
        let value = 1056964608u64;
        c.w(808, 4, value);
    }
    {
        // 0x00001d4a vstr.32 s15,[r1,#-0x30]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(48u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x00001d4e add.w r11,r1,#0x60
        let mut t187d00 = 0u64;
        t187d00 = (96u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = ((c.r(36, 4) as u128 + ((t187d00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_add(((t187d00 >> 0) & 0xffffffffu64));
        c.w(76, 4, value);
        let value = (signed(c.r(76, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(76, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00001d52 vldr.32 s15,[r5,#0x330]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(816u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00001d56 vmul.f32 s14,s25,s14
        let value = (f32::from_bits(c.r(868, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001d5a vldr.32 s13,[r11]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(76, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00001d5e sub.w r10,r11,#0x20
        let mut t187d00 = 0u64;
        t187d00 = (32u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(76, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(76, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(76, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(72, 4, value);
        let value = (signed(c.r(72, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(72, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00001d62 vadd.f32 s15,s15,s23
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(860, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001d66 vldr.32 s11,[r11,#-0x30]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(76, 4)).wrapping_sub(48u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x00001d6a vldr.32 s7,[r1,#0x28]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x00001d6e sub.w r9,r11,#0x24
        let mut t187d00 = 0u64;
        t187d00 = (36u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(76, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(76, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(76, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(68, 4, value);
        let value = (signed(c.r(68, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(68, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00001d72 ldr.w r1,[r11,#-0x24]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(76, 4)).wrapping_sub(36u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001d76 sub.w r8,r11,#0x28
        let mut t187d00 = 0u64;
        t187d00 = (40u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(76, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(76, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(76, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(64, 4, value);
        let value = (signed(c.r(64, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(64, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00001d7a vmul.f32 s15,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001d7e vldr.32 s12,[r11,#-0xc]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(76, 4)).wrapping_sub(12u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x00001d82 str.w r1,[r11,#-0x20]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(76, 4)).wrapping_sub(32u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x00001d86 sub.w lr,r11,#0x2c
        let mut t187d00 = 0u64;
        t187d00 = (44u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(76, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(76, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(76, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(88, 4, value);
        let value = (signed(c.r(88, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(88, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00001d8a ldr.w r1,[r11,#-0x28]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(76, 4)).wrapping_sub(40u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001d8e vadd.f32 s12,s12,s7
        let value = (f32::from_bits(c.r(816, 4) as u32) + f32::from_bits(c.r(796, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00001d92 vfma.f32 s15,s28,s27
        let value = f32::from_bits(c.r(880, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(876, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001d96 vldr.32 s8,[pc,#0x50]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((7574u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(80u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x00001d9a str.w r1,[r11,#-0x24]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(76, 4)).wrapping_sub(36u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x00001d9e sub.w r12,r11,#0xc
        let mut t187d00 = 0u64;
        t187d00 = (12u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(76, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(76, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(76, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(80, 4, value);
        let value = (signed(c.r(80, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(80, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00001da2 ldr.w r1,[r11,#-0x2c]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(76, 4)).wrapping_sub(44u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001da6 ldr.w r0,[r6,#0xed0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3792u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x00001daa str.w r1,[r11,#-0x28]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(76, 4)).wrapping_sub(40u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x00001dae ldr.w r1,[r6,#0xed8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3800u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001db2 vfms.f32 s11,s15,s13
        let value = (-f32::from_bits(c.r(828, 4) as u32))
            .fast_mul_add(
                f32::from_bits(c.r(820, 4) as u32),
                f32::from_bits(c.r(812, 4) as u32),
            )
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00001db6 ldr.w r4,[r6,#0xee0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3808u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x00001dba str r1,[sp,#0x20]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x00001dbc ldr.w r1,[r6,#0xf24]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3876u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001dc0 vldr.32 s7,[pc,#0x28]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((7616u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(40u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x00001dc4 vstr.32 s12,[r11,#-0xc]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(76, 4)).wrapping_sub(12u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x00001dc8 str r1,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x00001dca vfma.f32 s15,s13,s11
        let value = f32::from_bits(c.r(820, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(812, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001dce vstr.32 s11,[sp,#0x40]
        let mut t64000 = 0u64;
        t64000 = ((c.r(84, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(812, 4));
    }
    {
        // 0x00001dd2 str r2,[sp,#0x24]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x00001dd4 str r3,[sp,#0x38]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00001dd6 vstr.32 s15,[r11,#-0x2c]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(76, 4)).wrapping_sub(44u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x00001dda b 0x00001e10
        return 0x1e10u32;
    }
    0x1ddc
}
pub fn b_1e00(c: &mut Machine) -> u32 {
    {
        // 0x00001e00 movs r3,#0x0
        let value = 0u64;
        c.w(44, 4, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
    }
    {
        // 0x00001e02 str.w r3,[r6,#0xedc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3804u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00001e06 ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00001e08 cmp r0,r3
        let mut t1a6e00 = 0u64;
        let value = (c.r(44, 4) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), c.r(44, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(32, 4)).wrapping_sub(c.r(44, 4))) & 0xffffffffu64;
        let value = (signed(((t1a6e00 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a6e00 >> 0) & 0xffffffffu64) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x00001e0a bgt 0x00001ec6
        let mut t181b00 = 0u64;
        let mut t181c00 = 0u64;
        let mut t181e00 = 0u64;
        t181b00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t181c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t181e00 = (((((t181b00 >> 0) & 0xffu64) != 0) & (((t181c00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t181e00 >> 0) & 0xffu64) != 0 {
            return 0x1ec6u32;
        }
    }
    0x1e0c
}
pub fn b_1e0c(c: &mut Machine) -> u32 {
    {
        // 0x00001e0c vldr.32 s12,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    0x1e10
}
pub fn b_1e10(c: &mut Machine) -> u32 {
    {
        // 0x00001e10 vcmpe.f32 s12,s9
        let mut t35c00 = 0u64;
        let mut t35e00 = 0u64;
        let mut t36000 = 0u64;
        let mut t36100 = 0u64;
        let mut t36200 = 0u64;
        let mut t36400 = 0u64;
        let mut t36500 = 0u64;
        let mut t36600 = 0u64;
        let mut t36700 = 0u64;
        let mut t36800 = 0u64;
        let mut t36900 = 0u64;
        let mut t36a00 = 0u64;
        let mut t36b00 = 0u64;
        let mut t36c00 = 0u64;
        let mut t36d00 = 0u64;
        let mut t36e00 = 0u64;
        let mut t36f00 = 0u64;
        let mut t37000 = 0u64;
        t35c00 = ((f32::from_bits(c.r(816, 4) as u32) < f32::from_bits(c.r(804, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(816, 4) as u32) == f32::from_bits(c.r(804, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(804, 4) as u32) <= f32::from_bits(c.r(816, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(816, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(804, 4) as u32).is_nan() as u64) & 0xffu64;
        t36400 = (((t36100 >> 0) & 0xffu64) | ((t36200 >> 0) & 0xffu64)) & 0xffu64;
        t36500 = (c.r(180, 4) & 268435455u64) & 0xffffffffu64;
        t36600 = ((t35c00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36700 = (if 31u64 >= 32 {
            0
        } else {
            ((t36600 >> 0) & 0xffffffffu64) << (31u64 as u32)
        }) & 0xffffffffu64;
        t36800 =
            (((t36500 >> 0) & 0xffffffffu64) | ((t36700 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36900 = ((t35e00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36a00 = (if 30u64 >= 32 {
            0
        } else {
            ((t36900 >> 0) & 0xffffffffu64) << (30u64 as u32)
        }) & 0xffffffffu64;
        t36b00 =
            (((t36800 >> 0) & 0xffffffffu64) | ((t36a00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36c00 = ((t36000 >> 0) & 0xffu64) & 0xffffffffu64;
        t36d00 = (if 29u64 >= 32 {
            0
        } else {
            ((t36c00 >> 0) & 0xffffffffu64) << (29u64 as u32)
        }) & 0xffffffffu64;
        t36e00 =
            (((t36b00 >> 0) & 0xffffffffu64) | ((t36d00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36f00 = ((t36400 >> 0) & 0xffu64) & 0xffffffffu64;
        t37000 = (if 28u64 >= 32 {
            0
        } else {
            ((t36f00 >> 0) & 0xffffffffu64) << (28u64 as u32)
        }) & 0xffffffffu64;
        let value = (((t36e00 >> 0) & 0xffffffffu64) | ((t37000 >> 0) & 0xffffffffu64));
        c.w(180, 4, value);
    }
    {
        // 0x00001e14 vmrs apsr,fpscr
        let mut t6bd00 = 0u64;
        let mut t6bf00 = 0u64;
        let mut t6c000 = 0u64;
        let mut t6c200 = 0u64;
        let mut t6c300 = 0u64;
        let mut t6c500 = 0u64;
        let mut t6c600 = 0u64;
        t6bd00 = (if 31u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t6bd00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(96, 1, value);
        t6bf00 = (if 30u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (30u64 as u32)
        }) & 0xffffffffu64;
        t6c000 = (((t6bf00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c000 >> 0) & 0xffu64) & 1u64);
        c.w(97, 1, value);
        t6c200 = (if 29u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (29u64 as u32)
        }) & 0xffffffffu64;
        t6c300 = (((t6c200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c300 >> 0) & 0xffu64) & 1u64);
        c.w(98, 1, value);
        t6c500 = (if 28u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (28u64 as u32)
        }) & 0xffffffffu64;
        t6c600 = (((t6c500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c600 >> 0) & 0xffu64) & 1u64);
        c.w(99, 1, value);
    }
    {
        // 0x00001e18 ble 0x00001ede
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x1edeu32;
        }
    }
    0x1e1a
}
pub fn b_1e1a(c: &mut Machine) -> u32 {
    {
        // 0x00001e1a ldr r3,[0x00001fe8]
        let value = c.load(0x1fe8, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00001e1c vsub.f32 s12,s12,s9
        let value = (f32::from_bits(c.r(816, 4) as u32) - f32::from_bits(c.r(804, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00001e20 ldr r1,[sp,#0x8]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001e22 ldr r2,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00001e24 vldr.32 s15,[r3,#0x10]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00001e28 str r2,[r3,#0x4]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x00001e2a vcmpe.f32 s15,#0
        let mut t35c00 = 0u64;
        let mut t35e00 = 0u64;
        let mut t36000 = 0u64;
        let mut t36100 = 0u64;
        let mut t36200 = 0u64;
        let mut t36400 = 0u64;
        let mut t36500 = 0u64;
        let mut t36600 = 0u64;
        let mut t36700 = 0u64;
        let mut t36800 = 0u64;
        let mut t36900 = 0u64;
        let mut t36a00 = 0u64;
        let mut t36b00 = 0u64;
        let mut t36c00 = 0u64;
        let mut t36d00 = 0u64;
        let mut t36e00 = 0u64;
        let mut t36f00 = 0u64;
        let mut t37000 = 0u64;
        let mut t37900 = 0u64;
        t37900 = (0u64) & 0xffffffffu64;
        t35c00 = ((f32::from_bits(c.r(828, 4) as u32)
            < f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(828, 4) as u32)
            == f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)) as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)
            <= f32::from_bits(c.r(828, 4) as u32)) as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32).is_nan() as u64) & 0xffu64;
        t36400 = (((t36100 >> 0) & 0xffu64) | ((t36200 >> 0) & 0xffu64)) & 0xffu64;
        t36500 = (c.r(180, 4) & 268435455u64) & 0xffffffffu64;
        t36600 = ((t35c00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36700 = (if 31u64 >= 32 {
            0
        } else {
            ((t36600 >> 0) & 0xffffffffu64) << (31u64 as u32)
        }) & 0xffffffffu64;
        t36800 =
            (((t36500 >> 0) & 0xffffffffu64) | ((t36700 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36900 = ((t35e00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36a00 = (if 30u64 >= 32 {
            0
        } else {
            ((t36900 >> 0) & 0xffffffffu64) << (30u64 as u32)
        }) & 0xffffffffu64;
        t36b00 =
            (((t36800 >> 0) & 0xffffffffu64) | ((t36a00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36c00 = ((t36000 >> 0) & 0xffu64) & 0xffffffffu64;
        t36d00 = (if 29u64 >= 32 {
            0
        } else {
            ((t36c00 >> 0) & 0xffffffffu64) << (29u64 as u32)
        }) & 0xffffffffu64;
        t36e00 =
            (((t36b00 >> 0) & 0xffffffffu64) | ((t36d00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36f00 = ((t36400 >> 0) & 0xffu64) & 0xffffffffu64;
        t37000 = (if 28u64 >= 32 {
            0
        } else {
            ((t36f00 >> 0) & 0xffffffffu64) << (28u64 as u32)
        }) & 0xffffffffu64;
        let value = (((t36e00 >> 0) & 0xffffffffu64) | ((t37000 >> 0) & 0xffffffffu64));
        c.w(180, 4, value);
    }
    {
        // 0x00001e2e ldr.w r2,[r3,#-0x4]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00001e32 vstr.32 s12,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x00001e36 str r2,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x00001e38 ldr.w r2,[r3,#-0x8]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00001e3c vmrs apsr,fpscr
        let mut t6bd00 = 0u64;
        let mut t6bf00 = 0u64;
        let mut t6c000 = 0u64;
        let mut t6c200 = 0u64;
        let mut t6c300 = 0u64;
        let mut t6c500 = 0u64;
        let mut t6c600 = 0u64;
        t6bd00 = (if 31u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t6bd00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(96, 1, value);
        t6bf00 = (if 30u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (30u64 as u32)
        }) & 0xffffffffu64;
        t6c000 = (((t6bf00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c000 >> 0) & 0xffu64) & 1u64);
        c.w(97, 1, value);
        t6c200 = (if 29u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (29u64 as u32)
        }) & 0xffffffffu64;
        t6c300 = (((t6c200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c300 >> 0) & 0xffu64) & 1u64);
        c.w(98, 1, value);
        t6c500 = (if 28u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (28u64 as u32)
        }) & 0xffffffffu64;
        t6c600 = (((t6c500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c600 >> 0) & 0xffu64) & 1u64);
        c.w(99, 1, value);
    }
    {
        // 0x00001e40 str.w r2,[r3,#-0x4]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x00001e44 ldr.w r2,[r6,#0xedc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3804u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00001e48 sub.w r1,r2,r1
        let mut t1fe000 = 0u64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        t1fe000 = (c.r(36, 4)) & 0xffffffffu64;
        let value = (((t1fe000 >> 0) & 0xffffffffu64) <= c.r(40, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(40, 4), ((t1fe000 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(40, 4)).wrapping_sub(((t1fe000 >> 0) & 0xffffffffu64));
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00001e4c bne.w 0x0000265c
        let mut t182500 = 0u64;
        t182500 = ((c.r(97, 1) == 0u64) as u64) & 0xffu64;
        if ((t182500 >> 0) & 0xffu64) != 0 {
            return 0x265cu32;
        }
    }
    0x1e50
}
pub fn b_1e50(c: &mut Machine) -> u32 {
    {
        // 0x00001e50 cmp r1,#0x0
        let mut t1a6800 = 0u64;
        let value = (0u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 0u64, 4, true);
        c.w(103, 1, value);
        t1a6800 = ((c.r(36, 4)).wrapping_sub(0u64)) & 0xffffffffu64;
        let value = (signed(((t1a6800 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a6800 >> 0) & 0xffffffffu64) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x00001e52 it lt
        let mut t185c00 = 0u64;
        t185c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
    }
    {
        // 0x00001e54 add.lt r1,r1,r0
        let mut t1d00 = 0u64;
        let mut t2700 = 0u64;
        t1d00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2700 = ((((t1d00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x1e56u32;
        }
        let value = ((c.r(36, 4) as u128 + c.r(32, 4) as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), c.r(32, 4), 4, false);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_add(c.r(32, 4));
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    0x1e56
}
pub fn b_1e56(c: &mut Machine) -> u32 {
    {
        // 0x00001e56 add.w r1,r4,r1, lsl #0x2
        let mut t189200 = 0u64;
        let mut t189300 = 0u64;
        let mut t189500 = 0u64;
        let mut t189800 = 0u64;
        let mut t1a0b00 = 0u64;
        t189200 = ((32u64).wrapping_sub(2u64)) & 0xffffffffu64;
        t189300 = (if ((t189200 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(36, 4)) >> (((t189200 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t189500 = (((t189300 >> 0) & 0xffffffffu64) & 1u64) & 0xffffffffu64;
        let value = ((t189500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        t189800 = (if 2u64 >= 32 {
            0
        } else {
            (c.r(36, 4)) << (2u64 as u32)
        }) & 0xffffffffu64;
        t1a0b00 = ((t189800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        let value = ((c.r(48, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00001e5a ldr r1,[r1,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(36, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001e5c str.w r1,[r3,#-0x8]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    0x1e60
}
pub fn b_1e60(c: &mut Machine) -> u32 {
    {
        // 0x00001e60 vldr.32 s5,[r8]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(64, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(788, 4, value);
    }
    {
        // 0x00001e64 vldr.32 s6,[r9]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(68, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x00001e68 ldr r3,[0x00001fec]
        let value = c.load(0x1fec, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00001e6a vsub.f32 s13,s5,s6
        let value = (f32::from_bits(c.r(788, 4) as u32) - f32::from_bits(c.r(792, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00001e6e vldr.32 s4,[lr]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(88, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(784, 4, value);
    }
    {
        // 0x00001e72 vldr.32 s15,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00001e76 add.w r3,r4,r2, lsl #0x2
        let mut t189200 = 0u64;
        let mut t189300 = 0u64;
        let mut t189500 = 0u64;
        let mut t189800 = 0u64;
        let mut t1a0b00 = 0u64;
        t189200 = ((32u64).wrapping_sub(2u64)) & 0xffffffffu64;
        t189300 = (if ((t189200 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(40, 4)) >> (((t189200 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t189500 = (((t189300 >> 0) & 0xffffffffu64) & 1u64) & 0xffffffffu64;
        let value = ((t189500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        t189800 = (if 2u64 >= 32 {
            0
        } else {
            (c.r(40, 4)) << (2u64 as u32)
        }) & 0xffffffffu64;
        t1a0b00 = ((t189800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        let value = ((c.r(48, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(44, 4, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00001e7a vsub.f32 s6,s6,s4
        let value = (f32::from_bits(c.r(792, 4) as u32) - f32::from_bits(c.r(784, 4) as u32))
            .to_bits() as u64;
        c.w(792, 4, value);
    }
    {
        // 0x00001e7e vldr.32 s3,[r10]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(72, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(780, 4, value);
    }
    {
        // 0x00001e82 vmul.f32 s12,s12,s15
        let value = (f32::from_bits(c.r(816, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00001e86 adds r2,#0x1
        let value = ((c.r(40, 4) as u128 + 1u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(40, 4), 1u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(40, 4)).wrapping_add(1u64);
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x00001e88 vmov.f32 s15,s13
        let value = c.r(820, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00001e8c vsub.f32 s4,s3,s5
        let value = (f32::from_bits(c.r(780, 4) as u32) - f32::from_bits(c.r(788, 4) as u32))
            .to_bits() as u64;
        c.w(784, 4, value);
    }
    {
        // 0x00001e90 cmp r0,r2
        let mut t1a6e00 = 0u64;
        let value = (c.r(40, 4) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), c.r(40, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(32, 4)).wrapping_sub(c.r(40, 4))) & 0xffffffffu64;
        let value = (signed(((t1a6e00 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a6e00 >> 0) & 0xffffffffu64) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x00001e92 vfma.f32 s15,s6,s10
        let value = f32::from_bits(c.r(792, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001e96 vadd.f32 s13,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) + f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00001e9a vfma.f32 s13,s4,s10
        let value = f32::from_bits(c.r(784, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(820, 4) as u32),
            )
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00001e9e vadd.f32 s15,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001ea2 vfnms.f32 s15,s12,s13
        let mut t49a00 = 0u64;
        let mut t49b00 = 0u64;
        t49a00 = ((-f32::from_bits(c.r(828, 4) as u32)).to_bits() as u64) & 0xffffffffu64;
        t49b00 = ((f32::from_bits(c.r(816, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64)
            & 0xffffffffu64;
        let value = (f32::from_bits(((t49a00 >> 0) & 0xffffffffu64) as u32)
            + f32::from_bits(((t49b00 >> 0) & 0xffffffffu64) as u32))
        .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001ea6 vmov.f32 s13,s5
        let value = c.r(788, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00001eaa vmul.f32 s15,s12,s15
        let value = (f32::from_bits(c.r(816, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001eae vfma.f32 s15,s6,s10
        let value = f32::from_bits(c.r(792, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001eb2 vfma.f32 s13,s12,s15
        let value = f32::from_bits(c.r(816, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(820, 4) as u32),
            )
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00001eb6 vstr.32 s13,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x00001eba ble 0x00001e00
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x1e00u32;
        }
    }
    0x1ebc
}
pub fn b_1ebc(c: &mut Machine) -> u32 {
    {
        // 0x00001ebc ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00001ebe str.w r2,[r6,#0xedc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3804u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x00001ec2 cmp r0,r3
        let mut t1a6e00 = 0u64;
        let value = (c.r(44, 4) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), c.r(44, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(32, 4)).wrapping_sub(c.r(44, 4))) & 0xffffffffu64;
        let value = (signed(((t1a6e00 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a6e00 >> 0) & 0xffffffffu64) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x00001ec4 ble 0x00001e0c
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x1e0cu32;
        }
    }
    0x1ec6
}
pub fn b_1ec6(c: &mut Machine) -> u32 {
    {
        // 0x00001ec6 vldr.32 s12,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x00001eca ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00001ecc vcmpe.f32 s12,s9
        let mut t35c00 = 0u64;
        let mut t35e00 = 0u64;
        let mut t36000 = 0u64;
        let mut t36100 = 0u64;
        let mut t36200 = 0u64;
        let mut t36400 = 0u64;
        let mut t36500 = 0u64;
        let mut t36600 = 0u64;
        let mut t36700 = 0u64;
        let mut t36800 = 0u64;
        let mut t36900 = 0u64;
        let mut t36a00 = 0u64;
        let mut t36b00 = 0u64;
        let mut t36c00 = 0u64;
        let mut t36d00 = 0u64;
        let mut t36e00 = 0u64;
        let mut t36f00 = 0u64;
        let mut t37000 = 0u64;
        t35c00 = ((f32::from_bits(c.r(816, 4) as u32) < f32::from_bits(c.r(804, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(816, 4) as u32) == f32::from_bits(c.r(804, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(804, 4) as u32) <= f32::from_bits(c.r(816, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(816, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(804, 4) as u32).is_nan() as u64) & 0xffu64;
        t36400 = (((t36100 >> 0) & 0xffu64) | ((t36200 >> 0) & 0xffu64)) & 0xffu64;
        t36500 = (c.r(180, 4) & 268435455u64) & 0xffffffffu64;
        t36600 = ((t35c00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36700 = (if 31u64 >= 32 {
            0
        } else {
            ((t36600 >> 0) & 0xffffffffu64) << (31u64 as u32)
        }) & 0xffffffffu64;
        t36800 =
            (((t36500 >> 0) & 0xffffffffu64) | ((t36700 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36900 = ((t35e00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36a00 = (if 30u64 >= 32 {
            0
        } else {
            ((t36900 >> 0) & 0xffffffffu64) << (30u64 as u32)
        }) & 0xffffffffu64;
        t36b00 =
            (((t36800 >> 0) & 0xffffffffu64) | ((t36a00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36c00 = ((t36000 >> 0) & 0xffu64) & 0xffffffffu64;
        t36d00 = (if 29u64 >= 32 {
            0
        } else {
            ((t36c00 >> 0) & 0xffffffffu64) << (29u64 as u32)
        }) & 0xffffffffu64;
        t36e00 =
            (((t36b00 >> 0) & 0xffffffffu64) | ((t36d00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36f00 = ((t36400 >> 0) & 0xffu64) & 0xffffffffu64;
        t37000 = (if 28u64 >= 32 {
            0
        } else {
            ((t36f00 >> 0) & 0xffffffffu64) << (28u64 as u32)
        }) & 0xffffffffu64;
        let value = (((t36e00 >> 0) & 0xffffffffu64) | ((t37000 >> 0) & 0xffffffffu64));
        c.w(180, 4, value);
    }
    {
        // 0x00001ed0 adds r3,#0x1
        let value = ((c.r(44, 4) as u128 + 1u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), 1u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_add(1u64);
        c.w(44, 4, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x00001ed2 vmrs apsr,fpscr
        let mut t6bd00 = 0u64;
        let mut t6bf00 = 0u64;
        let mut t6c000 = 0u64;
        let mut t6c200 = 0u64;
        let mut t6c300 = 0u64;
        let mut t6c500 = 0u64;
        let mut t6c600 = 0u64;
        t6bd00 = (if 31u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t6bd00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(96, 1, value);
        t6bf00 = (if 30u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (30u64 as u32)
        }) & 0xffffffffu64;
        t6c000 = (((t6bf00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c000 >> 0) & 0xffu64) & 1u64);
        c.w(97, 1, value);
        t6c200 = (if 29u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (29u64 as u32)
        }) & 0xffffffffu64;
        t6c300 = (((t6c200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c300 >> 0) & 0xffu64) & 1u64);
        c.w(98, 1, value);
        t6c500 = (if 28u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (28u64 as u32)
        }) & 0xffffffffu64;
        t6c600 = (((t6c500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c600 >> 0) & 0xffu64) & 1u64);
        c.w(99, 1, value);
    }
    {
        // 0x00001ed6 str r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00001ed8 str.w r3,[r6,#0xf24]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3876u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00001edc bgt 0x00001e1a
        let mut t181b00 = 0u64;
        let mut t181c00 = 0u64;
        let mut t181e00 = 0u64;
        t181b00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t181c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t181e00 = (((((t181b00 >> 0) & 0xffu64) != 0) & (((t181c00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t181e00 >> 0) & 0xffu64) != 0 {
            return 0x1e1au32;
        }
    }
    0x1ede
}
pub fn b_1ede(c: &mut Machine) -> u32 {
    {
        // 0x00001ede ldrb.w r1,[r6,#0xf18]
        let mut t19c800 = 0u64;
        let mut t1aa600 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3864u64)) & 0xffffffffu64;
        t1aa600 = (c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 1)) & 0xffu64;
        let value = ((t1aa600 >> 0) & 0xffu64);
        c.w(36, 4, value);
    }
    {
        // 0x00001ee2 ldr r2,[sp,#0x24]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00001ee4 ldr r3,[sp,#0x38]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00001ee6 cbz r1,0x00001f0c
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(36, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x1f0cu32;
        }
    }
    0x1ee8
}
pub fn b_1ee8(c: &mut Machine) -> u32 {
    {
        // 0x00001ee8 ldr r1,[0x00001ff0]
        let value = c.load(0x1ff0, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001eea vmov.f32 s10,0x3f000000
        let value = 1056964608u64;
        c.w(808, 4, value);
    }
    {
        // 0x00001eee vmov.f32 s13,0x3f800000
        let value = 1065353216u64;
        c.w(820, 4, value);
    }
    {
        // 0x00001ef2 vldr.32 s15,[r1,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00001ef6 vmul.f32 s15,s12,s15
        let value = (f32::from_bits(c.r(816, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001efa vmul.f32 s12,s15,s10
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00001efe vcmpe.f32 s12,s9
        let mut t35c00 = 0u64;
        let mut t35e00 = 0u64;
        let mut t36000 = 0u64;
        let mut t36100 = 0u64;
        let mut t36200 = 0u64;
        let mut t36400 = 0u64;
        let mut t36500 = 0u64;
        let mut t36600 = 0u64;
        let mut t36700 = 0u64;
        let mut t36800 = 0u64;
        let mut t36900 = 0u64;
        let mut t36a00 = 0u64;
        let mut t36b00 = 0u64;
        let mut t36c00 = 0u64;
        let mut t36d00 = 0u64;
        let mut t36e00 = 0u64;
        let mut t36f00 = 0u64;
        let mut t37000 = 0u64;
        t35c00 = ((f32::from_bits(c.r(816, 4) as u32) < f32::from_bits(c.r(804, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(816, 4) as u32) == f32::from_bits(c.r(804, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(804, 4) as u32) <= f32::from_bits(c.r(816, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(816, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(804, 4) as u32).is_nan() as u64) & 0xffu64;
        t36400 = (((t36100 >> 0) & 0xffu64) | ((t36200 >> 0) & 0xffu64)) & 0xffu64;
        t36500 = (c.r(180, 4) & 268435455u64) & 0xffffffffu64;
        t36600 = ((t35c00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36700 = (if 31u64 >= 32 {
            0
        } else {
            ((t36600 >> 0) & 0xffffffffu64) << (31u64 as u32)
        }) & 0xffffffffu64;
        t36800 =
            (((t36500 >> 0) & 0xffffffffu64) | ((t36700 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36900 = ((t35e00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36a00 = (if 30u64 >= 32 {
            0
        } else {
            ((t36900 >> 0) & 0xffffffffu64) << (30u64 as u32)
        }) & 0xffffffffu64;
        t36b00 =
            (((t36800 >> 0) & 0xffffffffu64) | ((t36a00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36c00 = ((t36000 >> 0) & 0xffu64) & 0xffffffffu64;
        t36d00 = (if 29u64 >= 32 {
            0
        } else {
            ((t36c00 >> 0) & 0xffffffffu64) << (29u64 as u32)
        }) & 0xffffffffu64;
        t36e00 =
            (((t36b00 >> 0) & 0xffffffffu64) | ((t36d00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36f00 = ((t36400 >> 0) & 0xffu64) & 0xffffffffu64;
        t37000 = (if 28u64 >= 32 {
            0
        } else {
            ((t36f00 >> 0) & 0xffffffffu64) << (28u64 as u32)
        }) & 0xffffffffu64;
        let value = (((t36e00 >> 0) & 0xffffffffu64) | ((t37000 >> 0) & 0xffffffffu64));
        c.w(180, 4, value);
    }
    {
        // 0x00001f02 vmrs apsr,fpscr
        let mut t6bd00 = 0u64;
        let mut t6bf00 = 0u64;
        let mut t6c000 = 0u64;
        let mut t6c200 = 0u64;
        let mut t6c300 = 0u64;
        let mut t6c500 = 0u64;
        let mut t6c600 = 0u64;
        t6bd00 = (if 31u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t6bd00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(96, 1, value);
        t6bf00 = (if 30u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (30u64 as u32)
        }) & 0xffffffffu64;
        t6c000 = (((t6bf00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c000 >> 0) & 0xffu64) & 1u64);
        c.w(97, 1, value);
        t6c200 = (if 29u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (29u64 as u32)
        }) & 0xffffffffu64;
        t6c300 = (((t6c200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c300 >> 0) & 0xffu64) & 1u64);
        c.w(98, 1, value);
        t6c500 = (if 28u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (28u64 as u32)
        }) & 0xffffffffu64;
        t6c600 = (((t6c500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c600 >> 0) & 0xffu64) & 1u64);
        c.w(99, 1, value);
    }
    {
        // 0x00001f06 it pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x00001f08 vmov.pl.f32 s12,s13
        let mut tf00 = 0u64;
        let mut t2700 = 0u64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2700 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x1f0cu32;
        }
        let value = c.r(820, 4);
        c.w(816, 4, value);
    }
    0x1f0c
}
pub fn b_1f0c(c: &mut Machine) -> u32 {
    {
        // 0x00001f0c ldr r1,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001f0e cmp r0,r1
        let mut t1a6e00 = 0u64;
        let value = (c.r(36, 4) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), c.r(36, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(32, 4)).wrapping_sub(c.r(36, 4))) & 0xffffffffu64;
        let value = (signed(((t1a6e00 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a6e00 >> 0) & 0xffffffffu64) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x00001f10 bgt.w 0x00003114
        let mut t183c00 = 0u64;
        let mut t183d00 = 0u64;
        let mut t183f00 = 0u64;
        t183c00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t183d00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t183f00 = (((((t183c00 >> 0) & 0xffu64) != 0) & (((t183d00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t183f00 >> 0) & 0xffu64) != 0 {
            return 0x3114u32;
        }
    }
    0x1f14
}
pub fn b_1f14(c: &mut Machine) -> u32 {
    {
        // 0x00001f14 ldr r1,[0x00001fe8]
        let value = c.load(0x1fe8, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001f16 vmov.f32 s8,0x3f000000
        let value = 1056964608u64;
        c.w(800, 4, value);
    }
    {
        // 0x00001f1a vldr.32 s13,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00001f1e vldr.32 s9,[r1,#-0x4]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(804, 4, value);
    }
    {
        // 0x00001f22 vldr.32 s15,[r1,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00001f26 vsub.f32 s10,s13,s9
        let value = (f32::from_bits(c.r(820, 4) as u32) - f32::from_bits(c.r(804, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x00001f2a vldr.32 s7,[r1,#-0x8]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x00001f2e vsub.f32 s9,s9,s15
        let value = (f32::from_bits(c.r(804, 4) as u32) - f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(804, 4, value);
    }
    {
        // 0x00001f32 vsub.f32 s7,s7,s13
        let value = (f32::from_bits(c.r(796, 4) as u32) - f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(796, 4, value);
    }
    {
        // 0x00001f36 vmov.f32 s15,s10
        let value = c.r(808, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00001f3a vfma.f32 s15,s9,s8
        let value = f32::from_bits(c.r(804, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(800, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001f3e vadd.f32 s10,s10,s15
        let value = (f32::from_bits(c.r(808, 4) as u32) + f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x00001f42 vfma.f32 s10,s7,s8
        let value = f32::from_bits(c.r(796, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(800, 4) as u32),
                f32::from_bits(c.r(808, 4) as u32),
            )
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x00001f46 vadd.f32 s15,s15,s10
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001f4a vfnms.f32 s15,s10,s12
        let mut t49a00 = 0u64;
        let mut t49b00 = 0u64;
        t49a00 = ((-f32::from_bits(c.r(828, 4) as u32)).to_bits() as u64) & 0xffffffffu64;
        t49b00 = ((f32::from_bits(c.r(808, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64)
            & 0xffffffffu64;
        let value = (f32::from_bits(((t49a00 >> 0) & 0xffffffffu64) as u32)
            + f32::from_bits(((t49b00 >> 0) & 0xffffffffu64) as u32))
        .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001f4e vmul.f32 s15,s12,s15
        let value = (f32::from_bits(c.r(816, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001f52 vfma.f32 s15,s9,s8
        let value = f32::from_bits(c.r(804, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(800, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001f56 vfma.f32 s13,s12,s15
        let value = f32::from_bits(c.r(816, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(820, 4) as u32),
            )
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00001f5a vadd.f32 s15,s13,s23
        let value = (f32::from_bits(c.r(820, 4) as u32) + f32::from_bits(c.r(860, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    0x1f5e
}
pub fn b_1f5e(c: &mut Machine) -> u32 {
    {
        // 0x00001f5e ldr r1,[0x00001ff4]
        let value = c.load(0x1ff4, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001f60 vmov.f32 s9,0x3f800000
        let value = 1065353216u64;
        c.w(804, 4, value);
    }
    {
        // 0x00001f64 vstr.32 s15,[r11,#-0x30]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(76, 4)).wrapping_sub(48u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x00001f68 vmov.f32 s10,0x3f000000
        let value = 1056964608u64;
        c.w(808, 4, value);
    }
    {
        // 0x00001f6c vldr.32 s13,[r1,#0x3c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(60u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00001f70 add.w r11,r1,#0x3c
        let mut t187d00 = 0u64;
        t187d00 = (60u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = ((c.r(36, 4) as u128 + ((t187d00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_add(((t187d00 >> 0) & 0xffffffffu64));
        c.w(76, 4, value);
        let value = (signed(c.r(76, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(76, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00001f74 vldr.32 s15,[r11,#-0x30]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(76, 4)).wrapping_sub(48u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00001f78 vldr.32 s7,[r1,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x00001f7c sub.w r10,r11,#0x20
        let mut t187d00 = 0u64;
        t187d00 = (32u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(76, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(76, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(76, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(72, 4, value);
        let value = (signed(c.r(72, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(72, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00001f80 vfms.f32 s15,s13,s11
        let value = (-f32::from_bits(c.r(820, 4) as u32))
            .fast_mul_add(
                f32::from_bits(c.r(812, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001f84 ldr.w r1,[r11,#-0x24]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(76, 4)).wrapping_sub(36u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001f88 vldr.32 s12,[r11,#-0xc]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(76, 4)).wrapping_sub(12u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x00001f8c sub.w r9,r11,#0x24
        let mut t187d00 = 0u64;
        t187d00 = (36u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(76, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(76, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(76, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(68, 4, value);
        let value = (signed(c.r(68, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(68, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00001f90 str.w r1,[r11,#-0x20]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(76, 4)).wrapping_sub(32u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x00001f94 sub.w r8,r11,#0x28
        let mut t187d00 = 0u64;
        t187d00 = (40u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(76, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(76, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(76, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(64, 4, value);
        let value = (signed(c.r(64, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(64, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00001f98 ldr.w r1,[r11,#-0x28]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(76, 4)).wrapping_sub(40u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001f9c vadd.f32 s12,s12,s7
        let value = (f32::from_bits(c.r(816, 4) as u32) + f32::from_bits(c.r(796, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00001fa0 vldr.32 s8,[pc,#0x54]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((8096u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(84u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x00001fa4 sub.w lr,r11,#0x2c
        let mut t187d00 = 0u64;
        t187d00 = (44u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(76, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(76, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(76, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(88, 4, value);
        let value = (signed(c.r(88, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(88, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00001fa8 str.w r1,[r11,#-0x24]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(76, 4)).wrapping_sub(36u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x00001fac sub.w r12,r11,#0xc
        let mut t187d00 = 0u64;
        t187d00 = (12u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(76, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(76, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(76, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(80, 4, value);
        let value = (signed(c.r(80, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(80, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00001fb0 ldr.w r1,[r11,#-0x2c]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(76, 4)).wrapping_sub(44u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001fb4 vfma.f32 s11,s13,s15
        let value = f32::from_bits(c.r(820, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(812, 4) as u32),
            )
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00001fb8 ldr.w r0,[r6,#0xf90]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3984u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x00001fbc str.w r1,[r11,#-0x28]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(76, 4)).wrapping_sub(40u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x00001fc0 ldr.w r1,[r6,#0xf98]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3992u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001fc4 ldr.w r4,[r6,#0xfa0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(4000u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x00001fc8 str r1,[sp,#0x8]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x00001fca ldr.w r1,[r6,#0xfe4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(4068u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001fce vldr.32 s7,[pc,#0x2c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((8142u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(44u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x00001fd2 vstr.32 s12,[r11,#-0xc]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(76, 4)).wrapping_sub(12u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x00001fd6 vstr.32 s15,[sp,#0x40]
        let mut t64000 = 0u64;
        t64000 = ((c.r(84, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x00001fda vstr.32 s11,[r11,#-0x2c]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(76, 4)).wrapping_sub(44u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(812, 4));
    }
    {
        // 0x00001fde str r1,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x00001fe0 strd r2,r3,[sp,#0x20]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(84, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00001fe4 b 0x00002010
        return 0x2010u32;
    }
    0x1fe6
}
pub fn b_2000(c: &mut Machine) -> u32 {
    {
        // 0x00002000 movs r3,#0x0
        let value = 0u64;
        c.w(44, 4, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
    }
    {
        // 0x00002002 str.w r3,[r6,#0xf9c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3996u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00002006 ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00002008 cmp r0,r3
        let mut t1a6e00 = 0u64;
        let value = (c.r(44, 4) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), c.r(44, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(32, 4)).wrapping_sub(c.r(44, 4))) & 0xffffffffu64;
        let value = (signed(((t1a6e00 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a6e00 >> 0) & 0xffffffffu64) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x0000200a bgt 0x000020ca
        let mut t181b00 = 0u64;
        let mut t181c00 = 0u64;
        let mut t181e00 = 0u64;
        t181b00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t181c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t181e00 = (((((t181b00 >> 0) & 0xffu64) != 0) & (((t181c00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t181e00 >> 0) & 0xffu64) != 0 {
            return 0x20cau32;
        }
    }
    0x200c
}
pub fn b_200c(c: &mut Machine) -> u32 {
    {
        // 0x0000200c vldr.32 s12,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    0x2010
}
pub fn b_2010(c: &mut Machine) -> u32 {
    {
        // 0x00002010 vcmpe.f32 s12,s9
        let mut t35c00 = 0u64;
        let mut t35e00 = 0u64;
        let mut t36000 = 0u64;
        let mut t36100 = 0u64;
        let mut t36200 = 0u64;
        let mut t36400 = 0u64;
        let mut t36500 = 0u64;
        let mut t36600 = 0u64;
        let mut t36700 = 0u64;
        let mut t36800 = 0u64;
        let mut t36900 = 0u64;
        let mut t36a00 = 0u64;
        let mut t36b00 = 0u64;
        let mut t36c00 = 0u64;
        let mut t36d00 = 0u64;
        let mut t36e00 = 0u64;
        let mut t36f00 = 0u64;
        let mut t37000 = 0u64;
        t35c00 = ((f32::from_bits(c.r(816, 4) as u32) < f32::from_bits(c.r(804, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(816, 4) as u32) == f32::from_bits(c.r(804, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(804, 4) as u32) <= f32::from_bits(c.r(816, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(816, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(804, 4) as u32).is_nan() as u64) & 0xffu64;
        t36400 = (((t36100 >> 0) & 0xffu64) | ((t36200 >> 0) & 0xffu64)) & 0xffu64;
        t36500 = (c.r(180, 4) & 268435455u64) & 0xffffffffu64;
        t36600 = ((t35c00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36700 = (if 31u64 >= 32 {
            0
        } else {
            ((t36600 >> 0) & 0xffffffffu64) << (31u64 as u32)
        }) & 0xffffffffu64;
        t36800 =
            (((t36500 >> 0) & 0xffffffffu64) | ((t36700 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36900 = ((t35e00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36a00 = (if 30u64 >= 32 {
            0
        } else {
            ((t36900 >> 0) & 0xffffffffu64) << (30u64 as u32)
        }) & 0xffffffffu64;
        t36b00 =
            (((t36800 >> 0) & 0xffffffffu64) | ((t36a00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36c00 = ((t36000 >> 0) & 0xffu64) & 0xffffffffu64;
        t36d00 = (if 29u64 >= 32 {
            0
        } else {
            ((t36c00 >> 0) & 0xffffffffu64) << (29u64 as u32)
        }) & 0xffffffffu64;
        t36e00 =
            (((t36b00 >> 0) & 0xffffffffu64) | ((t36d00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36f00 = ((t36400 >> 0) & 0xffu64) & 0xffffffffu64;
        t37000 = (if 28u64 >= 32 {
            0
        } else {
            ((t36f00 >> 0) & 0xffffffffu64) << (28u64 as u32)
        }) & 0xffffffffu64;
        let value = (((t36e00 >> 0) & 0xffffffffu64) | ((t37000 >> 0) & 0xffffffffu64));
        c.w(180, 4, value);
    }
    {
        // 0x00002014 vmrs apsr,fpscr
        let mut t6bd00 = 0u64;
        let mut t6bf00 = 0u64;
        let mut t6c000 = 0u64;
        let mut t6c200 = 0u64;
        let mut t6c300 = 0u64;
        let mut t6c500 = 0u64;
        let mut t6c600 = 0u64;
        t6bd00 = (if 31u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t6bd00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(96, 1, value);
        t6bf00 = (if 30u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (30u64 as u32)
        }) & 0xffffffffu64;
        t6c000 = (((t6bf00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c000 >> 0) & 0xffu64) & 1u64);
        c.w(97, 1, value);
        t6c200 = (if 29u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (29u64 as u32)
        }) & 0xffffffffu64;
        t6c300 = (((t6c200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c300 >> 0) & 0xffu64) & 1u64);
        c.w(98, 1, value);
        t6c500 = (if 28u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (28u64 as u32)
        }) & 0xffffffffu64;
        t6c600 = (((t6c500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c600 >> 0) & 0xffu64) & 1u64);
        c.w(99, 1, value);
    }
    {
        // 0x00002018 ble 0x000020e2
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x20e2u32;
        }
    }
    0x201a
}
pub fn b_201a(c: &mut Machine) -> u32 {
    {
        // 0x0000201a ldr r3,[0x00002398]
        let value = c.load(0x2398, 4);
        c.w(44, 4, value);
    }
    {
        // 0x0000201c vsub.f32 s12,s12,s9
        let value = (f32::from_bits(c.r(816, 4) as u32) - f32::from_bits(c.r(804, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00002020 ldr r1,[sp,#0x10]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00002022 ldr r2,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00002024 vldr.32 s13,[r3,#0x10]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00002028 str r2,[r3,#0x4]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x0000202a vcmpe.f32 s13,#0
        let mut t35c00 = 0u64;
        let mut t35e00 = 0u64;
        let mut t36000 = 0u64;
        let mut t36100 = 0u64;
        let mut t36200 = 0u64;
        let mut t36400 = 0u64;
        let mut t36500 = 0u64;
        let mut t36600 = 0u64;
        let mut t36700 = 0u64;
        let mut t36800 = 0u64;
        let mut t36900 = 0u64;
        let mut t36a00 = 0u64;
        let mut t36b00 = 0u64;
        let mut t36c00 = 0u64;
        let mut t36d00 = 0u64;
        let mut t36e00 = 0u64;
        let mut t36f00 = 0u64;
        let mut t37000 = 0u64;
        let mut t37900 = 0u64;
        t37900 = (0u64) & 0xffffffffu64;
        t35c00 = ((f32::from_bits(c.r(820, 4) as u32)
            < f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(820, 4) as u32)
            == f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)) as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)
            <= f32::from_bits(c.r(820, 4) as u32)) as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(820, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32).is_nan() as u64) & 0xffu64;
        t36400 = (((t36100 >> 0) & 0xffu64) | ((t36200 >> 0) & 0xffu64)) & 0xffu64;
        t36500 = (c.r(180, 4) & 268435455u64) & 0xffffffffu64;
        t36600 = ((t35c00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36700 = (if 31u64 >= 32 {
            0
        } else {
            ((t36600 >> 0) & 0xffffffffu64) << (31u64 as u32)
        }) & 0xffffffffu64;
        t36800 =
            (((t36500 >> 0) & 0xffffffffu64) | ((t36700 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36900 = ((t35e00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36a00 = (if 30u64 >= 32 {
            0
        } else {
            ((t36900 >> 0) & 0xffffffffu64) << (30u64 as u32)
        }) & 0xffffffffu64;
        t36b00 =
            (((t36800 >> 0) & 0xffffffffu64) | ((t36a00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36c00 = ((t36000 >> 0) & 0xffu64) & 0xffffffffu64;
        t36d00 = (if 29u64 >= 32 {
            0
        } else {
            ((t36c00 >> 0) & 0xffffffffu64) << (29u64 as u32)
        }) & 0xffffffffu64;
        t36e00 =
            (((t36b00 >> 0) & 0xffffffffu64) | ((t36d00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36f00 = ((t36400 >> 0) & 0xffu64) & 0xffffffffu64;
        t37000 = (if 28u64 >= 32 {
            0
        } else {
            ((t36f00 >> 0) & 0xffffffffu64) << (28u64 as u32)
        }) & 0xffffffffu64;
        let value = (((t36e00 >> 0) & 0xffffffffu64) | ((t37000 >> 0) & 0xffffffffu64));
        c.w(180, 4, value);
    }
    {
        // 0x0000202e ldr.w r2,[r3,#-0x4]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00002032 vstr.32 s12,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x00002036 str r2,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x00002038 ldr.w r2,[r3,#-0x8]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x0000203c vmrs apsr,fpscr
        let mut t6bd00 = 0u64;
        let mut t6bf00 = 0u64;
        let mut t6c000 = 0u64;
        let mut t6c200 = 0u64;
        let mut t6c300 = 0u64;
        let mut t6c500 = 0u64;
        let mut t6c600 = 0u64;
        t6bd00 = (if 31u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t6bd00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(96, 1, value);
        t6bf00 = (if 30u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (30u64 as u32)
        }) & 0xffffffffu64;
        t6c000 = (((t6bf00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c000 >> 0) & 0xffu64) & 1u64);
        c.w(97, 1, value);
        t6c200 = (if 29u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (29u64 as u32)
        }) & 0xffffffffu64;
        t6c300 = (((t6c200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c300 >> 0) & 0xffu64) & 1u64);
        c.w(98, 1, value);
        t6c500 = (if 28u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (28u64 as u32)
        }) & 0xffffffffu64;
        t6c600 = (((t6c500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c600 >> 0) & 0xffu64) & 1u64);
        c.w(99, 1, value);
    }
    {
        // 0x00002040 str.w r2,[r3,#-0x4]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x00002044 ldr.w r2,[r6,#0xf9c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3996u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00002048 sub.w r1,r2,r1
        let mut t1fe000 = 0u64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        t1fe000 = (c.r(36, 4)) & 0xffffffffu64;
        let value = (((t1fe000 >> 0) & 0xffffffffu64) <= c.r(40, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(40, 4), ((t1fe000 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(40, 4)).wrapping_sub(((t1fe000 >> 0) & 0xffffffffu64));
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x0000204c bne.w 0x000026a0
        let mut t182500 = 0u64;
        t182500 = ((c.r(97, 1) == 0u64) as u64) & 0xffu64;
        if ((t182500 >> 0) & 0xffu64) != 0 {
            return 0x26a0u32;
        }
    }
    0x2050
}
pub fn b_2050(c: &mut Machine) -> u32 {
    {
        // 0x00002050 cmp r1,#0x0
        let mut t1a6800 = 0u64;
        let value = (0u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 0u64, 4, true);
        c.w(103, 1, value);
        t1a6800 = ((c.r(36, 4)).wrapping_sub(0u64)) & 0xffffffffu64;
        let value = (signed(((t1a6800 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a6800 >> 0) & 0xffffffffu64) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x00002052 it lt
        let mut t185c00 = 0u64;
        t185c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
    }
    {
        // 0x00002054 add.lt r1,r1,r0
        let mut t1d00 = 0u64;
        let mut t2700 = 0u64;
        t1d00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2700 = ((((t1d00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x2056u32;
        }
        let value = ((c.r(36, 4) as u128 + c.r(32, 4) as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), c.r(32, 4), 4, false);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_add(c.r(32, 4));
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    0x2056
}
pub fn b_2056(c: &mut Machine) -> u32 {
    {
        // 0x00002056 add.w r1,r4,r1, lsl #0x2
        let mut t189200 = 0u64;
        let mut t189300 = 0u64;
        let mut t189500 = 0u64;
        let mut t189800 = 0u64;
        let mut t1a0b00 = 0u64;
        t189200 = ((32u64).wrapping_sub(2u64)) & 0xffffffffu64;
        t189300 = (if ((t189200 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(36, 4)) >> (((t189200 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t189500 = (((t189300 >> 0) & 0xffffffffu64) & 1u64) & 0xffffffffu64;
        let value = ((t189500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        t189800 = (if 2u64 >= 32 {
            0
        } else {
            (c.r(36, 4)) << (2u64 as u32)
        }) & 0xffffffffu64;
        t1a0b00 = ((t189800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        let value = ((c.r(48, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x0000205a ldr r1,[r1,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(36, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x0000205c str.w r1,[r3,#-0x8]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    0x2060
}
pub fn b_2060(c: &mut Machine) -> u32 {
    {
        // 0x00002060 vldr.32 s5,[r8]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(64, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(788, 4, value);
    }
    {
        // 0x00002064 vldr.32 s6,[r9]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(68, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x00002068 ldr r3,[0x0000239c]
        let value = c.load(0x239c, 4);
        c.w(44, 4, value);
    }
    {
        // 0x0000206a vsub.f32 s13,s5,s6
        let value = (f32::from_bits(c.r(788, 4) as u32) - f32::from_bits(c.r(792, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x0000206e vldr.32 s3,[lr]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(88, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(780, 4, value);
    }
    {
        // 0x00002072 vldr.32 s4,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(784, 4, value);
    }
    {
        // 0x00002076 add.w r3,r4,r2, lsl #0x2
        let mut t189200 = 0u64;
        let mut t189300 = 0u64;
        let mut t189500 = 0u64;
        let mut t189800 = 0u64;
        let mut t1a0b00 = 0u64;
        t189200 = ((32u64).wrapping_sub(2u64)) & 0xffffffffu64;
        t189300 = (if ((t189200 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(40, 4)) >> (((t189200 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t189500 = (((t189300 >> 0) & 0xffffffffu64) & 1u64) & 0xffffffffu64;
        let value = ((t189500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        t189800 = (if 2u64 >= 32 {
            0
        } else {
            (c.r(40, 4)) << (2u64 as u32)
        }) & 0xffffffffu64;
        t1a0b00 = ((t189800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        let value = ((c.r(48, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(44, 4, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x0000207a vsub.f32 s6,s6,s3
        let value = (f32::from_bits(c.r(792, 4) as u32) - f32::from_bits(c.r(780, 4) as u32))
            .to_bits() as u64;
        c.w(792, 4, value);
    }
    {
        // 0x0000207e vldr.32 s11,[r10]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(72, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x00002082 vmul.f32 s12,s12,s4
        let value = (f32::from_bits(c.r(816, 4) as u32) * f32::from_bits(c.r(784, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00002086 adds r2,#0x1
        let value = ((c.r(40, 4) as u128 + 1u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(40, 4), 1u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(40, 4)).wrapping_add(1u64);
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x00002088 vmov.f32 s4,s13
        let value = c.r(820, 4);
        c.w(784, 4, value);
    }
    {
        // 0x0000208c vsub.f32 s11,s11,s5
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(788, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00002090 cmp r0,r2
        let mut t1a6e00 = 0u64;
        let value = (c.r(40, 4) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), c.r(40, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(32, 4)).wrapping_sub(c.r(40, 4))) & 0xffffffffu64;
        let value = (signed(((t1a6e00 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a6e00 >> 0) & 0xffffffffu64) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x00002092 vfma.f32 s4,s6,s10
        let value = f32::from_bits(c.r(792, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(784, 4) as u32),
            )
            .to_bits() as u64;
        c.w(784, 4, value);
    }
    {
        // 0x00002096 vadd.f32 s13,s13,s4
        let value = (f32::from_bits(c.r(820, 4) as u32) + f32::from_bits(c.r(784, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x0000209a vfma.f32 s13,s11,s10
        let value = f32::from_bits(c.r(812, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(820, 4) as u32),
            )
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x0000209e vadd.f32 s11,s4,s13
        let value = (f32::from_bits(c.r(784, 4) as u32) + f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x000020a2 vfnms.f32 s11,s12,s13
        let mut t49a00 = 0u64;
        let mut t49b00 = 0u64;
        t49a00 = ((-f32::from_bits(c.r(812, 4) as u32)).to_bits() as u64) & 0xffffffffu64;
        t49b00 = ((f32::from_bits(c.r(816, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64)
            & 0xffffffffu64;
        let value = (f32::from_bits(((t49a00 >> 0) & 0xffffffffu64) as u32)
            + f32::from_bits(((t49b00 >> 0) & 0xffffffffu64) as u32))
        .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x000020a6 vmul.f32 s13,s12,s11
        let value = (f32::from_bits(c.r(816, 4) as u32) * f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x000020aa vfma.f32 s13,s6,s10
        let value = f32::from_bits(c.r(792, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(820, 4) as u32),
            )
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x000020ae vmov.f32 s11,s13
        let value = c.r(820, 4);
        c.w(812, 4, value);
    }
    {
        // 0x000020b2 vmov.f32 s13,s5
        let value = c.r(788, 4);
        c.w(820, 4, value);
    }
    {
        // 0x000020b6 vfma.f32 s13,s12,s11
        let value = f32::from_bits(c.r(816, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(812, 4) as u32),
                f32::from_bits(c.r(820, 4) as u32),
            )
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x000020ba vstr.32 s13,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x000020be ble 0x00002000
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x2000u32;
        }
    }
    0x20c0
}
pub fn b_20c0(c: &mut Machine) -> u32 {
    {
        // 0x000020c0 ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x000020c2 str.w r2,[r6,#0xf9c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3996u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x000020c6 cmp r0,r3
        let mut t1a6e00 = 0u64;
        let value = (c.r(44, 4) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), c.r(44, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(32, 4)).wrapping_sub(c.r(44, 4))) & 0xffffffffu64;
        let value = (signed(((t1a6e00 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a6e00 >> 0) & 0xffffffffu64) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x000020c8 ble 0x0000200c
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x200cu32;
        }
    }
    0x20ca
}
pub fn b_20ca(c: &mut Machine) -> u32 {
    {
        // 0x000020ca vldr.32 s12,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x000020ce ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x000020d0 vcmpe.f32 s12,s9
        let mut t35c00 = 0u64;
        let mut t35e00 = 0u64;
        let mut t36000 = 0u64;
        let mut t36100 = 0u64;
        let mut t36200 = 0u64;
        let mut t36400 = 0u64;
        let mut t36500 = 0u64;
        let mut t36600 = 0u64;
        let mut t36700 = 0u64;
        let mut t36800 = 0u64;
        let mut t36900 = 0u64;
        let mut t36a00 = 0u64;
        let mut t36b00 = 0u64;
        let mut t36c00 = 0u64;
        let mut t36d00 = 0u64;
        let mut t36e00 = 0u64;
        let mut t36f00 = 0u64;
        let mut t37000 = 0u64;
        t35c00 = ((f32::from_bits(c.r(816, 4) as u32) < f32::from_bits(c.r(804, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(816, 4) as u32) == f32::from_bits(c.r(804, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(804, 4) as u32) <= f32::from_bits(c.r(816, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(816, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(804, 4) as u32).is_nan() as u64) & 0xffu64;
        t36400 = (((t36100 >> 0) & 0xffu64) | ((t36200 >> 0) & 0xffu64)) & 0xffu64;
        t36500 = (c.r(180, 4) & 268435455u64) & 0xffffffffu64;
        t36600 = ((t35c00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36700 = (if 31u64 >= 32 {
            0
        } else {
            ((t36600 >> 0) & 0xffffffffu64) << (31u64 as u32)
        }) & 0xffffffffu64;
        t36800 =
            (((t36500 >> 0) & 0xffffffffu64) | ((t36700 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36900 = ((t35e00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36a00 = (if 30u64 >= 32 {
            0
        } else {
            ((t36900 >> 0) & 0xffffffffu64) << (30u64 as u32)
        }) & 0xffffffffu64;
        t36b00 =
            (((t36800 >> 0) & 0xffffffffu64) | ((t36a00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36c00 = ((t36000 >> 0) & 0xffu64) & 0xffffffffu64;
        t36d00 = (if 29u64 >= 32 {
            0
        } else {
            ((t36c00 >> 0) & 0xffffffffu64) << (29u64 as u32)
        }) & 0xffffffffu64;
        t36e00 =
            (((t36b00 >> 0) & 0xffffffffu64) | ((t36d00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36f00 = ((t36400 >> 0) & 0xffu64) & 0xffffffffu64;
        t37000 = (if 28u64 >= 32 {
            0
        } else {
            ((t36f00 >> 0) & 0xffffffffu64) << (28u64 as u32)
        }) & 0xffffffffu64;
        let value = (((t36e00 >> 0) & 0xffffffffu64) | ((t37000 >> 0) & 0xffffffffu64));
        c.w(180, 4, value);
    }
    {
        // 0x000020d4 adds r3,#0x1
        let value = ((c.r(44, 4) as u128 + 1u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), 1u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_add(1u64);
        c.w(44, 4, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x000020d6 vmrs apsr,fpscr
        let mut t6bd00 = 0u64;
        let mut t6bf00 = 0u64;
        let mut t6c000 = 0u64;
        let mut t6c200 = 0u64;
        let mut t6c300 = 0u64;
        let mut t6c500 = 0u64;
        let mut t6c600 = 0u64;
        t6bd00 = (if 31u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t6bd00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(96, 1, value);
        t6bf00 = (if 30u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (30u64 as u32)
        }) & 0xffffffffu64;
        t6c000 = (((t6bf00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c000 >> 0) & 0xffu64) & 1u64);
        c.w(97, 1, value);
        t6c200 = (if 29u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (29u64 as u32)
        }) & 0xffffffffu64;
        t6c300 = (((t6c200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c300 >> 0) & 0xffu64) & 1u64);
        c.w(98, 1, value);
        t6c500 = (if 28u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (28u64 as u32)
        }) & 0xffffffffu64;
        t6c600 = (((t6c500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c600 >> 0) & 0xffu64) & 1u64);
        c.w(99, 1, value);
    }
    {
        // 0x000020da str r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x000020dc str.w r3,[r6,#0xfe4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(4068u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x000020e0 bgt 0x0000201a
        let mut t181b00 = 0u64;
        let mut t181c00 = 0u64;
        let mut t181e00 = 0u64;
        t181b00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t181c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t181e00 = (((((t181b00 >> 0) & 0xffu64) != 0) & (((t181c00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t181e00 >> 0) & 0xffu64) != 0 {
            return 0x201au32;
        }
    }
    0x20e2
}
pub fn b_20e2(c: &mut Machine) -> u32 {
    {
        // 0x000020e2 ldrb.w r1,[r6,#0xfd8]
        let mut t19c800 = 0u64;
        let mut t1aa600 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(4056u64)) & 0xffffffffu64;
        t1aa600 = (c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 1)) & 0xffu64;
        let value = ((t1aa600 >> 0) & 0xffu64);
        c.w(36, 4, value);
    }
    {
        // 0x000020e6 ldrd r2,r3,[sp,#0x20]
        let mut t19dc00 = 0u64;
        let mut t1ab800 = 0u64;
        t19dc00 = ((c.r(84, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        let value = c.load(((t19dc00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
        t1ab800 = (((t19dc00 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t1ab800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x000020ea cbz r1,0x00002110
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(36, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x2110u32;
        }
    }
    0x20ec
}
pub fn b_20ec(c: &mut Machine) -> u32 {
    {
        // 0x000020ec ldr r1,[0x000023a0]
        let value = c.load(0x23a0, 4);
        c.w(36, 4, value);
    }
    {
        // 0x000020ee vmov.f32 s10,0x3f000000
        let value = 1056964608u64;
        c.w(808, 4, value);
    }
    {
        // 0x000020f2 vmov.f32 s11,0x3f800000
        let value = 1065353216u64;
        c.w(812, 4, value);
    }
    {
        // 0x000020f6 vldr.32 s13,[r1,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x000020fa vmul.f32 s13,s12,s13
        let value = (f32::from_bits(c.r(816, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x000020fe vmul.f32 s12,s13,s10
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00002102 vcmpe.f32 s12,s9
        let mut t35c00 = 0u64;
        let mut t35e00 = 0u64;
        let mut t36000 = 0u64;
        let mut t36100 = 0u64;
        let mut t36200 = 0u64;
        let mut t36400 = 0u64;
        let mut t36500 = 0u64;
        let mut t36600 = 0u64;
        let mut t36700 = 0u64;
        let mut t36800 = 0u64;
        let mut t36900 = 0u64;
        let mut t36a00 = 0u64;
        let mut t36b00 = 0u64;
        let mut t36c00 = 0u64;
        let mut t36d00 = 0u64;
        let mut t36e00 = 0u64;
        let mut t36f00 = 0u64;
        let mut t37000 = 0u64;
        t35c00 = ((f32::from_bits(c.r(816, 4) as u32) < f32::from_bits(c.r(804, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(816, 4) as u32) == f32::from_bits(c.r(804, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(804, 4) as u32) <= f32::from_bits(c.r(816, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(816, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(804, 4) as u32).is_nan() as u64) & 0xffu64;
        t36400 = (((t36100 >> 0) & 0xffu64) | ((t36200 >> 0) & 0xffu64)) & 0xffu64;
        t36500 = (c.r(180, 4) & 268435455u64) & 0xffffffffu64;
        t36600 = ((t35c00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36700 = (if 31u64 >= 32 {
            0
        } else {
            ((t36600 >> 0) & 0xffffffffu64) << (31u64 as u32)
        }) & 0xffffffffu64;
        t36800 =
            (((t36500 >> 0) & 0xffffffffu64) | ((t36700 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36900 = ((t35e00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36a00 = (if 30u64 >= 32 {
            0
        } else {
            ((t36900 >> 0) & 0xffffffffu64) << (30u64 as u32)
        }) & 0xffffffffu64;
        t36b00 =
            (((t36800 >> 0) & 0xffffffffu64) | ((t36a00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36c00 = ((t36000 >> 0) & 0xffu64) & 0xffffffffu64;
        t36d00 = (if 29u64 >= 32 {
            0
        } else {
            ((t36c00 >> 0) & 0xffffffffu64) << (29u64 as u32)
        }) & 0xffffffffu64;
        t36e00 =
            (((t36b00 >> 0) & 0xffffffffu64) | ((t36d00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36f00 = ((t36400 >> 0) & 0xffu64) & 0xffffffffu64;
        t37000 = (if 28u64 >= 32 {
            0
        } else {
            ((t36f00 >> 0) & 0xffffffffu64) << (28u64 as u32)
        }) & 0xffffffffu64;
        let value = (((t36e00 >> 0) & 0xffffffffu64) | ((t37000 >> 0) & 0xffffffffu64));
        c.w(180, 4, value);
    }
    {
        // 0x00002106 vmrs apsr,fpscr
        let mut t6bd00 = 0u64;
        let mut t6bf00 = 0u64;
        let mut t6c000 = 0u64;
        let mut t6c200 = 0u64;
        let mut t6c300 = 0u64;
        let mut t6c500 = 0u64;
        let mut t6c600 = 0u64;
        t6bd00 = (if 31u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t6bd00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(96, 1, value);
        t6bf00 = (if 30u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (30u64 as u32)
        }) & 0xffffffffu64;
        t6c000 = (((t6bf00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c000 >> 0) & 0xffu64) & 1u64);
        c.w(97, 1, value);
        t6c200 = (if 29u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (29u64 as u32)
        }) & 0xffffffffu64;
        t6c300 = (((t6c200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c300 >> 0) & 0xffu64) & 1u64);
        c.w(98, 1, value);
        t6c500 = (if 28u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (28u64 as u32)
        }) & 0xffffffffu64;
        t6c600 = (((t6c500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c600 >> 0) & 0xffu64) & 1u64);
        c.w(99, 1, value);
    }
    {
        // 0x0000210a it pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x0000210c vmov.pl.f32 s12,s11
        let mut tf00 = 0u64;
        let mut t2700 = 0u64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2700 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x2110u32;
        }
        let value = c.r(812, 4);
        c.w(816, 4, value);
    }
    0x2110
}
pub fn b_2110(c: &mut Machine) -> u32 {
    {
        // 0x00002110 ldr r1,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00002112 cmp r0,r1
        let mut t1a6e00 = 0u64;
        let value = (c.r(36, 4) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), c.r(36, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(32, 4)).wrapping_sub(c.r(36, 4))) & 0xffffffffu64;
        let value = (signed(((t1a6e00 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a6e00 >> 0) & 0xffffffffu64) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x00002114 bgt.w 0x000030fc
        let mut t183c00 = 0u64;
        let mut t183d00 = 0u64;
        let mut t183f00 = 0u64;
        t183c00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t183d00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t183f00 = (((((t183c00 >> 0) & 0xffu64) != 0) & (((t183d00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t183f00 >> 0) & 0xffu64) != 0 {
            return 0x30fcu32;
        }
    }
    0x2118
}
pub fn b_2118(c: &mut Machine) -> u32 {
    {
        // 0x00002118 ldr r1,[0x00002398]
        let value = c.load(0x2398, 4);
        c.w(36, 4, value);
    }
    {
        // 0x0000211a vmov.f32 s8,0x3f000000
        let value = 1056964608u64;
        c.w(800, 4, value);
    }
    {
        // 0x0000211e vldr.32 s11,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x00002122 vldr.32 s9,[r1,#-0x4]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(804, 4, value);
    }
    {
        // 0x00002126 vldr.32 s13,[r1,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x0000212a vsub.f32 s10,s11,s9
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(804, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x0000212e vldr.32 s7,[r1,#-0x8]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x00002132 vsub.f32 s9,s9,s13
        let value = (f32::from_bits(c.r(804, 4) as u32) - f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(804, 4, value);
    }
    {
        // 0x00002136 vsub.f32 s7,s7,s11
        let value = (f32::from_bits(c.r(796, 4) as u32) - f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(796, 4, value);
    }
    {
        // 0x0000213a vmov.f32 s13,s10
        let value = c.r(808, 4);
        c.w(820, 4, value);
    }
    {
        // 0x0000213e vfma.f32 s13,s9,s8
        let value = f32::from_bits(c.r(804, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(800, 4) as u32),
                f32::from_bits(c.r(820, 4) as u32),
            )
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00002142 vadd.f32 s10,s10,s13
        let value = (f32::from_bits(c.r(808, 4) as u32) + f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x00002146 vfma.f32 s10,s7,s8
        let value = f32::from_bits(c.r(796, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(800, 4) as u32),
                f32::from_bits(c.r(808, 4) as u32),
            )
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x0000214a vadd.f32 s13,s13,s10
        let value = (f32::from_bits(c.r(820, 4) as u32) + f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x0000214e vfnms.f32 s13,s10,s12
        let mut t49a00 = 0u64;
        let mut t49b00 = 0u64;
        t49a00 = ((-f32::from_bits(c.r(820, 4) as u32)).to_bits() as u64) & 0xffffffffu64;
        t49b00 = ((f32::from_bits(c.r(808, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64)
            & 0xffffffffu64;
        let value = (f32::from_bits(((t49a00 >> 0) & 0xffffffffu64) as u32)
            + f32::from_bits(((t49b00 >> 0) & 0xffffffffu64) as u32))
        .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00002152 vmul.f32 s13,s12,s13
        let value = (f32::from_bits(c.r(816, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00002156 vfma.f32 s13,s9,s8
        let value = f32::from_bits(c.r(804, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(800, 4) as u32),
                f32::from_bits(c.r(820, 4) as u32),
            )
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x0000215a vfma.f32 s11,s12,s13
        let value = f32::from_bits(c.r(816, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(820, 4) as u32),
                f32::from_bits(c.r(812, 4) as u32),
            )
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x0000215e vadd.f32 s13,s11,s23
        let value = (f32::from_bits(c.r(812, 4) as u32) + f32::from_bits(c.r(860, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    0x2162
}
pub fn b_2162(c: &mut Machine) -> u32 {
    {
        // 0x00002162 vldr.32 s12,[r5,#0xa0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(160u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x00002166 vmov.f32 s9,0x3f800000
        let value = 1065353216u64;
        c.w(804, 4, value);
    }
    {
        // 0x0000216a vldr.32 s25,[r5,#0x70]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(112u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(868, 4, value);
    }
    {
        // 0x0000216e vmov.f32 s10,0x3f000000
        let value = 1056964608u64;
        c.w(808, 4, value);
    }
    {
        // 0x00002172 ldr r1,[r5,#0x7c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(124u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00002174 mov.w r10,#0x0
        let mut t187d00 = 0u64;
        t187d00 = (0u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = ((t187d00 >> 0) & 0xffffffffu64);
        c.w(72, 4, value);
        let value = (signed(c.r(72, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(72, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00002178 vfms.f32 s25,s12,s15
        let value = (-f32::from_bits(c.r(816, 4) as u32))
            .fast_mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(868, 4) as u32),
            )
            .to_bits() as u64;
        c.w(868, 4, value);
    }
    {
        // 0x0000217c vldr.32 s11,[r5,#0x68]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(104u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x00002180 vstr.32 s13,[r11,#-0x30]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(76, 4)).wrapping_sub(48u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x00002184 mov.w r9,#0x0
        let mut t187d00 = 0u64;
        t187d00 = (0u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = ((t187d00 >> 0) & 0xffffffffu64);
        c.w(68, 4, value);
        let value = (signed(c.r(68, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(68, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00002188 vldr.32 s13,[r5,#0x94]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(148u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x0000218c str.w r1,[r5,#0x80]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(128u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x00002190 ldr r1,[r5,#0x78]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(120u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00002192 vadd.f32 s13,s13,s11
        let value = (f32::from_bits(c.r(820, 4) as u32) + f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00002196 vldr.32 s8,[pc,#0x20c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((8598u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(524u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x0000219a vfma.f32 s15,s12,s25
        let value = f32::from_bits(c.r(816, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(868, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x0000219e str r1,[r5,#0x7c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(124u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x000021a0 ldr r1,[r5,#0x74]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(116u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x000021a2 ldr r4,[r5,#0x50]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(80u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x000021a4 ldr.w r8,[r5,#0x58]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(88u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(64, 4, value);
    }
    {
        // 0x000021a8 ldr r7,[r5,#0x60]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(96u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(60, 4, value);
    }
    {
        // 0x000021aa ldr.w lr,[r5,#0xa4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(164u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(88, 4, value);
    }
    {
        // 0x000021ae vstr.32 s13,[r5,#0x94]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(148u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x000021b2 vstr.32 s25,[sp,#0x40]
        let mut t64000 = 0u64;
        t64000 = ((c.r(84, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(868, 4));
    }
    {
        // 0x000021b6 str r1,[r5,#0x78]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(120u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x000021b8 vstr.32 s15,[r5,#0x74]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(116u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x000021bc b 0x000021ca
        return 0x21cau32;
    }
    0x21be
}
pub fn b_21be(c: &mut Machine) -> u32 {
    {
        // 0x000021be cmp r4,lr
        let mut t1a7000 = 0u64;
        let value = (c.r(88, 4) <= c.r(48, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), c.r(88, 4), 4, true);
        c.w(103, 1, value);
        t1a7000 = ((c.r(48, 4)).wrapping_sub(c.r(88, 4))) & 0xffffffffu64;
        let value = (signed(((t1a7000 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a7000 >> 0) & 0xffffffffu64) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x000021c0 str.w r9,[r5,#0x5c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(92u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(68, 4));
    }
    {
        // 0x000021c4 bgt 0x00002276
        let mut t181b00 = 0u64;
        let mut t181c00 = 0u64;
        let mut t181e00 = 0u64;
        t181b00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t181c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t181e00 = (((((t181b00 >> 0) & 0xffu64) != 0) & (((t181c00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t181e00 >> 0) & 0xffu64) != 0 {
            return 0x2276u32;
        }
    }
    0x21c6
}
pub fn b_21c6(c: &mut Machine) -> u32 {
    {
        // 0x000021c6 vldr.32 s13,[r5,#0x94]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(148u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    0x21ca
}
pub fn b_21ca(c: &mut Machine) -> u32 {
    {
        // 0x000021ca vcmpe.f32 s13,s9
        let mut t35c00 = 0u64;
        let mut t35e00 = 0u64;
        let mut t36000 = 0u64;
        let mut t36100 = 0u64;
        let mut t36200 = 0u64;
        let mut t36400 = 0u64;
        let mut t36500 = 0u64;
        let mut t36600 = 0u64;
        let mut t36700 = 0u64;
        let mut t36800 = 0u64;
        let mut t36900 = 0u64;
        let mut t36a00 = 0u64;
        let mut t36b00 = 0u64;
        let mut t36c00 = 0u64;
        let mut t36d00 = 0u64;
        let mut t36e00 = 0u64;
        let mut t36f00 = 0u64;
        let mut t37000 = 0u64;
        t35c00 = ((f32::from_bits(c.r(820, 4) as u32) < f32::from_bits(c.r(804, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(820, 4) as u32) == f32::from_bits(c.r(804, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(804, 4) as u32) <= f32::from_bits(c.r(820, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(820, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(804, 4) as u32).is_nan() as u64) & 0xffu64;
        t36400 = (((t36100 >> 0) & 0xffu64) | ((t36200 >> 0) & 0xffu64)) & 0xffu64;
        t36500 = (c.r(180, 4) & 268435455u64) & 0xffffffffu64;
        t36600 = ((t35c00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36700 = (if 31u64 >= 32 {
            0
        } else {
            ((t36600 >> 0) & 0xffffffffu64) << (31u64 as u32)
        }) & 0xffffffffu64;
        t36800 =
            (((t36500 >> 0) & 0xffffffffu64) | ((t36700 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36900 = ((t35e00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36a00 = (if 30u64 >= 32 {
            0
        } else {
            ((t36900 >> 0) & 0xffffffffu64) << (30u64 as u32)
        }) & 0xffffffffu64;
        t36b00 =
            (((t36800 >> 0) & 0xffffffffu64) | ((t36a00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36c00 = ((t36000 >> 0) & 0xffu64) & 0xffffffffu64;
        t36d00 = (if 29u64 >= 32 {
            0
        } else {
            ((t36c00 >> 0) & 0xffffffffu64) << (29u64 as u32)
        }) & 0xffffffffu64;
        t36e00 =
            (((t36b00 >> 0) & 0xffffffffu64) | ((t36d00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36f00 = ((t36400 >> 0) & 0xffu64) & 0xffffffffu64;
        t37000 = (if 28u64 >= 32 {
            0
        } else {
            ((t36f00 >> 0) & 0xffffffffu64) << (28u64 as u32)
        }) & 0xffffffffu64;
        let value = (((t36e00 >> 0) & 0xffffffffu64) | ((t37000 >> 0) & 0xffffffffu64));
        c.w(180, 4, value);
    }
    {
        // 0x000021ce vmrs apsr,fpscr
        let mut t6bd00 = 0u64;
        let mut t6bf00 = 0u64;
        let mut t6c000 = 0u64;
        let mut t6c200 = 0u64;
        let mut t6c300 = 0u64;
        let mut t6c500 = 0u64;
        let mut t6c600 = 0u64;
        t6bd00 = (if 31u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t6bd00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(96, 1, value);
        t6bf00 = (if 30u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (30u64 as u32)
        }) & 0xffffffffu64;
        t6c000 = (((t6bf00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c000 >> 0) & 0xffu64) & 1u64);
        c.w(97, 1, value);
        t6c200 = (if 29u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (29u64 as u32)
        }) & 0xffffffffu64;
        t6c300 = (((t6c200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c300 >> 0) & 0xffu64) & 1u64);
        c.w(98, 1, value);
        t6c500 = (if 28u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (28u64 as u32)
        }) & 0xffffffffu64;
        t6c600 = (((t6c500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c600 >> 0) & 0xffu64) & 1u64);
        c.w(99, 1, value);
    }
    {
        // 0x000021d2 ble 0x0000228c
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x228cu32;
        }
    }
    0x21d4
}
pub fn b_21d4(c: &mut Machine) -> u32 {
    {
        // 0x000021d4 ldr.w r1,[r5,#0x8c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(140u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x000021d8 vsub.f32 s13,s13,s9
        let value = (f32::from_bits(c.r(820, 4) as u32) - f32::from_bits(c.r(804, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x000021dc vldr.32 s15,[r5,#0x9c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(156u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x000021e0 str.w r1,[r5,#0x90]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(144u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x000021e4 vcmpe.f32 s15,#0
        let mut t35c00 = 0u64;
        let mut t35e00 = 0u64;
        let mut t36000 = 0u64;
        let mut t36100 = 0u64;
        let mut t36200 = 0u64;
        let mut t36400 = 0u64;
        let mut t36500 = 0u64;
        let mut t36600 = 0u64;
        let mut t36700 = 0u64;
        let mut t36800 = 0u64;
        let mut t36900 = 0u64;
        let mut t36a00 = 0u64;
        let mut t36b00 = 0u64;
        let mut t36c00 = 0u64;
        let mut t36d00 = 0u64;
        let mut t36e00 = 0u64;
        let mut t36f00 = 0u64;
        let mut t37000 = 0u64;
        let mut t37900 = 0u64;
        t37900 = (0u64) & 0xffffffffu64;
        t35c00 = ((f32::from_bits(c.r(828, 4) as u32)
            < f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(828, 4) as u32)
            == f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)) as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)
            <= f32::from_bits(c.r(828, 4) as u32)) as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32).is_nan() as u64) & 0xffu64;
        t36400 = (((t36100 >> 0) & 0xffu64) | ((t36200 >> 0) & 0xffu64)) & 0xffu64;
        t36500 = (c.r(180, 4) & 268435455u64) & 0xffffffffu64;
        t36600 = ((t35c00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36700 = (if 31u64 >= 32 {
            0
        } else {
            ((t36600 >> 0) & 0xffffffffu64) << (31u64 as u32)
        }) & 0xffffffffu64;
        t36800 =
            (((t36500 >> 0) & 0xffffffffu64) | ((t36700 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36900 = ((t35e00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36a00 = (if 30u64 >= 32 {
            0
        } else {
            ((t36900 >> 0) & 0xffffffffu64) << (30u64 as u32)
        }) & 0xffffffffu64;
        t36b00 =
            (((t36800 >> 0) & 0xffffffffu64) | ((t36a00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36c00 = ((t36000 >> 0) & 0xffu64) & 0xffffffffu64;
        t36d00 = (if 29u64 >= 32 {
            0
        } else {
            ((t36c00 >> 0) & 0xffffffffu64) << (29u64 as u32)
        }) & 0xffffffffu64;
        t36e00 =
            (((t36b00 >> 0) & 0xffffffffu64) | ((t36d00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36f00 = ((t36400 >> 0) & 0xffu64) & 0xffffffffu64;
        t37000 = (if 28u64 >= 32 {
            0
        } else {
            ((t36f00 >> 0) & 0xffffffffu64) << (28u64 as u32)
        }) & 0xffffffffu64;
        let value = (((t36e00 >> 0) & 0xffffffffu64) | ((t37000 >> 0) & 0xffffffffu64));
        c.w(180, 4, value);
    }
    {
        // 0x000021e8 ldr.w r1,[r5,#0x88]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(136u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x000021ec vstr.32 s13,[r5,#0x94]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(148u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x000021f0 str.w r1,[r5,#0x8c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(140u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x000021f4 ldr.w r1,[r5,#0x84]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(132u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x000021f8 vmrs apsr,fpscr
        let mut t6bd00 = 0u64;
        let mut t6bf00 = 0u64;
        let mut t6c000 = 0u64;
        let mut t6c200 = 0u64;
        let mut t6c300 = 0u64;
        let mut t6c500 = 0u64;
        let mut t6c600 = 0u64;
        t6bd00 = (if 31u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t6bd00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(96, 1, value);
        t6bf00 = (if 30u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (30u64 as u32)
        }) & 0xffffffffu64;
        t6c000 = (((t6bf00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c000 >> 0) & 0xffu64) & 1u64);
        c.w(97, 1, value);
        t6c200 = (if 29u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (29u64 as u32)
        }) & 0xffffffffu64;
        t6c300 = (((t6c200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c300 >> 0) & 0xffu64) & 1u64);
        c.w(98, 1, value);
        t6c500 = (if 28u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (28u64 as u32)
        }) & 0xffffffffu64;
        t6c600 = (((t6c500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c600 >> 0) & 0xffu64) & 1u64);
        c.w(99, 1, value);
    }
    {
        // 0x000021fc str.w r1,[r5,#0x88]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(136u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x00002200 ldr r1,[r5,#0x5c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(92u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00002202 sub.w r0,r1,r3
        let mut t1fe000 = 0u64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        t1fe000 = (c.r(44, 4)) & 0xffffffffu64;
        let value = (((t1fe000 >> 0) & 0xffffffffu64) <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), ((t1fe000 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(((t1fe000 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00002206 bne.w 0x000026e2
        let mut t182500 = 0u64;
        t182500 = ((c.r(97, 1) == 0u64) as u64) & 0xffu64;
        if ((t182500 >> 0) & 0xffu64) != 0 {
            return 0x26e2u32;
        }
    }
    0x220a
}
pub fn b_220a(c: &mut Machine) -> u32 {
    {
        // 0x0000220a cmp r0,#0x0
        let mut t1a6800 = 0u64;
        let value = (0u64 <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 0u64, 4, true);
        c.w(103, 1, value);
        t1a6800 = ((c.r(32, 4)).wrapping_sub(0u64)) & 0xffffffffu64;
        let value = (signed(((t1a6800 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a6800 >> 0) & 0xffffffffu64) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x0000220c it lt
        let mut t185c00 = 0u64;
        t185c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
    }
    {
        // 0x0000220e add.lt r0,r0,r4
        let mut t1d00 = 0u64;
        let mut t2700 = 0u64;
        t1d00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2700 = ((((t1d00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x2210u32;
        }
        let value = ((c.r(32, 4) as u128 + c.r(48, 4) as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), c.r(48, 4), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(c.r(48, 4));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    0x2210
}
pub fn b_2210(c: &mut Machine) -> u32 {
    {
        // 0x00002210 add.w r0,r7,r0, lsl #0x2
        let mut t189200 = 0u64;
        let mut t189300 = 0u64;
        let mut t189500 = 0u64;
        let mut t189800 = 0u64;
        let mut t1a0b00 = 0u64;
        t189200 = ((32u64).wrapping_sub(2u64)) & 0xffffffffu64;
        t189300 = (if ((t189200 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(32, 4)) >> (((t189200 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t189500 = (((t189300 >> 0) & 0xffffffffu64) & 1u64) & 0xffffffffu64;
        let value = ((t189500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        t189800 = (if 2u64 >= 32 {
            0
        } else {
            (c.r(32, 4)) << (2u64 as u32)
        }) & 0xffffffffu64;
        t1a0b00 = ((t189800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        let value = ((c.r(60, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(60, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(60, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00002214 ldr r0,[r0,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(32, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x00002216 str.w r0,[r5,#0x84]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(132u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    0x221a
}
pub fn b_221a(c: &mut Machine) -> u32 {
    {
        // 0x0000221a vldr.32 s12,[r5,#0x78]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(120u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x0000221e add.w r0,r7,r1, lsl #0x2
        let mut t189200 = 0u64;
        let mut t189300 = 0u64;
        let mut t189500 = 0u64;
        let mut t189800 = 0u64;
        let mut t1a0b00 = 0u64;
        t189200 = ((32u64).wrapping_sub(2u64)) & 0xffffffffu64;
        t189300 = (if ((t189200 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(36, 4)) >> (((t189200 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t189500 = (((t189300 >> 0) & 0xffffffffu64) & 1u64) & 0xffffffffu64;
        let value = ((t189500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        t189800 = (if 2u64 >= 32 {
            0
        } else {
            (c.r(36, 4)) << (2u64 as u32)
        }) & 0xffffffffu64;
        t1a0b00 = ((t189800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        let value = ((c.r(60, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(60, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(60, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00002222 vldr.32 s7,[r5,#0x7c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(124u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x00002226 adds r1,#0x1
        let value = ((c.r(36, 4) as u128 + 1u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 1u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_add(1u64);
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x00002228 vldr.32 s6,[r5,#0x74]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(116u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x0000222c vsub.f32 s5,s12,s7
        let value = (f32::from_bits(c.r(816, 4) as u32) - f32::from_bits(c.r(796, 4) as u32))
            .to_bits() as u64;
        c.w(788, 4, value);
    }
    {
        // 0x00002230 vldr.32 s15,[r5,#0x6c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(108u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00002234 vsub.f32 s7,s7,s6
        let value = (f32::from_bits(c.r(796, 4) as u32) - f32::from_bits(c.r(792, 4) as u32))
            .to_bits() as u64;
        c.w(796, 4, value);
    }
    {
        // 0x00002238 vldr.32 s11,[r5,#0x80]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(128u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x0000223c vmul.f32 s13,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00002240 cmp r4,r1
        let mut t1a6e00 = 0u64;
        let value = (c.r(36, 4) <= c.r(48, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), c.r(36, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(48, 4)).wrapping_sub(c.r(36, 4))) & 0xffffffffu64;
        let value = (signed(((t1a6e00 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a6e00 >> 0) & 0xffffffffu64) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x00002242 vmov.f32 s15,s5
        let value = c.r(788, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00002246 vsub.f32 s11,s11,s12
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x0000224a vfma.f32 s15,s7,s10
        let value = f32::from_bits(c.r(796, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x0000224e vadd.f32 s6,s5,s15
        let value = (f32::from_bits(c.r(788, 4) as u32) + f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(792, 4, value);
    }
    {
        // 0x00002252 vfma.f32 s6,s11,s10
        let value = f32::from_bits(c.r(812, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(792, 4) as u32),
            )
            .to_bits() as u64;
        c.w(792, 4, value);
    }
    {
        // 0x00002256 vadd.f32 s15,s15,s6
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(792, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x0000225a vfnms.f32 s15,s13,s6
        let mut t49a00 = 0u64;
        let mut t49b00 = 0u64;
        t49a00 = ((-f32::from_bits(c.r(828, 4) as u32)).to_bits() as u64) & 0xffffffffu64;
        t49b00 = ((f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(792, 4) as u32))
            .to_bits() as u64)
            & 0xffffffffu64;
        let value = (f32::from_bits(((t49a00 >> 0) & 0xffffffffu64) as u32)
            + f32::from_bits(((t49b00 >> 0) & 0xffffffffu64) as u32))
        .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x0000225e vmul.f32 s15,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002262 vfma.f32 s15,s7,s10
        let value = f32::from_bits(c.r(796, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002266 vfma.f32 s12,s13,s15
        let value = f32::from_bits(c.r(820, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(816, 4) as u32),
            )
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x0000226a vstr.32 s12,[r0]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(32, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x0000226e ble 0x000021be
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x21beu32;
        }
    }
    0x2270
}
pub fn b_2270(c: &mut Machine) -> u32 {
    {
        // 0x00002270 cmp r4,lr
        let mut t1a7000 = 0u64;
        let value = (c.r(88, 4) <= c.r(48, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), c.r(88, 4), 4, true);
        c.w(103, 1, value);
        t1a7000 = ((c.r(48, 4)).wrapping_sub(c.r(88, 4))) & 0xffffffffu64;
        let value = (signed(((t1a7000 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a7000 >> 0) & 0xffffffffu64) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x00002272 str r1,[r5,#0x5c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(92u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x00002274 ble 0x000021c6
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x21c6u32;
        }
    }
    0x2276
}
pub fn b_2276(c: &mut Machine) -> u32 {
    {
        // 0x00002276 vldr.32 s13,[r5,#0x94]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(148u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x0000227a add.w lr,lr,#0x1
        let mut t187d00 = 0u64;
        t187d00 = (1u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = ((c.r(88, 4) as u128 + ((t187d00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(88, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(88, 4)).wrapping_add(((t187d00 >> 0) & 0xffffffffu64));
        c.w(88, 4, value);
        let value = (signed(c.r(88, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(88, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x0000227e vcmpe.f32 s13,s9
        let mut t35c00 = 0u64;
        let mut t35e00 = 0u64;
        let mut t36000 = 0u64;
        let mut t36100 = 0u64;
        let mut t36200 = 0u64;
        let mut t36400 = 0u64;
        let mut t36500 = 0u64;
        let mut t36600 = 0u64;
        let mut t36700 = 0u64;
        let mut t36800 = 0u64;
        let mut t36900 = 0u64;
        let mut t36a00 = 0u64;
        let mut t36b00 = 0u64;
        let mut t36c00 = 0u64;
        let mut t36d00 = 0u64;
        let mut t36e00 = 0u64;
        let mut t36f00 = 0u64;
        let mut t37000 = 0u64;
        t35c00 = ((f32::from_bits(c.r(820, 4) as u32) < f32::from_bits(c.r(804, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(820, 4) as u32) == f32::from_bits(c.r(804, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(804, 4) as u32) <= f32::from_bits(c.r(820, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(820, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(804, 4) as u32).is_nan() as u64) & 0xffu64;
        t36400 = (((t36100 >> 0) & 0xffu64) | ((t36200 >> 0) & 0xffu64)) & 0xffu64;
        t36500 = (c.r(180, 4) & 268435455u64) & 0xffffffffu64;
        t36600 = ((t35c00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36700 = (if 31u64 >= 32 {
            0
        } else {
            ((t36600 >> 0) & 0xffffffffu64) << (31u64 as u32)
        }) & 0xffffffffu64;
        t36800 =
            (((t36500 >> 0) & 0xffffffffu64) | ((t36700 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36900 = ((t35e00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36a00 = (if 30u64 >= 32 {
            0
        } else {
            ((t36900 >> 0) & 0xffffffffu64) << (30u64 as u32)
        }) & 0xffffffffu64;
        t36b00 =
            (((t36800 >> 0) & 0xffffffffu64) | ((t36a00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36c00 = ((t36000 >> 0) & 0xffu64) & 0xffffffffu64;
        t36d00 = (if 29u64 >= 32 {
            0
        } else {
            ((t36c00 >> 0) & 0xffffffffu64) << (29u64 as u32)
        }) & 0xffffffffu64;
        t36e00 =
            (((t36b00 >> 0) & 0xffffffffu64) | ((t36d00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36f00 = ((t36400 >> 0) & 0xffu64) & 0xffffffffu64;
        t37000 = (if 28u64 >= 32 {
            0
        } else {
            ((t36f00 >> 0) & 0xffffffffu64) << (28u64 as u32)
        }) & 0xffffffffu64;
        let value = (((t36e00 >> 0) & 0xffffffffu64) | ((t37000 >> 0) & 0xffffffffu64));
        c.w(180, 4, value);
    }
    {
        // 0x00002282 str.w lr,[r5,#0xa4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(164u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(88, 4));
    }
    {
        // 0x00002286 vmrs apsr,fpscr
        let mut t6bd00 = 0u64;
        let mut t6bf00 = 0u64;
        let mut t6c000 = 0u64;
        let mut t6c200 = 0u64;
        let mut t6c300 = 0u64;
        let mut t6c500 = 0u64;
        let mut t6c600 = 0u64;
        t6bd00 = (if 31u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t6bd00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(96, 1, value);
        t6bf00 = (if 30u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (30u64 as u32)
        }) & 0xffffffffu64;
        t6c000 = (((t6bf00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c000 >> 0) & 0xffu64) & 1u64);
        c.w(97, 1, value);
        t6c200 = (if 29u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (29u64 as u32)
        }) & 0xffffffffu64;
        t6c300 = (((t6c200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c300 >> 0) & 0xffu64) & 1u64);
        c.w(98, 1, value);
        t6c500 = (if 28u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (28u64 as u32)
        }) & 0xffffffffu64;
        t6c600 = (((t6c500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c600 >> 0) & 0xffu64) & 1u64);
        c.w(99, 1, value);
    }
    {
        // 0x0000228a bgt 0x000021d4
        let mut t181b00 = 0u64;
        let mut t181c00 = 0u64;
        let mut t181e00 = 0u64;
        t181b00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t181c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t181e00 = (((((t181b00 >> 0) & 0xffu64) != 0) & (((t181c00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t181e00 >> 0) & 0xffu64) != 0 {
            return 0x21d4u32;
        }
    }
    0x228c
}
pub fn b_228c(c: &mut Machine) -> u32 {
    {
        // 0x0000228c ldrb.w r3,[r5,#0x98]
        let mut t19c800 = 0u64;
        let mut t1aa600 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(152u64)) & 0xffffffffu64;
        t1aa600 = (c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 1)) & 0xffu64;
        let value = ((t1aa600 >> 0) & 0xffu64);
        c.w(44, 4, value);
    }
    {
        // 0x00002290 cbz r3,0x000022b4
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(44, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x22b4u32;
        }
    }
    0x2292
}
pub fn b_2292(c: &mut Machine) -> u32 {
    {
        // 0x00002292 vldr.32 s15,[r5,#0x6c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(108u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00002296 vmov.f32 s11,0x3f000000
        let value = 1056964608u64;
        c.w(812, 4, value);
    }
    {
        // 0x0000229a vmov.f32 s12,0x3f800000
        let value = 1065353216u64;
        c.w(816, 4, value);
    }
    {
        // 0x0000229e vmul.f32 s15,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x000022a2 vmul.f32 s13,s15,s11
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x000022a6 vcmpe.f32 s13,s9
        let mut t35c00 = 0u64;
        let mut t35e00 = 0u64;
        let mut t36000 = 0u64;
        let mut t36100 = 0u64;
        let mut t36200 = 0u64;
        let mut t36400 = 0u64;
        let mut t36500 = 0u64;
        let mut t36600 = 0u64;
        let mut t36700 = 0u64;
        let mut t36800 = 0u64;
        let mut t36900 = 0u64;
        let mut t36a00 = 0u64;
        let mut t36b00 = 0u64;
        let mut t36c00 = 0u64;
        let mut t36d00 = 0u64;
        let mut t36e00 = 0u64;
        let mut t36f00 = 0u64;
        let mut t37000 = 0u64;
        t35c00 = ((f32::from_bits(c.r(820, 4) as u32) < f32::from_bits(c.r(804, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(820, 4) as u32) == f32::from_bits(c.r(804, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(804, 4) as u32) <= f32::from_bits(c.r(820, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(820, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(804, 4) as u32).is_nan() as u64) & 0xffu64;
        t36400 = (((t36100 >> 0) & 0xffu64) | ((t36200 >> 0) & 0xffu64)) & 0xffu64;
        t36500 = (c.r(180, 4) & 268435455u64) & 0xffffffffu64;
        t36600 = ((t35c00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36700 = (if 31u64 >= 32 {
            0
        } else {
            ((t36600 >> 0) & 0xffffffffu64) << (31u64 as u32)
        }) & 0xffffffffu64;
        t36800 =
            (((t36500 >> 0) & 0xffffffffu64) | ((t36700 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36900 = ((t35e00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36a00 = (if 30u64 >= 32 {
            0
        } else {
            ((t36900 >> 0) & 0xffffffffu64) << (30u64 as u32)
        }) & 0xffffffffu64;
        t36b00 =
            (((t36800 >> 0) & 0xffffffffu64) | ((t36a00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36c00 = ((t36000 >> 0) & 0xffu64) & 0xffffffffu64;
        t36d00 = (if 29u64 >= 32 {
            0
        } else {
            ((t36c00 >> 0) & 0xffffffffu64) << (29u64 as u32)
        }) & 0xffffffffu64;
        t36e00 =
            (((t36b00 >> 0) & 0xffffffffu64) | ((t36d00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36f00 = ((t36400 >> 0) & 0xffu64) & 0xffffffffu64;
        t37000 = (if 28u64 >= 32 {
            0
        } else {
            ((t36f00 >> 0) & 0xffffffffu64) << (28u64 as u32)
        }) & 0xffffffffu64;
        let value = (((t36e00 >> 0) & 0xffffffffu64) | ((t37000 >> 0) & 0xffffffffu64));
        c.w(180, 4, value);
    }
    {
        // 0x000022aa vmrs apsr,fpscr
        let mut t6bd00 = 0u64;
        let mut t6bf00 = 0u64;
        let mut t6c000 = 0u64;
        let mut t6c200 = 0u64;
        let mut t6c300 = 0u64;
        let mut t6c500 = 0u64;
        let mut t6c600 = 0u64;
        t6bd00 = (if 31u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t6bd00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(96, 1, value);
        t6bf00 = (if 30u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (30u64 as u32)
        }) & 0xffffffffu64;
        t6c000 = (((t6bf00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c000 >> 0) & 0xffu64) & 1u64);
        c.w(97, 1, value);
        t6c200 = (if 29u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (29u64 as u32)
        }) & 0xffffffffu64;
        t6c300 = (((t6c200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c300 >> 0) & 0xffu64) & 1u64);
        c.w(98, 1, value);
        t6c500 = (if 28u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (28u64 as u32)
        }) & 0xffffffffu64;
        t6c600 = (((t6c500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c600 >> 0) & 0xffu64) & 1u64);
        c.w(99, 1, value);
    }
    {
        // 0x000022ae it pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x000022b0 vmov.pl.f32 s13,s12
        let mut tf00 = 0u64;
        let mut t2700 = 0u64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2700 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x22b4u32;
        }
        let value = c.r(816, 4);
        c.w(820, 4, value);
    }
    0x22b4
}
pub fn b_22b4(c: &mut Machine) -> u32 {
    {
        // 0x000022b4 cmp r4,lr
        let mut t1a7000 = 0u64;
        let value = (c.r(88, 4) <= c.r(48, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), c.r(88, 4), 4, true);
        c.w(103, 1, value);
        t1a7000 = ((c.r(48, 4)).wrapping_sub(c.r(88, 4))) & 0xffffffffu64;
        let value = (signed(((t1a7000 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a7000 >> 0) & 0xffffffffu64) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x000022b6 bgt.w 0x00003104
        let mut t183c00 = 0u64;
        let mut t183d00 = 0u64;
        let mut t183f00 = 0u64;
        t183c00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t183d00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t183f00 = (((((t183c00 >> 0) & 0xffu64) != 0) & (((t183d00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t183f00 >> 0) & 0xffu64) != 0 {
            return 0x3104u32;
        }
    }
    0x22ba
}
pub fn b_22ba(c: &mut Machine) -> u32 {
    {
        // 0x000022ba vldr.32 s12,[r5,#0x8c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(140u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x000022be vmov.f32 s9,0x3f000000
        let value = 1056964608u64;
        c.w(804, 4, value);
    }
    {
        // 0x000022c2 vldr.32 s10,[r5,#0x88]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(136u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x000022c6 vldr.32 s15,[r5,#0x90]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(144u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x000022ca vsub.f32 s11,s12,s10
        let value = (f32::from_bits(c.r(816, 4) as u32) - f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x000022ce vldr.32 s8,[r5,#0x84]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(132u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x000022d2 vsub.f32 s10,s10,s15
        let value = (f32::from_bits(c.r(808, 4) as u32) - f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x000022d6 vsub.f32 s8,s8,s12
        let value = (f32::from_bits(c.r(800, 4) as u32) - f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(800, 4, value);
    }
    {
        // 0x000022da vmov.f32 s15,s11
        let value = c.r(812, 4);
        c.w(828, 4, value);
    }
    {
        // 0x000022de vfma.f32 s15,s10,s9
        let value = f32::from_bits(c.r(808, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x000022e2 vadd.f32 s11,s11,s15
        let value = (f32::from_bits(c.r(812, 4) as u32) + f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x000022e6 vfma.f32 s11,s8,s9
        let value = f32::from_bits(c.r(800, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(812, 4) as u32),
            )
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x000022ea vadd.f32 s15,s15,s11
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x000022ee vfnms.f32 s15,s11,s13
        let mut t49a00 = 0u64;
        let mut t49b00 = 0u64;
        t49a00 = ((-f32::from_bits(c.r(828, 4) as u32)).to_bits() as u64) & 0xffffffffu64;
        t49b00 = ((f32::from_bits(c.r(812, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64)
            & 0xffffffffu64;
        let value = (f32::from_bits(((t49a00 >> 0) & 0xffffffffu64) as u32)
            + f32::from_bits(((t49b00 >> 0) & 0xffffffffu64) as u32))
        .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x000022f2 vmul.f32 s15,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x000022f6 vfma.f32 s15,s10,s9
        let value = f32::from_bits(c.r(808, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x000022fa vfma.f32 s12,s13,s15
        let value = f32::from_bits(c.r(820, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(816, 4) as u32),
            )
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x000022fe vadd.f32 s13,s12,s23
        let value = (f32::from_bits(c.r(816, 4) as u32) + f32::from_bits(c.r(860, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    0x2302
}
pub fn b_2302(c: &mut Machine) -> u32 {
    {
        // 0x00002302 vldr.32 s15,[r5,#0x32c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(812u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00002306 vmov.f32 s9,0x3f800000
        let value = 1065353216u64;
        c.w(804, 4, value);
    }
    {
        // 0x0000230a ldr r3,[0x000023a8]
        let value = c.load(0x23a8, 4);
        c.w(44, 4, value);
    }
    {
        // 0x0000230c vmov.f32 s10,0x3f000000
        let value = 1056964608u64;
        c.w(808, 4, value);
    }
    {
        // 0x00002310 vadd.f32 s15,s15,s23
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(860, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002314 vstr.32 s13,[r5,#0x70]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(112u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x00002318 vldr.32 s12,[r3,#0xc]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(12u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x0000231c add.w r11,r3,#0x3c
        let mut t187d00 = 0u64;
        t187d00 = (60u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = ((c.r(44, 4) as u128 + ((t187d00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_add(((t187d00 >> 0) & 0xffffffffu64));
        c.w(76, 4, value);
        let value = (signed(c.r(76, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(76, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00002320 vldr.32 s11,[r3,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x00002324 vmul.f32 s15,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002328 vldr.32 s14,[r11]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(76, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x0000232c ldr.w r3,[r11,#-0x24]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(76, 4)).wrapping_sub(36u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00002330 sub.w r10,r11,#0x20
        let mut t187d00 = 0u64;
        t187d00 = (32u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(76, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(76, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(76, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(72, 4, value);
        let value = (signed(c.r(72, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(72, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00002334 vldr.32 s13,[r11,#-0xc]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(76, 4)).wrapping_sub(12u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00002338 sub.w r9,r11,#0x24
        let mut t187d00 = 0u64;
        t187d00 = (36u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(76, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(76, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(76, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(68, 4, value);
        let value = (signed(c.r(68, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(68, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x0000233c vfma.f32 s15,s28,s26
        let value = f32::from_bits(c.r(880, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(872, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002340 str.w r3,[r11,#-0x20]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(76, 4)).wrapping_sub(32u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00002344 ldr.w r3,[r11,#-0x28]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(76, 4)).wrapping_sub(40u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00002348 vadd.f32 s13,s13,s11
        let value = (f32::from_bits(c.r(820, 4) as u32) + f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x0000234c vldr.32 s8,[pc,#0x54]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((9036u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(84u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x00002350 sub.w r8,r11,#0x28
        let mut t187d00 = 0u64;
        t187d00 = (40u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(76, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(76, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(76, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(64, 4, value);
        let value = (signed(c.r(64, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(64, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00002354 str.w r3,[r11,#-0x24]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(76, 4)).wrapping_sub(36u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00002358 sub.w lr,r11,#0x2c
        let mut t187d00 = 0u64;
        t187d00 = (44u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(76, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(76, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(76, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(88, 4, value);
        let value = (signed(c.r(88, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(88, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x0000235c ldr.w r3,[r11,#-0x2c]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(76, 4)).wrapping_sub(44u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00002360 sub.w r12,r11,#0xc
        let mut t187d00 = 0u64;
        t187d00 = (12u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(76, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(76, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(76, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(80, 4, value);
        let value = (signed(c.r(80, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(80, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00002364 ldr.w r0,[r6,#0xf30]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3888u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x00002368 vfms.f32 s12,s15,s14
        let value = (-f32::from_bits(c.r(828, 4) as u32))
            .fast_mul_add(
                f32::from_bits(c.r(824, 4) as u32),
                f32::from_bits(c.r(816, 4) as u32),
            )
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x0000236c str.w r3,[r11,#-0x28]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(76, 4)).wrapping_sub(40u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00002370 ldr.w r3,[r6,#0xf38]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3896u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00002374 ldr.w r4,[r6,#0xf40]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3904u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x00002378 str r3,[sp,#0x8]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x0000237a ldr.w r3,[r6,#0xf84]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3972u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x0000237e vldr.32 s7,[pc,#0x2c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((9086u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(44u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x00002382 vfma.f32 s15,s14,s12
        let value = f32::from_bits(c.r(824, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(816, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002386 vstr.32 s13,[r11,#-0xc]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(76, 4)).wrapping_sub(12u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x0000238a vstr.32 s12,[sp,#0x44]
        let mut t64000 = 0u64;
        t64000 = ((c.r(84, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x0000238e str r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00002390 str r2,[sp,#0x10]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x00002392 vstr.32 s15,[r11,#-0x2c]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(76, 4)).wrapping_sub(44u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x00002396 b 0x000023c0
        return 0x23c0u32;
    }
    0x2398
}
pub fn b_23b0(c: &mut Machine) -> u32 {
    {
        // 0x000023b0 movs r3,#0x0
        let value = 0u64;
        c.w(44, 4, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
    }
    {
        // 0x000023b2 str.w r3,[r6,#0xf3c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3900u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x000023b6 ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x000023b8 cmp r0,r3
        let mut t1a6e00 = 0u64;
        let value = (c.r(44, 4) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), c.r(44, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(32, 4)).wrapping_sub(c.r(44, 4))) & 0xffffffffu64;
        let value = (signed(((t1a6e00 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a6e00 >> 0) & 0xffffffffu64) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x000023ba bgt 0x00002476
        let mut t181b00 = 0u64;
        let mut t181c00 = 0u64;
        let mut t181e00 = 0u64;
        t181b00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t181c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t181e00 = (((((t181b00 >> 0) & 0xffu64) != 0) & (((t181c00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t181e00 >> 0) & 0xffu64) != 0 {
            return 0x2476u32;
        }
    }
    0x23bc
}
pub fn b_23bc(c: &mut Machine) -> u32 {
    {
        // 0x000023bc vldr.32 s13,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    0x23c0
}
pub fn b_23c0(c: &mut Machine) -> u32 {
    {
        // 0x000023c0 vcmpe.f32 s13,s9
        let mut t35c00 = 0u64;
        let mut t35e00 = 0u64;
        let mut t36000 = 0u64;
        let mut t36100 = 0u64;
        let mut t36200 = 0u64;
        let mut t36400 = 0u64;
        let mut t36500 = 0u64;
        let mut t36600 = 0u64;
        let mut t36700 = 0u64;
        let mut t36800 = 0u64;
        let mut t36900 = 0u64;
        let mut t36a00 = 0u64;
        let mut t36b00 = 0u64;
        let mut t36c00 = 0u64;
        let mut t36d00 = 0u64;
        let mut t36e00 = 0u64;
        let mut t36f00 = 0u64;
        let mut t37000 = 0u64;
        t35c00 = ((f32::from_bits(c.r(820, 4) as u32) < f32::from_bits(c.r(804, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(820, 4) as u32) == f32::from_bits(c.r(804, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(804, 4) as u32) <= f32::from_bits(c.r(820, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(820, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(804, 4) as u32).is_nan() as u64) & 0xffu64;
        t36400 = (((t36100 >> 0) & 0xffu64) | ((t36200 >> 0) & 0xffu64)) & 0xffu64;
        t36500 = (c.r(180, 4) & 268435455u64) & 0xffffffffu64;
        t36600 = ((t35c00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36700 = (if 31u64 >= 32 {
            0
        } else {
            ((t36600 >> 0) & 0xffffffffu64) << (31u64 as u32)
        }) & 0xffffffffu64;
        t36800 =
            (((t36500 >> 0) & 0xffffffffu64) | ((t36700 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36900 = ((t35e00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36a00 = (if 30u64 >= 32 {
            0
        } else {
            ((t36900 >> 0) & 0xffffffffu64) << (30u64 as u32)
        }) & 0xffffffffu64;
        t36b00 =
            (((t36800 >> 0) & 0xffffffffu64) | ((t36a00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36c00 = ((t36000 >> 0) & 0xffu64) & 0xffffffffu64;
        t36d00 = (if 29u64 >= 32 {
            0
        } else {
            ((t36c00 >> 0) & 0xffffffffu64) << (29u64 as u32)
        }) & 0xffffffffu64;
        t36e00 =
            (((t36b00 >> 0) & 0xffffffffu64) | ((t36d00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36f00 = ((t36400 >> 0) & 0xffu64) & 0xffffffffu64;
        t37000 = (if 28u64 >= 32 {
            0
        } else {
            ((t36f00 >> 0) & 0xffffffffu64) << (28u64 as u32)
        }) & 0xffffffffu64;
        let value = (((t36e00 >> 0) & 0xffffffffu64) | ((t37000 >> 0) & 0xffffffffu64));
        c.w(180, 4, value);
    }
    {
        // 0x000023c4 vmrs apsr,fpscr
        let mut t6bd00 = 0u64;
        let mut t6bf00 = 0u64;
        let mut t6c000 = 0u64;
        let mut t6c200 = 0u64;
        let mut t6c300 = 0u64;
        let mut t6c500 = 0u64;
        let mut t6c600 = 0u64;
        t6bd00 = (if 31u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t6bd00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(96, 1, value);
        t6bf00 = (if 30u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (30u64 as u32)
        }) & 0xffffffffu64;
        t6c000 = (((t6bf00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c000 >> 0) & 0xffu64) & 1u64);
        c.w(97, 1, value);
        t6c200 = (if 29u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (29u64 as u32)
        }) & 0xffffffffu64;
        t6c300 = (((t6c200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c300 >> 0) & 0xffu64) & 1u64);
        c.w(98, 1, value);
        t6c500 = (if 28u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (28u64 as u32)
        }) & 0xffffffffu64;
        t6c600 = (((t6c500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c600 >> 0) & 0xffu64) & 1u64);
        c.w(99, 1, value);
    }
    {
        // 0x000023c8 ble 0x0000248e
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x248eu32;
        }
    }
    0x23ca
}
pub fn b_23ca(c: &mut Machine) -> u32 {
    {
        // 0x000023ca ldr r3,[0x00002724]
        let value = c.load(0x2724, 4);
        c.w(44, 4, value);
    }
    {
        // 0x000023cc vsub.f32 s13,s13,s9
        let value = (f32::from_bits(c.r(820, 4) as u32) - f32::from_bits(c.r(804, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x000023d0 ldr r1,[sp,#0xc]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(12u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x000023d2 ldr r2,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x000023d4 vldr.32 s15,[r3,#0x10]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x000023d8 str r2,[r3,#0x4]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x000023da vcmpe.f32 s15,#0
        let mut t35c00 = 0u64;
        let mut t35e00 = 0u64;
        let mut t36000 = 0u64;
        let mut t36100 = 0u64;
        let mut t36200 = 0u64;
        let mut t36400 = 0u64;
        let mut t36500 = 0u64;
        let mut t36600 = 0u64;
        let mut t36700 = 0u64;
        let mut t36800 = 0u64;
        let mut t36900 = 0u64;
        let mut t36a00 = 0u64;
        let mut t36b00 = 0u64;
        let mut t36c00 = 0u64;
        let mut t36d00 = 0u64;
        let mut t36e00 = 0u64;
        let mut t36f00 = 0u64;
        let mut t37000 = 0u64;
        let mut t37900 = 0u64;
        t37900 = (0u64) & 0xffffffffu64;
        t35c00 = ((f32::from_bits(c.r(828, 4) as u32)
            < f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(828, 4) as u32)
            == f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)) as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)
            <= f32::from_bits(c.r(828, 4) as u32)) as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32).is_nan() as u64) & 0xffu64;
        t36400 = (((t36100 >> 0) & 0xffu64) | ((t36200 >> 0) & 0xffu64)) & 0xffu64;
        t36500 = (c.r(180, 4) & 268435455u64) & 0xffffffffu64;
        t36600 = ((t35c00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36700 = (if 31u64 >= 32 {
            0
        } else {
            ((t36600 >> 0) & 0xffffffffu64) << (31u64 as u32)
        }) & 0xffffffffu64;
        t36800 =
            (((t36500 >> 0) & 0xffffffffu64) | ((t36700 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36900 = ((t35e00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36a00 = (if 30u64 >= 32 {
            0
        } else {
            ((t36900 >> 0) & 0xffffffffu64) << (30u64 as u32)
        }) & 0xffffffffu64;
        t36b00 =
            (((t36800 >> 0) & 0xffffffffu64) | ((t36a00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36c00 = ((t36000 >> 0) & 0xffu64) & 0xffffffffu64;
        t36d00 = (if 29u64 >= 32 {
            0
        } else {
            ((t36c00 >> 0) & 0xffffffffu64) << (29u64 as u32)
        }) & 0xffffffffu64;
        t36e00 =
            (((t36b00 >> 0) & 0xffffffffu64) | ((t36d00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36f00 = ((t36400 >> 0) & 0xffu64) & 0xffffffffu64;
        t37000 = (if 28u64 >= 32 {
            0
        } else {
            ((t36f00 >> 0) & 0xffffffffu64) << (28u64 as u32)
        }) & 0xffffffffu64;
        let value = (((t36e00 >> 0) & 0xffffffffu64) | ((t37000 >> 0) & 0xffffffffu64));
        c.w(180, 4, value);
    }
    {
        // 0x000023de ldr.w r2,[r3,#-0x4]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x000023e2 vstr.32 s13,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x000023e6 str r2,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x000023e8 ldr.w r2,[r3,#-0x8]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x000023ec vmrs apsr,fpscr
        let mut t6bd00 = 0u64;
        let mut t6bf00 = 0u64;
        let mut t6c000 = 0u64;
        let mut t6c200 = 0u64;
        let mut t6c300 = 0u64;
        let mut t6c500 = 0u64;
        let mut t6c600 = 0u64;
        t6bd00 = (if 31u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t6bd00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(96, 1, value);
        t6bf00 = (if 30u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (30u64 as u32)
        }) & 0xffffffffu64;
        t6c000 = (((t6bf00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c000 >> 0) & 0xffu64) & 1u64);
        c.w(97, 1, value);
        t6c200 = (if 29u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (29u64 as u32)
        }) & 0xffffffffu64;
        t6c300 = (((t6c200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c300 >> 0) & 0xffu64) & 1u64);
        c.w(98, 1, value);
        t6c500 = (if 28u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (28u64 as u32)
        }) & 0xffffffffu64;
        t6c600 = (((t6c500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c600 >> 0) & 0xffu64) & 1u64);
        c.w(99, 1, value);
    }
    {
        // 0x000023f0 str.w r2,[r3,#-0x4]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x000023f4 ldr.w r2,[r6,#0xf3c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3900u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x000023f8 sub.w r1,r2,r1
        let mut t1fe000 = 0u64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        t1fe000 = (c.r(36, 4)) & 0xffffffffu64;
        let value = (((t1fe000 >> 0) & 0xffffffffu64) <= c.r(40, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(40, 4), ((t1fe000 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(40, 4)).wrapping_sub(((t1fe000 >> 0) & 0xffffffffu64));
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x000023fc bne.w 0x00002734
        let mut t182500 = 0u64;
        t182500 = ((c.r(97, 1) == 0u64) as u64) & 0xffu64;
        if ((t182500 >> 0) & 0xffu64) != 0 {
            return 0x2734u32;
        }
    }
    0x2400
}
pub fn b_2400(c: &mut Machine) -> u32 {
    {
        // 0x00002400 cmp r1,#0x0
        let mut t1a6800 = 0u64;
        let value = (0u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 0u64, 4, true);
        c.w(103, 1, value);
        t1a6800 = ((c.r(36, 4)).wrapping_sub(0u64)) & 0xffffffffu64;
        let value = (signed(((t1a6800 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a6800 >> 0) & 0xffffffffu64) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x00002402 it lt
        let mut t185c00 = 0u64;
        t185c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
    }
    {
        // 0x00002404 add.lt r1,r1,r0
        let mut t1d00 = 0u64;
        let mut t2700 = 0u64;
        t1d00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2700 = ((((t1d00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x2406u32;
        }
        let value = ((c.r(36, 4) as u128 + c.r(32, 4) as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), c.r(32, 4), 4, false);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_add(c.r(32, 4));
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    0x2406
}
pub fn b_2406(c: &mut Machine) -> u32 {
    {
        // 0x00002406 add.w r1,r4,r1, lsl #0x2
        let mut t189200 = 0u64;
        let mut t189300 = 0u64;
        let mut t189500 = 0u64;
        let mut t189800 = 0u64;
        let mut t1a0b00 = 0u64;
        t189200 = ((32u64).wrapping_sub(2u64)) & 0xffffffffu64;
        t189300 = (if ((t189200 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(36, 4)) >> (((t189200 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t189500 = (((t189300 >> 0) & 0xffffffffu64) & 1u64) & 0xffffffffu64;
        let value = ((t189500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        t189800 = (if 2u64 >= 32 {
            0
        } else {
            (c.r(36, 4)) << (2u64 as u32)
        }) & 0xffffffffu64;
        t1a0b00 = ((t189800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        let value = ((c.r(48, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x0000240a ldr r1,[r1,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(36, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x0000240c str.w r1,[r3,#-0x8]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    0x2410
}
pub fn b_2410(c: &mut Machine) -> u32 {
    {
        // 0x00002410 vldr.32 s6,[r8]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(64, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x00002414 vldr.32 s4,[r9]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(68, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(784, 4, value);
    }
    {
        // 0x00002418 ldr r3,[0x00002728]
        let value = c.load(0x2728, 4);
        c.w(44, 4, value);
    }
    {
        // 0x0000241a vsub.f32 s14,s6,s4
        let value = (f32::from_bits(c.r(792, 4) as u32) - f32::from_bits(c.r(784, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x0000241e vldr.32 s5,[lr]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(88, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(788, 4, value);
    }
    {
        // 0x00002422 vldr.32 s15,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00002426 add.w r3,r4,r2, lsl #0x2
        let mut t189200 = 0u64;
        let mut t189300 = 0u64;
        let mut t189500 = 0u64;
        let mut t189800 = 0u64;
        let mut t1a0b00 = 0u64;
        t189200 = ((32u64).wrapping_sub(2u64)) & 0xffffffffu64;
        t189300 = (if ((t189200 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(40, 4)) >> (((t189200 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t189500 = (((t189300 >> 0) & 0xffffffffu64) & 1u64) & 0xffffffffu64;
        let value = ((t189500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        t189800 = (if 2u64 >= 32 {
            0
        } else {
            (c.r(40, 4)) << (2u64 as u32)
        }) & 0xffffffffu64;
        t1a0b00 = ((t189800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        let value = ((c.r(48, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(44, 4, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x0000242a vsub.f32 s5,s4,s5
        let value = (f32::from_bits(c.r(784, 4) as u32) - f32::from_bits(c.r(788, 4) as u32))
            .to_bits() as u64;
        c.w(788, 4, value);
    }
    {
        // 0x0000242e vldr.32 s11,[r10]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(72, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x00002432 vmul.f32 s13,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00002436 adds r2,#0x1
        let value = ((c.r(40, 4) as u128 + 1u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(40, 4), 1u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(40, 4)).wrapping_add(1u64);
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x00002438 vmov.f32 s15,s14
        let value = c.r(824, 4);
        c.w(828, 4, value);
    }
    {
        // 0x0000243c vsub.f32 s11,s11,s6
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(792, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00002440 cmp r0,r2
        let mut t1a6e00 = 0u64;
        let value = (c.r(40, 4) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), c.r(40, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(32, 4)).wrapping_sub(c.r(40, 4))) & 0xffffffffu64;
        let value = (signed(((t1a6e00 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a6e00 >> 0) & 0xffffffffu64) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x00002442 vfma.f32 s15,s5,s10
        let value = f32::from_bits(c.r(788, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002446 vadd.f32 s14,s14,s15
        let value = (f32::from_bits(c.r(824, 4) as u32) + f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x0000244a vfma.f32 s14,s11,s10
        let value = f32::from_bits(c.r(812, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x0000244e vadd.f32 s15,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002452 vfnms.f32 s15,s13,s14
        let mut t49a00 = 0u64;
        let mut t49b00 = 0u64;
        t49a00 = ((-f32::from_bits(c.r(828, 4) as u32)).to_bits() as u64) & 0xffffffffu64;
        t49b00 = ((f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64)
            & 0xffffffffu64;
        let value = (f32::from_bits(((t49a00 >> 0) & 0xffffffffu64) as u32)
            + f32::from_bits(((t49b00 >> 0) & 0xffffffffu64) as u32))
        .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002456 vmov.f32 s14,s6
        let value = c.r(792, 4);
        c.w(824, 4, value);
    }
    {
        // 0x0000245a vmul.f32 s15,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x0000245e vfma.f32 s15,s5,s10
        let value = f32::from_bits(c.r(788, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002462 vfma.f32 s14,s13,s15
        let value = f32::from_bits(c.r(820, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00002466 vstr.32 s14,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x0000246a ble 0x000023b0
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x23b0u32;
        }
    }
    0x246c
}
pub fn b_246c(c: &mut Machine) -> u32 {
    {
        // 0x0000246c ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x0000246e str.w r2,[r6,#0xf3c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3900u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x00002472 cmp r0,r3
        let mut t1a6e00 = 0u64;
        let value = (c.r(44, 4) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), c.r(44, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(32, 4)).wrapping_sub(c.r(44, 4))) & 0xffffffffu64;
        let value = (signed(((t1a6e00 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a6e00 >> 0) & 0xffffffffu64) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x00002474 ble 0x000023bc
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x23bcu32;
        }
    }
    0x2476
}
pub fn b_2476(c: &mut Machine) -> u32 {
    {
        // 0x00002476 vldr.32 s13,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x0000247a ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x0000247c vcmpe.f32 s13,s9
        let mut t35c00 = 0u64;
        let mut t35e00 = 0u64;
        let mut t36000 = 0u64;
        let mut t36100 = 0u64;
        let mut t36200 = 0u64;
        let mut t36400 = 0u64;
        let mut t36500 = 0u64;
        let mut t36600 = 0u64;
        let mut t36700 = 0u64;
        let mut t36800 = 0u64;
        let mut t36900 = 0u64;
        let mut t36a00 = 0u64;
        let mut t36b00 = 0u64;
        let mut t36c00 = 0u64;
        let mut t36d00 = 0u64;
        let mut t36e00 = 0u64;
        let mut t36f00 = 0u64;
        let mut t37000 = 0u64;
        t35c00 = ((f32::from_bits(c.r(820, 4) as u32) < f32::from_bits(c.r(804, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(820, 4) as u32) == f32::from_bits(c.r(804, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(804, 4) as u32) <= f32::from_bits(c.r(820, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(820, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(804, 4) as u32).is_nan() as u64) & 0xffu64;
        t36400 = (((t36100 >> 0) & 0xffu64) | ((t36200 >> 0) & 0xffu64)) & 0xffu64;
        t36500 = (c.r(180, 4) & 268435455u64) & 0xffffffffu64;
        t36600 = ((t35c00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36700 = (if 31u64 >= 32 {
            0
        } else {
            ((t36600 >> 0) & 0xffffffffu64) << (31u64 as u32)
        }) & 0xffffffffu64;
        t36800 =
            (((t36500 >> 0) & 0xffffffffu64) | ((t36700 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36900 = ((t35e00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36a00 = (if 30u64 >= 32 {
            0
        } else {
            ((t36900 >> 0) & 0xffffffffu64) << (30u64 as u32)
        }) & 0xffffffffu64;
        t36b00 =
            (((t36800 >> 0) & 0xffffffffu64) | ((t36a00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36c00 = ((t36000 >> 0) & 0xffu64) & 0xffffffffu64;
        t36d00 = (if 29u64 >= 32 {
            0
        } else {
            ((t36c00 >> 0) & 0xffffffffu64) << (29u64 as u32)
        }) & 0xffffffffu64;
        t36e00 =
            (((t36b00 >> 0) & 0xffffffffu64) | ((t36d00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36f00 = ((t36400 >> 0) & 0xffu64) & 0xffffffffu64;
        t37000 = (if 28u64 >= 32 {
            0
        } else {
            ((t36f00 >> 0) & 0xffffffffu64) << (28u64 as u32)
        }) & 0xffffffffu64;
        let value = (((t36e00 >> 0) & 0xffffffffu64) | ((t37000 >> 0) & 0xffffffffu64));
        c.w(180, 4, value);
    }
    {
        // 0x00002480 adds r3,#0x1
        let value = ((c.r(44, 4) as u128 + 1u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), 1u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_add(1u64);
        c.w(44, 4, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x00002482 vmrs apsr,fpscr
        let mut t6bd00 = 0u64;
        let mut t6bf00 = 0u64;
        let mut t6c000 = 0u64;
        let mut t6c200 = 0u64;
        let mut t6c300 = 0u64;
        let mut t6c500 = 0u64;
        let mut t6c600 = 0u64;
        t6bd00 = (if 31u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t6bd00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(96, 1, value);
        t6bf00 = (if 30u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (30u64 as u32)
        }) & 0xffffffffu64;
        t6c000 = (((t6bf00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c000 >> 0) & 0xffu64) & 1u64);
        c.w(97, 1, value);
        t6c200 = (if 29u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (29u64 as u32)
        }) & 0xffffffffu64;
        t6c300 = (((t6c200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c300 >> 0) & 0xffu64) & 1u64);
        c.w(98, 1, value);
        t6c500 = (if 28u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (28u64 as u32)
        }) & 0xffffffffu64;
        t6c600 = (((t6c500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c600 >> 0) & 0xffu64) & 1u64);
        c.w(99, 1, value);
    }
    {
        // 0x00002486 str r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00002488 str.w r3,[r6,#0xf84]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3972u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x0000248c bgt 0x000023ca
        let mut t181b00 = 0u64;
        let mut t181c00 = 0u64;
        let mut t181e00 = 0u64;
        t181b00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t181c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t181e00 = (((((t181b00 >> 0) & 0xffu64) != 0) & (((t181c00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t181e00 >> 0) & 0xffu64) != 0 {
            return 0x23cau32;
        }
    }
    0x248e
}
pub fn b_248e(c: &mut Machine) -> u32 {
    {
        // 0x0000248e ldrb.w r3,[r6,#0xf78]
        let mut t19c800 = 0u64;
        let mut t1aa600 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3960u64)) & 0xffffffffu64;
        t1aa600 = (c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 1)) & 0xffu64;
        let value = ((t1aa600 >> 0) & 0xffu64);
        c.w(44, 4, value);
    }
    {
        // 0x00002492 ldr r2,[sp,#0x10]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00002494 cbz r3,0x000024ba
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(44, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x24bau32;
        }
    }
    0x2496
}
pub fn b_2496(c: &mut Machine) -> u32 {
    {
        // 0x00002496 ldr r3,[0x0000272c]
        let value = c.load(0x272c, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00002498 vmov.f32 s11,0x3f000000
        let value = 1056964608u64;
        c.w(812, 4, value);
    }
    {
        // 0x0000249c vmov.f32 s14,0x3f800000
        let value = 1065353216u64;
        c.w(824, 4, value);
    }
    {
        // 0x000024a0 vldr.32 s15,[r3,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x000024a4 vmul.f32 s15,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x000024a8 vmul.f32 s13,s15,s11
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x000024ac vcmpe.f32 s13,s9
        let mut t35c00 = 0u64;
        let mut t35e00 = 0u64;
        let mut t36000 = 0u64;
        let mut t36100 = 0u64;
        let mut t36200 = 0u64;
        let mut t36400 = 0u64;
        let mut t36500 = 0u64;
        let mut t36600 = 0u64;
        let mut t36700 = 0u64;
        let mut t36800 = 0u64;
        let mut t36900 = 0u64;
        let mut t36a00 = 0u64;
        let mut t36b00 = 0u64;
        let mut t36c00 = 0u64;
        let mut t36d00 = 0u64;
        let mut t36e00 = 0u64;
        let mut t36f00 = 0u64;
        let mut t37000 = 0u64;
        t35c00 = ((f32::from_bits(c.r(820, 4) as u32) < f32::from_bits(c.r(804, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(820, 4) as u32) == f32::from_bits(c.r(804, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(804, 4) as u32) <= f32::from_bits(c.r(820, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(820, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(804, 4) as u32).is_nan() as u64) & 0xffu64;
        t36400 = (((t36100 >> 0) & 0xffu64) | ((t36200 >> 0) & 0xffu64)) & 0xffu64;
        t36500 = (c.r(180, 4) & 268435455u64) & 0xffffffffu64;
        t36600 = ((t35c00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36700 = (if 31u64 >= 32 {
            0
        } else {
            ((t36600 >> 0) & 0xffffffffu64) << (31u64 as u32)
        }) & 0xffffffffu64;
        t36800 =
            (((t36500 >> 0) & 0xffffffffu64) | ((t36700 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36900 = ((t35e00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36a00 = (if 30u64 >= 32 {
            0
        } else {
            ((t36900 >> 0) & 0xffffffffu64) << (30u64 as u32)
        }) & 0xffffffffu64;
        t36b00 =
            (((t36800 >> 0) & 0xffffffffu64) | ((t36a00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36c00 = ((t36000 >> 0) & 0xffu64) & 0xffffffffu64;
        t36d00 = (if 29u64 >= 32 {
            0
        } else {
            ((t36c00 >> 0) & 0xffffffffu64) << (29u64 as u32)
        }) & 0xffffffffu64;
        t36e00 =
            (((t36b00 >> 0) & 0xffffffffu64) | ((t36d00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36f00 = ((t36400 >> 0) & 0xffu64) & 0xffffffffu64;
        t37000 = (if 28u64 >= 32 {
            0
        } else {
            ((t36f00 >> 0) & 0xffffffffu64) << (28u64 as u32)
        }) & 0xffffffffu64;
        let value = (((t36e00 >> 0) & 0xffffffffu64) | ((t37000 >> 0) & 0xffffffffu64));
        c.w(180, 4, value);
    }
    {
        // 0x000024b0 vmrs apsr,fpscr
        let mut t6bd00 = 0u64;
        let mut t6bf00 = 0u64;
        let mut t6c000 = 0u64;
        let mut t6c200 = 0u64;
        let mut t6c300 = 0u64;
        let mut t6c500 = 0u64;
        let mut t6c600 = 0u64;
        t6bd00 = (if 31u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t6bd00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(96, 1, value);
        t6bf00 = (if 30u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (30u64 as u32)
        }) & 0xffffffffu64;
        t6c000 = (((t6bf00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c000 >> 0) & 0xffu64) & 1u64);
        c.w(97, 1, value);
        t6c200 = (if 29u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (29u64 as u32)
        }) & 0xffffffffu64;
        t6c300 = (((t6c200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c300 >> 0) & 0xffu64) & 1u64);
        c.w(98, 1, value);
        t6c500 = (if 28u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (28u64 as u32)
        }) & 0xffffffffu64;
        t6c600 = (((t6c500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c600 >> 0) & 0xffu64) & 1u64);
        c.w(99, 1, value);
    }
    {
        // 0x000024b4 it pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x000024b6 vmov.pl.f32 s13,s14
        let mut tf00 = 0u64;
        let mut t2700 = 0u64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2700 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x24bau32;
        }
        let value = c.r(824, 4);
        c.w(820, 4, value);
    }
    0x24ba
}
pub fn b_24ba(c: &mut Machine) -> u32 {
    {
        // 0x000024ba ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x000024bc cmp r0,r3
        let mut t1a6e00 = 0u64;
        let value = (c.r(44, 4) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), c.r(44, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(32, 4)).wrapping_sub(c.r(44, 4))) & 0xffffffffu64;
        let value = (signed(((t1a6e00 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a6e00 >> 0) & 0xffffffffu64) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x000024be bgt.w 0x000030ec
        let mut t183c00 = 0u64;
        let mut t183d00 = 0u64;
        let mut t183f00 = 0u64;
        t183c00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t183d00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t183f00 = (((((t183c00 >> 0) & 0xffu64) != 0) & (((t183d00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t183f00 >> 0) & 0xffu64) != 0 {
            return 0x30ecu32;
        }
    }
    0x24c2
}
pub fn b_24c2(c: &mut Machine) -> u32 {
    {
        // 0x000024c2 ldr r3,[0x00002724]
        let value = c.load(0x2724, 4);
        c.w(44, 4, value);
    }
    {
        // 0x000024c4 vmov.f32 s9,0x3f000000
        let value = 1056964608u64;
        c.w(804, 4, value);
    }
    {
        // 0x000024c8 vldr.32 s14,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x000024cc vldr.32 s10,[r3,#-0x4]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(44, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x000024d0 vldr.32 s15,[r3,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x000024d4 vsub.f32 s11,s14,s10
        let value = (f32::from_bits(c.r(824, 4) as u32) - f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x000024d8 vldr.32 s8,[r3,#-0x8]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(44, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x000024dc vsub.f32 s10,s10,s15
        let value = (f32::from_bits(c.r(808, 4) as u32) - f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x000024e0 vsub.f32 s8,s8,s14
        let value = (f32::from_bits(c.r(800, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(800, 4, value);
    }
    {
        // 0x000024e4 vmov.f32 s15,s11
        let value = c.r(812, 4);
        c.w(828, 4, value);
    }
    {
        // 0x000024e8 vfma.f32 s15,s10,s9
        let value = f32::from_bits(c.r(808, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x000024ec vadd.f32 s11,s11,s15
        let value = (f32::from_bits(c.r(812, 4) as u32) + f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x000024f0 vfma.f32 s11,s8,s9
        let value = f32::from_bits(c.r(800, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(812, 4) as u32),
            )
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x000024f4 vadd.f32 s15,s15,s11
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x000024f8 vfnms.f32 s15,s11,s13
        let mut t49a00 = 0u64;
        let mut t49b00 = 0u64;
        t49a00 = ((-f32::from_bits(c.r(828, 4) as u32)).to_bits() as u64) & 0xffffffffu64;
        t49b00 = ((f32::from_bits(c.r(812, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64)
            & 0xffffffffu64;
        let value = (f32::from_bits(((t49a00 >> 0) & 0xffffffffu64) as u32)
            + f32::from_bits(((t49b00 >> 0) & 0xffffffffu64) as u32))
        .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x000024fc vmul.f32 s15,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002500 vfma.f32 s15,s10,s9
        let value = f32::from_bits(c.r(808, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002504 vfma.f32 s14,s13,s15
        let value = f32::from_bits(c.r(820, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00002508 vadd.f32 s14,s14,s23
        let value = (f32::from_bits(c.r(824, 4) as u32) + f32::from_bits(c.r(860, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    0x250c
}
pub fn b_250c(c: &mut Machine) -> u32 {
    {
        // 0x0000250c vldr.32 s11,[r5,#0x40]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x00002510 vmov.f32 s9,0x3f800000
        let value = 1065353216u64;
        c.w(804, 4, value);
    }
    {
        // 0x00002514 vldr.32 s15,[r5,#0x10]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00002518 mov.w r9,#0x0
        let mut t187d00 = 0u64;
        t187d00 = (0u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = ((t187d00 >> 0) & 0xffffffffu64);
        c.w(68, 4, value);
        let value = (signed(c.r(68, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(68, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x0000251c vldr.32 s13,[r5,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00002520 mov.w r8,#0x0
        let mut t187d00 = 0u64;
        t187d00 = (0u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = ((t187d00 >> 0) & 0xffffffffu64);
        c.w(64, 4, value);
        let value = (signed(c.r(64, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(64, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00002524 vfms.f32 s15,s11,s12
        let value = (-f32::from_bits(c.r(812, 4) as u32))
            .fast_mul_add(
                f32::from_bits(c.r(816, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002528 vldr.32 s10,[r5,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x0000252c ldr r3,[r5,#0x1c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x0000252e vadd.f32 s13,s13,s10
        let value = (f32::from_bits(c.r(820, 4) as u32) + f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00002532 ldr r1,[r5,#0x18]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(24u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00002534 str r3,[r5,#0x20]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00002536 vmov.f32 s10,0x3f000000
        let value = 1056964608u64;
        c.w(808, 4, value);
    }
    {
        // 0x0000253a ldr r3,[r5,#0x14]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(20u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x0000253c vldr.32 s8,[pc,#0x1f0]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((9532u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(496u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x00002540 vfma.f32 s12,s11,s15
        let value = f32::from_bits(c.r(812, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(816, 4) as u32),
            )
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00002544 ldr.w r0,[r6,#0xff0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(4080u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x00002548 ldr.w lr,[r6,#0xff8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(4088u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(88, 4, value);
    }
    {
        // 0x0000254c ldr r4,[r5,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x0000254e ldr.w r12,[r5,#0x44]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(80, 4, value);
    }
    {
        // 0x00002552 vstr.32 s14,[r11,#-0x30]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(76, 4)).wrapping_sub(48u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x00002556 str r1,[r5,#0x1c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x00002558 vstr.32 s13,[r5,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x0000255c vstr.32 s15,[sp,#0x44]
        let mut t64000 = 0u64;
        t64000 = ((c.r(84, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x00002560 str r3,[r5,#0x18]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(24u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00002562 vstr.32 s12,[r5,#0x14]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(20u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x00002566 b 0x000025e4
        return 0x25e4u32;
    }
    0x2568
}
pub fn b_2568(c: &mut Machine) -> u32 {
    {
        // 0x00002568 cmp r1,#0x0
        let mut t1a6800 = 0u64;
        let value = (0u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 0u64, 4, true);
        c.w(103, 1, value);
        t1a6800 = ((c.r(36, 4)).wrapping_sub(0u64)) & 0xffffffffu64;
        let value = (signed(((t1a6800 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a6800 >> 0) & 0xffffffffu64) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x0000256a it lt
        let mut t185c00 = 0u64;
        t185c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
    }
    {
        // 0x0000256c add.lt r1,r1,r0
        let mut t1d00 = 0u64;
        let mut t2700 = 0u64;
        t1d00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2700 = ((((t1d00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x256eu32;
        }
        let value = ((c.r(36, 4) as u128 + c.r(32, 4) as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), c.r(32, 4), 4, false);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_add(c.r(32, 4));
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    0x256e
}
pub fn b_256e(c: &mut Machine) -> u32 {
    {
        // 0x0000256e add.w r1,r4,r1, lsl #0x2
        let mut t189200 = 0u64;
        let mut t189300 = 0u64;
        let mut t189500 = 0u64;
        let mut t189800 = 0u64;
        let mut t1a0b00 = 0u64;
        t189200 = ((32u64).wrapping_sub(2u64)) & 0xffffffffu64;
        t189300 = (if ((t189200 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(36, 4)) >> (((t189200 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t189500 = (((t189300 >> 0) & 0xffffffffu64) & 1u64) & 0xffffffffu64;
        let value = ((t189500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        t189800 = (if 2u64 >= 32 {
            0
        } else {
            (c.r(36, 4)) << (2u64 as u32)
        }) & 0xffffffffu64;
        t1a0b00 = ((t189800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        let value = ((c.r(48, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00002572 ldr r1,[r1,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(36, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00002574 str r1,[r5,#0x24]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    0x2576
}
pub fn b_2576(c: &mut Machine) -> u32 {
    {
        // 0x00002576 vldr.32 s7,[r5,#0x18]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(24u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x0000257a add.w r1,r4,r3, lsl #0x2
        let mut t189200 = 0u64;
        let mut t189300 = 0u64;
        let mut t189500 = 0u64;
        let mut t189800 = 0u64;
        let mut t1a0b00 = 0u64;
        t189200 = ((32u64).wrapping_sub(2u64)) & 0xffffffffu64;
        t189300 = (if ((t189200 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(44, 4)) >> (((t189200 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t189500 = (((t189300 >> 0) & 0xffffffffu64) & 1u64) & 0xffffffffu64;
        let value = ((t189500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        t189800 = (if 2u64 >= 32 {
            0
        } else {
            (c.r(44, 4)) << (2u64 as u32)
        }) & 0xffffffffu64;
        t1a0b00 = ((t189800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        let value = ((c.r(48, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x0000257e vldr.32 s12,[r5,#0x1c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x00002582 adds r3,#0x1
        let value = ((c.r(44, 4) as u128 + 1u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), 1u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_add(1u64);
        c.w(44, 4, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x00002584 vldr.32 s6,[r5,#0xc]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(12u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x00002588 vsub.f32 s14,s7,s12
        let value = (f32::from_bits(c.r(796, 4) as u32) - f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x0000258c vldr.32 s5,[r5,#0x14]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(20u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(788, 4, value);
    }
    {
        // 0x00002590 vmul.f32 s13,s13,s6
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(792, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00002594 vldr.32 s11,[r5,#0x20]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x00002598 vsub.f32 s12,s12,s5
        let value = (f32::from_bits(c.r(816, 4) as u32) - f32::from_bits(c.r(788, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x0000259c cmp r0,r3
        let mut t1a6e00 = 0u64;
        let value = (c.r(44, 4) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), c.r(44, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(32, 4)).wrapping_sub(c.r(44, 4))) & 0xffffffffu64;
        let value = (signed(((t1a6e00 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a6e00 >> 0) & 0xffffffffu64) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x0000259e vmov.f32 s6,s14
        let value = c.r(824, 4);
        c.w(792, 4, value);
    }
    {
        // 0x000025a2 vsub.f32 s11,s11,s7
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(796, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x000025a6 vfma.f32 s6,s12,s10
        let value = f32::from_bits(c.r(816, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(792, 4) as u32),
            )
            .to_bits() as u64;
        c.w(792, 4, value);
    }
    {
        // 0x000025aa vadd.f32 s14,s14,s6
        let value = (f32::from_bits(c.r(824, 4) as u32) + f32::from_bits(c.r(792, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x000025ae vfma.f32 s14,s11,s10
        let value = f32::from_bits(c.r(812, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x000025b2 vadd.f32 s11,s6,s14
        let value = (f32::from_bits(c.r(792, 4) as u32) + f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x000025b6 vfnms.f32 s11,s13,s14
        let mut t49a00 = 0u64;
        let mut t49b00 = 0u64;
        t49a00 = ((-f32::from_bits(c.r(812, 4) as u32)).to_bits() as u64) & 0xffffffffu64;
        t49b00 = ((f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64)
            & 0xffffffffu64;
        let value = (f32::from_bits(((t49a00 >> 0) & 0xffffffffu64) as u32)
            + f32::from_bits(((t49b00 >> 0) & 0xffffffffu64) as u32))
        .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x000025ba vmul.f32 s14,s13,s11
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x000025be vfma.f32 s14,s12,s10
        let value = f32::from_bits(c.r(816, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x000025c2 vmov.f32 s12,s14
        let value = c.r(824, 4);
        c.w(816, 4, value);
    }
    {
        // 0x000025c6 vmov.f32 s14,s7
        let value = c.r(796, 4);
        c.w(824, 4, value);
    }
    {
        // 0x000025ca vfma.f32 s14,s13,s12
        let value = f32::from_bits(c.r(820, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(816, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x000025ce vstr.32 s14,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x000025d2 bgt.w 0x00002776
        let mut t183c00 = 0u64;
        let mut t183d00 = 0u64;
        let mut t183f00 = 0u64;
        t183c00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t183d00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t183f00 = (((((t183c00 >> 0) & 0xffu64) != 0) & (((t183d00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t183f00 >> 0) & 0xffu64) != 0 {
            return 0x2776u32;
        }
    }
    0x25d6
}
pub fn b_25d6(c: &mut Machine) -> u32 {
    {
        // 0x000025d6 cmp r0,r12
        let mut t1a7000 = 0u64;
        let value = (c.r(80, 4) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), c.r(80, 4), 4, true);
        c.w(103, 1, value);
        t1a7000 = ((c.r(32, 4)).wrapping_sub(c.r(80, 4))) & 0xffffffffu64;
        let value = (signed(((t1a7000 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a7000 >> 0) & 0xffffffffu64) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x000025d8 str.w r8,[r6,#0xffc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(4092u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(64, 4));
    }
    {
        // 0x000025dc bgt.w 0x00002780
        let mut t183c00 = 0u64;
        let mut t183d00 = 0u64;
        let mut t183f00 = 0u64;
        t183c00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t183d00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t183f00 = (((((t183c00 >> 0) & 0xffu64) != 0) & (((t183d00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t183f00 >> 0) & 0xffu64) != 0 {
            return 0x2780u32;
        }
    }
    0x25e0
}
pub fn b_25e0(c: &mut Machine) -> u32 {
    {
        // 0x000025e0 vldr.32 s13,[r5,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    0x25e4
}
pub fn b_25e4(c: &mut Machine) -> u32 {
    {
        // 0x000025e4 vcmpe.f32 s13,s9
        let mut t35c00 = 0u64;
        let mut t35e00 = 0u64;
        let mut t36000 = 0u64;
        let mut t36100 = 0u64;
        let mut t36200 = 0u64;
        let mut t36400 = 0u64;
        let mut t36500 = 0u64;
        let mut t36600 = 0u64;
        let mut t36700 = 0u64;
        let mut t36800 = 0u64;
        let mut t36900 = 0u64;
        let mut t36a00 = 0u64;
        let mut t36b00 = 0u64;
        let mut t36c00 = 0u64;
        let mut t36d00 = 0u64;
        let mut t36e00 = 0u64;
        let mut t36f00 = 0u64;
        let mut t37000 = 0u64;
        t35c00 = ((f32::from_bits(c.r(820, 4) as u32) < f32::from_bits(c.r(804, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(820, 4) as u32) == f32::from_bits(c.r(804, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(804, 4) as u32) <= f32::from_bits(c.r(820, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(820, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(804, 4) as u32).is_nan() as u64) & 0xffu64;
        t36400 = (((t36100 >> 0) & 0xffu64) | ((t36200 >> 0) & 0xffu64)) & 0xffu64;
        t36500 = (c.r(180, 4) & 268435455u64) & 0xffffffffu64;
        t36600 = ((t35c00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36700 = (if 31u64 >= 32 {
            0
        } else {
            ((t36600 >> 0) & 0xffffffffu64) << (31u64 as u32)
        }) & 0xffffffffu64;
        t36800 =
            (((t36500 >> 0) & 0xffffffffu64) | ((t36700 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36900 = ((t35e00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36a00 = (if 30u64 >= 32 {
            0
        } else {
            ((t36900 >> 0) & 0xffffffffu64) << (30u64 as u32)
        }) & 0xffffffffu64;
        t36b00 =
            (((t36800 >> 0) & 0xffffffffu64) | ((t36a00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36c00 = ((t36000 >> 0) & 0xffu64) & 0xffffffffu64;
        t36d00 = (if 29u64 >= 32 {
            0
        } else {
            ((t36c00 >> 0) & 0xffffffffu64) << (29u64 as u32)
        }) & 0xffffffffu64;
        t36e00 =
            (((t36b00 >> 0) & 0xffffffffu64) | ((t36d00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36f00 = ((t36400 >> 0) & 0xffu64) & 0xffffffffu64;
        t37000 = (if 28u64 >= 32 {
            0
        } else {
            ((t36f00 >> 0) & 0xffffffffu64) << (28u64 as u32)
        }) & 0xffffffffu64;
        let value = (((t36e00 >> 0) & 0xffffffffu64) | ((t37000 >> 0) & 0xffffffffu64));
        c.w(180, 4, value);
    }
    {
        // 0x000025e8 vmrs apsr,fpscr
        let mut t6bd00 = 0u64;
        let mut t6bf00 = 0u64;
        let mut t6c000 = 0u64;
        let mut t6c200 = 0u64;
        let mut t6c300 = 0u64;
        let mut t6c500 = 0u64;
        let mut t6c600 = 0u64;
        t6bd00 = (if 31u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t6bd00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(96, 1, value);
        t6bf00 = (if 30u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (30u64 as u32)
        }) & 0xffffffffu64;
        t6c000 = (((t6bf00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c000 >> 0) & 0xffu64) & 1u64);
        c.w(97, 1, value);
        t6c200 = (if 29u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (29u64 as u32)
        }) & 0xffffffffu64;
        t6c300 = (((t6c200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c300 >> 0) & 0xffu64) & 1u64);
        c.w(98, 1, value);
        t6c500 = (if 28u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (28u64 as u32)
        }) & 0xffffffffu64;
        t6c600 = (((t6c500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c600 >> 0) & 0xffu64) & 1u64);
        c.w(99, 1, value);
    }
    {
        // 0x000025ec ble.w 0x00002798
        let mut t184000 = 0u64;
        let mut t184200 = 0u64;
        t184000 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t184200 = (((c.r(97, 1) != 0) | (((t184000 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t184200 >> 0) & 0xffu64) != 0 {
            return 0x2798u32;
        }
    }
    0x25f0
}
pub fn b_25f0(c: &mut Machine) -> u32 {
    {
        // 0x000025f0 ldr r3,[r5,#0x2c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x000025f2 vsub.f32 s13,s13,s9
        let value = (f32::from_bits(c.r(820, 4) as u32) - f32::from_bits(c.r(804, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x000025f6 vldr.32 s14,[r5,#0x3c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(60u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x000025fa str r3,[r5,#0x30]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x000025fc vcmpe.f32 s14,#0
        let mut t35c00 = 0u64;
        let mut t35e00 = 0u64;
        let mut t36000 = 0u64;
        let mut t36100 = 0u64;
        let mut t36200 = 0u64;
        let mut t36400 = 0u64;
        let mut t36500 = 0u64;
        let mut t36600 = 0u64;
        let mut t36700 = 0u64;
        let mut t36800 = 0u64;
        let mut t36900 = 0u64;
        let mut t36a00 = 0u64;
        let mut t36b00 = 0u64;
        let mut t36c00 = 0u64;
        let mut t36d00 = 0u64;
        let mut t36e00 = 0u64;
        let mut t36f00 = 0u64;
        let mut t37000 = 0u64;
        let mut t37900 = 0u64;
        t37900 = (0u64) & 0xffffffffu64;
        t35c00 = ((f32::from_bits(c.r(824, 4) as u32)
            < f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(824, 4) as u32)
            == f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)) as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)
            <= f32::from_bits(c.r(824, 4) as u32)) as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(824, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32).is_nan() as u64) & 0xffu64;
        t36400 = (((t36100 >> 0) & 0xffu64) | ((t36200 >> 0) & 0xffu64)) & 0xffu64;
        t36500 = (c.r(180, 4) & 268435455u64) & 0xffffffffu64;
        t36600 = ((t35c00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36700 = (if 31u64 >= 32 {
            0
        } else {
            ((t36600 >> 0) & 0xffffffffu64) << (31u64 as u32)
        }) & 0xffffffffu64;
        t36800 =
            (((t36500 >> 0) & 0xffffffffu64) | ((t36700 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36900 = ((t35e00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36a00 = (if 30u64 >= 32 {
            0
        } else {
            ((t36900 >> 0) & 0xffffffffu64) << (30u64 as u32)
        }) & 0xffffffffu64;
        t36b00 =
            (((t36800 >> 0) & 0xffffffffu64) | ((t36a00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36c00 = ((t36000 >> 0) & 0xffu64) & 0xffffffffu64;
        t36d00 = (if 29u64 >= 32 {
            0
        } else {
            ((t36c00 >> 0) & 0xffffffffu64) << (29u64 as u32)
        }) & 0xffffffffu64;
        t36e00 =
            (((t36b00 >> 0) & 0xffffffffu64) | ((t36d00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36f00 = ((t36400 >> 0) & 0xffu64) & 0xffffffffu64;
        t37000 = (if 28u64 >= 32 {
            0
        } else {
            ((t36f00 >> 0) & 0xffffffffu64) << (28u64 as u32)
        }) & 0xffffffffu64;
        let value = (((t36e00 >> 0) & 0xffffffffu64) | ((t37000 >> 0) & 0xffffffffu64));
        c.w(180, 4, value);
    }
    {
        // 0x00002600 ldr r3,[r5,#0x28]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00002602 vstr.32 s13,[r5,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x00002606 str r3,[r5,#0x2c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00002608 ldr r3,[r5,#0x24]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x0000260a vmrs apsr,fpscr
        let mut t6bd00 = 0u64;
        let mut t6bf00 = 0u64;
        let mut t6c000 = 0u64;
        let mut t6c200 = 0u64;
        let mut t6c300 = 0u64;
        let mut t6c500 = 0u64;
        let mut t6c600 = 0u64;
        t6bd00 = (if 31u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t6bd00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(96, 1, value);
        t6bf00 = (if 30u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (30u64 as u32)
        }) & 0xffffffffu64;
        t6c000 = (((t6bf00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c000 >> 0) & 0xffu64) & 1u64);
        c.w(97, 1, value);
        t6c200 = (if 29u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (29u64 as u32)
        }) & 0xffffffffu64;
        t6c300 = (((t6c200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c300 >> 0) & 0xffu64) & 1u64);
        c.w(98, 1, value);
        t6c500 = (if 28u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (28u64 as u32)
        }) & 0xffffffffu64;
        t6c600 = (((t6c500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c600 >> 0) & 0xffu64) & 1u64);
        c.w(99, 1, value);
    }
    {
        // 0x0000260e str r3,[r5,#0x28]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00002610 ldr.w r3,[r6,#0xffc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(4092u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00002614 sub.w r1,r3,r2
        let mut t1fe000 = 0u64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        t1fe000 = (c.r(40, 4)) & 0xffffffffu64;
        let value = (((t1fe000 >> 0) & 0xffffffffu64) <= c.r(44, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), ((t1fe000 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_sub(((t1fe000 >> 0) & 0xffffffffu64));
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00002618 beq 0x00002568
        let mut t180200 = 0u64;
        t180200 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        if ((t180200 >> 0) & 0xffu64) != 0 {
            return 0x2568u32;
        }
    }
    0x261a
}
pub fn b_261a(c: &mut Machine) -> u32 {
    {
        // 0x0000261a cmp r1,#0x0
        let mut t1a6800 = 0u64;
        let value = (0u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 0u64, 4, true);
        c.w(103, 1, value);
        t1a6800 = ((c.r(36, 4)).wrapping_sub(0u64)) & 0xffffffffu64;
        let value = (signed(((t1a6800 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a6800 >> 0) & 0xffffffffu64) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x0000261c vsub.f32 s7,s14,s8
        let value = (f32::from_bits(c.r(824, 4) as u32) - f32::from_bits(c.r(800, 4) as u32))
            .to_bits() as u64;
        c.w(796, 4, value);
    }
    {
        // 0x00002620 it lt
        let mut t185c00 = 0u64;
        t185c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
    }
    {
        // 0x00002622 add.lt r1,r1,r0
        let mut t1d00 = 0u64;
        let mut t2700 = 0u64;
        t1d00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2700 = ((((t1d00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x2624u32;
        }
        let value = ((c.r(36, 4) as u128 + c.r(32, 4) as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), c.r(32, 4), 4, false);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_add(c.r(32, 4));
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    0x2624
}
pub fn b_2624(c: &mut Machine) -> u32 {
    {
        // 0x00002624 subs.w r7,r3,lr
        let mut t1fe000 = 0u64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        t1fe000 = (c.r(88, 4)) & 0xffffffffu64;
        let value = (((t1fe000 >> 0) & 0xffffffffu64) <= c.r(44, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), ((t1fe000 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_sub(((t1fe000 >> 0) & 0xffffffffu64));
        c.w(60, 4, value);
        let value = (signed(c.r(60, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(60, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x00002628 vcmpe.f32 s7,#0
        let mut t35c00 = 0u64;
        let mut t35e00 = 0u64;
        let mut t36000 = 0u64;
        let mut t36100 = 0u64;
        let mut t36200 = 0u64;
        let mut t36400 = 0u64;
        let mut t36500 = 0u64;
        let mut t36600 = 0u64;
        let mut t36700 = 0u64;
        let mut t36800 = 0u64;
        let mut t36900 = 0u64;
        let mut t36a00 = 0u64;
        let mut t36b00 = 0u64;
        let mut t36c00 = 0u64;
        let mut t36d00 = 0u64;
        let mut t36e00 = 0u64;
        let mut t36f00 = 0u64;
        let mut t37000 = 0u64;
        let mut t37900 = 0u64;
        t37900 = (0u64) & 0xffffffffu64;
        t35c00 = ((f32::from_bits(c.r(796, 4) as u32)
            < f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(796, 4) as u32)
            == f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)) as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)
            <= f32::from_bits(c.r(796, 4) as u32)) as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(796, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32).is_nan() as u64) & 0xffu64;
        t36400 = (((t36100 >> 0) & 0xffu64) | ((t36200 >> 0) & 0xffu64)) & 0xffu64;
        t36500 = (c.r(180, 4) & 268435455u64) & 0xffffffffu64;
        t36600 = ((t35c00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36700 = (if 31u64 >= 32 {
            0
        } else {
            ((t36600 >> 0) & 0xffffffffu64) << (31u64 as u32)
        }) & 0xffffffffu64;
        t36800 =
            (((t36500 >> 0) & 0xffffffffu64) | ((t36700 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36900 = ((t35e00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36a00 = (if 30u64 >= 32 {
            0
        } else {
            ((t36900 >> 0) & 0xffffffffu64) << (30u64 as u32)
        }) & 0xffffffffu64;
        t36b00 =
            (((t36800 >> 0) & 0xffffffffu64) | ((t36a00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36c00 = ((t36000 >> 0) & 0xffu64) & 0xffffffffu64;
        t36d00 = (if 29u64 >= 32 {
            0
        } else {
            ((t36c00 >> 0) & 0xffffffffu64) << (29u64 as u32)
        }) & 0xffffffffu64;
        t36e00 =
            (((t36b00 >> 0) & 0xffffffffu64) | ((t36d00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36f00 = ((t36400 >> 0) & 0xffu64) & 0xffffffffu64;
        t37000 = (if 28u64 >= 32 {
            0
        } else {
            ((t36f00 >> 0) & 0xffffffffu64) << (28u64 as u32)
        }) & 0xffffffffu64;
        let value = (((t36e00 >> 0) & 0xffffffffu64) | ((t37000 >> 0) & 0xffffffffu64));
        c.w(180, 4, value);
    }
    {
        // 0x0000262c it mi
        let mut t184c00 = 0u64;
        t184c00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x0000262e add.mi r7,r7,r0
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x2630u32;
        }
        let value = ((c.r(60, 4) as u128 + c.r(32, 4) as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(60, 4), c.r(32, 4), 4, false);
        c.w(103, 1, value);
        let value = (c.r(60, 4)).wrapping_add(c.r(32, 4));
        c.w(60, 4, value);
        let value = (signed(c.r(60, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(60, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    0x2630
}
pub fn b_2630(c: &mut Machine) -> u32 {
    {
        // 0x00002630 add.w r1,r4,r1, lsl #0x2
        let mut t189200 = 0u64;
        let mut t189300 = 0u64;
        let mut t189500 = 0u64;
        let mut t189800 = 0u64;
        let mut t1a0b00 = 0u64;
        t189200 = ((32u64).wrapping_sub(2u64)) & 0xffffffffu64;
        t189300 = (if ((t189200 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(36, 4)) >> (((t189200 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t189500 = (((t189300 >> 0) & 0xffffffffu64) & 1u64) & 0xffffffffu64;
        let value = ((t189500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        t189800 = (if 2u64 >= 32 {
            0
        } else {
            (c.r(36, 4)) << (2u64 as u32)
        }) & 0xffffffffu64;
        t1a0b00 = ((t189800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        let value = ((c.r(48, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00002634 add.w r7,r4,r7, lsl #0x2
        let mut t189200 = 0u64;
        let mut t189300 = 0u64;
        let mut t189500 = 0u64;
        let mut t189800 = 0u64;
        let mut t1a0b00 = 0u64;
        t189200 = ((32u64).wrapping_sub(2u64)) & 0xffffffffu64;
        t189300 = (if ((t189200 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(60, 4)) >> (((t189200 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t189500 = (((t189300 >> 0) & 0xffffffffu64) & 1u64) & 0xffffffffu64;
        let value = ((t189500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        t189800 = (if 2u64 >= 32 {
            0
        } else {
            (c.r(60, 4)) << (2u64 as u32)
        }) & 0xffffffffu64;
        t1a0b00 = ((t189800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        let value = ((c.r(48, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(60, 4, value);
        let value = (signed(c.r(60, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(60, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00002638 vldr.32 s12,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x0000263c vmrs apsr,fpscr
        let mut t6bd00 = 0u64;
        let mut t6bf00 = 0u64;
        let mut t6c000 = 0u64;
        let mut t6c200 = 0u64;
        let mut t6c300 = 0u64;
        let mut t6c500 = 0u64;
        let mut t6c600 = 0u64;
        t6bd00 = (if 31u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t6bd00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(96, 1, value);
        t6bf00 = (if 30u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (30u64 as u32)
        }) & 0xffffffffu64;
        t6c000 = (((t6bf00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c000 >> 0) & 0xffu64) & 1u64);
        c.w(97, 1, value);
        t6c200 = (if 29u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (29u64 as u32)
        }) & 0xffffffffu64;
        t6c300 = (((t6c200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c300 >> 0) & 0xffu64) & 1u64);
        c.w(98, 1, value);
        t6c500 = (if 28u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (28u64 as u32)
        }) & 0xffffffffu64;
        t6c600 = (((t6c500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c600 >> 0) & 0xffu64) & 1u64);
        c.w(99, 1, value);
    }
    {
        // 0x00002640 vldr.32 s11,[r7]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(60, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x00002644 vsub.f32 s11,s11,s12
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00002648 ite pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x0000264a vstr.pl.32 s7,[r5,#0x3c]
        let mut tf00 = 0u64;
        let mut t2600 = 0u64;
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(60u64)) & 0xffffffffu64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2600 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2600 >> 0) & 0xffu64) != 0 {
            return 0x264eu32;
        }
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(796, 4));
    }
    0x264e
}
pub fn b_264e(c: &mut Machine) -> u32 {
    {
        // 0x0000264e str.w.mi r9,[r5,#0x3c]
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        let mut t19c800 = 0u64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x2652u32;
        }
        t19c800 = ((c.r(52, 4)).wrapping_add(60u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(68, 4));
    }
    0x2652
}
pub fn b_2652(c: &mut Machine) -> u32 {
    {
        // 0x00002652 vfma.f32 s12,s14,s11
        let value = f32::from_bits(c.r(824, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(812, 4) as u32),
                f32::from_bits(c.r(816, 4) as u32),
            )
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00002656 vstr.32 s12,[r5,#0x24]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x0000265a b 0x00002576
        return 0x2576u32;
    }
    0x265c
}
pub fn b_265c(c: &mut Machine) -> u32 {
    {
        // 0x0000265c cmp r1,#0x0
        let mut t1a6800 = 0u64;
        let value = (0u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 0u64, 4, true);
        c.w(103, 1, value);
        t1a6800 = ((c.r(36, 4)).wrapping_sub(0u64)) & 0xffffffffu64;
        let value = (signed(((t1a6800 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a6800 >> 0) & 0xffffffffu64) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x0000265e ldr r7,[sp,#0x20]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(60, 4, value);
    }
    {
        // 0x00002660 vsub.f32 s5,s15,s8
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(800, 4) as u32))
            .to_bits() as u64;
        c.w(788, 4, value);
    }
    {
        // 0x00002664 it lt
        let mut t185c00 = 0u64;
        t185c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
    }
    {
        // 0x00002666 add.lt r1,r1,r0
        let mut t1d00 = 0u64;
        let mut t2700 = 0u64;
        t1d00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2700 = ((((t1d00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x2668u32;
        }
        let value = ((c.r(36, 4) as u128 + c.r(32, 4) as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), c.r(32, 4), 4, false);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_add(c.r(32, 4));
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    0x2668
}
pub fn b_2668(c: &mut Machine) -> u32 {
    {
        // 0x00002668 subs r7,r2,r7
        let value = (c.r(60, 4) <= c.r(40, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(40, 4), c.r(60, 4), 4, true);
        c.w(103, 1, value);
        let value = (c.r(40, 4)).wrapping_sub(c.r(60, 4));
        c.w(60, 4, value);
        let value = (signed(c.r(60, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(60, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x0000266a it mi
        let mut t184c00 = 0u64;
        t184c00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x0000266c add.mi r7,r7,r0
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x266eu32;
        }
        let value = ((c.r(60, 4) as u128 + c.r(32, 4) as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(60, 4), c.r(32, 4), 4, false);
        c.w(103, 1, value);
        let value = (c.r(60, 4)).wrapping_add(c.r(32, 4));
        c.w(60, 4, value);
        let value = (signed(c.r(60, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(60, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    0x266e
}
pub fn b_266e(c: &mut Machine) -> u32 {
    {
        // 0x0000266e add.w r1,r4,r1, lsl #0x2
        let mut t189200 = 0u64;
        let mut t189300 = 0u64;
        let mut t189500 = 0u64;
        let mut t189800 = 0u64;
        let mut t1a0b00 = 0u64;
        t189200 = ((32u64).wrapping_sub(2u64)) & 0xffffffffu64;
        t189300 = (if ((t189200 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(36, 4)) >> (((t189200 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t189500 = (((t189300 >> 0) & 0xffffffffu64) & 1u64) & 0xffffffffu64;
        let value = ((t189500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        t189800 = (if 2u64 >= 32 {
            0
        } else {
            (c.r(36, 4)) << (2u64 as u32)
        }) & 0xffffffffu64;
        t1a0b00 = ((t189800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        let value = ((c.r(48, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00002672 vcmpe.f32 s5,#0
        let mut t35c00 = 0u64;
        let mut t35e00 = 0u64;
        let mut t36000 = 0u64;
        let mut t36100 = 0u64;
        let mut t36200 = 0u64;
        let mut t36400 = 0u64;
        let mut t36500 = 0u64;
        let mut t36600 = 0u64;
        let mut t36700 = 0u64;
        let mut t36800 = 0u64;
        let mut t36900 = 0u64;
        let mut t36a00 = 0u64;
        let mut t36b00 = 0u64;
        let mut t36c00 = 0u64;
        let mut t36d00 = 0u64;
        let mut t36e00 = 0u64;
        let mut t36f00 = 0u64;
        let mut t37000 = 0u64;
        let mut t37900 = 0u64;
        t37900 = (0u64) & 0xffffffffu64;
        t35c00 = ((f32::from_bits(c.r(788, 4) as u32)
            < f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(788, 4) as u32)
            == f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)) as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)
            <= f32::from_bits(c.r(788, 4) as u32)) as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(788, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32).is_nan() as u64) & 0xffu64;
        t36400 = (((t36100 >> 0) & 0xffu64) | ((t36200 >> 0) & 0xffu64)) & 0xffu64;
        t36500 = (c.r(180, 4) & 268435455u64) & 0xffffffffu64;
        t36600 = ((t35c00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36700 = (if 31u64 >= 32 {
            0
        } else {
            ((t36600 >> 0) & 0xffffffffu64) << (31u64 as u32)
        }) & 0xffffffffu64;
        t36800 =
            (((t36500 >> 0) & 0xffffffffu64) | ((t36700 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36900 = ((t35e00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36a00 = (if 30u64 >= 32 {
            0
        } else {
            ((t36900 >> 0) & 0xffffffffu64) << (30u64 as u32)
        }) & 0xffffffffu64;
        t36b00 =
            (((t36800 >> 0) & 0xffffffffu64) | ((t36a00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36c00 = ((t36000 >> 0) & 0xffu64) & 0xffffffffu64;
        t36d00 = (if 29u64 >= 32 {
            0
        } else {
            ((t36c00 >> 0) & 0xffffffffu64) << (29u64 as u32)
        }) & 0xffffffffu64;
        t36e00 =
            (((t36b00 >> 0) & 0xffffffffu64) | ((t36d00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36f00 = ((t36400 >> 0) & 0xffu64) & 0xffffffffu64;
        t37000 = (if 28u64 >= 32 {
            0
        } else {
            ((t36f00 >> 0) & 0xffffffffu64) << (28u64 as u32)
        }) & 0xffffffffu64;
        let value = (((t36e00 >> 0) & 0xffffffffu64) | ((t37000 >> 0) & 0xffffffffu64));
        c.w(180, 4, value);
    }
    {
        // 0x00002676 add.w r7,r4,r7, lsl #0x2
        let mut t189200 = 0u64;
        let mut t189300 = 0u64;
        let mut t189500 = 0u64;
        let mut t189800 = 0u64;
        let mut t1a0b00 = 0u64;
        t189200 = ((32u64).wrapping_sub(2u64)) & 0xffffffffu64;
        t189300 = (if ((t189200 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(60, 4)) >> (((t189200 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t189500 = (((t189300 >> 0) & 0xffffffffu64) & 1u64) & 0xffffffffu64;
        let value = ((t189500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        t189800 = (if 2u64 >= 32 {
            0
        } else {
            (c.r(60, 4)) << (2u64 as u32)
        }) & 0xffffffffu64;
        t1a0b00 = ((t189800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        let value = ((c.r(48, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(60, 4, value);
        let value = (signed(c.r(60, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(60, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x0000267a vldr.32 s13,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x0000267e vldr.32 s6,[r7]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(60, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x00002682 vmrs apsr,fpscr
        let mut t6bd00 = 0u64;
        let mut t6bf00 = 0u64;
        let mut t6c000 = 0u64;
        let mut t6c200 = 0u64;
        let mut t6c300 = 0u64;
        let mut t6c500 = 0u64;
        let mut t6c600 = 0u64;
        t6bd00 = (if 31u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t6bd00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(96, 1, value);
        t6bf00 = (if 30u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (30u64 as u32)
        }) & 0xffffffffu64;
        t6c000 = (((t6bf00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c000 >> 0) & 0xffu64) & 1u64);
        c.w(97, 1, value);
        t6c200 = (if 29u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (29u64 as u32)
        }) & 0xffffffffu64;
        t6c300 = (((t6c200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c300 >> 0) & 0xffu64) & 1u64);
        c.w(98, 1, value);
        t6c500 = (if 28u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (28u64 as u32)
        }) & 0xffffffffu64;
        t6c600 = (((t6c500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c600 >> 0) & 0xffu64) & 1u64);
        c.w(99, 1, value);
    }
    {
        // 0x00002686 vsub.f32 s6,s6,s13
        let value = (f32::from_bits(c.r(792, 4) as u32) - f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(792, 4, value);
    }
    {
        // 0x0000268a ite pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x0000268c vstr.pl.32 s5,[r3,#0x10]
        let mut tf00 = 0u64;
        let mut t2600 = 0u64;
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2600 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2600 >> 0) & 0xffu64) != 0 {
            return 0x2690u32;
        }
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(788, 4));
    }
    0x2690
}
pub fn b_2690(c: &mut Machine) -> u32 {
    {
        // 0x00002690 vstr.mi.32 s7,[r3,#0x10]
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x2694u32;
        }
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(796, 4));
    }
    0x2694
}
pub fn b_2694(c: &mut Machine) -> u32 {
    {
        // 0x00002694 vfma.f32 s13,s15,s6
        let value = f32::from_bits(c.r(828, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(792, 4) as u32),
                f32::from_bits(c.r(820, 4) as u32),
            )
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00002698 vstr.32 s13,[r3,#-0x8]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(44, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x0000269c b.w 0x00001e60
        return 0x1e60u32;
    }
    0x26a0
}
pub fn b_26a0(c: &mut Machine) -> u32 {
    {
        // 0x000026a0 cmp r1,#0x0
        let mut t1a6800 = 0u64;
        let value = (0u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 0u64, 4, true);
        c.w(103, 1, value);
        t1a6800 = ((c.r(36, 4)).wrapping_sub(0u64)) & 0xffffffffu64;
        let value = (signed(((t1a6800 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a6800 >> 0) & 0xffffffffu64) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x000026a2 ldr r7,[sp,#0x8]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(60, 4, value);
    }
    {
        // 0x000026a4 vsub.f32 s5,s13,s8
        let value = (f32::from_bits(c.r(820, 4) as u32) - f32::from_bits(c.r(800, 4) as u32))
            .to_bits() as u64;
        c.w(788, 4, value);
    }
    {
        // 0x000026a8 it lt
        let mut t185c00 = 0u64;
        t185c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
    }
    {
        // 0x000026aa add.lt r1,r1,r0
        let mut t1d00 = 0u64;
        let mut t2700 = 0u64;
        t1d00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2700 = ((((t1d00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x26acu32;
        }
        let value = ((c.r(36, 4) as u128 + c.r(32, 4) as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), c.r(32, 4), 4, false);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_add(c.r(32, 4));
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    0x26ac
}
pub fn b_26ac(c: &mut Machine) -> u32 {
    {
        // 0x000026ac subs r7,r2,r7
        let value = (c.r(60, 4) <= c.r(40, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(40, 4), c.r(60, 4), 4, true);
        c.w(103, 1, value);
        let value = (c.r(40, 4)).wrapping_sub(c.r(60, 4));
        c.w(60, 4, value);
        let value = (signed(c.r(60, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(60, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x000026ae it mi
        let mut t184c00 = 0u64;
        t184c00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x000026b0 add.mi r7,r7,r0
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x26b2u32;
        }
        let value = ((c.r(60, 4) as u128 + c.r(32, 4) as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(60, 4), c.r(32, 4), 4, false);
        c.w(103, 1, value);
        let value = (c.r(60, 4)).wrapping_add(c.r(32, 4));
        c.w(60, 4, value);
        let value = (signed(c.r(60, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(60, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    0x26b2
}
pub fn b_26b2(c: &mut Machine) -> u32 {
    {
        // 0x000026b2 add.w r1,r4,r1, lsl #0x2
        let mut t189200 = 0u64;
        let mut t189300 = 0u64;
        let mut t189500 = 0u64;
        let mut t189800 = 0u64;
        let mut t1a0b00 = 0u64;
        t189200 = ((32u64).wrapping_sub(2u64)) & 0xffffffffu64;
        t189300 = (if ((t189200 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(36, 4)) >> (((t189200 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t189500 = (((t189300 >> 0) & 0xffffffffu64) & 1u64) & 0xffffffffu64;
        let value = ((t189500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        t189800 = (if 2u64 >= 32 {
            0
        } else {
            (c.r(36, 4)) << (2u64 as u32)
        }) & 0xffffffffu64;
        t1a0b00 = ((t189800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        let value = ((c.r(48, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x000026b6 vcmpe.f32 s5,#0
        let mut t35c00 = 0u64;
        let mut t35e00 = 0u64;
        let mut t36000 = 0u64;
        let mut t36100 = 0u64;
        let mut t36200 = 0u64;
        let mut t36400 = 0u64;
        let mut t36500 = 0u64;
        let mut t36600 = 0u64;
        let mut t36700 = 0u64;
        let mut t36800 = 0u64;
        let mut t36900 = 0u64;
        let mut t36a00 = 0u64;
        let mut t36b00 = 0u64;
        let mut t36c00 = 0u64;
        let mut t36d00 = 0u64;
        let mut t36e00 = 0u64;
        let mut t36f00 = 0u64;
        let mut t37000 = 0u64;
        let mut t37900 = 0u64;
        t37900 = (0u64) & 0xffffffffu64;
        t35c00 = ((f32::from_bits(c.r(788, 4) as u32)
            < f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(788, 4) as u32)
            == f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)) as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)
            <= f32::from_bits(c.r(788, 4) as u32)) as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(788, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32).is_nan() as u64) & 0xffu64;
        t36400 = (((t36100 >> 0) & 0xffu64) | ((t36200 >> 0) & 0xffu64)) & 0xffu64;
        t36500 = (c.r(180, 4) & 268435455u64) & 0xffffffffu64;
        t36600 = ((t35c00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36700 = (if 31u64 >= 32 {
            0
        } else {
            ((t36600 >> 0) & 0xffffffffu64) << (31u64 as u32)
        }) & 0xffffffffu64;
        t36800 =
            (((t36500 >> 0) & 0xffffffffu64) | ((t36700 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36900 = ((t35e00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36a00 = (if 30u64 >= 32 {
            0
        } else {
            ((t36900 >> 0) & 0xffffffffu64) << (30u64 as u32)
        }) & 0xffffffffu64;
        t36b00 =
            (((t36800 >> 0) & 0xffffffffu64) | ((t36a00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36c00 = ((t36000 >> 0) & 0xffu64) & 0xffffffffu64;
        t36d00 = (if 29u64 >= 32 {
            0
        } else {
            ((t36c00 >> 0) & 0xffffffffu64) << (29u64 as u32)
        }) & 0xffffffffu64;
        t36e00 =
            (((t36b00 >> 0) & 0xffffffffu64) | ((t36d00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36f00 = ((t36400 >> 0) & 0xffu64) & 0xffffffffu64;
        t37000 = (if 28u64 >= 32 {
            0
        } else {
            ((t36f00 >> 0) & 0xffffffffu64) << (28u64 as u32)
        }) & 0xffffffffu64;
        let value = (((t36e00 >> 0) & 0xffffffffu64) | ((t37000 >> 0) & 0xffffffffu64));
        c.w(180, 4, value);
    }
    {
        // 0x000026ba add.w r7,r4,r7, lsl #0x2
        let mut t189200 = 0u64;
        let mut t189300 = 0u64;
        let mut t189500 = 0u64;
        let mut t189800 = 0u64;
        let mut t1a0b00 = 0u64;
        t189200 = ((32u64).wrapping_sub(2u64)) & 0xffffffffu64;
        t189300 = (if ((t189200 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(60, 4)) >> (((t189200 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t189500 = (((t189300 >> 0) & 0xffffffffu64) & 1u64) & 0xffffffffu64;
        let value = ((t189500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        t189800 = (if 2u64 >= 32 {
            0
        } else {
            (c.r(60, 4)) << (2u64 as u32)
        }) & 0xffffffffu64;
        t1a0b00 = ((t189800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        let value = ((c.r(48, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(60, 4, value);
        let value = (signed(c.r(60, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(60, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x000026be vldr.32 s11,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x000026c2 vldr.32 s6,[r7]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(60, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x000026c6 vmrs apsr,fpscr
        let mut t6bd00 = 0u64;
        let mut t6bf00 = 0u64;
        let mut t6c000 = 0u64;
        let mut t6c200 = 0u64;
        let mut t6c300 = 0u64;
        let mut t6c500 = 0u64;
        let mut t6c600 = 0u64;
        t6bd00 = (if 31u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t6bd00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(96, 1, value);
        t6bf00 = (if 30u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (30u64 as u32)
        }) & 0xffffffffu64;
        t6c000 = (((t6bf00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c000 >> 0) & 0xffu64) & 1u64);
        c.w(97, 1, value);
        t6c200 = (if 29u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (29u64 as u32)
        }) & 0xffffffffu64;
        t6c300 = (((t6c200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c300 >> 0) & 0xffu64) & 1u64);
        c.w(98, 1, value);
        t6c500 = (if 28u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (28u64 as u32)
        }) & 0xffffffffu64;
        t6c600 = (((t6c500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c600 >> 0) & 0xffu64) & 1u64);
        c.w(99, 1, value);
    }
    {
        // 0x000026ca vsub.f32 s6,s6,s11
        let value = (f32::from_bits(c.r(792, 4) as u32) - f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(792, 4, value);
    }
    {
        // 0x000026ce ite pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x000026d0 vstr.pl.32 s5,[r3,#0x10]
        let mut tf00 = 0u64;
        let mut t2600 = 0u64;
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2600 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2600 >> 0) & 0xffu64) != 0 {
            return 0x26d4u32;
        }
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(788, 4));
    }
    0x26d4
}
pub fn b_26d4(c: &mut Machine) -> u32 {
    {
        // 0x000026d4 vstr.mi.32 s7,[r3,#0x10]
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x26d8u32;
        }
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(796, 4));
    }
    0x26d8
}
pub fn b_26d8(c: &mut Machine) -> u32 {
    {
        // 0x000026d8 vfma.f32 s11,s13,s6
        let value = f32::from_bits(c.r(820, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(792, 4) as u32),
                f32::from_bits(c.r(812, 4) as u32),
            )
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x000026dc vstr.32 s11,[r3,#-0x8]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(44, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(812, 4));
    }
    {
        // 0x000026e0 b 0x00002060
        return 0x2060u32;
    }
    0x26e2
}
pub fn b_26e2(c: &mut Machine) -> u32 {
    {
        // 0x000026e2 cmp r0,#0x0
        let mut t1a6800 = 0u64;
        let value = (0u64 <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 0u64, 4, true);
        c.w(103, 1, value);
        t1a6800 = ((c.r(32, 4)).wrapping_sub(0u64)) & 0xffffffffu64;
        let value = (signed(((t1a6800 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a6800 >> 0) & 0xffffffffu64) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x000026e4 vsub.f32 s7,s15,s8
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(800, 4) as u32))
            .to_bits() as u64;
        c.w(796, 4, value);
    }
    {
        // 0x000026e8 it lt
        let mut t185c00 = 0u64;
        t185c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
    }
    {
        // 0x000026ea add.lt r0,r0,r4
        let mut t1d00 = 0u64;
        let mut t2700 = 0u64;
        t1d00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2700 = ((((t1d00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x26ecu32;
        }
        let value = ((c.r(32, 4) as u128 + c.r(48, 4) as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), c.r(48, 4), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(c.r(48, 4));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    0x26ec
}
pub fn b_26ec(c: &mut Machine) -> u32 {
    {
        // 0x000026ec subs.w r12,r1,r8
        let mut t1fe000 = 0u64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        t1fe000 = (c.r(64, 4)) & 0xffffffffu64;
        let value = (((t1fe000 >> 0) & 0xffffffffu64) <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), ((t1fe000 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(((t1fe000 >> 0) & 0xffffffffu64));
        c.w(80, 4, value);
        let value = (signed(c.r(80, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(80, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x000026f0 vcmpe.f32 s7,#0
        let mut t35c00 = 0u64;
        let mut t35e00 = 0u64;
        let mut t36000 = 0u64;
        let mut t36100 = 0u64;
        let mut t36200 = 0u64;
        let mut t36400 = 0u64;
        let mut t36500 = 0u64;
        let mut t36600 = 0u64;
        let mut t36700 = 0u64;
        let mut t36800 = 0u64;
        let mut t36900 = 0u64;
        let mut t36a00 = 0u64;
        let mut t36b00 = 0u64;
        let mut t36c00 = 0u64;
        let mut t36d00 = 0u64;
        let mut t36e00 = 0u64;
        let mut t36f00 = 0u64;
        let mut t37000 = 0u64;
        let mut t37900 = 0u64;
        t37900 = (0u64) & 0xffffffffu64;
        t35c00 = ((f32::from_bits(c.r(796, 4) as u32)
            < f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(796, 4) as u32)
            == f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)) as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)
            <= f32::from_bits(c.r(796, 4) as u32)) as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(796, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32).is_nan() as u64) & 0xffu64;
        t36400 = (((t36100 >> 0) & 0xffu64) | ((t36200 >> 0) & 0xffu64)) & 0xffu64;
        t36500 = (c.r(180, 4) & 268435455u64) & 0xffffffffu64;
        t36600 = ((t35c00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36700 = (if 31u64 >= 32 {
            0
        } else {
            ((t36600 >> 0) & 0xffffffffu64) << (31u64 as u32)
        }) & 0xffffffffu64;
        t36800 =
            (((t36500 >> 0) & 0xffffffffu64) | ((t36700 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36900 = ((t35e00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36a00 = (if 30u64 >= 32 {
            0
        } else {
            ((t36900 >> 0) & 0xffffffffu64) << (30u64 as u32)
        }) & 0xffffffffu64;
        t36b00 =
            (((t36800 >> 0) & 0xffffffffu64) | ((t36a00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36c00 = ((t36000 >> 0) & 0xffu64) & 0xffffffffu64;
        t36d00 = (if 29u64 >= 32 {
            0
        } else {
            ((t36c00 >> 0) & 0xffffffffu64) << (29u64 as u32)
        }) & 0xffffffffu64;
        t36e00 =
            (((t36b00 >> 0) & 0xffffffffu64) | ((t36d00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36f00 = ((t36400 >> 0) & 0xffu64) & 0xffffffffu64;
        t37000 = (if 28u64 >= 32 {
            0
        } else {
            ((t36f00 >> 0) & 0xffffffffu64) << (28u64 as u32)
        }) & 0xffffffffu64;
        let value = (((t36e00 >> 0) & 0xffffffffu64) | ((t37000 >> 0) & 0xffffffffu64));
        c.w(180, 4, value);
    }
    {
        // 0x000026f4 it mi
        let mut t184c00 = 0u64;
        t184c00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x000026f6 add.mi r12,r4
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x26f8u32;
        }
        let value = (c.r(80, 4)).wrapping_add(c.r(48, 4));
        c.w(80, 4, value);
    }
    0x26f8
}
pub fn b_26f8(c: &mut Machine) -> u32 {
    {
        // 0x000026f8 add.w r0,r7,r0, lsl #0x2
        let mut t189200 = 0u64;
        let mut t189300 = 0u64;
        let mut t189500 = 0u64;
        let mut t189800 = 0u64;
        let mut t1a0b00 = 0u64;
        t189200 = ((32u64).wrapping_sub(2u64)) & 0xffffffffu64;
        t189300 = (if ((t189200 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(32, 4)) >> (((t189200 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t189500 = (((t189300 >> 0) & 0xffffffffu64) & 1u64) & 0xffffffffu64;
        let value = ((t189500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        t189800 = (if 2u64 >= 32 {
            0
        } else {
            (c.r(32, 4)) << (2u64 as u32)
        }) & 0xffffffffu64;
        t1a0b00 = ((t189800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        let value = ((c.r(60, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(60, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(60, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x000026fc add.w r12,r7,r12, lsl #0x2
        let mut t189200 = 0u64;
        let mut t189300 = 0u64;
        let mut t189500 = 0u64;
        let mut t189800 = 0u64;
        let mut t1a0b00 = 0u64;
        t189200 = ((32u64).wrapping_sub(2u64)) & 0xffffffffu64;
        t189300 = (if ((t189200 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(80, 4)) >> (((t189200 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t189500 = (((t189300 >> 0) & 0xffffffffu64) & 1u64) & 0xffffffffu64;
        let value = ((t189500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        t189800 = (if 2u64 >= 32 {
            0
        } else {
            (c.r(80, 4)) << (2u64 as u32)
        }) & 0xffffffffu64;
        t1a0b00 = ((t189800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        let value = ((c.r(60, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(60, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(60, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(80, 4, value);
        let value = (signed(c.r(80, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(80, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00002700 vldr.32 s12,[r0]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(32, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x00002704 vmrs apsr,fpscr
        let mut t6bd00 = 0u64;
        let mut t6bf00 = 0u64;
        let mut t6c000 = 0u64;
        let mut t6c200 = 0u64;
        let mut t6c300 = 0u64;
        let mut t6c500 = 0u64;
        let mut t6c600 = 0u64;
        t6bd00 = (if 31u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t6bd00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(96, 1, value);
        t6bf00 = (if 30u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (30u64 as u32)
        }) & 0xffffffffu64;
        t6c000 = (((t6bf00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c000 >> 0) & 0xffu64) & 1u64);
        c.w(97, 1, value);
        t6c200 = (if 29u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (29u64 as u32)
        }) & 0xffffffffu64;
        t6c300 = (((t6c200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c300 >> 0) & 0xffu64) & 1u64);
        c.w(98, 1, value);
        t6c500 = (if 28u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (28u64 as u32)
        }) & 0xffffffffu64;
        t6c600 = (((t6c500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c600 >> 0) & 0xffu64) & 1u64);
        c.w(99, 1, value);
    }
    {
        // 0x00002708 vldr.32 s11,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x0000270c vsub.f32 s11,s11,s12
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00002710 ite pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x00002712 vstr.pl.32 s7,[r5,#0x9c]
        let mut tf00 = 0u64;
        let mut t2600 = 0u64;
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(156u64)) & 0xffffffffu64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2600 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2600 >> 0) & 0xffu64) != 0 {
            return 0x2716u32;
        }
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(796, 4));
    }
    0x2716
}
pub fn b_2716(c: &mut Machine) -> u32 {
    {
        // 0x00002716 str.w.mi r10,[r5,#0x9c]
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        let mut t19c800 = 0u64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x271au32;
        }
        t19c800 = ((c.r(52, 4)).wrapping_add(156u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(72, 4));
    }
    0x271a
}
pub fn b_271a(c: &mut Machine) -> u32 {
    {
        // 0x0000271a vfma.f32 s12,s15,s11
        let value = f32::from_bits(c.r(828, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(812, 4) as u32),
                f32::from_bits(c.r(816, 4) as u32),
            )
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x0000271e vstr.32 s12,[r5,#0x84]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(132u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x00002722 b 0x0000221a
        return 0x221au32;
    }
    0x2724
}
pub fn b_2734(c: &mut Machine) -> u32 {
    {
        // 0x00002734 cmp r1,#0x0
        let mut t1a6800 = 0u64;
        let value = (0u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 0u64, 4, true);
        c.w(103, 1, value);
        t1a6800 = ((c.r(36, 4)).wrapping_sub(0u64)) & 0xffffffffu64;
        let value = (signed(((t1a6800 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a6800 >> 0) & 0xffffffffu64) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x00002736 ldr r7,[sp,#0x8]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(60, 4, value);
    }
    {
        // 0x00002738 vsub.f32 s6,s15,s8
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(800, 4) as u32))
            .to_bits() as u64;
        c.w(792, 4, value);
    }
    {
        // 0x0000273c it lt
        let mut t185c00 = 0u64;
        t185c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
    }
    {
        // 0x0000273e add.lt r1,r1,r0
        let mut t1d00 = 0u64;
        let mut t2700 = 0u64;
        t1d00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2700 = ((((t1d00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x2740u32;
        }
        let value = ((c.r(36, 4) as u128 + c.r(32, 4) as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), c.r(32, 4), 4, false);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_add(c.r(32, 4));
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    0x2740
}
pub fn b_2740(c: &mut Machine) -> u32 {
    {
        // 0x00002740 subs r7,r2,r7
        let value = (c.r(60, 4) <= c.r(40, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(40, 4), c.r(60, 4), 4, true);
        c.w(103, 1, value);
        let value = (c.r(40, 4)).wrapping_sub(c.r(60, 4));
        c.w(60, 4, value);
        let value = (signed(c.r(60, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(60, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x00002742 it mi
        let mut t184c00 = 0u64;
        t184c00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x00002744 add.mi r7,r7,r0
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x2746u32;
        }
        let value = ((c.r(60, 4) as u128 + c.r(32, 4) as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(60, 4), c.r(32, 4), 4, false);
        c.w(103, 1, value);
        let value = (c.r(60, 4)).wrapping_add(c.r(32, 4));
        c.w(60, 4, value);
        let value = (signed(c.r(60, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(60, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    0x2746
}
pub fn b_2746(c: &mut Machine) -> u32 {
    {
        // 0x00002746 add.w r1,r4,r1, lsl #0x2
        let mut t189200 = 0u64;
        let mut t189300 = 0u64;
        let mut t189500 = 0u64;
        let mut t189800 = 0u64;
        let mut t1a0b00 = 0u64;
        t189200 = ((32u64).wrapping_sub(2u64)) & 0xffffffffu64;
        t189300 = (if ((t189200 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(36, 4)) >> (((t189200 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t189500 = (((t189300 >> 0) & 0xffffffffu64) & 1u64) & 0xffffffffu64;
        let value = ((t189500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        t189800 = (if 2u64 >= 32 {
            0
        } else {
            (c.r(36, 4)) << (2u64 as u32)
        }) & 0xffffffffu64;
        t1a0b00 = ((t189800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        let value = ((c.r(48, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x0000274a vcmpe.f32 s6,#0
        let mut t35c00 = 0u64;
        let mut t35e00 = 0u64;
        let mut t36000 = 0u64;
        let mut t36100 = 0u64;
        let mut t36200 = 0u64;
        let mut t36400 = 0u64;
        let mut t36500 = 0u64;
        let mut t36600 = 0u64;
        let mut t36700 = 0u64;
        let mut t36800 = 0u64;
        let mut t36900 = 0u64;
        let mut t36a00 = 0u64;
        let mut t36b00 = 0u64;
        let mut t36c00 = 0u64;
        let mut t36d00 = 0u64;
        let mut t36e00 = 0u64;
        let mut t36f00 = 0u64;
        let mut t37000 = 0u64;
        let mut t37900 = 0u64;
        t37900 = (0u64) & 0xffffffffu64;
        t35c00 = ((f32::from_bits(c.r(792, 4) as u32)
            < f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(792, 4) as u32)
            == f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)) as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)
            <= f32::from_bits(c.r(792, 4) as u32)) as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(792, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32).is_nan() as u64) & 0xffu64;
        t36400 = (((t36100 >> 0) & 0xffu64) | ((t36200 >> 0) & 0xffu64)) & 0xffu64;
        t36500 = (c.r(180, 4) & 268435455u64) & 0xffffffffu64;
        t36600 = ((t35c00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36700 = (if 31u64 >= 32 {
            0
        } else {
            ((t36600 >> 0) & 0xffffffffu64) << (31u64 as u32)
        }) & 0xffffffffu64;
        t36800 =
            (((t36500 >> 0) & 0xffffffffu64) | ((t36700 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36900 = ((t35e00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36a00 = (if 30u64 >= 32 {
            0
        } else {
            ((t36900 >> 0) & 0xffffffffu64) << (30u64 as u32)
        }) & 0xffffffffu64;
        t36b00 =
            (((t36800 >> 0) & 0xffffffffu64) | ((t36a00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36c00 = ((t36000 >> 0) & 0xffu64) & 0xffffffffu64;
        t36d00 = (if 29u64 >= 32 {
            0
        } else {
            ((t36c00 >> 0) & 0xffffffffu64) << (29u64 as u32)
        }) & 0xffffffffu64;
        t36e00 =
            (((t36b00 >> 0) & 0xffffffffu64) | ((t36d00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36f00 = ((t36400 >> 0) & 0xffu64) & 0xffffffffu64;
        t37000 = (if 28u64 >= 32 {
            0
        } else {
            ((t36f00 >> 0) & 0xffffffffu64) << (28u64 as u32)
        }) & 0xffffffffu64;
        let value = (((t36e00 >> 0) & 0xffffffffu64) | ((t37000 >> 0) & 0xffffffffu64));
        c.w(180, 4, value);
    }
    {
        // 0x0000274e add.w r7,r4,r7, lsl #0x2
        let mut t189200 = 0u64;
        let mut t189300 = 0u64;
        let mut t189500 = 0u64;
        let mut t189800 = 0u64;
        let mut t1a0b00 = 0u64;
        t189200 = ((32u64).wrapping_sub(2u64)) & 0xffffffffu64;
        t189300 = (if ((t189200 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(60, 4)) >> (((t189200 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t189500 = (((t189300 >> 0) & 0xffffffffu64) & 1u64) & 0xffffffffu64;
        let value = ((t189500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        t189800 = (if 2u64 >= 32 {
            0
        } else {
            (c.r(60, 4)) << (2u64 as u32)
        }) & 0xffffffffu64;
        t1a0b00 = ((t189800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        let value = ((c.r(48, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(60, 4, value);
        let value = (signed(c.r(60, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(60, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00002752 vldr.32 s14,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00002756 vldr.32 s11,[r7]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(60, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x0000275a vmrs apsr,fpscr
        let mut t6bd00 = 0u64;
        let mut t6bf00 = 0u64;
        let mut t6c000 = 0u64;
        let mut t6c200 = 0u64;
        let mut t6c300 = 0u64;
        let mut t6c500 = 0u64;
        let mut t6c600 = 0u64;
        t6bd00 = (if 31u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t6bd00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(96, 1, value);
        t6bf00 = (if 30u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (30u64 as u32)
        }) & 0xffffffffu64;
        t6c000 = (((t6bf00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c000 >> 0) & 0xffu64) & 1u64);
        c.w(97, 1, value);
        t6c200 = (if 29u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (29u64 as u32)
        }) & 0xffffffffu64;
        t6c300 = (((t6c200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c300 >> 0) & 0xffu64) & 1u64);
        c.w(98, 1, value);
        t6c500 = (if 28u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (28u64 as u32)
        }) & 0xffffffffu64;
        t6c600 = (((t6c500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c600 >> 0) & 0xffu64) & 1u64);
        c.w(99, 1, value);
    }
    {
        // 0x0000275e vsub.f32 s11,s11,s14
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00002762 ite pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x00002764 vstr.pl.32 s6,[r3,#0x10]
        let mut tf00 = 0u64;
        let mut t2600 = 0u64;
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2600 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2600 >> 0) & 0xffu64) != 0 {
            return 0x2768u32;
        }
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(792, 4));
    }
    0x2768
}
pub fn b_2768(c: &mut Machine) -> u32 {
    {
        // 0x00002768 vstr.mi.32 s7,[r3,#0x10]
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x276cu32;
        }
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(796, 4));
    }
    0x276c
}
pub fn b_276c(c: &mut Machine) -> u32 {
    {
        // 0x0000276c vfma.f32 s14,s15,s11
        let value = f32::from_bits(c.r(828, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(812, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00002770 vstr.32 s14,[r3,#-0x8]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(44, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x00002774 b 0x00002410
        return 0x2410u32;
    }
    0x2776
}
pub fn b_2776(c: &mut Machine) -> u32 {
    {
        // 0x00002776 cmp r0,r12
        let mut t1a7000 = 0u64;
        let value = (c.r(80, 4) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), c.r(80, 4), 4, true);
        c.w(103, 1, value);
        t1a7000 = ((c.r(32, 4)).wrapping_sub(c.r(80, 4))) & 0xffffffffu64;
        let value = (signed(((t1a7000 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a7000 >> 0) & 0xffffffffu64) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x00002778 str.w r3,[r6,#0xffc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(4092u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x0000277c ble.w 0x000025e0
        let mut t184000 = 0u64;
        let mut t184200 = 0u64;
        t184000 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t184200 = (((c.r(97, 1) != 0) | (((t184000 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t184200 >> 0) & 0xffu64) != 0 {
            return 0x25e0u32;
        }
    }
    0x2780
}
pub fn b_2780(c: &mut Machine) -> u32 {
    {
        // 0x00002780 vldr.32 s13,[r5,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00002784 add.w r12,r12,#0x1
        let mut t187d00 = 0u64;
        t187d00 = (1u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = ((c.r(80, 4) as u128 + ((t187d00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(80, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(80, 4)).wrapping_add(((t187d00 >> 0) & 0xffffffffu64));
        c.w(80, 4, value);
        let value = (signed(c.r(80, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(80, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00002788 vcmpe.f32 s13,s9
        let mut t35c00 = 0u64;
        let mut t35e00 = 0u64;
        let mut t36000 = 0u64;
        let mut t36100 = 0u64;
        let mut t36200 = 0u64;
        let mut t36400 = 0u64;
        let mut t36500 = 0u64;
        let mut t36600 = 0u64;
        let mut t36700 = 0u64;
        let mut t36800 = 0u64;
        let mut t36900 = 0u64;
        let mut t36a00 = 0u64;
        let mut t36b00 = 0u64;
        let mut t36c00 = 0u64;
        let mut t36d00 = 0u64;
        let mut t36e00 = 0u64;
        let mut t36f00 = 0u64;
        let mut t37000 = 0u64;
        t35c00 = ((f32::from_bits(c.r(820, 4) as u32) < f32::from_bits(c.r(804, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(820, 4) as u32) == f32::from_bits(c.r(804, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(804, 4) as u32) <= f32::from_bits(c.r(820, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(820, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(804, 4) as u32).is_nan() as u64) & 0xffu64;
        t36400 = (((t36100 >> 0) & 0xffu64) | ((t36200 >> 0) & 0xffu64)) & 0xffu64;
        t36500 = (c.r(180, 4) & 268435455u64) & 0xffffffffu64;
        t36600 = ((t35c00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36700 = (if 31u64 >= 32 {
            0
        } else {
            ((t36600 >> 0) & 0xffffffffu64) << (31u64 as u32)
        }) & 0xffffffffu64;
        t36800 =
            (((t36500 >> 0) & 0xffffffffu64) | ((t36700 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36900 = ((t35e00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36a00 = (if 30u64 >= 32 {
            0
        } else {
            ((t36900 >> 0) & 0xffffffffu64) << (30u64 as u32)
        }) & 0xffffffffu64;
        t36b00 =
            (((t36800 >> 0) & 0xffffffffu64) | ((t36a00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36c00 = ((t36000 >> 0) & 0xffu64) & 0xffffffffu64;
        t36d00 = (if 29u64 >= 32 {
            0
        } else {
            ((t36c00 >> 0) & 0xffffffffu64) << (29u64 as u32)
        }) & 0xffffffffu64;
        t36e00 =
            (((t36b00 >> 0) & 0xffffffffu64) | ((t36d00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36f00 = ((t36400 >> 0) & 0xffu64) & 0xffffffffu64;
        t37000 = (if 28u64 >= 32 {
            0
        } else {
            ((t36f00 >> 0) & 0xffffffffu64) << (28u64 as u32)
        }) & 0xffffffffu64;
        let value = (((t36e00 >> 0) & 0xffffffffu64) | ((t37000 >> 0) & 0xffffffffu64));
        c.w(180, 4, value);
    }
    {
        // 0x0000278c str.w r12,[r5,#0x44]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x00002790 vmrs apsr,fpscr
        let mut t6bd00 = 0u64;
        let mut t6bf00 = 0u64;
        let mut t6c000 = 0u64;
        let mut t6c200 = 0u64;
        let mut t6c300 = 0u64;
        let mut t6c500 = 0u64;
        let mut t6c600 = 0u64;
        t6bd00 = (if 31u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t6bd00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(96, 1, value);
        t6bf00 = (if 30u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (30u64 as u32)
        }) & 0xffffffffu64;
        t6c000 = (((t6bf00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c000 >> 0) & 0xffu64) & 1u64);
        c.w(97, 1, value);
        t6c200 = (if 29u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (29u64 as u32)
        }) & 0xffffffffu64;
        t6c300 = (((t6c200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c300 >> 0) & 0xffu64) & 1u64);
        c.w(98, 1, value);
        t6c500 = (if 28u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (28u64 as u32)
        }) & 0xffffffffu64;
        t6c600 = (((t6c500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c600 >> 0) & 0xffu64) & 1u64);
        c.w(99, 1, value);
    }
    {
        // 0x00002794 bgt.w 0x000025f0
        let mut t183c00 = 0u64;
        let mut t183d00 = 0u64;
        let mut t183f00 = 0u64;
        t183c00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t183d00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t183f00 = (((((t183c00 >> 0) & 0xffu64) != 0) & (((t183d00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t183f00 >> 0) & 0xffu64) != 0 {
            return 0x25f0u32;
        }
    }
    0x2798
}
pub fn b_2798(c: &mut Machine) -> u32 {
    {
        // 0x00002798 ldrb.w r3,[r5,#0x38]
        let mut t19c800 = 0u64;
        let mut t1aa600 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        t1aa600 = (c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 1)) & 0xffu64;
        let value = ((t1aa600 >> 0) & 0xffu64);
        c.w(44, 4, value);
    }
    {
        // 0x0000279c cbz r3,0x000027c0
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(44, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x27c0u32;
        }
    }
    0x279e
}
pub fn b_279e(c: &mut Machine) -> u32 {
    {
        // 0x0000279e vldr.32 s14,[r5,#0xc]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(12u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x000027a2 vmov.f32 s11,0x3f000000
        let value = 1056964608u64;
        c.w(812, 4, value);
    }
    {
        // 0x000027a6 vmov.f32 s12,0x3f800000
        let value = 1065353216u64;
        c.w(816, 4, value);
    }
    {
        // 0x000027aa vmul.f32 s14,s13,s14
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x000027ae vmul.f32 s13,s14,s11
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x000027b2 vcmpe.f32 s13,s9
        let mut t35c00 = 0u64;
        let mut t35e00 = 0u64;
        let mut t36000 = 0u64;
        let mut t36100 = 0u64;
        let mut t36200 = 0u64;
        let mut t36400 = 0u64;
        let mut t36500 = 0u64;
        let mut t36600 = 0u64;
        let mut t36700 = 0u64;
        let mut t36800 = 0u64;
        let mut t36900 = 0u64;
        let mut t36a00 = 0u64;
        let mut t36b00 = 0u64;
        let mut t36c00 = 0u64;
        let mut t36d00 = 0u64;
        let mut t36e00 = 0u64;
        let mut t36f00 = 0u64;
        let mut t37000 = 0u64;
        t35c00 = ((f32::from_bits(c.r(820, 4) as u32) < f32::from_bits(c.r(804, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(820, 4) as u32) == f32::from_bits(c.r(804, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(804, 4) as u32) <= f32::from_bits(c.r(820, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(820, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(804, 4) as u32).is_nan() as u64) & 0xffu64;
        t36400 = (((t36100 >> 0) & 0xffu64) | ((t36200 >> 0) & 0xffu64)) & 0xffu64;
        t36500 = (c.r(180, 4) & 268435455u64) & 0xffffffffu64;
        t36600 = ((t35c00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36700 = (if 31u64 >= 32 {
            0
        } else {
            ((t36600 >> 0) & 0xffffffffu64) << (31u64 as u32)
        }) & 0xffffffffu64;
        t36800 =
            (((t36500 >> 0) & 0xffffffffu64) | ((t36700 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36900 = ((t35e00 >> 0) & 0xffu64) & 0xffffffffu64;
        t36a00 = (if 30u64 >= 32 {
            0
        } else {
            ((t36900 >> 0) & 0xffffffffu64) << (30u64 as u32)
        }) & 0xffffffffu64;
        t36b00 =
            (((t36800 >> 0) & 0xffffffffu64) | ((t36a00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36c00 = ((t36000 >> 0) & 0xffu64) & 0xffffffffu64;
        t36d00 = (if 29u64 >= 32 {
            0
        } else {
            ((t36c00 >> 0) & 0xffffffffu64) << (29u64 as u32)
        }) & 0xffffffffu64;
        t36e00 =
            (((t36b00 >> 0) & 0xffffffffu64) | ((t36d00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        t36f00 = ((t36400 >> 0) & 0xffu64) & 0xffffffffu64;
        t37000 = (if 28u64 >= 32 {
            0
        } else {
            ((t36f00 >> 0) & 0xffffffffu64) << (28u64 as u32)
        }) & 0xffffffffu64;
        let value = (((t36e00 >> 0) & 0xffffffffu64) | ((t37000 >> 0) & 0xffffffffu64));
        c.w(180, 4, value);
    }
    {
        // 0x000027b6 vmrs apsr,fpscr
        let mut t6bd00 = 0u64;
        let mut t6bf00 = 0u64;
        let mut t6c000 = 0u64;
        let mut t6c200 = 0u64;
        let mut t6c300 = 0u64;
        let mut t6c500 = 0u64;
        let mut t6c600 = 0u64;
        t6bd00 = (if 31u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t6bd00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(96, 1, value);
        t6bf00 = (if 30u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (30u64 as u32)
        }) & 0xffffffffu64;
        t6c000 = (((t6bf00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c000 >> 0) & 0xffu64) & 1u64);
        c.w(97, 1, value);
        t6c200 = (if 29u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (29u64 as u32)
        }) & 0xffffffffu64;
        t6c300 = (((t6c200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c300 >> 0) & 0xffu64) & 1u64);
        c.w(98, 1, value);
        t6c500 = (if 28u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (28u64 as u32)
        }) & 0xffffffffu64;
        t6c600 = (((t6c500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c600 >> 0) & 0xffu64) & 1u64);
        c.w(99, 1, value);
    }
    {
        // 0x000027ba it pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x000027bc vmov.pl.f32 s13,s12
        let mut tf00 = 0u64;
        let mut t2700 = 0u64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2700 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x27c0u32;
        }
        let value = c.r(816, 4);
        c.w(820, 4, value);
    }
    0x27c0
}
pub fn b_27c0(c: &mut Machine) -> u32 {
    {
        // 0x000027c0 cmp r0,r12
        let mut t1a7000 = 0u64;
        let value = (c.r(80, 4) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), c.r(80, 4), 4, true);
        c.w(103, 1, value);
        t1a7000 = ((c.r(32, 4)).wrapping_sub(c.r(80, 4))) & 0xffffffffu64;
        let value = (signed(((t1a7000 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a7000 >> 0) & 0xffffffffu64) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x000027c2 bgt.w 0x000030f4
        let mut t183c00 = 0u64;
        let mut t183d00 = 0u64;
        let mut t183f00 = 0u64;
        t183c00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t183d00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t183f00 = (((((t183c00 >> 0) & 0xffu64) != 0) & (((t183d00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t183f00 >> 0) & 0xffu64) != 0 {
            return 0x30f4u32;
        }
    }
    0x27c6
}
pub fn b_27c6(c: &mut Machine) -> u32 {
    {
        // 0x000027c6 vldr.32 s12,[r5,#0x2c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x000027ca vmov.f32 s9,0x3f000000
        let value = 1056964608u64;
        c.w(804, 4, value);
    }
    {
        // 0x000027ce vldr.32 s10,[r5,#0x28]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x000027d2 vldr.32 s14,[r5,#0x30]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x000027d6 vsub.f32 s11,s12,s10
        let value = (f32::from_bits(c.r(816, 4) as u32) - f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x000027da vldr.32 s8,[r5,#0x24]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x000027de vsub.f32 s10,s10,s14
        let value = (f32::from_bits(c.r(808, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x000027e2 vsub.f32 s8,s8,s12
        let value = (f32::from_bits(c.r(800, 4) as u32) - f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(800, 4, value);
    }
    {
        // 0x000027e6 vmov.f32 s14,s11
        let value = c.r(812, 4);
        c.w(824, 4, value);
    }
    {
        // 0x000027ea vfma.f32 s14,s10,s9
        let value = f32::from_bits(c.r(808, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x000027ee vadd.f32 s11,s11,s14
        let value = (f32::from_bits(c.r(812, 4) as u32) + f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x000027f2 vfma.f32 s11,s8,s9
        let value = f32::from_bits(c.r(800, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(812, 4) as u32),
            )
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x000027f6 vadd.f32 s14,s14,s11
        let value = (f32::from_bits(c.r(824, 4) as u32) + f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x000027fa vfnms.f32 s14,s11,s13
        let mut t49a00 = 0u64;
        let mut t49b00 = 0u64;
        t49a00 = ((-f32::from_bits(c.r(824, 4) as u32)).to_bits() as u64) & 0xffffffffu64;
        t49b00 = ((f32::from_bits(c.r(812, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64)
            & 0xffffffffu64;
        let value = (f32::from_bits(((t49a00 >> 0) & 0xffffffffu64) as u32)
            + f32::from_bits(((t49b00 >> 0) & 0xffffffffu64) as u32))
        .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x000027fe vmul.f32 s14,s13,s14
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00002802 vfma.f32 s14,s10,s9
        let value = f32::from_bits(c.r(808, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00002806 vfma.f32 s12,s13,s14
        let value = f32::from_bits(c.r(820, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(824, 4) as u32),
                f32::from_bits(c.r(816, 4) as u32),
            )
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x0000280a vadd.f32 s14,s12,s23
        let value = (f32::from_bits(c.r(816, 4) as u32) + f32::from_bits(c.r(860, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    0x280e
}
pub fn b_280e(c: &mut Machine) -> u32 {
    {
        // 0x0000280e vldr.32 s12,[r5,#0x100]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(256u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x00002812 vmov.f32 s9,0x3f800000
        let value = 1065353216u64;
        c.w(804, 4, value);
    }
    {
        // 0x00002816 vldr.32 s28,[r5,#0xd0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(208u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(880, 4, value);
    }
    {
        // 0x0000281a vmov.f32 s10,0x3f000000
        let value = 1056964608u64;
        c.w(808, 4, value);
    }
    {
        // 0x0000281e vldr.32 s13,[r5,#0xf4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(244u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00002822 mov.w r8,#0x0
        let mut t187d00 = 0u64;
        t187d00 = (0u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = ((t187d00 >> 0) & 0xffffffffu64);
        c.w(64, 4, value);
        let value = (signed(c.r(64, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(64, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00002826 vfms.f32 s28,s12,s15
        let value = (-f32::from_bits(c.r(816, 4) as u32))
            .fast_mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(880, 4) as u32),
            )
            .to_bits() as u64;
        c.w(880, 4, value);
    }
    {
        // 0x0000282a vldr.32 s11,[r5,#0xc8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(200u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x0000282e ldr.w r1,[r5,#0xdc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(220u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00002832 mov.w lr,#0x0
        let mut t187d00 = 0u64;
        t187d00 = (0u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = ((t187d00 >> 0) & 0xffffffffu64);
        c.w(88, 4, value);
        let value = (signed(c.r(88, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(88, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00002836 vadd.f32 s13,s13,s11
        let value = (f32::from_bits(c.r(820, 4) as u32) + f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x0000283a ldr.w r2,[r5,#0xd8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(216u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x0000283e ldr.w r3,[r5,#0xd4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(212u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00002842 str.w r1,[r5,#0xe0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(224u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x00002846 vldr.32 s8,[pc,#-0x118]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((10310u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(280u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x0000284a vfma.f32 s15,s12,s28
        let value = f32::from_bits(c.r(816, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(880, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x0000284e ldr.w r1,[r5,#0xb0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(176u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00002852 ldr.w r12,[r5,#0xb8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(184u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(80, 4, value);
    }
    {
        // 0x00002856 ldr.w r0,[r5,#0xc0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(192u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x0000285a ldr.w r7,[r5,#0x104]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(260u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(60, 4, value);
    }
    {
        // 0x0000285e vstr.32 s14,[r5,#0x10]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x00002862 str.w r2,[r5,#0xdc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(220u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x00002866 vstr.32 s28,[sp,#0x44]
        let mut t64000 = 0u64;
        t64000 = ((c.r(84, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(880, 4));
    }
    {
        // 0x0000286a str.w r3,[r5,#0xd8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(216u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x0000286e vstr.32 s13,[r5,#0xf4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(244u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x00002872 vstr.32 s15,[r5,#0xd4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(212u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x00002876 b 0x00002884
        return 0x2884u32;
    }
    0x2878
}
pub fn b_2878(c: &mut Machine) -> u32 {
    {
        // 0x00002878 cmp r1,r7
        let mut t1a6e00 = 0u64;
        let value = (c.r(60, 4) <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), c.r(60, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(36, 4)).wrapping_sub(c.r(60, 4))) & 0xffffffffu64;
        let value = (signed(((t1a6e00 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a6e00 >> 0) & 0xffffffffu64) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(102, 1);
        c.w(98, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
        let value = c.r(103, 1);
        c.w(99, 1, value);
    }
    {
        // 0x0000287a str.w lr,[r5,#0xbc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(188u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(88, 4));
    }
    {
        // 0x0000287e bgt 0x0000293a
        let mut t181b00 = 0u64;
        let mut t181c00 = 0u64;
        let mut t181e00 = 0u64;
        t181b00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t181c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t181e00 = (((((t181b00 >> 0) & 0xffu64) != 0) & (((t181c00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t181e00 >> 0) & 0xffu64) != 0 {
            return 0x293au32;
        }
    }
    0x2880
}
pub fn b_2880(c: &mut Machine) -> u32 {
    {
        // 0x00002880 vldr.32 s13,[r5,#0xf4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(244u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    0x2884
}
