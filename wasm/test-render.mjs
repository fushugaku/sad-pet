// Render WAV through the shipping browser route and real utility/vocal workers.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { Worker } from "node:worker_threads";

const folder = resolve(process.argv[2] || "dist/web");
const { Core } = await import(pathToFileURL(resolve(folder, "core.mjs")));
const { BrowserBackend } = await import(pathToFileURL(resolve(folder, "browser-backend.mjs")));
const module = await WebAssembly.compile(await readFile(resolve(folder, "sad_pet.wasm")));
const workerUrl = pathToFileURL(resolve(folder, "worker.mjs")).href;
const rate = 16000;

function worker() {
  const source = `import {parentPort} from 'node:worker_threads';
    globalThis.self={postMessage:(message,transfer=[])=>parentPort.postMessage(message,transfer)};
    await import(${JSON.stringify(workerUrl)});
    parentPort.on('message',data=>self.onmessage({data}));`;
  const thread = new Worker(new URL(`data:text/javascript,${encodeURIComponent(source)}`));
  let next = 0;
  const pending = new Map(), events = [], waiting = [];
  thread.on("message", message => {
    if (message.id !== undefined) {
      const request = pending.get(message.id);
      pending.delete(message.id);
      clearTimeout(request.timer);
      message.error ? request.reject(new Error(message.error)) : request.resolve(message.result);
    } else if (waiting.length) waiting.shift()(message);
    else events.push(message);
  });
  return {
    thread,
    call(type, data, transfer = [], path) {
      return new Promise((resolve, reject) => {
        const id = ++next;
        const timer = setTimeout(() => reject(new Error(`Worker timed out: ${type}`)), 15000);
        pending.set(id, { resolve, reject, timer });
        thread.postMessage({ id, type, data, path }, transfer);
      });
    },
    event() {
      return events.length ? Promise.resolve(events.shift()) : new Promise(resolve => waiting.push(resolve));
    },
  };
}

const utility = worker(), vocal = worker();
try {
  const audio = new Core(module, rate);
  audio.call("sad_prepare", audio.call("sad_factory", undefined, 0));
  await utility.call("init", { module, rate });
  await vocal.call("vocal-init", { module, rate });
  await vocal.call("vocal-near", 0);
  await vocal.call("vocal-jobs", []);

  globalThis.location = { href: "https://example.test/sad-pet/" };
  const backend = Object.create(BrowserBackend.prototype);
  Object.assign(backend, {
    ready: Promise.resolve(),
    microphone: { active: false },
    worker: utility,
    vocal,
    audio: {
      async call(type) {
        assert.equal(type, "scene-pack");
        return audio.call("sad_scene_pack");
      },
    },
  });

  async function render() {
    const response = await backend.fetch("/api/render", { body: JSON.stringify({ seconds: 1 }) });
    assert.equal(response.status, 200, response.ok ? "" : await response.text());
    assert.equal(response.headers.get("Content-Type"), "audio/wav");
    const wav = new DataView(await response.arrayBuffer());
    const text = (at, length) => String.fromCharCode(...new Uint8Array(wav.buffer, at, length));
    assert.equal(text(0, 4), "RIFF");
    assert.equal(text(8, 4), "WAVE");
    assert.equal(wav.getUint32(24, true), rate);
    let at = 12;
    while (at + 8 <= wav.byteLength && text(at, 4) !== "data")
      at += 8 + wav.getUint32(at + 4, true);
    assert.equal(text(at, 4), "data");
    const size = wav.getUint32(at + 4, true);
    assert.equal(size, rate * 2 * 4, "A one-second stereo float WAV");
    const samples = new Float32Array(wav.buffer, at + 8, size / 4);
    let peak = 0;
    for (const sample of samples) {
      assert.ok(Number.isFinite(sample));
      peak = Math.max(peak, Math.abs(sample));
    }
    assert.ok(peak > 0.001, `Rendered audio must not be silent: ${peak}`);
    return samples;
  }

  assert.equal((await vocal.call("memory")).linear_bytes, 0);
  await render();
  await render();
  assert.equal((await vocal.call("memory")).linear_bytes, 0,
    "Exporting a scene without vocals initialized the unused Vocal engine");

  // A real rendered phrase must still reach Render WAV when Vocal is in use.
  const context = audio.request("/api/timbres/add", { kind: "vocal", classic: true });
  const part = context.timbres.find(part => part.kind === "vocal");
  audio.request("/api/timbres/remove", { id: 0 });
  audio.request("/api/timbres/select", { id: part.id });
  audio.request("/api/vocal/score", { score: {
    language: "ja", style: { auto_breath: false },
    notes: [{ id: 1, start: 480, length: 480, key: 64, lyric: "あ" }],
  } });
  await vocal.call("vocal-jobs", audio.json("sad_vocal_jobs"));
  const clipReady = async () => {
    for (;;) {
      const event = await vocal.event();
      if (event.type === "vocal-error") throw new Error(event.error);
      if (event.type === "vocal-update" && event.updates.some(update => update.clips.length)) return;
    }
  };
  let timer;
  try {
    await Promise.race([clipReady(), new Promise((_, reject) => {
      timer = setTimeout(() => reject(new Error("Vocal phrase was not rendered")), 15000);
    })]);
  } finally {
    clearTimeout(timer);
  }
  const songs = await vocal.call("vocal-songs");
  assert.equal(songs.length, 1);
  assert.ok(songs[0].clips.length > 0);
  assert.ok(songs[0].clips[0].data.byteLength > 0);
  const sung = await render();
  assert.ok(sung.subarray(0, rate / 20 * 2).every(sample => Math.abs(sample) < 1e-4),
    "Only the vocal phrase should sound after the silent lead-in");
  assert.ok((await vocal.call("memory")).linear_bytes > 0);
} finally {
  await Promise.all([utility.thread.terminate(), vocal.thread.terminate()]);
}
console.log("Render WAV passed: fresh/repeated non-vocal scenes, lazy Vocal memory and real vocal clip export.");
