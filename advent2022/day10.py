#!/usr/bin/env python3
import numpy as np
import numpy.typing as npt


def run_program(path: str) -> npt.NDArray[np.int_]:
    x = 1
    cycle = 1
    trace_ = [(cycle, x)]  # type: list[tuple[int, int]]
    for line in open(path):
        if line.startswith("addx "):
            x += int(line[5:])
            cycle += 2
        else:
            cycle += 1
        trace_.append((cycle, x))
    return np.array(trace_, dtype=int)


def solve(path: str) -> None:
    targets = np.array([20, 60, 100, 140, 180, 220])
    trace = run_program(path)
    inds = np.searchsorted(trace[:, 0], targets, side="right") - 1
    print("Part 1:", targets @ trace[inds, 1])

    all_cycles = np.arange(6 * 40)
    inds = np.searchsorted(trace[:, 0], all_cycles + 1, side="right") - 1
    image = (abs(trace[inds, 1] - (all_cycles % 40)) <= 1).reshape(6, 40)
    print("Part 2:")
    for line in image:
        print("".join("#" if x else " " for x in line))


if __name__ == "__main__":
    solve("inputs/10.full")
