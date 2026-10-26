// MIDI clock: follow another instrument's tempo, song position, Start and
// Stop from the chosen MIDI input, and send this scene's clock to a MIDI
// output. The row shows only where the backend answers /api/clock.
(() => {
  const STORE = "sad-pet-clock";
  const $ = (id) => document.getElementById(id);
  let host = null;
  let supported = null;
  let following = false;
  let saved = {};
  try {
    saved = JSON.parse(localStorage.getItem(STORE)) || {};
  } catch {}
  // This computer's choices, kept across visits.
  function remember(patch) {
    saved = { ...saved, ...patch };
    try {
      localStorage.setItem(STORE, JSON.stringify(saved));
    } catch {}
  }
  function option(value, label) {
    const element = document.createElement("option");
    element.value = value;
    element.textContent = label;
    return element;
  }
  async function apply(data) {
    try {
      render(await host.request("/api/clock", data));
    } catch (error) {
      host.notify(`MIDI clock: ${error.message}`);
    }
  }
  function render(summary) {
    supported = summary.supported;
    $("clock-controls").hidden = !supported?.follow && !supported?.send;
    $("clock-follow").checked = summary.follow;
    $("clock-follow").parentElement.hidden = !supported?.follow;
    $("clock-lead").parentElement.hidden = !supported?.follow;
    if (document.activeElement !== $("clock-lead"))
      $("clock-lead").value = Math.round(summary.lead_ms);
    const out = $("clock-out");
    out.hidden = !supported?.send;
    out.replaceChildren(
      option("", "No clock out"),
      ...(summary.outputs || []).map((port) => option(port.id, port.name)),
    );
    out.value = summary.out ?? "";
  }
  /** Picks the output chosen last time once the ports are listed. */
  function restoreOutput(summary) {
    if (!saved.out || summary.out) return;
    const port = summary.outputs?.find((p) => p.name === saved.out);
    if (port) return apply({ out: port.id });
  }
  async function setup(options) {
    host = options;
    $("clock-follow").onchange = (e) => {
      remember({ follow: e.target.checked });
      apply({ follow: e.target.checked });
    };
    $("clock-lead").onchange = (e) => {
      const lead = Math.max(-500, Math.min(500, Math.round(Number(e.target.value) || 0)));
      remember({ lead_ms: lead });
      apply({ lead_ms: lead });
    };
    $("clock-out").onchange = (e) => {
      const port = e.target.selectedOptions[0];
      remember({ out: e.target.value ? port.textContent : null });
      apply({ out: e.target.value || null });
    };
    let summary;
    try {
      summary = await host.request("/api/clock");
    } catch {
      // A DAW's transport or a build without MIDI clock.
      $("clock-controls").hidden = true;
      return;
    }
    const restore = {};
    if (saved.follow && !summary.follow) restore.follow = true;
    if (saved.lead_ms && saved.lead_ms !== summary.lead_ms)
      restore.lead_ms = saved.lead_ms;
    if (Object.keys(restore).length)
      summary = await host.request("/api/clock", restore).catch(() => summary);
    render(summary);
    await restoreOutput(summary);
  }
  /** New MIDI ports, such as after the browser grants MIDI access. */
  async function refresh() {
    if (!supported) return;
    try {
      const summary = await host.request("/api/clock");
      render(summary);
      await restoreOutput(summary);
    } catch {}
  }
  function sync(state) {
    const clock = state.clock;
    if (!clock || !supported) return;
    const present = clock.follow && clock.present;
    let text = "";
    if (clock.follow)
      text = present
        ? `${clock.running ? "Playing" : "Stopped"} · ${clock.tempo ? `${clock.tempo.toFixed(1)} BPM` : "…"}`
        : host.hasInput()
          ? "Waiting for clock"
          : "Choose a MIDI input to follow";
    if (clock.sending)
      text = text ? `${text} · out to ${clock.sending}` : `Sending to ${clock.sending}`;
    if (clock.error) text = clock.error;
    if ($("clock-status").textContent !== text) $("clock-status").textContent = text;
    $("clock-led").classList.toggle("off", !(present || clock.sending));
    // The master owns the tempo and transport while it is followed.
    const bpm = $("bpm");
    if (present && clock.tempo && document.activeElement !== bpm) {
      const shown = clock.tempo.toFixed(1);
      if (bpm.value !== shown) bpm.value = shown;
    }
    if (present !== following) {
      following = present;
      bpm.disabled = present;
      $("play").disabled = present;
      const note = present ? "Following the MIDI clock" : "";
      bpm.title = note;
      $("play").title = note;
      if (!present) host.released();
    }
  }
  window.SadMidiClock = { setup, refresh, sync };
})();
