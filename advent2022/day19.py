#!/usr/bin/env python3
import numpy as np
import numpy.typing as npt

Materials = npt.NDArray[np.int_]


def mats(ore=0, clay=0, obsidian=0) -> Materials:
    return np.array([ore, clay, obsidian], dtype=int)


def max_geodes(costs: Materials, yields: Materials, total_time: int) -> int:
    """Finds the max number of geodes that can be produced in T minutes."""
    max_cost = costs.max(axis=0)

    def rec(time: int, robots: Materials, inventory: Materials, last: int) -> int:
        if time <= 1:
            return 0
        maxd = max_cost * time - inventory
        need = robots * time <= maxd
        build = (inventory >= costs).all(axis=1)
        build[:3] &= robots < max_cost
        last_build = (inventory - robots >= costs).all(axis=1)
        t = time - 1
        inv = inventory + robots  # type: Materials
        if build[3]:
            return rec(t, robots, inv - costs[3], 3) + t
        best = 0
        if build[2] and need[2] and not (last in (2, -1) and last_build[2]):
            best = max(best, rec(t, robots + yields[2], inv - costs[2], 2))
        if build[1] and need[1] and need[2] and not (last in (1, -1) and last_build[1]):
            best = max(best, rec(t, robots + yields[1], inv - costs[1], 1))
        if build[0] and need[0] and not (last in (0, -1) and last_build[0]):
            best = max(best, rec(t, robots + yields[0], inv - costs[0], 0))
        if not build[0]:
            best = max(best, rec(t, robots, inv, -1))
        return best

    return rec(total_time, mats(ore=1), mats(), -1)


def solve(path: str) -> None:
    blueprints = []
    for line in open(path):
        parts = line.split()
        if len(parts) < 31:
            continue
        num = int(parts[1].rstrip(":"))
        costs = np.array(
            [
                mats(ore=int(parts[6])),
                mats(ore=int(parts[12])),
                mats(ore=int(parts[18]), clay=int(parts[21])),
                mats(ore=int(parts[27]), obsidian=int(parts[30])),
            ]
        )
        blueprints.append((num, costs))

    yields = np.eye(3, dtype=int)
    sum_quality = 0
    for num, costs in blueprints:
        sum_quality += num * max_geodes(costs, yields, 24)
    print("Part 1:", sum_quality)

    prod_geodes = 1
    for _, costs in blueprints[:3]:
        prod_geodes *= max_geodes(costs, yields, 32)
    print("Part 2:", prod_geodes)


if __name__ == "__main__":
    solve("inputs/19.full")
