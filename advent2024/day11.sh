#!/bin/bash
set -euo pipefail
# Usage:
#   ./day11.sh inputs/11.test
#   ./day11.sh inputs/11.full

# Adapted from https://unix.stackexchange.com/a/113607
function repiper() { 
    local -i n="$1";
    shift;
    if (( n > 1 )); then
        "$@" | repiper $((n-1)) "$@";
    else
        "$@";
    fi
}

infile=${1:-'inputs/11.full'}
# shellcheck disable=SC2016
awkprog='
/^0$/ {
  print "1"
  next
}
/^(..)+$/ {
  lhs = int(substr($0, 1, length($0)/2))
  rhs = int(substr($0, length($0)/2+1))
  printf("%d\n%d\n", lhs, rhs)
  next
}
{
  printf("%.0f\n", int($0) * 2024)
}
'

echo "Part 1:"
tr ' ' '\n' < "$infile" | repiper 25 awk "$awkprog" | wc -l