<!-- SOURCED FILE — do not edit here. Edit automation/publish/ in the generator; this copy is overwritten on the next sync. -->

# fellowship-data

Fellowship game data — heroes, abilities, items, talents, enemies, dungeons — as a Python package.

Everything is written into the modules themselves as frozen dataclasses and tuples. There is no
file to load, no JSON to parse, no async setup, and no state where the data is half-loaded.

```python
from fellowship_data.generated.heroes import HEROES

elarion = next(hero for hero in HEROES if hero.id == "Bowguy")
print(f"{elarion.name} — {elarion.title}")   # Elarion — The Skystrider
```

## Installing it

```
pip install fellowship-data
```

No dependencies of its own.

## Pick what you need

Each domain is its own module, so nothing loads a domain you did not ask for. Enemy difficulty
scaling is 440 curves of 151 points each — real weight if you're building a gear planner that never
looks at a mob:

```python
from fellowship_data.generated import heroes, items
```

## Missing values, curves, and media

`Value`, `Curve`, and `MediaHandle` all carry the same rule across every language this dataset is
published in: a gap is a value, never a silent default, and a curve never interpolates between the
points the data actually states. See the [main README](https://github.com/kevs-enterprises/fellowship-data#readme)
for the full explanation and how it reads in each of the four deliveries.

## Versions and freshness

`fellowship_data.BUILD_ID` tells you which game build the data came from, and every delivery in
this repository states the same one. Pin a version if you need a stable dataset.

## This is one of four deliveries

The same records are also published as a Rust crate, a TypeScript package, and plain JSON, all
rendered from one description of each domain so they cannot disagree. See the
[repository](https://github.com/kevs-enterprises/fellowship-data) for the others and for what isn't
specific to Python: images, legal, and the full set of conventions.

## Legal

This data is derived from a licensed copy of the game for interoperability with community tooling.
It contains no game assets, art, audio, or code — only facts about how the game is configured.
Fellowship is the property of its developers and publisher; this project is unaffiliated with them.

Licensed MIT.
