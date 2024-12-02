#!/usr/bin/env -S uv run
import sys


def check_diff(diff, diff_0):
    if abs(diff) > 3 or abs(diff) < 1:
        return 0
    elif (diff > 0) != (diff_0 > 0):
        return 0
    return 1


def check_nums(nums, recurse):
    diffs = list(map(lambda pair: pair[1] - pair[0], zip(nums, nums[1:])))

    for idx, diff in enumerate(diffs):
        if not check_diff(diff, diffs[0]):
            if recurse:
                return (
                    check_nums(nums[1:], False)
                    or check_nums(nums[:idx] + nums[idx + 1 :], False)
                    or check_nums(nums[: idx + 1] + nums[idx + 2 :], False)
                )

            else:
                return 0
    return 1


print(f"input file: {sys.argv[1]}")
print("Part 1:")

safe = 0
with open(sys.argv[1]) as input_file:
    for line in input_file:
        nums = list(map(int, line.split()))
        safe += check_nums(nums, False)

print(safe)


print("Part 2:")

safe = 0
with open(sys.argv[1]) as input_file:
    for line in input_file:
        nums = list(map(int, line.split()))
        safe += check_nums(nums, True)

print(safe)
