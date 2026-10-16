// The native host renders the audio. This bridge only carries editor commands.
const native = window.sadPetNativeRequest ||
  (await import("./juce.js")).getNativeFunction("sadRequest");
const originalFetch = window.fetch.bind(window);
let latest = null;

function base64(bytes) {
  let text = "";
  for (let i = 0; i < bytes.length; i += 32768)
    text += String.fromCharCode(...bytes.subarray(i, i + 32768));
  return btoa(text);
}

function updateHost(state) {
  latest = state;
  const tempo = document.getElementById("bpm");
  if (tempo) {
    tempo.disabled = state.plugin.sync;
    tempo.title = state.plugin.sync ? "Tempo from DAW" : "Tempo";
  }
  const sync = document.getElementById("host-sync");
  if (sync) {
    sync.setAttribute("aria-pressed", String(state.plugin.sync));
    sync.textContent = state.plugin.sync ? `DAW · ${state.plugin.host_bpm.toFixed(1)} BPM` : "Free tempo";
  }
}

window.sadPetFetch = async (path, options = {}) => {
  const url = new URL(path, location.href);
  if (!url.pathname.startsWith("/api/")) return originalFetch(path, options);
  const method = options.method || "GET";
  let data = null;
  if (options.body instanceof Blob || options.body instanceof ArrayBuffer || ArrayBuffer.isView(options.body)) {
    const bytes = options.body instanceof Blob
      ? new Uint8Array(await options.body.arrayBuffer())
      : new Uint8Array(options.body.buffer || options.body);
    const headers = new Headers(options.headers);
    data = {
      base64: base64(bytes),
      name: decodeURIComponent(headers.get("X-File-Name") || "Sample.wav"),
      id: headers.get("X-Library-Id"),
    };
  } else if (options.body) {
    data = JSON.parse(options.body);
  }
  const envelope = await native({ path: url.pathname + url.search, method, data });
  if (url.pathname === "/api/state" && envelope.status === 200) updateHost(envelope.data);
  if (envelope.data?.binary) {
    const text = atob(envelope.data.binary);
    const bytes = Uint8Array.from(text, (c) => c.charCodeAt(0));
    return new Response(bytes, { status: envelope.status, headers: { "Content-Type": envelope.data.mime } });
  }
  return new Response(JSON.stringify(envelope.data), { status: envelope.status, headers: { "Content-Type": "application/json" } });
};

window.sadPetAudio = {
  enable: async () => {},
  status: () => latest ? `${(latest.sample_rate / 1000).toFixed(1)} kHz · DAW` : "DAW",
};

document.documentElement.classList.add("plugin");
window.sadPetPlugin = true;
const initial = await (await window.sadPetFetch("/api/state")).json();
if (initial.plugin.effect) document.documentElement.classList.add("plugin-fx");
document.querySelector(".brand h1 span").textContent = initial.plugin.effect ? "FX" : "INSTRUMENT";
document.querySelector(".brand p").textContent = initial.plugin.effect ? "Granular audio effect" : "Granular instrument";
document.getElementById("source-mic").textContent = "DAW input";
document.getElementById("live-source-name").textContent = "DAW input";
document.getElementById("browser-input").hidden = !initial.plugin.effect;
document.getElementById("input-device").replaceChildren(new Option("DAW input", ""));
document.getElementById("input-mode").setAttribute("aria-label", "DAW input processing");

const hostSync = document.createElement("button");
hostSync.id = "host-sync";
hostSync.title = "Follow the DAW tempo and transport";
hostSync.onclick = async () => {
  await window.sadPetFetch("/api/transport", { method: "POST", body: JSON.stringify({ sync: !latest.plugin.sync }) });
};
document.querySelector("footer .render-controls").append(hostSync);
const midiLabel = document.createElement("span");
midiLabel.textContent = "DAW MIDI";
document.querySelector(".midi-controls").append(midiLabel);
updateHost(initial);

for (const name of ["library", "selection", "app"]) {
  await new Promise((resolve, reject) => {
    const script = document.createElement("script");
    script.src = `./${name}.js`;
    script.onload = resolve;
    script.onerror = () => reject(new Error(`Cannot load ${name}`));
    document.head.append(script);
  });
}
