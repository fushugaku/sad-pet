#!/usr/bin/env python3
"""Compare sample-aligned wavetable_opt capture directories; no fitting."""
import argparse
from array import array
import hashlib
import json
import math
from pathlib import Path
import sys

p = argparse.ArgumentParser(description=__doc__)
p.add_argument("before", type=Path)
p.add_argument("after", type=Path)
p.add_argument("report", type=Path)
a = p.parse_args()
rows = []
for case in json.loads((a.before / "manifest.json").read_text()):
    content = [(folder / case["file"]).read_bytes() for folder in [a.before, a.after]]
    waves = []
    for data in content:
        samples = array("f")
        samples.frombytes(data)
        if sys.byteorder != "little":
            samples.byteswap()
        waves.append(samples)
    before, after = waves
    assert len(before) == len(after) == case["frames"] * 2
    assert all(math.isfinite(x) for x in after)
    energy = sum(x*x for x in before)
    error = sum((x-y)**2 for x, y in zip(before, after))
    dbr = 10*math.log10(max(error, 1e-60) / max(energy, 1e-60))
    maximum = max(abs(x-y) for x, y in zip(before, after))
    rows.append({**case, "reference_sha256": hashlib.sha256(content[0]).hexdigest(),
                 "after_sha256": hashlib.sha256(content[1]).hexdigest(),
                 "rms": math.sqrt(energy/len(before)), "error_rms": math.sqrt(error/len(before)),
                 "error_dbr": dbr, "max_abs_error": maximum,
                 "bit_identical": content[0] == content[1],
                 "passed": maximum <= 1e-5 and dbr <= -95})
report = dict(scope="30 sample-aligned dynamic native production-Engine before/after renders. No alignment/gain fitting. Four complex waves, U4 then U2/U8, moving positions/Voice shift/index/sync ratio, notes and mode edits. 44.1/48/96 kHz, 1/8 notes. Numerical regression; separate from listening, independent dynamic alias tests and new routing.",
              frames_per_case=12288, criterion_max_abs_error=1e-5, criterion_error_dbr=-95,
              worst_error_dbr=max(r["error_dbr"] for r in rows),
              worst_max_abs_error=max(r["max_abs_error"] for r in rows),
              bit_identical_cases=sum(r["bit_identical"] for r in rows),
              passed=all(r["passed"] for r in rows), results=rows)
a.report.write_text(json.dumps(report, indent=2) + "\n")
print({k:v for k,v in report.items() if k != "results"})
assert report["passed"]
