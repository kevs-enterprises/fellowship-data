//! GENERATED FILE — do not edit by hand.
//!
//! Generated from `relic_powers.json`. Rows are stable-sorted so a data refresh diffs one entity
//! at a time. Anything changed here is overwritten on the next refresh.

#![cfg_attr(rustfmt, rustfmt::skip)]
// `unreadable_literal` asks for digit separators to help a human read a number. These are
// machine-emitted measurements — world coordinates, tick periods, sixty-odd thousand
// curve points — and nobody reads them one at a time. Grouping their digits would change
// every value on every refresh for no reader's benefit.
#![allow(clippy::unreadable_literal)]

use crate::types::{MediaHandle, MediaKind, Provenance, RelicPowerId};

/// A relic power: an activated effect carried by a relic.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RelicPower {
    pub id: RelicPowerId,
    pub name: &'static str,
    pub description: &'static str,
    /// The ability this relic power activates.
    ///
    /// Names a relic ability, which `abilities.json` does not carry. Recorded as a scope gap.
    pub gameplay_ability_asset: &'static str,
    pub icon: Option<MediaHandle>,
    pub provenance: Provenance,
}

/// Every relic power, sorted by `id`.
pub static RELIC_POWERS: &[RelicPower] = &[
    RelicPower { id: RelicPowerId("GA_RelicPower_CastedSingleTargetRevive"), name: "Revive", description: "<rt.absorb>Revive</> target dead ally.\n\n<rt.meikoability2>Can be used in combat</>", gameplay_ability_asset: "GA_RelicPower_CastedSingleTargetRevive_C", icon: None, provenance: Provenance::DATAMINE },
    RelicPower { id: RelicPowerId("GA_RelicPower_DamagePotion"), name: "(CUT) Elemental Empowerment", description: "+20 Bonus Main Stat. This is increased by 2.75 for every 15 item levels your item has.\n\nMain stats are Strength, Agility and Intellect.", gameplay_ability_asset: "GA_RelicPower_DamagePotion_C", icon: None, provenance: Provenance::DATAMINE },
    RelicPower { id: RelicPowerId("GA_RelicPower_InstantSingleInterrupt.GA_RelicPower_InstantSingleInterrupt"), name: "Interject Spell", description: "<rt.absorb>Interrupt</> the target's spellcasting and prevent it from casting for 4 sec. \nMust interrupt a spell, or it will have no effect.\n\n<rt.meikoability2>Can be used during Global Cooldown</>", gameplay_ability_asset: "GA_RelicPower_InstantSingleInterrupt_C", icon: None, provenance: Provenance::DATAMINE },
    RelicPower { id: RelicPowerId("GA_RelicPower_ManaPotion"), name: "Restore Mana", description: "Instantly restore <rt.mana>30% of your maximum mana</>.\n\n<rt.meikoability2>Can be used during Global Cooldown</>", gameplay_ability_asset: "GA_RelicPower_ManaPotion_C", icon: None, provenance: Provenance::DATAMINE },
    RelicPower { id: RelicPowerId("GA_RelicPower_MassDispel"), name: "Major Dispel", description: "Instantly <rt.heal>Dispel all harmful magic effects</> from you and your allies in a large radius around you.\n\n<rt.meikoability2>Can be used during Global Cooldown</>", gameplay_ability_asset: "GA_RelicPower_MassDispel_C", icon: None, provenance: Provenance::DATAMINE },
    RelicPower { id: RelicPowerId("GA_RelicPower_PartyDamageReductionBuff"), name: "Sanctuary", description: "Grants <rt.heal>15% Damage Reduction</> to yourself and nearby allies for 15 seconds.\n\n<rt.meikoability2>Can be used during Global Cooldown</>", gameplay_ability_asset: "GA_RelicPower_PartyDamageReductionBuff_C", icon: None, provenance: Provenance::DATAMINE },
    RelicPower { id: RelicPowerId("GA_RelicPower_PartyInvisibility"), name: "Major Invisibility", description: "Instantly conceal yourself all nearby allies, granting <rt.effect>Major Invisiblilty</> for 15 seconds.\n\n<rt.meikoability2>Can only be used out of combat.</>", gameplay_ability_asset: "GA_RelicPower_PartyInvisibility_C", icon: None, provenance: Provenance::DATAMINE },
    RelicPower { id: RelicPowerId("GA_RelicPower_PartyMoveSpeedBuff"), name: "Bloodrite Fervor", description: "You and nearby allies gain <rt.absorb>50% increased Movement Speed</> for 12 seconds.\n\n<rt.meikoability2>Can be used during Global Cooldown</>", gameplay_ability_asset: "GA_RelicPower_PartyMoveSpeedBuff_C", icon: None, provenance: Provenance::DATAMINE },
    RelicPower { id: RelicPowerId("GA_RelicPower_PersonalDamageReductionBuff.GA_RelicPower_PersonalDamageReductionBuff"), name: "Obsidian Skin", description: "Grants <rt.heal>30% Damage Reduction</> to yourself for 9 seconds.\n\n<rt.meikoability2>Can be used during Global Cooldown</>", gameplay_ability_asset: "GA_RelicPower_PersonalDamageReductionBuff_C", icon: None, provenance: Provenance::DATAMINE },
    RelicPower { id: RelicPowerId("GA_RelicPower_PolymorphTargetWithSharedCharges"), name: "Chickenize!", description: "Instantly turn your target into a chicken.\n\n<rt.absorb>Chickenize!</> can only be used a limited amount of times per dungeon. The charges are shared by everyone in the party.\n\n<rt.effect>Chickenized</> enemies <rt.warning>do not give Kill Score.</>\n\n<rt.meikoability2>Can be used during Global Cooldown</>", gameplay_ability_asset: "GA_RelicPower_PolymorphTargetWithSharedCharges_C", icon: None, provenance: Provenance::DATAMINE },
    RelicPower { id: RelicPowerId("GA_RelicPower_Portal"), name: "Conjure Portal", description: "Conjures two interactable <rt.absorb>Portals</> that you and your allies can use to safely travel between two points.\n\n<rt.meikoability2>Can only have 1 active portal at a time.</>", gameplay_ability_asset: "GA_RelicPower_Portal_C", icon: None, provenance: Provenance::DATAMINE },
    RelicPower { id: RelicPowerId("GA_RelicPower_Portal_Interact"), name: "Relic Teleport", description: "", gameplay_ability_asset: "GA_RelicPower_Portal_Interact_C", icon: None, provenance: Provenance::DATAMINE },
    RelicPower { id: RelicPowerId("GA_Relic_Potion_Health"), name: "Rejuvenate", description: "Instantly replenish <rt.heal>40% of your maximum health</>.\n\n<rt.meikoability2>Can be used during Global Cooldown</>", gameplay_ability_asset: "GA_RelicPower_HealthPotion_C", icon: None, provenance: Provenance::DATAMINE },
];
