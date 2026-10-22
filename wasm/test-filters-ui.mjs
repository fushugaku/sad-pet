// Execute the shipping filter editor, including code between its functions.
// This catches initialization/scope errors that DSP-only WASM tests cannot.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import vm from "node:vm";
const source = await readFile(
  process.argv[2] ?? new URL("../rust/web/app.js", import.meta.url),
  "utf8",
);
const elements = new Map(),
  dials = new Map(),
  bindings = new Map(),
  schemas = [],
  changes = [];
let presetWrites = 0;
function element() {
  const attrs = new Map(),
    classes = new Set(),
    parts = new Map(),
    label = { title: "" };
  const box = {
    classList: {
      toggle: (name, on) => (on ? classes.add(name) : classes.delete(name)),
    },
    querySelector: () => label,
  };
  const node = {
    classList: box.classList,
    children: [],
    dataset: {},
    append(...nodes) {
      node.children.push(...nodes);
    },
    replaceChildren(...nodes) {
      node.children = nodes;
    },
    setAttribute: (k, v) => attrs.set(k, v),
    getAttribute: (k) => attrs.get(k),
    closest: () => box,
    querySelector(selector) {
      const key = selector.match(/data-control-key="([^"]+)"/)?.[1];
      if (key) return dials.get(key) ?? null;
      const id = selector.match(/data-id="([^"]+)"/)?.[1];
      if (id) return node.children.find((c) => String(c.dataset.id) === id);
      if (!parts.has(selector)) parts.set(selector, element());
      return parts.get(selector);
    },
  };
  return node;
}
const get = (id) => {
  if (!elements.has(id)) elements.set(id, element());
  return elements.get(id);
};
const preset = {
  params: {
    filter_routing: "single",
    filter_model: "legacy",
    filter2_model: "legacy",
    vcf_type: "low_pass",
    vcf2_type: "low_pass",
    filter_env: "env2",
    filter2_env: "env2",
  },
  lfos: [{ destination: "none" }],
  envs: [],
  matrix: [],
};
const context = {
  preset,
  $: get,
  window: {},
  requestAnimationFrame: () => 1,
  percent: (v) => `${v * 100}%`,
  signedPercent: (v) => `${v * 100}%`,
  controls: new Map(),
  modulationViews: new Map(),
  option: (value, label) => ({ value, label }),
  document: {
    createElement: element,
    querySelector: (selector) =>
      dials.get(selector.match(/data-control-key="([^"]+)"/)?.[1]) ?? null,
  },
  createKnob(schema, bank, binding) {
    schemas.push(schema);
    dials.set(schema[0], element());
    if (binding) bindings.set(schema[0], binding);
  },
  syncControl() {
    context.syncFilters();
  },
  scheduleParameter: (id, value) => changes.push([id, value]),
  schedulePreset() {
    presetWrites++;
  },
  refreshColorDestinations() {},
  syncLfos() {},
  syncEnvs() {},
  syncMatrix() {},
};
vm.createContext(context);
const constants = source.slice(
  source.indexOf("const filterModels ="),
  source.indexOf("const grainSchema ="),
);
const editor = source.slice(
  source.indexOf("function buildFilters()"),
  source.indexOf("function syncAll()"),
);
assert.ok(constants.length > 0 && editor.length > 0);
vm.runInContext(`${constants}\n${editor}`, context);
context.buildFilters();
assert.equal(schemas.length, 12, "Filter controls were not registered");
assert.deepEqual(
  schemas.map((s) => s[0]).sort(),
  [
    "filter",
    "filter2",
    ...["filter", "filter2"].flatMap((p) =>
      ["resonance", "env_depth", "keytrack", "drive", "morph"].map(
        (k) => `${p}_${k}`,
      ),
    ),
  ].sort(),
);
context.syncFilters();
assert.equal(dials.get("filter_drive").getAttribute("aria-disabled"), "true");
get("filter-type").onchange({ target: { value: "m_poly_lp24" } });
assert.equal(preset.params.filter_model, "m_poly_lp24");
assert.equal(dials.get("filter_drive").getAttribute("aria-disabled"), "false");
assert.equal(dials.get("filter_morph").getAttribute("aria-disabled"), "true");
assert.deepEqual(changes.splice(0), [["filter_model", 10]]);
get("filter2-type").onchange({ target: { value: "radias2:comb" } });
assert.equal(preset.params.filter2_model, "radias2");
assert.equal(preset.params.vcf2_type, "comb");
assert.equal(get("filter2-type").value, "radias2:comb");
assert.deepEqual(changes.splice(0), [
  ["filter2_model", 19],
  ["vcf2_type", 3],
]);
for (const model of ["radias", "sem", "multi_filter"]) {
  get("filter-type").onchange({ target: { value: model } });
  assert.equal(
    dials.get("filter_morph").getAttribute("aria-disabled"),
    "false",
  );
  assert.equal(get("filter-type").value, model);
}
context.setParam("filter_morph", 0.4);
assert.equal(preset.params.filter_morph, 0.4);
assert.deepEqual(changes.at(-1), ["filter_morph", 0.4]);
assert.ok(elements.get("filters"));
assert.equal(presetWrites, 0, "A Filter 1/2 selection sent the full preset");

// Added filters: structural edits send the preset, knobs send host IDs by
// list position, and removal clears modulation routes to the slot.
const list = get("added-filters");
// Arrays made inside the VM have its own prototype: compare host copies.
const ids = () => [...preset.filter_slots.map((s) => s.id)];
assert.equal(preset.filter_slots.length, 0);
assert.equal(get("add-filter").disabled, false);
get("add-filter").onclick();
assert.equal(presetWrites, 1);
assert.equal(preset.filter_slots.length, 1);
assert.deepEqual(
  { ...preset.filter_slots[0] },
  {
    id: 1,
    enabled: true,
    position: "after_filters",
    model: "legacy",
    kind: "low_pass",
    cutoff: 0.7,
    resonance: 0.12,
    drive: 0,
    morph: 0,
    env: "env2",
    env_depth: 0,
    keytrack: 0,
  },
);
assert.equal(list.children.length, 1);
const dial = (field, id = 1) => dials.get(`filter_slot:${id}:${field}`);
assert.equal(dial("env_depth").getAttribute("aria-disabled"), "false");
assert.equal(dial("drive").getAttribute("aria-disabled"), "true");
bindings.get("filter_slot:1:cutoff").set(0.3);
assert.equal(preset.filter_slots[0].cutoff, 0.3);
assert.deepEqual(changes.at(-1), ["filter3_cutoff", 0.3]);

let card = list.querySelector('[data-id="1"]');
const position = card.querySelector(".added-filter-position");
position.value = "after_osc2";
position.onchange();
assert.equal(preset.filter_slots[0].position, "after_osc2");
assert.equal(presetWrites, 2);
// A shared position keeps its envelope and key tracking.
assert.equal(dial("env_depth").getAttribute("aria-disabled"), "false");
assert.equal(dial("keytrack").getAttribute("aria-disabled"), "false");
card = list.querySelector('[data-id="1"]');
assert.notEqual(card.querySelector(".added-filter-env").hidden, true);
const type = card.querySelector(".added-filter-type");
type.value = "ladder_lp24";
type.onchange();
assert.equal(preset.filter_slots[0].model, "ladder_lp24");
assert.equal(dial("drive").getAttribute("aria-disabled"), "false");

get("add-filter").onclick();
assert.equal(preset.filter_slots[1].id, 2);
list.querySelector('[data-id="2"]').querySelector(".color-up").onclick();
assert.deepEqual(ids(), [2, 1]);
bindings.get("filter_slot:1:resonance").set(0.5);
assert.deepEqual(changes.at(-1), ["filter4_resonance", 0.5]);

preset.lfos[0].destination = { filter_slot: { slot: 1, parameter: "cutoff" } };
list.querySelector('[data-id="1"]').querySelector(".color-remove").onclick();
assert.deepEqual(ids(), [2]);
assert.equal(preset.lfos[0].destination, "none");
while (preset.filter_slots.length < 16) get("add-filter").onclick();
assert.equal(get("add-filter").disabled, true);
get("add-filter").onclick();
assert.equal(preset.filter_slots.length, 16);
assert.equal(list.children.length, 16);
console.log(
  "Filter editor: initialization, 12 controls, model/Comb selection, conditional Morph, scalar transport and added filters (position, model, host IDs, order, removal, limit) passed.",
);
