"use strict";
(() => {
  const $ = (id) => document.getElementById(id);
  const clamp = (x, lo, hi) => Math.max(lo, Math.min(hi, x));
  const full = () => ({ start: 0, end: 1 });
  class SampleSelection {
    constructor(actions) {
      this.actions = actions;
      this.region = full();
      this.state = null;
      this.sending = null;
      this.queued = null;
      this.drag = null;
      this.saving = false;
      this.changedAt = 0;
      this.bind();
    }
    available() {
      return (
        !!this.state?.region &&
        !this.state.input?.active &&
        !this.state.input?.pending &&
        this.actions.library()?.ready &&
        !this.actions.library()?.loading &&
        !this.saving
      );
    }
    minimum() {
      return (
        1 /
        Math.max(
          1,
          this.state.sample_frames ||
            this.state.sample_seconds * (this.state.sample_rate || 48000),
        )
      );
    }
    sync(state) {
      const key = state.input?.active
        ? "live"
        : `${state.sample_id}:${state.sample_name}:${state.sample_seconds}`;
      if (this.key !== key) {
        this.key = key;
        this.region = state.region || full();
        this.drag = null;
        this.changedAt = 0;
      } else if (
        !this.drag &&
        !this.sending &&
        performance.now() - this.changedAt > 750
      ) {
        this.region = state.region || full();
      }
      this.state = state;
      this.render();
      if (this.focusId === state.sample_id && this.available()) {
        this.focusId = null;
        this.focus();
      }
    }
    render() {
      const available = this.available();
      $("sample-selection").hidden =
        !this.state?.region || !!this.state?.input?.active;
      for (const edge of ["start", "end"]) {
        const input = $(`region-${edge}`),
          handle = $(`region-${edge}-handle`);
        const seconds = this.region[edge] * (this.state?.sample_seconds || 0);
        if (document.activeElement !== input) input.value = seconds.toFixed(3);
        input.max = String(this.state?.sample_seconds || 0);
        input.disabled = !available;
        handle.hidden = $("sample-selection").hidden;
        handle.disabled = !available;
        handle.style.left = `clamp(14px, ${this.region[edge] * 100}%, calc(100% - 14px))`;
        handle.setAttribute("aria-valuemin", "0");
        handle.setAttribute(
          "aria-valuemax",
          String(this.state?.sample_seconds || 0),
        );
        handle.setAttribute("aria-valuenow", seconds.toFixed(3));
        handle.setAttribute("aria-valuetext", `${seconds.toFixed(3)} seconds`);
      }
      const duration =
        (this.region.end - this.region.start) *
        (this.state?.sample_seconds || 0);
      $("region-duration").textContent = `${duration.toFixed(3)} s loop`;
      const isFull = this.region.start === 0 && this.region.end === 1;
      $("region-reset").disabled = !available || isFull;
      $("region-save").disabled = !available || isFull;
      $("region-save").textContent = this.saving ? "Saving…" : "Save trim";
      $("drop-zone").classList.toggle("selecting", !!this.drag);
    }
    queue(region) {
      this.region = { ...region };
      this.queued = { ...region };
      this.error = null;
      this.changedAt = performance.now();
      this.render();
      if (this.sending) return;
      this.sending = (async () => {
        while (this.queued) {
          const next = this.queued;
          this.queued = null;
          const confirmed = await this.actions.request("/api/region", next);
          if (!this.queued && !this.drag) this.region = confirmed;
        }
      })()
        .catch((error) => {
          this.error = error;
          this.queued = null;
          this.region = this.state?.region || full();
          this.actions.notify(error.message, true);
        })
        .finally(() => {
          this.sending = null;
          this.changedAt = performance.now();
          this.render();
        });
    }
    async flush(strict = true) {
      if (this.sending) await this.sending;
      if (strict && this.error) throw this.error;
    }
    point(event) {
      const rect = $("wave").getBoundingClientRect();
      return clamp((event.clientX - rect.left) / rect.width, 0, 1);
    }
    edge(edge, value) {
      const region = { ...this.region },
        min = this.minimum();
      region[edge] =
        edge === "start"
          ? clamp(value, 0, region.end - min)
          : clamp(value, region.start + min, 1);
      this.queue(region);
    }
    bind() {
      const wave = $("drop-zone");
      wave.addEventListener("pointerdown", (event) => {
        if (!this.available() || event.button !== 0 || this.drag) return;
        const handle = event.target.closest(".region-handle");
        this.drag = {
          pointer: event.pointerId,
          edge: handle?.dataset.edge,
          anchor: this.point(event),
          x: event.clientX,
          previous: { ...this.region },
          moved: false,
        };
        wave.setPointerCapture(event.pointerId);
        event.preventDefault();
        if (handle) handle.focus();
      });
      wave.addEventListener("pointermove", (event) => {
        const drag = this.drag;
        if (!drag || event.pointerId !== drag.pointer) return;
        if (Math.abs(event.clientX - drag.x) < 4 && !drag.moved) return;
        drag.moved = true;
        const x = this.point(event);
        if (drag.edge) this.edge(drag.edge, x);
        else {
          const start = Math.min(drag.anchor, x),
            end = Math.max(drag.anchor, x);
          if (end - start >= this.minimum()) this.queue({ start, end });
        }
      });
      const finish = (event) => {
        const drag = this.drag;
        if (!drag || event.pointerId !== drag.pointer) return;
        this.drag = null;
        if (event.type === "pointercancel") this.queue(drag.previous);
        else if (!drag.moved && !drag.edge) {
          this.actions.position(
            clamp(
              (this.point(event) - this.region.start) /
                (this.region.end - this.region.start),
              0,
              1,
            ),
          );
        }
        if (wave.hasPointerCapture(event.pointerId))
          wave.releasePointerCapture(event.pointerId);
        this.render();
      };
      wave.addEventListener("pointerup", finish);
      wave.addEventListener("pointercancel", finish);
      for (const edge of ["start", "end"]) {
        $(`region-${edge}`).oninput = (event) => {
          const value = event.target.valueAsNumber / this.state.sample_seconds;
          if (
            Number.isFinite(value) &&
            value >= 0 &&
            value <= 1 &&
            (edge === "start"
              ? value <= this.region.end - this.minimum()
              : value >= this.region.start + this.minimum())
          )
            this.edge(edge, value);
        };
        $(`region-${edge}`).onchange = (event) => {
          if (
            !event.target.value ||
            !Number.isFinite(event.target.valueAsNumber)
          ) {
            event.target.value = (
              this.region[edge] * this.state.sample_seconds
            ).toFixed(3);
            return;
          }
          this.edge(
            edge,
            event.target.valueAsNumber / this.state.sample_seconds,
          );
          event.target.value = (
            this.region[edge] * this.state.sample_seconds
          ).toFixed(3);
        };
        $(`region-${edge}-handle`).onkeydown = (event) => {
          if (!["ArrowLeft", "ArrowRight", "Home", "End"].includes(event.key))
            return;
          event.preventDefault();
          const step =
            (event.shiftKey ? 0.1 : 0.01) / this.state.sample_seconds;
          this.edge(
            edge,
            event.key === "Home"
              ? 0
              : event.key === "End"
                ? 1
                : this.region[edge] +
                  (event.key === "ArrowLeft" ? -step : step),
          );
        };
      }
      $("region-reset").onclick = () => this.queue(full());
      $("region-save").onclick = () => this.save();
    }
    draw(context, width, height) {
      if (!this.state?.region || this.state.input?.active) return;
      const start = this.region.start * width,
        end = this.region.end * width;
      context.fillStyle = "#0d140db3";
      context.fillRect(0, 0, start, height);
      context.fillRect(end, 0, width - end, height);
      context.fillStyle = "#c4f48609";
      context.fillRect(start, 0, end - start, height);
      context.strokeStyle = "#c4f486";
      context.lineWidth = 1;
      context.beginPath();
      context.moveTo(Math.max(0.5, start), 0);
      context.lineTo(Math.max(0.5, start), height);
      context.moveTo(Math.min(width - 0.5, end), 0);
      context.lineTo(Math.min(width - 0.5, end), height);
      context.stroke();
    }
    async save() {
      if (!this.available()) return;
      this.saving = true;
      this.render();
      try {
        await this.flush();
        const library = this.actions.library(),
          source = library.files.find((file) => file.id === library.currentId);
        const region = { ...this.region },
          seconds = this.state.sample_seconds;
        const response = await this.actions.api("/api/trim", {
          ...region,
          sample_id: this.state.sample_id,
        });
        const label = `${(source?.displayName || this.state.sample_name).replace(/\.wav$/i, "")} · ${(region.start * seconds).toFixed(3)}–${(region.end * seconds).toFixed(3)} s`;
        const record = await library.storeBlob(await response.blob(), {
          kind: "audio",
          name: `${(source?.name || this.state.sample_name).replace(/\.wav$/i, "")}-trim.wav`,
          displayName: label,
          seconds: (region.end - region.start) * seconds,
          trimmedFrom: source?.id,
          region,
        });
        await library.activate(record);
        this.actions.saved(record);
        this.actions.notify("Trim saved in Library");
      } catch (error) {
        this.actions.notify(error.message, true);
      } finally {
        this.saving = false;
        this.render();
      }
    }
    focus(id) {
      if (id && (id !== this.state?.sample_id || !this.available())) {
        this.focusId = id;
        return;
      }
      $("drop-zone").scrollIntoView({ block: "center", behavior: "smooth" });
      $("region-start").focus({ preventScroll: true });
    }
  }
  window.SadSampleSelection = SampleSelection;
})();
