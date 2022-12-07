#!/usr/bin/env python3


def parse_input(path: str) -> tuple[list[list[str]], list[tuple[int, int, int]]]:
    drawing = []
    key = ""
    moves = []
    with open(path) as f:
        for line in f:
            if line[:2] == " 1":
                key = line
                break
            drawing.append(line)
        for line in f:
            if not line.startswith("move "):
                continue
            _, n, __, s, ___, t = line.split()
            moves.append((int(n), int(s) - 1, int(t) - 1))
    stacks = [[] for _ in range(len(key.strip().split()))]
    for line in drawing:
        pos = line.find("[")
        key_pos = pos
        while True:
            idx = int(key[key_pos + 1]) - 1
            stacks[idx].append(line[pos + 1])
            line = line[pos + 3 :]
            if (pos := line.find("[")) == -1:
                break
            key_pos += 3 + pos
    for stack in stacks:
        stack.reverse()
    return stacks, moves


def part1(path: str) -> str:
    stacks, moves = parse_input(path)
    for n, s, t in moves:
        for _ in range(n):
            stacks[t].append(stacks[s].pop())
    return "".join(stack[-1] for stack in stacks)


def part2(path: str) -> str:
    stacks, moves = parse_input(path)
    for n, s, t in moves:
        stacks[t].extend(stacks[s][-n:])
        del stacks[s][-n:]
    return "".join(stack[-1] for stack in stacks)


if __name__ == "__main__":
    print("Part 1:", part1("inputs/05.full"))
    print("Part 2:", part2("inputs/05.full"))
