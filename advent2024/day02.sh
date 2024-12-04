#!/bin/bash
set -euo pipefail
# Usage:
#   ./day02.sh inputs/02.test
#   ./day02.sh inputs/02.full

infile=${1:-'inputs/02.full'}

echo "Part 1:"
# Compute pairwise differences between numbers in each line
awk '{s=""; for(i=2;i<=NF;i++){s=s""($i - $(i-1))" "} print s }' "$infile" |
 # Drop any line containing a zero
 grep -v -w 0 |
 # Count the number of lines with either all positive or all negative diffs,
 # where all diffs are +/- 1, 2, or 3
 grep -E -x -c '(-[123] )+|([123] )+'

echo
echo "Part 2: TODO"
