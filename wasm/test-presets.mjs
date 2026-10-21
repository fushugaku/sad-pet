import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { gunzipSync } from "node:zlib";
import { runInNewContext } from "node:vm";

const window = {};
runInNewContext(
  await readFile(new URL("../rust/web/presets.js", import.meta.url), "utf8"),
  {
    window,
    Blob,
    CompressionStream,
    DecompressionStream,
    Response,
    TextEncoder,
    TextDecoder,
    DataView,
    Uint8Array,
    btoa,
    atob,
  },
);
const { encode, decode } = window.SadPresetFile;
// Restoring a remembered patch must not replace a track the user just added
// while the IndexedDB library was opening.
{
  const bank = Object.create(window.SadPresetLibrary.prototype);
  let edited = true,
    loads = 0;
  bank.actions = { edited: () => edited };
  bank.files = { store: { setting: async () => ({ id: "remembered" }) } };
  bank.users = [{ id: "remembered" }];
  bank.refresh = async () => {};
  bank.render = () => {};
  bank.load = async () => {
    loads++;
  };
  await bank.initialize({}, []);
  assert.equal(loads, 0);
  edited = false;
  await bank.initialize({}, []);
  assert.equal(loads, 1);
}
// Fresh browsers select the startup factory sound; remembered choices take priority.
{
  const factory = Array.from({ length: 6 }, (_, i) => ({ name: i === 5 ? "glasses5" : `Factory ${i}` }));
  const bank = Object.create(window.SadPresetLibrary.prototype);
  let last = null;
  const loads = [];
  bank.actions = { edited: () => false };
  bank.files = { store: { setting: async () => last } };
  bank.users = [{ id: "saved-user" }];
  bank.refresh = async () => {};
  bank.render = () => {};
  bank.load = async (id) => loads.push(id);
  const state = { preset: factory[5], startup_preset_index: 5 };
  await bank.initialize(state, factory);
  assert.equal(bank.selection.id, "factory:5");
  assert.equal(bank.selection.dirty, false);
  assert.equal(loads.length, 0);
  last = { id: "saved-user" };
  await bank.initialize(state, factory);
  assert.deepEqual(loads, ["saved-user"]);
  last = { id: "factory:0" };
  await bank.initialize(state, factory);
  assert.deepEqual(loads, ["saved-user", "factory:0"]);
  last = { id: "deleted-user" };
  await bank.initialize(state, factory);
  assert.equal(loads.length, 2);
  last = { id: "factory:5" };
  bank.selection.dirty = true; // The sample library restored the user's last audio.
  await bank.initialize(state, factory);
  assert.equal(loads.length, 2, "The startup scene was needlessly loaded again");
  assert.equal(bank.selection.dirty, true);
}
const record = {
  id: "1234567890abcdef1234567890abcdef",
  name: "Тихая комната 🎹",
  created_at: 1,
  modified_at: 2,
};
const state = {
  version: 1,
  preset: {
    format: "evil-pet-rust-v1",
    name: record.name,
    source_color_chain: [{ id: 2, kind: "flanger" }],
    pre_color_chain: [{ id: 3, kind: "waveshaper" }],
    color_chain: [{ id: 4, kind: "compressor" }],
  },
  sample_wav: "UklGRg==",
};
const blob = await encode(record, state);
const bytes = new Uint8Array(await blob.arrayBuffer());
assert.deepEqual([...bytes.subarray(0, 8)], [83, 65, 68, 80, 69, 84, 1, 0]);
const headerSize = new DataView(bytes.buffer).getUint32(8, true);
const header = JSON.parse(
  new TextDecoder().decode(bytes.subarray(12, 12 + headerSize)),
);
assert.equal(header.name, record.name);
assert.equal(header.codec, "gzip");
assert.deepEqual(
  JSON.parse(gunzipSync(bytes.subarray(12 + headerSize)).toString()),
  state,
);
const restored = await decode(blob);
assert.equal(restored.state.preset.name, record.name);
assert.equal(restored.state.preset.source_color_chain[0].kind, "flanger");
assert.equal(restored.state.preset.pre_color_chain[0].kind, "waveshaper");
assert.equal(restored.state.preset.color_chain[0].kind, "compressor");
assert.equal(
  (await decode(new Blob([JSON.stringify(state.preset)]))).preset.name,
  record.name,
);
await assert.rejects(() =>
  decode(new Blob([bytes.subarray(0, bytes.length - 4)])),
);
await assert.rejects(() => decode(new Blob(["{}"])));
const corrupt = bytes.slice();
new DataView(corrupt.buffer).setUint32(8, 99999, true);
await assert.rejects(() => decode(new Blob([corrupt])));
console.log(
  "Preset files passed: portable gzip format, Unicode names, Source FX and both racks, legacy import and damaged-file rejection.",
);
if (process.argv[2]) {
  const native = await decode(new Blob([await readFile(process.argv[2])]));
  assert.equal(native.state.version, 1);
  assert.equal(
    Buffer.from(native.state.sample_wav, "base64").subarray(0, 4).toString(),
    "RIFF",
  );
  assert.equal(
    native.state.preset.pre_color_chain[0].settings.waveshaper_type,
    "multi_sine",
  );
  console.log(
    "Native preset export opens in the browser codec with its embedded sample and waveshaper settings.",
  );
}
