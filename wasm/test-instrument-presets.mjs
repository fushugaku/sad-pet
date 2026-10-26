// Instrument presets in the editor: the .sadinst file layout the native
// library also reads, and the browser library's save, replace, categories,
// import and load requests.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { runInNewContext } from "node:vm";
import { gunzipSync } from "node:zlib";
import { webcrypto } from "node:crypto";
const window = {};
runInNewContext(await readFile(new URL("../rust/web/presets.js", import.meta.url), "utf8"), {
  window, Blob, CompressionStream, DecompressionStream, Response, TextEncoder,
  TextDecoder, Uint8Array, DataView, btoa, atob, structuredClone, crypto: webcrypto,
});
const { encode, decode } = window.SadInstrumentFile;
// Objects made in the sandbox, compared by value.
const plain = (value) => JSON.parse(JSON.stringify(value));
const record = {
  id: "1234567890abcdef1234567890abcdef", name: "Стеклянный пэд 🎹", category: "Пэды",
  kind: "granular", created_at: 1, modified_at: 2,
};
const state = {
  version: 1, meta: { name: record.name, kind: "granular" }, preset: { format: "evil-pet-rust-v1", name: "x" },
  sample_name: "Glass.wav", sample_wav: "UklGRg==", source: "file", region: { start: 0, end: 1 },
  fades: { fade_in: 0, fade_out: 0 }, sync: true, frozen: false, bypass: false, midi_gate: true,
  playing: false, timbres: [], active_timbre: 0,
};

// Magic, the header's length, the header, then the gzipped timbre state:
// the layout `plugin/src/instruments.rs` reads.
const blob = await encode(record, state), bytes = new Uint8Array(await blob.arrayBuffer());
assert.deepEqual([...bytes.subarray(0, 8)], [83, 65, 68, 73, 78, 83, 1, 0]);
const length = new DataView(bytes.buffer).getUint32(8, true);
assert.deepEqual(JSON.parse(new TextDecoder().decode(bytes.subarray(12, 12 + length))), { ...record, codec: "gzip" });
assert.deepEqual(JSON.parse(gunzipSync(bytes.subarray(12 + length)).toString()), state);
assert.deepEqual(plain(await decode(blob)), { record, state });
await assert.rejects(() => decode(new Blob([bytes.subarray(0, bytes.length - 4)])), /damaged/);
await assert.rejects(() => decode(new Blob(["SADPET\x01\0nope"])), /\.sadinst/);
await assert.rejects(async () => decode(await encode({ ...record, kind: "bus" }, state)), /Unsupported/);
await assert.rejects(async () => decode(await encode({ ...record, category: "x".repeat(41) }, state)), /category/);
await assert.rejects(async () => decode(await encode(record, { ...state, timbres: [state] })), /invalid/);

const data = new Map(), requests = [];
let documentState = { ...structuredClone(state), meta: { id: 3, name: "Synth 1", kind: "synth", gain: 0.5 }, playing: true };
const context = { active: 3, timbres: [{ id: 3, kind: "synth", name: "Synth 1" }, { id: 4, kind: "bus", name: "Bus 1" }] };
const bank = Object.create(window.SadInstrumentPresets.prototype);
Object.assign(bank, {
  browser: { hidden: true }, confirm: { hidden: false }, records: [], category: null, busy: false,
  actions: {
    native: false, notify() {}, flush: async () => {},
    context: () => context,
    request: async (path, data) => {
      requests.push([path, data]);
      if (path === "/api/timbres/document") return structuredClone(documentState);
      if (path === "/api/timbres/add") return { active: 9 };
      return {};
    },
    change: async (work) => work(),
    rename: async (id, name) => { context.timbres.find((t) => t.id === id).name = name; },
  },
  store: {
    db: {},
    read: async () => [...data].map(([key, value]) => ({ key, value })),
    setting: async (key, value) => (value === undefined ? data.get(key) : data.set(key, value)),
    remove: async (_, key) => data.delete(key),
  },
});
const stored = async (id) => decode(data.get("instrument:" + id).blob);
const find = (name, category) => bank.records.find((r) => r.name === name && r.category === category);

// Save keeps the active timbre's sound alone, under the name it gives the timbre.
await bank.refresh();
await bank.save("Wide Lead", "Leads");
assert.equal(context.timbres[0].name, "Wide Lead");
const lead = find("Wide Lead", "Leads");
assert.equal(lead.kind, "synth");
assert.equal(Object.hasOwn(lead, "blob"), false, "the list holds no files");
const saved = await stored(lead.id);
assert.deepEqual(plain(saved.state.meta), { name: "Wide Lead", kind: "synth" });
assert.equal(saved.state.playing, false);
// The same name in the same category replaces it.
await bank.save("wide lead", "leads");
assert.equal(bank.records.length, 1);
assert.equal(bank.records[0].id, lead.id);
assert.equal(bank.records[0].created_at, lead.created_at);
const replaced = await stored(lead.id);
await bank.save("Wide Lead", "Pads");
assert.equal(bank.records.length, 2);
const pad = find("Wide Lead", "Pads");

// Names stay unique within a category through renames and moves.
await assert.rejects(() => bank.edit(find("wide lead", "leads"), "Wide Lead", "Pads"), /Pads/);
await assert.rejects(() => bank.renameCategory("leads", "Pads"), /wide lead/i);
await bank.edit(find("wide lead", "leads"), "Lead 2", "leads");
await bank.renameCategory("leads", "Pads");
assert.deepEqual(plain(bank.records).map((r) => `${r.category}/${r.name}`), ["Pads/Lead 2", "Pads/Wide Lead"]);
assert.equal((await stored(lead.id)).record.category, "Pads", "the file's own header moves with it");
assert.deepEqual(plain((await stored(lead.id)).state), plain(replaced.state), "and its sound stays as saved");

// An import is a copy of its own, renamed when its category has the name.
const imported = await bank.importFile(data.get("instrument:" + pad.id).blob);
assert.notEqual(imported.id, pad.id);
assert.deepEqual(plain([imported.name, imported.category]), ["Wide Lead (2)", "Pads"]);

// Load replaces the active timbre's sound, or adds a timbre for it.
requests.length = 0;
await bank.load(find("Lead 2", "Pads"), false);
assert.deepEqual(requests.map(([path, data]) => [path, data.id, data.name]), [["/api/timbres/load", 3, "Lead 2"]]);
assert.equal(requests[0][1].state.meta.kind, "synth");
requests.length = 0;
await bank.load(pad, true);
assert.deepEqual(requests.map(([path, data]) => [path, data.kind ?? data.id]), [["/api/timbres/add", "synth"], ["/api/timbres/load", 9]]);

// The Save button says Replace for a name its category already has.
Object.assign(bank, { dialog: { mode: "save" }, nameInput: { value: " wide lead " }, categoryInput: { value: "pads" }, submit: {} });
bank.labelSubmit();
assert.equal(bank.submit.textContent, "Replace");
bank.categoryInput.value = "";
bank.labelSubmit();
assert.equal(bank.submit.textContent, "Save");

// A Bus holds no instrument; deleting removes the file.
context.active = 4;
await assert.rejects(() => bank.save("Bus sound", ""), /Bus and Master/);
await bank.remove(imported);
assert.equal(data.has("instrument:" + imported.id), false);
assert.equal(bank.records.length, 2);
console.log("instrument preset editor tests passed");
