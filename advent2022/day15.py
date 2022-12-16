#!/usr/bin/env python3
from collections import Counter
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


def segment_intersection(line1, line2) -> tuple[int, int] | None:
    (x1, y1), (x2, y2) = line1
    (x3, y3), (x4, y4) = line2
    if max(x1, x2) < min(x3, x4) or max(y1, y2) < min(y3, y4):
        return None
    slope1 = (y2 - y1) // (x2 - x1)
    slope2 = (y4 - y3) // (x4 - x3)
    off1 = y1 - slope1 * x1
    off2 = y3 - slope2 * x3
    if slope1 == slope2:
        return None
    x = (off2 - off1) // (slope1 - slope2)
    y = slope1 * x + off1
    if x < min(x1, x2) or x > max(x1, x2) or x < min(x3, x4) or x > max(x3, x4):
        return None
    return x, y


def part2(sensors: IntArray, beacons: IntArray, limit: int) -> int:
    dist = abs(sensors - beacons).sum(axis=1)  # type: IntArray
    # Find the outside corners of each sensor's range.
    minx, miny = (sensors - dist[:, None]).T - 1
    maxx, maxy = (sensors + dist[:, None]).T + 1
    x, y = sensors.T
    tops = np.column_stack([x, miny])
    bottoms = np.column_stack([x, maxy])
    lefts = np.column_stack([minx, y])
    rights = np.column_stack([maxx, y])
    # Group boundary lines by slope.
    backslash_lines = list(zip(tops, rights))
    backslash_lines.extend(zip(lefts, bottoms))
    forwardslash_lines = list(zip(lefts, tops))
    forwardslash_lines.extend(zip(bottoms, rights))
    # Find the most frequent intersection points of all boundary lines.
    points = Counter()
    for line1 in backslash_lines:
        for line2 in forwardslash_lines:
            if pt := segment_intersection(line1, line2):
                if 0 <= pt[0] <= limit and 0 <= pt[1] <= limit:
                    points[pt] += 1
    # Find the first (and only) point that is not in range of any sensor.
    for pt, _ in points.most_common():
        if (abs(sensors - pt).sum(axis=1) > dist).all():
            return pt[0] * 4000000 + pt[1]
    return -1


def solve(path: str, y: int) -> None:
    print(path)
    sensors, beacons = parse_input(path)
    print("Part 1:", part1(sensors, beacons, y))
    print("Part 2:", part2(sensors, beacons, 2 * y))
    print()


if __name__ == "__main__":
    solve("inputs/15.test", y=10)
    solve("inputs/15.full", y=2000000)
