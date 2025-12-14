#!/bin/bash
set -euo pipefail
# Usage:
#   ./day09.sh inputs/09.test
#   ./day09.sh inputs/09.full

infile=${1:-'inputs/09.full'}

join -j 999 "$infile" "$infile" | tr ',' ' ' | awk '
BEGIN { best = 0; }
{
  w = ($1 - $3);
  h = ($2 - $4);
  if (w < 0) { w = -w }
  if (h < 0) { h = -h }
  area = (w+1) * (h+1);
  if (area > best) { best = area; }
}
END { print "Part 1:", best; }
'
