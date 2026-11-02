"use strict";
// Each linked fragment runs the original editor against an explicitly addressed
// timbre. Frames contain UI only: audio, transport and scene storage stay in the
// host. This also covers controls whose behavior is more than a numeric value.
(() => {
  // Knob gestures prevent the browser's native touch menu. A stationary
  // long press provides the same menu while a moving finger still edits.
  let hold = null, heldTarget = null;
  const cancelHold = () => { if (hold) clearTimeout(hold.timer); hold = null; };
  document.addEventListener("pointerdown", event => {
    cancelHold(); heldTarget = null;
    if (event.pointerType !== "touch") return;
    const target = event.target.closest(".dial,button,input,select,.neuro-stage,.neuro-detail-stage,.drum-step");
    if (!target || target.closest(".control-tile-heading,.sad-menu-layer")) return;
    const x = event.clientX, y = event.clientY;
    hold = { x, y, timer: setTimeout(() => {
      target.dispatchEvent(new PointerEvent("pointercancel", { bubbles: true, pointerId: event.pointerId }));
      const menu = new MouseEvent("contextmenu", { bubbles: true, cancelable: true, clientX: x, clientY: y });
      target.dispatchEvent(menu);
      if (menu.defaultPrevented) heldTarget = target;
      hold = null;
    }, 600) };
  }, true);
  document.addEventListener("pointermove", event => {
    if (hold && Math.hypot(event.clientX - hold.x, event.clientY - hold.y) > 8) cancelHold();
  }, true);
  for (const type of ["pointerup", "pointercancel"]) document.addEventListener(type, cancelHold, true);
  document.addEventListener("click", event => {
    if (heldTarget?.contains(event.target)) { event.preventDefault(); event.stopImmediatePropagation(); }
    heldTarget = null;
  }, true);
  const frame = window.frameElement;
  if (frame?.controlItem && parent.SadControl) {
    const item = frame.controlItem, host = parent.SadControl;
    const pool = !!frame.controlPool;
    if (!pool) document.addEventListener("wheel", event => {
      if (event.defaultPrevented || event.ctrlKey) return;
      // An embedded module must not trap the page's vertical scroll.
      // Keep its own horizontal sequencer lanes usable.
      if (Math.abs(event.deltaX) > Math.abs(event.deltaY)) return;
      const unit = event.deltaMode === 1 ? 16 : event.deltaMode === 2 ? parent.innerHeight : 1;
      parent.scrollBy({ top: event.deltaY * unit, left: event.deltaX * unit, behavior: "instant" });
      event.preventDefault();
    }, { passive: false });
    window.sadPetFetch = (path, options) => host.fetch(item.timbre, path,
      { ...options, controlBaseline: editor?.baseline() });
    window.sadPetState = () => host.state(item.timbre);
    window.sadPetNative = parent.sadPetNative;
    document.documentElement.className = parent.document.documentElement.className;
    document.documentElement.classList.add("control-fragment-document");
    let editor = null, mounted = null, observer = null, failure = null;
    const clearPath = () => {
      for (const node of document.querySelectorAll(".control-fragment-path,.control-fragment-node"))
        node.classList.remove("control-fragment-path", "control-fragment-node");
    };
    const error = message => {
      if (pool) { host.poolFailed(frame, message); return; }
      clearPath(); observer?.disconnect(); mounted = null;
      failure ||= document.body.appendChild(document.createElement("div"));
      failure.className = "control-missing control-fragment-node";
      failure.textContent = message;
    };
    const size = () => {
      if (mounted) host.size(frame, mounted.getBoundingClientRect().height + 16);
    };
    const mount = () => {
      if (!editor || pool) return;
      if (!editor.valid(item)) return error("Source track removed");
      const node = document.querySelector(item.selector);
      if (!node) return error("Source control unavailable");
      // Keep the original DOM hierarchy intact: sequencer updates and module
      // editors use queries scoped to their own roots. Only the selected
      // branch participates in this fragment's layout.
      if (node !== mounted || !node.parentElement?.classList.contains("control-fragment-path")) {
        observer?.disconnect(); clearPath(); failure?.remove(); failure = null;
        for (let p = node.parentElement; p && p !== document.body; p = p.parentElement) {
          p.classList.add("control-fragment-path");
          if (p.tagName === "DETAILS") p.open = true;
        }
        node.hidden = false; node.classList.add("control-fragment-node");
        if (item.module) for (const grip of node.querySelectorAll(".rack-grip")) {
          grip.onpointerdown = event => host.move(frame, event);
          grip.onkeydown = event => host.moveKey(frame, event);
        }
        mounted = node;
        observer = new ResizeObserver(size); observer.observe(node);
      }
      size();
    };
    for (const type of ["pointerdown", "keydown"]) document.addEventListener(type, () => parent.sadPetResume?.(), { capture: true });
    window.sadPetControlSource = {
      item, pool,
      updated: () => { if (pool) host.paintSource(item.timbre); },
      error,
      ready(api) {
        editor = api;
        api.view(item.view);
        if (pool) host.poolReady(frame, api);
        mount();
      },
      mount,
      menu(anchor, model) {
        const choices = (items) => items.map(e => e.items
          ? { label: e.text, choices: choices(e.items) } : [e.value, e.text, e.label]);
        const proxy = this.proxyAnchor, context = this.proxyContext;
        const rect = frame.getBoundingClientRect(), box = anchor.getBoundingClientRect();
        const proxyBox = proxy?.getBoundingClientRect();
        parent.SadLazySelect.popup(proxy || frame, [{ choices: choices(model.items) }], value => {
          const choose = () => {
            if (model.choose) model.choose(value);
            else {
              window.SadLazySelect.setValue(anchor, value);
              for (const type of ["input", "change"])
                anchor.dispatchEvent(new Event(type, { bubbles: true }));
            }
          };
          context ? editor.run(context, choose) : choose();
          host.paintSource(item.timbre);
        }, { title: model.title || anchor.getAttribute("aria-label") || "",
          point: proxyBox ? [proxyBox.left, proxyBox.bottom] : model.point ? [rect.left + model.point[0], rect.top + model.point[1]]
            : [rect.left + box.left, rect.top + box.bottom], checked: model.checked });
      },
    };
    // The source's own MIDI/modulation menu also offers the surface's sequencer.
    window.SadControl = {
      menu: (node, key, label) => host.sequenceMenu(item.timbre, editor?.parameter(key) || key, label),
    };
    return;
  }

  const el = (tag, className, text) => {
    const node = document.createElement(tag);
    if (className) node.className = className;
    if (text !== undefined) node.textContent = text;
    return node;
  };
  const quote = value => JSON.stringify(String(value));
  const popup = (anchor, groups, choose, event, title) => window.SadLazySelect.popup(anchor, groups, choose,
    { title, point: event ? [event.clientX, event.clientY] : null });
  let hooks = null, surface = null, canvas = null, empty = null, current = null;
  let template = null, serial = Promise.resolve();
  const tiles = new Map(), states = new Map(), writes = new Map(), sources = new Map();
  const active = () => hooks?.context()?.timbres?.find(t => t.id === hooks.context().active);
  const controls = () => hooks?.context()?.timbres?.filter(t => t.kind === "control") || [];
  const parameter = key => hooks?.parameter(key) || null;

  function selector(node) {
    if (node.id) return `#${CSS.escape(node.id)}`;
    if (node.matches(".neuro-stage,.neuro-detail-stage"))
      return `#neuropolis .${node.classList.contains("neuro-stage") ? "neuro-stage" : "neuro-detail-stage"}[data-pick=${quote(node.dataset.pick)}]`;
    if (node.matches(".knob")) {
      const key = node.querySelector(".dial")?.dataset.controlKey;
      if (key) return `.knob:has(> .dial[data-control-key=${quote(key)}])`;
    }
    const label = node.getAttribute("aria-label");
    const scope = node.closest("[id]");
    if (label) {
      const query = `${node.tagName.toLowerCase()}[aria-label=${quote(label)}]`;
      if (document.querySelectorAll(query).length === 1) return query;
      if (scope) return `#${CSS.escape(scope.id)} ${query}`;
    }
    const path = [];
    for (let part = node; part && part !== document.body; part = part.parentElement) {
      if (part.id) { path.unshift(`#${CSS.escape(part.id)}`); break; }
      const siblings = [...part.parentElement.children].filter(n => n.tagName === part.tagName);
      path.unshift(`${part.tagName.toLowerCase()}:nth-of-type(${siblings.indexOf(part) + 1})`);
    }
    return path.join(" > ");
  }
  function capture(target, key, label) {
    if (!hooks || !active()) return null;
    let node = target.closest(".knob") || target;
    if (target.matches(".neuro-stage-number,.neuro-stage,.neuro-detail-stage"))
      node = target.closest(".neuro-stage,.neuro-detail-stage") || node;
    const module = !!target.closest(".rack-grip,.power,[data-module-handle]");
    if (module) node = target.closest(".filter-card,.lfo-card,.env-card,.color-slot,.rack-module,.bank,section") || node;
    else if (target.matches("input,select") && target.closest("label")) node = target.closest("label");
    if (node.closest("header,footer,#control-surface,.sad-menu-layer")) return null;
    if (node.closest("#timbres") && !node.closest(".timbre-mix,.timbre-sends")) return null;
    const box = node.getBoundingClientRect();
    return { id: crypto.randomUUID(), timbre: active().id,
      label: module ? node.getAttribute("aria-label") || node.querySelector("h2,h3")?.childNodes[0]?.textContent.trim() || label || "Module"
        : label || target.getAttribute("aria-label") || node.textContent.trim().slice(0, 60) || "Control",
      selector: selector(node), parameter: parameter(key || target.dataset.controlKey || target.id),
      module, view: { ...hooks.view(), sequence: !!node.closest("#neuropolis") }, x: 20, y: 20,
      width: Math.max(module ? 320 : 128, Math.ceil(box.width) + 16), height: Math.ceil(box.height) };
  }
  async function add(item, id) {
    await hooks.flush();
    if (id === "new") {
      const context = await hooks.request("/api/timbres/add", { kind: "control" });
      id = context.active;
    }
    const read = await hooks.request("/api/control/request", { timbre: Number(id), path: "/api/preset" });
    if (!read.control) throw new Error("Choose a Control timbre");
    if (read.control.items.some(i => i.timbre === item.timbre && i.selector === item.selector
        && JSON.stringify(i.view) === JSON.stringify(item.view))) return hooks.notify("Already on this Control");
    item.y = read.control.items.reduce((end, i) => Math.max(end, i.y + (i.height || 120) + 52), 20);
    read.control.items.push(item);
    await hooks.request("/api/control/request", { timbre: Number(id), path: "/api/preset", data: read });
    hooks.dirty();
    if (active()?.id === id || !hooks.context().timbres.some(t => t.id === id))
      await hooks.change("/api/timbres/select", { id });
    hooks.notify(`Added to ${controls().find(t => t.id === id)?.name || "Control"}`);
  }
  function menu(node, key, label) {
    const item = capture(node, key, label);
    if (!item) return null;
    const choices = controls().map(t => [`surface:${t.id}`, t.name]);
    choices.push(["surface:new", "+ New Control"]);
    return { groups: [{ label: "To control", choices }], choose(value) {
      if (!value.startsWith("surface:")) return;
      serial = serial.catch(() => {}).then(() => add(item, value.slice(8) === "new" ? "new" : Number(value.slice(8))))
        .catch(e => hooks.notify(e.message));
    } };
  }
  function sequenceMenu(timbre, key, label) {
    const id = parameter(key);
    if (!id || !hooks.read()?.control) return null;
    const destination = `control:${timbre}:${id}`, tracks = hooks.read().sequencer.tracks;
    return { groups: [{ label: "Control sequencer", choices: [
      ...tracks.map(t => [`control-track:${t.id}`, t.name]), ["control-track:new", "New track"],
    ] }], choose(value) {
      if (value === "control-track:new") {
        const track = hooks.addTrack(label, { kind: "parameter", parameter: destination });
        track.absolute = false;
        return;
      }
      const track = tracks.find(t => t.id === Number(value.split(":")[1]));
      if (!track) return;
      if (track.target.parameter === destination || track.destinations.includes(destination)) return;
      const slot = track.destinations.findIndex(d => !d);
      if (slot < 0) return hooks.notify("This track has no free destination");
      track.destinations[slot] = destination;
      hooks.changed();
    } };
  }

  async function sourceTemplate() {
    if (!template) template = fetch(location.href).then(r => r.text()).then(html => {
      const doc = new DOMParser().parseFromString(html, "text/html");
      for (const script of doc.querySelectorAll("script")) script.remove();
      for (const name of ["control", "library", "selection", "presets", "sequence", "panels", "tb303", "eq", "rack", "vocal", "sampler-library", "sampler", "midi", "clock", "modulation", "app"]) {
        const script = doc.createElement("script");
        script.src = `./${name}.js`; script.defer = true; doc.head.append(script);
      }
      const base = doc.createElement("base");
      base.href = new URL(".", location.href).href;
      doc.head.prepend(base);
      return `<!doctype html>${doc.documentElement.outerHTML}`;
    });
    return template;
  }
  function sourceEditor(timbre) {
    if (sources.has(timbre)) return sources.get(timbre).ready;
    const iframe = el("iframe", "control-source-pool");
    iframe.title = "Control source editor";
    iframe.controlItem = { timbre }; iframe.controlPool = true;
    let resolve, reject;
    const ready = new Promise((done, fail) => { resolve = done; reject = fail; });
    sources.set(timbre, { iframe, ready, resolve, reject });
    document.body.append(iframe);
    sourceTemplate().then(html => { if (iframe.isConnected) iframe.srcdoc = html; });
    return ready;
  }
  function clearSources() {
    if (sources.size) window.SadLazySelect.close();
    for (const source of sources.values()) source.iframe.remove();
    sources.clear(); states.clear();
  }
  function disposeTile(tile) {
    tile.layoutObserver?.disconnect();
    const controls = [...tile.querySelectorAll("[data-control-key]")];
    tile.remove();
    for (const node of controls) {
      if (!window.SadMidiLearn?.detach(node)) hooks.forgetControl(node.dataset.controlKey);
    }
  }
  function clearTiles() {
    for (const tile of tiles.values()) disposeTile(tile);
    tiles.clear(); canvas.replaceChildren();
  }
  // Numeric controls use the original binding and shared knob renderer. Other
  // controls reuse the source markup and its callbacks. No per-control editor,
  // polling loop, canvas animation or hidden instrument tree is created.
  async function proxyFor(tile, item) {
    const api = await sourceEditor(item.timbre);
    if (!tile.isConnected) return;
    if (!api.valid(item)) { tile.append(el("p", "control-missing", "Source track removed")); return; }
    let original = api.describe(item);
    if (!original) { tile.append(el("p", "control-missing", "Source control unavailable")); return; }
    const proxy = el("div", "control-proxy"), clone = document.importNode(original, true);
    tile.append(proxy);
    let slot = proxy;
    const ancestors = [];
    for (let p = original.parentElement; p && p.tagName !== "BODY" && p.tagName !== "MAIN"; p = p.parentElement)
      ancestors.unshift(p.className);
    for (const classes of ancestors) {
      const wrapper = el("div", `${classes} control-proxy-ancestor`);
      wrapper.classList.remove("control-fragment-path", "control-fragment-node");
      slot.append(wrapper); slot = wrapper;
    }
    slot.append(clone);
    clone.hidden = false;
    clone.classList.remove("control-fragment-node");
    const originals = [original, ...original.querySelectorAll("*")];
    const copies = [clone, ...clone.querySelectorAll("*")];
    const pairs = [], knobUpdates = [], bindings = new Set();
    for (let i = 0; i < originals.length; i++) {
      const source = originals[i], copy = copies[i];
      if (copy.namespaceURI === "http://www.w3.org/1999/xhtml") copy.removeAttribute("id");
      if (copy.tagName === "LABEL") copy.removeAttribute("for");
      if (source.closest(".knob")) {
        if (!source.matches(".knob")) continue;
        const saved = source.querySelector(".dial")?.controlBinding;
        if (!saved) continue;
        const [key, ...schema] = saved.schema, parameter = api.parameter(key);
        const format = schema[1];
        schema[1] = value => api.run(item, () => format(value));
        if (parameter) bindings.add(parameter);
        const bank = el("div"), binding = { ...saved.binding,
          register: false,
          get: () => api.run(item, saved.binding.get),
          set: value => api.run(item, () => saved.binding.set(value)),
          reset: () => api.run(item, saved.binding.reset),
          midiIncrement: (value, delta) => api.run(item, () => saved.binding.midiIncrement(value, delta)),
          midiQuantize: (value, origin) => api.run(item, () => saved.binding.midiQuantize(value, origin)),
          feedback: () => api.read().state?.knobs?.[key],
          gesture: active => api.gesture(active),
        };
        const update = hooks.knob([`control:${item.timbre}:${parameter || key}`, ...schema], bank, binding);
        const knob = bank.firstElementChild;
        knob.className = source.className;
        knob.classList.remove("control-fragment-node");
        copy.replaceWith(knob);
        knobUpdates.push(() => {
          const dial = knob.querySelector(".dial");
          const disabled = source.querySelector(".dial").getAttribute("aria-disabled");
          if (disabled) dial.setAttribute("aria-disabled", disabled);
          else dial.removeAttribute("aria-disabled");
          update.paint();
        });
        continue;
      }
      const pair = { source, copy, index: i };
      if (source.matches("input,select,textarea,button,output,.neuro-value")) pairs.push(pair);
      if (!source.matches("input,select,textarea,button")) continue;
      const forward = event => {
        const source = pair.source;
        event.stopPropagation();
        if (source.matches('input[type="checkbox"],input[type="radio"]') && event.type === "click") return;
        const window = source.ownerDocument.defaultView;
        const accepted = api.run(item, () => {
          if (source.matches(".neuro-stage-pick") && event.type === "click") {
            api.pick(Number(source.dataset.pick), event); return false;
          }
          if ("value" in source && "value" in copy) source.value = copy.value;
          if ("checked" in source) source.checked = copy.checked;
          window.sadPetControlSource.proxyAnchor = copy;
          window.sadPetControlSource.proxyContext = item;
          if (event.type === "pointerdown") {
            source.setPointerCapture = id => copy.setPointerCapture(id);
            source.releasePointerCapture = id => { if (copy.hasPointerCapture(id)) copy.releasePointerCapture(id); };
            source.hasPointerCapture = id => copy.hasPointerCapture(id);
            source.getBoundingClientRect = () => copy.getBoundingClientRect();
            source.focus = options => copy.focus(options);
          }
          const Event = event.type.startsWith("pointer") ? window.PointerEvent : event.type === "keydown" ? window.KeyboardEvent
            : event.type === "click" || event.type === "dblclick" ? window.MouseEvent : window.Event;
          return source.dispatchEvent(new Event(event.type, { bubbles: true, cancelable: true,
            key: event.key, shiftKey: event.shiftKey, ctrlKey: event.ctrlKey, metaKey: event.metaKey,
            pointerId: event.pointerId, pointerType: event.pointerType, button: event.button,
            buttons: event.buttons, clientX: event.clientX, clientY: event.clientY }));
        });
        if (!accepted) event.preventDefault();
        paint();
      };
      for (const type of ["input", "change", "click", "dblclick", "keydown"]) copy.addEventListener(type, forward);
      if (source.matches("button:not(.neuro-stage-pick)"))
        for (const type of ["pointerdown", "pointermove", "pointerup", "pointercancel", "lostpointercapture"])
          copy.addEventListener(type, forward);
      if (source.matches('input[type="range"]')) {
        copy.setAttribute("aria-label", source.getAttribute("aria-label") || item.label);
        let dragging = false;
        copy.addEventListener("pointerdown", event => {
          if (event.button > 0) return;
          copy.setPointerCapture(event.pointerId);
          dragging = true; api.gesture(true);
        });
        for (const type of ["pointerup", "pointercancel", "lostpointercapture"])
          copy.addEventListener(type, () => { if (dragging) { dragging = false; api.gesture(false); } });
        const key = api.parameter(source.dataset.controlKey || source.id);
        if (key) bindings.add(key);
        const midiKey = `control:${item.timbre}:${key || item.id}`;
        copy.dataset.controlKey = midiKey;
        window.SadMidiLearn?.attach(copy, midiKey, source.getAttribute("aria-label") || item.label, unit => {
          copy.value = String(Number(copy.min) + unit * (Number(copy.max) - Number(copy.min)));
          copy.dispatchEvent(new Event("input", { bubbles: true }));
        });
      }
    }
    function paint() {
      if (!tile.isConnected) return;
      if (original.matches(".neuro-stage,.neuro-detail-stage")) {
        const fresh = api.node(item);
        if (fresh && fresh !== original && Number(fresh.dataset.track) === item.view?.neuro?.selected) {
          const nodes = [fresh, ...fresh.querySelectorAll("*")];
          for (const pair of pairs) pair.source = nodes[pair.index] || pair.source;
          original = fresh;
        }
        const state = api.read().state, track = state.sequencer?.tracks.find(t => t.id === item.view?.neuro?.selected);
        const stage = Number(original.dataset.pick);
        clone.classList.toggle("selected", api.picked(stage));
        clone.classList.toggle("is-current", state.playing && track?.step === (item.view?.neuro?.visiblePage || 0) * 16 + stage);
      }
      for (const update of knobUpdates) update();
      for (const { source, copy } of pairs) {
        if (!copy.isConnected) continue;
        if (document.activeElement !== copy && "value" in source && "value" in copy) copy.value = source.value;
        if ("checked" in source) copy.checked = source.checked;
        if ("disabled" in source) copy.disabled = source.disabled;
        for (const name of ["aria-pressed", "aria-selected", "aria-valuenow", "aria-valuetext"])
          if (source.hasAttribute(name)) copy.setAttribute(name, source.getAttribute(name));
        if (source.matches("output,.neuro-value") && copy.textContent !== source.textContent) copy.textContent = source.textContent;
        if (source.matches("button") && copy.innerHTML !== source.innerHTML) copy.innerHTML = source.innerHTML;
        if (source.matches("button") && copy.className !== source.className) copy.className = source.className;
      }
      extent();
    }
    tile.paint = paint;
    tile.controlSpecs = hooks.catalog().parameters.filter(p => bindings.has(p.id)).map(spec => ({ ...spec,
      id: `control:${item.timbre}:${spec.id}`, name: `${hooks.context().timbres.find(t => t.id === item.timbre)?.name} · ${spec.name}` }));
    paint(); sync();
  }
  function move(event, tile, item) {
    if (event.button > 0 || event.target.closest(".control-tile-tools")) return;
    const grip = event.currentTarget, source = grip.ownerDocument.defaultView.frameElement;
    const point = e => {
      const rect = source?.getBoundingClientRect();
      return { x: e.clientX + (rect?.left || 0) + window.scrollX,
        y: e.clientY + (rect?.top || 0) + window.scrollY };
    };
    const start = { ...point(event), left: item.x, top: item.y };
    grip.setPointerCapture(event.pointerId);
    hooks.gesture(true);
    tile.classList.add("is-moving");
    const drag = e => {
      const position = point(e);
      item.x = Math.max(0, Math.round(start.left + position.x - start.x));
      item.y = Math.max(0, Math.round(start.top + position.y - start.y));
      place(tile, item);
      extent();
    };
    const end = () => {
      grip.removeEventListener("pointermove", drag);
      grip.removeEventListener("pointerup", end);
      grip.removeEventListener("pointercancel", end);
      grip.removeEventListener("lostpointercapture", end);
      tile.classList.remove("is-moving");
      hooks.gesture(false);
      hooks.save();
    };
    grip.addEventListener("pointermove", drag);
    for (const type of ["pointerup", "pointercancel", "lostpointercapture"]) grip.addEventListener(type, end);
    event.preventDefault();
  }
  function place(tile, item) {
    tile.style.left = `${item.x}px`;
    tile.style.top = `${item.y}px`;
    tile.style.width = `${item.width || (item.module ? 640 : 120)}px`;
  }
  function extent() {
    let width = 0, height = 360;
    for (const item of hooks.read()?.control?.items || []) {
      const tile = tiles.get(item.id);
      width = Math.max(width, item.x + (tile?.controlBounds?.width || item.width) + 20);
      height = Math.max(height, item.y + (tile?.controlBounds?.height || item.height) + 24);
    }
    canvas.style.minWidth = `${width}px`;
    canvas.style.height = `${height}px`;
  }
  function tileFor(item) {
    const tile = el("article", `control-tile${item.module ? " is-module" : ""}`);
    tile.dataset.item = item.id;
    const heading = el("div", "control-tile-heading"), grip = el("button", "control-tile-grip", "⠿");
    grip.setAttribute("aria-label", `Move ${item.label}`);
    const title = el("span", "control-tile-title", item.label);
    const tools = el("div", "control-tile-tools"), open = el("button", "", "↗"), remove = el("button", "", "×");
    open.setAttribute("aria-label", "Open source timbre");
    open.onclick = () => hooks.change("/api/timbres/select", { id: item.timbre });
    remove.setAttribute("aria-label", `Remove ${item.label} from Control`);
    remove.onclick = () => {
      hooks.read().control.items = hooks.read().control.items.filter(i => i.id !== item.id);
      hooks.save(); sync();
    };
    tools.append(open, remove); heading.append(grip, title, tools);
    heading.onpointerdown = event => move(event, tile, item);
    grip.onkeydown = event => {
      const delta = { ArrowLeft: [-1, 0], ArrowRight: [1, 0], ArrowUp: [0, -1], ArrowDown: [0, 1] }[event.key];
      if (!delta) return;
      event.preventDefault();
      item.x = Math.max(0, item.x + delta[0] * (event.shiftKey ? 20 : 4));
      item.y = Math.max(0, item.y + delta[1] * (event.shiftKey ? 20 : 4));
      place(tile, item); extent(); hooks.save();
    };
    const source = hooks.context().timbres.find(t => t.id === item.timbre && t.kind !== "empty");
    tile.append(heading);
    tile.controlSourceExists = !!source;
    if (source) {
      title.title = `${source.name} · ${item.label}`;
      if (!item.module) {
        proxyFor(tile, item).catch(error => hooks.notify(error.message));
        return tile;
      }
      const iframe = el("iframe", "control-tile-frame");
      iframe.title = `${source.name} · ${item.label}`;
      iframe.controlItem = item;
      iframe.style.height = `${Math.max(36, item.height || 120)}px`;
      tile.append(iframe);
      sourceTemplate().then(html => { if (iframe.isConnected) iframe.srcdoc = html; })
        .catch(error => hooks.notify(error.message));
    } else {
      open.disabled = true;
      tile.append(el("p", "control-missing", "Source timbre removed"));
    }
    return tile;
  }
  function sync() {
    if (!hooks) return;
    const settings = hooks.read()?.control, id = active()?.id;
    surface.hidden = !settings;
    document.body.classList.toggle("control-timbre", !!settings);
    if (!settings) {
      if (current !== null) { clearTiles(); clearSources(); current = null; }
      return;
    }
    if (current !== id) {
      clearTiles(); clearSources(); current = id;
    }
    const items = settings.items;
    for (const [key, tile] of tiles) {
      if (!items.some(i => i.id === key)) { disposeTile(tile); tiles.delete(key); }
    }
    for (const item of items) {
      let tile = tiles.get(item.id);
      const exists = hooks.context().timbres.some(t => t.id === item.timbre && t.kind !== "empty");
      if (tile && tile.controlSourceExists !== exists) {
        disposeTile(tile); tiles.delete(item.id); tile = null;
      }
      // A poll replaces the preset object, so event handlers must hold its
      // current item rather than an obsolete copy.
      if (tile && tile.controlItem !== item) {
        Object.assign(tile.controlItem, item);
        settings.items[items.indexOf(item)] = tile.controlItem;
      }
      if (!tile) {
        tile = tileFor(item); tile.controlItem = item;
        tiles.set(item.id, tile); canvas.append(tile);
        tile.layoutObserver = new ResizeObserver(entries => {
          const box = entries[0].borderBoxSize?.[0];
          tile.controlBounds = box ? { width: box.inlineSize, height: box.blockSize }
            : { width: entries[0].contentRect.width + 2, height: entries[0].contentRect.height + 2 };
          extent();
        });
        tile.layoutObserver.observe(tile);
      }
      place(tile, tile.controlItem);
    }
    empty.hidden = items.length > 0;
    const catalog = hooks.catalog(), specs = new Map();
    for (const item of items) {
      const source = hooks.context().timbres.find(t => t.id === item.timbre);
      if (item.parameter && source) {
        const spec = catalog.parameters.find(s => s.id === item.parameter);
        if (spec) specs.set(`control:${item.timbre}:${spec.id}`, { ...spec,
          id: `control:${item.timbre}:${spec.id}`, name: `${source.name} · ${item.label}` });
      }
    }
    // Module contents publish their numeric bindings when their editors mount.
    for (const tile of tiles.values())
      for (const spec of tile.controlSpecs || []) specs.set(spec.id, spec);
    hooks.targets([...specs.values()]);
    extent();
  }
  // Apply only what this editor changed. Two fragments may show different
  // modules of one source and hold different snapshots while being edited.
  function mergeEdit(latest, base, edited) {
    if (JSON.stringify(base) === JSON.stringify(edited)) return latest;
    if (!base || !edited || typeof base !== "object" || typeof edited !== "object"
        || Array.isArray(base) !== Array.isArray(edited)
        || (Array.isArray(base) && base.length !== edited.length)) return structuredClone(edited);
    const result = structuredClone(latest ?? (Array.isArray(edited) ? [] : {}));
    for (const key of new Set([...Object.keys(base), ...Object.keys(edited)])) {
      if (!(key in edited)) delete result[key];
      else result[key] = mergeEdit(result[key], base[key], edited[key]);
    }
    return result;
  }
  async function sendSource(timbre, path, options = {}) {
    const url = new URL(path, location.href);
    if (!url.pathname.startsWith("/api/")) return fetch(path, options);
    if (["/api/presets", "/api/color-defaults", "/api/sequencer/catalog", "/api/drums/catalog", "/api/samples", "/api/filter/response"].includes(url.pathname))
      return (window.sadPetFetch || window.fetch)(path, options);
    const data = { timbre, path: url.pathname };
    if (options.body !== undefined) {
      data.data = typeof options.body === "string" ? JSON.parse(options.body) : options.body;
      if (url.pathname === "/api/preset" && options.controlBaseline) {
        const latest = await sendSource(timbre, "/api/preset");
        const value = await latest.json();
        if (!latest.ok) return new Response(JSON.stringify(value), { status: latest.status });
        data.data = mergeEdit(value, options.controlBaseline, data.data);
      }
      states.delete(timbre);
      hooks.dirty();
    }
    const result = await (window.sadPetFetch || window.fetch)("/api/control/request", {
      method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify(data),
    });
    if (options.body !== undefined) states.delete(timbre);
    return result;
  }
  function fetchSource(timbre, path, options = {}) {
    const previous = writes.get(timbre) || Promise.resolve();
    const result = previous.catch(() => {}).then(() => sendSource(timbre, path, options));
    writes.set(timbre, result);
    return result;
  }
  async function sourceState(timbre) {
    const cached = states.get(timbre);
    if (cached && performance.now() - cached.time < 100) return structuredClone(await cached.value);
    const value = fetchSource(timbre, "/api/state").then(async response => {
      const state = await response.json();
      if (!response.ok) throw new Error(state.error);
      state.features = hooks.features();
      return state;
    });
    states.set(timbre, { time: performance.now(), value });
    return structuredClone(await value);
  }
  window.SadControl = { menu, sequenceMenu, sync, fetch: fetchSource, state: sourceState,
    poolReady(iframe, api) { sources.get(iframe.controlItem.timbre)?.resolve(api); },
    poolFailed(iframe, message) { sources.get(iframe.controlItem.timbre)?.reject(new Error(message)); },
    paintSource(timbre) {
      for (const tile of tiles.values()) if (tile.controlItem?.timbre === timbre) tile.paint?.();
    },
    move(iframe, event) {
      const tile = iframe.closest(".control-tile");
      if (tile) move(event, tile, tile.controlItem);
    },
    moveKey(iframe, event) {
      const tile = iframe.closest(".control-tile");
      if (tile) tile.querySelector(".control-tile-grip").onkeydown(event);
    },
    size(iframe, height) {
      if (!iframe.isConnected) return;
      const value = Math.max(36, Math.ceil(height));
      if (Math.abs(parseFloat(iframe.style.height) - value) > 1) iframe.style.height = `${value}px`;
      extent();
    },
    bindings(iframe, bindings) {
      const tile = iframe.closest(".control-tile");
      if (!tile) return;
      const timbre = iframe.controlItem.timbre, source = hooks.context().timbres.find(t => t.id === timbre);
      tile.controlSpecs = bindings.map(spec => ({ ...spec, id: `control:${timbre}:${spec.id}`,
        name: `${source?.name || "Source"} · ${spec.name}` }));
      sync();
    },
    setup(api) {
      hooks = api;
      surface = el("section", "bank control-surface"); surface.id = "control-surface"; surface.hidden = true;
      surface.setAttribute("aria-label", "Control canvas");
      const heading = el("div", "section-heading"), title = el("h2", "", "Control"), sequence = el("button", "", "+ Sequencer");
      sequence.onclick = () => hooks.addTrack("Control 1", { kind: "parameter", parameter: hooks.targets()[0]?.id || "" });
      heading.append(title, sequence);
      const scroll = el("div", "control-scroll"); canvas = el("div", "control-canvas");
      empty = el("p", "control-empty", "Right-click a control or a module → To control");
      scroll.append(canvas, empty); surface.append(heading, scroll);
      document.getElementById("neuropolis").before(surface);
      document.addEventListener("contextmenu", event => {
        if (event.defaultPrevented || event.target.closest(".dial,[data-control-key],#control-surface")) return;
        const target = event.target.closest("button,input,select,textarea,.neuro-stage,.neuro-detail-stage,.drum-step");
        if (!target) return;
        const actions = menu(target);
        if (!actions) return;
        event.preventDefault(); popup(target, actions.groups, actions.choose, event);
      });
      sync();
    },
  };
})();
