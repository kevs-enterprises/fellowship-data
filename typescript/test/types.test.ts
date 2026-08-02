// SOURCED FILE — do not edit here. Edit automation/publish/ in the generator; this copy is overwritten on the next sync.

/**
 * The vocabulary's behaviour, stated the same way every delivery states it.
 *
 * These mirror the tests the other deliveries carry. `at` is hand-written once per delivery, and
 * "does not interpolate" is the property the whole two-shaped curve exists to protect — so it is
 * the one most worth checking in each of them independently.
 */

import assert from "node:assert/strict";
import { test } from "node:test";

import { at, isEmpty, known, knownOrDeclaredDefault, DATAMINE } from "../src/types.js";
import type { Curve, Value } from "../src/types.js";

test("a dense curve is indexed from difficulty one", () => {
  const curve: Curve = { dense: [10, 20, 30] };
  assert.equal(at(curve, 1), 10);
  assert.equal(at(curve, 3), 30);
});

test("difficulty zero and out of range yield nothing", () => {
  const curve: Curve = { dense: [10, 20] };
  assert.equal(at(curve, 0), null);
  assert.equal(at(curve, -1), null);
  assert.equal(at(curve, 3), null);
});

test("a keyframed curve answers only at its keyframes", () => {
  const curve: Curve = { keyframes: [[1, 1], [151, 4]] };
  assert.equal(at(curve, 1), 1);
  assert.equal(at(curve, 151), 4);
});

test("a keyframed curve does not interpolate between keyframes", () => {
  // Interpolating here would invent a number nobody measured.
  const curve: Curve = { keyframes: [[1, 1], [151, 4]] };
  assert.equal(at(curve, 75), null);
});

test("emptiness is reported", () => {
  assert.equal(isEmpty({ dense: [] }), true);
  assert.equal(isEmpty({ dense: [1] }), false);
  assert.equal(isEmpty({ keyframes: [] }), true);
});

test("a known value reports itself", () => {
  const value: Value<number> = { known: 2.5 };
  assert.equal(known(value), 2.5);
});

test("a declared default is not reported as known", () => {
  // The distinction the whole type exists for: a declared fallback is not a measurement.
  const value: Value<number> = { curve_ref: "Bowguy.RangedRange", default: 1 };
  assert.equal(known(value), null);
  assert.equal(knownOrDeclaredDefault(value), 1);
});

test("an unresolved value offers nothing", () => {
  const value: Value<number> = { unresolved: "no constants row for this hero" };
  assert.equal(known(value), null);
  assert.equal(knownOrDeclaredDefault(value), null);
});

test("the datamine constant carries no authored detail", () => {
  assert.equal(DATAMINE.origin, "datamine");
  assert.equal(DATAMINE.dev_name, null);
  assert.equal(DATAMINE.source, null);
});
