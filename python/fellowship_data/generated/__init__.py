"""GENERATED FILE - do not edit by hand.

Each domain is its own module, imported on demand: nothing here loads a domain you did not ask
for.

Available modules:

- ``fellowship_data.generated.abilities``
- ``fellowship_data.generated.ability_constants``
- ``fellowship_data.generated.attributes``
- ``fellowship_data.generated.combat_constants``
- ``fellowship_data.generated.constants``
- ``fellowship_data.generated.dungeons``
- ``fellowship_data.generated.effect_executions``
- ``fellowship_data.generated.effects``
- ``fellowship_data.generated.finesses``
- ``fellowship_data.generated.heroes``
- ``fellowship_data.generated.items``
- ``fellowship_data.generated.mobs``
- ``fellowship_data.generated.modifiers``
- ``fellowship_data.generated.relic_powers``
- ``fellowship_data.generated.set_bonuses``
- ``fellowship_data.generated.tag_ids``
- ``fellowship_data.generated.talents``
- ``fellowship_data.generated.traits``
"""

from ..types import AbilityGuidScheme, Origin

#: The game build this data was extracted from.
BUILD_ID = "24133959"

#: The protocol used to derive every published Ability GUID.
ABILITY_GUID_SCHEME = AbilityGuidScheme(
    name="ability-guid-v1",
    namespace="c429f5ee-71e6-4a70-9e5b-4c63ee73e575",
    version=5,
    transform="strip-terminal-_C;resolve-GameplayAbility;ascii-lowercase;prefix=ability/",
    origin=Origin.DERIVED,
)

__all__ = ["BUILD_ID", "ABILITY_GUID_SCHEME"]

#: Every generated domain module, by name.
DOMAINS = ("abilities", "ability_constants", "attributes", "combat_constants", "constants", "dungeons", "effect_executions", "effects", "finesses", "heroes", "items", "mobs", "modifiers", "relic_powers", "set_bonuses", "tag_ids", "talents", "traits",)
