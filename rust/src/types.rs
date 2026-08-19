// SOURCED FILE — do not edit this copy; it is overwritten on the next data sync.

//! The vocabulary the data is expressed in.
//!
//! Unlike everything under [`crate::generated`], this file is hand-written. Data that does not fit
//! these types fails to compile, which is the only thing keeping the two in agreement.

/// A difficulty-indexed curve.
///
/// Two genuinely different shapes occur, and flattening them together would assert something
/// untrue. Enemy scaling is dense — one value per difficulty from 1 to 151. Hero scaling is a pair
/// of keyframes at the ends of that range, describing a ramp rather than a table.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Curve {
    /// One value per difficulty, covering `1..=len`.
    Dense(&'static [f32]),
    /// `(difficulty, value)` pairs in ascending order.
    Keyframes(&'static [(f32, f32)]),
}

impl Curve {
    /// The value at an exact difficulty, when the data states one.
    ///
    /// Returns `None` rather than interpolating between keyframes. How the game interpolates is a
    /// modelling decision for the caller; guessing it here would manufacture numbers nobody
    /// measured.
    #[must_use]
    pub fn at(&self, difficulty: u32) -> Option<f32> {
        match self {
            Self::Dense(values) => difficulty
                .checked_sub(1)
                .and_then(|index| usize::try_from(index).ok())
                .and_then(|index| values.get(index))
                .copied(),
            // Compared in `f64`, which represents every `f32` and every `u32` exactly, so the
            // match is on the actual values rather than on a lossy narrowing of the difficulty.
            Self::Keyframes(points) => points
                .iter()
                .find(|(at, _)| (f64::from(*at) - f64::from(difficulty)).abs() < f64::EPSILON)
                .map(|(_, value)| *value),
        }
    }

    /// Whether the curve states anything at all.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        match self {
            Self::Dense(values) => values.is_empty(),
            Self::Keyframes(points) => points.is_empty(),
        }
    }
}

/// A value that could not be fully resolved.
///
/// Collapsing an unresolved value to a plain number turns "we do not know" into "it is 1.0", and
/// the two read identically at the call site. Modelling the gap in the type forces a decision.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value<T: 'static> {
    /// Read directly from the game's data.
    Known(T),
    /// Defined by a game curve that has not been resolved. `default` is the fallback the asset
    /// declares — not a measurement.
    Curve {
        curve_ref: &'static str,
        default: Option<T>,
    },
    /// Known to exist, with no value recovered. Never silently omitted.
    Unresolved { reason: &'static str },
}

impl<T: Copy> Value<T> {
    /// The value only when it was actually measured.
    #[must_use]
    pub const fn known(&self) -> Option<T> {
        match self {
            Self::Known(value) => Some(*value),
            _ => None,
        }
    }

    /// The measured value, or the asset's declared fallback.
    ///
    /// Named the long way to make the compromise visible at the call site: a fallback is what the
    /// asset says to use, not what the game was observed to do.
    #[must_use]
    pub const fn known_or_declared_default(&self) -> Option<T> {
        match self {
            Self::Known(value) => Some(*value),
            Self::Curve { default, .. } => *default,
            Self::Unresolved { .. } => None,
        }
    }

    /// Whether this value carries a real measurement.
    #[must_use]
    pub const fn is_known(&self) -> bool {
        matches!(self, Self::Known(_))
    }
}

/// Stable consumer identity for an Ability.
///
/// The bytes are UUID bytes, kept dependency-free so consumers do not inherit a UUID crate merely
/// by depending on this data package. [`core::fmt::Display`] emits the canonical lowercase,
/// hyphenated representation used by the structural deliveries.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AbilityGuid([u8; 16]);

impl AbilityGuid {
    /// Construct from the UUID's 16 network-order bytes.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }

    /// The UUID's 16 network-order bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }
}

impl core::fmt::Display for AbilityGuid {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for (index, byte) in self.0.iter().enumerate() {
            if matches!(index, 4 | 6 | 8 | 10) {
                f.write_str("-")?;
            }
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

/// A source Ability occurrence paired with its stable consumer identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AbilityRef {
    pub source_id: AbilityId,
    pub guid: AbilityGuid,
}

/// Where a record came from.
///
/// Kept on every record because a hand-authored correction and a value read from the game must
/// never be indistinguishable to a consumer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Origin {
    /// Read directly from the game's own data.
    Datamine,
    /// Computed from other values by a named transform.
    Derived,
    /// Hand-authored to cover a known gap.
    Overlay,
}

/// Strength of the immutable source binding behind a published fact.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceAuthority {
    /// Written by an authenticated, allowlisted producer for the stated build.
    ProducerBound,
    /// Exact reviewed bytes whose original acquisition/build ancestry is not authenticated.
    LegacyUnbound,
    /// Loaded outside the authenticated source-binding gate.
    Unclassified,
}

/// The public protocol used to derive Ability GUIDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AbilityGuidScheme {
    pub name: &'static str,
    /// Canonical lowercase, hyphenated UUID text. This is a namespace, not an entity identity.
    pub namespace: &'static str,
    pub version: u8,
    pub transform: &'static str,
    pub origin: Origin,
}

/// Provenance carried alongside a record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Provenance {
    pub origin: Origin,
    /// The developer-facing identifier, where one exists.
    pub dev_name: Option<&'static str>,
    /// For [`Origin::Derived`], the transform. For [`Origin::Overlay`], the reason.
    pub source: Option<&'static str>,
    /// Acquisition authority of the weakest immutable input behind this record.
    pub source_authority: SourceAuthority,
}

impl Provenance {
    /// Read directly from the game's own data.
    pub const DATAMINE_UNCLASSIFIED: Self = Self {
        origin: Origin::Datamine,
        dev_name: None,
        source: None,
        source_authority: SourceAuthority::Unclassified,
    };

    /// A datamined record with its acquisition authority stated explicitly.
    #[must_use]
    pub const fn datamine(source_authority: SourceAuthority) -> Self {
        Self {
            origin: Origin::Datamine,
            dev_name: None,
            source: None,
            source_authority,
        }
    }
}

/// A reference to an image this crate does not contain.
///
/// A handle carries a stable id and the source dimensions, never a path. Map `id` onto wherever
/// you serve art from; ids stay stable across refreshes, so asset paths keep working.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MediaHandle {
    /// Stable across re-exports, so a consumer's own asset paths stay valid.
    pub id: &'static str,
    pub kind: MediaKind,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaKind {
    HeroPortrait,
    HeroSelectBackground,
    AbilityIcon,
    ItemIcon,
    TalentIcon,
    MobPortrait,
    DungeonMap,
    Other,
}

macro_rules! id_newtype {
    ($($(#[$doc:meta])* $name:ident),* $(,)?) => {
        $(
            $(#[$doc])*
            ///
            /// A newtype so a hero cannot be passed where an ability is expected — identifiers are
            /// all strings and several namespaces overlap.
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct $name(pub &'static str);

            impl $name {
                /// The underlying identifier.
                #[must_use]
                pub const fn as_str(&self) -> &'static str { self.0 }
            }

            impl core::fmt::Display for $name {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.write_str(self.0)
                }
            }
        )*
    };
}

id_newtype! {
    /// Identifies a playable hero.
    HeroId,
    /// Identifies an ability or passive.
    AbilityId,
    /// Identifies an item.
    ItemId,
    /// Identifies an enemy or boss.
    MobId,
    /// Identifies a character attribute.
    AttributeId,
    /// Identifies a gameplay effect.
    EffectId,
    /// Identifies a talent.
    TalentId,
    /// Identifies a dungeon or zone.
    DungeonId,
    /// Identifies an item trait.
    TraitId,
    /// Identifies an armor-set bonus.
    SetBonusId,
    /// Identifies a finesse.
    FinesseId,
    /// Identifies a relic power.
    RelicPowerId,
    /// Identifies a gameplay tag, as `<namespace>/<tag>`.
    TagId,
    /// Identifies a combat-model constant by its dotted path.
    CombatConstantId,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_dense_curve_is_indexed_from_difficulty_one() {
        let curve = Curve::Dense(&[10.0, 20.0, 30.0]);
        assert_eq!(curve.at(1), Some(10.0));
        assert_eq!(curve.at(3), Some(30.0));
    }

    /// Difficulty 0 does not exist. Indexing must not wrap into the last element.
    #[test]
    fn difficulty_zero_and_out_of_range_yield_nothing() {
        let curve = Curve::Dense(&[10.0, 20.0]);
        assert_eq!(curve.at(0), None);
        assert_eq!(curve.at(3), None);
    }

    #[test]
    fn a_keyframed_curve_answers_only_at_its_keyframes() {
        let curve = Curve::Keyframes(&[(1.0, 1.0), (151.0, 4.0)]);
        assert_eq!(curve.at(1), Some(1.0));
        assert_eq!(curve.at(151), Some(4.0));
    }

    /// Interpolating here would invent a number nobody measured.
    #[test]
    fn a_keyframed_curve_does_not_interpolate_between_keyframes() {
        let curve = Curve::Keyframes(&[(1.0, 1.0), (151.0, 4.0)]);
        assert_eq!(curve.at(75), None);
    }

    #[test]
    fn a_known_value_reports_itself() {
        let value: Value<f32> = Value::Known(2.5);
        assert_eq!(value.known(), Some(2.5));
        assert!(value.is_known());
    }

    /// The distinction the whole type exists for: a declared fallback is not a measurement, and
    /// `known()` must not surface it.
    #[test]
    fn a_declared_default_is_not_reported_as_known() {
        let value: Value<f32> = Value::Curve {
            curve_ref: "Bowguy.RangedRange",
            default: Some(1.0),
        };
        assert_eq!(value.known(), None);
        assert!(!value.is_known());
        assert_eq!(value.known_or_declared_default(), Some(1.0));
    }

    #[test]
    fn an_unresolved_value_offers_nothing() {
        let value: Value<f32> = Value::Unresolved {
            reason: "no constants row for this hero",
        };
        assert_eq!(value.known(), None);
        assert_eq!(value.known_or_declared_default(), None);
    }

    #[test]
    fn identifiers_of_different_kinds_are_distinct_types() {
        let hero = HeroId("Bowguy");
        assert_eq!(hero.as_str(), "Bowguy");
        assert_eq!(hero.to_string(), "Bowguy");
        // AbilityId("Bowguy") is a different type and will not compare with `hero`; that is the
        // point of the newtypes and is enforced at compile time rather than here.
    }

    #[test]
    fn an_ability_guid_is_exactly_sixteen_bytes_and_displays_canonically() {
        let guid = AbilityGuid::from_bytes([
            0xfa, 0xf2, 0x95, 0xc9, 0x97, 0x58, 0x5d, 0xc6, 0xab, 0xb9, 0xc4, 0xb2, 0xce,
            0xf8, 0xd5, 0x31,
        ]);
        assert_eq!(core::mem::size_of::<AbilityGuid>(), 16);
        assert_eq!(guid.as_bytes().len(), 16);
        assert_eq!(
            guid.to_string(),
            "faf295c9-9758-5dc6-abb9-c4b2cef8d531"
        );

        let leading_zero_bytes = AbilityGuid::from_bytes([
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c,
            0x0d, 0x0e, 0x0f,
        ]);
        assert_eq!(
            leading_zero_bytes.to_string(),
            "00010203-0405-0607-0809-0a0b0c0d0e0f"
        );

        let reference = AbilityRef {
            source_id: AbilityId("GA_Bowguy_RangedAutoAttack_C"),
            guid,
        };
        assert_eq!(reference.source_id.as_str(), "GA_Bowguy_RangedAutoAttack_C");
        assert_eq!(reference.guid, guid);
    }
}
