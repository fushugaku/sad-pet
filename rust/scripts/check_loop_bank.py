#!/usr/bin/env python3
"""Verify the recorded loop bank's provenance, unique PCM and bundled bytes."""
import array
import hashlib
import json
from pathlib import Path
import sys
import wave

ROOT = Path(__file__).resolve().parents[1]
BANK = ROOT / "assets/samples/loops-256"
manifest = json.loads((BANK / "manifest.json").read_text())
entries = manifest["samples"]
assert len(entries) == 256
assert len(list(BANK.glob("*.wav"))) == 256
identities, names, pcm_hashes = set(), set(), set()
for entry in entries:
    identity = entry["id"]
    assert identity not in identities and entry["name"] not in names
    identities.add(identity)
    names.add(entry["name"])
    assert entry["license"] == "CC0-1.0"
    assert entry["source_url"].startswith("https://freesound.org/s/")
    assert entry["creator"]
    annotation = entry["annotation"]
    assert annotation["well_cut"] and annotation["defined_tempo"] and not annotation["discard"]
    path = BANK / f"{identity}.wav"
    assert entry["asset"] == f"loops-256/{path.name}"
    assert hashlib.sha256(path.read_bytes()).hexdigest() == entry["sha256"], identity
    with wave.open(str(path)) as audio:
        assert audio.getsampwidth() == 2
        assert audio.getnchannels() == entry["channels"] in (1, 2)
        assert audio.getframerate() == entry["sample_rate"] >= 44100
        assert audio.getnframes() == entry["frames"]
        seconds = audio.getnframes() / audio.getframerate()
        assert 4 <= seconds <= 24 and abs(seconds - entry["seconds"]) < 1e-9
        pcm = audio.readframes(audio.getnframes())
    digest = hashlib.sha256(pcm).hexdigest()
    assert digest == entry["pcm_sha256"] and digest not in pcm_hashes, identity
    pcm_hashes.add(digest)
    beats = seconds * entry["annotated_bpm"] / 60
    assert beats >= 4 and abs(beats - entry["beats"]) < 0.16, identity
    values = array.array("h", pcm)
    if sys.byteorder != "little":
        values.byteswap()
    assert max(abs(value) for value in values) / 32768 >= 0.04, identity
    assert sum(abs(value) >= 32767 for value in values) / len(values) <= 0.002, identity
    quarter = len(values) // 4
    energies = [sum(v * v for v in values[i * quarter:(i + 1) * quarter]) for i in range(4)]
    assert min(energies) >= max(energies) * 0.015, identity
print(f"Verified {len(entries)} unique CC0 loops: hashes, PCM, 4–24 s duration, multi-beat timing, levels and sustained activity.")
