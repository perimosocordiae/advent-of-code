#!/usr/bin/env python3
import operator
import sympy
from typing import Callable

OPS = {
    "+": operator.add,
    "-": operator.sub,
    "*": operator.mul,
    "/": operator.truediv,
}


class Monkey:
    num: int | None = None
    a: str
    op: Callable[[int, int], int] | None
    b: str

    @staticmethod
    def from_str(line: str) -> tuple[str, "Monkey"]:
        name, rhs = line.strip().split(": ")
        out = Monkey()
        try:
            out.num = int(rhs)
        except ValueError:
            out.a, op, out.b = rhs.split()
            out.op = OPS[op]
        return name, out

    def __call__(self, monkeys: dict[str, "Monkey"]) -> int:
        if self.num is not None:
            return self.num
        assert self.op is not None
        return self.op(monkeys[self.a](monkeys), monkeys[self.b](monkeys))


def part1(path: str) -> None:
    monkeys = dict(Monkey.from_str(line) for line in open(path))
    print("Part 1:", int(monkeys["root"](monkeys)))

    monkeys["humn"].num = x = sympy.symbols("x", real=True)
    ma = monkeys["root"].a
    mb = monkeys["root"].b
    lhs = monkeys[ma](monkeys)
    rhs = monkeys[mb](monkeys)
    (soln,) = sympy.solve(lhs - rhs, x)
    print("Part 2:", round(soln))


if __name__ == "__main__":
    part1("inputs/21.full")
