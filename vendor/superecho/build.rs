//! Fuse the ITCM firmware (DSP_Process_Block and the control tick) without
//! changing its instructions. Each translated basic block is unchanged; what
//! changes is how execution moves between them:
//!
//! * Every ITCM block gets a slot in a direct jump table indexed by its address,
//!   replacing the two-level `match` dispatch for the hottest code.
//! * A block whose only static predecessor branches forward to it is inlined
//!   into that predecessor, so LLVM keeps emulated registers in locals across
//!   the pair. Only forward edges are inlined, so no loop becomes recursion.
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
use std::{collections::BTreeMap, env, fs, path::PathBuf};

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

    // Static successors: `return Ok((0x..u32) & !1);` branches and the tail
    // `Ok(0x..)` fallthrough. Dynamic returns through a register stay dynamic.
    let constant = |text: &str| -> Option<u32> {
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
    };
    let mut incoming = BTreeMap::<u32, Vec<u32>>::new();
    for (&address, body) in &blocks {
        for line in body.lines() {
            if let Some(target) = constant(line) {
                incoming.entry(target).or_default().push(address);
            }
        }
    }
    // A block is a chain continuation when exactly one ITCM block jumps to it
    // and that block precedes it. It also keeps its own table entry: returns
    // through a register may still enter it directly.
    let chained = |target: u32| -> Option<u32> {
        let sources = incoming.get(&target)?;
        match sources.as_slice() {
            [source] if *source < target && blocks.contains_key(&target) => Some(*source),
            _ => None,
        }
    };

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
    for (&address, body) in &blocks {
        let mut fused = String::new();
        for line in body.lines() {
            let direct = constant(line)
                .filter(|target| chained(*target) == Some(address))
                .filter(|target| blocks.contains_key(target));
            // A forward branch to a block that other blocks also reach is a
            // direct call of that block's wrapper: no loop iteration, table
            // lookup or indirect call in between. Backward edges still return
            // to the dispatcher, so no loop can recurse, and every forward
            // path is finite, so the call depth is bounded by the block count.
            let forward = constant(line)
                .filter(|_| direct.is_none())
                .filter(|target| *target > address && blocks.contains_key(target));
            if let Some(target) = forward {
                let call = format!("w_{target:08x}(m)");
                let rewritten = if let Some(start) = line.find("return Ok(") {
                    let end = start + line[start..].find(';').unwrap() + 1;
                    format!("{}return Ok({call});{}", &line[..start], &line[end..])
                } else {
                    let trimmed = line.trim();
                    debug_assert!(trimmed.starts_with("Ok("));
                    line.replacen(trimmed, &format!("Ok({call})"), 1)
                };
                fused.push_str(&rewritten);
                fused.push('\n');
                continue;
            }
            match direct {
                Some(target) if line.trim().starts_with("return ") => {
                    // `if cond { return Ok(X); }` keeps its braces around the call.
                    let call = format!("return b_{target:08x}(m);");
                    let rewritten = line.replacen(line.trim(), &call, 1);
                    // The branch form is the only `return Ok(` on its line.
                    let rewritten = if line.trim().starts_with("return Ok(") {
                        rewritten
                    } else {
                        let start = line.find("return Ok(").unwrap();
                        let end = start + line[start..].find(';').unwrap() + 1;
                        format!("{}{}{}", &line[..start], call, &line[end..])
                    };
                    fused.push_str(&rewritten);
                }
                Some(target) => fused.push_str(&format!("b_{target:08x}(m)")),
                None => fused.push_str(line),
            }
            fused.push('\n');
        }
        let fused = defuse(&fused, tail_calls);
        // Chain continuations inline into their predecessor; every block also
        // inlines into its own table wrapper.
        output.push_str(&format!(
            "#[inline(always)]\nfn b_{address:08x}(m: &mut Machine) -> u32 {{\n{fused}}}\n\
             #[inline(never)]\nfn w_{address:08x}(m: &mut Machine) -> u32 {{\n\
             b_{address:08x}(m)\n}}\n"
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
