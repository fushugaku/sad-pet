"use strict";
// MIDI addresses the audio model, not the currently mounted instrument editor.
window.SadMidiTargets = (() => {
  let hooks = null, timer = null, running = null;
  const values = new Map(), pending = new Map(), queues = new Map(), reads = new Map();
  const clamp = (v, a = 0, b = 1) => Math.max(a, Math.min(b, v));
  const exists = d => hooks?.context()?.timbres?.some(t => t.id === d.timbre &&
    t.kind !== "empty" && (!d.kind || t.kind === d.kind));
  const id = d => `control:${d.timbre}:${d.parameter}`;
  function pathOf(d, preset) {
    const path = d.path?.slice();
    if (!path || path[0] !== "sampler" || path[1] !== "pads" || !Number.isInteger(path[2]) ||
        path.length > 12 || path.some(k => ["__proto__", "constructor", "prototype"].includes(k))) return null;
    if (d.padId) path[2] = preset.sampler?.pads.findIndex(p => p.id === d.padId) ?? -1;
    return path[2] >= 0 ? path : null;
  }
  const pathValue = (preset, path) => path?.reduce((object, key) => object?.[key], preset);
  async function readGroup(timbre, descriptors) {
    const parameters = [...new Set(descriptors.filter(d => !d.path).flatMap(d =>
      [d.parameter, ...(d.parameter === "pitch" ? ["pitch_range"] : [])]))];
    const [result, preset] = await Promise.all([
      parameters.length ? hooks.request("/api/control/request", { timbre, path: "/api/parameters/values", data: parameters }) : {},
      descriptors.some(d => d.path) ? hooks.request("/api/control/request", { timbre, path: "/api/preset" }) : null,
    ]);
    for (const d of descriptors) if (d.path) result[d.parameter] = pathValue(preset, pathOf(d, preset));
    return result;
  }
  const scale = (d, v, inverse = false) => {
    const s = d.scale;
    if (s?.type === "log1p") return inverse
      ? s.floor + Math.expm1(clamp(v) * Math.log1p(s.max - s.floor))
      : Math.log1p(Math.max(0, v - s.floor)) / Math.log1p(s.max - s.floor);
    if (s?.type === "log") return inverse ? s.min * (s.max / s.min) ** clamp(v)
      : Math.log(Math.max(s.min, v) / s.min) / Math.log(s.max / s.min);
    if (s?.type === "table" && s.values?.length > 1) {
      const table = s.values, first = table[0], last = table.at(-1);
      if (inverse) {
        const target = first + clamp(v) * (last - first);
        const right = table.findIndex(x => x >= target);
        if (right <= 0) return right === 0 ? 0 : 1;
        return (right - 1 + (target - table[right - 1]) / (table[right] - table[right - 1])) / (table.length - 1);
      }
      const pos = clamp(v) * (table.length - 1), left = Math.floor(pos);
      const physical = table[left] + (table[Math.min(left + 1, table.length - 1)] - table[left]) * (pos - left);
      return (physical - first) / (last - first);
    }
    return inverse ? d.min + clamp(v) * (d.max - d.min) : (v - d.min) / (d.max - d.min);
  };
  async function read(d, force = false) {
    if (!exists(d)) return null;
    const key = id(d), cached = values.get(key);
    if (cached && (!force || cached.pending || Date.now() - cached.time < 80)) return cached;
    if (reads.has(key)) return reads.get(key);
    const revision = cached?.revision || 0;
    const promise = readGroup(d.timbre, [d])
      .then(result => {
        const latest = values.get(key);
        if ((latest?.revision || 0) !== revision || latest?.pending) return latest;
        const value = result[d.parameter];
        if (!Number.isFinite(value)) { values.delete(key); return null; }
        const next = { value, pitchRange: result.pitch_range, time: Date.now(), revision, pending: false };
        values.set(key, next);
        return next;
      }).finally(() => reads.delete(key));
    reads.set(key, promise);
    return promise;
  }
  function snapshot(d) {
    if (!exists(d)) return null;
    const entry = values.get(id(d));
    if (!entry) return null;
    const unit = clamp(scale(d, entry.value));
    return { ...d, value: entry.value, unit, instrument: hooks.context()?.timbres?.find(t => t.id === d.timbre)?.name,
      text: d.parameter === "pitch" && entry.pitchRange
        ? `${((entry.value * 2 - 1) * entry.pitchRange).toFixed(1)} st`
        : `${Number(entry.value.toFixed(3))}` };
  }
  async function flush() {
    clearTimeout(timer); timer = null;
    if (running) { await running; if (pending.size) return flush(); return; }
    running = (async () => {
      await hooks.flush();
      while (pending.size) {
        const batch = [...pending.entries()]; pending.clear();
        const groups = new Map();
        for (const [key, edit] of batch) {
          if (!exists(edit.target)) { values.delete(key); continue; }
          const group = groups.get(edit.target.timbre) || [];
          group.push([key, edit]); groups.set(edit.target.timbre, group);
        }
        for (const [timbre, edits] of groups) {
          const scalar = edits.filter(([, edit]) => !edit.target.path), paths = edits.filter(([, edit]) => edit.target.path);
          if (scalar.length) await hooks.request("/api/control/request", { timbre, path: "/api/parameters",
            data: Object.fromEntries(scalar.map(([, edit]) => [edit.target.parameter, edit.value])) });
          if (paths.length) {
            const preset = await hooks.request("/api/control/request", { timbre, path: "/api/preset" });
            let changed = false;
            for (const [key, edit] of paths) {
              const path = pathOf(edit.target, preset), previous = pathValue(preset, path);
              if (!Number.isFinite(previous)) { values.delete(key); continue; }
              const object = path.slice(0, -1).reduce((object, key) => object[key], preset);
              object[path.at(-1)] = edit.value; changed = true;
            }
            if (changed) await hooks.request("/api/control/request", { timbre, path: "/api/preset", data: preset });
          }
          for (const [key, edit] of edits) {
            const current = values.get(key);
            if (current?.revision === edit.revision) { current.pending = false; current.time = Date.now(); }
          }
        }
      }
    })().catch(error => {
      pending.clear(); values.clear(); hooks.notify(error.message);
    }).finally(() => {
      running = null;
      if (pending.size && !timer) timer = setTimeout(() => { timer = null; flush(); }, 16);
    });
    return running;
  }
  function command(d, format, amount) {
    const key = id(d);
    const work = (queues.get(key) || Promise.resolve()).catch(() => {}).then(async () => {
      await hooks.flush();
      const current = await read(d, true);
      if (!current || !exists(d)) return;
      const pitch = d.parameter === "pitch", step = pitch ? .1 / (2 * Math.max(.1, current.pitchRange || 36)) : d.step;
      let value;
      if (format === "relative") value = step
        ? current.value + (amount - 64) * step
        : scale(d, scale(d, current.value) + (amount - 64) / 1024, true);
      else value = scale(d, amount / (format === "14bit" ? 16383 : 127), true);
      if (d.scale?.type === "table" && d.scale.quantum) {
        const first = d.scale.values[0], range = d.scale.values.at(-1) - first;
        const origin = Math.round((first + scale(d, current.value) * range) * 10) / 10;
        const physical = format === "relative" ? origin + (amount - 64) * d.scale.quantum
          : origin + Math.round((first + scale(d, value) * range - origin) / d.scale.quantum) * d.scale.quantum;
        value = scale(d, (physical - first) / range, true);
      }
      if (step) value = d.min + Math.round((value - d.min) / step) * step;
      value = clamp(value, d.min, d.max);
      if (value === current.value) return;
      const revision = current.revision + 1;
      values.set(key, { ...current, value, revision, pending: true, time: Date.now() });
      if (d.parameter === "pitch_range") {
        const pitch = values.get(`control:${d.timbre}:pitch`);
        if (pitch) pitch.pitchRange = value;
      }
      pending.set(key, { target: d, value, revision });
      hooks.dirty();
      if (!timer && !running) timer = setTimeout(() => { timer = null; flush(); }, 16);
    }).catch(error => hooks.notify(error.message));
    queues.set(key, work);
    work.finally(() => { if (queues.get(key) === work) queues.delete(key); });
    return work;
  }
  return {
    get enabled() { return !!hooks; },
    setup(next) { if (window.sadPetAudio && !window.sadPetControlSource) hooks = next; },
    describe(d) {
      if (!hooks || !Number.isInteger(d.timbre) || !d.parameter ||
          !Number.isFinite(d.min) || !Number.isFinite(d.max) || d.max <= d.min) return null;
      return { ...d, id: id(d) };
    },
    command,
    observe(timbre, parameter, value) {
      if (parameter === "pitch_range") {
        const pitch = values.get(`control:${timbre}:pitch`);
        if (pitch) pitch.pitchRange = value;
      }
    },
    value: snapshot,
    async refresh(targets) {
      if (!hooks) return;
      const groups = new Map();
      for (const d of targets.filter(exists)) {
        const cached = values.get(id(d));
        if (cached && (cached.pending || Date.now() - cached.time < 80)) continue;
        const group = groups.get(d.timbre) || [];
        group.push([d, cached?.revision || 0]); groups.set(d.timbre, group);
      }
      await Promise.allSettled([...groups].map(async ([timbre, group]) => {
        const result = await readGroup(timbre, group.map(([d]) => d));
        for (const [d, revision] of group) {
          const key = id(d), latest = values.get(key);
          if (latest?.pending || (latest?.revision || 0) !== revision) continue;
          if (!Number.isFinite(result[d.parameter])) { values.delete(key); continue; }
          values.set(key, { value: result[d.parameter], pitchRange: result.pitch_range,
            revision, time: Date.now(), pending: false });
        }
      }));
    },
    async flush() {
      if (!hooks) return;
      await Promise.allSettled([...queues.values()]);
      if (pending.size || running) return flush();
    },
  };
})();
