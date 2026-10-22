"use strict";
(() => {
  const el = (tag, className, text) => {
    const node = document.createElement(tag);
    if (className) node.className = className;
    if (text !== undefined) node.textContent = text;
    return node;
  };
  const button = (text, label, action) => {
    const node = el("button", "", text);
    if (label) node.setAttribute("aria-label", label);
    node.onclick = action;
    return node;
  };
  class Timbres {
    constructor(root, hooks) {
      this.root = root;
      this.hooks = hooks;
      root.innerHTML = `<div class="timbre-top"><div class="timbre-tabs" role="tablist" aria-label="Timbres"></div><div class="timbre-add"><select aria-label="New timbre type"><option value="granular">Granular</option><option value="drums">Drum machine</option><option value="synth">Synth engine</option><option value="vocal">Vocal</option><option value="bus">Bus</option></select><button>+ Timbre</button></div></div><div class="timbre-mix"><input aria-label="Timbre name" maxlength="80"><button aria-pressed="false">Mute</button><button aria-pressed="false">Solo</button><label class="timbre-midi">MIDI<select aria-label="Timbre MIDI channel"><option value="0">All</option>${Array.from({ length: 16 }, (_, i) => `<option value="${i + 1}">${i + 1}</option>`).join("")}</select></label><label><span class="timbre-level-name">MASTER</span><input aria-label="Timbre level" type="range" min="0" max="2" step=".01"></label><button aria-label="Remove timbre">×</button></div><div class="timbre-sends" aria-label="Bus sends"></div>`;
      this.tabs = root.querySelector(".timbre-tabs");
      this.addType = root.querySelector(".timbre-add select");
      root.querySelector(".timbre-add button").onclick = () =>
        hooks.change("/api/timbres/add", { kind: this.addType.value });
      this.name = root.querySelector('[aria-label="Timbre name"]');
      this.name.onchange = () => this.update({ name: this.name.value });
      [this.mute, this.solo] = root.querySelectorAll(".timbre-mix > button");
      this.mute.onclick = () => this.update({ muted: !this.active().muted });
      this.solo.onclick = () => this.update({ solo: !this.active().solo });
      this.channel = root.querySelector('[aria-label="Timbre MIDI channel"]');
      this.channel.onchange = () =>
        this.update({ channel: Number(this.channel.value) });
      this.level = root.querySelector('[aria-label="Timbre level"]');
      this.level.oninput = () =>
        this.update({ gain: Number(this.level.value) });
      this.levelName = root.querySelector(".timbre-level-name");
      this.midi = root.querySelector(".timbre-midi");
      this.sends = root.querySelector(".timbre-sends");
      this.remove = root.querySelector('[aria-label="Remove timbre"]');
      this.remove.onclick = () =>
        hooks.change("/api/timbres/remove", { id: this.active().id });
    }
    active() {
      return this.context?.timbres.find((t) => t.id === this.context.active);
    }
    update(data) {
      const active = this.active();
      if (active)
        this.hooks.change(
          "/api/timbres/update",
          { id: active.id, ...data },
          true,
        );
    }
    sync(context) {
      this.root.hidden = !context?.timbres;
      if (!context?.timbres) return;
      this.context = context;
      const signature = JSON.stringify(
        context.timbres.map((t) => [t.id, t.name, t.kind]),
      );
      if (signature !== this.signature) {
        this.tabs.replaceChildren();
        for (const part of context.timbres) {
          const tab = button(part.name, `${part.name} timbre`, () =>
            this.hooks.change("/api/timbres/select", { id: part.id }),
          );
          tab.setAttribute("role", "tab");
          tab.dataset.id = part.id;
          tab.onkeydown = (e) => {
            const index = context.timbres.findIndex((t) => t.id === part.id);
            let next = index;
            if (e.key === "ArrowRight")
              next = (index + 1) % context.timbres.length;
            else if (e.key === "ArrowLeft")
              next =
                (index + context.timbres.length - 1) % context.timbres.length;
            else if (e.key === "Home") next = 0;
            else if (e.key === "End") next = context.timbres.length - 1;
            else return;
            e.preventDefault();
            this.hooks.change("/api/timbres/select", {
              id: context.timbres[next].id,
            });
          };
          this.tabs.append(tab);
        }
        this.signature = signature;
      }
      for (const tab of this.tabs.children) {
        const selected = Number(tab.dataset.id) === context.active;
        tab.setAttribute("aria-selected", String(selected));
        tab.tabIndex = selected ? 0 : -1;
      }
      const part = this.active();
      if (!part) return;
      if (document.activeElement !== this.name) this.name.value = part.name;
      this.mute.setAttribute("aria-pressed", String(part.muted));
      this.solo.setAttribute("aria-pressed", String(part.solo));
      if (document.activeElement !== this.channel)
        this.channel.value = part.channel;
      if (document.activeElement !== this.level) this.level.value = part.gain;
      const routing = part.kind === "bus" || part.kind === "master";
      this.solo.hidden = routing;
      this.midi.hidden = routing;
      this.levelName.textContent =
        part.kind === "master" ? "OUTPUT" : part.kind === "bus" ? "TO MASTER" : "MASTER";
      this.level.setAttribute(
        "aria-label",
        part.kind === "master" ? "Master output level" : "Level into Master",
      );
      this.remove.disabled = part.id === 0 || part.kind === "master";
      this.syncSends(part, routing);
    }
    /** One send per Bus; a timbre's output reaches the bus at this level. */
    syncSends(part, routing) {
      const buses = routing
        ? []
        : this.context.timbres
            .filter((t) => t.kind === "bus" && t.bus > 0)
            .sort((a, b) => a.bus - b.bus);
      this.sends.hidden = !buses.length;
      const signature = JSON.stringify([part.id, buses.map((b) => [b.bus, b.name])]);
      if (signature !== this.sendSignature) {
        this.sendSignature = signature;
        this.sends.replaceChildren(
          ...buses.map((bus) => {
            const label = el("label", "timbre-send", `→ ${bus.name}`);
            const range = el("input");
            range.type = "range";
            range.min = "0";
            range.max = "2";
            range.step = ".01";
            range.dataset.bus = bus.bus;
            range.setAttribute("aria-label", `Send to ${bus.name}`);
            range.oninput = () => {
              const sends = [...(this.active().sends ?? Array(8).fill(0))];
              sends[bus.bus - 1] = Number(range.value);
              this.update({ sends });
            };
            label.append(range);
            return label;
          }),
        );
      }
      for (const range of this.sends.querySelectorAll("input")) {
        if (document.activeElement !== range)
          range.value = part.sends?.[Number(range.dataset.bus) - 1] ?? 0;
      }
    }
  }
  const filters = [
    "Bypass",
    "Lowpass",
    "Highpass",
    "Bandpass",
    "Unit BP",
    "Notch",
    "Peak",
    "LP2",
    "Ladder LP24",
    "K35 LP",
    "K35 HP",
    "Diode LP24",
    "SEM",
    "Morph SVF",
    "Ladder HP24",
    "Ladder BP24",
    "Driven ladder",
    "SVF LP12",
    "SVF LP24",
    "SVF HP12",
    "SVF HP24",
    "SVF BP12",
    "SVF BP24",
    "SVF Notch12",
    "SVF Notch24",
  ];
  const wave = ["Sine", "Triangle", "Saw", "Square", "Noise", "PWM"];
  const defs = [
    ["Pitch", "hz", "log"],
    ["Waveform", "enum", wave],
    ["Decay", "time", "log"],
    ["Attack", "time", "log"],
    ["Slope", "pct"],
    ["Pitch sweep", "st"],
    ["Pitch decay", "time", "log"],
    ["FM ratio", "ratio", "log"],
    ["FM amount", "pct"],
    ["FM waveform", "enum", wave],
    ["Noise mix", "pct"],
    ["Click level", "pct"],
    ["Filter 1 cutoff", "hz", "log"],
    ["Filter 1 resonance", "pct"],
    ["LXR type", "enum", filters.slice(0, 8)],
    ["Drive", "pct"],
    ["Distortion", "enum", ["Off", "Shape", "Fold", "Clip"]],
    ["Reduction", "pct"],
    ["Pan", "pan"],
    ["Voice level", "db"],
    ["LFO rate", "hz", "log"],
    ["LFO depth", "pct"],
    ["LFO target", "enum", ["Pitch", "Cutoff", "FM amount", "Amplitude"]],
    ["LFO wave", "enum", ["Sine", "Triangle", "Saw", "Square", "Random"]],
    ["FM ratio 2", "ratio", "log"],
    ["FM gain 2", "pct"],
    ["Pulse width", "pct"],
    [
      "Transient",
      "enum",
      [
        "Snappy",
        "Offset",
        "Click",
        "Click 2",
        "Tick",
        "Kick",
        "Rim",
        "Drip",
        "Hat",
        "Clap",
        "Kick 2",
        "Snare",
        "Tom",
        "Snap",
      ],
    ],
    ["Repeat", "integer"],
    ["Open decay", "time", "log"],
    [
      "Velocity target",
      "enum",
      ["Off", "Pitch", "Cutoff", "FM amount", "Decay"],
    ],
    ["Velocity amount", "signedpct"],
    ["OSC mode", "enum", ["FM", "Mix"]],
    ["LFO retrigger", "enum", ["Free", "Retrigger"]],
    ["Transient rate", "ratio", "log"],
    ["Noise color", "hz", "log"],
    ["Filter 1 drive", "pct"],
    ["Pitch slope", "pct"],
    ["FM wave 2", "enum", wave],
    ["Filter env attack", "time", "log"],
    ["Filter env decay", "time", "log"],
    ["Filter env curve", "pct"],
    ["Filter 1 env amount", "st"],
    ["Assignable attack", "time", "log"],
    ["Assignable decay", "time", "log"],
    ["Assignable curve", "pct"],
    ["Filter 2 type", "enum", filters],
    ["Filter 2 cutoff", "hz", "log"],
    ["Filter 2 resonance", "pct"],
    ["Filter 2 drive", "pct"],
    ["Filter 2 env amount", "st"],
    ["Filter routing", "enum", ["Serial", "Parallel"]],
    ["Filter balance", "pct"],
    ["Filter 1 morph", "pct"],
    ["Filter 2 morph", "pct"],
    ["Filter 1 model", "enum", ["LXR", ...filters.slice(8)]],
  ].map(([name, unit, scale], id) => ({
    id,
    name,
    unit,
    options: Array.isArray(scale) ? scale : null,
    log: scale === "log",
    min: 0,
    max: 1,
  }));

  const clamp = (x, a, b) => Math.min(b, Math.max(a, x));
  function bindDefinitions(e) {
    for (const d of defs) {
      d.min = e.lxr_param_min(d.id);
      d.max = e.lxr_param_max(d.id);
    }
  }
  function constrain(d, v) {
    v = clamp(v, d.min, d.max);
    return d.options || d.unit === "integer" ? Math.round(v) : v;
  }
  function normalize(d, x) {
    return d.log
      ? Math.log((x + 0.001) / (d.min + 0.001)) /
          Math.log((d.max + 0.001) / (d.min + 0.001))
      : (x - d.min) / (d.max - d.min);
  }
  function denormalize(d, x) {
    return constrain(
      d,
      d.log
        ? (d.min + 0.001) *
            Math.pow((d.max + 0.001) / (d.min + 0.001), clamp(x, 0, 1)) -
            0.001
        : d.min + clamp(x, 0, 1) * (d.max - d.min),
    );
  }
  function format(d, v) {
    if (d.options) return d.options[Math.round(v)] || d.options[0];
    switch (d.unit) {
      case "hz":
        return v >= 1000
          ? `${(v / 1000).toFixed(v >= 10000 ? 1 : 2)} kHz`
          : `${v < 10 ? v.toFixed(2) : Math.round(v)} Hz`;
      case "time":
        return v >= 1
          ? `${v.toFixed(2)} s`
          : `${v < 0.01 ? (v * 1000).toFixed(1) : Math.round(v * 1000)} ms`;
      case "pct":
        return `${Math.round(v * 100)}%`;
      case "signedpct":
        return `${v > 0 ? "+" : ""}${Math.round(v * 100)}%`;
      case "ratio":
        return `${v.toFixed(2)}×`;
      case "st":
        return `${v > 0 ? "+" : ""}${v.toFixed(1)} st`;
      case "pan":
        return Math.abs(v) < 0.02
          ? "CENTER"
          : `${Math.round(Math.abs(v) * 100)} ${v < 0 ? "L" : "R"}`;
      case "db":
        return v < 0.001 ? "−∞ dB" : `${(20 * Math.log10(v)).toFixed(1)} dB`;
      default:
        return `${Math.round(v)}`;
    }
  }

  const names = [
    "Drum 1",
    "Drum 2",
    "Drum 3",
    "Snare",
    "Clap / Cym",
    "Closed HH",
    "Open HH",
  ];
  // Short names inside a module; screen readers keep the full parameter name.
  const short = {
    0: "Pitch", 1: "Wave", 2: "Decay", 3: "Attack", 4: "Curve", 5: "Sweep",
    6: "Decay", 7: "Ratio", 8: "Amount", 9: "Wave", 10: "Noise", 11: "Level",
    12: "Cutoff", 13: "Resonance", 14: "Type", 15: "Drive", 16: "Distortion",
    17: "Reduction", 18: "Pan", 19: "Level", 20: "Rate", 21: "Depth",
    22: "Target", 23: "Wave", 24: "Ratio 2", 25: "Amount 2", 26: "Width",
    27: "Type", 28: "Repeat", 29: "Open decay", 30: "Target", 31: "Amount",
    32: "Mode", 33: "Trigger", 34: "Rate", 35: "Color", 36: "Drive",
    37: "Curve", 38: "Wave 2", 39: "Attack", 40: "Decay", 41: "Curve",
    42: "Env", 43: "Attack", 44: "Decay", 45: "Curve", 46: "Type",
    47: "Cutoff", 48: "Resonance", 49: "Drive", 50: "Env",
    51: "Routing", 52: "Balance", 53: "Morph", 54: "Morph", 55: "Model",
  };
  const show = (p, ...ids) =>
    ids.map((id) => format(defs[id], p[id])).join(" · ");
  // Voice modules: sources, envelopes, filters and output, then modulation.
  // Three to a row in the All layout. Menus sit in the heading so knobs keep
  // the same places from module to module.
  const modules = [
    { title: "Osc", menus: [1, 32], knobs: [0, 26, 10, 35],
      summary: (p) => show(p, 1, 0) },
    { title: "FM", menus: [9, 38], knobs: [7, 8, 24, 25],
      summary: (p) => show(p, 7, 8) },
    { title: "Transient", menus: [27], knobs: [11, 34],
      summary: (p) => show(p, 27, 11) },
    { title: "Amp env", knobs: [3, 2, 4, 28, 29],
      summary: (p) => `${show(p, 3)} / ${show(p, 2)}` },
    { title: "Pitch env", knobs: [5, 6, 37], summary: (p) => show(p, 5, 6) },
    { title: "Filter env", knobs: [39, 40, 41],
      summary: (p) => `${show(p, 39)} / ${show(p, 40)}` },
    // A modeled Filter 1 replaces the LXR type.
    { title: "Filter 1", menus: [55, 14], knobs: [12, 13, 36, 53, 42],
      idle: { 14: (p) => p[55] > 0 },
      summary: (p) => show(p, p[55] > 0 ? 55 : 14, 12) },
    { title: "Filter 2", menus: [46, 51], knobs: [47, 48, 49, 54, 50, 52],
      off: (p) => !p[46], summary: (p) => (p[46] ? show(p, 46, 47) : "Bypass") },
    { title: "Output", menus: [16], knobs: [15, 17, 18, 19],
      summary: (p) => show(p, 16, 15, 19) },
    { title: "LFO", menus: [22, 23, 33], knobs: [20, 21],
      summary: (p) => (p[21] ? `→ ${show(p, 22, 21)}` : "Off") },
    { title: "Velocity", menus: [30], knobs: [31], off: (p) => !p[30],
      summary: (p) => (p[30] ? `→ ${show(p, 30, 31)}` : "Off") },
    { title: "Assign env", knobs: [43, 44, 45],
      summary: (p) => `${show(p, 43)} / ${show(p, 44)}` },
  ];
  const LAYOUT_KEY = "evil-pet-drum-voice-layout";
  class Drums {
    constructor(root, catalog, hooks) {
      this.root = root;
      this.catalog = catalog;
      this.hooks = hooks;
      this.views = new Map();
      this.knobs = [];
      defs.forEach((d, i) => Object.assign(d, catalog.controls[i]));
      root.innerHTML = `<div class="section-heading"><h2>Drum machine <span>DXM-088</span></h2></div><div class="drum-pages" role="tablist" aria-label="Drum pattern pages"></div><div class="drum-pattern-scroll"><div class="drum-pattern" role="group" aria-label="Drum steps"></div></div><div class="drum-track-settings"></div><div class="drum-step-panel"><div class="drum-step-title"></div><div class="drum-step-knobs knob-grid"></div><div class="drum-step-extras"></div></div><div class="drum-voice"><div class="drum-voice-head"><button class="drum-voice-name"></button><button class="drum-voice-previous" aria-label="Previous voice">‹</button><button class="drum-voice-next" aria-label="Next voice">›</button><span class="drum-voice-note"></span><span class="drum-voice-tools"><button class="drum-voice-copy">Copy</button><button class="drum-voice-paste" disabled>Paste</button></span><span class="drum-voice-layout" role="group" aria-label="Voice layout"><button class="drum-voice-chain" title="The signal chain beside one module">Chain</button><button class="drum-voice-all" title="Every module at once">All</button></span></div><div class="drum-voice-body"><div class="drum-chain" role="tablist" aria-label="Voice modules" aria-orientation="vertical"></div><div class="drum-modules"></div></div></div>`;
      this.voicePanel = root.querySelector(".drum-voice");
      this.module = 0;
      this.layout = "chain";
      try {
        if (localStorage.getItem(LAYOUT_KEY) === "all") this.layout = "all";
      } catch {}
      const head = (name) => this.voicePanel.querySelector(`.drum-voice-${name}`);
      head("name").onclick = () => hooks.trigger(this.view().voice);
      for (const [name, offset] of [["previous", -1], ["next", 1]])
        head(name).onclick = () => {
          const view = this.view();
          view.voice = view.track = (view.voice + offset + names.length) % names.length;
          this.signature = "";
          this.sync();
        };
      this.paste = head("paste");
      head("copy").onclick = () => {
        this.copied = [...this.settings().params[Math.min(5, this.view().voice)]];
        this.paste.disabled = false;
      };
      this.paste.onclick = () => {
        const voice = Math.min(5, this.view().voice);
        this.copied?.forEach((value, id) => {
          this.settings().params[voice][id] = value;
          hooks.parameter(`drum${voice + 1}_${id}`, value);
        });
        this.signature = "";
        this.sync();
      };
      for (const layout of ["chain", "all"])
        head(layout).onclick = () => {
          this.layout = layout;
          try {
            localStorage.setItem(LAYOUT_KEY, layout);
          } catch {}
          this.refreshVoice();
        };
      this.chain = this.voicePanel.querySelector(".drum-chain");
      this.chain.onkeydown = (e) => {
        const next = { ArrowDown: this.module + 1, ArrowUp: this.module - 1,
          Home: 0, End: modules.length - 1 }[e.key];
        if (next === undefined) return;
        e.preventDefault();
        this.module = (next + modules.length) % modules.length;
        this.refreshVoice();
        this.parts[this.module].tab.focus();
      };
      this.pages = root.querySelector(".drum-pages");
      for (let p = 0; p < 8; p++) {
        const page = button(
          `${p * 16 + 1}–${p * 16 + 16}`,
          `Drum steps ${p * 16 + 1} to ${p * 16 + 16}`,
          () => {
            this.view().page = p;
            this.buildPattern();
            this.sync();
          },
        );
        page.setAttribute("role", "tab");
        page.dataset.page = p;
        this.pages.append(page);
      }
    }
    settings() {
      return this.hooks.read().drums;
    }
    view() {
      const id = this.hooks.context()?.active ?? 0;
      if (!this.views.has(id))
        this.views.set(id, { page: 0, voice: 0, track: 0, step: 0 });
      return this.views.get(id);
    }
    edit(track, step, field, value) {
      const settings = this.settings();
      if (!settings) return;
      const t = settings.tracks[track];
      const stepKeys = [
        "velocity",
        "probability",
        "note",
        "ratchet",
        "micro",
        "condition",
        "every",
        "phase",
      ];
      const trackKeys = [
        "length",
        "division",
        "delay",
        "groove",
        "groove_amount",
        "swing",
        "level",
        "muted",
        "solo",
        "midi_note",
      ];
      if (step != null) {
        if (field < 8) t.steps[step][stepKeys[field]] = value;
        else {
          const lock = t.steps[step].locks[Math.floor((field - 8) / 2)];
          lock[field % 2 === 0 ? "param" : "value"] = value;
        }
      } else t[trackKeys[field]] = field === 7 || field === 8 ? !!value : value;
      this.hooks.edit({ track, step, field, value });
      this.sync();
    }
    field(parent, label, min, max, step, get, set) {
      const prefix = names.find((name) => label.startsWith(name + " "));
      const box = el(
          "label",
          "",
          prefix ? label.slice(prefix.length + 1) : label,
        ),
        input = el("input");
      input.type = "number";
      input.min = min;
      input.max = max;
      input.step = step;
      input.setAttribute("aria-label", label);
      input.oninput = () => {
        const value = Number(input.value);
        if (Number.isFinite(value)) set(Math.max(min, Math.min(max, value)));
      };
      box.append(input);
      parent.append(box);
      this.fields.push(() => {
        if (document.activeElement !== input)
          input.value = Number(
            Number(get()).toFixed(Math.max(0, -Math.floor(Math.log10(step)))),
          );
      });
      return input;
    }
    select(parent, label, options, get, set, text) {
      const prefix = names.find((name) => label.startsWith(name + " "));
      const box = el(
          "label",
          "",
          text ?? (prefix ? label.slice(prefix.length + 1) : label),
        ),
        select = el("select");
      select.setAttribute("aria-label", label);
      options.forEach((name, i) => {
        const option = el("option", "", name);
        option.value = i;
        select.append(option);
      });
      select.onchange = () => set(Number(select.value));
      box.append(select);
      parent.append(box);
      this.fields.push(() => {
        if (document.activeElement !== select) select.value = get();
      });
      return select;
    }
    buildPattern() {
      const grid = this.root.querySelector(".drum-pattern");
      grid.replaceChildren();
      const view = this.view();
      grid.append(el("span", "drum-corner", ""));
      for (let i = 0; i < 16; i++)
        grid.append(
          el(
            "span",
            "drum-step-number",
            String(view.page * 16 + i + 1).padStart(2, "0"),
          ),
        );
      this.hits = [];
      names.forEach((name, t) => {
        const title = el("div", "drum-track-label");
        const select = button(name, `${name} track`, () => {
          this.view().track = t;
          this.view().voice = t;
          this.signature = "";
          this.sync();
        });
        const audition = button("▷", `Audition ${name}`, () =>
          this.hooks.trigger(t),
        );
        title.append(select, audition);
        grid.append(title);
        for (let i = 0; i < 16; i++) {
          const step = view.page * 16 + i;
          const hit = button("", `${name} step ${step + 1}`, (e) => {
            this.view().track = t;
            this.view().voice = t;
            this.view().step = step;
            if (!e.shiftKey) {
              const current = this.settings().tracks[t];
              if (step >= current.length) this.edit(t, null, 0, step + 1);
              this.edit(
                t,
                step,
                0,
                current.steps[step].velocity > 0 ? 0 : 0.85,
              );
            }
            this.signature = "";
            this.sync();
          });
          hit.className = "drum-hit";
          hit.dataset.track = t;
          hit.dataset.step = step;
          grid.append(hit);
          this.hits.push(hit);
        }
      });
    }
    buildEditor() {
      const view = this.view(),
        track = view.track,
        step = view.step;
      const settings = this.root.querySelector(".drum-track-settings");
      settings.replaceChildren();
      this.fields = [];
      this.knobs = [];
      this.field(
        settings,
        `${names[track]} length`,
        1,
        128,
        1,
        () => this.settings().tracks[track].length,
        (v) => this.edit(track, null, 0, Math.round(v)),
      );
      this.select(
        settings,
        `${names[track]} clock`,
        [
          "1 bar",
          "1/2",
          "1/4",
          "1/8",
          "1/16",
          "1/32",
          "1/64",
          "1/4 T",
          "1/8 T",
          "1/16 T",
          "1/32 T",
          "1/8 D",
          "1/16 D",
        ],
        () => this.settings().tracks[track].division,
        (v) => this.edit(track, null, 1, v),
      );
      this.field(
        settings,
        `${names[track]} MIDI note`,
        0,
        127,
        1,
        () => this.settings().tracks[track].midi_note,
        (v) => this.edit(track, null, 9, Math.round(v)),
      );
      this.field(
        settings,
        `${names[track]} level`,
        0,
        1.25,
        0.01,
        () => this.settings().tracks[track].level,
        (v) => this.edit(track, null, 6, v),
      );
      this.field(
        settings,
        `${names[track]} swing`,
        0,
        0.5,
        0.01,
        () => this.settings().tracks[track].swing,
        (v) => this.edit(track, null, 5, v),
      );
      this.select(
        settings,
        `${names[track]} groove`,
        ["Straight", "Swing", "Groove 1", "Groove 2", "Groove 3"],
        () => this.settings().tracks[track].groove,
        (v) => this.edit(track, null, 3, v),
      );
      this.field(
        settings,
        `${names[track]} groove amount`,
        0,
        1,
        0.01,
        () => this.settings().tracks[track].groove_amount,
        (v) => this.edit(track, null, 4, v),
      );
      for (const [name, field, key] of [
        ["Mute", 7, "muted"],
        ["Solo", 8, "solo"],
      ]) {
        const node = button(name, `${names[track]} ${name.toLowerCase()}`, () =>
          this.edit(
            track,
            null,
            field,
            Number(!this.settings().tracks[track][key]),
          ),
        );
        settings.append(node);
        this.fields.push(() =>
          node.setAttribute(
            "aria-pressed",
            String(this.settings().tracks[track][key]),
          ),
        );
      }
      this.root.querySelector(".drum-step-title").textContent =
        `${names[track]} · Step ${step + 1}`;
      const knobs = this.root.querySelector(".drum-step-knobs");
      knobs.replaceChildren();
      for (const [key, label, field, min, max, integer, format] of [
        [
          "velocity",
          "VELOCITY",
          0,
          0,
          1,
          false,
          (v) => `${Math.round(v * 100)}%`,
        ],
        [
          "probability",
          "CHANCE",
          1,
          0,
          1,
          false,
          (v) => `${Math.round(v * 100)}%`,
        ],
        [
          "note",
          "PITCH",
          2,
          -36,
          36,
          true,
          (v) => `${v > 0 ? "+" : ""}${Math.round(v)} st`,
        ],
        ["ratchet", "RATCHET", 3, 1, 4, true, (v) => String(Math.round(v))],
        [
          "micro",
          "MICRO",
          4,
          -0.49,
          0.49,
          false,
          (v) => `${v > 0 ? "+" : ""}${Math.round(v * 100)}%`,
        ],
      ])
        this.knobs.push(
          this.hooks.knob(
            [
              `drum_step_${key}`,
              label,
              format,
              min,
              max,
              `${names[track]} step ${step + 1} ${key}`,
            ],
            knobs,
            {
              register: false,
              ariaScale: integer ? 1 : 100,
              step: integer ? 1 : undefined,
              get: () => this.settings().tracks[track].steps[step][key],
              set: (v) =>
                this.edit(track, step, field, integer ? Math.round(v) : v),
              reset: () => (key === "probability" || key === "ratchet" ? 1 : 0),
            },
          ),
        );
      const extra = this.root.querySelector(".drum-step-extras");
      extra.replaceChildren();
      this.select(
        extra,
        "Step condition",
        [
          "Always",
          "First",
          "Not first",
          "Fill",
          "Not fill",
          "Previous",
          "Not previous",
          "Every N",
        ],
        () => this.settings().tracks[track].steps[step].condition,
        (v) => this.edit(track, step, 5, v),
      );
      this.field(
        extra,
        "Every N",
        1,
        16,
        1,
        () => this.settings().tracks[track].steps[step].every,
        (v) => this.edit(track, step, 6, Math.round(v)),
      );
      this.field(
        extra,
        "Cycle phase",
        0,
        15,
        1,
        () => this.settings().tracks[track].steps[step].phase,
        (v) => this.edit(track, step, 7, Math.round(v)),
      );
      for (let i = 0; i < 2; i++) {
        const list = el("label", "", `Lock ${i + 1}`),
          select = el("select");
        select.setAttribute("aria-label", `Step lock ${i + 1} parameter`);
        const none = el("option", "", "Off");
        none.value = 255;
        select.append(none);
        defs.forEach((d) => {
          const option = el("option", "", d.name);
          option.value = d.id;
          select.append(option);
        });
        select.onchange = () => {
          this.edit(track, step, 8 + i * 2, Number(select.value));
          this.signature = "";
          this.sync();
        };
        list.append(select);
        extra.append(list);
        this.fields.push(
          () =>
            (select.value =
              this.settings().tracks[track].steps[step].locks[i].param),
        );
        const param = this.settings().tracks[track].steps[step].locks[i].param;
        if (param < 56) {
          const d = defs[param];
          this.field(
            extra,
            `Lock ${i + 1} value`,
            d.min,
            d.max,
            d.options ? 1 : (d.max - d.min) / 200,
            () => this.settings().tracks[track].steps[step].locks[i].value,
            (v) => this.edit(track, step, 9 + i * 2, constrain(d, v)),
          );
        }
      }
      this.buildVoice();
    }
    // Closed and Open HH play one voice; the open hat swaps Decay for Open decay.
    buildVoice() {
      const track = this.view().voice,
        voice = Math.min(5, track),
        name = names[track];
      this.voicePanel.querySelector(".drum-voice-name").textContent = `▷ ${name}`;
      this.voicePanel.querySelector(".drum-voice-name").setAttribute("aria-label", `Audition ${name}`);
      this.voicePanel.querySelector(".drum-voice-note").textContent =
        voice === 5 ? `Shared with ${names[track === 5 ? 6 : 5]}` : "";
      const panels = this.voicePanel.querySelector(".drum-modules");
      this.chain.replaceChildren();
      panels.replaceChildren();
      this.parts = modules.map((module, index) => {
        const tab = button("", `${module.title} module`, () => {
          this.module = index;
          this.refreshVoice();
        });
        tab.setAttribute("role", "tab");
        const summary = el("span");
        tab.append(el("b", "", module.title), summary);
        this.chain.append(tab);
        const panel = el("section", "drum-module"),
          heading = el("div", "drum-module-head"),
          grid = el("div", "drum-module-knobs");
        panel.setAttribute("aria-label", `${name} ${module.title}`);
        heading.append(el("h3", "", module.title));
        panel.append(heading, grid);
        panels.append(panel);
        const set = (id) => (v) => {
          const value = constrain(defs[id], v);
          this.settings().params[voice][id] = value;
          this.hooks.parameter(`drum${voice + 1}_${id}`, value);
          this.refreshVoice();
        };
        const menus = {};
        for (const id of module.menus ?? [])
          menus[id] = this.select(
            heading,
            `${name} ${defs[id].name}`,
            defs[id].options,
            () => this.settings().params[voice][id],
            set(id),
            short[id],
          ).parentNode;
        for (const id of module.knobs) {
          if (id === 29 && voice !== 5) continue;
          const d = defs[id];
          this.knobs.push(
            this.hooks.knob(
              [
                `drum${voice + 1}_${id}`,
                (id === 2 && voice === 5 ? "Closed decay" : short[id]).toUpperCase(),
                (v) => format(d, v),
                d.min,
                d.max,
                `${name} ${d.name}`,
              ],
              grid,
              {
                register: false,
                ariaScale: 1,
                normalize: d.log ? (v) => normalize(d, v) : undefined,
                denormalize: d.log ? (v) => denormalize(d, v) : undefined,
                step: d.unit === "integer" ? 1 : undefined,
                get: () => this.settings().params[voice][id],
                set: set(id),
                reset: () => this.catalog.defaults[voice][id],
              },
            ),
          );
        }
        return { module, tab, summary, panel, menus };
      });
      this.refreshVoice();
    }
    refreshVoice() {
      const params = this.settings()?.params[Math.min(5, this.view().voice)];
      if (!params || !this.parts) return;
      const all = this.layout === "all";
      this.voicePanel.classList.toggle("all", all);
      for (const layout of ["chain", "all"])
        this.voicePanel
          .querySelector(`.drum-voice-${layout}`)
          .setAttribute("aria-pressed", String(layout === this.layout));
      this.parts.forEach(({ module, tab, summary, panel, menus }, index) => {
        const selected = index === this.module,
          off = !!module.off?.(params);
        summary.textContent = module.summary(params);
        tab.setAttribute("aria-selected", String(selected));
        tab.tabIndex = selected ? 0 : -1;
        tab.classList.toggle("off", off);
        panel.classList.toggle("off", off);
        panel.hidden = !all && !selected;
        for (const [id, idle] of Object.entries(module.idle ?? {}))
          menus[id].classList.toggle("idle", idle(params));
      });
    }
    sync() {
      const settings = this.settings();
      this.root.hidden = !settings;
      if (!settings) return;
      const v = this.view(),
        id = this.hooks.context()?.active ?? 0,
        signature = `${id}:${v.track}:${v.step}:${v.voice}`;
      if (this.page !== `${id}:${v.page}`) {
        this.page = `${id}:${v.page}`;
        this.buildPattern();
      }
      if (this.signature !== signature) {
        this.signature = signature;
        this.buildEditor();
      }
      for (const tab of this.pages.children)
        tab.setAttribute(
          "aria-selected",
          String(Number(tab.dataset.page) === v.page),
        );
      for (const hit of this.hits) {
        const t = Number(hit.dataset.track),
          s = Number(hit.dataset.step),
          step = settings.tracks[t].steps[s];
        hit.setAttribute("aria-pressed", String(step.velocity > 0));
        hit.classList.toggle("selected", t === v.track && s === v.step);
        hit.classList.toggle("outside", s >= settings.tracks[t].length);
        hit.style.setProperty("--velocity", step.velocity);
      }
      for (const [track, row] of [...this.root.querySelectorAll(".drum-track-label")].entries())
        row.querySelector("button").setAttribute(
          "aria-pressed", String(track === v.track),
        );
      this.fields.forEach((update) => update());
      this.knobs.forEach((update) => update());
      this.refreshVoice();
    }
    update(view) {
      if (!this.settings()) return;
      for (const hit of this.hits || [])
        hit.classList.toggle(
          "playing",
          view?.playing &&
            view.positions[Number(hit.dataset.track)] ===
              Number(hit.dataset.step),
        );
    }
  }
  window.SadTimbres = Timbres;
  window.SadDrums = Drums;
})();
