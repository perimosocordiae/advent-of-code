#!/bin/bash
set -euo pipefail
# Usage:
#   ./day19.sh inputs/19.test
#   ./day19.sh inputs/19.full

infile=${1:-'inputs/19.full'}
patt=$(head -1 "$infile" | tr -s ', ' '|')
echo "Part 1:"
tail +3 "$infile" | grep -Exc "($patt)+"
