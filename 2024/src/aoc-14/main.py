#!/usr/bin/env -S uv run
import sys
import operator
import math
import statistics
import itertools


def add(a, b):
    return tuple(map(operator.add, a, b))


def mul(a, m):
    return tuple(map(lambda n: n * m, a))


def mod(a, m):
    if type(m) is tuple:
        return tuple(map(lambda n: n[0] % n[1], zip(a, m)))
    else:
        return tuple(map(lambda n: n % m, a))


def grid_to_string(positions, size):
    grid = list(positions)
    string = ""
    for y in range(size[1]):
        for x in range(size[0]):
            string += "#" if (x, y) in grid else " "

        string += "\n"
    return string


def variance(positions):
    positions_x, positions_y = itertools.tee(positions)
    variance_x = statistics.variance(map(lambda position: position[0], positions_x))
    variance_y = statistics.variance(map(lambda position: position[1], positions_y))
    return variance_x * variance_y


print(f"input file: {sys.argv[1]}")

print("Part 1:")

robots = []
with open(sys.argv[1]) as input_file:
    for line in input_file:
        (position, velocity) = line.strip().split(" ")
        p = tuple(map(int, position.split("=")[1].split(",")))
        v = tuple(map(int, velocity.split("=")[1].split(",")))

        robots.append((p, v))

size = (11, 7) if sys.argv[1].startswith("example") else (101, 103)
steps = 100

positions = list(
    map(lambda robot: (mod(add(robot[0], mul(robot[1], steps)), size)), robots)
)

is_in_quadrant = [
    lambda p: p[0] < int(size[0] / 2) and p[1] < int(size[1] / 2),
    lambda p: p[0] > int(size[0] / 2) and p[1] < int(size[1] / 2),
    lambda p: p[0] < int(size[0] / 2) and p[1] > int(size[1] / 2),
    lambda p: p[0] > int(size[0] / 2) and p[1] > int(size[1] / 2),
]
result = math.prod(
    map(
        len,
        [
            list(filter(is_in_quadrant[0], positions)),
            list(filter(is_in_quadrant[1], positions)),
            list(filter(is_in_quadrant[2], positions)),
            list(filter(is_in_quadrant[3], positions)),
        ],
    )
)
print(result)

print("Part 2:")


def robots_at_step(step):
    return map(
        lambda robot: (mod(add(robot[0], mul(robot[1], step)), size)),
        robots,
    )


def variance_at_step(step):
    return variance(robots_at_step(step))


best_step = min(
    map(
        lambda step: (variance_at_step(step), step),
        range(size[0] * size[1]),
    )
)
print(best_step[1])
print(grid_to_string(robots_at_step(best_step[1]), size))
