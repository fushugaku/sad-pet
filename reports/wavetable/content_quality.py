#!/usr/bin/env python3
"""Measure authored content, not lookup quality or subjective musical merit.

The low band is capped at harmonic 63: complexity only above that band is
not useful for a typical mid-keyboard note. No third-party audio is used.
"""
import gzip
import hashlib
import json
from pathlib import Path

import numpy as np

ROOT = Path(__file__).resolve().parents[2]
ASSETS = ROOT / "rust/assets/wavetables"
manifest_bytes = (ASSETS / "manifest.json").read_bytes()
manifest = json.loads(manifest_bytes)
original = json.loads((ROOT / "rust/tests/fixtures/wavetable/factory_v1_before.json").read_text())
by_id = {t["id"]: t for p in manifest["packs"] for t in p["tables"]}
assert all(all(by_id[t["id"]][k] == v for k, v in t.items()) for t in original), "Original factory identity changed"
rows = []
for pack in manifest["packs"]:
    for table in pack["tables"]:
        pcm = np.frombuffer(gzip.decompress((ASSETS / table["file"]).read_bytes()), dtype="<i2")
        samples = pcm.astype(np.float64).reshape(table["frames"], 2048) / 32768
        samples = (samples - samples.mean(axis=1, keepdims=True)).astype("<f4")
        header = b"SAD-WT1\0" + (2048).to_bytes(4, "little") + table["frames"].to_bytes(4, "little")
        assert hashlib.sha256(header + samples.tobytes()).hexdigest() == table["content_hash"]
        waves = samples.astype(np.float64)
        spectrum = np.fft.rfft(waves)[:, 1:64]
        powers = np.abs(spectrum)**2
        energy = powers.sum(axis=1)
        strong = (powers > powers.max(axis=1, keepdims=True) * 1e-4).sum(axis=1)
        effective = energy**2 / np.maximum((powers**2).sum(axis=1), 1e-30)
        rms = np.sqrt(np.mean(waves**2, axis=1))
        # Reconstruction RMS of h=1..63, with FFT scale accounted for.
        low_rms = np.sqrt(2 * energy) / 2048
        spectra = np.abs(spectrum)
        spectra /= np.maximum(np.linalg.norm(spectra, axis=1, keepdims=True), 1e-30)
        checkpoints = spectra[[0, 16, 32, 48, 63]]
        distances = [1 - float(a @ b) for i, a in enumerate(checkpoints) for b in checkpoints[i+1:]]
        mid = (waves[:-1] + waves[1:]) * .5
        mid_ratio = np.sqrt(np.mean(mid**2, axis=1)) / np.sqrt((rms[:-1]**2 + rms[1:]**2) * .5)
        complex_table = table["recipe"].startswith("complex:")
        row = dict(id=table["id"], name=table["name"], pack=pack["name"], complex=complex_table,
                   unique_frames=len({hashlib.sha256(w.tobytes()).digest() for w in samples}),
                   dc_max=float(np.abs(waves.mean(axis=1)).max()), peak=float(np.abs(waves).max()),
                   rms_min=float(rms.min()), rms_max=float(rms.max()),
                   low_band_rms_min=float(low_rms.min()),
                   low_band_strong_harmonics_min=int(strong.min()),
                   low_band_effective_harmonics_median=float(np.median(effective)),
                   spectral_distance_mean=float(np.mean(distances)),
                   adjacent_midpoint_rms_ratio_min=float(mid_ratio.min()))
        row["passed"] = row["dc_max"] <= 1e-7 and row["peak"] <= .961 and row["unique_frames"] >= (60 if complex_table else 2)
        if complex_table:
            row["passed"] &= (row["low_band_strong_harmonics_min"] >= 8
                              and row["low_band_effective_harmonics_median"] >= 4
                              and row["low_band_rms_min"] >= .06
                              and row["spectral_distance_mean"] >= .03
                              and row["adjacent_midpoint_rms_ratio_min"] >= .65)
        rows.append(row)
report = dict(scope="Authored content; independent FFT statistics, not listening or runtime anti-alias qualification",
              manifest_sha256=hashlib.sha256(manifest_bytes).hexdigest(), numpy=np.__version__,
              tables=len(rows), packs=len(manifest["packs"]), new_complex=sum(r["complex"] for r in rows),
              original_32_unchanged=True, complex_band_harmonics=63,
              criteria=dict(dc_max=1e-7, peak_max=.961, original_unique_frames_min=2, complex_unique_frames_min=60,
                            complex_strong_harmonics_min=8, complex_effective_harmonics_median_min=4,
                            complex_low_band_rms_min=.06, complex_spectral_distance_mean_min=.03,
                            complex_adjacent_midpoint_rms_ratio_min=.65),
              passed=all(r["passed"] for r in rows), rows=rows)
destination = Path(__file__).with_name("content-quality.json")
destination.write_text(json.dumps(report, indent=2) + "\n")
failed = [r for r in rows if not r["passed"]]
print(json.dumps(dict(tables=report["tables"], new_complex=report["new_complex"], passed=report["passed"], failed=failed), indent=2))
if failed:
    raise SystemExit(1)
