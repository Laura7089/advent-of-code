#!/usr/env python3

data = []
with open("data.txt") as dataFile:
    data = [i.replace("\n", "") for i in dataFile.readlines()]

total = 0
for point in data:
    total += int(point)
print(total)
