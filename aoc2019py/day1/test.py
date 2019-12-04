#!/usr/bin/env python3
import unittest

from solver import get_fuel_from_mass, get_fuel_recursive


class TestDay1(unittest.TestCase):
    def test_get_fuel_from_mass(self):
        self.assertEqual(
            get_fuel_from_mass(10),
            1,
        )
        self.assertEqual(
            get_fuel_from_mass(1000),
            331,
        )

    def test_get_fuel_recursive(self):
        self.assertEqual(
            get_fuel_recursive(-1),
            0,
        )
        self.assertEqual(
            get_fuel_recursive(3),
            0,
        )
        self.assertEqual(
            get_fuel_recursive(1969),
            966,
        )


if __name__ == "__main__":
    unittest.main()
