#!/usr/bin/env python3
"""Build, test and package the native SAD PET instrument and MIDI-capable effect."""
import argparse
import hashlib
import os
from pathlib import Path
import platform
import shutil
import subprocess
import urllib.request
import zipfile

from package_macos import build_pkg
from package_windows import build_exe

PLUGIN = Path(__file__).resolve().parents[1]
ROOT = PLUGIN.parent
WEBVIEW_VERSION = "1.0.3485.44"


def run(*args, env=None):
    subprocess.run([str(arg) for arg in args], cwd=ROOT, env=env, check=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--build-dir", type=Path, default=ROOT / "target/plugins")
    parser.add_argument("--juce", type=Path, help="Use an existing JUCE checkout")
    parser.add_argument("--universal", action="store_true", help="Build macOS arm64 + x86_64")
    parser.add_argument("--package-only", action="store_true")
    parser.add_argument("--skip-tests", action="store_true")
    parser.add_argument("--jobs", type=int, default=min(os.cpu_count() or 2, 6))
    args = parser.parse_args()
    system = platform.system()
    if system not in ("Darwin", "Windows"):
        parser.error("Native plugin packages target macOS or Windows")
    build = args.build_dir.resolve()
    rust_env = dict(os.environ)
    cargo = "cargo"
    if shutil.which("rustup"):
        # Homebrew cargo/rustc can shadow rustup and miss its cross-target stdlibs.
        cargo = subprocess.check_output(["rustup", "which", "cargo"], text=True).strip()
        rust_env.setdefault("RUSTC", subprocess.check_output(["rustup", "which", "rustc"], text=True).strip())
    options = ["-DCMAKE_BUILD_TYPE=Release"]
    options += [f"-DCARGO_EXECUTABLE={cargo}"]
    if args.juce:
        options += [f"-DSAD_PET_JUCE_PATH={args.juce.resolve()}"]
    if system == "Windows":
        options += ["-A", "x64", "-DSAD_PET_CARGO_TARGET=x86_64-pc-windows-msvc"]
        package_root = ROOT / "target/plugin-deps/WebView2"
        sdk = package_root / f"Microsoft.Web.WebView2.{WEBVIEW_VERSION}"
        if not (sdk / "build/native/include/WebView2.h").is_file():
            sdk.mkdir(parents=True, exist_ok=True)
            archive = package_root / "webview2.nupkg"
            urllib.request.urlretrieve(f"https://api.nuget.org/v3-flatcontainer/microsoft.web.webview2/{WEBVIEW_VERSION}/microsoft.web.webview2.{WEBVIEW_VERSION}.nupkg", archive)
            with zipfile.ZipFile(archive) as package:
                package.extractall(sdk)
        options += [f"-DJUCE_WEBVIEW2_PACKAGE_LOCATION={package_root}"]
    if args.universal and system != "Darwin":
        parser.error("--universal requires macOS")
    if args.universal and not args.package_only:
        rust = build / "universal-rust"
        archives = []
        run("rustup", "target", "add", "aarch64-apple-darwin", "x86_64-apple-darwin")
        for target in ("aarch64-apple-darwin", "x86_64-apple-darwin"):
            run(cargo, "build", "-p", "sad-pet-plugin", "--release", "--locked", "--target", target,
                env={**rust_env, "CARGO_TARGET_DIR": str(rust)})
            archives.append(rust / target / "release/libsad_pet_plugin.a")
        universal = rust / "libsad_pet_plugin.a"
        run("lipo", "-create", *archives, "-output", universal)
        options += ["-DCMAKE_OSX_ARCHITECTURES=arm64;x86_64", f"-DSAD_PET_RUST_ARCHIVE={universal}"]
    if not args.package_only:
        run("cmake", "-S", PLUGIN, "-B", build, *options, env=rust_env)
        run("cmake", "--build", build, "--config", "Release", "--parallel", args.jobs, env=rust_env)
        if not args.skip_tests:
            run("ctest", "--test-dir", build, "-C", "Release", "--output-on-failure")

    suffix = "macOS-universal" if args.universal else "macOS-" + platform.machine() if system == "Darwin" else "Windows-x64"
    output = ROOT / "dist/plugins" / f"SAD-PET-Plugins-{suffix}"
    output.mkdir(parents=True, exist_ok=True)
    for kind in (["VST3", "AU"] if system == "Darwin" else ["VST3"]):
        folder = output / kind
        folder.mkdir(exist_ok=True)
        for target, name in [("SadPet", "SAD PET"), ("SadPetFX", "SAD PET FX")]:
            extension = ".vst3" if kind == "VST3" else ".component"
            source = build / f"{target}_artefacts/Release" / kind / (name + extension)
            if not source.is_dir():
                raise FileNotFoundError(f"Build did not produce {source}")
            destination = folder / source.name
            if destination.exists():
                shutil.rmtree(destination)
            shutil.copytree(source, destination)
            if system == "Darwin":
                run("codesign", "--force", "--deep", "--sign", "-", destination)
                run("codesign", "--verify", "--deep", "--strict", destination)
    installer = "Install.command" if system == "Darwin" else "Install.ps1"
    shutil.copy2(PLUGIN / "scripts" / installer, output / installer)
    if system == "Darwin":
        (output / installer).chmod(0o755)
    shutil.copy2(PLUGIN / "README.md", output / "README.md")
    licenses = output / "Licenses"
    licenses.mkdir(exist_ok=True)
    juce_source = args.juce or build / "_deps/juce-src"
    if (juce_source / "LICENSE.md").is_file():
        shutil.copy2(juce_source / "LICENSE.md", licenses / "JUCE-LICENSE.md")
    shutil.copy2(PLUGIN / "THIRD_PARTY.md", licenses / "THIRD_PARTY.md")
    for license_file in (ROOT / "rust/assets/samples").glob("*SOURCES*"):
        shutil.copy2(license_file, licenses / license_file.name)
    if system == "Windows":
        build_exe(output)
    archive = Path(shutil.make_archive(str(output), "zip", output.parent, output.name))
    checksum = hashlib.sha256(archive.read_bytes()).hexdigest()
    archive.with_suffix(".zip.sha256").write_text(f"{checksum}  {archive.name}\n")
    print(f"Packaged {archive}")
    if system == "Darwin":
        build_pkg(output)


if __name__ == "__main__":
    main()
