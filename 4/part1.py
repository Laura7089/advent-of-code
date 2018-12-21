#!/usr/env python3
from datetime import datetime, timedelta
from operator import itemgetter


def formatData(dataStr):
    timestamp = datetime(*[int(i) for i in dataStr[1:17].replace("-", " ").replace(":", " ").split(" ")])
    try:
        activity = int(dataStr[19:].split(" ")[1].replace("#", ""))
    except Exception:
        activity = dataStr[19:]
    return (timestamp, activity)


# Scrape the data from the file
with open("data.txt") as dataFile:
    lines = [line.replace("\n", "") for line in dataFile.readlines()]
data = [formatData(line) for line in lines]
# Sort the data by date
data.sort(key=itemgetter(0))

# Generate the dictionary of guards
guards = dict()
for point in data:
    if isinstance(point[1], int):
        currentGuard = point[1]
        try:
            guards[currentGuard].append(list())
        except KeyError:
            guards[currentGuard] = [list()]
    else:
        guards[currentGuard][-1].append(point)

# Change the dictionary so each guard has a list of a list of tuples each showing (sleeptime, waketime)
for guardID, info in guards.items():
    print(info)
    newInfo = list()
    for day in info:
        for i in range(0, len(day), 2):
            newInfo.append([(info[i][0], info[i + 1][0]))
    guards[guardID] = newInfo
