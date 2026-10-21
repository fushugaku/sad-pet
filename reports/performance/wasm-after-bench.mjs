// Same-platform differential test of the actual compiled WASM. No audio device.
// node wasm/benchmark.mjs capture target/perf/wasm-before
// node wasm/benchmark.mjs compare target/perf/wasm-before
import assert from "node:assert/strict";
import { readFile, writeFile, mkdir } from "node:fs/promises";
import { Core } from "file:///Users/georgy/Projects/Evil_update/wasm/web/core.mjs";
const [mode, folder] = process.argv.slice(2);
assert.ok(["capture", "compare"].includes(mode) && folder);
await mkdir(folder, { recursive: true });
const module = await WebAssembly.compile(
  await readFile("dist/web/sad_pet.wasm"),
);
const utility = new Core(module);
utility.call("sad_prepare", utility.call("sad_factory", undefined, 0));
const prepared = utility.call("sad_pack");
const factory = utility.request("/api/presets");
for (const rate of [44100, 48000, 96000])
  for (let variant = 0; variant < 8; variant++) {
    const core = new Core(module, rate),
      p = structuredClone(factory[variant % 5]);
    p.params.loop_one_bar = variant === 5;
    if (variant >= 3)
      Object.assign(p.params, {
        grains: 1,
        grain_size: 0.9,
        filter_routing: "serial",
        vcf_type: "band_pass",
        filter: 0.65,
        filter2: 0.8,
      });
    core.call("sad_load", prepared);
    core.request("/api/preset", p);
    if (variant === 2 || variant === 5)
      core.request("/api/region", { start: 0.251, end: 0.613 });
    if (variant === 6) core.request("/api/region", { start: 0.5, end: 0.5001 });
    if (variant === 7)
      core.request("/api/input", { active: true, mode: "grains" });
    core.request("/api/transport", { playing: true, midi_gate: variant >= 3 });
    if (variant >= 3)
      for (const note of [36, 48, 55, 60, 67, 72, 79, 91])
        core.request("/api/note", {
          kind: "note_on",
          channel: 0,
          note,
          velocity: 100,
        });
    const blocks = Math.floor((rate * 2) / 128),
      output = new Float32Array(blocks * 256),
      times = [];
    const input = [new Float32Array(128).fill(0.2)];
    for (let block = 0; block < blocks; block++) {
      if (block % 137 === 0)
        core.request("/api/parameters", {
          in_gain: (block % 9) - 4,
          out_gain: (block % 7) - 3,
          pitch: 0.3 + (block % 5) * 0.15,
        });
      const start = performance.now();
      output.set(core.process(128, input), block * 256);
      if (block > 100) times.push((performance.now() - start) * 1000);
    }
    const file = `${folder}/${rate}-${variant}.f32`,
      bytes = Buffer.from(output.buffer);
    if (mode === "capture") await writeFile(file, bytes);
    else
      assert.equal(
        bytes.compare(await readFile(file)),
        0,
        `WASM samples changed: ${file}`,
      );
    times.sort((a, b) => a - b);
    console.log(
      `${rate}-${variant}: samples=${output.length} median_us=${times[Math.floor(times.length / 2)].toFixed(2)} p99_us=${times[Math.floor(times.length * 0.99)].toFixed(2)} ${mode}`,
    );
  }
