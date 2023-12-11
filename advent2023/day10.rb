# Usage: ruby day10.rb [input_file]

def connects_north?(row, col, lines)
  row > 0 && '|7F'.include?(lines[row - 1][col])
end

def connects_south?(row, col, lines)
  row < lines.size - 1 && '|LJ'.include?(lines[row + 1][col])
end

def connects_west?(row, col, lines)
  col > 0 && '-LF'.include?(lines[row][col - 1])
end

def connects_east?(row, col, lines)
  col < lines[0].size - 1 && '-J7'.include?(lines[row][col + 1])
end

def next_pos(row, col, direction)
  case direction
  when :north
    [row - 1, col]
  when :south
    [row + 1, col]
  when :west
    [row, col - 1]
  when :east
    [row, col + 1]
  end
end

def pipe_connections(pipe)
  case pipe
  when '|'
    [:north, :south]
  when '-'
    [:east, :west]
  when '7'
    [:south, :west]
  when 'F'
    [:east, :south]
  when 'J'
    [:north, :west]
  when 'L'
    [:east, :north]
  when 'S'
    [:north, :south, :west, :east]
  end
end

def connects?(row, col, lines, direction)
  case direction
  when :north
    connects_north?(row, col, lines)
  when :south
    connects_south?(row, col, lines)
  when :west
    connects_west?(row, col, lines)
  when :east
    connects_east?(row, col, lines)
  end
end

in_file = ARGV.first || "inputs/10.full"
# read the whole file into a list of strings, one per line
lines = File.readlines(in_file, chomp: true)

# find the row and column of the 'S' character
sr = lines.index { |line| line.include?('S') }
sc = lines[sr].index('S')

# check neighbors of S to start the search
# keep track of distances from S in a hash
distances = { [sr, sc] => 0 }
frontier = []
s_directions = []
pipe_connections('S').each do |direction|
  if connects?(sr, sc, lines, direction)
    neighbor = next_pos(sr, sc, direction)
    distances[neighbor] = 1
    frontier << neighbor
    s_directions << direction
  end
end

# find the pipe character that matches the directions from S
s_directions.sort!
s_char = '|-7FJL'.chars.filter { |c| pipe_connections(c) == s_directions }.first
lines[sr][sc] = s_char

# search until we run out of frontier
while !frontier.empty? do
  row, col = frontier.shift
  conns = pipe_connections(lines[row][col])
  distance = distances[[row, col]]
  conns.each do |direction|
    if connects?(row, col, lines, direction)
      neighbor = next_pos(row, col, direction)
      if !distances.key?(neighbor)
        distances[neighbor] = distance + 1
        frontier << neighbor
      end
    end
  end
end

# Find the maximum distance from S
puts "Part 1: #{distances.values.max}"

def pipe_to_patch(pipe)
  case pipe
  when '|'
    [[0,1,0],
     [0,1,0],
     [0,1,0]]
  when '-'
    [[0,0,0],
     [1,1,1],
     [0,0,0]]
  when '7'
    [[0,0,0],
     [1,1,0],
     [0,1,0]]
  when 'F'
    [[0,0,0],
     [0,1,1],
     [0,1,0]]
  when 'J'
    [[0,1,0],
     [1,1,0],
     [0,0,0]]
  when 'L'
    [[0,1,0],
     [0,1,1],
     [0,0,0]]
  end
end

# Create a "hi-res" map of the pipe loop.
# The map is 3x3 for each pipe character.
hi_res = Array.new(lines.size * 3) { Array.new(lines[0].size * 3, 0) }
distances.keys.each do |row, col|
  patch = pipe_to_patch(lines[row][col])
  r = row * 3
  c = col * 3
  patch.each_with_index do |prow, i|
    prow.each_with_index do |cell, j|
      hi_res[r + i][c + j] = cell
    end
  end
end

# Flood fill the hi-res map, starting at the boundary.
# Mark all cells reachable from the boundary as 2 (outside the loop).
h_height = hi_res.size
h_width = hi_res[0].size
frontier = []
h_height.times do |i|
  frontier << [i, 0]
  frontier << [i, h_width - 1]
end
h_width.times do |j|
  frontier << [0, j]
  frontier << [h_height - 1, j]
end
frontier.each do |row, col|
  hi_res[row][col] = 2
end

while !frontier.empty? do
  row, col = frontier.shift
  [[row - 1, col], [row + 1, col], [row, col - 1], [row, col + 1]].each do |r, c|
    if r >= 0 && r < hi_res.size && c >= 0 && c < hi_res[0].size && hi_res[r][c] == 0
      hi_res[r][c] = 2
      frontier << [r, c]
    end
  end
end

# Count the number of all-zero 3x3 patches in the hi-res map.
count = 0
(0...lines.size).each do |row|
  (0...lines[0].size).each do |col|
    count += 1 if hi_res[row * 3, 3].all? { |r| r[col * 3, 3].all? { |c| c == 0 } }
  end
end
puts "Part 2: #{count}"