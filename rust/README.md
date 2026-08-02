# fellowship-data

Fellowship game data — heroes, abilities, items, talents, enemies, dungeons — as a Rust library.

Everything is compiled in as `&'static` data. There is no file to load, no JSON to parse, no async
setup, and no way for the data to be missing at runtime.

```rust
use fellowship_data::generated::heroes::HEROES;

let elarion = HEROES.iter().find(|hero| hero.id.as_str() == "Bowguy").unwrap();
println!("{} — {}", elarion.name, elarion.title);   // Elarion — The Skystrider
```

## Adding it

```toml
[dependencies]
fellowship-data = "0.3"
```

No dependencies of its own.

## Pick what you need

Each domain is a feature, so you only compile the data you actually use. Enemy difficulty scaling
is 440 curves of 151 points each — real weight if you're building a gear planner that never looks
at a mob — so `mobs` is off unless you ask for it:

```toml
fellowship-data = { version = "0.3", default-features = false, features = ["heroes", "items"] }
```

Available: `heroes` · `abilities` · `talents` · `items` · `attributes` · `constants` · `effects` ·
`modifiers` · `dungeons` · `mobs`. The default set is `heroes`, `abilities`, `items`, `attributes`,
`constants`.

## Browser use

The crate builds for `wasm32-unknown-unknown` and every release is checked against that target. It
deliberately ships no `wasm-bindgen` bindings and no `cdylib` — if you're targeting the browser
you'll want to expose your own shape to JavaScript, and a binding baked in here would just get in
the way.

## Missing values, curves, and media

`Value<T>`, `Curve`, and `MediaHandle` all carry the same rule across every language this dataset is
published in: a gap is a value, never a silent default, and a curve never interpolates between the
points the data actually states. See the [main README](https://github.com/kevs-enterprises/fellowship-data#readme)
for the full explanation and how it reads in each of the four deliveries.

## Versions and freshness

`BUILD_ID` tells you which game build the data came from, and every delivery in this repository
states the same one. Pin a version if you need a stable dataset.

## This is one of four deliveries

The same records are also published as a Python package, a TypeScript package, and plain JSON, all
rendered from one description of each domain so they cannot disagree. See the
[repository](https://github.com/kevs-enterprises/fellowship-data) for the others and for what isn't
specific to Rust: images, legal, and the full set of conventions.

## Legal

This data is derived from a licensed copy of the game for interoperability with community tooling.
It contains no game assets, art, audio, or code — only facts about how the game is configured.
Fellowship is the property of its developers and publisher; this project is unaffiliated with them.

Licensed MIT.
