// Exercise the compiled WASM, including the exact core/worklet used by the site.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { Core } from "./web/core.mjs";

const module = await WebAssembly.compile(
  await readFile(new URL("../dist/web/sad_pet.wasm", import.meta.url)),
);
assert.deepEqual(
  WebAssembly.Module.imports(module),
  [],
  "WASM must not need a server, WASI or native imports",
);
const utility = new Core(module);
const wav = utility.call("sad_factory", undefined, 0);
utility.call("sad_prepare", wav);
utility.call("sad_sample_name", {
  name: "Стекло 🐾",
  id: "factory:glass-sequence:v1",
});
const prepared = utility.call("sad_pack");

function audio(rate, block, preset) {
  const core = new Core(module, rate);
  assert.equal(core.json("sad_load", prepared).name, "Стекло 🐾");
  if (preset) core.request("/api/preset", preset);
  core.request("/api/transport", { midi_gate: false, playing: true });
  const result = new Float32Array(rate * 2);
  for (let offset = 0; offset < rate; ) {
    const n = Math.min(block, rate - offset);
    result.set(core.process(n), offset * 2);
    offset += n;
  }
  assert.ok(result.every(Number.isFinite));
  assert.ok(
    result.some((v) => Math.abs(v) > 0.01),
    "the compiled engine must produce audible output",
  );
  return result;
}
for (const rate of [44100, 48000]) {
  assert.deepEqual(
    audio(rate, 128),
    audio(rate, 511),
    "DSP must be independent of audio quantum size",
  );
}

const preset = utility.request("/api/preset");
preset.performance.mode = "sequencer";
preset.performance.steps[0].notes = [60, 64, 67, 72];
const entry = new Core(module);
entry.json("sad_load", prepared);
entry.request("/api/preset", preset);
entry.request("/api/step-entry", { step: 1, reset: true, clear: true });
for (const note of [48, 52, 55])
  entry.request("/api/note", {
    kind: "note_on",
    channel: 0,
    note,
    velocity: 100,
  });
assert.deepEqual(entry.request("/api/state").step_edits[1].notes, [48, 52, 55]);
assert.equal(
  entry.request("/api/state").playing,
  false,
  "entering a chord must not start playback",
);
entry.request("/api/step-entry", { step: null });
entry.request("/api/transport", { playing: true });
entry.process(128);
assert.deepEqual(entry.request("/api/state").notes, [60, 64, 67, 72]);

const defaults = utility.request("/api/color-defaults");
preset.color_chain = ["distortion", "reverb", "lcr_delay", "mod_delay"].map(
  (kind, i) => ({ ...structuredClone(defaults.slot), id: i + 1, kind }),
);
const effected = audio(48000, 128, preset);
assert.notDeepEqual(
  effected,
  audio(48000, 128),
  "filters, chords and the effect stack must reach WASM audio",
);

const midi = new Core(module);
midi.json("sad_load", prepared);
midi.request("/api/midi-bytes", [0x90, 60, 100]);
midi.request("/api/midi-bytes", [64, 100, 0xf8, 67, 100]);
assert.deepEqual(midi.request("/api/state").notes, [60, 64, 67]);
midi.request("/api/midi-bytes", [60, 0, 64, 0, 67, 0]);
assert.deepEqual(midi.request("/api/state").notes, []);
assert.throws(() => midi.request("/api/preset", { format: "bad" }), /preset/);
assert.throws(() => midi.json("sad_load", new Uint8Array([1, 2, 3])), /sample/);

// Minimal worklet host: validate recording with varying quantum sizes and a
// partial final chunk. The synthesis and processor are the production modules.
globalThis.sampleRate = 48000;
let Processor;
globalThis.registerProcessor = (_name, implementation) => {
  Processor = implementation;
};
const messages = [];
globalThis.AudioWorkletProcessor = class {
  constructor() {
    this.port = {
      postMessage(message) {
        messages.push(message);
      },
    };
  }
};
await import("./web/worklet.mjs");
const processor = new Processor({ processorOptions: { module } });
const send = (type, data, path) =>
  processor.port.onmessage({ data: { id: 1, type, data, path } });
send("load", prepared);
send("request", { playing: true, midi_gate: false }, "/api/transport");
send("record-start");
let frameCount = 0;
for (const n of [128, 511, 4096, 1024, 128, 4096, 4096, 511, 256, 128, 4096]) {
  assert.equal(
    processor.process([], [[new Float32Array(n), new Float32Array(n)]]),
    true,
  );
  frameCount += n;
}
send("record-stop");
const chunks = messages
  .filter((m) => m.type === "record-chunk")
  .map((m) => new Float32Array(m.buffer));
assert.equal(
  chunks.reduce((n, c) => n + c.length / 2, 0),
  frameCount,
);
assert.ok(chunks.some((c) => c.some((v) => Math.abs(v) > 0.01)));
assert.ok(!messages.some((m) => m.error || m.type === "fatal"));
console.log(
  "WASM checks passed: portable module, stereo audio at 44.1/48 kHz, deterministic quanta, chord entry, MIDI, effect stack and sample-exact recording.",
);
