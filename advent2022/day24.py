#!/usr/bin/env python3
from collections import deque
import numpy as np
import numpy.typing as npt

OPEN = 0b00000000
WALL = 0b11111111
UP = 0b00000001
DOWN = 0b00000010
LEFT = 0b00000100
RIGHT = 0b00001000

CHAR_MAP = {
    ".": OPEN,
    "#": WALL,
    "^": UP,
    "v": DOWN,
    "<": LEFT,
    ">": RIGHT,
}
DIR_MAP = {
    DOWN: (1, 0),
    RIGHT: (0, 1),
    LEFT: (0, -1),
    UP: (-1, 0),
}


def make_next_grid(grid: npt.NDArray[np.int8]) -> npt.NDArray[np.int8]:
    inner = grid[1:-1, 1:-1]
    new_inner = np.zeros_like(inner)
    new_inner |= np.roll(inner & UP, -1, axis=0)
    new_inner |= np.roll(inner & DOWN, 1, axis=0)
    new_inner |= np.roll(inner & LEFT, -1, axis=1)
    new_inner |= np.roll(inner & RIGHT, 1, axis=1)
    new_grid = grid.copy()
    new_grid[1:-1, 1:-1] = new_inner
    return new_grid


def parse_input(
    path: str,
) -> tuple[npt.NDArray[np.int8], tuple[int, int], tuple[int, int]]:
    grid = []
    for line in open(path):
        grid.append([CHAR_MAP[c] for c in line.strip()])
    grid = np.array(grid, dtype=np.int8)
    start = (0, np.where(grid[0] == OPEN)[0][0])
    stop = (len(grid) - 1, np.where(grid[-1] == OPEN)[0][0])
    return grid, start, stop


def solve(path: str) -> None:
    initial_grid, start, stop = parse_input(path)
    all_grids = [initial_grid]
    seen = set()
    queue = deque([(start, 0)])  # type: deque[tuple[tuple[int, int], int]]
    while queue:
        pos, steps = queue.popleft()
        if pos == stop:
            print("Part 1:", steps)
            break
        if (pos, steps) in seen:
            continue
        seen.add((pos, steps))
        grid = all_grids[steps]
        next_steps = steps + 1
        if next_steps == len(all_grids):
            all_grids.append(make_next_grid(grid))
        next_grid = all_grids[next_steps]
        if next_grid[pos] == OPEN:
            queue.append((pos, next_steps))
        for dr, dc in DIR_MAP.values():
            next_pos = (pos[0] + dr, pos[1] + dc)
            if next_grid[next_pos] == OPEN:
                queue.append((next_pos, next_steps))
        # print(len(queue), len(all_grids))


if __name__ == "__main__":
    solve("inputs/24.full")
