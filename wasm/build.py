#!/usr/bin/env python3
"""Build the standalone static site; no server, Node packages or wasm-bindgen needed."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess

ROOT = Path(__file__).resolve().parent.parent


def copy_voice_packs(folder):
    """The voicebanks that come with SAD PET, and a list of them the browser
    reads before fetching one: id, header info, size and a content version."""
    folder.mkdir(parents=True, exist_ok=True)
    packs = []
    for pack in sorted((ROOT / "voice" / "packs").glob("*.sadvoice")):
        data = pack.read_bytes()
        assert data[:8] == b"SADVOICE", f"{pack} is not a voicebank"
        length = int.from_bytes(data[12:16], "little")
        info = json.loads(data[16:16 + length])["info"]
        shutil.copy2(pack, folder / pack.name)
        packs.append({"id": pack.stem, "file": pack.name, "name": info["name"], "language": info["language"],
                      "author": info.get("author", ""), "size": len(data),
                      "version": hashlib.sha256(data).hexdigest()[:12]})
    # In the order of the editor's languages, as `vocal::PACKS` lists them.
    packs.sort(key=lambda p: (["ja", "en", "ru"].index(p["language"]), p["name"]))
    (folder / "packs.json").write_text(json.dumps(packs, ensure_ascii=False, indent=1) + "\n")

def strip_debug_names(data):
    """Remove metadata only; preserve every executable/data section byte."""
    def integer(offset):
        value, shift = 0, 0
        while True:
            byte = data[offset]
            offset += 1
            value |= (byte & 127) << shift
            if byte < 128:
                return value, offset
            shift += 7

    output = bytearray(data[:8])
    offset = 8
    while offset < len(data):
        kind = data[offset]
        size, start = integer(offset + 1)
        end = start + size
        name = b""
        if kind == 0:
            length, name_start = integer(start)
            name = data[name_start:name_start + length]
        if name != b"name" and not name.startswith(b".debug_"):
            output.extend(data[offset:end])
        offset = end
    return output


def cargo_build(env, target, features):
    """Compile the engine; each target-feature variant keeps its own target directory."""
    env = env.copy()
    if features:
        target = target.with_name(target.name + "-" + "-".join(features))
        flags = ",".join("+" + feature for feature in features)
        env["RUSTFLAGS"] = (env.get("RUSTFLAGS", "") + f" -C target-feature={flags}").strip()
    env["CARGO_TARGET_DIR"] = str(target)
    subprocess.run([
        "cargo", "build", "--locked", "--profile", "wasm", "--target", "wasm32-unknown-unknown",
        "-p", "sad-pet-wasm",
    ], cwd=ROOT, env=env, check=True)
    return target / "wasm32-unknown-unknown" / "wasm" / "sad_pet_wasm.wasm"


def optimize(wasm_opt, level, path, features):
    """Binaryen keeps IEEE float semantics without --fast-math: identical renders."""
    enabled = [
        "--enable-bulk-memory", "--enable-nontrapping-float-to-int", "--enable-sign-ext",
        "--enable-mutable-globals", "--enable-multivalue", "--enable-reference-types",
    ]
    if "simd128" in features:
        enabled.append("--enable-simd")
    if "tail-call" in features:
        enabled.append("--enable-tail-call")
    subprocess.run([wasm_opt, level, *enabled, "-o", str(path), str(path)], check=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=ROOT / "dist" / "web")
    parser.add_argument("--keep-debug-names", action="store_true", help="Keep WASM names for profiling")
    parser.add_argument("--wasm-opt", nargs="?", const="-O1", metavar="LEVEL",
                        help="Also run Binaryen's wasm-opt at LEVEL (default -O1). Off by default: "
                             "-O3 measured 25-47%% slower on the translated SuperEcho firmware")
    parser.add_argument("--simd", action="store_true",
                        help="Also build sad_pet.simd.wasm (+simd128). Experimental: it renders "
                             "identical samples but measured about 5%% slower in V8")
    parser.add_argument("--tail-call", action="store_true",
                        help="Also build sad_pet.tc.wasm (+tail-call), which browsers that validate "
                             "tail calls load instead of the baseline: the translated firmware "
                             "runs block to block without its dispatch loop. Identical samples, "
                             "measured about 2.5%% faster on SuperEcho scenes, a second 20 MB file")
    args = parser.parse_args()
    env = os.environ.copy()
    # Homebrew's cargo can precede rustup on PATH; use the rustc that owns the
    # installed wasm target rather than the unrelated Homebrew sysroot.
    if shutil.which("rustup") and "RUSTC" not in env:
        env["RUSTC"] = subprocess.check_output(["rustup", "which", "rustc"], text=True).strip()
    target = Path(env.get("CARGO_TARGET_DIR", ROOT / "target" / "browser"))
    if not target.is_absolute():
        target = ROOT / target
    wasm_opt = None
    if args.wasm_opt:
        wasm_opt = env.get("WASM_OPT") or shutil.which("wasm-opt")
        if wasm_opt is None:
            raise SystemExit("--wasm-opt needs Binaryen's wasm-opt on PATH or in $WASM_OPT")
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=True)
    variants = [("sad_pet.wasm", [])]
    if args.tail_call:
        variants.append(("sad_pet.tc.wasm", ["tail-call"]))
    if args.simd:
        variants.append(("sad_pet.simd.wasm", ["simd128"]))
    for name, features in variants:
        built = cargo_build(env, target, features)
        shutil.copy2(built, output / name)
        if wasm_opt:
            optimize(wasm_opt, args.wasm_opt, output / name, features)
        if not args.keep_debug_names:
            wasm = output / name
            wasm.write_bytes(strip_debug_names(wasm.read_bytes()))
    if not args.simd:
        (output / "sad_pet.simd.wasm").unlink(missing_ok=True)
    if not args.tail_call:
        (output / "sad_pet.tc.wasm").unlink(missing_ok=True)
    for name in ("index.html", "app.js", "library.js", "selection.js", "presets.js", "sequence.js", "panels.js", "vocal.js", "sampler-library.js", "sampler.js", "midi.js", "modulation.js", "style.css", "vocal.css", "sampler.css"):
        shutil.copy2(ROOT / "rust" / "web" / name, output / name)
    index = (output / "index.html").read_text()
    index = index.replace('<script src="./library.js"', '<script src="./browser.js" defer></script>\n    <script src="./library.js"')
    index = index.replace("RUST EDITION", "WEBASSEMBLY")
    (output / "index.html").write_text(index)
    shutil.copytree(ROOT / "wasm" / "web", output, dirs_exist_ok=True)
    shutil.copytree(ROOT / "rust/assets/samples/loops-256", output / "samples/loops-256", dirs_exist_ok=True)
    # The Sampler's starter packs: CC0 kits the editor loads into IndexedDB on first use.
    shutil.copytree(ROOT / "rust/assets/sampler-packs", output / "sampler-packs", dirs_exist_ok=True)
    copy_voice_packs(output / "voices")
    if not args.tail_call:
        # Without the variant the loader must not spend a request on it.
        backend = output / "browser-backend.mjs"
        source = backend.read_text()
        reference = 'new URL("./sad_pet.tc.wasm", import.meta.url)'
        assert source.count(reference) == 1, "tail-call engine reference in browser-backend.mjs"
        backend.write_text(source.replace(reference, "undefined"))
    (output / "licenses").mkdir(exist_ok=True)
    shutil.copy2(ROOT / "rust/licenses/neuropolis-AGPL-3.0.txt", output / "licenses/neuropolis-AGPL-3.0.txt")
    for name in ("superecho", "alphaverb", "dxm088"):
        shutil.copy2(ROOT / "vendor" / name / "NOTICE.md", output / "licenses" / f"{name}-NOTICE.md")
    # A cached worklet/bridge from another build can otherwise run alongside
    # the new HTML. Give every runtime dependency one consistent build version.
    runtime = sorted(p for p in output.rglob("*") if p.is_file() and p.suffix in (".js", ".mjs", ".css", ".wasm"))
    version = hashlib.sha256(b"".join(p.read_bytes() for p in runtime)).hexdigest()[:12]
    for path in [output / "index.html", *[p for p in runtime if p.suffix in (".js", ".mjs") and "vendor" not in p.relative_to(output).parts]]:
        source = path.read_text()
        source = re.sub(r'([\"\'])(\./[^\"\'\s?]+\.(?:mjs|js|css|wasm))\1',
                        lambda match: f'{match[1]}{match[2]}?v={version}{match[1]}', source)
        path.write_text(source)
    (output / ".nojekyll").touch()
    sizes = ", ".join(f"{name} {(output / name).stat().st_size:,} bytes" for name, _ in variants)
    print(f"Built {output} ({sizes})")


if __name__ == "__main__":
    main()
