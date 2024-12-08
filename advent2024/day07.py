def has_solution1(total, nums) -> bool:
    if len(nums) == 1:
        return total == nums[0]
    # Check if last operator could be a multiply
    if total % nums[-1] == 0 and has_solution1(total // nums[-1], nums[:-1]):
        return True
    # Check if last operator could be a plus
    if total > nums[-1]:
        return has_solution1(total - nums[-1], nums[:-1])
    return False


def maybe_undo_concat(total, last_num) -> int | None:
    if total == last_num:
        return None
    st = str(total)
    sn = str(last_num)
    if st.endswith(sn):
        return int(st.removesuffix(sn))
    return None


def has_solution2(total, nums) -> bool:
    if len(nums) == 1:
        return total == nums[0]
    # Check if last operator could be a concat
    if (new_total := maybe_undo_concat(total, nums[-1])) and has_solution2(
        new_total, nums[:-1]
    ):
        return True
    # Check if last operator could be a multiply
    if total % nums[-1] == 0 and has_solution2(total // nums[-1], nums[:-1]):
        return True
    # Check if last operator could be a plus
    if total > nums[-1]:
        return has_solution2(total - nums[-1], nums[:-1])
    return False


infile = "inputs/07.full"
part1 = 0
part2 = 0
for line in open(infile):
    total, nums = line.strip().split(":")
    total = int(total)
    nums = list(map(int, nums.split()))
    if has_solution1(total, nums):
        part1 += total
    elif has_solution2(total, nums):
        part2 += total
print("Part 1:", part1)
print("Part 2:", part1 + part2)
