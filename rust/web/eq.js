"use strict";
/**
 * A Rack EQ module's editor. The response of its eight bands is drawn over
 * frequency, with a handle per band to drag; under it sit the selected band's
 * controls and the output gain. Every knob is a host parameter, so
 * automation, MIDI learn and modulation reach each band, and the curve
 * follows modulation as it plays. The curve is the engine's own: its sections
 * are designed here exactly as rust/src/dsp/eq.rs designs them.
 */
(() => {
  const BANDS = 8;
  const MIN_FREQUENCY = 20, MAX_FREQUENCY = 20000, MAX_GAIN = 24, MIN_Q = 0.1, MAX_Q = 20;
  const SLOPES = [6, 12, 18, 24, 36, 48];
  /** key, name, short name. */
  const KINDS = [
    ["bell", "Bell", "Bell"],
    ["low_shelf", "Low shelf", "Lo shelf"],
    ["high_shelf", "High shelf", "Hi shelf"],
    ["low_cut", "Low cut", "Lo cut"],
    ["high_cut", "High cut", "Hi cut"],
    ["notch", "Notch", "Notch"],
    ["band_pass", "Band pass", "Band"],
  ];
  const KIND = Object.fromEntries(KINDS.map(([key, name, short]) => [key, { name, short }]));
  /** key, button text, name. */
  const PLACEMENTS = [
    ["stereo", "St", "Stereo"],
    ["left", "L", "Left"],
    ["right", "R", "Right"],
    ["mid", "M", "Mid"],
    ["side", "S", "Side"],
  ];
  /** Low bands warm, high bands cool, as they stand on the graph. */
  const COLORS = ["#ff8f7e", "#ffb470", "#efdc72", "#c4f486", "#78dcc0", "#7fb2ee", "#ab9cf5", "#ee8cc6"];
  const SCALES = [6, 12, 24];
  const SQRT_HALF = Math.SQRT1_2;
  /** The Q each shape starts from. */
  const DEFAULT_Q = { bell: 1, low_shelf: SQRT_HALF, high_shelf: SQRT_HALF, low_cut: SQRT_HALF,
    high_cut: SQRT_HALF, notch: 4, band_pass: 1 };
  const hasGain = (kind) => kind === "bell" || kind === "low_shelf" || kind === "high_shelf";
  const isCut = (kind) => kind === "low_cut" || kind === "high_cut";
  const clamp = (v, lo, hi) => Math.min(hi, Math.max(lo, v));
  const FREQUENCY_SPAN = Math.log(MAX_FREQUENCY / MIN_FREQUENCY);
  const Q_SPAN = Math.log(MAX_Q / MIN_Q);
  /** Frequency and Q are host parameters along their log scales, 0–1. */
  const unitFrequency = (hz) => Math.log(hz / MIN_FREQUENCY) / FREQUENCY_SPAN;
  const frequencyOf = (unit) => MIN_FREQUENCY * Math.exp(clamp(unit, 0, 1) * FREQUENCY_SPAN);
  const unitQ = (q) => Math.log(q / MIN_Q) / Q_SPAN;
  const qOf = (unit) => MIN_Q * Math.exp(clamp(unit, 0, 1) * Q_SPAN);

  const hertz = (f) => (f < 1000 ? `${f < 100 ? f.toFixed(1) : Math.round(f)} Hz`
    : `${(f / 1000).toFixed(f < 10000 ? 2 : 1)} kHz`);
  /** A tab's short frequency: "90 Hz", "2.5 kHz". */
  const brief = (f) => (f < 1000 ? `${Math.round(f)} Hz` : `${+(f / 1000).toFixed(f < 10000 ? 1 : 0)} kHz`);
  const decibels = (g) => `${g > 0.005 ? "+" : g < -0.005 ? "−" : ""}${Math.abs(g).toFixed(1)} dB`;
  const quality = (q) => q.toFixed(q < 10 ? 2 : 1);

  /** The settings a new EQ starts from (rust/src/eq.rs, Equalizer::default). */
  function defaults(id) {
    const band = (kind, frequency, q, enabled) =>
      ({ enabled, kind, frequency, gain: 0, q, slope: 24, placement: "stereo" });
    return {
      id,
      bands: [
        band("low_cut", 30, SQRT_HALF, false),
        band("low_shelf", 90, SQRT_HALF, true),
        band("bell", 250, 1, true),
        band("bell", 800, 1, true),
        band("bell", 2500, 1, true),
        band("bell", 6000, 1, true),
        band("high_shelf", 10000, SQRT_HALF, true),
        band("high_cut", 18000, SQRT_HALF, false),
      ],
      output: 0,
    };
  }

  // The engine's sections. Each takes its poles from the analog prototype
  // and its zeros so that its magnitude matches at 0 Hz, at the band's
  // frequency and at Nyquist.
  const PI = Math.PI;
  const HIGHEST_POLE = 0.98 * PI, HIGHEST_BAND = 0.95 * PI;
  const phis = (w) => {
    const s = Math.sin(w * 0.5), phi1 = s * s, phi0 = 1 - phi1;
    return [phi0, phi1, 4 * phi0 * phi1];
  };
  const squares = ([a1, a2]) => [(1 + a1 + a2) ** 2, (1 - a1 + a2) ** 2, -4 * a2];
  const dot = (a, b) => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
  function poles(w, q) {
    w = Math.min(w, HIGHEST_POLE);
    const zeta = 0.5 / q, r = Math.exp(-zeta * w);
    const a1 = zeta <= 1 ? -2 * r * Math.cos(Math.sqrt(1 - zeta * zeta) * w)
      : -2 * r * Math.cosh(Math.sqrt(zeta * zeta - 1) * w);
    return [a1, r * r];
  }
  function analog(n, d, x) {
    const x2 = x * x;
    return ((n[0] - n[2] * x2) ** 2 + (n[1] * x) ** 2) / ((d[0] - d[2] * x2) ** 2 + (d[1] * x) ** 2);
  }
  function matched(a, w, [h0, hw, hpi]) {
    const sq = squares(a), phi = phis(w);
    const b0 = sq[0] * h0, b1 = sq[1] * hpi;
    const b2 = (dot(sq, phi) * hw - b0 * phi[0] - b1 * phi[1]) / phi[2];
    const s0 = Math.sqrt(b0), s1 = Math.sqrt(b1), half = 0.5 * (s0 + s1);
    const first = 0.5 * (half + Math.sqrt(Math.max(0, half * half + b2)));
    return { b: [first, 0.5 * (s0 - s1), half - first], a };
  }
  const fit = (n, d, w, [wp, qp]) =>
    matched(poles(wp, qp), w, [analog(n, d, 0), analog(n, d, 1), analog(n, d, PI / w)]);
  function bell(w, gain, q) {
    const a = 10 ** (gain / 40);
    return fit([1, a / q, 1], [1, 1 / (a * q), 1], w, [w, a * q]);
  }
  function lowShelf(w, gain, q) {
    const a = 10 ** (gain / 40), root = Math.sqrt(a);
    return fit([a * a, a * root / q, a], [1, root / q, a], w, [w / root, q]);
  }
  function highShelf(w, gain, q) {
    const a = 10 ** (gain / 40), root = Math.sqrt(a);
    return fit([a, a * root / q, a * a], [a, root / q, 1], w, [w * root, q]);
  }
  const bandPass = (w, q) => fit([0, 1 / q, 0], [1, 1 / q, 1], w, [w, q]);
  function notch(w, q) {
    const a = poles(w, q), c = (1 + a[0] + a[1]) / (2 - 2 * Math.cos(w));
    return { b: [c, -2 * c * Math.cos(w), c], a };
  }
  function lowPass(w, q) {
    const a = poles(w, q), sq = squares(a), phi = phis(w);
    const b0 = sq[0], b1 = Math.max(0, (dot(sq, phi) * q * q - b0 * phi[0]) / phi[1]);
    const first = 0.5 * (Math.sqrt(b0) + Math.sqrt(b1));
    return { b: [first, Math.sqrt(b0) - first, 0], a };
  }
  function highPass(w, q) {
    const a = poles(w, q), phi = phis(w);
    const b = Math.sqrt(dot(squares(a), phi)) * q / (4 * phi[1]);
    return { b: [b, -2 * b, b], a };
  }
  function onePole(w, high) {
    w = Math.min(w, HIGHEST_POLE);
    const p = Math.exp(-w), ratio = PI / w;
    if (high) {
      const c = 0.5 * (1 + p) * ratio / Math.sqrt(1 + ratio * ratio);
      return [c, -c, p];
    }
    const h = 1 / Math.sqrt(1 + ratio * ratio);
    return [0.5 * ((1 - p) + (1 + p) * h), 0.5 * ((1 - p) - (1 + p) * h), p];
  }
  /** A band's sections at `rate` Hz; none while it is off. */
  function design(band, rate) {
    const result = { sections: [], pole: null, terms: [] };
    if (!band.enabled) return result;
    const w = Math.min(2 * PI * band.frequency / rate, HIGHEST_BAND);
    const { gain, q } = band;
    switch (band.kind) {
      case "bell": result.sections.push(bell(w, gain, q)); break;
      case "low_shelf": result.sections.push(lowShelf(w, gain, q)); break;
      case "high_shelf": result.sections.push(highShelf(w, gain, q)); break;
      case "notch": result.sections.push(notch(w, q)); break;
      case "band_pass": result.sections.push(bandPass(w, q)); break;
      default: {
        const high = band.kind === "low_cut";
        const order = clamp(Math.floor((band.slope ?? 24) / 6), 1, 8);
        const pairs = Math.floor(order / 2);
        for (let k = 0; k < pairs; k++) {
          const angle = order % 2 === 0 ? (2 * k + 1) * PI / (2 * order) : (k + 1) * PI / order;
          let sectionQ = 0.5 / Math.cos(angle);
          if (k + 1 === pairs) sectionQ *= q / SQRT_HALF;
          result.sections.push(high ? highPass(w, sectionQ) : lowPass(w, sectionQ));
        }
        if (order % 2 === 1) result.pole = onePole(w, high);
      }
    }
    // Each section's numerator terms, ready for evaluation.
    result.terms = result.sections.map(({ b, a }) =>
      [[(b[0] + b[1] + b[2]) ** 2, (b[0] - b[1] + b[2]) ** 2, -4 * b[0] * b[2]], squares(a)]);
    return result;
  }
  /** A designed band's magnitude in dB at `w` radians per sample. */
  function level(designed, w, phi) {
    let power = 1;
    for (const [numerator, denominator] of designed.terms) power *= dot(numerator, phi) / dot(denominator, phi);
    if (designed.pole) {
      const [b0, b1, p] = designed.pole, c = Math.cos(w), s = Math.sin(w);
      power *= ((b0 + b1 * c) ** 2 + (b1 * s) ** 2) / ((1 - p * c) ** 2 + (p * s) ** 2);
    }
    return 10 * Math.log10(Math.max(power, 1e-30));
  }
  /** Each band's response and their sum, in dB, at `frequencies` Hz. */
  function responses(bands, rate, frequencies) {
    const designs = bands.map((band) => design(band, rate));
    const each = designs.map(() => new Float64Array(frequencies.length));
    const total = new Float64Array(frequencies.length);
    frequencies.forEach((f, i) => {
      const w = 2 * PI * f / rate;
      if (w >= PI) {
        each.forEach((curve) => (curve[i] = NaN));
        total[i] = NaN;
        return;
      }
      const phi = phis(w);
      designs.forEach((designed, b) => {
        const db = designed.terms.length || designed.pole ? level(designed, w, phi) : 0;
        each[b][i] = db;
        total[i] += db;
      });
    });
    return { each, total };
  }

  const el = (tag, className, text) => {
    const node = document.createElement(tag);
    if (className) node.className = className;
    if (text !== undefined) node.textContent = text;
    return node;
  };
  const button = (text, label, className = "") => {
    const node = el("button", className, text);
    node.type = "button";
    if (label) {
      node.setAttribute("aria-label", label);
      node.title = label;
    }
    return node;
  };
  const POWER = '<svg aria-hidden="true"><use href="#power-glyph" /></svg>';
  /** Where the graph draws its grid, in Hz. */
  const GRID = [20, 30, 40, 50, 60, 70, 80, 90, 100, 200, 300, 400, 500, 600, 700, 800, 900,
    1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000, 20000];
  const LABELS = [[50, "50"], [100, "100"], [200, "200"], [500, "500"], [1000, "1k"], [2000, "2k"],
    [5000, "5k"], [10000, "10k"]];
  /** Plot margins in CSS pixels: frequency labels under the curve, the dB
   * scale in a column of its own on the right. */
  const TOP = 10, BOTTOM = 20, LEFT = 12, RIGHT = 34;

  class Editor {
    /**
     * `hooks`: read() the EQ's settings (null when the rack has none), rate()
     * of the engine, parameter(id, value), structure() after a change of
     * shape, knob() as app.js makes them, feedback(key) of a modulated knob,
     * label() of the module.
     */
    constructor(section, id, hooks) {
      this.section = section;
      this.id = id;
      this.hooks = hooks;
      this.selected = 2;
      this.scale = 12;
      this.knobs = [];
      this.root = el("div", "eq-editor");
      this.root.style.setProperty("--eq-selected", COLORS[this.selected]);
      this.graph = el("div", "eq-graph");
      this.graph.setAttribute("role", "group");
      this.graph.setAttribute("aria-label", `${hooks.label()} response`);
      this.canvas = el("canvas");
      this.canvas.setAttribute("aria-hidden", "true");
      this.readout = el("output", "eq-readout");
      this.hint = el("span", "eq-hint", "Drag a point · wheel: Q · double-click: band on/off");
      this.scales = el("div", "eq-scale");
      this.scales.setAttribute("role", "group");
      this.scales.setAttribute("aria-label", "Graph range");
      for (const scale of SCALES) {
        const toggle = button(`±${scale}`, `Show ±${scale} dB`);
        toggle.dataset.scale = scale;
        toggle.onclick = () => {
          this.scale = scale;
          this.layout();
        };
        this.scales.append(toggle);
      }
      this.handles = [];
      this.ghosts = [];
      for (let b = 0; b < BANDS; b++) {
        const ghost = el("i", "eq-ghost");
        ghost.style.setProperty("--band", COLORS[b]);
        const handle = button(String(b + 1), "", "eq-handle");
        handle.style.setProperty("--band", COLORS[b]);
        handle.dataset.band = b;
        this.bindHandle(handle, b);
        this.ghosts.push(ghost);
        this.handles.push(handle);
      }
      this.graph.append(this.canvas, ...this.ghosts, ...this.handles, this.readout, this.hint, this.scales);
      this.bindGraph();

      // The bands: a tab each, then the selected band's shape and knobs.
      this.tabs = el("div", "eq-tabs");
      this.tabs.setAttribute("role", "tablist");
      this.tabs.setAttribute("aria-label", `${hooks.label()} bands`);
      this.tabButtons = [];
      for (let b = 0; b < BANDS; b++) {
        const tab = el("button", "eq-tab");
        tab.type = "button";
        tab.setAttribute("role", "tab");
        tab.style.setProperty("--band", COLORS[b]);
        tab.innerHTML = `<i aria-hidden="true"></i><b>${b + 1}</b><span class="eq-tab-kind"></span><span class="eq-tab-place"></span><span class="eq-tab-value"></span>`;
        tab.onclick = () => this.select(b);
        tab.onkeydown = (e) => {
          const to = { ArrowRight: b + 1, ArrowLeft: b - 1, Home: 0, End: BANDS - 1 }[e.key];
          if (to === undefined) return;
          e.preventDefault();
          this.select((to + BANDS) % BANDS);
          this.tabButtons[this.selected].focus();
        };
        this.tabButtons.push(tab);
        this.tabs.append(tab);
      }
      this.panel = el("div", "eq-panel");
      this.panel.setAttribute("role", "tabpanel");
      const head = el("div", "eq-band-head");
      this.power = button("", "", "power eq-band-power");
      this.power.innerHTML = POWER;
      this.power.onclick = () => this.change(this.selected, { enabled: !this.band(this.selected).enabled });
      this.kind = el("select", "eq-kind");
      for (const [key, name] of KINDS) this.kind.append(new Option(name, key));
      this.kind.onchange = () => this.shape(this.selected, { kind: this.kind.value });
      this.slope = el("select", "eq-slope");
      for (const slope of SLOPES) this.slope.append(new Option(`${slope} dB/oct`, String(slope)));
      this.slope.onchange = () => this.shape(this.selected, { slope: Number(this.slope.value) });
      this.placement = el("div", "eq-placement");
      this.placement.setAttribute("role", "group");
      this.placementButtons = PLACEMENTS.map(([key, text, name]) => {
        const toggle = button(text, `${name}: the band filters ${{ stereo: "both channels", left: "the left channel",
          right: "the right channel", mid: "the mid (L+R)", side: "the side (L−R)" }[key]}`);
        toggle.dataset.placement = key;
        toggle.onclick = () => this.shape(this.selected, { placement: key });
        this.placement.append(toggle);
        return toggle;
      });
      this.name = el("span", "eq-band-name");
      head.append(this.power, this.name, this.kind, this.slope, this.placement);
      this.groups = [];
      const knobs = el("div", "eq-knobs");
      for (let b = 0; b < BANDS; b++) {
        const group = el("div", "eq-knob-group");
        group.hidden = b !== this.selected;
        const value = (field) => () => this.band(b)[field];
        const updates = {
          frequency: hooks.knob([this.key(b, "frequency"), "FREQ", (v) => hertz(frequencyOf(v)), 0, 1,
            `${hooks.label()} band ${b + 1} frequency`], group, {
            get: () => unitFrequency(value("frequency")()),
            set: (v) => this.change(b, { frequency: frequencyOf(v) }),
            reset: () => unitFrequency(defaults(id).bands[b].frequency),
          }),
          gain: hooks.knob([this.key(b, "gain"), "GAIN", decibels, -MAX_GAIN, MAX_GAIN,
            `${hooks.label()} band ${b + 1} gain`], group, {
            get: value("gain"),
            set: (v) => this.change(b, { gain: v }),
            reset: () => 0,
            step: 0.1,
          }),
          q: hooks.knob([this.key(b, "q"), "Q", (v) => quality(qOf(v)), 0, 1,
            `${hooks.label()} band ${b + 1} Q`], group, {
            get: () => unitQ(value("q")()),
            set: (v) => this.change(b, { q: qOf(v) }),
            reset: () => unitQ(DEFAULT_Q[this.band(b).kind] ?? 1),
          }),
        };
        this.knobs.push(updates);
        this.groups.push(group);
        knobs.append(group);
      }
      this.panel.append(head, knobs);
      const output = el("div", "eq-output");
      this.outputKnob = hooks.knob([this.key(null, "output"), "OUTPUT", decibels, -MAX_GAIN, MAX_GAIN,
        `${hooks.label()} output`], output, {
        get: () => this.eq()?.output ?? 0,
        set: (v) => {
          const eq = this.eq();
          if (!eq) return;
          eq.output = v;
          this.hooks.parameter(this.key(null, "output"), v);
        },
        reset: () => 0,
        step: 0.1,
      });
      const controls = el("div", "eq-controls");
      controls.append(this.panel, output);
      this.root.append(this.graph, this.tabs, controls);
      section.append(this.root);
      this.size = { width: 0, height: 0 };
      new ResizeObserver(() => this.layout()).observe(this.graph);
    }

    key(band, field) {
      return band === null ? `rack_eq${this.id}_output` : `rack_eq${this.id}_band${band + 1}_${field}`;
    }
    eq() {
      return this.hooks.read();
    }
    band(b) {
      return this.eq()?.bands?.[b] ?? defaults(this.id).bands[b];
    }
    rate() {
      return this.hooks.rate() || 48000;
    }

    /** Shows the settings anew: every control, then the graph. */
    sync() {
      const eq = this.eq();
      if (!eq) return;
      for (let b = 0; b < BANDS; b++) {
        const band = eq.bands[b];
        const tab = this.tabButtons[b];
        tab.classList.toggle("off", !band.enabled);
        tab.setAttribute("aria-selected", String(b === this.selected));
        tab.tabIndex = b === this.selected ? 0 : -1;
        const kind = tab.querySelector(".eq-tab-kind"), value = tab.querySelector(".eq-tab-value");
        kind.textContent = KIND[band.kind]?.short ?? band.kind;
        value.textContent = brief(band.frequency);
        // A band on one channel, the mid or the side says which.
        const place = tab.querySelector(".eq-tab-place");
        place.textContent = band.placement === "stereo" ? ""
          : PLACEMENTS.find(([key]) => key === band.placement)?.[1] ?? "";
        tab.title = `Band ${b + 1}: ${this.describe(band)}`;
        tab.setAttribute("aria-label", tab.title);
      }
      const band = eq.bands[this.selected];
      const number = this.selected + 1;
      this.name.textContent = `Band ${number}`;
      this.kind.setAttribute("aria-label", `Band ${number} shape`);
      this.slope.setAttribute("aria-label", `Band ${number} slope`);
      this.placement.setAttribute("aria-label", `Band ${number} channels`);
      this.power.setAttribute("aria-pressed", String(band.enabled));
      const state = band.enabled ? "on" : "off";
      this.power.setAttribute("aria-label", `Band ${this.selected + 1} ${state}`);
      this.power.title = band.enabled ? `Switch band ${this.selected + 1} off` : `Switch band ${this.selected + 1} on`;
      if (document.activeElement !== this.kind) this.kind.value = band.kind;
      this.slope.hidden = !isCut(band.kind);
      if (document.activeElement !== this.slope) this.slope.value = String(band.slope);
      for (const toggle of this.placementButtons)
        toggle.setAttribute("aria-pressed", String(toggle.dataset.placement === band.placement));
      this.panel.classList.toggle("off", !band.enabled);
      this.panel.setAttribute("aria-label", `Band ${this.selected + 1}`);
      for (let b = 0; b < BANDS; b++) {
        this.groups[b].hidden = b !== this.selected;
        const { kind } = eq.bands[b];
        // A cut, notch or band pass has no gain; a 6 dB/oct cut has no Q.
        this.disable(this.groups[b].children[1], !hasGain(kind));
        this.disable(this.groups[b].children[2], isCut(kind) && eq.bands[b].slope === 6);
      }
      for (const updates of this.knobs) for (const update of Object.values(updates)) update();
      this.outputKnob();
      this.layout();
    }
    disable(knob, off) {
      knob?.classList.toggle("eq-knob-off", off);
      knob?.querySelector(".dial")?.setAttribute("aria-disabled", String(off));
    }
    describe(band) {
      const parts = [KIND[band.kind]?.name ?? band.kind, hertz(band.frequency)];
      if (hasGain(band.kind)) parts.push(decibels(band.gain));
      if (isCut(band.kind)) parts.push(`${band.slope} dB/oct`);
      if (!(isCut(band.kind) && band.slope === 6)) parts.push(`Q ${quality(band.q)}`);
      if (band.placement !== "stereo") parts.push(PLACEMENTS.find(([k]) => k === band.placement)?.[2]);
      if (!band.enabled) parts.push("off");
      return parts.join(", ");
    }
    /** The rack's folded summary: the bands that change anything. */
    summary() {
      const eq = this.eq();
      if (!eq) return "";
      const active = eq.bands.filter((b) => b.enabled && (!hasGain(b.kind) || Math.abs(b.gain) >= 0.05));
      const parts = active.slice(0, 4).map((b) =>
        `${KIND[b.kind].short} ${hertz(b.frequency)}${hasGain(b.kind) ? ` ${decibels(b.gain)}` : ""}`);
      if (active.length > 4) parts.push(`+${active.length - 4} more`);
      if (Math.abs(eq.output) >= 0.05) parts.push(`Out ${decibels(eq.output)}`);
      return parts.length ? parts.join(" · ") : "Flat";
    }

    select(b) {
      this.selected = b;
      this.root.style.setProperty("--eq-selected", COLORS[b]);
      this.sync();
    }

    /**
     * A band's numbers change: frequency, gain, Q or on. Each is a host
     * parameter, sent as its knob sends it.
     */
    change(b, fields) {
      const eq = this.eq();
      if (!eq) return;
      const band = eq.bands[b];
      for (const [field, value] of Object.entries(fields)) {
        if (field === "frequency") {
          band.frequency = clamp(value, MIN_FREQUENCY, MAX_FREQUENCY);
          this.hooks.parameter(this.key(b, "frequency"), unitFrequency(band.frequency));
        } else if (field === "gain") {
          band.gain = clamp(value, -MAX_GAIN, MAX_GAIN);
          this.hooks.parameter(this.key(b, "gain"), band.gain);
        } else if (field === "q") {
          band.q = clamp(value, MIN_Q, MAX_Q);
          this.hooks.parameter(this.key(b, "q"), unitQ(band.q));
        } else if (field === "enabled") {
          band.enabled = !!value;
          this.hooks.parameter(this.key(b, "on"), band.enabled ? 1 : 0);
        }
      }
      // A turned knob has repainted itself; everything follows in the next
      // frame, however many pointer moves came before it.
      this.later();
      this.hooks.changed?.();
    }
    /** A band's shape changes: kind, slope or placement, which the preset carries. */
    shape(b, fields) {
      const eq = this.eq();
      if (!eq) return;
      const band = eq.bands[b];
      if (fields.kind && fields.kind !== band.kind) {
        // A Q left where the old shape starts moves to where the new one does.
        if (Math.abs(band.q - (DEFAULT_Q[band.kind] ?? 1)) < 1e-3) band.q = DEFAULT_Q[fields.kind] ?? 1;
        band.enabled = true;
      }
      Object.assign(band, fields);
      this.hooks.structure();
      this.sync();
      this.hooks.changed?.();
    }
    later() {
      if (this.frame) return;
      this.frame = requestAnimationFrame(() => {
        this.frame = 0;
        this.sync();
      });
    }

    /** Fits the canvas to the graph, then draws. */
    layout() {
      const box = this.graph.getBoundingClientRect();
      if (!box.width || !box.height) return;
      const ratio = window.devicePixelRatio || 1;
      if (this.size.width !== box.width || this.size.height !== box.height || this.size.ratio !== ratio) {
        this.size = { width: box.width, height: box.height, ratio };
        this.canvas.width = Math.round(box.width * ratio);
        this.canvas.height = Math.round(box.height * ratio);
      }
      for (const toggle of this.scales.children)
        toggle.setAttribute("aria-pressed", String(Number(toggle.dataset.scale) === this.scale));
      this.draw();
    }
    span() {
      return this.size.width - LEFT - RIGHT;
    }
    x(f) {
      return LEFT + (Math.log(f / MIN_FREQUENCY) / FREQUENCY_SPAN) * this.span();
    }
    frequencyAt(x) {
      return MIN_FREQUENCY * Math.exp(clamp((x - LEFT) / this.span(), 0, 1) * FREQUENCY_SPAN);
    }
    y(db) {
      const half = (this.size.height - TOP - BOTTOM) / 2;
      return TOP + half - (db / this.scale) * half;
    }
    gainAt(y) {
      const half = (this.size.height - TOP - BOTTOM) / 2;
      return ((TOP + half - y) / half) * this.scale;
    }

    /** The bands as they sound now: modulated values where routes move them. */
    current(eq) {
      const feedback = this.feedback;
      if (!feedback) return { bands: eq.bands, moving: false };
      let moving = false;
      const bands = eq.bands.map((band, b) => {
        const live = (field) => {
          const state = feedback[this.key(b, field)];
          return state?.modulated ? state.current : null;
        };
        const on = live("on"), frequency = live("frequency"), gain = live("gain"), q = live("q");
        if (on === null && frequency === null && gain === null && q === null) return band;
        moving = true;
        return {
          ...band,
          enabled: on === null ? band.enabled : on >= 0.5,
          frequency: frequency === null ? band.frequency : frequencyOf(frequency),
          gain: gain === null ? band.gain : clamp(gain, -MAX_GAIN, MAX_GAIN),
          q: q === null ? band.q : qOf(q),
        };
      });
      return { bands, moving };
    }
    /** Where a band's handle stands. */
    point(band) {
      const x = this.x(band.frequency);
      const y = hasGain(band.kind) ? this.y(clamp(band.gain, -this.scale, this.scale)) : this.y(0);
      return [x, y];
    }

    draw() {
      const eq = this.eq();
      const { width, height, ratio } = this.size;
      if (!eq || !width) return;
      const c = this.canvas.getContext("2d");
      c.setTransform(ratio, 0, 0, ratio, 0, 0);
      c.clearRect(0, 0, width, height);
      const style = getComputedStyle(this.root);
      const accent = style.getPropertyValue("--accent").trim() || "#c4f486";
      const modulation = style.getPropertyValue("--modulation").trim() || "#8cdcd1";
      const muted = style.getPropertyValue("--muted").trim() || "#8c9a8c";
      // The grid: decades brighter, and the dB the scale shows.
      const right = width - RIGHT;
      c.lineWidth = 1;
      for (const f of GRID) {
        const x = Math.round(this.x(f)) + 0.5;
        c.strokeStyle = [100, 1000, 10000].includes(f) ? "#ffffff1f" : "#ffffff0b";
        c.beginPath();
        c.moveTo(x, TOP);
        c.lineTo(x, height - BOTTOM);
        c.stroke();
      }
      const step = this.scale / 2;
      c.font = "10px ui-monospace, monospace";
      c.textBaseline = "middle";
      c.textAlign = "right";
      for (let db = -this.scale; db <= this.scale; db += step) {
        const y = Math.round(this.y(db)) + 0.5;
        c.strokeStyle = db === 0 ? "#ffffff38" : "#ffffff10";
        c.beginPath();
        c.moveTo(LEFT, y);
        c.lineTo(right, y);
        c.stroke();
        c.fillStyle = db === 0 ? "#c9d3c6" : muted;
        c.fillText(db > 0 ? `+${db}` : db < 0 ? `−${-db}` : "0", width - 5, y);
      }
      c.textAlign = "center";
      c.textBaseline = "alphabetic";
      for (const [f, text] of LABELS) {
        c.fillStyle = muted;
        c.fillText(text, this.x(f), height - 6);
      }
      // The response at about one point per two pixels.
      const count = Math.max(64, Math.ceil(width / 2));
      const frequencies = Array.from({ length: count + 1 }, (_, i) =>
        MIN_FREQUENCY * Math.exp((i / count) * FREQUENCY_SPAN));
      const rate = this.rate();
      const base = responses(eq.bands, rate, frequencies);
      const { bands: live, moving } = this.current(eq);
      const now = moving ? responses(live, rate, frequencies) : null;
      const top = TOP, bottom = height - BOTTOM;
      const path = (curve) => {
        c.beginPath();
        let started = false;
        curve.forEach((db, i) => {
          if (!Number.isFinite(db)) return;
          const x = this.x(frequencies[i]);
          const y = clamp(this.y(db), top - 2, bottom + 2);
          if (started) c.lineTo(x, y);
          else c.moveTo(x, y), (started = true);
        });
        return started;
      };
      const fill = (curve, color) => {
        if (!path(curve)) return;
        const last = curve.findLastIndex(Number.isFinite);
        c.lineTo(this.x(frequencies[last]), this.y(0));
        c.lineTo(this.x(frequencies[0]), this.y(0));
        c.closePath();
        c.fillStyle = color;
        c.fill();
      };
      c.save();
      c.beginPath();
      c.rect(LEFT, top - 2, right - LEFT, bottom - top + 4);
      c.clip();
      // Each band on its own, the selected one filled in its color.
      eq.bands.forEach((band, b) => {
        if (!band.enabled) return;
        if (b === this.selected) fill(base.each[b], `${COLORS[b]}2e`);
        if (path(base.each[b])) {
          c.strokeStyle = `${COLORS[b]}${b === this.selected ? "cc" : "70"}`;
          c.lineWidth = b === this.selected ? 1.5 : 1;
          c.stroke();
        }
      });
      // Then their sum: what the EQ does.
      const gradient = c.createLinearGradient(0, top, 0, bottom);
      gradient.addColorStop(0, `${accent}33`);
      gradient.addColorStop(0.5, `${accent}08`);
      gradient.addColorStop(1, `${accent}33`);
      fill(base.total, gradient);
      if (path(base.total)) {
        c.strokeStyle = now ? `${accent}99` : accent;
        c.lineWidth = 2;
        c.lineJoin = "round";
        c.stroke();
      }
      if (now && path(now.total)) {
        c.strokeStyle = modulation;
        c.lineWidth = 2;
        c.stroke();
      }
      c.restore();
      // The handles, with a dot where modulation has a band now.
      eq.bands.forEach((band, b) => {
        const [x, y] = this.point(band);
        const handle = this.handles[b];
        handle.style.transform = `translate(${x}px, ${y}px)`;
        handle.classList.toggle("off", !band.enabled);
        handle.classList.toggle("selected", b === this.selected);
        handle.setAttribute("aria-label", `Band ${b + 1}: ${this.describe(band)}`);
        handle.title = `Band ${b + 1}: ${this.describe(band)}`;
        const ghost = this.ghosts[b];
        const moved = live[b] !== band && live[b].enabled;
        ghost.hidden = !moved;
        if (moved) {
          const [gx, gy] = this.point(live[b]);
          ghost.style.transform = `translate(${gx}px, ${gy}px)`;
        }
      });
    }

    /** The host's latest knob feedback: redraws only while modulation moves a band. */
    live(knobs) {
      const eq = this.eq();
      if (!eq) return;
      const prefix = `rack_eq${this.id}_`;
      let signature = "";
      for (const key in knobs ?? {}) {
        if (!key.startsWith(prefix)) continue;
        const state = knobs[key];
        if (state?.modulated) signature += `${key}:${state.current};`;
      }
      if (signature === this.liveSignature) return;
      this.liveSignature = signature;
      this.feedback = signature ? knobs : null;
      if (!this.section.isConnected || this.section.classList.contains("rack-folded")) return;
      this.draw();
    }

    show(text) {
      this.readout.textContent = text;
      this.readout.hidden = !text;
    }

    bindHandle(handle, b) {
      handle.addEventListener("pointerdown", (e) => {
        if (e.button !== 0) return;
        e.preventDefault();
        e.stopPropagation();
        if (this.selected !== b) this.select(b);
        handle.focus({ preventScroll: true });
        handle.setPointerCapture(e.pointerId);
        const band = this.band(b);
        const start = { x: e.clientX, y: e.clientY, frequency: band.frequency, gain: band.gain, q: band.q };
        handle.classList.add("dragging");
        this.dragging = true;
        const move = (event) => {
          // Shift moves a fifth as far, for fine adjustment.
          const fine = event.shiftKey ? 0.2 : 1;
          const dx = (event.clientX - start.x) * fine, dy = (event.clientY - start.y) * fine;
          const fields = { frequency: start.frequency * Math.exp((dx / this.span()) * FREQUENCY_SPAN) };
          if (hasGain(this.band(b).kind)) {
            const half = (this.size.height - TOP - BOTTOM) / 2;
            fields.gain = Math.round((start.gain - (dy / half) * this.scale) * 10) / 10;
          }
          this.change(b, fields);
          this.show(`${b + 1} · ${this.describe(this.band(b))}`);
        };
        const end = () => {
          handle.classList.remove("dragging");
          this.dragging = false;
          handle.removeEventListener("pointermove", move);
          handle.removeEventListener("pointerup", end);
          handle.removeEventListener("pointercancel", end);
          this.show("");
        };
        handle.addEventListener("pointermove", move);
        handle.addEventListener("pointerup", end);
        handle.addEventListener("pointercancel", end);
      });
      handle.addEventListener("dblclick", (e) => {
        e.stopPropagation();
        this.change(b, { enabled: !this.band(b).enabled });
      });
      handle.addEventListener("click", () => {
        if (this.selected !== b) this.select(b);
      });
      // The wheel over a handle sets its Q.
      handle.addEventListener("wheel", (e) => {
        e.preventDefault();
        if (this.selected !== b) this.select(b);
        const band = this.band(b);
        // A notch of a mouse wheel is 100 pixels; a trackpad sends less, more often.
        const pixels = e.deltaY * (e.deltaMode === 1 ? 33 : e.deltaMode === 2 ? 400 : 1);
        const notches = clamp(pixels, -400, 400) / 100;
        this.change(b, { q: band.q * Math.exp(-notches * (e.shiftKey ? 0.02 : 0.08)) });
        this.show(`${b + 1} · Q ${quality(this.band(b).q)}`);
        clearTimeout(this.readoutTimer);
        this.readoutTimer = setTimeout(() => this.show(""), 900);
      }, { passive: false });
      handle.addEventListener("keydown", (e) => {
        const band = this.band(b);
        const big = e.shiftKey;
        let fields = null;
        if (e.key === "ArrowRight" || e.key === "ArrowLeft") {
          const octaves = (big ? 1 / 3 : 1 / 24) * (e.key === "ArrowRight" ? 1 : -1);
          fields = { frequency: band.frequency * 2 ** octaves };
        } else if (e.key === "ArrowUp" || e.key === "ArrowDown") {
          const direction = e.key === "ArrowUp" ? 1 : -1;
          fields = hasGain(band.kind)
            ? { gain: Math.round((band.gain + direction * (big ? 3 : 0.5)) * 10) / 10 }
            : { q: band.q * (big ? 1.5 : 1.1) ** direction };
        } else if (e.key === "Delete" || e.key === "Backspace") fields = { enabled: !band.enabled };
        if (!fields) return;
        e.preventDefault();
        if (this.selected !== b) this.select(b);
        this.change(b, fields);
      });
    }

    bindGraph() {
      this.graph.addEventListener("pointermove", (e) => {
        if (this.dragging || e.target.closest(".eq-handle, .eq-scale")) return;
        const box = this.graph.getBoundingClientRect();
        const f = this.frequencyAt(e.clientX - box.left);
        const eq = this.eq();
        if (!eq) return;
        const total = responses(eq.bands, this.rate(), [f]).total[0];
        this.show(`${hertz(f)} · ${decibels(total)}`);
      });
      this.graph.addEventListener("pointerleave", () => {
        if (!this.dragging) this.show("");
      });
      // A double-click on the graph brings the nearest band that is off there.
      this.graph.addEventListener("dblclick", (e) => {
        if (e.target.closest(".eq-handle, .eq-scale")) return;
        const eq = this.eq();
        if (!eq) return;
        const box = this.graph.getBoundingClientRect();
        const frequency = this.frequencyAt(e.clientX - box.left);
        const gain = Math.round(clamp(this.gainAt(e.clientY - box.top), -MAX_GAIN, MAX_GAIN) * 10) / 10;
        const free = eq.bands.map((band, b) => [band, b]).filter(([band]) => !band.enabled);
        if (!free.length) {
          this.show("All eight bands are on: switch one off to place it here");
          clearTimeout(this.readoutTimer);
          this.readoutTimer = setTimeout(() => this.show(""), 2400);
          return;
        }
        const distance = (band) => Math.abs(Math.log(band.frequency / frequency));
        const [band, b] = free.reduce((best, next) => (distance(next[0]) < distance(best[0]) ? next : best));
        // A cut far from its own end becomes a bell.
        const far = (band.kind === "low_cut" && frequency > 150) || (band.kind === "high_cut" && frequency < 6000)
          || band.kind === "notch" || band.kind === "band_pass";
        this.selected = b;
        this.root.style.setProperty("--eq-selected", COLORS[b]);
        if (far) {
          band.kind = "bell";
          band.q = 1;
          this.hooks.structure();
        }
        this.change(b, { enabled: true, frequency, ...(hasGain(band.kind) ? { gain } : {}) });
        this.handles[b].focus({ preventScroll: true });
      });
    }
  }

  window.SadEq = { Editor, defaults, responses, design, unitFrequency, frequencyOf, unitQ, qOf, BANDS };
})();
