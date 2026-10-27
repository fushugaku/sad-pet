// The shipping worker prepares the complete first-open scene for the worklet.
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFile } from "node:fs/promises";
import { Worker } from "node:worker_threads";
import { Core } from "../dist/web/core.mjs";

const module = await WebAssembly.compile(await readFile("dist/web/sad_pet.wasm"));
const workerUrl = new URL("../dist/web/worker.mjs", import.meta.url).href;
const script = `import {parentPort} from 'node:worker_threads';
  globalThis.self={postMessage:(m,t=[])=>parentPort.postMessage(m,t)};
  await import(${JSON.stringify(workerUrl)});
  parentPort.on('message',data=>self.onmessage({data}));`;
const worker = new Worker(new URL(`data:text/javascript,${encodeURIComponent(script)}`));
let next = 0;
const pending = new Map();
worker.on("message", ({ id, result, error }) => {
  const task = pending.get(id);
  pending.delete(id);
  error ? task.reject(new Error(error)) : task.resolve(result);
});
const call = (type, data, path) => new Promise((resolve, reject) => {
  const id = ++next;
  pending.set(id, { resolve, reject });
  worker.postMessage({ id, type, data, path });
});
const request = (path, data) => call("request", data, path);

try {
  await call("init", { module, rate: 48000 });
  const packed = await call("startup", { generation: 1 });
  const core = new Core(module, 48000);
  core.call("sad_scene_load", packed);
  let state = core.request("/api/state");
  assert.equal(state.preset.name, "glasses5");
  assert.equal(state.timbres.name, "glasses5");
  // The scene opens on its preset as a Rack of the modules its engine ran.
  assert.deepEqual(state.timbres.timbres.map((t) => t.kind), ["rack", "master"]);
  assert.deepEqual(state.preset.rack.modules.map((m) => m.module),
    ["sample", "grains", "resynth", "pre_fx", "filters", "post_fx"]);
  assert.equal(state.sample_id, "factory:dusty-keys:v1");
  assert.equal(state.sample_seconds, 12);
  assert.equal(state.source_generation, 1);
  assert.equal(state.startup_preset_index, 5);
  assert.equal(state.playing, false);
  assert.equal(state.midi_gate, true);
  assert.deepEqual(state.region, { start: 0, end: 1 });
  assert.deepEqual(state.fades, { fade_in: 1, fade_out: 1 });
  assert.equal(state.preset.resynth.count, 4);
  assert.equal(state.preset.pre_color_chain.length, 2);
  assert.equal(state.preset.color_chain.length, 5);
  assert.equal(state.preset.sequencer.tracks.length, 2);
  const catalog = core.request("/api/presets");
  assert.equal(catalog[0].name, "Init"); // Existing IDs stay compatible.
  assert.equal(catalog[5].name, "glasses5");
  assert.ok(catalog.every((preset) => preset.rack), "the factory presets are Racks");
  const expected = new Core(module, 48000);
  expected.request("/api/preset", catalog[5]);
  assert.deepEqual(state.preset, expected.request("/api/preset"));
  // Every setting of the supplied preset is the Rack's.
  const supplied = new Core(module, 48000);
  supplied.request("/api/preset", JSON.parse(await readFile("rust/presets/glasses5.json", "utf8")));
  const classic = supplied.request("/api/preset");
  for (const key of ["params", "lfos", "envs", "resynth", "pre_color_chain", "color_chain", "sequencer"])
    assert.deepEqual(state.preset[key], classic[key], key);

  // This is the PCM fingerprint of the sample inside the supplied glasses5.sadpet.
  const wav = Buffer.from(await call("sample"));
  let offset = 12;
  while (wav.toString("ascii", offset, offset + 4) !== "data")
    offset += 8 + wav.readUInt32LE(offset + 4) + (wav.readUInt32LE(offset + 4) % 2);
  const pcm = wav.subarray(offset + 8, offset + 8 + wav.readUInt32LE(offset + 4));
  assert.equal(createHash("sha256").update(pcm).digest("hex"),
    "82f94610bf03aafda164fa3418151034751707370f9a106fac8bf66bb1f21bb5");

  // Recalling it after another scene removes extra timbres and old live-input state.
  await request("/api/timbres/add", { kind: "drums" });
  await request("/api/timbres/select", { id: 0 });
  await request("/api/input", { active: true, mode: "effects" });
  core.call("sad_scene_load", await call("startup", { generation: 7 }));
  state = core.request("/api/state");
  assert.deepEqual(state.timbres.timbres.map((t) => t.kind), ["rack", "master"]);
  assert.equal(state.timbres.active, 0);
  assert.equal(state.input.active, false);
  assert.equal(state.source_generation, 7);

  core.request("/api/transport", { playing: true, midi_gate: false });
  let peak = 0;
  for (let i = 0; i < 200; i++) {
    for (const value of core.process(128)) {
      assert.ok(Number.isFinite(value));
      peak = Math.max(peak, Math.abs(value));
    }
  }
  assert.ok(peak > 0.001, "The complete startup scene did not produce sound");

  // The microphone plays in the Rack's Sample, as in a Granular timbre.
  core.request("/api/input", { active: true, mode: "grains" });
  state = core.request("/api/state");
  assert.equal(state.input.active, true);
  const tone = [Float32Array.from({ length: 128 }, (_, i) => Math.sin(i * 0.2) * 0.4)];
  peak = 0;
  for (let i = 0; i < 400; i++)
    for (const value of core.process(128, tone)) peak = Math.max(peak, Math.abs(value));
  assert.ok(peak > 0.001, "The microphone did not play through the startup Rack");
  core.request("/api/input", { active: false });
  assert.equal(core.request("/api/state").input.active, false);
  console.log("Startup scene: glasses5, exact source PCM, full routing, quiet first open and factory recall passed.");
} finally {
  await worker.terminate();
}
