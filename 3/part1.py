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
data = [[int(char) for char in multiReplace(multiReplace(line, "#@:\n", ""), ",x", " ").split(" ") if char != ""] for line in data]

# Grid is a two dimensional dict with coords as keys and id's or "X" as values
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

# Count the number of X's in the whole thing
total = 0
for column in grid.values():
    for point in column.values():
        if point == "X":
            total += 1
print(total)
