"use strict";
// Keep large routing menus out of the live DOM until a pointer, keyboard or
// accessibility focus opens them. The selected label remains a native option.
window.SadLazySelect = (() => {
  const models = new WeakMap();
  const option = (value, label) => {
    const node = document.createElement("option");
    node.value = value;
    node.textContent = label;
    return node;
  };
  function render(select, expanded, value = select.value) {
    const model = models.get(select);
    if (!model) { select.value = value; return; }
    model.expanded = expanded;
    if (expanded) {
      const nodes = model.groups.map(({ label, choices }) => {
        if (!label) return choices.map(([v, text]) => option(v, text));
        const group = document.createElement("optgroup");
        group.label = label;
        group.append(...choices.map(([v, text]) => option(v, text)));
        return group;
      }).flat();
      if (!model.labels.has(value)) nodes.push(option(value, `${value} · unavailable`));
      select.replaceChildren(...nodes);
    } else {
      select.replaceChildren(option(value, model.labels.get(value) ?? `${value} · unavailable`));
    }
    select.value = value;
  }
  return {
    configure(select, value, groups) {
      if (!models.has(select)) {
        const expand = () => {
          if (!models.get(select).expanded) render(select, true);
        };
        select.onpointerdown = expand;
        select.onfocus = expand;
        select.onkeydown = expand;
        select.onblur = () => render(select, false);
      }
      models.set(select, { groups, labels: new Map(groups.flatMap(g => g.choices)), expanded: false });
      render(select, document.activeElement === select, String(value));
    },
    setValue(select, value) {
      value = String(value);
      if (select.value !== value) render(select, !!models.get(select)?.expanded, value);
    },
  };
})();

/* Neuropolis stage, pulse, gate and lane editor. Audio runs in metropolix-core. */
"use strict";
window.SadNeuropolis = class {
  constructor(root, catalog, hooks) {
    this.root = root;
    this.catalog = catalog;
    this.hooks = hooks;
    this.selected = null;
    this.page = 0;
    this.knobs = [];
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
  track(id = this.selected) {
    return this.program().tracks.find((t) => t.id === id);
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
  edit(track, stage, field, value, refresh = true) {
    if (!Number.isFinite(value)) return;
    // Polling replaces the preset object without rebuilding unchanged controls.
    // Event handlers must resolve the current lane, never mutate their captured
    // snapshot (which used to make a previous pitch slider jump back).
    const id = typeof track === "object" ? track?.id : track;
    track = id ? this.track(id) : null;
    if (id && !track) return;
    if (!track) {
      if (field === 0) this.program().enabled = !!value;
    } else if (stage !== null)
      track.steps[stage][this.stepFields[field]] = this.boolFields.has(field)
        ? !!value
        : value;
    else if (field < 43) track.config[field] = Math.round(value);
    else if (field >= 49 && field <= 52) {
      track.pitch_oscillators ??= [false, false, false, false];
      track.pitch_oscillators[field - 49] = !!value;
    } else if (field === 53) track.env_gate = !!value;
    else if (field < 47)
      track[
        { 43: "amount", 44: "absolute", 45: "gate_output", 46: "root_note" }[
          field
        ]
      ] = field === 44 || field === 45 ? !!value : value;
    this.hooks.edit({ track: track?.id || 0, stage, field, value });
    if (refresh) this.sync();
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
      "destination1",
      "destination2",
      "destination3",
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
  pitchText(track, value) {
    return track.target.kind === "oscillator"
      ? `${value >= 60 ? "+" : ""}${Math.round(value - 60)} st`
      : this.note(value);
  }
  configValue(track, field) {
    return field < 43
      ? track.config[field]
      : field === 53
        ? !!track.env_gate
        : field >= 49
          ? !!track.pitch_oscillators?.[field - 49]
          : track[
              {
                43: "amount",
                44: "absolute",
                45: "gate_output",
                46: "root_note",
              }[field]
            ];
  }
  targetName(t) {
    return t.target.kind === "main_pitch"
      ? "Main pitch"
      : t.target.kind === "notes"
        ? "Notes → grains + ENV"
        : t.target.kind === "oscillator"
          ? `OSC ${t.target.index + 1} offset`
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
        .map((t) => [t.id, t.name, t.target, t.destinations, t.config[3], t.config[8]]),
      this.selected,
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
        value = this.configValue(track, f);
      if (document.activeElement !== input) {
        if (input.type === "checkbox") input.checked = !!value;
        else input.value = Number(value);
      }
    }
    for (const input of this.root.querySelectorAll("[data-stage]")) {
      const stage = track.steps[Number(input.dataset.stage)],
        field = Number(input.dataset.field),
        raw = stage[this.stepFields[field]],
        value = input.dataset.relative === "true" && raw >= 0 ? raw - 60 : raw;
      if (input.tagName === "BUTTON") {
        if (field === 2) {
          input.textContent = ["Rest", "Single", "Multi", "Hold"][raw];
          input.dataset.mode = raw;
        } else input.setAttribute("aria-pressed", String(!!raw));
      } else if (document.activeElement !== input) {
        if (input.type === "checkbox") input.checked = !!value;
        else
          input.value = input.dataset.optional && raw < 0 ? "" : Number(value);
      }
    }
    for (const update of this.knobs) update();
    const valueLayer = track.target.kind === "parameter" ? "cv" : "pitch";
    this.root.querySelectorAll(".neuro-stage").forEach((card, i) => {
      const step = track.steps[i],
        value = step[valueLayer];
      card.classList.toggle("outside", i >= track.config[1]);
      card.classList.toggle("rest", step.gate === 0 || step.skip);
      card.querySelector(".neuro-value").textContent =
        valueLayer === "pitch"
          ? this.pitchText(track, value)
          : valueLayer === "cv"
            ? `${value > 0 ? "+" : ""}${(value / 50).toFixed(0)}%`
            : typeof value === "boolean"
              ? value
                ? "On"
                : "Off"
              : value;
      card
        .querySelector(".neuro-fader")
        .setAttribute(
          "aria-valuetext",
          card.querySelector(".neuro-value").textContent,
        );
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
      mute.onclick = () =>
        this.edit(t.id, null, 8, 1 - this.track(t.id).config[8]);
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
  stepInput(container, track, stage, field, min, max, label, options = {}) {
    const input = document.createElement("input");
    input.type = "number";
    input.min = min;
    input.max = max;
    input.step = 1;
    input.dataset.stage = stage;
    input.dataset.field = field;
    input.dataset.relative = String(!!options.relative);
    if (options.optional) {
      input.dataset.optional = "true";
      input.placeholder = "—";
    }
    input.setAttribute("aria-label", `Stage ${stage + 1} ${label}`);
    input.oninput = () => {
      if (input.value === "" && !options.optional) return;
      let value =
        input.value === ""
          ? -1
          : Math.round(Math.max(min, Math.min(max, Number(input.value))));
      if (options.relative && input.value !== "") value += 60;
      this.edit(track.id, stage, field, value);
    };
    container.append(input);
    return input;
  }
  stepToggle(container, track, stage, field, text) {
    const button = document.createElement("button");
    button.type = "button";
    button.textContent = text;
    button.dataset.stage = stage;
    button.dataset.field = field;
    button.setAttribute(
      "aria-label",
      `Stage ${stage + 1} ${text.toLowerCase()}`,
    );
    button.onclick = () =>
      this.edit(
        track.id,
        stage,
        field,
        Number(!this.track(track.id).steps[stage][this.stepFields[field]]),
      );
    container.append(button);
    return button;
  }
  rowLabels(grid, names, className = "") {
    const labels = document.createElement("div");
    labels.className = `neuro-row-labels ${className}`;
    labels.setAttribute("aria-hidden", "true");
    for (const name of names) {
      const label = document.createElement("span");
      label.textContent = name;
      labels.append(label);
    }
    grid.append(labels);
  }
  parameterGroups(prefix = "") {
    const preset = this.hooks.read(), groups = new Map();
    for (const spec of this.catalog.parameters) {
      const fx = /^(pre_)?fx(\d+)_/.exec(spec.id);
      if (fx) {
        const slot = (fx[1] ? preset.pre_color_chain : preset.color_chain)[Number(fx[2]) - 1];
        if (!slot || (this.hooks.fxParameters && !this.hooks.fxParameters(slot).includes(spec.id.slice(fx[0].length))))
          continue;
      }
      if (spec.id.startsWith("drum") && !preset.drums) continue;
      const family = fx
        ? `${fx[1] ? "Pre-filter" : "Post-filter"} FX ${fx[2]}`
        : spec.id.startsWith("resynth_") ? "Resynth"
        : spec.id.startsWith("drum") ? "Drums"
        : spec.id.startsWith("env") ? "Envelopes"
        : spec.id.startsWith("lfo") ? "LFO"
        : spec.id.startsWith("arp_") ? "Arpeggiator" : "Main & filters";
      if (!groups.has(family)) {
        groups.set(family, { label: family, choices: [] });
      }
      groups.get(family).choices.push([`${prefix}${spec.id}`, spec.name]);
    }
    return [...groups.values()];
  }
  buildStages(editor, track) {
    const parameter = track.target.kind === "parameter",
      relative = track.target.kind === "oscillator",
      field = parameter ? 12 : 0,
      min = parameter ? -5000 : relative ? -24 : 0,
      max = parameter ? 5000 : relative ? 24 : 127;
    const toolbar = document.createElement("div");
    toolbar.className = "neuro-stage-tools";
    const title = document.createElement("strong");
    title.textContent = relative
      ? "Pitch offset · −24…+24 st"
      : parameter
        ? "Parameter value"
        : "Pitch";
    const fill = document.createElement("button");
    fill.textContent = "Copy step 1 → all";
    fill.onclick = () => {
      const current = this.track(track.id),
        first = { ...current.steps[0] };
      for (let i = 1; i < 16; i++)
        this.stepFields.forEach((key, f) =>
          this.edit(track.id, i, f, Number(first[key]), false),
        );
      this.sync();
    };
    const live = document.createElement("output");
    live.className = "neuro-live";
    live.textContent = "—";
    toolbar.append(title, fill, live);
    editor.append(toolbar);
    const scroll = document.createElement("div");
    scroll.className = "neuro-stage-scroll";
    const grid = document.createElement("div");
    grid.className = "neuro-stages";
    grid.setAttribute("aria-label", "Neuropolis stages");
    this.rowLabels(grid, [
      "Step",
      parameter ? "Value" : relative ? "Offset" : "Note",
      "",
      "Gate",
      "Gate type",
      "Pulses",
      "Ratchet",
      "Slide",
      "Chance %",
      "Velocity",
      "Dest 1",
      "Dest 2",
      "Dest 3",
    ]);
    for (let i = 0; i < 16; i++) {
      const card = document.createElement("div");
      card.className = "neuro-stage";
      const number = document.createElement("button");
      number.className = "neuro-stage-number";
      number.textContent = String(i + 1).padStart(2, "0");
      number.setAttribute("aria-label", `Stage ${i + 1} enabled`);
      number.onclick = () =>
        this.edit(track.id, i, 6, Number(!this.track(track.id).steps[i].skip));
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
      fader.dataset.relative = String(relative);
      fader.setAttribute(
        "aria-label",
        `Stage ${i + 1} ${parameter ? "parameter value" : relative ? "pitch offset" : "pitch"}`,
      );
      fader.oninput = () =>
        this.edit(
          track.id,
          i,
          field,
          Number(fader.value) + (relative ? 60 : 0),
        );
      card.append(number, value, fader);
      const gate = document.createElement("div");
      gate.className = "neuro-gate";
      this.knobs.push(
        this.hooks.knob(
          [
            `sequence_${track.id}_${i}_gate`,
            "",
            (v) => `${Math.round(v * 100)}%`,
            0,
            1,
            `Stage ${i + 1} gate length`,
          ],
          gate,
          {
            register: false,
            get: () => {
              const t = this.track(track.id),
                n = t.steps[i].gate_override;
              return (n < 0 ? t.config[7] : n) / 100;
            },
            set: (v) => this.edit(track.id, i, 9, Math.round(v * 100)),
            reset: () => -0.01,
          },
        ),
      );
      card.append(gate);
      const mode = document.createElement("button");
      mode.className = "neuro-gate-mode";
      mode.dataset.stage = i;
      mode.dataset.field = 2;
      mode.setAttribute("aria-label", `Stage ${i + 1} gate type`);
      mode.title = "Rest / Single / Multi / Hold";
      mode.onclick = () =>
        this.edit(track.id, i, 2, (this.track(track.id).steps[i].gate + 1) % 4);
      card.append(mode);
      for (const [f, key] of [
        [1, "pulses"],
        [3, "ratchet"],
      ]) {
        const slot = document.createElement("div");
        slot.className = "neuro-step-knob";
        this.knobs.push(
          this.hooks.knob(
            [
              `sequence_${track.id}_${i}_${key}`,
              "",
              (v) => String(Math.round(v)),
              1,
              8,
              `Stage ${i + 1} ${key}`,
            ],
            slot,
            {
              register: false,
              step: 1,
              ariaScale: 1,
              get: () => this.track(track.id).steps[i][key],
              set: (v) => this.edit(track.id, i, f, Math.round(v)),
              reset: () => 1,
            },
          ),
        );
        card.append(slot);
      }
      this.stepToggle(card, track, i, 5, "Slide");
      this.stepInput(card, track, i, 4, 0, 100, "probability");
      this.stepInput(card, track, i, 15, 1, 127, "velocity");
      for (let destination = 0; destination < 3; destination++) {
        const key = `destination${destination + 1}`;
        const slot = document.createElement("div");
        slot.className = "neuro-step-knob neuro-destination-knob";
        this.knobs.push(this.hooks.knob(
          [
            `sequence_${track.id}_${i}_${key}`, "",
            (v) => `${v > 0 ? "+" : ""}${Math.round(v * 100)}%`,
            -1, 1, `Stage ${i + 1} destination ${destination + 1} amount`,
          ], slot, {
            register: false,
            step: 0.01,
            get: () => this.track(track.id).steps[i][key] ?? 0,
            set: (v) => this.edit(track.id, i, 16 + destination, v),
            reset: () => 0,
          },
        ));
        card.append(slot);
      }
      grid.append(card);
    }
    scroll.append(grid);
    editor.append(scroll);

    const details = document.createElement("details");
    details.className = "neuro-advanced";
    details.innerHTML = "<summary>Step overrides & accumulation</summary>";
    const advancedScroll = document.createElement("div");
    advancedScroll.className = "neuro-stage-scroll neuro-detail-scroll";
    const advanced = document.createElement("div");
    advanced.className = "neuro-stages neuro-detail-stages";
    this.rowLabels(
      advanced,
      [
        "Step",
        "Accumulate",
        "Pitch override",
        "Chance to",
        "Acc. trigger",
        "CV",
        "CV ramp",
        "CV gate",
      ],
      "neuro-detail-labels",
    );
    for (let i = 0; i < 16; i++) {
      const card = document.createElement("div");
      card.className = "neuro-detail-stage";
      const number = document.createElement("span");
      number.textContent = String(i + 1).padStart(2, "0");
      card.append(number);
      this.stepInput(card, track, i, 7, -28, 28, "accumulate");
      this.stepInput(
        card,
        track,
        i,
        8,
        relative ? -24 : 0,
        relative ? 24 : 127,
        "pitch override",
        { relative, optional: true },
      );
      for (const [f, label, options] of [
        [10, "probability target", ["Gate", "Ratchet"]],
        [11, "accumulate trigger", ["Stage", "Pulse", "Ratchet"]],
        [12, "cv", null],
        [13, "cv ramp", ["Off", "Stage", "Pulse"]],
      ]) {
        if (!options) {
          this.stepInput(card, track, i, f, -5000, 5000, label);
          continue;
        }
        const select = document.createElement("select");
        select.dataset.stage = i;
        select.dataset.field = f;
        select.setAttribute("aria-label", `Stage ${i + 1} ${label}`);
        options.forEach((text, n) => select.append(this.option(n, text)));
        select.onchange = () => this.edit(track.id, i, f, Number(select.value));
        card.append(select);
      }
      this.stepToggle(card, track, i, 14, "Gate");
      advanced.append(card);
    }
    advancedScroll.append(advanced);
    details.append(advancedScroll);
    editor.append(details);
  }
  buildEditor() {
    const editor = this.root.querySelector(".neuro-editor"),
      track = this.track();
    editor.replaceChildren();
    this.knobs = [];
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
      this.track(track.id).name = name.value.trim() || `Track ${track.id}`;
      this.signature = "";
      this.hooks.structure();
      this.sync();
    };
    const target = document.createElement("select");
    target.setAttribute("aria-label", "Track destination");
    window.SadLazySelect.configure(target,
      track.target.kind === "oscillator" ? `osc:${track.target.index}`
        : track.target.kind === "parameter" ? `param:${track.target.parameter}` : track.target.kind,
      [{ label: "Pitch & notes", choices: [
        ["main_pitch", "Main track pitch"], ["notes", "Notes → grains + ENV"],
        ...[0, 1, 2, 3].map(i => [`osc:${i}`, `OSC ${i + 1} pitch offset`]),
      ] }, ...this.parameterGroups("param:")]);
    target.onchange = () => {
      const current = this.track(track.id);
      current.target = target.value.startsWith("osc:")
        ? { kind: "oscillator", index: Number(target.value.slice(4)) }
        : target.value.startsWith("param:")
          ? { kind: "parameter", parameter: target.value.slice(6) }
          : { kind: target.value };
      if (current.target.kind === "oscillator") {
        const shift = 60 - current.root_note;
        current.config[5] = (((current.config[5] + shift) % 12) + 12) % 12;
        current.steps.forEach((step) => {
          step.pitch = Math.max(36, Math.min(84, step.pitch + shift));
          if (step.pitch_override >= 0)
            step.pitch_override = Math.max(
              36,
              Math.min(84, step.pitch_override + shift),
            );
        });
        current.root_note = 60;
      }
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
    if (track.target.kind === "main_pitch") {
      const routing = document.createElement("div");
      routing.className = "neuro-pitch-routing";
      const label = document.createElement("span");
      label.textContent = "Main pitch →";
      routing.append(label);
      for (let i = 0; i < 4; i++) {
        const check = document.createElement("label");
        check.className = "check";
        const input = document.createElement("input");
        input.type = "checkbox";
        input.dataset.config = 49 + i;
        input.setAttribute("aria-label", `Main pitch affects OSC ${i + 1}`);
        input.onchange = () =>
          this.edit(track.id, null, 49 + i, Number(input.checked));
        check.append(input, `OSC ${i + 1}`);
        routing.append(check);
      }
      editor.append(routing);
    }
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
    } else if (track.target.kind !== "oscillator") {
      this.configControl(
        controls,
        "Root note",
        46,
        Array.from({ length: 128 }, (_, i) => [i, this.note(i)]),
      );
      this.configControl(controls, "Depth", 43, null, -1, 1, 0.01);
    } else this.configControl(controls, "Depth", 43, null, -1, 1, 0.01);
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
    else if (track.target.kind === "main_pitch")
      this.configControl(controls, "Gate ENV", 53, null, 0, 1, 1, "checkbox");
    editor.append(controls);
    const destinations = document.createElement("div");
    destinations.className = "neuro-destinations";
    for (let i = 0; i < 3; i++) {
      const label = document.createElement("label");
      label.className = "neuro-control";
      const title = document.createElement("span");
      title.textContent = `DESTINATION ${i + 1}`;
      const select = document.createElement("select");
      select.setAttribute("aria-label", `Track destination ${i + 1}`);
      window.SadLazySelect.configure(select, track.destinations?.[i] || "", [
        { choices: [["", "Unassigned"]] }, ...this.parameterGroups(),
      ]);
      select.onchange = () => {
        const current = this.track(track.id);
        current.destinations ??= [null, null, null];
        current.destinations[i] = select.value || null;
        this.signature = "";
        this.hooks.structure();
        this.sync();
      };
      label.append(title, select);
      destinations.append(label);
    }
    editor.append(destinations);
    this.buildStages(editor, track);
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
        ? this.track()?.target.kind === "oscillator"
          ? `${selected.value >= 0 ? "+" : ""}${selected.value.toFixed(2)} st`
          : `${this.note(selected.pitch)} · ${selected.value > 0 ? "+" : ""}${selected.value.toFixed(2)}${this.track()?.target.kind === "parameter" ? "" : " st"}`
        : "—";
  }
};
