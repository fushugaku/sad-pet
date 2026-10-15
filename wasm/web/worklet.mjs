import { Core } from "./core.mjs";

class SadPetProcessor extends AudioWorkletProcessor {
  constructor(options) {
    super();
    this.core = new Core(options.processorOptions.module, sampleRate);
    this.telemetryFrames = 0;
    this.recording = false;
    this.recordedFrames = 0;
    this.recordBuffer = null;
    this.recordLength = 0;
    this.limit = Math.min(
      sampleRate * 30 * 60,
      Math.floor((512 * 1024 * 1024 - 4096) / 8),
    );
    this.port.onmessage = ({ data: message }) => {
      const { id, type, data } = message;
      try {
        let result;
        if (type === "load") result = this.core.json("sad_load", data);
        else if (type === "capture-input")
          result = this.core.call("sad_input_wav");
        else if (type === "record-start") {
          if (this.recording) throw new Error("Recording is already running.");
          this.recordedFrames = 0;
          this.recordBuffer = new Float32Array(16384 * 2);
          this.recordLength = 0;
          this.recording = true;
          result = { frames: 0 };
        } else if (type === "record-stop") {
          this.flushRecording();
          this.recording = false;
          result = { frames: this.recordedFrames };
        } else result = this.core.request(message.path, data);
        this.port.postMessage(
          { id, result },
          result instanceof ArrayBuffer ? [result] : [],
        );
        this.snapshot();
      } catch (error) {
        this.port.postMessage({ id, error: error.message });
      }
    };
    this.port.postMessage({ type: "ready" });
  }
  snapshot() {
    const state = this.core.request("/api/state");
    this.port.postMessage({
      type: "state",
      state,
      recordedFrames: this.recordedFrames,
    });
  }
  flushRecording() {
    if (!this.recordLength) return;
    const buffer =
      this.recordLength === this.recordBuffer.length
        ? this.recordBuffer
        : this.recordBuffer.slice(0, this.recordLength);
    this.port.postMessage({ type: "record-chunk", buffer: buffer.buffer }, [
      buffer.buffer,
    ]);
    this.recordBuffer = new Float32Array(16384 * 2);
    this.recordLength = 0;
  }
  process(inputs, outputs) {
    try {
      const channels = outputs[0];
      if (!channels?.length) return true;
      const count = channels[0].length;
      const frames = this.core.process(count, inputs[0]);
      for (let c = 0; c < channels.length; c++)
        for (let i = 0; i < count; i++)
          channels[c][i] = frames[i * 2 + Math.min(c, 1)];
      if (this.recording) {
        const n = Math.min(count, this.limit - this.recordedFrames);
        let offset = 0;
        while (offset < n * 2) {
          const length = Math.min(
            n * 2 - offset,
            this.recordBuffer.length - this.recordLength,
          );
          this.recordBuffer.set(
            frames.subarray(offset, offset + length),
            this.recordLength,
          );
          this.recordLength += length;
          offset += length;
          if (this.recordLength === this.recordBuffer.length)
            this.flushRecording();
        }
        this.recordedFrames += n;
        if (this.recordedFrames === this.limit) this.flushRecording();
        if (this.recordedFrames === this.limit) {
          this.recording = false;
          this.port.postMessage({
            type: "record-limit",
            frames: this.recordedFrames,
          });
        }
      }
      this.telemetryFrames += count;
      if (this.telemetryFrames >= sampleRate / 20) {
        this.telemetryFrames = 0;
        this.snapshot();
      }
    } catch (error) {
      this.port.postMessage({ type: "fatal", error: error.message });
      return false;
    }
    return true;
  }
}
registerProcessor("sad-pet", SadPetProcessor);
