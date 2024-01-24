// Usage: chpl day16.chpl && ./day16 <inputs/16.full && rm -f day16
use List;
use cell;

enum cell {
  EMPTY = 0,  // .
  VERT = 1,   // |
  HORZ = 2,   // -
  HRIGHT = 3, // /
  HLEFT = 4,  // \
};

proc total_energy(grid, (init_pos, init_dir)): int {
  var energy: [grid.domain] int;
  var beams: list(2*(2*int)) = [(init_pos, init_dir)];
  while (!beams.isEmpty()) {
    ref (pos, dir) = beams.last;
    pos += dir;
    if (!grid.domain.contains(pos)) {
      beams.popBack();
      continue;
    }
    const spot = grid[pos];
    if (spot == VERT) {
      if (dir[0] == 0) {
        if ((energy[pos] & 0b10) == 0) {
          dir = (1, 0);
          beams.pushBack((pos, (-1, 0)));
        } else {
          beams.popBack();
        }
      }
    } else if (spot == HORZ) {
      if (dir[1] == 0) {
        if ((energy[pos] & 0b01) == 0) {
          dir = (0, 1);
          beams.pushBack((pos, (0, -1)));
        } else {
          beams.popBack();
        }
      }
    } else if (spot == HRIGHT) {
      dir = (-dir[1], -dir[0]);
    } else if (spot == HLEFT) {
      dir = (dir[1], dir[0]);
    }
    if (dir[0] == 0) {
      energy[pos] |= 0b01;
    } else {
      energy[pos] |= 0b10;
    }
  }
  return + reduce (energy != 0);
}

proc charToCell(c: string): cell {
  if (c == '.') {
    return EMPTY;
  } else if (c == '|') {
    return VERT;
  } else if (c == '-') {
    return HORZ;
  } else if (c == '/') {
    return HRIGHT;
  } else if (c == '\\') {
    return HLEFT;
  } else {
    halt("Unknown cell: ", c);
  }
}

iter readLines() {
  use IO;
  var line: string;
  while readLine(line, stripNewline=true) {
    yield line;
  }
}

iter all_starts(grid) {
  const (nr, nc) = grid.shape;
  // Right from the left side and left from the right side.
  for r in 0..<nr {
    yield ((r, -1), (0, 1));
    yield ((r, nc), (0, -1));
  }
  // Down from the top side and up from the bottom side.
  for c in 0..<nc {
    yield ((-1, c), (1, 0));
    yield ((nr, c), (-1, 0));
  }
}

const Lines = readLines();
const grid = [(r,c) in {0..<Lines.size, 0..<Lines.first.size}] charToCell(Lines[r][c]);
writeln("Part 1: ", total_energy(grid, ((0, -1), (0, 1))));

const Starts = all_starts(grid);
writeln("Part 2: ", max reduce total_energy(grid, Starts));
