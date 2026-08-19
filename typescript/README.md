<!-- SOURCED FILE — do not edit this copy; it is overwritten on the next data sync. -->

# fellowship-data

Fellowship game data — heroes, abilities, items, talents, enemies, dungeons — as a TypeScript
package.

Everything is written into the modules themselves as `readonly` interfaces and `const` arrays.
There is no file to fetch, no await, and no state where the data is half-loaded.

```ts
import { HEROES } from "fellowship-data";

const elarion = HEROES.find((hero) => hero.id === "Bowguy");
console.log(`${elarion?.name} — ${elarion?.title}`); // Elarion — The Skystrider
```

## Installing it

```
npm install fellowship-data
```

No runtime dependencies of its own.

## Pick what you need

The package is side-effect free, so a bundler drops what you do not reference. Enemy difficulty
scaling is 440 curves of 151 points each — real weight if you're building a gear planner that never
looks at a mob — and a deep import makes leaving it out explicit:

```ts
import { MOBS } from "fellowship-data/generated/mobs.js";
```

## Typing the JSON delivery

These same types describe [`json/*.json`](https://github.com/kevs-enterprises/fellowship-data/tree/main/json)
as well as the data compiled in beside them — same field names, same curve shape, `null` for an
absent value in both. A `fetch()` of the JSON can be typed with them without either side lying
about the other:

```ts
import type { Hero } from "fellowship-data";

const heroes: readonly Hero[] = (await (await fetch(".../heroes.json")).json()).records;
```

## Missing values, curves, and media

`Value<T>`, `Curve`, and `MediaHandle` all carry the same rule across every language this dataset is
published in: a gap is a value, never a silent default, and a curve never interpolates between the
points the data actually states. See the [main README](https://github.com/kevs-enterprises/fellowship-data#readme)
for the full explanation and how it reads in each of the five deliveries.

## Versions and freshness

`BUILD_ID` tells you which game build the data came from, and every delivery in this repository
states the same one. Pin a version if you need a stable dataset.

## This is one of five deliveries

The same records are also published as a Rust crate, a Python package, plain JSON, and CSV
rectangles, all rendered from one description of each domain so they cannot disagree. See the
[repository](https://github.com/kevs-enterprises/fellowship-data) for the others and for what isn't
specific to TypeScript: images, legal, and the full set of conventions.

## Legal

This data is derived from a licensed copy of the game for interoperability with community tooling.
It contains no game assets, art, audio, or code — only facts about how the game is configured.
Fellowship is the property of its developers and publisher; this project is unaffiliated with them.

Licensed MIT.
