// Shipping WASM: independent Main pitch notes, live routing and block timing.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

const folder = resolve(process.argv[2] || "dist/web");
const variant = process.argv[3] || "sad_pet.wasm";
const { Core } = await import(pathToFileURL(resolve(folder, "core.mjs")));
const module = await WebAssembly.compile(await readFile(resolve(folder, variant)));
const rate = 16000;
const catalog = new Core(module, rate).request("/api/sequencer/catalog");
assert.equal(catalog.track.env_gate, true);
assert.equal(catalog.track.granular, true);
assert.equal(catalog.track.oscillator_mask, 2 ** 25 - 1);

function scene() {
  const core = new Core(module, rate);
  core.request("/api/timbres/add", { kind: "synth", classic: true });
  core.request("/api/timbres/update", { id: 0, muted: true });
  const preset = core.request("/api/preset");
  preset.color_chain = [];
  preset.pre_color_chain = [];
  preset.params.master_volume = 0.25;
  preset.params.filter = 1;
  preset.params.filter_env_depth = 0;
  Object.assign(preset.envs[0], { attack: 0, decay: 0, sustain: 1, release: 0.5 });
  preset.sequencer.enabled = true;
  preset.sequencer.tracks = [60, 72].map((pitch, index) => {
    const track = structuredClone(catalog.track);
    Object.assign(track, { id: index + 1, oscillator_mask: 1 << 8, granular: false });
    track.steps.forEach(step => Object.assign(step, { pitch, gate: 3 }));
    return track;
  });
  core.request("/api/preset", preset);
  core.request("/api/transport", { playing: true, midi_gate: true });
  return core;
}

function render(core, count, block) {
  const audio = new Float32Array(count * 2);
  for (let at = 0; at < count; at += block) {
    const n = Math.min(block, count - at);
    audio.set(core.process(n), at * 2);
  }
  assert.ok(audio.every(Number.isFinite));
  return audio;
}

const core = scene();
const sound = render(core, 2048, 128);
assert.ok(sound.some(v => Math.abs(v) > 0.001), "oscillator-only parties were silent");
assert.deepEqual(core.request("/api/state").notes, [60, 72], "Main pitch lanes did not own separate notes");
assert.deepEqual(sound, render(scene(), 2048, 511), "polyphony depends on audio block size");

const memory = core.wasm.memory.buffer;
core.request("/api/sequencer/edit", [
  { track: 1, field: 68, value: 0 }, // Synth OSC 1 off, only on the first lane.
  { track: 1, field: 84, value: 1 }, // Highest bit survives the f32 edit ABI.
  { track: 2, field: 57, value: 1 },
]);
render(core, 1024, 128);
const tracks = core.request("/api/preset").sequencer.tracks;
assert.equal(tracks[0].oscillator_mask, 1 << 24);
assert.equal(tracks[1].oscillator_mask, 1 << 8);
assert.equal(tracks[0].granular, false);
assert.equal(tracks[1].granular, true);
assert.equal(core.wasm.memory.buffer, memory, "live routing grew WASM memory");

core.request("/api/sequencer/edit", [{ track: 1, field: 8, value: 1 }]);
render(core, 128, 128);
assert.ok(core.request("/api/state").notes.includes(72), "muting one party released the other");

// A single file feeds both parties, with no oscillator selected. Test the
// audible pitches and note gates through the same WASM core as the browser.
function sineWav(hz, seconds) {
  const frames = Math.round(rate * seconds), bytes = Buffer.alloc(44 + frames * 2);
  bytes.write("RIFF", 0); bytes.writeUInt32LE(bytes.length - 8, 4);
  bytes.write("WAVEfmt ", 8); bytes.writeUInt32LE(16, 16);
  bytes.writeUInt16LE(1, 20); bytes.writeUInt16LE(1, 22);
  bytes.writeUInt32LE(rate, 24); bytes.writeUInt32LE(rate * 2, 28);
  bytes.writeUInt16LE(2, 32); bytes.writeUInt16LE(16, 34);
  bytes.write("data", 36); bytes.writeUInt32LE(frames * 2, 40);
  for (let i = 0; i < frames; i++)
    bytes.writeInt16LE(Math.round(Math.sin(2 * Math.PI * hz * i / rate) * 10000), 44 + i * 2);
  return bytes;
}
function spectrum(audio, hz) {
  let sin = 0, cos = 0;
  for (let i = 0; i < audio.length / 2; i++) {
    const phase = 2 * Math.PI * hz * i / rate;
    sin += audio[2 * i] * Math.sin(phase);
    cos += audio[2 * i] * Math.cos(phase);
  }
  return Math.hypot(sin, cos);
}
const utility = new Core(module, rate);
utility.call("sad_prepare", sineWav(220, 2));
const preparedFile = utility.call("sad_pack");
function fileGrainScene(midiGate, sparse = false, muteSecond = false) {
  const granular = new Core(module, rate);
  granular.call("sad_load", preparedFile);
  const preset = granular.request("/api/preset");
  preset.color_chain = [];
  preset.pre_color_chain = [];
  Object.assign(preset.params, { mix: 1, grains: sparse ? 0.2 : 0.9, grain_size: 0.8,
    grain_attack: 1, pitch: 0.5, cloud: 0, detune: 0.5, sample_root_note: 60,
    filter: 1, filter_env_depth: 0 });
  Object.assign(preset.envs[0], { attack: 0, decay: 0, sustain: 1, release: sparse ? 0.8 : 0 });
  preset.sequencer.enabled = true;
  preset.sequencer.tracks = [60, 72].map((pitch, index) => {
    const track = structuredClone(catalog.track);
    Object.assign(track, { id: index + 1, oscillator_mask: 0, granular: true, env_gate: true });
    track.config[3] = sparse ? (index === 0 ? 4 : 1) : 32;
    track.config[7] = sparse ? 55 : 100;
    track.config[8] = +(muteSecond && index === 1);
    track.steps.forEach(step => Object.assign(step, { pitch, gate: sparse ? 2 : 3 }));
    return track;
  });
  granular.request("/api/preset", preset);
  granular.request("/api/transport", { playing: true, midi_gate: midiGate });
  return granular;
}

for (const midiGate of [false, true]) {
  // At this density the common clock would launch just one initial grain,
  // leaving the fast party silent throughout its first 55%-length gate.
  const sparse = fileGrainScene(midiGate, true), baseline = fileGrainScene(midiGate, true, true);
  const memory = sparse.wasm.memory.buffer;
  const attack = render(sparse, rate / 20, 128);
  const onlyLow = render(baseline, rate / 20, 128);
  assert.deepEqual(attack, render(fileGrainScene(midiGate, true), rate / 20, 511),
    "file-grain note onsets depend on the audio block size");
  assert.equal(sparse.wasm.memory.buffer, memory, "file-grain note onsets grew WASM memory");
  const highAttack = spectrum(attack, 440), leakedHigh = spectrum(onlyLow, 440);
  assert.ok(highAttack > leakedHigh * 3 + 0.05,
    `the second party started without a grain: ${highAttack} vs ${leakedHigh}`);

  const granular = fileGrainScene(midiGate);
  render(granular, rate, 128);
  assert.deepEqual(granular.request("/api/state").notes, [60, 72]);
  const before = render(granular, rate / 2, 128);
  const low = spectrum(before, 220), high = spectrum(before, 440);
  assert.ok(low > 0.1 && high > 0.1, `file grains lost a pitch: ${low}/${high}`);
  granular.request("/api/sequencer/edit", [{ track: 1, field: 8, value: 1 }]);
  render(granular, rate / 2, 128);
  assert.ok(granular.request("/api/state").notes.includes(72));
  const after = render(granular, rate / 2, 128);
  const lowAfter = spectrum(after, 220), highAfter = spectrum(after, 440);
  assert.ok(lowAfter < low * 0.2 + 0.05 && highAfter > high * 0.2,
    `muting a grain party affected the other: ${low}/${high} -> ${lowAfter}/${highAfter}`);
}
console.log("Main pitch WASM: independent notes, oscillator and file-grain routing, exact mask bits, stable memory and block timing passed.");
