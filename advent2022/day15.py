#!/usr/bin/env python3
import numpy as np
import numpy.typing as npt

IntArray = npt.NDArray[np.int_]


def parse_input(path: str) -> tuple[IntArray, IntArray]:
    sensors = []
    beacons = []
    for line in open(path):
        lhs, rhs = line.strip().split("x=")[1:]
        x, y = lhs.split(", y=")
        sensors.append((int(x), int(y.split(":")[0])))
        x, y = rhs.split(", y=")
        beacons.append((int(x), int(y)))
    return np.array(sensors), np.array(beacons)


def part1(sensors: IntArray, beacons: IntArray, y: int) -> int:
    dist = abs(sensors - beacons).sum(axis=1)  # type: IntArray
    x_offset = dist - abs(sensors[:, 1] - y)
    ranges = sorted(zip(sensors[:, 0] - x_offset, sensors[:, 0] + x_offset))
    ranges = [(a, b) for a, b in ranges if a <= b]  # type: list[tuple[int, int]]
    unique_ranges = [ranges[0]]
    for c, d in ranges[1:]:
        a, b = unique_ranges[-1]
        if c <= b:
            unique_ranges[-1] = (a, max(b, d))
        else:
            unique_ranges.append((a, b))
    count = sum(b - a + 1 for a, b in unique_ranges)
    # Exclude known beacons at y from the count.
    count -= len(np.unique(beacons[beacons[:, 1] == y, 0]))
    return count


def part2(sensors: IntArray, beacons: IntArray, y: int) -> int:
    # TODO
    return 0


def solve(path: str, y: int) -> None:
    sensors, beacons = parse_input(path)
    print("Part 1:", part1(sensors, beacons, y))
    print("Part 2:", part2(sensors, beacons, 2 * y))


if __name__ == "__main__":
    solve("inputs/15.test", y=10)
    solve("inputs/15.full", y=2000000)
