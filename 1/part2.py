#!/usr/env python3


# Get the data from the file and clean it (get rid of newlines and convert them all to ints)
with open("data.txt") as dataFile:
    data = [int(i.replace("\n", "")) for i in dataFile.readlines()]

total = data[0]
totals = {total: True}
index = 1
while True:
    # Calculate the next total by adding the current data item to the last total
    nextTotal = total + data[index % len(data)]
    # If we've found a repeat, stop looping
    try:
        totals[nextTotal]
    except KeyError:
        totals[nextTotal] = True
        total = nextTotal
        index += 1
    else:
        break

print(nextTotal)
