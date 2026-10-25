"use strict";
// MIDI learn: right-click a knob or slider, choose MIDI learn and move a
// controller; from then on that controller turns the knob. A mapping names the
// knob, not a timbre, so the controller turns that knob on whichever timbre the
// editor shows. Mappings stay in this browser (or app) for every scene and
// preset. The same menu holds the knob's modulation (hooks.menu).
window.SadMidiLearn = (() => {
  const STORAGE = "sad-pet-midi-map";
  // CC 120–127 are channel mode messages (all notes off, reset…).
  const valid = (channel, controller) =>
    Number.isInteger(channel) && channel >= 0 && channel < 16 &&
    Number.isInteger(controller) && controller >= 0 && controller < 120;
  /** knob key → [channel, controller] */
  const map = new Map();
  try {
    for (const [key, cc] of Object.entries(JSON.parse(localStorage.getItem(STORAGE) || "{}")))
      if (Array.isArray(cc) && valid(cc[0], cc[1])) map.set(key, [cc[0], cc[1]]);
  } catch {
    // Without storage the mappings last until the editor closes.
  }
  /** "channel:controller" → knob key */
  const keys = new Map();
  const index = () => {
    keys.clear();
    for (const [key, [channel, controller]] of map) keys.set(`${channel}:${controller}`, key);
  };
  index();
  /** knob key → the knobs showing it: { dial, label, apply(unit) } */
  const knobs = new Map();
  let hooks = { notify() {}, hint: () => "", menu: () => null },
    learning = null,
    attached = 0;
  const name = ([channel, controller]) => `CC ${controller} · ch ${channel + 1}`;
  function save() {
    index();
    try {
      localStorage.setItem(STORAGE, JSON.stringify(Object.fromEntries(map)));
    } catch {
      // Storage may be full or blocked; the mapping still works now.
    }
  }
  function paint(key) {
    const cc = map.get(key);
    for (const { dial } of knobs.get(key) || []) {
      const box = dial.closest(".knob") || dial;
      box.classList.toggle("midi-mapped", !!cc);
      box.classList.toggle("midi-learning", learning === key);
      // A slider holds no children: its badge follows it.
      const slider = dial.tagName === "INPUT",
        next = dial.nextElementSibling;
      let badge = slider
        ? next?.classList.contains("knob-midi") ? next : null
        : dial.querySelector(".knob-midi");
      if (!cc) {
        badge?.remove();
        delete dial.dataset.midi;
        continue;
      }
      if (!badge) {
        badge = document.createElement("span");
        badge.className = slider ? "knob-midi slider-midi" : "knob-midi";
        badge.setAttribute("aria-hidden", "true");
        slider ? dial.after(badge) : dial.append(badge);
      }
      badge.textContent = String(cc[1]);
      dial.dataset.midi = name(cc);
    }
  }
  function label(key) {
    const shown = [...(knobs.get(key) || [])];
    return (shown.findLast((k) => k.dial.isConnected) ?? shown.at(-1))?.label ?? key;
  }
  // Rebuilt panels leave their old knobs behind; forget those now and then.
  function sweep() {
    for (const [key, set] of knobs) {
      for (const knob of set) if (!knob.dial.isConnected) set.delete(knob);
      if (!set.size) knobs.delete(key);
    }
  }
  /** createKnob registers every knob, and sliders register themselves;
   * apply(unit) sets it from 0–1. */
  function attach(dial, key, text, apply) {
    const knob = { dial, label: text, apply };
    const set = knobs.get(key) || new Set();
    for (const old of set) if (!old.dial.isConnected) set.delete(old);
    set.add(knob);
    knobs.set(key, set);
    if (++attached % 512 === 0) sweep();
    dial.addEventListener("contextmenu", (event) => {
      event.preventDefault();
      open(key, dial, event);
    });
    if (map.has(key) || learning === key) paint(key);
  }
  function escape(event) {
    if (event.key !== "Escape") return;
    event.preventDefault();
    event.stopPropagation();
    cancel();
  }
  function learn(key) {
    const previous = learning;
    learning = key;
    if (previous !== null) paint(previous);
    paint(key);
    document.addEventListener("keydown", escape, true);
    const hint = hooks.hint();
    hooks.notify(`MIDI learn: move a controller to turn ${label(key)} · Esc cancels${hint ? ` · ${hint}` : ""}`, true);
  }
  function stop() {
    const key = learning;
    learning = null;
    document.removeEventListener("keydown", escape, true);
    if (key !== null) paint(key);
  }
  function cancel() {
    if (learning === null) return;
    stop();
    hooks.notify("MIDI learn cancelled");
  }
  function forget(key) {
    if (!map.delete(key)) return;
    save();
    paint(key);
  }
  function forgetAll() {
    const mapped = [...map.keys()];
    map.clear();
    save();
    mapped.forEach(paint);
    hooks.notify(`Forgot ${mapped.length} MIDI mapping${mapped.length === 1 ? "" : "s"}`);
  }
  /** A controller moved: learn it, then turn the knob it is mapped to. */
  function input(channel, controller, value) {
    if (!valid(channel, controller) || !Number.isInteger(value) || value < 0 || value > 127) return;
    let key = keys.get(`${channel}:${controller}`);
    if (learning !== null) {
      const target = learning;
      stop();
      // One controller turns one knob: learning moves it from the previous one.
      if (key !== undefined && key !== target) {
        map.delete(key);
        paint(key);
      }
      map.set(target, [channel, controller]);
      save();
      paint(target);
      hooks.notify(`${name([channel, controller])} → ${label(target)}`);
      key = target;
    }
    if (key === undefined) return;
    for (const knob of knobs.get(key) || []) if (knob.dial.isConnected) knob.apply(value / 127);
  }
  /** One Web MIDI message. */
  function bytes(data) {
    if (data?.length >= 3 && (data[0] & 0xf0) === 0xb0) input(data[0] & 15, data[1], data[2]);
  }

  // The knob's menu: MIDI learn, then its modulation. Rows are values the
  // menu hands back; MIDI's own start with "midi:".
  function open(key, dial, event) {
    const cc = map.get(key),
      actions = new Map(),
      item = (text, run) => {
        const value = `midi:${actions.size}`;
        actions.set(value, run);
        return [value, text];
      };
    const choices = [
      learning === key ? item("Cancel MIDI learn", cancel) : item("MIDI learn", () => learn(key)),
    ];
    if (cc) choices.push(item(`Forget ${name(cc)}`, () => forget(key)));
    if (map.size > (cc ? 1 : 0))
      choices.push(item(`Forget all MIDI mappings (${map.size})`, forgetAll));
    const more = hooks.menu(key, label(key)),
      // A keyboard context menu has no pointer position: open by the knob.
      pointer = event.clientX || event.clientY;
    window.SadLazySelect.popup(
      dial,
      [{ choices }, ...(more?.groups || [])],
      (value) => (actions.has(value) ? actions.get(value)() : more?.choose(value)),
      {
        title: label(key),
        point: pointer ? [event.clientX, event.clientY] : null,
        checked: (value) => !!more?.checked?.(value),
      },
    );
  }

  return {
    attach,
    input,
    bytes,
    learn,
    cancel,
    forget,
    setup: (next) => (hooks = { ...hooks, ...next }),
    learning: () => learning,
    mappings: () => Object.fromEntries(map),
  };
})();
