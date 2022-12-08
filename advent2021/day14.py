#!/usr/bin/env python3
from collections import Counter


def parse_input(path: str) -> tuple[str, dict[str, str]]:
    lines = list(open(path))
    polymer_template = lines[0].strip()
    rules = {}
    for line in lines[2:]:
        lhs, rhs = line.strip().split(" -> ")
        rules[lhs] = rhs
    return polymer_template, rules


def solve(path: str) -> None:
    print(path)
    polymer, rules = parse_input(path)
    for _ in range(1, 11):
        result = polymer[0]
        for i in range(1, len(polymer)):
            pair = polymer[i - 1 : i + 1]
            to_insert = rules.get(pair, "")
            result += to_insert + pair[1]
        polymer = result
    counts = Counter(polymer).most_common()
    print("Part 1:", counts[0][1] - counts[-1][1])
    # TODO: Find the relationship and avoid growing the polymer
    print("Part 2:", "...")
    print()


if __name__ == "__main__":
    solve("inputs/14.test")
    solve("inputs/14.full")
