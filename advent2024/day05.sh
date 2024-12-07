#!/bin/bash
set -euo pipefail
# Usage:
#   ./day05.sh inputs/05.test
#   ./day05.sh inputs/05.full

infile=${1:-'inputs/05.full'}

# Extract edge information to a temporary file.
grep '|' "$infile" | tr '|' ' ' >order.tmp

echo "Part 1:"
# Process lists of nodes, one per line.
grep ',' "$infile" | tr ',' '|' | while read -r line; do
  # Replace each node with its index in the topological sort.
  replacements=$(grep -E "($line) ($line)" order.tmp |
    tsort | nl -v0 |
    awk '{print "s/\\b"$2"\\b/_"$1"/g"}' | sort | paste -sd';')
  # Check if the tsort order is valid (i.e., in sorted order).
  if echo "$line" | sed "$replacements" | tr '|_' '\n ' | sort -nuC ; then
    # If so, print the middle node.
    echo "$line" | awk 'BEGIN{FS="|";} { print $(NF/2+1); }'
  fi
done | paste -sd+ | bc

echo "Part 2:"
# Process lists of nodes, one per line.
grep ',' "$infile" | tr ',' '|' | while read -r line; do
  # Replace each node with its index in the topological sort.
  replacements=$(grep -E "($line) ($line)" order.tmp |
    tsort | nl -v0 |
    awk '{print "s/\\b"$2"\\b/_"$1"/g"}' | sort | paste -sd';')
  # Check if the tsort order is invalid (i.e., not in sorted order).
  if ! echo "$line" | sed "$replacements" | tr '|_' '\n ' | sort -nuC ; then
    # Determine the 1-based index of the middle node in the sequence.
    pipes="${line//[!|]/}"
    np="${#pipes}"
    mid=$(((np+3)/2))
    # Select the middle node from the sort order.
    grep -E "($line) ($line)" order.tmp | tsort | sed -n "${mid}p"
  fi
done | paste -sd+ | bc

rm order.tmp
