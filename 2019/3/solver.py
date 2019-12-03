#!/usr/bin/env python3
import functools
import operator
import unittest


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


DX = {"L": -1, "R": 1, "U": 0, "D": 0}
DY = {"L": 0, "R": 0, "U": 1, "D": -1}


def get_points(wire):
    posx = 0
    posy = 0
    for move in wire:
        dx = DX[move[0]]
        dy = DY[move[0]]
        for increase in range(int(move[1:])):
            posx += dx
            posy += dy
            yield (posx, posy)


def find_intersections(*wires):
    # Create a sequence of dictionaries (coord:lowest time wire crossed it)
    coords = [
        dict([elem[::-1] for elem in enumerate(get_points(wire))][::-1])
        for wire in wires
    ]
    # Get a set of coords that they all cross
    uniques = functools.reduce(
        operator.and_,
        (coord_group.keys() for coord_group in coords),
    )
    return {uniq: sum(x[uniq] for x in coords) for uniq in uniques}


def find_closest_intersection(intersections, along_wire=False):
    if not along_wire:
        return min(abs(x[0]) + abs(x[1]) for x in intersections.keys())
    else:
        return min(intersections.values()) + 2


wires = (line.split(",") for line in open("data.txt").readlines())

intersections = find_intersections(*wires)
closest_part1 = find_closest_intersection(intersections)
print(f"{closest_part1=}")

closest_part2 = find_closest_intersection(intersections, along_wire=True)
print(f"{closest_part2=}")

unittest.main()
