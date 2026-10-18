#!/usr/bin/env python3
"""Validate both installed AUs, allowing macOS's asynchronous discovery to finish."""
import subprocess
import sys
import time
from pathlib import Path
import plistlib
import os
import pwd


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

# Headless CI shells can inherit a bootstrap namespace without the user's AU
# registry. Validate through the login user's namespace if plain auval cannot see
# the installed component. The installer payload and discovery cache are unchanged.
command = ["auval"]
probe = subprocess.run(command + ["-a"], capture_output=True, text=True)
if "SaPi" not in probe.stdout and os.environ.get("GITHUB_ACTIONS") == "true":
    command = ["sudo", "launchctl", "asuser", str(os.getuid()), "sudo", "-u",
               pwd.getpwuid(os.getuid()).pw_name, "/usr/bin/auval"]
    print("Validating in the login user's bootstrap namespace", flush=True)
    subprocess.run(command + ["-a"], check=False)

for kind, code in (("aumu", "SaPi"), ("aumf", "SaPf")):
    for attempt in range(10):
        result = subprocess.run(command + ["-v", kind, code, "SdPt"],
                                stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
        if result.returncode == 0:
            print(result.stdout, flush=True)
            break
        if "didn't find the component" not in result.stdout or attempt == 9:
            subprocess.run(command + ["-a"], check=False)
            if os.environ.get("GITHUB_ACTIONS") == "true":
                subprocess.run(["/usr/bin/log", "show", "--last", "2m", "--style", "compact",
                                "--predicate", 'process == "AudioComponentRegistrar"'], check=False)
            print(result.stdout, flush=True)
            sys.exit(result.returncode)
        print(f"Waiting for macOS to discover {code} ({attempt + 1}/10)", flush=True)
        subprocess.run(["killall", "AudioComponentRegistrar"], stdout=subprocess.DEVNULL,
                       stderr=subprocess.DEVNULL, check=False)
        time.sleep(2)
