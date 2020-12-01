#!/usr/env python3
import multiprocessing.dummy, operator, functools, itertools
from collections import Counter
cpuPool = multiprocessing.dummy.Pool(multiprocessing.cpu_count())

# Scrape the data from the file
with open("data.txt") as dataFile:
    data = cpuPool.map(lambda dataPoint: dataPoint.replace("\n", ""), dataFile.readlines())

# Go through each ID, count the letter frequencies and add them to the frequency list
frequencies = cpuPool.map(lambda num: {freq: None for freq in Counter(num).values() if freq > 1}.keys(), data)
frequencies = list(itertools.chain.from_iterable(frequencies))
print(frequencies)

# Calculate all the totals of the frequencies (excluding the low ones)
totals = [total for total in Counter(frequencies).values() if total > 1]
checksum = functools.reduce(operator.mul, totals, 1)

print(checksum)
