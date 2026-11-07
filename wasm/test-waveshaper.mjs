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
const colorDefaults = utility.request("/api/color-defaults");
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
  filter1_ws_before_keytrack: -0.5, filter2_ws_after_keytrack: 0.75,
  grain_group1_waveshaper_output: 0.6,
});
const saved = core.request("/api/preset");
assert.ok(Math.abs(saved.params.filter_shapers[0].before.options.input - 0.7) < 1e-6);
assert.ok(Math.abs(saved.params.filter_shapers[1].after.options.morph - 0.4) < 1e-6);
assert.equal(saved.params.filter_shapers[0].before.keytrack, -0.5);
assert.equal(saved.params.filter_shapers[1].after.keytrack, 0.75);
assert.ok(Math.abs(saved.grain_groups.groups[0].waveshaper_options.output - 0.6) < 1e-6);
assert.ok(saved.params.filter_shapers[1].after.options.points.every((value, i) => Math.abs(value - points[i]) < 1e-6));
const recalled = load(JSON.parse(JSON.stringify(saved))).request("/api/preset");
assert.deepEqual(recalled.params.filter_shapers, saved.params.filter_shapers);
assert.deepEqual(recalled.params.filter_order, saved.params.filter_order);

// Reproduce editing the unused B curve while playing. Changing its points
// must be audible as soon as B participates in the A/B mix.
function editCurveWhilePlaying(morph, zero) {
  const p = structuredClone(initial);
  p.params.filter_shapers[0].before = {
    enabled: true, mode: "decimator", depth: 0, mix: 1,
    options: { quality: "clean2x", curve: "soft_clip", curve_b: "custom", morph },
  };
  const c = load(p);
  c.request("/api/transport", { midi_gate: false, playing: true });
  for (let i = 0; i < 32; i++) c.process(128);
  const next = c.request("/api/preset");
  if (zero) next.params.filter_shapers[0].before.options.points.fill(0);
  c.request("/api/preset", next);
  const out = new Float32Array(8192);
  for (let i = 0; i < 32; i++) out.set(c.process(128), i * 256);
  return out;
}
assert.deepEqual(editCurveWhilePlaying(0, false), editCurveWhilePlaying(0, true),
  "At A, editing the unused B curve must preserve the audio");
assert.notDeepEqual(editCurveWhilePlaying(1, false), editCurveWhilePlaying(1, true),
  "At B, a live custom curve edit must change the audio");

function envelopePreset(source, amount, placement, otherLevel = 1) {
  const p = structuredClone(initial);
  p.sequencer.enabled = false;
  p.matrix = [];
  p.macros = [];
  p.lfos.forEach(l => Object.assign(l, { destination: "none", destination2: "none", destination3: "none",
    depth: 0, depth2: 0, depth3: 0 }));
  for (const [index, level] of [[1, 0], [2, otherLevel]]) Object.assign(p.envs[index], {
    attack: 0, decay: 0, sustain: level, depth: 0, extra_depth: 0, depth3: 0,
    destination: "none", extra_destination: "none", destination3: "none",
  });
  const stage = {
    enabled: true, mode: "decimator", depth: 0, mix: 1,
    options: { quality: "clean2x", curve: "soft_clip", envelope: amount, envelope_source: source },
  };
  if (placement === "main") p.params.filter_shapers[0].before = stage;
  else if (placement === "added") p.filter_slots = [{ id: 7, enabled: true, position: "after_filters", cutoff: 1,
    env_depth: 0, shapers: { after: stage } }];
  else if (placement === "grain") {
    p.params.mix = 1;
    p.grain_groups = { advanced: true,
      groups: Array.from({ length: 4 }, (_, index) => ({ share: index === 0 ? 1 : 0 })) };
    Object.assign(p.grain_groups.groups[0], { waveshaper: "decimator", waveshaper_depth: 0,
      waveshaper_options: stage.options });
  } else {
    const slot = { ...structuredClone(colorDefaults.slot), id: 91, kind: "waveshaper", enabled: true, mix: 1 };
    Object.assign(slot.settings, { waveshaper_type: "decimator", waveshaper_depth: 0,
      waveshaper_options: stage.options });
    p[placement === "pre" ? "pre_color_chain" : "color_chain"] = [slot];
  }
  return p;
}
for (const placement of ["main", "added", "pre", "post", "grain"]) {
  const env2 = envelopePreset("env2", 1, placement);
  const silentEnvelope = render(env2, 128);
  assert.notDeepEqual(silentEnvelope, render(envelopePreset("env3", 1, placement), 128),
    `${placement}: selecting a different live envelope must affect the WS`);
  assert.deepEqual(silentEnvelope, render(envelopePreset("env2", 1, placement, 0), 128),
    `${placement}: an unselected envelope must not modulate the WS`);
  assert.deepEqual(render(envelopePreset("env2", .5, placement), 128),
    render(envelopePreset("env3", .5, placement), 128),
    `${placement}: neutral ENV depth must be independent of the selected source`);
  const restoredCore = load(env2);
  if (placement === "added") restoredCore.request("/api/parameters", { filter3_ws_after_keytrack: -0.4 });
  const restored = restoredCore.request("/api/preset");
  const options = placement === "main" ? restored.params.filter_shapers[0].before.options
    : placement === "added" ? restored.filter_slots[0].shapers.after.options
    : placement === "grain" ? restored.grain_groups.groups[0].waveshaper_options
    : restored[placement === "pre" ? "pre_color_chain" : "color_chain"][0].settings.waveshaper_options;
  assert.equal(options.envelope_source, "env2", `${placement}: the selected ENV must survive recall`);
  if (placement === "added") assert.ok(Math.abs(restored.filter_slots[0].shapers.after.keytrack + .4) < 1e-6,
    "An added filter WS must accept its own numeric Keytrack");
}
const keytrackIds = utility.request("/api/sequencer/catalog").parameters
  .filter(p => /^filter(?:[1-9]|1[0-8])_ws_(?:before|after)_keytrack$/.test(p.id)).map(p => p.id);
assert.equal(new Set(keytrackIds).size, 36, "Both WS positions of all 18 filters need a Keytrack target");
console.log("Waveshaper WASM: routing, Clean quantum parity, live A/B curves, external envelopes and recall passed");
