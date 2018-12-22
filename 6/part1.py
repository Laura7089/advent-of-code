#!/usr/env python3

# Scrape the data from the file into a list of coordinates (corrected to be 0-based)
with open("coords.txt") as coordsFile:
    coords = [[int(coord) - 1 for coord in line.replace("\n", "").replace(",", "").split(" ")] for line in coordsFile.readlines()]

# Generate a list of each axis coordinate
xCoords = [coord[0] for coord in coords]
yCoords = [coord[1] for coord in coords]
# Generate the grid we need to be as large as we need it to be
grid = [["." for y in range(max(yCoords) + 1)] for x in range(max(xCoords))]

# Place all the coords
for i in range(len(coords)):
    grid[xCoords[i]][yCoords[i]] = i
