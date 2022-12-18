#!/usr/bin/env python3
import numpy as np
from itertools import cycle

HORIZ = np.array([[1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]], dtype=int)
PLUS = np.array([[0, 1, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0], [0, 0, 0, 0]], dtype=int)
BELL = np.array([[1, 1, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 0, 0]], dtype=int)
VERT = np.array([[1, 0, 0, 0], [1, 0, 0, 0], [1, 0, 0, 0], [1, 0, 0, 0]], dtype=int)
BOX = np.array([[1, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]], dtype=int)
ROCKS = iter(cycle([HORIZ, PLUS, BELL, VERT, BOX]))


def solve(path: str) -> None:
    pattern = iter(cycle([-1 if ch == "<" else +1 for ch in open(path).read().strip()]))
    top = 0
    grid = np.ones((1, 11), dtype=int)
    for i in range(2022):
        rock = next(ROCKS)
        r, c = top + 4, 3
        if grid.shape[0] < top + 10:
            to_add = np.zeros((top + 10 - grid.shape[0], 11), dtype=int)
            to_add[:, 0] = 1
            to_add[:, 8] = 1
            grid = np.vstack([grid, to_add])
        while True:
            nc = c + next(pattern)
            if nc < 1 or nc > 7 or (grid[r : r + 4, nc : nc + 4] * rock).any():
                nc = c
            if (grid[r - 1 : r + 3, nc : nc + 4] * rock).any():
                grid[r : r + 4, nc : nc + 4] += rock
                top = max(top, r + rock.any(axis=1).nonzero()[0].max())
                break
            r -= 1
            c = nc
        # Debug: print the grid in reverse row order.
        # print("After rock", i + 1, "top =", top)
        # for row in reversed(grid):
        #     print("".join("#" if cell else "." for cell in row))
        # print()
    print("Part 1:", top)


if __name__ == "__main__":
    solve("inputs/17.full")
