"""Profile-guided optimization (PGO) of the native engine archive.

Each target is built twice: instrumented, to run plugin/examples/pgo_training.rs
(the C ABI the JUCE wrapper calls, through plug-in, effect and app scenes and a
sweep of every DSP path), then with the merged profile. PGO only lays out code
and steers inlining; float arithmetic is unchanged, so the optimized engine
renders the same samples. The build checks that by running the trainer again on
the optimized engine and comparing its PCM hashes with the instrumented run.

Training runs on this machine: natively, or x86_64 macOS under Rosetta 2. A
target that cannot run here, a missing llvm-profdata, or one whose LLVM major
version differs from rustc's (the profile formats would not match) falls back
to a normal build with a warning. A profile is named after a fingerprint of the
toolchain, target, rustc flags and every native source file, so any change
trains a new one and a stale profile is never used.
"""
import hashlib
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile
import time

ROOT = Path(__file__).resolve().parents[2]
TRAINER = "pgo_training"
# Everything compiled into the engine, plus the training run and this recipe.
SOURCES = ["Cargo.toml", "Cargo.lock", "rust", "vendor", "plugin/Cargo.toml", "plugin/src",
           "plugin/examples", "plugin/scripts/pgo.py"]


class Unavailable(Exception):
    """This machine cannot train the target, which then builds without PGO."""


def warn(message):
    # GitHub Actions shows ::warning:: lines as annotations on the run.
    prefix = "::warning::" if os.environ.get("GITHUB_ACTIONS") else "warning: "
    print(f"{prefix}PGO: {message}", flush=True)


def rustc_info(rust_env):
    """rustc's `-vV` text, host triple and LLVM version."""
    text = subprocess.check_output([rust_env.get("RUSTC", "rustc"), "-vV"], text=True)
    fields = dict(line.split(": ", 1) for line in text.splitlines() if ": " in line)
    return text, fields["host"], fields["LLVM version"]


def llvm_major(text):
    found = re.search(r"LLVM version:? (\d+)\.", text)
    return int(found.group(1)) if found else None


def find_profdata(rust_env, host, major):
    """llvm-profdata from the same LLVM major version as rustc, and its version text."""
    rustc = rust_env.get("RUSTC", "rustc")
    sysroot = Path(subprocess.check_output([rustc, "--print", "sysroot"], text=True).strip())
    exe = ".exe" if os.name == "nt" else ""
    candidates = [
        os.environ.get("LLVM_PROFDATA"),
        # `rustup component add llvm-tools` ships rustc's own LLVM build.
        sysroot / "lib/rustlib" / host / "bin" / f"llvm-profdata{exe}",
        Path(f"/opt/homebrew/opt/llvm@{major}/bin/llvm-profdata"),
        Path(f"/usr/local/opt/llvm@{major}/bin/llvm-profdata"),
        shutil.which(f"llvm-profdata-{major}"),
        shutil.which("llvm-profdata"),
    ]
    for candidate in filter(None, candidates):
        try:
            version = subprocess.run([str(candidate), "--version"], capture_output=True, text=True,
                                     check=True).stdout
        except (OSError, subprocess.CalledProcessError):
            continue
        if llvm_major(version) == major:
            return Path(candidate), version
        print(f"PGO: ignoring {candidate}: LLVM {llvm_major(version)}, rustc uses LLVM {major}")
    return None, None


def runs_here(target, host):
    """Whether this machine can execute the target's training binary."""
    if target == host:
        return True
    if host == "aarch64-apple-darwin" and target == "x86_64-apple-darwin":
        # Apple silicon runs x86_64 binaries when Rosetta 2 is installed.
        return subprocess.run(["arch", "-x86_64", "/usr/bin/true"], capture_output=True).returncode == 0
    return False


def with_rustflags(env, *flags):
    """The environment with flags added to the user's own rustc flags.

    CARGO_ENCODED_RUSTFLAGS wins over RUSTFLAGS and keeps paths with spaces whole.
    """
    env = dict(env)
    if "CARGO_ENCODED_RUSTFLAGS" in env:
        current = [flag for flag in env["CARGO_ENCODED_RUSTFLAGS"].split("\x1f") if flag]
    else:
        current = env.get("RUSTFLAGS", "").split()
    env.pop("RUSTFLAGS", None)
    env["CARGO_ENCODED_RUSTFLAGS"] = "\x1f".join([*current, *flags])
    return env


def fingerprint(*settings, root=ROOT):
    """A short hash of the build settings and the content of every source file."""
    digest = hashlib.sha256()
    for setting in settings:
        digest.update(setting.encode() + b"\0")
    for name in SOURCES:
        path = root / name
        files = [path] if path.is_file() else sorted(path.rglob("*"))
        for file in files:
            relative = file.relative_to(root)
            # Cargo output and editor/OS files do not change what gets compiled.
            if not file.is_file() or any(p == "target" or p.startswith(".") for p in relative.parts):
                continue
            digest.update(relative.as_posix().encode() + b"\0")
            digest.update(hashlib.sha256(file.read_bytes()).digest())
    return digest.hexdigest()[:16]


def cargo_build(cargo, env, target, target_dir, *extra):
    """Runs Cargo and returns how many functions rustc found changed since the profile was made."""
    command = [str(cargo), "build", "-p", "sad-pet-plugin", "--release", "--locked", "--target", target,
               *extra]
    stale = 0
    with subprocess.Popen(command, cwd=ROOT, env={**env, "CARGO_TARGET_DIR": str(target_dir)},
                          stderr=subprocess.PIPE, text=True, errors="replace") as cargo_process:
        for line in cargo_process.stderr:
            sys.stderr.write(line)
            # LLVM drops the counters of a function whose control flow no longer matches.
            stale += "(hash mismatch)" in line
    if cargo_process.returncode:
        raise subprocess.CalledProcessError(cargo_process.returncode, command)
    return stale


def run_trainer(trainer):
    """Runs the training binary and returns its per-scene PCM hashes."""
    with tempfile.TemporaryDirectory(prefix="sad-pet-pgo-") as folder:
        # The app scenes never touch the user's library, presets or settings.
        env = {**os.environ, "SAD_PET_APP_DIR": folder, "SAD_PET_LIBRARY_DIR": folder,
               "SAD_PET_PRESET_DIR": folder}
        # The instrumented binary writes where -Cprofile-generate points, not here.
        env.pop("LLVM_PROFILE_FILE", None)
        started = time.monotonic()
        output = subprocess.run([str(trainer)], cwd=folder, env=env, stdout=subprocess.PIPE, text=True,
                                check=True).stdout
    print(output, end="")
    print(f"PGO: {trainer.name} ran in {time.monotonic() - started:.1f} s", flush=True)
    return [(line.split(":")[0], line.rsplit("hash=", 1)[1]) for line in output.splitlines() if "hash=" in line]


def train(cargo, rust_env, target, work, profdata, profile):
    """Builds and runs the instrumented trainer, then merges its counters into profile."""
    raw = work / "raw"
    shutil.rmtree(raw, ignore_errors=True)
    raw.mkdir(parents=True)
    env = with_rustflags(rust_env, f"-Cprofile-generate={raw}")
    try:
        cargo_build(cargo, env, target, work / "instrumented", "--example", TRAINER)
    except subprocess.CalledProcessError:
        raise Unavailable("the instrumented build failed (no profiler runtime in this toolchain?)")
    exe = ".exe" if "windows" in target else ""
    trainer = work / "instrumented" / target / "release/examples" / f"{TRAINER}{exe}"
    try:
        hashes = run_trainer(trainer)
    except OSError as error:
        raise Unavailable(f"cannot run the trainer here: {error}")
    counters = sorted(raw.glob("*.profraw"))
    if not counters:
        raise Unavailable("the trainer wrote no profile")
    merged = profile.with_suffix(".partial")
    try:
        subprocess.run([str(profdata), "merge", "-o", str(merged), *map(str, counters)], check=True)
    except subprocess.CalledProcessError:
        raise Unavailable(f"{profdata} could not read the counters")
    merged.replace(profile)
    return hashes


def profile_guided(build, cargo, rust_env, target=None, targets="all"):
    """Builds the profile-guided engine for target (default: the host) and returns
    its archive. None means a normal build: target is not in the comma-separated
    targets, or this machine cannot train it (after a warning)."""
    info, host, llvm = rustc_info(rust_env)
    target = target or host
    if targets != "all" and target not in targets.split(","):
        print(f"PGO: not requested for {target}", flush=True)
        return None
    try:
        return optimized_engine(build, cargo, rust_env, target, info, host, llvm)
    except Unavailable as reason:
        warn(f"{reason}; building {target} without PGO")
        return None


def optimized_engine(build, cargo, rust_env, target, info, host, llvm):
    """Trains or reuses the target's profile, builds the engine with it and checks its output."""
    if not runs_here(target, host):
        raise Unavailable(f"this {host} machine cannot run {target} binaries")
    major = llvm_major(f"LLVM version {llvm}")
    profdata, version = find_profdata(rust_env, host, major)
    if not profdata:
        raise Unavailable(f"no llvm-profdata for LLVM {major} (run `rustup component add llvm-tools` "
                          "or set LLVM_PROFDATA)")
    work = build / "pgo" / target
    work.mkdir(parents=True, exist_ok=True)
    flags = with_rustflags(rust_env)["CARGO_ENCODED_RUSTFLAGS"]
    key = fingerprint(info, version, target, flags)
    profile = work / f"engine-{key}.profdata"
    optimized = work / "optimized"
    hashes_file = profile.with_suffix(".hashes")
    if profile.is_file() and hashes_file.is_file():
        print(f"PGO: {profile.name} matches the sources; reusing it for {target}", flush=True)
        hashes = [tuple(line.split()) for line in hashes_file.read_text().splitlines()]
    else:
        # A new profile rebuilds every crate anyway; drop the old ones' output.
        for old in work.glob("engine-*"):
            old.unlink()
        shutil.rmtree(optimized, ignore_errors=True)
        started = time.monotonic()
        hashes = train(cargo, rust_env, target, work, profdata, profile)
        hashes_file.write_text("".join(f"{name} {value}\n" for name, value in hashes))
        print(f"PGO: trained {profile.name} for {target} in {time.monotonic() - started:.0f} s", flush=True)
    # The profile name is part of the flags, so Cargo rebuilds whenever it changes.
    env = with_rustflags(rust_env, f"-Cprofile-use={profile}")
    stale = cargo_build(cargo, env, target, optimized, "--lib", "--example", TRAINER)
    if stale:
        # The fingerprint should rule this out; the build still renders the same samples.
        warn(f"{stale} {target} functions changed after training and were optimized without their profile")
    exe = ".exe" if "windows" in target else ""
    rendered = run_trainer(optimized / target / "release/examples" / f"{TRAINER}{exe}")
    changed = [name for (name, before), (_, after) in zip(hashes, rendered) if before != after]
    if len(rendered) != len(hashes) or changed:
        sys.exit(f"PGO changed the engine's output for {target}: {', '.join(changed) or 'scene list'}")
    print(f"PGO: {target} renders {len(hashes)} training scenes bit-identically", flush=True)
    library = "sad_pet_plugin.lib" if "msvc" in target else "libsad_pet_plugin.a"
    return optimized / target / "release" / library
