use advent2020::*;
use std::collections::HashMap;

fn main() {
    let grid = parse_grid("inputs/17.full");
    println!("part1: {}", part1(&grid));
    println!("part2: {}", part2(&grid));
}

#[test]
fn part1_small() {
    let grid = parse_grid("inputs/17.test");
    assert_eq!(part1(&grid), 112);
}

fn part1(grid: &[Point]) -> usize {
    run_n_cycles(grid, true, 6).len()
}

#[test]
fn part2_small() {
    let grid = parse_grid("inputs/17.test");
    assert_eq!(part2(&grid), 848);
}

fn part2(grid: &[Point]) -> usize {
    run_n_cycles(grid, false, 6).len()
}

fn run_n_cycles(
    grid: &[Point],
    is_part1: bool,
    num_cycles: usize,
) -> Vec<Point> {
    if num_cycles == 0 {
        grid.to_vec()
    } else {
        run_n_cycles(&run_step(grid, is_part1), is_part1, num_cycles - 1)
    }
}

fn run_step(grid: &[Point], is_part1: bool) -> Vec<Point> {
    let neighbors = if is_part1 {
        active_neighbors1(&grid)
    } else {
        active_neighbors2(&grid)
    };
    let mut next_grid = vec![];
    for p in grid {
        if let Some(n) = neighbors.get(&p) {
            if *n == 2 {
                next_grid.push(p.clone());
            }
        }
    }
    for (p, n) in neighbors.iter() {
        if *n == 3 {
            next_grid.push(p.clone());
        }
    }
    next_grid
}

fn active_neighbors1(grid: &[Point]) -> HashMap<Point, usize> {
    let mut res = HashMap::new();
    for p in grid {
        for dx in [-1, 0, 1].iter() {
            for dy in [-1, 0, 1].iter() {
                for dz in [-1, 0, 1].iter() {
                    let neighbor = Point {
                        x: p.x + dx,
                        y: p.y + dy,
                        z: p.z + dz,
                        w: 0,
                    };
                    if neighbor != *p {
                        *res.entry(neighbor).or_insert_with(|| 0) += 1;
                    }
                }
            }
        }
    }
    res
}

fn active_neighbors2(grid: &[Point]) -> HashMap<Point, usize> {
    let mut res = HashMap::new();
    for p in grid {
        for dx in [-1, 0, 1].iter() {
            for dy in [-1, 0, 1].iter() {
                for dz in [-1, 0, 1].iter() {
                    for dw in [-1, 0, 1].iter() {
                        let neighbor = Point {
                            x: p.x + dx,
                            y: p.y + dy,
                            z: p.z + dz,
                            w: p.w + dw,
                        };
                        if neighbor != *p {
                            *res.entry(neighbor).or_insert_with(|| 0) += 1;
                        }
                    }
                }
            }
        }
    }
    res
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

fn parse_grid(path: &str) -> Vec<Point> {
    let data = read_string(path);
    let mut grid = vec![];
    for (y, line) in data.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                grid.push(Point {
                    x: x as i64,
                    y: y as i64,
                    z: 0,
                    w: 0,
                });
            }
        }
    }
    grid
}
