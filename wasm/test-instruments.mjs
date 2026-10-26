// Instrument presets in the shipping WASM: the worker writes one timbre's
// state, prepares another timbre's new sound from it, and the worklet swaps
// that timbre in place while its mix and the other timbres stay.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
const folder = resolve(process.argv[2] || "dist/web");
const { Core } = await import(pathToFileURL(resolve(folder, "core.mjs")));
const module = await WebAssembly.compile(await readFile(resolve(folder, "sad_pet.wasm")));
const rate = 16000;
const core = new Core(module, rate),
  worker = new Core(module, rate);

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
const peak = (seconds) => {
  let max = 0;
  for (let i = 0; i < Math.round((seconds * rate) / 128); i++)
    for (const x of core.process(128)) {
      assert.ok(Number.isFinite(x));
      max = Math.max(max, Math.abs(x));
    }
  return max;
};
const preset = (id) => {
  core.request("/api/timbres/select", { id });
  return core.request("/api/state").preset;
};
/** What `/api/timbres/document` answers: the active timbre alone. */
const documentOf = (id) => {
  core.request("/api/timbres/select", { id });
  worker.call("sad_scene_adopt", core.call("sad_scene_pack"));
  return worker.json("sad_part_document");
};
/** What `/api/timbres/load` does: the worker prepares, the worklet swaps. */
const load = (id, state, name, generation = 9) => {
  worker.call("sad_scene_adopt", core.call("sad_scene_pack"));
  const prepared = worker.call("sad_instrument_prepare", { id, state, name, generation });
  return core.json("sad_instrument_load", prepared);
};
const timbre = (context, id) => context.timbres.find((t) => t.id === id);

worker.call("sad_prepare", worker.call("sad_factory", undefined, 0));
core.call("sad_load", worker.call("sad_pack"));
const sceneName = core.request("/api/timbres").name;

// The scene's own Granular timbre, with a setting of its own.
const grains = preset(0);
grains.params.mix = 0.375;
core.request("/api/preset", grains);
let context = core.request("/api/timbres/add", { kind: "synth" });
const synth = context.timbres.find((t) => t.kind === "synth");
core.request("/api/timbres/update", { id: synth.id, gain: 0.5, channel: 3 });

const granular = documentOf(0);
assert.equal(granular.version, 1);
assert.equal(granular.meta.kind, "granular");
assert.equal(granular.preset.params.mix, 0.375);
assert.deepEqual(granular.timbres, [], "a timbre's document holds only that timbre");
assert.ok(granular.sample_wav.length > 1000, "it carries its sample");

// Loading it into the Synth timbre makes that timbre Granular, under the
// preset's name, at its own level and MIDI channel and in its own place.
const ids = core.request("/api/timbres").timbres.map((t) => t.id);
context = load(synth.id, granular, "Grain Pad");
assert.deepEqual(context.timbres.map((t) => t.id), ids, "every timbre stays where it was");
const loaded = timbre(context, synth.id);
assert.deepEqual([loaded.kind, loaded.name, loaded.gain, loaded.channel], ["granular", "Grain Pad", 0.5, 3]);
assert.equal(context.name, sceneName, "the scene keeps its name");
let state = (core.request("/api/timbres/select", { id: synth.id }), core.request("/api/state"));
assert.equal(state.preset.params.mix, 0.375);
assert.equal(state.preset.name, "Grain Pad");
assert.equal(state.source_generation, 9, "Source FX for the timbre wait for its new sample");
assert.equal(timbre(context, 0).kind, "granular", "the other timbres are untouched");

// The loaded timbre plays on its own.
core.request("/api/timbres/update", { id: 0, muted: true });
core.request("/api/transport", { playing: true, midi_gate: false });
peak(0.3);
assert.ok(peak(0.5) > 0.001, "the loaded Granular timbre sounds");
core.request("/api/transport", { playing: false });

// A Synth sound into the scene's own timbre: the engine changes, the scene's
// name and the timbre's identity stay.
const synthSound = preset(synth.id);
context = core.request("/api/timbres/add", { kind: "synth" });
const second = context.timbres.find((t) => t.kind === "synth");
const bass = preset(second.id);
bass.params.mix = 0.625;
core.request("/api/preset", bass);
const synthDocument = documentOf(second.id);
assert.equal(synthDocument.meta.kind, "synth");
context = load(0, synthDocument, "Bass");
assert.deepEqual([timbre(context, 0).kind, timbre(context, 0).name], ["synth", "Bass"]);
assert.equal(context.name, sceneName);
assert.equal(preset(0).name, sceneName, "the scene's timbre carries the scene's name");
assert.equal(preset(0).params.mix, 0.625);
assert.equal(preset(synth.id).params.mix, synthSound.params.mix, "the earlier load stays");

// Bus and Master take no instruments, and a Bus is no instrument.
const master = context.timbres.find((t) => t.kind === "master");
assert.throws(() => load(master.id, synthDocument, "Lead"), /instrument timbre/);
context = core.request("/api/timbres/add", { kind: "bus" });
const bus = context.timbres.find((t) => t.kind === "bus");
assert.throws(() => load(second.id, documentOf(bus.id), "Bus sound"), /instrument preset/);
assert.equal(timbre(core.request("/api/timbres"), second.id).kind, "synth", "a refused load changes nothing");

// A Sampler's kit travels with it; its Pad FX timbres stay with the scene.
context = core.request("/api/timbres/add", { kind: "sampler" });
const kit = context.timbres.find((t) => t.kind === "sampler");
core.prefixedJson("sad_sampler_sample", "tone", worker.call("sad_sampler_prepare", sine(440, 0.3)), kit.id);
const pads = preset(kit.id).sampler;
pads.pads[0].zones = [{ sample: "tone" }];
core.request("/api/sampler/settings", { settings: pads });
const kitDocument = documentOf(kit.id);
assert.deepEqual(kitDocument.kit_samples.map((s) => s.id), ["tone"]);
context = load(second.id, kitDocument, "Tone kit");
assert.equal(timbre(context, second.id).kind, "sampler");
core.request("/api/timbres/select", { id: second.id });
assert.deepEqual(core.request("/api/sampler/state").view.loaded, ["tone"]);
peak(0.1);
core.request("/api/sampler/trigger", { pad: 1, velocity: 127 });
assert.ok(peak(0.1) > 0.3, "the loaded kit plays its pad");
peak(0.4);

core.request("/api/timbres/select", { id: kit.id });
context = core.request("/api/sampler/chain", { pad: 1 });
assert.ok(context.timbres.some((t) => t.kind === "pad_fx" && t.owner === kit.id));
context = load(kit.id, granular, "Not a kit");
assert.equal(timbre(context, kit.id).kind, "granular");
assert.ok(!context.timbres.some((t) => t.kind === "pad_fx"), "a kit's Pad FX timbres go when it stops being a Sampler");

// A Vocal sound asks the vocal worker to render it afresh.
core.json("sad_vocal_jobs");
context = core.request("/api/timbres/add", { kind: "vocal" });
const voice = context.timbres.find((t) => t.kind === "vocal");
const voiceDocument = documentOf(voice.id);
core.json("sad_vocal_jobs");
context = load(kit.id, voiceDocument, "Choir");
assert.equal(timbre(context, kit.id).kind, "vocal");
const jobs = core.json("sad_vocal_jobs");
assert.ok(jobs.some((j) => j.type === "job" && j.id === kit.id && j.fresh), "the new Vocal timbre is planned");
context = load(kit.id, granular, "Grains again");
assert.ok(core.json("sad_vocal_jobs").some((j) => j.type === "forget" && j.id === kit.id), "and forgotten when it stops singing");

// Scenes still save and recall every timbre after these loads.
worker.call("sad_scene_adopt", core.call("sad_scene_pack"));
const scene = worker.json("sad_scene_document");
const recalled = new Core(module, rate);
recalled.call("sad_scene_load", worker.call("sad_scene_prepare", scene));
assert.deepEqual(
  recalled.request("/api/timbres").timbres.map((t) => [t.id, t.kind, t.name]),
  core.request("/api/timbres").timbres.map((t) => [t.id, t.kind, t.name]),
);
console.log("instrument preset tests passed");
