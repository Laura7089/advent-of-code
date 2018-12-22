#!/usr/env python3
from datetime import datetime
from operator import itemgetter
from common import multiReplace


def formatData(dataStr):
    timestamp = datetime(*[int(i) for i in multiReplace(dataStr[1:17], "-:", " ").split(" ")])
    activityStr = dataStr[19:]
    try:
        activity = int(activityStr.split(" ")[1].replace("#", ""))
    except Exception:
        activity = activityStr
    return (timestamp, activity)


# Scrape the data from the file, format it and sort it
with open("data.txt") as dataFile:
    data = [formatData(line.replace("\n", "")) for line in dataFile.readlines()]
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
        guards[currentGuard][-1].append(point[0])

# Change the dictionary so each guard has a list of days each of which is a list of timestamps
for guardID, info in guards.items():
    newInfo = list()
    for day in info:
        daySeq = list()
        sleepTime = 
        for event in day:
            daySeq.append(event)
        for i in range(0, len(day), 2):

        newInfo.append(daySeq)

    guards[guardID] = newInfo
