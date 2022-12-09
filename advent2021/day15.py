#!/usr/bin/env python3
import numpy as np
import numpy.typing as npt
from collections import deque


def shortest_path(grid: npt.NDArray[np.int32]) -> int:
    costs = np.full_like(grid, 99999)
    costs[0, 0] = 0
    queue = deque([(0, 0)])  # type: deque[tuple[int, int]]
    while queue:
        x, y = queue.popleft()
        for dx, dy in ((0, 1), (1, 0), (0, -1), (-1, 0)):
            nx, ny = x + dx, y + dy
            if 0 <= nx < grid.shape[1] and 0 <= ny < grid.shape[0]:
                risk = costs[y, x] + grid[ny, nx]
                if costs[ny, nx] > risk:
                    costs[ny, nx] = risk
                    queue.append((nx, ny))
    return costs[-1, -1]


def solve(path: str) -> None:
    print(path)
    grid = np.array(
        [[int(ch) for ch in line] for line in open(path).read().splitlines()],
        dtype=np.int32,
    )
    print("Part 1:", shortest_path(grid))
    tiles = [[grid + (c + r - 1) for c in range(5)] for r in range(5)]
    big_grid = (np.block(tiles) % 9) + 1
    print("Part 2:", shortest_path(big_grid))
    print()


if __name__ == "__main__":
    solve("inputs/15.test")
    solve("inputs/15.full")
