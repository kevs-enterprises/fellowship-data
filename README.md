# fellowship-data

Fellowship game data — heroes, abilities, items, talents, enemies, dungeons — in four forms.

| | Install | Read it |
|---|---|---|
| [`rust/`](rust/) | `fellowship-data = { git = "…" }` | `use fellowship_data::generated::heroes::HEROES;` |
| [`python/`](python/) | `pip install fellowship-data` | `from fellowship_data.generated.heroes import HEROES` |
| [`typescript/`](typescript/) | `npm install fellowship-data` | `import { HEROES } from "fellowship-data";` |
| [`json/`](json/) | — | `json/heroes.json` |

None of them is the real one. They are rendered from a single description of each domain, so a
field cannot exist in one and not another, and every refresh checks that they still agree before
publishing. Pick whichever fits what you are building.

```rust
let elarion = HEROES.iter().find(|hero| hero.id.as_str() == "Bowguy").unwrap();
println!("{} — {}", elarion.name, elarion.title);   // Elarion — The Skystrider
```

```python
elarion = next(hero for hero in HEROES if hero.id == "Bowguy")
print(f"{elarion.name} — {elarion.title}")          # Elarion — The Skystrider
```

```ts
const elarion = HEROES.find((hero) => hero.id === "Bowguy");
console.log(`${elarion?.name} — ${elarion?.title}`); // Elarion — The Skystrider
```

## Nothing is loaded at runtime

In all three package deliveries the values are written into the source itself. There is no file to
find, no parse to fail, no async setup, and no state where the data is half-loaded. Import it and
read it.

That is worth the file sizes it costs. The dataset's enemy scaling is 440 curves of 151 points
each, and as `{"difficulty": 1.0, "value": 1.0}` objects it takes roughly 38 bytes to carry one
number. Written as source it is a few hundred kilobytes and no parse at all.

Everything is immutable — `&'static` in Rust, frozen dataclasses and tuples in Python, `readonly`
in TypeScript. This data is a constant, and a consumer that mutated it would be mutating it for
every other consumer in the process.

### Take only what you need

Enemy difficulty scaling is real weight if you are building a gear planner that never looks at a
mob, so each delivery lets you leave it out.

Rust uses features — available: `heroes` · `abilities` · `talents` · `items` · `attributes` ·
`constants` · `effects` · `modifiers` · `dungeons` · `mobs`, with `heroes`, `abilities`, `items`,
`attributes` and `constants` on by default:

```toml
fellowship-data = { git = "…", default-features = false, features = ["heroes", "items"] }
```

Python imports per domain, and nothing loads a domain you did not ask for:

```python
from fellowship_data.generated import heroes, items
```

TypeScript is side-effect free, so a bundler drops what you do not reference. A deep import makes
that explicit:

```ts
import { HEROES } from "fellowship-data/generated/heroes.js";
```

## Missing numbers stay missing

Some values in the game are defined by curves or lookup tables that this data doesn't resolve.
Where that happens the record says so, instead of quietly filling in a plausible number:

```rust
match ability.max_range {
    Value::Known(range)                  => // measured from the game's own data
    Value::Curve { curve_ref, default }  => // defined by a curve we haven't resolved
    Value::Unresolved { reason }         => // known to exist, no value recovered
}
```

`known()` gives you a value only for a real measurement. There's also
`known_or_declared_default()`, named the long way on purpose: a declared default is what the asset
falls back to, not what the game was observed to do. If you use it, you should know you did.

An optional field is empty — `None`, `None`, `null`, `null` — because the game gives no value
there. It is never a stand-in for zero, for an empty string, or for a number that exists upstream
and wasn't recovered.

Curves work the same way. A dense curve has one value per difficulty from 1. A keyframed curve has
only the points the data actually states — heroes typically carry two, at difficulty 1 and 151 —
and asking for a difficulty in between gives you nothing rather than a number nobody measured. If
your model wants a value there, interpolate deliberately, with your own assumptions visible.

This is the dataset's one opinion: it would rather be awkward than confidently wrong.

## Images

There aren't any. Portraits and icons are referenced by a media handle, which gives you a stable id
and the source dimensions but no path or URL — you map the id onto wherever you serve art from.

```rust
if let Some(portrait) = hero.portrait {
    let url = format!("/game-media/heroes/{}.webp", portrait.id);
}
```

Ids stay stable across data refreshes, so your asset paths keep working.

## Reading the JSON

One file per domain, plus an `index.json` saying what the folder holds. One record per line — these
files are large, and this keeps a data refresh diffing the entities that changed instead of
reflowing the whole file.

```json
{"id":"Bowguy","name":"Elarion","title":"The Skystrider","base_health":1687.0,"...":"..."}
```

Curves carry their shape: `{"dense": [...]}` or `{"keyframes": [[difficulty, value], ...]}`. JSON
has no `NaN` and no infinity, so those appear as the strings `"NaN"`, `"Infinity"` and
`"-Infinity"`; writing them as `null` would claim something else entirely.

The TypeScript types describe these files as well as the data compiled in beside them — same field
names, same curve shape, `null` for an absent value in both. If you fetch the JSON rather than
installing the package, you can still type it:

```ts
import type { Hero } from "fellowship-data";

const heroes: readonly Hero[] = (await (await fetch(".../heroes.json")).json()).records;
```

Field names are snake case everywhere, including TypeScript, for exactly that reason.

## Browser use

The Rust crate builds for `wasm32-unknown-unknown` and every release is checked against that
target. It deliberately ships no `wasm-bindgen` bindings and no `cdylib` — if you're targeting the
browser you'll want to expose your own shape to JavaScript, and a binding baked in here would just
get in the way. The TypeScript package is the more direct route.

## Versions and freshness

`BUILD_ID` tells you which game build the data came from, and every delivery states the same one.
All four share a version, which moves when the game does, so pin it if you need a stable dataset.

## A note on the generated files

`rust/src/generated/`, `python/fellowship_data/generated/`, `typescript/src/generated/` and `json/`
are generated and replaced wholesale whenever the data is refreshed, so a local fix disappears
without warning — and fixing one delivery alone would leave them disagreeing, which a refresh
refuses to publish. If a value looks wrong, report it rather than patching it here.

The vocabulary each delivery is expressed in — `rust/src/types.rs`,
`python/fellowship_data/types.py`, `typescript/src/types.ts` — is hand-written.

## Legal

This data is derived from a licensed copy of the game for interoperability with community tooling.
The repository contains no game assets, art, audio, or code — only facts about how the game is
configured. Fellowship is the property of its developers and publisher; this project is unaffiliated
with them.

Licensed MIT, which covers this project's own source. It says nothing about the underlying game
content, which isn't the project's to license.
