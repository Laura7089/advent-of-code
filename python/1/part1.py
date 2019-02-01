#!/usr/env python3

with open("data.txt") as dataFile:
    data = [int(i.replace("\n", "")) for i in dataFile.readlines()]
print(sum(data))
