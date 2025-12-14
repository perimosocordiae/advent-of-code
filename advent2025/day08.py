#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.12"
# dependencies = [
#     "numpy",
#     "scipy",
#     "networkx",
# ]
# ///

import networkx as nx
import numpy as np
from scipy.spatial.distance import pdist, squareform

pts = np.loadtxt('inputs/08.full',  delimiter=',')
n = 1000
dists = pdist(pts)
idx = np.argsort(dists)

# Part 1
tmp_dists = dists.copy()
tmp_dists[idx[n:]] = 0
g = nx.from_numpy_array(squareform(tmp_dists))
cc_sizes = [len(c) for c in nx.connected_components(g)]
cc_sizes.sort(reverse=True)
part1 = int(np.prod(cc_sizes[:3]))
print('Part 1:', part1)

# Part 2
r, c = np.triu_indices(len(pts), k=1)
for i in idx[n:]:
  d = dists[i]
  a, b = r[i], c[i]
  g.add_edge(a, b, weight=d)
  if nx.is_connected(g):
    xs = pts[[a,b], 0]
    print('Part 2:', int(np.prod(xs)))
    break