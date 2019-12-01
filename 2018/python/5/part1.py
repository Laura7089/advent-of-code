#!/usr/env python3


def react(polymerString):
    for i in range(len(polymerString) - 1):
        if polymerString[i] == polymerString[i + 1].swapcase():
            polymerString = polymerString[:i] + "--" + polymerString[i + 2:]
    return polymerString.replace("--", "")


def reactUntilDone(polymerString):
    lastLen = len(polymerString)
    while True:
        polymerString = react(polymerString)
        if len(polymerString) == lastLen:
            break
        else:
            lastLen = len(polymerString)
    return polymerString


# Scrape the string from the file
with open("data.txt") as dataFile:
    data = dataFile.readline().replace("\n", "")

# Simulate the reaction
result = reactUntilDone(data)

# Print the length of the result
print(len(result))
