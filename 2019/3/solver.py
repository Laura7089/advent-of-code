#!/usr/bin/env python3
import functools
import itertools
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


DIRECTIONS = {
    "U":
    lambda coord, dist: zip(
        itertools.repeat(coord[0]),
        range(
            coord[1] + 1,
            coord[1] + dist + 1,
        ),
    ),
    "D":
    lambda coord, dist: zip(
        itertools.repeat(coord[0]),
        range(
            coord[1] - 1,
            coord[1] - dist - 1,
            -1,
        ),
    ),
    "R":
    lambda coord, dist: zip(
        range(
            coord[0] + 1,
            coord[0] + dist + 1,
        ),
        itertools.repeat(coord[1]),
    ),
    "L":
    lambda coord, dist: zip(
        range(
            coord[0] - 1,
            coord[0] - dist - 1,
            -1,
        ),
        itertools.repeat(coord[1]),
    ),
}


def find_intersections(*wires):
    # Create a list containing (wire sequence, list of coords it crosses) pairs, then populate the coords
    form_wires = [(wire, [(0, 0)]) for wire in wires]
    for wire, coords in form_wires:
        for move in wire:
            coords.extend(DIRECTIONS[move[0]](coords[-1], int(move[1:])))
    coords = list(zip(*form_wires))[1]

    # Delete the first (0,0) coord from all of them
    for coords_group in coords:
        coords_group.pop(0)

    # Create a sequence of dictionaries (coord:lowest time wire crossed it)
    coords = [
        dict(reversed([elem[::-1] for elem in enumerate(x)])) for x in coords
    ]
    # Get a set of coords that they all cross
    uniques = functools.reduce(
        operator.and_,
        (coord_group.keys() for coord_group in coords),
    )
    return {uniq: sum(x[uniq] for x in coords) for uniq in uniques}


def find_closest_intersection(intersections, along_wire=False):
    if not along_wire:
        dists = map(lambda x: abs(x[0]) + abs(x[1]), intersections.keys())
        return min(dists)
    else:
        return min(intersections.values()) + 2


with open("data.txt") as data_file:
    wires = list(map(lambda x: x.split(","), data_file.readlines()))

intersections = find_intersections(*wires)
closest_part1 = find_closest_intersection(intersections)
print(f"{closest_part1=}")

closest_part2 = find_closest_intersection(intersections, along_wire=True)
print(f"{closest_part2=}")

unittest.main()
