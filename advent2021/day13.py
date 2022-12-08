#!/usr/bin/env python3
import numpy as np
import numpy.typing as npt


def parse_input(path: str) -> tuple[npt.NDArray[np.int8], list[tuple[int, bool]]]:
    folds = []  # type: list[tuple[int, bool]]
    coords = []  # type: list[tuple[int, int]]
    for line in open(path):
        if line.startswith("fold along "):
            xy, val = line[11:].strip().split("=")
            folds.append((int(val), xy == "y"))
        elif line[0].isdigit():
            x, y = line.strip().split(",")
            coords.append((int(x), int(y)))
    max_x, max_y = np.max(coords, axis=0)
    paper = np.zeros((max_y + 1, max_x + 1), dtype=np.int8)
    for x, y in coords:
        paper[y, x] = 1
    return paper, folds


def solve(path: str) -> None:
    print(path)
    paper, folds = parse_input(path)
    for i, (val, is_y) in enumerate(folds):
        if is_y:
            paper = paper[:val] + paper[(val + 1) :][::-1]
        else:
            paper = paper[:, :val] + paper[:, (val + 1) :][:, ::-1]
        if i == 0:
            print("Part 1:", np.count_nonzero(paper))
    for row in paper:
        print("".join("#" if x else " " for x in row))


if __name__ == "__main__":
    solve("inputs/13.test")
    solve("inputs/13.full")
