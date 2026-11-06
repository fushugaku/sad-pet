// Shipping WASM: independent Main pitch notes, live routing and block timing.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

const folder = resolve(process.argv[2] || "dist/web");
const { Core } = await import(pathToFileURL(resolve(folder, "core.mjs")));
const module = await WebAssembly.compile(await readFile(resolve(folder, "sad_pet.wasm")));
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
console.log("Main pitch WASM: independent notes, oscillator/grain routing, exact mask bits, stable memory and block timing passed.");
