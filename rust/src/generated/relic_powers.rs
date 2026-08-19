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

use crate::types::{AbilityGuid, AbilityId, AbilityRef, MediaHandle, MediaKind, Provenance, RelicPowerId, SourceAuthority};

/// Weakest acquisition authority across every immutable input behind this module.
pub const SOURCE_AUTHORITY: SourceAuthority = SourceAuthority::LegacyUnbound;

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
    /// The stable identity paired with `gameplay_ability_asset`.
    pub gameplay_ability_ref: AbilityRef,
    pub icon: Option<MediaHandle>,
    pub provenance: Provenance,
}

/// Every relic power, sorted by `id`.
pub static RELIC_POWERS: &[RelicPower] = &[
    RelicPower { id: RelicPowerId("GA_RelicPower_CastedSingleTargetRevive"), name: "Revive", description: "<rt.absorb>Revive</> target dead ally.\n\n<rt.meikoability2>Can be used in combat</>", gameplay_ability_asset: "GA_RelicPower_CastedSingleTargetRevive_C", gameplay_ability_ref: AbilityRef { source_id: AbilityId("GA_RelicPower_CastedSingleTargetRevive_C"), guid: AbilityGuid::from_bytes([0xb8, 0x8f, 0xe6, 0x76, 0x6d, 0xb9, 0x55, 0xcb, 0xb0, 0xec, 0x7d, 0xb2, 0x8d, 0x8e, 0xd6, 0x84]) }, icon: None, provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    RelicPower { id: RelicPowerId("GA_RelicPower_DamagePotion"), name: "(CUT) Elemental Empowerment", description: "+20 Bonus Main Stat. This is increased by 2.75 for every 15 item levels your item has.\n\nMain stats are Strength, Agility and Intellect.", gameplay_ability_asset: "GA_RelicPower_DamagePotion_C", gameplay_ability_ref: AbilityRef { source_id: AbilityId("GA_RelicPower_DamagePotion_C"), guid: AbilityGuid::from_bytes([0xc1, 0x79, 0x36, 0xfd, 0xd7, 0xc4, 0x50, 0x22, 0xa1, 0x74, 0xb1, 0x5c, 0x3a, 0x53, 0x4d, 0xb8]) }, icon: None, provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    RelicPower { id: RelicPowerId("GA_RelicPower_InstantSingleInterrupt.GA_RelicPower_InstantSingleInterrupt"), name: "Interject Spell", description: "<rt.absorb>Interrupt</> the target's spellcasting and prevent it from casting for 4 sec. \nMust interrupt a spell, or it will have no effect.\n\n<rt.meikoability2>Can be used during Global Cooldown</>", gameplay_ability_asset: "GA_RelicPower_InstantSingleInterrupt_C", gameplay_ability_ref: AbilityRef { source_id: AbilityId("GA_RelicPower_InstantSingleInterrupt_C"), guid: AbilityGuid::from_bytes([0x6c, 0x90, 0x76, 0x45, 0xdb, 0x76, 0x5d, 0xd7, 0xbf, 0x0a, 0x1d, 0xc1, 0x0e, 0xff, 0x78, 0xf2]) }, icon: None, provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    RelicPower { id: RelicPowerId("GA_RelicPower_ManaPotion"), name: "Restore Mana", description: "Instantly restore <rt.mana>30% of your maximum mana</>.\n\n<rt.meikoability2>Can be used during Global Cooldown</>", gameplay_ability_asset: "GA_RelicPower_ManaPotion_C", gameplay_ability_ref: AbilityRef { source_id: AbilityId("GA_RelicPower_ManaPotion_C"), guid: AbilityGuid::from_bytes([0x88, 0x77, 0x99, 0x6a, 0xee, 0xdd, 0x5b, 0xf6, 0x82, 0x4e, 0x26, 0x03, 0x80, 0xa5, 0x31, 0x82]) }, icon: None, provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    RelicPower { id: RelicPowerId("GA_RelicPower_MassDispel"), name: "Major Dispel", description: "Instantly <rt.heal>Dispel all harmful magic effects</> from you and your allies in a large radius around you.\n\n<rt.meikoability2>Can be used during Global Cooldown</>", gameplay_ability_asset: "GA_RelicPower_MassDispel_C", gameplay_ability_ref: AbilityRef { source_id: AbilityId("GA_RelicPower_MassDispel_C"), guid: AbilityGuid::from_bytes([0xb6, 0xc7, 0x6e, 0x62, 0x92, 0x32, 0x55, 0x83, 0x8c, 0xb2, 0xdd, 0x45, 0xbe, 0xc0, 0xc0, 0xff]) }, icon: None, provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    RelicPower { id: RelicPowerId("GA_RelicPower_PartyDamageReductionBuff"), name: "Sanctuary", description: "Grants <rt.heal>15% Damage Reduction</> to yourself and nearby allies for 15 seconds.\n\n<rt.meikoability2>Can be used during Global Cooldown</>", gameplay_ability_asset: "GA_RelicPower_PartyDamageReductionBuff_C", gameplay_ability_ref: AbilityRef { source_id: AbilityId("GA_RelicPower_PartyDamageReductionBuff_C"), guid: AbilityGuid::from_bytes([0x80, 0x54, 0x9f, 0x12, 0xbc, 0xee, 0x55, 0xd6, 0x8a, 0x05, 0x4f, 0x3f, 0xa0, 0x6f, 0x8e, 0xa9]) }, icon: None, provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    RelicPower { id: RelicPowerId("GA_RelicPower_PartyInvisibility"), name: "Major Invisibility", description: "Instantly conceal yourself all nearby allies, granting <rt.effect>Major Invisiblilty</> for 15 seconds.\n\n<rt.meikoability2>Can only be used out of combat.</>", gameplay_ability_asset: "GA_RelicPower_PartyInvisibility_C", gameplay_ability_ref: AbilityRef { source_id: AbilityId("GA_RelicPower_PartyInvisibility_C"), guid: AbilityGuid::from_bytes([0x2e, 0xd8, 0xbf, 0xea, 0x73, 0x33, 0x56, 0x8f, 0x8d, 0xdd, 0xa0, 0x3e, 0x36, 0xb7, 0x4a, 0x5d]) }, icon: None, provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    RelicPower { id: RelicPowerId("GA_RelicPower_PartyMoveSpeedBuff"), name: "Bloodrite Fervor", description: "You and nearby allies gain <rt.absorb>50% increased Movement Speed</> for 12 seconds.\n\n<rt.meikoability2>Can be used during Global Cooldown</>", gameplay_ability_asset: "GA_RelicPower_PartyMoveSpeedBuff_C", gameplay_ability_ref: AbilityRef { source_id: AbilityId("GA_RelicPower_PartyMoveSpeedBuff_C"), guid: AbilityGuid::from_bytes([0xa6, 0x47, 0xcb, 0x4a, 0x9c, 0x17, 0x53, 0xb3, 0xbb, 0xbf, 0xc8, 0xd7, 0xe6, 0xd6, 0x3c, 0xce]) }, icon: None, provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    RelicPower { id: RelicPowerId("GA_RelicPower_PersonalDamageReductionBuff.GA_RelicPower_PersonalDamageReductionBuff"), name: "Obsidian Skin", description: "Grants <rt.heal>30% Damage Reduction</> to yourself for 9 seconds.\n\n<rt.meikoability2>Can be used during Global Cooldown</>", gameplay_ability_asset: "GA_RelicPower_PersonalDamageReductionBuff_C", gameplay_ability_ref: AbilityRef { source_id: AbilityId("GA_RelicPower_PersonalDamageReductionBuff_C"), guid: AbilityGuid::from_bytes([0x2c, 0x86, 0x9b, 0x27, 0x5e, 0xff, 0x52, 0xf0, 0xaf, 0xf9, 0x21, 0x0d, 0x99, 0x3a, 0xed, 0x7a]) }, icon: None, provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    RelicPower { id: RelicPowerId("GA_RelicPower_PolymorphTargetWithSharedCharges"), name: "Chickenize!", description: "Instantly turn your target into a chicken.\n\n<rt.absorb>Chickenize!</> can only be used a limited amount of times per dungeon. The charges are shared by everyone in the party.\n\n<rt.effect>Chickenized</> enemies <rt.warning>do not give Kill Score.</>\n\n<rt.meikoability2>Can be used during Global Cooldown</>", gameplay_ability_asset: "GA_RelicPower_PolymorphTargetWithSharedCharges_C", gameplay_ability_ref: AbilityRef { source_id: AbilityId("GA_RelicPower_PolymorphTargetWithSharedCharges_C"), guid: AbilityGuid::from_bytes([0xa5, 0x3b, 0xa4, 0x42, 0x39, 0x59, 0x5a, 0x69, 0x97, 0x64, 0x09, 0xd9, 0xee, 0xfe, 0x08, 0x1e]) }, icon: None, provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    RelicPower { id: RelicPowerId("GA_RelicPower_Portal"), name: "Conjure Portal", description: "Conjures two interactable <rt.absorb>Portals</> that you and your allies can use to safely travel between two points.\n\n<rt.meikoability2>Can only have 1 active portal at a time.</>", gameplay_ability_asset: "GA_RelicPower_Portal_C", gameplay_ability_ref: AbilityRef { source_id: AbilityId("GA_RelicPower_Portal_C"), guid: AbilityGuid::from_bytes([0x26, 0xb4, 0x02, 0x91, 0x56, 0x03, 0x54, 0x94, 0x8b, 0xd2, 0x53, 0x6a, 0x8a, 0x56, 0xa6, 0x72]) }, icon: None, provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    RelicPower { id: RelicPowerId("GA_RelicPower_Portal_Interact"), name: "Relic Teleport", description: "", gameplay_ability_asset: "GA_RelicPower_Portal_Interact_C", gameplay_ability_ref: AbilityRef { source_id: AbilityId("GA_RelicPower_Portal_Interact_C"), guid: AbilityGuid::from_bytes([0x4e, 0x0d, 0xa0, 0xcb, 0x8b, 0x75, 0x51, 0x9d, 0xa3, 0x7d, 0x24, 0x8d, 0x99, 0x41, 0xb2, 0x7f]) }, icon: None, provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    RelicPower { id: RelicPowerId("GA_Relic_Potion_Health"), name: "Rejuvenate", description: "Instantly replenish <rt.heal>40% of your maximum health</>.\n\n<rt.meikoability2>Can be used during Global Cooldown</>", gameplay_ability_asset: "GA_RelicPower_HealthPotion_C", gameplay_ability_ref: AbilityRef { source_id: AbilityId("GA_RelicPower_HealthPotion_C"), guid: AbilityGuid::from_bytes([0x0d, 0xee, 0xcd, 0x6b, 0xd3, 0x9c, 0x5d, 0x35, 0xaf, 0x73, 0xa2, 0xdd, 0x59, 0xc9, 0x00, 0x57]) }, icon: None, provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
];
