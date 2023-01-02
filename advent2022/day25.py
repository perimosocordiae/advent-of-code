#!/usr/bin/env python3


def solve(path: str) -> None:
    total = 0
    for line in open(path):
        total += parse_snafu(line.strip())
    print("Part 1:", to_snafu(total))


def parse_snafu(digits: str) -> int:
    result = 0
    for i, digit in enumerate(digits[::-1]):
        if digit == "-":
            val = -1
        elif digit == "=":
            val = -2
        else:
            val = int(digit)
        place = 5**i
        result += val * place
    return result


def to_snafu(num: int) -> str:
    result = ""
    while num:
        digit = num % 5
        if digit == 4:
            digit = "-"
            num += 1
        elif digit == 3:
            digit = "="
            num += 2
        result += str(digit)
        num //= 5
    return result[::-1]


if __name__ == "__main__":
    solve("inputs/25.full")
