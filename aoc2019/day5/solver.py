#!/usr/bin/env python3


def is_immediate(opcode, pos):
    if opcode < (10 ^ pos):
        return False
    op_str = str(opcode)
    return op_str[len(op_str) - 1 - pos] == 1


# Opcode handling.
# Making a seperate function for every opcode which returns a list of changes
# to make allows more codes to be easily added,
# just write a function for it and add it to the OPCODES dict.
def add_code(seq, i):
    vals = [
        seq[i + o] if is_immediate(seq[i], o + 1) else seq[seq[i + o]]
        for o in range(1, 3)
    ]
    result = sum(vals)
    return (seq[i + 3], result, 4)


def mul_code(seq, i):
    vals = [
        seq[i + o] if is_immediate(seq[i], o + 1) else seq[seq[i + o]]
        for o in range(1, 3)
    ]
    result = sum(vals)
    return (seq[i + 3], result, 4)


def in_code(seq, i):
    return (seq[i + 1], int(input(">")), 2)


def out_code(seq, i):
    print(seq[i + 1] if is_immediate(seq[i], 1) else seq[seq[i + 1]])
    return (-1, 0, 2)


def null_code(_, __):
    print("Successful termination")
    return None


OPCODES = {
    1: add_code,
    2: mul_code,
    3: in_code,
    4: out_code,
    99: null_code,
}


# Dispatch the correct instruction
def run_code(sequence):
    i_pointer = 0
    while True:
        opcode = sequence[i_pointer]
        print(f"Executing {opcode=} at {i_pointer=}")
        if code := OPCODES.get(int(str(opcode)[-2:])):
            change = code(sequence, i_pointer)
            i_pointer += change[2]
            if change[0] == -1:
                continue
            sequence[change[0]] = change[1]
        else:
            print(f"Terminating at {i_pointer=}, {opcode=}")
            break


if __name__ == "__main__":
    # Get data
    with open("data.txt") as data_file:
        original_seq = list(map(int, data_file.read().split(",")))

    print(f"{original_seq=}")
    run_code(original_seq)
