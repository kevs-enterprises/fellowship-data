# SOURCED FILE — do not edit here. Edit automation/publish/ in the generator; this copy is overwritten on the next sync.

"""The vocabulary's behaviour, stated the same way every delivery states it.

Run with ``python3 -m unittest discover -s tests`` from the ``python/`` directory. No dependencies.

These mirror the tests the other deliveries carry. ``at`` is hand-written once per delivery, and
"does not interpolate" is the property the whole two-shaped curve exists to protect — so it is the
one most worth checking in each of them independently.
"""

from __future__ import annotations

import unittest

from fellowship_data.types import (
    CurveValue,
    Dense,
    Keyframes,
    Known,
    MediaKind,
    Origin,
    Provenance,
    Unresolved,
    known,
    known_or_declared_default,
)


class CurveTests(unittest.TestCase):
    def test_a_dense_curve_is_indexed_from_difficulty_one(self) -> None:
        curve = Dense((10.0, 20.0, 30.0))
        self.assertEqual(curve.at(1), 10.0)
        self.assertEqual(curve.at(3), 30.0)

    def test_difficulty_zero_and_out_of_range_yield_nothing(self) -> None:
        # Difficulty 0 does not exist, and Python's negative indexing would happily return the last
        # element for it.
        curve = Dense((10.0, 20.0))
        self.assertIsNone(curve.at(0))
        self.assertIsNone(curve.at(-1))
        self.assertIsNone(curve.at(3))

    def test_a_keyframed_curve_answers_only_at_its_keyframes(self) -> None:
        curve = Keyframes(((1.0, 1.0), (151.0, 4.0)))
        self.assertEqual(curve.at(1), 1.0)
        self.assertEqual(curve.at(151), 4.0)

    def test_a_keyframed_curve_does_not_interpolate_between_keyframes(self) -> None:
        # Interpolating here would invent a number nobody measured.
        curve = Keyframes(((1.0, 1.0), (151.0, 4.0)))
        self.assertIsNone(curve.at(75))

    def test_emptiness_is_reported(self) -> None:
        self.assertTrue(Dense(()).is_empty())
        self.assertFalse(Dense((1.0,)).is_empty())
        self.assertTrue(Keyframes(()).is_empty())


class ValueTests(unittest.TestCase):
    def test_a_known_value_reports_itself(self) -> None:
        self.assertEqual(known(Known(2.5)), 2.5)

    def test_a_declared_default_is_not_reported_as_known(self) -> None:
        # The distinction the whole type exists for: a declared fallback is not a measurement.
        value = CurveValue("Bowguy.RangedRange", 1.0)
        self.assertIsNone(known(value))
        self.assertEqual(known_or_declared_default(value), 1.0)

    def test_an_unresolved_value_offers_nothing(self) -> None:
        value = Unresolved("no constants row for this hero")
        self.assertIsNone(known(value))
        self.assertIsNone(known_or_declared_default(value))


class ProvenanceTests(unittest.TestCase):
    def test_the_datamine_constant_carries_no_authored_detail(self) -> None:
        self.assertEqual(Provenance.DATAMINE.origin, Origin.DATAMINE)
        self.assertIsNone(Provenance.DATAMINE.dev_name)
        self.assertIsNone(Provenance.DATAMINE.source)

    def test_enum_values_are_the_strings_the_other_deliveries_carry(self) -> None:
        # A consumer comparing a record read from `json/` against one imported from here must not
        # find two different spellings of the same fact.
        self.assertEqual(Origin.DATAMINE.value, "datamine")
        self.assertEqual(MediaKind.AbilityIcon.value, "AbilityIcon")


class ImmutabilityTests(unittest.TestCase):
    def test_records_cannot_be_mutated(self) -> None:
        # This data is a constant. A consumer that mutated it would be mutating it for every other
        # consumer in the process.
        handle = Dense((1.0,))
        with self.assertRaises(Exception):
            handle.values = ()  # type: ignore[misc]


if __name__ == "__main__":
    unittest.main()
