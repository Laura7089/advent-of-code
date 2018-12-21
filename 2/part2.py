#!/usr/env python3

# Scrape data
with open("data.txt") as dataFile:
    data = [point.replace("\n", "") for point in dataFile.readlines()]


def equalLetters(string1, string2):
    equal = ""
    for i in range(len(string1)):
        if string1[i] == string2[i]:
            equal += string1[i]
    return equal


for i in range(len(data)):
    for o in range(len(data[i:])):
        comparison = equalLetters(data[i], data[o])
        if len(comparison) == len(data[0]) - 1:
            print(comparison)
            break
