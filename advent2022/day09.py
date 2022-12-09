#!/usr/bin/env python3
from typing import Iterable
import numpy as np
import numpy.typing as npt

DIRECTIONS = {
    "R": np.array([1, 0], dtype=int),
    "L": np.array([-1, 0], dtype=int),
    "U": np.array([0, 1], dtype=int),
    "D": np.array([0, -1], dtype=int),
}  # type: dict[str, npt.NDArray[np.int_]]


def parse_input(path) -> Iterable[npt.NDArray[np.int_]]:
    for line in open(path):
        dir_, length = line.strip().split(" ")
        yield from [DIRECTIONS[dir_]] * int(length)


def move_rope(head_motions: list[npt.NDArray[np.int_]], rope_length: int) -> int:
    rope = np.zeros((rope_length, 2), dtype=int)  # type: npt.NDArray[np.int_]
    tail_positions = set()
    for dir_ in head_motions:
        rope[0] += dir_
        for i in range(1, rope_length):
            delta = rope[i - 1] - rope[i]
            if abs(delta).max() > 1:
                rope[i] += np.sign(delta)
        tail_positions.add(tuple(rope[-1]))
    return len(tail_positions)


def solve(path: str) -> None:
    head_motions = list(parse_input(path))
    print("Part 1:", move_rope(head_motions, 2))
    print("Part 1:", move_rope(head_motions, 10))


if __name__ == "__main__":
    solve("inputs/09.full")
