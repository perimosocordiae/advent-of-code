#!/usr/bin/env python3
from collections import deque


class Num:
    def __init__(self, line: str):
        self.val = int(line.strip())
        self.succ = None  # type: Num | None

    def __repr__(self) -> str:
        return f"{self.val}"


def parse_input(path: str) -> deque[Num]:
    nums = list(map(Num, open(path)))
    for i, num in enumerate(nums[:-1]):
        num.succ = nums[i + 1]
    return deque(nums)


def mix(nums: deque[Num], start: Num) -> None:
    num = start
    while num is not None:
        if num.val != 0:
            idx = nums.index(num)
            nums.rotate(-idx)
            nums.popleft()
            nums.rotate(-num.val)
            nums.appendleft(num)
        num = num.succ


def output(nums: deque[Num]) -> int:
    (idx0,) = (i for i, num in enumerate(nums) if num.val == 0)
    nums.rotate(-idx0)
    total = 0
    for idx in [1000, 2000, 3000]:
        total += nums[idx % len(nums)].val
    return total


def solve(path: str) -> None:
    nums = parse_input(path)
    mix(nums, nums[0])
    print("Part 1:", output(nums))

    nums = parse_input(path)
    for num in nums:
        num.val *= 811589153
    start = nums[0]
    for _ in range(10):
        mix(nums, start)
    print("Part 2:", output(nums))


if __name__ == "__main__":
    solve("inputs/20.full")
