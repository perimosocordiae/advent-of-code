#!/usr/bin/env python3
import re
import numpy as np

WALL = 2
OPEN = 1
NONE = 0
CHAR_MAP = {"#": WALL, ".": OPEN, " ": NONE}
FACINGS = [(0, 1), (1, 0), (0, -1), (-1, 0)]  # right, down, left, up


def parse_input(path: str):
    board_text, traj_text = open(path).read().split("\n\n")
    board_list = [[CHAR_MAP[c] for c in line] for line in board_text.splitlines()]
    max_len = max(len(line) for line in board_list)
    board = np.zeros((len(board_list), max_len), dtype=np.int8)
    for i, row in enumerate(board_list):
        board[i, : len(row)] = row
    traj = [
        int(tok) if tok.isdigit() else tok
        for tok in re.split(r"([RL])", traj_text.strip())
    ]
    return np.pad(board, 1, mode="constant"), traj


def solve(path: str) -> None:
    board, traj = parse_input(path)
    r = 1
    c = np.where(board[r] == OPEN)[0][0]
    facing = FACINGS[0]  # right
    for step in traj:
        if step == "L":
            facing = FACINGS[(FACINGS.index(facing) - 1) % 4]
        elif step == "R":
            facing = FACINGS[(FACINGS.index(facing) + 1) % 4]
        else:
            assert isinstance(step, int)
            fr, fc = facing
            for _ in range(step):
                r2 = r + fr
                c2 = c + fc
                cell = board[r2, c2]
                if cell == NONE:
                    if fr == -1:  # heading up, wrap to bottom
                        r2 = np.where(board[:, c])[0][-1]
                    elif fr == 1:  # heading down, wrap to top
                        r2 = np.where(board[:, c])[0][0]
                    elif fc == -1:  # heading left, wrap to right
                        c2 = np.where(board[r])[0][-1]
                    else:  # heading right, wrap to left
                        c2 = np.where(board[r])[0][0]
                    cell = board[r2, c2]
                if cell == WALL:
                    break
                if cell == OPEN:
                    r = r2
                    c = c2
    part1 = 1000 * r + 4 * c + FACINGS.index(facing)
    print("Part 1:", part1)


if __name__ == "__main__":
    solve("inputs/22.full")
