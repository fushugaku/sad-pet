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
