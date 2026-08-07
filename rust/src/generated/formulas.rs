//! GENERATED FILE — do not edit by hand.
//!
//! Generated from `no single dataset file`. Rows are stable-sorted so a data refresh diffs one entity
//! at a time. Anything changed here is overwritten on the next refresh.

#![cfg_attr(rustfmt, rustfmt::skip)]
// `unreadable_literal` asks for digit separators to help a human read a number. These are
// machine-emitted measurements — world coordinates, tick periods, sixty-odd thousand
// curve points — and nobody reads them one at a time. Grouping their digits would change
// every value on every refresh for no reader's benefit.
#![allow(clippy::unreadable_literal)]

use crate::types::{Confidence, Grade, Validation};

/// A damage-composition formula, described but not yet evaluable.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Formula {
    /// A stable identifier for this formula, scoped to this repository.
    pub id: &'static str,
    /// What this formula computes, in prose — not the expression itself.
    pub description: &'static str,
    /// An opaque handle for the evidence behind this formula. It resolves only against the internal ledger and states nothing about where the value was read from. Absent means no ledger entry exists yet — never a stand-in for one.
    pub evidence_id: Option<&'static str>,
    /// How much standing the extraction behind this formula has. A separate axis from `validation`: nothing having graded it and an oracle having disagreed are different failures, and one number cannot carry both.
    pub confidence: Confidence,
    /// Whether this description has been checked against a capture of the game. Every record here is unvalidated until one exists.
    pub validation: Validation,
    /// The build this description was derived from. A formula's constants are specific to one build, so a consumer must not evaluate this description against another build's numbers.
    pub build_id: &'static str,
}

/// Every recovered formula description, sorted by `id`.
pub static FORMULAS: &[Formula] = &[
];
