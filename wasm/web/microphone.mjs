// Capture stays inside this AudioContext. There is no network audio endpoint.
export class Microphone {
  constructor(backend, media = navigator.mediaDevices) {
    this.backend = backend;
    this.media = media;
    this.generation = 0;
    this.active = false;
    this.pending = false;
    this.mode = "effects";
    this.gain = 1;
    this.device = "";
    this.name = "";
    this.devices = [];
  }
  snapshot() {
    return {
      active: this.active,
      pending: this.pending,
      mode: this.mode,
      gain: this.gain,
      device: this.device,
      name: this.name,
      devices: this.devices,
      error: this.error || null,
    };
  }
  release() {
    this.source?.disconnect();
    this.gainNode?.disconnect();
    for (const track of this.stream?.getTracks() || []) {
      track.onended = null;
      track.stop();
    }
    this.stream = this.source = this.gainNode = null;
    this.active = false;
  }
  async stop() {
    ++this.generation;
    this.pending = false;
    this.release();
    await this.backend.audio.call(
      "request",
      { playing: false, clear_hold: true },
      [],
      "/api/transport",
    );
    await this.backend.audio.call(
      "request",
      { active: false },
      [],
      "/api/input",
    );
    return this.snapshot();
  }
  async start(device = this.device) {
    if (!this.media?.getUserMedia)
      throw new Error(
        "Microphone input needs HTTPS or localhost and a browser with audio capture support.",
      );
    const generation = ++this.generation;
    this.pending = true;
    this.error = null;
    let stream;
    try {
      stream = await this.media.getUserMedia({
        audio: {
          ...(device ? { deviceId: { exact: device } } : {}),
          echoCancellation: false,
          noiseSuppression: false,
          autoGainControl: false,
          channelCount: { ideal: 2 },
        },
        video: false,
      });
      if (generation !== this.generation) {
        stream.getTracks().forEach((track) => track.stop());
        return this.snapshot();
      }
      this.release();
      this.stream = stream;
      const track = stream.getAudioTracks()[0];
      if (!track) throw new Error("No audio input was returned.");
      this.device = track.getSettings?.().deviceId || device;
      this.name = track.label || "Microphone";
      this.source = this.backend.context.createMediaStreamSource(stream);
      this.gainNode = this.backend.context.createGain();
      this.gainNode.gain.value = this.gain;
      await this.backend.audio.call(
        "request",
        { active: true, mode: this.mode },
        [],
        "/api/input",
      );
      if (generation !== this.generation) return this.snapshot();
      this.source.connect(this.gainNode).connect(this.backend.node);
      this.active = true;
      track.onended = () => {
        if (this.stream !== stream) return;
        this.error =
          "Microphone disconnected. Choose an input and connect again.";
        this.stop().catch(() => {});
      };
      await this.backend.audio.call(
        "request",
        { playing: true, midi_gate: false },
        [],
        "/api/transport",
      );
      try {
        this.devices = (await this.media.enumerateDevices())
          .filter((item) => item.kind === "audioinput")
          .map((item) => ({
            id: item.deviceId,
            name: item.label || "Audio input",
          }));
      } catch {
        /* Capture can work even when device enumeration is unavailable. */
      }
      return this.snapshot();
    } catch (error) {
      if (generation !== this.generation) return this.snapshot();
      if (stream && this.stream === stream) {
        this.release();
        await this.backend.audio
          .call("request", { active: false }, [], "/api/input")
          .catch(() => {});
      } else stream?.getTracks().forEach((track) => track.stop());
      const messages = {
        NotAllowedError:
          "Microphone access was denied. Allow the microphone for this site in browser settings, then try again.",
        NotFoundError:
          "No microphone was found. Connect an audio input and try again.",
        NotReadableError:
          "The microphone could not be opened. Check the system microphone permission and other audio applications.",
        OverconstrainedError:
          "This audio input is no longer available. Choose another microphone.",
      };
      this.error = messages[error.name] || error.message;
      throw new Error(this.error);
    } finally {
      if (generation === this.generation) this.pending = false;
    }
  }
  async configure(data) {
    if (data.gain !== undefined) {
      this.gain = Math.min(2, Math.max(0, Number(data.gain) || 0));
      if (this.gainNode)
        this.gainNode.gain.setTargetAtTime(
          this.gain,
          this.backend.context.currentTime,
          0.01,
        );
    }
    if (data.mode !== undefined && data.mode !== this.mode) {
      this.mode = data.mode === "grains" ? "grains" : "effects";
      if (this.active)
        await this.backend.audio.call(
          "request",
          { active: true, mode: this.mode },
          [],
          "/api/input",
        );
    }
    if (data.active === false) {
      this.error = null;
      return this.active || this.pending ? this.stop() : this.snapshot();
    }
    if (
      data.active === true &&
      (!this.active ||
        (data.device !== undefined && data.device !== this.device))
    )
      return this.start(data.device ?? this.device);
    return this.snapshot();
  }
}
