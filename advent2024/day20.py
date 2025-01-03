import collections
import numpy as np
import scipy.spatial.distance as ssd


def main(infile="inputs/20.full"):
    grid, _, stop = _parse(infile)

    # Flood fill from stop to build a matrix of costs
    costs = np.full_like(grid, grid.size, dtype=int)
    costs[stop] = 0
    queue = collections.deque([stop])
    while queue:
        r, c = queue.popleft()
        next_cost = costs[r, c] + 1
        for dr, dc in ((0, 1), (1, 0), (0, -1), (-1, 0)):
            nr, nc = r + dr, c + dc
            if grid[nr, nc] and costs[nr, nc] > next_cost:
                costs[nr, nc] = next_cost
                queue.append((nr, nc))

    # Count how many cheats would save at least 100 steps
    part1 = np.count_nonzero(_search(grid, costs) >= 100)
    part1 += np.count_nonzero(_search(grid.T, costs.T) >= 100)
    print("Part 1:", part1)

    part2 = _search_n(grid, costs)
    print("Part 2:", part2)


def _search_n(grid, costs, n=20, min_saved=100) -> int:
    # Consider all tiles reachable in n steps (manhattan distance)
    pts = np.column_stack(np.where(grid))
    l1dist = ssd.pdist(pts, "cityblock")
    l1dist[l1dist > n] = 0
    ii, jj = np.triu_indices(len(pts), k=1)
    mask = l1dist > 0
    starts = pts[ii[mask]]
    ends = pts[jj[mask]]
    saved = abs(costs[*starts.T] - costs[*ends.T]) - l1dist[mask]
    return np.count_nonzero(saved >= min_saved)


def _search(grid, costs):
    # Search for [True, False, True] patterns in grid
    windows = np.lib.stride_tricks.sliding_window_view(grid, 3, axis=1)
    ii, jj = np.where(windows @ (1, 2, 4) == 5)
    # Steps saved: difference in costs, minus 2 for stepping through the wall.
    return abs(costs[ii, jj + 2] - costs[ii, jj]) - 2


def _parse(infile: str):
    grid = []
    start, stop = None, None
    for r, line in enumerate(open(infile)):
        grid.append(row := [])
        for c, ch in enumerate(line.strip()):
            if ch == "S":
                start = (r, c)
            elif ch == "E":
                stop = (r, c)
            row.append(ch != "#")
    return np.array(grid, dtype=bool), start, stop


if __name__ == "__main__":
    main()
