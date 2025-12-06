#!/bin/bash
set -euo pipefail
# Usage:
#   ./day05.sh inputs/05.test
#   ./day05.sh inputs/05.full

infile=${1:-'inputs/05.full'}

awk -F'-' '
BEGIN { idx = 0; ct = 0; }
/-/ {
  starts[idx] = $1;
  ends[idx] = $2;
  idx++;
}
/^[0-9]+$/ {
  for (i = 0; i < idx; i++) {
    if ($0 >= starts[i] && $0 <= ends[i]) {
      ct++;
      next;
    }
  }
}
END {
  print "Part 1:", ct;
}
' "$infile"

grep '-' "$infile" | tr '-' ' ' | sort -n | awk '
BEGIN { ct = 0; prev_start = -1; prev_end = -2; }
{
  start = $1;
  end = $2;
  if (start > prev_end) {
    ct += prev_end - prev_start + 1;
    prev_start = start;
    prev_end = end;
  } else if (end > prev_end) {
    prev_end = end;
  }
}
END {
  ct += prev_end - prev_start + 1;
  print "Part 2:", ct;
}
'