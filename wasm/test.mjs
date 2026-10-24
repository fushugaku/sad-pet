// Exercise the compiled WASM, including the exact core/worklet used by the site.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import vm from "node:vm";
import { Core } from "../dist/web/core.mjs";

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
// The editor must display the same fractional note times that reach the DSP.
const editorSource = await readFile(new URL("../rust/web/app.js", import.meta.url), "utf8");
const delayStart = editorSource.indexOf("const DELAY_NOTES = [");
const delayEnd = editorSource.indexOf("\nconst COLOR_SCHEMAS = {", delayStart);
assert.ok(delayStart > 0 && delayEnd > delayStart);
const delayView = vm.createContext({ colorDefaults: defaults, preset: { params: { bpm: 120 } } });
vm.runInContext(editorSource.slice(delayStart, delayEnd), delayView);
for (const prefix of ["lcr", "mod_delay"]) {
  const settings = structuredClone(defaults.slot.settings);
  const display = (note) => vm.runInContext(`delayTime(${note}, ${JSON.stringify({ settings })}, "${prefix}", true)`, delayView);
  assert.equal(settings[`${prefix}_smooth`], false, "Old presets must retain stepped delay times");
  assert.equal(display(7.75 / 13), "1/4 · 500 ms");
  settings[`${prefix}_smooth`] = true;
  assert.equal(display(7.5 / 13), "1/8.→1/4\n50% · 438 ms");
  assert.equal(display(8 / 13), "1/4\n500 ms");
  assert.equal(display(-1), "1/64\n31 ms");
  assert.equal(display(2), "1/1\n2000 ms");
}
// Dry/wet blends the source and grains upstream of all filters and FX.
const dryPreset = utility.request("/api/preset");
dryPreset.params.mix = 0;
dryPreset.params.filter_env_depth = 0;
dryPreset.params.grains = 0;
dryPreset.params.min_grains = 0;
dryPreset.color_chain = [];
function dryOutput(settings) {
  const core = new Core(module, 48000);
  core.json("sad_load", prepared);
  core.request("/api/preset", settings);
  core.request("/api/transport", { playing: true });
  const frames = new Float32Array(48000 * 2);
  for (let i = 0; i < 48000; i += 128)
    frames.set(core.process(Math.min(128, 48000 - i)), i * 2);
  assert.ok(frames.every(Number.isFinite));
  return frames.subarray(48000);
}
const signalEnergy = (frames) => frames.reduce((sum, v) => sum + v * v, 0);
const dryOpen = dryOutput(dryPreset);
dryPreset.params.filter = 0;
assert.ok(
  signalEnergy(dryOutput(dryPreset)) < signalEnergy(dryOpen) * 0.02,
  "0% wet must still pass through the filter in WASM",
);
dryPreset.params.filter = 1;
dryPreset.color_chain = [
  { ...structuredClone(defaults.slot), mix: 1, kind: "distortion" },
];
const dryFx = dryOutput(dryPreset);
assert.ok(
  dryFx.some((value, i) => Math.abs(value - dryOpen[i]) > 0.01),
  "0% wet must still reach the effect chain in WASM",
);
const compressor = {
  ...structuredClone(defaults.slot), mix: 1, kind: "compressor",
};
compressor.settings.compressor_threshold = 0.4;
compressor.settings.compressor_ratio = 1;
compressor.settings.compressor_attack = 0;
compressor.settings.compressor_knee = 0;
dryPreset.color_chain = [compressor];
assert.ok(
  signalEnergy(dryOutput(dryPreset)) < signalEnergy(dryOpen) * 0.25,
  "the stereo compressor must reduce above-threshold audio in WASM",
);
const metering = new Core(module, 48000);
metering.json("sad_load", prepared);
metering.request("/api/preset", dryPreset);
metering.request("/api/transport", { playing: true });
for (let i = 0; i < 48000; i += 128)
  metering.process(Math.min(128, 48000 - i));
const reductions = metering.request("/api/state").gain_reduction;
assert.ok(reductions[compressor.id] > 6, "WASM must publish actual compressor attenuation");
assert.deepEqual(metering.request("/api/state").gain_reduction, reductions,
  "reading the meter must not reset the compression envelope");
metering.request("/api/transport", { bypass: true });
assert.equal(metering.request("/api/state").gain_reduction[compressor.id], 0);
metering.request("/api/transport", { bypass: false, playing: false });
assert.equal(metering.request("/api/state").gain_reduction[compressor.id], 0);
compressor.settings.compressor_ratio = 0;
assert.deepEqual(dryOutput(dryPreset), dryOpen, "1:1 ratio must preserve audio");
preset.color_chain = [
  "distortion", "reverb", "lcr_delay", "mod_delay", "compressor",
].map((kind, i) => ({ ...structuredClone(defaults.slot), id: i + 1, kind }));
preset.color_chain[2].settings.lcr_smooth = true;
preset.color_chain[3].settings.mod_delay_smooth = true;
preset.lfos[0].destination = { color: { slot: 3, parameter: "lcr_left_note" } };
preset.lfos[0].depth = 0.12;
preset.envs[0].extra_destination = { color: { slot: 4, parameter: "mod_delay_right_note" } };
preset.envs[0].extra_depth = 0.15;
utility.request("/api/preset", preset);
const savedDelays = utility.request("/api/preset").color_chain;
assert.equal(savedDelays[2].settings.lcr_smooth, true);
assert.equal(savedDelays[3].settings.mod_delay_smooth, true);
const effected = audio(48000, 128, preset);
assert.deepEqual(
  effected, audio(48000, 511, preset), "FX must preserve block-size independence",
);
assert.notDeepEqual(
  effected,
  audio(48000, 128),
  "filters, chords and the effect stack must reach WASM audio",
);

// FX animation must use the audio owner's values after the worker imports a
// snapshot, including LFO 8 / ENV 8 and the pre-filter rack.
const fxAudio = new Core(module, 48000), fxWorker = new Core(module, 48000);
fxAudio.json("sad_load", prepared);
const fxPreset = fxAudio.request("/api/preset");
fxPreset.lfo_count = fxPreset.env_count = 8;
fxPreset.color_chain = [{ ...structuredClone(defaults.slot), id: 41, kind: "alpha_verb" }];
fxPreset.pre_color_chain = [{ ...structuredClone(defaults.slot), id: 42, kind: "super_echo" }];
fxPreset.color_chain[0].settings.alpha_verb_size = 0.5;
fxPreset.pre_color_chain[0].settings.super_echo_cutoff = 0.4;
Object.assign(fxPreset.lfos[7], {destination:{color:{slot:41,parameter:"alpha_verb_size"}},depth:0.4});
Object.assign(fxPreset.envs[7], {attack:0,decay:0,sustain:1,destination:{color:{slot:42,parameter:"super_echo_cutoff"}},depth:0.3});
fxAudio.request("/api/preset", fxPreset);
fxAudio.request("/api/midi-bytes", [0x90,60,127]);
const fxSnapshot = new Uint8Array(fxAudio.wasm.sad_snapshot_capacity());
let previousSize = NaN;
for (let i=0;i<4;i++) {
  fxAudio.process(2048);
  const length = fxAudio.snapshot(fxSnapshot);
  assert.ok(length > 0);
  const imported = fxWorker.json("sad_snapshot_import",fxSnapshot.slice(0,length)).state;
  const actual = fxAudio.request("/api/state");
  for (const key of ["color:41:alpha_verb_size","color:42:super_echo_cutoff"]) {
    assert.ok(imported.knobs[key].modulated, key);
    assert.deepEqual(imported.knobs[key],actual.knobs[key], `Stale worker FX feedback: ${key}`);
  }
  const size = imported.knobs["color:41:alpha_verb_size"].current;
  assert.notEqual(size, previousSize);
  previousSize = size;
  fxAudio.wasm.sad_snapshot_ack(fxAudio.wasm.sad_snapshot_sequence(),1);
}

const midi = new Core(module);
midi.json("sad_load", prepared);
midi.request("/api/midi-bytes", [0x90, 60, 100]);
midi.request("/api/midi-bytes", [64, 100, 0xf8, 67, 100]);
assert.deepEqual(midi.request("/api/state").notes, [60, 64, 67]);
midi.request("/api/midi-bytes", [60, 0, 64, 0, 67, 0]);
assert.deepEqual(midi.request("/api/state").notes, []);
assert.throws(() => midi.request("/api/preset", { format: "bad" }), /preset/);
const parameterState = midi.request("/api/state");
midi.request("/api/parameters", { position: 0.37, env1_attack: 0.12, lfo2_depth: -0.4 });
let numericState = midi.request("/api/state");
assert.ok(Math.abs(numericState.preset.params.position - 0.37) < 1e-6);
assert.ok(Math.abs(numericState.preset.envs[0].attack - 0.12) < 1e-6);
assert.ok(Math.abs(numericState.preset.lfos[1].depth + 0.4) < 1e-6);
assert.deepEqual(numericState.preset.matrix, parameterState.preset.matrix);
assert.throws(() => midi.request("/api/parameters", { position: 0.9, unknown: 0.3 }), /parameter/);
numericState = midi.request("/api/state");
assert.ok(Math.abs(numericState.preset.params.position - 0.37) < 1e-6,
  "Invalid numeric messages must not apply partial changes");
assert.throws(() => midi.json("sad_load", new Uint8Array([1, 2, 3])), /sample/);

// One bar is a scan clock per note, independent of pitch and the source length.
for (const note of [48, 60, 84]) {
  const loop = new Core(module, 48000);
  loop.json("sad_load", prepared);
  loop.request("/api/region", {start:0.25,end:0.75});
  loop.request("/api/parameters", {loop_one_bar:1,sample_root_note:60,bpm:120});
  loop.request("/api/midi-bytes", [0x90,note,100]);
  for (const pitch of [0.5,0.7]) {
    loop.request("/api/parameters", {pitch});
    for (let frame=0;frame<48000;frame+=128) loop.process(128);
    const state = loop.request("/api/state");
    assert.equal(state.preset.params.loop_one_bar, true);
    const expected = pitch === 0.5 ? 0.5 : 0.25;
    const distance = Math.abs(state.center - expected);
    assert.ok(Math.min(distance,Math.abs(distance-0.5)) < 1e-6,
      `One-bar WASM loop drifted at note ${note}, pitch ${pitch}: ${state.center}`);
  }
}

// Region commands run on the audio instance; trim/render run on the separate
// worker instance. Verify both use the same boundaries without altering a file.
const regionAudio = new Core(module, 48000);
regionAudio.json("sad_load", prepared);
const original = utility.call("sad_sample_wav");
const region = { start: 0.25, end: 0.5 };
regionAudio.request("/api/region", region);
regionAudio.request("/api/transport", { playing: true });
for (let i = 0; i < 200; i++) {
  regionAudio.process(511);
  const state = regionAudio.request("/api/state");
  assert.ok(state.center >= region.start && state.center < region.end);
  assert.ok(
    state.grains.every(
      (g) => g.position >= region.start && g.position < region.end,
    ),
  );
}
const trimmed = utility.call("sad_trim", {
  ...region,
  sample_id: "factory:glass-sequence:v1",
});
assert.deepEqual(
  utility.call("sad_sample_wav"),
  original,
  "trimming must preserve the original",
);
const trimmedFile = new Core(module, 48000);
trimmedFile.call("sad_prepare", trimmed);
assert.equal(trimmedFile.request("/api/state").sample_seconds, 2);
const pcm = (buffer) => {
  const view = new DataView(buffer);
  for (let i = 12; i + 8 <= buffer.byteLength; ) {
    const size = view.getUint32(i + 4, true);
    if (view.getUint32(i, false) === 0x64617461)
      return new Float32Array(buffer.slice(i + 8, i + 8 + size));
    i += 8 + size + (size % 2);
  }
  throw new Error("WAV has no audio data");
};
assert.deepEqual(
  pcm(trimmed),
  pcm(original).slice(48000 * 2 * 2, 48000 * 4 * 2),
);
const fades = { fade_in: 0.025, fade_out: 0.05 };
regionAudio.request("/api/fades", fades);
assert.deepEqual(regionAudio.request("/api/state").fades, fades);
const fadeTrim = utility.call("sad_trim", { ...region, fades });
const fadedPcm = pcm(fadeTrim);
assert.equal(fadedPcm.length, pcm(trimmed).length);
assert.ok(
  [fadedPcm[0], fadedPcm[1], fadedPcm.at(-1), fadedPcm.at(-2)].every(
    (v) => v === 0,
  ),
);
assert.equal(fadedPcm[10000], pcm(trimmed)[10000]);
assert.deepEqual(utility.call("sad_sample_wav"), original);
assert.throws(
  () => utility.call("sad_trim", { ...region, sample_id: "wrong-source" }),
  /source changed/,
);
assert.throws(
  () => utility.call("sad_trim", { start: 0.6, end: 0.4 }),
  /start/,
);
const renderedRegion = utility.call("sad_render", {
  seconds: 0.2,
  preset: utility.request("/api/preset"),
  bypass: true,
  frozen: false,
  region,
});
assert.ok(pcm(renderedRegion).some((v) => Math.abs(v) > 0.01));
const renderedFull = utility.call("sad_render", {
  seconds: 0.2,
  preset: utility.request("/api/preset"),
  bypass: true,
  frozen: false,
});
assert.notDeepEqual(
  pcm(renderedRegion),
  pcm(renderedFull),
  "offline renders must honor the selection",
);
regionAudio.json("sad_load", prepared);
assert.deepEqual(regionAudio.request("/api/state").region, {
  start: 0,
  end: 1,
});
assert.deepEqual(regionAudio.request("/api/state").fades, {
  fade_in: 0,
  fade_out: 0,
});

// Recorded/cover bank entries must work through the same prepare/load path as files,
// without a runtime request to the source video or a music service.
const catalog = utility.request("/api/samples");
assert.equal(catalog.length, 158, "the source bank must retain 30 sources and add 128");
assert.ok(catalog.every((track) => typeof track.category === "string" && track.category.length));
const bankPlayer = new Core(module, 48000);
for (const [index, track] of catalog.entries()) {
  if (index < 10) continue;
  const wave = utility.call("sad_factory", undefined, index);
  const samples = pcm(wave);
  assert.equal(samples.length, Math.round(track.seconds * 48000) * 2);
  assert.ok(samples.every((v) => Number.isFinite(v) && Math.abs(v) <= 0.8));
  assert.ok(signalEnergy(samples) / samples.length > 0.0004, `${track.id} must be audible`);
  utility.call("sad_prepare", wave);
  utility.call("sad_sample_name", { name: track.name, id: `factory:${track.id}:v1` });
  bankPlayer.json("sad_load", utility.call("sad_pack"));
  assert.equal(bankPlayer.request("/api/state").sample_name, track.name);
  bankPlayer.request("/api/transport", { playing: true, bypass: true });
  // Some percussion phrases open with a rest. Render through the first bar,
  // stopping at the first audible block rather than requiring a downbeat hit.
  const firstBar = Math.ceil(4 * 60 / (track.bpm || 60) * 48000);
  let audible = false;
  for (let frame = 0; frame < firstBar && !audible; frame += 4096)
    audible = bankPlayer.process(4096).some((v) => Math.abs(v) > 0.001);
  assert.ok(audible, `${track.id} must load and play`);
}
assert.ok(
  utility.wasm.memory.buffer.byteLength < 128 * 1024 * 1024,
  "exporting the bank must release sample buffers instead of retaining every track in mobile RAM",
);
for (const id of ["matrix-pov", "harvest-dawn-loop"]) {
  const index = catalog.findIndex((track) => track.id === id);
  assert.ok(index >= 0, `${id} must be in the built-in catalog`);
  utility.call("sad_prepare", utility.call("sad_factory", undefined, index));
  utility.call("sad_sample_name", {
    name: catalog[index].name,
    id: `factory:${id}:v1`,
  });
  const recorded = new Core(module, 48000);
  recorded.json("sad_load", utility.call("sad_pack"));
  const state = recorded.request("/api/state");
  assert.equal(state.sample_id, `factory:${id}:v1`);
  assert.equal(state.sample_seconds, catalog[index].seconds);
  assert.ok(state.waveform.some(([low, high]) => high - low > 0.01));
}

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
await import("../dist/web/worklet.mjs");
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
    true, JSON.stringify(messages.filter(m=>m.error||m.type==="fatal")),
  );
  frameCount += n;
}
send("record-stop");
const chunks = messages
  .filter((m) => m.type === "record-chunk")
  .map((m) => new Float32Array(m.buffer,0,m.length));
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

// A microphone-only take must contain the incoming signal, even with transport
// paused, and must duplicate mono capture without applying filters or volume.
messages.length = 0;
send("request", { playing: false }, "/api/transport");
send("record-start", { source: "input" });
const rawExpected = [];
for (const [n, stereo] of [
  [128, true],
  [511, false],
  [4096, true],
  [17, false],
]) {
  const left = Float32Array.from({ length: n }, (_, i) => ((i % 31) - 15) / 32);
  const right = stereo ? Float32Array.from(left, (v) => -v * 0.5) : left;
  processor.process(
    [stereo ? [left, right] : [left]],
    [[new Float32Array(n), new Float32Array(n)]],
  );
  for (let i = 0; i < n; i++) rawExpected.push(left[i], right[i]);
}
send("record-stop");
const rawActual = messages
  .filter((m) => m.type === "record-chunk")
  .flatMap((m) => Array.from(new Float32Array(m.buffer,0,m.length)));
assert.deepEqual(
  rawActual,
  rawExpected,
  "microphone takes must preserve exact input samples",
);
assert.ok(!messages.some((m) => m.error || m.type === "fatal"));
console.log(
  "WASM checks passed: 158 factory sources, bounded bank memory, stereo audio at 44.1/48 kHz, deterministic quanta, chord entry, MIDI, effect stack, live microphone, sample-exact recording, region loops and non-destructive trims/renders.",
);
