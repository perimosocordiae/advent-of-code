#!/bin/bash
set -euo pipefail
# Usage:
#   ./day03.sh inputs/03.test
#   ./day03.sh inputs/03.full

infile=${1:-'inputs/03.full'}

echo "Part 1:"
# For each line, find the largest and second largest digits,
# such that the first occurs before the second,
# then form a two-digit number from them and sum.
awk '
{
  len = length($0);
  max = 0;
  max_i = 0;
  for (i = 1; i < len; i++) {
    digit = int(substr($0, i, 1));
    if (digit > max) {
      max = digit;
      max_i = i;
    }
  }
  next_max = 0;
  for (i = max_i + 1; i <= len; i++) {
    digit = int(substr($0, i, 1));
    if (digit > next_max) {
      next_max = digit;
    }
  }
  print max * 10 + next_max;
}
' < "$infile" | paste -sd+ - | bc

echo
echo "Part 2:"
# Now make the largest 12-digit number from the given digits.
awk '
function largest_n_digit_num(s, n) {
  len = length(s);
  if (n == 0 || len == 0) {
    return "";
  }
  max_digit = -1;
  max_pos = -1;
  for (i = 1; i <= len - n + 1; i++) {
    digit = int(substr(s, i, 1));
    if (digit > max_digit) {
      max_digit = digit;
      max_pos = i;
    }
  }
  return sprintf("%d%s", max_digit, largest_n_digit_num(substr(s, max_pos + 1), n - 1));
}
{ print largest_n_digit_num($0, 12); }
' < "$infile" | paste -sd+ - | bc
