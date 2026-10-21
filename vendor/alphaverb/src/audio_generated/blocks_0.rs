#![allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut,
    unused_comparisons
)]
use crate::machine::{signed, signed_overflow, Machine};
pub fn b_200(c: &mut Machine) -> u32 {
    {
        // 0x00000200 push {r4,r5,r6,r7,r8,r9,r10,r11,lr}
        let value = (c.r(84, 4)).wrapping_sub(4u64);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 4, c.r(88, 4));
        let value = (c.r(128, 4)).wrapping_sub(4u64);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 4, c.r(76, 4));
        let value = (c.r(128, 4)).wrapping_sub(4u64);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 4, c.r(72, 4));
        let value = (c.r(128, 4)).wrapping_sub(4u64);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 4, c.r(68, 4));
        let value = (c.r(128, 4)).wrapping_sub(4u64);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 4, c.r(64, 4));
        let value = (c.r(128, 4)).wrapping_sub(4u64);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 4, c.r(60, 4));
        let value = (c.r(128, 4)).wrapping_sub(4u64);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 4, c.r(56, 4));
        let value = (c.r(128, 4)).wrapping_sub(4u64);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 4, c.r(52, 4));
        let value = (c.r(128, 4)).wrapping_sub(4u64);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 4, c.r(48, 4));
        let value = (c.r(128, 4)).wrapping_sub(4u64);
        c.w(128, 4, value);
        let value = (c.r(128, 4)).wrapping_add(4u64);
        c.w(84, 4, value);
    }
    {
        // 0x00000204 vpush {d8,d9,d10,d11,d12,d13,d14,d15}
        let mut t73700 = 0u64;
        t73700 = ((16u64).wrapping_mul(4u64)) & 0xffffffffu64;
        let value = (c.r(84, 4)).wrapping_sub(((t73700 >> 0) & 0xffffffffu64));
        c.w(84, 4, value);
        let value = c.r(84, 4);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 8, c.r(832, 8));
        let value = (c.r(128, 4)).wrapping_add(8u64);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 8, c.r(840, 8));
        let value = (c.r(128, 4)).wrapping_add(8u64);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 8, c.r(848, 8));
        let value = (c.r(128, 4)).wrapping_add(8u64);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 8, c.r(856, 8));
        let value = (c.r(128, 4)).wrapping_add(8u64);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 8, c.r(864, 8));
        let value = (c.r(128, 4)).wrapping_add(8u64);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 8, c.r(872, 8));
        let value = (c.r(128, 4)).wrapping_add(8u64);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 8, c.r(880, 8));
        let value = (c.r(128, 4)).wrapping_add(8u64);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 8, c.r(888, 8));
        let value = (c.r(128, 4)).wrapping_add(8u64);
        c.w(128, 4, value);
    }
    {
        // 0x00000208 sub sp,#0x54
        let mut t187500 = 0u64;
        t187500 = (84u64) & 0xffffffffu64;
        let value = (c.r(84, 4)).wrapping_sub(((t187500 >> 0) & 0xffffffffu64));
        c.w(84, 4, value);
    }
    {
        // 0x0000020a str r2,[sp,#0x34]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x0000020c cmp r2,#0x0
        let mut t1a6800 = 0u64;
        let value = (0u64 <= c.r(40, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(40, 4), 0u64, 4, true);
        c.w(103, 1, value);
        t1a6800 = ((c.r(40, 4)).wrapping_sub(0u64)) & 0xffffffffu64;
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
        // 0x0000020e beq.w 0x00003138
        let mut t182300 = 0u64;
        t182300 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        if ((t182300 >> 0) & 0xffu64) != 0 {
            return 0x3138u32;
        }
    }
    0x212
}
pub fn b_212(c: &mut Machine) -> u32 {
    {
        // 0x00000212 adds r3,r0,#0x4
        let value = ((c.r(32, 4) as u128 + 4u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 4u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(4u64);
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
        // 0x00000214 ldr r5,[0x00000240]
        let value = c.load(0x240, 4);
        c.w(52, 4, value);
    }
    {
        // 0x00000216 vldr.32 s23,[pc,#0x2c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((534u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(44u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(860, 4, value);
    }
    {
        // 0x0000021a str r3,[sp,#0x28]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x0000021c adds r3,r1,#0x4
        let value = ((c.r(36, 4) as u128 + 4u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 4u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_add(4u64);
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
        // 0x0000021e vldr.32 s22,[pc,#0x28]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((542u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(40u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(856, 4, value);
    }
    {
        // 0x00000222 sub.w r6,r5,#0x1000
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (4096u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = (((t188500 >> 0) & 0xffffffffu64) <= c.r(52, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(52, 4), ((t188500 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(52, 4)).wrapping_sub(((t188500 >> 0) & 0xffffffffu64));
        c.w(56, 4, value);
        let value = (signed(c.r(56, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(56, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00000226 str r3,[sp,#0x30]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00000228 movs r3,#0x0
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
        // 0x0000022a vldr.32 s21,[pc,#0x20]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((554u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(32u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(852, 4, value);
    }
    {
        // 0x0000022e vldr.32 s20,[pc,#0x20]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((558u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(32u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(848, 4, value);
    }
    {
        // 0x00000232 vldr.32 s19,[pc,#0x20]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((562u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(32u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(844, 4, value);
    }
    {
        // 0x00000236 vldr.32 s18,[pc,#0x20]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((566u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(32u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(840, 4, value);
    }
    {
        // 0x0000023a str r3,[sp,#0x2c]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x0000023c b 0x000008a8
        return 0x8a8u32;
    }
    0x23e
}
pub fn b_274(c: &mut Machine) -> u32 {
    {
        // 0x00000274 vldr.32 s15,[pc,#-0x14]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((628u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(20u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00000278 vcmpe.f32 s14,s15
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
        // 0x0000027c vmrs apsr,fpscr
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
        // 0x00000280 bmi.w 0x00003142
        let mut t182b00 = 0u64;
        t182b00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        if ((t182b00 >> 0) & 0xffu64) != 0 {
            return 0x3142u32;
        }
    }
    0x284
}
pub fn b_284(c: &mut Machine) -> u32 {
    {
        // 0x00000284 vldr.32 s13,[pc,#-0x20]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((644u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(32u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00000288 vldr.32 s8,[pc,#-0x30]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((648u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(48u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    0x28c
}
pub fn b_28c(c: &mut Machine) -> u32 {
    {
        // 0x0000028c vmul.f32 s15,s10,s22
        let value = (f32::from_bits(c.r(808, 4) as u32) * f32::from_bits(c.r(856, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00000290 vstr.32 s10,[r5,#0x238]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(568u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(808, 4));
    }
    {
        // 0x00000294 vmov.f32 s9,0x3f800000
        let value = 1065353216u64;
        c.w(804, 4, value);
    }
    {
        // 0x00000298 vstr.32 s13,[r5,#0x224]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(548u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x0000029c vldr.32 s11,[r5,#0x234]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(564u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x000002a0 vcvt.s32.f32 s14,s15
        let value = (f32::from_bits(c.r(828, 4) as u32) as i32) as u64;
        c.w(824, 4, value);
    }
    {
        // 0x000002a4 vldr.32 s6,[r5,#0x230]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(560u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x000002a8 vsub.f32 s12,s12,s9
        let value = (f32::from_bits(c.r(816, 4) as u32) - f32::from_bits(c.r(804, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x000002ac vldr.32 s7,[pc,#-0x50]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((684u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(80u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x000002b0 vcvt.f32.s32 s10,s14
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
        c.w(808, 4, value);
    }
    {
        // 0x000002b4 vmov r3,s14
        let value = c.r(824, 4);
        c.w(44, 4, value);
    }
    {
        // 0x000002b8 vmov.f32 s14,s9
        let value = c.r(804, 4);
        c.w(824, 4, value);
    }
    {
        // 0x000002bc vfma.f32 s13,s12,s8
        let value = f32::from_bits(c.r(816, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(800, 4) as u32),
                f32::from_bits(c.r(820, 4) as u32),
            )
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x000002c0 adds r3,#0x7f
        let value = ((c.r(44, 4) as u128 + 127u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), 127u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_add(127u64);
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
        // 0x000002c2 vsub.f32 s15,s15,s10
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x000002c6 vfma.f32 s14,s12,s8
        let value = f32::from_bits(c.r(816, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(800, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x000002ca lsls r3,r3,#0x17
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
            (c.r(44, 4)) << (((t1afd00 >> 0) & 0xffffffffu64) as u32)
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
            (c.r(44, 4)) << (23u64 as u32)
        };
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
        // 0x000002cc vmov.f32 s12,s20
        let value = c.r(848, 4);
        c.w(816, 4, value);
    }
    {
        // 0x000002d0 vfma.f32 s12,s15,s21
        let value = f32::from_bits(c.r(828, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(852, 4) as u32),
                f32::from_bits(c.r(816, 4) as u32),
            )
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x000002d4 vstr.32 s13,[r5,#0x22c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(556u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x000002d8 vmov.f32 s13,s18
        let value = c.r(840, 4);
        c.w(820, 4, value);
    }
    {
        // 0x000002dc vstr.32 s14,[r5,#0x228]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(552u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x000002e0 vmov.f32 s14,s19
        let value = c.r(844, 4);
        c.w(824, 4, value);
    }
    {
        // 0x000002e4 vfma.f32 s14,s15,s12
        let value = f32::from_bits(c.r(828, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(816, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x000002e8 vfma.f32 s13,s15,s14
        let value = f32::from_bits(c.r(828, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(824, 4) as u32),
                f32::from_bits(c.r(820, 4) as u32),
            )
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x000002ec vmov.f32 s14,s9
        let value = c.r(804, 4);
        c.w(824, 4, value);
    }
    {
        // 0x000002f0 vfma.f32 s14,s15,s13
        let value = f32::from_bits(c.r(828, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(820, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x000002f4 vmov s15,r3
        let value = c.r(44, 4);
        c.w(828, 4, value);
    }
    {
        // 0x000002f8 vmul.f32 s14,s14,s15
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x000002fc vmul.f32 s15,s14,s11
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00000300 vmul.f32 s14,s14,s14
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00000304 vmul.f32 s15,s15,s6
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(792, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00000308 vcmpe.f32 s15,s7
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
        t35c00 = ((f32::from_bits(c.r(828, 4) as u32) < f32::from_bits(c.r(796, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(828, 4) as u32) == f32::from_bits(c.r(796, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(796, 4) as u32) <= f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(796, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x0000030c vmrs apsr,fpscr
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
        // 0x00000310 ble.w 0x000009a6
        let mut t184000 = 0u64;
        let mut t184200 = 0u64;
        t184000 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t184200 = (((c.r(97, 1) != 0) | (((t184000 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t184200 >> 0) & 0xffu64) != 0 {
            return 0x9a6u32;
        }
    }
    0x314
}
pub fn b_314(c: &mut Machine) -> u32 {
    {
        // 0x00000314 vldr.32 s13,[pc,#-0xb4]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((788u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(180u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00000318 vcmpe.f32 s15,s13
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
        // 0x0000031c vmrs apsr,fpscr
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
        // 0x00000320 bmi.w 0x00003172
        let mut t182b00 = 0u64;
        t182b00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        if ((t182b00 >> 0) & 0xffu64) != 0 {
            return 0x3172u32;
        }
    }
    0x324
}
pub fn b_324(c: &mut Machine) -> u32 {
    {
        // 0x00000324 vldr.32 s13,[pc,#-0xc0]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((804u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(192u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00000328 vldr.32 s10,[pc,#-0xc0]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((808u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(192u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    0x32c
}
pub fn b_32c(c: &mut Machine) -> u32 {
    {
        // 0x0000032c vmov.f32 s12,0x3f800000
        let value = 1065353216u64;
        c.w(816, 4, value);
    }
    {
        // 0x00000330 vldr.32 s0,[r5,#0x268]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(616u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x00000334 vldr.32 s11,[r5,#0x25c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(604u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x00000338 vmov.f32 s9,0x3f000000
        let value = 1056964608u64;
        c.w(804, 4, value);
    }
    {
        // 0x0000033c vldr.32 s29,[r5,#0x274]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(628u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(884, 4, value);
    }
    {
        // 0x00000340 vsub.f32 s14,s14,s12
        let value = (f32::from_bits(c.r(824, 4) as u32) - f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00000344 vldr.32 s15,[r5,#0x260]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(608u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00000348 vadd.f32 s0,s0,s11
        let value = (f32::from_bits(c.r(768, 4) as u32) + f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x0000034c vldr.32 s8,[r5,#0x270]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(624u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x00000350 vsub.f32 s15,s15,s29
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(884, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00000354 vstr.32 s13,[r5,#0x23c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(572u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x00000358 vdiv.f32 s11,s14,s10
        let value = (f32::from_bits(c.r(824, 4) as u32) / f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x0000035c vcmpe.f32 s0,s9
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
        t35c00 = ((f32::from_bits(c.r(768, 4) as u32) < f32::from_bits(c.r(804, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(768, 4) as u32) == f32::from_bits(c.r(804, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(804, 4) as u32) <= f32::from_bits(c.r(768, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(768, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x00000360 vfma.f32 s29,s15,s8
        let value = f32::from_bits(c.r(828, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(800, 4) as u32),
                f32::from_bits(c.r(884, 4) as u32),
            )
            .to_bits() as u64;
        c.w(884, 4, value);
    }
    {
        // 0x00000364 vmrs apsr,fpscr
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
        // 0x00000368 it gt
        let mut t185d00 = 0u64;
        let mut t185e00 = 0u64;
        let mut t186000 = 0u64;
        t185d00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t185e00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t186000 = (((((t185d00 >> 0) & 0xffu64) != 0) & (((t185e00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
    }
    {
        // 0x0000036a vsub.gt.f32 s0,s0,s12
        let mut t1e00 = 0u64;
        let mut t1f00 = 0u64;
        let mut t2100 = 0u64;
        let mut t2700 = 0u64;
        t1e00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t1f00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t2100 =
            (((((t1e00 >> 0) & 0xffu64) != 0) & (((t1f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        t2700 = ((((t2100 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x36eu32;
        }
        let value = (f32::from_bits(c.r(768, 4) as u32) - f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    0x36e
}
pub fn b_36e(c: &mut Machine) -> u32 {
    {
        // 0x0000036e vadd.f32 s29,s29,s23
        let value = (f32::from_bits(c.r(884, 4) as u32) + f32::from_bits(c.r(860, 4) as u32))
            .to_bits() as u64;
        c.w(884, 4, value);
    }
    {
        // 0x00000372 vstr.32 s0,[r5,#0x268]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(616u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x00000376 vstr.32 s29,[r5,#0x274]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(628u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(884, 4));
    }
    {
        // 0x0000037a vadd.f32 s15,s11,s12
        let value = (f32::from_bits(c.r(812, 4) as u32) + f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x0000037e vsub.f32 s13,s13,s11
        let value = (f32::from_bits(c.r(820, 4) as u32) - f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00000382 vstr.32 s15,[r5,#0x240]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(576u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x00000386 vstr.32 s13,[r5,#0x244]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(580u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x0000038a bl 0x00003420
        let value = (910u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x3420u32;
    }
    0x38e
}
pub fn b_38e(c: &mut Machine) -> u32 {
    {
        // 0x0000038e vmov.f32 s14,0x3e000000
        let value = 1040187392u64;
        c.w(824, 4, value);
    }
    {
        // 0x00000392 vldr.32 s27,[r5,#0x268]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(616u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(876, 4, value);
    }
    {
        // 0x00000396 vmov.f32 s15,0x3f000000
        let value = 1056964608u64;
        c.w(828, 4, value);
    }
    {
        // 0x0000039a vmov.f32 s13,0x3e800000
        let value = 1048576000u64;
        c.w(820, 4, value);
    }
    {
        // 0x0000039e vadd.f32 s27,s27,s14
        let value = (f32::from_bits(c.r(876, 4) as u32) + f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(876, 4, value);
    }
    {
        // 0x000003a2 vsub.f32 s30,s0,s13
        let value = (f32::from_bits(c.r(768, 4) as u32) - f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(888, 4, value);
    }
    {
        // 0x000003a6 vcmpe.f32 s27,s15
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
        t35c00 = ((f32::from_bits(c.r(876, 4) as u32) < f32::from_bits(c.r(828, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(876, 4) as u32) == f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(828, 4) as u32) <= f32::from_bits(c.r(876, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(876, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x000003aa vmul.f32 s30,s30,s29
        let value = (f32::from_bits(c.r(888, 4) as u32) * f32::from_bits(c.r(884, 4) as u32))
            .to_bits() as u64;
        c.w(888, 4, value);
    }
    {
        // 0x000003ae vmrs apsr,fpscr
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
        // 0x000003b2 ble 0x000003bc
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x3bcu32;
        }
    }
    0x3b4
}
pub fn b_3b4(c: &mut Machine) -> u32 {
    {
        // 0x000003b4 vmov.f32 s15,0x3f800000
        let value = 1065353216u64;
        c.w(828, 4, value);
    }
    {
        // 0x000003b8 vsub.f32 s27,s27,s15
        let value = (f32::from_bits(c.r(876, 4) as u32) - f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(876, 4, value);
    }
    0x3bc
}
pub fn b_3bc(c: &mut Machine) -> u32 {
    {
        // 0x000003bc vmov.f32 s0,s27
        let value = c.r(876, 4);
        c.w(768, 4, value);
    }
    {
        // 0x000003c0 bl 0x00003420
        let value = (964u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x3420u32;
    }
    0x3c4
}
pub fn b_3c4(c: &mut Machine) -> u32 {
    {
        // 0x000003c4 vmov.f32 s14,0x3e000000
        let value = 1040187392u64;
        c.w(824, 4, value);
    }
    {
        // 0x000003c8 vmov.f32 s15,0x3f000000
        let value = 1056964608u64;
        c.w(828, 4, value);
    }
    {
        // 0x000003cc vmov.f32 s13,0x3e800000
        let value = 1048576000u64;
        c.w(820, 4, value);
    }
    {
        // 0x000003d0 vadd.f32 s27,s27,s14
        let value = (f32::from_bits(c.r(876, 4) as u32) + f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(876, 4, value);
    }
    {
        // 0x000003d4 vsub.f32 s31,s0,s13
        let value = (f32::from_bits(c.r(768, 4) as u32) - f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(892, 4, value);
    }
    {
        // 0x000003d8 vcmpe.f32 s27,s15
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
        t35c00 = ((f32::from_bits(c.r(876, 4) as u32) < f32::from_bits(c.r(828, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(876, 4) as u32) == f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(828, 4) as u32) <= f32::from_bits(c.r(876, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(876, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x000003dc vmul.f32 s31,s31,s29
        let value = (f32::from_bits(c.r(892, 4) as u32) * f32::from_bits(c.r(884, 4) as u32))
            .to_bits() as u64;
        c.w(892, 4, value);
    }
    {
        // 0x000003e0 vmrs apsr,fpscr
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
        // 0x000003e4 ble 0x000003ee
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x3eeu32;
        }
    }
    0x3e6
}
pub fn b_3e6(c: &mut Machine) -> u32 {
    {
        // 0x000003e6 vmov.f32 s15,0x3f800000
        let value = 1065353216u64;
        c.w(828, 4, value);
    }
    {
        // 0x000003ea vsub.f32 s27,s27,s15
        let value = (f32::from_bits(c.r(876, 4) as u32) - f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(876, 4, value);
    }
    0x3ee
}
pub fn b_3ee(c: &mut Machine) -> u32 {
    {
        // 0x000003ee vmov.f32 s0,s27
        let value = c.r(876, 4);
        c.w(768, 4, value);
    }
    {
        // 0x000003f2 bl 0x00003420
        let value = (1014u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x3420u32;
    }
    0x3f6
}
pub fn b_3f6(c: &mut Machine) -> u32 {
    {
        // 0x000003f6 vmov.f32 s15,0x3e800000
        let value = 1048576000u64;
        c.w(828, 4, value);
    }
    {
        // 0x000003fa vmov.f32 s13,0x3e000000
        let value = 1040187392u64;
        c.w(820, 4, value);
    }
    {
        // 0x000003fe vmov.f32 s14,0x3f000000
        let value = 1056964608u64;
        c.w(824, 4, value);
    }
    {
        // 0x00000402 vsub.f32 s15,s0,s15
        let value = (f32::from_bits(c.r(768, 4) as u32) - f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00000406 vadd.f32 s0,s27,s13
        let value = (f32::from_bits(c.r(876, 4) as u32) + f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x0000040a vmul.f32 s27,s15,s29
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(884, 4) as u32))
            .to_bits() as u64;
        c.w(876, 4, value);
    }
    {
        // 0x0000040e vcmpe.f32 s0,s14
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
        // 0x00000412 vmrs apsr,fpscr
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
        // 0x00000416 ble 0x00000420
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x420u32;
        }
    }
    0x418
}
pub fn b_418(c: &mut Machine) -> u32 {
    {
        // 0x00000418 vmov.f32 s15,0x3f800000
        let value = 1065353216u64;
        c.w(828, 4, value);
    }
    {
        // 0x0000041c vsub.f32 s0,s0,s15
        let value = (f32::from_bits(c.r(768, 4) as u32) - f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    0x420
}
pub fn b_420(c: &mut Machine) -> u32 {
    {
        // 0x00000420 bl 0x00003420
        let value = (1060u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x3420u32;
    }
    0x424
}
pub fn b_424(c: &mut Machine) -> u32 {
    {
        // 0x00000424 vmov.f32 s15,0x3e800000
        let value = 1048576000u64;
        c.w(828, 4, value);
    }
    {
        // 0x00000428 ldr.w r3,[r5,#0x378]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(888u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x0000042c vsub.f32 s0,s0,s15
        let value = (f32::from_bits(c.r(768, 4) as u32) - f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x00000430 vmul.f32 s0,s0,s29
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(884, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x00000434 cmp r3,#0x0
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
        // 0x00000436 bne.w 0x000009b0
        let mut t182500 = 0u64;
        t182500 = ((c.r(97, 1) == 0u64) as u64) & 0xffu64;
        if ((t182500 >> 0) & 0xffu64) != 0 {
            return 0x9b0u32;
        }
    }
    0x43a
}
pub fn b_43a(c: &mut Machine) -> u32 {
    {
        // 0x0000043a vldr.32 s14,[r5,#0x34c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(844u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x0000043e vldr.32 s13,[pc,#-0x1d0]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((1086u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(464u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00000442 vadd.f32 s14,s30,s14
        let value = (f32::from_bits(c.r(888, 4) as u32) + f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00000446 ldr r3,[0x0000071c]
        let value = c.load(0x71c, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00000448 vldr.32 s29,[r5,#0x350]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(848u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(884, 4, value);
    }
    {
        // 0x0000044c vmov.f32 s9,s13
        let value = c.r(820, 4);
        c.w(804, 4, value);
    }
    {
        // 0x00000450 vldr.32 s15,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00000454 vmov.f32 s24,s13
        let value = c.r(820, 4);
        c.w(864, 4, value);
    }
    {
        // 0x00000458 vcmpe.f32 s14,s13
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
        t35c00 = ((f32::from_bits(c.r(824, 4) as u32) < f32::from_bits(c.r(820, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(824, 4) as u32) == f32::from_bits(c.r(820, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(820, 4) as u32) <= f32::from_bits(c.r(824, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(824, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x0000045c vldr.32 s4,[r3,#0xb0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(176u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(784, 4, value);
    }
    {
        // 0x00000460 vsub.f32 s29,s29,s30
        let value = (f32::from_bits(c.r(884, 4) as u32) - f32::from_bits(c.r(888, 4) as u32))
            .to_bits() as u64;
        c.w(884, 4, value);
    }
    {
        // 0x00000464 vldr.32 s10,[r5,#0x354]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(852u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x00000468 vldr.32 s5,[r3,#0x160]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(352u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(788, 4, value);
    }
    {
        // 0x0000046c add.w r1,r3,#0x214
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (532u64) & 0xffffffffu64;
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
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00000470 vmrs apsr,fpscr
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
        // 0x00000474 vldr.32 s11,[r5,#0x358]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(856u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x00000478 vadd.f32 s10,s31,s10
        let value = (f32::from_bits(c.r(892, 4) as u32) + f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x0000047c vldr.32 s6,[r3,#0x210]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(528u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x00000480 vsub.f32 s11,s11,s31
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(892, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00000484 vldr.32 s12,[r5,#0x35c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(860u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x00000488 vselgt.f32 s14,s14,s13
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(820, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(824, 4, value);
    }
    {
        // 0x0000048c vcmpe.f32 s15,s14
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
        // 0x00000490 vadd.f32 s12,s27,s12
        let value = (f32::from_bits(c.r(876, 4) as u32) + f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00000494 vldr.32 s7,[r3,#0x2c0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(704u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x00000498 vldr.32 s8,[r3,#0x370]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(880u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x0000049c add.w r2,r3,#0x374
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (884u64) & 0xffffffffu64;
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
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x000004a0 vmrs apsr,fpscr
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
        // 0x000004a4 add.w r4,r3,#0xb4
        let mut t187d00 = 0u64;
        t187d00 = (180u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = ((c.r(44, 4) as u128 + ((t187d00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_add(((t187d00 >> 0) & 0xffffffffu64));
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x000004a8 vcmpe.f32 s29,s13
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
        t35c00 = ((f32::from_bits(c.r(884, 4) as u32) < f32::from_bits(c.r(820, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(884, 4) as u32) == f32::from_bits(c.r(820, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(820, 4) as u32) <= f32::from_bits(c.r(884, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(884, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x000004ac vldr.32 s13,[r5,#0x360]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(864u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x000004b0 add.w r0,r3,#0x2c4
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (708u64) & 0xffffffffu64;
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
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x000004b4 add.w r7,r3,#0x164
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (356u64) & 0xffffffffu64;
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
        c.w(60, 4, value);
        let value = (signed(c.r(60, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(60, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x000004b8 vselgt.f32 s15,s14,s15
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
        c.w(828, 4, value);
    }
    {
        // 0x000004bc vsub.f32 s13,s13,s27
        let value = (f32::from_bits(c.r(820, 4) as u32) - f32::from_bits(c.r(876, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x000004c0 vmrs apsr,fpscr
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
        // 0x000004c4 vldr.32 s14,[r5,#0x364]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(868u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x000004c8 vstr.32 s15,[r3,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x000004cc addw r3,r3,#0x424
        let value = ((c.r(44, 4) as u128 + 1060u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), 1060u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_add(1060u64);
        c.w(44, 4, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x000004d0 vadd.f32 s14,s0,s14
        let value = (f32::from_bits(c.r(768, 4) as u32) + f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x000004d4 vldr.32 s15,[r5,#0x368]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(872u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x000004d8 vselgt.f32 s29,s29,s9
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
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(884, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(804, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(884, 4, value);
    }
    {
        // 0x000004dc vcmpe.f32 s4,s29
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
        t35c00 = ((f32::from_bits(c.r(784, 4) as u32) < f32::from_bits(c.r(884, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(784, 4) as u32) == f32::from_bits(c.r(884, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(884, 4) as u32) <= f32::from_bits(c.r(784, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(784, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(884, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x000004e0 vldr.32 s9,[r1,#0x20c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(524u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(804, 4, value);
    }
    {
        // 0x000004e4 vsub.f32 s15,s15,s0
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x000004e8 vmul.f32 s0,s17,s26
        let value = (f32::from_bits(c.r(836, 4) as u32) * f32::from_bits(c.r(872, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x000004ec vmrs apsr,fpscr
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
        // 0x000004f0 vcmpe.f32 s10,s24
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
        t35c00 = ((f32::from_bits(c.r(808, 4) as u32) < f32::from_bits(c.r(864, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(808, 4) as u32) == f32::from_bits(c.r(864, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(864, 4) as u32) <= f32::from_bits(c.r(808, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(808, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(864, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x000004f4 vselgt.f32 s4,s29,s4
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
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(884, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(784, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(784, 4, value);
    }
    {
        // 0x000004f8 vstr.32 s4,[r4]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(48, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(784, 4));
    }
    {
        // 0x000004fc vmrs apsr,fpscr
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
        // 0x00000500 vldr.32 s4,[r3,#0xac]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(172u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(784, 4, value);
    }
    {
        // 0x00000504 add.w r4,r4,#0x420
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1056u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(48, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00000508 vselgt.f32 s10,s10,s24
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(864, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(808, 4, value);
    }
    {
        // 0x0000050c vcmpe.f32 s5,s10
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
        t35c00 = ((f32::from_bits(c.r(788, 4) as u32) < f32::from_bits(c.r(808, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(788, 4) as u32) == f32::from_bits(c.r(808, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(808, 4) as u32) <= f32::from_bits(c.r(788, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(788, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x00000510 vmrs apsr,fpscr
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
        // 0x00000514 vcmpe.f32 s11,s24
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
        t35c00 = ((f32::from_bits(c.r(812, 4) as u32) < f32::from_bits(c.r(864, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(812, 4) as u32) == f32::from_bits(c.r(864, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(864, 4) as u32) <= f32::from_bits(c.r(812, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(812, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(864, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x00000518 vselgt.f32 s5,s10,s5
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(788, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(788, 4, value);
    }
    {
        // 0x0000051c vstr.32 s5,[r7]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(60, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(788, 4));
    }
    {
        // 0x00000520 vmrs apsr,fpscr
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
        // 0x00000524 vselgt.f32 s11,s11,s24
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(864, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(812, 4, value);
    }
    {
        // 0x00000528 vcmpe.f32 s6,s11
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
        t35c00 = ((f32::from_bits(c.r(792, 4) as u32) < f32::from_bits(c.r(812, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(792, 4) as u32) == f32::from_bits(c.r(812, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(812, 4) as u32) <= f32::from_bits(c.r(792, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(792, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(812, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x0000052c vmrs apsr,fpscr
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
        // 0x00000530 vcmpe.f32 s12,s24
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
        t35c00 = ((f32::from_bits(c.r(816, 4) as u32) < f32::from_bits(c.r(864, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(816, 4) as u32) == f32::from_bits(c.r(864, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(864, 4) as u32) <= f32::from_bits(c.r(816, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(816, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(864, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x00000534 vselgt.f32 s6,s11,s6
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(792, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(792, 4, value);
    }
    {
        // 0x00000538 vstr.32 s6,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(792, 4));
    }
    {
        // 0x0000053c vmrs apsr,fpscr
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
        // 0x00000540 add r1,sp,#0x40
        let mut t187400 = 0u64;
        t187400 = ((c.r(84, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        let value = ((t187400 >> 0) & 0xffffffffu64);
        c.w(36, 4, value);
    }
    {
        // 0x00000542 vselgt.f32 s12,s12,s24
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(864, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(816, 4, value);
    }
    {
        // 0x00000546 vcmpe.f32 s7,s12
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
        t35c00 = ((f32::from_bits(c.r(796, 4) as u32) < f32::from_bits(c.r(816, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(796, 4) as u32) == f32::from_bits(c.r(816, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(816, 4) as u32) <= f32::from_bits(c.r(796, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(796, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x0000054a vmrs apsr,fpscr
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
        // 0x0000054e vcmpe.f32 s13,s24
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
        t35c00 = ((f32::from_bits(c.r(820, 4) as u32) < f32::from_bits(c.r(864, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(820, 4) as u32) == f32::from_bits(c.r(864, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(864, 4) as u32) <= f32::from_bits(c.r(820, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(820, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(864, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x00000552 vselgt.f32 s7,s12,s7
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(796, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(796, 4, value);
    }
    {
        // 0x00000556 vstr.32 s7,[r0]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(32, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(796, 4));
    }
    {
        // 0x0000055a vmrs apsr,fpscr
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
        // 0x0000055e subw r0,r4,#0xa58
        let value = (2648u64 <= c.r(48, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 2648u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_sub(2648u64);
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00000562 vselgt.f32 s13,s13,s24
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(864, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(820, 4, value);
    }
    {
        // 0x00000566 vcmpe.f32 s8,s13
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
        t35c00 = ((f32::from_bits(c.r(800, 4) as u32) < f32::from_bits(c.r(820, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(800, 4) as u32) == f32::from_bits(c.r(820, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(820, 4) as u32) <= f32::from_bits(c.r(800, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(800, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x0000056a vmrs apsr,fpscr
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
        // 0x0000056e vcmpe.f32 s14,s24
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
        t35c00 = ((f32::from_bits(c.r(824, 4) as u32) < f32::from_bits(c.r(864, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(824, 4) as u32) == f32::from_bits(c.r(864, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(864, 4) as u32) <= f32::from_bits(c.r(824, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(824, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(864, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x00000572 vselgt.f32 s8,s13,s8
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(800, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(800, 4, value);
    }
    {
        // 0x00000576 vstr.32 s8,[r2]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(40, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(800, 4));
    }
    {
        // 0x0000057a vmrs apsr,fpscr
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
        // 0x0000057e vselgt.f32 s14,s14,s24
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(864, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(824, 4, value);
    }
    {
        // 0x00000582 vcmpe.f32 s9,s14
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
        t35c00 = ((f32::from_bits(c.r(804, 4) as u32) < f32::from_bits(c.r(824, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(804, 4) as u32) == f32::from_bits(c.r(824, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(824, 4) as u32) <= f32::from_bits(c.r(804, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(804, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x00000586 vmrs apsr,fpscr
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
        // 0x0000058a vcmpe.f32 s15,s24
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
        t35c00 = ((f32::from_bits(c.r(828, 4) as u32) < f32::from_bits(c.r(864, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(828, 4) as u32) == f32::from_bits(c.r(864, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(864, 4) as u32) <= f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(864, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x0000058e vselgt.f32 s9,s14,s9
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(804, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(804, 4, value);
    }
    {
        // 0x00000592 vstr.32 s9,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(804, 4));
    }
    {
        // 0x00000596 vmrs apsr,fpscr
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
        // 0x0000059a vselgt.f32 s15,s15,s24
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(864, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(828, 4, value);
    }
    {
        // 0x0000059e vcmpe.f32 s4,s15
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
        t35c00 = ((f32::from_bits(c.r(784, 4) as u32) < f32::from_bits(c.r(828, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(784, 4) as u32) == f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(828, 4) as u32) <= f32::from_bits(c.r(784, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(784, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x000005a2 vmrs apsr,fpscr
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
        // 0x000005a6 vselgt.f32 s4,s15,s4
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(784, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(784, 4, value);
    }
    {
        // 0x000005aa vstr.32 s4,[r4]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(48, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(784, 4));
    }
    {
        // 0x000005ae bl 0x000033c8
        let value = (1458u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x33c8u32;
    }
    0x5b2
}
pub fn b_5b2(c: &mut Machine) -> u32 {
    {
        // 0x000005b2 vldr.32 s0,[sp,#0x40]
        let mut t64000 = 0u64;
        t64000 = ((c.r(84, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x000005b6 add r1,sp,#0x40
        let mut t187400 = 0u64;
        t187400 = ((c.r(84, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        let value = ((t187400 >> 0) & 0xffffffffu64);
        c.w(36, 4, value);
    }
    {
        // 0x000005b8 subw r0,r4,#0x8f8
        let value = (2296u64 <= c.r(48, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 2296u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_sub(2296u64);
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x000005bc bl 0x000033f8
        let value = (1472u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x33f8u32;
    }
    0x5c0
}
pub fn b_5c0(c: &mut Machine) -> u32 {
    {
        // 0x000005c0 vldr.32 s0,[sp,#0x40]
        let mut t64000 = 0u64;
        t64000 = ((c.r(84, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x000005c4 add r1,sp,#0x40
        let mut t187400 = 0u64;
        t187400 = ((c.r(84, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        let value = ((t187400 >> 0) & 0xffffffffu64);
        c.w(36, 4, value);
    }
    {
        // 0x000005c6 sub.w r0,r4,#0x798
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1944u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = (((t188500 >> 0) & 0xffffffffu64) <= c.r(48, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t188500 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_sub(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x000005ca bl 0x000033f8
        let value = (1486u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x33f8u32;
    }
    0x5ce
}
pub fn b_5ce(c: &mut Machine) -> u32 {
    {
        // 0x000005ce vldr.32 s0,[sp,#0x40]
        let mut t64000 = 0u64;
        t64000 = ((c.r(84, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x000005d2 add r1,sp,#0x48
        let mut t187400 = 0u64;
        t187400 = ((c.r(84, 4)).wrapping_add(72u64)) & 0xffffffffu64;
        let value = ((t187400 >> 0) & 0xffffffffu64);
        c.w(36, 4, value);
    }
    {
        // 0x000005d4 sub.w r0,r4,#0x638
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1592u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = (((t188500 >> 0) & 0xffffffffu64) <= c.r(48, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t188500 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_sub(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x000005d8 bl 0x000033f8
        let value = (1500u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x33f8u32;
    }
    0x5dc
}
pub fn b_5dc(c: &mut Machine) -> u32 {
    {
        // 0x000005dc vmul.f32 s0,s16,s26
        let value = (f32::from_bits(c.r(832, 4) as u32) * f32::from_bits(c.r(872, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x000005e0 add r1,sp,#0x44
        let mut t187400 = 0u64;
        t187400 = ((c.r(84, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        let value = ((t187400 >> 0) & 0xffffffffu64);
        c.w(36, 4, value);
    }
    {
        // 0x000005e2 subw r0,r4,#0x9a8
        let value = (2472u64 <= c.r(48, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 2472u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_sub(2472u64);
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x000005e6 bl 0x000033c8
        let value = (1514u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x33c8u32;
    }
    0x5ea
}
pub fn b_5ea(c: &mut Machine) -> u32 {
    {
        // 0x000005ea vldr.32 s0,[sp,#0x44]
        let mut t64000 = 0u64;
        t64000 = ((c.r(84, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x000005ee add r1,sp,#0x44
        let mut t187400 = 0u64;
        t187400 = ((c.r(84, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        let value = ((t187400 >> 0) & 0xffffffffu64);
        c.w(36, 4, value);
    }
    {
        // 0x000005f0 subw r0,r4,#0x848
        let value = (2120u64 <= c.r(48, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 2120u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_sub(2120u64);
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x000005f4 bl 0x000033f8
        let value = (1528u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x33f8u32;
    }
    0x5f8
}
pub fn b_5f8(c: &mut Machine) -> u32 {
    {
        // 0x000005f8 vldr.32 s0,[sp,#0x44]
        let mut t64000 = 0u64;
        t64000 = ((c.r(84, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x000005fc add r1,sp,#0x44
        let mut t187400 = 0u64;
        t187400 = ((c.r(84, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        let value = ((t187400 >> 0) & 0xffffffffu64);
        c.w(36, 4, value);
    }
    {
        // 0x000005fe sub.w r0,r4,#0x6e8
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1768u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = (((t188500 >> 0) & 0xffffffffu64) <= c.r(48, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t188500 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_sub(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00000602 bl 0x000033f8
        let value = (1542u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x33f8u32;
    }
    0x606
}
pub fn b_606(c: &mut Machine) -> u32 {
    {
        // 0x00000606 vldr.32 s0,[sp,#0x44]
        let mut t64000 = 0u64;
        t64000 = ((c.r(84, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x0000060a add r1,sp,#0x4c
        let mut t187400 = 0u64;
        t187400 = ((c.r(84, 4)).wrapping_add(76u64)) & 0xffffffffu64;
        let value = ((t187400 >> 0) & 0xffffffffu64);
        c.w(36, 4, value);
    }
    {
        // 0x0000060c sub.w r0,r4,#0x588
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1416u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = (((t188500 >> 0) & 0xffffffffu64) <= c.r(48, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t188500 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_sub(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00000610 bl 0x000033f8
        let value = (1556u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x33f8u32;
    }
    0x614
}
pub fn b_614(c: &mut Machine) -> u32 {
    {
        // 0x00000614 vldr.32 s0,[r5,#0x330]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(816u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x00000618 vldr.32 s27,[sp,#0x48]
        let mut t64000 = 0u64;
        t64000 = ((c.r(84, 4)).wrapping_add(72u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(876, 4, value);
    }
    {
        // 0x0000061c add r1,sp,#0x40
        let mut t187400 = 0u64;
        t187400 = ((c.r(84, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        let value = ((t187400 >> 0) & 0xffffffffu64);
        c.w(36, 4, value);
    }
    {
        // 0x0000061e vadd.f32 s0,s0,s23
        let value = (f32::from_bits(c.r(768, 4) as u32) + f32::from_bits(c.r(860, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x00000622 sub.w r0,r4,#0x4d8
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1240u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = (((t188500 >> 0) & 0xffffffffu64) <= c.r(48, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t188500 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_sub(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00000626 vmul.f32 s0,s0,s25
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(868, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x0000062a vfma.f32 s0,s28,s27
        let value = f32::from_bits(c.r(880, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(876, 4) as u32),
                f32::from_bits(c.r(768, 4) as u32),
            )
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x0000062e bl 0x000033f8
        let value = (1586u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x33f8u32;
    }
    0x632
}
pub fn b_632(c: &mut Machine) -> u32 {
    {
        // 0x00000632 vldr.32 s0,[sp,#0x40]
        let mut t64000 = 0u64;
        t64000 = ((c.r(84, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x00000636 add r1,sp,#0x40
        let mut t187400 = 0u64;
        t187400 = ((c.r(84, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        let value = ((t187400 >> 0) & 0xffffffffu64);
        c.w(36, 4, value);
    }
    {
        // 0x00000638 sub.w r0,r4,#0x378
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (888u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = (((t188500 >> 0) & 0xffffffffu64) <= c.r(48, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t188500 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_sub(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x0000063c bl 0x000033f8
        let value = (1600u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x33f8u32;
    }
    0x640
}
pub fn b_640(c: &mut Machine) -> u32 {
    {
        // 0x00000640 vldr.32 s0,[sp,#0x40]
        let mut t64000 = 0u64;
        t64000 = ((c.r(84, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x00000644 add r1,sp,#0x40
        let mut t187400 = 0u64;
        t187400 = ((c.r(84, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        let value = ((t187400 >> 0) & 0xffffffffu64);
        c.w(36, 4, value);
    }
    {
        // 0x00000646 sub.w r0,r4,#0x218
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (536u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = (((t188500 >> 0) & 0xffffffffu64) <= c.r(48, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t188500 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_sub(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x0000064a bl 0x000033f8
        let value = (1614u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x33f8u32;
    }
    0x64e
}
pub fn b_64e(c: &mut Machine) -> u32 {
    {
        // 0x0000064e vldr.32 s0,[r5,#0x32c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(812u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x00000652 vldr.32 s26,[sp,#0x4c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(84, 4)).wrapping_add(76u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(872, 4, value);
    }
    {
        // 0x00000656 add r1,sp,#0x44
        let mut t187400 = 0u64;
        t187400 = ((c.r(84, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        let value = ((t187400 >> 0) & 0xffffffffu64);
        c.w(36, 4, value);
    }
    {
        // 0x00000658 vadd.f32 s0,s0,s23
        let value = (f32::from_bits(c.r(768, 4) as u32) + f32::from_bits(c.r(860, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x0000065c sub.w r0,r4,#0x428
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1064u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = (((t188500 >> 0) & 0xffffffffu64) <= c.r(48, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t188500 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_sub(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00000660 vmul.f32 s0,s0,s25
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(868, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x00000664 vfma.f32 s0,s28,s26
        let value = f32::from_bits(c.r(880, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(872, 4) as u32),
                f32::from_bits(c.r(768, 4) as u32),
            )
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x00000668 bl 0x000033f8
        let value = (1644u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x33f8u32;
    }
    0x66c
}
pub fn b_66c(c: &mut Machine) -> u32 {
    {
        // 0x0000066c vldr.32 s0,[sp,#0x44]
        let mut t64000 = 0u64;
        t64000 = ((c.r(84, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x00000670 add r1,sp,#0x44
        let mut t187400 = 0u64;
        t187400 = ((c.r(84, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        let value = ((t187400 >> 0) & 0xffffffffu64);
        c.w(36, 4, value);
    }
    {
        // 0x00000672 sub.w r0,r4,#0x2c8
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (712u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = (((t188500 >> 0) & 0xffffffffu64) <= c.r(48, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t188500 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_sub(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00000676 bl 0x000033f8
        let value = (1658u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x33f8u32;
    }
    0x67a
}
pub fn b_67a(c: &mut Machine) -> u32 {
    {
        // 0x0000067a vldr.32 s0,[sp,#0x44]
        let mut t64000 = 0u64;
        t64000 = ((c.r(84, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x0000067e add r1,sp,#0x44
        let mut t187400 = 0u64;
        t187400 = ((c.r(84, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        let value = ((t187400 >> 0) & 0xffffffffu64);
        c.w(36, 4, value);
    }
    {
        // 0x00000680 sub.w r0,r4,#0x168
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (360u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = (((t188500 >> 0) & 0xffffffffu64) <= c.r(48, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t188500 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_sub(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00000684 bl 0x000033f8
        let value = (1672u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x33f8u32;
    }
    0x688
}
pub fn b_688(c: &mut Machine) -> u32 {
    {
        // 0x00000688 vldr.32 s25,[sp,#0x40]
        let mut t64000 = 0u64;
        t64000 = ((c.r(84, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(868, 4, value);
    }
    {
        // 0x0000068c addw r1,r4,#0x8d4
        let value = ((c.r(48, 4) as u128 + 2260u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 2260u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(2260u64);
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00000690 sub.w r0,r4,#0xb8
        let mut t187d00 = 0u64;
        t187d00 = (184u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(48, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00000694 vmov.f32 s0,s25
        let value = c.r(868, 4);
        c.w(768, 4, value);
    }
    {
        // 0x00000698 bl 0x000033c8
        let value = (1692u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x33c8u32;
    }
    0x69c
}
pub fn b_69c(c: &mut Machine) -> u32 {
    {
        // 0x0000069c vldr.32 s28,[sp,#0x44]
        let mut t64000 = 0u64;
        t64000 = ((c.r(84, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(880, 4, value);
    }
    {
        // 0x000006a0 addw r1,r4,#0x8d8
        let value = ((c.r(48, 4) as u128 + 2264u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 2264u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(2264u64);
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x000006a4 sub.w r0,r4,#0x8
        let mut t187d00 = 0u64;
        t187d00 = (8u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(48, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x000006a8 vmov.f32 s0,s28
        let value = c.r(880, 4);
        c.w(768, 4, value);
    }
    {
        // 0x000006ac bl 0x000033c8
        let value = (1712u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x33c8u32;
    }
    0x6b0
}
pub fn b_6b0(c: &mut Machine) -> u32 {
    {
        // 0x000006b0 vldr.32 s5,[r5,#0x32c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(812u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(788, 4, value);
    }
    {
        // 0x000006b4 vldr.32 s15,[r5,#0x228]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(552u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x000006b8 vldr.32 s13,[r5,#0x330]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(816u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x000006bc vldr.32 s9,[r5,#0x248]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(584u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(804, 4, value);
    }
    {
        // 0x000006c0 vldr.32 s10,[r5,#0x24c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(588u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x000006c4 vfma.f32 s9,s5,s15
        let value = f32::from_bits(c.r(788, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(804, 4) as u32),
            )
            .to_bits() as u64;
        c.w(804, 4, value);
    }
    {
        // 0x000006c8 vldr.32 s4,[r5,#0x240]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(576u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(784, 4, value);
    }
    {
        // 0x000006cc vfma.f32 s10,s15,s13
        let value = f32::from_bits(c.r(828, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(820, 4) as u32),
                f32::from_bits(c.r(808, 4) as u32),
            )
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x000006d0 vldr.32 s11,[r5,#0x250]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(592u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x000006d4 vldr.32 s6,[r5,#0x254]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(596u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x000006d8 vldr.32 s15,[r5,#0x224]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(548u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x000006dc vldr.32 s14,[r5,#0x23c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(572u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x000006e0 vldr.32 s7,[r5,#0x22c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(556u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x000006e4 vfma.f32 s11,s9,s4
        let value = f32::from_bits(c.r(804, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(784, 4) as u32),
                f32::from_bits(c.r(812, 4) as u32),
            )
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x000006e8 vldr.32 s8,[r5,#0x244]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(580u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x000006ec vfma.f32 s6,s10,s4
        let value = f32::from_bits(c.r(808, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(784, 4) as u32),
                f32::from_bits(c.r(792, 4) as u32),
            )
            .to_bits() as u64;
        c.w(792, 4, value);
    }
    {
        // 0x000006f0 vnmul.f32 s12,s15,s9
        let mut t71900 = 0u64;
        t71900 = ((f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(804, 4) as u32))
            .to_bits() as u64)
            & 0xffffffffu64;
        let value = (-f32::from_bits(((t71900 >> 0) & 0xffffffffu64) as u32)).to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x000006f4 vnmul.f32 s15,s10,s15
        let mut t71900 = 0u64;
        t71900 = ((f32::from_bits(c.r(808, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64)
            & 0xffffffffu64;
        let value = (-f32::from_bits(((t71900 >> 0) & 0xffffffffu64) as u32)).to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x000006f8 vfma.f32 s12,s5,s7
        let value = f32::from_bits(c.r(788, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(796, 4) as u32),
                f32::from_bits(c.r(816, 4) as u32),
            )
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x000006fc vfma.f32 s15,s7,s13
        let value = f32::from_bits(c.r(796, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(820, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00000700 vstr.32 s11,[r5,#0x32c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(812u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(812, 4));
    }
    {
        // 0x00000704 vnmul.f32 s13,s14,s11
        let mut t71900 = 0u64;
        t71900 = ((f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64)
            & 0xffffffffu64;
        let value = (-f32::from_bits(((t71900 >> 0) & 0xffffffffu64) as u32)).to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00000708 vnmul.f32 s14,s6,s14
        let mut t71900 = 0u64;
        t71900 = ((f32::from_bits(c.r(792, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64)
            & 0xffffffffu64;
        let value = (-f32::from_bits(((t71900 >> 0) & 0xffffffffu64) as u32)).to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x0000070c vfma.f32 s13,s9,s8
        let value = f32::from_bits(c.r(804, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(800, 4) as u32),
                f32::from_bits(c.r(820, 4) as u32),
            )
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00000710 vfma.f32 s14,s10,s8
        let value = f32::from_bits(c.r(808, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(800, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00000714 vadd.f32 s12,s12,s23
        let value = (f32::from_bits(c.r(816, 4) as u32) + f32::from_bits(c.r(860, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00000718 b 0x00000738
        return 0x738u32;
    }
    0x71a
}
pub fn b_738(c: &mut Machine) -> u32 {
    {
        // 0x00000738 vadd.f32 s10,s14,s23
        let value = (f32::from_bits(c.r(824, 4) as u32) + f32::from_bits(c.r(860, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x0000073c vldr.32 s14,[r5,#0x2a0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(672u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00000740 vstr.32 s12,[r5,#0x248]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(584u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x00000744 vadd.f32 s15,s15,s23
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(860, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00000748 vmul.f32 s12,s14,s28
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(880, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x0000074c vldr.32 s8,[r5,#0x29c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(668u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x00000750 vmul.f32 s25,s14,s25
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(868, 4) as u32))
            .to_bits() as u64;
        c.w(868, 4, value);
    }
    {
        // 0x00000754 vldr.32 s0,[r5,#0x1d8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(472u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x00000758 vstr.32 s15,[r5,#0x24c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(588u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x0000075c vadd.f32 s15,s13,s23
        let value = (f32::from_bits(c.r(820, 4) as u32) + f32::from_bits(c.r(860, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00000760 vfma.f32 s12,s8,s26
        let value = f32::from_bits(c.r(800, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(872, 4) as u32),
                f32::from_bits(c.r(816, 4) as u32),
            )
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00000764 vldr.32 s11,[r5,#0x1e4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(484u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x00000768 vfma.f32 s25,s8,s27
        let value = f32::from_bits(c.r(800, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(876, 4) as u32),
                f32::from_bits(c.r(868, 4) as u32),
            )
            .to_bits() as u64;
        c.w(868, 4, value);
    }
    {
        // 0x0000076c vldr.32 s9,[r5,#0x214]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(532u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(804, 4, value);
    }
    {
        // 0x00000770 vstr.32 s15,[r5,#0x250]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(592u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x00000774 vldr.32 s15,[r5,#0x1e0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(480u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00000778 vmov.f32 s14,s9
        let value = c.r(804, 4);
        c.w(824, 4, value);
    }
    {
        // 0x0000077c vldr.32 s13,[r5,#0x1fc]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(508u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00000780 vstr.32 s6,[r5,#0x330]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(816u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(792, 4));
    }
    {
        // 0x00000784 vmov.f32 s2,s12
        let value = c.r(816, 4);
        c.w(776, 4, value);
    }
    {
        // 0x00000788 vldr.32 s5,[r5,#0x1f8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(504u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(788, 4, value);
    }
    {
        // 0x0000078c vfma.f32 s15,s25,s0
        let value = f32::from_bits(c.r(868, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(768, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00000790 vldr.32 s1,[r5,#0x210]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(528u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(772, 4, value);
    }
    {
        // 0x00000794 vldr.32 s12,[r5,#0x1d4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(468u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x00000798 vfma.f32 s11,s2,s0
        let value = f32::from_bits(c.r(776, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(768, 4) as u32),
                f32::from_bits(c.r(812, 4) as u32),
            )
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x0000079c vstr.32 s10,[r5,#0x254]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(596u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(808, 4));
    }
    {
        // 0x000007a0 vldr.32 s24,[r5,#0x1dc]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(476u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(864, 4, value);
    }
    {
        // 0x000007a4 vldr.32 s3,[r5,#0x1f0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(496u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(780, 4, value);
    }
    {
        // 0x000007a8 vldr.32 s4,[r5,#0x208]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(520u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(784, 4, value);
    }
    {
        // 0x000007ac vsub.f32 s0,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x000007b0 vldr.32 s7,[r5,#0x2b0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(688u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x000007b4 vnmul.f32 s10,s12,s15
        let mut t71900 = 0u64;
        t71900 = ((f32::from_bits(c.r(816, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64)
            & 0xffffffffu64;
        let value = (-f32::from_bits(((t71900 >> 0) & 0xffffffffu64) as u32)).to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x000007b8 vldr.32 s8,[r5,#0x2ac]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(684u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x000007bc vsub.f32 s6,s11,s9
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(804, 4) as u32))
            .to_bits() as u64;
        c.w(792, 4, value);
    }
    {
        // 0x000007c0 ldrb.w r3,[r5,#0x384]
        let mut t19c800 = 0u64;
        let mut t1aa600 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(900u64)) & 0xffffffffu64;
        t1aa600 = (c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 1)) & 0xffu64;
        let value = ((t1aa600 >> 0) & 0xffu64);
        c.w(44, 4, value);
    }
    {
        // 0x000007c4 vfma.f32 s13,s0,s5
        let value = f32::from_bits(c.r(768, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(788, 4) as u32),
                f32::from_bits(c.r(820, 4) as u32),
            )
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x000007c8 vnmul.f32 s12,s11,s12
        let mut t71900 = 0u64;
        t71900 = ((f32::from_bits(c.r(812, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64)
            & 0xffffffffu64;
        let value = (-f32::from_bits(((t71900 >> 0) & 0xffffffffu64) as u32)).to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x000007cc vfma.f32 s14,s6,s1
        let value = f32::from_bits(c.r(792, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(772, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x000007d0 vfma.f32 s10,s25,s24
        let value = f32::from_bits(c.r(868, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(864, 4) as u32),
                f32::from_bits(c.r(808, 4) as u32),
            )
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x000007d4 vfma.f32 s12,s2,s24
        let value = f32::from_bits(c.r(776, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(864, 4) as u32),
                f32::from_bits(c.r(816, 4) as u32),
            )
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x000007d8 vsub.f32 s2,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(776, 4, value);
    }
    {
        // 0x000007dc vfma.f32 s13,s0,s5
        let value = f32::from_bits(c.r(768, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(788, 4) as u32),
                f32::from_bits(c.r(820, 4) as u32),
            )
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x000007e0 vsub.f32 s9,s11,s14
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(804, 4, value);
    }
    {
        // 0x000007e4 vfma.f32 s15,s2,s3
        let value = f32::from_bits(c.r(776, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(780, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x000007e8 vfma.f32 s14,s6,s1
        let value = f32::from_bits(c.r(792, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(772, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x000007ec vfma.f32 s11,s9,s4
        let value = f32::from_bits(c.r(804, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(784, 4) as u32),
                f32::from_bits(c.r(812, 4) as u32),
            )
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x000007f0 vadd.f32 s10,s10,s23
        let value = (f32::from_bits(c.r(808, 4) as u32) + f32::from_bits(c.r(860, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x000007f4 vadd.f32 s12,s12,s23
        let value = (f32::from_bits(c.r(816, 4) as u32) + f32::from_bits(c.r(860, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x000007f8 vadd.f32 s13,s13,s23
        let value = (f32::from_bits(c.r(820, 4) as u32) + f32::from_bits(c.r(860, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x000007fc vstr.32 s10,[r5,#0x1e0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(480u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(808, 4));
    }
    {
        // 0x00000800 vadd.f32 s14,s14,s23
        let value = (f32::from_bits(c.r(824, 4) as u32) + f32::from_bits(c.r(860, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00000804 vstr.32 s12,[r5,#0x1e4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(484u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x00000808 vsub.f32 s25,s15,s11
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(868, 4, value);
    }
    {
        // 0x0000080c vstr.32 s13,[r5,#0x1fc]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(508u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x00000810 vadd.f32 s15,s15,s11
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00000814 vstr.32 s14,[r5,#0x214]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(532u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x00000818 vmul.f32 s25,s25,s7
        let value = (f32::from_bits(c.r(868, 4) as u32) * f32::from_bits(c.r(796, 4) as u32))
            .to_bits() as u64;
        c.w(868, 4, value);
    }
    {
        // 0x0000081c vmov.f32 s24,s25
        let value = c.r(868, 4);
        c.w(864, 4, value);
    }
    {
        // 0x00000820 vfnms.f32 s25,s8,s15
        let mut t49a00 = 0u64;
        let mut t49b00 = 0u64;
        t49a00 = ((-f32::from_bits(c.r(868, 4) as u32)).to_bits() as u64) & 0xffffffffu64;
        t49b00 = ((f32::from_bits(c.r(800, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64)
            & 0xffffffffu64;
        let value = (f32::from_bits(((t49a00 >> 0) & 0xffffffffu64) as u32)
            + f32::from_bits(((t49b00 >> 0) & 0xffffffffu64) as u32))
        .to_bits() as u64;
        c.w(868, 4, value);
    }
    {
        // 0x00000824 vfma.f32 s24,s8,s15
        let value = f32::from_bits(c.r(800, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(864, 4) as u32),
            )
            .to_bits() as u64;
        c.w(864, 4, value);
    }
    {
        // 0x00000828 vstr.32 s25,[sp,#0x44]
        let mut t64000 = 0u64;
        t64000 = ((c.r(84, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(868, 4));
    }
    {
        // 0x0000082c vstr.32 s24,[sp,#0x40]
        let mut t64000 = 0u64;
        t64000 = ((c.r(84, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(864, 4));
    }
    {
        // 0x00000830 cmp r3,#0x0
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
        // 0x00000832 bne.w 0x00002e74
        let mut t182500 = 0u64;
        t182500 = ((c.r(97, 1) == 0u64) as u64) & 0xffu64;
        if ((t182500 >> 0) & 0xffu64) != 0 {
            return 0x2e74u32;
        }
    }
    0x836
}
pub fn b_836(c: &mut Machine) -> u32 {
    {
        // 0x00000836 ldrb.w r3,[r5,#0x385]
        let mut t19c800 = 0u64;
        let mut t1aa600 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(901u64)) & 0xffffffffu64;
        t1aa600 = (c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 1)) & 0xffu64;
        let value = ((t1aa600 >> 0) & 0xffu64);
        c.w(44, 4, value);
    }
    {
        // 0x0000083a cmp r3,#0x0
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
        // 0x0000083c bne.w 0x0000300e
        let mut t182500 = 0u64;
        t182500 = ((c.r(97, 1) == 0u64) as u64) & 0xffu64;
        if ((t182500 >> 0) & 0xffu64) != 0 {
            return 0x300eu32;
        }
    }
    0x840
}
pub fn b_840(c: &mut Machine) -> u32 {
    {
        // 0x00000840 vldr.32 s15,[r5,#0x2a8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(680u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00000844 vmov.f32 s14,0x40800000
        let value = 1082130432u64;
        c.w(824, 4, value);
    }
    {
        // 0x00000848 vldr.32 s13,[r5,#0x2a4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(676u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x0000084c vmul.f32 s24,s15,s24
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(864, 4) as u32))
            .to_bits() as u64;
        c.w(864, 4, value);
    }
    {
        // 0x00000850 ldr r2,[sp,#0x28]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00000852 vmul.f32 s15,s15,s25
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(868, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00000856 ldr r3,[sp,#0x2c]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00000858 vmov.f32 s12,s14
        let value = c.r(824, 4);
        c.w(816, 4, value);
    }
    {
        // 0x0000085c adds r2,#0x8
        let value = ((c.r(40, 4) as u128 + 8u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(40, 4), 8u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(40, 4)).wrapping_add(8u64);
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
        // 0x0000085e vfma.f32 s24,s17,s13
        let value = f32::from_bits(c.r(836, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(820, 4) as u32),
                f32::from_bits(c.r(864, 4) as u32),
            )
            .to_bits() as u64;
        c.w(864, 4, value);
    }
    {
        // 0x00000862 adds r3,#0x1
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
        // 0x00000864 vfma.f32 s15,s16,s13
        let value = f32::from_bits(c.r(832, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(820, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00000868 str r2,[sp,#0x28]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x0000086a vmov.f32 s11,s14
        let value = c.r(824, 4);
        c.w(812, 4, value);
    }
    {
        // 0x0000086e ldr r2,[sp,#0x30]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00000870 str r3,[sp,#0x2c]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00000872 vfma.f32 s11,s24,s24
        let value = f32::from_bits(c.r(864, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(864, 4) as u32),
                f32::from_bits(c.r(812, 4) as u32),
            )
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00000876 vfma.f32 s12,s15,s15
        let value = f32::from_bits(c.r(828, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(816, 4) as u32),
            )
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x0000087a vmul.f32 s15,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x0000087e vmul.f32 s24,s24,s14
        let value = (f32::from_bits(c.r(864, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(864, 4, value);
    }
    {
        // 0x00000882 vdiv.f32 s13,s24,s11
        let value = (f32::from_bits(c.r(864, 4) as u32) / f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00000886 vdiv.f32 s14,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) / f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x0000088a vcvt.s32.f32 s13,s13,#0x1f
        let mut t3d400 = 0u64;
        t3d400 = (31u64) & 0xffu64;
        let value = c.userop(223, c.r(820, 4), 32u64, ((t3d400 >> 0) & 0xffu64), 4, 4);
        c.w(820, 4, value);
    }
    {
        // 0x0000088e vstr.32 s13,[r2,#-0x4]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(40, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x00000892 vcvt.s32.f32 s14,s14,#0x1f
        let mut t3d400 = 0u64;
        t3d400 = (31u64) & 0xffu64;
        let value = c.userop(223, c.r(824, 4), 32u64, ((t3d400 >> 0) & 0xffu64), 4, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00000896 vmov r1,s14
        let value = c.r(824, 4);
        c.w(36, 4, value);
    }
    {
        // 0x0000089a str.w r1,[r2],#0x8
        let mut t19cb00 = 0u64;
        t19cb00 = (c.r(40, 4)) & 0xffffffffu64;
        let value = (c.r(40, 4)).wrapping_add(8u64);
        c.w(40, 4, value);
        c.store(((t19cb00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x0000089e str r2,[sp,#0x30]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x000008a0 ldr r2,[sp,#0x34]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x000008a2 cmp r3,r2
        let mut t1a6e00 = 0u64;
        let value = (c.r(40, 4) <= c.r(44, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), c.r(40, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(44, 4)).wrapping_sub(c.r(40, 4))) & 0xffffffffu64;
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
        // 0x000008a4 beq.w 0x00003138
        let mut t182300 = 0u64;
        t182300 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        if ((t182300 >> 0) & 0xffu64) != 0 {
            return 0x3138u32;
        }
    }
    0x8a8
}
pub fn b_8a8(c: &mut Machine) -> u32 {
    {
        // 0x000008a8 vldr.32 s24,[r5,#0x28c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(652u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(864, 4, value);
    }
    {
        // 0x000008ac vmov.f32 s11,0x3f800000
        let value = 1065353216u64;
        c.w(812, 4, value);
    }
    {
        // 0x000008b0 vldr.32 s13,[r5,#0x320]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(800u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x000008b4 vmov.f32 s15,0xc1400000
        let value = 3242196992u64;
        c.w(828, 4, value);
    }
    {
        // 0x000008b8 vldr.32 s9,[r5,#0x288]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(648u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(804, 4, value);
    }
    {
        // 0x000008bc vsub.f32 s13,s13,s24
        let value = (f32::from_bits(c.r(820, 4) as u32) - f32::from_bits(c.r(864, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x000008c0 vldr.32 s28,[r5,#0x298]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(664u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(880, 4, value);
    }
    {
        // 0x000008c4 vldr.32 s10,[r5,#0x37c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(892u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x000008c8 ldr r3,[sp,#0x28]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x000008ca vfma.f32 s24,s13,s9
        let value = f32::from_bits(c.r(820, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(864, 4) as u32),
            )
            .to_bits() as u64;
        c.w(864, 4, value);
    }
    {
        // 0x000008ce vldr.32 s9,[r5,#0x294]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(660u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(804, 4, value);
    }
    {
        // 0x000008d2 vsub.f32 s10,s10,s28
        let value = (f32::from_bits(c.r(808, 4) as u32) - f32::from_bits(c.r(880, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x000008d6 ldr.w r2,[r3,#-0x4]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x000008da ldr r3,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x000008dc vldr.32 s14,[r5,#0x324]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(804u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x000008e0 vmov s8,r2
        let value = c.r(40, 4);
        c.w(800, 4, value);
    }
    {
        // 0x000008e4 vfma.f32 s28,s10,s9
        let value = f32::from_bits(c.r(808, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(880, 4) as u32),
            )
            .to_bits() as u64;
        c.w(880, 4, value);
    }
    {
        // 0x000008e8 vmov s10,r3
        let value = c.r(44, 4);
        c.w(808, 4, value);
    }
    {
        // 0x000008ec vcvt.f32.s32 s17,s8
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
        let value = c.userop(219, c.r(800, 4), ((t3b500 >> 0) & 0xffu64), 0u64, 4, 4);
        c.w(836, 4, value);
    }
    {
        // 0x000008f0 vldr.32 s8,[pc,#-0x1d4]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((2288u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(468u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x000008f4 vadd.f32 s24,s24,s23
        let value = (f32::from_bits(c.r(864, 4) as u32) + f32::from_bits(c.r(860, 4) as u32))
            .to_bits() as u64;
        c.w(864, 4, value);
    }
    {
        // 0x000008f8 str r3,[sp,#0x8]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x000008fa vcvt.f32.s32 s16,s10
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
        c.w(832, 4, value);
    }
    {
        // 0x000008fe vldr.32 s10,[r5,#0x328]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(808u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x00000902 vmul.f32 s17,s17,s8
        let value = (f32::from_bits(c.r(836, 4) as u32) * f32::from_bits(c.r(800, 4) as u32))
            .to_bits() as u64;
        c.w(836, 4, value);
    }
    {
        // 0x00000906 vldr.32 s12,[r5,#0x21c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(540u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x0000090a vsub.f32 s26,s11,s24
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(864, 4) as u32))
            .to_bits() as u64;
        c.w(872, 4, value);
    }
    {
        // 0x0000090e vldr.32 s13,[r5,#0x218]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(536u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00000912 vmul.f32 s16,s16,s8
        let value = (f32::from_bits(c.r(832, 4) as u32) * f32::from_bits(c.r(800, 4) as u32))
            .to_bits() as u64;
        c.w(832, 4, value);
    }
    {
        // 0x00000916 vldr.32 s9,[pc,#-0x1f4]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((2326u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(500u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(804, 4, value);
    }
    {
        // 0x0000091a vadd.f32 s28,s28,s23
        let value = (f32::from_bits(c.r(880, 4) as u32) + f32::from_bits(c.r(860, 4) as u32))
            .to_bits() as u64;
        c.w(880, 4, value);
    }
    {
        // 0x0000091e vldr.32 s7,[r5,#0x31c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(796u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x00000922 vmul.f32 s15,s26,s15
        let value = (f32::from_bits(c.r(872, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00000926 str r2,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x00000928 vsub.f32 s25,s11,s26
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(872, 4) as u32))
            .to_bits() as u64;
        c.w(868, 4, value);
    }
    {
        // 0x0000092c vstr.32 s24,[r5,#0x28c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(652u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(864, 4));
    }
    {
        // 0x00000930 vstr.32 s28,[r5,#0x298]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(664u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(880, 4));
    }
    {
        // 0x00000934 vmul.f32 s14,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00000938 vmul.f32 s10,s15,s10
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x0000093c vfnms.f32 s25,s26,s7
        let mut t49a00 = 0u64;
        let mut t49b00 = 0u64;
        t49a00 = ((-f32::from_bits(c.r(868, 4) as u32)).to_bits() as u64) & 0xffffffffu64;
        t49b00 = ((f32::from_bits(c.r(872, 4) as u32) * f32::from_bits(c.r(796, 4) as u32))
            .to_bits() as u64)
            & 0xffffffffu64;
        let value = (f32::from_bits(((t49a00 >> 0) & 0xffffffffu64) as u32)
            + f32::from_bits(((t49b00 >> 0) & 0xffffffffu64) as u32))
        .to_bits() as u64;
        c.w(868, 4, value);
    }
    {
        // 0x00000940 vmul.f32 s15,s14,s22
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(856, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00000944 vstr.32 s14,[r5,#0x220]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(544u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x00000948 vcvt.s32.f32 s14,s15
        let value = (f32::from_bits(c.r(828, 4) as u32) as i32) as u64;
        c.w(824, 4, value);
    }
    {
        // 0x0000094c vcvt.f32.s32 s8,s14
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
        c.w(800, 4, value);
    }
    {
        // 0x00000950 vmov r3,s14
        let value = c.r(824, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00000954 vmov.f32 s14,s19
        let value = c.r(844, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00000958 adds r3,#0x7f
        let value = ((c.r(44, 4) as u128 + 127u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), 127u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_add(127u64);
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
        // 0x0000095a vsub.f32 s15,s15,s8
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(800, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x0000095e vmov.f32 s8,s20
        let value = c.r(848, 4);
        c.w(800, 4, value);
    }
    {
        // 0x00000962 lsls r3,r3,#0x17
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
            (c.r(44, 4)) << (((t1afd00 >> 0) & 0xffffffffu64) as u32)
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
            (c.r(44, 4)) << (23u64 as u32)
        };
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
        // 0x00000964 vfma.f32 s8,s15,s21
        let value = f32::from_bits(c.r(828, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(852, 4) as u32),
                f32::from_bits(c.r(800, 4) as u32),
            )
            .to_bits() as u64;
        c.w(800, 4, value);
    }
    {
        // 0x00000968 vfma.f32 s14,s15,s8
        let value = f32::from_bits(c.r(828, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(800, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x0000096c vmov.f32 s8,s18
        let value = c.r(840, 4);
        c.w(800, 4, value);
    }
    {
        // 0x00000970 vfma.f32 s8,s15,s14
        let value = f32::from_bits(c.r(828, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(824, 4) as u32),
                f32::from_bits(c.r(800, 4) as u32),
            )
            .to_bits() as u64;
        c.w(800, 4, value);
    }
    {
        // 0x00000974 vmov.f32 s14,s11
        let value = c.r(812, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00000978 vfma.f32 s14,s15,s8
        let value = f32::from_bits(c.r(828, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(800, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x0000097c vmov s15,r3
        let value = c.r(44, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00000980 vmul.f32 s15,s14,s15
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00000984 vdiv.f32 s14,s12,s15
        let value = (f32::from_bits(c.r(816, 4) as u32) / f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00000988 vmul.f32 s12,s15,s15
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x0000098c vmul.f32 s14,s14,s13
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00000990 vcmpe.f32 s14,s9
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
        t35c00 = ((f32::from_bits(c.r(824, 4) as u32) < f32::from_bits(c.r(804, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(824, 4) as u32) == f32::from_bits(c.r(804, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(804, 4) as u32) <= f32::from_bits(c.r(824, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(824, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x00000994 vmrs apsr,fpscr
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
        // 0x00000998 bgt.w 0x00000274
        let mut t183c00 = 0u64;
        let mut t183d00 = 0u64;
        let mut t183f00 = 0u64;
        t183c00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t183d00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t183f00 = (((((t183c00 >> 0) & 0xffu64) != 0) & (((t183d00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t183f00 >> 0) & 0xffu64) != 0 {
            return 0x274u32;
        }
    }
    0x99c
}
pub fn b_99c(c: &mut Machine) -> u32 {
    {
        // 0x0000099c vldr.32 s13,[pc,#-0x274]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((2460u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(628u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x000009a0 vldr.32 s8,[pc,#-0x27c]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((2464u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(636u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x000009a4 b 0x0000028c
        return 0x28cu32;
    }
    0x9a6
}
pub fn b_9a6(c: &mut Machine) -> u32 {
    {
        // 0x000009a6 vldr.32 s13,[pc,#-0x27c]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((2470u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(636u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x000009aa vldr.32 s10,[pc,#-0x27c]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((2474u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(636u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x000009ae b 0x0000032c
        return 0x32cu32;
    }
    0x9b0
}
pub fn b_9b0(c: &mut Machine) -> u32 {
    {
        // 0x000009b0 vldr.32 s11,[r5,#0x34c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(844u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x000009b4 vmov.f32 s29,0x3f800000
        let value = 1065353216u64;
        c.w(884, 4, value);
    }
    {
        // 0x000009b8 vldr.32 s8,[pc,#-0x288]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((2488u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(648u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x000009bc vadd.f32 s11,s30,s11
        let value = (f32::from_bits(c.r(888, 4) as u32) + f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x000009c0 vldr.32 s15,[r5,#0x350]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(848u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x000009c4 vldr.32 s12,[r5,#0x354]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(852u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x000009c8 vmov.f32 s6,s8
        let value = c.r(800, 4);
        c.w(792, 4, value);
    }
    {
        // 0x000009cc vsub.f32 s15,s15,s30
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(888, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x000009d0 vldr.32 s13,[r5,#0x358]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(856u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x000009d4 vcmpe.f32 s11,s8
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
        t35c00 = ((f32::from_bits(c.r(812, 4) as u32) < f32::from_bits(c.r(800, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(812, 4) as u32) == f32::from_bits(c.r(800, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(800, 4) as u32) <= f32::from_bits(c.r(812, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(812, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(800, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x000009d8 ldr.w r3,[r6,#0xed4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3796u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x000009dc vadd.f32 s12,s31,s12
        let value = (f32::from_bits(c.r(892, 4) as u32) + f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x000009e0 ldr.w r1,[r6,#0xf34]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3892u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x000009e4 vmov.f32 s9,s8
        let value = c.r(800, 4);
        c.w(804, 4, value);
    }
    {
        // 0x000009e8 vldr.32 s14,[r5,#0x35c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(860u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x000009ec vmrs apsr,fpscr
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
        // 0x000009f0 ldr.w r0,[r6,#0xf94]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3988u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x000009f4 vcmpe.f32 s15,s8
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
        t35c00 = ((f32::from_bits(c.r(828, 4) as u32) < f32::from_bits(c.r(800, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(828, 4) as u32) == f32::from_bits(c.r(800, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(800, 4) as u32) <= f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(800, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x000009f8 ldr.w r2,[r6,#0xff4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(4084u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x000009fc vsub.f32 s13,s13,s31
        let value = (f32::from_bits(c.r(820, 4) as u32) - f32::from_bits(c.r(892, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00000a00 vmov s5,r1
        let value = c.r(36, 4);
        c.w(788, 4, value);
    }
    {
        // 0x00000a04 vselgt.f32 s11,s11,s8
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(800, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(812, 4, value);
    }
    {
        // 0x00000a08 vmov.f32 s10,s8
        let value = c.r(800, 4);
        c.w(808, 4, value);
    }
    {
        // 0x00000a0c vmrs apsr,fpscr
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
        // 0x00000a10 str r3,[sp,#0x8]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00000a12 vcmpe.f32 s12,s8
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
        t35c00 = ((f32::from_bits(c.r(816, 4) as u32) < f32::from_bits(c.r(800, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(816, 4) as u32) == f32::from_bits(c.r(800, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(800, 4) as u32) <= f32::from_bits(c.r(816, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(816, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(800, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x00000a16 str r1,[sp,#0xc]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(12u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x00000a18 vmov.f32 s7,s8
        let value = c.r(800, 4);
        c.w(796, 4, value);
    }
    {
        // 0x00000a1c vmov s8,r3
        let value = c.r(44, 4);
        c.w(800, 4, value);
    }
    {
        // 0x00000a20 vselgt.f32 s15,s15,s9
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(804, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(828, 4, value);
    }
    {
        // 0x00000a24 vadd.f32 s14,s27,s14
        let value = (f32::from_bits(c.r(876, 4) as u32) + f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00000a28 vmrs apsr,fpscr
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
        // 0x00000a2c ldr r3,[r5,#0x54]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(84u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00000a2e vcmpe.f32 s13,s6
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
        t35c00 = ((f32::from_bits(c.r(820, 4) as u32) < f32::from_bits(c.r(792, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(820, 4) as u32) == f32::from_bits(c.r(792, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(792, 4) as u32) <= f32::from_bits(c.r(820, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(820, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(792, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x00000a32 vmov s9,r2
        let value = c.r(40, 4);
        c.w(804, 4, value);
    }
    {
        // 0x00000a36 vcvt.f32.s32 s8,s8
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
        let value = c.userop(219, c.r(800, 4), ((t3b500 >> 0) & 0xffu64), 0u64, 4, 4);
        c.w(800, 4, value);
    }
    {
        // 0x00000a3a ldr r1,[0x00000d44]
        let value = c.load(0xd44, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00000a3c vcvt.f32.s32 s3,s5
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
        c.w(780, 4, value);
    }
    {
        // 0x00000a40 vmov s5,r0
        let value = c.r(32, 4);
        c.w(788, 4, value);
    }
    {
        // 0x00000a44 vselgt.f32 s12,s12,s6
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(792, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(816, 4, value);
    }
    {
        // 0x00000a48 vmrs apsr,fpscr
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
        // 0x00000a4c vcmpe.f32 s14,s6
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
        t35c00 = ((f32::from_bits(c.r(824, 4) as u32) < f32::from_bits(c.r(792, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(824, 4) as u32) == f32::from_bits(c.r(792, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(792, 4) as u32) <= f32::from_bits(c.r(824, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(824, 4) as u32).is_nan() as u64) & 0xffu64;
        t36200 = (f32::from_bits(c.r(792, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x00000a50 str r0,[sp,#0x10]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x00000a52 vcvt.f32.s32 s4,s5
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
        c.w(784, 4, value);
    }
    {
        // 0x00000a56 ldr.w lr,[0x00000d54]
        let value = c.load(0xd54, 4);
        c.w(88, 4, value);
    }
    {
        // 0x00000a5a vcvt.f32.s32 s5,s9
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
        c.w(788, 4, value);
    }
    {
        // 0x00000a5e vselgt.f32 s13,s13,s7
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(796, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(820, 4, value);
    }
    {
        // 0x00000a62 vdiv.f32 s9,s8,s11
        let value = (f32::from_bits(c.r(800, 4) as u32) / f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(804, 4, value);
    }
    {
        // 0x00000a66 vmov s11,r3
        let value = c.r(44, 4);
        c.w(812, 4, value);
    }
    {
        // 0x00000a6a mov r11,lr
        let value = c.r(88, 4);
        c.w(76, 4, value);
    }
    {
        // 0x00000a6c sub.w r10,lr,#0x4
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
        // 0x00000a70 sub.w r9,lr,#0xc
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
        // 0x00000a74 add.w r12,lr,#0x14
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
        // 0x00000a78 sub.w r8,lr,#0x14
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
        // 0x00000a7c vcvt.f32.s32 s6,s11
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
        let value = c.userop(219, c.r(812, 4), ((t3b500 >> 0) & 0xffu64), 0u64, 4, 4);
        c.w(792, 4, value);
    }
    {
        // 0x00000a80 vldr.32 s11,[r5,#0x4c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(76u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x00000a84 vmrs apsr,fpscr
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
        // 0x00000a88 vdiv.f32 s8,s3,s15
        let value = (f32::from_bits(c.r(780, 4) as u32) / f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(800, 4, value);
    }
    {
        // 0x00000a8c vldr.32 s15,[r5,#0x360]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(864u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00000a90 vselgt.f32 s14,s14,s10
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(808, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(824, 4, value);
    }
    {
        // 0x00000a94 vdiv.f32 s3,s4,s12
        let value = (f32::from_bits(c.r(784, 4) as u32) / f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(780, 4, value);
    }
    {
        // 0x00000a98 vstr.32 s9,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(804, 4));
    }
    {
        // 0x00000a9c vdiv.f32 s12,s6,s14
        let value = (f32::from_bits(c.r(792, 4) as u32) / f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00000aa0 vldr.32 s14,[r1,#0x108]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(264u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00000aa4 vdiv.f32 s7,s5,s13
        let value = (f32::from_bits(c.r(788, 4) as u32) / f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(796, 4, value);
    }
    {
        // 0x00000aa8 vldr.32 s13,[r1,#0xa8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(168u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00000aac vstr.32 s8,[r1,#0x60]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(96u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(800, 4));
    }
    {
        // 0x00000ab0 vsub.f32 s15,s15,s27
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(876, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00000ab4 vmov.f32 s4,s10
        let value = c.r(808, 4);
        c.w(784, 4, value);
    }
    {
        // 0x00000ab8 vstr.32 s3,[r1,#0xc0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(192u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(780, 4));
    }
    {
        // 0x00000abc vmul.f32 s13,s3,s13
        let value = (f32::from_bits(c.r(780, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00000ac0 vcmpe.f32 s15,s10
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
        t35c00 = ((f32::from_bits(c.r(828, 4) as u32) < f32::from_bits(c.r(808, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(828, 4) as u32) == f32::from_bits(c.r(808, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(808, 4) as u32) <= f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x00000ac4 vldr.32 s10,[r1,#-0x18]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(24u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x00000ac8 vmul.f32 s10,s9,s10
        let value = (f32::from_bits(c.r(804, 4) as u32) * f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x00000acc vstr.32 s13,[r1,#0xc4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(196u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x00000ad0 vmul.f32 s11,s12,s11
        let value = (f32::from_bits(c.r(816, 4) as u32) * f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00000ad4 vstr.32 s12,[r5,#0x64]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(100u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x00000ad8 vmrs apsr,fpscr
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
        // 0x00000adc vdiv.f32 s27,s29,s10
        let value = (f32::from_bits(c.r(884, 4) as u32) / f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(876, 4, value);
    }
    {
        // 0x00000ae0 vstr.32 s10,[r1,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(808, 4));
    }
    {
        // 0x00000ae4 vstr.32 s11,[r5,#0x68]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(104u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(812, 4));
    }
    {
        // 0x00000ae8 vselgt.f32 s15,s15,s4
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(784, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(828, 4, value);
    }
    {
        // 0x00000aec vstr.32 s7,[r5,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(796, 4));
    }
    {
        // 0x00000af0 vdiv.f32 s12,s29,s11
        let value = (f32::from_bits(c.r(884, 4) as u32) / f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00000af4 vmul.f32 s14,s7,s14
        let value = (f32::from_bits(c.r(796, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00000af8 vmul.f32 s11,s17,s26
        let value = (f32::from_bits(c.r(836, 4) as u32) * f32::from_bits(c.r(872, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00000afc vmov.f32 s9,s29
        let value = c.r(884, 4);
        c.w(804, 4, value);
    }
    {
        // 0x00000b00 vstr.32 s27,[r1,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(876, 4));
    }
    {
        // 0x00000b04 vstr.32 s12,[r5,#0x6c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(108u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x00000b08 vldr.32 s12,[r1,#0x48]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(72u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x00000b0c vmul.f32 s12,s8,s12
        let value = (f32::from_bits(c.r(800, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00000b10 vldr.32 s8,[pc,#0x234]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((2832u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(564u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x00000b14 vstr.32 s12,[r1,#0x64]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(100u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x00000b18 vdiv.f32 s1,s29,s12
        let value = (f32::from_bits(c.r(884, 4) as u32) / f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(772, 4, value);
    }
    {
        // 0x00000b1c vdiv.f32 s12,s29,s13
        let value = (f32::from_bits(c.r(884, 4) as u32) / f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00000b20 vdiv.f32 s13,s29,s14
        let value = (f32::from_bits(c.r(884, 4) as u32) / f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00000b24 vstr.32 s1,[r1,#0x68]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(104u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(772, 4));
    }
    {
        // 0x00000b28 vstr.32 s12,[r1,#0xc8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(200u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x00000b2c vstr.32 s14,[r5,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x00000b30 ldr.w r1,[r5,#0xb4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(180u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00000b34 vldr.32 s3,[r5,#0xac]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(172u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(780, 4, value);
    }
    {
        // 0x00000b38 vmov s14,r1
        let value = c.r(36, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00000b3c str r1,[sp,#0x14]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(20u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x00000b3e vstr.32 s13,[r5,#0xc]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(12u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x00000b42 vcvt.f32.s32 s14,s14
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
        // 0x00000b46 ldr.w r1,[r5,#0x114]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(276u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00000b4a vldr.32 s12,[r5,#0x10c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(268u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x00000b4e str r1,[sp,#0x18]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(24u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x00000b50 vdiv.f32 s13,s14,s15
        let value = (f32::from_bits(c.r(824, 4) as u32) / f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00000b54 vldr.32 s14,[r5,#0x364]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(868u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00000b58 vldr.32 s15,[r5,#0x368]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(872u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00000b5c vldr.32 s7,[r5,#0x16c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(364u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x00000b60 ldr.w r0,[lr,#-0x8]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(88, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x00000b64 vadd.f32 s14,s0,s14
        let value = (f32::from_bits(c.r(768, 4) as u32) + f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00000b68 vsub.f32 s15,s15,s0
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00000b6c vcmpe.f32 s14,s4
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
        t35c00 = ((f32::from_bits(c.r(824, 4) as u32) < f32::from_bits(c.r(784, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(824, 4) as u32) == f32::from_bits(c.r(784, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(784, 4) as u32) <= f32::from_bits(c.r(824, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(824, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x00000b70 vmrs apsr,fpscr
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
        // 0x00000b74 vcmpe.f32 s15,s4
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
        t35c00 = ((f32::from_bits(c.r(828, 4) as u32) < f32::from_bits(c.r(784, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(828, 4) as u32) == f32::from_bits(c.r(784, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(784, 4) as u32) <= f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x00000b78 vselgt.f32 s14,s14,s4
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(784, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(824, 4, value);
    }
    {
        // 0x00000b7c vmrs apsr,fpscr
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
        // 0x00000b80 vstr.32 s13,[r5,#0xc4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(196u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x00000b84 vmul.f32 s13,s13,s3
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(780, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00000b88 vselgt.f32 s15,s15,s4
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(784, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(828, 4, value);
    }
    {
        // 0x00000b8c vdiv.f32 s3,s29,s13
        let value = (f32::from_bits(c.r(884, 4) as u32) / f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(780, 4, value);
    }
    {
        // 0x00000b90 vstr.32 s13,[r5,#0xc8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(200u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x00000b94 vmov s13,r1
        let value = c.r(36, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00000b98 ldr.w r1,[r5,#0x174]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(372u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00000b9c str r1,[sp,#0x1c]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x00000b9e vcvt.f32.s32 s6,s13
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
        c.w(792, 4, value);
    }
    {
        // 0x00000ba2 vdiv.f32 s13,s6,s14
        let value = (f32::from_bits(c.r(792, 4) as u32) / f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00000ba6 vstr.32 s3,[r5,#0xcc]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(204u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(780, 4));
    }
    {
        // 0x00000baa vmul.f32 s14,s13,s12
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00000bae vstr.32 s13,[r5,#0x124]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(292u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x00000bb2 vdiv.f32 s13,s29,s14
        let value = (f32::from_bits(c.r(884, 4) as u32) / f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00000bb6 vstr.32 s14,[r5,#0x128]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(296u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x00000bba vmov s14,r1
        let value = c.r(36, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00000bbe ldr.w r1,[lr,#-0x4]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(88, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00000bc2 str.w r0,[lr,#-0x4]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(88, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x00000bc6 str.w r1,[r11],#-0x8
        let mut t19c900 = 0u64;
        t19c900 = (c.r(76, 4)) & 0xffffffffu64;
        let value = (c.r(76, 4)).wrapping_sub(8u64);
        c.w(76, 4, value);
        c.store(((t19c900 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x00000bca ldr.w r0,[lr,#-0xc]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(88, 4)).wrapping_sub(12u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x00000bce ldr.w r1,[r6,#0xbd4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3028u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00000bd2 str.w r0,[lr,#-0x8]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(88, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x00000bd6 vstr.32 s11,[lr,#-0xc]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(88, 4)).wrapping_sub(12u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(812, 4));
    }
    {
        // 0x00000bda vstr.32 s13,[r5,#0x12c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(300u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x00000bde vcvt.f32.s32 s13,s14
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
        c.w(820, 4, value);
    }
    {
        // 0x00000be2 vdiv.f32 s14,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) / f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00000be6 vldr.32 s13,[lr,#0x14]
        let mut t64000 = 0u64;
        t64000 = ((c.r(88, 4)).wrapping_add(20u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00000bea vmul.f32 s15,s14,s7
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(796, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00000bee vstr.32 s14,[r5,#0x184]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(388u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x00000bf2 vldr.32 s7,[pc,#0x158]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((3058u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(344u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x00000bf6 vdiv.f32 s14,s29,s15
        let value = (f32::from_bits(c.r(884, 4) as u32) / f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00000bfa vstr.32 s15,[r5,#0x188]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(392u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x00000bfe vldr.32 s15,[lr,#-0x18]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(88, 4)).wrapping_sub(24u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00000c02 vadd.f32 s13,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) + f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00000c06 vstr.32 s13,[lr,#0x14]
        let mut t64000 = 0u64;
        t64000 = ((c.r(88, 4)).wrapping_add(20u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x00000c0a vstr.32 s14,[r5,#0x18c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(396u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x00000c0e str r1,[sp,#0x20]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x00000c10 ldr.w r1,[r6,#0xbd8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3032u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00000c14 ldr.w r0,[r6,#0xbd0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3024u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x00000c18 str r1,[sp,#0x24]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x00000c1a ldr.w r4,[r6,#0xbe0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3040u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x00000c1e ldr.w r1,[r6,#0xc24]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3108u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00000c22 strd r2,r3,[sp,#0x38]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(84, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00000c26 str r1,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    0xc28
}
pub fn b_c28(c: &mut Machine) -> u32 {
    {
        // 0x00000c28 vmov.f32 s10,0x3f000000
        let value = 1056964608u64;
        c.w(808, 4, value);
    }
    {
        // 0x00000c2c b 0x00000c3e
        return 0xc3eu32;
    }
    0xc2e
}
pub fn b_c2e(c: &mut Machine) -> u32 {
    {
        // 0x00000c2e movs r3,#0x0
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
        // 0x00000c30 str.w r3,[r6,#0xbdc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3036u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00000c34 ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00000c36 cmp r0,r3
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
        // 0x00000c38 bgt 0x00000cf2
        let mut t181b00 = 0u64;
        let mut t181c00 = 0u64;
        let mut t181e00 = 0u64;
        t181b00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t181c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t181e00 = (((((t181b00 >> 0) & 0xffu64) != 0) & (((t181c00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t181e00 >> 0) & 0xffu64) != 0 {
            return 0xcf2u32;
        }
    }
    0xc3a
}
pub fn b_c3a(c: &mut Machine) -> u32 {
    {
        // 0x00000c3a vldr.32 s13,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    0xc3e
}
pub fn b_c3e(c: &mut Machine) -> u32 {
    {
        // 0x00000c3e vcmpe.f32 s13,s9
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
        // 0x00000c42 vmrs apsr,fpscr
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
        // 0x00000c46 ble.w 0x00000d58
        let mut t184000 = 0u64;
        let mut t184200 = 0u64;
        t184000 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t184200 = (((c.r(97, 1) != 0) | (((t184000 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t184200 >> 0) & 0xffu64) != 0 {
            return 0xd58u32;
        }
    }
    0xc4a
}
pub fn b_c4a(c: &mut Machine) -> u32 {
    {
        // 0x00000c4a ldr r3,[0x00000d50]
        let value = c.load(0xd50, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00000c4c vsub.f32 s13,s13,s9
        let value = (f32::from_bits(c.r(820, 4) as u32) - f32::from_bits(c.r(804, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00000c50 ldr r1,[sp,#0x20]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00000c52 ldr r2,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00000c54 vldr.32 s15,[r3,#0x10]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00000c58 str r2,[r3,#0x4]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x00000c5a vcmpe.f32 s15,#0
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
        // 0x00000c5e ldr.w r2,[r3,#-0x4]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00000c62 vstr.32 s13,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x00000c66 str r2,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x00000c68 ldr.w r2,[r3,#-0x8]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00000c6c vmrs apsr,fpscr
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
        // 0x00000c70 str.w r2,[r3,#-0x4]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x00000c74 ldr.w r2,[r6,#0xbdc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3036u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00000c78 sub.w r1,r2,r1
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
        // 0x00000c7c bne 0x00000d02
        let mut t180400 = 0u64;
        t180400 = ((c.r(97, 1) == 0u64) as u64) & 0xffu64;
        if ((t180400 >> 0) & 0xffu64) != 0 {
            return 0xd02u32;
        }
    }
    0xc7e
}
pub fn b_c7e(c: &mut Machine) -> u32 {
    {
        // 0x00000c7e cmp r1,#0x0
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
        // 0x00000c80 it lt
        let mut t185c00 = 0u64;
        t185c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
    }
    {
        // 0x00000c82 add.lt r1,r1,r0
        let mut t1d00 = 0u64;
        let mut t2700 = 0u64;
        t1d00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2700 = ((((t1d00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0xc84u32;
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
    0xc84
}
pub fn b_c84(c: &mut Machine) -> u32 {
    {
        // 0x00000c84 add.w r1,r4,r1, lsl #0x2
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
        // 0x00000c88 ldr r1,[r1,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(36, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00000c8a str.w r1,[r3,#-0x8]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    0xc8e
}
pub fn b_c8e(c: &mut Machine) -> u32 {
    {
        // 0x00000c8e vldr.32 s6,[r11]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(76, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x00000c92 add.w r3,r4,r2, lsl #0x2
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
        // 0x00000c96 vldr.32 s12,[r10]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(72, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x00000c9a adds r2,#0x1
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
        // 0x00000c9c vldr.32 s15,[r8]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(64, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00000ca0 vsub.f32 s14,s6,s12
        let value = (f32::from_bits(c.r(792, 4) as u32) - f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00000ca4 vldr.32 s5,[r9]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(68, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(788, 4, value);
    }
    {
        // 0x00000ca8 vmul.f32 s13,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00000cac vldr.32 s11,[lr]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(88, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x00000cb0 vsub.f32 s12,s12,s5
        let value = (f32::from_bits(c.r(816, 4) as u32) - f32::from_bits(c.r(788, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00000cb4 cmp r0,r2
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
        // 0x00000cb6 vmov.f32 s15,s14
        let value = c.r(824, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00000cba vsub.f32 s11,s11,s6
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(792, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00000cbe vfma.f32 s15,s12,s10
        let value = f32::from_bits(c.r(816, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00000cc2 vadd.f32 s14,s14,s15
        let value = (f32::from_bits(c.r(824, 4) as u32) + f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00000cc6 vfma.f32 s14,s11,s10
        let value = f32::from_bits(c.r(812, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00000cca vadd.f32 s15,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00000cce vfnms.f32 s15,s13,s14
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
        // 0x00000cd2 vmov.f32 s14,s6
        let value = c.r(792, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00000cd6 vmul.f32 s15,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00000cda vfma.f32 s15,s12,s10
        let value = f32::from_bits(c.r(816, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00000cde vfma.f32 s14,s13,s15
        let value = f32::from_bits(c.r(820, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00000ce2 vstr.32 s14,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x00000ce6 ble 0x00000c2e
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0xc2eu32;
        }
    }
    0xce8
}
pub fn b_ce8(c: &mut Machine) -> u32 {
    {
        // 0x00000ce8 ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00000cea str.w r2,[r6,#0xbdc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3036u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x00000cee cmp r0,r3
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
        // 0x00000cf0 ble 0x00000c3a
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0xc3au32;
        }
    }
    0xcf2
}
pub fn b_cf2(c: &mut Machine) -> u32 {
    {
        // 0x00000cf2 ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00000cf4 vldr.32 s13,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00000cf8 adds r3,#0x1
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
        // 0x00000cfa str r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00000cfc str.w r3,[r6,#0xc24]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3108u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00000d00 b 0x00000c28
        return 0xc28u32;
    }
    0xd02
}
pub fn b_d02(c: &mut Machine) -> u32 {
    {
        // 0x00000d02 cmp r1,#0x0
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
        // 0x00000d04 ldr r7,[sp,#0x24]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(60, 4, value);
    }
    {
        // 0x00000d06 vsub.f32 s11,s15,s8
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(800, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00000d0a it lt
        let mut t185c00 = 0u64;
        t185c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
    }
    {
        // 0x00000d0c add.lt r1,r1,r0
        let mut t1d00 = 0u64;
        let mut t2700 = 0u64;
        t1d00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2700 = ((((t1d00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0xd0eu32;
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
    0xd0e
}
pub fn b_d0e(c: &mut Machine) -> u32 {
    {
        // 0x00000d0e subs r7,r2,r7
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
        // 0x00000d10 it mi
        let mut t184c00 = 0u64;
        t184c00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x00000d12 add.mi r7,r7,r0
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0xd14u32;
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
    0xd14
}
pub fn b_d14(c: &mut Machine) -> u32 {
    {
        // 0x00000d14 add.w r1,r4,r1, lsl #0x2
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
        // 0x00000d18 vcmpe.f32 s11,#0
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
        // 0x00000d1c add.w r7,r4,r7, lsl #0x2
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
        // 0x00000d20 vldr.32 s14,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00000d24 vldr.32 s12,[r7]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(60, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x00000d28 vmrs apsr,fpscr
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
        // 0x00000d2c vsub.f32 s12,s12,s14
        let value = (f32::from_bits(c.r(816, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00000d30 ite pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x00000d32 vstr.pl.32 s11,[r3,#0x10]
        let mut tf00 = 0u64;
        let mut t2600 = 0u64;
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2600 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2600 >> 0) & 0xffu64) != 0 {
            return 0xd36u32;
        }
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(812, 4));
    }
    0xd36
}
pub fn b_d36(c: &mut Machine) -> u32 {
    {
        // 0x00000d36 vstr.mi.32 s7,[r3,#0x10]
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0xd3au32;
        }
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(796, 4));
    }
    0xd3a
}
pub fn b_d3a(c: &mut Machine) -> u32 {
    {
        // 0x00000d3a vfma.f32 s14,s15,s12
        let value = f32::from_bits(c.r(828, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(816, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00000d3e vstr.32 s14,[r3,#-0x8]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(44, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x00000d42 b 0x00000c8e
        return 0xc8eu32;
    }
    0xd44
}
pub fn b_d58(c: &mut Machine) -> u32 {
    {
        // 0x00000d58 ldrb.w r1,[r6,#0xc18]
        let mut t19c800 = 0u64;
        let mut t1aa600 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3096u64)) & 0xffffffffu64;
        t1aa600 = (c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 1)) & 0xffu64;
        let value = ((t1aa600 >> 0) & 0xffu64);
        c.w(36, 4, value);
    }
    {
        // 0x00000d5c ldrd r2,r3,[sp,#0x38]
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
        // 0x00000d60 cbz r1,0x00000d86
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(36, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0xd86u32;
        }
    }
    0xd62
}
pub fn b_d62(c: &mut Machine) -> u32 {
    {
        // 0x00000d62 ldr r1,[0x0000108c]
        let value = c.load(0x108c, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00000d64 vmov.f32 s12,0x3f000000
        let value = 1056964608u64;
        c.w(816, 4, value);
    }
    {
        // 0x00000d68 vmov.f32 s14,0x3f800000
        let value = 1065353216u64;
        c.w(824, 4, value);
    }
    {
        // 0x00000d6c vldr.32 s15,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00000d70 vmul.f32 s15,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00000d74 vmul.f32 s13,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00000d78 vcmpe.f32 s13,s9
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
        // 0x00000d7c vmrs apsr,fpscr
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
        // 0x00000d80 it pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x00000d82 vmov.pl.f32 s13,s14
        let mut tf00 = 0u64;
        let mut t2700 = 0u64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2700 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0xd86u32;
        }
        let value = c.r(824, 4);
        c.w(820, 4, value);
    }
    0xd86
}
pub fn b_d86(c: &mut Machine) -> u32 {
    {
        // 0x00000d86 ldr r1,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00000d88 cmp r0,r1
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
        // 0x00000d8a bgt.w 0x000030be
        let mut t183c00 = 0u64;
        let mut t183d00 = 0u64;
        let mut t183f00 = 0u64;
        t183c00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t183d00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t183f00 = (((((t183c00 >> 0) & 0xffu64) != 0) & (((t183d00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t183f00 >> 0) & 0xffu64) != 0 {
            return 0x30beu32;
        }
    }
    0xd8e
}
pub fn b_d8e(c: &mut Machine) -> u32 {
    {
        // 0x00000d8e ldr r1,[0x00001090]
        let value = c.load(0x1090, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00000d90 vmov.f32 s10,0x3f000000
        let value = 1056964608u64;
        c.w(808, 4, value);
    }
    {
        // 0x00000d94 vldr.32 s15,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00000d98 vldr.32 s11,[r1,#-0x4]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x00000d9c vldr.32 s14,[r1,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00000da0 vsub.f32 s12,s15,s11
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00000da4 vldr.32 s9,[r1,#-0x8]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(804, 4, value);
    }
    {
        // 0x00000da8 vsub.f32 s11,s11,s14
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00000dac vsub.f32 s9,s9,s15
        let value = (f32::from_bits(c.r(804, 4) as u32) - f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(804, 4, value);
    }
    {
        // 0x00000db0 vmov.f32 s14,s12
        let value = c.r(816, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00000db4 vfma.f32 s14,s11,s10
        let value = f32::from_bits(c.r(812, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00000db8 vadd.f32 s12,s12,s14
        let value = (f32::from_bits(c.r(816, 4) as u32) + f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00000dbc vfma.f32 s12,s9,s10
        let value = f32::from_bits(c.r(804, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(816, 4) as u32),
            )
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00000dc0 vadd.f32 s14,s14,s12
        let value = (f32::from_bits(c.r(824, 4) as u32) + f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00000dc4 vfnms.f32 s14,s12,s13
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
        // 0x00000dc8 vmul.f32 s14,s13,s14
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00000dcc vfma.f32 s14,s11,s10
        let value = f32::from_bits(c.r(812, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00000dd0 vfma.f32 s15,s13,s14
        let value = f32::from_bits(c.r(820, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(824, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    0xdd4
}
pub fn b_dd4(c: &mut Machine) -> u32 {
    {
        // 0x00000dd4 ldr r1,[0x00001094]
        let value = c.load(0x1094, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00000dd6 vmov.f32 s9,0x3f800000
        let value = 1065353216u64;
        c.w(804, 4, value);
    }
    {
        // 0x00000dda ldr.w r4,[r6,#0xc98]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3224u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x00000dde vldr.32 s14,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00000de2 mov r0,r1
        let value = c.r(36, 4);
        c.w(32, 4, value);
    }
    {
        // 0x00000de4 vldr.32 s12,[r1,#-0x30]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(48u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x00000de8 vldr.32 s13,[r1,#-0xc]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(12u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00000dec sub.w r11,r0,#0x20
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
        // 0x00000df0 vfms.f32 s12,s14,s15
        let value = (-f32::from_bits(c.r(824, 4) as u32))
            .mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(816, 4) as u32),
            )
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00000df4 vldr.32 s11,[r1,#-0x38]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(56u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x00000df8 sub.w r10,r0,#0x24
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
        // 0x00000dfc sub.w r9,r0,#0x28
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
        // 0x00000e00 vadd.f32 s13,s13,s11
        let value = (f32::from_bits(c.r(820, 4) as u32) + f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00000e04 sub.w r8,r0,#0x2c
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
        // 0x00000e08 sub.w r12,r0,#0xc
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
        // 0x00000e0c ldr.w r7,[r6,#0xce4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3300u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(60, 4, value);
    }
    {
        // 0x00000e10 str r4,[sp,#0x24]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x00000e12 vstr.32 s13,[r1,#-0xc]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(12u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x00000e16 vfma.f32 s15,s14,s12
        let value = f32::from_bits(c.r(824, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(816, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00000e1a ldr.w r1,[r1,#-0x24]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(36, 4)).wrapping_sub(36u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00000e1e vldr.32 s8,[pc,#0x278]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((3614u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(632u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x00000e22 str.w r1,[r0,#-0x20]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(32, 4)).wrapping_sub(32u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x00000e26 ldr.w r1,[r0,#-0x28]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(32, 4)).wrapping_sub(40u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00000e2a ldr.w r4,[r6,#0xca0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3232u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x00000e2e str.w r1,[r0,#-0x24]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(32, 4)).wrapping_sub(36u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x00000e32 ldr.w r1,[r0,#-0x2c]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(32, 4)).wrapping_sub(44u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00000e36 vstr.32 s15,[r0,#-0x2c]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(32, 4)).wrapping_sub(44u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x00000e3a str.w r1,[r0,#-0x28]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(32, 4)).wrapping_sub(40u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x00000e3e mov r1,r0
        let value = c.r(32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00000e40 ldr.w r0,[r6,#0xc94]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3220u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x00000e44 vldr.32 s7,[pc,#0x254]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((3652u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(596u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x00000e48 sub.w lr,r1,#0x34
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
        // 0x00000e4c str r0,[sp,#0x20]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x00000e4e ldr.w r0,[r6,#0xc90]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3216u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x00000e52 vstr.32 s12,[sp,#0x40]
        let mut t64000 = 0u64;
        t64000 = ((c.r(84, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x00000e56 str r7,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(60, 4));
    }
    {
        // 0x00000e58 strd r2,r3,[sp,#0x38]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(84, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    0xe5c
}
pub fn b_e5c(c: &mut Machine) -> u32 {
    {
        // 0x00000e5c vmov.f32 s10,0x3f000000
        let value = 1056964608u64;
        c.w(808, 4, value);
    }
    {
        // 0x00000e60 b 0x00000edc
        return 0xedcu32;
    }
    0xe62
}
pub fn b_e62(c: &mut Machine) -> u32 {
    {
        // 0x00000e62 cmp r1,#0x0
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
        // 0x00000e64 it lt
        let mut t185c00 = 0u64;
        t185c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
    }
    {
        // 0x00000e66 add.lt r1,r1,r0
        let mut t1d00 = 0u64;
        let mut t2700 = 0u64;
        t1d00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2700 = ((((t1d00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0xe68u32;
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
    0xe68
}
pub fn b_e68(c: &mut Machine) -> u32 {
    {
        // 0x00000e68 add.w r1,r4,r1, lsl #0x2
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
        // 0x00000e6c ldr r1,[r1,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(36, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00000e6e str.w r1,[r3,#-0x8]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    0xe72
}
pub fn b_e72(c: &mut Machine) -> u32 {
    {
        // 0x00000e72 vldr.32 s6,[r9]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(68, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x00000e76 add.w r3,r4,r2, lsl #0x2
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
        // 0x00000e7a vldr.32 s4,[r10]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(72, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(784, 4, value);
    }
    {
        // 0x00000e7e adds r2,#0x1
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
        // 0x00000e80 vldr.32 s15,[lr]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(88, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00000e84 vsub.f32 s14,s6,s4
        let value = (f32::from_bits(c.r(792, 4) as u32) - f32::from_bits(c.r(784, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00000e88 vldr.32 s5,[r8]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(64, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(788, 4, value);
    }
    {
        // 0x00000e8c vmul.f32 s13,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00000e90 vldr.32 s11,[r11]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(76, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x00000e94 vsub.f32 s5,s4,s5
        let value = (f32::from_bits(c.r(784, 4) as u32) - f32::from_bits(c.r(788, 4) as u32))
            .to_bits() as u64;
        c.w(788, 4, value);
    }
    {
        // 0x00000e98 cmp r0,r2
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
        // 0x00000e9a vmov.f32 s15,s14
        let value = c.r(824, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00000e9e vsub.f32 s11,s11,s6
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(792, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00000ea2 vfma.f32 s15,s5,s10
        let value = f32::from_bits(c.r(788, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00000ea6 vadd.f32 s14,s14,s15
        let value = (f32::from_bits(c.r(824, 4) as u32) + f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00000eaa vfma.f32 s14,s11,s10
        let value = f32::from_bits(c.r(812, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00000eae vadd.f32 s15,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00000eb2 vfnms.f32 s15,s13,s14
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
        // 0x00000eb6 vmov.f32 s14,s6
        let value = c.r(792, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00000eba vmul.f32 s15,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00000ebe vfma.f32 s15,s5,s10
        let value = f32::from_bits(c.r(788, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00000ec2 vfma.f32 s14,s13,s15
        let value = f32::from_bits(c.r(820, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00000ec6 vstr.32 s14,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x00000eca bgt 0x00000f5c
        let mut t181b00 = 0u64;
        let mut t181c00 = 0u64;
        let mut t181e00 = 0u64;
        t181b00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t181c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t181e00 = (((((t181b00 >> 0) & 0xffu64) != 0) & (((t181c00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t181e00 >> 0) & 0xffu64) != 0 {
            return 0xf5cu32;
        }
    }
    0xecc
}
pub fn b_ecc(c: &mut Machine) -> u32 {
    {
        // 0x00000ecc movs r3,#0x0
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
        // 0x00000ece str.w r3,[r6,#0xc9c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3228u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00000ed2 ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00000ed4 cmp r0,r3
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
        // 0x00000ed6 bgt 0x00000f66
        let mut t181b00 = 0u64;
        let mut t181c00 = 0u64;
        let mut t181e00 = 0u64;
        t181b00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t181c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t181e00 = (((((t181b00 >> 0) & 0xffu64) != 0) & (((t181c00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t181e00 >> 0) & 0xffu64) != 0 {
            return 0xf66u32;
        }
    }
    0xed8
}
pub fn b_ed8(c: &mut Machine) -> u32 {
    {
        // 0x00000ed8 vldr.32 s13,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    0xedc
}
pub fn b_edc(c: &mut Machine) -> u32 {
    {
        // 0x00000edc vcmpe.f32 s13,s9
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
        // 0x00000ee0 vmrs apsr,fpscr
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
        // 0x00000ee4 ble 0x00000f76
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0xf76u32;
        }
    }
    0xee6
}
pub fn b_ee6(c: &mut Machine) -> u32 {
    {
        // 0x00000ee6 ldr r3,[0x000010a0]
        let value = c.load(0x10a0, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00000ee8 vsub.f32 s13,s13,s9
        let value = (f32::from_bits(c.r(820, 4) as u32) - f32::from_bits(c.r(804, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00000eec ldr r1,[sp,#0x20]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00000eee ldr r2,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00000ef0 vldr.32 s15,[r3,#0x10]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00000ef4 str r2,[r3,#0x4]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x00000ef6 vcmpe.f32 s15,#0
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
        // 0x00000efa ldr.w r2,[r3,#-0x4]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00000efe vstr.32 s13,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x00000f02 str r2,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x00000f04 ldr.w r2,[r3,#-0x8]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00000f08 vmrs apsr,fpscr
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
        // 0x00000f0c str.w r2,[r3,#-0x4]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x00000f10 ldr.w r2,[r6,#0xc9c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3228u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00000f14 sub.w r1,r2,r1
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
        // 0x00000f18 beq 0x00000e62
        let mut t180200 = 0u64;
        t180200 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        if ((t180200 >> 0) & 0xffu64) != 0 {
            return 0xe62u32;
        }
    }
    0xf1a
}
pub fn b_f1a(c: &mut Machine) -> u32 {
    {
        // 0x00000f1a cmp r1,#0x0
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
        // 0x00000f1c ldr r7,[sp,#0x24]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(60, 4, value);
    }
    {
        // 0x00000f1e vsub.f32 s6,s15,s8
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(800, 4) as u32))
            .to_bits() as u64;
        c.w(792, 4, value);
    }
    {
        // 0x00000f22 it lt
        let mut t185c00 = 0u64;
        t185c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
    }
    {
        // 0x00000f24 add.lt r1,r1,r0
        let mut t1d00 = 0u64;
        let mut t2700 = 0u64;
        t1d00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2700 = ((((t1d00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0xf26u32;
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
    0xf26
}
pub fn b_f26(c: &mut Machine) -> u32 {
    {
        // 0x00000f26 subs r7,r2,r7
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
        // 0x00000f28 it mi
        let mut t184c00 = 0u64;
        t184c00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x00000f2a add.mi r7,r7,r0
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0xf2cu32;
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
    0xf2c
}
pub fn b_f2c(c: &mut Machine) -> u32 {
    {
        // 0x00000f2c add.w r1,r4,r1, lsl #0x2
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
        // 0x00000f30 vcmpe.f32 s6,#0
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
        // 0x00000f34 add.w r7,r4,r7, lsl #0x2
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
        // 0x00000f38 vldr.32 s14,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00000f3c vldr.32 s11,[r7]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(60, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x00000f40 vmrs apsr,fpscr
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
        // 0x00000f44 vsub.f32 s11,s11,s14
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00000f48 ite pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x00000f4a vstr.pl.32 s6,[r3,#0x10]
        let mut tf00 = 0u64;
        let mut t2600 = 0u64;
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2600 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2600 >> 0) & 0xffu64) != 0 {
            return 0xf4eu32;
        }
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(792, 4));
    }
    0xf4e
}
pub fn b_f4e(c: &mut Machine) -> u32 {
    {
        // 0x00000f4e vstr.mi.32 s7,[r3,#0x10]
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0xf52u32;
        }
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(796, 4));
    }
    0xf52
}
pub fn b_f52(c: &mut Machine) -> u32 {
    {
        // 0x00000f52 vfma.f32 s14,s15,s11
        let value = f32::from_bits(c.r(828, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(812, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00000f56 vstr.32 s14,[r3,#-0x8]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(44, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x00000f5a b 0x00000e72
        return 0xe72u32;
    }
    0xf5c
}
pub fn b_f5c(c: &mut Machine) -> u32 {
    {
        // 0x00000f5c ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00000f5e str.w r2,[r6,#0xc9c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3228u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x00000f62 cmp r0,r3
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
        // 0x00000f64 ble 0x00000ed8
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0xed8u32;
        }
    }
    0xf66
}
pub fn b_f66(c: &mut Machine) -> u32 {
    {
        // 0x00000f66 ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00000f68 vldr.32 s13,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00000f6c adds r3,#0x1
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
        // 0x00000f6e str r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00000f70 str.w r3,[r6,#0xce4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3300u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00000f74 b 0x00000e5c
        return 0xe5cu32;
    }
    0xf76
}
pub fn b_f76(c: &mut Machine) -> u32 {
    {
        // 0x00000f76 ldrb.w r1,[r6,#0xcd8]
        let mut t19c800 = 0u64;
        let mut t1aa600 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3288u64)) & 0xffffffffu64;
        t1aa600 = (c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 1)) & 0xffu64;
        let value = ((t1aa600 >> 0) & 0xffu64);
        c.w(36, 4, value);
    }
    {
        // 0x00000f7a ldrd r2,r3,[sp,#0x38]
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
        // 0x00000f7e cbz r1,0x00000fa4
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(36, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0xfa4u32;
        }
    }
    0xf80
}
pub fn b_f80(c: &mut Machine) -> u32 {
    {
        // 0x00000f80 ldr r1,[0x000010a4]
        let value = c.load(0x10a4, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00000f82 vmov.f32 s11,0x3f000000
        let value = 1056964608u64;
        c.w(812, 4, value);
    }
    {
        // 0x00000f86 vmov.f32 s14,0x3f800000
        let value = 1065353216u64;
        c.w(824, 4, value);
    }
    {
        // 0x00000f8a vldr.32 s15,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00000f8e vmul.f32 s15,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00000f92 vmul.f32 s13,s15,s11
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00000f96 vcmpe.f32 s13,s9
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
        // 0x00000f9a vmrs apsr,fpscr
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
        // 0x00000f9e it pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x00000fa0 vmov.pl.f32 s13,s14
        let mut tf00 = 0u64;
        let mut t2700 = 0u64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2700 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0xfa4u32;
        }
        let value = c.r(824, 4);
        c.w(820, 4, value);
    }
    0xfa4
}
pub fn b_fa4(c: &mut Machine) -> u32 {
    {
        // 0x00000fa4 ldr r1,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00000fa6 cmp r0,r1
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
        // 0x00000fa8 bgt.w 0x000030e4
        let mut t183c00 = 0u64;
        let mut t183d00 = 0u64;
        let mut t183f00 = 0u64;
        t183c00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t183d00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t183f00 = (((((t183c00 >> 0) & 0xffu64) != 0) & (((t183d00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t183f00 >> 0) & 0xffu64) != 0 {
            return 0x30e4u32;
        }
    }
    0xfac
}
pub fn b_fac(c: &mut Machine) -> u32 {
    {
        // 0x00000fac ldr r1,[0x000010a0]
        let value = c.load(0x10a0, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00000fae vmov.f32 s9,0x3f000000
        let value = 1056964608u64;
        c.w(804, 4, value);
    }
    {
        // 0x00000fb2 vldr.32 s14,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00000fb6 vldr.32 s10,[r1,#-0x4]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x00000fba vldr.32 s15,[r1,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00000fbe vsub.f32 s11,s14,s10
        let value = (f32::from_bits(c.r(824, 4) as u32) - f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00000fc2 vldr.32 s8,[r1,#-0x8]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x00000fc6 vsub.f32 s10,s10,s15
        let value = (f32::from_bits(c.r(808, 4) as u32) - f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x00000fca vsub.f32 s8,s8,s14
        let value = (f32::from_bits(c.r(800, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(800, 4, value);
    }
    {
        // 0x00000fce vmov.f32 s15,s11
        let value = c.r(812, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00000fd2 vfma.f32 s15,s10,s9
        let value = f32::from_bits(c.r(808, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00000fd6 vadd.f32 s11,s11,s15
        let value = (f32::from_bits(c.r(812, 4) as u32) + f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00000fda vfma.f32 s11,s8,s9
        let value = f32::from_bits(c.r(800, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(812, 4) as u32),
            )
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00000fde vadd.f32 s15,s15,s11
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00000fe2 vfnms.f32 s15,s11,s13
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
        // 0x00000fe6 vmul.f32 s15,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00000fea vfma.f32 s15,s10,s9
        let value = f32::from_bits(c.r(808, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00000fee vfma.f32 s14,s13,s15
        let value = f32::from_bits(c.r(820, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00000ff2 vadd.f32 s15,s14,s23
        let value = (f32::from_bits(c.r(824, 4) as u32) + f32::from_bits(c.r(860, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    0xff6
}
pub fn b_ff6(c: &mut Machine) -> u32 {
    {
        // 0x00000ff6 ldr r1,[0x00001094]
        let value = c.load(0x1094, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00000ff8 vmov.f32 s9,0x3f800000
        let value = 1065353216u64;
        c.w(804, 4, value);
    }
    {
        // 0x00000ffc ldr.w r4,[r6,#0xd58]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3416u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x00001000 vldr.32 s14,[r1,#0xc0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(192u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00001004 adds r1,#0xc0
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
        // 0x00001006 vstr.32 s15,[r1,#-0xf0]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(240u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x0000100a vldr.32 s15,[r1,#-0x30]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(48u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x0000100e vldr.32 s13,[r1,#-0xc]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(12u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00001012 vfms.f32 s15,s14,s12
        let value = (-f32::from_bits(c.r(824, 4) as u32))
            .mul_add(
                f32::from_bits(c.r(816, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00001016 vldr.32 s11,[r1,#-0x38]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(56u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x0000101a mov r0,r1
        let value = c.r(36, 4);
        c.w(32, 4, value);
    }
    {
        // 0x0000101c ldr.w r7,[r6,#0xda4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3492u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(60, 4, value);
    }
    {
        // 0x00001020 vadd.f32 s13,s13,s11
        let value = (f32::from_bits(c.r(820, 4) as u32) + f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00001024 str r4,[sp,#0x24]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x00001026 sub.w r11,r0,#0x20
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
        // 0x0000102a sub.w r10,r0,#0x24
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
        // 0x0000102e sub.w r9,r0,#0x28
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
        // 0x00001032 sub.w r8,r0,#0x2c
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
        // 0x00001036 vstr.32 s13,[r1,#-0xc]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(36, 4)).wrapping_sub(12u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x0000103a sub.w r12,r0,#0xc
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
        // 0x0000103e ldr.w r1,[r1,#-0x24]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(36, 4)).wrapping_sub(36u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001042 vfma.f32 s12,s14,s15
        let value = f32::from_bits(c.r(824, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(816, 4) as u32),
            )
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00001046 vldr.32 s8,[pc,#0x50]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((4166u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(80u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x0000104a str.w r1,[r0,#-0x20]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(32, 4)).wrapping_sub(32u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x0000104e ldr.w r1,[r0,#-0x28]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(32, 4)).wrapping_sub(40u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001052 ldr.w r4,[r6,#0xd60]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3424u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x00001056 str.w r1,[r0,#-0x24]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(32, 4)).wrapping_sub(36u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x0000105a ldr.w r1,[r0,#-0x2c]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(32, 4)).wrapping_sub(44u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x0000105e vstr.32 s12,[r0,#-0x2c]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(32, 4)).wrapping_sub(44u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x00001062 str.w r1,[r0,#-0x28]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(32, 4)).wrapping_sub(40u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x00001066 mov r1,r0
        let value = c.r(32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001068 ldr.w r0,[r6,#0xd54]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3412u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x0000106c vldr.32 s7,[pc,#0x2c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((4204u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(44u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x00001070 sub.w lr,r1,#0x34
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
        // 0x00001074 str r0,[sp,#0x20]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x00001076 ldr.w r0,[r6,#0xd50]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3408u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x0000107a vstr.32 s15,[sp,#0x40]
        let mut t64000 = 0u64;
        t64000 = ((c.r(84, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x0000107e str r7,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(60, 4));
    }
    {
        // 0x00001080 strd r2,r3,[sp,#0x38]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(84, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    0x1084
}
pub fn b_1084(c: &mut Machine) -> u32 {
    {
        // 0x00001084 vmov.f32 s10,0x3f000000
        let value = 1056964608u64;
        c.w(808, 4, value);
    }
    {
        // 0x00001088 b 0x000010b8
        return 0x10b8u32;
    }
    0x108a
}
pub fn b_10a8(c: &mut Machine) -> u32 {
    {
        // 0x000010a8 movs r3,#0x0
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
        // 0x000010aa str.w r3,[r6,#0xd5c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3420u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x000010ae ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x000010b0 cmp r0,r3
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
        // 0x000010b2 bgt 0x0000116e
        let mut t181b00 = 0u64;
        let mut t181c00 = 0u64;
        let mut t181e00 = 0u64;
        t181b00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t181c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t181e00 = (((((t181b00 >> 0) & 0xffu64) != 0) & (((t181c00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t181e00 >> 0) & 0xffu64) != 0 {
            return 0x116eu32;
        }
    }
    0x10b4
}
pub fn b_10b4(c: &mut Machine) -> u32 {
    {
        // 0x000010b4 vldr.32 s13,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    0x10b8
}
pub fn b_10b8(c: &mut Machine) -> u32 {
    {
        // 0x000010b8 vcmpe.f32 s13,s9
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
        // 0x000010bc vmrs apsr,fpscr
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
        // 0x000010c0 ble 0x000011c0
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x11c0u32;
        }
    }
    0x10c2
}
pub fn b_10c2(c: &mut Machine) -> u32 {
    {
        // 0x000010c2 ldr r3,[0x000013e8]
        let value = c.load(0x13e8, 4);
        c.w(44, 4, value);
    }
    {
        // 0x000010c4 vsub.f32 s13,s13,s9
        let value = (f32::from_bits(c.r(820, 4) as u32) - f32::from_bits(c.r(804, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x000010c8 ldr r1,[sp,#0x20]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x000010ca ldr r2,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x000010cc vldr.32 s14,[r3,#0x10]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x000010d0 str r2,[r3,#0x4]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x000010d2 vcmpe.f32 s14,#0
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
        // 0x000010d6 ldr.w r2,[r3,#-0x4]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x000010da vstr.32 s13,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x000010de str r2,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x000010e0 ldr.w r2,[r3,#-0x8]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x000010e4 vmrs apsr,fpscr
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
        // 0x000010e8 str.w r2,[r3,#-0x4]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x000010ec ldr.w r2,[r6,#0xd5c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3420u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x000010f0 sub.w r1,r2,r1
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
        // 0x000010f4 bne 0x0000117e
        let mut t180400 = 0u64;
        t180400 = ((c.r(97, 1) == 0u64) as u64) & 0xffu64;
        if ((t180400 >> 0) & 0xffu64) != 0 {
            return 0x117eu32;
        }
    }
    0x10f6
}
pub fn b_10f6(c: &mut Machine) -> u32 {
    {
        // 0x000010f6 cmp r1,#0x0
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
        // 0x000010f8 it lt
        let mut t185c00 = 0u64;
        t185c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
    }
    {
        // 0x000010fa add.lt r1,r1,r0
        let mut t1d00 = 0u64;
        let mut t2700 = 0u64;
        t1d00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2700 = ((((t1d00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x10fcu32;
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
    0x10fc
}
pub fn b_10fc(c: &mut Machine) -> u32 {
    {
        // 0x000010fc add.w r1,r4,r1, lsl #0x2
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
        // 0x00001100 ldr r1,[r1,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(36, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00001102 str.w r1,[r3,#-0x8]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(44, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    0x1106
}
pub fn b_1106(c: &mut Machine) -> u32 {
    {
        // 0x00001106 vldr.32 s6,[r9]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(68, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x0000110a add.w r3,r4,r2, lsl #0x2
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
        // 0x0000110e vldr.32 s12,[r10]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(72, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x00001112 adds r2,#0x1
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
        // 0x00001114 vldr.32 s5,[lr]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(88, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(788, 4, value);
    }
    {
        // 0x00001118 vsub.f32 s14,s6,s12
        let value = (f32::from_bits(c.r(792, 4) as u32) - f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x0000111c vldr.32 s4,[r8]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(64, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(784, 4, value);
    }
    {
        // 0x00001120 vmul.f32 s13,s13,s5
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(788, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00001124 vldr.32 s11,[r11]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(76, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x00001128 vsub.f32 s12,s12,s4
        let value = (f32::from_bits(c.r(816, 4) as u32) - f32::from_bits(c.r(784, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x0000112c cmp r0,r2
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
        // 0x0000112e vmov.f32 s5,s14
        let value = c.r(824, 4);
        c.w(788, 4, value);
    }
    {
        // 0x00001132 vsub.f32 s11,s11,s6
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(792, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00001136 vfma.f32 s5,s12,s10
        let value = f32::from_bits(c.r(816, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(788, 4) as u32),
            )
            .to_bits() as u64;
        c.w(788, 4, value);
    }
    {
        // 0x0000113a vadd.f32 s14,s14,s5
        let value = (f32::from_bits(c.r(824, 4) as u32) + f32::from_bits(c.r(788, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x0000113e vfma.f32 s14,s11,s10
        let value = f32::from_bits(c.r(812, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001142 vadd.f32 s11,s5,s14
        let value = (f32::from_bits(c.r(788, 4) as u32) + f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00001146 vfnms.f32 s11,s13,s14
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
        // 0x0000114a vmul.f32 s14,s13,s11
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x0000114e vfma.f32 s14,s12,s10
        let value = f32::from_bits(c.r(816, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00001152 vmov.f32 s12,s14
        let value = c.r(824, 4);
        c.w(816, 4, value);
    }
    {
        // 0x00001156 vmov.f32 s14,s6
        let value = c.r(792, 4);
        c.w(824, 4, value);
    }
    {
        // 0x0000115a vfma.f32 s14,s13,s12
        let value = f32::from_bits(c.r(820, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(816, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x0000115e vstr.32 s14,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x00001162 ble 0x000010a8
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x10a8u32;
        }
    }
    0x1164
}
pub fn b_1164(c: &mut Machine) -> u32 {
    {
        // 0x00001164 ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00001166 str.w r2,[r6,#0xd5c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3420u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x0000116a cmp r0,r3
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
        // 0x0000116c ble 0x000010b4
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x10b4u32;
        }
    }
    0x116e
}
pub fn b_116e(c: &mut Machine) -> u32 {
    {
        // 0x0000116e ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00001170 vldr.32 s13,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00001174 adds r3,#0x1
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
        // 0x00001176 str r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00001178 str.w r3,[r6,#0xda4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3492u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x0000117c b 0x00001084
        return 0x1084u32;
    }
    0x117e
}
pub fn b_117e(c: &mut Machine) -> u32 {
    {
        // 0x0000117e cmp r1,#0x0
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
        // 0x00001180 ldr r7,[sp,#0x24]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(60, 4, value);
    }
    {
        // 0x00001182 vsub.f32 s6,s14,s8
        let value = (f32::from_bits(c.r(824, 4) as u32) - f32::from_bits(c.r(800, 4) as u32))
            .to_bits() as u64;
        c.w(792, 4, value);
    }
    {
        // 0x00001186 it lt
        let mut t185c00 = 0u64;
        t185c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
    }
    {
        // 0x00001188 add.lt r1,r1,r0
        let mut t1d00 = 0u64;
        let mut t2700 = 0u64;
        t1d00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2700 = ((((t1d00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x118au32;
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
    0x118a
}
pub fn b_118a(c: &mut Machine) -> u32 {
    {
        // 0x0000118a subs r7,r2,r7
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
        // 0x0000118c it mi
        let mut t184c00 = 0u64;
        t184c00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x0000118e add.mi r7,r7,r0
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x1190u32;
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
    0x1190
}
pub fn b_1190(c: &mut Machine) -> u32 {
    {
        // 0x00001190 add.w r1,r4,r1, lsl #0x2
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
        // 0x00001194 vcmpe.f32 s6,#0
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
        // 0x00001198 add.w r7,r4,r7, lsl #0x2
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
        // 0x0000119c vldr.32 s12,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x000011a0 vldr.32 s11,[r7]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(60, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x000011a4 vmrs apsr,fpscr
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
        // 0x000011a8 vsub.f32 s11,s11,s12
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x000011ac ite pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x000011ae vstr.pl.32 s6,[r3,#0x10]
        let mut tf00 = 0u64;
        let mut t2600 = 0u64;
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2600 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2600 >> 0) & 0xffu64) != 0 {
            return 0x11b2u32;
        }
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(792, 4));
    }
    0x11b2
}
pub fn b_11b2(c: &mut Machine) -> u32 {
    {
        // 0x000011b2 vstr.mi.32 s7,[r3,#0x10]
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x11b6u32;
        }
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(796, 4));
    }
    0x11b6
}
pub fn b_11b6(c: &mut Machine) -> u32 {
    {
        // 0x000011b6 vfma.f32 s12,s14,s11
        let value = f32::from_bits(c.r(824, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(812, 4) as u32),
                f32::from_bits(c.r(816, 4) as u32),
            )
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x000011ba vstr.32 s12,[r3,#-0x8]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(44, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        c.store(((t63e00 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x000011be b 0x00001106
        return 0x1106u32;
    }
    0x11c0
}
pub fn b_11c0(c: &mut Machine) -> u32 {
    {
        // 0x000011c0 ldrb.w r1,[r6,#0xd98]
        let mut t19c800 = 0u64;
        let mut t1aa600 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(3480u64)) & 0xffffffffu64;
        t1aa600 = (c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 1)) & 0xffu64;
        let value = ((t1aa600 >> 0) & 0xffu64);
        c.w(36, 4, value);
    }
    {
        // 0x000011c4 ldrd r2,r3,[sp,#0x38]
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
        // 0x000011c8 cbz r1,0x000011ee
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(36, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x11eeu32;
        }
    }
    0x11ca
}
pub fn b_11ca(c: &mut Machine) -> u32 {
    {
        // 0x000011ca ldr r1,[0x000013ec]
        let value = c.load(0x13ec, 4);
        c.w(36, 4, value);
    }
    {
        // 0x000011cc vmov.f32 s11,0x3f000000
        let value = 1056964608u64;
        c.w(812, 4, value);
    }
    {
        // 0x000011d0 vmov.f32 s12,0x3f800000
        let value = 1065353216u64;
        c.w(816, 4, value);
    }
    {
        // 0x000011d4 vldr.32 s14,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x000011d8 vmul.f32 s14,s13,s14
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x000011dc vmul.f32 s13,s14,s11
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x000011e0 vcmpe.f32 s13,s9
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
        // 0x000011e4 vmrs apsr,fpscr
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
        // 0x000011e8 it pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x000011ea vmov.pl.f32 s13,s12
        let mut tf00 = 0u64;
        let mut t2700 = 0u64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2700 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x11eeu32;
        }
        let value = c.r(816, 4);
        c.w(820, 4, value);
    }
    0x11ee
}
