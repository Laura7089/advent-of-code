#!/usr/env python3


# Get the data from the file and clean it (get rid of newlines and convert them all to ints)
with open("data.txt") as dataFile:
    data = [int(i.replace("\n", "")) for i in dataFile.readlines()]

totals = [data[0]]
index = 1
while True:
    # Calculate the next total by adding the current data item to the last total
    nextTotal = totals[-1] + data[index % len(data)]
    totals.append(nextTotal)
    # If we've found a repeat, stop looping
    if nextTotal in totals[:-1]:
        break
    index += 1

print(totals[-1])
