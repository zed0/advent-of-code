#!/usr/bin/env -S uv run
import sys
import re


print(f"input file: {sys.argv[1]}")
print("Part 1:")

with open(sys.argv[1]) as input_file:
    matches = re.findall(r"mul\((\d{1,3}),(\d{1,3})\)", input_file.read())
    result = sum(map(lambda nums: int(nums[0]) * int(nums[1]), matches))
    print(result)

print("Part 2:")

with open(sys.argv[1]) as input_file:
    matches = re.findall(
        r"(do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\))", input_file.read()
    )
    active = True
    total = 0
    for instruction in matches:
        if instruction[0] == "do()":
            active = True
        elif instruction[0] == "don't()":
            active = False
        else:
            if active:
                total += int(instruction[1]) * int(instruction[2])
    print(total)
