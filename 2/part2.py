#!/usr/env python3
import multiprocessing.dummy
cpuPool = multiprocessing.dummy.Pool(multiprocessing.cpu_count())

# Scrape data
with open("data.txt") as dataFile:
    data = cpuPool.map(lambda point: point.replace("\n", ""), dataFile.readlines())


# Return a string of the letters shared between the two arguments, in order
def equalLetters(string1, string2):
    equalLetters = [string1[i] for i in range(len(string1)) if string1[i] == string2[i]]
    return "".join(equalLetters)


# Return the longest output of equalLetters between all combinations of the source string and targets list
def findEqualLetters(source, targets):
    results = cpuPool.map(lambda string: equalLetters(source, string), targets)
    results.sort(key=lambda s: len(s), reverse=True)
    return results[0]


for i in range(len(data)):
    comparison = findEqualLetters(data[i], data[i + 1:])
    if len(comparison) == len(data[0]) - 1:
        print(comparison)
        break
