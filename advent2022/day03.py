#!/usr/bin/env python3
from typing import Iterable
import itertools


def common_priority(parts: Iterable[str]) -> int:
    (ch,) = set.intersection(*[set(p.strip()) for p in parts])
    return priority(ch)


def split_halves(line: str) -> tuple[str, str]:
    mid = len(line) // 2
    return line[:mid], line[mid:]


def priority(ch: str) -> int:
    if ch.isupper():
        return ord(ch) - ord("A") + 27
    return ord(ch) - ord("a") + 1


def batched(iterable: Iterable[str], n: int) -> Iterable[list[str]]:
    "Batch data into lists of length n. The last batch may be shorter."
    # batched('ABCDEFG', 3) --> ABC DEF G
    it = iter(iterable)
    while batch := list(itertools.islice(it, n)):
        yield batch


def part1(path: str) -> int:
    return sum(common_priority(split_halves(line)) for line in open(path))


def part2(path: str) -> int:
    return sum(common_priority(parts) for parts in batched(open(path), 3))


if __name__ == "__main__":
    print("Part 1:", part1("inputs/03.full"))
    print("Part 2:", part2("inputs/03.full"))
