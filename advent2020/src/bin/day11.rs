use advent2020::*;

fn main() {
    let grid = parse_input("inputs/11.full");
    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}

#[test]
fn test_part1() {
    let grid = parse_input("inputs/11.test");
    assert_eq!(part1(&grid), 37);
}

#[test]
fn test_part2() {
    let grid = parse_input("inputs/11.test");
    assert_eq!(part2(&grid), 26);
}

#[derive(Clone, Copy, PartialEq)]
enum Space {
    Floor,
    Open,
    Taken,
    Entering,
    Leaving,
}

type Grid = Vec<Vec<Space>>;
type GridRef = [Vec<Space>];

fn part1(starting_grid: &GridRef) -> usize {
    run_simulation(starting_grid.to_vec(), 4, occupied_neighbors)
}

fn part2(starting_grid: &GridRef) -> usize {
    run_simulation(starting_grid.to_vec(), 5, occupied_sightlines)
}

fn run_simulation(
    mut grid: Grid,
    full_threshold: usize,
    occupancy: fn(&GridRef, usize, usize) -> usize,
) -> usize {
    loop {
        // Mark entering/leaving cells.
        let mut dirty = false;
        for i in 1..(grid.len() - 1) {
            for j in 1..(grid[i].len() - 1) {
                match grid[i][j] {
                    Space::Open => {
                        if occupancy(&grid, i, j) == 0 {
                            grid[i][j] = Space::Entering;
                            dirty = true;
                        }
                    }
                    Space::Taken => {
                        if occupancy(&grid, i, j) >= full_threshold {
                            grid[i][j] = Space::Leaving;
                            dirty = true;
                        }
                    }
                    _ => {}
                }
            }
        }
        if !dirty {
            break;
        }
        // Resolve entering/leaving cells.
        for row in grid.iter_mut().skip(1) {
            for seat in row.iter_mut().skip(1) {
                if *seat == Space::Entering {
                    *seat = Space::Taken;
                } else if *seat == Space::Leaving {
                    *seat = Space::Open;
                }
            }
        }
    }
    num_occupied(&grid)
}

fn num_occupied(grid: &GridRef) -> usize {
    grid.iter()
        .flatten()
        .filter(|&s| *s == Space::Taken)
        .count()
}

fn is_occupied(seat: Space) -> bool {
    (seat == Space::Taken) || (seat == Space::Leaving)
}

fn occupied_neighbors(grid: &GridRef, i: usize, j: usize) -> usize {
    let mut count = 0;
    for &r in [i - 1, i + 1].iter() {
        count += grid[r][(j - 1)..=(j + 1)]
            .iter()
            .filter(|&s| is_occupied(*s))
            .count();
    }
    if is_occupied(grid[i][j - 1]) {
        count += 1;
    }
    if is_occupied(grid[i][j + 1]) {
        count += 1;
    }
    count
}

fn occupied_sightlines(grid: &GridRef, i: usize, j: usize) -> usize {
    let mut count = 0;
    for di in -1..=1 {
        for dj in -1..=1 {
            if di == 0 && dj == 0 {
                continue;
            }
            if sees_occupied(grid, i, j, di, dj) {
                count += 1;
            }
        }
    }
    count
}

fn sees_occupied(
    grid: &GridRef,
    si: usize,
    sj: usize,
    di: isize,
    dj: isize,
) -> bool {
    let num_rows = grid.len() as isize;
    let num_cols = grid.first().unwrap().len() as isize;
    let mut i = si as isize + di;
    let mut j = sj as isize + dj;
    while i >= 0 && i < num_rows && j >= 0 && j < num_cols {
        let s = grid[i as usize][j as usize];
        if s != Space::Floor {
            return is_occupied(s);
        }
        i += di;
        j += dj;
    }
    false
}

fn parse_input(path: &str) -> Grid {
    let data = read_string(path);
    let mut grid = vec![vec![]];
    for line in data.lines() {
        let mut row = vec![Space::Floor];
        for ch in line.chars() {
            row.push(match ch {
                '.' => Space::Floor,
                'L' => Space::Open,
                '#' => Space::Taken,
                _ => panic!("Unknown grid char: {:?}", ch),
            });
        }
        row.push(Space::Floor);
        grid.push(row);
    }
    let num_cols = grid.last().unwrap().len();
    for _ in 0..num_cols {
        grid[0].push(Space::Floor);
    }
    grid.push(grid[0].to_vec());
    grid
}
