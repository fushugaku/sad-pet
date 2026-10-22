// Synth engine timbres in the shipping WASM: notes, oscillator edits, the
// granular stage after the VCA, presets and whole-scene recall. No audio device.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
const folder = resolve(process.argv[2] || "dist/web");
const { Core } = await import(pathToFileURL(resolve(folder, "core.mjs")));
const module = await WebAssembly.compile(
  await readFile(resolve(folder, "sad_pet.wasm")),
);
const rate = 16000;
const utility = new Core(module, rate),
  core = new Core(module, rate);
utility.call("sad_prepare", utility.call("sad_factory", undefined, 0));
core.call("sad_load", utility.call("sad_pack"));
const context = core.request("/api/timbres/add", { kind: "synth" });
const synthId = context.active;
const part = context.timbres.find((t) => t.id === synthId);
assert.equal(part.kind, "synth");
assert.equal(part.name, "Synth 1");
core.request("/api/timbres/update", { id: 0, muted: true });
let state = core.request("/api/state");
assert.equal(state.sample_name, "Synth engine");
assert.equal(state.midi_gate, true, "A new synth waits for notes");
assert.equal(state.preset.synth.count, 1);
assert.equal(state.preset.synth.oscillators[0].waveform, "saw");
assert.equal(state.preset.synth.granular, false);
const peakOf = (blocks) => {
  let peak = 0;
  for (let i = 0; i < blocks; i++)
    for (const x of core.process(128)) {
      assert.ok(Number.isFinite(x));
      peak = Math.max(peak, Math.abs(x));
    }
  return peak;
};
core.process(4096);
assert.ok(peakOf(20) < 1e-5, "The synth sounded before a note");
core.request("/api/note", {
  kind: "note_on",
  channel: 0,
  note: 57,
  velocity: 110,
});
assert.ok(peakOf(60) > 0.01, "A note did not play the oscillator");
core.request("/api/parameters", {
  synth_count: 2,
  synth_osc2_waveform: 1,
  synth_osc2_transpose: 12,
  synth_osc1_mode: 2,
});
assert.ok(peakOf(20) > 0.01);
state = core.request("/api/state");
assert.equal(state.preset.synth.count, 2);
assert.equal(state.preset.synth.oscillators[1].waveform, "pulse");
assert.equal(state.preset.synth.oscillators[1].transpose, 12);
assert.equal(state.preset.synth.oscillators[0].mode, "unison");

const memory = core.wasm.memory.buffer.byteLength;
core.request("/api/parameters", {
  synth_granular: 1,
  synth_buffer_seconds: 1,
  grains: 0.9,
  position: 0,
});
assert.ok(peakOf(160) > 0.01, "The granular stage is silent");
state = core.request("/api/state");
assert.ok(state.active_grains > 0, "No grains read the synth history");
assert.equal(state.sample_seconds, 1);
assert.ok(
  state.waveform.some((bin) => bin.some((value) => Math.abs(value) > 0.001)),
  "The live synth waveform is missing",
);
assert.equal(
  core.wasm.memory.buffer.byteLength,
  memory,
  "Rendering grew WASM memory",
);
core.request("/api/note", {
  kind: "note_off",
  channel: 0,
  note: 57,
  velocity: 0,
});
peakOf(8);

utility.call("sad_scene_load", core.call("sad_scene_pack"));
const scene = utility.json("sad_scene_document");
assert.deepEqual(scene.timbres.map((t) => t.meta.kind), ["synth", "master"]);
assert.equal(scene.timbres[0].preset.synth.count, 2);
assert.equal(scene.timbres[0].preset.synth.granular, true);
assert.equal(scene.preset.synth, undefined, "A granular preset gained a synth");
const recalled = new Core(module, rate);
recalled.call("sad_scene_load", utility.call("sad_scene_prepare", scene));
const restored = recalled.request("/api/state");
assert.equal(restored.timbres.active, synthId);
assert.equal(restored.timbres.timbres[1].kind, "synth");
assert.equal(restored.sample_name, "Synth engine");
assert.equal(restored.preset.synth.oscillators[1].transpose, 12);
const saved = recalled.request("/api/preset");
saved.synth.oscillators[2].waveform = "formant";
saved.synth.count = 3;
assert.equal(recalled.request("/api/preset", saved).synth.count, 3);
recalled.request("/api/note", {
  kind: "note_on",
  channel: 0,
  note: 64,
  velocity: 100,
});
let recalledPeak = 0;
for (let i = 0; i < 160; i++)
  for (const x of recalled.process(128)) {
    assert.ok(Number.isFinite(x));
    recalledPeak = Math.max(recalledPeak, Math.abs(x));
  }
assert.ok(recalledPeak > 0.01, "The recalled synth is silent");
recalled.request("/api/timbres/select", { id: 0 });
const granular = recalled.request("/api/preset");
assert.equal(granular.synth, undefined);
// Loading a preset into a part follows its engine type, and back.
recalled.request("/api/preset", saved);
assert.equal(recalled.request("/api/state").timbres.timbres[0].kind, "synth");
recalled.request("/api/preset", granular);
assert.equal(recalled.request("/api/state").timbres.timbres[0].kind, "granular");
console.log(
  "Synth engine: note playback, oscillator parameters, granular history, preset and scene recall passed.",
);
