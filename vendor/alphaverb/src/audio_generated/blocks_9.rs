#![allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut,
    unused_comparisons
)]
use crate::machine::{signed, signed_overflow, Machine};
pub fn b_600412a4(c: &mut Machine) -> u32 {
    {
        // 0x600412a4 vmov.f32 s0,s16
        let value = c.r(832, 4);
        c.w(768, 4, value);
    }
    {
        // 0x600412a8 vpop {d8}
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
        // 0x600412ac str r4,[r0,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(32, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x600412ae pop {r4,pc}
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
    0x600412b0
}
pub fn b_600412b0(c: &mut Machine) -> u32 {
    {
        // 0x600412b0 cbz r0,0x600412c0
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x600412c0u32;
        }
    }
    0x600412b2
}
pub fn b_600412b2(c: &mut Machine) -> u32 {
    {
        // 0x600412b2 vneg.f32 s15,s0
        let value = (-f32::from_bits(c.r(768, 4) as u32)).to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x600412b6 vmul.f32 s0,s15,s0
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x600412ba movs r0,#0x22
        let value = 34u64;
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
        // 0x600412bc b.w 0x60041294
        return 0x60041294u32;
    }
    0x600412c0
}
pub fn b_600412c0(c: &mut Machine) -> u32 {
    {
        // 0x600412c0 vmov.f32 s15,s0
        let value = c.r(768, 4);
        c.w(828, 4, value);
    }
    {
        // 0x600412c4 vmul.f32 s0,s15,s0
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x600412c8 movs r0,#0x22
        let value = 34u64;
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
        // 0x600412ca b.w 0x60041294
        return 0x60041294u32;
    }
    0x600412ce
}
pub fn b_600412d0(c: &mut Machine) -> u32 {
    {
        // 0x600412d0 vldr.32 s0,[pc,#0x4]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610879696u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x600412d4 b.w 0x600412b0
        return 0x600412b0u32;
    }
    0x600412d8
}
pub fn b_600412dc(c: &mut Machine) -> u32 {
    {
        // 0x600412dc vldr.32 s0,[pc,#0x4]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610879708u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x600412e0 b.w 0x600412b0
        return 0x600412b0u32;
    }
    0x600412e4
}
pub fn b_600412e8(c: &mut Machine) -> u32 {
    {
        // 0x600412e8 vldr.32 s0,[pc,#0x4]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610879720u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x600412ec b.w 0x600412b0
        return 0x600412b0u32;
    }
    0x600412f0
}
pub fn b_600412f4(c: &mut Machine) -> u32 {
    {
        // 0x600412f4 vldr.32 s0,[pc,#0x1c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610879732u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(28u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x600412f8 vmov.f32 s15,0x3f800000
        let value = 1065353216u64;
        c.w(828, 4, value);
    }
    {
        // 0x600412fc vmov.f32 s14,0xbf800000
        let value = 3212836864u64;
        c.w(824, 4, value);
    }
    {
        // 0x60041300 cmp r0,#0x0
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
        // 0x60041302 mov.w r0,#0x22
        let mut t187d00 = 0u64;
        t187d00 = (34u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = ((t187d00 >> 0) & 0xffffffffu64);
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60041306 vseleq.f32 s15,s15,s14
        let mut tdb600 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb600 = (c.r(97, 1)) & 0xffu64;
        tdc500 = ((((tdb600 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(828, 4))) & 0xffffffffu64;
        tdc800 = ((((tdb600 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(824, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(828, 4, value);
    }
    {
        // 0x6004130a vdiv.f32 s0,s15,s0
        let value = (f32::from_bits(c.r(828, 4) as u32) / f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6004130e b.w 0x60041294
        return 0x60041294u32;
    }
    0x60041312
}
pub fn b_60041318(c: &mut Machine) -> u32 {
    {
        // 0x60041318 vmov.f32 s15,s0
        let value = c.r(768, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6004131c vsub.f32 s14,s0,s0
        let value = (f32::from_bits(c.r(768, 4) as u32) - f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x60041320 vcmpe.f32 s15,s15
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
        t35c00 = ((f32::from_bits(c.r(828, 4) as u32) < f32::from_bits(c.r(828, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(828, 4) as u32) == f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(828, 4) as u32) <= f32::from_bits(c.r(828, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(828, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x60041324 vmrs apsr,fpscr
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
        // 0x60041328 vdiv.f32 s0,s14,s14
        let value = (f32::from_bits(c.r(824, 4) as u32) / f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6004132c bvs 0x60041334
        let mut t180e00 = 0u64;
        t180e00 = ((c.r(99, 1) != 0u64) as u64) & 0xffu64;
        if ((t180e00 >> 0) & 0xffu64) != 0 {
            return 0x60041334u32;
        }
    }
    0x6004132e
}
pub fn b_6004132e(c: &mut Machine) -> u32 {
    {
        // 0x6004132e movs r0,#0x21
        let value = 33u64;
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
        // 0x60041330 b.w 0x60041294
        return 0x60041294u32;
    }
    0x60041334
}
pub fn b_60041334(c: &mut Machine) -> u32 {
    {
        // 0x60041334 bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x60041336
}
pub fn b_60041338(c: &mut Machine) -> u32 {
    {
        // 0x60041338 push {r3,lr}
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
        // 0x6004133a vpush {d8}
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
        // 0x6004133e vmov.f32 s16,s0
        let value = c.r(768, 4);
        c.w(832, 4, value);
    }
    {
        // 0x60041342 bl 0x6004138c
        let value = (1610879814u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x6004138cu32;
    }
    0x60041346
}
pub fn b_60041346(c: &mut Machine) -> u32 {
    {
        // 0x60041346 vcmpe.f32 s16,s16
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
        t35c00 = ((f32::from_bits(c.r(832, 4) as u32) < f32::from_bits(c.r(832, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(832, 4) as u32) == f32::from_bits(c.r(832, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(832, 4) as u32) <= f32::from_bits(c.r(832, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(832, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6004134a vmrs apsr,fpscr
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
        // 0x6004134e bvs 0x6004135e
        let mut t180e00 = 0u64;
        t180e00 = ((c.r(99, 1) != 0u64) as u64) & 0xffu64;
        if ((t180e00 >> 0) & 0xffu64) != 0 {
            return 0x6004135eu32;
        }
    }
    0x60041350
}
pub fn b_60041350(c: &mut Machine) -> u32 {
    {
        // 0x60041350 vldr.32 s17,[pc,#0x24]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610879824u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(36u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(836, 4, value);
    }
    {
        // 0x60041354 vcmpe.f32 s16,s17
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
        t35c00 = ((f32::from_bits(c.r(832, 4) as u32) < f32::from_bits(c.r(836, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(832, 4) as u32) == f32::from_bits(c.r(836, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(836, 4) as u32) <= f32::from_bits(c.r(832, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(832, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x60041358 vmrs apsr,fpscr
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
        // 0x6004135c bmi 0x60041364
        let mut t180a00 = 0u64;
        t180a00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        if ((t180a00 >> 0) & 0xffu64) != 0 {
            return 0x60041364u32;
        }
    }
    0x6004135e
}
pub fn b_6004135e(c: &mut Machine) -> u32 {
    {
        // 0x6004135e vpop {d8}
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
        // 0x60041362 pop {r3,pc}
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
    0x60041364
}
pub fn b_60041364(c: &mut Machine) -> u32 {
    {
        // 0x60041364 bl 0x600419c4
        let value = (1610879848u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600419c4u32;
    }
    0x60041368
}
pub fn b_60041368(c: &mut Machine) -> u32 {
    {
        // 0x60041368 vdiv.f32 s0,s17,s17
        let value = (f32::from_bits(c.r(836, 4) as u32) / f32::from_bits(c.r(836, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6004136c vpop {d8}
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
        // 0x60041370 movs r3,#0x21
        let value = 33u64;
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
        // 0x60041372 str r3,[r0,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(32, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x60041374 pop {r3,pc}
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
    0x60041376
}
pub fn b_6004137c(c: &mut Machine) -> u32 {
    {
        // 0x6004137c vmov r3,s0
        let value = c.r(768, 4);
        c.w(44, 4, value);
    }
    {
        // 0x60041380 bic r3,r3,#0x80000000
        let mut t188100 = 0u64;
        let mut t188300 = 0u64;
        let mut t1a4200 = 0u64;
        t188100 = (2147483648u64) & 0xffffffffu64;
        t188300 = (if 31u64 >= 32 {
            0
        } else {
            ((t188100 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188300 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        t1a4200 = (!((t188100 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        let value = (c.r(44, 4) & ((t1a4200 >> 0) & 0xffffffffu64));
        c.w(44, 4, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60041384 vmov s0,r3
        let value = c.r(44, 4);
        c.w(768, 4, value);
    }
    {
        // 0x60041388 bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x6004138a
}
pub fn b_6004138c(c: &mut Machine) -> u32 {
    {
        // 0x6004138c vsqrt.f32 s0,s0
        let value = f32::from_bits(c.r(768, 4) as u32).sqrt().to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x60041390 bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x60041392
}
pub fn b_60041394(c: &mut Machine) -> u32 {
    {
        // 0x60041394 push {r3,lr}
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
        // 0x60041396 vpush {d8}
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
        // 0x6004139a vmov.f32 s17,s0
        let value = c.r(768, 4);
        c.w(836, 4, value);
    }
    {
        // 0x6004139e vmov.f32 s16,s1
        let value = c.r(772, 4);
        c.w(832, 4, value);
    }
    {
        // 0x600413a2 bl 0x600413d0
        let value = (1610879910u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600413d0u32;
    }
    0x600413a6
}
pub fn b_600413a6(c: &mut Machine) -> u32 {
    {
        // 0x600413a6 cbnz r0,0x600413b2
        let mut t1a5900 = 0u64;
        t1a5900 = ((c.r(32, 4) != 0u64) as u64) & 0xffu64;
        if ((t1a5900 >> 0) & 0xffu64) != 0 {
            return 0x600413b2u32;
        }
    }
    0x600413a8
}
pub fn b_600413a8(c: &mut Machine) -> u32 {
    {
        // 0x600413a8 vmov.f32 s0,s16
        let value = c.r(832, 4);
        c.w(768, 4, value);
    }
    {
        // 0x600413ac vpop {d8}
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
        // 0x600413b0 pop {r3,pc}
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
    0x600413b2
}
pub fn b_600413b2(c: &mut Machine) -> u32 {
    {
        // 0x600413b2 vmov.f32 s0,s16
        let value = c.r(832, 4);
        c.w(768, 4, value);
    }
    {
        // 0x600413b6 bl 0x600413d0
        let value = (1610879930u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600413d0u32;
    }
    0x600413ba
}
pub fn b_600413ba(c: &mut Machine) -> u32 {
    {
        // 0x600413ba cbz r0,0x600413c6
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x600413c6u32;
        }
    }
    0x600413bc
}
pub fn b_600413bc(c: &mut Machine) -> u32 {
    {
        // 0x600413bc vcmpe.f32 s17,s16
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
        t35c00 = ((f32::from_bits(c.r(836, 4) as u32) < f32::from_bits(c.r(832, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(836, 4) as u32) == f32::from_bits(c.r(832, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(832, 4) as u32) <= f32::from_bits(c.r(836, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(836, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x600413c0 vmrs apsr,fpscr
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
        // 0x600413c4 ble 0x600413a8
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x600413a8u32;
        }
    }
    0x600413c6
}
pub fn b_600413c6(c: &mut Machine) -> u32 {
    {
        // 0x600413c6 vmov.f32 s0,s17
        let value = c.r(836, 4);
        c.w(768, 4, value);
    }
    {
        // 0x600413ca vpop {d8}
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
        // 0x600413ce pop {r3,pc}
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
    0x600413d0
}
pub fn b_600413d0(c: &mut Machine) -> u32 {
    {
        // 0x600413d0 vmov r3,s0
        let value = c.r(768, 4);
        c.w(44, 4, value);
    }
    {
        // 0x600413d4 bics r0,r3,#0x80000000
        let mut t188100 = 0u64;
        let mut t188300 = 0u64;
        let mut t1a4200 = 0u64;
        t188100 = (2147483648u64) & 0xffffffffu64;
        t188300 = (if 31u64 >= 32 {
            0
        } else {
            ((t188100 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188300 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        t1a4200 = (!((t188100 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        let value = (c.r(44, 4) & ((t1a4200 >> 0) & 0xffffffffu64));
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
        // 0x600413d8 beq 0x600413f8
        let mut t180200 = 0u64;
        t180200 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        if ((t180200 >> 0) & 0xffu64) != 0 {
            return 0x600413f8u32;
        }
    }
    0x600413da
}
pub fn b_600413da(c: &mut Machine) -> u32 {
    {
        // 0x600413da sub.w r3,r0,#0x800000
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
        let value = (((t188500 >> 0) & 0xffffffffu64) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188500 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_sub(((t188500 >> 0) & 0xffffffffu64));
        c.w(44, 4, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600413de cmp.w r3,#0x7f000000
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
        // 0x600413e2 bcc 0x60041400
        let mut t180800 = 0u64;
        t180800 = ((c.r(98, 1) == 0u64) as u64) & 0xffu64;
        if ((t180800 >> 0) & 0xffu64) != 0 {
            return 0x60041400u32;
        }
    }
    0x600413e4
}
pub fn b_600413e4(c: &mut Machine) -> u32 {
    {
        // 0x600413e4 ldr r3,[0x60041404]
        let value = c.load(0x60041404, 4);
        c.w(44, 4, value);
    }
    {
        // 0x600413e6 subs r2,r0,#0x1
        let value = (1u64 <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 1u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_sub(1u64);
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
        // 0x600413e8 cmp r2,r3
        let mut t1a6e00 = 0u64;
        let value = (c.r(44, 4) <= c.r(40, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(40, 4), c.r(44, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(40, 4)).wrapping_sub(c.r(44, 4))) & 0xffffffffu64;
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
        // 0x600413ea bls 0x600413fc
        let mut t181400 = 0u64;
        let mut t181600 = 0u64;
        t181400 = ((c.r(98, 1) == 0) as u64) & 0xffu64;
        t181600 = (((((t181400 >> 0) & 0xffu64) != 0) | (c.r(97, 1) != 0)) as u64) & 0xffu64;
        if ((t181600 >> 0) & 0xffu64) != 0 {
            return 0x600413fcu32;
        }
    }
    0x600413ec
}
pub fn b_600413ec(c: &mut Machine) -> u32 {
    {
        // 0x600413ec sub.w r0,r0,#0x7f800000
        let mut t188100 = 0u64;
        let mut t188300 = 0u64;
        t188100 = (2139095040u64) & 0xffffffffu64;
        t188300 = (if 31u64 >= 32 {
            0
        } else {
            ((t188100 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188300 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = (((t188100 >> 0) & 0xffffffffu64) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), ((t188100 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_sub(((t188100 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600413f0 clz r0,r0
        let value = ((c.r(32, 4)).leading_zeros() - 32) as u64;
        c.w(32, 4, value);
    }
    {
        // 0x600413f4 lsrs r0,r0,#0x5
        let mut t1b0b00 = 0u64;
        let mut t1b0c00 = 0u64;
        let mut t1b0e00 = 0u64;
        let mut t1b0f00 = 0u64;
        let mut t1b1000 = 0u64;
        let mut t1b1100 = 0u64;
        let mut t1b1200 = 0u64;
        let mut t1b1300 = 0u64;
        let mut t1b1600 = 0u64;
        t1b1600 = (5u64) & 0xffffffffu64;
        t1b0b00 = (((t1b1600 >> 0) & 0xffffffffu64).wrapping_sub(1u64)) & 0xffffffffu64;
        t1b0c00 = (if ((t1b0b00 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(32, 4)) >> (((t1b0b00 >> 0) & 0xffffffffu64) as u32)
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
        let value = if 5u64 >= 32 {
            0
        } else {
            (c.r(32, 4)) >> (5u64 as u32)
        };
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
    }
    {
        // 0x600413f6 bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x600413f8
}
pub fn b_600413f8(c: &mut Machine) -> u32 {
    {
        // 0x600413f8 movs r0,#0x2
        let value = 2u64;
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
        // 0x600413fa bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x600413fc
}
pub fn b_600413fc(c: &mut Machine) -> u32 {
    {
        // 0x600413fc movs r0,#0x3
        let value = 3u64;
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
        // 0x600413fe bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x60041400
}
pub fn b_60041400(c: &mut Machine) -> u32 {
    {
        // 0x60041400 movs r0,#0x4
        let value = 4u64;
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
        // 0x60041402 bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x60041404
}
pub fn b_60041408(c: &mut Machine) -> u32 {
    {
        // 0x60041408 vmov r2,s0
        let value = c.r(768, 4);
        c.w(40, 4, value);
    }
    {
        // 0x6004140c ubfx r3,r2,#0x17,#0x8
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
            (c.r(40, 4)) << (((t21de00 >> 0) & 0xffffffffu64) as u32)
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
        // 0x60041410 subs r3,#0x7f
        let value = (127u64 <= c.r(44, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), 127u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_sub(127u64);
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
        // 0x60041412 cmp r3,#0x16
        let mut t1a6800 = 0u64;
        let value = (22u64 <= c.r(44, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), 22u64, 4, true);
        c.w(103, 1, value);
        t1a6800 = ((c.r(44, 4)).wrapping_sub(22u64)) & 0xffffffffu64;
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
        // 0x60041414 bic r1,r2,#0x80000000
        let mut t188100 = 0u64;
        let mut t188300 = 0u64;
        let mut t1a4200 = 0u64;
        t188100 = (2147483648u64) & 0xffffffffu64;
        t188300 = (if 31u64 >= 32 {
            0
        } else {
            ((t188100 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188300 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        t1a4200 = (!((t188100 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        let value = (c.r(40, 4) & ((t1a4200 >> 0) & 0xffffffffu64));
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60041418 bgt 0x6004147a
        let mut t181b00 = 0u64;
        let mut t181c00 = 0u64;
        let mut t181e00 = 0u64;
        t181b00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t181c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t181e00 = (((((t181b00 >> 0) & 0xffu64) != 0) & (((t181c00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t181e00 >> 0) & 0xffu64) != 0 {
            return 0x6004147au32;
        }
    }
    0x6004141a
}
pub fn b_6004141a(c: &mut Machine) -> u32 {
    {
        // 0x6004141a cmp r3,#0x0
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
        // 0x6004141c mov r0,r2
        let value = c.r(40, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6004141e blt 0x60041452
        let mut t181a00 = 0u64;
        t181a00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        if ((t181a00 >> 0) & 0xffu64) != 0 {
            return 0x60041452u32;
        }
    }
    0x60041420
}
pub fn b_60041420(c: &mut Machine) -> u32 {
    {
        // 0x60041420 ldr r1,[0x60041488]
        let value = c.load(0x60041488, 4);
        c.w(36, 4, value);
    }
    {
        // 0x60041422 asrs r1,r3
        let mut t1a1a00 = 0u64;
        let mut t1a1b00 = 0u64;
        let mut t1a1d00 = 0u64;
        let mut t1a1e00 = 0u64;
        let mut t1a1f00 = 0u64;
        let mut t1a2000 = 0u64;
        let mut t1a2100 = 0u64;
        let mut t1a2200 = 0u64;
        let mut t1a2700 = 0u64;
        t1a2700 = (c.r(44, 4) & 255u64) & 0xffffffffu64;
        t1a1a00 = (((t1a2700 >> 0) & 0xffffffffu64).wrapping_sub(1u64)) & 0xffffffffu64;
        t1a1b00 = ((signed(c.r(36, 4), 4) >> ((t1a1a00 >> 0) & 0xffffffffu64).min(31) as u32)
            as u64)
            & 0xffffffffu64;
        t1a1d00 = (((t1a1b00 >> 0) & 0xffffffffu64) & 1u64) & 0xffffffffu64;
        t1a1e00 = ((((t1a2700 >> 0) & 0xffffffffu64) == 0u64) as u64) & 0xffu64;
        t1a1f00 = (((((t1a1e00 >> 0) & 0xffu64) != 0) & (c.r(98, 1) != 0)) as u64) & 0xffu64;
        t1a2000 = ((((t1a2700 >> 0) & 0xffffffffu64) != 0u64) as u64) & 0xffu64;
        t1a2100 = ((((t1a1d00 >> 0) & 0xffffffffu64) != 0u64) as u64) & 0xffu64;
        t1a2200 = (((((t1a2000 >> 0) & 0xffu64) != 0) & (((t1a2100 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        let value = ((((t1a1f00 >> 0) & 0xffu64) != 0) | (((t1a2200 >> 0) & 0xffu64) != 0)) as u64;
        c.w(102, 1, value);
        let value =
            (signed(c.r(36, 4), 4) >> ((t1a2700 >> 0) & 0xffffffffu64).min(31) as u32) as u64;
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
        // 0x60041424 tst r2,r1
        let mut t207800 = 0u64;
        t207800 = (c.r(40, 4) & c.r(36, 4)) & 0xffffffffu64;
        let value = (((t207800 >> 0) & 0xffffffffu64) == 0u64) as u64;
        c.w(97, 1, value);
        let value = (signed(((t207800 >> 0) & 0xffffffffu64), 4) < signed(0u64, 4)) as u64;
        c.w(96, 1, value);
    }
    {
        // 0x60041426 beq 0x60041478
        let mut t180200 = 0u64;
        t180200 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        if ((t180200 >> 0) & 0xffu64) != 0 {
            return 0x60041478u32;
        }
    }
    0x60041428
}
pub fn b_60041428(c: &mut Machine) -> u32 {
    {
        // 0x60041428 vldr.32 s15,[pc,#0x60]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610880040u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(96u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6004142c vadd.f32 s15,s0,s15
        let value = (f32::from_bits(c.r(768, 4) as u32) + f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60041430 vcmpe.f32 s15,#0
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
        // 0x60041434 vmrs apsr,fpscr
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
        // 0x60041438 ble 0x60041478
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x60041478u32;
        }
    }
    0x6004143a
}
pub fn b_6004143a(c: &mut Machine) -> u32 {
    {
        // 0x6004143a cmp r2,#0x0
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
        // 0x6004143c ble 0x60041448
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x60041448u32;
        }
    }
    0x6004143e
}
pub fn b_6004143e(c: &mut Machine) -> u32 {
    {
        // 0x6004143e mov.w r2,#0x800000
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
        let value = ((t188500 >> 0) & 0xffffffffu64);
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60041442 asr.w r3,r2,r3
        let mut t1a1a00 = 0u64;
        let mut t1a1b00 = 0u64;
        let mut t1a1d00 = 0u64;
        let mut t1a1e00 = 0u64;
        let mut t1a1f00 = 0u64;
        let mut t1a2000 = 0u64;
        let mut t1a2100 = 0u64;
        let mut t1a2200 = 0u64;
        let mut t1a2a00 = 0u64;
        t1a2a00 = (c.r(44, 4) & 255u64) & 0xffffffffu64;
        t1a1a00 = (((t1a2a00 >> 0) & 0xffffffffu64).wrapping_sub(1u64)) & 0xffffffffu64;
        t1a1b00 = ((signed(c.r(40, 4), 4) >> ((t1a1a00 >> 0) & 0xffffffffu64).min(31) as u32)
            as u64)
            & 0xffffffffu64;
        t1a1d00 = (((t1a1b00 >> 0) & 0xffffffffu64) & 1u64) & 0xffffffffu64;
        t1a1e00 = ((((t1a2a00 >> 0) & 0xffffffffu64) == 0u64) as u64) & 0xffu64;
        t1a1f00 = (((((t1a1e00 >> 0) & 0xffu64) != 0) & (c.r(98, 1) != 0)) as u64) & 0xffu64;
        t1a2000 = ((((t1a2a00 >> 0) & 0xffffffffu64) != 0u64) as u64) & 0xffu64;
        t1a2100 = ((((t1a1d00 >> 0) & 0xffffffffu64) != 0u64) as u64) & 0xffu64;
        t1a2200 = (((((t1a2000 >> 0) & 0xffu64) != 0) & (((t1a2100 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        let value = ((((t1a1f00 >> 0) & 0xffu64) != 0) | (((t1a2200 >> 0) & 0xffu64) != 0)) as u64;
        c.w(102, 1, value);
        let value =
            (signed(c.r(40, 4), 4) >> ((t1a2a00 >> 0) & 0xffffffffu64).min(31) as u32) as u64;
        c.w(44, 4, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60041446 add r0,r3
        let value = (c.r(32, 4)).wrapping_add(c.r(44, 4));
        c.w(32, 4, value);
    }
    0x60041448
}
pub fn b_60041448(c: &mut Machine) -> u32 {
    {
        // 0x60041448 bic.w r3,r0,r1
        let mut t1a4400 = 0u64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        t1a4400 = (!(c.r(36, 4))) & 0xffffffffu64;
        let value = (c.r(32, 4) & ((t1a4400 >> 0) & 0xffffffffu64));
        c.w(44, 4, value);
        let value = c.r(104, 1);
        c.w(102, 1, value);
        let value = c.r(99, 1);
        c.w(103, 1, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6004144c vmov s0,r3
        let value = c.r(44, 4);
        c.w(768, 4, value);
    }
    {
        // 0x60041450 bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x60041452
}
pub fn b_60041452(c: &mut Machine) -> u32 {
    {
        // 0x60041452 vldr.32 s15,[pc,#0x38]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610880082u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(56u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x60041456 vadd.f32 s15,s0,s15
        let value = (f32::from_bits(c.r(768, 4) as u32) + f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6004145a vcmpe.f32 s15,#0
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
        // 0x6004145e vmrs apsr,fpscr
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
        // 0x60041462 ble 0x60041478
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x60041478u32;
        }
    }
    0x60041464
}
pub fn b_60041464(c: &mut Machine) -> u32 {
    {
        // 0x60041464 cmp r2,#0x0
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
        // 0x60041466 blt 0x60041474
        let mut t181a00 = 0u64;
        t181a00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        if ((t181a00 >> 0) & 0xffu64) != 0 {
            return 0x60041474u32;
        }
    }
    0x60041468
}
pub fn b_60041468(c: &mut Machine) -> u32 {
    {
        // 0x60041468 vmov.f32 s15,0x3f800000
        let value = 1065353216u64;
        c.w(828, 4, value);
    }
    {
        // 0x6004146c cmp r1,#0x0
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
        // 0x6004146e vseleq.f32 s0,s0,s15
        let mut tdb600 = 0u64;
        let mut tdc500 = 0u64;
        let mut tdc600 = 0u64;
        let mut tdc700 = 0u64;
        let mut tdc800 = 0u64;
        let mut tdc900 = 0u64;
        let mut tdca00 = 0u64;
        tdb600 = (c.r(97, 1)) & 0xffu64;
        tdc500 = ((((tdb600 >> 0) & 0xffu64) != 0u64) as u64) & 0xffu64;
        tdc600 = ((tdc500 >> 0) & 0xffu64) & 0xffffffffu64;
        tdc700 = (((tdc600 >> 0) & 0xffffffffu64).wrapping_mul(c.r(768, 4))) & 0xffffffffu64;
        tdc800 = ((((tdb600 >> 0) & 0xffu64) == 0u64) as u64) & 0xffu64;
        tdc900 = ((tdc800 >> 0) & 0xffu64) & 0xffffffffu64;
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(828, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(768, 4, value);
    }
    {
        // 0x60041472 bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x60041474
}
pub fn b_60041474(c: &mut Machine) -> u32 {
    {
        // 0x60041474 vldr.32 s0,[pc,#0x18]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610880116u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(24u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    0x60041478
}
pub fn b_60041478(c: &mut Machine) -> u32 {
    {
        // 0x60041478 bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x6004147a
}
pub fn b_6004147a(c: &mut Machine) -> u32 {
    {
        // 0x6004147a cmp.w r1,#0x7f800000
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
        // 0x6004147e bcc 0x60041478
        let mut t180800 = 0u64;
        t180800 = ((c.r(98, 1) == 0u64) as u64) & 0xffu64;
        if ((t180800 >> 0) & 0xffu64) != 0 {
            return 0x60041478u32;
        }
    }
    0x60041480
}
pub fn b_60041480(c: &mut Machine) -> u32 {
    {
        // 0x60041480 vadd.f32 s0,s0,s0
        let value = (f32::from_bits(c.r(768, 4) as u32) + f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x60041484 bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x60041486
}
pub fn b_600414c0(c: &mut Machine) -> u32 {
    {
        // 0x600414c0 ldr r3,[0x600414cc]
        let value = c.load(0x600414cc, 4);
        c.w(44, 4, value);
    }
    {
        // 0x600414c2 mov r1,r0
        let value = c.r(32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x600414c4 ldr r0,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x600414c6 b.w 0x60041524
        return 0x60041524u32;
        return 0xfffffff0u32;
    }
    0x600414ca
}
pub fn b_600414d0(c: &mut Machine) -> u32 {
    {
        // 0x600414d0 ldr r3,[0x600414dc]
        let value = c.load(0x600414dc, 4);
        c.w(44, 4, value);
    }
    {
        // 0x600414d2 mov r1,r0
        let value = c.r(32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x600414d4 ldr r0,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x600414d6 b.w 0x60041a5c
        return 0x60041a5cu32;
        return 0xfffffff0u32;
    }
    0x600414da
}
pub fn b_600414e0(c: &mut Machine) -> u32 {
    {
        // 0x600414e0 push {r4,r5,r6,lr}
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
        // 0x600414e2 ldr r6,[0x60041520]
        let value = c.load(0x60041520, 4);
        c.w(56, 4, value);
    }
    {
        // 0x600414e4 mov r4,r1
        let value = c.r(36, 4);
        c.w(48, 4, value);
    }
    {
        // 0x600414e6 ldr r1,[r6,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x600414e8 mov r5,r0
        let value = c.r(32, 4);
        c.w(52, 4, value);
    }
    {
        // 0x600414ea cbnz r1,0x600414f2
        let mut t1a5900 = 0u64;
        t1a5900 = ((c.r(36, 4) != 0u64) as u64) & 0xffu64;
        if ((t1a5900 >> 0) & 0xffu64) != 0 {
            return 0x600414f2u32;
        }
    }
    0x600414ec
}
pub fn b_600414ec(c: &mut Machine) -> u32 {
    {
        // 0x600414ec bl 0x600419a4
        let value = (1610880240u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600419a4u32;
    }
    0x600414f0
}
pub fn b_600414f0(c: &mut Machine) -> u32 {
    {
        // 0x600414f0 str r0,[r6,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    0x600414f2
}
pub fn b_600414f2(c: &mut Machine) -> u32 {
    {
        // 0x600414f2 mov r1,r4
        let value = c.r(48, 4);
        c.w(36, 4, value);
    }
    {
        // 0x600414f4 mov r0,r5
        let value = c.r(52, 4);
        c.w(32, 4, value);
    }
    {
        // 0x600414f6 bl 0x600419a4
        let value = (1610880250u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600419a4u32;
    }
    0x600414fa
}
pub fn b_600414fa(c: &mut Machine) -> u32 {
    {
        // 0x600414fa adds r3,r0,#0x1
        let value = ((c.r(32, 4) as u128 + 1u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 1u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(1u64);
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
        // 0x600414fc bne 0x60041506
        let mut t180400 = 0u64;
        t180400 = ((c.r(97, 1) == 0u64) as u64) & 0xffu64;
        if ((t180400 >> 0) & 0xffu64) != 0 {
            return 0x60041506u32;
        }
    }
    0x600414fe
}
pub fn b_600414fe(c: &mut Machine) -> u32 {
    {
        // 0x600414fe mov.w r4,#0xffffffff
        let mut t188000 = 0u64;
        t188000 = (4294967295u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = ((t188000 >> 0) & 0xffffffffu64);
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    0x60041502
}
pub fn b_60041502(c: &mut Machine) -> u32 {
    {
        // 0x60041502 mov r0,r4
        let value = c.r(48, 4);
        c.w(32, 4, value);
    }
    {
        // 0x60041504 pop {r4,r5,r6,pc}
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
    0x60041506
}
pub fn b_60041506(c: &mut Machine) -> u32 {
    {
        // 0x60041506 adds r4,r0,#0x3
        let value = ((c.r(32, 4) as u128 + 3u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 3u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(3u64);
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
        // 0x60041508 bic r4,r4,#0x3
        let mut t187d00 = 0u64;
        let mut t1a4200 = 0u64;
        t187d00 = (3u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        t1a4200 = (!((t187d00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        let value = (c.r(48, 4) & ((t1a4200 >> 0) & 0xffffffffu64));
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6004150c cmp r0,r4
        let mut t1a6e00 = 0u64;
        let value = (c.r(48, 4) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), c.r(48, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(32, 4)).wrapping_sub(c.r(48, 4))) & 0xffffffffu64;
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
        // 0x6004150e beq 0x60041502
        let mut t180200 = 0u64;
        t180200 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        if ((t180200 >> 0) & 0xffu64) != 0 {
            return 0x60041502u32;
        }
    }
    0x60041510
}
pub fn b_60041510(c: &mut Machine) -> u32 {
    {
        // 0x60041510 subs r1,r4,r0
        let value = (c.r(32, 4) <= c.r(48, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), c.r(32, 4), 4, true);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_sub(c.r(32, 4));
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
        // 0x60041512 mov r0,r5
        let value = c.r(52, 4);
        c.w(32, 4, value);
    }
    {
        // 0x60041514 bl 0x600419a4
        let value = (1610880280u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600419a4u32;
    }
    0x60041518
}
pub fn b_60041518(c: &mut Machine) -> u32 {
    {
        // 0x60041518 adds r0,#0x1
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
        // 0x6004151a bne 0x60041502
        let mut t180400 = 0u64;
        t180400 = ((c.r(97, 1) == 0u64) as u64) & 0xffu64;
        if ((t180400 >> 0) & 0xffu64) != 0 {
            return 0x60041502u32;
        }
    }
    0x6004151c
}
pub fn b_6004151c(c: &mut Machine) -> u32 {
    {
        // 0x6004151c b 0x600414fe
        return 0x600414feu32;
    }
    0x6004151e
}
pub fn b_60041524(c: &mut Machine) -> u32 {
    {
        // 0x60041524 push {r3,r4,r5,r6,r7,r8,r9,lr}
        let value = (c.r(84, 4)).wrapping_sub(4u64);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 4, c.r(88, 4));
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
        c.store(c.r(128, 4) as u32, 4, c.r(44, 4));
        let value = (c.r(128, 4)).wrapping_sub(4u64);
        c.w(128, 4, value);
        let value = (c.r(128, 4)).wrapping_add(4u64);
        c.w(84, 4, value);
    }
    {
        // 0x60041528 adds r5,r1,#0x3
        let value = ((c.r(36, 4) as u128 + 3u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 3u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_add(3u64);
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
        // 0x6004152a bic r5,r5,#0x3
        let mut t187d00 = 0u64;
        let mut t1a4200 = 0u64;
        t187d00 = (3u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        t1a4200 = (!((t187d00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        let value = (c.r(52, 4) & ((t1a4200 >> 0) & 0xffffffffu64));
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6004152e adds r5,#0x8
        let value = ((c.r(52, 4) as u128 + 8u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(52, 4), 8u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(52, 4)).wrapping_add(8u64);
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
        // 0x60041530 cmp r5,#0xc
        let mut t1a6800 = 0u64;
        let value = (12u64 <= c.r(52, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(52, 4), 12u64, 4, true);
        c.w(103, 1, value);
        t1a6800 = ((c.r(52, 4)).wrapping_sub(12u64)) & 0xffffffffu64;
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
        // 0x60041532 it cc
        let mut t184a00 = 0u64;
        t184a00 = ((c.r(98, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x60041534 mov.cc r5,#0xc
        let mut tb00 = 0u64;
        let mut t2700 = 0u64;
        tb00 = ((c.r(98, 1) == 0u64) as u64) & 0xffu64;
        t2700 = ((((tb00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x60041536u32;
        }
        let value = 12u64;
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    0x60041536
}
pub fn b_60041536(c: &mut Machine) -> u32 {
    {
        // 0x60041536 cmp r5,#0x0
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
        // 0x60041538 mov r6,r0
        let value = c.r(32, 4);
        c.w(56, 4, value);
    }
    {
        // 0x6004153a blt 0x60041540
        let mut t181a00 = 0u64;
        t181a00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        if ((t181a00 >> 0) & 0xffu64) != 0 {
            return 0x60041540u32;
        }
    }
    0x6004153c
}
pub fn b_6004153c(c: &mut Machine) -> u32 {
    {
        // 0x6004153c cmp r1,r5
        let mut t1a6e00 = 0u64;
        let value = (c.r(52, 4) <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), c.r(52, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(36, 4)).wrapping_sub(c.r(52, 4))) & 0xffffffffu64;
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
        // 0x6004153e bls 0x6004154a
        let mut t181400 = 0u64;
        let mut t181600 = 0u64;
        t181400 = ((c.r(98, 1) == 0) as u64) & 0xffu64;
        t181600 = (((((t181400 >> 0) & 0xffu64) != 0) | (c.r(97, 1) != 0)) as u64) & 0xffu64;
        if ((t181600 >> 0) & 0xffu64) != 0 {
            return 0x6004154au32;
        }
    }
    0x60041540
}
pub fn b_60041540(c: &mut Machine) -> u32 {
    {
        // 0x60041540 movs r3,#0xc
        let value = 12u64;
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
        // 0x60041542 str r3,[r6,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    0x60041544
}
pub fn b_60041544(c: &mut Machine) -> u32 {
    {
        // 0x60041544 movs r0,#0x0
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
    0x60041546
}
pub fn b_60041546(c: &mut Machine) -> u32 {
    {
        // 0x60041546 pop.w {r3,r4,r5,r6,r7,r8,r9,pc}
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
        c.w(64, 4, value);
        let value = (c.r(128, 4)).wrapping_add(4u64);
        c.w(128, 4, value);
        let value = c.load(c.r(128, 4) as u32, 4);
        c.w(68, 4, value);
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
    0x6004154a
}
pub fn b_6004154a(c: &mut Machine) -> u32 {
    {
        // 0x6004154a ldr.w r8,[0x60041620]
        let value = c.load(0x60041620, 4);
        c.w(64, 4, value);
    }
    {
        // 0x6004154e bl 0x6004168c
        let value = (1610880338u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x6004168cu32;
    }
    0x60041552
}
pub fn b_60041552(c: &mut Machine) -> u32 {
    {
        // 0x60041552 ldr.w r3,[r8,#0x0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(64, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x60041556 mov r4,r3
        let value = c.r(44, 4);
        c.w(48, 4, value);
    }
    0x60041558
}
pub fn b_60041558(c: &mut Machine) -> u32 {
    {
        // 0x60041558 cbnz r4,0x600415ac
        let mut t1a5900 = 0u64;
        t1a5900 = ((c.r(48, 4) != 0u64) as u64) & 0xffu64;
        if ((t1a5900 >> 0) & 0xffu64) != 0 {
            return 0x600415acu32;
        }
    }
    0x6004155a
}
pub fn b_6004155a(c: &mut Machine) -> u32 {
    {
        // 0x6004155a mov r1,r5
        let value = c.r(52, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6004155c mov r0,r6
        let value = c.r(56, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6004155e bl 0x600414e0
        let value = (1610880354u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414e0u32;
    }
    0x60041562
}
pub fn b_60041562(c: &mut Machine) -> u32 {
    {
        // 0x60041562 adds r3,r0,#0x1
        let value = ((c.r(32, 4) as u128 + 1u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), 1u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_add(1u64);
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
        // 0x60041564 mov r4,r0
        let value = c.r(32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x60041566 bne 0x6004161a
        let mut t180400 = 0u64;
        t180400 = ((c.r(97, 1) == 0u64) as u64) & 0xffu64;
        if ((t180400 >> 0) & 0xffu64) != 0 {
            return 0x6004161au32;
        }
    }
    0x60041568
}
pub fn b_60041568(c: &mut Machine) -> u32 {
    {
        // 0x60041568 ldr.w r4,[r8,#0x0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(64, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6004156c mov r7,r4
        let value = c.r(48, 4);
        c.w(60, 4, value);
    }
    0x6004156e
}
pub fn b_6004156e(c: &mut Machine) -> u32 {
    {
        // 0x6004156e cmp r7,#0x0
        let mut t1a6800 = 0u64;
        let value = (0u64 <= c.r(60, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(60, 4), 0u64, 4, true);
        c.w(103, 1, value);
        t1a6800 = ((c.r(60, 4)).wrapping_sub(0u64)) & 0xffffffffu64;
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
        // 0x60041570 bne 0x600415fa
        let mut t180400 = 0u64;
        t180400 = ((c.r(97, 1) == 0u64) as u64) & 0xffu64;
        if ((t180400 >> 0) & 0xffu64) != 0 {
            return 0x600415fau32;
        }
    }
    0x60041572
}
pub fn b_60041572(c: &mut Machine) -> u32 {
    {
        // 0x60041572 cmp r4,#0x0
        let mut t1a6800 = 0u64;
        let value = (0u64 <= c.r(48, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 0u64, 4, true);
        c.w(103, 1, value);
        t1a6800 = ((c.r(48, 4)).wrapping_sub(0u64)) & 0xffffffffu64;
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
        // 0x60041574 beq 0x6004160e
        let mut t180200 = 0u64;
        t180200 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        if ((t180200 >> 0) & 0xffu64) != 0 {
            return 0x6004160eu32;
        }
    }
    0x60041576
}
pub fn b_60041576(c: &mut Machine) -> u32 {
    {
        // 0x60041576 ldr r3,[r4,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x60041578 mov r1,r7
        let value = c.r(60, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6004157a mov r0,r6
        let value = c.r(56, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6004157c add.w r9,r4,r3
        let mut t1a0b00 = 0u64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        t1a0b00 = (c.r(44, 4)) & 0xffffffffu64;
        let value = ((c.r(48, 4) as u128 + ((t1a0b00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t1a0b00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(((t1a0b00 >> 0) & 0xffffffffu64));
        c.w(68, 4, value);
        let value = (signed(c.r(68, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(68, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60041580 bl 0x600419a4
        let value = (1610880388u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600419a4u32;
    }
    0x60041584
}
pub fn b_60041584(c: &mut Machine) -> u32 {
    {
        // 0x60041584 cmp r9,r0
        let mut t1a7000 = 0u64;
        let value = (c.r(32, 4) <= c.r(68, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(68, 4), c.r(32, 4), 4, true);
        c.w(103, 1, value);
        t1a7000 = ((c.r(68, 4)).wrapping_sub(c.r(32, 4))) & 0xffffffffu64;
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
        // 0x60041586 bne 0x6004160e
        let mut t180400 = 0u64;
        t180400 = ((c.r(97, 1) == 0u64) as u64) & 0xffu64;
        if ((t180400 >> 0) & 0xffu64) != 0 {
            return 0x6004160eu32;
        }
    }
    0x60041588
}
pub fn b_60041588(c: &mut Machine) -> u32 {
    {
        // 0x60041588 ldr r1,[r4,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6004158a subs r5,r5,r1
        let value = (c.r(36, 4) <= c.r(52, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(52, 4), c.r(36, 4), 4, true);
        c.w(103, 1, value);
        let value = (c.r(52, 4)).wrapping_sub(c.r(36, 4));
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
        // 0x6004158c mov r1,r5
        let value = c.r(52, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6004158e mov r0,r6
        let value = c.r(56, 4);
        c.w(32, 4, value);
    }
    {
        // 0x60041590 bl 0x600414e0
        let value = (1610880404u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414e0u32;
    }
    0x60041594
}
pub fn b_60041594(c: &mut Machine) -> u32 {
    {
        // 0x60041594 adds r0,#0x1
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
        // 0x60041596 beq 0x6004160e
        let mut t180200 = 0u64;
        t180200 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        if ((t180200 >> 0) & 0xffu64) != 0 {
            return 0x6004160eu32;
        }
    }
    0x60041598
}
pub fn b_60041598(c: &mut Machine) -> u32 {
    {
        // 0x60041598 ldr r3,[r4,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6004159a add r3,r5
        let value = (c.r(44, 4)).wrapping_add(c.r(52, 4));
        c.w(44, 4, value);
    }
    {
        // 0x6004159c str r3,[r4,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6004159e ldr.w r3,[r8,#0x0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(64, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x600415a2 ldr r2,[r3,#0x4]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x600415a4 cbnz r2,0x60041600
        let mut t1a5900 = 0u64;
        t1a5900 = ((c.r(40, 4) != 0u64) as u64) & 0xffu64;
        if ((t1a5900 >> 0) & 0xffu64) != 0 {
            return 0x60041600u32;
        }
    }
    0x600415a6
}
pub fn b_600415a6(c: &mut Machine) -> u32 {
    {
        // 0x600415a6 str.w r7,[r8,#0x0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(64, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(60, 4));
    }
    {
        // 0x600415aa b 0x600415cc
        return 0x600415ccu32;
    }
    0x600415ac
}
pub fn b_600415ac(c: &mut Machine) -> u32 {
    {
        // 0x600415ac ldr r2,[r4,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x600415ae subs r2,r2,r5
        let value = (c.r(52, 4) <= c.r(40, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(40, 4), c.r(52, 4), 4, true);
        c.w(103, 1, value);
        let value = (c.r(40, 4)).wrapping_sub(c.r(52, 4));
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
        // 0x600415b0 bmi 0x600415f4
        let mut t180a00 = 0u64;
        t180a00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        if ((t180a00 >> 0) & 0xffu64) != 0 {
            return 0x600415f4u32;
        }
    }
    0x600415b2
}
pub fn b_600415b2(c: &mut Machine) -> u32 {
    {
        // 0x600415b2 cmp r2,#0xb
        let mut t1a6800 = 0u64;
        let value = (11u64 <= c.r(40, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(40, 4), 11u64, 4, true);
        c.w(103, 1, value);
        t1a6800 = ((c.r(40, 4)).wrapping_sub(11u64)) & 0xffffffffu64;
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
        // 0x600415b4 bls 0x600415e6
        let mut t181400 = 0u64;
        let mut t181600 = 0u64;
        t181400 = ((c.r(98, 1) == 0) as u64) & 0xffu64;
        t181600 = (((((t181400 >> 0) & 0xffu64) != 0) | (c.r(97, 1) != 0)) as u64) & 0xffu64;
        if ((t181600 >> 0) & 0xffu64) != 0 {
            return 0x600415e6u32;
        }
    }
    0x600415b6
}
pub fn b_600415b6(c: &mut Machine) -> u32 {
    {
        // 0x600415b6 adds r1,r4,r5
        let value = ((c.r(48, 4) as u128 + c.r(52, 4) as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), c.r(52, 4), 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(c.r(52, 4));
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
        // 0x600415b8 cmp r3,r4
        let mut t1a6e00 = 0u64;
        let value = (c.r(48, 4) <= c.r(44, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), c.r(48, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(44, 4)).wrapping_sub(c.r(48, 4))) & 0xffffffffu64;
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
        // 0x600415ba str r5,[r4,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x600415bc it ne
        let mut t184600 = 0u64;
        t184600 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x600415be str.ne r1,[r3,#0x4]
        let mut t700 = 0u64;
        let mut t2700 = 0u64;
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        t700 = ((c.r(97, 1) == 0u64) as u64) & 0xffu64;
        t2700 = ((((t700 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x600415c0u32;
        }
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    0x600415c0
}
pub fn b_600415c0(c: &mut Machine) -> u32 {
    {
        // 0x600415c0 ldr r3,[r4,#0x4]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x600415c2 it eq
        let mut t184400 = 0u64;
        t184400 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x600415c4 str.w.eq r1,[r8,#0x0]
        let mut t500 = 0u64;
        let mut t2700 = 0u64;
        let mut t19c800 = 0u64;
        t500 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((t500 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x600415c8u32;
        }
        t19c800 = ((c.r(64, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    0x600415c8
}
pub fn b_600415c8(c: &mut Machine) -> u32 {
    {
        // 0x600415c8 str r2,[r4,r5]
        let mut t19e600 = 0u64;
        t19e600 = ((c.r(48, 4)).wrapping_add(c.r(52, 4))) & 0xffffffffu64;
        c.store(((t19e600 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x600415ca str r3,[r1,#0x4]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(36, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    0x600415cc
}
pub fn b_600415cc(c: &mut Machine) -> u32 {
    {
        // 0x600415cc mov r0,r6
        let value = c.r(56, 4);
        c.w(32, 4, value);
    }
    {
        // 0x600415ce bl 0x60041698
        let value = (1610880466u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041698u32;
    }
    0x600415d2
}
pub fn b_600415d2(c: &mut Machine) -> u32 {
    {
        // 0x600415d2 add.w r0,r4,#0xb
        let mut t187d00 = 0u64;
        t187d00 = (11u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = ((c.r(48, 4) as u128 + ((t187d00 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(((t187d00 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600415d6 adds r3,r4,#0x4
        let value = ((c.r(48, 4) as u128 + 4u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 4u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(4u64);
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
        // 0x600415d8 bic r0,r0,#0x7
        let mut t187d00 = 0u64;
        let mut t1a4200 = 0u64;
        t187d00 = (7u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        t1a4200 = (!((t187d00 >> 0) & 0xffffffffu64)) & 0xffffffffu64;
        let value = (c.r(32, 4) & ((t1a4200 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x600415dc subs r2,r0,r3
        let value = (c.r(44, 4) <= c.r(32, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(32, 4), c.r(44, 4), 4, true);
        c.w(103, 1, value);
        let value = (c.r(32, 4)).wrapping_sub(c.r(44, 4));
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
        // 0x600415de itt ne
        let mut t184600 = 0u64;
        t184600 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x600415e0 sub.ne r3,r3,r0
        let mut t700 = 0u64;
        let mut t2600 = 0u64;
        t700 = ((c.r(97, 1) == 0u64) as u64) & 0xffu64;
        t2600 = ((((t700 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2600 >> 0) & 0xffu64) != 0 {
            return 0x600415e2u32;
        }
        let value = (c.r(32, 4) <= c.r(44, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), c.r(32, 4), 4, true);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_sub(c.r(32, 4));
        c.w(44, 4, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    0x600415e2
}
pub fn b_600415e2(c: &mut Machine) -> u32 {
    {
        // 0x600415e2 str.ne r3,[r4,r2]
        let mut t700 = 0u64;
        let mut t2700 = 0u64;
        let mut t19e600 = 0u64;
        t19e600 = ((c.r(48, 4)).wrapping_add(c.r(40, 4))) & 0xffffffffu64;
        t700 = ((c.r(97, 1) == 0u64) as u64) & 0xffu64;
        t2700 = ((((t700 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x600415e4u32;
        }
        c.store(((t19e600 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    0x600415e4
}
pub fn b_600415e4(c: &mut Machine) -> u32 {
    {
        // 0x600415e4 b 0x60041546
        return 0x60041546u32;
    }
    0x600415e6
}
pub fn b_600415e6(c: &mut Machine) -> u32 {
    {
        // 0x600415e6 ldr r2,[r4,#0x4]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x600415e8 cmp r3,r4
        let mut t1a6e00 = 0u64;
        let value = (c.r(48, 4) <= c.r(44, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), c.r(48, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(44, 4)).wrapping_sub(c.r(48, 4))) & 0xffffffffu64;
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
        // 0x600415ea ite eq
        let mut t184400 = 0u64;
        t184400 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x600415ec str.w.eq r2,[r8,#0x0]
        let mut t500 = 0u64;
        let mut t2600 = 0u64;
        let mut t19c800 = 0u64;
        t500 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        t2600 = ((((t500 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2600 >> 0) & 0xffu64) != 0 {
            return 0x600415f0u32;
        }
        t19c800 = ((c.r(64, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    0x600415f0
}
pub fn b_600415f0(c: &mut Machine) -> u32 {
    {
        // 0x600415f0 str.ne r2,[r3,#0x4]
        let mut t700 = 0u64;
        let mut t2700 = 0u64;
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        t700 = ((c.r(97, 1) == 0u64) as u64) & 0xffu64;
        t2700 = ((((t700 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x600415f2u32;
        }
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    0x600415f2
}
pub fn b_600415f2(c: &mut Machine) -> u32 {
    {
        // 0x600415f2 b 0x600415cc
        return 0x600415ccu32;
    }
    0x600415f4
}
pub fn b_600415f4(c: &mut Machine) -> u32 {
    {
        // 0x600415f4 mov r3,r4
        let value = c.r(48, 4);
        c.w(44, 4, value);
    }
    {
        // 0x600415f6 ldr r4,[r4,#0x4]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x600415f8 b 0x60041558
        return 0x60041558u32;
    }
    0x600415fa
}
pub fn b_600415fa(c: &mut Machine) -> u32 {
    {
        // 0x600415fa mov r4,r7
        let value = c.r(60, 4);
        c.w(48, 4, value);
    }
    {
        // 0x600415fc ldr r7,[r7,#0x4]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(60, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(60, 4, value);
    }
    {
        // 0x600415fe b 0x6004156e
        return 0x6004156eu32;
    }
    0x60041600
}
pub fn b_60041600(c: &mut Machine) -> u32 {
    {
        // 0x60041600 mov r2,r3
        let value = c.r(44, 4);
        c.w(40, 4, value);
    }
    {
        // 0x60041602 ldr r3,[r3,#0x4]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x60041604 cmp r3,r4
        let mut t1a6e00 = 0u64;
        let value = (c.r(48, 4) <= c.r(44, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), c.r(48, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(44, 4)).wrapping_sub(c.r(48, 4))) & 0xffffffffu64;
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
        // 0x60041606 bne 0x60041600
        let mut t180400 = 0u64;
        t180400 = ((c.r(97, 1) == 0u64) as u64) & 0xffu64;
        if ((t180400 >> 0) & 0xffu64) != 0 {
            return 0x60041600u32;
        }
    }
    0x60041608
}
pub fn b_60041608(c: &mut Machine) -> u32 {
    {
        // 0x60041608 movs r3,#0x0
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
        // 0x6004160a str r3,[r2,#0x4]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(40, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6004160c b 0x600415cc
        return 0x600415ccu32;
    }
    0x6004160e
}
pub fn b_6004160e(c: &mut Machine) -> u32 {
    {
        // 0x6004160e movs r3,#0xc
        let value = 12u64;
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
        // 0x60041610 str r3,[r6,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x60041612 mov r0,r6
        let value = c.r(56, 4);
        c.w(32, 4, value);
    }
    {
        // 0x60041614 bl 0x60041698
        let value = (1610880536u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041698u32;
    }
    0x60041618
}
pub fn b_60041618(c: &mut Machine) -> u32 {
    {
        // 0x60041618 b 0x60041544
        return 0x60041544u32;
    }
    0x6004161a
}
pub fn b_6004161a(c: &mut Machine) -> u32 {
    {
        // 0x6004161a str r5,[r0,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(32, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6004161c b 0x600415cc
        return 0x600415ccu32;
    }
    0x6004161e
}
pub fn b_6004168c(c: &mut Machine) -> u32 {
    {
        // 0x6004168c ldr r0,[0x60041694]
        let value = c.load(0x60041694, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6004168e b.w 0x60041a1a
        return 0x60041a1au32;
        return 0xfffffff0u32;
    }
    0x60041692
}
