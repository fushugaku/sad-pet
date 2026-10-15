// This bridge also runs in AudioWorkletGlobalScope, where TextEncoder and
// TextDecoder are not available in every browser.
function encode(text) {
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
  let text = "";
  for (let i = 0; i < bytes.length; ) {
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

export class Core {
  constructor(module, rate = 48000) {
    this.wasm = new WebAssembly.Instance(module, {}).exports;
    this.wasm.sad_init(rate);
  }
  input(value) {
    const bytes =
      value instanceof ArrayBuffer
        ? new Uint8Array(value)
        : value instanceof Uint8Array
          ? value
          : encode(JSON.stringify(value));
    const pointer = this.wasm.sad_input(bytes.length);
    if (!pointer) throw new Error("The audio file is too large.");
    new Uint8Array(this.wasm.memory.buffer, pointer, bytes.length).set(bytes);
  }
  output() {
    return new Uint8Array(
      this.wasm.memory.buffer,
      this.wasm.sad_output(),
      this.wasm.sad_output_len(),
    );
  }
  call(name, input, ...args) {
    if (input !== undefined) this.input(input);
    const status = this.wasm[name](...args);
    if (status !== 200)
      throw new Error(JSON.parse(decode(this.output())).error);
    return this.output().slice().buffer;
  }
  json(name, input) {
    return JSON.parse(decode(new Uint8Array(this.call(name, input))));
  }
  request(path, data = null) {
    return this.json("sad_command", { path, data });
  }
  process(frames, input = []) {
    if (!Number.isInteger(frames) || frames < 1 || frames > 4096)
      throw new Error("Invalid audio block size.");
    const incoming = new Float32Array(
      this.wasm.memory.buffer,
      this.wasm.sad_audio_input(),
      frames * 2,
    );
    for (let i = 0; i < frames; i++) {
      incoming[i * 2] = input[0]?.[i] ?? 0;
      incoming[i * 2 + 1] = (input[1] || input[0])?.[i] ?? 0;
    }
    const pointer = this.wasm.sad_process(frames);
    // A sample load or recording may grow memory. Always acquire the current
    // buffer after entering WASM, never retain a detached view between blocks.
    return new Float32Array(this.wasm.memory.buffer, pointer, frames * 2);
  }
}
