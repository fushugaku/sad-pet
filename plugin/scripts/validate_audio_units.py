#!/usr/bin/env python3
"""Validate both installed AUs, allowing macOS's asynchronous discovery to finish."""
import subprocess
import sys
import time
from pathlib import Path
import plistlib
import os


print("AU validation user:", os.getuid(), "home:", Path.home(), flush=True)
folder = Path.home() / "Library/Audio/Plug-Ins/Components"
for parent in [Path.home(), *reversed(folder.relative_to(Path.home()).parents)]:
    if not parent.is_absolute():
        parent = Path.home() / parent
    subprocess.run(["ls", "-ld", str(parent)], check=False)
subprocess.run(["ls", "-ld", str(folder)], check=False)
for bundle in folder.glob("SAD PET*.component"):
    info = plistlib.loads((bundle / "Contents/Info.plist").read_bytes())
    print(bundle, info.get("AudioComponents"), flush=True)
    subprocess.run(["ls", "-ld", str(bundle), str(bundle / "Contents/MacOS" / info["CFBundleExecutable"])], check=False)

# Enumerate first: this triggers normal host discovery before validating IDs.
subprocess.run(["auval", "-a"], check=True, stdout=subprocess.DEVNULL)

for kind, code in (("aumu", "SaPi"), ("aumf", "SaPf")):
    for attempt in range(10):
        result = subprocess.run(["auval", "-v", kind, code, "SdPt"],
                                stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
        if result.returncode == 0:
            print(result.stdout, flush=True)
            break
        if "didn't find the component" not in result.stdout or attempt == 9:
            subprocess.run(["auval", "-a"], check=False)
            print(result.stdout, flush=True)
            sys.exit(result.returncode)
        print(f"Waiting for macOS to discover {code} ({attempt + 1}/10)", flush=True)
        subprocess.run(["killall", "AudioComponentRegistrar"], stdout=subprocess.DEVNULL,
                       stderr=subprocess.DEVNULL, check=False)
        time.sleep(2)
