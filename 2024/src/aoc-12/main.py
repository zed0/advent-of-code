#!/usr/bin/env -S uv run
import sys
import operator


def add(a, b):
    return tuple(map(operator.add, a, b))


print(f"input file: {sys.argv[1]}")

print("Part 1:")

grid = {}
with open(sys.argv[1]) as input_file:
    for y, line in enumerate(input_file):
        for x, c in enumerate(line.strip()):
            max_x = x
            max_y = y
            grid[(x, y)] = {"color": c, "marked": False}

directions = [
    (0, -1),
    (1, 0),
    (0, 1),
    (-1, 0),
]


def visit(coord, record):
    if grid[coord]["marked"]:
        return
    grid[coord]["marked"] = True
    record["members"].append(coord)
    record["size"] += 1

    for direction in directions:
        next_coord = add(coord, direction)

        if next_coord in grid and grid[next_coord]["color"] == grid[coord]["color"]:
            visit(next_coord, record)
        else:
            record["edges"] += 1
            record["permitter"].add(next_coord)


def get_sides(record):
    permitters = sorted(record["permitter"])
    sides = 0
    while len(permitters):
        current_dir = 0
        start_pos = permitters.pop(0)
        current_pos = start_pos
        direction_to_region = (
            "left" if add(start_pos, (-1, 0)) in record["members"] else "right"
        )

        while True:
            try:
                permitters.remove(current_pos)
            except ValueError:
                pass
            away = (
                (current_dir + 1) % 4
                if direction_to_region == "left"
                else (current_dir - 1) % 4
            )
            towards = (
                (current_dir - 1) % 4
                if direction_to_region == "left"
                else (current_dir + 1) % 4
            )
            straight_pos = add(current_pos, directions[current_dir])
            towards_pos = add(current_pos, directions[towards])

            if towards_pos not in record["members"]:
                sides += 1
                current_dir = towards
                current_pos = towards_pos
            elif straight_pos in record["members"]:
                sides += 1
                current_dir = away
            else:
                current_pos = straight_pos

            if current_pos == start_pos and current_dir == 0:
                break

    return sides


regions = []

for y in range(max_y + 1):
    for x in range(max_x + 1):
        if not grid[(x, y)]["marked"]:
            record = {
                "color": grid[(x, y)]["color"],
                "start": (x, y),
                "edges": 0,
                "permitter": set(),
                "size": 0,
                "members": [],
            }
            visit((x, y), record)
            regions.append(record)

result = sum(map(lambda region: region["size"] * region["edges"], regions))
print(result)

print("Part 2:")
result = sum(map(lambda region: region["size"] * get_sides(region), regions))
print(result)
