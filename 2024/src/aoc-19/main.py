#!/usr/bin/env -S uv run
import sys
from functools import lru_cache

print(f"input file: {sys.argv[1]}")

with open(sys.argv[1]) as input_file:
    (towels, patterns) = input_file.read().strip().split("\n\n")
    towels = towels.split(", ")
    patterns = patterns.split("\n")


@lru_cache
def find_combinations(pattern):
    valid_patterns = 0

    for towel in towels:
        if pattern.startswith(towel):
            if towel == pattern:
                valid_patterns += 1
            else:
                valid_patterns += find_combinations(pattern[len(towel) :])

    return valid_patterns


print("Part 1:")
results = list(map(lambda p: find_combinations(p), patterns))
result = len(list(filter(lambda r: r != 0, results)))
print(result)

print("Part 2:")
result = sum(results)
print(result)
print(find_combinations.cache_info())
