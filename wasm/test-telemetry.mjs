// Compare complete JSON and deltas against the original audio-host decoder.
// Optional reference WASM enables before/after compatibility checks.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { Core, TelemetryCore } from "../dist/web/core.mjs";

const module = await WebAssembly.compile(await readFile("dist/web/sad_pet.wasm"));
const reference = process.argv[2]
  ? await WebAssembly.compile(await readFile(process.argv[2])) : module;

for (const rate of [8000, 44100, 48000, 96000]) {
  const audio = new Core(module, rate), legacy = new Core(reference, rate);
  const light = new TelemetryCore(module, rate);
  assert.ok(light.wasm.memory.buffer.byteLength < legacy.wasm.memory.buffer.byteLength,
    "Telemetry initialized full audio storage");
  audio.call("sad_prepare", audio.call("sad_factory", undefined, 0));
  audio.call("sad_load", audio.call("sad_pack"));
  let oversized = true;
  const publish = (delta = true, success = 1) => {
    audio.process(128);
    // The destination pool may be larger than this timbre's own WASM buffer.
    const bytes = new Uint8Array(oversized ? audio.wasm.memory.buffer.byteLength
      : audio.wasm.sad_snapshot_capacity());
    oversized = false;
    const length = audio.snapshot(bytes);
    assert.ok(length > 0);
    const packet = bytes.subarray(0, length);
    const expected = legacy.json(delta ? "sad_snapshot_import_delta" : "sad_snapshot_import", packet);
    const actual = light.snapshot(packet, delta);
    assert.deepEqual(actual, expected, `Decoder changed state at ${rate} Hz`);
    audio.wasm.sad_snapshot_ack(audio.wasm.sad_snapshot_sequence(), success);
    return packet.slice();
  };
  publish();
  publish();
  audio.request("/api/transport", { playing: true, midi_gate: false });
  audio.request("/api/parameters", { filter: 0.41, in_gain: -6.1, out_gain: 2.3 });
  publish();
  publish(false);
  audio.request("/api/region", { start: 0.1, end: 0.7 });
  publish();
  const drums = audio.request("/api/timbres/add", { kind: "drums" }).active;
  publish();
  audio.request("/api/drums/edit", [{ track: 0, step: 127, field: 0, value: 0.8 }]);
  audio.request("/api/parameters", { drum_granular: 1, drum1_0: 73 });
  publish();
  publish(false);
  const obsolete = publish();
  audio.request("/api/timbres/add", { kind: "granular" });
  publish();
  audio.request("/api/timbres/select", { id: 0 });
  publish();
  audio.request("/api/timbres/remove", { id: drums });
  publish();
  assert.throws(() => light.snapshot(obsolete), /Obsolete timbre snapshot/);
  publish(false);
  // A worker restarted between packets must request a full configuration.
  const restarted = new TelemetryCore(module, rate);
  const delta = publish(true, 0);
  assert.throws(() => restarted.snapshot(delta), /configuration is missing/);
  const resync = publish();
  assert.deepEqual(restarted.snapshot(resync), light.snapshot(resync));
  assert.throws(() => light.snapshot(new Uint8Array(8)));
  publish();
  assert.ok(light.wasm.memory.buffer.byteLength < legacy.wasm.memory.buffer.byteLength,
    "Multiple timbres allocated synthesis storage in the decoder");
  console.log(`Telemetry ${rate}: full/delta state, edits, timbre switch/removal and resync match; `
    + `${legacy.wasm.memory.buffer.byteLength / 2 ** 20} -> ${light.wasm.memory.buffer.byteLength / 2 ** 20} MiB`);
}
