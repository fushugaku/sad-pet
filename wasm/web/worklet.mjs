import { Core } from "./core.mjs";

class SadPetProcessor extends AudioWorkletProcessor {
  constructor(options) {
    super();
    this.core = new Core(options.processorOptions.module, sampleRate);
    this.telemetryFrames = 0;
    this.stateStamp = 0;
    this.recording = false;
    this.recordedFrames = 0;
    this.recordBuffer = null;
    this.recordLength = 0;
    this.snapshotPool = Array.from({ length: 3 }, () => new Uint8Array(131072));
    this.recordPool = Array.from(
      { length: 4 },
      () => new Float32Array(16384 * 2),
    );
    this.limit = Math.min(
      sampleRate * 30 * 60,
      Math.floor((512 * 1024 * 1024 - 4096) / 8),
    );
    this.port.onmessage = ({ data: message }) => {
      const { id, type, data } = message;
      if (type === "prepare-idle") {
        while (this.preparingTimbre) this.advanceTimbre();
        return;
      }
      if (type === "snapshot-now") {
        this.snapshot();
        return;
      }
      if (type === "snapshot-return") {
        this.core.wasm.sad_snapshot_ack(
          message.sequence,
          message.success ? 1 : 0,
        );
        const capacity = this.core.wasm.sad_snapshot_capacity();
        this.snapshotPool.push(
          data.byteLength >= capacity
            ? new Uint8Array(data)
            : new Uint8Array(capacity),
        );
        return;
      }
      if (type === "record-return") {
        if (this.recordPool.length < 8)
          this.recordPool.push(new Float32Array(data));
        return;
      }
      try {
        let result;
        if (type === "scene-load" || (type === "request" && message.path === "/api/timbres/reset"))
          this.cancelTimbre("Timbre preparation cancelled by a scene change.");
        if (type === "add-timbre") {
          if (this.preparingTimbre) throw new Error("A timbre is already being prepared.");
          this.core.json("sad_timbre_prepare_begin", data);
          this.preparingTimbre = { id };
          if (data.idle) while (this.preparingTimbre) this.advanceTimbre();
          return;
        }
        if (type === "scene-pack") result = this.core.call("sad_scene_pack");
        else if (type === "scene-load")
          result = this.core.json("sad_scene_load", data);
        else if (type === "load") {
          if (this.sourceTransfer)
            this.port.postMessage({
              type: "source-ready",
              key: this.sourceTransfer.key,
              generation: this.sourceTransfer.generation,
              status: 2,
            });
          this.sourceTransfer = null;
          result = this.core.json("sad_load", data.prepared || data);
          this.core.wasm.sad_source_generation(
            data.generation ?? (this.sourceGeneration || 0) + 1,
          );
          this.sourceGeneration =
            data.generation ?? (this.sourceGeneration || 0) + 1;
        } else if (type === "source-load") {
          const previous =
            data.timbre === undefined
              ? null
              : this.core.wasm.sad_timbre_target(data.timbre);
          result =
            previous === 0xffffffff
              ? { started: false }
              : this.core.json("sad_source_begin", {
                  recipe: data.recipe,
                  header: data.header,
                  generation: data.generation,
                });
          if (previous !== null) this.core.wasm.sad_timbre_target(previous);
          if (result.started) {
            if (this.sourceTransfer)
              this.port.postMessage({
                type: "source-ready",
                key: this.sourceTransfer.key,
                generation: this.sourceTransfer.generation,
                status: 2,
              });
            this.sourceTransfer = {
              bytes: data.prepared
                ? new Uint8Array(data.prepared, data.offset)
                : new Uint8Array(0),
              offset: 0,
              key: data.key,
              timbre: data.timbre,
              generation: data.generation,
            };
          }
        } else if (type === "vocal-update") {
          this.vocalUpdate(data);
          result = true;
        } else if (type === "capture-input")
          result = this.core.call("sad_input_wav");
        else if (type === "record-start") {
          if (this.recording) throw new Error("Recording is already running.");
          this.recordedFrames = 0;
          this.recordSource = data?.source === "input" ? "input" : "output";
          this.recordBuffer ||=
            this.recordPool.pop() || new Float32Array(16384 * 2);
          this.recordLength = 0;
          this.recording = true;
          result = { frames: 0 };
        } else if (type === "record-stop") {
          this.flushRecording();
          this.recording = false;
          result = { frames: this.recordedFrames };
        } else result = this.core.request(message.path, data);
        if (type !== "capture-input" && (type !== "request" || data != null))
          this.stateStamp++;
        const capacity = this.core.wasm.sad_snapshot_capacity();
        this.snapshotPool = this.snapshotPool.map((buffer) =>
          buffer.length >= capacity ? buffer : new Uint8Array(capacity),
        );
        this.port.postMessage(
          { id, result, stamp: this.stateStamp },
          result instanceof ArrayBuffer ? [result] : [],
        );
        this.vocalJobs();
        if (
          ![
            "/api/note",
            "/api/midi-bytes",
            "/api/parameters",
            "/api/sequencer/edit",
            "/api/drums/edit",
            "/api/drums/trigger",
          ].includes(message.path)
        )
          this.snapshot();
      } catch (error) {
        this.port.postMessage({ id, error: error.message });
      }
    };
    this.port.postMessage({ type: "ready" });
  }
  /** Scores that changed go to the vocal render worker. */
  vocalJobs() {
    const jobs = this.core.json("sad_vocal_jobs");
    if (jobs.length) this.port.postMessage({ type: "vocal-jobs", jobs });
  }
  /** Keeps the planned clips of each Vocal part and adds the new ones. */
  vocalUpdate(updates) {
    const wasm = this.core.wasm;
    for (const update of updates) {
      const count = update.plan.length;
      const plan = new Float64Array(wasm.memory.buffer, wasm.sad_vocal_plan(count), count * 3);
      update.plan.forEach((entry, i) => plan.set(entry, i * 3));
      wasm.sad_vocal_plan_commit(update.id, count);
      for (const clip of update.clips) {
        const samples = new Float32Array(clip.data);
        const pointer = wasm.sad_vocal_clip_begin(update.id, clip.key, clip.start, samples.length);
        new Float32Array(wasm.memory.buffer, pointer, samples.length).set(samples);
        wasm.sad_vocal_clip_commit();
      }
    }
  }
  advanceTimbre() {
    const pending = this.preparingTimbre;
    if (!pending) return;
    try {
      const result = this.core.json("sad_timbre_prepare_step");
      if (!result.done) return;
      this.preparingTimbre = null;
      this.stateStamp++;
      const capacity = this.core.wasm.sad_snapshot_capacity();
      this.snapshotPool = this.snapshotPool.map(buffer => buffer.length >= capacity ? buffer : new Uint8Array(capacity));
      this.port.postMessage({ id: pending.id, result: result.context, stamp: this.stateStamp });
      this.vocalJobs();
      this.snapshot();
    } catch (error) {
      this.preparingTimbre = null;
      this.core.wasm.sad_timbre_prepare_cancel();
      this.port.postMessage({ id: pending.id, error: error.message });
    }
  }
  cancelTimbre(reason) {
    if (!this.preparingTimbre) return;
    const { id } = this.preparingTimbre;
    this.preparingTimbre = null;
    this.core.wasm.sad_timbre_prepare_cancel();
    this.port.postMessage({ id, error: reason });
  }
  snapshot() {
    const buffer = this.snapshotPool.pop();
    // UI backpressure must never delay audio/MIDI or discard unreported peaks.
    if (!buffer) return;
    let length;
    try {
      length = this.core.snapshot(buffer);
    } catch {
      this.snapshotPool.push(buffer);
      return;
    }
    if (!length) {
      this.snapshotPool.push(buffer);
      return;
    }
    this.port.postMessage(
      {
        type: "snapshot",
        stamp: this.stateStamp,
        buffer: buffer.buffer,
        length,
        sequence: this.core.wasm.sad_snapshot_sequence(),
        recordedFrames: this.recordedFrames,
      },
      [buffer.buffer],
    );
  }
  flushRecording() {
    if (!this.recordLength) return;
    const buffer = this.recordBuffer;
    this.port.postMessage(
      {
        type: "record-chunk",
        buffer: buffer.buffer,
        length: this.recordLength,
      },
      [buffer.buffer],
    );
    // If the utility worker is busy, keep every recorded frame. Normally its
    // returned buffers avoid allocations here; starvation has a lossless fallback.
    this.recordBuffer = this.recordPool.pop() || new Float32Array(16384 * 2);
    this.recordLength = 0;
  }
  process(inputs, outputs) {
    try {
      const channels = outputs[0];
      if (!channels?.length) return true;
      const count = channels[0].length;
      if (this.sourceTransfer) {
        const stage = this.sourceTransfer;
        const previous =
          stage.timbre === undefined
            ? null
            : this.core.wasm.sad_timbre_target(stage.timbre);
        const status =
          previous === 0xffffffff
            ? 2
            : this.core.sourceChunk(stage.bytes, stage.offset);
        if (previous !== null) this.core.wasm.sad_timbre_target(previous);
        stage.offset += this.core.sourceCopied;
        if (status !== 0) {
          this.port.postMessage({
            type: "source-ready",
            key: stage.key,
            generation: stage.generation,
            status,
          });
          this.sourceTransfer = null;
        }
      }
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
          if (this.recordSource === "input") {
            const incoming = inputs[0] || [];
            for (let i = 0; i < length; i++) {
              const sample = offset + i,
                channel = sample % 2;
              const value =
                (incoming[channel] || incoming[0])?.[Math.floor(sample / 2)] ??
                0;
              this.recordBuffer[this.recordLength + i] = Number.isFinite(value)
                ? value
                : 0;
            }
          } else
            for (let i = 0; i < length; i++)
              this.recordBuffer[this.recordLength + i] = frames[offset + i];
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
      // Source FX preparation also consumes these views while the UI is hidden.
      this.telemetryFrames += count;
      if (this.telemetryFrames >= sampleRate / 20) {
        this.telemetryFrames = 0;
        this.snapshot();
      }
      this.advanceTimbre();
    } catch (error) {
      this.port.postMessage({ type: "fatal", error: error.message });
      return false;
    }
    return true;
  }
}
registerProcessor("sad-pet", SadPetProcessor);
