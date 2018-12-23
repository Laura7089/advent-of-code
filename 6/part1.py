#!/usr/env python3
from multiprocessing.dummy import Pool as ThreadPool
from multiprocessing import cpu_count
from collections import Counter
cpuPool = ThreadPool(cpu_count())


def nearestCoord(targetGrid, targetX, targetY):
    # If it's called on a coord, just return that coord number
    if isinstance(targetGrid[targetX][targetY], int):
        return targetGrid[targetX][targetY]

    # Set vars
    layer = 0
    escape = False
    found = list()
    nums = [1, 0, -1]

    # Main loop
    while not escape:
        layer += 1
        # Generate the coords
        toCheck = nums + list(reversed(nums[1:-1]))
        coords = cpuPool.map(lambda i: (toCheck[(i + layer) % len(toCheck)], toCheck[i]), range(layer * 4))

        # Check all the coords
        for coord in coords:
            try:
                currentVal = targetGrid[coord[0] + targetX][coord[1] + targetY]
                if isinstance(currentVal, int):
                    found.append(currentVal)
                    escape = True
            except IndexError:
                continue

        # Expand our numbers list by one at each end
        nums = [nums[0] + 1] + nums + [nums[-1] - 1]

    # Return the correct string representing the closest coord (or . if there's multiple)
    if len(found) > 1:
        return "."
    else:
        return str(found[0])


# Scrape the data from the file into a list of coordinates (corrected to be 0-based)
with open("data.txt") as coordsFile:
    coords = cpuPool.map(lambda line: [int(coord) - 1 for coord in line.replace("\n", "").replace(",", "").split(" ")], coordsFile.readlines())

# Generate a list of each axis coordinate
xCoords = [coord[0] for coord in coords]
yCoords = [coord[1] for coord in coords]

# Generate the grid we need to be as large as we need it to be and fill it with "."
grid = cpuPool.map(lambda xPos: ["." for y in range(max(yCoords) + 1)], range(max(xCoords) + 1))
# Place all the coords
for i in range(len(xCoords)):
    grid[xCoords[i]][yCoords[i]] = i

# Generate all the areas
for x in range(len(grid)):
    print("Generating line #" + str(x) + " of " + str(len(grid)))
    grid[x] = cpuPool.map(lambda y: nearestCoord(grid, x, y), range(len(grid[0])))

# Count up the areas, marking which ones contact the outside borders
areas = {num: 0 for num in list(range(len(xCoords)))}
for x in range(len(grid)):
    for key, value in Counter(grid[x]).items():
        areas[key] += value
del areas["."]
# Delete all the "bad" areas
# for bad in invalid:
    # del(areas[bad])

# Print the largest area's size
print(max(areas.keys()))
