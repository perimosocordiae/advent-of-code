#!/bin/bash
set -euo pipefail
# Usage:
#   ./day01.sh inputs/01.test
#   ./day01.sh inputs/01.full

infile=${1:-'inputs/01.full'}

echo "Part 1:"
# Convert L to '-' and R to '', then cumsum from 50
# and count how many times we hit a multiple of 100.
tr 'LR' '- ' < "$infile" | awk '
BEGIN { x=50; ct=0; }
{ x += $0; if (x%100 == 0) { ct++; } }
END { print ct; }
'

echo
echo "Part 2:"
tr 'LR' '- ' < "$infile" | awk '
BEGIN { x=50; ct=0; }
{
  spins = int($0 / 100);
  ct += spins < 0 ? -spins : spins;
  tmp = x + ($0 - spins * 100);
  y = (tmp>=0) ? tmp % 100 : 100 + (tmp % 100);
  if (y == 0 || x != 0 && tmp != y) {
    ct++;
  }
  x = y;
}
END { print ct; }
'