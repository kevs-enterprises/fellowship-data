"""GENERATED FILE - do not edit by hand.

Rendered from the published `relic_powers.json`. Edits are overwritten on the next refresh.
"""

from __future__ import annotations

from dataclasses import dataclass
from uuid import UUID

from ..types import AbilityGuid, AbilityId, AbilityRef, MediaHandle, MediaKind, Provenance, RelicPowerId, SourceAuthority


#: Weakest acquisition authority across every immutable input behind this module.
SOURCE_AUTHORITY: SourceAuthority = SourceAuthority.LEGACY_UNBOUND

@dataclass(frozen=True)
class RelicPower:
    """A relic power: an activated effect carried by a relic."""

    id: RelicPowerId
    name: str
    description: str
    gameplay_ability_asset: str
    #: The ability this relic power activates.
    #:
    #: Names a relic ability, which `abilities.json` does not carry. Recorded as a scope gap.
    gameplay_ability_ref: AbilityRef
    #: The stable identity paired with `gameplay_ability_asset`.
    icon: MediaHandle | None
    provenance: Provenance


#: Every relic power, sorted by `id`.
RELIC_POWERS: tuple[RelicPower, ...] = (
    RelicPower(id=RelicPowerId("GA_RelicPower_CastedSingleTargetRevive"), name="Revive", description="<rt.absorb>Revive</> target dead ally.\n\n<rt.meikoability2>Can be used in combat</>", gameplay_ability_asset="GA_RelicPower_CastedSingleTargetRevive_C", gameplay_ability_ref=AbilityRef(source_id=AbilityId("GA_RelicPower_CastedSingleTargetRevive_C"), guid=AbilityGuid(UUID("b88fe676-6db9-55cb-b0ec-7db28d8ed684"))), icon=None, provenance=Provenance.datamine(SourceAuthority.LEGACY_UNBOUND)),
    RelicPower(id=RelicPowerId("GA_RelicPower_DamagePotion"), name="(CUT) Elemental Empowerment", description="+20 Bonus Main Stat. This is increased by 2.75 for every 15 item levels your item has.\n\nMain stats are Strength, Agility and Intellect.", gameplay_ability_asset="GA_RelicPower_DamagePotion_C", gameplay_ability_ref=AbilityRef(source_id=AbilityId("GA_RelicPower_DamagePotion_C"), guid=AbilityGuid(UUID("c17936fd-d7c4-5022-a174-b15c3a534db8"))), icon=None, provenance=Provenance.datamine(SourceAuthority.LEGACY_UNBOUND)),
    RelicPower(id=RelicPowerId("GA_RelicPower_InstantSingleInterrupt.GA_RelicPower_InstantSingleInterrupt"), name="Interject Spell", description="<rt.absorb>Interrupt</> the target's spellcasting and prevent it from casting for 4 sec. \nMust interrupt a spell, or it will have no effect.\n\n<rt.meikoability2>Can be used during Global Cooldown</>", gameplay_ability_asset="GA_RelicPower_InstantSingleInterrupt_C", gameplay_ability_ref=AbilityRef(source_id=AbilityId("GA_RelicPower_InstantSingleInterrupt_C"), guid=AbilityGuid(UUID("6c907645-db76-5dd7-bf0a-1dc10eff78f2"))), icon=None, provenance=Provenance.datamine(SourceAuthority.LEGACY_UNBOUND)),
    RelicPower(id=RelicPowerId("GA_RelicPower_ManaPotion"), name="Restore Mana", description="Instantly restore <rt.mana>30% of your maximum mana</>.\n\n<rt.meikoability2>Can be used during Global Cooldown</>", gameplay_ability_asset="GA_RelicPower_ManaPotion_C", gameplay_ability_ref=AbilityRef(source_id=AbilityId("GA_RelicPower_ManaPotion_C"), guid=AbilityGuid(UUID("8877996a-eedd-5bf6-824e-260380a53182"))), icon=None, provenance=Provenance.datamine(SourceAuthority.LEGACY_UNBOUND)),
    RelicPower(id=RelicPowerId("GA_RelicPower_MassDispel"), name="Major Dispel", description="Instantly <rt.heal>Dispel all harmful magic effects</> from you and your allies in a large radius around you.\n\n<rt.meikoability2>Can be used during Global Cooldown</>", gameplay_ability_asset="GA_RelicPower_MassDispel_C", gameplay_ability_ref=AbilityRef(source_id=AbilityId("GA_RelicPower_MassDispel_C"), guid=AbilityGuid(UUID("b6c76e62-9232-5583-8cb2-dd45bec0c0ff"))), icon=None, provenance=Provenance.datamine(SourceAuthority.LEGACY_UNBOUND)),
    RelicPower(id=RelicPowerId("GA_RelicPower_PartyDamageReductionBuff"), name="Sanctuary", description="Grants <rt.heal>15% Damage Reduction</> to yourself and nearby allies for 15 seconds.\n\n<rt.meikoability2>Can be used during Global Cooldown</>", gameplay_ability_asset="GA_RelicPower_PartyDamageReductionBuff_C", gameplay_ability_ref=AbilityRef(source_id=AbilityId("GA_RelicPower_PartyDamageReductionBuff_C"), guid=AbilityGuid(UUID("80549f12-bcee-55d6-8a05-4f3fa06f8ea9"))), icon=None, provenance=Provenance.datamine(SourceAuthority.LEGACY_UNBOUND)),
    RelicPower(id=RelicPowerId("GA_RelicPower_PartyInvisibility"), name="Major Invisibility", description="Instantly conceal yourself all nearby allies, granting <rt.effect>Major Invisiblilty</> for 15 seconds.\n\n<rt.meikoability2>Can only be used out of combat.</>", gameplay_ability_asset="GA_RelicPower_PartyInvisibility_C", gameplay_ability_ref=AbilityRef(source_id=AbilityId("GA_RelicPower_PartyInvisibility_C"), guid=AbilityGuid(UUID("2ed8bfea-7333-568f-8ddd-a03e36b74a5d"))), icon=None, provenance=Provenance.datamine(SourceAuthority.LEGACY_UNBOUND)),
    RelicPower(id=RelicPowerId("GA_RelicPower_PartyMoveSpeedBuff"), name="Bloodrite Fervor", description="You and nearby allies gain <rt.absorb>50% increased Movement Speed</> for 12 seconds.\n\n<rt.meikoability2>Can be used during Global Cooldown</>", gameplay_ability_asset="GA_RelicPower_PartyMoveSpeedBuff_C", gameplay_ability_ref=AbilityRef(source_id=AbilityId("GA_RelicPower_PartyMoveSpeedBuff_C"), guid=AbilityGuid(UUID("a647cb4a-9c17-53b3-bbbf-c8d7e6d63cce"))), icon=None, provenance=Provenance.datamine(SourceAuthority.LEGACY_UNBOUND)),
    RelicPower(id=RelicPowerId("GA_RelicPower_PersonalDamageReductionBuff.GA_RelicPower_PersonalDamageReductionBuff"), name="Obsidian Skin", description="Grants <rt.heal>30% Damage Reduction</> to yourself for 9 seconds.\n\n<rt.meikoability2>Can be used during Global Cooldown</>", gameplay_ability_asset="GA_RelicPower_PersonalDamageReductionBuff_C", gameplay_ability_ref=AbilityRef(source_id=AbilityId("GA_RelicPower_PersonalDamageReductionBuff_C"), guid=AbilityGuid(UUID("2c869b27-5eff-52f0-aff9-210d993aed7a"))), icon=None, provenance=Provenance.datamine(SourceAuthority.LEGACY_UNBOUND)),
    RelicPower(id=RelicPowerId("GA_RelicPower_PolymorphTargetWithSharedCharges"), name="Chickenize!", description="Instantly turn your target into a chicken.\n\n<rt.absorb>Chickenize!</> can only be used a limited amount of times per dungeon. The charges are shared by everyone in the party.\n\n<rt.effect>Chickenized</> enemies <rt.warning>do not give Kill Score.</>\n\n<rt.meikoability2>Can be used during Global Cooldown</>", gameplay_ability_asset="GA_RelicPower_PolymorphTargetWithSharedCharges_C", gameplay_ability_ref=AbilityRef(source_id=AbilityId("GA_RelicPower_PolymorphTargetWithSharedCharges_C"), guid=AbilityGuid(UUID("a53ba442-3959-5a69-9764-09d9eefe081e"))), icon=None, provenance=Provenance.datamine(SourceAuthority.LEGACY_UNBOUND)),
    RelicPower(id=RelicPowerId("GA_RelicPower_Portal"), name="Conjure Portal", description="Conjures two interactable <rt.absorb>Portals</> that you and your allies can use to safely travel between two points.\n\n<rt.meikoability2>Can only have 1 active portal at a time.</>", gameplay_ability_asset="GA_RelicPower_Portal_C", gameplay_ability_ref=AbilityRef(source_id=AbilityId("GA_RelicPower_Portal_C"), guid=AbilityGuid(UUID("26b40291-5603-5494-8bd2-536a8a56a672"))), icon=None, provenance=Provenance.datamine(SourceAuthority.LEGACY_UNBOUND)),
    RelicPower(id=RelicPowerId("GA_RelicPower_Portal_Interact"), name="Relic Teleport", description="", gameplay_ability_asset="GA_RelicPower_Portal_Interact_C", gameplay_ability_ref=AbilityRef(source_id=AbilityId("GA_RelicPower_Portal_Interact_C"), guid=AbilityGuid(UUID("4e0da0cb-8b75-519d-a37d-248d9941b27f"))), icon=None, provenance=Provenance.datamine(SourceAuthority.LEGACY_UNBOUND)),
    RelicPower(id=RelicPowerId("GA_Relic_Potion_Health"), name="Rejuvenate", description="Instantly replenish <rt.heal>40% of your maximum health</>.\n\n<rt.meikoability2>Can be used during Global Cooldown</>", gameplay_ability_asset="GA_RelicPower_HealthPotion_C", gameplay_ability_ref=AbilityRef(source_id=AbilityId("GA_RelicPower_HealthPotion_C"), guid=AbilityGuid(UUID("0deecd6b-d39c-5d35-af73-a2dd59c90057"))), icon=None, provenance=Provenance.datamine(SourceAuthority.LEGACY_UNBOUND)),
)
