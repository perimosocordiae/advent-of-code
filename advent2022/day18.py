#!/usr/bin/env python3
import numpy as np


def solve(path: str) -> None:
    coords = np.loadtxt(path, dtype=int, delimiter=",")
    dist = np.abs(coords[:, None] - coords).sum(axis=2)
    surface_area = (6 - (dist == 1).sum(axis=0)).sum()
    print("Part 1:", surface_area)


if __name__ == "__main__":
    solve("inputs/18.full")
