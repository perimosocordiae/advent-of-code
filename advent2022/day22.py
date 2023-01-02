#!/usr/bin/env python3
import re
import numpy as np
import numpy.typing as npt

WALL = 2
OPEN = 1
NONE = 0
CHAR_MAP = {"#": WALL, ".": OPEN, " ": NONE}
RIGHT = (0, 1)
DOWN = (1, 0)
LEFT = (0, -1)
UP = (-1, 0)
FACINGS = [RIGHT, DOWN, LEFT, UP]  # type: list[tuple[int, int]]


def parse_input(path: str) -> tuple[npt.NDArray[np.int8], list[int | str]]:
    board_text, traj_text = open(path).read().split("\n\n")
    board_list = [[CHAR_MAP[c] for c in line] for line in board_text.splitlines()]
    max_len = max(len(line) for line in board_list)
    board = np.zeros((len(board_list), max_len), dtype=np.int8)
    for i, row in enumerate(board_list):
        board[i, : len(row)] = row
    traj = [
        int(tok) if tok.isdigit() else tok
        for tok in re.split(r"([RL])", traj_text.strip())
    ]  # type: list[int | str]
    return np.pad(board, 1, mode="constant"), traj


def find_password(board: npt.NDArray[np.int8], traj: list[int | str], wrap_fn) -> int:
    r = 1  # type: int
    c = np.where(board[r] == OPEN)[0][0]  # type: int
    facing = RIGHT  # type: tuple[int, int]
    for step in traj:
        if step == "L":
            facing = FACINGS[(FACINGS.index(facing) - 1) % 4]
        elif step == "R":
            facing = FACINGS[(FACINGS.index(facing) + 1) % 4]
        else:
            assert isinstance(step, int)
            for _ in range(step):
                r2 = r + facing[0]
                c2 = c + facing[1]
                facing2 = facing
                cell = board[r2, c2]
                if cell == NONE:
                    r2, c2, facing2 = wrap_fn(board, r, c, facing)
                    cell = board[r2, c2]
                if cell == WALL:
                    break
                if cell == OPEN:
                    r = r2
                    c = c2
                    facing = facing2
    return 1000 * r + 4 * c + FACINGS.index(facing)


def part1_wrap(
    board: npt.NDArray[np.int8], r: int, c: int, facing: tuple[int, int]
) -> tuple[int, int, tuple[int, int]]:
    fr, fc = facing
    if fc == 0:
        row_hits = np.where(board[:, c])[0]
        if fr == -1:  # heading up, wrap to bottom
            r2 = row_hits[-1]
        else:  # heading down, wrap to top
            r2 = row_hits[0]
        return r2, c, facing

    col_hits = np.where(board[r])[0]
    if fc == -1:  # heading left, wrap to right
        c2 = col_hits[-1]
    else:  # heading right, wrap to left
        c2 = col_hits[0]
    return r, c2, facing


# Part 2 diagram:
# _ 1 2
# _ 3 _
# 4 5 _
# 6 _ _
# This forms a cube, where each face is a 50x50 square.
SIDES = {
    (0, 1): 1,  # rows 1-50,    cols 51-100
    (0, 2): 2,  # rows 1-50,    cols 101-150
    (1, 1): 3,  # rows 51-100,  cols 51-100
    (2, 0): 4,  # rows 101-150, cols 1-50
    (2, 1): 5,  # rows 101-150, cols 51-100
    (3, 0): 6,  # rows 151-200, cols 1-50
}  # type: dict[tuple[int, int], int]


def part2_wrap(
    _, r: int, c: int, facing: tuple[int, int]
) -> tuple[int, int, tuple[int, int]]:
    r1 = r - 1
    c1 = c - 1
    side = SIDES[r1 // 50, c1 // 50]
    rr = r1 % 50
    cc = c1 % 50
    if side == 1:
        if facing is UP:  # wrap to face 6 on left
            return 151 + cc, 1, RIGHT
        assert facing is LEFT  # wrap to face 4 upside down on left
        return 150 - rr, 1, RIGHT
    if side == 2:
        if facing is UP:  # wrap to face 6 on bottom
            return 200, 1 + cc, UP
        if facing is DOWN:  # wrap to face 3 on right
            return 51 + cc, 100, LEFT
        assert facing is RIGHT  # wrap to face 5 upside down on right
        return 150 - rr, 100, LEFT
    if side == 3:
        if facing is LEFT:  # wrap to face 4 on top
            return 101, 1 + rr, DOWN
        assert facing is RIGHT  # wrap to face 2 on bottom
        return 50, 101 + rr, UP
    if side == 4:
        if facing is UP:  # wrap to face 3 on left
            return 51 + cc, 51, RIGHT
        assert facing is LEFT  # wrap to face 1 upside down on left
        return 50 - rr, 51, RIGHT
    if side == 5:
        if facing is DOWN:  # wrap to face 6 on right
            return 151 + cc, 50, LEFT
        assert facing is RIGHT  # wrap to face 2 upside down on right
        return 50 - rr, 150, LEFT
    assert side == 6
    if facing is DOWN:  # wrap to face 2 on top
        return 1, 101 + cc, DOWN
    if facing is LEFT:  # wrap to face 1 on top
        return 1, 51 + rr, DOWN
    assert facing is RIGHT  # wrap to face 5 on bottom
    return 150, 51 + rr, UP


def solve(path: str) -> None:
    board, traj = parse_input(path)
    print("Part 1:", find_password(board, traj, part1_wrap))
    print("Part 2:", find_password(board, traj, part2_wrap))


if __name__ == "__main__":
    solve("inputs/22.full")
