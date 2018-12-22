#!/usr/env python3


def react(polymerString):
    for i in range(len(polymerString) - 1):
        if polymerString[i] == polymerString[i + 1].swapcase():
            polymerString = polymerString[:i] + "11" + polymerString[i + 2:]
    return polymerString.replace("11", "")


def reactUntilDone(polymerString):
    lastLen = len(polymerString)
    result = react(polymerString)
    while True:
        result = react(result)
        if len(result) == lastLen:
            break
        else:
            lastLen = len(result)
    return result


# Scrape the string from the file
with open("data.txt") as dataFile:
    data = dataFile.readline().replace("\n", "")

# Simulate the reaction
result = reactUntilDone(data)

# Print the length of the result
print(len(result))
