import numpy as np


def _checksum(layout) -> int:
    return sum(i * c for i, c in enumerate(layout) if c)


def part1(numbers) -> int:
    layout = []
    for i, (b, f) in enumerate(numbers):
        layout.extend([i] * b + [None] * f)
    try:
        idx = layout.index(None)
        while True:
            layout[idx] = layout.pop()
            idx = layout.index(None, idx)
    except ValueError:
        return _checksum(layout)


def part2(numbers) -> int:
    layout = []
    files = []
    freelist = []
    for i, (b, f) in enumerate(numbers):
        if b > 0:
            files.append((len(layout), int(b)))
            layout.extend([i] * b)
        if f > 0:
            freelist.append((len(layout), int(f)))
            layout.extend([None] * f)
    for pos, sz in files[::-1]:
        for i in range(len(freelist)):
            fpos, fsz = freelist[i]
            if fsz >= sz and fpos < pos:
                layout[fpos : fpos + sz] = layout[pos : pos + sz]
                layout[pos : pos + sz] = [None] * sz
                freelist[i] = (fpos + sz, fsz - sz)
                break
    return _checksum(layout)


infile = "inputs/09.full"
numbers = list(map(int, open(infile).read()))
if len(numbers) % 2 == 1:
    numbers.append(0)
numbers = np.array(numbers).reshape(-1, 2)
print("Part 1:", part1(numbers))
print("Part 2:", part2(numbers))
