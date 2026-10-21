#![allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut,
    unused_comparisons
)]
use crate::machine::{signed, signed_overflow, Machine};
pub fn b_11ee(c: &mut Machine) -> u32 {
    {
        // 0x000011ee ldr r1,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x000011f0 cmp r0,r1
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
        // 0x000011f2 bgt.w 0x000030dc
        let mut t183c00 = 0u64;
        let mut t183d00 = 0u64;
        let mut t183f00 = 0u64;
        t183c00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t183d00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t183f00 = (((((t183c00 >> 0) & 0xffu64) != 0) & (((t183d00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t183f00 >> 0) & 0xffu64) != 0 {
            return 0x30dcu32;
        }
    }
    0x11f6
}
pub fn b_11f6(c: &mut Machine) -> u32 {
    {
        // 0x000011f6 ldr r1,[0x000013e8]
        let value = c.load(0x13e8, 4);
        c.w(36, 4, value);
    }
    {
        // 0x000011f8 vmov.f32 s9,0x3f000000
        let value = 1056964608u64;
        c.w(804, 4, value);
    }
    {
        // 0x000011fc vldr.32 s12,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x00001200 vldr.32 s10,[r1,#-0x4]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x00001204 vldr.32 s14,[r1,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00001208 vsub.f32 s11,s12,s10
        let value = (f32::from_bits(c.r(816, 4) as u32) - f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x0000120c vldr.32 s8,[r1,#-0x8]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x00001210 vsub.f32 s10,s10,s14
        let value = (f32::from_bits(c.r(808, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x00001214 vsub.f32 s8,s8,s12
        let value = (f32::from_bits(c.r(800, 4) as u32) - f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(800, 4, value);
    }
    {
        // 0x00001218 vmov.f32 s14,s11
        let value = c.r(812, 4);
        c.w(824, 4, value);
    }
    {
        // 0x0000121c vfma.f32 s14,s10,s9
        let value = f32::from_bits(c.r(808, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001220 vadd.f32 s11,s11,s14
        let value = (f32::from_bits(c.r(812, 4) as u32) + f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00001224 vfma.f32 s11,s8,s9
        let value = f32::from_bits(c.r(800, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(812, 4) as u32),
            )
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00001228 vadd.f32 s14,s14,s11
        let value = (f32::from_bits(c.r(824, 4) as u32) + f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x0000122c vfnms.f32 s14,s11,s13
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
        // 0x00001230 vmul.f32 s14,s13,s14
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001234 vfma.f32 s14,s10,s9
        let value = f32::from_bits(c.r(808, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001238 vfma.f32 s12,s13,s14
        let value = f32::from_bits(c.r(820, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(824, 4) as u32),
                f32::from_bits(c.r(816, 4) as u32),
            )
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x0000123c vadd.f32 s14,s12,s23
        let value = (f32::from_bits(c.r(816, 4) as u32) + f32::from_bits(c.r(860, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    0x1240
}
pub fn b_1240(c: &mut Machine) -> u32 {
    {
        // 0x00001240 ldr r1,[0x000013f0]
        let value = c.load(0x13f0, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001242 vmov.f32 s9,0x3f800000
        let value = 1065353216u64;
        c.w(804, 4, value);
    }
    {
        // 0x00001246 ldr.w r4,[r6,#0xe18]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3608u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x0000124a vldr.32 s27,[r1,#0x90]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(144u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(876, 4, value);
    }
    {
        // 0x0000124e adds r1,#0xc0
        let value = ((c.r(36, 4) as u128 + 192u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 192u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_add(192u64);
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
        // 0x00001250 vstr.32 s14,[r1,#-0xf0]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(240u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x00001254 vldr.32 s14,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00001258 vldr.32 s13,[r1,#-0xc]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(12u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x0000125c vfms.f32 s27,s14,s15
        let value = (-f32::from_bits(c.r(824, 4) as u32))
            .mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(876, 4) as u32),
            )
            .to_bits() as u64;
        c.w(876, 4, value);
    }
    {
        // 0x00001260 vldr.32 s12,[r1,#-0x38]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(56u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x00001264 mov r0,r1
        let value = c.r(36, 4);
        c.w(32, 4, value);
    }
    {
        // 0x00001266 ldr.w r7,[r6,#0xe64]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3684u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(60, 4, value);
    }
    {
        // 0x0000126a vadd.f32 s13,s13,s12
        let value = (f32::from_bits(c.r(820, 4) as u32) + f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x0000126e str r4,[sp,#0x24]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x00001270 sub.w r11,r0,#0x20
        let mut t187d00 = 0u64;
        t187d00 = (32u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(76, 4, value);
        let value = (signed(c.r(76, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(76, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00001274 sub.w r10,r0,#0x24
        let mut t187d00 = 0u64;
        t187d00 = (36u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(72, 4, value);
        let value = (signed(c.r(72, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(72, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00001278 sub.w r9,r0,#0x28
        let mut t187d00 = 0u64;
        t187d00 = (40u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(68, 4, value);
        let value = (signed(c.r(68, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(68, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x0000127c sub.w r8,r0,#0x2c
        let mut t187d00 = 0u64;
        t187d00 = (44u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(64, 4, value);
        let value = (signed(c.r(64, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(64, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00001280 vstr.32 s13,[r1,#-0xc]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(12u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x00001284 sub.w r12,r0,#0xc
        let mut t187d00 = 0u64;
        t187d00 = (12u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(80, 4, value);
        let value = (signed(c.r(80, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(80, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00001288 ldr.w r1,[r1,#-0x24]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(36, 4)).wrapping_sub(36u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x0000128c vfma.f32 s15,s14,s27
        let value = f32::from_bits(c.r(824, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(876, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001290 vldr.32 s8,[pc,#0x160]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((4752u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(352u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x00001294 str.w r1,[r0,#-0x20]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(32, 4)).wrapping_sub(32u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x00001298 ldr.w r1,[r0,#-0x28]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(32, 4)).wrapping_sub(40u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x0000129c ldr.w r4,[r6,#0xe20]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3616u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x000012a0 str.w r1,[r0,#-0x24]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(32, 4)).wrapping_sub(36u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x000012a4 ldr.w r1,[r0,#-0x2c]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(32, 4)).wrapping_sub(44u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x000012a8 vstr.32 s15,[r0,#-0x2c]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(32, 4)).wrapping_sub(44u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x000012ac str.w r1,[r0,#-0x28]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(32, 4)).wrapping_sub(40u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x000012b0 mov r1,r0
        let value = c.r(32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x000012b2 ldr.w r0,[r6,#0xe14]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3604u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x000012b6 vldr.32 s7,[pc,#0x140]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((4790u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(320u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x000012ba sub.w lr,r1,#0x34
        let mut t187d00 = 0u64;
        t187d00 = (52u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(88, 4, value);
        let value = (signed(c.r(88, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(88, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x000012be str r0,[sp,#0x20]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x000012c0 ldr.w r0,[r6,#0xe10]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3600u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x000012c4 vstr.32 s27,[sp,#0x48]
        let mut t64000 = 0u64;
        t64000 = ((c.r(84, 4)).wrapping_add(72u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(876, 4));
    }
    {
        // 0x000012c8 str r7,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(60, 4));
    }
    {
        // 0x000012ca strd r2,r3,[sp,#0x38]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(84, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    0x12ce
}
pub fn b_12ce(c: &mut Machine) -> u32 {
    {
        // 0x000012ce vmov.f32 s10,0x3f000000
        let value = 1056964608u64;
        c.w(808, 4, value);
    }
    {
        // 0x000012d2 b 0x0000134e
        return 0x134eu32;
    }
    0x12d4
}
pub fn b_12d4(c: &mut Machine) -> u32 {
    {
        // 0x000012d4 cmp r1,#0x0
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
        // 0x000012d6 it lt
        let mut t185c00 = 0u64;
        t185c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
    }
    {
        // 0x000012d8 add.lt r1,r1,r0
        let mut t1d00 = 0u64;
        let mut t2700 = 0u64;
        t1d00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2700 = ((((t1d00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x12dau32;
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
    0x12da
}
pub fn b_12da(c: &mut Machine) -> u32 {
    {
        // 0x000012da add.w r1,r4,r1, lsl #0x2
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
        // 0x000012de ldr r1,[r1,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(36, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x000012e0 str.w r1,[r3,#-0x8]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    0x12e4
}
pub fn b_12e4(c: &mut Machine) -> u32 {
    {
        // 0x000012e4 vldr.32 s6,[r9]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(68, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x000012e8 add.w r3,r4,r2, lsl #0x2
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
        // 0x000012ec vldr.32 s12,[r10]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(72, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x000012f0 adds r2,#0x1
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
        // 0x000012f2 vldr.32 s15,[lr]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(88, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x000012f6 vsub.f32 s14,s6,s12
        let value = (f32::from_bits(c.r(792, 4) as u32) - f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x000012fa vldr.32 s5,[r8]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(64, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(788, 4, value);
    }
    {
        // 0x000012fe vmul.f32 s13,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00001302 vldr.32 s11,[r11]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(76, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x00001306 vsub.f32 s12,s12,s5
        let value = (f32::from_bits(c.r(816, 4) as u32) - f32::from_bits(c.r(788, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x0000130a cmp r0,r2
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
        // 0x0000130c vmov.f32 s15,s14
        let value = c.r(824, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00001310 vsub.f32 s11,s11,s6
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(792, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00001314 vfma.f32 s15,s12,s10
        let value = f32::from_bits(c.r(816, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001318 vadd.f32 s14,s14,s15
        let value = (f32::from_bits(c.r(824, 4) as u32) + f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x0000131c vfma.f32 s14,s11,s10
        let value = f32::from_bits(c.r(812, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001320 vadd.f32 s15,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001324 vfnms.f32 s15,s13,s14
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
        // 0x00001328 vmov.f32 s14,s6
        let value = c.r(792, 4);
        c.w(824, 4, value);
    }
    {
        // 0x0000132c vmul.f32 s15,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001330 vfma.f32 s15,s12,s10
        let value = f32::from_bits(c.r(816, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001334 vfma.f32 s14,s13,s15
        let value = f32::from_bits(c.r(820, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001338 vstr.32 s14,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x0000133c bgt 0x000013ce
        let mut t181b00 = 0u64;
        let mut t181c00 = 0u64;
        let mut t181e00 = 0u64;
        t181b00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t181c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t181e00 = (((((t181b00 >> 0) & 0xffu64) != 0) & (((t181c00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t181e00 >> 0) & 0xffu64) != 0 {
            return 0x13ceu32;
        }
    }
    0x133e
}
pub fn b_133e(c: &mut Machine) -> u32 {
    {
        // 0x0000133e movs r3,#0x0
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
        // 0x00001340 str.w r3,[r6,#0xe1c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3612u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00001344 ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00001346 cmp r0,r3
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
        // 0x00001348 bgt 0x000013d8
        let mut t181b00 = 0u64;
        let mut t181c00 = 0u64;
        let mut t181e00 = 0u64;
        t181b00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t181c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t181e00 = (((((t181b00 >> 0) & 0xffu64) != 0) & (((t181c00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t181e00 >> 0) & 0xffu64) != 0 {
            return 0x13d8u32;
        }
    }
    0x134a
}
pub fn b_134a(c: &mut Machine) -> u32 {
    {
        // 0x0000134a vldr.32 s13,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    0x134e
}
pub fn b_134e(c: &mut Machine) -> u32 {
    {
        // 0x0000134e vcmpe.f32 s13,s9
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
        // 0x00001352 vmrs apsr,fpscr
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
        // 0x00001356 ble 0x00001400
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x1400u32;
        }
    }
    0x1358
}
pub fn b_1358(c: &mut Machine) -> u32 {
    {
        // 0x00001358 ldr r3,[0x000013fc]
        let value = c.load(0x13fc, 4);
        c.w(44, 4, value);
    }
    {
        // 0x0000135a vsub.f32 s13,s13,s9
        let value = (f32::from_bits(c.r(820, 4) as u32) - f32::from_bits(c.r(804, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x0000135e ldr r1,[sp,#0x20]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001360 ldr r2,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00001362 vldr.32 s15,[r3,#0x10]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00001366 str r2,[r3,#0x4]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x00001368 vcmpe.f32 s15,#0
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
        // 0x0000136c ldr.w r2,[r3,#-0x4]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00001370 vstr.32 s13,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x00001374 str r2,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x00001376 ldr.w r2,[r3,#-0x8]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x0000137a vmrs apsr,fpscr
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
        // 0x0000137e str.w r2,[r3,#-0x4]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x00001382 ldr.w r2,[r6,#0xe1c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3612u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00001386 sub.w r1,r2,r1
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
        // 0x0000138a beq 0x000012d4
        let mut t180200 = 0u64;
        t180200 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        if ((t180200 >> 0) & 0xffu64) != 0 {
            return 0x12d4u32;
        }
    }
    0x138c
}
pub fn b_138c(c: &mut Machine) -> u32 {
    {
        // 0x0000138c cmp r1,#0x0
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
        // 0x0000138e ldr r7,[sp,#0x24]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(60, 4, value);
    }
    {
        // 0x00001390 vsub.f32 s11,s15,s8
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(800, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00001394 it lt
        let mut t185c00 = 0u64;
        t185c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
    }
    {
        // 0x00001396 add.lt r1,r1,r0
        let mut t1d00 = 0u64;
        let mut t2700 = 0u64;
        t1d00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2700 = ((((t1d00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x1398u32;
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
    0x1398
}
pub fn b_1398(c: &mut Machine) -> u32 {
    {
        // 0x00001398 subs r7,r2,r7
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
        // 0x0000139a it mi
        let mut t184c00 = 0u64;
        t184c00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x0000139c add.mi r7,r7,r0
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x139eu32;
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
    0x139e
}
pub fn b_139e(c: &mut Machine) -> u32 {
    {
        // 0x0000139e add.w r1,r4,r1, lsl #0x2
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
        // 0x000013a2 vcmpe.f32 s11,#0
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
        t35c00 = ((f32::from_bits(c.r(812, 4) as u32)
            < f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(812, 4) as u32)
            == f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)) as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)
            <= f32::from_bits(c.r(812, 4) as u32)) as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(812, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x000013a6 add.w r7,r4,r7, lsl #0x2
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
        // 0x000013aa vldr.32 s14,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x000013ae vldr.32 s12,[r7]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(60, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x000013b2 vmrs apsr,fpscr
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
        // 0x000013b6 vsub.f32 s12,s12,s14
        let value = (f32::from_bits(c.r(816, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x000013ba ite pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x000013bc vstr.pl.32 s11,[r3,#0x10]
        let mut tf00 = 0u64;
        let mut t2600 = 0u64;
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2600 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2600 >> 0) & 0xffu64) != 0 {
            return 0x13c0u32;
        }
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(812, 4));
    }
    0x13c0
}
pub fn b_13c0(c: &mut Machine) -> u32 {
    {
        // 0x000013c0 vstr.mi.32 s7,[r3,#0x10]
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x13c4u32;
        }
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(796, 4));
    }
    0x13c4
}
pub fn b_13c4(c: &mut Machine) -> u32 {
    {
        // 0x000013c4 vfma.f32 s14,s15,s12
        let value = f32::from_bits(c.r(828, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(816, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x000013c8 vstr.32 s14,[r3,#-0x8]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(44, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x000013cc b 0x000012e4
        return 0x12e4u32;
    }
    0x13ce
}
pub fn b_13ce(c: &mut Machine) -> u32 {
    {
        // 0x000013ce ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x000013d0 str.w r2,[r6,#0xe1c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3612u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x000013d4 cmp r0,r3
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
        // 0x000013d6 ble 0x0000134a
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x134au32;
        }
    }
    0x13d8
}
pub fn b_13d8(c: &mut Machine) -> u32 {
    {
        // 0x000013d8 ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x000013da vldr.32 s13,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x000013de adds r3,#0x1
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
        // 0x000013e0 str r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x000013e2 str.w r3,[r6,#0xe64]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3684u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x000013e6 b 0x000012ce
        return 0x12ceu32;
    }
    0x13e8
}
pub fn b_1400(c: &mut Machine) -> u32 {
    {
        // 0x00001400 ldrb.w r1,[r6,#0xe58]
        let mut t19c800 = 0u64;
        let mut t1aa600 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3672u64)) & 0xffffffffu64;
        t1aa600 = (c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 1)) & 0xffu64;
        let value = ((t1aa600 >> 0) & 0xffu64);
        c.w(36, 4, value);
    }
    {
        // 0x00001404 ldrd r2,r3,[sp,#0x38]
        let mut t19dc00 = 0u64;
        let mut t1ab800 = 0u64;
        t19dc00 = ((c.r(84, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        let value = c.load(((t19dc00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
        t1ab800 = (((t19dc00 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t1ab800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00001408 cbz r1,0x0000142e
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(36, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x142eu32;
        }
    }
    0x140a
}
pub fn b_140a(c: &mut Machine) -> u32 {
    {
        // 0x0000140a ldr r1,[0x0000171c]
        let value = c.load(0x171c, 4);
        c.w(36, 4, value);
    }
    {
        // 0x0000140c vmov.f32 s12,0x3f000000
        let value = 1056964608u64;
        c.w(816, 4, value);
    }
    {
        // 0x00001410 vmov.f32 s14,0x3f800000
        let value = 1065353216u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001414 vldr.32 s15,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00001418 vmul.f32 s15,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x0000141c vmul.f32 s13,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00001420 vcmpe.f32 s13,s9
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
        // 0x00001424 vmrs apsr,fpscr
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
        // 0x00001428 it pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x0000142a vmov.pl.f32 s13,s14
        let mut tf00 = 0u64;
        let mut t2700 = 0u64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2700 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x142eu32;
        }
        let value = c.r(824, 4);
        c.w(820, 4, value);
    }
    0x142e
}
pub fn b_142e(c: &mut Machine) -> u32 {
    {
        // 0x0000142e ldr r1,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001430 cmp r0,r1
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
        // 0x00001432 bgt.w 0x000030d4
        let mut t183c00 = 0u64;
        let mut t183d00 = 0u64;
        let mut t183f00 = 0u64;
        t183c00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t183d00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t183f00 = (((((t183c00 >> 0) & 0xffu64) != 0) & (((t183d00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t183f00 >> 0) & 0xffu64) != 0 {
            return 0x30d4u32;
        }
    }
    0x1436
}
pub fn b_1436(c: &mut Machine) -> u32 {
    {
        // 0x00001436 ldr r1,[0x00001720]
        let value = c.load(0x1720, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001438 vmov.f32 s10,0x3f000000
        let value = 1056964608u64;
        c.w(808, 4, value);
    }
    {
        // 0x0000143c vldr.32 s14,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00001440 vldr.32 s11,[r1,#-0x4]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x00001444 vldr.32 s15,[r1,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00001448 vsub.f32 s12,s14,s11
        let value = (f32::from_bits(c.r(824, 4) as u32) - f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x0000144c vldr.32 s9,[r1,#-0x8]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(804, 4, value);
    }
    {
        // 0x00001450 vsub.f32 s11,s11,s15
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00001454 vsub.f32 s9,s9,s14
        let value = (f32::from_bits(c.r(804, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(804, 4, value);
    }
    {
        // 0x00001458 vmov.f32 s15,s12
        let value = c.r(816, 4);
        c.w(828, 4, value);
    }
    {
        // 0x0000145c vfma.f32 s15,s11,s10
        let value = f32::from_bits(c.r(812, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001460 vadd.f32 s12,s12,s15
        let value = (f32::from_bits(c.r(816, 4) as u32) + f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00001464 vfma.f32 s12,s9,s10
        let value = f32::from_bits(c.r(804, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(816, 4) as u32),
            )
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00001468 vadd.f32 s15,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x0000146c vfnms.f32 s15,s12,s13
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
        // 0x00001470 vmul.f32 s15,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001474 vfma.f32 s15,s11,s10
        let value = f32::from_bits(c.r(812, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001478 vfma.f32 s14,s13,s15
        let value = f32::from_bits(c.r(820, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x0000147c vadd.f32 s15,s14,s23
        let value = (f32::from_bits(c.r(824, 4) as u32) + f32::from_bits(c.r(860, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    0x1480
}
pub fn b_1480(c: &mut Machine) -> u32 {
    {
        // 0x00001480 ldr r1,[0x00001724]
        let value = c.load(0x1724, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001482 vmul.f32 s26,s16,s26
        let value = (f32::from_bits(c.r(832, 4) as u32) * f32::from_bits(c.r(872, 4) as u32))
            .to_bits() as u64;
        c.w(872, 4, value);
    }
    {
        // 0x00001486 vmov.f32 s9,0x3f800000
        let value = 1065353216u64;
        c.w(804, 4, value);
    }
    {
        // 0x0000148a vldr.32 s8,[pc,#0x29c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((5258u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(668u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x0000148e sub.w lr,r1,#0x200
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (512u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = (((t188500 >> 0) & 0xffffffffu64) <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), ((t188500 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(((t188500 >> 0) & 0xffffffffu64));
        c.w(88, 4, value);
        let value = (signed(c.r(88, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(88, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00001492 vstr.32 s15,[r1,#-0x30]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(48u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x00001496 ldr.w r4,[r6,#0xc40]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3136u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x0000149a ldr.w r1,[lr,#-0x4]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(88, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x0000149e mov r11,lr
        let value = c.r(88, 4);
        c.w(76, 4, value);
    }
    {
        // 0x000014a0 ldr.w r0,[lr,#-0x8]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(88, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x000014a4 sub.w r10,lr,#0x4
        let mut t187d00 = 0u64;
        t187d00 = (4u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(88, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(88, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(88, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(72, 4, value);
        let value = (signed(c.r(72, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(72, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x000014a8 str.w r1,[r11],#-0x8
        let mut t19c900 = 0u64;
        t19c900 = (c.r(76, 4)) & 0xffffffffu64;
        let value = (c.r(76, 4)).wrapping_sub(8u64);
        c.w(76, 4, value);
        c.store(((t19c900 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x000014ac sub.w r9,lr,#0xc
        let mut t187d00 = 0u64;
        t187d00 = (12u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(88, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(88, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(88, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(68, 4, value);
        let value = (signed(c.r(68, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(68, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x000014b0 ldr.w r1,[r6,#0xc34]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3124u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x000014b4 add.w r12,lr,#0x14
        let mut t187d00 = 0u64;
        t187d00 = (20u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = ((c.r(88, 4) as u128 + ((t187d00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(88, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(88, 4)).wrapping_add(((t187d00 >> 0) & 0xffffffffu64));
        c.w(80, 4, value);
        let value = (signed(c.r(80, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(80, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x000014b8 vldr.32 s13,[lr,#0x14]
        let mut t64000 = 0u64;
        t64000 = ((c.r(88, 4)).wrapping_add(20u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x000014bc sub.w r8,lr,#0x14
        let mut t187d00 = 0u64;
        t187d00 = (20u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(88, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(88, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(88, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(64, 4, value);
        let value = (signed(c.r(64, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(64, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x000014c0 vldr.32 s15,[lr,#-0x18]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(88, 4)).wrapping_sub(24u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x000014c4 str.w r0,[lr,#-0x4]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(88, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x000014c8 str r1,[sp,#0x20]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x000014ca vadd.f32 s13,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) + f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x000014ce ldr.w r0,[lr,#-0xc]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(88, 4)).wrapping_sub(12u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x000014d2 ldr.w r1,[r6,#0xc38]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3128u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x000014d6 str.w r0,[lr,#-0x8]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(88, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x000014da str r1,[sp,#0x24]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x000014dc ldr.w r0,[r6,#0xc30]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3120u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x000014e0 ldr.w r1,[r6,#0xc84]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3204u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x000014e4 vldr.32 s7,[pc,#0x244]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((5348u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(580u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x000014e8 vstr.32 s13,[lr,#0x14]
        let mut t64000 = 0u64;
        t64000 = ((c.r(88, 4)).wrapping_add(20u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x000014ec vstr.32 s26,[lr,#-0xc]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(88, 4)).wrapping_sub(12u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(872, 4));
    }
    {
        // 0x000014f0 str r1,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x000014f2 strd r2,r3,[sp,#0x38]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(84, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    0x14f6
}
pub fn b_14f6(c: &mut Machine) -> u32 {
    {
        // 0x000014f6 vmov.f32 s10,0x3f000000
        let value = 1056964608u64;
        c.w(808, 4, value);
    }
    {
        // 0x000014fa b 0x00001576
        return 0x1576u32;
    }
    0x14fc
}
pub fn b_14fc(c: &mut Machine) -> u32 {
    {
        // 0x000014fc cmp r1,#0x0
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
        // 0x000014fe it lt
        let mut t185c00 = 0u64;
        t185c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
    }
    {
        // 0x00001500 add.lt r1,r1,r0
        let mut t1d00 = 0u64;
        let mut t2700 = 0u64;
        t1d00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2700 = ((((t1d00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x1502u32;
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
    0x1502
}
pub fn b_1502(c: &mut Machine) -> u32 {
    {
        // 0x00001502 add.w r1,r4,r1, lsl #0x2
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
        // 0x00001506 ldr r1,[r1,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(36, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001508 str.w r1,[r3,#-0x8]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    0x150c
}
pub fn b_150c(c: &mut Machine) -> u32 {
    {
        // 0x0000150c vldr.32 s6,[r11]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(76, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x00001510 add.w r3,r4,r2, lsl #0x2
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
        // 0x00001514 vldr.32 s12,[r10]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(72, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x00001518 adds r2,#0x1
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
        // 0x0000151a vldr.32 s15,[r8]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(64, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x0000151e vsub.f32 s14,s6,s12
        let value = (f32::from_bits(c.r(792, 4) as u32) - f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001522 vldr.32 s5,[r9]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(68, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(788, 4, value);
    }
    {
        // 0x00001526 vmul.f32 s13,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x0000152a vldr.32 s11,[lr]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(88, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x0000152e vsub.f32 s12,s12,s5
        let value = (f32::from_bits(c.r(816, 4) as u32) - f32::from_bits(c.r(788, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00001532 cmp r0,r2
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
        // 0x00001534 vmov.f32 s15,s14
        let value = c.r(824, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00001538 vsub.f32 s11,s11,s6
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(792, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x0000153c vfma.f32 s15,s12,s10
        let value = f32::from_bits(c.r(816, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001540 vadd.f32 s14,s14,s15
        let value = (f32::from_bits(c.r(824, 4) as u32) + f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001544 vfma.f32 s14,s11,s10
        let value = f32::from_bits(c.r(812, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001548 vadd.f32 s15,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x0000154c vfnms.f32 s15,s13,s14
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
        // 0x00001550 vmov.f32 s14,s6
        let value = c.r(792, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00001554 vmul.f32 s15,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001558 vfma.f32 s15,s12,s10
        let value = f32::from_bits(c.r(816, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x0000155c vfma.f32 s14,s13,s15
        let value = f32::from_bits(c.r(820, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001560 vstr.32 s14,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x00001564 bgt 0x000015f6
        let mut t181b00 = 0u64;
        let mut t181c00 = 0u64;
        let mut t181e00 = 0u64;
        t181b00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t181c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t181e00 = (((((t181b00 >> 0) & 0xffu64) != 0) & (((t181c00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t181e00 >> 0) & 0xffu64) != 0 {
            return 0x15f6u32;
        }
    }
    0x1566
}
pub fn b_1566(c: &mut Machine) -> u32 {
    {
        // 0x00001566 movs r3,#0x0
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
        // 0x00001568 str.w r3,[r6,#0xc3c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3132u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x0000156c ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x0000156e cmp r0,r3
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
        // 0x00001570 bgt 0x00001600
        let mut t181b00 = 0u64;
        let mut t181c00 = 0u64;
        let mut t181e00 = 0u64;
        t181b00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t181c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t181e00 = (((((t181b00 >> 0) & 0xffu64) != 0) & (((t181c00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t181e00 >> 0) & 0xffu64) != 0 {
            return 0x1600u32;
        }
    }
    0x1572
}
pub fn b_1572(c: &mut Machine) -> u32 {
    {
        // 0x00001572 vldr.32 s13,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    0x1576
}
pub fn b_1576(c: &mut Machine) -> u32 {
    {
        // 0x00001576 vcmpe.f32 s13,s9
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
        // 0x0000157a vmrs apsr,fpscr
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
        // 0x0000157e ble 0x00001610
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x1610u32;
        }
    }
    0x1580
}
pub fn b_1580(c: &mut Machine) -> u32 {
    {
        // 0x00001580 ldr r3,[0x00001730]
        let value = c.load(0x1730, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00001582 vsub.f32 s13,s13,s9
        let value = (f32::from_bits(c.r(820, 4) as u32) - f32::from_bits(c.r(804, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00001586 ldr r1,[sp,#0x20]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001588 ldr r2,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x0000158a vldr.32 s15,[r3,#0x10]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x0000158e str r2,[r3,#0x4]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x00001590 vcmpe.f32 s15,#0
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
        // 0x00001594 ldr.w r2,[r3,#-0x4]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00001598 vstr.32 s13,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x0000159c str r2,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x0000159e ldr.w r2,[r3,#-0x8]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x000015a2 vmrs apsr,fpscr
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
        // 0x000015a6 str.w r2,[r3,#-0x4]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x000015aa ldr.w r2,[r6,#0xc3c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3132u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x000015ae sub.w r1,r2,r1
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
        // 0x000015b2 beq 0x000014fc
        let mut t180200 = 0u64;
        t180200 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        if ((t180200 >> 0) & 0xffu64) != 0 {
            return 0x14fcu32;
        }
    }
    0x15b4
}
pub fn b_15b4(c: &mut Machine) -> u32 {
    {
        // 0x000015b4 cmp r1,#0x0
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
        // 0x000015b6 ldr r7,[sp,#0x24]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(60, 4, value);
    }
    {
        // 0x000015b8 vsub.f32 s11,s15,s8
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(800, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x000015bc it lt
        let mut t185c00 = 0u64;
        t185c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
    }
    {
        // 0x000015be add.lt r1,r1,r0
        let mut t1d00 = 0u64;
        let mut t2700 = 0u64;
        t1d00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2700 = ((((t1d00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x15c0u32;
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
    0x15c0
}
pub fn b_15c0(c: &mut Machine) -> u32 {
    {
        // 0x000015c0 subs r7,r2,r7
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
        // 0x000015c2 it mi
        let mut t184c00 = 0u64;
        t184c00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x000015c4 add.mi r7,r7,r0
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x15c6u32;
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
    0x15c6
}
pub fn b_15c6(c: &mut Machine) -> u32 {
    {
        // 0x000015c6 add.w r1,r4,r1, lsl #0x2
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
        // 0x000015ca vcmpe.f32 s11,#0
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
        t35c00 = ((f32::from_bits(c.r(812, 4) as u32)
            < f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(812, 4) as u32)
            == f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)) as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)
            <= f32::from_bits(c.r(812, 4) as u32)) as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(812, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x000015ce add.w r7,r4,r7, lsl #0x2
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
        // 0x000015d2 vldr.32 s14,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x000015d6 vldr.32 s12,[r7]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(60, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x000015da vmrs apsr,fpscr
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
        // 0x000015de vsub.f32 s12,s12,s14
        let value = (f32::from_bits(c.r(816, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x000015e2 ite pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x000015e4 vstr.pl.32 s11,[r3,#0x10]
        let mut tf00 = 0u64;
        let mut t2600 = 0u64;
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2600 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2600 >> 0) & 0xffu64) != 0 {
            return 0x15e8u32;
        }
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(812, 4));
    }
    0x15e8
}
pub fn b_15e8(c: &mut Machine) -> u32 {
    {
        // 0x000015e8 vstr.mi.32 s7,[r3,#0x10]
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x15ecu32;
        }
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(796, 4));
    }
    0x15ec
}
pub fn b_15ec(c: &mut Machine) -> u32 {
    {
        // 0x000015ec vfma.f32 s14,s15,s12
        let value = f32::from_bits(c.r(828, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(816, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x000015f0 vstr.32 s14,[r3,#-0x8]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(44, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x000015f4 b 0x0000150c
        return 0x150cu32;
    }
    0x15f6
}
pub fn b_15f6(c: &mut Machine) -> u32 {
    {
        // 0x000015f6 ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x000015f8 str.w r2,[r6,#0xc3c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3132u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x000015fc cmp r0,r3
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
        // 0x000015fe ble 0x00001572
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x1572u32;
        }
    }
    0x1600
}
pub fn b_1600(c: &mut Machine) -> u32 {
    {
        // 0x00001600 ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00001602 vldr.32 s13,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00001606 adds r3,#0x1
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
        // 0x00001608 str r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x0000160a str.w r3,[r6,#0xc84]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3204u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x0000160e b 0x000014f6
        return 0x14f6u32;
    }
    0x1610
}
pub fn b_1610(c: &mut Machine) -> u32 {
    {
        // 0x00001610 ldrb.w r1,[r6,#0xc78]
        let mut t19c800 = 0u64;
        let mut t1aa600 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3192u64)) & 0xffffffffu64;
        t1aa600 = (c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 1)) & 0xffu64;
        let value = ((t1aa600 >> 0) & 0xffu64);
        c.w(36, 4, value);
    }
    {
        // 0x00001614 ldrd r2,r3,[sp,#0x38]
        let mut t19dc00 = 0u64;
        let mut t1ab800 = 0u64;
        t19dc00 = ((c.r(84, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        let value = c.load(((t19dc00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
        t1ab800 = (((t19dc00 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t1ab800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00001618 cbz r1,0x0000163e
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(36, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x163eu32;
        }
    }
    0x161a
}
pub fn b_161a(c: &mut Machine) -> u32 {
    {
        // 0x0000161a ldr r1,[0x00001734]
        let value = c.load(0x1734, 4);
        c.w(36, 4, value);
    }
    {
        // 0x0000161c vmov.f32 s12,0x3f000000
        let value = 1056964608u64;
        c.w(816, 4, value);
    }
    {
        // 0x00001620 vmov.f32 s14,0x3f800000
        let value = 1065353216u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001624 vldr.32 s15,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00001628 vmul.f32 s15,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x0000162c vmul.f32 s13,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00001630 vcmpe.f32 s13,s9
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
        // 0x00001634 vmrs apsr,fpscr
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
        // 0x00001638 it pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x0000163a vmov.pl.f32 s13,s14
        let mut tf00 = 0u64;
        let mut t2700 = 0u64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2700 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x163eu32;
        }
        let value = c.r(824, 4);
        c.w(820, 4, value);
    }
    0x163e
}
pub fn b_163e(c: &mut Machine) -> u32 {
    {
        // 0x0000163e ldr r1,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001640 cmp r0,r1
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
        // 0x00001642 bgt.w 0x000030cc
        let mut t183c00 = 0u64;
        let mut t183d00 = 0u64;
        let mut t183f00 = 0u64;
        t183c00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t183d00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t183f00 = (((((t183c00 >> 0) & 0xffu64) != 0) & (((t183d00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t183f00 >> 0) & 0xffu64) != 0 {
            return 0x30ccu32;
        }
    }
    0x1646
}
pub fn b_1646(c: &mut Machine) -> u32 {
    {
        // 0x00001646 ldr r1,[0x00001730]
        let value = c.load(0x1730, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001648 vmov.f32 s10,0x3f000000
        let value = 1056964608u64;
        c.w(808, 4, value);
    }
    {
        // 0x0000164c vldr.32 s15,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00001650 vldr.32 s11,[r1,#-0x4]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x00001654 vldr.32 s14,[r1,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00001658 vsub.f32 s12,s15,s11
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x0000165c vldr.32 s9,[r1,#-0x8]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(804, 4, value);
    }
    {
        // 0x00001660 vsub.f32 s11,s11,s14
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00001664 vsub.f32 s9,s9,s15
        let value = (f32::from_bits(c.r(804, 4) as u32) - f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(804, 4, value);
    }
    {
        // 0x00001668 vmov.f32 s14,s12
        let value = c.r(816, 4);
        c.w(824, 4, value);
    }
    {
        // 0x0000166c vfma.f32 s14,s11,s10
        let value = f32::from_bits(c.r(812, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001670 vadd.f32 s12,s12,s14
        let value = (f32::from_bits(c.r(816, 4) as u32) + f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00001674 vfma.f32 s12,s9,s10
        let value = f32::from_bits(c.r(804, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(816, 4) as u32),
            )
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00001678 vadd.f32 s14,s14,s12
        let value = (f32::from_bits(c.r(824, 4) as u32) + f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x0000167c vfnms.f32 s14,s12,s13
        let mut t49a00 = 0u64;
        let mut t49b00 = 0u64;
        t49a00 = ((-f32::from_bits(c.r(824, 4) as u32)).to_bits() as u64) & 0xffffffffu64;
        t49b00 = ((f32::from_bits(c.r(816, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64)
            & 0xffffffffu64;
        let value = (f32::from_bits(((t49a00 >> 0) & 0xffffffffu64) as u32)
            + f32::from_bits(((t49b00 >> 0) & 0xffffffffu64) as u32))
        .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001680 vmul.f32 s14,s13,s14
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001684 vfma.f32 s14,s11,s10
        let value = f32::from_bits(c.r(812, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001688 vfma.f32 s15,s13,s14
        let value = f32::from_bits(c.r(820, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(824, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    0x168c
}
pub fn b_168c(c: &mut Machine) -> u32 {
    {
        // 0x0000168c ldr r1,[0x00001738]
        let value = c.load(0x1738, 4);
        c.w(36, 4, value);
    }
    {
        // 0x0000168e vmov.f32 s9,0x3f800000
        let value = 1065353216u64;
        c.w(804, 4, value);
    }
    {
        // 0x00001692 ldr.w r4,[r6,#0xcf8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3320u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x00001696 vldr.32 s14,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x0000169a mov r0,r1
        let value = c.r(36, 4);
        c.w(32, 4, value);
    }
    {
        // 0x0000169c vldr.32 s12,[r1,#-0x30]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(48u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x000016a0 vldr.32 s13,[r1,#-0xc]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(12u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x000016a4 sub.w r11,r0,#0x20
        let mut t187d00 = 0u64;
        t187d00 = (32u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(76, 4, value);
        let value = (signed(c.r(76, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(76, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x000016a8 vfms.f32 s12,s14,s15
        let value = (-f32::from_bits(c.r(824, 4) as u32))
            .mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(816, 4) as u32),
            )
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x000016ac vldr.32 s11,[r1,#-0x38]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(56u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x000016b0 sub.w r10,r0,#0x24
        let mut t187d00 = 0u64;
        t187d00 = (36u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(72, 4, value);
        let value = (signed(c.r(72, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(72, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x000016b4 sub.w r9,r0,#0x28
        let mut t187d00 = 0u64;
        t187d00 = (40u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(68, 4, value);
        let value = (signed(c.r(68, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(68, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x000016b8 vadd.f32 s13,s13,s11
        let value = (f32::from_bits(c.r(820, 4) as u32) + f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x000016bc sub.w r8,r0,#0x2c
        let mut t187d00 = 0u64;
        t187d00 = (44u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(64, 4, value);
        let value = (signed(c.r(64, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(64, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x000016c0 sub.w r12,r0,#0xc
        let mut t187d00 = 0u64;
        t187d00 = (12u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(80, 4, value);
        let value = (signed(c.r(80, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(80, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x000016c4 ldr.w r7,[r6,#0xd44]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3396u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(60, 4, value);
    }
    {
        // 0x000016c8 str r4,[sp,#0x24]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x000016ca vstr.32 s13,[r1,#-0xc]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(12u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x000016ce vfma.f32 s15,s14,s12
        let value = f32::from_bits(c.r(824, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(816, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x000016d2 ldr.w r1,[r1,#-0x24]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(36, 4)).wrapping_sub(36u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x000016d6 vldr.32 s8,[pc,#0x50]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((5846u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(80u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x000016da str.w r1,[r0,#-0x20]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(32, 4)).wrapping_sub(32u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x000016de ldr.w r1,[r0,#-0x28]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(32, 4)).wrapping_sub(40u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x000016e2 ldr.w r4,[r6,#0xd00]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3328u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x000016e6 str.w r1,[r0,#-0x24]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(32, 4)).wrapping_sub(36u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x000016ea ldr.w r1,[r0,#-0x2c]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(32, 4)).wrapping_sub(44u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x000016ee vstr.32 s15,[r0,#-0x2c]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(32, 4)).wrapping_sub(44u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x000016f2 str.w r1,[r0,#-0x28]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(32, 4)).wrapping_sub(40u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x000016f6 mov r1,r0
        let value = c.r(32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x000016f8 ldr.w r0,[r6,#0xcf4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3316u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x000016fc vldr.32 s7,[pc,#0x2c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((5884u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(44u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x00001700 sub.w lr,r1,#0x34
        let mut t187d00 = 0u64;
        t187d00 = (52u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(88, 4, value);
        let value = (signed(c.r(88, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(88, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00001704 str r0,[sp,#0x20]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x00001706 ldr.w r0,[r6,#0xcf0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3312u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x0000170a vstr.32 s12,[sp,#0x44]
        let mut t64000 = 0u64;
        t64000 = ((c.r(84, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x0000170e str r7,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(60, 4));
    }
    {
        // 0x00001710 strd r2,r3,[sp,#0x38]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(84, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    0x1714
}
pub fn b_1714(c: &mut Machine) -> u32 {
    {
        // 0x00001714 vmov.f32 s10,0x3f000000
        let value = 1056964608u64;
        c.w(808, 4, value);
    }
    {
        // 0x00001718 b 0x000017b6
        return 0x17b6u32;
    }
    0x171a
}
pub fn b_173c(c: &mut Machine) -> u32 {
    {
        // 0x0000173c cmp r1,#0x0
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
        // 0x0000173e it lt
        let mut t185c00 = 0u64;
        t185c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
    }
    {
        // 0x00001740 add.lt r1,r1,r0
        let mut t1d00 = 0u64;
        let mut t2700 = 0u64;
        t1d00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2700 = ((((t1d00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x1742u32;
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
    0x1742
}
pub fn b_1742(c: &mut Machine) -> u32 {
    {
        // 0x00001742 add.w r1,r4,r1, lsl #0x2
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
        // 0x00001746 ldr r1,[r1,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(36, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001748 str.w r1,[r3,#-0x8]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    0x174c
}
pub fn b_174c(c: &mut Machine) -> u32 {
    {
        // 0x0000174c vldr.32 s6,[r9]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(68, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x00001750 add.w r3,r4,r2, lsl #0x2
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
        // 0x00001754 vldr.32 s4,[r10]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(72, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(784, 4, value);
    }
    {
        // 0x00001758 adds r2,#0x1
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
        // 0x0000175a vldr.32 s15,[lr]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(88, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x0000175e vsub.f32 s14,s6,s4
        let value = (f32::from_bits(c.r(792, 4) as u32) - f32::from_bits(c.r(784, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001762 vldr.32 s5,[r8]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(64, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(788, 4, value);
    }
    {
        // 0x00001766 vmul.f32 s13,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x0000176a vldr.32 s11,[r11]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(76, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x0000176e vsub.f32 s5,s4,s5
        let value = (f32::from_bits(c.r(784, 4) as u32) - f32::from_bits(c.r(788, 4) as u32))
            .to_bits() as u64;
        c.w(788, 4, value);
    }
    {
        // 0x00001772 cmp r0,r2
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
        // 0x00001774 vmov.f32 s15,s14
        let value = c.r(824, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00001778 vsub.f32 s11,s11,s6
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(792, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x0000177c vfma.f32 s15,s5,s10
        let value = f32::from_bits(c.r(788, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001780 vadd.f32 s14,s14,s15
        let value = (f32::from_bits(c.r(824, 4) as u32) + f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001784 vfma.f32 s14,s11,s10
        let value = f32::from_bits(c.r(812, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001788 vadd.f32 s15,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x0000178c vfnms.f32 s15,s13,s14
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
        // 0x00001790 vmov.f32 s14,s6
        let value = c.r(792, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00001794 vmul.f32 s15,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001798 vfma.f32 s15,s5,s10
        let value = f32::from_bits(c.r(788, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x0000179c vfma.f32 s14,s13,s15
        let value = f32::from_bits(c.r(820, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x000017a0 vstr.32 s14,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x000017a4 bgt 0x00001836
        let mut t181b00 = 0u64;
        let mut t181c00 = 0u64;
        let mut t181e00 = 0u64;
        t181b00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t181c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t181e00 = (((((t181b00 >> 0) & 0xffu64) != 0) & (((t181c00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t181e00 >> 0) & 0xffu64) != 0 {
            return 0x1836u32;
        }
    }
    0x17a6
}
pub fn b_17a6(c: &mut Machine) -> u32 {
    {
        // 0x000017a6 movs r3,#0x0
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
        // 0x000017a8 str.w r3,[r6,#0xcfc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3324u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x000017ac ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x000017ae cmp r0,r3
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
        // 0x000017b0 bgt 0x00001840
        let mut t181b00 = 0u64;
        let mut t181c00 = 0u64;
        let mut t181e00 = 0u64;
        t181b00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t181c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t181e00 = (((((t181b00 >> 0) & 0xffu64) != 0) & (((t181c00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t181e00 >> 0) & 0xffu64) != 0 {
            return 0x1840u32;
        }
    }
    0x17b2
}
pub fn b_17b2(c: &mut Machine) -> u32 {
    {
        // 0x000017b2 vldr.32 s13,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    0x17b6
}
pub fn b_17b6(c: &mut Machine) -> u32 {
    {
        // 0x000017b6 vcmpe.f32 s13,s9
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
        // 0x000017ba vmrs apsr,fpscr
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
        // 0x000017be ble 0x00001850
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x1850u32;
        }
    }
    0x17c0
}
pub fn b_17c0(c: &mut Machine) -> u32 {
    {
        // 0x000017c0 ldr r3,[0x00001a7c]
        let value = c.load(0x1a7c, 4);
        c.w(44, 4, value);
    }
    {
        // 0x000017c2 vsub.f32 s13,s13,s9
        let value = (f32::from_bits(c.r(820, 4) as u32) - f32::from_bits(c.r(804, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x000017c6 ldr r1,[sp,#0x20]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x000017c8 ldr r2,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x000017ca vldr.32 s15,[r3,#0x10]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x000017ce str r2,[r3,#0x4]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x000017d0 vcmpe.f32 s15,#0
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
        // 0x000017d4 ldr.w r2,[r3,#-0x4]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x000017d8 vstr.32 s13,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x000017dc str r2,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x000017de ldr.w r2,[r3,#-0x8]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x000017e2 vmrs apsr,fpscr
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
        // 0x000017e6 str.w r2,[r3,#-0x4]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x000017ea ldr.w r2,[r6,#0xcfc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3324u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x000017ee sub.w r1,r2,r1
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
        // 0x000017f2 beq 0x0000173c
        let mut t180200 = 0u64;
        t180200 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        if ((t180200 >> 0) & 0xffu64) != 0 {
            return 0x173cu32;
        }
    }
    0x17f4
}
pub fn b_17f4(c: &mut Machine) -> u32 {
    {
        // 0x000017f4 cmp r1,#0x0
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
        // 0x000017f6 ldr r7,[sp,#0x24]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(60, 4, value);
    }
    {
        // 0x000017f8 vsub.f32 s6,s15,s8
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(800, 4) as u32))
            .to_bits() as u64;
        c.w(792, 4, value);
    }
    {
        // 0x000017fc it lt
        let mut t185c00 = 0u64;
        t185c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
    }
    {
        // 0x000017fe add.lt r1,r1,r0
        let mut t1d00 = 0u64;
        let mut t2700 = 0u64;
        t1d00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2700 = ((((t1d00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x1800u32;
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
    0x1800
}
pub fn b_1800(c: &mut Machine) -> u32 {
    {
        // 0x00001800 subs r7,r2,r7
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
        // 0x00001802 it mi
        let mut t184c00 = 0u64;
        t184c00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x00001804 add.mi r7,r7,r0
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x1806u32;
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
    0x1806
}
pub fn b_1806(c: &mut Machine) -> u32 {
    {
        // 0x00001806 add.w r1,r4,r1, lsl #0x2
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
        // 0x0000180a vcmpe.f32 s6,#0
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
        // 0x0000180e add.w r7,r4,r7, lsl #0x2
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
        // 0x00001812 vldr.32 s14,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00001816 vldr.32 s11,[r7]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(60, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x0000181a vmrs apsr,fpscr
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
        // 0x0000181e vsub.f32 s11,s11,s14
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00001822 ite pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x00001824 vstr.pl.32 s6,[r3,#0x10]
        let mut tf00 = 0u64;
        let mut t2600 = 0u64;
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2600 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2600 >> 0) & 0xffu64) != 0 {
            return 0x1828u32;
        }
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(792, 4));
    }
    0x1828
}
pub fn b_1828(c: &mut Machine) -> u32 {
    {
        // 0x00001828 vstr.mi.32 s7,[r3,#0x10]
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x182cu32;
        }
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(796, 4));
    }
    0x182c
}
pub fn b_182c(c: &mut Machine) -> u32 {
    {
        // 0x0000182c vfma.f32 s14,s15,s11
        let value = f32::from_bits(c.r(828, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(812, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001830 vstr.32 s14,[r3,#-0x8]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(44, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x00001834 b 0x0000174c
        return 0x174cu32;
    }
    0x1836
}
pub fn b_1836(c: &mut Machine) -> u32 {
    {
        // 0x00001836 ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00001838 str.w r2,[r6,#0xcfc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3324u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x0000183c cmp r0,r3
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
        // 0x0000183e ble 0x000017b2
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x17b2u32;
        }
    }
    0x1840
}
pub fn b_1840(c: &mut Machine) -> u32 {
    {
        // 0x00001840 ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00001842 vldr.32 s13,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00001846 adds r3,#0x1
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
        // 0x00001848 str r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x0000184a str.w r3,[r6,#0xd44]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3396u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x0000184e b 0x00001714
        return 0x1714u32;
    }
    0x1850
}
pub fn b_1850(c: &mut Machine) -> u32 {
    {
        // 0x00001850 ldrb.w r1,[r6,#0xd38]
        let mut t19c800 = 0u64;
        let mut t1aa600 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3384u64)) & 0xffffffffu64;
        t1aa600 = (c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 1)) & 0xffu64;
        let value = ((t1aa600 >> 0) & 0xffu64);
        c.w(36, 4, value);
    }
    {
        // 0x00001854 ldrd r2,r3,[sp,#0x38]
        let mut t19dc00 = 0u64;
        let mut t1ab800 = 0u64;
        t19dc00 = ((c.r(84, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        let value = c.load(((t19dc00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
        t1ab800 = (((t19dc00 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t1ab800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00001858 cbz r1,0x0000187e
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(36, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x187eu32;
        }
    }
    0x185a
}
pub fn b_185a(c: &mut Machine) -> u32 {
    {
        // 0x0000185a ldr r1,[0x00001a80]
        let value = c.load(0x1a80, 4);
        c.w(36, 4, value);
    }
    {
        // 0x0000185c vmov.f32 s11,0x3f000000
        let value = 1056964608u64;
        c.w(812, 4, value);
    }
    {
        // 0x00001860 vmov.f32 s14,0x3f800000
        let value = 1065353216u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001864 vldr.32 s15,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00001868 vmul.f32 s15,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x0000186c vmul.f32 s13,s15,s11
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00001870 vcmpe.f32 s13,s9
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
        // 0x00001874 vmrs apsr,fpscr
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
        // 0x00001878 it pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x0000187a vmov.pl.f32 s13,s14
        let mut tf00 = 0u64;
        let mut t2700 = 0u64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2700 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x187eu32;
        }
        let value = c.r(824, 4);
        c.w(820, 4, value);
    }
    0x187e
}
pub fn b_187e(c: &mut Machine) -> u32 {
    {
        // 0x0000187e ldr r1,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001880 cmp r0,r1
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
        // 0x00001882 bgt.w 0x00003124
        let mut t183c00 = 0u64;
        let mut t183d00 = 0u64;
        let mut t183f00 = 0u64;
        t183c00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t183d00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t183f00 = (((((t183c00 >> 0) & 0xffu64) != 0) & (((t183d00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t183f00 >> 0) & 0xffu64) != 0 {
            return 0x3124u32;
        }
    }
    0x1886
}
pub fn b_1886(c: &mut Machine) -> u32 {
    {
        // 0x00001886 ldr r1,[0x00001a7c]
        let value = c.load(0x1a7c, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001888 vmov.f32 s9,0x3f000000
        let value = 1056964608u64;
        c.w(804, 4, value);
    }
    {
        // 0x0000188c vldr.32 s14,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00001890 vldr.32 s10,[r1,#-0x4]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x00001894 vldr.32 s15,[r1,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00001898 vsub.f32 s11,s14,s10
        let value = (f32::from_bits(c.r(824, 4) as u32) - f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x0000189c vldr.32 s8,[r1,#-0x8]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x000018a0 vsub.f32 s10,s10,s15
        let value = (f32::from_bits(c.r(808, 4) as u32) - f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x000018a4 vsub.f32 s8,s8,s14
        let value = (f32::from_bits(c.r(800, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(800, 4, value);
    }
    {
        // 0x000018a8 vmov.f32 s15,s11
        let value = c.r(812, 4);
        c.w(828, 4, value);
    }
    {
        // 0x000018ac vfma.f32 s15,s10,s9
        let value = f32::from_bits(c.r(808, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x000018b0 vadd.f32 s11,s11,s15
        let value = (f32::from_bits(c.r(812, 4) as u32) + f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x000018b4 vfma.f32 s11,s8,s9
        let value = f32::from_bits(c.r(800, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(812, 4) as u32),
            )
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x000018b8 vadd.f32 s15,s15,s11
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x000018bc vfnms.f32 s15,s11,s13
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
        // 0x000018c0 vmul.f32 s15,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x000018c4 vfma.f32 s15,s10,s9
        let value = f32::from_bits(c.r(808, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x000018c8 vfma.f32 s14,s13,s15
        let value = f32::from_bits(c.r(820, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x000018cc vadd.f32 s15,s14,s23
        let value = (f32::from_bits(c.r(824, 4) as u32) + f32::from_bits(c.r(860, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    0x18d0
}
pub fn b_18d0(c: &mut Machine) -> u32 {
    {
        // 0x000018d0 ldr r1,[0x00001a84]
        let value = c.load(0x1a84, 4);
        c.w(36, 4, value);
    }
    {
        // 0x000018d2 vmov.f32 s9,0x3f800000
        let value = 1065353216u64;
        c.w(804, 4, value);
    }
    {
        // 0x000018d6 ldr.w r4,[r6,#0xdb8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3512u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x000018da vldr.32 s14,[r1,#0xc0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(192u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x000018de adds r1,#0xc0
        let value = ((c.r(36, 4) as u128 + 192u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 192u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_add(192u64);
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
        // 0x000018e0 vstr.32 s15,[r1,#-0xf0]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(240u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x000018e4 vldr.32 s15,[r1,#-0x30]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(48u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x000018e8 vldr.32 s13,[r1,#-0xc]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(12u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x000018ec vfms.f32 s15,s14,s12
        let value = (-f32::from_bits(c.r(824, 4) as u32))
            .mul_add(
                f32::from_bits(c.r(816, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x000018f0 vldr.32 s11,[r1,#-0x38]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(56u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x000018f4 mov r0,r1
        let value = c.r(36, 4);
        c.w(32, 4, value);
    }
    {
        // 0x000018f6 ldr.w r7,[r6,#0xe04]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3588u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(60, 4, value);
    }
    {
        // 0x000018fa vadd.f32 s13,s13,s11
        let value = (f32::from_bits(c.r(820, 4) as u32) + f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x000018fe str r4,[sp,#0x24]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x00001900 sub.w r11,r0,#0x20
        let mut t187d00 = 0u64;
        t187d00 = (32u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(76, 4, value);
        let value = (signed(c.r(76, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(76, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00001904 sub.w r10,r0,#0x24
        let mut t187d00 = 0u64;
        t187d00 = (36u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(72, 4, value);
        let value = (signed(c.r(72, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(72, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00001908 sub.w r9,r0,#0x28
        let mut t187d00 = 0u64;
        t187d00 = (40u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(68, 4, value);
        let value = (signed(c.r(68, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(68, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x0000190c sub.w r8,r0,#0x2c
        let mut t187d00 = 0u64;
        t187d00 = (44u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(64, 4, value);
        let value = (signed(c.r(64, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(64, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00001910 vstr.32 s13,[r1,#-0xc]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(12u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x00001914 sub.w r12,r0,#0xc
        let mut t187d00 = 0u64;
        t187d00 = (12u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(80, 4, value);
        let value = (signed(c.r(80, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(80, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00001918 ldr.w r1,[r1,#-0x24]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(36, 4)).wrapping_sub(36u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x0000191c vfma.f32 s12,s14,s15
        let value = f32::from_bits(c.r(824, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(816, 4) as u32),
            )
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00001920 vldr.32 s8,[pc,#0x164]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((6432u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(356u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x00001924 str.w r1,[r0,#-0x20]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(32, 4)).wrapping_sub(32u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x00001928 ldr.w r1,[r0,#-0x28]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(32, 4)).wrapping_sub(40u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x0000192c ldr.w r4,[r6,#0xdc0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3520u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x00001930 str.w r1,[r0,#-0x24]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(32, 4)).wrapping_sub(36u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x00001934 ldr.w r1,[r0,#-0x2c]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(32, 4)).wrapping_sub(44u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001938 vstr.32 s12,[r0,#-0x2c]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(32, 4)).wrapping_sub(44u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x0000193c str.w r1,[r0,#-0x28]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(32, 4)).wrapping_sub(40u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x00001940 mov r1,r0
        let value = c.r(32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001942 ldr.w r0,[r6,#0xdb4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3508u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x00001946 vldr.32 s7,[pc,#0x144]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((6470u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(324u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x0000194a sub.w lr,r1,#0x34
        let mut t187d00 = 0u64;
        t187d00 = (52u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(88, 4, value);
        let value = (signed(c.r(88, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(88, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x0000194e str r0,[sp,#0x20]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x00001950 ldr.w r0,[r6,#0xdb0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3504u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x00001954 vstr.32 s15,[sp,#0x44]
        let mut t64000 = 0u64;
        t64000 = ((c.r(84, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x00001958 str r7,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(60, 4));
    }
    {
        // 0x0000195a strd r2,r3,[sp,#0x38]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(84, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    0x195e
}
pub fn b_195e(c: &mut Machine) -> u32 {
    {
        // 0x0000195e vmov.f32 s10,0x3f000000
        let value = 1056964608u64;
        c.w(808, 4, value);
    }
    {
        // 0x00001962 b 0x000019e2
        return 0x19e2u32;
    }
    0x1964
}
pub fn b_1964(c: &mut Machine) -> u32 {
    {
        // 0x00001964 cmp r1,#0x0
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
        // 0x00001966 it lt
        let mut t185c00 = 0u64;
        t185c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
    }
    {
        // 0x00001968 add.lt r1,r1,r0
        let mut t1d00 = 0u64;
        let mut t2700 = 0u64;
        t1d00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2700 = ((((t1d00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x196au32;
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
    0x196a
}
pub fn b_196a(c: &mut Machine) -> u32 {
    {
        // 0x0000196a add.w r1,r4,r1, lsl #0x2
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
        // 0x0000196e ldr r1,[r1,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(36, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001970 str.w r1,[r3,#-0x8]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    0x1974
}
pub fn b_1974(c: &mut Machine) -> u32 {
    {
        // 0x00001974 vldr.32 s6,[r9]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(68, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x00001978 add.w r3,r4,r2, lsl #0x2
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
        // 0x0000197c vldr.32 s12,[r10]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(72, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x00001980 adds r2,#0x1
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
        // 0x00001982 vldr.32 s5,[lr]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(88, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(788, 4, value);
    }
    {
        // 0x00001986 vsub.f32 s14,s6,s12
        let value = (f32::from_bits(c.r(792, 4) as u32) - f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x0000198a vldr.32 s4,[r8]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(64, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(784, 4, value);
    }
    {
        // 0x0000198e vmul.f32 s13,s13,s5
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(788, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00001992 vldr.32 s11,[r11]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(76, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x00001996 vsub.f32 s12,s12,s4
        let value = (f32::from_bits(c.r(816, 4) as u32) - f32::from_bits(c.r(784, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x0000199a cmp r0,r2
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
        // 0x0000199c vmov.f32 s5,s14
        let value = c.r(824, 4);
        c.w(788, 4, value);
    }
    {
        // 0x000019a0 vsub.f32 s11,s11,s6
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(792, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x000019a4 vfma.f32 s5,s12,s10
        let value = f32::from_bits(c.r(816, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(788, 4) as u32),
            )
            .to_bits() as u64;
        c.w(788, 4, value);
    }
    {
        // 0x000019a8 vadd.f32 s14,s14,s5
        let value = (f32::from_bits(c.r(824, 4) as u32) + f32::from_bits(c.r(788, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x000019ac vfma.f32 s14,s11,s10
        let value = f32::from_bits(c.r(812, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x000019b0 vadd.f32 s11,s5,s14
        let value = (f32::from_bits(c.r(788, 4) as u32) + f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x000019b4 vfnms.f32 s11,s13,s14
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
        // 0x000019b8 vmul.f32 s14,s13,s11
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x000019bc vfma.f32 s14,s12,s10
        let value = f32::from_bits(c.r(816, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x000019c0 vmov.f32 s12,s14
        let value = c.r(824, 4);
        c.w(816, 4, value);
    }
    {
        // 0x000019c4 vmov.f32 s14,s6
        let value = c.r(792, 4);
        c.w(824, 4, value);
    }
    {
        // 0x000019c8 vfma.f32 s14,s13,s12
        let value = f32::from_bits(c.r(820, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(816, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x000019cc vstr.32 s14,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x000019d0 bgt 0x00001a62
        let mut t181b00 = 0u64;
        let mut t181c00 = 0u64;
        let mut t181e00 = 0u64;
        t181b00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t181c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t181e00 = (((((t181b00 >> 0) & 0xffu64) != 0) & (((t181c00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t181e00 >> 0) & 0xffu64) != 0 {
            return 0x1a62u32;
        }
    }
    0x19d2
}
pub fn b_19d2(c: &mut Machine) -> u32 {
    {
        // 0x000019d2 movs r3,#0x0
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
        // 0x000019d4 str.w r3,[r6,#0xdbc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3516u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x000019d8 ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x000019da cmp r0,r3
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
        // 0x000019dc bgt 0x00001a6c
        let mut t181b00 = 0u64;
        let mut t181c00 = 0u64;
        let mut t181e00 = 0u64;
        t181b00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t181c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t181e00 = (((((t181b00 >> 0) & 0xffu64) != 0) & (((t181c00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t181e00 >> 0) & 0xffu64) != 0 {
            return 0x1a6cu32;
        }
    }
    0x19de
}
pub fn b_19de(c: &mut Machine) -> u32 {
    {
        // 0x000019de vldr.32 s13,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    0x19e2
}
pub fn b_19e2(c: &mut Machine) -> u32 {
    {
        // 0x000019e2 vcmpe.f32 s13,s9
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
        // 0x000019e6 vmrs apsr,fpscr
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
        // 0x000019ea ble 0x00001a94
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x1a94u32;
        }
    }
    0x19ec
}
pub fn b_19ec(c: &mut Machine) -> u32 {
    {
        // 0x000019ec ldr r3,[0x00001a90]
        let value = c.load(0x1a90, 4);
        c.w(44, 4, value);
    }
    {
        // 0x000019ee vsub.f32 s13,s13,s9
        let value = (f32::from_bits(c.r(820, 4) as u32) - f32::from_bits(c.r(804, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x000019f2 ldr r1,[sp,#0x20]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x000019f4 ldr r2,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x000019f6 vldr.32 s14,[r3,#0x10]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x000019fa str r2,[r3,#0x4]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x000019fc vcmpe.f32 s14,#0
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
        // 0x00001a00 ldr.w r2,[r3,#-0x4]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00001a04 vstr.32 s13,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x00001a08 str r2,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x00001a0a ldr.w r2,[r3,#-0x8]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00001a0e vmrs apsr,fpscr
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
        // 0x00001a12 str.w r2,[r3,#-0x4]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x00001a16 ldr.w r2,[r6,#0xdbc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3516u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00001a1a sub.w r1,r2,r1
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
        // 0x00001a1e beq 0x00001964
        let mut t180200 = 0u64;
        t180200 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        if ((t180200 >> 0) & 0xffu64) != 0 {
            return 0x1964u32;
        }
    }
    0x1a20
}
pub fn b_1a20(c: &mut Machine) -> u32 {
    {
        // 0x00001a20 cmp r1,#0x0
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
        // 0x00001a22 ldr r7,[sp,#0x24]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(60, 4, value);
    }
    {
        // 0x00001a24 vsub.f32 s6,s14,s8
        let value = (f32::from_bits(c.r(824, 4) as u32) - f32::from_bits(c.r(800, 4) as u32))
            .to_bits() as u64;
        c.w(792, 4, value);
    }
    {
        // 0x00001a28 it lt
        let mut t185c00 = 0u64;
        t185c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
    }
    {
        // 0x00001a2a add.lt r1,r1,r0
        let mut t1d00 = 0u64;
        let mut t2700 = 0u64;
        t1d00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2700 = ((((t1d00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x1a2cu32;
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
    0x1a2c
}
pub fn b_1a2c(c: &mut Machine) -> u32 {
    {
        // 0x00001a2c subs r7,r2,r7
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
        // 0x00001a2e it mi
        let mut t184c00 = 0u64;
        t184c00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x00001a30 add.mi r7,r7,r0
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x1a32u32;
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
    0x1a32
}
pub fn b_1a32(c: &mut Machine) -> u32 {
    {
        // 0x00001a32 add.w r1,r4,r1, lsl #0x2
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
        // 0x00001a36 vcmpe.f32 s6,#0
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
        // 0x00001a3a add.w r7,r4,r7, lsl #0x2
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
        // 0x00001a3e vldr.32 s12,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x00001a42 vldr.32 s11,[r7]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(60, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x00001a46 vmrs apsr,fpscr
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
        // 0x00001a4a vsub.f32 s11,s11,s12
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00001a4e ite pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x00001a50 vstr.pl.32 s6,[r3,#0x10]
        let mut tf00 = 0u64;
        let mut t2600 = 0u64;
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2600 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2600 >> 0) & 0xffu64) != 0 {
            return 0x1a54u32;
        }
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(792, 4));
    }
    0x1a54
}
pub fn b_1a54(c: &mut Machine) -> u32 {
    {
        // 0x00001a54 vstr.mi.32 s7,[r3,#0x10]
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x1a58u32;
        }
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(796, 4));
    }
    0x1a58
}
pub fn b_1a58(c: &mut Machine) -> u32 {
    {
        // 0x00001a58 vfma.f32 s12,s14,s11
        let value = f32::from_bits(c.r(824, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(812, 4) as u32),
                f32::from_bits(c.r(816, 4) as u32),
            )
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00001a5c vstr.32 s12,[r3,#-0x8]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(44, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x00001a60 b 0x00001974
        return 0x1974u32;
    }
    0x1a62
}
pub fn b_1a62(c: &mut Machine) -> u32 {
    {
        // 0x00001a62 ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00001a64 str.w r2,[r6,#0xdbc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3516u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x00001a68 cmp r0,r3
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
        // 0x00001a6a ble 0x000019de
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x19deu32;
        }
    }
    0x1a6c
}
pub fn b_1a6c(c: &mut Machine) -> u32 {
    {
        // 0x00001a6c ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00001a6e vldr.32 s13,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00001a72 adds r3,#0x1
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
        // 0x00001a74 str r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00001a76 str.w r3,[r6,#0xe04]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3588u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00001a7a b 0x0000195e
        return 0x195eu32;
    }
    0x1a7c
}
pub fn b_1a94(c: &mut Machine) -> u32 {
    {
        // 0x00001a94 ldrb.w r1,[r6,#0xdf8]
        let mut t19c800 = 0u64;
        let mut t1aa600 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3576u64)) & 0xffffffffu64;
        t1aa600 = (c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 1)) & 0xffu64;
        let value = ((t1aa600 >> 0) & 0xffu64);
        c.w(36, 4, value);
    }
    {
        // 0x00001a98 ldrd r2,r3,[sp,#0x38]
        let mut t19dc00 = 0u64;
        let mut t1ab800 = 0u64;
        t19dc00 = ((c.r(84, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        let value = c.load(((t19dc00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
        t1ab800 = (((t19dc00 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t1ab800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00001a9c cbz r1,0x00001ac2
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(36, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x1ac2u32;
        }
    }
    0x1a9e
}
pub fn b_1a9e(c: &mut Machine) -> u32 {
    {
        // 0x00001a9e ldr r1,[0x00001ddc]
        let value = c.load(0x1ddc, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001aa0 vmov.f32 s11,0x3f000000
        let value = 1056964608u64;
        c.w(812, 4, value);
    }
    {
        // 0x00001aa4 vmov.f32 s12,0x3f800000
        let value = 1065353216u64;
        c.w(816, 4, value);
    }
    {
        // 0x00001aa8 vldr.32 s14,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00001aac vmul.f32 s14,s13,s14
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001ab0 vmul.f32 s13,s14,s11
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00001ab4 vcmpe.f32 s13,s9
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
        // 0x00001ab8 vmrs apsr,fpscr
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
        // 0x00001abc it pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x00001abe vmov.pl.f32 s13,s12
        let mut tf00 = 0u64;
        let mut t2700 = 0u64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2700 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x1ac2u32;
        }
        let value = c.r(816, 4);
        c.w(820, 4, value);
    }
    0x1ac2
}
pub fn b_1ac2(c: &mut Machine) -> u32 {
    {
        // 0x00001ac2 ldr r1,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001ac4 cmp r0,r1
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
        // 0x00001ac6 bgt.w 0x0000311c
        let mut t183c00 = 0u64;
        let mut t183d00 = 0u64;
        let mut t183f00 = 0u64;
        t183c00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t183d00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t183f00 = (((((t183c00 >> 0) & 0xffu64) != 0) & (((t183d00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t183f00 >> 0) & 0xffu64) != 0 {
            return 0x311cu32;
        }
    }
    0x1aca
}
pub fn b_1aca(c: &mut Machine) -> u32 {
    {
        // 0x00001aca ldr r1,[0x00001de0]
        let value = c.load(0x1de0, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001acc vmov.f32 s9,0x3f000000
        let value = 1056964608u64;
        c.w(804, 4, value);
    }
    {
        // 0x00001ad0 vldr.32 s12,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x00001ad4 vldr.32 s10,[r1,#-0x4]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x00001ad8 vldr.32 s14,[r1,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00001adc vsub.f32 s11,s12,s10
        let value = (f32::from_bits(c.r(816, 4) as u32) - f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00001ae0 vldr.32 s8,[r1,#-0x8]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x00001ae4 vsub.f32 s10,s10,s14
        let value = (f32::from_bits(c.r(808, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x00001ae8 vsub.f32 s8,s8,s12
        let value = (f32::from_bits(c.r(800, 4) as u32) - f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(800, 4, value);
    }
    {
        // 0x00001aec vmov.f32 s14,s11
        let value = c.r(812, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00001af0 vfma.f32 s14,s10,s9
        let value = f32::from_bits(c.r(808, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001af4 vadd.f32 s11,s11,s14
        let value = (f32::from_bits(c.r(812, 4) as u32) + f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00001af8 vfma.f32 s11,s8,s9
        let value = f32::from_bits(c.r(800, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(812, 4) as u32),
            )
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00001afc vadd.f32 s14,s14,s11
        let value = (f32::from_bits(c.r(824, 4) as u32) + f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001b00 vfnms.f32 s14,s11,s13
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
        // 0x00001b04 vmul.f32 s14,s13,s14
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001b08 vfma.f32 s14,s10,s9
        let value = f32::from_bits(c.r(808, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001b0c vfma.f32 s12,s13,s14
        let value = f32::from_bits(c.r(820, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(824, 4) as u32),
                f32::from_bits(c.r(816, 4) as u32),
            )
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00001b10 vadd.f32 s14,s12,s23
        let value = (f32::from_bits(c.r(816, 4) as u32) + f32::from_bits(c.r(860, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    0x1b14
}
pub fn b_1b14(c: &mut Machine) -> u32 {
    {
        // 0x00001b14 ldr r1,[0x00001de4]
        let value = c.load(0x1de4, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001b16 vmov.f32 s9,0x3f800000
        let value = 1065353216u64;
        c.w(804, 4, value);
    }
    {
        // 0x00001b1a ldr.w r4,[r6,#0xe78]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3704u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x00001b1e vldr.32 s26,[r1,#0x90]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(144u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(872, 4, value);
    }
    {
        // 0x00001b22 adds r1,#0xc0
        let value = ((c.r(36, 4) as u128 + 192u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 192u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_add(192u64);
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
        // 0x00001b24 vstr.32 s14,[r1,#-0xf0]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(240u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x00001b28 vldr.32 s14,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00001b2c vldr.32 s13,[r1,#-0xc]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(12u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00001b30 vfms.f32 s26,s14,s15
        let value = (-f32::from_bits(c.r(824, 4) as u32))
            .mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(872, 4) as u32),
            )
            .to_bits() as u64;
        c.w(872, 4, value);
    }
    {
        // 0x00001b34 vldr.32 s12,[r1,#-0x38]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(56u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x00001b38 mov r0,r1
        let value = c.r(36, 4);
        c.w(32, 4, value);
    }
    {
        // 0x00001b3a ldr.w r7,[r6,#0xec4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3780u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(60, 4, value);
    }
    {
        // 0x00001b3e vadd.f32 s13,s13,s12
        let value = (f32::from_bits(c.r(820, 4) as u32) + f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00001b42 str r4,[sp,#0x24]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x00001b44 sub.w r11,r0,#0x20
        let mut t187d00 = 0u64;
        t187d00 = (32u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(76, 4, value);
        let value = (signed(c.r(76, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(76, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00001b48 sub.w r10,r0,#0x24
        let mut t187d00 = 0u64;
        t187d00 = (36u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(72, 4, value);
        let value = (signed(c.r(72, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(72, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00001b4c sub.w r9,r0,#0x28
        let mut t187d00 = 0u64;
        t187d00 = (40u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(68, 4, value);
        let value = (signed(c.r(68, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(68, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00001b50 sub.w r8,r0,#0x2c
        let mut t187d00 = 0u64;
        t187d00 = (44u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(64, 4, value);
        let value = (signed(c.r(64, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(64, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00001b54 vstr.32 s13,[r1,#-0xc]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(12u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x00001b58 sub.w r12,r0,#0xc
        let mut t187d00 = 0u64;
        t187d00 = (12u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(80, 4, value);
        let value = (signed(c.r(80, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(80, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00001b5c ldr.w r1,[r1,#-0x24]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(36, 4)).wrapping_sub(36u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001b60 vfma.f32 s15,s14,s26
        let value = f32::from_bits(c.r(824, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(872, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001b64 vldr.32 s8,[pc,#0x280]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((7012u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(640u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x00001b68 str.w r1,[r0,#-0x20]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(32, 4)).wrapping_sub(32u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x00001b6c ldr.w r1,[r0,#-0x28]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(32, 4)).wrapping_sub(40u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001b70 ldr.w r4,[r6,#0xe80]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3712u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x00001b74 str.w r1,[r0,#-0x24]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(32, 4)).wrapping_sub(36u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x00001b78 ldr.w r1,[r0,#-0x2c]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(32, 4)).wrapping_sub(44u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001b7c vstr.32 s15,[r0,#-0x2c]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(32, 4)).wrapping_sub(44u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x00001b80 str.w r1,[r0,#-0x28]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(32, 4)).wrapping_sub(40u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x00001b84 mov r1,r0
        let value = c.r(32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001b86 ldr.w r0,[r6,#0xe74]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3700u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x00001b8a vldr.32 s7,[pc,#0x260]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((7050u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(608u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x00001b8e sub.w lr,r1,#0x34
        let mut t187d00 = 0u64;
        t187d00 = (52u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(88, 4, value);
        let value = (signed(c.r(88, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(88, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00001b92 str r0,[sp,#0x20]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x00001b94 ldr.w r0,[r6,#0xe70]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3696u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x00001b98 vstr.32 s26,[sp,#0x4c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(84, 4)).wrapping_add(76u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(872, 4));
    }
    {
        // 0x00001b9c str r7,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(60, 4));
    }
    {
        // 0x00001b9e strd r2,r3,[sp,#0x38]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(84, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    0x1ba2
}
pub fn b_1ba2(c: &mut Machine) -> u32 {
    {
        // 0x00001ba2 vmov.f32 s10,0x3f000000
        let value = 1056964608u64;
        c.w(808, 4, value);
    }
    {
        // 0x00001ba6 b 0x00001c22
        return 0x1c22u32;
    }
    0x1ba8
}
pub fn b_1ba8(c: &mut Machine) -> u32 {
    {
        // 0x00001ba8 cmp r1,#0x0
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
        // 0x00001baa it lt
        let mut t185c00 = 0u64;
        t185c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
    }
    {
        // 0x00001bac add.lt r1,r1,r0
        let mut t1d00 = 0u64;
        let mut t2700 = 0u64;
        t1d00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2700 = ((((t1d00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x1baeu32;
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
    0x1bae
}
pub fn b_1bae(c: &mut Machine) -> u32 {
    {
        // 0x00001bae add.w r1,r4,r1, lsl #0x2
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
        // 0x00001bb2 ldr r1,[r1,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(36, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001bb4 str.w r1,[r3,#-0x8]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    0x1bb8
}
pub fn b_1bb8(c: &mut Machine) -> u32 {
    {
        // 0x00001bb8 vldr.32 s6,[r9]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(68, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x00001bbc add.w r3,r4,r2, lsl #0x2
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
        // 0x00001bc0 vldr.32 s12,[r10]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(72, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x00001bc4 adds r2,#0x1
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
        // 0x00001bc6 vldr.32 s15,[lr]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(88, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00001bca vsub.f32 s14,s6,s12
        let value = (f32::from_bits(c.r(792, 4) as u32) - f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001bce vldr.32 s5,[r8]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(64, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(788, 4, value);
    }
    {
        // 0x00001bd2 vmul.f32 s13,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00001bd6 vldr.32 s11,[r11]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(76, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x00001bda vsub.f32 s12,s12,s5
        let value = (f32::from_bits(c.r(816, 4) as u32) - f32::from_bits(c.r(788, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00001bde cmp r0,r2
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
        // 0x00001be0 vmov.f32 s15,s14
        let value = c.r(824, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00001be4 vsub.f32 s11,s11,s6
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(792, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00001be8 vfma.f32 s15,s12,s10
        let value = f32::from_bits(c.r(816, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001bec vadd.f32 s14,s14,s15
        let value = (f32::from_bits(c.r(824, 4) as u32) + f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001bf0 vfma.f32 s14,s11,s10
        let value = f32::from_bits(c.r(812, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001bf4 vadd.f32 s15,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001bf8 vfnms.f32 s15,s13,s14
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
        // 0x00001bfc vmov.f32 s14,s6
        let value = c.r(792, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00001c00 vmul.f32 s15,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001c04 vfma.f32 s15,s12,s10
        let value = f32::from_bits(c.r(816, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001c08 vfma.f32 s14,s13,s15
        let value = f32::from_bits(c.r(820, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001c0c vstr.32 s14,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x00001c10 bgt 0x00001ca2
        let mut t181b00 = 0u64;
        let mut t181c00 = 0u64;
        let mut t181e00 = 0u64;
        t181b00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t181c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t181e00 = (((((t181b00 >> 0) & 0xffu64) != 0) & (((t181c00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t181e00 >> 0) & 0xffu64) != 0 {
            return 0x1ca2u32;
        }
    }
    0x1c12
}
pub fn b_1c12(c: &mut Machine) -> u32 {
    {
        // 0x00001c12 movs r3,#0x0
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
        // 0x00001c14 str.w r3,[r6,#0xe7c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3708u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00001c18 ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00001c1a cmp r0,r3
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
        // 0x00001c1c bgt 0x00001cac
        let mut t181b00 = 0u64;
        let mut t181c00 = 0u64;
        let mut t181e00 = 0u64;
        t181b00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t181c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t181e00 = (((((t181b00 >> 0) & 0xffu64) != 0) & (((t181c00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t181e00 >> 0) & 0xffu64) != 0 {
            return 0x1cacu32;
        }
    }
    0x1c1e
}
pub fn b_1c1e(c: &mut Machine) -> u32 {
    {
        // 0x00001c1e vldr.32 s13,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    0x1c22
}
pub fn b_1c22(c: &mut Machine) -> u32 {
    {
        // 0x00001c22 vcmpe.f32 s13,s9
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
        // 0x00001c26 vmrs apsr,fpscr
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
        // 0x00001c2a ble 0x00001cbc
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x1cbcu32;
        }
    }
    0x1c2c
}
pub fn b_1c2c(c: &mut Machine) -> u32 {
    {
        // 0x00001c2c ldr r3,[0x00001df0]
        let value = c.load(0x1df0, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00001c2e vsub.f32 s13,s13,s9
        let value = (f32::from_bits(c.r(820, 4) as u32) - f32::from_bits(c.r(804, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00001c32 ldr r1,[sp,#0x20]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001c34 ldr r2,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00001c36 vldr.32 s15,[r3,#0x10]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00001c3a str r2,[r3,#0x4]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x00001c3c vcmpe.f32 s15,#0
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
        // 0x00001c40 ldr.w r2,[r3,#-0x4]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00001c44 vstr.32 s13,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x00001c48 str r2,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x00001c4a ldr.w r2,[r3,#-0x8]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00001c4e vmrs apsr,fpscr
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
        // 0x00001c52 str.w r2,[r3,#-0x4]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x00001c56 ldr.w r2,[r6,#0xe7c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3708u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00001c5a sub.w r1,r2,r1
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
        // 0x00001c5e beq 0x00001ba8
        let mut t180200 = 0u64;
        t180200 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        if ((t180200 >> 0) & 0xffu64) != 0 {
            return 0x1ba8u32;
        }
    }
    0x1c60
}
pub fn b_1c60(c: &mut Machine) -> u32 {
    {
        // 0x00001c60 cmp r1,#0x0
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
        // 0x00001c62 ldr r7,[sp,#0x24]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(60, 4, value);
    }
    {
        // 0x00001c64 vsub.f32 s11,s15,s8
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(800, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00001c68 it lt
        let mut t185c00 = 0u64;
        t185c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
    }
    {
        // 0x00001c6a add.lt r1,r1,r0
        let mut t1d00 = 0u64;
        let mut t2700 = 0u64;
        t1d00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2700 = ((((t1d00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x1c6cu32;
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
    0x1c6c
}
pub fn b_1c6c(c: &mut Machine) -> u32 {
    {
        // 0x00001c6c subs r7,r2,r7
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
        // 0x00001c6e it mi
        let mut t184c00 = 0u64;
        t184c00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x00001c70 add.mi r7,r7,r0
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x1c72u32;
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
    0x1c72
}
pub fn b_1c72(c: &mut Machine) -> u32 {
    {
        // 0x00001c72 add.w r1,r4,r1, lsl #0x2
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
        // 0x00001c76 vcmpe.f32 s11,#0
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
        t35c00 = ((f32::from_bits(c.r(812, 4) as u32)
            < f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(812, 4) as u32)
            == f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)) as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)
            <= f32::from_bits(c.r(812, 4) as u32)) as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(812, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x00001c7a add.w r7,r4,r7, lsl #0x2
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
        // 0x00001c7e vldr.32 s14,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00001c82 vldr.32 s12,[r7]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(60, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x00001c86 vmrs apsr,fpscr
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
        // 0x00001c8a vsub.f32 s12,s12,s14
        let value = (f32::from_bits(c.r(816, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00001c8e ite pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x00001c90 vstr.pl.32 s11,[r3,#0x10]
        let mut tf00 = 0u64;
        let mut t2600 = 0u64;
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2600 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2600 >> 0) & 0xffu64) != 0 {
            return 0x1c94u32;
        }
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(812, 4));
    }
    0x1c94
}
pub fn b_1c94(c: &mut Machine) -> u32 {
    {
        // 0x00001c94 vstr.mi.32 s7,[r3,#0x10]
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x1c98u32;
        }
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(796, 4));
    }
    0x1c98
}
pub fn b_1c98(c: &mut Machine) -> u32 {
    {
        // 0x00001c98 vfma.f32 s14,s15,s12
        let value = f32::from_bits(c.r(828, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(816, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001c9c vstr.32 s14,[r3,#-0x8]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(44, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x00001ca0 b 0x00001bb8
        return 0x1bb8u32;
    }
    0x1ca2
}
pub fn b_1ca2(c: &mut Machine) -> u32 {
    {
        // 0x00001ca2 ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00001ca4 str.w r2,[r6,#0xe7c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3708u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x00001ca8 cmp r0,r3
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
        // 0x00001caa ble 0x00001c1e
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x1c1eu32;
        }
    }
    0x1cac
}
pub fn b_1cac(c: &mut Machine) -> u32 {
    {
        // 0x00001cac ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00001cae vldr.32 s13,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00001cb2 adds r3,#0x1
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
        // 0x00001cb4 str r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00001cb6 str.w r3,[r6,#0xec4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3780u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00001cba b 0x00001ba2
        return 0x1ba2u32;
    }
    0x1cbc
}
pub fn b_1cbc(c: &mut Machine) -> u32 {
    {
        // 0x00001cbc ldrb.w r1,[r6,#0xeb8]
        let mut t19c800 = 0u64;
        let mut t1aa600 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3768u64)) & 0xffffffffu64;
        t1aa600 = (c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 1)) & 0xffu64;
        let value = ((t1aa600 >> 0) & 0xffu64);
        c.w(36, 4, value);
    }
    {
        // 0x00001cc0 ldrd r2,r3,[sp,#0x38]
        let mut t19dc00 = 0u64;
        let mut t1ab800 = 0u64;
        t19dc00 = ((c.r(84, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        let value = c.load(((t19dc00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
        t1ab800 = (((t19dc00 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t1ab800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00001cc4 cbz r1,0x00001cea
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(36, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x1ceau32;
        }
    }
    0x1cc6
}
pub fn b_1cc6(c: &mut Machine) -> u32 {
    {
        // 0x00001cc6 ldr r1,[0x00001df4]
        let value = c.load(0x1df4, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001cc8 vmov.f32 s12,0x3f000000
        let value = 1056964608u64;
        c.w(816, 4, value);
    }
    {
        // 0x00001ccc vmov.f32 s14,0x3f800000
        let value = 1065353216u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001cd0 vldr.32 s15,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00001cd4 vmul.f32 s15,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001cd8 vmul.f32 s13,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00001cdc vcmpe.f32 s13,s9
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
        // 0x00001ce0 vmrs apsr,fpscr
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
        // 0x00001ce4 it pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x00001ce6 vmov.pl.f32 s13,s14
        let mut tf00 = 0u64;
        let mut t2700 = 0u64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2700 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x1ceau32;
        }
        let value = c.r(824, 4);
        c.w(820, 4, value);
    }
    0x1cea
}
