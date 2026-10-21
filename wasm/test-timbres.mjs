// Exercise the actual shipping WASM in discarded buffers, without an audio device.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
const folder = resolve(process.argv[2] || "dist/web");
const { Core } = await import(pathToFileURL(resolve(folder, "core.mjs")));
const module = await WebAssembly.compile(
  await readFile(resolve(folder, "sad_pet.wasm")),
);
const utility = new Core(module, 8000),
  core = new Core(module, 8000);
utility.call("sad_prepare", utility.call("sad_factory", undefined, 0));
core.call("sad_load", utility.call("sad_pack"));
const root = core.request("/api/preset");
assert.equal(root.lfos.length, 8);
assert.equal(root.envs.length, 8);
assert.equal(core.request("/api/timbres/add", { kind: "drums" }).active, 1);
core.request("/api/timbres/update", { id: 0, muted: true });
core.request("/api/drums/edit", [
  { track: 0, step: null, field: 0, value: 128 },
  { track: 0, step: null, field: 1, value: 6 },
  { track: 0, step: 127, field: 0, value: 0.9 },
]);
core.request("/api/parameters", { drum1_0: 73, drum1_12: 5000 });
core.process(4096); // Settle the muted, stopped root before measuring the sole drum hit.
core.request("/api/transport", { playing: true, midi_gate: false });
let peak = 0;
for (let i = 0; i < 101; i++)
  for (const x of core.process(128)) {
    assert.ok(Number.isFinite(x));
    peak = Math.max(peak, Math.abs(x));
  }
let state = core.request("/api/state");
assert.equal(state.preset.drums.tracks[0].length, 128);
assert.ok(peak < 0.00001, "The pattern sounded before its only enabled step");
assert.equal(state.drums.positions[0], 51); // At 8 kHz and 120 BPM a 1/64 step is 250 frames.
// Advance to the last step. Its sole hit must be reachable beyond the old 64-step limit.
for (let i = 0; i < 148; i++)
  for (const x of core.process(128)) {
    assert.ok(Number.isFinite(x));
    peak = Math.max(peak, Math.abs(x));
  }
state = core.request("/api/state");
assert.ok(state.drums.positions[0] >= 126);
assert.ok(peak > 0.001, "The 128th drum step did not sound");
const before = state.drums.positions[0];
core.request("/api/parameters", { drum1_0: 91 });
core.process(64);
assert.equal(
  core.request("/api/state").drums.positions[0],
  before,
  "A scalar parameter restarted the drum clock",
);
const capacity = core.wasm.sad_snapshot_capacity(),
  packet = new Uint8Array(capacity);
let size = core.snapshot(packet),
  sequence = core.wasm.sad_snapshot_sequence();
assert.ok(size > 0);
const decoded = utility.json("sad_snapshot_import", packet.slice(0, size));
assert.equal(decoded.state.timbres.active, 1);
assert.equal(decoded.state.preset.drums.tracks[0].steps.length, 128);
core.request("/api/timbres/select", { id: 0 });
core.wasm.sad_snapshot_ack(sequence, 1); // Ack must still reach the previous active part.
assert.equal(core.request("/api/preset").name, root.name);
core.request("/api/timbres/select", { id: 1 });
const prepared = core.call("sad_scene_pack");
utility.call("sad_scene_load", prepared);
const scene = utility.json("sad_scene_document");
assert.equal(scene.timbres.length, 1);
assert.equal(scene.active_timbre, 1);
assert.equal(scene.timbres[0].preset.drums.params[0][0], 91);
const imported = new Core(module, 8000);
imported.call("sad_scene_load", utility.call("sad_scene_prepare", scene));
assert.equal(imported.request("/api/state").timbres.active, 1);
assert.equal(
  imported.request("/api/preset").drums.tracks[0].steps[127].velocity,
  scene.timbres[0].preset.drums.tracks[0].steps[127].velocity,
);
imported.request("/api/parameters", { drum_granular: 1 });
imported.process(4096);
imported.request("/api/drums/edit", [
  { track: 0, step: 0, field: 0, value: 1 },
]);
imported.request("/api/transport", { playing: true, midi_gate: false });
let wet = 0;
const memory = imported.wasm.memory.buffer.byteLength;
for (let i = 0; i < 80; i++)
  for (const x of imported.process(128)) {
    assert.ok(Number.isFinite(x));
    wet = Math.max(wet, Math.abs(x));
  }
assert.ok(wet > 0.001, "Drums did not reach the granular engine");
assert.equal(
  imported.wasm.memory.buffer.byteLength,
  memory,
  "Rendering grew WASM memory",
);
imported.request("/api/timbres/remove", { id: 1 });
assert.equal(
  imported.request("/api/timbres/add", { kind: "granular" }).active,
  2,
  "A deleted identity was reused",
);
console.log(
  "Timbres: 128-step drum playback, independent controls, stable IDs, cross-part snapshot acknowledgements, whole-scene recall and granular routing passed.",
);

// Read-only scene export must not advance the worker past the audio context.
for (let i = 0; i < 4; i++)
  utility.call("sad_scene_adopt", imported.call("sad_scene_pack"));
const context = imported.request("/api/timbres/select", { id: 0 });
utility.request("/api/timbres/context", context);
assert.equal(utility.request("/api/state").timbres.active, 0);
