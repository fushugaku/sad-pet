#!/usr/bin/env python3
"""Build, smoke-test and package the SAD PET standalone app for macOS or Windows.

The app runs the native Rust engine on the computer's audio and MIDI devices
through JUCE; its editor never carries sample data (see plugin/native/Standalone.h).
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import shutil
import subprocess
import tempfile
import time

from build import ROOT, PLUGIN, native_rust, run, rust_toolchain, universal_rust, webview2_options

def smoke_test(executable):
    """Starts the app until its editor has polled the engine without errors."""
    with tempfile.TemporaryDirectory(prefix="sad-pet-app-smoke-") as folder:
        folder = Path(folder)
        report = folder / "report.json"
        # Settings, session, presets and library stay in the temporary folder.
        env = {**os.environ, "SAD_PET_SMOKE_TEST": str(report), "SAD_PET_APP_DIR": str(folder),
               "SAD_PET_LIBRARY_DIR": str(folder / "Library"), "SAD_PET_PRESET_DIR": str(folder / "Presets")}
        process = subprocess.Popen([str(executable)], env=env)
        try:
            code = process.wait(timeout=120)
        except subprocess.TimeoutExpired:
            process.kill()
            raise SystemExit("The app did not finish its smoke test within two minutes")
        result = json.loads(report.read_text()) if report.is_file() else {"ok": False}
        print("Smoke test:", json.dumps(result), flush=True)
        if code != 0 or not result.get("ok"):
            raise SystemExit("The app smoke test failed")


def copy_notices(destination):
    licenses = destination / "Licenses"
    licenses.mkdir(exist_ok=True)
    shutil.copy2(ROOT / "rust/licenses/neuropolis-AGPL-3.0.txt", licenses / "Neuropolis-AGPL-3.0.txt")
    for name in ("superecho", "alphaverb", "dxm088"):
        shutil.copy2(ROOT / "vendor" / name / "NOTICE.md", licenses / f"{name}-NOTICE.md")
    shutil.copy2(PLUGIN / "THIRD_PARTY.md", licenses / "THIRD_PARTY.md")
    for license_file in (ROOT / "rust/assets/samples").glob("*SOURCES*"):
        shutil.copy2(license_file, licenses / license_file.name)
    shutil.copy2(PLUGIN / "APP.md", destination / "README.md")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--build-dir", type=Path, default=ROOT / "target/app")
    parser.add_argument("--juce", type=Path, help="Use an existing JUCE checkout")
    parser.add_argument("--universal", action="store_true", help="Build macOS arm64 + x86_64")
    parser.add_argument("--pgo", nargs="?", const="all", metavar="TARGETS",
                        help="Profile-guide the engine (see pgo.py): every target, or these comma-separated triples")
    parser.add_argument("--smoke", action="store_true", help="Launch the built app once and check its editor")
    parser.add_argument("--jobs", type=int, default=min(os.cpu_count() or 2, 6))
    args = parser.parse_args()
    system = platform.system()
    if system not in ("Darwin", "Windows"):
        parser.error("The app targets macOS or Windows")
    if args.universal and system != "Darwin":
        parser.error("--universal requires macOS")
    build = args.build_dir.resolve()
    cargo, rust_env = rust_toolchain()
    options = ["-DCMAKE_BUILD_TYPE=Release", f"-DCARGO_EXECUTABLE={cargo}", "-DSAD_PET_BUILD_HOST_TESTS=OFF"]
    if args.juce:
        options += [f"-DSAD_PET_JUCE_PATH={args.juce.resolve()}"]
    if system == "Windows":
        options += webview2_options()
    if args.universal:
        options += universal_rust(build, cargo, rust_env, args.pgo)
    else:
        options += native_rust(build, cargo, rust_env, args.pgo)
    run("cmake", "-S", PLUGIN, "-B", build, *options, env=rust_env)
    run("cmake", "--build", build, "--config", "Release", "--target", "SadPetApp_Standalone",
        "--parallel", args.jobs, env=rust_env)

    products = build / "SadPetApp_artefacts/Release/Standalone"
    suffix = "macOS-universal" if args.universal else f"macOS-{platform.machine()}" if system == "Darwin" else "Windows-x64"
    output = ROOT / "dist/app" / f"SAD-PET-App-{suffix}"
    if output.exists():
        shutil.rmtree(output)
    output.mkdir(parents=True)
    if system == "Darwin":
        app = output / "SAD PET.app"
        shutil.copytree(products / "SAD PET.app", app, symlinks=True)
        run("codesign", "--force", "--deep", "--sign", "-", app)
        run("codesign", "--verify", "--deep", "--strict", app)
        executable = app / "Contents/MacOS/SAD PET"
    else:
        # The executable and the C++ runtime it was built with, kept side by side.
        for file in products.iterdir():
            if file.suffix.lower() in (".exe", ".dll"):
                shutil.copy2(file, output / file.name)
        executable = output / "SAD PET.exe"
    if not executable.is_file():
        raise FileNotFoundError(f"Build did not produce {executable}")
    if args.smoke:
        started = time.monotonic()
        smoke_test(executable)
        print(f"Smoke test passed in {time.monotonic() - started:.1f} s")
    copy_notices(output)
    juce_source = args.juce or build / "_deps/juce-src"
    if (juce_source / "LICENSE.md").is_file():
        shutil.copy2(juce_source / "LICENSE.md", output / "Licenses/JUCE-LICENSE.md")

    archive = output.with_suffix(".zip")
    archive.unlink(missing_ok=True)
    if system == "Darwin":
        # ditto keeps the bundle's signature and metadata intact.
        run("ditto", "-c", "-k", "--keepParent", output, archive)
    else:
        shutil.make_archive(str(output), "zip", output.parent, output.name)
    checksum = hashlib.sha256(archive.read_bytes()).hexdigest()
    archive.with_suffix(".zip.sha256").write_text(f"{checksum}  {archive.name}\n")
    print(f"Packaged {archive}")


if __name__ == "__main__":
    main()
