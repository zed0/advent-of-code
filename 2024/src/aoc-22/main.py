#!/usr/bin/env -S uv run
import sys


def generate_next(secret):
    secret = (secret ^ secret * 64) % 16777216
    secret = (secret ^ int(secret / 32)) % 16777216
    secret = (secret ^ secret * 2048) % 16777216
    return secret


print(f"input file: {sys.argv[1]}")

seeds = []
with open(sys.argv[1]) as input_file:
    for line in input_file:
        seeds.append(int(line.strip()))

print("Part 1:")

total = 0
for seed in seeds:
    for _ in range(2000):
        seed = generate_next(seed)
    total += seed
print(total)

print("Part 2:")

records = {}
used = set()

for idx, seed in enumerate(seeds):
    s1 = None
    s2 = None
    s3 = None
    s4 = None
    for _ in range(2000):
        last = seed
        seed = generate_next(seed)
        s1 = s2
        s2 = s3
        s3 = s4
        s4 = (seed % 10) - (last % 10)
        if s1 is not None:
            if ((s1, s2, s3, s4), idx) in used:
                continue
            used.add(((s1, s2, s3, s4), idx))

            if (s1, s2, s3, s4) not in records:
                records[(s1, s2, s3, s4)] = 0
            records[(s1, s2, s3, s4)] += seed % 10
best = max(records.items(), key=lambda i: i[1])
print(best[1])
