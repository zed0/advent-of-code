#!/usr/bin/env -S uv run
import sys
import operator
import heapq
import functools


def add(a, b):
    return tuple(map(operator.add, a, b))


def grid_to_string(grid, start, end, size):
    string = ""
    for y in range(size[1]):
        for x in range(size[0]):
            if start == (x, y):
                string += "S"
            elif end == (x, y):
                string += "E"
            elif (x, y) in grid:
                string += "#"
            else:
                string += " "

        string += "\n"
    return string


def find_path(grid, start, end, size):
    to_check = []
    heapq.heappush(to_check, (0, start, 1, set()))
    best = {}

    directions = [
        (0, -1),
        (1, 0),
        (0, 1),
        (-1, 0),
    ]

    while len(to_check):
        (score, pos, dir, path) = heapq.heappop(to_check)
        if pos in grid:
            continue

        if (pos, dir) in best and score > best[(pos, dir)][0]:
            continue

        if (pos, dir) not in best or score < best[(pos, dir)][0]:
            best[(pos, dir)] = (score, path | set([pos]))
        else:
            best[(pos, dir)] = (score, best[(pos, dir)][1] | path | set([pos]))

        heapq.heappush(
            to_check, (score + 1, add(pos, directions[dir]), dir, path | set([pos]))
        )  # forward
        heapq.heappush(to_check, (score + 1000, pos, (dir - 1) % 4, path))  # turn left
        heapq.heappush(to_check, (score + 1000, pos, (dir + 1) % 4, path))  # turn right

    min_score = min(
        [
            best[(end, 0)][0],
            best[(end, 1)][0],
            best[(end, 2)][0],
            best[(end, 3)][0],
        ]
    )

    min_paths = functools.reduce(
        lambda x, y: x.union(y),
        map(
            lambda x: x[1],
            filter(
                lambda x: x[0] == min_score,
                [
                    best[(end, 0)],
                    best[(end, 1)],
                    best[(end, 2)],
                    best[(end, 3)],
                ],
            ),
        ),
        set(),
    )

    return (min_score, min_paths)


print(f"input file: {sys.argv[1]}")

grid = set()
start = None
end = None
size = None
with open(sys.argv[1]) as input_file:
    for y, line in enumerate(input_file):
        for x, c in enumerate(line.strip()):
            size = (x + 1, y + 1)
            if c == "#":
                grid.add((x, y))
            elif c == "S":
                start = (x, y)
            elif c == "E":
                end = (x, y)

# print(grid_to_string(grid, start, end, size))
(best_score, path_positions) = find_path(grid, start, end, size)
print("Part 1:")
print(best_score)
print("Part 2:")
print(len(path_positions))
