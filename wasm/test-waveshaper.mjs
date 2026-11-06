// Exercise the shipped core/host, including saved routing and numeric edits.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

const directory = resolve(process.env.SADPET_WASM_DIR || "dist/web");
const { Core } = await import(pathToFileURL(`${directory}/core.mjs`));
const module = await WebAssembly.compile(await readFile(`${directory}/sad_pet.wasm`));
const utility = new Core(module, 48000);
utility.call("sad_prepare", utility.call("sad_factory", undefined, 0));
const sample = utility.call("sad_pack");
const initial = utility.request("/api/preset");
initial.source_color_chain = [];
initial.pre_color_chain = [];
initial.color_chain = [];
initial.resynth.enabled = false;
Object.assign(initial.params, {
  mix: 0, filter: 0.4, filter2: 0.6, filter_env_depth: 0,
  filter2_env_depth: 0, filter_routing: "serial", filter_order: [0, 1],
});

function load(preset) {
  const core = new Core(module, 48000);
  core.call("sad_load", sample);
  core.request("/api/preset", preset);
  return core;
}
function render(preset, block) {
  const core = load(preset);
  core.request("/api/transport", { midi_gate: false, playing: true });
  const samples = new Float32Array(8192);
  for (let offset = 0; offset < 4096;) {
    const n = Math.min(block, 4096 - offset);
    samples.set(core.process(n), offset * 2);
    offset += n;
  }
  assert.ok(samples.every(Number.isFinite));
  assert.ok(samples.some(x => Math.abs(x) > 1e-4));
  return samples;
}

const before = structuredClone(initial);
before.params.filter_shapers[0].before = {
  enabled: true, mode: "hard_clip", depth: 0.8, mix: 1,
  options: { quality: "clean4x", curve: "radias", dc_block: true, auto_gain: true },
};
before.params.filter_shapers[0].after = {
  enabled: true, mode: "drive", depth: 0.3, mix: 0.5,
  options: { quality: "clean2x", curve: "soft_clip", morph: 0.2, curve_b: "diode" },
};
const audio = render(before, 128);
assert.deepEqual(audio, render(before, 511), "Clean DSP must not depend on the audio quantum");
const moved = structuredClone(before);
moved.params.filter_order = [1, 0];
assert.notDeepEqual(audio, render(moved, 128), "Filter reordering must change the real signal path");

const core = load(before);
const points = Array.from({ length: 17 }, (_, i) => Math.sin((i / 8 - 1) * Math.PI));
const preset = core.request("/api/preset");
preset.params.filter_shapers[1].after = {
  enabled: true, mode: "multi_sine", depth: 0.5, mix: 0.7,
  options: { quality: "clean4x", curve: "custom", points },
};
core.request("/api/preset", preset);
core.request("/api/parameters", {
  filter1_ws_before_input: 0.7, filter2_ws_after_morph: 0.4,
  grain_group1_waveshaper_output: 0.6,
});
const saved = core.request("/api/preset");
assert.ok(Math.abs(saved.params.filter_shapers[0].before.options.input - 0.7) < 1e-6);
assert.ok(Math.abs(saved.params.filter_shapers[1].after.options.morph - 0.4) < 1e-6);
assert.ok(Math.abs(saved.grain_groups.groups[0].waveshaper_options.output - 0.6) < 1e-6);
assert.ok(saved.params.filter_shapers[1].after.options.points.every((value, i) => Math.abs(value - points[i]) < 1e-6));
const recalled = load(JSON.parse(JSON.stringify(saved))).request("/api/preset");
assert.deepEqual(recalled.params.filter_shapers, saved.params.filter_shapers);
assert.deepEqual(recalled.params.filter_order, saved.params.filter_order);
console.log("Waveshaper WASM: routing, Clean quantum parity, controls and curve recall passed");
