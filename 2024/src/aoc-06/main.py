#!/usr/bin/env -S uv run
import sys
import operator


def add(a, b):
    return tuple(map(operator.add, a, b))


def sub(a, b):
    return tuple(map(operator.sub, a, b))


def mul(a, m):
    return tuple(map(lambda n: n * m, a))


def get_path(
    pos,
    direction,
    obstacles,
    max_x,
    max_y,
    path=[],
):
    visited = set(map(lambda visited_direction: visited_direction[0], path))
    visited_directions = set(path)

    while True:
        if (pos, direction) in visited_directions:
            return None

        visited.add(pos)
        visited_directions.add((pos, direction))
        path.append((pos, direction))

        next_pos = add(pos, directions[direction])
        if (
            next_pos[0] < 0
            or next_pos[0] > max_x
            or next_pos[1] < 0
            or next_pos[1] > max_y
        ):
            return (visited, visited_directions, path)
        elif next_pos in obstacles:
            direction = (direction + 1) % 4
        else:
            pos = next_pos


print(f"input file: {sys.argv[1]}")
print("Part 1:")

directions = [
    (0, -1),
    (1, 0),
    (0, 1),
    (-1, 0),
]

direction = None
pos = None
obstacles = set()
max_x = None
max_y = None
with open(sys.argv[1]) as input_file:
    for y, line in enumerate(input_file):
        max_y = y
        for x, c in enumerate(line.strip()):
            max_x = x
            if c == "^":
                pos = (x, y)
                direction = 0
            elif c == "#":
                obstacles.add((x, y))

(visited, visited_directions, path) = get_path(pos, direction, obstacles, max_x, max_y)

print(len(visited))

print("Part 2:")

added_obstacles = set()
for i in range(1, len(path)):
    print(f"{i}/{len(path) - 1}")
    (new_obstacle, _) = path[i]
    (previous_pos, previous_direction) = path[i - 1]
    path_so_far = path[: i - 1]

    if new_obstacle == pos:
        continue

    full_run = False
    if (new_obstacle, 0) in path_so_far:
        full_run = True
    if (new_obstacle, 1) in path_so_far:
        full_run = True
    if (new_obstacle, 2) in path_so_far:
        full_run = True
    if (new_obstacle, 3) in path_so_far:
        full_run = True

    current_obstacles = obstacles.union([new_obstacle])
    if (
        get_path(
            pos if full_run else previous_pos,
            direction if full_run else previous_direction,
            current_obstacles,
            max_x,
            max_y,
            [] if full_run else path_so_far,
        )
        is None
    ):
        added_obstacles.add(new_obstacle)
print(len(added_obstacles))
