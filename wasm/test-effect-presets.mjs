import assert from "node:assert/strict";
import { readFile, writeFile, mkdir } from "node:fs/promises";
import { runInNewContext } from "node:vm";
import { gunzipSync } from "node:zlib";
import { webcrypto } from "node:crypto";
const window = {};
runInNewContext(await readFile(new URL("../rust/web/presets.js", import.meta.url), "utf8"), {
  window, Blob, CompressionStream, DecompressionStream, Response, TextEncoder,
  TextDecoder, Uint8Array, DataView, btoa, atob, structuredClone, crypto: webcrypto,
});
const { encode, decode } = window.SadEffectPresetFile;
const record = {
  id: "1234567890abcdef1234567890abcdef", created_at: 1, modified_at: 2,
  format: "sad-pet-effect-v1", name: "Мягкое эхо 🎚", kind: "lcr_delay", mix: 0.83,
  settings: { lcr_sync: false, lcr_smooth: true, lcr_feedback: 0.71, reverb_type: "plate_s" },
};
const blob = await encode(record), bytes = new Uint8Array(await blob.arrayBuffer());
assert.deepEqual([...bytes.subarray(0,8)], [83,65,68,70,88,1,0,0]);
assert.deepEqual(JSON.parse(gunzipSync(bytes.subarray(8)).toString()), record);
assert.equal((await decode(blob)).name, record.name);
assert.equal((await decode(blob)).settings.lcr_smooth, true);
await assert.rejects(() => decode(new Blob([bytes.subarray(0, bytes.length - 4)])));
await assert.rejects(() => decode(new Blob(["{}"])));
await assert.rejects(() => decode(new Blob([new Uint8Array(65537)])));
const data = new Map();
const bank = Object.create(window.SadEffectPresets.prototype);
Object.assign(bank, {
  actions: { native: false, notify() {} },
  defaults: { mix: 0.5, settings: { lcr_sync: true, lcr_smooth: false, lcr_feedback: 0.5, reverb_type: "plate_s" } },
  types: [["lcr_delay","L/C/R Delay"],["reverb","Reverb"]],
  choices: { reverb_type: ["plate_s","room"] }, records: [], views: new Set(), selections: new Map(), revision: 0,
  store: {
    read: async () => [...data].map(([key,value]) => ({key,value})),
    setting: async (key,value) => value === undefined ? data.get(key) : data.set(key, structuredClone(value)),
  },
});
await bank.refresh();
const saved = await bank.write(record);
await bank.refresh();
assert.equal(bank.records.length, 1);
assert(bank.matches({ kind: record.kind, mix: record.mix, settings: {...record.settings, reverb_type:"room"} }, saved),
  "inactive algorithms do not make the current effect preset dirty");
await assert.rejects(() => bank.write(record), /already exists/);
const updated = await bank.write({...record, mix:0.2}, saved.id);
assert.equal(updated.id, saved.id);
assert.equal(updated.created_at, saved.created_at);
const other = await bank.write({...record, kind:"reverb"});
assert.notEqual(other.id, saved.id, "different effect types can have the same preset name");
await bank.refresh();
await bank.delete({get:()=>({kind:"reverb",mix:record.mix,settings:record.settings})});
assert.equal(bank.records.length, 1);
await bank.undo();
assert.equal(bank.records.length, 2);
const same = await bank.write({...record, name:"Alternate"});
await bank.refresh();
const view = {key:"part:slot:type", get:()=>({kind:record.kind,mix:record.mix,settings:record.settings})};
bank.selections.set(view.key, same.id);
assert.equal(bank.match(view).id, same.id, "identical presets keep the name explicitly selected");
bank.selections.set(view.key, saved.id);
assert.equal(bank.match(view), undefined, "editing retains the selected preset for overwrite rather than another matching preset");
assert.throws(() => bank.validate({...record, settings:{lcr_feedback:NaN}}));
assert.throws(() => bank.validate({...record, settings:{reverb_type:"missing"}}));
if (process.argv.includes("--fixture")) {
  await mkdir("target/effect-presets-qa", {recursive:true});
  await writeFile("target/effect-presets-qa/browser-export.sadfx", bytes);
}
console.log("Effect presets: portable gzip, Unicode, shared per-type banks, overwrite, soft delete/undo, modes and damaged-file rejection passed.");
