#!/usr/bin/env python3
import numpy as np


def parse_grid(path: str) -> ...:
    lines = open(path).read().splitlines()
    chars = np.array(list(map(list, lines)), dtype="|S1")
    sx, sy = map(int, np.where(chars == b"S"))
    tx, ty = map(int, np.where(chars == b"E"))
    chars[sx, sy] = b"a"
    chars[tx, ty] = b"z"
    grid = np.pad(chars.view(np.uint8) - 96, 1, constant_values=99)
    return grid, (sx + 1, sy + 1), (tx + 1, ty + 1)


def part1(grid, start: tuple[int, int], end: tuple[int, int]) -> int:
    min_path = np.full_like(grid, 500, dtype=np.int32)
    queue = [(start, 0)]  # type: list[tuple[tuple[int, int], int]]
    while queue:
        (x, y), steps = queue.pop()
        if min_path[x, y] < steps:
            continue
        min_path[x, y] = steps
        if (x, y) == end:
            # New best path found, but keep searching for the shortest path.
            continue
        curr = grid[x, y]
        for dx, dy in ((0, 1), (0, -1), (1, 0), (-1, 0)):
            pos = (x + dx, y + dy)
            if (
                grid[pos] <= curr + 1
                and min_path[pos] > steps + 1
                and min_path[end] > steps + 1
            ):
                queue.append((pos, steps + 1))
    return int(min_path[end])


def part2(grid, end: tuple[int, int], best: int) -> int:
    min_path = np.full_like(grid, best + 1, dtype=np.int32)
    # Work backwards from the end to any grid cell with value 1.
    queue = [(end, 0)]  # type: list[tuple[tuple[int, int], int]]
    while queue:
        (x, y), steps = queue.pop()
        if min_path[x, y] < steps or steps >= best:
            continue
        min_path[x, y] = steps
        curr = grid[x, y]
        if curr == 1:
            # New best path found, but keep searching for the shortest path.
            best = steps
            continue
        for dx, dy in ((0, 1), (0, -1), (1, 0), (-1, 0)):
            pos = (x + dx, y + dy)
            val = grid[pos]
            if (
                val != 99
                and val >= curr - 1
                and min_path[pos] > steps + 1
                and best > steps + 1
            ):
                queue.append((pos, steps + 1))
    return best


def solve(path: str) -> None:
    grid, start, end = parse_grid(path)
    min_steps = part1(grid, start, end)
    print("Part 1:", min_steps)
    print("Part 2:", part2(grid, end, min_steps))


if __name__ == "__main__":
    solve("inputs/12.full")
