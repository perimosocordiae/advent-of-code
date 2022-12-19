#!/usr/bin/env python3
import dataclasses


@dataclasses.dataclass
class Materials:
    ore: int
    clay: int
    obsidian: int
    geode: int

    def all_satisfied(self, other: "Materials") -> bool:
        return (
            self.ore >= other.ore
            and self.clay >= other.clay
            and self.obsidian >= other.obsidian
            and self.geode >= other.geode
        )

    def __isub__(self, other: "Materials") -> "Materials":
        self.ore -= other.ore
        self.clay -= other.clay
        self.obsidian -= other.obsidian
        self.geode -= other.geode
        return self

    def __str__(self) -> str:
        res = []
        if self.ore > 0:
            res.append(f"{self.ore} ore")
        if self.clay > 0:
            res.append(f"{self.clay} clay")
        if self.obsidian > 0:
            res.append(f"{self.obsidian} obsidian")
        if self.geode > 0:
            res.append(f"{self.geode} geode")
        return ", ".join(res)


def solve(path: str) -> None:
    for line in open(path):
        parts = line.split()
        if len(parts) < 31:
            continue
        blueprint = int(parts[1].rstrip(":"))
        robot_costs = {
            "ore": Materials(int(parts[6]), 0, 0, 0),
            "clay": Materials(int(parts[12]), 0, 0, 0),
            "obsidian": Materials(int(parts[18]), int(parts[21]), 0, 0),
            "geode": Materials(int(parts[27]), 0, int(parts[30]), 0),
        }
        print(blueprint, robot_costs)

        # TODO: Decompose this into a graph, then search for the best path.
        total_time = 24
        robots = Materials(1, 0, 0, 0)
        materials = Materials(0, 0, 0, 0)
        for t in range(total_time):
            materials.ore += robots.ore
            materials.clay += robots.clay
            materials.obsidian += robots.obsidian
            materials.geode += robots.geode
            if materials.all_satisfied(robot_costs["geode"]):
                materials -= robot_costs["geode"]
                robots.geode += 1
            if materials.all_satisfied(robot_costs["obsidian"]):
                materials -= robot_costs["obsidian"]
                robots.obsidian += 1
            if materials.all_satisfied(robot_costs["clay"]):
                materials -= robot_costs["clay"]
                robots.clay += 1
            if materials.all_satisfied(robot_costs["ore"]):
                materials -= robot_costs["ore"]
                robots.ore += 1
            print("Time", t + 1)
            print("\tRobots", robots)
            print("\tMaterials", materials)


if __name__ == "__main__":
    solve("inputs/19.test")
