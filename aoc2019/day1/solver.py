#!/usr/bin/env python3


def get_fuel_from_mass(mass):
    return int(mass / 3) - 2


def get_fuel_recursive(mass):
    required_fuel = int(mass / 3) - 2
    if required_fuel <= 0:
        return 0
    return required_fuel + get_fuel_recursive(required_fuel)


if __name__ == "__main__":
    with open("data.txt") as data_file:
        data = list(map(int, data_file.readlines()))

    part1 = sum(map(get_fuel_from_mass, data))
    print(f"{part1=}")

    part2 = sum(map(get_fuel_recursive, data))
    print(f"{part2=}")
