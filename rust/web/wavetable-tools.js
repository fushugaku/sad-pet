"use strict";
(() => {
  const node = (tag, cls, text) => {
    const element = document.createElement(tag);
    if (cls) element.className = cls;
    if (text !== undefined) element.textContent = text;
    return element;
  };
  const encode = async (file) => {
    const bytes = new Uint8Array(await file.arrayBuffer());
    let text = "";
    for (let n = 0; n < bytes.length; n += 16384)
      text += String.fromCharCode(...bytes.subarray(n, n + 16384));
    return btoa(text);
  };
  const defaults = () => ({ reverse: false, phase: 0, align: false, bits: 24,
    phase_steps: 2048, harmonics: 1023, tilt: 0, normalize: false, frame: null, gains: [], locals: [] });

  class Tools {
    constructor(editor) { this.editor = editor; }
    dialog(i, title, kind) {
      const ed = this.editor;
      ed.dialog?.close();
      const dialog = node("dialog", `wt-import-dialog wt-tools-dialog ${kind}`);
      dialog.waveIndex = i;
      dialog.waveSettings = ed.settings(i);
      ed.dialog = dialog;
      dialog.innerHTML = `<div class="wt-dialog-heading"><h2></h2><button type="button" aria-label="Close ${title}">×</button></div><div class="wt-tool-body"></div><canvas class="wt-import-wave" role="img" aria-label="${title} preview"></canvas><p class="wt-tool-status" role="status"></p><p class="wt-error" role="alert" hidden></p><div class="wt-dialog-actions"><button type="button" class="wt-preview" disabled>Preview</button><button type="button" class="wt-cancel">Cancel</button><button type="button" class="wt-apply" disabled>Apply</button></div>`;
      dialog.querySelector("h2").textContent = title;
      const previous = document.activeElement;
      const close = () => { if (!dialog.committing) dialog.close(); };
      dialog.querySelector(".wt-dialog-heading button").onclick = close;
      dialog.querySelector(".wt-cancel").onclick = close;
      dialog.oncancel = (event) => { if (dialog.committing) event.preventDefault(); };
      dialog.valid = () => dialog.isConnected && ed.settings(i) === dialog.waveSettings;
      dialog.error = (error) => {
        const output = dialog.querySelector(".wt-error");
        output.textContent = error?.message ?? String(error ?? "");
        output.hidden = !output.textContent;
      };
      dialog.status = (text) => { dialog.querySelector(".wt-tool-status").textContent = text; };
      dialog.onclose = () => {
        clearTimeout(dialog.debounce);
        clearTimeout(dialog.captureTimer);
        if (dialog.capturing) ed.hooks.request("/api/capture/stop", {}).catch(() => {});
        ed.stopAudition();
        if (dialog.generation) ed.hooks.request("/api/wavetables/preview-clear", { generation: dialog.generation }).catch(() => {});
        dialog.remove();
        if (ed.dialog === dialog) ed.dialog = null;
        if (previous?.isConnected) previous.focus({ preventScroll: true });
      };
      dialog.querySelector(".wt-preview").onclick = () => dialog.record && ed.audition(dialog.record.id, (status) => {
        dialog.querySelector(".wt-preview").textContent = status === "loading" ? "Loading…" : status === "playing" ? "Stop preview" : "Preview";
      });
      dialog.querySelector(".wt-apply").onclick = async () => {
        if (!dialog.valid() || !dialog.record || dialog.busy) return;
        dialog.committing = true;
        dialog.querySelector(".wt-apply").disabled = true;
        dialog.error(null);
        try {
          const record = await ed.hooks.request("/api/wavetables/commit", { id: dialog.record.id, generation: dialog.generation });
          ed.previews.set(record.id, Promise.resolve(record));
          await ed.catalog();
          if (dialog.valid()) await ed.choose(i, record.id);
          dialog.committing = false;
          dialog.close();
        } catch (error) {
          dialog.error(error);
          dialog.committing = false;
          dialog.querySelector(".wt-apply").disabled = false;
        }
      };
      dialog.prepare = (path, payload, done) => {
        clearTimeout(dialog.debounce);
        const generation = window.SadWavetable.nextGeneration();
        dialog.generation = generation;
        dialog.record = null;
        dialog.busy = true;
        dialog.querySelector(".wt-apply").disabled = true;
        dialog.querySelector(".wt-preview").disabled = true;
        ed.stopAudition();
        dialog.status("Preparing…");
        dialog.debounce = setTimeout(async () => {
          try {
            const record = await ed.hooks.request(path, { ...payload, generation });
            if (!dialog.valid() || generation !== dialog.generation) return;
            dialog.record = record;
            dialog.error(null);
            dialog.status(`${record.frames} frames`);
            dialog.querySelector(".wt-apply").disabled = false;
            dialog.querySelector(".wt-preview").disabled = false;
            this.draw(dialog, record.waves[0]);
            done?.(record);
          } catch (error) {
            if (dialog.valid() && generation === dialog.generation) { dialog.error(error); dialog.status(""); }
          } finally {
            if (generation === dialog.generation) dialog.busy = false;
          }
        }, 160);
      };
      document.body.append(dialog);
      dialog.showModal();
      return dialog;
    }
    draw(dialog, wave) {
      if (!wave) return;
      const canvas = dialog.querySelector(".wt-import-wave"), c = canvas.getContext("2d");
      canvas.width = 800; canvas.height = 160;
      c.strokeStyle = "#c4f486"; c.lineWidth = 1.5; c.beginPath();
      wave.forEach((value, n) => { const x = 8 + n / (wave.length - 1) * 784, y = 80 - value * 66; n ? c.lineTo(x, y) : c.moveTo(x, y); });
      c.stroke();
    }
    async edit(i) {
      const ed = this.editor, original = ed.settings(i);
      try {
        const source = await ed.preview(original.asset_id);
        if (ed.settings(i) !== original) return;
        const dialog = this.dialog(i, "Edit table", "wt-frame-editor");
        const body = dialog.querySelector(".wt-tool-body");
        body.innerHTML = `<label class="wt-tool-name">Name<input maxlength="80" aria-label="Edited table name"></label><div class="wt-filmstrip" role="listbox" aria-label="Table frames"></div><div class="wt-frame-actions"><button type="button" data-action="left" aria-label="Move frame left">←</button><button type="button" data-action="right" aria-label="Move frame right">→</button><button type="button" data-action="duplicate">Duplicate</button><button type="button" data-action="delete">Delete</button><button type="button" data-action="reverse">Reverse</button></div><div class="wt-trim"><label>First<input type="number" min="1" value="1" aria-label="Trim first frame"></label><label>Last<input type="number" min="1" aria-label="Trim last frame"></label><button type="button" class="wt-trim-apply">Trim</button></div><details class="wt-tool-details" open><summary>Frame spectrum</summary><label>Phase<input class="wt-frame-phase" type="range" min="0" max="1" step=".001" aria-label="Selected frame phase"></label><div class="wt-harmonics"></div></details><details class="wt-tool-details"><summary>Character</summary><div class="wt-edit-fields"></div><div class="wt-edit-checks"><label><input type="checkbox" data-setting="align">Align phases</label><label><input type="checkbox" data-setting="normalize">Normalize</label></div></details>`;
        const name = body.querySelector(".wt-tool-name input"); name.value = `${source.name} edit`;
        const settings = defaults();
        let frames = Array.from({ length: source.frames }, (_, index) => ({ source: index, phase: 0, gains: Array(16).fill(1) }));
        let active = 0, rendered = null;
        const controls = body.querySelector(".wt-edit-fields");
        for (const [key, label, choices] of [
          ["bits", "Bit depth", [[24, "Full"], [16, "16 bit"], [12, "12 bit"], [8, "8 bit"], [6, "6 bit"], [4, "4 bit"]]],
          ["phase_steps", "Phase resolution", [2048, 1024, 512, 256, 128, 64, 32, 16].map(n => [n, String(n)])],
        ]) {
          const holder = node("label", "", label), select = node("select"); select.setAttribute("aria-label", `Table ${label.toLowerCase()}`);
          for (const [value, text] of choices) { const option = node("option", "", text); option.value = value; select.append(option); }
          select.value = settings[key]; select.onchange = () => { settings[key] = Number(select.value); prepare(); };
          holder.append(select); controls.append(holder);
        }
        for (const [key, label, min, max, step] of [["harmonics", "Harmonics", 1, 1023, 1], ["tilt", "Tilt dB/oct", -12, 12, .1]]) {
          const holder = node("label", "", label), input = node("input"); input.type = "number"; input.min = min; input.max = max; input.step = step; input.value = settings[key]; input.setAttribute("aria-label", `Table ${label}`);
          input.onchange = () => { const value = Number(input.value); if (Number.isFinite(value) && value >= min && value <= max) { settings[key] = value; prepare(); } }; holder.append(input); controls.append(holder);
        }
        const harmonicInputs = [];
        for (let h = 1; h <= 16; h++) {
          const label = node("label", "", `H${h}`), input = node("input"); input.type = "range"; input.min = 0; input.max = 4; input.step = .01;
          input.oninput = () => { frames[active].gains[h - 1] = Number(input.value); prepare(); };
          label.append(input); body.querySelector(".wt-harmonics").append(label); harmonicInputs.push(input);
        }
        const filmstrip = body.querySelector(".wt-filmstrip"), phase = body.querySelector(".wt-frame-phase"), first = body.querySelector('[aria-label="Trim first frame"]'), last = body.querySelector('[aria-label="Trim last frame"]');
        const selectFrame = () => {
          active = Math.min(active, frames.length - 1);
          phase.value = frames[active].phase;
          harmonicInputs.forEach((input, n) => { input.value = frames[active].gains[n]; input.setAttribute("aria-label", `Frame ${active + 1} harmonic ${n + 1} gain`); });
          [...filmstrip.children].forEach((button, index) => { button.setAttribute("aria-selected", String(index === active)); button.tabIndex = index === active ? 0 : -1; });
          this.draw(dialog, rendered?.waves[active] ?? source.waves[frames[active].source]);
        };
        const strip = () => {
          filmstrip.replaceChildren();
          frames.forEach((frame, index) => {
            const button = node("button", "wt-frame-thumb"); button.type = "button"; button.role = "option"; button.setAttribute("aria-label", `Frame ${index + 1}`);
            const canvas = node("canvas"); canvas.width = 96; canvas.height = 36; const c = canvas.getContext("2d"); c.strokeStyle = "#91b77b"; c.beginPath();
            const wave = rendered?.waves[index] ?? source.waves[frame.source]; wave.forEach((v, n) => { const x = n / (wave.length - 1) * 96, y = 18 - v * 15; n ? c.lineTo(x, y) : c.moveTo(x, y); }); c.stroke();
            button.append(canvas, node("span", "", String(index + 1))); button.onclick = () => { active = index; selectFrame(); };
            button.onkeydown = event => { if (!["ArrowLeft", "ArrowRight"].includes(event.key)) return; event.preventDefault(); active = Math.max(0, Math.min(frames.length - 1, active + (event.key === "ArrowLeft" ? -1 : 1))); selectFrame(); filmstrip.children[active].focus(); };
            filmstrip.append(button);
          });
          if(dialog.stripCount!==frames.length){first.value=1;last.value=frames.length;dialog.stripCount=frames.length;}first.max=last.max=frames.length;
          body.querySelector('[data-action="delete"]').disabled = frames.length <= 1;
          body.querySelector('[data-action="duplicate"]').disabled = frames.length >= 256;
          selectFrame();
        };
        const prepare = () => {
          rendered = null;
          const locals = frames.flatMap((frame, index) => frame.phase || frame.gains.some(g => g !== 1) ? [{ frame: index, phase: frame.phase, gains: frame.gains.flatMap((gain, n) => gain === 1 ? [] : [{ harmonic: n + 1, gain }]) }] : []);
          dialog.prepare("/api/wavetables/edit", { id: original.asset_id, name: name.value.trim() || source.name, settings: { ...settings, order: frames.map(f => f.source), locals } }, record => { rendered = record; strip(); });
        };
        for (const button of body.querySelectorAll("[data-action]")) button.onclick = () => {
          const action = button.dataset.action;
          if (action === "left" && active > 0) { [frames[active - 1], frames[active]] = [frames[active], frames[active - 1]]; active--; }
          if (action === "right" && active + 1 < frames.length) { [frames[active + 1], frames[active]] = [frames[active], frames[active + 1]]; active++; }
          if (action === "duplicate" && frames.length < 256) { frames.splice(active + 1, 0, structuredClone(frames[active])); active++; }
          if (action === "delete" && frames.length > 1) frames.splice(active, 1);
          if (action === "reverse") { frames.reverse(); active = frames.length - 1 - active; }
          rendered = null; strip(); prepare();
        };
        body.querySelector(".wt-trim-apply").onclick = () => {
          const from = Number(first.value), to = Number(last.value);
          if (!Number.isInteger(from) || !Number.isInteger(to) || from < 1 || to < from || to > frames.length) return dialog.error(new Error("Choose a valid frame range"));
          frames = frames.slice(from - 1, to); active = 0; rendered = null; strip(); prepare();
        };
        for (const input of body.querySelectorAll("[data-setting]")) input.onchange = () => { settings[input.dataset.setting] = input.checked; prepare(); };
        phase.oninput = () => { frames[active].phase = Number(phase.value); prepare(); };
        name.onchange = prepare;
        strip(); prepare();
      } catch (error) { if (ed.settings(i) === original) ed.error(i, error); }
    }

    maker(i) {
      const ed = this.editor, dialog = this.dialog(i, "Wave Maker", "wt-maker");
      const body = dialog.querySelector(".wt-tool-body");
      body.innerHTML = `<div class="wt-maker-source"><button type="button" class="wt-maker-file">Choose WAV</button><button type="button" class="wt-capture">Capture output · 2 s</button><input type="file" accept=".wav,audio/wav" hidden></div><p class="wt-maker-hint">Capture hears the scene output. Solo a Rack to isolate it; use MIDI or a running sequence while capturing.</p><label class="wt-tool-name">Name<input aria-label="Made table name" maxlength="80" value="Captured Wavetable"></label><div class="wt-maker-fields"><label>Method<select aria-label="Wave Maker method"><option value="spectral">Spectral snapshots</option><option value="cycles">Pitch cycles</option></select></label><label>Frames<input aria-label="Wave Maker frames" type="number" min="1" max="256" value="64"></label><label>Channel<select aria-label="Wave Maker channel"><option value="mix">Mix</option><option value="left">Left</option><option value="right">Right</option></select></label><label class="wt-maker-root">Root note<select aria-label="Wave Maker root note"><option value="">Detect pitch</option></select></label><label>Start · s<input aria-label="Wave Maker start" type="number" min="0" step=".01" value="0"></label><label>End · s<input aria-label="Wave Maker end" type="number" min="0" step=".01"></label></div>`;
      let wav = null, duration = 0;
      const file = body.querySelector('input[type="file"]'), name = body.querySelector('[aria-label="Made table name"]'), method = body.querySelector('[aria-label="Wave Maker method"]'), frames = body.querySelector('[aria-label="Wave Maker frames"]'), channel = body.querySelector('[aria-label="Wave Maker channel"]'), root = body.querySelector('[aria-label="Wave Maker root note"]'), start = body.querySelector('[aria-label="Wave Maker start"]'), end = body.querySelector('[aria-label="Wave Maker end"]'), capture = body.querySelector(".wt-capture");
      for (let note = 0; note < 128; note++) { const option = node("option", "", ["C", "C♯", "D", "D♯", "E", "F", "F♯", "G", "G♯", "A", "A♯", "B"][note % 12] + (Math.floor(note / 12) - 1)); option.value = note; root.append(option); }
      const prepare = () => {
        body.querySelector(".wt-maker-root").hidden = method.value !== "cycles";
        if (!wav) return;
        dialog.prepare("/api/wavetables/make", { wav, name: name.value.trim() || "Captured Wavetable", settings: { method: method.value, frames: Number(frames.value), start: Number(start.value), end: end.value === "" ? duration : Number(end.value), channel: channel.value, root_note: root.value === "" ? null : Number(root.value) } });
      };
      const use = async (value, filename) => {
        const info = await ed.hooks.request("/api/wavetables/import", { wav: value, inspect: true });
        if (!dialog.valid()) return;
        wav = value; duration = info.samples_per_channel / info.sample_rate; name.value = filename;
        start.value = "0"; end.value = Math.min(10, duration).toFixed(3); end.max = start.max = duration;
        channel.closest("label").hidden = info.channels === 1; prepare();
      };
      body.querySelector(".wt-maker-file").onclick = () => file.click();
      file.onchange = async () => {
        try { if (!file.files[0]) return; dialog.captureGeneration=(dialog.captureGeneration??0)+1;clearTimeout(dialog.captureTimer);if(dialog.capturing){await ed.hooks.request("/api/capture/stop",{});dialog.capturing=false;capture.textContent="Capture output · 2 s";} if (file.files[0].size > 16 * 1024 * 1024) throw new Error("The WAV exceeds 16 MiB"); const source = file.files[0]; await use(await encode(source), source.name.replace(/\.wav$/i, "")); }
        catch (error) { dialog.error(error); }
        file.value = "";
      };
      capture.onclick = async () => {
        try {
          if (dialog.capturing) { await ed.hooks.request("/api/capture/stop", {}); return; }
          const generation=dialog.captureGeneration=(dialog.captureGeneration??0)+1;
          dialog.record=null;dialog.querySelector(".wt-apply").disabled=true;dialog.querySelector(".wt-preview").disabled=true;ed.stopAudition();
          dialog.error(null); dialog.capturing = true; capture.textContent = "Stop capture";
          await ed.hooks.request("/api/capture/start", { seconds: 2, source: "output" });
          const poll = async () => {
            if (!dialog.valid()||generation!==dialog.captureGeneration) return;
            try {
              const view = await ed.hooks.request("/api/capture/status");
              if (!dialog.valid()||generation!==dialog.captureGeneration) return;
              dialog.status(`Capturing · ${view.seconds.toFixed(1)} / 2 s`);
              if (view.ready) {
                dialog.capturing = false; capture.textContent = "Capture output · 2 s"; dialog.status("Reading capture…");
                const take = await ed.hooks.request("/api/capture/wav");
                if (dialog.valid()&&generation===dialog.captureGeneration) await use(take.wav, "Captured Wavetable");
              } else dialog.captureTimer = setTimeout(poll, 150);
            } catch (error) { dialog.capturing = false; capture.textContent = "Capture output · 2 s"; dialog.error(error); }
          };
          poll();
        } catch (error) { dialog.capturing = false; capture.textContent = "Capture output · 2 s"; dialog.error(error); }
      };
      for (const input of [name, method, frames, channel, root, start, end]) input.onchange = prepare;
      prepare();
    }
  }
  window.SadWavetableTools = Tools;
})();
