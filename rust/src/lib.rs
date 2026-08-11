// SOURCED FILE — do not edit here. Edit automation/publish-data/ in the generator; this copy is overwritten on the next sync.

//! Fellowship game data, as a plain Rust library.
//!
//! Everything under [`generated`] is produced by a tool and overwritten on each refresh.
//! [`types`] is hand-written, and is the vocabulary the generated data is expressed in.
//!
//! # Shape
//!
//! There is no runtime loading. Data is `&'static` and compiled in, so there is nothing to parse,
//! nothing to await, and no failure mode where the data is missing. Enable only the domains you
//! need — the enemy difficulty curves are large and off by default.
//!
//! ```toml
//! fellowship-data = { git = "...", default-features = false, features = ["heroes", "items"] }
//! ```
//!
//! # What this crate deliberately is not
//!
//! It exposes no `wasm-bindgen` surface and builds no `cdylib`. It compiles cleanly for
//! `wasm32-unknown-unknown` but does not assume it is going there; a consumer that wants a browser
//! binding writes the one it needs rather than inheriting one.
//!
//! It also contains no images. Records reference art through [`types::MediaHandle`], which carries
//! a stable id and the source dimensions but no path — map the id onto wherever you serve art.
//!
//! # Gaps are values, not silence
//!
//! Where a number could not be resolved, the record says so through [`types::Value`] rather than
//! substituting a default. Deciding what to do about it is left to the caller, on purpose: the
//! alternative is a plausible number nobody measured.

#![cfg_attr(not(test), no_std)]

pub mod types;

/// Generated data. Every module here is overwritten on refresh; edits do not survive.
pub mod generated;

pub use generated::ABILITY_GUID_SCHEME;
pub use types::{
    AbilityGuid, AbilityGuidScheme, AbilityId, AbilityRef, AttributeId, Curve, DungeonId, EffectId,
    HeroId, ItemId, MediaHandle, MediaKind, MobId, Origin, Provenance, TalentId, Value,
};

/// The game build this data was extracted from.
pub const BUILD_ID: &str = include_str!("generated/build_id.txt");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_package_root_exports_the_frozen_ability_guid_scheme() {
        assert_eq!(ABILITY_GUID_SCHEME.name, "ability-guid-v1");
        assert_eq!(
            ABILITY_GUID_SCHEME.namespace,
            "c429f5ee-71e6-4a70-9e5b-4c63ee73e575"
        );
        assert_eq!(ABILITY_GUID_SCHEME.version, 5);
        assert_eq!(
            ABILITY_GUID_SCHEME.transform,
            "strip-terminal-_C;resolve-GameplayAbility;ascii-lowercase;prefix=ability/"
        );
        assert_eq!(ABILITY_GUID_SCHEME.origin, Origin::Derived);
    }
}
