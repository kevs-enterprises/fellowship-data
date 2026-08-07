//! GENERATED FILE — do not edit by hand.
//!
//! Generated from `set_bonuses.json`. Rows are stable-sorted so a data refresh diffs one entity
//! at a time. Anything changed here is overwritten on the next refresh.

#![cfg_attr(rustfmt, rustfmt::skip)]
// `unreadable_literal` asks for digit separators to help a human read a number. These are
// machine-emitted measurements — world coordinates, tick periods, sixty-odd thousand
// curve points — and nobody reads them one at a time. Grouping their digits would change
// every value on every refresh for no reader's benefit.
#![allow(clippy::unreadable_literal)]

use crate::types::{MediaHandle, MediaKind, Provenance, SetBonusId};

/// A bonus granted for wearing several pieces of one item set.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SetBonus {
    pub id: SetBonusId,
    /// The gameplay tag this set bonus is addressed by.
    pub set_bonus_id: &'static str,
    pub numeric_id: u32,
    pub display_name: &'static str,
    /// How many pieces of the set must be worn.
    pub set_count: u8,
    pub description: &'static str,
    /// The primary attributes this set bonus may roll for.
    ///
    /// Published in the same namespace `attributes.json` uses. The source carries the enum
    /// spelling (`ATTRIBUTE_AGILITY`), which resolves against nothing — the same mismatch
    /// `heroes.primary_attribute` had, and the same named transform.
    pub allowed_attributes: &'static [&'static str],
    pub icon: Option<MediaHandle>,
    pub provenance: Provenance,
}

/// Every item set bonus, sorted by `id`.
pub static SET_BONUSES: &[SetBonus] = &[
    SetBonus { id: SetBonusId("Quickplay"), set_bonus_id: "ItemID.SetBonus.Quickplay", numeric_id: 2098, display_name: "Expedition Heroism", set_count: 10, description: "+15% Critical Chance\n+20% Haste\n+20% Expertise\n+20% Spirit", allowed_attributes: &[], icon: Some(MediaHandle { id: "T_UI_Icon_Rarity_Common", kind: MediaKind::Other, width: 256, height: 256 }), provenance: Provenance::DATAMINE },
    SetBonus { id: SetBonusId("SetA_Percentage_DH"), set_bonus_id: "ItemID.SetBonus.SetA_Percentage_DH", numeric_id: 1769, display_name: "Draconic Deceit", set_count: 2, description: "+4% Critical Strike Chance\n-15% Threat Generation", allowed_attributes: &["Intellect"], icon: Some(MediaHandle { id: "T_Icon_Frost_134_BetaCropped", kind: MediaKind::Other, width: 256, height: 256 }), provenance: Provenance::DATAMINE },
    SetBonus { id: SetBonusId("SetA_Percentage_T"), set_bonus_id: "ItemID.SetBonus.SetA_Percentage_T", numeric_id: 1770, display_name: "Draconic Fury", set_count: 2, description: "+4% Critical Strike Chance\n+15% Threat Generation", allowed_attributes: &["Strength", "Agility", "Intellect"], icon: Some(MediaHandle { id: "T_Icon_Frost_134_BetaCropped", kind: MediaKind::Other, width: 256, height: 256 }), provenance: Provenance::DATAMINE },
    SetBonus { id: SetBonusId("SetA_Proc_Agility"), set_bonus_id: "ItemID.SetBonus.SetA_Proc_Agility", numeric_id: 677, display_name: "Draconic Might", set_count: 2, description: "Your critical strikes have a chance (0.9 PPM) to increase your Agility by 18% for 14 seconds.", allowed_attributes: &["Agility"], icon: Some(MediaHandle { id: "T_Icon_Frost_113", kind: MediaKind::Other, width: 256, height: 256 }), provenance: Provenance::DATAMINE },
    SetBonus { id: SetBonusId("SetA_Proc_Intellect"), set_bonus_id: "ItemID.SetBonus.SetA_Proc_Intellect", numeric_id: 678, display_name: "Draconic Might", set_count: 2, description: "Your critical strikes have a chance (0.9 PPM) to increase your Intellect by 18% for 14 seconds.", allowed_attributes: &["Intellect"], icon: Some(MediaHandle { id: "T_Icon_Frost_113", kind: MediaKind::Other, width: 256, height: 256 }), provenance: Provenance::DATAMINE },
    SetBonus { id: SetBonusId("SetA_Proc_Strength"), set_bonus_id: "ItemID.SetBonus.SetA_Proc_Strength", numeric_id: 679, display_name: "Draconic Might", set_count: 2, description: "Your critical strikes have a chance (0.9 PPM) to increase your Strength by 18% for 14 seconds.", allowed_attributes: &["Strength"], icon: Some(MediaHandle { id: "T_Icon_Frost_113", kind: MediaKind::Other, width: 256, height: 256 }), provenance: Provenance::DATAMINE },
    SetBonus { id: SetBonusId("SetB_Percentage_HDT"), set_bonus_id: "ItemID.SetBonus.SetB_Percentage_HDT", numeric_id: 680, display_name: "Tuzari Grace", set_count: 2, description: "+4% Haste\n+20% Movement Speed", allowed_attributes: &["Strength", "Agility", "Intellect"], icon: Some(MediaHandle { id: "T_Icon_Fire_162_BetaCropped", kind: MediaKind::Other, width: 256, height: 256 }), provenance: Provenance::DATAMINE },
    SetBonus { id: SetBonusId("SetB_Proc_HDT"), set_bonus_id: "ItemID.SetBonus.SetB_Proc_HDT", numeric_id: 681, display_name: "Dark Prophecy", set_count: 2, description: "Your abilities have a chance (0.8 PPM) to increase your Haste by 25% for 20 seconds.", allowed_attributes: &["Strength", "Agility", "Intellect"], icon: Some(MediaHandle { id: "T_Icon_Fire_162_BetaCropped", kind: MediaKind::Other, width: 256, height: 256 }), provenance: Provenance::DATAMINE },
    SetBonus { id: SetBonusId("SetC_Percentage_D"), set_bonus_id: "ItemID.SetBonus.SetC_Percentage_D", numeric_id: 682, display_name: "Sin Warding", set_count: 2, description: "+4% Expertise\n+5% Max Health", allowed_attributes: &["Strength", "Agility", "Intellect"], icon: Some(MediaHandle { id: "T_Icon_Gold_147_BetaCropped", kind: MediaKind::Other, width: 256, height: 256 }), provenance: Provenance::DATAMINE },
    SetBonus { id: SetBonusId("SetC_Percentage_H"), set_bonus_id: "ItemID.SetBonus.SetC_Percentage_H", numeric_id: 683, display_name: "Sin Warding", set_count: 2, description: "+4% Expertise\n+5% Max Health", allowed_attributes: &["Strength", "Agility", "Intellect"], icon: Some(MediaHandle { id: "T_Icon_Gold_147_BetaCropped", kind: MediaKind::Other, width: 256, height: 256 }), provenance: Provenance::DATAMINE },
    SetBonus { id: SetBonusId("SetC_Percentage_T"), set_bonus_id: "ItemID.SetBonus.SetC_Percentage_T", numeric_id: 684, display_name: "Sin Warding", set_count: 2, description: "+4% Expertise\n+5% Max Health", allowed_attributes: &["Strength", "Agility", "Intellect"], icon: Some(MediaHandle { id: "T_Icon_Gold_147_BetaCropped", kind: MediaKind::Other, width: 256, height: 256 }), provenance: Provenance::DATAMINE },
    SetBonus { id: SetBonusId("SetC_Proc_Agility"), set_bonus_id: "ItemID.SetBonus.SetC_Proc_Agility", numeric_id: 685, display_name: "Torment of Bael'Aurum", set_count: 2, description: "+5% Agility\n\nWhen dipping below 20% Health, you are instantly healed for 40% of your maximum health. This can occur once every 180 seconds.", allowed_attributes: &["Agility"], icon: Some(MediaHandle { id: "T_Icon_Gold_147_BetaCropped", kind: MediaKind::Other, width: 256, height: 256 }), provenance: Provenance::DATAMINE },
    SetBonus { id: SetBonusId("SetC_Proc_Intellect"), set_bonus_id: "ItemID.SetBonus.SetC_Proc_Intellect", numeric_id: 686, display_name: "Torment of Bael'Aurum", set_count: 2, description: "+5% Intellect\n\nWhen dipping below 20% Health, you are instantly healed for 40% of your maximum health. This can occur once every 180 seconds.", allowed_attributes: &["Intellect"], icon: Some(MediaHandle { id: "T_Icon_Gold_147_BetaCropped", kind: MediaKind::Other, width: 256, height: 256 }), provenance: Provenance::DATAMINE },
    SetBonus { id: SetBonusId("SetC_Proc_Strength"), set_bonus_id: "ItemID.SetBonus.SetC_Proc_Strength", numeric_id: 687, display_name: "Torment of Bael'Aurum", set_count: 2, description: "+5% Strength\n\nWhen dipping below 20% Health, you are instantly healed for 40% of your maximum health. This can occur once every 180 seconds.", allowed_attributes: &["Strength"], icon: Some(MediaHandle { id: "T_Icon_Gold_147_BetaCropped", kind: MediaKind::Other, width: 256, height: 256 }), provenance: Provenance::DATAMINE },
    SetBonus { id: SetBonusId("SetD_Percentage_D"), set_bonus_id: "ItemID.SetBonus.SetD_Percentage_D", numeric_id: 1771, display_name: "Death's Grasp", set_count: 2, description: "+4% Spirit\n+10% damage to Low Health enemies.", allowed_attributes: &["Strength", "Agility", "Intellect"], icon: Some(MediaHandle { id: "LootModifier_SetBonus", kind: MediaKind::Other, width: 256, height: 256 }), provenance: Provenance::DATAMINE },
    SetBonus { id: SetBonusId("SetD_Percentage_H"), set_bonus_id: "ItemID.SetBonus.SetD_Percentage_H", numeric_id: 1772, display_name: "Haunting Lament", set_count: 2, description: "+4% Spirit\n+15% Max Mana", allowed_attributes: &["Strength", "Agility", "Intellect"], icon: Some(MediaHandle { id: "LootModifier_SetBonus", kind: MediaKind::Other, width: 256, height: 256 }), provenance: Provenance::DATAMINE },
    SetBonus { id: SetBonusId("SetD_Percentage_T"), set_bonus_id: "ItemID.SetBonus.SetD_Percentage_T", numeric_id: 689, display_name: "Sinthara's Veil", set_count: 2, description: "+4% Spirit\n+10% magic damage reduction", allowed_attributes: &["Strength", "Agility", "Intellect"], icon: Some(MediaHandle { id: "LootModifier_SetBonus", kind: MediaKind::Other, width: 256, height: 256 }), provenance: Provenance::DATAMINE },
    SetBonus { id: SetBonusId("SetD_Proc_Agility"), set_bonus_id: "ItemID.SetBonus.SetD_Proc_Agility", numeric_id: 690, display_name: "Drakheim's Absolution", set_count: 2, description: "Your Spirit Ability grants you Drakheim's Absolution, increasing your Agility by 20% for 20 seconds.", allowed_attributes: &["Agility"], icon: Some(MediaHandle { id: "T_Icon_Energy_125", kind: MediaKind::Other, width: 256, height: 256 }), provenance: Provenance::DATAMINE },
    SetBonus { id: SetBonusId("SetD_Proc_Intellect"), set_bonus_id: "ItemID.SetBonus.SetD_Proc_Intellect", numeric_id: 691, display_name: "Drakheim's Absolution", set_count: 2, description: "Your Spirit Ability grants you Drakheim's Absolution, increasing your Intellect by 20% for 20 seconds.", allowed_attributes: &["Intellect"], icon: Some(MediaHandle { id: "T_Icon_Energy_125", kind: MediaKind::Other, width: 256, height: 256 }), provenance: Provenance::DATAMINE },
    SetBonus { id: SetBonusId("SetD_Proc_Strength"), set_bonus_id: "ItemID.SetBonus.SetD_Proc_Strength", numeric_id: 692, display_name: "Drakheim's Absolution", set_count: 2, description: "Your Spirit Ability grants you Drakheim's Absolution, increasing your Strength by 20% for 20 seconds.", allowed_attributes: &["Strength"], icon: Some(MediaHandle { id: "T_Icon_Energy_125", kind: MediaKind::Other, width: 256, height: 256 }), provenance: Provenance::DATAMINE },
    SetBonus { id: SetBonusId("SetE_Percentage_HDT"), set_bonus_id: "ItemID.SetBonus.SetE_Percentage_HDT", numeric_id: 5638, display_name: "Seal of the Heskyr", set_count: 2, description: "Your Gem Power is increased by 25%.", allowed_attributes: &["Strength", "Agility", "Intellect"], icon: Some(MediaHandle { id: "T_Icons_Generic_Placeholder", kind: MediaKind::Other, width: 128, height: 128 }), provenance: Provenance::DATAMINE },
];
