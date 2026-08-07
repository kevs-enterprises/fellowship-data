"""GENERATED FILE - do not edit by hand.

Rendered from the published `no single dataset file`. Edits are overwritten on the next refresh.
"""

from __future__ import annotations

from dataclasses import dataclass

from ..types import Confidence, Contradicted, Grade, Graded, Unassessed, Unvalidated, Validated, Validation


@dataclass(frozen=True)
class Formula:
    """A damage-composition formula, described but not yet evaluable."""

    id: str
    #: A stable identifier for this formula, scoped to this repository.
    description: str
    #: What this formula computes, in prose — not the expression itself.
    evidence_id: str | None
    #: An opaque handle for the evidence behind this formula. It resolves only against the internal ledger and states nothing about where the value was read from. Absent means no ledger entry exists yet — never a stand-in for one.
    confidence: Confidence
    #: How much standing the extraction behind this formula has. A separate axis from `validation`: nothing having graded it and an oracle having disagreed are different failures, and one number cannot carry both.
    validation: Validation
    #: Whether this description has been checked against a capture of the game. Every record here is unvalidated until one exists.
    build_id: str
    #: The build this description was derived from. A formula's constants are specific to one build, so a consumer must not evaluate this description against another build's numbers.


#: Every recovered formula description, sorted by `id`.
FORMULAS: tuple[Formula, ...] = (
)
