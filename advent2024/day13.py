import re
import numpy as np
from scipy.optimize import LinearConstraint, Bounds, milp

cost = np.array([3, 1], dtype=int)
integrality = np.ones_like(cost)
bounds = Bounds(np.zeros_like(cost), np.full_like(cost, 100))

infile = "inputs/13.full"
part1 = 0
part2 = 0
for chunk in open(infile).read().split("\n\n"):
    x1, x2, b = [
        list(map(int, re.findall(r"\d+", line))) for line in chunk.splitlines()
    ]
    A = np.column_stack([x1, x2])
    b = np.array(b, dtype=int)
    res1 = milp(
        cost,
        integrality=integrality,
        bounds=bounds,
        constraints=LinearConstraint(A, b, b),
    )
    if res1.success:
        part1 += int(res1.fun)
    b += 10000000000000
    res2 = milp(
        cost,
        constraints=LinearConstraint(A, b, b),
    )
    if res2.success:
        rx = np.round(res2.x).astype(int)
        if (A @ rx == b).all():
            part2 += rx @ cost

print("Part 1:", part1)
print("Part 2:", part2)
