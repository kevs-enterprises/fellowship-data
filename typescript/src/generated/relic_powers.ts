// GENERATED FILE - do not edit by hand.
//
// Rendered from the published `relic_powers.json`. Edits are overwritten on the next refresh.

import type { MediaHandle, MediaKind, Provenance, RelicPowerId } from "../types.js";

/** A relic power: an activated effect carried by a relic. */
export interface RelicPower {
  readonly id: RelicPowerId;
  readonly name: string;
  readonly description: string;
  /**
   * The ability this relic power activates.
   *
   * Names a relic ability, which `abilities.json` does not carry. Recorded as a scope gap.
   */
  readonly gameplay_ability_asset: string;
  readonly icon: MediaHandle | null;
  readonly provenance: Provenance;
}

/** Every relic power, sorted by `id`. */
export const RELIC_POWERS: readonly RelicPower[] = [
  { id: "GA_RelicPower_CastedSingleTargetRevive" as RelicPowerId, name: "Revive", description: "<rt.absorb>Revive</> target dead ally.\n\n<rt.meikoability2>Can be used in combat</>", gameplay_ability_asset: "GA_RelicPower_CastedSingleTargetRevive_C", icon: null, provenance: { origin: "datamine", dev_name: null, source: null } },
  { id: "GA_RelicPower_DamagePotion" as RelicPowerId, name: "(CUT) Elemental Empowerment", description: "+20 Bonus Main Stat. This is increased by 2.75 for every 15 item levels your item has.\n\nMain stats are Strength, Agility and Intellect.", gameplay_ability_asset: "GA_RelicPower_DamagePotion_C", icon: null, provenance: { origin: "datamine", dev_name: null, source: null } },
  { id: "GA_RelicPower_InstantSingleInterrupt.GA_RelicPower_InstantSingleInterrupt" as RelicPowerId, name: "Interject Spell", description: "<rt.absorb>Interrupt</> the target's spellcasting and prevent it from casting for 4 sec. \nMust interrupt a spell, or it will have no effect.\n\n<rt.meikoability2>Can be used during Global Cooldown</>", gameplay_ability_asset: "GA_RelicPower_InstantSingleInterrupt_C", icon: null, provenance: { origin: "datamine", dev_name: null, source: null } },
  { id: "GA_RelicPower_ManaPotion" as RelicPowerId, name: "Restore Mana", description: "Instantly restore <rt.mana>30% of your maximum mana</>.\n\n<rt.meikoability2>Can be used during Global Cooldown</>", gameplay_ability_asset: "GA_RelicPower_ManaPotion_C", icon: null, provenance: { origin: "datamine", dev_name: null, source: null } },
  { id: "GA_RelicPower_MassDispel" as RelicPowerId, name: "Major Dispel", description: "Instantly <rt.heal>Dispel all harmful magic effects</> from you and your allies in a large radius around you.\n\n<rt.meikoability2>Can be used during Global Cooldown</>", gameplay_ability_asset: "GA_RelicPower_MassDispel_C", icon: null, provenance: { origin: "datamine", dev_name: null, source: null } },
  { id: "GA_RelicPower_PartyDamageReductionBuff" as RelicPowerId, name: "Sanctuary", description: "Grants <rt.heal>15% Damage Reduction</> to yourself and nearby allies for 15 seconds.\n\n<rt.meikoability2>Can be used during Global Cooldown</>", gameplay_ability_asset: "GA_RelicPower_PartyDamageReductionBuff_C", icon: null, provenance: { origin: "datamine", dev_name: null, source: null } },
  { id: "GA_RelicPower_PartyInvisibility" as RelicPowerId, name: "Major Invisibility", description: "Instantly conceal yourself all nearby allies, granting <rt.effect>Major Invisiblilty</> for 15 seconds.\n\n<rt.meikoability2>Can only be used out of combat.</>", gameplay_ability_asset: "GA_RelicPower_PartyInvisibility_C", icon: null, provenance: { origin: "datamine", dev_name: null, source: null } },
  { id: "GA_RelicPower_PartyMoveSpeedBuff" as RelicPowerId, name: "Bloodrite Fervor", description: "You and nearby allies gain <rt.absorb>50% increased Movement Speed</> for 12 seconds.\n\n<rt.meikoability2>Can be used during Global Cooldown</>", gameplay_ability_asset: "GA_RelicPower_PartyMoveSpeedBuff_C", icon: null, provenance: { origin: "datamine", dev_name: null, source: null } },
  { id: "GA_RelicPower_PersonalDamageReductionBuff.GA_RelicPower_PersonalDamageReductionBuff" as RelicPowerId, name: "Obsidian Skin", description: "Grants <rt.heal>30% Damage Reduction</> to yourself for 9 seconds.\n\n<rt.meikoability2>Can be used during Global Cooldown</>", gameplay_ability_asset: "GA_RelicPower_PersonalDamageReductionBuff_C", icon: null, provenance: { origin: "datamine", dev_name: null, source: null } },
  { id: "GA_RelicPower_PolymorphTargetWithSharedCharges" as RelicPowerId, name: "Chickenize!", description: "Instantly turn your target into a chicken.\n\n<rt.absorb>Chickenize!</> can only be used a limited amount of times per dungeon. The charges are shared by everyone in the party.\n\n<rt.effect>Chickenized</> enemies <rt.warning>do not give Kill Score.</>\n\n<rt.meikoability2>Can be used during Global Cooldown</>", gameplay_ability_asset: "GA_RelicPower_PolymorphTargetWithSharedCharges_C", icon: null, provenance: { origin: "datamine", dev_name: null, source: null } },
  { id: "GA_RelicPower_Portal" as RelicPowerId, name: "Conjure Portal", description: "Conjures two interactable <rt.absorb>Portals</> that you and your allies can use to safely travel between two points.\n\n<rt.meikoability2>Can only have 1 active portal at a time.</>", gameplay_ability_asset: "GA_RelicPower_Portal_C", icon: null, provenance: { origin: "datamine", dev_name: null, source: null } },
  { id: "GA_RelicPower_Portal_Interact" as RelicPowerId, name: "Relic Teleport", description: "", gameplay_ability_asset: "GA_RelicPower_Portal_Interact_C", icon: null, provenance: { origin: "datamine", dev_name: null, source: null } },
  { id: "GA_Relic_Potion_Health" as RelicPowerId, name: "Rejuvenate", description: "Instantly replenish <rt.heal>40% of your maximum health</>.\n\n<rt.meikoability2>Can be used during Global Cooldown</>", gameplay_ability_asset: "GA_RelicPower_HealthPotion_C", icon: null, provenance: { origin: "datamine", dev_name: null, source: null } },
];
