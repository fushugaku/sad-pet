#!/usr/bin/env python3
"""Isolated spectral experiment, not the SAD PET engine or a CPU benchmark.

Requires Python 3 and NumPy. Writes reproducible numeric results beside this
file. No audio devices, network, project settings or production DSP are used.
"""

import json
import platform
from pathlib import Path

import numpy as np

RATE = 48_000
COUNT = 131_072
MAX_HARMONIC = 1023
PHASE_OFFSET = 0.123456789
FREQUENCIES = (27.5, 55.0, 110.0, 440.0, 1760.0, 7040.0)


def coefficients(length, harmonics, kind, phase=0.0, bin_step=1):
    spectrum = np.zeros(length // 2 + 1, dtype=np.complex128)
    selected = np.arange(1, harmonics + 1) if kind == "saw" else np.array([harmonics])
    amplitude = 1.0 / selected if kind == "saw" else np.ones(len(selected))
    spectrum[selected * bin_step] = -0.5j * length * amplitude * np.exp(
        2j * np.pi * selected * phase
    )
    return np.fft.irfft(spectrum, n=length), selected


def lookup(table, phase, method):
    position = phase * len(table)
    index = np.floor(position).astype(np.int64)
    t = position - index
    read = lambda offset: table[(index + offset) % len(table)].astype(np.float64)
    if method == "linear":
        result = read(0) * (1.0 - t) + read(1) * t
    elif method == "lagrange3":
        result = (
            read(-1) * (-t * (t - 1.0) * (t - 2.0) / 6.0)
            + read(0) * ((t + 1.0) * (t - 1.0) * (t - 2.0) / 2.0)
            - read(1) * ((t + 1.0) * t * (t - 2.0) / 2.0)
            + read(2) * ((t + 1.0) * t * (t - 1.0) / 6.0)
        )
    else:
        taps = int(method.removeprefix("sinc"))
        radius = taps / 2.0
        total = np.zeros_like(t)
        weights = np.zeros_like(t)
        for offset in range(1 - taps // 2, 1 + taps // 2):
            distance = t - offset
            # Continuous Blackman window, shifted with the fractional phase.
            window = (
                0.42
                + 0.5 * np.cos(np.pi * distance / radius)
                + 0.08 * np.cos(2.0 * np.pi * distance / radius)
            )
            weight = np.sinc(distance) * window
            total += read(offset) * weight
            weights += weight
        result = total / weights
    return result.astype(np.float32).astype(np.float64)


def db(value):
    return float(20.0 * np.log10(max(float(value), 1e-15)))


def main():
    rows = []
    # The first is deliberately unfiltered; the other cases remove harmonics
    # off-thread, as a future prepared wavetable mip level would.
    methods = (
        ("naive_linear_2048", "linear", "naive", 0),
        ("bandlimited_linear_2048", "linear", "fixed", 0),
        ("bandlimited_lagrange3_2048", "lagrange3", "fixed", 0),
        ("compact4_lagrange3", "lagrange3", "compact", 4),
        ("compact8_lagrange3", "lagrange3", "compact", 8),
        ("compact4_sinc8", "sinc8", "compact", 4),
        ("compact4_sinc16", "sinc16", "compact", 4),
        ("compact8_sinc8", "sinc8", "compact", 8),
    )
    for requested in FREQUENCIES:
        bin_step = max(1, round(requested * COUNT / RATE))
        if bin_step % 2 == 0:
            bin_step += 1  # Coprime to the power-of-two record length.
        frequency = bin_step * RATE / COUNT
        phase = (np.arange(COUNT) * bin_step / COUNT + PHASE_OFFSET) % 1.0
        safe = min(MAX_HARMONIC, int(0.45 * RATE / frequency))
        harmonic_limit = max(1, 2 ** int(np.floor(np.log2(safe + 1))) - 1)
        nyquist_limit = min(MAX_HARMONIC, (COUNT // 2 - 1) // bin_step)
        for kind in ("saw", "highest_partial"):
            for name, interpolation, layout, density in methods:
                harmonics = MAX_HARMONIC if layout == "naive" else harmonic_limit
                table_length = max(64, density * (harmonics + 1)) if layout == "compact" else 2048
                table, _ = coefficients(table_length, harmonics, kind)
                table = table.astype(np.float32)
                signal = lookup(table, phase, interpolation)
                reference_harmonics = min(harmonics, nyquist_limit)
                if layout == "naive" and kind == "highest_partial":
                    # Above-Nyquist single partial: compare to silence, normalize
                    # to its original sine RMS. It should disappear after an
                    # ideal anti-alias filter, not be replaced by a lower partial.
                    reference = np.zeros(COUNT)
                    selected = np.array([], dtype=np.int64)
                    reference_rms = np.sqrt(0.5)
                    reference_harmonics = 0
                else:
                    reference, selected = coefficients(
                        COUNT, reference_harmonics, kind, PHASE_OFFSET, bin_step
                    )
                    reference_rms = np.sqrt(np.mean(reference**2))
                residual = np.sqrt(np.mean((signal - reference) ** 2)) / reference_rms
                spectrum = np.fft.rfft(signal)
                model_bins = np.zeros(len(spectrum), dtype=bool)
                model_bins[selected * bin_step] = True
                power = np.abs(spectrum) ** 2
                power[1:-1] *= 2.0
                spurious = np.sqrt(power[~model_bins].sum()) / COUNT / reference_rms
                rows.append({
                    "requested_hz": requested,
                    "actual_hz": frequency,
                    "kind": kind,
                    "method": name,
                    "table_samples": table_length,
                    "table_harmonics": harmonics,
                    "reference_harmonics": reference_harmonics,
                    "spurious_db_relative_to_reference_rms": db(spurious),
                    "total_error_db_relative_to_reference_rms": db(residual),
                })
    lengths4 = [max(64, 4 * (2**k)) for k in range(10, 0, -1)]
    lengths8 = [max(64, 8 * (2**k)) for k in range(10, 0, -1)]
    result = {
        "scope": "Static isolated oscillator reconstruction; no rack, scan, mip transitions, CPU or worklet measurements",
        "python": platform.python_version(),
        "numpy": np.__version__,
        "sample_rate": RATE,
        "record_samples": COUNT,
        "table_dtype": "float32",
        "lookup_arithmetic": "float64, rounded to float32 output",
        "phase_offset_cycles": PHASE_OFFSET,
        "bandlimit_guard": 0.45,
        "notes": [
            "Spurious means energy outside the reference Fourier bins; total error also includes passband error.",
            "Each bandlimited case uses the same harmonic cap at a given pitch; caps change by pitch.",
            "Naive cases retain 1023 source harmonics and therefore have a different intended bandwidth.",
            "highest_partial is an adversarial harmonic, not a loudness-matched commercial wavetable.",
            "These are algorithm feasibility measurements, not production quality or timing guarantees.",
        ],
        "memory_256_frames_bytes_without_guards": {
            "canonical_2048_f32": 256 * 2048 * 4,
            "fixed_2048_ten_levels": 256 * 2048 * 4 * 10,
            "compact4_ten_levels": 256 * sum(lengths4) * 4,
            "compact8_ten_levels": 256 * sum(lengths8) * 4,
        },
        "rows": rows,
    }
    destination = Path(__file__).with_name("lookup_results.json")
    destination.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n")
    for name, *_ in methods:
        subset = [row for row in rows if row["method"] == name]
        worst = max(row["spurious_db_relative_to_reference_rms"] for row in subset)
        error = max(row["total_error_db_relative_to_reference_rms"] for row in subset)
        print(f"{name:34} worst spurious {worst:8.2f} dBr | worst total error {error:8.2f} dBr")
    print(f"Wrote {len(rows)} cases to {destination}")


if __name__ == "__main__":
    main()
