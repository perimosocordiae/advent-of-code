#!/bin/bash

infile=inputs/02.full

awk '
BEGIN { FS="[: ]" }
/[2-9][0-9] / {next}
/1[3-9] red/ {next}
/1[4-9] green/ {next}
/1[5-9] blue/ {next}
{s+=$2}
END {print "Part 1:", s}
' $infile

awk '
BEGIN { RS="[:,;] |\n" }
/Game / {g=$2; next}
/ red/ { if (red[g] < $1) { red[g] = $1 } }
/ green/ { if (green[g] < $1) { green[g] = $1 } }
/ blue/ { if (blue[g] < $1) { blue[g] = $1 } }
END {
  s=0;
  for (g in red) {
    s += red[g] * green[g] * blue[g];
  }
  print "Part 2:", s;
}
' $infile
