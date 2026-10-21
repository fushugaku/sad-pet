// Exact PCM and render cost for one/two AlphaVerb instances, including tails.
// node wasm/benchmark-alphaverb.mjs capture target/perf/alphaverb [engine.wasm]
// node wasm/benchmark-alphaverb.mjs compare target/perf/alphaverb [engine.wasm]
import assert from "node:assert/strict";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { Core } from "./web/core.mjs";

const [mode, folder, binary = "dist/web/sad_pet.wasm"] = process.argv.slice(2);
assert.ok(["capture", "compare"].includes(mode) && folder);
await mkdir(folder, { recursive: true });
const module = await WebAssembly.compile(await readFile(binary));
const utility = new Core(module);
utility.call("sad_prepare", utility.call("sad_factory", undefined, 0));
const source = utility.call("sad_pack");
const timings = [];
for (const rate of [44100, 48000, 96000]) {
  for (const count of [1, 2]) {
    const core = new Core(module, rate);
    core.call("sad_load", source);
    const preset = core.request("/api/preset");
    const defaults = core.request("/api/color-defaults").slot;
    preset.source_color_chain = [];
    preset.pre_color_chain = [];
    preset.color_chain = Array.from({ length: count }, (_, i) => ({
      ...structuredClone(defaults), id: i + 1, kind: "alpha_verb", mix: 0.7,
    }));
    const start = performance.now();
    core.request("/api/preset", preset);
    const prepareMs = performance.now() - start;
    core.request("/api/transport", { playing: true, midi_gate: false });
    const blocks = Math.ceil(rate * 3 / 128);
    const output = new Float32Array(blocks * 256), times = [];
    const memory = core.wasm.memory.buffer;
    for (let block = 0; block < blocks; block++) {
      if (block === 100) core.request("/api/parameters", {
        fx1_alpha_verb_dirty: 1, fx1_alpha_verb_size: 0.72,
        fx1_alpha_verb_shimmer: 0.61, fx1_alpha_verb_dynamics: 1 / 3,
      });
      if (block === 200) core.request("/api/parameters", {
        fx1_alpha_verb_dynamics: 2 / 3, fx1_alpha_verb_feedback: 0.81,
      });
      if (block === 300) core.request("/api/parameters", {
        fx1_alpha_verb_dynamics: 1, fx1_alpha_verb_freeze: 1,
      });
      if (block === 400) core.request("/api/parameters", {
        fx1_alpha_verb_freeze: 0, fx1_alpha_verb_dirty: 0,
      });
      if (block === 500) core.request("/api/transport", { playing: false });
      const start = performance.now();
      const samples = core.process(128);
      times.push(performance.now() - start);
      output.set(samples, block * 256);
    }
    assert.equal(core.wasm.memory.buffer, memory, "render/automation grew memory");
    assert.ok(output.every(Number.isFinite));
    assert.ok(output.some(v => Math.abs(v) > 0.0001), "silent AlphaVerb");
    const bytes = Buffer.from(output.buffer), file = `${folder}/${rate}-${count}.f32`;
    if (mode === "capture") await writeFile(file, bytes);
    else assert.equal(bytes.compare(await readFile(file)), 0, `PCM changed: ${file}`);
    times.sort((a, b) => a - b);
    const result = {
      rate, count, prepareMs,
      meanMs: times.reduce((a, b) => a + b, 0) / blocks,
      p95Ms: times[Math.floor(blocks * 0.95)],
      deadlineMs: 128000 / rate,
    };
    timings.push(result);
    console.log(JSON.stringify(result));
  }
}
await writeFile(`${folder}/${mode}-timings.json`, JSON.stringify(timings, null, 2));
