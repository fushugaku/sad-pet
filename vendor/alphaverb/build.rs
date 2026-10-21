//! Fuse the two hot delay-line routines without changing their instructions.
//! Their control-flow graphs are acyclic: direct, inlined edges let LLVM keep
//! emulated registers in locals instead of dispatching every firmware block.
use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::PathBuf,
};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let mut blocks = BTreeMap::new();
    for index in [4, 5, 9] {
        let path = format!("src/audio_generated/blocks_{index}.rs");
        println!("cargo:rerun-if-changed={path}");
        let source = fs::read_to_string(path).unwrap();
        for function in source.split("pub fn b_").skip(1) {
            let (address, _) = function.split_once('(').unwrap();
            let address = u32::from_str_radix(address, 16).unwrap();
            let body = function.split_once('{').unwrap().1;
            let body = body.split_once("\n}").unwrap().0;
            blocks.insert(address, body.to_owned());
        }
    }
    let hot =
        |pc| (0x60029898..=0x60029b54).contains(&pc) || (0x60029dcc..=0x6002a014).contains(&pc);
    assert_eq!(
        blocks.keys().filter(|pc| hot(**pc)).count(),
        40,
        "The pinned firmware blocks changed; review the fusion boundaries"
    );
    // Keep shared suffixes out of line: duplicating every branch combination
    // makes both LLVM compilation and browser WASM compilation unnecessarily big.
    let target = |line: &str| {
        let line = line.trim().strip_prefix("return ").unwrap_or(line.trim());
        u32::from_str_radix(
            line.trim_end_matches(';')
                .trim_end_matches("u32")
                .strip_prefix("0x")?,
            16,
        )
        .ok()
    };
    let mut incoming = BTreeMap::<u32, usize>::new();
    for (&address, body) in blocks.iter().filter(|(pc, _)| hot(**pc)) {
        let mut destinations = BTreeSet::new();
        let mut depth = 0;
        for line in body.lines() {
            if let Some(pc) = target(line).filter(|pc| hot(*pc)) {
                destinations.insert(pc);
            }
            // The translator emits a fallthrough even after an unconditional
            // branch/return. It is unreachable and must not split a hot chain.
            if depth == 1 && line.trim().starts_with("return ") {
                break;
            }
            depth += line.chars().filter(|c| *c == '{').count() as i32;
            depth -= line.chars().filter(|c| *c == '}').count() as i32;
        }
        if address == 0x60029a3a {
            destinations.insert(0x60029aa2);
        }
        if address == 0x60029ef6 {
            destinations.insert(0x60029f5a);
        }
        for pc in destinations {
            *incoming.entry(pc).or_default() += 1;
        }
    }
    let mut output = String::from("use crate::machine::{Machine, signed, signed_overflow};\n");
    // Retain the original abs instructions, including their register/flag effects.
    output.push_str(&format!(
        "#[inline(always)]\nfn abs(c: &mut Machine) -> u32 {{{}\n}}\n",
        blocks[&0x6004137c]
    ));
    for (&address, body) in blocks.iter().filter(|(pc, _)| hot(**pc)) {
        let mut fused = String::new();
        for line in body.lines() {
            let trimmed = line.trim();
            let target = trimmed
                .strip_prefix("return ")
                .unwrap_or(trimmed)
                .trim_end_matches(';')
                .trim_end_matches("u32");
            if let Some(hex) = target.strip_prefix("0x") {
                if let Ok(pc) = u32::from_str_radix(hex, 16) {
                    // An unreachable fallthrough after a dynamic return is kept.
                    if hot(pc) && blocks.contains_key(&pc) {
                        fused.push_str(&format!("if c.fault.get().is_some() {{ return 0x{pc:x}; }}\nreturn b_{pc:x}(c);\n"));
                        continue;
                    }
                    if pc == 0x6004137c {
                        let continuation = match address {
                            0x60029a3a => 0x60029aa2,
                            0x60029ef6 => 0x60029f5a,
                            _ => panic!("Unexpected abs caller: {address:x}"),
                        };
                        fused.push_str(&format!("if c.fault.get().is_some() {{ return 0x{pc:x}; }}\nlet pc = abs(c);\nif c.fault.get().is_some() || pc != 0x{continuation:x} {{ return pc; }}\nreturn b_{continuation:x}(c);\n"));
                        continue;
                    }
                }
            }
            fused.push_str(line);
            fused.push('\n');
        }
        let inline = if incoming.get(&address).copied().unwrap_or(0) > 1
            || matches!(address, 0x60029898 | 0x60029dcc)
        {
            "never"
        } else {
            "always"
        };
        output.push_str(&format!(
            "#[inline({inline})]\npub fn b_{address:x}(c: &mut Machine) -> u32 {{\n{fused}\n}}\n"
        ));
    }
    fs::write(
        PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("fused.rs"),
        output,
    )
    .unwrap();
}
