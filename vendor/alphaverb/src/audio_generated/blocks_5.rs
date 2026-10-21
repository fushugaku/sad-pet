#![allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut,
    unused_comparisons
)]
use crate::machine::{signed, signed_overflow, Machine};
pub fn b_60029ef6(c: &mut Machine) -> u32 {
    {
        // 0x60029ef6 vdiv.f32 s17,s11,s16
        let value = (f32::from_bits(c.r(812, 4) as u32) / f32::from_bits(c.r(832, 4) as u32))
            .to_bits() as u64;
        c.w(836, 4, value);
    }
    {
        // 0x60029efa vldr.32 s9,[pc,#0x128]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610784506u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(296u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(804, 4, value);
    }
    {
        // 0x60029efe vldr.32 s13,[pc,#0x128]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610784510u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(296u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x60029f02 vldr.32 s8,[pc,#0x128]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610784514u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(296u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x60029f06 vstr.32 s12,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x60029f0a vmov.f32 s14,0x3f800000
        let value = 1065353216u64;
        c.w(824, 4, value);
    }
    {
        // 0x60029f0e vmov.f32 s11,0x40000000
        let value = 1073741824u64;
        c.w(812, 4, value);
    }
    {
        // 0x60029f12 vmov.f32 s10,0x41480000
        let value = 1095237632u64;
        c.w(808, 4, value);
    }
    {
        // 0x60029f16 vmov r3,s17
        let value = c.r(836, 4);
        c.w(44, 4, value);
    }
    {
        // 0x60029f1a bic r2,r3,#0xc0000000
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
        // 0x60029f1e ubfx r3,r3,#0x17,#0x8
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
        // 0x60029f22 orr r2,r2,#0x3f800000
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
        // 0x60029f26 vmov s0,r3
        let value = c.r(44, 4);
        c.w(768, 4, value);
    }
    {
        // 0x60029f2a vmov s15,r2
        let value = c.r(40, 4);
        c.w(828, 4, value);
    }
    {
        // 0x60029f2e vcvt.f32.s32 s0,s0
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
        // 0x60029f32 vsub.f32 s15,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029f36 vsub.f32 s0,s0,s8
        let value = (f32::from_bits(c.r(768, 4) as u32) - f32::from_bits(c.r(800, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x60029f3a vfma.f32 s13,s15,s9
        let value = f32::from_bits(c.r(828, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(820, 4) as u32),
            )
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x60029f3e vsub.f32 s12,s11,s15
        let value = (f32::from_bits(c.r(812, 4) as u32) - f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x60029f42 vfma.f32 s0,s15,s13
        let value = f32::from_bits(c.r(828, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(820, 4) as u32),
                f32::from_bits(c.r(768, 4) as u32),
            )
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x60029f46 vsub.f32 s14,s14,s13
        let value = (f32::from_bits(c.r(824, 4) as u32) - f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x60029f4a vmul.f32 s14,s14,s15
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x60029f4e vfma.f32 s0,s14,s12
        let value = f32::from_bits(c.r(824, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(816, 4) as u32),
                f32::from_bits(c.r(768, 4) as u32),
            )
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x60029f52 vmul.f32 s0,s0,s10
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x60029f56 bl 0x6004137c
        let value = (1610784602u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x6004137cu32;
    }
    0x60029f5a
}
pub fn b_60029f5a(c: &mut Machine) -> u32 {
    {
        // 0x60029f5a vmov.f32 s15,0x3e000000
        let value = 1040187392u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029f5e vsub.f32 s16,s16,s18
        let value = (f32::from_bits(c.r(832, 4) as u32) - f32::from_bits(c.r(840, 4) as u32))
            .to_bits() as u64;
        c.w(832, 4, value);
    }
    {
        // 0x60029f62 vmov.f32 s14,0x3f800000
        let value = 1065353216u64;
        c.w(824, 4, value);
    }
    {
        // 0x60029f66 vsub.f32 s0,s15,s0
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x60029f6a vcmpe.f32 s0,#0
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
        // 0x60029f6e vmrs apsr,fpscr
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
        // 0x60029f72 it le
        let mut t186100 = 0u64;
        let mut t186200 = 0u64;
        let mut t186400 = 0u64;
        t186100 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t186200 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t186400 = (((((t186100 >> 0) & 0xffu64) != 0) & (((t186200 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
    }
    {
        // 0x60029f74 vldr.le.32 s0,[pc,#0xa8]
        let mut t2200 = 0u64;
        let mut t2400 = 0u64;
        let mut t2700 = 0u64;
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610784628u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(168u64)) & 0xffffffffu64;
        t2200 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2400 = (((c.r(97, 1) != 0) | (((t2200 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        t2700 = ((((t2400 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x60029f78u32;
        }
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    0x60029f78
}
pub fn b_60029f78(c: &mut Machine) -> u32 {
    {
        // 0x60029f78 vfma.f32 s14,s16,s0
        let value = f32::from_bits(c.r(832, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(768, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x60029f7c vsub.f32 s14,s14,s17
        let value = (f32::from_bits(c.r(824, 4) as u32) - f32::from_bits(c.r(836, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x60029f80 vpop {d8,d9}
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
        // 0x60029f84 vcvt.f64.f32 d7,s14
        let value = (f32::from_bits(c.r(824, 4) as u32) as f64).to_bits() as u64;
        c.w(824, 8, value);
    }
    {
        // 0x60029f88 vstr.64 d7,[r4,#0x10]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 8, c.r(824, 8));
    }
    {
        // 0x60029f8c pop {r4,r5,r6,pc}
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
    0x60029f8e
}
pub fn b_60029f8e(c: &mut Machine) -> u32 {
    {
        // 0x60029f8e subs r3,r3,r2
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
        // 0x60029f90 ands r6,r3
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
        // 0x60029f92 str r3,[r4,#0x44]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x60029f94 add.w r5,r5,r6, lsl #0x2
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
        // 0x60029f98 vldr.32 s11,[r5]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(52, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x60029f9c b 0x60029e86
        return 0x60029e86u32;
    }
    0x60029f9e
}
pub fn b_60029f9e(c: &mut Machine) -> u32 {
    {
        // 0x60029f9e subs r3,r3,r2
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
        // 0x60029fa0 cmp r2,#0x0
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
        // 0x60029fa2 and.w r5,r6,r3
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (c.r(56, 4) & c.r(44, 4));
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x60029fa6 str r3,[r4,#0x7c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(124u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x60029fa8 add.w r5,r0,r5, lsl #0x2
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
        // 0x60029fac vldr.32 s13,[r5]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(52, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x60029fb0 ble 0x6002a014
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x6002a014u32;
        }
    }
    0x60029fb2
}
pub fn b_60029fb2(c: &mut Machine) -> u32 {
    {
        // 0x60029fb2 adds r2,r3,#0x1
        let value = ((c.r(44, 4) as u128 + 1u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), 1u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_add(1u64);
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
        // 0x60029fb4 ands r2,r6
        let value = (c.r(40, 4) & c.r(56, 4));
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
        // 0x60029fb6 add.w r2,r0,r2, lsl #0x2
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
        // 0x60029fba vldr.32 s12,[r2]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(40, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    0x60029fbe
}
pub fn b_60029fbe(c: &mut Machine) -> u32 {
    {
        // 0x60029fbe subs r2,r3,#0x1
        let value = (1u64 <= c.r(44, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), 1u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_sub(1u64);
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
        // 0x60029fc0 vmov.f32 s9,0x3f000000
        let value = 1056964608u64;
        c.w(804, 4, value);
    }
    {
        // 0x60029fc4 subs r3,#0x2
        let value = (2u64 <= c.r(44, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), 2u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(44, 4)).wrapping_sub(2u64);
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
        // 0x60029fc6 vldr.32 s10,[r4,#0x68]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(104u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x60029fca ands r2,r6
        let value = (c.r(40, 4) & c.r(56, 4));
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
        // 0x60029fcc ands r3,r6
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
        // 0x60029fce add.w r2,r0,r2, lsl #0x2
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
        // 0x60029fd2 add.w r0,r0,r3, lsl #0x2
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
        // 0x60029fd6 vldr.32 s14,[r2]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(40, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x60029fda vldr.32 s8,[r0]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(32, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x60029fde vsub.f32 s12,s14,s12
        let value = (f32::from_bits(c.r(824, 4) as u32) - f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x60029fe2 vsub.f32 s14,s13,s14
        let value = (f32::from_bits(c.r(820, 4) as u32) - f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x60029fe6 vsub.f32 s8,s8,s13
        let value = (f32::from_bits(c.r(800, 4) as u32) - f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(800, 4, value);
    }
    {
        // 0x60029fea vmov.f32 s15,s14
        let value = c.r(824, 4);
        c.w(828, 4, value);
    }
    {
        // 0x60029fee vfma.f32 s15,s12,s9
        let value = f32::from_bits(c.r(816, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029ff2 vadd.f32 s14,s14,s15
        let value = (f32::from_bits(c.r(824, 4) as u32) + f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x60029ff6 vfma.f32 s14,s8,s9
        let value = f32::from_bits(c.r(800, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x60029ffa vadd.f32 s15,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x60029ffe vfnms.f32 s15,s10,s14
        let mut t49a00 = 0u64;
        let mut t49b00 = 0u64;
        t49a00 = ((-f32::from_bits(c.r(828, 4) as u32)).to_bits() as u64) & 0xffffffffu64;
        t49b00 = ((f32::from_bits(c.r(808, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64)
            & 0xffffffffu64;
        let value = (f32::from_bits(((t49a00 >> 0) & 0xffffffffu64) as u32)
            + f32::from_bits(((t49b00 >> 0) & 0xffffffffu64) as u32))
        .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002a002 vmul.f32 s15,s10,s15
        let value = (f32::from_bits(c.r(808, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002a006 vfma.f32 s15,s12,s9
        let value = f32::from_bits(c.r(816, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002a00a vfma.f32 s13,s10,s15
        let value = f32::from_bits(c.r(808, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(820, 4) as u32),
            )
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002a00e vmov.f32 s12,s13
        let value = c.r(820, 4);
        c.w(816, 4, value);
    }
    {
        // 0x6002a012 b 0x60029ef6
        return 0x60029ef6u32;
    }
    0x6002a014
}
pub fn b_6002a014(c: &mut Machine) -> u32 {
    {
        // 0x6002a014 vmov.f32 s12,s13
        let value = c.r(820, 4);
        c.w(816, 4, value);
    }
    {
        // 0x6002a018 b 0x60029fbe
        return 0x60029fbeu32;
    }
    0x6002a01a
}
pub fn b_6002a030(c: &mut Machine) -> u32 {
    {
        // 0x6002a030 vmov.f32 s15,0xc1900000
        let value = 3247439872u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002a034 vldr.32 s13,[pc,#0xec]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610784820u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(236u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002a038 vldr.32 s14,[pc,#0xec]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610784824u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(236u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002a03c add.w r0,r0,#0x1000
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
        // 0x6002a040 vldr.32 s6,[pc,#0xe8]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610784832u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(232u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x6002a044 vmov.f32 s10,0x3f800000
        let value = 1065353216u64;
        c.w(808, 4, value);
    }
    {
        // 0x6002a048 vfma.f32 s15,s0,s13
        let value = f32::from_bits(c.r(768, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(820, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002a04c vldr.32 s11,[pc,#0xe0]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610784844u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(224u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x6002a050 vldr.32 s12,[pc,#0xe0]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610784848u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(224u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x6002a054 vldr.32 s13,[pc,#0xe0]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610784852u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(224u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002a058 vldr.32 s7,[r0,#0x1cc]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(460u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x6002a05c vldr.32 s8,[r0,#0x1c8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(456u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x6002a060 vldr.32 s9,[pc,#0xd8]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610784864u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(216u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(804, 4, value);
    }
    {
        // 0x6002a064 vmul.f32 s14,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002a068 vstr.32 s15,[r0,#0x1d0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(464u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002a06c vcvt.s32.f32 s15,s14
        let value = (f32::from_bits(c.r(824, 4) as u32) as i32) as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002a070 vcvt.f32.s32 s5,s15
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
        c.w(788, 4, value);
    }
    {
        // 0x6002a074 vmov r3,s15
        let value = c.r(828, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002a078 vmov.f32 s15,s12
        let value = c.r(816, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002a07c adds r3,#0x7f
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
        // 0x6002a07e vsub.f32 s14,s14,s5
        let value = (f32::from_bits(c.r(824, 4) as u32) - f32::from_bits(c.r(788, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002a082 lsls r3,r3,#0x17
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
        // 0x6002a084 vfma.f32 s11,s14,s6
        let value = f32::from_bits(c.r(824, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(792, 4) as u32),
                f32::from_bits(c.r(812, 4) as u32),
            )
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x6002a088 vfma.f32 s15,s14,s11
        let value = f32::from_bits(c.r(824, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(812, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002a08c vfma.f32 s13,s14,s15
        let value = f32::from_bits(c.r(824, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(828, 4) as u32),
                f32::from_bits(c.r(820, 4) as u32),
            )
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002a090 vmov.f32 s15,s10
        let value = c.r(808, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002a094 vfma.f32 s15,s14,s13
        let value = f32::from_bits(c.r(824, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(820, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002a098 vmov s14,r3
        let value = c.r(44, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002a09c vmul.f32 s15,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002a0a0 vdiv.f32 s14,s7,s15
        let value = (f32::from_bits(c.r(796, 4) as u32) / f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002a0a4 vmul.f32 s15,s15,s15
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002a0a8 vmul.f32 s14,s14,s8
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(800, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002a0ac vcmpe.f32 s14,s9
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
        // 0x6002a0b0 vmrs apsr,fpscr
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
        // 0x6002a0b4 ble 0x6002a0ea
        let mut t181f00 = 0u64;
        let mut t182100 = 0u64;
        t181f00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t182100 = (((c.r(97, 1) != 0) | (((t181f00 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        if ((t182100 >> 0) & 0xffu64) != 0 {
            return 0x6002a0eau32;
        }
    }
    0x6002a0b6
}
pub fn b_6002a0b6(c: &mut Machine) -> u32 {
    {
        // 0x6002a0b6 vldr.32 s13,[pc,#0x88]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610784950u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(136u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002a0ba vcmpe.f32 s14,s13
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
        // 0x6002a0be vmrs apsr,fpscr
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
        // 0x6002a0c2 bmi 0x6002a0f4
        let mut t180a00 = 0u64;
        t180a00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        if ((t180a00 >> 0) & 0xffu64) != 0 {
            return 0x6002a0f4u32;
        }
    }
    0x6002a0c4
}
pub fn b_6002a0c4(c: &mut Machine) -> u32 {
    {
        // 0x6002a0c4 vldr.32 s14,[pc,#0x7c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610784964u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(124u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002a0c8 vldr.32 s12,[pc,#0x7c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610784968u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(124u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    0x6002a0cc
}
pub fn b_6002a0cc(c: &mut Machine) -> u32 {
    {
        // 0x6002a0cc vmov.f32 s13,0x3f800000
        let value = 1065353216u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002a0d0 vstr.32 s14,[r0,#0x1d4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(468u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002a0d4 vsub.f32 s15,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002a0d8 vfma.f32 s13,s15,s12
        let value = f32::from_bits(c.r(828, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(816, 4) as u32),
                f32::from_bits(c.r(820, 4) as u32),
            )
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002a0dc vfma.f32 s14,s15,s12
        let value = f32::from_bits(c.r(828, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(816, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002a0e0 vstr.32 s13,[r0,#0x1d8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(472u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x6002a0e4 vstr.32 s14,[r0,#0x1dc]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(476u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002a0e8 bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x6002a0ea
}
pub fn b_6002a0ea(c: &mut Machine) -> u32 {
    {
        // 0x6002a0ea vldr.32 s14,[pc,#0x60]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610785002u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(96u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002a0ee vldr.32 s12,[pc,#0x60]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610785006u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(96u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x6002a0f2 b 0x6002a0cc
        return 0x6002a0ccu32;
    }
    0x6002a0f4
}
pub fn b_6002a0f4(c: &mut Machine) -> u32 {
    {
        // 0x6002a0f4 vmul.f32 s13,s14,s14
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002a0f8 vldr.32 s9,[pc,#0x58]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610785016u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(88u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(804, 4, value);
    }
    {
        // 0x6002a0fc vldr.32 s11,[pc,#0x58]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610785020u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(88u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x6002a100 vmul.f32 s12,s14,s13
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x6002a104 vfma.f32 s14,s12,s9
        let value = f32::from_bits(c.r(816, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(804, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002a108 vmul.f32 s13,s13,s12
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002a10c vfma.f32 s14,s13,s11
        let value = f32::from_bits(c.r(820, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(812, 4) as u32),
                f32::from_bits(c.r(824, 4) as u32),
            )
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002a110 vadd.f32 s13,s14,s10
        let value = (f32::from_bits(c.r(824, 4) as u32) + f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002a114 vsub.f32 s10,s14,s10
        let value = (f32::from_bits(c.r(824, 4) as u32) - f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x6002a118 vdiv.f32 s12,s14,s13
        let value = (f32::from_bits(c.r(824, 4) as u32) / f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x6002a11c vdiv.f32 s14,s10,s13
        let value = (f32::from_bits(c.r(808, 4) as u32) / f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002a120 b 0x6002a0cc
        return 0x6002a0ccu32;
    }
    0x6002a122
}
pub fn b_6002a15c(c: &mut Machine) -> u32 {
    {
        // 0x6002a15c vldr.32 s15,[pc,#0x318]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610785116u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(792u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002a160 push {r3,r4,r5,lr}
        let value = c.r(84, 4);
        c.w(128, 4, value);
        let value = (c.r(128, 4)).wrapping_sub(4u64);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 4, c.r(88, 4));
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
        // 0x6002a162 vmul.f32 s15,s0,s15
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002a166 add.w r5,r0,#0x1000
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
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002a16a mov r4,r0
        let value = c.r(32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6002a16c vpush {d8,d9}
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
        // 0x6002a170 vldr.32 s16,[pc,#0x308]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610785136u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(776u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(832, 4, value);
    }
    {
        // 0x6002a174 vmov.f32 s18,0x3f800000
        let value = 1065353216u64;
        c.w(840, 4, value);
    }
    {
        // 0x6002a178 vldr.32 s17,[r5,#0x36c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(876u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(836, 4, value);
    }
    {
        // 0x6002a17c vstr.32 s0,[r5,#0x374]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(884u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002a180 vmul.f32 s0,s15,s16
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(832, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002a184 bl 0x60040c70
        let value = (1610785160u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002a188
}
pub fn b_6002a188(c: &mut Machine) -> u32 {
    {
        // 0x6002a188 vdiv.f32 s15,s17,s0
        let value = (f32::from_bits(c.r(836, 4) as u32) / f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002a18c vldr.32 s13,[pc,#0x2f0]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610785164u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(752u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002a190 vldr.32 s14,[pc,#0x2f0]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610785168u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(752u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002a194 vldr.32 s17,[pc,#0x2f0]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610785172u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(752u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(836, 4, value);
    }
    {
        // 0x6002a198 vmul.f32 s0,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002a19c vcmpe.f32 s0,s14
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
        // 0x6002a1a0 vmrs apsr,fpscr
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
        // 0x6002a1a4 ite gt
        let mut t185d00 = 0u64;
        let mut t185e00 = 0u64;
        let mut t186000 = 0u64;
        t185d00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t185e00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t186000 = (((((t185d00 >> 0) & 0xffu64) != 0) & (((t185e00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
    }
    {
        // 0x6002a1a6 vmul.gt.f32 s0,s0,s16
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
            return 0x6002a1aau32;
        }
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(832, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    0x6002a1aa
}
pub fn b_6002a1aa(c: &mut Machine) -> u32 {
    {
        // 0x6002a1aa vldr.le.32 s0,[pc,#0x2e0]
        let mut t2200 = 0u64;
        let mut t2400 = 0u64;
        let mut t2700 = 0u64;
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610785194u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(736u64)) & 0xffffffffu64;
        t2200 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2400 = (((c.r(97, 1) != 0) | (((t2200 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        t2700 = ((((t2400 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x6002a1aeu32;
        }
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    0x6002a1ae
}
pub fn b_6002a1ae(c: &mut Machine) -> u32 {
    {
        // 0x6002a1ae bl 0x60040c70
        let value = (1610785202u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002a1b2
}
pub fn b_6002a1b2(c: &mut Machine) -> u32 {
    {
        // 0x6002a1b2 vneg.f32 s0,s0
        let value = (-f32::from_bits(c.r(768, 4) as u32)).to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002a1b6 vstr.32 s0,[r5,#0x31c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(796u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002a1ba bl 0x6004137c
        let value = (1610785214u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x6004137cu32;
    }
    0x6002a1be
}
pub fn b_6002a1be(c: &mut Machine) -> u32 {
    {
        // 0x6002a1be vmov.f32 s15,0x3f800000
        let value = 1065353216u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002a1c2 vsub.f32 s0,s15,s0
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002a1c6 bl 0x60041338
        let value = (1610785226u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041338u32;
    }
    0x6002a1ca
}
pub fn b_6002a1ca(c: &mut Machine) -> u32 {
    {
        // 0x6002a1ca vldr.32 s15,[r5,#0x36c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(876u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002a1ce vldr.32 s12,[r5,#0x370]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(880u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x6002a1d2 add.w r1,r4,#0x420
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
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002a1d6 vldr.32 s14,[pc,#0x2b8]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610785238u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(696u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002a1da add.w r3,r4,#0x4d0
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
        let value = ((c.r(48, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(44, 4, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002a1de vmul.f32 s12,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x6002a1e2 vldr.32 s13,[pc,#0x2b0]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610785250u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(688u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002a1e6 addw r2,r4,#0xc94
        let value = ((c.r(48, 4) as u128 + 3220u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 3220u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(3220u64);
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002a1ea vldr.32 s6,[r1,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x6002a1ee vmul.f32 s9,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(804, 4, value);
    }
    {
        // 0x6002a1f2 vldr.32 s7,[r3,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x6002a1f6 vmul.f32 s11,s12,s14
        let value = (f32::from_bits(c.r(816, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x6002a1fa vldr.32 s1,[r2]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(40, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(772, 4, value);
    }
    {
        // 0x6002a1fe vstr.32 s15,[r5,#0x368]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(872u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002a202 addw r2,r4,#0xc88
        let value = ((c.r(48, 4) as u128 + 3208u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 3208u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(3208u64);
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002a206 vstr.32 s9,[r5,#0x364]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(868u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(804, 4));
    }
    {
        // 0x6002a20a vmul.f32 s15,s9,s13
        let value = (f32::from_bits(c.r(804, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002a20e vmul.f32 s10,s11,s14
        let value = (f32::from_bits(c.r(812, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x6002a212 vldr.32 s8,[pc,#0x284]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610785298u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(644u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x6002a216 vldr.32 s5,[pc,#0x284]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610785302u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(644u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(788, 4, value);
    }
    {
        // 0x6002a21a vcvt.f32.s32 s1,s1
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
        let value = c.userop(219, c.r(772, 4), ((t3b500 >> 0) & 0xffu64), 0u64, 4, 4);
        c.w(772, 4, value);
    }
    {
        // 0x6002a21e vstr.32 s15,[r5,#0x360]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(864u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002a222 vmul.f32 s4,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(784, 4, value);
    }
    {
        // 0x6002a226 vmul.f32 s9,s10,s14
        let value = (f32::from_bits(c.r(808, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(804, 4, value);
    }
    {
        // 0x6002a22a vldr.32 s15,[pc,#0x274]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610785322u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(628u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002a22e vfma.f32 s5,s0,s8
        let value = f32::from_bits(c.r(768, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(800, 4) as u32),
                f32::from_bits(c.r(788, 4) as u32),
            )
            .to_bits() as u64;
        c.w(788, 4, value);
    }
    {
        // 0x6002a232 vldr.32 s2,[r4,#0x164]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(356u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(776, 4, value);
    }
    {
        // 0x6002a236 vmul.f32 s0,s4,s13
        let value = (f32::from_bits(c.r(784, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002a23a vldr.32 s3,[r4,#0x214]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(532u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(780, 4, value);
    }
    {
        // 0x6002a23e vmul.f32 s8,s9,s14
        let value = (f32::from_bits(c.r(804, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(800, 4, value);
    }
    {
        // 0x6002a242 vstr.32 s4,[r5,#0x35c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(860u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(784, 4));
    }
    {
        // 0x6002a246 vstr.32 s12,[r5,#0x348]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(840u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x6002a24a addw r0,r4,#0xce8
        let value = ((c.r(48, 4) as u128 + 3304u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 3304u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(3304u64);
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002a24e vstr.32 s0,[r5,#0x358]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(856u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002a252 vmul.f32 s4,s0,s13
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(784, 4, value);
    }
    {
        // 0x6002a256 vmul.f32 s14,s8,s14
        let value = (f32::from_bits(c.r(800, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002a25a vstr.32 s11,[r5,#0x344]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(836u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(812, 4));
    }
    {
        // 0x6002a25e vstr.32 s5,[r5,#0x37c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(892u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(788, 4));
    }
    {
        // 0x6002a262 vmul.f32 s5,s4,s13
        let value = (f32::from_bits(c.r(784, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(788, 4, value);
    }
    {
        // 0x6002a266 vstr.32 s10,[r5,#0x340]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(832u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(808, 4));
    }
    {
        // 0x6002a26a vcmpe.f32 s14,s15
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
        // 0x6002a26e vstr.32 s9,[r5,#0x33c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(828u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(804, 4));
    }
    {
        // 0x6002a272 vstr.32 s8,[r5,#0x338]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(824u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(800, 4));
    }
    {
        // 0x6002a276 vmul.f32 s13,s5,s13
        let value = (f32::from_bits(c.r(788, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002a27a vstr.32 s14,[r5,#0x334]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(820u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002a27e vmrs apsr,fpscr
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
        // 0x6002a282 vstr.32 s4,[r5,#0x354]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(852u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(784, 4));
    }
    {
        // 0x6002a286 vldr.32 s4,[r4,#0x2c4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(708u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(784, 4, value);
    }
    {
        // 0x6002a28a vstr.32 s5,[r5,#0x350]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(848u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(788, 4));
    }
    {
        // 0x6002a28e vselgt.f32 s0,s14,s15
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
        // 0x6002a292 vcmpe.f32 s2,s0
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
        // 0x6002a296 vstr.32 s13,[r5,#0x34c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(844u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x6002a29a vldr.32 s13,[r2,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002a29e vmrs apsr,fpscr
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
        // 0x6002a2a2 vldr.32 s5,[r4,#0x374]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(884u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(788, 4, value);
    }
    {
        // 0x6002a2a6 vcmpe.f32 s8,s15
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
        // 0x6002a2aa vselgt.f32 s2,s0,s2
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
        // 0x6002a2ae vstr.32 s2,[r4,#0x168]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(360u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(776, 4));
    }
    {
        // 0x6002a2b2 vmrs apsr,fpscr
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
        // 0x6002a2b6 vselgt.f32 s2,s8,s15
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
        // 0x6002a2ba vcmpe.f32 s3,s2
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
        // 0x6002a2be vmrs apsr,fpscr
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
        // 0x6002a2c2 vcmpe.f32 s9,s15
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
        // 0x6002a2c6 vselgt.f32 s3,s2,s3
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
        // 0x6002a2ca vstr.32 s3,[r4,#0x218]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(536u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(780, 4));
    }
    {
        // 0x6002a2ce vmrs apsr,fpscr
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
        // 0x6002a2d2 vselgt.f32 s3,s9,s15
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
        c.w(780, 4, value);
    }
    {
        // 0x6002a2d6 vcmpe.f32 s4,s3
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
        // 0x6002a2da vmrs apsr,fpscr
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
        // 0x6002a2de vcmpe.f32 s10,s15
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
        // 0x6002a2e2 vselgt.f32 s4,s3,s4
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
        // 0x6002a2e6 vstr.32 s4,[r4,#0x2c8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(712u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(784, 4));
    }
    {
        // 0x6002a2ea vmrs apsr,fpscr
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
        // 0x6002a2ee vselgt.f32 s4,s10,s15
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
        // 0x6002a2f2 vcmpe.f32 s5,s4
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
        // 0x6002a2f6 vmrs apsr,fpscr
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
        // 0x6002a2fa vcmpe.f32 s11,s15
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
        // 0x6002a2fe vselgt.f32 s5,s4,s5
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
        // 0x6002a302 vstr.32 s5,[r4,#0x378]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(888u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(788, 4));
    }
    {
        // 0x6002a306 vmrs apsr,fpscr
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
        // 0x6002a30a vselgt.f32 s5,s11,s15
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
        // 0x6002a30e vcmpe.f32 s6,s5
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
        // 0x6002a312 vmrs apsr,fpscr
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
        // 0x6002a316 vcmpe.f32 s12,s15
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
        // 0x6002a31a vselgt.f32 s6,s5,s6
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
        // 0x6002a31e vstr.32 s6,[r1,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(792, 4));
    }
    {
        // 0x6002a322 vmrs apsr,fpscr
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
        // 0x6002a326 addw r1,r4,#0xd48
        let value = ((c.r(48, 4) as u128 + 3400u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 3400u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(3400u64);
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002a32a vselgt.f32 s15,s12,s15
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
        // 0x6002a32e vcmpe.f32 s7,s15
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
        // 0x6002a332 vmrs apsr,fpscr
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
        // 0x6002a336 vcmpe.f32 s14,s17
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
        t35c00 = ((f32::from_bits(c.r(824, 4) as u32) < f32::from_bits(c.r(836, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(824, 4) as u32) == f32::from_bits(c.r(836, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(836, 4) as u32) <= f32::from_bits(c.r(824, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(824, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002a33a vselgt.f32 s7,s15,s7
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
        // 0x6002a33e vstr.32 s7,[r3,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(796, 4));
    }
    {
        // 0x6002a342 vmrs apsr,fpscr
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
        // 0x6002a346 addw r3,r4,#0xe68
        let value = ((c.r(48, 4) as u128 + 3688u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 3688u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(3688u64);
        c.w(44, 4, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002a34a vcmpe.f32 s8,s17
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
        t35c00 = ((f32::from_bits(c.r(800, 4) as u32) < f32::from_bits(c.r(836, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(800, 4) as u32) == f32::from_bits(c.r(836, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(836, 4) as u32) <= f32::from_bits(c.r(800, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(800, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002a34e vselgt.f32 s14,s14,s17
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(836, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(824, 4, value);
    }
    {
        // 0x6002a352 vdiv.f32 s7,s1,s14
        let value = (f32::from_bits(c.r(772, 4) as u32) / f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(796, 4, value);
    }
    {
        // 0x6002a356 vmrs apsr,fpscr
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
        // 0x6002a35a vcmpe.f32 s9,s17
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
        t35c00 = ((f32::from_bits(c.r(804, 4) as u32) < f32::from_bits(c.r(836, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(804, 4) as u32) == f32::from_bits(c.r(836, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(836, 4) as u32) <= f32::from_bits(c.r(804, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(804, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002a35e vselgt.f32 s8,s8,s17
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(836, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(800, 4, value);
    }
    {
        // 0x6002a362 vmrs apsr,fpscr
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
        // 0x6002a366 vcmpe.f32 s10,s17
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
        t35c00 = ((f32::from_bits(c.r(808, 4) as u32) < f32::from_bits(c.r(836, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(808, 4) as u32) == f32::from_bits(c.r(836, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(836, 4) as u32) <= f32::from_bits(c.r(808, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(808, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002a36a vselgt.f32 s9,s9,s17
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(836, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(804, 4, value);
    }
    {
        // 0x6002a36e vmrs apsr,fpscr
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
        // 0x6002a372 vcmpe.f32 s11,s17
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
        t35c00 = ((f32::from_bits(c.r(812, 4) as u32) < f32::from_bits(c.r(836, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(812, 4) as u32) == f32::from_bits(c.r(836, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(836, 4) as u32) <= f32::from_bits(c.r(812, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(812, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002a376 vmul.f32 s13,s7,s13
        let value = (f32::from_bits(c.r(796, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002a37a vstr.32 s7,[r2,#0x1c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(796, 4));
    }
    {
        // 0x6002a37e vselgt.f32 s10,s10,s17
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(836, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(808, 4, value);
    }
    {
        // 0x6002a382 vmrs apsr,fpscr
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
        // 0x6002a386 vcmpe.f32 s12,s17
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
        t35c00 = ((f32::from_bits(c.r(816, 4) as u32) < f32::from_bits(c.r(836, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(816, 4) as u32) == f32::from_bits(c.r(836, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(836, 4) as u32) <= f32::from_bits(c.r(816, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(816, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002a38a vdiv.f32 s6,s18,s13
        let value = (f32::from_bits(c.r(840, 4) as u32) / f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(792, 4, value);
    }
    {
        // 0x6002a38e vstr.32 s13,[r2,#0x20]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x6002a392 vldr.32 s14,[r1,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002a396 vselgt.f32 s11,s11,s17
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(836, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(812, 4, value);
    }
    {
        // 0x6002a39a vldr.32 s7,[r0,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x6002a39e vldr.32 s15,[r3,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002a3a2 vmrs apsr,fpscr
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
        // 0x6002a3a6 vselgt.f32 s12,s12,s17
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(836, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(816, 4, value);
    }
    {
        // 0x6002a3aa vstr.32 s6,[r2,#0x24]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(792, 4));
    }
    {
        // 0x6002a3ae ldr.w r2,[r4,#0xcf4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(3316u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x6002a3b2 vmov s13,r2
        let value = c.r(40, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002a3b6 ldr.w r2,[r4,#0xd54]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(3412u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x6002a3ba vcvt.f32.s32 s6,s13
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
        // 0x6002a3be vmov s13,r2
        let value = c.r(40, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002a3c2 ldr.w r2,[r4,#0xdb4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(3508u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x6002a3c6 vcvt.f32.s32 s13,s13
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
        // 0x6002a3ca vdiv.f32 s4,s6,s8
        let value = (f32::from_bits(c.r(792, 4) as u32) / f32::from_bits(c.r(800, 4) as u32))
            .to_bits() as u64;
        c.w(784, 4, value);
    }
    {
        // 0x6002a3ce vmov s8,r2
        let value = c.r(40, 4);
        c.w(800, 4, value);
    }
    {
        // 0x6002a3d2 addw r2,r4,#0xda8
        let value = ((c.r(48, 4) as u128 + 3496u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 3496u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(3496u64);
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002a3d6 vdiv.f32 s5,s13,s9
        let value = (f32::from_bits(c.r(820, 4) as u32) / f32::from_bits(c.r(804, 4) as u32))
            .to_bits() as u64;
        c.w(788, 4, value);
    }
    {
        // 0x6002a3da vldr.32 s13,[r2,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002a3de vcvt.f32.s32 s8,s8
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
        // 0x6002a3e2 vdiv.f32 s6,s8,s10
        let value = (f32::from_bits(c.r(800, 4) as u32) / f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(792, 4, value);
    }
    {
        // 0x6002a3e6 vstr.32 s4,[r0,#0x1c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(784, 4));
    }
    {
        // 0x6002a3ea vmul.f32 s10,s4,s7
        let value = (f32::from_bits(c.r(784, 4) as u32) * f32::from_bits(c.r(796, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x6002a3ee vmul.f32 s14,s5,s14
        let value = (f32::from_bits(c.r(788, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002a3f2 vstr.32 s10,[r0,#0x20]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(808, 4));
    }
    {
        // 0x6002a3f6 vdiv.f32 s9,s18,s10
        let value = (f32::from_bits(c.r(840, 4) as u32) / f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(804, 4, value);
    }
    {
        // 0x6002a3fa vdiv.f32 s10,s18,s14
        let value = (f32::from_bits(c.r(840, 4) as u32) / f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x6002a3fe vmul.f32 s13,s6,s13
        let value = (f32::from_bits(c.r(792, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002a402 vdiv.f32 s8,s18,s13
        let value = (f32::from_bits(c.r(840, 4) as u32) / f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(800, 4, value);
    }
    {
        // 0x6002a406 vstr.32 s9,[r0,#0x24]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(804, 4));
    }
    {
        // 0x6002a40a vstr.32 s14,[r1,#0x20]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002a40e vstr.32 s5,[r1,#0x1c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(788, 4));
    }
    {
        // 0x6002a412 vstr.32 s10,[r1,#0x24]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(808, 4));
    }
    {
        // 0x6002a416 addw r1,r4,#0xe14
        let value = ((c.r(48, 4) as u128 + 3604u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 3604u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(3604u64);
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002a41a vldr.32 s9,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(804, 4, value);
    }
    {
        // 0x6002a41e addw r1,r4,#0xe74
        let value = ((c.r(48, 4) as u128 + 3700u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 3700u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(3700u64);
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002a422 addw r4,r4,#0xe08
        let value = ((c.r(48, 4) as u128 + 3592u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 3592u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(3592u64);
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002a426 vldr.32 s10,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x6002a42a vcvt.f32.s32 s9,s9
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
        // 0x6002a42e vldr.32 s14,[r4,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002a432 vcvt.f32.s32 s10,s10
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
        // 0x6002a436 vstr.32 s13,[r2,#0x20]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x6002a43a vstr.32 s6,[r2,#0x1c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(792, 4));
    }
    {
        // 0x6002a43e vstr.32 s8,[r2,#0x24]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(800, 4));
    }
    {
        // 0x6002a442 vdiv.f32 s8,s9,s11
        let value = (f32::from_bits(c.r(804, 4) as u32) / f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(800, 4, value);
    }
    {
        // 0x6002a446 vdiv.f32 s13,s10,s12
        let value = (f32::from_bits(c.r(808, 4) as u32) / f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002a44a vmul.f32 s14,s8,s14
        let value = (f32::from_bits(c.r(800, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002a44e vstr.32 s8,[r4,#0x1c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(800, 4));
    }
    {
        // 0x6002a452 vstr.32 s14,[r4,#0x20]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002a456 vdiv.f32 s12,s18,s14
        let value = (f32::from_bits(c.r(840, 4) as u32) / f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x6002a45a vmul.f32 s15,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002a45e vdiv.f32 s14,s18,s15
        let value = (f32::from_bits(c.r(840, 4) as u32) / f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002a462 vpop {d8,d9}
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
        // 0x6002a466 vstr.32 s12,[r4,#0x24]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x6002a46a vstr.32 s13,[r3,#0x1c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x6002a46e vstr.32 s15,[r3,#0x20]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002a472 vstr.32 s14,[r3,#0x24]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002a476 pop {r3,r4,r5,pc}
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
    0x6002a478
}
pub fn b_6002a4a4(c: &mut Machine) -> u32 {
    {
        // 0x6002a4a4 vmov.f32 s15,0x3e000000
        let value = 1040187392u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002a4a8 vldr.32 s13,[pc,#0x324]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610785960u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(804u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002a4ac vldr.32 s14,[pc,#0x324]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610785964u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(804u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002a4b0 push {r3,r4,r5,lr}
        let value = c.r(84, 4);
        c.w(128, 4, value);
        let value = (c.r(128, 4)).wrapping_sub(4u64);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 4, c.r(88, 4));
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
        // 0x6002a4b2 add.w r5,r0,#0x1000
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
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002a4b6 vfma.f32 s15,s0,s13
        let value = f32::from_bits(c.r(768, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(820, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002a4ba mov r4,r0
        let value = c.r(32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6002a4bc vldr.32 s0,[r5,#0x374]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(884u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002a4c0 vmul.f32 s0,s0,s14
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002a4c4 vpush {d8,d9}
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
        // 0x6002a4c8 vldr.32 s16,[pc,#0x30c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610785992u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(780u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(832, 4, value);
    }
    {
        // 0x6002a4cc vmov.f32 s18,0x3f800000
        let value = 1065353216u64;
        c.w(840, 4, value);
    }
    {
        // 0x6002a4d0 vldr.32 s17,[r5,#0x36c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(876u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(836, 4, value);
    }
    {
        // 0x6002a4d4 vmul.f32 s0,s0,s16
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(832, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002a4d8 vstr.32 s15,[r5,#0x370]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(880u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002a4dc bl 0x60040c70
        let value = (1610786016u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002a4e0
}
pub fn b_6002a4e0(c: &mut Machine) -> u32 {
    {
        // 0x6002a4e0 vdiv.f32 s15,s17,s0
        let value = (f32::from_bits(c.r(836, 4) as u32) / f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002a4e4 vldr.32 s13,[pc,#0x2f4]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610786020u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(756u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002a4e8 vldr.32 s14,[pc,#0x2f4]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610786024u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(756u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002a4ec vldr.32 s17,[pc,#0x2f4]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610786028u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(756u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(836, 4, value);
    }
    {
        // 0x6002a4f0 vmul.f32 s0,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002a4f4 vcmpe.f32 s0,s14
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
        // 0x6002a4f8 vmrs apsr,fpscr
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
        // 0x6002a4fc ite gt
        let mut t185d00 = 0u64;
        let mut t185e00 = 0u64;
        let mut t186000 = 0u64;
        t185d00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t185e00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t186000 = (((((t185d00 >> 0) & 0xffu64) != 0) & (((t185e00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
    }
    {
        // 0x6002a4fe vmul.gt.f32 s0,s0,s16
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
            return 0x6002a502u32;
        }
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(832, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    0x6002a502
}
pub fn b_6002a502(c: &mut Machine) -> u32 {
    {
        // 0x6002a502 vldr.le.32 s0,[pc,#0x2e4]
        let mut t2200 = 0u64;
        let mut t2400 = 0u64;
        let mut t2700 = 0u64;
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610786050u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(740u64)) & 0xffffffffu64;
        t2200 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2400 = (((c.r(97, 1) != 0) | (((t2200 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        t2700 = ((((t2400 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x6002a506u32;
        }
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    0x6002a506
}
pub fn b_6002a506(c: &mut Machine) -> u32 {
    {
        // 0x6002a506 bl 0x60040c70
        let value = (1610786058u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002a50a
}
pub fn b_6002a50a(c: &mut Machine) -> u32 {
    {
        // 0x6002a50a vneg.f32 s0,s0
        let value = (-f32::from_bits(c.r(768, 4) as u32)).to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002a50e vstr.32 s0,[r5,#0x31c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(796u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002a512 bl 0x6004137c
        let value = (1610786070u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x6004137cu32;
    }
    0x6002a516
}
pub fn b_6002a516(c: &mut Machine) -> u32 {
    {
        // 0x6002a516 vmov.f32 s15,0x3f800000
        let value = 1065353216u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002a51a vsub.f32 s0,s15,s0
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002a51e bl 0x60041338
        let value = (1610786082u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041338u32;
    }
    0x6002a522
}
pub fn b_6002a522(c: &mut Machine) -> u32 {
    {
        // 0x6002a522 vldr.32 s15,[r5,#0x36c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(876u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002a526 vldr.32 s12,[r5,#0x370]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(880u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x6002a52a add.w r1,r4,#0x420
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
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002a52e vldr.32 s14,[pc,#0x2bc]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610786094u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(700u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002a532 add.w r3,r4,#0x4d0
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
        let value = ((c.r(48, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(44, 4, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002a536 vmul.f32 s12,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x6002a53a vldr.32 s13,[pc,#0x2b4]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610786106u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(692u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002a53e addw r2,r4,#0xc94
        let value = ((c.r(48, 4) as u128 + 3220u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 3220u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(3220u64);
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002a542 vldr.32 s6,[r1,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x6002a546 vmul.f32 s9,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(804, 4, value);
    }
    {
        // 0x6002a54a vldr.32 s7,[r3,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x6002a54e vmul.f32 s11,s12,s14
        let value = (f32::from_bits(c.r(816, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x6002a552 vldr.32 s1,[r2]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(40, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(772, 4, value);
    }
    {
        // 0x6002a556 vstr.32 s15,[r5,#0x368]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(872u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002a55a addw r2,r4,#0xc88
        let value = ((c.r(48, 4) as u128 + 3208u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 3208u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(3208u64);
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002a55e vstr.32 s9,[r5,#0x364]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(868u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(804, 4));
    }
    {
        // 0x6002a562 vmul.f32 s15,s9,s13
        let value = (f32::from_bits(c.r(804, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002a566 vmul.f32 s10,s11,s14
        let value = (f32::from_bits(c.r(812, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x6002a56a vldr.32 s8,[pc,#0x288]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610786154u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(648u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x6002a56e vldr.32 s5,[pc,#0x288]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610786158u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(648u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(788, 4, value);
    }
    {
        // 0x6002a572 vcvt.f32.s32 s1,s1
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
        let value = c.userop(219, c.r(772, 4), ((t3b500 >> 0) & 0xffu64), 0u64, 4, 4);
        c.w(772, 4, value);
    }
    {
        // 0x6002a576 vstr.32 s15,[r5,#0x360]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(864u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002a57a vmul.f32 s4,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(784, 4, value);
    }
    {
        // 0x6002a57e vmul.f32 s9,s10,s14
        let value = (f32::from_bits(c.r(808, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(804, 4, value);
    }
    {
        // 0x6002a582 vldr.32 s15,[pc,#0x278]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610786178u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(632u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002a586 vfma.f32 s5,s0,s8
        let value = f32::from_bits(c.r(768, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(800, 4) as u32),
                f32::from_bits(c.r(788, 4) as u32),
            )
            .to_bits() as u64;
        c.w(788, 4, value);
    }
    {
        // 0x6002a58a vldr.32 s2,[r4,#0x164]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(356u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(776, 4, value);
    }
    {
        // 0x6002a58e vmul.f32 s0,s4,s13
        let value = (f32::from_bits(c.r(784, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002a592 vldr.32 s3,[r4,#0x214]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(532u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(780, 4, value);
    }
    {
        // 0x6002a596 vmul.f32 s8,s9,s14
        let value = (f32::from_bits(c.r(804, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(800, 4, value);
    }
    {
        // 0x6002a59a vstr.32 s4,[r5,#0x35c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(860u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(784, 4));
    }
    {
        // 0x6002a59e vstr.32 s12,[r5,#0x348]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(840u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x6002a5a2 addw r0,r4,#0xce8
        let value = ((c.r(48, 4) as u128 + 3304u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 3304u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(3304u64);
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002a5a6 vstr.32 s0,[r5,#0x358]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(856u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002a5aa vmul.f32 s4,s0,s13
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(784, 4, value);
    }
    {
        // 0x6002a5ae vmul.f32 s14,s8,s14
        let value = (f32::from_bits(c.r(800, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002a5b2 vstr.32 s11,[r5,#0x344]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(836u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(812, 4));
    }
    {
        // 0x6002a5b6 vstr.32 s5,[r5,#0x37c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(892u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(788, 4));
    }
    {
        // 0x6002a5ba vmul.f32 s5,s4,s13
        let value = (f32::from_bits(c.r(784, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(788, 4, value);
    }
    {
        // 0x6002a5be vstr.32 s10,[r5,#0x340]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(832u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(808, 4));
    }
    {
        // 0x6002a5c2 vcmpe.f32 s14,s15
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
        // 0x6002a5c6 vstr.32 s9,[r5,#0x33c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(828u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(804, 4));
    }
    {
        // 0x6002a5ca vstr.32 s8,[r5,#0x338]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(824u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(800, 4));
    }
    {
        // 0x6002a5ce vmul.f32 s13,s5,s13
        let value = (f32::from_bits(c.r(788, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002a5d2 vstr.32 s14,[r5,#0x334]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(820u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002a5d6 vmrs apsr,fpscr
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
        // 0x6002a5da vstr.32 s4,[r5,#0x354]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(852u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(784, 4));
    }
    {
        // 0x6002a5de vldr.32 s4,[r4,#0x2c4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(708u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(784, 4, value);
    }
    {
        // 0x6002a5e2 vstr.32 s5,[r5,#0x350]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(848u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(788, 4));
    }
    {
        // 0x6002a5e6 vselgt.f32 s0,s14,s15
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
        // 0x6002a5ea vcmpe.f32 s2,s0
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
        // 0x6002a5ee vstr.32 s13,[r5,#0x34c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(844u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x6002a5f2 vldr.32 s13,[r2,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002a5f6 vmrs apsr,fpscr
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
        // 0x6002a5fa vldr.32 s5,[r4,#0x374]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(884u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(788, 4, value);
    }
    {
        // 0x6002a5fe vcmpe.f32 s8,s15
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
        // 0x6002a602 vselgt.f32 s2,s0,s2
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
        // 0x6002a606 vstr.32 s2,[r4,#0x168]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(360u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(776, 4));
    }
    {
        // 0x6002a60a vmrs apsr,fpscr
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
        // 0x6002a60e vselgt.f32 s2,s8,s15
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
        // 0x6002a612 vcmpe.f32 s3,s2
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
        // 0x6002a616 vmrs apsr,fpscr
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
        // 0x6002a61a vcmpe.f32 s9,s15
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
        // 0x6002a61e vselgt.f32 s3,s2,s3
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
        // 0x6002a622 vstr.32 s3,[r4,#0x218]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(536u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(780, 4));
    }
    {
        // 0x6002a626 vmrs apsr,fpscr
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
        // 0x6002a62a vselgt.f32 s3,s9,s15
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
        c.w(780, 4, value);
    }
    {
        // 0x6002a62e vcmpe.f32 s4,s3
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
        // 0x6002a632 vmrs apsr,fpscr
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
        // 0x6002a636 vcmpe.f32 s10,s15
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
        // 0x6002a63a vselgt.f32 s4,s3,s4
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
        // 0x6002a63e vstr.32 s4,[r4,#0x2c8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(712u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(784, 4));
    }
    {
        // 0x6002a642 vmrs apsr,fpscr
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
        // 0x6002a646 vselgt.f32 s4,s10,s15
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
        // 0x6002a64a vcmpe.f32 s5,s4
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
        // 0x6002a64e vmrs apsr,fpscr
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
        // 0x6002a652 vcmpe.f32 s11,s15
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
        // 0x6002a656 vselgt.f32 s5,s4,s5
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
        // 0x6002a65a vstr.32 s5,[r4,#0x378]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(888u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(788, 4));
    }
    {
        // 0x6002a65e vmrs apsr,fpscr
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
        // 0x6002a662 vselgt.f32 s5,s11,s15
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
        // 0x6002a666 vcmpe.f32 s6,s5
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
        // 0x6002a66a vmrs apsr,fpscr
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
        // 0x6002a66e vcmpe.f32 s12,s15
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
        // 0x6002a672 vselgt.f32 s6,s5,s6
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
        // 0x6002a676 vstr.32 s6,[r1,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(792, 4));
    }
    {
        // 0x6002a67a vmrs apsr,fpscr
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
        // 0x6002a67e addw r1,r4,#0xd48
        let value = ((c.r(48, 4) as u128 + 3400u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 3400u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(3400u64);
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002a682 vselgt.f32 s15,s12,s15
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
        // 0x6002a686 vcmpe.f32 s7,s15
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
        // 0x6002a68a vmrs apsr,fpscr
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
        // 0x6002a68e vcmpe.f32 s14,s17
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
        t35c00 = ((f32::from_bits(c.r(824, 4) as u32) < f32::from_bits(c.r(836, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(824, 4) as u32) == f32::from_bits(c.r(836, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(836, 4) as u32) <= f32::from_bits(c.r(824, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(824, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002a692 vselgt.f32 s7,s15,s7
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
        // 0x6002a696 vstr.32 s7,[r3,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(796, 4));
    }
    {
        // 0x6002a69a vmrs apsr,fpscr
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
        // 0x6002a69e addw r3,r4,#0xe68
        let value = ((c.r(48, 4) as u128 + 3688u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 3688u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(3688u64);
        c.w(44, 4, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002a6a2 vcmpe.f32 s8,s17
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
        t35c00 = ((f32::from_bits(c.r(800, 4) as u32) < f32::from_bits(c.r(836, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(800, 4) as u32) == f32::from_bits(c.r(836, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(836, 4) as u32) <= f32::from_bits(c.r(800, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(800, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002a6a6 vselgt.f32 s14,s14,s17
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(836, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(824, 4, value);
    }
    {
        // 0x6002a6aa vdiv.f32 s7,s1,s14
        let value = (f32::from_bits(c.r(772, 4) as u32) / f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(796, 4, value);
    }
    {
        // 0x6002a6ae vmrs apsr,fpscr
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
        // 0x6002a6b2 vcmpe.f32 s9,s17
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
        t35c00 = ((f32::from_bits(c.r(804, 4) as u32) < f32::from_bits(c.r(836, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(804, 4) as u32) == f32::from_bits(c.r(836, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(836, 4) as u32) <= f32::from_bits(c.r(804, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(804, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002a6b6 vselgt.f32 s8,s8,s17
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(836, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(800, 4, value);
    }
    {
        // 0x6002a6ba vmrs apsr,fpscr
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
        // 0x6002a6be vcmpe.f32 s10,s17
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
        t35c00 = ((f32::from_bits(c.r(808, 4) as u32) < f32::from_bits(c.r(836, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(808, 4) as u32) == f32::from_bits(c.r(836, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(836, 4) as u32) <= f32::from_bits(c.r(808, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(808, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002a6c2 vselgt.f32 s9,s9,s17
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(836, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(804, 4, value);
    }
    {
        // 0x6002a6c6 vmrs apsr,fpscr
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
        // 0x6002a6ca vcmpe.f32 s11,s17
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
        t35c00 = ((f32::from_bits(c.r(812, 4) as u32) < f32::from_bits(c.r(836, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(812, 4) as u32) == f32::from_bits(c.r(836, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(836, 4) as u32) <= f32::from_bits(c.r(812, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(812, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002a6ce vmul.f32 s13,s7,s13
        let value = (f32::from_bits(c.r(796, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002a6d2 vstr.32 s7,[r2,#0x1c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(796, 4));
    }
    {
        // 0x6002a6d6 vselgt.f32 s10,s10,s17
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(836, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(808, 4, value);
    }
    {
        // 0x6002a6da vmrs apsr,fpscr
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
        // 0x6002a6de vcmpe.f32 s12,s17
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
        t35c00 = ((f32::from_bits(c.r(816, 4) as u32) < f32::from_bits(c.r(836, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(816, 4) as u32) == f32::from_bits(c.r(836, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(836, 4) as u32) <= f32::from_bits(c.r(816, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(816, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002a6e2 vdiv.f32 s6,s18,s13
        let value = (f32::from_bits(c.r(840, 4) as u32) / f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(792, 4, value);
    }
    {
        // 0x6002a6e6 vstr.32 s13,[r2,#0x20]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x6002a6ea vldr.32 s14,[r1,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002a6ee vselgt.f32 s11,s11,s17
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(836, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(812, 4, value);
    }
    {
        // 0x6002a6f2 vldr.32 s7,[r0,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x6002a6f6 vldr.32 s15,[r3,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002a6fa vmrs apsr,fpscr
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
        // 0x6002a6fe vselgt.f32 s12,s12,s17
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(836, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(816, 4, value);
    }
    {
        // 0x6002a702 vstr.32 s6,[r2,#0x24]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(792, 4));
    }
    {
        // 0x6002a706 ldr.w r2,[r4,#0xcf4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(3316u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x6002a70a vmov s13,r2
        let value = c.r(40, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002a70e ldr.w r2,[r4,#0xd54]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(3412u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x6002a712 vcvt.f32.s32 s6,s13
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
        // 0x6002a716 vmov s13,r2
        let value = c.r(40, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002a71a ldr.w r2,[r4,#0xdb4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(3508u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x6002a71e vcvt.f32.s32 s13,s13
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
        // 0x6002a722 vdiv.f32 s4,s6,s8
        let value = (f32::from_bits(c.r(792, 4) as u32) / f32::from_bits(c.r(800, 4) as u32))
            .to_bits() as u64;
        c.w(784, 4, value);
    }
    {
        // 0x6002a726 vmov s8,r2
        let value = c.r(40, 4);
        c.w(800, 4, value);
    }
    {
        // 0x6002a72a addw r2,r4,#0xda8
        let value = ((c.r(48, 4) as u128 + 3496u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 3496u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(3496u64);
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002a72e vdiv.f32 s5,s13,s9
        let value = (f32::from_bits(c.r(820, 4) as u32) / f32::from_bits(c.r(804, 4) as u32))
            .to_bits() as u64;
        c.w(788, 4, value);
    }
    {
        // 0x6002a732 vldr.32 s13,[r2,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002a736 vcvt.f32.s32 s8,s8
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
        // 0x6002a73a vdiv.f32 s6,s8,s10
        let value = (f32::from_bits(c.r(800, 4) as u32) / f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(792, 4, value);
    }
    {
        // 0x6002a73e vstr.32 s4,[r0,#0x1c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(784, 4));
    }
    {
        // 0x6002a742 vmul.f32 s10,s4,s7
        let value = (f32::from_bits(c.r(784, 4) as u32) * f32::from_bits(c.r(796, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x6002a746 vmul.f32 s14,s5,s14
        let value = (f32::from_bits(c.r(788, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002a74a vstr.32 s10,[r0,#0x20]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(808, 4));
    }
    {
        // 0x6002a74e vdiv.f32 s9,s18,s10
        let value = (f32::from_bits(c.r(840, 4) as u32) / f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(804, 4, value);
    }
    {
        // 0x6002a752 vdiv.f32 s10,s18,s14
        let value = (f32::from_bits(c.r(840, 4) as u32) / f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x6002a756 vmul.f32 s13,s6,s13
        let value = (f32::from_bits(c.r(792, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002a75a vdiv.f32 s8,s18,s13
        let value = (f32::from_bits(c.r(840, 4) as u32) / f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(800, 4, value);
    }
    {
        // 0x6002a75e vstr.32 s9,[r0,#0x24]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(804, 4));
    }
    {
        // 0x6002a762 vstr.32 s14,[r1,#0x20]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002a766 vstr.32 s5,[r1,#0x1c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(788, 4));
    }
    {
        // 0x6002a76a vstr.32 s10,[r1,#0x24]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(808, 4));
    }
    {
        // 0x6002a76e addw r1,r4,#0xe14
        let value = ((c.r(48, 4) as u128 + 3604u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 3604u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(3604u64);
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002a772 vldr.32 s9,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(804, 4, value);
    }
    {
        // 0x6002a776 addw r1,r4,#0xe74
        let value = ((c.r(48, 4) as u128 + 3700u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 3700u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(3700u64);
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002a77a addw r4,r4,#0xe08
        let value = ((c.r(48, 4) as u128 + 3592u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 3592u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(3592u64);
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002a77e vldr.32 s10,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x6002a782 vcvt.f32.s32 s9,s9
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
        // 0x6002a786 vldr.32 s14,[r4,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002a78a vcvt.f32.s32 s10,s10
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
        // 0x6002a78e vstr.32 s13,[r2,#0x20]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x6002a792 vstr.32 s6,[r2,#0x1c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(792, 4));
    }
    {
        // 0x6002a796 vstr.32 s8,[r2,#0x24]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(800, 4));
    }
    {
        // 0x6002a79a vdiv.f32 s8,s9,s11
        let value = (f32::from_bits(c.r(804, 4) as u32) / f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(800, 4, value);
    }
    {
        // 0x6002a79e vdiv.f32 s13,s10,s12
        let value = (f32::from_bits(c.r(808, 4) as u32) / f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002a7a2 vmul.f32 s14,s8,s14
        let value = (f32::from_bits(c.r(800, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002a7a6 vstr.32 s8,[r4,#0x1c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(800, 4));
    }
    {
        // 0x6002a7aa vstr.32 s14,[r4,#0x20]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002a7ae vdiv.f32 s12,s18,s14
        let value = (f32::from_bits(c.r(840, 4) as u32) / f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x6002a7b2 vmul.f32 s15,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002a7b6 vdiv.f32 s14,s18,s15
        let value = (f32::from_bits(c.r(840, 4) as u32) / f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002a7ba vpop {d8,d9}
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
        // 0x6002a7be vstr.32 s12,[r4,#0x24]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x6002a7c2 vstr.32 s13,[r3,#0x1c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x6002a7c6 vstr.32 s15,[r3,#0x20]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002a7ca vstr.32 s14,[r3,#0x24]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002a7ce pop {r3,r4,r5,pc}
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
    0x6002a7d0
}
pub fn b_6002a800(c: &mut Machine) -> u32 {
    {
        // 0x6002a800 vmov.f32 s15,0x41c00000
        let value = 1103101952u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002a804 vldr.32 s13,[pc,#0x33c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610786820u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(828u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002a808 vldr.32 s14,[pc,#0x33c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610786824u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(828u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002a80c vfma.f32 s15,s0,s13
        let value = f32::from_bits(c.r(768, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(820, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002a810 push {r3,r4,r5,lr}
        let value = c.r(84, 4);
        c.w(128, 4, value);
        let value = (c.r(128, 4)).wrapping_sub(4u64);
        c.w(128, 4, value);
        c.store(c.r(128, 4) as u32, 4, c.r(88, 4));
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
        // 0x6002a812 vpush {d8,d9}
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
        // 0x6002a816 mov r4,r0
        let value = c.r(32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6002a818 add.w r5,r0,#0x1000
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
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002a81c vmul.f32 s0,s15,s14
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002a820 vldr.32 s17,[pc,#0x328]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610786848u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(808u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(836, 4, value);
    }
    {
        // 0x6002a824 bl 0x60040c70
        let value = (1610786856u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002a828
}
pub fn b_6002a828(c: &mut Machine) -> u32 {
    {
        // 0x6002a828 vldr.32 s12,[pc,#0x324]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610786856u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(804u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x6002a82c vmov.f32 s14,0x3f800000
        let value = 1065353216u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002a830 vldr.32 s13,[pc,#0x320]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610786864u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(800u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002a834 vmul.f32 s0,s0,s12
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002a838 vldr.32 s15,[r5,#0x374]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(884u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002a83c vmov.f32 s18,0x3f800000
        let value = 1065353216u64;
        c.w(840, 4, value);
    }
    {
        // 0x6002a840 vmul.f32 s15,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002a844 vdiv.f32 s16,s14,s0
        let value = (f32::from_bits(c.r(824, 4) as u32) / f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(832, 4, value);
    }
    {
        // 0x6002a848 vmul.f32 s0,s15,s17
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(836, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002a84c vstr.32 s16,[r5,#0x36c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(876u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(832, 4));
    }
    {
        // 0x6002a850 bl 0x60040c70
        let value = (1610786900u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002a854
}
pub fn b_6002a854(c: &mut Machine) -> u32 {
    {
        // 0x6002a854 vdiv.f32 s15,s16,s0
        let value = (f32::from_bits(c.r(832, 4) as u32) / f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002a858 vldr.32 s13,[pc,#0x2fc]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610786904u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(764u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002a85c vldr.32 s14,[pc,#0x2fc]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610786908u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(764u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002a860 vmul.f32 s0,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002a864 vcmpe.f32 s0,s14
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
        // 0x6002a868 vmrs apsr,fpscr
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
        // 0x6002a86c ite gt
        let mut t185d00 = 0u64;
        let mut t185e00 = 0u64;
        let mut t186000 = 0u64;
        t185d00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t185e00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t186000 = (((((t185d00 >> 0) & 0xffu64) != 0) & (((t185e00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
    }
    {
        // 0x6002a86e vmul.gt.f32 s0,s0,s17
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
            return 0x6002a872u32;
        }
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(836, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    0x6002a872
}
pub fn b_6002a872(c: &mut Machine) -> u32 {
    {
        // 0x6002a872 vldr.le.32 s0,[pc,#0x2ec]
        let mut t2200 = 0u64;
        let mut t2400 = 0u64;
        let mut t2700 = 0u64;
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610786930u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(748u64)) & 0xffffffffu64;
        t2200 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2400 = (((c.r(97, 1) != 0) | (((t2200 >> 0) & 0xffu64) != 0)) as u64) & 0xffu64;
        t2700 = ((((t2400 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x6002a876u32;
        }
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    0x6002a876
}
pub fn b_6002a876(c: &mut Machine) -> u32 {
    {
        // 0x6002a876 bl 0x60040c70
        let value = (1610786938u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002a87a
}
pub fn b_6002a87a(c: &mut Machine) -> u32 {
    {
        // 0x6002a87a vneg.f32 s0,s0
        let value = (-f32::from_bits(c.r(768, 4) as u32)).to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002a87e vldr.32 s17,[pc,#0x2e4]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610786942u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(740u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(836, 4, value);
    }
    {
        // 0x6002a882 vstr.32 s0,[r5,#0x31c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(796u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002a886 bl 0x6004137c
        let value = (1610786954u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x6004137cu32;
    }
    0x6002a88a
}
pub fn b_6002a88a(c: &mut Machine) -> u32 {
    {
        // 0x6002a88a vmov.f32 s15,0x3f800000
        let value = 1065353216u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002a88e vsub.f32 s0,s15,s0
        let value = (f32::from_bits(c.r(828, 4) as u32) - f32::from_bits(c.r(768, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002a892 bl 0x60041338
        let value = (1610786966u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041338u32;
    }
    0x6002a896
}
pub fn b_6002a896(c: &mut Machine) -> u32 {
    {
        // 0x6002a896 vldr.32 s15,[r5,#0x36c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(876u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002a89a vldr.32 s12,[r5,#0x370]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(880u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x6002a89e add.w r1,r4,#0x420
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
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002a8a2 vldr.32 s14,[pc,#0x2c4]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610786978u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(708u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002a8a6 add.w r3,r4,#0x4d0
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
        let value = ((c.r(48, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(44, 4, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002a8aa vmul.f32 s12,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x6002a8ae vldr.32 s13,[pc,#0x2bc]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610786990u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(700u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002a8b2 addw r2,r4,#0xc94
        let value = ((c.r(48, 4) as u128 + 3220u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 3220u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(3220u64);
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002a8b6 vldr.32 s6,[r1,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(792, 4, value);
    }
    {
        // 0x6002a8ba vmul.f32 s9,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(804, 4, value);
    }
    {
        // 0x6002a8be vldr.32 s7,[r3,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x6002a8c2 vmul.f32 s11,s12,s14
        let value = (f32::from_bits(c.r(816, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(812, 4, value);
    }
    {
        // 0x6002a8c6 vldr.32 s1,[r2]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(40, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(772, 4, value);
    }
    {
        // 0x6002a8ca vstr.32 s15,[r5,#0x368]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(872u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002a8ce addw r2,r4,#0xc88
        let value = ((c.r(48, 4) as u128 + 3208u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 3208u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(3208u64);
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002a8d2 vstr.32 s9,[r5,#0x364]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(868u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(804, 4));
    }
    {
        // 0x6002a8d6 vmul.f32 s15,s9,s13
        let value = (f32::from_bits(c.r(804, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002a8da vmul.f32 s10,s11,s14
        let value = (f32::from_bits(c.r(812, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x6002a8de vldr.32 s8,[pc,#0x290]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610787038u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(656u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(800, 4, value);
    }
    {
        // 0x6002a8e2 vldr.32 s5,[pc,#0x290]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610787042u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(656u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(788, 4, value);
    }
    {
        // 0x6002a8e6 vcvt.f32.s32 s1,s1
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
        let value = c.userop(219, c.r(772, 4), ((t3b500 >> 0) & 0xffu64), 0u64, 4, 4);
        c.w(772, 4, value);
    }
    {
        // 0x6002a8ea vstr.32 s15,[r5,#0x360]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(864u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002a8ee vmul.f32 s4,s15,s13
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(784, 4, value);
    }
    {
        // 0x6002a8f2 vmul.f32 s9,s10,s14
        let value = (f32::from_bits(c.r(808, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(804, 4, value);
    }
    {
        // 0x6002a8f6 vldr.32 s15,[pc,#0x280]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610787062u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(640u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002a8fa vfma.f32 s5,s0,s8
        let value = f32::from_bits(c.r(768, 4) as u32)
            .mul_add(
                f32::from_bits(c.r(800, 4) as u32),
                f32::from_bits(c.r(788, 4) as u32),
            )
            .to_bits() as u64;
        c.w(788, 4, value);
    }
    {
        // 0x6002a8fe vldr.32 s2,[r4,#0x164]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(356u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(776, 4, value);
    }
    {
        // 0x6002a902 vmul.f32 s0,s4,s13
        let value = (f32::from_bits(c.r(784, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002a906 vldr.32 s3,[r4,#0x214]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(532u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(780, 4, value);
    }
    {
        // 0x6002a90a vmul.f32 s8,s9,s14
        let value = (f32::from_bits(c.r(804, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(800, 4, value);
    }
    {
        // 0x6002a90e vstr.32 s4,[r5,#0x35c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(860u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(784, 4));
    }
    {
        // 0x6002a912 vstr.32 s12,[r5,#0x348]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(840u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x6002a916 addw r0,r4,#0xce8
        let value = ((c.r(48, 4) as u128 + 3304u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 3304u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(3304u64);
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002a91a vstr.32 s0,[r5,#0x358]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(856u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002a91e vmul.f32 s4,s0,s13
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(784, 4, value);
    }
    {
        // 0x6002a922 vmul.f32 s14,s8,s14
        let value = (f32::from_bits(c.r(800, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002a926 vstr.32 s11,[r5,#0x344]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(836u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(812, 4));
    }
    {
        // 0x6002a92a vstr.32 s5,[r5,#0x37c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(892u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(788, 4));
    }
    {
        // 0x6002a92e vmul.f32 s5,s4,s13
        let value = (f32::from_bits(c.r(784, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(788, 4, value);
    }
    {
        // 0x6002a932 vstr.32 s10,[r5,#0x340]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(832u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(808, 4));
    }
    {
        // 0x6002a936 vcmpe.f32 s14,s15
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
        // 0x6002a93a vstr.32 s9,[r5,#0x33c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(828u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(804, 4));
    }
    {
        // 0x6002a93e vstr.32 s8,[r5,#0x338]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(824u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(800, 4));
    }
    {
        // 0x6002a942 vmul.f32 s13,s5,s13
        let value = (f32::from_bits(c.r(788, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002a946 vstr.32 s14,[r5,#0x334]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(820u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002a94a vmrs apsr,fpscr
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
        // 0x6002a94e vstr.32 s4,[r5,#0x354]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(852u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(784, 4));
    }
    {
        // 0x6002a952 vldr.32 s4,[r4,#0x2c4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(708u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(784, 4, value);
    }
    {
        // 0x6002a956 vstr.32 s5,[r5,#0x350]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(848u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(788, 4));
    }
    {
        // 0x6002a95a vselgt.f32 s0,s14,s15
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
        // 0x6002a95e vcmpe.f32 s2,s0
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
        // 0x6002a962 vstr.32 s13,[r5,#0x34c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(52, 4)).wrapping_add(844u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x6002a966 vldr.32 s13,[r2,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002a96a vmrs apsr,fpscr
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
        // 0x6002a96e vldr.32 s5,[r4,#0x374]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(884u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(788, 4, value);
    }
    {
        // 0x6002a972 vcmpe.f32 s8,s15
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
        // 0x6002a976 vselgt.f32 s2,s0,s2
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
        // 0x6002a97a vstr.32 s2,[r4,#0x168]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(360u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(776, 4));
    }
    {
        // 0x6002a97e vmrs apsr,fpscr
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
        // 0x6002a982 vselgt.f32 s2,s8,s15
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
        // 0x6002a986 vcmpe.f32 s3,s2
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
        // 0x6002a98a vmrs apsr,fpscr
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
        // 0x6002a98e vcmpe.f32 s9,s15
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
        // 0x6002a992 vselgt.f32 s3,s2,s3
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
        // 0x6002a996 vstr.32 s3,[r4,#0x218]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(536u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(780, 4));
    }
    {
        // 0x6002a99a vmrs apsr,fpscr
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
        // 0x6002a99e vselgt.f32 s3,s9,s15
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
        c.w(780, 4, value);
    }
    {
        // 0x6002a9a2 vcmpe.f32 s4,s3
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
        // 0x6002a9a6 vmrs apsr,fpscr
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
        // 0x6002a9aa vcmpe.f32 s10,s15
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
        // 0x6002a9ae vselgt.f32 s4,s3,s4
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
        // 0x6002a9b2 vstr.32 s4,[r4,#0x2c8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(712u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(784, 4));
    }
    {
        // 0x6002a9b6 vmrs apsr,fpscr
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
        // 0x6002a9ba vselgt.f32 s4,s10,s15
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
        // 0x6002a9be vcmpe.f32 s5,s4
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
        // 0x6002a9c2 vmrs apsr,fpscr
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
        // 0x6002a9c6 vcmpe.f32 s11,s15
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
        // 0x6002a9ca vselgt.f32 s5,s4,s5
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
        // 0x6002a9ce vstr.32 s5,[r4,#0x378]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(888u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(788, 4));
    }
    {
        // 0x6002a9d2 vmrs apsr,fpscr
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
        // 0x6002a9d6 vselgt.f32 s5,s11,s15
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
        // 0x6002a9da vcmpe.f32 s6,s5
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
        // 0x6002a9de vmrs apsr,fpscr
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
        // 0x6002a9e2 vcmpe.f32 s12,s15
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
        // 0x6002a9e6 vselgt.f32 s6,s5,s6
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
        // 0x6002a9ea vstr.32 s6,[r1,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(792, 4));
    }
    {
        // 0x6002a9ee vmrs apsr,fpscr
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
        // 0x6002a9f2 addw r1,r4,#0xd48
        let value = ((c.r(48, 4) as u128 + 3400u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 3400u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(3400u64);
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002a9f6 vselgt.f32 s15,s12,s15
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
        // 0x6002a9fa vcmpe.f32 s7,s15
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
        // 0x6002a9fe vmrs apsr,fpscr
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
        // 0x6002aa02 vcmpe.f32 s14,s17
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
        t35c00 = ((f32::from_bits(c.r(824, 4) as u32) < f32::from_bits(c.r(836, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(824, 4) as u32) == f32::from_bits(c.r(836, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(836, 4) as u32) <= f32::from_bits(c.r(824, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(824, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002aa06 vselgt.f32 s7,s15,s7
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
        // 0x6002aa0a vstr.32 s7,[r3,#0x8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(796, 4));
    }
    {
        // 0x6002aa0e vmrs apsr,fpscr
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
        // 0x6002aa12 addw r3,r4,#0xe68
        let value = ((c.r(48, 4) as u128 + 3688u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 3688u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(3688u64);
        c.w(44, 4, value);
        let value = (signed(c.r(44, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(44, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002aa16 vcmpe.f32 s8,s17
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
        t35c00 = ((f32::from_bits(c.r(800, 4) as u32) < f32::from_bits(c.r(836, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(800, 4) as u32) == f32::from_bits(c.r(836, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(836, 4) as u32) <= f32::from_bits(c.r(800, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(800, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002aa1a vselgt.f32 s14,s14,s17
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(836, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(824, 4, value);
    }
    {
        // 0x6002aa1e vdiv.f32 s7,s1,s14
        let value = (f32::from_bits(c.r(772, 4) as u32) / f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(796, 4, value);
    }
    {
        // 0x6002aa22 vmrs apsr,fpscr
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
        // 0x6002aa26 vcmpe.f32 s9,s17
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
        t35c00 = ((f32::from_bits(c.r(804, 4) as u32) < f32::from_bits(c.r(836, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(804, 4) as u32) == f32::from_bits(c.r(836, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(836, 4) as u32) <= f32::from_bits(c.r(804, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(804, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002aa2a vselgt.f32 s8,s8,s17
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(836, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(800, 4, value);
    }
    {
        // 0x6002aa2e vmrs apsr,fpscr
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
        // 0x6002aa32 vcmpe.f32 s10,s17
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
        t35c00 = ((f32::from_bits(c.r(808, 4) as u32) < f32::from_bits(c.r(836, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(808, 4) as u32) == f32::from_bits(c.r(836, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(836, 4) as u32) <= f32::from_bits(c.r(808, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(808, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002aa36 vselgt.f32 s9,s9,s17
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(836, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(804, 4, value);
    }
    {
        // 0x6002aa3a vmrs apsr,fpscr
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
        // 0x6002aa3e vcmpe.f32 s11,s17
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
        t35c00 = ((f32::from_bits(c.r(812, 4) as u32) < f32::from_bits(c.r(836, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(812, 4) as u32) == f32::from_bits(c.r(836, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(836, 4) as u32) <= f32::from_bits(c.r(812, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(812, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002aa42 vmul.f32 s13,s7,s13
        let value = (f32::from_bits(c.r(796, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002aa46 vstr.32 s7,[r2,#0x1c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(796, 4));
    }
    {
        // 0x6002aa4a vselgt.f32 s10,s10,s17
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(836, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(808, 4, value);
    }
    {
        // 0x6002aa4e vmrs apsr,fpscr
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
        // 0x6002aa52 vcmpe.f32 s12,s17
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
        t35c00 = ((f32::from_bits(c.r(816, 4) as u32) < f32::from_bits(c.r(836, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(816, 4) as u32) == f32::from_bits(c.r(836, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(836, 4) as u32) <= f32::from_bits(c.r(816, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(816, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002aa56 vdiv.f32 s6,s18,s13
        let value = (f32::from_bits(c.r(840, 4) as u32) / f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(792, 4, value);
    }
    {
        // 0x6002aa5a vstr.32 s13,[r2,#0x20]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x6002aa5e vldr.32 s14,[r1,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002aa62 vselgt.f32 s11,s11,s17
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(836, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(812, 4, value);
    }
    {
        // 0x6002aa66 vldr.32 s7,[r0,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(796, 4, value);
    }
    {
        // 0x6002aa6a vldr.32 s15,[r3,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002aa6e vmrs apsr,fpscr
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
        // 0x6002aa72 vselgt.f32 s12,s12,s17
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
        tdca00 = (((tdc900 >> 0) & 0xffffffffu64).wrapping_mul(c.r(836, 4))) & 0xffffffffu64;
        let value = ((tdc700 >> 0) & 0xffffffffu64).wrapping_add(((tdca00 >> 0) & 0xffffffffu64));
        c.w(816, 4, value);
    }
    {
        // 0x6002aa76 vstr.32 s6,[r2,#0x24]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(792, 4));
    }
    {
        // 0x6002aa7a ldr.w r2,[r4,#0xcf4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(3316u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x6002aa7e vmov s13,r2
        let value = c.r(40, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002aa82 ldr.w r2,[r4,#0xd54]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(3412u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x6002aa86 vcvt.f32.s32 s6,s13
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
        // 0x6002aa8a vmov s13,r2
        let value = c.r(40, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002aa8e ldr.w r2,[r4,#0xdb4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(3508u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(40, 4, value);
    }
    {
        // 0x6002aa92 vcvt.f32.s32 s13,s13
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
        // 0x6002aa96 vdiv.f32 s4,s6,s8
        let value = (f32::from_bits(c.r(792, 4) as u32) / f32::from_bits(c.r(800, 4) as u32))
            .to_bits() as u64;
        c.w(784, 4, value);
    }
    {
        // 0x6002aa9a vmov s8,r2
        let value = c.r(40, 4);
        c.w(800, 4, value);
    }
    {
        // 0x6002aa9e addw r2,r4,#0xda8
        let value = ((c.r(48, 4) as u128 + 3496u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 3496u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(3496u64);
        c.w(40, 4, value);
        let value = (signed(c.r(40, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(40, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002aaa2 vdiv.f32 s5,s13,s9
        let value = (f32::from_bits(c.r(820, 4) as u32) / f32::from_bits(c.r(804, 4) as u32))
            .to_bits() as u64;
        c.w(788, 4, value);
    }
    {
        // 0x6002aaa6 vldr.32 s13,[r2,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002aaaa vcvt.f32.s32 s8,s8
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
        // 0x6002aaae vdiv.f32 s6,s8,s10
        let value = (f32::from_bits(c.r(800, 4) as u32) / f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(792, 4, value);
    }
    {
        // 0x6002aab2 vstr.32 s4,[r0,#0x1c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(784, 4));
    }
    {
        // 0x6002aab6 vmul.f32 s10,s4,s7
        let value = (f32::from_bits(c.r(784, 4) as u32) * f32::from_bits(c.r(796, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x6002aaba vmul.f32 s14,s5,s14
        let value = (f32::from_bits(c.r(788, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002aabe vstr.32 s10,[r0,#0x20]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(808, 4));
    }
    {
        // 0x6002aac2 vdiv.f32 s9,s18,s10
        let value = (f32::from_bits(c.r(840, 4) as u32) / f32::from_bits(c.r(808, 4) as u32))
            .to_bits() as u64;
        c.w(804, 4, value);
    }
    {
        // 0x6002aac6 vdiv.f32 s10,s18,s14
        let value = (f32::from_bits(c.r(840, 4) as u32) / f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(808, 4, value);
    }
    {
        // 0x6002aaca vmul.f32 s13,s6,s13
        let value = (f32::from_bits(c.r(792, 4) as u32) * f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002aace vdiv.f32 s8,s18,s13
        let value = (f32::from_bits(c.r(840, 4) as u32) / f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(800, 4, value);
    }
    {
        // 0x6002aad2 vstr.32 s9,[r0,#0x24]
        let mut t64000 = 0u64;
        t64000 = ((c.r(32, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(804, 4));
    }
    {
        // 0x6002aad6 vstr.32 s14,[r1,#0x20]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002aada vstr.32 s5,[r1,#0x1c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(788, 4));
    }
    {
        // 0x6002aade vstr.32 s10,[r1,#0x24]
        let mut t64000 = 0u64;
        t64000 = ((c.r(36, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(808, 4));
    }
    {
        // 0x6002aae2 addw r1,r4,#0xe14
        let value = ((c.r(48, 4) as u128 + 3604u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 3604u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(3604u64);
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002aae6 vldr.32 s9,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(804, 4, value);
    }
    {
        // 0x6002aaea addw r1,r4,#0xe74
        let value = ((c.r(48, 4) as u128 + 3700u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 3700u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(3700u64);
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002aaee addw r4,r4,#0xe08
        let value = ((c.r(48, 4) as u128 + 3592u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 3592u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(3592u64);
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002aaf2 vldr.32 s10,[r1]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x6002aaf6 vcvt.f32.s32 s9,s9
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
        // 0x6002aafa vldr.32 s14,[r4,#0x4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002aafe vcvt.f32.s32 s10,s10
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
        // 0x6002ab02 vstr.32 s13,[r2,#0x20]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x6002ab06 vstr.32 s6,[r2,#0x1c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(792, 4));
    }
    {
        // 0x6002ab0a vstr.32 s8,[r2,#0x24]
        let mut t64000 = 0u64;
        t64000 = ((c.r(40, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(800, 4));
    }
    {
        // 0x6002ab0e vdiv.f32 s8,s9,s11
        let value = (f32::from_bits(c.r(804, 4) as u32) / f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(800, 4, value);
    }
    {
        // 0x6002ab12 vdiv.f32 s13,s10,s12
        let value = (f32::from_bits(c.r(808, 4) as u32) / f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002ab16 vmul.f32 s14,s8,s14
        let value = (f32::from_bits(c.r(800, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002ab1a vstr.32 s8,[r4,#0x1c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(800, 4));
    }
    {
        // 0x6002ab1e vstr.32 s14,[r4,#0x20]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002ab22 vdiv.f32 s12,s18,s14
        let value = (f32::from_bits(c.r(840, 4) as u32) / f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x6002ab26 vmul.f32 s15,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002ab2a vdiv.f32 s14,s18,s15
        let value = (f32::from_bits(c.r(840, 4) as u32) / f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002ab2e vpop {d8,d9}
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
        // 0x6002ab32 vstr.32 s12,[r4,#0x24]
        let mut t64000 = 0u64;
        t64000 = ((c.r(48, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x6002ab36 vstr.32 s13,[r3,#0x1c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x6002ab3a vstr.32 s15,[r3,#0x20]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002ab3e vstr.32 s14,[r3,#0x24]
        let mut t64000 = 0u64;
        t64000 = ((c.r(44, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002ab42 pop {r3,r4,r5,pc}
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
    0x6002ab44
}
pub fn b_6002ab80(c: &mut Machine) -> u32 {
    {
        // 0x6002ab80 push {r4,r5,r6,r7,r8,r9,r10,r11,lr}
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
        // 0x6002ab84 ldr.w r11,[0x6002af4c]
        let value = c.load(0x6002af4c, 4);
        c.w(76, 4, value);
    }
    {
        // 0x6002ab88 movs r4,#0x0
        let value = 0u64;
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
    }
    {
        // 0x6002ab8a movs r5,#0x0
        let value = 0u64;
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
        // 0x6002ab8c vldr.32 s0,[pc,#0x3a8]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610787724u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(936u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002ab90 add.w r10,r11,#0x1000
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
        let value = ((c.r(76, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(76, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(76, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(72, 4, value);
        let value = (signed(c.r(72, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(72, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002ab94 addw r0,r11,#0xc1c
        let value = ((c.r(76, 4) as u128 + 3100u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(76, 4), 3100u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(76, 4)).wrapping_add(3100u64);
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002ab98 addw r6,r11,#0xc7c
        let value = ((c.r(76, 4) as u128 + 3196u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(76, 4), 3196u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(76, 4)).wrapping_add(3196u64);
        c.w(56, 4, value);
        let value = (signed(c.r(56, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(56, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002ab9c vpush {d8,d9,d10}
        let mut t73700 = 0u64;
        t73700 = ((6u64).wrapping_mul(4u64)) & 0xffffffffu64;
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
    }
    {
        // 0x6002aba0 str.w r4,[r11,#0x48]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(72u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002aba4 sub sp,#0x54
        let mut t187500 = 0u64;
        t187500 = (84u64) & 0xffffffffu64;
        let value = (c.r(84, 4)).wrapping_sub(((t187500 >> 0) & 0xffffffffu64));
        c.w(84, 4, value);
    }
    {
        // 0x6002aba6 str.w r4,[r11,#0x80]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(128u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002abaa str.w r4,[r11,#0xf8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(248u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002abae str.w r4,[r11,#0x130]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(304u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002abb2 str.w r4,[r11,#0x1a8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(424u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002abb6 str.w r4,[r11,#0x1e0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(480u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002abba str.w r4,[r11,#0x258]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(600u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002abbe str.w r4,[r11,#0x290]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(656u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002abc2 str.w r4,[r11,#0x308]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(776u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002abc6 str.w r4,[r11,#0x340]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(832u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002abca str.w r4,[r11,#0x3b8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(952u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002abce str.w r4,[r11,#0x3f0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1008u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002abd2 str.w r4,[r11,#0x468]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1128u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002abd6 str.w r4,[r11,#0x4a0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1184u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002abda str.w r4,[r11,#0x518]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1304u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002abde str.w r4,[r11,#0x550]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1360u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002abe2 str.w r4,[r11,#0x5c8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1480u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002abe6 str.w r4,[r11,#0x600]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1536u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002abea str.w r4,[r11,#0x678]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1656u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002abee str.w r4,[r11,#0x6b0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1712u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002abf2 str.w r4,[r11,#0x728]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1832u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002abf6 str.w r4,[r11,#0x760]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1888u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002abfa str.w r4,[r11,#0x7d8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2008u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002abfe str.w r4,[r11,#0x810]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2064u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ac02 str.w r4,[r11,#0x888]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2184u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ac06 str.w r4,[r11,#0x8c0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2240u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ac0a str.w r4,[r11,#0x938]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2360u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ac0e str.w r4,[r11,#0x970]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2416u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ac12 str.w r4,[r11,#0x9e8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2536u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ac16 str.w r4,[r11,#0xa20]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2592u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ac1a str.w r4,[r11,#0xa98]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2712u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ac1e ldr.w r1,[r10,#0x314]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(788u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002ac22 str r5,[r0,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(32, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002ac24 add.w r0,r1,#0x2000
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (8192u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(36, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002ac28 str r5,[r6,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002ac2a addw r6,r11,#0xcdc
        let value = ((c.r(76, 4) as u128 + 3292u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(76, 4), 3292u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(76, 4)).wrapping_add(3292u64);
        c.w(56, 4, value);
        let value = (signed(c.r(56, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(56, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002ac2e str.w r4,[r11,#0xad0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2768u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ac32 str.w r0,[r11,#0xc40]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3136u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002ac36 add.w r0,r1,#0x4000
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (16384u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(36, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002ac3a str r5,[r6,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002ac3c addw r6,r11,#0xd3c
        let value = ((c.r(76, 4) as u128 + 3388u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(76, 4), 3388u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(76, 4)).wrapping_add(3388u64);
        c.w(56, 4, value);
        let value = (signed(c.r(56, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(56, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002ac40 str.w r0,[r11,#0xca0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3232u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002ac44 add.w r0,r1,#0x6000
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (24576u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(36, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002ac48 str r5,[r6,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002ac4a addw r6,r11,#0xd9c
        let value = ((c.r(76, 4) as u128 + 3484u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(76, 4), 3484u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(76, 4)).wrapping_add(3484u64);
        c.w(56, 4, value);
        let value = (signed(c.r(56, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(56, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002ac4e str.w r0,[r11,#0xd00]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3328u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002ac52 add.w r0,r1,#0x8000
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (32768u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(36, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002ac56 str r5,[r6,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002ac58 addw r6,r11,#0xdfc
        let value = ((c.r(76, 4) as u128 + 3580u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(76, 4), 3580u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(76, 4)).wrapping_add(3580u64);
        c.w(56, 4, value);
        let value = (signed(c.r(56, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(56, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002ac5c str.w r0,[r11,#0xd60]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3424u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002ac60 add.w r0,r1,#0xa000
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (40960u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(36, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002ac64 str r5,[r6,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002ac66 addw r6,r11,#0xe5c
        let value = ((c.r(76, 4) as u128 + 3676u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(76, 4), 3676u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(76, 4)).wrapping_add(3676u64);
        c.w(56, 4, value);
        let value = (signed(c.r(56, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(56, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002ac6a str.w r0,[r11,#0xdc0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3520u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002ac6e add.w r0,r1,#0xc000
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (49152u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(36, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002ac72 str r5,[r6,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002ac74 addw r6,r11,#0xebc
        let value = ((c.r(76, 4) as u128 + 3772u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(76, 4), 3772u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(76, 4)).wrapping_add(3772u64);
        c.w(56, 4, value);
        let value = (signed(c.r(56, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(56, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002ac78 str.w r0,[r11,#0xe20]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3616u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002ac7c add.w r0,r1,#0xe000
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (57344u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(36, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002ac80 str r5,[r6,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002ac82 addw r6,r11,#0xf1c
        let value = ((c.r(76, 4) as u128 + 3868u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(76, 4), 3868u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(76, 4)).wrapping_add(3868u64);
        c.w(56, 4, value);
        let value = (signed(c.r(56, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(56, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002ac86 str.w r0,[r11,#0xe80]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3712u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002ac8a add.w r0,r1,#0x10000
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (65536u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(36, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002ac8e str r5,[r6,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002ac90 addw r6,r11,#0xf7c
        let value = ((c.r(76, 4) as u128 + 3964u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(76, 4), 3964u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(76, 4)).wrapping_add(3964u64);
        c.w(56, 4, value);
        let value = (signed(c.r(56, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(56, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002ac94 str.w r0,[r11,#0xee0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3808u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002ac98 add.w r0,r1,#0x12000
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (73728u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(36, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002ac9c str.w r4,[r11,#0xb5c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2908u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002aca0 str.w r0,[r11,#0xf40]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3904u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002aca4 add.w r0,r1,#0x14000
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (81920u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(36, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002aca8 str.w r4,[r11,#0xbc0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3008u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002acac str.w r0,[r11,#0xfa0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(4000u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002acb0 add.w r0,r1,#0x16000
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (90112u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(36, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002acb4 str.w r1,[r11,#0xbe0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3040u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002acb8 str.w r0,[r10,#0x0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002acbc add.w r0,r1,#0x18000
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (98304u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(36, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002acc0 str.w r5,[r10,#0x3c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(60u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002acc4 str.w r0,[r10,#0x60]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(96u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002acc8 add.w r0,r1,#0x1a000
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (106496u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(36, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002accc str.w r5,[r10,#0x9c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(156u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002acd0 str.w r0,[r10,#0xc0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(192u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002acd4 add.w r0,r1,#0x1c000
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (114688u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(36, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002acd8 str.w r5,[r10,#0xfc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(252u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002acdc add.w r1,r1,#0x1e000
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (122880u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((c.r(36, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002ace0 str.w r0,[r10,#0x120]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(288u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002ace4 str.w r5,[r10,#0x15c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(348u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002ace8 ldr r0,[0x6002af3c]
        let value = c.load(0x6002af3c, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002acea str r5,[r6,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002acec addw r6,r11,#0xfdc
        let value = ((c.r(76, 4) as u128 + 4060u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(76, 4), 4060u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(76, 4)).wrapping_add(4060u64);
        c.w(56, 4, value);
        let value = (signed(c.r(56, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(56, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002acf0 str.w r1,[r10,#0x180]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(384u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002acf4 str.w r0,[r10,#0x318]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(792u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002acf8 str.w r0,[r11,#0x18]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(24u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002acfc str r5,[r6,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002acfe str.w r5,[r10,#0x1bc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(444u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002ad02 bl 0x60040e08
        let value = (1610788102u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040e08u32;
    }
    0x6002ad06
}
pub fn b_6002ad06(c: &mut Machine) -> u32 {
    {
        // 0x6002ad06 bl 0x60041408
        let value = (1610788106u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041408u32;
    }
    0x6002ad0a
}
pub fn b_6002ad0a(c: &mut Machine) -> u32 {
    {
        // 0x6002ad0a bl 0x60040c70
        let value = (1610788110u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002ad0e
}
pub fn b_6002ad0e(c: &mut Machine) -> u32 {
    {
        // 0x6002ad0e vcvt.u32.f32 s15,s0
        let mut tade00 = 0u64;
        let mut tadf00 = 0u64;
        let mut tae000 = 0u64;
        tade00 =
            ((f32::from_bits(0u64 as u32) < f32::from_bits(c.r(768, 4) as u32)) as u64) & 0xffu64;
        tadf00 = ((tade00 >> 0) & 0xffu64) & 0xffffffffu64;
        tae000 = ((f32::from_bits(c.r(768, 4) as u32) as i32) as u64) & 0xffffffffu64;
        let value = ((tadf00 >> 0) & 0xffffffffu64).wrapping_mul(((tae000 >> 0) & 0xffffffffu64));
        c.w(828, 4, value);
    }
    {
        // 0x6002ad12 str.w r4,[r11,#0x2c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ad16 str.w r4,[r11,#0x38]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ad1a vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002ad1e str.w r4,[r11,#0x44]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ad22 ldr.w r0,[r11,#0x48]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(72u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002ad26 subs r4,r1,#0x1
        let value = (1u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 1u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(1u64);
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
        // 0x6002ad28 str.w r5,[r11,#0x28]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002ad2c str.w r5,[r11,#0x30]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002ad30 str.w r4,[r11,#0x20]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ad34 subs r4,r1,#0x2
        let value = (2u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 2u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(2u64);
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
        // 0x6002ad36 str.w r5,[r11,#0x34]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002ad3a vstr.32 s15,[r11,#0x1c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(76, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002ad3e str.w r4,[r11,#0x24]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ad42 cbz r0,0x6002ad4c
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x6002ad4cu32;
        }
    }
    0x6002ad44
}
pub fn b_6002ad44(c: &mut Machine) -> u32 {
    {
        // 0x6002ad44 bl 0x600414d0
        let value = (1610788168u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414d0u32;
    }
    0x6002ad48
}
pub fn b_6002ad48(c: &mut Machine) -> u32 {
    {
        // 0x6002ad48 ldr.w r1,[r11,#0x1c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x6002ad4c
}
pub fn b_6002ad4c(c: &mut Machine) -> u32 {
    {
        // 0x6002ad4c lsls r0,r1,#0x2
        let mut t1afd00 = 0u64;
        let mut t1afe00 = 0u64;
        let mut t1b0000 = 0u64;
        let mut t1b0100 = 0u64;
        let mut t1b0200 = 0u64;
        let mut t1b0300 = 0u64;
        let mut t1b0400 = 0u64;
        let mut t1b0500 = 0u64;
        t1afd00 = ((2u64).wrapping_sub(1u64)) & 0xffffffffu64;
        t1afe00 = (if ((t1afd00 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(36, 4)) << (((t1afd00 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t1b0000 = (((t1afe00 >> 0) & 0xffffffffu64) & 2147483648u64) & 0xffffffffu64;
        t1b0100 = ((2u64 == 0u64) as u64) & 0xffu64;
        t1b0200 = (((((t1b0100 >> 0) & 0xffu64) != 0) & (c.r(98, 1) != 0)) as u64) & 0xffu64;
        t1b0300 = ((2u64 != 0u64) as u64) & 0xffu64;
        t1b0400 = ((((t1b0000 >> 0) & 0xffffffffu64) != 0u64) as u64) & 0xffu64;
        t1b0500 = (((((t1b0300 >> 0) & 0xffu64) != 0) & (((t1b0400 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        let value = ((((t1b0200 >> 0) & 0xffu64) != 0) | (((t1b0500 >> 0) & 0xffu64) != 0)) as u64;
        c.w(102, 1, value);
        let value = if 2u64 >= 32 {
            0
        } else {
            (c.r(36, 4)) << (2u64 as u32)
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
        // 0x6002ad4e movs r4,#0x0
        let value = 0u64;
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
    }
    {
        // 0x6002ad50 bl 0x600414c0
        let value = (1610788180u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414c0u32;
    }
    0x6002ad54
}
pub fn b_6002ad54(c: &mut Machine) -> u32 {
    {
        // 0x6002ad54 mvn r5,#0x2
        let mut t187d00 = 0u64;
        t187d00 = (2u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = !((t187d00 >> 0) & 0xffffffffu64);
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002ad58 ldr r1,[0x6002af3c]
        let value = c.load(0x6002af3c, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002ad5a vldr.32 s0,[pc,#0x1dc]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610788186u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(476u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002ad5e str.w r0,[r11,#0x48]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(72u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002ad62 str.w r1,[r11,#0x50]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(80u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002ad66 str.w r4,[r11,#0x40]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ad6a str.w r4,[r11,#0x4c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(76u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ad6e str.w r5,[r11,#0x3c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(60u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002ad72 bl 0x60040e08
        let value = (1610788214u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040e08u32;
    }
    0x6002ad76
}
pub fn b_6002ad76(c: &mut Machine) -> u32 {
    {
        // 0x6002ad76 bl 0x60041408
        let value = (1610788218u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041408u32;
    }
    0x6002ad7a
}
pub fn b_6002ad7a(c: &mut Machine) -> u32 {
    {
        // 0x6002ad7a bl 0x60040c70
        let value = (1610788222u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002ad7e
}
pub fn b_6002ad7e(c: &mut Machine) -> u32 {
    {
        // 0x6002ad7e vcvt.u32.f32 s15,s0
        let mut tade00 = 0u64;
        let mut tadf00 = 0u64;
        let mut tae000 = 0u64;
        tade00 =
            ((f32::from_bits(0u64 as u32) < f32::from_bits(c.r(768, 4) as u32)) as u64) & 0xffu64;
        tadf00 = ((tade00 >> 0) & 0xffu64) & 0xffffffffu64;
        tae000 = ((f32::from_bits(c.r(768, 4) as u32) as i32) as u64) & 0xffffffffu64;
        let value = ((tadf00 >> 0) & 0xffffffffu64).wrapping_mul(((tae000 >> 0) & 0xffffffffu64));
        c.w(828, 4, value);
    }
    {
        // 0x6002ad82 str.w r4,[r11,#0x60]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(96u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ad86 str.w r4,[r11,#0x68]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(104u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ad8a vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002ad8e str.w r4,[r11,#0x6c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(108u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ad92 ldr.w r0,[r11,#0x80]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(128u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002ad96 subs r4,r1,#0x1
        let value = (1u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 1u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(1u64);
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
        // 0x6002ad98 strd r1,r4,[r11,#0x54]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(76, 4)).wrapping_add(84u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ad9c subs r4,r1,#0x2
        let value = (2u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 2u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(2u64);
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
        // 0x6002ad9e str.w r4,[r11,#0x5c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(92u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ada2 movs r4,#0x0
        let value = 0u64;
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
    }
    {
        // 0x6002ada4 str.w r4,[r11,#0x64]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(100u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ada8 str.w r4,[r11,#0x70]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(112u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002adac str.w r4,[r11,#0x7c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(124u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002adb0 cbz r0,0x6002adba
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x6002adbau32;
        }
    }
    0x6002adb2
}
pub fn b_6002adb2(c: &mut Machine) -> u32 {
    {
        // 0x6002adb2 bl 0x600414d0
        let value = (1610788278u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414d0u32;
    }
    0x6002adb6
}
pub fn b_6002adb6(c: &mut Machine) -> u32 {
    {
        // 0x6002adb6 ldr.w r1,[r11,#0x54]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(84u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x6002adba
}
pub fn b_6002adba(c: &mut Machine) -> u32 {
    {
        // 0x6002adba lsls r0,r1,#0x2
        let mut t1afd00 = 0u64;
        let mut t1afe00 = 0u64;
        let mut t1b0000 = 0u64;
        let mut t1b0100 = 0u64;
        let mut t1b0200 = 0u64;
        let mut t1b0300 = 0u64;
        let mut t1b0400 = 0u64;
        let mut t1b0500 = 0u64;
        t1afd00 = ((2u64).wrapping_sub(1u64)) & 0xffffffffu64;
        t1afe00 = (if ((t1afd00 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(36, 4)) << (((t1afd00 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t1b0000 = (((t1afe00 >> 0) & 0xffffffffu64) & 2147483648u64) & 0xffffffffu64;
        t1b0100 = ((2u64 == 0u64) as u64) & 0xffu64;
        t1b0200 = (((((t1b0100 >> 0) & 0xffu64) != 0) & (c.r(98, 1) != 0)) as u64) & 0xffu64;
        t1b0300 = ((2u64 != 0u64) as u64) & 0xffu64;
        t1b0400 = ((((t1b0000 >> 0) & 0xffffffffu64) != 0u64) as u64) & 0xffu64;
        t1b0500 = (((((t1b0300 >> 0) & 0xffu64) != 0) & (((t1b0400 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        let value = ((((t1b0200 >> 0) & 0xffu64) != 0) | (((t1b0500 >> 0) & 0xffu64) != 0)) as u64;
        c.w(102, 1, value);
        let value = if 2u64 >= 32 {
            0
        } else {
            (c.r(36, 4)) << (2u64 as u32)
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
        // 0x6002adbc movs r4,#0x0
        let value = 0u64;
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
    }
    {
        // 0x6002adbe bl 0x600414c0
        let value = (1610788290u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414c0u32;
    }
    0x6002adc2
}
pub fn b_6002adc2(c: &mut Machine) -> u32 {
    {
        // 0x6002adc2 mvn r1,#0x2
        let mut t187d00 = 0u64;
        t187d00 = (2u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = !((t187d00 >> 0) & 0xffffffffu64);
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002adc6 str.w r0,[r11,#0x80]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(128u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002adca movs r0,#0x0
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
        // 0x6002adcc str.w r1,[r11,#0x74]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(116u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002add0 mov.w r5,#0x3f800000
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
        let value = ((t188100 >> 0) & 0xffffffffu64);
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002add4 str.w r1,[r11,#0x3c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(60u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002add8 movs r1,#0x0
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
        // 0x6002adda vldr.32 s0,[pc,#0x15c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610788314u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(348u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002adde str.w r4,[r11,#0x78]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(120u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ade2 str.w r4,[r11,#0x84]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(132u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ade6 str.w r4,[r11,#0x4c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(76u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002adea str.w r4,[r11,#0xa4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(164u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002adee str.w r4,[r11,#0xa8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(168u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002adf2 str.w r5,[r11,#0x4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002adf6 strd r0,r1,[r11,#0x10]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(76, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002adfa mov.w r1,#0x3f000000
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
        // 0x6002adfe str.w r1,[r11,#0x8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002ae02 str.w r1,[r11,#0xc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(12u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002ae06 str.w r1,[r11,#0xa0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(160u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002ae0a str.w r1,[r11,#0x40]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002ae0e adr r1,[0x6002af30]
        let value = 1610788656u64;
        c.w(36, 4, value);
    }
    {
        // 0x6002ae10 ldrd r0,r1,[r1,#0x0]
        let mut t19dc00 = 0u64;
        let mut t1ab800 = 0u64;
        t19dc00 = ((c.r(36, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19dc00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
        t1ab800 = (((t19dc00 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t1ab800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002ae14 strd r0,r1,[r11,#0x88]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(76, 4)).wrapping_add(136u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002ae18 ldr r0,[0x6002af40]
        let value = c.load(0x6002af40, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002ae1a ldr r1,[0x6002af3c]
        let value = c.load(0x6002af3c, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002ae1c str.w r0,[r11,#0x98]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(152u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002ae20 ldr r0,[0x6002af44]
        let value = c.load(0x6002af44, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002ae22 str.w r1,[r11,#0x0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002ae26 str.w r1,[r11,#0xc8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(200u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002ae2a str.w r0,[r11,#0x9c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(156u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002ae2e movs r0,#0x0
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
        // 0x6002ae30 ldr r1,[0x6002af48]
        let value = c.load(0x6002af48, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002ae32 strd r0,r1,[r11,#0x90]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(76, 4)).wrapping_add(144u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002ae36 bl 0x60040e08
        let value = (1610788410u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040e08u32;
    }
    0x6002ae3a
}
pub fn b_6002ae3a(c: &mut Machine) -> u32 {
    {
        // 0x6002ae3a bl 0x60041408
        let value = (1610788414u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041408u32;
    }
    0x6002ae3e
}
pub fn b_6002ae3e(c: &mut Machine) -> u32 {
    {
        // 0x6002ae3e bl 0x60040c70
        let value = (1610788418u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002ae42
}
pub fn b_6002ae42(c: &mut Machine) -> u32 {
    {
        // 0x6002ae42 vcvt.u32.f32 s15,s0
        let mut tade00 = 0u64;
        let mut tadf00 = 0u64;
        let mut tae000 = 0u64;
        tade00 =
            ((f32::from_bits(0u64 as u32) < f32::from_bits(c.r(768, 4) as u32)) as u64) & 0xffu64;
        tadf00 = ((tade00 >> 0) & 0xffu64) & 0xffffffffu64;
        tae000 = ((f32::from_bits(c.r(768, 4) as u32) as i32) as u64) & 0xffffffffu64;
        let value = ((tadf00 >> 0) & 0xffffffffu64).wrapping_mul(((tae000 >> 0) & 0xffffffffu64));
        c.w(828, 4, value);
    }
    {
        // 0x6002ae46 str.w r4,[r11,#0xd8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(216u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ae4a str.w r4,[r11,#0xe0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(224u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ae4e vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002ae52 str.w r4,[r11,#0xe4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(228u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ae56 ldr.w r0,[r11,#0xf8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(248u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002ae5a subs r4,r1,#0x1
        let value = (1u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 1u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(1u64);
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
        // 0x6002ae5c strd r1,r4,[r11,#0xcc]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(76, 4)).wrapping_add(204u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ae60 subs r4,r1,#0x2
        let value = (2u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 2u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(2u64);
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
        // 0x6002ae62 str.w r4,[r11,#0xd4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(212u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ae66 movs r4,#0x0
        let value = 0u64;
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
    }
    {
        // 0x6002ae68 str.w r4,[r11,#0xdc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(220u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ae6c str.w r4,[r11,#0xe8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(232u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ae70 str.w r4,[r11,#0xf4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(244u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ae74 cbz r0,0x6002ae7e
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x6002ae7eu32;
        }
    }
    0x6002ae76
}
pub fn b_6002ae76(c: &mut Machine) -> u32 {
    {
        // 0x6002ae76 bl 0x600414d0
        let value = (1610788474u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414d0u32;
    }
    0x6002ae7a
}
pub fn b_6002ae7a(c: &mut Machine) -> u32 {
    {
        // 0x6002ae7a ldr.w r1,[r11,#0xcc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(204u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x6002ae7e
}
pub fn b_6002ae7e(c: &mut Machine) -> u32 {
    {
        // 0x6002ae7e lsls r0,r1,#0x2
        let mut t1afd00 = 0u64;
        let mut t1afe00 = 0u64;
        let mut t1b0000 = 0u64;
        let mut t1b0100 = 0u64;
        let mut t1b0200 = 0u64;
        let mut t1b0300 = 0u64;
        let mut t1b0400 = 0u64;
        let mut t1b0500 = 0u64;
        t1afd00 = ((2u64).wrapping_sub(1u64)) & 0xffffffffu64;
        t1afe00 = (if ((t1afd00 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(36, 4)) << (((t1afd00 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t1b0000 = (((t1afe00 >> 0) & 0xffffffffu64) & 2147483648u64) & 0xffffffffu64;
        t1b0100 = ((2u64 == 0u64) as u64) & 0xffu64;
        t1b0200 = (((((t1b0100 >> 0) & 0xffu64) != 0) & (c.r(98, 1) != 0)) as u64) & 0xffu64;
        t1b0300 = ((2u64 != 0u64) as u64) & 0xffu64;
        t1b0400 = ((((t1b0000 >> 0) & 0xffffffffu64) != 0u64) as u64) & 0xffu64;
        t1b0500 = (((((t1b0300 >> 0) & 0xffu64) != 0) & (((t1b0400 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        let value = ((((t1b0200 >> 0) & 0xffu64) != 0) | (((t1b0500 >> 0) & 0xffu64) != 0)) as u64;
        c.w(102, 1, value);
        let value = if 2u64 >= 32 {
            0
        } else {
            (c.r(36, 4)) << (2u64 as u32)
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
        // 0x6002ae80 movs r4,#0x0
        let value = 0u64;
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
    }
    {
        // 0x6002ae82 bl 0x600414c0
        let value = (1610788486u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414c0u32;
    }
    0x6002ae86
}
pub fn b_6002ae86(c: &mut Machine) -> u32 {
    {
        // 0x6002ae86 mvn r5,#0x2
        let mut t187d00 = 0u64;
        t187d00 = (2u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = !((t187d00 >> 0) & 0xffffffffu64);
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002ae8a ldr r1,[0x6002af3c]
        let value = c.load(0x6002af3c, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002ae8c vldr.32 s0,[pc,#0xa8]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610788492u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(168u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002ae90 str.w r0,[r11,#0xf8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(248u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002ae94 str.w r1,[r11,#0x100]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(256u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002ae98 str.w r4,[r11,#0xf0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(240u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ae9c str.w r4,[r11,#0xfc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(252u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002aea0 str.w r5,[r11,#0xec]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(236u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002aea4 bl 0x60040e08
        let value = (1610788520u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040e08u32;
    }
    0x6002aea8
}
pub fn b_6002aea8(c: &mut Machine) -> u32 {
    {
        // 0x6002aea8 bl 0x60041408
        let value = (1610788524u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041408u32;
    }
    0x6002aeac
}
pub fn b_6002aeac(c: &mut Machine) -> u32 {
    {
        // 0x6002aeac bl 0x60040c70
        let value = (1610788528u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002aeb0
}
pub fn b_6002aeb0(c: &mut Machine) -> u32 {
    {
        // 0x6002aeb0 vcvt.u32.f32 s15,s0
        let mut tade00 = 0u64;
        let mut tadf00 = 0u64;
        let mut tae000 = 0u64;
        tade00 =
            ((f32::from_bits(0u64 as u32) < f32::from_bits(c.r(768, 4) as u32)) as u64) & 0xffu64;
        tadf00 = ((tade00 >> 0) & 0xffu64) & 0xffffffffu64;
        tae000 = ((f32::from_bits(c.r(768, 4) as u32) as i32) as u64) & 0xffffffffu64;
        let value = ((tadf00 >> 0) & 0xffffffffu64).wrapping_mul(((tae000 >> 0) & 0xffffffffu64));
        c.w(828, 4, value);
    }
    {
        // 0x6002aeb4 str.w r4,[r11,#0x110]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(272u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002aeb8 str.w r4,[r11,#0x118]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(280u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002aebc vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002aec0 str.w r4,[r11,#0x11c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(284u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002aec4 ldr.w r0,[r11,#0x130]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(304u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002aec8 subs r4,r1,#0x1
        let value = (1u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 1u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(1u64);
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
        // 0x6002aeca strd r1,r4,[r11,#0x104]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(76, 4)).wrapping_add(260u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002aece subs r4,r1,#0x2
        let value = (2u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 2u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(2u64);
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
        // 0x6002aed0 str.w r4,[r11,#0x10c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(268u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002aed4 movs r4,#0x0
        let value = 0u64;
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
    }
    {
        // 0x6002aed6 str.w r4,[r11,#0x114]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(276u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002aeda str.w r4,[r11,#0x120]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(288u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002aede str.w r4,[r11,#0x12c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(300u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002aee2 cbz r0,0x6002aeec
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x6002aeecu32;
        }
    }
    0x6002aee4
}
pub fn b_6002aee4(c: &mut Machine) -> u32 {
    {
        // 0x6002aee4 bl 0x600414d0
        let value = (1610788584u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414d0u32;
    }
    0x6002aee8
}
pub fn b_6002aee8(c: &mut Machine) -> u32 {
    {
        // 0x6002aee8 ldr.w r1,[r11,#0x104]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(260u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x6002aeec
}
pub fn b_6002aeec(c: &mut Machine) -> u32 {
    {
        // 0x6002aeec lsls r0,r1,#0x2
        let mut t1afd00 = 0u64;
        let mut t1afe00 = 0u64;
        let mut t1b0000 = 0u64;
        let mut t1b0100 = 0u64;
        let mut t1b0200 = 0u64;
        let mut t1b0300 = 0u64;
        let mut t1b0400 = 0u64;
        let mut t1b0500 = 0u64;
        t1afd00 = ((2u64).wrapping_sub(1u64)) & 0xffffffffu64;
        t1afe00 = (if ((t1afd00 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(36, 4)) << (((t1afd00 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t1b0000 = (((t1afe00 >> 0) & 0xffffffffu64) & 2147483648u64) & 0xffffffffu64;
        t1b0100 = ((2u64 == 0u64) as u64) & 0xffu64;
        t1b0200 = (((((t1b0100 >> 0) & 0xffu64) != 0) & (c.r(98, 1) != 0)) as u64) & 0xffu64;
        t1b0300 = ((2u64 != 0u64) as u64) & 0xffu64;
        t1b0400 = ((((t1b0000 >> 0) & 0xffffffffu64) != 0u64) as u64) & 0xffu64;
        t1b0500 = (((((t1b0300 >> 0) & 0xffu64) != 0) & (((t1b0400 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        let value = ((((t1b0200 >> 0) & 0xffu64) != 0) | (((t1b0500 >> 0) & 0xffu64) != 0)) as u64;
        c.w(102, 1, value);
        let value = if 2u64 >= 32 {
            0
        } else {
            (c.r(36, 4)) << (2u64 as u32)
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
        // 0x6002aeee movs r4,#0x0
        let value = 0u64;
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
    }
    {
        // 0x6002aef0 bl 0x600414c0
        let value = (1610788596u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414c0u32;
    }
    0x6002aef4
}
pub fn b_6002aef4(c: &mut Machine) -> u32 {
    {
        // 0x6002aef4 mvn r1,#0x2
        let mut t187d00 = 0u64;
        t187d00 = (2u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = !((t187d00 >> 0) & 0xffffffffu64);
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002aef8 str.w r0,[r11,#0x130]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(304u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002aefc movs r0,#0x0
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
        // 0x6002aefe str.w r1,[r11,#0x124]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(292u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002af02 mov.w r5,#0x3f800000
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
        let value = ((t188100 >> 0) & 0xffffffffu64);
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002af06 str.w r1,[r11,#0xec]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(236u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002af0a movs r1,#0x0
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
        // 0x6002af0c vldr.32 s0,[pc,#0x28]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610788620u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(40u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002af10 str.w r4,[r11,#0x128]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(296u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002af14 str.w r4,[r11,#0x134]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(308u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002af18 str.w r4,[r11,#0xfc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(252u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002af1c str.w r4,[r11,#0x154]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(340u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002af20 str.w r4,[r11,#0x158]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(344u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002af24 str.w r5,[r11,#0xb4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(180u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002af28 b 0x6002af50
        return 0x6002af50u32;
    }
    0x6002af2a
}
pub fn b_6002af50(c: &mut Machine) -> u32 {
    {
        // 0x6002af50 strd r0,r1,[r11,#0xc0]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(76, 4)).wrapping_add(192u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002af54 mov.w r1,#0x3f000000
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
        // 0x6002af58 str.w r1,[r11,#0xb8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(184u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002af5c str.w r1,[r11,#0xbc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(188u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002af60 str.w r1,[r11,#0x150]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(336u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002af64 str.w r1,[r11,#0xf0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(240u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002af68 adr r1,[0x6002b2f8]
        let value = 1610789624u64;
        c.w(36, 4, value);
    }
    {
        // 0x6002af6a ldrd r0,r1,[r1,#0x0]
        let mut t19dc00 = 0u64;
        let mut t1ab800 = 0u64;
        t19dc00 = ((c.r(36, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19dc00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
        t1ab800 = (((t19dc00 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t1ab800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002af6e strd r0,r1,[r11,#0x138]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(76, 4)).wrapping_add(312u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002af72 ldr r0,[0x6002b300]
        let value = c.load(0x6002b300, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002af74 ldr r1,[0x6002b304]
        let value = c.load(0x6002b304, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002af76 str.w r0,[r11,#0x148]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(328u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002af7a ldr r0,[0x6002b308]
        let value = c.load(0x6002b308, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002af7c str.w r1,[r11,#0xb0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(176u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002af80 str.w r1,[r11,#0x178]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(376u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002af84 str.w r0,[r11,#0x14c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(332u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002af88 movs r0,#0x0
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
        // 0x6002af8a ldr r1,[0x6002b30c]
        let value = c.load(0x6002b30c, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002af8c strd r0,r1,[r11,#0x140]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(76, 4)).wrapping_add(320u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002af90 bl 0x60040e08
        let value = (1610788756u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040e08u32;
    }
    0x6002af94
}
pub fn b_6002af94(c: &mut Machine) -> u32 {
    {
        // 0x6002af94 bl 0x60041408
        let value = (1610788760u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041408u32;
    }
    0x6002af98
}
pub fn b_6002af98(c: &mut Machine) -> u32 {
    {
        // 0x6002af98 bl 0x60040c70
        let value = (1610788764u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002af9c
}
pub fn b_6002af9c(c: &mut Machine) -> u32 {
    {
        // 0x6002af9c vcvt.u32.f32 s15,s0
        let mut tade00 = 0u64;
        let mut tadf00 = 0u64;
        let mut tae000 = 0u64;
        tade00 =
            ((f32::from_bits(0u64 as u32) < f32::from_bits(c.r(768, 4) as u32)) as u64) & 0xffu64;
        tadf00 = ((tade00 >> 0) & 0xffu64) & 0xffffffffu64;
        tae000 = ((f32::from_bits(c.r(768, 4) as u32) as i32) as u64) & 0xffffffffu64;
        let value = ((tadf00 >> 0) & 0xffffffffu64).wrapping_mul(((tae000 >> 0) & 0xffffffffu64));
        c.w(828, 4, value);
    }
    {
        // 0x6002afa0 str.w r4,[r11,#0x188]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(392u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002afa4 str.w r4,[r11,#0x190]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(400u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002afa8 vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002afac str.w r4,[r11,#0x194]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(404u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002afb0 ldr.w r0,[r11,#0x1a8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(424u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002afb4 subs r4,r1,#0x1
        let value = (1u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 1u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(1u64);
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
        // 0x6002afb6 strd r1,r4,[r11,#0x17c]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(76, 4)).wrapping_add(380u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002afba subs r4,r1,#0x2
        let value = (2u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 2u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(2u64);
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
        // 0x6002afbc str.w r4,[r11,#0x184]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(388u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002afc0 movs r4,#0x0
        let value = 0u64;
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
    }
    {
        // 0x6002afc2 str.w r4,[r11,#0x18c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(396u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002afc6 str.w r4,[r11,#0x198]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(408u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002afca str.w r4,[r11,#0x1a4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(420u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002afce cbz r0,0x6002afd8
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x6002afd8u32;
        }
    }
    0x6002afd0
}
pub fn b_6002afd0(c: &mut Machine) -> u32 {
    {
        // 0x6002afd0 bl 0x600414d0
        let value = (1610788820u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414d0u32;
    }
    0x6002afd4
}
pub fn b_6002afd4(c: &mut Machine) -> u32 {
    {
        // 0x6002afd4 ldr.w r1,[r11,#0x17c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(380u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x6002afd8
}
pub fn b_6002afd8(c: &mut Machine) -> u32 {
    {
        // 0x6002afd8 lsls r0,r1,#0x2
        let mut t1afd00 = 0u64;
        let mut t1afe00 = 0u64;
        let mut t1b0000 = 0u64;
        let mut t1b0100 = 0u64;
        let mut t1b0200 = 0u64;
        let mut t1b0300 = 0u64;
        let mut t1b0400 = 0u64;
        let mut t1b0500 = 0u64;
        t1afd00 = ((2u64).wrapping_sub(1u64)) & 0xffffffffu64;
        t1afe00 = (if ((t1afd00 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(36, 4)) << (((t1afd00 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t1b0000 = (((t1afe00 >> 0) & 0xffffffffu64) & 2147483648u64) & 0xffffffffu64;
        t1b0100 = ((2u64 == 0u64) as u64) & 0xffu64;
        t1b0200 = (((((t1b0100 >> 0) & 0xffu64) != 0) & (c.r(98, 1) != 0)) as u64) & 0xffu64;
        t1b0300 = ((2u64 != 0u64) as u64) & 0xffu64;
        t1b0400 = ((((t1b0000 >> 0) & 0xffffffffu64) != 0u64) as u64) & 0xffu64;
        t1b0500 = (((((t1b0300 >> 0) & 0xffu64) != 0) & (((t1b0400 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        let value = ((((t1b0200 >> 0) & 0xffu64) != 0) | (((t1b0500 >> 0) & 0xffu64) != 0)) as u64;
        c.w(102, 1, value);
        let value = if 2u64 >= 32 {
            0
        } else {
            (c.r(36, 4)) << (2u64 as u32)
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
        // 0x6002afda movs r4,#0x0
        let value = 0u64;
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
    }
    {
        // 0x6002afdc bl 0x600414c0
        let value = (1610788832u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414c0u32;
    }
    0x6002afe0
}
pub fn b_6002afe0(c: &mut Machine) -> u32 {
    {
        // 0x6002afe0 mvn r5,#0x2
        let mut t187d00 = 0u64;
        t187d00 = (2u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = !((t187d00 >> 0) & 0xffffffffu64);
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002afe4 ldr r1,[0x6002b304]
        let value = c.load(0x6002b304, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002afe6 vldr.32 s0,[pc,#0x328]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610788838u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(808u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002afea str.w r0,[r11,#0x1a8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(424u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002afee str.w r1,[r11,#0x1b0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(432u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002aff2 str.w r4,[r11,#0x1a0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(416u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002aff6 str.w r4,[r11,#0x1ac]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(428u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002affa str.w r5,[r11,#0x19c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(412u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002affe bl 0x60040e08
        let value = (1610788866u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040e08u32;
    }
    0x6002b002
}
pub fn b_6002b002(c: &mut Machine) -> u32 {
    {
        // 0x6002b002 bl 0x60041408
        let value = (1610788870u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041408u32;
    }
    0x6002b006
}
pub fn b_6002b006(c: &mut Machine) -> u32 {
    {
        // 0x6002b006 bl 0x60040c70
        let value = (1610788874u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002b00a
}
pub fn b_6002b00a(c: &mut Machine) -> u32 {
    {
        // 0x6002b00a vcvt.u32.f32 s15,s0
        let mut tade00 = 0u64;
        let mut tadf00 = 0u64;
        let mut tae000 = 0u64;
        tade00 =
            ((f32::from_bits(0u64 as u32) < f32::from_bits(c.r(768, 4) as u32)) as u64) & 0xffu64;
        tadf00 = ((tade00 >> 0) & 0xffu64) & 0xffffffffu64;
        tae000 = ((f32::from_bits(c.r(768, 4) as u32) as i32) as u64) & 0xffffffffu64;
        let value = ((tadf00 >> 0) & 0xffffffffu64).wrapping_mul(((tae000 >> 0) & 0xffffffffu64));
        c.w(828, 4, value);
    }
    {
        // 0x6002b00e str.w r4,[r11,#0x1c0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(448u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b012 str.w r4,[r11,#0x1c8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(456u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b016 vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b01a str.w r4,[r11,#0x1cc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(460u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b01e ldr.w r0,[r11,#0x1e0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(480u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b022 subs r4,r1,#0x1
        let value = (1u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 1u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(1u64);
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
        // 0x6002b024 strd r1,r4,[r11,#0x1b4]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(76, 4)).wrapping_add(436u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b028 subs r4,r1,#0x2
        let value = (2u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 2u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(2u64);
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
        // 0x6002b02a str.w r4,[r11,#0x1bc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(444u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b02e movs r4,#0x0
        let value = 0u64;
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
    }
    {
        // 0x6002b030 str.w r4,[r11,#0x1c4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(452u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b034 str.w r4,[r11,#0x1d0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(464u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b038 str.w r4,[r11,#0x1dc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(476u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b03c cbz r0,0x6002b046
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x6002b046u32;
        }
    }
    0x6002b03e
}
pub fn b_6002b03e(c: &mut Machine) -> u32 {
    {
        // 0x6002b03e bl 0x600414d0
        let value = (1610788930u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414d0u32;
    }
    0x6002b042
}
pub fn b_6002b042(c: &mut Machine) -> u32 {
    {
        // 0x6002b042 ldr.w r1,[r11,#0x1b4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(436u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x6002b046
}
pub fn b_6002b046(c: &mut Machine) -> u32 {
    {
        // 0x6002b046 lsls r0,r1,#0x2
        let mut t1afd00 = 0u64;
        let mut t1afe00 = 0u64;
        let mut t1b0000 = 0u64;
        let mut t1b0100 = 0u64;
        let mut t1b0200 = 0u64;
        let mut t1b0300 = 0u64;
        let mut t1b0400 = 0u64;
        let mut t1b0500 = 0u64;
        t1afd00 = ((2u64).wrapping_sub(1u64)) & 0xffffffffu64;
        t1afe00 = (if ((t1afd00 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(36, 4)) << (((t1afd00 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t1b0000 = (((t1afe00 >> 0) & 0xffffffffu64) & 2147483648u64) & 0xffffffffu64;
        t1b0100 = ((2u64 == 0u64) as u64) & 0xffu64;
        t1b0200 = (((((t1b0100 >> 0) & 0xffu64) != 0) & (c.r(98, 1) != 0)) as u64) & 0xffu64;
        t1b0300 = ((2u64 != 0u64) as u64) & 0xffu64;
        t1b0400 = ((((t1b0000 >> 0) & 0xffffffffu64) != 0u64) as u64) & 0xffu64;
        t1b0500 = (((((t1b0300 >> 0) & 0xffu64) != 0) & (((t1b0400 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        let value = ((((t1b0200 >> 0) & 0xffu64) != 0) | (((t1b0500 >> 0) & 0xffu64) != 0)) as u64;
        c.w(102, 1, value);
        let value = if 2u64 >= 32 {
            0
        } else {
            (c.r(36, 4)) << (2u64 as u32)
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
        // 0x6002b048 movs r4,#0x0
        let value = 0u64;
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
    }
    {
        // 0x6002b04a bl 0x600414c0
        let value = (1610788942u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414c0u32;
    }
    0x6002b04e
}
pub fn b_6002b04e(c: &mut Machine) -> u32 {
    {
        // 0x6002b04e mvn r1,#0x2
        let mut t187d00 = 0u64;
        t187d00 = (2u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = !((t187d00 >> 0) & 0xffffffffu64);
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002b052 str.w r0,[r11,#0x1e0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(480u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b056 movs r0,#0x0
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
        // 0x6002b058 str.w r1,[r11,#0x1d4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(468u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b05c mov.w r5,#0x3f800000
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
        let value = ((t188100 >> 0) & 0xffffffffu64);
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002b060 str.w r1,[r11,#0x19c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(412u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b064 movs r1,#0x0
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
        // 0x6002b066 vldr.32 s0,[pc,#0x2a8]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610788966u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(680u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002b06a str.w r4,[r11,#0x1d8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(472u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b06e str.w r4,[r11,#0x1e4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(484u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b072 str.w r4,[r11,#0x1ac]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(428u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b076 str.w r4,[r11,#0x204]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(516u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b07a str.w r4,[r11,#0x208]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(520u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b07e str.w r5,[r11,#0x164]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(356u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b082 strd r0,r1,[r11,#0x170]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(76, 4)).wrapping_add(368u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b086 mov.w r1,#0x3f000000
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
        // 0x6002b08a str.w r1,[r11,#0x168]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(360u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b08e str.w r1,[r11,#0x16c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(364u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b092 str.w r1,[r11,#0x200]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(512u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b096 str.w r1,[r11,#0x1a0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(416u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b09a adr r1,[0x6002b2f8]
        let value = 1610789624u64;
        c.w(36, 4, value);
    }
    {
        // 0x6002b09c ldrd r0,r1,[r1,#0x0]
        let mut t19dc00 = 0u64;
        let mut t1ab800 = 0u64;
        t19dc00 = ((c.r(36, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19dc00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
        t1ab800 = (((t19dc00 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t1ab800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b0a0 strd r0,r1,[r11,#0x1e8]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(76, 4)).wrapping_add(488u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b0a4 ldr r0,[0x6002b300]
        let value = c.load(0x6002b300, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b0a6 ldr r1,[0x6002b304]
        let value = c.load(0x6002b304, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b0a8 str.w r0,[r11,#0x1f8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(504u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b0ac ldr r0,[0x6002b308]
        let value = c.load(0x6002b308, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b0ae str.w r1,[r11,#0x160]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(352u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b0b2 str.w r1,[r11,#0x228]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(552u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b0b6 str.w r0,[r11,#0x1fc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(508u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b0ba movs r0,#0x0
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
        // 0x6002b0bc ldr r1,[0x6002b30c]
        let value = c.load(0x6002b30c, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b0be strd r0,r1,[r11,#0x1f0]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(76, 4)).wrapping_add(496u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b0c2 bl 0x60040e08
        let value = (1610789062u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040e08u32;
    }
    0x6002b0c6
}
pub fn b_6002b0c6(c: &mut Machine) -> u32 {
    {
        // 0x6002b0c6 bl 0x60041408
        let value = (1610789066u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041408u32;
    }
    0x6002b0ca
}
pub fn b_6002b0ca(c: &mut Machine) -> u32 {
    {
        // 0x6002b0ca bl 0x60040c70
        let value = (1610789070u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002b0ce
}
pub fn b_6002b0ce(c: &mut Machine) -> u32 {
    {
        // 0x6002b0ce vcvt.u32.f32 s15,s0
        let mut tade00 = 0u64;
        let mut tadf00 = 0u64;
        let mut tae000 = 0u64;
        tade00 =
            ((f32::from_bits(0u64 as u32) < f32::from_bits(c.r(768, 4) as u32)) as u64) & 0xffu64;
        tadf00 = ((tade00 >> 0) & 0xffu64) & 0xffffffffu64;
        tae000 = ((f32::from_bits(c.r(768, 4) as u32) as i32) as u64) & 0xffffffffu64;
        let value = ((tadf00 >> 0) & 0xffffffffu64).wrapping_mul(((tae000 >> 0) & 0xffffffffu64));
        c.w(828, 4, value);
    }
    {
        // 0x6002b0d2 str.w r4,[r11,#0x238]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(568u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b0d6 str.w r4,[r11,#0x240]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(576u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b0da vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b0de str.w r4,[r11,#0x244]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(580u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b0e2 ldr.w r0,[r11,#0x258]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(600u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b0e6 subs r4,r1,#0x1
        let value = (1u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 1u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(1u64);
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
        // 0x6002b0e8 strd r1,r4,[r11,#0x22c]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(76, 4)).wrapping_add(556u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b0ec subs r4,r1,#0x2
        let value = (2u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 2u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(2u64);
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
        // 0x6002b0ee str.w r4,[r11,#0x234]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(564u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b0f2 movs r4,#0x0
        let value = 0u64;
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
    }
    {
        // 0x6002b0f4 str.w r4,[r11,#0x23c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(572u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b0f8 str.w r4,[r11,#0x248]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(584u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b0fc str.w r4,[r11,#0x254]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(596u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b100 cbz r0,0x6002b10a
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x6002b10au32;
        }
    }
    0x6002b102
}
pub fn b_6002b102(c: &mut Machine) -> u32 {
    {
        // 0x6002b102 bl 0x600414d0
        let value = (1610789126u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414d0u32;
    }
    0x6002b106
}
pub fn b_6002b106(c: &mut Machine) -> u32 {
    {
        // 0x6002b106 ldr.w r1,[r11,#0x22c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(556u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x6002b10a
}
pub fn b_6002b10a(c: &mut Machine) -> u32 {
    {
        // 0x6002b10a lsls r0,r1,#0x2
        let mut t1afd00 = 0u64;
        let mut t1afe00 = 0u64;
        let mut t1b0000 = 0u64;
        let mut t1b0100 = 0u64;
        let mut t1b0200 = 0u64;
        let mut t1b0300 = 0u64;
        let mut t1b0400 = 0u64;
        let mut t1b0500 = 0u64;
        t1afd00 = ((2u64).wrapping_sub(1u64)) & 0xffffffffu64;
        t1afe00 = (if ((t1afd00 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(36, 4)) << (((t1afd00 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t1b0000 = (((t1afe00 >> 0) & 0xffffffffu64) & 2147483648u64) & 0xffffffffu64;
        t1b0100 = ((2u64 == 0u64) as u64) & 0xffu64;
        t1b0200 = (((((t1b0100 >> 0) & 0xffu64) != 0) & (c.r(98, 1) != 0)) as u64) & 0xffu64;
        t1b0300 = ((2u64 != 0u64) as u64) & 0xffu64;
        t1b0400 = ((((t1b0000 >> 0) & 0xffffffffu64) != 0u64) as u64) & 0xffu64;
        t1b0500 = (((((t1b0300 >> 0) & 0xffu64) != 0) & (((t1b0400 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        let value = ((((t1b0200 >> 0) & 0xffu64) != 0) | (((t1b0500 >> 0) & 0xffu64) != 0)) as u64;
        c.w(102, 1, value);
        let value = if 2u64 >= 32 {
            0
        } else {
            (c.r(36, 4)) << (2u64 as u32)
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
        // 0x6002b10c movs r4,#0x0
        let value = 0u64;
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
    }
    {
        // 0x6002b10e bl 0x600414c0
        let value = (1610789138u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414c0u32;
    }
    0x6002b112
}
pub fn b_6002b112(c: &mut Machine) -> u32 {
    {
        // 0x6002b112 mvn r5,#0x2
        let mut t187d00 = 0u64;
        t187d00 = (2u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = !((t187d00 >> 0) & 0xffffffffu64);
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002b116 ldr r1,[0x6002b304]
        let value = c.load(0x6002b304, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b118 vldr.32 s0,[pc,#0x1f4]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610789144u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(500u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002b11c str.w r0,[r11,#0x258]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(600u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b120 str.w r1,[r11,#0x260]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(608u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b124 str.w r4,[r11,#0x250]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(592u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b128 str.w r4,[r11,#0x25c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(604u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b12c str.w r5,[r11,#0x24c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(588u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b130 bl 0x60040e08
        let value = (1610789172u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040e08u32;
    }
    0x6002b134
}
pub fn b_6002b134(c: &mut Machine) -> u32 {
    {
        // 0x6002b134 bl 0x60041408
        let value = (1610789176u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041408u32;
    }
    0x6002b138
}
pub fn b_6002b138(c: &mut Machine) -> u32 {
    {
        // 0x6002b138 bl 0x60040c70
        let value = (1610789180u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002b13c
}
pub fn b_6002b13c(c: &mut Machine) -> u32 {
    {
        // 0x6002b13c vcvt.u32.f32 s15,s0
        let mut tade00 = 0u64;
        let mut tadf00 = 0u64;
        let mut tae000 = 0u64;
        tade00 =
            ((f32::from_bits(0u64 as u32) < f32::from_bits(c.r(768, 4) as u32)) as u64) & 0xffu64;
        tadf00 = ((tade00 >> 0) & 0xffu64) & 0xffffffffu64;
        tae000 = ((f32::from_bits(c.r(768, 4) as u32) as i32) as u64) & 0xffffffffu64;
        let value = ((tadf00 >> 0) & 0xffffffffu64).wrapping_mul(((tae000 >> 0) & 0xffffffffu64));
        c.w(828, 4, value);
    }
    {
        // 0x6002b140 str.w r4,[r11,#0x270]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(624u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b144 str.w r4,[r11,#0x278]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(632u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b148 vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b14c str.w r4,[r11,#0x27c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(636u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b150 ldr.w r0,[r11,#0x290]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(656u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b154 subs r4,r1,#0x1
        let value = (1u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 1u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(1u64);
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
        // 0x6002b156 strd r1,r4,[r11,#0x264]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(76, 4)).wrapping_add(612u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b15a subs r4,r1,#0x2
        let value = (2u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 2u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(2u64);
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
        // 0x6002b15c str.w r4,[r11,#0x26c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(620u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b160 movs r4,#0x0
        let value = 0u64;
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
    }
    {
        // 0x6002b162 str.w r4,[r11,#0x274]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(628u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b166 str.w r4,[r11,#0x280]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(640u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b16a str.w r4,[r11,#0x28c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(652u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b16e cbz r0,0x6002b178
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x6002b178u32;
        }
    }
    0x6002b170
}
pub fn b_6002b170(c: &mut Machine) -> u32 {
    {
        // 0x6002b170 bl 0x600414d0
        let value = (1610789236u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414d0u32;
    }
    0x6002b174
}
pub fn b_6002b174(c: &mut Machine) -> u32 {
    {
        // 0x6002b174 ldr.w r1,[r11,#0x264]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(612u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x6002b178
}
pub fn b_6002b178(c: &mut Machine) -> u32 {
    {
        // 0x6002b178 lsls r0,r1,#0x2
        let mut t1afd00 = 0u64;
        let mut t1afe00 = 0u64;
        let mut t1b0000 = 0u64;
        let mut t1b0100 = 0u64;
        let mut t1b0200 = 0u64;
        let mut t1b0300 = 0u64;
        let mut t1b0400 = 0u64;
        let mut t1b0500 = 0u64;
        t1afd00 = ((2u64).wrapping_sub(1u64)) & 0xffffffffu64;
        t1afe00 = (if ((t1afd00 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(36, 4)) << (((t1afd00 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t1b0000 = (((t1afe00 >> 0) & 0xffffffffu64) & 2147483648u64) & 0xffffffffu64;
        t1b0100 = ((2u64 == 0u64) as u64) & 0xffu64;
        t1b0200 = (((((t1b0100 >> 0) & 0xffu64) != 0) & (c.r(98, 1) != 0)) as u64) & 0xffu64;
        t1b0300 = ((2u64 != 0u64) as u64) & 0xffu64;
        t1b0400 = ((((t1b0000 >> 0) & 0xffffffffu64) != 0u64) as u64) & 0xffu64;
        t1b0500 = (((((t1b0300 >> 0) & 0xffu64) != 0) & (((t1b0400 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        let value = ((((t1b0200 >> 0) & 0xffu64) != 0) | (((t1b0500 >> 0) & 0xffu64) != 0)) as u64;
        c.w(102, 1, value);
        let value = if 2u64 >= 32 {
            0
        } else {
            (c.r(36, 4)) << (2u64 as u32)
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
        // 0x6002b17a movs r4,#0x0
        let value = 0u64;
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
    }
    {
        // 0x6002b17c bl 0x600414c0
        let value = (1610789248u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414c0u32;
    }
    0x6002b180
}
pub fn b_6002b180(c: &mut Machine) -> u32 {
    {
        // 0x6002b180 mvn r1,#0x2
        let mut t187d00 = 0u64;
        t187d00 = (2u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = !((t187d00 >> 0) & 0xffffffffu64);
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002b184 str.w r0,[r11,#0x290]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(656u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b188 movs r0,#0x0
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
        // 0x6002b18a str.w r1,[r11,#0x284]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(644u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b18e mov.w r5,#0x3f800000
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
        let value = ((t188100 >> 0) & 0xffffffffu64);
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002b192 str.w r1,[r11,#0x24c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(588u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b196 movs r1,#0x0
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
        // 0x6002b198 vldr.32 s0,[pc,#0x174]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610789272u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(372u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002b19c str.w r4,[r11,#0x288]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(648u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b1a0 str.w r4,[r11,#0x294]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(660u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b1a4 str.w r4,[r11,#0x25c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(604u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b1a8 str.w r4,[r11,#0x2b4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(692u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b1ac str.w r4,[r11,#0x2b8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(696u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b1b0 str.w r5,[r11,#0x214]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(532u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b1b4 strd r0,r1,[r11,#0x220]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(76, 4)).wrapping_add(544u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b1b8 mov.w r1,#0x3f000000
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
        // 0x6002b1bc str.w r1,[r11,#0x218]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(536u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b1c0 str.w r1,[r11,#0x21c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(540u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b1c4 str.w r1,[r11,#0x2b0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(688u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b1c8 str.w r1,[r11,#0x250]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(592u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b1cc adr r1,[0x6002b2f8]
        let value = 1610789624u64;
        c.w(36, 4, value);
    }
    {
        // 0x6002b1ce ldrd r0,r1,[r1,#0x0]
        let mut t19dc00 = 0u64;
        let mut t1ab800 = 0u64;
        t19dc00 = ((c.r(36, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19dc00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
        t1ab800 = (((t19dc00 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t1ab800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b1d2 strd r0,r1,[r11,#0x298]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(76, 4)).wrapping_add(664u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b1d6 ldr r0,[0x6002b300]
        let value = c.load(0x6002b300, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b1d8 ldr r1,[0x6002b304]
        let value = c.load(0x6002b304, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b1da str.w r0,[r11,#0x2a8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(680u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b1de ldr r0,[0x6002b308]
        let value = c.load(0x6002b308, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b1e0 str.w r1,[r11,#0x210]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(528u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b1e4 str.w r1,[r11,#0x2d8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(728u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b1e8 str.w r0,[r11,#0x2ac]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(684u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b1ec movs r0,#0x0
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
        // 0x6002b1ee ldr r1,[0x6002b30c]
        let value = c.load(0x6002b30c, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b1f0 strd r0,r1,[r11,#0x2a0]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(76, 4)).wrapping_add(672u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b1f4 bl 0x60040e08
        let value = (1610789368u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040e08u32;
    }
    0x6002b1f8
}
pub fn b_6002b1f8(c: &mut Machine) -> u32 {
    {
        // 0x6002b1f8 bl 0x60041408
        let value = (1610789372u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041408u32;
    }
    0x6002b1fc
}
pub fn b_6002b1fc(c: &mut Machine) -> u32 {
    {
        // 0x6002b1fc bl 0x60040c70
        let value = (1610789376u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002b200
}
pub fn b_6002b200(c: &mut Machine) -> u32 {
    {
        // 0x6002b200 vcvt.u32.f32 s15,s0
        let mut tade00 = 0u64;
        let mut tadf00 = 0u64;
        let mut tae000 = 0u64;
        tade00 =
            ((f32::from_bits(0u64 as u32) < f32::from_bits(c.r(768, 4) as u32)) as u64) & 0xffu64;
        tadf00 = ((tade00 >> 0) & 0xffu64) & 0xffffffffu64;
        tae000 = ((f32::from_bits(c.r(768, 4) as u32) as i32) as u64) & 0xffffffffu64;
        let value = ((tadf00 >> 0) & 0xffffffffu64).wrapping_mul(((tae000 >> 0) & 0xffffffffu64));
        c.w(828, 4, value);
    }
    {
        // 0x6002b204 str.w r4,[r11,#0x2e8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(744u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b208 str.w r4,[r11,#0x2f0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(752u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b20c vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b210 str.w r4,[r11,#0x2f4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(756u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b214 ldr.w r0,[r11,#0x308]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(776u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b218 subs r4,r1,#0x1
        let value = (1u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 1u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(1u64);
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
        // 0x6002b21a strd r1,r4,[r11,#0x2dc]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(76, 4)).wrapping_add(732u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b21e subs r4,r1,#0x2
        let value = (2u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 2u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(2u64);
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
        // 0x6002b220 str.w r4,[r11,#0x2e4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(740u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b224 movs r4,#0x0
        let value = 0u64;
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
    }
    {
        // 0x6002b226 str.w r4,[r11,#0x2ec]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(748u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b22a str.w r4,[r11,#0x2f8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(760u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b22e str.w r4,[r11,#0x304]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(772u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b232 cbz r0,0x6002b23c
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x6002b23cu32;
        }
    }
    0x6002b234
}
pub fn b_6002b234(c: &mut Machine) -> u32 {
    {
        // 0x6002b234 bl 0x600414d0
        let value = (1610789432u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414d0u32;
    }
    0x6002b238
}
pub fn b_6002b238(c: &mut Machine) -> u32 {
    {
        // 0x6002b238 ldr.w r1,[r11,#0x2dc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(732u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x6002b23c
}
pub fn b_6002b23c(c: &mut Machine) -> u32 {
    {
        // 0x6002b23c lsls r0,r1,#0x2
        let mut t1afd00 = 0u64;
        let mut t1afe00 = 0u64;
        let mut t1b0000 = 0u64;
        let mut t1b0100 = 0u64;
        let mut t1b0200 = 0u64;
        let mut t1b0300 = 0u64;
        let mut t1b0400 = 0u64;
        let mut t1b0500 = 0u64;
        t1afd00 = ((2u64).wrapping_sub(1u64)) & 0xffffffffu64;
        t1afe00 = (if ((t1afd00 >> 0) & 0xffffffffu64) >= 32 {
            0
        } else {
            (c.r(36, 4)) << (((t1afd00 >> 0) & 0xffffffffu64) as u32)
        }) & 0xffffffffu64;
        t1b0000 = (((t1afe00 >> 0) & 0xffffffffu64) & 2147483648u64) & 0xffffffffu64;
        t1b0100 = ((2u64 == 0u64) as u64) & 0xffu64;
        t1b0200 = (((((t1b0100 >> 0) & 0xffu64) != 0) & (c.r(98, 1) != 0)) as u64) & 0xffu64;
        t1b0300 = ((2u64 != 0u64) as u64) & 0xffu64;
        t1b0400 = ((((t1b0000 >> 0) & 0xffffffffu64) != 0u64) as u64) & 0xffu64;
        t1b0500 = (((((t1b0300 >> 0) & 0xffu64) != 0) & (((t1b0400 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        let value = ((((t1b0200 >> 0) & 0xffu64) != 0) | (((t1b0500 >> 0) & 0xffu64) != 0)) as u64;
        c.w(102, 1, value);
        let value = if 2u64 >= 32 {
            0
        } else {
            (c.r(36, 4)) << (2u64 as u32)
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
        // 0x6002b23e movs r4,#0x0
        let value = 0u64;
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
        let value = c.r(101, 1);
        c.w(97, 1, value);
        let value = c.r(100, 1);
        c.w(96, 1, value);
    }
    {
        // 0x6002b240 bl 0x600414c0
        let value = (1610789444u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414c0u32;
    }
    0x6002b244
}
