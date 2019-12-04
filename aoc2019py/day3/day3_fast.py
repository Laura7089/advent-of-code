#!/usr/bin/env python3
import functools
import itertools
import operator
import unittest


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
    coords = [[(0, 0)] for wire in wires]
    for i, wire in enumerate(wires):
        path = coords[i]
        for move in wire:
            path.extend(DIRECTIONS[move[0]](path[-1], int(move[1:])))

    # Delete the first (0,0) coord from all of them
    for coords_group in coords:
        del coords_group[0]

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


if __name__ == "__main__":
    wires = (x.split(",") for x in open("data/day3.txt").readlines())

    intersections = find_intersections(*wires)
    closest_part1 = find_closest_intersection(intersections)
    print(f"{closest_part1=}")

    closest_part2 = find_closest_intersection(intersections, along_wire=True)
    print(f"{closest_part2=}")
