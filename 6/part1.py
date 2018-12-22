#!/usr/env python3


def nearestCoord(targetGrid, targetX, targetY):
    # If it's called on a coord
    if isinstance(targetGrid[targetX][targetY], int):
        return targetGrid[targetX][targetY]

    # Set vars
    layer = 0
    escape = False
    found = list()
    # Main loop
    while not escape:
        # Increment the layer and set our starting values
        layer += 1
        toCheck = [i for i in range(layer, (-1 * layer) - 1, -1)]
        toCheck.extend(list(range((-1 * layer) + 1, layer)))
        # Generate the coords
        for i in range(layer * 4):
            coords.append((toCheck[(i + layer) % len(toCheck)], toCheck[i]))

        # Check all the coords
        for coord in coords:
            try:
                currentVal = targetGrid[coord[0] + targetX][coord[1] + targetY]
                if isinstance(currentVal, int):
                    found.append(currentVal)
                    escape = True
            except IndexError:
                continue

    # Return the correct string representing the closest coord (or . if there's multiple)
    if len(found) > 1:
        return "."
    else:
        return str(found[0])


def renderGrid(targetGrid):
    gridStr = ""
    for y in range(len(targetGrid[0]) - 1, -1, -1):
        for x in range(len(targetGrid)):
            gridStr += str(targetGrid[x][y]) + " "
        gridStr += "\n"
    return gridStr


# Scrape the data from the file into a list of coordinates (corrected to be 0-based)
with open("data.txt") as coordsFile:
    coords = [[int(coord) - 1 for coord in line.replace("\n", "").replace(",", "").split(" ")] for line in coordsFile.readlines()]

# Generate a list of each axis coordinate
xCoords = [coord[0] for coord in coords]
yCoords = [coord[1] for coord in coords]
# Generate the grid we need to be as large as we need it to be and fill it with "."
grid = [["." for y in range(max(yCoords) + 1)] for x in range(max(xCoords) + 1)]

# Place all the coords
for i in range(len(coords)):
    grid[xCoords[i]][yCoords[i]] = i

# Generate all the areas
for x in range(len(grid)):
    for y in range(len(grid[0])):
        # print("Doing (" + str(x) + ", " + str(y) + ")")
        grid[x][y] = nearestCoord(grid, x, y)

# Count up the areas, marking which ones contact the outside borders
areas = dict()
invalid = list()
for x in range(len(grid)):
    for y in range(len(grid[0])):
        currentVal = str(grid[x][y])
        if currentVal != ".":
            if x == len(grid) - 1 or y == len(grid[0]) - 1:
                invalid.append(currentVal)
            else:
                try:
                    areas[currentVal] += 1
                except KeyError:
                    areas[currentVal] = 1

# Delete all the "bad" areas
for bad in invalid:
    try:
        del(areas[bad])
    except KeyError:
        continue

# Print the largest area's size
print(max(areas.keys()))
