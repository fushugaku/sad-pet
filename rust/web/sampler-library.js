"use strict";
// The Sampler's sample library: every sample, take, slice, edit and bounce is
// a WAV in IndexedDB, and packs are named sets of samples with an optional
// kit. Packs move between browsers as .zip files: pack.json and the WAVs, so
// other software opens them too.
(() => {
  const DB = "sad-pet-samples";

  /** Mono or stereo PCM (8/16/24/32-bit) or float WAV as two channels. */
  function parseWav(buffer) {
    const view = new DataView(buffer);
    const text = (offset) =>
      String.fromCharCode(...new Uint8Array(buffer, offset, 4));
    if (buffer.byteLength < 12 || text(0) !== "RIFF" || text(8) !== "WAVE")
      return null;
    let format = null,
      data = null;
    for (let offset = 12; offset + 8 <= buffer.byteLength; ) {
      const id = text(offset),
        size = view.getUint32(offset + 4, true);
      if (id === "fmt ")
        format = {
          tag: view.getUint16(offset + 8, true),
          channels: view.getUint16(offset + 10, true),
          rate: view.getUint32(offset + 12, true),
          bits: view.getUint16(offset + 22, true),
        };
      else if (id === "data")
        data = { offset: offset + 8, size: Math.min(size, buffer.byteLength - offset - 8) };
      offset += 8 + size + (size & 1);
    }
    if (!format || !data || !format.channels) return null;
    const bytes = format.bits / 8,
      stride = bytes * format.channels;
    const frames = Math.floor(data.size / stride);
    const left = new Float32Array(frames),
      right = new Float32Array(frames);
    const float = format.tag === 3 || (format.tag === 0xfffe && format.bits === 32 && isFloatExtensible(view, buffer));
    const read = (at) => {
      if (float) return bytes === 8 ? view.getFloat64(at, true) : view.getFloat32(at, true);
      if (bytes === 1) return (view.getUint8(at) - 128) / 128;
      if (bytes === 2) return view.getInt16(at, true) / 32768;
      if (bytes === 3) {
        const v = view.getUint8(at) | (view.getUint8(at + 1) << 8) | (view.getInt8(at + 2) << 16);
        return v / 8388608;
      }
      return view.getInt32(at, true) / 2147483648;
    };
    for (let i = 0; i < frames; i++) {
      const at = data.offset + i * stride;
      left[i] = read(at);
      right[i] = format.channels > 1 ? read(at + bytes) : left[i];
    }
    return { rate: format.rate, left, right, frames, channels: Math.min(2, format.channels) };
  }
  // WAVE_FORMAT_EXTENSIBLE: the sub-format GUID starts with the format tag.
  function isFloatExtensible(view, buffer) {
    for (let offset = 12; offset + 8 <= buffer.byteLength; ) {
      const id = String.fromCharCode(...new Uint8Array(buffer, offset, 4));
      const size = view.getUint32(offset + 4, true);
      if (id === "fmt " && size >= 40) return view.getUint16(offset + 32, true) === 3;
      offset += 8 + size + (size & 1);
    }
    return false;
  }

  /** A 32-bit float stereo WAV. */
  function encodeWav(rate, left, right = left) {
    const frames = left.length;
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
    for (let i = 0; i < frames; i++) {
      data[i * 2] = left[i];
      data[i * 2 + 1] = right[i];
    }
    return buffer;
  }

  /** WAV at its own rate; anything else the browser decodes (MP3, AAC, OGG,
   * FLAC, AIFF on Safari) at the audio device's rate. */
  async function decodeAudio(buffer) {
    const wav = parseWav(buffer);
    if (wav && wav.frames) return wav;
    const Context = window.OfflineAudioContext || window.webkitOfflineAudioContext;
    if (!Context) throw new Error("This browser cannot decode the file");
    const context = new Context(2, 1, 48000);
    const audio = await context.decodeAudioData(buffer.slice(0));
    const left = audio.getChannelData(0);
    const right = audio.numberOfChannels > 1 ? audio.getChannelData(1) : left;
    return {
      rate: audio.sampleRate,
      left: new Float32Array(left),
      right: new Float32Array(right),
      frames: audio.length,
      channels: Math.min(2, audio.numberOfChannels),
    };
  }

  // --- ZIP: stored entries out, stored and deflated entries in ------------
  const CRC = (() => {
    const table = new Uint32Array(256);
    for (let n = 0; n < 256; n++) {
      let c = n;
      for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1;
      table[n] = c >>> 0;
    }
    return table;
  })();
  function crc32(bytes) {
    let c = 0xffffffff;
    for (let i = 0; i < bytes.length; i++) c = CRC[(c ^ bytes[i]) & 255] ^ (c >>> 8);
    return (c ^ 0xffffffff) >>> 0;
  }
  function zip(files) {
    const encoder = new TextEncoder();
    const parts = [],
      central = [];
    let offset = 0;
    for (const file of files) {
      const name = encoder.encode(file.name),
        data = file.data instanceof Uint8Array ? file.data : new Uint8Array(file.data);
      const crc = crc32(data);
      const local = new DataView(new ArrayBuffer(30));
      local.setUint32(0, 0x04034b50, true);
      local.setUint16(4, 20, true);
      local.setUint16(6, 0x0800, true);
      local.setUint32(14, crc, true);
      local.setUint32(18, data.length, true);
      local.setUint32(22, data.length, true);
      local.setUint16(26, name.length, true);
      parts.push(local.buffer, name, data);
      const entry = new DataView(new ArrayBuffer(46));
      entry.setUint32(0, 0x02014b50, true);
      entry.setUint16(4, 20, true);
      entry.setUint16(6, 20, true);
      entry.setUint16(8, 0x0800, true);
      entry.setUint32(16, crc, true);
      entry.setUint32(20, data.length, true);
      entry.setUint32(24, data.length, true);
      entry.setUint16(28, name.length, true);
      entry.setUint32(42, offset, true);
      central.push(entry.buffer, name);
      offset += 30 + name.length + data.length;
    }
    const size = central.reduce((n, p) => n + p.byteLength, 0);
    const end = new DataView(new ArrayBuffer(22));
    end.setUint32(0, 0x06054b50, true);
    end.setUint16(8, files.length, true);
    end.setUint16(10, files.length, true);
    end.setUint32(12, size, true);
    end.setUint32(16, offset, true);
    return new Blob([...parts, ...central, end.buffer], { type: "application/zip" });
  }
  async function inflate(data) {
    if (typeof DecompressionStream !== "function")
      throw new Error("This browser cannot unpack compressed .zip files");
    const stream = new Blob([data]).stream().pipeThrough(new DecompressionStream("deflate-raw"));
    return new Uint8Array(await new Response(stream).arrayBuffer());
  }
  async function unzip(buffer) {
    const view = new DataView(buffer),
      bytes = new Uint8Array(buffer);
    let end = -1;
    for (let i = buffer.byteLength - 22; i >= Math.max(0, buffer.byteLength - 65557); i--)
      if (view.getUint32(i, true) === 0x06054b50) {
        end = i;
        break;
      }
    if (end < 0) throw new Error("Not a .zip file");
    const count = view.getUint16(end + 10, true);
    let at = view.getUint32(end + 16, true);
    const decoder = new TextDecoder();
    const files = [];
    for (let i = 0; i < count; i++) {
      if (view.getUint32(at, true) !== 0x02014b50) throw new Error("Damaged .zip file");
      const method = view.getUint16(at + 10, true);
      const compressed = view.getUint32(at + 20, true);
      const nameLength = view.getUint16(at + 28, true),
        extra = view.getUint16(at + 30, true),
        comment = view.getUint16(at + 32, true);
      const local = view.getUint32(at + 42, true);
      const name = decoder.decode(bytes.subarray(at + 46, at + 46 + nameLength));
      at += 46 + nameLength + extra + comment;
      if (name.endsWith("/")) continue;
      const start = local + 30 + view.getUint16(local + 26, true) + view.getUint16(local + 28, true);
      const raw = bytes.subarray(start, start + compressed);
      if (method !== 0 && method !== 8) continue;
      files.push({ name, data: method === 0 ? raw.slice() : await inflate(raw) });
    }
    return files;
  }

  // --- IndexedDB -----------------------------------------------------------
  class Library {
    constructor() {
      this.ready = this.open();
      this.listeners = new Set();
      if (typeof BroadcastChannel !== "undefined") {
        this.channel = new BroadcastChannel(DB);
        this.channel.onmessage = () => this.changed(false);
      }
    }
    open() {
      return new Promise((resolve, reject) => {
        const request = indexedDB.open(DB, 1);
        request.onupgradeneeded = () => {
          const db = request.result;
          db.createObjectStore("samples", { keyPath: "id" });
          db.createObjectStore("packs", { keyPath: "id" });
        };
        request.onerror = () => reject(request.error);
        request.onblocked = () => reject(new Error("Close the other SAD PET tabs to open the sample library."));
        request.onsuccess = () => {
          this.db = request.result;
          this.db.onversionchange = () => this.db.close();
          resolve(this.db);
        };
      });
    }
    async run(store, mode, action) {
      await this.ready;
      return new Promise((resolve, reject) => {
        const transaction = this.db.transaction(store, mode);
        const request = action(transaction.objectStore(store));
        let result;
        if (request) request.onsuccess = () => (result = request.result);
        transaction.oncomplete = () => resolve(result);
        transaction.onerror = () => reject(transaction.error);
        transaction.onabort = () => reject(transaction.error || new Error("The library write was cancelled."));
      });
    }
    changed(broadcast = true) {
      if (broadcast) this.channel?.postMessage("changed");
      for (const listener of this.listeners) listener();
    }
    /** Every sample without its audio, newest first. */
    async samples() {
      const all = await this.run("samples", "readonly", (s) => s.getAll());
      return all
        .map(({ wav, ...info }) => info)
        .sort((a, b) => (b.createdAt || 0) - (a.createdAt || 0));
    }
    get(id) {
      return this.run("samples", "readonly", (s) => s.get(id));
    }
    async put(record) {
      const value = { createdAt: Date.now(), ...record, size: record.wav.size };
      await this.run("samples", "readwrite", (s) => s.put(value));
      this.changed();
      return value;
    }
    async rename(id, name) {
      const record = await this.get(id);
      if (!record) return;
      record.name = name;
      await this.run("samples", "readwrite", (s) => s.put(record));
      this.changed();
    }
    async remove(id) {
      await this.run("samples", "readwrite", (s) => s.delete(id));
      this.changed();
    }
    packs() {
      return this.run("packs", "readonly", (s) => s.getAll()).then((all) =>
        all.sort((a, b) => a.name.localeCompare(b.name)),
      );
    }
    pack(id) {
      return this.run("packs", "readonly", (s) => s.get(id));
    }
    async putPack(pack) {
      await this.run("packs", "readwrite", (s) => s.put({ createdAt: Date.now(), ...pack }));
      this.changed();
    }
    async removePack(id) {
      await this.run("packs", "readwrite", (s) => s.delete(id));
      this.changed();
    }
  }

  window.SadSamplerFiles = { parseWav, encodeWav, decodeAudio, zip, unzip, Library };
})();
