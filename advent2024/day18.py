import heapq


def main(infile="inputs/18.full") -> None:
    # grid, coords = _parse_input(infile, 12, 7)
    grid, coords = _parse_input(infile, 1024, 71)
    stop = (len(grid) - 1, len(grid[0]) - 1)
    visited = _shortest_path(grid, stop)
    print("Part 1:", visited[stop][0])

    path = set(_recover_path(visited, stop))
    for x, y in coords:
        grid[x][y] = "#"
        if (x, y) not in path:
            continue
        visited = _shortest_path(grid, stop)
        if not visited:
            print(f"Part 2: {x},{y}")
            break
        path = set(_recover_path(visited, stop))


def _recover_path(visited, stop):
    pos = stop
    while pos:
        yield pos
        _, pos = visited[pos]


def _shortest_path(grid, stop, max_cost=float("inf")):
    # Dijkstra's algorithm
    queue = [(0, (0, 0))]
    visited = {(0, 0): (0, None)}
    while queue:
        cost, pos = heapq.heappop(queue)
        if pos == stop:
            return visited
        for npos in _nexts(grid, pos):
            pcost, _ = visited.get(npos, (max_cost, None))
            if cost + 1 < pcost:
                visited[npos] = (cost + 1, pos)
                heapq.heappush(queue, (cost + 1, npos))
    return None


def _nexts(grid, pos):
    # down
    if pos[0] < len(grid) - 1 and grid[pos[0] + 1][pos[1]] != "#":
        yield pos[0] + 1, pos[1]
    # right
    if pos[1] < len(grid[0]) - 1 and grid[pos[0]][pos[1] + 1] != "#":
        yield pos[0], pos[1] + 1
    # up
    if pos[0] > 0 and grid[pos[0] - 1][pos[1]] != "#":
        yield pos[0] - 1, pos[1]
    # left
    if pos[1] > 0 and grid[pos[0]][pos[1] - 1] != "#":
        yield pos[0], pos[1] - 1


def _parse_input(path, max_lines, grid_size) -> list[list[str]]:
    coords = []
    for line in open(path):
        x, y = map(int, line.strip().split(","))
        coords.append((x, y))
    grid = [list("." * grid_size) for _ in range(grid_size)]
    for x, y in coords[:max_lines]:
        grid[x][y] = "#"
    return grid, coords[max_lines:]


if __name__ == "__main__":
    main()
