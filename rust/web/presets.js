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
      $("preset-import").onclick = () => $("preset-file").click();
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
      if (this.native) {
        this.observe(state);
      }
      await this.refresh();
      if (!this.native) {
        const last = await this.files.store.setting("preset-selection");
        if (
          !this.actions.edited?.() &&
          last &&
          this.users.some((r) => r.id === last.id)
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
      return this.actions.read().preset?.name || "Init";
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
          ? this.factory.map((p, i) => ({ id: `factory:${i}`, name: p.name }))
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
      if (!this.native)
        await this.actions.applyPreset({ ...this.actions.read().preset, name });
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
        await this.actions.applyPreset(
          structuredClone(this.factory[Number(id.slice(8))]),
        );
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
      await this.refresh();
      await this.load(record.id);
      this.bank = "user";
      this.actions.notify(`Imported “${record.name}”`);
      this.channel?.postMessage("changed");
    }
    async export() {
      const record = this.users.find((r) => r.id === this.selection.id);
      if (!record) return;
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
})();
