// Exact PCM and render cost of Tape/BBD, two instances and AlphaVerb together.
// node wasm/benchmark-superecho.mjs capture target/perf/superecho [engine.wasm]
// node wasm/benchmark-superecho.mjs compare target/perf/superecho [engine.wasm]
import assert from "node:assert/strict";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { Core } from "./web/core.mjs";

const [mode, folder, binary = "dist/web/sad_pet.wasm"] = process.argv.slice(2);
assert.ok(["capture", "compare"].includes(mode) && folder);
await mkdir(folder, { recursive: true });
const module = await WebAssembly.compile(await readFile(binary));
const utility = new Core(module);
utility.call("sad_prepare", utility.call("sad_factory", undefined, 0));
const source = utility.call("sad_pack"), timings = [];
for (const rate of [44100, 48000, 96000]) {
  for (const scene of ["tape", "bbd", "two", "alphaverb"]) {
    const core = new Core(module, rate);
    core.call("sad_load", source);
    const preset = core.request("/api/preset");
    const defaults = core.request("/api/color-defaults").slot;
    preset.source_color_chain = [];
    preset.pre_color_chain = [];
    const kinds = scene === "two" ? ["super_echo", "super_echo"]
      : scene === "alphaverb" ? ["super_echo", "alpha_verb"] : ["super_echo"];
    preset.color_chain = kinds.map((kind, i) => ({
      ...structuredClone(defaults), id: i + 1, kind, enabled: true, mix: 0.7,
      settings: { ...defaults.settings, super_echo_dirty: +(scene === "bbd" || i === 1),
        super_echo_time: 0.3, super_echo_feedback: 0.55 },
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
        fx1_super_echo_shimmer: 0.6, fx1_super_echo_compression: 0.5,
        fx1_super_echo_reverb: 0.4, fx1_super_echo_mod_depth: 0.6,
      });
      if (block === 200) core.request("/api/parameters", {
        fx1_super_echo_shimmer_mode: 1, fx1_super_echo_shimmer_style: 1,
        fx1_super_echo_filter_mode: 1, fx1_super_echo_dub: 1,
      });
      if (block === 300) core.request("/api/parameters", {
        fx1_super_echo_sync: 1, fx1_super_echo_note: 0.375,
        fx1_super_echo_freeze: 1,
      });
      if (block === 400) core.request("/api/parameters", {
        fx1_super_echo_freeze: 0, fx1_super_echo_sync: 0,
        fx1_super_echo_time: 0.63,
      });
      if (block === 500) core.request("/api/transport", { playing: false });
      const start = performance.now();
      const samples = core.process(128);
      times.push(performance.now() - start);
      output.set(samples, block * 256);
    }
    assert.equal(core.wasm.memory.buffer, memory, "render/automation grew memory");
    assert.ok(output.every(Number.isFinite));
    assert.ok(output.some(v => Math.abs(v) > 0.0001), `silent ${scene}`);
    const bytes = Buffer.from(output.buffer), file = `${folder}/${rate}-${scene}.f32`;
    if (mode === "capture") await writeFile(file, bytes);
    else assert.equal(bytes.compare(await readFile(file)), 0, `PCM changed: ${file}`);
    times.sort((a, b) => a - b);
    const result = { rate, scene, prepareMs,
      meanMs: times.reduce((a, b) => a + b, 0) / blocks,
      p95Ms: times[Math.floor(blocks * 0.95)], deadlineMs: 128000 / rate };
    timings.push(result);
    console.log(JSON.stringify(result));
  }
}
await writeFile(`${folder}/${mode}-timings.json`, JSON.stringify(timings, null, 2));
