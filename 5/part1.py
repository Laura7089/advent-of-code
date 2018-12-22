#!/usr/env python3


def react(polymerString):
    for i in range(len(polymerString) - 1):
        if polymerString[i] == polymerString[i + 1].swapcase():
            polymerString = polymerString[:i] + "11" + polymerString[i + 2:]
    return polymerString.replace("11", "")


# Scrape the string from the file
with open("data.txt") as dataFile:
    data = dataFile.readline().replace("\n", "")

# Set the initial values
lastLen = len(data)
result = react(data)

# Loop until we reach an unchanging state
while True:
    result = react(result)
    # print(result)
    if len(result) == lastLen:
        break
    else:
        lastLen = len(result)

# Print the length of the result
print(len(result))
