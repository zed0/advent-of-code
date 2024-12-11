#!/usr/bin/env -S uv run
import sys

cache = {}


def cached_get_sequence(num, remaining_steps, depth=0):
    if num in cache and remaining_steps in cache[num]:
        return cache[num][remaining_steps]

    if remaining_steps == 0:
        result = 1
    else:
        result = sum(
            map(
                lambda n: cached_get_sequence(n, remaining_steps - 1, depth + 1),
                next_in_sequence(num),
            )
        )

    if num not in cache:
        cache[num] = {}
    cache[num][remaining_steps] = result

    return result


def next_in_sequence(num):
    if num == 0:
        return [1]
    elif len(str(num)) % 2 == 0:
        divisor = 10 ** int(len(str(num)) / 2)
        return [
            int(num / divisor),
            int(num % divisor),
        ]
    else:
        return [num * 2024]


print(f"input file: {sys.argv[1]}")

print("Part 1:")

nums = None
with open(sys.argv[1]) as input_file:
    nums = list(
        map(
            int,
            input_file.read().strip().split(),
        )
    )

result = sum(map(lambda n: cached_get_sequence(n, 75), nums))
print(result)
