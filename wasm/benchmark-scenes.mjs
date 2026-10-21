// Differential PCM for stateful effects, Resynth and drum scenes. No audio device.
// node wasm/benchmark-scenes.mjs capture target/perf/scenes [baseline.wasm]
// node wasm/benchmark-scenes.mjs compare target/perf/scenes [candidate.wasm]
import assert from "node:assert/strict";
import { readFile, writeFile, mkdir } from "node:fs/promises";
import { Core } from "./web/core.mjs";

const [mode, folder, binary = "dist/web/sad_pet.wasm"] = process.argv.slice(2);
assert.ok(["capture", "compare"].includes(mode) && folder);
await mkdir(folder, { recursive: true });
const module = await WebAssembly.compile(await readFile(binary));
const utility = new Core(module);
utility.call("sad_prepare", utility.call("sad_factory", undefined, 0));
const source = utility.call("sad_pack");
const initial = utility.request("/api/presets")[0];

for (const rate of [44100, 48000, 96000]) {
  for (const scene of ["effects", "routing", "resynth", "drums"]) {
    const core = new Core(module, rate);
    core.call("sad_load", source);
    let preset = structuredClone(initial);
    if (scene === "effects" || scene === "routing") {
      preset.color_chain = ["reverb", "lcr_delay", "mod_delay", "chorus", "flanger"]
        .map((kind, i) => ({ ...structuredClone(initial.color_chain[0]), id: i + 10, kind, mix: 0.3 }));
      preset.pre_color_chain = scene === "routing" ? [] : preset.color_chain.map(s => ({ ...structuredClone(s), id: s.id + 100 }));
    } else if (scene === "resynth") {
      preset.resynth.enabled = true;
      preset.resynth.count = 4;
      preset.resynth.oscillators.forEach((o, i) => Object.assign(o, {
        waveform: ["saw", "sine", "pulse", "pcm"][i],
        mode: i === 0 ? "unison" : "wave", transpose: i * 7 - 12,
        interaction: i ? "ring_sync" : "off", interaction_source: i ? i + 1 : 0,
      }));
    }
    core.request("/api/preset", preset);
    if (scene === "drums") {
      core.request("/api/timbres/add", { kind: "drums" });
      core.request("/api/timbres/update", { id: 0, muted: true });
      core.request("/api/drums/edit", [0, 4, 8, 12].map(step => ({ track: 0, step, field: 0, value: 0.9 })));
      preset = core.request("/api/preset");
    }
    core.request("/api/transport", { playing: true, midi_gate: false });
    const blocks = Math.floor(rate * 2 / 128), output = new Float32Array(blocks * 256);
    for (let block = 0; block < blocks; block++) {
      if (block === 100) {
        // Same identities and algorithms must retain their internal histories.
        preset.color_chain.reverse();
        preset.pre_color_chain.reverse();
        core.request("/api/preset", preset);
      }
      if (scene === "routing" && [150, 200, 250, 270, 310, 320].includes(block)) {
        if (block === 150) preset.pre_color_chain.push(preset.color_chain.pop());
        if (block === 200) preset.pre_color_chain[0].kind = "lcr_delay";
        if (block === 250) preset.pre_color_chain[0].kind = "chorus";
        if (block === 270) preset.pre_color_chain[0].kind = "mod_delay";
        if (block === 310) {
          preset.pre_color_chain[0].settings.mod_delay_smooth = false;
          preset.pre_color_chain[0].settings.mod_delay_ratio = 0.3;
        }
        if (block === 320) preset.color_chain.push(preset.pre_color_chain.pop());
        core.request("/api/preset", preset);
      }
      if (block === 180 && scene === "drums")
        core.request("/api/parameters", { drum_granular: 1 });
      if (block === 240 && scene === "effects") {
        preset.color_chain[0].enabled = false;
        core.request("/api/preset", preset);
      }
      if (block === 280 && scene === "effects") {
        preset.color_chain[0].enabled = true;
        core.request("/api/preset", preset);
      }
      if (block === 350) core.request("/api/transport", { playing: false });
      output.set(core.process(128), block * 256);
    }
    assert.ok(output.every(Number.isFinite));
    assert.ok(output.some(v => Math.abs(v) > 0.0001), `${scene} produced silence`);
    const bytes = Buffer.from(output.buffer), file = `${folder}/${rate}-${scene}.f32`;
    if (mode === "capture") await writeFile(file, bytes);
    else assert.equal(bytes.compare(await readFile(file)), 0, `PCM changed: ${file}`);
    console.log(`${rate} ${scene}: ${output.length} samples ${mode}`);
  }
}
