# SOURCED FILE — do not edit here. Edit automation/publish/ in the generator; this copy is overwritten on the next sync.

"""The vocabulary the data is expressed in.

Unlike everything under :mod:`fellowship_data.generated`, this file is hand-written. It states the
same types the other deliveries state, in Python's idiom.
"""

from __future__ import annotations

from dataclasses import dataclass
from enum import Enum
from typing import ClassVar, Generic, NewType, TypeVar

__all__ = [
    "AbilityId",
    "AttributeId",
    "Confidence",
    "Curve",
    "Contradicted",
    "CurveValue",
    "Dense",
    "DungeonId",
    "EffectId",
    "Grade",
    "Graded",
    "HeroId",
    "ItemId",
    "Keyframes",
    "Known",
    "MediaHandle",
    "MediaKind",
    "MobId",
    "Origin",
    "Provenance",
    "TalentId",
    "Unassessed",
    "Unresolved",
    "Unvalidated",
    "Validated",
    "Validation",
    "Value",
    "contradiction_delta",
    "is_graded",
    "is_validated",
]

T = TypeVar("T")


@dataclass(frozen=True)
class Dense:
    """One value per difficulty, covering ``1..=len(values)``."""

    values: tuple[float, ...]

    def at(self, difficulty: int) -> float | None:
        """The value at an exact difficulty, when the data states one."""
        if difficulty < 1 or difficulty > len(self.values):
            return None
        return self.values[difficulty - 1]

    def is_empty(self) -> bool:
        return not self.values


@dataclass(frozen=True)
class Keyframes:
    """``(difficulty, value)`` pairs in ascending order."""

    points: tuple[tuple[float, float], ...]

    def at(self, difficulty: int) -> float | None:
        """The value at an exact keyframe.

        Returns ``None`` rather than interpolating between keyframes. How the game interpolates is
        a modelling decision for the caller; guessing it here would manufacture numbers nobody
        measured.
        """
        for at, value in self.points:
            if at == difficulty:
                return value
        return None

    def is_empty(self) -> bool:
        return not self.points


#: A difficulty-indexed curve.
#:
#: Two genuinely different shapes occur, and flattening them together would assert something
#: untrue. Enemy scaling is dense — one value per difficulty from 1 to 151. Hero scaling is a pair
#: of keyframes at the ends of that range, describing a ramp rather than a table.
Curve = Dense | Keyframes


@dataclass(frozen=True)
class Known(Generic[T]):
    """Read directly from the game's data."""

    value: T


@dataclass(frozen=True)
class CurveValue(Generic[T]):
    """Defined by a game curve that has not been resolved.

    ``default`` is the fallback the asset declares — not a measurement.

    Named ``CurveValue`` rather than ``Curve``: the Rust delivery distinguishes ``Value::Curve``
    from the ``Curve`` type by their namespaces, and Python has one namespace here.
    """

    curve_ref: str
    default: T | None = None


@dataclass(frozen=True)
class Unresolved:
    """Known to exist, with no value recovered. Never silently omitted."""

    reason: str


@dataclass(frozen=True)
class Validated:
    """Checked against at least one capture, and it agreed."""


@dataclass(frozen=True)
class Unvalidated:
    """No capture has checked this formula."""

    reason: str


@dataclass(frozen=True)
class Contradicted:
    """A capture checked this formula and disagreed.

    ``delta`` is the observed difference. A contradicted formula still publishes: the marker
    records that two methods disagreed, not which one is wrong.
    """

    reason: str
    delta: float


#: Whether a modelled formula has been checked against a recorded capture.
#:
#: Not a confidence number. "Never checked" and "checked and disagreed" are different states with
#: independent failure modes, and one scalar cannot carry both.
Validation = Validated | Unvalidated | Contradicted


def is_validated(validation: Validation) -> bool:
    """Whether an oracle has agreed with this formula."""
    return isinstance(validation, Validated)


def contradiction_delta(validation: Validation) -> float | None:
    """The observed disagreement, when one was measured."""
    return validation.delta if isinstance(validation, Contradicted) else None


class Grade(Enum):
    """One axis of how much standing an extraction has."""

    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    VERIFIED = "verified"


@dataclass(frozen=True)
class Unassessed:
    """Nothing has graded this extraction.

    Carries a reason rather than being a bottom grade: "nobody assessed this" and "assessed, and
    it is weak" are different claims.
    """

    reason: str


@dataclass(frozen=True)
class Graded:
    """Graded on each axis the extraction actually has.

    ``binding`` is how firmly the value is bound to the thing it claims to describe,
    ``extraction`` how reliably the bytes were read, and ``interpretation`` how well what was read
    is understood to mean what it is published as.
    """

    binding: Grade
    extraction: Grade
    interpretation: Grade


#: How much standing the extraction behind a formula has.
#:
#: A separate axis from :data:`Validation`, not a finer grade of it. Validation asks whether an
#: oracle agreed; this asks how well the value was recovered in the first place. Coverage is a
#: corpus-level summary and is deliberately absent: it is not a property of any one formula.
Confidence = Unassessed | Graded


def is_graded(confidence: Confidence) -> bool:
    """Whether this extraction has been graded at all."""
    return isinstance(confidence, Graded)


#: A value that could not be fully resolved.
#:
#: Collapsing an unresolved value to a plain number turns "we do not know" into "it is 1.0", and
#: the two read identically at the call site. Modelling the gap in the type forces a decision.
Value = Known[T] | CurveValue[T] | Unresolved


def known(value: Value[T]) -> T | None:
    """The value only when it was actually measured."""
    return value.value if isinstance(value, Known) else None


def known_or_declared_default(value: Value[T]) -> T | None:
    """The measured value, or the asset's declared fallback.

    Named the long way to make the compromise visible at the call site: a fallback is what the
    asset says to use, not what the game was observed to do.
    """
    if isinstance(value, Known):
        return value.value
    if isinstance(value, CurveValue):
        return value.default
    return None


class Origin(str, Enum):
    """Where a record came from.

    Kept on every record because a hand-authored correction and a value read from the game must
    never be indistinguishable to a consumer.
    """

    #: Read directly from the game's own data.
    DATAMINE = "datamine"
    #: Computed from other values by a named transform.
    DERIVED = "derived"
    #: Hand-authored to cover a known gap.
    OVERLAY = "overlay"


@dataclass(frozen=True)
class Provenance:
    """Provenance carried alongside a record."""

    origin: Origin
    #: The developer-facing identifier, where one exists.
    dev_name: str | None = None
    #: For :attr:`Origin.DERIVED`, the transform. For :attr:`Origin.OVERLAY`, the reason.
    source: str | None = None

    #: Read directly from the game's own data. Assigned below, because a frozen dataclass cannot
    #: reference itself inside its own body.
    DATAMINE: ClassVar[Provenance]


Provenance.DATAMINE = Provenance(origin=Origin.DATAMINE)


class MediaKind(str, Enum):
    """What a media handle points at.

    A ``str`` enum so its value is the same string the JSON and TypeScript deliveries carry.
    """

    HeroPortrait = "HeroPortrait"
    HeroSelectBackground = "HeroSelectBackground"
    AbilityIcon = "AbilityIcon"
    ItemIcon = "ItemIcon"
    TalentIcon = "TalentIcon"
    MobPortrait = "MobPortrait"
    DungeonMap = "DungeonMap"
    Other = "Other"


@dataclass(frozen=True)
class MediaHandle:
    """A reference to an image this package does not contain.

    A handle carries a stable id and the source dimensions, never a path. Map ``id`` onto wherever
    you serve art from; ids stay stable across refreshes, so asset paths keep working.
    """

    #: Stable across re-exports, so a consumer's own asset paths stay valid.
    id: str
    kind: MediaKind
    width: int
    height: int


# Identifiers are all strings and several namespaces overlap, so each kind is its own type. These
# are `NewType`, which costs nothing at runtime and makes a type checker refuse to pass a hero
# where an ability is expected — the same guarantee the Rust delivery's newtypes give.

#: Identifies a playable hero.
HeroId = NewType("HeroId", str)
#: Identifies an ability or passive.
AbilityId = NewType("AbilityId", str)
#: Identifies an item.
ItemId = NewType("ItemId", str)
#: Identifies an enemy or boss.
MobId = NewType("MobId", str)
#: Identifies a character attribute.
AttributeId = NewType("AttributeId", str)
#: Identifies a gameplay effect.
EffectId = NewType("EffectId", str)
#: Identifies a talent.
TalentId = NewType("TalentId", str)
#: Identifies a dungeon or zone.
DungeonId = NewType("DungeonId", str)
