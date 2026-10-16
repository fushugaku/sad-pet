"use strict";
(() => {
  const fetch = (...args) => (window.sadPetFetch || window.fetch)(...args);
  const byId = (id) => document.getElementById(id);
  const factoryId = (track) => `factory:${track.id}:v1`;
  const makeOption = (value, label) => new Option(label, value);
  const sizeLabel = (bytes) =>
    bytes >= 1024 * 1024
      ? `${(bytes / 1024 / 1024).toFixed(1)} MB`
      : `${Math.max(1, Math.round(bytes / 1024))} KB`;
  const stem = (name) => name.replace(/\.((?:sad|evil)\.json|wav|json)$/i, "");

  class Store {
    async open() {
      this.db = await new Promise((resolve, reject) => {
        // Keep the storage name so existing libraries survive the SAD PET rename.
        const request = indexedDB.open("evil-pet-library", 1);
        request.onupgradeneeded = () => {
          const db = request.result;
          db.createObjectStore("files", { keyPath: "id" });
          db.createObjectStore("settings", { keyPath: "key" });
        };
        request.onerror = () => reject(request.error);
        request.onblocked = () =>
          reject(
            new Error("Close the other SAD PET tabs to open the library."),
          );
        request.onsuccess = () => resolve(request.result);
      });
      this.db.onversionchange = () => this.db.close();
    }
    read(store, key) {
      return new Promise((resolve, reject) => {
        const request = this.db
          .transaction(store)
          .objectStore(store)
          [key === undefined ? "getAll" : "get"](key);
        request.onsuccess = () => resolve(request.result);
        request.onerror = () => reject(request.error);
      });
    }
    write(store, value) {
      return new Promise((resolve, reject) => {
        const transaction = this.db.transaction(store, "readwrite");
        transaction.objectStore(store).put(value);
        transaction.oncomplete = () => resolve(value);
        transaction.onerror = () => reject(transaction.error);
        transaction.onabort = () =>
          reject(
            transaction.error || new Error("Library write was cancelled."),
          );
      });
    }
    async put(record) {
      const existing = await this.read("files", record.id);
      return this.write("files", {
        ...record,
        size: record.blob.size + (record.mp3Blob?.size || 0),
        addedAt: existing?.addedAt || Date.now(),
        modifiedAt: Date.now(),
        trashedAt: null,
      });
    }
    setting(key, value) {
      return value === undefined
        ? this.read("settings", key).then((item) => item?.value)
        : this.write("settings", { key, value });
    }
  }

  class FileLibrary {
    constructor(actions) {
      this.actions = actions;
      this.store = new Store();
      this.files = [];
      this.catalog = [];
      this.inflight = new Map();
      this.currentId = null;
      this.filter = "all";
      this.ready = false;
      this.initializing = true;
      this.loading = false;
      this.activation = Promise.resolve();
      this.search = "";
      this.status = "Opening library…";
      this.bind();
    }
    error(error) {
      const message =
        error.name === "QuotaExceededError"
          ? "Device storage is full. The file could not be saved to the library."
          : error.message || String(error);
      this.actions.notify(message, true);
    }
    bind() {
      byId("library-toggle").onclick = () => {
        const expanded = byId("file-library").hidden;
        byId("file-library").hidden = !expanded;
        byId("library-toggle").setAttribute("aria-expanded", String(expanded));
      };
      byId("sample-select").onchange = (event) => {
        this.activateId(event.target.value).catch((error) => this.error(error));
        event.target.blur();
      };
      byId("library-search").oninput = (event) => {
        this.search = event.target.value.trim().toLocaleLowerCase();
        this.render();
      };
      for (const button of document.querySelectorAll("[data-library-filter]"))
        button.onclick = () => {
          this.filter = button.dataset.libraryFilter;
          for (const tab of document.querySelectorAll("[data-library-filter]"))
            tab.setAttribute("aria-pressed", String(tab === button));
          this.render();
        };
      byId("library-import").onclick = () => byId("library-files").click();
      byId("library-files").onchange = async (event) => {
        try {
          await this.importFiles([...event.target.files]);
        } catch (error) {
          this.error(error);
        } finally {
          event.target.value = "";
        }
      };
      if (typeof BroadcastChannel !== "undefined") {
        this.channel = new BroadcastChannel("evil-pet-library");
        this.channel.onmessage = () => {
          if (this.ready) this.refresh().catch((error) => this.error(error));
        };
      }
    }
    async initialize(native, presets) {
      if (native.plugin && native.input?.active && native.file_source)
        native = { ...native, ...native.file_source };
      this.instanceId = native.instance_id;
      const [, response] = await Promise.all([
        this.store.open(),
        fetch("/api/samples"),
      ]);
      if (!response.ok) throw new Error("Cannot load the sample catalog.");
      this.catalog = await response.json();
      for (const [index, preset] of (this.actions.presets ? [] : presets).entries()) {
        const id = `factory-preset:${index}:v1`;
        if (await this.store.read("files", id)) continue;
        await this.store.put({
          id,
          kind: "preset",
          factory: true,
          displayName: preset.name,
          name: `${preset.name.toLowerCase().replace(/[^a-z0-9]+/g, "-")}.sad.json`,
          blob: new Blob([JSON.stringify(preset, null, 2)], {
            type: "application/json",
          }),
        });
      }
      await this.refresh();
      this.ready = true;
      byId("load-wav").disabled = false;
      byId("sample-select").disabled = false;
      byId("library-import").disabled = false;
      const last = await this.store.setting("last-audio");
      const previous = await this.previousAudio(last);
      if (
        !native.plugin &&
        previous &&
        !previous.trashedAt &&
        native.sample_id === factoryId(this.catalog[0]) &&
        !native.playing &&
        last.instance !== native.instance_id &&
        last.id !== native.sample_id
      ) {
        await this.activate(previous);
      } else {
        await this.rememberNative(native);
        if (!native.plugin && native.fades && !native.playing && !native.input?.active)
          await this.actions.loadFades(await this.fades(this.currentId));
      }
      this.status = "";
      this.initializing = false;
      byId("source-mic").disabled = false;
      this.render();
      this.seed().catch((error) => this.error(error));
    }
    changed() {
      this.channel?.postMessage("files");
    }
    async previousAudio(last) {
      if (!last) return;
      // Upgrade the previously selected soundtrack source to its new cover,
      // while retaining the user's saved original as an accessible library file.
      const cover =
        this.catalog.find((track) => track.id === "harvest-dawn-loop") ||
        this.catalog.find((track) => track.id === "harvest-dawn-cover");
      if (
        cover &&
        (last.id === "factory:harvest-dawn-excerpt:v1" ||
          (cover.id === "harvest-dawn-loop" &&
            last.id === "factory:harvest-dawn-cover:v1"))
      )
        return this.ensureFactory(cover);
      return this.store.read("files", last.id);
    }
    async fades(id) {
      return (
        (await this.store.setting(`sample-fades:${id}`)) || {
          fade_in: 0,
          fade_out: 0,
        }
      );
    }
    async saveFades(id, fades) {
      if (id) await this.store.setting(`sample-fades:${id}`, fades);
    }
    async refresh() {
      this.files = await this.store.read("files");
      const order = new Map(
        this.catalog.map((track, index) => [factoryId(track), index]),
      );
      this.files.sort((a, b) => {
        if (Boolean(a.factory) !== Boolean(b.factory))
          return a.factory ? -1 : 1;
        if (a.factory)
          return (
            (order.get(a.id) ?? 100) - (order.get(b.id) ?? 100) ||
            a.id.localeCompare(b.id)
          );
        return b.addedAt - a.addedAt || a.name.localeCompare(b.name);
      });
      this.renderSources();
      this.render();
    }
    async hash(blob) {
      const bytes = await crypto.subtle.digest(
        "SHA-256",
        await blob.arrayBuffer(),
      );
      return [...new Uint8Array(bytes)]
        .map((byte) => byte.toString(16).padStart(2, "0"))
        .join("");
    }
    async ensureFactory(track) {
      const id = factoryId(track);
      const saved = await this.store.read("files", id);
      if (saved) return saved;
      if (!this.inflight.has(id)) {
        this.inflight.set(
          id,
          (async () => {
            const response = await fetch(`/api/samples/${track.id}.wav`);
            if (!response.ok) throw new Error(`Cannot save ${track.name}.`);
            const record = await this.store.put({
              id,
              kind: "audio",
              factory: true,
              displayName: track.name,
              name: `${track.id}.wav`,
              character: track.character,
              seconds: track.seconds,
              bpm: track.bpm,
              blob: await response.blob(),
            });
            this.changed();
            await this.refresh();
            return record;
          })().finally(() => this.inflight.delete(id)),
        );
      }
      return this.inflight.get(id);
    }
    async seed() {
      const queue = [...this.catalog];
      const catalogIds = new Set(queue.map(factoryId));
      const worker = async () => {
        while (queue.length) {
          const track = queue.shift();
          this.status = `Saving tracks ${this.files.filter((file) => catalogIds.has(file.id)).length} / ${this.catalog.length}…`;
          this.render();
          await this.ensureFactory(track);
        }
      };
      try {
        await Promise.all([worker(), worker()]);
      } finally {
        this.status = "";
        this.render();
      }
    }
    async rememberNative(native) {
      this.instanceId = native.instance_id;
      const signature =
        native.sample_id ||
        `${native.instance_id}:${native.sample_name}:${native.sample_seconds}`;
      this.observed = signature;
      let record =
        native.sample_id && (await this.store.read("files", native.sample_id));
      if (!record) {
        const track = this.catalog.find(
          (track) => factoryId(track) === native.sample_id,
        );
        if (track) record = await this.ensureFactory(track);
        else {
          const response = await fetch("/api/sample.wav");
          if (!response.ok)
            throw new Error("Cannot save the current audio file.");
          const blob = await response.blob();
          record = await this.store.put({
            id: native.sample_id || `audio:${await this.hash(blob)}`,
            kind: "audio",
            name: /\.wav$/i.test(native.sample_name)
              ? native.sample_name
              : `${native.sample_name}.wav`,
            displayName: stem(native.sample_name),
            seconds: native.sample_seconds,
            blob,
          });
          this.changed();
          await this.refresh();
        }
      }
      this.currentId = record.id;
      await this.store.setting("last-audio", {
        id: record.id,
        instance: native.instance_id,
      });
      this.renderSources();
      this.renderCurrent();
    }
    observe(native) {
      if (native.input?.active) return;
      if (!this.ready || this.initializing || this.loading || this.observing)
        return;
      if (
        this.expected &&
        native.sample_id !== this.expected.id &&
        performance.now() < this.expected.until
      )
        return;
      this.expected = null;
      const signature =
        native.sample_id ||
        `${native.instance_id}:${native.sample_name}:${native.sample_seconds}`;
      if (signature === this.observed && native.instance_id === this.instanceId)
        return;
      this.observing = this.synchronizeNative(native)
        .catch((error) => this.error(error))
        .finally(() => {
          this.observing = null;
        });
    }
    async synchronizeNative(native) {
      if (
        !native.plugin &&
        native.instance_id !== this.instanceId &&
        !native.playing &&
        native.sample_id === factoryId(this.catalog[0])
      ) {
        const last = await this.store.setting("last-audio");
        const previous = await this.previousAudio(last);
        if (
          previous &&
          !previous.trashedAt &&
          previous.id !== native.sample_id
        ) {
          this.instanceId = native.instance_id;
          await this.activate(previous);
          return;
        }
      }
      await this.rememberNative(native);
    }
    async activateId(id) {
      const track = this.catalog.find((track) => factoryId(track) === id);
      const record = track
        ? await this.ensureFactory(track)
        : await this.store.read("files", id);
      if (!record) throw new Error("File is no longer in the library.");
      await this.activate(record);
    }
    activate(record) {
      const next = this.activation.then(() => this.activateNow(record));
      this.activation = next.catch(() => {});
      return next;
    }
    async activateNow(record) {
      if (!this.ready) throw new Error("The library is still opening.");
      this.loading = true;
      byId("sample-select").disabled = true;
      this.render();
      try {
        if (record.kind === "preset") {
          await this.actions.loadPreset(JSON.parse(await record.blob.text()));
        } else {
          const result = await this.actions.loadAudio(record);
          if (result.seconds && record.seconds !== result.seconds) {
            record.seconds = result.seconds;
            await this.store.write("files", record);
            await this.refresh();
          }
          this.currentId = record.id;
          this.observed = record.id;
          this.expected = { id: record.id, until: performance.now() + 8000 };
          await this.store.setting("last-audio", {
            id: record.id,
            instance: this.instanceId,
          });
        }
      } finally {
        this.loading = false;
        byId("sample-select").disabled = false;
        this.renderSources();
        this.render();
      }
    }
    async storeBlob(blob, metadata) {
      if (!this.ready) throw new Error("The library is still opening.");
      const record = await this.store.put({
        ...metadata,
        id: `${metadata.kind}:${await this.hash(blob)}`,
        blob,
      });
      this.changed();
      await this.refresh();
      return record;
    }
    async savePreset(preset, name) {
      return this.storeBlob(
        new Blob([JSON.stringify(preset, null, 2)], {
          type: "application/json",
        }),
        { kind: "preset", name, displayName: preset.name || stem(name) },
      );
    }
    async saveRender(blob, name, displayName, seconds) {
      return this.storeBlob(blob, {
        kind: "render",
        name,
        displayName,
        seconds,
      });
    }
    async saveRecording(take) {
      if (!this.ready) throw new Error("The library is still opening.");
      const id = `recording:${take.id}`;
      let record = await this.store.read("files", id);
      const read = async (extension) => {
        const response = await fetch(
          `/api/recording.${extension}?id=${take.id}`,
        );
        if (!response.ok)
          throw new Error(
            (await response.json()).error || "Cannot save recording.",
          );
        return response.blob();
      };
      if (!record) {
        const stamp = new Date()
          .toISOString()
          .slice(0, 19)
          .replace(/[-:T]/g, "");
        record = await this.store.put({
          id,
          kind: "recording",
          name: `sad-pet-${take.source === "input" ? "microphone" : "recording"}-${stamp}.wav`,
          displayName: `${take.source === "input" ? "Microphone" : "Output"} ${new Date().toLocaleString(undefined, { month: "short", day: "numeric", hour: "2-digit", minute: "2-digit", second: "2-digit" })}`,
          recordedSource: take.source || "output",
          seconds: take.seconds,
          blob: await read("wav"),
        });
        this.changed();
        await this.refresh();
      }
      if (!record.mp3Blob) {
        record = await this.store.put({
          ...record,
          mp3Blob: await read("mp3"),
        });
        this.changed();
        await this.refresh();
      }
      return record;
    }
    reveal(record) {
      this.filter = record.kind === "recording" ? "recording" : "audio";
      this.search = (
        record.displayName || stem(record.name)
      ).toLocaleLowerCase();
      byId("library-search").value = record.displayName || stem(record.name);
      for (const button of document.querySelectorAll("[data-library-filter]"))
        button.setAttribute(
          "aria-pressed",
          String(button.dataset.libraryFilter === this.filter),
        );
      byId("file-library").hidden = false;
      byId("library-toggle").setAttribute("aria-expanded", "true");
      this.render();
      byId("file-library").scrollIntoView({ block: "nearest" });
    }
    async importFiles(files) {
      if (!this.ready) throw new Error("The library is still opening.");
      let last;
      const errors = [];
      for (const file of files) {
        try {
          if (file.size > 128 * 1024 * 1024)
            throw new Error(`${file.name}: file exceeds 128 MB.`);
          if (this.actions.importPreset && /\.(sadpet|json)$/i.test(file.name)) {
            await this.actions.importPreset(file);
            continue;
          }
          if (/\.json$/i.test(file.name)) {
            const value = JSON.parse(await file.text());
            if (value.format !== "evil-pet-rust-v1")
              throw new Error(`${file.name}: use a SAD PET preset.`);
            last = await this.storeBlob(file, {
              kind: "preset",
              name: file.name,
              displayName: value.name || stem(file.name),
            });
          } else {
            const header = new Uint8Array(
              await file.slice(0, 12).arrayBuffer(),
            );
            const word = (offset) =>
              String.fromCharCode(...header.slice(offset, offset + 4));
            if (word(0) !== "RIFF" || word(8) !== "WAVE")
              throw new Error(`${file.name}: use a WAV audio file.`);
            last = await this.storeBlob(file, {
              kind: "audio",
              name: file.name,
              displayName: stem(file.name),
            });
          }
        } catch (error) {
          errors.push(error.message);
        }
      }
      if (last) {
        await this.activate(last);
        navigator.storage?.persist?.().catch(() => {});
      }
      if (errors.length) throw new Error(errors.join("\n"));
    }
    async trash(id, restore) {
      const record = await this.store.read("files", id);
      if (!record || record.factory) return;
      record.trashedAt = restore ? null : Date.now();
      await this.store.write("files", record);
      this.changed();
      await this.refresh();
    }
    async download(id, format = "wav") {
      const record = await this.store.read("files", id);
      if (!record) throw new Error("File is no longer in the library.");
      if (format === "mp3") {
        if (!record.mp3Blob)
          throw new Error("The MP3 copy has not been saved yet.");
        this.actions.download(
          record.mp3Blob,
          record.name.replace(/\.wav$/i, ".mp3"),
        );
      } else this.actions.download(record.blob, record.name);
    }
    renderSources() {
      const select = byId("sample-select");
      const factory = document.createElement("optgroup");
      factory.label = "Built-in tracks";
      for (const track of this.catalog)
        factory.append(makeOption(factoryId(track), track.name));
      const user = document.createElement("optgroup");
      user.label = "Your audio";
      const catalogIds = new Set(this.catalog.map(factoryId));
      for (const file of this.files.filter(
        (file) =>
          (!file.factory || !catalogIds.has(file.id)) &&
          file.kind !== "preset" &&
          (!file.trashedAt || file.id === this.currentId),
      ))
        user.append(makeOption(file.id, file.displayName || stem(file.name)));
      select.replaceChildren(
        factory,
        ...(user.childElementCount ? [user] : []),
      );
      if (this.currentId) select.value = this.currentId;
      if (!select.value && this.catalog.length)
        select.value = factoryId(this.catalog[0]);
    }
    renderCurrent() {
      byId("sample-select").value = this.currentId || "";
      for (const row of document.querySelectorAll(".library-file")) {
        const current = row.dataset.fileId === this.currentId;
        row.classList.toggle("current", current);
        if (current) row.setAttribute("aria-current", "true");
        else row.removeAttribute("aria-current");
      }
    }
    render() {
      const files = this.files.filter(file => !this.actions.presets || file.kind !== "preset");
      const active = files.filter((file) => !file.trashedAt);
      const total = files.reduce((sum, file) => sum + file.size, 0);
      byId("library-count").textContent = String(active.length);
      byId("library-status").textContent =
        this.status || `${active.length} files · ${sizeLabel(total)}`;
      const list = byId("library-list");
      const shown = files.filter((file) => {
        if (this.filter === "trash" ? !file.trashedAt : file.trashedAt)
          return false;
        if (
          !["all", "trash"].includes(this.filter) &&
          file.kind !== this.filter
        )
          return false;
        return `${file.displayName} ${file.name} ${file.character || ""}`
          .toLocaleLowerCase()
          .includes(this.search);
      });
      const rows = shown.map((file) => {
        const row = document.createElement("div");
        row.className = "library-file";
        row.dataset.fileId = file.id;
        row.dataset.kind = file.kind;
        const info = document.createElement("div");
        info.className = "library-file-info";
        const title = document.createElement("strong");
        title.textContent = file.displayName || stem(file.name);
        title.title = file.name;
        const detail = document.createElement("span");
        const description =
          file.kind === "preset"
            ? "Preset"
            : file.kind === "render"
              ? "Render"
              : file.kind === "recording"
                ? "Recording · WAV + MP3"
                : file.character || "Audio";
        detail.textContent = [
          file.factory ? "Built-in" : description,
          file.factory && file.kind === "audio" ? file.character : null,
          file.kind === "preset" && file.factory ? "Preset" : null,
          file.seconds ? `${file.seconds.toFixed(2)} s` : null,
          file.bpm ? `${file.bpm} BPM` : null,
          sizeLabel(file.size),
          !file.factory
            ? new Date(file.addedAt).toLocaleString(undefined, {
                month: "short",
                day: "numeric",
                hour: "2-digit",
                minute: "2-digit",
              })
            : null,
        ]
          .filter(Boolean)
          .join(" · ");
        info.append(title, detail);
        const actions = document.createElement("div");
        actions.className = "library-file-actions";
        const button = (label, accessible, action) => {
          const element = document.createElement("button");
          element.textContent = label;
          element.setAttribute(
            "aria-label",
            `${accessible} ${file.displayName || file.name}`,
          );
          element.title = accessible;
          element.onclick = () => action().catch((error) => this.error(error));
          actions.append(element);
          return element;
        };
        if (file.trashedAt)
          button("Restore", "Restore", () => this.trash(file.id, true));
        else
          button("Load", "Load", () => this.activateId(file.id)).disabled =
            this.loading;
        if (file.kind === "recording") {
          button("WAV", "Download WAV", () =>
            this.download(file.id),
          ).className = "download-format";
          const mp3 = button("MP3", "Download MP3", () =>
            this.download(file.id, "mp3"),
          );
          mp3.className = "download-format";
          mp3.disabled = !file.mp3Blob;
        } else button("↓", "Download", () => this.download(file.id));
        if (!file.factory && !file.trashedAt)
          button("×", "Move to trash", () => this.trash(file.id, false));
        row.append(info, actions);
        return row;
      });
      if (!rows.length) {
        const empty = document.createElement("p");
        empty.className = "library-empty";
        empty.textContent = this.search
          ? "No matching files"
          : this.filter === "trash"
            ? "Trash is empty"
            : "No files yet";
        rows.push(empty);
      }
      list.replaceChildren(...rows);
      this.renderCurrent();
    }
  }
  window.EvilFileLibrary = FileLibrary;
})();
