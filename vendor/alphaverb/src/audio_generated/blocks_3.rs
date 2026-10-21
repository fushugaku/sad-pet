#![allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut,
    unused_comparisons
)]
use crate::machine::{signed, signed_overflow, Machine};
pub fn b_2884(c: &mut Machine) -> u32 {
    {
        // 0x00002884 vcmpe.f32 s13,s9
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
        // 0x00002888 vmrs apsr,fpscr
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
        // 0x0000288c ble 0x0000294e
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x294eu32;
        }
    }
    0x288e
}
pub fn b_288e(c: &mut Machine) -> u32 {
    {
        // 0x0000288e ldr.w r3,[r5,#0xec]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(236u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00002892 vsub.f32 s13,s13,s9
        let value = (f32::from_bits(c.r(820, 4) as u32) - f32::from_bits(c.r(804, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00002896 vldr.32 s15,[r5,#0xfc]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(252u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x0000289a str.w r3,[r5,#0xf0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(240u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x0000289e vcmpe.f32 s15,#0
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
        // 0x000028a2 ldr.w r3,[r5,#0xe8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(232u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x000028a6 ldr r2,[sp,#0x14]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(20u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x000028a8 str.w r3,[r5,#0xec]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(236u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x000028ac ldr.w r3,[r5,#0xe4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(228u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x000028b0 vmrs apsr,fpscr
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
        // 0x000028b4 vstr.32 s13,[r5,#0xf4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(244u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x000028b8 str.w r3,[r5,#0xe8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(232u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x000028bc ldr.w r3,[r5,#0xbc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(188u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x000028c0 sub.w r2,r3,r2
        let mut t1fe000 = 0u64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        t1fe000 = (c.r(40, 4)) & 0xffffffffu64;
        let value = (((t1fe000 >> 0) & 0xffffffffu64) <= c.r(44, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), ((t1fe000 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_sub(((t1fe000 >> 0) & 0xffffffffu64));
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x000028c4 bne.w 0x00002b16
        let mut t182500 = 0u64;
        t182500 = ((c.r(97, 1) == 0u64) as u64) & 0xffu64;
        if ((t182500 >> 0) & 0xffu64) != 0 {
            return 0x2b16u32;
        }
    }
    0x28c8
}
pub fn b_28c8(c: &mut Machine) -> u32 {
    {
        // 0x000028c8 cmp r2,#0x0
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
        // 0x000028ca it lt
        let mut t185c00 = 0u64;
        t185c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
    }
    {
        // 0x000028cc add.lt r2,r2,r1
        let mut t1d00 = 0u64;
        let mut t2700 = 0u64;
        t1d00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2700 = ((((t1d00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x28ceu32;
        }
        let value = ((c.r(40, 4) as u128 + c.r(36, 4) as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(40, 4), c.r(36, 4), 4, false);
        c.w(103, 1, value);
        let value = (c.r(40, 4)).wrapping_add(c.r(36, 4));
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    0x28ce
}
pub fn b_28ce(c: &mut Machine) -> u32 {
    {
        // 0x000028ce add.w r2,r0,r2, lsl #0x2
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
        // 0x000028d2 ldr r2,[r2,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(40, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x000028d4 str.w r2,[r5,#0xe4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(228u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    0x28d8
}
pub fn b_28d8(c: &mut Machine) -> u32 {
    {
        // 0x000028d8 vldr.32 s7,[r5,#0xd8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(216u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x000028dc add.w r2,r0,r3, lsl #0x2
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
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x000028e0 vldr.32 s12,[r5,#0xdc]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(220u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x000028e4 adds r3,#0x1
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
        // 0x000028e6 vldr.32 s15,[r5,#0xcc]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(204u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x000028ea vsub.f32 s14,s7,s12
        let value = (f32::from_bits(c.r(796, 4) as u32) - f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x000028ee vldr.32 s6,[r5,#0xd4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(212u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x000028f2 vmul.f32 s13,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x000028f6 vldr.32 s11,[r5,#0xe0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(224u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x000028fa vsub.f32 s12,s12,s6
        let value = (f32::from_bits(c.r(816, 4) as u32) - f32::from_bits(c.r(792, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x000028fe cmp r1,r3
        let mut t1a6e00 = 0u64;
        let value = (c.r(44, 4) <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), c.r(44, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(36, 4)).wrapping_sub(c.r(44, 4))) & 0xffffffffu64;
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
        // 0x00002900 vmov.f32 s15,s14
        let value = c.r(824, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00002904 vsub.f32 s11,s11,s7
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(796, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00002908 vfma.f32 s15,s12,s10
        let value = f32::from_bits(c.r(816, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x0000290c vadd.f32 s14,s14,s15
        let value = (f32::from_bits(c.r(824, 4) as u32) + f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00002910 vfma.f32 s14,s11,s10
        let value = f32::from_bits(c.r(812, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00002914 vadd.f32 s15,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002918 vfnms.f32 s15,s13,s14
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
        // 0x0000291c vmov.f32 s14,s7
        let value = c.r(796, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00002920 vmul.f32 s15,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002924 vfma.f32 s15,s12,s10
        let value = f32::from_bits(c.r(816, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002928 vfma.f32 s14,s13,s15
        let value = f32::from_bits(c.r(820, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x0000292c vstr.32 s14,[r2]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(40, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x00002930 ble 0x00002878
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x2878u32;
        }
    }
    0x2932
}
pub fn b_2932(c: &mut Machine) -> u32 {
    {
        // 0x00002932 cmp r1,r7
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
        // 0x00002934 str.w r3,[r5,#0xbc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(188u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00002938 ble 0x00002880
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x2880u32;
        }
    }
    0x293a
}
pub fn b_293a(c: &mut Machine) -> u32 {
    {
        // 0x0000293a vldr.32 s13,[r5,#0xf4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(244u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x0000293e adds r7,#0x1
        let value = ((c.r(60, 4) as u128 + 1u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(60, 4), 1u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(60, 4)).wrapping_add(1u64);
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
        // 0x00002940 vcmpe.f32 s13,s9
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
        // 0x00002944 str.w r7,[r5,#0x104]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(260u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(60, 4));
    }
    {
        // 0x00002948 vmrs apsr,fpscr
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
        // 0x0000294c bgt 0x0000288e
        let mut t181b00 = 0u64;
        let mut t181c00 = 0u64;
        let mut t181e00 = 0u64;
        t181b00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t181c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t181e00 = (((((t181b00 >> 0) & 0xffu64) != 0) & (((t181c00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t181e00 >> 0) & 0xffu64) != 0 {
            return 0x288eu32;
        }
    }
    0x294e
}
pub fn b_294e(c: &mut Machine) -> u32 {
    {
        // 0x0000294e ldrb.w r3,[r5,#0xf8]
        let mut t19c800 = 0u64;
        let mut t1aa600 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(248u64)) & 0xffffffffu64;
        t1aa600 = (c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 1)) & 0xffu64;
        let value = ((t1aa600 >> 0) & 0xffu64);
        c.w(44, 4, value);
    }
    {
        // 0x00002952 cbz r3,0x00002976
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(44, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x2976u32;
        }
    }
    0x2954
}
pub fn b_2954(c: &mut Machine) -> u32 {
    {
        // 0x00002954 vldr.32 s15,[r5,#0xcc]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(204u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00002958 vmov.f32 s12,0x3f000000
        let value = 1056964608u64;
        c.w(816, 4, value);
    }
    {
        // 0x0000295c vmov.f32 s14,0x3f800000
        let value = 1065353216u64;
        c.w(824, 4, value);
    }
    {
        // 0x00002960 vmul.f32 s15,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002964 vmul.f32 s13,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00002968 vcmpe.f32 s13,s9
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
        // 0x0000296c vmrs apsr,fpscr
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
        // 0x00002970 it pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x00002972 vmov.pl.f32 s13,s14
        let mut tf00 = 0u64;
        let mut t2700 = 0u64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2700 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x2976u32;
        }
        let value = c.r(824, 4);
        c.w(820, 4, value);
    }
    0x2976
}
pub fn b_2976(c: &mut Machine) -> u32 {
    {
        // 0x00002976 cmp r1,r7
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
        // 0x00002978 bgt.w 0x000030b2
        let mut t183c00 = 0u64;
        let mut t183d00 = 0u64;
        let mut t183f00 = 0u64;
        t183c00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t183d00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t183f00 = (((((t183c00 >> 0) & 0xffu64) != 0) & (((t183d00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t183f00 >> 0) & 0xffu64) != 0 {
            return 0x30b2u32;
        }
    }
    0x297c
}
pub fn b_297c(c: &mut Machine) -> u32 {
    {
        // 0x0000297c vldr.32 s14,[r5,#0xec]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(236u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00002980 vmov.f32 s10,0x3f000000
        let value = 1056964608u64;
        c.w(808, 4, value);
    }
    {
        // 0x00002984 vldr.32 s11,[r5,#0xe8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(232u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x00002988 vldr.32 s15,[r5,#0xf0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(240u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x0000298c vsub.f32 s12,s14,s11
        let value = (f32::from_bits(c.r(824, 4) as u32) - f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00002990 vldr.32 s9,[r5,#0xe4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(228u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(804, 4, value);
    }
    {
        // 0x00002994 vsub.f32 s11,s11,s15
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00002998 vsub.f32 s9,s9,s14
        let value = (f32::from_bits(c.r(804, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(804, 4, value);
    }
    {
        // 0x0000299c vmov.f32 s15,s12
        let value = c.r(816, 4);
        c.w(828, 4, value);
    }
    {
        // 0x000029a0 vfma.f32 s15,s11,s10
        let value = f32::from_bits(c.r(812, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x000029a4 vadd.f32 s12,s12,s15
        let value = (f32::from_bits(c.r(816, 4) as u32) + f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x000029a8 vfma.f32 s12,s9,s10
        let value = f32::from_bits(c.r(804, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(816, 4) as u32),
            )
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x000029ac vadd.f32 s15,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x000029b0 vfnms.f32 s15,s12,s13
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
        // 0x000029b4 vmul.f32 s15,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x000029b8 vfma.f32 s15,s11,s10
        let value = f32::from_bits(c.r(812, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x000029bc vfma.f32 s14,s13,s15
        let value = f32::from_bits(c.r(820, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x000029c0 vadd.f32 s15,s14,s23
        let value = (f32::from_bits(c.r(824, 4) as u32) + f32::from_bits(c.r(860, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    0x29c4
}
pub fn b_29c4(c: &mut Machine) -> u32 {
    {
        // 0x000029c4 vldr.32 s13,[r5,#0x154]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(340u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x000029c8 vmov.f32 s9,0x3f800000
        let value = 1065353216u64;
        c.w(804, 4, value);
    }
    {
        // 0x000029cc vldr.32 s14,[r5,#0x128]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(296u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x000029d0 vmov.f32 s10,0x3f000000
        let value = 1056964608u64;
        c.w(808, 4, value);
    }
    {
        // 0x000029d4 ldr.w r0,[r5,#0x13c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(316u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x000029d8 mov.w lr,#0x0
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
        // 0x000029dc vadd.f32 s13,s13,s14
        let value = (f32::from_bits(c.r(820, 4) as u32) + f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x000029e0 ldr.w r2,[r5,#0x138]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(312u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x000029e4 ldr.w r3,[r5,#0x134]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(308u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x000029e8 str.w r0,[r5,#0x140]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(320u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x000029ec ldr.w r1,[r5,#0x110]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(272u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x000029f0 vldr.32 s8,[pc,#-0x2c4]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((10736u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(708u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x000029f4 ldr.w r12,[r5,#0x118]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(280u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(80, 4, value);
    }
    {
        // 0x000029f8 ldr.w r4,[r5,#0x120]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(288u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x000029fc ldr.w r0,[r5,#0x164]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(356u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x00002a00 vstr.32 s15,[r5,#0xd0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(208u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x00002a04 str.w r2,[r5,#0x13c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(316u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x00002a08 str.w r3,[r5,#0x138]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(312u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00002a0c vstr.32 s25,[r5,#0x134]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(308u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(868, 4));
    }
    {
        // 0x00002a10 vstr.32 s13,[r5,#0x154]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(340u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x00002a14 b 0x00002a92
        return 0x2a92u32;
    }
    0x2a16
}
pub fn b_2a16(c: &mut Machine) -> u32 {
    {
        // 0x00002a16 cmp r2,#0x0
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
        // 0x00002a18 it lt
        let mut t185c00 = 0u64;
        t185c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
    }
    {
        // 0x00002a1a add.lt r2,r2,r1
        let mut t1d00 = 0u64;
        let mut t2700 = 0u64;
        t1d00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2700 = ((((t1d00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x2a1cu32;
        }
        let value = ((c.r(40, 4) as u128 + c.r(36, 4) as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(40, 4), c.r(36, 4), 4, false);
        c.w(103, 1, value);
        let value = (c.r(40, 4)).wrapping_add(c.r(36, 4));
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    0x2a1c
}
pub fn b_2a1c(c: &mut Machine) -> u32 {
    {
        // 0x00002a1c add.w r2,r4,r2, lsl #0x2
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
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00002a20 ldr r2,[r2,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(40, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00002a22 str.w r2,[r5,#0x144]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(324u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    0x2a26
}
pub fn b_2a26(c: &mut Machine) -> u32 {
    {
        // 0x00002a26 vldr.32 s7,[r5,#0x138]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(312u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x00002a2a cmp r1,r0
        let mut t1a6e00 = 0u64;
        let value = (c.r(32, 4) <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), c.r(32, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(36, 4)).wrapping_sub(c.r(32, 4))) & 0xffffffffu64;
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
        // 0x00002a2c vldr.32 s12,[r5,#0x13c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(316u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x00002a30 add.w r2,r4,r3, lsl #0x2
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
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00002a34 vldr.32 s15,[r5,#0x12c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(300u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00002a38 vsub.f32 s14,s7,s12
        let value = (f32::from_bits(c.r(796, 4) as u32) - f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00002a3c vldr.32 s6,[r5,#0x134]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(308u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x00002a40 vmul.f32 s13,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00002a44 vldr.32 s11,[r5,#0x140]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(320u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x00002a48 vsub.f32 s12,s12,s6
        let value = (f32::from_bits(c.r(816, 4) as u32) - f32::from_bits(c.r(792, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00002a4c vmov.f32 s15,s14
        let value = c.r(824, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00002a50 vsub.f32 s11,s11,s7
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(796, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00002a54 vfma.f32 s15,s12,s10
        let value = f32::from_bits(c.r(816, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002a58 vadd.f32 s14,s14,s15
        let value = (f32::from_bits(c.r(824, 4) as u32) + f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00002a5c vfma.f32 s14,s11,s10
        let value = f32::from_bits(c.r(812, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00002a60 vadd.f32 s15,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002a64 vfnms.f32 s15,s13,s14
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
        // 0x00002a68 vmov.f32 s14,s7
        let value = c.r(796, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00002a6c vmul.f32 s15,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002a70 vfma.f32 s15,s12,s10
        let value = f32::from_bits(c.r(816, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002a74 vfma.f32 s14,s13,s15
        let value = f32::from_bits(c.r(820, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00002a78 ble 0x00002b58
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x2b58u32;
        }
    }
    0x2a7a
}
pub fn b_2a7a(c: &mut Machine) -> u32 {
    {
        // 0x00002a7a adds r3,#0x1
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
        // 0x00002a7c adds r0,#0x1
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
        // 0x00002a7e cmp r1,r3
        let mut t1a6e00 = 0u64;
        let value = (c.r(44, 4) <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), c.r(44, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(36, 4)).wrapping_sub(c.r(44, 4))) & 0xffffffffu64;
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
        // 0x00002a80 str.w r0,[r5,#0x164]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(356u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x00002a84 vstr.32 s14,[r2]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(40, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x00002a88 str.w r3,[r5,#0x11c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(284u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00002a8c ble 0x00002b72
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x2b72u32;
        }
    }
    0x2a8e
}
pub fn b_2a8e(c: &mut Machine) -> u32 {
    {
        // 0x00002a8e vldr.32 s13,[r5,#0x154]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(340u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    0x2a92
}
pub fn b_2a92(c: &mut Machine) -> u32 {
    {
        // 0x00002a92 vcmpe.f32 s13,s9
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
        // 0x00002a96 vmrs apsr,fpscr
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
        // 0x00002a9a ble 0x00002b86
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x2b86u32;
        }
    }
    0x2a9c
}
pub fn b_2a9c(c: &mut Machine) -> u32 {
    {
        // 0x00002a9c ldr.w r3,[r5,#0x14c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(332u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00002aa0 vsub.f32 s13,s13,s9
        let value = (f32::from_bits(c.r(820, 4) as u32) - f32::from_bits(c.r(804, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00002aa4 vldr.32 s15,[r5,#0x15c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(348u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00002aa8 str.w r3,[r5,#0x150]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(336u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00002aac vcmpe.f32 s15,#0
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
        // 0x00002ab0 ldr.w r3,[r5,#0x148]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(328u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00002ab4 ldr r2,[sp,#0x18]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(24u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00002ab6 str.w r3,[r5,#0x14c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(332u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00002aba ldr.w r3,[r5,#0x144]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(324u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00002abe vmrs apsr,fpscr
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
        // 0x00002ac2 vstr.32 s13,[r5,#0x154]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(340u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x00002ac6 str.w r3,[r5,#0x148]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(328u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00002aca ldr.w r3,[r5,#0x11c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(284u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00002ace sub.w r2,r3,r2
        let mut t1fe000 = 0u64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        t1fe000 = (c.r(40, 4)) & 0xffffffffu64;
        let value = (((t1fe000 >> 0) & 0xffffffffu64) <= c.r(44, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), ((t1fe000 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_sub(((t1fe000 >> 0) & 0xffffffffu64));
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00002ad2 beq 0x00002a16
        let mut t180200 = 0u64;
        t180200 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        if ((t180200 >> 0) & 0xffu64) != 0 {
            return 0x2a16u32;
        }
    }
    0x2ad4
}
pub fn b_2ad4(c: &mut Machine) -> u32 {
    {
        // 0x00002ad4 cmp r2,#0x0
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
        // 0x00002ad6 vsub.f32 s11,s15,s8
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(800, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00002ada it lt
        let mut t185c00 = 0u64;
        t185c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
    }
    {
        // 0x00002adc add.lt r2,r2,r1
        let mut t1d00 = 0u64;
        let mut t2700 = 0u64;
        t1d00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2700 = ((((t1d00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x2adeu32;
        }
        let value = ((c.r(40, 4) as u128 + c.r(36, 4) as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(40, 4), c.r(36, 4), 4, false);
        c.w(103, 1, value);
        let value = (c.r(40, 4)).wrapping_add(c.r(36, 4));
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    0x2ade
}
pub fn b_2ade(c: &mut Machine) -> u32 {
    {
        // 0x00002ade subs.w r7,r3,r12
        let mut t1fe000 = 0u64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        t1fe000 = (c.r(80, 4)) & 0xffffffffu64;
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
        // 0x00002ae2 vcmpe.f32 s11,#0
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
        // 0x00002ae6 it mi
        let mut t184c00 = 0u64;
        t184c00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x00002ae8 add.mi r7,r7,r1
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x2aeau32;
        }
        let value = ((c.r(60, 4) as u128 + c.r(36, 4) as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(60, 4), c.r(36, 4), 4, false);
        c.w(103, 1, value);
        let value = (c.r(60, 4)).wrapping_add(c.r(36, 4));
        c.w(60, 4, value);
        let value = (signed(c.r(60, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(60, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    0x2aea
}
pub fn b_2aea(c: &mut Machine) -> u32 {
    {
        // 0x00002aea add.w r2,r4,r2, lsl #0x2
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
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00002aee add.w r7,r4,r7, lsl #0x2
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
        // 0x00002af2 vldr.32 s14,[r2]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(40, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00002af6 vmrs apsr,fpscr
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
        // 0x00002afa vldr.32 s12,[r7]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(60, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x00002afe vsub.f32 s12,s12,s14
        let value = (f32::from_bits(c.r(816, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00002b02 ite pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x00002b04 vstr.pl.32 s11,[r5,#0x15c]
        let mut tf00 = 0u64;
        let mut t2600 = 0u64;
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(348u64)) & 0xffffffffu64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2600 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2600 >> 0) & 0xffu64) != 0 {
            return 0x2b08u32;
        }
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(812, 4));
    }
    0x2b08
}
pub fn b_2b08(c: &mut Machine) -> u32 {
    {
        // 0x00002b08 str.w.mi lr,[r5,#0x15c]
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        let mut t19c800 = 0u64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x2b0cu32;
        }
        t19c800 = ((c.r(52, 4)).wrapping_add(348u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(88, 4));
    }
    0x2b0c
}
pub fn b_2b0c(c: &mut Machine) -> u32 {
    {
        // 0x00002b0c vfma.f32 s14,s15,s12
        let value = f32::from_bits(c.r(828, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(816, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00002b10 vstr.32 s14,[r5,#0x144]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(324u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x00002b14 b 0x00002a26
        return 0x2a26u32;
    }
    0x2b16
}
pub fn b_2b16(c: &mut Machine) -> u32 {
    {
        // 0x00002b16 cmp r2,#0x0
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
        // 0x00002b18 vsub.f32 s11,s15,s8
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(800, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00002b1c it lt
        let mut t185c00 = 0u64;
        t185c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
    }
    {
        // 0x00002b1e add.lt r2,r2,r1
        let mut t1d00 = 0u64;
        let mut t2700 = 0u64;
        t1d00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2700 = ((((t1d00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x2b20u32;
        }
        let value = ((c.r(40, 4) as u128 + c.r(36, 4) as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(40, 4), c.r(36, 4), 4, false);
        c.w(103, 1, value);
        let value = (c.r(40, 4)).wrapping_add(c.r(36, 4));
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    0x2b20
}
pub fn b_2b20(c: &mut Machine) -> u32 {
    {
        // 0x00002b20 subs.w r4,r3,r12
        let mut t1fe000 = 0u64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        t1fe000 = (c.r(80, 4)) & 0xffffffffu64;
        let value = (((t1fe000 >> 0) & 0xffffffffu64) <= c.r(44, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), ((t1fe000 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_sub(((t1fe000 >> 0) & 0xffffffffu64));
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
        // 0x00002b24 vcmpe.f32 s11,#0
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
        // 0x00002b28 it mi
        let mut t184c00 = 0u64;
        t184c00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x00002b2a add.mi r4,r4,r1
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x2b2cu32;
        }
        let value = ((c.r(48, 4) as u128 + c.r(36, 4) as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), c.r(36, 4), 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(c.r(36, 4));
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    0x2b2c
}
pub fn b_2b2c(c: &mut Machine) -> u32 {
    {
        // 0x00002b2c add.w r2,r0,r2, lsl #0x2
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
        // 0x00002b30 add.w r4,r0,r4, lsl #0x2
        let mut t189200 = 0u64;
        let mut t189300 = 0u64;
        let mut t189500 = 0u64;
        let mut t189800 = 0u64;
        let mut t1a0b00 = 0u64;
        t189200 = ((32u64).wrapping_sub(2u64)) & 0xffffffffu64;
        t189300 = (if ((t189200 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(48, 4)) >> (((t189200 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t189500 = (((t189300 >> 0) & 0xffffffffu64) & 1u64) & 0xffffffffu64;
        let value = ((t189500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        t189800 = (if 2u64 >= 32 {
            0
        } else {
            (c.r(48, 4)) << (2u64 as u32)
        }) & 0xffffffffu64;
        t1a0b00 = ((t189800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        let value = ((c.r(32, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00002b34 vldr.32 s14,[r2]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(40, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00002b38 vmrs apsr,fpscr
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
        // 0x00002b3c vldr.32 s12,[r4]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(48, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x00002b40 vsub.f32 s12,s12,s14
        let value = (f32::from_bits(c.r(816, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00002b44 ite pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x00002b46 vstr.pl.32 s11,[r5,#0xfc]
        let mut tf00 = 0u64;
        let mut t2600 = 0u64;
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(252u64)) & 0xffffffffu64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2600 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2600 >> 0) & 0xffu64) != 0 {
            return 0x2b4au32;
        }
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(812, 4));
    }
    0x2b4a
}
pub fn b_2b4a(c: &mut Machine) -> u32 {
    {
        // 0x00002b4a str.w.mi r8,[r5,#0xfc]
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        let mut t19c800 = 0u64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x2b4eu32;
        }
        t19c800 = ((c.r(52, 4)).wrapping_add(252u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(64, 4));
    }
    0x2b4e
}
pub fn b_2b4e(c: &mut Machine) -> u32 {
    {
        // 0x00002b4e vfma.f32 s14,s15,s12
        let value = f32::from_bits(c.r(828, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(816, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00002b52 vstr.32 s14,[r5,#0xe4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(228u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x00002b56 b 0x000028d8
        return 0x28d8u32;
    }
    0x2b58
}
pub fn b_2b58(c: &mut Machine) -> u32 {
    {
        // 0x00002b58 vldr.32 s15,[r2]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(40, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00002b5c adds r3,#0x1
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
        // 0x00002b5e vsub.f32 s15,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002b62 cmp r1,r3
        let mut t1a6e00 = 0u64;
        let value = (c.r(44, 4) <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), c.r(44, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(36, 4)).wrapping_sub(c.r(44, 4))) & 0xffffffffu64;
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
        // 0x00002b64 vfma.f32 s14,s24,s15
        let value = f32::from_bits(c.r(864, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00002b68 vstr.32 s14,[r2]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(40, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x00002b6c str.w r3,[r5,#0x11c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(284u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00002b70 bgt 0x00002a8e
        let mut t181b00 = 0u64;
        let mut t181c00 = 0u64;
        let mut t181e00 = 0u64;
        t181b00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t181c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t181e00 = (((((t181b00 >> 0) & 0xffu64) != 0) & (((t181c00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t181e00 >> 0) & 0xffu64) != 0 {
            return 0x2a8eu32;
        }
    }
    0x2b72
}
pub fn b_2b72(c: &mut Machine) -> u32 {
    {
        // 0x00002b72 vldr.32 s13,[r5,#0x154]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(340u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00002b76 movs r3,#0x0
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
        // 0x00002b78 vcmpe.f32 s13,s9
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
        // 0x00002b7c str.w r3,[r5,#0x11c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(284u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00002b80 vmrs apsr,fpscr
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
        // 0x00002b84 bgt 0x00002a9c
        let mut t181b00 = 0u64;
        let mut t181c00 = 0u64;
        let mut t181e00 = 0u64;
        t181b00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t181c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t181e00 = (((((t181b00 >> 0) & 0xffu64) != 0) & (((t181c00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t181e00 >> 0) & 0xffu64) != 0 {
            return 0x2a9cu32;
        }
    }
    0x2b86
}
pub fn b_2b86(c: &mut Machine) -> u32 {
    {
        // 0x00002b86 ldrb.w r3,[r5,#0x158]
        let mut t19c800 = 0u64;
        let mut t1aa600 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(344u64)) & 0xffffffffu64;
        t1aa600 = (c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 1)) & 0xffu64;
        let value = ((t1aa600 >> 0) & 0xffu64);
        c.w(44, 4, value);
    }
    {
        // 0x00002b8a cbz r3,0x00002bae
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(44, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x2baeu32;
        }
    }
    0x2b8c
}
pub fn b_2b8c(c: &mut Machine) -> u32 {
    {
        // 0x00002b8c vldr.32 s15,[r5,#0x12c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(300u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00002b90 vmov.f32 s12,0x3f000000
        let value = 1056964608u64;
        c.w(816, 4, value);
    }
    {
        // 0x00002b94 vmov.f32 s14,0x3f800000
        let value = 1065353216u64;
        c.w(824, 4, value);
    }
    {
        // 0x00002b98 vmul.f32 s15,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002b9c vmul.f32 s13,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00002ba0 vcmpe.f32 s13,s9
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
        // 0x00002ba4 vmrs apsr,fpscr
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
        // 0x00002ba8 it pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x00002baa vmov.pl.f32 s13,s14
        let mut tf00 = 0u64;
        let mut t2700 = 0u64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2700 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x2baeu32;
        }
        let value = c.r(824, 4);
        c.w(820, 4, value);
    }
    0x2bae
}
pub fn b_2bae(c: &mut Machine) -> u32 {
    {
        // 0x00002bae cmp r1,r0
        let mut t1a6e00 = 0u64;
        let value = (c.r(32, 4) <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), c.r(32, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(36, 4)).wrapping_sub(c.r(32, 4))) & 0xffffffffu64;
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
        // 0x00002bb0 bgt.w 0x000030b8
        let mut t183c00 = 0u64;
        let mut t183d00 = 0u64;
        let mut t183f00 = 0u64;
        t183c00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t183d00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t183f00 = (((((t183c00 >> 0) & 0xffu64) != 0) & (((t183d00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t183f00 >> 0) & 0xffu64) != 0 {
            return 0x30b8u32;
        }
    }
    0x2bb4
}
pub fn b_2bb4(c: &mut Machine) -> u32 {
    {
        // 0x00002bb4 vldr.32 s14,[r5,#0x14c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(332u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00002bb8 vmov.f32 s10,0x3f000000
        let value = 1056964608u64;
        c.w(808, 4, value);
    }
    {
        // 0x00002bbc vldr.32 s11,[r5,#0x148]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(328u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x00002bc0 vldr.32 s15,[r5,#0x150]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(336u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00002bc4 vsub.f32 s12,s14,s11
        let value = (f32::from_bits(c.r(824, 4) as u32) - f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00002bc8 vldr.32 s9,[r5,#0x144]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(324u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(804, 4, value);
    }
    {
        // 0x00002bcc vsub.f32 s11,s11,s15
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00002bd0 vsub.f32 s9,s9,s14
        let value = (f32::from_bits(c.r(804, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(804, 4, value);
    }
    {
        // 0x00002bd4 vmov.f32 s15,s12
        let value = c.r(816, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00002bd8 vfma.f32 s15,s11,s10
        let value = f32::from_bits(c.r(812, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002bdc vadd.f32 s12,s12,s15
        let value = (f32::from_bits(c.r(816, 4) as u32) + f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00002be0 vfma.f32 s12,s9,s10
        let value = f32::from_bits(c.r(804, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(816, 4) as u32),
            )
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00002be4 vadd.f32 s15,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002be8 vfnms.f32 s15,s12,s13
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
        // 0x00002bec vmul.f32 s15,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002bf0 vfma.f32 s15,s11,s10
        let value = f32::from_bits(c.r(812, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002bf4 vfma.f32 s14,s13,s15
        let value = f32::from_bits(c.r(820, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    0x2bf8
}
pub fn b_2bf8(c: &mut Machine) -> u32 {
    {
        // 0x00002bf8 vldr.32 s13,[r5,#0x1b4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(436u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00002bfc vmov.f32 s9,0x3f800000
        let value = 1065353216u64;
        c.w(804, 4, value);
    }
    {
        // 0x00002c00 vldr.32 s15,[r5,#0x188]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(392u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00002c04 vmov.f32 s10,0x3f000000
        let value = 1056964608u64;
        c.w(808, 4, value);
    }
    {
        // 0x00002c08 ldr.w r0,[r5,#0x19c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(412u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x00002c0c mov.w lr,#0x0
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
        // 0x00002c10 vadd.f32 s13,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) + f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00002c14 ldr.w r2,[r5,#0x198]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(408u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00002c18 ldr.w r3,[r5,#0x194]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(404u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00002c1c str.w r0,[r5,#0x1a0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(416u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x00002c20 ldr.w r1,[r5,#0x170]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(368u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x00002c24 ldr.w r7,[r5,#0x178]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(376u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(60, 4, value);
    }
    {
        // 0x00002c28 vldr.32 s8,[pc,#0x228]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((11304u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(552u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x00002c2c ldr.w r4,[r5,#0x180]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(384u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x00002c30 ldr.w r0,[r5,#0x1c4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(452u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x00002c34 vstr.32 s14,[r5,#0x32c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(812u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x00002c38 str.w r2,[r5,#0x19c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(412u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x00002c3c str.w r3,[r5,#0x198]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(408u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00002c40 vstr.32 s28,[r5,#0x194]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(404u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(880, 4));
    }
    {
        // 0x00002c44 vstr.32 s13,[r5,#0x1b4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(436u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x00002c48 b 0x00002c62
        return 0x2c62u32;
    }
    0x2c4a
}
pub fn b_2c4a(c: &mut Machine) -> u32 {
    {
        // 0x00002c4a adds r3,#0x1
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
        // 0x00002c4c adds r0,#0x1
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
        // 0x00002c4e cmp r1,r3
        let mut t1a6e00 = 0u64;
        let value = (c.r(44, 4) <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), c.r(44, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(36, 4)).wrapping_sub(c.r(44, 4))) & 0xffffffffu64;
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
        // 0x00002c50 str.w r0,[r5,#0x1c4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(452u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x00002c54 vstr.32 s14,[r2]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(40, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x00002c58 str.w r3,[r5,#0x17c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(380u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00002c5c ble 0x00002d24
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x2d24u32;
        }
    }
    0x2c5e
}
pub fn b_2c5e(c: &mut Machine) -> u32 {
    {
        // 0x00002c5e vldr.32 s13,[r5,#0x1b4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(436u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    0x2c62
}
pub fn b_2c62(c: &mut Machine) -> u32 {
    {
        // 0x00002c62 vcmpe.f32 s13,s9
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
        // 0x00002c66 vmrs apsr,fpscr
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
        // 0x00002c6a ble 0x00002d38
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x2d38u32;
        }
    }
    0x2c6c
}
pub fn b_2c6c(c: &mut Machine) -> u32 {
    {
        // 0x00002c6c ldr.w r3,[r5,#0x1ac]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(428u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00002c70 vsub.f32 s13,s13,s9
        let value = (f32::from_bits(c.r(820, 4) as u32) - f32::from_bits(c.r(804, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00002c74 vldr.32 s15,[r5,#0x1bc]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(444u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00002c78 str.w r3,[r5,#0x1b0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(432u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00002c7c vcmpe.f32 s15,#0
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
        // 0x00002c80 ldr.w r3,[r5,#0x1a8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(424u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00002c84 ldr r2,[sp,#0x1c]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00002c86 str.w r3,[r5,#0x1ac]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(428u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00002c8a ldr.w r3,[r5,#0x1a4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(420u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00002c8e vmrs apsr,fpscr
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
        // 0x00002c92 vstr.32 s13,[r5,#0x1b4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(436u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x00002c96 str.w r3,[r5,#0x1a8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(424u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00002c9a ldr.w r3,[r5,#0x17c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(380u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x00002c9e sub.w r2,r3,r2
        let mut t1fe000 = 0u64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        t1fe000 = (c.r(40, 4)) & 0xffffffffu64;
        let value = (((t1fe000 >> 0) & 0xffffffffu64) <= c.r(44, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), ((t1fe000 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_sub(((t1fe000 >> 0) & 0xffffffffu64));
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00002ca2 bne.w 0x00002e12
        let mut t182500 = 0u64;
        t182500 = ((c.r(97, 1) == 0u64) as u64) & 0xffu64;
        if ((t182500 >> 0) & 0xffu64) != 0 {
            return 0x2e12u32;
        }
    }
    0x2ca6
}
pub fn b_2ca6(c: &mut Machine) -> u32 {
    {
        // 0x00002ca6 cmp r2,#0x0
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
        // 0x00002ca8 it lt
        let mut t185c00 = 0u64;
        t185c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
    }
    {
        // 0x00002caa add.lt r2,r2,r1
        let mut t1d00 = 0u64;
        let mut t2700 = 0u64;
        t1d00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2700 = ((((t1d00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x2cacu32;
        }
        let value = ((c.r(40, 4) as u128 + c.r(36, 4) as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(40, 4), c.r(36, 4), 4, false);
        c.w(103, 1, value);
        let value = (c.r(40, 4)).wrapping_add(c.r(36, 4));
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    0x2cac
}
pub fn b_2cac(c: &mut Machine) -> u32 {
    {
        // 0x00002cac add.w r2,r4,r2, lsl #0x2
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
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00002cb0 ldr r2,[r2,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(40, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x00002cb2 str.w r2,[r5,#0x1a4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(420u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    0x2cb6
}
pub fn b_2cb6(c: &mut Machine) -> u32 {
    {
        // 0x00002cb6 vldr.32 s7,[r5,#0x198]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(408u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x00002cba cmp r1,r0
        let mut t1a6e00 = 0u64;
        let value = (c.r(32, 4) <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), c.r(32, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(36, 4)).wrapping_sub(c.r(32, 4))) & 0xffffffffu64;
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
        // 0x00002cbc vldr.32 s12,[r5,#0x19c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(412u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x00002cc0 add.w r2,r4,r3, lsl #0x2
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
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00002cc4 vldr.32 s15,[r5,#0x18c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(396u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00002cc8 vsub.f32 s14,s7,s12
        let value = (f32::from_bits(c.r(796, 4) as u32) - f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00002ccc vldr.32 s6,[r5,#0x194]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(404u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x00002cd0 vmul.f32 s13,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00002cd4 vldr.32 s11,[r5,#0x1a0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(416u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x00002cd8 vsub.f32 s12,s12,s6
        let value = (f32::from_bits(c.r(816, 4) as u32) - f32::from_bits(c.r(792, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00002cdc vmov.f32 s15,s14
        let value = c.r(824, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00002ce0 vsub.f32 s11,s11,s7
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(796, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00002ce4 vfma.f32 s15,s12,s10
        let value = f32::from_bits(c.r(816, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002ce8 vadd.f32 s14,s14,s15
        let value = (f32::from_bits(c.r(824, 4) as u32) + f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00002cec vfma.f32 s14,s11,s10
        let value = f32::from_bits(c.r(812, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00002cf0 vadd.f32 s15,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002cf4 vfnms.f32 s15,s13,s14
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
        // 0x00002cf8 vmov.f32 s14,s7
        let value = c.r(796, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00002cfc vmul.f32 s15,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002d00 vfma.f32 s15,s12,s10
        let value = f32::from_bits(c.r(816, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002d04 vfma.f32 s14,s13,s15
        let value = f32::from_bits(c.r(820, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00002d08 bgt 0x00002c4a
        let mut t181b00 = 0u64;
        let mut t181c00 = 0u64;
        let mut t181e00 = 0u64;
        t181b00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t181c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t181e00 = (((((t181b00 >> 0) & 0xffu64) != 0) & (((t181c00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t181e00 >> 0) & 0xffu64) != 0 {
            return 0x2c4au32;
        }
    }
    0x2d0a
}
pub fn b_2d0a(c: &mut Machine) -> u32 {
    {
        // 0x00002d0a vldr.32 s15,[r2]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(40, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00002d0e adds r3,#0x1
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
        // 0x00002d10 vsub.f32 s15,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002d14 cmp r1,r3
        let mut t1a6e00 = 0u64;
        let value = (c.r(44, 4) <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), c.r(44, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(36, 4)).wrapping_sub(c.r(44, 4))) & 0xffffffffu64;
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
        // 0x00002d16 vfma.f32 s14,s24,s15
        let value = f32::from_bits(c.r(864, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00002d1a vstr.32 s14,[r2]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(40, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x00002d1e str.w r3,[r5,#0x17c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(380u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00002d22 bgt 0x00002c5e
        let mut t181b00 = 0u64;
        let mut t181c00 = 0u64;
        let mut t181e00 = 0u64;
        t181b00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t181c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t181e00 = (((((t181b00 >> 0) & 0xffu64) != 0) & (((t181c00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t181e00 >> 0) & 0xffu64) != 0 {
            return 0x2c5eu32;
        }
    }
    0x2d24
}
pub fn b_2d24(c: &mut Machine) -> u32 {
    {
        // 0x00002d24 vldr.32 s13,[r5,#0x1b4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(436u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00002d28 movs r3,#0x0
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
        // 0x00002d2a vcmpe.f32 s13,s9
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
        // 0x00002d2e str.w r3,[r5,#0x17c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(380u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x00002d32 vmrs apsr,fpscr
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
        // 0x00002d36 bgt 0x00002c6c
        let mut t181b00 = 0u64;
        let mut t181c00 = 0u64;
        let mut t181e00 = 0u64;
        t181b00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t181c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t181e00 = (((((t181b00 >> 0) & 0xffu64) != 0) & (((t181c00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t181e00 >> 0) & 0xffu64) != 0 {
            return 0x2c6cu32;
        }
    }
    0x2d38
}
pub fn b_2d38(c: &mut Machine) -> u32 {
    {
        // 0x00002d38 ldrb.w r3,[r5,#0x1b8]
        let mut t19c800 = 0u64;
        let mut t1aa600 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(440u64)) & 0xffffffffu64;
        t1aa600 = (c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 1)) & 0xffu64;
        let value = ((t1aa600 >> 0) & 0xffu64);
        c.w(44, 4, value);
    }
    {
        // 0x00002d3c cbz r3,0x00002d60
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(44, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x2d60u32;
        }
    }
    0x2d3e
}
pub fn b_2d3e(c: &mut Machine) -> u32 {
    {
        // 0x00002d3e vldr.32 s15,[r5,#0x18c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(396u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00002d42 vmov.f32 s12,0x3f000000
        let value = 1056964608u64;
        c.w(816, 4, value);
    }
    {
        // 0x00002d46 vmov.f32 s14,0x3f800000
        let value = 1065353216u64;
        c.w(824, 4, value);
    }
    {
        // 0x00002d4a vmul.f32 s15,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002d4e vmul.f32 s13,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00002d52 vcmpe.f32 s13,s9
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
        // 0x00002d56 vmrs apsr,fpscr
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
        // 0x00002d5a it pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x00002d5c vmov.pl.f32 s13,s14
        let mut tf00 = 0u64;
        let mut t2700 = 0u64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2700 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x2d60u32;
        }
        let value = c.r(824, 4);
        c.w(820, 4, value);
    }
    0x2d60
}
pub fn b_2d60(c: &mut Machine) -> u32 {
    {
        // 0x00002d60 cmp r1,r0
        let mut t1a6e00 = 0u64;
        let value = (c.r(32, 4) <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), c.r(32, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(36, 4)).wrapping_sub(c.r(32, 4))) & 0xffffffffu64;
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
        // 0x00002d62 bgt.w 0x000030c6
        let mut t183c00 = 0u64;
        let mut t183d00 = 0u64;
        let mut t183f00 = 0u64;
        t183c00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t183d00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t183f00 = (((((t183c00 >> 0) & 0xffu64) != 0) & (((t183d00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t183f00 >> 0) & 0xffu64) != 0 {
            return 0x30c6u32;
        }
    }
    0x2d66
}
pub fn b_2d66(c: &mut Machine) -> u32 {
    {
        // 0x00002d66 vldr.32 s8,[r5,#0x1ac]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(428u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x00002d6a vmov.f32 s11,0x3f000000
        let value = 1056964608u64;
        c.w(812, 4, value);
    }
    {
        // 0x00002d6e vldr.32 s12,[r5,#0x1a8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(424u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x00002d72 vldr.32 s15,[r5,#0x1b0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(432u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00002d76 vsub.f32 s14,s8,s12
        let value = (f32::from_bits(c.r(800, 4) as u32) - f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00002d7a vldr.32 s10,[r5,#0x1a4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(420u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x00002d7e vsub.f32 s12,s12,s15
        let value = (f32::from_bits(c.r(816, 4) as u32) - f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00002d82 vsub.f32 s10,s10,s8
        let value = (f32::from_bits(c.r(808, 4) as u32) - f32::from_bits(c.r(800, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x00002d86 vmov.f32 s15,s14
        let value = c.r(824, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00002d8a vfma.f32 s15,s12,s11
        let value = f32::from_bits(c.r(816, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(812, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002d8e vadd.f32 s14,s14,s15
        let value = (f32::from_bits(c.r(824, 4) as u32) + f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00002d92 vfma.f32 s14,s10,s11
        let value = f32::from_bits(c.r(808, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(812, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00002d96 vadd.f32 s15,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002d9a vfnms.f32 s15,s14,s13
        let mut t49a00 = 0u64;
        let mut t49b00 = 0u64;
        t49a00 = ((-f32::from_bits(c.r(828, 4) as u32)).to_bits() as u64) & 0xffffffffu64;
        t49b00 = ((f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64)
            & 0xffffffffu64;
        let value = (f32::from_bits(((t49a00 >> 0) & 0xffffffffu64) as u32)
            + f32::from_bits(((t49b00 >> 0) & 0xffffffffu64) as u32))
        .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002d9e vmul.f32 s15,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002da2 vfma.f32 s15,s12,s11
        let value = f32::from_bits(c.r(816, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(812, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002da6 vfma.f32 s8,s13,s15
        let value = f32::from_bits(c.r(820, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(800, 4) as u32),
            )
            .to_bits() as u64;
        c.w(800, 4, value);
    }
    0x2daa
}
pub fn b_2daa(c: &mut Machine) -> u32 {
    {
        // 0x00002daa vldr.32 s5,[r5,#0x32c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(812u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(788, 4, value);
    }
    {
        // 0x00002dae vldr.32 s15,[r5,#0x228]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(552u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00002db2 vldr.32 s9,[r5,#0x248]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(584u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(804, 4, value);
    }
    {
        // 0x00002db6 vldr.32 s10,[r5,#0x24c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(588u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x00002dba vfma.f32 s9,s5,s15
        let value = f32::from_bits(c.r(788, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(804, 4) as u32),
            )
            .to_bits() as u64;
        c.w(804, 4, value);
    }
    {
        // 0x00002dbe vldr.32 s12,[r5,#0x240]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(576u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x00002dc2 vfma.f32 s10,s15,s8
        let value = f32::from_bits(c.r(828, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(800, 4) as u32),
                f32::from_bits(c.r(808, 4) as u32),
            )
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x00002dc6 vldr.32 s11,[r5,#0x250]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(592u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x00002dca vldr.32 s15,[r5,#0x224]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(548u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00002dce vldr.32 s6,[r5,#0x254]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(596u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x00002dd2 vldr.32 s13,[r5,#0x22c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(556u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00002dd6 vldr.32 s14,[r5,#0x23c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(572u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00002dda vfma.f32 s11,s9,s12
        let value = f32::from_bits(c.r(804, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(816, 4) as u32),
                f32::from_bits(c.r(812, 4) as u32),
            )
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00002dde vldr.32 s7,[r5,#0x244]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(580u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x00002de2 vfma.f32 s6,s10,s12
        let value = f32::from_bits(c.r(808, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(816, 4) as u32),
                f32::from_bits(c.r(792, 4) as u32),
            )
            .to_bits() as u64;
        c.w(792, 4, value);
    }
    {
        // 0x00002de6 vnmul.f32 s12,s15,s9
        let mut t71900 = 0u64;
        t71900 = ((f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(804, 4) as u32))
            .to_bits() as u64)
            & 0xffffffffu64;
        let value = (-f32::from_bits(((t71900 >> 0) & 0xffffffffu64) as u32)).to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00002dea vnmul.f32 s15,s10,s15
        let mut t71900 = 0u64;
        t71900 = ((f32::from_bits(c.r(808, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64)
            & 0xffffffffu64;
        let value = (-f32::from_bits(((t71900 >> 0) & 0xffffffffu64) as u32)).to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002dee vfma.f32 s12,s5,s13
        let value = f32::from_bits(c.r(788, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(820, 4) as u32),
                f32::from_bits(c.r(816, 4) as u32),
            )
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00002df2 vfma.f32 s15,s13,s8
        let value = f32::from_bits(c.r(820, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(800, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002df6 vstr.32 s11,[r5,#0x32c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(812u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(812, 4));
    }
    {
        // 0x00002dfa vnmul.f32 s13,s14,s11
        let mut t71900 = 0u64;
        t71900 = ((f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64)
            & 0xffffffffu64;
        let value = (-f32::from_bits(((t71900 >> 0) & 0xffffffffu64) as u32)).to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00002dfe vnmul.f32 s14,s6,s14
        let mut t71900 = 0u64;
        t71900 = ((f32::from_bits(c.r(792, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64)
            & 0xffffffffu64;
        let value = (-f32::from_bits(((t71900 >> 0) & 0xffffffffu64) as u32)).to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00002e02 vfma.f32 s13,s9,s7
        let value = f32::from_bits(c.r(804, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(796, 4) as u32),
                f32::from_bits(c.r(820, 4) as u32),
            )
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00002e06 vfma.f32 s14,s10,s7
        let value = f32::from_bits(c.r(808, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(796, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00002e0a vadd.f32 s12,s12,s23
        let value = (f32::from_bits(c.r(816, 4) as u32) + f32::from_bits(c.r(860, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00002e0e b.w 0x00000738
        return 0x738u32;
    }
    0x2e12
}
pub fn b_2e12(c: &mut Machine) -> u32 {
    {
        // 0x00002e12 cmp r2,#0x0
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
        // 0x00002e14 vsub.f32 s11,s15,s8
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(800, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00002e18 it lt
        let mut t185c00 = 0u64;
        t185c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
    }
    {
        // 0x00002e1a add.lt r2,r2,r1
        let mut t1d00 = 0u64;
        let mut t2700 = 0u64;
        t1d00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2700 = ((((t1d00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x2e1cu32;
        }
        let value = ((c.r(40, 4) as u128 + c.r(36, 4) as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(40, 4), c.r(36, 4), 4, false);
        c.w(103, 1, value);
        let value = (c.r(40, 4)).wrapping_add(c.r(36, 4));
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    0x2e1c
}
pub fn b_2e1c(c: &mut Machine) -> u32 {
    {
        // 0x00002e1c subs.w r12,r3,r7
        let mut t1fe000 = 0u64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        t1fe000 = (c.r(60, 4)) & 0xffffffffu64;
        let value = (((t1fe000 >> 0) & 0xffffffffu64) <= c.r(44, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), ((t1fe000 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_sub(((t1fe000 >> 0) & 0xffffffffu64));
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
        // 0x00002e20 vcmpe.f32 s11,#0
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
        // 0x00002e24 it mi
        let mut t184c00 = 0u64;
        t184c00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x00002e26 add.mi r12,r1
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x2e28u32;
        }
        let value = (c.r(80, 4)).wrapping_add(c.r(36, 4));
        c.w(80, 4, value);
    }
    0x2e28
}
pub fn b_2e28(c: &mut Machine) -> u32 {
    {
        // 0x00002e28 add.w r2,r4,r2, lsl #0x2
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
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x00002e2c add.w r12,r4,r12, lsl #0x2
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
        // 0x00002e30 vldr.32 s14,[r2]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(40, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00002e34 vmrs apsr,fpscr
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
        // 0x00002e38 vldr.32 s12,[r12]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(80, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x00002e3c vsub.f32 s12,s12,s14
        let value = (f32::from_bits(c.r(816, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00002e40 ite pl
        let mut t184e00 = 0u64;
        t184e00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x00002e42 vstr.pl.32 s11,[r5,#0x1bc]
        let mut tf00 = 0u64;
        let mut t2600 = 0u64;
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(444u64)) & 0xffffffffu64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2600 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2600 >> 0) & 0xffu64) != 0 {
            return 0x2e46u32;
        }
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(812, 4));
    }
    0x2e46
}
pub fn b_2e46(c: &mut Machine) -> u32 {
    {
        // 0x00002e46 str.w.mi lr,[r5,#0x1bc]
        let mut td00 = 0u64;
        let mut t2700 = 0u64;
        let mut t19c800 = 0u64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x2e4au32;
        }
        t19c800 = ((c.r(52, 4)).wrapping_add(444u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(88, 4));
    }
    0x2e4a
}
pub fn b_2e4a(c: &mut Machine) -> u32 {
    {
        // 0x00002e4a vfma.f32 s14,s15,s12
        let value = f32::from_bits(c.r(828, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(816, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00002e4e vstr.32 s14,[r5,#0x1a4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(420u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x00002e52 b 0x00002cb6
        return 0x2cb6u32;
    }
    0x2e54
}
pub fn b_2e74(c: &mut Machine) -> u32 {
    {
        // 0x00002e74 vmov.f32 s0,s24
        let value = c.r(864, 4);
        c.w(768, 4, value);
    }
    {
        // 0x00002e78 bl 0x00003420
        let value = (11900u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x3420u32;
    }
    0x2e7c
}
pub fn b_2e7c(c: &mut Machine) -> u32 {
    {
        // 0x00002e7c vmov.f32 s26,s0
        let value = c.r(768, 4);
        c.w(872, 4, value);
    }
    {
        // 0x00002e80 vmov.f32 s0,s25
        let value = c.r(868, 4);
        c.w(768, 4, value);
    }
    {
        // 0x00002e84 bl 0x00003420
        let value = (11912u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x3420u32;
    }
    0x2e88
}
pub fn b_2e88(c: &mut Machine) -> u32 {
    {
        // 0x00002e88 vmov.f32 s1,s0
        let value = c.r(768, 4);
        c.w(772, 4, value);
    }
    {
        // 0x00002e8c vmov.f32 s0,s26
        let value = c.r(872, 4);
        c.w(768, 4, value);
    }
    {
        // 0x00002e90 bl 0x00003390
        let value = (11924u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x3390u32;
    }
    0x2e94
}
pub fn b_2e94(c: &mut Machine) -> u32 {
    {
        // 0x00002e94 vldr.32 s15,[pc,#-0x40]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((11924u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(64u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00002e98 vcmpe.f32 s0,s15
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
        // 0x00002e9c vmrs apsr,fpscr
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
        // 0x00002ea0 vselgt.f32 s0,s0,s15
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
        c.w(768, 4, value);
    }
    {
        // 0x00002ea4 bl 0x000033e8
        let value = (11944u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x33e8u32;
    }
    0x2ea8
}
pub fn b_2ea8(c: &mut Machine) -> u32 {
    {
        // 0x00002ea8 vldr.32 s12,[r5,#0x2c4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(708u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x00002eac vldr.32 s13,[pc,#-0x54]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((11948u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(84u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00002eb0 vmov.f32 s15,s12
        let value = c.r(816, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00002eb4 vldr.32 s14,[r5,#0x2d4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(724u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00002eb8 vldr.32 s11,[r5,#0x2d8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(728u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x00002ebc vfnms.f32 s15,s0,s13
        let mut t49a00 = 0u64;
        let mut t49b00 = 0u64;
        t49a00 = ((-f32::from_bits(c.r(828, 4) as u32)).to_bits() as u64) & 0xffffffffu64;
        t49b00 = ((f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64)
            & 0xffffffffu64;
        let value = (f32::from_bits(((t49a00 >> 0) & 0xffffffffu64) as u32)
            + f32::from_bits(((t49b00 >> 0) & 0xffffffffu64) as u32))
        .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002ec0 vmov.f32 s13,0x40000000
        let value = 1073741824u64;
        c.w(820, 4, value);
    }
    {
        // 0x00002ec4 vadd.f32 s15,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002ec8 vcmpe.f32 s15,#0
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
        // 0x00002ecc vmrs apsr,fpscr
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
        // 0x00002ed0 it le
        let mut t186100 = 0u64;
        let mut t186200 = 0u64;
        let mut t186400 = 0u64;
        t186100 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t186200 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t186400 = (((((t186100 >> 0) & 0xffu64) != 0) & (((t186200 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
    }
    {
        // 0x00002ed2 vldr.le.32 s15,[pc,#-0x6c]
        let mut t2200 = 0u64;
        let mut t2400 = 0u64;
        let mut t2700 = 0u64;
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((11986u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(108u64)) & 0xffffffffu64;
        t2200 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2400 = (((c.r(97, 1) != 0) | (((t2200 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        t2700 = ((((t2400 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x2ed6u32;
        }
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    0x2ed6
}
pub fn b_2ed6(c: &mut Machine) -> u32 {
    {
        // 0x00002ed6 vmul.f32 s15,s15,s11
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002eda vcmpe.f32 s15,s13
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
        // 0x00002ede vmrs apsr,fpscr
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
        // 0x00002ee2 bpl.w 0x00003132
        let mut t182d00 = 0u64;
        t182d00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        if ((t182d00 >> 0) & 0xffu64) != 0 {
            return 0x3132u32;
        }
    }
    0x2ee6
}
pub fn b_2ee6(c: &mut Machine) -> u32 {
    {
        // 0x00002ee6 vmov.f32 s13,0x3e800000
        let value = 1048576000u64;
        c.w(820, 4, value);
    }
    {
        // 0x00002eea vmov.f32 s11,0x40800000
        let value = 1082130432u64;
        c.w(812, 4, value);
    }
    {
        // 0x00002eee vmul.f32 s13,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00002ef2 vfms.f32 s13,s13,s13
        let value = (-f32::from_bits(c.r(820, 4) as u32))
            .mul_add(
                f32::from_bits(c.r(820, 4) as u32),
                f32::from_bits(c.r(820, 4) as u32),
            )
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00002ef6 vmul.f32 s13,s13,s11
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    0x2efa
}
pub fn b_2efa(c: &mut Machine) -> u32 {
    {
        // 0x00002efa vsub.f32 s15,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002efe vldr.32 s13,[r5,#0x2e0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(736u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00002f02 vmul.f32 s15,s14,s15
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002f06 vcmpe.f32 s15,s13
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
        // 0x00002f0a vmrs apsr,fpscr
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
        // 0x00002f0e bgt 0x00002f20
        let mut t181b00 = 0u64;
        let mut t181c00 = 0u64;
        let mut t181e00 = 0u64;
        t181b00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t181c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t181e00 = (((((t181b00 >> 0) & 0xffu64) != 0) & (((t181c00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t181e00 >> 0) & 0xffu64) != 0 {
            return 0x2f20u32;
        }
    }
    0x2f10
}
pub fn b_2f10(c: &mut Machine) -> u32 {
    {
        // 0x00002f10 vsub.f32 s11,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00002f14 vldr.32 s14,[r5,#0x2d0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(720u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00002f18 vadd.f32 s15,s13,s23
        let value = (f32::from_bits(c.r(820, 4) as u32) + f32::from_bits(c.r(860, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002f1c vfma.f32 s15,s11,s14
        let value = f32::from_bits(c.r(812, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(824, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    0x2f20
}
pub fn b_2f20(c: &mut Machine) -> u32 {
    {
        // 0x00002f20 vldr.32 s27,[r5,#0x2dc]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(732u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(876, 4, value);
    }
    {
        // 0x00002f24 vmov.f32 s0,0x40400000
        let value = 1077936128u64;
        c.w(768, 4, value);
    }
    {
        // 0x00002f28 vldr.32 s13,[r5,#0x2cc]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(716u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00002f2c vsub.f32 s11,s15,s27
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(876, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00002f30 vldr.32 s14,[r5,#0x2c8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(712u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00002f34 vadd.f32 s27,s27,s23
        let value = (f32::from_bits(c.r(876, 4) as u32) + f32::from_bits(c.r(860, 4) as u32))
            .to_bits() as u64;
        c.w(876, 4, value);
    }
    {
        // 0x00002f38 vldr.32 s28,[pc,#-0xdc]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((12088u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(220u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(880, 4, value);
    }
    {
        // 0x00002f3c vstr.32 s15,[r5,#0x2e0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(736u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x00002f40 vfma.f32 s27,s11,s13
        let value = f32::from_bits(c.r(812, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(820, 4) as u32),
                f32::from_bits(c.r(876, 4) as u32),
            )
            .to_bits() as u64;
        c.w(876, 4, value);
    }
    {
        // 0x00002f44 vadd.f32 s0,s27,s0
        let value = (f32::from_bits(c.r(876, 4) as u32) + f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x00002f48 vstr.32 s27,[r5,#0x2dc]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(732u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(876, 4));
    }
    {
        // 0x00002f4c vfma.f32 s0,s12,s14
        let value = f32::from_bits(c.r(816, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(824, 4) as u32),
                f32::from_bits(c.r(768, 4) as u32),
            )
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x00002f50 vmul.f32 s0,s0,s28
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(880, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x00002f54 bl 0x000033b0
        let value = (12120u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x33b0u32;
    }
    0x2f58
}
pub fn b_2f58(c: &mut Machine) -> u32 {
    {
        // 0x00002f58 vmov.f32 s15,0x3fc00000
        let value = 1069547520u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002f5c vldr.32 s11,[pc,#-0xfc]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((12124u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(252u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x00002f60 vldr.32 s12,[r5,#0x2c8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(712u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x00002f64 vmul.f32 s15,s0,s15
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002f68 vmul.f32 s0,s0,s0
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x00002f6c vnmul.f32 s12,s12,s27
        let mut t71900 = 0u64;
        t71900 = ((f32::from_bits(c.r(816, 4) as u32) * f32::from_bits(c.r(876, 4) as u32))
            .to_bits() as u64)
            & 0xffffffffu64;
        let value = (-f32::from_bits(((t71900 >> 0) & 0xffffffffu64) as u32)).to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00002f70 vneg.f32 s14,s15
        let value = (-f32::from_bits(c.r(828, 4) as u32)).to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00002f74 vstr.32 s15,[r5,#0x2b4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(692u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x00002f78 vdiv.f32 s13,s11,s0
        let value = (f32::from_bits(c.r(812, 4) as u32) / f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00002f7c vstr.32 s14,[r5,#0x2b8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(696u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x00002f80 vcmpe.f32 s24,s14
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
        t35c00 = ((f32::from_bits(c.r(864, 4) as u32) < f32::from_bits(c.r(824, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(864, 4) as u32) == f32::from_bits(c.r(824, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(824, 4) as u32) <= f32::from_bits(c.r(864, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(864, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x00002f84 vmul.f32 s0,s12,s28
        let value = (f32::from_bits(c.r(816, 4) as u32) * f32::from_bits(c.r(880, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x00002f88 vmrs apsr,fpscr
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
        // 0x00002f8c vselgt.f32 s24,s24,s14
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
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(864, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(824, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(864, 4, value);
    }
    {
        // 0x00002f90 vcmpe.f32 s15,s24
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
        // 0x00002f94 vmrs apsr,fpscr
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
        // 0x00002f98 vcmpe.f32 s25,s14
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
        t35c00 = ((f32::from_bits(c.r(868, 4) as u32) < f32::from_bits(c.r(824, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(868, 4) as u32) == f32::from_bits(c.r(824, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(824, 4) as u32) <= f32::from_bits(c.r(868, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(868, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x00002f9c vstr.32 s13,[r5,#0x2bc]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(700u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x00002fa0 vselgt.f32 s24,s24,s15
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
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(864, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(828, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(864, 4, value);
    }
    {
        // 0x00002fa4 vmul.f32 s12,s24,s24
        let value = (f32::from_bits(c.r(864, 4) as u32) * f32::from_bits(c.r(864, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00002fa8 vmrs apsr,fpscr
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
        // 0x00002fac vmul.f32 s12,s12,s24
        let value = (f32::from_bits(c.r(816, 4) as u32) * f32::from_bits(c.r(864, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00002fb0 vselgt.f32 s14,s25,s14
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
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(868, 4))) & 0xffffffffu64;
        tdc800 = ((((tdbc00 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(824, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(824, 4, value);
    }
    {
        // 0x00002fb4 vcmpe.f32 s15,s14
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
        // 0x00002fb8 vfms.f32 s24,s13,s12
        let value = (-f32::from_bits(c.r(820, 4) as u32))
            .mul_add(
                f32::from_bits(c.r(816, 4) as u32),
                f32::from_bits(c.r(864, 4) as u32),
            )
            .to_bits() as u64;
        c.w(864, 4, value);
    }
    {
        // 0x00002fbc vmrs apsr,fpscr
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
        // 0x00002fc0 vselgt.f32 s15,s14,s15
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
        // 0x00002fc4 vmul.f32 s14,s15,s15
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00002fc8 vmul.f32 s14,s14,s15
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00002fcc vfms.f32 s15,s13,s14
        let value = (-f32::from_bits(c.r(820, 4) as u32))
            .mul_add(
                f32::from_bits(c.r(824, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002fd0 vmov.f32 s26,s15
        let value = c.r(828, 4);
        c.w(872, 4, value);
    }
    {
        // 0x00002fd4 bl 0x000033b0
        let value = (12248u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x33b0u32;
    }
    0x2fd8
}
pub fn b_2fd8(c: &mut Machine) -> u32 {
    {
        // 0x00002fd8 vldr.32 s15,[r5,#0x2c8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(712u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00002fdc vmul.f32 s24,s24,s0
        let value = (f32::from_bits(c.r(864, 4) as u32) * f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(864, 4, value);
    }
    {
        // 0x00002fe0 vnmul.f32 s15,s15,s27
        let mut t71900 = 0u64;
        t71900 = ((f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(876, 4) as u32))
            .to_bits() as u64)
            & 0xffffffffu64;
        let value = (-f32::from_bits(((t71900 >> 0) & 0xffffffffu64) as u32)).to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002fe4 vmul.f32 s0,s15,s28
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(880, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x00002fe8 bl 0x000033b0
        let value = (12268u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x33b0u32;
    }
    0x2fec
}
pub fn b_2fec(c: &mut Machine) -> u32 {
    {
        // 0x00002fec vmul.f32 s15,s26,s0
        let value = (f32::from_bits(c.r(872, 4) as u32) * f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00002ff0 vldr.32 s25,[r5,#0x380]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(896u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(868, 4, value);
    }
    {
        // 0x00002ff4 ldrb.w r3,[r5,#0x385]
        let mut t19c800 = 0u64;
        let mut t1aa600 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(901u64)) & 0xffffffffu64;
        t1aa600 = (c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 1)) & 0xffu64;
        let value = ((t1aa600 >> 0) & 0xffu64);
        c.w(44, 4, value);
    }
    {
        // 0x00002ff8 vmul.f32 s24,s25,s24
        let value = (f32::from_bits(c.r(868, 4) as u32) * f32::from_bits(c.r(864, 4) as u32))
            .to_bits() as u64;
        c.w(864, 4, value);
    }
    {
        // 0x00002ffc vmul.f32 s25,s15,s25
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(868, 4) as u32))
            .to_bits() as u64;
        c.w(868, 4, value);
    }
    {
        // 0x00003000 vstr.32 s24,[sp,#0x40]
        let mut t64000 = 0u64;
        t64000 = ((c.r(84, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(864, 4));
    }
    {
        // 0x00003004 vstr.32 s25,[sp,#0x44]
        let mut t64000 = 0u64;
        t64000 = ((c.r(84, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(868, 4));
    }
    {
        // 0x00003008 cmp r3,#0x0
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
        // 0x0000300a beq.w 0x00000840
        let mut t182300 = 0u64;
        t182300 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        if ((t182300 >> 0) & 0xffu64) != 0 {
            return 0x840u32;
        }
    }
    0x300e
}
pub fn b_300e(c: &mut Machine) -> u32 {
    {
        // 0x0000300e vmov.f32 s0,s17
        let value = c.r(836, 4);
        c.w(768, 4, value);
    }
    {
        // 0x00003012 bl 0x00003420
        let value = (12310u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x3420u32;
    }
    0x3016
}
pub fn b_3016(c: &mut Machine) -> u32 {
    {
        // 0x00003016 vmov.f32 s26,s0
        let value = c.r(768, 4);
        c.w(872, 4, value);
    }
    {
        // 0x0000301a vmov.f32 s0,s16
        let value = c.r(832, 4);
        c.w(768, 4, value);
    }
    {
        // 0x0000301e bl 0x00003420
        let value = (12322u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x3420u32;
    }
    0x3022
}
pub fn b_3022(c: &mut Machine) -> u32 {
    {
        // 0x00003022 vmov.f32 s1,s0
        let value = c.r(768, 4);
        c.w(772, 4, value);
    }
    {
        // 0x00003026 vmov.f32 s0,s26
        let value = c.r(872, 4);
        c.w(768, 4, value);
    }
    {
        // 0x0000302a bl 0x00003390
        let value = (12334u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x3390u32;
    }
    0x302e
}
pub fn b_302e(c: &mut Machine) -> u32 {
    {
        // 0x0000302e vldr.32 s15,[r5,#0x2e8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(744u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00003032 vmov.f32 s14,0x3f800000
        let value = 1065353216u64;
        c.w(824, 4, value);
    }
    {
        // 0x00003036 vcmpe.f32 s0,s15
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
        // 0x0000303a vmrs apsr,fpscr
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
        // 0x0000303e bgt 0x0000312c
        let mut t181b00 = 0u64;
        let mut t181c00 = 0u64;
        let mut t181e00 = 0u64;
        t181b00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t181c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t181e00 = (((((t181b00 >> 0) & 0xffu64) != 0) & (((t181c00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t181e00 >> 0) & 0xffu64) != 0 {
            return 0x312cu32;
        }
    }
    0x3040
}
pub fn b_3040(c: &mut Machine) -> u32 {
    {
        // 0x00003040 vldr.32 s13,[r5,#0x310]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(784u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00003044 vadd.f32 s14,s13,s14
        let value = (f32::from_bits(c.r(820, 4) as u32) + f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    0x3048
}
pub fn b_3048(c: &mut Machine) -> u32 {
    {
        // 0x00003048 vldr.32 s11,[r5,#0x30c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(780u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x0000304c vstr.32 s14,[r5,#0x310]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(784u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x00003050 vcmpe.f32 s11,s13
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
        t35c00 = ((f32::from_bits(c.r(812, 4) as u32) < f32::from_bits(c.r(820, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(812, 4) as u32) == f32::from_bits(c.r(820, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(820, 4) as u32) <= f32::from_bits(c.r(812, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(812, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x00003054 vldr.32 s15,[r5,#0x304]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(772u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00003058 vldr.32 s12,[r5,#0x300]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(768u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x0000305c vmrs apsr,fpscr
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
        // 0x00003060 ite ge
        let mut t185a00 = 0u64;
        t185a00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
    }
    {
        // 0x00003062 mov.ge r3,#0x1
        let mut t1b00 = 0u64;
        let mut t2600 = 0u64;
        t1b00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t2600 = ((((t1b00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2600 >> 0) & 0xffu64) != 0 {
            return 0x3064u32;
        }
        let value = 1u64;
        c.w(44, 4, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    0x3064
}
pub fn b_3064(c: &mut Machine) -> u32 {
    {
        // 0x00003064 mov.lt r3,#0x0
        let mut t1d00 = 0u64;
        let mut t2700 = 0u64;
        t1d00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2700 = ((((t1d00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x3066u32;
        }
        let value = 0u64;
        c.w(44, 4, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    0x3066
}
pub fn b_3066(c: &mut Machine) -> u32 {
    {
        // 0x00003066 vmov s14,r3
        let value = c.r(44, 4);
        c.w(824, 4, value);
    }
    {
        // 0x0000306a vcvt.f32.s32 s14,s14
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
        // 0x0000306e vsub.f32 s14,s14,s15
        let value = (f32::from_bits(c.r(824, 4) as u32) - f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00003072 vfma.f32 s15,s14,s12
        let value = f32::from_bits(c.r(824, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(816, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00003076 vldr.32 s14,[r5,#0x2f8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(760u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x0000307a vadd.f32 s15,s15,s23
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(860, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x0000307e vcmpe.f32 s15,#0
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
        // 0x00003082 vstr.32 s15,[r5,#0x304]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(772u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x00003086 vmrs apsr,fpscr
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
        // 0x0000308a it le
        let mut t186100 = 0u64;
        let mut t186200 = 0u64;
        let mut t186400 = 0u64;
        t186100 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t186200 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t186400 = (((((t186100 >> 0) & 0xffu64) != 0) & (((t186200 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
    }
    {
        // 0x0000308c vldr.le.32 s15,[pc,#-0x228]
        let mut t2200 = 0u64;
        let mut t2400 = 0u64;
        let mut t2700 = 0u64;
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((12428u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(552u64)) & 0xffffffffu64;
        t2200 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2400 = (((c.r(97, 1) != 0) | (((t2200 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        t2700 = ((((t2400 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x3090u32;
        }
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    0x3090
}
pub fn b_3090(c: &mut Machine) -> u32 {
    {
        // 0x00003090 vcmpe.f32 s14,s15
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
        // 0x00003094 vmrs apsr,fpscr
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
        // 0x00003098 bmi 0x000030a2
        let mut t180a00 = 0u64;
        t180a00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        if ((t180a00 >> 0) & 0xffu64) != 0 {
            return 0x30a2u32;
        }
    }
    0x309a
}
pub fn b_309a(c: &mut Machine) -> u32 {
    {
        // 0x0000309a vldr.32 s15,[r5,#0x2f0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(752u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x0000309e vmul.f32 s15,s14,s15
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    0x30a2
}
pub fn b_30a2(c: &mut Machine) -> u32 {
    {
        // 0x000030a2 vmul.f32 s24,s24,s15
        let value = (f32::from_bits(c.r(864, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(864, 4, value);
    }
    {
        // 0x000030a6 vstr.32 s15,[r5,#0x2f8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(760u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x000030aa vmul.f32 s25,s25,s15
        let value = (f32::from_bits(c.r(868, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(868, 4, value);
    }
    {
        // 0x000030ae b.w 0x00000840
        return 0x840u32;
    }
    0x30b2
}
pub fn b_30b2(c: &mut Machine) -> u32 {
    {
        // 0x000030b2 vmov.f32 s15,s23
        let value = c.r(860, 4);
        c.w(828, 4, value);
    }
    {
        // 0x000030b6 b 0x000029c4
        return 0x29c4u32;
    }
    0x30b8
}
pub fn b_30b8(c: &mut Machine) -> u32 {
    {
        // 0x000030b8 vldr.32 s14,[pc,#-0x254]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((12472u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(596u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x000030bc b 0x00002bf8
        return 0x2bf8u32;
    }
    0x30be
}
pub fn b_30be(c: &mut Machine) -> u32 {
    {
        // 0x000030be vldr.32 s15,[pc,#-0x258]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((12478u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(600u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x000030c2 b.w 0x00000dd4
        return 0xdd4u32;
    }
    0x30c6
}
pub fn b_30c6(c: &mut Machine) -> u32 {
    {
        // 0x000030c6 vldr.32 s8,[pc,#-0x260]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((12486u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(608u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x000030ca b 0x00002daa
        return 0x2daau32;
    }
    0x30cc
}
pub fn b_30cc(c: &mut Machine) -> u32 {
    {
        // 0x000030cc vldr.32 s15,[pc,#-0x268]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((12492u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(616u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x000030d0 b.w 0x0000168c
        return 0x168cu32;
    }
    0x30d4
}
pub fn b_30d4(c: &mut Machine) -> u32 {
    {
        // 0x000030d4 vmov.f32 s15,s23
        let value = c.r(860, 4);
        c.w(828, 4, value);
    }
    {
        // 0x000030d8 b.w 0x00001480
        return 0x1480u32;
    }
    0x30dc
}
pub fn b_30dc(c: &mut Machine) -> u32 {
    {
        // 0x000030dc vmov.f32 s14,s23
        let value = c.r(860, 4);
        c.w(824, 4, value);
    }
    {
        // 0x000030e0 b.w 0x00001240
        return 0x1240u32;
    }
    0x30e4
}
pub fn b_30e4(c: &mut Machine) -> u32 {
    {
        // 0x000030e4 vmov.f32 s15,s23
        let value = c.r(860, 4);
        c.w(828, 4, value);
    }
    {
        // 0x000030e8 b.w 0x00000ff6
        return 0xff6u32;
    }
    0x30ec
}
pub fn b_30ec(c: &mut Machine) -> u32 {
    {
        // 0x000030ec vmov.f32 s14,s23
        let value = c.r(860, 4);
        c.w(824, 4, value);
    }
    {
        // 0x000030f0 b.w 0x0000250c
        return 0x250cu32;
    }
    0x30f4
}
pub fn b_30f4(c: &mut Machine) -> u32 {
    {
        // 0x000030f4 vmov.f32 s14,s23
        let value = c.r(860, 4);
        c.w(824, 4, value);
    }
    {
        // 0x000030f8 b.w 0x0000280e
        return 0x280eu32;
    }
    0x30fc
}
pub fn b_30fc(c: &mut Machine) -> u32 {
    {
        // 0x000030fc vmov.f32 s13,s23
        let value = c.r(860, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00003100 b.w 0x00002162
        return 0x2162u32;
    }
    0x3104
}
pub fn b_3104(c: &mut Machine) -> u32 {
    {
        // 0x00003104 vmov.f32 s13,s23
        let value = c.r(860, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00003108 b.w 0x00002302
        return 0x2302u32;
    }
    0x310c
}
pub fn b_310c(c: &mut Machine) -> u32 {
    {
        // 0x0000310c vmov.f32 s15,s23
        let value = c.r(860, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00003110 b.w 0x00001d3c
        return 0x1d3cu32;
    }
    0x3114
}
pub fn b_3114(c: &mut Machine) -> u32 {
    {
        // 0x00003114 vmov.f32 s15,s23
        let value = c.r(860, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00003118 b.w 0x00001f5e
        return 0x1f5eu32;
    }
    0x311c
}
pub fn b_311c(c: &mut Machine) -> u32 {
    {
        // 0x0000311c vmov.f32 s14,s23
        let value = c.r(860, 4);
        c.w(824, 4, value);
    }
    {
        // 0x00003120 b.w 0x00001b14
        return 0x1b14u32;
    }
    0x3124
}
pub fn b_3124(c: &mut Machine) -> u32 {
    {
        // 0x00003124 vmov.f32 s15,s23
        let value = c.r(860, 4);
        c.w(828, 4, value);
    }
    {
        // 0x00003128 b.w 0x000018d0
        return 0x18d0u32;
    }
    0x312c
}
pub fn b_312c(c: &mut Machine) -> u32 {
    {
        // 0x0000312c vldr.32 s13,[pc,#-0x2c8]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((12588u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(712u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x00003130 b 0x00003048
        return 0x3048u32;
    }
    0x3132
}
pub fn b_3132(c: &mut Machine) -> u32 {
    {
        // 0x00003132 vmov.f32 s13,0x3f800000
        let value = 1065353216u64;
        c.w(820, 4, value);
    }
    {
        // 0x00003136 b 0x00002efa
        return 0x2efau32;
    }
    0x3138
}
pub fn b_3138(c: &mut Machine) -> u32 {
    {
        // 0x00003138 add sp,#0x54
        let mut t187500 = 0u64;
        t187500 = (84u64) & 0xffffffffu64;
        let value = (c.r(84, 4)).wrapping_add(((t187500 >> 0) & 0xffffffffu64));
        c.w(84, 4, value);
    }
    {
        // 0x0000313a vpop {d8,d9,d10,d11,d12,d13,d14,d15}
        let mut t74b00 = 0u64;
        let value = c.r(84, 4);
        c.w(128, 4, value);
        t74b00 = ((16u64).wrapping_mul(4u64)) & 0xffffffffu64;
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
        let value = c.load(c.r(128, 4) as u32, 8);
        c.w(856, 8, value);
        let value = (c.r(128, 4)).wrapping_add(8u64);
        c.w(128, 4, value);
        let value = c.load(c.r(128, 4) as u32, 8);
        c.w(864, 8, value);
        let value = (c.r(128, 4)).wrapping_add(8u64);
        c.w(128, 4, value);
        let value = c.load(c.r(128, 4) as u32, 8);
        c.w(872, 8, value);
        let value = (c.r(128, 4)).wrapping_add(8u64);
        c.w(128, 4, value);
        let value = c.load(c.r(128, 4) as u32, 8);
        c.w(880, 8, value);
        let value = (c.r(128, 4)).wrapping_add(8u64);
        c.w(128, 4, value);
        let value = c.load(c.r(128, 4) as u32, 8);
        c.w(888, 8, value);
        let value = (c.r(128, 4)).wrapping_add(8u64);
        c.w(128, 4, value);
    }
    {
        // 0x0000313e pop.w {r4,r5,r6,r7,r8,r9,r10,r11,pc}
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
    0x3142
}
pub fn b_3142(c: &mut Machine) -> u32 {
    {
        // 0x00003142 vmul.f32 s15,s14,s14
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00003146 vldr.32 s8,[pc,#-0x2dc]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((12614u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(732u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x0000314a vldr.32 s9,[pc,#-0x2dc]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((12618u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(732u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(804, 4, value);
    }
    {
        // 0x0000314e vmul.f32 s13,s14,s15
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00003152 vfma.f32 s14,s13,s8
        let value = f32::from_bits(c.r(820, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(800, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x00003156 vmul.f32 s15,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x0000315a vfma.f32 s14,s15,s9
        let value = f32::from_bits(c.r(828, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x0000315e vadd.f32 s15,s14,s11
        let value = (f32::from_bits(c.r(824, 4) as u32) + f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00003162 vsub.f32 s11,s14,s11
        let value = (f32::from_bits(c.r(824, 4) as u32) - f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x00003166 vdiv.f32 s8,s14,s15
        let value = (f32::from_bits(c.r(824, 4) as u32) / f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(800, 4, value);
    }
    {
        // 0x0000316a vdiv.f32 s13,s11,s15
        let value = (f32::from_bits(c.r(812, 4) as u32) / f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x0000316e b.w 0x0000028c
        return 0x28cu32;
    }
    0x3172
}
pub fn b_3172(c: &mut Machine) -> u32 {
    {
        // 0x00003172 vmul.f32 s13,s15,s15
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x00003176 vldr.32 s10,[pc,#-0x30c]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((12662u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(780u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x0000317a vldr.32 s11,[pc,#-0x30c]
        let mut t64700 = 0u64;
        let mut t64800 = 0u64;
        let mut t64a00 = 0u64;
        t64700 = ((12666u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64800 = (((t64700 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64a00 = (((t64800 >> 0) & 0xffffffffu64).wrapping_sub(780u64)) & 0xffffffffu64;
        let value = c.load(((t64a00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x0000317e vmul.f32 s12,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x00003182 vfma.f32 s15,s12,s10
        let value = f32::from_bits(c.r(816, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00003186 vmul.f32 s13,s13,s12
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x0000318a vfma.f32 s15,s13,s11
        let value = f32::from_bits(c.r(820, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(812, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x0000318e vadd.f32 s10,s15,s9
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(804, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x00003192 vsub.f32 s15,s15,s9
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(804, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x00003196 vdiv.f32 s13,s15,s10
        let value = (f32::from_bits(c.r(828, 4) as u32) / f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x0000319a b.w 0x0000032c
        return 0x32cu32;
    }
    0x319e
}
