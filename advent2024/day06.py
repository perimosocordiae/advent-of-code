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


infile = "inputs/06.full"
grid = np.array([list(line.strip()) for line in open(infile)])

direction = "^"
i, j = [a.item(0) for a in np.where(grid == "^")]

keep_going = True
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
    direction = RIGHT_TURNS[direction]
    grid[i, j] = direction
print("Part 1:", np.count_nonzero(grid == "X"))
