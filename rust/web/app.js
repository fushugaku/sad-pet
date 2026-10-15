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
async function request(path, data) {
  return (await api(path, data)).json();
}
function download(blob, name) {
  const url = URL.createObjectURL(blob),
    a = document.createElement("a");
  a.href = url;
  a.download = name;
  a.click();
  setTimeout(() => URL.revokeObjectURL(url), 1000);
}
function schedulePreset() {
  pending++;
  clearTimeout(commitTimer);
  commitTimer = setTimeout(async () => {
    const version = pending;
    try {
      await request("/api/preset", preset);
      if (version === pending) pending = 0;
    } catch (e) {
      notify(e.message);
      if (version === pending) pending = 0;
    }
  }, 50);
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
    const message = state.input.pending
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
function setParam(key, value) {
  preset.params[key] = value;
  syncControl(key);
  schedulePreset();
}
const grainSchema = [
  ["position", "POSITION", (v) => percent(v)],
  ["grains", "GRAINS", (v) => `${(128 * v ** 3).toFixed(1)} /s`],
  [
    "grain_size",
    "SIZE",
    (v) =>
      `${v < 0.5 ? "←" : "→"} ${Math.round((0.01 + 2 * Math.abs(v * 2 - 1) ** 3) * 1000)} ms`,
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
];
function createKnob(
  [key, label, format, min = 0, max = 1, aria = label],
  bank,
  binding = null,
) {
  const get = () => (binding ? binding.get() : preset.params[key]);
  const set = (v) => (binding ? binding.set(v) : setParam(key, v));
  const box = document.createElement("div");
  box.className = "knob";
  box.innerHTML = `<div class="dial" role="slider" tabindex="0" aria-label="${label}" aria-valuemin="0" aria-valuemax="100"><svg viewBox="0 0 77 77" aria-hidden="true"><defs><radialGradient id="knob-face"><stop offset="0" stop-color="#414c3b"/><stop offset=".8" stop-color="#2d3629"/><stop offset="1" stop-color="#20291e"/></radialGradient></defs><circle class="track" cx="38.5" cy="38.5" r="25"/><circle class="arc" cx="38.5" cy="38.5" r="25"/><path class="modulation-range"/><circle class="face" cx="38.5" cy="38.5" r="19.5"/><line class="pointer" x1="38.5" y1="38.5" x2="38.5" y2="23.5"/><g class="modulation-live"><circle cx="38.5" cy="7.5" r="3.2"/></g></svg></div><div class="knob-label">${label}</div><output class="knob-value"></output><span class="knob-mod-range" aria-hidden="true"></span>`;
  const dial = box.querySelector(".dial"),
    value = box.querySelector("output"),
    pointer = box.querySelector(".pointer"),
    arc = box.querySelector(".arc"),
    modRange = box.querySelector(".modulation-range"),
    modLive = box.querySelector(".modulation-live"),
    modText = box.querySelector(".knob-mod-range");
  dial.dataset.controlKey = key;
  dial.setAttribute("aria-label", aria);
  dial.setAttribute("aria-valuemin", String(min * 100));
  dial.setAttribute("aria-valuemax", String(max * 100));
  const clamp = (v) => Math.max(min, Math.min(max, v));
  let drag = null;
  const modulation = (feedback) => {
    const base = get();
    const active = Boolean(feedback?.modulated);
    box.classList.toggle("is-modulated", active);
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
    const unit = (v) => (v - min) / (max - min);
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
      text.replace(/\s*·.*$/, "").replace(/\s+(Hz|ms|st|s)\b/g, "$1");
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
    const v = get();
    const unit = (v - min) / (max - min);
    value.textContent = format(v);
    pointer.style.transform = `rotate(${-135 + 270 * unit}deg)`;
    arc.style.strokeDasharray = `${Math.max(0.01, 117.81 * unit)} 157.08`;
    dial.setAttribute("aria-valuenow", String(Math.round(v * 100)));
    dial.setAttribute("aria-valuetext", format(v));
    modulation(state?.knobs?.[key]);
  };
  dial.addEventListener("focus", update);
  dial.addEventListener("blur", update);
  dial.addEventListener("pointerdown", (e) => {
    if (!preset) return;
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
    set(clamp(drag.value + delta * (max - min)));
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
  dial.addEventListener("keydown", (e) => {
    let v = get();
    const step = (e.shiftKey ? 0.05 : 0.005) * (max - min);
    if (e.key === "ArrowUp" || e.key === "ArrowRight") v += step;
    else if (e.key === "ArrowDown" || e.key === "ArrowLeft") v -= step;
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
      set(clamp(get() - Math.sign(e.deltaY) * 0.005 * (max - min)));
    },
    { passive: false },
  );
  controls.set(key, update);
  modulationViews.set(key, modulation);
  $(bank).append(box);
}
function syncControl(key) {
  controls.get(key)?.();
  if (key.startsWith("filter") || key.startsWith("vcf")) syncFilters();
  if (key === "mix" || key === "master_volume") syncSliders();
  if (key === "bpm") syncColors();
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
    card.innerHTML = `<div class="filter-title"><strong>Filter ${number}</strong><select id="${prefix}-type" aria-label="Filter ${number} type"><option value="low_pass">Low pass</option><option value="band_pass">Band pass</option><option value="high_pass">High pass</option><option value="comb">Comb</option></select><select id="${prefix}-env" aria-label="Filter ${number} envelope"><option value="env1">ENV 1</option><option value="env2">ENV 2</option><option value="env3">ENV 3</option></select></div><div id="${prefix}-controls" class="knob-grid filter-controls"></div>`;
    $("filters").append(card);
    const type = i === 0 ? "vcf_type" : "vcf2_type";
    $(`${prefix}-type`).onchange = (e) => setParam(type, e.target.value);
    $(`${prefix}-env`).onchange = (e) =>
      setParam(`${prefix}_env`, e.target.value);
    const cutoff = (v) => {
      const comb = preset.params[type] === "comb";
      return `${Math.round((comb ? 30 : 20) * (comb ? 200 : 1000) ** v).toLocaleString()} Hz`;
    };
    for (const schema of [
      [prefix, "CUTOFF", cutoff],
      [`${prefix}_resonance`, "RESONANCE", percent],
      [`${prefix}_env_depth`, "ENV DEPTH", signedPercent, -1, 1],
      [`${prefix}_keytrack`, "KEYTRACK", percent],
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
    $(`${prefix}-type`).value = p[type];
    $(`${prefix}-env`).value = p[`${prefix}_env`];
    controls.get(prefix)?.();
  }
}
function syncAll() {
  syncColors();
  for (const update of controls.values()) update();
  syncSliders();
  $("antialias").checked = preset.params.antialiasing;
  $("bpm").value = preset.params.bpm;
  syncFilters();
  $("reverse").setAttribute(
    "aria-pressed",
    String(preset.params.grain_size < 0.5),
  );
  syncLfos();
  syncEnvs();
  syncMatrix();
  syncPerformance();
}
function option(value, label) {
  const o = document.createElement("option");
  o.value = value;
  o.textContent = label;
  return o;
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
];
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
const delayIndex = (v, count = 128) =>
  Math.round(Math.max(0, Math.min(1, v)) * (count - 1));
function delayRatio(v, slot, prefix) {
  const tables = colorDefaults.delay_tables;
  return `${(tables[slot.settings[`${prefix}_sync`] ? "sync_ratio" : "free_ratio"][delayIndex(v)] / 10).toFixed(1)}%`;
}
function delayTime(v, slot, prefix, sync) {
  const tables = colorDefaults.delay_tables;
  const ratio =
    tables[sync ? "sync_ratio" : "free_ratio"][
      delayIndex(slot.settings[`${prefix}_ratio`])
    ] / 1000;
  const seconds = sync
    ? ((60 / preset.params.bpm) * ratio * tables.notes[delayIndex(v, 14)]) / 192
    : (ratio * tables[prefix === "lcr" ? "lcr_ms" : "mod_ms"][delayIndex(v)]) /
      1000;
  const ms = Math.round(Math.min(tables.max_seconds, seconds) * 1000);
  return sync ? `${DELAY_NOTES[delayIndex(v, 14)]} · ${ms} ms` : `${ms} ms`;
}
const COLOR_SCHEMAS = {
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
    ["compressor_attack", "ATTACK", (v) => `${(0.1 * 1000 ** v).toFixed(1)} ms`],
    ["compressor_release", "RELEASE", (v) => `${Math.round(10 * 200 ** v)} ms`],
    ["compressor_knee", "KNEE", (v) => `${(24 * v).toFixed(1)} dB`],
    ["compressor_makeup", "MAKEUP", (v) => `${(24 * v).toFixed(1)} dB`],
  ],
};
function colorSchema(kind, slot = null) {
  const prefix = delayPrefix(kind);
  const sync = prefix && slot?.settings[`${prefix}_sync`];
  const schema = COLOR_SCHEMAS[kind].map(([parameter, label, format]) => {
    if (sync && /_(left|center|right)_delay$/.test(parameter))
      return [
        parameter.replace(/_delay$/, "_note"),
        label,
        (v, s) => delayTime(v, s, prefix, true),
      ];
    return [parameter, label, format];
  });
  return [...schema, ["mix", "MIX", percent]];
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
  return schema;
}
const destinationValue = (value) =>
  typeof value === "string" ? value : JSON.stringify(value);
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
  preset.color_chain = preset.color_chain.filter((s) => s.id !== id);
  for (const lfo of preset.lfos)
    if (lfo.destination?.color?.slot === id) lfo.destination = "none";
  for (const env of preset.envs)
    for (const field of ["destination", "extra_destination"])
      if (env[field]?.color?.slot === id) env[field] = "none";
  for (const route of preset.matrix)
    if (route.destination?.color?.slot === id) route.destination = "none";
  editColorChain();
}
function moveColor(id, delta) {
  const index = preset.color_chain.findIndex((s) => s.id === id),
    target = index + delta;
  if (index < 0 || target < 0 || target >= preset.color_chain.length) return;
  [preset.color_chain[index], preset.color_chain[target]] = [
    preset.color_chain[target],
    preset.color_chain[index],
  ];
  editColorChain();
}
function buildColors() {
  $("color-add-type").replaceChildren(
    ...COLOR_TYPES.map(([value, label]) => option(value, label)),
  );
  $("color-add").onclick = () => {
    if (preset.color_chain.length >= colorDefaults.max_slots) return;
    nextColorId = Math.max(
      nextColorId,
      ...preset.color_chain.map((s) => s.id + 1),
    );
    if (nextColorId > 4294967295) nextColorId = 1;
    while (preset.color_chain.some((s) => s.id === nextColorId)) nextColorId++;
    const slot = structuredClone(colorDefaults.slot);
    slot.id = nextColorId++;
    slot.kind = $("color-add-type").value;
    if (slot.kind === "compressor") slot.mix = 1;
    preset.color_chain.push(slot);
    editColorChain();
  };
}
function syncColors() {
  if (!preset?.color_chain || !colorDefaults) return;
  const signature = preset.color_chain
    .map(
      (s) =>
        `${s.id}:${s.kind}:${s.settings[`${delayPrefix(s.kind)}_sync`] ?? ""}`,
    )
    .join(",");
  if (
    signature !== colorSignature ||
    $("color-chain").children.length !== preset.color_chain.length
  ) {
    colorSignature = signature;
    for (const key of controls.keys())
      if (key.startsWith("color:")) {
        controls.delete(key);
        modulationViews.delete(key);
      }
    $("color-chain").replaceChildren();
    for (const [index, slot] of preset.color_chain.entries()) {
      const number = index + 1,
        id = slot.id;
      const card = document.createElement("div");
      card.className = "color-card";
      card.dataset.id = id;
      card.innerHTML = `<div class="color-title"><strong class="color-order">${String(number).padStart(2, "0")}</strong><select aria-label="FX ${number} type"></select><div class="color-actions"><button class="color-bypass" aria-label="Bypass FX ${number}" title="Bypass" aria-pressed="false">⏻</button><button class="color-up" aria-label="Move FX ${number} up" title="Move up">↑</button><button class="color-down" aria-label="Move FX ${number} down" title="Move down">↓</button><button class="color-remove" aria-label="Remove FX ${number}" title="Remove">×</button></div></div><div id="color-${id}-knobs" class="knob-grid color-knobs"></div>`;
      const select = card.querySelector("select");
      select.replaceChildren(
        ...COLOR_TYPES.map(([value, label]) => option(value, label)),
      );
      select.value = slot.kind;
      select.onchange = () => {
        preset.color_chain.find((s) => s.id === id).kind = select.value;
        editColorChain();
      };
      if (slot.kind === "reverb") {
        const title = card.querySelector(".color-title");
        title.classList.add("has-options");
        const mode = document.createElement("select");
        mode.className = "color-option color-reverb-type";
        mode.setAttribute("aria-label", `FX ${number} reverb type`);
        mode.replaceChildren(
          ...REVERB_TYPES.map(([value, label]) => option(value, label)),
        );
        mode.value = slot.settings.reverb_type;
        mode.onchange = () => {
          preset.color_chain.find((s) => s.id === id).settings.reverb_type =
            mode.value;
          editColorChain();
        };
        title.insertBefore(mode, card.querySelector(".color-actions"));
      }
      const prefix = delayPrefix(slot.kind);
      if (prefix) {
        const title = card.querySelector(".color-title");
        title.classList.add("has-options");
        const clock = document.createElement("select");
        clock.className = "color-option color-delay-clock";
        clock.setAttribute("aria-label", `FX ${number} delay clock`);
        clock.replaceChildren(
          option("free", "Milliseconds"),
          option("sync", "Tempo sync"),
        );
        clock.value = slot.settings[`${prefix}_sync`] ? "sync" : "free";
        clock.onchange = () => {
          preset.color_chain.find((s) => s.id === id).settings[
            `${prefix}_sync`
          ] = clock.value === "sync";
          editColorChain();
        };
        title.insertBefore(clock, card.querySelector(".color-actions"));
      }
      card.querySelector(".color-bypass").onclick = () => {
        const s = preset.color_chain.find((s) => s.id === id);
        s.enabled = !s.enabled;
        editColorChain();
      };
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
      $("color-chain").append(card);
      for (const [parameter, label, format] of schema) {
        const key = `color:${id}:${parameter}`;
        const getSlot = () => preset.color_chain.find((s) => s.id === id);
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
            `FX ${number} ${label.toLowerCase()}`,
          ],
          `color-${id}-knobs`,
          {
            get,
            set: (value) => {
              const s = getSlot();
              if (parameter === "mix") s.mix = value;
              else s.settings[parameter] = value;
              colorLegacy();
              controls.get(key)?.();
              if (parameter.endsWith("_ratio")) syncColors();
              schedulePreset();
            },
            reset: () =>
              parameter === "mix"
                ? slot.kind === "compressor"
                  ? 1
                  : colorDefaults.slot.mix
                : colorDefaults.slot.settings[parameter],
          },
        );
      }
    }
    colorRoutesSignature = "";
  }
  const routesSignature = JSON.stringify([
    preset.lfos.map((l) => l.destination),
    preset.envs.map((e) => [e.destination, e.extra_destination]),
  ]);
  if (routesSignature !== colorRoutesSignature) {
    colorRoutesSignature = routesSignature;
    refreshColorDestinations();
  }
  for (const [i, slot] of preset.color_chain.entries()) {
    const card = $("color-chain").children[i];
    const mode = card.querySelector(".color-reverb-type");
    if (mode && document.activeElement !== mode)
      mode.value = slot.settings.reverb_type;
    card.classList.toggle("is-bypassed", !slot.enabled);
    card
      .querySelector(".color-bypass")
      .setAttribute("aria-pressed", String(!slot.enabled));
    card.querySelector(".color-up").disabled = i === 0;
    card.querySelector(".color-down").disabled =
      i === preset.color_chain.length - 1;
  }
  $("color-add").disabled =
    preset.color_chain.length >= colorDefaults.max_slots;
  for (const [key, update] of controls) if (key.startsWith("color:")) update();
}
function fillDestination(select, destination) {
  const value = destinationValue(destination);
  select.replaceChildren(...DESTINATIONS.map(([v, label]) => option(v, label)));
  for (const [label, choices] of modulationGroups()) {
    const group = document.createElement("optgroup");
    group.label = label;
    for (const [value, text] of choices)
      group.append(option(destinationValue(value), text));
    select.append(group);
  }
  for (const [index, slot] of preset.color_chain.entries()) {
    const group = document.createElement("optgroup");
    group.label = `FX ${index + 1} · ${COLOR_TYPES.find(([k]) => k === slot.kind)[1]}`;
    for (const [parameter, label] of colorDestinationSchema(slot.kind))
      group.append(
        option(
          destinationValue({ color: { slot: slot.id, parameter } }),
          `FX ${index + 1} · ${label.charAt(0) + label.slice(1).toLowerCase()}`,
        ),
      );
    select.append(group);
  }
  if (
    ![...select.options].some((o) => o.value === value) &&
    typeof destination !== "string"
  ) {
    const index = preset.color_chain.findIndex(
      (s) => s.id === destination.color.slot,
    );
    const label = destination.color.parameter.replaceAll("_", " ");
    select.append(option(value, `FX ${index + 1} · ${label} (inactive)`));
  }
  select.value = value;
}
function refreshColorDestinations() {
  for (const [i, scope] of scopes.entries())
    fillDestination(
      scope.card.querySelector(".destination"),
      preset.lfos[i].destination,
    );
  for (const [i, card] of envCards.entries())
    for (const route of card.routes)
      fillDestination(route.select, preset.envs[i][route.destination]);
  for (const [i, row] of matrixRows.entries())
    fillDestination(
      row.querySelector(".matrix-destination"),
      preset.matrix[i].destination,
    );
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
      ],
    ],
  ];
  for (let index = 0; index < 3; index++) {
    groups.push([
      `LFO ${index + 1}`,
      [
        ["rate", "Rate"],
        ["depth", "Depth"],
        ["division", "Sync division"],
      ].map(([key, label]) => [
        { lfo: { index, parameter: key } },
        `LFO ${index + 1} · ${label}`,
      ]),
    ]);
    groups.push([
      `ENV ${index + 1}`,
      [
        ["attack", "Attack"],
        ["decay", "Decay"],
        ["sustain", "Sustain"],
        ["release", "Release"],
        ["depth", "Route 1 depth"],
        ["extra_depth", "Route 2 depth"],
      ].map(([key, label]) => [
        { env: { index, parameter: key } },
        `ENV ${index + 1} · ${label}`,
      ]),
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
  return groups;
}
const MOD_SOURCES = [
  ["lfo1", "LFO 1"],
  ["lfo2", "LFO 2"],
  ["lfo3", "LFO 3"],
  ["env1", "ENV 1 · Amp"],
  ["env2", "ENV 2"],
  ["env3", "ENV 3"],
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
      row.innerHTML = `<strong class="matrix-order">${String(number).padStart(2, "0")}<i class="matrix-signal" aria-hidden="true"></i></strong><select class="matrix-source" aria-label="Modulation ${number} source"></select><select class="matrix-destination" aria-label="Modulation ${number} destination"></select><label class="matrix-depth"><input type="range" min="-1" max="1" step="0.005" aria-label="Modulation ${number} depth"><output></output></label><div class="matrix-actions"><button class="matrix-bypass" title="Bypass" aria-label="Bypass modulation ${number}" aria-pressed="false">⏻</button><button class="matrix-remove" title="Remove" aria-label="Remove modulation ${number}">×</button></div>`;
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
        row.querySelector("output").textContent = signedPercent(
          preset.matrix[i].depth,
        );
        schedulePreset();
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
    row.querySelector("output").textContent = signedPercent(route.depth);
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
function buildLfos() {
  for (let i = 0; i < 3; i++) {
    const card = document.createElement("div");
    card.className = "lfo-card";
    card.innerHTML = `<div class="lfo-title"><strong>LFO ${i + 1}</strong><canvas class="lfo-scope" width="144" height="40"></canvas></div><div class="lfo-row"><select aria-label="LFO ${i + 1} waveform" class="waveform"></select><select aria-label="LFO ${i + 1} destination" class="destination"></select></div><div class="lfo-row"><label>RATE<input aria-label="LFO ${i + 1} rate" class="rate" type="number" min="0.01" max="40" step="0.01"></label><label>DEPTH<input aria-label="LFO ${i + 1} depth" class="depth" type="range" min="-1" max="1" step="0.01"></label></div><div class="lfo-row"><label class="check"><input type="checkbox" class="sync">Tempo sync</label><select class="division" aria-label="LFO ${i + 1} tempo division"><option value="0.25">1/16</option><option value="0.5">1/8</option><option value="1">1/4</option><option value="2">1/2</option><option value="4">1 bar</option><option value="8">2 bars</option><option value="16">4 bars</option><option value="32">8 bars</option></select></div>`;
    for (const [value, label] of WAVEFORMS)
      card.querySelector(".waveform").append(option(value, label));
    for (const [value, label] of DESTINATIONS)
      card.querySelector(".destination").append(option(value, label));
    for (const [field, selector] of [
      ["waveform", ".waveform"],
      ["destination", ".destination"],
      ["rate_hz", ".rate"],
      ["depth", ".depth"],
      ["sync", ".sync"],
      ["division", ".division"],
    ]) {
      const input = card.querySelector(selector);
      input.addEventListener("input", () => {
        let value = input.type === "checkbox" ? input.checked : input.value;
        if (field === "destination") value = parseDestination(value);
        if (["rate_hz", "depth", "division"].includes(field)) {
          value = Number(value);
          if (!Number.isFinite(value)) return;
        }
        preset.lfos[i][field] = value;
        if (field === "sync") syncLfos();
        schedulePreset();
      });
    }
    $("lfos").append(card);
    scopes.push({ card, canvas: card.querySelector("canvas"), values: [] });
  }
}
function syncLfos() {
  for (let i = 0; i < 3; i++) {
    const card = scopes[i].card,
      l = preset.lfos[i];
    for (const [field, selector] of [
      ["waveform", ".waveform"],
      ["destination", ".destination"],
      ["rate_hz", ".rate"],
      ["depth", ".depth"],
      ["division", ".division"],
    ]) {
      const input = card.querySelector(selector);
      if (document.activeElement !== input)
        input.value =
          field === "destination" ? destinationValue(l[field]) : l[field];
    }
    card.querySelector(".sync").checked = l.sync;
    card.querySelector(".rate").disabled = l.sync;
    card.querySelector(".division").disabled = !l.sync;
  }
}
const envCards = [];
const ENV_FIELDS = ["attack", "decay", "sustain", "release"];
const envSeconds = (env, field) =>
  (field === "attack" ? 0.002 : 0.005) +
  env[field] ** 3 * (field === "release" ? 12 : 8);
const envTime = (seconds) =>
  seconds < 1 ? `${Math.round(seconds * 1000)} ms` : `${seconds.toFixed(2)} s`;
const signedPercent = (value) =>
  `${value > 0 ? "+" : ""}${Math.round(value * 100)}%`;
function buildEnvs() {
  for (let i = 0; i < 3; i++) {
    const card = document.createElement("div");
    card.className = "env-card";
    card.innerHTML = `<div class="env-title"><strong>ENV ${i + 1}</strong>${i === 0 ? '<span class="env-role">AMP</span>' : ""}<output class="env-level">0%</output></div><canvas class="env-scope" width="288" height="64" aria-label="ENV ${i + 1} envelope"></canvas><div class="env-controls"></div>`;
    for (const field of ENV_FIELDS) {
      const label = document.createElement("label");
      label.className = "env-parameter";
      label.innerHTML = `<span>${field.toUpperCase()}<output></output></span><input type="range" min="0" max="1" step="0.001" aria-label="ENV ${i + 1} ${field}">`;
      const input = label.querySelector("input");
      input.oninput = () => {
        preset.envs[i][field] = Number(input.value);
        syncEnvs();
        schedulePreset();
      };
      card.querySelector(".env-controls").append(label);
    }
    const routes = [];
    for (let j = 0; j < 2; j++) {
      const row = document.createElement("div");
      row.className = "env-route";
      row.innerHTML = `<select aria-label="ENV ${i + 1} destination ${j + 1}"></select><label><input type="range" min="-1" max="1" step="0.01" aria-label="ENV ${i + 1} depth ${j + 1}"><output></output></label>`;
      const select = row.querySelector("select"),
        input = row.querySelector("input");
      for (const [value, label] of DESTINATIONS)
        select.append(option(value, label));
      const destination = j === 0 ? "destination" : "extra_destination",
        depth = j === 0 ? "depth" : "extra_depth";
      select.oninput = () => {
        preset.envs[i][destination] = parseDestination(select.value);
        syncEnvs();
        schedulePreset();
      };
      input.oninput = () => {
        preset.envs[i][depth] = Number(input.value);
        syncEnvs();
        schedulePreset();
      };
      routes.push({
        select,
        input,
        output: row.querySelector("output"),
        destination,
        depth,
      });
      card.append(row);
    }
    $("envs").append(card);
    envCards.push({ card, canvas: card.querySelector("canvas"), routes });
  }
}
function syncEnvs() {
  if (!preset?.envs) return;
  for (let i = 0; i < envCards.length; i++) {
    const { card, routes } = envCards[i],
      env = preset.envs[i];
    for (let j = 0; j < ENV_FIELDS.length; j++) {
      const field = ENV_FIELDS[j],
        label = card.querySelectorAll(".env-parameter")[j],
        input = label.querySelector("input");
      if (document.activeElement !== input) input.value = env[field];
      label.querySelector("output").textContent =
        field === "sustain"
          ? percent(env[field])
          : envTime(envSeconds(env, field));
    }
    for (const route of routes) {
      if (document.activeElement !== route.select)
        route.select.value = destinationValue(env[route.destination]);
      if (document.activeElement !== route.input)
        route.input.value = env[route.depth];
      route.output.textContent = signedPercent(env[route.depth]);
    }
  }
}
function drawEnvs() {
  for (let i = 0; i < envCards.length; i++) {
    const { canvas } = envCards[i],
      env = preset.envs[i],
      [c, w, h] = resizeCanvas(canvas);
    c.clearRect(0, 0, w, h);
    const top = 7,
      bottom = h - 7,
      level = bottom - (bottom - top) * env.sustain;
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
    const live = state?.envs?.[i] || 0;
    c.fillStyle = "#2b392a";
    c.fillRect(w - 6, top, 3, bottom - top);
    c.fillStyle = "#d5f6a8";
    c.fillRect(w - 6, bottom - (bottom - top) * live, 3, (bottom - top) * live);
  }
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
const sequenceCards = [];
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
  for (let i = 0; i < 16; i++) {
    const card = document.createElement("div");
    card.className = "sequence-step";
    const toggle = document.createElement("button");
    toggle.className = "step-toggle";
    toggle.textContent = String(i + 1).padStart(2, "0");
    toggle.setAttribute("aria-label", `Step ${i + 1} enabled`);
    toggle.onclick = () => {
      preset.performance.steps[i].enabled =
        !preset.performance.steps[i].enabled;
      syncPerformance();
      schedulePreset();
    };
    const note = document.createElement("button");
    note.type = "button";
    note.className = "step-note";
    note.onclick = () => {
      const step = state?.step_entry === i ? null : i;
      queueStepEntry(step);
      if (step !== null)
        $("keyboard-section").scrollIntoView({ block: "nearest" });
    };
    card.append(toggle, note);
    $("sequence-steps").append(card);
    sequenceCards.push({ card, toggle, note });
  }
}
function syncPerformance() {
  const p = preset.performance;
  const arp = ["arp", "sequencer_arp"].includes(p.mode);
  const sequence = ["sequencer", "sequencer_arp"].includes(p.mode);
  $("performance-mode").value = p.mode;
  $("arp-controls").hidden = !arp;
  $("hold-controls").hidden = !["arp", "hold"].includes(p.mode);
  $("arp-hold-label").hidden = p.mode !== "arp";
  $("sequence-controls").hidden = !sequence;
  $("sequence-steps").hidden = !sequence;
  $("sequence-gate-label").hidden = p.mode === "sequencer_arp";
  $("sequence-arp-hint").hidden = p.mode !== "sequencer_arp";
  for (const [id, field] of [
    ["arp-direction", "direction"],
    ["arp-division", "arp_division"],
    ["arp-octaves", "octaves"],
    ["arp-gate", "arp_gate"],
    ["sequence-division", "sequence_division"],
    ["sequence-gate", "sequence_gate"],
    ["sequence-length", "length"],
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
  $("sequence-gate-value").textContent = percent(p.sequence_gate);
  if (document.activeElement !== $("performance-chord"))
    $("performance-chord").value = p.chord.map(noteName).join(" ");
  for (let i = 0; i < sequenceCards.length; i++) {
    const { card, toggle, note } = sequenceCards[i],
      step = p.steps[i];
    card.classList.toggle("rest", !step.enabled);
    card.classList.toggle("outside", i >= p.length);
    toggle.setAttribute("aria-pressed", String(step.enabled));
    const names = step.notes.map(noteName);
    note.replaceChildren(
      ...names.map((name) => {
        const label = document.createElement("span");
        label.textContent = name;
        return label;
      }),
    );
    if (!names.length) note.textContent = "—";
    note.title = names.join(" · ") || "Rest";
    note.setAttribute(
      "aria-label",
      `Step ${i + 1} notes, ${names.join(" ") || "rest"}`,
    );
  }
  syncStepEntry();
}
function queueStepEntry(step, reset = false, clear = false) {
  noteChain = noteChain
    .then(async () => {
      if (pending && !reset) {
        clearTimeout(commitTimer);
        const version = pending;
        await request("/api/preset", preset);
        if (pending === version) pending = 0;
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
  sequenceCards.forEach(({ card, note }, i) => {
    card.classList.toggle("armed", active && entry === i);
    note.setAttribute("aria-pressed", String(active && entry === i));
  });
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
    ["sequence-division", "sequence_division", true],
    ["sequence-gate", "sequence_gate", true],
    ["sequence-length", "length", true],
  ])
    $(id).oninput = (e) => {
      let value = numeric ? Number(e.target.value) : e.target.value;
      if (numeric && (!Number.isFinite(value) || e.target.value === "")) return;
      if (field === "length")
        value = Math.max(1, Math.min(16, Math.round(value)));
      preset.performance[field] = value;
      syncPerformance();
      schedulePreset();
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
    if (pending) {
      clearTimeout(commitTimer);
      await request("/api/preset", preset).catch((e) => notify(e.message));
      pending = 0;
    }
    await transport({ clear_hold: true });
  };
  $("reset-sequence").onclick = async () => {
    clearTimeout(commitTimer);
    pending = 0;
    await queueStepEntry(null, true);
    const defaults = presets[0].performance;
    preset.performance.steps = structuredClone(defaults.steps);
    preset.performance.length = defaults.length;
    syncPerformance();
    schedulePreset();
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
  const response = await api("/api/wav", record.blob, {
    "Content-Type": "audio/wav",
    "X-File-Name": encodeURIComponent(
      record.factory ? record.displayName : record.name,
    ),
    "X-Library-Id": record.id,
  });
  const info = await response.json();
  await request("/api/fades", await fileLibrary.fades(record.id));
  return info;
}
async function loadStoredPreset(loaded) {
  clearTimeout(commitTimer);
  pending = 0;
  await queueStepEntry(null, true);
  preset = await request("/api/preset", loaded);
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
  $("antialias").onchange = (e) => setParam("antialiasing", e.target.checked);
  $("bpm").oninput = (e) =>
    setParam("bpm", Math.max(20, Math.min(300, Number(e.target.value) || 120)));
  $("preset").onchange = async (e) => {
    clearTimeout(commitTimer);
    pending = 0;
    await queueStepEntry(null, true);
    preset = structuredClone(presets[Number(e.target.value)]);
    syncAll();
    schedulePreset();
  };
  $("save-preset").onclick = async () => {
    try {
      const name = `${preset.name.toLowerCase().replace(/[^a-z0-9]+/g, "-") || "sad-pet"}.sad.json`;
      const saved = await fileLibrary.savePreset(preset, name);
      download(saved.blob, saved.name);
    } catch (error) {
      notify(error.message, true);
    }
  };
  $("load-preset").onclick = () => $("preset-file").click();
  $("preset-file").onchange = async (e) => {
    const file = e.target.files[0];
    if (!file) return;
    try {
      await fileLibrary.importFiles([file]);
    } catch (error) {
      notify(error.message);
    }
    e.target.value = "";
  };
  $("load-wav").onclick = () => $("wav-file").click();
  $("wav-file").onchange = (e) => loadWav(e.target.files[0]);
  const drop = $("drop-zone");
  for (const type of ["dragenter", "dragover"])
    drop.addEventListener(type, (e) => {
      e.preventDefault();
      drop.classList.add("drag");
    });
  for (const type of ["dragleave", "drop"])
    drop.addEventListener(type, (e) => {
      e.preventDefault();
      drop.classList.remove("drag");
    });
  drop.addEventListener("drop", (e) =>
    fileLibrary
      .importFiles([...e.dataTransfer.files])
      .catch((error) => notify(error.message, true)),
  );
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
      if (pending) {
        clearTimeout(commitTimer);
        await request("/api/preset", preset);
        pending = 0;
        await new Promise((resolve) => setTimeout(resolve, 70));
      }
      await sampleSelection?.flush();
      const response = await api("/api/render", {
        seconds: Number($("render-duration").value),
      });
      const blob = await response.blob();
      const seconds = Number($("render-duration").value);
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
      e.target.matches("input,select,textarea,[role=slider]") ||
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
function resizeCanvas(canvas) {
  const r = canvas.getBoundingClientRect(),
    dpr = Math.min(devicePixelRatio || 1, 2),
    w = Math.round(r.width * dpr),
    h = Math.round(r.height * dpr);
  if (canvas.width !== w || canvas.height !== h) {
    canvas.width = w;
    canvas.height = h;
  }
  const c = canvas.getContext("2d");
  c.setTransform(dpr, 0, 0, dpr, 0, 0);
  return [c, r.width, r.height];
}
function draw() {
  if (state) {
    const [c, w, h] = resizeCanvas($("wave"));
    c.clearRect(0, 0, w, h);
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
    sampleSelection?.draw(c, w, h);
    for (const grain of state.grains) {
      const x = grain.position * w,
        alpha = Math.sin(Math.PI * grain.age) * 0.6 + 0.15;
      c.strokeStyle = `rgba(203,255,148,${alpha})`;
      c.beginPath();
      c.moveTo(x, 8);
      c.lineTo(x, h - 8);
      c.stroke();
      c.fillStyle = `rgba(219,255,182,${alpha})`;
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
    for (let i = 0; i < scopes.length; i++) {
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
  requestAnimationFrame(draw);
}
let failures = 0;
let inputDevicesKey = "";
function syncInput(input) {
  if (!input) return;
  const active = input.active;
  $("source-file").setAttribute(
    "aria-pressed",
    String(!active && !input.pending),
  );
  $("source-mic").setAttribute("aria-pressed", String(active || input.pending));
  $("source-mic").textContent = input.pending
    ? "Cancel microphone"
    : "Microphone";
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
  $("input-listen").setAttribute("aria-pressed", String(!!input.monitor));
  $("input-listen").textContent = input.monitor ? "Listen on" : "Listen off";
  $("sample-select").hidden = active;
  $("live-source-name").hidden = !active;
  $("live-source-name").textContent = input.name || "Microphone";
  $("input-status").classList.toggle("live", active);
  const peak = Math.max(...(input.peak || [0, 0]));
  $("input-level").style.width = `${Math.min(1, peak) * 100}%`;
  $("input-level").parentElement.setAttribute(
    "aria-valuenow",
    String(Math.round(Math.min(1, peak) * 100)),
  );
  if (active) $("render").disabled = true;
  else if ($("render").dataset.liveDisabled === "true")
    $("render").disabled = false;
  $("render").dataset.liveDisabled = String(active);
  $("render").title = active ? "Use Record to save the live output" : "";
  const effectsOnly = active && input.mode === "effects";
  $("grain-controls").inert = effectsOnly;
  $("grain-controls").classList.toggle("inactive", effectsOnly);
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
async function poll() {
  try {
    const next = await request("/api/state");
    const initial = state === null;
    state = next;
    connected = true;
    failures = 0;
    if (initial) {
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
      (!pending &&
        !editing &&
        !document.activeElement?.matches("input,select") &&
        JSON.stringify(preset) !== JSON.stringify(next.preset))
    ) {
      preset = structuredClone(next.preset);
      syncAll();
    }
    $("sample-name").textContent = next.sample_name;
    fileLibrary?.observe(next);
    $("sample-info").textContent = `${next.sample_seconds.toFixed(2)} s`;
    syncInput(next.input);
    sampleSelection?.sync(next);
    $("sample-end").textContent = formatTime(next.sample_seconds);
    $("position-info").textContent = formatTime(
      next.center * next.sample_seconds,
    );
    $("grain-count").textContent = next.active_grains;
    $("play").setAttribute("aria-pressed", String(next.playing));
    $("play-icon").textContent = next.playing ? "Ⅱ" : "▶";
    $("play-label").textContent = next.playing ? "Pause" : "Play";
    for (const key of ["freeze", "bypass"])
      $(key).setAttribute(
        "aria-pressed",
        String(next[key === "freeze" ? "frozen" : key]),
      );
    $("meter-l").style.width = `${Math.min(1, next.peak[0]) * 100}%`;
    $("meter-r").style.width = `${Math.min(1, next.peak[1]) * 100}%`;
    $("audio-status").textContent =
      window.sadPetAudio?.status() ||
      (next.audio_enabled
        ? `${(next.sample_rate / 1000).toFixed(1)} kHz · stereo`
        : "Audio output disabled");
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
    $("cpu").textContent = `DSP ${next.cpu_percent.toFixed(1)}%`;
    if (next.audio_error) notify(next.audio_error, true);
    for (let i = 0; i < 3; i++) {
      const a = scopes[i].values;
      a.push(next.lfos[i]);
      if (a.length > 80) a.shift();
      envCards[i].card.querySelector(".env-level").textContent = percent(
        next.envs[i],
      );
    }
    for (const key of document.querySelectorAll(".key"))
      key.classList.toggle(
        "active",
        held.has(Number(key.dataset.note)) ||
          next.notes.includes(Number(key.dataset.note)),
      );
    for (let i = 0; i < sequenceCards.length; i++)
      sequenceCards[i].card.classList.toggle(
        "current",
        next.playing && next.performance.step === i,
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
    }
  } finally {
    setTimeout(poll, 80);
  }
}
async function init() {
  try {
    presets = await request("/api/presets");
    colorDefaults = await request("/api/color-defaults");
    preset = structuredClone(presets[0]);
    $("preset").replaceChildren(
      ...presets.map((p, i) => option(String(i), p.name)),
    );
    for (const s of grainSchema) createKnob(s, "grain-controls");
    buildFilters();
    buildColors();
    buildLfos();
    buildEnvs();
    buildMatrix();
    buildSequence();
    buildKeyboard();
    bind();
    sampleSelection = new window.SadSampleSelection({
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
    });
    request("/api/state")
      .then((native) => fileLibrary.initialize(native, presets))
      .catch((error) => notify(`Library: ${error.message}`, true));
    refreshMidi();
    poll();
    draw();
  } catch (e) {
    notify(`Cannot connect to SAD PET: ${e.message}`, true);
  }
}
init();
