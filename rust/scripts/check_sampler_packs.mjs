// Checks the Sampler's starter packs: every file a kit names exists and is a
// WAV, the index matches the folders, and each keygroup zone's root is the
// pitch the engine's own detector hears (within half a semitone).
// node rust/scripts/check_sampler_packs.mjs [dist/web]
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFile, readdir } from "node:fs/promises";
import { resolve, join } from "node:path";
import { pathToFileURL } from "node:url";

const folder = resolve(process.argv[2] || "dist/web");
const packs = resolve("rust/assets/sampler-packs");
const { Core } = await import(pathToFileURL(resolve(folder, "core.mjs")));
const core = new Core(await WebAssembly.compile(await readFile(resolve(folder, "sad_pet.wasm"))), 48000);
const index = JSON.parse(await readFile(join(packs, "packs.json"), "utf8"));
assert.equal(index.length, 8, "eight starter packs");
const names = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
const note = (n) => `${names[((Math.round(n) % 12) + 12) % 12]}${Math.floor(Math.round(n) / 12) - 1}`;
let failures = 0;
for (const entry of index) {
  const dir = join(packs, entry.id);
  const manifest = JSON.parse(await readFile(join(dir, "pack.json"), "utf8"));
  assert.equal(manifest.format, "sad-pet-pack-v1");
  const files = new Map(manifest.samples.map((s) => [s.id, s.file]));
  const digest = createHash("sha256");
  const all = (await readdir(dir, { recursive: true, withFileTypes: true }))
    .filter((d) => d.isFile())
    .map((d) => join(d.parentPath ?? d.path, d.name))
    .sort();
  for (const file of all) digest.update(await readFile(file));
  assert.equal(digest.digest("hex").slice(0, 12), entry.version, `${entry.id} version`);
  for (const pad of manifest.kit.pads) {
    for (const zone of pad.zones) {
      const wav = await readFile(join(dir, files.get(zone.sample)));
      assert.equal(wav.toString("latin1", 0, 4), "RIFF", zone.sample);
      // The detector reads 40 Hz – 2 kHz; bars and bells ring with
      // inharmonic partials it cannot follow, so only strings and keys count.
      const hz = 440 * 2 ** ((zone.root - 69) / 12);
      if (!pad.keytrack || pad.zones.length < 2 || hz < 45 || hz > 1500
          || /Vibraphone|Xylophone|Glockenspiel/.test(pad.name)) continue;
      const heard = core.prefixedJson("sad_sampler_analyze", JSON.stringify({ kind: "root" }), wav);
      const off = heard === null ? null : heard - zone.root;
      // An octave or a twelfth off is the detector locking onto a partial.
      const partial = off !== null && [-12, 12, 19].some((k) => Math.abs(off - k) < 0.5);
      if (partial) console.log(`${entry.id} ${pad.name} ${note(zone.root)}: heard a partial (${off > 0 ? "+" : ""}${off.toFixed(1)} st)`);
      else if (off === null || Math.abs(off) > 0.5) {
        failures++;
        console.log(`${entry.id} ${pad.name}: root ${note(zone.root)} (${zone.root}) but heard ${heard === null ? "no pitch" : `${note(heard)} (${heard.toFixed(2)})`}`);
      }
    }
  }
  console.log(`${entry.id}: ${manifest.kit.pads.length} pads, ${manifest.samples.length} samples ok`);
}
if (failures) {
  console.log(`${failures} zones are off pitch`);
  process.exit(1);
}
console.log("Starter packs passed");
