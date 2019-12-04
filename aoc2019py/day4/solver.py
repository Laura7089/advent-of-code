#!/usr/bin/env python3


def test_valid_pass(password):
    if not isinstance(password, int):
        return False
    password = str(password)
    if len(password) != 6:
        return False
    for i in range(len(password) - 1):
        if password[i] == password[i + 1]:
            break
        return False
    digits = list(map(int, password))
    if sorted(digits) != digits:
        return False
    return True


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
