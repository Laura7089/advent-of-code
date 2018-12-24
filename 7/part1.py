#!/usr/bin/env python3

# Scrape the data from the file into two lists which correspond to one another
with open("data.txt") as dataFile:
    lines = [line.replace("\n", "").split(" ") for line in dataFile.readlines()]
    requisites = [line[1] for line in lines]
    requirers = [line[7] for line in lines]
del lines

# Generate a dictionary of each step's requisites
requirements = {step: list() for step in requisites + requirers}
for i in range(len(requisites)):
    requirements[requirers[i]].append(requisites[i])

# Sort it
requireList = list()
for key, item in requirements.items():
    item.sort()
    # requirements[key] = item
    requireList.append((key, item))
requireList.sort(key=lambda elem: len(elem[1]))

for requirement in requireList:
    print(requirement)
