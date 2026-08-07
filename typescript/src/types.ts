// SOURCED FILE — do not edit here. Edit automation/publish/ in the generator; this copy is overwritten on the next sync.

/**
 * The vocabulary the data is expressed in.
 *
 * Unlike everything under `generated/`, this file is hand-written. It states the same types the
 * other deliveries state, in TypeScript's idiom — with two deliberate exceptions, both so these
 * types can describe the JSON delivery as well as the data compiled in beside them. See `Curve`,
 * and note that fields keep the snake case every delivery uses rather than being camel-cased here.
 */

/**
 * A difficulty-indexed curve.
 *
 * Two genuinely different shapes occur, and flattening them together would assert something
 * untrue. Enemy scaling is dense — one value per difficulty from 1 to 151. Hero scaling is a pair
 * of keyframes at the ends of that range, describing a ramp rather than a table.
 *
 * The union is keyed by which property is present rather than by a `kind` tag, because that is the
 * shape `json/*.json` carries. `"dense" in curve` narrows it exactly as a tag would.
 */
export type Curve =
  | { readonly dense: readonly number[] }
  | { readonly keyframes: readonly (readonly [number, number])[] };

/**
 * The value at an exact difficulty, when the curve states one.
 *
 * Returns `null` rather than interpolating between keyframes. How the game interpolates is a
 * modelling decision for the caller; guessing it here would manufacture numbers nobody measured.
 */
export function at(curve: Curve, difficulty: number): number | null {
  if ("dense" in curve) {
    if (difficulty < 1 || difficulty > curve.dense.length) return null;
    return curve.dense[difficulty - 1] ?? null;
  }
  for (const [point, value] of curve.keyframes) {
    if (point === difficulty) return value;
  }
  return null;
}

/** Whether the curve states anything at all. */
export function isEmpty(curve: Curve): boolean {
  return "dense" in curve ? curve.dense.length === 0 : curve.keyframes.length === 0;
}

/**
 * A value that could not be fully resolved.
 *
 * Collapsing an unresolved value to a plain number turns "we do not know" into "it is 1.0", and
 * the two read identically at the call site. Modelling the gap in the type forces a decision.
 */
export type Value<T> =
  | { readonly known: T }
  | { readonly curve_ref: string; readonly default: T | null }
  | { readonly unresolved: string };

/** The value only when it was actually measured. */
export function known<T>(value: Value<T>): T | null {
  return "known" in value ? value.known : null;
}

/**
 * The measured value, or the asset's declared fallback.
 *
 * Named the long way to make the compromise visible at the call site: a fallback is what the asset
 * says to use, not what the game was observed to do.
 */
export function knownOrDeclaredDefault<T>(value: Value<T>): T | null {
  if ("known" in value) return value.known;
  if ("curve_ref" in value) return value.default;
  return null;
}

/**
 * Where a record came from.
 *
 * Kept on every record because a hand-authored correction and a value read from the game must
 * never be indistinguishable to a consumer.
 */
/**
 * Whether a modelled formula has been checked against a recorded capture.
 *
 * Not a confidence number. "Never checked" and "checked and disagreed" are different states with
 * independent failure modes, and one scalar cannot carry both. A contradicted formula still
 * publishes: the marker records that two methods disagreed, not which one is wrong.
 */
export type Validation =
  | { readonly status: "validated" }
  | { readonly status: "unvalidated"; readonly reason: string }
  | { readonly status: "contradicted"; readonly reason: string; readonly delta: number };

/** Whether an oracle has agreed with this formula. */
export function isValidated(validation: Validation): boolean {
  return validation.status === "validated";
}

/** The observed disagreement, when one was measured. */
export function contradictionDelta(validation: Validation): number | null {
  return validation.status === "contradicted" ? validation.delta : null;
}

/** One axis of how much standing an extraction has. */
export type Grade = "low" | "medium" | "high" | "verified";

/**
 * How much standing the extraction behind a formula has.
 *
 * A separate axis from {@link Validation}, not a finer grade of it. Validation asks whether an
 * oracle agreed; this asks how well the value was recovered in the first place. Graded on the
 * three axes the extraction actually has — coverage is a corpus-level summary and is deliberately
 * absent, because it is not a property of any one formula.
 */
export type Confidence =
  | { readonly status: "unassessed"; readonly reason: string }
  | {
      readonly status: "graded";
      /** How firmly the value is bound to the thing it claims to describe. */
      readonly binding: Grade;
      /** How reliably the bytes were read. */
      readonly extraction: Grade;
      /** How well what was read is understood to mean what it is published as. */
      readonly interpretation: Grade;
    };

/**
 * Whether this extraction has been graded at all.
 *
 * `!isGraded()` means nobody assessed it, which is not the same as a low grade.
 */
export function isGraded(confidence: Confidence): boolean {
  return confidence.status === "graded";
}

export type Origin = "datamine" | "derived" | "overlay";

/** Provenance carried alongside a record. */
export interface Provenance {
  readonly origin: Origin;
  /** The developer-facing identifier, where one exists. */
  readonly dev_name: string | null;
  /** For `"derived"`, the transform. For `"overlay"`, the reason. */
  readonly source: string | null;
}

/** Read directly from the game's own data. */
export const DATAMINE: Provenance = { origin: "datamine", dev_name: null, source: null };

/** What a media handle points at. */
export type MediaKind =
  | "HeroPortrait"
  | "HeroSelectBackground"
  | "AbilityIcon"
  | "ItemIcon"
  | "TalentIcon"
  | "MobPortrait"
  | "DungeonMap"
  | "Other";

/**
 * A reference to an image this package does not contain.
 *
 * A handle carries a stable id and the source dimensions, never a path. Map `id` onto wherever you
 * serve art from; ids stay stable across refreshes, so asset paths keep working.
 */
export interface MediaHandle {
  /** Stable across re-exports, so a consumer's own asset paths stay valid. */
  readonly id: string;
  readonly kind: MediaKind;
  readonly width: number;
  readonly height: number;
}

// Identifiers are all strings and several namespaces overlap, so each kind is its own branded type.
// The brand exists only at compile time — at runtime these are strings — and it makes the compiler
// refuse to pass a hero where an ability is expected, which is what the Rust delivery's newtypes
// do. `__brand` is never present on a value; it is a phantom property.

/** Identifies a playable hero. */
export type HeroId = string & { readonly __brand: "HeroId" };
/** Identifies an ability or passive. */
export type AbilityId = string & { readonly __brand: "AbilityId" };
/** Identifies an item. */
export type ItemId = string & { readonly __brand: "ItemId" };
/** Identifies an enemy or boss. */
export type MobId = string & { readonly __brand: "MobId" };
/** Identifies a character attribute. */
export type AttributeId = string & { readonly __brand: "AttributeId" };
/** Identifies a gameplay effect. */
export type EffectId = string & { readonly __brand: "EffectId" };
/** Identifies a talent. */
export type TalentId = string & { readonly __brand: "TalentId" };
/** Identifies a dungeon or zone. */
export type DungeonId = string & { readonly __brand: "DungeonId" };
