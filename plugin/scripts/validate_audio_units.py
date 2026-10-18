#!/usr/bin/env python3
"""Validate both installed AUs, allowing macOS's asynchronous discovery to finish."""
import subprocess
import sys
import time


for kind, code in (("aumu", "SaPi"), ("aumf", "SaPf")):
    for attempt in range(10):
        result = subprocess.run(["auval", "-v", kind, code, "SdPt"],
                                stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
        if result.returncode == 0:
            print(result.stdout, flush=True)
            break
        if "didn't find the component" not in result.stdout or attempt == 9:
            print(result.stdout, flush=True)
            sys.exit(result.returncode)
        print(f"Waiting for macOS to discover {code} ({attempt + 1}/10)", flush=True)
        subprocess.run(["killall", "AudioComponentRegistrar"], stdout=subprocess.DEVNULL,
                       stderr=subprocess.DEVNULL, check=False)
        time.sleep(2)
