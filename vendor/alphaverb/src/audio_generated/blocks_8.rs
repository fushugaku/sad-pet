#![allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut,
    unused_comparisons
)]
use crate::machine::{signed, signed_overflow, Machine};
pub fn b_6002cb28(c: &mut Machine) -> u32 {
    {
        // 0x6002cb28 ldr r1,[0x6002cd64]
        let value = c.load(0x6002cd64, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002cb2a vldr.32 s0,[pc,#0x234]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610795818u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(564u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002cb2e str.w r1,[r10,#0x20c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(524u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002cb32 ldr r1,[0x6002cd70]
        let value = c.load(0x6002cd70, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002cb34 vmov.f32 s15,0xbf800000
        let value = 3212836864u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002cb38 vstr.32 s13,[r10,#0x1f8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(504u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x6002cb3c vmov.f32 s16,0x3f800000
        let value = 1065353216u64;
        c.w(832, 4, value);
    }
    {
        // 0x6002cb40 str.w r1,[r10,#0x200]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(512u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002cb44 ldr r1,[0x6002cd78]
        let value = c.load(0x6002cd78, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002cb46 vfma.f32 s15,s14,s14
        let value = f32::from_bits(c.r(824, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(824, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002cb4a vstr.32 s0,[r10,#0x214]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(532u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002cb4e vstr.32 s0,[r10,#0x208]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(520u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002cb52 vstr.32 s16,[r10,#0x204]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(516u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(832, 4));
    }
    {
        // 0x6002cb56 str.w r1,[r10,#0x210]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(528u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002cb5a vstr.32 s15,[r10,#0x1f0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(496u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002cb5e bl 0x60040c70
        let value = (1610795874u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002cb62
}
pub fn b_6002cb62(c: &mut Machine) -> u32 {
    {
        // 0x6002cb62 vldr.32 s15,[r10,#0x200]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(512u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002cb66 vldr.32 s13,[r10,#0x20c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(524u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002cb6a vmul.f32 s15,s0,s15
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002cb6e vldr.32 s14,[pc,#0x210]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610795886u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(528u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002cb72 vstr.32 s0,[r10,#0x204]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(516u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002cb76 vmul.f32 s15,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002cb7a vcmpe.f32 s15,s14
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
        t35c00 = ((f32::from_bits(c.r(828, 4) as u32) < f32::from_bits(c.r(824, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(828, 4) as u32) == f32::from_bits(c.r(824, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(824, 4) as u32) <= f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(824, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002cb7e vmrs apsr,fpscr
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
        // 0x6002cb82 bpl.w 0x6002dba4
        let mut t182d00 = 0u64;
        t182d00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        if ((t182d00 >> 0) & 0xffu64) != 0 {
            return 0x6002dba4u32;
        }
    }
    0x6002cb86
}
pub fn b_6002cb86(c: &mut Machine) -> u32 {
    {
        // 0x6002cb86 vmul.f32 s14,s15,s15
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002cb8a vldr.32 s11,[pc,#0x1f8]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610795914u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(504u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x6002cb8e vldr.32 s12,[pc,#0x1f8]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610795918u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(504u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x6002cb92 vmul.f32 s13,s14,s15
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002cb96 vfma.f32 s15,s13,s11
        let value = f32::from_bits(c.r(820, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(812, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002cb9a vmul.f32 s14,s14,s13
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002cb9e vfma.f32 s15,s14,s12
        let value = f32::from_bits(c.r(824, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(816, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002cba2 vadd.f32 s16,s15,s16
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(832, 4) as u32))
            .to_bits() as u64;
        c.w(832, 4, value);
    }
    {
        // 0x6002cba6 vdiv.f32 s14,s15,s16
        let value = (f32::from_bits(c.r(828, 4) as u32) / f32::from_bits(c.r(832, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    0x6002cbaa
}
pub fn b_6002cbaa(c: &mut Machine) -> u32 {
    {
        // 0x6002cbaa ldr.w r12,[0x6002cd3c]
        let value = c.load(0x6002cd3c, 4);
        c.w(80, 4, value);
    }
    {
        // 0x6002cbae mov.w lr,#0x3f000000
        let mut t188100 = 0u64;
        let mut t188300 = 0u64;
        t188100 = (1056964608u64) & 0xffffffffu64;
        t188300 = (if 31u64 >= 32 {
            0
        } else {
            ((t188100 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188300 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((t188100 >> 0) & 0xffffffffu64);
        c.w(88, 4, value);
        let value = (signed(c.r(88, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(88, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002cbb2 ldr r0,[0x6002cd8c]
        let value = c.load(0x6002cd8c, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002cbb4 str.w r12,[r10,#0x278]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(632u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002cbb8 str.w r12,[r10,#0x27c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(636u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002cbbc str.w r12,[r10,#0x284]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(644u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002cbc0 vmov.f32 s15,0xbf800000
        let value = 3212836864u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002cbc4 str.w r12,[r10,#0x290]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(656u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002cbc8 vmov.f32 s18,0x3f800000
        let value = 1065353216u64;
        c.w(840, 4, value);
    }
    {
        // 0x6002cbcc ldr.w r12,[0x6002cd64]
        let value = c.load(0x6002cd64, 4);
        c.w(80, 4, value);
    }
    {
        // 0x6002cbd0 vfma.f32 s15,s0,s0
        let value = f32::from_bits(c.r(768, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(768, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002cbd4 vldr.32 s0,[pc,#0x188]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610795988u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(392u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002cbd8 str.w r12,[r10,#0x218]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(536u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002cbdc str.w r12,[r10,#0x230]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(560u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002cbe0 ldr.w r12,[0x6002cd98]
        let value = c.load(0x6002cd98, 4);
        c.w(80, 4, value);
    }
    {
        // 0x6002cbe4 str.w lr,[r10,#0x280]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(640u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(88, 4));
    }
    {
        // 0x6002cbe8 str.w r12,[r10,#0x224]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(548u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002cbec str.w r12,[r10,#0x22c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(556u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002cbf0 ldr.w r12,[0x6002cd9c]
        let value = c.load(0x6002cd9c, 4);
        c.w(80, 4, value);
    }
    {
        // 0x6002cbf4 str.w lr,[r10,#0x298]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(664u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(88, 4));
    }
    {
        // 0x6002cbf8 str.w r12,[r10,#0x23c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(572u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002cbfc str.w r12,[r10,#0x244]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(580u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002cc00 ldr.w r12,[0x6002cda0]
        let value = c.load(0x6002cda0, 4);
        c.w(80, 4, value);
    }
    {
        // 0x6002cc04 str.w r0,[r11,#0x16c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(364u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002cc08 str.w r12,[r10,#0x21c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(540u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002cc0c ldr.w r12,[0x6002cda4]
        let value = c.load(0x6002cda4, 4);
        c.w(80, 4, value);
    }
    {
        // 0x6002cc10 str.w r0,[r11,#0x21c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(540u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002cc14 str.w r12,[r10,#0x234]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(564u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002cc18 ldr.w r12,[0x6002cda8]
        let value = c.load(0x6002cda8, 4);
        c.w(80, 4, value);
    }
    {
        // 0x6002cc1c str.w r0,[r11,#0x2cc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(716u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002cc20 str.w r12,[r10,#0x288]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(648u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002cc24 ldr.w r12,[0x6002cdac]
        let value = c.load(0x6002cdac, 4);
        c.w(80, 4, value);
    }
    {
        // 0x6002cc28 vstr.32 s14,[r10,#0x210]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(528u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002cc2c str.w r12,[r10,#0x294]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(660u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002cc30 vstr.32 s15,[r10,#0x208]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(520u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002cc34 vstr.32 s18,[r10,#0x228]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(552u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(840, 4));
    }
    {
        // 0x6002cc38 vstr.32 s18,[r10,#0x240]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(576u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(840, 4));
    }
    {
        // 0x6002cc3c vstr.32 s0,[r10,#0x1e0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(480u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002cc40 vstr.32 s0,[r10,#0x1e4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(484u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002cc44 vstr.32 s0,[r10,#0x220]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(544u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002cc48 vstr.32 s0,[r10,#0x238]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(568u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002cc4c vstr.32 s0,[r10,#0x248]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(584u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002cc50 vstr.32 s0,[r10,#0x24c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(588u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002cc54 vstr.32 s0,[r10,#0x250]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(592u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002cc58 vstr.32 s0,[r10,#0x254]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(596u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002cc5c vstr.32 s0,[r10,#0x28c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(652u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002cc60 str.w r0,[r11,#0x37c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(892u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002cc64 ldr r3,[sp,#0x3c]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(60u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002cc66 str.w r0,[r8,#0x0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(64, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002cc6a str r0,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002cc6c ldr r3,[sp,#0x40]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002cc6e str r0,[r7,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(60, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002cc70 str r0,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002cc72 ldr r3,[sp,#0x20]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002cc74 str r0,[r6,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002cc76 str r0,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002cc78 ldr r3,[sp,#0x24]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002cc7a str r0,[r4,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002cc7c str r0,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002cc7e ldr r3,[sp,#0x28]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002cc80 str r0,[r5,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002cc82 str r0,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002cc84 ldr r3,[sp,#0x2c]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002cc86 vldr.32 s10,[pc,#0xcc]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610796166u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(204u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x6002cc8a str r0,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002cc8c ldr r3,[sp,#0x30]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002cc8e str.w r0,[r10,#0x40]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002cc92 str r0,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002cc94 ldr r3,[sp,#0x34]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002cc96 str.w r0,[r10,#0xa0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(160u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002cc9a str r0,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002cc9c ldr r3,[0x6002cd90]
        let value = c.load(0x6002cd90, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002cc9e str.w r0,[r10,#0x100]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(256u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002cca2 str r0,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002cca4 str r0,[r3,#0x60]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(96u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002cca6 str.w r0,[r3,#0xc0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(44, 4)).wrapping_add(192u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002ccaa str.w r0,[r3,#0x120]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(44, 4)).wrapping_add(288u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002ccae vstr.32 s0,[r10,#0x32c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(812u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002ccb2 vstr.32 s0,[r10,#0x330]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(816u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002ccb6 vstr.32 s0,[r10,#0x334]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(820u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002ccba vstr.32 s0,[r10,#0x338]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(824u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002ccbe vstr.32 s0,[r10,#0x33c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(828u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002ccc2 vldr.32 s12,[r11,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x6002ccc6 vldr.32 s13,[r11,#0xb4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(180u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002ccca vcmpe.f32 s12,s10
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
        t35c00 = ((f32::from_bits(c.r(816, 4) as u32) < f32::from_bits(c.r(808, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(816, 4) as u32) == f32::from_bits(c.r(808, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(808, 4) as u32) <= f32::from_bits(c.r(816, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(816, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(808, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002ccce ldr r3,[sp,#0x38]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002ccd0 vldr.32 s15,[r9]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(68, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002ccd4 vldr.32 s14,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002ccd8 vmrs apsr,fpscr
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
        // 0x6002ccdc ldr.w r3,[r11,#0xbd4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3028u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002cce0 vcmpe.f32 s13,s10
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
        t35c00 = ((f32::from_bits(c.r(820, 4) as u32) < f32::from_bits(c.r(808, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(820, 4) as u32) == f32::from_bits(c.r(808, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(808, 4) as u32) <= f32::from_bits(c.r(820, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(820, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(808, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002cce4 ldr r1,[0x6002cd94]
        let value = c.load(0x6002cd94, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002cce6 str.w lr,[r10,#0x37c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(892u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(88, 4));
    }
    {
        // 0x6002ccea vselgt.f32 s12,s10,s12
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(808, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(816, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(816, 4, value);
    }
    {
        // 0x6002ccee vstr.32 s12,[r11,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x6002ccf2 vmrs apsr,fpscr
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
        // 0x6002ccf6 vstr.32 s0,[r10,#0x340]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(832u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002ccfa vstr.32 s0,[r10,#0x344]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(836u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002ccfe vstr.32 s0,[r10,#0x348]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(840u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002cd02 vselgt.f32 s13,s10,s13
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(808, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(820, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(820, 4, value);
    }
    {
        // 0x6002cd06 vstr.32 s13,[r11,#0xb8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(184u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x6002cd0a vmov s13,r3
        let value = c.r(44, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002cd0e ldr.w r3,[r11,#0xc34]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3124u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002cd12 vstr.32 s0,[r10,#0x34c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(844u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002cd16 vcvt.f32.s32 s13,s13
        let mut t3b200 = 0u64;
        let mut t3b300 = 0u64;
        let mut t3b500 = 0u64;
        t3b200 = (if 22u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (22u64 as u32)
        }) & 0xffffffffu64;
        t3b300 = (((t3b200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        t3b500 = (((t3b300 >> 0) & 0xffu64) & 3u64) & 0xffu64;
        let value = c.userop(219, c.r(820, 4), ((t3b500 >> 0) & 0xffu64), 0u64, 4, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002cd1a vstr.32 s0,[r10,#0x350]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(848u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002cd1e vstr.32 s0,[r10,#0x354]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(852u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002cd22 vstr.32 s0,[r10,#0x358]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(856u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002cd26 vdiv.f32 s12,s13,s10
        let value = (f32::from_bits(c.r(820, 4) as u32) / f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x6002cd2a vmov s13,r3
        let value = c.r(44, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002cd2e vstr.32 s0,[r10,#0x35c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(860u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002cd32 vstr.32 s0,[r10,#0x360]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(864u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002cd36 vstr.32 s0,[r10,#0x364]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(868u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002cd3a b 0x6002cdb4
        return 0x6002cdb4u32;
    }
    0x6002cd3c
}
pub fn b_6002cdb4(c: &mut Machine) -> u32 {
    {
        // 0x6002cdb4 vstr.32 s0,[r10,#0x368]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(872u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002cdb8 vcvt.f32.s32 s13,s13
        let mut t3b200 = 0u64;
        let mut t3b300 = 0u64;
        let mut t3b500 = 0u64;
        t3b200 = (if 22u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (22u64 as u32)
        }) & 0xffffffffu64;
        t3b300 = (((t3b200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        t3b500 = (((t3b300 >> 0) & 0xffu64) & 3u64) & 0xffu64;
        let value = c.userop(219, c.r(820, 4), ((t3b500 >> 0) & 0xffu64), 0u64, 4, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002cdbc vldr.32 s16,[pc,#-0x10]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((1610796476u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(16u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(832, 4, value);
    }
    {
        // 0x6002cdc0 ldr.w r9,[0x6002d200]
        let value = c.load(0x6002d200, 4);
        c.w(68, 4, value);
    }
    {
        // 0x6002cdc4 vdiv.f32 s11,s13,s10
        let value = (f32::from_bits(c.r(820, 4) as u32) / f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x6002cdc8 add.w r8,r9,#0x60
        let mut t187d00 = 0u64;
        t187d00 = (96u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = ((c.r(68, 4) as u128 + ((t187d00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(68, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(68, 4)).wrapping_add(((t187d00 >> 0) & 0xffffffffu64));
        c.w(64, 4, value);
        let value = (signed(c.r(64, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(64, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002cdcc add.w r7,r9,#0xc0
        let mut t187d00 = 0u64;
        t187d00 = (192u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = ((c.r(68, 4) as u128 + ((t187d00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(68, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(68, 4)).wrapping_add(((t187d00 >> 0) & 0xffffffffu64));
        c.w(60, 4, value);
        let value = (signed(c.r(60, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(60, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002cdd0 add.w r6,r9,#0x120
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (288u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(68, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(68, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(68, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(56, 4, value);
        let value = (signed(c.r(56, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(56, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002cdd4 add.w r5,r9,#0x180
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (384u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(68, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(68, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(68, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002cdd8 add.w r4,r9,#0x1e0
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (480u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(68, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(68, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(68, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002cddc vmul.f32 s15,s12,s15
        let value = (f32::from_bits(c.r(816, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002cde0 vdiv.f32 s9,s18,s15
        let value = (f32::from_bits(c.r(840, 4) as u32) / f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(804, 4, value);
    }
    {
        // 0x6002cde4 vmul.f32 s14,s11,s14
        let value = (f32::from_bits(c.r(812, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002cde8 vstr.32 s11,[r1,#0x60]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(96u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(812, 4));
    }
    {
        // 0x6002cdec vdiv.f32 s13,s18,s14
        let value = (f32::from_bits(c.r(840, 4) as u32) / f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002cdf0 vstr.32 s14,[r1,#0x64]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(100u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002cdf4 vstr.32 s9,[r1,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(804, 4));
    }
    {
        // 0x6002cdf8 vstr.32 s13,[r1,#0x68]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(104u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x6002cdfc vstmia r1!,{s12}
        let mut t5ad00 = 0u64;
        let value = c.r(36, 4);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 4, c.r(816, 4));
        let value = (c.r(128, 4)).wrapping_add(4u64);
        c.w(128, 4, value);
        t5ad00 = (if 2u64 >= 32 {
            0
        } else {
            (1u64) << (2u64 as u32)
        }) & 0xffffffffu64;
        let value = (c.r(36, 4)).wrapping_add(((t5ad00 >> 0) & 0xffffffffu64));
        c.w(36, 4, value);
    }
    {
        // 0x6002ce00 vstr.32 s15,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002ce04 bl 0x60040c70
        let value = (1610796552u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002ce08
}
pub fn b_6002ce08(c: &mut Machine) -> u32 {
    {
        // 0x6002ce08 vldr.32 s13,[pc,#0x3cc]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610796552u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(972u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002ce0c vldr.32 s14,[pc,#0x3cc]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610796556u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(972u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002ce10 vmul.f32 s0,s0,s13
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002ce14 vldr.32 s15,[r10,#0x374]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(884u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002ce18 vmul.f32 s15,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002ce1c vdiv.f32 s17,s18,s0
        let value = (f32::from_bits(c.r(840, 4) as u32) / f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(836, 4, value);
    }
    {
        // 0x6002ce20 vmul.f32 s0,s15,s16
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(832, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002ce24 vstr.32 s17,[r10,#0x36c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(876u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(836, 4));
    }
    {
        // 0x6002ce28 bl 0x60040c70
        let value = (1610796588u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002ce2c
}
pub fn b_6002ce2c(c: &mut Machine) -> u32 {
    {
        // 0x6002ce2c vdiv.f32 s15,s17,s0
        let value = (f32::from_bits(c.r(836, 4) as u32) / f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002ce30 vldr.32 s13,[pc,#0x3ac]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610796592u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(940u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002ce34 vldr.32 s14,[pc,#0x3ac]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610796596u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(940u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002ce38 vmov.f32 s17,0x3f800000
        let value = 1065353216u64;
        c.w(836, 4, value);
    }
    {
        // 0x6002ce3c vmov.f32 s18,s17
        let value = c.r(836, 4);
        c.w(840, 4, value);
    }
    {
        // 0x6002ce40 vmul.f32 s0,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002ce44 vcmpe.f32 s0,s14
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
        t35c00 = ((f32::from_bits(c.r(768, 4) as u32) < f32::from_bits(c.r(824, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(768, 4) as u32) == f32::from_bits(c.r(824, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(824, 4) as u32) <= f32::from_bits(c.r(768, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(768, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(824, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002ce48 vmrs apsr,fpscr
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
        // 0x6002ce4c ite gt
        let mut t185d00 = 0u64;
        let mut t185e00 = 0u64;
        let mut t186000 = 0u64;
        t185d00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t185e00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t186000 = (((((t185d00 >> 0) & 0xffu64) != 0) & (((t185e00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
    }
    {
        // 0x6002ce4e vmul.gt.f32 s0,s0,s16
        let mut t1e00 = 0u64;
        let mut t1f00 = 0u64;
        let mut t2100 = 0u64;
        let mut t2600 = 0u64;
        t1e00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t1f00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t2100 =
            (((((t1e00 >> 0) & 0xffu64) != 0) & (((t1f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        t2600 = ((((t2100 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2600 >> 0) & 0xffu64) != 0 {
            return 0x6002ce52u32;
        }
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(832, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    0x6002ce52
}
pub fn b_6002ce52(c: &mut Machine) -> u32 {
    {
        // 0x6002ce52 vldr.le.32 s0,[pc,#0x394]
        let mut t2200 = 0u64;
        let mut t2400 = 0u64;
        let mut t2700 = 0u64;
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610796626u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(916u64)) & 0xffffffffu64;
        t2200 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2400 = (((c.r(97, 1) != 0) | (((t2200 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        t2700 = ((((t2400 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x6002ce56u32;
        }
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    0x6002ce56
}
pub fn b_6002ce56(c: &mut Machine) -> u32 {
    {
        // 0x6002ce56 bl 0x60040c70
        let value = (1610796634u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002ce5a
}
pub fn b_6002ce5a(c: &mut Machine) -> u32 {
    {
        // 0x6002ce5a vneg.f32 s0,s0
        let value = (-f32::from_bits(c.r(768, 4) as u32)).to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002ce5e vstr.32 s0,[r10,#0x31c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(796u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002ce62 bl 0x6004137c
        let value = (1610796646u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x6004137cu32;
    }
    0x6002ce66
}
pub fn b_6002ce66(c: &mut Machine) -> u32 {
    {
        // 0x6002ce66 vmov.f32 s15,0x3f800000
        let value = 1065353216u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002ce6a vsub.f32 s0,s15,s0
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002ce6e bl 0x60041338
        let value = (1610796658u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041338u32;
    }
    0x6002ce72
}
pub fn b_6002ce72(c: &mut Machine) -> u32 {
    {
        // 0x6002ce72 vldr.32 s16,[r10,#0x36c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(876u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(832, 4, value);
    }
    {
        // 0x6002ce76 vldr.32 s11,[r10,#0x370]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(880u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x6002ce7a vldr.32 s13,[pc,#0x370]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610796666u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(880u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002ce7e vmul.f32 s11,s16,s11
        let value = (f32::from_bits(c.r(832, 4) as u32) * f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x6002ce82 vldr.32 s12,[pc,#0x36c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610796674u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(876u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x6002ce86 vldr.32 s14,[pc,#0x37c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610796678u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(892u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002ce8a vmul.f32 s8,s16,s12
        let value = (f32::from_bits(c.r(832, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(800, 4, value);
    }
    {
        // 0x6002ce8e vldr.32 s6,[pc,#0x378]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610796686u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(888u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x6002ce92 vmul.f32 s10,s11,s13
        let value = (f32::from_bits(c.r(812, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x6002ce96 vldr.32 s15,[pc,#0x35c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610796694u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(860u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002ce9a vfma.f32 s6,s0,s14
        let value = f32::from_bits(c.r(768, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(824, 4) as u32),
                f32::from_bits(c.r(792, 4) as u32),
            )
            .to_bits() as u64;
        c.w(792, 4, value);
    }
    {
        // 0x6002ce9e vldr.32 s2,[r11,#0x164]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(356u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(776, 4, value);
    }
    {
        // 0x6002cea2 vmul.f32 s14,s8,s12
        let value = (f32::from_bits(c.r(800, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002cea6 vstr.32 s8,[r10,#0x364]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(868u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(800, 4));
    }
    {
        // 0x6002ceaa vmul.f32 s9,s10,s13
        let value = (f32::from_bits(c.r(808, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(804, 4, value);
    }
    {
        // 0x6002ceae vldr.32 s3,[r11,#0x214]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(532u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(780, 4, value);
    }
    {
        // 0x6002ceb2 vldr.32 s4,[r11,#0x2c4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(708u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(784, 4, value);
    }
    {
        // 0x6002ceb6 vmov.f32 s0,s17
        let value = c.r(836, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002ceba vstr.32 s14,[r10,#0x360]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(864u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002cebe vmul.f32 s14,s14,s12
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002cec2 vmul.f32 s8,s9,s13
        let value = (f32::from_bits(c.r(804, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(800, 4, value);
    }
    {
        // 0x6002cec6 vldr.32 s5,[r11,#0x374]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(884u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(788, 4, value);
    }
    {
        // 0x6002ceca ldr r3,[sp,#0x10]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002cecc vstr.32 s14,[r10,#0x35c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(860u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002ced0 vmul.f32 s14,s14,s12
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002ced4 vmul.f32 s7,s8,s13
        let value = (f32::from_bits(c.r(800, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(796, 4, value);
    }
    {
        // 0x6002ced8 vstr.32 s6,[r10,#0x37c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(892u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(792, 4));
    }
    {
        // 0x6002cedc vldr.32 s6,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x6002cee0 vstr.32 s14,[r10,#0x358]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(856u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002cee4 vmul.f32 s14,s14,s12
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002cee8 vmul.f32 s13,s7,s13
        let value = (f32::from_bits(c.r(796, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002ceec ldr r3,[sp,#0x18]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(24u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002ceee vstr.32 s11,[r10,#0x348]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(840u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(812, 4));
    }
    {
        // 0x6002cef2 vstr.32 s14,[r10,#0x354]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(852u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002cef6 vmul.f32 s14,s14,s12
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002cefa vcmpe.f32 s13,s15
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
        t35c00 = ((f32::from_bits(c.r(820, 4) as u32) < f32::from_bits(c.r(828, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(820, 4) as u32) == f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(828, 4) as u32) <= f32::from_bits(c.r(820, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(820, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002cefe vstr.32 s16,[r10,#0x368]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(872u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(832, 4));
    }
    {
        // 0x6002cf02 vstr.32 s10,[r10,#0x344]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(836u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(808, 4));
    }
    {
        // 0x6002cf06 vmul.f32 s12,s14,s12
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x6002cf0a vstr.32 s14,[r10,#0x350]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(848u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002cf0e vmrs apsr,fpscr
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
        // 0x6002cf12 vstr.32 s9,[r10,#0x340]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(832u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(804, 4));
    }
    {
        // 0x6002cf16 vstr.32 s8,[r10,#0x33c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(828u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(800, 4));
    }
    {
        // 0x6002cf1a vstr.32 s12,[r10,#0x34c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(844u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x6002cf1e vselgt.f32 s14,s13,s15
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(820, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(828, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(824, 4, value);
    }
    {
        // 0x6002cf22 vcmpe.f32 s2,s14
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
        t35c00 = ((f32::from_bits(c.r(776, 4) as u32) < f32::from_bits(c.r(824, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(776, 4) as u32) == f32::from_bits(c.r(824, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(824, 4) as u32) <= f32::from_bits(c.r(776, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(776, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(824, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002cf26 vldr.32 s12,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x6002cf2a ldr r3,[sp,#0xc]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(12u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002cf2c vmrs apsr,fpscr
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
        // 0x6002cf30 vstr.32 s7,[r10,#0x338]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(824u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(796, 4));
    }
    {
        // 0x6002cf34 vcmpe.f32 s7,s15
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
        t35c00 = ((f32::from_bits(c.r(796, 4) as u32) < f32::from_bits(c.r(828, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(796, 4) as u32) == f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(828, 4) as u32) <= f32::from_bits(c.r(796, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(796, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002cf38 vstr.32 s13,[r10,#0x334]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(820u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x6002cf3c vldr.32 s1,[pc,#0x2cc]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610796860u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(716u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(772, 4, value);
    }
    {
        // 0x6002cf40 vselgt.f32 s2,s14,s2
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(824, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(776, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(776, 4, value);
    }
    {
        // 0x6002cf44 vstr.32 s2,[r11,#0x168]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(360u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(776, 4));
    }
    {
        // 0x6002cf48 vmrs apsr,fpscr
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
        // 0x6002cf4c vldr.32 s14,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002cf50 ldr r3,[sp,#0x14]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(20u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002cf52 ldr r1,[0x6002d1f8]
        let value = c.load(0x6002d1f8, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002cf54 vselgt.f32 s2,s7,s15
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(796, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(828, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(776, 4, value);
    }
    {
        // 0x6002cf58 vcmpe.f32 s3,s2
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
        t35c00 = ((f32::from_bits(c.r(780, 4) as u32) < f32::from_bits(c.r(776, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(780, 4) as u32) == f32::from_bits(c.r(776, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(776, 4) as u32) <= f32::from_bits(c.r(780, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(780, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(776, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002cf5c vmrs apsr,fpscr
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
        // 0x6002cf60 vcmpe.f32 s8,s15
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
        t35c00 = ((f32::from_bits(c.r(800, 4) as u32) < f32::from_bits(c.r(828, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(800, 4) as u32) == f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(828, 4) as u32) <= f32::from_bits(c.r(800, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(800, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002cf64 vselgt.f32 s3,s2,s3
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(776, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(780, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(780, 4, value);
    }
    {
        // 0x6002cf68 vstr.32 s3,[r11,#0x218]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(536u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(780, 4));
    }
    {
        // 0x6002cf6c vmrs apsr,fpscr
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
        // 0x6002cf70 vselgt.f32 s3,s8,s15
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(800, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(828, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(780, 4, value);
    }
    {
        // 0x6002cf74 vcmpe.f32 s4,s3
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
        t35c00 = ((f32::from_bits(c.r(784, 4) as u32) < f32::from_bits(c.r(780, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(784, 4) as u32) == f32::from_bits(c.r(780, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(780, 4) as u32) <= f32::from_bits(c.r(784, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(784, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(780, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002cf78 vmrs apsr,fpscr
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
        // 0x6002cf7c vcmpe.f32 s9,s15
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
        t35c00 = ((f32::from_bits(c.r(804, 4) as u32) < f32::from_bits(c.r(828, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(804, 4) as u32) == f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(828, 4) as u32) <= f32::from_bits(c.r(804, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(804, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002cf80 vselgt.f32 s4,s3,s4
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(780, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(784, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(784, 4, value);
    }
    {
        // 0x6002cf84 vstr.32 s4,[r11,#0x2c8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(712u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(784, 4));
    }
    {
        // 0x6002cf88 vmrs apsr,fpscr
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
        // 0x6002cf8c vselgt.f32 s4,s9,s15
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(804, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(828, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(784, 4, value);
    }
    {
        // 0x6002cf90 vcmpe.f32 s5,s4
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
        t35c00 = ((f32::from_bits(c.r(788, 4) as u32) < f32::from_bits(c.r(784, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(788, 4) as u32) == f32::from_bits(c.r(784, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(784, 4) as u32) <= f32::from_bits(c.r(788, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(788, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(784, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002cf94 vmrs apsr,fpscr
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
        // 0x6002cf98 vcmpe.f32 s10,s15
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
        t35c00 = ((f32::from_bits(c.r(808, 4) as u32) < f32::from_bits(c.r(828, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(808, 4) as u32) == f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(828, 4) as u32) <= f32::from_bits(c.r(808, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(808, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002cf9c vselgt.f32 s5,s4,s5
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(784, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(788, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(788, 4, value);
    }
    {
        // 0x6002cfa0 vstr.32 s5,[r11,#0x378]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(888u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(788, 4));
    }
    {
        // 0x6002cfa4 vmrs apsr,fpscr
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
        // 0x6002cfa8 vselgt.f32 s5,s10,s15
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(808, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(828, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(788, 4, value);
    }
    {
        // 0x6002cfac vcmpe.f32 s6,s5
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
        t35c00 = ((f32::from_bits(c.r(792, 4) as u32) < f32::from_bits(c.r(788, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(792, 4) as u32) == f32::from_bits(c.r(788, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(788, 4) as u32) <= f32::from_bits(c.r(792, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(792, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(788, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002cfb0 vmrs apsr,fpscr
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
        // 0x6002cfb4 vcmpe.f32 s11,s15
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
        t35c00 = ((f32::from_bits(c.r(812, 4) as u32) < f32::from_bits(c.r(828, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(812, 4) as u32) == f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(828, 4) as u32) <= f32::from_bits(c.r(812, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(812, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002cfb8 vselgt.f32 s6,s5,s6
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(788, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(792, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(792, 4, value);
    }
    {
        // 0x6002cfbc vstr.32 s6,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(792, 4));
    }
    {
        // 0x6002cfc0 vmrs apsr,fpscr
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
        // 0x6002cfc4 ldr r3,[sp,#0x1c]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002cfc6 vldr.32 s5,[pc,#0x214]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610796998u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(532u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(788, 4, value);
    }
    {
        // 0x6002cfca vselgt.f32 s15,s11,s15
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(812, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(828, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(828, 4, value);
    }
    {
        // 0x6002cfce vcmpe.f32 s12,s15
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
        t35c00 = ((f32::from_bits(c.r(816, 4) as u32) < f32::from_bits(c.r(828, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(816, 4) as u32) == f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(828, 4) as u32) <= f32::from_bits(c.r(816, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(816, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002cfd2 vmrs apsr,fpscr
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
        // 0x6002cfd6 vcmpe.f32 s13,s1
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
        t35c00 = ((f32::from_bits(c.r(820, 4) as u32) < f32::from_bits(c.r(772, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(820, 4) as u32) == f32::from_bits(c.r(772, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(772, 4) as u32) <= f32::from_bits(c.r(820, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(820, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(772, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002cfda vselgt.f32 s12,s15,s12
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(828, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(816, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(816, 4, value);
    }
    {
        // 0x6002cfde vstr.32 s12,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x6002cfe2 ldr.w r3,[r11,#0xc94]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3220u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002cfe6 vmrs apsr,fpscr
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
        // 0x6002cfea vcmpe.f32 s7,s1
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
        t35c00 = ((f32::from_bits(c.r(796, 4) as u32) < f32::from_bits(c.r(772, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(796, 4) as u32) == f32::from_bits(c.r(772, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(772, 4) as u32) <= f32::from_bits(c.r(796, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(796, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(772, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002cfee vmov s15,r3
        let value = c.r(44, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002cff2 mov r3,r9
        let value = c.r(68, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002cff4 vselgt.f32 s13,s13,s1
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(820, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(772, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(820, 4, value);
    }
    {
        // 0x6002cff8 vcvt.f32.s32 s15,s15
        let mut t3b200 = 0u64;
        let mut t3b300 = 0u64;
        let mut t3b500 = 0u64;
        t3b200 = (if 22u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (22u64 as u32)
        }) & 0xffffffffu64;
        t3b300 = (((t3b200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        t3b500 = (((t3b300 >> 0) & 0xffu64) & 3u64) & 0xffu64;
        let value = c.userop(219, c.r(828, 4), ((t3b500 >> 0) & 0xffu64), 0u64, 4, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002cffc adds r3,#0x4
        let value = ((c.r(44, 4) as u128 + 4u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), 4u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_add(4u64);
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
        // 0x6002cffe vmrs apsr,fpscr
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
        // 0x6002d002 str r3,[sp,#0x20]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002d004 vcmpe.f32 s8,s1
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
        t35c00 = ((f32::from_bits(c.r(800, 4) as u32) < f32::from_bits(c.r(772, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(800, 4) as u32) == f32::from_bits(c.r(772, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(772, 4) as u32) <= f32::from_bits(c.r(800, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(800, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(772, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d008 vdiv.f32 s12,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) / f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x6002d00c vldr.32 s13,[r9,#0x108]
        let mut t64000 = 0u64;
        t64000 = ((c.r(68, 4)).wrapping_add(264u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002d010 vselgt.f32 s7,s7,s1
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(796, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(772, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(796, 4, value);
    }
    {
        // 0x6002d014 vmrs apsr,fpscr
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
        // 0x6002d018 vcmpe.f32 s9,s1
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
        t35c00 = ((f32::from_bits(c.r(804, 4) as u32) < f32::from_bits(c.r(772, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(804, 4) as u32) == f32::from_bits(c.r(772, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(772, 4) as u32) <= f32::from_bits(c.r(804, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(804, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(772, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d01c vselgt.f32 s8,s8,s1
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(800, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(772, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(800, 4, value);
    }
    {
        // 0x6002d020 vmrs apsr,fpscr
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
        // 0x6002d024 vcmpe.f32 s10,s1
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
        t35c00 = ((f32::from_bits(c.r(808, 4) as u32) < f32::from_bits(c.r(772, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(808, 4) as u32) == f32::from_bits(c.r(772, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(772, 4) as u32) <= f32::from_bits(c.r(808, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(808, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(772, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d028 vselgt.f32 s9,s9,s1
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(804, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(772, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(804, 4, value);
    }
    {
        // 0x6002d02c vmrs apsr,fpscr
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
        // 0x6002d030 vcmpe.f32 s11,s1
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
        t35c00 = ((f32::from_bits(c.r(812, 4) as u32) < f32::from_bits(c.r(772, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(812, 4) as u32) == f32::from_bits(c.r(772, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(772, 4) as u32) <= f32::from_bits(c.r(812, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(812, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(772, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d034 vmul.f32 s15,s12,s14
        let value = (f32::from_bits(c.r(816, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d038 vldr.32 s14,[r3,#0x44]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002d03c ldr.w r3,[r11,#0xcf4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3316u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d040 vselgt.f32 s10,s10,s1
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(808, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(772, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(808, 4, value);
    }
    {
        // 0x6002d044 vmrs apsr,fpscr
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
        // 0x6002d048 vstr.32 s12,[r9]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(68, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x6002d04c vdiv.f32 s2,s17,s15
        let value = (f32::from_bits(c.r(836, 4) as u32) / f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(776, 4, value);
    }
    {
        // 0x6002d050 vstr.32 s15,[r9,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(68, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002d054 vmov s15,r3
        let value = c.r(44, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002d058 ldr.w r3,[r11,#0xd54]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3412u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d05c vldr.32 s12,[r9,#0xa8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(68, 4)).wrapping_add(168u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x6002d060 vselgt.f32 s11,s11,s1
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(812, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(772, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(812, 4, value);
    }
    {
        // 0x6002d064 vldr.32 s17,[pc,#0x194]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610797156u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(404u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(836, 4, value);
    }
    {
        // 0x6002d068 vcvt.f32.s32 s15,s15
        let mut t3b200 = 0u64;
        let mut t3b300 = 0u64;
        let mut t3b500 = 0u64;
        t3b200 = (if 22u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (22u64 as u32)
        }) & 0xffffffffu64;
        t3b300 = (((t3b200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        t3b500 = (((t3b300 >> 0) & 0xffu64) & 3u64) & 0xffu64;
        let value = c.userop(219, c.r(828, 4), ((t3b500 >> 0) & 0xffu64), 0u64, 4, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002d06c vdiv.f32 s6,s15,s7
        let value = (f32::from_bits(c.r(828, 4) as u32) / f32::from_bits(c.r(796, 4) as u32))
            .to_bits() as u64;
        c.w(792, 4, value);
    }
    {
        // 0x6002d070 vmov s15,r3
        let value = c.r(44, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002d074 ldr.w r3,[r11,#0xdb4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3508u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d078 vcvt.f32.s32 s15,s15
        let mut t3b200 = 0u64;
        let mut t3b300 = 0u64;
        let mut t3b500 = 0u64;
        t3b200 = (if 22u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (22u64 as u32)
        }) & 0xffffffffu64;
        t3b300 = (((t3b200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        t3b500 = (((t3b300 >> 0) & 0xffu64) & 3u64) & 0xffu64;
        let value = c.userop(219, c.r(828, 4), ((t3b500 >> 0) & 0xffu64), 0u64, 4, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002d07c vstr.32 s2,[r9,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(68, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(776, 4));
    }
    {
        // 0x6002d080 vmul.f32 s7,s6,s14
        let value = (f32::from_bits(c.r(792, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(796, 4, value);
    }
    {
        // 0x6002d084 vldr.32 s14,[r9,#0x168]
        let mut t64000 = 0u64;
        t64000 = ((c.r(68, 4)).wrapping_add(360u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002d088 vstr.32 s6,[r8]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(64, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(792, 4));
    }
    {
        // 0x6002d08c vdiv.f32 s19,s18,s7
        let value = (f32::from_bits(c.r(840, 4) as u32) / f32::from_bits(c.r(796, 4) as u32))
            .to_bits() as u64;
        c.w(844, 4, value);
    }
    {
        // 0x6002d090 vstr.32 s7,[r8,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(64, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(796, 4));
    }
    {
        // 0x6002d094 vdiv.f32 s18,s15,s8
        let value = (f32::from_bits(c.r(828, 4) as u32) / f32::from_bits(c.r(800, 4) as u32))
            .to_bits() as u64;
        c.w(840, 4, value);
    }
    {
        // 0x6002d098 vmov s15,r3
        let value = c.r(44, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002d09c ldr.w r3,[r11,#0xe14]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3604u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d0a0 vcvt.f32.s32 s15,s15
        let mut t3b200 = 0u64;
        let mut t3b300 = 0u64;
        let mut t3b500 = 0u64;
        t3b200 = (if 22u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (22u64 as u32)
        }) & 0xffffffffu64;
        t3b300 = (((t3b200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        t3b500 = (((t3b300 >> 0) & 0xffu64) & 3u64) & 0xffu64;
        let value = c.userop(219, c.r(828, 4), ((t3b500 >> 0) & 0xffu64), 0u64, 4, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002d0a4 vdiv.f32 s8,s15,s9
        let value = (f32::from_bits(c.r(828, 4) as u32) / f32::from_bits(c.r(804, 4) as u32))
            .to_bits() as u64;
        c.w(800, 4, value);
    }
    {
        // 0x6002d0a8 vmov s15,r3
        let value = c.r(44, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002d0ac ldr.w r3,[r11,#0xe74]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3700u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d0b0 vstr.32 s19,[r8,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(64, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(844, 4));
    }
    {
        // 0x6002d0b4 vcvt.f32.s32 s15,s15
        let mut t3b200 = 0u64;
        let mut t3b300 = 0u64;
        let mut t3b500 = 0u64;
        t3b200 = (if 22u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (22u64 as u32)
        }) & 0xffffffffu64;
        t3b300 = (((t3b200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        t3b500 = (((t3b300 >> 0) & 0xffu64) & 3u64) & 0xffu64;
        let value = c.userop(219, c.r(828, 4), ((t3b500 >> 0) & 0xffu64), 0u64, 4, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002d0b8 vdiv.f32 s9,s15,s10
        let value = (f32::from_bits(c.r(828, 4) as u32) / f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(804, 4, value);
    }
    {
        // 0x6002d0bc vmov s15,r3
        let value = c.r(44, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002d0c0 mov r3,r9
        let value = c.r(68, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d0c2 vstr.32 s18,[r7]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(60, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(840, 4));
    }
    {
        // 0x6002d0c6 adds r3,#0x8
        let value = ((c.r(44, 4) as u128 + 8u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), 8u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_add(8u64);
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
        // 0x6002d0c8 vcvt.f32.s32 s15,s15
        let mut t3b200 = 0u64;
        let mut t3b300 = 0u64;
        let mut t3b500 = 0u64;
        t3b200 = (if 22u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (22u64 as u32)
        }) & 0xffffffffu64;
        t3b300 = (((t3b200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        t3b500 = (((t3b300 >> 0) & 0xffu64) & 3u64) & 0xffu64;
        let value = c.userop(219, c.r(828, 4), ((t3b500 >> 0) & 0xffu64), 0u64, 4, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002d0cc str r3,[sp,#0x24]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002d0ce mov r3,r8
        let value = c.r(64, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d0d0 vmul.f32 s12,s18,s12
        let value = (f32::from_bits(c.r(840, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x6002d0d4 vdiv.f32 s10,s15,s11
        let value = (f32::from_bits(c.r(828, 4) as u32) / f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x6002d0d8 adds r3,#0x4
        let value = ((c.r(44, 4) as u128 + 4u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), 4u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_add(4u64);
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
        // 0x6002d0da vldr.32 s15,[r9,#0x1c8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(68, 4)).wrapping_add(456u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002d0de str r3,[sp,#0x28]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002d0e0 mov r3,r8
        let value = c.r(64, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d0e2 vstr.32 s8,[r6]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(56, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(800, 4));
    }
    {
        // 0x6002d0e6 adds r3,#0x8
        let value = ((c.r(44, 4) as u128 + 8u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), 8u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_add(8u64);
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
        // 0x6002d0e8 vstr.32 s12,[r7,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(60, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x6002d0ec vdiv.f32 s20,s0,s12
        let value = (f32::from_bits(c.r(768, 4) as u32) / f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(848, 4, value);
    }
    {
        // 0x6002d0f0 str r3,[sp,#0x2c]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002d0f2 mov r3,r7
        let value = c.r(60, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d0f4 adds r3,#0x4
        let value = ((c.r(44, 4) as u128 + 4u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), 4u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_add(4u64);
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
        // 0x6002d0f6 vstr.32 s9,[r5]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(52, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(804, 4));
    }
    {
        // 0x6002d0fa str r3,[sp,#0x30]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002d0fc mov r3,r7
        let value = c.r(60, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d0fe vmul.f32 s13,s8,s13
        let value = (f32::from_bits(c.r(800, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002d102 adds r3,#0x8
        let value = ((c.r(44, 4) as u128 + 8u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), 8u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_add(8u64);
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
        // 0x6002d104 vmul.f32 s14,s9,s14
        let value = (f32::from_bits(c.r(804, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002d108 str r3,[sp,#0x34]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002d10a mov r3,r6
        let value = c.r(56, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d10c vdiv.f32 s1,s0,s14
        let value = (f32::from_bits(c.r(768, 4) as u32) / f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(772, 4, value);
    }
    {
        // 0x6002d110 adds r3,#0x4
        let value = ((c.r(44, 4) as u128 + 4u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), 4u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_add(4u64);
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
        // 0x6002d112 vstr.32 s10,[r4]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(48, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(808, 4));
    }
    {
        // 0x6002d116 vmul.f32 s15,s10,s15
        let value = (f32::from_bits(c.r(808, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d11a vdiv.f32 s21,s0,s13
        let value = (f32::from_bits(c.r(768, 4) as u32) / f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(852, 4, value);
    }
    {
        // 0x6002d11e vstr.32 s20,[r7,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(60, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(848, 4));
    }
    {
        // 0x6002d122 vdiv.f32 s11,s0,s15
        let value = (f32::from_bits(c.r(768, 4) as u32) / f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x6002d126 vldr.32 s0,[r10,#0x374]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(884u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002d12a vstr.32 s13,[r6,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(56, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x6002d12e str r3,[sp,#0x38]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002d130 mov r3,r6
        let value = c.r(56, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d132 str.w r1,[r10,#0x370]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(880u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002d136 adds r3,#0x8
        let value = ((c.r(44, 4) as u128 + 8u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), 8u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_add(8u64);
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
        // 0x6002d138 vstr.32 s14,[r5,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002d13c vmul.f32 s0,s0,s5
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(788, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002d140 vstr.32 s1,[r5,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(772, 4));
    }
    {
        // 0x6002d144 str r3,[sp,#0x3c]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(60u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002d146 mov r3,r5
        let value = c.r(52, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d148 vstr.32 s15,[r4,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002d14c adds r3,#0x4
        let value = ((c.r(44, 4) as u128 + 4u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), 4u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_add(4u64);
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
        // 0x6002d14e vmul.f32 s0,s0,s17
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(836, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002d152 str r3,[sp,#0x40]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002d154 mov r3,r5
        let value = c.r(52, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d156 vstr.32 s21,[r6,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(56, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(852, 4));
    }
    {
        // 0x6002d15a adds r3,#0x8
        let value = ((c.r(44, 4) as u128 + 8u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), 8u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_add(8u64);
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
        // 0x6002d15c str r3,[sp,#0x44]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002d15e mov r3,r4
        let value = c.r(48, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d160 adds r3,#0x4
        let value = ((c.r(44, 4) as u128 + 4u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), 4u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_add(4u64);
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
        // 0x6002d162 vstr.32 s11,[r4,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(812, 4));
    }
    {
        // 0x6002d166 str r3,[sp,#0x48]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(72u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002d168 mov r3,r4
        let value = c.r(48, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d16a adds r3,#0x8
        let value = ((c.r(44, 4) as u128 + 8u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), 8u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_add(8u64);
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
        // 0x6002d16c str r3,[sp,#0x4c]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(76u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002d16e bl 0x60040c70
        let value = (1610797426u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002d172
}
pub fn b_6002d172(c: &mut Machine) -> u32 {
    {
        // 0x6002d172 vdiv.f32 s15,s16,s0
        let value = (f32::from_bits(c.r(832, 4) as u32) / f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d176 vldr.32 s13,[pc,#0x68]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610797430u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(104u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002d17a vldr.32 s14,[pc,#0x68]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610797434u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(104u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002d17e vmul.f32 s0,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002d182 vcmpe.f32 s0,s14
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
        t35c00 = ((f32::from_bits(c.r(768, 4) as u32) < f32::from_bits(c.r(824, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(768, 4) as u32) == f32::from_bits(c.r(824, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(824, 4) as u32) <= f32::from_bits(c.r(768, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(768, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(824, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d186 vmrs apsr,fpscr
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
        // 0x6002d18a ite gt
        let mut t185d00 = 0u64;
        let mut t185e00 = 0u64;
        let mut t186000 = 0u64;
        t185d00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t185e00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t186000 = (((((t185d00 >> 0) & 0xffu64) != 0) & (((t185e00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
    }
    {
        // 0x6002d18c vmul.gt.f32 s0,s0,s17
        let mut t1e00 = 0u64;
        let mut t1f00 = 0u64;
        let mut t2100 = 0u64;
        let mut t2600 = 0u64;
        t1e00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t1f00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t2100 =
            (((((t1e00 >> 0) & 0xffu64) != 0) & (((t1f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        t2600 = ((((t2100 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2600 >> 0) & 0xffu64) != 0 {
            return 0x6002d190u32;
        }
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(836, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    0x6002d190
}
pub fn b_6002d190(c: &mut Machine) -> u32 {
    {
        // 0x6002d190 vldr.le.32 s0,[pc,#0x54]
        let mut t2200 = 0u64;
        let mut t2400 = 0u64;
        let mut t2700 = 0u64;
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610797456u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(84u64)) & 0xffffffffu64;
        t2200 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2400 = (((c.r(97, 1) != 0) | (((t2200 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        t2700 = ((((t2400 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x6002d194u32;
        }
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    0x6002d194
}
pub fn b_6002d194(c: &mut Machine) -> u32 {
    {
        // 0x6002d194 bl 0x60040c70
        let value = (1610797464u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002d198
}
pub fn b_6002d198(c: &mut Machine) -> u32 {
    {
        // 0x6002d198 vneg.f32 s0,s0
        let value = (-f32::from_bits(c.r(768, 4) as u32)).to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002d19c vstr.32 s0,[r10,#0x31c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(796u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002d1a0 bl 0x6004137c
        let value = (1610797476u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x6004137cu32;
    }
    0x6002d1a4
}
pub fn b_6002d1a4(c: &mut Machine) -> u32 {
    {
        // 0x6002d1a4 vmov.f32 s15,0x3f800000
        let value = 1065353216u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d1a8 vsub.f32 s0,s15,s0
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002d1ac bl 0x60041338
        let value = (1610797488u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041338u32;
    }
    0x6002d1b0
}
pub fn b_6002d1b0(c: &mut Machine) -> u32 {
    {
        // 0x6002d1b0 vldr.32 s10,[r10,#0x36c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(876u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x6002d1b4 vldr.32 s12,[r10,#0x370]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(880u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x6002d1b8 vldr.32 s14,[pc,#0x30]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610797496u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(48u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002d1bc vmul.f32 s12,s10,s12
        let value = (f32::from_bits(c.r(808, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x6002d1c0 vldr.32 s13,[pc,#0x2c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610797504u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(44u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002d1c4 vstr.32 s10,[r10,#0x368]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(872u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(808, 4));
    }
    {
        // 0x6002d1c8 vmul.f32 s9,s10,s13
        let value = (f32::from_bits(c.r(808, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(804, 4, value);
    }
    {
        // 0x6002d1cc vldr.32 s15,[pc,#0x24]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610797516u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(36u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002d1d0 vmul.f32 s11,s12,s14
        let value = (f32::from_bits(c.r(816, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x6002d1d4 b 0x6002d214
        return 0x6002d214u32;
    }
    0x6002d1d6
}
pub fn b_6002d214(c: &mut Machine) -> u32 {
    {
        // 0x6002d214 vldr.32 s2,[r11,#0x164]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(356u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(776, 4, value);
    }
    {
        // 0x6002d218 vldr.32 s6,[pc,#-0x18]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((1610797592u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(24u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x6002d21c vmul.f32 s8,s9,s13
        let value = (f32::from_bits(c.r(804, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(800, 4, value);
    }
    {
        // 0x6002d220 vstr.32 s9,[r10,#0x364]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(868u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(804, 4));
    }
    {
        // 0x6002d224 vmul.f32 s10,s11,s14
        let value = (f32::from_bits(c.r(812, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x6002d228 vldr.32 s7,[pc,#-0x24]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((1610797608u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(36u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x6002d22c vldr.32 s3,[r11,#0x214]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(532u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(780, 4, value);
    }
    {
        // 0x6002d230 vmul.f32 s1,s8,s13
        let value = (f32::from_bits(c.r(800, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(772, 4, value);
    }
    {
        // 0x6002d234 vstr.32 s8,[r10,#0x360]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(864u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(800, 4));
    }
    {
        // 0x6002d238 vmul.f32 s9,s10,s14
        let value = (f32::from_bits(c.r(808, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(804, 4, value);
    }
    {
        // 0x6002d23c vldr.32 s4,[r11,#0x2c4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(708u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(784, 4, value);
    }
    {
        // 0x6002d240 vfma.f32 s7,s0,s6
        let value = f32::from_bits(c.r(768, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(792, 4) as u32),
                f32::from_bits(c.r(796, 4) as u32),
            )
            .to_bits() as u64;
        c.w(796, 4, value);
    }
    {
        // 0x6002d244 vldr.32 s5,[r11,#0x374]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(884u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(788, 4, value);
    }
    {
        // 0x6002d248 vmul.f32 s6,s1,s13
        let value = (f32::from_bits(c.r(772, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(792, 4, value);
    }
    {
        // 0x6002d24c ldr r3,[sp,#0x10]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d24e vmul.f32 s8,s9,s14
        let value = (f32::from_bits(c.r(804, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(800, 4, value);
    }
    {
        // 0x6002d252 vstr.32 s1,[r10,#0x35c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(860u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(772, 4));
    }
    {
        // 0x6002d256 vstr.32 s12,[r10,#0x348]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(840u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x6002d25a vmul.f32 s1,s6,s13
        let value = (f32::from_bits(c.r(792, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(772, 4, value);
    }
    {
        // 0x6002d25e vstr.32 s6,[r10,#0x358]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(856u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(792, 4));
    }
    {
        // 0x6002d262 vmul.f32 s14,s8,s14
        let value = (f32::from_bits(c.r(800, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002d266 vldr.32 s6,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x6002d26a vstr.32 s7,[r10,#0x37c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(892u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(796, 4));
    }
    {
        // 0x6002d26e vmul.f32 s7,s1,s13
        let value = (f32::from_bits(c.r(772, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(796, 4, value);
    }
    {
        // 0x6002d272 ldr r3,[sp,#0x18]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(24u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d274 vcmpe.f32 s14,s15
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
        t35c00 = ((f32::from_bits(c.r(824, 4) as u32) < f32::from_bits(c.r(828, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(824, 4) as u32) == f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(828, 4) as u32) <= f32::from_bits(c.r(824, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(824, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d278 vstr.32 s11,[r10,#0x344]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(836u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(812, 4));
    }
    {
        // 0x6002d27c vstr.32 s10,[r10,#0x340]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(832u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(808, 4));
    }
    {
        // 0x6002d280 vmul.f32 s13,s7,s13
        let value = (f32::from_bits(c.r(796, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002d284 vstr.32 s7,[r10,#0x350]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(848u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(796, 4));
    }
    {
        // 0x6002d288 vmrs apsr,fpscr
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
        // 0x6002d28c vldr.32 s7,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x6002d290 ldr r3,[sp,#0xc]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(12u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d292 vstr.32 s13,[r10,#0x34c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(844u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x6002d296 vselgt.f32 s0,s14,s15
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(824, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(828, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(768, 4, value);
    }
    {
        // 0x6002d29a vcmpe.f32 s2,s0
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
        t35c00 = ((f32::from_bits(c.r(776, 4) as u32) < f32::from_bits(c.r(768, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(776, 4) as u32) == f32::from_bits(c.r(768, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(768, 4) as u32) <= f32::from_bits(c.r(776, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(776, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(768, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d29e vldr.32 s13,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002d2a2 ldr r3,[sp,#0x14]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(20u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d2a4 vmrs apsr,fpscr
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
        // 0x6002d2a8 vstr.32 s9,[r10,#0x33c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(828u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(804, 4));
    }
    {
        // 0x6002d2ac vcmpe.f32 s8,s15
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
        t35c00 = ((f32::from_bits(c.r(800, 4) as u32) < f32::from_bits(c.r(828, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(800, 4) as u32) == f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(828, 4) as u32) <= f32::from_bits(c.r(800, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(800, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d2b0 vstr.32 s8,[r10,#0x338]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(824u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(800, 4));
    }
    {
        // 0x6002d2b4 vstr.32 s14,[r10,#0x334]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(820u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002d2b8 vselgt.f32 s2,s0,s2
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(768, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(776, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(776, 4, value);
    }
    {
        // 0x6002d2bc vstr.32 s2,[r11,#0x168]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(360u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(776, 4));
    }
    {
        // 0x6002d2c0 vmrs apsr,fpscr
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
        // 0x6002d2c4 vstr.32 s1,[r10,#0x354]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(852u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(772, 4));
    }
    {
        // 0x6002d2c8 vldr.32 s1,[pc,#-0xc0]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((1610797768u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(192u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(772, 4, value);
    }
    {
        // 0x6002d2cc vselgt.f32 s2,s8,s15
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(800, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(828, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(776, 4, value);
    }
    {
        // 0x6002d2d0 vcmpe.f32 s3,s2
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
        t35c00 = ((f32::from_bits(c.r(780, 4) as u32) < f32::from_bits(c.r(776, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(780, 4) as u32) == f32::from_bits(c.r(776, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(776, 4) as u32) <= f32::from_bits(c.r(780, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(780, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(776, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d2d4 vmrs apsr,fpscr
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
        // 0x6002d2d8 vcmpe.f32 s9,s15
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
        t35c00 = ((f32::from_bits(c.r(804, 4) as u32) < f32::from_bits(c.r(828, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(804, 4) as u32) == f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(828, 4) as u32) <= f32::from_bits(c.r(804, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(804, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d2dc vselgt.f32 s3,s2,s3
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(776, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(780, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(780, 4, value);
    }
    {
        // 0x6002d2e0 vstr.32 s3,[r11,#0x218]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(536u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(780, 4));
    }
    {
        // 0x6002d2e4 vmrs apsr,fpscr
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
        // 0x6002d2e8 vmov.f32 s3,0x3f800000
        let value = 1065353216u64;
        c.w(780, 4, value);
    }
    {
        // 0x6002d2ec vselgt.f32 s2,s9,s15
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(804, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(828, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(776, 4, value);
    }
    {
        // 0x6002d2f0 vcmpe.f32 s4,s2
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
        t35c00 = ((f32::from_bits(c.r(784, 4) as u32) < f32::from_bits(c.r(776, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(784, 4) as u32) == f32::from_bits(c.r(776, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(776, 4) as u32) <= f32::from_bits(c.r(784, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(784, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(776, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d2f4 vmov.f32 s0,s3
        let value = c.r(780, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002d2f8 vmov.f32 s16,s3
        let value = c.r(780, 4);
        c.w(832, 4, value);
    }
    {
        // 0x6002d2fc vmrs apsr,fpscr
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
        // 0x6002d300 vcmpe.f32 s10,s15
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
        t35c00 = ((f32::from_bits(c.r(808, 4) as u32) < f32::from_bits(c.r(828, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(808, 4) as u32) == f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(828, 4) as u32) <= f32::from_bits(c.r(808, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(808, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d304 vselgt.f32 s4,s2,s4
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(776, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(784, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(784, 4, value);
    }
    {
        // 0x6002d308 vstr.32 s4,[r11,#0x2c8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(712u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(784, 4));
    }
    {
        // 0x6002d30c vmrs apsr,fpscr
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
        // 0x6002d310 vselgt.f32 s4,s10,s15
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(808, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(828, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(784, 4, value);
    }
    {
        // 0x6002d314 vcmpe.f32 s5,s4
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
        t35c00 = ((f32::from_bits(c.r(788, 4) as u32) < f32::from_bits(c.r(784, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(788, 4) as u32) == f32::from_bits(c.r(784, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(784, 4) as u32) <= f32::from_bits(c.r(788, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(788, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(784, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d318 vmrs apsr,fpscr
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
        // 0x6002d31c vcmpe.f32 s11,s15
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
        t35c00 = ((f32::from_bits(c.r(812, 4) as u32) < f32::from_bits(c.r(828, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(812, 4) as u32) == f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(828, 4) as u32) <= f32::from_bits(c.r(812, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(812, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d320 vselgt.f32 s5,s4,s5
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(784, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(788, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(788, 4, value);
    }
    {
        // 0x6002d324 vstr.32 s5,[r11,#0x378]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(888u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(788, 4));
    }
    {
        // 0x6002d328 vmrs apsr,fpscr
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
        // 0x6002d32c vldr.32 s4,[pc,#-0x120]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((1610797868u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(288u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(784, 4, value);
    }
    {
        // 0x6002d330 vselgt.f32 s5,s11,s15
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(812, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(828, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(788, 4, value);
    }
    {
        // 0x6002d334 vcmpe.f32 s6,s5
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
        t35c00 = ((f32::from_bits(c.r(792, 4) as u32) < f32::from_bits(c.r(788, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(792, 4) as u32) == f32::from_bits(c.r(788, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(788, 4) as u32) <= f32::from_bits(c.r(792, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(792, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(788, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d338 vmrs apsr,fpscr
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
        // 0x6002d33c vcmpe.f32 s12,s15
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
        t35c00 = ((f32::from_bits(c.r(816, 4) as u32) < f32::from_bits(c.r(828, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(816, 4) as u32) == f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(828, 4) as u32) <= f32::from_bits(c.r(816, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(816, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d340 vselgt.f32 s6,s5,s6
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(788, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(792, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(792, 4, value);
    }
    {
        // 0x6002d344 vstr.32 s6,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(792, 4));
    }
    {
        // 0x6002d348 vmrs apsr,fpscr
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
        // 0x6002d34c ldr r3,[sp,#0x1c]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d34e vstr.32 s4,[r10,#0x324]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(804u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(784, 4));
    }
    {
        // 0x6002d352 vstr.32 s4,[r10,#0x328]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(808u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(784, 4));
    }
    {
        // 0x6002d356 vselgt.f32 s15,s12,s15
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(816, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(828, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(828, 4, value);
    }
    {
        // 0x6002d35a vcmpe.f32 s7,s15
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
        t35c00 = ((f32::from_bits(c.r(796, 4) as u32) < f32::from_bits(c.r(828, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(796, 4) as u32) == f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(828, 4) as u32) <= f32::from_bits(c.r(796, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(796, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d35e vstr.32 s4,[r10,#0x1d0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(464u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(784, 4));
    }
    {
        // 0x6002d362 vmrs apsr,fpscr
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
        // 0x6002d366 vcmpe.f32 s14,s1
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
        t35c00 = ((f32::from_bits(c.r(824, 4) as u32) < f32::from_bits(c.r(772, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(824, 4) as u32) == f32::from_bits(c.r(772, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(772, 4) as u32) <= f32::from_bits(c.r(824, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(824, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(772, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d36a vselgt.f32 s7,s15,s7
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(828, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(796, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(796, 4, value);
    }
    {
        // 0x6002d36e vstr.32 s7,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(796, 4));
    }
    {
        // 0x6002d372 ldr.w r3,[r11,#0xc94]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3220u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d376 vmrs apsr,fpscr
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
        // 0x6002d37a vcmpe.f32 s8,s1
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
        t35c00 = ((f32::from_bits(c.r(800, 4) as u32) < f32::from_bits(c.r(772, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(800, 4) as u32) == f32::from_bits(c.r(772, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(772, 4) as u32) <= f32::from_bits(c.r(800, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(800, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(772, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d37e vmov s7,r3
        let value = c.r(44, 4);
        c.w(796, 4, value);
    }
    {
        // 0x6002d382 ldr r3,[sp,#0x20]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d384 vselgt.f32 s14,s14,s1
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(824, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(772, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(824, 4, value);
    }
    {
        // 0x6002d388 vmov.f32 s15,s3
        let value = c.r(780, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002d38c vcvt.f32.s32 s7,s7
        let mut t3b200 = 0u64;
        let mut t3b300 = 0u64;
        let mut t3b500 = 0u64;
        t3b200 = (if 22u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (22u64 as u32)
        }) & 0xffffffffu64;
        t3b300 = (((t3b200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        t3b500 = (((t3b300 >> 0) & 0xffu64) & 3u64) & 0xffu64;
        let value = c.userop(219, c.r(796, 4), ((t3b500 >> 0) & 0xffu64), 0u64, 4, 4);
        c.w(796, 4, value);
    }
    {
        // 0x6002d390 vmrs apsr,fpscr
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
        // 0x6002d394 vcmpe.f32 s9,s1
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
        t35c00 = ((f32::from_bits(c.r(804, 4) as u32) < f32::from_bits(c.r(772, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(804, 4) as u32) == f32::from_bits(c.r(772, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(772, 4) as u32) <= f32::from_bits(c.r(804, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(804, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(772, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d398 vdiv.f32 s6,s7,s14
        let value = (f32::from_bits(c.r(796, 4) as u32) / f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(792, 4, value);
    }
    {
        // 0x6002d39c vselgt.f32 s8,s8,s1
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(800, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(772, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(800, 4, value);
    }
    {
        // 0x6002d3a0 vmrs apsr,fpscr
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
        // 0x6002d3a4 vcmpe.f32 s10,s1
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
        t35c00 = ((f32::from_bits(c.r(808, 4) as u32) < f32::from_bits(c.r(772, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(808, 4) as u32) == f32::from_bits(c.r(772, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(772, 4) as u32) <= f32::from_bits(c.r(808, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(808, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(772, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d3a8 vselgt.f32 s9,s9,s1
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(804, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(772, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(804, 4, value);
    }
    {
        // 0x6002d3ac vmrs apsr,fpscr
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
        // 0x6002d3b0 vcmpe.f32 s11,s1
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
        t35c00 = ((f32::from_bits(c.r(812, 4) as u32) < f32::from_bits(c.r(772, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(812, 4) as u32) == f32::from_bits(c.r(772, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(772, 4) as u32) <= f32::from_bits(c.r(812, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(812, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(772, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d3b4 vselgt.f32 s10,s10,s1
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(808, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(772, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(808, 4, value);
    }
    {
        // 0x6002d3b8 vmrs apsr,fpscr
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
        // 0x6002d3bc vcmpe.f32 s12,s1
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
        t35c00 = ((f32::from_bits(c.r(816, 4) as u32) < f32::from_bits(c.r(772, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(816, 4) as u32) == f32::from_bits(c.r(772, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(772, 4) as u32) <= f32::from_bits(c.r(816, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(816, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(772, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d3c0 vmul.f32 s14,s6,s13
        let value = (f32::from_bits(c.r(792, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002d3c4 vstr.32 s6,[r9]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(68, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(792, 4));
    }
    {
        // 0x6002d3c8 vselgt.f32 s11,s11,s1
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(812, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(772, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(812, 4, value);
    }
    {
        // 0x6002d3cc vmrs apsr,fpscr
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
        // 0x6002d3d0 vdiv.f32 s13,s3,s14
        let value = (f32::from_bits(c.r(780, 4) as u32) / f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002d3d4 vstr.32 s14,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002d3d8 ldr r3,[0x6002d79c]
        let value = c.load(0x6002d79c, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d3da vselgt.f32 s12,s12,s1
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(816, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(772, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(816, 4, value);
    }
    {
        // 0x6002d3de vldr.32 s5,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(788, 4, value);
    }
    {
        // 0x6002d3e2 vldr.32 s6,[r3,#0x60]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(96u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x6002d3e6 vldr.32 s7,[r3,#0xc0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(192u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x6002d3ea ldr r3,[sp,#0x24]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d3ec vstr.32 s13,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x6002d3f0 ldr.w r3,[r11,#0xcf4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3316u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d3f4 vmov s14,r3
        let value = c.r(44, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002d3f8 ldr r3,[0x6002d7a0]
        let value = c.load(0x6002d7a0, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d3fa vcvt.f32.s32 s14,s14
        let mut t3b200 = 0u64;
        let mut t3b300 = 0u64;
        let mut t3b500 = 0u64;
        t3b200 = (if 22u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (22u64 as u32)
        }) & 0xffffffffu64;
        t3b300 = (((t3b200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        t3b500 = (((t3b300 >> 0) & 0xffu64) & 3u64) & 0xffu64;
        let value = c.userop(219, c.r(824, 4), ((t3b500 >> 0) & 0xffu64), 0u64, 4, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002d3fe vldr.32 s13,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002d402 vdiv.f32 s3,s14,s8
        let value = (f32::from_bits(c.r(824, 4) as u32) / f32::from_bits(c.r(800, 4) as u32))
            .to_bits() as u64;
        c.w(780, 4, value);
    }
    {
        // 0x6002d406 vldr.32 s14,[r3,#0x60]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(96u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002d40a ldr r3,[sp,#0x28]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d40c vmul.f32 s8,s3,s5
        let value = (f32::from_bits(c.r(780, 4) as u32) * f32::from_bits(c.r(788, 4) as u32))
            .to_bits() as u64;
        c.w(800, 4, value);
    }
    {
        // 0x6002d410 vstr.32 s3,[r8]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(64, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(780, 4));
    }
    {
        // 0x6002d414 vldr.32 s3,[r10,#0x1c8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(456u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(780, 4, value);
    }
    {
        // 0x6002d418 vdiv.f32 s5,s16,s8
        let value = (f32::from_bits(c.r(832, 4) as u32) / f32::from_bits(c.r(800, 4) as u32))
            .to_bits() as u64;
        c.w(788, 4, value);
    }
    {
        // 0x6002d41c vstr.32 s8,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(800, 4));
    }
    {
        // 0x6002d420 ldr r3,[sp,#0x2c]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d422 vldr.32 s8,[r10,#0x1cc]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(460u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x6002d426 vldr.32 s16,[pc,#0x37c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610798118u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(892u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(832, 4, value);
    }
    {
        // 0x6002d42a vmul.f32 s8,s8,s3
        let value = (f32::from_bits(c.r(800, 4) as u32) * f32::from_bits(c.r(780, 4) as u32))
            .to_bits() as u64;
        c.w(800, 4, value);
    }
    {
        // 0x6002d42e vcmpe.f32 s8,s16
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
        t35c00 = ((f32::from_bits(c.r(800, 4) as u32) < f32::from_bits(c.r(832, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(800, 4) as u32) == f32::from_bits(c.r(832, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(832, 4) as u32) <= f32::from_bits(c.r(800, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(800, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(832, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d432 vmrs apsr,fpscr
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
        // 0x6002d436 vstr.32 s5,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(788, 4));
    }
    {
        // 0x6002d43a ldr.w r3,[r11,#0xd54]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3412u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d43e vmov s5,r3
        let value = c.r(44, 4);
        c.w(788, 4, value);
    }
    {
        // 0x6002d442 ldr r3,[sp,#0x30]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d444 vcvt.f32.s32 s5,s5
        let mut t3b200 = 0u64;
        let mut t3b300 = 0u64;
        let mut t3b500 = 0u64;
        t3b200 = (if 22u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (22u64 as u32)
        }) & 0xffffffffu64;
        t3b300 = (((t3b200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        t3b500 = (((t3b300 >> 0) & 0xffu64) & 3u64) & 0xffu64;
        let value = c.userop(219, c.r(788, 4), ((t3b500 >> 0) & 0xffu64), 0u64, 4, 4);
        c.w(788, 4, value);
    }
    {
        // 0x6002d448 vdiv.f32 s3,s5,s9
        let value = (f32::from_bits(c.r(788, 4) as u32) / f32::from_bits(c.r(804, 4) as u32))
            .to_bits() as u64;
        c.w(780, 4, value);
    }
    {
        // 0x6002d44c vmul.f32 s9,s3,s6
        let value = (f32::from_bits(c.r(780, 4) as u32) * f32::from_bits(c.r(792, 4) as u32))
            .to_bits() as u64;
        c.w(804, 4, value);
    }
    {
        // 0x6002d450 vstr.32 s3,[r7]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(60, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(780, 4));
    }
    {
        // 0x6002d454 vdiv.f32 s6,s0,s9
        let value = (f32::from_bits(c.r(768, 4) as u32) / f32::from_bits(c.r(804, 4) as u32))
            .to_bits() as u64;
        c.w(792, 4, value);
    }
    {
        // 0x6002d458 vstr.32 s9,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(804, 4));
    }
    {
        // 0x6002d45c ldr r3,[sp,#0x34]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d45e vstr.32 s6,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(792, 4));
    }
    {
        // 0x6002d462 ldr.w r3,[r11,#0xdb4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3508u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d466 vmov s9,r3
        let value = c.r(44, 4);
        c.w(804, 4, value);
    }
    {
        // 0x6002d46a vcvt.f32.s32 s9,s9
        let mut t3b200 = 0u64;
        let mut t3b300 = 0u64;
        let mut t3b500 = 0u64;
        t3b200 = (if 22u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (22u64 as u32)
        }) & 0xffffffffu64;
        t3b300 = (((t3b200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        t3b500 = (((t3b300 >> 0) & 0xffu64) & 3u64) & 0xffu64;
        let value = c.userop(219, c.r(804, 4), ((t3b500 >> 0) & 0xffu64), 0u64, 4, 4);
        c.w(804, 4, value);
    }
    {
        // 0x6002d46e vdiv.f32 s6,s9,s10
        let value = (f32::from_bits(c.r(804, 4) as u32) / f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(792, 4, value);
    }
    {
        // 0x6002d472 vmul.f32 s10,s6,s7
        let value = (f32::from_bits(c.r(792, 4) as u32) * f32::from_bits(c.r(796, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x6002d476 vstr.32 s6,[r6]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(56, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(792, 4));
    }
    {
        // 0x6002d47a ldr r3,[sp,#0x38]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d47c vdiv.f32 s9,s0,s10
        let value = (f32::from_bits(c.r(768, 4) as u32) / f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(804, 4, value);
    }
    {
        // 0x6002d480 vstr.32 s10,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(808, 4));
    }
    {
        // 0x6002d484 ldr r3,[sp,#0x3c]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(60u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d486 vstr.32 s9,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(804, 4));
    }
    {
        // 0x6002d48a ldr.w r3,[r11,#0xe14]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3604u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d48e vmov s10,r3
        let value = c.r(44, 4);
        c.w(808, 4, value);
    }
    {
        // 0x6002d492 ldr r3,[sp,#0x40]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d494 vcvt.f32.s32 s10,s10
        let mut t3b200 = 0u64;
        let mut t3b300 = 0u64;
        let mut t3b500 = 0u64;
        t3b200 = (if 22u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (22u64 as u32)
        }) & 0xffffffffu64;
        t3b300 = (((t3b200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        t3b500 = (((t3b300 >> 0) & 0xffu64) & 3u64) & 0xffu64;
        let value = c.userop(219, c.r(808, 4), ((t3b500 >> 0) & 0xffu64), 0u64, 4, 4);
        c.w(808, 4, value);
    }
    {
        // 0x6002d498 vdiv.f32 s9,s10,s11
        let value = (f32::from_bits(c.r(808, 4) as u32) / f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(804, 4, value);
    }
    {
        // 0x6002d49c vmul.f32 s13,s9,s13
        let value = (f32::from_bits(c.r(804, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002d4a0 vstr.32 s9,[r5]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(52, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(804, 4));
    }
    {
        // 0x6002d4a4 vdiv.f32 s11,s0,s13
        let value = (f32::from_bits(c.r(768, 4) as u32) / f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x6002d4a8 vstr.32 s13,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x6002d4ac ldr r3,[sp,#0x44]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d4ae vstr.32 s11,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(812, 4));
    }
    {
        // 0x6002d4b2 ldr.w r3,[r11,#0xe74]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3700u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d4b6 vmov s13,r3
        let value = c.r(44, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002d4ba ldr r3,[sp,#0x48]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(72u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d4bc vcvt.f32.s32 s13,s13
        let mut t3b200 = 0u64;
        let mut t3b300 = 0u64;
        let mut t3b500 = 0u64;
        t3b200 = (if 22u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (22u64 as u32)
        }) & 0xffffffffu64;
        t3b300 = (((t3b200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        t3b500 = (((t3b300 >> 0) & 0xffu64) & 3u64) & 0xffu64;
        let value = c.userop(219, c.r(820, 4), ((t3b500 >> 0) & 0xffu64), 0u64, 4, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002d4c0 vdiv.f32 s11,s13,s12
        let value = (f32::from_bits(c.r(820, 4) as u32) / f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x6002d4c4 vmul.f32 s14,s11,s14
        let value = (f32::from_bits(c.r(812, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002d4c8 vstr.32 s11,[r4]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(48, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(812, 4));
    }
    {
        // 0x6002d4cc vdiv.f32 s13,s0,s14
        let value = (f32::from_bits(c.r(768, 4) as u32) / f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002d4d0 vstr.32 s14,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002d4d4 ldr r3,[sp,#0x4c]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(76u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d4d6 vstr.32 s13,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x6002d4da ble.w 0x6002db9a
        let mut t184000 = 0u64;
        let mut t184200 = 0u64;
        t184000 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t184200 = (((c.r(97, 1) != 0) | (((t184000 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t184200 >> 0) & 0xffu64) != 0 {
            return 0x6002db9au32;
        }
    }
    0x6002d4de
}
pub fn b_6002d4de(c: &mut Machine) -> u32 {
    {
        // 0x6002d4de vldr.32 s14,[pc,#0x2c8]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610798302u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(712u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002d4e2 vcmpe.f32 s8,s14
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
        t35c00 = ((f32::from_bits(c.r(800, 4) as u32) < f32::from_bits(c.r(824, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(800, 4) as u32) == f32::from_bits(c.r(824, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(824, 4) as u32) <= f32::from_bits(c.r(800, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(800, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(824, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d4e6 vmrs apsr,fpscr
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
        // 0x6002d4ea bmi.w 0x6002dbac
        let mut t182b00 = 0u64;
        t182b00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        if ((t182b00 >> 0) & 0xffu64) != 0 {
            return 0x6002dbacu32;
        }
    }
    0x6002d4ee
}
pub fn b_6002d4ee(c: &mut Machine) -> u32 {
    {
        // 0x6002d4ee vldr.32 s14,[pc,#0x2bc]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610798318u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(700u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002d4f2 vmov.f32 s13,s14
        let value = c.r(824, 4);
        c.w(820, 4, value);
    }
    0x6002d4f6
}
pub fn b_6002d4f6(c: &mut Machine) -> u32 {
    {
        // 0x6002d4f6 vldr.32 s0,[pc,#0x2b8]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610798326u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(696u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002d4fa vstr.32 s13,[r10,#0x1d4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(468u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x6002d4fe vstr.32 s15,[r10,#0x1d8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(472u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002d502 vstr.32 s14,[r10,#0x1dc]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(476u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002d506 bl 0x60040c70
        let value = (1610798346u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002d50a
}
pub fn b_6002d50a(c: &mut Machine) -> u32 {
    {
        // 0x6002d50a vldr.32 s15,[r10,#0x1e8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(488u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002d50e vmov.f32 s14,s0
        let value = c.r(768, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002d512 vldr.32 s12,[r10,#0x1f4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(500u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x6002d516 vmul.f32 s15,s0,s15
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d51a vldr.32 s13,[pc,#0x298]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610798362u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(664u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002d51e vstr.32 s0,[r10,#0x1ec]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(492u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002d522 vmul.f32 s15,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d526 vcmpe.f32 s15,s13
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
        t35c00 = ((f32::from_bits(c.r(828, 4) as u32) < f32::from_bits(c.r(820, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(828, 4) as u32) == f32::from_bits(c.r(820, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(820, 4) as u32) <= f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(820, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d52a vmrs apsr,fpscr
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
        // 0x6002d52e bpl.w 0x6002db94
        let mut t182d00 = 0u64;
        t182d00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        if ((t182d00 >> 0) & 0xffu64) != 0 {
            return 0x6002db94u32;
        }
    }
    0x6002d532
}
pub fn b_6002d532(c: &mut Machine) -> u32 {
    {
        // 0x6002d532 vmul.f32 s13,s15,s15
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002d536 vldr.32 s9,[pc,#0x280]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610798390u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(640u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(804, 4, value);
    }
    {
        // 0x6002d53a vldr.32 s10,[pc,#0x280]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610798394u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(640u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x6002d53e vmov.f32 s12,0x3f800000
        let value = 1065353216u64;
        c.w(816, 4, value);
    }
    {
        // 0x6002d542 vmul.f32 s11,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x6002d546 vfma.f32 s15,s11,s9
        let value = f32::from_bits(c.r(812, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d54a vmul.f32 s13,s13,s11
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002d54e vfma.f32 s15,s13,s10
        let value = f32::from_bits(c.r(820, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d552 vadd.f32 s12,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x6002d556 vdiv.f32 s13,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) / f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    0x6002d55a
}
pub fn b_6002d55a(c: &mut Machine) -> u32 {
    {
        // 0x6002d55a vldr.32 s0,[pc,#0x254]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610798426u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(596u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002d55e vstr.32 s13,[r10,#0x1f8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(504u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x6002d562 vmov.f32 s15,0xbf800000
        let value = 3212836864u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d566 vfma.f32 s15,s14,s14
        let value = f32::from_bits(c.r(824, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(824, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d56a vstr.32 s15,[r10,#0x1f0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(496u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002d56e bl 0x60040c70
        let value = (1610798450u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002d572
}
pub fn b_6002d572(c: &mut Machine) -> u32 {
    {
        // 0x6002d572 vldr.32 s15,[r10,#0x200]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(512u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002d576 vmov.f32 s14,s0
        let value = c.r(768, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002d57a vldr.32 s12,[r10,#0x20c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(524u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x6002d57e vmul.f32 s15,s0,s15
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d582 vldr.32 s13,[pc,#0x230]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610798466u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(560u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002d586 vstr.32 s0,[r10,#0x204]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(516u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002d58a vmul.f32 s15,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d58e vcmpe.f32 s15,s13
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
        t35c00 = ((f32::from_bits(c.r(828, 4) as u32) < f32::from_bits(c.r(820, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(828, 4) as u32) == f32::from_bits(c.r(820, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(820, 4) as u32) <= f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(820, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d592 vmrs apsr,fpscr
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
        // 0x6002d596 bpl.w 0x6002db8e
        let mut t182d00 = 0u64;
        t182d00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        if ((t182d00 >> 0) & 0xffu64) != 0 {
            return 0x6002db8eu32;
        }
    }
    0x6002d59a
}
pub fn b_6002d59a(c: &mut Machine) -> u32 {
    {
        // 0x6002d59a vmul.f32 s13,s15,s15
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002d59e vldr.32 s9,[pc,#0x218]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610798494u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(536u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(804, 4, value);
    }
    {
        // 0x6002d5a2 vldr.32 s10,[pc,#0x218]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610798498u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(536u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x6002d5a6 vmov.f32 s12,0x3f800000
        let value = 1065353216u64;
        c.w(816, 4, value);
    }
    {
        // 0x6002d5aa vmul.f32 s11,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x6002d5ae vfma.f32 s15,s11,s9
        let value = f32::from_bits(c.r(812, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d5b2 vmul.f32 s13,s13,s11
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002d5b6 vfma.f32 s15,s13,s10
        let value = f32::from_bits(c.r(820, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d5ba vadd.f32 s12,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x6002d5be vdiv.f32 s13,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) / f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    0x6002d5c2
}
pub fn b_6002d5c2(c: &mut Machine) -> u32 {
    {
        // 0x6002d5c2 vldr.32 s16,[r10,#0x36c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(876u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(832, 4, value);
    }
    {
        // 0x6002d5c6 mov.w r1,#0x3f000000
        let mut t188100 = 0u64;
        let mut t188300 = 0u64;
        t188100 = (1056964608u64) & 0xffffffffu64;
        t188300 = (if 31u64 >= 32 {
            0
        } else {
            ((t188100 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188300 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((t188100 >> 0) & 0xffffffffu64);
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002d5ca vldr.32 s0,[pc,#0x1f4]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610798538u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(500u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002d5ce vstr.32 s13,[r10,#0x210]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(528u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x6002d5d2 str.w r1,[r10,#0x374]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(884u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002d5d6 vmov.f32 s15,0xbf800000
        let value = 3212836864u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d5da vfma.f32 s15,s14,s14
        let value = f32::from_bits(c.r(824, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(824, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d5de vstr.32 s15,[r10,#0x208]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(520u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002d5e2 bl 0x60040c70
        let value = (1610798566u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002d5e6
}
pub fn b_6002d5e6(c: &mut Machine) -> u32 {
    {
        // 0x6002d5e6 vdiv.f32 s15,s16,s0
        let value = (f32::from_bits(c.r(832, 4) as u32) / f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d5ea vldr.32 s13,[pc,#0x1d8]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610798570u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(472u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002d5ee vldr.32 s14,[pc,#0x1d8]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610798574u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(472u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002d5f2 vmul.f32 s0,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002d5f6 vcmpe.f32 s0,s14
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
        t35c00 = ((f32::from_bits(c.r(768, 4) as u32) < f32::from_bits(c.r(824, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(768, 4) as u32) == f32::from_bits(c.r(824, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(824, 4) as u32) <= f32::from_bits(c.r(768, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(768, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(824, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d5fa vmrs apsr,fpscr
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
        // 0x6002d5fe ble.w 0x6002db62
        let mut t184000 = 0u64;
        let mut t184200 = 0u64;
        t184000 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t184200 = (((c.r(97, 1) != 0) | (((t184000 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t184200 >> 0) & 0xffu64) != 0 {
            return 0x6002db62u32;
        }
    }
    0x6002d602
}
pub fn b_6002d602(c: &mut Machine) -> u32 {
    {
        // 0x6002d602 vldr.32 s15,[pc,#0x1c8]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610798594u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(456u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002d606 vmul.f32 s0,s0,s15
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    0x6002d60a
}
pub fn b_6002d60a(c: &mut Machine) -> u32 {
    {
        // 0x6002d60a bl 0x60040c70
        let value = (1610798606u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002d60e
}
pub fn b_6002d60e(c: &mut Machine) -> u32 {
    {
        // 0x6002d60e vneg.f32 s0,s0
        let value = (-f32::from_bits(c.r(768, 4) as u32)).to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002d612 vmov.f32 s20,0x3f800000
        let value = 1065353216u64;
        c.w(848, 4, value);
    }
    {
        // 0x6002d616 vldr.32 s17,[pc,#0x198]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610798614u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(408u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(836, 4, value);
    }
    {
        // 0x6002d61a vstr.32 s0,[r10,#0x31c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(796u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002d61e bl 0x6004137c
        let value = (1610798626u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x6004137cu32;
    }
    0x6002d622
}
pub fn b_6002d622(c: &mut Machine) -> u32 {
    {
        // 0x6002d622 vmov.f32 s15,0x3f800000
        let value = 1065353216u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d626 vsub.f32 s0,s15,s0
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002d62a bl 0x60041338
        let value = (1610798638u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041338u32;
    }
    0x6002d62e
}
pub fn b_6002d62e(c: &mut Machine) -> u32 {
    {
        // 0x6002d62e vldr.32 s15,[r10,#0x36c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(876u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002d632 vldr.32 s12,[r10,#0x370]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(880u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x6002d636 vldr.32 s13,[pc,#0x198]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610798646u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(408u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002d63a vmul.f32 s12,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x6002d63e vldr.32 s14,[pc,#0x194]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610798654u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(404u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002d642 vstr.32 s15,[r10,#0x368]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(872u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002d646 vmul.f32 s15,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d64a vldr.32 s10,[pc,#0x18c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610798666u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(396u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x6002d64e vmul.f32 s11,s12,s14
        let value = (f32::from_bits(c.r(816, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x6002d652 vldr.32 s8,[pc,#0x188]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610798674u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(392u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x6002d656 vmul.f32 s5,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(788, 4, value);
    }
    {
        // 0x6002d65a vstr.32 s15,[r10,#0x364]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(868u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002d65e vfma.f32 s8,s0,s10
        let value = f32::from_bits(c.r(768, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(800, 4) as u32),
            )
            .to_bits() as u64;
        c.w(800, 4, value);
    }
    {
        // 0x6002d662 vldr.32 s3,[r11,#0x164]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(356u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(780, 4, value);
    }
    {
        // 0x6002d666 vmul.f32 s10,s11,s14
        let value = (f32::from_bits(c.r(812, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x6002d66a vldr.32 s4,[r11,#0x214]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(532u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(784, 4, value);
    }
    {
        // 0x6002d66e vmul.f32 s15,s5,s13
        let value = (f32::from_bits(c.r(788, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d672 vstr.32 s5,[r10,#0x360]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(864u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(788, 4));
    }
    {
        // 0x6002d676 ldr r3,[sp,#0x10]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d678 vmul.f32 s9,s10,s14
        let value = (f32::from_bits(c.r(808, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(804, 4, value);
    }
    {
        // 0x6002d67c vstr.32 s12,[r10,#0x348]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(840u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x6002d680 vmul.f32 s0,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002d684 vstr.32 s15,[r10,#0x35c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(860u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002d688 vstr.32 s8,[r10,#0x37c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(892u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(800, 4));
    }
    {
        // 0x6002d68c vmul.f32 s8,s9,s14
        let value = (f32::from_bits(c.r(804, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(800, 4, value);
    }
    {
        // 0x6002d690 vldr.32 s15,[pc,#0x14c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610798736u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(332u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002d694 vmul.f32 s5,s0,s13
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(788, 4, value);
    }
    {
        // 0x6002d698 vstr.32 s0,[r10,#0x358]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(856u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002d69c vldr.32 s6,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x6002d6a0 vmul.f32 s14,s8,s14
        let value = (f32::from_bits(c.r(800, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002d6a4 ldr r3,[sp,#0x18]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(24u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d6a6 vmul.f32 s0,s5,s13
        let value = (f32::from_bits(c.r(788, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002d6aa vstr.32 s5,[r10,#0x354]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(852u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(788, 4));
    }
    {
        // 0x6002d6ae vldr.32 s5,[r11,#0x2c4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(708u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(788, 4, value);
    }
    {
        // 0x6002d6b2 vcmpe.f32 s14,s15
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
        t35c00 = ((f32::from_bits(c.r(824, 4) as u32) < f32::from_bits(c.r(828, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(824, 4) as u32) == f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(828, 4) as u32) <= f32::from_bits(c.r(824, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(824, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d6b6 vldr.32 s7,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x6002d6ba vmul.f32 s13,s0,s13
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002d6be ldr r3,[sp,#0xc]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(12u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d6c0 vstr.32 s11,[r10,#0x344]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(836u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(812, 4));
    }
    {
        // 0x6002d6c4 vmrs apsr,fpscr
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
        // 0x6002d6c8 vldr.32 s2,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(776, 4, value);
    }
    {
        // 0x6002d6cc vstr.32 s13,[r10,#0x34c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(844u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x6002d6d0 ldr r3,[sp,#0x14]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(20u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d6d2 vselgt.f32 s13,s14,s15
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(824, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(828, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(820, 4, value);
    }
    {
        // 0x6002d6d6 vcmpe.f32 s3,s13
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
        t35c00 = ((f32::from_bits(c.r(780, 4) as u32) < f32::from_bits(c.r(820, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(780, 4) as u32) == f32::from_bits(c.r(820, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(820, 4) as u32) <= f32::from_bits(c.r(780, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(780, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(820, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d6da vstr.32 s10,[r10,#0x340]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(832u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(808, 4));
    }
    {
        // 0x6002d6de vstr.32 s9,[r10,#0x33c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(828u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(804, 4));
    }
    {
        // 0x6002d6e2 vmrs apsr,fpscr
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
        // 0x6002d6e6 vstr.32 s8,[r10,#0x338]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(824u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(800, 4));
    }
    {
        // 0x6002d6ea vcmpe.f32 s8,s15
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
        t35c00 = ((f32::from_bits(c.r(800, 4) as u32) < f32::from_bits(c.r(828, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(800, 4) as u32) == f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(828, 4) as u32) <= f32::from_bits(c.r(800, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(800, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d6ee vstr.32 s14,[r10,#0x334]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(820u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002d6f2 vstr.32 s0,[r10,#0x350]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(848u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002d6f6 vmov.f32 s0,s17
        let value = c.r(836, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002d6fa vselgt.f32 s3,s13,s3
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(820, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(780, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(780, 4, value);
    }
    {
        // 0x6002d6fe vstr.32 s3,[r11,#0x168]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(360u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(780, 4));
    }
    {
        // 0x6002d702 vmrs apsr,fpscr
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
        // 0x6002d706 vldr.32 s13,[r11,#0x374]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(884u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002d70a vldr.32 s1,[pc,#0xd8]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610798858u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(216u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(772, 4, value);
    }
    {
        // 0x6002d70e ldr r0,[0x6002d7e8]
        let value = c.load(0x6002d7e8, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002d710 vselgt.f32 s3,s8,s15
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(800, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(828, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(780, 4, value);
    }
    {
        // 0x6002d714 vcmpe.f32 s4,s3
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
        t35c00 = ((f32::from_bits(c.r(784, 4) as u32) < f32::from_bits(c.r(780, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(784, 4) as u32) == f32::from_bits(c.r(780, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(780, 4) as u32) <= f32::from_bits(c.r(784, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(784, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(780, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d718 vmrs apsr,fpscr
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
        // 0x6002d71c vcmpe.f32 s9,s15
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
        t35c00 = ((f32::from_bits(c.r(804, 4) as u32) < f32::from_bits(c.r(828, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(804, 4) as u32) == f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(828, 4) as u32) <= f32::from_bits(c.r(804, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(804, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d720 vselgt.f32 s4,s3,s4
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(780, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(784, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(784, 4, value);
    }
    {
        // 0x6002d724 vstr.32 s4,[r11,#0x218]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(536u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(784, 4));
    }
    {
        // 0x6002d728 vmrs apsr,fpscr
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
        // 0x6002d72c vmov.f32 s3,0x3e800000
        let value = 1048576000u64;
        c.w(780, 4, value);
    }
    {
        // 0x6002d730 vselgt.f32 s4,s9,s15
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(804, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(828, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(784, 4, value);
    }
    {
        // 0x6002d734 vcmpe.f32 s5,s4
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
        t35c00 = ((f32::from_bits(c.r(788, 4) as u32) < f32::from_bits(c.r(784, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(788, 4) as u32) == f32::from_bits(c.r(784, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(784, 4) as u32) <= f32::from_bits(c.r(788, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(788, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(784, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d738 vmrs apsr,fpscr
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
        // 0x6002d73c vcmpe.f32 s10,s15
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
        t35c00 = ((f32::from_bits(c.r(808, 4) as u32) < f32::from_bits(c.r(828, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(808, 4) as u32) == f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(828, 4) as u32) <= f32::from_bits(c.r(808, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(808, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d740 vselgt.f32 s5,s4,s5
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(784, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(788, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(788, 4, value);
    }
    {
        // 0x6002d744 vstr.32 s5,[r11,#0x2c8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(712u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(788, 4));
    }
    {
        // 0x6002d748 vmrs apsr,fpscr
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
        // 0x6002d74c vselgt.f32 s5,s10,s15
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(808, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(828, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(788, 4, value);
    }
    {
        // 0x6002d750 vcmpe.f32 s13,s5
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
        t35c00 = ((f32::from_bits(c.r(820, 4) as u32) < f32::from_bits(c.r(788, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(820, 4) as u32) == f32::from_bits(c.r(788, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(788, 4) as u32) <= f32::from_bits(c.r(820, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(820, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(788, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d754 vmrs apsr,fpscr
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
        // 0x6002d758 vcmpe.f32 s11,s15
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
        t35c00 = ((f32::from_bits(c.r(812, 4) as u32) < f32::from_bits(c.r(828, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(812, 4) as u32) == f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(828, 4) as u32) <= f32::from_bits(c.r(812, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(812, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d75c vselgt.f32 s13,s5,s13
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(788, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(820, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(820, 4, value);
    }
    {
        // 0x6002d760 vstr.32 s13,[r11,#0x378]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(888u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x6002d764 vmrs apsr,fpscr
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
        // 0x6002d768 vselgt.f32 s13,s11,s15
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(812, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(828, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(820, 4, value);
    }
    {
        // 0x6002d76c vcmpe.f32 s6,s13
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
        t35c00 = ((f32::from_bits(c.r(792, 4) as u32) < f32::from_bits(c.r(820, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(792, 4) as u32) == f32::from_bits(c.r(820, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(820, 4) as u32) <= f32::from_bits(c.r(792, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(792, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(820, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d770 vmrs apsr,fpscr
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
        // 0x6002d774 vcmpe.f32 s12,s15
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
        t35c00 = ((f32::from_bits(c.r(816, 4) as u32) < f32::from_bits(c.r(828, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(816, 4) as u32) == f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(828, 4) as u32) <= f32::from_bits(c.r(816, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(816, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d778 vselgt.f32 s6,s13,s6
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(820, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(792, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(792, 4, value);
    }
    {
        // 0x6002d77c vstr.32 s6,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(792, 4));
    }
    {
        // 0x6002d780 vmrs apsr,fpscr
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
        // 0x6002d784 ldr r3,[sp,#0x1c]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d786 vstr.32 s17,[r10,#0x29c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(668u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(836, 4));
    }
    {
        // 0x6002d78a vstr.32 s20,[r10,#0x2a0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(672u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(848, 4));
    }
    {
        // 0x6002d78e vselgt.f32 s15,s12,s15
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(816, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(828, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(828, 4, value);
    }
    {
        // 0x6002d792 vcmpe.f32 s7,s15
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
        t35c00 = ((f32::from_bits(c.r(796, 4) as u32) < f32::from_bits(c.r(828, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(796, 4) as u32) == f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(828, 4) as u32) <= f32::from_bits(c.r(796, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(796, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d796 vstr.32 s17,[r10,#0x2a4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(676u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(836, 4));
    }
    {
        // 0x6002d79a b 0x6002d7ec
        return 0x6002d7ecu32;
    }
    0x6002d79c
}
pub fn b_6002d7ec(c: &mut Machine) -> u32 {
    {
        // 0x6002d7ec vmrs apsr,fpscr
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
        // 0x6002d7f0 vcmpe.f32 s14,s1
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
        t35c00 = ((f32::from_bits(c.r(824, 4) as u32) < f32::from_bits(c.r(772, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(824, 4) as u32) == f32::from_bits(c.r(772, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(772, 4) as u32) <= f32::from_bits(c.r(824, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(824, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(772, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d7f4 vselgt.f32 s7,s15,s7
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(828, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(796, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(796, 4, value);
    }
    {
        // 0x6002d7f8 vstr.32 s7,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(796, 4));
    }
    {
        // 0x6002d7fc ldr.w r3,[r11,#0xc94]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3220u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d800 vmrs apsr,fpscr
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
        // 0x6002d804 vcmpe.f32 s8,s1
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
        t35c00 = ((f32::from_bits(c.r(800, 4) as u32) < f32::from_bits(c.r(772, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(800, 4) as u32) == f32::from_bits(c.r(772, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(772, 4) as u32) <= f32::from_bits(c.r(800, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(800, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(772, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d808 vmov s15,r3
        let value = c.r(44, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002d80c ldr r3,[sp,#0x20]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d80e vselgt.f32 s14,s14,s1
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(824, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(772, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(824, 4, value);
    }
    {
        // 0x6002d812 vcvt.f32.s32 s13,s15
        let mut t3b200 = 0u64;
        let mut t3b300 = 0u64;
        let mut t3b500 = 0u64;
        t3b200 = (if 22u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (22u64 as u32)
        }) & 0xffffffffu64;
        t3b300 = (((t3b200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        t3b500 = (((t3b300 >> 0) & 0xffu64) & 3u64) & 0xffu64;
        let value = c.userop(219, c.r(828, 4), ((t3b500 >> 0) & 0xffu64), 0u64, 4, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002d816 vmrs apsr,fpscr
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
        // 0x6002d81a vcmpe.f32 s9,s1
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
        t35c00 = ((f32::from_bits(c.r(804, 4) as u32) < f32::from_bits(c.r(772, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(804, 4) as u32) == f32::from_bits(c.r(772, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(772, 4) as u32) <= f32::from_bits(c.r(804, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(804, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(772, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d81e vdiv.f32 s15,s13,s14
        let value = (f32::from_bits(c.r(820, 4) as u32) / f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d822 vselgt.f32 s8,s8,s1
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(800, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(772, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(800, 4, value);
    }
    {
        // 0x6002d826 vmrs apsr,fpscr
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
        // 0x6002d82a vcmpe.f32 s10,s1
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
        t35c00 = ((f32::from_bits(c.r(808, 4) as u32) < f32::from_bits(c.r(772, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(808, 4) as u32) == f32::from_bits(c.r(772, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(772, 4) as u32) <= f32::from_bits(c.r(808, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(808, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(772, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d82e vselgt.f32 s9,s9,s1
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(804, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(772, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(804, 4, value);
    }
    {
        // 0x6002d832 vmrs apsr,fpscr
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
        // 0x6002d836 vcmpe.f32 s11,s1
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
        t35c00 = ((f32::from_bits(c.r(812, 4) as u32) < f32::from_bits(c.r(772, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(812, 4) as u32) == f32::from_bits(c.r(772, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(772, 4) as u32) <= f32::from_bits(c.r(812, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(812, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(772, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d83a vselgt.f32 s10,s10,s1
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(808, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(772, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(808, 4, value);
    }
    {
        // 0x6002d83e vmrs apsr,fpscr
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
        // 0x6002d842 vcmpe.f32 s12,s1
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
        t35c00 = ((f32::from_bits(c.r(816, 4) as u32) < f32::from_bits(c.r(772, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(816, 4) as u32) == f32::from_bits(c.r(772, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(772, 4) as u32) <= f32::from_bits(c.r(816, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(816, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(772, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d846 vstr.32 s15,[r9]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(68, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002d84a vmul.f32 s15,s15,s2
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(776, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d84e vselgt.f32 s11,s11,s1
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(812, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(772, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(812, 4, value);
    }
    {
        // 0x6002d852 vmrs apsr,fpscr
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
        // 0x6002d856 vdiv.f32 s14,s20,s15
        let value = (f32::from_bits(c.r(848, 4) as u32) / f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002d85a vstr.32 s15,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002d85e ldr r3,[0x6002dbec]
        let value = c.load(0x6002dbec, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d860 vselgt.f32 s12,s12,s1
        let mut tdb900 = 0u64;
        let mut tdba00 = 0u64;
        let mut tdbc00 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb900 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        tdba00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        tdbc00 = (((((tdb900 >> 0) & 0xffu64) != 0) & (((tdba00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        tdc500 = ((((tdbc00 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(816, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(772, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(816, 4, value);
    }
    {
        // 0x6002d864 vldr.32 s5,[r3,#0x120]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(288u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(788, 4, value);
    }
    {
        // 0x6002d868 vldr.32 s1,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(772, 4, value);
    }
    {
        // 0x6002d86c vldr.32 s6,[r3,#0x180]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(384u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x6002d870 vldr.32 s2,[r3,#0x60]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(96u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(776, 4, value);
    }
    {
        // 0x6002d874 vldr.32 s13,[r3,#0xc0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(192u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002d878 ldr r3,[sp,#0x24]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d87a vstr.32 s14,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002d87e ldr.w r3,[r11,#0xcf4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3316u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d882 vmov s15,r3
        let value = c.r(44, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002d886 ldr r3,[sp,#0x28]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d888 vcvt.f32.s32 s14,s15
        let mut t3b200 = 0u64;
        let mut t3b300 = 0u64;
        let mut t3b500 = 0u64;
        t3b200 = (if 22u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (22u64 as u32)
        }) & 0xffffffffu64;
        t3b300 = (((t3b200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        t3b500 = (((t3b300 >> 0) & 0xffu64) & 3u64) & 0xffu64;
        let value = c.userop(219, c.r(828, 4), ((t3b500 >> 0) & 0xffu64), 0u64, 4, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002d88c vdiv.f32 s15,s14,s8
        let value = (f32::from_bits(c.r(824, 4) as u32) / f32::from_bits(c.r(800, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d890 vstr.32 s15,[r8]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(64, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002d894 vmul.f32 s15,s15,s1
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(772, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d898 vdiv.f32 s14,s20,s15
        let value = (f32::from_bits(c.r(848, 4) as u32) / f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002d89c vstr.32 s15,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002d8a0 ldr r3,[sp,#0x2c]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d8a2 vstr.32 s14,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002d8a6 ldr.w r3,[r11,#0xd54]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3412u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d8aa vmov s15,r3
        let value = c.r(44, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002d8ae ldr r3,[sp,#0x30]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d8b0 vcvt.f32.s32 s14,s15
        let mut t3b200 = 0u64;
        let mut t3b300 = 0u64;
        let mut t3b500 = 0u64;
        t3b200 = (if 22u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (22u64 as u32)
        }) & 0xffffffffu64;
        t3b300 = (((t3b200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        t3b500 = (((t3b300 >> 0) & 0xffu64) & 3u64) & 0xffu64;
        let value = c.userop(219, c.r(828, 4), ((t3b500 >> 0) & 0xffu64), 0u64, 4, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002d8b4 vdiv.f32 s15,s14,s9
        let value = (f32::from_bits(c.r(824, 4) as u32) / f32::from_bits(c.r(804, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d8b8 vstr.32 s15,[r7]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(60, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002d8bc vmul.f32 s15,s15,s2
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(776, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d8c0 vdiv.f32 s14,s20,s15
        let value = (f32::from_bits(c.r(848, 4) as u32) / f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002d8c4 vstr.32 s15,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002d8c8 vldr.32 s15,[r10,#0x258]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(600u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002d8cc ldr r3,[sp,#0x34]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d8ce vmul.f32 s15,s15,s3
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(780, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d8d2 vstr.32 s15,[r10,#0x25c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(604u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002d8d6 vstr.32 s14,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002d8da ldr.w r3,[r11,#0xdb4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3508u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d8de vmov s15,r3
        let value = c.r(44, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002d8e2 vcvt.f32.s32 s14,s15
        let mut t3b200 = 0u64;
        let mut t3b300 = 0u64;
        let mut t3b500 = 0u64;
        t3b200 = (if 22u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (22u64 as u32)
        }) & 0xffffffffu64;
        t3b300 = (((t3b200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        t3b500 = (((t3b300 >> 0) & 0xffu64) & 3u64) & 0xffu64;
        let value = c.userop(219, c.r(828, 4), ((t3b500 >> 0) & 0xffu64), 0u64, 4, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002d8e6 vdiv.f32 s15,s14,s10
        let value = (f32::from_bits(c.r(824, 4) as u32) / f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d8ea vstr.32 s15,[r6]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(56, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002d8ee vmul.f32 s15,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d8f2 ldr r3,[sp,#0x38]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d8f4 vstr.32 s20,[r10,#0x2a8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(680u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(848, 4));
    }
    {
        // 0x6002d8f8 vdiv.f32 s14,s20,s15
        let value = (f32::from_bits(c.r(848, 4) as u32) / f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002d8fc vstr.32 s15,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002d900 ldr r3,[sp,#0x3c]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(60u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d902 vstr.32 s14,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002d906 ldr.w r3,[r11,#0xe14]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3604u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d90a vmov s15,r3
        let value = c.r(44, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002d90e addw r3,r11,#0xe74
        let value = ((c.r(76, 4) as u128 + 3700u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(76, 4), 3700u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(76, 4)).wrapping_add(3700u64);
        c.w(44, 4, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002d912 vcvt.f32.s32 s15,s15
        let mut t3b200 = 0u64;
        let mut t3b300 = 0u64;
        let mut t3b500 = 0u64;
        t3b200 = (if 22u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (22u64 as u32)
        }) & 0xffffffffu64;
        t3b300 = (((t3b200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        t3b500 = (((t3b300 >> 0) & 0xffu64) & 3u64) & 0xffu64;
        let value = c.userop(219, c.r(828, 4), ((t3b500 >> 0) & 0xffu64), 0u64, 4, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002d916 vldr.32 s13,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002d91a ldr r3,[sp,#0x40]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d91c vcvt.f32.s32 s13,s13
        let mut t3b200 = 0u64;
        let mut t3b300 = 0u64;
        let mut t3b500 = 0u64;
        t3b200 = (if 22u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (22u64 as u32)
        }) & 0xffffffffu64;
        t3b300 = (((t3b200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        t3b500 = (((t3b300 >> 0) & 0xffu64) & 3u64) & 0xffu64;
        let value = c.userop(219, c.r(820, 4), ((t3b500 >> 0) & 0xffu64), 0u64, 4, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002d920 vdiv.f32 s14,s15,s11
        let value = (f32::from_bits(c.r(828, 4) as u32) / f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002d924 vdiv.f32 s15,s13,s12
        let value = (f32::from_bits(c.r(820, 4) as u32) / f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d928 vstr.32 s14,[r5]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(52, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002d92c vmul.f32 s14,s14,s5
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(788, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002d930 vdiv.f32 s13,s20,s14
        let value = (f32::from_bits(c.r(848, 4) as u32) / f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002d934 vstr.32 s14,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002d938 ldr r3,[sp,#0x48]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(72u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d93a vstr.32 s15,[r4]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(48, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002d93e vmul.f32 s15,s15,s6
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(792, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d942 vdiv.f32 s14,s20,s15
        let value = (f32::from_bits(c.r(848, 4) as u32) / f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002d946 vstr.32 s15,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002d94a ldr r3,[sp,#0x44]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d94c vstr.32 s13,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x6002d950 ldr r3,[sp,#0x4c]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(76u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d952 vstr.32 s14,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002d956 ldr r3,[0x6002dbf0]
        let value = c.load(0x6002dbf0, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d958 str.w r3,[r10,#0x260]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(608u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002d95c bl 0x60028f18
        let value = (1610799456u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60028f18u32;
    }
    0x6002d960
}
pub fn b_6002d960(c: &mut Machine) -> u32 {
    {
        // 0x6002d960 vmov.f32 s0,s20
        let value = c.r(848, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002d964 vstr.32 s17,[r10,#0x320]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(800u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(836, 4));
    }
    {
        // 0x6002d968 bl 0x6004137c
        let value = (1610799468u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x6004137cu32;
    }
    0x6002d96c
}
pub fn b_6002d96c(c: &mut Machine) -> u32 {
    {
        // 0x6002d96c vmov.f32 s15,0x41d80000
        let value = 1104674816u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d970 vldr.32 s14,[r10,#0x2d4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(724u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002d974 vmov.f32 s13,0x3f000000
        let value = 1056964608u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002d978 vsub.f32 s0,s20,s0
        let value = (f32::from_bits(c.r(848, 4) as u32) - f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002d97c vldr.32 s12,[pc,#0x274]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610799484u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(628u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x6002d980 vadd.f32 s15,s14,s15
        let value = (f32::from_bits(c.r(824, 4) as u32) + f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d984 ldr r2,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x6002d986 ldr r3,[0x6002dbf8]
        let value = c.load(0x6002dbf8, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002d988 vfma.f32 s13,s0,s12
        let value = f32::from_bits(c.r(768, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(816, 4) as u32),
                f32::from_bits(c.r(820, 4) as u32),
            )
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002d98c vldr.32 s12,[r10,#0x2d8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(728u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x6002d990 vcmpe.f32 s15,s17
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
        t35c00 = ((f32::from_bits(c.r(828, 4) as u32) < f32::from_bits(c.r(836, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(828, 4) as u32) == f32::from_bits(c.r(836, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(836, 4) as u32) <= f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(836, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d994 vstr.32 s17,[r2]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(40, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(836, 4));
    }
    {
        // 0x6002d998 ldr r2,[sp,#0x8]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x6002d99a str.w r3,[r10,#0x2c4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(708u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002d99e vmrs apsr,fpscr
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
        // 0x6002d9a2 vstr.32 s17,[r2]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(40, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(836, 4));
    }
    {
        // 0x6002d9a6 vstr.32 s13,[r10,#0x2ac]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(684u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x6002d9aa it le
        let mut t186100 = 0u64;
        let mut t186200 = 0u64;
        let mut t186400 = 0u64;
        t186100 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t186200 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t186400 = (((((t186100 >> 0) & 0xffu64) != 0) & (((t186200 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
    }
    {
        // 0x6002d9ac vmov.le.f32 s15,s17
        let mut t2200 = 0u64;
        let mut t2400 = 0u64;
        let mut t2700 = 0u64;
        t2200 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2400 = (((c.r(97, 1) != 0) | (((t2200 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        t2700 = ((((t2400 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x6002d9b0u32;
        }
        let value = c.r(836, 4);
        c.w(828, 4, value);
    }
    0x6002d9b0
}
pub fn b_6002d9b0(c: &mut Machine) -> u32 {
    {
        // 0x6002d9b0 vstr.32 s13,[r10,#0x2b0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(688u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x6002d9b4 vmov.f32 s13,0x40000000
        let value = 1073741824u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002d9b8 vmul.f32 s15,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d9bc vcmpe.f32 s15,s13
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
        t35c00 = ((f32::from_bits(c.r(828, 4) as u32) < f32::from_bits(c.r(820, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(828, 4) as u32) == f32::from_bits(c.r(820, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(820, 4) as u32) <= f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(820, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002d9c0 vmrs apsr,fpscr
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
        // 0x6002d9c4 bpl.w 0x6002db5c
        let mut t182d00 = 0u64;
        t182d00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        if ((t182d00 >> 0) & 0xffu64) != 0 {
            return 0x6002db5cu32;
        }
    }
    0x6002d9c8
}
pub fn b_6002d9c8(c: &mut Machine) -> u32 {
    {
        // 0x6002d9c8 vmov.f32 s13,0x3e800000
        let value = 1048576000u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002d9cc vmov.f32 s12,0x40800000
        let value = 1082130432u64;
        c.w(816, 4, value);
    }
    {
        // 0x6002d9d0 vmul.f32 s13,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002d9d4 vfms.f32 s13,s13,s13
        let value = (-f32::from_bits(c.r(820, 4) as u32))
            .mul_add(
                f32::from_bits(c.r(820, 4) as u32),
                f32::from_bits(c.r(820, 4) as u32),
            )
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002d9d8 vmul.f32 s13,s13,s12
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    0x6002d9dc
}
pub fn b_6002d9dc(c: &mut Machine) -> u32 {
    {
        // 0x6002d9dc vsub.f32 s15,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d9e0 vldr.32 s13,[r10,#0x2c8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(712u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002d9e4 vldr.32 s0,[pc,#0x214]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610799588u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(532u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002d9e8 vmul.f32 s14,s14,s15
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002d9ec vmul.f32 s14,s14,s13
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002d9f0 vmul.f32 s0,s14,s0
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002d9f4 bl 0x60040c70
        let value = (1610799608u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002d9f8
}
pub fn b_6002d9f8(c: &mut Machine) -> u32 {
    {
        // 0x6002d9f8 vmov.f32 s15,0xc1700000
        let value = 3245342720u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002d9fc vldr.32 s13,[r10,#0x2c4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(708u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002da00 vldr.32 s14,[r10,#0x2d4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(724u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002da04 vsub.f32 s15,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002da08 vldr.32 s12,[r10,#0x2d8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(728u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x6002da0c vmov.f32 s13,0x40000000
        let value = 1073741824u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002da10 ldr r3,[0x6002dc00]
        let value = c.load(0x6002dc00, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002da12 vstr.32 s0,[r10,#0x380]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(896u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002da16 vadd.f32 s15,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002da1a str.w r3,[r10,#0x2c8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(712u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002da1e vcmpe.f32 s15,#0
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
        // 0x6002da22 vmrs apsr,fpscr
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
        // 0x6002da26 it le
        let mut t186100 = 0u64;
        let mut t186200 = 0u64;
        let mut t186400 = 0u64;
        t186100 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t186200 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t186400 = (((((t186100 >> 0) & 0xffu64) != 0) & (((t186200 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
    }
    {
        // 0x6002da28 vldr.le.32 s15,[pc,#0x1d8]
        let mut t2200 = 0u64;
        let mut t2400 = 0u64;
        let mut t2700 = 0u64;
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610799656u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(472u64)) & 0xffffffffu64;
        t2200 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2400 = (((c.r(97, 1) != 0) | (((t2200 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        t2700 = ((((t2400 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x6002da2cu32;
        }
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    0x6002da2c
}
pub fn b_6002da2c(c: &mut Machine) -> u32 {
    {
        // 0x6002da2c vmul.f32 s15,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002da30 vcmpe.f32 s15,s13
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
        t35c00 = ((f32::from_bits(c.r(828, 4) as u32) < f32::from_bits(c.r(820, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(828, 4) as u32) == f32::from_bits(c.r(820, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(820, 4) as u32) <= f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(820, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002da34 vmrs apsr,fpscr
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
        // 0x6002da38 bpl.w 0x6002db68
        let mut t182d00 = 0u64;
        t182d00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        if ((t182d00 >> 0) & 0xffu64) != 0 {
            return 0x6002db68u32;
        }
    }
    0x6002da3c
}
pub fn b_6002da3c(c: &mut Machine) -> u32 {
    {
        // 0x6002da3c vmov.f32 s13,0x3e800000
        let value = 1048576000u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002da40 vmov.f32 s12,0x40800000
        let value = 1082130432u64;
        c.w(816, 4, value);
    }
    {
        // 0x6002da44 vmul.f32 s13,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002da48 vfms.f32 s13,s13,s13
        let value = (-f32::from_bits(c.r(820, 4) as u32))
            .mul_add(
                f32::from_bits(c.r(820, 4) as u32),
                f32::from_bits(c.r(820, 4) as u32),
            )
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002da4c vmul.f32 s13,s13,s12
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    0x6002da50
}
pub fn b_6002da50(c: &mut Machine) -> u32 {
    {
        // 0x6002da50 vsub.f32 s15,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002da54 vldr.32 s13,[pc,#0x1b0]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610799700u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(432u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002da58 vldr.32 s0,[pc,#0x1a0]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610799704u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(416u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002da5c ldr r5,[0x6002dc0c]
        let value = c.load(0x6002dc0c, 4);
        c.w(52, 4, value);
    }
    {
        // 0x6002da5e vmul.f32 s14,s14,s15
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002da62 ldr r7,[0x6002dc10]
        let value = c.load(0x6002dc10, 4);
        c.w(60, 4, value);
    }
    {
        // 0x6002da64 mov r4,r5
        let value = c.r(52, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6002da66 add.w r6,r5,#0x630
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1584u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(52, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(52, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(52, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(56, 4, value);
        let value = (signed(c.r(56, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(56, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002da6a vmul.f32 s14,s14,s13
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002da6e vmul.f32 s0,s14,s0
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002da72 bl 0x60040c70
        let value = (1610799734u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002da76
}
pub fn b_6002da76(c: &mut Machine) -> u32 {
    {
        // 0x6002da76 vldr.32 s11,[r10,#0x2c0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(704u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x6002da7a vldr.32 s10,[pc,#0x198]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610799738u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(408u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x6002da7e vmov.f32 s13,0x3f000000
        let value = 1056964608u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002da82 vldr.32 s14,[pc,#0x194]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610799746u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(404u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002da86 vmov.f32 s12,s0
        let value = c.r(768, 4);
        c.w(816, 4, value);
    }
    {
        // 0x6002da8a vdiv.f32 s15,s10,s11
        let value = (f32::from_bits(c.r(808, 4) as u32) / f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002da8e vldr.32 s0,[pc,#0x18c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610799758u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(396u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002da92 vstr.32 s12,[r10,#0x380]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(896u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x6002da96 vmul.f32 s15,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002da9a vcmpe.f32 s15,s14
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
        t35c00 = ((f32::from_bits(c.r(828, 4) as u32) < f32::from_bits(c.r(824, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(828, 4) as u32) == f32::from_bits(c.r(824, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(824, 4) as u32) <= f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(824, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002da9e vmrs apsr,fpscr
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
        // 0x6002daa2 it pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x6002daa4 vmov.pl.f32 s15,s14
        let mut tf00 = 0u64;
        let mut t2700 = 0u64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2700 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x6002daa8u32;
        }
        let value = c.r(824, 4);
        c.w(828, 4, value);
    }
    0x6002daa8
}
pub fn b_6002daa8(c: &mut Machine) -> u32 {
    {
        // 0x6002daa8 vstr.32 s15,[r10,#0x2d0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(720u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002daac bl 0x60040c70
        let value = (1610799792u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002dab0
}
pub fn b_6002dab0(c: &mut Machine) -> u32 {
    {
        // 0x6002dab0 vmov.f32 s15,s0
        let value = c.r(768, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002dab4 vldr.32 s0,[pc,#0x168]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610799796u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(360u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002dab8 vstr.32 s15,[r10,#0x2e8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(744u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002dabc bl 0x60040c70
        let value = (1610799808u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002dac0
}
pub fn b_6002dac0(c: &mut Machine) -> u32 {
    {
        // 0x6002dac0 vmov.f32 s14,0x3f800000
        let value = 1065353216u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002dac4 vldr.32 s10,[pc,#0x14c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610799812u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(332u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x6002dac8 vldr.32 s11,[r10,#0x2e4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(740u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x6002dacc movs r3,#0x0
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
        // 0x6002dace vldr.32 s12,[pc,#0x134]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610799822u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(308u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x6002dad2 movs r1,#0x0
        let value = 0u64;
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
    }
    {
        // 0x6002dad4 vdiv.f32 s15,s10,s11
        let value = (f32::from_bits(c.r(808, 4) as u32) / f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002dad8 vldr.32 s13,[r10,#0x308]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(776u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002dadc vldr.32 s10,[pc,#0x144]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610799836u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(324u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x6002dae0 ldr r2,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x6002dae2 vstr.32 s0,[r10,#0x2ec]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(748u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002dae6 str r3,[r2,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(40, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002dae8 vdiv.f32 s11,s14,s12
        let value = (f32::from_bits(c.r(824, 4) as u32) / f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x6002daec vldr.32 s12,[pc,#0x128]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610799852u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(296u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x6002daf0 ldr r2,[sp,#0x8]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x6002daf2 strh.w r1,[r10,#0x384]
        let mut t19c800 = 0u64;
        let mut t1fb700 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(900u64)) & 0xffffffffu64;
        t1fb700 = ((c.r(36, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            2,
            ((t1fb700 >> 0) & 0xffffu64),
        );
    }
    {
        // 0x6002daf6 str r3,[r2,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(40, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002daf8 vmul.f32 s13,s13,s10
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002dafc vstr.32 s13,[r10,#0x30c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(780u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x6002db00 vmul.f32 s15,s15,s11
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002db04 vcmpe.f32 s15,s12
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
        t35c00 = ((f32::from_bits(c.r(828, 4) as u32) < f32::from_bits(c.r(816, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(828, 4) as u32) == f32::from_bits(c.r(816, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(816, 4) as u32) <= f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(816, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002db08 vmrs apsr,fpscr
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
        // 0x6002db0c ite mi
        let mut t184c00 = 0u64;
        t184c00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x6002db0e vsub.mi.f32 s14,s14,s15
        let mut td00 = 0u64;
        let mut t2600 = 0u64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2600 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2600 >> 0) & 0xffu64) != 0 {
            return 0x6002db12u32;
        }
        let value = (f32::from_bits(c.r(824, 4) as u32) - f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    0x6002db12
}
pub fn b_6002db12(c: &mut Machine) -> u32 {
    {
        // 0x6002db12 vldr.pl.32 s14,[pc,#0x114]
        let mut tf00 = 0u64;
        let mut t2700 = 0u64;
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610799890u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(276u64)) & 0xffffffffu64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2700 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x6002db16u32;
        }
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    0x6002db16
}
pub fn b_6002db16(c: &mut Machine) -> u32 {
    {
        // 0x6002db16 vstr.32 s14,[r10,#0x2f0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(752u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002db1a b 0x6002db2a
        return 0x6002db2au32;
    }
    0x6002db1c
}
pub fn b_6002db1c(c: &mut Machine) -> u32 {
    {
        // 0x6002db1c blx r3
        let mut t0 = 0u64;
        t0 = (c.r(44, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(44, 4) & 4294967294u64);
        c.w(92, 4, value);
        let value = (1610799902u64 | 1u64);
        c.w(88, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x6002db1e
}
pub fn b_6002db1e(c: &mut Machine) -> u32 {
    {
        // 0x6002db1e ldr r3,[r4,#0x40]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002db20 mov r0,r7
        let value = c.r(60, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002db22 blx r3
        let mut t0 = 0u64;
        t0 = (c.r(44, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(44, 4) & 4294967294u64);
        c.w(92, 4, value);
        let value = (1610799908u64 | 1u64);
        c.w(88, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x6002db24
}
pub fn b_6002db24(c: &mut Machine) -> u32 {
    {
        // 0x6002db24 adds r4,#0x48
        let value = ((c.r(48, 4) as u128 + 72u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 72u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(72u64);
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
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
        // 0x6002db26 cmp r4,r6
        let mut t1a6e00 = 0u64;
        let value = (c.r(56, 4) <= c.r(48, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), c.r(56, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(48, 4)).wrapping_sub(c.r(56, 4))) & 0xffffffffu64;
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
        // 0x6002db28 beq 0x6002db4c
        let mut t180200 = 0u64;
        t180200 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        if ((t180200 >> 0) & 0xffu64) != 0 {
            return 0x6002db4cu32;
        }
    }
    0x6002db2a
}
pub fn b_6002db2a(c: &mut Machine) -> u32 {
    {
        // 0x6002db2a ldr r3,[r4,#0x40]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002db2c cmp r3,#0x0
        let mut t1a6800 = 0u64;
        let value = (0u64 <= c.r(44, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), 0u64, 4, true);
        c.w(103, 1, value);
        t1a6800 = ((c.r(44, 4)).wrapping_sub(0u64)) & 0xffffffffu64;
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
        // 0x6002db2e beq 0x6002db24
        let mut t180200 = 0u64;
        t180200 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        if ((t180200 >> 0) & 0xffu64) != 0 {
            return 0x6002db24u32;
        }
    }
    0x6002db30
}
pub fn b_6002db30(c: &mut Machine) -> u32 {
    {
        // 0x6002db30 vldr.32 s2,[r4,#0x20]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(776, 4, value);
    }
    {
        // 0x6002db34 ldr r3,[r4,#0x3c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(60u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002db36 vldr.32 s1,[r4,#0x1c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(772, 4, value);
    }
    {
        // 0x6002db3a vldr.32 s0,[r4,#0x18]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(24u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002db3e vstr.32 s2,[r4,#0x28]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(776, 4));
    }
    {
        // 0x6002db42 cmp r3,#0x0
        let mut t1a6800 = 0u64;
        let value = (0u64 <= c.r(44, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), 0u64, 4, true);
        c.w(103, 1, value);
        t1a6800 = ((c.r(44, 4)).wrapping_sub(0u64)) & 0xffffffffu64;
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
        // 0x6002db44 bne 0x6002db1c
        let mut t180400 = 0u64;
        t180400 = ((c.r(97, 1) == 0u64) as u64) & 0xffu64;
        if ((t180400 >> 0) & 0xffu64) != 0 {
            return 0x6002db1cu32;
        }
    }
    0x6002db46
}
pub fn b_6002db46(c: &mut Machine) -> u32 {
    {
        // 0x6002db46 bl 0x6003005c
        let value = (1610799946u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x6003005cu32;
    }
    0x6002db4a
}
pub fn b_6002db4a(c: &mut Machine) -> u32 {
    {
        // 0x6002db4a b 0x6002db1e
        return 0x6002db1eu32;
    }
    0x6002db4c
}
pub fn b_6002db4c(c: &mut Machine) -> u32 {
    {
        // 0x6002db4c ldr r3,[0x6002dc2c]
        let value = c.load(0x6002dc2c, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002db4e str.w r3,[r5,#0xc0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(192u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002db52 add sp,#0x54
        let mut t187500 = 0u64;
        t187500 = (84u64) & 0xffffffffu64;
        let value = (c.r(84, 4)).wrapping_add(((t187500 >> 0) & 0xffffffffu64));
        c.w(84, 4, value);
    }
    {
        // 0x6002db54 vpop {d8,d9,d10}
        let mut t74b00 = 0u64;
        let value = c.r(84, 4);
        c.w(128, 4, value);
        t74b00 = ((6u64).wrapping_mul(4u64)) & 0xffffffffu64;
        let value = (c.r(84, 4)).wrapping_add(((t74b00 >> 0) & 0xffffffffu64));
        c.w(84, 4, value);
        let value = c.load(c.r(128, 4) as u32, 8);
        c.w(832, 8, value);
        let value = (c.r(128, 4)).wrapping_add(8u64);
        c.w(128, 4, value);
        let value = c.load(c.r(128, 4) as u32, 8);
        c.w(840, 8, value);
        let value = (c.r(128, 4)).wrapping_add(8u64);
        c.w(128, 4, value);
        let value = c.load(c.r(128, 4) as u32, 8);
        c.w(848, 8, value);
        let value = (c.r(128, 4)).wrapping_add(8u64);
        c.w(128, 4, value);
    }
    {
        // 0x6002db58 pop.w {r4,r5,r6,r7,r8,r9,r10,r11,pc}
        let mut t0 = 0u64;
        let value = c.r(84, 4);
        c.w(128, 4, value);
        let value = c.load(c.r(128, 4) as u32, 4);
        c.w(48, 4, value);
        let value = (c.r(128, 4)).wrapping_add(4u64);
        c.w(128, 4, value);
        let value = c.load(c.r(128, 4) as u32, 4);
        c.w(52, 4, value);
        let value = (c.r(128, 4)).wrapping_add(4u64);
        c.w(128, 4, value);
        let value = c.load(c.r(128, 4) as u32, 4);
        c.w(56, 4, value);
        let value = (c.r(128, 4)).wrapping_add(4u64);
        c.w(128, 4, value);
        let value = c.load(c.r(128, 4) as u32, 4);
        c.w(60, 4, value);
        let value = (c.r(128, 4)).wrapping_add(4u64);
        c.w(128, 4, value);
        let value = c.load(c.r(128, 4) as u32, 4);
        c.w(64, 4, value);
        let value = (c.r(128, 4)).wrapping_add(4u64);
        c.w(128, 4, value);
        let value = c.load(c.r(128, 4) as u32, 4);
        c.w(68, 4, value);
        let value = (c.r(128, 4)).wrapping_add(4u64);
        c.w(128, 4, value);
        let value = c.load(c.r(128, 4) as u32, 4);
        c.w(72, 4, value);
        let value = (c.r(128, 4)).wrapping_add(4u64);
        c.w(128, 4, value);
        let value = c.load(c.r(128, 4) as u32, 4);
        c.w(76, 4, value);
        let value = (c.r(128, 4)).wrapping_add(4u64);
        c.w(128, 4, value);
        let value = c.load(c.r(128, 4) as u32, 4);
        c.w(92, 4, value);
        let value = (c.r(128, 4)).wrapping_add(4u64);
        c.w(128, 4, value);
        let value = c.r(128, 4);
        c.w(84, 4, value);
        t0 = (c.r(92, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(92, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x6002db5c
}
pub fn b_6002db5c(c: &mut Machine) -> u32 {
    {
        // 0x6002db5c vmov.f32 s13,0x3f800000
        let value = 1065353216u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002db60 b 0x6002d9dc
        return 0x6002d9dcu32;
    }
    0x6002db62
}
pub fn b_6002db62(c: &mut Machine) -> u32 {
    {
        // 0x6002db62 vldr.32 s0,[pc,#0xcc]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610799970u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(204u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002db66 b 0x6002d60a
        return 0x6002d60au32;
    }
    0x6002db68
}
pub fn b_6002db68(c: &mut Machine) -> u32 {
    {
        // 0x6002db68 vmov.f32 s13,0x3f800000
        let value = 1065353216u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002db6c b 0x6002da50
        return 0x6002da50u32;
    }
    0x6002db6e
}
pub fn b_6002db6e(c: &mut Machine) -> u32 {
    {
        // 0x6002db6e vmov.f32 s13,s11
        let value = c.r(812, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002db72 vldr.32 s16,[pc,#0xc0]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610799986u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(192u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(832, 4, value);
    }
    {
        // 0x6002db76 b.w 0x6002c244
        return 0x6002c244u32;
    }
    0x6002db7a
}
pub fn b_6002db7a(c: &mut Machine) -> u32 {
    {
        // 0x6002db7a vldr.32 s13,[pc,#0xbc]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610799994u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(188u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002db7e b.w 0x6002cb28
        return 0x6002cb28u32;
    }
    0x6002db82
}
pub fn b_6002db82(c: &mut Machine) -> u32 {
    {
        // 0x6002db82 vmov.f32 s13,s11
        let value = c.r(812, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002db86 vldr.32 s15,[pc,#0xb4]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610800006u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(180u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002db8a b.w 0x6002c404
        return 0x6002c404u32;
    }
    0x6002db8e
}
pub fn b_6002db8e(c: &mut Machine) -> u32 {
    {
        // 0x6002db8e vldr.32 s13,[pc,#0xa8]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610800014u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(168u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002db92 b 0x6002d5c2
        return 0x6002d5c2u32;
    }
    0x6002db94
}
pub fn b_6002db94(c: &mut Machine) -> u32 {
    {
        // 0x6002db94 vldr.32 s13,[pc,#0xa0]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610800020u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(160u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002db98 b 0x6002d55a
        return 0x6002d55au32;
    }
    0x6002db9a
}
pub fn b_6002db9a(c: &mut Machine) -> u32 {
    {
        // 0x6002db9a vldr.32 s14,[pc,#0xa4]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610800026u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(164u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002db9e vmov.f32 s13,s14
        let value = c.r(824, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002dba2 b 0x6002d4f6
        return 0x6002d4f6u32;
    }
    0x6002dba4
}
pub fn b_6002dba4(c: &mut Machine) -> u32 {
    {
        // 0x6002dba4 vldr.32 s14,[pc,#0x90]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610800036u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(144u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002dba8 b.w 0x6002cbaa
        return 0x6002cbaau32;
    }
    0x6002dbac
}
pub fn b_6002dbac(c: &mut Machine) -> u32 {
    {
        // 0x6002dbac vmul.f32 s13,s8,s8
        let value = (f32::from_bits(c.r(800, 4) as u32) * f32::from_bits(c.r(800, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002dbb0 vldr.32 s10,[pc,#0x90]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610800048u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(144u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x6002dbb4 vmov.f32 s14,s8
        let value = c.r(800, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002dbb8 vldr.32 s11,[pc,#0x8c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610800056u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(140u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x6002dbbc vmul.f32 s12,s8,s13
        let value = (f32::from_bits(c.r(800, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x6002dbc0 vfma.f32 s14,s12,s10
        let value = f32::from_bits(c.r(816, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002dbc4 vmul.f32 s13,s13,s12
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002dbc8 vfma.f32 s14,s13,s11
        let value = f32::from_bits(c.r(820, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(812, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002dbcc vadd.f32 s11,s14,s0
        let value = (f32::from_bits(c.r(824, 4) as u32) + f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x6002dbd0 vsub.f32 s10,s14,s0
        let value = (f32::from_bits(c.r(824, 4) as u32) - f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x6002dbd4 vdiv.f32 s12,s14,s11
        let value = (f32::from_bits(c.r(824, 4) as u32) / f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x6002dbd8 vdiv.f32 s13,s10,s11
        let value = (f32::from_bits(c.r(808, 4) as u32) / f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002dbdc vfma.f32 s15,s12,s4
        let value = f32::from_bits(c.r(816, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(784, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002dbe0 vmov.f32 s14,s13
        let value = c.r(820, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002dbe4 vfma.f32 s14,s12,s4
        let value = f32::from_bits(c.r(816, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(784, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002dbe8 b 0x6002d4f6
        return 0x6002d4f6u32;
    }
    0x6002dbea
}
pub fn b_6003005c(c: &mut Machine) -> u32 {
    {
        // 0x6003005c vsub.f32 s1,s1,s0
        let value = (f32::from_bits(c.r(772, 4) as u32) - f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(772, 4, value);
    }
    {
        // 0x60030060 vfma.f32 s0,s1,s2
        let value = f32::from_bits(c.r(772, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(776, 4) as u32),
                f32::from_bits(c.r(768, 4) as u32),
            )
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x60030064 bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x60030066
}
pub fn b_60033dc0(c: &mut Machine) -> u32 {
    {
        // 0x60033dc0 ldr r3,[0x60033e00]
        let value = c.load(0x60033e00, 4);
        c.w(44, 4, value);
    }
    {
        // 0x60033dc2 ldr r2,[0x60033e04]
        let value = c.load(0x60033e04, 4);
        c.w(40, 4, value);
    }
    {
        // 0x60033dc4 ldrb r3,[r3,#0x0]
        let mut t19e400 = 0u64;
        let mut t1a7d00 = 0u64;
        t19e400 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        t1a7d00 = (c.load(((t19e400 >> 0) & 0xffffffffu64) as u32, 1)) & 0xffu64;
        let value = ((t1a7d00 >> 0) & 0xffu64);
        c.w(44, 4, value);
    }
    {
        // 0x60033dc6 push {lr}
        let value = c.r(84, 4);
        c.w(128, 4, value);
        let value = (c.r(128, 4)).wrapping_sub(4u64);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 4, c.r(88, 4));
        let value = c.r(128, 4);
        c.w(84, 4, value);
    }
    {
        // 0x60033dc8 strh r0,[r2,#0x0]
        let mut t19e200 = 0u64;
        let mut t1fa100 = 0u64;
        t19e200 = ((c.r(40, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        t1fa100 = ((c.r(32, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffffu64;
        c.store(
            ((t19e200 >> 0) & 0xffffffffu64) as u32,
            2,
            ((t1fa100 >> 0) & 0xffffu64),
        );
    }
    {
        // 0x60033dca sub sp,#0xc
        let mut t187500 = 0u64;
        t187500 = (12u64) & 0xffffffffu64;
        let value = (c.r(84, 4)).wrapping_sub(((t187500 >> 0) & 0xffffffffu64));
        c.w(84, 4, value);
    }
    {
        // 0x60033dcc cbnz r3,0x60033dd4
        let mut t1a5900 = 0u64;
        t1a5900 = ((c.r(44, 4) != 0u64) as u64) & 0xffu64;
        if ((t1a5900 >> 0) & 0xffu64) != 0 {
            return 0x60033dd4u32;
        }
    }
    0x60033dce
}
pub fn b_60033dce(c: &mut Machine) -> u32 {
    {
        // 0x60033dce add sp,#0xc
        let mut t187500 = 0u64;
        t187500 = (12u64) & 0xffffffffu64;
        let value = (c.r(84, 4)).wrapping_add(((t187500 >> 0) & 0xffffffffu64));
        c.w(84, 4, value);
    }
    {
        // 0x60033dd0 pop.w pc
        let mut t0 = 0u64;
        let mut t1ba100 = 0u64;
        t1ba100 = (c.load(c.r(84, 4) as u32, 4)) & 0xffffffffu64;
        let value = (c.r(84, 4)).wrapping_add(4u64);
        c.w(84, 4, value);
        t0 = (((t1ba100 >> 0) & 0xffffffffu64) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (((t1ba100 >> 0) & 0xffffffffu64) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x60033dd4
}
pub fn b_60033dd4(c: &mut Machine) -> u32 {
    {
        // 0x60033dd4 movw r12,#0xfff
        let value = 4095u64;
        c.w(80, 4, value);
        let value = (signed(c.r(80, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(80, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60033dd8 ldr r1,[0x60033e08]
        let value = c.load(0x60033e08, 4);
        c.w(36, 4, value);
    }
    {
        // 0x60033dda orr lr,r0,#0x3000
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (12288u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = (c.r(32, 4) | ((t188500 >> 0) & 0xffffffffu64));
        c.w(88, 4, value);
        let value = c.r(104, 1);
        c.w(102, 1, value);
        let value = c.r(99, 1);
        c.w(103, 1, value);
        let value = (signed(c.r(88, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(88, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60033dde movs r3,#0x0
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
        // 0x60033de0 sub.w r12,r12,r0
        let mut t1fe000 = 0u64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        t1fe000 = (c.r(32, 4)) & 0xffffffffu64;
        let value = (((t1fe000 >> 0) & 0xffffffffu64) <= c.r(80, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(80, 4), ((t1fe000 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(80, 4)).wrapping_sub(((t1fe000 >> 0) & 0xffffffffu64));
        c.w(80, 4, value);
        let value = (signed(c.r(80, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(80, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60033de4 movs r2,#0x4
        let value = 4u64;
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
    }
    {
        // 0x60033de6 ldr r0,[0x60033e0c]
        let value = c.load(0x60033e0c, 4);
        c.w(32, 4, value);
    }
    {
        // 0x60033de8 orr r12,r12,#0xb000
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (45056u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = (c.r(80, 4) | ((t188500 >> 0) & 0xffffffffu64));
        c.w(80, 4, value);
        let value = c.r(104, 1);
        c.w(102, 1, value);
        let value = c.r(99, 1);
        c.w(103, 1, value);
        let value = (signed(c.r(80, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(80, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60033dec str r3,[sp,#0x0]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x60033dee strh.w lr,[r1,#0x0]
        let mut t19c800 = 0u64;
        let mut t1fb700 = 0u64;
        t19c800 = ((c.r(36, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        t1fb700 = ((c.r(88, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            2,
            ((t1fb700 >> 0) & 0xffffu64),
        );
    }
    {
        // 0x60033df2 strh.w r12,[r1,#0x2]
        let mut t19c800 = 0u64;
        let mut t1fb700 = 0u64;
        t19c800 = ((c.r(36, 4)).wrapping_add(2u64)) & 0xffffffffu64;
        t1fb700 = ((c.r(80, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            2,
            ((t1fb700 >> 0) & 0xffffu64),
        );
    }
    {
        // 0x60033df6 bl 0x60034f74
        let value = (1610825210u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60034f74u32;
    }
    0x60033dfa
}
pub fn b_60033dfa(c: &mut Machine) -> u32 {
    {
        // 0x60033dfa add sp,#0xc
        let mut t187500 = 0u64;
        t187500 = (12u64) & 0xffffffffu64;
        let value = (c.r(84, 4)).wrapping_add(((t187500 >> 0) & 0xffffffffu64));
        c.w(84, 4, value);
    }
    {
        // 0x60033dfc pop.w pc
        let mut t0 = 0u64;
        let mut t1ba100 = 0u64;
        t1ba100 = (c.load(c.r(84, 4) as u32, 4)) & 0xffffffffu64;
        let value = (c.r(84, 4)).wrapping_add(4u64);
        c.w(84, 4, value);
        t0 = (((t1ba100 >> 0) & 0xffffffffu64) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (((t1ba100 >> 0) & 0xffffffffu64) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x60033e00
}
pub fn b_60034f74(c: &mut Machine) -> u32 {
    {
        // 0x60034f74 push {r4,r5}
        let value = c.r(84, 4);
        c.w(128, 4, value);
        let value = (c.r(128, 4)).wrapping_sub(4u64);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 4, c.r(52, 4));
        let value = (c.r(128, 4)).wrapping_sub(4u64);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 4, c.r(48, 4));
        let value = c.r(128, 4);
        c.w(84, 4, value);
    }
    {
        // 0x60034f76 ldr r5,[r0,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(32, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(52, 4, value);
    }
    {
        // 0x60034f78 ldr r4,[0x60034f88]
        let value = c.load(0x60034f88, 4);
        c.w(48, 4, value);
    }
    {
        // 0x60034f7a add.w r12,r4,r5, lsl #0x5
        let mut t189200 = 0u64;
        let mut t189300 = 0u64;
        let mut t189500 = 0u64;
        let mut t189800 = 0u64;
        let mut t1a0b00 = 0u64;
        t189200 = ((32u64).wrapping_sub(5u64)) & 0xffffffffu64;
        t189300 = (if ((t189200 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(52, 4)) >> (((t189200 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t189500 = (((t189300 >> 0) & 0xffffffffu64) & 1u64) & 0xffffffffu64;
        let value = ((t189500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        t189800 = (if 5u64 >= 32 {
            0
        } else {
            (c.r(52, 4)) << (5u64 as u32)
        }) & 0xffffffffu64;
        t1a0b00 = ((t189800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        let value = ((c.r(48, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(80, 4, value);
        let value = (signed(c.r(80, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(80, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60034f7e ldr.w r4,[r12,#0x18]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(80, 4)).wrapping_add(24u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x60034f82 mov r12,r4
        let value = c.r(48, 4);
        c.w(80, 4, value);
    }
    {
        // 0x60034f84 pop {r4,r5}
        let value = c.r(84, 4);
        c.w(128, 4, value);
        let value = c.load(c.r(128, 4) as u32, 4);
        c.w(48, 4, value);
        let value = (c.r(128, 4)).wrapping_add(4u64);
        c.w(128, 4, value);
        let value = c.load(c.r(128, 4) as u32, 4);
        c.w(52, 4, value);
        let value = (c.r(128, 4)).wrapping_add(4u64);
        c.w(128, 4, value);
        let value = c.r(128, 4);
        c.w(84, 4, value);
    }
    {
        // 0x60034f86 bx r12
        let mut t0 = 0u64;
        t0 = (c.r(80, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(80, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x60034f88
}
pub fn b_60040c70(c: &mut Machine) -> u32 {
    {
        // 0x60040c70 vmov r3,s0
        let value = c.r(768, 4);
        c.w(44, 4, value);
    }
    {
        // 0x60040c74 ubfx r2,r3,#0x14,#0xb
        let mut t188900 = 0u64;
        let mut t188b00 = 0u64;
        let mut t21dc00 = 0u64;
        let mut t21de00 = 0u64;
        t188900 = (20u64) & 0xffffffffu64;
        t188b00 = (10u64) & 0xffffffffu64;
        t21dc00 = (((t188900 >> 0) & 0xffffffffu64).wrapping_add(((t188b00 >> 0) & 0xffffffffu64)))
            & 0xffffffffu64;
        t21de00 = ((31u64).wrapping_sub(((t21dc00 >> 0) & 0xffffffffu64))) & 0xffffffffu64;
        let value = if ((t21de00 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(44, 4)) << (((t21de00 >> 0) & 0xffffffffu64) as u32)
        };
        c.w(40, 4, value);
        t21de00 = ((31u64).wrapping_sub(((t188b00 >> 0) & 0xffffffffu64))) & 0xffffffffu64;
        let value = if ((t21de00 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(40, 4)) >> (((t21de00 >> 0) & 0xffffffffu64) as u32)
        };
        c.w(40, 4, value);
    }
    {
        // 0x60040c78 cmp.w r2,#0x430
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        let mut t1a6a00 = 0u64;
        t188500 = (1072u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = (((t188500 >> 0) & 0xffffffffu64) <= c.r(40, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(40, 4), ((t188500 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        t1a6a00 = ((c.r(40, 4)).wrapping_sub(((t188500 >> 0) & 0xffffffffu64))) & 0xffffffffu64;
        let value = (signed(((t1a6a00 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a6a00 >> 0) & 0xffffffffu64) == 0u64) as u64;
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
        // 0x60040c7c vcvt.f64.f32 d7,s0
        let value = (f32::from_bits(c.r(768, 4) as u32) as f64).to_bits() as u64;
        c.w(824, 8, value);
    }
    {
        // 0x60040c80 bcs 0x60040ce0
        let mut t180600 = 0u64;
        t180600 = ((c.r(98, 1) != 0u64) as u64) & 0xffu64;
        if ((t180600 >> 0) & 0xffu64) != 0 {
            return 0x60040ce0u32;
        }
    }
    0x60040c82
}
pub fn b_60040c82(c: &mut Machine) -> u32 {
    {
        // 0x60040c82 vmov.f64 d0,0x3ff0000000000000
        let value = 4607182418800017408u64;
        c.w(768, 8, value);
    }
    {
        // 0x60040c86 ldr r3,[0x60040d30]
        let value = c.load(0x60040d30, 4);
        c.w(44, 4, value);
    }
    {
        // 0x60040c88 vldr.64 d2,[r3,#0x100]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(256u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 8);
        c.w(784, 8, value);
    }
    {
        // 0x60040c8c vadd.f64 d3,d2,d7
        let value = (f64::from_bits(c.r(784, 8) as u64) + f64::from_bits(c.r(824, 8) as u64))
            .to_bits() as u64;
        c.w(792, 8, value);
    }
    {
        // 0x60040c90 vmov r0,s6
        let value = c.r(792, 4);
        c.w(32, 4, value);
    }
    {
        // 0x60040c94 vsub.f64 d3,d3,d2
        let value = (f64::from_bits(c.r(792, 8) as u64) - f64::from_bits(c.r(784, 8) as u64))
            .to_bits() as u64;
        c.w(792, 8, value);
    }
    {
        // 0x60040c98 and r2,r0,#0x1f
        let mut t187d00 = 0u64;
        t187d00 = (31u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (c.r(32, 4) & ((t187d00 >> 0) & 0xffffffffu64));
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60040c9c add.w r1,r3,r2, lsl #0x3
        let mut t189200 = 0u64;
        let mut t189300 = 0u64;
        let mut t189500 = 0u64;
        let mut t189800 = 0u64;
        let mut t1a0b00 = 0u64;
        t189200 = ((32u64).wrapping_sub(3u64)) & 0xffffffffu64;
        t189300 = (if ((t189200 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(40, 4)) >> (((t189200 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t189500 = (((t189300 >> 0) & 0xffffffffu64) & 1u64) & 0xffffffffu64;
        let value = ((t189500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        t189800 = (if 3u64 >= 32 {
            0
        } else {
            (c.r(40, 4)) << (3u64 as u32)
        }) & 0xffffffffu64;
        t1a0b00 = ((t189800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        let value = ((c.r(44, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60040ca0 push {r4,r5}
        let value = c.r(84, 4);
        c.w(128, 4, value);
        let value = (c.r(128, 4)).wrapping_sub(4u64);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 4, c.r(52, 4));
        let value = (c.r(128, 4)).wrapping_sub(4u64);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 4, c.r(48, 4));
        let value = c.r(128, 4);
        c.w(84, 4, value);
    }
    {
        // 0x60040ca2 vsub.f64 d7,d7,d3
        let value = (f64::from_bits(c.r(824, 8) as u64) - f64::from_bits(c.r(792, 8) as u64))
            .to_bits() as u64;
        c.w(824, 8, value);
    }
    {
        // 0x60040ca6 vldr.64 d4,[r3,#0x108]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(264u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 8);
        c.w(800, 8, value);
    }
    {
        // 0x60040caa ldr.w r4,[r3,r2,lsl #0x3]
        let mut t188f00 = 0u64;
        let mut t189100 = 0u64;
        t188f00 = (if 3u64 >= 32 {
            0
        } else {
            (c.r(40, 4)) << (3u64 as u32)
        }) & 0xffffffffu64;
        t189100 = ((c.r(44, 4)).wrapping_add(((t188f00 >> 0) & 0xffffffffu64))) & 0xffffffffu64;
        let value = c.load(((t189100 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x60040cae ldr r5,[r1,#0x4]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(36, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(52, 4, value);
    }
    {
        // 0x60040cb0 vldr.64 d6,[r3,#0x110]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(272u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 8);
        c.w(816, 8, value);
    }
    {
        // 0x60040cb4 vldr.64 d5,[r3,#0x118]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(280u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 8);
        c.w(808, 8, value);
    }
    {
        // 0x60040cb8 vfma.f64 d6,d4,d7
        let value = f64::from_bits(c.r(800, 8) as u64)
            .mul_add(
                f64::from_bits(c.r(824, 8) as u64),
                f64::from_bits(c.r(816, 8) as u64),
            )
            .to_bits() as u64;
        c.w(816, 8, value);
    }
    {
        // 0x60040cbc vmul.f64 d4,d7,d7
        let value = (f64::from_bits(c.r(824, 8) as u64) * f64::from_bits(c.r(824, 8) as u64))
            .to_bits() as u64;
        c.w(800, 8, value);
    }
    {
        // 0x60040cc0 movs r1,#0x0
        let value = 0u64;
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
    }
    {
        // 0x60040cc2 adds r2,r1,r4
        let value = ((c.r(36, 4) as u128 + c.r(48, 4) as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), c.r(48, 4), 4, false);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_add(c.r(48, 4));
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
        // 0x60040cc4 add.w r3,r5,r0, lsl #0xf
        let mut t189200 = 0u64;
        let mut t189300 = 0u64;
        let mut t189500 = 0u64;
        let mut t189800 = 0u64;
        let mut t1a0b00 = 0u64;
        t189200 = ((32u64).wrapping_sub(15u64)) & 0xffffffffu64;
        t189300 = (if ((t189200 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(32, 4)) >> (((t189200 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t189500 = (((t189300 >> 0) & 0xffffffffu64) & 1u64) & 0xffffffffu64;
        let value = ((t189500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        t189800 = (if 15u64 >= 32 {
            0
        } else {
            (c.r(32, 4)) << (15u64 as u32)
        }) & 0xffffffffu64;
        t1a0b00 = ((t189800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        let value = ((c.r(52, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(52, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(52, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(44, 4, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60040cc8 vfma.f64 d0,d5,d7
        let value = f64::from_bits(c.r(808, 8) as u64)
            .mul_add(
                f64::from_bits(c.r(824, 8) as u64),
                f64::from_bits(c.r(768, 8) as u64),
            )
            .to_bits() as u64;
        c.w(768, 8, value);
    }
    {
        // 0x60040ccc vmov d7,r2,r3
        let mut t6ac00 = 0u64;
        let mut t6ad00 = 0u64;
        let mut t6ae00 = 0u64;
        t6ac00 = (c.r(44, 4)) & 0xffffffffffffffffu64;
        t6ad00 = (if 32u64 >= 64 {
            0
        } else {
            ((t6ac00 >> 0) & 0xffffffffffffffffu64) << (32u64 as u32)
        }) & 0xffffffffffffffffu64;
        t6ae00 = (c.r(40, 4)) & 0xffffffffffffffffu64;
        let value = ((t6ad00 >> 0) & 0xffffffffffffffffu64)
            .wrapping_add(((t6ae00 >> 0) & 0xffffffffffffffffu64));
        c.w(824, 8, value);
    }
    {
        // 0x60040cd0 vfma.f64 d0,d6,d4
        let value = f64::from_bits(c.r(816, 8) as u64)
            .mul_add(
                f64::from_bits(c.r(800, 8) as u64),
                f64::from_bits(c.r(768, 8) as u64),
            )
            .to_bits() as u64;
        c.w(768, 8, value);
    }
    {
        // 0x60040cd4 pop {r4,r5}
        let value = c.r(84, 4);
        c.w(128, 4, value);
        let value = c.load(c.r(128, 4) as u32, 4);
        c.w(48, 4, value);
        let value = (c.r(128, 4)).wrapping_add(4u64);
        c.w(128, 4, value);
        let value = c.load(c.r(128, 4) as u32, 4);
        c.w(52, 4, value);
        let value = (c.r(128, 4)).wrapping_add(4u64);
        c.w(128, 4, value);
        let value = c.r(128, 4);
        c.w(84, 4, value);
    }
    {
        // 0x60040cd6 vmul.f64 d0,d0,d7
        let value = (f64::from_bits(c.r(768, 8) as u64) * f64::from_bits(c.r(824, 8) as u64))
            .to_bits() as u64;
        c.w(768, 8, value);
    }
    {
        // 0x60040cda vcvt.f32.f64 s0,d0
        let value = (f64::from_bits(c.r(768, 8) as u64) as f32).to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x60040cde bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x60040ce0
}
pub fn b_60040ce0(c: &mut Machine) -> u32 {
    {
        // 0x60040ce0 cmn r3,#0x800000
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        let mut t1a6200 = 0u64;
        t188500 = (8388608u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(44, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        t1a6200 = ((c.r(44, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64))) & 0xffffffffu64;
        let value = (signed(((t1a6200 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a6200 >> 0) & 0xffffffffu64) == 0u64) as u64;
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
        // 0x60040ce4 beq 0x60040d24
        let mut t180200 = 0u64;
        t180200 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        if ((t180200 >> 0) & 0xffu64) != 0 {
            return 0x60040d24u32;
        }
    }
    0x60040ce6
}
pub fn b_60040ce6(c: &mut Machine) -> u32 {
    {
        // 0x60040ce6 cmp.w r2,#0x7f8
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        let mut t1a6a00 = 0u64;
        t188500 = (2040u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = (((t188500 >> 0) & 0xffffffffu64) <= c.r(40, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(40, 4), ((t188500 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        t1a6a00 = ((c.r(40, 4)).wrapping_sub(((t188500 >> 0) & 0xffffffffu64))) & 0xffffffffu64;
        let value = (signed(((t1a6a00 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a6a00 >> 0) & 0xffffffffu64) == 0u64) as u64;
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
        // 0x60040cea bcs 0x60040d1e
        let mut t180600 = 0u64;
        t180600 = ((c.r(98, 1) != 0u64) as u64) & 0xffu64;
        if ((t180600 >> 0) & 0xffu64) != 0 {
            return 0x60040d1eu32;
        }
    }
    0x60040cec
}
pub fn b_60040cec(c: &mut Machine) -> u32 {
    {
        // 0x60040cec vcmpe.f32 s0,#0
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
        t35c00 = ((f32::from_bits(c.r(768, 4) as u32)
            < f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(768, 4) as u32)
            == f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)) as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(((t37900 >> 0) & 0xffffffffu64) as u32)
            <= f32::from_bits(c.r(768, 4) as u32)) as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(768, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x60040cf0 vmrs apsr,fpscr
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
        // 0x60040cf4 bgt 0x60040d18
        let mut t181b00 = 0u64;
        let mut t181c00 = 0u64;
        let mut t181e00 = 0u64;
        t181b00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t181c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t181e00 = (((((t181b00 >> 0) & 0xffu64) != 0) & (((t181c00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t181e00 >> 0) & 0xffu64) != 0 {
            return 0x60040d18u32;
        }
    }
    0x60040cf6
}
pub fn b_60040cf6(c: &mut Machine) -> u32 {
    {
        // 0x60040cf6 vldr.32 s13,[pc,#0x3c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610878198u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(60u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x60040cfa vcmpe.f32 s0,s13
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
        t35c00 = ((f32::from_bits(c.r(768, 4) as u32) < f32::from_bits(c.r(820, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(768, 4) as u32) == f32::from_bits(c.r(820, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(820, 4) as u32) <= f32::from_bits(c.r(768, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(768, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(820, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x60040cfe vmrs apsr,fpscr
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
        // 0x60040d02 bls 0x60040d2a
        let mut t181400 = 0u64;
        let mut t181600 = 0u64;
        t181400 = ((c.r(98, 1) == 0) as u64) & 0xffu64;
        t181600 = (((((t181400 >> 0) & 0xffu64) != 0) | (c.r(97, 1) != 0)) as u64) & 0xffu64;
        if ((t181600 >> 0) & 0xffu64) != 0 {
            return 0x60040d2au32;
        }
    }
    0x60040d04
}
pub fn b_60040d04(c: &mut Machine) -> u32 {
    {
        // 0x60040d04 vldr.32 s13,[pc,#0x30]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610878212u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(48u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x60040d08 vcmpe.f32 s0,s13
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
        t35c00 = ((f32::from_bits(c.r(768, 4) as u32) < f32::from_bits(c.r(820, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(768, 4) as u32) == f32::from_bits(c.r(820, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(820, 4) as u32) <= f32::from_bits(c.r(768, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(768, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(820, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x60040d0c vmrs apsr,fpscr
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
        // 0x60040d10 bpl 0x60040c82
        let mut t180c00 = 0u64;
        t180c00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        if ((t180c00 >> 0) & 0xffu64) != 0 {
            return 0x60040c82u32;
        }
    }
    0x60040d12
}
pub fn b_60040d12(c: &mut Machine) -> u32 {
    {
        // 0x60040d12 movs r0,#0x0
        let value = 0u64;
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
    }
    {
        // 0x60040d14 b.w 0x600412dc
        return 0x600412dcu32;
    }
    0x60040d18
}
pub fn b_60040d18(c: &mut Machine) -> u32 {
    {
        // 0x60040d18 movs r0,#0x0
        let value = 0u64;
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
    }
    {
        // 0x60040d1a b.w 0x600412e8
        return 0x600412e8u32;
    }
    0x60040d1e
}
pub fn b_60040d1e(c: &mut Machine) -> u32 {
    {
        // 0x60040d1e vadd.f32 s0,s0,s0
        let value = (f32::from_bits(c.r(768, 4) as u32) + f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x60040d22 bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x60040d24
}
pub fn b_60040d24(c: &mut Machine) -> u32 {
    {
        // 0x60040d24 vldr.32 s0,[pc,#0x14]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610878244u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(20u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x60040d28 bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x60040d2a
}
pub fn b_60040d2a(c: &mut Machine) -> u32 {
    {
        // 0x60040d2a movs r0,#0x0
        let value = 0u64;
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
    }
    {
        // 0x60040d2c b.w 0x600412d0
        return 0x600412d0u32;
    }
    0x60040d30
}
pub fn b_60040e08(c: &mut Machine) -> u32 {
    {
        // 0x60040e08 vmov r3,s0
        let value = c.r(768, 4);
        c.w(44, 4, value);
    }
    {
        // 0x60040e0c cmp.w r3,#0x3f800000
        let mut t188100 = 0u64;
        let mut t188300 = 0u64;
        let mut t1a6a00 = 0u64;
        t188100 = (1065353216u64) & 0xffffffffu64;
        t188300 = (if 31u64 >= 32 {
            0
        } else {
            ((t188100 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188300 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = (((t188100 >> 0) & 0xffffffffu64) <= c.r(44, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), ((t188100 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        t1a6a00 = ((c.r(44, 4)).wrapping_sub(((t188100 >> 0) & 0xffffffffu64))) & 0xffffffffu64;
        let value = (signed(((t1a6a00 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a6a00 >> 0) & 0xffffffffu64) == 0u64) as u64;
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
        // 0x60040e10 beq 0x60040e8e
        let mut t180200 = 0u64;
        t180200 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        if ((t180200 >> 0) & 0xffu64) != 0 {
            return 0x60040e8eu32;
        }
    }
    0x60040e12
}
pub fn b_60040e12(c: &mut Machine) -> u32 {
    {
        // 0x60040e12 sub.w r1,r3,#0x800000
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (8388608u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = (((t188500 >> 0) & 0xffffffffu64) <= c.r(44, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), ((t188500 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_sub(((t188500 >> 0) & 0xffffffffu64));
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60040e16 cmp.w r1,#0x7f000000
        let mut t188100 = 0u64;
        let mut t188300 = 0u64;
        let mut t1a6a00 = 0u64;
        t188100 = (2130706432u64) & 0xffffffffu64;
        t188300 = (if 31u64 >= 32 {
            0
        } else {
            ((t188100 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188300 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = (((t188100 >> 0) & 0xffffffffu64) <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), ((t188100 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        t1a6a00 = ((c.r(36, 4)).wrapping_sub(((t188100 >> 0) & 0xffffffffu64))) & 0xffffffffu64;
        let value = (signed(((t1a6a00 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a6a00 >> 0) & 0xffffffffu64) == 0u64) as u64;
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
        // 0x60040e1a mov r2,r3
        let value = c.r(44, 4);
        c.w(40, 4, value);
    }
    {
        // 0x60040e1c bcs 0x60040e94
        let mut t180600 = 0u64;
        t180600 = ((c.r(98, 1) != 0u64) as u64) & 0xffu64;
        if ((t180600 >> 0) & 0xffu64) != 0 {
            return 0x60040e94u32;
        }
    }
    0x60040e1e
}
pub fn b_60040e1e(c: &mut Machine) -> u32 {
    {
        // 0x60040e1e vmov.f64 d2,-0x4010000000000000
        let value = 13830554455654793216u64;
        c.w(784, 8, value);
    }
    {
        // 0x60040e22 add.w r3,r2,#0xc0000000
        let mut t188100 = 0u64;
        let mut t188300 = 0u64;
        t188100 = (3221225472u64) & 0xffffffffu64;
        t188300 = (if 31u64 >= 32 {
            0
        } else {
            ((t188100 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188300 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(40, 4) as u128 + ((t188100 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(40, 4), ((t188100 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(40, 4)).wrapping_add(((t188100 >> 0) & 0xffffffffu64));
        c.w(44, 4, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60040e26 ldr r1,[0x60040ecc]
        let value = c.load(0x60040ecc, 4);
        c.w(36, 4, value);
    }
    {
        // 0x60040e28 add.w r3,r3,#0xcd0000
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (13434880u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(44, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(44, 4, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60040e2c ubfx r0,r3,#0x13,#0x4
        let mut t188900 = 0u64;
        let mut t188b00 = 0u64;
        let mut t21dc00 = 0u64;
        let mut t21de00 = 0u64;
        t188900 = (19u64) & 0xffffffffu64;
        t188b00 = (3u64) & 0xffffffffu64;
        t21dc00 = (((t188900 >> 0) & 0xffffffffu64).wrapping_add(((t188b00 >> 0) & 0xffffffffu64)))
            & 0xffffffffu64;
        t21de00 = ((31u64).wrapping_sub(((t21dc00 >> 0) & 0xffffffffu64))) & 0xffffffffu64;
        let value = if ((t21de00 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(44, 4)) << (((t21de00 >> 0) & 0xffffffffu64) as u32)
        };
        c.w(32, 4, value);
        t21de00 = ((31u64).wrapping_sub(((t188b00 >> 0) & 0xffffffffu64))) & 0xffffffffu64;
        let value = if ((t21de00 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(32, 4)) >> (((t21de00 >> 0) & 0xffffffffu64) as u32)
        };
        c.w(32, 4, value);
    }
    {
        // 0x60040e30 add.w r0,r1,r0, lsl #0x4
        let mut t189200 = 0u64;
        let mut t189300 = 0u64;
        let mut t189500 = 0u64;
        let mut t189800 = 0u64;
        let mut t1a0b00 = 0u64;
        t189200 = ((32u64).wrapping_sub(4u64)) & 0xffffffffu64;
        t189300 = (if ((t189200 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(32, 4)) >> (((t189200 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t189500 = (((t189300 >> 0) & 0xffffffffu64) & 1u64) & 0xffffffffu64;
        let value = ((t189500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        t189800 = (if 4u64 >= 32 {
            0
        } else {
            (c.r(32, 4)) << (4u64 as u32)
        }) & 0xffffffffu64;
        t1a0b00 = ((t189800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        let value = ((c.r(36, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60040e34 vldr.64 d4,[r1,#0x108]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(264u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 8);
        c.w(800, 8, value);
    }
    {
        // 0x60040e38 vldr.64 d7,[r1,#0x110]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(272u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 8);
        c.w(824, 8, value);
    }
    {
        // 0x60040e3c vldr.64 d6,[r1,#0x100]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(256u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 8);
        c.w(816, 8, value);
    }
    {
        // 0x60040e40 vldr.64 d5,[r1,#0x118]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(280u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 8);
        c.w(808, 8, value);
    }
    {
        // 0x60040e44 lsrs r1,r3,#0x17
        let mut t1b0b00 = 0u64;
        let mut t1b0c00 = 0u64;
        let mut t1b0e00 = 0u64;
        let mut t1b0f00 = 0u64;
        let mut t1b1000 = 0u64;
        let mut t1b1100 = 0u64;
        let mut t1b1200 = 0u64;
        let mut t1b1300 = 0u64;
        let mut t1b1600 = 0u64;
        t1b1600 = (23u64) & 0xffffffffu64;
        t1b0b00 = (((t1b1600 >> 0) & 0xffffffffu64).wrapping_sub(1u64)) & 0xffffffffu64;
        t1b0c00 = (if ((t1b0b00 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(44, 4)) >> (((t1b0b00 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t1b0e00 = (((t1b0c00 >> 0) & 0xffffffffu64) & 1u64) & 0xffffffffu64;
        t1b0f00 = ((((t1b1600 >> 0) & 0xffffffffu64) == 0u64) as u64) & 0xffu64;
        t1b1000 = (((((t1b0f00 >> 0) & 0xffu64) != 0) & (c.r(98, 1) != 0)) as u64) & 0xffu64;
        t1b1100 = ((((t1b1600 >> 0) & 0xffffffffu64) != 0u64) as u64) & 0xffu64;
        t1b1200 = ((((t1b0e00 >> 0) & 0xffffffffu64) != 0u64) as u64) & 0xffu64;
        t1b1300 = (((((t1b1100 >> 0) & 0xffu64) != 0) & (((t1b1200 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        let value = ((((t1b1000 >> 0) & 0xffu64) != 0) | (((t1b1300 >> 0) & 0xffu64) != 0)) as u64;
        c.w(102, 1, value);
        let value = if 23u64 >= 32 {
            0
        } else {
            (c.r(44, 4)) >> (23u64 as u32)
        };
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
    }
    {
        // 0x60040e46 lsls r1,r1,#0x17
        let mut t1afd00 = 0u64;
        let mut t1afe00 = 0u64;
        let mut t1b0000 = 0u64;
        let mut t1b0100 = 0u64;
        let mut t1b0200 = 0u64;
        let mut t1b0300 = 0u64;
        let mut t1b0400 = 0u64;
        let mut t1b0500 = 0u64;
        t1afd00 = ((23u64).wrapping_sub(1u64)) & 0xffffffffu64;
        t1afe00 = (if ((t1afd00 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(36, 4)) << (((t1afd00 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t1b0000 = (((t1afe00 >> 0) & 0xffffffffu64) & 2147483648u64) & 0xffffffffu64;
        t1b0100 = ((23u64 == 0u64) as u64) & 0xffu64;
        t1b0200 = (((((t1b0100 >> 0) & 0xffu64) != 0) & (c.r(98, 1) != 0)) as u64) & 0xffu64;
        t1b0300 = ((23u64 != 0u64) as u64) & 0xffu64;
        t1b0400 = ((((t1b0000 >> 0) & 0xffffffffu64) != 0u64) as u64) & 0xffu64;
        t1b0500 = (((((t1b0300 >> 0) & 0xffu64) != 0) & (((t1b0400 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        let value = ((((t1b0200 >> 0) & 0xffu64) != 0) | (((t1b0500 >> 0) & 0xffu64) != 0)) as u64;
        c.w(102, 1, value);
        let value = if 23u64 >= 32 {
            0
        } else {
            (c.r(36, 4)) << (23u64 as u32)
        };
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
    }
    {
        // 0x60040e48 vpush {d8}
        let mut t73700 = 0u64;
        t73700 = ((2u64).wrapping_mul(4u64)) & 0xffffffffu64;
        let value = (c.r(84, 4)).wrapping_sub(((t73700 >> 0) & 0xffffffffu64));
        c.w(84, 4, value);
        let value = c.r(84, 4);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 8, c.r(832, 8));
        let value = (c.r(128, 4)).wrapping_add(8u64);
        c.w(128, 4, value);
    }
    {
        // 0x60040e4c vldr.64 d1,[r0,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 8);
        c.w(776, 8, value);
    }
    {
        // 0x60040e50 vldr.64 d8,[r0]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(32, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 8);
        c.w(832, 8, value);
    }
    {
        // 0x60040e54 subs r2,r2,r1
        let value = (c.r(36, 4) <= c.r(40, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(40, 4), c.r(36, 4), 4, true);
        c.w(103, 1, value);
        let value = (c.r(40, 4)).wrapping_sub(c.r(36, 4));
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
        // 0x60040e56 asrs r3,r3,#0x17
        let mut t1a1a00 = 0u64;
        let mut t1a1b00 = 0u64;
        let mut t1a1d00 = 0u64;
        let mut t1a1e00 = 0u64;
        let mut t1a1f00 = 0u64;
        let mut t1a2000 = 0u64;
        let mut t1a2100 = 0u64;
        let mut t1a2200 = 0u64;
        t1a1a00 = ((23u64).wrapping_sub(1u64)) & 0xffffffffu64;
        t1a1b00 = ((signed(c.r(44, 4), 4) >> ((t1a1a00 >> 0) & 0xffffffffu64).min(31) as u32)
            as u64)
            & 0xffffffffu64;
        t1a1d00 = (((t1a1b00 >> 0) & 0xffffffffu64) & 1u64) & 0xffffffffu64;
        t1a1e00 = ((23u64 == 0u64) as u64) & 0xffu64;
        t1a1f00 = (((((t1a1e00 >> 0) & 0xffu64) != 0) & (c.r(98, 1) != 0)) as u64) & 0xffu64;
        t1a2000 = ((23u64 != 0u64) as u64) & 0xffu64;
        t1a2100 = ((((t1a1d00 >> 0) & 0xffffffffu64) != 0u64) as u64) & 0xffu64;
        t1a2200 = (((((t1a2000 >> 0) & 0xffu64) != 0) & (((t1a2100 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        let value = ((((t1a1f00 >> 0) & 0xffu64) != 0) | (((t1a2200 >> 0) & 0xffu64) != 0)) as u64;
        c.w(102, 1, value);
        let value = (signed(c.r(44, 4), 4) >> (23u64).min(31) as u32) as u64;
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
    }
    {
        // 0x60040e58 vmov s0,r3
        let value = c.r(44, 4);
        c.w(768, 4, value);
    }
    {
        // 0x60040e5c vmov s7,r2
        let value = c.r(40, 4);
        c.w(796, 4, value);
    }
    {
        // 0x60040e60 vcvt.f64.s32 d0,s0
        let mut t3ad00 = 0u64;
        let mut t3ae00 = 0u64;
        let mut t3b000 = 0u64;
        t3ad00 = (if 22u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (22u64 as u32)
        }) & 0xffffffffu64;
        t3ae00 = (((t3ad00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        t3b000 = (((t3ae00 >> 0) & 0xffu64) & 3u64) & 0xffu64;
        let value = c.userop(219, c.r(768, 4), ((t3b000 >> 0) & 0xffu64), 0u64, 4, 8);
        c.w(768, 8, value);
    }
    {
        // 0x60040e64 vcvt.f64.f32 d3,s7
        let value = (f32::from_bits(c.r(796, 4) as u32) as f64).to_bits() as u64;
        c.w(792, 8, value);
    }
    {
        // 0x60040e68 vadd.f64 d0,d0,d1
        let value = (f64::from_bits(c.r(768, 8) as u64) + f64::from_bits(c.r(776, 8) as u64))
            .to_bits() as u64;
        c.w(768, 8, value);
    }
    {
        // 0x60040e6c vfma.f64 d2,d8,d3
        let value = f64::from_bits(c.r(832, 8) as u64)
            .mul_add(
                f64::from_bits(c.r(792, 8) as u64),
                f64::from_bits(c.r(784, 8) as u64),
            )
            .to_bits() as u64;
        c.w(784, 8, value);
    }
    {
        // 0x60040e70 vpop {d8}
        let mut t74b00 = 0u64;
        let value = c.r(84, 4);
        c.w(128, 4, value);
        t74b00 = ((2u64).wrapping_mul(4u64)) & 0xffffffffu64;
        let value = (c.r(84, 4)).wrapping_add(((t74b00 >> 0) & 0xffffffffu64));
        c.w(84, 4, value);
        let value = c.load(c.r(128, 4) as u32, 8);
        c.w(832, 8, value);
        let value = (c.r(128, 4)).wrapping_add(8u64);
        c.w(128, 4, value);
    }
    {
        // 0x60040e74 vmul.f64 d3,d2,d2
        let value = (f64::from_bits(c.r(784, 8) as u64) * f64::from_bits(c.r(784, 8) as u64))
            .to_bits() as u64;
        c.w(792, 8, value);
    }
    {
        // 0x60040e78 vfma.f64 d7,d4,d2
        let value = f64::from_bits(c.r(800, 8) as u64)
            .mul_add(
                f64::from_bits(c.r(784, 8) as u64),
                f64::from_bits(c.r(824, 8) as u64),
            )
            .to_bits() as u64;
        c.w(824, 8, value);
    }
    {
        // 0x60040e7c vfma.f64 d0,d5,d2
        let value = f64::from_bits(c.r(808, 8) as u64)
            .mul_add(
                f64::from_bits(c.r(784, 8) as u64),
                f64::from_bits(c.r(768, 8) as u64),
            )
            .to_bits() as u64;
        c.w(768, 8, value);
    }
    {
        // 0x60040e80 vfma.f64 d7,d6,d3
        let value = f64::from_bits(c.r(816, 8) as u64)
            .mul_add(
                f64::from_bits(c.r(792, 8) as u64),
                f64::from_bits(c.r(824, 8) as u64),
            )
            .to_bits() as u64;
        c.w(824, 8, value);
    }
    {
        // 0x60040e84 vfma.f64 d0,d3,d7
        let value = f64::from_bits(c.r(792, 8) as u64)
            .mul_add(
                f64::from_bits(c.r(824, 8) as u64),
                f64::from_bits(c.r(768, 8) as u64),
            )
            .to_bits() as u64;
        c.w(768, 8, value);
    }
    {
        // 0x60040e88 vcvt.f32.f64 s0,d0
        let value = (f64::from_bits(c.r(768, 8) as u64) as f32).to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x60040e8c bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x60040e8e
}
pub fn b_60040e8e(c: &mut Machine) -> u32 {
    {
        // 0x60040e8e vldr.32 s0,[pc,#0x40]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610878606u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(64u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x60040e92 bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x60040e94
}
pub fn b_60040e94(c: &mut Machine) -> u32 {
    {
        // 0x60040e94 lsls r1,r3,#0x1
        let mut t1afd00 = 0u64;
        let mut t1afe00 = 0u64;
        let mut t1b0000 = 0u64;
        let mut t1b0100 = 0u64;
        let mut t1b0200 = 0u64;
        let mut t1b0300 = 0u64;
        let mut t1b0400 = 0u64;
        let mut t1b0500 = 0u64;
        t1afd00 = ((1u64).wrapping_sub(1u64)) & 0xffffffffu64;
        t1afe00 = (if ((t1afd00 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(44, 4)) << (((t1afd00 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t1b0000 = (((t1afe00 >> 0) & 0xffffffffu64) & 2147483648u64) & 0xffffffffu64;
        t1b0100 = ((1u64 == 0u64) as u64) & 0xffu64;
        t1b0200 = (((((t1b0100 >> 0) & 0xffu64) != 0) & (c.r(98, 1) != 0)) as u64) & 0xffu64;
        t1b0300 = ((1u64 != 0u64) as u64) & 0xffu64;
        t1b0400 = ((((t1b0000 >> 0) & 0xffffffffu64) != 0u64) as u64) & 0xffu64;
        t1b0500 = (((((t1b0300 >> 0) & 0xffu64) != 0) & (((t1b0400 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        let value = ((((t1b0200 >> 0) & 0xffu64) != 0) | (((t1b0500 >> 0) & 0xffu64) != 0)) as u64;
        c.w(102, 1, value);
        let value = if 1u64 >= 32 {
            0
        } else {
            (c.r(44, 4)) << (1u64 as u32)
        };
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
    }
    {
        // 0x60040e96 beq 0x60040ec2
        let mut t180200 = 0u64;
        t180200 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        if ((t180200 >> 0) & 0xffu64) != 0 {
            return 0x60040ec2u32;
        }
    }
    0x60040e98
}
pub fn b_60040e98(c: &mut Machine) -> u32 {
    {
        // 0x60040e98 cmp.w r3,#0x7f800000
        let mut t188100 = 0u64;
        let mut t188300 = 0u64;
        let mut t1a6a00 = 0u64;
        t188100 = (2139095040u64) & 0xffffffffu64;
        t188300 = (if 31u64 >= 32 {
            0
        } else {
            ((t188100 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188300 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = (((t188100 >> 0) & 0xffffffffu64) <= c.r(44, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), ((t188100 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        t1a6a00 = ((c.r(44, 4)).wrapping_sub(((t188100 >> 0) & 0xffffffffu64))) & 0xffffffffu64;
        let value = (signed(((t1a6a00 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a6a00 >> 0) & 0xffffffffu64) == 0u64) as u64;
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
        // 0x60040e9c beq 0x60040ec8
        let mut t180200 = 0u64;
        t180200 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        if ((t180200 >> 0) & 0xffu64) != 0 {
            return 0x60040ec8u32;
        }
    }
    0x60040e9e
}
pub fn b_60040e9e(c: &mut Machine) -> u32 {
    {
        // 0x60040e9e cmp r3,#0x0
        let mut t1a6800 = 0u64;
        let value = (0u64 <= c.r(44, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), 0u64, 4, true);
        c.w(103, 1, value);
        t1a6800 = ((c.r(44, 4)).wrapping_sub(0u64)) & 0xffffffffu64;
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
        // 0x60040ea0 blt 0x60040eba
        let mut t181a00 = 0u64;
        t181a00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        if ((t181a00 >> 0) & 0xffu64) != 0 {
            return 0x60040ebau32;
        }
    }
    0x60040ea2
}
pub fn b_60040ea2(c: &mut Machine) -> u32 {
    {
        // 0x60040ea2 cmp.w r1,#0xff000000
        let mut t188100 = 0u64;
        let mut t188300 = 0u64;
        let mut t1a6a00 = 0u64;
        t188100 = (4278190080u64) & 0xffffffffu64;
        t188300 = (if 31u64 >= 32 {
            0
        } else {
            ((t188100 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188300 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = (((t188100 >> 0) & 0xffffffffu64) <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), ((t188100 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        t1a6a00 = ((c.r(36, 4)).wrapping_sub(((t188100 >> 0) & 0xffffffffu64))) & 0xffffffffu64;
        let value = (signed(((t1a6a00 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (((t1a6a00 >> 0) & 0xffffffffu64) == 0u64) as u64;
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
        // 0x60040ea6 bcs 0x60040eba
        let mut t180600 = 0u64;
        t180600 = ((c.r(98, 1) != 0u64) as u64) & 0xffu64;
        if ((t180600 >> 0) & 0xffu64) != 0 {
            return 0x60040ebau32;
        }
    }
    0x60040ea8
}
pub fn b_60040ea8(c: &mut Machine) -> u32 {
    {
        // 0x60040ea8 vldr.32 s15,[pc,#0x28]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610878632u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(40u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x60040eac vmul.f32 s15,s0,s15
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60040eb0 vmov r3,s15
        let value = c.r(828, 4);
        c.w(44, 4, value);
    }
    {
        // 0x60040eb4 sub.w r2,r3,#0xb800000
        let mut t188100 = 0u64;
        let mut t188300 = 0u64;
        t188100 = (192937984u64) & 0xffffffffu64;
        t188300 = (if 31u64 >= 32 {
            0
        } else {
            ((t188100 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188300 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = (((t188100 >> 0) & 0xffffffffu64) <= c.r(44, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), ((t188100 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_sub(((t188100 >> 0) & 0xffffffffu64));
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60040eb8 b 0x60040e1e
        return 0x60040e1eu32;
    }
    0x60040eba
}
pub fn b_60040eba(c: &mut Machine) -> u32 {
    {
        // 0x60040eba vmov s0,r3
        let value = c.r(44, 4);
        c.w(768, 4, value);
    }
    {
        // 0x60040ebe b.w 0x60041318
        return 0x60041318u32;
    }
    0x60040ec2
}
pub fn b_60040ec2(c: &mut Machine) -> u32 {
    {
        // 0x60040ec2 movs r0,#0x1
        let value = 1u64;
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
    }
    {
        // 0x60040ec4 b.w 0x600412f4
        return 0x600412f4u32;
    }
    0x60040ec8
}
pub fn b_60040ec8(c: &mut Machine) -> u32 {
    {
        // 0x60040ec8 bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x60040eca
}
pub fn b_60041294(c: &mut Machine) -> u32 {
    {
        // 0x60041294 push {r4,lr}
        let value = c.r(84, 4);
        c.w(128, 4, value);
        let value = (c.r(128, 4)).wrapping_sub(4u64);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 4, c.r(88, 4));
        let value = (c.r(128, 4)).wrapping_sub(4u64);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 4, c.r(48, 4));
        let value = c.r(128, 4);
        c.w(84, 4, value);
    }
    {
        // 0x60041296 vpush {d8}
        let mut t73700 = 0u64;
        t73700 = ((2u64).wrapping_mul(4u64)) & 0xffffffffu64;
        let value = (c.r(84, 4)).wrapping_sub(((t73700 >> 0) & 0xffffffffu64));
        c.w(84, 4, value);
        let value = c.r(84, 4);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 8, c.r(832, 8));
        let value = (c.r(128, 4)).wrapping_add(8u64);
        c.w(128, 4, value);
    }
    {
        // 0x6004129a vmov.f32 s16,s0
        let value = c.r(768, 4);
        c.w(832, 4, value);
    }
    {
        // 0x6004129e mov r4,r0
        let value = c.r(32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x600412a0 bl 0x600419c4
        let value = (1610879652u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600419c4u32;
    }
    0x600412a4
}
