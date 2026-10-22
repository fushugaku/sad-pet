import os
from pathlib import Path
import tempfile
import unittest

from pgo import fingerprint, llvm_major, profile_guided, runs_here, with_rustflags
from pgo_benchmark import RESULT, table


class ProfileGuidedBuild(unittest.TestCase):
    def test_flags_keep_the_users_own_and_paths_with_spaces(self):
        env = with_rustflags({"RUSTFLAGS": "-Ctarget-cpu=native  -Dwarnings"}, "-Cprofile-use=/a b/x.profdata")
        self.assertNotIn("RUSTFLAGS", env)
        self.assertEqual(env["CARGO_ENCODED_RUSTFLAGS"].split("\x1f"),
                         ["-Ctarget-cpu=native", "-Dwarnings", "-Cprofile-use=/a b/x.profdata"])
        # Cargo ignores RUSTFLAGS when the encoded form is set.
        env = with_rustflags({"CARGO_ENCODED_RUSTFLAGS": "-Ca\x1f-Cb", "RUSTFLAGS": "-Cignored"}, "-Cc")
        self.assertEqual(env["CARGO_ENCODED_RUSTFLAGS"], "-Ca\x1f-Cb\x1f-Cc")
        self.assertEqual(with_rustflags({})["CARGO_ENCODED_RUSTFLAGS"], "")

    def test_any_source_or_toolchain_change_needs_a_new_profile(self):
        with tempfile.TemporaryDirectory() as folder:
            root = Path(folder)
            source = root / "rust/src/lib.rs"
            source.parent.mkdir(parents=True)
            source.write_text("fn a() {}")
            key = fingerprint("rustc 1.97.1", "aarch64-apple-darwin", root=root)
            (root / "rust/target").mkdir()
            (root / "rust/target/lib.o").write_text("build output")
            (root / "rust/.DS_Store").write_text("")
            self.assertEqual(fingerprint("rustc 1.97.1", "aarch64-apple-darwin", root=root), key)
            self.assertNotEqual(fingerprint("rustc 1.97.1", "x86_64-apple-darwin", root=root), key)
            self.assertNotEqual(fingerprint("rustc 1.98.0", "aarch64-apple-darwin", root=root), key)
            source.write_text("fn b() {}")
            self.assertNotEqual(fingerprint("rustc 1.97.1", "aarch64-apple-darwin", root=root), key)

    def test_llvm_versions_compare_by_major(self):
        self.assertEqual(llvm_major("LLVM version: 22.1.6"), 22)
        self.assertEqual(llvm_major("Homebrew LLVM version 22.1.8\n  Optimized build."), 22)
        self.assertEqual(llvm_major("LLVM (http://llvm.org/):\n  LLVM version 22.1.6-rust-1.97.1-stable"), 22)
        self.assertIsNone(llvm_major("llvm-profdata: unknown"))

    def test_unlisted_targets_build_normally(self):
        with tempfile.TemporaryDirectory() as folder:
            archive = profile_guided(Path(folder), "cargo", dict(os.environ), "x86_64-apple-darwin",
                                     "aarch64-apple-darwin,x86_64-pc-windows-msvc")
            self.assertIsNone(archive)
            self.assertEqual(list(Path(folder).iterdir()), [])

    def test_training_needs_a_runnable_target(self):
        self.assertTrue(runs_here("x86_64-pc-windows-msvc", "x86_64-pc-windows-msvc"))
        self.assertFalse(runs_here("aarch64-apple-darwin", "x86_64-apple-darwin"))
        self.assertFalse(runs_here("aarch64-pc-windows-msvc", "x86_64-pc-windows-msvc"))

    def test_benchmark_keeps_each_cases_fastest_round(self):
        found = [RESULT.match(line) for line in (
            "44100-11: samples=8 median_us=100.50 p99_us=1.00 max_us=2.00 compare",
            "source_fx: rate=48000 blocks=9 block_us=50.00 hash=timing-dependent",
            "PGO: engine_benchmark ran in 1.0 s")]
        self.assertEqual([(match["case"], match["us"], match["hash"]) for match in found[:2]],
                         [("44100-11", "100.50", None), ("source_fx", "50.00", "timing-dependent")])
        self.assertIsNone(found[2])
        rounds = {"normal": {"bench": {"a": [120.0, 100.0], "b": [10.0]}},
                  "pgo": {"bench": {"a": [90.0, 81.0], "b": [10.0]}}}
        report = table(rounds, ["bench"])
        self.assertIn("| bench a | 100.00 | 81.00 | -19.0% |", report)
        self.assertIn("| bench b | 10.00 | 10.00 | +0.0% |", report)
        # Geometric mean of 0.81 and 1.0.
        self.assertIn("| **bench geomean** (2 cases) | | | **-10.0%** |", report)


if __name__ == "__main__":
    unittest.main()
