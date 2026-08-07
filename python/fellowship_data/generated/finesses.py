"""GENERATED FILE - do not edit by hand.

Rendered from the published `finesses.json`. Edits are overwritten on the next refresh.
"""

from __future__ import annotations

from dataclasses import dataclass

from ..types import FinesseId, MediaHandle, MediaKind, Provenance


@dataclass(frozen=True)
class Finesse:
    """A finesse: a chosen passive that alters how a hero's abilities behave."""

    id: FinesseId
    name: str
    description: str
    granted_abilities: tuple[str, ...]
    #: Abilities this finesse grants.
    #:
    #: These name finesse passives, which `abilities.json` does not carry — it publishes hero
    #: abilities only. Recorded as a scope gap in the integrity check rather than a broken
    #: reference.
    icon: MediaHandle | None
    provenance: Provenance


#: Every finesse, sorted by `id`.
FINESSES: tuple[Finesse, ...] = (
    Finesse(id=FinesseId("CAA_Finesse_BasicToAoe"), name="The Vehement", description="Every <rt.bold>6 / 5 / 4 / 3</> <rt.absorb>BASIC</> ability causes your target to erupt, dealing the damage again to all enemies in a radius around the target as poison damage. \n\nThe <rt.warning>damage is increased</> by a percentage equal to your <rt.warning>Critical Strike chance %</>, up to a <rt.warning>maximum of {critthreshold}</>.", granted_abilities=("GA_Finesse_Passive_BasicToAoe_C",), icon=None, provenance=Provenance.DATAMINE),
    Finesse(id=FinesseId("CAA_Finesse_BasicToIncreasedPower"), name="The Intrepid", description="", granted_abilities=("GA_Finesse_Passive_BasicToIncreasedPower_C",), icon=None, provenance=Provenance.DATAMINE),
    Finesse(id=FinesseId("CAA_Finesse_CritChanceToHealOrDamage"), name="The Usurper", description="Your <rt.absorb>CORE</> abilities have a chance equal to your Critical Strike chance % to trigger <rt.effect>The Usurper's Verdict</> on the target, either dealing <rt.warning>143% / 228% / 365% / 584% Main Stat damage</> to up to <rt.warning>4 enemies</> near your target, or healing up to <rt.heal>4 allies</> for <rt.heal>122% / 195% / 312% / 500% Main Stat health</>.\n \n<rt.effect>The Usurper's Verdict</> has a 20% chance to trigger both the <rt.warning>Damage</> and <rt.heal>Healing</> effect.", granted_abilities=("GA_Finesse_Passive_CritChanceToHealOrDamage_C",), icon=None, provenance=Provenance.DATAMINE),
    Finesse(id=FinesseId("CAA_Finesse_CurrentSpiritToBuff"), name="The Philosopher", description="Your <rt.absorb>[MAJOR]</> ability grants you <rt.warning>+{increase} Haste, Expertise and Critical Strike</> for every <rt.bold>4.0% / 2.5% / 1.6% / 1.0% Spirit</> you have for <rt.bold>8 seconds</>, up to a maximum of 10% to each.", granted_abilities=("GA_Finesse_Passive_CurrentSpiritToBuff_C",), icon=None, provenance=Provenance.DATAMINE),
    Finesse(id=FinesseId("CAA_Finesse_DrainHealthBasedOnPrimaryStat"), name="The Heretic", description="Each time you use a <rt.absorb>[CORE]</> ability, there is a <rt.bold>{chance} chance</> you drain <rt.warning>325% / 520% / 832% / 1331% Main Stat</> health from the nearest enemy, <rt.heal>healing you for the same amount</>.", granted_abilities=("GA_Finesse_Passive_DrainHealthBasedOnPrimaryStat_C",), icon=None, provenance=Provenance.DATAMINE),
    Finesse(id=FinesseId("CAA_Finesse_IncreasedBasicDamage"), name="The Vainglorious", description="", granted_abilities=("GA_Finesse_Passive_IncreasedBasicDamage_C",), icon=None, provenance=Provenance.DATAMINE),
    Finesse(id=FinesseId("CAA_Finesse_IncreasedPowerPerCurrentSpirit"), name="The Mystic", description="Your <rt.absorb>POWER</> abilities gain <rt.warning>0.3% / 0.5% / 0.8% / 1.2% increased power</> for every <rt.bold>{spiritpercentage}% Spirit</> you have, up to {MaxSpiritPercentage} Spirit.", granted_abilities=("GA_Finesse_Passive_IncreasedPowerPerCurrentSpirit_C",), icon=None, provenance=Provenance.DATAMINE),
    Finesse(id=FinesseId("CAA_Finesse_MajorIncreasedCooldownRecoveryPerHaste"), name="The Monarch", description="Your <rt.absorb>MAJOR</> abilities gain <rt.warning>3% / 5% / 8% / 12% Cooldown Acceleration</>. The bonus is <rt.bold>increased by +0.1%</> for every <rt.bold>1% Haste</> you have, up to a maximum of 5%.", granted_abilities=(), icon=None, provenance=Provenance.DATAMINE),
    Finesse(id=FinesseId("CAA_Finesse_MajorToReducedControlCooldown"), name="The Subduer", description="You deal <rt.warning>5% / 10% / 15% / 20%</> more damage to <rt.bold>Low Health Enemies</>.\n\nYour <rt.absorb>[MAJOR]</> abilities reduce the cooldown of one of your <rt.absorb>[CONTROL]</> abilities by up to <rt.bold>1 / 2 / 3 / 4 seconds</>. The amount is relative to the default cooldown of the <rt.absorb>MAJOR</> ability used.", granted_abilities=("GA_Finesse_Passive_MajorToReducedControlCooldown_C",), icon=None, provenance=Provenance.DATAMINE),
    Finesse(id=FinesseId("CAA_Finesse_PeriodicHasteBuff"), name="The Wayfarer", description="Every {cooldown} you gain <rt.effect>{EffectName}</>, granting you <rt.warning>+6% / +10% / +16% / +25% Haste</> for {duration}. Each time you use your <rt.absorb>[CORE]</> ability, <rt.bold>the time to</> <rt.effect>{EffectName}'s</> <rt.bold>next application is reduced by 0.2 to 8 seconds</>, relative to the cooldown of the <rt.absorb>[CORE]</>ability used. Abilities with no cooldown provide 0.2 seconds reduction.", granted_abilities=("GA_Finesse_Passive_PeriodicHasteBuff_Manager_C",), icon=None, provenance=Provenance.DATAMINE),
    Finesse(id=FinesseId("CAA_Finesse_PowerChanceSpirit"), name="The Celestial", description="Your <rt.absorb>[POWER]</> abilities have a <rt.bold>5% / 8% / 13% / 20%</> chance to generate <rt.bold>1 Spirit Point</> toward your <rt.absorb>SPIRIT</> ability.", granted_abilities=("GA_Finesse_Passive_PowerChanceSpirit_C",), icon=None, provenance=Provenance.DATAMINE),
    Finesse(id=FinesseId("CAA_Finesse_PowerIncreasedCritChanceAndPower"), name="The Sinister", description="", granted_abilities=(), icon=None, provenance=Provenance.DATAMINE),
    Finesse(id=FinesseId("CAA_Finesse_PowerToIncreasedCritChance"), name="The Trickster", description="", granted_abilities=("GA_Finesse_Passive_PowerToIncreasedCritChance_C",), icon=None, provenance=Provenance.DATAMINE),
    Finesse(id=FinesseId("CAA_Finesse_StartWithSpiritPoints"), name="The Herald", description="You start dungeons with  <rt.warning>12 / 20 / 30 / 50 Spirit Points</> toward your <rt.absorb>SPIRIT</> ability.", granted_abilities=("GA_Finesse_Passive_StartWithSpiritPoints_C",), icon=None, provenance=Provenance.DATAMINE),
)
