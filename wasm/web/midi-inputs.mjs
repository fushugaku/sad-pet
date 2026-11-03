import { LaunchControl, launchControlModel } from "./launch-control.mjs";
import { Xl3Feedback, isLaunchControlPort, matchingOutput, xl3DawPorts } from "./xl3-feedback.mjs";
import { Xl3Sequencer } from "./xl3-sequencer.mjs";

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
        enabled: saved.enabled !== false, mode: ["learn", "native", "daw", "sequencer"].includes(saved.mode) ? saved.mode : "mixer",
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
    this.highResolution = new Map();
    this.pageButtons = new Set();
    this.transportButtons = new Set();
    this.shiftHeld = false; this.shiftOffReply = false;
    this.transportQueue = Promise.resolve();
    this.sequencer = new Xl3Sequencer(() => this.ui()?.sequencer?.());
    this.feedback = new Xl3Feedback({ learn: this.learn,
      surface: () => this.settings.mode === "sequencer"
        ? { mode: 1, ...this.sequencer.feedback(this.shiftHeld) } : null,
      transport: () => this.ui()?.transportState?.(),
      page: page => {
        this.clearHighResolution();
        this.learn()?.selectPage?.(page, { preserveLearning: true });
        this.changed();
      },
      error: message => { if (this.error !== message) this.ui()?.notify(message); this.error = message; },
    });
  }
  save() {
    try { this.storage?.setItem(STORAGE, JSON.stringify(this.settings)); } catch { /* Optional. */ }
  }
  clearHighResolution() {
    for (const pair of this.highResolution.values()) clearTimeout(pair.timer);
    this.highResolution.clear();
  }
  async access({ sysex = false } = {}) {
    if (!this.navigator?.requestMIDIAccess)
      throw new Error(globalThis.isSecureContext === false
        ? "MIDI needs HTTPS or localhost" : "Web MIDI is unavailable in this browser. Use Chrome or Edge");
    if (this.accessPending) await this.accessPending;
    if (!this.backend.midiAccess || (sysex && !this.backend.midiAccess.sysexEnabled)) {
      this.accessPending = this.navigator.requestMIDIAccess({ sysex });
      try {
        const access = await this.accessPending;
        if (this.backend.midiAccess && this.backend.midiAccess !== access)
          this.backend.midiAccess.onstatechange = null;
        this.backend.midiAccess = access;
      } catch (error) {
        this.error = error.name === "NotAllowedError"
          ? "MIDI access was denied. Allow MIDI and system exclusive messages for XL3 lights and display" : error.message;
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
      const sysex = (await this.navigator.permissions.query({ name: "midi", sysex: true })).state === "granted";
      if (sysex || (await this.navigator.permissions.query({ name: "midi", sysex: false })).state === "granted")
        await this.access({ sysex });
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
      this.clearHighResolution();
      this.pageButtons.clear();
      this.resetDawButtons();
      this.surface.resetPickup();
      this.surface.buttonsDown.clear();
      this.surface.lights.clear();
      this.error = null;
    }
    let keyboard = this.inputs.find(p => p.id === this.keyboardId) || null;
    // The control surface's buttons are never musical notes.
    if (keyboard && isLaunchControlPort(keyboard.name)) { keyboard = null; this.keyboardId = null; }
    if (keyboard !== this.backend.selectedMidi) {
      this.backend.selectedMidi = keyboard;
      this.gateChange = this.backend.audio.call("request", { midi_gate: !!keyboard }, [], "/api/transport")
        .catch(error => { this.error = error.message; });
    }
    const outputs = [...(this.backend.midiAccess?.outputs.values() || [])];
    const output = matchingOutput(this.controller, outputs);
    this.surface.configure({ model: launchControlModel(this.controller?.name) || "xl",
      mode: this.settings.mode, output, page: this.learn()?.page?.() ?? 0 });
    const daw = this.surface.model === "xl3" && ["daw", "sequencer"].includes(this.settings.mode)
      ? xl3DawPorts(this.controller, this.inputs, outputs) : {};
    if (this.feedbackInput !== (daw.input || null)) this.resetDawButtons();
    this.feedbackInput = daw.input || null;
    this.feedbackOutput = daw.output || null;
    this.feedback.configure({ output: daw.output || null,
      enabled: !!this.backend.midiAccess?.sysexEnabled, page: this.learn()?.page?.() ?? 0 });
    const desired = new Set([keyboard, this.controller, this.feedbackInput].filter(Boolean));
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
    if (port === this.feedbackInput) { this.dawInput(event.data); return; }
    if (port === this.controller) {
      const learner = this.learn();
      if (this.settings.mode === "sequencer") return;
      if (this.settings.mode === "daw" && this.feedback.active) return;
      if (["native", "daw"].includes(this.settings.mode)) {
        if (this.surface.model === "xl3") this.nativeInput(event.data, learner);
        return;
      }
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
  dawInput(data) {
    if (data?.length !== 3 || !this.feedback.active) return;
    if (data[0] === 0xb6 && data[1] === 30) {
      if (this.feedback.modeReply(data)) return;
      if (data[2] === 1 || data[2] === 2 || data[2] >= 6 && data[2] <= 9 || data[2] >= 18 && data[2] <= 29) {
        this.settings.mode = data[2] === 1 ? "sequencer" : "daw";
        this.save();
      }
      this.resetDawButtons(); this.feedback.input(data); this.changed(); return;
    }
    const [status, cc, value] = data;
    if (value < 0 || value > 127) return;
    if (status === 0xb6 && cc === 63) {
      // XL3's built-in Shift previews controls and suppresses their CCs.
      // Keep the physical modifier here, but release firmware preview so
      // rotations still arrive. Feature controls acknowledge this release.
      if (!value && this.shiftOffReply) { this.shiftOffReply = false; return; }
      this.shiftHeld = !!value;
      if (value) {
        this.sequencer.shift();
        this.shiftOffReply = true;
        this.feedback.send([0xb6, 63, 0]);
      }
      if (this.settings.mode === "sequencer") this.feedback.sync();
      return;
    }
    if (status === 0xb0 && [116, 118].includes(cc)) {
      this.feedback.colors.delete(cc);
      if (!value) { this.transportButtons.delete(cc); return; }
      if (this.transportButtons.has(cc)) return;
      this.transportButtons.add(cc);
      const action = cc === 116 ? "play" : "record";
      // Read the current app state after the previous press has completed.
      this.transportQueue = this.transportQueue.then(() => this.ui()?.transport?.(action))
        .catch(error => this.ui()?.notify(error.message));
      return;
    }
    if (status === 0xbe && cc >= 5 && cc <= 36) {
      if (value) this.feedback.touch(cc);
      return;
    }
    if (this.settings.mode === "sequencer") {
      if (status === 0xb0 && cc >= 37 && cc <= 52) {
        this.sequencer.button(cc - 37, value, this.shiftHeld);
      } else if (status === 0xbf && cc >= 77 && cc <= 100) {
        this.feedback.touch(cc - 64);
        this.sequencer.turn(cc - 64, value);
      } else if (status === 0xbf && cc >= 5 && cc <= 11) {
        this.feedback.faderInput(cc, value);
        this.sequencer.fader(cc, value);
      } else if (status === 0xb0 && [103, 102, 106, 107].includes(cc)) {
        if (!value) this.pageButtons.delete(cc);
        else if (!this.pageButtons.has(cc)) {
          this.pageButtons.add(cc);
          if (cc === 103 || cc === 102) this.sequencer.navigate(cc === 103 ? -1 : 1);
          else this.navigateInstrument(cc === 106 ? -1 : 1);
        }
      }
      this.feedback.sync();
      return;
    }
    const learner = this.learn(), page = learner?.page?.() ?? 0;
    if (status === 0xb0 && [106, 107, 103, 102].includes(cc)) {
      if (!value) { this.pageButtons.delete(cc); return; }
      if (this.pageButtons.has(cc)) return;
      this.pageButtons.add(cc);
      if (cc === 103 || cc === 102) {
        const next = Math.max(0, Math.min(5, (learner?.track?.() ?? 0) + (cc === 103 ? -1 : 1)));
        learner?.selectTrack?.(next, { preserveLearning: true });
        this.feedback.sync(); this.changed();
        return;
      }
      const next = Math.max(0, Math.min(15, page + ([106, 103].includes(cc) ? -1 : 1)));
      learner?.selectPage?.(next, { preserveLearning: true });
      this.feedback.selectPage(next); this.changed();
      return;
    }
    if (status === 0xbf && cc >= 77 && cc <= 100) {
      const controller = cc - 64;
      this.feedback.touch(controller);
      learner?.input(page, controller, value, "relative", this.shiftHeld ? 10 : 1);
    } else if (status === 0xbf && cc >= 5 && cc <= 12) {
      this.feedback.faderInput(cc, value);
      learner?.input(page, cc, value);
    } else if (status === 0xb0 && cc >= 37 && cc <= 52) {
      learner?.input(page, page === 15 ? cc : cc + 43, value);
    }
  }
  resetDawButtons() {
    this.shiftHeld = false; this.shiftOffReply = false;
    this.transportButtons.clear();
    this.pageButtons.clear();
    this.sequencer.reset();
  }
  navigateInstrument(direction) {
    const context = this.ui()?.context(), parts = this.surface.parts();
    const index = parts.findIndex(t => t.id === context?.active), next = parts[index + direction];
    if (next) {
      this.sequencer.reset(true);
      this.surface.select(next);
    }
  }
  // SAD PET XL3 Custom Modes 1–15: relative encoders, 14-bit faders,
  // freely assignable buttons. Factory Mode 16 keeps its fixed 7-bit format.
  nativeInput(data, learner) {
    if (data?.length !== 3 || (data[0] & 0xf0) !== 0xb0 ||
        data.some(value => !Number.isInteger(value) || value < 0 || value > 255) ||
        data[1] > 119 || data[2] > 127) return;
    const channel = data[0] & 15, cc = data[1], value = data[2];
    const encoder = cc >= 13 && cc <= 36, fader = cc >= 5 && cc <= 12;
    const lsb = channel < 15 && cc >= 37 && cc <= 44;
    const button = channel < 15 ? cc >= 80 && cc <= 95 : cc >= 37 && cc <= 52;
    // Side/transport buttons use the global channel, not the custom mode's
    // channel. They must never choose an unrelated mapping page.
    if (!encoder && !fader && !lsb && !button) return;
    if (button && value === 0 && learner?.page?.() !== channel) return;
    if (learner?.page?.() !== channel) {
      learner?.selectPage?.(channel, { preserveLearning: true });
      this.changed();
    }
    this.feedback.selectPage(channel, true);
    this.feedback.touch(lsb ? cc - 32 : cc);
    if (channel === 15) { learner?.input(channel, cc, value); return; }
    if (encoder) { learner?.input(channel, cc, value, "relative"); return; }
    if (button) { learner?.input(channel, cc, value); return; }
    const controller = lsb ? cc - 32 : cc, key = `${channel}:${controller}`;
    const pair = this.highResolution.get(key) || { msb: null, lsb: null, dirty: 0, timer: null };
    this.highResolution.set(key, pair);
    pair[lsb ? "lsb" : "msb"] = value;
    pair.dirty |= lsb ? 2 : 1;
    if (pair.msb === null || pair.lsb === null) return;
    const apply = () => {
      clearTimeout(pair.timer);
      pair.timer = null;
      pair.dirty = 0;
      if (this.settings.mode === "native" && learner?.page?.() === channel)
        learner?.input(channel, controller, pair.msb * 128 + pair.lsb, "14bit");
    };
    // Accept either byte order and LSB-only changes. Wait briefly when only
    // one half changes, so an MSB crossing cannot emit a coarse intermediate.
    if (pair.dirty === 3) apply();
    else if (!pair.timer) pair.timer = setTimeout(apply, 4);
  }
  async keyboard(path, data) {
    if (path.searchParams.has("request")) await this.access();
    else this.refresh();
    if (data && "port" in data) {
      const port = data.port === null ? null : this.inputs[data.port];
      if (data.port !== null && (!port || isLaunchControlPort(port.name)))
        throw new Error("Choose a connected keyboard input");
      this.keyboardId = port?.id || null;
      this.refresh();
      await this.gateChange;
    }
    return {
      ports: this.inputs.flatMap((p, id) => isLaunchControlPort(p.name) ? [] : [{ id, name: p.name || "MIDI input" }]),
      connected: this.backend.selectedMidi?.name || null,
    };
  }
  async request(path, data) {
    const prepareLearn = path.searchParams.has("learn");
    let reassertFeedback = false;
    if (path.searchParams.has("request") || prepareLearn) await this.access({ sysex: true });
    if (prepareLearn) {
      const xl3 = [this.controller, ...this.candidates()].find(p => p && launchControlModel(p.name) === "xl3");
      if (xl3) {
        await this.surface.flush();
        // Learn is the user's request for live parameter feedback. Reassert
        // DAW mode even if another client previously disabled the hardware.
        reassertFeedback = !!this.feedback.active;
        this.clearHighResolution(); this.pageButtons.clear(); this.resetDawButtons();
        Object.assign(this.settings, { enabled: true, mode: "daw", id: xl3.id, name: xl3.name });
        this.learn()?.configureXL3?.();
        this.error = null; this.save();
      }
    }
    if (data) {
      if ("mode" in data && !["mixer", "learn", "native", "daw", "sequencer"].includes(data.mode)) throw new Error("Unknown controller mode");
      if ("page" in data && (!Number.isInteger(data.page) || data.page < 0 || data.page >= 16))
        throw new Error("Choose a MIDI mapping page from 1 to 16");
      if ("mappingTrack" in data && (!Number.isInteger(data.mappingTrack) || data.mappingTrack < 0 || data.mappingTrack >= 6))
        throw new Error("Choose a mapping track from 1 to 6");
      if ("device" in data && data.device !== "auto" && data.device !== null &&
          !this.candidates().some(p => p.id === data.device)) throw new Error("That controller is no longer connected");
      if (data.mode === "sequencer") await this.access({ sysex: true });
      await this.surface.flush();
      if ("device" in data) {
        const selected = this.candidates().find(p => p.id === data.device);
        this.settings.enabled = data.device !== null;
        this.settings.id = selected?.id || null;
        this.settings.name = selected?.name || null;
      }
      if ("mode" in data) {
        if (this.settings.mode !== data.mode) {
          this.clearHighResolution(); this.pageButtons.clear(); this.resetDawButtons();
        }
        this.settings.mode = data.mode;
        if (["native", "daw"].includes(data.mode)) this.learn()?.configureXL3?.();
      }
      if ("page" in data) {
        this.clearHighResolution();
        this.learn()?.selectPage?.(data.page);
      }
      if ("mappingTrack" in data) {
        this.clearHighResolution();
        this.learn()?.selectTrack?.(data.mappingTrack);
      }
      this.error = null;
      this.save();
    }
    this.refresh();
    if (reassertFeedback) this.feedback.reassert();
    return this.status();
  }
  status() {
    const learner = this.learn();
    this.surface.mappingPage = learner?.page?.() ?? 0;
    return {
      supported: !!this.navigator?.requestMIDIAccess, access: !!this.backend.midiAccess,
      device: !this.settings.enabled ? null : this.settings.id || "auto",
      ports: this.candidates().map(p => ({ id: p.id, name: p.name })),
      xl3Available: this.candidates().some(p => launchControlModel(p.name) === "xl3"),
      connected: this.controller?.name || null, mode: this.settings.mode,
      page: this.surface.mappingPage, pages: learner?.pages?.() || [],
      mappingTrack: learner?.track?.() ?? 0, mappingTracks: learner?.tracks?.() || [],
      feedback: this.feedback.active || (!!this.surface.output && this.surface.model === "xl"),
      feedbackPermission: !!this.backend.midiAccess?.sysexEnabled,
      feedbackAvailable: this.surface.model === "xl3" && ["daw", "sequencer"].includes(this.settings.mode) && !!this.feedbackInput && !!this.feedbackOutput,
      sequencer: this.settings.mode === "sequencer" ? this.sequencer.feedback(this.shiftHeld).fields : null,
      error: this.error || null,
      ...this.surface.status(),
    };
  }
  dispose() {
    this.feedback.dispose();
    this.clearHighResolution();
    this.pageButtons.clear();
    this.resetDawButtons();
    this.surface.dispose();
    for (const port of this.bound) { port.onmidimessage = null; port.close?.().catch(() => {}); }
    this.bound.clear();
    if (this.backend.midiAccess) this.backend.midiAccess.onstatechange = null;
  }
}
