OPCODES = ["adv", "bxl", "bst", "jnz", "bxc", "out", "bdv", "cdv"]


def main(infile="inputs/17.full") -> None:
    reg, prog = _parse_input(infile)
    output = _run_program(list(reg), prog)
    print("Part 1:", ",".join(map(str, output)))

    # Part 2
    for a in range(35184496000000, 281474976710656):
        if a % 1000000 == 0:
            print(a)
        for idx, val in enumerate(_fast_program(a)):
            if val != prog[idx]:
                break
        else:
            if idx == len(prog) - 1:
                print("Part 2:", a)
                break


def _fast_program(a: int):
    while a:
        # Take last 3 bits of a, xor with 0b101
        b = (a % 8) ^ 0b101
        yield ((b ^ 0b110) ^ (a >> b)) % 8
        # Remove last 3 bits of a
        a //= 8


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
