#![allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut,
    unused_comparisons
)]
use crate::machine::{signed, signed_overflow, Machine};
pub fn b_60041698(c: &mut Machine) -> u32 {
    {
        // 0x60041698 ldr r0,[0x600416a0]
        let value = c.load(0x600416a0, 4);
        c.w(32, 4, value);
    }
    {
        // 0x6004169a b.w 0x60041a1c
        return 0x60041a1cu32;
        return 0xfffffff0u32;
    }
    0x6004169e
}
pub fn b_600419a4(c: &mut Machine) -> u32 {
    {
        // 0x600419a4 push {r3,r4,r5,lr}
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
        // 0x600419a6 ldr r5,[0x600419c0]
        let value = c.load(0x600419c0, 4);
        c.w(52, 4, value);
    }
    {
        // 0x600419a8 movs r3,#0x0
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
        // 0x600419aa mov r4,r0
        let value = c.r(32, 4);
        c.w(48, 4, value);
    }
    {
        // 0x600419ac mov r0,r1
        let value = c.r(36, 4);
        c.w(32, 4, value);
    }
    {
        // 0x600419ae str r3,[r5,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x600419b0 bl 0x60020c98
        let value = (1610881460u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x60020c98u32;
    }
    0x600419b4
}
pub fn b_600419b4(c: &mut Machine) -> u32 {
    {
        // 0x600419b4 adds r3,r0,#0x1
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
        // 0x600419b6 bne 0x600419be
        let mut t180400 = 0u64;
        t180400 = ((c.r(97, 1) == 0u64) as u64) & 0xffu64;
        if ((t180400 >> 0) & 0xffu64) != 0 {
            return 0x600419beu32;
        }
    }
    0x600419b8
}
pub fn b_600419b8(c: &mut Machine) -> u32 {
    {
        // 0x600419b8 ldr r3,[r5,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x600419ba cbz r3,0x600419be
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(44, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x600419beu32;
        }
    }
    0x600419bc
}
pub fn b_600419bc(c: &mut Machine) -> u32 {
    {
        // 0x600419bc str r3,[r4,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    0x600419be
}
pub fn b_600419be(c: &mut Machine) -> u32 {
    {
        // 0x600419be pop {r3,r4,r5,pc}
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
    0x600419c0
}
pub fn b_600419c4(c: &mut Machine) -> u32 {
    {
        // 0x600419c4 ldr r3,[0x600419cc]
        let value = c.load(0x600419cc, 4);
        c.w(44, 4, value);
    }
    {
        // 0x600419c6 ldr r0,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x600419c8 bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x600419ca
}
pub fn b_60041a1a(c: &mut Machine) -> u32 {
    {
        // 0x60041a1a bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x60041a1c
}
pub fn b_60041a1c(c: &mut Machine) -> u32 {
    {
        // 0x60041a1c bx lr
        let mut t0 = 0u64;
        t0 = (c.r(88, 4) & 1u64) & 0xffffffffu64;
        let value = (((t0 >> 0) & 0xffffffffu64) != 0u64) as u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        let value = (c.r(88, 4) & 4294967294u64);
        c.w(92, 4, value);
        return (c.r(92, 4) as u32 & !1);
    }
    0x60041a1e
}
pub fn b_60041a5c(c: &mut Machine) -> u32 {
    {
        // 0x60041a5c push {r3,r4,r5,lr}
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
        // 0x60041a5e mov r5,r0
        let value = c.r(32, 4);
        c.w(52, 4, value);
    }
    {
        // 0x60041a60 cmp r1,#0x0
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
        // 0x60041a62 beq 0x60041ae8
        let mut t180200 = 0u64;
        t180200 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        if ((t180200 >> 0) & 0xffu64) != 0 {
            return 0x60041ae8u32;
        }
    }
    0x60041a64
}
pub fn b_60041a64(c: &mut Machine) -> u32 {
    {
        // 0x60041a64 ldr.w r3,[r1,#-0x4]
        let mut t19ce00 = 0u64;
        t19ce00 = ((c.r(36, 4)).wrapping_sub(4u64)) & 0xffffffffu64;
        let value = c.load(((t19ce00 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x60041a68 subs r4,r1,#0x4
        let value = (4u64 <= c.r(36, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(36, 4), 4u64, 4, true);
        c.w(103, 1, value);
        let value = (c.r(36, 4)).wrapping_sub(4u64);
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
        // 0x60041a6a cmp r3,#0x0
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
        // 0x60041a6c it lt
        let mut t185c00 = 0u64;
        t185c00 = ((c.r(96, 1) == c.r(99, 1)) as u64) & 0xffu64;
    }
    {
        // 0x60041a6e add.lt r4,r4,r3
        let mut t1d00 = 0u64;
        let mut t2700 = 0u64;
        t1d00 = ((c.r(96, 1) != c.r(99, 1)) as u64) & 0xffu64;
        t2700 = ((((t1d00 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x60041a70u32;
        }
        let value = ((c.r(48, 4) as u128 + c.r(44, 4) as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), c.r(44, 4), 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(c.r(44, 4));
        c.w(48, 4, value);
        let value = (signed(c.r(48, 4), 4) < signed(0u64, 4)) as u64;
        c.w(100, 1, value);
        let value = (c.r(48, 4) == 0u64) as u64;
        c.w(101, 1, value);
    }
    0x60041a70
}
pub fn b_60041a70(c: &mut Machine) -> u32 {
    {
        // 0x60041a70 bl 0x6004168c
        let value = (1610881652u64 | 1u64);
        c.w(88, 4, value);
        let value = 1u64;
        c.w(105, 1, value);
        c.userop(62, c.r(105, 1), 0u64, 0u64, 1, 0);
        return 0x6004168cu32;
    }
    0x60041a74
}
pub fn b_60041a74(c: &mut Machine) -> u32 {
    {
        // 0x60041a74 ldr r2,[0x60041aec]
        let value = c.load(0x60041aec, 4);
        c.w(40, 4, value);
    }
    {
        // 0x60041a76 ldr r3,[r2,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(40, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x60041a78 cbnz r3,0x60041a88
        let mut t1a5900 = 0u64;
        t1a5900 = ((c.r(44, 4) != 0u64) as u64) & 0xffu64;
        if ((t1a5900 >> 0) & 0xffu64) != 0 {
            return 0x60041a88u32;
        }
    }
    0x60041a7a
}
pub fn b_60041a7a(c: &mut Machine) -> u32 {
    {
        // 0x60041a7a str r3,[r4,#0x4]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x60041a7c str r4,[r2,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(40, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    0x60041a7e
}
pub fn b_60041a7e(c: &mut Machine) -> u32 {
    {
        // 0x60041a7e mov r0,r5
        let value = c.r(52, 4);
        c.w(32, 4, value);
    }
    {
        // 0x60041a80 pop.w {r3,r4,r5,lr}
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
        c.w(88, 4, value);
        let value = (c.r(128, 4)).wrapping_add(4u64);
        c.w(128, 4, value);
        let value = c.r(128, 4);
        c.w(84, 4, value);
    }
    {
        // 0x60041a84 b.w 0x60041698
        return 0x60041698u32;
        return 0xfffffff0u32;
    }
    0x60041a88
}
pub fn b_60041a88(c: &mut Machine) -> u32 {
    {
        // 0x60041a88 cmp r3,r4
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
        // 0x60041a8a bls 0x60041a9e
        let mut t181400 = 0u64;
        let mut t181600 = 0u64;
        t181400 = ((c.r(98, 1) == 0) as u64) & 0xffu64;
        t181600 = (((((t181400 >> 0) & 0xffu64) != 0) | (c.r(97, 1) != 0)) as u64) & 0xffu64;
        if ((t181600 >> 0) & 0xffu64) != 0 {
            return 0x60041a9eu32;
        }
    }
    0x60041a8c
}
pub fn b_60041a8c(c: &mut Machine) -> u32 {
    {
        // 0x60041a8c ldr r0,[r4,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x60041a8e adds r1,r4,r0
        let value = ((c.r(48, 4) as u128 + c.r(32, 4) as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), c.r(32, 4), 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(c.r(32, 4));
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
        // 0x60041a90 cmp r3,r1
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
        // 0x60041a92 itttt eq
        let mut t184400 = 0u64;
        t184400 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x60041a94 ldr.eq r1,[r3,#0x0]
        let mut t500 = 0u64;
        let mut t2600 = 0u64;
        let mut t19e000 = 0u64;
        t500 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        t2600 = ((((t500 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2600 >> 0) & 0xffu64) != 0 {
            return 0x60041a96u32;
        }
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x60041a96
}
pub fn b_60041a96(c: &mut Machine) -> u32 {
    {
        // 0x60041a96 ldr.eq r3,[r3,#0x4]
        let mut t500 = 0u64;
        let mut t2600 = 0u64;
        let mut t19e000 = 0u64;
        t500 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        t2600 = ((((t500 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2600 >> 0) & 0xffu64) != 0 {
            return 0x60041a98u32;
        }
        t19e000 = ((c.r(44, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    0x60041a98
}
pub fn b_60041a98(c: &mut Machine) -> u32 {
    {
        // 0x60041a98 add.eq r1,r1,r0
        let mut t500 = 0u64;
        let mut t2600 = 0u64;
        t500 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        t2600 = ((((t500 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2600 >> 0) & 0xffu64) != 0 {
            return 0x60041a9au32;
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
    0x60041a9a
}
pub fn b_60041a9a(c: &mut Machine) -> u32 {
    {
        // 0x60041a9a str.eq r1,[r4,#0x0]
        let mut t500 = 0u64;
        let mut t2700 = 0u64;
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        t500 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((t500 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x60041a9cu32;
        }
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    0x60041a9c
}
pub fn b_60041a9c(c: &mut Machine) -> u32 {
    {
        // 0x60041a9c b 0x60041a7a
        return 0x60041a7au32;
    }
    0x60041a9e
}
pub fn b_60041a9e(c: &mut Machine) -> u32 {
    {
        // 0x60041a9e mov r2,r3
        let value = c.r(44, 4);
        c.w(40, 4, value);
    }
    {
        // 0x60041aa0 ldr r3,[r3,#0x4]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x60041aa2 cbz r3,0x60041aa8
        let mut t1a5b00 = 0u64;
        t1a5b00 = ((c.r(44, 4) == 0u64) as u64) & 0xffu64;
        if ((t1a5b00 >> 0) & 0xffu64) != 0 {
            return 0x60041aa8u32;
        }
    }
    0x60041aa4
}
pub fn b_60041aa4(c: &mut Machine) -> u32 {
    {
        // 0x60041aa4 cmp r3,r4
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
        // 0x60041aa6 bls 0x60041a9e
        let mut t181400 = 0u64;
        let mut t181600 = 0u64;
        t181400 = ((c.r(98, 1) == 0) as u64) & 0xffu64;
        t181600 = (((((t181400 >> 0) & 0xffu64) != 0) | (c.r(97, 1) != 0)) as u64) & 0xffu64;
        if ((t181600 >> 0) & 0xffu64) != 0 {
            return 0x60041a9eu32;
        }
    }
    0x60041aa8
}
pub fn b_60041aa8(c: &mut Machine) -> u32 {
    {
        // 0x60041aa8 ldr r1,[r2,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(40, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    {
        // 0x60041aaa adds r0,r2,r1
        let value = ((c.r(40, 4) as u128 + c.r(36, 4) as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(40, 4), c.r(36, 4), 4, false);
        c.w(103, 1, value);
        let value = (c.r(40, 4)).wrapping_add(c.r(36, 4));
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
        // 0x60041aac cmp r0,r4
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
        // 0x60041aae bne 0x60041ac8
        let mut t180400 = 0u64;
        t180400 = ((c.r(97, 1) == 0u64) as u64) & 0xffu64;
        if ((t180400 >> 0) & 0xffu64) != 0 {
            return 0x60041ac8u32;
        }
    }
    0x60041ab0
}
pub fn b_60041ab0(c: &mut Machine) -> u32 {
    {
        // 0x60041ab0 ldr r0,[r4,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x60041ab2 add r1,r0
        let value = (c.r(36, 4)).wrapping_add(c.r(32, 4));
        c.w(36, 4, value);
    }
    {
        // 0x60041ab4 adds r0,r2,r1
        let value = ((c.r(40, 4) as u128 + c.r(36, 4) as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(40, 4), c.r(36, 4), 4, false);
        c.w(103, 1, value);
        let value = (c.r(40, 4)).wrapping_add(c.r(36, 4));
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
        // 0x60041ab6 cmp r3,r0
        let mut t1a6e00 = 0u64;
        let value = (c.r(32, 4) <= c.r(44, 4)) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(44, 4), c.r(32, 4), 4, true);
        c.w(103, 1, value);
        t1a6e00 = ((c.r(44, 4)).wrapping_sub(c.r(32, 4))) & 0xffffffffu64;
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
        // 0x60041ab8 str r1,[r2,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(40, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    {
        // 0x60041aba bne 0x60041a7e
        let mut t180400 = 0u64;
        t180400 = ((c.r(97, 1) == 0u64) as u64) & 0xffu64;
        if ((t180400 >> 0) & 0xffu64) != 0 {
            return 0x60041a7eu32;
        }
    }
    0x60041abc
}
pub fn b_60041abc(c: &mut Machine) -> u32 {
    {
        // 0x60041abc ldr r0,[r3,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x60041abe ldr r3,[r3,#0x4]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(44, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    {
        // 0x60041ac0 str r3,[r2,#0x4]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(40, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x60041ac2 add r0,r1
        let value = (c.r(32, 4)).wrapping_add(c.r(36, 4));
        c.w(32, 4, value);
    }
    {
        // 0x60041ac4 str r0,[r2,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(40, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(32, 4));
    }
    {
        // 0x60041ac6 b 0x60041a7e
        return 0x60041a7eu32;
    }
    0x60041ac8
}
pub fn b_60041ac8(c: &mut Machine) -> u32 {
    {
        // 0x60041ac8 bls 0x60041ad0
        let mut t181400 = 0u64;
        let mut t181600 = 0u64;
        t181400 = ((c.r(98, 1) == 0) as u64) & 0xffu64;
        t181600 = (((((t181400 >> 0) & 0xffu64) != 0) | (c.r(97, 1) != 0)) as u64) & 0xffu64;
        if ((t181600 >> 0) & 0xffu64) != 0 {
            return 0x60041ad0u32;
        }
    }
    0x60041aca
}
pub fn b_60041aca(c: &mut Machine) -> u32 {
    {
        // 0x60041aca movs r3,#0xc
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
        // 0x60041acc str r3,[r5,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(52, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x60041ace b 0x60041a7e
        return 0x60041a7eu32;
    }
    0x60041ad0
}
pub fn b_60041ad0(c: &mut Machine) -> u32 {
    {
        // 0x60041ad0 ldr r0,[r4,#0x0]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(32, 4, value);
    }
    {
        // 0x60041ad2 adds r1,r4,r0
        let value = ((c.r(48, 4) as u128 + c.r(32, 4) as u128) > 0xffffffffu64 as u128) as u64;
        c.w(102, 1, value);
        let value = signed_overflow(c.r(48, 4), c.r(32, 4), 4, false);
        c.w(103, 1, value);
        let value = (c.r(48, 4)).wrapping_add(c.r(32, 4));
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
        // 0x60041ad4 cmp r3,r1
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
        // 0x60041ad6 itt eq
        let mut t184400 = 0u64;
        t184400 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x60041ad8 ldr.eq r1,[r3,#0x0]
        let mut t500 = 0u64;
        let mut t2600 = 0u64;
        let mut t19e000 = 0u64;
        t500 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        t2600 = ((((t500 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2600 >> 0) & 0xffu64) != 0 {
            return 0x60041adau32;
        }
        t19e000 = ((c.r(44, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(36, 4, value);
    }
    0x60041ada
}
pub fn b_60041ada(c: &mut Machine) -> u32 {
    {
        // 0x60041ada ldr.eq r3,[r3,#0x4]
        let mut t500 = 0u64;
        let mut t2700 = 0u64;
        let mut t19e000 = 0u64;
        t500 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((t500 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x60041adcu32;
        }
        t19e000 = ((c.r(44, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        let value = c.load(((t19e000 >> 0) & 0xffffffffu64) as u32, 4);
        c.w(44, 4, value);
    }
    0x60041adc
}
pub fn b_60041adc(c: &mut Machine) -> u32 {
    {
        // 0x60041adc str r3,[r4,#0x4]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(44, 4));
    }
    {
        // 0x60041ade itt eq
        let mut t184400 = 0u64;
        t184400 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
    }
    {
        // 0x60041ae0 add.eq r1,r1,r0
        let mut t500 = 0u64;
        let mut t2600 = 0u64;
        t500 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        t2600 = ((((t500 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2600 >> 0) & 0xffu64) != 0 {
            return 0x60041ae2u32;
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
    0x60041ae2
}
pub fn b_60041ae2(c: &mut Machine) -> u32 {
    {
        // 0x60041ae2 str.eq r1,[r4,#0x0]
        let mut t500 = 0u64;
        let mut t2700 = 0u64;
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(48, 4)).wrapping_add(0u64)) & 0xffffffffu64;
        t500 = ((c.r(97, 1) != 0u64) as u64) & 0xffu64;
        t2700 = ((((t500 >> 0) & 0xffu64) == 0) as u64) & 0xffu64;
        if ((t2700 >> 0) & 0xffu64) != 0 {
            return 0x60041ae4u32;
        }
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(36, 4));
    }
    0x60041ae4
}
pub fn b_60041ae4(c: &mut Machine) -> u32 {
    {
        // 0x60041ae4 str r4,[r2,#0x4]
        let mut t19e000 = 0u64;
        t19e000 = ((c.r(40, 4)).wrapping_add(4u64)) & 0xffffffffu64;
        c.store(((t19e000 >> 0) & 0xffffffffu64) as u32, 4, c.r(48, 4));
    }
    {
        // 0x60041ae6 b 0x60041a7e
        return 0x60041a7eu32;
    }
    0x60041ae8
}
pub fn b_60041ae8(c: &mut Machine) -> u32 {
    {
        // 0x60041ae8 pop {r3,r4,r5,pc}
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
    0x60041aea
}
