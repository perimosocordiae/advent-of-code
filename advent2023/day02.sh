#!/bin/bash
# Usage:
#   ./day02.sh inputs/02.test
#   ./day02.sh inputs/02.full

infile=${1:-'inputs/02.full'}

awk -f day02.1.awk "$infile"
awk -f day02.2.awk "$infile"
