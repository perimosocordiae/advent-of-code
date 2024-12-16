import heapq
import networkx as nx

LEFT = {"^": "<", "<": "v", "v": ">", ">": "^"}
RIGHT = {"^": ">", ">": "v", "v": "<", "<": "^"}


def main(infile="inputs/16.full") -> None:
    grid, start, stop = _parse_input(infile)
    cost = _shortest_path_cost(grid, stop, (start, ">"))
    print("Part 1:", cost)
    print("Part 2:", part2(grid, stop, (start, ">"), cost))


def part2(grid, stop, initial, max_cost) -> int:
    # TODO: Write this properly instead of using networkx
    G = nx.DiGraph()
    # DFS with a cost limit
    queue = [(0, *initial)]
    visited = dict()
    finals = []
    while queue:
        cost, pos, facing = queue.pop()
        if cost == max_cost and pos == stop:
            finals.append((pos, facing))
            continue
        if cost >= max_cost or visited.get((pos, facing), max_cost + 1) <= cost:
            continue
        visited[(pos, facing)] = cost
        # Move forward
        npos = _step(pos, facing)
        if grid[npos[0]][npos[1]] != "#":
            queue.append((cost + 1, npos, facing))
            G.add_edge((pos, facing), (npos, facing), weight=1)
        if cost + 1000 < max_cost:
            # Turn left
            queue.append((cost + 1000, pos, LEFT[facing]))
            G.add_edge((pos, facing), (pos, LEFT[facing]), weight=1000)
            # Turn right
            queue.append((cost + 1000, pos, RIGHT[facing]))
            G.add_edge((pos, facing), (pos, RIGHT[facing]), weight=1000)
    cells = set()
    for target in finals:
        for path in nx.all_shortest_paths(G, initial, target, weight="weight"):
            for pos, facing in path:
                cells.add(pos)
    return len(cells)


def _shortest_path_cost(grid, stop, initial) -> int:
    # Dijkstra's algorithm
    queue = [(0, *initial)]
    visited = set()
    while queue:
        cost, pos, facing = heapq.heappop(queue)
        if pos == stop:
            return cost
        if (pos, facing) in visited:
            continue
        visited.add((pos, facing))
        # Move forward
        npos = _step(pos, facing)
        if grid[npos[0]][npos[1]] != "#":
            heapq.heappush(queue, (cost + 1, npos, facing))
        # Turn left
        heapq.heappush(queue, (cost + 1000, pos, LEFT[facing]))
        # Turn right
        heapq.heappush(queue, (cost + 1000, pos, RIGHT[facing]))


def _step(pos, d) -> tuple[int, int]:
    i, j = pos
    if d == "^":
        return i - 1, j
    elif d == "v":
        return i + 1, j
    elif d == "<":
        return i, j - 1
    elif d == ">":
        return i, j + 1
    raise ValueError(f"Invalid direction: {d}")


def _parse_input(path: str):
    grid = []
    start = None
    stop = None
    for i, line in enumerate(open(path)):
        grid.append(line.strip())
        try:
            j = grid[-1].index("S")
            start = i, j
        except ValueError:
            pass
        try:
            j = grid[-1].index("E")
            stop = i, j
        except ValueError:
            pass
    return grid, start, stop


if __name__ == "__main__":
    main()
