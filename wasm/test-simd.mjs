// The SIMD engine must render exactly the samples of the baseline engine: it
// only changes how the compiler schedules the same IEEE float arithmetic.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { Core } from "./web/core.mjs";
import { supportsSimd } from "./web/compile.mjs";

assert.equal(typeof supportsSimd(), "boolean");
const simdBytes = await readFile("dist/web/sad_pet.simd.wasm").catch(() => null);
if (!simdBytes) {
  console.log("No SIMD engine in dist/web (build with wasm/build.py --simd); nothing to compare");
  process.exit(0);
}
const [base, simd] = await Promise.all(
  [await readFile("dist/web/sad_pet.wasm"), simdBytes].map((bytes) => WebAssembly.compile(bytes)),
);

function render(module) {
  const utility = new Core(module);
  utility.call("sad_prepare", utility.call("sad_factory", undefined, 0));
  const prepared = utility.call("sad_pack");
  const preset = utility.request("/api/presets")[2];
  Object.assign(preset.params, {
    grains: 0.8,
    grain_size: 0.7,
    filter_routing: "serial",
    filter_model: "ms20_lp12",
    filter: 0.55,
    filter2: 0.7,
    filter_resonance: 0.6,
  });
  const slot = structuredClone(preset.color_chain[0] ?? utility.request("/api/color-defaults").slot);
  preset.color_chain = ["reverb", "lcr_delay", "chorus", "compressor", "distortion"].map(
    (kind, i) => ({ ...structuredClone(slot), id: 20 + i, kind, enabled: true, mix: 0.4 }),
  );
  preset.resynth.enabled = true;
  preset.resynth.count = 2;
  const output = [];
  for (const rate of [44100, 48000]) {
    const core = new Core(module, rate);
    core.call("sad_load", prepared);
    core.request("/api/preset", preset);
    core.request("/api/transport", { playing: true, midi_gate: true });
    for (const note of [48, 55, 60, 67])
      core.request("/api/note", { kind: "note_on", channel: 0, note, velocity: 100 });
    const blocks = Math.floor(rate / 128);
    const samples = new Float32Array(blocks * 256);
    const input = [new Float32Array(128).fill(0.1)];
    for (let block = 0; block < blocks; block++) {
      if (block === 120) core.request("/api/note", { kind: "note_off", channel: 0, note: 55, velocity: 0 });
      if (block % 97 === 0) core.request("/api/parameters", { pitch: 0.4 + (block % 3) * 0.1 });
      samples.set(core.process(128, input), block * 256);
    }
    assert.ok(samples.some((v) => Math.abs(v) > 1e-4), `${rate}: silent render`);
    output.push(Buffer.from(samples.buffer));
  }
  return Buffer.concat(output);
}

assert.equal(Buffer.compare(render(base), render(simd)), 0, "SIMD engine renders different samples");
console.log("SIMD engine matches the baseline engine bit for bit");
