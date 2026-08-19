//! GENERATED FILE — do not edit by hand.
//!
//! Generated from `attributes.json`. Rows are stable-sorted so a data refresh diffs one entity
//! at a time. Anything changed here is overwritten on the next refresh.

#![cfg_attr(rustfmt, rustfmt::skip)]
// `unreadable_literal` asks for digit separators to help a human read a number. These are
// machine-emitted measurements — world coordinates, tick periods, sixty-odd thousand
// curve points — and nobody reads them one at a time. Grouping their digits would change
// every value on every refresh for no reader's benefit.
#![allow(clippy::unreadable_literal)]

use crate::types::{AttributeId, Provenance, SourceAuthority};

/// Weakest acquisition authority across every immutable input behind this module.
pub const SOURCE_AUTHORITY: SourceAuthority = SourceAuthority::LegacyUnbound;

/// One step of a soft cap: beyond `max_bracket_value`, the next bracket applies.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DiminishingBracket {
    pub max_bracket_value: f32,
    pub penalty_percentage: f32,
}

/// A character attribute.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Attribute {
    pub id: AttributeId,
    pub attribute_id: &'static str,
    pub display_name: &'static str,
    pub attribute_name: &'static str,
    pub attribute_set: &'static str,
    /// Whether the value reads as a percentage rather than a flat amount.
    pub is_percent: bool,
    pub display_as_reduction: bool,
    pub round_to_decimals: u8,
    pub base_stat_multiplier: f32,
    pub final_penalty_percentage: f32,
    /// Empty when the attribute has no soft cap.
    pub diminishing_returns: &'static [DiminishingBracket],
    pub provenance: Provenance,
}

/// Every attribute, sorted by `id`.
pub static ATTRIBUTES: &[Attribute] = &[
    Attribute { id: AttributeId("Agility"), attribute_id: "CRBasicAttributeSet.Agility", display_name: "Agility", attribute_name: "Agility", attribute_set: "CRBasicAttributeSet", is_percent: false, display_as_reduction: false, round_to_decimals: 0, base_stat_multiplier: 1.0, final_penalty_percentage: 0.0, diminishing_returns: &[], provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    Attribute { id: AttributeId("Armor"), attribute_id: "CRBasicAttributeSet.Armor", display_name: "Armor", attribute_name: "Armor", attribute_set: "CRBasicAttributeSet", is_percent: false, display_as_reduction: false, round_to_decimals: 0, base_stat_multiplier: 1.0, final_penalty_percentage: 1.0, diminishing_returns: &[DiminishingBracket { max_bracket_value: 50.0, penalty_percentage: 0.9 }, DiminishingBracket { max_bracket_value: 100.0, penalty_percentage: 0.8 }], provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    Attribute { id: AttributeId("AttackSpeed"), attribute_id: "CRBasicAttributeSet.AttackSpeed", display_name: "Attack Speed", attribute_name: "AttackSpeed", attribute_set: "CRBasicAttributeSet", is_percent: true, display_as_reduction: false, round_to_decimals: 0, base_stat_multiplier: 0.05, final_penalty_percentage: 1.0, diminishing_returns: &[], provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    Attribute { id: AttributeId("BlockChance"), attribute_id: "CRBasicAttributeSet.BlockChance", display_name: "Block Chance", attribute_name: "BlockChance", attribute_set: "CRBasicAttributeSet", is_percent: true, display_as_reduction: false, round_to_decimals: 4, base_stat_multiplier: 0.001, final_penalty_percentage: 1.0, diminishing_returns: &[], provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    Attribute { id: AttributeId("BonusArmor"), attribute_id: "CRBasicAttributeSet.Armor", display_name: "Armor", attribute_name: "Armor", attribute_set: "CRBasicAttributeSet", is_percent: false, display_as_reduction: false, round_to_decimals: 0, base_stat_multiplier: 1.0, final_penalty_percentage: 1.0, diminishing_returns: &[], provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    Attribute { id: AttributeId("CooldownReduction"), attribute_id: "CRBasicAttributeSet.CooldownReduction", display_name: "Cooldown Reduction", attribute_name: "CooldownReduction", attribute_set: "CRBasicAttributeSet", is_percent: true, display_as_reduction: false, round_to_decimals: 0, base_stat_multiplier: 0.01, final_penalty_percentage: 0.6, diminishing_returns: &[DiminishingBracket { max_bracket_value: 999.0, penalty_percentage: 1.0 }, DiminishingBracket { max_bracket_value: 1000.0, penalty_percentage: 0.9 }, DiminishingBracket { max_bracket_value: 1000.0, penalty_percentage: 0.8 }, DiminishingBracket { max_bracket_value: 1000.0, penalty_percentage: 0.7 }], provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    Attribute { id: AttributeId("CritChance"), attribute_id: "CRHeroAttributeSet.CritRating", display_name: "Critical Strike", attribute_name: "CritRating", attribute_set: "CRHeroAttributeSet", is_percent: false, display_as_reduction: false, round_to_decimals: 0, base_stat_multiplier: 0.16, final_penalty_percentage: 0.92, diminishing_returns: &[DiminishingBracket { max_bracket_value: 10.0, penalty_percentage: 1.0 }, DiminishingBracket { max_bracket_value: 15.0, penalty_percentage: 0.98 }, DiminishingBracket { max_bracket_value: 20.0, penalty_percentage: 0.96 }, DiminishingBracket { max_bracket_value: 25.0, penalty_percentage: 0.94 }], provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    Attribute { id: AttributeId("Dodge"), attribute_id: "CRBasicAttributeSet.DodgeChance", display_name: "Dodge", attribute_name: "DodgeChance", attribute_set: "CRBasicAttributeSet", is_percent: true, display_as_reduction: false, round_to_decimals: 4, base_stat_multiplier: 0.001, final_penalty_percentage: 1.0, diminishing_returns: &[], provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    Attribute { id: AttributeId("Expertise"), attribute_id: "CRHeroAttributeSet.ExpertiseRating", display_name: "Expertise", attribute_name: "ExpertiseRating", attribute_set: "CRHeroAttributeSet", is_percent: false, display_as_reduction: false, round_to_decimals: 0, base_stat_multiplier: 0.16, final_penalty_percentage: 0.92, diminishing_returns: &[DiminishingBracket { max_bracket_value: 10.0, penalty_percentage: 1.0 }, DiminishingBracket { max_bracket_value: 15.0, penalty_percentage: 0.98 }, DiminishingBracket { max_bracket_value: 20.0, penalty_percentage: 0.96 }, DiminishingBracket { max_bracket_value: 25.0, penalty_percentage: 0.94 }], provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    Attribute { id: AttributeId("FlatHealth"), attribute_id: "CRBasicAttributeSet.BaseHealth", display_name: "Health", attribute_name: "BaseHealth", attribute_set: "CRBasicAttributeSet", is_percent: false, display_as_reduction: false, round_to_decimals: 0, base_stat_multiplier: 1.0, final_penalty_percentage: 1.0, diminishing_returns: &[], provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    Attribute { id: AttributeId("FlatMana"), attribute_id: "CRBasicAttributeSet.MaxMana", display_name: "Mana", attribute_name: "MaxMana", attribute_set: "CRBasicAttributeSet", is_percent: true, display_as_reduction: false, round_to_decimals: 0, base_stat_multiplier: 1.0, final_penalty_percentage: 1.0, diminishing_returns: &[], provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    Attribute { id: AttributeId("Haste"), attribute_id: "CRHeroAttributeSet.HasteRating", display_name: "Haste", attribute_name: "HasteRating", attribute_set: "CRHeroAttributeSet", is_percent: false, display_as_reduction: false, round_to_decimals: 0, base_stat_multiplier: 0.16, final_penalty_percentage: 0.92, diminishing_returns: &[DiminishingBracket { max_bracket_value: 10.0, penalty_percentage: 1.0 }, DiminishingBracket { max_bracket_value: 15.0, penalty_percentage: 0.98 }, DiminishingBracket { max_bracket_value: 20.0, penalty_percentage: 0.96 }, DiminishingBracket { max_bracket_value: 25.0, penalty_percentage: 0.94 }], provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    Attribute { id: AttributeId("HealthRegen"), attribute_id: "CRBasicAttributeSet.HealthRegenTickTime", display_name: "Health Regen Rate", attribute_name: "HealthRegenTickTime", attribute_set: "CRBasicAttributeSet", is_percent: true, display_as_reduction: false, round_to_decimals: 2, base_stat_multiplier: 0.03, final_penalty_percentage: 1.0, diminishing_returns: &[], provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    Attribute { id: AttributeId("Intellect"), attribute_id: "CRBasicAttributeSet.Intellect", display_name: "Intellect", attribute_name: "Intellect", attribute_set: "CRBasicAttributeSet", is_percent: false, display_as_reduction: false, round_to_decimals: 0, base_stat_multiplier: 1.0, final_penalty_percentage: 0.0, diminishing_returns: &[], provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    Attribute { id: AttributeId("LifeSteal"), attribute_id: "CRBasicAttributeSet.LifeSteal", display_name: "Life Steal", attribute_name: "LifeSteal", attribute_set: "CRBasicAttributeSet", is_percent: true, display_as_reduction: false, round_to_decimals: 4, base_stat_multiplier: 0.001, final_penalty_percentage: 1.0, diminishing_returns: &[], provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    Attribute { id: AttributeId("Mana"), attribute_id: "CRBasicAttributeSet.MaxMana", display_name: "Max Mana", attribute_name: "MaxMana", attribute_set: "CRBasicAttributeSet", is_percent: true, display_as_reduction: false, round_to_decimals: 2, base_stat_multiplier: 0.03, final_penalty_percentage: 1.0, diminishing_returns: &[], provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    Attribute { id: AttributeId("ManaRegen"), attribute_id: "CRBasicAttributeSet.ManaRegenTickTime", display_name: "Mana Regen Rate", attribute_name: "ManaRegenTickTime", attribute_set: "CRBasicAttributeSet", is_percent: true, display_as_reduction: false, round_to_decimals: 2, base_stat_multiplier: 0.015, final_penalty_percentage: 1.0, diminishing_returns: &[], provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    Attribute { id: AttributeId("MoveSpeed"), attribute_id: "CRBasicAttributeSet.MoveSpeed", display_name: "Move Speed", attribute_name: "MoveSpeed", attribute_set: "CRBasicAttributeSet", is_percent: true, display_as_reduction: false, round_to_decimals: 4, base_stat_multiplier: 0.001, final_penalty_percentage: 1.0, diminishing_returns: &[], provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    Attribute { id: AttributeId("Parry"), attribute_id: "CRBasicAttributeSet.ParryChance", display_name: "Parry", attribute_name: "ParryChance", attribute_set: "CRBasicAttributeSet", is_percent: true, display_as_reduction: false, round_to_decimals: 4, base_stat_multiplier: 0.001, final_penalty_percentage: 1.0, diminishing_returns: &[], provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    Attribute { id: AttributeId("ReflectHeal"), attribute_id: "CRBasicAttributeSet.ReflectHeal", display_name: "Reflect Heal", attribute_name: "ReflectHeal", attribute_set: "CRBasicAttributeSet", is_percent: true, display_as_reduction: false, round_to_decimals: 4, base_stat_multiplier: 0.001, final_penalty_percentage: 1.0, diminishing_returns: &[], provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    Attribute { id: AttributeId("Spirit"), attribute_id: "CRHeroAttributeSet.SpiritRating", display_name: "Spirit", attribute_name: "SpiritRating", attribute_set: "CRHeroAttributeSet", is_percent: false, display_as_reduction: false, round_to_decimals: 0, base_stat_multiplier: 0.16, final_penalty_percentage: 0.92, diminishing_returns: &[DiminishingBracket { max_bracket_value: 10.0, penalty_percentage: 1.0 }, DiminishingBracket { max_bracket_value: 15.0, penalty_percentage: 0.98 }, DiminishingBracket { max_bracket_value: 20.0, penalty_percentage: 0.96 }, DiminishingBracket { max_bracket_value: 25.0, penalty_percentage: 0.94 }], provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    Attribute { id: AttributeId("Stamina"), attribute_id: "CRBasicAttributeSet.Stamina", display_name: "Stamina", attribute_name: "Stamina", attribute_set: "CRBasicAttributeSet", is_percent: false, display_as_reduction: false, round_to_decimals: 0, base_stat_multiplier: 1.0, final_penalty_percentage: 0.0, diminishing_returns: &[], provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    Attribute { id: AttributeId("Strength"), attribute_id: "CRBasicAttributeSet.Strength", display_name: "Strength", attribute_name: "Strength", attribute_set: "CRBasicAttributeSet", is_percent: false, display_as_reduction: false, round_to_decimals: 0, base_stat_multiplier: 1.0, final_penalty_percentage: 0.0, diminishing_returns: &[], provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
    Attribute { id: AttributeId("Tenacity"), attribute_id: "CRBasicAttributeSet.Tenacity", display_name: "Tenacity", attribute_name: "Tenacity", attribute_set: "CRBasicAttributeSet", is_percent: false, display_as_reduction: false, round_to_decimals: 0, base_stat_multiplier: 1.0, final_penalty_percentage: 1.0, diminishing_returns: &[], provenance: Provenance::datamine(SourceAuthority::LegacyUnbound) },
];
