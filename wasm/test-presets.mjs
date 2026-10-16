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
  "Preset files passed: portable gzip format, Unicode names, both FX racks, legacy import and damaged-file rejection.",
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
