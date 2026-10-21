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
use std::{collections::BTreeMap, env, fs, path::PathBuf};

const ITCM_END: u32 = 0x3500;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let mut blocks = BTreeMap::new();
    // Flash pages keep their translated blocks; only their dispatch changes.
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
        if page <= 0x3 {
            continue;
        }
        let path = format!("src/generated/{name}");
        println!("cargo:rerun-if-changed={path}");
        let source = fs::read_to_string(&path).unwrap().replace("\r\n", "\n");
        for function in source.split("\npub(crate) fn b_").skip(1) {
            let (address, _) = function.split_once('(').unwrap();
            let address = u32::from_str_radix(address, 16).unwrap();
            assert_eq!(address >> 12, page, "block {address:#x} in page {page:#x}");
            flash.entry(page).or_default().push(address);
        }
    }
    for page in ["00000", "00001", "00002", "00003"] {
        let path = format!("src/generated/page_{page}.rs");
        println!("cargo:rerun-if-changed={path}");
        // Windows checkouts may convert the generated sources to CRLF.
        let source = fs::read_to_string(&path).unwrap().replace("\r\n", "\n");
        for function in source.split("\npub(crate) fn b_").skip(1) {
            let (address, rest) = function.split_once('(').unwrap();
            let address = u32::from_str_radix(address, 16).unwrap();
            assert!(address < ITCM_END, "ITCM page holds {address:#x}");
            let body = rest.split_once("{\n").unwrap().1;
            let body = &body[..body.rfind("\n}").unwrap()];
            assert!(body.starts_with("m.ticks += "), "block {address:#x} layout");
            blocks.insert(address, body.to_owned());
        }
    }
    assert_eq!(
        blocks.len(),
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
         fn missing(m: &mut Machine) -> Result<u32> {\n\
         Err(m.fault(format!(\"missing translated block 0x{:08x}\", m.pc)))\n}\n",
    );
    for (&address, body) in &blocks {
        let mut fused = String::new();
        for line in body.lines() {
            let direct = constant(line)
                .filter(|target| chained(*target) == Some(address))
                .filter(|target| blocks.contains_key(target));
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
        let inline = if chained(address).is_some() {
            "always"
        } else {
            "never"
        };
        output.push_str(&format!(
            "#[inline({inline})]\nfn b_{address:08x}(m: &mut Machine) -> Result<u32> {{\n{fused}}}\n"
        ));
    }
    let slots = (ITCM_END / 2) as usize;
    output.push_str(&format!(
        "static ITCM: [fn(&mut Machine) -> Result<u32>; {slots}] = [\n"
    ));
    for slot in 0..slots {
        let pc = slot as u32 * 2;
        if blocks.contains_key(&pc) {
            output.push_str(&format!("b_{pc:08x},\n"));
        } else {
            output.push_str("missing,\n");
        }
    }
    output.push_str("];\n");
    // Direct tables for the translated flash pages too: one index per block
    // address instead of a two-level sparse match for every executed block.
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
                output.push_str("missing,\n");
            }
        }
        output.push_str("];\n");
    }
    output.push_str(&format!(
        "/// One ITCM block. The caller guarantees `pc < {ITCM_END:#x}`.\n\
         #[inline(always)]\n\
         pub fn itcm(m: &mut Machine, pc: u32) -> Result<u32> {{\n\
         ITCM[(pc >> 1) as usize](m)\n}}\n\
         /// One block anywhere: ITCM and flash through their tables, the rest\n\
         /// through the page dispatcher.\n\
         #[inline(always)]\n\
         pub fn step(m: &mut Machine, pc: u32) -> Result<u32> {{\n\
         if pc < {ITCM_END:#x} {{ return itcm(m, pc); }}\n\
         match pc >> 12 {{\n"
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
