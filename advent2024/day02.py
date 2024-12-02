import numpy as np


def is_safe(nums) -> bool:
    d = np.diff(nums)
    max_d = abs(d).max()
    return 0 < max_d <= 3 and ((d > 0).all() or (d < 0).all())


part1 = 0
part2 = 0
for line in open("inputs/02.full"):
    nums = list(map(int, line.split()))
    if is_safe(nums):
        part1 += 1
        part2 += 1
        continue
    for i in range(len(nums)):
        if is_safe(nums[:i] + nums[i + 1 :]):
            part2 += 1
            break

print("Part 1:", part1)
print("Part 2:", part2)
