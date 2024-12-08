import numpy as np

RIGHT_TURNS = {"^": ">", ">": "v", "v": "<", "<": "^"}


def slice_line(grid, i, j, direction):
    if direction == "^":
        return grid[: i + 1, j]
    elif direction == ">":
        return grid[i, j:]
    elif direction == "v":
        return grid[i:, j]
    elif direction == "<":
        return grid[i, : j + 1]


def print_grid(grid):
    for row in grid:
        print("".join(row))
    print()


def _unpaired_value(a: int, b: int, c: int) -> int:
    if a == b:
        return c
    if a == c:
        return b
    assert b == c
    return a


# Look back at the last 3 spots to calculate the next spot that would form a
# rectangle with the last 3. If that spot doesn't cross an existing line,
# count it, otherwise move on.
def _can_make_loop(grid, positions) -> int:
    points = positions[-3:]
    rs, cs = zip(*points)
    i, j = points[-1]
    ni, nj = _unpaired_value(*rs), _unpaired_value(*cs)
    if i == ni:
        line = grid[i, min(j, nj) : max(j, nj) + 1]
    else:
        line = grid[min(i, ni) : max(i, ni) + 1, j]
    return int((line[1:-1] == ".").all())


infile = "inputs/06.full"
grid = np.array([list(line.strip()) for line in open(infile)])

direction = "^"
i, j = [a.item(0) for a in np.where(grid == "^")]

keep_going = True
positions = [(i, j)]
part2_count = 0
while keep_going:
    line = slice_line(grid, i, j, direction)
    (idxs,) = np.where(line == "#")
    if len(idxs) == 0:
        idxs = [max(grid.shape)] if direction in "v>" else [-1]
        keep_going = False
    if direction == "^":
        stop = idxs[-1] + 1
        line[stop:] = "X"
        i = stop
    elif direction == ">":
        stop = idxs[0]
        line[:stop] = "X"
        j += stop - 1
    elif direction == "v":
        stop = idxs[0]
        line[:stop] = "X"
        i += stop - 1
    else:
        stop = idxs[-1] + 1
        line[stop:] = "X"
        j = stop
    if not keep_going:
        break
    positions.append((i, j))
    if len(positions) > 3:
        part2_count += _can_make_loop(grid, positions)
        # Also check if we could loop back to the start
        si, sj = positions[0]
        pj = positions[-2][1]
        if direction == "<" and i > si and j < sj and pj > sj:
            if (grid[si + 1 : i, sj] == ".").all():
                part2_count += 1
    direction = RIGHT_TURNS[direction]
print("Part 1:", np.count_nonzero(grid == "X"))
print("Part 2 (incorrect):", part2_count)
