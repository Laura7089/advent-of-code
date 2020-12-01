#!/usr/env python3


def multiReplace(targetStr, target, replacement):
    finished = targetStr
    for char in target:
        finished = finished.replace(char, replacement)
    return finished
