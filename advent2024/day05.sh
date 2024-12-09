#!/bin/bash
set -euo pipefail
# Usage:
#   ./day05.sh inputs/05.test
#   ./day05.sh inputs/05.full

infile=${1:-'inputs/05.full'}

# Extract edge information to a temporary file.
grep '|' "$infile" | tr '|' ' ' >order.tmp

# Process lists of nodes, one per line.
grep ',' "$infile" | tr ',' '|' | while read -r line; do
  # Topologically sort the nodes present in the list.
  sort_order=$(grep -E "($line) ($line)" order.tmp | tsort)
  # Replace each node with its index in the topological sort.
  replacements=$(echo "$sort_order" | nl -v0 |
    awk '{print "s/\\b"$2"\\b/_"$1"/g"}' | sort | paste -sd';')
  # Check if the tsort order is valid (i.e., in sorted order).
  if echo "$line" | sed "$replacements" | tr '|_' '\n ' | sort -nuC ; then
    # If so, print the middle node.
    echo "$line" | awk -F'|' '{ print "Part1", $(NF/2+1); }'
  else
    # Determine the 1-based index of the middle node in the sequence.
    pipes="${line//[!|]/}"
    np="${#pipes}"
    mid=$(((np+3)/2))
    # Select the middle node from the sort order.
    echo "$sort_order" | sed -n "${mid}p"
  fi
done | awk '
/Part1/ { p1 += $2; }
!/Part1/ { p2 += $1; }
END { print "Part 1:", p1; print "Part 2:", p2; }
'

rm order.tmp
