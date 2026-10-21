import { Microphone } from "./microphone.mjs";

class Rpc {
  constructor(port) {
    this.port = port;
    this.next = 0;
    this.pending = new Map();
    port.onmessage = ({ data }) => {
      if (data.id !== undefined) {
        this.stamp = Math.max(this.stamp || 0, data.stamp || 0);
        const pending = this.pending.get(data.id);
        if (!pending) return;
        this.pending.delete(data.id);
        clearTimeout(pending.timer);
        data.error
          ? pending.reject(new Error(data.error))
          : pending.resolve(data.result);
      } else this.onmessage?.(data);
    };
    port.onerror = (event) =>
      this.fail(new Error(event.message || "Audio worker stopped."));
  }
  fail(error) {
    for (const pending of this.pending.values()) {
      clearTimeout(pending.timer);
      pending.reject(error);
    }
    this.pending.clear();
    this.onerror?.(error);
  }
  call(type, data, transfer = [], path) {
    return new Promise((resolve, reject) => {
      const id = ++this.next;
      const timer = setTimeout(() => {
        this.pending.delete(id);
        reject(
          new Error(
            "The audio worker did not respond. Reload the page to restart it.",
          ),
        );
      }, 180000);
      this.pending.set(id, { resolve, reject, timer });
      this.port.postMessage({ id, type, path, data }, transfer);
    });
  }
}

export class BrowserBackend {
  constructor() {
    this.instanceId = crypto.randomUUID();
    this.viewStamp = 0;
    this.stateWaiters = new Set();
    this.recording = {
      active: false,
      id: null,
      seconds: 0,
      available: false,
      error: null,
    };
    this.midiPorts = [];
    this.status = "Loading WebAssembly…";
    this.ready = this.initialize();
  }
  async initialize() {
    this.sourceGeneration = 1;
    if (!window.AudioContext || !window.AudioWorkletNode)
      throw new Error(
        "This browser needs Web Audio with AudioWorklet support.",
      );
    this.context = new AudioContext({ latencyHint: "interactive" });
    this.rate = this.context.sampleRate;
    this.context.onstatechange = () => this.updateStatus();
    this.worker = new Rpc(
      new Worker(new URL("./worker.mjs", import.meta.url), { type: "module" }),
    );
    this.worker.onerror = (error) => {
      this.fatal = error.message;
      this.updateStatus();
    };
    this.worker.onmessage = (message) => this.workerMessage(message);
    this.telemetry = new Rpc(
      new Worker(new URL("./worker.mjs", import.meta.url), { type: "module" }),
    );
    this.recorder = new Rpc(
      new Worker(new URL("./worker.mjs", import.meta.url), { type: "module" }),
    );
    this.telemetry.onmessage = (message) => this.workerMessage(message);
    this.recorder.onmessage = (message) => this.workerMessage(message);
    const response = await fetch(new URL("./sad_pet.wasm", import.meta.url));
    if (!response.ok)
      throw new Error("Cannot download the WebAssembly engine.");
    const module = await WebAssembly.compile(await response.arrayBuffer());
    await this.worker.call("init", { module, rate: this.rate });
    await this.telemetry.call("init", { module, rate: this.rate });
    const prepared = await this.worker.call("startup", {
      generation: this.sourceGeneration,
    });
    await this.context.audioWorklet.addModule(
      new URL("./worklet.mjs", import.meta.url),
    );
    this.node = new AudioWorkletNode(this.context, "sad-pet", {
      numberOfInputs: 1,
      numberOfOutputs: 1,
      outputChannelCount: [2],
      channelCount: 2,
      channelCountMode: "explicit",
      processorOptions: { module },
    });
    this.audio = new Rpc(this.node.port);
    this.audio.onmessage = (message) => this.audioMessage(message);
    this.microphone = new Microphone(this);
    window.addEventListener("pagehide", () => {
      this.microphone.release();
      this.stopInput().catch(() => {});
    });
    this.node.onprocessorerror = () => {
      this.microphone.release();
      this.fatal = "The audio engine stopped. Reload the page to restart it.";
      this.audio.fail(new Error(this.fatal));
      this.updateStatus();
    };
    this.monitor = this.context.createGain();
    this.node.connect(this.monitor).connect(this.context.destination);
    await this.audio.call("scene-load", prepared, [prepared]);
    this.state = await this.audio.call("request", null, [], "/api/state");
    this.viewStamp = this.audio.stamp || 0;
    const quit = document.getElementById("quit");
    quit.textContent = "Stop audio";
    document.querySelector("h1 span").textContent = "WEBASSEMBLY";
    document.getElementById("midi-refresh").title = "Connect a MIDI keyboard";
    document.getElementById("cpu").hidden = true;
    document.getElementById("browser-input").hidden = false;
    if (!navigator.requestMIDIAccess)
      document.getElementById("midi-refresh").disabled = true;
    this.updateStatus();
  }
  updateStatus() {
    this.status = this.fatal
      ? "Audio stopped"
      : this.context?.state === "running"
        ? `${(this.rate / 1000).toFixed(1)} kHz · WebAssembly`
        : "Click Play or a key to enable audio";
  }
  resume() {
    if (this.context?.state === "suspended")
      this.context.resume().catch((error) => {
        this.fatal = error.message;
      });
  }
  updateMonitor() {
    this.monitor?.gain.setTargetAtTime(
      this.microphone.active && !this.microphone.monitor ? 0 : 1,
      this.context.currentTime,
      0.01,
    );
  }
  async stopInput() {
    if (this.stoppingInput) return this.stoppingInput;
    this.stoppingInput = (async () => {
      // Finish a take before changing its source, so File and device disconnect
      // cannot silently leave an input recording running into silence.
      if (this.recording.active && this.recording.microphone)
        await this.finishRecording();
      const result = await this.microphone.stop();
      this.updateMonitor();
      return result;
    })();
    try {
      return await this.stoppingInput;
    } finally {
      this.stoppingInput = null;
    }
  }
  audioMessage(message) {
    if (message.type === "source-ready") {
      if (message.generation !== this.sourceGeneration) return;
      if (this.sourceUploadingKey === message.key)
        this.sourceUploadingKey = null;
      if (message.status === 1) this.sourcePublishedKey = message.key;
      else if (message.status === 3) {
        this.sourceFailedKey = message.key;
        this.sourceError = "Could not load Source FX";
      } else if (this.sourceRecipeKey === message.key)
        this.sourceRecipeKey = null;
    } else if (message.type === "snapshot") {
      this.telemetry.port.postMessage(message, [message.buffer]);
      if (this.recording.active)
        this.recording.seconds = message.recordedFrames / this.rate;
    } else if (message.type === "record-chunk") {
      this.recorder.port.postMessage(
        { type: "record-chunk", data: message.buffer, length: message.length },
        [message.buffer],
      );
    } else if (message.type === "record-limit") {
      this.finishRecording(
        "Recording reached the take size limit and was saved.",
      ).catch((error) => {
        this.recording.error = error.message;
      });
    } else if (message.type === "fatal") {
      this.microphone.release();
      this.fatal = message.error;
      this.audio.fail(new Error(message.error));
      this.updateStatus();
    }
  }
  workerMessage(message) {
    if (message.type === "snapshot") {
      if (message.state && (message.stamp || 0) >= (this.viewStamp || 0)) {
        this.state = message.state;
        this.viewStamp = message.stamp || 0;
        this.scheduleSourceEffects(message.state);
        for (const waiter of this.stateWaiters || []) {
          if (this.viewStamp >= waiter.stamp) {
            clearTimeout(waiter.timer);
            this.stateWaiters.delete(waiter);
            waiter.resolve();
          }
        }
      }
      this.audio.port.postMessage(
        {
          type: "snapshot-return",
          data: message.buffer,
          sequence: message.sequence,
          success: !message.error,
        },
        [message.buffer],
      );
    } else if (message.type === "record-return") {
      this.audio.port.postMessage(
        { type: "record-return", data: message.buffer },
        [message.buffer],
      );
    } else if (message.type === "fatal") {
      this.fatal = message.error;
      this.updateStatus();
    }
  }
  scheduleSourceEffects(state) {
    if (state?.input?.active || this.loadingSource) return;
    const p = state?.preset;
    if (!p) return;
    const slot = p.source_color_chain?.[0];
    const recipe = state.source_fx?.recipe || {
      region: state.region,
      slot: slot?.enabled && slot.mix > 0 ? slot : null,
      bpm: p.params.bpm,
      root: p.params.sample_root_note ?? 60,
    };
    const timbre = state.timbres?.active ?? 0;
    const key = JSON.stringify([timbre, this.sourceGeneration, recipe]);
    if (key === this.sourceFailedKey) {
      state.source_fx = {
        ...state.source_fx,
        pending: false,
        error: this.sourceError,
      };
      return;
    }
    if (
      key === this.sourceRecipeKey &&
      (!state.source_fx?.pending ||
        this.preparingSource ||
        this.sourceUploadingKey === key)
    )
      return;
    this.sourceRecipeKey = key;
    this.sourceFailedKey = null;
    this.queuedSource = {
      key,
      timbre,
      generation: this.sourceGeneration,
      recipe,
    };
    this.prepareSourceEffects();
  }
  async freshState() {
    const stamp = this.audio.stamp || 0;
    if ((this.viewStamp || 0) >= stamp) return;
    if (this.context.state !== "running") {
      this.state = await this.audio.call("request", null, [], "/api/state");
      this.viewStamp = this.audio.stamp || 0;
      return;
    }
    // Do not delay knob writes or audio. Only a state reader waits for a view
    // that contains its acknowledged edits, instead of undoing them with an old
    // decoded packet. The bounded transfer pool still provides backpressure.
    await new Promise((resolve, reject) => {
      const waiter = { stamp, resolve, timer: null };
      this.stateWaiters ||= new Set();
      waiter.timer = setTimeout(() => {
        this.stateWaiters.delete(waiter);
        reject(
          new Error(
            "Audio views did not update. Reload the page to reconnect.",
          ),
        );
      }, 5000);
      this.stateWaiters.add(waiter);
      this.audio.port.postMessage({ type: "snapshot-now" });
    });
  }
  async prepareSourceEffects() {
    if (this.preparingSource) return;
    this.preparingSource = true;
    try {
      while (this.queuedSource) {
        const job = this.queuedSource;
        this.queuedSource = null;
        try {
          const result = await this.worker.call("source-fx", job);
          if (
            !result.stale &&
            job.key === this.sourceRecipeKey &&
            job.generation === this.sourceGeneration &&
            !this.microphone.active &&
            !this.loadingSource
          ) {
            const started = await this.audio.call(
              "source-load",
              {
                ...result,
                key: job.key,
                timbre: job.timbre,
                generation: job.generation,
              },
              result.prepared ? [result.prepared] : [],
            );
            if (started.started) this.sourceUploadingKey = job.key;
            else if (this.sourceRecipeKey === job.key)
              this.sourceRecipeKey = null;
          }
        } catch (error) {
          this.sourceFailedKey = job.key;
          this.sourceError = error.message;
        }
      }
    } finally {
      this.preparingSource = false;
    }
  }
  snapshot() {
    return {
      ...this.state,
      instance_id: this.instanceId,
      audio_enabled: this.context.state === "running" && !this.fatal,
      audio_error: this.fatal || null,
      recording: { ...this.recording },
      input: {
        ...this.state?.input,
        ...(this.state?.preset?.drums ? {} : this.microphone.snapshot()),
        // A release is asynchronous: keep the live source visible until the
        // worklet confirms it has restored the file, including for Library.
        active: this.microphone.active || !!this.state?.input?.active,
      },
    };
  }
  async finishRecording(error = null) {
    if (this.finishing) return this.finishing;
    this.finishing = (async () => {
      const result = await this.audio.call("record-stop");
      const saved = await this.recorder.call("record-stop");
      if (saved.frames !== result.frames)
        throw new Error(
          "Recording was interrupted before all audio could be saved.",
        );
      this.recording = {
        ...this.recording,
        active: false,
        available: true,
        seconds: saved.frames / this.rate,
        error,
      };
      return { ...this.recording };
    })();
    try {
      return await this.finishing;
    } finally {
      this.finishing = null;
    }
  }
  async midi(path, data) {
    if (
      path.searchParams.has("request") &&
      navigator.requestMIDIAccess &&
      !this.midiAccess
    ) {
      this.midiAccess = await navigator.requestMIDIAccess({ sysex: false });
      this.midiAccess.onstatechange = () => {
        if (this.selectedMidi?.state === "disconnected") {
          this.selectedMidi.onmidimessage = null;
          this.selectedMidi = null;
          this.audio
            .call("request", { midi_gate: false }, [], "/api/transport")
            .catch(() => {});
        }
      };
    }
    if (data) {
      if (this.selectedMidi) this.selectedMidi.onmidimessage = null;
      this.selectedMidi = data.port === null ? null : this.midiPorts[data.port];
      if (this.selectedMidi) {
        this.selectedMidi.onmidimessage = (event) => {
          this.resume();
          this.audio
            .call("request", [...event.data], [], "/api/midi-bytes")
            .catch(() => {});
        };
      }
      await this.audio.call(
        "request",
        { midi_gate: !!this.selectedMidi },
        [],
        "/api/transport",
      );
    } else
      this.midiPorts = [...(this.midiAccess?.inputs.values() || [])].filter(
        (p) => p.state === "connected",
      );
    return {
      ports: this.midiPorts.map((p, id) => ({
        id,
        name: p.name || "MIDI input",
      })),
      connected: this.selectedMidi?.name || null,
    };
  }
  async fetch(path, options = {}) {
    try {
      await this.ready;
      if (this.fatal) throw new Error(this.fatal);
      const url = new URL(path, location.href),
        route = url.pathname;
      const binary = route === "/api/wav";
      const data =
        options.body === undefined || binary ? null : JSON.parse(options.body);
      let result,
        mime = "application/json";
      if (route === "/api/state") {
        await this.freshState();
        result = this.snapshot();
      } else if (route === "/api/scene" || (route === "/api/startup" && data)) {
        if (data) {
          await this.sourceLoadTail;
          const prepared =
            route === "/api/startup"
              ? await this.worker.call("startup", {
                  generation: this.sourceGeneration + 1,
                })
              : await this.worker.call("scene-prepare", data);
          result = await this.audio.call("scene-load", prepared, [prepared]);
          this.sourceRecipeKey = null;
          this.state = await this.audio.call("request", null, [], "/api/state");
          this.viewStamp = this.audio.stamp || 0;
          this.sourceGeneration = this.state.source_generation ?? 0;
          this.queuedSource = null;
          await this.worker.call(
            "request",
            this.state.timbres,
            [],
            "/api/timbres/context",
          );
        } else {
          const packed = await this.audio.call("scene-pack");
          result = await this.worker.call("scene-document", packed, [packed]);
        }
      } else if (route.startsWith("/api/timbres")) {
        await this.sourceLoadTail;
        result = await this.audio.call("request", data, [], route);
        if (route === "/api/timbres/update") {
          this.state.timbres = result;
        } else {
          await this.worker.call("request", result, [], "/api/timbres/context");
          this.queuedSource = null;
          this.sourceRecipeKey = null;
          this.state = await this.audio.call("request", null, [], "/api/state");
          this.sourceGeneration =
            this.state.source_generation ?? this.sourceGeneration;
        }
        this.viewStamp = this.audio.stamp || 0;
      } else if (route === "/api/input")
        result = data
          ? data.active === false
            ? await this.stopInput()
            : await this.microphone.configure(data)
          : this.microphone.snapshot();
      else if (route === "/api/input.wav") {
        result = await this.audio.call("capture-input");
        mime = "audio/wav";
      } else if (route === "/api/midi") result = await this.midi(url, data);
      else if (route === "/api/wav") {
        // Keep utility sample state, worklet state and the generation in one
        // order, including two file selections made before either load returns.
        const previousLoad = this.sourceLoadTail;
        let finishLoad;
        const thisLoad = new Promise((resolve) => {
          finishLoad = resolve;
        });
        this.sourceLoadTail = thisLoad;
        this.loadingSource = true;
        this.queuedSource = null;
        await previousLoad;
        try {
          const wav =
            options.body instanceof Blob
              ? await options.body.arrayBuffer()
              : options.body;
          if (wav.byteLength > 128 * 1024 * 1024)
            throw new Error("File exceeds 128 MB");
          const headers = new Headers(options.headers);
          const generation = this.sourceGeneration + 1;
          const prepared = await this.worker.call(
            "prepare",
            {
              wav,
              name: decodeURIComponent(
                headers.get("X-File-Name") || "Sample.wav",
              ),
              libraryId: headers.get("X-Library-Id"),
              generation,
            },
            [wav],
          );
          if (this.microphone.active || this.microphone.pending)
            await this.stopInput();
          await this.audio.call(
            "load",
            { prepared: prepared.prepared, generation },
            [prepared.prepared],
          );
          this.sourceGeneration = generation;
          this.sourceRecipeKey = null;
          this.sourceUploadingKey = null;
          this.sourceFailedKey = null;
          result = prepared.info;
        } finally {
          finishLoad();
          if (this.sourceLoadTail === thisLoad) this.loadingSource = false;
        }
      } else if (route === "/api/sample.wav") {
        result = await this.worker.call("sample");
        mime = "audio/wav";
      } else if (route === "/api/trim") {
        if (this.microphone.active || this.state?.input?.active)
          throw new Error("Load a recording or file to trim it.");
        result = await this.worker.call("trim", data);
        mime = "audio/wav";
      } else if (/^\/api\/samples\/[^/]+\.wav$/.test(route)) {
        const catalog = await this.worker.call(
          "request",
          null,
          [],
          "/api/samples",
        );
        const id = route.slice("/api/samples/".length, -4);
        const index = catalog.findIndex((t) => t.id === id);
        if (index < 0) throw new Error("Unknown track");
        result = await this.worker.call("factory", index);
        mime = "audio/wav";
      } else if (route === "/api/render") {
        if (this.microphone.active)
          throw new Error(
            "Use Record to save the live output, or switch to File before rendering.",
          );
        const packed = await this.audio.call("scene-pack");
        const scene = await this.worker.call("scene-document", packed, [
          packed,
        ]);
        result = await this.worker.call("scene-render", {
          seconds: data.seconds,
          scene,
        });
        mime = "audio/wav";
      } else if (route === "/api/recording") {
        if (data.active) {
          if (this.recording.active)
            throw new Error("Recording is already running.");
          const id = crypto.randomUUID();
          const source = data.source === "input" ? "input" : "output";
          if (source === "input" && !this.microphone.active)
            throw new Error(
              "Connect the microphone before recording its input.",
            );
          await this.recorder.call("record-start", { id, rate: this.rate });
          await this.audio.call("record-start", { source });
          this.recording = {
            active: true,
            id,
            seconds: 0,
            available: false,
            error: null,
            source,
            microphone: this.microphone.active,
          };
          result = { ...this.recording };
        } else result = await this.finishRecording();
      } else if (
        route === "/api/recording.wav" ||
        route === "/api/recording.mp3"
      ) {
        result = await this.recorder.call(
          route.endsWith(".mp3") ? "record-mp3" : "record-wav",
          url.searchParams.get("id"),
        );
        mime = route.endsWith(".mp3") ? "audio/mpeg" : "audio/wav";
      } else if (route === "/api/quit") {
        if (this.microphone.active || this.microphone.pending)
          await this.stopInput();
        if (this.recording.active) await this.finishRecording();
        await this.audio.call(
          "request",
          { playing: false, clear_hold: true, midi_gate: false },
          [],
          "/api/transport",
        );
        await this.context.suspend();
        result = { ok: true };
      } else if (
        [
          "/api/presets",
          "/api/samples",
          "/api/color-defaults",
          "/api/sequencer/catalog",
          "/api/drums/catalog",
        ].includes(route)
      ) {
        result = await this.worker.call("request", data, [], route);
      } else result = await this.audio.call("request", data, [], route);
      return new Response(
        mime === "application/json" ? JSON.stringify(result) : result,
        { headers: { "Content-Type": mime } },
      );
    } catch (error) {
      return new Response(JSON.stringify({ error: error.message }), {
        status: 400,
        headers: { "Content-Type": "application/json" },
      });
    }
  }
}
