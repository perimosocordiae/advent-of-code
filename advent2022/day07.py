#!/usr/bin/env python3
from typing import Optional, Iterable


class INode:
    def __init__(self, name: str, *, parent: "Optional[INode]" = None, size: int = 0):
        self.name = name
        self.children = {}  # type: dict[str, INode]
        self.parent = parent if parent is not None else self
        self.size = size
        self._total_size = None  # type: Optional[int]

    def subdir(self, name: str) -> "INode":
        if name not in self.children:
            self.children[name] = INode(name, parent=self)
        return self.children[name]

    def total_size(self) -> int:
        if self._total_size is None:
            self._total_size = self.size + sum(
                child.total_size() for child in self.children.values()
            )
        return self._total_size

    def walk_dirs(self) -> "Iterable[INode]":
        if self.size == 0:
            yield self
            for child in self.children.values():
                yield from child.walk_dirs()

    def __str__(self, indent: int = 0) -> str:
        return f"{'  ' * indent}{self.name} ({self.size})\n" + "".join(
            child.__str__(indent + 1) for child in self.children.values()
        )

    def __repr__(self) -> str:
        return f"INode({self.name!r}, parent={self.parent!r}, size={self.size})"


def parse_input(path: str) -> INode:
    root = INode("<root>")
    cwd = root
    for command in open(path).read().split("$ ")[1:]:
        if command.startswith("cd "):
            dest = command[3:].strip()
            if dest == "..":
                cwd = cwd.parent
            else:
                cwd = cwd.subdir(dest)
        elif command.startswith("ls\n"):
            for row in command[3:].splitlines():
                if row.startswith("dir "):
                    cwd.subdir(row[4:])
                else:
                    size, name = row.split()
                    cwd.children[name] = INode(name, parent=cwd, size=int(size))
        else:
            raise ValueError(f"Unknown command: {command!r}")
    return root.children["/"]


def solve(path: str) -> None:
    fs = parse_input(path)
    part1 = sum(total for d in fs.walk_dirs() if (total := d.total_size()) <= 100000)
    print("Part 1:", part1)

    to_delete = 30000000 - (70000000 - fs.total_size())
    part2 = min(total for d in fs.walk_dirs() if (total := d.total_size()) >= to_delete)
    print("Part 2:", part2)


if __name__ == "__main__":
    solve("inputs/07.full")
