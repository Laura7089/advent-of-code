#!/usr/bin/env python3
from itertools import product, islice

with open("input.txt", "rt") as data_file:
    int_list = [
        int(val.replace("\n", "")) for val in data_file.readlines()
        if val != "\n"
    ]

for i, o in product(int_list, int_list):
    if i + o == 2020:
        print(f"{i} + {o} = 2020\n{i} * {o} = {i * o}")
