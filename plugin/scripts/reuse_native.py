#!/usr/bin/env python3
"""Reuse validated binaries only when their complete native source is unchanged."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import zipfile


ROOT = Path(__file__).resolve().parents[2]
VALIDATED = {
    "macOS-universal": (38980814940, "562db49057171eee7c4078a6f1903b8a0aab57b5"),
    "Windows-x64": (39128391146, "b4305b42ba585c9f0f4072a8237bca1526a728dd"),
}
NATIVE_SOURCES = ["Cargo.toml", "Cargo.lock", "rust", "plugin/Cargo.toml",
                  "plugin/CMakeLists.txt", "plugin/src", "plugin/native", "plugin/web"]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("platform", choices=VALIDATED)
    args = parser.parse_args()
    run, reference = VALIDATED[args.platform]
    subprocess.run(["git", "fetch", "--depth=1", "origin", reference], check=True)
    changed = subprocess.run(["git", "diff", "--quiet", reference, "HEAD", "--", *NATIVE_SOURCES])
    if changed.returncode == 1:
        print("Native sources changed; compiling from source")
        return
    changed.check_returncode()
    jobs = json.loads(subprocess.check_output(
        ["gh", "api", f"repos/{os.environ['GITHUB_REPOSITORY']}/actions/runs/{run}/jobs"], text=True))
    if not any(j["name"] == args.platform and j["conclusion"] == "success" for j in jobs["jobs"]):
        raise ValueError("Reference native build did not pass validation")
    downloaded = ROOT / "target/validated-native" / args.platform
    downloaded.mkdir(parents=True, exist_ok=True)
    subprocess.run(["gh", "run", "download", str(run), "--name", f"SAD-PET-Plugins-{args.platform}",
                    "--dir", str(downloaded)], check=True)
    archive = downloaded / f"SAD-PET-Plugins-{args.platform}.zip"
    expected = Path(str(archive) + ".sha256").read_text().split()[0]
    if hashlib.sha256(archive.read_bytes()).hexdigest() != expected:
        raise ValueError("Validated native archive checksum mismatch")
    output = ROOT / "dist/plugins"
    output.mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(archive) as package:
        for entry in package.infolist():
            if not (output / entry.filename).resolve().is_relative_to(output.resolve()):
                raise ValueError("Unsafe artifact path")
        package.extractall(output)
    staged = output / f"SAD-PET-Plugins-{args.platform}"
    for kind in (("AU", "VST3") if args.platform.startswith("macOS") else ("VST3",)):
        for target, name in (("SadPet", "SAD PET"), ("SadPetFX", "SAD PET FX")):
            extension = ".component" if kind == "AU" else ".vst3"
            destination = ROOT / f"target/plugins/{target}_artefacts/Release/{kind}/{name}{extension}"
            shutil.copytree(staged / kind / (name + extension), destination)
            if args.platform.startswith("macOS"):
                # zipfile does not restore executable mode bits.
                for executable in (destination / "Contents/MacOS").iterdir():
                    executable.chmod(0o755)
    if "GITHUB_OUTPUT" in os.environ:
        with Path(os.environ["GITHUB_OUTPUT"]).open("a") as output:
            output.write("arguments=--package-only\n")
    print(f"Reusing native binaries validated in run {run}; native source matches {reference}")


if __name__ == "__main__":
    main()
