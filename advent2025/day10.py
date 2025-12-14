#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.12"
# dependencies = [
#     "numpy",
#     "scipy",
#     "ipython",
# ]
# ///
import itertools
import numpy as np
from scipy.optimize import linprog

light_table = str.maketrans('.#', '01', '[]')
def parse_lights(s):
  return int(s.translate(light_table)[::-1], base=2)

def parse_button_binary(s):
  idxs = map(int, s[1:-1].split(','))
  return sum(1 << i for i in idxs)

def min_presses_part1(target, buttons):
  for n in range(1, len(buttons)+1):
    for combo in itertools.combinations(buttons, n):
      val = 0
      for b in combo:
        val ^= b
      if val == target:
        return n
  raise ValueError("No combination found")

def parse_joltage(s):
  return np.array(list(map(int, s[1:-1].split(','))), dtype=int)

def parse_button_array(s, n):
  arr = np.zeros(n, dtype=int)
  arr[parse_joltage(s)] = 1
  return arr
  
def min_presses_part2(target, buttons):
  c = np.ones(buttons.shape[1])
  res = linprog(c, A_eq=buttons, b_eq=target, integrality=True)
  if res.success:
    return int(res.fun)
  raise ValueError("No solution found")


part1 = 0
part2 = 0
for line in open("inputs/10.full"):
  parts = line.strip().split()
  target = parse_lights(parts[0])
  buttons = list(map(parse_button_binary, parts[1:-1]))
  part1 += min_presses_part1(target, buttons)

  target = parse_joltage(parts[-1])
  buttons = [parse_button_array(b, len(target)) for b in parts[1:-1]]
  part2 += min_presses_part2(target, np.column_stack(buttons))

print("Part 1:", part1)
print("Part 2:", part2)
