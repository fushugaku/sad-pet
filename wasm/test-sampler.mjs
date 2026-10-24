// Sampler timbres in the shipping WASM: the worklet registry plays a kit
// from its sample pool, routes pads through Pad FX timbres, keeps pools in
// packed scenes and saved documents, and the utility worker edits, analyses
// and bounces samples on its copy of the scene.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
const folder = resolve(process.argv[2] || "dist/web");
const { Core } = await import(pathToFileURL(resolve(folder, "core.mjs")));
const module = await WebAssembly.compile(await readFile(resolve(folder, "sad_pet.wasm")));
const rate = 16000;
const core = new Core(module, rate),
  utility = new Core(module, rate);

/** A 32-bit float stereo WAV of a sine. */
function sine(hz, seconds, level = 0.5) {
  const frames = Math.round(rate * seconds);
  const buffer = new ArrayBuffer(44 + frames * 8),
    view = new DataView(buffer);
  const text = (o, v) => [...v].forEach((c, i) => view.setUint8(o + i, c.charCodeAt(0)));
  text(0, "RIFF");
  view.setUint32(4, buffer.byteLength - 8, true);
  text(8, "WAVE");
  text(12, "fmt ");
  view.setUint32(16, 16, true);
  view.setUint16(20, 3, true);
  view.setUint16(22, 2, true);
  view.setUint32(24, rate, true);
  view.setUint32(28, rate * 8, true);
  view.setUint16(32, 8, true);
  view.setUint16(34, 32, true);
  text(36, "data");
  view.setUint32(40, frames * 8, true);
  const data = new Float32Array(buffer, 44);
  for (let i = 0; i < frames; i++) data[i * 2] = data[i * 2 + 1] = Math.sin((2 * Math.PI * hz * i) / rate) * level;
  return buffer;
}
/** The float samples of a WAV's data chunk. */
function samples(buffer) {
  const view = new DataView(buffer);
  for (let at = 12; at + 8 <= buffer.byteLength; ) {
    const id = String.fromCharCode(...new Uint8Array(buffer, at, 4)),
      size = view.getUint32(at + 4, true);
    if (id === "data") return new Float32Array(buffer.slice(at + 8, at + 8 + size));
    at += 8 + size + (size & 1);
  }
  throw new Error("No WAV data");
}
const wavPeak = (buffer) => samples(buffer).reduce((m, v) => Math.max(m, Math.abs(v)), 0);
const peak = (seconds, target = core) => {
  let max = 0;
  for (let i = 0; i < Math.round((seconds * rate) / 128); i++)
    for (const x of target.process(128)) {
      assert.ok(Number.isFinite(x));
      max = Math.max(max, Math.abs(x));
    }
  return max;
};

utility.call("sad_prepare", utility.call("sad_factory", undefined, 0));
core.call("sad_load", utility.call("sad_pack"));
// Only the kit is heard.
core.request("/api/timbres/update", { id: 0, muted: true });
peak(0.2);

const context = core.request("/api/timbres/add", { kind: "sampler" });
const kit = context.timbres.find((t) => t.kind === "sampler");
assert.ok(kit, "a Sampler timbre");
assert.equal(context.active, kit.id);
let preset = core.request("/api/state").preset;
assert.equal(preset.sampler.pads.length, 16);
assert.ok(preset.bus && !preset.bus.granular && !preset.bus.filters, "the kit chain starts off");

// The worker prepares a sample; the worklet takes it into the kit's pool.
const prepared = utility.call("sad_sampler_prepare", sine(440, 0.3));
const loaded = core.prefixedJson("sad_sampler_sample", "tone", prepared, kit.id);
assert.equal(loaded.frames, Math.round(rate * 0.3));
assert.deepEqual(core.request("/api/sampler/state").view.loaded, ["tone"]);

const settings = structuredClone(preset.sampler);
settings.pads[0].zones = [{ sample: "tone" }];
settings.pads[1].zones = [{ sample: "tone", tune: 12 }];
core.request("/api/sampler/settings", { settings });
assert.equal(peak(0.1), 0, "silent until played");
core.request("/api/sampler/trigger", { pad: 1, velocity: 127 });
const played = peak(0.1);
assert.ok(played > 0.4 && played < 0.55, `pad 1 plays the tone at its level: ${played}`);
peak(0.4);
assert.equal(peak(0.1), 0, "a one-shot ends with its sample");
// MIDI picks pads by key: C2 (36) is pad 1, 37 pad 2.
core.request("/api/note", { kind: "note_on", channel: 0, note: 37, velocity: 127 });
assert.ok(peak(0.05) > 0.3, "a note plays its pad");
peak(0.5);
let hits = Object.fromEntries(core.request("/api/sampler/state").view.hits);
assert.equal(hits[1], 1);
assert.equal(hits[2], 1);

// A Pad FX timbre takes pad 1's voices and returns them to the kit.
let chained = core.request("/api/sampler/chain", { pad: 1 });
const fx = chained.timbres.find((t) => t.kind === "pad_fx");
assert.ok(fx && fx.owner === kit.id && fx.pad === 1, "the pad's chain");
assert.equal(chained.active, fx.id);
const fxPreset = core.request("/api/state").preset;
fxPreset.params.master_volume = 0.25;
core.request("/api/preset", fxPreset);
assert.equal(core.request("/api/state").timbres.timbres.find((t) => t.id === fx.id).kind, "pad_fx", "a loaded preset keeps the Pad FX role");
core.request("/api/timbres/select", { id: kit.id });
assert.deepEqual(core.request("/api/sampler/state").chains.map((c) => c.pad), [1]);
// The chain's level glides to its new value.
peak(0.3);
core.request("/api/sampler/trigger", { pad: 1, velocity: 127 });
const through = peak(0.1);
assert.ok(Math.abs(through - played * 0.25) < 0.03, `pad 1 runs through its chain at a quarter: ${through}`);
peak(0.4);
core.request("/api/sampler/trigger", { pad: 2, velocity: 127 });
assert.ok(peak(0.05) > 0.4, "other pads skip the chain");
peak(0.4);

// Packed scenes carry each kit's pool; the worker's copy can bounce.
const packed = core.call("sad_scene_pack");
const copy = new Core(module, rate);
copy.call("sad_scene_load", packed);
copy.request("/api/timbres/select", { id: kit.id });
assert.deepEqual(copy.request("/api/sampler/state").view.loaded, ["tone"]);
copy.request("/api/sampler/trigger", { pad: 2, velocity: 127 });
assert.ok(peak(0.05, copy) > 0.4, "the loaded scene plays its kit");

utility.call("sad_scene_adopt", packed);
const options = { low: 60, high: 72, step: 12, velocity: 127, hold: 0.1, tail: 1 };
const dry = utility.call("sad_sampler_bounce", { part: kit.id, pad: 2, note: 60, options });
assert.ok(Math.abs(wavPeak(dry) - 0.5) < 0.02, `a bounce of a plain pad is the pad: ${wavPeak(dry)}`);
const wet = utility.call("sad_sampler_bounce", { part: kit.id, pad: 1, note: 60, options });
assert.ok(Math.abs(wavPeak(wet) - 0.125) < 0.02, `a bounce runs through the pad's chain: ${wavPeak(wet)}`);

// Saved documents embed the kit's samples.
const document = utility.json("sad_scene_document");
const saved = [document, ...document.timbres].find((t) => t.meta.id === kit.id);
assert.equal(saved.kit_samples.length, 1);
assert.equal(saved.kit_samples[0].id, "tone");
assert.equal([document, ...document.timbres].find((t) => t.meta.kind === "pad_fx").meta.owner, kit.id);
const restored = new Core(module, rate);
restored.call("sad_scene_load", restored.call("sad_scene_prepare", document));
restored.request("/api/timbres/select", { id: kit.id });
assert.deepEqual(restored.request("/api/sampler/state").chains.map((c) => c.pad), [1]);
assert.deepEqual(restored.request("/api/sampler/state").view.loaded, ["tone"]);

// Sample editor edits and analysis.
const header = (request) => JSON.stringify(request);
const louder = utility.prefixedCall("sad_sampler_process", header({ op: "normalize", start: 0, end: 0, peak: 0 }), sine(440, 0.1, 0.25));
assert.ok(Math.abs(wavPeak(louder) - 1) < 1e-3, "normalized to 0 dB");
const halves = utility.prefixedJson("sad_sampler_analyze", header({ kind: "divide", count: 2 }), sine(440, 0.1));
assert.deepEqual(halves, [0, 800]);
const root = utility.prefixedJson("sad_sampler_analyze", header({ kind: "root" }), sine(440, 0.3));
assert.ok(Math.abs(root - 69) < 0.2, `440 Hz is A4: ${root}`);
const clip = sine(220, 0.05);
const target = new Uint8Array(sine(440, 0.1));
const both = new Uint8Array(target.length + clip.byteLength);
both.set(target);
both.set(new Uint8Array(clip), target.length);
const pasted = utility.prefixedCall("sad_sampler_process", header({ op: "paste", at: 100, mix: false, clip: clip.byteLength }), both);
assert.equal(samples(pasted).length / 2, 1600 + 800);

// A Neuropolis track plays a pad whatever its step pitch.
preset = core.request("/api/state").preset;
const catalog = core.request("/api/sequencer/catalog");
const track = structuredClone(catalog.track);
track.id = 9;
track.target = { kind: "pad", pad: 2 };
track.steps.forEach((s) => (s.pitch = 90));
preset.sequencer = { enabled: true, tracks: [track] };
core.request("/api/preset", preset);
core.request("/api/transport", { playing: true });
peak(1.5);
core.request("/api/transport", { playing: false });
hits = Object.fromEntries(core.request("/api/sampler/state").view.hits);
assert.ok(hits[2] >= 3, `the pad track played pad 2: ${hits[2]}`);

// Removing the Sampler removes its Pad FX timbres.
const after = core.request("/api/timbres/remove", { id: kit.id });
assert.ok(!after.timbres.some((t) => t.kind === "pad_fx" || t.kind === "sampler"));
console.log("Sampler WASM tests passed");
