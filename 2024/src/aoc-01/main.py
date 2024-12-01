#!/usr/bin/env -S uv run
import sys

print(f"input file: {sys.argv[1]}")
print("Part 1:")
list_a = []
list_b = []

with open(sys.argv[1]) as input_file:
    for line in input_file:
        (a, b) = line.split()
        list_a.append(int(a))
        list_b.append(int(b))

list_a.sort()
list_b.sort()

result = sum(map(lambda ab: abs(ab[0] - ab[1]), zip(list_a, list_b)))
print(result)


print("Part 2:")
list_a = []
list_b = {}

with open(sys.argv[1]) as input_file:
    for line in input_file:
        (a, b) = line.split()
        list_a.append(a)
        if b not in list_b:
            list_b[b] = 0
        list_b[b] += 1

result = sum(map(lambda a: int(a) * list_b.get(a, 0), list_a))
print(result)
