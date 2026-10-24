"use strict";
// Routing and effect pickers: a button that opens a cascading section → target
// menu, like a macOS plug-in menu. Menu nodes exist only while it is open, so a
// closed picker is one button however many targets it routes to. The button
// keeps a <select>'s value and fires the same input and change events.
window.SadLazySelect = (() => {
  const models = new WeakMap();
  let menu = null,
    pressed = null;
  // [value, text, label?] is a target (label names it on the closed button);
  // { label, choices } is a section.
  const entry = (choice) =>
    Array.isArray(choice)
      ? { value: String(choice[0]), text: choice[1], label: choice[2] ?? choice[1] }
      : { text: choice.label, items: entries(choice.choices) };
  const entries = (choices) =>
    choices.map(entry).filter((e) => !e.items || e.items.length);
  const holds = (e, value) =>
    e.items ? e.items.some((i) => holds(i, value)) : e.value === value;
  function labels(items, map = new Map()) {
    for (const e of items) e.items ? labels(e.items, map) : map.set(e.value, e.label);
    return map;
  }
  function render(anchor, value) {
    const label = models.get(anchor)?.labels.get(value) ?? `${value} · unavailable`;
    anchor.value = value;
    anchor.textContent = label;
    anchor.title = label;
  }
  function show(anchor) {
    hide(false);
    const layer = document.createElement("div"),
      view = viewport();
    // Phones drill into a section in place instead of flying out.
    menu = {
      anchor, layer, levels: [], drill: view.right - view.left < 600,
      timer: 0, pointer: null, hovered: null, typed: "", typedAt: 0,
    };
    layer.className = menu.drill ? "sad-menu-layer is-drill" : "sad-menu-layer";
    layer.onkeydown = key;
    layer.onclick = click;
    layer.onpointermove = hover;
    layer.onpointerleave = () => clearTimeout(menu?.timer);
    layer.addEventListener("focusout", blurred);
    document.body.append(layer);
    anchor.setAttribute("aria-expanded", "true");
    document.addEventListener("pointerdown", outside, true);
    window.addEventListener("scroll", scrolled, true);
    window.addEventListener("resize", dismiss);
    window.addEventListener("blur", dismiss);
    const root = level(models.get(anchor).items, anchor.getAttribute("aria-label") || "", 0);
    menu.levels.push(root);
    layer.append(root.list);
    place(root.list);
    focus(0, start(root));
  }
  function hide(refocus) {
    if (!menu) return;
    const { anchor, layer, timer } = menu;
    menu = null;
    clearTimeout(timer);
    layer.remove();
    anchor.setAttribute("aria-expanded", "false");
    document.removeEventListener("pointerdown", outside, true);
    window.removeEventListener("scroll", scrolled, true);
    window.removeEventListener("resize", dismiss);
    window.removeEventListener("blur", dismiss);
    if (refocus && anchor.isConnected) anchor.focus({ preventScroll: true });
  }
  const dismiss = () => hide(false);
  function outside(event) {
    if (!menu.layer.contains(event.target) && !menu.anchor.contains(event.target)) hide(false);
  }
  function scrolled(event) {
    if (!menu.layer.contains(event.target)) hide(false);
  }
  function blurred() {
    setTimeout(() => {
      const active = document.activeElement;
      if (menu && !menu.layer.contains(active) && active !== menu.anchor) hide(false);
    });
  }
  function level(items, title, depth) {
    const rows = menu.drill && depth ? [{ back: true, text: title }, ...items] : items,
      list = document.createElement("div");
    list.className = "sad-menu";
    list.tabIndex = -1;
    list.setAttribute("role", "menu");
    list.setAttribute("aria-label", title);
    list.append(...rows.map(row));
    return { list, rows, active: -1, from: -1 };
  }
  function row(e, index) {
    const node = document.createElement("div"),
      text = document.createElement("span"),
      value = menu.anchor.value;
    node.className = e.back ? "sad-menu-item sad-menu-back" : "sad-menu-item";
    node.tabIndex = -1;
    node.dataset.index = index;
    node.setAttribute("role", e.items || e.back ? "menuitem" : "menuitemradio");
    if (e.items) {
      node.setAttribute("aria-haspopup", "menu");
      node.setAttribute("aria-expanded", "false");
    } else if (!e.back) node.setAttribute("aria-checked", String(e.value === value));
    if (!e.back && holds(e, value)) node.classList.add("is-current");
    text.textContent = e.text;
    node.append(text);
    return node;
  }
  function start({ rows }) {
    const i = rows.findIndex((e) => !e.back && holds(e, menu.anchor.value));
    return i >= 0 ? i : rows[0]?.back ? 1 : 0;
  }
  // The visible page: inside its scrollbars and, on a zoomed phone, the
  // zoomed-in part.
  function viewport() {
    const v = window.visualViewport,
      root = document.documentElement;
    return v
      ? { left: v.offsetLeft, top: v.offsetTop, right: v.offsetLeft + v.width, bottom: v.offsetTop + v.height }
      : { left: 0, top: 0, right: root.clientWidth, bottom: root.clientHeight };
  }
  // Keep a menu on screen: below or above its button, or beside its section
  // row, flipped left when the right side has no room.
  function place(list, from) {
    const gap = 8,
      view = viewport(),
      anchor = menu.anchor.getBoundingClientRect(),
      style = list.style;
    style.left = style.top = "0px";
    style.maxHeight = `${view.bottom - view.top - 2 * gap}px`;
    if (menu.drill) style.width = `${Math.min(view.right - view.left - 32, Math.max(anchor.width, 280))}px`;
    else if (!from) style.minWidth = `max(180px, ${anchor.width}px)`;
    const box = list.getBoundingClientRect(),
      right = view.right - gap - box.width,
      bottom = view.bottom - gap - box.height;
    let left, top;
    if (from) {
      const item = from.getBoundingClientRect(),
        side = from.parentNode.getBoundingClientRect();
      left = side.right - 3;
      if (left > right) left = side.left - box.width + 3;
      if (left < view.left + gap) left = Math.max(view.left + gap, right);
      top = Math.max(view.top + gap, Math.min(item.top - 5, bottom));
    } else {
      const edge = menu.drill ? 16 : gap,
        below = view.bottom - gap - anchor.bottom - 4,
        above = anchor.top - 4 - gap - view.top;
      left = Math.max(view.left + edge, Math.min(anchor.left, view.right - edge - box.width));
      if (box.height <= below || below >= above) {
        top = anchor.bottom + 4;
        if (box.height > below) style.maxHeight = `${Math.max(below, 0)}px`;
      } else {
        top = Math.max(view.top + gap, anchor.top - 4 - box.height);
        if (box.height > above) style.maxHeight = `${above}px`;
      }
    }
    style.left = `${left}px`;
    style.top = `${top}px`;
  }
  function mark(level, index) {
    level.list.children[level.active]?.classList.remove("is-active");
    level.active = index;
    level.list.children[index].classList.add("is-active");
  }
  function focus(depth, index) {
    const level = menu.levels[depth],
      count = level.rows.length,
      list = level.list;
    if (!count) return;
    mark(level, ((index % count) + count) % count);
    // The path to an open submenu stays lit, as in a macOS menu.
    for (let d = depth; d > 0; d--) mark(menu.levels[d - 1], menu.levels[d].from);
    const node = list.children[level.active];
    node.focus({ preventScroll: true });
    if (node.offsetTop < list.scrollTop) list.scrollTop = node.offsetTop - 4;
    else if (node.offsetTop + node.offsetHeight > list.scrollTop + list.clientHeight)
      list.scrollTop = node.offsetTop + node.offsetHeight - list.clientHeight + 4;
  }
  function expand(depth, index, enter) {
    const parent = menu.levels[depth],
      e = parent.rows[index];
    if (!e?.items) return;
    if (menu.levels[depth + 1]?.from !== index) {
      truncate(depth);
      const next = level(e.items, e.text, depth + 1),
        from = parent.list.children[index];
      next.from = index;
      menu.levels.push(next);
      from.setAttribute("aria-expanded", "true");
      if (menu.drill) parent.list.replaceWith(next.list);
      else menu.layer.append(next.list);
      place(next.list, menu.drill ? null : from);
    }
    if (enter || menu.drill) focus(depth + 1, start(menu.levels[depth + 1]));
  }
  // Close every submenu below depth.
  function truncate(depth) {
    const levels = menu.levels;
    if (levels.length <= depth + 1) return;
    while (levels.length > depth + 1) {
      const gone = levels.pop(),
        parent = levels[levels.length - 1];
      parent.list.children[gone.from].setAttribute("aria-expanded", "false");
      if (menu.drill) gone.list.replaceWith(parent.list);
      else gone.list.remove();
    }
    if (menu.drill) place(levels[depth].list);
  }
  function back(depth) {
    if (!depth) return;
    const from = menu.levels[depth].from;
    truncate(depth - 1);
    focus(depth - 1, from);
  }
  function activate(depth, index, enter) {
    const e = menu.levels[depth].rows[index];
    clearTimeout(menu.timer);
    if (e.back) back(depth);
    else if (e.items) expand(depth, index, enter);
    else choose(e.value);
  }
  function choose(value) {
    const anchor = menu.anchor;
    hide(true);
    if (!anchor.isConnected || value === anchor.value) return;
    render(anchor, value);
    for (const type of ["input", "change"])
      anchor.dispatchEvent(new Event(type, { bubbles: true }));
  }
  const depthOf = (node) => menu.levels.findIndex((l) => l.list.contains(node));
  function click(event) {
    const node = event.target.closest(".sad-menu-item");
    if (node) activate(depthOf(node), Number(node.dataset.index), event.detail === 0);
  }
  // Hovering a section opens it after a moment, so a diagonal path into its
  // submenu can cross other rows without switching. A menu that opened under
  // a resting pointer ignores it until it really moves.
  function hover(event) {
    if (menu.drill || event.pointerType === "touch") return;
    const at = `${event.clientX},${event.clientY}`,
      resting = !menu.pointer || menu.pointer === at;
    menu.pointer = at;
    if (resting) return;
    const node = event.target.closest(".sad-menu-item");
    if (!node || node === menu.hovered) return;
    menu.hovered = node;
    const depth = depthOf(node),
      index = Number(node.dataset.index),
      e = menu.levels[depth].rows[index];
    focus(depth, index);
    clearTimeout(menu.timer);
    if (menu.levels[depth + 1]?.from === index || (!e.items && menu.levels.length === depth + 1)) return;
    menu.timer = setTimeout(
      () => menu?.hovered === node && (e.items ? expand(depth, index, false) : truncate(depth)),
      e.items ? 120 : 250,
    );
  }
  function key(event) {
    // Menu keys never reach the editor's keyboard shortcuts.
    event.stopPropagation();
    const depth = Math.max(0, depthOf(event.target)),
      at = menu.levels[depth].active,
      e = menu.levels[depth].rows[at],
      k = event.key;
    if (k.length === 1 && !event.metaKey && !event.ctrlKey && !event.altKey && (k !== " " || Date.now() - menu.typedAt < 700)) {
      find(depth, k);
      event.preventDefault();
      return;
    }
    // Any other key starts a new type-ahead word.
    menu.typedAt = 0;
    if (k === "ArrowDown") focus(depth, at + 1);
    else if (k === "ArrowUp") focus(depth, at < 0 ? -1 : at - 1);
    else if (k === "Home" || k === "PageUp") focus(depth, 0);
    else if (k === "End" || k === "PageDown") focus(depth, -1);
    else if (k === "ArrowRight") e?.items && expand(depth, at, true);
    else if (k === "ArrowLeft") back(depth);
    else if (k === "Enter" || k === " ") e && activate(depth, at, true);
    else if (k === "Escape") hide(true);
    else {
      // Tab leaves from the button, to the next control.
      if (k === "Tab") hide(true);
      return;
    }
    event.preventDefault();
  }
  // Type-ahead within the open level; repeating one letter cycles its rows.
  function find(depth, char) {
    const level = menu.levels[depth],
      rows = level.rows,
      now = Date.now();
    menu.typed = (now - menu.typedAt < 700 ? menu.typed : "") + char.toLowerCase();
    menu.typedAt = now;
    const typed = menu.typed,
      passes = typed.length > 1 ? [[typed, level.active]] : [];
    if ([...typed].every((c) => c === typed[0])) passes.push([typed[0], level.active + 1]);
    for (const [prefix, from] of passes)
      for (let k = 0; k < rows.length; k++) {
        const i = (Math.max(from, 0) + k) % rows.length;
        if (!rows[i].back && rows[i].text.toLowerCase().startsWith(prefix)) return focus(depth, i);
      }
  }
  return {
    configure(anchor, value, groups) {
      if (!models.has(anchor)) {
        anchor.type = "button";
        anchor.classList.add("sad-select");
        anchor.setAttribute("aria-haspopup", "menu");
        anchor.setAttribute("aria-expanded", "false");
        anchor.onpointerdown = () => {
          pressed = menu?.anchor === anchor ? anchor : null;
        };
        // A press on the open picker's button closes its menu.
        anchor.onclick = () => {
          const close = pressed === anchor;
          pressed = null;
          close ? hide(true) : show(anchor);
        };
        anchor.onkeydown = (event) => {
          if (event.key !== "ArrowDown" && event.key !== "ArrowUp") return;
          event.preventDefault();
          show(anchor);
        };
      }
      const items = entries(groups.flatMap((g) => (g.label ? [g] : g.choices)));
      models.set(anchor, { items, labels: labels(items) });
      render(anchor, String(value));
    },
    setValue(anchor, value) {
      value = String(value);
      if (anchor.value !== value) render(anchor, value);
    },
    // A section whose targets drop its name: "LFO 1 rate" reads "Rate" inside
    // LFO 1 and keeps its full name on the closed button.
    section(label, choices, prefix = label) {
      return {
        label,
        choices: choices.map((choice) => {
          if (!Array.isArray(choice)) return choice;
          const [value, text, full = text] = choice,
            rest = text.startsWith(prefix) && /^[\s·]+(.+)$/.exec(text.slice(prefix.length))?.[1];
          return rest ? [value, rest.charAt(0).toUpperCase() + rest.slice(1), full] : choice;
        }),
      };
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
    // Stage indices stay selected across track switches, so a phrase copied
    // from one track pastes into the same steps of another.
    this.selection = new Set();
    this.anchor = null;
    this.clipboard = null;
    document.addEventListener("keydown", (e) => this.shortcut(e));
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
  /** "Pad A01 · Kick" for a Sampler pad's track. */
  padName(id) {
    const pads = this.hooks.read().sampler?.pads || [];
    const index = pads.findIndex((p) => p.id === id);
    if (index < 0) return `Pad ${id}`;
    const label = `${"ABCDEFGH"[Math.floor(index / 16)]}${String((index % 16) + 1).padStart(2, "0")}`;
    return `Pad ${label}${pads[index].name ? ` · ${pads[index].name}` : ""}`;
  }
  targetName(t) {
    if (t.target.kind === "pad") return this.padName(t.target.pad);
    return t.target.kind === "main_pitch"
      ? "Main pitch"
      : t.target.kind === "notes"
        ? this.hooks.read().sampler ? "Notes → pads by key" : "Notes → grains + ENV"
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
    this.syncSelection();
  }
  syncSelection() {
    const selected = this.selection,
      copied = this.clipboard?.steps.filter(Boolean).length ?? 0;
    this.root.querySelectorAll(".neuro-stage, .neuro-detail-stage").forEach((card) =>
      card.classList.toggle("selected", selected.has(Number(card.dataset.pick))),
    );
    for (const bar of this.root.querySelectorAll(".neuro-stage-pick"))
      bar.setAttribute("aria-pressed", String(selected.has(Number(bar.dataset.pick))));
    const copy = this.root.querySelector(".neuro-copy"),
      paste = this.root.querySelector(".neuro-paste"),
      status = this.root.querySelector(".neuro-selection");
    if (!copy) return;
    copy.disabled = !selected.size;
    paste.disabled = !selected.size || !copied;
    const steps = `${copied} ${copied === 1 ? "step" : "steps"} copied`;
    status.textContent = selected.size
      ? `${selected.size === 1 ? "Step" : "Steps"} ${this.stageRanges(selected)}${copied ? ` · ${steps}` : ""}`
      : copied
        ? `${steps} · select where to paste`
        : "Drag over the bars above the steps to select";
  }
  stageRanges(stages) {
    const sorted = [...stages].sort((a, b) => a - b),
      parts = [];
    for (let i = 0; i < sorted.length; i++) {
      const start = sorted[i];
      while (sorted[i + 1] === sorted[i] + 1) i++;
      parts.push(start === sorted[i] ? `${start + 1}` : `${start + 1}–${sorted[i] + 1}`);
    }
    return parts.join(", ");
  }
  select(stages) {
    this.selection = stages;
    this.syncSelection();
  }
  // Click picks one step (again clears it), drag picks a run, Shift extends
  // from the last step picked, and Cmd/Ctrl adds or removes steps.
  pickStart(stage, e) {
    if (e.button > 0) return;
    e.preventDefault();
    const additive = e.metaKey || e.ctrlKey,
      selected = this.selection;
    this.drag = {
      from: e.shiftKey && this.anchor !== null ? this.anchor : stage,
      base: additive ? new Set(selected) : new Set(),
      remove: additive && !e.shiftKey && selected.has(stage),
      clear: !additive && !e.shiftKey && selected.size === 1 && selected.has(stage),
      last: null,
    };
    if (!e.shiftKey) this.anchor = stage;
    // preventDefault keeps focus where it was, which could leave ⌘C in a
    // number field; the bar takes it so the shortcuts reach the sequencer.
    e.currentTarget.focus?.({ preventScroll: true });
    e.currentTarget.setPointerCapture?.(e.pointerId);
    this.pickTo(stage);
  }
  pickTo(stage) {
    const drag = this.drag;
    if (!drag || stage === null || drag.last === stage) return;
    if (drag.last !== null) drag.clear = false;
    drag.last = stage;
    const next = new Set(drag.base);
    for (let i = Math.min(drag.from, stage); i <= Math.max(drag.from, stage); i++)
      drag.remove ? next.delete(i) : next.add(i);
    this.select(next);
  }
  pickEnd() {
    if (this.drag?.clear) this.select(new Set());
    this.drag = null;
  }
  // Only the horizontal position counts while dragging along the bars.
  pickAt(x) {
    let stage = null,
      distance = Infinity;
    for (const bar of this.root.querySelectorAll(".neuro-stage-pick")) {
      const r = bar.getBoundingClientRect(),
        d = x < r.left ? r.left - x : x > r.right ? x - r.right : 0;
      if (d < distance) {
        distance = d;
        stage = Number(bar.dataset.pick);
      }
    }
    return stage;
  }
  pickKey(stage, e) {
    const next = new Set(this.selection);
    if (e.shiftKey && this.anchor !== null)
      for (let i = Math.min(this.anchor, stage); i <= Math.max(this.anchor, stage); i++)
        next.add(i);
    else {
      next.has(stage) ? next.delete(stage) : next.add(stage);
      this.anchor = stage;
    }
    this.select(next);
  }
  copy() {
    const track = this.track(),
      stages = [...this.selection].sort((a, b) => a - b);
    if (!track || !stages.length) return;
    // Gaps between picked steps are kept, and pasting leaves them untouched.
    this.clipboard = {
      steps: Array.from({ length: stages.at(-1) - stages[0] + 1 }, (_, i) =>
        this.selection.has(stages[0] + i) ? { ...track.steps[stages[0] + i] } : null,
      ),
      oscillator: track.target.kind === "oscillator",
      root: track.target.kind === "oscillator" ? 60 : track.root_note,
    };
    this.sync();
  }
  // Each selected run is filled with the copied steps, repeating them when the
  // run is longer; a lone selected step is where the whole copy starts.
  paste() {
    const track = this.track(),
      clip = this.clipboard,
      stages = [...this.selection].sort((a, b) => a - b);
    if (!track || !clip || !stages.length) return;
    const oscillator = track.target.kind === "oscillator",
      shift =
        oscillator === clip.oscillator
          ? 0
          : (oscillator ? 60 : track.root_note) - clip.root,
      [lo, hi] = oscillator ? [36, 84] : [0, 127],
      covered = new Set();
    for (let i = 0; i < stages.length; i++) {
      const start = stages[i];
      while (stages[i + 1] === stages[i] + 1) i++;
      const length = stages[i] > start ? stages[i] - start + 1 : clip.steps.length;
      for (let k = 0; k < length && start + k < 16; k++) {
        const step = clip.steps[k % clip.steps.length],
          stage = start + k;
        covered.add(stage);
        if (!step) continue;
        this.stepFields.forEach((key, f) => {
          let value = Number(step[key]);
          // Oscillator lanes count from 60; note lanes keep the interval to
          // their root, so a copied phrase sounds the same on either kind.
          if (f === 0 || (f === 8 && value >= 0))
            value = Math.max(lo, Math.min(hi, value + shift));
          if (Number(this.track(track.id).steps[stage][key]) !== value)
            this.edit(track.id, stage, f, value, false);
        });
      }
    }
    this.selection = covered;
    this.sync();
  }
  shortcut(e) {
    if (
      !this.selection.size ||
      !this.root.getClientRects().length ||
      (e.target !== document.body && !this.root.contains(e.target)) ||
      e.target.matches(
        "textarea,[contenteditable],input:not([type=range],[type=checkbox],[type=button])",
      )
    )
      return;
    if (e.key === "Escape" && !e.metaKey && !e.ctrlKey && !e.altKey)
      return this.select(new Set());
    if (!(e.metaKey || e.ctrlKey) || e.altKey || e.shiftKey) return;
    // Non-Latin layouts (Russian, Greek…) report their own letter in e.key.
    const key = /^[a-z]$/i.test(e.key)
      ? e.key.toLowerCase()
      : { KeyC: "c", KeyV: "v" }[e.code];
    if (key === "c" && !String(window.getSelection?.() ?? "")) {
      e.preventDefault();
      this.copy();
    } else if (key === "v" && this.clipboard) {
      e.preventDefault();
      this.paste();
    }
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
  // Destination menu sections. Numbered filters, LFOs, envelopes, effect slots,
  // oscillators and drum voices each get a submenu inside their family.
  parameterGroups(prefix = "") {
    const preset = this.hooks.read(), families = new Map(),
      order = ["Main", "Filters", "LFO", "Envelopes", "Arpeggiator", "Pre-filter FX", "Post-filter FX", "Resynth", "Synth engine", "Drums"];
    for (const spec of this.catalog.parameters) {
      const id = spec.id, fx = /^(pre_)?fx(\d+)_/.exec(id);
      let slot = null;
      if (fx) {
        slot = (fx[1] ? preset.pre_color_chain : preset.color_chain)[Number(fx[2]) - 1];
        if (!slot || (this.hooks.fxParameters && !this.hooks.fxParameters(slot).includes(id.slice(fx[0].length))))
          continue;
      }
      if (id.startsWith("drum") && !preset.drums) continue;
      const n = /\d+/.exec(id)?.[0], filter = /^(?:filter|vcf)(\d*)(?:_|$)/.exec(id),
        slotName = fx && `${fx[1] ? "Pre FX" : "FX"} ${fx[2]}`, kind = slot && this.hooks.fxName?.(slot);
      const [family, sub, strip = sub] = fx
        ? [`${fx[1] ? "Pre-filter" : "Post-filter"} FX`, kind ? `${slotName} · ${kind}` : slotName, slotName]
        : id.startsWith("resynth_osc") ? ["Resynth", `OSC ${n}`, `Resynth OSC ${n}`]
        : id.startsWith("resynth_") ? ["Resynth"]
        : id.startsWith("drum") ? ["Drums", spec.name.split(" · ")[0]]
        : id.startsWith("env") ? ["Envelopes", `ENV ${n}`]
        : id.startsWith("lfo") ? ["LFO", `LFO ${n}`]
        : id.startsWith("arp_") ? ["Arpeggiator"]
        : id.startsWith("synth_osc") ? ["Synth engine", `Synth OSC ${n}`]
        : id.startsWith("synth_") ? ["Synth engine"]
        : filter ? ["Filters", `Filter ${filter[1] || 1}`] : ["Main"];
      if (!families.has(family)) families.set(family, { targets: [], subs: new Map() });
      const { targets, subs } = families.get(family), choice = [`${prefix}${id}`, spec.name];
      if (!sub) targets.push(choice);
      else {
        if (!subs.has(sub)) subs.set(sub, { strip, choices: [] });
        subs.get(sub).choices.push(choice);
      }
    }
    const section = window.SadLazySelect.section;
    return [...families]
      .sort(([a], [b]) => order.indexOf(a) - order.indexOf(b))
      .map(([family, { targets, subs }]) => section(family, [
        ...targets, ...[...subs].map(([label, s]) => section(label, s.choices, s.strip)),
      ]));
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
    const copy = document.createElement("button");
    copy.className = "neuro-copy";
    copy.textContent = "Copy";
    copy.title = "Copy the selected steps (⌘C / Ctrl+C)";
    copy.setAttribute("aria-keyshortcuts", "Meta+C Control+C");
    copy.onclick = () => this.copy();
    const paste = document.createElement("button");
    paste.className = "neuro-paste";
    paste.textContent = "Paste";
    paste.title =
      "Paste into the selected steps (⌘V / Ctrl+V). A single selected step is where the copy starts; a longer selection is filled, repeating the copy.";
    paste.setAttribute("aria-keyshortcuts", "Meta+V Control+V");
    paste.onclick = () => this.paste();
    const status = document.createElement("output");
    status.className = "neuro-selection";
    status.setAttribute("aria-live", "polite");
    const live = document.createElement("output");
    live.className = "neuro-live";
    live.textContent = "—";
    toolbar.append(title, fill, copy, paste, status, live);
    editor.append(toolbar);
    const scroll = document.createElement("div");
    scroll.className = "neuro-stage-scroll";
    const grid = document.createElement("div");
    grid.className = "neuro-stages";
    grid.setAttribute("aria-label", "Neuropolis stages");
    this.rowLabels(grid, [
      "Select",
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
      card.dataset.pick = i;
      const pick = document.createElement("button");
      pick.type = "button";
      pick.className = "neuro-stage-pick";
      pick.dataset.pick = i;
      pick.setAttribute("aria-label", `Select stage ${i + 1}`);
      pick.setAttribute("aria-pressed", "false");
      pick.onpointerdown = (e) => this.pickStart(i, e);
      pick.onpointermove = (e) => this.pickTo(this.pickAt(e.clientX));
      pick.onpointerup = pick.onpointercancel = () => this.pickEnd();
      // Pointer picks happen on pointerdown; this is Enter / Space.
      pick.onclick = (e) => {
        if (e.detail === 0) this.pickKey(i, e);
      };
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
      card.append(pick, number, value, fader);
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
      card.dataset.pick = i;
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
    const target = document.createElement("button");
    target.setAttribute("aria-label", "Track destination");
    const pads = this.hooks.read().sampler?.pads || [];
    window.SadLazySelect.configure(target,
      track.target.kind === "oscillator" ? `osc:${track.target.index}`
        : track.target.kind === "pad" ? `pad:${track.target.pad}`
        : track.target.kind === "parameter" ? `param:${track.target.parameter}` : track.target.kind,
      [{ label: "Pitch & notes", choices: [
        ["main_pitch", "Main track pitch"], ["notes", pads.length ? "Notes → pads by key" : "Notes → grains + ENV"],
        ...[0, 1, 2, 3].map(i => [`osc:${i}`, `OSC ${i + 1} pitch offset`]),
      ] },
      // A kit's pads, each played by its own track whatever its keys.
      ...(pads.length ? [{ label: "Sampler pads", choices: pads
        .filter((p, i) => p.zones.length || i < 16)
        .map((p) => [`pad:${p.id}`, this.padName(p.id)]) }] : []),
      ...this.parameterGroups("param:")]);
    target.onchange = () => {
      const current = this.track(track.id);
      current.target = target.value.startsWith("osc:")
        ? { kind: "oscillator", index: Number(target.value.slice(4)) }
        : target.value.startsWith("pad:")
          ? { kind: "pad", pad: Number(target.value.slice(4)) }
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
      const select = document.createElement("button");
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
