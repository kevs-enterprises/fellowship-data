// SOURCED FILE — do not edit here. Edit automation/publish-data/ in the generator; this copy is overwritten on the next sync.

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

/// Whether a modelled formula has been checked against a recorded capture.
///
/// Not a confidence number. "Never checked" and "checked and disagreed" are different states with
/// independent failure modes — the first is absence of evidence, the second is evidence of a
/// problem — and one scalar cannot carry both, so they are separate variants and a consumer has to
/// decide which it can live with.
///
/// A contradicted formula still publishes. The marker records that two methods disagreed; it does
/// not assert which one is wrong.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Validation {
    /// Checked against at least one capture, and it agreed.
    Validated,
    /// No capture has checked this formula.
    Unvalidated { reason: &'static str },
    /// A capture checked this formula and disagreed. `delta` is the observed difference.
    Contradicted { reason: &'static str, delta: f32 },
}

impl Validation {
    /// Whether an oracle has agreed with this formula.
    ///
    /// Named for what it asserts. `!is_validated()` covers both "nobody checked" and "somebody
    /// checked and it was wrong", which are not the same risk.
    #[must_use]
    pub const fn is_validated(&self) -> bool {
        matches!(self, Self::Validated)
    }

    /// The observed disagreement, when one was measured.
    #[must_use]
    pub const fn contradiction_delta(&self) -> Option<f32> {
        match self {
            Self::Contradicted { delta, .. } => Some(*delta),
            _ => None,
        }
    }
}

/// One axis of how much standing an extraction has.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grade {
    Low,
    Medium,
    High,
    Verified,
}

/// How much standing the extraction behind a formula has.
///
/// A separate axis from [`Validation`], not a finer grade of it. Validation asks whether an oracle
/// agreed; this asks how well the value was recovered in the first place. A formula can be
/// well-extracted and contradicted, or ungraded and validated, and collapsing the two onto one
/// number would lose whichever failed.
///
/// Graded on the three axes the extraction actually has. Coverage is a corpus-level summary and is
/// deliberately not here: it is not a property of any one formula.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Confidence {
    /// Nothing has graded this extraction. Carries a reason rather than being a bottom grade —
    /// "nobody assessed this" and "assessed, and it is weak" are different claims.
    Unassessed { reason: &'static str },
    /// Graded on each axis.
    Graded {
        /// How firmly the value is bound to the thing it claims to describe.
        binding: Grade,
        /// How reliably the bytes were read.
        extraction: Grade,
        /// How well what was read is understood to mean what it is published as.
        interpretation: Grade,
    },
}

impl Confidence {
    /// Whether this extraction has been graded at all.
    ///
    /// Named for what it asserts. `!is_graded()` means nobody has assessed it, which is not the
    /// same as a low grade — see [`Confidence::Unassessed`].
    #[must_use]
    pub const fn is_graded(&self) -> bool {
        matches!(self, Self::Graded { .. })
    }
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

/// Provenance carried alongside a record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Provenance {
    pub origin: Origin,
    /// The developer-facing identifier, where one exists.
    pub dev_name: Option<&'static str>,
    /// For [`Origin::Derived`], the transform. For [`Origin::Overlay`], the reason.
    pub source: Option<&'static str>,
}

impl Provenance {
    /// Read directly from the game's own data.
    pub const DATAMINE: Self = Self {
        origin: Origin::Datamine,
        dev_name: None,
        source: None,
    };
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
}
