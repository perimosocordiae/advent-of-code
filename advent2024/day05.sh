#!/bin/bash
set -euo pipefail
# Usage:
#   ./day05.sh inputs/05.test
#   ./day05.sh inputs/05.full

infile=${1:-'inputs/05.full'}

grep '|' "$infile" | tr '|' ' ' >order.tmp
echo "Part 1:"
grep ',' "$infile" | tr ',' '|' | while read -r line; do
  replacements=$(grep -E "($line) ($line)" order.tmp |
    tsort | nl -v0 |
    awk '{print "s/\\b"$2"\\b/_"$1"/g"}' | sort | paste -sd';')
  if echo "$line" | sed "$replacements" | tr '|_' '\n ' | sort -nuC ; then
    echo "$line" | awk 'BEGIN{FS="|";} { print $(NF/2+1); }'
  fi
done | paste -sd+ | bc
rm order.tmp
