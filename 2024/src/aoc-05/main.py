#!/usr/bin/env -S uv run
import sys


def find_invalid_index(update_nums, ordermap):
    forbidden = set()
    for idx, page in enumerate(update_nums):
        if page in forbidden:
            return idx
        if page in ordermap:
            forbidden = forbidden.union(ordermap[page])
    return None


def sum_middle_elements(updates):
    return sum(
        map(
            lambda update_nums: update_nums[int(len(update_nums) / 2)],
            updates,
        )
    )


print(f"input file: {sys.argv[1]}")

with open(sys.argv[1]) as input_file:
    (orders, updates) = input_file.read().split("\n\n", 1)
    ordermap = {}
    for order in orders.splitlines():
        (first, second) = map(int, order.split("|"))
        if second not in ordermap:
            ordermap[second] = set()
        ordermap[second].add(first)

    valid = []
    invalid = []
    for update in updates.splitlines():
        update_nums = list(map(int, update.split(",")))
        if find_invalid_index(update_nums, ordermap) is None:
            valid.append(update_nums)
        else:
            while True:
                idx = find_invalid_index(update_nums, ordermap)
                if idx is None:
                    invalid.append(update_nums)
                    break

                (update_nums[idx - 1], update_nums[idx]) = (
                    update_nums[idx],
                    update_nums[idx - 1],
                )

    print("Part 1:")
    print(sum_middle_elements(valid))

    print("Part 2:")
    print(sum_middle_elements(invalid))
