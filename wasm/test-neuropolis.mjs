// Run the actual shared Rust core and the site's AudioWorklet bridge.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { Core } from "../dist/web/core.mjs";
import { BrowserBackend } from "../dist/web/browser-backend.mjs";
const module = await WebAssembly.compile(
  await readFile("dist/web/sad_pet.wasm"),
);
const core = new Core(module, 48000),
  worker = new Core(module, 48000);
const catalog = core.request("/api/sequencer/catalog");
assert.equal(catalog.orders.length, 21);
assert.equal(catalog.scales.length, 49);
assert.ok(catalog.parameters.some((p) => p.id === "resynth_osc4_control1"));
assert.ok(!catalog.parameters.some((p) => p.id.startsWith("source_fx")));
const preset = core.request("/api/preset");
preset.sequencer.enabled = true;
preset.sequencer.tracks = Array.from({ length: 257 }, (_, i) => {
  const t = structuredClone(catalog.track);
  t.id = i + 1;
  t.name = `Lane ${i + 1}`;
  t.config[3] = i % 2 ? 2 : 1;
  t.target =
    i === 0
      ? { kind: "main_pitch" }
      : i === 1
        ? { kind: "oscillator", index: 3 }
        : { kind: "parameter", parameter: "filter" };
  t.amount = i < 2 ? 1 : 0;
  if (i === 0) t.pitch_oscillators = [true, false, true, false];
  t.steps.forEach((s, n) => {
    s.pitch = 60 + n;
    s.cv = (n - 8) * 500;
  });
  return t;
});
core.request("/api/preset", preset);
core.request("/api/transport", { playing: true, midi_gate: false });
for (let i = 0; i < 70; i++)
  assert.ok(core.process(128).every(Number.isFinite));
let state = core.request("/api/state");
assert.equal(state.sequencer.total, 257);
assert.equal(state.sequencer.tracks[0].step, 1);
assert.equal(state.sequencer.tracks[1].step, 0);
assert.equal(state.preset.params.sample_tune_semis, 0);
assert.equal(state.knobs.pitch.base, preset.params.pitch);
core.request("/api/sequencer/view", { offset: 193 });
let buffer = new Uint8Array(core.wasm.sad_snapshot_capacity());
let length = core.snapshot(buffer);
assert.ok(length > 131072);
let imported = worker.json(
  "sad_snapshot_import",
  buffer.slice(0, length),
).state;
assert.equal(imported.preset.sequencer.tracks.length, 257);
assert.equal(imported.sequencer.offset, 193);
core.wasm.sad_snapshot_ack(core.wasm.sad_snapshot_sequence(), 1);
core.request("/api/sequencer/edit", [
  { track: 256, stage: 5, field: 0, value: 77 },
  { track: 2, field: 3, value: 65 },
  { track: 1, field: 52, value: 1 },
  { track: 2, stage: 0, field: 9, value: 27 },
]);
core.request("/api/parameters", { filter: 0.38 });
length = core.snapshot(buffer);
assert.ok(length < 40000, `A scalar edit resent ${length} bytes`);
imported = worker.json("sad_snapshot_import", buffer.slice(0, length)).state;
assert.equal(imported.preset.sequencer.tracks[255].steps[5].pitch, 77);
assert.equal(imported.preset.sequencer.tracks[1].config[3], 65);
assert.deepEqual(imported.preset.sequencer.tracks[0].pitch_oscillators, [
  true,
  false,
  true,
  true,
]);
assert.equal(imported.preset.sequencer.tracks[1].steps[0].gate_override, 27);
assert.ok(Math.abs(imported.preset.params.filter - 0.38) < 1e-6);
core.wasm.sad_snapshot_ack(core.wasm.sad_snapshot_sequence(), 1);
const next = core.request("/api/preset");
next.sequencer.tracks.reverse();
next.sequencer.tracks = next.sequencer.tracks.filter((t) => t.id !== 3);
core.request("/api/preset", next);
core.request("/api/sequencer/view", { offset: 192 });
state = core.request("/api/state");
assert.equal(state.sequencer.total, 256);
assert.equal(
  state.sequencer.tracks.find((t) => t.id === 1).step,
  1,
  "Adding/removing tracks reset the main lane",
);
assert.equal(state.sequencer.tracks.find((t) => t.id === 2).step, 0);

let Processor;
globalThis.sampleRate = 48000;
globalThis.registerProcessor = (_, p) => (Processor = p);
const messages = [];
globalThis.AudioWorkletProcessor = class {
  constructor() {
    this.port = {
      postMessage: (m, transfer = []) =>
        messages.push(structuredClone(m, { transfer })),
    };
  }
};
await import("../dist/web/worklet.mjs");
const processor = new Processor({ processorOptions: { module } });
let id = 0;
const send = (path, data) =>
  processor.port.onmessage({ data: { id: ++id, type: "request", path, data } });
send("/api/preset", preset);
assert.ok(
  processor.snapshotPool.every(
    (b) => b.length >= processor.core.wasm.sad_snapshot_capacity(),
  ),
);
const first = messages.find((m) => m.type === "snapshot");
assert.ok(first.length > 131072);
assert.ok(
  first.stamp > 0,
  "Snapshot must identify its acknowledged configuration",
);
processor.port.onmessage({
  data: {
    type: "snapshot-return",
    data: first.buffer,
    sequence: first.sequence,
    success: true,
  },
});
send("/api/transport", { playing: true, midi_gate: false });
send("/api/sequencer/edit", [{ track: 1, stage: 6, field: 1, value: 8 }]);
const memory = processor.core.wasm.memory.buffer.byteLength;
for (let i = 0; i < 100; i++)
  assert.equal(
    processor.process([], [[new Float32Array(128), new Float32Array(128)]]),
    true,
  );
assert.equal(
  processor.core.wasm.memory.buffer.byteLength,
  memory,
  "Steady audio processing grew the heap",
);
assert.equal(messages.filter((m) => m.error).length, 0);

// A successful add-track reply can overtake telemetry decoding. A state reader
// must not return the previous one-track program and deselect the new track.
const backend = Object.create(BrowserBackend.prototype),
  posted = [];
globalThis.location = { href: "https://example.test/sad-pet/" };
backend.ready = Promise.resolve();
backend.context = { state: "running" };
backend.state = { preset: { sequencer: { tracks: [{ id: 1 }] } } };
backend.viewStamp = 1;
backend.stateWaiters = new Set();
backend.scheduleSourceEffects = () => {};
backend.snapshot = () => backend.state;
backend.audio = { stamp: 2, port: { postMessage: (m) => posted.push(m) } };
let responded = false;
const reading = backend.fetch("/api/state").then(async (response) => {
  responded = true;
  return response.json();
});
await new Promise((resolve) => setImmediate(resolve));
assert.equal(
  responded,
  false,
  "An acknowledged edit was overwritten by stale feedback",
);
assert.ok(posted.some((m) => m.type === "snapshot-now"));
backend.workerMessage({
  type: "snapshot",
  stamp: 1,
  state: { preset: { sequencer: { tracks: [{ id: 1 }] } } },
  buffer: new ArrayBuffer(8),
  sequence: 1,
});
assert.equal(responded, false);
backend.workerMessage({
  type: "snapshot",
  stamp: 2,
  state: { preset: { sequencer: { tracks: [{ id: 1 }, { id: 2 }] } } },
  buffer: new ArrayBuffer(8),
  sequence: 2,
});
assert.equal((await reading).preset.sequencer.tracks.length, 2);
backend.workerMessage({
  type: "snapshot",
  stamp: 1,
  state: { preset: { sequencer: { tracks: [{ id: 1 }] } } },
  buffer: new ArrayBuffer(8),
  sequence: 1,
});
assert.equal(
  backend.state.preset.sequencer.tracks.length,
  2,
  "An older view replaced the new track",
);
backend.context.state = "suspended";
backend.audio.stamp = 3;
backend.audio.call = async () => ({
  preset: { sequencer: { tracks: [{ id: 1 }, { id: 2 }, { id: 3 }] } },
});
assert.equal(
  (await (await backend.fetch("/api/state")).json()).preset.sequencer.tracks
    .length,
  3,
);
console.log(
  "Neuropolis WASM: 257 tracks, independent clocks, ID adoption, sparse edits and worklet buffers passed",
);
