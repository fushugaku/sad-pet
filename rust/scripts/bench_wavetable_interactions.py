#!/usr/bin/env python3
"""Alternate identical before/after production-Engine CPU workloads."""
import argparse
import datetime
import hashlib
import json
from pathlib import Path
import platform
import statistics
import subprocess

p = argparse.ArgumentParser(description=__doc__)
p.add_argument("before", type=Path)
p.add_argument("after", type=Path)
p.add_argument("output", type=Path)
p.add_argument("--seconds", type=float, default=2)
p.add_argument("--rounds", type=int, default=3)
p.add_argument("--focused", action="store_true", help="Recheck representative profiles after control-path fixes")
a = p.parse_args()
cases = [("off", 8, 4), ("fm", 4, 4), ("fm", 8, 4), ("fm", 16, 2),
         ("fm", 8, 8), ("pm", 8, 4), ("sync", 4, 4), ("sync", 8, 4),
         ("mixed", 4, 4), ("mixed", 8, 4)]
if a.focused:
    cases = [("fm", 4, 4), ("sync", 4, 4), ("mixed", 4, 2)]
runs = []
for index, (mode, notes, unison) in enumerate(cases):
    for repeat in range(a.rounds):
        order = [("before", a.before), ("after", a.after)]
        if (repeat + index) % 2:
            order.reverse()
        for version, binary in order:
            row = json.loads(subprocess.check_output(
                [str(binary.resolve()), "cpu", mode, str(notes), str(unison),
                 str(a.seconds), "128", "48000"], text=True))
            row.update(version=version, repeat=repeat)
            runs.append(row)
    print(f"{mode}: {notes} notes × 4 waves × U{unison}", flush=True)

summary = []
for mode, notes, unison in cases:
    row = dict(mode=mode, notes=notes, waves=4, unison=unison)
    for version in ["before", "after"]:
        selected = [r for r in runs if (r["mode"], r["notes"], r["unison"], r["version"]) == (mode, notes, unison, version)]
        row[version] = {clock: {key: statistics.median(r[clock][key] for r in selected)
                       for key in selected[0][clock]} for clock in ["thread_cpu", "wall"]}
    row["thread_cpu_p50_reduction_percent"] = 100 * (1 - row["after"]["thread_cpu"]["p50_ms"] / row["before"]["thread_cpu"]["p50_ms"])
    row["wall_p99_fits_deadline"] = row["after"]["wall"]["p99_ms"] < 128 / 48
    summary.append(row)

routed = []
for mode in (["routed-mixed"] if a.focused else ["routed-fm", "routed-pm", "routed-sync", "routed-mixed"]):
    for notes in ([4] if a.focused else [4, 8]):
        routed.append(json.loads(subprocess.check_output(
            [str(a.after.resolve()), "cpu", mode, str(notes), "2" if a.focused else "4", str(a.seconds), "128", "48000"], text=True)))
    print(mode, flush=True)

# A larger host block averages brief timing variation without changing DSP.
# WebAudio's fixed 128-frame callback is still reported separately above.
for mode in ([] if a.focused else ["routed-sync", "routed-mixed"]):
    routed.append(json.loads(subprocess.check_output(
        [str(a.after.resolve()), "cpu", mode, "4", "4", str(a.seconds), "256", "48000"], text=True)))

report = dict(created_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
              machine=platform.platform(), processor=subprocess.check_output(["sysctl", "-n", "machdep.cpu.brand_string"], text=True).strip() if platform.system() == "Darwin" else platform.processor(),
              binaries={name: {"sha256": hashlib.sha256(path.read_bytes()).hexdigest()}
                        for name, path in [("before", a.before), ("after", a.after)]},
              method="Sequential alternating AB/BA, no concurrent benchmark jobs. Median of each run's block percentiles, not a percentile over pooled blocks. Compilation and asset preparation excluded. Thread CPU excludes descheduling; wall includes other local work. Frequency/thermal contention is not isolated.",
              rounds=a.rounds, audio_seconds_per_run=a.seconds, sample_rate=48000, block=128, deadline_ms=128/48,
              scope="Shared production Engine, four distinct complex tables, unison detune and Voice shift, no effects. Legacy modes compare identical patches; routed modes are new features and have no before equivalent. In-process timing does not establish device/DAW underrun guarantees.",
              summary=summary, runs=runs, routed=routed)
a.output.parent.mkdir(parents=True, exist_ok=True)
a.output.write_text(json.dumps(report, indent=2) + "\n")
print(a.output, flush=True)
