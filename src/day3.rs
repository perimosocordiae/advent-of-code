use std::cmp;
use std::collections::HashSet;
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

#[derive(Debug, PartialEq)]
struct Rectangle {
    id: u16,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

impl FromStr for Rectangle {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s[1..].split_whitespace().collect();
        let pos: Vec<&str> = parts[2].trim_matches(':').split(',').collect();
        let size: Vec<&str> = parts[3].split('x').collect();
        Ok(Rectangle {
            id: parts[0].parse()?,
            left: pos[0].parse()?,
            top: pos[1].parse()?,
            width: size[0].parse()?,
            height: size[1].parse()?,
        })
    }
}

fn setup() -> Vec<Rectangle> {
    let f = File::open("inputs/03.txt").unwrap();
    BufReader::new(&f)
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect()
}

fn find_bbox(data: &Vec<Rectangle>) -> (usize, usize) {
    let mut num_rows = 1;
    let mut num_cols = 1;
    for rect in data.iter() {
        num_rows = cmp::max(num_rows, rect.left + rect.width);
        num_cols = cmp::max(num_cols, rect.top + rect.height);
    }
    (num_rows, num_cols)
}

fn part1(data: &Vec<Rectangle>) -> i32 {
    let (num_rows, num_cols) = find_bbox(&data);
    // Count each square's overlaps separately.
    let mut grid: Vec<u16> = vec![0; num_cols * num_rows];
    let index = |r, c| r * num_cols + c;
    for rect in data {
        for r in rect.left..rect.left + rect.width {
            for c in rect.top..rect.top + rect.height {
                grid[index(r, c)] += 1;
            }
        }
    }
    // Count the number of grid cells with overlap
    grid.into_iter().filter(|&c| c > 1).map(|_| 1).sum()
}

fn part2(data: &Vec<Rectangle>) -> u16 {
    let (num_rows, num_cols) = find_bbox(&data);
    let mut grid: Vec<u16> = vec![0; num_cols * num_rows];
    let index = |r, c| r * num_cols + c;
    // Keep track of which rects had overlaps.
    let mut overlaps = HashSet::new();
    for rect in data.iter() {
        for r in rect.left..rect.left + rect.width {
            for c in rect.top..rect.top + rect.height {
                let idx: usize = index(r, c);
                if grid[idx] != 0 {
                    overlaps.insert(grid[idx]);
                    overlaps.insert(rect.id.clone());
                }
                grid[idx] = rect.id;
            }
        }
    }
    for rect in data {
        if !overlaps.contains(&rect.id) {
            return rect.id;
        }
    }
    0
}
