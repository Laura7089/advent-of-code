#!/usr/bin/env python3
from collections import Counter


def test_valid_pass(password):
    # Check it's a 6-digit integer
    if not isinstance(password, int):
        return False
    password = str(password)
    if len(password) != 6:
        return False

    # Check ordering and double-digit
    double = False
    for i in range(len(password) - 1):
        if password[i] <= password[i + 1]:
            if not double and password[i] == password[i + 1]:
                double = True
        else:
            return False
    if not double:
        return False

    return True


def test_valid_strict(password):
    if not test_valid_pass(password):
        return False

    counter = Counter(str(password))
    if 2 in counter.values():
        return True
    return False


if __name__ == "__main__":
    wanted_range = [
        int(x) for x in open("data.txt", "rt").readline().split("-")
    ]
    wanted_range[1] += 1
    print(f"{wanted_range=}")

    part1_valid = list(filter(test_valid_pass, range(*wanted_range)))
    # print(f"{part1_valid=}")
    part1 = len(part1_valid)
    print(f"{part1=}")

    part2_valid = list(filter(test_valid_strict, range(*wanted_range)))
    part2 = len(part2_valid)
    print(f"{part2=}")
