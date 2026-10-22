// The SAD PET app renders, records, imports and exports all audio natively.
// This bridge carries editor commands and engine state; it refuses sample data
// in both directions, and file choices happen in native dialogs.
const native = window.sadPetNativeRequest ||
  (await import("./juce.js")).getNativeFunction("sadRequest");
const originalFetch = window.fetch.bind(window);
const NO_AUDIO = "The SAD PET app handles audio files natively.";
let latest = null;

const call = async (request) => {
  const envelope = await native(request);
  if (envelope?.data?.binary !== undefined) throw new Error(NO_AUDIO);
  return envelope;
};

window.sadPetNative = {
  // Shows a native file dialog; the app then runs the request with the chosen
  // file. Resolves to { cancelled: true } when the dialog is dismissed.
  async dialog(path, data, dialog) {
    const envelope = await call({ path, method: "POST", data, dialog });
    if (envelope.status !== 200) throw new Error(envelope.data?.error || "Request failed");
    return envelope.data;
  },
  settings: () => call({ path: "/api/app/audio-settings", method: "POST", data: {} }),
  log: (message) =>
    call({ path: "/api/app/log", method: "POST", data: { message: String(message) } }).catch(() => {}),
};

window.sadPetFetch = async (path, options = {}) => {
  const url = new URL(path, location.href);
  if (!url.pathname.startsWith("/api/")) return originalFetch(path, options);
  const body = options.body;
  if (body instanceof Blob || body instanceof ArrayBuffer || ArrayBuffer.isView(body))
    throw new Error(NO_AUDIO);
  const envelope = await call({
    path: url.pathname + url.search,
    method: options.method || "GET",
    data: body ? JSON.parse(body) : null,
  });
  if (url.pathname === "/api/state" && envelope.status === 200) latest = envelope.data;
  return new Response(JSON.stringify(envelope.data), {
    status: envelope.status,
    headers: { "Content-Type": "application/json" },
  });
};

window.sadPetAudio = {
  enable: async () => {},
  status: () => {
    if (!latest) return "Audio device";
    if (!latest.audio_enabled) return latest.audio_device || "No audio device";
    return `${latest.audio_device} · ${(latest.sample_rate / 1000).toFixed(1)} kHz${latest.audio_input ? " · input on" : ""}`;
  },
};

window.addEventListener("error", (event) => window.sadPetNative.log(event.message));
window.addEventListener("unhandledrejection", (event) =>
  window.sadPetNative.log(event.reason?.message || event.reason));

document.documentElement.classList.add("standalone");
window.sadPetStandalone = true;
document.querySelector(".brand h1 span").textContent = "APP";
document.getElementById("browser-input").hidden = false;
document.getElementById("source-mic").textContent = "Microphone";
document.getElementById("input-device").replaceChildren(new Option("Audio input", ""));

const settingsButton = (label, title) => {
  const button = document.createElement("button");
  button.className = "text-button audio-settings";
  button.textContent = label;
  button.title = title;
  button.onclick = () => window.sadPetNative.settings();
  return button;
};
document.querySelector(".runtime-status")
  .append(settingsButton("Audio/MIDI…", "Choose the audio device, sample rate, buffer size and MIDI inputs"));
const midiLabel = document.createElement("span");
midiLabel.className = "midi-summary";
midiLabel.textContent = "All MIDI inputs";
document.querySelector(".midi-controls")
  .append(midiLabel, settingsButton("Settings…", "Choose MIDI inputs and the audio device"));

for (const name of ["library", "native-library", "selection", "presets", "sequence", "app"]) {
  await new Promise((resolve, reject) => {
    const script = document.createElement("script");
    script.src = `./${name}.js`;
    script.onload = resolve;
    script.onerror = () => reject(new Error(`Cannot load ${name}`));
    document.head.append(script);
  });
}

// Persistent notices are errors; the app log keeps them for diagnosis.
const notify = window.notify;
window.notify = (message, persistent = false) => {
  if (persistent) window.sadPetNative.log(message);
  return notify(message, persistent);
};

// Files dropped on the window are imported by the app before this event.
window.__JUCE__?.backend.addEventListener("sadLibraryImported", (envelope) => {
  const library = window.sadPetLibrary;
  if (envelope?.status !== 200) window.notify(envelope?.data?.error || "Cannot import the files.", true);
  else library?.imported(envelope.data).catch((error) => library.error(error));
});
