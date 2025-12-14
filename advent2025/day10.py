#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.12"
# ///
import itertools

light_table = str.maketrans('.#', '01', '[]')
def parse_lights(s):
  return int(s.translate(light_table)[::-1], base=2)

def parse_button(s):
  idxs = map(int, s[1:-1].split(','))
  return sum(1 << i for i in idxs)

def min_presses(target, buttons):
  for n in range(1, len(buttons)+1):
    for combo in itertools.combinations(buttons, n):
      val = 0
      for b in combo:
        val ^= b
      if val == target:
        return n
  raise ValueError("No combination found")

part1 = 0
for line in open("inputs/10.full"):
  parts = line.strip().split()
  target = parse_lights(parts[0])
  buttons = list(map(parse_button, parts[1:-1]))
  part1 += min_presses(target, buttons)
print("Part 1:", part1)
