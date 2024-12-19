#!/usr/bin/env -S uv run
import sys
import operator
import heapq


def add(a, b):
    return tuple(map(operator.add, a, b))


def grid_to_string(positions, size):
    grid = list(positions)
    string = ""
    for y in range(size[1]):
        for x in range(size[0]):
            string += "#" if (x, y) in grid else "."

        string += "\n"
    return string


def best_paths(positions, size):
    to_check = []
    heapq.heappush(to_check, (0, (0, 0)))
    best = {}
    directions = [
        (0, -1),
        (1, 0),
        (0, 1),
        (-1, 0),
    ]

    def is_inbounds(pos):
        return pos[0] >= 0 and pos[0] < size[0] and pos[1] >= 0 and pos[1] < size[1]

    while len(to_check):
        cost, pos = heapq.heappop(to_check)
        if pos in best and cost >= best[pos]:
            continue
        best[pos] = cost

        for direction in directions:
            new_pos = add(pos, direction)
            if is_inbounds(new_pos) and pos not in positions:
                heapq.heappush(to_check, (cost + 1, new_pos))

    return best


print(f"input file: {sys.argv[1]}")

drops = []
with open(sys.argv[1]) as input_file:
    for line in input_file:
        (x, y) = map(int, line.strip().split(","))
        drops.append((x, y))

size = (7, 7) if sys.argv[1].startswith("example") else (71, 71)
steps = 12 if sys.argv[1].startswith("example") else 1024
goal = (size[0] - 1, size[1] - 1)
# print(grid_to_string(drops[:steps], size))

print("Part 1:")
best = best_paths(drops[:steps], size)
print(best[goal])

print("Part 2:")

upper_bound = len(drops) - 1
lower_bound = 0

while lower_bound < upper_bound - 1:
    steps = lower_bound + int((upper_bound - lower_bound) / 2)
    best = best_paths(drops[:steps], size)
    if goal in best:
        lower_bound = steps
    else:
        upper_bound = steps

print(drops[lower_bound])
