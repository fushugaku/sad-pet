"use strict";
(() => {
  const $ = (id) => document.getElementById(id);
  const MAGIC = Uint8Array.of(83, 65, 68, 80, 69, 84, 1, 0);
  const MIME = "application/x-sad-pet-preset";
  const MAX_FILE = 128 * 1024 * 1024;
  const MAX_STATE = 256 * 1024 * 1024;
  const title = (value) => {
    const result = String(value || "").trim();
    if (!result) throw new Error("Enter a preset name.");
    if ([...result].length > 80 || /[\x00-\x1f\x7f]/.test(result))
      throw new Error("Use a preset name of up to 80 characters.");
    return result;
  };
  const filename = (name) =>
    `${name.replace(/[\\/:*?"<>|\x00-\x1f]/g, "_")}.sadpet`;
  const base64 = (bytes) => {
    let value = "";
    for (let i = 0; i < bytes.length; i += 32768)
      value += String.fromCharCode(...bytes.subarray(i, i + 32768));
    return btoa(value);
  };
  const unbase64 = (value) =>
    Uint8Array.from(atob(value), (c) => c.charCodeAt(0));
  async function encode(record, state) {
    const metadata = { ...record, codec: "gzip" };
    const header = new TextEncoder().encode(JSON.stringify(metadata));
    const size = new Uint8Array(4);
    new DataView(size.buffer).setUint32(0, header.length, true);
    const body = new Blob([JSON.stringify(state)])
      .stream()
      .pipeThrough(new CompressionStream("gzip"));
    return new Blob([MAGIC, size, header, await new Response(body).blob()], {
      type: MIME,
    });
  }
  async function decode(blob) {
    if (blob.size > MAX_FILE)
      throw new Error("The preset file exceeds 128 MB.");
    const bytes = new Uint8Array(await blob.arrayBuffer());
    if (!MAGIC.every((b, i) => bytes[i] === b)) {
      const preset = JSON.parse(new TextDecoder().decode(bytes));
      if (preset.format !== "evil-pet-rust-v1")
        throw new Error("Use a SAD PET preset (.sadpet or .sad.json).");
      return { preset };
    }
    if (bytes.length < 12) throw new Error("The preset file is incomplete.");
    const length = new DataView(bytes.buffer).getUint32(8, true);
    if (length < 1 || length > 4096 || 12 + length >= bytes.length)
      throw new Error("The preset header is invalid.");
    const record = JSON.parse(
      new TextDecoder().decode(bytes.subarray(12, 12 + length)),
    );
    title(record.name);
    if (record.codec !== "gzip") throw new Error("Unsupported preset format.");
    const reader = new Blob([bytes.subarray(12 + length)])
      .stream()
      .pipeThrough(new DecompressionStream("gzip"))
      .getReader();
    const chunks = [];
    let size = 0;
    for (;;) {
      const { done, value } = await reader.read();
      if (done) break;
      size += value.byteLength;
      if (size > MAX_STATE) {
        await reader.cancel();
        throw new Error("The preset is too large.");
      }
      chunks.push(value);
    }
    const state = JSON.parse(await new Blob(chunks).text());
    if (
      state.version !== 1 ||
      state.preset?.format !== "evil-pet-rust-v1" ||
      !state.sample_wav
    )
      throw new Error("The preset data is invalid.");
    return { record, state };
  }

  class PresetLibrary {
    constructor(actions, files) {
      this.actions = actions;
      this.files = files;
      this.factory = [];
      this.users = [];
      this.selection = { id: "factory:0", dirty: false };
      this.bank = "factory";
      this.busy = false;
      this.ready = false;
      this.bind();
    }
    bind() {
      $("preset").onclick = () => this.toggle();
      $("preset").onkeydown = (event) => {
        if (event.key === "ArrowDown" && $("preset-browser").hidden) {
          event.preventDefault();
          this.toggle().then(() =>
            $("preset-list").querySelector("button")?.focus(),
          );
        }
      };
      $("preset-picker").addEventListener("focusout", (event) => {
        if (
          event.relatedTarget &&
          !$("preset-picker").contains(event.relatedTarget)
        )
          this.close(false);
      });
      $("preset-close").onclick = () => this.close();
      $("preset-search").oninput = () => this.renderList();
      for (const tab of document.querySelectorAll("[data-preset-bank]"))
        tab.onclick = () => {
          this.bank = tab.dataset.presetBank;
          this.renderList();
        };
      $("save-preset").onclick = () => this.openSave();
      $("preset-save-cancel").onclick = () => this.closeSave();
      $("preset-save-form").onsubmit = (event) => {
        event.preventDefault();
        this.run(() => this.save(false), $("preset-save-error"));
      };
      $("preset-save-new").onclick = () =>
        this.run(() => this.save(true), $("preset-save-error"));
      $("preset-import").onclick = () =>
        window.sadPetNative
          ? this.run(() => this.importDialog(), $("preset-error"))
          : $("preset-file").click();
      $("preset-file").onchange = async (event) => {
        const file = event.target.files[0];
        if (file)
          await this.run(() => this.importFile(file), $("preset-error"));
        event.target.value = "";
      };
      $("preset-export").onclick = () =>
        this.run(() => this.export(), $("preset-error"));
      $("preset-delete").onclick = () => {
        $("preset-delete-label").textContent =
          `Delete “${this.currentName()}”?`;
        $("preset-delete-confirm").hidden = false;
        $("preset-delete-cancel").focus();
      };
      $("preset-delete-cancel").onclick = () => {
        $("preset-delete-confirm").hidden = true;
      };
      $("preset-delete-yes").onclick = () =>
        this.run(() => this.remove(), $("preset-error"));
      document.addEventListener("pointerdown", (event) => {
        if (!$("preset-picker").contains(event.target)) this.close(false);
        if (event.target === $("preset-save-modal")) this.closeSave();
      });
      document.addEventListener(
        "keydown",
        (event) => {
          if (!$("preset-save-modal").hidden) {
            if (event.key === "Escape") {
              event.preventDefault();
              this.closeSave();
            }
            if (event.key === "Tab") {
              const focusable = [
                ...$("preset-save-form").querySelectorAll("input,button"),
              ].filter((e) => !e.disabled && !e.hidden);
              const first = focusable[0],
                last = focusable.at(-1);
              if (event.shiftKey && document.activeElement === first) {
                event.preventDefault();
                last.focus();
              } else if (!event.shiftKey && document.activeElement === last) {
                event.preventDefault();
                first.focus();
              }
            }
            event.stopImmediatePropagation();
          } else if (!$("preset-browser").hidden) {
            if (event.key === "Escape") {
              event.preventDefault();
              this.close();
            }
            if (
              event.target.matches('[role="tab"]') &&
              ["ArrowLeft", "ArrowRight"].includes(event.key)
            ) {
              event.preventDefault();
              this.bank = this.bank === "factory" ? "user" : "factory";
              this.renderList();
              document
                .querySelector(`[data-preset-bank="${this.bank}"]`)
                .focus();
            }
            if (["ArrowDown", "ArrowUp"].includes(event.key)) {
              const options = [...$("preset-list").querySelectorAll("button")];
              const index = options.indexOf(document.activeElement);
              const next =
                options[
                  (index +
                    (event.key === "ArrowDown" ? 1 : -1) +
                    options.length) %
                    options.length
                ];
              if (next) {
                event.preventDefault();
                next.focus();
              }
            }
            event.stopImmediatePropagation();
          }
        },
        true,
      );
      if (typeof BroadcastChannel !== "undefined") {
        this.channel = new BroadcastChannel("sad-pet-user-presets");
        this.channel.onmessage = () => this.refresh().catch(() => {});
      }
    }
    async initialize(state, factory) {
      this.native = Boolean(state.plugin);
      this.factory = factory;
      this.startupIndex = state.startup_preset_index ?? -1;
      if (this.native) {
        this.observe(state);
      } else if (
        !this.actions.edited?.() &&
        factory[this.startupIndex] &&
        state.preset?.name === factory[this.startupIndex]?.name
      ) {
        this.selection = {
          id: `factory:${this.startupIndex}`,
          dirty: Boolean(this.selection?.dirty),
        };
      }
      await this.refresh();
      if (!this.native) {
        const last = await this.files.store.setting("preset-selection");
        if (
          !this.actions.edited?.() &&
          last &&
          (this.users.some((r) => r.id === last.id) ||
            this.factory.some((_, i) => i !== this.startupIndex && last.id === `factory:${i}`))
        )
          await this.load(last.id);
      }
      this.ready = true;
      this.render();
    }
    async migrate() {
      for (const record of this.files.files.filter(
        (r) => r.kind === "preset" && !r.factory && !r.trashedAt,
      )) {
        await this.actions.request("/api/preset-library/import", {
          base64: base64(new Uint8Array(await record.blob.arrayBuffer())),
          import_key: record.id,
        });
      }
      await this.refresh();
    }
    async refresh() {
      this.users = this.native
        ? await this.actions.request("/api/preset-library")
        : (await this.files.store.read("files"))
            .filter((r) => r.kind === "preset" && !r.factory && !r.trashedAt)
            .map((r) => ({ ...r, name: r.displayName || r.name }));
      this.users.sort((a, b) => a.name.localeCompare(b.name));
      this.render();
    }
    currentName() {
      const current = this.actions.read();
      return current.state?.timbres?.name || current.preset?.name || "Init";
    }
    markDirty() {
      this.selection.dirty = true;
      this.renderName();
    }
    observe(state, pending = false) {
      if (this.busy) return;
      if (state.preset_selection) {
        this.selection = {
          ...state.preset_selection,
          dirty: pending || state.preset_selection.dirty,
        };
      }
      this.renderName();
    }
    renderName() {
      $("preset-name").textContent = this.currentName();
      $("preset-modified").hidden = !this.selection.dirty;
      $("preset").title =
        `${this.currentName()}${this.selection.dirty ? " · Modified" : ""}`;
    }
    render() {
      this.renderName();
      $("preset").disabled = !this.ready || this.busy;
      $("save-preset").disabled = !this.ready || this.busy;
      for (const id of [
        "preset-import",
        "preset-save-submit",
        "preset-save-new",
        "preset-delete-yes",
      ])
        $(id).disabled = this.busy;
      const user = this.users.some((r) => r.id === this.selection.id);
      $("preset-export").disabled = this.busy || !user;
      $("preset-delete").disabled = this.busy || !user;
      this.renderList();
    }
    renderList() {
      for (const tab of document.querySelectorAll("[data-preset-bank]")) {
        const active = tab.dataset.presetBank === this.bank;
        tab.setAttribute("aria-selected", String(active));
        tab.tabIndex = active ? 0 : -1;
      }
      const query = $("preset-search").value.toLocaleLowerCase().trim();
      const choices =
        this.bank === "factory"
          ? this.factory
              .map((p, i) => ({ id: `factory:${i}`, name: p.name }))
              .sort(
                (a, b) =>
                  Number(b.id === `factory:${this.startupIndex}`) -
                  Number(a.id === `factory:${this.startupIndex}`),
              )
          : this.users;
      const shown = choices.filter((p) =>
        p.name.toLocaleLowerCase().includes(query),
      );
      const list = $("preset-list");
      list.replaceChildren();
      for (const record of shown) {
        const button = document.createElement("button");
        button.className = "preset-choice";
        button.textContent = record.name;
        button.setAttribute("role", "option");
        button.setAttribute(
          "aria-selected",
          String(record.id === this.selection.id),
        );
        button.disabled = this.busy;
        button.onclick = () =>
          this.run(async () => {
            await this.load(record.id);
            this.close();
          }, $("preset-error"));
        list.append(button);
      }
      $("preset-empty").hidden = shown.length > 0;
      $("preset-empty").textContent = query
        ? "No matching presets."
        : "No user presets yet.";
    }
    async toggle() {
      if (!$("preset-browser").hidden) return this.close();
      this.bank = this.selection.id.startsWith("factory:") ? "factory" : "user";
      $("preset-error").textContent = "";
      $("preset-delete-confirm").hidden = true;
      $("preset-browser").hidden = false;
      $("preset").setAttribute("aria-expanded", "true");
      $("preset-search").value = "";
      await this.run(() => this.refresh(), $("preset-error"));
      $("preset-search").focus();
    }
    close(focus = true) {
      if ($("preset-browser").hidden) return;
      $("preset-browser").hidden = true;
      $("preset").setAttribute("aria-expanded", "false");
      if (focus) $("preset").focus();
    }
    openSave() {
      this.close(false);
      const existing = this.users.some((r) => r.id === this.selection.id);
      $("preset-save-name").value = existing
        ? this.currentName()
        : `${this.currentName()} copy`;
      $("preset-save-new").hidden = !existing;
      $("preset-save-error").textContent = "";
      $("preset-save-modal").hidden = false;
      $("preset-save-name").focus();
      $("preset-save-name").select();
    }
    closeSave() {
      if (this.busy) return;
      $("preset-save-modal").hidden = true;
      $("save-preset").focus();
    }
    async run(action, errorElement) {
      if (this.busy) return;
      this.busy = true;
      if (errorElement) errorElement.textContent = "";
      this.render();
      try {
        await action();
      } catch (error) {
        if (errorElement) errorElement.textContent = error.message;
        else this.actions.notify(error.message, true);
      } finally {
        this.busy = false;
        this.render();
      }
    }
    async snapshot(preset = this.actions.read().preset) {
      const current = this.actions.read().state;
      if (current.timbres?.timbres) {
        const override = preset !== this.actions.read().preset;
        const scene = await this.actions.request("/api/scene");
        if (override) {
          scene.preset = structuredClone(preset);
          scene.timbres = [];
          scene.active_timbre = 0;
          scene.meta = {
            id: 0,
            name: "Granular 1",
            kind: "granular",
            gain: 1,
            muted: false,
            solo: false,
            channel: 0,
          };
        }
        return scene;
      }
      const audio = await this.files.store.read("files", this.files.currentId);
      const blob =
        audio?.blob ||
        (await (await this.actions.api("/api/sample.wav")).blob());
      return {
        version: 1,
        preset: structuredClone(preset),
        sample_name: audio?.displayName || current.sample_name,
        sample_id: audio?.id || current.sample_id,
        sample_wav: base64(new Uint8Array(await blob.arrayBuffer())),
        source: current.input?.active ? current.input.mode : "file",
        region: current.region,
        fades: current.fades || { fade_in: 0, fade_out: 0 },
        sync: true,
        frozen: false,
        bypass: false,
        midi_gate: true,
        playing: false,
      };
    }
    async browserWrite(name, id, state) {
      if (
        this.users.some(
          (r) =>
            r.name.toLocaleLowerCase() === name.toLocaleLowerCase() &&
            r.id !== id,
        )
      )
        throw new Error(
          `A preset named “${name}” already exists. Choose another name.`,
        );
      const previous = id ? await this.files.store.read("files", id) : null;
      const record = {
        id: id || crypto.randomUUID().replaceAll("-", ""),
        name,
        created_at: previous?.addedAt || Date.now(),
        modified_at: Date.now(),
      };
      state.preset.name = name;
      const blob = await encode(record, state);
      await this.files.store.put({
        id: record.id,
        kind: "preset",
        displayName: name,
        name: filename(name),
        blob,
      });
      this.files.changed();
      return record;
    }
    async save(asNew) {
      const name = title($("preset-save-name").value);
      await this.actions.flush();
      const id =
        !asNew && this.users.some((r) => r.id === this.selection.id)
          ? this.selection.id
          : undefined;
      const record = this.native
        ? await this.actions.request("/api/preset-library/save", { name, id })
        : await this.browserWrite(name, id, await this.snapshot());
      if (!this.native) {
        if (this.actions.read().state?.timbres?.timbres)
          await this.actions.request("/api/scene/name", { name });
        else
          await this.actions.applyPreset({
            ...this.actions.read().preset,
            name,
          });
      }
      await this.actions.refresh();
      this.selection = { id: record.id, dirty: false };
      await this.remember();
      await this.refresh();
      $("preset-save-modal").hidden = true;
      $("save-preset").focus();
      this.actions.notify(`Saved “${name}”`);
      this.channel?.postMessage("changed");
    }
    async remember() {
      if (!this.native)
        await this.files.store.setting("preset-selection", this.selection);
    }
    async applyBrowser(state) {
      if (state.meta || Array.isArray(state.timbres)) {
        await this.actions.request("/api/scene", state);
        return;
      }
      if (this.actions.read().state?.timbres?.timbres)
        await this.actions.request("/api/timbres/reset", {});
      if (state.sample_wav) {
        const blob = new Blob([unbase64(state.sample_wav)], {
          type: "audio/wav",
        });
        await this.actions.api("/api/wav", blob, {
          "Content-Type": "audio/wav",
          "X-File-Name": encodeURIComponent(state.sample_name || "Sample.wav"),
          "X-Library-Id": state.sample_id || "",
        });
      }
      await this.actions.applyPreset(state.preset);
      if (state.sample_wav) {
        await this.actions.request("/api/region", state.region);
        await this.actions.request("/api/fades", state.fades);
      }
    }
    async load(id) {
      await this.actions.recall();
      if (this.native) {
        await this.actions.request(
          id.startsWith("factory:")
            ? "/api/preset-library/factory"
            : "/api/preset-library/load",
          id.startsWith("factory:") ? { index: Number(id.slice(8)) } : { id },
        );
      } else if (id.startsWith("factory:")) {
        const index = Number(id.slice(8));
        if (index === this.startupIndex) {
          await this.actions.request("/api/startup", { load: true });
        } else {
          if (this.actions.read().state?.timbres?.timbres)
            await this.actions.request("/api/timbres/reset", {});
          await this.actions.applyPreset(structuredClone(this.factory[index]));
        }
      } else {
        const record = await this.files.store.read("files", id);
        if (!record || record.trashedAt)
          throw new Error("This preset is no longer in the library.");
        const document = await decode(record.blob);
        if (document.state) await this.applyBrowser(document.state);
        else await this.actions.applyPreset(document.preset);
      }
      await this.actions.refresh();
      this.selection = { id, dirty: false };
      await this.remember();
      this.render();
    }
    async importFile(file) {
      await this.actions.flush();
      let record;
      if (this.native) {
        if (file.size > MAX_FILE)
          throw new Error("The preset file exceeds 128 MB.");
        record = await this.actions.request("/api/preset-library/import", {
          base64: base64(new Uint8Array(await file.arrayBuffer())),
        });
      } else {
        const document = await decode(file);
        const state = document.state || (await this.snapshot(document.preset));
        const baseName = title(state.preset.name);
        let name = baseName,
          suffix = 2;
        while (
          this.users.some(
            (r) => r.name.toLocaleLowerCase() === name.toLocaleLowerCase(),
          )
        )
          name = `${[...baseName].slice(0, 72).join("")} (${suffix++})`;
        record = await this.browserWrite(name, null, state);
      }
      await this.imported(record);
    }
    // The app reads the chosen preset file, including its audio.
    async importDialog() {
      await this.actions.flush();
      const record = await window.sadPetNative.dialog("/api/preset-library/import", {}, {
        mode: "open",
        title: "Import preset",
        patterns: "*.sadpet;*.json",
      });
      if (!record.cancelled) await this.imported(record);
    }
    async imported(record) {
      await this.refresh();
      await this.load(record.id);
      this.bank = "user";
      this.actions.notify(`Imported “${record.name}”`);
      this.channel?.postMessage("changed");
    }
    async export() {
      const record = this.users.find((r) => r.id === this.selection.id);
      if (!record) return;
      if (window.sadPetNative) {
        const result = await window.sadPetNative.dialog(
          "/api/preset-library/export",
          { id: record.id },
          { mode: "save", title: "Export preset", filename: filename(record.name), patterns: "*.sadpet" },
        );
        if (!result.cancelled) this.actions.notify(`Exported “${record.name}”`);
        return;
      }
      let blob;
      if (this.native)
        blob = await (
          await this.actions.api("/api/preset-library/export", {
            id: record.id,
          })
        ).blob();
      else {
        const stored = await this.files.store.read("files", record.id);
        const document = await decode(stored.blob);
        blob = document.state
          ? stored.blob
          : await encode(
              {
                id: crypto.randomUUID().replaceAll("-", ""),
                name: record.name,
                created_at: Date.now(),
                modified_at: Date.now(),
              },
              await this.snapshot(document.preset),
            );
      }
      this.actions.download(blob, filename(record.name));
    }
    async remove() {
      const id = this.selection.id;
      if (this.native)
        await this.actions.request("/api/preset-library/delete", { id });
      else await this.files.trash(id, false);
      this.selection = { id: "", dirty: true };
      await this.remember();
      $("preset-delete-confirm").hidden = true;
      await this.actions.refresh();
      await this.refresh();
      this.channel?.postMessage("changed");
    }
  }
  window.SadPresetLibrary = PresetLibrary;
  window.SadPresetFile = { encode, decode };

  const FX_MAGIC = Uint8Array.of(83, 65, 68, 70, 88, 1, 0, 0);
  const FX_MIME = "application/x-sad-pet-effect-preset";
  async function encodeEffect(record) {
    const body = new Blob([JSON.stringify(record)]).stream()
      .pipeThrough(new CompressionStream("gzip"));
    return new Blob([FX_MAGIC, await new Response(body).blob()], { type: FX_MIME });
  }
  async function decodeEffect(file) {
    if (file.size > 65536) throw new Error("The effect preset exceeds 64 KB.");
    const bytes = new Uint8Array(await file.arrayBuffer());
    if (!FX_MAGIC.every((b, i) => bytes[i] === b))
      throw new Error("Use a SAD PET effect preset (.sadfx).");
    const reader = new Blob([bytes.subarray(8)]).stream()
      .pipeThrough(new DecompressionStream("gzip")).getReader();
    const chunks = [];
    let size = 0;
    for (;;) {
      const { done, value } = await reader.read();
      if (done) break;
      size += value.byteLength;
      if (size > 1048576) {
        await reader.cancel();
        throw new Error("The effect preset is too large.");
      }
      chunks.push(value);
    }
    const record = JSON.parse(await new Blob(chunks).text());
    if (record.format !== "sad-pet-effect-v1" || !/^[a-f0-9]{32}$/i.test(record.id))
      throw new Error("Invalid effect preset data.");
    title(record.name);
    return record;
  }
  const fxPrefix = (kind) =>
    kind.startsWith("mooz_") ? "mooz_p" : (kind === "lcr_delay" ? "lcr" : kind) + "_";
  const closeValue = (a, b) => typeof a === "number" && typeof b === "number"
    ? Math.abs(a - b) < 0.000001 : a === b;
  class EffectPresets {
    constructor(actions, store, defaults, types, choices) {
      this.actions = actions;
      this.store = store;
      this.defaults = defaults;
      this.types = types;
      this.choices = choices;
      this.factory = [];
      this.records = [];
      this.views = new Set();
      this.selections = new Map();
      this.ready = false;
      this.busy = false;
      this.revision = 0;
      this.makeDialog();
      if (typeof BroadcastChannel !== "undefined") {
        this.channel = new BroadcastChannel("sad-pet-effect-presets");
        this.channel.onmessage = () => this.refresh().catch(() => {});
      }
    }
    async initialize() {
      this.factory = (await this.actions.request("/api/effect-presets/factory"))
        .map((record) => ({ ...this.validate(record), id: record.id, factory: true }));
      await this.refresh();
      this.ready = true;
      this.sync();
    }
    validate(data) {
      if (data.format !== "sad-pet-effect-v1" ||
          !this.types.some(([kind]) => kind === data.kind) ||
          !Number.isFinite(data.mix) || data.mix < 0 || data.mix > 1 ||
          !data.settings || typeof data.settings !== "object")
        throw new Error("Invalid effect preset data.");
      const settings = structuredClone(this.defaults.settings);
      for (const [key, initial] of Object.entries(settings)) {
        const value = data.settings[key];
        if (value === undefined) continue;
        if (typeof value !== typeof initial ||
            (typeof value === "number" && (!Number.isFinite(value) || value < 0 || value > 1)) ||
            (this.choices[key] && !this.choices[key].includes(value)))
          throw new Error("Invalid effect setting: " + key);
        settings[key] = value;
      }
      return { format: "sad-pet-effect-v1", name: title(data.name),
        kind: data.kind, mix: data.mix, settings };
    }
    async refresh() {
      const records = this.actions.native
        ? await this.actions.request("/api/effect-presets")
        : (await this.store.read("settings"))
          .filter((r) => r.key.startsWith("effect-preset:"))
          .map((r) => r.value).filter((r) => !r.trashed_at);
      records.sort((a, b) => a.name.localeCompare(b.name));
      records.push(...(this.factory || []));
      const signature = JSON.stringify(records);
      if (signature !== this.recordsSignature) {
        this.records = records;
        this.recordsSignature = signature;
        this.revision++;
      }
      this.refreshed = Date.now();
      this.sync();
    }
    label(kind) { return this.types.find(([k]) => k === kind)?.[1] || kind; }
    matches(slot, record) {
      return slot?.kind === record.kind && closeValue(slot.mix, record.mix) &&
        Object.keys(this.defaults.settings).filter((k) => k.startsWith(fxPrefix(slot.kind)))
          .every((k) => closeValue(slot.settings[k], record.settings[k] ?? this.defaults.settings[k]));
    }
    current(view) {
      const slot = view.get();
      if (!slot || (view.card && !view.card.isConnected))
        throw new Error("The effect slot changed. Select it again.");
      return slot;
    }
    match(view) {
      if (this.selections.has(view.key)) {
        const id = this.selections.get(view.key);
        return this.records.find((r) => r.id === id && this.matches(view.get(), r));
      }
      return this.records.find((r) => this.matches(view.get(), r));
    }
    attach(card, label, get, key) {
      const bar = document.createElement("div");
      bar.className = "effect-presets";
      bar.innerHTML = '<select aria-label="' + label + ' preset"></select><button class="effect-save" aria-label="Save ' + label + ' preset">Save</button><details class="effect-preset-menu"><summary aria-label="' + label + ' preset options" title="Preset options">⋯</summary><div><button data-action="import">Import…</button><button data-action="export">Export…</button><button data-action="delete">Delete</button><button data-action="undo">Undo delete</button></div></details>';
      const view = { card, bar, get, key, select: bar.querySelector("select"), revision: -1 };
      this.views.add(view);
      card.querySelector(".color-title").after(bar);
      view.select.onchange = () => {
        const id = view.select.value;
        if (!id) return;
        this.run(async () => {
          const slot = this.current(view);
          const record = id === "init"
            ? { ...this.defaults, kind: slot.kind, mix: slot.kind === "compressor" ? 1 : this.defaults.mix }
            : this.records.find((r) => r.id === id);
          if (!record || record.kind !== slot.kind) throw new Error("Choose a preset of the same effect type.");
          this.selections.set(view.key, id === "init" ? null : id);
          await this.actions.apply(slot.id, record);
        });
      };
      view.select.onfocus = () => {
        if (this.ready && Date.now() - this.refreshed > 1000)
          this.refresh().catch((e) => this.actions.notify(e.message, true));
      };
      bar.querySelector(".effect-save").onclick = () =>
        this.run(async () => { await this.refresh(); this.openSave(view); });
      for (const button of bar.querySelectorAll("[data-action]"))
        button.onclick = () => {
          bar.querySelector("details").open = false;
          this.run(() => this[button.dataset.action](view));
        };
      this.paint(view);
    }
    sync() {
      for (const view of this.views) {
        if (!view.card.isConnected) this.views.delete(view);
        else this.paint(view);
      }
    }
    paint(view) {
      const slot = view.get();
      if (!slot) return;
      if (view.revision !== this.revision) {
        view.select.replaceChildren(new Option("Custom", ""), new Option("Init", "init"));
        for (const [label, factory] of [["Factory", true], ["User", false]]) {
          const records = this.records.filter((r) => r.kind === slot.kind && Boolean(r.factory) === factory);
          if (!records.length) continue;
          const group = document.createElement("optgroup");
          group.label = label;
          group.append(...records.map((r) => new Option(r.name, r.id)));
          view.select.append(group);
        }
        view.revision = this.revision;
      }
      const match = this.match(view);
      if (match) this.selections.set(view.key, match.id);
      if (document.activeElement !== view.select) {
        const initial = this.matches(slot, {
          ...this.defaults, kind: slot.kind,
          mix: slot.kind === "compressor" ? 1 : this.defaults.mix,
        }) && !this.selections.get(view.key);
        view.select.value = match?.id || (initial ? "init" : "");
      }
      for (const input of view.bar.querySelectorAll("select,button"))
        input.disabled = !this.ready || this.busy;
      view.bar.querySelector('[data-action="export"]').disabled = !this.ready || this.busy || !match;
      view.bar.querySelector('[data-action="delete"]').disabled = !this.ready || this.busy || !match || Boolean(match.factory);
      view.bar.querySelector('[data-action="undo"]').disabled = !this.ready || this.busy || !this.deleted;
    }
    async run(action) {
      if (this.busy || !this.ready) return;
      this.busy = true;
      this.sync();
      try { await action(); }
      catch (e) { this.error.textContent = e.message; this.actions.notify(e.message, true); }
      finally {
        this.busy = false;
        this.sync();
        this.focusAfterSave?.focus();
        this.focusAfterSave = null;
      }
    }
    makeDialog() {
      this.modal = document.createElement("div");
      this.modal.className = "effect-preset-modal";
      this.modal.hidden = true;
      this.modal.setAttribute("role", "dialog");
      this.modal.setAttribute("aria-modal", "true");
      this.modal.setAttribute("aria-labelledby", "effect-save-title");
      this.modal.innerHTML = '<form><h2 id="effect-save-title">Save effect preset</h2><label>Name<input aria-label="Effect preset name" maxlength="80" autocomplete="off" required></label><p class="effect-preset-error" role="alert"></p><div class="effect-save-actions"><button type="button" data-save="cancel">Cancel</button><button type="button" data-save="new">Save as new</button><button type="submit">Save</button></div></form>';
      document.body.append(this.modal);
      this.name = this.modal.querySelector("input");
      this.error = this.modal.querySelector(".effect-preset-error");
      this.modal.querySelector("form").onsubmit = (e) => { e.preventDefault(); this.run(() => this.save(false)); };
      this.modal.querySelector('[data-save="new"]').onclick = () => this.run(() => this.save(true));
      this.modal.querySelector('[data-save="cancel"]').onclick = () => this.closeSave();
      this.modal.onclick = (e) => { if (e.target === this.modal) this.closeSave(); };
      document.addEventListener("keydown", (e) => {
        if (this.modal.hidden) return;
        if (e.key === "Escape") { e.preventDefault(); this.closeSave(); }
        if (e.key === "Tab") {
          const controls = [...this.modal.querySelectorAll("input,button")].filter((x) => !x.disabled && !x.hidden);
          const first = controls[0], last = controls.at(-1);
          if (e.shiftKey && document.activeElement === first) { e.preventDefault(); last.focus(); }
          if (!e.shiftKey && document.activeElement === last) { e.preventDefault(); first.focus(); }
        }
        e.stopImmediatePropagation();
      }, true);
    }
    openSave(view) {
      const slot = this.current(view);
      this.saving = view;
      const selected = this.records.find((r) => r.id === this.selections.get(view.key) && r.kind === slot.kind);
      this.previous = selected?.factory ? undefined : selected;
      this.modal.querySelector("h2").textContent = "Save " + this.label(slot.kind) + " preset";
      this.name.value = selected?.name || this.label(slot.kind);
      this.modal.querySelector('[data-save="new"]').hidden = !this.previous;
      this.error.textContent = "";
      this.modal.hidden = false;
      this.name.focus();
      this.name.select();
    }
    closeSave() {
      if (this.busy) return;
      this.modal.hidden = true;
      this.saving?.bar.querySelector(".effect-save").focus();
    }
    async write(preset, id) {
      preset = this.validate(preset);
      if (this.actions.native)
        return this.actions.request("/api/effect-presets/save", { preset, id });
      if (this.records.some((r) => !r.factory && r.id !== id && r.kind === preset.kind &&
          r.name.toLocaleLowerCase() === preset.name.toLocaleLowerCase()))
        throw new Error("An effect preset named “" + preset.name + "” already exists.");
      const previous = id ? await this.store.setting("effect-preset:" + id) : null;
      if (id && (!previous || previous.trashed_at)) throw new Error("This preset was removed. Use Save as new.");
      if (previous && previous.kind !== preset.kind)
        throw new Error("Choose a preset of the same effect type.");
      const record = { ...preset, id: id || crypto.randomUUID().replaceAll("-", ""),
        created_at: previous?.created_at || Date.now(), modified_at: Date.now() };
      await this.store.setting("effect-preset:" + record.id, record);
      return record;
    }
    async changed() {
      await this.refresh();
      this.channel?.postMessage("changed");
    }
    async save(asNew) {
      const slot = this.current(this.saving);
      const preset = { format: "sad-pet-effect-v1", name: this.name.value,
        kind: slot.kind, mix: slot.mix, settings: structuredClone(slot.settings) };
      await this.actions.flush();
      const record = await this.write(preset, asNew ? undefined : this.previous?.id);
      this.selections.set(this.saving.key, record.id);
      await this.changed();
      this.modal.hidden = true;
      this.focusAfterSave = this.saving.bar.querySelector(".effect-save");
      this.actions.notify("Saved “" + record.name + "”");
    }
    async export(view) {
      const record = this.match(view);
      if (!record) return;
      const portable = record.factory
        ? { ...this.validate(record), id: crypto.randomUUID().replaceAll("-", ""),
            created_at: Date.now(), modified_at: Date.now() }
        : record;
      if (window.sadPetNative) {
        const result = await window.sadPetNative.dialog(
          "/api/effect-presets/export",
          record.factory ? { preset: this.validate(record) } : { id: record.id },
          { mode: "save", title: "Export effect preset", filename: filename(record.name).replace(/\.sadpet$/, ".sadfx"), patterns: "*.sadfx" },
        );
        if (!result.cancelled) this.actions.notify("Exported “" + record.name + "”");
        return;
      }
      const blob = this.actions.native && !record.factory
        ? await (await this.actions.api("/api/effect-presets/export", { id: record.id })).blob()
        : await encodeEffect(portable);
      this.actions.download(blob, filename(record.name).replace(/\.sadpet$/, ".sadfx"));
    }
    async import(view) {
      if (window.sadPetNative) {
        // Already inside run(): the app's dialog keeps the menu busy until it closes.
        const record = await window.sadPetNative.dialog(
          "/api/effect-presets/import",
          { kind: this.current(view).kind },
          { mode: "open", title: "Import effect preset", patterns: "*.sadfx" },
        );
        if (record.cancelled) return;
        await this.changed();
        await this.actions.apply(this.current(view).id, record);
        return;
      }
      const input = document.createElement("input");
      input.type = "file";
      input.accept = ".sadfx";
      input.onchange = () => this.run(async () => {
        const file = input.files[0];
        if (!file) return;
        if (file.size > 65536) throw new Error("The effect preset exceeds 64 KB.");
        const kind = this.current(view).kind;
        let record;
        if (this.actions.native)
          record = await this.actions.request("/api/effect-presets/import", {
            base64: base64(new Uint8Array(await file.arrayBuffer())),
            kind,
          });
        else {
          const preset = this.validate(await decodeEffect(file));
          if (preset.kind !== kind) throw new Error("Choose a preset of the same effect type.");
          const base = preset.name;
          let suffix = 2;
          while (this.records.some((r) => !r.factory && r.kind === preset.kind && r.name.toLocaleLowerCase() === preset.name.toLocaleLowerCase()))
            preset.name = [...base].slice(0, 70).join("") + " (" + suffix++ + ")";
          record = await this.write(preset);
        }
        await this.changed();
        const slot = this.current(view);
        await this.actions.apply(slot.id, record);
      });
      input.click();
    }
    async delete(view) {
      const record = this.match(view);
      if (!record || record.factory) return;
      if (this.actions.native) await this.actions.request("/api/effect-presets/delete", { id: record.id });
      else await this.store.setting("effect-preset:" + record.id, { ...record, trashed_at: Date.now() });
      this.deleted = record;
      await this.changed();
      this.actions.notify("Deleted “" + record.name + "”");
    }
    async undo() {
      if (!this.deleted) return;
      if (this.records.some((r) => !r.factory && r.kind === this.deleted.kind &&
          r.name.toLocaleLowerCase() === this.deleted.name.toLocaleLowerCase()))
        throw new Error("A preset with this name already exists. Rename it before undoing the deletion.");
      if (this.actions.native) await this.actions.request("/api/effect-presets/restore", { id: this.deleted.id });
      else await this.store.setting("effect-preset:" + this.deleted.id, { ...this.deleted, trashed_at: null });
      this.deleted = null;
      await this.changed();
    }
  }
  window.SadEffectPresets = EffectPresets;
  window.SadEffectPresetFile = { encode: encodeEffect, decode: decodeEffect };

  // Instrument presets: one timbre's sound with its sample (or kit), filed
  // in the user's own categories. Loading one replaces the active timbre's
  // sound or adds it as a new timbre; the other timbres play on.
  const INSTRUMENT_MAGIC = Uint8Array.of(83, 65, 68, 73, 78, 83, 1, 0);
  const INSTRUMENT_MIME = "application/x-sad-pet-instrument";
  const INSTRUMENT_KEY = "instrument:";
  const INSTRUMENT_KINDS = { granular: "Granular", drums: "Drums", synth: "Synth", vocal: "Vocal", sampler: "Sampler" };
  const sameName = (a, b) => a.toLocaleLowerCase() === b.toLocaleLowerCase();
  const categoryName = (value) => {
    const result = String(value || "").trim();
    if ([...result].length > 40 || /[\x00-\x1f\x7f]/.test(result))
      throw new Error("Use a category name of up to 40 characters.");
    return result;
  };
  const instrumentFile = (name) => filename(name).replace(/\.sadpet$/, ".sadinst");
  const taken = (name, category) => new Error(category
    ? `“${category}” already has an instrument named “${name}”.`
    : `An instrument named “${name}” already exists.`);
  /** What the library lists of an instrument, without its file. */
  const listed = ({ id, name, category, kind, created_at, modified_at }) =>
    ({ id, name, category, kind, created_at, modified_at });
  function withHeader(record, body) {
    const header = new TextEncoder().encode(JSON.stringify({ ...listed(record), codec: "gzip" }));
    const size = new Uint8Array(4);
    new DataView(size.buffer).setUint32(0, header.length, true);
    return new Blob([INSTRUMENT_MAGIC, size, header, body], { type: INSTRUMENT_MIME });
  }
  async function encodeInstrument(record, state) {
    const body = new Blob([JSON.stringify(state)]).stream()
      .pipeThrough(new CompressionStream("gzip"));
    return withHeader(record, await new Response(body).blob());
  }
  /** A file's header and its compressed timbre state. */
  async function splitInstrument(blob) {
    if (blob.size > MAX_FILE) throw new Error("The instrument preset exceeds 128 MB.");
    const head = new Uint8Array(await blob.slice(0, 12).arrayBuffer());
    if (head.length < 12 || !INSTRUMENT_MAGIC.every((b, i) => head[i] === b))
      throw new Error("Use a SAD PET instrument preset (.sadinst).");
    const length = new DataView(head.buffer).getUint32(8, true);
    if (length < 1 || length > 4096 || 12 + length >= blob.size)
      throw new Error("The instrument preset header is invalid.");
    let record;
    try {
      record = JSON.parse(await blob.slice(12, 12 + length).text());
    } catch {
      throw new Error("The instrument preset header is invalid.");
    }
    const { id, kind, created_at, modified_at } = record;
    if (!/^[a-f0-9]{32}$/i.test(id) || !INSTRUMENT_KINDS[kind] || record.codec !== "gzip")
      throw new Error("Unsupported instrument preset format.");
    return {
      record: { id, kind, created_at, modified_at,
        name: title(record.name), category: categoryName(record.category) },
      body: blob.slice(12 + length),
    };
  }
  async function decodeInstrument(blob) {
    const { record, body } = await splitInstrument(blob);
    const reader = body.stream().pipeThrough(new DecompressionStream("gzip")).getReader();
    const chunks = [];
    let size = 0;
    try {
      for (;;) {
        const { done, value } = await reader.read();
        if (done) break;
        size += value.byteLength;
        if (size > MAX_STATE) {
          await reader.cancel();
          throw new Error("The instrument preset is too large.");
        }
        chunks.push(value);
      }
    } catch (error) {
      throw error.message.includes("too large") ? error
        : new Error("The instrument preset is damaged or incomplete.");
    }
    let state;
    try {
      state = JSON.parse(await new Blob(chunks).text());
    } catch {
      throw new Error("The instrument data is invalid.");
    }
    if (state.version !== 1 || state.preset?.format !== "evil-pet-rust-v1" ||
        typeof state.sample_wav !== "string" || state.timbres?.length)
      throw new Error("The instrument data is invalid.");
    return { record, state };
  }

  class InstrumentPresets {
    /** `picker` holds the button that opens the browser, in the timbre's row. */
    constructor(picker, actions, store) {
      this.picker = picker;
      this.button = picker.querySelector("button");
      this.actions = actions;
      this.store = store;
      this.records = [];
      // null shows every category; "" the instruments in none.
      this.category = null;
      this.busy = false;
      this.build();
      this.makeDialog();
      if (typeof BroadcastChannel !== "undefined") {
        this.channel = new BroadcastChannel("sad-pet-instruments");
        this.channel.onmessage = () => {
          if (!this.browser.hidden) this.refresh().catch(() => {});
        };
      }
    }
    build() {
      const browser = (this.browser = document.createElement("div"));
      browser.className = "instrument-browser";
      browser.id = "instrument-browser";
      browser.hidden = true;
      browser.setAttribute("role", "dialog");
      browser.setAttribute("aria-label", "Instrument presets");
      browser.innerHTML = '<div class="instrument-top"><input type="search" placeholder="Search instruments" aria-label="Search instruments" autocomplete="off"><button class="instrument-close" aria-label="Close instrument presets">×</button></div><p class="instrument-target"></p><div class="instrument-body"><div class="instrument-categories" role="group" aria-label="Categories"></div><div class="instrument-list" role="list" aria-label="Instruments"></div></div><div class="instrument-confirm" hidden><span></span><div><button data-confirm="no">Cancel</button><button data-confirm="yes">Delete</button></div></div><p class="preset-error" role="alert"></p><div class="instrument-actions"><button data-act="save">Save…</button><button data-act="import">Import…</button><button data-act="category" hidden>Rename category…</button></div>';
      this.picker.after(browser);
      this.search = browser.querySelector('[type="search"]');
      this.target = browser.querySelector(".instrument-target");
      this.categories = browser.querySelector(".instrument-categories");
      this.list = browser.querySelector(".instrument-list");
      this.confirm = browser.querySelector(".instrument-confirm");
      this.error = browser.querySelector(".preset-error");
      this.button.setAttribute("aria-controls", browser.id);
      this.button.onclick = () => this.toggle();
      this.search.oninput = () => this.renderList();
      // An open row menu stays under its button only while the list is still.
      this.list.onscroll = () => {
        for (const menu of this.list.querySelectorAll("details[open]")) menu.open = false;
      };
      browser.querySelector(".instrument-close").onclick = () => this.close();
      browser.querySelector('[data-act="save"]').onclick = () => this.openDialog("save");
      browser.querySelector('[data-act="import"]').onclick = () => this.run(() => this.import());
      browser.querySelector('[data-act="category"]').onclick = () => this.openDialog("category");
      this.confirm.querySelector('[data-confirm="no"]').onclick = () => {
        this.confirm.hidden = true;
        this.deleting = null;
      };
      this.confirm.querySelector('[data-confirm="yes"]').onclick = () =>
        this.run(() => this.remove(this.deleting));
      document.addEventListener("pointerdown", (event) => {
        for (const menu of this.list.querySelectorAll("details[open]"))
          if (!menu.contains(event.target)) menu.open = false;
        if (!this.browser.hidden && this.modal.hidden &&
            !this.browser.contains(event.target) && !this.picker.contains(event.target))
          this.close(false);
      });
      // Keys typed here do not play the computer keyboard.
      browser.addEventListener("keydown", (event) => {
        if (event.key === "Escape") {
          event.preventDefault();
          this.close();
        }
        event.stopPropagation();
      });
    }
    makeDialog() {
      this.modal = document.createElement("div");
      this.modal.className = "effect-preset-modal instrument-modal";
      this.modal.hidden = true;
      this.modal.setAttribute("role", "dialog");
      this.modal.setAttribute("aria-modal", "true");
      this.modal.setAttribute("aria-labelledby", "instrument-dialog-title");
      this.modal.innerHTML = '<form><h2 id="instrument-dialog-title"></h2><label data-field="name">Name<input aria-label="Instrument name" maxlength="80" autocomplete="off"></label><label>Category<input aria-label="Category" maxlength="40" autocomplete="off" list="instrument-category-list" placeholder="No category"></label><datalist id="instrument-category-list"></datalist><p class="effect-preset-error" role="alert"></p><div class="effect-save-actions"><button type="button" data-dialog="cancel">Cancel</button><button type="submit"></button></div></form>';
      document.body.append(this.modal);
      [this.nameInput, this.categoryInput] = this.modal.querySelectorAll("input");
      this.dialogError = this.modal.querySelector(".effect-preset-error");
      this.submit = this.modal.querySelector('[type="submit"]');
      this.suggestions = this.modal.querySelector("datalist");
      const label = () => this.labelSubmit();
      this.nameInput.oninput = label;
      this.categoryInput.oninput = label;
      this.modal.querySelector("form").onsubmit = (e) => {
        e.preventDefault();
        this.run(() => this.finishDialog());
      };
      this.modal.querySelector('[data-dialog="cancel"]').onclick = () => this.closeDialog();
      this.modal.onclick = (e) => {
        if (e.target === this.modal) this.closeDialog();
      };
      this.modal.addEventListener("keydown", (e) => {
        if (e.key === "Escape") {
          e.preventDefault();
          this.closeDialog();
        }
        if (e.key === "Tab") {
          const controls = [...this.modal.querySelectorAll("input,button")]
            .filter((x) => !x.disabled && !x.closest("[hidden]"));
          const first = controls[0], last = controls.at(-1);
          if (e.shiftKey && document.activeElement === first) { e.preventDefault(); last.focus(); }
          if (!e.shiftKey && document.activeElement === last) { e.preventDefault(); first.focus(); }
        }
        e.stopPropagation();
      });
    }
    /** The active timbre, which loads replace and Save keeps. */
    part() {
      const context = this.actions.context();
      return context?.timbres?.find((t) => t.id === context.active);
    }
    playable(record) {
      return !(this.actions.native && record.kind === "sampler");
    }
    async toggle() {
      if (!this.browser.hidden) return this.close();
      this.confirm.hidden = true;
      this.error.textContent = "";
      this.search.value = "";
      this.browser.hidden = false;
      this.button.setAttribute("aria-expanded", "true");
      this.render();
      await this.run(() => this.refresh());
      this.search.focus();
    }
    close(focus = true) {
      if (this.browser.hidden) return;
      this.browser.hidden = true;
      this.button.setAttribute("aria-expanded", "false");
      if (focus) this.button.focus();
    }
    async run(action) {
      if (this.busy) return;
      this.busy = true;
      const error = this.modal.hidden ? this.error : this.dialogError;
      error.textContent = "";
      this.render();
      try {
        await action();
      } catch (e) {
        error.textContent = e.message;
        if (this.browser.hidden && this.modal.hidden) this.actions.notify(e.message, true);
      } finally {
        this.busy = false;
        this.render();
      }
    }
    async refresh() {
      let records;
      if (this.actions.native) records = await this.actions.request("/api/instruments");
      else {
        if (!this.store.db) throw new Error("The library is still opening. Try again in a moment.");
        records = (await this.store.read("settings"))
          .filter((item) => item.key.startsWith(INSTRUMENT_KEY))
          .map((item) => listed(item.value));
      }
      // By category, those in none last, then by name.
      records.sort((a, b) => Number(!a.category) - Number(!b.category) ||
        a.category.localeCompare(b.category) || a.name.localeCompare(b.name));
      this.records = records;
      if (this.category !== null && !records.some((r) => r.category === this.category))
        this.category = null;
      this.render();
    }
    async changed() {
      await this.refresh();
      this.channel?.postMessage("changed");
    }
    render() {
      if (this.browser.hidden) return;
      const part = this.part();
      const holds = part && INSTRUMENT_KINDS[part.kind];
      this.target.textContent = !part ? ""
        : holds ? `Loads into “${part.name}”. + adds a new timbre.`
          : `${part.name} holds no instrument. + adds one as a new timbre.`;
      for (const button of this.browser.querySelectorAll(".instrument-actions button"))
        button.disabled = this.busy;
      this.browser.querySelector('[data-act="save"]').disabled = this.busy || !holds;
      this.browser.querySelector('[data-act="category"]').hidden = !this.category;
      this.confirm.querySelector('[data-confirm="yes"]').disabled = this.busy;
      this.renderCategories();
      this.renderList();
    }
    renderCategories() {
      const counts = new Map();
      for (const r of this.records) counts.set(r.category, (counts.get(r.category) || 0) + 1);
      const names = [...counts.keys()].filter(Boolean);
      const choices = [[null, "All", this.records.length], ...names.map((n) => [n, n, counts.get(n)])];
      if (counts.has("")) choices.push(["", "No category", counts.get("")]);
      this.categories.replaceChildren(...choices.map(([value, label, count]) => {
        const button = document.createElement("button");
        button.className = "instrument-category";
        button.setAttribute("aria-pressed", String(value === this.category));
        const name = document.createElement("span");
        name.textContent = label;
        const total = document.createElement("span");
        total.className = "instrument-count";
        total.textContent = count;
        button.append(name, total);
        button.title = label;
        button.onclick = () => {
          this.category = value;
          this.confirm.hidden = true;
          this.render();
        };
        return button;
      }));
      this.suggestions.replaceChildren(...names.map((n) => new Option(n)));
    }
    renderList() {
      const query = this.search.value.toLocaleLowerCase().trim();
      const part = this.part();
      const holds = part && INSTRUMENT_KINDS[part.kind];
      const shown = this.records.filter((r) =>
        (this.category === null || r.category === this.category) &&
        `${r.name} ${r.category} ${INSTRUMENT_KINDS[r.kind]}`.toLocaleLowerCase().includes(query));
      const rows = [];
      let heading;
      for (const record of shown) {
        if (this.category === null && record.category !== heading) {
          heading = record.category;
          const title = document.createElement("h4");
          title.className = "instrument-heading";
          title.textContent = heading || "No category";
          rows.push(title);
        }
        rows.push(this.row(record, part, holds));
      }
      if (!shown.length) {
        const empty = document.createElement("p");
        empty.className = "instrument-empty";
        empty.textContent = query ? "No matching instruments."
          : this.records.length ? "No instruments in this category."
            : "No instruments yet. Save this timbre’s sound with Save….";
        rows.push(empty);
      }
      this.list.replaceChildren(...rows);
    }
    row(record, part, holds) {
      const row = document.createElement("div");
      row.className = "instrument-row";
      row.setAttribute("role", "listitem");
      const playable = this.playable(record);
      const load = document.createElement("button");
      load.className = "instrument-load";
      const name = document.createElement("span");
      name.className = "instrument-name";
      name.textContent = record.name;
      const kind = document.createElement("span");
      kind.className = "instrument-kind";
      kind.textContent = INSTRUMENT_KINDS[record.kind];
      load.append(name, kind);
      load.title = !playable ? "Sampler instruments play in the browser version for now"
        : holds ? `Load into ${part.name}` : "Use + to load it as a new timbre";
      load.disabled = this.busy || !holds || !playable;
      load.onclick = () => this.run(() => this.load(record, false));
      if (part && record.kind === part.kind && sameName(record.name, part.name)) {
        row.classList.add("current");
        load.setAttribute("aria-current", "true");
      }
      const add = document.createElement("button");
      add.className = "instrument-add";
      add.textContent = "+";
      add.title = "Load as a new timbre";
      add.setAttribute("aria-label", `Load ${record.name} as a new timbre`);
      add.disabled = this.busy || !playable;
      add.onclick = () => this.run(() => this.load(record, true));
      const menu = document.createElement("details");
      menu.className = "effect-preset-menu instrument-menu";
      menu.innerHTML = '<summary title="Instrument options">⋯</summary><div><button data-act="edit">Rename or move…</button><button data-act="export">Export…</button><button data-act="delete">Delete…</button></div>';
      menu.querySelector("summary").setAttribute("aria-label", `${record.name} options`);
      // Placed before it opens, so it never shows anywhere else.
      menu.querySelector("summary").onclick = (event) => {
        if (menu.open) return;
        for (const other of this.list.querySelectorAll("details[open]")) other.open = false;
        const box = event.currentTarget.getBoundingClientRect();
        const options = menu.querySelector("div");
        options.style.top = `${box.bottom + 4}px`;
        options.style.left = `${Math.max(8, box.right - 156)}px`;
        options.style.right = "auto";
      };
      for (const button of menu.querySelectorAll("button")) {
        button.disabled = this.busy;
        button.onclick = () => {
          menu.open = false;
          const act = button.dataset.act;
          if (act === "edit") this.openDialog("edit", record);
          else if (act === "export") this.run(() => this.export(record));
          else this.askDelete(record);
        };
      }
      row.append(load, add, menu);
      return row;
    }
    openDialog(mode, record = null) {
      const part = this.part();
      this.dialog = { mode, record };
      this.modal.querySelector('[data-field="name"]').hidden = mode === "category";
      this.nameInput.required = mode !== "category";
      if (mode === "save") {
        // The timbre's own preset, when it came from one, is the one to replace.
        const own = this.records.find((r) => r.kind === part?.kind && sameName(r.name, part?.name || ""));
        this.nameInput.value = part?.name || "";
        this.categoryInput.value = own?.category ?? this.category ?? "";
      } else if (mode === "edit") {
        this.nameInput.value = record.name;
        this.categoryInput.value = record.category;
      } else this.categoryInput.value = this.category || "";
      this.modal.querySelector("h2").textContent = mode === "save"
        ? `Save ${part?.name || "timbre"} as an instrument`
        : mode === "edit" ? "Rename or move instrument" : `Rename “${this.category}”`;
      this.dialogError.textContent = "";
      this.labelSubmit();
      this.modal.hidden = false;
      const field = mode === "category" ? this.categoryInput : this.nameInput;
      field.focus();
      field.select();
    }
    /** Save says Replace when an instrument already has that name there. */
    labelSubmit() {
      const { mode } = this.dialog || {};
      if (mode === "save") {
        const name = this.nameInput.value.trim(), category = this.categoryInput.value.trim();
        this.submit.textContent = this.records.some((r) => sameName(r.name, name) && sameName(r.category, category))
          ? "Replace" : "Save";
      } else this.submit.textContent = mode === "edit" ? "Save" : "Rename";
    }
    closeDialog() {
      if (this.busy) return;
      this.modal.hidden = true;
      (this.browser.hidden ? this.button : this.browser.querySelector(".instrument-actions button")).focus();
    }
    async finishDialog() {
      const { mode, record } = this.dialog;
      const category = categoryName(this.categoryInput.value);
      if (mode === "save") await this.save(title(this.nameInput.value), category);
      else if (mode === "edit") await this.edit(record, title(this.nameInput.value), category);
      else await this.renameCategory(this.category, category);
      this.modal.hidden = true;
      if (!this.browser.hidden) this.search.focus();
    }
    async save(name, category) {
      const part = this.part();
      if (!part || !INSTRUMENT_KINDS[part.kind])
        throw new Error("Choose an instrument timbre to save. Bus and Master hold no instrument.");
      await this.actions.flush();
      const existing = this.records.find((r) => sameName(r.name, name) && sameName(r.category, category));
      if (this.actions.native)
        await this.actions.request("/api/instruments/save", { name, category, id: existing?.id });
      else {
        const state = await this.actions.request("/api/timbres/document");
        if (!INSTRUMENT_KINDS[state.meta?.kind]) throw new Error("This timbre holds no instrument.");
        // The sound alone: no mix, place in the scene or transport.
        Object.assign(state, { meta: { name, kind: state.meta.kind }, source: "file", sync: true,
          frozen: false, bypass: false, playing: false, timbres: [], active_timbre: 0 });
        delete state.selection;
        const now = Date.now();
        const record = { id: existing?.id || crypto.randomUUID().replaceAll("-", ""), name, category,
          kind: state.meta.kind, created_at: existing?.created_at || now, modified_at: now };
        await this.store.setting(INSTRUMENT_KEY + record.id,
          { ...record, blob: await encodeInstrument(record, state) });
      }
      // The timbre takes the instrument's name, which marks it in the list.
      if (part.name !== name) await this.actions.rename(part.id, name);
      if (this.category !== null) this.category = category;
      await this.changed();
      this.actions.notify(`Saved “${name}”`);
    }
    async load(record, asNew) {
      await this.actions.change(async () => {
        if (this.actions.native)
          return this.actions.request("/api/instruments/load", { id: record.id, new: asNew });
        const stored = await this.store.setting(INSTRUMENT_KEY + record.id);
        if (!stored) throw new Error("This instrument is no longer in the library.");
        const { state } = await decodeInstrument(stored.blob);
        const id = asNew
          ? (await this.actions.request("/api/timbres/add", { kind: record.kind })).active
          : this.part()?.id;
        return this.actions.request("/api/timbres/load", { id, state, name: record.name });
      }, !asNew);
      this.close();
      this.actions.notify(`Loaded “${record.name}”`);
    }
    async rewrite(record, next) {
      const stored = await this.store.setting(INSTRUMENT_KEY + record.id);
      if (!stored) throw new Error("This instrument is no longer in the library.");
      const { body } = await splitInstrument(stored.blob);
      await this.store.setting(INSTRUMENT_KEY + record.id, { ...next, blob: withHeader(next, body) });
    }
    async edit(record, name, category) {
      if (this.records.some((r) => r.id !== record.id && sameName(r.name, name) && sameName(r.category, category)))
        throw taken(name, category);
      if (this.actions.native)
        await this.actions.request("/api/instruments/edit", { id: record.id, name, category });
      else await this.rewrite(record, { ...record, name, category, modified_at: Date.now() });
      if (this.category !== null) this.category = category;
      await this.changed();
    }
    async renameCategory(from, to) {
      const moving = this.records.filter((r) => r.category === from);
      for (const record of moving)
        if (this.records.some((r) => r.category !== from && sameName(r.category, to) && sameName(r.name, record.name)))
          throw taken(record.name, to);
      if (this.actions.native) await this.actions.request("/api/instruments/category", { from, to });
      else
        for (const record of moving)
          await this.rewrite(record, { ...record, category: to, modified_at: Date.now() });
      this.category = to;
      await this.changed();
    }
    askDelete(record) {
      this.deleting = record;
      this.confirm.querySelector("span").textContent = `Delete “${record.name}”?`;
      this.confirm.hidden = false;
      this.confirm.querySelector('[data-confirm="no"]').focus();
    }
    async remove(record) {
      if (!record) return;
      if (this.actions.native) await this.actions.request("/api/instruments/delete", { id: record.id });
      else await this.store.remove("settings", INSTRUMENT_KEY + record.id);
      this.deleting = null;
      this.confirm.hidden = true;
      await this.changed();
      this.actions.notify(`Deleted “${record.name}”`);
    }
    async export(record) {
      if (window.sadPetNative) {
        const result = await window.sadPetNative.dialog(
          "/api/instruments/export",
          { id: record.id },
          { mode: "save", title: "Export instrument", filename: instrumentFile(record.name), patterns: "*.sadinst" },
        );
        if (!result.cancelled) this.actions.notify(`Exported “${record.name}”`);
        return;
      }
      const blob = this.actions.native
        ? await (await this.actions.api("/api/instruments/export", { id: record.id })).blob()
        : (await this.store.setting(INSTRUMENT_KEY + record.id))?.blob;
      if (!blob) throw new Error("This instrument is no longer in the library.");
      this.actions.download(blob, instrumentFile(record.name));
    }
    async import() {
      if (window.sadPetNative) {
        // Already inside run(): the app's dialog keeps the browser busy until it closes.
        const record = await window.sadPetNative.dialog("/api/instruments/import", {},
          { mode: "open", title: "Import instrument", patterns: "*.sadinst" });
        if (!record.cancelled) await this.imported(record);
        return;
      }
      const input = document.createElement("input");
      input.type = "file";
      input.accept = ".sadinst";
      input.onchange = () => this.run(async () => {
        const file = input.files[0];
        if (file) await this.imported(await this.importFile(file));
      });
      input.click();
    }
    async importFile(file) {
      if (this.actions.native) {
        if (file.size > MAX_FILE) throw new Error("The instrument preset exceeds 128 MB.");
        return this.actions.request("/api/instruments/import",
          { base64: base64(new Uint8Array(await file.arrayBuffer())) });
      }
      const { record } = await decodeInstrument(file);
      const { body } = await splitInstrument(file);
      // A copy of its own, under a name free in its category.
      const now = Date.now();
      const next = { ...record, id: crypto.randomUUID().replaceAll("-", ""), created_at: now, modified_at: now };
      for (let suffix = 2; this.records.some((r) => sameName(r.category, next.category) && sameName(r.name, next.name)); suffix++)
        next.name = `${[...record.name].slice(0, 72).join("")} (${suffix})`;
      await this.store.setting(INSTRUMENT_KEY + next.id, { ...next, blob: withHeader(next, body) });
      return next;
    }
    async imported(record) {
      this.category = record.category;
      await this.changed();
      this.actions.notify(`Imported “${record.name}”`);
    }
  }
  window.SadInstrumentPresets = InstrumentPresets;
  window.SadInstrumentFile = { encode: encodeInstrument, decode: decodeInstrument };
})();
