// Exercise the actual shipping Rust/WASM audio engine, without audio devices.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { Core } from "../dist/web/core.mjs";
const module = await WebAssembly.compile(
  await readFile(new URL("../dist/web/sad_pet.wasm", import.meta.url)),
);
const utility = new Core(module);
utility.call("sad_prepare", utility.call("sad_factory", undefined, 0));
const prepared = utility.call("sad_pack");
const p = utility.request("/api/preset");
p.color_chain = [];
p.pre_color_chain = [];
p.source_color_chain = [];
Object.assign(p.params, {
  filter: 0.62,
  filter2: 0.47,
  filter_env_depth: 0,
  filter2_env_depth: 0,
  filter_resonance: 0.4,
  filter2_resonance: 0.6,
  filter_drive: 0.3,
  filter2_drive: 0.2,
  filter_morph: 0.26,
  filter2_morph: 0.65,
  mix: 0,
});
const models = [
  "legacy",
  "radias",
  "ladder_lp24",
  "ms20_lp12",
  "ms20_hp12",
  "pro_lp24",
  "sem",
  "multi_filter",
  "ladder_hp24",
  "ladder_bp24",
  "m_poly_lp24",
  "linear_lp12",
  "linear_lp24",
  "linear_hp12",
  "linear_hp24",
  "linear_bp12",
  "linear_bp24",
  "linear_notch12",
  "linear_notch24",
  "radias2",
];
function render(preset, rate, block) {
  const core = new Core(module, rate);
  core.json("sad_load", prepared);
  core.request("/api/preset", preset);
  core.request("/api/transport", { playing: true, midi_gate: false });
  const data = new Float32Array(8192 * 2);
  for (let offset = 0; offset < 8192;) {
    const n = Math.min(block, 8192 - offset);
    data.set(core.process(n), offset * 2);
    offset += n;
  }
  assert.ok(data.every(Number.isFinite));
  assert.ok(
    data.some((x) => Math.abs(x) > 1e-9),
    `Silent ${preset.params.filter_model}`,
  );
  return { core, data };
}
for (const rate of [44100, 48000, 96000]) {
  for (const [i, model] of models.entries()) {
    Object.assign(p.params, {
      filter_model: model,
      filter2_model: models[(i + 7) % models.length],
      vcf_type: "comb",
      vcf2_type: "comb",
      filter_routing: ["single", "serial", "parallel"][i % 3],
    });
    const { core, data } = render(p, rate, 128);
    assert.deepEqual(
      data,
      render(p, rate, 511).data,
      `Quantum-dependent ${model} at ${rate}`,
    );
    const memory = core.wasm.memory.buffer;
    for (let f = 0; f < 64; f++)
      assert.ok(core.process(128).every(Number.isFinite));
    assert.ok(core.wasm.memory.buffer === memory, "Audio grew WASM memory");
    core.request("/api/parameters", {
      filter_drive: 0.71,
      filter2_morph: 0.24,
      filter_model: i,
      filter2_model: 19 - i,
    });
    core.process(128);
    const state = core.request("/api/state");
    assert.equal(state.preset.params.filter_model, model);
    assert.equal(state.preset.params.filter2_model, models[19 - i]);
    assert.ok(Math.abs(state.preset.params.filter_drive - 0.71) < 1e-6);
    assert.ok(Math.abs(state.preset.params.filter2_morph - 0.24) < 1e-6);
    assert.ok(
      core.wasm.memory.buffer === memory,
      "Scalar filter edit grew WASM memory",
    );
  }
}
const catalog = utility.request("/api/sequencer/catalog");
assert.ok(JSON.stringify(catalog).includes("filter_morph"));
assert.ok(JSON.stringify(catalog).includes("filter2_drive"));
console.log(
  "Rustias filters: 20 model selections, 3 host rates, both slots, all routings, scalar recall, quantum invariance and no audio memory growth passed.",
);
