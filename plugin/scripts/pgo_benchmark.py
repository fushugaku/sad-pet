#!/usr/bin/env python3
"""A/B benchmark of the profile-guided engine against a normal build of the same sources.

Trains this machine's profile with pgo.py, builds engine_benchmark
(rust/examples/audio_benchmark.rs in the plug-in's crate graph) and the
pgo_training scenes with and without it, captures reference renders with the
normal build, then runs both builds in alternating order for several rounds.
Each case keeps its fastest round (median block time for a benchmark case, mean
block time for a training scene); the summary is the geometric mean of the
PGO/normal ratios. Fails if the profile-guided engine changes any sample or
training hash.
  python3 plugin/scripts/pgo_benchmark.py --rounds 5
  python3 plugin/scripts/pgo_benchmark.py --target x86_64-apple-darwin  # under Rosetta
"""
import argparse
import json
import math
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile
import time

from build import ROOT, rust_toolchain
from pgo import TRAINER, cargo_build, profile_guided, rustc_info, warn, with_rustflags

BENCHMARK = "engine_benchmark"
# "44100-11: samples=… median_us=…" and "resynth: rate=… block_us=… hash=…"
RESULT = re.compile(r"^(?P<case>[\w-]+): .*?\b(?:median_us|block_us)=(?P<us>[\d.]+)(?:.* hash=(?P<hash>\S+))?")


def run(name, binary, *args):
    """Runs one build's benchmark binary and returns {case: (microseconds, PCM hash or None)}."""
    with tempfile.TemporaryDirectory(prefix="sad-pet-pgo-benchmark-") as folder:
        # The trainer's app scenes never touch the user's library, presets or settings.
        env = {**os.environ, "SAD_PET_APP_DIR": folder, "SAD_PET_LIBRARY_DIR": folder,
               "SAD_PET_PRESET_DIR": folder}
        started = time.monotonic()
        process = subprocess.run([str(binary), *map(str, args)], cwd=folder, env=env, stdout=subprocess.PIPE,
                                 text=True)
    if process.returncode:
        # `compare` panics on the first case whose samples differ from the references.
        sys.exit(f"{name} {binary.name} {' '.join(map(str, args))} failed with exit code {process.returncode}")
    results = {}
    for line in process.stdout.splitlines():
        if found := RESULT.match(line):
            results[found["case"]] = float(found["us"]), found["hash"]
    print(f"  {name} {binary.stem}: {len(results)} cases in {time.monotonic() - started:.1f} s", flush=True)
    return results


def table(rounds, programs):
    """Markdown rows of each case's fastest round and the geometric mean per program."""
    rows = ["| Case | Normal µs | PGO µs | Change |", "| --- | ---: | ---: | ---: |"]
    for program in programs:
        logs = []
        for case, normal in rounds["normal"][program].items():
            before, after = min(normal), min(rounds["pgo"][program][case])
            logs.append(math.log(after / before))
            rows.append(f"| {program} {case} | {before:.2f} | {after:.2f} | {after / before - 1:+.1%} |")
        mean = math.exp(sum(logs) / len(logs))
        rows.append(f"| **{program} geomean** ({len(logs)} cases) | | | **{mean - 1:+.1%}** |")
    return "\n".join(rows)


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--build-dir", type=Path, default=ROOT / "target/pgo-benchmark")
    parser.add_argument("--rounds", type=int, default=5, help="Runs of each build (default 5)")
    parser.add_argument("--target", help="Another target this machine runs, e.g. x86_64-apple-darwin under Rosetta")
    args = parser.parse_args()
    if args.rounds < 1:
        parser.error("--rounds must be at least 1")
    build = args.build_dir.resolve()
    cargo, rust_env = rust_toolchain()
    target = args.target or rustc_info(rust_env)[1]
    # Trains (or reuses) the profile and checks the optimized trainer's hashes.
    if not profile_guided(build, cargo, rust_env, target):
        sys.exit(f"PGO is unavailable for {target}, so there is nothing to compare")
    work = build / "pgo" / target
    (profile,) = work.glob("engine-*.profdata")
    # --lib as in pgo.py, so the profile-guided library and trainer are reused as built.
    units = ("--lib", "--example", BENCHMARK, "--example", TRAINER)
    cargo_build(cargo, rust_env, target, build / "normal", *units)
    stale = cargo_build(cargo, with_rustflags(rust_env, f"-Cprofile-use={profile}"), target, work / "optimized",
                        *units)
    if stale:
        warn(f"{stale} {target} functions were optimized without their profile")
    exe = ".exe" if "windows" in target else ""
    folders = {"normal": build / "normal", "pgo": work / "optimized"}
    binaries = {name: folder / target / "release/examples" for name, folder in folders.items()}
    references = build / "references"
    run("normal", binaries["normal"] / f"{BENCHMARK}{exe}", "capture", references)

    programs = {BENCHMARK: ("compare", references), TRAINER: ()}
    rounds = {name: {program: {} for program in programs} for name in binaries}
    hashes, changed = {}, set()
    for index in range(args.rounds):
        print(f"Round {index + 1}/{args.rounds}", flush=True)
        # Alternating which build goes first keeps a warming or throttling machine from favouring one.
        for name in list(binaries)[:: 1 if index % 2 == 0 else -1]:
            for program, arguments in programs.items():
                for case, (us, value) in run(name, binaries[name] / f"{program}{exe}", *arguments).items():
                    rounds[name][program].setdefault(case, []).append(us)
                    if value and value != "timing-dependent" and hashes.setdefault(case, value) != value:
                        changed.add(case)

    report = table(rounds, programs)
    title = f"PGO benchmark, {target}, fastest of {args.rounds} rounds"
    print(f"\n{title}\n{report}")
    (build / "results.md").write_text(f"## {title}\n\n{report}\n", encoding="utf-8")
    (build / "results.json").write_text(json.dumps({"target": target, "rounds": rounds}, indent=1))
    if summary := os.environ.get("GITHUB_STEP_SUMMARY"):
        with open(summary, "a", encoding="utf-8") as file:
            file.write(f"## {title}\n\n{report}\n")
    if changed:
        sys.exit(f"Training scenes rendered differently between builds or rounds: {', '.join(sorted(changed))}")


if __name__ == "__main__":
    main()
