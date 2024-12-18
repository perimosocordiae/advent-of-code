import heapq
import itertools


def main(infile="inputs/18.full") -> None:
    grid = _parse_input(infile, 1024, 71)
    print("Part 1:", _shortest_path_cost(grid))


def _shortest_path_cost(grid) -> int:
    stop = (len(grid) - 1, len(grid[0]) - 1)
    # Dijkstra's algorithm
    queue = [(0, (0, 0))]
    visited = set()
    while queue:
        cost, pos = heapq.heappop(queue)
        if pos == stop:
            return cost
        if pos in visited:
            continue
        visited.add(pos)
        # down
        if pos[0] < len(grid) - 1 and grid[pos[0] + 1][pos[1]] != "#":
            heapq.heappush(queue, (cost + 1, (pos[0] + 1, pos[1])))
        # right
        if pos[1] < len(grid[0]) - 1 and grid[pos[0]][pos[1] + 1] != "#":
            heapq.heappush(queue, (cost + 1, (pos[0], pos[1] + 1)))
        # up
        if pos[0] > 0 and grid[pos[0] - 1][pos[1]] != "#":
            heapq.heappush(queue, (cost + 1, (pos[0] - 1, pos[1])))
        # left
        if pos[1] > 0 and grid[pos[0]][pos[1] - 1] != "#":
            heapq.heappush(queue, (cost + 1, (pos[0], pos[1] - 1)))


def _parse_input(path, max_lines, grid_size) -> list[list[str]]:
    grid = [list("." * grid_size) for _ in range(grid_size)]
    for line in itertools.islice(open(path), max_lines):
        x, y = map(int, line.strip().split(","))
        grid[y][x] = "#"
    return grid


if __name__ == "__main__":
    main()
