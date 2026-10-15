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

// Live input must replace the file, work with mono capture, and reach both
// the filters/effects and the granular engine at real AudioWorklet block sizes.
for (const rate of [44100, 48000]) {
  for (const mode of ["effects", "grains"]) {
    const live = new Core(module, rate);
    live.json("sad_load", prepared);
    live.request("/api/input", { active: true, mode });
    live.request("/api/transport", { playing: true, midi_gate: false });
    let peak = 0;
    for (let offset = 0; offset < rate * 2; offset += 128) {
      const n = Math.min(128, rate * 2 - offset);
      const input = Float32Array.from(
        { length: n },
        (_, i) => Math.sin((offset + i) * 0.06) * 0.2,
      );
      const output = live.process(n, [input]);
      assert.ok(output.every(Number.isFinite));
      if (offset > rate)
        for (const value of output) peak = Math.max(peak, Math.abs(value));
    }
    assert.ok(peak > 0.01, `${mode} must process incoming audio at ${rate} Hz`);
    assert.equal(live.request("/api/state").input.seconds, 2);
    const capture = live.call("sad_input_wav");
    live.request("/api/transport", { frozen: true });
    live.process(128, [new Float32Array(128).fill(0.9)]);
    assert.deepEqual(
      live.call("sad_input_wav"),
      capture,
      "Freeze must retain input history",
    );
    live.request("/api/input", { active: false });
    assert.equal(live.request("/api/state").sample_name, "Стекло 🐾");
    assert.throws(() => live.call("sad_input_wav"), /off/);
  }
}

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
send("request", { active: true, mode: "effects" }, "/api/input");
const microphoneChannels = [new Float32Array(128).fill(0.1)];
const microphoneOutput = [[new Float32Array(128), new Float32Array(128)]];
assert.equal(processor.process([microphoneChannels], microphoneOutput), true);
assert.ok(microphoneOutput[0][0].some((v) => Math.abs(v) > 0.001));
send("capture-input");
assert.ok(messages.some((m) => m.result instanceof ArrayBuffer));
console.log(
  "WASM checks passed: portable module, stereo audio at 44.1/48 kHz, deterministic quanta, chord entry, MIDI, effect stack, live microphone processing/capture/freeze and sample-exact recording.",
);
