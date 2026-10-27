// Grain read quality: the engine's /api/interpolation choice, kept to one
// WebAssembly instance, and the editor's Reads select that restores this
// device's choice from localStorage.
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFile } from "node:fs/promises";
import vm from "node:vm";
import { Core } from "./web/core.mjs";

const engines = ["dist/web/sad_pet.wasm", "dist/web/sad_pet.simd.wasm"];
for (const path of engines) {
  const bytes = await readFile(path).catch(() => null);
  if (!bytes) continue;
  const module = await WebAssembly.compile(bytes);
  const utility = new Core(module);
  utility.call("sad_prepare", utility.call("sad_factory", undefined, 0));
  const prepared = utility.call("sad_pack");
  const preset = utility.request("/api/presets")[0];
  // Dense grains a fifth up: the sinc kernels and the cubic read them apart.
  Object.assign(preset.params, { grains: 1, grain_size: 0.9, pitch: 0.5 + 7 / 72 });
  const render = (quality) => {
    const core = new Core(module, 48000);
    core.call("sad_load", prepared);
    core.request("/api/preset", preset);
    core.request("/api/transport", { playing: true, midi_gate: true });
    for (const note of [48, 60, 67])
      core.request("/api/note", { kind: "note_on", channel: 0, note, velocity: 100 });
    assert.deepEqual(core.request("/api/interpolation"), { quality: "high" });
    if (quality)
      assert.deepEqual(core.request("/api/interpolation", { quality }), { quality });
    const hash = createHash("sha256");
    for (let block = 0; block < 200; block++) hash.update(core.process(128));
    return hash.digest("hex");
  };
  const high = render();
  assert.equal(render("high"), high, `${path}: High is the default`);
  const eco = render("eco");
  assert.notEqual(eco, high, `${path}: Eco reads the cubic`);
  assert.equal(render("eco"), eco, `${path}: Eco renders reproducibly`);
  // Each instance keeps its own choice: offline renders in another keep High.
  assert.equal(render(), high, `${path}: another instance stays High`);
  assert.throws(
    () => new Core(module, 48000).request("/api/interpolation", { quality: "low" }),
    /High or Eco/,
  );
}

// The editor: shown in the browser only, restored from and saved to
// localStorage, and sent to the engine.
const app = await readFile(new URL("../rust/web/app.js", import.meta.url), "utf8");
const start = app.indexOf("function grainQuality() {");
const source = app.slice(start, app.indexOf("\n}\n", start) + 2);
function editor(saved, browser = true) {
  const elements = {
    "grain-quality": { value: "", onchange: null },
    "grain-quality-field": { hidden: true },
  };
  const stored = new Map(saved ? [["sad-pet-grain-quality", saved]] : []);
  const sent = [];
  const context = {
    window: browser ? { sadPetAudio: {} } : {},
    $: (id) => elements[id],
    localStorage: {
      getItem: (key) => stored.get(key) ?? null,
      setItem: (key, value) => stored.set(key, String(value)),
    },
    // Objects made inside the context compare by their JSON.
    api: async (path, data) => sent.push(JSON.stringify([path, data])),
    notify: () => {},
  };
  vm.runInNewContext(`${source}\ngrainQuality();`, context);
  return { elements, stored, sent };
}
{
  const { elements, stored, sent } = editor("eco");
  assert.equal(elements["grain-quality"].value, "eco");
  assert.equal(elements["grain-quality-field"].hidden, false);
  assert.deepEqual(sent, ['["/api/interpolation",{"quality":"eco"}]']);
  elements["grain-quality"].value = "high";
  elements["grain-quality"].onchange();
  assert.equal(stored.get("sad-pet-grain-quality"), "high");
  assert.equal(sent[1], '["/api/interpolation",{"quality":"high"}]');
}
assert.equal(editor(null).elements["grain-quality"].value, "high");
assert.equal(editor("nonsense").elements["grain-quality"].value, "high");
{
  const { elements, sent } = editor("eco", false);
  assert.equal(elements["grain-quality-field"].hidden, true);
  assert.deepEqual(sent, []);
}
console.log("Grain read quality: High by default, Eco per instance, restored by the editor");
