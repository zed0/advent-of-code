#!/usr/bin/env -S uv run
import sys
import re


def hash(step):
    result = 0
    for char in step:
        result += ord(char)
        result *= 17
        result %= 256
    # print(f"{step}: {result}")
    return result


def focussing_power(boxes):
    total = 0
    for box_no, box in enumerate(boxes):
        for lens_no, lens in enumerate(box):
            total += (box_no + 1) * (lens_no + 1) * int(lens[1])
    return total


print(f"input file: {sys.argv[1]}")
print("Part 1:")

with open(sys.argv[1]) as input_file:
    result = sum(map(hash, input_file.read().strip().split(",")))

print(result)

print("Part 2:")

with open(sys.argv[1]) as input_file:
    steps = input_file.read().strip().split(",")
    boxes = [[] for i in range(256)]
    for step in steps:
        label = re.split(r"[=-]", step)[0]
        box_no = hash(label)

        if "=" in step:
            value = re.split(r"[=-]", step)[1]
            try:
                idx = [x[0] for x in boxes[box_no]].index(label)
                boxes[box_no][idx][1] = value
            except ValueError:
                boxes[box_no].append([label, value])
        else:
            boxes[box_no] = list(filter(lambda x: x[0] != label, boxes[box_no]))

    print(focussing_power(boxes))
