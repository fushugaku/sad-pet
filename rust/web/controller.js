"use strict";
// Browser-only control surface row. The instrument's normal edit queue owns
// all changes, including selecting a timbre while it has unsent preset edits.
window.SadController = (() => {
  let hooks, busy = false, connected = false, ports = "", rows = "", lastData, preparing;
  const $ = id => document.getElementById(id);
  const option = (value, text) => new Option(text, value);
  const text = (id, value) => { const node = $(id); if (node.textContent !== value) node.textContent = value; };
  const selectValue = (id, value) => { const node = $(id); if (node.value !== value) node.value = value; };
  function syncPages() {
    if (!hooks || !window.SadMidiLearn?.pages) return;
    const pages = window.SadMidiLearn.pages(), current = window.SadMidiLearn.page();
    const select = $("controller-page");
    if (select.options.length !== pages.length)
      select.replaceChildren(...pages.map(p => option(String(p.id), `Page ${p.id + 1}`)));
    selectValue("controller-page", String(current));
    $("controller-page-prev").disabled = current === 0;
    $("controller-page-next").disabled = current === pages.length - 1;
    const tracks = window.SadMidiLearn.tracks(), track = window.SadMidiLearn.track();
    const trackSelect = $("controller-mapping-track");
    if (trackSelect.options.length !== tracks.length)
      trackSelect.replaceChildren(...tracks.map(t => option(String(t.id), `Track ${t.id + 1}`)));
    selectValue("controller-mapping-track", String(track));
    $("controller-track-prev").disabled = track === 0;
    $("controller-track-next").disabled = track === tracks.length - 1;
    const count = pages[current].count;
    text("controller-page-count", count ? `${count} assignment${count === 1 ? "" : "s"}` : "No assignments");
  }
  function sync(data) {
    if (!hooks || !data) return;
    lastData = data;
    connected = !!data.connected;
    $("controller-led").classList.toggle("off", !connected);
    const signature = JSON.stringify([data.ports, data.device]);
    if (signature !== ports) {
      ports = signature;
      const select = $("controller-device");
      select.replaceChildren(option("auto", "Launch Control XL · Auto"), option("", "Off"));
      for (const p of data.ports) select.append(option(p.id, p.name));
      if (data.device && data.device !== "auto" && !data.ports.some(p => p.id === data.device))
        select.append(option(data.device, "Saved controller · offline"));
    }
    if (!busy) {
      selectValue("controller-device", data.device ?? "");
      selectValue("controller-mode", data.mode);
      $("controller-connect").disabled = !data.supported;
      text("controller-connect", data.feedbackAvailable && !data.feedbackPermission ? "Enable feedback"
        : data.access ? "Refresh" : "Connect");
    }
    text("controller-status", data.error || (!data.supported
      ? "Web MIDI needs Chrome or Edge on HTTPS or localhost"
      : data.device === null ? "Disconnected"
      : !data.access ? "Connect USB, then allow MIDI access"
      : !connected ? "Waiting for Launch Control XL"
      : data.mode !== "mixer" ? `${data.connected} · ${data.feedback ? "RGB + display" : "right-click a knob → MIDI learn"}`
      : data.pickup || `${data.connected} · Bank ${data.bank + 1}/${data.banks}`));
    text("controller-template", `${data.template}. Faders and knobs take over when they reach the value in the project.`);
    $("controller-map").hidden = data.mode !== "mixer";
    $("controller-pages").hidden = data.mode === "mixer";
    $("controller-learn-hint").hidden = data.mode === "mixer";
    text("controller-learn-hint", data.mode === "daw"
      ? "Mode + 1–16: page · Track ◀/▶: 1–6 · Shift + turn: ×10 · Play: start/stop · Record: record/save."
      : data.mode === "native"
      ? "Custom Modes: relative knobs, 14-bit faders; fixed colours. For live lights and OLED, choose 16 pages + RGB."
      : "Upper buttons: pages 1–8 · lower: 9–16 · ← / → changes page.");
    syncPages();
    text("controller-send-names", data.sends.length ? data.sends.join(" · ") : "Add a Bus timbre to use sends");
    const trackKey = JSON.stringify([data.bank, data.tracks]);
    if (trackKey !== rows) {
      rows = trackKey;
      $("controller-tracks").replaceChildren(...data.tracks.map((p, i) => {
        const node = document.createElement("span");
        node.textContent = `${i + 1} · ${p.name}`;
        return node;
      }));
    }
  }
  async function perform(path, data) {
    if (busy) return;
    busy = true;
    $("controller-connect").disabled = true;
    try {
      const result = await hooks.request(path, data);
      busy = false;
      sync(result);
    } catch (error) {
      $("controller-status").textContent = error.message;
      hooks.notify(error.message, true);
    } finally {
      busy = false;
      $("controller-connect").disabled = !navigator.requestMIDIAccess;
    }
  }
  return {
    setup(next) {
      if (!window.sadPetAudio || window.sadPetControlSource) return;
      hooks = next;
      $("controller-controls").hidden = false;
      $("controller-connect").onclick = () => perform("/api/controller?request=1");
      $("controller-device").onchange = e => perform("/api/controller", { device: e.target.value || null });
      $("controller-mode").onchange = e => perform("/api/controller", { mode: e.target.value });
      $("controller-page").onchange = e => perform("/api/controller", { page: Number(e.target.value) });
      $("controller-page-prev").onclick = () => perform("/api/controller", { page: window.SadMidiLearn.page() - 1 });
      $("controller-page-next").onclick = () => perform("/api/controller", { page: window.SadMidiLearn.page() + 1 });
      $("controller-mapping-track").onchange = e => perform("/api/controller", { mappingTrack: Number(e.target.value) });
      $("controller-track-prev").onclick = () => perform("/api/controller", { mappingTrack: window.SadMidiLearn.track() - 1 });
      $("controller-track-next").onclick = () => perform("/api/controller", { mappingTrack: window.SadMidiLearn.track() + 1 });
      window.addEventListener("sad-midi-change", () => perform("/api/controller"));
      perform("/api/controller");
    },
    sync,
    syncPages,
    prepareLearn() {
      if (!hooks || !navigator.requestMIDIAccess || (lastData?.access && !lastData.xl3Available)) return;
      if (!preparing) preparing = hooks.request("/api/controller?learn=1")
        .then(sync).catch(error => {
          text("controller-status", error.message); hooks.notify(error.message, true);
        }).finally(() => { preparing = null; });
      return preparing;
    },
    context: () => hooks?.context(),
    change: (...args) => hooks.change(...args),
    transport: action => hooks?.transport?.(action),
    transportState: () => hooks?.transportState?.(),
    notify: (...args) => hooks?.notify(...args),
    get connected() { return connected; },
  };
})();
