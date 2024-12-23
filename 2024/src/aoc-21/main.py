#!/usr/bin/env -S uv run
import sys
import operator
import math
from functools import lru_cache
import itertools


def add(a, b):
    return tuple(map(operator.add, a, b))


@lru_cache
def len_by_group(x):
    return len(list(itertools.groupby(x)))


directions = {
    "^": (0, -1),
    ">": (1, 0),
    "v": (0, 1),
    "<": (-1, 0),
}


def create_paths(grid):
    characters = {}
    positions = {}
    for y, line in enumerate(grid.split("\n")):
        for x, c in enumerate(line):
            if c != " ":
                characters[c] = (x, y)
                positions[(x, y)] = c

    paths = {}
    for character in characters:
        to_check = [([], characters[character])]
        while len(to_check):
            (path, pos) = to_check.pop(0)
            if pos not in positions:
                continue

            pair = (character, positions[pos])
            path_str = "".join(path)
            group_len = len_by_group(path_str)
            if pair not in paths:
                paths[pair] = [path_str]
            elif len_by_group(paths[pair][0]) > group_len:
                paths[pair] = [path_str]
            elif len_by_group(paths[pair][0]) == group_len:
                paths[pair].append(path_str)
            else:
                continue

            for direction, vec in directions.items():
                to_check.append((path + [direction], add(pos, vec)))

    return paths


print(f"input file: {sys.argv[1]}")

codes = []
with open(sys.argv[1]) as input_file:
    for line in input_file:
        codes.append(line.strip())

# +---+---+---+
# | 7 | 8 | 9 |
# +---+---+---+
# | 4 | 5 | 6 |
# +---+---+---+
# | 1 | 2 | 3 |
# +---+---+---+
#     | 0 | A |
#     +---+---+
numeric_grid = """
789
456
123
 0A
"""
numeric_paths = create_paths(numeric_grid)
# print(numeric_paths)

#     +---+---+
#     | ^ | A |
# +---+---+---+
# | < | v | > |
# +---+---+---+
direction_grid = """
 ^A
<v>
"""
direction_paths = create_paths(direction_grid)

best_paths = [
    [numeric_paths, direction_paths, direction_paths],
    [numeric_paths] + 25 * [direction_paths],
]


@lru_cache
def length_for_pair(pair, path_index, depth=0):
    paths = best_paths[path_index]
    best = math.inf
    for path in paths[depth][pair]:
        path = path + "A"
        if depth == len(paths) - 1:
            return len(path)

        total = 0
        for a, b in zip("A" + path, path):
            total += length_for_pair((a, b), path_index, depth + 1)
        if total < best:
            best = total

    return best


print("Part 1:")
total = 0
for code in codes:
    button_presses = 0
    for a, b in zip("A" + code, code):
        button_presses += length_for_pair((a, b), 0)
    total += int(code[:-1]) * button_presses
print(total)

print("Part 2:")
total = 0
for code in codes:
    button_presses = 0
    for a, b in zip("A" + code, code):
        button_presses += length_for_pair((a, b), 1)
    total += int(code[:-1]) * button_presses
print(total)
