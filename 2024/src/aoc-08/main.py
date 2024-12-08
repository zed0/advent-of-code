#!/usr/bin/env -S uv run
import sys
import itertools
import operator
import math


def add(a, b):
    return tuple(map(operator.add, a, b))


def sub(a, b):
    return tuple(map(operator.sub, a, b))


def mul(a, m):
    return tuple(map(lambda n: n * m, a))


def div(a, m):
    return tuple(map(lambda n: n / m, a))


def is_exact_coordinate(a):
    return float(a[0]).is_integer() and float(a[1]).is_integer()


def is_inside_grid(a, max_x, max_y):
    return a[0] >= 0 and a[0] <= max_x and a[1] >= 0 and a[1] <= max_y


print(f"input file: {sys.argv[1]}")

antennae = {}

max_x = None
max_y = None
with open(sys.argv[1]) as input_file:
    for y, line in enumerate(input_file):
        max_y = y
        for x, c in enumerate(line.strip()):
            max_x = x
            if c != ".":
                if c not in antennae:
                    antennae[c] = set()
                antennae[c].add((x, y))

print("Part 1:")

antinodes = set()
for frequency, locations in antennae.items():
    for pair in itertools.combinations(locations, 2):
        diff = sub(pair[1], pair[0])
        new_antinodes = set(
            filter(
                lambda n: is_inside_grid(n, max_x, max_y) and is_exact_coordinate(n),
                [
                    sub(pair[0], diff),
                    add(pair[0], div(diff, 3)),
                    add(pair[1], diff),
                    sub(pair[1], div(diff, 3)),
                ],
            )
        )

        antinodes = antinodes.union(new_antinodes)

print(len(antinodes))

print("Part 2:")

antinodes = set()
for frequency, locations in antennae.items():
    for pair in itertools.combinations(locations, 2):
        diff = sub(pair[1], pair[0])
        vector = div(diff, math.gcd(diff[0], diff[1]))
        new_antinodes = set()
        next_antinode = pair[0]
        while is_inside_grid(next_antinode, max_x, max_y):
            new_antinodes.add(next_antinode)
            next_antinode = add(next_antinode, vector)
        next_antinode = pair[0]
        while is_inside_grid(next_antinode, max_x, max_y):
            new_antinodes.add(next_antinode)
            next_antinode = sub(next_antinode, vector)

        antinodes = antinodes.union(new_antinodes)

print(len(antinodes))
