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
    print("Part 1:", part1(polymer, rules))
    print("Part 2:", part2(polymer, rules))
    print()


def part1(polymer: str, rules: dict[str, str]) -> int:
    for _ in range(10):
        result = polymer[0]
        for i in range(len(polymer) - 1):
            pair = polymer[i : i + 2]
            to_insert = rules.get(pair, "")
            result += to_insert + pair[1]
        polymer = result
    counts = Counter(polymer).most_common()
    return counts[0][1] - counts[-1][1]


def part2(polymer: str, rules: dict[str, str]) -> int:
    pairs = Counter(polymer[i : i + 2] for i in range(len(polymer) - 1))
    for _ in range(40):
        new_pairs = Counter()
        for pair, count in pairs.items():
            if pair in rules:
                ch = rules[pair]
                new_pairs[pair[0] + ch] += count
                new_pairs[ch + pair[1]] += count
            else:
                new_pairs[pair] += count
        pairs = new_pairs
    singles = Counter()
    for pair, count in pairs.items():
        if not singles:
            singles[pair[0]] += count
        singles[pair[1]] += count
    counts = singles.most_common()
    return counts[0][1] - counts[-1][1]


if __name__ == "__main__":
    solve("inputs/14.test")
    solve("inputs/14.full")
