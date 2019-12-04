#!/usr/bin/env python3
import functools
import operator

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


if __name__ == "__main__":
    wires = (line.split(",") for line in open("data.txt").readlines())

    intersections = find_intersections(*wires)
    closest_part1 = find_closest_intersection(intersections)
    print(f"{closest_part1=}")

    closest_part2 = find_closest_intersection(intersections, along_wire=True)
    print(f"{closest_part2=}")
