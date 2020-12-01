#!/usr/env python3
import multiprocessing.dummy, re, collections
cpuPool = multiprocessing.dummy.Pool(multiprocessing.cpu_count())


# Scrape the data to a list using multithreading to format it, with elements of this format:
# (id, coordX, coordY, sizeX, sizeY)
with open("data.txt") as dataFile:
    data = cpuPool.map(lambda line: [int(char) for char in re.sub("[^\d]", " ", line).split(" ") if char != ""], dataFile.readlines())

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

# Count up all the Xs using multithreading
crossoverAreas = cpuPool.map(lambda column: collections.Counter(column.values())["X"], grid.values())
print(sum(crossoverAreas))
