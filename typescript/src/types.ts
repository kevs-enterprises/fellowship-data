// SOURCED FILE — do not edit this copy; it is overwritten on the next data sync.

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
export type Origin = "datamine" | "derived" | "overlay";

/** Strength of the immutable source binding behind a published fact. */
export type SourceAuthority = "producer_bound" | "legacy_unbound" | "unclassified";

/** Provenance carried alongside a record. */
export interface Provenance {
  readonly origin: Origin;
  /** The developer-facing identifier, where one exists. */
  readonly dev_name: string | null;
  /** For `"derived"`, the transform. For `"overlay"`, the reason. */
  readonly source: string | null;
  /** Acquisition authority of the weakest immutable input behind this record. */
  readonly source_authority: SourceAuthority;
}

/** Read directly from the game's own data. */
export const DATAMINE_UNCLASSIFIED: Provenance = {
  origin: "datamine",
  dev_name: null,
  source: null,
  source_authority: "unclassified",
};

/** Stable consumer identity for an Ability, represented as a canonical UUID string. */
export type AbilityGuid = string & { readonly __brand: "AbilityGuid" };

/** The public protocol used to derive Ability GUIDs. */
export interface AbilityGuidScheme {
  readonly name: string;
  /** Canonical UUID text. This is a namespace, not an entity identity. */
  readonly namespace: string;
  readonly version: number;
  readonly transform: string;
  readonly origin: Origin;
}

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
/** A source Ability occurrence paired with its stable consumer identity. */
export interface AbilityRef {
  readonly source_id: AbilityId;
  readonly guid: AbilityGuid;
}
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
/** Identifies an item trait. */
export type TraitId = string & { readonly __brand: "TraitId" };
/** Identifies an armor-set bonus. */
export type SetBonusId = string & { readonly __brand: "SetBonusId" };
/** Identifies a finesse. */
export type FinesseId = string & { readonly __brand: "FinesseId" };
/** Identifies a relic power. */
export type RelicPowerId = string & { readonly __brand: "RelicPowerId" };
/** Identifies a gameplay tag, as `<namespace>/<tag>`. */
export type TagId = string & { readonly __brand: "TagId" };
/** Identifies a combat-model constant by its dotted path. */
export type CombatConstantId = string & { readonly __brand: "CombatConstantId" };
