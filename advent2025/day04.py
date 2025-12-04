#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.12"
# dependencies = [
#     "numpy",
#     "scipy",
# ]
# ///

import numpy as np
from scipy.signal import convolve2d

lines = np.loadtxt('inputs/04.full',  dtype=str)
shape = (lines.shape[0], len(lines[0]))
mtx = lines.view(dtype=np.int32).reshape(shape) // ord('@')

patch = np.ones((3, 3), dtype=int)
patch[1, 1] = 0
c = convolve2d(mtx, patch, mode='same')
removable = mtx & (c < 4)
print("Part 1:", removable.sum())

next = mtx.copy()
while removable.any():
  next -= removable
  c = convolve2d(next, patch, mode='same')
  removable = next & (c < 4)
print("Part 2:", mtx.sum() - next.sum())
