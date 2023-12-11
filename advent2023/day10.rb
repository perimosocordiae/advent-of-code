
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
    [:west, :east]
  when '7'
    [:south, :west]
  when 'F'
    [:south, :east]
  when 'J'
    [:north, :west]
  when 'L'
    [:north, :east]
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

in_file = ARGV.first || "inputs/10.test"
# read the whole file into a list of strings, one per line
lines = File.readlines(in_file, chomp: true)

# find the row and column of the 'S' character
row = lines.index { |line| line.include?('S') }
col = lines[row].index('S')

# check neighbors of S to start the search
# keep track of distances from S in a hash
distances = { [row, col] => 0 }
frontier = []
pipe_connections('S').each do |direction|
  if connects?(row, col, lines, direction)
    neighbor = next_pos(row, col, direction)
    distances[neighbor] = 1
    frontier << neighbor
  end
end

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
