#!/usr/bin/env -S uv run
import sys
import operator
from functools import lru_cache


def add(a, b):
    return tuple(map(operator.add, a, b))


print(f"input file: {sys.argv[1]}")

print("Part 1:")

heights = {}
with open(sys.argv[1]) as input_file:
    for y, line in enumerate(input_file):
        for x, c in enumerate(line.strip()):
            max_x = x
            max_y = y
            heights[(x, y)] = int(c)

trailheads = [pos for pos, height in heights.items() if height == 0]


@lru_cache
def get_score(coordinate):
    current_height = heights[coordinate]
    if current_height == 9:
        return set([coordinate])

    directions = [
        (-1, 0),
        (0, 1),
        (1, 0),
        (0, -1),
    ]
    return set().union(
        *map(
            get_score,
            filter(
                lambda new_coord: new_coord in heights
                and heights[new_coord] == current_height + 1,
                map(
                    lambda direction: add(coordinate, direction),
                    directions,
                ),
            ),
        )
    )


total = sum(map(lambda trailhead: len(get_score(trailhead)), trailheads))

print(total)


print("Part 2:")


@lru_cache
def get_score_2(coordinate):
    current_height = heights[coordinate]
    if current_height == 9:
        return 1

    directions = [
        (-1, 0),
        (0, 1),
        (1, 0),
        (0, -1),
    ]
    return sum(
        map(
            get_score_2,
            filter(
                lambda new_coord: new_coord in heights
                and heights[new_coord] == current_height + 1,
                map(
                    lambda direction: add(coordinate, direction),
                    directions,
                ),
            ),
        )
    )


total_2 = sum(map(get_score_2, trailheads))

print(total_2)
