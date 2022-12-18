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


def parse_input(path: str) -> tuple[np.ndarray, np.ndarray]:
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
    return adj, rates


def part1(path: str) -> None:
    adj, rates = parse_input(path)
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


def part2(path: str) -> None:
    adj, rates = parse_input(path)
    max_flow = 0
    # Stores (flow, node1, node2, time, rates)
    queue = [
        (0, (0, 26), (0, 26), rates)
    ]  # type: list[tuple[int, tuple[int, int], tuple[int, int], np.ndarray]]
    while queue:
        flow, (n1, t1), (n2, t2), rates = queue.pop()
        if t1 >= t2 and rates[n1] > 0:
            new_flow = flow + rates[n1] * (t1 - 1)
            new_rates = rates.copy()
            new_rates[n1] = 0
            queue.append((new_flow, (n1, t1 - 1), (n2, t2), new_rates))
            if new_flow > max_flow:
                max_flow = new_flow
                print(new_flow)
            continue
        if t2 > t1 and rates[n2] > 0:
            new_flow = flow + rates[n2] * (t2 - 1)
            new_rates = rates.copy()
            new_rates[n2] = 0
            queue.append((new_flow, (n1, t1), (n2, t2 - 1), new_rates))
            if new_flow > max_flow:
                max_flow = new_flow
                print(new_flow)
            continue
        if t1 > t2 and rates[n1] == 0:
            for nn1 in np.nonzero(rates)[0]:
                cost = adj[n1, nn1]
                if t1 > cost:
                    queue.append((flow, (nn1, t1 - cost), (n2, t2), rates))
            continue
        if t2 > t1 and rates[n2] == 0:
            for nn2 in np.nonzero(rates)[0]:
                cost = adj[n2, nn2]
                if t2 > cost:
                    queue.append((flow, (n1, t1), (nn2, t2 - cost), rates))
            continue
        if t1 == t2:
            if rates[n1] == 0 and rates[n2] == 0:
                (inds,) = np.nonzero(rates)
                for nn1 in inds:
                    cost1 = adj[n1, nn1]
                    if t1 > cost1:
                        for nn2 in inds:
                            cost2 = adj[n2, nn2]
                            if nn2 != nn1 and t2 > cost2:
                                queue.append(
                                    (flow, (nn1, t1 - cost1), (nn2, t2 - cost2), rates)
                                )
            elif rates[n1] == 0:
                for nn1 in np.nonzero(rates)[0]:
                    cost1 = adj[n1, nn1]
                    if t1 > cost1:
                        queue.append((flow, (nn1, t1 - cost1), (n2, t2), rates))
            elif rates[n2] == 0:
                for nn2 in np.nonzero(rates)[0]:
                    cost2 = adj[n2, nn2]
                    if t2 > cost2:
                        queue.append((flow, (n1, t1), (nn2, t2 - cost2), rates))
    print("Part 2:", max_flow)


if __name__ == "__main__":
    part1("inputs/16.full")
    part2("inputs/16.full")
