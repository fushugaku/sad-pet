import { LaunchControl, launchControlModel } from "./launch-control.mjs";

const STORAGE = "sad-pet-launch-control";
/** One owner for Web MIDI ports: keyboard and control surface can coexist. */
export class MidiInputs {
  constructor(backend, { navigator = globalThis.navigator, storage = globalThis.localStorage,
    ui = () => globalThis.window?.SadController, learn = () => globalThis.window?.SadMidiLearn,
    changed = () => globalThis.window?.dispatchEvent(new Event("sad-midi-change")) } = {}) {
    this.backend = backend;
    this.navigator = navigator;
    this.storage = storage;
    this.ui = ui;
    this.learn = learn;
    this.changed = changed;
    this.inputs = [];
    this.bound = new Set();
    this.settings = { enabled: true, mode: "mixer", id: null, name: null };
    try {
      const saved = JSON.parse(storage?.getItem(STORAGE) || "null");
      if (saved) this.settings = {
        enabled: saved.enabled !== false, mode: saved.mode === "learn" ? "learn" : "mixer",
        id: typeof saved.id === "string" ? saved.id : null,
        name: typeof saved.name === "string" ? saved.name : null,
      };
    } catch { /* Device settings still work when storage is unavailable. */ }
    this.surface = new LaunchControl({
      context: () => this.ui()?.context() ?? backend.state?.timbres,
      change: (...args) => {
        const editor = this.ui();
        if (!editor?.change) throw new Error("The instrument is still loading");
        return editor.change(...args);
      },
      error: message => { this.error = message; this.ui()?.notify(message); },
    });
  }
  save() {
    try { this.storage?.setItem(STORAGE, JSON.stringify(this.settings)); } catch { /* Optional. */ }
  }
  async access() {
    if (!this.navigator?.requestMIDIAccess)
      throw new Error(globalThis.isSecureContext === false
        ? "MIDI needs HTTPS or localhost" : "Web MIDI is unavailable in this browser. Use Chrome or Edge");
    if (!this.backend.midiAccess) {
      this.accessPending ||= this.navigator.requestMIDIAccess({ sysex: false });
      try {
        this.backend.midiAccess = await this.accessPending;
      } catch (error) {
        this.error = error.name === "NotAllowedError"
          ? "MIDI access was denied. Allow MIDI for this site in your browser settings" : error.message;
        if (error.name === "NotAllowedError") throw new Error(this.error);
        throw error;
      } finally { this.accessPending = null; }
      this.error = null;
    }
    this.backend.midiAccess.onstatechange = () => this.refresh();
    this.refresh();
  }
  async restore() {
    // Permission checks do not show a prompt; a fresh visit waits for Connect.
    if (!this.navigator?.permissions?.query || !this.navigator.requestMIDIAccess) return;
    try {
      if ((await this.navigator.permissions.query({ name: "midi", sysex: false })).state === "granted")
        await this.access();
    } catch { /* Browsers without a MIDI permission query still have Connect. */ }
  }
  candidates() { return this.inputs.filter(p => launchControlModel(p.name)); }
  refresh() {
    const previous = this.controller;
    this.inputs = [...(this.backend.midiAccess?.inputs.values() || [])].filter(p => p.state === "connected");
    this.backend.midiPorts = this.inputs;
    const candidates = this.candidates();
    this.controller = !this.settings.enabled ? null :
      this.settings.id ? candidates.find(p => p.id === this.settings.id) ||
        candidates.find(p => p.name === this.settings.name) || null : candidates[0] || null;
    if (this.controller && this.settings.id && this.settings.id !== this.controller.id) {
      this.settings.id = this.controller.id;
      this.save();
    }
    if (previous !== this.controller) {
      this.surface.resetPickup();
      this.surface.buttonsDown.clear();
      this.surface.lights.clear();
      this.error = null;
    }
    let keyboard = this.inputs.find(p => p.id === this.keyboardId) || null;
    // The control surface's buttons are never musical notes.
    if (keyboard && launchControlModel(keyboard.name)) { keyboard = null; this.keyboardId = null; }
    if (keyboard !== this.backend.selectedMidi) {
      this.backend.selectedMidi = keyboard;
      this.gateChange = this.backend.audio.call("request", { midi_gate: !!keyboard }, [], "/api/transport")
        .catch(error => { this.error = error.message; });
    }
    const outputs = [...(this.backend.midiAccess?.outputs.values() || [])];
    const output = this.controller ? outputs.find(p => p.state === "connected" &&
      p.name === this.controller.name && p.manufacturer === this.controller.manufacturer) || null : null;
    this.surface.configure({ model: launchControlModel(this.controller?.name) || "xl",
      mode: this.settings.mode, output, page: this.learn()?.page?.() ?? 0 });
    const desired = new Set([keyboard, this.controller].filter(Boolean));
    for (const port of this.bound) if (!desired.has(port)) {
      port.onmidimessage = null;
      port.close?.().catch(() => {});
    }
    for (const port of desired) if (!this.bound.has(port)) {
      port.onmidimessage = event => this.message(port, event);
    }
    this.bound = desired;
    const signature = JSON.stringify([this.inputs.map(p => [p.id, p.name]), keyboard?.id,
      this.controller?.id, this.settings.mode, this.settings.enabled]);
    if (signature !== this.signature) { this.signature = signature; this.changed(); }
  }
  message(port, event) {
    if (port === this.controller) {
      const learner = this.learn();
      this.surface.mappingPage = learner?.page?.() ?? 0;
      const page = this.surface.pageInput(event.data);
      if (page !== null) {
        if (page >= 0) {
          learner?.selectPage?.(page);
          this.surface.mappingPage = learner?.page?.() ?? page;
          this.surface.sync();
          this.changed();
        }
        return;
      }
      // MIDI Learn takes precedence over the ready-made mixer. Nothing from
      // this port reaches the synth or the keyboard's external clock.
      if (!learner?.bytes(event.data)) this.surface.input(event.data);
      return;
    }
    if (port !== this.backend.selectedMidi) return;
    if (this.backend.midiClock.input(event)) return;
    this.learn()?.bytes(event.data);
    const bytes = [...event.data];
    this.backend.resume().then(() => this.backend.audio.call("request", bytes, [], "/api/midi-bytes"))
      .catch(() => {});
  }
  async keyboard(path, data) {
    if (path.searchParams.has("request")) await this.access();
    else this.refresh();
    if (data && "port" in data) {
      const port = data.port === null ? null : this.inputs[data.port];
      if (data.port !== null && (!port || launchControlModel(port.name)))
        throw new Error("Choose a connected keyboard input");
      this.keyboardId = port?.id || null;
      this.refresh();
      await this.gateChange;
    }
    return {
      ports: this.inputs.flatMap((p, id) => launchControlModel(p.name) ? [] : [{ id, name: p.name || "MIDI input" }]),
      connected: this.backend.selectedMidi?.name || null,
    };
  }
  async request(path, data) {
    if (path.searchParams.has("request")) await this.access();
    if (data) {
      if ("mode" in data && !["mixer", "learn"].includes(data.mode)) throw new Error("Unknown controller mode");
      if ("page" in data && (!Number.isInteger(data.page) || data.page < 0 || data.page >= 16))
        throw new Error("Choose a MIDI mapping page from 1 to 16");
      if ("device" in data && data.device !== "auto" && data.device !== null &&
          !this.candidates().some(p => p.id === data.device)) throw new Error("That controller is no longer connected");
      await this.surface.flush();
      if ("device" in data) {
        const selected = this.candidates().find(p => p.id === data.device);
        this.settings.enabled = data.device !== null;
        this.settings.id = selected?.id || null;
        this.settings.name = selected?.name || null;
      }
      if ("mode" in data) this.settings.mode = data.mode;
      if ("page" in data) this.learn()?.selectPage?.(data.page);
      this.error = null;
      this.save();
    }
    this.refresh();
    return this.status();
  }
  status() {
    const learner = this.learn();
    this.surface.mappingPage = learner?.page?.() ?? 0;
    return {
      supported: !!this.navigator?.requestMIDIAccess, access: !!this.backend.midiAccess,
      device: !this.settings.enabled ? null : this.settings.id || "auto",
      ports: this.candidates().map(p => ({ id: p.id, name: p.name })),
      connected: this.controller?.name || null, mode: this.settings.mode,
      page: this.surface.mappingPage, pages: learner?.pages?.() || [],
      feedback: !!this.surface.output && this.surface.model === "xl", error: this.error || null,
      ...this.surface.status(),
    };
  }
  dispose() {
    this.surface.dispose();
    for (const port of this.bound) { port.onmidimessage = null; port.close?.().catch(() => {}); }
    this.bound.clear();
    if (this.backend.midiAccess) this.backend.midiAccess.onstatechange = null;
  }
}
