// Real MP3 decoding in the shipping WASM, then the shared file-library and
// wavetable import flows. Stored sources always remain WAV.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { runInNewContext } from "node:vm";

const folder = resolve(process.argv[2] || "dist/web");
const { Core } = await import(pathToFileURL(resolve(folder, "core.mjs")));
const module = await WebAssembly.compile(await readFile(resolve(folder, "sad_pet.wasm")));
const core = new Core(module, 48000);
const mono = new File([await readFile(new URL("../rust/tests/fixtures/mono-44100.mp3", import.meta.url))], "Mono.MP3", { type: "audio/mpeg" });
const stereo = new File([await readFile(new URL("../rust/tests/fixtures/stereo-32000.mp3", import.meta.url))], "Stereo.mp3", { type: "audio/mpeg" });
let decodes = 0;
const window = {
  fetch: async (path, options) => {
    assert.equal(path, "/api/audio/decode");
    decodes++;
    try {
      return new Response(core.call("sad_audio_decode", await options.body.arrayBuffer(), Number(options.headers["X-Max-Bytes"])), { headers: { "Content-Type": "audio/wav" } });
    } catch (error) {
      return new Response(JSON.stringify({ error: error.message }), { status: 400 });
    }
  },
};
const context = { window, Blob, File, Uint8Array, btoa, navigator: { storage: {} } };
runInNewContext(await readFile(new URL("../rust/web/library.js", import.meta.url), "utf8"), context);
const wavFile = window.SadAudioImport.wavFile;
const wav = await wavFile(mono);
assert.equal(wav.name, "Mono.wav");
assert.equal(wav.type, "audio/wav");
const bytes = await wav.arrayBuffer();
const info = core.request("/api/wavetables/import", { wav: Buffer.from(bytes).toString("base64"), inspect: true });
assert.equal(info.channels, 1);
assert.equal(info.sample_rate, 44100);
assert.equal(info.samples_per_channel, 16384, "MP3 encoder delay/padding changed the wavetable length");
const count = decodes;
assert.equal(await wavFile(wav), wav, "An existing WAV was unnecessarily re-encoded");
assert.equal(decodes, count);
await assert.rejects(wavFile(mono, 8192), /as WAV/);

// Importing MP3 stores a WAV and activates that stored file through the normal
// granular sample preparation, with a useful name and the original audio rate.
const library = Object.create(window.EvilFileLibrary.prototype);
const stored = [];
let active;
Object.assign(library, {
  ready: true, actions: {},
  storeBlob: async (blob, metadata) => {
    const record = { ...metadata, blob };
    stored.push(record);
    return record;
  },
  activate: async record => {
    core.call("sad_prepare", await record.blob.arrayBuffer());
    active = record;
  },
});
await library.importFiles([stereo]);
assert.equal(active, stored[0]);
assert.equal(active.name, "Stereo.wav");
assert.equal(active.displayName, "Stereo");
assert.equal(active.blob.type, "audio/wav");
assert.equal(core.request("/sample-info").seconds, 0.25);
const current = active;
await assert.rejects(library.importFiles([new File(["ID3damaged MP3"], "Broken.mp3")]), /MP3/);
assert.equal(stored.length, 1, "A failed decoder polluted the library");
assert.equal(active, current, "A failed decoder replaced the current sound");

// Periodic MP3 sources retain the existing frame/channel choice. Arbitrary
// lengths open Wave Maker, where the user previews and applies the new table.
runInNewContext(await readFile(new URL("../rust/web/wavetable.js", import.meta.url), "utf8"), context);
const editor = Object.create(window.SadWavetable.Editor.prototype);
const settings = { asset_id: "factory:wt-basic-morph:v1" };
let periodic, maker, failure;
Object.assign(editor, {
  previews: new Map(), settings: () => settings,
  error: (_, error) => { failure = error; },
  hooks: { request: async (path, data) => core.request(path, data) },
  importDialog: (_, input, info) => { periodic = { input, info }; },
  tools: { maker: (_, input) => { maker = input; } },
  catalog: async () => {},
  choose: async (_, id) => { settings.asset_id = id; },
});
await editor.importFile(0, mono);
assert.equal(failure, null);
assert.equal(periodic.input.name, "Mono");
assert.equal(periodic.info.sample_rate, 44100);
assert.equal(settings.asset_id, "factory:wt-basic-morph:v1", "Import applied before the frame choice");
await editor.applyImport(0, { ...periodic.input, frame_size: 2048 });
assert.ok(settings.asset_id.startsWith("user:"));
const selected = settings.asset_id;
await editor.importFile(0, stereo);
assert.equal(failure, null);
assert.equal(maker.name, "Stereo");
assert.equal(settings.asset_id, selected, "Wave Maker applied before confirmation");
const made = core.request("/api/wavetables/make", {
  ...maker, generation: 1,
  settings: { method: "spectral", frames: 8, channel: "mix" },
});
assert.equal(made.frames, 8);
assert.ok(made.waves[0].some(x => Math.abs(x) > 0.05));
console.log("MP3 import: native-rate gapless WAV, granular library, safe failure, periodic tables and Wave Maker passed.");
