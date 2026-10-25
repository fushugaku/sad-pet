"use strict";
// Modulation from a control's own menu, and macros. Right-click a knob or
// slider to route an LFO, envelope, sequencer track, macro or matrix source to
// it, or a new LFO, envelope, track or macro, or to take a route away. A macro
// is a knob of the user's own whose value turns every destination it lists.
// The editor hands over its preset and destination helpers with setup().
window.SadModulation = (() => {
  const MAX_MODULATORS = 8,
    MAX_ROUTES = 8,
    MAX_MACROS = 8,
    MAX_TARGETS = 8;
  // Each LFO and ENV has three routes of its own; the matrix takes the rest.
  const ROUTES = {
    lfo: [["destination", "depth"], ["destination2", "depth2"], ["destination3", "depth3"]],
    env: [["destination", "depth"], ["extra_destination", "extra_depth"], ["destination3", "depth3"]],
  };
  const BANKS = { lfo: "lfos", env: "envs" };
  // New routes sweep about half the control's range: an LFO route adds half
  // its ±depth, a matrix route its source times the depth.
  const DEPTH = { lfo: 0.5, env: 0.5, macro: 0.5 },
    MATRIX_DEPTH = { lfo: 0.25, env: 0.5 },
    OTHER_DEPTH = 0.25;
  let hooks = null,
    cards = [],
    signature = "";
  const none = (d) => !d || d === "none";
  const modulatorName = (kind, i) => `${kind.toUpperCase()} ${i + 1}`;
  const macroName = (m, i) => m.name?.trim() || `Macro ${i + 1}`;
  const sourceName = (source) => hooks.sources.find(([s]) => s === source)?.[1] ?? source;

  /** Every route that turns the control with this key, or the sequencer
   * parameter that lane plays. */
  function existing(p, key, parameter) {
    const routes = [],
      hits = (d) => !none(d) && hooks.key(d) === key;
    for (const kind of ["lfo", "env"])
      for (let i = 0; i < (p[`${kind}_count`] || 3); i++)
        ROUTES[kind].forEach(([field, depth], route) => {
          if (hits(p[BANKS[kind]][i][field]))
            routes.push({ kind, index: i, field, depth, text: `${modulatorName(kind, i)} · route ${route + 1}` });
        });
    p.matrix.forEach((r, index) => {
      if (hits(r.destination))
        routes.push({ kind: "matrix", index, source: r.source, text: `Matrix ${index + 1} · ${sourceName(r.source)}` });
    });
    (p.macros || []).forEach((m, index) =>
      m.targets.forEach((t, target) => {
        if (hits(t.destination)) routes.push({ kind: "macro", index, target, text: macroName(m, index) });
      }),
    );
    if (parameter)
      for (const t of p.sequencer?.tracks || []) {
        if (t.target.kind === "parameter" && t.target.parameter === parameter)
          routes.push({ kind: "track", id: t.id, name: t.name, text: `Delete track ${t.name}` });
        (t.destinations || []).forEach((d, slot) => {
          if (d === parameter) routes.push({ kind: "track", id: t.id, slot, text: `${t.name} · destination ${slot + 1}` });
        });
      }
    return routes;
  }

  /** The menu rows for a control: { groups, choose, checked }, or null when
   * nothing can modulate it. */
  function menu(key, label) {
    const p = hooks?.read();
    if (!p) return null;
    const destination = hooks.destination(key),
      parameter = hooks.sequencer(key);
    if (!destination && !parameter) return null;
    const routes = existing(p, key, parameter),
      actions = new Map(),
      checked = new Set(),
      groups = [];
    // The menu holds the editor's focus, so the preset stays the same object
    // while it is open; an action from an older one does nothing.
    const action = (text, run, on = false) => {
      const value = `mod:${actions.size}`;
      actions.set(value, () => (hooks.read() === p ? run(p) : hooks.notify("The preset changed. Open the menu again.")));
      if (on) checked.add(value);
      return [value, text];
    };
    if (destination)
      for (const kind of ["lfo", "env"]) {
        const count = p[`${kind}_count`] || 3,
          choices = [];
        for (let i = 0; i < count; i++) {
          const on = routes.some((r) => (r.kind === kind && r.index === i) ||
            (r.kind === "matrix" && r.source === `${kind}${i + 1}`));
          const full = !on && !free(p[BANKS[kind]][i], kind);
          choices.push(action(
            `${modulatorName(kind, i)}${kind === "env" && i === 0 ? " · Amp" : ""}${full ? " · via matrix" : ""}`,
            (p) => assign(p, kind, i, destination, key, label), on));
        }
        if (count < MAX_MODULATORS)
          choices.push(action(`New ${modulatorName(kind, count)}`, (p) => create(p, kind, destination, key, label)));
        groups.push({ label: kind.toUpperCase(), choices });
      }
    if (parameter) {
      const choices = (p.sequencer?.tracks || []).map((t) =>
        action(t.name, (p) => assignTrack(p, t.id, parameter, label),
          routes.some((r) => r.kind === "track" && r.id === t.id)));
      choices.push(action("New track", () => {
        const track = hooks.addTrack(label, { kind: "parameter", parameter });
        hooks.notify(`Track ${track.id} → ${label} · draw its steps in the sequencer`);
      }));
      groups.push({ label: "Sequencer", choices });
    }
    if (destination) {
      const macros = p.macros || [],
        choices = macros.map((m, i) => action(macroName(m, i),
          (p) => assignMacro(p, i, destination, key, label),
          routes.some((r) => r.kind === "macro" && r.index === i)));
      if (macros.length < MAX_MACROS)
        choices.push(action("New macro", (p) => createMacro(p, destination, label)));
      groups.push({ label: "Macro", choices });
      groups.push({
        label: "Mod matrix",
        choices: hooks.sources
          .filter(([source]) => !/^(lfo|env)\d/.test(source))
          .map(([source, text]) => action(text, (p) => assignMatrix(p, source, destination, label),
            routes.some((r) => r.kind === "matrix" && r.source === source))),
      });
    }
    if (routes.length)
      groups.push({ label: "Remove modulation", choices: routes.map((r) => action(r.text, (p) => remove(p, r, label))) });
    return {
      groups,
      choose: (value) => actions.get(value)?.(),
      checked: (value) => checked.has(value),
    };
  }

  const free = (holder, kind) => ROUTES[kind].find(([field]) => none(holder[field]));
  function done(text, card = null) {
    hooks.changed();
    hooks.notify(text);
    if (card) reveal(card);
  }
  // Bring a new LFO, ENV or macro into view and flash it.
  function reveal(card) {
    card.scrollIntoView?.({ behavior: "smooth", block: "nearest" });
    card.classList.add("is-new");
    setTimeout(() => card.classList.remove("is-new"), 1200);
  }
  function assign(p, kind, i, destination, key, label, card = null) {
    const holder = p[BANKS[kind]][i],
      name = modulatorName(kind, i);
    if (ROUTES[kind].some(([field]) => !none(holder[field]) && hooks.key(holder[field]) === key) ||
        p.matrix.some((r) => r.source === `${kind}${i + 1}` && hooks.key(r.destination) === key))
      return hooks.notify(`${name} already modulates ${label}`);
    const slot = free(holder, kind);
    if (slot) {
      holder[slot[0]] = destination;
      holder[slot[1]] = DEPTH[kind];
      return done(`${name} → ${label}`, card);
    }
    if (p.matrix.length >= MAX_ROUTES)
      return hooks.notify(`${name} has no free route and the mod matrix is full`);
    p.matrix.push({ source: `${kind}${i + 1}`, destination, depth: MATRIX_DEPTH[kind], enabled: true });
    done(`${name} → ${label} · in the mod matrix: its three routes are taken`, card);
  }
  function create(p, kind, destination, key, label) {
    const count = p[`${kind}_count`] || 3,
      bank = BANKS[kind];
    if (count >= MAX_MODULATORS) return;
    // A fresh one, as − LFO / − ENV leave it.
    p[bank][count] = structuredClone(hooks.defaults()?.[bank]?.[count] ?? p[bank][count]);
    p[`${kind}_count`] = count + 1;
    const card = document.querySelectorAll(kind === "lfo" ? "#lfos .lfo-card" : "#envs .env-card")[count];
    assign(p, kind, count, destination, key, label, card);
  }
  function assignTrack(p, id, parameter, label) {
    const track = p.sequencer.tracks.find((t) => t.id === id);
    if ((track.target.kind === "parameter" && track.target.parameter === parameter) ||
        track.destinations?.includes(parameter))
      return hooks.notify(`${track.name} already plays ${label}`);
    track.destinations ??= [null, null, null];
    const slot = [0, 1, 2].find((i) => !track.destinations[i]);
    if (slot === undefined)
      return hooks.notify(`${track.name} has no free destination`);
    track.destinations[slot] = parameter;
    done(`${track.name} destination ${slot + 1} → ${label} · draw its values in the sequencer`);
  }
  function assignMacro(p, i, destination, key, label) {
    const m = p.macros[i];
    if (m.targets.some((t) => hooks.key(t.destination) === key))
      return hooks.notify(`${macroName(m, i)} already turns ${label}`);
    if (m.targets.length >= MAX_TARGETS)
      return hooks.notify(`${macroName(m, i)} already turns ${MAX_TARGETS} controls`);
    m.targets.push({ destination, depth: DEPTH.macro });
    done(`${macroName(m, i)} → ${label}`);
  }
  function createMacro(p, destination, label) {
    p.macros ??= [];
    if (p.macros.length >= MAX_MACROS) return;
    const m = { name: `Macro ${p.macros.length + 1}`, value: 0, targets: [{ destination, depth: DEPTH.macro }] };
    p.macros.push(m);
    hooks.changed();
    hooks.notify(`${m.name} → ${label} · turn it in Macros`);
    if (cards.length) reveal(cards.at(-1).card);
  }
  function assignMatrix(p, source, destination, label) {
    if (p.matrix.some((r) => r.source === source && hooks.key(r.destination) === hooks.key(destination)))
      return hooks.notify(`${sourceName(source)} already modulates ${label}`);
    if (p.matrix.length >= MAX_ROUTES) return hooks.notify("The mod matrix is full");
    p.matrix.push({ source, destination, depth: OTHER_DEPTH, enabled: true });
    done(`${sourceName(source)} → ${label}`);
  }
  function remove(p, route, label) {
    if (route.kind === "lfo" || route.kind === "env") {
      const holder = p[BANKS[route.kind]][route.index];
      holder[route.field] = "none";
      holder[route.depth] = 0;
    } else if (route.kind === "matrix") p.matrix.splice(route.index, 1);
    else if (route.kind === "macro") p.macros[route.index].targets.splice(route.target, 1);
    else if (route.slot === undefined)
      p.sequencer.tracks = p.sequencer.tracks.filter((t) => t.id !== route.id);
    else p.sequencer.tracks.find((t) => t.id === route.id).destinations[route.slot] = null;
    done(route.kind === "track" && route.slot === undefined
      ? `Deleted track ${route.name}`
      : `${route.text} no longer modulates ${label}`);
  }

  // Macros: a card per macro with its name, knob and target routes.
  function build() {
    const p = hooks.read(),
      macros = p.macros || [],
      grid = document.getElementById("macros");
    grid.replaceChildren();
    cards = macros.map((m, i) => {
      const card = document.createElement("div"),
        number = i + 1;
      card.className = "macro-card";
      card.innerHTML = `<div class="macro-title"><input class="macro-name" maxlength="40" spellcheck="false" aria-label="Macro ${number} name"><button class="macro-remove" title="Remove" aria-label="Remove macro ${number}">×</button></div><div class="macro-body"><div class="macro-knob"></div><div class="macro-targets"></div></div><button class="macro-target-add">+ Target</button>`;
      const name = card.querySelector(".macro-name");
      name.onchange = () => {
        hooks.read().macros[i].name = name.value.trim().slice(0, 40) || `Macro ${number}`;
        hooks.changed();
      };
      card.querySelector(".macro-remove").onclick = () => {
        hooks.read().macros.splice(i, 1);
        hooks.changed();
      };
      card.querySelector(".macro-target-add").onclick = () => {
        const targets = hooks.read().macros[i].targets;
        if (targets.length >= MAX_TARGETS) return;
        targets.push({ destination: "none", depth: DEPTH.macro });
        hooks.changed();
      };
      const update = hooks.knob(
        [`macro${number}`, "AMOUNT", (v) => `${Math.round(v * 100)}%`, 0, 1, `Macro ${number}`],
        card.querySelector(".macro-knob"),
        {
          get: () => hooks.read()?.macros?.[i]?.value ?? 0,
          set: (v) => {
            const knob = hooks.read().macros[i];
            if (!knob) return;
            knob.value = v;
            hooks.parameter(`macro${number}`, v);
          },
          reset: () => 0,
        },
      );
      const targets = m.targets.map((t, j) => {
        const row = document.createElement("div");
        row.className = "env-route macro-target";
        row.innerHTML = `<button aria-label="Macro ${number} target ${j + 1}"></button><label><input type="range" min="-1" max="1" step="0.01" aria-label="Macro ${number} depth ${j + 1}"><output></output></label><button class="macro-target-remove" title="Remove" aria-label="Remove macro ${number} target ${j + 1}">×</button>`;
        const [select, removeTarget] = row.querySelectorAll("button"),
          input = row.querySelector("input"),
          output = row.querySelector("output");
        select.oninput = () => {
          hooks.read().macros[i].targets[j].destination = hooks.parse(select.value);
          hooks.changed();
        };
        input.oninput = () => {
          const target = hooks.read().macros[i].targets[j];
          target.depth = Number(input.value);
          output.textContent = hooks.depthText(target.destination, target.depth);
          hooks.parameter(`macro${number}_depth${j + 1}`, target.depth);
        };
        removeTarget.onclick = () => {
          hooks.read().macros[i].targets.splice(j, 1);
          hooks.changed();
        };
        card.querySelector(".macro-targets").append(row);
        return { select, input, output };
      });
      grid.append(card);
      return { card, name, update, targets };
    });
    fill();
  }
  /** Refill the target pickers: the destinations they offer changed. */
  function fill(groups = null) {
    const macros = hooks?.read()?.macros || [];
    cards.forEach(({ targets }, i) =>
      targets.forEach(({ select }, j) => {
        const target = macros[i]?.targets[j];
        if (target) hooks.fill(select, target.destination, groups);
      }));
  }
  function sync() {
    const p = hooks?.read(),
      grid = document.getElementById("macros");
    if (!p || !grid) return;
    const macros = p.macros || [],
      shape = macros.map((m) => m.targets.length).join(",");
    if (shape !== signature || grid.children.length !== macros.length) {
      signature = shape;
      build();
    }
    cards.forEach(({ card, name, update, targets }, i) => {
      const m = macros[i],
        title = macroName(m, i);
      if (document.activeElement !== name) name.value = title;
      card.querySelector(".dial")?.setAttribute("aria-label", title);
      card.querySelector(".macro-target-add").disabled = m.targets.length >= MAX_TARGETS;
      card.classList.toggle("is-empty", !m.targets.length);
      update();
      targets.forEach(({ select, input, output }, j) => {
        const t = m.targets[j];
        if (document.activeElement !== select)
          window.SadLazySelect.setValue(select, hooks.value(t.destination));
        if (document.activeElement !== input) input.value = t.depth;
        output.textContent = hooks.depthText(t.destination, t.depth);
      });
    });
    document.getElementById("macro-count").textContent = `${macros.length} / ${MAX_MACROS}`;
    document.getElementById("macro-add").disabled = macros.length >= MAX_MACROS;
    document.getElementById("macro-empty").hidden = macros.length > 0;
  }
  function setup(next) {
    hooks = next;
    const add = document.getElementById("macro-add");
    if (add)
      add.onclick = () => {
        const p = hooks.read();
        p.macros ??= [];
        if (p.macros.length >= MAX_MACROS) return;
        p.macros.push({ name: `Macro ${p.macros.length + 1}`, value: 0, targets: [] });
        hooks.changed();
        if (cards.length) reveal(cards.at(-1).card);
      };
  }
  return { setup, menu, sync, fill };
})();
