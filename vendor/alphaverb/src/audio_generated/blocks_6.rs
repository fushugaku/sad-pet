#![allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut,
    unused_comparisons
)]
use crate::machine::{signed, signed_overflow, Machine};
pub fn b_6002b244(c: &mut Machine) -> u32 {
    {
        // 0x6002b244 mvn r5,#0x2
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
        // 0x6002b248 ldr r1,[0x6002b304]
        let value = c.load(0x6002b304, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b24a vldr.32 s0,[pc,#0xc4]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610789450u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(196u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002b24e str.w r0,[r11,#0x308]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(776u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b252 str.w r1,[r11,#0x310]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(784u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b256 str.w r4,[r11,#0x300]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(768u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b25a str.w r4,[r11,#0x30c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(780u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b25e str.w r5,[r11,#0x2fc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(764u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b262 bl 0x60040e08
        let value = (1610789478u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040e08u32;
    }
    0x6002b266
}
pub fn b_6002b266(c: &mut Machine) -> u32 {
    {
        // 0x6002b266 bl 0x60041408
        let value = (1610789482u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041408u32;
    }
    0x6002b26a
}
pub fn b_6002b26a(c: &mut Machine) -> u32 {
    {
        // 0x6002b26a bl 0x60040c70
        let value = (1610789486u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002b26e
}
pub fn b_6002b26e(c: &mut Machine) -> u32 {
    {
        // 0x6002b26e vcvt.u32.f32 s15,s0
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
        // 0x6002b272 str.w r4,[r11,#0x320]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(800u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b276 str.w r4,[r11,#0x328]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(808u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b27a vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b27e str.w r4,[r11,#0x32c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(812u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b282 ldr.w r0,[r11,#0x340]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(832u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b286 subs r4,r1,#0x1
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
        // 0x6002b288 strd r1,r4,[r11,#0x314]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(76, 4)).wrapping_add(788u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b28c subs r4,r1,#0x2
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
        // 0x6002b28e str.w r4,[r11,#0x31c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(796u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b292 movs r4,#0x0
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
        // 0x6002b294 str.w r4,[r11,#0x324]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(804u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b298 str.w r4,[r11,#0x330]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(816u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b29c str.w r4,[r11,#0x33c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(828u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b2a0 cbz r0,0x6002b2aa
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x6002b2aau32;
        }
    }
    0x6002b2a2
}
pub fn b_6002b2a2(c: &mut Machine) -> u32 {
    {
        // 0x6002b2a2 bl 0x600414d0
        let value = (1610789542u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414d0u32;
    }
    0x6002b2a6
}
pub fn b_6002b2a6(c: &mut Machine) -> u32 {
    {
        // 0x6002b2a6 ldr.w r1,[r11,#0x314]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(788u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x6002b2aa
}
pub fn b_6002b2aa(c: &mut Machine) -> u32 {
    {
        // 0x6002b2aa lsls r0,r1,#0x2
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
        // 0x6002b2ac movs r4,#0x0
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
        // 0x6002b2ae bl 0x600414c0
        let value = (1610789554u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414c0u32;
    }
    0x6002b2b2
}
pub fn b_6002b2b2(c: &mut Machine) -> u32 {
    {
        // 0x6002b2b2 mvn r1,#0x2
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
        // 0x6002b2b6 str.w r0,[r11,#0x340]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(832u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b2ba movs r0,#0x0
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
        // 0x6002b2bc str.w r1,[r11,#0x334]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(820u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b2c0 mov.w r5,#0x3f800000
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
        // 0x6002b2c4 str.w r1,[r11,#0x2fc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(764u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b2c8 movs r1,#0x0
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
        // 0x6002b2ca vldr.32 s0,[pc,#0x44]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610789578u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(68u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002b2ce str.w r4,[r11,#0x338]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(824u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b2d2 str.w r4,[r11,#0x344]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(836u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b2d6 str.w r4,[r11,#0x30c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(780u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b2da str.w r4,[r11,#0x364]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(868u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b2de str.w r4,[r11,#0x368]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(872u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b2e2 str.w r5,[r11,#0x2c4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(708u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b2e6 strd r0,r1,[r11,#0x2d0]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(76, 4)).wrapping_add(720u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b2ea mov.w r1,#0x3f000000
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
        // 0x6002b2ee str.w r1,[r11,#0x2c8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(712u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b2f2 str.w r1,[r11,#0x2cc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(716u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b2f6 b 0x6002b314
        return 0x6002b314u32;
    }
    0x6002b2f8
}
pub fn b_6002b314(c: &mut Machine) -> u32 {
    {
        // 0x6002b314 str.w r1,[r11,#0x360]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(864u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b318 str.w r1,[r11,#0x300]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(768u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b31c adr r1,[0x6002b668]
        let value = 1610790504u64;
        c.w(36, 4, value);
    }
    {
        // 0x6002b31e ldrd r0,r1,[r1,#0x0]
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
        // 0x6002b322 strd r0,r1,[r11,#0x348]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(76, 4)).wrapping_add(840u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b326 ldr r0,[0x6002b670]
        let value = c.load(0x6002b670, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b328 ldr r1,[0x6002b674]
        let value = c.load(0x6002b674, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b32a str.w r0,[r11,#0x358]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(856u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b32e ldr r0,[0x6002b678]
        let value = c.load(0x6002b678, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b330 str.w r1,[r11,#0x2c0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(704u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b334 str.w r1,[r11,#0x388]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(904u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b338 str.w r0,[r11,#0x35c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(860u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b33c movs r0,#0x0
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
        // 0x6002b33e ldr r1,[0x6002b67c]
        let value = c.load(0x6002b67c, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b340 strd r0,r1,[r11,#0x350]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(76, 4)).wrapping_add(848u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b344 bl 0x60040e08
        let value = (1610789704u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040e08u32;
    }
    0x6002b348
}
pub fn b_6002b348(c: &mut Machine) -> u32 {
    {
        // 0x6002b348 bl 0x60041408
        let value = (1610789708u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041408u32;
    }
    0x6002b34c
}
pub fn b_6002b34c(c: &mut Machine) -> u32 {
    {
        // 0x6002b34c bl 0x60040c70
        let value = (1610789712u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002b350
}
pub fn b_6002b350(c: &mut Machine) -> u32 {
    {
        // 0x6002b350 vcvt.u32.f32 s15,s0
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
        // 0x6002b354 str.w r4,[r11,#0x398]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(920u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b358 str.w r4,[r11,#0x3a0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(928u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b35c vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b360 str.w r4,[r11,#0x3a4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(932u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b364 ldr.w r0,[r11,#0x3b8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(952u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b368 subs r4,r1,#0x1
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
        // 0x6002b36a strd r1,r4,[r11,#0x38c]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(76, 4)).wrapping_add(908u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b36e subs r4,r1,#0x2
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
        // 0x6002b370 str.w r4,[r11,#0x394]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(916u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b374 movs r4,#0x0
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
        // 0x6002b376 str.w r4,[r11,#0x39c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(924u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b37a str.w r4,[r11,#0x3a8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(936u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b37e str.w r4,[r11,#0x3b4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(948u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b382 cbz r0,0x6002b38c
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x6002b38cu32;
        }
    }
    0x6002b384
}
pub fn b_6002b384(c: &mut Machine) -> u32 {
    {
        // 0x6002b384 bl 0x600414d0
        let value = (1610789768u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414d0u32;
    }
    0x6002b388
}
pub fn b_6002b388(c: &mut Machine) -> u32 {
    {
        // 0x6002b388 ldr.w r1,[r11,#0x38c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(908u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x6002b38c
}
pub fn b_6002b38c(c: &mut Machine) -> u32 {
    {
        // 0x6002b38c lsls r0,r1,#0x2
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
        // 0x6002b38e movs r4,#0x0
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
        // 0x6002b390 bl 0x600414c0
        let value = (1610789780u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414c0u32;
    }
    0x6002b394
}
pub fn b_6002b394(c: &mut Machine) -> u32 {
    {
        // 0x6002b394 mvn r5,#0x2
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
        // 0x6002b398 ldr r1,[0x6002b674]
        let value = c.load(0x6002b674, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b39a vldr.32 s0,[pc,#0x2e4]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610789786u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(740u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002b39e str.w r0,[r11,#0x3b8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(952u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b3a2 str.w r1,[r11,#0x3c0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(960u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b3a6 str.w r4,[r11,#0x3b0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(944u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b3aa str.w r4,[r11,#0x3bc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(956u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b3ae str.w r5,[r11,#0x3ac]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(940u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b3b2 bl 0x60040e08
        let value = (1610789814u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040e08u32;
    }
    0x6002b3b6
}
pub fn b_6002b3b6(c: &mut Machine) -> u32 {
    {
        // 0x6002b3b6 bl 0x60041408
        let value = (1610789818u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041408u32;
    }
    0x6002b3ba
}
pub fn b_6002b3ba(c: &mut Machine) -> u32 {
    {
        // 0x6002b3ba bl 0x60040c70
        let value = (1610789822u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002b3be
}
pub fn b_6002b3be(c: &mut Machine) -> u32 {
    {
        // 0x6002b3be vcvt.u32.f32 s15,s0
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
        // 0x6002b3c2 str.w r4,[r11,#0x3d0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(976u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b3c6 str.w r4,[r11,#0x3d8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(984u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b3ca vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b3ce str.w r4,[r11,#0x3dc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(988u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b3d2 ldr.w r0,[r11,#0x3f0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1008u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b3d6 subs r4,r1,#0x1
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
        // 0x6002b3d8 strd r1,r4,[r11,#0x3c4]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(76, 4)).wrapping_add(964u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b3dc subs r4,r1,#0x2
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
        // 0x6002b3de str.w r4,[r11,#0x3cc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(972u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b3e2 movs r4,#0x0
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
        // 0x6002b3e4 str.w r4,[r11,#0x3d4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(980u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b3e8 str.w r4,[r11,#0x3e0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(992u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b3ec str.w r4,[r11,#0x3ec]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1004u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b3f0 cbz r0,0x6002b3fa
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x6002b3fau32;
        }
    }
    0x6002b3f2
}
pub fn b_6002b3f2(c: &mut Machine) -> u32 {
    {
        // 0x6002b3f2 bl 0x600414d0
        let value = (1610789878u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414d0u32;
    }
    0x6002b3f6
}
pub fn b_6002b3f6(c: &mut Machine) -> u32 {
    {
        // 0x6002b3f6 ldr.w r1,[r11,#0x3c4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(964u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x6002b3fa
}
pub fn b_6002b3fa(c: &mut Machine) -> u32 {
    {
        // 0x6002b3fa lsls r0,r1,#0x2
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
        // 0x6002b3fc ldr r4,[0x6002b684]
        let value = c.load(0x6002b684, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6002b3fe bl 0x600414c0
        let value = (1610789890u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414c0u32;
    }
    0x6002b402
}
pub fn b_6002b402(c: &mut Machine) -> u32 {
    {
        // 0x6002b402 mvn r1,#0x2
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
        // 0x6002b406 str.w r0,[r11,#0x3f0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1008u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b40a movs r0,#0x0
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
        // 0x6002b40c str.w r1,[r11,#0x3e4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(996u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b410 movs r5,#0x0
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
        // 0x6002b412 str.w r1,[r11,#0x3ac]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(940u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b416 movs r1,#0x0
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
        // 0x6002b418 vldr.32 s0,[pc,#0x264]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610789912u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(612u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002b41c str r5,[r4,#0xc]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(12u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b41e str r5,[r4,#0x10]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b420 str.w r5,[r11,#0x3e8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1000u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b424 str.w r5,[r11,#0x3f4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1012u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b428 str.w r5,[r11,#0x3bc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(956u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b42c strd r0,r1,[r11,#0x380]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(76, 4)).wrapping_add(896u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b430 mov.w r1,#0x3f000000
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
        // 0x6002b434 ldr r0,[0x6002b670]
        let value = c.load(0x6002b670, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b436 str r1,[r4,#0x8]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b438 str.w r1,[r11,#0x378]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(888u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b43c str.w r1,[r11,#0x37c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(892u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b440 str.w r1,[r11,#0x3b0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(944u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b444 mov.w r1,#0x3f800000
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
        c.w(36, 4, value);
        let value = (signed(c.r(36, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(36, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002b448 str.w r1,[r11,#0x374]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(884u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b44c mov r1,r4
        let value = c.r(48, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b44e str.w r0,[r1],#0x4
        let mut t19cb00 = 0u64;
        t19cb00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = (c.r(36, 4)).wrapping_add(4u64);
        c.w(36, 4, value);
        c.store(((t19cb00 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b452 ldr r0,[0x6002b674]
        let value = c.load(0x6002b674, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b454 str r0,[r4,#0x30]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b456 str.w r0,[r11,#0x370]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(880u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b45a ldr r0,[0x6002b678]
        let value = c.load(0x6002b678, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b45c adr r7,[0x6002b668]
        let value = 1610790504u64;
        c.w(60, 4, value);
    }
    {
        // 0x6002b45e ldrd r6,r7,[r7,#0x0]
        let mut t19dc00 = 0u64;
        let mut t1ab800 = 0u64;
        t19dc00 = ((c.r(60, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19dc00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(56, 4, value);
        t1ab800 = (((t19dc00 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t1ab800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(60, 4, value);
    }
    {
        // 0x6002b462 str r0,[r1,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(36, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b464 movs r0,#0x0
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
        // 0x6002b466 ldr r1,[0x6002b67c]
        let value = c.load(0x6002b67c, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b468 strd r6,r7,[r11,#0x3f8]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(76, 4)).wrapping_add(1016u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(60, 4));
    }
    {
        // 0x6002b46c strd r0,r1,[r4,#-0x8]
        let mut t19d800 = 0u64;
        let mut t1fb400 = 0u64;
        t19d800 = ((c.r(48, 4)).wrapping_sub(8u64)) & 0xffffffffu64;
        t1fb400 = ((t19d800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b470 bl 0x60040e08
        let value = (1610790004u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040e08u32;
    }
    0x6002b474
}
pub fn b_6002b474(c: &mut Machine) -> u32 {
    {
        // 0x6002b474 bl 0x60041408
        let value = (1610790008u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041408u32;
    }
    0x6002b478
}
pub fn b_6002b478(c: &mut Machine) -> u32 {
    {
        // 0x6002b478 bl 0x60040c70
        let value = (1610790012u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002b47c
}
pub fn b_6002b47c(c: &mut Machine) -> u32 {
    {
        // 0x6002b47c vcvt.u32.f32 s15,s0
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
        // 0x6002b480 str r5,[r4,#0x40]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b482 str r5,[r4,#0x48]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(72u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b484 vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b488 str r5,[r4,#0x4c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(76u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b48a ldr.w r0,[r11,#0x468]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1128u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b48e subs r4,r1,#0x1
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
        // 0x6002b490 str.w r1,[r11,#0x43c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1084u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b494 str.w r4,[r11,#0x440]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1088u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b498 subs r4,r1,#0x2
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
        // 0x6002b49a str.w r4,[r11,#0x444]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1092u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b49e movs r4,#0x0
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
        // 0x6002b4a0 str.w r4,[r11,#0x44c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1100u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b4a4 str.w r4,[r11,#0x458]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1112u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b4a8 str.w r4,[r11,#0x464]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1124u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b4ac cbz r0,0x6002b4b6
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x6002b4b6u32;
        }
    }
    0x6002b4ae
}
pub fn b_6002b4ae(c: &mut Machine) -> u32 {
    {
        // 0x6002b4ae bl 0x600414d0
        let value = (1610790066u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414d0u32;
    }
    0x6002b4b2
}
pub fn b_6002b4b2(c: &mut Machine) -> u32 {
    {
        // 0x6002b4b2 ldr.w r1,[r11,#0x43c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1084u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x6002b4b6
}
pub fn b_6002b4b6(c: &mut Machine) -> u32 {
    {
        // 0x6002b4b6 lsls r0,r1,#0x2
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
        // 0x6002b4b8 ldr r5,[0x6002b688]
        let value = c.load(0x6002b688, 4);
        c.w(52, 4, value);
    }
    {
        // 0x6002b4ba bl 0x600414c0
        let value = (1610790078u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414c0u32;
    }
    0x6002b4be
}
pub fn b_6002b4be(c: &mut Machine) -> u32 {
    {
        // 0x6002b4be mvn r1,#0x2
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
        // 0x6002b4c2 movs r4,#0x0
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
        // 0x6002b4c4 str.w r0,[r11,#0x468]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1128u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b4c8 str.w r1,[r11,#0x45c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1116u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b4cc mov r6,r5
        let value = c.r(52, 4);
        c.w(56, 4, value);
    }
    {
        // 0x6002b4ce mov r1,r5
        let value = c.r(52, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b4d0 ldr r0,[0x6002b674]
        let value = c.load(0x6002b674, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b4d2 vldr.32 s0,[pc,#0x1ac]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610790098u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(428u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002b4d6 str.w r4,[r1],#0x10
        let mut t19cb00 = 0u64;
        t19cb00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = (c.r(36, 4)).wrapping_add(16u64);
        c.w(36, 4, value);
        c.store(((t19cb00 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b4da str.w r4,[r6,#0xc]!
        let mut t19d200 = 0u64;
        t19d200 = ((c.r(56, 4)).wrapping_add(12u64)) & 0xffffffffu64;
        let value = ((t19d200 >> 0) & 0xffffffffu64);
        c.w(56, 4, value);
        c.store(((t19d200 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b4de str r0,[r1,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(36, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b4e0 bl 0x60040e08
        let value = (1610790116u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040e08u32;
    }
    0x6002b4e4
}
pub fn b_6002b4e4(c: &mut Machine) -> u32 {
    {
        // 0x6002b4e4 bl 0x60041408
        let value = (1610790120u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041408u32;
    }
    0x6002b4e8
}
pub fn b_6002b4e8(c: &mut Machine) -> u32 {
    {
        // 0x6002b4e8 bl 0x60040c70
        let value = (1610790124u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002b4ec
}
pub fn b_6002b4ec(c: &mut Machine) -> u32 {
    {
        // 0x6002b4ec vcvt.u32.f32 s15,s0
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
        // 0x6002b4f0 str r4,[r5,#0x20]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b4f2 str r4,[r5,#0x28]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b4f4 vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b4f8 str r4,[r5,#0x2c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b4fa movs r4,#0x0
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
        // 0x6002b4fc ldr.w r0,[r11,#0x4a0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1184u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b500 str.w r1,[r11,#0x474]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1140u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b504 str.w r4,[r11,#0x484]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1156u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b508 str.w r4,[r11,#0x490]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1168u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b50c str.w r4,[r11,#0x49c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1180u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b510 subs r4,r1,#0x1
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
        // 0x6002b512 str.w r4,[r11,#0x478]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1144u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b516 subs r4,r1,#0x2
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
        // 0x6002b518 str.w r4,[r11,#0x47c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1148u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b51c cbz r0,0x6002b526
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x6002b526u32;
        }
    }
    0x6002b51e
}
pub fn b_6002b51e(c: &mut Machine) -> u32 {
    {
        // 0x6002b51e bl 0x600414d0
        let value = (1610790178u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414d0u32;
    }
    0x6002b522
}
pub fn b_6002b522(c: &mut Machine) -> u32 {
    {
        // 0x6002b522 ldr.w r1,[r11,#0x474]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1140u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x6002b526
}
pub fn b_6002b526(c: &mut Machine) -> u32 {
    {
        // 0x6002b526 lsls r0,r1,#0x2
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
        // 0x6002b528 ldr r4,[0x6002b68c]
        let value = c.load(0x6002b68c, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6002b52a bl 0x600414c0
        let value = (1610790190u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414c0u32;
    }
    0x6002b52e
}
pub fn b_6002b52e(c: &mut Machine) -> u32 {
    {
        // 0x6002b52e mvn r1,#0x2
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
        // 0x6002b532 str.w r0,[r11,#0x4a0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1184u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b536 mov r0,r4
        let value = c.r(48, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b538 str.w r1,[r11,#0x494]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1172u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b53c mov r3,r4
        let value = c.r(48, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002b53e str.w r1,[r11,#0x45c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1116u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b542 mov.w r1,#0x3f000000
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
        // 0x6002b546 mov.w r7,#0x3f800000
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
        c.w(60, 4, value);
        let value = (signed(c.r(60, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(60, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002b54a vldr.32 s0,[pc,#0x134]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610790218u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(308u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002b54e str r1,[r5,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b550 movs r5,#0x0
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
        // 0x6002b552 str.w r7,[r3,#-0x74]!
        let mut t19d000 = 0u64;
        t19d000 = ((c.r(44, 4)).wrapping_sub(116u64)) & 0xffffffffu64;
        let value = ((t19d000 >> 0) & 0xffffffffu64);
        c.w(44, 4, value);
        c.store(((t19d000 >> 0) & 0xffffffffu64) as u32, 4, c.r(60, 4));
    }
    {
        // 0x6002b556 str.w r5,[r0],#0xc
        let mut t19cb00 = 0u64;
        t19cb00 = (c.r(32, 4)) & 0xffffffffu64;
        let value = (c.r(32, 4)).wrapping_add(12u64);
        c.w(32, 4, value);
        c.store(((t19cb00 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b55a str r5,[r0,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(32, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b55c ldr r0,[0x6002b674]
        let value = c.load(0x6002b674, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b55e str r3,[sp,#0x10]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002b560 mov r3,r4
        let value = c.r(48, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002b562 str.w r0,[r4,#-0x78]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(48, 4)).wrapping_sub(120u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b566 str r0,[r4,#0x50]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(80u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b568 ldr r0,[0x6002b670]
        let value = c.load(0x6002b670, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b56a str.w r1,[r3,#-0x70]!
        let mut t19d000 = 0u64;
        t19d000 = ((c.r(44, 4)).wrapping_sub(112u64)) & 0xffffffffu64;
        let value = ((t19d000 >> 0) & 0xffffffffu64);
        c.w(44, 4, value);
        c.store(((t19d000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b56e str r0,[r4,#0x20]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b570 str r3,[sp,#0x14]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(20u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002b572 mov r3,r4
        let value = c.r(48, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002b574 ldr r0,[0x6002b678]
        let value = c.load(0x6002b678, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b576 str.w r1,[r3,#-0x6c]!
        let mut t19d000 = 0u64;
        t19d000 = ((c.r(44, 4)).wrapping_sub(108u64)) & 0xffffffffu64;
        let value = ((t19d000 >> 0) & 0xffffffffu64);
        c.w(44, 4, value);
        c.store(((t19d000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b57a str r1,[r4,#0x28]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b57c str r0,[r4,#0x24]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b57e str r3,[sp,#0x3c]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(60u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002b580 str r5,[r6,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b582 str r5,[r4,#0x2c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b584 str r5,[r4,#0x30]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b586 adr r1,[0x6002b668]
        let value = 1610790504u64;
        c.w(36, 4, value);
    }
    {
        // 0x6002b588 ldrd r0,r1,[r1,#0x0]
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
        // 0x6002b58c strd r0,r1,[r4,#0x10]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(48, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b590 movs r0,#0x0
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
        // 0x6002b592 ldr r1,[0x6002b67c]
        let value = c.load(0x6002b67c, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b594 strd r0,r1,[r4,#0x18]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(48, 4)).wrapping_add(24u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b598 movs r0,#0x0
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
        // 0x6002b59a movs r1,#0x0
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
        // 0x6002b59c strd r0,r1,[r4,#-0x68]
        let mut t19d800 = 0u64;
        let mut t1fb400 = 0u64;
        t19d800 = ((c.r(48, 4)).wrapping_sub(104u64)) & 0xffffffffu64;
        t1fb400 = ((t19d800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b5a0 bl 0x60040e08
        let value = (1610790308u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040e08u32;
    }
    0x6002b5a4
}
pub fn b_6002b5a4(c: &mut Machine) -> u32 {
    {
        // 0x6002b5a4 bl 0x60041408
        let value = (1610790312u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041408u32;
    }
    0x6002b5a8
}
pub fn b_6002b5a8(c: &mut Machine) -> u32 {
    {
        // 0x6002b5a8 bl 0x60040c70
        let value = (1610790316u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002b5ac
}
pub fn b_6002b5ac(c: &mut Machine) -> u32 {
    {
        // 0x6002b5ac vcvt.u32.f32 s15,s0
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
        // 0x6002b5b0 movs r0,#0x0
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
        // 0x6002b5b2 str r5,[r4,#0x60]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(96u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b5b4 str r5,[r4,#0x68]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(104u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b5b6 vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b5ba str.w r0,[r11,#0x4fc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1276u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b5be str.w r0,[r11,#0x508]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1288u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b5c2 str.w r0,[r11,#0x514]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1300u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b5c6 subs r0,r1,#0x1
        let value = (1u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 1u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(1u64);
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
        // 0x6002b5c8 str r5,[r4,#0x6c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(108u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b5ca str.w r0,[r11,#0x4f0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1264u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b5ce subs r0,r1,#0x2
        let value = (2u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 2u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(2u64);
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
        // 0x6002b5d0 str.w r1,[r11,#0x4ec]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1260u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b5d4 str.w r0,[r11,#0x4f4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1268u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b5d8 ldr.w r0,[r11,#0x518]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1304u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b5dc cbz r0,0x6002b5e6
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x6002b5e6u32;
        }
    }
    0x6002b5de
}
pub fn b_6002b5de(c: &mut Machine) -> u32 {
    {
        // 0x6002b5de bl 0x600414d0
        let value = (1610790370u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414d0u32;
    }
    0x6002b5e2
}
pub fn b_6002b5e2(c: &mut Machine) -> u32 {
    {
        // 0x6002b5e2 ldr.w r1,[r11,#0x4ec]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1260u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x6002b5e6
}
pub fn b_6002b5e6(c: &mut Machine) -> u32 {
    {
        // 0x6002b5e6 lsls r0,r1,#0x2
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
        // 0x6002b5e8 ldr r5,[0x6002b690]
        let value = c.load(0x6002b690, 4);
        c.w(52, 4, value);
    }
    {
        // 0x6002b5ea bl 0x600414c0
        let value = (1610790382u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414c0u32;
    }
    0x6002b5ee
}
pub fn b_6002b5ee(c: &mut Machine) -> u32 {
    {
        // 0x6002b5ee mvn r1,#0x2
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
        // 0x6002b5f2 movs r4,#0x0
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
        // 0x6002b5f4 str.w r0,[r11,#0x518]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1304u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b5f8 str.w r1,[r11,#0x50c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1292u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b5fc mov r6,r5
        let value = c.r(52, 4);
        c.w(56, 4, value);
    }
    {
        // 0x6002b5fe mov r1,r5
        let value = c.r(52, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b600 ldr r0,[0x6002b674]
        let value = c.load(0x6002b674, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b602 vldr.32 s0,[pc,#0x7c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610790402u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(124u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002b606 str.w r4,[r1],#0x10
        let mut t19cb00 = 0u64;
        t19cb00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = (c.r(36, 4)).wrapping_add(16u64);
        c.w(36, 4, value);
        c.store(((t19cb00 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b60a str.w r4,[r6,#0xc]!
        let mut t19d200 = 0u64;
        t19d200 = ((c.r(56, 4)).wrapping_add(12u64)) & 0xffffffffu64;
        let value = ((t19d200 >> 0) & 0xffffffffu64);
        c.w(56, 4, value);
        c.store(((t19d200 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b60e str r0,[r1,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(36, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b610 bl 0x60040e08
        let value = (1610790420u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040e08u32;
    }
    0x6002b614
}
pub fn b_6002b614(c: &mut Machine) -> u32 {
    {
        // 0x6002b614 bl 0x60041408
        let value = (1610790424u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041408u32;
    }
    0x6002b618
}
pub fn b_6002b618(c: &mut Machine) -> u32 {
    {
        // 0x6002b618 bl 0x60040c70
        let value = (1610790428u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002b61c
}
pub fn b_6002b61c(c: &mut Machine) -> u32 {
    {
        // 0x6002b61c vcvt.u32.f32 s15,s0
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
        // 0x6002b620 movs r0,#0x0
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
        // 0x6002b622 str r4,[r5,#0x20]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b624 str r4,[r5,#0x28]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b626 vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b62a str.w r0,[r11,#0x534]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1332u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b62e str.w r0,[r11,#0x540]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1344u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b632 str.w r0,[r11,#0x54c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1356u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b636 subs r0,r1,#0x1
        let value = (1u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 1u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(1u64);
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
        // 0x6002b638 str r4,[r5,#0x2c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b63a str.w r0,[r11,#0x528]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1320u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b63e subs r0,r1,#0x2
        let value = (2u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 2u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(2u64);
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
        // 0x6002b640 str.w r1,[r11,#0x524]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1316u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b644 str.w r0,[r11,#0x52c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1324u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b648 ldr.w r0,[r11,#0x550]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1360u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b64c cbz r0,0x6002b656
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x6002b656u32;
        }
    }
    0x6002b64e
}
pub fn b_6002b64e(c: &mut Machine) -> u32 {
    {
        // 0x6002b64e bl 0x600414d0
        let value = (1610790482u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414d0u32;
    }
    0x6002b652
}
pub fn b_6002b652(c: &mut Machine) -> u32 {
    {
        // 0x6002b652 ldr.w r1,[r11,#0x524]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1316u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x6002b656
}
pub fn b_6002b656(c: &mut Machine) -> u32 {
    {
        // 0x6002b656 lsls r0,r1,#0x2
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
        // 0x6002b658 ldr r4,[0x6002b694]
        let value = c.load(0x6002b694, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6002b65a bl 0x600414c0
        let value = (1610790494u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414c0u32;
    }
    0x6002b65e
}
pub fn b_6002b65e(c: &mut Machine) -> u32 {
    {
        // 0x6002b65e mov.w r1,#0x3f000000
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
        // 0x6002b662 str.w r0,[r11,#0x550]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1360u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b666 b 0x6002b698
        return 0x6002b698u32;
    }
    0x6002b668
}
pub fn b_6002b698(c: &mut Machine) -> u32 {
    {
        // 0x6002b698 mvn r0,#0x2
        let mut t187d00 = 0u64;
        t187d00 = (2u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = !((t187d00 >> 0) & 0xffffffffu64);
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002b69c str r1,[r5,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b69e movs r5,#0x0
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
        // 0x6002b6a0 str.w r0,[r11,#0x544]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1348u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b6a4 mov r3,r4
        let value = c.r(48, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002b6a6 str.w r0,[r11,#0x50c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1292u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b6aa mov r0,r4
        let value = c.r(48, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b6ac mov.w r7,#0x3f800000
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
        c.w(60, 4, value);
        let value = (signed(c.r(60, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(60, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002b6b0 str r1,[r4,#0x28]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b6b2 str.w r5,[r0],#0xc
        let mut t19cb00 = 0u64;
        t19cb00 = (c.r(32, 4)) & 0xffffffffu64;
        let value = (c.r(32, 4)).wrapping_add(12u64);
        c.w(32, 4, value);
        c.store(((t19cb00 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b6b6 str r5,[r0,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(32, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b6b8 ldr r0,[0x6002b9e8]
        let value = c.load(0x6002b9e8, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b6ba str.w r7,[r3,#-0x74]!
        let mut t19d000 = 0u64;
        t19d000 = ((c.r(44, 4)).wrapping_sub(116u64)) & 0xffffffffu64;
        let value = ((t19d000 >> 0) & 0xffffffffu64);
        c.w(44, 4, value);
        c.store(((t19d000 >> 0) & 0xffffffffu64) as u32, 4, c.r(60, 4));
    }
    {
        // 0x6002b6be str.w r0,[r4,#-0x78]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(48, 4)).wrapping_sub(120u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b6c2 str r0,[r4,#0x50]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(80u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b6c4 str r3,[sp,#0x18]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(24u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002b6c6 mov r3,r4
        let value = c.r(48, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002b6c8 ldr r0,[0x6002b9ec]
        let value = c.load(0x6002b9ec, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b6ca str.w r1,[r3,#-0x70]!
        let mut t19d000 = 0u64;
        t19d000 = ((c.r(44, 4)).wrapping_sub(112u64)) & 0xffffffffu64;
        let value = ((t19d000 >> 0) & 0xffffffffu64);
        c.w(44, 4, value);
        c.store(((t19d000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b6ce str r0,[r4,#0x20]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b6d0 str r3,[sp,#0x1c]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002b6d2 mov r3,r4
        let value = c.r(48, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002b6d4 ldr r0,[0x6002b9f0]
        let value = c.load(0x6002b9f0, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b6d6 str.w r1,[r3,#-0x6c]!
        let mut t19d000 = 0u64;
        t19d000 = ((c.r(44, 4)).wrapping_sub(108u64)) & 0xffffffffu64;
        let value = ((t19d000 >> 0) & 0xffffffffu64);
        c.w(44, 4, value);
        c.store(((t19d000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b6da str r0,[r4,#0x24]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b6dc vldr.32 s0,[pc,#0x314]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610790620u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(788u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002b6e0 str r3,[sp,#0x40]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002b6e2 str r5,[r6,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b6e4 str r5,[r4,#0x2c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b6e6 str r5,[r4,#0x30]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b6e8 adr r1,[0x6002b9e0]
        let value = 1610791392u64;
        c.w(36, 4, value);
    }
    {
        // 0x6002b6ea ldrd r0,r1,[r1,#0x0]
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
        // 0x6002b6ee strd r0,r1,[r4,#0x10]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(48, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b6f2 movs r0,#0x0
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
        // 0x6002b6f4 ldr r1,[0x6002b9f8]
        let value = c.load(0x6002b9f8, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b6f6 strd r0,r1,[r4,#0x18]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(48, 4)).wrapping_add(24u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b6fa movs r0,#0x0
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
        // 0x6002b6fc movs r1,#0x0
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
        // 0x6002b6fe strd r0,r1,[r4,#-0x68]
        let mut t19d800 = 0u64;
        let mut t1fb400 = 0u64;
        t19d800 = ((c.r(48, 4)).wrapping_sub(104u64)) & 0xffffffffu64;
        t1fb400 = ((t19d800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b702 bl 0x60040e08
        let value = (1610790662u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040e08u32;
    }
    0x6002b706
}
pub fn b_6002b706(c: &mut Machine) -> u32 {
    {
        // 0x6002b706 bl 0x60041408
        let value = (1610790666u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041408u32;
    }
    0x6002b70a
}
pub fn b_6002b70a(c: &mut Machine) -> u32 {
    {
        // 0x6002b70a bl 0x60040c70
        let value = (1610790670u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002b70e
}
pub fn b_6002b70e(c: &mut Machine) -> u32 {
    {
        // 0x6002b70e vcvt.u32.f32 s15,s0
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
        // 0x6002b712 movs r0,#0x0
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
        // 0x6002b714 str r5,[r4,#0x60]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(96u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b716 str r5,[r4,#0x68]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(104u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b718 vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b71c str.w r0,[r11,#0x5ac]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1452u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b720 str.w r0,[r11,#0x5b8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1464u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b724 str.w r0,[r11,#0x5c4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1476u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b728 subs r0,r1,#0x1
        let value = (1u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 1u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(1u64);
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
        // 0x6002b72a str r5,[r4,#0x6c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(108u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b72c str.w r0,[r11,#0x5a0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1440u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b730 subs r0,r1,#0x2
        let value = (2u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 2u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(2u64);
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
        // 0x6002b732 str.w r1,[r11,#0x59c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1436u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b736 str.w r0,[r11,#0x5a4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1444u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b73a ldr.w r0,[r11,#0x5c8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1480u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b73e cbz r0,0x6002b748
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x6002b748u32;
        }
    }
    0x6002b740
}
pub fn b_6002b740(c: &mut Machine) -> u32 {
    {
        // 0x6002b740 bl 0x600414d0
        let value = (1610790724u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414d0u32;
    }
    0x6002b744
}
pub fn b_6002b744(c: &mut Machine) -> u32 {
    {
        // 0x6002b744 ldr.w r1,[r11,#0x59c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1436u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x6002b748
}
pub fn b_6002b748(c: &mut Machine) -> u32 {
    {
        // 0x6002b748 lsls r0,r1,#0x2
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
        // 0x6002b74a ldr r6,[0x6002b9fc]
        let value = c.load(0x6002b9fc, 4);
        c.w(56, 4, value);
    }
    {
        // 0x6002b74c bl 0x600414c0
        let value = (1610790736u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414c0u32;
    }
    0x6002b750
}
pub fn b_6002b750(c: &mut Machine) -> u32 {
    {
        // 0x6002b750 mvn r1,#0x2
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
        // 0x6002b754 movs r5,#0x0
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
        // 0x6002b756 str.w r0,[r11,#0x5c8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1480u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b75a str.w r1,[r11,#0x5bc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1468u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b75e mov r4,r6
        let value = c.r(56, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6002b760 mov r1,r6
        let value = c.r(56, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b762 ldr r0,[0x6002b9e8]
        let value = c.load(0x6002b9e8, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b764 vldr.32 s0,[pc,#0x28c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610790756u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(652u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002b768 str.w r5,[r1],#0x10
        let mut t19cb00 = 0u64;
        t19cb00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = (c.r(36, 4)).wrapping_add(16u64);
        c.w(36, 4, value);
        c.store(((t19cb00 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b76c str.w r5,[r4,#0xc]!
        let mut t19d200 = 0u64;
        t19d200 = ((c.r(48, 4)).wrapping_add(12u64)) & 0xffffffffu64;
        let value = ((t19d200 >> 0) & 0xffffffffu64);
        c.w(48, 4, value);
        c.store(((t19d200 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b770 str r0,[r1,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(36, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b772 bl 0x60040e08
        let value = (1610790774u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040e08u32;
    }
    0x6002b776
}
pub fn b_6002b776(c: &mut Machine) -> u32 {
    {
        // 0x6002b776 bl 0x60041408
        let value = (1610790778u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041408u32;
    }
    0x6002b77a
}
pub fn b_6002b77a(c: &mut Machine) -> u32 {
    {
        // 0x6002b77a bl 0x60040c70
        let value = (1610790782u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002b77e
}
pub fn b_6002b77e(c: &mut Machine) -> u32 {
    {
        // 0x6002b77e vcvt.u32.f32 s15,s0
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
        // 0x6002b782 movs r0,#0x0
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
        // 0x6002b784 str r5,[r6,#0x20]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b786 str r5,[r6,#0x28]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b788 vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b78c str.w r0,[r11,#0x5e4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1508u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b790 str.w r0,[r11,#0x5f0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1520u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b794 str.w r0,[r11,#0x5fc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1532u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b798 subs r0,r1,#0x1
        let value = (1u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 1u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(1u64);
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
        // 0x6002b79a str r5,[r6,#0x2c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b79c str.w r0,[r11,#0x5d8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1496u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b7a0 subs r0,r1,#0x2
        let value = (2u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 2u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(2u64);
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
        // 0x6002b7a2 str.w r1,[r11,#0x5d4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1492u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b7a6 str.w r0,[r11,#0x5dc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1500u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b7aa ldr.w r0,[r11,#0x600]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1536u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b7ae cbz r0,0x6002b7b8
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x6002b7b8u32;
        }
    }
    0x6002b7b0
}
pub fn b_6002b7b0(c: &mut Machine) -> u32 {
    {
        // 0x6002b7b0 bl 0x600414d0
        let value = (1610790836u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414d0u32;
    }
    0x6002b7b4
}
pub fn b_6002b7b4(c: &mut Machine) -> u32 {
    {
        // 0x6002b7b4 ldr.w r1,[r11,#0x5d4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1492u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x6002b7b8
}
pub fn b_6002b7b8(c: &mut Machine) -> u32 {
    {
        // 0x6002b7b8 lsls r0,r1,#0x2
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
        // 0x6002b7ba ldr r5,[0x6002ba00]
        let value = c.load(0x6002ba00, 4);
        c.w(52, 4, value);
    }
    {
        // 0x6002b7bc bl 0x600414c0
        let value = (1610790848u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414c0u32;
    }
    0x6002b7c0
}
pub fn b_6002b7c0(c: &mut Machine) -> u32 {
    {
        // 0x6002b7c0 str.w r0,[r11,#0x600]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1536u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b7c4 mov.w r0,#0x3f800000
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
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002b7c8 mov.w r1,#0x3f000000
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
        // 0x6002b7cc mov r3,r5
        let value = c.r(52, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002b7ce vldr.32 s0,[pc,#0x224]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610790862u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(548u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002b7d2 str.w r0,[r5,#-0x74]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(52, 4)).wrapping_sub(116u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b7d6 mvn r0,#0x2
        let mut t187d00 = 0u64;
        t187d00 = (2u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = !((t187d00 >> 0) & 0xffffffffu64);
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002b7da str r1,[r6,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b7dc movs r6,#0x0
        let value = 0u64;
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
        // 0x6002b7de str.w r0,[r11,#0x5f4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1524u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b7e2 str.w r0,[r11,#0x5bc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1468u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b7e6 mov r0,r5
        let value = c.r(52, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b7e8 str.w r1,[r3,#-0x6c]!
        let mut t19d000 = 0u64;
        t19d000 = ((c.r(44, 4)).wrapping_sub(108u64)) & 0xffffffffu64;
        let value = ((t19d000 >> 0) & 0xffffffffu64);
        c.w(44, 4, value);
        c.store(((t19d000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b7ec str.w r6,[r0],#0xc
        let mut t19cb00 = 0u64;
        t19cb00 = (c.r(32, 4)) & 0xffffffffu64;
        let value = (c.r(32, 4)).wrapping_add(12u64);
        c.w(32, 4, value);
        c.store(((t19cb00 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002b7f0 str r6,[r0,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(32, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002b7f2 ldr r0,[0x6002b9e8]
        let value = c.load(0x6002b9e8, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b7f4 str.w r1,[r5,#-0x70]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(52, 4)).wrapping_sub(112u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b7f8 str.w r0,[r5,#-0x78]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(52, 4)).wrapping_sub(120u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b7fc str r0,[r5,#0x50]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(80u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b7fe ldr r0,[0x6002b9ec]
        let value = c.load(0x6002b9ec, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b800 str r1,[r5,#0x28]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b802 str r0,[r5,#0x20]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b804 ldr r0,[0x6002b9f0]
        let value = c.load(0x6002b9f0, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b806 str r3,[sp,#0x20]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002b808 str r0,[r5,#0x24]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b80a str r6,[r4,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002b80c str r6,[r5,#0x2c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002b80e str r6,[r5,#0x30]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002b810 adr r1,[0x6002b9e0]
        let value = 1610791392u64;
        c.w(36, 4, value);
    }
    {
        // 0x6002b812 ldrd r0,r1,[r1,#0x0]
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
        // 0x6002b816 strd r0,r1,[r5,#0x10]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(52, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b81a movs r0,#0x0
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
        // 0x6002b81c ldr r1,[0x6002b9f8]
        let value = c.load(0x6002b9f8, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b81e strd r0,r1,[r5,#0x18]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(52, 4)).wrapping_add(24u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b822 movs r0,#0x0
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
        // 0x6002b824 movs r1,#0x0
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
        // 0x6002b826 strd r0,r1,[r5,#-0x68]
        let mut t19d800 = 0u64;
        let mut t1fb400 = 0u64;
        t19d800 = ((c.r(52, 4)).wrapping_sub(104u64)) & 0xffffffffu64;
        t1fb400 = ((t19d800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b82a bl 0x60040e08
        let value = (1610790958u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040e08u32;
    }
    0x6002b82e
}
pub fn b_6002b82e(c: &mut Machine) -> u32 {
    {
        // 0x6002b82e bl 0x60041408
        let value = (1610790962u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041408u32;
    }
    0x6002b832
}
pub fn b_6002b832(c: &mut Machine) -> u32 {
    {
        // 0x6002b832 bl 0x60040c70
        let value = (1610790966u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002b836
}
pub fn b_6002b836(c: &mut Machine) -> u32 {
    {
        // 0x6002b836 vcvt.u32.f32 s15,s0
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
        // 0x6002b83a movs r0,#0x0
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
        // 0x6002b83c str r6,[r5,#0x60]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(96u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002b83e str r6,[r5,#0x68]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(104u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002b840 vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b844 str.w r0,[r11,#0x65c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1628u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b848 str.w r0,[r11,#0x668]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1640u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b84c str.w r0,[r11,#0x674]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1652u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b850 subs r0,r1,#0x1
        let value = (1u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 1u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(1u64);
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
        // 0x6002b852 str r6,[r5,#0x6c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(108u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002b854 str.w r0,[r11,#0x650]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1616u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b858 subs r0,r1,#0x2
        let value = (2u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 2u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(2u64);
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
        // 0x6002b85a str.w r1,[r11,#0x64c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1612u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b85e str.w r0,[r11,#0x654]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1620u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b862 ldr.w r0,[r11,#0x678]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1656u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b866 cbz r0,0x6002b870
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x6002b870u32;
        }
    }
    0x6002b868
}
pub fn b_6002b868(c: &mut Machine) -> u32 {
    {
        // 0x6002b868 bl 0x600414d0
        let value = (1610791020u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414d0u32;
    }
    0x6002b86c
}
pub fn b_6002b86c(c: &mut Machine) -> u32 {
    {
        // 0x6002b86c ldr.w r1,[r11,#0x64c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1612u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x6002b870
}
pub fn b_6002b870(c: &mut Machine) -> u32 {
    {
        // 0x6002b870 lsls r0,r1,#0x2
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
        // 0x6002b872 ldr r5,[0x6002ba04]
        let value = c.load(0x6002ba04, 4);
        c.w(52, 4, value);
    }
    {
        // 0x6002b874 bl 0x600414c0
        let value = (1610791032u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414c0u32;
    }
    0x6002b878
}
pub fn b_6002b878(c: &mut Machine) -> u32 {
    {
        // 0x6002b878 mvn r1,#0x2
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
        // 0x6002b87c movs r6,#0x0
        let value = 0u64;
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
        // 0x6002b87e str.w r0,[r11,#0x678]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1656u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b882 str.w r1,[r11,#0x66c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1644u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b886 mov r4,r5
        let value = c.r(52, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6002b888 mov r1,r5
        let value = c.r(52, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b88a ldr r0,[0x6002b9e8]
        let value = c.load(0x6002b9e8, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b88c vldr.32 s0,[pc,#0x164]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610791052u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(356u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002b890 str.w r6,[r1],#0x10
        let mut t19cb00 = 0u64;
        t19cb00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = (c.r(36, 4)).wrapping_add(16u64);
        c.w(36, 4, value);
        c.store(((t19cb00 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002b894 str.w r6,[r4,#0xc]!
        let mut t19d200 = 0u64;
        t19d200 = ((c.r(48, 4)).wrapping_add(12u64)) & 0xffffffffu64;
        let value = ((t19d200 >> 0) & 0xffffffffu64);
        c.w(48, 4, value);
        c.store(((t19d200 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002b898 str r0,[r1,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(36, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b89a bl 0x60040e08
        let value = (1610791070u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040e08u32;
    }
    0x6002b89e
}
pub fn b_6002b89e(c: &mut Machine) -> u32 {
    {
        // 0x6002b89e bl 0x60041408
        let value = (1610791074u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041408u32;
    }
    0x6002b8a2
}
pub fn b_6002b8a2(c: &mut Machine) -> u32 {
    {
        // 0x6002b8a2 bl 0x60040c70
        let value = (1610791078u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002b8a6
}
pub fn b_6002b8a6(c: &mut Machine) -> u32 {
    {
        // 0x6002b8a6 vcvt.u32.f32 s15,s0
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
        // 0x6002b8aa movs r0,#0x0
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
        // 0x6002b8ac str r6,[r5,#0x20]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002b8ae str r6,[r5,#0x28]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002b8b0 vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b8b4 str.w r0,[r11,#0x694]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1684u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b8b8 str.w r0,[r11,#0x6a0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1696u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b8bc str.w r0,[r11,#0x6ac]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1708u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b8c0 subs r0,r1,#0x1
        let value = (1u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 1u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(1u64);
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
        // 0x6002b8c2 str r6,[r5,#0x2c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002b8c4 str.w r0,[r11,#0x688]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1672u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b8c8 subs r0,r1,#0x2
        let value = (2u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 2u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(2u64);
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
        // 0x6002b8ca str.w r1,[r11,#0x684]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1668u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b8ce str.w r0,[r11,#0x68c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1676u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b8d2 ldr.w r0,[r11,#0x6b0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1712u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b8d6 cbz r0,0x6002b8e0
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x6002b8e0u32;
        }
    }
    0x6002b8d8
}
pub fn b_6002b8d8(c: &mut Machine) -> u32 {
    {
        // 0x6002b8d8 bl 0x600414d0
        let value = (1610791132u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414d0u32;
    }
    0x6002b8dc
}
pub fn b_6002b8dc(c: &mut Machine) -> u32 {
    {
        // 0x6002b8dc ldr.w r1,[r11,#0x684]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1668u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x6002b8e0
}
pub fn b_6002b8e0(c: &mut Machine) -> u32 {
    {
        // 0x6002b8e0 lsls r0,r1,#0x2
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
        // 0x6002b8e2 ldr r6,[0x6002ba08]
        let value = c.load(0x6002ba08, 4);
        c.w(56, 4, value);
    }
    {
        // 0x6002b8e4 bl 0x600414c0
        let value = (1610791144u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414c0u32;
    }
    0x6002b8e8
}
pub fn b_6002b8e8(c: &mut Machine) -> u32 {
    {
        // 0x6002b8e8 mov.w r1,#0x3f000000
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
        // 0x6002b8ec str.w r0,[r11,#0x6b0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1712u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b8f0 mov.w r0,#0x3f800000
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
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002b8f4 str r1,[r5,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b8f6 movs r5,#0x0
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
        // 0x6002b8f8 str.w r0,[r6,#-0x74]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(56, 4)).wrapping_sub(116u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b8fc mvn r0,#0x2
        let mut t187d00 = 0u64;
        t187d00 = (2u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = !((t187d00 >> 0) & 0xffffffffu64);
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002b900 str r5,[r4,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b902 mov r3,r6
        let value = c.r(56, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002b904 ldr r4,[0x6002b9e8]
        let value = c.load(0x6002b9e8, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6002b906 str.w r0,[r11,#0x6a4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1700u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b90a str.w r0,[r11,#0x66c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1644u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b90e mov r0,r6
        let value = c.r(56, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b910 str.w r4,[r6,#-0x78]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(56, 4)).wrapping_sub(120u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b914 str r4,[r6,#0x50]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(80u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b916 ldr r4,[0x6002b9ec]
        let value = c.load(0x6002b9ec, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6002b918 str.w r1,[r3,#-0x6c]!
        let mut t19d000 = 0u64;
        t19d000 = ((c.r(44, 4)).wrapping_sub(108u64)) & 0xffffffffu64;
        let value = ((t19d000 >> 0) & 0xffffffffu64);
        c.w(44, 4, value);
        c.store(((t19d000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b91c str.w r5,[r0],#0xc
        let mut t19cb00 = 0u64;
        t19cb00 = (c.r(32, 4)) & 0xffffffffu64;
        let value = (c.r(32, 4)).wrapping_add(12u64);
        c.w(32, 4, value);
        c.store(((t19cb00 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b920 str.w r1,[r6,#-0x70]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(56, 4)).wrapping_sub(112u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b924 str r1,[r6,#0x28]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b926 movs r1,#0x0
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
        // 0x6002b928 str r5,[r0,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(32, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b92a movs r0,#0x0
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
        // 0x6002b92c str r4,[r6,#0x20]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b92e ldr r4,[0x6002b9f0]
        let value = c.load(0x6002b9f0, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6002b930 vldr.32 s0,[pc,#0xc0]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610791216u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(192u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002b934 str r3,[sp,#0x24]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002b936 str r5,[r6,#0x2c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b938 str r5,[r6,#0x30]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b93a str r4,[r6,#0x24]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002b93c adr.w r9,0x6002b9e0
        let value = 1610791392u64;
        c.w(68, 4, value);
    }
    {
        // 0x6002b940 ldrd r8,r9,[r9,#0x0]
        let mut t19dc00 = 0u64;
        let mut t1ab800 = 0u64;
        t19dc00 = ((c.r(68, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19dc00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(64, 4, value);
        t1ab800 = (((t19dc00 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t1ab800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(68, 4, value);
    }
    {
        // 0x6002b944 strd r0,r1,[r6,#-0x68]
        let mut t19d800 = 0u64;
        let mut t1fb400 = 0u64;
        t19d800 = ((c.r(56, 4)).wrapping_sub(104u64)) & 0xffffffffu64;
        t1fb400 = ((t19d800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b948 strd r8,r9,[r6,#0x10]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(56, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(64, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(68, 4));
    }
    {
        // 0x6002b94c mov.w r8,#0x0
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
        // 0x6002b950 ldr.w r9,[0x6002b9f8]
        let value = c.load(0x6002b9f8, 4);
        c.w(68, 4, value);
    }
    {
        // 0x6002b954 strd r8,r9,[r6,#0x18]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(56, 4)).wrapping_add(24u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(64, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(68, 4));
    }
    {
        // 0x6002b958 bl 0x60040e08
        let value = (1610791260u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040e08u32;
    }
    0x6002b95c
}
pub fn b_6002b95c(c: &mut Machine) -> u32 {
    {
        // 0x6002b95c bl 0x60041408
        let value = (1610791264u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041408u32;
    }
    0x6002b960
}
pub fn b_6002b960(c: &mut Machine) -> u32 {
    {
        // 0x6002b960 bl 0x60040c70
        let value = (1610791268u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002b964
}
pub fn b_6002b964(c: &mut Machine) -> u32 {
    {
        // 0x6002b964 vcvt.u32.f32 s15,s0
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
        // 0x6002b968 movs r0,#0x0
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
        // 0x6002b96a str r5,[r6,#0x60]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(96u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b96c str r5,[r6,#0x68]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(104u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b96e vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b972 str.w r0,[r11,#0x70c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1804u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b976 str.w r0,[r11,#0x718]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1816u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b97a str.w r0,[r11,#0x724]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1828u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b97e subs r0,r1,#0x1
        let value = (1u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 1u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(1u64);
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
        // 0x6002b980 str r5,[r6,#0x6c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(108u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002b982 str.w r0,[r11,#0x700]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1792u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b986 subs r0,r1,#0x2
        let value = (2u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 2u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(2u64);
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
        // 0x6002b988 str.w r1,[r11,#0x6fc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1788u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b98c str.w r0,[r11,#0x704]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1796u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b990 ldr.w r0,[r11,#0x728]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1832u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b994 cbz r0,0x6002b99e
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x6002b99eu32;
        }
    }
    0x6002b996
}
pub fn b_6002b996(c: &mut Machine) -> u32 {
    {
        // 0x6002b996 bl 0x600414d0
        let value = (1610791322u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414d0u32;
    }
    0x6002b99a
}
pub fn b_6002b99a(c: &mut Machine) -> u32 {
    {
        // 0x6002b99a ldr.w r1,[r11,#0x6fc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1788u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x6002b99e
}
pub fn b_6002b99e(c: &mut Machine) -> u32 {
    {
        // 0x6002b99e lsls r0,r1,#0x2
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
        // 0x6002b9a0 ldr r5,[0x6002ba0c]
        let value = c.load(0x6002ba0c, 4);
        c.w(52, 4, value);
    }
    {
        // 0x6002b9a2 bl 0x600414c0
        let value = (1610791334u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414c0u32;
    }
    0x6002b9a6
}
pub fn b_6002b9a6(c: &mut Machine) -> u32 {
    {
        // 0x6002b9a6 mvn r1,#0x2
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
        // 0x6002b9aa movs r6,#0x0
        let value = 0u64;
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
        // 0x6002b9ac str.w r0,[r11,#0x728]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1832u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b9b0 str.w r1,[r11,#0x71c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1820u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002b9b4 mov r4,r5
        let value = c.r(52, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6002b9b6 mov r1,r5
        let value = c.r(52, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002b9b8 ldr r0,[0x6002b9e8]
        let value = c.load(0x6002b9e8, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002b9ba vldr.32 s0,[pc,#0x38]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610791354u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(56u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002b9be str.w r6,[r1],#0x10
        let mut t19cb00 = 0u64;
        t19cb00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = (c.r(36, 4)).wrapping_add(16u64);
        c.w(36, 4, value);
        c.store(((t19cb00 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002b9c2 str.w r6,[r4,#0xc]!
        let mut t19d200 = 0u64;
        t19d200 = ((c.r(48, 4)).wrapping_add(12u64)) & 0xffffffffu64;
        let value = ((t19d200 >> 0) & 0xffffffffu64);
        c.w(48, 4, value);
        c.store(((t19d200 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002b9c6 str r0,[r1,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(36, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002b9c8 bl 0x60040e08
        let value = (1610791372u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040e08u32;
    }
    0x6002b9cc
}
pub fn b_6002b9cc(c: &mut Machine) -> u32 {
    {
        // 0x6002b9cc bl 0x60041408
        let value = (1610791376u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041408u32;
    }
    0x6002b9d0
}
pub fn b_6002b9d0(c: &mut Machine) -> u32 {
    {
        // 0x6002b9d0 bl 0x60040c70
        let value = (1610791380u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002b9d4
}
pub fn b_6002b9d4(c: &mut Machine) -> u32 {
    {
        // 0x6002b9d4 vcvt.u32.f32 s15,s0
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
        // 0x6002b9d8 movs r0,#0x0
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
        // 0x6002b9da b 0x6002ba10
        return 0x6002ba10u32;
    }
    0x6002b9dc
}
pub fn b_6002ba10(c: &mut Machine) -> u32 {
    {
        // 0x6002ba10 str r6,[r5,#0x20]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002ba12 str r6,[r5,#0x28]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002ba14 vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002ba18 str.w r0,[r11,#0x744]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1860u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002ba1c str.w r0,[r11,#0x750]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1872u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002ba20 str.w r0,[r11,#0x75c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1884u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002ba24 subs r0,r1,#0x1
        let value = (1u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 1u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(1u64);
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
        // 0x6002ba26 str r6,[r5,#0x2c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002ba28 str.w r0,[r11,#0x738]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1848u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002ba2c subs r0,r1,#0x2
        let value = (2u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 2u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(2u64);
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
        // 0x6002ba2e str.w r1,[r11,#0x734]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1844u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002ba32 str.w r0,[r11,#0x73c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1852u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002ba36 ldr.w r0,[r11,#0x760]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1888u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002ba3a cbz r0,0x6002ba44
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x6002ba44u32;
        }
    }
    0x6002ba3c
}
pub fn b_6002ba3c(c: &mut Machine) -> u32 {
    {
        // 0x6002ba3c bl 0x600414d0
        let value = (1610791488u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414d0u32;
    }
    0x6002ba40
}
pub fn b_6002ba40(c: &mut Machine) -> u32 {
    {
        // 0x6002ba40 ldr.w r1,[r11,#0x734]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1844u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x6002ba44
}
pub fn b_6002ba44(c: &mut Machine) -> u32 {
    {
        // 0x6002ba44 lsls r0,r1,#0x2
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
        // 0x6002ba46 ldr r6,[0x6002bd80]
        let value = c.load(0x6002bd80, 4);
        c.w(56, 4, value);
    }
    {
        // 0x6002ba48 bl 0x600414c0
        let value = (1610791500u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414c0u32;
    }
    0x6002ba4c
}
pub fn b_6002ba4c(c: &mut Machine) -> u32 {
    {
        // 0x6002ba4c mov.w r1,#0x3f000000
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
        // 0x6002ba50 str.w r0,[r11,#0x760]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1888u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002ba54 mov.w r0,#0x3f800000
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
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002ba58 str r1,[r5,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002ba5a movs r5,#0x0
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
        // 0x6002ba5c str.w r0,[r6,#-0x74]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(56, 4)).wrapping_sub(116u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002ba60 mvn r0,#0x2
        let mut t187d00 = 0u64;
        t187d00 = (2u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = !((t187d00 >> 0) & 0xffffffffu64);
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002ba64 str r5,[r4,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002ba66 mov r3,r6
        let value = c.r(56, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002ba68 ldr r4,[0x6002bd84]
        let value = c.load(0x6002bd84, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6002ba6a str.w r0,[r11,#0x754]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1876u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002ba6e str.w r0,[r11,#0x71c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1820u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002ba72 mov r0,r6
        let value = c.r(56, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002ba74 str.w r4,[r6,#-0x78]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(56, 4)).wrapping_sub(120u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ba78 str r4,[r6,#0x50]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(80u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ba7a ldr r4,[0x6002bd88]
        let value = c.load(0x6002bd88, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6002ba7c str.w r1,[r3,#-0x6c]!
        let mut t19d000 = 0u64;
        t19d000 = ((c.r(44, 4)).wrapping_sub(108u64)) & 0xffffffffu64;
        let value = ((t19d000 >> 0) & 0xffffffffu64);
        c.w(44, 4, value);
        c.store(((t19d000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002ba80 str.w r5,[r0],#0xc
        let mut t19cb00 = 0u64;
        t19cb00 = (c.r(32, 4)) & 0xffffffffu64;
        let value = (c.r(32, 4)).wrapping_add(12u64);
        c.w(32, 4, value);
        c.store(((t19cb00 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002ba84 str.w r1,[r6,#-0x70]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(56, 4)).wrapping_sub(112u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002ba88 str r1,[r6,#0x28]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002ba8a movs r1,#0x0
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
        // 0x6002ba8c str r5,[r0,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(32, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002ba8e movs r0,#0x0
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
        // 0x6002ba90 str r4,[r6,#0x20]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002ba92 ldr r4,[0x6002bd8c]
        let value = c.load(0x6002bd8c, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6002ba94 vldr.32 s0,[pc,#0x2f8]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610791572u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(760u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002ba98 str r3,[sp,#0x28]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002ba9a str r5,[r6,#0x2c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002ba9c str r5,[r6,#0x30]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002ba9e str r4,[r6,#0x24]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002baa0 adr.w r9,0x6002bd78
        let value = 1610792312u64;
        c.w(68, 4, value);
    }
    {
        // 0x6002baa4 ldrd r8,r9,[r9,#0x0]
        let mut t19dc00 = 0u64;
        let mut t1ab800 = 0u64;
        t19dc00 = ((c.r(68, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19dc00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(64, 4, value);
        t1ab800 = (((t19dc00 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t1ab800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(68, 4, value);
    }
    {
        // 0x6002baa8 strd r0,r1,[r6,#-0x68]
        let mut t19d800 = 0u64;
        let mut t1fb400 = 0u64;
        t19d800 = ((c.r(56, 4)).wrapping_sub(104u64)) & 0xffffffffu64;
        t1fb400 = ((t19d800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002baac strd r8,r9,[r6,#0x10]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(56, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(64, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(68, 4));
    }
    {
        // 0x6002bab0 mov.w r8,#0x0
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
        // 0x6002bab4 ldr.w r9,[0x6002bda8]
        let value = c.load(0x6002bda8, 4);
        c.w(68, 4, value);
    }
    {
        // 0x6002bab8 strd r8,r9,[r6,#0x18]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(56, 4)).wrapping_add(24u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(64, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(68, 4));
    }
    {
        // 0x6002babc bl 0x60040e08
        let value = (1610791616u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040e08u32;
    }
    0x6002bac0
}
pub fn b_6002bac0(c: &mut Machine) -> u32 {
    {
        // 0x6002bac0 bl 0x60041408
        let value = (1610791620u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041408u32;
    }
    0x6002bac4
}
pub fn b_6002bac4(c: &mut Machine) -> u32 {
    {
        // 0x6002bac4 bl 0x60040c70
        let value = (1610791624u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002bac8
}
pub fn b_6002bac8(c: &mut Machine) -> u32 {
    {
        // 0x6002bac8 vcvt.u32.f32 s15,s0
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
        // 0x6002bacc movs r0,#0x0
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
        // 0x6002bace str r5,[r6,#0x60]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(96u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002bad0 str r5,[r6,#0x68]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(104u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002bad2 vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002bad6 str.w r0,[r11,#0x7bc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1980u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bada str.w r0,[r11,#0x7c8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1992u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bade str.w r0,[r11,#0x7d4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2004u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bae2 subs r0,r1,#0x1
        let value = (1u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 1u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(1u64);
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
        // 0x6002bae4 str r5,[r6,#0x6c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(108u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002bae6 str.w r0,[r11,#0x7b0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1968u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002baea subs r0,r1,#0x2
        let value = (2u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 2u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(2u64);
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
        // 0x6002baec str.w r1,[r11,#0x7ac]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1964u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002baf0 str.w r0,[r11,#0x7b4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1972u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002baf4 ldr.w r0,[r11,#0x7d8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2008u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002baf8 cbz r0,0x6002bb02
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x6002bb02u32;
        }
    }
    0x6002bafa
}
pub fn b_6002bafa(c: &mut Machine) -> u32 {
    {
        // 0x6002bafa bl 0x600414d0
        let value = (1610791678u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414d0u32;
    }
    0x6002bafe
}
pub fn b_6002bafe(c: &mut Machine) -> u32 {
    {
        // 0x6002bafe ldr.w r1,[r11,#0x7ac]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1964u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x6002bb02
}
