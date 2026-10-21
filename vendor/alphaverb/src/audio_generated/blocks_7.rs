#![allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut,
    unused_comparisons
)]
use crate::machine::{signed, signed_overflow, FastFma, Machine};
pub fn b_6002bb02(c: &mut Machine) -> u32 {
    {
        // 0x6002bb02 lsls r0,r1,#0x2
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
        // 0x6002bb04 ldr r5,[0x6002bd94]
        let value = c.load(0x6002bd94, 4);
        c.w(52, 4, value);
    }
    {
        // 0x6002bb06 bl 0x600414c0
        let value = (1610791690u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414c0u32;
    }
    0x6002bb0a
}
pub fn b_6002bb0a(c: &mut Machine) -> u32 {
    {
        // 0x6002bb0a mvn r1,#0x2
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
        // 0x6002bb0e movs r6,#0x0
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
        // 0x6002bb10 str.w r0,[r11,#0x7d8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2008u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bb14 str.w r1,[r11,#0x7cc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1996u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002bb18 mov r4,r5
        let value = c.r(52, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6002bb1a mov r1,r5
        let value = c.r(52, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002bb1c ldr r0,[0x6002bd84]
        let value = c.load(0x6002bd84, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002bb1e vldr.32 s0,[pc,#0x270]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610791710u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(624u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002bb22 str.w r6,[r1],#0x10
        let mut t19cb00 = 0u64;
        t19cb00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = (c.r(36, 4)).wrapping_add(16u64);
        c.w(36, 4, value);
        c.store(((t19cb00 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002bb26 str.w r6,[r4,#0xc]!
        let mut t19d200 = 0u64;
        t19d200 = ((c.r(48, 4)).wrapping_add(12u64)) & 0xffffffffu64;
        let value = ((t19d200 >> 0) & 0xffffffffu64);
        c.w(48, 4, value);
        c.store(((t19d200 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002bb2a str r0,[r1,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(36, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bb2c bl 0x60040e08
        let value = (1610791728u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040e08u32;
    }
    0x6002bb30
}
pub fn b_6002bb30(c: &mut Machine) -> u32 {
    {
        // 0x6002bb30 bl 0x60041408
        let value = (1610791732u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041408u32;
    }
    0x6002bb34
}
pub fn b_6002bb34(c: &mut Machine) -> u32 {
    {
        // 0x6002bb34 bl 0x60040c70
        let value = (1610791736u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002bb38
}
pub fn b_6002bb38(c: &mut Machine) -> u32 {
    {
        // 0x6002bb38 vcvt.u32.f32 s15,s0
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
        // 0x6002bb3c movs r0,#0x0
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
        // 0x6002bb3e str r6,[r5,#0x20]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002bb40 str r6,[r5,#0x28]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002bb42 vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002bb46 str.w r0,[r11,#0x7f4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2036u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bb4a str.w r0,[r11,#0x800]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2048u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bb4e str.w r0,[r11,#0x80c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2060u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bb52 subs r0,r1,#0x1
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
        // 0x6002bb54 str r6,[r5,#0x2c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002bb56 str.w r0,[r11,#0x7e8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2024u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bb5a subs r0,r1,#0x2
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
        // 0x6002bb5c str.w r1,[r11,#0x7e4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2020u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002bb60 str.w r0,[r11,#0x7ec]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2028u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bb64 ldr.w r0,[r11,#0x810]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2064u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002bb68 cbz r0,0x6002bb72
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x6002bb72u32;
        }
    }
    0x6002bb6a
}
pub fn b_6002bb6a(c: &mut Machine) -> u32 {
    {
        // 0x6002bb6a bl 0x600414d0
        let value = (1610791790u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414d0u32;
    }
    0x6002bb6e
}
pub fn b_6002bb6e(c: &mut Machine) -> u32 {
    {
        // 0x6002bb6e ldr.w r1,[r11,#0x7e4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2020u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x6002bb72
}
pub fn b_6002bb72(c: &mut Machine) -> u32 {
    {
        // 0x6002bb72 lsls r0,r1,#0x2
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
        // 0x6002bb74 ldr r6,[0x6002bd98]
        let value = c.load(0x6002bd98, 4);
        c.w(56, 4, value);
    }
    {
        // 0x6002bb76 bl 0x600414c0
        let value = (1610791802u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414c0u32;
    }
    0x6002bb7a
}
pub fn b_6002bb7a(c: &mut Machine) -> u32 {
    {
        // 0x6002bb7a mov.w r1,#0x3f000000
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
        // 0x6002bb7e str.w r0,[r11,#0x810]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2064u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bb82 mov.w r0,#0x3f800000
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
        // 0x6002bb86 str r1,[r5,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002bb88 movs r5,#0x0
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
        // 0x6002bb8a str.w r0,[r6,#-0x74]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(56, 4)).wrapping_sub(116u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bb8e mvn r0,#0x2
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
        // 0x6002bb92 str r5,[r4,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002bb94 mov r3,r6
        let value = c.r(56, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002bb96 ldr r4,[0x6002bd84]
        let value = c.load(0x6002bd84, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6002bb98 str.w r0,[r11,#0x804]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2052u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bb9c str.w r0,[r11,#0x7cc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(1996u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bba0 mov r0,r6
        let value = c.r(56, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002bba2 str.w r4,[r6,#-0x78]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(56, 4)).wrapping_sub(120u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002bba6 str r4,[r6,#0x50]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(80u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002bba8 ldr r4,[0x6002bd88]
        let value = c.load(0x6002bd88, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6002bbaa str.w r1,[r3,#-0x6c]!
        let mut t19d000 = 0u64;
        t19d000 = ((c.r(44, 4)).wrapping_sub(108u64)) & 0xffffffffu64;
        let value = ((t19d000 >> 0) & 0xffffffffu64);
        c.w(44, 4, value);
        c.store(((t19d000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002bbae str.w r5,[r0],#0xc
        let mut t19cb00 = 0u64;
        t19cb00 = (c.r(32, 4)) & 0xffffffffu64;
        let value = (c.r(32, 4)).wrapping_add(12u64);
        c.w(32, 4, value);
        c.store(((t19cb00 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002bbb2 str.w r1,[r6,#-0x70]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(56, 4)).wrapping_sub(112u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002bbb6 str r1,[r6,#0x28]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002bbb8 movs r1,#0x0
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
        // 0x6002bbba str r5,[r0,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(32, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002bbbc movs r0,#0x0
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
        // 0x6002bbbe str r4,[r6,#0x20]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002bbc0 ldr r4,[0x6002bd8c]
        let value = c.load(0x6002bd8c, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6002bbc2 vldr.32 s0,[pc,#0x1cc]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610791874u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(460u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002bbc6 str r3,[sp,#0x2c]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002bbc8 str r5,[r6,#0x2c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002bbca str r5,[r6,#0x30]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002bbcc str r4,[r6,#0x24]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002bbce adr.w r9,0x6002bd78
        let value = 1610792312u64;
        c.w(68, 4, value);
    }
    {
        // 0x6002bbd2 ldrd r8,r9,[r9,#0x0]
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
        // 0x6002bbd6 strd r0,r1,[r6,#-0x68]
        let mut t19d800 = 0u64;
        let mut t1fb400 = 0u64;
        t19d800 = ((c.r(56, 4)).wrapping_sub(104u64)) & 0xffffffffu64;
        t1fb400 = ((t19d800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002bbda strd r8,r9,[r6,#0x10]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(56, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(64, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(68, 4));
    }
    {
        // 0x6002bbde mov.w r8,#0x0
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
        // 0x6002bbe2 ldr.w r9,[0x6002bda8]
        let value = c.load(0x6002bda8, 4);
        c.w(68, 4, value);
    }
    {
        // 0x6002bbe6 strd r8,r9,[r6,#0x18]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(56, 4)).wrapping_add(24u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(64, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(68, 4));
    }
    {
        // 0x6002bbea bl 0x60040e08
        let value = (1610791918u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040e08u32;
    }
    0x6002bbee
}
pub fn b_6002bbee(c: &mut Machine) -> u32 {
    {
        // 0x6002bbee bl 0x60041408
        let value = (1610791922u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041408u32;
    }
    0x6002bbf2
}
pub fn b_6002bbf2(c: &mut Machine) -> u32 {
    {
        // 0x6002bbf2 bl 0x60040c70
        let value = (1610791926u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002bbf6
}
pub fn b_6002bbf6(c: &mut Machine) -> u32 {
    {
        // 0x6002bbf6 vcvt.u32.f32 s15,s0
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
        // 0x6002bbfa movs r0,#0x0
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
        // 0x6002bbfc str r5,[r6,#0x60]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(96u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002bbfe str r5,[r6,#0x68]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(104u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002bc00 vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002bc04 str.w r0,[r11,#0x86c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2156u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bc08 str.w r0,[r11,#0x878]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2168u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bc0c str.w r0,[r11,#0x884]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2180u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bc10 subs r0,r1,#0x1
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
        // 0x6002bc12 str r5,[r6,#0x6c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(108u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002bc14 str.w r0,[r11,#0x860]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2144u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bc18 subs r0,r1,#0x2
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
        // 0x6002bc1a str.w r1,[r11,#0x85c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2140u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002bc1e str.w r0,[r11,#0x864]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2148u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bc22 ldr.w r0,[r11,#0x888]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2184u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002bc26 cbz r0,0x6002bc30
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x6002bc30u32;
        }
    }
    0x6002bc28
}
pub fn b_6002bc28(c: &mut Machine) -> u32 {
    {
        // 0x6002bc28 bl 0x600414d0
        let value = (1610791980u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414d0u32;
    }
    0x6002bc2c
}
pub fn b_6002bc2c(c: &mut Machine) -> u32 {
    {
        // 0x6002bc2c ldr.w r1,[r11,#0x85c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2140u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x6002bc30
}
pub fn b_6002bc30(c: &mut Machine) -> u32 {
    {
        // 0x6002bc30 lsls r0,r1,#0x2
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
        // 0x6002bc32 ldr r5,[0x6002bd9c]
        let value = c.load(0x6002bd9c, 4);
        c.w(52, 4, value);
    }
    {
        // 0x6002bc34 bl 0x600414c0
        let value = (1610791992u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414c0u32;
    }
    0x6002bc38
}
pub fn b_6002bc38(c: &mut Machine) -> u32 {
    {
        // 0x6002bc38 mvn r1,#0x2
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
        // 0x6002bc3c movs r6,#0x0
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
        // 0x6002bc3e str.w r0,[r11,#0x888]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2184u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bc42 str.w r1,[r11,#0x87c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2172u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002bc46 mov r4,r5
        let value = c.r(52, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6002bc48 mov r1,r5
        let value = c.r(52, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002bc4a ldr r0,[0x6002bd84]
        let value = c.load(0x6002bd84, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002bc4c vldr.32 s0,[pc,#0x140]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610792012u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(320u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002bc50 str.w r6,[r1],#0x10
        let mut t19cb00 = 0u64;
        t19cb00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = (c.r(36, 4)).wrapping_add(16u64);
        c.w(36, 4, value);
        c.store(((t19cb00 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002bc54 str.w r6,[r4,#0xc]!
        let mut t19d200 = 0u64;
        t19d200 = ((c.r(48, 4)).wrapping_add(12u64)) & 0xffffffffu64;
        let value = ((t19d200 >> 0) & 0xffffffffu64);
        c.w(48, 4, value);
        c.store(((t19d200 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002bc58 str r0,[r1,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(36, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bc5a bl 0x60040e08
        let value = (1610792030u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040e08u32;
    }
    0x6002bc5e
}
pub fn b_6002bc5e(c: &mut Machine) -> u32 {
    {
        // 0x6002bc5e bl 0x60041408
        let value = (1610792034u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041408u32;
    }
    0x6002bc62
}
pub fn b_6002bc62(c: &mut Machine) -> u32 {
    {
        // 0x6002bc62 bl 0x60040c70
        let value = (1610792038u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002bc66
}
pub fn b_6002bc66(c: &mut Machine) -> u32 {
    {
        // 0x6002bc66 vcvt.u32.f32 s15,s0
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
        // 0x6002bc6a movs r0,#0x0
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
        // 0x6002bc6c str r6,[r5,#0x20]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002bc6e str r6,[r5,#0x28]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002bc70 vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002bc74 str.w r0,[r11,#0x8a4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2212u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bc78 str.w r0,[r11,#0x8b0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2224u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bc7c str.w r0,[r11,#0x8bc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2236u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bc80 subs r0,r1,#0x1
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
        // 0x6002bc82 str r6,[r5,#0x2c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002bc84 str.w r0,[r11,#0x898]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2200u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bc88 subs r0,r1,#0x2
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
        // 0x6002bc8a str.w r1,[r11,#0x894]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2196u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002bc8e str.w r0,[r11,#0x89c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2204u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bc92 ldr.w r0,[r11,#0x8c0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2240u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002bc96 cbz r0,0x6002bca0
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x6002bca0u32;
        }
    }
    0x6002bc98
}
pub fn b_6002bc98(c: &mut Machine) -> u32 {
    {
        // 0x6002bc98 bl 0x600414d0
        let value = (1610792092u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414d0u32;
    }
    0x6002bc9c
}
pub fn b_6002bc9c(c: &mut Machine) -> u32 {
    {
        // 0x6002bc9c ldr.w r1,[r11,#0x894]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2196u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x6002bca0
}
pub fn b_6002bca0(c: &mut Machine) -> u32 {
    {
        // 0x6002bca0 lsls r0,r1,#0x2
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
        // 0x6002bca2 ldr r6,[0x6002bda0]
        let value = c.load(0x6002bda0, 4);
        c.w(56, 4, value);
    }
    {
        // 0x6002bca4 bl 0x600414c0
        let value = (1610792104u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414c0u32;
    }
    0x6002bca8
}
pub fn b_6002bca8(c: &mut Machine) -> u32 {
    {
        // 0x6002bca8 mov.w r1,#0x3f000000
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
        // 0x6002bcac str.w r0,[r11,#0x8c0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2240u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bcb0 mov.w r0,#0x3f800000
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
        // 0x6002bcb4 str r1,[r5,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002bcb6 movs r5,#0x0
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
        // 0x6002bcb8 str.w r0,[r6,#-0x74]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(56, 4)).wrapping_sub(116u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bcbc mvn r0,#0x2
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
        // 0x6002bcc0 str r5,[r4,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002bcc2 mov r3,r6
        let value = c.r(56, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002bcc4 ldr r4,[0x6002bd84]
        let value = c.load(0x6002bd84, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6002bcc6 str.w r0,[r11,#0x8b4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2228u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bcca str.w r0,[r11,#0x87c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2172u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bcce mov r0,r6
        let value = c.r(56, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002bcd0 str.w r4,[r6,#-0x78]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(56, 4)).wrapping_sub(120u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002bcd4 str r4,[r6,#0x50]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(80u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002bcd6 ldr r4,[0x6002bd88]
        let value = c.load(0x6002bd88, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6002bcd8 str.w r1,[r3,#-0x6c]!
        let mut t19d000 = 0u64;
        t19d000 = ((c.r(44, 4)).wrapping_sub(108u64)) & 0xffffffffu64;
        let value = ((t19d000 >> 0) & 0xffffffffu64);
        c.w(44, 4, value);
        c.store(((t19d000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002bcdc str.w r5,[r0],#0xc
        let mut t19cb00 = 0u64;
        t19cb00 = (c.r(32, 4)) & 0xffffffffu64;
        let value = (c.r(32, 4)).wrapping_add(12u64);
        c.w(32, 4, value);
        c.store(((t19cb00 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002bce0 str.w r1,[r6,#-0x70]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(56, 4)).wrapping_sub(112u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002bce4 str r1,[r6,#0x28]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002bce6 movs r1,#0x0
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
        // 0x6002bce8 str r5,[r0,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(32, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002bcea movs r0,#0x0
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
        // 0x6002bcec str r4,[r6,#0x20]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002bcee ldr r4,[0x6002bd8c]
        let value = c.load(0x6002bd8c, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6002bcf0 vldr.32 s0,[pc,#0x9c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610792176u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(156u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002bcf4 str r3,[sp,#0x30]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002bcf6 str r5,[r6,#0x2c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002bcf8 str r5,[r6,#0x30]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002bcfa str r4,[r6,#0x24]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002bcfc adr.w r9,0x6002bd78
        let value = 1610792312u64;
        c.w(68, 4, value);
    }
    {
        // 0x6002bd00 ldrd r8,r9,[r9,#0x0]
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
        // 0x6002bd04 strd r0,r1,[r6,#-0x68]
        let mut t19d800 = 0u64;
        let mut t1fb400 = 0u64;
        t19d800 = ((c.r(56, 4)).wrapping_sub(104u64)) & 0xffffffffu64;
        t1fb400 = ((t19d800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002bd08 strd r8,r9,[r6,#0x10]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(56, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(64, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(68, 4));
    }
    {
        // 0x6002bd0c mov.w r8,#0x0
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
        // 0x6002bd10 ldr.w r9,[0x6002bda8]
        let value = c.load(0x6002bda8, 4);
        c.w(68, 4, value);
    }
    {
        // 0x6002bd14 strd r8,r9,[r6,#0x18]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(56, 4)).wrapping_add(24u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(64, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(68, 4));
    }
    {
        // 0x6002bd18 bl 0x60040e08
        let value = (1610792220u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040e08u32;
    }
    0x6002bd1c
}
pub fn b_6002bd1c(c: &mut Machine) -> u32 {
    {
        // 0x6002bd1c bl 0x60041408
        let value = (1610792224u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041408u32;
    }
    0x6002bd20
}
pub fn b_6002bd20(c: &mut Machine) -> u32 {
    {
        // 0x6002bd20 bl 0x60040c70
        let value = (1610792228u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002bd24
}
pub fn b_6002bd24(c: &mut Machine) -> u32 {
    {
        // 0x6002bd24 vcvt.u32.f32 s15,s0
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
        // 0x6002bd28 movs r0,#0x0
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
        // 0x6002bd2a str r5,[r6,#0x60]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(96u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002bd2c str r5,[r6,#0x68]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(104u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002bd2e vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002bd32 str.w r0,[r11,#0x91c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2332u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bd36 str.w r0,[r11,#0x928]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2344u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bd3a str.w r0,[r11,#0x934]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2356u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bd3e subs r0,r1,#0x1
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
        // 0x6002bd40 str r5,[r6,#0x6c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(108u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002bd42 str.w r0,[r11,#0x910]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2320u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bd46 subs r0,r1,#0x2
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
        // 0x6002bd48 str.w r1,[r11,#0x90c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2316u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002bd4c str.w r0,[r11,#0x914]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2324u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bd50 ldr.w r0,[r11,#0x938]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2360u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002bd54 cbz r0,0x6002bd5e
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x6002bd5eu32;
        }
    }
    0x6002bd56
}
pub fn b_6002bd56(c: &mut Machine) -> u32 {
    {
        // 0x6002bd56 bl 0x600414d0
        let value = (1610792282u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414d0u32;
    }
    0x6002bd5a
}
pub fn b_6002bd5a(c: &mut Machine) -> u32 {
    {
        // 0x6002bd5a ldr.w r1,[r11,#0x90c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2316u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x6002bd5e
}
pub fn b_6002bd5e(c: &mut Machine) -> u32 {
    {
        // 0x6002bd5e lsls r0,r1,#0x2
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
        // 0x6002bd60 ldr r5,[0x6002bda4]
        let value = c.load(0x6002bda4, 4);
        c.w(52, 4, value);
    }
    {
        // 0x6002bd62 bl 0x600414c0
        let value = (1610792294u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414c0u32;
    }
    0x6002bd66
}
pub fn b_6002bd66(c: &mut Machine) -> u32 {
    {
        // 0x6002bd66 mvn r1,#0x2
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
        // 0x6002bd6a movs r6,#0x0
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
        // 0x6002bd6c str.w r0,[r11,#0x938]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2360u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bd70 b 0x6002bdac
        return 0x6002bdacu32;
    }
    0x6002bd72
}
pub fn b_6002bdac(c: &mut Machine) -> u32 {
    {
        // 0x6002bdac str.w r1,[r11,#0x92c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2348u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002bdb0 mov r4,r5
        let value = c.r(52, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6002bdb2 mov r1,r5
        let value = c.r(52, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002bdb4 ldr r0,[0x6002c0e8]
        let value = c.load(0x6002c0e8, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002bdb6 vldr.32 s0,[pc,#0x334]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610792374u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(820u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002bdba str.w r6,[r1],#0x10
        let mut t19cb00 = 0u64;
        t19cb00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = (c.r(36, 4)).wrapping_add(16u64);
        c.w(36, 4, value);
        c.store(((t19cb00 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002bdbe str.w r6,[r4,#0xc]!
        let mut t19d200 = 0u64;
        t19d200 = ((c.r(48, 4)).wrapping_add(12u64)) & 0xffffffffu64;
        let value = ((t19d200 >> 0) & 0xffffffffu64);
        c.w(48, 4, value);
        c.store(((t19d200 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002bdc2 str r0,[r1,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(36, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bdc4 bl 0x60040e08
        let value = (1610792392u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040e08u32;
    }
    0x6002bdc8
}
pub fn b_6002bdc8(c: &mut Machine) -> u32 {
    {
        // 0x6002bdc8 bl 0x60041408
        let value = (1610792396u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041408u32;
    }
    0x6002bdcc
}
pub fn b_6002bdcc(c: &mut Machine) -> u32 {
    {
        // 0x6002bdcc bl 0x60040c70
        let value = (1610792400u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002bdd0
}
pub fn b_6002bdd0(c: &mut Machine) -> u32 {
    {
        // 0x6002bdd0 vcvt.u32.f32 s15,s0
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
        // 0x6002bdd4 movs r0,#0x0
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
        // 0x6002bdd6 str r6,[r5,#0x20]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002bdd8 str r6,[r5,#0x28]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002bdda vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002bdde str.w r0,[r11,#0x954]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2388u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bde2 str.w r0,[r11,#0x960]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2400u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bde6 str.w r0,[r11,#0x96c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2412u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bdea subs r0,r1,#0x1
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
        // 0x6002bdec str r6,[r5,#0x2c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002bdee str.w r0,[r11,#0x948]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2376u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bdf2 subs r0,r1,#0x2
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
        // 0x6002bdf4 str.w r1,[r11,#0x944]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2372u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002bdf8 str.w r0,[r11,#0x94c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2380u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bdfc ldr.w r0,[r11,#0x970]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2416u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002be00 cbz r0,0x6002be0a
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x6002be0au32;
        }
    }
    0x6002be02
}
pub fn b_6002be02(c: &mut Machine) -> u32 {
    {
        // 0x6002be02 bl 0x600414d0
        let value = (1610792454u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414d0u32;
    }
    0x6002be06
}
pub fn b_6002be06(c: &mut Machine) -> u32 {
    {
        // 0x6002be06 ldr.w r1,[r11,#0x944]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2372u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x6002be0a
}
pub fn b_6002be0a(c: &mut Machine) -> u32 {
    {
        // 0x6002be0a lsls r0,r1,#0x2
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
        // 0x6002be0c ldr r6,[0x6002c0f0]
        let value = c.load(0x6002c0f0, 4);
        c.w(56, 4, value);
    }
    {
        // 0x6002be0e bl 0x600414c0
        let value = (1610792466u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414c0u32;
    }
    0x6002be12
}
pub fn b_6002be12(c: &mut Machine) -> u32 {
    {
        // 0x6002be12 mov.w r1,#0x3f000000
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
        // 0x6002be16 str.w r0,[r11,#0x970]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2416u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002be1a mov.w r0,#0x3f800000
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
        // 0x6002be1e str r1,[r5,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002be20 movs r5,#0x0
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
        // 0x6002be22 str.w r0,[r6,#-0x74]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(56, 4)).wrapping_sub(116u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002be26 mvn r0,#0x2
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
        // 0x6002be2a str r5,[r4,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002be2c mov r3,r6
        let value = c.r(56, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002be2e ldr r4,[0x6002c0e8]
        let value = c.load(0x6002c0e8, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6002be30 str.w r0,[r11,#0x964]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2404u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002be34 str.w r0,[r11,#0x92c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2348u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002be38 mov r0,r6
        let value = c.r(56, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002be3a str.w r4,[r6,#-0x78]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(56, 4)).wrapping_sub(120u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002be3e str r4,[r6,#0x50]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(80u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002be40 ldr r4,[0x6002c0f4]
        let value = c.load(0x6002c0f4, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6002be42 str.w r1,[r3,#-0x6c]!
        let mut t19d000 = 0u64;
        t19d000 = ((c.r(44, 4)).wrapping_sub(108u64)) & 0xffffffffu64;
        let value = ((t19d000 >> 0) & 0xffffffffu64);
        c.w(44, 4, value);
        c.store(((t19d000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002be46 str.w r5,[r0],#0xc
        let mut t19cb00 = 0u64;
        t19cb00 = (c.r(32, 4)) & 0xffffffffu64;
        let value = (c.r(32, 4)).wrapping_add(12u64);
        c.w(32, 4, value);
        c.store(((t19cb00 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002be4a str.w r1,[r6,#-0x70]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(56, 4)).wrapping_sub(112u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002be4e str r1,[r6,#0x28]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002be50 movs r1,#0x0
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
        // 0x6002be52 str r5,[r0,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(32, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002be54 movs r0,#0x0
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
        // 0x6002be56 str r4,[r6,#0x20]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002be58 ldr r4,[0x6002c0f8]
        let value = c.load(0x6002c0f8, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6002be5a vldr.32 s0,[pc,#0x290]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610792538u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(656u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002be5e str r3,[sp,#0x34]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002be60 str r5,[r6,#0x2c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002be62 str r5,[r6,#0x30]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002be64 str r4,[r6,#0x24]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002be66 adr.w r9,0x6002c0e0
        let value = 1610793184u64;
        c.w(68, 4, value);
    }
    {
        // 0x6002be6a ldrd r8,r9,[r9,#0x0]
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
        // 0x6002be6e strd r0,r1,[r6,#-0x68]
        let mut t19d800 = 0u64;
        let mut t1fb400 = 0u64;
        t19d800 = ((c.r(56, 4)).wrapping_sub(104u64)) & 0xffffffffu64;
        t1fb400 = ((t19d800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002be72 strd r8,r9,[r6,#0x10]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(56, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(64, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(68, 4));
    }
    {
        // 0x6002be76 mov.w r8,#0x0
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
        // 0x6002be7a ldr.w r9,[0x6002c110]
        let value = c.load(0x6002c110, 4);
        c.w(68, 4, value);
    }
    {
        // 0x6002be7e strd r8,r9,[r6,#0x18]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(56, 4)).wrapping_add(24u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(64, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(68, 4));
    }
    {
        // 0x6002be82 bl 0x60040e08
        let value = (1610792582u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040e08u32;
    }
    0x6002be86
}
pub fn b_6002be86(c: &mut Machine) -> u32 {
    {
        // 0x6002be86 bl 0x60041408
        let value = (1610792586u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041408u32;
    }
    0x6002be8a
}
pub fn b_6002be8a(c: &mut Machine) -> u32 {
    {
        // 0x6002be8a bl 0x60040c70
        let value = (1610792590u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002be8e
}
pub fn b_6002be8e(c: &mut Machine) -> u32 {
    {
        // 0x6002be8e vcvt.u32.f32 s15,s0
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
        // 0x6002be92 movs r0,#0x0
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
        // 0x6002be94 str r5,[r6,#0x60]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(96u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002be96 str r5,[r6,#0x68]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(104u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002be98 vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002be9c str.w r0,[r11,#0x9cc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2508u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bea0 str.w r0,[r11,#0x9d8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2520u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bea4 str.w r0,[r11,#0x9e4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2532u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bea8 subs r0,r1,#0x1
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
        // 0x6002beaa str r5,[r6,#0x6c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(108u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002beac str.w r0,[r11,#0x9c0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2496u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002beb0 subs r0,r1,#0x2
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
        // 0x6002beb2 str.w r1,[r11,#0x9bc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2492u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002beb6 str.w r0,[r11,#0x9c4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2500u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002beba ldr.w r0,[r11,#0x9e8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2536u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002bebe cbz r0,0x6002bec8
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x6002bec8u32;
        }
    }
    0x6002bec0
}
pub fn b_6002bec0(c: &mut Machine) -> u32 {
    {
        // 0x6002bec0 bl 0x600414d0
        let value = (1610792644u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414d0u32;
    }
    0x6002bec4
}
pub fn b_6002bec4(c: &mut Machine) -> u32 {
    {
        // 0x6002bec4 ldr.w r1,[r11,#0x9bc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2492u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x6002bec8
}
pub fn b_6002bec8(c: &mut Machine) -> u32 {
    {
        // 0x6002bec8 lsls r0,r1,#0x2
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
        // 0x6002beca ldr r5,[0x6002c0fc]
        let value = c.load(0x6002c0fc, 4);
        c.w(52, 4, value);
    }
    {
        // 0x6002becc bl 0x600414c0
        let value = (1610792656u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414c0u32;
    }
    0x6002bed0
}
pub fn b_6002bed0(c: &mut Machine) -> u32 {
    {
        // 0x6002bed0 mvn r1,#0x2
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
        // 0x6002bed4 movs r6,#0x0
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
        // 0x6002bed6 str.w r0,[r11,#0x9e8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2536u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002beda str.w r1,[r11,#0x9dc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2524u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002bede mov r4,r5
        let value = c.r(52, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6002bee0 mov r1,r5
        let value = c.r(52, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002bee2 ldr r0,[0x6002c0e8]
        let value = c.load(0x6002c0e8, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002bee4 vldr.32 s0,[pc,#0x204]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610792676u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(516u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002bee8 str.w r6,[r1],#0x10
        let mut t19cb00 = 0u64;
        t19cb00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = (c.r(36, 4)).wrapping_add(16u64);
        c.w(36, 4, value);
        c.store(((t19cb00 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002beec str.w r6,[r4,#0xc]!
        let mut t19d200 = 0u64;
        t19d200 = ((c.r(48, 4)).wrapping_add(12u64)) & 0xffffffffu64;
        let value = ((t19d200 >> 0) & 0xffffffffu64);
        c.w(48, 4, value);
        c.store(((t19d200 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002bef0 str r0,[r1,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(36, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bef2 bl 0x60040e08
        let value = (1610792694u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040e08u32;
    }
    0x6002bef6
}
pub fn b_6002bef6(c: &mut Machine) -> u32 {
    {
        // 0x6002bef6 bl 0x60041408
        let value = (1610792698u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041408u32;
    }
    0x6002befa
}
pub fn b_6002befa(c: &mut Machine) -> u32 {
    {
        // 0x6002befa bl 0x60040c70
        let value = (1610792702u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002befe
}
pub fn b_6002befe(c: &mut Machine) -> u32 {
    {
        // 0x6002befe vcvt.u32.f32 s15,s0
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
        // 0x6002bf02 movs r0,#0x0
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
        // 0x6002bf04 str r6,[r5,#0x20]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002bf06 str r6,[r5,#0x28]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002bf08 vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002bf0c str.w r0,[r11,#0xa04]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2564u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bf10 str.w r0,[r11,#0xa10]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2576u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bf14 str.w r0,[r11,#0xa1c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2588u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bf18 subs r0,r1,#0x1
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
        // 0x6002bf1a str r6,[r5,#0x2c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002bf1c str.w r0,[r11,#0x9f8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2552u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bf20 subs r0,r1,#0x2
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
        // 0x6002bf22 str.w r1,[r11,#0x9f4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2548u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002bf26 str.w r0,[r11,#0x9fc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2556u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bf2a ldr.w r0,[r11,#0xa20]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2592u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002bf2e cbz r0,0x6002bf38
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x6002bf38u32;
        }
    }
    0x6002bf30
}
pub fn b_6002bf30(c: &mut Machine) -> u32 {
    {
        // 0x6002bf30 bl 0x600414d0
        let value = (1610792756u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414d0u32;
    }
    0x6002bf34
}
pub fn b_6002bf34(c: &mut Machine) -> u32 {
    {
        // 0x6002bf34 ldr.w r1,[r11,#0x9f4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2548u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x6002bf38
}
pub fn b_6002bf38(c: &mut Machine) -> u32 {
    {
        // 0x6002bf38 lsls r0,r1,#0x2
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
        // 0x6002bf3a ldr r6,[0x6002c100]
        let value = c.load(0x6002c100, 4);
        c.w(56, 4, value);
    }
    {
        // 0x6002bf3c bl 0x600414c0
        let value = (1610792768u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414c0u32;
    }
    0x6002bf40
}
pub fn b_6002bf40(c: &mut Machine) -> u32 {
    {
        // 0x6002bf40 mov.w r1,#0x3f000000
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
        // 0x6002bf44 str.w r0,[r11,#0xa20]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2592u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bf48 movs r0,#0x0
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
        // 0x6002bf4a str r1,[r5,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002bf4c movs r5,#0x0
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
        // 0x6002bf4e str.w r1,[r6,#-0x70]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(56, 4)).wrapping_sub(112u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002bf52 str.w r1,[r6,#-0x6c]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(56, 4)).wrapping_sub(108u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002bf56 str r1,[r6,#0x28]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002bf58 mov.w r1,#0x3f800000
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
        // 0x6002bf5c vldr.32 s0,[pc,#0x18c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610792796u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(396u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002bf60 str.w r1,[r6,#-0x74]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(56, 4)).wrapping_sub(116u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002bf64 mvn r1,#0x2
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
        // 0x6002bf68 str r5,[r4,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002bf6a str.w r1,[r11,#0xa14]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2580u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002bf6e str.w r1,[r11,#0x9dc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2524u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002bf72 mov r1,r6
        let value = c.r(56, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002bf74 str r5,[r6,#0x2c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002bf76 str.w r5,[r1],#0xc
        let mut t19cb00 = 0u64;
        t19cb00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = (c.r(36, 4)).wrapping_add(12u64);
        c.w(36, 4, value);
        c.store(((t19cb00 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002bf7a str r5,[r1,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(36, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002bf7c ldr r1,[0x6002c0e8]
        let value = c.load(0x6002c0e8, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002bf7e str r5,[r6,#0x30]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002bf80 str.w r1,[r6,#-0x78]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(56, 4)).wrapping_sub(120u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002bf84 str r1,[r6,#0x50]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(80u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002bf86 ldr r1,[0x6002c0f4]
        let value = c.load(0x6002c0f4, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002bf88 adr.w r9,0x6002c0e0
        let value = 1610793184u64;
        c.w(68, 4, value);
    }
    {
        // 0x6002bf8c ldrd r8,r9,[r9,#0x0]
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
        // 0x6002bf90 str r1,[r6,#0x20]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002bf92 ldr r1,[0x6002c0f8]
        let value = c.load(0x6002c0f8, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002bf94 strd r8,r9,[r6,#0x10]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(56, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(64, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(68, 4));
    }
    {
        // 0x6002bf98 str r1,[r6,#0x24]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002bf9a mov.w r8,#0x0
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
        // 0x6002bf9e movs r1,#0x0
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
        // 0x6002bfa0 ldr.w r9,[0x6002c110]
        let value = c.load(0x6002c110, 4);
        c.w(68, 4, value);
    }
    {
        // 0x6002bfa4 strd r0,r1,[r6,#-0x68]
        let mut t19d800 = 0u64;
        let mut t1fb400 = 0u64;
        t19d800 = ((c.r(56, 4)).wrapping_sub(104u64)) & 0xffffffffu64;
        t1fb400 = ((t19d800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002bfa8 strd r8,r9,[r6,#0x18]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(56, 4)).wrapping_add(24u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(64, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(68, 4));
    }
    {
        // 0x6002bfac bl 0x60040e08
        let value = (1610792880u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040e08u32;
    }
    0x6002bfb0
}
pub fn b_6002bfb0(c: &mut Machine) -> u32 {
    {
        // 0x6002bfb0 bl 0x60041408
        let value = (1610792884u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041408u32;
    }
    0x6002bfb4
}
pub fn b_6002bfb4(c: &mut Machine) -> u32 {
    {
        // 0x6002bfb4 bl 0x60040c70
        let value = (1610792888u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002bfb8
}
pub fn b_6002bfb8(c: &mut Machine) -> u32 {
    {
        // 0x6002bfb8 vcvt.u32.f32 s15,s0
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
        // 0x6002bfbc movs r0,#0x0
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
        // 0x6002bfbe str r5,[r6,#0x60]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(96u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002bfc0 str r5,[r6,#0x68]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(104u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002bfc2 vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002bfc6 str.w r0,[r11,#0xa7c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2684u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bfca str.w r0,[r11,#0xa88]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2696u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bfce str.w r0,[r11,#0xa94]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2708u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bfd2 subs r0,r1,#0x1
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
        // 0x6002bfd4 str r5,[r6,#0x6c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(108u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002bfd6 str.w r0,[r11,#0xa70]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2672u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bfda subs r0,r1,#0x2
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
        // 0x6002bfdc str.w r1,[r11,#0xa6c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2668u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002bfe0 str.w r0,[r11,#0xa74]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2676u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002bfe4 ldr.w r0,[r11,#0xa98]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2712u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002bfe8 cbz r0,0x6002bff2
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x6002bff2u32;
        }
    }
    0x6002bfea
}
pub fn b_6002bfea(c: &mut Machine) -> u32 {
    {
        // 0x6002bfea bl 0x600414d0
        let value = (1610792942u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414d0u32;
    }
    0x6002bfee
}
pub fn b_6002bfee(c: &mut Machine) -> u32 {
    {
        // 0x6002bfee ldr.w r1,[r11,#0xa6c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2668u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x6002bff2
}
pub fn b_6002bff2(c: &mut Machine) -> u32 {
    {
        // 0x6002bff2 lsls r0,r1,#0x2
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
        // 0x6002bff4 ldr r5,[0x6002c104]
        let value = c.load(0x6002c104, 4);
        c.w(52, 4, value);
    }
    {
        // 0x6002bff6 bl 0x600414c0
        let value = (1610792954u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414c0u32;
    }
    0x6002bffa
}
pub fn b_6002bffa(c: &mut Machine) -> u32 {
    {
        // 0x6002bffa mvn r1,#0x2
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
        // 0x6002bffe movs r6,#0x0
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
        // 0x6002c000 str.w r0,[r11,#0xa98]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2712u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c004 str.w r1,[r11,#0xa8c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2700u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c008 mov r4,r5
        let value = c.r(52, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6002c00a mov r1,r5
        let value = c.r(52, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002c00c ldr r0,[0x6002c0e8]
        let value = c.load(0x6002c0e8, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002c00e vldr.32 s0,[pc,#0xdc]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610792974u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(220u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002c012 str.w r6,[r1],#0x10
        let mut t19cb00 = 0u64;
        t19cb00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = (c.r(36, 4)).wrapping_add(16u64);
        c.w(36, 4, value);
        c.store(((t19cb00 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002c016 str.w r6,[r4,#0xc]!
        let mut t19d200 = 0u64;
        t19d200 = ((c.r(48, 4)).wrapping_add(12u64)) & 0xffffffffu64;
        let value = ((t19d200 >> 0) & 0xffffffffu64);
        c.w(48, 4, value);
        c.store(((t19d200 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002c01a str r0,[r1,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(36, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c01c bl 0x60040e08
        let value = (1610792992u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040e08u32;
    }
    0x6002c020
}
pub fn b_6002c020(c: &mut Machine) -> u32 {
    {
        // 0x6002c020 bl 0x60041408
        let value = (1610792996u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041408u32;
    }
    0x6002c024
}
pub fn b_6002c024(c: &mut Machine) -> u32 {
    {
        // 0x6002c024 bl 0x60040c70
        let value = (1610793000u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002c028
}
pub fn b_6002c028(c: &mut Machine) -> u32 {
    {
        // 0x6002c028 vcvt.u32.f32 s15,s0
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
        // 0x6002c02c movs r0,#0x0
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
        // 0x6002c02e str r6,[r5,#0x20]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002c030 str r6,[r5,#0x28]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002c032 vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002c036 str.w r0,[r11,#0xab4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2740u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c03a str.w r0,[r11,#0xac0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2752u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c03e str.w r0,[r11,#0xacc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2764u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c042 subs r0,r1,#0x1
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
        // 0x6002c044 str r6,[r5,#0x2c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002c046 str.w r0,[r11,#0xaa8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2728u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c04a subs r0,r1,#0x2
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
        // 0x6002c04c str.w r1,[r11,#0xaa4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2724u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c050 str.w r0,[r11,#0xaac]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2732u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c054 ldr.w r0,[r11,#0xad0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2768u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002c058 cbz r0,0x6002c062
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x6002c062u32;
        }
    }
    0x6002c05a
}
pub fn b_6002c05a(c: &mut Machine) -> u32 {
    {
        // 0x6002c05a bl 0x600414d0
        let value = (1610793054u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414d0u32;
    }
    0x6002c05e
}
pub fn b_6002c05e(c: &mut Machine) -> u32 {
    {
        // 0x6002c05e ldr.w r1,[r11,#0xaa4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2724u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x6002c062
}
pub fn b_6002c062(c: &mut Machine) -> u32 {
    {
        // 0x6002c062 lsls r0,r1,#0x2
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
        // 0x6002c064 ldr r6,[0x6002c108]
        let value = c.load(0x6002c108, 4);
        c.w(56, 4, value);
    }
    {
        // 0x6002c066 bl 0x600414c0
        let value = (1610793066u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414c0u32;
    }
    0x6002c06a
}
pub fn b_6002c06a(c: &mut Machine) -> u32 {
    {
        // 0x6002c06a mov.w r1,#0x3f000000
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
        // 0x6002c06e str.w r0,[r11,#0xad0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2768u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c072 movs r0,#0x0
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
        // 0x6002c074 str r1,[r5,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c076 movs r5,#0x0
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
        // 0x6002c078 str.w r1,[r6,#-0x70]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(56, 4)).wrapping_sub(112u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c07c str.w r1,[r6,#-0x6c]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(56, 4)).wrapping_sub(108u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c080 str r1,[r6,#0x28]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c082 mov.w r1,#0x3f800000
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
        // 0x6002c086 vldr.32 s0,[pc,#0x84]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610793094u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(132u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002c08a str.w r1,[r6,#-0x74]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(56, 4)).wrapping_sub(116u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c08e mvn r1,#0x2
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
        // 0x6002c092 str r5,[r4,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002c094 str.w r1,[r11,#0xac4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2756u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c098 str.w r1,[r11,#0xa8c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2700u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c09c mov r1,r6
        let value = c.r(56, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002c09e str r5,[r6,#0x2c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002c0a0 str.w r5,[r1],#0xc
        let mut t19cb00 = 0u64;
        t19cb00 = (c.r(36, 4)) & 0xffffffffu64;
        let value = (c.r(36, 4)).wrapping_add(12u64);
        c.w(36, 4, value);
        c.store(((t19cb00 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002c0a4 str r5,[r1,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(36, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002c0a6 ldr r1,[0x6002c0e8]
        let value = c.load(0x6002c0e8, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002c0a8 str r5,[r6,#0x30]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002c0aa str.w r1,[r6,#-0x78]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(56, 4)).wrapping_sub(120u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c0ae str r1,[r6,#0x64]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(100u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c0b0 ldr r1,[0x6002c0f4]
        let value = c.load(0x6002c0f4, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002c0b2 adr.w r9,0x6002c0e0
        let value = 1610793184u64;
        c.w(68, 4, value);
    }
    {
        // 0x6002c0b6 ldrd r8,r9,[r9,#0x0]
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
        // 0x6002c0ba str r1,[r6,#0x20]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c0bc ldr r1,[0x6002c0f8]
        let value = c.load(0x6002c0f8, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002c0be strd r8,r9,[r6,#0x10]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(56, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(64, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(68, 4));
    }
    {
        // 0x6002c0c2 str r1,[r6,#0x24]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c0c4 mov.w r8,#0x0
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
        // 0x6002c0c8 movs r1,#0x0
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
        // 0x6002c0ca ldr.w r9,[0x6002c110]
        let value = c.load(0x6002c110, 4);
        c.w(68, 4, value);
    }
    {
        // 0x6002c0ce strd r0,r1,[r6,#-0x68]
        let mut t19d800 = 0u64;
        let mut t1fb400 = 0u64;
        t19d800 = ((c.r(56, 4)).wrapping_sub(104u64)) & 0xffffffffu64;
        t1fb400 = ((t19d800 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c0d2 strd r8,r9,[r6,#0x18]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(56, 4)).wrapping_add(24u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(64, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(68, 4));
    }
    {
        // 0x6002c0d6 bl 0x60040e08
        let value = (1610793178u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040e08u32;
    }
    0x6002c0da
}
pub fn b_6002c0da(c: &mut Machine) -> u32 {
    {
        // 0x6002c0da bl 0x60041408
        let value = (1610793182u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041408u32;
    }
    0x6002c0de
}
pub fn b_6002c0de(c: &mut Machine) -> u32 {
    {
        // 0x6002c0de b 0x6002c114
        return 0x6002c114u32;
    }
    0x6002c0e0
}
pub fn b_6002c114(c: &mut Machine) -> u32 {
    {
        // 0x6002c114 bl 0x60040c70
        let value = (1610793240u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002c118
}
pub fn b_6002c118(c: &mut Machine) -> u32 {
    {
        // 0x6002c118 vcvt.u32.f32 s15,s0
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
        // 0x6002c11c movs r0,#0x0
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
        // 0x6002c11e str r5,[r6,#0x74]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(116u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002c120 str r5,[r6,#0x7c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(56, 4)).wrapping_add(124u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002c122 vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002c126 str.w r0,[r11,#0xb40]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2880u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c12a str.w r0,[r11,#0xb4c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2892u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c12e str.w r0,[r11,#0xb58]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2904u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c132 subs r0,r1,#0x1
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
        // 0x6002c134 str.w r5,[r6,#0x80]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(56, 4)).wrapping_add(128u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(52, 4));
    }
    {
        // 0x6002c138 str.w r0,[r11,#0xb34]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2868u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c13c subs r0,r1,#0x2
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
        // 0x6002c13e str.w r1,[r11,#0xb30]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2864u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c142 str.w r0,[r11,#0xb38]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2872u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c146 ldr.w r0,[r11,#0xb5c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2908u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002c14a cbz r0,0x6002c154
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x6002c154u32;
        }
    }
    0x6002c14c
}
pub fn b_6002c14c(c: &mut Machine) -> u32 {
    {
        // 0x6002c14c bl 0x600414d0
        let value = (1610793296u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414d0u32;
    }
    0x6002c150
}
pub fn b_6002c150(c: &mut Machine) -> u32 {
    {
        // 0x6002c150 ldr.w r1,[r11,#0xb30]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2864u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x6002c154
}
pub fn b_6002c154(c: &mut Machine) -> u32 {
    {
        // 0x6002c154 ldr r4,[0x6002c4fc]
        let value = c.load(0x6002c4fc, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6002c156 lsls r0,r1,#0x2
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
        // 0x6002c158 bl 0x600414c0
        let value = (1610793308u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414c0u32;
    }
    0x6002c15c
}
pub fn b_6002c15c(c: &mut Machine) -> u32 {
    {
        // 0x6002c15c str.w r0,[r11,#0xb5c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2908u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c160 mov r5,r4
        let value = c.r(48, 4);
        c.w(52, 4, value);
    }
    {
        // 0x6002c162 mov.w r0,#0x3f000000
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
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002c166 movs r1,#0x0
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
        // 0x6002c168 vldr.32 s15,[pc,#0x394]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610793320u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(916u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002c16c str.w r0,[r5,#-0x48]!
        let mut t19d000 = 0u64;
        t19d000 = ((c.r(52, 4)).wrapping_sub(72u64)) & 0xffffffffu64;
        let value = ((t19d000 >> 0) & 0xffffffffu64);
        c.w(52, 4, value);
        c.store(((t19d000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c170 mov r0,r4
        let value = c.r(48, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002c172 str.w r1,[r4,#-0x30]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(48, 4)).wrapping_sub(48u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c176 mov r3,r4
        let value = c.r(48, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002c178 str.w r1,[r4,#-0x2c]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(48, 4)).wrapping_sub(44u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c17c mov r6,r4
        let value = c.r(48, 4);
        c.w(56, 4, value);
    }
    {
        // 0x6002c17e str.w r1,[r0],#0xc
        let mut t19cb00 = 0u64;
        t19cb00 = (c.r(32, 4)) & 0xffffffffu64;
        let value = (c.r(32, 4)).wrapping_add(12u64);
        c.w(32, 4, value);
        c.store(((t19cb00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c182 mov r9,r4
        let value = c.r(48, 4);
        c.w(68, 4, value);
    }
    {
        // 0x6002c184 str r1,[r0,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(32, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c186 mvn r1,#0x2
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
        // 0x6002c18a vldr.32 s0,[r4,#-0x44]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(48, 4)).wrapping_sub(68u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002c18e mov.w r12,#0x3fc00000
        let mut t188100 = 0u64;
        let mut t188300 = 0u64;
        t188100 = (1069547520u64) & 0xffffffffu64;
        t188300 = (if 31u64 >= 32 {
            0
        } else {
            ((t188100 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188300 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((t188100 >> 0) & 0xffffffffu64);
        c.w(80, 4, value);
        let value = (signed(c.r(80, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(80, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002c192 str.w r1,[r11,#0xb50]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2896u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c196 vmov.f32 s16,0xc0000000
        let value = 3221225472u64;
        c.w(832, 4, value);
    }
    {
        // 0x6002c19a ldr r1,[0x6002c504]
        let value = c.load(0x6002c504, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002c19c vmul.f32 s0,s0,s15
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002c1a0 ldr r7,[0x6002c508]
        let value = c.load(0x6002c508, 4);
        c.w(60, 4, value);
    }
    {
        // 0x6002c1a2 vmov.f32 s17,0x3f800000
        let value = 1065353216u64;
        c.w(836, 4, value);
    }
    {
        // 0x6002c1a6 str.w r1,[r4,#-0x54]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(48, 4)).wrapping_sub(84u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c1aa sub.w r8,r4,#0x44
        let mut t187d00 = 0u64;
        t187d00 = (68u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(48, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(64, 4, value);
        let value = (signed(c.r(64, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(64, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002c1ae ldr r1,[0x6002c50c]
        let value = c.load(0x6002c50c, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002c1b0 str.w r7,[r9,#-0x38]!
        let mut t19d000 = 0u64;
        t19d000 = ((c.r(68, 4)).wrapping_sub(56u64)) & 0xffffffffu64;
        let value = ((t19d000 >> 0) & 0xffffffffu64);
        c.w(68, 4, value);
        c.store(((t19d000 >> 0) & 0xffffffffu64) as u32, 4, c.r(60, 4));
    }
    {
        // 0x6002c1b4 sub.w r7,r4,#0x3c
        let mut t187d00 = 0u64;
        t187d00 = (60u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(48, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(60, 4, value);
        let value = (signed(c.r(60, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(60, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002c1b8 str.w r12,[r3,#-0x4c]!
        let mut t19d000 = 0u64;
        t19d000 = ((c.r(44, 4)).wrapping_sub(76u64)) & 0xffffffffu64;
        let value = ((t19d000 >> 0) & 0xffffffffu64);
        c.w(44, 4, value);
        c.store(((t19d000 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c1bc str.w r1,[r6,#-0x50]!
        let mut t19d000 = 0u64;
        t19d000 = ((c.r(56, 4)).wrapping_sub(80u64)) & 0xffffffffu64;
        let value = ((t19d000 >> 0) & 0xffffffffu64);
        c.w(56, 4, value);
        c.store(((t19d000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c1c0 str r3,[sp,#0x8]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002c1c2 bl 0x60040c70
        let value = (1610793414u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002c1c6
}
pub fn b_6002c1c6(c: &mut Machine) -> u32 {
    {
        // 0x6002c1c6 vmov.f32 s13,s0
        let value = c.r(768, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002c1ca vldr.32 s15,[r5]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(52, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002c1ce vldr.32 s14,[r6]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(56, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002c1d2 vmul.f32 s15,s15,s16
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(832, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002c1d6 str r4,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002c1d8 vadd.f32 s13,s13,s17
        let value = (f32::from_bits(c.r(820, 4) as u32) + f32::from_bits(c.r(836, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002c1dc ldr r1,[0x6002c510]
        let value = c.load(0x6002c510, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002c1de subs r4,#0x34
        let value = (52u64 <= c.r(48, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 52u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_sub(52u64);
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
        // 0x6002c1e0 vldr.32 s0,[pc,#0x330]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610793440u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(816u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002c1e4 vstr.32 s15,[r9]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(68, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002c1e8 vdiv.f32 s12,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) / f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x6002c1ec str.w r1,[r8,#0x0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(64, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c1f0 vmul.f32 s15,s14,s12
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002c1f4 vstr.32 s12,[r7]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(60, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x6002c1f8 vstr.32 s15,[r4]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(48, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002c1fc bl 0x60040c70
        let value = (1610793472u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002c200
}
pub fn b_6002c200(c: &mut Machine) -> u32 {
    {
        // 0x6002c200 vldr.32 s14,[r5]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(52, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002c204 vadd.f32 s0,s0,s17
        let value = (f32::from_bits(c.r(768, 4) as u32) + f32::from_bits(c.r(836, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002c208 ldr r3,[sp,#0x8]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002c20a vmul.f32 s14,s14,s16
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(832, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002c20e vldr.32 s11,[pc,#0x308]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610793486u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(776u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x6002c212 vldr.32 s13,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002c216 vldr.32 s15,[r6]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(56, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002c21a vdiv.f32 s12,s0,s14
        let value = (f32::from_bits(c.r(768, 4) as u32) / f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x6002c21e ldr r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002c220 ldr r1,[0x6002c51c]
        let value = c.load(0x6002c51c, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002c222 str.w r1,[r3,#-0x40]!
        let mut t19d000 = 0u64;
        t19d000 = ((c.r(44, 4)).wrapping_sub(64u64)) & 0xffffffffu64;
        let value = ((t19d000 >> 0) & 0xffffffffu64);
        c.w(44, 4, value);
        c.store(((t19d000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c226 str r3,[sp,#0x4]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002c228 vcmpe.f32 s13,s11
        let mut t35c00 = 0u64;
        let mut t35e00 = 0u64;
        let mut t36000 = 0u64;
        let mut t36100 = 0u64;
        let mut t36200 = 0u64;
        let mut t36400 = 0u64;
        let mut t36500 = 0u64;
        let mut t36600 = 0u64;
        let mut t36700 = 0u64;
        let mut t36800 = 0u64;
        let mut t36900 = 0u64;
        let mut t36a00 = 0u64;
        let mut t36b00 = 0u64;
        let mut t36c00 = 0u64;
        let mut t36d00 = 0u64;
        let mut t36e00 = 0u64;
        let mut t36f00 = 0u64;
        let mut t37000 = 0u64;
        t35c00 = ((f32::from_bits(c.r(820, 4) as u32) < f32::from_bits(c.r(812, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(820, 4) as u32) == f32::from_bits(c.r(812, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(812, 4) as u32) <= f32::from_bits(c.r(820, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(820, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002c22c vmrs apsr,fpscr
        let mut t6bd00 = 0u64;
        let mut t6bf00 = 0u64;
        let mut t6c000 = 0u64;
        let mut t6c200 = 0u64;
        let mut t6c300 = 0u64;
        let mut t6c500 = 0u64;
        let mut t6c600 = 0u64;
        t6bd00 = (if 31u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t6bd00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(96, 1, value);
        t6bf00 = (if 30u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (30u64 as u32)
        }) & 0xffffffffu64;
        t6c000 = (((t6bf00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c000 >> 0) & 0xffu64) & 1u64);
        c.w(97, 1, value);
        t6c200 = (if 29u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (29u64 as u32)
        }) & 0xffffffffu64;
        t6c300 = (((t6c200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c300 >> 0) & 0xffu64) & 1u64);
        c.w(98, 1, value);
        t6c500 = (if 28u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (28u64 as u32)
        }) & 0xffffffffu64;
        t6c600 = (((t6c500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c600 >> 0) & 0xffu64) & 1u64);
        c.w(99, 1, value);
    }
    {
        // 0x6002c230 vmul.f32 s15,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002c234 vstr.32 s12,[r7]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(60, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x6002c238 vstr.32 s15,[r4]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(48, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002c23c bgt.w 0x6002db6e
        let mut t183c00 = 0u64;
        let mut t183d00 = 0u64;
        let mut t183f00 = 0u64;
        t183c00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t183d00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t183f00 = (((((t183c00 >> 0) & 0xffu64) != 0) & (((t183d00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t183f00 >> 0) & 0xffu64) != 0 {
            return 0x6002db6eu32;
        }
    }
    0x6002c240
}
pub fn b_6002c240(c: &mut Machine) -> u32 {
    {
        // 0x6002c240 vmul.f32 s16,s13,s16
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(832, 4) as u32))
            .to_bits() as u64;
        c.w(832, 4, value);
    }
    0x6002c244
}
pub fn b_6002c244(c: &mut Machine) -> u32 {
    {
        // 0x6002c244 vldr.32 s15,[pc,#0x2b8]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610793540u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(696u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002c248 vmov.f32 s17,0xc0000000
        let value = 3221225472u64;
        c.w(836, 4, value);
    }
    {
        // 0x6002c24c vldr.32 s0,[r8]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(64, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002c250 vmov.f32 s18,0x3f800000
        let value = 1065353216u64;
        c.w(840, 4, value);
    }
    {
        // 0x6002c254 vstr.32 s13,[r5]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(52, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x6002c258 vmul.f32 s0,s0,s15
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002c25c vstr.32 s16,[r9]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(68, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(832, 4));
    }
    {
        // 0x6002c260 bl 0x60040c70
        let value = (1610793572u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002c264
}
pub fn b_6002c264(c: &mut Machine) -> u32 {
    {
        // 0x6002c264 vmov.f32 s13,s0
        let value = c.r(768, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002c268 vldr.32 s15,[r5]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(52, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002c26c vldr.32 s14,[r6]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(56, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002c270 vmul.f32 s15,s15,s17
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(836, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002c274 ldr r1,[0x6002c510]
        let value = c.load(0x6002c510, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002c276 vadd.f32 s13,s13,s18
        let value = (f32::from_bits(c.r(820, 4) as u32) + f32::from_bits(c.r(840, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002c27a vldr.32 s0,[pc,#0x298]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610793594u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(664u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002c27e str.w r1,[r8,#0x0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(64, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c282 vstr.32 s15,[r9]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(68, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002c286 vdiv.f32 s12,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) / f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x6002c28a vmul.f32 s15,s14,s12
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002c28e vstr.32 s12,[r7]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(60, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x6002c292 vstr.32 s15,[r4]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(48, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002c296 bl 0x60040c70
        let value = (1610793626u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002c29a
}
pub fn b_6002c29a(c: &mut Machine) -> u32 {
    {
        // 0x6002c29a vmov.f32 s13,s0
        let value = c.r(768, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002c29e vldr.32 s14,[r5]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(52, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002c2a2 vldr.32 s15,[r6]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(56, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002c2a6 vmul.f32 s14,s14,s17
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(836, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002c2aa ldr r1,[0x6002c504]
        let value = c.load(0x6002c504, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002c2ac vadd.f32 s13,s13,s18
        let value = (f32::from_bits(c.r(820, 4) as u32) + f32::from_bits(c.r(840, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002c2b0 vldr.32 s0,[pc,#0x26c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610793648u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(620u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002c2b4 vdiv.f32 s12,s13,s14
        let value = (f32::from_bits(c.r(820, 4) as u32) / f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x6002c2b8 vmul.f32 s15,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002c2bc vstr.32 s12,[r7]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(60, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x6002c2c0 vstr.32 s15,[r4]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(48, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002c2c4 ldr r4,[0x6002c524]
        let value = c.load(0x6002c524, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6002c2c6 mov r5,r4
        let value = c.r(48, 4);
        c.w(52, 4, value);
    }
    {
        // 0x6002c2c8 str.w r1,[r5],#0x10
        let mut t19cb00 = 0u64;
        t19cb00 = (c.r(52, 4)) & 0xffffffffu64;
        let value = (c.r(52, 4)).wrapping_add(16u64);
        c.w(52, 4, value);
        c.store(((t19cb00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c2cc bl 0x60040e08
        let value = (1610793680u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040e08u32;
    }
    0x6002c2d0
}
pub fn b_6002c2d0(c: &mut Machine) -> u32 {
    {
        // 0x6002c2d0 bl 0x60041408
        let value = (1610793684u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60041408u32;
    }
    0x6002c2d4
}
pub fn b_6002c2d4(c: &mut Machine) -> u32 {
    {
        // 0x6002c2d4 bl 0x60040c70
        let value = (1610793688u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002c2d8
}
pub fn b_6002c2d8(c: &mut Machine) -> u32 {
    {
        // 0x6002c2d8 vcvt.u32.f32 s15,s0
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
        // 0x6002c2dc movs r1,#0x0
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
        // 0x6002c2de movs r0,#0x0
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
        // 0x6002c2e0 str r1,[r5,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c2e2 str r1,[r4,#0x18]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(24u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c2e4 str r1,[r4,#0x1c]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c2e6 vmov r1,s15
        let value = c.r(828, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002c2ea str.w r0,[r11,#0xba4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2980u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c2ee str.w r0,[r11,#0xbb0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2992u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c2f2 str.w r0,[r11,#0xbbc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3004u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c2f6 subs r0,r1,#0x1
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
        // 0x6002c2f8 str.w r1,[r11,#0xb94]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2964u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c2fc str.w r0,[r11,#0xb98]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2968u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c300 subs r0,r1,#0x2
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
        // 0x6002c302 str.w r0,[r11,#0xb9c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2972u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c306 ldr.w r0,[r11,#0xbc0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3008u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002c30a cbz r0,0x6002c314
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(32, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x6002c314u32;
        }
    }
    0x6002c30c
}
pub fn b_6002c30c(c: &mut Machine) -> u32 {
    {
        // 0x6002c30c bl 0x600414d0
        let value = (1610793744u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414d0u32;
    }
    0x6002c310
}
pub fn b_6002c310(c: &mut Machine) -> u32 {
    {
        // 0x6002c310 ldr.w r1,[r11,#0xb94]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2964u64)) & 0xffffffffu64;
        let value = c.load(((t19c800 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x6002c314
}
pub fn b_6002c314(c: &mut Machine) -> u32 {
    {
        // 0x6002c314 ldr r4,[0x6002c528]
        let value = c.load(0x6002c528, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6002c316 lsls r0,r1,#0x2
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
        // 0x6002c318 bl 0x600414c0
        let value = (1610793756u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x600414c0u32;
    }
    0x6002c31c
}
pub fn b_6002c31c(c: &mut Machine) -> u32 {
    {
        // 0x6002c31c str.w r0,[r11,#0xbc0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3008u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c320 mov r5,r4
        let value = c.r(48, 4);
        c.w(52, 4, value);
    }
    {
        // 0x6002c322 mov.w r0,#0x3f000000
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
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002c326 movs r1,#0x0
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
        // 0x6002c328 vldr.32 s15,[pc,#0x1d4]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610793768u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(468u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002c32c str.w r0,[r5,#-0x48]!
        let mut t19d000 = 0u64;
        t19d000 = ((c.r(52, 4)).wrapping_sub(72u64)) & 0xffffffffu64;
        let value = ((t19d000 >> 0) & 0xffffffffu64);
        c.w(52, 4, value);
        c.store(((t19d000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c330 mov r0,r4
        let value = c.r(48, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002c332 str.w r1,[r4,#-0x30]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(48, 4)).wrapping_sub(48u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c336 mov r3,r4
        let value = c.r(48, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002c338 str.w r1,[r4,#-0x2c]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(48, 4)).wrapping_sub(44u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c33c mov r6,r4
        let value = c.r(48, 4);
        c.w(56, 4, value);
    }
    {
        // 0x6002c33e str.w r1,[r0],#0xc
        let mut t19cb00 = 0u64;
        t19cb00 = (c.r(32, 4)) & 0xffffffffu64;
        let value = (c.r(32, 4)).wrapping_add(12u64);
        c.w(32, 4, value);
        c.store(((t19cb00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c342 mov r9,r4
        let value = c.r(48, 4);
        c.w(68, 4, value);
    }
    {
        // 0x6002c344 str r1,[r0,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(32, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c346 mvn r1,#0x2
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
        // 0x6002c34a vldr.32 s0,[r4,#-0x44]
        let mut t63e00 = 0u64;
        t63e00 = ((c.r(48, 4)).wrapping_sub(68u64)) & 0xffffffffu64;
        let value = c.load(((t63e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002c34e mov.w r12,#0x3fc00000
        let mut t188100 = 0u64;
        let mut t188300 = 0u64;
        t188100 = (1069547520u64) & 0xffffffffu64;
        t188300 = (if 31u64 >= 32 {
            0
        } else {
            ((t188100 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188300 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((t188100 >> 0) & 0xffffffffu64);
        c.w(80, 4, value);
        let value = (signed(c.r(80, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(80, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002c352 str.w r1,[r11,#0xbb4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(2996u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c356 vmov.f32 s16,0xc0000000
        let value = 3221225472u64;
        c.w(832, 4, value);
    }
    {
        // 0x6002c35a ldr r1,[0x6002c504]
        let value = c.load(0x6002c504, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002c35c vmul.f32 s0,s0,s15
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002c360 ldr r7,[0x6002c508]
        let value = c.load(0x6002c508, 4);
        c.w(60, 4, value);
    }
    {
        // 0x6002c362 vmov.f32 s17,0x3f800000
        let value = 1065353216u64;
        c.w(836, 4, value);
    }
    {
        // 0x6002c366 str.w r1,[r4,#-0x54]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(48, 4)).wrapping_sub(84u64)) & 0xffffffffu64;
        c.store(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c36a sub.w r8,r4,#0x44
        let mut t187d00 = 0u64;
        t187d00 = (68u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(48, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(64, 4, value);
        let value = (signed(c.r(64, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(64, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002c36e ldr r1,[0x6002c50c]
        let value = c.load(0x6002c50c, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002c370 str.w r7,[r9,#-0x38]!
        let mut t19d000 = 0u64;
        t19d000 = ((c.r(68, 4)).wrapping_sub(56u64)) & 0xffffffffu64;
        let value = ((t19d000 >> 0) & 0xffffffffu64);
        c.w(68, 4, value);
        c.store(((t19d000 >> 0) & 0xffffffffu64) as u32, 4, c.r(60, 4));
    }
    {
        // 0x6002c374 sub.w r7,r4,#0x3c
        let mut t187d00 = 0u64;
        t187d00 = (60u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = (((t187d00 >> 0) & 0xffffffffu64) <= c.r(48, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t187d00 >> 0) & 0xffffffffu64), 4, true);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_sub(((t187d00 >> 0) & 0xffffffffu64));
        c.w(60, 4, value);
        let value = (signed(c.r(60, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(60, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002c378 str.w r12,[r3,#-0x4c]!
        let mut t19d000 = 0u64;
        t19d000 = ((c.r(44, 4)).wrapping_sub(76u64)) & 0xffffffffu64;
        let value = ((t19d000 >> 0) & 0xffffffffu64);
        c.w(44, 4, value);
        c.store(((t19d000 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c37c str.w r1,[r6,#-0x50]!
        let mut t19d000 = 0u64;
        t19d000 = ((c.r(56, 4)).wrapping_sub(80u64)) & 0xffffffffu64;
        let value = ((t19d000 >> 0) & 0xffffffffu64);
        c.w(56, 4, value);
        c.store(((t19d000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c380 str r3,[sp,#0xc]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(12u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002c382 bl 0x60040c70
        let value = (1610793862u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002c386
}
pub fn b_6002c386(c: &mut Machine) -> u32 {
    {
        // 0x6002c386 vmov.f32 s13,s0
        let value = c.r(768, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002c38a vldr.32 s15,[r5]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(52, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002c38e vldr.32 s14,[r6]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(56, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002c392 vmul.f32 s15,s15,s16
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(832, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002c396 str r4,[sp,#0x8]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x6002c398 vadd.f32 s13,s13,s17
        let value = (f32::from_bits(c.r(820, 4) as u32) + f32::from_bits(c.r(836, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002c39c ldr r1,[0x6002c510]
        let value = c.load(0x6002c510, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002c39e subs r4,#0x34
        let value = (52u64 <= c.r(48, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 52u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_sub(52u64);
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
        // 0x6002c3a0 vldr.32 s0,[pc,#0x170]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610793888u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(368u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002c3a4 vstr.32 s15,[r9]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(68, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002c3a8 vdiv.f32 s12,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) / f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x6002c3ac str.w r1,[r8,#0x0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(64, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c3b0 vmul.f32 s15,s14,s12
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002c3b4 vstr.32 s12,[r7]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(60, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x6002c3b8 vstr.32 s15,[r4]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(48, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002c3bc bl 0x60040c70
        let value = (1610793920u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002c3c0
}
pub fn b_6002c3c0(c: &mut Machine) -> u32 {
    {
        // 0x6002c3c0 vldr.32 s14,[r5]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(52, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002c3c4 vadd.f32 s0,s0,s17
        let value = (f32::from_bits(c.r(768, 4) as u32) + f32::from_bits(c.r(836, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002c3c8 ldr r3,[sp,#0xc]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(12u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002c3ca vmul.f32 s14,s14,s16
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(832, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002c3ce vldr.32 s11,[pc,#0x15c]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610793934u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(348u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x6002c3d2 vldr.32 s13,[r3]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(44, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002c3d6 vldr.32 s15,[r6]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(56, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002c3da vdiv.f32 s12,s0,s14
        let value = (f32::from_bits(c.r(768, 4) as u32) / f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x6002c3de ldr r3,[sp,#0x8]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        let value = c.load(((t19e900 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002c3e0 ldr r1,[0x6002c51c]
        let value = c.load(0x6002c51c, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002c3e2 str.w r1,[r3,#-0x40]!
        let mut t19d000 = 0u64;
        t19d000 = ((c.r(44, 4)).wrapping_sub(64u64)) & 0xffffffffu64;
        let value = ((t19d000 >> 0) & 0xffffffffu64);
        c.w(44, 4, value);
        c.store(((t19d000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c3e6 str r3,[sp,#0x8]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(8u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002c3e8 vcmpe.f32 s13,s11
        let mut t35c00 = 0u64;
        let mut t35e00 = 0u64;
        let mut t36000 = 0u64;
        let mut t36100 = 0u64;
        let mut t36200 = 0u64;
        let mut t36400 = 0u64;
        let mut t36500 = 0u64;
        let mut t36600 = 0u64;
        let mut t36700 = 0u64;
        let mut t36800 = 0u64;
        let mut t36900 = 0u64;
        let mut t36a00 = 0u64;
        let mut t36b00 = 0u64;
        let mut t36c00 = 0u64;
        let mut t36d00 = 0u64;
        let mut t36e00 = 0u64;
        let mut t36f00 = 0u64;
        let mut t37000 = 0u64;
        t35c00 = ((f32::from_bits(c.r(820, 4) as u32) < f32::from_bits(c.r(812, 4) as u32)) as u64)
            & 0xffu64;
        t35e00 = ((f32::from_bits(c.r(820, 4) as u32) == f32::from_bits(c.r(812, 4) as u32))
            as u64)
            & 0xffu64;
        t36000 = ((f32::from_bits(c.r(812, 4) as u32) <= f32::from_bits(c.r(820, 4) as u32))
            as u64)
            & 0xffu64;
        t36100 = (f32::from_bits(c.r(820, 4) as u32).is_nan() as u64) & 0xffu64;
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
        // 0x6002c3ec vmrs apsr,fpscr
        let mut t6bd00 = 0u64;
        let mut t6bf00 = 0u64;
        let mut t6c000 = 0u64;
        let mut t6c200 = 0u64;
        let mut t6c300 = 0u64;
        let mut t6c500 = 0u64;
        let mut t6c600 = 0u64;
        t6bd00 = (if 31u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t6bd00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(96, 1, value);
        t6bf00 = (if 30u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (30u64 as u32)
        }) & 0xffffffffu64;
        t6c000 = (((t6bf00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c000 >> 0) & 0xffu64) & 1u64);
        c.w(97, 1, value);
        t6c200 = (if 29u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (29u64 as u32)
        }) & 0xffffffffu64;
        t6c300 = (((t6c200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c300 >> 0) & 0xffu64) & 1u64);
        c.w(98, 1, value);
        t6c500 = (if 28u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (28u64 as u32)
        }) & 0xffffffffu64;
        t6c600 = (((t6c500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c600 >> 0) & 0xffu64) & 1u64);
        c.w(99, 1, value);
    }
    {
        // 0x6002c3f0 vmul.f32 s15,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002c3f4 vstr.32 s12,[r7]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(60, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x6002c3f8 vstr.32 s15,[r4]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(48, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002c3fc bgt.w 0x6002db82
        let mut t183c00 = 0u64;
        let mut t183d00 = 0u64;
        let mut t183f00 = 0u64;
        t183c00 = ((c.r(97, 1) == 0) as u64) & 0xffu64;
        t183d00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
        t183f00 = (((((t183c00 >> 0) & 0xffu64) != 0) & (((t183d00 >> 0) & 0xffu64) != 0)) as u64)
            & 0xffu64;
        if ((t183f00 >> 0) & 0xffu64) != 0 {
            return 0x6002db82u32;
        }
    }
    0x6002c400
}
pub fn b_6002c400(c: &mut Machine) -> u32 {
    {
        // 0x6002c400 vmul.f32 s15,s13,s16
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(832, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    0x6002c404
}
pub fn b_6002c404(c: &mut Machine) -> u32 {
    {
        // 0x6002c404 vldr.32 s0,[r8]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(64, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002c408 vmov.f32 s16,0x3f800000
        let value = 1065353216u64;
        c.w(832, 4, value);
    }
    {
        // 0x6002c40c vldr.32 s14,[pc,#0xf0]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610793996u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(240u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002c410 vmov.f32 s17,0xc0000000
        let value = 3221225472u64;
        c.w(836, 4, value);
    }
    {
        // 0x6002c414 vstr.32 s13,[r5]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(52, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x6002c418 vmul.f32 s0,s0,s14
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(768, 4, value);
    }
    {
        // 0x6002c41c vstr.32 s15,[r9]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(68, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002c420 bl 0x60040c70
        let value = (1610794020u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002c424
}
pub fn b_6002c424(c: &mut Machine) -> u32 {
    {
        // 0x6002c424 vmov.f32 s13,s0
        let value = c.r(768, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002c428 vldr.32 s15,[r5]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(52, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002c42c vldr.32 s14,[r6]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(56, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002c430 vadd.f32 s13,s13,s16
        let value = (f32::from_bits(c.r(820, 4) as u32) + f32::from_bits(c.r(832, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002c434 ldr r1,[0x6002c510]
        let value = c.load(0x6002c510, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002c436 vmul.f32 s15,s15,s17
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(836, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002c43a vldr.32 s0,[pc,#0xd8]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610794042u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(216u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002c43e str.w r1,[r8,#0x0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(64, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c442 ldr.w r8,[0x6002c538]
        let value = c.load(0x6002c538, 4);
        c.w(64, 4, value);
    }
    {
        // 0x6002c446 vdiv.f32 s12,s13,s15
        let value = (f32::from_bits(c.r(820, 4) as u32) / f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x6002c44a vstr.32 s15,[r9]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(68, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002c44e vmul.f32 s15,s14,s12
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002c452 vstr.32 s12,[r7]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(60, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x6002c456 vstr.32 s15,[r4]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(48, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002c45a bl 0x60040c70
        let value = (1610794078u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002c45e
}
pub fn b_6002c45e(c: &mut Machine) -> u32 {
    {
        // 0x6002c45e vmov.f32 s13,s0
        let value = c.r(768, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002c462 vldr.32 s14,[r5]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(52, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002c466 vldr.32 s15,[r6]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(56, 4)) & 0xffffffffu64;
        let value = c.load(((t63c00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002c46a mov.w r12,#0x0
        let mut t187d00 = 0u64;
        t187d00 = (0u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = ((t187d00 >> 0) & 0xffffffffu64);
        c.w(80, 4, value);
        let value = (signed(c.r(80, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(80, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002c46e vmul.f32 s14,s14,s17
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(836, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002c472 movs r1,#0x0
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
        // 0x6002c474 vadd.f32 s13,s13,s16
        let value = (f32::from_bits(c.r(820, 4) as u32) + f32::from_bits(c.r(832, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002c478 mov.w r0,#0x800
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (2048u64) & 0xffffffffu64;
        t188700 = (if 31u64 >= 32 {
            0
        } else {
            ((t188500 >> 0) & 0xffffffffu64) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t188700 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(104, 1, value);
        let value = ((t188500 >> 0) & 0xffffffffu64);
        c.w(32, 4, value);
        let value = (signed(c.r(32, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(32, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002c47c ldr.w lr,[0x6002c504]
        let value = c.load(0x6002c504, 4);
        c.w(88, 4, value);
    }
    {
        // 0x6002c480 mvn r6,#0x3
        let mut t187d00 = 0u64;
        t187d00 = (3u64) & 0xffffffffu64;
        let value = c.r(98, 1);
        c.w(104, 1, value);
        let value = !((t187d00 >> 0) & 0xffffffffu64);
        c.w(56, 4, value);
        let value = (signed(c.r(56, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(56, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002c484 ldr r3,[0x6002c50c]
        let value = c.load(0x6002c50c, 4);
        c.w(44, 4, value);
    }
    {
        // 0x6002c486 vmov.f32 s17,0x3f800000
        let value = 1065353216u64;
        c.w(836, 4, value);
    }
    {
        // 0x6002c48a vdiv.f32 s12,s13,s14
        let value = (f32::from_bits(c.r(820, 4) as u32) / f32::from_bits(c.r(824, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x6002c48e str.w r0,[r11,#0xbd0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3024u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c492 str.w r0,[r11,#0xbd4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3028u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c496 str.w r0,[r11,#0xc30]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3120u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c49a str.w r0,[r11,#0xc34]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3124u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c49e str.w r1,[r11,#0xbdc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3036u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c4a2 strb.w r1,[r11,#0xc18]
        let mut t19c800 = 0u64;
        let mut t1faa00 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3096u64)) & 0xffffffffu64;
        t1faa00 = ((c.r(36, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            1,
            ((t1faa00 >> 0) & 0xffu64),
        );
    }
    {
        // 0x6002c4a6 vmov.f32 s13,0xc1400000
        let value = 3242196992u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002c4aa str.w r6,[r11,#0xc24]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3108u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002c4ae vmov.f32 s14,0x3f000000
        let value = 1056964608u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002c4b2 str.w r6,[r11,#0xc84]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3204u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002c4b6 vldr.32 s0,[pc,#0x78]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610794166u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(120u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002c4ba vmul.f32 s15,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002c4be vstr.32 s12,[r7]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(60, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(816, 4));
    }
    {
        // 0x6002c4c2 vstr.32 s15,[r4]
        let mut t63c00 = 0u64;
        t63c00 = (c.r(48, 4)) & 0xffffffffu64;
        c.store(((t63c00 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002c4c6 vmov.f32 s15,0x41100000
        let value = 1091567616u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002c4ca ldr r4,[0x6002c534]
        let value = c.load(0x6002c534, 4);
        c.w(48, 4, value);
    }
    {
        // 0x6002c4cc mov r5,r4
        let value = c.r(48, 4);
        c.w(52, 4, value);
    }
    {
        // 0x6002c4ce mov r2,r4
        let value = c.r(48, 4);
        c.w(40, 4, value);
    }
    {
        // 0x6002c4d0 mov r9,r4
        let value = c.r(48, 4);
        c.w(68, 4, value);
    }
    {
        // 0x6002c4d2 str.w lr,[r4,#0x60]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(96u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(88, 4));
    }
    {
        // 0x6002c4d6 str.w lr,[r5],#0x4c
        let mut t19cb00 = 0u64;
        t19cb00 = (c.r(52, 4)) & 0xffffffffu64;
        let value = (c.r(52, 4)).wrapping_add(76u64);
        c.w(52, 4, value);
        c.store(((t19cb00 >> 0) & 0xffffffffu64) as u32, 4, c.r(88, 4));
    }
    {
        // 0x6002c4da add.w r7,r4,#0x420
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
        c.w(60, 4, value);
        let value = (signed(c.r(60, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(60, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002c4de str.w r3,[r2,#0x64]!
        let mut t19d200 = 0u64;
        t19d200 = ((c.r(40, 4)).wrapping_add(100u64)) & 0xffffffffu64;
        let value = ((t19d200 >> 0) & 0xffffffffu64);
        c.w(40, 4, value);
        c.store(((t19d200 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002c4e2 str.w r3,[r9,#0x4]!
        let mut t19d200 = 0u64;
        t19d200 = ((c.r(68, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = ((t19d200 >> 0) & 0xffffffffu64);
        c.w(68, 4, value);
        c.store(((t19d200 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002c4e6 str.w r12,[r4,#0x28]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c4ea str.w r12,[r4,#0x2c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c4ee str.w r12,[r4,#0x30]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c4f2 str.w r12,[r4,#0x34]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c4f6 str.w r12,[r4,#0x38]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c4fa b 0x6002c53c
        return 0x6002c53cu32;
    }
    0x6002c4fc
}
pub fn b_6002c53c(c: &mut Machine) -> u32 {
    {
        // 0x6002c53c str.w r12,[r4,#0x3c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(60u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c540 str.w r12,[r4,#0x40]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c544 str.w r12,[r4,#0x44]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c548 str.w r12,[r4,#0x48]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(72u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c54c str.w r12,[r4,#0xac]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(172u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c550 str.w r12,[r4,#0x88]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(136u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c554 str.w r12,[r4,#0x8c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(140u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c558 str.w r12,[r4,#0x90]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(144u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c55c str.w r12,[r4,#0x94]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(148u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c560 str.w r8,[r4,#0x58]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(88u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(64, 4));
    }
    {
        // 0x6002c564 str.w r12,[r5,#0x0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c568 addw r5,r4,#0x40c
        let value = ((c.r(48, 4) as u128 + 1036u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 1036u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(1036u64);
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002c56c str r2,[sp,#0x38]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x6002c56e mov r2,r4
        let value = c.r(48, 4);
        c.w(40, 4, value);
    }
    {
        // 0x6002c570 str.w lr,[r4,#0xc0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(192u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(88, 4));
    }
    {
        // 0x6002c574 str.w lr,[r4,#0x120]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(288u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(88, 4));
    }
    {
        // 0x6002c578 str.w r3,[r4,#0x124]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(292u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002c57c str.w r12,[r4,#0x98]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(152u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c580 str.w r12,[r4,#0x9c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(156u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c584 str.w r12,[r4,#0xa0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(160u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c588 str.w r12,[r4,#0xa4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(164u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c58c str.w r12,[r4,#0xa8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(168u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c590 str.w r8,[r4,#0xb8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(184u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(64, 4));
    }
    {
        // 0x6002c594 str.w r12,[r4,#0x10c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(268u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c598 str.w r12,[r4,#0xe8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(232u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c59c str.w r12,[r4,#0xec]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(236u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c5a0 str.w r12,[r4,#0xf0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(240u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c5a4 str.w r12,[r4,#0xf4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(244u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c5a8 str.w r12,[r4,#0xf8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(248u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c5ac str.w r12,[r4,#0xfc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(252u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c5b0 str.w r12,[r4,#0x100]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(256u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c5b4 str.w r12,[r4,#0x104]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(260u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c5b8 str.w r12,[r4,#0x108]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(264u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c5bc str.w r12,[r4,#0x16c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(364u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c5c0 str.w r12,[r4,#0x148]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(328u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c5c4 str.w r8,[r4,#0x118]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(280u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(64, 4));
    }
    {
        // 0x6002c5c8 str.w r1,[r11,#0xc3c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3132u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c5cc strb.w r1,[r11,#0xc78]
        let mut t19c800 = 0u64;
        let mut t1faa00 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3192u64)) & 0xffffffffu64;
        t1faa00 = ((c.r(36, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            1,
            ((t1faa00 >> 0) & 0xffu64),
        );
    }
    {
        // 0x6002c5d0 str.w r0,[r11,#0xc90]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3216u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c5d4 str.w r0,[r11,#0xc94]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3220u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c5d8 str.w r6,[r11,#0xce4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3300u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002c5dc str.w r1,[r11,#0xc9c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3228u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c5e0 strb.w r1,[r11,#0xcd8]
        let mut t19c800 = 0u64;
        let mut t1faa00 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3288u64)) & 0xffffffffu64;
        t1faa00 = ((c.r(36, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            1,
            ((t1faa00 >> 0) & 0xffu64),
        );
    }
    {
        // 0x6002c5e4 str.w r0,[r11,#0xcf0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3312u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c5e8 str.w r0,[r11,#0xcf4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3316u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c5ec str.w r6,[r11,#0xd44]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3396u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002c5f0 str.w r3,[r2,#0xc4]!
        let mut t19d200 = 0u64;
        t19d200 = ((c.r(40, 4)).wrapping_add(196u64)) & 0xffffffffu64;
        let value = ((t19d200 >> 0) & 0xffffffffu64);
        c.w(40, 4, value);
        c.store(((t19d200 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002c5f4 str.w lr,[r4,#0x180]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(384u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(88, 4));
    }
    {
        // 0x6002c5f8 str.w lr,[r4,#0x1e0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(480u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(88, 4));
    }
    {
        // 0x6002c5fc str.w r3,[r4,#0x184]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(388u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002c600 str r2,[sp,#0xc]
        let mut t19e900 = 0u64;
        t19e900 = ((c.r(84, 4)).wrapping_add(12u64)) & 0xffffffffu64;
        c.store(((t19e900 >> 0) & 0xffffffffu64) as u32, 4, c.r(40, 4));
    }
    {
        // 0x6002c602 str.w r12,[r4,#0x14c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(332u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c606 str.w r12,[r4,#0x150]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(336u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c60a str.w r12,[r4,#0x154]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(340u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c60e str.w r12,[r4,#0x158]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(344u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c612 str.w r12,[r4,#0x15c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(348u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c616 str.w r12,[r4,#0x160]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(352u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c61a str.w r12,[r4,#0x164]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(356u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c61e str.w r12,[r4,#0x168]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(360u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c622 str.w r12,[r4,#0x1cc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(460u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c626 str.w r12,[r4,#0x1a8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(424u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c62a str.w r12,[r4,#0x1ac]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(428u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c62e str.w r12,[r4,#0x1b0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(432u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c632 str.w r12,[r4,#0x1b4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(436u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c636 str.w r12,[r4,#0x1b8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(440u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c63a str.w r12,[r4,#0x1bc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(444u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c63e str.w r12,[r4,#0x1c0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(448u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c642 str.w r12,[r4,#0x1c4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(452u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c646 str.w r12,[r4,#0x1c8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(456u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c64a str.w r8,[r4,#0x178]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(376u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(64, 4));
    }
    {
        // 0x6002c64e str.w r1,[r11,#0xcfc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3324u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c652 strb.w r1,[r11,#0xd38]
        let mut t19c800 = 0u64;
        let mut t1faa00 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3384u64)) & 0xffffffffu64;
        t1faa00 = ((c.r(36, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            1,
            ((t1faa00 >> 0) & 0xffu64),
        );
    }
    {
        // 0x6002c656 str.w r0,[r11,#0xd50]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3408u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c65a str.w r0,[r11,#0xd54]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3412u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c65e str.w r6,[r11,#0xda4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3492u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002c662 str.w r1,[r11,#0xd5c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3420u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c666 strb.w r1,[r11,#0xd98]
        let mut t19c800 = 0u64;
        let mut t1faa00 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3480u64)) & 0xffffffffu64;
        t1faa00 = ((c.r(36, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            1,
            ((t1faa00 >> 0) & 0xffu64),
        );
    }
    {
        // 0x6002c66a str.w r0,[r11,#0xdb0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3504u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c66e str.w r0,[r11,#0xdb4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3508u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c672 str.w r8,[r4,#0x1d8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(472u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(64, 4));
    }
    {
        // 0x6002c676 str.w r3,[r4,#0x1e4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(484u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002c67a str.w lr,[r4,#0x240]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(576u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(88, 4));
    }
    {
        // 0x6002c67e str.w lr,[r4,#0x2a0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(672u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(88, 4));
    }
    {
        // 0x6002c682 str.w r12,[r4,#0x22c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(556u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c686 str.w r12,[r4,#0x208]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(520u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c68a str.w r12,[r4,#0x20c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(524u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c68e str.w r12,[r4,#0x210]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(528u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c692 str.w r12,[r4,#0x214]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(532u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c696 str.w r12,[r4,#0x218]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(536u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c69a str.w r12,[r4,#0x21c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(540u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c69e str.w r12,[r4,#0x220]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(544u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c6a2 str.w r12,[r4,#0x224]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(548u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c6a6 str.w r12,[r4,#0x228]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(552u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c6aa str.w r12,[r4,#0x28c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(652u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c6ae str.w r12,[r4,#0x268]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(616u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c6b2 str.w r12,[r4,#0x26c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(620u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c6b6 str.w r12,[r4,#0x270]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(624u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c6ba str.w r12,[r4,#0x274]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(628u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c6be str.w r12,[r4,#0x278]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(632u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c6c2 str.w r12,[r4,#0x27c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(636u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c6c6 str.w r12,[r4,#0x280]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(640u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c6ca str.w r12,[r4,#0x284]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(644u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c6ce str.w r12,[r4,#0x288]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(648u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c6d2 str.w r8,[r4,#0x238]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(568u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(64, 4));
    }
    {
        // 0x6002c6d6 str.w r6,[r11,#0xe04]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3588u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002c6da str.w r1,[r11,#0xdbc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3516u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c6de strb.w r1,[r11,#0xdf8]
        let mut t19c800 = 0u64;
        let mut t1faa00 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3576u64)) & 0xffffffffu64;
        t1faa00 = ((c.r(36, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            1,
            ((t1faa00 >> 0) & 0xffu64),
        );
    }
    {
        // 0x6002c6e2 str.w r0,[r11,#0xe10]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3600u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c6e6 str.w r0,[r11,#0xe14]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3604u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c6ea str.w r6,[r11,#0xe64]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3684u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002c6ee str.w r1,[r11,#0xe1c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3612u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c6f2 strb.w r1,[r11,#0xe58]
        let mut t19c800 = 0u64;
        let mut t1faa00 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3672u64)) & 0xffffffffu64;
        t1faa00 = ((c.r(36, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            1,
            ((t1faa00 >> 0) & 0xffu64),
        );
    }
    {
        // 0x6002c6f6 str.w r3,[r4,#0x244]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(580u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002c6fa str.w r3,[r4,#0x2a4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(676u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002c6fe str.w lr,[r4,#0x300]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(768u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(88, 4));
    }
    {
        // 0x6002c702 str.w r3,[r4,#0x304]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(772u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002c706 str.w r8,[r4,#0x298]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(664u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(64, 4));
    }
    {
        // 0x6002c70a str.w r12,[r4,#0x2ec]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(748u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c70e str.w r12,[r4,#0x2c8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(712u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c712 str.w r12,[r4,#0x2cc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(716u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c716 str.w r12,[r4,#0x2d0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(720u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c71a str.w r12,[r4,#0x2d4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(724u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c71e str.w r12,[r4,#0x2d8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(728u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c722 str.w r12,[r4,#0x2dc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(732u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c726 str.w r12,[r4,#0x2e0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(736u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c72a str.w r12,[r4,#0x2e4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(740u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c72e str.w r12,[r4,#0x2e8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(744u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c732 str.w r12,[r4,#0x34c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(844u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c736 str.w r12,[r4,#0x328]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(808u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c73a str.w r12,[r4,#0x32c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(812u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c73e str.w r12,[r4,#0x330]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(816u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c742 str.w r12,[r4,#0x334]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(820u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c746 str.w r12,[r4,#0x338]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(824u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c74a str.w r12,[r4,#0x33c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(828u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c74e str.w r12,[r4,#0x340]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(832u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c752 str.w r12,[r4,#0x344]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(836u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c756 str.w r12,[r4,#0x348]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(840u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c75a str.w r0,[r11,#0xe70]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3696u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c75e str.w r0,[r11,#0xe74]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3700u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c762 str.w r6,[r11,#0xec4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3780u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002c766 str.w r1,[r11,#0xe7c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3708u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c76a strb.w r1,[r11,#0xeb8]
        let mut t19c800 = 0u64;
        let mut t1faa00 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3768u64)) & 0xffffffffu64;
        t1faa00 = ((c.r(36, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            1,
            ((t1faa00 >> 0) & 0xffu64),
        );
    }
    {
        // 0x6002c76e str.w r0,[r11,#0xed0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3792u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c772 str.w r0,[r11,#0xed4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3796u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c776 str.w r6,[r11,#0xf24]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3876u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002c77a str.w r1,[r11,#0xedc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3804u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c77e str.w lr,[r4,#0x360]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(864u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(88, 4));
    }
    {
        // 0x6002c782 str.w r3,[r4,#0x364]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(868u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002c786 str.w lr,[r4,#0x3c0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(960u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(88, 4));
    }
    {
        // 0x6002c78a str.w r3,[r4,#0x3c4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(964u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002c78e str.w r8,[r4,#0x2f8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(760u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(64, 4));
    }
    {
        // 0x6002c792 str.w r12,[r4,#0x3ac]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(940u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c796 str.w r12,[r4,#0x388]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(904u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c79a str.w r12,[r4,#0x38c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(908u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c79e str.w r12,[r4,#0x390]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(912u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c7a2 str.w r12,[r4,#0x394]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(916u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c7a6 str.w r12,[r4,#0x398]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(920u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c7aa str.w r12,[r4,#0x39c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(924u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c7ae str.w r12,[r4,#0x3a0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(928u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c7b2 str.w r12,[r4,#0x3a4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(932u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c7b6 str.w r12,[r4,#0x3a8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(936u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c7ba str.w r12,[r4,#0x3e8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(1000u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c7be str.w r12,[r4,#0x3ec]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(1004u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c7c2 str.w r12,[r4,#0x3f0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(1008u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c7c6 str.w r12,[r4,#0x3f4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(1012u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c7ca str.w r12,[r4,#0x3f8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(1016u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c7ce str.w r12,[r4,#0x3fc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(1020u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c7d2 str.w r8,[r4,#0x358]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(856u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(64, 4));
    }
    {
        // 0x6002c7d6 str.w r8,[r4,#0x3b8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(48, 4)).wrapping_add(952u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(64, 4));
    }
    {
        // 0x6002c7da strb.w r1,[r11,#0xf18]
        let mut t19c800 = 0u64;
        let mut t1faa00 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3864u64)) & 0xffffffffu64;
        t1faa00 = ((c.r(36, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            1,
            ((t1faa00 >> 0) & 0xffu64),
        );
    }
    {
        // 0x6002c7de str.w r0,[r11,#0xf30]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3888u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c7e2 str.w r0,[r11,#0xf34]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3892u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c7e6 str.w r6,[r11,#0xf84]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3972u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002c7ea str.w r1,[r11,#0xf3c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3900u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c7ee strb.w r1,[r11,#0xf78]
        let mut t19c800 = 0u64;
        let mut t1faa00 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3960u64)) & 0xffffffffu64;
        t1faa00 = ((c.r(36, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            1,
            ((t1faa00 >> 0) & 0xffu64),
        );
    }
    {
        // 0x6002c7f2 str.w r0,[r11,#0xf90]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3984u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c7f6 str.w r0,[r11,#0xf94]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3988u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c7fa str.w r6,[r11,#0xfe4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(4068u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002c7fe str.w r12,[r5,#0x0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c802 add.w r5,r4,#0x400
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
        let value = ((c.r(48, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002c806 str.w lr,[r10,#0x48]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(72u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(88, 4));
    }
    {
        // 0x6002c80a str.w r12,[r5,#0x0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c80e addw r5,r4,#0x404
        let value = ((c.r(48, 4) as u128 + 1028u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 1028u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(1028u64);
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002c812 str.w r3,[r10,#0x4c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(76u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002c816 str.w r12,[r5,#0x0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c81a add.w r5,r4,#0x408
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
        let value = ((c.r(48, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002c81e str.w r1,[r11,#0xf9c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3996u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c822 str.w r12,[r5,#0x0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c826 add.w r5,r4,#0x418
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
        let value = ((c.r(48, 4) as u128 + ((t188500 >> 0) & 0xffffffffu64) as u128)
            > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), ((t188500 >> 0) & 0xffffffffu64), 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(((t188500 >> 0) & 0xffffffffu64));
        c.w(52, 4, value);
        let value = (signed(c.r(52, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(52, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002c82a strb.w r1,[r11,#0xfd8]
        let mut t19c800 = 0u64;
        let mut t1faa00 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(4056u64)) & 0xffffffffu64;
        t1faa00 = ((c.r(36, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            1,
            ((t1faa00 >> 0) & 0xffu64),
        );
    }
    {
        // 0x6002c82e str.w r0,[r11,#0xff0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(4080u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c832 str.w r0,[r11,#0xff4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(4084u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c836 str.w r6,[r10,#0x44]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(68u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002c83a str.w r12,[r10,#0x34]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(52u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c83e str.w r12,[r10,#0x10]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(16u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c842 str.w r12,[r10,#0x14]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(20u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c846 str.w r12,[r10,#0x18]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(24u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c84a str.w r12,[r10,#0x1c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(28u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c84e str.w r12,[r10,#0x20]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(32u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c852 str.w r12,[r10,#0x24]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(36u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c856 str.w r12,[r10,#0x28]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(40u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c85a str.w r12,[r10,#0x2c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(44u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c85e str.w r12,[r10,#0x30]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(48u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c862 str.w r1,[r11,#0xffc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(4092u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c866 strb.w r1,[r10,#0x38]
        let mut t19c800 = 0u64;
        let mut t1faa00 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        t1faa00 = ((c.r(36, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            1,
            ((t1faa00 >> 0) & 0xffu64),
        );
    }
    {
        // 0x6002c86a str.w r8,[r10,#0x40]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(64u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(64, 4));
    }
    {
        // 0x6002c86e str.w r6,[r10,#0xa4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(164u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002c872 str.w r12,[r10,#0x94]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(148u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c876 str.w r12,[r10,#0x70]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(112u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c87a str.w r12,[r10,#0x74]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(116u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c87e str.w r12,[r10,#0x78]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(120u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c882 str.w r12,[r10,#0x7c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(124u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c886 str.w r8,[r5,#0x0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(52, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(64, 4));
    }
    {
        // 0x6002c88a strd r0,r0,[r10,#0x50]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(72, 4)).wrapping_add(80u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c88e str.w lr,[r7,#0x0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(60, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(88, 4));
    }
    {
        // 0x6002c892 addw r7,r4,#0x424
        let value = ((c.r(48, 4) as u128 + 1060u64 as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), 1060u64, 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(1060u64);
        c.w(60, 4, value);
        let value = (signed(c.r(60, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(60, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002c896 str.w lr,[r10,#0xa8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(168u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(88, 4));
    }
    {
        // 0x6002c89a str r3,[r7,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(60, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002c89c add.w r7,r4,#0x2f8
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (760u64) & 0xffffffffu64;
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
        c.w(60, 4, value);
        let value = (signed(c.r(60, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(60, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002c8a0 str.w r3,[r10,#0xac]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(172u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002c8a4 str.w lr,[r10,#0x108]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(264u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(88, 4));
    }
    {
        // 0x6002c8a8 str.w r3,[r10,#0x10c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(268u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002c8ac str.w r12,[r10,#0x80]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(128u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c8b0 str.w r12,[r10,#0x84]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(132u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c8b4 str.w r12,[r10,#0x88]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(136u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c8b8 str.w r12,[r10,#0x8c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(140u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c8bc str.w r12,[r10,#0x90]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(144u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c8c0 str.w r1,[r10,#0x5c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(92u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c8c4 strb.w r1,[r10,#0x98]
        let mut t19c800 = 0u64;
        let mut t1faa00 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(152u64)) & 0xffffffffu64;
        t1faa00 = ((c.r(36, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            1,
            ((t1faa00 >> 0) & 0xffu64),
        );
    }
    {
        // 0x6002c8c8 str.w r8,[r10,#0xa0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(160u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(64, 4));
    }
    {
        // 0x6002c8cc str.w r6,[r10,#0x104]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(260u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002c8d0 str.w r12,[r10,#0xf4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(244u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c8d4 str.w r12,[r10,#0xd0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(208u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c8d8 str.w r12,[r10,#0xd4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(212u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c8dc str.w r12,[r10,#0xd8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(216u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c8e0 str.w r12,[r10,#0xdc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(220u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c8e4 str.w r12,[r10,#0xe0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(224u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c8e8 str.w r12,[r10,#0xe4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(228u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c8ec str.w r12,[r10,#0xe8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(232u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c8f0 str.w r12,[r10,#0xec]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(236u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c8f4 str.w r12,[r10,#0xf0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(240u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c8f8 str.w r1,[r10,#0xbc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(188u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c8fc strb.w r1,[r10,#0xf8]
        let mut t19c800 = 0u64;
        let mut t1faa00 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(248u64)) & 0xffffffffu64;
        t1faa00 = ((c.r(36, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            1,
            ((t1faa00 >> 0) & 0xffu64),
        );
    }
    {
        // 0x6002c900 str.w r8,[r10,#0x100]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(256u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(64, 4));
    }
    {
        // 0x6002c904 str.w r6,[r10,#0x164]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(356u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002c908 strd r0,r0,[r10,#0xb0]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(72, 4)).wrapping_add(176u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c90c strd r0,r0,[r10,#0x110]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(72, 4)).wrapping_add(272u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c910 str.w r12,[r10,#0x154]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(340u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c914 str.w r3,[r10,#0x16c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(364u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x6002c918 vstr.32 s13,[r10,#0x2c4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(708u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(820, 4));
    }
    {
        // 0x6002c91c vstr.32 s14,[r10,#0x2c8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(712u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002c920 str.w lr,[r10,#0x168]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(360u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(88, 4));
    }
    {
        // 0x6002c924 vstr.32 s15,[r10,#0x2d4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(724u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002c928 str.w r12,[r10,#0x130]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(304u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c92c str.w r12,[r10,#0x134]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(308u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c930 str.w r6,[r10,#0x1c4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(452u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(56, 4));
    }
    {
        // 0x6002c934 add.w r6,r4,#0x358
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (856u64) & 0xffffffffu64;
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
        c.w(56, 4, value);
        let value = (signed(c.r(56, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(56, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002c938 str.w r12,[r10,#0x138]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(312u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c93c str.w r12,[r10,#0x13c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(316u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c940 str.w r12,[r10,#0x140]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(320u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c944 str.w r12,[r10,#0x144]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(324u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c948 str.w r12,[r10,#0x148]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(328u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c94c str.w r12,[r10,#0x14c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(332u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c950 str.w r12,[r10,#0x150]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(336u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c954 str.w r1,[r10,#0x11c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(284u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c958 strb.w r1,[r10,#0x158]
        let mut t19c800 = 0u64;
        let mut t1faa00 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(344u64)) & 0xffffffffu64;
        t1faa00 = ((c.r(36, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            1,
            ((t1faa00 >> 0) & 0xffu64),
        );
    }
    {
        // 0x6002c95c str.w r8,[r10,#0x160]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(352u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(64, 4));
    }
    {
        // 0x6002c960 str.w r8,[r10,#0x1c0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(448u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(64, 4));
    }
    {
        // 0x6002c964 add.w r8,r4,#0x298
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (664u64) & 0xffffffffu64;
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
        c.w(64, 4, value);
        let value = (signed(c.r(64, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(64, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    {
        // 0x6002c968 str.w r12,[r10,#0x1b4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(436u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c96c add.w r4,r4,#0x3b8
        let mut t188500 = 0u64;
        let mut t188700 = 0u64;
        t188500 = (952u64) & 0xffffffffu64;
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
        // 0x6002c970 str.w r12,[r10,#0x190]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(400u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c974 str.w r12,[r10,#0x194]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(404u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c978 str.w r12,[r10,#0x198]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(408u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c97c str.w r12,[r10,#0x19c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(412u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c980 str.w r12,[r10,#0x1a0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(416u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c984 str.w r12,[r10,#0x1a4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(420u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c988 str.w r12,[r10,#0x1a8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(424u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c98c str.w r12,[r10,#0x1ac]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(428u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c990 str.w r12,[r10,#0x1b0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(432u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c994 str.w r1,[r10,#0x17c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(380u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c998 strd r0,r0,[r10,#0x170]
        let mut t19dc00 = 0u64;
        let mut t1fb400 = 0u64;
        t19dc00 = ((c.r(72, 4)).wrapping_add(368u64)) & 0xffffffffu64;
        t1fb400 = ((t19dc00 >> 0) & 0xffffffffu64) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
        t1fb400 = (((t1fb400 >> 0) & 0xffffffffu64).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t1fb400 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c99c strb.w r1,[r10,#0x1b8]
        let mut t19c800 = 0u64;
        let mut t1faa00 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(440u64)) & 0xffffffffu64;
        t1faa00 = ((c.r(36, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            1,
            ((t1faa00 >> 0) & 0xffu64),
        );
    }
    {
        // 0x6002c9a0 ldr r1,[0x6002cd3c]
        let value = c.load(0x6002cd3c, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002c9a2 ldr r0,[0x6002cd40]
        let value = c.load(0x6002cd40, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002c9a4 str.w r1,[r10,#0x2d0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(720u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c9a8 str.w r1,[r10,#0x2fc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(764u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c9ac ldr r1,[0x6002cd44]
        let value = c.load(0x6002cd44, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002c9ae str.w lr,[r10,#0x2c0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(704u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(88, 4));
    }
    {
        // 0x6002c9b2 str.w r1,[r10,#0x2cc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(716u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c9b6 ldr r1,[0x6002cd48]
        let value = c.load(0x6002cd48, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002c9b8 str.w r0,[r10,#0x2d8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(728u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002c9bc str.w lr,[r10,#0x2e4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(740u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(88, 4));
    }
    {
        // 0x6002c9c0 str.w lr,[r10,#0x308]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(776u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(88, 4));
    }
    {
        // 0x6002c9c4 str.w r1,[r10,#0x300]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(768u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002c9c8 str.w r12,[r10,#0x2dc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(732u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c9cc str.w r12,[r10,#0x2e0]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(736u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c9d0 str.w r12,[r10,#0x304]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(772u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c9d4 str.w r12,[r10,#0x310]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(784u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(80, 4));
    }
    {
        // 0x6002c9d8 bl 0x60040c70
        let value = (1610795484u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002c9dc
}
pub fn b_6002c9dc(c: &mut Machine) -> u32 {
    {
        // 0x6002c9dc vmov.f32 s15,s0
        let value = c.r(768, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002c9e0 vldr.32 s0,[pc,#0x368]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610795488u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(872u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002c9e4 vstr.32 s15,[r10,#0x2e8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(744u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(828, 4));
    }
    {
        // 0x6002c9e8 bl 0x60040c70
        let value = (1610795500u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002c9ec
}
pub fn b_6002c9ec(c: &mut Machine) -> u32 {
    {
        // 0x6002c9ec vldr.32 s13,[r10,#0x2e4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(740u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002c9f0 vmov.f32 s12,0x41a00000
        let value = 1101004800u64;
        c.w(816, 4, value);
    }
    {
        // 0x6002c9f4 vldr.32 s10,[pc,#0x358]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610795508u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(856u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x6002c9f8 movs r1,#0x1
        let value = 1u64;
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
        // 0x6002c9fa vldr.32 s14,[r10,#0x308]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(776u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002c9fe vdiv.f32 s15,s10,s13
        let value = (f32::from_bits(c.r(808, 4) as u32) / f32::from_bits(c.r(820, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002ca02 vldr.32 s13,[pc,#0x350]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610795522u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(848u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002ca06 vldr.32 s11,[pc,#0x350]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610795526u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(848u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x6002ca0a ldr r0,[0x6002cd5c]
        let value = c.load(0x6002cd5c, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002ca0c vstr.32 s0,[r10,#0x2ec]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(748u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002ca10 strb.w r1,[r11,#0xc18]
        let mut t19c800 = 0u64;
        let mut t1faa00 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3096u64)) & 0xffffffffu64;
        t1faa00 = ((c.r(36, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            1,
            ((t1faa00 >> 0) & 0xffu64),
        );
    }
    {
        // 0x6002ca14 vmul.f32 s14,s14,s11
        let value = (f32::from_bits(c.r(824, 4) as u32) * f32::from_bits(c.r(812, 4) as u32))
            .to_bits() as u64;
        c.w(824, 4, value);
    }
    {
        // 0x6002ca18 vldr.32 s0,[pc,#0x344]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610795544u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(836u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(768, 4, value);
    }
    {
        // 0x6002ca1c strb.w r1,[r11,#0xc78]
        let mut t19c800 = 0u64;
        let mut t1faa00 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3192u64)) & 0xffffffffu64;
        t1faa00 = ((c.r(36, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            1,
            ((t1faa00 >> 0) & 0xffu64),
        );
    }
    {
        // 0x6002ca20 strb.w r1,[r11,#0xcd8]
        let mut t19c800 = 0u64;
        let mut t1faa00 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3288u64)) & 0xffffffffu64;
        t1faa00 = ((c.r(36, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            1,
            ((t1faa00 >> 0) & 0xffu64),
        );
    }
    {
        // 0x6002ca24 strb.w r1,[r11,#0xd38]
        let mut t19c800 = 0u64;
        let mut t1faa00 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3384u64)) & 0xffffffffu64;
        t1faa00 = ((c.r(36, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            1,
            ((t1faa00 >> 0) & 0xffu64),
        );
    }
    {
        // 0x6002ca28 strb.w r1,[r11,#0xd98]
        let mut t19c800 = 0u64;
        let mut t1faa00 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3480u64)) & 0xffffffffu64;
        t1faa00 = ((c.r(36, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            1,
            ((t1faa00 >> 0) & 0xffu64),
        );
    }
    {
        // 0x6002ca2c strb.w r1,[r11,#0xdf8]
        let mut t19c800 = 0u64;
        let mut t1faa00 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3576u64)) & 0xffffffffu64;
        t1faa00 = ((c.r(36, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            1,
            ((t1faa00 >> 0) & 0xffu64),
        );
    }
    {
        // 0x6002ca30 strb.w r1,[r11,#0xe58]
        let mut t19c800 = 0u64;
        let mut t1faa00 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3672u64)) & 0xffffffffu64;
        t1faa00 = ((c.r(36, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            1,
            ((t1faa00 >> 0) & 0xffu64),
        );
    }
    {
        // 0x6002ca34 strb.w r1,[r11,#0xeb8]
        let mut t19c800 = 0u64;
        let mut t1faa00 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3768u64)) & 0xffffffffu64;
        t1faa00 = ((c.r(36, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            1,
            ((t1faa00 >> 0) & 0xffu64),
        );
    }
    {
        // 0x6002ca38 strb.w r1,[r11,#0xf18]
        let mut t19c800 = 0u64;
        let mut t1faa00 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3864u64)) & 0xffffffffu64;
        t1faa00 = ((c.r(36, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            1,
            ((t1faa00 >> 0) & 0xffu64),
        );
    }
    {
        // 0x6002ca3c strb.w r1,[r11,#0xf78]
        let mut t19c800 = 0u64;
        let mut t1faa00 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(3960u64)) & 0xffffffffu64;
        t1faa00 = ((c.r(36, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            1,
            ((t1faa00 >> 0) & 0xffu64),
        );
    }
    {
        // 0x6002ca40 strb.w r1,[r11,#0xfd8]
        let mut t19c800 = 0u64;
        let mut t1faa00 = 0u64;
        t19c800 = ((c.r(76, 4)).wrapping_add(4056u64)) & 0xffffffffu64;
        t1faa00 = ((c.r(36, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            1,
            ((t1faa00 >> 0) & 0xffu64),
        );
    }
    {
        // 0x6002ca44 vmul.f32 s15,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002ca48 strb.w r1,[r10,#0x38]
        let mut t19c800 = 0u64;
        let mut t1faa00 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(56u64)) & 0xffffffffu64;
        t1faa00 = ((c.r(36, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            1,
            ((t1faa00 >> 0) & 0xffu64),
        );
    }
    {
        // 0x6002ca4c strb.w r1,[r10,#0x98]
        let mut t19c800 = 0u64;
        let mut t1faa00 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(152u64)) & 0xffffffffu64;
        t1faa00 = ((c.r(36, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            1,
            ((t1faa00 >> 0) & 0xffu64),
        );
    }
    {
        // 0x6002ca50 strb.w r1,[r10,#0xf8]
        let mut t19c800 = 0u64;
        let mut t1faa00 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(248u64)) & 0xffffffffu64;
        t1faa00 = ((c.r(36, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            1,
            ((t1faa00 >> 0) & 0xffu64),
        );
    }
    {
        // 0x6002ca54 vcmpe.f32 s15,s13
        let mut t35c00 = 0u64;
        let mut t35e00 = 0u64;
        let mut t36000 = 0u64;
        let mut t36100 = 0u64;
        let mut t36200 = 0u64;
        let mut t36400 = 0u64;
        let mut t36500 = 0u64;
        let mut t36600 = 0u64;
        let mut t36700 = 0u64;
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
        // 0x6002ca58 strb.w r1,[r10,#0x158]
        let mut t19c800 = 0u64;
        let mut t1faa00 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(344u64)) & 0xffffffffu64;
        t1faa00 = ((c.r(36, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            1,
            ((t1faa00 >> 0) & 0xffu64),
        );
    }
    {
        // 0x6002ca5c strb.w r1,[r10,#0x1b8]
        let mut t19c800 = 0u64;
        let mut t1faa00 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(440u64)) & 0xffffffffu64;
        t1faa00 = ((c.r(36, 4)).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        c.store(
            ((t19c800 >> 0) & 0xffffffffu64) as u32,
            1,
            ((t1faa00 >> 0) & 0xffu64),
        );
    }
    {
        // 0x6002ca60 str.w r0,[r10,#0x1d4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(468u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002ca64 vmrs apsr,fpscr
        let mut t6bd00 = 0u64;
        let mut t6bf00 = 0u64;
        let mut t6c000 = 0u64;
        let mut t6c200 = 0u64;
        let mut t6c300 = 0u64;
        let mut t6c500 = 0u64;
        let mut t6c600 = 0u64;
        t6bd00 = (if 31u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t6bd00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(96, 1, value);
        t6bf00 = (if 30u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (30u64 as u32)
        }) & 0xffffffffu64;
        t6c000 = (((t6bf00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c000 >> 0) & 0xffu64) & 1u64);
        c.w(97, 1, value);
        t6c200 = (if 29u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (29u64 as u32)
        }) & 0xffffffffu64;
        t6c300 = (((t6c200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c300 >> 0) & 0xffu64) & 1u64);
        c.w(98, 1, value);
        t6c500 = (if 28u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (28u64 as u32)
        }) & 0xffffffffu64;
        t6c600 = (((t6c500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c600 >> 0) & 0xffu64) & 1u64);
        c.w(99, 1, value);
    }
    {
        // 0x6002ca68 ldr r1,[0x6002cd64]
        let value = c.load(0x6002cd64, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002ca6a str.w r0,[r10,#0x1dc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(476u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002ca6e ldr r0,[0x6002cd3c]
        let value = c.load(0x6002cd3c, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002ca70 ite mi
        let mut t184c00 = 0u64;
        t184c00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x6002ca72 vsub.mi.f32 s16,s16,s15
        let mut td00 = 0u64;
        let mut t2600 = 0u64;
        td00 = ((c.r(96, 1) != 0u64) as u64) & 0xffu64;
        t2600 = ((((td00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2600 >> 0) & 0xffu64) != 0 {
            return 0x6002ca76u32;
        }
        let value = (f32::from_bits(c.r(832, 4) as u32) - f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(832, 4, value);
    }
    0x6002ca76
}
pub fn b_6002ca76(c: &mut Machine) -> u32 {
    {
        // 0x6002ca76 vldr.pl.32 s16,[pc,#0x2f0]
        let mut tf00 = 0u64;
        let mut t2700 = 0u64;
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610795638u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(752u64)) & 0xffffffffu64;
        tf00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        t2700 = ((((tf00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x6002ca7au32;
        }
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(832, 4, value);
    }
    0x6002ca7a
}
pub fn b_6002ca7a(c: &mut Machine) -> u32 {
    {
        // 0x6002ca7a vstr.32 s14,[r10,#0x30c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(780u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(824, 4));
    }
    {
        // 0x6002ca7e vstr.32 s0,[r10,#0x2f4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(756u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002ca82 vstr.32 s0,[r10,#0x2f8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(760u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002ca86 vstr.32 s0,[r10,#0x274]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(628u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002ca8a vstr.32 s0,[r10,#0x25c]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(604u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002ca8e vstr.32 s0,[r10,#0x260]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(608u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002ca92 vstr.32 s0,[r10,#0x268]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(616u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002ca96 vstr.32 s0,[r10,#0x1d0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(464u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002ca9a str.w r1,[r10,#0x1c8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(456u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002ca9e vstr.32 s16,[r10,#0x2f0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(752u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(832, 4));
    }
    {
        // 0x6002caa2 vstr.32 s17,[r10,#0x1d8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(472u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(836, 4));
    }
    {
        // 0x6002caa6 str.w r0,[r10,#0x26c]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(620u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002caaa ldr r0,[0x6002cd6c]
        let value = c.load(0x6002cd6c, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002caac str.w r1,[r10,#0x1f4]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(500u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002cab0 str.w r0,[r10,#0x270]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(624u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002cab4 ldr r1,[0x6002cd70]
        let value = c.load(0x6002cd70, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002cab6 ldr r0,[0x6002cd74]
        let value = c.load(0x6002cd74, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002cab8 str.w r1,[r10,#0x1e8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(488u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002cabc str.w r0,[r10,#0x258]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(600u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002cac0 ldr r1,[0x6002cd78]
        let value = c.load(0x6002cd78, 4);
        c.w(36, 4, value);
    }
    {
        // 0x6002cac2 ldr r0,[0x6002cd7c]
        let value = c.load(0x6002cd7c, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6002cac4 vstr.32 s0,[r10,#0x1fc]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(508u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002cac8 str.w r0,[r10,#0x1cc]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(460u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x6002cacc vstr.32 s17,[r10,#0x1ec]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(492u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(836, 4));
    }
    {
        // 0x6002cad0 vstr.32 s0,[r10,#0x1f0]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(496u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002cad4 str.w r1,[r10,#0x1f8]
        let mut t19c800 = 0u64;
        t19c800 = ((c.r(72, 4)).wrapping_add(504u64)) & 0xffffffffu64;
        c.store(((t19c800 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x6002cad8 bl 0x60040c70
        let value = (1610795740u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60040c70u32;
    }
    0x6002cadc
}
pub fn b_6002cadc(c: &mut Machine) -> u32 {
    {
        // 0x6002cadc vldr.32 s15,[r10,#0x1e8]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(488u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(828, 4, value);
    }
    {
        // 0x6002cae0 vmov.f32 s14,s0
        let value = c.r(768, 4);
        c.w(824, 4, value);
    }
    {
        // 0x6002cae4 vldr.32 s12,[r10,#0x1f4]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(500u64)) & 0xffffffffu64;
        let value = c.load(((t64000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(816, 4, value);
    }
    {
        // 0x6002cae8 vmul.f32 s15,s0,s15
        let value = (f32::from_bits(c.r(768, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002caec vldr.32 s13,[pc,#0x290]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610795756u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(656u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(820, 4, value);
    }
    {
        // 0x6002caf0 vstr.32 s0,[r10,#0x1ec]
        let mut t64000 = 0u64;
        t64000 = ((c.r(72, 4)).wrapping_add(492u64)) & 0xffffffffu64;
        c.store(((t64000 >> 0) & 0xffffffffu64) as u32, 4, c.r(768, 4));
    }
    {
        // 0x6002caf4 vmul.f32 s15,s15,s12
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002caf8 vcmpe.f32 s15,s13
        let mut t35c00 = 0u64;
        let mut t35e00 = 0u64;
        let mut t36000 = 0u64;
        let mut t36100 = 0u64;
        let mut t36200 = 0u64;
        let mut t36400 = 0u64;
        let mut t36500 = 0u64;
        let mut t36600 = 0u64;
        let mut t36700 = 0u64;
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
        // 0x6002cafc vmrs apsr,fpscr
        let mut t6bd00 = 0u64;
        let mut t6bf00 = 0u64;
        let mut t6c000 = 0u64;
        let mut t6c200 = 0u64;
        let mut t6c300 = 0u64;
        let mut t6c500 = 0u64;
        let mut t6c600 = 0u64;
        t6bd00 = (if 31u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (31u64 as u32)
        }) & 0xffffffffu64;
        let value = ((t6bd00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32);
        c.w(96, 1, value);
        t6bf00 = (if 30u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (30u64 as u32)
        }) & 0xffffffffu64;
        t6c000 = (((t6bf00 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c000 >> 0) & 0xffu64) & 1u64);
        c.w(97, 1, value);
        t6c200 = (if 29u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (29u64 as u32)
        }) & 0xffffffffu64;
        t6c300 = (((t6c200 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c300 >> 0) & 0xffu64) & 1u64);
        c.w(98, 1, value);
        t6c500 = (if 28u64 >= 32 {
            0
        } else {
            (c.r(180, 4)) >> (28u64 as u32)
        }) & 0xffffffffu64;
        t6c600 = (((t6c500 >> 0) & 0xffffffffu64).wrapping_shr((0u64 * 8) as u32)) & 0xffu64;
        let value = (((t6c600 >> 0) & 0xffu64) & 1u64);
        c.w(99, 1, value);
    }
    {
        // 0x6002cb00 bpl.w 0x6002db7a
        let mut t182d00 = 0u64;
        t182d00 = ((c.r(96, 1) == 0u64) as u64) & 0xffu64;
        if ((t182d00 >> 0) & 0xffu64) != 0 {
            return 0x6002db7au32;
        }
    }
    0x6002cb04
}
pub fn b_6002cb04(c: &mut Machine) -> u32 {
    {
        // 0x6002cb04 vmul.f32 s12,s15,s15
        let value = (f32::from_bits(c.r(828, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(816, 4, value);
    }
    {
        // 0x6002cb08 vldr.32 s10,[pc,#0x278]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610795784u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(632u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(808, 4, value);
    }
    {
        // 0x6002cb0c vldr.32 s11,[pc,#0x278]
        let mut t64b00 = 0u64;
        let mut t64c00 = 0u64;
        let mut t64e00 = 0u64;
        t64b00 = ((1610795788u64).wrapping_add(4u64)) & 0xffffffffu64;
        t64c00 = (((t64b00 >> 0) & 0xffffffffu64) & 4294967292u64) & 0xffffffffu64;
        t64e00 = (((t64c00 >> 0) & 0xffffffffu64).wrapping_add(632u64)) & 0xffffffffu64;
        let value = c.load(((t64e00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(812, 4, value);
    }
    {
        // 0x6002cb10 vmul.f32 s13,s12,s15
        let value = (f32::from_bits(c.r(816, 4) as u32) * f32::from_bits(c.r(828, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002cb14 vfma.f32 s15,s13,s10
        let value = f32::from_bits(c.r(820, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(808, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002cb18 vmul.f32 s13,s13,s12
        let value = (f32::from_bits(c.r(820, 4) as u32) * f32::from_bits(c.r(816, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    {
        // 0x6002cb1c vfma.f32 s15,s13,s11
        let value = f32::from_bits(c.r(820, 4) as u32)
            .fast_mul_add(
                f32::from_bits(c.r(812, 4) as u32),
                f32::from_bits(c.r(828, 4) as u32),
            )
            .to_bits() as u64;
        c.w(828, 4, value);
    }
    {
        // 0x6002cb20 vadd.f32 s17,s15,s17
        let value = (f32::from_bits(c.r(828, 4) as u32) + f32::from_bits(c.r(836, 4) as u32))
            .to_bits() as u64;
        c.w(836, 4, value);
    }
    {
        // 0x6002cb24 vdiv.f32 s13,s15,s17
        let value = (f32::from_bits(c.r(828, 4) as u32) / f32::from_bits(c.r(836, 4) as u32))
            .to_bits() as u64;
        c.w(820, 4, value);
    }
    0x6002cb28
}
