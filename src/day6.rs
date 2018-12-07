use ndarray::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::num::ParseIntError;
use std::str::FromStr;

pub fn run() {
    let data = setup();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn setup() -> Vec<Point> {
    let f = File::open("inputs/06.txt").unwrap();
    BufReader::new(&f)
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect()
}

fn part1(data: &[Point]) -> i32 {
    let min_x = data.iter().map(|p| p.x).min().unwrap();
    let min_y = data.iter().map(|p| p.y).min().unwrap();
    let max_x = data.iter().map(|p| p.x).max().unwrap();
    let max_y = data.iter().map(|p| p.y).max().unwrap();
    let num_rows = (max_x - min_x + 1) as usize;
    let num_cols = (max_y - min_y + 1) as usize;
    let mut grid = Array2::zeros((num_rows, num_cols));
    for (idx, p) in data.iter().enumerate() {
        let r = (p.x - min_x) as usize;
        let c = (p.y - min_y) as usize;
        grid[[r, c]] = idx + 1;
    }
    // Fill the grid based on unique closest (L1) points.
    for r in 0..num_rows {
        for c in 0..num_cols {
            if grid[[r, c]] > 0 {
                continue;
            }
            let pt = Point {
                x: min_x + r as i32,
                y: min_y + c as i32,
            };
            let dists: Vec<i32> = data.iter().map(|p| l1_dist(&pt, p)).collect();
            let mut order: Vec<usize> = (0..dists.len()).collect();
            order.sort_unstable_by_key(|i| dists[*i]);
            if dists[order[0]] != dists[order[1]] {
                grid[[r, c]] = order[0] + 1;
            }
        }
    }
    // Disqualify infinite areas (touching the border).
    let mut border_ids = HashSet::new();
    for r in 0..num_rows {
        border_ids.insert(grid[[r, 0]]);
        border_ids.insert(grid[[r, num_cols - 1]]);
    }
    for c in 1..(num_cols - 1) {
        border_ids.insert(grid[[0, c]]);
        border_ids.insert(grid[[num_rows - 1, c]]);
    }
    // Count non-disqualified regions.
    let mut area_sizes = HashMap::new();
    for r in 1..(num_rows - 1) {
        for c in 1..(num_cols - 1) {
            let id = grid[[r, c]];
            if border_ids.contains(&id) {
                continue;
            }
            *area_sizes.entry(id).or_insert(0) += 1;
        }
    }
    // Return the size of the largest internal region.
    *area_sizes.values().max().unwrap()
}

fn part2(data: &[Point]) -> i32 {
    let min_x = data.iter().map(|p| p.x).min().unwrap();
    let min_y = data.iter().map(|p| p.y).min().unwrap();
    let max_x = data.iter().map(|p| p.x).max().unwrap();
    let max_y = data.iter().map(|p| p.y).max().unwrap();
    let mut count: i32 = 0;
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let pt = Point { x, y };
            let sum_dist = data.iter().map(|p| l1_dist(&pt, p)).sum::<i32>();
            if sum_dist < 10000 {
                count += 1;
            }
        }
    }
    count
}

fn l1_dist(a: &Point, b: &Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(", ").collect();
        Ok(Point {
            x: parts[1].parse()?,
            y: parts[0].parse()?,
        })
    }
}
