def apply(n: int) -> list[int]:
    if n == 0:
        return [1]
    s = str(n)
    if len(s) % 2 == 1:
        return [n * 2024]
    mid = len(s) // 2
    return [int(s[:mid]), int(s[mid:])]


# Global memoization table for the dynamic program.
memo = []  # type: list[dict[int, int]]


def count_stones(num: int, reps: int) -> int:
    if reps == 0:
        return 1
    while reps >= len(memo):
        memo.append(dict())
    md = memo[reps]
    if num not in md:
        md[num] = sum(count_stones(x, reps - 1) for x in apply(num))
    return md[num]


def solve(sequence: list[int], reps: int) -> int:
    return sum(count_stones(x, reps) for x in sequence)


infile = "inputs/11.full"
seq = [int(x) for x in open(infile).read().split()]
print("Part 1:", solve(seq, 25))
print("Part 2:", solve(seq, 75))
