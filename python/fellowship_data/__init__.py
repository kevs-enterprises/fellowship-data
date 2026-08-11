# SOURCED FILE — do not edit here. Edit automation/publish-data/ in the generator; this copy is overwritten on the next sync.

"""Fellowship game data, as a plain Python package.

Everything under :mod:`fellowship_data.generated` is produced by a tool and overwritten on each
refresh. :mod:`fellowship_data.types` is hand-written, and is the vocabulary the generated data is
expressed in.

Shape
-----

There is no runtime loading. The values are written into the modules themselves, so there is
nothing to parse, nothing to await, and no failure mode where the data is missing::

    from fellowship_data.generated.heroes import HEROES

    elarion = next(hero for hero in HEROES if hero.id == "Bowguy")
    print(f"{elarion.name} - {elarion.title}")   # Elarion - The Skystrider

Import only the domains you need. The enemy difficulty curves are large, and nothing here loads
them unless you ask::

    from fellowship_data.generated import mobs

Everything is a frozen dataclass and every collection is a tuple. This data is a constant, and a
consumer that mutated it would be mutating it for every other consumer in the process.

What this package deliberately is not
-------------------------------------

It contains no images. Records reference art through :class:`~fellowship_data.types.MediaHandle`,
which carries a stable id and the source dimensions but no path — map the id onto wherever you
serve art.

Gaps are values, not silence
----------------------------

Where a number could not be resolved, the record says so rather than substituting a default. An
optional field is ``None`` because the game gives no value, never because the value was lost.
"""

from __future__ import annotations

from .generated import ABILITY_GUID_SCHEME, BUILD_ID
from .types import (
    AbilityGuid,
    AbilityGuidScheme,
    AbilityId,
    AbilityRef,
    AttributeId,
    Curve,
    CurveValue,
    Dense,
    DungeonId,
    EffectId,
    HeroId,
    ItemId,
    Keyframes,
    Known,
    MediaHandle,
    MediaKind,
    MobId,
    Origin,
    Provenance,
    TalentId,
    Unresolved,
    Value,
    known,
    known_or_declared_default,
)

__all__ = [
    "ABILITY_GUID_SCHEME",
    "BUILD_ID",
    "AbilityGuid",
    "AbilityGuidScheme",
    "AbilityId",
    "AbilityRef",
    "AttributeId",
    "Curve",
    "CurveValue",
    "Dense",
    "DungeonId",
    "EffectId",
    "HeroId",
    "ItemId",
    "Keyframes",
    "Known",
    "MediaHandle",
    "MediaKind",
    "MobId",
    "Origin",
    "Provenance",
    "TalentId",
    "Unresolved",
    "Value",
    # The two accessors, beside the type they read. `Value` is on every effect modifier's
    # magnitude, so reaching into `fellowship_data.types` for the only supported way to read one
    # would make the awkward path the common one — and the convenient alternative is
    # `m.value.value`, which reads a `Known` and raises on the other two arms.
    "known",
    "known_or_declared_default",
]
