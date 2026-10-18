#!/usr/bin/env python3
"""Verify both tested installer artifacts, then publish one complete GitHub release."""
import argparse
import hashlib
import json
from pathlib import Path
import plistlib
import re
import shutil
import subprocess
import tempfile
import tomllib
import zipfile


PLUGIN = Path(__file__).resolve().parents[1]
PACKAGES = (
    "SAD-PET-Plugins-macOS-universal.pkg",
    "SAD-PET-Plugins-macOS-universal.zip",
    "SAD-PET-Plugins-Windows-x64-Setup.exe",
    "SAD-PET-Plugins-Windows-x64.zip",
)


def validate_tag(tag: str) -> str:
    version = tomllib.loads((PLUGIN / "Cargo.toml").read_text())["package"]["version"]
    if tag != f"plugins-v{version}":
        raise ValueError(f"Expected release tag plugins-v{version}, got {tag}")
    return version


def stage_assets(artifacts: Path, output: Path, version: str) -> list[Path]:
    files = {}
    for name in PACKAGES:
        for filename in (name, name + ".sha256"):
            matches = list(artifacts.rglob(filename))
            if len(matches) != 1:
                raise ValueError(f"Expected one {filename}, found {len(matches)}")
            files[filename] = matches[0]
    checksums = []
    for name in PACKAGES:
        checksum = hashlib.sha256(files[name].read_bytes()).hexdigest()
        expected = files[name + ".sha256"].read_text().strip()
        if expected != f"{checksum}  {name}":
            raise ValueError(f"Checksum mismatch: {name}")
        checksums.append(expected)
    for system, extension in (("macOS-universal", "component"), ("Windows-x64", "vst3")):
        with zipfile.ZipFile(files[f"SAD-PET-Plugins-{system}.zip"]) as archive:
            for name in ("SAD PET", "SAD PET FX"):
                prefix = f"SAD-PET-Plugins-{system}/{'AU' if extension == 'component' else 'VST3'}/{name}.{extension}/Contents/"
                if extension == "component":
                    actual = plistlib.loads(archive.read(prefix + "Info.plist"))["CFBundleShortVersionString"]
                else:
                    info = archive.read(prefix + "Resources/moduleinfo.json").decode()
                    match = re.search(r'"Version"\s*:\s*"([^\"]+)"', info)
                    actual = match.group(1) if match else None
                if actual != version:
                    raise ValueError(f"Expected {version} in {system}/{name}, got {actual}")
    if files[PACKAGES[0]].read_bytes()[:4] != b"xar!":
        raise ValueError("Not a macOS installer package")
    if files[PACKAGES[2]].read_bytes()[:2] != b"MZ":
        raise ValueError("Not a Windows installer executable")
    output.mkdir(parents=True, exist_ok=True)
    for name, source in files.items():
        shutil.copy2(source, output / name)
    (output / "SHA256SUMS.txt").write_text("\n".join(checksums) + "\n")
    return [output / name for name in sorted([*files, "SHA256SUMS.txt"])]


def publish(tag: str, assets: list[Path], version: str) -> None:
    existing = subprocess.run(["gh", "release", "view", tag, "--json", "isDraft"],
                              text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if existing.returncode == 0 and not json.loads(existing.stdout)["isDraft"]:
        # A retried run must not replace binaries in an already public release.
        with tempfile.TemporaryDirectory(prefix="sad-pet-release-") as temporary:
            subprocess.run(["gh", "release", "download", tag, "--dir", temporary], check=True)
            for asset in assets:
                downloaded = Path(temporary) / asset.name
                if not downloaded.is_file() or downloaded.read_bytes() != asset.read_bytes():
                    raise ValueError(f"Published asset differs: {asset.name}")
        print(f"Release {tag} is already published with these assets")
        return
    notes = assets[0].parent / "release-notes.md"
    notes.write_text((PLUGIN / "RELEASE_NOTES.md").read_text().format(tag=tag, version=version))
    if existing.returncode != 0:
        subprocess.run(["gh", "release", "create", tag, "--verify-tag", "--draft",
                        "--title", f"SAD PET Plug-ins {version}", "--notes-file", str(notes)], check=True)
    subprocess.run(["gh", "release", "upload", tag, *map(str, assets), "--clobber"], check=True)
    subprocess.run(["gh", "release", "edit", tag, "--draft=false", "--latest",
                    "--notes-file", str(notes)], check=True)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("tag")
    parser.add_argument("--artifacts", type=Path, default=Path("dist/release"))
    parser.add_argument("--output", type=Path, default=Path("target/release-assets"))
    parser.add_argument("--check-tag", action="store_true")
    parser.add_argument("--check-only", action="store_true")
    args = parser.parse_args()
    version = validate_tag(args.tag)
    if not args.check_tag:
        assets = stage_assets(args.artifacts, args.output, version)
        print(f"Verified {len(assets)} release files for {args.tag}", flush=True)
        if not args.check_only:
            publish(args.tag, assets, version)
