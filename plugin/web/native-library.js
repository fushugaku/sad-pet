"use strict";
// The app's library lives on disk and is read and written by native code.
// This view lists names and durations and asks the app to load, import,
// trim, render and export; no sample data reaches the editor.
(() => {
  const byId = (id) => document.getElementById(id);
  const factoryId = (track) => `factory:${track.id}:v1`;
  const wavName = (name) => (/\.wav$/i.test(name) ? name : `${name}.wav`);
  const stamp = () => new Date().toISOString().slice(0, 19).replace(/[-:T]/g, "");
  const request = async (path, data) => {
    const response = await window.sadPetFetch(
      path,
      data === undefined ? {} : { method: "POST", body: JSON.stringify(data) },
    );
    const value = await response.json();
    if (!response.ok) throw new Error(value.error || "Request failed");
    return value;
  };

  class NativeFileLibrary extends window.EvilFileLibrary {
    constructor(actions) {
      super(actions);
      this.native = true;
      this.fadeMap = {};
      this.currentName = "";
      window.sadPetLibrary = this;
    }
    bind() {
      super.bind();
      byId("library-import").onclick = () =>
        this.importDialog().catch((error) => this.error(error));
    }
    async initialize(native) {
      if ((native.input?.active || native.preset?.drums || native.preset?.synth) && native.file_source)
        native = { ...native, ...native.file_source };
      this.instanceId = native.instance_id;
      this.catalog = await request("/api/samples");
      await this.refresh();
      this.ready = true;
      this.initializing = false;
      this.status = "";
      this.remember(native);
      for (const id of ["load-wav", "sample-select", "library-import", "source-mic"])
        byId(id).disabled = false;
      this.renderSources();
      this.render();
    }
    async refresh() {
      const { files, fades } = await request("/api/library");
      this.fadeMap = fades || {};
      const factory = this.catalog.map((track) => ({
        id: factoryId(track),
        kind: "audio",
        factory: true,
        displayName: track.name,
        name: `${track.id}.wav`,
        character: track.character,
        seconds: track.seconds,
        bpm: track.bpm,
        size: Math.round(track.seconds * 48000) * 8 + 80,
        trashedAt: null,
      }));
      files.sort((a, b) => b.addedAt - a.addedAt || a.name.localeCompare(b.name));
      this.files = [...factory, ...files];
      this.renderSources();
      this.render();
    }
    remember(native) {
      this.currentId = native.sample_id || null;
      this.currentName = native.sample_name || "";
      this.observed = this.currentId;
    }
    observe(native) {
      if (!this.ready || this.loading || native.input?.active || native.preset?.drums || native.preset?.synth)
        return;
      if (this.expected && native.sample_id !== this.expected.id && performance.now() < this.expected.until)
        return;
      this.expected = null;
      if ((native.sample_id || null) === this.currentId) return;
      this.remember(native);
      this.renderSources();
      this.renderCurrent();
    }
    sourceGroups() {
      const groups = super.sourceGroups();
      // Audio recalled from a preset may not be a library file; show its name.
      if (this.currentId && !this.files.some((file) => file.id === this.currentId)) {
        groups.push({ label: "Current", choices: [[this.currentId, this.currentName || "Preset audio"]] });
      }
      return groups;
    }
    async activateId(id) {
      this.stopPreview();
      const record = this.files.find((file) => file.id === id);
      if (!record) throw new Error("File is no longer in the library.");
      await this.activate(record);
    }
    async activateNow(record) {
      if (!this.ready) throw new Error("The library is still opening.");
      this.loading = true;
      byId("sample-select").disabled = true;
      this.render();
      try {
        const result = await this.actions.loadAudio(record);
        this.currentId = result.sample_id || record.id;
        this.currentName = result.name || record.displayName;
        this.observed = this.currentId;
        this.expected = { id: this.currentId, until: performance.now() + 2000 };
      } finally {
        this.loading = false;
        byId("sample-select").disabled = false;
        this.renderSources();
        this.render();
      }
    }
    async importDialog(audioOnly = false) {
      const result = await window.sadPetNative.dialog("/api/library/import", {}, {
        mode: "open",
        title: audioOnly ? "Open WAV" : "Import audio or presets",
        patterns: audioOnly ? "*.wav" : "*.wav;*.sadpet;*.json",
        multiple: !audioOnly,
      });
      if (!result.cancelled) await this.imported(result);
    }
    // Also receives files dropped on the window, which the app imports itself.
    async imported(result) {
      await this.refresh();
      for (const record of result.presets || []) await this.actions.presetImported?.(record);
      const last = result.files?.at(-1);
      if (last) await this.activate(this.files.find((file) => file.id === last.id) || last);
      if (result.errors?.length) throw new Error(result.errors.join("\n"));
    }
    async importFiles() {
      throw new Error("Use Open WAV or + Import files: the SAD PET app opens audio files itself.");
    }
    storeBlob() {
      throw new Error("The SAD PET app stores audio natively.");
    }
    async fades(id) {
      return this.fadeMap[id] || { fade_in: 0, fade_out: 0 };
    }
    async saveFades(id, fades) {
      if (!id || !this.files.some((file) => file.id === id)) return;
      await request("/api/library/fades", { id, fades });
      this.fadeMap[id] = fades;
    }
    async added(record) {
      await this.refresh();
      return this.files.find((file) => file.id === record.id) || record;
    }
    async saveTrim(trim, metadata) {
      return this.added(await request("/api/library/trim", { ...trim, ...metadata }));
    }
    async renderScene(seconds, displayName) {
      return this.added(await request("/api/library/render", {
        seconds,
        name: `sad-pet-${stamp()}.wav`,
        displayName,
      }));
    }
    async saveRecording(take) {
      if (!this.ready) throw new Error("The library is still opening.");
      const input = take.source === "input";
      return this.added(await request("/api/library/take", {
        id: take.id,
        name: `sad-pet-${input ? "microphone" : "recording"}-${stamp()}.wav`,
        displayName: `${input ? "Microphone" : "Output"} ${new Date().toLocaleString(undefined, { month: "short", day: "numeric", hour: "2-digit", minute: "2-digit", second: "2-digit" })}`,
      }));
    }
    async trash(id, restore) {
      if (this.files.find((file) => file.id === id)?.factory) return;
      await request("/api/library/trash", { id, restore });
      await this.refresh();
    }
    async download(id) {
      const record = this.files.find((file) => file.id === id);
      if (!record) throw new Error("File is no longer in the library.");
      const name = wavName(record.factory ? record.displayName : record.name);
      const result = await window.sadPetNative.dialog("/api/library/export", { id }, {
        mode: "save",
        title: "Export WAV",
        filename: name,
        patterns: "*.wav",
      });
      if (!result.cancelled) this.actions.notify(`Saved ${name}`);
    }
  }
  window.EvilFileLibrary = NativeFileLibrary;
})();
