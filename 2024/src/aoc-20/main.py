#!/usr/bin/env -S uv run
import sys
import operator
import heapq


def add(a, b):
    return tuple(map(operator.add, a, b))


def manhattan_distance(pos, other):
    return abs(pos[0] - other[0]) + abs(pos[1] - other[1])


def grid_to_string(positions, size, start, end):
    grid = list(positions)
    string = ""
    for y in range(size[1]):
        for x in range(size[0]):
            if (x, y) == start:
                string += "S"
            elif (x, y) == end:
                string += "E"
            elif (x, y) in grid:
                string += "#"
            else:
                string += " "

        string += "\n"
    return string


def get_base_route(positions, start, end):
    directions = [
        (0, -1),
        (1, 0),
        (0, 1),
        (-1, 0),
    ]

    best = {}
    to_check = []
    heapq.heappush(to_check, (0, start))

    while len(to_check):
        (cost, pos) = heapq.heappop(to_check)
        if pos in best and best[pos] < cost:
            continue

        best[pos] = cost
        for direction in directions:
            next = add(pos, direction)
            if next not in positions:
                heapq.heappush(to_check, (cost + 1, next))

    return best


def find_time_saves(bests, min_save, max_distance):
    time_saves = 0
    for pos, cost in bests.items():
        time_saves += len(
            list(
                filter(
                    lambda other: other[1] >= min_save,
                    map(
                        lambda other: (
                            other[0],
                            other[1] - cost - manhattan_distance(pos, other[0]),
                        ),
                        filter(
                            lambda other: manhattan_distance(pos, other[0])
                            <= max_distance,
                            bests.items(),
                        ),
                    ),
                )
            )
        )
    return time_saves


print(f"input file: {sys.argv[1]}")

grid = set()
start = None
end = None
with open(sys.argv[1]) as input_file:
    for y, line in enumerate(input_file):
        for x, c in enumerate(line.strip()):
            size = (x + 1, y + 1)
            if c == "S":
                start = (x, y)
            elif c == "E":
                end = (x, y)
            elif c == "#":
                grid.add((x, y))

print(grid_to_string(grid, size, start, end))
bests = get_base_route(grid, start, end)

min_save = 100

print("Part 1:")
time_saves = find_time_saves(bests, 100, 2)
print(time_saves)

print("Part 2:")
time_saves = find_time_saves(bests, 100, 20)
print(time_saves)
