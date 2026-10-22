"use strict";
// The Vocal timbre's editor: a piano roll whose notes carry lyrics, the
// parameter lanes of VOCALOID5, a note inspector, styles, voicebanks and the
// song transport. Every edit posts the whole score; the host renders only the
// phrases that changed.
(() => {
  const TPB = 480;
  const KEYS = 44;
  const RULER = 22;
  const LANE = 96;
  const MIN_KEY = 24;
  const MAX_KEY = 108;
  const PARAMS = [
    ["dynamics", "DYN", "Dynamics", 0, 127, 64],
    ["breathiness", "BRE", "Breathiness", 0, 127, 0],
    ["brightness", "BRI", "Brightness", 0, 127, 64],
    ["clearness", "CLE", "Clearness", 0, 127, 0],
    ["character", "CHR", "Character", -64, 63, 0],
    ["exciter", "EXC", "Exciter", -64, 63, 0],
    ["growl", "GWL", "Growl", 0, 127, 0],
    ["air", "AIR", "Air", 0, 127, 0],
    ["mouth", "MTH", "Mouth", -127, 0, 0],
    ["portamento_timing", "POR", "Portamento timing", 0, 127, 64],
    ["pitch_bend", "PIT", "Pitch bend", -8192, 8191, 0],
    ["pitch_bend_sensitivity", "PBS", "Pitch bend sensitivity", 0, 24, 2],
  ].map(([key, short, name, min, max, value]) => ({ key, short, name, min, max, value }));
  const ATTACKS = ["none", "scoop", "bend", "soft", "accent", "breathy", "turn"];
  const RELEASES = ["none", "fall", "rise", "soft", "breathy", "swell"];
  const LANGUAGES = [["ja", "日本語"], ["en", "English"], ["ru", "Русский"]];
  const DEFAULT_LYRIC = { ja: "ら", en: "la", ru: "ла" };
  const NAMES = ["C", "C♯", "D", "D♯", "E", "F", "F♯", "G", "G♯", "A", "A♯", "B"];
  const STYLES = {
    Natural: {},
    Pop: { transition: 0.07, overshoot: 0.45, scoop: 60, vibrato_depth: 25, vibrato_rate: 6, vibrato_delay: 0.4, color: { brightness: 10, clearness: 15 } },
    Ballad: { transition: 0.13, overshoot: 0.2, preparation: 0.3, scoop: 70, fall: 60, vibrato_depth: 45, vibrato_rate: 5.2, vibrato_delay: 0.3, color: { breathiness: 25, brightness: -8 } },
    Rock: { transition: 0.06, overshoot: 0.6, scoop: 90, fall: 80, vibrato_depth: 30, vibrato_rate: 6.2, color: { growl: 35, exciter: 20, dynamics: 10 } },
    Whisper: { vibrato_depth: 10, color: { breathiness: 110, air: 60, dynamics: -10 } },
    Opera: { transition: 0.12, overshoot: 0.25, vibrato_depth: 70, vibrato_rate: 5.4, vibrato_delay: 0.15, vibrato_min_length: 0.3, color: { clearness: 40, character: -10 } },
    Robot: { transition: 0.02, overshoot: 0, preparation: 0, scoop: 0, fall: 0, vibrato_depth: 0, drift: 0, robot: true, auto_breath: false },
  };
  const NATURAL = {
    transition: 0.09, overshoot: 0.35, preparation: 0.2, scoop: 40, fall: 30,
    vibrato_depth: 35, vibrato_rate: 5.6, vibrato_delay: 0.35, vibrato_min_length: 0.45,
    drift: 8, robot: false, auto_breath: true, breath_level: 0.5, color: {},
  };
  const el = (tag, className, text) => {
    const node = document.createElement(tag);
    if (className) node.className = className;
    if (text !== undefined) node.textContent = text;
    return node;
  };
  const option = (value, label) => new Option(label, value);
  const clamp = (v, lo, hi) => Math.max(lo, Math.min(hi, v));
  const noteName = (key) => `${NAMES[key % 12]}${Math.floor(key / 12) - 1}`;
  const base64 = (buffer) => {
    const bytes = new Uint8Array(buffer);
    let text = "";
    for (let i = 0; i < bytes.length; i += 32768)
      text += String.fromCharCode(...bytes.subarray(i, i + 32768));
    return btoa(text);
  };
  /** Splits typed lyrics into one entry per note. */
  function splitLyrics(text, language) {
    const words = text.trim().split(/\s+/).filter(Boolean);
    if (language === "ja" && words.every((w) => /^[぀-ヿ]+$/.test(w))) {
      // One mora per note: small kana and ー join the previous character.
      const out = [];
      for (const char of words.join("")) {
        if (/[ゃゅょぁぃぅぇぉゎャュョァィゥェォヮ]/.test(char) && out.length) out[out.length - 1] += char;
        else out.push(char);
      }
      return out;
    }
    // "hel-lo world" → hel-, lo, world; a lone "-" holds the vowel.
    return words.flatMap((word) => {
      if (word === "-" || word === "+") return [word];
      const parts = word.split("-");
      return parts.map((part, i) => (i < parts.length - 1 ? `${part}-` : part)).filter((p) => p && p !== "-");
    });
  }

  class Vocal {
    constructor(root, hooks) {
      this.root = root;
      this.hooks = hooks;
      this.score = null;
      this.selected = new Set();
      this.view = { tick: 0, key: 79, pixels: 0.12, keyHeight: 14 };
      this.snap = TPB / 2;
      this.lane = "dynamics";
      this.undo = [];
      this.redo = [];
      this.phonemes = [];
      this.status = { playing: false, position: 0, progress: null };
      this.build();
    }

    // --- DOM -------------------------------------------------------------
    build() {
      const root = this.root;
      root.innerHTML = "";
      const heading = el("div", "section-heading vocal-heading");
      const title = el("h2", "", "Vocal ");
      title.append(el("span", "", "score · lyrics · voice"));
      this.statusText = el("span", "vocal-status", "");
      heading.append(title, this.statusText);
      const tools = el("div", "vocal-tools");
      // Voice
      this.voice = el("select");
      this.voice.setAttribute("aria-label", "Voicebank");
      this.voice.onchange = () => this.edit((s) => (s.voice = this.voice.value));
      const voiceMenu = el("details", "vocal-menu");
      const voiceSummary = el("summary", "", "Voices ▾");
      const menu = el("div", "vocal-menu-items");
      const item = (text, action) => {
        const b = el("button", "", text);
        b.onclick = async () => {
          voiceMenu.open = false;
          try {
            await action();
          } catch (error) {
            this.hooks.notify(error.message);
          }
        };
        menu.append(b);
        return b;
      };
      item("Import UTAU folder…", () => this.importVoice("folder"));
      item("Import .zip or .sadvoice…", () => this.importVoice("file"));
      item("Export voicebank…", () => this.exportVoice());
      item("Remove voicebank", () => this.removeVoice());
      voiceMenu.append(voiceSummary, menu);
      // Language, style, tempo, snap
      this.languageSelect = el("select");
      this.languageSelect.setAttribute("aria-label", "Lyrics language");
      this.languageSelect.append(...LANGUAGES.map(([v, l]) => option(v, l)));
      this.languageSelect.onchange = () => this.edit((s) => (s.language = this.languageSelect.value));
      this.style = el("select");
      this.style.setAttribute("aria-label", "Singing style");
      this.style.append(...Object.keys(STYLES).map((name) => option(name, name)));
      this.style.onchange = () =>
        this.edit((s) => (s.style = { ...NATURAL, ...STYLES[this.style.value], name: this.style.value }));
      const tempoLabel = el("label", "vocal-field", "BPM");
      this.tempo = el("input");
      Object.assign(this.tempo, { type: "number", min: 20, max: 300, step: 0.1 });
      this.tempo.setAttribute("aria-label", "Score tempo");
      this.tempo.onchange = () =>
        this.edit((s) => (s.tempo = [{ tick: 0, bpm: clamp(Number(this.tempo.value) || 120, 20, 300) }]));
      tempoLabel.append(this.tempo);
      const followLabel = el("label", "check vocal-follow");
      this.follow = el("input");
      this.follow.type = "checkbox";
      this.follow.onchange = () => this.edit((s) => (s.follow_host = this.follow.checked));
      followLabel.append(this.follow, "DAW tempo");
      this.followLabel = followLabel;
      const snapLabel = el("label", "vocal-field", "Grid");
      this.snapSelect = el("select");
      this.snapSelect.setAttribute("aria-label", "Grid");
      for (const [value, label] of [[TPB, "1/4"], [TPB / 2, "1/8"], [TPB / 4, "1/16"], [TPB / 8, "1/32"], [TPB / 3, "1/8T"], [1, "Off"]])
        this.snapSelect.append(option(value, label));
      this.snapSelect.value = String(this.snap);
      this.snapSelect.onchange = () => (this.snap = Number(this.snapSelect.value));
      snapLabel.append(this.snapSelect);
      // Actions
      this.lyricsButton = el("button", "", "Lyrics…");
      this.lyricsButton.title = "Type lyrics for the selected notes (or all)";
      this.lyricsButton.onclick = () => this.openLyrics();
      this.scoreButton = el("button", "", "Import score…");
      this.scoreButton.title = "MIDI, UST, MusicXML or VOCALOID5 .vpr";
      this.scoreButton.onclick = () => this.importScore();
      this.play = el("button", "vocal-play", "▶ Play");
      this.play.setAttribute("aria-pressed", "false");
      this.play.onclick = () => this.transport(!this.status.playing);
      tools.append(this.voice, voiceMenu, this.languageSelect, this.style, tempoLabel, followLabel, snapLabel, this.lyricsButton, this.scoreButton, this.play);
      // Canvas
      this.frame = el("div", "vocal-roll");
      this.canvas = el("canvas");
      this.canvas.tabIndex = 0;
      this.canvas.setAttribute("aria-label", "Piano roll: click and drag to add notes, double-click a note to type its lyric");
      this.frame.append(this.canvas);
      this.lyricInput = el("input", "vocal-lyric-input");
      this.lyricInput.hidden = true;
      this.lyricInput.setAttribute("aria-label", "Lyric");
      this.frame.append(this.lyricInput);
      // Lane chooser
      const lanes = el("div", "vocal-lanes");
      lanes.setAttribute("role", "tablist");
      lanes.setAttribute("aria-label", "Parameter lane");
      this.laneButtons = PARAMS.map((p) => {
        const b = el("button", "", p.short);
        b.title = `${p.name}: draw with the mouse, Alt-drag to erase`;
        b.setAttribute("role", "tab");
        b.onclick = () => {
          this.lane = p.key;
          this.draw();
          this.syncLanes();
        };
        lanes.append(b);
        return { button: b, key: p.key };
      });
      this.laneReset = el("button", "vocal-lane-reset", "Reset lane");
      this.laneReset.onclick = () => this.edit((s) => delete s.curves[this.lane]);
      lanes.append(this.laneReset);
      // Inspector
      this.inspector = el("div", "vocal-inspector");
      this.buildInspector();
      // Lyrics dialog
      this.lyricsDialog = el("div", "vocal-lyrics");
      this.lyricsDialog.hidden = true;
      this.lyricsText = el("textarea");
      this.lyricsText.setAttribute("aria-label", "Lyrics");
      this.lyricsText.placeholder = "さくら さくら · twin-kle twin-kle lit-tle star · в ле-су ро-ди-лась ё-лоч-ка";
      const apply = el("button", "", "Apply to notes");
      apply.onclick = () => this.applyLyrics();
      const cancel = el("button", "", "Cancel");
      cancel.onclick = () => (this.lyricsDialog.hidden = true);
      this.lyricsDialog.append(this.lyricsText, apply, cancel);
      // File inputs for the browser and plug-ins
      this.folderInput = el("input");
      Object.assign(this.folderInput, { type: "file", multiple: true, hidden: true });
      this.folderInput.webkitdirectory = true;
      this.fileInput = el("input");
      Object.assign(this.fileInput, { type: "file", hidden: true, accept: ".zip,.sadvoice" });
      this.scoreInput = el("input");
      Object.assign(this.scoreInput, { type: "file", hidden: true, accept: ".mid,.midi,.ust,.musicxml,.xml,.mxl,.vpr,.json" });
      const hint = el("p", "muted vocal-hint", "Click and drag to add notes · double-click to type a lyric (Tab moves on) · Delete removes · ↑↓ transposes · Space plays · ⌘/Ctrl+Z undoes");
      root.append(heading, tools, this.frame, lanes, this.inspector, this.lyricsDialog, hint, this.folderInput, this.fileInput, this.scoreInput);
      this.bindCanvas();
      new ResizeObserver(() => this.resize()).observe(this.frame);
    }

    buildInspector() {
      const ins = this.inspector;
      const field = (label, input) => {
        const l = el("label", "vocal-field", label);
        l.append(input);
        ins.append(l);
        return input;
      };
      const text = (label, apply, placeholder = "") => {
        const input = el("input");
        input.placeholder = placeholder;
        input.onchange = () => this.editNotes((n) => apply(n, input.value));
        return field(label, input);
      };
      const number = (label, min, max, step, apply) => {
        const input = el("input");
        Object.assign(input, { type: "number", min, max, step });
        input.placeholder = "style";
        input.onchange = () =>
          this.editNotes((n) => apply(n, input.value === "" ? null : clamp(Number(input.value), min, max)));
        return field(label, input);
      };
      const select = (label, values, apply) => {
        const s = el("select");
        s.append(...values.map((v) => option(v, v)));
        s.onchange = () => this.editNotes((n) => apply(n, s.value));
        return field(label, s);
      };
      this.noteLabel = el("strong", "vocal-note-label", "No note selected");
      ins.append(this.noteLabel);
      this.fields = {
        lyric: text("Lyric", (n, v) => (n.lyric = v)),
        phonemes: text("Phonemes", (n, v) => {
          if (v.trim()) n.phonemes = v.trim();
          else delete n.phonemes;
        }, "auto"),
        language: select("Language", ["auto", "ja", "en", "ru"], (n, v) => {
          if (v === "auto") delete n.language;
          else n.language = v;
        }),
        velocity: number("VEL", 0, 127, 1, (n, v) => (n.velocity = v ?? 64)),
        depth: number("Vib. depth ¢", 0, 200, 1, (n, v) => (n.vibrato = { ...n.vibrato, depth: v })),
        rate: number("Vib. rate Hz", 1, 12, 0.1, (n, v) => (n.vibrato = { ...n.vibrato, rate: v })),
        delay: number("Vib. delay s", 0, 4, 0.05, (n, v) => (n.vibrato = { ...n.vibrato, delay: v })),
        attack: select("Attack", ATTACKS, (n, v) => (n.attack = { kind: v, amount: n.attack?.amount ?? 0.5 })),
        attackAmount: number("Amount", 0, 1, 0.05, (n, v) => (n.attack = { kind: n.attack?.kind ?? "none", amount: v ?? 0.5 })),
        release: select("Release", RELEASES, (n, v) => (n.release = { kind: v, amount: n.release?.amount ?? 0.5 })),
        releaseAmount: number("Amount", 0, 1, 0.05, (n, v) => (n.release = { kind: n.release?.kind ?? "none", amount: v ?? 0.5 })),
      };
      this.fields.velocity.placeholder = "64";
      this.fields.attackAmount.placeholder = "0.5";
      this.fields.releaseAmount.placeholder = "0.5";
    }

    // --- Model -----------------------------------------------------------
    get language() {
      return this.score?.language || "ja";
    }
    defaultLyric() {
      return this.score?.default_lyric?.trim() || DEFAULT_LYRIC[this.language] || "la";
    }
    bpm() {
      return this.score?.tempo?.[0]?.bpm || 120;
    }
    secondsAt(tick) {
      // A constant tempo is what the editor offers; the host may follow its own.
      const tempo = this.status.hostBpm || this.bpm();
      return (tick / TPB) * (60 / tempo);
    }
    tickAt(seconds) {
      const tempo = this.status.hostBpm || this.bpm();
      return (seconds * tempo * TPB) / 60;
    }
    snapTick(tick) {
      return this.snap <= 1 ? Math.round(tick) : Math.round(tick / this.snap) * this.snap;
    }
    nextId() {
      return (this.score.notes.reduce((m, n) => Math.max(m, n.id || 0), 0) || 0) + 1;
    }
    sortNotes() {
      this.score.notes.sort((a, b) => a.start - b.start || a.key - b.key);
    }
    /** Records undo history, applies a change and posts the score. */
    edit(change) {
      if (!this.score) return;
      this.undo.push(JSON.stringify(this.score));
      if (this.undo.length > 100) this.undo.shift();
      this.redo = [];
      change(this.score);
      this.score.curves ||= {};
      this.sortNotes();
      this.commit();
    }
    editNotes(change) {
      const notes = this.score?.notes.filter((n) => this.selected.has(n.id)) ?? [];
      if (!notes.length) return;
      this.edit(() => notes.forEach(change));
    }
    history(from, to) {
      if (!from.length) return;
      to.push(JSON.stringify(this.score));
      this.score = JSON.parse(from.pop());
      this.commit();
    }
    commit() {
      this.syncControls();
      this.draw();
      clearTimeout(this.commitTimer);
      this.commitTimer = setTimeout(() => this.post(), 180);
    }
    async post() {
      const score = this.score;
      if (!score) return;
      try {
        const reply = await this.hooks.request("/api/vocal/score", { score });
        const preset = this.hooks.read();
        if (preset?.vocal) preset.vocal.score = reply.score;
        this.sent = JSON.stringify(reply.score);
        // The host numbers new notes; keep our selection on the same notes.
        if (this.score === score && reply.score.notes.length === score.notes.length)
          reply.score.notes.forEach((n, i) => (score.notes[i].id = n.id));
        this.refreshPhonemes();
      } catch (error) {
        this.hooks.notify(error.message);
      }
    }
    async refreshPhonemes() {
      const score = this.score;
      if (!score?.notes.length) {
        this.phonemes = [];
        return;
      }
      try {
        const lyrics = score.notes.map((n) => n.lyric || "");
        this.phonemes = await this.hooks.request("/api/vocal/phonemes", { language: score.language || "ja", lyrics });
        this.draw();
      } catch {
        this.phonemes = [];
      }
    }

    // --- Host sync ---------------------------------------------------------
    sync() {
      const preset = this.hooks.read();
      const vocal = preset?.vocal;
      this.root.hidden = !vocal;
      if (!vocal) {
        this.stopPolling();
        return;
      }
      const incoming = JSON.stringify(vocal.score);
      // Adopt the host's score when it changed elsewhere (presets, undo in
      // another window), never while our own edit is on its way.
      if (incoming !== this.sent && !this.commitTimer) {
        this.score = structuredClone(vocal.score);
        this.score.curves ||= {};
        this.sent = incoming;
        this.selected.clear();
        this.refreshPhonemes();
        if (!this.centered && this.score.notes.length) {
          const keys = this.score.notes.map((n) => n.key);
          this.view.key = Math.min(MAX_KEY, Math.round((Math.max(...keys) + Math.min(...keys)) / 2) + 10);
          this.centered = true;
        }
      }
      if (!this.voices) this.loadVoices();
      this.syncControls();
      this.resize();
      this.startPolling();
    }
    async loadVoices() {
      try {
        this.voices = await this.hooks.request("/api/vocal/voices");
      } catch {
        this.voices = [{ id: "", name: "SAD PET (built-in)", builtin: true }];
      }
      this.syncControls();
    }
    syncControls() {
      const s = this.score;
      if (!s) return;
      const voices = this.voices || [{ id: "", name: "SAD PET (built-in)" }];
      const signature = JSON.stringify(voices.map((v) => [v.id, v.name]));
      if (signature !== this.voiceSignature) {
        this.voice.replaceChildren(...voices.map((v) => option(v.id, v.language ? `${v.name} (${v.language})` : v.name)));
        this.voiceSignature = signature;
      }
      if (s.voice && !voices.some((v) => v.id === s.voice))
        this.voice.append(option(s.voice, `${s.voice} (missing)`));
      this.voice.value = s.voice || "";
      this.languageSelect.value = s.language || "ja";
      this.style.value = STYLES[s.style?.name] ? s.style.name : "Natural";
      if (document.activeElement !== this.tempo) this.tempo.value = this.bpm();
      this.follow.checked = s.follow_host !== false;
      const plugin = !!window.sadPetPlugin && !window.sadPetNative;
      this.followLabel.hidden = !plugin;
      this.tempo.disabled = plugin && s.follow_host !== false;
      this.syncLanes();
      this.syncInspector();
    }
    syncLanes() {
      for (const { button, key } of this.laneButtons) {
        button.setAttribute("aria-selected", String(key === this.lane));
        button.classList.toggle("vocal-lane-used", !!this.score?.curves?.[key]?.length);
      }
    }
    syncInspector() {
      const notes = this.score?.notes.filter((n) => this.selected.has(n.id)) ?? [];
      const disabled = !notes.length;
      for (const input of Object.values(this.fields)) input.disabled = disabled;
      if (disabled) {
        this.noteLabel.textContent = "No note selected";
        for (const input of Object.values(this.fields)) if (input.tagName === "INPUT") input.value = "";
        return;
      }
      const n = notes[0];
      this.noteLabel.textContent =
        notes.length > 1 ? `${notes.length} notes` : `${noteName(n.key)} · beat ${(n.start / TPB + 1).toFixed(2)}`;
      const set = (input, value) => {
        if (document.activeElement !== input) input.value = value ?? "";
      };
      set(this.fields.lyric, n.lyric);
      set(this.fields.phonemes, n.phonemes);
      this.fields.phonemes.placeholder = this.phonemes[this.score.notes.indexOf(n)] || "auto";
      set(this.fields.language, n.language || "auto");
      set(this.fields.velocity, n.velocity === 64 ? "" : n.velocity);
      set(this.fields.depth, n.vibrato?.depth);
      set(this.fields.rate, n.vibrato?.rate);
      set(this.fields.delay, n.vibrato?.delay);
      set(this.fields.attack, n.attack?.kind || "none");
      set(this.fields.attackAmount, n.attack?.kind && n.attack.kind !== "none" ? n.attack.amount : "");
      set(this.fields.release, n.release?.kind || "none");
      set(this.fields.releaseAmount, n.release?.kind && n.release.kind !== "none" ? n.release.amount : "");
    }
    startPolling() {
      if (this.poll) return;
      this.poll = setInterval(() => this.refreshStatus(), 120);
    }
    stopPolling() {
      clearInterval(this.poll);
      this.poll = null;
    }
    async refreshStatus() {
      if (this.root.hidden || this.polling) return;
      this.polling = true;
      try {
        const state = await this.hooks.request("/api/vocal/state");
        let progress = state.progress;
        if (!progress) {
          const all = await this.hooks.request("/api/vocal/progress");
          progress = all.progress?.[String(this.hooks.context()?.active)] ?? null;
          this.renderError = all.error;
        }
        const plugin = this.hooks.host?.();
        this.status = {
          playing: state.playing,
          position: state.position,
          progress,
          host: state.host,
          hostBpm: state.host && this.score?.follow_host !== false ? plugin?.host_bpm : null,
        };
        this.showStatus();
        this.draw();
      } catch {
        // The editor keeps working; the next poll tries again.
      } finally {
        this.polling = false;
      }
    }
    showStatus() {
      const p = this.status.progress;
      const voice = this.voices?.find((v) => v.id === (this.score?.voice || ""));
      let text = voice ? voice.name : "Built-in voice";
      if (p?.missing_voice) text = "Voicebank missing: the built-in voice sings";
      if (p && p.rendered < p.phrases) text += ` · rendering ${p.rendered}/${p.phrases}`;
      else if (p?.phrases) text += ` · ${p.phrases} phrase${p.phrases === 1 ? "" : "s"} ready`;
      if (this.renderError) text += ` · ${this.renderError}`;
      this.statusText.textContent = text;
      this.play.textContent = this.status.playing ? "■ Stop" : "▶ Play";
      this.play.setAttribute("aria-pressed", String(this.status.playing));
      this.play.disabled = !!this.status.host;
      this.play.title = this.status.host ? "The DAW's transport plays the score" : "Play from the cursor (Space)";
    }
    async transport(playing) {
      try {
        const start = this.secondsAt(this.cursor ?? 0);
        await this.hooks.request("/api/vocal/transport", { playing, start });
        this.status.playing = playing;
        this.showStatus();
      } catch (error) {
        this.hooks.notify(error.message);
      }
    }

    // --- Voicebanks and scores ----------------------------------------------
    pick(input) {
      return new Promise((resolve) => {
        input.value = "";
        input.onchange = () => resolve([...input.files]);
        input.click();
      });
    }
    async importVoice(kind) {
      const language = this.languageSelect.value;
      if (window.sadPetNative) {
        const result = await window.sadPetNative.dialog("/api/vocal/import", { language }, {
          title: kind === "folder" ? "Choose a UTAU voicebank folder" : "Choose a voicebank",
          patterns: kind === "folder" ? "" : "*.zip;*.sadvoice",
          folders: kind === "folder",
        });
        if (result?.cancelled) return;
        await this.voiceImported(result);
        return;
      }
      const files = await this.pick(kind === "folder" ? this.folderInput : this.fileInput);
      if (!files.length) return;
      this.statusText.textContent = "Analyzing the voicebank…";
      let body;
      if (kind === "folder") {
        const list = files.filter((f) => /\.(wav|ini|txt|map|yaml)$/i.test(f.name));
        const entries = await Promise.all(list.map(async (f) => ({ path: f.webkitRelativePath || f.name, data: await f.arrayBuffer() })));
        body = { files: entries, language };
      } else {
        const data = await files[0].arrayBuffer();
        body = files[0].name.toLowerCase().endsWith(".sadvoice") ? { voice: data } : { zip: data, language };
      }
      let info;
      if (window.sadPetPlugin) {
        const json = { language };
        if (body.files) json.files = body.files.map((f) => ({ path: f.path, base64: base64(f.data) }));
        if (body.voice) json.voice = base64(body.voice);
        if (body.zip) json.zip = base64(body.zip);
        info = await this.hooks.request("/api/vocal/import", json);
      } else {
        info = await (await this.hooks.api("/api/vocal/import", body, { "X-Vocal-Import": "1" })).json();
      }
      await this.voiceImported(info);
    }
    async voiceImported(info) {
      this.voices = null;
      await this.loadVoices();
      this.hooks.notify(`Imported ${info.name}`);
      this.edit((s) => (s.voice = info.id));
    }
    async exportVoice() {
      const id = this.score?.voice;
      if (!id) throw new Error("The built-in voice has no file to export");
      if (window.sadPetNative) {
        await window.sadPetNative.dialog("/api/vocal/export", { id }, { title: "Export voicebank", mode: "save", filename: `${id}.sadvoice`, patterns: "*.sadvoice" });
        return;
      }
      const response = await this.hooks.api("/api/vocal/export", { id });
      this.hooks.download(await response.blob(), `${id}.sadvoice`);
    }
    async removeVoice() {
      const id = this.score?.voice;
      if (!id) throw new Error("The built-in voice cannot be removed");
      if (!confirm(`Remove the voicebank “${this.voice.selectedOptions[0]?.textContent}”?`)) return;
      await this.hooks.request("/api/vocal/voice/remove", { id });
      this.voices = null;
      await this.loadVoices();
      this.edit((s) => (s.voice = ""));
    }
    async importScore() {
      let score;
      if (window.sadPetNative) {
        const result = await window.sadPetNative.dialog("/api/vocal/import-score", { language: this.languageSelect.value }, {
          title: "Import a score", patterns: "*.mid;*.midi;*.ust;*.musicxml;*.xml;*.mxl;*.vpr",
        });
        if (result?.cancelled) return;
        score = result.score;
      } else {
        const [file] = await this.pick(this.scoreInput);
        if (!file) return;
        const data = await file.arrayBuffer();
        score = (await this.hooks.request("/api/vocal/import-score", { name: file.name, base64: base64(data), language: this.languageSelect.value })).score;
      }
      const replace = !this.score.notes.length || confirm("Replace the current notes with the imported score? Cancel adds it after them.");
      this.edit((s) => {
        const offset = replace ? 0 : Math.ceil(Math.max(0, ...s.notes.map((n) => n.start + n.length)) / (TPB * 4)) * TPB * 4;
        let id = replace ? 1 : this.nextId();
        const notes = score.notes.map((n) => ({ ...n, id: id++, start: n.start + offset }));
        s.notes = replace ? notes : [...s.notes, ...notes];
        if (replace) {
          if (score.tempo?.length) s.tempo = score.tempo;
          if (score.language) s.language = score.language;
          if (score.curves) s.curves = score.curves;
        }
      });
      this.hooks.notify(`Imported ${score.notes.length} notes`);
    }

    // --- Lyrics -----------------------------------------------------------
    openLyrics() {
      const notes = this.targetNotes();
      this.lyricsText.value = notes.map((n) => n.lyric || this.defaultLyric()).join(" ");
      this.lyricsDialog.hidden = false;
      this.lyricsText.focus();
    }
    targetNotes() {
      const selected = this.score?.notes.filter((n) => this.selected.has(n.id)) ?? [];
      return selected.length ? selected : this.score?.notes ?? [];
    }
    applyLyrics() {
      const notes = this.targetNotes();
      const lyrics = splitLyrics(this.lyricsText.value, this.language);
      this.edit(() => notes.forEach((n, i) => (n.lyric = lyrics[i] ?? n.lyric)));
      this.lyricsDialog.hidden = true;
    }
    editLyric(note) {
      const rect = this.noteRect(note);
      if (!rect) return;
      const input = this.lyricInput;
      input.hidden = false;
      Object.assign(input.style, {
        left: `${rect.x}px`,
        top: `${rect.y - 2}px`,
        width: `${Math.max(56, rect.w)}px`,
      });
      input.value = note.lyric || "";
      input.focus();
      input.select();
      const finish = (move) => {
        input.onkeydown = input.onblur = null;
        input.hidden = true;
        const value = input.value.trim();
        if (value !== (note.lyric || "")) this.edit(() => (note.lyric = value));
        if (move) {
          const index = this.score.notes.indexOf(note) + move;
          const next = this.score.notes[index];
          if (next) {
            this.selected = new Set([next.id]);
            this.ensureVisible(next);
            this.draw();
            this.editLyric(next);
            return;
          }
        }
        this.canvas.focus();
      };
      input.onkeydown = (e) => {
        if (e.key === "Enter") finish(0);
        else if (e.key === "Tab") {
          e.preventDefault();
          finish(e.shiftKey ? -1 : 1);
        } else if (e.key === "Escape") {
          input.value = note.lyric || "";
          finish(0);
        }
        e.stopPropagation();
      };
      input.onblur = () => finish(0);
    }

    // --- Geometry ---------------------------------------------------------
    resize() {
      const width = Math.max(320, this.frame.clientWidth);
      const height = 300 + LANE;
      const ratio = window.devicePixelRatio || 1;
      if (this.width === width && this.height === height && this.ratio === ratio) return this.draw();
      Object.assign(this, { width, height, ratio });
      this.canvas.width = Math.round(width * ratio);
      this.canvas.height = Math.round(height * ratio);
      this.canvas.style.width = `${width}px`;
      this.canvas.style.height = `${height}px`;
      this.draw();
    }
    get rollBottom() {
      return this.height - LANE;
    }
    x(tick) {
      return KEYS + (tick - this.view.tick) * this.view.pixels;
    }
    tickX(x) {
      return this.view.tick + (x - KEYS) / this.view.pixels;
    }
    y(key) {
      return RULER + (this.view.key - key) * this.view.keyHeight;
    }
    keyY(y) {
      return Math.floor(this.view.key - (y - RULER) / this.view.keyHeight) + 1;
    }
    noteRect(n) {
      const x = this.x(n.start);
      const w = Math.max(3, n.length * this.view.pixels);
      const y = this.y(n.key);
      if (y + this.view.keyHeight < RULER || y > this.rollBottom) return null;
      return { x, y, w, h: this.view.keyHeight };
    }
    noteAt(px, py) {
      for (let i = this.score.notes.length - 1; i >= 0; i--) {
        const n = this.score.notes[i];
        const r = this.noteRect(n);
        if (r && px >= r.x && px <= r.x + r.w && py >= r.y && py <= r.y + r.h) return n;
      }
      return null;
    }
    ensureVisible(n) {
      const visible = (this.width - KEYS) / this.view.pixels;
      if (n.start < this.view.tick || n.start + n.length > this.view.tick + visible)
        this.view.tick = Math.max(0, n.start - visible * 0.2);
      const rows = (this.rollBottom - RULER) / this.view.keyHeight;
      if (n.key > this.view.key || n.key < this.view.key - rows + 1)
        this.view.key = clamp(n.key + Math.floor(rows / 2), MIN_KEY + rows, MAX_KEY);
    }

    // --- Drawing ----------------------------------------------------------
    draw() {
      if (!this.score || !this.width || this.root.hidden) return;
      cancelAnimationFrame(this.frameRequest);
      this.frameRequest = requestAnimationFrame(() => this.paint());
    }
    paint() {
      const c = this.canvas.getContext("2d");
      const { width, height, ratio, view } = this;
      const css = getComputedStyle(document.documentElement);
      const color = (name, fallback) => css.getPropertyValue(name).trim() || fallback;
      const bg = color("--panel", "#1c201d"), line = color("--line", "#303831"),
        text = color("--text", "#e6ebe5"), muted = color("--muted", "#8c9a8c"),
        accent = color("--accent", "#c4f486"), blue = color("--blue", "#b6d5c8");
      c.setTransform(ratio, 0, 0, ratio, 0, 0);
      c.fillStyle = bg;
      c.fillRect(0, 0, width, height);
      const bottom = this.rollBottom;
      // Key rows
      c.save();
      c.beginPath();
      c.rect(0, RULER, width, bottom - RULER);
      c.clip();
      for (let key = MAX_KEY; key >= MIN_KEY; key--) {
        const y = this.y(key);
        if (y > bottom || y + view.keyHeight < RULER) continue;
        const black = [1, 3, 6, 8, 10].includes(key % 12);
        c.fillStyle = black ? "rgba(0,0,0,0.22)" : "rgba(255,255,255,0.015)";
        c.fillRect(KEYS, y, width - KEYS, view.keyHeight);
        if (key % 12 === 0) {
          c.fillStyle = line;
          c.fillRect(KEYS, y + view.keyHeight - 1, width - KEYS, 1);
        }
        c.fillStyle = black ? "#2a302a" : "#d9ded8";
        c.fillRect(0, y, KEYS - 8, view.keyHeight - 1);
        if (key % 12 === 0) {
          c.fillStyle = "#1c201d";
          c.font = "9px Inter, sans-serif";
          c.fillText(noteName(key), 3, y + view.keyHeight - 3);
        }
      }
      // Beats and bars
      const visible = (width - KEYS) / view.pixels;
      const step = view.pixels * TPB < 14 ? TPB * 4 : TPB;
      for (let t = Math.floor(view.tick / step) * step; t < view.tick + visible; t += step) {
        const x = this.x(t);
        if (x < KEYS) continue;
        c.fillStyle = t % (TPB * 4) === 0 ? "rgba(255,255,255,0.13)" : "rgba(255,255,255,0.05)";
        c.fillRect(Math.round(x), RULER, 1, bottom - RULER);
      }
      // Notes
      c.font = "11px Inter, sans-serif";
      this.score.notes.forEach((n, i) => {
        const r = this.noteRect(n);
        if (!r || r.x > width || r.x + r.w < KEYS) return;
        const selected = this.selected.has(n.id);
        c.fillStyle = selected ? accent : "#7f9d64";
        c.globalAlpha = selected ? 1 : 0.85;
        c.fillRect(r.x, r.y + 1, r.w, r.h - 2);
        c.globalAlpha = 1;
        c.fillStyle = "#11140f";
        const lyric = n.lyric || "·";
        c.save();
        c.beginPath();
        c.rect(r.x, r.y, r.w, r.h);
        c.clip();
        c.fillText(lyric, r.x + 3, r.y + r.h - 3);
        c.restore();
        const phonemes = this.phonemes[i];
        if (phonemes && view.keyHeight >= 12) {
          c.fillStyle = muted;
          c.font = "9px Inter, sans-serif";
          c.fillText(n.phonemes ? `[${n.phonemes}]` : phonemes, r.x + 1, r.y + r.h + 10);
          c.font = "11px Inter, sans-serif";
        }
      });
      // Selection rectangle
      if (this.marquee) {
        const m = this.marquee;
        c.strokeStyle = accent;
        c.setLineDash([4, 3]);
        c.strokeRect(Math.min(m.x0, m.x1), Math.min(m.y0, m.y1), Math.abs(m.x1 - m.x0), Math.abs(m.y1 - m.y0));
        c.setLineDash([]);
      }
      c.restore();
      // Ruler
      c.fillStyle = "#151915";
      c.fillRect(0, 0, width, RULER);
      c.fillStyle = muted;
      c.font = "10px Inter, sans-serif";
      for (let bar = Math.floor(view.tick / (TPB * 4)); bar * TPB * 4 < view.tick + visible; bar++) {
        const x = this.x(bar * TPB * 4);
        if (x >= KEYS) c.fillText(String(bar + 1), x + 3, 14);
      }
      // Cursor (play start) and playhead
      const cursorX = this.x(this.cursor ?? 0);
      if (cursorX >= KEYS) {
        c.fillStyle = blue;
        c.beginPath();
        c.moveTo(cursorX - 5, 0);
        c.lineTo(cursorX + 5, 0);
        c.lineTo(cursorX, 8);
        c.fill();
      }
      if (this.status.playing) {
        const px = this.x(this.tickAt(this.status.position));
        if (px >= KEYS && px <= width) {
          c.fillStyle = accent;
          c.fillRect(Math.round(px), 0, 1.5, bottom);
        }
      }
      // Parameter lane
      this.paintLane(c, bottom, { line, muted, accent });
    }
    paintLane(c, top, colors) {
      const { width } = this;
      const param = PARAMS.find((p) => p.key === this.lane);
      c.fillStyle = "#171b17";
      c.fillRect(0, top, width, LANE);
      c.fillStyle = colors.line;
      c.fillRect(0, top, width, 1);
      c.fillStyle = colors.muted;
      c.font = "10px Inter, sans-serif";
      c.fillText(param.short, 6, top + 14);
      c.fillText(String(param.max), 6, top + 26);
      c.fillText(String(param.min), 6, top + LANE - 4);
      const valueY = (v) => top + LANE - 4 - ((v - param.min) / (param.max - param.min)) * (LANE - 10);
      // Default line
      c.fillStyle = "rgba(255,255,255,0.08)";
      c.fillRect(KEYS, Math.round(valueY(param.value)), width - KEYS, 1);
      // Note shadows show where singing happens.
      c.fillStyle = "rgba(196,244,134,0.06)";
      for (const n of this.score.notes) {
        const x = this.x(n.start);
        c.fillRect(x, top + 1, n.length * this.view.pixels, LANE - 1);
      }
      const curve = this.score.curves?.[this.lane] ?? [];
      c.strokeStyle = colors.accent;
      c.lineWidth = 1.5;
      c.beginPath();
      if (!curve.length) {
        c.moveTo(KEYS, valueY(param.value));
        c.lineTo(width, valueY(param.value));
        c.strokeStyle = "rgba(196,244,134,0.35)";
      } else {
        c.moveTo(KEYS, valueY(curve[0].value));
        for (const p of curve) c.lineTo(clamp(this.x(p.tick), KEYS, width), valueY(p.value));
        c.lineTo(width, valueY(curve[curve.length - 1].value));
      }
      c.stroke();
      c.lineWidth = 1;
    }

    // --- Input ------------------------------------------------------------
    bindCanvas() {
      const canvas = this.canvas;
      const local = (e) => {
        const r = canvas.getBoundingClientRect();
        return { x: e.clientX - r.left, y: e.clientY - r.top };
      };
      canvas.addEventListener("pointerdown", (e) => {
        if (!this.score) return;
        canvas.focus();
        const p = local(e);
        canvas.setPointerCapture(e.pointerId);
        if (p.y < RULER) {
          this.cursor = Math.max(0, this.snapTick(this.tickX(Math.max(KEYS, p.x))));
          this.hooks.request("/api/vocal/transport", { start: this.secondsAt(this.cursor) }).catch(() => {});
          this.draw();
          return;
        }
        if (p.y >= this.rollBottom) {
          this.drag = { kind: "lane", erase: e.altKey || e.button === 2, last: null, before: JSON.stringify(this.score) };
          this.paintCurve(p);
          return;
        }
        if (p.x < KEYS) return;
        const note = this.noteAt(p.x, p.y);
        if (note) {
          if (e.shiftKey || e.metaKey || e.ctrlKey) {
            if (this.selected.has(note.id)) this.selected.delete(note.id);
            else this.selected.add(note.id);
          } else if (!this.selected.has(note.id)) this.selected = new Set([note.id]);
          const r = this.noteRect(note);
          const resize = p.x > r.x + r.w - 6 && r.w > 10;
          this.drag = {
            kind: resize ? "resize" : "move",
            start: p,
            before: JSON.stringify(this.score),
            notes: this.score.notes.filter((n) => this.selected.has(n.id)).map((n) => ({ n, start: n.start, key: n.key, length: n.length })),
            moved: false,
          };
        } else if (e.shiftKey) {
          this.drag = { kind: "marquee" };
          this.marquee = { x0: p.x, y0: p.y, x1: p.x, y1: p.y };
        } else {
          // Draw a new note.
          const start = Math.max(0, Math.floor(this.tickX(p.x) / Math.max(1, this.snap)) * Math.max(1, this.snap));
          const note = {
            id: this.nextId(), start, length: Math.max(this.snap, 30), key: clamp(this.keyY(p.y), MIN_KEY, MAX_KEY),
            lyric: this.defaultLyric(), velocity: 64,
          };
          const before = JSON.stringify(this.score);
          this.score.notes.push(note);
          this.sortNotes();
          this.selected = new Set([note.id]);
          this.drag = { kind: "resize", start: p, before, notes: [{ n: note, start: note.start, key: note.key, length: note.length }], moved: true, created: true };
        }
        this.syncInspector();
        this.draw();
      });
      canvas.addEventListener("pointermove", (e) => {
        const d = this.drag;
        if (!d) {
          const p = local(e);
          const note = p.y > RULER && p.y < this.rollBottom ? this.noteAt(p.x, p.y) : null;
          const r = note && this.noteRect(note);
          canvas.style.cursor = note ? (p.x > r.x + r.w - 6 ? "ew-resize" : "grab") : p.y < RULER ? "pointer" : "crosshair";
          return;
        }
        const p = local(e);
        if (d.kind === "lane") return this.paintCurve(p);
        if (d.kind === "marquee") {
          Object.assign(this.marquee, { x1: p.x, y1: p.y });
          const [x0, x1] = [Math.min(this.marquee.x0, p.x), Math.max(this.marquee.x0, p.x)];
          const [y0, y1] = [Math.min(this.marquee.y0, p.y), Math.max(this.marquee.y0, p.y)];
          this.selected = new Set(this.score.notes.filter((n) => {
            const r = this.noteRect(n);
            return r && r.x < x1 && r.x + r.w > x0 && r.y < y1 && r.y + r.h > y0;
          }).map((n) => n.id));
          this.draw();
          return;
        }
        const dt = (p.x - d.start.x) / this.view.pixels;
        const dk = Math.round((d.start.y - p.y) / this.view.keyHeight);
        if (Math.abs(p.x - d.start.x) + Math.abs(p.y - d.start.y) > 3) d.moved = true;
        for (const item of d.notes) {
          if (d.kind === "move") {
            item.n.start = Math.max(0, this.snapTick(item.start + dt));
            item.n.key = clamp(item.key + dk, MIN_KEY, MAX_KEY);
          } else {
            const end = this.snap <= 1 ? item.start + item.length + dt : this.snapTick(item.start + item.length + dt);
            item.n.length = Math.max(Math.max(15, Math.min(this.snap, TPB / 8)), Math.round(end - item.start));
          }
        }
        this.sortNotes();
        this.syncInspector();
        this.draw();
      });
      const end = () => {
        const d = this.drag;
        this.drag = null;
        if (this.marquee) {
          this.marquee = null;
          this.syncInspector();
          this.draw();
        }
        if (!d) return;
        if ((d.kind === "move" || d.kind === "resize") && d.moved) {
          this.undo.push(d.before);
          this.redo = [];
          this.commit();
        } else if (d.kind === "lane" && d.changed) {
          this.undo.push(d.before);
          this.redo = [];
          this.commit();
        }
      };
      canvas.addEventListener("pointerup", end);
      canvas.addEventListener("pointercancel", end);
      canvas.addEventListener("contextmenu", (e) => e.preventDefault());
      canvas.addEventListener("dblclick", (e) => {
        const p = local(e);
        const note = this.noteAt(p.x, p.y);
        if (note) {
          this.selected = new Set([note.id]);
          this.editLyric(note);
        }
      });
      canvas.addEventListener("wheel", (e) => {
        e.preventDefault();
        if (e.ctrlKey || e.metaKey) {
          const p = local(e);
          const at = this.tickX(p.x);
          this.view.pixels = clamp(this.view.pixels * (e.deltaY < 0 ? 1.15 : 1 / 1.15), 0.01, 1.2);
          this.view.tick = Math.max(0, at - (p.x - KEYS) / this.view.pixels);
        } else if (e.shiftKey || Math.abs(e.deltaX) > Math.abs(e.deltaY)) {
          this.view.tick = Math.max(0, this.view.tick + (e.deltaX || e.deltaY) / this.view.pixels);
        } else {
          this.view.key = clamp(this.view.key - Math.sign(e.deltaY) * 2, MIN_KEY + 12, MAX_KEY);
        }
        this.draw();
      }, { passive: false });
      canvas.addEventListener("keydown", (e) => this.key(e));
    }
    paintCurve(p) {
      const d = this.drag;
      const param = PARAMS.find((q) => q.key === this.lane);
      const top = this.rollBottom;
      const tick = Math.max(0, Math.round(this.tickX(Math.max(KEYS, p.x)) / 15) * 15);
      const value = clamp(
        param.min + ((top + LANE - 4 - p.y) / (LANE - 10)) * (param.max - param.min),
        param.min, param.max,
      );
      const rounded = Math.abs(param.max - param.min) > 50 ? Math.round(value) : Math.round(value * 10) / 10;
      const curve = (this.score.curves[this.lane] ||= []);
      const from = d.last ?? tick;
      const [lo, hi] = [Math.min(from, tick), Math.max(from, tick)];
      // Replace the dragged span; erasing leaves it at the default.
      const kept = curve.filter((q) => q.tick < lo || q.tick > hi);
      if (!d.erase) {
        const lastValue = d.lastValue ?? rounded;
        for (let t = lo; t <= hi; t += 15) {
          const k = hi === lo ? 1 : (t - from) / (tick - from || 1);
          kept.push({ tick: t, value: Math.round((lastValue + (rounded - lastValue) * clamp(k, 0, 1)) * 10) / 10 });
        }
      }
      kept.sort((a, b) => a.tick - b.tick);
      if (kept.length) this.score.curves[this.lane] = kept;
      else delete this.score.curves[this.lane];
      d.last = tick;
      d.lastValue = rounded;
      d.changed = true;
      this.syncLanes();
      this.draw();
    }
    key(e) {
      const mod = e.metaKey || e.ctrlKey;
      const selected = this.score?.notes.filter((n) => this.selected.has(n.id)) ?? [];
      if (mod && e.key.toLowerCase() === "z") {
        e.preventDefault();
        if (e.shiftKey) this.history(this.redo, this.undo);
        else this.history(this.undo, this.redo);
      } else if (mod && e.key.toLowerCase() === "y") {
        e.preventDefault();
        this.history(this.redo, this.undo);
      } else if (mod && e.key.toLowerCase() === "a") {
        e.preventDefault();
        this.selected = new Set(this.score.notes.map((n) => n.id));
        this.syncInspector();
        this.draw();
      } else if (mod && (e.key.toLowerCase() === "c" || e.key.toLowerCase() === "x") && selected.length) {
        e.preventDefault();
        const first = Math.min(...selected.map((n) => n.start));
        this.clipboard = selected.map((n) => ({ ...structuredClone(n), start: n.start - first }));
        if (e.key.toLowerCase() === "x") this.edit((s) => (s.notes = s.notes.filter((n) => !this.selected.has(n.id))));
      } else if (mod && e.key.toLowerCase() === "v" && this.clipboard?.length) {
        e.preventDefault();
        const at = this.cursor ?? 0;
        let id = this.nextId();
        const pasted = this.clipboard.map((n) => ({ ...structuredClone(n), id: id++, start: n.start + at }));
        this.edit((s) => s.notes.push(...pasted));
        this.selected = new Set(pasted.map((n) => n.id));
        this.syncInspector();
      } else if ((e.key === "Delete" || e.key === "Backspace") && selected.length) {
        e.preventDefault();
        this.edit((s) => (s.notes = s.notes.filter((n) => !this.selected.has(n.id))));
        this.selected.clear();
        this.syncInspector();
      } else if ((e.key === "ArrowUp" || e.key === "ArrowDown") && selected.length) {
        e.preventDefault();
        const by = (e.key === "ArrowUp" ? 1 : -1) * (e.shiftKey ? 12 : 1);
        this.editNotes((n) => (n.key = clamp(n.key + by, MIN_KEY, MAX_KEY)));
      } else if ((e.key === "ArrowLeft" || e.key === "ArrowRight") && selected.length) {
        e.preventDefault();
        const by = (e.key === "ArrowRight" ? 1 : -1) * Math.max(this.snap, 15);
        this.editNotes((n) => (n.start = Math.max(0, n.start + by)));
      } else if (e.key === "Enter" && selected.length) {
        e.preventDefault();
        this.editLyric(selected[0]);
      } else if (e.key === " ") {
        e.preventDefault();
        if (!this.play.disabled) this.transport(!this.status.playing);
      } else if (e.key === "Escape") {
        this.selected.clear();
        this.syncInspector();
        this.draw();
      }
    }
  }
  window.SadVocal = Vocal;
  window.SadVocal.splitLyrics = splitLyrics;
})();
