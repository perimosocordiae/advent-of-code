#!/usr/bin/env python3
import dataclasses
import numpy as np


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
    nodes = sorted(valves.keys())
    n = len(nodes)
    rates = np.array([valves[k].flow_rate for k in nodes], dtype=int)
    adj = np.full((n, n), 99, dtype=int)
    np.fill_diagonal(adj, 0)
    for key, valve in valves.items():
        i = nodes.index(key)
        for tunnel in valve.tunnels:
            j = nodes.index(tunnel)
            adj[i, j] = 1
    # Floyd-Warshall
    for k in range(n):
        for i in range(n):
            for j in range(n):
                adj[i, j] = min(adj[i, j], adj[i, k] + adj[k, j])
    rates[0] = 1
    (inds,) = np.nonzero(rates)
    adj = adj[inds, inds[:, None]]
    rates = rates[inds]
    rates[0] = 0

    max_flow = 0
    # Stores (flow, node, time, rates)
    queue = [(0, 0, 30, rates)]  # type: list[tuple[int, int, int, np.ndarray]]
    while queue:
        flow, node, time, rates = queue.pop()
        if rates[node] > 0:
            new_flow = flow + rates[node] * (time - 1)
            new_rates = rates.copy()
            new_rates[node] = 0
            queue.append((new_flow, node, time - 1, new_rates))
            if new_flow > max_flow:
                max_flow = new_flow
        else:
            for next_node in np.nonzero(rates)[0]:
                cost = adj[node, next_node]
                if time > cost:
                    queue.append((flow, next_node, time - cost, rates))
    print("Part 1:", max_flow)


if __name__ == "__main__":
    solve("inputs/16.full")
