#!/usr/bin/env -S uv run
import sys
import operator

print(f"input file: {sys.argv[1]}")
print("Part 1:")

directions = [
    (0, -1),  # up
    (1, -1),  # up right
    (1, 0),  # right
    (1, 1),  # down right
    (0, 1),  # down
    (-1, 1),  # down left
    (-1, 0),  # left
    (-1, -1),  # up left
]


def add(a, b):
    return tuple(map(operator.add, a, b))


def sub(a, b):
    return tuple(map(operator.sub, a, b))


def mul(a, m):
    return tuple(map(lambda n: n * m, a))


grid = {}
word = "XMAS"
total = 0
with open(sys.argv[1]) as input_file:
    for y, line in enumerate(input_file):
        for x, c in enumerate(line):
            grid[(x, y)] = c

for direction in directions:
    candidates = grid.keys()
    for idx, c in enumerate(word):
        candidates = list(
            filter(
                lambda xy: grid.get(add(xy, mul(direction, idx)), "") == c,
                candidates,
            )
        )
    total += len(candidates)

print(total)


print("Part 2:")

directions = [
    (1, -1),  # up right
    (1, 1),  # down right
]

grid = {}
total = 0
with open(sys.argv[1]) as input_file:
    for y, line in enumerate(input_file):
        for x, c in enumerate(line):
            grid[(x, y)] = c


def is_center(xy):
    return grid.get(xy, "") == "A"


def is_cross(xy):
    return all(
        map(
            lambda direction: (
                (grid.get(add(xy, direction), "?") in "MS")
                and (grid.get(sub(xy, direction), "?") in "MS")
                and (grid[add(xy, direction)] != grid[sub(xy, direction)])
            ),
            directions,
        )
    )


candidates = list(filter(is_cross, filter(is_center, grid.keys())))

print(len(candidates))
