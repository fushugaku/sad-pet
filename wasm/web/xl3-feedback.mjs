// Novation XL3 programmer reference: DAW USB interface, RGB command 01 53,
// display commands 04/06, feature channel 7. Live LEDs require DAW Control;
// Custom Modes have a separate surface and retain their template colours.
const HEADER = [0xf0, 0, 0x20, 0x29, 2, 0x15];
const unit = v => Math.max(0, Math.min(1, Number(v) || 0));
const modePage = value => value >= 6 && value <= 9 ? value - 6 : value >= 18 && value <= 29 ? value - 14 : null;
const ascii = text => [...String(text).normalize("NFKD").replace(/[^\x20-\x7e]/g, " ").replace(/\s+/g, " ").trim().slice(0, 48)]
  .map(char => char.charCodeAt(0));
export function parameterColor(parameter = "", label = "") {
  const name = `${parameter} ${label}`.toLowerCase();
  if (/resonan|(?:^|[_ :])(?:res|q)(?:$|[_ :])/.test(name)) return [0, 25, 127];
  if (/cutoff|frequency|^filter\d?(?: |$)|\bcut\b/.test(name)) return [0, 127, 12];
  if (/pitch|detune|transpose|tuning|\btune\b|\bfine\b/.test(name)) return [127, 0, 0];
  if (/volume|gain|level|velocity|mix|send|sustain/.test(name)) return [127, 95, 0];
  if (/attack|decay|release|time|size|rate|tempo|bpm|delay/.test(name)) return [127, 40, 0];
  if (/lfo|env|mod|macro|depth/.test(name)) return [90, 0, 127];
  if (/pan|spread|stereo|position|cloud/.test(name)) return [0, 105, 127];
  if (/drive|distort|shap|reverb|chorus|flang|fx/.test(name)) return [0, 127, 100];
  return [85, 85, 85];
}
export function feedbackRgb(parameter, label, value) {
  const brightness = .08 + .92 * unit(value);
  return parameterColor(parameter, label).map(channel => Math.round(channel * brightness));
}
export const isLaunchControlPort = name => /launch\s*control\s*xl\b|\blcxl3\b/i.test(name || "");
const portName = name => (name || "").toLowerCase().replace(/\b(?:in|out)(?:put)?\b/g, "").replace(/\s+/g, " ").trim();
const deviceName = name => portName(name).replace(/\b\d*\s*(?:midi|daw)\b/g, "").replace(/\s+/g, " ").trim();
export function matchingOutput(input, outputs) {
  if (!input) return null;
  return outputs.find(p => p.state === "connected" && p.manufacturer === input.manufacturer &&
    portName(p.name) === portName(input.name)) || null;
}
export function xl3DawPorts(input, inputs, outputs) {
  if (!input) return {};
  const candidates = inputs.filter(p => p.state === "connected" && /daw/i.test(p.name) &&
    isLaunchControlPort(p.name) && p.manufacturer === input.manufacturer && deviceName(p.name) === deviceName(input.name));
  // Do not guess between indistinguishable units connected at the same time.
  const dawInput = candidates.length === 1 ? candidates[0] : null;
  return { input: dawInput, output: matchingOutput(dawInput, outputs) };
}

export class Xl3Feedback {
  constructor({ learn, page, error, transport = () => null }) {
    this.learn = learn; this.pageChanged = page; this.error = error;
    this.transport = transport;
    this.colors = new Map(); this.positions = new Map(); this.names = new Map();
    this.page = 0; this.hardwareMode = null; this.lastControl = null;
  }
  send(bytes, port = this.output) {
    if (!port) return;
    try { port.send(bytes); } catch (error) { this.error(`XL3 feedback: ${error.message}`); }
  }
  sysex(...bytes) { this.send([...HEADER, ...bytes, 0xf7]); }
  configure({ output, enabled, page = this.page } = {}) {
    const active = !!(enabled && output);
    if (this.output !== output || this.active !== active) {
      this.stop(); this.output = output; this.active = active;
      if (active) {
        this.send([0x9f, 12, 127]);
        this.send([0x9f, 11, 127]);
        this.hardwareMode = null;
        this.interval = setInterval(() => this.sync(), 50);
      }
    }
    this.selectPage(page);
  }
  configureControls() {
    // Selecting DAW Control resets row 1 to absolute on the device. Set the
    // relative format AFTER that command, including every hardware page change.
    for (const row of [69, 72, 73]) this.send([0xb6, row, 127]);
    this.send([0xb6, 70, 127]);
    this.send([0xb6, 71, 127]);
  }
  reassert() {
    if (!this.active) return;
    this.send([0x9f, 12, 127]); this.send([0x9f, 11, 127]);
    this.hardwareMode = null;
    this.selectPage(this.page);
  }
  selectPage(page) {
    if (!Number.isInteger(page) || page < 0 || page > 15) return;
    if (this.page !== page) {
      this.page = page; this.colors.clear(); this.positions.clear(); this.names.clear(); this.lastControl = null;
      if (this.active) this.sysex(4, 54, 0);
    }
    if (this.active && this.hardwareMode !== 2) {
      this.hardwareMode = 2;
      this.send([0xb6, 30, 2]);
      this.configureControls();
      this.colors.clear(); this.positions.clear(); this.names.clear();
      this.displaySignature = null; this.lastControl = null;
    }
    if (this.active) this.sync();
  }
  input(data) {
    if (data?.length !== 3 || data[0] !== 0xb6 || data[1] !== 30) return;
    const page = modePage(data[2]);
    if (page === null && ![1, 2].includes(data[2])) return;
    this.hardwareMode = data[2];
    if (this.active && data[2] === 2) {
      this.configureControls();
      this.colors.clear(); this.positions.clear(); this.names.clear();
      this.displaySignature = null;
    }
    // Mode + 1–16 selects the software page, then returns to the DAW surface.
    // This preserves the familiar hardware gesture and every saved mapping.
    if (page !== null) this.pageChanged(page);
    this.selectPage(page ?? this.page);
  }
  touch(controller) {
    if (controller < 5 || controller > 36) return;
    this.lastControl = controller;
    // Relative encoders extinguish their LED on movement. Refresh even when
    // the parameter is clamped at an endpoint and its value did not change.
    this.colors.delete(controller);
    this.displayDirty = true;
  }
  async sync() {
    if (!this.active || this.syncing) return;
    this.syncing = true;
    const page = this.page, output = this.output, track = this.learn()?.track?.() ?? 0;
    if (this.track !== track) {
      this.track = track; this.colors.clear(); this.positions.clear(); this.names.clear();
      this.lastControl = null; this.displaySignature = null;
      this.sysex(4, 54, 0);
    }
    try {
      const learner = this.learn();
      const transport = this.transport();
      const trackCount = learner?.tracks?.().length || 1;
      for (const [control, rgb] of [[116, [0, transport?.playing ? 127 : 12, 0]],
        [118, [transport?.recording ? 127 : 12, 0, 0]],
        [103, track > 0 ? [0, 80, 100] : [0, 0, 0]],
        [102, track < trackCount - 1 ? [0, 80, 100] : [0, 0, 0]]]) {
        const signature = rgb.join();
        if (this.colors.get(control) !== signature) {
          this.sysex(1, 83, control, ...rgb); this.colors.set(control, signature);
        }
      }
      await learner?.refreshFeedback?.();
      if (!this.active || this.page !== page || this.output !== output || (learner?.track?.() ?? 0) !== track) return;
      const controls = new Map((learner?.feedback?.() || []).filter(m => m.channel === page).map(m => [m.controller, m]));
      for (let control = 13; control <= 52; control++) {
        const cc = control <= 36 ? control : page === 15 ? control : control + 43;
        const binding = controls.get(cc);
        const rgb = binding?.available ? feedbackRgb(binding.parameter, binding.label, binding.unit) : [0, 0, 0];
        const signature = rgb.join();
        if (this.colors.get(control) !== signature) {
          this.sysex(1, 83, control, ...rgb); this.colors.set(control, signature);
        }
      }
      for (let control = 5; control <= 36; control++) {
        const binding = controls.get(control);
        if (!binding?.available) {
          if (this.names.get(control) !== null) {
            this.sysex(4, control, 0);
            this.sysex(4, control, 1); // No automatic touch/change overlays.
            this.names.set(control, null);
          }
          this.positions.delete(control);
          continue;
        }
        const value = Math.round(unit(binding.unit) * 127);
        if (this.positions.get(control) !== value) {
          this.send([0xbf, control, value]);
          this.positions.set(control, value);
        }
        const name = ascii(binding.label);
        if (this.names.get(control) !== name.join()) {
          this.sysex(4, control, 0);
          this.sysex(4, control, 1);
          this.sysex(6, control, 0, ...name);
          this.names.set(control, name.join());
        }
      }
      const selected = controls.get(this.lastControl);
      const fields = selected ? [selected.instrument || `Page ${page + 1}`, selected.label,
        selected.available ? selected.text || `${Math.round(unit(selected.unit) * 100)}%` : "Loading..."]
        : ["SAD PET", `Page ${page + 1} / Track ${track + 1}`, `${controls.size} assignments`];
      const signature = JSON.stringify([page, track, selected?.key, fields]);
      if (this.displayDirty || this.displaySignature !== signature) {
        // Target 53 is stationary. Temporary target 54 expires according to
        // the device's timeout and cannot hold the last parameter on screen.
        this.sysex(4, 53, 2);
        fields.forEach((text, field) => this.sysex(6, 53, field, ...ascii(text)));
        this.sysex(4, 53, 127);
        this.displaySignature = signature; this.displayDirty = false;
      }
    } catch (error) { this.error(`XL3 feedback: ${error.message}`); }
    finally { this.syncing = false; }
  }
  stop() {
    clearInterval(this.interval); this.interval = null;
    if (this.active) {
      this.sysex(4, 54, 0);
      this.sysex(4, 53, 0);
      this.send([0x9f, 12, 0]);
      this.send([0x9f, 11, 0]);
    }
    this.active = false; this.colors.clear(); this.positions.clear(); this.names.clear();
    this.displaySignature = null; this.hardwareMode = null;
  }
  dispose() { this.stop(); this.output = null; }
}
