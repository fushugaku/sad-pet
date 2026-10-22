//! Fuse the firmware's hot code without changing its instructions: the two
//! delay-line routines and the ITCM blocks (the per-sample DSP). Each
//! translated block keeps its statements; what changes is how execution moves
//! between blocks and where the emulated registers live:
//!
//! * A block with a single static predecessor is pasted into it, so a chain
//!   of blocks becomes one function. Every other fused ITCM block is a root
//!   with a function of its own, and forward branches to it are direct calls;
//!   a loop's back edge returns to the dispatcher, so no loop becomes
//!   recursion. Each delay-line routine is a single function whose roots are
//!   the arms of a local block loop.
//! * Inside each function the emulated registers are locals (see `promote`),
//!   written back wherever the function exits.
//!
//! All memory accesses, faults and dynamic jumps stay exactly as translated.
use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::PathBuf,
};

/// ITCM holds 13440 bytes of translated code; blocks start at even addresses.
const ITCM_END: u32 = 0x3480;
const ITCM_SLOTS: u32 = ITCM_END / 2;
/// The delay-line routines' `abs` helper (`bic` of the sign bit, `bx lr`).
const ABS: u32 = 0x6004137c;

/// How a fused block continues at a static successor.
enum Link {
    /// Paste the successor's body in place of the branch.
    Paste,
    /// Call the successor's root function.
    Call(String),
    /// Continue at another root of the same routine function.
    Goto(u32),
    /// Run `abs` in place; its return must reach this continuation.
    Abs(u32),
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    // Every translated block: address -> body, and its module.
    let mut blocks = BTreeMap::new();
    let mut modules = BTreeMap::new();
    for entry in fs::read_dir("src/audio_generated").unwrap() {
        let name = entry.unwrap().file_name().into_string().unwrap();
        let Some(module) = name
            .strip_suffix(".rs")
            .filter(|module| module.starts_with("blocks_"))
        else {
            continue;
        };
        let path = format!("src/audio_generated/{name}");
        println!("cargo:rerun-if-changed={path}");
        // Windows checkouts may convert the generated sources to CRLF.
        let source = fs::read_to_string(path).unwrap().replace("\r\n", "\n");
        for function in source.split("pub fn b_").skip(1) {
            let (address, _) = function.split_once('(').unwrap();
            let address = u32::from_str_radix(address, 16).unwrap();
            let body = function.split_once('{').unwrap().1;
            let body = body.split_once("\n}").unwrap().0;
            blocks.insert(address, body.to_owned());
            modules.insert(address, module.to_owned());
        }
    }
    // The two delay-line routines: entry -> last block.
    let routines = [(0x60029898, 0x60029b54), (0x60029dcc, 0x6002a014)];
    let routine = |pc: u32| {
        routines
            .iter()
            .position(|(entry, end)| (*entry..=*end).contains(&pc))
    };
    let delay = |pc| routine(pc).is_some();
    let itcm = |pc| pc < ITCM_END && blocks.contains_key(&pc);
    assert_eq!(
        blocks.keys().filter(|pc| delay(**pc)).count(),
        40,
        "The pinned firmware blocks changed; review the fusion boundaries"
    );
    assert_eq!(
        blocks.keys().filter(|pc| itcm(**pc)).count(),
        406,
        "The pinned ITCM translation changed; review the fusion"
    );

    // Static predecessors of each block, from every block's reachable
    // constant branches and fallthrough.
    let mut predecessors = BTreeMap::<u32, BTreeSet<u32>>::new();
    for (&address, body) in &blocks {
        for pc in successors(body) {
            predecessors.entry(pc).or_default().insert(address);
        }
    }
    // The delay routines return from `abs` to the instruction after the call.
    let abs_continuation = |address| match address {
        0x60029a3a => 0x60029aa2,
        0x60029ef6 => 0x60029f5a,
        _ => panic!("Unexpected abs caller: {address:x}"),
    };
    for caller in [0x60029a3a, 0x60029ef6] {
        predecessors
            .entry(abs_continuation(caller))
            .or_default()
            .insert(caller);
    }
    // A pasted block's only static predecessor is a fused block of the same
    // code that reaches it forward; every other fused block is a root.
    let pasted = |pc: u32| {
        let Some(sources) = predecessors.get(&pc).filter(|s| s.len() == 1) else {
            return false;
        };
        let source = *sources.first().unwrap();
        if delay(pc) {
            delay(source) && !matches!(pc, 0x60029898 | 0x60029dcc)
        } else {
            itcm(pc) && itcm(source) && source < pc
        }
    };
    let link = |source: u32, target: u32| -> Option<Link> {
        let (fused, prefix) = if delay(source) {
            if target == ABS {
                return Some(Link::Abs(abs_continuation(source)));
            }
            if delay(target) && blocks.contains_key(&target) && !pasted(target) {
                assert_eq!(
                    routine(source),
                    routine(target),
                    "a branch between routines"
                );
                return Some(Link::Goto(target));
            }
            (delay(target) && blocks.contains_key(&target), "b")
        } else {
            (itcm(target) && target > source, "i")
        };
        fused.then(|| {
            if pasted(target) {
                Link::Paste
            } else {
                Link::Call(format!("{prefix}_{target:x}"))
            }
        })
    };

    let mut output =
        String::from("use crate::machine::{FastFma, Machine, signed, signed_overflow};\n");
    // Each delay-line routine is one function: its roots are the arms of a
    // local block loop, so its registers stay in locals across the joins.
    for (index, (entry, _)) in routines.iter().enumerate() {
        let arms: String = blocks
            .keys()
            .filter(|pc| routine(**pc) == Some(index) && !pasted(**pc))
            .map(|&root| format!("0x{root:x} => {{\n{}}}\n", paste(root, &blocks, &link)))
            .collect();
        let body = format!(
            "let mut block = 0x{entry:x}u32;\nloop {{\nmatch block {{\n{arms}_ => unreachable!(),\n}}\n}}\n"
        );
        output.push_str(&format!(
            "#[inline(never)]\npub fn b_{entry:x}(c: &mut Machine) -> u32 {{\n{}}}\n",
            promote(&body)
        ));
    }
    for &address in blocks.keys().filter(|pc| itcm(**pc) && !pasted(**pc)) {
        output.push_str(&format!(
            "#[inline(never)]\nfn i_{address:x}(c: &mut Machine) -> u32 {{\n{}}}\n",
            promote(&paste(address, &blocks, &link))
        ));
    }
    // Direct table for the ITCM blocks: one index per block address instead
    // of a thousand-arm match for every executed block. A pasted block runs
    // inside its predecessor; its translated original serves any dynamic jump.
    output.push_str(&format!(
        "pub static ITCM: [Option<fn(&mut Machine) -> u32>; {ITCM_SLOTS}] = [\n"
    ));
    for slot in 0..ITCM_SLOTS {
        let pc = slot * 2;
        if !itcm(pc) {
            output.push_str("None,\n");
        } else if pasted(pc) {
            output.push_str(&format!(
                "Some(crate::generated::{}::b_{pc:x}),\n",
                modules[&pc]
            ));
        } else {
            output.push_str(&format!("Some(i_{pc:x}),\n"));
        }
    }
    output.push_str("];\n");
    fs::write(
        PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("fused.rs"),
        output,
    )
    .unwrap();
}

/// A constant branch target (`return 0x..u32;`) or fallthrough (`0x..`).
fn target(line: &str) -> Option<u32> {
    let line = line.trim().strip_prefix("return ").unwrap_or(line.trim());
    u32::from_str_radix(
        line.trim_end_matches(';')
            .trim_end_matches("u32")
            .strip_prefix("0x")?,
        16,
    )
    .ok()
}

/// A `return` at the top level of an instruction: an unconditional branch
/// or return. (Conditional branches sit inside an `if`.)
fn unconditional(depth: i32, line: &str) -> bool {
    depth == 1 && line.trim().starts_with("return ")
}

fn depth_after(depth: i32, line: &str) -> i32 {
    depth + line.chars().filter(|c| *c == '{').count() as i32
        - line.chars().filter(|c| *c == '}').count() as i32
}

/// The static successors of a block. The translator emits a fallthrough even
/// after an unconditional branch or return; it is unreachable and not one.
fn successors(body: &str) -> BTreeSet<u32> {
    let mut successors = BTreeSet::new();
    let mut depth = 0;
    for line in body.lines() {
        successors.extend(target(line));
        if unconditional(depth, line) {
            break;
        }
        depth = depth_after(depth, line);
    }
    successors
}

/// The body of `address` with its successors linked: pasted bodies in place
/// of their branches, calls of root functions, and every other branch or
/// fallthrough an exit. All exits become `return` statements, so `promote`
/// can write the registers back before each of them.
fn paste(
    address: u32,
    blocks: &BTreeMap<u32, String>,
    link: &impl Fn(u32, u32) -> Option<Link>,
) -> String {
    let lines: Vec<&str> = blocks[&address].lines().collect();
    let last = lines
        .iter()
        .rposition(|line| !line.trim().is_empty())
        .unwrap();
    let mut out = String::new();
    let mut depth = 0;
    let mut reachable = true;
    for (index, line) in lines.into_iter().enumerate() {
        let linked = target(line)
            .filter(|_| reachable)
            .and_then(|pc| Some((pc, link(address, pc)?)));
        match linked {
            Some((pc, linked)) => out.push_str(&jump(pc, linked, address, blocks, link)),
            // The fallthrough (the tail expression) leaves the fused code.
            None if index == last && line.trim().starts_with("0x") => {
                out.push_str(&format!("return {};\n", line.trim()))
            }
            None => {
                out.push_str(line);
                out.push('\n');
            }
        }
        if unconditional(depth, line) {
            reachable = false;
        }
        depth = depth_after(depth, line);
    }
    out
}

/// The statements that continue at `pc` from `source`, after the fault check
/// that every dispatched block boundary makes.
fn jump(
    pc: u32,
    linked: Link,
    source: u32,
    blocks: &BTreeMap<u32, String>,
    link: &impl Fn(u32, u32) -> Option<Link>,
) -> String {
    let guard = format!("if c.fault.get().is_some() {{ return 0x{pc:x}; }}\n");
    match linked {
        Link::Paste => format!("{guard}return {{\n{}}};\n", paste(pc, blocks, link)),
        Link::Call(function) => format!("{guard}return {function}(c);\n"),
        Link::Goto(root) => format!("{guard}block = 0x{root:x};\ncontinue;\n"),
        Link::Abs(continuation) => {
            // The original abs instructions, including their register and
            // flag effects, run in place; their `bx lr` yields the next pc.
            let abs = blocks[&ABS].replace("return ", "break 'abs ");
            let next = link(source, continuation).expect("abs continuation is fused");
            format!(
                "{guard}let pc: u32 = 'abs: {{{abs}\n}};\n\
                 if c.fault.get().is_some() || pc != 0x{continuation:x} {{ return pc; }}\n{}",
                jump(continuation, next, source, blocks, link)
            )
        }
    }
}

/// Keep the emulated registers of one fused function in local variables.
/// Every translated register access is `c.r(OFFSET, SIZE)` or
/// `c.w(OFFSET, SIZE, value);` with constant operands: 4-byte core and single
/// registers, 8-byte double registers (two singles) and 1-byte flags. Memory
/// accesses and the three user operations never touch the register bytes, so
/// the function reads each register once on entry and writes back every
/// register it assigns before each `return` that leaves it (pasted bodies stay
/// inside). LLVM cannot prove that firmware stores leave the register bytes
/// alone; locals need no proof, so the registers stay in host registers.
fn promote(body: &str) -> String {
    // Offset -> width (4 or 1) of each local, and the ones assigned.
    let mut locals = BTreeMap::<usize, usize>::new();
    let mut written = BTreeSet::new();
    let mut local = |offset: usize, width: usize| {
        let previous = locals.insert(offset, width);
        assert!(
            previous.is_none_or(|w| w == width),
            "register {offset} accessed as {width} bytes"
        );
        assert!(width == 1 || offset % 4 == 0, "unaligned register {offset}");
        format!("r{offset}")
    };
    let mut code = String::new();
    for line in body.lines() {
        if let Some(operands) = line.trim().strip_prefix("c.w(") {
            let operands = operands
                .strip_suffix(", value);")
                .expect("register write of another form");
            let (offset, size) = operands.split_once(", ").unwrap();
            let (offset, size): (usize, usize) = (offset.parse().unwrap(), size.parse().unwrap());
            match size {
                1 => code.push_str(&format!("{} = value & 0xff;\n", local(offset, 1))),
                4 => code.push_str(&format!("{} = value & 0xffff_ffff;\n", local(offset, 4))),
                8 => code.push_str(&format!(
                    "{} = value & 0xffff_ffff;\n{} = value >> 32;\n",
                    local(offset, 4),
                    local(offset + 4, 4)
                )),
                _ => panic!("register write of {size} bytes"),
            }
            written.insert(offset);
            if size == 8 {
                written.insert(offset + 4);
            }
            continue;
        }
        if let Some(index) = line.find("c.userop(") {
            let id = &line[index + 9..];
            let id = &id[..id.find(',').unwrap()];
            assert!(
                matches!(id, "62" | "219" | "223"),
                "user operation {id} may touch the registers"
            );
        }
        let mut rest = line;
        while let Some(index) = rest.find("c.r(") {
            code.push_str(&rest[..index]);
            let close = index + rest[index..].find(')').unwrap();
            let (offset, size) = rest[index + 4..close].split_once(", ").unwrap();
            let (offset, size): (usize, usize) = (offset.parse().unwrap(), size.parse().unwrap());
            code.push_str(&match size {
                1 | 4 => local(offset, size),
                8 => format!("({} | {} << 32)", local(offset, 4), local(offset + 4, 4)),
                _ => panic!("register read of {size} bytes"),
            });
            rest = &rest[close + 1..];
        }
        code.push_str(rest);
        code.push('\n');
    }
    // Flag bytes and register words never share bytes.
    for (&offset, &width) in &locals {
        if width == 1 {
            assert!(locals.get(&(offset & !3)).is_none_or(|w| *w == 1));
        }
    }
    let flush: String = written
        .iter()
        .map(|offset| format!("c.w({offset}, {}, r{offset});\n", locals[offset]))
        .collect();
    let mut out: String = locals
        .iter()
        .map(|(offset, width)| format!("let mut r{offset} = c.r({offset}, {width});\n"))
        .collect();
    for line in code.lines() {
        match line.find("return ") {
            Some(index) if !line[index + 7..].starts_with('{') => {
                let end = index + line[index..].find(';').unwrap();
                out.push_str(&format!(
                    "{}{{\n{flush}{}; }}{}\n",
                    &line[..index],
                    &line[index..end],
                    &line[end + 1..]
                ));
            }
            _ => {
                out.push_str(line);
                out.push('\n');
            }
        }
    }
    out
}
