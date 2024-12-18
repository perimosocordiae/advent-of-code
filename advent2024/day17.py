OPCODES = ["adv", "bxl", "bst", "jnz", "bxc", "out", "bdv", "cdv"]


def main(infile="inputs/17.full") -> None:
    reg, prog = _parse_input(infile)
    output = _run_program(list(reg), prog)
    print("Part 1:", ",".join(map(str, output)))
    print("Part 2:", _match_program(prog))


def _match_program(prog: list[int], start=0) -> int:
    if not prog:
        return start
    target = prog[-1]
    base = start << 3
    for a in _valid_a(base, target):
        answer = _match_program(prog[:-1], base + a)
        if answer != -1:
            return answer
    return -1


def _valid_a(base: int, target: int):
    for a in range(8):
        if (a ^ 0b011) ^ ((a + base) >> (a ^ 0b101)) % 8 == target:
            yield a


def _run_program(reg: list[int], prog: list[int]):
    ip = 0
    while ip < len(prog):
        op, x = prog[ip], prog[ip + 1]
        # print(f"{ip=}, {OPCODES[op]}, {x=}, {reg=}")
        if op == 0:  # adv
            reg[0] >>= _combo(x, reg)
        elif op == 1:  # bxl
            reg[1] ^= x
        elif op == 2:  # bst
            reg[1] = _combo(x, reg) % 8
        elif op == 3:  # jnz
            if reg[0] != 0:
                ip = x - 2
        elif op == 4:  # bxc
            reg[1] ^= reg[2]
        elif op == 5:  # out
            yield _combo(x, reg) % 8
        elif op == 6:  # bdv
            reg[1] = reg[0] >> _combo(x, reg)
        elif op == 7:  # cdv
            reg[2] = reg[0] >> _combo(x, reg)
        else:
            raise ValueError(f"Invalid opcode: {op}")
        ip += 2


def _combo(x: int, reg: list[int]) -> int:
    if x < 4:
        return x
    if x < 7:
        return reg[x - 4]
    raise ValueError(f"Invalid combo operand: {x}")


def _parse_input(path: str):
    ra, rb, rc = 0, 0, 0
    for line in open(path):
        if line.startswith("Register "):
            reg, val = line.split(": ")
            val = int(val)
            if reg[-1] == "A":
                ra = val
            elif reg[-1] == "B":
                rb = val
            else:
                rc = val
        elif line.startswith("Program: "):
            prog = map(int, line.split(": ", 1)[1].strip().split(","))
    return [ra, rb, rc], list(prog)


if __name__ == "__main__":
    main()
