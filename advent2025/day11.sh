#!/bin/bash
set -euo pipefail
# Usage:
#   ./day11.sh inputs/11.test
#   ./day11.sh inputs/11.full

infile=${1:-'inputs/11.full'}

# Determine topological order
order=$(awk '{
  src = substr($1, 1, length($1)-1);
  for (i = 2; i <= NF; i++) {
    print src, $i;
  }
}' "$infile" \
| tsort \
| paste -sd':' )

# Part 1: Count paths from 'you' to 'out'
awk -v order_str="$order" '
BEGIN {
  split(order_str, order, ":");
  n = length(order);
  paths["you"] = 1;
}
{
  split($0, parts, ": ");
  src = parts[1];
  adj[src] = parts[2];
}
END {
  for (i=1; i<=n; i++) {
    node = order[i];
    p = paths[node];
    split(adj[node], neighbors, " ");
    for (j in neighbors) {
      nbr = neighbors[j];
      paths[nbr] += p;
    }
  }
  print "Part 1:", paths["out"];
}
' "$infile"
