#!/usr/bin/env python3
import numpy as np


def solve(path: str) -> None:
    nums = np.loadtxt(path, dtype=int)
    print(nums)


if __name__ == "__main__":
    solve("inputs/20.test")
