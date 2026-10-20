/* Neuropolis stage, pulse, gate and lane editor. Audio runs in metropolix-core. */
"use strict";
window.SadNeuropolis = class {
  constructor(root, catalog, hooks) {
    this.root = root;
    this.catalog = catalog;
    this.hooks = hooks;
    this.selected = null;
    this.page = 0;
    this.layer = "pitch";
    this.signature = "";
    root.innerHTML = `<div class="section-heading"><h2>Neuropolis <span class="neuro-count"></span></h2><div class="neuro-actions"><label class="check"><input class="neuro-enabled" type="checkbox">Enabled</label><button class="neuro-restart" title="Restart all sequencer tracks">Restart</button><button class="neuro-add">+ Add track</button></div></div><div class="neuro-layout"><aside><div class="neuro-tracks" role="tablist" aria-label="Sequencer tracks"></div><div class="neuro-pages"><button class="neuro-prev" aria-label="Previous tracks">←</button><output></output><button class="neuro-next" aria-label="Next tracks">→</button></div></aside><div class="neuro-editor"></div></div>`;
    root.querySelector(".neuro-enabled").onchange = (e) =>
      this.edit(null, null, 0, Number(e.target.checked));
    root.querySelector(".neuro-restart").onclick = () =>
      this.edit(null, null, 1, 1);
    root.querySelector(".neuro-add").onclick = () => this.add();
    root.querySelector(".neuro-prev").onclick = () =>
      this.setPage(this.page - 1);
    root.querySelector(".neuro-next").onclick = () =>
      this.setPage(this.page + 1);
  }
  program() {
    return this.hooks.read().sequencer;
  }
  track() {
    return this.program().tracks.find((t) => t.id === this.selected);
  }
  setPage(page) {
    this.page = Math.max(
      0,
      Math.min(page, Math.ceil(this.program().tracks.length / 64) - 1),
    );
    this.signature = "";
    this.sync();
    this.hooks.view(this.page * 64);
  }
  add(copy = false) {
    const p = this.program(),
      base = copy ? this.track() : this.catalog.track;
    const track = structuredClone(base),
      used = new Set(p.tracks.map((t) => t.id));
    track.id = p.tracks.reduce((max, t) => Math.max(max, t.id), 0) + 1;
    if (track.id > 4294967295) {
      track.id = 1;
      while (used.has(track.id)) track.id++;
    }
    track.name = copy ? `${base.name} copy` : `Track ${track.id}`;
    p.tracks.push(track);
    this.selected = track.id;
    this.page = Math.floor((p.tracks.length - 1) / 64);
    this.signature = "";
    this.hooks.structure();
    this.sync();
    this.hooks.view(this.page * 64);
  }
  edit(track, stage, field, value) {
    if (!Number.isFinite(value)) return;
    if (!track) {
      if (field === 0) this.program().enabled = !!value;
    } else if (stage !== null)
      track.steps[stage][this.stepFields[field]] = this.boolFields.has(field)
        ? !!value
        : value;
    else if (field < 43) track.config[field] = Math.round(value);
    else
      track[
        { 43: "amount", 44: "absolute", 45: "gate_output", 46: "root_note" }[
          field
        ]
      ] = field === 44 || field === 45 ? !!value : value;
    this.hooks.edit({ track: track?.id || 0, stage, field, value });
    this.sync();
  }
  get stepFields() {
    return [
      "pitch",
      "pulses",
      "gate",
      "ratchet",
      "probability",
      "slide",
      "skip",
      "accumulate",
      "pitch_override",
      "gate_override",
      "probability_target",
      "accumulate_trigger",
      "cv",
      "cv_ramp",
      "cv_gate",
      "velocity",
    ];
  }
  get boolFields() {
    return new Set([5, 6, 14]);
  }
  note(n) {
    return (
      ["C", "C♯", "D", "D♯", "E", "F", "F♯", "G", "G♯", "A", "A♯", "B"][
        Math.round(n) % 12
      ] +
      (Math.floor(n / 12) - 1)
    );
  }
  targetName(t) {
    return t.target.kind === "main_pitch"
      ? "Main pitch"
      : t.target.kind === "notes"
        ? "Notes → grains + ENV"
        : t.target.kind === "oscillator"
          ? `OSC ${t.target.index + 1} pitch`
          : this.catalog.parameters.find((p) => p.id === t.target.parameter)
              ?.name || t.target.parameter;
  }
  speed(n) {
    return n === 65
      ? "×2 · 1/32"
      : n === 66
        ? "×4 · 1/64"
        : n === 1
          ? "1/16"
          : `÷${n}`;
  }
  option(value, text) {
    const o = document.createElement("option");
    o.value = value;
    o.textContent = text;
    return o;
  }
  sync() {
    const p = this.program();
    if (!p) return;
    if (!p.tracks.some((t) => t.id === this.selected))
      this.selected = p.tracks[0]?.id ?? null;
    this.page = Math.max(
      0,
      Math.min(this.page, Math.ceil(p.tracks.length / 64) - 1),
    );
    this.root.querySelector(".neuro-enabled").checked = p.enabled;
    this.root.querySelector(".neuro-count").textContent =
      `${p.tracks.length} ${p.tracks.length === 1 ? "track" : "tracks"}`;
    const key = JSON.stringify([
      p.tracks
        .slice(this.page * 64, this.page * 64 + 64)
        .map((t) => [t.id, t.name, t.target, t.config[3], t.config[8]]),
      this.selected,
      this.layer,
      this.hooks.read().color_chain.map((s) => [s.id, s.kind]),
      this.hooks.read().pre_color_chain.map((s) => [s.id, s.kind]),
      this.hooks.read().resynth.count,
    ]);
    if (key !== this.signature) {
      this.signature = key;
      this.buildTracks();
      this.buildEditor();
    }
    const track = this.track();
    if (!track) return;
    for (const input of this.root.querySelectorAll("[data-config]")) {
      const f = Number(input.dataset.config),
        value =
          f < 43
            ? track.config[f]
            : track[
                {
                  43: "amount",
                  44: "absolute",
                  45: "gate_output",
                  46: "root_note",
                }[f]
              ];
      if (document.activeElement !== input) {
        if (input.type === "checkbox") input.checked = !!value;
        else input.value = Number(value);
      }
    }
    for (const input of this.root.querySelectorAll("[data-stage]")) {
      const stage = track.steps[Number(input.dataset.stage)],
        field = Number(input.dataset.field),
        value = stage[this.stepFields[field]];
      if (document.activeElement !== input) {
        if (input.type === "checkbox") input.checked = !!value;
        else input.value = Number(value);
      }
    }
    const kind = track.target.kind,
      valueLayer =
        kind === "parameter" && this.layer === "pitch" ? "cv" : this.layer;
    this.root.querySelectorAll(".neuro-stage").forEach((card, i) => {
      const step = track.steps[i],
        value = step[valueLayer];
      card.classList.toggle("outside", i >= track.config[1]);
      card.classList.toggle("rest", step.gate === 0 || step.skip);
      card.querySelector(".neuro-value").textContent =
        valueLayer === "pitch"
          ? this.note(value)
          : valueLayer === "cv"
            ? `${value > 0 ? "+" : ""}${(value / 50).toFixed(0)}%`
            : typeof value === "boolean"
              ? value
                ? "On"
                : "Off"
              : value;
      card
        .querySelector(".neuro-stage-number")
        .setAttribute("aria-pressed", String(!step.skip));
    });
  }
  buildTracks() {
    const p = this.program(),
      list = this.root.querySelector(".neuro-tracks");
    list.replaceChildren();
    for (const t of p.tracks.slice(this.page * 64, this.page * 64 + 64)) {
      const row = document.createElement("div");
      row.className = "neuro-track";
      row.dataset.track = t.id;
      const tab = document.createElement("button");
      tab.type = "button";
      tab.className = "neuro-track-select";
      tab.setAttribute("role", "tab");
      tab.setAttribute("aria-selected", String(t.id === this.selected));
      const name = document.createElement("strong");
      name.textContent = t.name;
      const target = document.createElement("span");
      target.textContent = this.targetName(t);
      const clock = document.createElement("small");
      clock.textContent = this.speed(t.config[3]);
      const activity = document.createElement("i");
      activity.className = "neuro-track-led";
      tab.append(activity, name, target, clock);
      tab.onclick = () => {
        this.selected = t.id;
        this.signature = "";
        this.sync();
      };
      const mute = document.createElement("button");
      mute.type = "button";
      mute.className = "neuro-mute";
      mute.textContent = "M";
      mute.setAttribute("aria-label", `Mute ${t.name}`);
      mute.setAttribute("aria-pressed", String(!!t.config[8]));
      mute.onclick = () => this.edit(t, null, 8, 1 - t.config[8]);
      row.append(tab, mute);
      list.append(row);
    }
    const pages = this.root.querySelector(".neuro-pages");
    pages.hidden = p.tracks.length <= 64;
    pages.querySelector("output").textContent =
      `${this.page * 64 + 1}–${Math.min(p.tracks.length, this.page * 64 + 64)}`;
    pages.querySelector(".neuro-prev").disabled = this.page === 0;
    pages.querySelector(".neuro-next").disabled =
      (this.page + 1) * 64 >= p.tracks.length;
  }
  configControl(
    container,
    text,
    field,
    options = null,
    min = 0,
    max = 100,
    step = 1,
    type = "number",
  ) {
    const label = document.createElement("label");
    label.className = "neuro-control";
    const title = document.createElement("span");
    title.textContent = text;
    const input = document.createElement(options ? "select" : "input");
    input.dataset.config = field;
    input.setAttribute("aria-label", `Track ${text.toLowerCase()}`);
    if (options)
      for (const [value, name] of options)
        input.append(this.option(value, name));
    else {
      input.type = type;
      input.min = min;
      input.max = max;
      input.step = step;
    }
    input.oninput = () => {
      if (input.value === "") return;
      this.edit(
        this.track(),
        null,
        field,
        type === "checkbox" && !options
          ? Number(input.checked)
          : Number(input.value),
      );
    };
    label.append(title, input);
    container.append(label);
    return input;
  }
  buildEditor() {
    const editor = this.root.querySelector(".neuro-editor"),
      track = this.track();
    editor.replaceChildren();
    if (!track) {
      const empty = document.createElement("p");
      empty.className = "muted";
      empty.textContent = "Add a track to start a pattern.";
      editor.append(empty);
      return;
    }
    const header = document.createElement("div");
    header.className = "neuro-edit-head";
    const name = document.createElement("input");
    name.type = "text";
    name.value = track.name;
    name.setAttribute("aria-label", "Track name");
    name.onchange = () => {
      track.name = name.value.trim() || `Track ${track.id}`;
      this.signature = "";
      this.hooks.structure();
      this.sync();
    };
    const target = document.createElement("select");
    target.setAttribute("aria-label", "Track destination");
    const group = document.createElement("optgroup");
    group.label = "Pitch & notes";
    for (const [value, label] of [
      ["main_pitch", "Main track pitch"],
      ["notes", "Notes → grains + ENV"],
      ...[0, 1, 2, 3].map((i) => [`osc:${i}`, `OSC ${i + 1} pitch`]),
    ])
      group.append(this.option(value, label));
    target.append(group);
    const preset = this.hooks.read(),
      groups = new Map();
    for (const spec of this.catalog.parameters) {
      const fx = /^(pre_)?fx(\d+)_/.exec(spec.id);
      if (fx) {
        const slot = (fx[1] ? preset.pre_color_chain : preset.color_chain)[
          Number(fx[2]) - 1
        ];
        if (!slot) continue;
      }
      const family = fx
        ? `${fx[1] ? "Pre-filter" : "Post-filter"} FX ${fx[2]}`
        : spec.id.startsWith("resynth_")
          ? "Resynth"
          : spec.id.startsWith("env")
            ? "Envelopes"
            : spec.id.startsWith("lfo")
              ? "LFO"
              : spec.id.startsWith("arp_")
                ? "Arpeggiator"
                : "Main & filters";
      if (!groups.has(family)) {
        const g = document.createElement("optgroup");
        g.label = family;
        groups.set(family, g);
        target.append(g);
      }
      groups.get(family).append(this.option(`param:${spec.id}`, spec.name));
    }
    target.value =
      track.target.kind === "oscillator"
        ? `osc:${track.target.index}`
        : track.target.kind === "parameter"
          ? `param:${track.target.parameter}`
          : track.target.kind;
    if (!target.value)
      (target.append(
        this.option(
          `param:${track.target.parameter}`,
          `${track.target.parameter} · unavailable`,
        ),
      ),
        (target.value = `param:${track.target.parameter}`));
    target.onchange = () => {
      track.target = target.value.startsWith("osc:")
        ? { kind: "oscillator", index: Number(target.value.slice(4)) }
        : target.value.startsWith("param:")
          ? { kind: "parameter", parameter: target.value.slice(6) }
          : { kind: target.value };
      this.signature = "";
      this.hooks.structure();
      this.sync();
    };
    const duplicate = document.createElement("button");
    duplicate.textContent = "Duplicate";
    duplicate.onclick = () => this.add(true);
    const remove = document.createElement("button");
    remove.textContent = "Remove";
    remove.onclick = () => {
      this.program().tracks = this.program().tracks.filter(
        (t) => t.id !== track.id,
      );
      this.signature = "";
      this.hooks.structure();
      this.sync();
    };
    header.append(name, target, duplicate, remove);
    editor.append(header);
    const controls = document.createElement("div");
    controls.className = "neuro-controls";
    this.configControl(
      controls,
      "Order",
      0,
      this.catalog.orders.map((n, i) => [i, n]),
    );
    this.configControl(controls, "Clock", 3, [
      [66, "×4 · 1/64"],
      [65, "×2 · 1/32"],
      ...Array.from({ length: 64 }, (_, i) => [i + 1, this.speed(i + 1)]),
    ]);
    this.configControl(controls, "Length", 1, null, 1, 16);
    this.configControl(controls, "Swing %", 11, null, 50, 78);
    this.configControl(controls, "Gate %", 7, null, 0, 100);
    this.configControl(controls, "Transpose", 6, null, -60, 60);
    if (track.target.kind === "parameter") {
      this.configControl(controls, "Depth", 43, null, -1, 1, 0.01);
      this.configControl(controls, "Mapping", 44, [
        [0, "Relative"],
        [1, "Absolute"],
      ]);
    } else {
      this.configControl(
        controls,
        "Root note",
        46,
        Array.from({ length: 128 }, (_, i) => [i, this.note(i)]),
      );
      this.configControl(controls, "Depth", 43, null, -1, 1, 0.01);
    }
    if (track.target.kind === "oscillator")
      this.configControl(
        controls,
        "Gate oscillator",
        45,
        null,
        0,
        1,
        1,
        "checkbox",
      );
    editor.append(controls);
    const toolbar = document.createElement("div");
    toolbar.className = "neuro-stage-tools";
    const layer = document.createElement("select");
    layer.setAttribute("aria-label", "Stage layer");
    const layers = [
      ["pitch", "Pitch"],
      ["cv", "Parameter value"],
      ["ratchet", "Ratchet"],
      ["probability", "Probability"],
      ["velocity", "Velocity"],
      ["slide", "Slide"],
      ["skip", "Skip"],
      ["accumulate", "Accumulate"],
      ["pitch_override", "Pitch override"],
      ["gate_override", "Gate override"],
      ["probability_target", "Probability target"],
      ["accumulate_trigger", "Accumulate trigger"],
      ["cv_ramp", "CV ramp"],
      ["cv_gate", "CV gate"],
    ];
    for (const [v, n] of layers) layer.append(this.option(v, n));
    layer.value =
      track.target.kind === "parameter" && this.layer === "pitch"
        ? "cv"
        : this.layer;
    layer.onchange = () => {
      this.layer = layer.value;
      this.signature = "";
      this.sync();
    };
    const fill = document.createElement("button");
    fill.textContent = "Copy first → all";
    fill.onclick = () => {
      const f = this.stepFields.indexOf(layer.value);
      for (let i = 1; i < 16; i++)
        this.edit(track, i, f, Number(track.steps[0][layer.value]));
    };
    const live = document.createElement("output");
    live.className = "neuro-live";
    live.textContent = "—";
    toolbar.append(layer, fill, live);
    editor.append(toolbar);
    const scroll = document.createElement("div");
    scroll.className = "neuro-stage-scroll";
    const grid = document.createElement("div");
    grid.className = "neuro-stages";
    grid.setAttribute("aria-label", "Neuropolis stages");
    const ranges = {
      pitch: [0, 127],
      cv: [-5000, 5000],
      ratchet: [1, 8],
      probability: [0, 100],
      velocity: [1, 127],
      slide: [0, 1],
      skip: [0, 1],
      accumulate: [-28, 28],
      pitch_override: [-1, 127],
      gate_override: [-1, 100],
      probability_target: [0, 1],
      accumulate_trigger: [0, 2],
      cv_ramp: [0, 2],
      cv_gate: [0, 1],
    };
    const selected = layer.value,
      field = this.stepFields.indexOf(selected),
      [min, max] = ranges[selected];
    for (let i = 0; i < 16; i++) {
      const card = document.createElement("div");
      card.className = "neuro-stage";
      const number = document.createElement("button");
      number.className = "neuro-stage-number";
      number.textContent = String(i + 1).padStart(2, "0");
      number.setAttribute("aria-label", `Stage ${i + 1} enabled`);
      number.onclick = () =>
        this.edit(track, i, 6, Number(!track.steps[i].skip));
      const value = document.createElement("output");
      value.className = "neuro-value";
      const fader = document.createElement("input");
      fader.type = "range";
      fader.className = "neuro-fader";
      fader.min = min;
      fader.max = max;
      fader.step = 1;
      fader.dataset.stage = i;
      fader.dataset.field = field;
      fader.setAttribute(
        "aria-label",
        `Stage ${i + 1} ${selected.replaceAll("_", " ")}`,
      );
      fader.oninput = () => this.edit(track, i, field, Number(fader.value));
      const pulses = document.createElement("input");
      pulses.type = "number";
      pulses.min = 1;
      pulses.max = 8;
      pulses.dataset.stage = i;
      pulses.dataset.field = 1;
      pulses.setAttribute("aria-label", `Stage ${i + 1} pulses`);
      pulses.title = "Pulses";
      pulses.oninput = () => {
        if (pulses.value != "")
          this.edit(
            track,
            i,
            1,
            Math.max(1, Math.min(8, Number(pulses.value))),
          );
      };
      const gate = document.createElement("select");
      gate.dataset.stage = i;
      gate.dataset.field = 2;
      gate.setAttribute("aria-label", `Stage ${i + 1} gate`);
      for (const [v, n] of [
        [0, "Rest"],
        [1, "Single"],
        [2, "Multi"],
        [3, "Hold"],
      ])
        gate.append(this.option(v, n));
      gate.onchange = () => this.edit(track, i, 2, Number(gate.value));
      card.append(number, value, fader, pulses, gate);
      grid.append(card);
    }
    scroll.append(grid);
    editor.append(scroll);
    const advanced = document.createElement("details");
    advanced.className = "neuro-advanced";
    advanced.innerHTML = "<summary>Scale, slide & accumulation</summary>";
    const extra = document.createElement("div");
    extra.className = "neuro-controls";
    this.configControl(
      extra,
      "Scale",
      4,
      this.catalog.scales.map((s, i) => [i, s]),
    );
    this.configControl(
      extra,
      "Scale root",
      5,
      ["C", "C♯", "D", "D♯", "E", "F", "F♯", "G", "G♯", "A", "A♯", "B"].map(
        (n, i) => [i, n],
      ),
    );
    this.configControl(extra, "Reverse", 2, [
      [0, "Off"],
      [1, "On"],
    ]);
    this.configControl(extra, "Start offset", 14, null, 0, 15);
    this.configControl(extra, "Slide %", 12, null, 0, 100);
    this.configControl(extra, "Slide type", 13, [
      [0, "Analog"],
      [1, "Tempo"],
      [2, "Acid"],
    ]);
    this.configControl(extra, "Pulse length", 15, null, 0, 64);
    this.configControl(extra, "Pulse divide", 16, null, 1, 8);
    this.configControl(extra, "Gate scale %", 17, null, 0, 100);
    this.configControl(extra, "Ratchet mode", 27, [
      [0, "Multiply"],
      [1, "Pulse"],
      [2, "Gated"],
    ]);
    this.configControl(extra, "Acc. order", 9, [
      [0, "Wrap"],
      [1, "Pendulum"],
      [2, "Random walk"],
      [3, "Hold"],
    ]);
    this.configControl(extra, "Acc. positive", 10, null, 1, 28);
    this.configControl(extra, "Acc. negative", 18, null, 1, 28);
    this.configControl(extra, "Acc. scope", 19, [
      [0, "Stage"],
      [1, "Track"],
    ]);
    this.configControl(extra, "Acc. polarity", 20, [
      [0, "Unipolar"],
      [1, "Bipolar"],
    ]);
    this.configControl(extra, "Acc. reset", 21, [
      [0, "Manual"],
      [1, "Auto"],
      [2, "Mod only"],
      [3, "After stages"],
    ]);
    this.configControl(extra, "Acc. stages", 42, null, 1, 1024);
    this.configControl(extra, "Velocity %", 30, null, 0, 100);
    this.configControl(extra, "CV mode", 32, [
      [0, "Bipolar"],
      [1, "Unipolar"],
      [2, "Gate"],
    ]);
    advanced.append(extra);
    const resets = document.createElement("div");
    resets.className = "neuro-stage-tools";
    for (const [field, text] of [
      [47, "Restart track"],
      [48, "Reset accumulator"],
    ]) {
      const button = document.createElement("button");
      button.textContent = text;
      button.onclick = () =>
        this.hooks.edit({ track: track.id, stage: null, field, value: 1 });
      resets.append(button);
    }
    advanced.append(resets);
    editor.append(advanced);
  }
  update(view, playing) {
    if (!view) return;
    const tracks = view.tracks.slice(0, view.count),
      active = this.program().enabled && playing;
    const selected = tracks.find((t) => t.id === this.selected);
    for (const row of this.root.querySelectorAll(".neuro-track")) {
      const t = tracks.find((t) => t.id === Number(row.dataset.track));
      row.classList.toggle("is-gated", active && !!t?.gate);
    }
    this.root
      .querySelectorAll(".neuro-stage")
      .forEach((card, i) =>
        card.classList.toggle("is-current", active && selected?.step === i),
      );
    const out = this.root.querySelector(".neuro-live");
    if (out && selected)
      out.textContent = active
        ? `${this.note(selected.pitch)} · ${selected.value > 0 ? "+" : ""}${selected.value.toFixed(2)}${this.track()?.target.kind === "parameter" ? "" : " st"}`
        : "—";
  }
};
