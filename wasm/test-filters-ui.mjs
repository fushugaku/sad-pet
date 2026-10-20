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
  schemas = [],
  changes = [];
function element() {
  const attrs = new Map(),
    classes = new Set(),
    label = { title: "" };
  const box = {
    classList: {
      toggle: (name, on) => (on ? classes.add(name) : classes.delete(name)),
    },
    querySelector: () => label,
  };
  return {
    classList: box.classList,
    append() {},
    setAttribute: (k, v) => attrs.set(k, v),
    getAttribute: (k) => attrs.get(k),
    closest: () => box,
  };
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
};
const context = {
  preset,
  $: get,
  percent: (v) => `${v * 100}%`,
  signedPercent: (v) => `${v * 100}%`,
  controls: new Map(),
  document: {
    createElement: element,
    querySelector: (selector) =>
      dials.get(selector.match(/data-control-key="([^"]+)"/)?.[1]) ?? null,
  },
  createKnob(schema) {
    schemas.push(schema);
    dials.set(schema[0], element());
  },
  syncControl() {
    context.syncFilters();
  },
  scheduleParameter: (id, value) => changes.push([id, value]),
  schedulePreset() {
    throw Error("A filter selection sent the full preset");
  },
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
console.log(
  "Filter editor: initialization, 12 controls, model/Comb selection, conditional Morph and scalar transport passed.",
);
