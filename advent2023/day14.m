% Usage: octave --no-gui day14.m
1;

function row = slide_rocks(row)
  block_idxs = find(row == 2);
  fenceposts = [0, block_idxs, length(row)];
  for i = 1:(length(fenceposts) - 1)
    s = fenceposts(i) + 1;
    t = fenceposts(i + 1);
    row(s:t) = sort(row(s:t));
  end
end

function load = calc_load(grid)
  rocks_per_row = sum(grid == 0, 2);
  load = (size(grid, 1):-1:1) * rocks_per_row;
end

grid = zeros(0, 0);
fid = fopen('inputs/14.full', 'r');
line = fgetl(fid);
row_idx = 1;
while ischar(line)
  row = char(num2cell(line));
  grid(row_idx, :) = (row == '.') + 2 .* (row == '#');
  line = fgetl(fid);
  row_idx = row_idx + 1;
end
fclose(fid);

grid = grid';
for i = 1:length(grid)
  grid(i, :) = slide_rocks(grid(i, :));
end
grid = grid';

fprintf('Part 1: %d\n', calc_load(grid));

cycle_loads = zeros(300, 1);
for cycle = 1:length(cycle_loads)
  for d = 'WSEN'
    grid = rot90(grid, 3);
    if d == 'N'
      cycle_loads(cycle) = calc_load(grid);
    end
    grid = grid';
    for i = 1:length(grid)
      grid(i, :) = slide_rocks(grid(i, :));
    end
    grid = grid';
  end
end
y = cycle_loads(201:end);
for i = 2:length(y)
  if all(y(i:end) == y(1:(end - i + 1)))
    period = i - 1;
    break;
  end
end
fprintf('Part 2: %d\n', y(mod(1000000000 - 201, period) + 1));
