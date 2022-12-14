#!/usr/bin/env python3


def parse_input(path: str) -> set[tuple[int, int]]:
    rocks = set()
    for line in open(path):
        trace = [
            tuple(map(int, coord.split(","))) for coord in line.strip().split(" -> ")
        ]
        sx, sy = trace[0]
        rocks.add((sx, sy))
        for nx, ny in trace[1:]:
            if nx == sx:
                for y in range(min(ny, sy), max(ny, sy) + 1):
                    rocks.add((sx, y))
            else:
                for x in range(min(nx, sx), max(nx, sx) + 1):
                    rocks.add((x, sy))
            sx, sy = nx, ny
    return rocks


def drop_sand(sx: int, sy: int, rocks: set[tuple[int, int]], max_y: int) -> bool:
    while (sx, sy) not in rocks:
        sy += 1
        if sy > max_y:
            return False
    if (sx - 1, sy) not in rocks:
        return drop_sand(sx - 1, sy, rocks, max_y)
    if (sx + 1, sy) not in rocks:
        return drop_sand(sx + 1, sy, rocks, max_y)
    rocks.add((sx, sy - 1))
    return True


def part1(path: str) -> None:
    rocks = parse_input(path)
    max_y = max(y for _, y in rocks)
    count = 0
    while drop_sand(500, 0, rocks, max_y):
        count += 1
    print("Part 1:", count)


def part2(path: str) -> None:
    rocks = parse_input(path)
    floor = max(y for _, y in rocks) + 2
    for x in range(300, 700):
        rocks.add((x, floor))
    start = (500, 0)
    count = 0
    while start not in rocks:
        assert drop_sand(*start, rocks, floor)
        count += 1
    print("Part 2:", count)


if __name__ == "__main__":
    part1("inputs/14.full")
    part2("inputs/14.full")
