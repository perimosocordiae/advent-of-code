// Usage: chpl day18.chpl && ./day18 <inputs/18.full && rm -f day18
use IO;
use List;
use direction;

enum direction {
  R = 0,
  D = 1,
  L = 2,
  U = 3,
};

iter parseLines() {
  var d1: string;
  var n1: int;
  var color: uint;
  while readf("%s %i (#%xu)\n", d1, n1, color) {
    const n2 = color / 16;
    const d2 = color % 4;
    yield ((parseDirection(d1), n1), (d2:direction, n2:int));
  }
}

// Order: R, D, L, U
const Directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

proc parseDirection(d: string): direction {
  if (d == 'R') { return R; }
  if (d == 'D') { return D; }
  if (d == 'L') { return L; }
  if (d == 'U') { return U; }
  halt("invalid direction: ", d);
}

iter corners(dirs: list(direction)) {
  const lastIdx = dirs.size - 1;
  yield (dirs[lastIdx], dirs[0]);
  for i in 0..<lastIdx do {
    yield (dirs[i], dirs[i + 1]);
  }
  yield (dirs[lastIdx], dirs[0]);
}

iter bumpedOutCorners(path: list(2*int), dirs: list(direction)) {
  for (pos, corner) in zip(path, corners(dirs)) do {
    if (corner == (R, D) || corner == (D, R)) {
      // convex 7 or concave L
      yield pos + (-0.5, 0.5);
    } else if (corner == (U, L) || corner == (L, U)) {
      // concave 7 or convex L
      yield pos + (0.5, -0.5);
    } else if (corner == (D, L) || corner == (L, D)) {
      // convex J or concave F
      yield pos + (0.5, 0.5);
    } else if (corner == (R, U) || corner == (U, R)) {
      // concave J or convex F
      yield pos - (0.5, 0.5);
    } else {
      halt("invalid corner: ", corner);
    }
  }
}

proc shoelace(points): real {
  var area: real = 0;
  for i in 0..<points.size do {
    const j = (i + 1) % points.size;
    const (xi, yi) = points[i];
    const (xj, yj) = points[j];
    area += xi * yj - xj * yi;
  }
  return abs(area) / 2;
}

proc solve(inputs) {
  var path: list(2*int) = [(0, 0)];
  var dirs: list(direction);
  for (d, n) in inputs {
    dirs.pushBack(d);
    const (dr, dc) = Directions[d:int];
    const (r, c) = path.last;
    path.pushBack((r + dr * n, c + dc * n));
  }
  const padded = bumpedOutCorners(path, dirs);
  return shoelace(padded);
}

const Inputs = parseLines();
cobegin {
  writef("Part 1: %.0dr\n", solve([(p1, p2) in Inputs] p1));
  writef("Part 2: %.0dr\n", solve([(p1, p2) in Inputs] p2));
}