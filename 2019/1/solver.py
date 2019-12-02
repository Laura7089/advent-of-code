#!/usr/bin/env python3
import unittest


class TestMe(unittest.TestCase):
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


def get_fuel_from_mass(mass):
    return int(mass / 3) - 2


def get_fuel_recursive(mass):
    required_fuel = int(mass / 3) - 2
    if required_fuel <= 0:
        return 0
    return required_fuel + get_fuel_recursive(required_fuel)


with open("data.txt") as data_file:
    data = list(map(int, data_file.readlines()))

part1 = sum(map(get_fuel_from_mass, data))
print(f"{part1=}")

part2 = sum(map(get_fuel_recursive, data))
print(f"{part2=}")

unittest.main()
