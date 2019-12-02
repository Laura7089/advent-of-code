#!/usr/bin/env python3
import unittest


# Opcode handling.
# Making a seperate function for every opcode which returns a list of changes
# to make allows more codes to be easily added,
# just write a function for it and add it to the OPCODES dict.
def add_code(sequence, i_pointer):
    pos1, pos2, result_pos = sequence[i_pointer + 1:i_pointer + 4]
    result = sequence[pos1] + sequence[pos2]
    return [(result_pos, result)]


def mul_code(sequence, i_pointer):
    pos1, pos2, result_pos = sequence[i_pointer + 1:i_pointer + 4]
    result = sequence[pos1] * sequence[pos2]
    return [(result_pos, result)]


def null_code(sequence, i_pointer):
    return None


OPCODES = {
    1: add_code,
    2: mul_code,
    99: null_code,
}


# Dispatch the correct instruction
def run_code(sequence):
    for i_pointer in range(0, len(sequence), 4):
        try:
            opcode = sequence[i_pointer]
            if changes := OPCODES[opcode](sequence, i_pointer):
                for change in changes:
                    sequence[change[0]] = change[1]
            else:
                break
        except KeyError:
            raise ValueError(f"{opcode=} at {i_pointer=} not recognised")



class TestMe(unittest.TestCase):
    def test_first(self):
        first_test_data = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]
        run_code(first_test_data)
        self.assertEqual(
            [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
            first_test_data,
        )

    def test_other_given(self):
        further_test_data = [
            (
                [1, 0, 0, 0, 99],
                [2, 0, 0, 0, 99],
            ),
            (
                [2, 3, 0, 3, 99],
                [2, 3, 0, 6, 99],
            ),
            (
                [2, 4, 4, 5, 99, 0],
                [2, 4, 4, 5, 99, 9801],
            ),
            (
                [1, 1, 1, 4, 99, 5, 6, 0, 99],
                [30, 1, 1, 4, 2, 5, 6, 0, 99],
            ),
        ]

        for start, result in further_test_data:
            run_code(start)
            self.assertEqual(start, result)


# Get data
with open("data.txt") as data_file:
    original_seq = list(map(int, data_file.read().split(",")))

# Part 1
opcode_sequence = original_seq.copy()
opcode_sequence[1:3] = (12, 2)
run_code(opcode_sequence)
print(f"Part1: {opcode_sequence[0]}")

# Part 2, Bruteforce solution :/
for noun in range(100):
    for verb in range(100):
        seq = original_seq.copy()
        seq[1:3] = (noun, verb)
        run_code(seq)
        if seq[0] == 19690720:
            result = (100 * noun) + verb
            print(f"Part 2: {noun=}, {verb=}, {result=}")

unittest.main()
