// This bridge also runs in AudioWorkletGlobalScope, where TextEncoder and
// TextDecoder are not available in every browser.
const encoder = typeof TextEncoder === "function" ? new TextEncoder() : null;
const decoder = typeof TextDecoder === "function" ? new TextDecoder() : null;
function encode(text) {
  if (encoder) return encoder.encode(text);
  const bytes = [];
  for (const char of text) {
    const c = char.codePointAt(0);
    if (c < 128) bytes.push(c);
    else if (c < 2048) bytes.push(192 | (c >> 6), 128 | (c & 63));
    else if (c < 65536)
      bytes.push(224 | (c >> 12), 128 | ((c >> 6) & 63), 128 | (c & 63));
    else
      bytes.push(
        240 | (c >> 18),
        128 | ((c >> 12) & 63),
        128 | ((c >> 6) & 63),
        128 | (c & 63),
      );
  }
  return new Uint8Array(bytes);
}
function decode(bytes) {
  if (decoder) return decoder.decode(bytes);
  let text = "";
  for (let i = 0; i < bytes.length;) {
    const a = bytes[i++];
    let c = a;
    if (a >= 240)
      c =
        ((a & 7) << 18) |
        ((bytes[i++] & 63) << 12) |
        ((bytes[i++] & 63) << 6) |
        (bytes[i++] & 63);
    else if (a >= 224)
      c = ((a & 15) << 12) | ((bytes[i++] & 63) << 6) | (bytes[i++] & 63);
    else if (a >= 192) c = ((a & 31) << 6) | (bytes[i++] & 63);
    text += String.fromCodePoint(c);
  }
  return text;
}

const NO_INPUT = [];
class Abi {
  constructor(module, rate, prefix) {
    this.wasm = new WebAssembly.Instance(module, {}).exports;
    this.allocateInput = this.wasm[`${prefix}_input`];
    this.outputPointer = this.wasm[`${prefix}_output`];
    this.outputLength = this.wasm[`${prefix}_output_len`];
    this.wasm[`${prefix}_init`](rate);
  }
  input(value) {
    const bytes =
      value instanceof ArrayBuffer
        ? new Uint8Array(value)
        : value instanceof Uint8Array
          ? value
          : encode(JSON.stringify(value));
    const pointer = this.allocateInput(bytes.length);
    if (!pointer) throw new Error("The audio file is too large.");
    new Uint8Array(this.wasm.memory.buffer, pointer, bytes.length).set(bytes);
  }
  output() {
    return new Uint8Array(
      this.wasm.memory.buffer,
      this.outputPointer(),
      this.outputLength(),
    );
  }
  call(name, input, ...args) {
    if (input !== undefined) this.input(input);
    const status = this.wasm[name](...args);
    if (status !== 200)
      throw new Error(JSON.parse(decode(this.output())).error);
    return this.output().slice().buffer;
  }
  json(name, input, ...args) {
    return JSON.parse(decode(new Uint8Array(this.call(name, input, ...args))));
  }
}

export class TelemetryCore extends Abi {
  constructor(module, rate = 48000) {
    super(module, rate, "sad_telemetry");
  }
  snapshot(packet, delta = true) {
    return this.json("sad_telemetry_import", packet, Number(delta));
  }
}

export class Core extends Abi {
  constructor(module, rate = 48000) {
    super(module, rate, "sad");
  }
  request(path, data = null) {
    return this.json("sad_command", { path, data });
  }
  snapshot(target) {
    const length = this.wasm.sad_snapshot();
    if (!length) return 0;
    if (length > target.length)
      throw new Error("Telemetry buffer is too small.");
    const pointer = this.wasm.sad_snapshot_pointer(),
      memory = this.wasm.memory.buffer;
    if (
      this.snapshotView?.buffer !== memory ||
      this.snapshotView?.length < length ||
      this.snapshotView.byteOffset !== pointer
    )
      this.snapshotView = new Uint8Array(memory, pointer, length);
    target.set(this.snapshotView.subarray(0, length));
    return length;
  }
  sourceChunk(bytes, offset) {
    const length = Math.min(32768, bytes.length - offset),
      pointer = this.wasm.sad_source_input(),
      memory = this.wasm.memory.buffer;
    if (
      this.sourceView?.buffer !== memory ||
      this.sourceView.byteOffset !== pointer
    )
      this.sourceView = new Uint8Array(memory, pointer, 32768);
    this.sourceView.set(bytes.subarray(offset, offset + length));
    this.sourceCopied = length;
    return this.wasm.sad_source_push(length / 8);
  }
  process(frames, input = NO_INPUT) {
    if (!Number.isInteger(frames) || frames < 1 || frames > 4096)
      throw new Error("Invalid audio block size.");
    const inputPointer = this.wasm.sad_audio_input(),
      before = this.wasm.memory.buffer;
    if (
      this.incoming?.buffer !== before ||
      this.incoming.byteOffset !== inputPointer ||
      this.incoming.length !== frames * 2
    )
      this.incoming = new Float32Array(before, inputPointer, frames * 2);
    const incoming = this.incoming;
    for (let i = 0; i < frames; i++) {
      incoming[i * 2] = input[0]?.[i] ?? 0;
      incoming[i * 2 + 1] = (input[1] || input[0])?.[i] ?? 0;
    }
    const pointer = this.wasm.sad_process(frames);
    const after = this.wasm.memory.buffer;
    if (
      this.processed?.buffer !== after ||
      this.processed.byteOffset !== pointer ||
      this.processed.length !== frames * 2
    )
      this.processed = new Float32Array(after, pointer, frames * 2);
    return this.processed;
  }
}
