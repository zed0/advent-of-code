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


print(f"input file: {sys.argv[1]}")

print("Part 1:")

with open(sys.argv[1]) as input_file:
    for line in input_file:
