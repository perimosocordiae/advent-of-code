#!/usr/bin/env python3
from collections import deque


def find_seq(message: str, n: int) -> int:
    buffer = deque(maxlen=n)
    for i, char in enumerate(message):
        buffer.append(char)
        if len(buffer) != n:
            continue
        if len(set(buffer)) == n:
            return i + 1
    return -1


def part1(path: str) -> str:
    return ",".join(str(find_seq(line, 4)) for line in open(path))


def part2(path: str) -> str:
    return ",".join(str(find_seq(line, 14)) for line in open(path))


if __name__ == "__main__":
    print("Part 1:", part1("inputs/06.full"))
    print("Part 2:", part2("inputs/06.full"))
