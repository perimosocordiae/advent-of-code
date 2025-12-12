#!/bin/bash
set -euo pipefail
# Usage:
#   ./day06.sh inputs/06.test
#   ./day06.sh inputs/06.full

infile=${1:-'inputs/06.full'}

echo "Part 1:"
awk '{
  for (i=1; i<=NF; i++) {
    print i, $i;
  }
}' "$infile" | sort -n | awk '
{ lines[$1] = lines[$1] $2 " "; }
END {
  for (i in lines) {
    print lines[i];
  }
}
' | awk '
{
  oper = $1;
  res = (oper == "+") ? 0 : 1;
  for (i = 2; i <= NF; i++) {
    if (oper == "+") {
      res += $i;
    } else if (oper == "*") {
      res *= $i;
    }
  }
  print res;
}
' | paste -sd+ - | bc

echo
echo "Part 2:"
awk '{
  n = length($0);
  for (i=1; i<=n; i++) {
    print i, NR, substr($0, i, 1);
  }
}' "$infile" \
 | sort -n \
 | awk '
{ lines[$1] = lines[$1] $3; }
END {
  for (i in lines) {
    print i, lines[i];
  }
}' \
 | sort -n \
 | awk '
BEGIN { op = ""; total = 0; num = 0; }
$2 ~ /[0-9]+[*+]/ {
  op = substr($2, length($2), 1);
  num = substr($2, 1, length($2)-1);
  next
}
$2 ~ /[0-9]+/ {
  if (op == "+") {
    num += $2;
  } else if (op == "*") {
    num *= $2;
  } else {
    print "Unknown operator " op > "/dev/stderr";
    exit 1;
  }
  next
}
{ total += num; num = 0; op = ""; }
END { print total + num; }
'