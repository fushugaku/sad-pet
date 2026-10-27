"use strict";
/**
 * A Rack timbre's page: its modules in the order sound runs through them,
 * top to bottom. Each module is the section other timbres show (Synth engine,
 * Filters, an effect rack…) with the rack's own controls in its heading.
 * Every source starts a chain of its own; the chains meet in the Mixer. The
 * modulators, sequencer and keyboard stay below the rack.
 */
(() => {
  const $ = (id) => document.getElementById(id);
  const el = (tag, className, text) => {
    const node = document.createElement(tag);
    if (className) node.className = className;
    if (text !== undefined) node.textContent = text;
    return node;
  };
  const button = (text, label, action, className = "") => {
    const node = el("button", className, text);
    node.type = "button";
    if (label) {
      node.setAttribute("aria-label", label);
      node.title = label;
    }
    if (action) node.onclick = action;
    return node;
  };
  const POWER = '<svg aria-hidden="true"><use href="#power-glyph" /></svg>';

  /** What a Rack can hold, as the module menu lists it. Each at most once. */
  const MODULES = [
    { key: "sample", label: "Sample", group: "Sources",
      hint: "The loaded file, played by the notes" },
    { key: "synth", label: "Synth", group: "Sources",
      hint: "Up to four oscillators per note" },
    { key: "drums", label: "Drums", group: "Sources",
      hint: "Drum machine that plays its own pattern" },
    { key: "sampler", label: "Sampler", group: "Sources",
      hint: "Pads of samples, played by the notes and pad tracks" },
    { key: "vocal", label: "Vocal", group: "Sources",
      hint: "Sings its score with the song transport" },
    { key: "grains", label: "Grain engine", group: "Processing",
      hint: "Grains of the Sample right above, or of the sound above" },
    { key: "filters", label: "Filters", group: "Processing",
      hint: "Filter 1 and 2; per note right under a Synth or Sample" },
    { key: "resynth", label: "Resynth", group: "Processing",
      hint: "16 bands of analysis played by up to four oscillators" },
    { key: "source_fx", label: "Insert FX", group: "Effects", hint: "One effect" },
    { key: "pre_fx", label: "FX A", group: "Effects", hint: "A chain of up to 12 effects" },
    { key: "post_fx", label: "FX B", group: "Effects", hint: "A chain of up to 12 effects" },
    { key: "mixer", label: "Mixer", group: "Routing",
      hint: "Where the chains above meet: level, pan, mute, solo" },
  ];
  const INFO = Object.fromEntries(MODULES.map((m) => [m.key, m]));
  /** The sources in Mixer strip order (rack::STRIP_SOURCES), as the host's
   * meters list them. */
  const SOURCES = ["sample", "synth", "drums", "sampler", "vocal"];
  /** Sources whose settings a removed module leaves for its undo. */
  const KEPT = ["drums", "synth", "sampler", "vocal"];
  const CHAINS = { sample: "Sample", synth: "Synth", drums: "Drums", sampler: "Sampler", vocal: "Vocal", mix: "Mix" };
  /** The section tap (Output row) of each processor. */
  const STAGES = { source_fx: "source", grains: "grains", resynth: "resynth",
    pre_fx: "pre_fx", filters: "filters", post_fx: "post_fx" };
  const STRIP = { level: 1, pan: 0.5, muted: false, solo: false };

  /**
   * Which chain each module processes, as the engine plans it (rack::Plan):
   * a source starts one, the Mixer sums the chains above it, and the notes
   * stay apart from a Synth or Sample through their Filters.
   */
  function plan(modules) {
    const mixer = modules.findIndex((m) => m.module === "mixer");
    let chain = null, perNote = false, previous = null;
    return modules.map((slot, i) => {
      const enabled = slot.enabled !== false;
      const step = { module: slot.module, enabled, chain, perNote: false, readsFile: false, mixed: false };
      if (SOURCES.includes(slot.module)) {
        chain = step.chain = slot.module;
        // Drums, kit and voice are streams with no notes of their own.
        perNote = step.perNote = (slot.module === "sample" || slot.module === "synth") && enabled;
        step.mixed = mixer > i;
      } else if (slot.module === "mixer") {
        chain = step.chain = "mix";
        perNote = false;
      } else if (slot.module === "grains") {
        step.readsFile = enabled && previous?.module === "sample" && previous.enabled !== false;
        perNote = step.perNote = step.readsFile;
      } else if (slot.module === "filters") step.perNote = perNote;
      else perNote = false;
      previous = slot;
      return step;
    });
  }
  /**
   * Gives `parent` the children `nodes`, in order, moving only those out of
   * place. Taking every module out would blur the button just clicked, and
   * Chrome would lay out the short page then and cut its scroll.
   */
  function reorder(parent, nodes) {
    const kept = new Set(nodes);
    for (const child of [...parent.children]) if (!kept.has(child)) child.remove();
    let at = parent.firstElementChild;
    for (const node of nodes) {
      if (node === at) at = node.nextElementSibling;
      else parent.insertBefore(node, at);
    }
  }
  const decibels = (v) => (v < 1e-4 ? "−∞ dB" : `${v >= 1 ? "+" : ""}${(20 * Math.log10(v)).toFixed(1)} dB`.replace("+0.0", "0.0"));
  const panText = (v) => (Math.abs(v - 0.5) < 0.005 ? "C"
    : `${Math.round(Math.abs(v - 0.5) * 200)}${v < 0.5 ? "L" : "R"}`);

  class Rack {
    constructor(hooks) {
      this.hooks = hooks;
      /** Folded modules, per timbre: "id:module". */
      this.folded = new Set();
      /** Drum and Synth settings of removed modules, per timbre, so that
       * adding the module again brings its sound back. */
      this.stash = new Map();
      this.layout = "";
      this.meterLevels = SOURCES.map(() => 0);
      this.area = el("section", "rack-area");
      this.area.id = "rack";
      this.area.setAttribute("aria-label", "Rack");
      this.area.hidden = true;
      this.area.innerHTML = `<div class="rack-toolbar"><div class="rack-heading"><h2>Rack</h2><span class="rack-count"></span></div><p class="rack-lede">Sound runs from the top module down. Each source starts a chain; chains meet in the Mixer.</p><div class="rack-actions"><span class="rack-undo" role="status" hidden><span></span><button type="button">Undo</button></span><button type="button" class="rack-fold-all" aria-pressed="false">Fold all</button><button type="button" class="rack-add light" aria-haspopup="menu" aria-expanded="false">+ Module</button></div></div><div class="rack-empty" hidden><p>An empty rack. Start with a source:</p><div class="rack-empty-sources"></div></div><div class="rack-stack"></div><div class="rack-out" aria-label="Output"></div>`;
      this.count = this.area.querySelector(".rack-count");
      this.stack = this.area.querySelector(".rack-stack");
      this.out = this.area.querySelector(".rack-out");
      this.empty = this.area.querySelector(".rack-empty");
      this.undoBar = this.area.querySelector(".rack-undo");
      this.undoBar.querySelector("button").onclick = () => this.undo?.();
      this.foldAll = this.area.querySelector(".rack-fold-all");
      this.foldAll.onclick = () => this.toggleAll();
      this.addButton = this.area.querySelector(".rack-add");
      this.addButton.onclick = () => this.openMenu(this.addButton, null);
      $("recording-result-slot").after(this.area);

      // The Sample module holds the waveform panel; the Mixer is the rack's own.
      this.sampleCard = this.card("sample", "Sample", "rack-sample");
      this.mixerCard = this.card("mixer", "Mixer", "rack-mixer");
      this.strips = el("div", "mixer-strips rack-strips");
      this.strips.setAttribute("role", "group");
      this.strips.setAttribute("aria-label", "Rack mixer");
      this.mixerCard.append(this.strips);
      const tap = (stage) => document.querySelector(`section[data-tap="${stage}"]`);
      this.sections = {
        sample: this.sampleCard,
        synth: $("synth-engine"),
        drums: $("drum-machine"),
        sampler: $("sampler-editor"),
        vocal: $("vocal-editor"),
        source_fx: tap("source"),
        grains: $("grain-engine"),
        resynth: tap("resynth"),
        pre_fx: tap("pre_fx"),
        filters: tap("filters"),
        post_fx: tap("post_fx"),
        mixer: this.mixerCard,
      };
      // Each section returns to where the page keeps it when a timbre that
      // is not a Rack opens.
      this.homes = new Map();
      // Sampler and Vocal are their editors, where the host plays a Rack's
      // kit or sings its score; elsewhere a saved module shows as a card
      // that says so, and the menu leaves it out.
      this.unavailable = new Set();
      for (const [key, feature] of [["sampler", "sampler"], ["vocal", "rack_vocal"]]) {
        const ready = !!this.sections[key]?.querySelector(":scope > .section-heading h2");
        if (ready && hooks.features?.()?.[feature]) continue;
        this.unavailable.add(key);
        const card = this.card(key, INFO[key].label, "rack-unavailable");
        card.append(el("p", "muted rack-unavailable-note", `A Rack's ${key === "sampler" ? "kit plays" : "voice sings"} in the browser version for now; here this module is silent.`));
        this.sections[key] = card;
      }
      for (const key of SOURCES.filter((k) => !this.unavailable.has(k)))
        this.empty.querySelector(".rack-empty-sources").append(
          button(`+ ${INFO[key].label}`, `Add ${INFO[key].label}`, () => this.add(key, 0)));
      for (const [key, section] of Object.entries(this.sections)) {
        section.dataset.module = key;
        if (section.isConnected) {
          const home = document.createComment(`rack home: ${key}`);
          section.before(home);
          this.homes.set(key, home);
        }
        this.decorate(key);
      }
      this.samplePanel = document.querySelector(".sample-panel");
      this.mix = $("mix").closest("label");
      this.mixHome = this.mix.parentElement;
      // The drum pattern joins the sequencers under the rack.
      this.pattern = el("section", "bank rack-pattern");
      this.pattern.setAttribute("aria-label", "Drum pattern");
      this.pattern.hidden = true;
      this.pattern.innerHTML = `<div class="section-heading"><h2>Drum pattern <span>DXM-088</span></h2><span class="rack-pattern-note muted">The Drums module plays it with the transport; notes play the Synth, Sample and Sampler</span></div><div class="rack-pattern-body"></div>`;
      this.patternBody = this.pattern.querySelector(".rack-pattern-body");
      document.querySelector(".performance-section").before(this.pattern);
      this.menu = el("div", "rack-menu");
      this.menu.setAttribute("role", "menu");
      this.menu.hidden = true;
      document.body.append(this.menu);
      this.closeMenu = this.closeMenu.bind(this);
    }

    /** A section of the rack's own (Sample, Mixer), shaped like the others. */
    card(key, title, className) {
      const section = el("section", `bank ${className}`);
      section.setAttribute("aria-label", title);
      const heading = el("div", "section-heading");
      heading.append(el("h2", "", title));
      section.append(heading);
      return section;
    }

    /** The rack's controls in a module's heading: they show only in a Rack. */
    decorate(key) {
      const section = this.sections[key];
      const heading = section.querySelector(":scope > .section-heading");
      const h2 = heading.querySelector("h2");
      const label = INFO[key].label;
      const lead = el("div", "rack-lead");
      const grip = button("", `Move ${label}`, null, "rack-grip");
      grip.title = `Drag to move ${label} · ↑ ↓ keys move it too`;
      grip.innerHTML = '<span aria-hidden="true"></span>';
      grip.onpointerdown = (e) => this.drag(e, key);
      grip.onkeydown = (e) => {
        const by = { ArrowUp: -1, ArrowDown: 1 }[e.key];
        if (!by) return;
        e.preventDefault();
        this.move(key, by, "grip");
      };
      lead.append(grip, el("span", "rack-dot"));
      heading.prepend(lead);
      const chip = el("span", "rack-chip");
      const summary = el("span", "rack-summary");
      h2.after(chip, summary);
      const spacer = el("span", "rack-spacer");
      (h2.parentElement === heading ? summary : h2.parentElement).after(spacer);
      const tools = el("div", "rack-tools");
      // Resynth keeps its own power button; the Mixer has none.
      const power = key === "resynth" || key === "mixer" ? null
        : button("", `${label} on`, () => this.toggle(key), "power rack-power");
      if (power) {
        power.innerHTML = POWER;
        power.setAttribute("aria-pressed", "true");
      }
      const up = button("↑", `Move ${label} up`, () => this.move(key, -1, "up"), "rack-up");
      const down = button("↓", `Move ${label} down`, () => this.move(key, 1, "down"), "rack-down");
      const fold = button("", `Fold ${label}`, () => this.fold(key), "rack-fold");
      fold.setAttribute("aria-expanded", "true");
      fold.innerHTML = '<span aria-hidden="true"></span>';
      const remove = button("×", `Remove ${label}`, () => this.remove(key), "rack-remove");
      tools.append(...[power, up, down, fold, remove].filter(Boolean));
      heading.append(tools);
      const text = [...h2.childNodes].find((n) => n.nodeType === Node.TEXT_NODE);
      section.rack = { heading, h2, text, original: text?.nodeValue ?? "", chip, summary, power, up, down, fold, remove, grip };
    }

    preset() {
      return this.hooks.read();
    }
    timbre() {
      return this.hooks.context()?.active ?? 0;
    }
    modules() {
      return this.preset()?.rack?.modules ?? [];
    }

    /** Shows the active timbre: its modules in order when it is a Rack. */
    sync() {
      const preset = this.preset();
      const rack = preset?.rack;
      document.body.classList.toggle("rack-mode", !!rack);
      if (!rack) {
        if (this.active) this.leave();
        return;
      }
      this.active = true;
      this.area.hidden = false;
      const steps = plan(rack.modules);
      const layout = JSON.stringify([
        this.timbre(),
        steps.map((s) => [s.module, s.enabled, s.chain, s.perNote, s.readsFile, s.mixed]),
        !!preset.drums,
        !!preset.synth,
      ]);
      if (layout !== this.layout) {
        this.layout = layout;
        this.arrange(steps);
      }
      // The page's own routing shows the waveform again on every sync.
      this.place(steps);
      this.refresh(steps);
    }

    /** Puts the modules in order, with the links that show where sound goes. */
    arrange(steps) {
      const present = new Set(steps.map((s) => s.module));
      for (const [key, section] of Object.entries(this.sections)) {
        if (present.has(key)) continue;
        this.home(key);
        section.classList.add("rack-absent");
      }
      const nodes = [];
      steps.forEach((step, i) => {
        if (i) nodes.push(this.link(steps, i));
        const section = this.sections[step.module];
        section.classList.remove("rack-absent");
        section.classList.add("rack-module");
        section.classList.toggle("rack-idle", step.chain === null);
        section.dataset.chain = step.chain ?? "none";
        nodes.push(section);
      });
      reorder(this.stack, nodes);
      this.empty.hidden = steps.length > 0;
      this.count.textContent = steps.length
        ? `${steps.length} ${steps.length === 1 ? "module" : "modules"}`
        : "";
      this.buildOut(steps);
      this.buildStrips(steps);
    }

    /** Between two modules: the signal going on, a new chain, or chains
     * meeting in the Mixer, and a + to add a module there. */
    link(steps, index) {
      const next = steps[index];
      const previous = steps[index - 1];
      const link = el("div", "rack-link");
      const kind = SOURCES.includes(next.module) ? "start"
        : next.module === "mixer" ? "merge"
        : previous.chain === null ? "none" : "flow";
      link.classList.add(`rack-link-${kind}`);
      link.dataset.chain = kind === "merge" ? "mix" : next.chain ?? "none";
      link.append(el("span", "rack-wire"));
      if (kind === "merge") {
        const chains = steps.filter((s) => s.mixed).map((s) => s.module);
        const label = el("span", "rack-link-label");
        label.append(...chains.map((c) => {
          const dot = el("i", "rack-tag", CHAINS[c]);
          dot.dataset.chain = c;
          return dot;
        }), document.createTextNode(chains.length ? " meet in the Mixer" : "No chain reaches the Mixer yet"));
        link.append(label);
      }
      const insert = button("+", `Add a module between ${INFO[previous.module].label} and ${INFO[next.module].label}`,
        null, "rack-insert");
      insert.onclick = () => this.openMenu(insert, index);
      link.append(insert);
      return link;
    }

    /** Where the rack ends: the chains that reach the output. */
    buildOut(steps) {
      this.out.replaceChildren();
      if (!steps.length) return;
      const reaching = [];
      let mixed = false;
      for (const step of steps) {
        if (SOURCES.includes(step.module) && !step.mixed) reaching.push(step.module);
        if (step.mixed) mixed = true;
      }
      if (mixed) reaching.unshift("mix");
      const last = steps[steps.length - 1];
      const tail = el("div", "rack-link rack-link-flow");
      tail.dataset.chain = last.chain ?? "none";
      tail.append(el("span", "rack-wire"));
      const insert = button("+", "Add a module at the end", null, "rack-insert");
      insert.onclick = () => this.openMenu(insert, steps.length);
      tail.append(insert);
      const label = el("div", "rack-out-label");
      label.append(el("strong", "", "Output"));
      for (const chain of reaching) {
        const tag = el("i", "rack-tag", CHAINS[chain]);
        tag.dataset.chain = chain;
        label.append(tag);
      }
      if (!reaching.length) label.append(el("span", "muted", "nothing reaches it yet"));
      this.out.append(tail, label);
    }

    /** The waveform, Dry/Wet and the drum pattern go where the modules need them. */
    place(steps) {
      const has = (key) => steps.some((s) => s.module === key);
      const grains = steps.find((s) => s.module === "grains");
      const history = grains && grains.enabled && !grains.readsFile;
      const panel = this.samplePanel;
      if (has("sample")) {
        if (panel.parentElement !== this.sampleCard) this.sampleCard.append(panel);
      } else if (history) {
        const heading = this.sections.grains.rack.heading;
        if (heading.nextElementSibling !== panel) heading.after(panel);
      } else if ($("sample-panel-home").nextElementSibling !== panel) {
        // Unused, it waits at home rather than in a module out of the page.
        $("sample-panel-home").after(panel);
      }
      panel.hidden = !has("sample") && !history;
      // Dry/Wet blends the Grain engine's input with its grains.
      if (has("grains")) {
        const actions = this.sections.grains.querySelector(".resynth-actions");
        if (this.mix.parentElement !== actions) {
          actions.prepend(this.mix);
          this.mix.classList.add("rack-mix");
        }
      } else this.restoreMix();
      const drums = this.hooks.drums?.();
      drums?.dock(has("drums") ? this.patternBody : null);
      this.pattern.hidden = !has("drums");
    }

    restoreMix() {
      this.mix.classList.remove("rack-mix");
      if (this.mix.parentElement !== this.mixHome) this.mixHome.prepend(this.mix);
    }

    home(key) {
      const section = this.sections[key];
      const home = this.homes.get(key);
      if (home) {
        if (home.nextSibling !== section) home.after(section);
      } else section.remove();
      section.classList.remove("rack-module", "rack-idle", "rack-folded", "rack-dragging");
      delete section.dataset.chain;
    }

    /** Back to the page every other timbre shows. */
    leave() {
      this.active = false;
      this.layout = "";
      this.area.hidden = true;
      this.pattern.hidden = true;
      this.closeMenu();
      for (const [key, section] of Object.entries(this.sections)) {
        this.home(key);
        section.classList.remove("rack-absent");
        const { text, original } = section.rack;
        if (text) text.nodeValue = original;
      }
      this.restoreMix();
      this.hooks.drums?.()?.dock(null);
      // syncTimbreRouting puts the waveform and Grain engine back home.
      this.hooks.routing?.();
    }

    /** What changes without moving anything: names, power, chips, summaries. */
    refresh(steps) {
      const preset = this.preset();
      const id = this.timbre();
      const count = steps.length;
      steps.forEach((step, index) => {
        const section = this.sections[step.module];
        const r = section.rack;
        if (r.text) r.text.nodeValue = `${INFO[step.module].label} `;
        const on = step.module === "resynth" ? !!preset.resynth?.enabled : step.enabled;
        section.classList.toggle("rack-off", !on);
        if (r.power) {
          r.power.setAttribute("aria-pressed", String(on));
          r.power.setAttribute("aria-label", `${INFO[step.module].label} ${on ? "on" : "off"}`);
          r.power.title = on ? `Switch ${INFO[step.module].label} off` : `Switch ${INFO[step.module].label} on`;
        }
        r.up.disabled = index === 0;
        r.down.disabled = index === count - 1;
        const folded = this.folded.has(`${id}:${step.module}`);
        section.classList.toggle("rack-folded", folded);
        r.fold.setAttribute("aria-expanded", String(!folded));
        r.fold.setAttribute("aria-label", `${folded ? "Unfold" : "Fold"} ${INFO[step.module].label}`);
        r.fold.title = r.fold.getAttribute("aria-label");
        this.setText(r.chip, this.chip(step, steps));
        r.chip.dataset.chain = step.chain ?? "none";
        this.setText(r.summary, folded ? this.summary(step, preset) : "");
      });
      const allFolded = count > 0 && steps.every((s) => this.folded.has(`${id}:${s.module}`));
      this.foldAll.textContent = allFolded ? "Unfold all" : "Fold all";
      this.foldAll.setAttribute("aria-pressed", String(allFolded));
      this.syncStrips();
    }
    setText(node, text) {
      if (node.textContent !== text) node.textContent = text;
    }

    /** Where a module's sound comes from or goes, beside its name. */
    chip(step, steps) {
      if (step.chain === null) return "No sound reaches it — move it under a source";
      if (SOURCES.includes(step.module))
        return step.mixed ? "→ Mixer" : steps.some((s) => s.module === "mixer") ? "→ Output, past the Mixer" : "→ Output";
      if (step.module === "mixer") {
        const n = steps.filter((s) => s.mixed).length;
        return n ? `${n} ${n === 1 ? "chain" : "chains"} → Output` : "No chains yet";
      }
      if (step.module === "grains")
        return step.readsFile ? "Granulates the Sample" : `Granulates the ${CHAINS[step.chain]} chain`;
      if (step.module === "filters")
        return step.perNote ? `Per note · ${CHAINS[step.chain]}` : `${CHAINS[step.chain]} chain`;
      return `${CHAINS[step.chain]} chain`;
    }

    /** A folded module's settings at a glance. */
    summary(step, preset) {
      const effects = (chain) => {
        const names = (chain ?? []).filter((s) => s.enabled !== false)
          .map((s) => this.hooks.effectName?.(s.kind) ?? s.kind);
        return names.length ? names.join(" · ") : "No effects";
      };
      switch (step.module) {
        case "sample":
          return this.hooks.state()?.sample_name ?? "";
        case "synth": {
          const n = preset.synth?.count ?? 1;
          return `${n} ${n === 1 ? "oscillator" : "oscillators"}`;
        }
        case "drums": {
          const tracks = preset.drums?.tracks ?? [];
          const set = tracks.filter((t) => t.steps?.some((s) => s.velocity > 0)).length;
          return `${set} of ${tracks.length} tracks play`;
        }
        case "sampler": {
          const pads = preset.sampler?.pads ?? [];
          const set = pads.filter((p) => p.zones?.length || p.bounce).length;
          return `${set} of ${pads.length} pads hold samples`;
        }
        case "vocal": {
          const n = preset.vocal?.score?.notes?.length ?? 0;
          return `${n} ${n === 1 ? "note" : "notes"}`;
        }
        case "grains":
          return `${Math.round((preset.params.mix ?? 1) * 100)}% wet`;
        case "filters":
          return { single: "Filter 1", serial: "Filter 1 → 2", parallel: "Filter 1 + 2" }[preset.params.filter_routing] ?? "";
        case "resynth":
          return preset.resynth?.enabled ? `${preset.resynth.count ?? 1} oscillators` : "Off";
        case "source_fx":
          return effects(preset.source_color_chain);
        case "pre_fx":
          return effects(preset.pre_color_chain);
        case "post_fx":
          return effects(preset.color_chain);
        case "mixer":
          return plan(preset.rack.modules).filter((s) => s.mixed)
            .map(({ module }) => `${CHAINS[module]} ${decibels(this.strip(module).level)}`).join(" · ");
        default:
          return "";
      }
    }

    strip(source) {
      return { ...STRIP, ...this.preset()?.rack?.strips?.[source] };
    }

    /** The Mixer: a strip per chain that reaches it, in rack order. */
    buildStrips(steps) {
      const sources = steps.filter((s) => s.mixed).map((s) => s.module);
      this.strips.replaceChildren();
      this.stripViews = [];
      if (!sources.length) {
        this.strips.append(el("p", "mixer-empty", "Sources above the Mixer feed it: move one up, or the Mixer down."));
        return;
      }
      for (const source of sources) {
        const chain = [];
        let seen = false;
        for (const step of steps) {
          if (step.module === source) seen = true;
          else if (seen && (SOURCES.includes(step.module) || step.module === "mixer")) break;
          else if (seen) chain.push(INFO[step.module].label);
        }
        this.stripViews.push(this.stripView(source, chain));
      }
    }

    stripView(source, chain) {
      const label = CHAINS[source];
      const box = el("div", "mixer-strip rack-strip");
      box.dataset.chain = source;
      const name = button(label, `Show ${label}`, () => this.reveal(source), "mixer-name");
      const via = el("span", "mixer-kind", chain.length ? chain.join(" · ") : "Straight in");
      via.title = chain.length ? `${label} → ${chain.join(" → ")} → Mixer` : `${label} → Mixer`;
      const body = el("div", "mixer-body");
      const pan = el("div", "rack-pan");
      const id = (field) => `rack_${source}_${field}`;
      const updatePan = this.hooks.knob([id("pan"), "PAN", panText, 0, 1, `${label} pan`], pan, {
        get: () => this.strip(source).pan,
        set: (v) => this.set(source, "pan", v),
        reset: () => 0.5,
      });
      const meter = el("div", "rack-meter");
      meter.setAttribute("role", "meter");
      meter.setAttribute("aria-label", `${label} level`);
      meter.setAttribute("aria-valuemin", "0");
      meter.setAttribute("aria-valuemax", "100");
      const fill = el("i");
      meter.append(fill);
      const fader = el("label", "mixer-fader");
      const range = el("input");
      Object.assign(range, { type: "range", min: "0", max: "2", step: ".01" });
      range.setAttribute("aria-label", `${label} level into the Mixer`);
      range.oninput = () => this.set(source, "level", Number(range.value));
      range.ondblclick = () => {
        range.value = "1";
        this.set(source, "level", 1);
      };
      this.hooks.slider?.(range, id("level"), `${label} level`);
      const value = el("output", "mixer-level");
      fader.append(range, value);
      body.append(pan, meter, fader);
      const buttons = el("div", "mixer-buttons");
      const mute = button("M", `Mute ${label}`, () => this.set(source, "mute", !this.strip(source).muted));
      const solo = button("S", `Solo ${label}`, () => this.set(source, "solo", !this.strip(source).solo));
      buttons.append(mute, solo);
      box.append(name, via, body, buttons);
      this.strips.append(box);
      let shown = "";
      return {
        source,
        fill,
        meter,
        sync: () => {
          const strip = this.strip(source);
          const soloing = SOURCES.some((s) => this.strip(s).solo && this.stripViews.some((v) => v.source === s));
          const signature = JSON.stringify([strip, soloing]);
          if (signature === shown) return;
          shown = signature;
          if (document.activeElement !== range) range.value = strip.level;
          value.textContent = decibels(strip.level);
          if (document.activeElement !== pan.querySelector(".dial")) updatePan();
          mute.setAttribute("aria-pressed", String(!!strip.muted));
          solo.setAttribute("aria-pressed", String(!!strip.solo));
          box.classList.toggle("idle", !!strip.muted || (soloing && !strip.solo));
        },
      };
    }

    syncStrips() {
      for (const view of this.stripViews ?? []) view.sync();
    }

    /** Each chain's level after its strip, from the host's latest peaks. */
    meters(peaks) {
      if (!this.active || !peaks) return;
      for (const view of this.stripViews ?? []) {
        const index = SOURCES.indexOf(view.source);
        const peak = Math.min(1, peaks[index] ?? 0);
        // Falls back slowly, as the output meters do.
        const level = Math.max(peak, this.meterLevels[index] * 0.82);
        this.meterLevels[index] = level;
        view.fill.style.height = `${level * 100}%`;
        view.meter.setAttribute("aria-valuenow", String(Math.round(level * 100)));
      }
    }

    /** A strip control: a host parameter, so it is smooth, automatable and
     * MIDI-learnable, and modulation can move it. */
    set(source, field, value) {
      const rack = this.preset()?.rack;
      if (!rack) return;
      rack.strips ??= {};
      const strip = (rack.strips[source] = this.strip(source));
      const number = Number(value);
      if (field === "mute") strip.muted = !!value;
      else if (field === "solo") strip.solo = !!value;
      else strip[field] = number;
      this.hooks.parameter(`rack_${source}_${field}`, number);
      this.syncStrips();
      this.refresh(plan(rack.modules));
    }

    /** Edits the module list, then writes the preset; `reload` takes the
     * host's answer back, with the settings a new source starts from. */
    async edit(change, { reload = false, focus = null } = {}) {
      const rack = this.preset()?.rack;
      if (!rack) return;
      // The control in use stays under the pointer and keeps focus while its
      // module stays: the next click lands where the last one did.
      const active = this.stack.contains(document.activeElement) ? document.activeElement : null;
      const top = active?.getBoundingClientRect().top;
      change(rack.modules);
      this.hooks.structure();
      if (reload) await this.hooks.reload();
      else this.hooks.sync();
      if (active && this.stack.contains(active)) {
        window.scrollBy(0, active.getBoundingClientRect().top - top);
        if (document.activeElement !== active) active.focus({ preventScroll: true });
      }
      if (focus) requestAnimationFrame(focus);
    }

    toggle(key) {
      this.edit((modules) => {
        const slot = modules.find((m) => m.module === key);
        if (slot) slot.enabled = slot.enabled === false;
      });
    }

    move(key, by, control) {
      const modules = this.modules();
      const from = modules.findIndex((m) => m.module === key);
      const to = from + by;
      if (from < 0 || to < 0 || to >= modules.length) return;
      const section = this.sections[key];
      const top = section.getBoundingClientRect().top;
      this.edit((list) => list.splice(to, 0, ...list.splice(from, 1)), {
        focus: () => {
          // The module stays where the eye was; its button keeps focus.
          window.scrollBy(0, section.getBoundingClientRect().top - top);
          const target = section.rack[control];
          (target && !target.disabled ? target : section.rack.grip).focus({ preventScroll: true });
        },
      });
    }

    fold(key) {
      const id = `${this.timbre()}:${key}`;
      if (!this.folded.delete(id)) this.folded.add(id);
      this.refresh(plan(this.modules()));
    }

    toggleAll() {
      const id = this.timbre();
      const keys = this.modules().map((m) => `${id}:${m.module}`);
      const fold = !keys.every((k) => this.folded.has(k));
      for (const k of keys) fold ? this.folded.add(k) : this.folded.delete(k);
      this.refresh(plan(this.modules()));
    }

    /** Adds `key` at `index` (null: where it usually goes). */
    add(key, index = null) {
      const modules = this.modules();
      if (modules.some((m) => m.module === key)) return this.reveal(key);
      if (index === null) {
        const mixer = modules.findIndex((m) => m.module === "mixer");
        // A new source feeds the Mixer; anything else goes to the end.
        index = SOURCES.includes(key) && mixer >= 0 ? mixer : modules.length;
      }
      const preset = this.preset();
      const stash = this.stash.get(this.timbre());
      if (KEPT.includes(key) && stash?.[key]) preset[key] = stash[key];
      const source = SOURCES.includes(key) && key !== "sample";
      this.hideUndo();
      this.edit((list) => list.splice(index, 0, { module: key, enabled: true }), {
        reload: source,
        focus: () => this.reveal(key, true),
      });
    }

    remove(key) {
      const modules = this.modules();
      const index = modules.findIndex((m) => m.module === key);
      if (index < 0) return;
      const preset = this.preset();
      const slot = modules[index];
      const timbre = this.timbre();
      const stash = this.stash.get(timbre) ?? {};
      const kept = KEPT.includes(key);
      if (kept) stash[key] = structuredClone(preset[key]);
      this.stash.set(timbre, stash);
      const neighbor = this.sections[(modules[index + 1] ?? modules[index - 1])?.module];
      this.edit((list) => list.splice(index, 1), {
        reload: kept,
        // Keys go on from the module that takes its place.
        focus: () => neighbor?.rack.grip.focus({ preventScroll: true }),
      });
      this.showUndo(`${INFO[key].label} removed`, () => {
        if (this.timbre() !== timbre || this.modules().some((m) => m.module === key)) return;
        if (kept) this.preset()[key] = stash[key];
        this.edit((list) => list.splice(Math.min(index, list.length), 0, slot), {
          reload: kept,
          focus: () => this.reveal(key, true),
        });
      });
    }

    showUndo(text, action) {
      clearTimeout(this.undoTimer);
      this.undo = () => {
        this.hideUndo();
        action();
      };
      this.undoBar.querySelector("span").textContent = text;
      this.undoBar.hidden = false;
      this.undoTimer = setTimeout(() => this.hideUndo(), 10_000);
    }
    hideUndo() {
      clearTimeout(this.undoTimer);
      this.undo = null;
      this.undoBar.hidden = true;
    }

    /** Scrolls to a module, unfolding it; `flash` marks it as new. */
    reveal(key, flash = false) {
      const section = this.sections[key];
      if (!section?.isConnected || section.classList.contains("rack-absent")) return;
      this.folded.delete(`${this.timbre()}:${key}`);
      this.refresh(plan(this.modules()));
      section.scrollIntoView({ behavior: "smooth", block: "start" });
      if (flash) {
        section.classList.remove("rack-new");
        void section.offsetWidth;
        section.classList.add("rack-new");
        // Moving the module would play the flash again.
        section.addEventListener("animationend", () => section.classList.remove("rack-new"), { once: true });
      }
      section.rack.grip.focus({ preventScroll: true });
    }

    /** The module menu, from + Module (`index` null) or a + between modules. */
    openMenu(anchor, index) {
      if (!this.menu.hidden && this.menuAnchor === anchor) return this.closeMenu();
      this.closeMenu();
      this.menuAnchor = anchor;
      anchor.setAttribute("aria-expanded", "true");
      const present = new Set(this.modules().map((m) => m.module));
      this.menu.replaceChildren();
      let group = "";
      for (const module of MODULES.filter((m) => !this.unavailable.has(m.key))) {
        if (module.group !== group) {
          group = module.group;
          this.menu.append(el("div", "rack-menu-group", group));
        }
        const item = el("button", "rack-menu-item");
        item.type = "button";
        item.setAttribute("role", "menuitem");
        item.dataset.module = module.key;
        if (SOURCES.includes(module.key) || module.key === "mixer") item.dataset.chain = module.key === "mixer" ? "mix" : module.key;
        const name = el("strong", "", module.label);
        const hint = el("span", "", present.has(module.key) ? "In the rack — show it" : module.hint);
        item.append(el("i", "rack-menu-dot"), name, hint);
        item.classList.toggle("present", present.has(module.key));
        item.onclick = () => {
          this.closeMenu();
          this.add(module.key, index);
        };
        this.menu.append(item);
      }
      this.menu.hidden = false;
      const box = anchor.getBoundingClientRect();
      const width = this.menu.offsetWidth, height = this.menu.offsetHeight;
      const below = box.bottom + 6 + height <= window.innerHeight || box.top - 6 - height < 0;
      this.menu.style.left = `${Math.max(8, Math.min(box.left + box.width / 2 - width / 2, window.innerWidth - width - 8))}px`;
      this.menu.style.top = `${below ? box.bottom + 6 : box.top - 6 - height}px`;
      (this.menu.querySelector(".rack-menu-item:not(.present)") ?? this.menu.querySelector(".rack-menu-item"))?.focus();
      this.menu.onkeydown = (e) => {
        const items = [...this.menu.querySelectorAll(".rack-menu-item")];
        const at = items.indexOf(document.activeElement);
        const next = { ArrowDown: at + 1, ArrowUp: at - 1, Home: 0, End: items.length - 1 }[e.key];
        if (e.key === "Escape" || e.key === "Tab") {
          e.preventDefault();
          this.closeMenu(true);
        } else if (next !== undefined) {
          e.preventDefault();
          items[(next + items.length) % items.length].focus();
        }
      };
      setTimeout(() => {
        document.addEventListener("pointerdown", this.closeMenu, true);
        window.addEventListener("scroll", this.closeMenu, { passive: true, once: true });
      });
    }
    closeMenu(event) {
      if (event?.type === "pointerdown" && this.menu.contains(event.target)) return;
      if (event?.type === "pointerdown" && this.menuAnchor?.contains(event.target)) return;
      document.removeEventListener("pointerdown", this.closeMenu, true);
      window.removeEventListener("scroll", this.closeMenu);
      if (this.menu.hidden) return;
      this.menu.hidden = true;
      this.menuAnchor?.setAttribute("aria-expanded", "false");
      if (event === true) this.menuAnchor?.focus();
      this.menuAnchor = null;
    }

    /**
     * Drag a module by its grip: every module folds to its heading, so the
     * whole rack fits on screen, and the module follows the pointer.
     */
    drag(event, key) {
      if (event.button !== 0) return;
      event.preventDefault();
      const grip = event.currentTarget;
      const section = this.sections[key];
      const order = this.modules().map((m) => m.module);
      const before = section.getBoundingClientRect().top;
      this.stack.classList.add("rack-arranging");
      section.classList.add("rack-dragging");
      // Folding moves everything; keep the grabbed heading under the pointer.
      window.scrollBy(0, section.getBoundingClientRect().top - before);
      let pointer = event.clientY;
      let frame = 0;
      const modules = () => [...this.stack.querySelectorAll(":scope > .rack-module")];
      const enabled = new Map(this.modules().map((m) => [m.module, m.enabled !== false]));
      const place = () => {
        const others = modules().filter((m) => m !== section);
        const target = others.find((m) => {
          const box = m.getBoundingClientRect();
          return pointer < box.top + box.height / 2;
        });
        if (target) {
          if (section.nextElementSibling !== target) this.stack.insertBefore(section, target);
        } else if (this.stack.lastElementChild !== section) this.stack.append(section);
        // Each module takes the color of the chain it would join here.
        const now = modules();
        plan(now.map((m) => ({ module: m.dataset.module, enabled: enabled.get(m.dataset.module) })))
          .forEach((step, i) => {
            now[i].dataset.chain = step.chain ?? "none";
            now[i].classList.toggle("rack-idle", step.chain === null);
          });
      };
      const scroll = () => {
        const edge = 60;
        const speed = pointer < edge ? -(edge - pointer) : pointer > window.innerHeight - edge
          ? pointer - (window.innerHeight - edge) : 0;
        if (speed) {
          window.scrollBy(0, speed * 0.4);
          place();
        }
        frame = requestAnimationFrame(scroll);
      };
      frame = requestAnimationFrame(scroll);
      let done = false;
      const finish = (commit) => {
        if (done) return;
        done = true;
        cancelAnimationFrame(frame);
        window.removeEventListener("pointermove", move, true);
        window.removeEventListener("pointerup", up, true);
        window.removeEventListener("pointercancel", abort, true);
        window.removeEventListener("blur", abort);
        document.removeEventListener("keydown", cancel, true);
        const next = modules().map((m) => m.dataset.module);
        const top = section.getBoundingClientRect().top;
        this.stack.classList.remove("rack-arranging");
        section.classList.remove("rack-dragging");
        if (commit && next.join() !== order.join()) {
          const slots = new Map(this.modules().map((m) => [m.module, m]));
          this.edit((list) => list.splice(0, list.length, ...next.map((k) => slots.get(k))));
        } else {
          this.layout = "";
          this.sync();
        }
        // Unfolding moves everything back; the dropped module stays put.
        window.scrollBy(0, section.getBoundingClientRect().top - top);
        grip.focus({ preventScroll: true });
      };
      const cancel = (e) => {
        if (e.key !== "Escape") return;
        e.preventDefault();
        e.stopPropagation();
        finish(false);
      };
      // The window hears the pointer wherever it goes until it is released.
      const move = (e) => {
        pointer = e.clientY;
        place();
      };
      const up = (e) => {
        pointer = e.clientY;
        place();
        finish(true);
      };
      const abort = () => finish(false);
      document.addEventListener("keydown", cancel, true);
      window.addEventListener("pointermove", move, true);
      window.addEventListener("pointerup", up, true);
      window.addEventListener("pointercancel", abort, true);
      window.addEventListener("blur", abort);
    }
  }
  Rack.plan = plan;
  Rack.reorder = reorder;
  Rack.modules = MODULES;
  window.SadRack = Rack;
})();
