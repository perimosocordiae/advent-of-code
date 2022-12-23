#!/usr/bin/env python3
from collections import defaultdict, deque


class Elf:
    def __init__(self, r: int, c: int):
        self.loc = (r, c)

    def propose(self, grove, directions) -> tuple[int, int] | None:
        r, c = self.loc
        NE = (r - 1, c + 1) in grove
        NW = (r - 1, c - 1) in grove
        SE = (r + 1, c + 1) in grove
        SW = (r + 1, c - 1) in grove
        N = (r - 1, c) in grove
        S = (r + 1, c) in grove
        E = (r, c + 1) in grove
        W = (r, c - 1) in grove
        if not any((NE, NW, SE, SW, N, S, E, W)):
            return None
        for d in directions:
            if d == "N":
                if not any((NE, NW, N)):
                    return (r - 1, c)
            elif d == "S":
                if not any((SE, SW, S)):
                    return (r + 1, c)
            elif d == "E":
                if not any((NE, SE, E)):
                    return (r, c + 1)
            elif d == "W":
                if not any((NW, SW, W)):
                    return (r, c - 1)
        return self.loc


def solve(path: str) -> None:
    grove = set()
    elves = []
    for r, line in enumerate(open(path)):
        for c, ch in enumerate(line.strip()):
            if ch == "#":
                grove.add((r, c))
                elves.append(Elf(r, c))

    directions = deque(["N", "S", "W", "E"])
    for round_number in range(30000):
        proposals = defaultdict(list)
        dirty = False
        for elf in elves:
            prop = elf.propose(grove, directions)
            if prop is not None:
                proposals[prop].append(elf)
                dirty = True
            else:
                proposals[elf.loc].append(elf)
        if not dirty:
            print("Part 2:", round_number + 1)
            break
        for loc, p_elves in proposals.items():
            if len(p_elves) == 1:
                elf = p_elves[0]
                grove.remove(elf.loc)
                grove.add(loc)
                elf.loc = loc
        directions.rotate(-1)
        if round_number == 9:
            # compute the bounding box of the grove
            max_r = max(r for r, c in grove)
            min_r = min(r for r, c in grove)
            max_c = max(c for r, c in grove)
            min_c = min(c for r, c in grove)
            num_empty = (max_r - min_r + 1) * (max_c - min_c + 1) - len(elves)
            print("Part 1:", num_empty)


if __name__ == "__main__":
    solve("inputs/23.full")
