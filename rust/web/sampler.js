"use strict";
// The Sampler timbre's editor, in the manner of an MPC: banks of sixteen
// pads, the Sample editor (Trim, Loop, Chop and Process on the waveform), pad
// and zone settings, Pad FX chains, Bounce, recording and the sample library
// with packs. Kit edits change the preset's `sampler` in place and post it
// like any structural edit; the engine keeps a sample pool that this editor
// fills from the library.
(() => {
  const Files = window.SadSamplerFiles;
  const BANK = 16;
  const BANKS = "ABCDEFGH";
  const NAMES = ["C", "C♯", "D", "D♯", "E", "F", "F♯", "G", "G♯", "A", "A♯", "B"];
  const noteName = (n) => `${NAMES[n % 12]}${Math.floor(n / 12) - 1}`;
  const clamp = (v, lo, hi) => Math.max(lo, Math.min(hi, v));
  const el = (tag, className, text) => {
    const node = document.createElement(tag);
    if (className) node.className = className;
    if (text !== undefined) node.textContent = text;
    return node;
  };
  const button = (text, title, action) => {
    const b = el("button", "", text);
    if (title) b.title = title;
    b.onclick = action;
    return b;
  };
  const label = (index) => `${BANKS[Math.floor(index / BANK)] || "?"}${String((index % BANK) + 1).padStart(2, "0")}`;
  const stem = (name) => name.replace(/\.[a-z0-9]{2,5}$/i, "");
  const newId = () => `smp-${crypto.randomUUID()}`;
  const ms = (v) => (v >= 1000 ? `${(v / 1000).toFixed(2)} s` : `${Math.round(v)} ms`);
  const db = (v) => `${v > 0 ? "+" : ""}${v.toFixed(1)} dB`;
  const signed = (v, unit, digits = 0) => `${v > 0 ? "+" : ""}${v.toFixed(digits)}${unit}`;
  // Knob scales: times and frequencies feel even on a log dial.
  const logScale = (max, floor = 0) => ({
    normalize: (v) => Math.log1p(Math.max(0, v - floor)) / Math.log1p(max - floor),
    denormalize: (u) => floor + Math.expm1(clamp(u, 0, 1) * Math.log1p(max - floor)),
  });
  const hertz = {
    normalize: (v) => Math.log(v / 20) / Math.log(1000),
    denormalize: (u) => 20 * 1000 ** clamp(u, 0, 1),
  };
  const OPERATIONS = [
    ["crop", "Crop to selection", []],
    ["delete", "Delete selection", []],
    ["silence", "Silence selection", []],
    ["insert_silence", "Insert silence", [["ms", "ms", 250, 1, 600000]]],
    ["normalize", "Normalize", [["peak", "Peak dB", -0.3, -60, 6]]],
    ["gain", "Gain", [["db", "dB", 3, -96, 48]]],
    ["reverse", "Reverse", []],
    ["fade_in", "Fade in", [["curve", "Curve", 0, -1, 1]]],
    ["fade_out", "Fade out", [["curve", "Curve", 0, -1, 1]]],
    ["remove_dc", "Remove DC offset", []],
    ["invert", "Invert phase", []],
    ["mono", "Sum to mono", []],
    ["swap", "Swap left / right", []],
    ["pitch", "Pitch shift", [["semitones", "Semitones", 12, -48, 48], ["keep_length", "Keep length", 1, 0, 1]]],
    ["stretch", "Time stretch", [["factor", "Length ×", 1.5, 0.125, 8]]],
    ["resample", "Resample", [["rate", "Hz", 22050, 8000, 384000]]],
    ["crush", "Bit crush", [["bits", "Bits", 12, 1, 24], ["rate", "Hz", 26040, 100, 384000]]],
    ["trim_silence", "Trim silence", [["threshold", "Threshold dB", -60, -120, 0]]],
  ];

  class Sampler {
    constructor(root, hooks) {
      this.root = root;
      this.hooks = hooks;
      this.library = new Files.Library();
      this.library.listeners.add(() => this.refreshLibrary());
      this.cache = new Map();
      this.bank = 0;
      this.zone = 0;
      this.mode = "trim";
      this.view = [0, 1];
      this.selection = null;
      this.slices = [];
      this.history = [];
      this.loaded = new Set();
      this.uploads = new Map();
      this.missing = new Set();
      this.chains = [];
      this.hits = new Map();
      this.flash = new Map();
      this.voices = [];
      this.build();
      this.loop();
    }

    // --- Model ------------------------------------------------------------
    kit() {
      return this.hooks.read()?.sampler || null;
    }
    /** The timbre whose kit this is: a Sampler, or a Rack with a Sampler
     * module where the host plays kits. */
    part() {
      const context = this.hooks.context();
      const active = context?.timbres?.find((t) => t.id === context.active);
      if (active?.kind === "sampler") return active.id;
      return active?.kind === "rack" && this.kit() && this.hooks.features?.()?.sampler ? active.id : null;
    }
    pad() {
      const kit = this.kit();
      return kit?.pads.find((p) => p.id === kit.selected) || null;
    }
    padIndex(pad = this.pad()) {
      return this.kit()?.pads.indexOf(pad) ?? -1;
    }
    zoneOf(pad = this.pad()) {
      if (!pad?.zones.length) return null;
      this.zone = clamp(this.zone, 0, pad.zones.length - 1);
      return pad.zones[this.zone];
    }
    /** Edits the kit in place and posts the preset. */
    change(edit, { redraw = true } = {}) {
      const kit = this.kit();
      if (!kit) return;
      edit(kit);
      this.hooks.structure();
      if (redraw) this.sync();
    }
    editPad(edit, options) {
      const pad = this.pad();
      if (pad) this.change(() => edit(pad), options);
    }
    newPad(kit, index) {
      const used = new Set(kit.pads.map((p) => p.id));
      let id = 1;
      while (used.has(id)) id++;
      const note = clamp(36 + index, 0, 127);
      return {
        id, name: "", keys: [note, note], keytrack: false, trigger: "one_shot", mono: false,
        choke: 0, layers: "velocity", zones: [], tune: 0, fine: 0, gain: 0, pan: 0,
        envelope: { attack: 0, hold: 0, decay: 300, sustain: 1, release: 30 },
        filter: { kind: "off", cutoff: 8000, resonance: 0.1, envelope: 0, attack: 0, decay: 300, keytrack: 0, velocity: 0 },
        pitch: { amount: 0, decay: 100 }, velocity: { level: 1, start: 0, attack: 0 },
        muted: false, solo: false, bounce: null,
      };
    }
    zoneFor(sample, frames) {
      return {
        sample, start: 0, end: frames || 0, loop_mode: "off", loop_start: 0, crossfade: 0,
        reverse: false, fade_in: 0, fade_out: 0, fade_curve: [0, 0], root: 60, tune: 0,
        fine: 0, gain: 0, pan: 0, keys: [0, 127], velocities: [1, 127],
      };
    }
    /** Pads up to `index`, adding whole banks as needed. */
    ensurePads(kit, index) {
      while (kit.pads.length <= index && kit.pads.length < 128) {
        const bank = kit.pads.length;
        for (let i = 0; i < BANK && kit.pads.length < 128; i++)
          kit.pads.push(this.newPad(kit, bank + i));
      }
    }

    // --- Audio data ---------------------------------------------------------
    /** Decoded audio of a pool sample: from the library, else the engine. */
    async audio(id) {
      if (!id) return null;
      if (this.cache.has(id)) return this.cache.get(id);
      const promise = (async () => {
        let record = await this.library.get(id);
        if (!record) {
          const part = this.part();
          if (part === null) return null;
          const response = await this.hooks.api("/api/sampler/sample.wav", { part, id });
          const wav = await response.arrayBuffer();
          // Samples that came with a scene join the library.
          record = await this.library.put({ id, name: "Sample", wav: new Blob([wav], { type: "audio/wav" }), origin: "scene" });
        }
        const audio = Files.parseWav(await record.wav.arrayBuffer());
        if (!audio) throw new Error(`${record.name}: unreadable audio`);
        audio.name = record.name;
        return audio;
      })();
      this.cache.set(id, promise);
      try {
        const audio = await promise;
        this.cache.set(id, audio);
        return audio;
      } catch (error) {
        this.cache.delete(id);
        throw error;
      }
    }
    async wavOf(id) {
      const record = await this.library.get(id);
      if (record) return record.wav;
      const audio = await this.audio(id);
      return new Blob([Files.encodeWav(audio.rate, audio.left, audio.right)], { type: "audio/wav" });
    }
    /** Stores a sample and sends it to the engine; returns its record. */
    async store(name, wav, origin, extra = {}) {
      const blob = wav instanceof Blob ? wav : new Blob([wav], { type: "audio/wav" });
      const audio = Files.parseWav(await blob.arrayBuffer());
      if (!audio) throw new Error(`${name}: unreadable audio`);
      const record = await this.library.put({
        id: newId(), name, wav: blob, rate: audio.rate, frames: audio.frames,
        channels: audio.channels, origin, ...extra,
      });
      audio.name = name;
      this.cache.set(record.id, audio);
      await this.upload(record.id);
      return record;
    }
    async upload(id) {
      const part = this.part();
      if (part === null || this.loaded.has(id)) return;
      const key = `${part}:${id}`;
      if (!this.uploads.has(key)) {
        this.uploads.set(key, (async () => {
          try {
            const record = await this.library.get(id);
            if (!record) {
              this.missing.add(id);
              return;
            }
            await this.hooks.api("/api/sampler/upload", { id, part, wav: record.wav }, { "X-Sampler": "1" });
            this.loaded.add(id);
            this.missing.delete(id);
          } finally {
            this.uploads.delete(key);
          }
        })());
      }
      return this.uploads.get(key);
    }
    /** Every sample the kit plays reaches the engine's pool. */
    syncPool() {
      const kit = this.kit();
      if (!kit || this.part() === null) return;
      const ids = new Set();
      for (const pad of kit.pads)
        for (const zone of [...pad.zones, ...(pad.bounce?.zones || [])])
          if (zone.sample) ids.add(zone.sample);
      for (const id of ids)
        if (!this.loaded.has(id))
          this.upload(id).catch((e) => this.hooks.notify(e.message));
    }
    /** Imports files: audio becomes samples, a .zip a pack. */
    async importFiles(files, at = this.padIndex()) {
      const kit = this.kit();
      if (!kit) return;
      let index = Math.max(0, at);
      for (const file of files) {
        const buffer = await file.arrayBuffer();
        if (/\.zip$/i.test(file.name)) {
          await this.importPack(buffer, file.name);
          continue;
        }
        const audio = await Files.decodeAudio(buffer);
        const wav = /\.wav$/i.test(file.name) && Files.parseWav(buffer) ? buffer : Files.encodeWav(audio.rate, audio.left, audio.right);
        const record = await this.store(stem(file.name), wav, "import");
        // The first file goes to the chosen pad, the rest to the next empty ones.
        if (files.length > 1)
          while (index < 128 && kit.pads[index]?.zones.length && index !== at) index++;
        this.assign(index, record);
        index++;
      }
      this.sync();
    }
    /** One sample on a pad, replacing its zones. */
    assign(index, record) {
      this.change((kit) => {
        this.ensurePads(kit, index);
        const pad = kit.pads[index];
        pad.zones = [this.zoneFor(record.id, record.frames)];
        pad.bounce = null;
        pad.name = record.name.slice(0, 80);
        kit.selected = pad.id;
      });
      this.zone = 0;
      this.view = [0, 1];
      this.selection = null;
      this.slices = [];
    }

    // --- DOM ----------------------------------------------------------------
    build() {
      const root = this.root;
      root.innerHTML = "";
      const heading = el("div", "section-heading sampler-heading");
      const title = el("h2", "", "Sampler ");
      title.append(el("span", "", "pads · samples · packs"));
      this.statusText = el("span", "sampler-status", "");
      heading.append(title, this.statusText);
      const tools = el("div", "sampler-tools");
      this.banks = el("div", "sampler-banks");
      this.banks.setAttribute("role", "group");
      this.banks.setAttribute("aria-label", "Pad banks");
      for (let b = 0; b < BANKS.length; b++) {
        const bank = button(BANKS[b], `Bank ${BANKS[b]}: pads ${b * BANK + 1}–${b * BANK + BANK}`, () => {
          this.bank = b;
          this.sync();
        });
        bank.dataset.bank = b;
        this.banks.append(bank);
      }
      this.fileInput = Object.assign(el("input"), { type: "file", multiple: true, hidden: true, accept: "audio/*,.wav,.aif,.aiff,.flac,.mp3,.ogg,.m4a,.zip" });
      this.fileInput.onchange = () => {
        const files = [...this.fileInput.files];
        this.fileInput.value = "";
        this.importFiles(files).catch((e) => this.hooks.notify(e.message));
      };
      const importButton = button("Import…", "Import audio files or a pack (.zip) onto the pads", () => this.fileInput.click());
      this.recordButton = button("● Record", "Record a sample from an input or from SAD PET's output", () => {
        this.recordPanel.hidden = !this.recordPanel.hidden;
        if (!this.recordPanel.hidden) this.listInputs();
      });
      this.recordButton.className = "sampler-record-toggle";
      this.libraryButton = button("Library", "Samples and packs saved in this browser", () =>
        this.showLibrary(this.libraryPanel.hidden));
      this.libraryButton.setAttribute("aria-pressed", "false");
      this.polyphony = el("select");
      this.polyphony.setAttribute("aria-label", "Polyphony");
      for (const n of [1, 2, 4, 8, 16, 24, 32, 48, 64]) this.polyphony.append(new Option(`${n} voices`, n));
      this.polyphony.onchange = () => this.change((kit) => (kit.polyphony = Number(this.polyphony.value)));
      // Starter packs come with SAD PET and load on first use.
      this.starter = el("select");
      this.starter.setAttribute("aria-label", "Load a starter pack");
      this.starter.append(new Option("Starter packs…", ""));
      this.starter.onchange = () => {
        const id = this.starter.value;
        this.starter.value = "";
        this.bundled().then((packs) => {
          const entry = packs.find((p) => p.id === id);
          if (entry) return this.loadBundled(entry);
        }).catch((e) => this.hooks.notify(e.message));
      };
      this.bundled().then((packs) => {
        for (const p of packs) this.starter.append(new Option(`${p.name} · ${p.pads} pad${p.pads === 1 ? "" : "s"}`, p.id));
        this.starter.hidden = !packs.length;
      });
      tools.append(this.banks, this.starter, importButton, this.recordButton, this.libraryButton, this.polyphony, this.fileInput);
      this.recordPanel = this.buildRecord();
      this.libraryPanel = this.buildLibrary();
      const layout = el("div", "sampler-layout");
      this.grid = el("div", "sampler-pads");
      this.grid.setAttribute("aria-label", "Pads");
      this.padButtons = [];
      for (let i = 0; i < BANK; i++) {
        const pad = el("button", "sampler-pad");
        pad.innerHTML = `<span class="pad-label"></span><span class="pad-name"></span><span class="pad-note"></span><span class="pad-badges"></span>`;
        this.bindPad(pad, i);
        this.padButtons.push(pad);
      }
      // MPC order: pad 1 at the bottom left.
      for (const row of [3, 2, 1, 0])
        for (let c = 0; c < 4; c++) this.grid.append(this.padButtons[row * 4 + c]);
      const side = el("div", "sampler-side");
      side.append(this.buildPadHeader(), this.buildWave(), this.buildModes());
      layout.append(this.grid, side);
      this.settings = this.buildSettings();
      root.append(heading, tools, this.recordPanel, this.libraryPanel, layout, this.settings);
      root.addEventListener("dragover", (e) => {
        if ([...(e.dataTransfer?.types || [])].includes("Files")) e.preventDefault();
      });
    }
    bindPad(element, slot) {
      const index = () => this.bank * BANK + slot;
      let held = null;
      element.addEventListener("pointerdown", (e) => {
        if (e.button !== 0) return;
        const kit = this.kit();
        if (!kit) return;
        const pad = kit.pads[index()];
        if (!pad) {
          this.change((k) => this.ensurePads(k, index()));
          return;
        }
        if (kit.selected !== pad.id) {
          kit.selected = pad.id;
          this.zone = 0;
          this.view = [0, 1];
          this.selection = null;
          this.slices = [];
          this.hooks.structure();
          this.sync();
        }
        // Higher on the pad plays louder.
        const box = element.getBoundingClientRect();
        const velocity = Math.round(clamp(127 - ((e.clientY - box.top) / box.height) * 87, 40, 127));
        held = pad.id;
        element.setPointerCapture(e.pointerId);
        this.hooks.request("/api/sampler/trigger", { pad: pad.id, velocity }).catch((err) => this.hooks.notify(err.message));
      });
      const release = () => {
        if (held === null) return;
        const pad = held;
        held = null;
        this.hooks.request("/api/sampler/trigger", { pad, off: true }).catch(() => {});
      };
      element.addEventListener("pointerup", release);
      element.addEventListener("pointercancel", release);
      element.addEventListener("keydown", (e) => {
        if (e.key === " " || e.key === "Enter") {
          e.preventDefault();
          const pad = this.kit()?.pads[index()];
          if (pad) this.hooks.request("/api/sampler/trigger", { pad: pad.id, velocity: 110 }).catch(() => {});
        }
      });
      element.addEventListener("dragover", (e) => {
        e.preventDefault();
        element.classList.add("drop");
      });
      element.addEventListener("dragleave", () => element.classList.remove("drop"));
      element.addEventListener("drop", (e) => {
        e.preventDefault();
        element.classList.remove("drop");
        const id = e.dataTransfer.getData("application/x-sad-sample");
        if (id) {
          this.library.get(id).then((record) => record && this.assign(index(), record));
          return;
        }
        const files = [...(e.dataTransfer.files || [])];
        if (files.length) this.importFiles(files, index()).catch((err) => this.hooks.notify(err.message));
      });
    }
    buildPadHeader() {
      const header = el("div", "sampler-pad-header");
      this.padTitle = el("strong", "sampler-pad-title");
      this.padName = el("input");
      this.padName.maxLength = 80;
      this.padName.placeholder = "Pad name";
      this.padName.setAttribute("aria-label", "Pad name");
      this.padName.onchange = () => this.editPad((pad) => (pad.name = this.padName.value.trim()));
      this.muteButton = button("Mute", "Mute this pad", () => this.editPad((pad) => (pad.muted = !pad.muted)));
      this.soloButton = button("Solo", "Only soloed pads sound", () => this.editPad((pad) => (pad.solo = !pad.solo)));
      this.copyButton = button("Copy pad", "Copy this pad to the next empty pad", () => this.duplicatePad());
      this.clearButton = button("Clear", "Remove the pad's samples and settings", () => {
        const pad = this.pad();
        if (!pad) return;
        const index = this.padIndex(pad);
        this.change((kit) => {
          const fresh = this.newPad(kit, index);
          fresh.id = pad.id;
          fresh.keys = pad.keys;
          kit.pads[index] = fresh;
        });
      });
      header.append(this.padTitle, this.padName, this.muteButton, this.soloButton, this.copyButton, this.clearButton);
      return header;
    }
    duplicatePad() {
      const pad = this.pad();
      if (!pad) return;
      this.change((kit) => {
        let index = kit.pads.findIndex((p, i) => i > this.padIndex(pad) && !p.zones.length);
        if (index < 0) {
          index = kit.pads.length;
          this.ensurePads(kit, index);
        }
        const target = kit.pads[index];
        if (!target) return;
        const copy = structuredClone(pad);
        copy.id = target.id;
        copy.keys = target.keys;
        copy.name = pad.name ? `${pad.name} copy`.slice(0, 80) : "";
        kit.pads[index] = copy;
        kit.selected = copy.id;
        this.bank = Math.floor(index / BANK);
      });
    }
    buildWave() {
      const box = el("div", "sampler-wave");
      const bar = el("div", "sampler-wave-bar");
      this.modeTabs = el("div", "sampler-mode-tabs");
      this.modeTabs.setAttribute("role", "tablist");
      for (const [mode, text] of [["trim", "Trim"], ["loop", "Loop"], ["chop", "Chop"], ["process", "Process"]]) {
        const tab = button(text, "", () => {
          this.mode = mode;
          this.syncModes();
          this.draw();
        });
        tab.setAttribute("role", "tab");
        tab.dataset.mode = mode;
        this.modeTabs.append(tab);
      }
      this.zoneSelect = el("select");
      this.zoneSelect.setAttribute("aria-label", "Zone");
      this.zoneSelect.onchange = () => {
        this.zone = Number(this.zoneSelect.value);
        this.view = [0, 1];
        this.selection = null;
        this.sync();
      };
      const zoom = el("div", "sampler-zoom");
      zoom.append(
        button("−", "Zoom out", () => this.zoom(2)),
        button("+", "Zoom in", () => this.zoom(0.5)),
        button("Fit", "Show the whole sample", () => {
          this.view = [0, 1];
          this.draw();
        }),
      );
      this.snap = Object.assign(el("input"), { type: "checkbox", checked: true });
      const snap = el("label", "check", "");
      snap.append(this.snap, "Zero snap");
      snap.title = "Markers and slices move to the nearest zero crossing";
      this.readout = el("output", "sampler-readout");
      bar.append(this.modeTabs, this.zoneSelect, zoom, snap, this.readout);
      const stage = el("div", "sampler-canvas");
      this.canvas = el("canvas");
      this.canvas.tabIndex = 0;
      this.canvas.setAttribute("aria-label", "Sample waveform. Drag markers; drag the waveform to select; the wheel zooms.");
      this.overlay = el("canvas", "sampler-overlay");
      this.empty = el("div", "sampler-empty", "Drop audio here or on a pad, or Import / Record.");
      stage.append(this.canvas, this.overlay, this.empty);
      this.bindCanvas();
      box.append(bar, stage);
      return box;
    }
    zoom(factor, center = (this.view[0] + this.view[1]) / 2) {
      const span = clamp((this.view[1] - this.view[0]) * factor, 1e-5, 1);
      let start = center - span / 2;
      start = clamp(start, 0, 1 - span);
      this.view = [start, start + span];
      this.draw();
    }
    buildModes() {
      const modes = el("div", "sampler-modes");
      // Trim
      this.trimRow = el("div", "sampler-row");
      this.startInput = this.numberField("Start", "First frame played", (v) => this.editZone((z) => (z.start = v)));
      this.endInput = this.numberField("End", "Frame after the last one played", (v) => this.editZone((z) => (z.end = v)));
      this.fadeIn = this.numberField("Fade in ms", "", (v) => this.editZone((z) => (z.fade_in = v)), 0, 60000);
      this.fadeInCurve = this.rangeField("Curve", -1, 1, 0.05, (v) => this.editZone((z) => (z.fade_curve[0] = v)));
      this.fadeOut = this.numberField("Fade out ms", "", (v) => this.editZone((z) => (z.fade_out = v)), 0, 60000);
      this.fadeOutCurve = this.rangeField("Curve", -1, 1, 0.05, (v) => this.editZone((z) => (z.fade_curve[1] = v)));
      this.reverse = this.checkField("Reverse", (v) => this.editZone((z) => (z.reverse = v)));
      this.rootSelect = this.noteField("Root", (v) => this.editZone((z) => (z.root = v)));
      const detect = button("Detect root", "Find the sample's pitch", () => this.detectRoot());
      this.trimRow.append(this.startInput.box, this.endInput.box, this.fadeIn.box, this.fadeInCurve.box,
        this.fadeOut.box, this.fadeOutCurve.box, this.reverse.box, this.rootSelect.box, detect);
      // Loop
      this.loopRow = el("div", "sampler-row");
      this.loopMode = this.selectField("Loop", [["off", "Off"], ["forward", "Forward"], ["alternate", "Back and forth"], ["sustain", "While held"]],
        (v) => this.editZone((z) => (z.loop_mode = v)));
      this.loopStart = this.numberField("Loop start", "The loop runs from here to End", (v) => this.editZone((z) => (z.loop_start = v)));
      this.crossfade = this.numberField("Crossfade ms", "Blends audio before the loop start into its end", (v) => this.editZone((z) => (z.crossfade = v)), 0, 10000);
      const bake = button("Bake crossfade", "Write the crossfade into a new sample, so a plain loop joins smoothly", () => {
        const zone = this.zoneOf();
        if (!zone?.crossfade) return this.hooks.notify("Set a crossfade first");
        this.process({ op: "loop_crossfade", loop_start: zone.loop_start, end: zone.end, ms: zone.crossfade }, { after: (z) => (z.crossfade = 0) });
      });
      this.loopRow.append(this.loopMode.box, this.loopStart.box, this.crossfade.box, bake);
      // Chop
      this.chopRow = el("div", "sampler-row");
      this.chopMethod = this.selectField("Chop", [["transients", "Transients"], ["regions", "Regions"], ["beats", "Beats"], ["manual", "Manual (click)"]], () => this.syncModes());
      this.sensitivity = this.rangeField("Sensitivity", 0, 1, 0.01, () => {});
      this.sensitivity.input.value = "0.5";
      this.regions = this.numberField("Slices", "", () => {}, 1, 128);
      this.regions.input.value = "16";
      this.chopBpm = this.numberField("BPM", "", () => {}, 20, 999);
      this.chopBpm.input.value = "120";
      this.chopBeats = this.selectField("Every", [["0.25", "1/16"], ["0.5", "1/8"], ["1", "1 beat"], ["2", "2 beats"], ["4", "1 bar"]], () => {});
      this.chopBeats.input.value = "1";
      const find = button("Find slices", "", () => this.chop());
      this.sliceCount = el("output", "sampler-readout");
      const toPads = button("→ Pads", "Each slice onto its own pad, from this one on (same sample)", () => this.slicesToPads(false));
      const toKeys = button("→ Keys", "All slices on this pad, one per key from its note", () => this.slicesToKeys());
      const toSamples = button("→ New samples", "Cut each slice into its own sample on its own pad", () => this.slicesToPads(true));
      const clear = button("Clear", "", () => {
        this.slices = [];
        this.draw();
        this.syncModes();
      });
      this.chopRow.append(this.chopMethod.box, this.sensitivity.box, this.regions.box, this.chopBpm.box, this.chopBeats.box, find, this.sliceCount, toPads, toKeys, toSamples, clear);
      // Process
      this.processRow = el("div", "sampler-row");
      this.operation = this.selectField("Process", OPERATIONS.map(([id, text]) => [id, text]), () => this.syncOperation());
      this.operationFields = el("div", "sampler-op-fields");
      const apply = button("Apply", "Make a new version of the sample; the original stays in the library", () => this.applyOperation());
      apply.className = "light";
      this.undoButton = button("Undo", "Back to the previous version of this zone", () => this.undoEdit());
      const copy = button("Copy", "Copy the selection", () => this.copySelection());
      const paste = button("Paste", "Insert the copied audio at the selection start", () => this.pasteClip(false));
      const mix = button("Mix paste", "Mix the copied audio in from the selection start", () => this.pasteClip(true));
      this.processRow.append(this.operation.box, this.operationFields, apply, this.undoButton, copy, paste, mix);
      modes.append(this.trimRow, this.loopRow, this.chopRow, this.processRow);
      this.syncOperation();
      return modes;
    }
    numberField(text, title, onchange, min = 0, max = 2 ** 32 - 1) {
      const box = el("label", "sampler-field", text);
      if (title) box.title = title;
      const input = Object.assign(el("input"), { type: "number", min, max, step: "any" });
      input.onchange = () => {
        const v = Number(input.value);
        if (Number.isFinite(v)) onchange(clamp(v, min, max));
      };
      box.append(input);
      return { box, input };
    }
    rangeField(text, min, max, step, onchange) {
      const box = el("label", "sampler-field", text);
      const input = Object.assign(el("input"), { type: "range", min, max, step });
      input.oninput = () => onchange(Number(input.value));
      input.ondblclick = () => {
        input.value = "0";
        onchange(0);
      };
      box.append(input);
      return { box, input };
    }
    checkField(text, onchange) {
      const box = el("label", "check sampler-check", "");
      const input = Object.assign(el("input"), { type: "checkbox" });
      input.onchange = () => onchange(input.checked);
      box.append(input, text);
      return { box, input };
    }
    selectField(text, options, onchange) {
      const box = el("label", "sampler-field", text);
      const input = el("select");
      for (const [value, name] of options) input.append(new Option(name, value));
      input.onchange = () => onchange(input.value);
      box.append(input);
      return { box, input };
    }
    noteField(text, onchange) {
      return this.selectField(text, Array.from({ length: 128 }, (_, n) => [String(n), `${noteName(n)} · ${n}`]), (v) => onchange(Number(v)));
    }
    buildSettings() {
      const box = el("div", "sampler-settings");
      const knob = (container, key, text, format, min, max, path, extra = {}) =>
        this.knobs.push(this.hooks.knob([`sampler_${key}`, text, format, min, max, `Pad ${text.toLowerCase()}`], container, {
          register: false,
          get: () => {
            const pad = this.pad();
            return pad ? path.reduce((o, k) => o[k], pad) : extra.reset ?? 0;
          },
          set: (v) => {
            const pad = this.pad();
            if (!pad) return;
            const target = path.slice(0, -1).reduce((o, k) => o[k], pad);
            target[path.at(-1)] = extra.step ? Math.round(v / extra.step) * extra.step : v;
            this.hooks.structure();
          },
          reset: () => extra.reset ?? 0,
          ...extra,
        }));
      this.knobs = [];
      const group = (title) => {
        const section = el("div", "sampler-group");
        section.append(el("h3", "", title));
        const knobs = el("div", "sampler-knobs");
        section.append(knobs);
        box.append(section);
        return { section, knobs };
      };
      const sound = group("Pad");
      knob(sound.knobs, "tune", "TUNE", (v) => signed(v, " st"), -48, 48, ["tune"], { step: 1 });
      knob(sound.knobs, "fine", "FINE", (v) => signed(v, " ct"), -100, 100, ["fine"], { step: 1 });
      knob(sound.knobs, "gain", "LEVEL", db, -60, 12, ["gain"]);
      knob(sound.knobs, "pan", "PAN", (v) => (Math.abs(v) < 0.01 ? "C" : v < 0 ? `L${Math.round(-v * 100)}` : `R${Math.round(v * 100)}`), -1, 1, ["pan"]);
      const play = el("div", "sampler-play");
      this.trigger = this.selectField("Trigger", [["one_shot", "One shot"], ["note_on", "Note on"]], (v) => this.editPad((p) => (p.trigger = v)));
      this.keyLow = this.noteField("Keys", (v) => this.editPad((p) => (p.keys = [v, Math.max(v, p.keys[1])])));
      this.keyHigh = this.noteField("to", (v) => this.editPad((p) => (p.keys = [Math.min(v, p.keys[0]), v])));
      this.keytrack = this.checkField("Keytrack", (v) => this.editPad((p) => (p.keytrack = v)));
      this.mono = this.checkField("Mono", (v) => this.editPad((p) => (p.mono = v)));
      this.choke = this.selectField("Mute group", [["0", "Off"], ...Array.from({ length: 16 }, (_, i) => [String(i + 1), String(i + 1)])], (v) => this.editPad((p) => (p.choke = Number(v))));
      this.layers = this.selectField("Layers", [["velocity", "By velocity"], ["cycle", "Cycle"], ["random", "Random"]], (v) => this.editPad((p) => (p.layers = v)));
      play.append(this.trigger.box, this.keyLow.box, this.keyHigh.box, this.keytrack.box, this.mono.box, this.choke.box, this.layers.box);
      sound.section.append(play);
      const amp = group("Amp envelope");
      knob(amp.knobs, "attack", "ATTACK", ms, 0, 30000, ["envelope", "attack"], logScale(30000));
      knob(amp.knobs, "hold", "HOLD", ms, 0, 30000, ["envelope", "hold"], logScale(30000));
      knob(amp.knobs, "decay", "DECAY", ms, 1, 30000, ["envelope", "decay"], { ...logScale(30000, 1), reset: 300 });
      knob(amp.knobs, "sustain", "SUSTAIN", (v) => `${Math.round(v * 100)}%`, 0, 1, ["envelope", "sustain"], { reset: 1 });
      knob(amp.knobs, "release", "RELEASE", ms, 1, 30000, ["envelope", "release"], { ...logScale(30000, 1), reset: 30 });
      const filter = group("Filter");
      this.filterKind = this.selectField("Type", [["off", "Off"], ["lowpass", "Low pass"], ["highpass", "High pass"], ["bandpass", "Band pass"], ["notch", "Notch"]], (v) => this.editPad((p) => (p.filter.kind = v)));
      filter.section.querySelector("h3").after(this.filterKind.box);
      knob(filter.knobs, "cutoff", "CUTOFF", (v) => (v >= 1000 ? `${(v / 1000).toFixed(1)} kHz` : `${Math.round(v)} Hz`), 20, 20000, ["filter", "cutoff"], { ...hertz, reset: 8000 });
      knob(filter.knobs, "resonance", "RESO", (v) => `${Math.round(v * 100)}%`, 0, 1, ["filter", "resonance"], { reset: 0.1 });
      knob(filter.knobs, "fenv", "ENV", (v) => signed(v, " oct", 1), -8, 8, ["filter", "envelope"]);
      knob(filter.knobs, "fattack", "F.ATTACK", ms, 0, 30000, ["filter", "attack"], logScale(30000));
      knob(filter.knobs, "fdecay", "F.DECAY", ms, 1, 30000, ["filter", "decay"], { ...logScale(30000, 1), reset: 300 });
      knob(filter.knobs, "fkey", "KEY", (v) => `${Math.round(v * 100)}%`, 0, 1, ["filter", "keytrack"]);
      knob(filter.knobs, "fvel", "VEL", (v) => `${v.toFixed(1)} oct`, 0, 8, ["filter", "velocity"]);
      const pitch = group("Pitch envelope · velocity");
      knob(pitch.knobs, "pamount", "P.AMOUNT", (v) => signed(v, " st", 1), -48, 48, ["pitch", "amount"]);
      knob(pitch.knobs, "pdecay", "P.DECAY", ms, 1, 10000, ["pitch", "decay"], { ...logScale(10000, 1), reset: 100 });
      knob(pitch.knobs, "vlevel", "VEL→LVL", (v) => `${Math.round(v * 100)}%`, 0, 1, ["velocity", "level"], { reset: 1 });
      knob(pitch.knobs, "vstart", "VEL→START", (v) => `${Math.round(v * 100)}%`, 0, 1, ["velocity", "start"]);
      knob(pitch.knobs, "vattack", "VEL→ATK", (v) => `${Math.round(v * 100)}%`, 0, 1, ["velocity", "attack"]);
      // Zones
      const zones = el("div", "sampler-group sampler-zones");
      zones.append(el("h3", "", "Zones · velocity layers and key groups"));
      this.zoneTable = el("div", "sampler-zone-table");
      const addZone = button("+ Layer from library…", "Add a zone with another sample", () => {
        this.layerTarget = this.pad()?.id ?? null;
        this.showLibrary();
        this.hooks.notify("Choose a sample in the Library and press + Layer");
      });
      zones.append(this.zoneTable, addZone);
      box.append(zones);
      // FX, bounce and sequencer
      const fx = el("div", "sampler-group sampler-fx");
      fx.append(el("h3", "", "Pad FX · Bounce · Sequencer"));
      this.fxInfo = el("p", "muted sampler-fx-info");
      this.fxButton = button("Add FX chain", "", () => this.openChain());
      this.fxRemove = button("Remove FX", "Remove this pad's FX chain", () => {
        const chain = this.chainOf();
        if (chain && confirm(`Remove ${chain.name}?`)) this.hooks.change("/api/timbres/remove", { id: chain.id });
      });
      const fxRow = el("div", "sampler-row");
      fxRow.append(this.fxButton, this.fxRemove);
      const bounceRow = el("div", "sampler-row");
      this.bounceLow = this.noteField("From", () => {});
      this.bounceHigh = this.noteField("to", () => {});
      this.bounceStep = this.selectField("Every", [["1", "1 st"], ["2", "2 st"], ["3", "3 st"], ["4", "4 st"], ["6", "6 st"], ["12", "Octave"]], () => {});
      this.bounceVelocity = this.numberField("Velocity", "", () => {}, 1, 127);
      this.bounceHold = this.numberField("Hold s", "Seconds the note is held before its release", () => {}, 0.01, 30);
      this.bounceTail = this.numberField("Tail s", "Most seconds kept after the release", () => {}, 0, 30);
      this.bounceButton = button("Bounce", "Render each note through the pad's FX into new samples, which the pad then plays", () => this.bounce());
      this.bounceButton.className = "light";
      this.unbounceButton = button("Unbounce", "Play the pad live again, through its FX", () => this.editPad((p) => (p.bounce = null)));
      bounceRow.append(this.bounceLow.box, this.bounceHigh.box, this.bounceStep.box, this.bounceVelocity.box, this.bounceHold.box, this.bounceTail.box, this.bounceButton, this.unbounceButton);
      this.bounceProgress = el("progress");
      this.bounceProgress.hidden = true;
      const sequence = button("Sequence this pad", "Add a Neuropolis track that plays this pad", () => {
        const pad = this.pad();
        if (pad) this.hooks.sequence(pad.id, pad.name || `Pad ${label(this.padIndex(pad))}`);
      });
      const seqRow = el("div", "sampler-row");
      seqRow.append(sequence);
      fx.append(this.fxInfo, fxRow, bounceRow, this.bounceProgress, seqRow);
      box.append(fx);
      return box;
    }
    buildRecord() {
      const panel = el("div", "sampler-record");
      panel.hidden = true;
      this.recordSource = el("select");
      this.recordSource.setAttribute("aria-label", "Record from");
      this.recordThreshold = el("select");
      this.recordThreshold.setAttribute("aria-label", "Start at level");
      for (const [value, text] of [["", "Start now"], ["-54", "Wait for −54 dB"], ["-42", "Wait for −42 dB"], ["-30", "Wait for −30 dB"], ["-18", "Wait for −18 dB"]])
        this.recordThreshold.append(new Option(text, value));
      this.recordLength = el("select");
      this.recordLength.setAttribute("aria-label", "Longest take");
      for (const s of [5, 10, 30, 60, 120, 300]) this.recordLength.append(new Option(`up to ${s} s`, s));
      this.recordLength.value = "30";
      this.recordStart = button("● Record", "", () => this.toggleRecord());
      this.recordStart.className = "sampler-record-button";
      this.recordMeter = el("meter");
      this.recordMeter.min = 0;
      this.recordMeter.max = 1;
      this.recordMeter.low = 0.5;
      this.recordMeter.high = 0.9;
      this.recordInfo = el("span", "muted", "Records onto the selected pad.");
      panel.append(this.recordSource, this.recordThreshold, this.recordLength, this.recordStart, this.recordMeter, this.recordInfo);
      return panel;
    }
    async listInputs() {
      const current = this.recordSource.value;
      this.recordSource.replaceChildren(new Option("SAD PET output (resample)", "output"), new Option("Default input", "input:"));
      try {
        for (const device of await navigator.mediaDevices.enumerateDevices())
          if (device.kind === "audioinput" && device.deviceId && device.deviceId !== "default")
            this.recordSource.append(new Option(device.label || "Audio input", `input:${device.deviceId}`));
      } catch {
        /* Inputs are listed by name only after permission. */
      }
      if ([...this.recordSource.options].some((o) => o.value === current)) this.recordSource.value = current;
      else this.recordSource.value = "input:";
    }
    buildLibrary() {
      const panel = el("div", "sampler-library");
      panel.hidden = true;
      const bar = el("div", "sampler-row");
      this.search = Object.assign(el("input"), { type: "search", placeholder: "Search samples" });
      this.search.oninput = () => this.renderLibrary();
      this.packInput = Object.assign(el("input"), { type: "file", hidden: true, accept: ".zip" });
      this.packInput.onchange = async () => {
        const file = this.packInput.files[0];
        this.packInput.value = "";
        if (file) this.importPack(await file.arrayBuffer(), file.name).catch((e) => this.hooks.notify(e.message));
      };
      bar.append(
        this.search,
        button("Save kit as pack", "Name the kit's samples and settings as a pack in this browser", () => this.saveKitPack()),
        button("Export kit (.zip)", "Download the kit and its samples as a pack", () => this.exportPack(null)),
        button("Import pack…", "Add a pack (.zip) to the library", () => this.packInput.click()),
        this.packInput,
      );
      this.packList = el("div", "sampler-pack-list");
      this.sampleList = el("div", "sampler-sample-list");
      this.starterList = el("div", "sampler-pack-list");
      panel.append(bar, el("h3", "", "Starter packs · CC0"), this.starterList, el("h3", "", "Your packs"), this.packList, el("h3", "", "Samples"), this.sampleList);
      return panel;
    }

    // --- Waveform --------------------------------------------------------
    bindCanvas() {
      const canvas = this.canvas;
      const frameAt = (x) => {
        const audio = this.current;
        if (!audio) return 0;
        const [a, b] = this.view;
        return Math.round(clamp((a + (x / canvas.clientWidth) * (b - a)) * audio.frames, 0, audio.frames));
      };
      const xOf = (frame) => {
        const audio = this.current;
        const [a, b] = this.view;
        return ((frame / audio.frames - a) / (b - a)) * canvas.clientWidth;
      };
      const local = (e) => e.clientX - canvas.getBoundingClientRect().left;
      canvas.addEventListener("pointerdown", (e) => {
        const zone = this.zoneOf();
        const audio = this.current;
        if (!zone || !audio) return;
        canvas.focus();
        canvas.setPointerCapture(e.pointerId);
        const x = local(e);
        const near = (frame) => frame !== null && Math.abs(xOf(frame) - x) < 6;
        const end = zone.end || audio.frames;
        let marker = null;
        if (this.mode === "chop") {
          const slice = this.slices.findIndex((s) => near(s));
          if (slice >= 0) {
            if (e.altKey || e.button === 2) {
              this.slices.splice(slice, 1);
              this.draw();
              this.syncModes();
              return;
            }
            marker = { slice };
          } else if (this.chopMethod.input.value === "manual" || e.shiftKey) {
            this.slices.push(this.snapFrame(frameAt(x)));
            this.slices.sort((a, b) => a - b);
            marker = { slice: this.slices.indexOf(this.snapFrame(frameAt(x))) };
            this.draw();
            this.syncModes();
          }
        } else {
          const candidates = [
            ["start", zone.start],
            ["end", end],
            ...(this.mode === "loop" ? [["loop_start", zone.loop_start || zone.start]] : []),
            ...(this.mode === "trim" ? [["fade_in", zone.start + (zone.fade_in * audio.rate) / 1000], ["fade_out", end - (zone.fade_out * audio.rate) / 1000]] : []),
          ];
          const hit = candidates.find(([, frame]) => near(frame));
          if (hit) marker = { key: hit[0] };
        }
        if (!marker) {
          const frame = frameAt(x);
          marker = { select: frame };
          this.selection = [frame, frame];
        }
        this.dragging = marker;
        this.draw();
      });
      canvas.addEventListener("pointermove", (e) => {
        const drag = this.dragging;
        const zone = this.zoneOf();
        const audio = this.current;
        if (!drag || !zone || !audio) {
          if (audio) canvas.title = `${(frameAt(local(e)) / audio.rate).toFixed(3)} s`;
          return;
        }
        const frame = frameAt(local(e));
        const end = zone.end || audio.frames;
        if (drag.select !== undefined) {
          this.selection = [Math.min(drag.select, frame), Math.max(drag.select, frame)];
        } else if (drag.slice !== undefined) {
          this.slices[drag.slice] = this.snapFrame(frame);
        } else if (drag.key === "start") zone.start = clamp(this.snapFrame(frame), 0, end - 1);
        else if (drag.key === "end") zone.end = clamp(this.snapFrame(frame), zone.start + 1, audio.frames);
        else if (drag.key === "loop_start") zone.loop_start = clamp(this.snapFrame(frame), zone.start, end - 1);
        else if (drag.key === "fade_in") zone.fade_in = Math.round(clamp(((frame - zone.start) / audio.rate) * 1000, 0, 60000));
        else if (drag.key === "fade_out") zone.fade_out = Math.round(clamp(((end - frame) / audio.rate) * 1000, 0, 60000));
        drag.moved = true;
        this.syncModes();
        this.draw();
      });
      const end = () => {
        const drag = this.dragging;
        this.dragging = null;
        if (!drag) return;
        if (drag.select !== undefined && this.selection && this.selection[1] - this.selection[0] < 2) this.selection = null;
        if (drag.slice !== undefined) this.slices.sort((a, b) => a - b);
        if (drag.key && drag.moved) this.hooks.structure();
        this.syncModes();
        this.draw();
      };
      canvas.addEventListener("pointerup", end);
      canvas.addEventListener("pointercancel", end);
      canvas.addEventListener("contextmenu", (e) => e.preventDefault());
      canvas.addEventListener("dblclick", () => {
        this.view = [0, 1];
        this.selection = null;
        this.draw();
      });
      canvas.addEventListener("wheel", (e) => {
        if (!this.current) return;
        e.preventDefault();
        const [a, b] = this.view;
        if (e.shiftKey || Math.abs(e.deltaX) > Math.abs(e.deltaY)) {
          const shift = ((e.deltaX || e.deltaY) / canvas.clientWidth) * (b - a);
          const span = b - a;
          const start = clamp(a + shift, 0, 1 - span);
          this.view = [start, start + span];
          this.draw();
        } else this.zoom(e.deltaY > 0 ? 1.25 : 0.8, a + (local(e) / canvas.clientWidth) * (b - a));
      }, { passive: false });
      canvas.addEventListener("keydown", (e) => {
        if ((e.key === "Delete" || e.key === "Backspace") && this.selection && this.mode === "process") {
          e.preventDefault();
          this.process({ op: "delete", start: this.selection[0], end: this.selection[1] });
        } else if (e.key === "Escape") {
          this.selection = null;
          this.draw();
        }
      });
      this.root.addEventListener("drop", (e) => {
        if (!e.target.closest?.(".sampler-canvas")) return;
        e.preventDefault();
        const files = [...(e.dataTransfer?.files || [])];
        if (files.length) this.importFiles(files.slice(0, 1)).catch((err) => this.hooks.notify(err.message));
      });
      new ResizeObserver(() => this.draw()).observe(canvas);
    }
    snapFrame(frame) {
      const audio = this.current;
      if (!audio || !this.snap.checked) return frame;
      const radius = Math.round(audio.rate / 200);
      const value = (i) => audio.left[i] + audio.right[i];
      for (let d = 0; d <= radius; d++)
        for (const i of [frame - d, frame + d])
          if (i > 0 && i < audio.frames && (value(i - 1) <= 0) !== (value(i) <= 0)) return i;
      return frame;
    }
    /** Loads the selected zone's audio, then paints. */
    async showZone() {
      const zone = this.zoneOf();
      const id = zone?.sample || null;
      if (id !== this.currentId) {
        this.currentId = id;
        this.current = null;
        this.draw();
        if (!id) return;
        try {
          const audio = await this.audio(id);
          if (this.currentId === id) {
            this.current = audio;
            this.syncModes();
          }
        } catch (error) {
          this.hooks.notify(error.message);
        }
      }
      this.draw();
    }
    draw() {
      const canvas = this.canvas;
      const width = Math.max(1, canvas.clientWidth),
        height = Math.max(1, canvas.clientHeight);
      const ratio = window.devicePixelRatio || 1;
      for (const c of [canvas, this.overlay]) {
        if (c.width !== Math.round(width * ratio) || c.height !== Math.round(height * ratio)) {
          c.width = Math.round(width * ratio);
          c.height = Math.round(height * ratio);
        }
      }
      const g = canvas.getContext("2d");
      g.setTransform(ratio, 0, 0, ratio, 0, 0);
      g.clearRect(0, 0, width, height);
      const audio = this.current;
      const zone = this.zoneOf();
      this.empty.hidden = !!(audio || zone);
      if (!audio || !zone) {
        this.readout.textContent = zone ? "Loading…" : "";
        this.drawOverlay();
        return;
      }
      const [a, b] = this.view;
      const first = a * audio.frames,
        span = (b - a) * audio.frames;
      const x = (frame) => ((frame - first) / span) * width;
      const end = zone.end || audio.frames;
      // Outside the region is dimmed.
      g.fillStyle = "rgba(255,255,255,0.03)";
      g.fillRect(x(zone.start), 0, x(end) - x(zone.start), height);
      if (zone.loop_mode !== "off") {
        g.fillStyle = "rgba(140,220,209,0.09)";
        g.fillRect(x(zone.loop_start), 0, x(end) - x(zone.loop_start), height);
      }
      // The accent of the module the editor stands in.
      const accent = getComputedStyle(g.canvas).getPropertyValue("--accent").trim() || "#c4f486";
      if (this.selection) {
        g.fillStyle = `${accent}29`;
        g.fillRect(x(this.selection[0]), 0, x(this.selection[1]) - x(this.selection[0]), height);
      }
      const half = height / 4;
      for (const [channel, center] of [[audio.left, height / 4], [audio.right, (height * 3) / 4]]) {
        g.strokeStyle = "rgba(255,255,255,0.08)";
        g.beginPath();
        g.moveTo(0, center);
        g.lineTo(width, center);
        g.stroke();
        g.fillStyle = "#b6d5c8";
        const per = span / width;
        for (let px = 0; px < width; px++) {
          const from = Math.floor(first + px * per),
            to = Math.max(from + 1, Math.floor(first + (px + 1) * per));
          let lo = 1,
            hi = -1;
          const step = Math.max(1, Math.floor((to - from) / 256));
          for (let i = Math.max(0, from); i < Math.min(audio.frames, to); i += step) {
            const v = channel[i];
            if (v < lo) lo = v;
            if (v > hi) hi = v;
          }
          if (hi < lo) continue;
          const inside = from >= zone.start && from < end;
          g.globalAlpha = inside ? 1 : 0.35;
          g.fillRect(px, center - hi * half * 0.95, 1, Math.max(1, (hi - lo) * half * 0.95));
        }
        g.globalAlpha = 1;
      }
      // Fades as ramps over the region's edges.
      const fadeIn = (zone.fade_in * audio.rate) / 1000,
        fadeOut = (zone.fade_out * audio.rate) / 1000;
      g.strokeStyle = accent;
      g.lineWidth = 1.5;
      if (fadeIn > 0 || fadeOut > 0) {
        g.beginPath();
        g.moveTo(x(zone.start), fadeIn > 0 ? height : 0);
        const steps = 24;
        for (let i = 0; i <= steps && fadeIn > 0; i++) {
          const t = i / steps;
          const shape = t ** (4 ** (zone.fade_curve?.[0] || 0));
          g.lineTo(x(zone.start + fadeIn * t), height - shape * height);
        }
        g.lineTo(x(end - fadeOut), 0);
        for (let i = 0; i <= steps && fadeOut > 0; i++) {
          const t = 1 - i / steps;
          const shape = t ** (4 ** (zone.fade_curve?.[1] || 0));
          g.lineTo(x(end - fadeOut * t), height - shape * height);
        }
        g.stroke();
      }
      const marker = (frame, color, text, top = true) => {
        const px = x(frame);
        if (px < -2 || px > width + 2) return;
        g.fillStyle = color;
        g.fillRect(Math.round(px) - 1, 0, 2, height);
        g.font = "10px Inter, sans-serif";
        g.fillText(text, Math.round(px) + 3, top ? 11 : height - 4);
      };
      marker(zone.start, accent, "S");
      marker(end, accent, "E");
      if (zone.loop_mode !== "off") marker(zone.loop_start, "#8cdcd1", "L", false);
      if (zone.crossfade > 0 && zone.loop_mode !== "off") {
        g.fillStyle = "rgba(140,220,209,0.2)";
        const xf = (zone.crossfade * audio.rate) / 1000;
        g.fillRect(x(end - xf), 0, x(end) - x(end - xf), height);
      }
      this.slices.forEach((slice, i) => marker(slice, "#efb4a9", String(i + 1), false));
      const seconds = (f) => (f / audio.rate).toFixed(3);
      this.readout.textContent = this.selection
        ? `Selection ${seconds(this.selection[0])}–${seconds(this.selection[1])} s · ${this.selection[1] - this.selection[0]} frames`
        : `${seconds(end - zone.start)} s of ${seconds(audio.frames)} s · ${audio.rate} Hz`;
      this.drawOverlay();
    }
    drawOverlay() {
      const overlay = this.overlay;
      const g = overlay.getContext("2d");
      const ratio = window.devicePixelRatio || 1;
      const width = overlay.width / ratio,
        height = overlay.height / ratio;
      g.setTransform(ratio, 0, 0, ratio, 0, 0);
      g.clearRect(0, 0, width, height);
      const audio = this.current,
        pad = this.pad();
      if (!audio || !pad) return;
      const [a, b] = this.view;
      g.fillStyle = "#ffffff";
      for (const [id, zone, frame] of this.voices) {
        if (id !== pad.id || (zone !== this.zone && !pad.bounce)) continue;
        if (pad.bounce) continue;
        const px = ((frame / audio.frames - a) / (b - a)) * width;
        g.fillRect(px, 0, 1.5, height);
      }
    }

    // --- Modes ----------------------------------------------------------
    editZone(edit) {
      const zone = this.zoneOf();
      if (!zone) return;
      edit(zone);
      this.hooks.structure();
      this.syncModes();
      this.draw();
    }
    syncModes() {
      for (const tab of this.modeTabs.children) tab.setAttribute("aria-selected", String(tab.dataset.mode === this.mode));
      this.trimRow.hidden = this.mode !== "trim";
      this.loopRow.hidden = this.mode !== "loop";
      this.chopRow.hidden = this.mode !== "chop";
      this.processRow.hidden = this.mode !== "process";
      const zone = this.zoneOf();
      const frames = this.current?.frames || 0;
      const set = (field, value) => {
        if (document.activeElement !== field.input) field.input.value = value;
      };
      if (zone) {
        set(this.startInput, zone.start);
        set(this.endInput, zone.end || frames);
        set(this.fadeIn, Math.round(zone.fade_in));
        set(this.fadeOut, Math.round(zone.fade_out));
        set(this.fadeInCurve, zone.fade_curve?.[0] ?? 0);
        set(this.fadeOutCurve, zone.fade_curve?.[1] ?? 0);
        this.reverse.input.checked = !!zone.reverse;
        set(this.rootSelect, zone.root);
        set(this.loopMode, zone.loop_mode);
        set(this.loopStart, zone.loop_start);
        set(this.crossfade, Math.round(zone.crossfade));
      }
      const method = this.chopMethod.input.value;
      this.sensitivity.box.hidden = method !== "transients";
      this.regions.box.hidden = method !== "regions";
      this.chopBpm.box.hidden = this.chopBeats.box.hidden = method !== "beats";
      this.sliceCount.textContent = this.slices.length ? `${this.slices.length} slices` : "No slices";
      this.undoButton.disabled = !this.history.some((h) => h.pad === this.pad()?.id && h.zone === this.zone);
    }
    syncOperation() {
      const [, , fields] = OPERATIONS.find(([id]) => id === this.operation.input.value);
      this.operationFields.replaceChildren(
        ...fields.map(([key, text, value, min, max]) => {
          if (key === "keep_length") {
            const field = this.checkField(text, () => {});
            field.input.checked = !!value;
            field.input.dataset.key = key;
            return field.box;
          }
          const field = this.numberField(text, "", () => {}, min, max);
          field.input.value = value;
          field.input.dataset.key = key;
          return field.box;
        }),
      );
    }
    applyOperation() {
      const op = this.operation.input.value;
      const zone = this.zoneOf();
      if (!zone || !this.current) return;
      const request = { op };
      for (const input of this.operationFields.querySelectorAll("input")) {
        const key = input.dataset.key;
        request[key] = input.type === "checkbox" ? input.checked : Number(input.value);
      }
      if ("bits" in request) request.bits = Math.round(request.bits);
      if (op === "resample") request.rate = Math.round(request.rate);
      if (["delete", "silence"].includes(op) && !this.selection)
        return this.hooks.notify("Drag over the waveform to select first");
      // Without a selection an edit works on the zone's region, as on a slice.
      const end = zone.end || this.current.frames;
      const [from, to] = this.selection || [zone.start, end];
      const regional = ["crop", "delete", "silence", "normalize", "gain", "reverse", "fade_in", "fade_out", "remove_dc", "invert"];
      if (regional.includes(op)) Object.assign(request, { start: from, end: to });
      if (op === "insert_silence") request.at = this.selection?.[0] ?? zone.start;
      if (op === "crop") return this.process(request, { region: "full" });
      if (op === "delete") return this.process(request, { region: "shift", at: from, removed: to - from });
      if (op === "insert_silence") return this.process(request, { region: "shift", at: request.at });
      // Length changes of the whole region: the region becomes the sample.
      if (["pitch", "stretch", "resample", "trim_silence"].includes(op))
        return this.process(request, { region: "full", source: "region" });
      this.process(request, { region: "keep" });
    }
    /** A destructive edit of the zone's sample: a new library sample. The
     * zone keeps its region, takes the whole new sample, or has its end
     * shifted by what was removed or inserted at `at`. */
    async process(request, { region = "keep", source = "sample", at = 0, removed, after, clip } = {}) {
      const pad = this.pad(),
        zone = this.zoneOf(pad);
      if (!zone?.sample) return;
      const index = this.zone;
      try {
        this.status = "Processing…";
        this.syncStatus();
        const before = (await this.audio(zone.sample)).frames;
        const wav = source === "region" ? await this.regionWav(zone) : await this.wavOf(zone.sample);
        const response = await this.hooks.api("/api/sampler/process", { request, wav, clip }, { "X-Sampler": "1" });
        const result = await response.arrayBuffer();
        const original = await this.library.get(zone.sample);
        const name = (original?.name || this.current?.name || "Sample").replace(/ · edit \d+$/, "");
        const count = this.history.filter((h) => h.pad === pad.id).length + 1;
        const record = await this.store(`${name} · edit ${count}`, result, "edit", { parent: zone.sample });
        this.history.push({ pad: pad.id, zone: index, sample: zone.sample, start: zone.start, end: zone.end, loop_start: zone.loop_start });
        this.editZone((z) => {
          const end = z.end || before;
          z.sample = record.id;
          if (region === "full") {
            z.start = 0;
            z.end = record.frames;
            z.loop_start = 0;
          } else if (region === "shift") {
            const delta = removed !== undefined ? -removed : record.frames - before;
            const move = (frame) => (frame > at ? Math.max(at, frame + delta) : frame);
            z.end = end >= before ? record.frames : clamp(move(end), z.start + 1, record.frames);
            z.loop_start = clamp(move(z.loop_start), z.start, z.end - 1);
          } else {
            z.end = Math.min(end >= before ? record.frames : end, record.frames);
          }
          after?.(z);
        });
        this.selection = null;
        this.status = "";
        await this.showZone();
      } catch (error) {
        this.status = "";
        this.hooks.notify(error.message);
      }
      this.syncStatus();
      this.syncModes();
    }
    undoEdit() {
      const pad = this.pad();
      const at = this.history.findLastIndex((h) => h.pad === pad?.id && h.zone === this.zone);
      if (at < 0) return;
      const [step] = this.history.splice(at, 1);
      this.editZone((z) => {
        z.sample = step.sample;
        z.start = step.start;
        z.end = step.end;
        z.loop_start = step.loop_start;
      });
      this.showZone();
    }
    async copySelection() {
      const zone = this.zoneOf();
      if (!zone || !this.selection) return this.hooks.notify("Select audio to copy");
      const wav = await this.wavOf(zone.sample);
      const response = await this.hooks.api("/api/sampler/process", { request: { op: "crop", start: this.selection[0], end: this.selection[1] }, wav }, { "X-Sampler": "1" });
      this.clipboard = await response.arrayBuffer();
      this.hooks.notify("Copied");
    }
    pasteClip(mix) {
      if (!this.clipboard) return this.hooks.notify("Copy audio first");
      const at = this.selection?.[0] ?? 0;
      this.process({ op: "paste", at, mix }, { clip: this.clipboard.slice(0), region: mix ? "keep" : "shift", at });
    }
    async detectRoot() {
      const zone = this.zoneOf();
      if (!zone) return;
      try {
        const wav = await this.regionWav(zone);
        const response = await this.hooks.api("/api/sampler/analyze", { request: { kind: "root" }, wav }, { "X-Sampler": "1" });
        const note = await response.json();
        if (note === null) return this.hooks.notify("No clear pitch in this sample");
        const root = clamp(Math.round(note), 0, 127);
        const cents = Math.round((note - root) * 100);
        this.editZone((z) => {
          z.root = root;
          z.fine = -cents;
        });
        this.hooks.notify(`Root ${noteName(root)} ${cents ? signed(cents, " ct") : ""}`);
      } catch (error) {
        this.hooks.notify(error.message);
      }
    }
    /** The zone's region as a WAV, for analysis. */
    async regionWav(zone) {
      const audio = await this.audio(zone.sample);
      const end = zone.end || audio.frames;
      return Files.encodeWav(audio.rate, audio.left.subarray(zone.start, end), audio.right.subarray(zone.start, end));
    }
    async chop() {
      const zone = this.zoneOf();
      if (!zone) return;
      const method = this.chopMethod.input.value;
      if (method === "manual") return this.hooks.notify("Click the waveform to add slices; Alt-click removes one");
      try {
        const wav = await this.regionWav(zone);
        const request = method === "transients"
          ? { kind: "transients", sensitivity: Number(this.sensitivity.input.value), gap: 60 }
          : method === "regions"
            ? { kind: "divide", count: Number(this.regions.input.value) }
            : { kind: "beats", bpm: Number(this.chopBpm.input.value), beats: Number(this.chopBeats.input.value) };
        const response = await this.hooks.api("/api/sampler/analyze", { request, wav }, { "X-Sampler": "1" });
        this.slices = (await response.json()).map((s) => s + zone.start).slice(0, 128);
        this.syncModes();
        this.draw();
      } catch (error) {
        this.hooks.notify(error.message);
      }
    }
    /** Slice regions: each slice to the next, the last to the zone's end. */
    sliceRegions() {
      const zone = this.zoneOf();
      const end = zone.end || this.current?.frames || 0;
      const starts = [...new Set(this.slices.filter((s) => s >= zone.start && s < end))].sort((a, b) => a - b);
      if (!starts.length || starts[0] > zone.start) starts.unshift(zone.start);
      return starts.map((s, i) => [s, starts[i + 1] ?? end]).filter(([a, b]) => b - a > 1);
    }
    async slicesToPads(destructive) {
      const pad = this.pad(),
        zone = this.zoneOf(pad);
      if (!zone || this.slices.length < 1) return this.hooks.notify("Find or add slices first");
      const regions = this.sliceRegions();
      const kit = this.kit();
      const from = this.padIndex(pad);
      const used = regions.filter((_, i) => kit.pads[from + i]?.zones.length && from + i !== from).length;
      if (used && !confirm(`${used} pads after this one already have samples. Replace them?`)) return;
      const base = (await this.library.get(zone.sample))?.name || pad.name || "Slice";
      const samples = [];
      for (const [i, [start, end]] of regions.entries()) {
        if (destructive) {
          const wav = await this.wavOf(zone.sample);
          const response = await this.hooks.api("/api/sampler/process", { request: { op: "crop", start, end }, wav }, { "X-Sampler": "1" });
          const record = await this.store(`${base} ${i + 1}`, await response.arrayBuffer(), "slice", { parent: zone.sample });
          samples.push({ sample: record.id, start: 0, end: record.frames });
        } else samples.push({ sample: zone.sample, start, end });
      }
      this.change((k) => {
        this.ensurePads(k, Math.min(127, from + regions.length - 1));
        samples.forEach((s, i) => {
          const target = k.pads[from + i];
          if (!target) return;
          const copy = structuredClone(zone);
          Object.assign(copy, s, { loop_mode: "off", loop_start: s.start, fade_in: Math.min(zone.fade_in, 2), fade_out: Math.max(zone.fade_out, 3) });
          target.zones = [copy];
          target.bounce = null;
          target.name = `${base} ${i + 1}`.slice(0, 80);
        });
      });
      this.slices = [];
      this.sync();
      this.hooks.notify(`${regions.length} slices on pads ${label(from)}–${label(from + regions.length - 1)}`);
    }
    slicesToKeys() {
      const pad = this.pad(),
        zone = this.zoneOf(pad);
      if (!zone || !this.slices.length) return this.hooks.notify("Find or add slices first");
      const regions = this.sliceRegions().slice(0, 128 - pad.keys[0]);
      const base = pad.keys[0];
      this.editPad((p) => {
        p.zones = regions.map(([start, end], i) => ({
          ...structuredClone(zone), start, end, loop_mode: "off", loop_start: start,
          keys: [base + i, base + i], root: base + i, velocities: [1, 127],
        }));
        p.keys = [base, base + regions.length - 1];
        p.keytrack = true;
        p.bounce = null;
      });
      this.slices = [];
      this.zone = 0;
      this.sync();
      this.hooks.notify(`${regions.length} slices on ${noteName(base)}–${noteName(base + regions.length - 1)}`);
    }

    // --- FX, bounce ------------------------------------------------------
    chainOf(pad = this.pad()) {
      return this.chains.find((c) => c.pad === pad?.id) || null;
    }
    openChain() {
      const pad = this.pad();
      if (!pad) return;
      this.hooks.change("/api/sampler/chain", { pad: pad.id });
    }
    bounceOptions(pad) {
      const options = pad.bounce?.options || {};
      const keys = pad.keytrack ? [Math.max(pad.keys[0], 24), Math.min(pad.keys[1], 96)] : [pad.keys[0], pad.keys[0]];
      return {
        low: options.low ?? keys[0], high: options.high ?? keys[1], step: options.step ?? (pad.keytrack ? 3 : 1),
        velocity: options.velocity ?? 127, hold: options.hold ?? 1, tail: options.tail ?? 4,
      };
    }
    async bounce() {
      const pad = this.pad();
      const part = this.part();
      if (!pad?.zones.length || part === null) return this.hooks.notify("Put a sample on the pad first");
      await this.hooks.flush?.();
      const options = {
        low: Number(this.bounceLow.input.value), high: Number(this.bounceHigh.input.value),
        step: Number(this.bounceStep.input.value), velocity: clamp(Number(this.bounceVelocity.input.value) || 127, 1, 127),
        hold: clamp(Number(this.bounceHold.input.value) || 1, 0.01, 30), tail: clamp(Number(this.bounceTail.input.value) || 0, 0, 30),
      };
      if (options.high < options.low) [options.low, options.high] = [options.high, options.low];
      const notes = [];
      for (let n = options.low; n <= options.high; n += options.step) notes.push(n);
      const name = pad.name || label(this.padIndex(pad));
      this.bounceButton.disabled = true;
      this.bounceProgress.hidden = false;
      this.bounceProgress.max = notes.length;
      this.bounceProgress.value = 0;
      const rendered = [];
      try {
        for (const [i, note] of notes.entries()) {
          this.status = `Bouncing ${name}: ${noteName(note)} (${i + 1}/${notes.length})`;
          this.syncStatus();
          const response = await this.hooks.api("/api/sampler/bounce", { part, pad: pad.id, note, options, adopt: i === 0 });
          const record = await this.store(`${name} · ${noteName(note)}`, await response.arrayBuffer(), "bounce", { pad: pad.id, note });
          rendered.push([note, record.id]);
          this.bounceProgress.value = i + 1;
        }
        const zones = rendered.map(([note, sample], i) => ({
          ...this.zoneFor(sample, 0), root: note,
          keys: [i === 0 ? 0 : note, rendered[i + 1] ? rendered[i + 1][0] - 1 : 127],
        }));
        this.editPad((p) => (p.bounce = { zones, options }));
        this.hooks.notify(`${name} bounced: ${rendered.length} note${rendered.length === 1 ? "" : "s"}`);
      } catch (error) {
        this.hooks.notify(error.message);
      } finally {
        this.status = "";
        this.bounceButton.disabled = false;
        this.bounceProgress.hidden = true;
        this.syncStatus();
      }
    }

    // --- Recording -------------------------------------------------------
    async toggleRecord() {
      if (this.stopping) return;
      try {
        if (this.recording) {
          this.stopping = true;
          const response = await this.hooks.api("/api/sampler/record", { active: false });
          this.recording = false;
          this.syncRecord();
          const wav = await response.arrayBuffer();
          this.takes = (this.takes || 0) + 1;
          const record = await this.store(`Take ${this.takes}`, wav, "record");
          this.assign(Math.max(0, this.padIndex()), record);
          this.mode = "chop";
          this.syncModes();
          return;
        }
        const source = this.recordSource.value || "input:";
        const input = source.startsWith("input:");
        await this.hooks.request("/api/sampler/record", {
          active: true,
          source: input ? "input" : "output",
          device: input ? source.slice(6) : null,
          threshold: this.recordThreshold.value === "" ? null : Number(this.recordThreshold.value),
          seconds: Number(this.recordLength.value),
        });
        this.recording = true;
        this.syncRecord();
        if (input) this.listInputs();
      } catch (error) {
        this.recording = false;
        this.syncRecord();
        this.hooks.notify(error.message);
      } finally {
        this.stopping = false;
      }
    }
    syncRecord(status) {
      this.recordStart.textContent = this.recording ? "■ Stop" : "● Record";
      this.recordStart.setAttribute("aria-pressed", String(!!this.recording));
      this.recordButton.classList.toggle("recording", !!this.recording);
      if (status) {
        this.recordMeter.value = Math.min(1, status.peak);
        this.recordInfo.textContent = status.armed ? "Waiting for the level…" : `${status.seconds.toFixed(1)} s`;
        // The take reached its length on its own.
        if (this.recording && !status.active) this.toggleRecord();
      } else if (!this.recording) this.recordInfo.textContent = `Records onto pad ${label(Math.max(0, this.padIndex()))}.`;
    }

    // --- Library & packs ---------------------------------------------------
    showLibrary(show = true) {
      this.libraryPanel.hidden = !show;
      this.libraryButton.setAttribute("aria-pressed", String(show));
      if (show) this.refreshLibrary();
    }
    async refreshLibrary() {
      if (this.libraryPanel.hidden) return;
      [this.samples, this.packs] = await Promise.all([this.library.samples(), this.library.packs()]);
      this.renderLibrary();
    }
    renderLibrary() {
      if (!this.samples) return;
      const query = this.search.value.trim().toLocaleLowerCase();
      const rows = this.samples.filter((s) => !query || s.name.toLocaleLowerCase().includes(query)).slice(0, 300);
      this.sampleList.replaceChildren(
        ...rows.map((sample) => {
          const row = el("div", "sampler-sample");
          row.draggable = true;
          row.ondragstart = (e) => e.dataTransfer.setData("application/x-sad-sample", sample.id);
          const name = el("span", "sampler-sample-name", sample.name);
          name.title = `${sample.origin || "sample"} · ${new Date(sample.createdAt).toLocaleString()}`;
          const info = el("span", "muted", sample.frames ? `${(sample.frames / sample.rate).toFixed(2)} s · ${(sample.rate / 1000).toFixed(1)} kHz` : "");
          row.append(
            button("▶", "Listen", () => this.preview(sample.id)),
            name,
            info,
            button("→ Pad", "Put on the selected pad", async () => this.assign(Math.max(0, this.padIndex()), await this.library.get(sample.id))),
            button("+ Layer", "Add as another zone of the selected pad", () => this.addLayer(sample)),
            button("↓", "Download WAV", async () => this.hooks.download((await this.library.get(sample.id)).wav, `${sample.name}.wav`)),
            button("×", "Delete from the library", async () => {
              const used = this.kit()?.pads.some((p) => p.zones.some((z) => z.sample === sample.id));
              if (used && !confirm(`${sample.name} plays on a pad. Delete it from the library anyway?`)) return;
              await this.library.remove(sample.id);
            }),
          );
          return row;
        }),
      );
      if (!rows.length) this.sampleList.append(el("p", "muted", "No samples yet: import, record, slice or bounce."));
      this.packList.replaceChildren(
        ...(this.packs || []).map((pack) => {
          const row = el("div", "sampler-sample");
          row.append(
            el("span", "sampler-sample-name", pack.name),
            el("span", "muted", `${pack.samples.length} samples${pack.kit ? " · kit" : ""}`),
            button("Load", "Load the pack's kit, or put its samples on the pads", () => this.loadPack(pack)),
            button("↓ .zip", "Download the pack", () => this.exportPack(pack)),
            button("×", "Delete the pack (its samples stay)", async () => {
              if (confirm(`Delete pack ${pack.name}?`)) await this.library.removePack(pack.id);
            }),
          );
          return row;
        }),
      );
      this.bundled().then((packs) => this.starterList.replaceChildren(...packs.map((entry) => {
        const row = el("div", "sampler-sample");
        const name = el("span", "sampler-sample-name", entry.name);
        name.title = entry.description;
        row.append(name, el("span", "muted", `${entry.pads} pad${entry.pads === 1 ? "" : "s"} · ${(entry.size / 1e6).toFixed(1)} MB`),
          button("Load", entry.description, () => this.loadBundled(entry)));
        return row;
      })));
      if (!this.packs?.length) this.packList.append(el("p", "muted", "No packs: save the kit as one, or import a .zip."));
    }
    async preview(id) {
      this.previewAudio?.pause();
      const record = await this.library.get(id);
      if (!record) return;
      URL.revokeObjectURL(this.previewUrl || "");
      this.previewUrl = URL.createObjectURL(record.wav);
      this.previewAudio = new Audio(this.previewUrl);
      this.previewAudio.play().catch((e) => this.hooks.notify(e.message));
    }
    addLayer(sample) {
      const pad = this.kit()?.pads.find((p) => p.id === (this.layerTarget ?? this.kit().selected));
      if (!pad) return;
      this.change(() => {
        pad.zones.push(this.zoneFor(sample.id, sample.frames));
        // Layers split the velocity range evenly.
        const n = pad.zones.length;
        pad.zones.forEach((z, i) => (z.velocities = [Math.round((i * 127) / n) + 1, Math.round(((i + 1) * 127) / n)]));
        if (!pad.name) pad.name = sample.name.slice(0, 80);
      });
      this.layerTarget = null;
      this.zone = pad.zones.length - 1;
      this.sync();
    }
    async saveKitPack() {
      const kit = this.kit();
      if (!kit) return;
      const name = prompt("Pack name", this.hooks.context()?.timbres.find((t) => t.id === this.part())?.name || "Kit");
      if (!name) return;
      const samples = [...new Set(kit.pads.flatMap((p) => [...p.zones, ...(p.bounce?.zones || [])].map((z) => z.sample)).filter(Boolean))];
      for (const id of samples) await this.wavOf(id).then(async (wav) => !(await this.library.get(id)) && this.library.put({ id, name: id, wav }));
      await this.library.putPack({ id: newId().replace("smp", "pack"), name: name.trim().slice(0, 80), samples, kit: structuredClone(kit) });
      this.hooks.notify(`Saved pack ${name}`);
      this.showLibrary();
    }
    async exportPack(pack) {
      try {
        const kit = pack ? pack.kit : this.kit();
        const ids = pack ? pack.samples : [...new Set(kit.pads.flatMap((p) => [...p.zones, ...(p.bounce?.zones || [])].map((z) => z.sample)).filter(Boolean))];
        const names = new Map();
        const files = [];
        const used = new Set();
        for (const id of ids) {
          const record = await this.library.get(id);
          const wav = record?.wav || (await this.wavOf(id));
          let file = (record?.name || id).replace(/[\\/:*?"<>|]+/g, "_").slice(0, 60) || "sample";
          let n = 1;
          while (used.has(file.toLowerCase())) file = `${record?.name || "sample"} ${++n}`;
          used.add(file.toLowerCase());
          names.set(id, { id, name: record?.name || id, file: `samples/${file}.wav` });
          files.push({ name: `samples/${file}.wav`, data: new Uint8Array(await wav.arrayBuffer()) });
        }
        const name = pack?.name || this.hooks.context()?.timbres.find((t) => t.id === this.part())?.name || "Kit";
        const manifest = { format: "sad-pet-pack-v1", name, kit: kit || null, samples: [...names.values()] };
        files.unshift({ name: "pack.json", data: new TextEncoder().encode(JSON.stringify(manifest, null, 1)) });
        this.hooks.download(Files.zip(files), `${name}.sadpack.zip`);
      } catch (error) {
        this.hooks.notify(error.message);
      }
    }
    async importPack(buffer, fileName) {
      const files = await Files.unzip(buffer);
      const manifestFile = files.find((f) => f.name.split("/").pop() === "pack.json");
      const manifest = manifestFile ? JSON.parse(new TextDecoder().decode(manifestFile.data)) : null;
      const ids = [];
      const byFile = new Map(files.map((f) => [f.name, f]));
      const entries = manifest?.samples?.length
        ? manifest.samples.map((s) => ({ ...s, data: byFile.get(s.file)?.data }))
        : files.filter((f) => /\.(wav|aiff?|flac|mp3|ogg|m4a)$/i.test(f.name)).map((f) => ({ id: newId(), name: stem(f.name.split("/").pop()), data: f.data }));
      for (const entry of entries) {
        if (!entry.data) continue;
        let wav = entry.data.buffer.slice(entry.data.byteOffset, entry.data.byteOffset + entry.data.byteLength);
        const parsed = Files.parseWav(wav);
        const audio = parsed || (await Files.decodeAudio(wav));
        if (!parsed) wav = Files.encodeWav(audio.rate, audio.left, audio.right);
        const id = entry.id && !(await this.library.get(entry.id)) ? entry.id : entry.id || newId();
        if (!(await this.library.get(id)))
          await this.library.put({ id, name: entry.name || "Sample", wav: new Blob([wav], { type: "audio/wav" }), rate: audio.rate, frames: audio.frames, channels: audio.channels, origin: "pack" });
        ids.push(id);
      }
      if (!ids.length) throw new Error(`${fileName}: no audio found`);
      const pack = { id: newId().replace("smp", "pack"), name: manifest?.name || stem(fileName.replace(/\.sadpack\.zip$/i, ".zip")), samples: ids, kit: manifest?.kit || null };
      await this.library.putPack(pack);
      this.hooks.notify(`Pack ${pack.name}: ${ids.length} samples`);
      if (this.kit() && confirm(`Load ${pack.name} onto the pads now?`)) await this.loadPack(pack);
      this.showLibrary();
    }
    /** The starter packs that come with SAD PET, from the site's own files. */
    bundled() {
      this.bundledList ??= fetch(new URL("./sampler-packs/packs.json", location.href))
        .then((r) => (r.ok ? r.json() : []))
        .catch(() => []);
      return this.bundledList;
    }
    /** Copies a starter pack's samples into the library once, then loads its kit. */
    async loadBundled(entry) {
      const base = new URL(`./sampler-packs/${entry.id}/`, location.href);
      try {
        const response = await fetch(new URL(`pack.json?v=${entry.version}`, base));
        if (!response.ok) throw new Error(`${entry.name}: ${response.status}`);
        const manifest = await response.json();
        const ids = [];
        for (const [i, sample] of manifest.samples.entries()) {
          ids.push(sample.id);
          if (await this.library.get(sample.id)) continue;
          this.status = `Loading ${entry.name}: ${i + 1}/${manifest.samples.length}`;
          this.syncStatus();
          const file = await fetch(new URL(`${sample.file}?v=${entry.version}`, base));
          if (!file.ok) throw new Error(`${sample.name}: ${file.status}`);
          const wav = await file.arrayBuffer();
          const audio = Files.parseWav(wav);
          await this.library.put({
            id: sample.id, name: sample.name, wav: new Blob([wav], { type: "audio/wav" }),
            rate: audio.rate, frames: audio.frames, channels: audio.channels, origin: "starter", pack: entry.id,
          });
        }
        await this.loadPack({ kit: manifest.kit, samples: ids });
        this.hooks.notify(`${entry.name}: ${manifest.kit.pads.length} pad${manifest.kit.pads.length === 1 ? "" : "s"}`);
      } catch (error) {
        this.hooks.notify(error.message);
      } finally {
        this.status = "";
        this.syncStatus();
      }
    }
    async loadPack(pack) {
      if (pack.kit) {
        this.change((kit) => {
          const loaded = structuredClone(pack.kit);
          kit.pads = loaded.pads;
          kit.polyphony = loaded.polyphony ?? kit.polyphony;
          kit.selected = loaded.selected ?? loaded.pads[0]?.id ?? 1;
        });
      } else {
        const records = (await Promise.all(pack.samples.map((id) => this.library.get(id)))).filter(Boolean);
        this.change((kit) => {
          this.ensurePads(kit, records.length - 1);
          records.forEach((record, i) => {
            kit.pads[i].zones = [this.zoneFor(record.id, record.frames)];
            kit.pads[i].bounce = null;
            kit.pads[i].name = record.name.slice(0, 80);
          });
        });
      }
      this.bank = 0;
      // The engine forgets the samples of the kit this one replaced; the
      // library keeps them.
      const part = this.part();
      const used = new Set(this.kit().pads.flatMap((p) => [...p.zones, ...(p.bounce?.zones || [])].map((z) => z.sample)));
      const unused = [...this.loaded].filter((id) => !used.has(id));
      if (unused.length && part !== null) {
        await this.hooks.request("/api/sampler/forget", { ids: unused });
        for (const id of unused) this.loaded.delete(id);
      }
      this.syncPool();
      this.sync();
    }

    // --- Sync and polling ----------------------------------------------------
    syncStatus() {
      const missing = this.missing.size ? ` · ${this.missing.size} samples missing from this browser` : "";
      this.statusText.textContent = (this.status || "") + missing;
    }
    sync() {
      const kit = this.kit();
      const part = this.part();
      this.root.hidden = !kit || part === null;
      if (this.root.hidden) return;
      if (part !== this.shownPart) {
        // Another kit: its pool is not this one's.
        this.shownPart = part;
        this.loaded.clear();
        this.missing.clear();
        this.history = [];
        this.slices = [];
        this.selection = null;
        this.bank = Math.floor(Math.max(0, kit.pads.findIndex((p) => p.id === kit.selected)) / BANK);
        // A new, empty kit starts with the first starter pack, as a Vocal
        // timbre starts with a voice.
        if (!kit.pads.some((p) => p.zones.length))
          this.bundled().then((packs) => {
            if (packs[0] && this.part() === part && !this.kit()?.pads.some((p) => p.zones.length))
              return this.loadBundled(packs[0]);
          });
      }
      const banks = Math.ceil(Math.max(kit.pads.length, 1) / BANK);
      for (const bank of this.banks.children) {
        const b = Number(bank.dataset.bank);
        bank.setAttribute("aria-pressed", String(b === this.bank));
        bank.classList.toggle("empty", b >= banks);
      }
      if (document.activeElement !== this.polyphony) this.polyphony.value = String(kit.polyphony);
      this.padButtons.forEach((element, slot) => this.syncPad(element, kit.pads[this.bank * BANK + slot], this.bank * BANK + slot));
      const pad = this.pad();
      const index = this.padIndex(pad);
      this.padTitle.textContent = pad ? `Pad ${label(index)} · ${noteName(pad.keys[0])}${pad.keys[1] !== pad.keys[0] ? `–${noteName(pad.keys[1])}` : ""}` : "No pad";
      if (document.activeElement !== this.padName) this.padName.value = pad?.name || "";
      this.muteButton.setAttribute("aria-pressed", String(!!pad?.muted));
      this.soloButton.setAttribute("aria-pressed", String(!!pad?.solo));
      const zones = pad?.zones || [];
      this.zoneSelect.hidden = zones.length < 2;
      if (this.zoneSelect.options.length !== zones.length || this.zoneSignature !== JSON.stringify(zones.map((z) => z.sample))) {
        this.zoneSignature = JSON.stringify(zones.map((z) => z.sample));
        this.zoneSelect.replaceChildren(...zones.map((z, i) => new Option(`Zone ${i + 1} · vel ${z.velocities[0]}–${z.velocities[1]} · ${noteName(z.keys[0])}–${noteName(z.keys[1])}`, i)));
      }
      this.zoneSelect.value = String(this.zone);
      if (pad) {
        const set = (field, value) => {
          if (document.activeElement !== field.input) field.input.value = String(value);
        };
        set(this.trigger, pad.trigger);
        set(this.keyLow, pad.keys[0]);
        set(this.keyHigh, pad.keys[1]);
        this.keytrack.input.checked = pad.keytrack;
        this.mono.input.checked = pad.mono;
        set(this.choke, pad.choke);
        set(this.layers, pad.layers);
        set(this.filterKind, pad.filter.kind);
        const options = this.bounceOptions(pad);
        if (this.bounceFor !== pad.id || !this.root.contains(document.activeElement)) {
          this.bounceFor = pad.id;
          set(this.bounceLow, options.low);
          set(this.bounceHigh, options.high);
          set(this.bounceStep, options.step);
          set(this.bounceVelocity, options.velocity);
          set(this.bounceHold, options.hold);
          set(this.bounceTail, options.tail);
        }
      }
      for (const update of this.knobs) update();
      this.syncZones(pad);
      this.syncFx(pad);
      this.syncModes();
      this.syncStatus();
      this.syncRecord();
      this.syncPool();
      this.showZone();
    }
    syncPad(element, pad, index) {
      const [labelNode, name, note, badges] = element.children;
      labelNode.textContent = label(index);
      element.classList.toggle("absent", !pad);
      if (!pad) {
        name.textContent = index < 128 ? "+ bank" : "";
        note.textContent = "";
        badges.textContent = "";
        element.setAttribute("aria-label", `Add bank ${BANKS[Math.floor(index / BANK)]}`);
        element.classList.remove("selected", "empty", "muted", "missing");
        return;
      }
      const empty = !pad.zones.length;
      name.textContent = pad.name || (empty ? "" : "Sample");
      note.textContent = noteName(pad.keys[0]);
      const marks = [];
      if (this.chainOf(pad)) marks.push("FX");
      if (pad.bounce) marks.push("B");
      if (pad.zones.some((z) => z.loop_mode !== "off")) marks.push("∞");
      if (pad.zones.length > 1) marks.push(`${pad.zones.length}L`);
      if (pad.choke) marks.push(`M${pad.choke}`);
      badges.textContent = marks.join(" ");
      element.classList.toggle("selected", pad.id === this.kit().selected);
      element.classList.toggle("empty", empty);
      element.classList.toggle("muted", pad.muted);
      element.classList.toggle("missing", pad.zones.some((z) => this.missing.has(z.sample)));
      element.setAttribute("aria-label", `Pad ${label(index)}${pad.name ? `, ${pad.name}` : empty ? ", empty" : ""}, ${noteName(pad.keys[0])}`);
      const hits = this.hits.get(pad.id) ?? 0;
      if (this.flash.get(pad.id) !== undefined && this.flash.get(pad.id) !== hits) {
        element.classList.remove("hit");
        void element.offsetWidth;
        element.classList.add("hit");
      }
      this.flash.set(pad.id, hits);
    }
    syncZones(pad) {
      const zones = pad?.zones || [];
      const signature = JSON.stringify([pad?.id, zones, this.zone]);
      if (signature === this.zoneTableSignature) return;
      this.zoneTableSignature = signature;
      const rows = zones.map((zone, i) => {
        const row = el("div", "sampler-zone");
        row.classList.toggle("selected", i === this.zone);
        const pick = button(`${i + 1}`, "Show this zone", () => {
          this.zone = i;
          this.view = [0, 1];
          this.sync();
        });
        const name = el("span", "sampler-sample-name", "…");
        this.library.get(zone.sample).then((r) => (name.textContent = r?.name || zone.sample));
        const number = (text, value, min, max, apply) => {
          const field = this.numberField(text, "", (v) => this.change(() => apply(Math.round(v * 100) / 100)), min, max);
          field.input.value = value;
          return field.box;
        };
        row.append(
          pick, name,
          number("Vel", zone.velocities[0], 1, 127, (v) => (zone.velocities = [v, Math.max(v, zone.velocities[1])])),
          number("to", zone.velocities[1], 1, 127, (v) => (zone.velocities = [Math.min(v, zone.velocities[0]), v])),
          number("Key", zone.keys[0], 0, 127, (v) => (zone.keys = [v, Math.max(v, zone.keys[1])])),
          number("to", zone.keys[1], 0, 127, (v) => (zone.keys = [Math.min(v, zone.keys[0]), v])),
          number("Tune", zone.tune, -48, 48, (v) => (zone.tune = v)),
          number("dB", zone.gain, -60, 12, (v) => (zone.gain = v)),
          number("Pan", zone.pan, -1, 1, (v) => (zone.pan = v)),
          button("×", "Remove this zone", () => {
            this.change(() => pad.zones.splice(i, 1));
            this.zone = 0;
          }),
        );
        return row;
      });
      this.zoneTable.replaceChildren(...rows);
      if (!rows.length) this.zoneTable.append(el("p", "muted", "No sample on this pad."));
    }
    syncFx(pad) {
      const chain = this.chainOf(pad);
      this.fxButton.textContent = chain ? `Edit ${chain.name}` : "Add FX chain";
      this.fxButton.title = chain
        ? "Open the pad's chain: Granular, Filters, Resynth and both effect racks"
        : "Give this pad its own chain: Granular, Filters, Resynth and effects, which Bounce bakes into samples";
      this.fxRemove.hidden = !chain;
      this.unbounceButton.hidden = !pad?.bounce;
      this.fxInfo.textContent = pad?.bounce
        ? `Bounced: plays ${pad.bounce.zones.length} rendered note${pad.bounce.zones.length === 1 ? "" : "s"}${chain ? `; ${chain.name} is bypassed until Unbounce` : ""}.`
        : chain
          ? `Plays through ${chain.name} live. Bounce renders it into samples and frees its CPU.`
          : "Plays straight into the timbre's chain. Add an FX chain for its own effects, or bounce to freeze the envelope and filter.";
    }
    loop() {
      const tick = async () => {
        if (!this.root.hidden && this.part() !== null && !document.hidden && !this.polling) {
          this.polling = true;
          try {
            const state = await this.hooks.request("/api/sampler/state");
            const before = this.loaded.size;
            this.loaded = new Set(state.view.loaded);
            for (const id of this.loaded) this.missing.delete(id);
            this.hits = new Map(state.view.hits);
            this.voices = state.view.voices;
            const chains = JSON.stringify(state.chains);
            if (chains !== this.chainSignature) {
              this.chainSignature = chains;
              this.chains = state.chains;
              this.sync();
            } else {
              const kit = this.kit();
              if (kit) this.padButtons.forEach((element, slot) => this.syncPad(element, kit.pads[this.bank * BANK + slot], this.bank * BANK + slot));
              this.drawOverlay();
            }
            if (this.loaded.size !== before) this.syncPool();
            if (this.recording) this.syncRecord(await this.hooks.request("/api/sampler/record"));
          } catch {
            /* The next tick tries again. */
          } finally {
            this.polling = false;
          }
        }
        setTimeout(tick, this.voices.length || this.recording ? 50 : 150);
      };
      tick();
    }
  }
  window.SadSampler = Sampler;
})();
