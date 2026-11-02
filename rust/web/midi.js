"use strict";
// MIDI learn: right-click a knob or slider, choose MIDI learn and move a
// controller; from then on that controller turns the knob. A mapping names the
// knob, not a timbre, so the controller turns that knob on whichever timbre the
// editor shows. Mappings stay in this browser (or app) for every scene and
// preset. Sixteen independent pages reuse the same physical controls. The
// same menu holds the knob's modulation (hooks.menu).
window.SadMidiLearn = (() => {
  const STORAGE = "sad-pet-midi-map";
  const PAGE_STORAGE = "sad-pet-midi-pages", PAGE_COUNT = 16;
  // CC 120–127 are channel mode messages (all notes off, reset…).
  const valid = (channel, controller) =>
    Number.isInteger(channel) && channel >= 0 && channel < 16 &&
    Number.isInteger(controller) && controller >= 0 && controller < 120;
  const validPage = value => Number.isInteger(value) && value >= 0 && value < PAGE_COUNT;
  /** Each page: knob key → [channel, controller, optional format]. */
  const pages = Array.from({ length: PAGE_COUNT }, () => new Map());
  let page = 0;
  const load = (target, value) => {
    if (!value || typeof value !== "object" || Array.isArray(value)) return;
    for (const [key, cc] of Object.entries(value))
      if (Array.isArray(cc) && valid(cc[0], cc[1])) target.set(key,
        [cc[0], cc[1], ...(["relative", "14bit"].includes(cc[2]) ? [cc[2]] : [])]);
  };
  try {
    const saved = JSON.parse(localStorage.getItem(PAGE_STORAGE) || "null");
    if (saved?.version === 1 && Array.isArray(saved.pages)) {
      pages.forEach((target, i) => load(target, saved.pages[i]));
      if (validPage(saved.active)) page = saved.active;
    } else load(pages[0], JSON.parse(localStorage.getItem(STORAGE) || "{}"));
  } catch {
    // An unreadable page store must not discard the previous single-page map.
    try { load(pages[0], JSON.parse(localStorage.getItem(STORAGE) || "{}")); } catch { /* Optional storage. */ }
  }
  let map = pages[page];
  /** "channel:controller" → knob key */
  const keys = new Map();
  const index = () => {
    keys.clear();
    for (const [key, [channel, controller]] of map) keys.set(`${channel}:${controller}`, key);
  };
  index();
  /** knob key → the knobs showing it: { dial, label, apply(unit), increment(delta) } */
  const knobs = new Map();
  let hooks = { notify() {}, hint: () => "", menu: () => null, changed() {} },
    learning = null,
    attached = 0;
  const name = ([channel, controller]) => `CC ${controller} · ch ${channel + 1}`;
  function save() {
    index();
    try {
      localStorage.setItem(PAGE_STORAGE, JSON.stringify({ version: 1, active: page,
        pages: pages.map(p => Object.fromEntries(p)) }));
      // Retain Page 1 in the previous format for older builds.
      localStorage.setItem(STORAGE, JSON.stringify(Object.fromEntries(pages[0])));
    } catch {
      // Storage may be full or blocked; the mapping still works now.
    }
    hooks.changed();
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
  function attach(dial, key, text, apply, increment) {
    const knob = { dial, label: text, apply, increment };
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
    hooks.notify(`MIDI learn: move a controller to turn ${label(key)} · Page ${page + 1} · Esc cancels${hint ? ` · ${hint}` : ""}`, true);
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
  function selectPage(next, { preserveLearning = false } = {}) {
    if (!validPage(next)) throw new Error("Choose a MIDI mapping page from 1 to 16");
    if (next === page) return page;
    // Neither an open menu nor an armed Learn may act on a different page.
    window.SadLazySelect?.close();
    if (learning !== null && !preserveLearning) cancel();
    const repaint = new Set([...map.keys(), ...pages[next].keys()]);
    page = next;
    map = pages[page];
    save();
    repaint.forEach(paint);
    return page;
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
    hooks.notify(`Forgot ${mapped.length} MIDI mapping${mapped.length === 1 ? "" : "s"} on Page ${page + 1}`);
  }
  /** A controller moved: learn it, then turn the knob it is mapped to. */
  function input(channel, controller, value, format = "absolute") {
    const maximum = format === "14bit" ? 16383 : 127;
    if (!valid(channel, controller) || !["absolute", "relative", "14bit"].includes(format) ||
        !Number.isInteger(value) || value < 0 || value > maximum) return false;
    // Novation relative encoders use offset binary: 63 = −1, 65 = +1.
    if (format === "relative" && value === 64) return true;
    let key = keys.get(`${channel}:${controller}`);
    if (learning !== null) {
      const target = learning;
      stop();
      // One controller turns one knob: learning moves it from the previous one.
      if (key !== undefined && key !== target) {
        map.delete(key);
        paint(key);
      }
      map.set(target, [channel, controller, ...(format === "absolute" ? [] : [format])]);
      save();
      paint(target);
      hooks.notify(`${name([channel, controller])} → ${label(target)}`);
      key = target;
    }
    if (key === undefined) return false;
    const shown = [...(knobs.get(key) || [])].filter(knob => knob.dial.isConnected);
    if (format === "relative") {
      // Linked views may show one parameter more than once. Increment once.
      shown.findLast(knob => knob.increment)?.increment(value - 64);
    } else for (const knob of shown) knob.apply(value / maximum);
    return true;
  }

  // The supplied XL3 modes retain the CC numbers of its factory controls.
  // Move existing assignments to each mode's channel and keep a local backup.
  function configureXL3() {
    let changed = false;
    const before = JSON.stringify({ version: 1, active: page, pages: pages.map(p => Object.fromEntries(p)) });
    pages.forEach((p, channel) => {
      if (channel === 15) return;
      for (const [key, [oldChannel, cc]] of p) {
        if (oldChannel !== 15) continue;
        if (cc >= 5 && cc <= 36) {
          p.set(key, [channel, cc, cc < 13 ? "14bit" : "relative"]);
          changed = true;
        } else if (cc >= 37 && cc <= 52) {
          p.set(key, [channel, cc + 43]);
          changed = true;
        }
      }
    });
    if (changed) {
      try {
        if (!localStorage.getItem("sad-pet-midi-pages-before-xl3-native"))
          localStorage.setItem("sad-pet-midi-pages-before-xl3-native", before);
      } catch { /* Optional storage. */ }
      save();
      map.forEach((_, key) => paint(key));
    }
  }
  /** One Web MIDI message. */
  function bytes(data) {
    return data?.length >= 3 && (data[0] & 0xf0) === 0xb0
      ? input(data[0] & 15, data[1], data[2]) : false;
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
      choices.push(item(`Forget all mappings on Page ${page + 1} (${map.size})`, forgetAll));
    const more = hooks.menu(key, label(key)),
      surface = window.SadControl?.menu(dial, key, label(key)),
      // A keyboard context menu has no pointer position: open by the knob.
      pointer = event.clientX || event.clientY;
    window.SadLazySelect.popup(
      dial,
      [{ choices }, ...(more?.groups || []), ...(surface?.groups || [])],
      (value) => (actions.has(value) ? actions.get(value)() : /^(surface:|control-track:)/.test(value)
        ? surface?.choose(value) : more?.choose(value)),
      {
        title: label(key),
        point: pointer ? [event.clientX, event.clientY] : null,
        checked: (value) => !!more?.checked?.(value),
      },
    );
  }

  return {
    attach,
    detach(dial) {
      const key = dial.dataset.controlKey, set = knobs.get(key);
      if (!set) return 0;
      for (const registration of set) if (registration.dial === dial) set.delete(registration);
      if (!set.size) {
        knobs.delete(key);
        if (learning === key) cancel();
      }
      return set.size;
    },
    input,
    bytes,
    learn,
    cancel,
    forget,
    selectPage,
    configureXL3,
    page: () => page,
    pages: () => pages.map((p, id) => ({ id, count: p.size })),
    setup: (next) => (hooks = { ...hooks, ...next }),
    learning: () => learning,
    mappings: () => Object.fromEntries(map),
  };
})();
