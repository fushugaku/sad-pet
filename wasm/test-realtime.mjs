// Actual compiled ABI, transferred buffer ownership and actual utility workers.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { Worker } from "node:worker_threads";
import { Core } from "../dist/web/core.mjs";
import { BrowserBackend } from "../dist/web/browser-backend.mjs";
const module = await WebAssembly.compile(
  await readFile("dist/web/sad_pet.wasm"),
);
const utility = new Core(module);
utility.call("sad_prepare", utility.call("sad_factory", undefined, 0));
const prepared = utility.call("sad_pack");
const workerUrl = new URL("../dist/web/worker.mjs", import.meta.url).href;
function worker() {
  const script = `import {parentPort} from 'node:worker_threads';globalThis.self={postMessage:(m,t=[])=>parentPort.postMessage(m,t)};await import(${JSON.stringify(workerUrl)});parentPort.on('message',data=>self.onmessage({data}));`;
  const w = new Worker(
    new URL(`data:text/javascript,${encodeURIComponent(script)}`),
    { type: "module" },
  );
  let next = 0;
  const pending = new Map(),
    events = [],
    waiting = [];
  w.on("message", (m) => {
    if (m.id !== undefined) {
      const p = pending.get(m.id);
      pending.delete(m.id);
      m.error ? p.reject(new Error(m.error)) : p.resolve(m.result);
    } else if (waiting.length) waiting.shift()(m);
    else events.push(m);
  });
  return {
    w,
    call: (type, data, path) =>
      new Promise((resolve, reject) => {
        const id = ++next;
        pending.set(id, { resolve, reject });
        w.postMessage({ id, type, data, path });
      }),
    event: () =>
      events.length
        ? Promise.resolve(events.shift())
        : new Promise((r) => waiting.push(r)),
  };
}
const telemetry = worker(),
  recorder = worker();
await telemetry.call("init", { module, rate: 48000, telemetry: true });
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
const send = (type, data = {}, path, extra = {}) =>
  processor.port.onmessage({ data: { id: ++id, type, data, path, ...extra } });
const audio = () =>
  processor.process([], [[new Float32Array(128), new Float32Array(128)]]);
const takeSnapshot = () => {
  const i = messages.findIndex((m) => m.type === "snapshot");
  assert.ok(i >= 0);
  return messages.splice(i, 1)[0];
};
async function importSnapshot(fail = false) {
  const m = takeSnapshot();
  if (fail) new Uint8Array(m.buffer, 0, 4).fill(0);
  telemetry.w.postMessage(m, [m.buffer]);
  const result = await telemetry.event();
  assert.equal(!!result.error, fail);
  send("snapshot-return", result.buffer, undefined, {
    sequence: result.sequence,
    success: !result.error,
  });
  return result;
}
send("load", { prepared: prepared.slice(0), generation: 1 });
for (let i = 0; i < 3; i++) {
  await importSnapshot(true);
  processor.snapshot();
}
const good = await importSnapshot();
assert.equal(good.state.sample_name, utility.request("/api/state").sample_name);
assert.equal(
  processor.snapshotPool.length,
  3,
  "Import errors lost telemetry buffers",
);
const snapshot = processor.core.snapshot.bind(processor.core);
processor.core.snapshot = () => {
  throw new Error("overflow");
};
processor.telemetryFrames = 48000;
assert.equal(audio(), true);
assert.equal(processor.snapshotPool.length, 3);
processor.core.snapshot = snapshot;
processor.core.wasm.memory.grow(1);
assert.equal(audio(), true);
assert.equal(
  processor.core.processed.buffer,
  processor.core.wasm.memory.buffer,
);

for (let cycle = 0; cycle < 10; cycle++) {
  await recorder.call("record-start", { id: `take${cycle}`, rate: 48000 });
  send("record-start");
  assert.equal(audio(), true);
  send("record-stop");
  while (messages.some((m) => m.type === "snapshot")) await importSnapshot();
  for (const m of messages.splice(0).filter((m) => m.type === "record-chunk")) {
    recorder.w.postMessage(
      { type: "record-chunk", data: m.buffer, length: m.length },
      [m.buffer],
    );
    const returned = await recorder.event();
    assert.equal(returned.type, "record-return");
    send("record-return", returned.buffer);
  }
  const saved = await recorder.call("record-stop");
  assert.equal(saved.frames, 128);
  assert.equal(
    processor.recordPool.length + (processor.recordBuffer ? 1 : 0),
    4,
    "Repeated takes depleted the recording pool",
  );
}

// Source cache is uploaded incrementally while the old source keeps playing.
const preset = utility.request("/api/preset"),
  defaults = utility.request("/api/color-defaults");
preset.source_color_chain = [
  { ...structuredClone(defaults.slot), id: 90, kind: "waveshaper", mix: 1 },
];
preset.source_color_chain[0].settings.waveshaper_type = "hard_clip";
preset.source_color_chain[0].settings.waveshaper_depth = 0.8;
const region = { start: 0.25, end: 0.5 };
send("request", preset, "/api/preset");
send("request", region, "/api/region");
send("request", { playing: true, midi_gate: false }, "/api/transport");
const recipe = {
  region,
  slot: preset.source_color_chain[0],
  bpm: 120,
  root: 60,
};
const cache = utility.call("sad_source_prepare", recipe),
  n = new DataView(cache).getUint32(8, true),
  header = JSON.parse(new TextDecoder().decode(new Uint8Array(cache, 12, n)));
send("source-load", {
  recipe,
  header,
  prepared: cache,
  key: "source-A",
  generation: 1,
  offset: 12 + n,
});
let completed = false;
for (let i = 0; i < 1000; i++) {
  assert.equal(audio(), true);
  if (
    messages.some(
      (m) =>
        m.type === "source-ready" && m.key === "source-A" && m.status === 1,
    )
  ) {
    completed = true;
    break;
  }
}
assert.ok(completed, "Prepared source was never published");
assert.equal(processor.core.request("/api/state").source_fx.pending, false);
processor.core.process(128);
const stable = processor.core.processed;
processor.core.process(128);
assert.equal(
  processor.core.processed,
  stable,
  "Every block allocated a new output view",
);
send("source-load", {
  recipe,
  header,
  prepared: cache,
  key: "old-file",
  generation: 0,
  offset: 12 + n,
});
assert.equal(
  messages.findLast((m) => m.id === id).result.started,
  false,
  "A cache for an old file generation was accepted",
);
// Parallel selections must keep the utility/worklet file and generation in
// the same order, even when both files have exactly the same rate and length.
const loadWorker = worker();
await loadWorker.call("init", { module, rate: 48000 });
let firstStarted, releaseFirst;
const firstReady = new Promise((r) => {
    firstStarted = r;
  }),
  firstGate = new Promise((r) => {
    releaseFirst = r;
  });
const generations = [],
  loader = Object.create(BrowserBackend.prototype);
globalThis.location = { href: "http://127.0.0.1/" };
Object.assign(loader, {
  ready: Promise.resolve(),
  sourceGeneration: 1,
  microphone: { active: false, pending: false },
  worker: {
    async call(type, data, transfers, path) {
      if (type === "prepare") {
        generations.push(data.generation);
        if (generations.length === 1) {
          firstStarted();
          await firstGate;
        }
      }
      return loadWorker.call(type, data, path);
    },
  },
  audio: {
    async call(type, data) {
      send(type, data);
      const response = messages.findLast((m) => m.id === id);
      if (response.error) throw new Error(response.error);
      return response.result;
    },
  },
});
const wav = utility.call("sad_factory", undefined, 0);
const firstLoad = loader.fetch("/api/wav", {
  body: wav.slice(0),
  headers: { "X-File-Name": "First.wav" },
});
await firstReady;
const secondLoad = loader.fetch("/api/wav", {
  body: wav.slice(0),
  headers: { "X-File-Name": "Last.wav" },
});
await Promise.resolve();
await Promise.resolve();
assert.deepEqual(generations, [2], "Two file loads prepared concurrently");
releaseFirst();
assert.equal((await firstLoad).status, 200);
assert.equal(loader.loadingSource, true);
assert.equal((await secondLoad).status, 200);
assert.equal(loader.loadingSource, false);
assert.deepEqual(generations, [2, 3]);
assert.equal(loader.sourceGeneration, 3);
assert.equal(processor.core.request("/api/state").sample_name, "Last.wav");
send("source-load", {
  recipe: processor.core.request("/api/state").source_fx.recipe,
  header,
  prepared: cache,
  key: "previous-same-length-file",
  generation: 2,
  offset: 12 + n,
});
assert.equal(
  messages.findLast((m) => m.id === id).result.started,
  false,
  "Same-length file reused the previous generation",
);
// Incremental timbre construction must leave every sample of existing parts
// untouched, then start the new part at the same boundary as a synchronous add.
for (const rate of [48000, 96000, 192000]) for (const kind of ["granular", "drums"]) {
  globalThis.sampleRate = rate;
  const staged = new Processor({ processorOptions: { module } });
  const reference = new Core(module, rate);
  for (const core of [staged.core, reference]) {
    core.call("sad_load", prepared);
    core.request("/api/transport", { playing: true, midi_gate: false });
  }
  const requestId = ++id;
  staged.port.onmessage({ data: { id: requestId, type: "add-timbre", data: { kind } } });
  assert.ok(!messages.some(m => m.id === requestId), "Preparation blocked the message handler");
  let finished = false;
  for (let block = 0; block < 180; block++) {
    const channels = [new Float32Array(128), new Float32Array(128)];
    assert.equal(staged.process([], [channels]), true);
    const expected = reference.process(128);
    for (let frame = 0; frame < 128; frame++)
      for (let c = 0; c < 2; c++)
        assert.equal(channels[c][frame], expected[frame * 2 + c], `${rate}/${kind}: audio changed at ${block}/${frame}`);
    if (!finished && messages.some(m => m.id === requestId)) {
      const response = messages.findLast(m => m.id === requestId);
      assert.ok(!response.error, response.error);
      assert.equal(response.result.timbres.filter((t) => t.kind !== "master").length, 2);
      reference.request("/api/timbres/add", { kind });
      finished = true;
    }
  }
  assert.ok(finished, "Timbre preparation never completed");
}
globalThis.sampleRate = 48000;
{
  const staged = new Processor({ processorOptions: { module } });
  const adding = ++id;
  staged.port.onmessage({ data: { id: adding, type: "add-timbre", data: { kind: "drums" } } });
  staged.process([], [[new Float32Array(128), new Float32Array(128)]]);
  staged.port.onmessage({ data: { id: ++id, type: "request", path: "/api/timbres/reset", data: {} } });
  assert.match(messages.findLast(m => m.id === adding).error, /cancelled/);
  for (let block = 0; block < 80; block++)
    staged.process([], [[new Float32Array(128), new Float32Array(128)]]);
  assert.equal(staged.core.request("/api/timbres").timbres.filter((t) => t.kind !== "master").length, 1);
  const resumed = ++id;
  staged.port.onmessage({ data: { id: resumed, type: "add-timbre", data: { kind: "drums" } } });
  staged.process([], [[new Float32Array(128), new Float32Array(128)]]);
  staged.port.onmessage({ data: { type: "prepare-idle" } });
  assert.equal(messages.findLast(m => m.id === resumed).result.timbres.filter((t) => t.kind !== "master").length, 2);
}

// A meter-only message retains the preset object. Rejected deltas request a
// full resync, so a late message cannot lose a new preset or timbre selection.
const receiver = Object.create(BrowserBackend.prototype), acknowledgements = [];
Object.assign(receiver, {
  viewStamp: 0, stateWaiters: new Set(), scheduleSourceEffects() {},
  audio: { port: { postMessage: message => acknowledgements.push(message) } },
});
const savedPreset = { name: "Delta test" };
receiver.workerMessage({ type: "snapshot", delta: true, stamp: 1,
  state: { preset: savedPreset, timbres: { active: 0 }, waveform: [[0, 1]], input: { active: false } } });
receiver.workerMessage({ type: "snapshot", delta: true, stamp: 2,
  state: { timbres: { active: 0 }, peak: [0.2, 0.3], input: { seconds: 2 } } });
assert.equal(receiver.state.preset, savedPreset);
assert.equal(receiver.state.input.active, false);
receiver.workerMessage({ type: "snapshot", delta: true, stamp: 1,
  state: { preset: { name: "Stale" }, timbres: { active: 0 } } });
assert.equal(acknowledgements.at(-1).success, false);
assert.equal(receiver.state.preset, savedPreset);
receiver.workerMessage({ type: "snapshot", delta: true, stamp: 3,
  state: { timbres: { active: 1 } } });
assert.equal(acknowledgements.at(-1).success, false);
assert.equal(receiver.state.timbres.active, 0);

await Promise.all([
  telemetry.w.terminate(),
  recorder.w.terminate(),
  loadWorker.w.terminate(),
]);
console.log(
  "Realtime checks passed: telemetry failure/resync, transferable pools, repeated takes, memory.grow, incremental Source FX and parallel file-load isolation.",
);
