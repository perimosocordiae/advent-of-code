# Picross, basically.
import numpy as np

FULL = 2
EMPTY = 1


def count_arrangements(row, groups):
  while True:
    new_row = _solve_1d(row, groups)
    if np.array_equal(new_row, row):
      break
    row = new_row
  unknowns, = np.where(row == 0)
  # If we don't have any unknowns, there's only one arrangement.
  if len(unknowns) == 0:
    return 1
  # TODO: Avoid the recursion here and look for a closed form.
  # For the first unknown, try both possibilities and recurse.
  total = 0
  i = unknowns[0]
  for v in (EMPTY, FULL):
    row[i] = v
    try:
      new_row = _solve_1d(row, groups)
    except Exception:
      continue
    total += count_arrangements(new_row, groups)
  return total


def _solve_1d(known, constraints):
  nk = len(known)
  nc = len(constraints)
  num_full = sum(constraints)
  num_empty = nk - num_full

  new_known = None
  for gaps in valid_gaps(nc, num_empty):
    # what would the array look like given these gaps?
    arr = _candidate_array(gaps, constraints, known)
    if arr is None:
      # the array would have violated our known values
      continue
    # accumulate common values across all candidates
    if new_known is None:
      new_known = arr
    else:
      new_known = np.bitwise_and(new_known, arr)
  if new_known is None:
    raise Exception("Unsolvable constraint: %s" % constraints)
  return new_known


def _candidate_array(gaps, constraints, known):
  arr = np.zeros_like(known)
  i = 0
  for g, c in zip(gaps, constraints):
    if (known[i : i + g] == FULL).any():
      return None
    arr[i : i + g] = EMPTY
    i += g
    if (known[i : i + c] == EMPTY).any():
      return None
    arr[i : i + c] = FULL
    i += c
  if (known[i:] == FULL).any():
    return None
  arr[i:] = EMPTY
  return arr


def valid_gaps(num_constraints, num_empty):
  for gaps in _sum_combinations(num_constraints + 1, num_empty):
    # make sure there's at least a gap of 1 between full sections
    if len(gaps) > 2 and not all(gaps[1:-1]):
      continue
    yield gaps


def _sum_combinations(n, s):
  """Generate all lists of length n that sum to s."""
  if n == 1:
    yield [s]
  else:
    for i in range(s + 1):
      for j in _sum_combinations(n - 1, s - i):
        yield [i] + j


total = 0
for i, line in enumerate(open("inputs/12.full")):
  lhs, rhs = line.strip().split(" ")
  row = np.array(["?.#".index(c) for c in lhs], dtype=int)
  groups = [int(c) for c in rhs.split(",")]
  # print(i, row, groups)
  c = count_arrangements(row, groups)
  # print(c)
  total += c
print(total)
