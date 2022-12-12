#!/usr/bin/env python3
from typing import Callable
from dataclasses import dataclass
from operator import add, mul, floordiv, mod
from math import prod

OPERATORS = {"+": add, "*": mul}


@dataclass
class Monkey:
    items: list[int]
    operation: Callable[[int], int]
    relief: Callable[[int], int]
    divisble_by: int = 0
    true_index: int = 0
    false_index: int = 0
    num_inspections: int = 0

    def __init__(self) -> None:
        self.items = []
        self.operation = lambda x: x
        self.relief = lambda x: x

    def take_turn(self, monkeys: list["Monkey"]) -> None:
        for item in self.items:
            item = self.operation(item)
            item = self.relief(item)
            if item % self.divisble_by == 0:
                monkeys[self.true_index].items.append(item)
            else:
                monkeys[self.false_index].items.append(item)
        self.num_inspections += len(self.items)
        del self.items[:]


def parse_input(path: str) -> list[Monkey]:
    monkeys = []
    for line in open(path):
        if line.startswith("Monkey"):
            monkeys.append(Monkey())
        elif line.startswith("  Starting items: "):
            monkeys[-1].items.extend(map(int, line[18:].strip().split(", ")))
        elif line.startswith("  Operation: new = old "):
            op, rhs = line[22:].strip().split()
            func = OPERATORS[op]
            if rhs == "old":
                monkeys[-1].operation = lambda x, func=func: func(x, x)
            else:
                y = int(rhs)
                monkeys[-1].operation = lambda x, y=y, func=func: func(x, y)
        elif line.startswith("  Test: divisible by "):
            monkeys[-1].divisble_by = int(line[21:].strip())
        elif line.startswith("    If true: throw to monkey "):
            monkeys[-1].true_index = int(line[29:].strip())
        elif line.startswith("    If false: throw to monkey "):
            monkeys[-1].false_index = int(line[30:].strip())
    return monkeys


def part1(path: str) -> None:
    monkeys = parse_input(path)
    for monkey in monkeys:
        monkey.relief = lambda x: floordiv(x, 3)
    for _ in range(20):
        for monkey in monkeys:
            monkey.take_turn(monkeys)
    most_active = sorted(m.num_inspections for m in monkeys)[-2:]
    print("Part 1:", prod(most_active))


def part2(path: str) -> None:
    monkeys = parse_input(path)
    common = prod(m.divisble_by for m in monkeys)
    for monkey in monkeys:
        monkey.relief = lambda x: mod(x, common)
    for _ in range(1, 10001):
        for monkey in monkeys:
            monkey.take_turn(monkeys)
    most_active = sorted(m.num_inspections for m in monkeys)[-2:]
    print("Part 2:", prod(most_active))


if __name__ == "__main__":
    part1("inputs/11.full")
    part2("inputs/11.full")
