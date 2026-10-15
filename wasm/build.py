#!/usr/bin/env python3
"""Build the standalone static site; no server, Node packages or wasm-bindgen needed."""
import argparse
import os
from pathlib import Path
import shutil
import subprocess

ROOT = Path(__file__).resolve().parent.parent


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=ROOT / "dist" / "web")
    args = parser.parse_args()
    env = os.environ.copy()
    # Homebrew's cargo can precede rustup on PATH; use the rustc that owns the
    # installed wasm target rather than the unrelated Homebrew sysroot.
    if shutil.which("rustup") and "RUSTC" not in env:
        env["RUSTC"] = subprocess.check_output(["rustup", "which", "rustc"], text=True).strip()
    target = Path(env.get("CARGO_TARGET_DIR", ROOT / "target" / "browser"))
    if not target.is_absolute():
        target = ROOT / target
    env["CARGO_TARGET_DIR"] = str(target)
    subprocess.run([
        "cargo", "build", "--locked", "--release", "--target", "wasm32-unknown-unknown",
        "-p", "sad-pet-wasm",
    ], cwd=ROOT, env=env, check=True)
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=True)
    for name in ("index.html", "app.js", "library.js", "style.css"):
        shutil.copy2(ROOT / "rust" / "web" / name, output / name)
    index = (output / "index.html").read_text()
    index = index.replace('<script src="./library.js"', '<script src="./browser.js" defer></script>\n    <script src="./library.js"')
    index = index.replace("RUST EDITION", "WEBASSEMBLY")
    (output / "index.html").write_text(index)
    shutil.copytree(ROOT / "wasm" / "web", output, dirs_exist_ok=True)
    shutil.copy2(target / "wasm32-unknown-unknown" / "release" / "sad_pet_wasm.wasm", output / "sad_pet.wasm")
    (output / ".nojekyll").touch()
    print(f"Built {output} ({(output / 'sad_pet.wasm').stat().st_size:,} byte WASM engine)")


if __name__ == "__main__":
    main()
