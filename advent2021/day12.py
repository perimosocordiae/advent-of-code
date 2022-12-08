#!/usr/bin/env python3
import numpy as np
import numpy.typing as npt


def count_unique_paths(adj: list[list[int]], nodes: list[str], room_limit: int) -> int:
    start = nodes.index("start")
    end = nodes.index("end")
    visit_limit = np.array(
        [9999 if n.isupper() else room_limit for n in nodes], dtype=int
    )

    def rec_helper(node: int, visit_count: npt.NDArray[np.int_]) -> int:
        if node == end:
            return 1
        visit_count[node] += 1
        count = 0
        for next_node in adj[node]:
            if next_node == start:
                continue
            vc = visit_count[next_node]
            vl = visit_limit[next_node]
            if vc >= vl:
                continue
            if vc > 0 and vl < 9999 and (visit_limit - visit_count).min() == 0:
                continue
            count += rec_helper(next_node, visit_count)
        visit_count[node] -= 1
        return count

    return rec_helper(start, np.zeros(len(nodes), dtype=int))


def solve(path: str) -> None:
    nodes = []  # type: list[str]
    edges = []  # type: list[tuple[int, int]]
    for line in open(path):
        s, t = line.strip().split("-")
        try:
            s_idx = nodes.index(s)
        except ValueError:
            s_idx = len(nodes)
            nodes.append(s)
        try:
            t_idx = nodes.index(t)
        except ValueError:
            t_idx = len(nodes)
            nodes.append(t)
        edges.append((s_idx, t_idx))
    n = len(nodes)
    adj = [[] for _ in range(n)]  # type: list[list[int]]
    for s, t in sorted(edges):
        adj[s].append(t)
        adj[t].append(s)
    print(path)
    print("Part 1:", count_unique_paths(adj, nodes, room_limit=1))
    print("Part 2:", count_unique_paths(adj, nodes, room_limit=2))
    print()


if __name__ == "__main__":
    solve("inputs/12.test1")
    solve("inputs/12.test2")
    solve("inputs/12.test3")
    solve("inputs/12.full")
