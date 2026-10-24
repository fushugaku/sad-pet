// The Sampler's recorder, an AudioWorklet of its own: it never shares the
// engine's processor or its WASM memory.
/** Records its input for the Sampler: from the first frame above `threshold`
 * (with 10 ms before it), up to `limit` frames, posting chunks and levels. */
class CaptureProcessor extends AudioWorkletProcessor {
  constructor(options) {
    super();
    const { threshold = 0, limit = sampleRate * 60 } = options.processorOptions || {};
    this.threshold = threshold;
    this.limit = limit;
    this.armed = threshold > 0;
    this.frames = 0;
    this.chunk = new Float32Array(8192 * 2);
    this.length = 0;
    this.preroll = new Float32Array(Math.ceil(sampleRate * 0.01) * 2);
    this.prerollAt = 0;
    this.prerollFull = false;
    this.peak = 0;
    this.meter = 0;
    this.running = true;
    this.port.onmessage = ({ data }) => {
      if (data === "stop") {
        this.flush();
        this.running = false;
        this.port.postMessage({ type: "stopped", frames: this.frames });
      }
    };
  }
  flush() {
    if (!this.length) return;
    this.port.postMessage({ type: "chunk", data: this.chunk.slice(0, this.length) });
    this.length = 0;
  }
  push(left, right) {
    this.chunk[this.length++] = left;
    this.chunk[this.length++] = right;
    this.frames++;
    if (this.length === this.chunk.length) this.flush();
  }
  process(inputs) {
    if (!this.running) return false;
    const input = inputs[0] || [];
    const left = input[0], right = input[1] || input[0];
    const count = left?.length || 128;
    for (let i = 0; i < count; i++) {
      const l = left?.[i] ?? 0, r = right?.[i] ?? 0;
      const level = Math.max(Math.abs(l), Math.abs(r));
      this.peak = Math.max(this.peak, level);
      if (this.armed) {
        this.preroll[this.prerollAt++] = l;
        this.preroll[this.prerollAt++] = r;
        if (this.prerollAt >= this.preroll.length) {
          this.prerollAt = 0;
          this.prerollFull = true;
        }
        if (level < this.threshold) continue;
        this.armed = false;
        const size = this.preroll.length;
        const start = this.prerollFull ? this.prerollAt : 0;
        const filled = this.prerollFull ? size : this.prerollAt;
        for (let k = 0; k < filled; k += 2) {
          const at = (start + k) % size;
          this.push(this.preroll[at], this.preroll[at + 1]);
        }
        continue;
      }
      if (this.frames >= this.limit) {
        this.flush();
        this.running = false;
        this.port.postMessage({ type: "stopped", frames: this.frames, limit: true });
        return false;
      }
      this.push(l, r);
    }
    this.meter += count;
    if (this.meter >= sampleRate / 20) {
      this.meter = 0;
      this.port.postMessage({ type: "level", peak: this.peak, armed: this.armed, frames: this.frames });
      this.peak = 0;
    }
    return true;
  }
}
registerProcessor("sad-capture", CaptureProcessor);
