// Exact before/after PCM for all filter models and cached gain changes.
// node wasm/benchmark-controls.mjs capture|compare target/perf/controls [engine.wasm]
import assert from "node:assert/strict";
import { readFile, writeFile, mkdir } from "node:fs/promises";
import { Core } from "./web/core.mjs";

const [mode, folder, binary = "dist/web/sad_pet.wasm"] = process.argv.slice(2);
assert.ok(["capture", "compare"].includes(mode) && folder);
await mkdir(folder, { recursive: true });
const module = await WebAssembly.compile(await readFile(binary));
const utility = new Core(module);
utility.call("sad_prepare", utility.call("sad_factory", undefined, 0));
const sample = utility.call("sad_pack");
const models = ["legacy", "radias", "ladder_lp24", "ms20_lp12", "ms20_hp12",
  "pro_lp24", "sem", "multi_filter", "ladder_hp24", "ladder_bp24", "m_poly_lp24",
  "linear_lp12", "linear_lp24", "linear_hp12", "linear_hp24", "linear_bp12",
  "linear_bp24", "linear_notch12", "linear_notch24", "radias2"];

for (const rate of [44100, 48000, 96000]) {
  for (const [i, model] of models.entries()) {
    const core = new Core(module, rate), preset = core.request("/api/preset");
    core.call("sad_load", sample);
    preset.color_chain = [];
    preset.pre_color_chain = [];
    Object.assign(preset.params, {
      filter_model: model, filter2_model: models[(i + 7) % models.length],
      vcf_type: "comb", vcf2_type: "comb", filter: 0.62, filter2: 0.47,
      filter_resonance: 0.4, filter2_resonance: 0.6,
      filter_drive: 0.3, filter2_drive: 0.2, filter_morph: 0.26, filter2_morph: 0.65,
      filter_routing: ["single", "serial", "parallel"][i % 3],
      in_gain: -3.25, out_gain: 1.5, mix: 0.5,
    });
    core.request("/api/preset", preset);
    core.request("/api/transport", { playing: true, midi_gate: true });
    for (const note of [48, 60, 67, 72])
      core.request("/api/note", { kind: "note_on", channel: 0, note, velocity: 100 });
    const output = new Float32Array(128 * 128 * 2);
    for (let block = 0; block < 128; block++) {
      if (block === 8) core.request("/api/parameters", { in_gain: 7 });
      if (block === 16) core.request("/api/parameters", { out_gain: -4 });
      if (block === 24) core.request("/api/preset", preset);
      if (block === 32) core.request("/api/parameters", { in_gain: -3.25, out_gain: 1.5 });
      if (block === 64) core.request("/api/parameters", {
        filter: 0.42, filter2: 0.71, in_gain: -12, out_gain: 5,
      });
      output.set(core.process(128), block * 256);
    }
    assert.ok(output.every(Number.isFinite));
    assert.ok(output.some(value => value !== 0), `${model}: silent render`);
    const file = `${folder}/${rate}-${model}.f32`, bytes = Buffer.from(output.buffer);
    if (mode === "capture") await writeFile(file, bytes);
    else assert.equal(bytes.compare(await readFile(file)), 0, `PCM changed: ${file}`);
    console.log(`${rate} ${model}: ${output.length} samples ${mode}`);
  }
}
