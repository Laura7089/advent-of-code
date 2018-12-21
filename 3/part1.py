#!/usr/env python3


def multiReplace(targetStr, target, replacement):
    finished = targetStr
    for char in target:
        finished = finished.replace(char, replacement)
    return finished


# Scrape the data to a list with elements of this format:
# (id, coordX, coordY, sizeX, sizeY)
with open("data.txt") as dataFile:
    data = dataFile.readlines()
for i in range(len(data)):
    data[i] = [int(char) for char in multiReplace(multiReplace(data[i], "#@:\n", ""), ",x", " ").split(" ") if char != ""]

# Grid is a two dimensional dict
grid = dict()
for claim in data:
    for x in range(claim[1], claim[1] + claim[3]):
        try:
            grid[x]
        except KeyError:
            grid[x] = dict()
        for y in range(claim[2], claim[2] + claim[4]):
            try:
                grid[x][y]
            except KeyError:
                grid[x][y] = claim[0]
            else:
                grid[x][y] = "X"

total = 0
for column in grid.values():
    for point in column.values():
        if point == "X":
            total += 1
print(total)
