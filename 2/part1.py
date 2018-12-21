#!/usr/env python3
from copy import deepcopy

# Constants
with open("data.txt") as dataFile:
    data = [dataPoint.replace("\n", "") for dataPoint in dataFile.readlines()]
alphDict = {char: 0 for char in "abcdefghijklmnopqrstuvwxyz"}

totals = {i: 0 for i in range(2, len(data[0]) + 1)}
for num in data:
    # Copy the letter dictionary
    freqDict = deepcopy(alphDict)
    for letter in num:
        freqDict[letter] += 1
    # Extremely ugly, but generate a dictionary of the frequencies to remove duplicates & irrelevant values, and reduce access time
    hashTable = {i: True for i in freqDict.values() if i > 1}
    for i in hashTable.keys():
        totals[i] += 1

# Generate the checksum by multiplying together all the non-zero values in the totals dict
checksum = 1
for i in totals.values():
    if i > 1:
        checksum *= i

# Output the result
print(checksum)
