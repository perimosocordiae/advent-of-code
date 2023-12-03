#!/usr/bin/env python3
import numpy as np
from scipy.ndimage import binary_dilation, grey_dilation, label

infile = "inputs/03.full"
chars = np.array([list(x.strip()) for x in open(infile)], dtype="|U1")
non_symbols = np.array(list(".1234567890"))
symbols = np.setdiff1d(np.unique(chars.flat), non_symbols, assume_unique=True)
connect_diag = np.ones((3, 3))
connect_line = np.array([[0, 0, 0], [1, 1, 1], [0, 0, 0]])
adj_mask = binary_dilation(np.isin(chars, symbols), connect_diag)
num_mask = np.isin(chars, non_symbols[1:])
valid_mask = binary_dilation(
    num_mask & adj_mask, connect_line, iterations=2, mask=num_mask
)

tmp = np.full_like(chars, " ")
tmp[valid_mask] = chars[valid_mask]
print("Part 1:", sum(int(x) for x in "".join(tmp.flat).split()))

rr, cc = np.where(chars == "*")
star_ids = np.full_like(chars, -1, dtype=int)
star_ids[rr, cc] = np.arange(len(rr))
grey_dilation(star_ids, (3, 3), output=star_ids)
num_ids, _ = label(num_mask, connect_line)
n = chars.shape[0]
off = np.array([-1, 0, 1])
star_idx = off[:, None] * n + off + (rr * n + cc)[:, None, None]
patches = np.take(num_ids, star_idx).reshape((-1, 9))
# TODO: find a vectorized approach for this part.
part2 = 0
for p in patches:
    nbr_nums = np.unique(p)
    if len(nbr_nums) == 3:
        assert nbr_nums[0] == 0
        n1 = int("".join(chars[num_ids == nbr_nums[1]]))
        n2 = int("".join(chars[num_ids == nbr_nums[2]]))
        part2 += n1 * n2
print("Part 2:", part2)
