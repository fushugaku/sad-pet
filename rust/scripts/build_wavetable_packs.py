#!/usr/bin/env python3
"""Build SAD PET's original, deterministic factory wavetable packs.

Requires NumPy. The shipped gzip PCM is read by the shared Rust engine, so
native and WASM receive identical canonical waves without runtime generators.
"""
import gzip
import hashlib
import json
from pathlib import Path

import numpy as np

ROOT = Path(__file__).resolve().parents[1]
DEST = ROOT / "assets" / "wavetables"
N = 2048
FRAMES = 64
H = np.arange(1, N // 2, dtype=np.float64)
PHASE = np.arange(N, dtype=np.float64) / N
PACKS = [
    ("analog", "Analog Motion", [
        ("basic-morph", "Basic Morph", "morph"),
        ("pulse-width", "Pulse Width", "pulse"),
        ("triangle-bloom", "Triangle Bloom", "triangle"),
        ("saw-brass", "Saw Brass", "brass"),
        ("sync-contours", "Sync Contours", "sync"),
        ("organ-blend", "Organ Blend", "organ"),
        ("hollow-pulse", "Hollow Pulse", "hollow"),
        ("rounded-square", "Rounded Square", "round"),
    ]),
    ("harmonic", "Harmonic Worlds", [
        ("odd-even", "Odd / Even", "odd-even"),
        ("harmonic-shift", "Harmonic Shift", "shift"),
        ("spectral-peaks", "Spectral Peaks", "peaks"),
        ("additive-glass", "Additive Glass", "glass"),
        ("octave-stack", "Octave Stack", "octaves"),
        ("bell-metals", "Bell Metals", "bells"),
        ("comb-sweep", "Comb Sweep", "comb"),
        ("harmonic-steps", "Harmonic Steps", "steps"),
    ]),
    ("formant", "Vocal Matter", [
        ("vowel-sweep", "Vowel Sweep", "vowel"),
        ("choir-colors", "Choir Colors", "choir"),
        ("nasal-reed", "Nasal Reed", "nasal"),
        ("throat-bloom", "Throat Bloom", "throat"),
        ("breath-formants", "Breath Formants", "breath"),
        ("talking-bass", "Talking Bass", "talk"),
    ]),
    ("digital", "Digital Geometry", [
        ("bit-shapes", "Bit Shapes", "bits"),
        ("phase-spiral", "Phase Spiral", "spiral"),
        ("razor-stack", "Razor Stack", "razor"),
        ("metallic-scan", "Metallic Scan", "metal"),
        ("prism-fold", "Prism Fold", "fold"),
        ("digital-reeds", "Digital Reeds", "reeds"),
    ]),
    ("texture", "Living Texture", [
        ("dust-contours", "Dust Contours", "dust"),
        ("frozen-spark", "Frozen Spark", "spark"),
        ("noise-cloud", "Noise Cloud", "cloud"),
        ("fractured-air", "Fractured Air", "air"),
    ]),
]

# Each new recipe has a different spectral/phase trajectory, not a differently
# named copy of a basic waveform. Append packs to preserve the original seeds.
COMPLEX_PACKS = [
    ("fm", "Complex FM", "fm", [
        ("cascade-steel", "Cascade Steel"), ("ratio-bloom", "Ratio Bloom"),
        ("glass-operators", "Glass Operators"), ("nested-bells", "Nested Bells"),
        ("folded-carrier", "Folded Carrier"), ("electric-alloy", "Electric Alloy"),
        ("reed-operators", "Reed Operators"), ("crystal-feedback", "Crystal Feedback"),
    ]),
    ("formant_motion", "Formant Journeys", "formant", [
        ("vowel-labyrinth", "Vowel Labyrinth"), ("choir-machine", "Choir Machine"),
        ("liquid-throat", "Liquid Throat"), ("shifting-mouths", "Shifting Mouths"),
        ("nasal-orbit", "Nasal Orbit"), ("whisper-glass", "Whisper Glass"),
        ("formant-weather", "Formant Weather"), ("synthetic-syllables", "Synthetic Syllables"),
    ]),
    ("resonant", "Resonant Objects", "modal", [
        ("modal-bronze", "Modal Bronze"), ("glass-vessel", "Glass Vessel"),
        ("hollow-ceramic", "Hollow Ceramic"), ("bowed-metal", "Bowed Metal"),
        ("wire-chamber", "Wire Chamber"), ("tubular-flux", "Tubular Flux"),
        ("prism-chimes", "Prism Chimes"), ("reed-cavity", "Reed Cavity"),
    ]),
    ("phase", "Phase Architectures", "phase", [
        ("phase-maze", "Phase Maze"), ("bend-lattice", "Bend Lattice"),
        ("windowed-sync", "Windowed Sync"), ("fold-cascade", "Fold Cascade"),
        ("mirrored-pulse", "Mirrored Pulse"), ("helix-carrier", "Helix Carrier"),
        ("phase-fracture", "Phase Fracture"), ("twisted-reed", "Twisted Reed"),
    ]),
    ("spectral_morph", "Spectral Journeys", "spectral", [
        ("spectral-weave", "Spectral Weave"), ("peak-migration", "Peak Migration"),
        ("harmonic-eclipse", "Harmonic Eclipse"), ("comb-constellation", "Comb Constellation"),
        ("oddity-layers", "Oddity Layers"), ("spectral-inversion", "Spectral Inversion"),
        ("moving-cavities", "Moving Cavities"), ("harmonic-origami", "Harmonic Origami"),
    ]),
    ("vector", "Vector Worlds", "vector", [
        ("fourfold-motion", "Fourfold Motion"), ("string-glass-choir", "String Glass Choir"),
        ("alloy-garden", "Alloy Garden"), ("reed-to-steel", "Reed to Steel"),
        ("neon-orchestra", "Neon Orchestra"), ("ember-prism", "Ember Prism"),
        ("hybrid-tesseract", "Hybrid Tesseract"), ("polar-currents", "Polar Currents"),
    ]),
    ("fractal", "Fractal Matter", "fractal", [
        ("fractal-bloom", "Fractal Bloom"), ("dust-cathedral", "Dust Cathedral"),
        ("frozen-current", "Frozen Current"), ("prismatic-noise", "Prismatic Noise"),
        ("spectral-foam", "Spectral Foam"), ("quantum-grain", "Quantum Grain"),
        ("vapor-lattice", "Vapor Lattice"), ("particle-choir", "Particle Choir"),
    ]),
    ("digital_motion", "Digital Evolution", "digital", [
        ("byte-labyrinth", "Byte Labyrinth"), ("clockwork-formants", "Clockwork Formants"),
        ("folding-circuit", "Folding Circuit"), ("spectral-gears", "Spectral Gears"),
        ("segmented-glass", "Segmented Glass"), ("logic-choir", "Logic Choir"),
        ("serrated-orbit", "Serrated Orbit"), ("terminal-bloom", "Terminal Bloom"),
    ]),
]
PACKS += [(key, title, [(slug, name, f"complex:{family}:{i}") for i, (slug, name) in enumerate(entries)])
          for key, title, family, entries in COMPLEX_PACKS]


def complex_spectrum(family, t, v, seed):
    """Original harmonic trajectories, authored for an audible 1..63 band.

    Modal frequencies are projected onto integer partials: these are periodic
    tables, not claims of physically inharmonic resonator playback.
    """
    phase = np.pi * H * (H - 1) / (31 + v * 3)  # low crest-factor starting phases
    base = (0.15 + 0.16 * np.sin(H * (0.23 + v * 0.037)) ** 2) / H**0.55
    if family == "formant":
        knots = np.array([[3, 11, 23, 43], [6, 18, 34, 51], [10, 14, 28, 39],
                          [4, 24, 31, 56], [8, 12, 40, 48]], dtype=float)
        at = t * 3.8
        a = int(at)
        centers = (1 - at + a) * knots[(a + v) % 5] + (at - a) * knots[(a + v + 1) % 5]
        widths = np.array([1.2, 1.9, 2.6, 3.8]) * (0.8 + (v % 3) * .23)
        envelope = sum((.95 - i * .15) / (1 + ((H - c) / widths[i])**4)
                       for i, c in enumerate(centers))
        amplitude = base + envelope / H**.15
        amplitude *= .55 + .45 * np.sin(H * (.11 + .09 * t) + v)**2
        phase += .4 * np.sin(H * .31 + t * 4 + v)
    elif family == "modal":
        modes = [1.0, 2.76, 5.4, 8.93, 13.35, 18.64, 25.0, 32.4]
        centers = np.array(modes) * (1 + .12 * v + .8 * t) + .6 * np.sin(t * 5 + np.arange(8))
        envelope = sum((1 / (1 + .22 * i)) * np.exp(-.5 * ((H - c) / (.6 + .08 * i + .2 * t))**2)
                       for i, c in enumerate(centers))
        amplitude = base * .9 + envelope
        amplitude *= .35 + .65 * np.cos(H * (.13 + .17 * t) + v)**2
        phase += .5 * np.sin(H * (.07 + .006 * v) + t * 4)
    elif family == "spectral":
        ridge = .15 + .85 * np.sin(H * (.07 + .018 * v + .21 * t) + t * 3)**4
        moving = sum(np.exp(-((H - (c + (i % 2 * 2 - 1) * t * (8 + v))) / (1.4 + i))**2)
                     for i, c in enumerate([5 + v, 17 + v * 1.3, 37, 59]))
        amplitude = (ridge / H**.42 + moving * .8) * (.25 + .75 * np.cos(H * .17 - t * (2 + v * .3))**2)
        phase += .6 * np.sin(t * 4.5 + H * .19)
    elif family == "fractal":
        rng = np.random.default_rng(seed)
        random_phase, drift = rng.uniform(-np.pi, np.pi, (2, len(H)))
        field = sum(np.sin(np.log2(H + 1) * (2**i) * (.7 + v * .08) + t * (i + 1) * 3 + v)
                    / 2**(.65 * i) for i in range(5))
        amplitude = (.2 + np.exp(field * .65)) / H**(.40 + (v % 3) * .08)
        amplitude *= .25 + .75 * np.sin(H * (.09 + v * .01) + t * 4)**2
        phase = random_phase + drift * .7 * np.sin(t * 2.7)
    else:
        raise ValueError(family)
    # A stable low anchor keeps high/formant frames useful across the keyboard.
    coefficients = amplitude * np.exp(1j * phase)
    coefficients[0] = .35j
    return coefficients


def complex_waveform(family, t, v, seed):
    if family in {"formant", "modal", "spectral", "fractal"}:
        frequency = np.zeros(N // 2 + 1, dtype=np.complex128)
        frequency[1:-1] = complex_spectrum(family, t, v, seed)
        return np.fft.irfft(frequency, n=N) * N
    if family == "vector":
        # Four *different* spectral sources follow a curved path with no silent
        # center. This is authored content, independent of a future live XY mix.
        x = .5 + .48 * np.sin(t * 4.1 + v * .47)
        y = .5 + .48 * np.cos(t * 3.7 + v * .63)
        weights = [(1-x)*(1-y), x*(1-y), (1-x)*y, x*y]
        families = ["formant", "modal", "spectral", "fractal"]
        return sum(w * complex_waveform(f, .1 + .8 * (.5 + .5 * np.sin(t * 2.7 + i * 1.1 + v * .23)), (v + i) % 8, seed + i)
                   for i, (f, w) in enumerate(zip(families, weights)))
    # Nonlinear content is authored at 8x then Fourier-truncated to the
    # canonical bandwidth. No nonlinear transform or oversampling in render.
    q = np.arange(N * 8, dtype=np.float64) / (N * 8)
    a = 2 * np.pi * q
    r = [2, 3, 5, 7, 4, 6, 9, 11][v]
    if family == "fm":
        inner = np.sin(r * a + (1.1 + 3.4 * t) * np.sin((r + 2) * a + .3 * v))
        carrier = np.sin(a + (2.2 + 6 * t) * inner + .6 * np.sin((v % 3 + 2) * a))
        side = np.sin(3 * a + (1.3 + 4 * (1-t)) * np.sin((r + 1) * a + t * 2))
        wave = .63 * carrier + .27 * side + .10 * np.sin(a)
        if v in (4, 7):
            wave = np.sin(wave * (1.5 + 2.5 * t))
    elif family == "phase":
        bent = q + (.055 + .10 * t) * np.sin((v % 3 + 1) * a + t) + .025 * np.sin(r * a)
        carrier = np.sin(2 * np.pi * bent + (1.3 + t * 4) * np.sin((r + 1) * a))
        gate = .48 + .52 * np.sin(np.pi * q)**2
        wave = carrier * gate + .28 * np.sin(3 * a + .9 * np.sin(r * a))
        if v in (2, 3, 6):
            wave = np.sin(wave * (2.5 + 5 * t))
    elif family == "digital":
        carrier = np.sin(a + (2 + 4 * t) * np.sin(r * a + .7 * np.sin(3 * a)))
        folds = np.sin((1.2 + 4 * t) * (carrier + .23 * np.sin((r + 2) * a)))
        steps = 4 + v * 2
        coarse = np.round(folds * steps) / steps
        logic = np.tanh((np.sin((v % 4 + 2) * a) * carrier + .22 * np.sin(a)) * 5)
        wave = .55 * coarse + .35 * logic + .1 * carrier
    else:
        raise ValueError(family)
    frequency = np.fft.rfft(wave) / 8
    frequency[0] = 0
    frequency[N // 2] = 0
    return np.fft.irfft(frequency[:N // 2 + 1], n=N)


def saw():
    return 1j / H


def pulse(width):
    # Integrating a rectangular pulse contributes 1/(i*h). Omitting i makes
    # logarithmic edge spikes instead of a pulse and steals the whole pack's
    # headroom when the morph includes a narrow duty cycle. This polarity
    # aligns its fundamental with saw/triangle; 1/2 matches their amplitude.
    return (np.exp(-2j * np.pi * H * width) - 1.0) / (2j * H)


def triangle():
    signs = np.where(((H.astype(int) - 1) // 2) % 2 == 0, 1.0, -1.0)
    return 1j * np.where(H % 2 == 1, signs, 0) / H**2


def spectral(kind, t, seed):
    sine = np.zeros(len(H), dtype=np.complex128)
    sine[0] = 1j
    if kind == "morph":
        shapes = [saw(), triangle(), sine, pulse(0.5), pulse(0.1)]
        at = min(t * 4, 3.999999999)
        i = int(at)
        return shapes[i] * (1 - (at - i)) + shapes[i + 1] * (at - i)
    if kind == "pulse":
        return pulse(0.02 + 0.48 * t)
    if kind == "triangle":
        return triangle() + saw() * (t * t * 0.45)
    if kind == "brass":
        return saw() * np.exp(-H / (3 + 120 * t * t))
    if kind == "organ":
        return 1j * np.exp(-((np.log2(H) - 3.5 * t) / 1.4) ** 2) / H**0.4
    if kind == "hollow":
        return pulse(0.5) * (0.2 + 0.8 * np.sin(H * (0.05 + 0.35 * t)) ** 2)
    if kind == "odd-even":
        return saw() * np.where(H % 2 == 1, 1 - 0.85 * t, 0.15 + 0.85 * t)
    if kind == "shift":
        return 1j * np.exp(-((H - (1 + 40 * t)) / (1.2 + 8 * t)) ** 2) + 0.12 * saw()
    if kind == "peaks":
        return saw() * (0.08 + np.sin(H * (0.07 + 0.6 * t)) ** 8)
    if kind == "glass":
        centers = [1, 3 + 8 * t, 9 + 20 * t, 23 + 35 * t]
        return 1j * sum(np.exp(-((H - c) / (0.6 + t)) ** 2) / (i + 1) for i, c in enumerate(centers))
    if kind == "octaves":
        return 1j * sum(np.exp(-((H - 2**i) / (0.3 + 1.5 * t)) ** 2) / (1 + 0.5 * i) for i in range(8))
    if kind == "bells":
        return np.exp(1j * H * (0.1 + t)) * sum(np.exp(-((H - c) / (0.6 + 2 * t)) ** 2) / (i + 1) for i, c in enumerate([1, 4, 9, 17, 31, 53]))
    if kind == "comb":
        return saw() * (0.06 + np.cos(H * (0.03 + t)) ** 12)
    if kind == "steps":
        return saw() * np.exp(-H / (2 ** (2 + 7 * t))) * (0.2 + 0.8 * ((H.astype(int) + int(t * 12)) % 5 == 0))
    if kind in {"vowel", "choir", "nasal", "throat", "breath", "talk"}:
        offset = ["vowel", "choir", "nasal", "throat", "breath", "talk"].index(kind)
        centers = [3 + 8 * t + offset, 11 + 17 * (1 - t) + offset, 30 + 12 * np.sin(t * np.pi)]
        width = 1.3 + 0.45 * offset
        envelope = sum(np.exp(-0.5 * ((H - c) / (width * (i + 1))) ** 2) / (i + 1) for i, c in enumerate(centers))
        return saw() * (0.05 + 2.0 * envelope)
    if kind in {"dust", "spark", "cloud", "air"}:
        rng = np.random.default_rng(seed)
        a, b = rng.uniform(-np.pi, np.pi, (2, len(H)))
        contour = 0.05 + np.exp(-((np.log2(H) - (1 + 7 * t)) / (1.1 + (seed % 3) * 0.3)) ** 2)
        return contour * np.exp(1j * (a + np.sin(t * np.pi) * b)) / H**(0.35 + (seed % 4) * 0.12)
    return None


def waveform(kind, t, seed):
    if kind.startswith("complex:"):
        _, family, variant = kind.split(":")
        return complex_waveform(family, t, int(variant), seed)
    coefficients = spectral(kind, t, seed)
    if coefficients is not None:
        frequency = np.zeros(N // 2 + 1, dtype=np.complex128)
        frequency[1:-1] = coefficients
        return np.fft.irfft(frequency, n=N) * N
    x = PHASE * 2 - 1
    if kind == "round":
        return np.tanh(np.sin(2 * np.pi * PHASE) * (1 + 12 * t))
    if kind == "sync":
        phase = PHASE * (1 + 7 * t)
        return np.sin(2 * np.pi * phase) * np.sin(np.pi * PHASE) ** 2
    if kind == "bits":
        steps = 2 + int(30 * (1 - t))
        return np.round(np.sin(2 * np.pi * PHASE) * steps) / steps
    if kind == "spiral":
        return np.sin(2 * np.pi * PHASE + (0.2 + 8 * t) * np.sin(2 * np.pi * PHASE))
    if kind == "razor":
        return np.tanh((np.sin(2 * np.pi * PHASE) + t * np.sin(14 * np.pi * PHASE)) * 4)
    if kind == "metal":
        return np.sin(2 * np.pi * PHASE + 6 * t * np.sin(10 * np.pi * PHASE))
    if kind == "fold":
        return np.sin(x * (1 + 9 * t) * np.pi)
    if kind == "reeds":
        return np.tanh((np.sin(2 * np.pi * PHASE) + 0.4 * np.sin(6 * np.pi * PHASE + t * 2)) * (1 + 8 * t))
    raise ValueError(kind)


def main():
    DEST.mkdir(parents=True, exist_ok=True)
    manifest = {"version": 1, "frame_size": N, "author": "SAD PET", "provenance": "Original procedural waves; no third-party recordings or wavetable data", "packs": []}
    rust = ["// Generated by rust/scripts/build_wavetable_packs.py. Do not edit.", "pub static FACTORY: &[Factory] = &["]
    total = 0
    for pack_index, (pack, title, entries) in enumerate(PACKS):
        record = {"id": pack, "name": title, "tables": []}
        for index, (key, name, recipe) in enumerate(entries):
            seed = 0x57540000 + pack_index * 100 + index
            data = np.stack([waveform(recipe, i / (FRAMES - 1), seed) for i in range(FRAMES)])
            data -= data.mean(axis=1, keepdims=True)
            if recipe.startswith("complex:"):
                # Explicit factory-content loudness curation. The importer and
                # bank compiler still preserve every asset's frame amplitudes.
                rms = np.sqrt(np.mean(data * data, axis=1, keepdims=True))
                data *= .30 / np.maximum(rms, 1e-12)
            data *= 0.96 / max(1e-12, float(np.max(np.abs(data))))
            pcm = np.rint(data * 32768).astype('<i2')
            decoded = pcm.astype(np.float64) / 32768.0
            canonical = (decoded - decoded.mean(axis=1, keepdims=True)).astype('<f4')
            header = b'SAD-WT1\0' + N.to_bytes(4, 'little') + FRAMES.to_bytes(4, 'little')
            digest = hashlib.sha256(header + canonical.tobytes()).hexdigest()
            encoded = gzip.compress(pcm.tobytes(), compresslevel=9, mtime=0)
            filename = key + '.pcm.gz'
            (DEST / filename).write_bytes(encoded)
            total += len(encoded)
            id_ = f'factory:wt-{key}:v1'
            record['tables'].append({"id": id_, "name": name, "recipe": recipe, "seed": seed, "frames": FRAMES, "content_hash": digest, "file": filename, "packed_bytes": len(encoded)})
            rust.append('    Factory { id: '+json.dumps(id_)+', name: '+json.dumps(name)+', pack: '+json.dumps(pack)+', pack_name: '+json.dumps(title)+', frames: '+str(FRAMES)+', content_hash: '+json.dumps(digest)+', bytes: include_bytes!("../../assets/wavetables/'+filename+'") },')
        manifest['packs'].append(record)
    rust.append('];\n')
    (DEST / 'manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')
    (ROOT / 'src' / 'wavetable' / 'factory_data.rs').parent.mkdir(parents=True, exist_ok=True)
    (ROOT / 'src' / 'wavetable' / 'factory_data.rs').write_text('\n'.join(rust))
    print(f'Built {sum(len(p[2]) for p in PACKS)} tables in {len(PACKS)} packs: {total:,} compressed bytes.')


if __name__ == '__main__':
    main()
