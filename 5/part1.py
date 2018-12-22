#!/usr/env python3


def react(polymerString):
    startLen = len(polymerString)
    for i in range(len(polymerString) - 1):
        if abs(ord(polymerString[i]) - ord(polymerString[i + 1])) == 32:
            polymerString = polymerString[:i] + 2 * "\n" + polymerString[i + 2:]
    polymerString = polymerString.replace("\n", "")
    return len(polymerString) == startLen


with open("data.txt") as dataFile:
    data = dataFile.readline()
while True:
    result = react(data)
    print(result)
    if result:
        break

print(len(data))
