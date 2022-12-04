#!/usr/bin/env python3


def parse(line: str) -> tuple[range, range]:
    lhs, rhs = line.split(",")
    return parse_range(lhs), parse_range(rhs)


def parse_range(line: str) -> range:
    lo, hi = map(int, line.split("-"))
    return range(lo, hi + 1)


def fully_contains(a: range, b: range) -> bool:
    return (a[0] in b and a[-1] in b) or (b[0] in a and b[-1] in a)


def has_overlap(a: range, b: range) -> bool:
    return bool(range(max(a[0], b[0]), min(a[-1], b[-1]) + 1))


def part1(path: str) -> int:
    return sum(fully_contains(*parse(line)) for line in open(path))


def part2(path: str) -> int:
    return sum(has_overlap(*parse(line)) for line in open(path))


if __name__ == "__main__":
    print("Part 1:", part1("inputs/04.full"))
    print("Part 2:", part2("inputs/04.full"))
