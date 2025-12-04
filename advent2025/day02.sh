#!/bin/bash
set -euo pipefail
# Usage:
#   ./day02.sh inputs/02.test
#   ./day02.sh inputs/02.full

infile=${1:-'inputs/02.full'}

# Make one range of IDs per line, space separated,
# then expand the ranges to one ID per line.
tr ',-' '\n ' < "$infile" | awk '
{
  start = int($1);
  end = int($2);
  for (i = start; i <= end; i++) {
    print i;
  }
}
' | grep -xP '(\d{1,})\1+' | awk '
BEGIN { part1 = 0; part2 = 0; }
{
  part2 += $0;
  len = length($0);
  if (len % 2 == 0) {
    half = len / 2;
    first = substr($0, 1, half);
    second = substr($0, half + 1, half);
    if (first == second) {
      part1 += $0;
    }
  }
}
END {
  print "Part 1: " part1;
  print "Part 2: " part2;
}
'
