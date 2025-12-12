#!/bin/bash
set -euo pipefail
# Usage:
#   ./day07.sh inputs/07.test
#   ./day07.sh inputs/07.full

infile=${1:-'inputs/07.full'}

awk '
/S/ {
  pos = index($0, "S");
  beams[pos] = 1;
  num_splits = 0;
  next
}
/\^/ {
  pos = 0
  while (match(substr($0, pos + 1), /\^/)) {
    pos += RSTART
    if (beams[pos]) {
      beams[pos - 1] += beams[pos]
      beams[pos + 1] += beams[pos]
      beams[pos] = 0
      num_splits++
    }
  }
}
END {
  print "Part 1:", num_splits;
  total = 0
  for (b in beams) {
    total += beams[b]
  }
  print "Part 2:", total;
}
' "$infile"
