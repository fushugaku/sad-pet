//! Fuse the ITCM firmware (DSP_Process_Block and the control tick) without
//! changing its instructions. Each translated basic block is unchanged; what
//! changes is how execution moves between them:
//!
//! * Every ITCM block gets a slot in a direct jump table indexed by its address,
//!   replacing the two-level `match` dispatch for the hottest code.
//! * A block whose only static predecessor branches forward to it is pasted
//!   into that predecessor, so a chain of blocks is one function. A forward
//!   branch to any other block is a direct call of its wrapper. Backward
//!   edges return to the dispatcher, so no loop becomes recursion.
//! * Inside each function the emulated registers are locals (`promote`),
//!   written back wherever the function exits.
//!
//! Every block still adds its own tick count, and all memory accesses, faults
//! and dynamic jumps stay exactly as translated.
//!
//! ITCM blocks return the next program counter as a plain `u32`. Their
//! memory accesses use the fault-cell adapters (`load_itcm`/`store_itcm`): an
//! unmapped access parks the fault in the machine and yields zero instead of
//! returning early, and the dispatcher checks the cell after each block. The
//! translated `Result` form kept 32-byte `Result<u32, Fault>` returns crossing
//! every indirect call through wasm32 linear memory, and every `?` was an
//! early exit before which each emulated register had to be written back.
//! Without them LLVM keeps the registers of a whole fused chain in locals.
use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::PathBuf,
};

const ITCM_END: u32 = 0x3500;
/// Flash pages fused like ITCM: the pitch shifter and control code the audio
/// block calls on every firmware block. No host call lives in these pages
/// (`Machine::host_call` replaces 0x600413e0, 0x600413f0, 0x60030af4,
/// 0x60041868 and 0x6002047c), so their blocks never need the dispatcher.
const FUSED_PAGES: [u32; 3] = [0x60029, 0x6002a, 0x60040];
const HOST_CALLS: [u32; 5] = [0x600413e0, 0x600413f0, 0x60030af4, 0x60041868, 0x6002047c];

/// Turn a translated ITCM body from `Result<u32>` form into plain `u32` form:
/// `m.load(..)?` and `m.store(..)?` become the fault-cell adapters, and every
/// `Ok(..)` return loses its wrapper. The memory accesses are the only `?`
/// sites in ITCM, which this checks while matching their parentheses.
fn defuse(body: &str, tail_calls: bool) -> String {
    let mut out = String::with_capacity(body.len());
    let mut rest = body;
    loop {
        let next = ["m.load(", "m.store("]
            .iter()
            .filter_map(|pattern| rest.find(pattern).map(|index| (index, *pattern)))
            .min();
        let Some((index, pattern)) = next else {
            out.push_str(rest);
            break;
        };
        out.push_str(&rest[..index]);
        let open = index + pattern.len() - 1;
        let mut depth = 0;
        let mut close = None;
        for (offset, ch) in rest[open..].char_indices() {
            match ch {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        close = Some(open + offset);
                        break;
                    }
                }
                _ => {}
            }
        }
        let close = close.expect("unbalanced memory access");
        assert_eq!(
            rest.as_bytes().get(close + 1),
            Some(&b'?'),
            "memory access without `?`: {}",
            &rest[index..=close]
        );
        out.push_str(if pattern == "m.load(" {
            "m.load_itcm("
        } else {
            "m.store_itcm("
        });
        out.push_str(&rest[open + 1..=close]);
        rest = &rest[close + 2..];
    }
    assert!(!out.contains('?'), "a `?` survived in an ITCM block");
    let mut unwrapped = String::with_capacity(out.len());
    for line in out.lines() {
        // Branches: `return Ok(X);`, also inside `if cond { return Ok(X); }`.
        let line = &line.replace("return Ok(", "return (");
        // With wasm tail calls, a remaining `return (X)` (a backward constant
        // or a register) and the `Ok(X)` fallthrough continue inside ITCM as
        // `m.jump(X)`: a call in tail position, so the stack does not grow,
        // guarded by the fault cell and the tick budget.
        let line = &if tail_calls && !line.contains("(m)") {
            line.replace("return (", "return m.jump(")
        } else {
            line.clone()
        };
        let trimmed = line.trim_start();
        // The fallthrough: a bare `Ok(X)` tail expression.
        if let Some(tail) = trimmed.strip_prefix("Ok(") {
            unwrapped.push_str(&line[..line.len() - trimmed.len()]);
            unwrapped.push_str(if tail_calls && !tail.contains("(m)") {
                "m.jump("
            } else {
                "("
            });
            unwrapped.push_str(tail);
        } else {
            unwrapped.push_str(line);
        }
        unwrapped.push('\n');
    }
    assert!(
        !unwrapped.contains("Ok("),
        "an `Ok(` survived in an ITCM block"
    );
    unwrapped
}

/// Static successor: a `return Ok((0x..u32) & !1);` branch or the tail
/// `Ok(0x..)` fallthrough. Dynamic returns through a register stay dynamic.
fn constant(text: &str) -> Option<u32> {
    let text = text.trim();
    let inner = text
        .strip_prefix("return Ok(")
        .or_else(|| text.strip_prefix("Ok("))?;
    let inner = inner.trim_end_matches(';').strip_suffix(')')?;
    let hex = inner
        .strip_prefix("(0x")
        .and_then(|rest| rest.strip_suffix("u32) & !1"))
        .or_else(|| inner.strip_prefix("0x"))?;
    u32::from_str_radix(hex, 16).ok().map(|pc| pc & !1)
}

/// Replace a line's branch (`return Ok(X);`, also inside `if cond { .. }`)
/// or its tail `Ok(X)` with `with` (`return with;` or the tail `with`).
fn retarget(line: &str, with: &str) -> String {
    if let Some(start) = line.find("return Ok(") {
        let end = start + line[start..].find(';').unwrap() + 1;
        format!("{}return {with};{}", &line[..start], &line[end..])
    } else {
        let trimmed = line.trim();
        debug_assert!(trimmed.starts_with("Ok("));
        line.replacen(trimmed, with, 1)
    }
}

struct Fusion<'a> {
    blocks: &'a BTreeMap<u32, String>,
    pasted: &'a BTreeSet<u32>,
    tail_calls: bool,
}

impl Fusion<'_> {
    /// The plain-`u32` body of a block with its forward branches linked: a
    /// pasted successor's body (with `chain`) or a direct call of the
    /// successor's wrapper. A direct call has no loop iteration, table lookup
    /// or indirect call in between, and every forward path is finite, so the
    /// call depth is bounded by the block count. Every exit is a `return`.
    fn link(&self, address: u32, chain: bool) -> String {
        let mut linked = String::new();
        let mut pastes = Vec::new();
        let mut reachable = true;
        for line in self.blocks[&address].lines() {
            let forward = constant(line)
                .filter(|target| reachable && *target > address)
                .filter(|target| self.blocks.contains_key(target));
            let line = match forward {
                Some(target) if chain && self.pasted.contains(&target) => {
                    pastes.push(target);
                    retarget(line, &format!("__paste_{target:08x}__"))
                }
                Some(target) => retarget(line, &format!("Ok(w_{target:08x}(m))")),
                None => line.to_owned(),
            };
            if line.trim().starts_with("return ") {
                reachable = false;
            }
            linked.push_str(&line);
            linked.push('\n');
        }
        let defused = defuse(&linked, self.tail_calls);
        // The tail expression leaves the function too.
        let last = defused.trim_end().rfind('\n').map_or(0, |index| index + 1);
        let mut body = format!("{}return {};\n", &defused[..last], defused[last..].trim());
        for target in pastes {
            body = body.replacen(
                &format!("__paste_{target:08x}__"),
                &format!("{{\n{}}}", self.link(target, true)),
                1,
            );
        }
        body
    }
}

/// Keep the emulated registers of one fused function in local variables.
/// Every translated register access is `m.reg(OFFSET, SIZE)` or a statement
/// `m.set_reg(OFFSET, SIZE, value);` with constant operands: 4-byte core and
/// single registers, 8-byte double registers (two singles) and 1-byte flags.
/// Memory accesses never touch the register bytes, so the function reads each
/// register once on entry and writes back every register it assigns before
/// each `return` that leaves it (pasted bodies stay inside). LLVM cannot prove
/// that firmware stores leave the register bytes alone; locals need no proof,
/// so the registers stay in host registers.
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
    let mut reads = |line: &str| {
        let mut out = String::new();
        let mut rest = line;
        while let Some(index) = rest.find("m.reg(") {
            out.push_str(&rest[..index]);
            let close = index + rest[index..].find(')').unwrap();
            let (offset, size) = rest[index + 6..close].split_once(", ").unwrap();
            let (offset, size): (usize, usize) = (offset.parse().unwrap(), size.parse().unwrap());
            out.push_str(&match size {
                1 | 4 => local(offset, size),
                8 => format!("({} | {} << 32)", local(offset, 4), local(offset + 4, 4)),
                _ => panic!("register read of {size} bytes"),
            });
            rest = &rest[close + 1..];
        }
        out.push_str(rest);
        out
    };
    let mut code = String::new();
    let mut writes = Vec::new();
    for line in body.lines() {
        let line = reads(line);
        let Some(operands) = line.trim().strip_prefix("m.set_reg(") else {
            assert!(
                !line.contains("m.set_reg("),
                "register write inside: {line}"
            );
            code.push_str(&line);
            code.push('\n');
            continue;
        };
        let operands = operands
            .strip_suffix(");")
            .expect("register write statement");
        let (offset, rest) = operands.split_once(", ").unwrap();
        let (size, value) = rest.split_once(", ").unwrap();
        let (offset, size): (usize, usize) = (offset.parse().unwrap(), size.parse().unwrap());
        writes.push((offset, size));
        code.push_str(&match size {
            1 => format!("r{offset} = ({value}) & 0xff;\n"),
            4 => format!("r{offset} = ({value}) & 0xffff_ffff;\n"),
            8 => format!(
                "{{ let value: u64 = {value}; r{offset} = value & 0xffff_ffff; r{} = value >> 32; }}\n",
                offset + 4
            ),
            _ => panic!("register write of {size} bytes"),
        });
    }
    drop(reads);
    for (offset, size) in writes {
        if size == 8 {
            local(offset, 4);
            local(offset + 4, 4);
            written.extend([offset, offset + 4]);
        } else {
            local(offset, size);
            written.insert(offset);
        }
    }
    // Flag bytes and register words never share bytes.
    for (&offset, &width) in &locals {
        if width == 1 {
            assert!(locals.get(&(offset & !3)).is_none_or(|w| *w == 1));
        }
    }
    let flush: String = written
        .iter()
        .map(|offset| format!("m.set_reg({offset}, {}, r{offset});\n", locals[offset]))
        .collect();
    let mut out: String = locals
        .iter()
        .map(|(offset, width)| format!("let mut r{offset} = m.reg({offset}, {width});\n"))
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

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_FEATURE");
    let tail_calls = env::var("CARGO_CFG_TARGET_FEATURE")
        .is_ok_and(|features| features.split(',').any(|f| f == "tail-call"));
    assert!(HOST_CALLS
        .iter()
        .all(|pc| !FUSED_PAGES.contains(&(pc >> 12))));
    let mut blocks = BTreeMap::new();
    // Other flash pages keep their translated blocks; only their dispatch changes.
    let mut flash = BTreeMap::<u32, Vec<u32>>::new();
    for entry in fs::read_dir("src/generated").unwrap() {
        let name = entry.unwrap().file_name().into_string().unwrap();
        let Some(page) = name
            .strip_prefix("page_")
            .and_then(|rest| rest.strip_suffix(".rs"))
        else {
            continue;
        };
        let page = u32::from_str_radix(page, 16).unwrap();
        let path = format!("src/generated/{name}");
        println!("cargo:rerun-if-changed={path}");
        // Windows checkouts may convert the generated sources to CRLF.
        let source = fs::read_to_string(&path).unwrap().replace("\r\n", "\n");
        let fused_page = page <= 0x3 || FUSED_PAGES.contains(&page);
        for function in source.split("\npub(crate) fn b_").skip(1) {
            let (address, rest) = function.split_once('(').unwrap();
            let address = u32::from_str_radix(address, 16).unwrap();
            if page <= 0x3 {
                assert!(address < ITCM_END, "ITCM page holds {address:#x}");
            } else {
                assert_eq!(address >> 12, page, "block {address:#x} in page {page:#x}");
            }
            if !fused_page {
                flash.entry(page).or_default().push(address);
                continue;
            }
            let body = rest.split_once("{\n").unwrap().1;
            let body = &body[..body.rfind("\n}").unwrap()];
            assert!(body.starts_with("m.ticks += "), "block {address:#x} layout");
            blocks.insert(address, body.to_owned());
        }
    }
    assert_eq!(
        blocks.keys().filter(|pc| **pc < ITCM_END).count(),
        443,
        "The pinned ITCM translation changed; review the fusion"
    );

    // Static predecessors from each block's reachable branches and
    // fallthrough. The translator emits the fallthrough even after an
    // unconditional branch or return (a call's continuation, which the
    // callee's return reaches dynamically); that one is not an edge.
    let mut predecessors = BTreeMap::<u32, Vec<u32>>::new();
    for (&address, body) in &blocks {
        for line in body.lines() {
            if let Some(target) = constant(line) {
                predecessors.entry(target).or_default().push(address);
            }
            if line.trim().starts_with("return ") {
                break;
            }
        }
    }
    // A block whose only predecessor branches forward to it is pasted into
    // that predecessor. Every other block is a root: its wrapper runs its
    // whole chain of pasted blocks.
    let pasted: BTreeSet<u32> = predecessors
        .iter()
        .filter(|(target, sources)| {
            matches!(sources.as_slice(), [source] if source < target) && blocks.contains_key(target)
        })
        .map(|(target, _)| *target)
        .collect();

    let mut output = String::from(
        "use crate::machine::*;\n\
         #[cold]\n#[inline(never)]\n\
         fn missing(m: &mut Machine) -> u32 {\n\
         let fault = m.fault(format!(\"missing translated block 0x{:08x}\", m.pc));\n\
         m.fail(fault);\n0\n}\n\
         #[cold]\n#[inline(never)]\n\
         fn missing_page(m: &mut Machine) -> Result<u32> {\n\
         Err(m.fault(format!(\"missing translated block 0x{:08x}\", m.pc)))\n}\n",
    );
    let fusion = Fusion {
        blocks: &blocks,
        pasted: &pasted,
        tail_calls,
    };
    for &address in blocks.keys() {
        // A pasted block also keeps a table entry of its own (a dynamic jump
        // could still enter it): the block alone, without its chain.
        let body = fusion.link(address, !pasted.contains(&address));
        output.push_str(&format!(
            "#[inline(never)]\nfn w_{address:08x}(m: &mut Machine) -> u32 {{\n{}}}\n",
            promote(&body)
        ));
    }
    let slots = (ITCM_END / 2) as usize;
    output.push_str(&format!(
        "static ITCM: [fn(&mut Machine) -> u32; {slots}] = [\n"
    ));
    for slot in 0..slots {
        let pc = slot as u32 * 2;
        if blocks.contains_key(&pc) {
            output.push_str(&format!("w_{pc:08x},\n"));
        } else {
            output.push_str("missing,\n");
        }
    }
    output.push_str("];\n");
    for &page in &FUSED_PAGES {
        output.push_str(&format!(
            "static FUSED_{page:05X}: [fn(&mut Machine) -> u32; 2048] = [\n"
        ));
        for slot in 0..2048u32 {
            let pc = (page << 12) | (slot << 1);
            if blocks.contains_key(&pc) {
                output.push_str(&format!("w_{pc:08x},\n"));
            } else {
                output.push_str("missing,\n");
            }
        }
        output.push_str("];\n");
    }
    // Direct tables for the other translated flash pages too: one index per
    // block address instead of a two-level sparse match for every executed block.
    for (&page, addresses) in &flash {
        output.push_str(&format!(
            "static PAGE_{page:05X}: [fn(&mut Machine) -> Result<u32>; 2048] = [\n"
        ));
        let set: std::collections::BTreeSet<u32> = addresses.iter().copied().collect();
        for slot in 0..2048u32 {
            let pc = (page << 12) | (slot << 1);
            if set.contains(&pc) {
                output.push_str(&format!("crate::generated::page_{page:05x}::b_{pc:08x},\n"));
            } else {
                output.push_str("missing_page,\n");
            }
        }
        output.push_str("];\n");
    }
    output.push_str(&format!(
        "/// One fused block (ITCM or a fused flash page): the next program\n\
         /// counter, with any fault parked in the machine (`Machine::faulted`).\n\
         /// `None` for every other address.\n\
         #[inline(always)]\n\
         pub fn fused(m: &mut Machine, pc: u32) -> Option<u32> {{\n\
         if pc < {ITCM_END:#x} {{ return Some(ITCM[(pc >> 1) as usize](m)); }}\n\
         match pc >> 12 {{\n"
    ));
    for &page in &FUSED_PAGES {
        output.push_str(&format!(
            "{page:#x} => Some(FUSED_{page:05X}[((pc & 0xfff) >> 1) as usize](m)),\n"
        ));
    }
    output.push_str(&format!(
        "_ => None,\n}}\n}}\n\
         /// Whether `fused` dispatches this address.\n\
         #[allow(dead_code)]\n\
         pub fn is_fused(pc: u32) -> bool {{\n\
         pc < {ITCM_END:#x} || matches!(pc >> 12, {})\n}}\n\
         /// One block anywhere: fused blocks and flash tables, the rest\n\
         /// through the page dispatcher.\n\
         #[inline(always)]\n\
         pub fn step(m: &mut Machine, pc: u32) -> Result<u32> {{\n\
         if let Some(next) = fused(m, pc) {{\n\
         return if m.faulted() {{ Err(m.take_fault()) }} else {{ Ok(next) }};\n\
         }}\n\
         match pc >> 12 {{\n",
        FUSED_PAGES
            .iter()
            .map(|page| format!("{page:#x}"))
            .collect::<Vec<_>>()
            .join(" | ")
    ));
    for &page in flash.keys() {
        output.push_str(&format!(
            "{page:#x} => PAGE_{page:05X}[((pc & 0xfff) >> 1) as usize](m),\n"
        ));
    }
    output.push_str("_ => crate::generated::run_block(m, pc),\n}\n}\n");
    fs::write(
        PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("fused.rs"),
        output,
    )
    .unwrap();
}
