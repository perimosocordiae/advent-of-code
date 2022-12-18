#!/usr/bin/env python3
import numpy as np
import scipy.ndimage as ndi


def surface_area(coords) -> int:
    dist = np.abs(coords[:, None] - coords).sum(axis=2)
    return (6 - (dist == 1).sum(axis=0)).sum()


def solve(path: str) -> None:
    coords = np.loadtxt(path, dtype=int, delimiter=",")
    total_surface = surface_area(coords)
    print("Part 1:", total_surface)

    min_pt = coords.min(axis=0)
    max_pt = coords.max(axis=0)
    shape = max_pt - min_pt + 1
    grid = np.ones(shape, dtype=int)
    i, j, k = (coords - min_pt).T
    grid[i, j, k] = 0
    labels, n = ndi.label(grid)
    external_surface = total_surface
    for i in range(2, n + 1):
        external_surface -= surface_area(np.column_stack(np.where(labels == i)))
    print("Part 2:", external_surface)


if __name__ == "__main__":
    solve("inputs/18.full")
