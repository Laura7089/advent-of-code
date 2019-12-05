#!/usr/bin/env python3
import unittest

from solver import find_closest_intersection, find_intersections


class TestMe(unittest.TestCase):
    def test_given1(self):
        self.assertEqual(
            find_closest_intersection(
                find_intersections(
                    [
                        "R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7",
                        "L72"
                    ],
                    ["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"],
                )),
            159,
        )

    def test_given2(self):
        self.assertEqual(
            find_closest_intersection(
                find_intersections(
                    [
                        "R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20",
                        "R33", "U53", "R51"
                    ],
                    [
                        "U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15",
                        "U6", "R7"
                    ],
                )),
            135,
        )

    def test_given3(self):
        self.assertEqual(
            find_closest_intersection(
                find_intersections(
                    [
                        "R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7",
                        "L72"
                    ],
                    ["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"],
                ),
                along_wire=True,
            ),
            610,
        )

    def test_given4(self):
        self.assertEqual(
            find_closest_intersection(
                find_intersections(
                    [
                        "R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20",
                        "R33", "U53", "R51"
                    ],
                    [
                        "U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15",
                        "U6", "R7"
                    ],
                ),
                along_wire=True,
            ),
            410,
        )


if __name__ == "__main__":
    unittest.main()
