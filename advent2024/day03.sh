#!/bin/bash
set -euo pipefail
# Usage:
#   ./day03.sh inputs/03.test
#   ./day03.sh inputs/03.full

infile=${1:-'inputs/03.full'}

echo "Part 1:"
# Select all valid mul(a,b) expressions
grep -o 'mul([0-9]\+,[0-9]\+)' "$infile" |
 # Replace mul(a,b) with (a*b)
 tr ',mul' '*   ' |
 # Sum all the products
 paste -s -d+ | bc

echo
echo "Part 2:"
# Select all mul(a,b), do(), and don't() expressions
grep -E -o "(mul|do|don't)\(([0-9]+,[0-9]+)?\)" "$infile" |
 # Replace mul(a,b) with (a*b)
 tr ',mul' '*   ' |
 # Drop lines suppressed by don't()
 awk '
BEGIN { on=1; }
/^don/ { on=0; }
/^do\(/ { on=1; }
/^ / { if (on) { print $0; } }' |
 # Sum all the remaining products
 paste -s -d+ | bc
