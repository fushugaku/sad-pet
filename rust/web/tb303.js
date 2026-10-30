"use strict";
(() => {
  const defaults = { tuning: 0, cutoff: .45, resonance: .72, env_mod: .55, decay: .35, accent: .65, waveform: 0, level: 1, transpose: 0 };
  const notes = ["C", "C♯", "D", "D♯", "E", "F", "F♯", "G", "G♯", "A", "A♯", "B"];
  const noteName = (n) => `${notes[n % 12]}${Math.floor(n / 12) - 1}`;
  const percent = (v) => `${Math.round(v * 100)}%`;
  class Bass303 {
    constructor(root, hooks) {
      this.root = root;
      this.hooks = hooks;
      root.innerHTML = `<div class="section-heading"><h2>303 Bass <span>Mono</span></h2><div class="tb303-waveforms" role="group" aria-label="303 waveform"><button data-wave="0" aria-pressed="true">Saw</button><button data-wave="1" aria-pressed="false">Square</button></div></div><div class="knob-grid tb303-knobs"></div><div class="tb303-pattern-heading"><button class="tb303-pattern-toggle" aria-pressed="true">Pattern</button><label>Length <select aria-label="303 pattern length">${Array.from({ length: 16 }, (_, i) => `<option value="${i + 1}">${i + 1}</option>`).join("")}</select></label></div><div class="tb303-steps" aria-label="303 pattern"></div>`;
      const radias = document.createElement("div");
      radias.className = "tb303-radias";
      radias.innerHTML = `<div class="section-heading"><h3>RADIAS</h3><button id="tb303-add">+ Oscillator</button></div><div id="tb303-oscillators" class="resynth-oscillators"></div>`;
      root.append(radias);
      radias.querySelector("button").onclick = () => hooks.addOscillator();
      for (const [key, label, format, min, max] of [
        ["tuning", "TUNING", (v) => `${v > 0 ? "+" : ""}${v.toFixed(2)} st`, -7, 7],
        ["cutoff", "CUTOFF", percent, 0, 1],
        ["resonance", "RESONANCE", percent, 0, 1],
        ["env_mod", "ENV MOD", percent, 0, 1],
        ["decay", "DECAY", (v) => `${Math.round(200 * 12.5 ** v)} ms`, 0, 1],
        ["accent", "ACCENT", percent, 0, 1],
        ["transpose", "OSC PITCH", (v) => `${v >= 0 ? "+" : ""}${v.toFixed(1)} st`, -48, 48],
        ["level", "OSC LEVEL", percent, 0, 1],
      ]) hooks.knob([`tb303_${key}`, label, format, min, max, `303 ${label.toLowerCase()}`], root.querySelector(".tb303-knobs"), {
        get: () => this.settings()?.[key] ?? defaults[key],
        set: (value) => this.edit(key, value), reset: () => defaults[key],
      });
      this.waves = [...root.querySelectorAll("[data-wave]")];
      for (const button of this.waves) button.onclick = () => this.edit("waveform", Number(button.dataset.wave));
      this.pattern = root.querySelector(".tb303-pattern-toggle");
      this.pattern.onclick = () => this.edit("pattern", !this.settings()?.pattern);
      this.length = root.querySelector('[aria-label="303 pattern length"]');
      this.length.onchange = () => this.edit("length", Number(this.length.value));
      this.steps = Array.from({ length: 16 }, (_, i) => {
        const step = document.createElement("div");
        step.className = "tb303-step";
        step.innerHTML = `<button class="tb303-gate" aria-label="303 step ${i + 1} note on" title="Note / rest" aria-pressed="true">${String(i + 1).padStart(2, "0")}</button><select class="tb303-note" aria-label="303 step ${i + 1} pitch">${Array.from({ length: 128 }, (_, n) => `<option value="${n}">${noteName(n)}</option>`).join("")}</select><button class="tb303-accent" aria-label="303 step ${i + 1} accent" aria-pressed="false">Accent</button><button class="tb303-slide" aria-label="303 step ${i + 1} slide" aria-pressed="false">Slide</button>`;
        for (const field of ["gate", "accent", "slide"]) step.querySelector(`.tb303-${field}`).onclick = () => this.editStep(i, field, !this.settings().steps[i][field]);
        const pitch = step.querySelector("select");
        pitch.onchange = () => this.editStep(i, "note", Number(pitch.value));
        root.querySelector(".tb303-steps").append(step);
        return step;
      });
    }
    settings() { return this.hooks.read()?.tb303; }
    edit(key, value) {
      const settings = this.settings();
      if (!settings) return;
      settings[key] = value;
      this.hooks.parameter(`tb303_${key}`, Number(value));
      this.sync();
    }
    editStep(i, key, value) {
      const settings = this.settings();
      if (!settings) return;
      settings.steps[i][key] = value;
      this.hooks.parameter(`tb303_step${i + 1}_${key}`, Number(value));
      this.sync();
    }
    sync() {
      const s = this.settings();
      this.root.hidden = !s;
      this.hooks.oscillators();
      if (!s) return;
      for (const button of this.waves) button.setAttribute("aria-pressed", String(Number(button.dataset.wave) === s.waveform));
      this.pattern.setAttribute("aria-pressed", String(s.pattern));
      const sequence = this.hooks.read().sequencer;
      const external = sequence?.enabled && sequence.tracks.some(t => t.target.kind === "notes" && !t.config[8]);
      this.pattern.textContent = external ? "Neuropolis" : "Pattern";
      this.pattern.disabled = external;
      this.pattern.title = external ? "Notes, gates, accent and slide come from the Neuropolis Notes track" : "Play the 303 pattern";
      this.root.querySelector(".tb303-steps").hidden = external;
      this.length.parentElement.hidden = external;
      if (document.activeElement !== this.length) this.length.value = s.length;
      this.steps.forEach((cell, i) => {
        const step = s.steps[i];
        cell.hidden = i >= s.length;
        cell.classList.toggle("is-rest", !step.gate);
        for (const field of ["gate", "accent", "slide"]) cell.querySelector(`.tb303-${field}`).setAttribute("aria-pressed", String(step[field]));
        const pitch = cell.querySelector("select");
        if (document.activeElement !== pitch) pitch.value = step.note;
      });
    }
    update(view) {
      if (this.root.hidden) return;
      this.steps.forEach((cell, i) => {
        const active = view?.step === i;
        cell.classList.toggle("is-playing", active);
        if (active) cell.setAttribute("aria-current", "step");
        else cell.removeAttribute("aria-current");
      });
    }
  }
  window.Sad303 = Bass303;
})();
