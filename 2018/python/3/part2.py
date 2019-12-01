#!/usr/env python3
import multiprocessing.dummy, re, itertools
cpuPool = multiprocessing.dummy.Pool(multiprocessing.cpu_count())


# Scrape the data to a list with elements of this format:
# (id, (coordX, coordX + sizeX), (coordY, coordY + sizeY))
with open("data.txt") as dataFile:
    data = cpuPool.map(lambda line: [int(char) for char in re.sub("[^\d]", " ", line).split(" ") if char != ""], dataFile.readlines())
data = cpuPool.map(lambda point: (point[0], (point[1], point[1] + point[3]), (point[2], point[2] + point[4])), data)

# Grid is a two dimensional dict, this is probably best as access times are better with dicts and it doesn't need to be populated with empty values at the start
grid = dict()
grid = {
overlaps = {point[0]: False for point in data}
for claim in data:
    for x in range(*claim[1]):
        try:
            grid[x]
        except KeyError:
            grid[x] = dict()

        for y in range(*claim[2]):
            try:
                # If theres already a number there, replace it with an X and set the overlap of both areas to be true
                grid[x][y] = "X"
                overlaps[grid[x][y]] = True
                overlaps[claim[0]] = True
            except KeyError:
                # If not, just add our number there
                grid[x][y] = claim[0]

for key, value in overlaps.items():
    if not value:
        print(key)
