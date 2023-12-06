# Usage:
#   crystal day05.cr inputs/05.test
#   crystal day05.cr inputs/05.full

class Seed
  property value : Int64
  property done : Bool

  def initialize(@value)
    @done = true
  end

  def matches_map?(dst : Int64, src : Int64, n : Int64) : Bool
    !@done && @value >= src && @value - src < n
  end

  def update(dst : Int64, src : Int64, n : Int64)
    @done = true
    @value += dst - src
  end
end

class SeedRange
  property start : Int64
  property length : Int64
  property done : Bool

  def initialize(@start, @length)
    @done = false
  end

  def matches_map?(dst : Int64, src : Int64, n : Int64) : Bool
    return false if @done
    # match if there's any overlap at all
    src < (@start + @length) && @start < (src + n)
  end

  def update(dst : Int64, src : Int64, n : Int64) : Array(SeedRange)
    results = [] of SeedRange
    # Check for a part of our range before the map
    if src > @start
      before = SeedRange.new(@start, src - @start)
      @start = src
      @length -= before.length
      results << before
    end
    # Check for a part of our range after the map
    if src + n < @start + @length
      after = SeedRange.new(src + n, @start + @length - (src + n))
      @length -= after.length
      results << after
    end
    # Now the map fully covers our range
    @start += dst - src
    @done = true
    results
  end

  def inspect(io)
    if @done
      io << "[#{@start}...#{@start + @length}]"
    else
      io << "(#{@start}...#{@start + @length})"
    end
  end
end

in_file = ARGV.first? || "inputs/05.full"
part1_seeds = [] of Seed
part2_seeds = [] of SeedRange
File.open(in_file) do |file|
  file.each_line do |line|
    next if line.strip.empty?
    if line.starts_with?("seeds: ")
      line[7..-1].split(" ").each do |seed|
        part1_seeds << Seed.new(seed.to_i64)
      end
      part1_seeds.each_slice(2) do |s|
        part2_seeds << SeedRange.new(s[0].value, s[1].value)
      end
    elsif line.ends_with?(" map:")
      # mark all seeds as ready for the next map
      part1_seeds.each do |seed|
        seed.done = false
      end
      part2_seeds.each do |seed|
        seed.done = false
      end
    else
      # apply the map to any ready seed that matches
      dst, src, n = line.split(" ").map(&.to_i64)
      part1_seeds.each do |seed|
        if seed.matches_map?(dst, src, n)
          seed.update(dst, src, n)
        end
      end
      new_seeds = [] of SeedRange
      part2_seeds.each do |seed|
        if seed.matches_map?(dst, src, n)
          new_seeds.concat(seed.update(dst, src, n))
        end
      end
      part2_seeds.concat(new_seeds)
    end
  end
end
# find the smallest value among all seeds
puts "Part 1: #{part1_seeds.map(&.value).min}"
puts "Part 2: #{part2_seeds.map(&.start).min}"
