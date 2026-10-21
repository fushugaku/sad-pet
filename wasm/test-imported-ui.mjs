// Execute the shipping FX schema so every imported Rust control is reachable and
// every knob formatter can run before opening an audio device.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import vm from "node:vm";
const source = await readFile(
  process.argv[2] ?? new URL("../rust/web/app.js", import.meta.url),
  "utf8",
);
const rust = await readFile(
  new URL("../rust/src/color.rs", import.meta.url),
  "utf8",
);
const context = {
  percent: (v) => `${Math.round(v * 100)}%`,
  signedPercent: (v) => `${v * 100}%`,
  preset: { params: { bpm: 120 } },
  colorDefaults: { slot: { settings: {} } },
};
vm.createContext(context);
const start = source.indexOf("const COLOR_TYPES ="),
  end = source.indexOf("const destinationValue =");
assert.ok(start > 0 && end > start);
vm.runInContext(source.slice(start, end), context);
for (const [kind, title, count] of [
  ["super_echo", "SuperEcho", 14],
  ["alpha_verb", "AlphaVerb", 19],
]) {
  assert.ok(
    vm.runInContext(
      `COLOR_TYPES.some(([key,label])=>key==='${kind}'&&label==='${title}')`,
      context,
    ),
  );
  const schema = vm.runInContext(
    `colorSchema('${kind}',{settings:{super_echo_sync:0}})`,
    context,
  );
  assert.equal(schema.length, count);
  for (const [key, label, format] of schema) {
    assert.ok(key && label);
    for (const value of [0, 0.37, 1]) {
      const text = format(value, { settings: {} });
      assert.equal(typeof text, "string");
      assert.ok(
        !/NaN|undefined|Infinity/.test(text),
        `${kind}/${key}: ${text}`,
      );
    }
  }
  const destinations = vm.runInContext(
    `colorDestinationSchema('${kind}').map(([p])=>p)`,
    context,
  );
  const fields = [
    ...rust.matchAll(new RegExp(`\\w+ => (${kind}_[a-z_]+),`, "g")),
  ].map((m) => m[1]);
  assert.equal(fields.length, 21);
  for (const field of fields)
    assert.ok(
      destinations.includes(field),
      `Unreachable modulation control ${field}`,
    );
  for (const [key, label, values] of vm.runInContext(
    `IMPORTED_FX_CHOICES.${kind}`,
    context,
  )) {
    assert.ok(fields.includes(key) && label && values.length >= 2);
  }
}
const manifest = JSON.parse(
  await readFile(
    new URL("../rust/tests/fixtures/erika-fx/manifest.json", import.meta.url),
    "utf8",
  ),
);
const dynamics = vm.runInContext(
  "IMPORTED_FX_CHOICES.alpha_verb.find(([p]) => p === 'alpha_verb_dynamics')[2]",
  context,
);
for (const mode of manifest.alpha_verb.dynamics_modes)
  assert.equal(
    dynamics[Math.round(mode.normalized * 3)],
    mode.label,
    "AlphaVerb mode name does not match the original firmware value",
  );
const synced = vm.runInContext(
  `colorSchema('super_echo',{settings:{super_echo_sync:1}})`,
  context,
);
assert.equal(synced[0][0], "super_echo_note");
assert.equal(synced[0][2](0.5), "1/4");
assert.equal(synced[0][2](1), "2/1");
console.log(
  "SuperEcho / AlphaVerb UI: all 42 controls, 33 knobs, original modes, sync divisions and formatters passed.",
);

// The ratio gesture must cross RADIAS table plateaus in exact quarter-percent
// increments, with either clock and regardless of the Smooth toggle.
const delayRust = await readFile(new URL('../rust/src/radias_delay.rs', import.meta.url), 'utf8');
const delayTables = {};
for (const [name, key] of [['FREE_RATIO','free_ratio'],['SYNC_RATIO','sync_ratio']]) {
  const match = new RegExp(`pub const ${name}: \\[u16; 128\\] = \\[([\\s\\S]*?)\\];`).exec(delayRust);
  assert.ok(match, name);
  delayTables[key] = match[1].split(',').map(v => v.trim()).filter(Boolean).map(Number);
}
context.colorDefaults.delay_tables = delayTables;
for (const sync of [false, true]) {
  for (const smooth of [false, true]) {
    context.slot = {settings:{lcr_sync:sync,lcr_smooth:smooth}};
    const binding = vm.runInContext('lcrRatioBinding(() => slot)', context);
    const display = v => vm.runInContext(`delayRatio(${v},slot,'lcr')`, context);
    let value = 0;
    const minimum = sync ? 12.5 : 0.5;
    assert.equal(parseFloat(display(value)), minimum);
    for (let expected = minimum + 0.25; expected <= 400; expected += 0.25) {
      value = Math.fround(binding.increment(value, 1));
      assert.equal(display(value), `${expected.toFixed(2)}%`, `${sync}/${smooth} up`);
    }
    for (let expected = 399.75; expected >= minimum; expected -= 0.25) {
      value = Math.fround(binding.increment(value, -1));
      assert.equal(display(value), `${expected.toFixed(2)}%`, `${sync}/${smooth} down`);
    }
    // Old presets include 16.7%, 66.6% etc.; a key gesture must add 0.25%,
    // rather than snap that first gesture to a global quarter-percent grid.
    for (let i = 0; i < 120; i++) {
      const before = parseFloat(display(i / 127));
      const after = parseFloat(display(Math.fround(binding.increment(i / 127, 1))));
      assert.ok(Math.abs(after - before - 0.25) < 0.011, `${before} -> ${after}`);
    }
    for (const unit of [0, 0.001, 0.1, 0.25, 0.5, 0.9, 1]) {
      const v = Math.fround(binding.quantize(binding.denormalize(unit)));
      assert.ok(Math.abs(binding.normalize(v) - unit) < 0.0004);
    }
  }
}
console.log('L/C/R Time Ratio: continuous physical gestures, 0.25% up/down across all plateaus, both clocks and Smooth modes passed.');
