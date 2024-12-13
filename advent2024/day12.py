import numpy as np
from scipy.ndimage import find_objects, label
from scipy.signal import convolve2d

chars = np.array(
    [[ord(ch) - 64 for ch in line.strip()] for line in open("inputs/12.full")],
    dtype=int,
)
plus = np.array([[0, 1, 0], [1, 0, 1], [0, 1, 0]], dtype=int)
part1 = 0
for i, bbox in enumerate(find_objects(chars), start=1):
    if bbox is None:
        continue
    name = chr(i + 64)
    labels, num_labels = label(chars[bbox] == i)
    for j in range(1, num_labels + 1):
        mask = labels == j
        area = np.count_nonzero(mask)
        c = convolve2d(mask, plus, mode="full")
        tmp = 4 - c[1:-1, 1:-1]
        tmp[~mask] = 0
        perim = int(tmp.sum())
        # print(f"{name}({j}): {area=}\t{perim=}")
        part1 += area * perim
print("Part 1:", part1)
