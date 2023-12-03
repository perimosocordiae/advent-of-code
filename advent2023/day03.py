#!/usr/bin/env python3
import numpy as np
from scipy.ndimage import binary_dilation

infile = "inputs/03.full"
chars = np.array([list(x.strip()) for x in open(infile)], dtype="|U1")
non_symbols = np.array(list(".1234567890"))
symbols = np.setdiff1d(np.unique(chars.flat), non_symbols, assume_unique=True)
adj_mask = binary_dilation(np.isin(chars, symbols), np.ones((3, 3)))
num_mask = np.isin(chars, non_symbols[1:])
valid_mask = binary_dilation(
    num_mask & adj_mask, structure=np.ones((1, 3)), iterations=2, mask=num_mask
)

tmp = np.full_like(chars, " ")
tmp[valid_mask] = chars[valid_mask]
print("Part 1:", sum(int(x) for x in "".join(tmp.flat).split()))
