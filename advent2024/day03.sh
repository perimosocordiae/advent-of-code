#!/bin/bash
set -euo pipefail
# Usage:
#   ./day03.sh inputs/03.test
#   ./day03.sh inputs/03.full

infile=${1:-'inputs/03.full'}

echo "Part 1:"
grep -o 'mul([0-9]\+,[0-9]\+)' "$infile" \
 | tr ',mul' '*   ' | paste -s -d+ | bc

echo
echo "Part 2:"
grep -E -o "(mul|do|don't)\(([0-9]+,[0-9]+)?\)" "$infile" \
| tr ',mul' '*   ' | awk '
BEGIN { on=1; }
/^don/ { on=0; }
/^do\(/ { on=1; }
/^ / { if (on) { print $0; } }' | paste -s -d+ | bc