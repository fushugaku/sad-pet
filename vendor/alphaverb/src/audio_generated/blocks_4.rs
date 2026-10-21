#![allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut,
    unused_comparisons
)]
use crate::machine::{signed, signed_overflow, FastFma, Machine};
pub fn b_3390(c: &mut Machine) -> u32 {
    {
        // 0x00003390 ldr.w pc,[0x00003394]
        let mut t0 = 0u64;
        let value = c.load(0x3394, 4);
        c.w(92, 4, value);
        t0 = (c.r(92, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(92, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x3394
}
pub fn b_33b0(c: &mut Machine) -> u32 {
    {
        // 0x000033b0 ldr.w pc,[0x000033b4]
        let mut t0 = 0u64;
        let value = c.load(0x33b4, 4);
        c.w(92, 4, value);
        t0 = (c.r(92, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(92, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x33b4
}
pub fn b_33c8(c: &mut Machine) -> u32 {
    {
        // 0x000033c8 ldr.w pc,[0x000033cc]
        let mut t0 = 0u64;
        let value = c.load(0x33cc, 4);
        c.w(92, 4, value);
        t0 = (c.r(92, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(92, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x33cc
}
pub fn b_33e8(c: &mut Machine) -> u32 {
    {
        // 0x000033e8 ldr.w pc,[0x000033ec]
        let mut t0 = 0u64;
        let value = c.load(0x33ec, 4);
        c.w(92, 4, value);
        t0 = (c.r(92, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(92, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x33ec
}
pub fn b_33f8(c: &mut Machine) -> u32 {
    {
        // 0x000033f8 ldr.w pc,[0x000033fc]
        let mut t0 = 0u64;
        let value = c.load(0x33fc, 4);
        c.w(92, 4, value);
        t0 = (c.r(92, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(92, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x33fc
}
pub fn b_3420(c: &mut Machine) -> u32 {
    {
        // 0x00003420 ldr.w pc,[0x00003424]
        let mut t0 = 0u64;
        let value = c.load(0x3424, 4);
        c.w(92, 4, value);
        t0 = (c.r(92, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(92, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x3424
}
pub fn b_60020c98(c: &mut Machine) -> u32 {
    {
        // 0x60020c98 ldr r2,[0x60020cc8]
        let value = c.load(0x60020cc8, 4);
        c.w(40, 4, value);
    }
    {
        // 0x60020c9a push {r3,lr}
        let value = c.r(84, 4);
        c.w(128, 4, value);
        let value = (c.r(128, 4)).wrapping_sub(4u64);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 4, c.r(88, 4));
        let value = (c.r(128, 4)).wrapping_sub(4u64);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 4, c.r(44, 4));
        let value = c.r(128, 4);
        c.w(84, 4, value);
    }
    {
        // 0x60020c9c mov r3,r0
        let value = c.r(32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x60020c9e ldr r0,[r2,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(40, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x60020ca0 cbz r0,0x60020cbe
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x60020cbeu32;
        }
    }
    0x60020ca2
}
pub fn b_60020ca2(c: &mut Machine) -> u32 {
    {
        // 0x60020ca2 add r3,r0
        let value = (c.r(44, 4)).wrapping_add(c.r(32, 4));
        c.w(44, 4, value);
    }
    {
        // 0x60020ca4 ldr r1,[0x60020ccc]
        let value = c.load(0x60020ccc, 4);
        c.w(36, 4, value);
    }
    {
        // 0x60020ca6 cmp r3,r1
        let mut t1a6e00 = 0u64;
        let value = (c.r(36, 4) <= c.r(44, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), c.r(36, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(44, 4)).wrapping_sub(c.r(36, 4))) & 0xffffffffu64;
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
        // 0x60020ca8 bhi 0x60020cae
        let mut t181100 = 0u64;
        let mut t181300 = 0u64;
        t181100 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t181300 = (((c.r(98, 1) != 0) & (((t181100 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t181300 >> 0) & 0xffu64) != 0 {
            return 0x60020caeu32;
        }
    }
    0x60020caa
}
pub fn b_60020caa(c: &mut Machine) -> u32 {
    {
        // 0x60020caa str r3,[r2,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(40, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x60020cac pop {r3,pc}
        let mut t0 = 0u64;
        let value = c.r(84, 4);
        c.w(128, 4, value);
        let value = c.load(c.r(128, 4) as u32, 4);
        c.w(44, 4, value);
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
    0x60020cae
}
pub fn b_60020cae(c: &mut Machine) -> u32 {
    {
        // 0x60020cae bl 0x600419c4
        let value = (1610747058u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600419c4u32;
    }
    0x60020cb2
}
pub fn b_60020cb2(c: &mut Machine) -> u32 {
    {
        // 0x60020cb2 movs r2,#0xc
        let value = 12u64;
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
        // 0x60020cb4 mov r3,r0
        let value = c.r(32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x60020cb6 mov.w r0,#0xffffffff
        let mut t188000 = 0u64;
        t188000 = (4294967295u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = ((t188000 >> 0) & 0xffffffffu64);
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60020cba str r2,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x60020cbc pop {r3,pc}
        let mut t0 = 0u64;
        let value = c.r(84, 4);
        c.w(128, 4, value);
        let value = c.load(c.r(128, 4) as u32, 4);
        c.w(44, 4, value);
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
    0x60020cbe
}
pub fn b_60020cbe(c: &mut Machine) -> u32 {
    {
        // 0x60020cbe ldr r1,[0x60020cd0]
        let value = c.load(0x60020cd0, 4);
        c.w(36, 4, value);
    }
    {
        // 0x60020cc0 mov r0,r1
        let value = c.r(36, 4);
        c.w(32, 4, value);
    }
    {
        // 0x60020cc2 str r1,[r2,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(40, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x60020cc4 b 0x60020ca2
        return 0x60020ca2u32;
    }
    0x60020cc6
}
pub fn b_60028dac(c: &mut Machine) -> u32 {
    {
        // 0x60028dac vldr.32 s15,[pc,#0xa4]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610780076u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(164u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x60028db0 addw r2,r0,#0xbc8
        let value = ((c.r(32, 4) as u128 + 3016u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 3016u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(3016u64);
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60028db4 vldr.32 s11,[r0,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x60028db8 vmov.f32 s8,0x3f800000
        let value = 1065353216u64;
        c.w(800, 4, value);
    }
    {
        // 0x60028dbc vcmpe.f32 s0,s15
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
        t35c00 = ((f32::from_bits(c.r(768, 4) as u32) < f32::from_bits(c.r(828, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(768, 4) as u32) == f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(828, 4) as u32) <= f32::from_bits(c.r(768, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(768, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x60028dc0 vldr.32 s12,[r0,#0xb4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(180u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x60028dc4 ldr.w r3,[r0,#0xbd4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(3028u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x60028dc8 vldr.32 s7,[pc,#0x8c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610780104u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(140u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x60028dcc vmrs apsr,fpscr
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
        // 0x60028dd0 vmov s14,r3
        let value = c.r(44, 4);
        c.w(824, 4, value);
    }
    {
        // 0x60028dd4 ldr.w r3,[r0,#0xc34]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(3124u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x60028dd8 vcvt.f32.s32 s9,s14
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
        c.w(804, 4, value);
    }
    {
        // 0x60028ddc vldr.32 s13,[r2,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x60028de0 vselgt.f32 s15,s0,s15
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(828, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(828, 4, value);
    }
    {
        // 0x60028de4 vcmpe.f32 s11,s15
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
        // 0x60028de8 vmov s14,r3
        let value = c.r(44, 4);
        c.w(824, 4, value);
    }
    {
        // 0x60028dec addw r3,r0,#0xc28
        let value = ((c.r(32, 4) as u128 + 3112u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 3112u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(3112u64);
        c.w(44, 4, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60028df0 vmrs apsr,fpscr
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
        // 0x60028df4 vcmpe.f32 s12,s15
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
        // 0x60028df8 vcvt.f32.s32 s10,s14
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
        // 0x60028dfc vldr.32 s14,[r3,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x60028e00 vselgt.f32 s11,s15,s11
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(812, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(812, 4, value);
    }
    {
        // 0x60028e04 vstr.32 s11,[r0,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(812, 4));
    }
    {
        // 0x60028e08 vmrs apsr,fpscr
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
        // 0x60028e0c vcmpe.f32 s0,s7
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
        t35c00 = ((f32::from_bits(c.r(768, 4) as u32) < f32::from_bits(c.r(796, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(768, 4) as u32) == f32::from_bits(c.r(796, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(796, 4) as u32) <= f32::from_bits(c.r(768, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(768, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x60028e10 vselgt.f32 s12,s15,s12
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
        // 0x60028e14 vstr.32 s12,[r0,#0xb8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(184u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x60028e18 vmrs apsr,fpscr
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
        // 0x60028e1c vselgt.f32 s0,s0,s7
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(796, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(768, 4, value);
    }
    {
        // 0x60028e20 vdiv.f32 s15,s9,s0
        let value = (f32::from_bits(c.r(804, 4) as u32) / f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60028e24 vdiv.f32 s12,s10,s0
        let value = (f32::from_bits(c.r(808, 4) as u32) / f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x60028e28 vstr.32 s15,[r2,#0x1c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60028e2c vmul.f32 s13,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x60028e30 vdiv.f32 s11,s8,s13
        let value = (f32::from_bits(c.r(800, 4) as u32) / f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x60028e34 vstr.32 s13,[r2,#0x20]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x60028e38 vmul.f32 s15,s12,s14
        let value = (f32::from_bits(c.r(816, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60028e3c vdiv.f32 s14,s8,s15
        let value = (f32::from_bits(c.r(800, 4) as u32) / f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x60028e40 vstr.32 s11,[r2,#0x24]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(812, 4));
    }
    {
        // 0x60028e44 vstr.32 s12,[r3,#0x1c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x60028e48 vstr.32 s15,[r3,#0x20]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60028e4c vstr.32 s14,[r3,#0x24]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x60028e50 bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x60028e52
}
pub fn b_60028e5c(c: &mut Machine) -> u32 {
    {
        // 0x60028e5c add.w r0,r0,#0x1000
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
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60028e60 vstr.32 s0,[r0,#0x324]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(804u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x60028e64 bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x60028e66
}
pub fn b_60028e68(c: &mut Machine) -> u32 {
    {
        // 0x60028e68 add.w r0,r0,#0x1000
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
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60028e6c vstr.32 s0,[r0,#0x328]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(808u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x60028e70 bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x60028e72
}
pub fn b_60028e74(c: &mut Machine) -> u32 {
    {
        // 0x60028e74 vmov.f32 s15,0x3f000000
        let value = 1056964608u64;
        c.w(828, 4, value);
    }
    {
        // 0x60028e78 vldr.32 s14,[pc,#0x28]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610780280u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(40u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x60028e7c add.w r0,r0,#0x1000
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
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60028e80 vmov.f32 s13,0x40800000
        let value = 1082130432u64;
        c.w(820, 4, value);
    }
    {
        // 0x60028e84 vmul.f32 s15,s0,s15
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60028e88 vmul.f32 s0,s0,s14
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x60028e8c vldr.32 s14,[r0,#0x258]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(600u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x60028e90 vmul.f32 s15,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60028e94 vmul.f32 s0,s0,s13
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x60028e98 vstr.32 s15,[r0,#0x25c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(604u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60028e9c vstr.32 s0,[r0,#0x260]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(608u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x60028ea0 bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x60028ea2
}
pub fn b_60028ea8(c: &mut Machine) -> u32 {
    {
        // 0x60028ea8 vmov.f32 s13,0x3f000000
        let value = 1056964608u64;
        c.w(820, 4, value);
    }
    {
        // 0x60028eac vldr.32 s15,[pc,#0x1c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610780332u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(28u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x60028eb0 vmov.f32 s14,0x3f800000
        let value = 1065353216u64;
        c.w(824, 4, value);
    }
    {
        // 0x60028eb4 add.w r0,r0,#0x1000
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
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60028eb8 vcmpe.f32 s0,s13
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
        // 0x60028ebc vmrs apsr,fpscr
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
        // 0x60028ec0 it pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x60028ec2 vmov.pl.f32 s15,s14
        let mut tf00 = 0u64;
        let mut t2700 = 0u64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2700 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x60028ec6u32;
        }
        let value = c.r(824, 4);
        c.w(828, 4, value);
    }
    0x60028ec6
}
pub fn b_60028ec6(c: &mut Machine) -> u32 {
    {
        // 0x60028ec6 vstr.32 s15,[r0,#0x320]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(800u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60028eca bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x60028ecc
}
pub fn b_60028ed0(c: &mut Machine) -> u32 {
    {
        // 0x60028ed0 vmov.f32 s14,0x3f800000
        let value = 1065353216u64;
        c.w(824, 4, value);
    }
    {
        // 0x60028ed4 add.w r0,r0,#0x1000
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
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60028ed8 vmov.f32 s15,0x40000000
        let value = 1073741824u64;
        c.w(828, 4, value);
    }
    {
        // 0x60028edc vsub.f32 s14,s14,s0
        let value = (f32::from_bits(c.r(824, 4) as u32) - f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x60028ee0 vsub.f32 s13,s15,s0
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x60028ee4 vsub.f32 s15,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60028ee8 vmul.f32 s13,s13,s0
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x60028eec vmul.f32 s15,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60028ef0 vstr.32 s13,[r0,#0x2a0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(672u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x60028ef4 vstr.32 s15,[r0,#0x29c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(668u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60028ef8 bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x60028efa
}
pub fn b_60028efc(c: &mut Machine) -> u32 {
    {
        // 0x60028efc vmov.f32 s15,0x3f600000
        let value = 1063256064u64;
        c.w(828, 4, value);
    }
    {
        // 0x60028f00 add.w r3,r0,#0xb00
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (2816u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(44, 4, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60028f04 addw r0,r0,#0xb64
        let value = ((c.r(32, 4) as u128 + 2916u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 2916u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(2916u64);
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60028f08 vmul.f32 s0,s0,s15
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x60028f0c vstr.32 s0,[r3,#0x14]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(20u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x60028f10 vstr.32 s0,[r0,#0x14]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(20u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x60028f14 bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x60028f16
}
pub fn b_60028f18(c: &mut Machine) -> u32 {
    {
        // 0x60028f18 vmov.f32 s15,0x3f000000
        let value = 1056964608u64;
        c.w(828, 4, value);
    }
    {
        // 0x60028f1c add.w r3,r0,#0x1000
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
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(44, 4, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60028f20 ldr.w r2,[r3,#0x378]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(44, 4)).wrapping_add(888u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x60028f24 vadd.f32 s0,s0,s15
        let value = (f32::from_bits(c.r(768, 4) as u32) + f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x60028f28 push {r4,r5,r6,r7,r8,r9,r10,r11,lr}
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
        // 0x60028f2c vcvt.s32.f32 s15,s0
        let value = (f32::from_bits(c.r(768, 4) as u32) as i32) as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60028f30 sub sp,#0x1c
        let mut t187500 = 0u64;
        t187500 = (28u64) & 0xffffffffu64;
        let value = (c.r(84, 4)).wrapping_sub(((t187500 >> 0) & 0xffffffffu64));
        c.w(84, 4, value);
    }
    {
        // 0x60028f32 vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x60028f36 cmp r2,r1
        let mut t1a6e00 = 0u64;
        let value = (c.r(36, 4) <= c.r(40, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(40, 4), c.r(36, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(40, 4)).wrapping_sub(c.r(36, 4))) & 0xffffffffu64;
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
        // 0x60028f38 beq.w 0x600296f4
        let mut t182300 = 0u64;
        t182300 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        if ((t182300 >> 0) & 0xffu64) != 0 {
            return 0x600296f4u32;
        }
    }
    0x60028f3c
}
pub fn b_60028f3c(c: &mut Machine) -> u32 {
    {
        // 0x60028f3c vldr.32 s12,[r0,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x60028f40 mvn r2,#0x2
        let mut t187d00 = 0u64;
        t187d00 = (2u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = !((t187d00 >> 0) & 0xffffffffu64);
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60028f44 vldr.32 s13,[r0,#0xb8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(184u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x60028f48 add.w r4,r0,#0x408
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1032u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60028f4c vldr.32 s14,[r0,#0x168]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(360u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x60028f50 vcvt.f64.f32 d3,s12
        let value = (f32::from_bits(c.r(816, 4) as u32) as f64).to_bits() as u64;
        c.w(792, 8, value);
    }
    {
        // 0x60028f54 str.w r1,[r3,#0x378]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(44, 4)).wrapping_add(888u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x60028f58 vcvt.f64.f32 d4,s13
        let value = (f32::from_bits(c.r(820, 4) as u32) as f64).to_bits() as u64;
        c.w(800, 8, value);
    }
    {
        // 0x60028f5c vcvt.f64.f32 d5,s14
        let value = (f32::from_bits(c.r(824, 4) as u32) as f64).to_bits() as u64;
        c.w(808, 8, value);
    }
    {
        // 0x60028f60 vstr.32 s12,[r0,#0xa0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(160u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x60028f64 vstr.32 s12,[r0,#0x40]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x60028f68 add.w r1,r0,#0x400
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1024u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60028f6c vstr.32 s13,[r0,#0x150]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(336u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x60028f70 add.w lr,r0,#0x420
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
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(88, 4, value);
        let value = (signed(c.r(88, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(88, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60028f74 vstr.32 s13,[r0,#0xf0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(240u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x60028f78 add.w r5,r0,#0x4d0
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1232u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60028f7c vldr.32 s15,[pc,#0x3e8]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610780540u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(1000u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x60028f80 add.w r10,r0,#0x418
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1048u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(72, 4, value);
        let value = (signed(c.r(72, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(72, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60028f84 str r2,[r0,#0x3c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(32, 4)).wrapping_add(60u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x60028f86 add.w r9,r0,#0x438
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1080u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(68, 4, value);
        let value = (signed(c.r(68, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(68, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60028f8a str r2,[r0,#0x74]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(32, 4)).wrapping_add(116u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x60028f8c add.w r7,r0,#0x4e8
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1256u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(60, 4, value);
        let value = (signed(c.r(60, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(60, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60028f90 str.w r2,[r0,#0xec]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(236u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x60028f94 add.w r8,r0,#0x630
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
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(64, 4, value);
        let value = (signed(c.r(64, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(64, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60028f98 str.w r2,[r0,#0x124]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(292u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x60028f9c add.w r12,r0,#0x6e0
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1760u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(80, 4, value);
        let value = (signed(c.r(80, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(80, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60028fa0 vstr.32 s14,[r0,#0x200]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(512u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x60028fa4 add.w r6,r0,#0x790
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1936u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(56, 4, value);
        let value = (signed(c.r(56, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(56, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60028fa8 str.w r2,[r0,#0x19c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(412u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x60028fac add.w r11,r0,#0x9a0
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (2464u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(76, 4, value);
        let value = (signed(c.r(76, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(76, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60028fb0 vstr.32 s14,[r0,#0x1a0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(416u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x60028fb4 str.w r2,[r0,#0x1d4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(468u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x60028fb8 vstr.32 s15,[r0,#0x4c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(76u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60028fbc vstr.32 s15,[r0,#0x84]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(132u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60028fc0 vstr.32 s15,[r0,#0xa4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(164u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60028fc4 vstr.32 s15,[r0,#0xa8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(168u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60028fc8 vstr.32 s15,[r0,#0xfc]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(252u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60028fcc vstr.32 s15,[r0,#0x134]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(308u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60028fd0 vstr.32 s15,[r0,#0x154]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(340u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60028fd4 vstr.32 s15,[r0,#0x158]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(344u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60028fd8 vstr.32 s15,[r0,#0x1ac]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(428u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60028fdc vstr.32 s15,[r0,#0x1e4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(484u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60028fe0 vldr.64 d6,[pc,#0x37c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610780640u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(892u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 8);
        c.w(816, 8, value);
    }
    {
        // 0x60028fe4 vstr.64 d5,[r0,#0x1f0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(496u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 8, c.r(808, 8));
    }
    {
        // 0x60028fe8 vstr.64 d3,[r0,#0x90]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(144u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 8, c.r(792, 8));
    }
    {
        // 0x60028fec vstr.64 d4,[r0,#0x140]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(320u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 8, c.r(800, 8));
    }
    {
        // 0x60028ff0 vstr.64 d6,[r0,#0x10]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 8, c.r(816, 8));
    }
    {
        // 0x60028ff4 vstr.64 d6,[r0,#0xc0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(192u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 8, c.r(816, 8));
    }
    {
        // 0x60028ff8 vstr.64 d6,[r0,#0x170]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(368u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 8, c.r(816, 8));
    }
    {
        // 0x60028ffc vstr.32 s15,[r0,#0x204]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(516u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029000 vldr.32 s10,[r0,#0x218]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(536u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x60029004 vldr.32 s11,[r0,#0x2c8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(712u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x60029008 vldr.32 s14,[r0,#0x378]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(888u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002900c vcvt.f64.f32 d2,s10
        let value = (f32::from_bits(c.r(808, 4) as u32) as f64).to_bits() as u64;
        c.w(784, 8, value);
    }
    {
        // 0x60029010 vcvt.f64.f32 d3,s11
        let value = (f32::from_bits(c.r(812, 4) as u32) as f64).to_bits() as u64;
        c.w(792, 8, value);
    }
    {
        // 0x60029014 vstr.32 s10,[r0,#0x2b0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(688u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(808, 4));
    }
    {
        // 0x60029018 vcvt.f64.f32 d4,s14
        let value = (f32::from_bits(c.r(824, 4) as u32) as f64).to_bits() as u64;
        c.w(800, 8, value);
    }
    {
        // 0x6002901c str.w r2,[r0,#0x24c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(588u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x60029020 vstr.32 s10,[r0,#0x250]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(592u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(808, 4));
    }
    {
        // 0x60029024 str.w r2,[r0,#0x284]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(644u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x60029028 vstr.32 s11,[r0,#0x360]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(864u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(812, 4));
    }
    {
        // 0x6002902c str.w r2,[r0,#0x2fc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(764u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x60029030 vstr.32 s11,[r0,#0x300]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(768u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(812, 4));
    }
    {
        // 0x60029034 str.w r2,[r0,#0x334]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(820u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x60029038 vstr.32 s15,[r0,#0x208]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(520u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002903c vstr.32 s15,[r0,#0x25c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(604u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029040 vstr.32 s15,[r0,#0x294]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(660u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029044 vstr.32 s15,[r0,#0x2b4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(692u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029048 vstr.32 s15,[r0,#0x2b8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(696u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002904c vstr.32 s15,[r0,#0x30c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(780u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029050 vstr.32 s15,[r0,#0x344]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(836u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029054 vstr.32 s15,[r0,#0x364]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(868u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029058 vstr.32 s15,[r0,#0x368]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(872u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002905c vstr.64 d2,[r0,#0x2a0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(672u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 8, c.r(784, 8));
    }
    {
        // 0x60029060 vstr.64 d3,[r0,#0x350]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(848u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 8, c.r(792, 8));
    }
    {
        // 0x60029064 vstr.64 d6,[r0,#0x220]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(544u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 8, c.r(816, 8));
    }
    {
        // 0x60029068 vstr.64 d6,[r0,#0x2d0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(720u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 8, c.r(816, 8));
    }
    {
        // 0x6002906c vstr.64 d4,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 8, c.r(800, 8));
    }
    {
        // 0x60029070 vstr.32 s14,[r4,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x60029074 addw r4,r0,#0x414
        let value = ((c.r(32, 4) as u128 + 1044u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 1044u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(1044u64);
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029078 str.w r2,[r0,#0x3ac]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(940u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x6002907c add.w r1,r0,#0x580
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1408u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029080 vstr.32 s14,[r0,#0x3b0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(944u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x60029084 str.w r2,[r0,#0x3e4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(996u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x60029088 vstr.32 s15,[r0,#0x3bc]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(956u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002908c vstr.32 s15,[r0,#0x3f4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(1012u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029090 vstr.64 d6,[r0,#0x380]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(896u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 8, c.r(816, 8));
    }
    {
        // 0x60029094 vstr.32 s15,[r4]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(48, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029098 add.w r4,r0,#0x598
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1432u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002909c vldr.32 s10,[lr,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(88, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x600290a0 vldr.32 s14,[r1,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x600290a4 vcvt.f64.f32 d2,s10
        let value = (f32::from_bits(c.r(808, 4) as u32) as f64).to_bits() as u64;
        c.w(784, 8, value);
    }
    {
        // 0x600290a8 vldr.32 s11,[r5,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x600290ac vstr.32 s15,[r10]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(72, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600290b0 add.w r10,r0,#0x4b0
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1200u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(72, 4, value);
        let value = (signed(c.r(72, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(72, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600290b4 vcvt.f64.f32 d3,s11
        let value = (f32::from_bits(c.r(812, 4) as u32) as f64).to_bits() as u64;
        c.w(792, 8, value);
    }
    {
        // 0x600290b8 vcvt.f64.f32 d4,s14
        let value = (f32::from_bits(c.r(824, 4) as u32) as f64).to_bits() as u64;
        c.w(800, 8, value);
    }
    {
        // 0x600290bc vstr.64 d2,[r10]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(72, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 8, c.r(784, 8));
    }
    {
        // 0x600290c0 add.w r10,r0,#0x4b8
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1208u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(72, 4, value);
        let value = (signed(c.r(72, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(72, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600290c4 vstr.32 s10,[r10,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(808, 4));
    }
    {
        // 0x600290c8 add.w r10,r0,#0x628
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1576u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(72, 4, value);
        let value = (signed(c.r(72, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(72, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600290cc str.w r2,[r0,#0x45c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(1116u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x600290d0 vstr.32 s15,[r9,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(68, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600290d4 vstr.32 s10,[r9,#0x28]
        let mut t64000 = 0u64;
        t64000 = ((c.r(68, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(808, 4));
    }
    {
        // 0x600290d8 add.w r9,r0,#0x470
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1136u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(68, 4, value);
        let value = (signed(c.r(68, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(68, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600290dc str.w r2,[r0,#0x494]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(1172u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x600290e0 vstr.32 s15,[r9,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(68, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600290e4 add.w r9,r0,#0x648
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1608u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(68, 4, value);
        let value = (signed(c.r(68, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(68, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600290e8 vstr.64 d6,[lr,#0x10]
        let mut t64000 = 0u64;
        t64000 = ((c.r(88, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 8, c.r(816, 8));
    }
    {
        // 0x600290ec addw lr,r0,#0x4c4
        let value = ((c.r(32, 4) as u128 + 1220u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 1220u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(1220u64);
        c.w(88, 4, value);
        let value = (signed(c.r(88, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(88, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600290f0 vstr.32 s15,[lr]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(88, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600290f4 add.w lr,r0,#0x4c8
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1224u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(88, 4, value);
        let value = (signed(c.r(88, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(88, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600290f8 vstr.32 s15,[lr]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(88, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600290fc add.w lr,r0,#0x560
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1376u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(88, 4, value);
        let value = (signed(c.r(88, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(88, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029100 vstr.64 d3,[lr]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(88, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 8, c.r(792, 8));
    }
    {
        // 0x60029104 add.w lr,r0,#0x568
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1384u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(88, 4, value);
        let value = (signed(c.r(88, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(88, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029108 vstr.32 s11,[lr,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(88, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(812, 4));
    }
    {
        // 0x6002910c add.w lr,r0,#0x6f8
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1784u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(88, 4, value);
        let value = (signed(c.r(88, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(88, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029110 str.w r2,[r0,#0x50c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(1292u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x60029114 vstr.32 s15,[r7,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(60, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029118 vstr.32 s11,[r7,#0x28]
        let mut t64000 = 0u64;
        t64000 = ((c.r(60, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(812, 4));
    }
    {
        // 0x6002911c add.w r7,r0,#0x520
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1312u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(60, 4, value);
        let value = (signed(c.r(60, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(60, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029120 str.w r2,[r0,#0x544]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(1348u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x60029124 vstr.32 s15,[r7,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(60, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029128 add.w r7,r0,#0x7a8
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1960u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(60, 4, value);
        let value = (signed(c.r(60, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(60, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002912c vstr.64 d6,[r5,#0x10]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 8, c.r(816, 8));
    }
    {
        // 0x60029130 addw r5,r0,#0x574
        let value = ((c.r(32, 4) as u128 + 1396u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 1396u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(1396u64);
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029134 vstr.32 s15,[r5]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(52, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029138 add.w r5,r0,#0x578
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1400u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002913c vstr.32 s15,[r5]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(52, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029140 add.w r5,r0,#0x610
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1552u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029144 vstr.64 d4,[r5]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(52, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 8, c.r(800, 8));
    }
    {
        // 0x60029148 add.w r5,r0,#0x618
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1560u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002914c vstr.32 s14,[r5,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x60029150 add.w r5,r0,#0x840
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (2112u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029154 str.w r2,[r0,#0x5bc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(1468u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x60029158 vstr.32 s15,[r4,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002915c vstr.32 s14,[r4,#0x28]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x60029160 add.w r4,r0,#0x5d0
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1488u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029164 str.w r2,[r0,#0x5f4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(1524u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x60029168 vstr.32 s15,[r4,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002916c add.w r4,r0,#0x8f0
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (2288u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029170 vstr.64 d6,[r1,#0x10]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 8, c.r(816, 8));
    }
    {
        // 0x60029174 addw r1,r0,#0x624
        let value = ((c.r(32, 4) as u128 + 1572u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 1572u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(1572u64);
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029178 vstr.32 s15,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002917c add.w r1,r0,#0xa50
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (2640u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029180 vldr.32 s10,[r8,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(64, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x60029184 vldr.32 s14,[r6,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(56, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x60029188 vcvt.f64.f32 d2,s10
        let value = (f32::from_bits(c.r(808, 4) as u32) as f64).to_bits() as u64;
        c.w(784, 8, value);
    }
    {
        // 0x6002918c vldr.32 s11,[r12,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(80, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x60029190 vstr.32 s15,[r10]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(72, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029194 add.w r10,r0,#0x6c0
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1728u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(72, 4, value);
        let value = (signed(c.r(72, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(72, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029198 vcvt.f64.f32 d3,s11
        let value = (f32::from_bits(c.r(812, 4) as u32) as f64).to_bits() as u64;
        c.w(792, 8, value);
    }
    {
        // 0x6002919c vcvt.f64.f32 d4,s14
        let value = (f32::from_bits(c.r(824, 4) as u32) as f64).to_bits() as u64;
        c.w(800, 8, value);
    }
    {
        // 0x600291a0 vstr.64 d2,[r10]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(72, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 8, c.r(784, 8));
    }
    {
        // 0x600291a4 add.w r10,r0,#0x6c8
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1736u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(72, 4, value);
        let value = (signed(c.r(72, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(72, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600291a8 vstr.32 s10,[r10,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(808, 4));
    }
    {
        // 0x600291ac addw r10,r0,#0xa68
        let value = ((c.r(32, 4) as u128 + 2664u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 2664u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(2664u64);
        c.w(72, 4, value);
        let value = (signed(c.r(72, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(72, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600291b0 str.w r2,[r0,#0x66c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(1644u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x600291b4 vstr.32 s15,[r9,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(68, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600291b8 vstr.32 s10,[r9,#0x28]
        let mut t64000 = 0u64;
        t64000 = ((c.r(68, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(808, 4));
    }
    {
        // 0x600291bc add.w r9,r0,#0x680
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1664u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(68, 4, value);
        let value = (signed(c.r(68, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(68, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600291c0 str.w r2,[r0,#0x6a4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(1700u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x600291c4 vstr.32 s15,[r9,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(68, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600291c8 addw r9,r0,#0xbc8
        let value = ((c.r(32, 4) as u128 + 3016u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 3016u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(3016u64);
        c.w(68, 4, value);
        let value = (signed(c.r(68, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(68, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600291cc vstr.64 d6,[r8,#0x10]
        let mut t64000 = 0u64;
        t64000 = ((c.r(64, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 8, c.r(816, 8));
    }
    {
        // 0x600291d0 addw r8,r0,#0x6d4
        let value = ((c.r(32, 4) as u128 + 1748u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 1748u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(1748u64);
        c.w(64, 4, value);
        let value = (signed(c.r(64, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(64, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600291d4 vstr.32 s15,[r8]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(64, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600291d8 add.w r8,r0,#0x6d8
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1752u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(64, 4, value);
        let value = (signed(c.r(64, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(64, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600291dc vstr.32 s15,[r8]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(64, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600291e0 add.w r8,r0,#0x770
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1904u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(64, 4, value);
        let value = (signed(c.r(64, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(64, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600291e4 vstr.64 d3,[r8]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(64, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 8, c.r(792, 8));
    }
    {
        // 0x600291e8 add.w r8,r0,#0x778
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1912u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(64, 4, value);
        let value = (signed(c.r(64, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(64, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600291ec vstr.32 s11,[r8,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(64, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(812, 4));
    }
    {
        // 0x600291f0 addw r8,r0,#0xc28
        let value = ((c.r(32, 4) as u128 + 3112u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 3112u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(3112u64);
        c.w(64, 4, value);
        let value = (signed(c.r(64, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(64, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600291f4 str.w r2,[r0,#0x71c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(1820u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x600291f8 vstr.32 s15,[lr,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(88, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600291fc vstr.32 s11,[lr,#0x28]
        let mut t64000 = 0u64;
        t64000 = ((c.r(88, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(812, 4));
    }
    {
        // 0x60029200 add.w lr,r0,#0x730
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1840u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(88, 4, value);
        let value = (signed(c.r(88, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(88, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029204 str.w r2,[r0,#0x754]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(1876u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x60029208 vstr.32 s15,[lr,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(88, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002920c addw lr,r0,#0x838
        let value = ((c.r(32, 4) as u128 + 2104u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 2104u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(2104u64);
        c.w(88, 4, value);
        let value = (signed(c.r(88, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(88, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029210 vstr.64 d6,[r12,#0x10]
        let mut t64000 = 0u64;
        t64000 = ((c.r(80, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 8, c.r(816, 8));
    }
    {
        // 0x60029214 addw r12,r0,#0x784
        let value = ((c.r(32, 4) as u128 + 1924u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 1924u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(1924u64);
        c.w(80, 4, value);
        let value = (signed(c.r(80, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(80, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029218 vstr.32 s15,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002921c add.w r12,r0,#0x788
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (1928u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(80, 4, value);
        let value = (signed(c.r(80, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(80, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029220 vstr.32 s15,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029224 add.w r12,r0,#0x820
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (2080u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(80, 4, value);
        let value = (signed(c.r(80, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(80, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029228 vstr.64 d4,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 8, c.r(800, 8));
    }
    {
        // 0x6002922c addw r12,r0,#0x828
        let value = ((c.r(32, 4) as u128 + 2088u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 2088u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(2088u64);
        c.w(80, 4, value);
        let value = (signed(c.r(80, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(80, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029230 vstr.32 s14,[r12,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(80, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x60029234 addw r12,r0,#0x858
        let value = ((c.r(32, 4) as u128 + 2136u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 2136u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(2136u64);
        c.w(80, 4, value);
        let value = (signed(c.r(80, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(80, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029238 str.w r2,[r0,#0x7cc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(1996u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x6002923c vstr.32 s15,[r7,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(60, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029240 vstr.32 s14,[r7,#0x28]
        let mut t64000 = 0u64;
        t64000 = ((c.r(60, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x60029244 add.w r7,r0,#0x7e0
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (2016u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(60, 4, value);
        let value = (signed(c.r(60, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(60, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029248 str.w r2,[r0,#0x804]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(2052u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x6002924c vstr.32 s15,[r7,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(60, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029250 addw r7,r0,#0x908
        let value = ((c.r(32, 4) as u128 + 2312u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 2312u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(2312u64);
        c.w(60, 4, value);
        let value = (signed(c.r(60, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(60, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029254 vstr.64 d6,[r6,#0x10]
        let mut t64000 = 0u64;
        t64000 = ((c.r(56, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 8, c.r(816, 8));
    }
    {
        // 0x60029258 addw r6,r0,#0x834
        let value = ((c.r(32, 4) as u128 + 2100u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 2100u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(2100u64);
        c.w(56, 4, value);
        let value = (signed(c.r(56, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(56, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002925c vstr.32 s15,[r6]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(56, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029260 addw r6,r0,#0x9b8
        let value = ((c.r(32, 4) as u128 + 2488u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 2488u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(2488u64);
        c.w(56, 4, value);
        let value = (signed(c.r(56, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(56, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029264 vldr.32 s10,[r5,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x60029268 vldr.32 s14,[r11,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002926c vcvt.f64.f32 d2,s10
        let value = (f32::from_bits(c.r(808, 4) as u32) as f64).to_bits() as u64;
        c.w(784, 8, value);
    }
    {
        // 0x60029270 vldr.32 s11,[r4,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x60029274 vstr.32 s15,[lr]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(88, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029278 add.w lr,r0,#0x8d0
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (2256u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(88, 4, value);
        let value = (signed(c.r(88, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(88, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002927c vcvt.f64.f32 d3,s11
        let value = (f32::from_bits(c.r(812, 4) as u32) as f64).to_bits() as u64;
        c.w(792, 8, value);
    }
    {
        // 0x60029280 str r1,[sp,#0x0]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x60029282 vcvt.f64.f32 d4,s14
        let value = (f32::from_bits(c.r(824, 4) as u32) as f64).to_bits() as u64;
        c.w(800, 8, value);
    }
    {
        // 0x60029286 addw r1,r0,#0xa44
        let value = ((c.r(32, 4) as u128 + 2628u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 2628u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(2628u64);
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002928a vstr.64 d2,[lr]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(88, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 8, c.r(784, 8));
    }
    {
        // 0x6002928e addw lr,r0,#0x8d8
        let value = ((c.r(32, 4) as u128 + 2264u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 2264u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(2264u64);
        c.w(88, 4, value);
        let value = (signed(c.r(88, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(88, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029292 vstr.32 s10,[lr,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(88, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(808, 4));
    }
    {
        // 0x60029296 addw lr,r0,#0xf88
        let value = ((c.r(32, 4) as u128 + 3976u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 3976u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(3976u64);
        c.w(88, 4, value);
        let value = (signed(c.r(88, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(88, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002929a str.w r2,[r0,#0x87c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(2172u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x6002929e vstr.32 s15,[r12,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(80, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600292a2 vstr.32 s10,[r12,#0x28]
        let mut t64000 = 0u64;
        t64000 = ((c.r(80, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(808, 4));
    }
    {
        // 0x600292a6 add.w r12,r0,#0x890
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (2192u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(80, 4, value);
        let value = (signed(c.r(80, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(80, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600292aa str.w r2,[r0,#0x8b4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(2228u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x600292ae vstr.32 s15,[r12,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(80, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600292b2 addw r12,r0,#0xfe8
        let value = ((c.r(32, 4) as u128 + 4072u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 4072u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(4072u64);
        c.w(80, 4, value);
        let value = (signed(c.r(80, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(80, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600292b6 vstr.64 d6,[r5,#0x10]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 8, c.r(816, 8));
    }
    {
        // 0x600292ba addw r5,r0,#0x8e4
        let value = ((c.r(32, 4) as u128 + 2276u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 2276u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(2276u64);
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600292be vstr.32 s15,[r5]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(52, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600292c2 addw r5,r0,#0x8e8
        let value = ((c.r(32, 4) as u128 + 2280u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 2280u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(2280u64);
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600292c6 vstr.32 s15,[r5]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(52, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600292ca add.w r5,r0,#0x980
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (2432u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600292ce vstr.64 d3,[r5]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(52, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 8, c.r(792, 8));
    }
    {
        // 0x600292d2 addw r5,r0,#0x988
        let value = ((c.r(32, 4) as u128 + 2440u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 2440u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(2440u64);
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600292d6 vstr.32 s11,[r5,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(812, 4));
    }
    {
        // 0x600292da add.w r5,r0,#0x940
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (2368u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600292de str.w r2,[r0,#0x92c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(2348u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x600292e2 vstr.32 s11,[r7,#0x28]
        let mut t64000 = 0u64;
        t64000 = ((c.r(60, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(812, 4));
    }
    {
        // 0x600292e6 vstr.32 s15,[r7,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(60, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600292ea addw r7,r0,#0xd48
        let value = ((c.r(32, 4) as u128 + 3400u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 3400u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(3400u64);
        c.w(60, 4, value);
        let value = (signed(c.r(60, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(60, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600292ee str.w r2,[r0,#0x964]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(2404u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x600292f2 vstr.32 s15,[r5,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600292f6 addw r5,r0,#0xc88
        let value = ((c.r(32, 4) as u128 + 3208u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 3208u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(3208u64);
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600292fa vstr.64 d6,[r4,#0x10]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 8, c.r(816, 8));
    }
    {
        // 0x600292fe addw r4,r0,#0x994
        let value = ((c.r(32, 4) as u128 + 2452u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 2452u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(2452u64);
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029302 vstr.32 s15,[r4]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(48, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029306 addw r4,r0,#0x998
        let value = ((c.r(32, 4) as u128 + 2456u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 2456u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(2456u64);
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002930a vstr.32 s15,[r4]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(48, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002930e add.w r4,r0,#0xa30
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (2608u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029312 vstr.64 d4,[r4]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(48, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 8, c.r(800, 8));
    }
    {
        // 0x60029316 addw r4,r0,#0xa38
        let value = ((c.r(32, 4) as u128 + 2616u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 2616u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(2616u64);
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002931a vstr.32 s14,[r4,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002931e add.w r4,r0,#0x9f0
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (2544u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029322 str.w r2,[r0,#0x9dc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(2524u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x60029326 vstr.32 s14,[r6,#0x28]
        let mut t64000 = 0u64;
        t64000 = ((c.r(56, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002932a vstr.32 s15,[r6,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(56, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002932e addw r6,r0,#0xce8
        let value = ((c.r(32, 4) as u128 + 3304u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 3304u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(3304u64);
        c.w(56, 4, value);
        let value = (signed(c.r(56, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(56, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029332 str.w r2,[r0,#0xa14]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(2580u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x60029336 vstr.32 s15,[r4,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002933a mvn r4,#0x3
        let mut t187d00 = 0u64;
        t187d00 = (3u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = !((t187d00 >> 0) & 0xffffffffu64);
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002933e vstr.64 d6,[r11,#0x10]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 8, c.r(816, 8));
    }
    {
        // 0x60029342 vstr.32 s15,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029346 addw r11,r0,#0xe08
        let value = ((c.r(32, 4) as u128 + 3592u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 3592u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(3592u64);
        c.w(76, 4, value);
        let value = (signed(c.r(76, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(76, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002934a ldr r1,[sp,#0x0]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002934c str r6,[sp,#0x8]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002934e addw r6,r0,#0xa48
        let value = ((c.r(32, 4) as u128 + 2632u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 2632u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(2632u64);
        c.w(56, 4, value);
        let value = (signed(c.r(56, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(56, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029352 vldr.32 s14,[r1,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x60029356 movs r1,#0x0
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
        // 0x60029358 vstr.32 s15,[r6]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(56, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002935c b 0x6002936c
        return 0x6002936cu32;
    }
    0x6002935e
}
pub fn b_6002936c(c: &mut Machine) -> u32 {
    {
        // 0x6002936c add.w r6,r0,#0xae0
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (2784u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(56, 4, value);
        let value = (signed(c.r(56, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(56, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029370 vcvt.f64.f32 d5,s14
        let value = (f32::from_bits(c.r(824, 4) as u32) as f64).to_bits() as u64;
        c.w(808, 8, value);
    }
    {
        // 0x60029374 str r5,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x60029376 addw r5,r0,#0xda8
        let value = ((c.r(32, 4) as u128 + 3496u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 3496u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(3496u64);
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002937a str r7,[sp,#0xc]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(12u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(60, 4));
    }
    {
        // 0x6002937c addw r7,r0,#0xe68
        let value = ((c.r(32, 4) as u128 + 3688u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 3688u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(3688u64);
        c.w(60, 4, value);
        let value = (signed(c.r(60, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(60, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029380 str r5,[sp,#0x10]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x60029382 addw r5,r0,#0xec8
        let value = ((c.r(32, 4) as u128 + 3784u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 3784u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(3784u64);
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029386 vstr.64 d5,[r6]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(56, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 8, c.r(808, 8));
    }
    {
        // 0x6002938a addw r6,r0,#0xae8
        let value = ((c.r(32, 4) as u128 + 2792u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 2792u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(2792u64);
        c.w(56, 4, value);
        let value = (signed(c.r(56, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(56, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002938e str r5,[sp,#0x14]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(20u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x60029390 addw r5,r0,#0xf28
        let value = ((c.r(32, 4) as u128 + 3880u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 3880u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(3880u64);
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029394 vstr.32 s14,[r6,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(56, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x60029398 str.w r2,[r0,#0xa8c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(2700u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x6002939c vstr.32 s15,[r10,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600293a0 vstr.32 s14,[r10,#0x28]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x600293a4 add.w r10,r0,#0xaa0
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (2720u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(72, 4, value);
        let value = (signed(c.r(72, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(72, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600293a8 str.w r2,[r0,#0xac4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(2756u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x600293ac ldr r6,[sp,#0x0]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(56, 4, value);
    }
    {
        // 0x600293ae vstr.32 s15,[r10,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600293b2 addw r10,r0,#0xaf4
        let value = ((c.r(32, 4) as u128 + 2804u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 2804u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(2804u64);
        c.w(72, 4, value);
        let value = (signed(c.r(72, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(72, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600293b6 vstr.64 d6,[r6,#0x10]
        let mut t64000 = 0u64;
        t64000 = ((c.r(56, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 8, c.r(816, 8));
    }
    {
        // 0x600293ba vstr.32 s15,[r10]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(72, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600293be addw r10,r0,#0xaf8
        let value = ((c.r(32, 4) as u128 + 2808u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 2808u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(2808u64);
        c.w(72, 4, value);
        let value = (signed(c.r(72, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(72, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600293c2 vstr.32 s15,[r10]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(72, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600293c6 addw r10,r0,#0xb2c
        let value = ((c.r(32, 4) as u128 + 2860u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 2860u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(2860u64);
        c.w(72, 4, value);
        let value = (signed(c.r(72, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(72, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600293ca str.w r2,[r0,#0xb50]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(2896u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x600293ce vstr.32 s15,[r10,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600293d2 add.w r10,r0,#0xb00
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (2816u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(72, 4, value);
        let value = (signed(c.r(72, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(72, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600293d6 vstr.32 s15,[r10,#0x24]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600293da str.w r2,[r0,#0xbb4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(2996u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x600293de add.w r2,r0,#0xb90
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (2960u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600293e2 vstr.32 s15,[r2,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600293e6 addw r2,r0,#0xb64
        let value = ((c.r(32, 4) as u128 + 2916u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 2916u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(2916u64);
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600293ea vstr.32 s15,[r2,#0x24]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600293ee str.w r4,[r0,#0xc24]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(3108u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x600293f2 vstr.32 s15,[r9,#0x4c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(68, 4)).wrapping_add(76u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600293f6 vstr.32 s15,[r9,#0x28]
        let mut t64000 = 0u64;
        t64000 = ((c.r(68, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600293fa vstr.32 s15,[r9,#0x2c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(68, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600293fe vstr.32 s15,[r9,#0x30]
        let mut t64000 = 0u64;
        t64000 = ((c.r(68, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029402 vstr.32 s15,[r9,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(68, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029406 vstr.32 s15,[r9,#0x38]
        let mut t64000 = 0u64;
        t64000 = ((c.r(68, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002940a ldr r2,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x6002940c vstr.32 s15,[r9,#0x3c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(68, 4)).wrapping_add(60u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029410 vstr.32 s15,[r9,#0x40]
        let mut t64000 = 0u64;
        t64000 = ((c.r(68, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029414 vstr.32 s15,[r9,#0x44]
        let mut t64000 = 0u64;
        t64000 = ((c.r(68, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029418 vstr.32 s15,[r9,#0x48]
        let mut t64000 = 0u64;
        t64000 = ((c.r(68, 4)).wrapping_add(72u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002941c str.w r1,[r0,#0xbdc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(3036u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x60029420 str.w r4,[r0,#0xc84]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(3204u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x60029424 vstr.32 s15,[r8,#0x4c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(64, 4)).wrapping_add(76u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029428 vstr.32 s15,[r8,#0x28]
        let mut t64000 = 0u64;
        t64000 = ((c.r(64, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002942c vstr.32 s15,[r8,#0x2c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(64, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029430 vstr.32 s15,[r8,#0x30]
        let mut t64000 = 0u64;
        t64000 = ((c.r(64, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029434 vstr.32 s15,[r8,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(64, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029438 vstr.32 s15,[r8,#0x38]
        let mut t64000 = 0u64;
        t64000 = ((c.r(64, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002943c vstr.32 s15,[r8,#0x3c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(64, 4)).wrapping_add(60u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029440 vstr.32 s15,[r8,#0x40]
        let mut t64000 = 0u64;
        t64000 = ((c.r(64, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029444 vstr.32 s15,[r8,#0x44]
        let mut t64000 = 0u64;
        t64000 = ((c.r(64, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029448 vstr.32 s15,[r8,#0x48]
        let mut t64000 = 0u64;
        t64000 = ((c.r(64, 4)).wrapping_add(72u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002944c str.w r1,[r0,#0xc3c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(3132u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x60029450 str.w r4,[r0,#0xce4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(3300u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x60029454 vstr.32 s15,[r2,#0x4c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(76u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029458 vstr.32 s15,[r2,#0x28]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002945c vstr.32 s15,[r2,#0x2c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029460 vstr.32 s15,[r2,#0x30]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029464 vstr.32 s15,[r2,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029468 vstr.32 s15,[r2,#0x38]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002946c vstr.32 s15,[r2,#0x3c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(60u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029470 vstr.32 s15,[r2,#0x40]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029474 vstr.32 s15,[r2,#0x44]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029478 vstr.32 s15,[r2,#0x48]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(72u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002947c ldr r2,[sp,#0x8]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x6002947e str.w r1,[r0,#0xc9c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(3228u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x60029482 str.w r4,[r0,#0xd44]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(3396u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x60029486 vstr.32 s15,[r2,#0x4c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(76u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002948a vstr.32 s15,[r2,#0x28]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002948e vstr.32 s15,[r2,#0x2c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029492 vstr.32 s15,[r2,#0x30]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029496 vstr.32 s15,[r2,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002949a vstr.32 s15,[r2,#0x38]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002949e vstr.32 s15,[r2,#0x3c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(60u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600294a2 vstr.32 s15,[r2,#0x40]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600294a6 vstr.32 s15,[r2,#0x44]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600294aa vstr.32 s15,[r2,#0x48]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(72u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600294ae ldr r2,[sp,#0xc]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(12u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x600294b0 str.w r1,[r0,#0xcfc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(3324u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x600294b4 str.w r4,[r0,#0xda4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(3492u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x600294b8 vstr.32 s15,[r2,#0x4c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(76u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600294bc vstr.32 s15,[r2,#0x28]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600294c0 vstr.32 s15,[r2,#0x2c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600294c4 vstr.32 s15,[r2,#0x30]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600294c8 vstr.32 s15,[r2,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600294cc vstr.32 s15,[r2,#0x38]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600294d0 vstr.32 s15,[r2,#0x3c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(60u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600294d4 vstr.32 s15,[r2,#0x40]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600294d8 vstr.32 s15,[r2,#0x44]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600294dc vstr.32 s15,[r2,#0x48]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(72u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600294e0 ldr r2,[sp,#0x10]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x600294e2 str.w r1,[r0,#0xd5c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(3420u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x600294e6 str.w r4,[r0,#0xe04]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(3588u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x600294ea vstr.32 s15,[r2,#0x4c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(76u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600294ee vstr.32 s15,[r2,#0x28]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600294f2 vstr.32 s15,[r2,#0x2c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600294f6 vstr.32 s15,[r2,#0x30]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600294fa vstr.32 s15,[r2,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600294fe vstr.32 s15,[r2,#0x38]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029502 vstr.32 s15,[r2,#0x3c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(60u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029506 vstr.32 s15,[r2,#0x40]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002950a vstr.32 s15,[r2,#0x44]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002950e vstr.32 s15,[r2,#0x48]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(72u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029512 ldr r2,[sp,#0x14]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(20u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x60029514 str.w r1,[r0,#0xdbc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(3516u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x60029518 str.w r4,[r0,#0xe64]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(3684u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002951c vstr.32 s15,[r11,#0x4c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(76u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029520 vstr.32 s15,[r11,#0x28]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029524 vstr.32 s15,[r11,#0x2c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029528 vstr.32 s15,[r11,#0x30]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002952c vstr.32 s15,[r11,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029530 vstr.32 s15,[r11,#0x38]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029534 vstr.32 s15,[r11,#0x3c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(60u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029538 vstr.32 s15,[r11,#0x40]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002953c vstr.32 s15,[r11,#0x44]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029540 vstr.32 s15,[r11,#0x48]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(72u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029544 str.w r1,[r0,#0xe1c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(3612u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x60029548 str.w r4,[r0,#0xec4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(3780u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002954c vstr.32 s15,[r7,#0x4c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(60, 4)).wrapping_add(76u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029550 vstr.32 s15,[r7,#0x28]
        let mut t64000 = 0u64;
        t64000 = ((c.r(60, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029554 vstr.32 s15,[r7,#0x2c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(60, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029558 vstr.32 s15,[r7,#0x30]
        let mut t64000 = 0u64;
        t64000 = ((c.r(60, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002955c vstr.32 s15,[r7,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(60, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029560 vstr.32 s15,[r7,#0x38]
        let mut t64000 = 0u64;
        t64000 = ((c.r(60, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029564 vstr.32 s15,[r7,#0x3c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(60, 4)).wrapping_add(60u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029568 vstr.32 s15,[r7,#0x40]
        let mut t64000 = 0u64;
        t64000 = ((c.r(60, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002956c vstr.32 s15,[r7,#0x44]
        let mut t64000 = 0u64;
        t64000 = ((c.r(60, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029570 vstr.32 s15,[r7,#0x48]
        let mut t64000 = 0u64;
        t64000 = ((c.r(60, 4)).wrapping_add(72u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029574 str.w r1,[r0,#0xe7c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(3708u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x60029578 str.w r4,[r0,#0xf24]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(3876u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002957c vstr.32 s15,[r2,#0x4c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(76u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029580 vstr.32 s15,[r2,#0x28]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029584 vstr.32 s15,[r2,#0x2c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029588 vstr.32 s15,[r2,#0x30]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002958c vstr.32 s15,[r2,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029590 vstr.32 s15,[r2,#0x38]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029594 vstr.32 s15,[r2,#0x3c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(60u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029598 vstr.32 s15,[r2,#0x40]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002959c vstr.32 s15,[r2,#0x44]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600295a0 vstr.32 s15,[r2,#0x48]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(72u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600295a4 str.w r1,[r0,#0xedc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(3804u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x600295a8 str.w r4,[r0,#0xf84]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(3972u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x600295ac vstr.32 s15,[r5,#0x4c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(76u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600295b0 vstr.32 s15,[r5,#0x28]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600295b4 vstr.32 s15,[r5,#0x2c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600295b8 vstr.32 s15,[r5,#0x30]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600295bc vstr.32 s15,[r5,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600295c0 vstr.32 s15,[r5,#0x38]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600295c4 vstr.32 s15,[r5,#0x3c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(60u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600295c8 vstr.32 s15,[r5,#0x40]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600295cc vstr.32 s15,[r5,#0x44]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600295d0 vstr.32 s15,[r5,#0x48]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(72u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600295d4 str.w r1,[r0,#0xf3c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(3900u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x600295d8 str.w r4,[r0,#0xfe4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(4068u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x600295dc vstr.32 s15,[lr,#0x4c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(88, 4)).wrapping_add(76u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600295e0 vstr.32 s15,[lr,#0x28]
        let mut t64000 = 0u64;
        t64000 = ((c.r(88, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600295e4 vstr.32 s15,[lr,#0x2c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(88, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600295e8 vstr.32 s15,[lr,#0x30]
        let mut t64000 = 0u64;
        t64000 = ((c.r(88, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600295ec vstr.32 s15,[lr,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(88, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600295f0 vstr.32 s15,[lr,#0x38]
        let mut t64000 = 0u64;
        t64000 = ((c.r(88, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600295f4 vstr.32 s15,[lr,#0x3c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(88, 4)).wrapping_add(60u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600295f8 vstr.32 s15,[lr,#0x40]
        let mut t64000 = 0u64;
        t64000 = ((c.r(88, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600295fc vstr.32 s15,[lr,#0x44]
        let mut t64000 = 0u64;
        t64000 = ((c.r(88, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029600 vstr.32 s15,[lr,#0x48]
        let mut t64000 = 0u64;
        t64000 = ((c.r(88, 4)).wrapping_add(72u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029604 str.w r1,[r0,#0xf9c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(3996u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x60029608 str r4,[r3,#0x44]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002960a vstr.32 s15,[r12,#0x4c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(80, 4)).wrapping_add(76u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002960e vstr.32 s15,[r12,#0x28]
        let mut t64000 = 0u64;
        t64000 = ((c.r(80, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029612 vstr.32 s15,[r12,#0x2c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(80, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029616 vstr.32 s15,[r12,#0x30]
        let mut t64000 = 0u64;
        t64000 = ((c.r(80, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002961a vstr.32 s15,[r12,#0x34]
        let mut t64000 = 0u64;
        t64000 = ((c.r(80, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002961e vstr.32 s15,[r12,#0x38]
        let mut t64000 = 0u64;
        t64000 = ((c.r(80, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029622 vstr.32 s15,[r12,#0x3c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(80, 4)).wrapping_add(60u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029626 vstr.32 s15,[r12,#0x40]
        let mut t64000 = 0u64;
        t64000 = ((c.r(80, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002962a vstr.32 s15,[r12,#0x44]
        let mut t64000 = 0u64;
        t64000 = ((c.r(80, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002962e vstr.32 s15,[r12,#0x48]
        let mut t64000 = 0u64;
        t64000 = ((c.r(80, 4)).wrapping_add(72u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029632 str.w r1,[r0,#0xffc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(4092u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x60029636 str.w r4,[r3,#0xa4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(44, 4)).wrapping_add(164u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002963a vstr.32 s15,[r3,#0x94]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(148u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002963e vstr.32 s15,[r3,#0x70]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(112u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029642 vstr.32 s15,[r3,#0x74]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(116u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029646 vstr.32 s15,[r3,#0x78]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(120u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002964a vstr.32 s15,[r3,#0x7c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(124u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002964e vstr.32 s15,[r3,#0x80]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(128u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029652 vstr.32 s15,[r3,#0x84]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(132u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029656 vstr.32 s15,[r3,#0x88]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(136u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002965a vstr.32 s15,[r3,#0x8c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(140u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002965e vstr.32 s15,[r3,#0x90]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(144u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029662 str r1,[r3,#0x5c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(92u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x60029664 str.w r4,[r3,#0x104]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(44, 4)).wrapping_add(260u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x60029668 vstr.32 s15,[r3,#0xf4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(244u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002966c vstr.32 s15,[r3,#0xd0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(208u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029670 vstr.32 s15,[r3,#0xd4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(212u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029674 vstr.32 s15,[r3,#0xd8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(216u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029678 vstr.32 s15,[r3,#0xdc]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(220u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002967c vstr.32 s15,[r3,#0xe0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(224u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029680 vstr.32 s15,[r3,#0xe4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(228u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029684 vstr.32 s15,[r3,#0xe8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(232u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029688 vstr.32 s15,[r3,#0xec]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(236u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002968c vstr.32 s15,[r3,#0xf0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(240u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029690 str.w r1,[r3,#0xbc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(44, 4)).wrapping_add(188u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x60029694 str.w r4,[r3,#0x164]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(44, 4)).wrapping_add(356u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x60029698 vstr.32 s15,[r3,#0x154]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(340u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002969c vstr.32 s15,[r3,#0x130]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(304u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600296a0 vstr.32 s15,[r3,#0x134]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(308u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600296a4 vstr.32 s15,[r3,#0x138]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(312u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600296a8 vstr.32 s15,[r3,#0x13c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(316u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600296ac vstr.32 s15,[r3,#0x140]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(320u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600296b0 vstr.32 s15,[r3,#0x144]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(324u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600296b4 vstr.32 s15,[r3,#0x148]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(328u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600296b8 vstr.32 s15,[r3,#0x14c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(332u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600296bc vstr.32 s15,[r3,#0x150]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(336u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600296c0 str.w r1,[r3,#0x11c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(44, 4)).wrapping_add(284u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x600296c4 str.w r4,[r3,#0x1c4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(44, 4)).wrapping_add(452u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x600296c8 vstr.32 s15,[r3,#0x1b4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(436u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600296cc vstr.32 s15,[r3,#0x190]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(400u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600296d0 vstr.32 s15,[r3,#0x194]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(404u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600296d4 vstr.32 s15,[r3,#0x198]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(408u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600296d8 vstr.32 s15,[r3,#0x19c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(412u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600296dc vstr.32 s15,[r3,#0x1a0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(416u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600296e0 vstr.32 s15,[r3,#0x1a4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(420u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600296e4 vstr.32 s15,[r3,#0x1a8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(424u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600296e8 vstr.32 s15,[r3,#0x1ac]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(428u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600296ec vstr.32 s15,[r3,#0x1b0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(432u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600296f0 str.w r1,[r3,#0x17c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(44, 4)).wrapping_add(380u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    0x600296f4
}
pub fn b_600296f4(c: &mut Machine) -> u32 {
    {
        // 0x600296f4 add sp,#0x1c
        let mut t187500 = 0u64;
        t187500 = (28u64) & 0xffffffffu64;
        let value = (c.r(84, 4)).wrapping_add(((t187500 >> 0) & 0xffffffffu64));
        c.w(84, 4, value);
    }
    {
        // 0x600296f6 pop.w {r4,r5,r6,r7,r8,r9,r10,r11,pc}
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
    0x600296fa
}
pub fn b_600296fc(c: &mut Machine) -> u32 {
    {
        // 0x600296fc add.w r0,r0,#0x1000
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
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029700 vmov.f32 s15,0x3f800000
        let value = 1065353216u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029704 vldr.32 s11,[pc,#0x28]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610782468u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(40u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x60029708 vldr.32 s12,[r0,#0x2c0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(704u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x6002970c vdiv.f32 s13,s15,s0
        let value = (f32::from_bits(c.r(828, 4) as u32) / f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x60029710 vldr.32 s14,[pc,#0x20]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610782480u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(32u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x60029714 vdiv.f32 s15,s11,s12
        let value = (f32::from_bits(c.r(812, 4) as u32) / f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029718 vmul.f32 s15,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002971c vcmpe.f32 s15,s14
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
        // 0x60029720 vmrs apsr,fpscr
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
        // 0x60029724 it pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x60029726 vmov.pl.f32 s15,s14
        let mut tf00 = 0u64;
        let mut t2700 = 0u64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2700 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x6002972au32;
        }
        let value = c.r(824, 4);
        c.w(828, 4, value);
    }
    0x6002972a
}
pub fn b_6002972a(c: &mut Machine) -> u32 {
    {
        // 0x6002972a vstr.32 s15,[r0,#0x2d0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(720u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002972e bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x60029730
}
pub fn b_60029738(c: &mut Machine) -> u32 {
    {
        // 0x60029738 add.w r0,r0,#0x1000
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
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002973c vldr.32 s15,[r0,#0x308]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(776u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x60029740 vmul.f32 s15,s15,s0
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029744 vstr.32 s15,[r0,#0x30c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(780u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029748 bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x6002974a
}
pub fn b_6002974c(c: &mut Machine) -> u32 {
    {
        // 0x6002974c add.w r0,r0,#0x1000
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
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029750 vmov.f32 s15,0x3f800000
        let value = 1065353216u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029754 vldr.32 s10,[pc,#0x2c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610782548u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(44u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x60029758 vldr.32 s11,[r0,#0x2e4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(740u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x6002975c vdiv.f32 s12,s15,s0
        let value = (f32::from_bits(c.r(828, 4) as u32) / f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x60029760 vldr.32 s13,[pc,#0x24]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610782560u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(36u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x60029764 vdiv.f32 s14,s10,s11
        let value = (f32::from_bits(c.r(808, 4) as u32) / f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x60029768 vmul.f32 s14,s14,s12
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002976c vcmpe.f32 s14,s13
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
        // 0x60029770 vmrs apsr,fpscr
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
        // 0x60029774 ite mi
        let mut t184c00 = 0u64;
        t184c00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x60029776 vsub.mi.f32 s15,s15,s14
        let mut td00 = 0u64;
        let mut t2600 = 0u64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2600 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2600 >> 0) & 0xffu64) != 0 {
            return 0x6002977au32;
        }
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    0x6002977a
}
pub fn b_6002977a(c: &mut Machine) -> u32 {
    {
        // 0x6002977a vldr.pl.32 s15,[pc,#0x10]
        let mut tf00 = 0u64;
        let mut t2700 = 0u64;
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610782586u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(16u64)) & 0xffffffffu64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2700 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x6002977eu32;
        }
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    0x6002977e
}
pub fn b_6002977e(c: &mut Machine) -> u32 {
    {
        // 0x6002977e vstr.32 s15,[r0,#0x2f0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(752u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029782 bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x60029784
}
pub fn b_60029790(c: &mut Machine) -> u32 {
    {
        // 0x60029790 vcvt.s32.f32 s15,s0
        let value = (f32::from_bits(c.r(768, 4) as u32) as i32) as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029794 vmov r3,s15
        let value = c.r(828, 4);
        c.w(44, 4, value);
    }
    {
        // 0x60029798 cbnz r3,0x600297a4
        let mut t1a5900 = 0u64;
        t1a5900 = ((c.r(44, 4) != 0u64) as u64) & 0xffu64;
        if ((t1a5900 >> 0) & 0xffu64) != 0 {
            return 0x600297a4u32;
        }
    }
    0x6002979a
}
pub fn b_6002979a(c: &mut Machine) -> u32 {
    {
        // 0x6002979a add.w r0,r0,#0x1000
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
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002979e strh.w r3,[r0,#0x384]
        let mut t19c800 = 0u64;
        let mut t1fb700 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(900u64)) & 0xffffffffu64;
        t1fb700 = ((c.r(44, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            2,
            ((t1fb700 >> 0) & 0xffffu64),
        );
    }
    {
        // 0x600297a2 bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x600297a4
}
pub fn b_600297a4(c: &mut Machine) -> u32 {
    {
        // 0x600297a4 cmp r3,#0x1
        let mut t1a6800 = 0u64;
        let value = (1u64 <= c.r(44, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), 1u64, 4, true);
        c.w(103, 1, value);
        t1a6800 = ((c.r(44, 4)).wrapping_sub(1u64)) & 0xffffffffu64;
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
        // 0x600297a6 beq 0x600297b2
        let mut t180200 = 0u64;
        t180200 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        if ((t180200 >> 0) & 0xffu64) != 0 {
            return 0x600297b2u32;
        }
    }
    0x600297a8
}
pub fn b_600297a8(c: &mut Machine) -> u32 {
    {
        // 0x600297a8 cmp r3,#0x2
        let mut t1a6800 = 0u64;
        let value = (2u64 <= c.r(44, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), 2u64, 4, true);
        c.w(103, 1, value);
        t1a6800 = ((c.r(44, 4)).wrapping_sub(2u64)) & 0xffffffffu64;
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
        // 0x600297aa beq 0x600297ce
        let mut t180200 = 0u64;
        t180200 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        if ((t180200 >> 0) & 0xffu64) != 0 {
            return 0x600297ceu32;
        }
    }
    0x600297ac
}
pub fn b_600297ac(c: &mut Machine) -> u32 {
    {
        // 0x600297ac cmp r3,#0x3
        let mut t1a6800 = 0u64;
        let value = (3u64 <= c.r(44, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), 3u64, 4, true);
        c.w(103, 1, value);
        t1a6800 = ((c.r(44, 4)).wrapping_sub(3u64)) & 0xffffffffu64;
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
        // 0x600297ae beq 0x600297c0
        let mut t180200 = 0u64;
        t180200 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        if ((t180200 >> 0) & 0xffu64) != 0 {
            return 0x600297c0u32;
        }
    }
    0x600297b0
}
pub fn b_600297b0(c: &mut Machine) -> u32 {
    {
        // 0x600297b0 bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x600297b2
}
pub fn b_600297b2(c: &mut Machine) -> u32 {
    {
        // 0x600297b2 add.w r0,r0,#0x1000
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
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600297b6 movw r3,#0x101
        let value = 257u64;
        c.w(44, 4, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600297ba strh.w r3,[r0,#0x384]
        let mut t19c800 = 0u64;
        let mut t1fb700 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(900u64)) & 0xffffffffu64;
        t1fb700 = ((c.r(44, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            2,
            ((t1fb700 >> 0) & 0xffffu64),
        );
    }
    {
        // 0x600297be bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x600297c0
}
pub fn b_600297c0(c: &mut Machine) -> u32 {
    {
        // 0x600297c0 add.w r0,r0,#0x1000
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
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600297c4 mov.w r3,#0x100
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (256u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((t188500 >> 0) & 0xffffffffu64);
        c.w(44, 4, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600297c8 strh.w r3,[r0,#0x384]
        let mut t19c800 = 0u64;
        let mut t1fb700 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(900u64)) & 0xffffffffu64;
        t1fb700 = ((c.r(44, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            2,
            ((t1fb700 >> 0) & 0xffffu64),
        );
    }
    {
        // 0x600297cc b 0x600297b0
        return 0x600297b0u32;
    }
    0x600297ce
}
pub fn b_600297ce(c: &mut Machine) -> u32 {
    {
        // 0x600297ce add.w r0,r0,#0x1000
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
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600297d2 movs r3,#0x1
        let value = 1u64;
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
        // 0x600297d4 strh.w r3,[r0,#0x384]
        let mut t19c800 = 0u64;
        let mut t1fb700 = 0u64;
        t19c800 = ((c.r(32, 4)).wrapping_add(900u64)) & 0xffffffffu64;
        t1fb700 = ((c.r(44, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            2,
            ((t1fb700 >> 0) & 0xffffu64),
        );
    }
    {
        // 0x600297d8 bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x600297da
}
pub fn b_6002982c(c: &mut Machine) -> u32 {
    {
        // 0x6002982c push {r4,lr}
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
        // 0x6002982e vpush {d8}
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
        // 0x60029832 mov r4,r0
        let value = c.r(32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x60029834 vmov.f32 s16,s0
        let value = c.r(768, 4);
        c.w(832, 4, value);
    }
    {
        // 0x60029838 bl 0x6004137c
        let value = (1610782780u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x6004137cu32;
    }
    0x6002983c
}
pub fn b_6002983c(c: &mut Machine) -> u32 {
    {
        // 0x6002983c vmov.f32 s14,0x3f800000
        let value = 1065353216u64;
        c.w(824, 4, value);
    }
    {
        // 0x60029840 vmov.f32 s15,0x3f000000
        let value = 1056964608u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029844 vldr.32 s13,[pc,#0x20]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610782788u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(32u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x60029848 add.w r0,r4,#0x1000
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
        let value = ((c.r(48, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002984c vsub.f32 s14,s14,s0
        let value = (f32::from_bits(c.r(824, 4) as u32) - f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x60029850 vfma.f32 s15,s14,s13
        let value = f32::from_bits(c.r(824, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(820, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029854 vmul.f32 s16,s16,s15
        let value = (f32::from_bits(c.r(832, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(832, 4, value);
    }
    {
        // 0x60029858 vstr.32 s15,[r0,#0x2ac]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(684u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002985c vstr.32 s16,[r0,#0x2b0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(688u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(832, 4));
    }
    {
        // 0x60029860 vpop {d8}
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
        // 0x60029864 pop {r4,pc}
        let mut t0 = 0u64;
        let value = c.r(84, 4);
        c.w(128, 4, value);
        let value = c.load(c.r(128, 4) as u32, 4);
        c.w(48, 4, value);
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
    0x60029866
}
pub fn b_6002986c(c: &mut Machine) -> u32 {
    {
        // 0x6002986c ldr r3,[0x60029890]
        let value = c.load(0x60029890, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002986e ldrb r3,[r3,#0x0]
        let mut t19e400 = 0u64;
        let mut t1a7d00 = 0u64;
        t19e400 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        t1a7d00 = (c.load(((t19e400 >> 0) & 0xffffffffu64) as u32, 1)) & 0xffu64;
        let value = ((t1a7d00 >> 0) & 0xffu64);
        c.w(44, 4, value);
    }
    {
        // 0x60029870 cbnz r3,0x60029888
        let mut t1a5900 = 0u64;
        t1a5900 = ((c.r(44, 4) != 0u64) as u64) & 0xffu64;
        if ((t1a5900 >> 0) & 0xffu64) != 0 {
            return 0x60029888u32;
        }
    }
    0x60029872
}
pub fn b_60029872(c: &mut Machine) -> u32 {
    {
        // 0x60029872 vldr.32 s15,[pc,#0x20]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610782834u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(32u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x60029876 vmul.f32 s0,s0,s15
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002987a vcvt.u32.f32 s0,s0
        let mut tade00 = 0u64;
        let mut tadf00 = 0u64;
        let mut tae000 = 0u64;
        tade00 =
            ((f32::from_bits(0u64 as u32) < f32::from_bits(c.r(768, 4) as u32)) as u64) & 0xffu64;
        tadf00 = ((tade00 >> 0) & 0xffu64) & 0xffffffffu64;
        tae000 = ((f32::from_bits(c.r(768, 4) as u32) as i32) as u64) & 0xffffffffu64;
        let value = ((tadf00 >> 0) & 0xffffffffu64).wrapping_mul(((tae000 >> 0) & 0xffffffffu64));
        c.w(768, 4, value);
    }
    {
        // 0x6002987e vmov r3,s0
        let value = c.r(768, 4);
        c.w(44, 4, value);
    }
    {
        // 0x60029882 uxth r0,r3
        let mut t222100 = 0u64;
        t222100 = ((c.r(44, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffffu64;
        let value = ((t222100 >> 0) & 0xffffu64);
        c.w(32, 4, value);
    }
    {
        // 0x60029884 b.w 0x60033dc0
        return 0x60033dc0u32;
        return 0xfffffff0u32;
    }
    0x60029888
}
pub fn b_60029888(c: &mut Machine) -> u32 {
    {
        // 0x60029888 movs r0,#0x0
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
        // 0x6002988a b.w 0x60033dc0
        return 0x60033dc0u32;
        return 0xfffffff0u32;
    }
    0x6002988e
}
pub fn b_60029898(c: &mut Machine) -> u32 {
    {
        // 0x60029898 vldr.64 d4,[r0,#0x90]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(144u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 8);
        c.w(800, 8, value);
    }
    {
        // 0x6002989c vldr.64 d7,[r0,#0x10]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 8);
        c.w(824, 8, value);
    }
    {
        // 0x600298a0 push {r3,r4,r5,r6,r7,lr}
        let value = c.r(84, 4);
        c.w(128, 4, value);
        let value = (c.r(128, 4)).wrapping_sub(4u64);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 4, c.r(88, 4));
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
        c.store(c.r(128, 4) as u32, 4, c.r(44, 4));
        let value = c.r(128, 4);
        c.w(84, 4, value);
    }
    {
        // 0x600298a2 vpush {d8,d9}
        let mut t73700 = 0u64;
        t73700 = ((4u64).wrapping_mul(4u64)) & 0xffffffffu64;
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
    }
    {
        // 0x600298a6 vcvt.f32.f64 s18,d4
        let value = (f64::from_bits(c.r(800, 8) as u64) as f32).to_bits() as u64;
        c.w(840, 4, value);
    }
    {
        // 0x600298aa vldr.32 s16,[r0,#0xa0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(160u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(832, 4, value);
    }
    {
        // 0x600298ae vcvt.f32.f64 s14,d7
        let value = (f64::from_bits(c.r(824, 8) as u64) as f32).to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x600298b2 vldr.32 s15,[r0,#0x18]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(24u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x600298b6 vldr.32 s7,[r0,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x600298ba mov r4,r0
        let value = c.r(32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x600298bc vmul.f32 s15,s18,s15
        let value = (f32::from_bits(c.r(840, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x600298c0 vldr.32 s5,[r0,#0x9c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(156u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(788, 4, value);
    }
    {
        // 0x600298c4 vcvt.f64.f32 d6,s14
        let value = (f32::from_bits(c.r(824, 4) as u32) as f64).to_bits() as u64;
        c.w(816, 8, value);
    }
    {
        // 0x600298c8 vldr.32 s14,[r0,#0x24]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x600298cc vsub.f32 s7,s7,s16
        let value = (f32::from_bits(c.r(796, 4) as u32) - f32::from_bits(c.r(832, 4) as u32))
            .to_bits() as u64;
        c.w(796, 4, value);
    }
    {
        // 0x600298d0 ldr r3,[r0,#0x38]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(32, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x600298d2 vcmpe.f32 s15,#0
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
        // 0x600298d6 vldr.32 s6,[pc,#0x284]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610782934u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(644u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x600298da vcvt.f32.s32 s14,s14
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
        // 0x600298de ldr r6,[r0,#0x20]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(32, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(56, 4, value);
    }
    {
        // 0x600298e0 vfma.f32 s16,s7,s5
        let value = f32::from_bits(c.r(796, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(788, 4) as u32),
                f32::from_bits(c.r(832, 4) as u32),
            )
            .to_bits() as u64;
        c.w(832, 4, value);
    }
    {
        // 0x600298e4 adds r3,#0x1
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
        // 0x600298e6 vmrs apsr,fpscr
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
        // 0x600298ea and.w r3,r3,r6
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (c.r(44, 4) & c.r(56, 4));
        c.w(44, 4, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600298ee it mi
        let mut t184c00 = 0u64;
        t184c00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x600298f0 vldr.mi.32 s15,[pc,#0x26c]
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610782960u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(620u64)) & 0xffffffffu64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x600298f4u32;
        }
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    0x600298f4
}
pub fn b_600298f4(c: &mut Machine) -> u32 {
    {
        // 0x600298f4 str r3,[r0,#0x38]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(32, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x600298f6 vcmpe.f32 s14,s15
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
        // 0x600298fa vadd.f32 s16,s16,s6
        let value = (f32::from_bits(c.r(832, 4) as u32) + f32::from_bits(c.r(792, 4) as u32))
            .to_bits() as u64;
        c.w(832, 4, value);
    }
    {
        // 0x600298fe vmrs apsr,fpscr
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
        // 0x60029902 vstr.32 s16,[r0,#0xa0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(160u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(832, 4));
    }
    {
        // 0x60029906 vldr.64 d5,[r0,#0x88]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(136u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 8);
        c.w(808, 8, value);
    }
    {
        // 0x6002990a it mi
        let mut t184c00 = 0u64;
        t184c00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x6002990c vmov.mi.f32 s15,s14
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x60029910u32;
        }
        let value = c.r(824, 4);
        c.w(828, 4, value);
    }
    0x60029910
}
pub fn b_60029910(c: &mut Machine) -> u32 {
    {
        // 0x60029910 vfma.f64 d4,d6,d5
        let value = f64::from_bits(c.r(816, 8) as u64)
            .fast_mul_add(
                f64::from_bits(c.r(808, 8) as u64),
                f64::from_bits(c.r(800, 8) as u64),
            )
            .to_bits() as u64;
        c.w(800, 8, value);
    }
    {
        // 0x60029914 vstr.32 s15,[r0,#0x28]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029918 ldr r5,[r4,#0x1c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(52, 4, value);
    }
    {
        // 0x6002991a vcvt.u32.f32 s14,s15
        let mut tade00 = 0u64;
        let mut tadf00 = 0u64;
        let mut tae000 = 0u64;
        tade00 =
            ((f32::from_bits(0u64 as u32) < f32::from_bits(c.r(828, 4) as u32)) as u64) & 0xffu64;
        tadf00 = ((tade00 >> 0) & 0xffu64) & 0xffffffffu64;
        tae000 = ((f32::from_bits(c.r(828, 4) as u32) as i32) as u64) & 0xffffffffu64;
        let value = ((tadf00 >> 0) & 0xffffffffu64).wrapping_mul(((tae000 >> 0) & 0xffffffffu64));
        c.w(824, 4, value);
    }
    {
        // 0x6002991e vmov r2,s14
        let value = c.r(824, 4);
        c.w(40, 4, value);
    }
    {
        // 0x60029922 vcvt.f32.s32 s14,s14
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
        // 0x60029926 str r2,[r0,#0x2c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(32, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x60029928 vsub.f32 s15,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002992c vstr.64 d4,[r0,#0x90]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(144u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 8, c.r(800, 8));
    }
    {
        // 0x60029930 vstr.32 s15,[r0,#0x30]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029934 ldr r0,[r0,#0x3c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(32, 4)).wrapping_add(60u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x60029936 cmp r0,r5
        let mut t1a6e00 = 0u64;
        let value = (c.r(52, 4) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), c.r(52, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(32, 4)).wrapping_sub(c.r(52, 4))) & 0xffffffffu64;
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
        // 0x60029938 bge 0x6002993e
        let mut t181800 = 0u64;
        t181800 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        if ((t181800 >> 0) & 0xffu64) != 0 {
            return 0x6002993eu32;
        }
    }
    0x6002993a
}
pub fn b_6002993a(c: &mut Machine) -> u32 {
    {
        // 0x6002993a adds r0,#0x1
        let value = ((c.r(32, 4) as u128 + 1u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 1u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(1u64);
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
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
        // 0x6002993c str r0,[r4,#0x3c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(60u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    0x6002993e
}
pub fn b_6002993e(c: &mut Machine) -> u32 {
    {
        // 0x6002993e ldr r5,[r4,#0x48]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(72u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(52, 4, value);
    }
    {
        // 0x60029940 cmp r2,r0
        let mut t1a6e00 = 0u64;
        let value = (c.r(32, 4) <= c.r(40, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(40, 4), c.r(32, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(40, 4)).wrapping_sub(c.r(32, 4))) & 0xffffffffu64;
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
        // 0x60029942 add.w r7,r5,r3, lsl #0x2
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
        let value = ((c.r(52, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(52, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(52, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(60, 4, value);
        let value = (signed(c.r(60, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(60, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029946 vstr.32 s16,[r7]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(60, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(832, 4));
    }
    {
        // 0x6002994a ble.w 0x60029b44
        let mut t184000 = 0u64;
        let mut t184200 = 0u64;
        t184000 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t184200 = (((c.r(97, 1) != 0) | (((t184000 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t184200 >> 0) & 0xffu64) != 0 {
            return 0x60029b44u32;
        }
    }
    0x6002994e
}
pub fn b_6002994e(c: &mut Machine) -> u32 {
    {
        // 0x6002994e vldr.32 s11,[r4,#0x40]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    0x60029952
}
pub fn b_60029952(c: &mut Machine) -> u32 {
    {
        // 0x60029952 vldr.32 s15,[r4,#0x50]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(80u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x60029956 vldr.32 s14,[r4,#0x5c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(92u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002995a vmul.f32 s15,s18,s15
        let value = (f32::from_bits(c.r(840, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002995e ldr.w r0,[r4,#0x80]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(128u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x60029962 vcvt.f32.s32 s14,s14
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
        // 0x60029966 ldr r7,[r4,#0x58]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(88u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(60, 4, value);
    }
    {
        // 0x60029968 vcmpe.f32 s15,#0
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
        // 0x6002996c ldrd r3,r6,[r4,#0x70]
        let mut t19dc00 = 0u64;
        let mut t1ab800 = 0u64;
        t19dc00 = ((c.r(48, 4)).wrapping_add(112u64)) & 0xffffffffu64;
        let value = c.load(((t19dc00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
        t1ab800 = (((t19dc00 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t1ab800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(56, 4, value);
    }
    {
        // 0x60029970 vmrs apsr,fpscr
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
        // 0x60029974 it mi
        let mut t184c00 = 0u64;
        t184c00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x60029976 vldr.mi.32 s15,[pc,#0x1e8]
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610783094u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(488u64)) & 0xffffffffu64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x6002997au32;
        }
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    0x6002997a
}
pub fn b_6002997a(c: &mut Machine) -> u32 {
    {
        // 0x6002997a vcmpe.f32 s14,s15
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
        // 0x6002997e vmrs apsr,fpscr
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
        // 0x60029982 it mi
        let mut t184c00 = 0u64;
        t184c00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x60029984 vmov.mi.f32 s15,s14
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x60029988u32;
        }
        let value = c.r(824, 4);
        c.w(828, 4, value);
    }
    0x60029988
}
pub fn b_60029988(c: &mut Machine) -> u32 {
    {
        // 0x60029988 vcvt.u32.f32 s14,s15
        let mut tade00 = 0u64;
        let mut tadf00 = 0u64;
        let mut tae000 = 0u64;
        tade00 =
            ((f32::from_bits(0u64 as u32) < f32::from_bits(c.r(828, 4) as u32)) as u64) & 0xffu64;
        tadf00 = ((tade00 >> 0) & 0xffu64) & 0xffffffffu64;
        tae000 = ((f32::from_bits(c.r(828, 4) as u32) as i32) as u64) & 0xffffffffu64;
        let value = ((tadf00 >> 0) & 0xffffffffu64).wrapping_mul(((tae000 >> 0) & 0xffffffffu64));
        c.w(824, 4, value);
    }
    {
        // 0x6002998c vstr.32 s15,[r4,#0x60]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(96u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029990 vmov r5,s14
        let value = c.r(824, 4);
        c.w(52, 4, value);
    }
    {
        // 0x60029994 vcvt.f32.s32 s14,s14
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
        // 0x60029998 cmp r5,r6
        let mut t1a6e00 = 0u64;
        let value = (c.r(56, 4) <= c.r(52, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(52, 4), c.r(56, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(52, 4)).wrapping_sub(c.r(56, 4))) & 0xffffffffu64;
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
        // 0x6002999a str r5,[r4,#0x64]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(100u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002999c vsub.f32 s15,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x600299a0 vstr.32 s15,[r4,#0x68]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(104u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600299a4 ble.w 0x60029ad6
        let mut t184000 = 0u64;
        let mut t184200 = 0u64;
        t184000 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t184200 = (((c.r(97, 1) != 0) | (((t184000 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t184200 >> 0) & 0xffu64) != 0 {
            return 0x60029ad6u32;
        }
    }
    0x600299a8
}
pub fn b_600299a8(c: &mut Machine) -> u32 {
    {
        // 0x600299a8 vldr.32 s14,[r4,#0x78]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(120u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    0x600299ac
}
pub fn b_600299ac(c: &mut Machine) -> u32 {
    {
        // 0x600299ac ldr r2,[0x60029b64]
        let value = c.load(0x60029b64, 4);
        c.w(40, 4, value);
    }
    {
        // 0x600299ae adds r3,#0x1
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
        // 0x600299b0 vstr.32 s14,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x600299b4 vldr.32 s15,[r2,#0x228]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(552u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x600299b8 ands r3,r7
        let value = (c.r(44, 4) & c.r(60, 4));
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
        // 0x600299ba vldr.32 s13,[r4,#0xa4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(164u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x600299be vldr.32 s12,[pc,#0x19c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610783166u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(412u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x600299c2 vfma.f32 s13,s15,s14
        let value = f32::from_bits(c.r(828, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(824, 4) as u32),
                f32::from_bits(c.r(820, 4) as u32),
            )
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x600299c6 vstr.32 s13,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x600299ca vldr.32 s15,[r2,#0x224]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(548u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x600299ce vldr.32 s10,[r2,#0x22c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(556u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x600299d2 vnmul.f32 s15,s15,s13
        let mut t71900 = 0u64;
        t71900 = ((f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64)
            & 0xffffffffu64;
        let value = (-f32::from_bits(((t71900 >> 0) & 0xffffffffu64) as u32)).to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x600299d6 vldr.32 s13,[r4,#0xa8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(168u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x600299da vldr.32 s9,[r2,#0x240]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(576u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(804, 4, value);
    }
    {
        // 0x600299de vfma.f32 s15,s10,s14
        let value = f32::from_bits(c.r(808, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(824, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x600299e2 vmov.f32 s14,s13
        let value = c.r(820, 4);
        c.w(824, 4, value);
    }
    {
        // 0x600299e6 vadd.f32 s15,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x600299ea vstr.32 s15,[r4,#0xa4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(164u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x600299ee vldr.32 s10,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x600299f2 vfma.f32 s14,s10,s9
        let value = f32::from_bits(c.r(808, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x600299f6 vstr.32 s14,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x600299fa vldr.32 s15,[r2,#0x23c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(572u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x600299fe vldr.32 s9,[r2,#0x244]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(580u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(804, 4, value);
    }
    {
        // 0x60029a02 vnmul.f32 s15,s15,s14
        let mut t71900 = 0u64;
        t71900 = ((f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64)
            & 0xffffffffu64;
        let value = (-f32::from_bits(((t71900 >> 0) & 0xffffffffu64) as u32)).to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029a06 vldr.32 s13,[r4,#0xc]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(12u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x60029a0a ldr r2,[r4,#0x54]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(84u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x60029a0c vfma.f32 s15,s10,s9
        let value = f32::from_bits(c.r(808, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029a10 cmp r2,r6
        let mut t1a6e00 = 0u64;
        let value = (c.r(56, 4) <= c.r(40, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(40, 4), c.r(56, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(40, 4)).wrapping_sub(c.r(56, 4))) & 0xffffffffu64;
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
        // 0x60029a12 vadd.f32 s15,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029a16 vstr.32 s15,[r4,#0xa8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(168u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029a1a vldr.32 s15,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x60029a1e vfms.f32 s15,s13,s0
        let value = (-f32::from_bits(c.r(820, 4) as u32))
            .fast_mul_add(
                f32::from_bits(c.r(768, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029a22 vstr.32 s15,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029a26 vldr.32 s14,[r4,#0xc]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(12u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x60029a2a str r3,[r4,#0x70]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(112u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x60029a2c vfma.f32 s0,s15,s14
        let value = f32::from_bits(c.r(828, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(824, 4) as u32),
                f32::from_bits(c.r(768, 4) as u32),
            )
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x60029a30 vadd.f32 s15,s0,s12
        let value = (f32::from_bits(c.r(768, 4) as u32) + f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029a34 ble 0x60029a3a
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x60029a3au32;
        }
    }
    0x60029a36
}
pub fn b_60029a36(c: &mut Machine) -> u32 {
    {
        // 0x60029a36 adds r6,#0x1
        let value = ((c.r(56, 4) as u128 + 1u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(56, 4), 1u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(56, 4)).wrapping_add(1u64);
        c.w(56, 4, value);
        let value = (signed(c.r(56, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(56, 4) == 0u64) as u64;
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
        // 0x60029a38 str r6,[r4,#0x74]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(116u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    0x60029a3a
}
pub fn b_60029a3a(c: &mut Machine) -> u32 {
    {
        // 0x60029a3a vdiv.f32 s17,s11,s16
        let value = (f32::from_bits(c.r(812, 4) as u32) / f32::from_bits(c.r(832, 4) as u32))
            .to_bits() as u64;
        c.w(836, 4, value);
    }
    {
        // 0x60029a3e add.w r0,r0,r3, lsl #0x2
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
        let value = ((c.r(32, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029a42 vldr.32 s10,[pc,#0x124]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610783298u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(292u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x60029a46 vstr.32 s15,[r0]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(32, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029a4a vldr.32 s13,[pc,#0x120]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610783306u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(288u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x60029a4e vldr.32 s9,[pc,#0x120]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610783310u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(288u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(804, 4, value);
    }
    {
        // 0x60029a52 vmov.f32 s14,0x3f800000
        let value = 1065353216u64;
        c.w(824, 4, value);
    }
    {
        // 0x60029a56 vmov.f32 s12,0x40000000
        let value = 1073741824u64;
        c.w(816, 4, value);
    }
    {
        // 0x60029a5a vmov.f32 s11,0x41480000
        let value = 1095237632u64;
        c.w(812, 4, value);
    }
    {
        // 0x60029a5e vmov r3,s17
        let value = c.r(836, 4);
        c.w(44, 4, value);
    }
    {
        // 0x60029a62 bic r2,r3,#0xc0000000
        let mut t188100 = 0u64;
        let mut t188300 = 0u64;
        let mut t1a4200 = 0u64;
        t188100 = (3221225472u64) & 0xffffffffu64;
        t188300 = (if 31u64 >= 32 {
            0
        } else {
            ((t188100 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188300 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        t1a4200 = (!((t188100 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        let value = (c.r(44, 4) & ((t1a4200 >> 0) & 0xffffffffu64));
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029a66 ubfx r3,r3,#0x17,#0x8
        let mut t188900 = 0u64;
        let mut t188b00 = 0u64;
        let mut t21dc00 = 0u64;
        let mut t21de00 = 0u64;
        t188900 = (23u64) & 0xffffffffu64;
        t188b00 = (7u64) & 0xffffffffu64;
        t21dc00 = (((t188900 >> 0) & 0xffffffffu64).wrapping_add(((t188b00 >> 0) & 0xffffffffu64)))
            & 0xffffffffu64;
        t21de00 = ((31u64).wrapping_sub(((t21dc00 >> 0) & 0xffffffffu64))) & 0xffffffffu64;
        let value = if ((t21de00 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(44, 4)) << (((t21de00 >> 0) & 0xffffffffu64) as u32)
        };
        c.w(44, 4, value);
        t21de00 = ((31u64).wrapping_sub(((t188b00 >> 0) & 0xffffffffu64))) & 0xffffffffu64;
        let value = if ((t21de00 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(44, 4)) >> (((t21de00 >> 0) & 0xffffffffu64) as u32)
        };
        c.w(44, 4, value);
    }
    {
        // 0x60029a6a orr r2,r2,#0x3f800000
        let mut t188100 = 0u64;
        let mut t188300 = 0u64;
        t188100 = (1065353216u64) & 0xffffffffu64;
        t188300 = (if 31u64 >= 32 {
            0
        } else {
            ((t188100 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188300 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = (c.r(40, 4) | ((t188100 >> 0) & 0xffffffffu64));
        c.w(40, 4, value);
        let value = c.r(104, 1);
        c.w(102, 1, value);
        let value = c.r(99, 1);
        c.w(103, 1, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029a6e vmov s0,r3
        let value = c.r(44, 4);
        c.w(768, 4, value);
    }
    {
        // 0x60029a72 vmov s15,r2
        let value = c.r(40, 4);
        c.w(828, 4, value);
    }
    {
        // 0x60029a76 vcvt.f32.s32 s0,s0
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
        let value = c.userop(219, c.r(768, 4), ((t3b500 >> 0) & 0xffu64), 0u64, 4, 4);
        c.w(768, 4, value);
    }
    {
        // 0x60029a7a vsub.f32 s15,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029a7e vsub.f32 s0,s0,s9
        let value = (f32::from_bits(c.r(768, 4) as u32) - f32::from_bits(c.r(804, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x60029a82 vfma.f32 s13,s15,s10
        let value = f32::from_bits(c.r(828, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(820, 4) as u32),
            )
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x60029a86 vsub.f32 s12,s12,s15
        let value = (f32::from_bits(c.r(816, 4) as u32) - f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x60029a8a vfma.f32 s0,s15,s13
        let value = f32::from_bits(c.r(828, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(820, 4) as u32),
                f32::from_bits(c.r(768, 4) as u32),
            )
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x60029a8e vsub.f32 s14,s14,s13
        let value = (f32::from_bits(c.r(824, 4) as u32) - f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x60029a92 vmul.f32 s14,s14,s15
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x60029a96 vfma.f32 s0,s14,s12
        let value = f32::from_bits(c.r(824, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(816, 4) as u32),
                f32::from_bits(c.r(768, 4) as u32),
            )
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x60029a9a vmul.f32 s0,s0,s11
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x60029a9e bl 0x6004137c
        let value = (1610783394u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x6004137cu32;
    }
    0x60029aa2
}
pub fn b_60029aa2(c: &mut Machine) -> u32 {
    {
        // 0x60029aa2 vmov.f32 s15,0x3e000000
        let value = 1040187392u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029aa6 vsub.f32 s16,s16,s18
        let value = (f32::from_bits(c.r(832, 4) as u32) - f32::from_bits(c.r(840, 4) as u32))
            .to_bits() as u64;
        c.w(832, 4, value);
    }
    {
        // 0x60029aaa vmov.f32 s14,0x3f800000
        let value = 1065353216u64;
        c.w(824, 4, value);
    }
    {
        // 0x60029aae vsub.f32 s0,s15,s0
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x60029ab2 vcmpe.f32 s0,#0
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
        // 0x60029ab6 vmrs apsr,fpscr
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
        // 0x60029aba it le
        let mut t186100 = 0u64;
        let mut t186200 = 0u64;
        let mut t186400 = 0u64;
        t186100 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t186200 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t186400 = (((((t186100 >> 0) & 0xffu64) != 0) & (((t186200 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
    }
    {
        // 0x60029abc vldr.le.32 s0,[pc,#0xa0]
        let mut t2200 = 0u64;
        let mut t2400 = 0u64;
        let mut t2700 = 0u64;
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610783420u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(160u64)) & 0xffffffffu64;
        t2200 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2400 = (((c.r(97, 1) != 0) | (((t2200 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        t2700 = ((((t2400 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x60029ac0u32;
        }
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    0x60029ac0
}
pub fn b_60029ac0(c: &mut Machine) -> u32 {
    {
        // 0x60029ac0 vfma.f32 s14,s16,s0
        let value = f32::from_bits(c.r(832, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(768, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x60029ac4 vsub.f32 s14,s14,s17
        let value = (f32::from_bits(c.r(824, 4) as u32) - f32::from_bits(c.r(836, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x60029ac8 vpop {d8,d9}
        let mut t74b00 = 0u64;
        let value = c.r(84, 4);
        c.w(128, 4, value);
        t74b00 = ((4u64).wrapping_mul(4u64)) & 0xffffffffu64;
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
    }
    {
        // 0x60029acc vcvt.f64.f32 d7,s14
        let value = (f32::from_bits(c.r(824, 4) as u32) as f64).to_bits() as u64;
        c.w(824, 8, value);
    }
    {
        // 0x60029ad0 vstr.64 d7,[r4,#0x10]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 8, c.r(824, 8));
    }
    {
        // 0x60029ad4 pop {r3,r4,r5,r6,r7,pc}
        let mut t0 = 0u64;
        let value = c.r(84, 4);
        c.w(128, 4, value);
        let value = c.load(c.r(128, 4) as u32, 4);
        c.w(44, 4, value);
        let value = (c.r(128, 4)).wrapping_add(4u64);
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
    0x60029ad6
}
pub fn b_60029ad6(c: &mut Machine) -> u32 {
    {
        // 0x60029ad6 subs r2,r3,r5
        let value = (c.r(52, 4) <= c.r(44, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), c.r(52, 4), 4, true);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_sub(c.r(52, 4));
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
        // 0x60029ad8 cmp r5,#0x0
        let mut t1a6800 = 0u64;
        let value = (0u64 <= c.r(52, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(52, 4), 0u64, 4, true);
        c.w(103, 1, value);
        t1a6800 = ((c.r(52, 4)).wrapping_sub(0u64)) & 0xffffffffu64;
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
        // 0x60029ada and.w r12,r2,r7
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (c.r(40, 4) & c.r(60, 4));
        c.w(80, 4, value);
        let value = (signed(c.r(80, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(80, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029ade str r2,[r4,#0x7c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(124u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x60029ae0 add.w r12,r0,r12, lsl #0x2
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
        let value = ((c.r(32, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(80, 4, value);
        let value = (signed(c.r(80, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(80, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029ae4 vldr.32 s14,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x60029ae8 ble 0x60029b54
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x60029b54u32;
        }
    }
    0x60029aea
}
pub fn b_60029aea(c: &mut Machine) -> u32 {
    {
        // 0x60029aea adds r5,r2,#0x1
        let value = ((c.r(40, 4) as u128 + 1u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(40, 4), 1u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(40, 4)).wrapping_add(1u64);
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
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
        // 0x60029aec ands r5,r7
        let value = (c.r(52, 4) & c.r(60, 4));
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
    }
    {
        // 0x60029aee add.w r5,r0,r5, lsl #0x2
        let mut t189200 = 0u64;
        let mut t189300 = 0u64;
        let mut t189500 = 0u64;
        let mut t189800 = 0u64;
        let mut t1a0b00 = 0u64;
        t189200 = ((32u64).wrapping_sub(2u64)) & 0xffffffffu64;
        t189300 = (if ((t189200 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(52, 4)) >> (((t189200 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t189500 = (((t189300 >> 0) & 0xffffffffu64) & 1u64) & 0xffffffffu64;
        let value = ((t189500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        t189800 = (if 2u64 >= 32 {
            0
        } else {
            (c.r(52, 4)) << (2u64 as u32)
        }) & 0xffffffffu64;
        t1a0b00 = ((t189800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        let value = ((c.r(32, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029af2 vldr.32 s10,[r5]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(52, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    0x60029af6
}
pub fn b_60029af6(c: &mut Machine) -> u32 {
    {
        // 0x60029af6 subs r5,r2,#0x1
        let value = (1u64 <= c.r(40, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(40, 4), 1u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(40, 4)).wrapping_sub(1u64);
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
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
        // 0x60029af8 vmov.f32 s9,0x3f000000
        let value = 1056964608u64;
        c.w(804, 4, value);
    }
    {
        // 0x60029afc subs r2,#0x2
        let value = (2u64 <= c.r(40, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(40, 4), 2u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(40, 4)).wrapping_sub(2u64);
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
        // 0x60029afe ands r5,r7
        let value = (c.r(52, 4) & c.r(60, 4));
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
    }
    {
        // 0x60029b00 ands r2,r7
        let value = (c.r(40, 4) & c.r(60, 4));
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
        // 0x60029b02 add.w r5,r0,r5, lsl #0x2
        let mut t189200 = 0u64;
        let mut t189300 = 0u64;
        let mut t189500 = 0u64;
        let mut t189800 = 0u64;
        let mut t1a0b00 = 0u64;
        t189200 = ((32u64).wrapping_sub(2u64)) & 0xffffffffu64;
        t189300 = (if ((t189200 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(52, 4)) >> (((t189200 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t189500 = (((t189300 >> 0) & 0xffffffffu64) & 1u64) & 0xffffffffu64;
        let value = ((t189500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        t189800 = (if 2u64 >= 32 {
            0
        } else {
            (c.r(52, 4)) << (2u64 as u32)
        }) & 0xffffffffu64;
        t1a0b00 = ((t189800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        let value = ((c.r(32, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029b06 add.w r2,r0,r2, lsl #0x2
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
        let value = ((c.r(32, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029b0a vldr.32 s12,[r5]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(52, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x60029b0e vldr.32 s8,[r2]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(40, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x60029b12 vsub.f32 s10,s12,s10
        let value = (f32::from_bits(c.r(816, 4) as u32) - f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x60029b16 vsub.f32 s12,s14,s12
        let value = (f32::from_bits(c.r(824, 4) as u32) - f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x60029b1a vsub.f32 s8,s8,s14
        let value = (f32::from_bits(c.r(800, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(800, 4, value);
    }
    {
        // 0x60029b1e vmov.f32 s13,s12
        let value = c.r(816, 4);
        c.w(820, 4, value);
    }
    {
        // 0x60029b22 vfma.f32 s13,s10,s9
        let value = f32::from_bits(c.r(808, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(820, 4) as u32),
            )
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x60029b26 vadd.f32 s12,s12,s13
        let value = (f32::from_bits(c.r(816, 4) as u32) + f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x60029b2a vfma.f32 s12,s8,s9
        let value = f32::from_bits(c.r(800, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(816, 4) as u32),
            )
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x60029b2e vadd.f32 s13,s13,s12
        let value = (f32::from_bits(c.r(820, 4) as u32) + f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x60029b32 vfnms.f32 s13,s15,s12
        let mut t49a00 = 0u64;
        let mut t49b00 = 0u64;
        t49a00 = ((-f32::from_bits(c.r(820, 4) as u32)).to_bits() as u64) & 0xffffffffu64;
        t49b00 = ((f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64)
            & 0xffffffffu64;
        let value = (f32::from_bits(((t49a00 >> 0) & 0xffffffffu64) as u32)
            + f32::from_bits(((t49b00 >> 0) & 0xffffffffu64) as u32))
        .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x60029b36 vmul.f32 s13,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x60029b3a vfma.f32 s13,s10,s9
        let value = f32::from_bits(c.r(808, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(820, 4) as u32),
            )
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x60029b3e vfma.f32 s14,s15,s13
        let value = f32::from_bits(c.r(828, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(820, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x60029b42 b 0x600299ac
        return 0x600299acu32;
    }
    0x60029b44
}
pub fn b_60029b44(c: &mut Machine) -> u32 {
    {
        // 0x60029b44 subs r3,r3,r2
        let value = (c.r(40, 4) <= c.r(44, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), c.r(40, 4), 4, true);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_sub(c.r(40, 4));
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
        // 0x60029b46 ands r6,r3
        let value = (c.r(56, 4) & c.r(44, 4));
        c.w(56, 4, value);
        let value = (signed(c.r(56, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(56, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
    }
    {
        // 0x60029b48 str r3,[r4,#0x44]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x60029b4a add.w r5,r5,r6, lsl #0x2
        let mut t189200 = 0u64;
        let mut t189300 = 0u64;
        let mut t189500 = 0u64;
        let mut t189800 = 0u64;
        let mut t1a0b00 = 0u64;
        t189200 = ((32u64).wrapping_sub(2u64)) & 0xffffffffu64;
        t189300 = (if ((t189200 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(56, 4)) >> (((t189200 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t189500 = (((t189300 >> 0) & 0xffffffffu64) & 1u64) & 0xffffffffu64;
        let value = ((t189500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        t189800 = (if 2u64 >= 32 {
            0
        } else {
            (c.r(56, 4)) << (2u64 as u32)
        }) & 0xffffffffu64;
        t1a0b00 = ((t189800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        let value = ((c.r(52, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(52, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(52, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029b4e vldr.32 s11,[r5]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(52, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x60029b52 b 0x60029952
        return 0x60029952u32;
    }
    0x60029b54
}
pub fn b_60029b54(c: &mut Machine) -> u32 {
    {
        // 0x60029b54 vmov.f32 s10,s14
        let value = c.r(824, 4);
        c.w(808, 4, value);
    }
    {
        // 0x60029b58 b 0x60029af6
        return 0x60029af6u32;
    }
    0x60029b5a
}
pub fn b_60029b74(c: &mut Machine) -> u32 {
    {
        // 0x60029b74 push {r4,lr}
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
        // 0x60029b76 vpush {d8}
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
        // 0x60029b7a vldr.32 s17,[pc,#0x34]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610783610u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(52u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(836, 4, value);
    }
    {
        // 0x60029b7e vmov.f32 s16,s0
        let value = c.r(768, 4);
        c.w(832, 4, value);
    }
    {
        // 0x60029b82 mov r4,r0
        let value = c.r(32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x60029b84 vmul.f32 s0,s0,s17
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(836, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x60029b88 bl 0x60040c70
        let value = (1610783628u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x60029b8c
}
pub fn b_60029b8c(c: &mut Machine) -> u32 {
    {
        // 0x60029b8c vmov.f32 s15,0x3f800000
        let value = 1065353216u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029b90 add.w r4,r4,#0x1000
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
        // 0x60029b94 vstr.32 s0,[r4,#0x2e8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(744u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x60029b98 vsub.f32 s0,s16,s15
        let value = (f32::from_bits(c.r(832, 4) as u32) - f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x60029b9c vmul.f32 s0,s0,s17
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(836, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x60029ba0 bl 0x60040c70
        let value = (1610783652u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x60029ba4
}
pub fn b_60029ba4(c: &mut Machine) -> u32 {
    {
        // 0x60029ba4 vstr.32 s0,[r4,#0x2ec]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(748u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x60029ba8 vpop {d8}
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
        // 0x60029bac pop {r4,pc}
        let mut t0 = 0u64;
        let value = c.r(84, 4);
        c.w(128, 4, value);
        let value = c.load(c.r(128, 4) as u32, 4);
        c.w(48, 4, value);
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
    0x60029bae
}
pub fn b_60029bb4(c: &mut Machine) -> u32 {
    {
        // 0x60029bb4 vmov.f32 s15,0xc1700000
        let value = 3245342720u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029bb8 vmov.f32 s13,0x40000000
        let value = 1073741824u64;
        c.w(820, 4, value);
    }
    {
        // 0x60029bbc push {r4,lr}
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
        // 0x60029bbe add.w r4,r0,#0x1000
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
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029bc2 vsub.f32 s15,s15,s0
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029bc6 vldr.32 s14,[r4,#0x2d4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(724u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x60029bca vldr.32 s12,[r4,#0x2d8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(728u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x60029bce vadd.f32 s15,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029bd2 vstr.32 s0,[r4,#0x2c4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(708u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x60029bd6 vcmpe.f32 s15,#0
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
        // 0x60029bda vmrs apsr,fpscr
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
        // 0x60029bde it le
        let mut t186100 = 0u64;
        let mut t186200 = 0u64;
        let mut t186400 = 0u64;
        t186100 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t186200 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t186400 = (((((t186100 >> 0) & 0xffu64) != 0) & (((t186200 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
    }
    {
        // 0x60029be0 vldr.le.32 s15,[pc,#0x4c]
        let mut t2200 = 0u64;
        let mut t2400 = 0u64;
        let mut t2700 = 0u64;
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610783712u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(76u64)) & 0xffffffffu64;
        t2200 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2400 = (((c.r(97, 1) != 0) | (((t2200 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        t2700 = ((((t2400 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x60029be4u32;
        }
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    0x60029be4
}
pub fn b_60029be4(c: &mut Machine) -> u32 {
    {
        // 0x60029be4 vmul.f32 s15,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029be8 vcmpe.f32 s15,s13
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
        // 0x60029bec vmrs apsr,fpscr
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
        // 0x60029bf0 bpl 0x60029c28
        let mut t180c00 = 0u64;
        t180c00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        if ((t180c00 >> 0) & 0xffu64) != 0 {
            return 0x60029c28u32;
        }
    }
    0x60029bf2
}
pub fn b_60029bf2(c: &mut Machine) -> u32 {
    {
        // 0x60029bf2 vmov.f32 s13,0x3e800000
        let value = 1048576000u64;
        c.w(820, 4, value);
    }
    {
        // 0x60029bf6 vmov.f32 s12,0x40800000
        let value = 1082130432u64;
        c.w(816, 4, value);
    }
    {
        // 0x60029bfa vmul.f32 s13,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x60029bfe vfms.f32 s13,s13,s13
        let value = (-f32::from_bits(c.r(820, 4) as u32))
            .fast_mul_add(
                f32::from_bits(c.r(820, 4) as u32),
                f32::from_bits(c.r(820, 4) as u32),
            )
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x60029c02 vmul.f32 s13,s13,s12
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    0x60029c06
}
pub fn b_60029c06(c: &mut Machine) -> u32 {
    {
        // 0x60029c06 vsub.f32 s15,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029c0a vldr.32 s13,[r4,#0x2c8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(712u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x60029c0e vldr.32 s0,[pc,#0x24]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610783758u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(36u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x60029c12 vmul.f32 s14,s14,s15
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x60029c16 vmul.f32 s14,s14,s13
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x60029c1a vmul.f32 s0,s14,s0
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x60029c1e bl 0x60040c70
        let value = (1610783778u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x60029c22
}
pub fn b_60029c22(c: &mut Machine) -> u32 {
    {
        // 0x60029c22 vstr.32 s0,[r4,#0x380]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(896u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x60029c26 pop {r4,pc}
        let mut t0 = 0u64;
        let value = c.r(84, 4);
        c.w(128, 4, value);
        let value = c.load(c.r(128, 4) as u32, 4);
        c.w(48, 4, value);
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
    0x60029c28
}
pub fn b_60029c28(c: &mut Machine) -> u32 {
    {
        // 0x60029c28 vmov.f32 s13,0x3f800000
        let value = 1065353216u64;
        c.w(820, 4, value);
    }
    {
        // 0x60029c2c b 0x60029c06
        return 0x60029c06u32;
    }
    0x60029c2e
}
pub fn b_60029c38(c: &mut Machine) -> u32 {
    {
        // 0x60029c38 push {r4,lr}
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
        // 0x60029c3a add.w r4,r0,#0x1000
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
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029c3e vmov.f32 s15,0xc1700000
        let value = 3245342720u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029c42 vldr.32 s13,[r4,#0x2c4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(708u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x60029c46 vldr.32 s14,[r4,#0x2d4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(724u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x60029c4a vsub.f32 s15,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029c4e vldr.32 s12,[r4,#0x2d8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(728u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x60029c52 vmov.f32 s13,0x40000000
        let value = 1073741824u64;
        c.w(820, 4, value);
    }
    {
        // 0x60029c56 vstr.32 s0,[r4,#0x2c8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(712u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x60029c5a vadd.f32 s15,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029c5e vcmpe.f32 s15,#0
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
        // 0x60029c62 vmrs apsr,fpscr
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
        // 0x60029c66 it le
        let mut t186100 = 0u64;
        let mut t186200 = 0u64;
        let mut t186400 = 0u64;
        t186100 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t186200 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t186400 = (((((t186100 >> 0) & 0xffu64) != 0) & (((t186200 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
    }
    {
        // 0x60029c68 vldr.le.32 s15,[pc,#0x48]
        let mut t2200 = 0u64;
        let mut t2400 = 0u64;
        let mut t2700 = 0u64;
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610783848u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(72u64)) & 0xffffffffu64;
        t2200 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2400 = (((c.r(97, 1) != 0) | (((t2200 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        t2700 = ((((t2400 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x60029c6cu32;
        }
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    0x60029c6c
}
pub fn b_60029c6c(c: &mut Machine) -> u32 {
    {
        // 0x60029c6c vmul.f32 s15,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029c70 vcmpe.f32 s15,s13
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
        // 0x60029c74 vmrs apsr,fpscr
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
        // 0x60029c78 bpl 0x60029cac
        let mut t180c00 = 0u64;
        t180c00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        if ((t180c00 >> 0) & 0xffu64) != 0 {
            return 0x60029cacu32;
        }
    }
    0x60029c7a
}
pub fn b_60029c7a(c: &mut Machine) -> u32 {
    {
        // 0x60029c7a vmov.f32 s13,0x3e800000
        let value = 1048576000u64;
        c.w(820, 4, value);
    }
    {
        // 0x60029c7e vmov.f32 s12,0x40800000
        let value = 1082130432u64;
        c.w(816, 4, value);
    }
    {
        // 0x60029c82 vmul.f32 s13,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x60029c86 vfms.f32 s13,s13,s13
        let value = (-f32::from_bits(c.r(820, 4) as u32))
            .fast_mul_add(
                f32::from_bits(c.r(820, 4) as u32),
                f32::from_bits(c.r(820, 4) as u32),
            )
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x60029c8a vmul.f32 s13,s13,s12
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    0x60029c8e
}
pub fn b_60029c8e(c: &mut Machine) -> u32 {
    {
        // 0x60029c8e vsub.f32 s15,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029c92 vldr.32 s13,[pc,#0x24]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610783890u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(36u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x60029c96 vmul.f32 s14,s14,s15
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x60029c9a vmul.f32 s0,s14,s0
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x60029c9e vmul.f32 s0,s0,s13
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x60029ca2 bl 0x60040c70
        let value = (1610783910u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x60029ca6
}
pub fn b_60029ca6(c: &mut Machine) -> u32 {
    {
        // 0x60029ca6 vstr.32 s0,[r4,#0x380]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(896u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x60029caa pop {r4,pc}
        let mut t0 = 0u64;
        let value = c.r(84, 4);
        c.w(128, 4, value);
        let value = c.load(c.r(128, 4) as u32, 4);
        c.w(48, 4, value);
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
    0x60029cac
}
pub fn b_60029cac(c: &mut Machine) -> u32 {
    {
        // 0x60029cac vmov.f32 s13,0x3f800000
        let value = 1065353216u64;
        c.w(820, 4, value);
    }
    {
        // 0x60029cb0 b 0x60029c8e
        return 0x60029c8eu32;
    }
    0x60029cb2
}
pub fn b_60029cbc(c: &mut Machine) -> u32 {
    {
        // 0x60029cbc push {r4,lr}
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
        // 0x60029cbe vpush {d8}
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
        // 0x60029cc2 vmov.f32 s16,0xc1900000
        let value = 3247439872u64;
        c.w(832, 4, value);
    }
    {
        // 0x60029cc6 vldr.32 s14,[pc,#0xec]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610783942u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(236u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x60029cca vldr.32 s15,[pc,#0xec]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610783946u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(236u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x60029cce add.w r4,r0,#0x1000
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
        let value = ((c.r(32, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029cd2 vfma.f32 s16,s0,s14
        let value = f32::from_bits(c.r(768, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(824, 4) as u32),
                f32::from_bits(c.r(832, 4) as u32),
            )
            .to_bits() as u64;
        c.w(832, 4, value);
    }
    {
        // 0x60029cd6 vmul.f32 s16,s16,s15
        let value = (f32::from_bits(c.r(832, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(832, 4, value);
    }
    {
        // 0x60029cda vmov.f32 s0,s16
        let value = c.r(832, 4);
        c.w(768, 4, value);
    }
    {
        // 0x60029cde bl 0x60040c70
        let value = (1610783970u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x60029ce2
}
pub fn b_60029ce2(c: &mut Machine) -> u32 {
    {
        // 0x60029ce2 vldr.32 s15,[r4,#0x1e8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(488u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x60029ce6 vldr.32 s12,[r4,#0x1f4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(500u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x60029cea vmov.f32 s14,s0
        let value = c.r(768, 4);
        c.w(824, 4, value);
    }
    {
        // 0x60029cee vmul.f32 s15,s0,s15
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029cf2 vldr.32 s13,[pc,#0xc8]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610783986u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(200u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x60029cf6 vstr.32 s0,[r4,#0x1ec]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(492u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x60029cfa vmul.f32 s15,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029cfe vcmpe.f32 s15,s13
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
        // 0x60029d02 vmrs apsr,fpscr
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
        // 0x60029d06 bpl 0x60029da8
        let mut t180c00 = 0u64;
        t180c00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        if ((t180c00 >> 0) & 0xffu64) != 0 {
            return 0x60029da8u32;
        }
    }
    0x60029d08
}
pub fn b_60029d08(c: &mut Machine) -> u32 {
    {
        // 0x60029d08 vmul.f32 s13,s15,s15
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x60029d0c vldr.32 s9,[pc,#0xb0]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610784012u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(176u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(804, 4, value);
    }
    {
        // 0x60029d10 vldr.32 s10,[pc,#0xb0]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610784016u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(176u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x60029d14 vmov.f32 s12,0x3f800000
        let value = 1065353216u64;
        c.w(816, 4, value);
    }
    {
        // 0x60029d18 vmul.f32 s11,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x60029d1c vfma.f32 s15,s11,s9
        let value = f32::from_bits(c.r(812, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029d20 vmul.f32 s13,s13,s11
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x60029d24 vfma.f32 s15,s13,s10
        let value = f32::from_bits(c.r(820, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029d28 vadd.f32 s12,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x60029d2c vdiv.f32 s13,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) / f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    0x60029d30
}
pub fn b_60029d30(c: &mut Machine) -> u32 {
    {
        // 0x60029d30 vstr.32 s13,[r4,#0x1f8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(504u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x60029d34 vmov.f32 s15,0xbf800000
        let value = 3212836864u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029d38 vmov.f32 s0,s16
        let value = c.r(832, 4);
        c.w(768, 4, value);
    }
    {
        // 0x60029d3c vfma.f32 s15,s14,s14
        let value = f32::from_bits(c.r(824, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(824, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029d40 vstr.32 s15,[r4,#0x1f0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(496u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029d44 bl 0x60040c70
        let value = (1610784072u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x60029d48
}
pub fn b_60029d48(c: &mut Machine) -> u32 {
    {
        // 0x60029d48 vldr.32 s15,[r4,#0x200]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(512u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x60029d4c vldr.32 s13,[r4,#0x20c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(524u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x60029d50 vmul.f32 s15,s0,s15
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029d54 vldr.32 s14,[pc,#0x64]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610784084u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(100u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x60029d58 vstr.32 s0,[r4,#0x204]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(516u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x60029d5c vmul.f32 s15,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029d60 vcmpe.f32 s15,s14
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
        // 0x60029d64 vmrs apsr,fpscr
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
        // 0x60029d68 bpl 0x60029dae
        let mut t180c00 = 0u64;
        t180c00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        if ((t180c00 >> 0) & 0xffu64) != 0 {
            return 0x60029daeu32;
        }
    }
    0x60029d6a
}
pub fn b_60029d6a(c: &mut Machine) -> u32 {
    {
        // 0x60029d6a vmul.f32 s14,s15,s15
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x60029d6e vldr.32 s10,[pc,#0x50]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610784110u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(80u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x60029d72 vldr.32 s11,[pc,#0x50]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610784114u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(80u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x60029d76 vmov.f32 s13,0x3f800000
        let value = 1065353216u64;
        c.w(820, 4, value);
    }
    {
        // 0x60029d7a vmul.f32 s12,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x60029d7e vfma.f32 s15,s12,s10
        let value = f32::from_bits(c.r(816, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029d82 vmul.f32 s14,s14,s12
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x60029d86 vfma.f32 s15,s14,s11
        let value = f32::from_bits(c.r(824, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(812, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029d8a vadd.f32 s13,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x60029d8e vdiv.f32 s14,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) / f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    0x60029d92
}
pub fn b_60029d92(c: &mut Machine) -> u32 {
    {
        // 0x60029d92 vstr.32 s14,[r4,#0x210]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(528u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x60029d96 vmov.f32 s15,0xbf800000
        let value = 3212836864u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029d9a vpop {d8}
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
        // 0x60029d9e vfma.f32 s15,s0,s0
        let value = f32::from_bits(c.r(768, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(768, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029da2 vstr.32 s15,[r4,#0x208]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(520u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029da6 pop {r4,pc}
        let mut t0 = 0u64;
        let value = c.r(84, 4);
        c.w(128, 4, value);
        let value = c.load(c.r(128, 4) as u32, 4);
        c.w(48, 4, value);
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
    0x60029da8
}
pub fn b_60029da8(c: &mut Machine) -> u32 {
    {
        // 0x60029da8 vldr.32 s13,[pc,#0x1c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610784168u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(28u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x60029dac b 0x60029d30
        return 0x60029d30u32;
    }
    0x60029dae
}
pub fn b_60029dae(c: &mut Machine) -> u32 {
    {
        // 0x60029dae vldr.32 s14,[pc,#0x18]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610784174u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(24u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x60029db2 b 0x60029d92
        return 0x60029d92u32;
    }
    0x60029db4
}
pub fn b_60029dcc(c: &mut Machine) -> u32 {
    {
        // 0x60029dcc vldr.64 d4,[r0,#0x90]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(144u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 8);
        c.w(800, 8, value);
    }
    {
        // 0x60029dd0 vldr.64 d7,[r0,#0x10]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 8);
        c.w(824, 8, value);
    }
    {
        // 0x60029dd4 push {r4,r5,r6,lr}
        let value = c.r(84, 4);
        c.w(128, 4, value);
        let value = (c.r(128, 4)).wrapping_sub(4u64);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 4, c.r(88, 4));
        let value = (c.r(128, 4)).wrapping_sub(4u64);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 4, c.r(56, 4));
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
        // 0x60029dd6 vpush {d8,d9}
        let mut t73700 = 0u64;
        t73700 = ((4u64).wrapping_mul(4u64)) & 0xffffffffu64;
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
    }
    {
        // 0x60029dda vcvt.f32.f64 s18,d4
        let value = (f64::from_bits(c.r(800, 8) as u64) as f32).to_bits() as u64;
        c.w(840, 4, value);
    }
    {
        // 0x60029dde vldr.32 s16,[r0,#0xa0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(160u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(832, 4, value);
    }
    {
        // 0x60029de2 vcvt.f32.f64 s14,d7
        let value = (f64::from_bits(c.r(824, 8) as u64) as f32).to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x60029de6 vldr.32 s15,[r0,#0x18]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(24u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x60029dea vldr.32 s7,[r0,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x60029dee mov r4,r0
        let value = c.r(32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x60029df0 vmul.f32 s15,s18,s15
        let value = (f32::from_bits(c.r(840, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029df4 vldr.32 s5,[r0,#0x9c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(156u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(788, 4, value);
    }
    {
        // 0x60029df8 vcvt.f64.f32 d6,s14
        let value = (f32::from_bits(c.r(824, 4) as u32) as f64).to_bits() as u64;
        c.w(816, 8, value);
    }
    {
        // 0x60029dfc vldr.32 s14,[r0,#0x24]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x60029e00 vsub.f32 s7,s7,s16
        let value = (f32::from_bits(c.r(796, 4) as u32) - f32::from_bits(c.r(832, 4) as u32))
            .to_bits() as u64;
        c.w(796, 4, value);
    }
    {
        // 0x60029e04 ldr r3,[r0,#0x38]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(32, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x60029e06 vcmpe.f32 s15,#0
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
        // 0x60029e0a vldr.32 s6,[pc,#0x210]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610784266u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(528u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x60029e0e vcvt.f32.s32 s14,s14
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
        // 0x60029e12 ldr r6,[r0,#0x20]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(32, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(56, 4, value);
    }
    {
        // 0x60029e14 vfma.f32 s16,s7,s5
        let value = f32::from_bits(c.r(796, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(788, 4) as u32),
                f32::from_bits(c.r(832, 4) as u32),
            )
            .to_bits() as u64;
        c.w(832, 4, value);
    }
    {
        // 0x60029e18 adds r3,#0x1
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
        // 0x60029e1a vmrs apsr,fpscr
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
        // 0x60029e1e and.w r3,r3,r6
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (c.r(44, 4) & c.r(56, 4));
        c.w(44, 4, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029e22 it mi
        let mut t184c00 = 0u64;
        t184c00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x60029e24 vldr.mi.32 s15,[pc,#0x1f8]
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610784292u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(504u64)) & 0xffffffffu64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x60029e28u32;
        }
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    0x60029e28
}
pub fn b_60029e28(c: &mut Machine) -> u32 {
    {
        // 0x60029e28 str r3,[r0,#0x38]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(32, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x60029e2a vcmpe.f32 s14,s15
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
        // 0x60029e2e vadd.f32 s16,s16,s6
        let value = (f32::from_bits(c.r(832, 4) as u32) + f32::from_bits(c.r(792, 4) as u32))
            .to_bits() as u64;
        c.w(832, 4, value);
    }
    {
        // 0x60029e32 vmrs apsr,fpscr
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
        // 0x60029e36 vstr.32 s16,[r0,#0xa0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(160u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(832, 4));
    }
    {
        // 0x60029e3a vldr.64 d5,[r0,#0x88]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(136u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 8);
        c.w(808, 8, value);
    }
    {
        // 0x60029e3e it mi
        let mut t184c00 = 0u64;
        t184c00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x60029e40 vmov.mi.f32 s15,s14
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x60029e44u32;
        }
        let value = c.r(824, 4);
        c.w(828, 4, value);
    }
    0x60029e44
}
pub fn b_60029e44(c: &mut Machine) -> u32 {
    {
        // 0x60029e44 vfma.f64 d4,d6,d5
        let value = f64::from_bits(c.r(816, 8) as u64)
            .fast_mul_add(
                f64::from_bits(c.r(808, 8) as u64),
                f64::from_bits(c.r(800, 8) as u64),
            )
            .to_bits() as u64;
        c.w(800, 8, value);
    }
    {
        // 0x60029e48 vstr.32 s15,[r0,#0x28]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029e4c ldr r5,[r4,#0x1c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(52, 4, value);
    }
    {
        // 0x60029e4e vcvt.u32.f32 s14,s15
        let mut tade00 = 0u64;
        let mut tadf00 = 0u64;
        let mut tae000 = 0u64;
        tade00 =
            ((f32::from_bits(0u64 as u32) < f32::from_bits(c.r(828, 4) as u32)) as u64) & 0xffu64;
        tadf00 = ((tade00 >> 0) & 0xffu64) & 0xffffffffu64;
        tae000 = ((f32::from_bits(c.r(828, 4) as u32) as i32) as u64) & 0xffffffffu64;
        let value = ((tadf00 >> 0) & 0xffffffffu64).wrapping_mul(((tae000 >> 0) & 0xffffffffu64));
        c.w(824, 4, value);
    }
    {
        // 0x60029e52 vmov r2,s14
        let value = c.r(824, 4);
        c.w(40, 4, value);
    }
    {
        // 0x60029e56 vcvt.f32.s32 s14,s14
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
        // 0x60029e5a str r2,[r0,#0x2c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(32, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x60029e5c vsub.f32 s15,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029e60 vstr.64 d4,[r0,#0x90]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(144u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 8, c.r(800, 8));
    }
    {
        // 0x60029e64 vstr.32 s15,[r0,#0x30]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029e68 ldr r0,[r0,#0x3c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(32, 4)).wrapping_add(60u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x60029e6a cmp r0,r5
        let mut t1a6e00 = 0u64;
        let value = (c.r(52, 4) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), c.r(52, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(32, 4)).wrapping_sub(c.r(52, 4))) & 0xffffffffu64;
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
        // 0x60029e6c bge 0x60029e72
        let mut t181800 = 0u64;
        t181800 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        if ((t181800 >> 0) & 0xffu64) != 0 {
            return 0x60029e72u32;
        }
    }
    0x60029e6e
}
pub fn b_60029e6e(c: &mut Machine) -> u32 {
    {
        // 0x60029e6e adds r0,#0x1
        let value = ((c.r(32, 4) as u128 + 1u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 1u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(1u64);
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
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
        // 0x60029e70 str r0,[r4,#0x3c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(60u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    0x60029e72
}
pub fn b_60029e72(c: &mut Machine) -> u32 {
    {
        // 0x60029e72 ldr r5,[r4,#0x48]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(72u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(52, 4, value);
    }
    {
        // 0x60029e74 cmp r2,r0
        let mut t1a6e00 = 0u64;
        let value = (c.r(32, 4) <= c.r(40, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(40, 4), c.r(32, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(40, 4)).wrapping_sub(c.r(32, 4))) & 0xffffffffu64;
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
        // 0x60029e76 add.w r12,r5,r3, lsl #0x2
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
        let value = ((c.r(52, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(52, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(52, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(80, 4, value);
        let value = (signed(c.r(80, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(80, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029e7a vstr.32 s16,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(832, 4));
    }
    {
        // 0x60029e7e ble.w 0x60029f8e
        let mut t184000 = 0u64;
        let mut t184200 = 0u64;
        t184000 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t184200 = (((c.r(97, 1) != 0) | (((t184000 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t184200 >> 0) & 0xffu64) != 0 {
            return 0x60029f8eu32;
        }
    }
    0x60029e82
}
pub fn b_60029e82(c: &mut Machine) -> u32 {
    {
        // 0x60029e82 vldr.32 s11,[r4,#0x40]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    0x60029e86
}
pub fn b_60029e86(c: &mut Machine) -> u32 {
    {
        // 0x60029e86 vldr.32 s15,[r4,#0x50]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(80u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x60029e8a vldr.32 s14,[r4,#0x5c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(92u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x60029e8e vmul.f32 s15,s18,s15
        let value = (f32::from_bits(c.r(840, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029e92 ldr r3,[r4,#0x70]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(112u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x60029e94 vcvt.f32.s32 s14,s14
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
        // 0x60029e98 ldr r6,[r4,#0x58]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(88u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(56, 4, value);
    }
    {
        // 0x60029e9a adds r3,#0x1
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
        // 0x60029e9c ldr r5,[r4,#0x74]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(116u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(52, 4, value);
    }
    {
        // 0x60029e9e vcmpe.f32 s15,#0
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
        // 0x60029ea2 ldr r0,[r4,#0x54]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(84u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x60029ea4 ands r3,r6
        let value = (c.r(44, 4) & c.r(56, 4));
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
        // 0x60029ea6 vmrs apsr,fpscr
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
        // 0x60029eaa str r3,[r4,#0x70]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(112u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x60029eac it mi
        let mut t184c00 = 0u64;
        t184c00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x60029eae vldr.mi.32 s15,[pc,#0x170]
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610784430u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(368u64)) & 0xffffffffu64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x60029eb2u32;
        }
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    0x60029eb2
}
pub fn b_60029eb2(c: &mut Machine) -> u32 {
    {
        // 0x60029eb2 vcmpe.f32 s14,s15
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
        // 0x60029eb6 vmrs apsr,fpscr
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
        // 0x60029eba it mi
        let mut t184c00 = 0u64;
        t184c00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x60029ebc vmov.mi.f32 s15,s14
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x60029ec0u32;
        }
        let value = c.r(824, 4);
        c.w(828, 4, value);
    }
    0x60029ec0
}
pub fn b_60029ec0(c: &mut Machine) -> u32 {
    {
        // 0x60029ec0 cmp r5,r0
        let mut t1a6e00 = 0u64;
        let value = (c.r(32, 4) <= c.r(52, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(52, 4), c.r(32, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(52, 4)).wrapping_sub(c.r(32, 4))) & 0xffffffffu64;
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
        // 0x60029ec2 vcvt.u32.f32 s14,s15
        let mut tade00 = 0u64;
        let mut tadf00 = 0u64;
        let mut tae000 = 0u64;
        tade00 =
            ((f32::from_bits(0u64 as u32) < f32::from_bits(c.r(828, 4) as u32)) as u64) & 0xffu64;
        tadf00 = ((tade00 >> 0) & 0xffu64) & 0xffffffffu64;
        tae000 = ((f32::from_bits(c.r(828, 4) as u32) as i32) as u64) & 0xffffffffu64;
        let value = ((tadf00 >> 0) & 0xffffffffu64).wrapping_mul(((tae000 >> 0) & 0xffffffffu64));
        c.w(824, 4, value);
    }
    {
        // 0x60029ec6 vstr.32 s15,[r4,#0x60]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(96u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029eca vmov r2,s14
        let value = c.r(824, 4);
        c.w(40, 4, value);
    }
    {
        // 0x60029ece vcvt.f32.s32 s14,s14
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
        // 0x60029ed2 str r2,[r4,#0x64]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(100u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x60029ed4 vsub.f32 s15,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029ed8 vstr.32 s15,[r4,#0x68]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(104u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x60029edc bge 0x60029ee2
        let mut t181800 = 0u64;
        t181800 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        if ((t181800 >> 0) & 0xffu64) != 0 {
            return 0x60029ee2u32;
        }
    }
    0x60029ede
}
pub fn b_60029ede(c: &mut Machine) -> u32 {
    {
        // 0x60029ede adds r5,#0x1
        let value = ((c.r(52, 4) as u128 + 1u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(52, 4), 1u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(52, 4)).wrapping_add(1u64);
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
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
        // 0x60029ee0 str r5,[r4,#0x74]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(116u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    0x60029ee2
}
pub fn b_60029ee2(c: &mut Machine) -> u32 {
    {
        // 0x60029ee2 ldr.w r0,[r4,#0x80]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(128u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x60029ee6 cmp r2,r5
        let mut t1a6e00 = 0u64;
        let value = (c.r(52, 4) <= c.r(40, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(40, 4), c.r(52, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(40, 4)).wrapping_sub(c.r(52, 4))) & 0xffffffffu64;
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
        // 0x60029ee8 add.w r12,r0,r3, lsl #0x2
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
        let value = ((c.r(32, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(80, 4, value);
        let value = (signed(c.r(80, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(80, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029eec vstr.32 s0,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x60029ef0 ble 0x60029f9e
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x60029f9eu32;
        }
    }
    0x60029ef2
}
pub fn b_60029ef2(c: &mut Machine) -> u32 {
    {
        // 0x60029ef2 vldr.32 s12,[r4,#0x78]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(120u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    0x60029ef6
}
