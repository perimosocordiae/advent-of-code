#!/usr/bin/env python3
elves = open("inputs/01.full").read().split("\n\n")
totals = sorted(sum(int(x.strip()) for x in elf.split()) for elf in elves)
print("Part 1:", totals[-1])
print("Part 2:", sum(totals[-3:]))
