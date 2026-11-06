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
const windowListeners = new Map(), documentListeners = new Map();
let pointerTarget = null;
let presetWrites = 0;
function element() {
  const attrs = new Map(),
    classes = new Set(),
    parts = new Map(),
    label = { title: "" };
  const box = {
    classList: {
      toggle: (name, on) => (on ? classes.add(name) : classes.delete(name)),
      add: (...names) => names.forEach(name => classes.add(name)),
      remove: (...names) => names.forEach(name => classes.delete(name)),
      contains: name => classes.has(name),
    },
    querySelector: () => label,
  };
  const node = {
    before(other) { elements.set(other.id, other); },
    classList: box.classList,
    children: [],
    dataset: {},
    style: { setProperty() {} },
    append(...nodes) {
      for (const child of nodes) {
        if (child?.parentElement) child.parentElement.children = child.parentElement.children.filter(n => n !== child);
        if (child && typeof child === "object") child.parentElement = node;
        node.children.push(child);
      }
    },
    prepend(...nodes) {
      for (const child of nodes.reverse()) {
        if (child?.parentElement) child.parentElement.children = child.parentElement.children.filter(n => n !== child);
        if (child && typeof child === "object") child.parentElement = node;
        node.children.unshift(child);
      }
    },
    insertBefore(child, target) {
      if (child.parentElement) child.parentElement.children = child.parentElement.children.filter(n => n !== child);
      child.parentElement = node;
      node.children.splice(node.children.indexOf(target), 0, child);
    },
    getBoundingClientRect: () => node.rect ?? { left: 0, top: 0, bottom: 100, width: 100 },
    replaceChildren(...nodes) {
      node.children = [];
      node.append(...nodes);
    },
    focus() {},
    setAttribute: (k, v) => attrs.set(k, v),
    removeAttribute: k => attrs.delete(k),
    getAttribute: (k) => attrs.get(k),
    closest: selector => selector === ".filter-shaper" && node.className === "filter-shaper" ? node : box,
    querySelector(selector) {
      const key = selector.match(/data-control-key="([^"]+)"/)?.[1];
      if (key) return dials.get(key) ?? null;
      const id = selector.match(/data-id="([^"]+)"/)?.[1];
      if (id) return node.children.find((c) => String(c.dataset.id) === id);
      if (!parts.has(selector)) parts.set(selector, element());
      const field = selector.match(/data-field="([^"]+)"/)?.[1];
      if (field) parts.get(selector).dataset.field = field;
      return parts.get(selector);
    },
    querySelectorAll: selector => selector === ".dial"
      ? node.children
      : [],
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
  structuredClone,
  WAVESHAPER_TYPES: [["drive", "Drive"], ["hard_clip", "Hard Clip"]],
  state: null,
  timbreEditor: null,
  tapSettings: stage => ({ chain: 1, send: 0, bus: 0, ...preset.taps?.[stage] }),
  editTap: (stage, field, value) => {
    preset.taps ??= {};
    preset.taps[stage] = { ...context.tapSettings(stage), [field]: value };
    changes.push([`tap_${stage}_${field}`, value]);
    context.syncFilterOutputs();
  },
  sendTargets: () => ({ sends: true, buses: [{ bus: 1, name: "Bus 1" }] }),
  fillBusMenu: (select, value) => { select.value = String(value); },
  $: get,
  sequenceCatalog: { filter_presets: [{id:"silk",name:"Silk",params:{filter_routing:"serial",filter2_model:"ladder_lp24"}}] },
  window: { addEventListener: (type, fn) => windowListeners.set(type, fn),
    removeEventListener: type => windowListeners.delete(type), SadLazySelect: { configure(node, value, groups) {
    assert.ok(groups.every(group => Array.isArray(group.choices)), "LazySelect expects choice groups");
    for (const group of groups) for (const choice of group.choices) assert.equal(choice.length, 2);
    node.groups=groups;
  } } },
  requestAnimationFrame: () => 1,
  percent: (v) => `${v * 100}%`,
  signedPercent: (v) => `${v * 100}%`,
  controls: new Map(),
  modulationViews: new Map(),
  option: (value, label) => ({ value, label }),
  document: {
    createElement: element,
    addEventListener: (type, fn) => documentListeners.set(type, fn),
    removeEventListener: type => documentListeners.delete(type),
    elementFromPoint: () => pointerTarget,
    querySelector: (selector) =>
      dials.get(selector.match(/data-control-key="([^"]+)"/)?.[1]) ?? null,
    querySelectorAll: selector => {
      const walk = n => [n, ...n.children.flatMap(walk)];
      return [get("filters"), get("added-filters")].flatMap(walk).filter(n => n.className === selector.slice(1));
    },
  },
  createKnob(schema, bank, binding) {
    schemas.push(schema);
    const dial = element();
    dials.set(schema[0], dial);
    if (typeof bank !== "string") bank.append(dial);
    if (binding) bindings.set(schema[0], binding);
    return () => {};
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
const colorTypes = source.slice(source.indexOf("const COLOR_TYPES ="), source.indexOf("// Sections of the effect menus"));
vm.runInContext(`${colorTypes}\n${constants}\n${editor}`, context);
context.buildFilters();
const filterSchemas = schemas.filter((s) => !/^filter\d+_ws_/.test(s[0]));
assert.equal(filterSchemas.length, 12, "Filter controls were not registered");
assert.deepEqual(
  filterSchemas.map((s) => s[0]).sort(),
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
  JSON.parse(JSON.stringify(preset.filter_slots[0])),
  {
    id: 1,
    enabled: true,
    position: "after_filters",
    fx_chain: "b",
    fx_slot: null,
    model: "legacy",
    kind: "low_pass",
    cutoff: 0.7,
    resonance: 0.12,
    drive: 0,
    morph: 0,
    env: "env2",
    env_depth: 0,
    keytrack: 0,
    output: { chain: 1, send: 0, bus: 0 },
  },
);
assert.equal(list.children.length, 1);
const outputRow = number => get("filter-outputs").children[number - 1];
let output = outputRow(3);
assert.equal(output.className, "tap filter-output");
assert.ok(!list.children[0].children.includes(output), "Output belongs below the filter cards");
output.querySelector("select").value = "1";
output.querySelector("select").onchange();
assert.equal(dials.get("filter3_send").getAttribute("aria-disabled"), "false");
bindings.get("filter3_send").set(.6);
assert.deepEqual(changes.at(-1), ["filter3_send", .6]);
assert.equal(preset.filter_slots[0].output.send, .6);
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
bindings.get("filter4_send").set(.3);
assert.deepEqual(changes.at(-1), ["filter4_send", .3], "Reordered send edited the wrong filter");
assert.equal(preset.filter_slots[1].output.bus, 1);
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
// New envelopes become selectable immediately in every filter, including
// when syncEnvs is called by the add/copy/modulation editor paths.
context.envCards = [];
vm.runInContext(source.slice(source.indexOf("function syncEnvs()"), source.indexOf("function drawEnvs()")), context);
for (let count = 4; count <= 8; count++) {
  preset.env_count = count;
  context.syncEnvs();
  const sources = Array.from({ length: count }, (_, i) => `env${i + 1}`);
  for (const prefix of ["filter", "filter2"]) {
    const select = get(`${prefix}-env`);
    assert.deepEqual(select.children.map(o => o.value), sources);
    select.onchange({ target: { value: `env${count}` } });
    assert.equal(preset.params[`${prefix}_env`], `env${count}`);
    assert.equal(select.value, `env${count}`);
  }
  for (const card of list.children)
    assert.deepEqual(card.querySelector(".added-filter-env").children.map(o => o.value), sources);
  const select = list.children[0].querySelector(".added-filter-env");
  select.value = `env${count}`;
  select.onchange();
  assert.equal(preset.filter_slots[0].env, `env${count}`);
}
// A removed source stays visible without silently changing the saved route.
preset.env_count = 3;
context.syncEnvs();
for (const select of [get("filter-env"), get("filter2-env"), list.children[0].querySelector(".added-filter-env")]) {
  assert.equal(select.value, "env8");
  const missing = select.children.find(o => o.value === "env8");
  assert.equal(missing.disabled, true);
  assert.equal(missing.label, "ENV 8 · unavailable");
}
preset.env_count = 8;
context.syncEnvs();
for (const select of [get("filter-env"), get("filter2-env"), list.children[0].querySelector(".added-filter-env")]) {
  assert.equal(select.value, "env8");
  assert.equal(select.children.find(o => o.value === "env8").disabled, false);
}
preset.env_count = 3;
preset.params.filter_env = preset.params.filter2_env = preset.filter_slots[0].env = "env2";
context.syncEnvs();
// The source envelope appears in every filter only with a 303 module.
assert.deepEqual(get("filter-env").children.map(o => o.value), ["env1", "env2", "env3"]);
preset.tb303 = {};
context.syncFilters();
for (const prefix of ["filter", "filter2"]) {
  assert.deepEqual(get(`${prefix}-env`).children.map(o => o.value), ["env1", "env2", "env3", "tb303"]);
  get(`${prefix}-env`).onchange({ target: { value: "tb303" } });
  assert.equal(preset.params[`${prefix}_env`], "tb303");
}
const addedEnv = list.children[0].querySelector(".added-filter-env");
assert.ok(addedEnv.children.some(o => o.value === "tb303" && !o.disabled));
addedEnv.value = "tb303"; addedEnv.onchange();
assert.equal(preset.filter_slots[0].env, "tb303");
delete preset.tb303;
preset.params.filter_env = "env2";
context.syncFilters();
assert.ok(!get("filter-env").children.some(o => o.value === "tb303"));
assert.ok(get("filter2-env").children.find(o => o.value === "tb303").disabled);
const pairMenu=get("filter-pair-preset");
assert.equal(pairMenu.groups[0].choices[1][0],"silk");
pairMenu.value="silk";pairMenu.onchange();
assert.equal(preset.params.filter_routing,"serial");
assert.equal(preset.params.filter2_model,"ladder_lp24");

// After FX chooses a rack, then an effect by stable ID or its moving end.
const filter = preset.filter_slots[0];
const fxCard = () => list.querySelector(`[data-id="${filter.id}"]`);
const fxSelect = name => fxCard().querySelector(`.added-filter-${name}`);
assert.equal(fxSelect("fx-chain").hidden, true);
preset.pre_color_chain = [{ id: 31, kind: "distortion" }, { id: 32, kind: "chorus" }];
preset.color_chain = [{ id: 41, kind: "reverb" }];
preset.source_color_chain = [];
fxSelect("position").value = "after_fx";
fxSelect("position").onchange();
assert.equal(fxSelect("fx-chain").hidden, false);
assert.equal(fxSelect("fx-chain").value, "b");
assert.deepEqual(fxSelect("fx-slot").children.map(o => [o.value, o.label]), [["41", "1 · Reverb"], ["last", "Last"]]);
assert.equal(fxSelect("fx-slot").value, "last");
fxSelect("fx-chain").value = "a";
fxSelect("fx-chain").onchange();
assert.equal(filter.fx_chain, "a");
assert.deepEqual(fxSelect("fx-slot").children.map(o => o.value), ["31", "32", "last"]);
fxSelect("fx-slot").value = "31";
fxSelect("fx-slot").onchange();
assert.equal(filter.fx_slot, 31);
preset.pre_color_chain.reverse();
context.syncAddedFilters();
assert.equal(fxSelect("fx-slot").value, "31");
assert.equal(fxSelect("fx-slot").children[1].label, "2 · Distortion");
preset.pre_color_chain[1].kind = "crusher";
context.syncAddedFilters();
assert.equal(fxSelect("fx-slot").children[1].label, "2 · Bit crusher");
fxSelect("fx-slot").value = "last";
fxSelect("fx-slot").onchange();
assert.equal(filter.fx_slot, null);
preset.pre_color_chain.push({ id: 33, kind: "flanger" });
context.syncAddedFilters();
assert.equal(fxSelect("fx-slot").value, "last");
fxSelect("fx-slot").value = "33";
fxSelect("fx-slot").onchange();
fxSelect("fx-chain").value = "b";
fxSelect("fx-chain").onchange();
assert.equal(filter.fx_slot, null, "Changing rack resets to its Last");
fxSelect("fx-slot").value = "41";
fxSelect("fx-slot").onchange();
const racks = source.slice(source.indexOf("const COLOR_RACKS ="), source.indexOf("function colorEntries()"));
const removeColor = source.slice(source.indexOf("function removeColor(id)"), source.indexOf("function moveColor(id, delta)"));
context.editColorChain = () => context.syncAddedFilters();
vm.runInContext(`${racks}\n${removeColor}`, context);
context.removeColor(41);
assert.equal(filter.fx_slot, null, "Removing the selected effect falls back to Last");
assert.deepEqual(fxSelect("fx-slot").children.map(o => o.value), ["last"]);
fxSelect("position").value = "after_grains";
fxSelect("position").onchange();
assert.equal(fxSelect("fx-chain").hidden, true);
assert.equal(fxSelect("fx-slot").hidden, true);

// A stage move swaps the complete payload, including unknown future fields.
const mainCard = index => get("filters").children.find(card => Number(card.dataset.mainFilter) === index);
const stage = (card, side) => card.children.find(node => node.dataset.shaper?.endsWith(`:${side}`));
const press = (node, key) => node.onkeydown({ key, preventDefault() {} });
const before = stage(mainCard(0), "before"), after = stage(mainCard(0), "after");
before.querySelector(".filter-shaper-power").onclick();
const beforeMode = before.querySelector(".filter-shaper-mode");
beforeMode.value = "hard_clip"; beforeMode.onchange();
bindings.get("filter1_ws_before_depth").set(.73);
assert.deepEqual(changes.at(-1), ["filter1_ws_before_depth", .73]);
const options = before.querySelector(".filter-shaper-body").children[0];
options.open = true; options.ontoggle();
bindings.get("filter1_ws_before_knee").set(.36);
assert.deepEqual(changes.at(-1), ["filter1_ws_before_knee", .36]);
preset.params.filter_shapers[0].before.future = { keep: true };
preset.params.filter_shapers[0].before.options.future_option = 17;
press(before.querySelector(".filter-shaper-grip"), "ArrowRight");
assert.equal(preset.params.filter_shapers[0].after.mode, "hard_clip");
assert.equal(preset.params.filter_shapers[0].after.depth, .73);
assert.equal(preset.params.filter_shapers[0].after.future.keep, true);
assert.equal(preset.params.filter_shapers[0].after.options.future_option, 17);
assert.equal(preset.params.filter_shapers[0].before.enabled, false);
assert.ok(!changes.some(([id]) => id.startsWith("filter_shaper")), "Per-filter shapers use actual host IDs");

// Pointer drop crosses into an added filter; Escape cancels without a write.
const addedBefore = stage(list.children[0], "before");
const grip = after.querySelector(".filter-shaper-grip");
const down = () => grip.onpointerdown({ button: 0, currentTarget: grip, preventDefault() {} });
down(); pointerTarget = addedBefore;
windowListeners.get("pointermove")({ clientX: 10, clientY: 10 });
windowListeners.get("pointerup")({ clientX: 10, clientY: 10 });
assert.equal(preset.filter_slots[0].shapers.before.mode, "hard_clip");
assert.equal(preset.filter_slots[0].shapers.before.future.keep, true);
bindings.get("filter3_ws_before_mix").set(.61);
assert.deepEqual(changes.at(-1), ["filter3_ws_before_mix", .61]);
assert.equal(preset.params.filter_shapers[0].after.enabled, false);
const addedGrip = addedBefore.querySelector(".filter-shaper-grip");
addedGrip.onpointerdown({ button: 0, currentTarget: addedGrip, preventDefault() {} });
pointerTarget = before; windowListeners.get("pointermove")({ clientX: 1, clientY: 1 });
const writesBeforeCancel = presetWrites;
documentListeners.get("keydown")({ key: "Escape", preventDefault() {}, stopPropagation() {} });
assert.equal(presetWrites, writesBeforeCancel);
assert.equal(preset.filter_slots[0].shapers.before.mode, "hard_clip");

// Main filter reorder changes only order, keeping both filters and their WS identities.
press(mainCard(0).querySelector(".filter-title").children[0], "ArrowRight");
assert.deepEqual(Array.from(preset.params.filter_order), [1, 0]);
assert.deepEqual(get("filters").children.map(card => Number(card.dataset.mainFilter)), [1, 0]);
assert.equal(preset.filter_slots[0].shapers.before.mode, "hard_clip");
const firstMain = mainCard(1), secondMain = mainCard(0);
firstMain.rect = { left: 0, top: 0, bottom: 100, width: 100 };
secondMain.rect = { left: 120, top: 0, bottom: 100, width: 100 };
const mainGrip = secondMain.querySelector(".filter-title").children[0];
mainGrip.onpointerdown({ button: 0, currentTarget: mainGrip, preventDefault() {} });
windowListeners.get("pointermove")({ clientX: 5, clientY: 50 });
windowListeners.get("pointerup")({ clientX: 5, clientY: 50 });
assert.deepEqual(Array.from(preset.params.filter_order), [0, 1]);
mainGrip.onpointerdown({ button: 0, currentTarget: mainGrip, preventDefault() {} });
windowListeners.get("pointermove")({ clientX: 180, clientY: 50 });
const writesBeforeFilterCancel = presetWrites;
documentListeners.get("keydown")({ key: "Escape", preventDefault() {}, stopPropagation() {} });
assert.deepEqual(get("filters").children.map(card => Number(card.dataset.mainFilter)), [0, 1]);
assert.equal(presetWrites, writesBeforeFilterCancel);

const movedSlot = preset.filter_slots[0].id;
context.moveFilter(movedSlot, 1);
assert.equal(preset.filter_slots[1].shapers.before.mode, "hard_clip");
bindings.get("filter4_ws_before_mix").set(.42);
assert.deepEqual(changes.at(-1), ["filter4_ws_before_mix", .42]);
console.log(
  "Filter editor: controls, routing, stage transfer and cancellation, filter reorder and FX targets passed.",
);
