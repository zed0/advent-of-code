#!/usr/bin/env -S uv run
import sys
import itertools
import math


print(f"input file: {sys.argv[1]}")

print("Part 1:")

decompressed = []
with open(sys.argv[1]) as input_file:
    for pos, c in enumerate(input_file.read().strip()):
        if pos % 2 == 0:
            decompressed += [int(pos / 2)] * int(c)
        else:
            decompressed += ["."] * int(c)

front = 0
back = len(decompressed)

checksum = 0
while front < back:
    if decompressed[front] == ".":
        back -= 1
        while decompressed[back] == "." and front < back:
            back -= 1
        if front < back:
            checksum += front * int(decompressed[back])
    else:
        checksum += front * int(decompressed[front])
    front += 1

print(checksum)


print("Part 2:")

layout = []
to_check = []
with open(sys.argv[1]) as input_file:
    for pos, c in enumerate(input_file.read().strip()):
        if pos % 2 == 0:
            layout.append((int(pos / 2), int(c)))
            to_check.append((int(pos / 2), int(c)))
        else:
            layout.append((".", int(c)))

for id, size in reversed(to_check):
    original_idx = next((i for i, e in enumerate(layout) if e[0] == id), None)
    idx = next((i for i, e in enumerate(layout) if e[0] == "." and e[1] >= size), None)
    if idx is None or idx > original_idx:
        continue

    (_, original_size) = layout[original_idx]
    gap = layout[idx]
    layout[original_idx] = (".", original_size)
    layout[idx : idx + 1] = [(id, size), (".", gap[1] - size)]

decompressed = []
for id, size in layout:
    decompressed += [id] * size

checksum = 0
for pos, c in enumerate(decompressed):
    if c != ".":
        checksum += pos * int(c)

print(checksum)
