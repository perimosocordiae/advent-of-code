#!/usr/bin/env python3
import dataclasses


@dataclasses.dataclass
class Valve:
    flow_rate: int
    tunnels: list[str]

    @staticmethod
    def from_str(line: str) -> tuple[str, "Valve"]:
        parts = line.split()
        name = parts[1]
        rate = int(parts[4].rstrip(";").split("=")[1])
        tunnels = [tunnel.rstrip(",") for tunnel in parts[9:]]
        return name, Valve(rate, tunnels)


def solve(path: str) -> None:
    valves = dict(Valve.from_str(line) for line in open(path))
    start = "AA"


if __name__ == "__main__":
    solve("inputs/16.test")
