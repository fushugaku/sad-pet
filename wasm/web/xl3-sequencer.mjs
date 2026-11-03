// The row nearest the faders is row 3 in Novation's MIDI protocol.
const PARAMETERS = [
  { cc: 29, key: "pitch", label: "Pitch", field: 0, min: 0, max: 127 },
  { cc: 30, key: "gate_override", label: "Gate", field: 9, min: 0, max: 100 },
  { cc: 31, key: "velocity", label: "Velocity", field: 15, min: 1, max: 127 },
  { cc: 32, key: "ratchet", label: "Ratchet", field: 3, min: 1, max: 8 },
  { cc: 33, key: "slide", label: "Slide", field: 5, min: 0, max: 1 },
  { cc: 34, key: "gate", label: "Gate type", field: 2, min: 0, max: 3 },
  { cc: 35, key: "pulses", label: "Pulses", field: 1, min: 1, max: 8 },
  ...[0, 1, 2].map(i => ({ cc: 21 + i, key: `destination${i + 1}`,
    label: `Dest ${i + 1}`, field: 16 + i, min: -1, max: 1, increment: .01 })),
];
const clamp = (value, min, max) => Math.max(min, Math.min(max, value));

/** Edits the browser's current Neuropolis track through its normal edit queue. */
export class Xl3Sequencer {
  constructor(read = () => null) {
    this.read = read;
    this.held = new Map();
    this.down = new Set();
  }
  reset(keepButtons = false) {
    this.held.clear();
    if (!keepButtons) this.down.clear();
    this.key = null;
    if (this.editor) {
      this.editor.controllerHeld = new Set();
      this.editor.syncSelection();
    }
    this.editor = null;
  }
  current() {
    const source = this.read(), editor = source?.editor, track = editor?.track();
    const offset = track ? editor.offset(track) : 0;
    // A release from a previous instrument/track/page must never toggle a
    // step on the newly visible track, even when track IDs happen to match.
    const key = track ? `${source.context}:${track.id}:${offset}` : null;
    if (key !== this.key || editor !== this.editor) {
      this.reset(true); this.key = key; this.editor = editor;
    }
    return track ? { ...source, editor, track, offset, key } : null;
  }
  paint(current) {
    if (!current) return;
    current.editor.controllerHeld = new Set([...this.held.values()].map(h => h.stage));
    current.editor.syncSelection();
  }
  shift() {
    // A held note used as part of a page gesture is no longer a plain tap.
    for (const held of this.held.values()) held.used = true;
  }
  button(index, value, shifted) {
    const repeated = this.down.has(index);
    value ? this.down.add(index) : this.down.delete(index);
    const current = this.current();
    if (!current) return;
    const { editor, track, offset, key } = current;
    if (value) {
      if (repeated) return;
      if (shifted) {
        this.shift();
        if (index < editor.pages(track)) editor.showPage(index);
        this.current();
      } else {
        this.held.set(index, { stage: offset + index, key, used: false });
        this.paint(current);
      }
      return;
    }
    const held = this.held.get(index);
    this.held.delete(index);
    if (held && held.key === key && !held.used && track.steps[held.stage])
      editor.edit(track.id, held.stage, 2, track.steps[held.stage].gate === 0 ? 2 : 0);
    this.paint(current);
  }
  targets(current) {
    const stages = new Set([...current.editor.selection]
      .filter(n => Number.isInteger(n) && n >= 0 && n < 16).map(n => current.offset + n));
    for (const held of this.held.values()) if (held.key === current.key) stages.add(held.stage);
    return [...stages].filter(n => current.track.steps[n]);
  }
  value(parameter, track, step) {
    if (parameter.key === "gate_override" && step.gate_override < 0) return track.config[7];
    return Number(step[parameter.key] ?? 0);
  }
  turn(cc, value) {
    const parameter = PARAMETERS.find(p => p.cc === cc), delta = value - 64;
    const current = this.current();
    if (!parameter || !delta || !current) return;
    const { editor, track } = current, targets = this.targets(current);
    for (const held of this.held.values()) held.used = true;
    for (const stage of targets) {
      const step = editor.track(track.id)?.steps[stage];
      if (!step) continue;
      const previous = this.value(parameter, track, step);
      let next;
      if (parameter.key === "pitch") {
        const notes = editor.sliderNotes(track);
        next = notes[clamp(editor.slot(notes, previous) + delta, 0, notes.length - 1)];
      } else if (parameter.key === "slide" || (parameter.field >= 16 &&
          /^lfo\d+_reset$/.test(track.destinations?.[parameter.field - 16] || ""))) {
        next = Number(delta > 0);
      } else next = clamp(Math.round((previous + delta * (parameter.increment || 1)) * 100) / 100,
        parameter.min, parameter.max);
      if (previous !== next || parameter.key === "gate_override" && step.gate_override < 0)
        editor.edit(track.id, stage, parameter.field, next, false);
    }
    if (targets.length) editor.sync();
  }
  navigate(direction) {
    const current = this.current();
    if (!current) return;
    const { editor, track } = current, tracks = editor.program().tracks;
    const index = tracks.findIndex(t => t.id === track.id), next = tracks[index + direction];
    if (!next) return;
    this.reset(true);
    editor.selected = next.id;
    editor.setPage(Math.floor((index + direction) / 64));
  }
  feedback(shifted = false) {
    const current = this.current();
    const controls = new Map(), colors = new Map();
    for (let cc = 37; cc <= 52; cc++) colors.set(cc, [0, 0, 0]);
    if (!current) return { key: "sequencer:empty", controls, colors,
      fields: ["Sequencer", "No track selected", "Select a track in the browser"] };
    const { editor, track, offset } = current, page = offset / 16;
    const targets = this.targets(current), selected = new Set(targets);
    const playing = current.playing && editor.program().enabled && !track.config[8] &&
      current.view?.tracks?.slice(0, current.view.count).find(t => t.id === track.id);
    for (let i = 0; i < 16; i++) {
      const step = track.steps[offset + i];
      colors.set(37 + i, shifted
        ? i >= editor.pages(track) ? [0, 0, 0] : i === page ? [100, 100, 127] : [0, 30, 80]
        : playing && playing.step === offset + i ? [127, 127, 127]
        : selected.has(offset + i) ? [0, 100, 127]
        : step && !step.skip && step.gate !== 0 ? [0, 75, 8] : [4, 8, 4]);
    }
    const tracks = editor.program().tracks, index = tracks.findIndex(t => t.id === track.id);
    colors.set(103, index > 0 ? [0, 80, 100] : [0, 0, 0]);
    colors.set(102, index < tracks.length - 1 ? [0, 80, 100] : [0, 0, 0]);
    for (const parameter of PARAMETERS) {
      const stage = targets[0], step = track.steps[stage];
      const values = targets.map(n => this.value(parameter, track, track.steps[n]));
      const value = step ? values[0] : parameter.min;
      let text = targets.length ? values.some(v => v !== value) ? "Mixed" : String(value) : "Hold or select steps";
      if (targets.length && text !== "Mixed") {
        if (parameter.key === "pitch") text = editor.pitchText(track, value);
        if (parameter.key === "gate_override") text = `${value}%`;
        if (parameter.key === "slide") text = value ? "On" : "Off";
        if (parameter.key === "gate") text = ["Rest", "Single", "Multi", "Hold"][value];
        if (parameter.field >= 16) text = editor.depthText(track.destinations?.[parameter.field - 16], value);
      }
      controls.set(parameter.cc, { key: `sequencer:${track.id}:${parameter.key}`, controller: parameter.cc,
        parameter: parameter.key, label: parameter.label, instrument: `${current.instrument} / ${track.name}`,
        available: true, unit: (value - parameter.min) / (parameter.max - parameter.min), text });
    }
    return { key: `sequencer:${current.key}`, controls, colors,
      fields: [current.instrument || "Sequencer", track.name, shifted
        ? `Steps ${offset + 1}-${offset + 16} / ${editor.pages(track) * 16}`
        : `${offset + 1}-${offset + 16}${targets.length ? ` / ${targets.length} selected` : " / Hold or select steps"}`] };
  }
}
