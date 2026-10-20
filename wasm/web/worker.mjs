import { Core } from "./core.mjs";
import { Mp3Encoder } from "./vendor/lamejs.mjs";

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

function mp3(chunks, rate) {
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

async function handle(message) {
  const { id, type, data } = message;
  try {
    let result;
    switch (type) {
      case "snapshot": {
        const packet = core.json(
          "sad_snapshot_import",
          new Uint8Array(message.buffer, 0, message.length),
        );
        self.postMessage(
          {
            type: "snapshot",
            ...packet,
            sequence: message.sequence,
            buffer: message.buffer,
          },
          [message.buffer],
        );
        return;
      }
      case "init":
        core = new Core(data.module, data.rate);
        result = true;
        break;
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
        result = {
          prepared: core.call("sad_pack"),
          info: core.request("/sample-info"),
        };
        break;
      case "source-fx": {
        if (data.generation !== sourceGeneration) {
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
        break;
      }
      case "sample":
        result = core.call("sad_sample_wav");
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
            : mp3(take.chunks, take.rate);
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
