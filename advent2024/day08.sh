#!/bin/bash
set -euo pipefail
# Usage:
#   ./day08.sh inputs/08.test
#   ./day08.sh inputs/08.full

infile=${1:-'inputs/08.full'}

# Square grid size
grid_size=$(wc -w "$infile" | cut -d' ' -f1)

function solve() {
  loop_bounds=$1
  # Get byte positions of each antenna
  grep -E -b -o '[^.]' "$infile" |
  # Group by antenna code
  awk -F: '{ arr[$2]=arr[$2] " " $1; }
  END { for (idx in arr) print arr[idx]; }' |
  # For all combinations of pairs within each antenna code,
  # compute the next point in the line.
  awk -v nc="$((grid_size+1))" "
  {
    for (i=1; i<=NF; i++) {
      ri = int(\$i / nc);
      ci = \$i % nc;
      for (j=1; j<=NF; j++) {
        if (i != j) {
          rj = int(\$j / nc);
          cj = \$j % nc;
          dr = rj - ri;
          dc = cj - ci;
          for ($loop_bounds; k++) {
            print rj + k*dr, cj + k*dc;
          }
        }
      }
    }
  }" |
  # Drop negative values
  grep -v '-' |
  # Drop out-of-bounds values
  awk -v n="$grid_size" '$1 < n && $2 < n { print $1, $2; }' |
  # Count unique points
  sort -n | uniq | wc -l
}

echo -n "Part 1: "
solve 'k=1; k<2'
echo -n "Part 2: "
solve 'k=0; k<nc'
