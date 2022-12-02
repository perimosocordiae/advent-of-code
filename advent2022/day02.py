#!/usr/bin/env python3


def parse(line: str) -> tuple[int, int]:
    a, b = line.strip().split()
    return ord(a) - 65, ord(b) - 88


def score(a: int, b: int) -> int:
    if a == b:
        return 4 + b
    if (a + 1) % 3 == b:
        return 7 + b
    return 1 + b


def score2(a: int, b: int) -> int:
    if b == 0:  # lose
        new_b = a - 1 if a > 0 else 2
    elif b == 1:  # tie
        new_b = a
    else:  # win
        new_b = (a + 1) % 3
    return score(a, new_b)


if __name__ == "__main__":
    rounds = list(map(parse, open("inputs/02.full")))
    print("Part 1:", sum(score(a, b) for a, b in rounds))
    print("Part 2:", sum(score2(a, b) for a, b in rounds))
