//! GENERATED FILE — do not edit by hand.
//!
//! Each domain is feature-gated so a consumer compiles in only what it uses.

use crate::types::{AbilityGuidScheme, Origin};

/// The protocol used to derive every published Ability GUID.
pub const ABILITY_GUID_SCHEME: AbilityGuidScheme = AbilityGuidScheme {
    name: "ability-guid-v1",
    namespace: "c429f5ee-71e6-4a70-9e5b-4c63ee73e575",
    version: 5,
    transform: "strip-terminal-_C;resolve-GameplayAbility;ascii-lowercase;prefix=ability/",
    origin: Origin::Derived,
};

#[cfg(feature = "abilities")]
pub mod abilities;
#[cfg(feature = "ability_constants")]
pub mod ability_constants;
#[cfg(feature = "attributes")]
pub mod attributes;
#[cfg(feature = "combat_constants")]
pub mod combat_constants;
#[cfg(feature = "constants")]
pub mod constants;
#[cfg(feature = "dungeons")]
pub mod dungeons;
#[cfg(feature = "effect_executions")]
pub mod effect_executions;
#[cfg(feature = "effects")]
pub mod effects;
#[cfg(feature = "finesses")]
pub mod finesses;
#[cfg(feature = "heroes")]
pub mod heroes;
#[cfg(feature = "items")]
pub mod items;
#[cfg(feature = "mobs")]
pub mod mobs;
#[cfg(feature = "modifiers")]
pub mod modifiers;
#[cfg(feature = "relic_powers")]
pub mod relic_powers;
#[cfg(feature = "set_bonuses")]
pub mod set_bonuses;
#[cfg(feature = "tag_ids")]
pub mod tag_ids;
#[cfg(feature = "talents")]
pub mod talents;
#[cfg(feature = "traits")]
pub mod traits;
