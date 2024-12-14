#!/usr/bin/env -S uv run
import sys
import math


def parse_button(line):
    [_, values] = line.split(": ")
    [x_str, y_str] = values.split(", ")
    x = int(x_str.split("+")[1])
    y = int(y_str.split("+")[1])
    return (x, y)


def parse_prize(line):
    [_, values] = line.split(": ")
    [x_str, y_str] = values.split(", ")
    x = int(x_str.split("=")[1])
    y = int(y_str.split("=")[1])
    return (x, y)


def get_intercept(a_x, a_y, b_x, b_y, p_x, p_y):
    x = (p_y - ((p_x * b_y) / b_x)) / ((a_y / a_x) - (b_y / b_x))

    a_n = round(x / a_x)
    b_n = round((p_x - x) / b_x)

    if a_n * a_x + b_n * b_x == p_x and a_n * a_y + b_n * b_y == p_y:
        return (a_n, b_n)
    else:
        return None


print(f"input file: {sys.argv[1]}")

print("Part 1:")

total = 0
with open(sys.argv[1]) as input_file:
    for machine in input_file.read().strip().split("\n\n"):
        [a, b, p] = machine.split("\n")
        (a_x, a_y) = parse_button(a)
        (b_x, b_y) = parse_button(b)
        (p_x, p_y) = parse_prize(p)
        counts = get_intercept(a_x, a_y, b_x, b_y, p_x, p_y)
        if counts:
            total += 3 * counts[0] + counts[1]

print(total)

print("Part 2:")

total = 0
with open(sys.argv[1]) as input_file:
    for machine in input_file.read().strip().split("\n\n"):
        [a, b, p] = machine.split("\n")
        (a_x, a_y) = parse_button(a)
        (b_x, b_y) = parse_button(b)
        (p_x, p_y) = parse_prize(p)
        to_add = 10000000000000
        counts = get_intercept(a_x, a_y, b_x, b_y, p_x + to_add, p_y + to_add)
        if counts:
            total += 3 * counts[0] + counts[1]

print(total)
