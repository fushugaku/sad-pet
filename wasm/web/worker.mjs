import { Core, TelemetryCore, VocalCore } from "./core.mjs";


let core,
  take = null;
let queue = Promise.resolve();
let sourceGeneration = 0;

function wave(chunks, rate, frames) {
  const buffer = new ArrayBuffer(44 + frames * 8),
    view = new DataView(buffer);
  const text = (offset, value) => {
    for (let i = 0; i < value.length; i++)
      view.setUint8(offset + i, value.charCodeAt(i));
  };
  text(0, "RIFF");
  view.setUint32(4, buffer.byteLength - 8, true);
  text(8, "WAVE");
  text(12, "fmt ");
  view.setUint32(16, 16, true);
  view.setUint16(20, 3, true);
  view.setUint16(22, 2, true);
  view.setUint32(24, rate, true);
  view.setUint32(28, rate * 8, true);
  view.setUint16(32, 8, true);
  view.setUint16(34, 32, true);
  text(36, "data");
  view.setUint32(40, frames * 8, true);
  const data = new Float32Array(buffer, 44);
  let offset = 0;
  for (const chunk of chunks) {
    data.set(chunk, offset);
    offset += chunk.length;
  }
  return buffer;
}

async function mp3(chunks, rate) {
  const { Mp3Encoder } = await import("./vendor/lamejs.mjs");
  const encoder = new Mp3Encoder(2, rate, 192),
    output = [];
  const left = new Int16Array(1152),
    right = new Int16Array(1152);
  let length = 0;
  const flush = () => {
    const bytes = encoder.encodeBuffer(
      left.subarray(0, length),
      right.subarray(0, length),
    );
    if (bytes.length) output.push(new Uint8Array(bytes));
    length = 0;
  };
  for (const chunk of chunks) {
    for (let i = 0; i < chunk.length; i += 2) {
      left[length] = Math.round(Math.max(-1, Math.min(1, chunk[i])) * 32767);
      right[length++] = Math.round(
        Math.max(-1, Math.min(1, chunk[i + 1])) * 32767,
      );
      if (length === 1152) flush();
    }
  }
  if (length) flush();
  const end = encoder.flush();
  if (end.length) output.push(new Uint8Array(end));
  const bytes = new Uint8Array(output.reduce((n, b) => n + b.length, 0));
  let offset = 0;
  for (const block of output) {
    bytes.set(block, offset);
    offset += block.length;
  }
  return bytes.buffer;
}

// Vocal rendering: one phrase per task, so new jobs and the playhead are
// seen between phrases; each finished phrase goes straight to the worklet.
let vocalNear = 0,
  vocalPumping = false;
function postVocalUpdates() {
  const updates = core.updates();
  const transfer = [];
  for (const update of updates) {
    update.clips = update.fresh.map((key) => {
      const data = core.clip(key);
      transfer.push(data);
      const entry = update.plan.find((p) => p[0] === key);
      return { key, start: entry ? entry[1] : 0, data };
    });
    delete update.fresh;
  }
  if (updates.length) self.postMessage({ type: "vocal-update", updates }, transfer);
}
function pumpVocal() {
  if (vocalPumping) return;
  vocalPumping = true;
  const step = () => {
    let rendered = 0;
    try {
      rendered = core.step(vocalNear);
      if (rendered) postVocalUpdates();
    } catch (error) {
      self.postMessage({ type: "vocal-error", error: error.message });
    }
    if (rendered) setTimeout(step, 0);
    else vocalPumping = false;
  };
  setTimeout(step, 0);
}

async function handle(message) {
  const { id, type, data } = message;
  try {
    let result;
    switch (type) {
      case "snapshot": {
        const bytes = new Uint8Array(message.buffer, 0, message.length);
        const packet = core instanceof TelemetryCore
          ? core.snapshot(bytes)
          : core.json("sad_snapshot_import_delta", bytes);
        self.postMessage(
          {
            type: "snapshot",
            ...packet,
            stamp: message.stamp || 0,
            sequence: message.sequence,
            buffer: message.buffer,
          },
          [message.buffer],
        );
        return;
      }
      case "vocal-init":
        core = new VocalCore(data.module, data.rate);
        result = true;
        break;
      case "vocal-jobs":
        for (const job of data) core.command(job);
        postVocalUpdates();
        pumpVocal();
        result = true;
        break;
      case "vocal":
        result = core.command(data);
        if (data.type === "voice-remove") {
          postVocalUpdates();
          pumpVocal();
        }
        break;
      case "vocal-near":
        vocalNear = Number(data) || 0;
        pumpVocal();
        result = true;
        break;
      case "vocal-voice":
        result = core.addVoice(data.id, data.bytes);
        postVocalUpdates();
        pumpVocal();
        break;
      case "vocal-import": {
        core.command({
          type: "import-begin",
          language: data.language || null,
          name: data.name || null,
        });
        for (const file of data.files) core.importFile(file.path, file.data);
        result = core.importFinish();
        break;
      }
      case "init":
        core = data.telemetry
          ? new TelemetryCore(data.module, data.rate)
          : new Core(data.module, data.rate);
        result = true;
        break;
      case "scene-document":
        core.call("sad_scene_adopt", data);
        result = core.json("sad_scene_document");
        break;
      case "scene-prepare":
        result = core.call("sad_scene_prepare", data);
        break;
      // Instrument presets: the active timbre's state, and a timbre's new
      // sound prepared here, where Source FX will read its sample.
      case "part-document":
        core.call("sad_scene_adopt", data);
        result = core.json("sad_part_document");
        break;
      case "instrument-prepare":
        result = core.call("sad_instrument_prepare", data);
        break;
      case "scene-render": {
        // Rendered Vocal phrases arrive from the vocal worker.
        const wasm = core.wasm;
        for (const song of data.vocal || [])
          for (const clip of song.clips) {
            const samples = new Float32Array(clip.data);
            const pointer = wasm.sad_render_clip(song.id, clip.start, samples.length);
            new Float32Array(wasm.memory.buffer, pointer, samples.length).set(samples);
          }
        result = core.call("sad_scene_render_document", {
          seconds: data.seconds,
          scene: data.scene,
        });
        break;
      }
      case "vocal-songs": {
        const songs = core.json("sad_vocal_songs");
        const transfer = [];
        for (const song of songs)
          song.clips = song.clips.map(([key, start]) => {
            const data = core.clip(key);
            transfer.push(data);
            return { start, data };
          });
        self.postMessage({ id, result: songs }, transfer);
        return;
      }
      case "startup": {
        const scene = core.request("/api/startup");
        core.request("/api/timbres/reset", {});
        core.request("/api/input", { active: false });
        core.call(
          "sad_prepare",
          core.call("sad_factory", undefined, scene.sample_index),
        );
        core.call("sad_sample_name", {
          name: scene.sample_name,
          id: scene.sample_id,
        });
        sourceGeneration = data.generation ?? sourceGeneration + 1;
        core.wasm.sad_source_generation(sourceGeneration);
        core.request("/api/preset", scene.preset);
        core.request("/api/region", scene.region);
        core.request("/api/fades", scene.fades);
        core.request("/api/transport", {
          playing: false,
          midi_gate: true,
          frozen: false,
          bypass: false,
        });
        result = core.call("sad_scene_pack");
        break;
      }
      case "request":
        result = core.request(message.path, data);
        break;
      case "factory":
        result = core.call("sad_factory", undefined, data);
        break;
      case "prepare":
        core.call("sad_prepare", data.wav);
        core.call("sad_sample_name", { name: data.name, id: data.libraryId });
        sourceGeneration = data.generation ?? sourceGeneration + 1;
        core.wasm.sad_source_generation(sourceGeneration);
        result = {
          prepared: core.call("sad_pack"),
          info: core.request("/sample-info"),
        };
        break;
      case "source-fx": {
        const previous =
          data.timbre === undefined
            ? null
            : core.wasm.sad_timbre_target(data.timbre);
        try {
          if (
            previous === 0xffffffff ||
            data.generation !== core.wasm.sad_source_current_generation()
          ) {
            result = { stale: true };
            break;
          }
          const prepared = data.recipe.slot
            ? core.call("sad_source_prepare", data.recipe)
            : null;
          let header = null,
            offset = 0;
          if (prepared) {
            const length = new DataView(prepared).getUint32(8, true);
            header = JSON.parse(
              new TextDecoder().decode(new Uint8Array(prepared, 12, length)),
            );
            offset = 12 + length;
          }
          result = { prepared, header, offset, recipe: data.recipe };
        } finally {
          if (previous !== null) core.wasm.sad_timbre_target(previous);
        }
        break;
      }
      case "sample":
        result = core.call("sad_sample_wav");
        break;
      // Sampler: preparation, Sample editor edits and analysis, bounces.
      case "sampler-prepare":
        result = core.call("sad_sampler_prepare", data);
        break;
      case "sampler-process": {
        let bytes = new Uint8Array(data.wav);
        const request = { ...data.request };
        if (data.clip) {
          const clip = new Uint8Array(data.clip);
          const all = new Uint8Array(bytes.length + clip.length);
          all.set(bytes);
          all.set(clip, bytes.length);
          bytes = all;
          request.clip = clip.length;
        }
        result = core.prefixedCall("sad_sampler_process", JSON.stringify(request), bytes);
        break;
      }
      case "sampler-analyze":
        result = core.prefixedJson("sad_sampler_analyze", JSON.stringify(data.request), data.wav);
        break;
      case "sampler-adopt":
        // Bounces and exports read this worker's copy of the scene.
        core.call("sad_scene_adopt", data);
        result = true;
        break;
      case "sampler-bounce":
        result = core.call("sad_sampler_bounce", data);
        break;
      case "sampler-wav":
        result = core.call("sad_sampler_wav", new TextEncoder().encode(data.id), data.part);
        break;
      case "trim":
        result = core.call("sad_trim", data);
        break;
      case "render":
        result = core.call("sad_render", data);
        break;
      case "record-start":
        take = {
          id: data.id,
          rate: data.rate,
          chunks: [],
          frames: 0,
          done: false,
        };
        result = true;
        break;
      case "record-chunk":
        if (take && !take.done) {
          const chunk = new Float32Array(
            data,
            0,
            message.length ?? data.byteLength / 4,
          ).slice();
          take.chunks.push(chunk);
          take.frames += chunk.length / 2;
        }
        self.postMessage({ type: "record-return", buffer: data }, [data]);
        return;
      case "record-stop":
        if (!take) throw new Error("There is no recording yet.");
        take.done = true;
        result = { frames: take.frames };
        break;
      case "record-wav":
      case "record-mp3":
        if (!take?.done || data !== take.id)
          throw new Error(
            "This recording is no longer in the engine. Open its saved copy in Library.",
          );
        result =
          type === "record-wav"
            ? wave(take.chunks, take.rate, take.frames)
            : await mp3(take.chunks, take.rate);
        break;
      default:
        throw new Error(`Unknown worker command: ${type}`);
    }
    const transfers =
      result instanceof ArrayBuffer
        ? [result]
        : result?.prepared
          ? [result.prepared]
          : [];
    self.postMessage({ id, result }, transfers);
  } catch (error) {
    if (type === "snapshot") {
      self.postMessage(
        {
          type: "snapshot",
          sequence: message.sequence,
          buffer: message.buffer,
          error: error.message,
        },
        [message.buffer],
      );
      return;
    }
    self.postMessage({ id, error: error.message });
  }
}
self.onmessage = ({ data }) => {
  queue = queue.then(() => handle(data));
};
