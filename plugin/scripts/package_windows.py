#!/usr/bin/env python3
"""Compile the Windows installer from the staged x64 VST3 bundles."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import urllib.request


SCRIPTS = Path(__file__).resolve().parent
ROOT = SCRIPTS.parents[1]
PREREQUISITES = {
    "MicrosoftEdgeWebview2Setup.exe": "https://go.microsoft.com/fwlink/p/?LinkId=2124703",
    "vc_redist.x64.exe": "https://aka.ms/vc14/vc_redist.x64.exe",
}


def build_exe(source: Path, compiler: Path | None = None) -> Path:
    source = source.resolve()
    versions = set()
    for name in ("SAD PET", "SAD PET FX"):
        bundle = source / "VST3" / f"{name}.vst3" / "Contents"
        # Steinberg's generated moduleinfo allows trailing commas.
        version = re.search(r'"Version"\s*:\s*"(\d+\.\d+\.\d+)"',
                            (bundle / "Resources/moduleinfo.json").read_text())
        if not version:
            raise ValueError(f"No plug-in version in {bundle}")
        versions.add(version.group(1))
        executable = bundle / "x86_64-win" / f"{name}.vst3"
        if executable.read_bytes()[:2] != b"MZ":
            raise ValueError(f"Not a Windows plug-in: {executable}")
    if len(versions) != 1:
        raise ValueError("Both Windows plug-ins must have the same version")
    version = versions.pop()
    if compiler is None:
        found = shutil.which("ISCC.exe")
        compiler = Path(found) if found else Path(os.environ["ProgramFiles(x86)"]) / "Inno Setup 6/ISCC.exe"
    if not compiler.is_file():
        raise FileNotFoundError("Install Inno Setup 6, or pass --compiler /path/to/ISCC.exe")

    dependencies = ROOT / "target/plugin-deps/WindowsInstaller"
    dependencies.mkdir(parents=True, exist_ok=True)
    notices = {}
    for name, url in PREREQUISITES.items():
        path = dependencies / name
        if not path.is_file():
            temporary = path.with_suffix(".download")
            with urllib.request.urlopen(url, timeout=120) as response, temporary.open("wb") as output:
                shutil.copyfileobj(response, output)
            temporary.replace(path)
        # Only redistribute the original Microsoft-signed prerequisite installers.
        info = json.loads(subprocess.check_output(
            ["powershell", "-NoProfile", "-NonInteractive", "-File",
             str(SCRIPTS / "windows_prerequisite.ps1"), str(path)], text=True))
        notices[name] = {"url": url, "sha256": hashlib.sha256(path.read_bytes()).hexdigest(), **info}
    runtime_version = re.search(r"\d+\.\d+\.\d+\.\d+", notices["vc_redist.x64.exe"]["version"])
    if not runtime_version:
        raise ValueError("Cannot read the Visual C++ runtime version")
    (source / "Licenses/Microsoft-runtime-packages.json").write_text(
        json.dumps(notices, indent=2) + "\n", encoding="utf-8")
    subprocess.run([
        str(compiler), f"/DSourceDir={source}", f"/DPluginVersion={version}",
        f"/DOutputDir={source.parent}", f"/DPrerequisitesDir={dependencies}",
        f"/DVCVersion={runtime_version.group(0)}", str(SCRIPTS / "windows.iss"),
    ], check=True)
    installer = source.parent / "SAD-PET-Plugins-Windows-x64-Setup.exe"
    checksum = hashlib.sha256(installer.read_bytes()).hexdigest()
    installer.with_suffix(".exe.sha256").write_text(f"{checksum}  {installer.name}\n")
    print(f"Packaged {installer}")
    return installer


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source", type=Path)
    parser.add_argument("--compiler", type=Path)
    args = parser.parse_args()
    build_exe(args.source, args.compiler)
