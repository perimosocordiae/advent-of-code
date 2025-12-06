#!/bin/bash
set -euo pipefail
# Usage:
#   ./day06.sh inputs/06.test
#   ./day06.sh inputs/06.full

infile=${1:-'inputs/06.full'}

awk '{
  for (i=1; i<=NF; i++) {
    print i, $i;
  }
}' "$infile" | sort -n | awk '
{ lines[$1] = lines[$1] $2 " "; }
END {
  for (i in lines) {
    print lines[i];
  }
}
' | awk '
{
  oper = $1;
  res = (oper == "+") ? 0 : 1;
  for (i = 2; i <= NF; i++) {
    if (oper == "+") {
      res += $i;
    } else if (oper == "*") {
      res *= $i;
    }
  }
  print res;
}
' | paste -sd+ - | bc