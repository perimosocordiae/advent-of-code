use advent2020::*;
use itertools::Itertools;
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

fn part1(initial_grid: &[Point]) -> usize {
    let mut grid = initial_grid.to_vec();
    for _ in 0..6 {
        grid = run_step(&grid, 3);
    }
    grid.len()
}

#[test]
fn part2_small() {
    let grid = parse_grid("inputs/17.test");
    assert_eq!(part2(&grid), 848);
}

fn part2(initial_grid: &[Point]) -> usize {
    let mut grid = initial_grid.to_vec();
    for _ in 0..6 {
        grid = run_step(&grid, 4);
    }
    grid.len()
}

fn run_step(grid: &[Point], dimension: usize) -> Vec<Point> {
    let neighbors = active_neighbors(&grid, dimension);
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

fn active_neighbors(grid: &[Point], dim: usize) -> HashMap<Point, usize> {
    let mut res = HashMap::new();
    for p in grid {
        for deltas in (0..dim)
            .map(|_| [-1, 0, 1 as i64].iter())
            .multi_cartesian_product()
        {
            let neighbor = p.offset(deltas);
            if neighbor != *p {
                *res.entry(neighbor).or_insert_with(|| 0) += 1;
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
impl Point {
    fn offset(&self, deltas: Vec<&i64>) -> Self {
        let w = if deltas.len() == 4 {
            self.w + deltas[3]
        } else {
            0
        };
        Point {
            x: self.x + deltas[0],
            y: self.y + deltas[1],
            z: self.z + deltas[2],
            w,
        }
    }
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
