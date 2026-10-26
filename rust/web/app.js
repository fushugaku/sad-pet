"use strict";
const $ = (id) => document.getElementById(id);
let state = null,
  preset = null,
  presets = [],
  pending = 0,
  editing = 0,
  commitTimer = null,
  noticeTimer = null,
  connected = true;
const controls = new Map(),
  modulationViews = new Map(),
  held = new Set(),
  scopes = [];
let noteChain = Promise.resolve();
let fileLibrary = null;
let presetLibrary = null;
let effectPresetLibrary = null;
let presetWrites = Promise.resolve();
let presetWriteCount = 0;
let structuralPending = false;
let editGeneration = 0;
const parameterEdits = new Map();
const sequenceEdits = new Map();
let neuroEditor = null;
let timbreEditor = null,
  busMixer = null,
  drumEditor = null,
  vocalEditor = null,
  samplerEditor = null,
  timbreBusy = false;
const drumEdits = new Map();
let sequenceCatalog = null;
let sampleSelection = null;
let recordingBusy = false,
  recordingAction = null,
  lastSavedRecording = null,
  savedRecordingId = null,
  failedRecordingId = null;
let keyboardOctave = 4;
const inputNotes = new Map();
const KEY_OFFSETS = {
  KeyA: 0,
  KeyW: 1,
  KeyS: 2,
  KeyE: 3,
  KeyD: 4,
  KeyF: 5,
  KeyT: 6,
  KeyG: 7,
  KeyY: 8,
  KeyH: 9,
  KeyU: 10,
  KeyJ: 11,
  KeyK: 12,
};
const formatTime = (s) =>
  `${String(Math.floor(s / 60)).padStart(2, "0")}:${(s % 60).toFixed(2).padStart(5, "0")}`;
const percent = (v) => `${Math.round(v * 100)}%`;
function notify(message, persistent = false) {
  clearTimeout(noticeTimer);
  $("notice").textContent = message;
  $("notice").hidden = false;
  if (!persistent)
    noticeTimer = setTimeout(() => ($("notice").hidden = true), 6500);
}
async function api(path, data, headers) {
  const response = await (window.sadPetFetch || window.fetch)(
    path,
    data === undefined
      ? {}
      : {
          method: "POST",
          headers: headers || { "Content-Type": "application/json" },
          body: headers ? data : JSON.stringify(data),
        },
  );
  if (!response.ok) {
    let error;
    try {
      error = (await response.json()).error;
    } catch {
      error = response.statusText;
    }
    throw new Error(error || "Request failed");
  }
  return response;
}
// The native engine answers a poll that names the previous answer's token
// with only what changed; omitted parts keep their objects, so the editor and
// its waveform cache can tell they did not change.
let stateCache = null;
// Polls come faster while a MIDI controller moves, so a learned knob follows it.
let midiMovedAt = -Infinity;
async function readState() {
  if (window.sadPetState) return window.sadPetState();
  const token = stateCache?.state_token;
  const next = await (
    await api(token ? `/api/state?since=${token}` : "/api/state")
  ).json();
  if (next.delta && stateCache)
    for (const key of ["preset", "waveform", "file_source"])
      if (!(key in next)) next[key] = stateCache[key];
  if (next.knob_changes) {
    next.knobs = { ...stateCache?.knobs };
    for (const [key, value] of Object.entries(next.knob_changes))
      if (value === null) delete next.knobs[key];
      else next.knobs[key] = value;
    delete next.knob_changes;
  }
  stateCache = next;
  // Native engines list the controllers moved on their MIDI input since the
  // last answer; the browser build hands them to MIDI learn directly.
  if (next.midi_controls?.length) {
    midiMovedAt = performance.now();
    for (const [channel, controller, value] of next.midi_controls)
      window.SadMidiLearn?.input(channel, controller, value);
  }
  return next;
}
async function request(path, data) {
  if (path === "/api/state" && data === undefined) return readState();
  const result = await (await api(path, data)).json();
  if (
    data !== undefined &&
    [
      "/api/preset",
      "/api/parameters",
      "/api/sequencer/edit",
      "/api/drums/edit",
      "/api/scene",
      "/api/input",
      "/api/region",
      "/api/fades",
    ].includes(path)
  )
    presetLibrary?.markDirty();
  return result;
}
function download(blob, name) {
  if (window.sadPetNative)
    throw new Error("The SAD PET app saves files through its own dialogs.");
  const url = URL.createObjectURL(blob),
    a = document.createElement("a");
  a.href = url;
  a.download = name;
  a.click();
  setTimeout(() => URL.revokeObjectURL(url), 1000);
}
function schedulePreset() {
  editGeneration++;
  presetLibrary?.markDirty();
  pending++;
  structuralPending = true;
  parameterEdits.clear();
  for (const [key, edit] of sequenceEdits)
    if (!sequenceCommand(edit)) sequenceEdits.delete(key);
  clearTimeout(commitTimer);
  armPresetWrite(50);
}
function scheduleParameter(id, value) {
  if (!Number.isFinite(value)) return;
  editGeneration++;
  presetLibrary?.markDirty();
  pending++;
  parameterEdits.set(id, value);
  // Throttle while dragging; do not wait for the gesture to stop.
  if (commitTimer === null && !presetWriteCount) armPresetWrite(16);
}
function scheduleSequence(edit) {
  editGeneration++;
  presetLibrary?.markDirty();
  pending++;
  sequenceEdits.set(`${edit.track}:${edit.stage}:${edit.field}`, edit);
  if (commitTimer === null && !presetWriteCount) armPresetWrite(16);
}
function sequenceCommand(edit) {
  return (
    edit.stage === null &&
    ((edit.track === 0 && edit.field === 1) ||
      edit.field === 47 ||
      edit.field === 48)
  );
}
function armPresetWrite(delay) {
  commitTimer = setTimeout(() => {
    commitTimer = null;
    return commitEdits().catch((e) => notify(e.message));
  }, delay);
}
function enqueuePresetWrite(path, value) {
  presetWriteCount++;
  const write = presetWrites.catch(() => {}).then(() => request(path, value));
  presetWrites = write.finally(() => {
    presetWriteCount--;
    if (
      !presetWriteCount &&
      commitTimer === null &&
      (structuralPending ||
        parameterEdits.size ||
        sequenceEdits.size ||
        drumEdits.size)
    )
      armPresetWrite(16);
  });
  return presetWrites;
}
function writePreset(value) {
  return enqueuePresetWrite("/api/preset", structuredClone(value));
}
function commitEdits() {
  if (
    !structuralPending &&
    ((!parameterEdits.size && !sequenceEdits.size && !drumEdits.size) ||
      presetWriteCount)
  )
    return presetWrites;
  const version = pending;
  const full = structuralPending;
  const values = Object.fromEntries(parameterEdits);
  const seqValues = [...sequenceEdits.values()];
  const drumValues = [...drumEdits.values()];
  drumEdits.clear();
  structuralPending = false;
  parameterEdits.clear();
  sequenceEdits.clear();
  const sequenceValues = full ? seqValues.filter(sequenceCommand) : seqValues;
  const writes = [];
  if (full) writes.push(writePreset(preset));
  else if (Object.keys(values).length)
    writes.push(enqueuePresetWrite("/api/parameters", values));
  if (sequenceValues.length)
    writes.push(enqueuePresetWrite("/api/sequencer/edit", sequenceValues));
  if (!full && drumValues.length)
    writes.push(enqueuePresetWrite("/api/drums/edit", drumValues));
  const write = Promise.all(writes);
  return write.then(
    () => {
      if (version === pending) pending = 0;
    },
    (error) => {
      if (full) structuralPending = true;
      else {
        for (const [id, value] of Object.entries(values))
          if (!parameterEdits.has(id)) parameterEdits.set(id, value);
        for (const edit of seqValues) {
          const key = `${edit.track}:${edit.stage}:${edit.field}`;
          if (!sequenceEdits.has(key)) sequenceEdits.set(key, edit);
        }
      }
      if (!full)
        for (const edit of drumValues) {
          const key = `${edit.track}:${edit.step}:${edit.field}`;
          if (!drumEdits.has(key)) drumEdits.set(key, edit);
        }
      if (full)
        for (const edit of sequenceValues) {
          const key = `${edit.track}:${edit.stage}:${edit.field}`;
          if (!sequenceEdits.has(key)) sequenceEdits.set(key, edit);
        }
      throw error;
    },
  );
}
async function flushEdits() {
  do {
    clearTimeout(commitTimer);
    commitTimer = null;
    await commitEdits();
  } while (
    structuralPending ||
    parameterEdits.size ||
    sequenceEdits.size ||
    drumEdits.size ||
    presetWriteCount
  );
  clearTimeout(commitTimer);
  commitTimer = null;
}
async function flushPreset() {
  await flushEdits();
  await sampleSelection?.flush();
  await refreshPresetState();
}
async function preparePresetRecall() {
  effectPresetLibrary?.selections.clear();
  if (drumEditor) {
    drumEditor.views.clear();
    drumEditor.signature = "";
    drumEditor.page = "";
  }
  releaseKeyboard();
  clearTimeout(commitTimer);
  commitTimer = null;
  structuralPending = false;
  parameterEdits.clear();
  sequenceEdits.clear();
  drumEdits.clear();
  pending = 0;
  await presetWrites.catch(() => {});
  // A failed in-flight write may have restored its unsent changes.
  clearTimeout(commitTimer);
  commitTimer = null;
  structuralPending = false;
  parameterEdits.clear();
  sequenceEdits.clear();
  drumEdits.clear();
  await queueStepEntry(null, true);
}
async function refreshPresetState() {
  state = await request("/api/state");
  preset = structuredClone(state.preset);
  syncAll();
  sampleSelection?.sync(state);
  fileLibrary?.observe(state);
}
function syncRecording() {
  const take = state?.recording;
  const active = !!take?.active;
  const retry = take?.available && take.id === failedRecordingId;
  $("record").disabled =
    recordingBusy ||
    !!state?.input?.pending ||
    !fileLibrary?.ready ||
    !connected;
  $("record").setAttribute("aria-pressed", String(active));
  $("record").setAttribute(
    "aria-label",
    active
      ? "Stop recording"
      : retry
        ? "Retry saving recording"
        : state?.input?.active
          ? "Record microphone"
          : "Record output",
  );
  $("record-label").textContent = recordingBusy
    ? recordingAction === "starting"
      ? "Starting…"
      : "Saving…"
    : active
      ? "Stop & save"
      : retry
        ? "Retry save"
        : state?.input?.active
          ? "Record mic"
          : "Record output";
  $("record-time").hidden = !active;
  const seconds = Math.floor(take?.seconds || 0);
  $("record-time").textContent =
    `${String(Math.floor(seconds / 60)).padStart(2, "0")}:${String(seconds % 60).padStart(2, "0")}`;
  $("record-source").disabled = active || recordingBusy;
  if (active && take.source) $("record-source").value = take.source;
  $("record").title =
    state?.input?.active && $("record-source").value === "input"
      ? "Record microphone input before filters and effects"
      : "Record the processed audio output";
  $("input-device").disabled = active || recordingBusy;
  if (state?.input) {
    const message = window.sadPetPlugin
      ? ""
      : state.input.pending
        ? "Allow microphone access in your browser…"
        : state.input.error ||
          (state.input.active
            ? recordingBusy
              ? recordingAction === "starting"
                ? "Starting recording…"
                : "Saving recording…"
              : active
                ? `Recording ${take.source === "input" ? "microphone" : "processed audio"} · ${$("record-time").textContent}`
                : "Microphone connected · not recording"
            : "");
    if ($("input-status").textContent !== message)
      $("input-status").textContent = message;
    $("input-status").classList.toggle("recording", active);
  }
}
function showSavedRecording(record) {
  lastSavedRecording = record;
  $("recording-result").hidden = false;
  $("recording-result-title").textContent =
    record.kind === "recording"
      ? "Recording saved"
      : record.appliedFades?.fade_in || record.appliedFades?.fade_out
        ? "Edit saved"
        : record.trimmedFrom
          ? "Trim saved"
          : "Sample saved";
  $("recording-result-info").textContent =
    `${record.displayName} · ${formatTime(record.seconds || 0)} · Library`;
  $("recording-mp3").hidden = !record.mp3Blob;
}
async function saveCompletedRecording(take) {
  recordingBusy = true;
  recordingAction = "saving";
  syncRecording();
  try {
    showSavedRecording(await fileLibrary.saveRecording(take));
    savedRecordingId = take.id;
    failedRecordingId = null;
    if (take.error) notify(take.error, true);
  } catch (error) {
    failedRecordingId = take.id;
    notify(`${error.message} Press Retry save to try again.`, true);
  } finally {
    recordingBusy = false;
    recordingAction = null;
    syncRecording();
  }
}
async function toggleRecording() {
  if (recordingBusy) return;
  if (state?.recording?.available && state.recording.id === failedRecordingId)
    return saveCompletedRecording(state.recording);
  recordingBusy = true;
  recordingAction = state?.recording?.active ? "saving" : "starting";
  syncRecording();
  try {
    const active = !state?.recording?.active;
    const source = state?.input?.active ? $("record-source").value : "output";
    if (active && source === "output" && !state.playing)
      await transport({ playing: true, midi_gate: false });
    const take = await request(
      "/api/recording",
      window.sadPetAudio ? { active, source } : { active },
    );
    state.recording = take;
    if (!active) await saveCompletedRecording(take);
  } catch (error) {
    notify(error.message);
  } finally {
    recordingBusy = false;
    recordingAction = null;
    syncRecording();
  }
}
const filterModels = [
  ["legacy", "SAD PET"],
  ["radias", "RADIAS"],
  ["ladder_lp24", "Ladder LP24"],
  ["ms20_lp12", "MS-20 LP12"],
  ["ms20_hp12", "MS-20 HP12"],
  ["pro_lp24", "Pro LP24"],
  ["sem", "SEM"],
  ["multi_filter", "Multi Filter"],
  ["ladder_hp24", "Ladder HP24"],
  ["ladder_bp24", "Ladder BP24"],
  ["m_poly_lp24", "M/Poly LP24"],
  ["linear_lp12", "Linear LP12"],
  ["linear_lp24", "Linear LP24"],
  ["linear_hp12", "Linear HP12"],
  ["linear_hp24", "Linear HP24"],
  ["linear_bp12", "Linear BP12"],
  ["linear_bp24", "Linear BP24"],
  ["linear_notch12", "Linear Notch12"],
  ["linear_notch24", "Linear Notch24"],
  ["radias2", "RADIAS Filter 2"],
];
const filterResponses = [
  ["low_pass", "Low pass"],
  ["band_pass", "Band pass"],
  ["high_pass", "High pass"],
  ["comb", "Comb"],
];
const filterChoices = {
  filter_model: filterModels.map(([id]) => id),
  filter2_model: filterModels.map(([id]) => id),
  vcf_type: filterResponses.map(([id]) => id),
  vcf2_type: filterResponses.map(([id]) => id),
};
function setParam(key, value) {
  preset.params[key] = value;
  syncControl(key);
  if (/^(filter2?|vcf2?_type)(_|$)/.test(key)) refreshFilterGraphs();
  if (typeof value === "number") scheduleParameter(key, value);
  else if (filterChoices[key])
    scheduleParameter(key, filterChoices[key].indexOf(value));
  else if (key === "loop_one_bar") scheduleParameter(key, Number(value));
  else schedulePreset();
}
/** Drum, Synth, Bus and Master timbres keep their sound in a windowed history. */
const historySettings = () => preset?.drums ?? preset?.synth ?? preset?.bus;
function grainSeconds(value) {
  // They share the windowed history's grain scaling.
  const history = historySettings();
  const span = history
    ? Math.min(2, Math.max(0, (history.buffer_seconds ?? 8) - 0.01))
    : 2;
  return 0.01 + span * Math.abs(value * 2 - 1) ** 3;
}
const grainSchema = [
  ["position", "POSITION", (v) => percent(v)],
  ["grains", "GRAINS", (v) => `${(128 * v ** 3).toFixed(1)} /s`],
  [
    "grain_size",
    "SIZE",
    (v) =>
      `${v < 0.5 ? "←" : "→"} ${Math.round(grainSeconds(v) * 1000)} ms`,
  ],
  [
    "pitch",
    "PITCH",
    (v) => `${((v * 2 - 1) * preset.params.pitch_range).toFixed(1)} st`,
  ],
  ["grain_shape", "SHAPE", percent],
  ["cloud", "CLOUD", percent],
  [
    "detune",
    "DETUNE",
    (v) => {
      const x = v * 2 - 1;
      const st =
        Math.abs(x) > 0.05
          ? Math.sign(x) * ((Math.abs(x) - 0.05) / 0.95) ** 3 * 12
          : 0;
      return `${st.toFixed(2)} st`;
    },
  ],
  ["stereo_spread", "SPREAD", percent],
  ["grain_feedback", "FEEDBACK", percent],
];
// Advanced grain mode (rust/src/grain_groups.rs): grains are dealt to four
// groups, each with its own pitch, level, filter and two envelopes that run
// over every grain's own life. The amp envelope is the grains' window, its
// segments curved by the group's SHAPE.
const GRAIN_GROUP_NAMES = ["A", "B", "C", "D"];
const GRAIN_GROUP_COLORS = [
  [203, 255, 148],
  [140, 220, 255],
  [255, 204, 120],
  [255, 150, 200],
];
const GRAIN_FILTER_TYPES = [
  ["low_pass", "Low pass"],
  ["band_pass", "Band pass"],
  ["high_pass", "High pass"],
];
// What a group's pitch offsets snap to, in host order.
const GRAIN_SCALES = [
  ["off", "Free"],
  ["chromatic", "Chromatic"],
  ["major", "Major"],
  ["minor", "Minor"],
  ["dorian", "Dorian"],
  ["mixolydian", "Mixolydian"],
  ["harmonic_minor", "Harmonic minor"],
  ["pentatonic_major", "Pentatonic major"],
  ["pentatonic_minor", "Pentatonic minor"],
  ["whole_tone", "Whole tone"],
  ["fifths", "Fifths"],
  ["octaves", "Octaves"],
  ["chord", "Held chord"],
];
// The group menus and their choices, by field.
const GRAIN_GROUP_CHOICES = { filter_type: GRAIN_FILTER_TYPES, scale: GRAIN_SCALES };
const grainGroupDefault = () => ({
  share: 1,
  level: 1,
  pitch: 0,
  shape: 0.9,
  filter_type: "low_pass",
  cutoff: 1,
  resonance: 0.1,
  env_depth: 0,
  // The engine's window: up over the first half of a grain, down over the second.
  amp: { attack: 0.5, decay: 0, sustain: 1, release: 0.5 },
  filter_env: { attack: 0, decay: 0.5, sustain: 0, release: 0 },
  // Kept in the timbre, sent nowhere.
  output: { chain: 1, send: 0, bus: 0 },
  // Placement and randomness move from the engine's; no scale.
  size: 0,
  position: 0,
  reverse: 0,
  spread: 0,
  random_pitch: 0,
  random_cutoff: 0,
  random_level: 0,
  scale: "off",
});
// Reading leaves the preset alone: the host omits untouched groups.
const grainGroupSettings = () =>
  preset?.grain_groups ?? { advanced: false, groups: GRAIN_GROUP_NAMES.map(grainGroupDefault) };
function ensureGrainGroups() {
  preset.grain_groups ??= structuredClone(grainGroupSettings());
  return preset.grain_groups;
}
// Host field → where it lives in a group, e.g. amp_attack → amp.attack.
const grainGroupPath = (field) =>
  /^(amp|filter)_(attack|decay|sustain|release)$/.test(field)
    ? [field.startsWith("amp") ? "amp" : "filter_env", field.replace(/^(amp|filter)_/, "")]
    : ["chain", "send", "bus"].includes(field)
      ? ["output", field]
      : [field];
function grainGroupField(group, field) {
  const [a, b] = grainGroupPath(field);
  const value = b ? group[a]?.[b] : group[a];
  return value ?? grainGroupField(grainGroupDefault(), field);
}
let grainGroupPage = 0;
const grainOutputViews = [];
function editGrainGroup(index, field, value) {
  const group = ensureGrainGroups().groups[index],
    [a, b] = grainGroupPath(field);
  if (b) group[a] = { ...grainGroupDefault()[a], ...group[a], [b]: value };
  else group[a] = value;
  const key = `grain_group${index + 1}_${field}`;
  controls.get(key)?.();
  const choices = GRAIN_GROUP_CHOICES[field];
  scheduleParameter(key, choices ? choices.findIndex(([id]) => id === value) : value);
  syncGrainGroupOutputs();
  drawGrainGroupCurve();
}
/** One grain's length in milliseconds at the engine's SIZE. */
const grainMs = (share) =>
  Math.round(share * grainSeconds(preset?.params.grain_size ?? 0.69) * 1000);
function buildGrainGroups() {
  const tabs = $("grain-group-tabs"),
    pages = $("grain-group-pages");
  const cutoff = (v) => `${Math.round(20 * 1000 ** v).toLocaleString()} Hz`,
    time = (v) => {
      const ms = grainMs(v);
      return `${Math.round(v * 100)}% ${ms < 1000 ? `${ms}ms` : `${(ms / 1000).toFixed(1)}s`}`;
    },
    pitch = (v) => `${v > 0 ? "+" : ""}${Number.isInteger(v) ? v : v.toFixed(1)} st`;
  const spread = (v) => `±${percent(v)}`,
    factor = (v) => `×${Number((2 ** v).toFixed(2))}`;
  // [title, menu field, knobs: [field, label, format, min, max, name]]
  const cards = [
    ["Grain", null, [
      ["share", "SHARE", (v) => (v > 0 ? percent(v) : "Off")],
      ["level", "LEVEL", percent],
      ["random_level", "RND LEVEL", (v) => (v > 0 ? `−${percent(v)}` : "0%"), 0, 1, "random level"],
      ["shape", "SHAPE", percent],
    ]],
    ["Pitch", "scale", [
      ["pitch", "PITCH", pitch, -24, 24],
      ["random_pitch", "RND PITCH", (v) => `±${v.toFixed(1)} st`, 0, 24, "random pitch"],
    ]],
    ["Playback", null, [
      ["size", "SIZE", factor, -3, 2],
      ["position", "POSITION", signedPercent, -1, 1],
      ["reverse", "REVERSE", percent],
      ["spread", "SPREAD", signedPercent, -1, 1],
    ]],
    ["Filter", "filter_type", [
      ["cutoff", "CUTOFF", cutoff, 0, 1, "filter cutoff"],
      ["resonance", "RESO", percent, 0, 1, "filter reso"],
      ["env_depth", "ENV DEPTH", signedPercent, -1, 1, "filter env depth"],
      ["random_cutoff", "RND CUTOFF", spread, 0, 1, "random cutoff"],
    ]],
    ["Amp envelope", null, [
      ["amp_attack", "ATTACK", time],
      ["amp_decay", "DECAY", time],
      ["amp_sustain", "SUSTAIN", percent],
      ["amp_release", "RELEASE", time],
    ]],
    ["Filter envelope", null, [
      ["filter_attack", "ATTACK", time],
      ["filter_decay", "DECAY", time],
      ["filter_sustain", "SUSTAIN", percent],
      ["filter_release", "RELEASE", time],
    ]],
  ];
  GRAIN_GROUP_NAMES.forEach((name, index) => {
    const tab = document.createElement("button");
    tab.id = `grain-group-tab-${index + 1}`;
    tab.setAttribute("role", "tab");
    tab.setAttribute("aria-controls", `grain-group-page-${index + 1}`);
    tab.style.setProperty("--group-color", `rgb(${GRAIN_GROUP_COLORS[index]})`);
    tab.innerHTML = `<i aria-hidden="true"></i>Group ${name}<small></small>`;
    tab.onclick = () => {
      grainGroupPage = index;
      syncGrainGroups();
    };
    tab.onkeydown = (e) => {
      const step = { ArrowRight: 1, ArrowLeft: -1 }[e.key];
      if (!step) return;
      e.preventDefault();
      grainGroupPage = (index + step + GRAIN_GROUP_NAMES.length) % GRAIN_GROUP_NAMES.length;
      syncGrainGroups();
      tabs.children[grainGroupPage].focus();
    };
    tabs.append(tab);
    const page = document.createElement("div");
    page.id = `grain-group-page-${index + 1}`;
    page.className = "grain-group-page";
    page.setAttribute("role", "tabpanel");
    page.setAttribute("aria-labelledby", tab.id);
    page.style.setProperty("--group-color", `rgb(${GRAIN_GROUP_COLORS[index]})`);
    for (const [title, menu, knobs] of cards) {
      const card = document.createElement("div");
      card.className = "grain-group-card";
      card.innerHTML = `<div class="grain-group-card-title"><h3>${title}</h3></div><div class="grain-group-knobs"></div>`;
      if (menu) {
        const select = document.createElement("select");
        select.className = "grain-group-menu";
        select.dataset.field = menu;
        select.setAttribute("aria-label", `Group ${name} ${menu === "scale" ? "scale" : "filter type"}`);
        for (const [id, label] of GRAIN_GROUP_CHOICES[menu]) select.append(option(id, label));
        select.onchange = () => editGrainGroup(index, menu, select.value);
        card.firstElementChild.append(select);
      }
      const envelope = /envelope/.test(title) ? `${title.toLowerCase()} ` : "";
      for (const [field, label, format, min = 0, max = 1, aria] of knobs) {
        const reset = grainGroupField(grainGroupDefault(), field);
        createKnob(
          [`grain_group${index + 1}_${field}`, label, format, min, max,
            `Group ${name} ${aria ?? `${envelope}${label.toLowerCase()}`}`],
          card.lastElementChild,
          {
            get: () => grainGroupField(grainGroupSettings().groups[index], field),
            set: (v) => editGrainGroup(index, field, v),
            reset: () => reset,
            // Whole semitones: groups stack intervals over the engine's PITCH.
            ...(field === "pitch"
              ? { quantize: (v) => Math.round(v), step: 1, ariaScale: 1 }
              : {}),
          },
        );
      }
      page.append(card);
    }
    // Where the group's grains go, as a section's Output row does.
    const row = document.createElement("div");
    row.className = "tap grain-group-output";
    row.innerHTML = `<svg class="tap-flow" viewBox="0 0 30 52" aria-hidden="true"><path class="tap-chain" d="M9 0V52"/><path class="tap-branch" d="M9 26H23"/><path class="tap-branch" d="M19 21.5l4.5 4.5-4.5 4.5"/></svg><div class="tap-name"><strong>Output</strong><span>Group ${name}</span></div><div class="tap-knobs"></div><label class="tap-bus">to<select aria-label="Group ${name} bus"></select></label><output class="tap-status"></output>`;
    const knobs = row.querySelector(".tap-knobs");
    for (const [field, label, aria] of [["chain", "CHAIN", "chain"], ["send", "SEND", "bus send"]])
      createKnob(
        [`grain_group${index + 1}_${field}`, label, percent, 0, 1, `Group ${name} ${aria}`],
        knobs,
        {
          get: () => grainGroupField(grainGroupSettings().groups[index], field),
          set: (v) => editGrainGroup(index, field, v),
          reset: () => grainGroupField(grainGroupDefault(), field),
        },
      );
    const [chain, send] = knobs.querySelectorAll(".dial");
    chain.title = "Share of this group's grains that stays in the timbre. At 0% only its bus hears them.";
    send.closest(".knob").title = "Share of this group's grains sent to the chosen bus, whatever CHAIN is";
    const select = row.querySelector("select");
    select.onchange = () => {
      if (select.value === "new")
        addTapBus(null, (bus) => editGrainGroup(index, "bus", bus));
      else editGrainGroup(index, "bus", Number(select.value));
    };
    page.append(row);
    grainOutputViews.push({ row, select, send, signature: "" });
    pages.append(page);
  });
  $("grain-advanced").onclick = () => {
    const on = !grainGroupSettings().advanced;
    if (on && !preset.grain_groups) {
      // Groups start from the engine's own SHAPE, so switching on keeps the sound.
      const settings = ensureGrainGroups();
      for (const group of settings.groups) group.shape = preset.params.grain_shape;
      settings.advanced = true;
      schedulePreset();
    } else {
      ensureGrainGroups().advanced = on;
      scheduleParameter("grain_advanced", Number(on));
    }
    syncGrainGroups();
  };
}
function syncGrainGroupTabs() {
  const { groups } = grainGroupSettings(),
    { buses } = sendTargets();
  [...$("grain-group-tabs").children].forEach((tab, index) => {
    const share = groups[index].share,
      { send, bus } = groups[index].output ?? grainGroupDefault().output,
      target = send > 0 && buses.find((b) => b.bus === bus);
    setAttr(tab, "aria-selected", String(index === grainGroupPage));
    tab.tabIndex = index === grainGroupPage ? 0 : -1;
    tab.classList.toggle("is-off", share <= 0);
    setText(
      tab.lastElementChild,
      `${share > 0 ? percent(share) : "Off"}${target ? ` → ${target.name}` : ""}`,
    );
  });
}
function syncGrainGroupOutputs() {
  if (!preset || !grainOutputViews.length) return;
  const { sends, buses } = sendTargets(),
    { groups } = grainGroupSettings();
  grainOutputViews.forEach((view, index) => {
    const { row, select, send } = view,
      out = { ...grainGroupDefault().output, ...groups[index].output },
      target = buses.find((b) => b.bus === out.bus),
      sending = !!target && out.send > 0,
      muted = out.chain === 0;
    row.classList.toggle("no-sends", !sends);
    row.classList.toggle("sending", sending);
    row.classList.toggle("skipped", muted);
    row.style.setProperty("--chain", String(out.chain));
    const signature = JSON.stringify([out.bus, buses.map((b) => [b.bus, b.name])]);
    if (signature !== view.signature) {
      view.signature = signature;
      fillBusMenu(select, out.bus, buses);
    } else if (document.activeElement !== select) select.value = String(out.bus);
    setAttr(send, "aria-disabled", !out.bus);
    send.tabIndex = out.bus ? 0 : -1;
    send.closest(".knob").classList.toggle("filter-unused", !out.bus);
    const name = target?.name ?? `Bus ${out.bus}`;
    setText(
      row.querySelector(".tap-status"),
      !sends
        ? muted ? "Muted: the group plays nowhere" : ""
        : out.bus && !target
          ? `${name} is gone: nothing is sent`
          : muted
            ? sending ? `${name} only: the timbre leaves this group out` : "Muted: the group plays nowhere"
            : target && !sending ? `Turn SEND up to feed ${name}` : "",
    );
  });
  syncGrainGroupTabs();
}
function syncGrainGroups() {
  const settings = grainGroupSettings(),
    advanced = !!settings.advanced;
  setAttr($("grain-advanced"), "aria-pressed", String(advanced));
  $("grain-groups").hidden = !advanced;
  // Each group sets its own SHAPE while advanced mode is on.
  const shape = document.querySelector('#grain-controls [data-control-key="grain_shape"]');
  if (shape) {
    shape.setAttribute("aria-disabled", String(advanced));
    shape.tabIndex = advanced ? -1 : 0;
    const knob = shape.closest(".knob");
    knob.classList.toggle("is-replaced", advanced);
    knob.title = advanced ? "Each group sets its own SHAPE in Advanced mode" : "";
  }
  [...$("grain-group-pages").children].forEach((page, index) => {
    page.hidden = index !== grainGroupPage;
    for (const select of page.querySelectorAll(".grain-group-menu"))
      select.value = grainGroupField(settings.groups[index], select.dataset.field);
  });
  syncGrainGroupOutputs();
  drawGrainGroupCurve();
}
/** The selected group over one grain: its amp envelope as an area, the
 * cutoff its filter envelope moves as a line. */
function drawGrainGroupCurve() {
  const canvas = $("grain-group-curve");
  if (!preset || canvas.closest("[hidden]")) return;
  const [c, w, h] = resizeCanvas(canvas),
    group = grainGroupSettings().groups[grainGroupPage],
    color = GRAIN_GROUP_COLORS[grainGroupPage];
  // As dsp/grain_groups.rs: `rise` bends the attack, `fall` decay and release.
  const segments = (env, rise, fall) => {
    const scale = 1 / Math.max(1, env.attack + env.decay + env.release),
      attack = env.attack * scale,
      decay = env.decay * scale,
      release = env.release * scale;
    return (t) => {
      if (t < attack) return rise(t / attack);
      if (t < attack + decay)
        return env.sustain + (1 - env.sustain) * fall(1 - (t - attack) / decay);
      if (release <= 0 || t < 1 - release) return env.sustain;
      return env.sustain * fall(Math.min(1, Math.max(0, (1 - t) / release)));
    };
  };
  // The window's curve: a ramp bent towards a half cosine by SHAPE.
  const curve = (x) => x + group.shape * (0.5 - 0.5 * Math.cos(Math.PI * x) - x);
  const amp = segments(group.amp, curve, curve),
    sweep = segments(group.filter_env, (x) => x, (x) => x * x);
  const top = 4, bottom = h - 4, points = 96;
  c.clearRect(0, 0, w, h);
  c.fillStyle = `rgba(${color},0.28)`;
  c.strokeStyle = `rgb(${color})`;
  c.lineWidth = 1.5;
  c.beginPath();
  c.moveTo(0, bottom);
  for (let i = 0; i <= points; i++) {
    const t = i / points;
    c.lineTo(t * w, bottom - amp(t) * group.level * (bottom - top));
  }
  c.lineTo(w, bottom);
  c.fill();
  c.stroke();
  c.strokeStyle = "#8cdcd1";
  c.setLineDash([3, 3]);
  c.beginPath();
  for (let i = 0; i <= points; i++) {
    const t = i / points,
      cutoff = Math.min(1, Math.max(0, group.cutoff + group.env_depth * sweep(t))),
      y = bottom - cutoff * (bottom - top);
    if (i === 0) c.moveTo(0, y);
    else c.lineTo(t * w, y);
  }
  c.stroke();
  c.setLineDash([]);
}
function createKnob(
  [key, label, format, min = 0, max = 1, aria = label],
  bank,
  binding = null,
) {
  const get = () => (binding ? binding.get() : preset.params[key]);
  // When this knob last changed here. Feedback describing the value before
  // that is stale: a native host publishes it only 30 times a second, so its
  // old "current" would hold the pointer back while the knob is turned.
  let edited = -Infinity;
  const set = (v) => {
    if (dial.getAttribute("aria-disabled") === "true") return;
    edited = Date.now();
    const result = binding ? binding.set(v) : setParam(key, v);
    update();
    return result;
  };
  const box = document.createElement("div");
  box.className = "knob";
  box.innerHTML = `<div class="dial" role="slider" tabindex="0" aria-label="${label}" aria-valuemin="0" aria-valuemax="100"><svg viewBox="0 0 77 77" aria-hidden="true"><defs><radialGradient id="knob-face"><stop offset="0" stop-color="#414c3b"/><stop offset=".8" stop-color="#2d3629"/><stop offset="1" stop-color="#20291e"/></radialGradient></defs><circle class="track" cx="38.5" cy="38.5" r="25"/><circle class="arc" cx="38.5" cy="38.5" r="25"/><path class="modulation-range"/><circle class="face" cx="38.5" cy="38.5" r="19.5"/><line class="base-pointer" x1="38.5" y1="14" x2="38.5" y2="20"/><line class="pointer" x1="38.5" y1="38.5" x2="38.5" y2="23.5"/><g class="modulation-live"><circle cx="38.5" cy="7.5" r="3.2"/></g></svg></div><div class="knob-label">${label}</div><output class="knob-value"></output><span class="knob-mod-range" aria-hidden="true"></span>`;
  const dial = box.querySelector(".dial"),
    value = box.querySelector("output"),
    pointer = box.querySelector(".pointer"),
    basePointer = box.querySelector(".base-pointer"),
    arc = box.querySelector(".arc"),
    modRange = box.querySelector(".modulation-range"),
    modLive = box.querySelector(".modulation-live"),
    modText = box.querySelector(".knob-mod-range");
  dial.dataset.controlKey = key;
  dial.setAttribute("aria-label", aria);
  const ariaScale = binding?.ariaScale ?? 100;
  dial.setAttribute("aria-valuemin", String(min * ariaScale));
  dial.setAttribute("aria-valuemax", String(max * ariaScale));
  const clamp = (v) => Math.max(min, Math.min(max, v));
  const toUnit = binding?.normalize || ((v) => (v - min) / (max - min));
  const fromUnit = binding?.denormalize || ((v) => min + v * (max - min));
  let drag = null;
  // Polls repeat the same feedback for most knobs; repainting them anyway
  // costs a style and layout pass of the whole editor. update() repaints
  // the knob directly, so it forgets what was shown.
  let shown = null;
  const modulation = (feedback) => {
    const base = get();
    const since = Date.now() - edited,
      stale = !!drag || since < 300 ||
        (since < 2000 && Math.abs((feedback?.base ?? base) - base) > Math.abs(max - min) * 1e-6);
    const active = Boolean(feedback?.modulated) && !stale;
    const signature = active
      ? [base, feedback.current, feedback.min, feedback.max, !!drag,
          document.activeElement === dial, format(base),
          ...[feedback.current, feedback.min, feedback.max].map((v) => format(clamp(v)))].join("|")
      : `${base}|${format(base)}`;
    if (signature === shown) return;
    shown = signature;
    box.classList.toggle("is-modulated", active);
    basePointer.style.transform = `rotate(${-135 + 270 * toUnit(base)}deg)`;
    pointer.style.transform = `rotate(${-135 + 270 * toUnit(active ? clamp(feedback.current) : base)}deg)`;
    if (!active) {
      value.textContent = format(base);
      modText.textContent = "";
      dial.title = format(base);
      dial.removeAttribute("aria-description");
      for (const field of [
        "modulationCurrent",
        "modulationMin",
        "modulationMax",
      ])
        delete dial.dataset[field];
      return;
    }
    const current = clamp(feedback.current),
      low = clamp(feedback.min),
      high = clamp(feedback.max);
    const unit = toUnit;
    const point = (v) => {
      const angle = ((-135 + 270 * unit(v)) * Math.PI) / 180;
      return [38.5 + 31 * Math.sin(angle), 38.5 - 31 * Math.cos(angle)];
    };
    const [x1, y1] = point(low),
      [x2, y2] = point(high);
    modRange.setAttribute(
      "d",
      `M ${x1} ${y1} A 31 31 0 ${unit(high) - unit(low) > 2 / 3 ? 1 : 0} 1 ${x2} ${y2}`,
    );
    modLive.style.transform = `rotate(${-135 + 270 * unit(current)}deg)`;
    const now = format(current),
      from = format(low),
      to = format(high);
    const compact = (text) =>
      text
        .replace(/\n.*$/s, "")
        .replace(/\s*·.*$/, "")
        .replace(/\s+(Hz|ms|st|s)\b/g, "$1");
    value.textContent =
      drag || document.activeElement === dial ? format(base) : now;
    modText.textContent =
      from === to ? compact(from) : `${compact(from)}–${compact(to)}`;
    const description = `Base: ${format(base)} · Now: ${now} · Range: ${from} – ${to}`;
    dial.title = description;
    dial.setAttribute("aria-description", description);
    dial.dataset.modulationCurrent = String(current);
    dial.dataset.modulationMin = String(low);
    dial.dataset.modulationMax = String(high);
  };
  const update = () => {
    if (!preset) return;
    shown = null;
    const v = get();
    const unit = toUnit(v);
    value.textContent = format(v);
    pointer.style.transform = `rotate(${-135 + 270 * unit}deg)`;
    arc.style.strokeDasharray = `${Math.max(0.01, 117.81 * unit)} 157.08`;
    dial.setAttribute("aria-valuenow", String(v * ariaScale));
    dial.setAttribute("aria-valuetext", format(v));
    modulation(state?.knobs?.[key]);
  };
  dial.addEventListener("focus", update);
  dial.addEventListener("blur", update);
  dial.addEventListener("pointerdown", (e) => {
    if (!preset || dial.getAttribute("aria-disabled") === "true") return;
    // The right button opens the knob's menu (MIDI learn) instead.
    if (e.button === 2) return;
    dial.focus();
    dial.setPointerCapture(e.pointerId);
    drag = { x: e.clientX, y: e.clientY, value: get() };
    dial.classList.add("dragging");
    editing++;
    e.preventDefault();
  });
  dial.addEventListener("pointermove", (e) => {
    if (!drag) return;
    const delta = (drag.y - e.clientY + (e.clientX - drag.x) * 0.45) / 180;
    const v = clamp(fromUnit(toUnit(drag.value) + delta));
    set(binding?.quantize ? binding.quantize(v, drag.value) : v);
  });
  const release = () => {
    if (drag) {
      drag = null;
      editing = Math.max(0, editing - 1);
      dial.classList.remove("dragging");
    }
  };
  dial.addEventListener("pointerup", release);
  dial.addEventListener("pointercancel", release);
  dial.addEventListener("lostpointercapture", release);
  // A macOS Control-click opens the menu after its press began a drag.
  dial.addEventListener("contextmenu", release);
  dial.addEventListener("keydown", (e) => {
    let v = get();
    const step = binding?.step ?? null;
    const change = (direction) =>
      binding?.increment ? binding.increment(v, direction) : step === null
        ? fromUnit(toUnit(v) + direction * (e.shiftKey ? 0.05 : 0.005))
        : v + direction * step;
    if (e.key === "ArrowUp" || e.key === "ArrowRight") v = change(1);
    else if (e.key === "ArrowDown" || e.key === "ArrowLeft") v = change(-1);
    else if (e.key === "Home") v = min;
    else if (e.key === "End") v = max;
    else return;
    e.preventDefault();
    set(clamp(v));
  });
  dial.addEventListener("dblclick", () =>
    set(binding ? binding.reset() : presets[0].params[key]),
  );
  dial.addEventListener(
    "wheel",
    (e) => {
      if (document.activeElement !== dial) return;
      e.preventDefault();
      const step = binding?.step;
      const direction = -Math.sign(e.deltaY);
      set(
        clamp(
          binding?.increment ? binding.increment(get(), direction) : step === undefined
            ? fromUnit(toUnit(get()) + direction * 0.005)
            : get() + direction * step,
        ),
      );
    },
    { passive: false },
  );
  // A learned MIDI controller turns the knob across its whole range.
  globalThis.SadMidiLearn?.attach(dial, key, aria, (unit) => {
    if (!preset) return;
    let v = clamp(fromUnit(unit));
    if (binding?.quantize) v = binding.quantize(v, get());
    else if (binding?.step) v = min + Math.round((v - min) / binding.step) * binding.step;
    v = clamp(v);
    if (v !== get()) set(v);
  });
  if (binding?.register !== false) {
    controls.set(key, update);
    modulationViews.set(key, modulation);
  }
  knobUnits.set(key, { format, min, max, get });
  (typeof bank === "string" ? $(bank) : bank).append(box);
  return update;
}
function syncControl(key) {
  controls.get(key)?.();
  // Group envelope times read in milliseconds of the current grain SIZE.
  if (key === "grain_size")
    for (const [k, update] of controls)
      if (/^grain_group\d_(amp|filter)_(attack|decay|release)$/.test(k)) update();
  if (key.startsWith("filter") || key.startsWith("vcf")) syncFilters();
  if (key === "mix" || key === "master_volume") syncSliders();
  if (key === "bpm") syncColors();
  if (["loop_one_bar", "sample_root_note", "bpm"].includes(key))
    syncLoopControls();
  $("reverse").setAttribute(
    "aria-pressed",
    String(preset.params.grain_size < 0.5),
  );
}
function syncSliders() {
  $("mix").value = preset.params.mix;
  $("volume").value = preset.params.master_volume;
  $("mix-value").textContent = percent(preset.params.mix);
  $("volume-value").textContent = percent(preset.params.master_volume);
}
function buildFilters() {
  for (let i = 0; i < 2; i++) {
    const number = i + 1,
      prefix = i === 0 ? "filter" : "filter2";
    const card = document.createElement("div");
    card.className = "filter-card";
    card.id = `${prefix}-card`;
    const responses = (model) =>
      filterResponses
        .map(
          ([id, label]) =>
            `<option value="${model}:${id}">${model === "radias2" ? "RADIAS 2 · " : ""}${label}</option>`,
        )
        .join("");
    const models = filterModels
      .slice(1, -1)
      .map(([id, label]) => `<option value="${id}">${label}</option>`)
      .join("");
    card.innerHTML = `<div class="filter-title"><strong>Filter ${number}</strong><select id="${prefix}-type" aria-label="Filter ${number} model"><optgroup label="SAD PET">${responses("legacy")}</optgroup><optgroup label="Rustias">${models}</optgroup><optgroup label="RADIAS Filter 2">${responses("radias2")}</optgroup></select><select id="${prefix}-env" aria-label="Filter ${number} envelope"><option value="env1">ENV 1</option><option value="env2">ENV 2</option><option value="env3">ENV 3</option></select></div>${filterGraphMarkup(number)}<div id="${prefix}-controls" class="knob-grid filter-controls"></div>`;
    $("filters").append(card);
    const type = i === 0 ? "vcf_type" : "vcf2_type";
    card.querySelector(".filter-graph").spec = () => {
      const p = preset?.params;
      return p && filterGraphSpec(p[`${prefix}_model`] ?? "legacy", p[type], p[prefix],
        p[`${prefix}_env_depth`], p[`${prefix}_resonance`], p[`${prefix}_morph`]);
    };
    $(`${prefix}-type`).onchange = (e) => {
      const [model, response] = e.target.value.split(":");
      setParam(`${prefix}_model`, model);
      if (response) setParam(type, response);
    };
    $(`${prefix}-env`).onchange = (e) =>
      setParam(`${prefix}_env`, e.target.value);
    const cutoff = (v) => {
      const comb =
        preset.params[`${prefix}_model`] !== "legacy" ||
        preset.params[type] === "comb";
      return `${Math.round((comb ? 30 : 20) * (comb ? 200 : 1000) ** v).toLocaleString()} Hz`;
    };
    for (const schema of [
      [prefix, "CUTOFF", cutoff],
      [`${prefix}_resonance`, "RESONANCE", percent],
      [`${prefix}_env_depth`, "ENV DEPTH", signedPercent, -1, 1],
      [`${prefix}_keytrack`, "KEYTRACK", percent],
      [`${prefix}_drive`, "DRIVE", percent],
      [`${prefix}_morph`, "MORPH", percent],
    ]) {
      const [key, label, format, min = 0, max = 1] = schema;
      createKnob(
        [
          key,
          label,
          format,
          min,
          max,
          `Filter ${number} ${label.toLowerCase()}`,
        ],
        `${prefix}-controls`,
      );
    }
  }
  $("filter-routing").onchange = (e) =>
    setParam("filter_routing", e.target.value);
  $("add-filter").onclick = addFilter;
}
// Filter cards plot the engine's own response: it rings an impulse through
// the real filter, one request per card at a time.
const filterGraphMarkup = (number) =>
  `<div class="filter-graph" role="img" aria-label="Filter ${number} response">${window.SadFilterGraph?.markup ?? ""}</div>`;
// The dashed curve is the cutoff at the envelope's peak.
const filterGraphSpec = (model, kind, cutoff, depth, resonance, morph) => ({
  model, kind, resonance, morph,
  cutoffs: depth ? [cutoff, Math.min(1, Math.max(0, cutoff + depth))] : [cutoff],
});
let filterGraphFrame = 0;
function refreshFilterGraphs() {
  if (filterGraphFrame) return;
  filterGraphFrame = requestAnimationFrame(() => {
    filterGraphFrame = 0;
    document.querySelectorAll(".filter-card .filter-graph").forEach(drawFilterGraph);
  });
}
async function drawFilterGraph(graph) {
  const spec = graph.spec?.(),
    shape = JSON.stringify(spec);
  if (!spec || graph.busy || graph.dataset.shape === shape) return;
  graph.busy = true;
  try {
    window.SadFilterGraph.draw(graph, (await request("/api/filter/response", spec)).curves);
    graph.dataset.shape = shape;
  } catch {
    // An engine without the route keeps the card as it was.
    return;
  } finally {
    graph.busy = false;
  }
  if (graph.isConnected && JSON.stringify(graph.spec()) !== shape) drawFilterGraph(graph);
}
function syncFilters() {
  if (!preset) return;
  const p = preset.params;
  $("filter-routing").value = p.filter_routing;
  $("filter-path").textContent = {
    single: "IN → F1 → OUT",
    serial: "IN → F1 → F2 → OUT",
    parallel: "IN → F1 ∥ F2 → OUT",
  }[p.filter_routing];
  $("filter2-card").classList.toggle("inactive", p.filter_routing === "single");
  for (const [prefix, type] of [
    ["filter", "vcf_type"],
    ["filter2", "vcf2_type"],
  ]) {
    const model = p[`${prefix}_model`] ?? "legacy";
    $(`${prefix}-type`).value = ["legacy", "radias2"].includes(model)
      ? `${model}:${p[type]}`
      : model;
    $(`${prefix}-env`).value = p[`${prefix}_env`];
    for (const [field, active] of [
      ["drive", model !== "legacy"],
      ["morph", ["radias", "sem", "multi_filter"].includes(model)],
    ]) {
      const dial = document.querySelector(
        `[data-control-key="${prefix}_${field}"]`,
      );
      if (dial) {
        dial.closest(".knob").classList.toggle("filter-unused", !active);
        dial.setAttribute("aria-disabled", String(!active));
        dial.tabIndex = active ? 0 : -1;
      }
    }
    const morph = document.querySelector(
      `[data-control-key="${prefix}_morph"]`,
    );
    if (morph)
      morph.closest(".knob").querySelector(".knob-label").title =
        {
          radias: "LP24 → LP12 → BP → HP → Thru",
          sem: "LP → Notch → HP",
          multi_filter: "LP → BP → HP → Notch",
        }[model] ?? "";
    controls.get(prefix)?.();
  }
  syncAddedFilters();
  refreshFilterGraphs();
}
// Filters 3 and up. Positions inside a note's path run one filter per voice
// with its own envelope and key tracking; the others are one shared filter
// that follows the loudest voice's envelope and the last note.
const FILTER_POSITIONS = [
  ["after_grains", "After grains"],
  ["after_osc1", "After OSC 1"],
  ["after_osc2", "After OSC 2"],
  ["after_osc3", "After OSC 3"],
  ["after_osc4", "After OSC 4"],
  ["after_resynth", "After Resynth"],
  ["before_filters", "Before Filter 1·2"],
  ["after_filters", "After Filter 1·2"],
  ["after_fx", "After FX"],
];
const MAX_FILTER_SLOTS = 16;
const FILTER_SLOT_DEFAULT = {
  id: 0,
  enabled: true,
  position: "after_filters",
  model: "legacy",
  kind: "low_pass",
  cutoff: 0.7,
  resonance: 0.12,
  drive: 0,
  morph: 0,
  env: "env2",
  env_depth: 0,
  keytrack: 0,
};
const FILTER_SLOT_CONTROLS = [
  ["cutoff", "CUTOFF"],
  ["resonance", "RESONANCE"],
  ["env_depth", "ENV DEPTH", -1, 1],
  ["keytrack", "KEYTRACK"],
  ["drive", "DRIVE"],
  ["morph", "MORPH"],
];
let addedFilterSignature = "";
const findFilterSlot = (id) => preset.filter_slots?.find((s) => s.id === id);
function filterModelOptions() {
  const responses = (model) =>
    filterResponses
      .map(
        ([id, label]) =>
          `<option value="${model}:${id}">${model === "radias2" ? "RADIAS 2 · " : ""}${label}</option>`,
      )
      .join("");
  const models = filterModels
    .slice(1, -1)
    .map(([id, label]) => `<option value="${id}">${label}</option>`)
    .join("");
  return `<optgroup label="SAD PET">${responses("legacy")}</optgroup><optgroup label="Rustias">${models}</optgroup><optgroup label="RADIAS Filter 2">${responses("radias2")}</optgroup>`;
}
function editAddedFilters() {
  syncAddedFilters();
  refreshColorDestinations();
  schedulePreset();
}
function addFilter() {
  preset.filter_slots ||= [];
  if (preset.filter_slots.length >= MAX_FILTER_SLOTS) return;
  let id = 1;
  while (preset.filter_slots.some((s) => s.id === id)) id++;
  preset.filter_slots.push({ ...FILTER_SLOT_DEFAULT, id });
  editAddedFilters();
}
// Unroute every LFO, ENV, matrix and macro route whose destination matches.
function unroute(matches) {
  for (const holder of [...preset.lfos, ...preset.envs])
    for (const field of ["destination", "destination2", "destination3", "extra_destination"])
      if (matches(holder[field])) holder[field] = "none";
  for (const route of preset.matrix)
    if (matches(route.destination)) route.destination = "none";
  for (const knob of preset.macros || [])
    knob.targets = knob.targets.filter((t) => !matches(t.destination));
}
function removeFilter(id) {
  preset.filter_slots = preset.filter_slots.filter((s) => s.id !== id);
  unroute((d) => d?.filter_slot?.slot === id);
  syncLfos();
  syncEnvs();
  syncMatrix();
  editAddedFilters();
}
function moveFilter(id, delta) {
  const slots = preset.filter_slots;
  const index = slots.findIndex((s) => s.id === id),
    target = index + delta;
  if (index < 0 || target < 0 || target >= slots.length) return;
  [slots[index], slots[target]] = [slots[target], slots[index]];
  editAddedFilters();
}
function syncAddedFilters() {
  if (!preset) return;
  const slots = (preset.filter_slots ||= []);
  $("add-filter").disabled = slots.length >= MAX_FILTER_SLOTS;
  const signature = slots
    .map((s) => `${s.id}:${s.position}:${s.model}:${s.kind}:${s.enabled}`)
    .join(",");
  if (
    signature !== addedFilterSignature ||
    $("added-filters").children.length !== slots.length
  ) {
    addedFilterSignature = signature;
    for (const key of controls.keys())
      if (key.startsWith("filter_slot:")) {
        controls.delete(key);
        modulationViews.delete(key);
      }
    $("added-filters").replaceChildren();
    for (const [index, slot] of slots.entries())
      $("added-filters").append(addedFilterCard(index, slot));
  }
  for (const slot of slots) {
    const card = $("added-filters").querySelector(`[data-id="${slot.id}"]`);
    if (!card) continue;
    card.querySelector(".added-filter-env").value = slot.env;
    for (const [field] of FILTER_SLOT_CONTROLS)
      controls.get(`filter_slot:${slot.id}:${field}`)?.();
  }
  refreshFilterGraphs();
}
function addedFilterCard(index, slot) {
  const number = index + 3,
    id = slot.id;
  const card = document.createElement("div");
  card.className = "filter-card added-filter-card";
  card.classList.toggle("inactive", !slot.enabled);
  card.dataset.id = id;
  card.innerHTML = `<div class="filter-title added-filter-title"><strong>Filter ${number}</strong><select class="added-filter-position" aria-label="Filter ${number} position"></select><select class="added-filter-type" aria-label="Filter ${number} model">${filterModelOptions()}</select><select class="added-filter-env" aria-label="Filter ${number} envelope"><option value="env1">ENV 1</option><option value="env2">ENV 2</option><option value="env3">ENV 3</option></select><div class="color-actions"><button class="color-bypass" aria-label="Bypass Filter ${number}" title="Bypass" aria-pressed="${!slot.enabled}">⏻</button><button class="color-up" aria-label="Move Filter ${number} up" title="Move up">↑</button><button class="color-down" aria-label="Move Filter ${number} down" title="Move down">↓</button><button class="color-remove" aria-label="Remove Filter ${number}" title="Remove">×</button></div></div>${filterGraphMarkup(number)}<div id="filter-slot-${id}-controls" class="knob-grid filter-controls"></div>`;
  card.querySelector(".filter-graph").spec = () => {
    const s = findFilterSlot(id);
    return s && filterGraphSpec(s.model, s.kind, s.cutoff, s.env_depth, s.resonance, s.morph);
  };
  const position = card.querySelector(".added-filter-position");
  position.replaceChildren(
    ...FILTER_POSITIONS.map(([value, label]) => option(value, label)),
  );
  position.value = slot.position;
  position.onchange = () => {
    findFilterSlot(id).position = position.value;
    editAddedFilters();
  };
  const type = card.querySelector(".added-filter-type");
  type.value = ["legacy", "radias2"].includes(slot.model)
    ? `${slot.model}:${slot.kind}`
    : slot.model;
  type.onchange = () => {
    const [model, response] = type.value.split(":");
    const s = findFilterSlot(id);
    s.model = model;
    if (response) s.kind = response;
    editAddedFilters();
  };
  const env = card.querySelector(".added-filter-env");
  env.onchange = () => {
    findFilterSlot(id).env = env.value;
    schedulePreset();
  };
  card.querySelector(".color-bypass").onclick = () => {
    const s = findFilterSlot(id);
    s.enabled = !s.enabled;
    editAddedFilters();
  };
  card.querySelector(".color-up").onclick = () => moveFilter(id, -1);
  card.querySelector(".color-down").onclick = () => moveFilter(id, 1);
  card.querySelector(".color-remove").onclick = () => removeFilter(id);
  const container = card.querySelector(".filter-controls");
  for (const [field, label, min = 0, max = 1] of FILTER_SLOT_CONTROLS) {
    const key = `filter_slot:${id}:${field}`;
    const current = () => findFilterSlot(id) || FILTER_SLOT_DEFAULT;
    const format =
      field === "cutoff"
        ? (v) => {
            const s = current();
            const comb = s.model !== "legacy" || s.kind === "comb";
            return `${Math.round((comb ? 30 : 20) * (comb ? 200 : 1000) ** v).toLocaleString()} Hz`;
          }
        : field === "env_depth"
          ? signedPercent
          : percent;
    createKnob(
      [key, label, format, min, max, `Filter ${number} ${label.toLowerCase()}`],
      container,
      {
        get: () => current()[field],
        set: (value) => {
          const s = findFilterSlot(id);
          if (!s) return;
          s[field] = value;
          controls.get(key)?.();
          scheduleParameter(`filter${number}_${field}`, value);
          refreshFilterGraphs();
        },
        reset: () => FILTER_SLOT_DEFAULT[field],
      },
    );
    const active =
      field === "drive"
        ? slot.model !== "legacy"
        : field === "morph"
          ? ["radias", "sem", "multi_filter"].includes(slot.model)
          : true;
    const dial = container.querySelector(`[data-control-key="${key}"]`);
    dial.closest(".knob").classList.toggle("filter-unused", !active);
    dial.setAttribute("aria-disabled", String(!active));
    dial.tabIndex = active ? 0 : -1;
  }
  return card;
}
function scheduleDrum(edit) {
  editGeneration++;
  pending++;
  presetLibrary?.markDirty();
  drumEdits.set(`${edit.track}:${edit.step}:${edit.field}`, edit);
  if (commitTimer === null && !presetWriteCount) armPresetWrite(16);
}
let timbreWrites = Promise.resolve();
function changeTimbre(path, data, scalar = false) {
  timbreWrites = timbreWrites
    .catch(() => {})
    .then(async () => {
      timbreBusy = true;
      try {
        await flushEdits();
        await sampleSelection?.flush();
        await noteChain;
        const context = await request(path, data);
        if (scalar) {
          state.timbres = context;
          timbreEditor?.sync(context);
          busMixer?.sync(context);
          return;
        }
        const old = state?.timbres?.active;
        const next = await request("/api/state");
        state = next;
        if (!scalar || old !== context.active) {
          preset = structuredClone(next.preset);
          if (neuroEditor) {
            neuroEditor.signature = "";
            neuroEditor.selected = preset.sequencer.tracks[0]?.id ?? 0;
          }
          drumEditor && (drumEditor.signature = "");
          syncAll();
          sampleSelection?.sync(next);
          fileLibrary?.observe(next);
        } else {
          timbreEditor?.sync(context);
          busMixer?.sync(context);
        }
      } catch (e) {
        notify(e.message);
      } finally {
        timbreBusy = false;
      }
    });
  return timbreWrites;
}
function syncTimbreRouting() {
  const bus = preset?.bus,
    drums = !!preset?.drums,
    synth = !drums && !!preset?.synth,
    history = drums || synth || !!bus;
  $("load-wav").hidden = history;
  const samplePanel = document.querySelector(".sample-panel");
  samplePanel.classList.toggle("drum-waveform", history);
  // The transport stays on top of every timbre; below it each engine keeps the
  // Granular order: its own section, then the waveform, Source FX and Grain engine.
  if ($("sample-panel-home").nextElementSibling !== samplePanel)
    $("sample-panel-home").after(samplePanel);
  samplePanel.setAttribute("aria-label", drums ? "Drum waveform and grains"
    : synth ? "Synth waveform and grains" : "Sample and grains");
  $("drum-source-name").hidden = !history;
  $("drum-source-name").textContent = preset?.vocal ? "Vocal"
    : preset?.sampler ? "Kit"
    : bus ? bus.master ? "Master input" : "Bus input"
    : synth ? "Synth engine" : "Drum machine";
  $("wave").setAttribute("aria-label", drums
    ? "Live drum waveform and grains"
    : synth ? "Live synth waveform and grains"
    : "Sample waveform. Drag to select a loop, click to change position");
  $("wave").title = history
    ? `Live ${drums ? "drum" : bus ? "bus" : "synth"} signal · last ${(preset.drums ?? preset.synth ?? bus).buffer_seconds ?? 8} seconds`
    : "Drag to select a loop · drag the handles to adjust it";
  // A bus without Granular has no history to show or select from.
  samplePanel.hidden = !!bus && !bus.granular;
  // Grain engine stays in place with Granular off: its power button lives in
  // the heading, and syncInput dims the knobs.
  const grains = $("grain-engine");
  // Filter 1 and 2 of a bus stay visible but are bypassed until switched on.
  $("filters").classList.toggle("bypassed", !!bus && !bus.filters);
  $("filters").title = bus && !bus.filters
    ? "Bypassed: switch Filters on with the power button in their heading" : "";
  if ($("grain-engine-home").nextElementSibling !== grains)
    $("grain-engine-home").after(grains);
  $("drum-grain-count").hidden = !history;
  syncGrainBuffer();
  syncControl("grain_size");
  // One Granular switch in the Grain engine heading for every engine with a
  // live history.
  const own = historySettings();
  $("history-granular").hidden = !own;
  setAttr($("history-granular"), "aria-pressed", !!own?.granular);
  $("mix").disabled = !!own && !own.granular;
  $("mix").title = own && !own.granular
    ? "Switch on Granular to blend in grains"
    : "Blend source audio and grains before filters and effects";
  const performance = $("performance-mode").closest("section");
  // A kit plays notes like a synth: keys, arpeggiator and hold apply.
  if (performance) performance.hidden = drums || (!!bus && !preset.sampler);
  syncBus();
  syncEngineType();
  timbreEditor?.sync(state?.timbres);
  busMixer?.sync(state?.timbres);
  drumEditor?.sync();
  vocalEditor?.sync();
  samplerEditor?.sync();
  syncSynth();
}
function syncAll() {
  syncTimbreRouting();
  syncColors();
  syncResynth();
  syncTaps();
  for (const update of controls.values()) update();
  syncGrainGroups();
  syncSliders();
  $("antialias").checked = preset.params.antialiasing;
  $("bpm").value = preset.params.bpm;
  syncLoopControls();
  syncFilters();
  $("reverse").setAttribute(
    "aria-pressed",
    String(preset.params.grain_size < 0.5),
  );
  syncLfos();
  syncEnvs();
  syncMatrix();
  window.SadModulation?.sync();
  syncPerformance();
  neuroEditor?.sync();
}
function syncLoopControls() {
  $("loop-one-bar").checked = !!preset.params.loop_one_bar;
  $("sample-root-note").value = String(preset.params.sample_root_note ?? 60);
  sampleSelection?.render();
}
function option(value, label) {
  const o = document.createElement("option");
  o.value = value;
  o.textContent = label;
  return o;
}

const RESYNTH_WAVES = [
  ["saw", "Saw"],
  ["pulse", "Pulse"],
  ["triangle", "Triangle"],
  ["sine", "Sine"],
  ["formant", "Formant"],
  ["noise", "Noise"],
  ["audio_in", "Audio In"],
  ["pcm", "PCM"],
];
const RESYNTH_MODES = [
  ["wave", "Wave"],
  ["cross", "Cross"],
  ["unison", "Unison"],
  ["vpm", "VPM"],
];
const RESYNTH_MODELS = [
  ["radias", "RADIAS"],
  ["wavefolder", "Wavefolder · OSC 4"],
];
const RESYNTH_FOLDS = [
  ["triangle", "Triangle fold"],
  ["sine", "Sine fold"],
  ["soft", "Soft fold"],
  ["asymmetric", "Asymmetric fold"],
  ["rectify", "Rectify"],
];
const RESYNTH_INTERACTIONS = [
  ["off", "Off"],
  ["ring", "Ring"],
  ["sync", "Sync"],
  ["ring_sync", "Ring + Sync"],
];
const RESYNTH_CROSS_INPUTS = ["analysis", "source", "grains"];
const resynthOscDefault = () => ({
  enabled: true,
  model: "radias",
  waveform: "sine",
  mode: "wave",
  fold_shape: "triangle",
  interaction: "off",
  interaction_source: 0,
  control1: 0,
  control2: 0,
  transpose: 0,
  fine: 0,
  level: 0.7,
  pan: 0.5,
  cross_source: 0,
  cross_input: "analysis",
});
function resynthSettings() {
  const r = (preset.resynth ||= {
    enabled: false,
    input: "source",
    mix: 0.8,
    sensitivity: 0.5,
    attack: 0.15,
    release: 0.3,
    threshold: 0.2,
    count: 1,
    oscillators: Array.from({ length: 4 }, resynthOscDefault),
  });
  for (let i = 0; i < 4; i++)
    if (r.oscillators[i].model === undefined)
      r.oscillators[i] = { ...resynthOscDefault(), ...r.oscillators[i] };
  return r;
}
const synthOscDefault = (index = 0) => ({
  ...resynthOscDefault(),
  waveform: index === 0 ? "saw" : "sine",
  interaction_source: index === 0 ? 3 : 2,
});
function synthSettings() {
  const s = preset.synth;
  for (let i = 0; i < 4; i++)
    s.oscillators[i] = { ...synthOscDefault(i), ...s.oscillators[i] };
  return s;
}
// Resynth carriers and Synth engine voices share one oscillator card editor.
// A Synth oscillator hears no analysis or external input: its Ring/Sync and
// Cross references are other oscillators (2.. and 1.. respectively).
const OSC_BANKS = {
  resynth: {
    label: "Resynth",
    settings: () => resynthSettings(),
    defaults: () => resynthOscDefault(),
    waves: RESYNTH_WAVES,
    references: [
      [0, "Source"],
      [1, "Grains"],
    ],
    detached: 0,
    cross: [
      ["analysis", "Analysis input"],
      ["source", "Source"],
      ["grains", "Grains"],
    ],
    sync: () => syncResynth(),
    signature: "",
  },
  synth: {
    label: "Synth",
    settings: () => synthSettings(),
    defaults: (index) => synthOscDefault(index),
    waves: RESYNTH_WAVES.filter(([key]) => key !== "audio_in"),
    references: [],
    detached: 2,
    cross: [["analysis", "Off"]],
    sync: () => syncSynth(),
    signature: "",
  },
};
const OSC_CHOICES = {
  waveform: RESYNTH_WAVES,
  mode: RESYNTH_MODES,
  model: RESYNTH_MODELS,
  fold_shape: RESYNTH_FOLDS,
  interaction: RESYNTH_INTERACTIONS,
};
function editOscillator(bank, field, value, index = null) {
  const r = OSC_BANKS[bank].settings();
  const target = index === null ? r : r.oscillators[index];
  target[field] = value;
  // Synth oscillators always take their Cross input from another oscillator.
  if (bank === "synth" && field === "cross_input") return;
  let numeric = value;
  if (OSC_CHOICES[field])
    numeric = OSC_CHOICES[field].findIndex(([key]) => key === value);
  if (field === "cross_input") numeric = RESYNTH_CROSS_INPUTS.indexOf(value);
  if (field === "input") numeric = value === "grains" ? 1 : 0;
  if (typeof value === "boolean") numeric = Number(value);
  const key =
    index === null ? `${bank}_${field}` : `${bank}_osc${index + 1}_${field}`;
  controls.get(key)?.();
  scheduleParameter(key, numeric);
}
const editResynth = (field, value, index = null) =>
  editOscillator("resynth", field, value, index);
const editSynth = (field, value, index = null) =>
  editOscillator("synth", field, value, index);
function oscillatorKnob(
  bank,
  index,
  field,
  label,
  format = percent,
  min = 0,
  max = 1,
  role = "",
) {
  const { label: name, settings, defaults } = OSC_BANKS[bank];
  const key =
    index === null ? `${bank}_${field}` : `${bank}_osc${index + 1}_${field}`;
  const get = () =>
    index === null ? settings()[field] : settings().oscillators[index][field];
  const values =
    index === null
      ? {
          mix: 0.8,
          sensitivity: 0.5,
          attack: 0.15,
          release: 0.3,
          threshold: 0.2,
        }
      : defaults(index);
  createKnob(
    [
      key,
      label,
      format,
      min,
      max,
      index === null
        ? `${name} ${label.toLowerCase()}`
        : `${name} OSC ${index + 1} ${label.toLowerCase()}${role ? " " + role.toLowerCase() : ""}`,
    ],
    index === null ? `${bank}-controls` : `${bank}-osc${index + 1}-knobs`,
    {
      get,
      set: (value) => editOscillator(bank, field, value, index),
      reset: () => values[field],
    },
  );
  if (role) {
    const hint = document.createElement("span");
    hint.className = "resynth-control-role";
    hint.textContent = role;
    $(`${bank}-osc${index + 1}-knobs`)
      .lastElementChild.querySelector(".knob-label")
      .append(hint);
  }
}
const resynthKnob = (...args) => oscillatorKnob("resynth", ...args);
function buildResynth() {
  resynthKnob(
    null,
    "sensitivity",
    "SENSITIVITY",
    (v) => `${v * 48 - 24 >= 0 ? "+" : ""}${(v * 48 - 24).toFixed(1)} dB`,
  );
  resynthKnob(
    null,
    "attack",
    "ATTACK",
    (v) => `${Math.round(1 + v ** 3 * 500)} ms`,
  );
  resynthKnob(
    null,
    "release",
    "RELEASE",
    (v) => `${Math.round(5 + v ** 3 * 2000)} ms`,
  );
  resynthKnob(
    null,
    "threshold",
    "THRESHOLD",
    (v) => `${Math.round(-84 + v * 60)} dB`,
  );
  resynthKnob(null, "mix", "MIX");
  for (let i = 0; i < 16; i++) {
    const bar = document.createElement("div");
    bar.className = "resynth-band";
    bar.innerHTML = "<i></i>";
    $("resynth-spectrum").append(bar);
  }
  $("resynth-enabled").onclick = () => {
    editResynth("enabled", !resynthSettings().enabled);
    syncResynth();
  };
  $("resynth-input").onchange = () =>
    editResynth("input", $("resynth-input").value);
  $("resynth-add").onclick = () => {
    const r = resynthSettings();
    if (r.count >= 4) return;
    editResynth("count", r.count + 1);
    syncResynth();
  };
}
function syncResynth() {
  const r = resynthSettings();
  setAttr($("resynth-enabled"), "aria-pressed", r.enabled);
  $("resynth-input").value = r.input;
  $("resynth-add").disabled = r.count >= 4;
  $("resynth-spectrum").classList.toggle("inactive", !r.enabled);
  syncOscillators("resynth");
}
// Every section hands its output on down the chain (CHAIN) and to one Bus
// (SEND). At CHAIN 0% the chain skips the section and its input goes on,
// while the bus still hears the section. Host order: source → post FX.
const TAP_STAGES = [
  ["source", "Source FX"],
  ["grains", "Grain engine"],
  ["resynth", "Resynth"],
  ["pre_fx", "Pre-filter FX"],
  ["filters", "Filters"],
  ["post_fx", "Post-filter FX"],
];
const tapViews = new Map();
// Resynth keeps the send IDs it was released with.
const tapId = (stage, field) =>
  stage === "resynth" && field !== "chain"
    ? `resynth_${field}`
    : `tap_${stage}_${field}`;
// Reading leaves the preset alone: the host omits taps at their defaults.
const tapSettings = (stage) => ({ chain: 1, send: 0, bus: 0, ...preset.taps?.[stage] });
function editTap(stage, field, value) {
  preset.taps = { ...preset.taps, [stage]: { ...tapSettings(stage), [field]: value } };
  scheduleParameter(tapId(stage, field), value);
  syncTaps();
}
function buildTaps() {
  for (const [stage, label] of TAP_STAGES) {
    const section = document.querySelector(`[data-tap="${stage}"]`);
    const row = document.createElement("div");
    row.className = "tap";
    row.innerHTML = `<svg class="tap-flow" viewBox="0 0 30 52" aria-hidden="true"><path class="tap-chain" d="M9 0V52"/><path class="tap-branch" d="M9 26H23"/><path class="tap-branch" d="M19 21.5l4.5 4.5-4.5 4.5"/></svg><div class="tap-name"><strong>Output</strong><span>${label}</span></div><div class="tap-knobs"></div><label class="tap-bus">to<select aria-label="${label} bus"></select></label><output class="tap-status"></output>`;
    section.append(row);
    const knobs = row.querySelector(".tap-knobs");
    createKnob(
      [tapId(stage, "chain"), "CHAIN", percent, 0, 1, `${label} chain`],
      knobs,
      {
        get: () => tapSettings(stage).chain,
        set: (v) => editTap(stage, "chain", v),
        reset: () => 1,
      },
    );
    createKnob(
      [tapId(stage, "send"), "SEND", percent, 0, 1, `${label} bus send`],
      knobs,
      {
        get: () => tapSettings(stage).send,
        set: (v) => editTap(stage, "send", v),
        reset: () => 0,
      },
    );
    const [chain, send] = knobs.querySelectorAll(".dial");
    chain.title = "Share of this output that goes on down the chain. At 0% the chain skips the section: its input goes on, and the bus still hears the section.";
    send.closest(".knob").title = "Share of this output sent to the chosen bus, whatever CHAIN is";
    const select = row.querySelector("select");
    select.onchange = () => {
      if (select.value === "new") addTapBus(stage);
      else editTap(stage, "bus", Number(select.value));
    };
    const chips = document.createElement("span");
    chips.className = "tap-chips";
    section.querySelector(".section-heading h2").append(chips);
    tapViews.set(stage, { section, row, select, send, chips, label, signature: "" });
  }
}
/** Adds the next Bus timbre for this section, or for what `choose` routes
 * there, and stays on the timbre. */
function addTapBus(stage, choose = (bus) => editTap(stage, "bus", bus)) {
  const context = state?.timbres;
  if (!context) return;
  const used = new Set(context.timbres.filter((t) => t.kind === "bus").map((t) => t.bus));
  const bus = [1, 2, 3, 4, 5, 6, 7, 8].find((b) => !used.has(b));
  if (!bus) return;
  choose(bus);
  const home = context.active;
  changeTimbre("/api/timbres/add", { kind: "bus" });
  changeTimbre("/api/timbres/select", { id: home });
}
/** Whether this timbre may send, and the buses it can send to. */
function sendTargets() {
  const context = state?.timbres,
    part = context?.timbres?.find((t) => t.id === context.active),
    // Buses, Master and Pad FX take no sends, so they do not send either.
    sends = !!part && !["bus", "master", "pad_fx"].includes(part.kind),
    buses = sends
      ? context.timbres
          .filter((t) => t.kind === "bus" && t.bus > 0)
          .sort((a, b) => a.bus - b.bus)
      : [];
  return { sends, buses };
}
/** A bus menu: none, each bus, a removed one still chosen, then + New bus. */
function fillBusMenu(select, chosen, buses) {
  select.replaceChildren(
    option(0, buses.length ? "No bus" : "No bus yet"),
    ...buses.map((b) => option(b.bus, b.name)),
  );
  // A removed bus stays chosen until another is, and receives nothing.
  if (chosen && !buses.some((b) => b.bus === chosen))
    select.append(option(chosen, `Bus ${chosen} (removed)`));
  if (buses.length < 8) select.append(option("new", "+ New bus"));
  select.value = String(chosen);
}
function syncTaps() {
  if (!preset || !tapViews.size) return;
  const { sends, buses } = sendTargets();
  for (const [stage, view] of tapViews) {
    const { section, row, select, send, chips, label } = view;
    const tap = tapSettings(stage),
      target = buses.find((b) => b.bus === tap.bus),
      sending = !!target && tap.send > 0,
      skipped = tap.chain === 0;
    // A Synth's grains read its filtered voices.
    setText(
      row.querySelector(".tap-name span"),
      stage === "grains" && preset.synth ? `${label}, after Filters` : label,
    );
    row.classList.toggle("no-sends", !sends);
    row.classList.toggle("sending", sending);
    row.classList.toggle("skipped", skipped);
    row.style.setProperty("--chain", String(tap.chain));
    section.classList.toggle("tap-skipped", skipped);
    const signature = JSON.stringify([tap.bus, buses.map((b) => [b.bus, b.name])]);
    if (signature !== view.signature) {
      view.signature = signature;
      fillBusMenu(select, tap.bus, buses);
    } else if (document.activeElement !== select) select.value = String(tap.bus);
    setAttr(send, "aria-disabled", !tap.bus);
    send.tabIndex = tap.bus ? 0 : -1;
    send.closest(".knob").classList.toggle("filter-unused", !tap.bus);
    const name = target?.name ?? `Bus ${tap.bus}`;
    setText(
      row.querySelector(".tap-status"),
      !sends
        ? skipped ? "Skipped: its input goes on" : ""
        : tap.bus && !target
          ? `${name} is gone: nothing is sent`
          : skipped
            ? sending ? `${name} only: the chain skips this section` : "Skipped: its input goes on"
            : target && !sending ? `Turn SEND up to feed ${name}` : "",
    );
    const labels = [
      ...(skipped ? [["skip", "Skipped"]] : tap.chain < 1 ? [["chain", `Chain ${percent(tap.chain)}`]] : []),
      ...(sending ? [["send", `→ ${target.name} ${percent(tap.send)}`]] : []),
    ];
    const text = JSON.stringify(labels);
    if (chips.dataset.labels !== text) {
      chips.dataset.labels = text;
      chips.replaceChildren(
        ...labels.map(([kind, caption]) => {
          const chip = document.createElement("span");
          chip.className = `tap-chip ${kind}`;
          chip.textContent = caption;
          return chip;
        }),
      );
    }
  }
}
function bindGranularToggle() {
  $("history-granular").onclick = () => {
    const on = $("history-granular").getAttribute("aria-pressed") !== "true";
    if (preset?.drums) {
      preset.drums.granular = on;
      scheduleParameter("drum_granular", Number(on));
    } else if (preset?.synth) editSynth("granular", on);
    else if (preset?.bus) {
      // Structural: a Granular bus prepares its history with the preset.
      preset.bus.granular = on;
      schedulePreset();
    } else return;
    syncTimbreRouting();
    if (state?.input) syncInput(state.input);
  };
}
/** The live history's length belongs to Granular, so it sits in Grain engine. */
const historyBuffers = (bus) =>
  [0.125, 0.25, 0.5, 1, 2, 4, 8, ...(bus ? [] : [16, 32])];
function bindGrainBuffer() {
  const buffer = $("grain-buffer");
  buffer.onchange = () => {
    const seconds = Number(buffer.value);
    if (preset?.drums) {
      preset.drums.buffer_seconds = seconds;
      scheduleParameter("drum_buffer_seconds", seconds);
    } else if (preset?.synth) editSynth("buffer_seconds", seconds);
    else if (preset?.bus) {
      // Structural: a Granular bus allocates its history while the preset is
      // prepared, never on the audio thread.
      preset.bus.buffer_seconds = seconds;
      schedulePreset();
    } else return;
    syncTimbreRouting();
  };
}
function syncGrainBuffer() {
  const own = historySettings(),
    buffer = $("grain-buffer");
  $("grain-buffer-field").hidden = !own;
  if (!own) return;
  const bus = !preset.drums && !preset.synth,
    seconds = own.buffer_seconds ?? (bus ? 4 : 8),
    choices = historyBuffers(bus),
    custom = choices.includes(seconds) ? "" : seconds;
  // A preset may carry a length the menu doesn't offer; keep it selectable.
  if (buffer.dataset.choices !== `${bus}:${custom}`) {
    buffer.dataset.choices = `${bus}:${custom}`;
    buffer.replaceChildren(
      ...choices.map((s) => option(String(s), s < 1 ? `${s * 1000} ms` : `${s} s`)),
    );
    if (custom !== "")
      buffer.append(option(String(seconds), `${Number(seconds.toFixed(3))} s`));
  }
  if (document.activeElement !== buffer) buffer.value = String(seconds);
}
function buildBus() {
  $("bus-filters").onclick = () => {
    if (!preset?.bus) return;
    preset.bus.filters = !preset.bus.filters;
    schedulePreset();
    syncTimbreRouting();
  };
}
function syncBus() {
  const bus = preset?.bus;
  $("bus-engine").hidden = !bus;
  // Only a bus can switch Filter 1 and 2 off; elsewhere they always run.
  $("bus-filters").hidden = !bus;
  if (!bus) return;
  const title = $("bus-title");
  const vocal = !!preset.vocal;
  const kit = !!preset.sampler;
  const active = state?.timbres?.timbres?.find((t) => t.id === state.timbres.active);
  const padFx = active?.kind === "pad_fx";
  const owner = padFx && state.timbres.timbres.find((t) => t.id === active.owner);
  title.firstChild.textContent = bus.master ? "Master " : vocal ? "Voice chain "
    : kit ? "Kit chain " : padFx ? `${active.name} ` : "Bus ";
  title.querySelector("span").textContent = bus.master
    ? "every timbre and bus, then the output"
    : vocal ? "the sung voice, then Master"
    : kit ? "every pad, then Master"
    : padFx ? `one pad's voices, back into ${owner?.name || "its Sampler"}`
    : "sends from timbres, then Master";
  $("bus-engine").setAttribute("aria-label", bus.master ? "Master" : vocal ? "Voice chain"
    : kit ? "Kit chain" : padFx ? "Pad FX chain" : "Bus");
  $("bus-back").hidden = !owner;
  $("bus-back").textContent = owner ? `◀ ${owner.name}` : "";
  $("bus-back").onclick = () => owner && changeTimbre("/api/timbres/select", { id: owner.id });
  setAttr($("bus-filters"), "aria-pressed", !!bus.filters);
  const effects = [preset.color_chain, preset.pre_color_chain, preset.source_color_chain]
    .some((chain) => chain?.some((slot) => slot.enabled && slot.mix > 0));
  $("bus-hint").hidden =
    !!bus.granular || !!bus.filters || !!preset.resynth?.enabled || effects
    || (preset.filter_slots?.length ?? 0) > 0;
}
function buildSynth() {
  bindEngineType();
  buildBus();
  bindGranularToggle();
  bindGrainBuffer();
  $("synth-add").onclick = () => {
    const s = synthSettings();
    if (s.count >= 4) return;
    editSynth("count", s.count + 1);
    syncSynth();
  };
}
const synthDefault = () => ({
  count: 1,
  oscillators: Array.from({ length: 4 }, (_, i) => synthOscDefault(i)),
  granular: false,
  buffer_seconds: 8,
});
// Hosts without timbres (desktop) switch their single engine instead.
function syncEngineType() {
  const root = $("engine-type"),
    select = $("engine-kind");
  root.hidden = !state || !!state.timbres?.timbres;
  select.querySelector('[value="drums"]').hidden = !preset?.drums;
  if (document.activeElement !== select)
    select.value = preset?.drums ? "drums" : preset?.synth ? "synth" : "granular";
}
function bindEngineType() {
  $("engine-kind").onchange = () => {
    const synth = $("engine-kind").value === "synth";
    if (synth === !!preset.synth) return;
    if (synth) {
      preset.drums = null;
      preset.synth = synthDefault();
      // A synth waits for notes instead of droning at C4 with the transport.
      request("/api/transport", { midi_gate: true }).catch((e) =>
        notify(e.message, true),
      );
    } else delete preset.synth;
    schedulePreset();
    syncAll();
  };
}
function syncSynth() {
  const bank = $("synth-engine");
  bank.hidden = !preset?.synth;
  if (!preset?.synth) {
    if (OSC_BANKS.synth.signature) {
      OSC_BANKS.synth.signature = "";
      for (const key of controls.keys())
        if (key.startsWith("synth_osc")) {
          controls.delete(key);
          modulationViews.delete(key);
        }
      $("synth-oscillators").replaceChildren();
    }
    return;
  }
  const s = synthSettings();
  $("synth-add").disabled = s.count >= 4;
  syncOscillators("synth");
}
function syncOscillators(bank) {
  const config = OSC_BANKS[bank],
    { label: name } = config,
    edit = (field, value, index) => editOscillator(bank, field, value, index),
    knob = (...args) => oscillatorKnob(bank, ...args),
    container = $(`${bank}-oscillators`);
  const r = config.settings();
  const signature = JSON.stringify([
    r.count,
    r.oscillators.slice(0, r.count).map((o) => [o.model, o.waveform, o.mode]),
  ]);
  if (signature !== config.signature) {
    config.signature = signature;
    for (const key of controls.keys())
      if (key.startsWith(`${bank}_osc`)) {
        controls.delete(key);
        modulationViews.delete(key);
      }
    container.replaceChildren();
    for (let i = 0; i < r.count; i++) {
      const o = r.oscillators[i],
        analog = RESYNTH_WAVES.findIndex(([key]) => key === o.waveform) < 4,
        folder = o.model === "wavefolder";
      const card = document.createElement("div");
      card.className = "color-card resynth-oscillator";
      card.dataset.oscillator = i + 1;
      card.innerHTML = `<div class="resynth-osc-title"><strong class="color-order">${String(i + 1).padStart(2, "0")}</strong><select class="resynth-wave" aria-label="${name} OSC ${i + 1} waveform"></select><select class="resynth-mode" aria-label="${name} OSC ${i + 1} mode"></select><div class="color-actions"><button class="color-bypass" aria-label="Bypass ${name} OSC ${i + 1}" title="Bypass">⏻</button><button class="resynth-remove" aria-label="Remove ${name} OSC ${i + 1}" title="Remove">×</button></div></div><div class="resynth-extra"></div><div id="${bank}-osc${i + 1}-knobs" class="knob-grid resynth-knobs"></div>`;
      const wave = card.querySelector(".resynth-wave"),
        mode = card.querySelector(".resynth-mode");
      wave.replaceChildren(
        ...config.waves
          .filter(([key]) => !folder || RESYNTH_WAVES.findIndex(([k]) => k === key) < 4)
          .map(([key, label]) => option(key, label)),
      );
      wave.value = o.waveform;
      wave.onchange = () => {
        edit("waveform", wave.value, i);
        config.sync();
      };
      mode.replaceChildren(
        ...RESYNTH_MODES.map(([key, label]) => option(key, label)),
      );
      mode.value = o.mode;
      mode.hidden = !analog || folder;
      mode.onchange = () => {
        edit("mode", mode.value, i);
        config.sync();
      };
      card.querySelector(".color-bypass").onclick = () => {
        edit("enabled", !config.settings().oscillators[i].enabled, i);
        config.sync();
      };
      card.querySelector(".resynth-remove").disabled = r.count === 1;
      card.querySelector(".resynth-remove").onclick = () => {
        const current = config.settings();
        const remaining = current.oscillators
          .slice(0, current.count)
          .filter((_, index) => index !== i);
        for (const osc of remaining) {
          if (osc.cross_source === i + 1) osc.cross_source = 0;
          else if (osc.cross_source > i + 1) osc.cross_source--;
          if (osc.interaction_source === i + 2)
            osc.interaction_source = config.detached;
          else if (osc.interaction_source > i + 2) osc.interaction_source--;
        }
        while (remaining.length < 4)
          remaining.push(config.defaults(remaining.length));
        remaining.forEach((osc, index) =>
          Object.entries(osc).forEach(([field, value]) =>
            edit(field, value, index),
          ),
        );
        edit("count", current.count - 1);
        config.sync();
      };
      const choice = (field, label, entries, value, change) => {
        const wrapper = document.createElement("label");
        wrapper.className = `resynth-choice resynth-choice-${field}`;
        const caption = document.createElement("span");
        caption.textContent = label;
        const select = document.createElement("select");
        select.className = `resynth-${field}`;
        select.setAttribute(
          "aria-label",
          `${name} OSC ${i + 1} ${label.toLowerCase()}`,
        );
        select.replaceChildren(
          ...entries.map(([key, text]) => option(key, text)),
        );
        select.value = String(value);
        select.onchange = () => change(select.value);
        wrapper.append(caption, select);
        card.querySelector(".resynth-extra").append(wrapper);
        return wrapper;
      };
      const model = choice(
        "model",
        "Generator",
        RESYNTH_MODELS,
        o.model,
        (value) => {
          edit("model", value, i);
          if (value === "wavefolder") {
            if (!analog) edit("waveform", "sine", i);
            edit("control1", 32 / 127, i);
            edit("control2", 64 / 127, i);
          }
          config.sync();
        },
      );
      model.classList.toggle("resynth-choice-wide", !folder);
      if (folder)
        choice("fold_shape", "Shaper", RESYNTH_FOLDS, o.fold_shape, (value) =>
          edit("fold_shape", value, i),
        );
      choice(
        "interaction",
        "Ring / Sync",
        RESYNTH_INTERACTIONS.filter(
          ([key]) => o.waveform !== "audio_in" || ["off", "ring"].includes(key),
        ),
        o.interaction,
        (value) => edit("interaction", value, i),
      );
      const references = [
        ...config.references,
        ...Array.from({ length: r.count }, (_, j) => [
          j + 2,
          `OSC ${j + 1}`,
        ]).filter(([id]) => id !== i + 2),
      ];
      if (!references.some(([id]) => id === o.interaction_source))
        references.push([
          o.interaction_source,
          `OSC ${o.interaction_source - 1} · inactive`,
        ]);
      choice(
        "interaction_source",
        "Ring / Sync source",
        references,
        o.interaction_source,
        (value) => edit("interaction_source", Number(value), i),
      );
      if (analog && !folder && o.mode === "cross") {
        const references = [
          ...config.cross,
          ...Array.from({ length: r.count }, (_, j) => [
            `osc${j + 1}`,
            `OSC ${j + 1}`,
          ]),
        ];
        choice(
          "cross-source",
          "Cross source",
          references,
          o.cross_source ? `osc${o.cross_source}` : o.cross_input,
          (value) => {
            edit(
              "cross_source",
              value.startsWith("osc") ? Number(value.slice(3)) : 0,
              i,
            );
            edit(
              "cross_input",
              value.startsWith("osc") ? "analysis" : value,
              i,
            );
          },
        ).classList.add("resynth-choice-wide");
      }
      container.append(card);
      if (o.waveform !== "audio_in") {
        const labels = folder
          ? ["FOLD DEPTH", "BIAS"]
          : o.waveform === "pcm"
            ? ["START", "LOOP FADE"]
            : analog
              ? {
                  wave: ["SHAPE", "LFO 1"],
                  cross: ["DEPTH", "LFO 1"],
                  unison: ["DETUNE", "PHASE"],
                  vpm: ["DEPTH", "RATIO"],
                }[o.mode]
              : ["CTRL 1", "CTRL 2"];
        knob(
          i,
          "control1",
          "CTRL 1",
          percent,
          0,
          1,
          labels[0] === "CTRL 1" ? "" : labels[0],
        );
        knob(
          i,
          "control2",
          "CTRL 2",
          folder
            ? (v) => `${Math.round(((Math.round(v * 127) - 64) / 64) * 100)}%`
            : analog && o.mode === "vpm"
              ? (v) => `${Math.floor(Math.round(v * 127) / 4) + 1}:1`
              : percent,
          0,
          1,
          labels[1] === "CTRL 2" ? "" : labels[1],
        );
        knob(
          i,
          "transpose",
          "TRANSPOSE",
          (v) => `${v >= 0 ? "+" : ""}${v.toFixed(1)} st`,
          -48,
          48,
        );
        knob(
          i,
          "fine",
          "FINE",
          (v) => `${v >= 0 ? "+" : ""}${v.toFixed(1)} ct`,
          -100,
          100,
        );
      }
      knob(i, "level", "LEVEL");
      knob(i, "pan", "PAN", (v) =>
        v === 0.5
          ? "C"
          : `${Math.round(Math.abs(v - 0.5) * 200)}${v < 0.5 ? "L" : "R"}`,
      );
    }
  }
  for (const card of container.children) {
    const o = r.oscillators[Number(card.dataset.oscillator) - 1];
    card.classList.toggle("is-bypassed", !o.enabled);
    card
      .querySelector(".color-bypass")
      .setAttribute("aria-pressed", String(!o.enabled));
    for (const field of [
      "model",
      "fold_shape",
      "interaction",
      "interaction_source",
    ])
      if (card.querySelector(`.resynth-${field}`))
        card.querySelector(`.resynth-${field}`).value = String(o[field]);
    const cross = card.querySelector(".resynth-cross-source");
    if (cross)
      cross.value = o.cross_source ? `osc${o.cross_source}` : o.cross_input;
  }
  for (const [key, update] of controls)
    if (key.startsWith(`${bank}_`)) update();
}
function updateResynthMeter(meter) {
  const active = resynthSettings().enabled && state?.playing;
  [...$("resynth-spectrum").children].forEach((bar, i) => {
    const level = active ? meter?.bands?.[i] || 0 : 0;
    bar.firstElementChild.style.transform = `scaleY(${Math.min(1, Math.sqrt(level))})`;
    const hz = meter?.frequencies?.[i];
    if (hz)
      bar.title = `${hz < 1000 ? Math.round(hz) + " Hz" : (hz / 1000).toFixed(1) + " kHz"} · ${Math.round(level * 100)}%`;
  });
}

let colorDefaults = null,
  colorSignature = "",
  colorRoutesSignature = "",
  nextColorId = 1;
const COLOR_TYPES = [
  ["distortion", "Distortion"],
  ["reducer", "Rate reducer"],
  ["crusher", "Bit crusher"],
  ["flanger", "Flanger"],
  ["chorus", "Chorus"],
  ["binaural", "Binaural"],
  ["reverb", "Reverb"],
  ["lcr_delay", "L/C/R Delay"],
  ["mod_delay", "Mod Delay"],
  ["compressor", "Compressor"],
  ["waveshaper", "Waveshaper"],
  ["super_echo", "SuperEcho"],
  ["alpha_verb", "AlphaVerb"],
];
// Sections of the effect menus, as in a DAW's plug-in menu.
const COLOR_SECTIONS = [
  ["Distortion", ["distortion", "waveshaper", "crusher", "reducer"]],
  ["Modulation", ["chorus", "flanger"]],
  ["Delay", ["lcr_delay", "mod_delay", "super_echo"]],
  ["Reverb", ["reverb", "alpha_verb"]],
  ["Dynamics", ["compressor"]],
  ["Imaging", ["binaural"]],
];
const colorTypeGroups = () =>
  COLOR_SECTIONS.map(([label, kinds]) => ({
    label,
    choices: kinds.map((kind) => COLOR_TYPES.find(([k]) => k === kind)),
  }));
// The MS-70CDR set arrives with the color defaults: one type per model, named
// "mooz <original name>", with nine generic controls read through its catalog.
const MOOZ = { models: new Map(), delayNotes: [], rateNotes: [] };
const isMooz = (kind) => typeof kind === "string" && kind.startsWith("mooz_");
function moozValue(param, v) {
  return Math.round(Math.max(0, Math.min(1, v)) * param.max);
}
function moozFormat(param, v) {
  const value = moozValue(param, v),
    d = param.display;
  if (!d) return String(value);
  const number = (x) => `${(x * d.scale + d.offset).toFixed(d.decimals)}${d.unit}`;
  if (d.type === "labels") return d.labels[value] ?? String(value);
  if (d.type === "delay_time")
    return value <= d.threshold
      ? `${value + (d.offset ?? 1)} ms`
      : MOOZ.delayNotes[value - d.threshold - 1] ?? String(value);
  if (d.type === "sync_rate")
    return value <= d.threshold
      ? number(value)
      : MOOZ.rateNotes[value - d.threshold - 1] ?? String(value);
  return number(value);
}
function registerMooz(catalog) {
  if (!catalog || MOOZ.models.size) return;
  MOOZ.delayNotes = catalog.delay_notes;
  MOOZ.rateNotes = catalog.rate_notes;
  const models = [...catalog.models].sort((a, b) =>
    a.name.localeCompare(b.name, "en", { sensitivity: "base" }),
  );
  for (const model of models) {
    MOOZ.models.set(model.key, model);
    COLOR_TYPES.push([model.key, `mooz ${model.name}`]);
    COLOR_SCHEMAS[model.key] = model.params.map((param, i) => [
      `mooz_p${i + 1}`,
      param.name.toUpperCase(),
      (v) => moozFormat(param, v),
    ]);
  }
  for (const { key, label } of catalog.categories) {
    const kinds = models.filter((m) => m.category === key).map((m) => m.key);
    if (kinds.length) COLOR_SECTIONS.push([`mooz · ${label}`, kinds]);
  }
}
function moozParam(kind, parameter) {
  const match = /^mooz_p(\d)$/.exec(parameter);
  return match && MOOZ.models.get(kind)?.params[Number(match[1]) - 1];
}
// Pedal values are integers; long ranges step faster from the keyboard.
function moozBinding(kind, parameter) {
  const param = moozParam(kind, parameter);
  if (!param || !param.max) return {};
  const step = param.max > 200 ? Math.round(param.max / 200) : 1;
  const snap = (v) => Math.max(0, Math.min(param.max, Math.round(v * param.max))) / param.max;
  return {
    quantize: (v) => snap(v),
    increment: (v, direction) => snap(v + (direction * step) / param.max),
    reset: () => param.default / param.max,
  };
}
// A new or retyped slot starts at its model's original settings.
function applyKindDefaults(slot) {
  if (slot.kind === "compressor") slot.mix = 1;
  const model = MOOZ.models.get(slot.kind);
  if (!model) return;
  slot.mix = 1;
  for (let i = 0; i < 9; i++) {
    const param = model.params[i];
    slot.settings[`mooz_p${i + 1}`] = param && param.max ? param.default / param.max : 0;
  }
}
const COLOR_RACKS = [
  {
    key: "source_color_chain",
    element: "source-color-chain",
    add: "source-color-add",
    label: "Source FX",
    max: 1,
    prefix: "source_",
  },
  {
    key: "pre_color_chain",
    element: "pre-color-chain",
    add: "pre-color-add",
    label: "Pre FX",
    prefix: "pre_",
  },
  { key: "color_chain", element: "color-chain", add: "color-add", label: "FX" },
];
const WAVESHAPER_TYPES = [
  ["drive", "Drive"],
  ["hard_clip", "Hard Clip"],
  ["decimator", "Decimator"],
  ["oct_saw", "OctSaw"],
  ["multi_triangle", "MultiTri"],
  ["multi_sine", "MultiSin"],
  ["sub_saw", "SubSaw"],
  ["sub_square", "SubSquare"],
  ["sub_triangle", "SubTriangle"],
  ["sub_sine", "SubSine"],
  ["pickup", "Pickup"],
  ["level_boost", "LvlBoost"],
];
function colorEntries() {
  return COLOR_RACKS.flatMap((rack) =>
    (preset?.[rack.key] || []).map((slot, index) => ({ rack, slot, index })),
  );
}
function findColor(id) {
  return colorEntries().find((entry) => entry.slot.id === id)?.slot;
}
const REVERB_TYPES = [
  ["plate_s", "Small plate"],
  ["plate_l", "Large plate"],
  ["room", "Room"],
  ["reverse", "Reverse"],
];
const colorRate = (v) => `${(0.01 * 2000 ** v).toFixed(2)} Hz`;
const colorDrive = (v) => `${(20 * Math.log10(1 + 18 * v)).toFixed(1)} dB`;
const colorFeedback = (v) => signedPercent((2 * v - 1) * 0.95);
const colorStereo = (v) => `${Math.round(v * 180)}°`;
const DELAY_NOTES = [
  "1/64",
  "1/32",
  "1/16T",
  "1/16",
  "1/8T",
  "1/8",
  "1/4T",
  "1/8.",
  "1/4",
  "1/2T",
  "1/4.",
  "1/2",
  "1/2.",
  "1/1",
];
const delayPrefix = (kind) =>
  kind === "lcr_delay" ? "lcr" : kind === "mod_delay" ? "mod_delay" : null;
// Match the engine's f32 control positions, including exact halfway values.
const delayPosition = (v, count) =>
  Math.fround(Math.fround(Math.max(0, Math.min(1, v))) * (count - 1));
const delayIndex = (v, count = 128) => Math.round(delayPosition(v, count));
function delayLookup(table, v, smooth) {
  if (!smooth) return table[delayIndex(v, table.length)];
  const position = delayPosition(v, table.length);
  const left = Math.floor(position),
    right = Math.min(left + 1, table.length - 1);
  return table[left] + (table[right] - table[left]) * (position - left);
}
function delayRatio(v, slot, prefix) {
  const tables = colorDefaults.delay_tables;
  const table =
    tables[slot.settings[`${prefix}_sync`] ? "sync_ratio" : "free_ratio"];
  return `${(delayLookup(table, v, prefix === "lcr" || slot.settings[`${prefix}_smooth`]) / 10).toFixed(prefix === "lcr" ? 2 : 1)}%`;
}
// Keep existing preset/automation coordinates; gestures use physical percent,
// avoiding the long repeated entries in the original RADIAS sync table.
function lcrRatioBinding(getSlot) {
  const table = () => colorDefaults.delay_tables[getSlot().settings.lcr_sync ? "sync_ratio" : "free_ratio"];
  const ratio = (v) => delayLookup(table(), v, true);
  const position = (value) => delayRatioPosition(table(), value);
  return {
    normalize: (v) => (ratio(v) - table()[0]) / (table().at(-1) - table()[0]),
    denormalize: (v) => position(table()[0] + v * (table().at(-1) - table()[0])),
    quantize: (v, origin = 0) => {
      if (v <= 0 || v >= 1) return v;
      const base = Math.round(ratio(origin) * 10) / 10;
      return position(base + Math.round((ratio(v) - base) / 2.5) * 2.5);
    },
    increment: (v, direction) => position(Math.round(ratio(v) * 10) / 10 + direction * 2.5),
  };
}
function delayRatioPosition(table, value) {
  if (value <= table[0]) return 0;
  const right = table.findIndex((ratio) => ratio >= value);
  if (right < 0) return 1;
  const left = right - 1;
  return (left + (value - table[left]) / (table[right] - table[left])) / (table.length - 1);
}
function delayTime(v, slot, prefix, sync) {
  const tables = colorDefaults.delay_tables;
  const smooth = slot.settings[`${prefix}_smooth`];
  const ratio =
    delayLookup(
      tables[sync ? "sync_ratio" : "free_ratio"],
      slot.settings[`${prefix}_ratio`],
      prefix === "lcr" || smooth,
    ) / 1000;
  const seconds = sync
    ? ((60 / preset.params.bpm) *
        ratio *
        delayLookup(tables.notes, v, smooth)) /
      192
    : (ratio *
        delayLookup(
          tables[prefix === "lcr" ? "lcr_ms" : "mod_ms"],
          v,
          smooth,
        )) /
      1000;
  const ms = Math.round(Math.min(tables.max_seconds, seconds) * 1000);
  if (sync && smooth) {
    const position = delayPosition(v, DELAY_NOTES.length),
      nearest = Math.round(position);
    if (Math.abs(position - nearest) < 0.005)
      return `${DELAY_NOTES[nearest]}\n${ms} ms`;
    const left = Math.floor(position),
      fraction = Math.max(1, Math.min(99, Math.round((position - left) * 100)));
    return `${DELAY_NOTES[left]}→${DELAY_NOTES[left + 1]}\n${fraction}% · ${ms} ms`;
  }
  return sync ? `${DELAY_NOTES[delayIndex(v, 14)]} · ${ms} ms` : `${ms} ms`;
}
const SUPER_ECHO_NOTES = [
  "1/32",
  "1/16T",
  "1/16",
  "1/8T",
  "1/16D",
  "1/8",
  "1/4T",
  "1/8D",
  "1/4",
  "1/2T",
  "1/4D",
  "1/2",
  "1/1T",
  "1/2D",
  "1/1",
  "1/1D",
  "2/1",
];
const IMPORTED_FX_CHOICES = {
  super_echo: [
    ["super_echo_sync", "CLOCK", ["Free", "Tempo sync"]],
    ["super_echo_dirty", "DELAY", ["Tape", "BBD"]],
    ["super_echo_shimmer_style", "SHIMMER", ["Pitch", "Frequency"]],
    ["super_echo_shimmer_mode", "SHIMMER MODE", ["A", "B"]],
    ["super_echo_filter_mode", "FILTER", ["Feedback", "Series"]],
    ["super_echo_dub", "DUB", ["Off", "On"]],
    ["super_echo_freeze", "FREEZE", ["Off", "On"]],
  ],
  alpha_verb: [
    ["alpha_verb_dirty", "CHARACTER", ["Clean", "Dirty"]],
    [
      "alpha_verb_dynamics",
      "DYNAMICS",
      ["Off", "Comp + Gate", "Compressor", "Gate"],
    ],
    ["alpha_verb_freeze", "FREEZE", ["Off", "On"]],
  ],
};
const importedDb = (v) => `${(-96 + 96 * v).toFixed(1)} dB`;
const importedRelease = (v) => `${Math.round(50 + 950 * v)} ms`;
const COLOR_SCHEMAS = {
  super_echo: [
    [
      "super_echo_time",
      "TIME",
      (v) => `${(1000 * 2 ** ((-96 + 120 * v) / 12)).toFixed(1)} ms`,
    ],
    ["super_echo_feedback", "FEEDBACK", (v) => percent(v * 1.27)],
    [
      "super_echo_cutoff",
      "CUTOFF",
      (v) => {
        const hz = 440 * 2 ** ((10 + 127 * v - 69) / 12);
        return hz >= 1000
          ? `${(hz / 1000).toFixed(2)} kHz`
          : `${Math.round(hz)} Hz`;
      },
    ],
    ["super_echo_resonance", "RESONANCE", percent],
    ["super_echo_orbit", "ORBIT", percent],
    [
      "super_echo_ping_pong",
      "PING PONG",
      (v) =>
        Math.abs(v - 0.5) < 0.01
          ? "Centre"
          : `${Math.round(Math.abs(v * 2 - 1) * 100)}% ${v < 0.5 ? "L" : "R"}`,
    ],
    ["super_echo_mod_speed", "MOD RATE", (v) => `${(v * 2).toFixed(2)} Hz`],
    ["super_echo_mod_depth", "MOD DEPTH", percent],
    ["super_echo_pitch", "PITCH", (v) => `${(v * 24 - 12).toFixed(1)} st`],
    ["super_echo_shimmer", "SHIMMER", percent],
    ["super_echo_reverb", "REVERB", percent],
    ["super_echo_compression", "COMPRESS", percent],
    ["super_echo_dub_mix", "DUB MIX", percent],
  ],
  alpha_verb: [
    ["alpha_verb_predelay", "PREDELAY", (v) => `${Math.round(4 + 196 * v)} ms`],
    ["alpha_verb_early_late", "EARLY / LATE", percent],
    ["alpha_verb_size", "SIZE", percent],
    ["alpha_verb_shape", "SHAPE", percent],
    ["alpha_verb_feedback", "FEEDBACK", percent],
    ["alpha_verb_low_damp", "LOW DAMP", percent],
    ["alpha_verb_high_damp", "HIGH DAMP", percent],
    ["alpha_verb_bass", "BASS", percent],
    ["alpha_verb_treble", "TREBLE", percent],
    ["alpha_verb_spin", "SPIN", percent],
    ["alpha_verb_stereo", "STEREO", (v) => percent(v * 2)],
    ["alpha_verb_shimmer", "SHIMMER", percent],
    ["alpha_verb_comp_threshold", "COMP THRESH", importedDb],
    ["alpha_verb_comp_ratio", "COMP RATIO", percent],
    ["alpha_verb_comp_release", "COMP RELEASE", importedRelease],
    ["alpha_verb_gate_threshold", "GATE THRESH", importedDb],
    ["alpha_verb_gate_hold", "GATE HOLD", importedRelease],
    ["alpha_verb_gate_release", "GATE RELEASE", importedRelease],
  ],
  waveshaper: [
    ["waveshaper_depth", "DEPTH", (v) => String(Math.round(v * 127))],
  ],
  distortion: [
    ["distortion_drive", "DRIVE", colorDrive],
    [
      "distortion_tone",
      "TONE",
      (v) => `${Math.round(20 * 1000 ** v).toLocaleString()} Hz`,
    ],
    ["distortion_bias", "BIAS", (v) => signedPercent(v * 2 - 1)],
  ],
  reducer: [
    [
      "reducer_rate",
      "RATE",
      (v) =>
        `${Math.round((state?.sample_rate || 48000) / 2 ** (10 * (1 - v))).toLocaleString()} Hz`,
    ],
    ["reducer_jitter", "JITTER", percent],
    ["reducer_smooth", "SMOOTH", percent],
  ],
  crusher: [
    ["crusher_bits", "BITS", (v) => `${(1 + 15 * v).toFixed(1)} bit`],
    ["crusher_drive", "DRIVE", colorDrive],
    ["crusher_dither", "DITHER", percent],
  ],
  flanger: [
    ["flanger_rate", "RATE", colorRate],
    ["flanger_depth", "DEPTH", percent],
    ["flanger_delay", "DELAY", (v) => `${(0.1 + 9.9 * v).toFixed(2)} ms`],
    ["flanger_feedback", "FEEDBACK", colorFeedback],
    ["flanger_stereo", "STEREO", colorStereo],
  ],
  chorus: [
    ["chorus_rate", "RATE", colorRate],
    ["chorus_depth", "DEPTH", percent],
    ["chorus_delay", "DELAY", (v) => `${(5 + 35 * v).toFixed(1)} ms`],
    ["chorus_feedback", "FEEDBACK", colorFeedback],
    ["chorus_stereo", "STEREO", colorStereo],
  ],
  binaural: [
    ["binaural_delay", "DELAY", (v) => `${(1.5 * v).toFixed(2)} ms`],
    ["binaural_crossfeed", "CROSSFEED", (v) => percent(v * 0.75)],
    ["binaural_width", "WIDTH", (v) => percent(v * 2)],
  ],
  reverb: [
    ["reverb_predelay", "PREDELAY", (v) => `${Math.round(v * v * 200)} ms`],
    ["reverb_decay", "DECAY", (v) => `${(0.25 + v * v * 14).toFixed(2)} s`],
    ["reverb_tone", "TONE", percent],
    ["reverb_shimmer", "SHIMMER", percent],
    ["reverb_diffuse", "DIFFUSE", percent],
  ],
  lcr_delay: [
    ["lcr_left_delay", "L DELAY", (v, s) => delayTime(v, s, "lcr", false)],
    ["lcr_center_delay", "C DELAY", (v, s) => delayTime(v, s, "lcr", false)],
    ["lcr_right_delay", "R DELAY", (v, s) => delayTime(v, s, "lcr", false)],
    ["lcr_left_level", "L LEVEL", percent],
    ["lcr_center_level", "C LEVEL", percent],
    ["lcr_right_level", "R LEVEL", percent],
    ["lcr_ratio", "TIME RATIO", (v, s) => delayRatio(v, s, "lcr")],
    ["lcr_feedback", "C FEEDBACK", (v) => percent(Math.min(0.995, v))],
    ["lcr_high_damp", "HIGH DAMP", percent],
    ["lcr_low_damp", "LOW DAMP", percent],
    ["lcr_trim", "TRIM", percent],
    ["lcr_spread", "SPREAD", percent],
  ],
  mod_delay: [
    [
      "mod_delay_left_delay",
      "L DELAY",
      (v, s) => delayTime(v, s, "mod_delay", false),
    ],
    [
      "mod_delay_right_delay",
      "R DELAY",
      (v, s) => delayTime(v, s, "mod_delay", false),
    ],
    ["mod_delay_ratio", "TIME RATIO", (v, s) => delayRatio(v, s, "mod_delay")],
    ["mod_delay_feedback", "FEEDBACK", (v) => percent(Math.min(0.995, v))],
    ["mod_delay_depth", "MOD DEPTH", percent],
    [
      "mod_delay_rate",
      "LFO FREQ",
      (v) =>
        `${colorDefaults.delay_tables.lfo_hz[delayIndex(v)].toFixed(2)} Hz`,
    ],
    [
      "mod_delay_spread",
      "LFO SPREAD",
      (v) => `${Math.round((v * 2 - 1) * 180)}°`,
    ],
  ],
  compressor: [
    [
      "compressor_threshold",
      "THRESHOLD",
      (v) => `${(-60 + 60 * v).toFixed(1)} dB`,
    ],
    ["compressor_ratio", "RATIO", (v) => `${(20 ** v).toFixed(1)}:1`],
    [
      "compressor_attack",
      "ATTACK",
      (v) => `${(0.1 * 1000 ** v).toFixed(1)} ms`,
    ],
    ["compressor_release", "RELEASE", (v) => `${Math.round(10 * 200 ** v)} ms`],
    ["compressor_knee", "KNEE", (v) => `${(24 * v).toFixed(1)} dB`],
    ["compressor_makeup", "MAKEUP", (v) => `${(24 * v).toFixed(1)} dB`],
  ],
};
function colorSchema(kind, slot = null) {
  const prefix = delayPrefix(kind);
  const sync = prefix && slot?.settings[`${prefix}_sync`];
  const schema = (COLOR_SCHEMAS[kind] || []).map(([parameter, label, format]) => {
    if (
      kind === "super_echo" &&
      parameter === "super_echo_time" &&
      slot?.settings.super_echo_sync >= 0.5
    )
      return [
        "super_echo_note",
        "TIME",
        (v) => SUPER_ECHO_NOTES[Math.round(v * 16)],
      ];
    if (sync && /_(left|center|right)_delay$/.test(parameter))
      return [
        parameter.replace(/_delay$/, "_note"),
        label,
        (v, s) => delayTime(v, s, prefix, true),
      ];
    return [parameter, label, format];
  });
  // Mooz models have their own MIX; the slot's dry/wet reads as BLEND there.
  return [...schema, ["mix", isMooz(kind) ? "BLEND" : "MIX", percent]];
}
function colorDestinationSchema(kind) {
  const schema = colorSchema(kind);
  const prefix = delayPrefix(kind);
  if (prefix)
    schema.push(
      ...colorSchema(kind, { settings: { [`${prefix}_sync`]: true } })
        .filter(([parameter]) => parameter.endsWith("_note"))
        .map(([parameter, label, format]) => [
          parameter,
          `${label} SYNC`,
          format,
        ]),
    );
  if (kind === "super_echo")
    schema.push([
      "super_echo_note",
      "TIME SYNC",
      (v) => SUPER_ECHO_NOTES[Math.round(v * 16)],
    ]);
  for (const [parameter, label, values] of IMPORTED_FX_CHOICES[kind] || [])
    schema.push([
      parameter,
      label,
      (v) => values[Math.round(v * (values.length - 1))],
    ]);
  return schema;
}
const destinationValue = (value) =>
  typeof value === "string"
    ? value
    : JSON.stringify(value, (_, entry) =>
        entry && typeof entry === "object" && !Array.isArray(entry)
          ? Object.fromEntries(
              Object.keys(entry)
                .sort()
                .map((key) => [key, entry[key]]),
            )
          : entry,
      );
const parseDestination = (value) =>
  value.startsWith("{") ? JSON.parse(value) : value;
function colorLegacy() {
  const first = preset.color_chain[0];
  preset.params.saturation = first?.mix ?? 0;
  preset.params.saturator_type = first?.kind ?? "distortion";
  const reverb = preset.color_chain.find((s) => s.kind === "reverb");
  preset.fx_chain_version = 1;
  preset.params.reverb = reverb?.mix ?? 0;
  if (reverb) {
    preset.params.reverb_type = reverb.settings.reverb_type;
    for (const [field, parameter] of [
      ["predelay", "reverb_predelay"],
      ["reverb_decay", "reverb_decay"],
      ["reverb_tone", "reverb_tone"],
      ["shimmer", "reverb_shimmer"],
      ["diffuse", "reverb_diffuse"],
    ])
      preset.params[field] = reverb.settings[parameter];
  }
}
function editColorChain() {
  colorLegacy();
  syncColors();
  syncLfos();
  syncEnvs();
  schedulePreset();
}
function removeColor(id) {
  for (const rack of COLOR_RACKS)
    preset[rack.key] = preset[rack.key].filter((s) => s.id !== id);
  unroute((d) => d?.color?.slot === id);
  editColorChain();
}
function moveColor(id, delta) {
  const rack = COLOR_RACKS.find((r) => preset[r.key].some((s) => s.id === id));
  if (!rack) return;
  const chain = preset[rack.key];
  const index = chain.findIndex((s) => s.id === id),
    target = index + delta;
  if (target < 0 || target >= chain.length) return;
  [chain[index], chain[target]] = [chain[target], chain[index]];
  editColorChain();
}
function buildColors() {
  for (const rack of COLOR_RACKS) {
    window.SadLazySelect.configure(
      $(`${rack.add}-type`),
      COLOR_TYPES[0][0],
      colorTypeGroups(),
    );
    $(rack.add).onclick = () => {
      if (preset[rack.key].length >= (rack.max || colorDefaults.max_slots))
        return;
      const slots = colorEntries().map((e) => e.slot);
      nextColorId = Math.max(nextColorId, ...slots.map((s) => s.id + 1));
      if (nextColorId > 4294967295) nextColorId = 1;
      while (slots.some((s) => s.id === nextColorId)) nextColorId++;
      const slot = structuredClone(colorDefaults.slot);
      slot.id = nextColorId++;
      slot.kind = $(`${rack.add}-type`).value;
      applyKindDefaults(slot);
      preset[rack.key].push(slot);
      editColorChain();
    };
  }
}
function syncColors() {
  if (!preset?.color_chain || !colorDefaults) return;
  preset.pre_color_chain ||= [];
  preset.source_color_chain ||= [];
  const signature = colorEntries()
    .map(
      ({ rack, slot: s }) =>
        `${rack.key}:${s.id}:${s.kind}:${s.settings[`${delayPrefix(s.kind)}_sync`] ?? ""}:${s.kind === "super_echo" ? s.settings.super_echo_sync >= 0.5 : ""}`,
    )
    .join(",");
  if (
    signature !== colorSignature ||
    COLOR_RACKS.some(
      (r) => $(r.element).children.length !== preset[r.key].length,
    )
  ) {
    colorSignature = signature;
    for (const key of controls.keys())
      if (key.startsWith("color:")) {
        controls.delete(key);
        modulationViews.delete(key);
      }
    for (const rack of COLOR_RACKS) $(rack.element).replaceChildren();
    for (const { rack, index, slot } of colorEntries()) {
      const number = index + 1,
        id = slot.id;
      const card = document.createElement("div");
      card.className = "color-card";
      card.dataset.id = id;
      card.innerHTML = `<div class="color-title"><strong class="color-order">${String(number).padStart(2, "0")}</strong><button class="color-type" aria-label="${rack.label} ${number} type"></button><div class="color-actions"><button class="color-bypass" aria-label="Bypass ${rack.label} ${number}" title="Bypass" aria-pressed="false">⏻</button><button class="color-up" aria-label="Move ${rack.label} ${number} up" title="Move up">↑</button><button class="color-down" aria-label="Move ${rack.label} ${number} down" title="Move down">↓</button><button class="color-remove" aria-label="Remove ${rack.label} ${number}" title="Remove">×</button></div></div><div id="color-${id}-knobs" class="knob-grid color-knobs"></div>`;
      const select = card.querySelector(".color-type");
      window.SadLazySelect.configure(select, slot.kind, colorTypeGroups());
      select.onchange = () => {
        const focused = document.activeElement === select;
        const retyped = findColor(id);
        retyped.kind = select.value;
        if (isMooz(retyped.kind)) applyKindDefaults(retyped);
        editColorChain();
        // The card is rebuilt for its new type; keep keyboard focus on it.
        if (focused)
          document.querySelector(`.color-card[data-id="${id}"] .color-type`)?.focus();
      };
      if (slot.kind === "reverb") {
        const title = card.querySelector(".color-title");
        title.classList.add("has-options");
        const mode = document.createElement("select");
        mode.className = "color-option color-reverb-type";
        mode.setAttribute("aria-label", `${rack.label} ${number} reverb type`);
        mode.replaceChildren(
          ...REVERB_TYPES.map(([value, label]) => option(value, label)),
        );
        mode.value = slot.settings.reverb_type;
        mode.onchange = () => {
          findColor(id).settings.reverb_type = mode.value;
          editColorChain();
        };
        title.insertBefore(mode, card.querySelector(".color-actions"));
      }
      if (slot.kind === "waveshaper") {
        const title = card.querySelector(".color-title");
        title.classList.add("has-options");
        const mode = document.createElement("select");
        mode.className = "color-option color-waveshaper-type";
        mode.setAttribute(
          "aria-label",
          `${rack.label} ${number} waveshaper type`,
        );
        mode.replaceChildren(
          ...WAVESHAPER_TYPES.map(([value, label]) => option(value, label)),
        );
        mode.value = slot.settings.waveshaper_type;
        mode.onchange = () => {
          findColor(id).settings.waveshaper_type = mode.value;
          editColorChain();
        };
        title.insertBefore(mode, card.querySelector(".color-actions"));
      }
      if (IMPORTED_FX_CHOICES[slot.kind]) {
        card.classList.add("color-imported");
        const options = document.createElement("div");
        options.className = "color-import-options";
        for (const [parameter, label, values] of IMPORTED_FX_CHOICES[
          slot.kind
        ]) {
          const field = document.createElement("label");
          const caption = document.createElement("span");
          caption.textContent = label;
          const choice = document.createElement("select");
          choice.dataset.parameter = parameter;
          choice.setAttribute(
            "aria-label",
            `${rack.label} ${number} ${label.toLowerCase()}`,
          );
          choice.replaceChildren(
            ...values.map((label, i) => option(String(i), label)),
          );
          choice.value = String(
            Math.round(slot.settings[parameter] * (values.length - 1)),
          );
          choice.onchange = () => {
            const value = Number(choice.value) / (values.length - 1);
            findColor(id).settings[parameter] = value;
            scheduleParameter(
              `${rack.prefix || ""}fx${index + 1}_${parameter}`,
              value,
            );
            if (parameter === "super_echo_sync") syncColors();
          };
          field.append(caption, choice);
          options.append(field);
        }
        card.insertBefore(options, card.querySelector(".color-knobs"));
      }
      const prefix = delayPrefix(slot.kind);
      if (prefix) {
        const title = card.querySelector(".color-title");
        title.classList.add("has-options", "has-delay-options");
        const options = document.createElement("div");
        options.className = "color-delay-options";
        const clock = document.createElement("select");
        clock.className = "color-option color-delay-clock";
        clock.setAttribute("aria-label", `${rack.label} ${number} delay clock`);
        clock.replaceChildren(
          option("free", "Milliseconds"),
          option("sync", "Tempo sync"),
        );
        clock.value = slot.settings[`${prefix}_sync`] ? "sync" : "free";
        clock.onchange = () => {
          const settings = findColor(id).settings;
          const tables = colorDefaults.delay_tables;
          const ratio = prefix === "lcr" ? delayLookup(
            tables[settings.lcr_sync ? "sync_ratio" : "free_ratio"], settings.lcr_ratio, true,
          ) : null;
          settings[`${prefix}_sync`] = clock.value === "sync";
          if (prefix === "lcr") settings.lcr_ratio = delayRatioPosition(
            tables[settings.lcr_sync ? "sync_ratio" : "free_ratio"], ratio,
          );
          editColorChain();
        };
        const smooth = document.createElement("label");
        smooth.className = "check color-delay-smooth";
        smooth.title =
          "Continuous delay times between note divisions, with a smooth pitch glide";
        const checkbox = document.createElement("input");
        checkbox.type = "checkbox";
        checkbox.setAttribute(
          "aria-label",
          `${rack.label} ${number} smooth delay`,
        );
        checkbox.onchange = () => {
          findColor(id).settings[`${prefix}_smooth`] = checkbox.checked;
          editColorChain();
        };
        smooth.append(checkbox, "Smooth");
        options.append(clock, smooth);
        title.insertBefore(options, card.querySelector(".color-actions"));
      }
      card.querySelector(".color-bypass").onclick = () => {
        const s = findColor(id);
        s.enabled = !s.enabled;
        editColorChain();
      };
      if (rack.max === 1) {
        card.querySelector(".color-up").hidden = true;
        card.querySelector(".color-down").hidden = true;
      }
      card.querySelector(".color-up").onclick = () => moveColor(id, -1);
      card.querySelector(".color-down").onclick = () => moveColor(id, 1);
      card.querySelector(".color-remove").onclick = () => removeColor(id);
      const schema = colorSchema(slot.kind, slot);
      card
        .querySelector(".color-knobs")
        .style.setProperty(
          "--color-knobs",
          schema.length === 8 ? 4 : Math.min(7, schema.length),
        );
      card.querySelector(".color-knobs").dataset.count = schema.length;
      $(rack.element).append(card);
      const timbreId = state?.timbres?.active ?? 0;
      effectPresetLibrary?.attach(card, `${rack.label} ${number}`, () =>
        (state?.timbres?.active ?? 0) === timbreId ? findColor(id) : null,
        `${timbreId}:${id}:${slot.kind}`,
      );
      for (const [parameter, label, format] of schema) {
        const key = `color:${id}:${parameter}`;
        const getSlot = () => findColor(id);
        const get = () => {
          const current = getSlot() || colorDefaults.slot;
          return parameter === "mix"
            ? current.mix
            : current.settings[parameter];
        };
        createKnob(
          [
            key,
            label,
            (v) => format(v, getSlot() || colorDefaults.slot),
            0,
            1,
            `${rack.label} ${number} ${label.toLowerCase()}`,
          ],
          `color-${id}-knobs`,
          {
            ...(parameter === "lcr_ratio" ? lcrRatioBinding(getSlot) : {}),
            ...moozBinding(slot.kind, parameter),
            get,
            set: (value) => {
              const s = getSlot();
              if (parameter === "mix") s.mix = value;
              else s.settings[parameter] = value;
              colorLegacy();
              controls.get(key)?.();
              if (parameter.endsWith("_ratio")) syncColors();
              scheduleParameter(
                `${rack.prefix || ""}fx${index + 1}_${parameter}`,
                value,
              );
            },
            reset: () =>
              moozParam(slot.kind, parameter)
                ? moozBinding(slot.kind, parameter).reset()
                : parameter === "mix"
                ? slot.kind === "compressor" || isMooz(slot.kind)
                  ? 1
                  : colorDefaults.slot.mix
                : colorDefaults.slot.settings[parameter],
          },
        );
      }
      if (slot.kind === "compressor") {
        const meter = document.createElement("div");
        meter.className = "gain-reduction";
        meter.dataset.slot = id;
        meter.setAttribute("role", "meter");
        meter.setAttribute(
          "aria-label",
          `${rack.label} ${number} gain reduction`,
        );
        meter.setAttribute("aria-valuemin", "0");
        meter.setAttribute("aria-valuemax", "30");
        meter.setAttribute("aria-valuenow", "0");
        meter.setAttribute("aria-valuetext", "0.0 dB gain reduction");
        meter.title = "Compression before Makeup and Mix";
        meter.innerHTML = `<div class="gr-heading"><span>GAIN REDUCTION</span><output>0.0 dB</output></div><div class="gr-track" aria-hidden="true"><span class="gr-fill"></span></div><div class="gr-scale" aria-hidden="true"><span>0</span><span>6</span><span>12</span><span>18</span><span>24</span><span>30 dB</span></div>`;
        card.append(meter);
      }
    }
    colorRoutesSignature = "";
  }
  const routesSignature = JSON.stringify([
    preset.lfo_count,
    preset.env_count,
    !!preset.drums,
    !!preset.synth,
    preset.lfos.map((l) => LFO_ROUTES.map(([field]) => l[field])),
    preset.envs.map((e) => ENV_ROUTES.map(([field]) => e[field])),
    (preset.macros || []).map((m) => m.targets.map((t) => t.destination)),
  ]);
  if (routesSignature !== colorRoutesSignature) {
    colorRoutesSignature = routesSignature;
    refreshColorDestinations();
  }
  for (const { rack, index, slot } of colorEntries()) {
    const card = $(rack.element).children[index];
    const mode = card.querySelector(".color-reverb-type");
    if (mode && document.activeElement !== mode)
      mode.value = slot.settings.reverb_type;
    const shaper = card.querySelector(".color-waveshaper-type");
    if (shaper && document.activeElement !== shaper)
      shaper.value = slot.settings.waveshaper_type;
    for (const [parameter, , values] of IMPORTED_FX_CHOICES[slot.kind] || []) {
      const choice = card.querySelector(`[data-parameter="${parameter}"]`);
      if (choice && document.activeElement !== choice)
        choice.value = String(
          Math.round(slot.settings[parameter] * (values.length - 1)),
        );
    }
    const prefix = delayPrefix(slot.kind);
    if (prefix) {
      const smooth = Boolean(slot.settings[`${prefix}_smooth`]);
      card.querySelector(".color-delay-smooth input").checked = smooth;
      card.classList.toggle(
        "has-smooth-delay",
        smooth && slot.settings[`${prefix}_sync`],
      );
    }
    card.classList.toggle("is-bypassed", !slot.enabled);
    card
      .querySelector(".color-bypass")
      .setAttribute("aria-pressed", String(!slot.enabled));
    card.querySelector(".color-up").disabled = index === 0;
    card.querySelector(".color-down").disabled =
      index === preset[rack.key].length - 1;
  }
  for (const rack of COLOR_RACKS)
    $(rack.add).disabled =
      preset[rack.key].length >= (rack.max || colorDefaults.max_slots);
  for (const [key, update] of controls) if (key.startsWith("color:")) update();
  effectPresetLibrary?.sync();
  syncGainReduction(state);
}
function syncGainReduction(next) {
  $("source-fx-status").textContent =
    next?.source_fx?.error || (next?.source_fx?.pending ? "Preparing…" : "");
  for (const meter of document.querySelectorAll(
    ".color-chain .gain-reduction",
  )) {
    const slot = findColor(Number(meter.dataset.slot));
    meter.hidden =
      preset?.source_color_chain?.some(
        (s) => s.id === Number(meter.dataset.slot),
      ) && !next?.input?.active;
    const raw = Number(next?.gain_reduction?.[meter.dataset.slot] ?? 0);
    const value =
      next?.playing &&
      !next.bypass &&
      slot?.enabled &&
      slot.mix > 0 &&
      Number.isFinite(raw)
        ? Math.max(0, raw)
        : 0;
    const rounded = Math.round(value * 10) / 10;
    const text = `${rounded > 0 ? "−" : ""}${rounded.toFixed(1)} dB`;
    meter.querySelector("output").textContent = text;
    meter.querySelector(".gr-fill").style.width =
      `${(Math.min(30, value) / 30) * 100}%`;
    meter.setAttribute("aria-valuenow", String(Math.min(30, rounded)));
    meter.setAttribute(
      "aria-valuetext",
      `${rounded.toFixed(1)} dB gain reduction`,
    );
  }
}
// Destination menus: sections, then a submenu per filter, LFO, envelope,
// effect slot, oscillator, drum voice or sequencer step.
// Pickers filled together share one set of groups.
function fillDestination(select, destination, shared = null) {
  const groups = [...(shared ?? destinationGroups())],
    value = destinationValue(destination),
    listed = (choices) => choices.some((c) => (Array.isArray(c) ? c[0] === value : listed(c.choices)));
  if (typeof destination !== "string" && !listed(groups)) {
    if (destination.filter_slot) {
      const index = (preset.filter_slots || []).findIndex(s => s.id === destination.filter_slot.slot);
      const label = destination.filter_slot.parameter.replaceAll("_", " ");
      groups.push({ choices: [[value, `${index >= 0 ? `Filter ${index + 3}` : "Filter"} · ${label} (inactive)`]] });
    } else {
      const entry = colorEntries().find(e => e.slot.id === destination.color?.slot);
      const label = destination.color?.parameter.replaceAll("_", " ") || "Unassigned";
      groups.push({ choices: [[value, `${entry ? `${entry.rack.label} ${entry.index + 1}` : "FX"} · ${label} (inactive)`]] });
    }
  }
  window.SadLazySelect.configure(select, value, groups);
}
function destinationGroups() {
  const section = window.SadLazySelect.section,
    sentence = (label) => label.charAt(0) + label.slice(1).toLowerCase(),
    named = new Map(modulationGroups().map(([label, choices]) => [
      label,
      choices.map(([value, text]) => [destinationValue(value), text]),
    ])),
    take = (label) => {
      const choices = named.get(label) || [];
      named.delete(label);
      return choices;
    },
    pull = (pattern) =>
      [...named.keys()].filter((label) => pattern.test(label)).map((label) => [label, take(label)]),
    routes = [...DESTINATIONS.filter(([id]) => id.startsWith("filter")), ...take("Filter controls")],
    filters = [1, 2].map((n) =>
      section(`Filter ${n}`, routes.filter(([, text]) => text.startsWith(`Filter ${n} `))),
    ),
    steps = take("Sequencer steps");
  for (const [index, slot] of (preset.filter_slots || []).entries())
    filters.push(section(`Filter ${index + 3}`, FILTER_SLOT_CONTROLS.map(([parameter, label]) => [
      destinationValue({ filter_slot: { slot: slot.id, parameter } }),
      `Filter ${index + 3} · ${sentence(label)}`,
    ])));
  const effects = (key) =>
    colorEntries().filter((e) => e.rack.key === key).map(({ rack, index, slot }) => section(
      `${rack.label} ${index + 1} · ${COLOR_TYPES.find(([k]) => k === slot.kind)[1]}`,
      colorDestinationSchema(slot.kind).map(([parameter, label]) => [
        destinationValue({ color: { slot: slot.id, parameter } }),
        `${rack.label} ${index + 1} · ${sentence(label)}`,
      ]),
      `${rack.label} ${index + 1}`,
    ));
  const groups = [
    { choices: [DESTINATIONS[0]] },
    { label: "Main", choices: DESTINATIONS.slice(1).filter(([id]) => !id.startsWith("filter") && !GRAIN_DESTINATIONS.has(id)) },
    // The grain engine's controls, then each advanced mode group's.
    { label: "Grains", choices: [
      ...DESTINATIONS.filter(([id]) => GRAIN_DESTINATIONS.has(id)),
      ...take("Grains"),
      ...pull(/^Grain group [A-D]$/).map(([label, choices]) => section(label, choices)),
    ] },
    { label: "Filters", choices: filters },
    { label: "LFO", choices: pull(/^LFO \d+$/).map(([label, choices]) => section(label, choices)) },
    { label: "Envelopes", choices: pull(/^ENV \d+$/).map(([label, choices]) => section(label, choices)) },
    { label: "Pre-filter FX", choices: effects("pre_color_chain") },
    { label: "Post-filter FX", choices: effects("color_chain") },
    section("Resynth", [
      ...take("Resynth"),
      ...pull(/^OSC \d+$/).map(([label, choices]) => section(label, choices, `Resynth ${label}`)),
    ]),
    { label: "Synth engine", choices: [
      ...take("Synth engine"),
      ...pull(/^Synth OSC \d+$/).map(([label, choices]) => section(label, choices)),
    ] },
    ...drumDestinations(pull(/^Drum \d+$/)),
    { label: "Performance", choices: take("Performance") },
    { label: "Sequencer steps", choices: Array.from({ length: 16 }, (_, i) =>
      section(`Step ${i + 1}`, steps.filter(([, text]) => text.startsWith(`Step ${i + 1} ·`)))) },
    { label: "Global controls", choices: take("Global controls") },
    ...[...named].map(([label, choices]) => ({ label, choices })),
  ];
  return groups;
}
// A knob's or slider's control key and the route destination that turns it,
// so its menu can route a source to it.
const PARAMETER_KEYS = {
  tempo: "bpm",
  input_gain: "in_gain",
  output_gain: "out_gain",
  pitch_bend_range: "midi_pitchbend_range",
};
function destinationKey(destination) {
  if (!destination || destination === "none") return null;
  if (typeof destination === "string")
    return destination === "volume" ? "master_volume" : destination;
  const [[kind, d]] = Object.entries(destination);
  if (kind === "parameter") return PARAMETER_KEYS[d.parameter] ?? d.parameter;
  if (kind === "color" || kind === "filter_slot") return `${kind}:${d.slot}:${d.parameter}`;
  if (kind === "lfo" || kind === "env") return `${kind}${d.index + 1}_${d.parameter}`;
  if (kind === "performance") return d.parameter;
  if (kind === "control")
    return sequenceCatalog?.modulation?.find((s) => s.index === d.index)?.id ?? null;
  return null;
}
/** The destination the menus list for this control key; null when none does. */
function knobDestination(key) {
  const flat = (choices) => choices.flatMap((c) => (Array.isArray(c) ? [c] : flat(c.choices))),
    choice = flat(destinationGroups()).find(
      ([value]) => value !== "none" && destinationKey(parseDestination(value)) === key,
    );
  return choice ? parseDestination(choice[0]) : null;
}
/** The sequencer lane parameter for this control key, if a lane can play it:
 * effects and added filters go by position there. */
function sequencerParameter(key) {
  const color = /^color:(\d+):(.+)$/.exec(key),
    filter = /^filter_slot:(\d+):(.+)$/.exec(key);
  let id = key;
  if (color) {
    const entry = colorEntries().find((e) => e.slot.id === Number(color[1]));
    id = entry?.rack.key !== "source_color_chain" && entry &&
      `${entry.rack.key === "pre_color_chain" ? "pre_" : ""}fx${entry.index + 1}_${color[2]}`;
  } else if (filter) {
    const index = (preset.filter_slots || []).findIndex((s) => s.id === Number(filter[1]));
    id = index >= 0 && `filter${index + 3}_${filter[2]}`;
  }
  return id && sequenceCatalog?.parameters?.some((p) => p.id === id) ? id : null;
}
// Each drum voice is a top-level section, its controls grouped by module like
// the voice editor: "Snare → Filter 1 → Cutoff". Drum voices go by their kit
// names ("Closed HH · Decay").
function drumDestinations(voices) {
  const section = window.SadLazySelect.section,
    sections = window.SadDrums?.controlSections;
  if (!voices.length) return [];
  if (!sections)
    return [{ label: "Drums", choices: voices.map(([, choices]) =>
      section(choices[0][1].split(" · ")[0], choices)) }];
  const specs = new Map((sequenceCatalog?.modulation || []).map((spec) => [spec.id, spec]));
  const control = (id, name, label) => {
    const spec = specs.get(id);
    return spec ? [[destinationValue({ control: { index: spec.index } }), name, label]] : [];
  };
  // Every track shifts its pattern too; Open HH plays the Closed HH voice.
  const voiceCount = window.SadDrums.voiceNames.length;
  return window.SadDrums.trackNames.map((track, t) => ({
    label: track,
    choices: [
      ...(t < voiceCount ? sections(t) : []).map(([module, controls]) => section(module,
        controls.flatMap(([id, name]) => control(`drum${t + 1}_${id}`, name, `${track} · ${name}`)))),
      section("Pattern", control(`drum${t + 1}_note_shift`, "Note shift", `${track} · Pattern note shift`)),
    ],
  }));
}
function refreshColorDestinations() {
  const groups = destinationGroups();
  for (const [i, scope] of scopes.entries())
    for (const route of scope.routes)
      fillDestination(route.select, preset.lfos[i][route.destination] ?? "none", groups);
  for (const [i, card] of envCards.entries())
    for (const route of card.routes)
      fillDestination(route.select, preset.envs[i][route.destination] ?? "none", groups);
  for (const [i, row] of matrixRows.entries())
    fillDestination(
      row.querySelector(".matrix-destination"),
      preset.matrix[i].destination,
      groups,
    );
  window.SadModulation?.fill(groups);
}
function modulationGroups() {
  const parameter = (key, label) => [{ parameter: { parameter: key } }, label];
  const groups = [
    [
      "Filter controls",
      [
        parameter("filter_env_depth", "Filter 1 ENV depth"),
        parameter("filter_keytrack", "Filter 1 keytrack"),
        parameter("filter2_env_depth", "Filter 2 ENV depth"),
        parameter("filter2_keytrack", "Filter 2 keytrack"),
        parameter("filter_drive", "Filter 1 drive"),
        parameter("filter_morph", "Filter 1 morph"),
        parameter("filter2_drive", "Filter 2 drive"),
        parameter("filter2_morph", "Filter 2 morph"),
      ],
    ],
  ];
  for (let index = 0; index < 8; index++) {
    if (index < (preset.lfo_count || 3))
      groups.push([
        `LFO ${index + 1}`,
        [
          ["rate", "Rate"],
          ["depth", "Route 1 depth"],
          ["depth2", "Route 2 depth"],
          ["depth3", "Route 3 depth"],
          ["division", "Sync division"],
        ].map(([key, label]) => [
          { lfo: { index, parameter: key } },
          `LFO ${index + 1} · ${label}`,
        ]),
      ]);
    if (index < (preset.env_count || 3))
      groups.push([
        `ENV ${index + 1}`,
        [
          ["attack", "Attack"],
          ["decay", "Decay"],
          ["sustain", "Sustain"],
          ["release", "Release"],
          ["depth", "Route 1 depth"],
          ["extra_depth", "Route 2 depth"],
          ["depth3", "Route 3 depth"],
        ]
          .map(([key, label]) => [
            { env: { index, parameter: key } },
            `ENV ${index + 1} · ${label}`,
          ])
          .concat(
            // Rustias type controls are numeric host parameters.
            [
              ["attack_level", "Attack level"],
              ["decay2", "Decay 2"],
              ["sustain2", "Sustain 2"],
            ].flatMap(([key, label]) => {
              const spec = sequenceCatalog?.modulation?.find(
                (s) => s.id === `env${index + 1}_${key}`,
              );
              return spec
                ? [
                    [
                      { control: { index: spec.index } },
                      `ENV ${index + 1} · ${label}`,
                    ],
                  ]
                : [];
            }),
          ),
      ]);
  }
  groups.push([
    "Performance",
    [
      ["arp_rate", "Arp rate"],
      ["arp_gate", "Arp gate"],
      ["arp_octaves", "Arp octaves"],
      ["sequence_rate", "Sequence rate"],
      ["sequence_gate", "Sequence gate"],
      ["sequence_length", "Sequence length"],
    ].map(([key, label]) => [{ performance: { parameter: key } }, label]),
  ]);
  groups.push([
    "Sequencer steps",
    Array.from({ length: 16 }, (_, index) =>
      ["note", "velocity"].map((key) => [
        { step: { index, parameter: key } },
        `Step ${index + 1} · ${key}`,
      ]),
    ).flat(),
  ]);
  groups.push([
    "Global controls",
    [
      parameter("tempo", "Tempo"),
      parameter("input_gain", "Input gain"),
      parameter("output_gain", "Output gain"),
      parameter("pitch_range", "Pitch range"),
      parameter("sample_tune_semis", "Sample tune · semitones"),
      parameter("sample_tune_cents", "Sample tune · cents"),
      parameter("pitch_bend_range", "Pitch bend range"),
      parameter("min_grains", "Minimum grains"),
    ],
  ]);
  const extra = new Map();
  for (const spec of sequenceCatalog?.modulation || []) {
    if (spec.id.startsWith("drum") && !preset.drums) continue;
    if (spec.id.startsWith("synth_") && !preset.synth) continue;
    // Added filters have their own groups, addressed by slot id.
    if (
      /^(pre_)?fx\d+_/.test(spec.id) ||
      /^(lfo|env)\d+_/.test(spec.id) ||
      /^filter([3-9]|1\d)_/.test(spec.id)
    )
      continue;
    // Existing named routes keep their serialized identities. The catalog adds
    // oscillator controls and any numeric control absent from those groups.
    if (
      DESTINATIONS.some(([id]) => id === spec.id) ||
      [
        "master_volume",
        "sample_tune_semis",
        "sample_tune_cents",
        "in_gain",
        "out_gain",
        "pitch_range",
        "midi_pitchbend_range",
        "min_grains",
        "arp_rate",
        "arp_gate",
        "arp_octaves",
        "sequence_rate",
        "sequence_gate",
        "sequence_length",
        "filter_env_depth",
        "filter_keytrack",
        "filter2_env_depth",
        "filter2_keytrack",
        "filter_drive",
        "filter_morph",
        "filter2_drive",
        "filter2_morph",
      ].includes(spec.id)
    )
      continue;
    const osc = /^resynth_osc(\d+)_/.exec(spec.id);
    const drum = /^drum(\d+)_/.exec(spec.id);
    const synthOsc = /^synth_osc(\d+)_/.exec(spec.id);
    const grainGroup = /^grain_group(\d)_/.exec(spec.id);
    if (grainGroup || ["grain_advanced", "grain_feedback"].includes(spec.id)) {
      const label = grainGroup
        ? `Grain group ${GRAIN_GROUP_NAMES[grainGroup[1] - 1]}`
        : "Grains";
      if (!extra.has(label)) extra.set(label, []);
      extra.get(label).push([
        { control: { index: spec.index } },
        grainGroup
          ? spec.name.replace(/^Grain group ([A-D]) (.)/, (_, g, c) => `Group ${g} · ${c.toUpperCase()}`)
          : spec.id === "grain_advanced" ? "Advanced mode" : "Feedback",
      ]);
      continue;
    }
    const group = drum
      ? `Drum ${drum[1]}`
      : osc
        ? `OSC ${osc[1]}`
        : synthOsc
          ? `Synth OSC ${synthOsc[1]}`
          : spec.id.startsWith("resynth_")
            ? "Resynth"
            : spec.id.startsWith("synth_")
              ? "Synth engine"
              : "Other controls";
    if (!extra.has(group)) extra.set(group, []);
    extra.get(group).push([{ control: { index: spec.index } }, spec.name]);
  }
  groups.push(...extra);
  return groups;
}
const MOD_SOURCES = [
  ["lfo1", "LFO 1"],
  ["lfo2", "LFO 2"],
  ["lfo3", "LFO 3"],
  ...Array.from({ length: 5 }, (_, i) => [`lfo${i + 4}`, `LFO ${i + 4}`]),
  ["env1", "ENV 1 · Amp"],
  ["env2", "ENV 2"],
  ["env3", "ENV 3"],
  ...Array.from({ length: 5 }, (_, i) => [`env${i + 4}`, `ENV ${i + 4}`]),
  ["velocity", "Velocity"],
  ["key_track", "Key track"],
  ["mod_wheel", "Mod wheel · CC 1"],
  ["pitch_bend", "Pitch bend"],
  ["aftertouch", "Aftertouch"],
  ["expression", "Expression · CC 11"],
  ["random_note", "Random / note"],
  ["gate", "Note gate"],
  ["follower", "Input follower"],
];
let matrixRows = [],
  matrixSignature = -1;
function buildMatrix() {
  $("matrix-add").onclick = () => {
    if (preset.matrix.length >= 8) return;
    preset.matrix.push({
      source: "lfo1",
      destination: "none",
      depth: 0.25,
      enabled: true,
    });
    syncMatrix();
    schedulePreset();
  };
}
function syncMatrix() {
  if (!preset?.matrix) return;
  if (matrixSignature !== preset.matrix.length) {
    matrixSignature = preset.matrix.length;
    $("matrix").replaceChildren();
    matrixRows = preset.matrix.map((route, i) => {
      const row = document.createElement("div");
      row.className = "matrix-row";
      const number = i + 1;
      row.innerHTML = `<strong class="matrix-order">${String(number).padStart(2, "0")}<i class="matrix-signal" aria-hidden="true"></i></strong><select class="matrix-source" aria-label="Modulation ${number} source"></select><button class="matrix-destination" aria-label="Modulation ${number} destination"></button><label class="matrix-depth"><input type="range" min="-1" max="1" step="0.005" aria-label="Modulation ${number} depth"><output></output></label><div class="matrix-actions"><button class="matrix-bypass" title="Bypass" aria-label="Bypass modulation ${number}" aria-pressed="false">⏻</button><button class="matrix-remove" title="Remove" aria-label="Remove modulation ${number}">×</button></div>`;
      const source = row.querySelector(".matrix-source"),
        dest = row.querySelector(".matrix-destination"),
        depth = row.querySelector("input");
      source.replaceChildren(
        ...MOD_SOURCES.map(([value, label]) => option(value, label)),
      );
      source.onchange = () => {
        preset.matrix[i].source = source.value;
        schedulePreset();
      };
      dest.onchange = () => {
        preset.matrix[i].destination = parseDestination(dest.value);
        schedulePreset();
      };
      depth.oninput = () => {
        preset.matrix[i].depth = Number(depth.value);
        row.querySelector("output").textContent = depthText(
          preset.matrix[i].destination,
          preset.matrix[i].depth,
        );
        scheduleParameter(`matrix${i + 1}_depth`, preset.matrix[i].depth);
      };
      row.querySelector(".matrix-bypass").onclick = () => {
        preset.matrix[i].enabled = !preset.matrix[i].enabled;
        syncMatrix();
        schedulePreset();
      };
      row.querySelector(".matrix-remove").onclick = () => {
        preset.matrix.splice(i, 1);
        matrixSignature = -1;
        syncMatrix();
        schedulePreset();
      };
      $("matrix").append(row);
      return row;
    });
  }
  for (const [i, row] of matrixRows.entries()) {
    const route = preset.matrix[i],
      source = row.querySelector(".matrix-source"),
      destination = row.querySelector(".matrix-destination"),
      depth = row.querySelector("input");
    if (document.activeElement !== source) source.value = route.source;
    if (document.activeElement !== destination)
      fillDestination(destination, route.destination);
    if (document.activeElement !== depth) depth.value = route.depth;
    row.querySelector("output").textContent = depthText(route.destination, route.depth);
    row
      .querySelector(".matrix-bypass")
      .setAttribute("aria-pressed", String(!route.enabled));
    row.classList.toggle("is-bypassed", !route.enabled);
  }
  $("matrix-count").textContent = `${preset.matrix.length} / 8`;
  $("matrix-add").disabled = preset.matrix.length >= 8;
}
const WAVEFORMS = [
  ["sine", "Sine"],
  ["triangle", "Triangle"],
  ["saw", "Saw"],
  ["square", "Square"],
  ["random", "Random"],
  ["smooth_random", "Smooth random"],
  ["envelope", "Env. follower"],
];
// Route destinations the grain engine owns, listed under Grains.
const GRAIN_DESTINATIONS = new Set([
  "position",
  "pitch",
  "grain_size",
  "grains",
  "grain_shape",
  "detune",
  "cloud",
  "stereo_spread",
]);
const DESTINATIONS = [
  ["none", "Unassigned"],
  ["position", "Position"],
  ["pitch", "Pitch"],
  ["grain_size", "Grain size"],
  ["grains", "Density"],
  ["filter", "Filter 1 cutoff"],
  ["mix", "Dry / wet"],
  ["pan", "Stereo pan"],
  ["reverb", "First reverb mix"],
  ["grain_shape", "Grain shape"],
  ["detune", "Detune"],
  ["cloud", "Cloud"],
  ["stereo_spread", "Stereo spread"],
  ["filter_resonance", "Filter 1 resonance"],
  ["filter2", "Filter 2 cutoff"],
  ["filter2_resonance", "Filter 2 resonance"],
  ["saturation", "First FX mix"],
  ["reverb_decay", "First reverb decay"],
  ["reverb_tone", "First reverb tone"],
  ["predelay", "First reverb predelay"],
  ["shimmer", "First reverb shimmer"],
  ["diffuse", "First reverb diffuse"],
  ["volume", "Volume"],
];
function removeModulator(kind) {
  const bank = kind === "lfo" ? "lfos" : "envs";
  const countKey = `${kind}_count`;
  const count = preset[countKey] || 3;
  if (count <= 3) return;
  preset[bank][count - 1] = structuredClone(presets[0][bank][count - 1]);
  preset[countKey] = count - 1;
  preset.matrix = preset.matrix.filter((route) => route.source !== `${kind}${count}`);
  if (kind === "lfo") scopes[count - 1].values = [];
  syncLfos();
  syncEnvs();
  syncMatrix();
  refreshColorDestinations();
  schedulePreset();
}
function buildLfos() {
  $("lfo-remove").onclick = () => removeModulator("lfo");
  $("lfo-add").onclick = () => {
    if ((preset.lfo_count || 3) >= 8) return;
    preset.lfo_count = (preset.lfo_count || 3) + 1;
    syncLfos();
    refreshColorDestinations();
    schedulePreset();
  };
  for (let i = 0; i < 8; i++) {
    const card = document.createElement("div");
    card.className = "lfo-card";
    card.innerHTML = `<div class="lfo-title"><strong>LFO ${i + 1}</strong><canvas class="lfo-scope" width="144" height="40"></canvas></div><div class="lfo-row"><select aria-label="LFO ${i + 1} waveform" class="waveform"></select><label>RATE<input aria-label="LFO ${i + 1} rate" class="rate" type="number" min="0.01" max="40" step="0.01"></label></div><div class="lfo-row"><label class="check"><input type="checkbox" class="sync">Tempo sync</label><select class="division" aria-label="LFO ${i + 1} tempo division"><option value="0.25">1/16</option><option value="0.5">1/8</option><option value="1">1/4</option><option value="2">1/2</option><option value="4">1 bar</option><option value="8">2 bars</option><option value="16">4 bars</option><option value="32">8 bars</option><option value="64">16 bars</option><option value="128">32 bars</option></select></div>`;
    for (const [value, label] of WAVEFORMS)
      card.querySelector(".waveform").append(option(value, label));
    for (const [field, selector] of [
      ["waveform", ".waveform"],
      ["rate_hz", ".rate"],
      ["sync", ".sync"],
      ["division", ".division"],
    ]) {
      const input = card.querySelector(selector);
      input.addEventListener("input", () => {
        let value = input.type === "checkbox" ? input.checked : input.value;
        if (["rate_hz", "division"].includes(field)) {
          value = Number(value);
          if (!Number.isFinite(value)) return;
        }
        preset.lfos[i][field] = value;
        if (field === "sync") syncLfos();
        if (typeof value === "number")
          scheduleParameter(
            `lfo${i + 1}_${field === "rate_hz" ? "rate" : field}`,
            value,
          );
        else schedulePreset();
      });
    }
    const routes = modulatorRoutes(card, "lfo", i, LFO_ROUTES, syncLfos);
    $("lfos").append(card);
    scopes.push({ card, canvas: card.querySelector("canvas"), values: [], routes });
  }
}
// Each LFO and ENV routes to three destinations of its own: a picker and a
// depth slider each, under the host IDs of their depths.
const LFO_ROUTES = [
  ["destination", "depth"],
  ["destination2", "depth2"],
  ["destination3", "depth3"],
];
const ENV_ROUTES = [
  ["destination", "depth"],
  ["extra_destination", "extra_depth"],
  ["destination3", "depth3"],
];
function modulatorRoutes(card, kind, i, fields, sync) {
  const name = `${kind.toUpperCase()} ${i + 1}`,
    bank = kind === "lfo" ? "lfos" : "envs";
  return fields.map(([destination, depth], j) => {
    const row = document.createElement("div");
    row.className = "env-route";
    row.innerHTML = `<button aria-label="${name} destination ${j + 1}"></button><label><input type="range" min="-1" max="1" step="0.01" aria-label="${name} depth ${j + 1}"><output></output></label>`;
    const select = row.querySelector("button"),
      input = row.querySelector("input"),
      id = `${kind}${i + 1}_${depth}`;
    select.oninput = () => {
      preset[bank][i][destination] = parseDestination(select.value);
      sync();
      schedulePreset();
    };
    input.oninput = () => {
      preset[bank][i][depth] = Number(input.value);
      sync();
      scheduleParameter(id, preset[bank][i][depth]);
    };
    sliderMenu(input, id, `${name} depth ${j + 1}`);
    card.append(row);
    return { select, input, output: row.querySelector("output"), destination, depth };
  });
}
function syncModulatorRoutes(routes, holder) {
  for (const route of routes) {
    const destination = holder[route.destination] ?? "none",
      depth = holder[route.depth] ?? 0;
    if (document.activeElement !== route.select)
      window.SadLazySelect.setValue(route.select, destinationValue(destination));
    if (document.activeElement !== route.input) route.input.value = depth;
    route.output.textContent = depthText(destination, depth);
    route.select.closest(".env-route").classList.toggle("is-unrouted", destination === "none");
  }
}
// A slider's right-click menu: MIDI learn and modulation, like a knob's.
function sliderMenu(input, key, label) {
  input.dataset.controlKey = key;
  window.SadMidiLearn?.attach(input, key, label, (unit) => {
    if (!preset || input.disabled) return;
    const min = Number(input.min),
      max = Number(input.max),
      step = Number(input.step) || 0,
      raw = min + unit * (max - min),
      value = step ? min + Math.round((raw - min) / step) * step : raw;
    input.value = String(Math.max(min, Math.min(max, value)));
    input.dispatchEvent(new Event("input", { bubbles: true }));
    input.dispatchEvent(new Event("change", { bubbles: true }));
  });
}
function syncLfos() {
  const count = preset.lfo_count || 3;
  $("lfo-count").textContent = `${count} / 8`;
  $("lfo-add").disabled = count >= 8;
  $("lfo-remove").hidden = count <= 3;
  $("lfo-remove").setAttribute("aria-label", `Remove LFO ${count}`);
  for (let i = 0; i < scopes.length; i++) {
    const card = scopes[i].card,
      l = preset.lfos[i];
    card.hidden = i >= count;
    if (!l) continue;
    for (const [field, selector] of [
      ["waveform", ".waveform"],
      ["rate_hz", ".rate"],
      ["division", ".division"],
    ]) {
      const input = card.querySelector(selector);
      if (document.activeElement !== input) input.value = l[field];
    }
    syncModulatorRoutes(scopes[i].routes, l);
    card.querySelector(".sync").checked = l.sync;
    card.querySelector(".rate").disabled = l.sync;
    card.querySelector(".division").disabled = !l.sync;
  }
}
const envCards = [];
const ENV_FIELDS = [
  "attack",
  "decay",
  "sustain",
  "release",
  "attack_level",
  "decay2",
  "sustain2",
];
const ENV_LEVELS = ["sustain", "attack_level", "sustain2"];
const ENV_EXTENDED = ["attack_level", "decay2", "sustain2"];
// Host values index these; the order matches preset::EnvMode::ALL.
const ENV_MODES = [
  ["legacy", "SAD PET"],
  ["radias", "RADIAS"],
  ["q_adsr", "Q · ADSR"],
  ["q_ads1ds2r", "Q · ADS1DS2R"],
  ["q_one_shot", "Q · One Shot"],
  ["q_loop_s1s2", "Q · Loop S1S2"],
  ["q_loop_all", "Q · Loop All"],
];
// Rustias firmware-free RADIAS decay/release curves 0..7.
const ENV_CURVES = [
  "Cubic",
  "Square",
  "S-curve",
  "Linear",
  "Root",
  "Linear 2",
  "Inverse cubic",
  "Power 1.5",
];
const envCurve = (curve, t) =>
  [t ** 3, t * t, t * t * (3 - 2 * t), t, Math.sqrt(t), t, 1 - (1 - t) ** 3, t ** 1.5][curve] ?? t * t;
const envMode = (env) => env.mode ?? "legacy";
const envExtended = (env) =>
  ["q_ads1ds2r", "q_loop_s1s2", "q_loop_all"].includes(envMode(env));
// Rustias EG types read 0..127 controls.
const envControl = (value) => Math.round(Math.min(1, Math.max(0, value)) * 127);
function envSeconds(env, field) {
  const mode = envMode(env);
  if (mode === "legacy")
    return (
      (field === "attack" ? 0.002 : 0.005) +
      env[field] ** 3 * (field === "release" ? 12 : 8)
    );
  const value = envControl(env[field]);
  if (mode === "radias") return 0.003 * 2000 ** (value / 127);
  // Rustias q_envelope TIME_TICKS at 2 kHz: zero is instant, 127 is 60 s.
  return value === 0
    ? 0
    : Math.max(1, Math.round(120000 ** ((value - 1) / 126))) / 2000;
}
const envLevel = (env, field) =>
  envMode(env) === "legacy" ? env[field] : envControl(env[field]) / 127;
const envTime = (seconds) =>
  seconds < 1 ? `${Math.round(seconds * 1000)} ms` : `${seconds.toFixed(2)} s`;
const signedPercent = (value) =>
  `${value > 0 ? "+" : ""}${Math.round(value * 100)}%`;
// Each knob's display, so modulation depths read in their target's units.
const knobUnits = new Map();
// Ranges the engine scales Parameter routes by (matrix.rs route ranges),
// with the knobs that show them.
const PARAMETER_ROUTES = {
  filter_env_depth: [-1, 1], filter_keytrack: [0, 1], filter2_env_depth: [-1, 1],
  filter2_keytrack: [0, 1], filter_drive: [0, 1], filter_morph: [0, 1],
  filter2_drive: [0, 1], filter2_morph: [0, 1], sample_tune_semis: [-48, 48],
  sample_tune_cents: [-100, 100],
};
/** The display of a route's target: its format, the range a depth of 1
 * spans and the current value; null when it has no knob. */
function destinationUnit(destination) {
  if (!destination || destination === "none") return null;
  if (typeof destination === "string")
    // Main destinations add the depth to a 0…1 control.
    return knobUnits.get(destination === "volume" ? "master_volume" : destination) ?? null;
  if (destination.parameter) {
    const key = destination.parameter.parameter,
      unit = knobUnits.get(key);
    return unit && PARAMETER_ROUTES[key]
      ? { ...unit, min: PARAMETER_ROUTES[key][0], max: PARAMETER_ROUTES[key][1] }
      : null;
  }
  if (destination.color)
    return knobUnits.get(`color:${destination.color.slot}:${destination.color.parameter}`) ?? null;
  if (destination.control) {
    const id = sequenceCatalog?.modulation?.find((s) => s.index === destination.control.index)?.id;
    return id ? parameterUnit(id) : null;
  }
  return null;
}
/** A catalog control's display; drum voices not on screen ask the editor. */
function parameterUnit(id) {
  const spec = sequenceCatalog?.parameters?.find((p) => p.id === id),
    drum = /^drum(\d+)_(\d+)$/.exec(id),
    shift = /^drum(\d+)_note_shift$/.exec(id),
    unit = knobUnits.get(id) ??
      (drum && window.SadDrums?.controlUnit?.(Number(drum[2]),
        () => preset.drums?.params?.[drum[1] - 1]?.[drum[2]] ?? 0)) ??
      (shift && { format: (v) => `${v > 0 ? "+" : ""}${Math.round(v)} steps`,
        get: () => preset.drums?.tracks?.[shift[1] - 1]?.shift ?? 0 });
  return spec && unit ? { ...unit, min: spec.min, max: spec.max } : null;
}
// "+12.0 st" from a format like "-24.0 st": a display linear in the value
// reads as a signed amount; any other shows where full depth takes it.
const depthText = (destination, depth) => unitDepth(destinationUnit(destination), depth);
function unitDepth(unit, depth) {
  if (!unit) return signedPercent(depth);
  const { format, min, max, get } = unit,
    delta = depth * (max - min),
    read = (v) => {
      const m = /^([+\-−]?\d[\d,]*(?:\.(\d+))?)\s*(.*)$/.exec(String(format(v)).trim());
      return m && { n: Number(m[1].replace(/,/g, "").replace("−", "-")), digits: m[2]?.length ?? 0, unit: m[3] };
    },
    [low, mid, high] = [min, (min + max) / 2, max].map(read);
  if (low && mid && high && low.unit === high.unit && mid.unit === low.unit &&
      Math.abs(mid.n - (low.n + high.n) / 2) <= Math.abs(high.n - low.n) * 0.01) {
    const amount = ((high.n - low.n) / (max - min)) * delta,
      text = Math.abs(amount).toFixed(low.digits);
    return `${Number(text) === 0 ? "" : amount > 0 ? "+" : "−"}${text}${low.unit === "%" ? "%" : low.unit ? ` ${low.unit}` : ""}`;
  }
  const target = String(format(Math.max(min, Math.min(max, get() + delta))));
  return /^[←→]/.test(target) ? target : `→ ${target}`;
}
function buildEnvs() {
  $("env-remove").onclick = () => removeModulator("env");
  $("env-add").onclick = () => {
    if ((preset.env_count || 3) >= 8) return;
    preset.env_count = (preset.env_count || 3) + 1;
    syncEnvs();
    refreshColorDestinations();
    schedulePreset();
  };
  for (let i = 0; i < 8; i++) {
    const card = document.createElement("div");
    card.className = "env-card";
    card.innerHTML = `<div class="env-title"><strong>ENV ${i + 1}</strong>${i === 0 ? '<span class="env-role">AMP</span>' : ""}<output class="env-level">0%</output></div><div class="env-type"><select class="env-mode" aria-label="ENV ${i + 1} type"><option value="legacy">SAD PET</option><optgroup label="Rustias"></optgroup></select><select class="env-curve" aria-label="ENV ${i + 1} RADIAS curve"></select></div><canvas class="env-scope" width="288" height="64" aria-label="ENV ${i + 1} envelope"></canvas><div class="env-controls"></div>`;
    const mode = card.querySelector(".env-mode"),
      curve = card.querySelector(".env-curve");
    for (const [value, label] of ENV_MODES.slice(1))
      mode.querySelector("optgroup").append(option(value, label));
    ENV_CURVES.forEach((label, value) =>
      curve.append(option(String(value), `Curve ${value} · ${label}`)),
    );
    mode.onchange = () => {
      preset.envs[i].mode = mode.value;
      syncEnvs();
      scheduleParameter(
        `env${i + 1}_mode`,
        ENV_MODES.findIndex(([value]) => value === mode.value),
      );
    };
    curve.onchange = () => {
      preset.envs[i].curve = Number(curve.value);
      syncEnvs();
      scheduleParameter(`env${i + 1}_curve`, preset.envs[i].curve);
    };
    for (const field of ENV_FIELDS) {
      const label = document.createElement("label");
      label.className = "env-parameter";
      label.dataset.field = field;
      label.innerHTML = `<span><span class="env-name">${field.replace("_", " ").replace(/(\d)$/, " $1").toUpperCase()}</span><output></output></span><input type="range" min="0" max="1" step="0.001" aria-label="ENV ${i + 1} ${field.replace("_", " ")}">`;
      const input = label.querySelector("input");
      input.oninput = () => {
        preset.envs[i][field] = Number(input.value);
        syncEnvs();
        scheduleParameter(`env${i + 1}_${field}`, preset.envs[i][field]);
      };
      sliderMenu(input, `env${i + 1}_${field}`, `ENV ${i + 1} ${field.replace("_", " ")}`);
      card.querySelector(".env-controls").append(label);
    }
    const routes = modulatorRoutes(card, "env", i, ENV_ROUTES, syncEnvs);
    $("envs").append(card);
    envCards.push({
      card,
      canvas: card.querySelector("canvas"),
      routes,
      mode,
      curve,
      parameters: [...card.querySelectorAll(".env-parameter")],
    });
  }
}
function syncEnvs() {
  if (!preset?.envs) return;
  const count = preset.env_count || 3;
  $("env-count").textContent = `${count} / 8`;
  $("env-add").disabled = count >= 8;
  $("env-remove").hidden = count <= 3;
  $("env-remove").setAttribute("aria-label", `Remove ENV ${count}`);
  for (let i = 0; i < envCards.length; i++) {
    const { card, routes, mode, curve, parameters } = envCards[i],
      env = preset.envs[i];
    card.hidden = i >= count;
    if (!env) continue;
    const extended = envExtended(env);
    if (document.activeElement !== mode) mode.value = envMode(env);
    if (document.activeElement !== curve) curve.value = String(env.curve ?? 1);
    curve.hidden = envMode(env) !== "radias";
    for (let j = 0; j < ENV_FIELDS.length; j++) {
      const field = ENV_FIELDS[j],
        label = parameters[j],
        input = label.querySelector("input");
      label.hidden = ENV_EXTENDED.includes(field) && !extended;
      if (label.hidden) continue;
      if (field === "decay" || field === "sustain")
        label.querySelector(".env-name").textContent =
          `${field.toUpperCase()}${extended ? " 1" : ""}`;
      if (document.activeElement !== input) input.value = env[field];
      label.querySelector("output").textContent = ENV_LEVELS.includes(field)
        ? percent(envLevel(env, field))
        : envTime(envSeconds(env, field));
    }
    syncModulatorRoutes(routes, env);
  }
}
function drawEnvs() {
  for (let i = 0; i < envCards.length; i++) {
    const { canvas } = envCards[i];
    if (!visibleCanvas(canvas)) continue;
    const env = preset.envs[i], [c, w, h] = resizeCanvas(canvas);
    const drawKey = [w, h, devicePixelRatio, ...ENV_FIELDS.map((field) => env[field]), envMode(env), env.curve, state?.envs?.[i] || 0].join(":");
    if (envCards[i].drawKey === drawKey) continue;
    envCards[i].drawKey = drawKey;
    c.clearRect(0, 0, w, h);
    const top = 7,
      bottom = h - 7,
      level = bottom - (bottom - top) * env.sustain;
    if (envMode(env) !== "legacy") {
      drawRustiasEnv(c, env, w, top, bottom);
      drawEnvMeter(c, w, top, bottom, state?.envs?.[i] || 0);
      continue;
    }
    const durations = [
        12 + env.attack * 38,
        12 + env.decay * 38,
        24,
        12 + env.release * 38,
      ],
      scale = (w - 22) / durations.reduce((a, b) => a + b, 0);
    const a = 3 + durations[0] * scale,
      d = a + durations[1] * scale,
      s = d + durations[2] * scale,
      r = s + durations[3] * scale;
    c.strokeStyle = "#354832";
    c.lineWidth = 1;
    c.beginPath();
    c.moveTo(3, bottom);
    c.lineTo(w - 15, bottom);
    c.stroke();
    c.beginPath();
    c.moveTo(3, bottom);
    c.lineTo(a, top);
    c.bezierCurveTo(
      a + (d - a) * 0.2,
      level,
      d - (d - a) * 0.4,
      level,
      d,
      level,
    );
    c.lineTo(s, level);
    c.bezierCurveTo(
      s + (r - s) * 0.2,
      bottom,
      r - (r - s) * 0.4,
      bottom,
      r,
      bottom,
    );
    c.strokeStyle = "#b4d58e";
    c.lineWidth = 1.5;
    c.stroke();
    c.lineTo(3, bottom);
    c.fillStyle = "#a9cf8114";
    c.fill();
    drawEnvMeter(c, w, top, bottom, state?.envs?.[i] || 0);
  }
}
function drawEnvMeter(c, w, top, bottom, live) {
  c.fillStyle = "#2b392a";
  c.fillRect(w - 6, top, 3, bottom - top);
  c.fillStyle = "#d5f6a8";
  c.fillRect(w - 6, bottom - (bottom - top) * live, 3, (bottom - top) * live);
}
// Stage outline of a Rustias EG type, as its Q manual / RADIAS lifecycle runs.
function drawRustiasEnv(c, env, w, top, bottom) {
  const mode = envMode(env),
    radias = mode === "radias",
    extended = envExtended(env),
    span = (field) =>
      !radias && envControl(env[field]) === 0 ? 0 : 12 + env[field] * 38,
    shape = radias
      ? (t) => envCurve(env.curve ?? 1, t)
      : (t) => 1 - (1 - t) ** 3,
    linear = (t) => t,
    attack = {
      width: span("attack"),
      level: extended ? envLevel(env, "attack_level") : 1,
      shape: linear,
    },
    decay = { width: span("decay"), level: envLevel(env, "sustain"), shape },
    decay2 = {
      width: span("decay2"),
      level: envLevel(env, "sustain2"),
      shape,
    },
    stages = [attack, decay];
  if (extended) stages.push(decay2);
  if (mode === "q_loop_s1s2") stages.push(decay, decay2);
  if (["radias", "q_adsr", "q_ads1ds2r"].includes(mode))
    stages.push({ width: 24, level: stages.at(-1).level, shape: linear });
  stages.push({ width: span("release"), level: 0, shape });
  const total = stages.reduce((sum, stage) => sum + stage.width, 0) || 1,
    scale = (w - 22) / total,
    y = (value) => bottom - (bottom - top) * value;
  let x = 3,
    from = 0;
  c.strokeStyle = "#354832";
  c.lineWidth = 1;
  c.beginPath();
  c.moveTo(3, bottom);
  c.lineTo(w - 15, bottom);
  c.stroke();
  c.beginPath();
  c.moveTo(3, bottom);
  for (const stage of stages) {
    for (let k = 1; k <= 16; k++) {
      const t = k / 16;
      c.lineTo(
        x + stage.width * scale * t,
        y(from + (stage.level - from) * stage.shape(t)),
      );
    }
    x += stage.width * scale;
    from = stage.level;
  }
  c.strokeStyle = "#b4d58e";
  c.lineWidth = 1.5;
  c.stroke();
  c.lineTo(3, bottom);
  c.fillStyle = "#a9cf8114";
  c.fill();
}
const NOTE_NAMES = [
  "C",
  "C#",
  "D",
  "D#",
  "E",
  "F",
  "F#",
  "G",
  "G#",
  "A",
  "A#",
  "B",
];
const noteName = (note) =>
  `${NOTE_NAMES[note % 12]}${Math.floor(note / 12) - 1}`;
function parseNote(text) {
  const value = text.trim();
  if (/^\d{1,3}$/.test(value)) {
    const note = Number(value);
    return note <= 127 ? note : null;
  }
  const match = /^([a-g])([#b]?)(-?\d{1,2})$/i.exec(value);
  if (!match) return null;
  const base = { c: 0, d: 2, e: 4, f: 5, g: 7, a: 9, b: 11 }[
    match[1].toLowerCase()
  ];
  const note =
    (Number(match[3]) + 1) * 12 +
    base +
    (match[2] === "#" ? 1 : match[2].toLowerCase() === "b" ? -1 : 0);
  return note >= 0 && note <= 127 ? note : null;
}
function buildSequence() {
  neuroEditor = new window.SadNeuropolis($("neuropolis"), sequenceCatalog, {
    read: () => preset,
    edit: scheduleSequence,
    structure: schedulePreset,
    fxParameters: (slot) => colorDestinationSchema(slot.kind).map(([key]) => key),
    fxName: (slot) => COLOR_TYPES.find(([kind]) => kind === slot.kind)?.[1],
    knob: (schema, container, binding) =>
      createKnob(schema, container, binding),
    depthText: (parameter, depth) => unitDepth(parameterUnit(parameter), depth),
    view: (offset) =>
      flushEdits()
        .then(() => request("/api/sequencer/view", { offset }))
        .catch((e) => notify(e.message)),
  });
}
/** A Neuropolis track that plays one Sampler pad, and the sequencer on. */
const addPadTrack = (pad, name) => addTrack(name, { kind: "pad", pad });
/** A new Neuropolis track for the target, selected and shown, sequencer on. */
function addTrack(name, target) {
  const track = structuredClone(sequenceCatalog.track);
  const tracks = preset.sequencer.tracks;
  track.id = tracks.reduce((max, t) => Math.max(max, t.id), 0) + 1;
  track.name = name;
  track.target = target;
  tracks.push(track);
  preset.sequencer.enabled = true;
  if (neuroEditor) {
    neuroEditor.selected = track.id;
    neuroEditor.signature = "";
  }
  schedulePreset();
  syncAll();
  $("neuropolis").scrollIntoView({ behavior: "smooth", block: "start" });
  return track;
}
function syncPerformance() {
  const p = preset.performance;
  const arp = ["arp", "sequencer_arp"].includes(p.mode);
  $("performance-mode").value = p.mode;
  $("arp-controls").hidden = !arp;
  $("hold-controls").hidden = !["arp", "hold"].includes(p.mode);
  $("arp-hold-label").hidden = p.mode !== "arp";
  for (const [id, field] of [
    ["arp-direction", "direction"],
    ["arp-division", "arp_division"],
    ["arp-octaves", "octaves"],
    ["arp-gate", "arp_gate"],
  ]) {
    const input = $(id);
    if (document.activeElement === input) continue;
    if (input.tagName === "SELECT" && typeof p[field] === "number") {
      const selected = [...input.options].find(
        (o) => Math.abs(Number(o.value) - p[field]) < 0.00001,
      );
      input.value = selected?.value ?? String(p[field]);
    } else input.value = p[field];
  }
  $("arp-hold").checked = p.hold;
  $("performance-chord").disabled = arp && !p.hold;
  $("arp-gate-value").textContent = percent(p.arp_gate);
  if (document.activeElement !== $("performance-chord"))
    $("performance-chord").value = p.chord.map(noteName).join(" ");
  syncStepEntry();
}
function queueStepEntry(step, reset = false, clear = false) {
  noteChain = noteChain
    .then(async () => {
      if (pending && !reset) {
        await flushEdits();
      }
      await request("/api/step-entry", { step, reset, clear });
    })
    .catch((e) => notify(e.message));
  return noteChain;
}
function syncStepEntry() {
  const entry = ["sequencer", "sequencer_arp"].includes(preset.performance.mode)
    ? state?.step_entry
    : null;
  const active = Number.isInteger(entry);
  $("step-entry-prompt").hidden = !active;
  const notes = active ? preset.performance.steps[entry].notes : [];
  $("step-entry-label").textContent = active
    ? `Step ${String(entry + 1).padStart(2, "0")} · ${notes.length} / 8 notes`
    : "";
  $("step-entry-clear").disabled = notes.length === 0;
  $("keyboard").classList.toggle("choosing-note", active);
  for (const key of document.querySelectorAll(".key")) {
    const selected = notes.includes(Number(key.dataset.note));
    key.classList.toggle("selected-note", selected);
    if (active) key.setAttribute("aria-pressed", String(selected));
    else key.removeAttribute("aria-pressed");
  }
}
function bindPerformance() {
  $("performance-mode").onchange = (e) => {
    if (e.target.value === "hold" && preset.performance.mode === "keys")
      preset.performance.chord = [60];
    preset.performance.mode = e.target.value;
    releaseKeyboard();
    syncPerformance();
    schedulePreset();
  };
  for (const [id, field, numeric] of [
    ["arp-direction", "direction", false],
    ["arp-division", "arp_division", true],
    ["arp-octaves", "octaves", true],
    ["arp-gate", "arp_gate", true],
  ])
    $(id).oninput = (e) => {
      let value = numeric ? Number(e.target.value) : e.target.value;
      if (numeric && (!Number.isFinite(value) || e.target.value === "")) return;
      if (field === "length")
        value = Math.max(1, Math.min(16, Math.round(value)));
      preset.performance[field] = value;
      syncPerformance();
      if (numeric) {
        const id =
          {
            arp_division: "arp_rate",
            octaves: "arp_octaves",
            sequence_division: "sequence_rate",
            length: "sequence_length",
          }[field] || field;
        scheduleParameter(id, value);
      } else schedulePreset();
    };
  $("arp-hold").onchange = (e) => {
    preset.performance.hold = e.target.checked;
    preset.performance.chord = e.target.checked
      ? [...(state?.performance.chord || [])]
      : [...held];
    syncPerformance();
    schedulePreset();
  };
  $("performance-chord").onchange = (e) => {
    const text = e.target.value.trim(),
      notes = text ? text.split(/[\s,;]+/).map(parseNote) : [];
    if (notes.some((note) => note === null) || notes.length > 32) {
      e.target.value = preset.performance.chord.map(noteName).join(" ");
      notify("Enter up to 32 notes, for example C4 E4 G4.");
      return;
    }
    preset.performance.chord = [...new Set(notes)];
    e.target.value = preset.performance.chord.map(noteName).join(" ");
    schedulePreset();
  };
  $("clear-hold").onclick = async () => {
    releaseKeyboard();
    preset.performance.chord = [];
    syncPerformance();
    schedulePreset();
    await flushPreset().catch((e) => notify(e.message));
    await transport({ clear_hold: true });
  };
}
async function transport(data) {
  try {
    await request("/api/transport", data);
    if (state) Object.assign(state, data);
  } catch (e) {
    notify(e.message);
  }
}
function noteOn(note) {
  if (held.has(note)) return;
  held.add(note);
  vocalEditor?.stepInput(note);
  document.querySelector(`[data-note="${note}"]`)?.classList.add("active");
  noteChain = noteChain
    .then(() =>
      request("/api/note", {
        kind: "note_on",
        channel: 0,
        note,
        velocity: 100,
      }),
    )
    .catch((e) => {
      held.delete(note);
      notify(e.message);
    });
}
function noteOff(note) {
  if (!held.delete(note)) return;
  document.querySelector(`[data-note="${note}"]`)?.classList.remove("active");
  noteChain = noteChain
    .then(() =>
      request("/api/note", { kind: "note_off", channel: 0, note, velocity: 0 }),
    )
    .catch((e) => notify(e.message));
}
function startKey(source, note) {
  if (inputNotes.has(source) || note < 0 || note > 127) return;
  inputNotes.set(source, note);
  noteOn(note);
}
function finishKey(source) {
  const note = inputNotes.get(source);
  if (note === undefined) return;
  inputNotes.delete(source);
  if (![...inputNotes.values()].includes(note)) noteOff(note);
}
function releaseKeyboard() {
  inputNotes.clear();
  for (const note of [...held]) noteOff(note);
}
function setKeyboardOctave(value) {
  const next = Math.max(-1, Math.min(9, Math.round(Number(value))));
  if (!Number.isFinite(next)) {
    setKeyboardOctave(4);
    return;
  }
  if (next !== keyboardOctave) releaseKeyboard();
  keyboardOctave = next;
  const base = (next + 1) * 12;
  $("keyboard-octave").value = String(next);
  $("octave-down").disabled = next === -1;
  $("octave-up").disabled = next === 9;
  $("keyboard-range").textContent =
    `${noteName(base)}–${noteName(Math.min(127, base + 12))}`;
  $("keyboard").setAttribute(
    "aria-label",
    `${noteName(base)} to ${noteName(Math.min(127, base + 12))} keyboard`,
  );
  for (const key of document.querySelectorAll(".key")) {
    const note = base + Number(key.dataset.offset);
    key.dataset.note = String(note);
    key.disabled = note > 127;
    key.setAttribute(
      "aria-label",
      `Note ${noteName(note)}, key ${key.dataset.shortcut}`,
    );
    key.querySelector(".note-name").textContent = noteName(note);
    key.classList.toggle(
      "active",
      held.has(note) || !!state?.notes.includes(note),
    );
  }
  try {
    localStorage.setItem("evil-pet-keyboard-octave", String(next));
  } catch {}
}
function buildKeyboard() {
  const whites = [0, 2, 4, 5, 7, 9, 11, 12],
    blacks = [
      [1, 9],
      [3, 21.5],
      [6, 46.5],
      [8, 59],
      [10, 71.5],
    ],
    keys = Object.fromEntries(
      Object.entries(KEY_OFFSETS).map(([k, v]) => [v, k.slice(3)]),
    );
  function make(offset, black, left) {
    const b = document.createElement("button");
    b.className = `key${black ? " black" : ""}`;
    b.dataset.offset = String(offset);
    b.dataset.shortcut = keys[offset];
    if (black) b.style.left = `${left}%`;
    const span = document.createElement("span");
    span.className = "shortcut";
    span.textContent = keys[offset];
    const label = document.createElement("span");
    label.className = "note-name";
    b.append(span, label);
    b.addEventListener("pointerdown", (e) => {
      b.setPointerCapture(e.pointerId);
      startKey(`pointer:${e.pointerId}`, Number(b.dataset.note));
      e.preventDefault();
    });
    for (const event of ["pointerup", "pointercancel", "lostpointercapture"])
      b.addEventListener(event, (e) => finishKey(`pointer:${e.pointerId}`));
    b.addEventListener("keydown", (e) => {
      if (!["Enter", "Space"].includes(e.code) || e.repeat) return;
      e.preventDefault();
      e.stopPropagation();
      startKey(`access:${offset}`, Number(b.dataset.note));
    });
    b.addEventListener("keyup", (e) => {
      if (!["Enter", "Space"].includes(e.code)) return;
      e.preventDefault();
      e.stopPropagation();
      finishKey(`access:${offset}`);
    });
    b.addEventListener("blur", () => finishKey(`access:${offset}`));
    b.onclick = (e) => {
      if (e.detail !== 0) return;
      startKey(`click:${offset}`, Number(b.dataset.note));
      finishKey(`click:${offset}`);
    };
    $("keyboard").append(b);
  }
  for (const note of whites) make(note, false);
  for (const [note, left] of blacks) make(note, true, left);
  for (let octave = -1; octave <= 9; octave++)
    $("keyboard-octave").append(option(String(octave), `Oct ${octave}`));
  let saved = 4;
  try {
    saved = Number(localStorage.getItem("evil-pet-keyboard-octave") ?? 4);
  } catch {}
  setKeyboardOctave(saved);
}
async function loadWav(file) {
  if (!file) return;
  notify(`Loading ${file.name}…`, true);
  try {
    await fileLibrary.importFiles([file]);
    notify(`Loaded ${file.name}`);
  } catch (e) {
    notify(e.message);
  } finally {
    $("wav-file").value = "";
  }
}
async function loadStoredAudio(record) {
  await sampleSelection?.flush(false);
  if (fileLibrary.native) {
    // The app reads the file and applies its saved fades.
    const info = await request("/api/library/load", { id: record.id });
    presetLibrary?.markDirty();
    return info;
  }
  const response = await api("/api/wav", record.blob, {
    "Content-Type": "audio/wav",
    "X-File-Name": encodeURIComponent(
      record.factory ? record.displayName : record.name,
    ),
    "X-Library-Id": record.id,
  });
  const info = await response.json();
  presetLibrary?.markDirty();
  await request("/api/fades", await fileLibrary.fades(record.id));
  return info;
}
async function loadStoredPreset(loaded) {
  await preparePresetRecall();
  preset = await writePreset(loaded);
  syncAll();
}
async function refreshMidi(ask = false) {
  try {
    const data = await request(
        window.sadPetFetch && ask ? "/api/midi?request=1" : "/api/midi",
      ),
      select = $("midi");
    select.replaceChildren(option("", "Computer keyboard"));
    for (const p of data.ports) select.append(option(String(p.id), p.name));
    if (data.connected) {
      const found = data.ports.find((p) => p.name === data.connected);
      if (found) select.value = String(found.id);
    }
    $("midi-led").classList.toggle("off", !data.connected);
    window.SadMidiClock?.refresh();
  } catch (e) {
    notify(`MIDI: ${e.message}`);
  }
}
function bind() {
  bindPerformance();
  const input = (data) =>
    request("/api/input", data).catch((error) => notify(error.message, true));
  $("source-file").onclick = () => {
    if (state?.input?.active || state?.input?.pending || state?.input?.error)
      input({ active: false });
  };
  $("source-mic").onclick = () =>
    input({ active: !(state?.input?.active || state?.input?.pending) });
  $("input-device").onchange = (e) =>
    input({ active: true, device: e.target.value });
  $("input-mode").onchange = (e) => input({ mode: e.target.value });
  $("input-listen").onclick = () => input({ monitor: !state?.input?.monitor });
  $("input-gain").oninput = (e) => {
    $("input-gain-value").textContent = percent(Number(e.target.value));
    input({ gain: Number(e.target.value) });
  };
  $("input-capture").onclick = async () => {
    const button = $("input-capture");
    button.disabled = true;
    try {
      const response = await api("/api/input.wav");
      const stamp = new Date().toISOString().slice(0, 19).replace(/[-:T]/g, "");
      showSavedRecording(
        await fileLibrary.storeBlob(await response.blob(), {
          kind: "audio",
          name: `sad-pet-input-${stamp}.wav`,
          displayName: `Input ${new Date().toLocaleTimeString()}`,
          seconds: state?.input?.seconds,
        }),
      );
    } catch (error) {
      notify(error.message);
    } finally {
      button.disabled = false;
    }
  };
  $("record").onclick = toggleRecording;
  $("recording-use").onclick = () => {
    if (lastSavedRecording)
      fileLibrary.activate(lastSavedRecording).catch((e) => notify(e.message));
  };
  $("recording-trim").onclick = async () => {
    if (!lastSavedRecording) return;
    try {
      await fileLibrary.activate(lastSavedRecording);
      // Load acknowledgement precedes the next state poll.
      sampleSelection.focus(lastSavedRecording.id);
    } catch (error) {
      notify(error.message, true);
    }
  };
  $("recording-wav").onclick = () => {
    if (lastSavedRecording)
      fileLibrary
        .download(lastSavedRecording.id)
        .catch((e) => notify(e.message));
  };
  $("recording-mp3").onclick = () => {
    if (lastSavedRecording)
      fileLibrary
        .download(lastSavedRecording.id, "mp3")
        .catch((e) => notify(e.message));
  };
  $("recording-library").onclick = () => {
    if (lastSavedRecording) fileLibrary.reveal(lastSavedRecording);
  };
  $("octave-down").onclick = () => setKeyboardOctave(keyboardOctave - 1);
  $("octave-up").onclick = () => setKeyboardOctave(keyboardOctave + 1);
  $("keyboard-octave").onchange = (e) => {
    setKeyboardOctave(e.target.value);
    e.target.blur();
  };
  $("step-entry-cancel").onclick = () => queueStepEntry(null);
  $("step-entry-clear").onclick = () => {
    if (Number.isInteger(state?.step_entry))
      queueStepEntry(state.step_entry, false, true);
  };
  $("quit").onclick = async () => {
    try {
      await request("/api/quit", {});
      notify("SAD PET stopped. You can close this tab.", true);
    } catch (e) {
      notify(e.message);
    }
  };
  $("play").onclick = () =>
    transport({ playing: !state?.playing, midi_gate: false });
  $("freeze").onclick = () => transport({ frozen: !state?.frozen });
  $("bypass").onclick = () => transport({ bypass: !state?.bypass });
  $("reverse").onclick = () =>
    setParam("grain_size", 1 - preset.params.grain_size);
  $("mix").oninput = (e) => setParam("mix", Number(e.target.value));
  $("volume").oninput = (e) =>
    setParam("master_volume", Number(e.target.value));
  sliderMenu($("mix"), "mix", "Dry / wet");
  sliderMenu($("volume"), "master_volume", "Output volume");
  $("antialias").onchange = (e) => setParam("antialiasing", e.target.checked);
  $("loop-one-bar").onchange = (e) =>
    setParam("loop_one_bar", e.target.checked);
  for (let note = 0; note < 128; note++)
    $("sample-root-note").append(option(note, noteName(note)));
  $("sample-root-note").onchange = (e) =>
    setParam("sample_root_note", Number(e.target.value));
  $("bpm").oninput = (e) =>
    setParam("bpm", Math.max(20, Math.min(300, Number(e.target.value) || 120)));
  $("load-wav").onclick = () =>
    fileLibrary?.native
      ? fileLibrary.importDialog(true).catch((e) => notify(e.message, true))
      : $("wav-file").click();
  $("wav-file").onchange = (e) => loadWav(e.target.files[0]);
  const drop = $("drop-zone");
  for (const type of ["dragenter", "dragover"])
    drop.addEventListener(type, (e) => {
      e.preventDefault();
      if (!preset?.drums && !preset?.synth) drop.classList.add("drag");
    });
  for (const type of ["dragleave", "drop"])
    drop.addEventListener(type, (e) => {
      e.preventDefault();
      drop.classList.remove("drag");
    });
  drop.addEventListener("drop", (e) => {
    if (historySettings()) return;
    fileLibrary
      .importFiles([...e.dataTransfer.files])
      .catch((error) => notify(error.message, true));
  });
  $("midi-refresh").onclick = () => refreshMidi(true);
  $("midi").onchange = async (e) => {
    try {
      await request("/api/midi", {
        port: e.target.value === "" ? null : Number(e.target.value),
      });
      $("midi-led").classList.toggle("off", e.target.value === "");
    } catch (error) {
      notify(error.message);
    }
  };
  $("render").onclick = async () => {
    const b = $("render");
    b.disabled = true;
    b.textContent = "Rendering…";
    try {
      await flushEdits();
      await sampleSelection?.flush();
      const seconds = Number($("render-duration").value);
      if (fileLibrary.native) {
        // Rendered and stored by the app, then saved where the user chooses.
        const saved = await fileLibrary.renderScene(seconds, `${preset.name} · ${seconds}s`);
        await fileLibrary.download(saved.id);
        return;
      }
      const response = await api("/api/render", { seconds });
      const blob = await response.blob();
      const stamp = new Date().toISOString().slice(0, 19).replace(/[-:T]/g, "");
      const name = `sad-pet-${stamp}.wav`;
      const saved = await fileLibrary.saveRender(
        blob,
        name,
        `${preset.name} · ${seconds}s`,
        seconds,
      );
      download(saved.blob, saved.name);
    } catch (e) {
      notify(e.message);
    } finally {
      b.disabled = false;
      b.textContent = "Render WAV ↓";
    }
  };
  document.addEventListener("keydown", (e) => {
    if (
      e.repeat ||
      e.ctrlKey ||
      e.metaKey ||
      e.altKey ||
      e.target.matches("input,select,textarea,[role=slider],.sad-select") ||
      e.target.closest(".sample-selection,.recording-result") ||
      e.target.closest(".file-library,.sample-top")
    )
      return;
    if (e.code === "Escape") {
      queueStepEntry(null);
    } else if (e.code === "KeyZ" || e.code === "KeyX") {
      e.preventDefault();
      setKeyboardOctave(keyboardOctave + (e.code === "KeyZ" ? -1 : 1));
    } else if (e.code === "Space") {
      if (
        e.target.closest(
          ".sequence-step,.octave-controls,#step-entry-prompt,.color-card,.color-add",
        )
      )
        return;
      e.preventDefault();
      transport({ playing: !state?.playing, midi_gate: false });
    } else if (KEY_OFFSETS[e.code] !== undefined) {
      e.preventDefault();
      startKey(
        `key:${e.code}`,
        (keyboardOctave + 1) * 12 + KEY_OFFSETS[e.code],
      );
    }
  });
  document.addEventListener("keyup", (e) => {
    finishKey(`key:${e.code}`);
  });
  window.addEventListener("blur", () => {
    releaseKeyboard();
  });
}
const canvasInfo = new WeakMap();
const observedCanvases = new Set();
let canvasDirty = true, drawnState = null, waveBackground = null;
const canvasVisibility = typeof IntersectionObserver === "function"
  ? new IntersectionObserver(entries => {
      for (const entry of entries) canvasInfo.get(entry.target).visible = entry.isIntersecting;
      canvasDirty = true;
    }) : null;
const canvasResize = typeof ResizeObserver === "function"
  ? new ResizeObserver(entries => {
      for (const entry of entries) canvasInfo.get(entry.target).size = null;
      canvasDirty = true;
    }) : null;
function visibleCanvas(canvas) {
  if (canvas.closest("[hidden]")) return false;
  if (!canvasInfo.has(canvas)) {
    canvasInfo.set(canvas, { visible: true, size: null });
    observedCanvases.add(canvas);
    canvasVisibility?.observe(canvas);
    canvasResize?.observe(canvas);
  }
  return canvasInfo.get(canvas).visible;
}
for (const event of ["input", "pointerdown", "pointermove", "keydown", "visibilitychange"])
  document.addEventListener(event, () => { canvasDirty = true; }, { passive: true });
window.addEventListener("resize", () => {
  for (const canvas of observedCanvases) canvasInfo.get(canvas).size = null;
  canvasDirty = true;
}, { passive: true });
function resizeCanvas(canvas) {
  const info = canvasInfo.get(canvas);
  const r = info?.size || canvas.getBoundingClientRect();
  if (info) info.size = { width: r.width, height: r.height };
  const dpr = Math.min(devicePixelRatio || 1, 2),
    w = Math.round(r.width * dpr), h = Math.round(r.height * dpr);
  if (canvas.width !== w || canvas.height !== h) {
    canvas.width = w;
    canvas.height = h;
  }
  const c = canvas.getContext("2d");
  c.setTransform(dpr, 0, 0, dpr, 0, 0);
  return [c, r.width, r.height];
}

// A complete state answer carries a new waveform array even when its points
// did not change; rebuilding the background bitmap is the costly part.
function sameWaveform(a, b) {
  if (a === b) return true;
  if (a?.length !== b?.length) return false;
  for (let i = 0; i < a.length; i++)
    if (a[i][0] !== b[i][0] || a[i][1] !== b[i][1]) return false;
  return true;
}
function paintWaveform(context, w, h) {
  // Layout can report an empty canvas before the first resize.
  if (!w || !h) return;
  const dpr = Math.min(devicePixelRatio || 1, 2);
  const key = [w, h, dpr, state.sample_frames, state.sample_seconds, state.input?.active,
    sampleSelection?.region.start, sampleSelection?.region.end,
    sampleSelection?.fades.fade_in, sampleSelection?.fades.fade_out].join(":");
  if (!waveBackground || waveBackground.key !== key || !sameWaveform(waveBackground.waveform, state.waveform)) {
    const bitmap = document.createElement("canvas");
    bitmap.width = Math.round(w * dpr);
    bitmap.height = Math.round(h * dpr);
    const c = bitmap.getContext("2d");
    c.setTransform(dpr, 0, 0, dpr, 0, 0);
    c.strokeStyle = "#2b392a";
    c.lineWidth = 1;
    for (let i = 0; i < 17; i++) {
      const x = (i * w) / 16;
      c.beginPath();
      c.moveTo(x, 0);
      c.lineTo(x, h);
      c.stroke();
    }
    c.strokeStyle = "#354832";
    c.beginPath();
    c.moveTo(0, h / 2);
    c.lineTo(w, h / 2);
    c.stroke();
    const waveform = state.waveform;
    if (waveform.length) {
      c.fillStyle = "#8cba627a";
      c.beginPath();
      for (let i = 0; i < waveform.length; i++) {
        const x = (i * w) / (waveform.length - 1),
          y =
            h / 2 -
            waveform[i][1] *
              (sampleSelection?.gainAt(i / (waveform.length - 1)) ?? 1) *
              h *
              0.43;
        if (i === 0) c.moveTo(x, y);
        else c.lineTo(x, y);
      }
      for (let i = waveform.length - 1; i >= 0; i--)
        c.lineTo(
          (i * w) / (waveform.length - 1),
          h / 2 -
            waveform[i][0] *
              (sampleSelection?.gainAt(i / (waveform.length - 1)) ?? 1) *
              h *
              0.43,
        );
      c.closePath();
      c.fill();
    }
    waveBackground = { key, waveform: state.waveform, bitmap };
  }
  context.drawImage(waveBackground.bitmap, 0, 0, w, h);
}
function draw() {
  requestAnimationFrame(draw);
  if (!state || document.hidden || (drawnState === state && !canvasDirty)) return;
  canvasDirty = false;
  drawnState = state;
  if (visibleCanvas($("wave"))) {
    const [c, w, h] = resizeCanvas($("wave"));
    c.clearRect(0, 0, w, h);
    paintWaveform(c, w, h);
    sampleSelection?.draw(c, w, h);
    for (const grain of state.grains) {
      const x = grain.position * w,
        alpha = Math.sin(Math.PI * grain.age) * 0.6 + 0.15,
        // Advanced mode grains take their group's color.
        color = grain.group ? GRAIN_GROUP_COLORS[grain.group - 1] : null;
      c.strokeStyle = `rgba(${color ?? "203,255,148"},${alpha})`;
      c.beginPath();
      c.moveTo(x, 8);
      c.lineTo(x, h - 8);
      c.stroke();
      c.fillStyle = `rgba(${color ?? "219,255,182"},${alpha})`;
      c.beginPath();
      c.arc(x, h * 0.25 + grain.pan * h * 0.5, 2.5, 0, Math.PI * 2);
      c.fill();
    }
    const x = state.center * w;
    c.strokeStyle = "#d8f8b6";
    c.lineWidth = 1;
    c.beginPath();
    c.moveTo(x, 0);
    c.lineTo(x, h);
    c.stroke();
    c.fillStyle = "#d8f8b6";
    c.beginPath();
    c.moveTo(x - 4, 0);
    c.lineTo(x + 4, 0);
    c.lineTo(x, 5);
    c.fill();
  }
    for (let i = 0; i < scopes.length; i++) {
      if (!visibleCanvas(scopes[i].canvas)) continue;
      const s = scopes[i],
        ctx = s.canvas.getContext("2d"),
        a = s.values;
      ctx.clearRect(0, 0, 144, 40);
      ctx.strokeStyle = "#a9cf81";
      ctx.lineWidth = 1.4;
      ctx.beginPath();
      a.forEach((v, j) => {
        const x = (j * 144) / 80,
          y = 20 - v * 15;
        j ? ctx.lineTo(x, y) : ctx.moveTo(x, y);
      });
      ctx.stroke();
    }
    drawEnvs();
}
let failures = 0;
let receivedPreset = null;
let inputDevicesKey = "";
function syncInput(input) {
  if (!input) return;
  const active = input.active;
  setAttr($("source-file"), "aria-pressed", !active && !input.pending);
  setAttr($("source-mic"), "aria-pressed", active || input.pending);
  setText(
    $("source-mic"),
    input.pending
      ? "Cancel microphone"
      : window.sadPetPlugin
        ? "DAW input"
        : "Microphone",
  );
  $("microphone-controls").hidden = !active;
  const recordSlot = active
    ? $("microphone-record-slot")
    : document.querySelector(".transport-buttons");
  if ($("record").parentElement !== recordSlot) recordSlot.append($("record"));
  const resultSlot = active
    ? $("microphone-result-slot")
    : $("recording-result-slot");
  if ($("recording-result").parentElement !== resultSlot)
    resultSlot.append($("recording-result"));
  setAttr($("input-listen"), "aria-pressed", !!input.monitor);
  setText($("input-listen"), input.monitor ? "Listen on" : "Listen off");
  $("sample-select").hidden = active;
  $("live-source-name").hidden = !active;
  setText($("live-source-name"), input.name || "Microphone");
  $("input-status").classList.toggle("live", active);
  const peak = Math.max(...(input.peak || [0, 0]));
  $("input-level").style.width = `${Math.min(1, peak) * 100}%`;
  setAttr(
    $("input-level").parentElement,
    "aria-valuenow",
    Math.round(Math.min(1, peak) * 100),
  );
  if (active) $("render").disabled = true;
  else if ($("render").dataset.liveDisabled === "true")
    $("render").disabled = false;
  setAttr($("render"), "data-live-disabled", active);
  setAttr($("render"), "title", active ? "Use Record to save the live output" : "");
  const history = historySettings();
  const effectsOnly = history
    ? !history.granular : active && input.mode === "effects";
  // Grain engine stays visible with Granular off, Advanced groups included.
  for (const id of ["grain-controls", "grain-groups"]) {
    $(id).inert = effectsOnly;
    $(id).classList.toggle("inactive", effectsOnly);
  }
  $("antialias").disabled = effectsOnly;
  $("reverse").disabled = effectsOnly;
  const devicesKey = JSON.stringify(input.devices || []);
  if (devicesKey !== inputDevicesKey) {
    inputDevicesKey = devicesKey;
    $("input-device").replaceChildren(
      option("", "Default microphone"),
      ...(input.devices || []).map((d) => option(d.id, d.name)),
    );
  }
  if (document.activeElement !== $("input-device"))
    $("input-device").value = (input.devices || []).some(
      (d) => d.id === input.device,
    )
      ? input.device
      : "";
  if (document.activeElement !== $("input-mode"))
    $("input-mode").value = input.mode || "effects";
  if (document.activeElement !== $("input-gain")) {
    $("input-gain").value = input.gain;
    $("input-gain-value").textContent = percent(input.gain);
  }
}
// A poll repeats mostly unchanged values; equal writes would still make the
// browser restyle and lay out the editor.
function setText(element, text) {
  text = String(text);
  if (element.textContent !== text) element.textContent = text;
}
function setAttr(element, name, value) {
  value = String(value);
  if (element.getAttribute(name) !== value) element.setAttribute(name, value);
}
async function poll() {
  try {
    const generation = editGeneration;
    if (timbreBusy) return;
    const next = await request("/api/state");
    if (timbreBusy) return;
    const initial = state === null;
    state = next;
    connected = true;
    failures = 0;
    if (initial && generation === editGeneration && !pending) {
      preset = structuredClone(next.preset);
      syncAll();
    }
    let captured = false;
    for (let i = 0; i < 16; i++) {
      if (next.step_edits[i]) {
        preset.performance.steps[i] = structuredClone(next.step_edits[i]);
        captured = true;
      }
    }
    if (captured) {
      syncPerformance();
      if (!pending) schedulePreset();
    }
    if (
      !preset ||
      (generation === editGeneration &&
        !pending &&
        !editing &&
        !document.activeElement?.matches("input,select,.sad-menu-layer *") &&
        next.preset !== receivedPreset &&
        JSON.stringify(preset) !== JSON.stringify(next.preset))
    ) {
      preset = structuredClone(next.preset);
      syncAll();
    }
    if (generation === editGeneration && !pending && !editing
        && !document.activeElement?.matches("input,select,.sad-menu-layer *"))
      receivedPreset = next.preset;
    setText($("sample-name"), next.sample_name);
    presetLibrary?.observe(next, pending > 0);
    fileLibrary?.observe(next);
    setText($("sample-info"), `${next.sample_seconds.toFixed((next.preset?.drums || next.preset?.synth) && next.sample_seconds < 1 ? 3 : 2)} s`);
    syncInput(next.input);
    updateResynthMeter(next.resynth);
    neuroEditor?.update(next.sequencer, next.playing);
    timbreEditor?.sync(next.timbres);
    busMixer?.sync(next.timbres);
    syncTaps();
    syncGrainGroupOutputs();
    drumEditor?.update(next.drums);
    sampleSelection?.sync(next);
    setText($("sample-end"), formatTime(next.sample_seconds));
    setText($("position-info"), formatTime(next.center * next.sample_seconds));
    setText($("grain-count"), next.active_grains);
    setText($("drum-grain-count"), `${next.active_grains} grains`);
    window.SadMidiClock?.sync(next);
    setAttr($("play"), "aria-pressed", next.playing);
    setText($("play-icon"), next.playing ? "Ⅱ" : "▶");
    setText($("play-label"), next.playing ? "Pause" : "Play");
    for (const key of ["freeze", "bypass"])
      setAttr($(key), "aria-pressed", next[key === "freeze" ? "frozen" : key]);
    $("meter-l").style.width = `${Math.min(1, next.peak[0]) * 100}%`;
    $("meter-r").style.width = `${Math.min(1, next.peak[1]) * 100}%`;
    syncGainReduction(next);
    setText(
      $("audio-status"),
      window.sadPetAudio?.status() ||
        (next.audio_enabled
          ? `${(next.sample_rate / 1000).toFixed(1)} kHz · stereo`
          : "Audio output disabled"),
    );
    $("audio-led").classList.toggle("off", !next.audio_enabled);
    syncRecording();
    if (
      fileLibrary?.ready &&
      next.recording?.available &&
      !recordingBusy &&
      next.recording.id !== savedRecordingId &&
      next.recording.id !== failedRecordingId
    )
      saveCompletedRecording(next.recording);
    setText($("cpu"), `DSP ${next.cpu_percent.toFixed(1)}%`);
    if (next.audio_error) notify(next.audio_error, true);
    for (let i = 0; i < scopes.length; i++) {
      const a = scopes[i].values;
      a.push(next.lfos[i] || 0);
      if (a.length > 80) a.shift();
      setText(envCards[i].card.querySelector(".env-level"), percent(next.envs[i] || 0));
    }
    for (const key of document.querySelectorAll(".key"))
      key.classList.toggle(
        "active",
        held.has(Number(key.dataset.note)) ||
          next.notes.includes(Number(key.dataset.note)),
      );
    syncStepEntry();
    for (const [key, update] of modulationViews) update(next.knobs?.[key]);
    for (const [i, row] of matrixRows.entries())
      row.querySelector(".matrix-signal").style.opacity = String(
        0.15 + Math.min(1, Math.abs(next.matrix_values?.[i] || 0)) * 0.85,
      );
  } catch (e) {
    failures++;
    if (failures >= 3) {
      connected = false;
      $("audio-status").textContent = "Application disconnected";
      $("audio-led").classList.add("off");
      syncGainReduction(null);
    }
  } finally {
    // A hidden editor still notices finished recordings and audio errors.
    setTimeout(poll, document.hidden ? 500 : performance.now() - midiMovedAt < 400 ? 20 : 80);
  }
}
async function init() {
  try {
    presets = await request("/api/presets");
    colorDefaults = await request("/api/color-defaults");
    registerMooz(colorDefaults.mooz);
    sequenceCatalog = await request("/api/sequencer/catalog");
    state = await request("/api/state");
    preset = structuredClone(state.preset);
    for (const s of grainSchema) createKnob(s, "grain-controls");
    buildGrainGroups();
    buildFilters();
    buildColors();
    buildResynth();
    buildTaps();
    buildSynth();
    buildLfos();
    buildEnvs();
    buildMatrix();
    buildSequence();
    timbreEditor = new window.SadTimbres($("timbres"), {
      change: changeTimbre,
      features: () => state?.features,
    });
    if (window.SadMixer && state.timbres?.timbres)
      busMixer = new window.SadMixer($("bus-mixer"), {
        change: changeTimbre,
        knob: createKnob,
      });
    if (state.timbres?.timbres) {
      const drumCatalog = await request("/api/drums/catalog");
      drumEditor = new window.SadDrums($("drum-machine"), drumCatalog, {
        read: () => preset,
        context: () => state?.timbres,
        edit: scheduleDrum,
        parameter: scheduleParameter,
        structure: schedulePreset,
        knob: createKnob,
        routing: syncTimbreRouting,
        trigger: (track) =>
          request("/api/drums/trigger", { track, velocity: 1 }).catch((e) =>
            notify(e.message),
          ),
      });
    }
    if (window.SadSampler && state.timbres?.timbres)
      samplerEditor = new window.SadSampler($("sampler-editor"), {
        read: () => preset,
        context: () => state?.timbres,
        structure: schedulePreset,
        flush: flushEdits,
        change: changeTimbre,
        knob: createKnob,
        sequence: addPadTrack,
        request,
        api,
        notify,
        download,
      });
    if (window.SadVocal && state.timbres?.timbres)
      vocalEditor = new window.SadVocal($("vocal-editor"), {
        read: () => preset,
        context: () => state?.timbres,
        host: () => state?.plugin,
        request,
        api,
        notify,
        download,
      });
    window.SadModulation?.setup({
      read: () => preset,
      defaults: () => presets[0],
      catalog: () => sequenceCatalog,
      sources: MOD_SOURCES,
      destination: knobDestination,
      key: destinationKey,
      sequencer: sequencerParameter,
      value: destinationValue,
      parse: parseDestination,
      fill: fillDestination,
      depthText,
      knob: createKnob,
      parameter: scheduleParameter,
      notify,
      addTrack,
      // A structural edit: send the preset, then show it everywhere.
      changed() {
        if (neuroEditor) neuroEditor.signature = "";
        schedulePreset();
        syncAll();
      },
    });
    window.SadMidiLearn?.setup({
      notify,
      // The browser and desktop builds play the MIDI input chosen by Keyboard.
      hint: () =>
        getComputedStyle($("midi")).display !== "none" && !$("midi").value
          ? "choose a MIDI input next to Keyboard"
          : "",
      menu: (key, label) => window.SadModulation?.menu(key, label) ?? null,
    });
    buildKeyboard();
    bind();
    window.SadMidiClock?.setup({
      request,
      notify,
      // The app takes every MIDI input; the browser plays the one chosen.
      hasInput: () =>
        getComputedStyle($("midi")).display === "none" || !!$("midi").value,
      released: () => {
        if (preset) $("bpm").value = preset.params.bpm;
      },
    });
    sampleSelection = new window.SadSampleSelection({
      params: () => preset?.params,
      request,
      api,
      notify,
      library: () => fileLibrary,
      position: (value) => setParam("position", value),
      saved: showSavedRecording,
    });
    syncAll();
    fileLibrary = new window.EvilFileLibrary({
      notify,
      download,
      loadAudio: loadStoredAudio,
      loadPreset: loadStoredPreset,
      loadFades: (fades) => request("/api/fades", fades),
      presets: true,
      importPreset: (file) => presetLibrary.importFile(file),
      presetImported: (record) => presetLibrary.imported(record),
    });
    presetLibrary = new window.SadPresetLibrary(
      {
        request,
        api,
        notify,
        download,
        read: () => ({ preset, state }),
        edited: () => editGeneration > 0,
        flush: flushPreset,
        recall: preparePresetRecall,
        applyPreset: loadStoredPreset,
        refresh: refreshPresetState,
      },
      fileLibrary,
    );
    effectPresetLibrary = new window.SadEffectPresets(
      {
        native: Boolean(state.plugin),
        request, api, notify, download, flush: flushPreset,
        apply: async (id, saved) => {
          const slot = findColor(id);
          if (!slot || slot.kind !== saved.kind)
            throw new Error("The effect slot changed. Select it again.");
          const prefix = isMooz(slot.kind)
            ? "mooz_p"
            : (slot.kind === "lcr_delay" ? "lcr" : slot.kind) + "_";
          for (const key of Object.keys(slot.settings))
            if (key.startsWith(prefix))
              slot.settings[key] = saved.settings[key] ?? colorDefaults.slot.settings[key];
          slot.mix = saved.mix;
          editColorChain();
          await flushPreset();
        },
      },
      fileLibrary.store, colorDefaults.slot, COLOR_TYPES,
      {
        reverb_type: REVERB_TYPES.map(([value]) => value),
        waveshaper_type: WAVESHAPER_TYPES.map(([value]) => value),
      },
    );
    colorSignature = "";
    syncColors();
    Promise.resolve(state)
      .then(async (native) => {
        sampleSelection.sync(native);
        if (native.plugin) await presetLibrary.initialize(native, presets);
        await fileLibrary.initialize(native, presets);
        await effectPresetLibrary.initialize();
        if (native.plugin) await presetLibrary.migrate();
        else await presetLibrary.initialize(native, presets);
      })
      .catch((error) => notify(`Library: ${error.message}`, true));
    refreshMidi();
    poll();
    draw();
  } catch (e) {
    notify(`Cannot connect to SAD PET: ${e.message}`, true);
  }
}
init();
