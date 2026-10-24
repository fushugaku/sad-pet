#!/usr/bin/env python3
"""Build the Sampler's bundled packs in rust/assets/sampler-packs.

Every sound comes from a CC0 source pinned to a commit: the Versilian Community
Sample Library (VCSL) and Michael Fischer's TR-808 set as published by
TidalCycles. Files are downloaded once into --cache, trimmed of silence, faded,
capped in length, converted to 16-bit 44.1 kHz WAV with one gain per pad (so
velocity layers keep their dynamics) and written with a pack.json kit in the
editor's sad-pet-pack-v1 format, credits and an index. Needs ffmpeg.
"""
import argparse
import hashlib
import json
import re
import shutil
import struct
import subprocess
import urllib.parse
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent
OUT = ROOT / "rust" / "assets" / "sampler-packs"
RATE = 44100
SOURCES = {
    "vcsl": {
        "url": "https://raw.githubusercontent.com/sgossner/VCSL/c1ea7bcc3c7309650ab0da9d15c9cd1fbc4a4c7e/",
        "tree": "https://api.github.com/repos/sgossner/VCSL/git/trees/c1ea7bcc3c7309650ab0da9d15c9cd1fbc4a4c7e?recursive=1",
        "name": "Versilian Community Sample Library (VCSL), Versilian Studios LLC",
        "home": "https://github.com/sgossner/VCSL",
        "license": "CC0 1.0 Universal",
    },
    "tr808": {
        "url": "https://raw.githubusercontent.com/tidalcycles/sounds-tr808-fischer/85fbecf1bec32553395625ea659e2a56dfd7c0e1/",
        "name": "Roland TR-808 Sound Sample Set 1.0.0, Michael Fischer (Technopolis, 1994), "
        "published by TidalCycles",
        "home": "https://github.com/tidalcycles/sounds-tr808-fischer",
        "license": "CC0 1.0 Universal (repository dedication; the author released the set free of charge)",
    },
}
NAMES = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"]


def note_number(name, octave_shift=0):
    """'A#3' or 'C-1' → MIDI, with C4 = 60 plus the source's octave shift."""
    match = re.fullmatch(r"([A-G])(#|b)?(-?\d)", name)
    pc = NAMES.index(match[1]) + {"#": 1, "b": -1, None: 0}[match[2]]
    return (int(match[3]) + 1 + octave_shift) * 12 + pc


# --- Kit structures, complete so the editor never meets a missing field ----------
def zone(sample, **fields):
    z = {
        "sample": sample, "start": 0, "end": 0, "loop_mode": "off", "loop_start": 0,
        "crossfade": 0, "reverse": False, "fade_in": 0, "fade_out": 0, "fade_curve": [0, 0],
        "root": 60, "tune": 0, "fine": 0, "gain": 0, "pan": 0, "keys": [0, 127], "velocities": [1, 127],
    }
    z.update(fields)
    return z


def pad(name, zones, **fields):
    p = {
        "name": name, "keys": None, "keytrack": False, "trigger": "one_shot", "mono": False, "choke": 0,
        "layers": "velocity", "zones": zones, "tune": 0, "fine": 0, "gain": 0, "pan": 0,
        "envelope": {"attack": 0, "hold": 0, "decay": 300, "sustain": 1, "release": 30},
        "filter": {"kind": "off", "cutoff": 8000, "resonance": 0.1, "envelope": 0, "attack": 0,
                   "decay": 300, "keytrack": 0, "velocity": 0},
        "pitch": {"amount": 0, "decay": 100}, "velocity": {"level": 1, "start": 0, "attack": 0},
        "muted": False, "solo": False, "bounce": None,
    }
    for key, value in fields.items():
        if key in ("envelope", "filter", "pitch", "velocity"):
            p[key].update(value)
        else:
            p[key] = value
    return p


class Pack:
    def __init__(self, id, name, description, source, max_seconds=4.0, mono=False):
        self.id, self.name, self.description, self.source = id, name, description, source
        self.max_seconds, self.mono = max_seconds, mono
        self.pads = []
        self.files = {}  # sample id -> (source path, display name)

    def sample(self, path, name):
        slug = re.sub(r"[^a-z0-9]+", "-", name.lower()).strip("-")
        sid = f"{self.id}-{slug}"
        n = 2
        while sid in self.files and self.files[sid][0] != path:
            sid = f"{self.id}-{slug}-{n}"
            n += 1
        self.files[sid] = (path, name)
        return sid

    def drum(self, name, layers, **fields):
        """A one-shot pad; layers run soft to loud over the velocity range."""
        zones = []
        for i, path in enumerate(layers):
            low = 1 + i * 127 // len(layers)
            high = (i + 1) * 127 // len(layers)
            label = name if len(layers) == 1 else f"{name} {i + 1}"
            zones.append(zone(self.sample(path, label), velocities=[low, high]))
        self.pads.append(pad(name, zones, **fields))

    def keys(self, name, notes, low, high, **fields):
        """A keygroup pad: (path, MIDI root) samples spread over low…high."""
        notes = sorted(notes, key=lambda n: n[1])
        zones = []
        for i, (path, root) in enumerate(notes):
            lo = low if i == 0 else (notes[i - 1][1] + root) // 2 + 1
            hi = high if i == len(notes) - 1 else (root + notes[i + 1][1]) // 2
            label = f"{name} {NAMES[root % 12]}{root // 12 - 1}"
            zones.append(zone(self.sample(path, label), root=root, keys=[lo, hi]))
        self.pads.append(pad(name, zones, keytrack=True, keys=[low, high], **{"trigger": "note_on", **fields}))


def pick(tree, folder, pattern, octave_shift=0, every=1):
    """Files under `folder` whose name matches `pattern` with a (?P<note>…) group."""
    found = {}
    for path in tree:
        if not path.startswith(folder + "/"):
            continue
        match = re.search(pattern, path[len(folder) + 1:])
        if match:
            root = note_number(match["note"], octave_shift)
            found.setdefault(root, path)
    roots = sorted(found)[::every]
    return [(found[r], r) for r in roots]


def define(tree):
    V = lambda *parts: "/".join(parts)
    MS = "Membranophones/Struck Membranophones"
    IS = "Idiophones/Struck Idiophones"
    packs = []

    p = Pack("tr808", "TR-808", "Michael Fischer's Roland TR-808 set: kicks, snares, hats, toms, "
             "congas and percussion, with kick and snare variations on bank B", "tr808", max_seconds=4)
    for name, layers, extra in [
        ("Kick", ["bd8/BD5050.WAV"], {}), ("Kick long", ["bd8/BD5075.WAV"], {}),
        ("Snare", ["sd8/SD5050.WAV"], {}), ("Snare snappy", ["sd8/SD5075.WAV"], {}),
        ("Rimshot", ["rs8/RS.WAV"], {}), ("Clap", ["cp8/CP.WAV"], {}),
        ("Closed hat", ["ch8/CH.WAV"], {"choke": 1}), ("Open hat", ["oh8/OH50.WAV"], {"choke": 1}),
        ("Cymbal", ["cy8/CY5050.WAV"], {}), ("Cowbell", ["cb8/CB.WAV"], {}),
        ("Claves", ["cl8/CL.WAV"], {}), ("Maracas", ["ma8/MA.WAV"], {}),
        ("Low tom", ["lt8/LT50.WAV"], {}), ("Mid tom", ["mt8/MT50.WAV"], {}),
        ("High tom", ["ht8/HT50.WAV"], {}), ("High conga", ["hc8/HC50.WAV"], {}),
        ("Kick tight", ["bd8/BD5010.WAV"], {}), ("Kick boom", ["bd8/BD2575.WAV"], {}),
        ("Kick bright", ["bd8/BD7550.WAV"], {}), ("Kick dark", ["bd8/BD0050.WAV"], {}),
        ("Kick short", ["bd8/BD5000.WAV"], {}), ("Snare dark", ["sd8/SD0050.WAV"], {}),
        ("Snare bright", ["sd8/SD7550.WAV"], {}), ("Snare tight", ["sd8/SD5010.WAV"], {}),
        ("Open hat short", ["oh8/OH10.WAV"], {"choke": 1}), ("Open hat long", ["oh8/OH75.WAV"], {"choke": 1}),
        ("Cymbal long", ["cy8/CY5075.WAV"], {}), ("Low conga", ["lc8/LC50.WAV"], {}),
        ("Mid conga", ["mc8/MC50.WAV"], {}), ("Low tom long", ["lt8/LT75.WAV"], {}),
        ("Mid tom long", ["mt8/MT75.WAV"], {}), ("High tom long", ["ht8/HT75.WAV"], {}),
    ]:
        p.drum(name, layers, **extra)
    packs.append(p)

    p = Pack("acoustic-kit", "Acoustic Kit", "A recorded drum kit from VCSL with velocity layers: "
             "kick, snares, toms, hi-hat, cymbals, claps and tambourine", "vcsl", max_seconds=3.5)
    hh = V(IS, "Hi-Hat Cymbal")
    for name, layers, extra in [
        ("Kick", [V(MS, "Bass Drum 1", f"BDrumNew_hit_v{v}_rr1_Sum.wav") for v in (2, 3, 5, 7)], {}),
        ("Snare", [V(MS, "Snare Drum, Modern 1", f"Snare2_HitSN_v{v}_rr1_Mid.wav") for v in (3, 5, 6, 7, 9)], {}),
        ("Snare off", [V(MS, "Snare Drum, Modern 1", f"Snare2_HitNS_v{v}_rr1_Mid.wav") for v in (2, 4, 5, 6)], {}),
        ("Cross stick", [V(MS, "Snare Drum, Modern 3", f"Snare4_Xstick_v{v}_rr1_Mid.wav") for v in (2,)], {}),
        ("Closed hat", [V(hh, f"HiHat_HitC_v{v}_rr1_Mid.wav") for v in (1, 2, 3, 4)], {"choke": 1}),
        ("Pedal hat", [V(hh, "HiHat_Close_rr1_Mid.wav")], {"choke": 1}),
        ("Loose hat", [V(hh, "HiHat_HitLoose_rr1_Mid.wav")], {"choke": 1}),
        ("Open hat", [V(hh, "HiHat_HitO_rr1_Mid.wav")], {"choke": 1}),
        ("High tom", [V(MS, "Tom 1", f"Stick/TomH_HitS_v{v}_rr1_Mid.wav") for v in (2, 3, 4)], {}),
        ("Low tom", [V(MS, "Tom 2", f"Stick/TomL_HitS_v{v}_rr1_Mid.wav") for v in (2, 3, 4)], {}),
        ("Crash", [V(IS, "Suspended Cymbal 1", f"susCymb1_hit_{v}1.wav") for v in ("mp", "f", "fff")], {}),
        ("Ride", [V(IS, "Suspended Cymbal 1", f"susCymb1_hit_stick_{v}1.wav") for v in ("pp", "mp", "f")], {}),
        ("Bell", [V(IS, "Suspended Cymbal 1", f"susCymb1_hit_bell_{v}1.wav") for v in ("mf", "fff")], {}),
        ("Clap", [V(IS, "Claps", f"SoloClap_vl{v}.wav") for v in (1, 2, 3, 4)], {}),
        ("Tambourine", [V(IS, "Tambourine 1", f"Tamb1_Hit_v{v}_rr1_Mid.wav") for v in (1, 2)], {}),
        ("Snare roll", [V(MS, "Snare Drum, Modern 1", "Snare2_rollSN_v4_rr1_Mid.wav")], {"trigger": "note_on", "envelope": {"release": 120}}),
    ]:
        p.drum(name, layers, **extra)
    packs.append(p)

    p = Pack("hand-percussion", "Hand Percussion", "Congas, bongos, cajón, darbuka, frame drum, "
             "shakers and small percussion from VCSL, with velocity layers", "vcsl", max_seconds=3)
    for name, layers, extra in [
        ("Conga open", [V(MS, "Conga", f"Conga_HitN_v{v}_rr1_Sum.wav") for v in (1, 2)], {}),
        ("Conga muted", [V(MS, "Conga", f"Conga_HitFM_v{v}_rr1_Sum.wav") for v in (1, 2)], {}),
        ("Bongo high", [V(MS, "Bongos", f"BongoH_Hit1_v{v}_rr1_Mid.wav") for v in (1, 2, 3)], {}),
        ("Bongo muted", [V(MS, "Bongos", f"BongoH_HitMuted1_v{v}_rr1_Mid.wav") for v in (1,)], {}),
        ("Cajón bass", [V(IS, "Cajon", f"Cajon_hit1_{v}_rr1.wav") for v in ("mp", "f", "fff")], {}),
        ("Cajón slap", [V(IS, "Cajon", f"Cajon_hit2_{v}_rr1.wav") for v in ("pp", "mp", "f")], {}),
        ("Darbuka doum", [V(MS, "Darbuka", f"Darbuka_1_hit_vl{v}_rr1.wav") for v in (1, 2)], {}),
        ("Darbuka tek", [V(MS, "Darbuka", f"Darbuka_2_hit_vl{v}_rr1.wav") for v in (1, 2)], {}),
        ("Frame drum", [V(MS, "Frame Drum", f"HDrumL_Hit_v{v}_rr1_Sum.wav") for v in (2,)], {}),
        ("Shaker", [V(IS, "Shaker, Small", "Mid_ShakerDouble_Down_rr1.wav")], {}),
        ("Tambourine", [V(IS, "Tambourine 1", f"Tamb1_Hit_v{v}_rr1_Mid.wav") for v in (1, 2)], {}),
        ("Claves", [V(IS, "Claves", f"Claves1_Hit_v{v}_rr1_Mid.wav") for v in (1, 2, 3)], {}),
        ("Woodblock", [V(IS, "Woodblock", f) for f in ("wood_click_mp.wav", "wood_click_f_rr1.wav", "wood_click_ff.wav")], {}),
        ("Agogo high", [V(IS, "Agogo Bells", f"Agogo_High_v{v}_rr1_Mid.wav") for v in (1, 2, 3)], {}),
        ("Agogo low", [V(IS, "Agogo Bells", f"Agogo_Low_v{v}_rr1_Mid.wav") for v in (1, 2)], {}),
        ("Guiro", [V(IS, "Guiro", "Guiro_Hit_rr1_Mid.wav")], {}),
        ("Cowbell", [V(IS, "Cowbells", f"Cowbell1_Hit_v{v}_rr1_Mid.wav") for v in (2, 3, 4)], {}),
        ("Cowbell muted", [V(IS, "Cowbells", f"Cowbell1_Muted_v{v}_rr1_Mid.wav") for v in (2, 3)], {}),
        ("Cabasa", [V(IS, "Cabasa", "Cabasa1_Hit_rr1_Mid.wav")], {}),
        ("Large shaker", [V(IS, "Shaker, Large", "LShaker_Hit_rr1_Mid.wav")], {}),
        ("Slit drum high", [V(IS, "Slit Drum", f"LogDrumHi_MedM_v{v}_rr1_Sum.wav") for v in (1, 2, 3)], {}),
        ("Slit drum low", [V(IS, "Slit Drum", f"LogDrumLo_MedM_v{v}_rr1_Sum.wav") for v in (1, 2, 3)], {}),
        ("Sleigh bells", [V(IS, "Sleigh Bells", "Sleighbells_Hit_rr1_Mid.wav")], {}),
        ("Slapstick", [V(IS, "Slapstick", "slapstick_rr1.wav")], {}),
    ]:
        p.drum(name, layers, **extra)
    packs.append(p)

    p = Pack("grand-piano", "Grand Piano", "VCSL's Kawai grand, one sample every few keys over the "
             "whole keyboard; it plays from the keyboard and holds while a key is down", "vcsl", max_seconds=3.5)
    notes = pick(tree, "Chordophones/Zithers/Grand Piano, Kawai/Sustains",
                 r"GPiano_sus_(?P<note>[A-G]#?-?\d)_v3_rr1", octave_shift=1, every=2)
    p.keys("Grand piano", notes, 21, 108, envelope={"release": 350}, velocity={"level": 0.8})
    packs.append(p)

    p = Pack("fm-piano", "FM Electric Piano", "The Yamaha TX81Z's FM piano from VCSL, a sample every "
             "four keys, for the whole keyboard", "vcsl", max_seconds=4)
    notes = pick(tree, "Electrophones/TX81Z/FM Piano", r"FMPiano_(?P<note>[A-G]#?-?\d)_vl2")
    p.keys("FM piano", notes, 21, 108, envelope={"release": 250}, velocity={"level": 0.7})
    packs.append(p)

    p = Pack("mallets", "Mallets", "Marimba, vibraphone, xylophone and glockenspiel from VCSL, split "
             "over the keyboard from low to high", "vcsl", max_seconds=3)
    marimba = pick(tree, V(IS, "Marimba"), r"Marimba_hit_Outrigger_(?P<note>[A-G]#?-?\d)_med_01", octave_shift=1)
    vibes = pick(tree, V(IS, "Vibraphone"), r"Hard Mallets/Vibes_hard_(?P<note>[A-G]#?-?\d)_v2_rr1", octave_shift=1)
    xylo = pick(tree, V(IS, "Xylophone"), r"Medium Mallets/Xylo_Medium_(?P<note>[A-G]#?-?\d)_ff_01", octave_shift=1)
    glock = pick(tree, V(IS, "Glockenspiel"), r"glock_medium_(?P<note>[A-G]#?-?\d)_01", octave_shift=1)
    p.keys("Marimba", marimba, 24, 59, trigger="one_shot")
    p.keys("Vibraphone", vibes, 60, 76, envelope={"release": 600})
    p.keys("Xylophone", xylo, 77, 91, trigger="one_shot")
    p.keys("Glockenspiel", glock, 92, 108, trigger="one_shot")
    packs.append(p)

    p = Pack("concert-harp", "Concert Harp", "VCSL's concert harp, plucked mezzo-forte, over the "
             "whole keyboard; notes ring on after the key", "vcsl", max_seconds=3.5)
    notes = pick(tree, "Chordophones/Composite Chordophones/Concert Harp",
                 r"KSHarp_(?P<note>[A-G]#?-?\d)_mf\d")
    p.keys("Concert harp", notes, 24, 103, trigger="one_shot")
    packs.append(p)

    p = Pack("cinematic", "Cinematic Hits & FX", "Timpani, orchestral bass drum, gong, clash cymbals, "
             "tubular bells, wind chimes, ocean drum and other textures from VCSL", "vcsl", max_seconds=6)
    for name, layers, extra in [
        ("Timpani", [V(MS, "Timpani 1", f"Hit/Timpani1_Hit_v{v}_rr1_Sum.wav") for v in (2, 4)], {}),
        ("Timpani high", [V(MS, "Timpani 1", f"Hit/Timpani4_Hit_v{v}_rr1_Sum.wav") for v in (2, 4)], {}),
        ("Bass drum", [V(MS, "Bass Drum 2", f"bassdrum_hit_{v}.wav") for v in ("mf1", "ff")], {}),
        ("Bass drum swell", [V(MS, "Bass Drum 2", "bassdrum_cresc_short.wav")], {}),
        ("Gong", [V(IS, "Gong 1", f"gong_{v}.wav") for v in ("mf", "fff")], {}),
        ("Clash cymbals", [V(IS, "Clash Cymbals 1", f"cymbal_crash1_{v}.wav") for v in ("mf1", "ff2")], {}),
        ("Cymbal swell", [V(IS, "Suspended Cymbal 1", "susCymb1_cresc_2s.wav")], {}),
        ("Tubular bell", [V(IS, "Tubular Bells 1", "chimes_C4_ff_rr2.wav")], {"keytrack": True}),
        ("Wind chimes", [V(IS, "Mark Trees", "Legacy/windchimes_slowDesc1.wav")], {}),
        ("Bell tree", [V(IS, "Bell Tree", "Stroke/BellTree_Stroke_10_Mid.wav")], {}),
        ("Ocean drum", [V("Membranophones/Other Membranophones/Ocean Drum", "OceanDrum_Sus_1_Mid.wav")], {}),
        ("Wine glass", [V("Idiophones/Friction Idiophones/Wine Glasses", "Releases/Slow/glass1_D#4_Slow_1_Decay.wav")], {}),
        ("Anvil", [V(IS, "Anvil", f"Anvil_Hit1_v{v}_rr1_Mid.wav") for v in (1, 2, 3)], {}),
        ("Brake drum", [V(IS, "Brake Drum", f"BrakeDrum1_Hammer_v{v}_rr1_Mid.wav") for v in (1, 2, 3)], {}),
        ("Flexatone", [V(IS, "Flexatone", "flexatone1.wav")], {}),
        ("Ratchet", [V(IS, "Ratchet", "Ratchet1_Fast_rr1_Mid.wav")], {}),
    ]:
        p.drum(name, layers, **extra)
    packs.append(p)

    for pack in packs:
        # Drum-style pads sit on consecutive keys from C2, sixteen to a bank.
        for i, pd in enumerate(pack.pads):
            pd["id"] = i + 1
            if pd["keys"] is None:
                pd["keys"] = [36 + i, 36 + i]
    return packs


# --- Audio --------------------------------------------------------------------
def fetch(source, path, cache):
    target = cache / source / path
    if not target.exists():
        target.parent.mkdir(parents=True, exist_ok=True)
        url = SOURCES[source]["url"] + urllib.parse.quote(path)
        with urllib.request.urlopen(url) as response, open(target, "wb") as out:
            shutil.copyfileobj(response, out)
    return target


def decode(path):
    """Interleaved stereo float frames at RATE, as a list of (l, r)."""
    raw = subprocess.run(
        ["ffmpeg", "-v", "error", "-i", str(path), "-ac", "2", "-ar", str(RATE), "-f", "f32le", "-"],
        check=True, capture_output=True).stdout
    values = struct.unpack(f"<{len(raw) // 4}f", raw)
    return values[0::2], values[1::2]


def analyse(left, right, max_seconds):
    """Where the sound starts and ends, its peak, and whether it is mono."""
    level = [max(abs(a), abs(b)) for a, b in zip(left, right)]
    peak = max(level) if level else 0.0
    if peak == 0.0:
        raise SystemExit("silent sample")
    floor = peak * 10 ** (-62 / 20)
    start = next(i for i, v in enumerate(level) if v > peak * 10 ** (-50 / 20))
    start = max(0, start - RATE // 1000)
    end = len(level) - next(i for i, v in enumerate(reversed(level)) if v > floor)
    end = min(len(level), end + RATE // 50, start + int(max_seconds * RATE))
    mono = all(abs(a - b) <= peak * 1e-3 for a, b in zip(left[start:end:7], right[start:end:7]))
    return start, end, max(level[start:end]), mono


def write(path, left, right, start, end, gain, mono, capped):
    frames = end - start
    fade = min(frames, RATE // 20 if capped else RATE // 400)
    channels = 1 if mono else 2
    data = bytearray()
    for i in range(frames):
        g = gain * (1.0 if i < frames - fade else (frames - i) / fade)
        values = (left[start + i],) if mono else (left[start + i], right[start + i])
        for v in values:
            data += struct.pack("<h", max(-32767, min(32767, round(v * g * 32767))))
    with open(path, "wb") as out:
        out.write(b"RIFF" + struct.pack("<I", 36 + len(data)) + b"WAVEfmt ")
        out.write(struct.pack("<IHHIIHH", 16, 1, channels, RATE, RATE * channels * 2, channels * 2, 16))
        out.write(b"data" + struct.pack("<I", len(data)) + data)
    return frames


def credits(pack):
    source = SOURCES[pack.source]
    lines = [f"{pack.name} — a SAD PET Sampler pack", "=" * 40, "",
             f"Source: {source['name']}", f"  {source['home']}", f"License: {source['license']}",
             f"Pinned at: {source['url']}", "",
             "Trimmed of silence, faded, capped in length and converted to 16-bit 44.1 kHz;",
             "one gain per pad; its velocity layers stay within 12 dB. Original files:", ""]
    for sid, (path, name) in sorted(pack.files.items()):
        lines.append(f"{sid}.wav  ←  {path}")
    return "\n".join(lines) + "\n"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cache", type=Path, required=True, help="Download folder, kept between runs")
    parser.add_argument("--only", nargs="*", help="Pack ids to rebuild")
    args = parser.parse_args()
    args.cache.mkdir(parents=True, exist_ok=True)
    tree_file = args.cache / "vcsl-tree.json"
    if not tree_file.exists():
        with urllib.request.urlopen(SOURCES["vcsl"]["tree"]) as response:
            tree_file.write_bytes(response.read())
    tree = [e["path"] for e in json.loads(tree_file.read_text())["tree"] if e["type"] == "blob"]
    known = set(tree)
    missing = [f"{pack.id}: {path}" for pack in define(tree) if pack.source == "vcsl"
               for path, _ in pack.files.values() if path not in known]
    if missing:
        raise SystemExit("Not in VCSL:\n  " + "\n  ".join(missing))
    index = []
    OUT.mkdir(parents=True, exist_ok=True)
    for pack in define(tree):
        if args.only and pack.id not in args.only:
            index.extend(i for i in json.loads((OUT / "packs.json").read_text()) if i["id"] == pack.id)
            continue
        folder = OUT / pack.id
        shutil.rmtree(folder, ignore_errors=True)
        (folder / "samples").mkdir(parents=True)
        audio = {}
        for sid, (path, _) in pack.files.items():
            left, right = decode(fetch(pack.source, path, args.cache))
            audio[sid] = (left, right, *analyse(left, right, pack.max_seconds))
        # One gain per pad: its loudest layer peaks at -1 dBFS and quieter
        # layers keep their order, at most 12 dB down; the pad's velocity
        # curve adds the rest, so every pad of a kit is usable as is.
        gains = {}
        for pd in pack.pads:
            loudest = max(audio[z["sample"]][4] for z in pd["zones"])
            for z in pd["zones"]:
                assert z["sample"] not in gains, f"{z['sample']} is on two pads"
                peak = audio[z["sample"]][4]
                lift = max(1.0, loudest * 10 ** (-12 / 20) / peak) if not pd["keytrack"] else 1.0
                gains[z["sample"]] = 10 ** (-1 / 20) / loudest * lift
        frames = {}
        for sid, (left, right, start, end, _, mono) in audio.items():
            capped = end - start >= int(pack.max_seconds * RATE)
            frames[sid] = write(folder / "samples" / f"{sid}.wav", left, right, start, end, gains[sid],
                                mono or pack.mono, capped)
        for pd in pack.pads:
            for z in pd["zones"]:
                z["end"] = frames[z["sample"]]
        kit = {"pads": pack.pads, "polyphony": 32, "selected": 1}
        manifest = {
            "format": "sad-pet-pack-v1", "name": pack.name, "description": pack.description,
            "license": SOURCES[pack.source]["license"], "kit": kit,
            "samples": [{"id": sid, "name": name, "file": f"samples/{sid}.wav"}
                        for sid, (_, name) in pack.files.items()],
        }
        (folder / "pack.json").write_text(json.dumps(manifest, ensure_ascii=False, indent=1) + "\n")
        (folder / "CREDITS.txt").write_text(credits(pack))
        digest = hashlib.sha256()
        size = 0
        for file in sorted(folder.rglob("*")):
            if file.is_file():
                digest.update(file.read_bytes())
                size += file.stat().st_size
        index.append({"id": pack.id, "name": pack.name, "description": pack.description,
                      "pads": len(pack.pads), "samples": len(pack.files), "size": size,
                      "version": digest.hexdigest()[:12]})
        print(f"{pack.id}: {len(pack.pads)} pads, {len(pack.files)} samples, {size / 1e6:.1f} MB")
    order = [p.id for p in define(tree)]
    index.sort(key=lambda i: order.index(i["id"]))
    (OUT / "packs.json").write_text(json.dumps(index, ensure_ascii=False, indent=1) + "\n")


if __name__ == "__main__":
    main()
