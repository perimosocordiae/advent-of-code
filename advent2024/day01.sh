#!/bin/bash
set -euo pipefail
# Usage:
#   ./day01.sh inputs/01.test
#   ./day01.sh inputs/01.full

infile=${1:-'inputs/01.full'}

echo "Part 1:"
paste -d'-' \
 <(cut -f1 -d' ' "$infile" | sort -n) \
 <(cut -f4 -d' ' "$infile" | sort -n) \
| bc | sed 's/-//' | paste -s -d+ - | bc

echo
echo "Part 2:"
awk '{ x[$1]++; y[$2]++; }
END {
 for (i in x) { sum += x[i] * y[i] * i; }
 print sum;
}' "$infile"
