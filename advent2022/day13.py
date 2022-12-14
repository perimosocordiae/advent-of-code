#!/usr/bin/env python3
import ast
from functools import cmp_to_key


def compare(left, right) -> int:
    if isinstance(left, int):
        if isinstance(right, int):
            return left - right
        return compare([left], right)
    if isinstance(right, int):
        return compare(left, [right])
    for lx, rx in zip(left, right):
        if res := compare(lx, rx):
            return res
    return len(left) - len(right)


def solve(path: str) -> None:
    chunks = open(path).read().split("\n\n")
    count = 0
    for i, chunk in enumerate(chunks):
        left, right = map(ast.literal_eval, chunk.splitlines())
        if compare(left, right) < 0:
            count += i + 1
    print("Part 1:", count)

    divider1 = [[2]]
    divider2 = [[6]]
    packets = [
        ast.literal_eval(line) for chunk in chunks for line in chunk.splitlines()
    ]
    packets.extend([divider1, divider2])
    packets.sort(key=cmp_to_key(compare))
    idx1 = packets.index(divider1) + 1
    idx2 = packets.index(divider2) + 1
    print("Part 2:", idx1 * idx2)


if __name__ == "__main__":
    solve("inputs/13.full")
