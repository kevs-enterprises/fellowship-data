// SOURCED FILE — do not edit here. Edit automation/publish-data/ in the generator; this copy is overwritten on the next sync.

/**
 * Fellowship game data, as a plain TypeScript package.
 *
 * Everything under `generated/` is produced by a tool and overwritten on each refresh. `types.ts`
 * is hand-written, and is the vocabulary the generated data is expressed in.
 *
 * ## Shape
 *
 * There is no runtime loading. The values are written into the modules themselves, so there is
 * nothing to fetch, nothing to await, and no failure mode where the data is missing:
 *
 * ```ts
 * import { HEROES } from "fellowship-data";
 *
 * const elarion = HEROES.find((hero) => hero.id === "Bowguy");
 * console.log(`${elarion?.name} - ${elarion?.title}`); // Elarion - The Skystrider
 * ```
 *
 * Import only the domains you need. The enemy difficulty curves are large; a bundler drops what
 * you do not reference, and a deep import makes that explicit:
 *
 * ```ts
 * import { MOBS } from "fellowship-data/generated/mobs.js";
 * ```
 *
 * Everything is `readonly`. This data is a constant, and a consumer that mutated it would be
 * mutating it for every other consumer in the process.
 *
 * ## Typing the JSON delivery
 *
 * These interfaces describe `json/*.json` as well as the data compiled in beside them — same field
 * names, same curve shape, `null` for an absent value in both. A `fetch()` of the JSON can be typed
 * with them without either side lying about the other.
 *
 * ## Gaps are values, not silence
 *
 * Where a number could not be resolved, the record says so rather than substituting a default. An
 * optional field is `null` because the game gives no value, never because the value was lost.
 */

export * from "./types.js";
export * from "./generated/index.js";
