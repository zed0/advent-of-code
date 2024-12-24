#!/usr/bin/env -S uv run
import sys


def find_triplets(links):
    triplets = set()
    for a, a_links in links.items():
        if not a.startswith("t"):
            continue
        for b in a_links:
            for c in links[b]:
                for a_candidate in links[c]:
                    if a_candidate == a:
                        triplets.add(tuple(sorted([a, b, c])))
    return triplets


def find_more_connected(existing, links):
    next_layer = set()
    for current in existing:
        for node in current:
            for other_node in links[node]:
                if current & links[other_node] == current:
                    next = frozenset(current | set([other_node]))
                    next_layer.add(next)

    return next_layer


print(f"input file: {sys.argv[1]}")

links = {}
with open(sys.argv[1]) as input_file:
    for line in input_file:
        a, b = line.strip().split("-")
        if a not in links:
            links[a] = set()
        if b not in links:
            links[b] = set()
        links[a].add(b)
        links[b].add(a)

print("Part 1:")

triplets = find_triplets(links)
print(len(triplets))

print("Part 2:")

fully_connected = set([frozenset([link]) for link in links])
count = 1
while True:
    print(count, len(fully_connected))
    count += 1
    next = find_more_connected(fully_connected, links)
    if len(next) == 0:
        break
    fully_connected = next

result = ",".join(sorted([list(n) for n in fully_connected][0]))
print(result)
