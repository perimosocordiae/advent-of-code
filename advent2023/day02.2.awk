
BEGIN { RS="[:,;] |\n" }

/Game / { g = $2; next }

/ red/ { if (red[g] < $1) { red[g] = $1 } }
/ green/ { if (green[g] < $1) { green[g] = $1 } }
/ blue/ { if (blue[g] < $1) { blue[g] = $1 } }

END {
  s = 0;
  for (g in red) {
    s += red[g] * green[g] * blue[g];
  }
  print "Part 2:", s;
}
