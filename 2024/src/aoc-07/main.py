#!/usr/bin/env -S uv run
import sys
import itertools
import operator


def sum_valid(ops):
    valid = 0
    with open(sys.argv[1]) as input_file:
        for line in input_file:
            (goal, nums) = line.split(": ", 1)
            goal = int(goal)
            nums = list(map(int, nums.split(" ")))

            for op_list in itertools.product(ops, repeat=(len(nums) - 1)):
                total = nums[0]
                for op, num in zip(op_list, nums[1:]):
                    total = op(total, num)
                    if total > goal:
                        break

                if total == goal:
                    valid += goal
                    break
    return valid


print(f"input file: {sys.argv[1]}")

print("Part 1:")
ops = [operator.add, operator.mul]
print(sum_valid(ops))


print("Part 2:")
ops = [operator.add, operator.mul, lambda a, b: int(str(a) + str(b))]
print(sum_valid(ops))
