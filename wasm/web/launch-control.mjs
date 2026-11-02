// Launch Control XL Mk1/Mk2: Factory 1 (channel 9).
// Launch Control XL 3: the uneditable Custom Mode 16 (channel 16).
// No template uploads or SysEx permission are needed.
// Legacy control numbers: Ardour's Novation_LaunchControl_XL.map; LED format:
// Novation's Launch Control XL Programmer's Reference, pages 4 and 8.
// XL3 numbers: Novation's Launch Control XL 3 appendix, Default Mode (16).
const LEGACY = {
  channel: 8, rows: [13, 29, 49], fader: 77,
  focus: [41, 42, 43, 44, 57, 58, 59, 60],
  buttons: [73, 74, 75, 76, 89, 90, 91, 92],
};
const XL3 = {
  channel: 15, rows: [13, 21, 29], fader: 5,
  focus: [37, 38, 39, 40, 41, 42, 43, 44],
  buttons: [45, 46, 47, 48, 49, 50, 51, 52],
};
const EXCLUDED = new Set(["empty", "control", "bus", "master", "pad_fx"]);
export function launchControlModel(name = "") {
  if (/HUI|DAW/i.test(name) || !/launch\s*control\s*xl\b|\blcxl3\b/i.test(name)) return null;
  return /\b(?:mk\s*)?3\b|lcxl3/i.test(name) ? "xl3" : "xl";
}

export class LaunchControl {
  constructor(hooks) {
    this.hooks = hooks;
    this.bank = this.sendBank = 0;
    this.action = "muted";
    this.mode = "mixer";
    this.mappingPage = 0;
    this.model = "xl";
    this.pickups = new Map();
    this.pending = new Map();
    this.optimistic = new Map();
    this.lights = new Map();
    this.buttonsDown = new Set();
  }
  configure({ model = this.model, mode = this.mode, output = this.output, page = this.mappingPage } = {}) {
    if (model !== this.model || mode !== this.mode || output !== this.output) {
      this.resetPickup();
      this.buttonsDown.clear();
      this.lights.clear();
    }
    this.model = model;
    this.mode = mode;
    this.output = output;
    this.mappingPage = page;
    this.sync();
  }
  resetPickup() {
    this.pickups.clear();
    this.pickup = "";
  }
  parts() {
    return (this.hooks.context()?.timbres || []).filter(p => !EXCLUDED.has(p.kind)).map(p =>
      Object.assign(p, this.optimistic.get(p.id)));
  }
  buses() {
    return (this.hooks.context()?.timbres || [])
      .filter(p => p.kind === "bus" && p.bus > 0).sort((a, b) => a.bus - b.bus);
  }
  strips() { return this.parts().slice(this.bank * 8, this.bank * 8 + 8); }
  sends() { return this.buses().slice(this.sendBank * 3, this.sendBank * 3 + 3); }
  // A knob must meet or cross the current value before it takes over. Also
  // re-arm it after a mouse edit or scene recall, even when IDs are unchanged.
  take(key, value, target, label) {
    let p = this.pickups.get(key);
    if (!p) this.pickups.set(key, p = { previous: null, active: false });
    const tolerance = 1.5 / 127;
    if (p.active && Math.abs(target - p.last) > tolerance) p.active = false;
    if (!p.active) {
      p.active = Math.abs(value - target) <= tolerance ||
        (p.previous !== null && (p.previous - target) * (value - target) <= 0);
    }
    p.previous = value;
    if (!p.active) {
      this.pickup = `${label}: move ${value < target ? "up" : "down"} to ${Math.round(target * 100)}%`;
      return false;
    }
    p.last = value;
    this.pickup = "";
    return true;
  }
  edit(part, data) {
    // Coalesce hardware bursts; the normal editor's timbre queue handles
    // flushing preset edits and changing the selected timbre safely.
    Object.assign(part, data);
    this.optimistic.set(part.id, { ...this.optimistic.get(part.id), ...data });
    this.pending.set(part.id, { ...this.pending.get(part.id), ...data, id: part.id });
    if (!this.timer && !this.writing)
      this.timer = setTimeout(() => { this.timer = null; this.flush(); }, 16);
  }
  flush() {
    if (this.timer) clearTimeout(this.timer);
    this.timer = null;
    if (this.writing) return this.writing;
    this.writing = (async () => {
      while (this.pending.size) {
        const edits = [...this.pending.values()];
        this.pending.clear();
        for (const data of edits) {
          // A removed track must never be redirected to another track.
          if (!this.parts().some(p => p.id === data.id)) { this.optimistic.delete(data.id); continue; }
          await this.hooks.change("/api/timbres/update", data, true);
          // A reply may replace the UI's whole timbre context. Preserve any
          // newer hardware edits until their own reply arrives, especially
          // when several rows share the same sends array.
          const desired = this.optimistic.get(data.id);
          if (desired) {
            for (const key of Object.keys(data)) if (desired[key] === data[key]) delete desired[key];
            if (!Object.keys(desired).length) this.optimistic.delete(data.id);
          }
        }
      }
    })().catch(error => {
      this.pending.clear(); this.optimistic.clear();
      this.hooks.error(error.message);
    }).finally(() => { this.writing = null; });
    return this.writing;
  }
  select(part) {
    if (!part) return;
    const id = part.id;
    this.selections = (this.selections || Promise.resolve()).then(async () => {
      await this.flush();
      if (this.parts().some(p => p.id === id))
        await this.hooks.change("/api/timbres/select", { id });
    }).catch(error => this.hooks.error(error.message));
  }
  // In custom mode the sixteen channel buttons choose pages, and arrows
  // step between them. Consume releases/repeats before MIDI Learn sees CCs.
  pageInput(data) {
    if (this.mode !== "learn" || data?.length !== 3) return null;
    const [status, number, value] = data, profile = this.model === "xl3" ? XL3 : LEGACY;
    if ((status & 15) !== profile.channel) return null;
    const cc = (status & 0xf0) === 0xb0, type = status & 0xf0;
    const buttons = this.model === "xl3" ? cc : type === 0x90 || type === 0x80;
    const focus = buttons ? profile.focus.indexOf(number) : -1;
    const lower = buttons ? profile.buttons.indexOf(number) : -1;
    const arrow = cc && (number === 106 || number === 107);
    if (focus < 0 && lower < 0 && !arrow) return null;
    const key = `${cc ? "cc" : "note"}:${number}`, previous = this.buttonsDown.has(key);
    const down = type !== 0x80 && value > 0;
    down ? this.buttonsDown.add(key) : this.buttonsDown.delete(key);
    if (!down || previous) return -1;
    return focus >= 0 ? focus : lower >= 0 ? lower + 8
      : Math.max(0, Math.min(15, this.mappingPage + (number === 106 ? -1 : 1)));
  }
  input(data) {
    if (this.mode !== "mixer" || data?.length !== 3) return;
    const [status, number, value] = data, profile = this.model === "xl3" ? XL3 : LEGACY;
    if ((status & 15) !== profile.channel || number > 127 || value > 127) return;
    this.sync();
    const type = status & 0xf0, cc = type === 0xb0;
    const note = type === 0x90 || type === 0x80;
    if (!cc && !note) return;
    const down = type !== 0x80 && value > 0;
    const buttonKey = `${cc ? "cc" : "note"}:${number}`;
    // Buttons fire on a rising edge only, including devices that repeat 127.
    const press = () => {
      const previous = this.buttonsDown.has(buttonKey);
      down ? this.buttonsDown.add(buttonKey) : this.buttonsDown.delete(buttonKey);
      return down && !previous;
    };
    const buttons = this.model === "xl3" ? cc : note;
    if (buttons) {
      const focus = profile.focus.indexOf(number), control = profile.buttons.indexOf(number);
      if (focus >= 0) { if (press()) this.select(this.strips()[focus]); return; }
      if (control >= 0) {
        if (!press()) return;
        const part = this.strips()[control];
        if (part) this.edit(part, { [this.action]: !part[this.action] });
        this.sync();
        return;
      }
      if (this.model === "xl" && (number === 106 || number === 107)) {
        if (press()) { this.action = number === 106 ? "muted" : "solo"; this.sync(); }
        return;
      }
      if (this.model === "xl3" && (number === 65 || number === 66)) {
        if (press()) { this.action = number === 66 ? "muted" : "solo"; this.sync(); }
        return;
      }
    }
    if (!cc) return;
    if (number >= 104 && number <= 107 && this.model === "xl") {
      if (!press()) return;
      if (number >= 106) this.bank += number === 106 ? -1 : 1;
      else this.sendBank += number === 104 ? -1 : 1;
      this.resetPickup();
      this.sync();
      return;
    }
    if (this.model === "xl3" && (number === 106 || number === 107)) {
      if (press()) { this.bank += number === 106 ? -1 : 1; this.resetPickup(); this.sync(); }
      return;
    }
    const unit = value / 127, strips = this.strips();
    if (number >= profile.fader && number < profile.fader + 8) {
      const column = number - profile.fader, part = strips[column];
      if (part && this.take(number, unit, part.gain / 2, `Fader ${column + 1}`))
        this.edit(part, { gain: unit * 2 });
      return;
    }
    const row = profile.rows.findIndex(start => number >= start && number < start + 8);
    if (row < 0) return;
    const column = number - profile.rows[row], part = strips[column], bus = this.sends()[row];
    if (!part || !bus) return;
    const sends = [...(part.sends || Array(8).fill(0))];
    if (this.take(number, unit, sends[bus.bus - 1] / 2, `Knob ${row + 1}.${column + 1}`)) {
      sends[bus.bus - 1] = unit * 2;
      this.edit(part, { sends });
    }
  }
  light(type, number, value) {
    const key = `${type}:${number}`;
    if (!this.output || this.lights.get(key) === value) return;
    try {
      this.output.send([type | LEGACY.channel, number, value]);
      this.lights.set(key, value);
    } catch (error) {
      this.output = null;
      this.hooks.error(`Launch Control LEDs: ${error.message}`);
    }
  }
  sync() {
    const parts = this.parts(), buses = this.buses();
    this.bank = Math.max(0, Math.min(Math.max(0, Math.ceil(parts.length / 8) - 1), this.bank));
    this.sendBank = Math.max(0, Math.min(Math.max(0, Math.ceil(buses.length / 3) - 1), this.sendBank));
    const signature = JSON.stringify([parts.map(p => p.id), buses.map(p => p.bus), this.bank, this.sendBank]);
    if (signature !== this.signature) { this.signature = signature; this.resetPickup(); }
    // XL3 uses a different RGB palette; leave its custom-mode colours intact.
    if (this.model !== "xl") return;
    if (this.mode === "learn") {
      for (let i = 0; i < 8; i++) {
        this.light(0x90, LEGACY.focus[i], this.mappingPage === i ? 60 : 12);
        this.light(0x90, LEGACY.buttons[i], this.mappingPage === i + 8 ? 60 : 12);
      }
      for (const note of [106, 107]) this.light(0x90, note, 0);
      this.light(0xb0, 104, 0); this.light(0xb0, 105, 0);
      this.light(0xb0, 106, this.mappingPage > 0 ? 127 : 0);
      this.light(0xb0, 107, this.mappingPage < 15 ? 127 : 0);
      return;
    }
    const strips = this.strips(), sends = this.sends(), active = this.hooks.context()?.active;
    for (let i = 0; i < 8; i++) {
      const part = strips[i];
      this.light(0x90, LEGACY.focus[i], !part ? 12 : part.id === active ? 60 : 28);
      this.light(0x90, LEGACY.buttons[i], !part ? 12 : part[this.action] ? (this.action === "muted" ? 15 : 63) : 28);
      for (let row = 0; row < 3; row++) this.light(0x90, 13 + i * 16 + row, part && sends[row] ? 28 : 12);
    }
    this.light(0x90, 106, this.action === "muted" ? 127 : 0);
    this.light(0x90, 107, this.action === "solo" ? 127 : 0);
    this.light(0xb0, 104, this.sendBank > 0 ? 127 : 0);
    this.light(0xb0, 105, (this.sendBank + 1) * 3 < buses.length ? 127 : 0);
    this.light(0xb0, 106, this.bank > 0 ? 127 : 0);
    this.light(0xb0, 107, (this.bank + 1) * 8 < parts.length ? 127 : 0);
  }
  status() {
    this.sync();
    return {
      bank: this.bank, banks: Math.max(1, Math.ceil(this.parts().length / 8)),
      tracks: this.strips().map(p => ({ id: p.id, name: p.name })),
      sends: this.sends().map(p => p.name), action: this.action, pickup: this.pickup,
      template: this.model === "xl3" ? "Custom Mode 16 · channel 16" : "Factory 1 · channel 9",
    };
  }
  dispose() {
    if (this.timer) clearTimeout(this.timer);
    this.timer = null;
    this.pending.clear();
    this.optimistic.clear();
    this.output = null;
    this.buttonsDown.clear();
    this.resetPickup();
  }
}
