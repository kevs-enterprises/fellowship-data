# SOURCED FILE — do not edit this copy; it is overwritten on the next data sync.

"""The vocabulary the data is expressed in.

Unlike everything under :mod:`fellowship_data.generated`, this file is hand-written. It states the
same types the other deliveries state, in Python's idiom.
"""

from __future__ import annotations

from dataclasses import dataclass
from enum import Enum
from typing import ClassVar, Generic, NewType, TypeVar
from uuid import UUID

__all__ = [
    "AbilityGuid",
    "AbilityGuidScheme",
    "AbilityId",
    "AbilityRef",
    "AttributeId",
    "CombatConstantId",
    "Curve",
    "CurveValue",
    "Dense",
    "DungeonId",
    "EffectId",
    "FinesseId",
    "HeroId",
    "ItemId",
    "Keyframes",
    "Known",
    "MediaHandle",
    "MediaKind",
    "MobId",
    "Origin",
    "Provenance",
    "RelicPowerId",
    "SetBonusId",
    "TagId",
    "TalentId",
    "TraitId",
    "Unresolved",
    "Value",
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


class SourceAuthority(str, Enum):
    """Strength of the immutable source binding behind a published fact."""

    PRODUCER_BOUND = "producer_bound"
    LEGACY_UNBOUND = "legacy_unbound"
    UNCLASSIFIED = "unclassified"


@dataclass(frozen=True)
class AbilityGuidScheme:
    """The public protocol used to derive Ability GUIDs."""

    name: str
    #: Canonical UUID text. This is a namespace, not an entity identity.
    namespace: str
    version: int
    transform: str
    origin: Origin


@dataclass(frozen=True)
class Provenance:
    """Provenance carried alongside a record."""

    origin: Origin
    #: Acquisition authority of the weakest immutable input behind this record.
    source_authority: SourceAuthority
    #: The developer-facing identifier, where one exists.
    dev_name: str | None = None
    #: For :attr:`Origin.DERIVED`, the transform. For :attr:`Origin.OVERLAY`, the reason.
    source: str | None = None
    #: Read directly from the game's own data. Assigned below, because a frozen dataclass cannot
    #: reference itself inside its own body.
    DATAMINE_UNCLASSIFIED: ClassVar[Provenance]

    @classmethod
    def datamine(cls, source_authority: SourceAuthority) -> Provenance:
        """A datamined record with its acquisition authority stated explicitly."""
        return cls(origin=Origin.DATAMINE, source_authority=source_authority)


Provenance.DATAMINE_UNCLASSIFIED = Provenance(
    origin=Origin.DATAMINE,
    source_authority=SourceAuthority.UNCLASSIFIED,
)


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
#: Stable consumer identity for an Ability, backed by :class:`uuid.UUID`.
AbilityGuid = NewType("AbilityGuid", UUID)
#: Identifies an ability or passive.
AbilityId = NewType("AbilityId", str)


@dataclass(frozen=True)
class AbilityRef:
    """A source Ability occurrence paired with its stable consumer identity."""

    source_id: AbilityId
    guid: AbilityGuid


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
#: Identifies an item trait.
TraitId = NewType("TraitId", str)
#: Identifies an armor-set bonus.
SetBonusId = NewType("SetBonusId", str)
#: Identifies a finesse.
FinesseId = NewType("FinesseId", str)
#: Identifies a relic power.
RelicPowerId = NewType("RelicPowerId", str)
#: Identifies a gameplay tag, as ``<namespace>/<tag>``.
TagId = NewType("TagId", str)
#: Identifies a combat-model constant by its dotted path.
CombatConstantId = NewType("CombatConstantId", str)
