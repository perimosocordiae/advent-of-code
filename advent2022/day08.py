#!/usr/bin/env python3
import numpy as np


def solve(path: str) -> None:
    trees = np.array(
        [list(map(int, line.strip())) for line in open(path)], dtype=np.int8
    )
    n, m = trees.shape
    visible = n * 2 + m * 2 - 4
    best_score = 0
    for r in range(1, n - 1):
        for c in range(1, m - 1):
            height = trees[r, c]
            left = trees[r, :c][::-1]
            right = trees[r, c + 1 :]
            up = trees[:r, c][::-1]
            down = trees[r + 1 :, c]
            if (
                height > left.max()
                or height > right.max()
                or height > up.max()
                or height > down.max()
            ):
                visible += 1
            sL = trees_seen(left, height)
            sR = trees_seen(right, height)
            sU = trees_seen(up, height)
            sD = trees_seen(down, height)
            score = sL * sR * sU * sD
            if score > best_score:
                best_score = score
    print("Part 1:", visible)
    print("Part 2:", best_score)


def trees_seen(arr, height) -> int:
    tmp = arr >= height
    idx = tmp.argmax()
    if not tmp[idx]:
        return len(arr)
    return idx + 1


if __name__ == "__main__":
    solve("inputs/08.full")
