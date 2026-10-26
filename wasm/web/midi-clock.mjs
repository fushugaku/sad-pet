// MIDI clock out: from the scene's transport as the audio worklet reports it,
// ticks (24 per quarter note), Start, Stop, Continue and Song Position,
// stamped with the time their beat sounds so Web MIDI sends them on time.

export const TICKS_PER_BEAT = 24;
/** How far ahead of the sound ticks are scheduled while the scene plays. */
export const LOOKAHEAD = 0.1;
/** A page that stalled longer than this many ticks skips them. */
const BURST = 48;
const TICK = 0xf8,
  START = 0xfa,
  CONTINUE = 0xfb,
  STOP = 0xfc,
  POSITION = 0xf2;

export class ClockSender {
  constructor() {
    this.reset();
  }
  /** Forgets the transport: the next report starts the clock afresh. */
  reset() {
    this.epoch = null;
    this.playing = false;
    // The next tick: its beat while playing, its time while stopped.
    this.beat = 0;
    this.time = null;
    // The time of the last message scheduled; nothing goes out before it.
    this.last = -Infinity;
  }
  /**
   * `transport` is the scene at a rendered frame: {time, beat, bpm, playing,
   * epoch}, `time` in seconds of the audio context; the epoch changes when
   * the scene starts, stops or moves. `now` is the context time sounding
   * now. Returns the messages to send, [{time, bytes}], in time order.
   */
  update(transport, now) {
    const out = [];
    const secondsPerBeat = 60 / transport.bpm;
    const period = secondsPerBeat / TICKS_PER_BEAT;
    const timeOf = (beat) =>
      transport.time + (beat - transport.beat) * secondsPerBeat;
    const send = (time, bytes) => {
      this.last = Math.max(time, now, this.last);
      out.push({ time: this.last, bytes });
    };
    if (transport.epoch !== this.epoch) {
      if (this.playing) send(transport.time, [STOP]);
      if (transport.playing) {
        // A slave counts every tick after Start or Continue, so the song
        // resumes on the first sixteenth after the ticks already scheduled.
        let sixteenth = Math.max(0, Math.ceil(transport.beat * 4 - 1e-6));
        while (timeOf(sixteenth / 4) <= this.last) sixteenth++;
        const time = timeOf(sixteenth / 4);
        if (sixteenth === 0) send(time, [START]);
        else {
          send(time, [POSITION, sixteenth & 0x7f, (sixteenth >> 7) & 0x7f]);
          send(time, [CONTINUE]);
        }
        this.beat = sixteenth / 4;
      } else if (this.time === null || this.playing)
        this.time = Math.max(transport.time, this.last);
      this.epoch = transport.epoch;
      this.playing = transport.playing;
    }
    // While stopped, ticks go no further than the audio rendered, so a Start
    // there finds none of them after it.
    const horizon = this.playing
      ? now + LOOKAHEAD
      : Math.min(now + LOOKAHEAD, transport.time);
    for (let sent = 0; ; sent++) {
      const time = this.playing ? timeOf(this.beat) : this.time;
      if (time > horizon) break;
      if (sent === BURST) {
        // The page stalled: carry on from now.
        if (this.playing)
          this.beat =
            Math.ceil(
              (transport.beat + (now - transport.time) / secondsPerBeat) *
                TICKS_PER_BEAT,
            ) / TICKS_PER_BEAT;
        else this.time = now;
        continue;
      }
      send(time, [TICK]);
      if (this.playing) this.beat += 1 / TICKS_PER_BEAT;
      else this.time = Math.max(time, now) + period;
    }
    return out;
  }
}

const CLOCK_STATUS = new Set([TICK, START, CONTINUE, STOP, POSITION]);

/**
 * The browser build's MIDI clock: clock messages from the chosen MIDI input
 * go to the audio worklet stamped with the time they sound, which follows
 * them when Follow is on; the scene's clock goes out to a MIDI output.
 * `backend` has `context`, `audio` (the worklet's Rpc) and `midiAccess`.
 */
export class MidiClock {
  constructor(backend) {
    this.backend = backend;
    this.follow = false;
    this.leadMs = 0;
    this.output = null;
    this.error = null;
    this.view = null;
    this.sender = new ClockSender();
  }
  /** [context time now sounding, performance time of a context time]. */
  clock() {
    const context = this.backend.context;
    const stamp = context.getOutputTimestamp?.();
    if (stamp?.performanceTime > 0)
      return [
        stamp.contextTime,
        (time) => stamp.performanceTime + (time - stamp.contextTime) * 1000,
      ];
    // Before the device reports: the render clock less its latency.
    const now =
      context.currentTime -
      (context.outputLatency || 0) -
      (context.baseLatency || 0);
    const at = performance.now();
    return [now, (time) => at + (time - now) * 1000];
  }
  /** Takes a Web MIDI message; returns whether it belonged to the clock. */
  input(event) {
    const [status, first = 0, second = 0] = event.data;
    if (!CLOCK_STATUS.has(status)) return false;
    const [now, toPerformance] = this.clock();
    // The context time at which the message's beat sounds.
    const time = now + (event.timeStamp - toPerformance(now)) / 1000;
    this.backend.audio.port.postMessage({
      type: "clock-event",
      data: [time, status, first, second],
    });
    return true;
  }
  outputs() {
    return [...(this.backend.midiAccess?.outputs.values() || [])].filter(
      (port) => port.state === "connected",
    );
  }
  /** `/api/clock`: {follow, lead_ms, out: an output's id or null}. */
  async request(data) {
    if (data) {
      if ("out" in data) {
        this.output =
          data.out == null
            ? null
            : this.outputs().find((port) => port.id === data.out) || null;
        if (data.out != null && !this.output)
          throw new Error("That MIDI output is no longer connected");
        this.error = null;
        this.sender.reset();
      }
      const engine = {};
      if (typeof data.follow === "boolean") engine.follow = data.follow;
      if (Number.isFinite(data.lead_ms))
        engine.lead_ms = Math.max(-500, Math.min(500, data.lead_ms));
      if (Object.keys(engine).length) {
        const state = await this.backend.audio.call(
          "request",
          engine,
          [],
          "/api/clock",
        );
        this.follow = state.follow;
        this.leadMs = state.lead_ms;
      }
      this.backend.audio.port.postMessage({
        type: "clock-active",
        data: this.follow || !!this.output,
      });
    }
    return {
      supported: { follow: true, send: true },
      follow: this.follow,
      lead_ms: this.leadMs,
      out: this.output?.id ?? null,
      outputs: this.outputs().map((port) => ({
        id: port.id,
        name: port.name || "MIDI output",
      })),
      ...this.status(),
    };
  }
  /** For /api/state. */
  status() {
    const view = this.view;
    return {
      follow: this.follow,
      present: !!view?.present,
      running: !!view?.running,
      tempo: view?.present ? view.tempo : null,
      sending: this.output?.name || null,
      error: this.error,
    };
  }
  /** The worklet's report of the scene's transport; sends clock out. */
  report({ view, time }) {
    const [, beat, bpm, playing, epoch, , present, running, tempo] = view;
    this.view = {
      present: !!present,
      running: !!running,
      tempo: tempo || null,
    };
    const output = this.output;
    if (!output) return;
    if (output.state === "disconnected") {
      this.output = null;
      this.error = `${output.name || "The MIDI output"} disconnected`;
      return;
    }
    const [now, toPerformance] = this.clock();
    const messages = this.sender.update(
      { time, beat, bpm, playing: !!playing, epoch },
      now,
    );
    try {
      for (const { time, bytes } of messages)
        output.send(bytes, toPerformance(time));
    } catch (error) {
      this.output = null;
      this.error = error.message;
    }
  }
}
