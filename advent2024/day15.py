import numpy as np

EMPTY = 0
WALL = 1
BOX = 2
ROBOT = 3
LBOX = 4
RBOX = 5
CELL_CHARS = ".#O@[]"


def main():
    infile = "inputs/15.full"
    grid, directions, start = _parse_input(infile)

    # Part 1
    grid1 = grid.copy()
    pos = start
    for d in directions:
        pos = _move1(grid1, pos, d)
    print("Part 1:", _score(grid1))

    # Convert grid to double-width
    grid2 = np.zeros((grid.shape[0], grid.shape[1] * 2), dtype=int)
    for i, row in enumerate(grid):
        for j, cell in enumerate(row):
            if cell == WALL:
                grid2[i, j * 2 : j * 2 + 2] = WALL
            elif cell == BOX:
                grid2[i, j * 2 : j * 2 + 2] = LBOX, RBOX
            elif cell == ROBOT:
                grid2[i, j * 2] = ROBOT
    # Part 2
    bbox = start[0], start[1] * 2, start[1] * 2 + 1
    if infile.endswith(".test"):
        _display(grid2)
        while ch := input().strip():
            d = {"w": "^", "s": "v", "a": "<", "d": ">"}[ch]
            print("Move", d)
            bbox = _move2(grid2, bbox, d)
            _display(grid2)
    else:
        for d in directions:
            bbox = _move2(grid2, bbox, d)
        print("Part 2:", _score(grid2))


def _score(grid) -> int:
    score = 0
    for i, row in enumerate(grid):
        for j, cell in enumerate(row):
            if cell == BOX or cell == LBOX:
                score += 100 * i + j
    return score


def _display(grid) -> None:
    for row in grid:
        print("".join(CELL_CHARS[cell] for cell in row))


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


def _move1(grid, pos, d) -> tuple[int, int]:
    npos = _step(pos, d)
    ncell = grid[npos]
    if ncell == WALL:
        return pos
    if ncell == EMPTY or _move1(grid, npos, d) != npos:
        grid[npos], grid[pos] = grid[pos], EMPTY
        return npos
    return pos


def _step2(bbox, d) -> tuple[int, int, int]:
    i, j1, j2 = bbox
    if d == "^":
        return i - 1, j1, j2
    elif d == "v":
        return i + 1, j1, j2
    elif d == "<":
        return i, j1 - 1, j2 - 1
    elif d == ">":
        return i, j1 + 1, j2 + 1
    raise ValueError(f"Invalid direction: {d}")


def _move2(grid, bbox, d) -> tuple[int, int, int]:
    nbbox = _step2(bbox, d)
    ni, nj1, nj2 = nbbox
    ncells = grid[ni, nj1:nj2]
    if (ncells == WALL).any():
        return bbox
    i, j1, j2 = bbox
    if (ncells == EMPTY).all():
        tmp = grid[i, j1:j2].copy()
        grid[i, j1:j2] = EMPTY
        grid[ni, nj1:nj2] = tmp
        return nbbox
    if d in "<>":
        if _move2(grid, nbbox, d) != nbbox:
            tmp = grid[i, j1:j2].copy()
            grid[i, j1:j2] = EMPTY
            grid[ni, nj1:nj2] = tmp
            return nbbox
        return bbox
    # Adjust the bbox if needed
    lpad, rpad = 0, 0
    if ncells[0] == RBOX:
        lpad += 1
    elif ncells[0] == EMPTY:
        lpad -= (ncells != EMPTY).argmax()
    if ncells[-1] == LBOX:
        rpad += 1
    elif ncells[-1] == EMPTY:
        rpad -= (ncells[::-1] != EMPTY).argmax()
    nnbbox = ni, nj1 - lpad, nj2 + rpad
    if _move2(grid, nnbbox, d) != nnbbox:
        tmp = grid[i, j1:j2].copy()
        grid[i, j1:j2] = EMPTY
        grid[ni, nj1 - lpad : nj1] = EMPTY
        grid[ni, nj1:nj2] = tmp
        grid[ni, nj2 : nj2 + rpad] = EMPTY
        return nbbox
    return bbox


def _parse_input(path: str):
    grid = []
    directions = []
    in_grid = True
    for line in open(path):
        line = list(line.strip())
        if not line:
            in_grid = False
            continue
        if in_grid:
            grid.append(line)
        else:
            directions.extend(line)
    grid_shape = len(grid), len(grid[0])
    arr = np.zeros(grid_shape, dtype=int)
    start = None
    for i, row in enumerate(grid):
        for j, val in enumerate(row):
            if val == "#":
                arr[i, j] = WALL
            elif val == "O":
                arr[i, j] = BOX
            elif val == "@":
                arr[i, j] = ROBOT
                start = (i, j)
    return arr, directions, start


if __name__ == "__main__":
    main()
