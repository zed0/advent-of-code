#!/usr/bin/env -S uv run

import sys
import math


def main(in_string, min_length, max_length):
    grid = in_string.splitlines()

    target = (len(grid[0]) - 1, len(grid) - 1)

    # (x, y, direction, cost)     0 = up, 1 = right, 2 = down 3 = left
    heads = [(0, 0, 1), (0, 0, 2)]

    best = {}

    while len(heads) > 0:
        current_head = heads.pop(0)
        x = current_head[0]
        y = current_head[1]
        direction = current_head[2]
        cost = best.get(current_head, 0)

        for i in range(min_length, max_length + 1):
            new_cost = cost
            if direction == 0:
                new_dirs = [1, 3]
                new_x = x
                new_y = y - i
            elif direction == 1:
                new_dirs = [0, 2]
                new_x = x + i
                new_y = y
            elif direction == 2:
                new_dirs = [1, 3]
                new_x = x
                new_y = y + i
            elif direction == 3:
                new_dirs = [0, 2]
                new_x = x - i
                new_y = y

            if (
                new_x >= 0
                and new_x <= len(grid[0]) - 1
                and new_y >= 0
                and new_y <= len(grid) - 1
            ):
                for j in range(1, i + 1):
                    if direction == 0:
                        new_cost += int(grid[y - j][x])
                    elif direction == 1:
                        new_cost += int(grid[y][x + j])
                    elif direction == 2:
                        new_cost += int(grid[y + j][x])
                    elif direction == 3:
                        new_cost += int(grid[y][x - j])

                for nd in new_dirs:
                    if new_cost >= best.get((new_x, new_y, nd), math.inf):
                        continue

                    new_head = (new_x, new_y, nd)
                    best[(new_x, new_y, nd)] = new_cost
                    heads.append(new_head)

    print(
        min(
            [
                best[(target[0], target[1], 0)],
                best[(target[0], target[1], 1)],
                best[(target[0], target[1], 2)],
                best[(target[0], target[1], 3)],
            ]
        )
    )


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("No input found.")
    else:
        with open(sys.argv[1], "r") as f:
            in_string = f.read().strip()
            print("Part 1")
            main(in_string, 1, 3)
            print("Part 2")
            main(in_string, 4, 10)
