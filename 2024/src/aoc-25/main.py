#!/usr/bin/env -S uv run
import sys
import itertools


print(f"input file: {sys.argv[1]}")

print("Part 1:")

grid_height = None
locks = []
keys = []
with open(sys.argv[1]) as input_file:
    for grid in input_file.read().strip().split("\n\n"):
        grid_height = len(grid.strip().split("\n")) - 1
        (first_line, _) = grid.strip().split("\n", 1)
        heights = [0] * len(first_line)

        if all(map(lambda c: c == "#", first_line)):
            for y, line in enumerate(grid.strip().split("\n")):
                for x, c in enumerate(line):
                    if c == "#":
                        heights[x] = y
            locks.append(heights)
        else:
            for y, line in enumerate(reversed(grid.strip().split("\n"))):
                for x, c in enumerate(line):
                    if c == "#":
                        heights[x] = y
            keys.append(heights)

# print(locks)
# print(keys)

result = list(
    filter(
        lambda pairs: not any(
            map(lambda pair: pair[0] + pair[1] >= grid_height, pairs)
        ),
        map(lambda pair: zip(pair[0], pair[1]), itertools.product(locks, keys)),
    )
)
print(len(result))
