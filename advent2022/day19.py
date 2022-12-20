#!/usr/bin/env python3
import numpy as np


class Materials:
    def __init__(self, ore=0, clay=0, obsidian=0, geode=0) -> None:
        self.x = np.array([ore, clay, obsidian, geode], dtype=int)

    @property
    def geode(self) -> int:
        return self.x[3]

    def all_satisfied(self, other: "Materials") -> bool:
        return bool((self.x >= other.x).all())

    def sort_key(self) -> int:
        return np.inner(self.x, [1, 100, 10000, 1000000])

    def __add__(self, other: "Materials") -> "Materials":
        return Materials(*self.x + other.x)

    def __sub__(self, other: "Materials") -> "Materials":
        return Materials(*self.x - other.x)

    def __repr__(self) -> str:
        return repr(self.x)

    def __str__(self) -> str:
        ore, clay, obsidian, geode = self.x
        res = []
        if ore > 0:
            res.append(f"{ore} ore")
        if clay > 0:
            res.append(f"{clay} clay")
        if obsidian > 0:
            res.append(f"{obsidian} obsidian")
        if geode > 0:
            res.append(f"{geode} geode")
        return ", ".join(res)

    def __hash__(self) -> int:
        return hash(tuple(self.x))


def max_geodes_old(blueprint: dict[str, Materials]) -> int:
    """Finds the max number of geodes that can be produced in 24 minutes."""
    best_geodes = 0
    queue = [
        (24, Materials(ore=1), Materials())
    ]  # type: list[tuple[int, Materials, Materials]]
    while queue:
        time, robots, inventory = queue.pop()
        if time == 0:
            if inventory.geode > best_geodes:
                best_geodes = inventory.geode
                print("New best:", best_geodes)
            continue
        for kind in ["ore", "clay", "obsidian", "geode"]:
            cost = blueprint[kind]
            if inventory.all_satisfied(cost):
                new_inventory = inventory - cost + robots
                new_robots = robots + Materials(**{kind: 1})
                queue.append((time - 1, new_robots, new_inventory))
        inventory += robots
        queue.append((time - 1, robots, inventory))
        queue.sort(key=lambda x: x[1].sort_key())
    return best_geodes


def max_geodes(blueprint: dict[str, Materials]) -> int:
    """Finds the max number of geodes that can be produced in 24 minutes."""

    def rec(time: int, robots: Materials, inventory: Materials) -> int:
        if time == 1:
            return inventory.geode + robots.geode
        best = rec(time - 1, robots, inventory + robots)
        for kind in ["ore", "clay", "obsidian", "geode"]:
            cost = blueprint[kind]
            if inventory.all_satisfied(cost):
                new_inventory = inventory - cost + robots
                new_robots = robots + Materials(**{kind: 1})
                best = max(best, rec(time - 1, new_robots, new_inventory))
        return best

    return rec(24, Materials(ore=1), Materials())


def solve(path: str) -> None:
    sum_geodes = 0
    for line in open(path):
        parts = line.split()
        if len(parts) < 31:
            continue
        num = int(parts[1].rstrip(":"))
        blueprint = {
            "ore": Materials(ore=int(parts[6])),
            "clay": Materials(ore=int(parts[12])),
            "obsidian": Materials(ore=int(parts[18]), clay=int(parts[21])),
            "geode": Materials(ore=int(parts[27]), obsidian=int(parts[30])),
        }
        print(num, blueprint)
        sum_geodes += max_geodes(blueprint)
    print("Part 1:", sum_geodes)


if __name__ == "__main__":
    solve("inputs/19.test")
