// The Vocal editor's lyric splitting, without a browser: one entry per note,
// small kana joined to their mora, hyphenated words spread over notes.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import vm from "node:vm";

const source = await readFile(new URL("../rust/web/vocal.js", import.meta.url), "utf8");
const window = {};
vm.runInNewContext(source, { window, document: {}, ResizeObserver: class {} });
// Arrays from the sandbox have its own prototype; compare plain copies.
const split = (text, language) => [...window.SadVocal.splitLyrics(text, language)];

assert.deepEqual(split("さくら さくら", "ja"), ["さ", "く", "ら", "さ", "く", "ら"]);
assert.deepEqual(split("きゃっと ちょー", "ja"), ["きゃ", "っ", "と", "ちょ", "ー"]);
assert.deepEqual(split("twin-kle twin-kle lit-tle star", "en"), ["twin-", "kle", "twin-", "kle", "lit-", "tle", "star"]);
assert.deepEqual(split("в ле-су ро-ди-лась - ё-лоч-ка", "ru"), ["в", "ле-", "су", "ро-", "ди-", "лась", "-", "ё-", "лоч-", "ка"]);
assert.deepEqual(split("  hello +  world ", "en"), ["hello", "+", "world"]);
console.log("vocal editor lyrics ok");
