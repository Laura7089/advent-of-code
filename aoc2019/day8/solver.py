#!/usr/bin/env python3
from collections import Counter
from itertools import chain

from aocd import get_data

PIXELS = [" ", "█"]


def get_layers(data, width=25, height=6):
    data = list(map(int, data))
    layers = [data[i:i + width] for i in range(0, len(data), width)]
    return [layers[i:i + height] for i in range(0, len(layers), height)]


def part1(layers):
    count = sorted(map(Counter, map(chain.from_iterable, layers)),
                   key=lambda x: x[0])[0]
    return count[1] * count[2]


def part2(layers, width=25, height=6):
    final = [[0 for o in range(width)] for i in range(height)]
    for x in range(width):
        for y in range(height):
            for layer in layers:
                pixel = layer[y][x]
                if pixel == 2:
                    continue
                else:
                    final[y][x] = PIXELS[pixel]
                    break
    return "\n".join(map(lambda x: "".join(x), final))


if __name__ == "__main__":
    layers = get_layers(get_data(year=2019, day=8))
    print(f"part1={part1(layers)}")
    print(f"part2=\n{part2(layers)}")
