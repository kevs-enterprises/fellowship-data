# fellowship-data

Fellowship game data — heroes, abilities, items, talents, enemies, dungeons — as a Rust library.

Everything is compiled in as `&'static` data. There is no file to load, no JSON to parse, no async
setup, and no way for the data to be missing at runtime. Add the crate, read the statics.

```rust
use fellowship_data::generated::heroes::HEROES;

let elarion = HEROES.iter().find(|hero| hero.id.as_str() == "Bowguy").unwrap();
println!("{} — {}", elarion.name, elarion.title);   // Elarion — The Skystrider
```

## Adding it

```toml
[dependencies]
fellowship-data = { git = "https://github.com/kevs-enterprises/fellowship-data" }
```

The crate has no dependencies of its own.

### Pick what you need

Each domain is a feature, so you only compile the data you actually use. Enemy difficulty scaling
is 440 curves of 151 points each — real weight if you're building a gear planner that never looks
at a mob — so `mobs` is off unless you ask for it.

```toml
fellowship-data = { git = "...", default-features = false, features = ["heroes", "items"] }
```

Available: `heroes` · `abilities` · `talents` · `items` · `attributes` · `constants` · `effects` ·
`modifiers` · `dungeons` · `mobs`

The default set is `heroes`, `abilities`, `items`, `attributes`, `constants`.

## Missing numbers stay missing

Some values in the game are defined by curves or lookup tables that this data doesn't resolve. Where
that happens the crate says so, instead of quietly filling in a plausible number:

```rust
match ability.max_range {
    Value::Known(range)                  => // measured from the game's own data
    Value::Curve { curve_ref, default }  => // defined by a curve we haven't resolved
    Value::Unresolved { reason }         => // known to exist, no value recovered
}
```

`known()` gives you `Some` only for a real measurement. There's also
`known_or_declared_default()`, named the long way on purpose: a declared default is what the asset
falls back to, not what the game was observed to do. If you use it, you should know you did.

Curves work the same way. `Curve::Dense` has one value per difficulty. `Curve::Keyframes` has only
the points the data actually states — heroes typically carry two, at difficulty 1 and 151 — and
`at()` returns `None` in between rather than interpolating a number nobody measured. If your model
wants a value there, interpolate deliberately, with your own assumptions visible.

This is the crate's one opinion: it would rather be awkward than confidently wrong.

## Images

There aren't any. Portraits and icons are referenced by `MediaHandle`, which gives you a stable id
and the source dimensions but no path or URL — you map the id onto wherever you serve art from.

```rust
if let Some(portrait) = hero.portrait {
    let url = format!("/game-media/heroes/{}.webp", portrait.id);
}
```

Ids stay stable across data refreshes, so your asset paths keep working.

## Browser use

The crate builds for `wasm32-unknown-unknown` and every release is checked against that target. It
deliberately ships no `wasm-bindgen` bindings and no `cdylib` — if you're targeting the browser
you'll want to expose your own shape to JavaScript, and a binding baked in here would just get in
the way.

## If you aren't writing Rust

The crate is the delivery, and everything above describes the shape this data is meant to be used
in. The same records are also written out as JSON under [`json/`](json/), for consumers that can't
link a Rust library — a script, a spreadsheet, a service in another language.

It's a mirror, not a second dataset. Every record there is the record the crate publishes, with the
same fields, the same stable order, and the same refusals to guess. One file per domain, plus an
`index.json` saying what the folder holds.

```json
{"id":"Bowguy","name":"Elarion","title":"The Skystrider","base_health":1687.0,"...":"..."}
```

Four conventions are worth knowing before you read it:

- **`null` means the record does not carry that value.** It is never a stand-in for zero, for an
  empty string, or for a number that exists upstream and wasn't recovered.
- **Curves keep both shapes.** `{"dense": [...]}` is one value per difficulty from 1;
  `{"keyframes": [[difficulty, value], ...]}` states only the points the data gives, and nothing
  between them is interpolated for you — for the reason in "Missing numbers stay missing" above.
- **JSON has no `NaN` and no infinity**, so those appear as the strings `"NaN"`, `"Infinity"`, and
  `"-Infinity"`. Writing them as `null` would claim something else entirely.
- **One record per line.** These files are large, and this keeps a data refresh diffing the
  entities that changed instead of reflowing the whole file.

Media works the same way it does in the crate: a handle carries a stable id and the source
dimensions, never a path.

## Versions and freshness

`BUILD_ID` tells you which game build the data came from. The version moves when the game does, so
pin a tag if you need a stable dataset:

```toml
fellowship-data = { git = "...", tag = "v0.3.0" }
```

## A note on the generated files

`rust/src/generated/` and `json/` are generated and replaced wholesale whenever the data is refreshed, so a local fix to either
disappears without warning — and fixing one alone would leave the two disagreeing, which a refresh
refuses to publish. If a value looks wrong, report it rather than patching it here.
`rust/src/types.rs` is hand-written — it's the vocabulary the generated data is expressed in.

## Legal

This data is derived from a licensed copy of the game for interoperability with community tooling.
The repository contains no game assets, art, audio, or code — only facts about how the game is
configured. Fellowship is the property of its developers and publisher; this project is unaffiliated
with them.

Licensed MIT, which covers this crate's own source. It says nothing about the underlying game
content, which isn't the project's to license.
