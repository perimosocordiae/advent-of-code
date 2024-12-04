import numpy as np
from collections import defaultdict

XMAS = np.array([1, 2, 3, 4], dtype=np.uint8)


def is_xmas(arr) -> bool:
    return np.array_equal(arr, XMAS) or np.array_equal(arr, XMAS[::-1])


def count_xmas(grid) -> int:
    count = 0
    for i in range(grid.shape[0]):
        for j in range(grid.shape[1]):
            patch = grid[i : i + 4, j : j + 4]
            count += is_xmas(patch[0])
            count += is_xmas(patch[:, 0])
            count += is_xmas(patch.diagonal())
            count += is_xmas(np.fliplr(patch).diagonal())
    return count


def count_mas_xs(grid) -> int:
    h, w = grid.shape
    count = 0
    pattern = [(2, 4), (4, 2)]  # MAS or SAM
    for i in range(1, h - 1):
        for j in range(1, w - 1):
            if grid[i, j] != 3:
                continue
            patch = grid[i - 1 : i + 2, j - 1 : j + 2]
            d1 = (patch[0, 0], patch[2, 2])
            d2 = (patch[0, 2], patch[2, 0])
            if d1 in pattern and d2 in pattern:
                count += 1
    return count


infile = "inputs/04.full"
chars = defaultdict(int, dict(zip("XMAS", XMAS)))
grid = np.array(
    [[chars[x] for x in line.strip()] for line in open(infile)], dtype=np.uint8
)
print("Part 1:", count_xmas(grid))
print("Part 2:", count_mas_xs(grid))
