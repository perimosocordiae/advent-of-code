use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

pub fn run() {
    let (initial, rules) = setup();
    println!("Part 1: {}", part1(&initial, &rules));
    println!("Part 2: {}", part2(&initial, &rules));
}

fn setup() -> (Vec<bool>, Vec<Rule>) {
    let f = File::open("inputs/12.txt").unwrap();
    let mut lines = BufReader::new(&f).lines();
    let line: String = lines.next().unwrap().unwrap();
    let parts: Vec<&str> = line.split(": ").collect();
    let initial = parse_hashes(parts[1]);
    lines.next();
    let rules = lines.map(|l| l.unwrap().parse().unwrap()).collect();
    (initial, rules)
}

fn part1(initial: &[bool], rules: &[Rule]) -> i64 {
    let mut start_idx = 0i64;
    let mut grid: Vec<bool> = initial.to_vec();
    for _ in 0..20 {
        let (new_grid, added_left) = run_rules(&grid, rules);
        grid = new_grid;
        start_idx -= added_left;
    }
    sum_plants(&grid, start_idx)
}

fn part2(initial: &[bool], rules: &[Rule]) -> i64 {
    let mut start_idx = 0i64;
    let mut grid: Vec<bool> = initial.to_vec();
    let mut old = 0i64;
    let mut diff = 0i64;
    let stop = 200;
    for _ in 1..=stop {
        let (new_grid, added_left) = run_rules(&grid, rules);
        grid = new_grid;
        start_idx -= added_left;
        let new = sum_plants(&grid, start_idx);
        diff = new - old;
        old = new;
    }
    (50_000_000_000 - stop) * diff + old
}

fn sum_plants(grid: &[bool], start_idx: i64) -> i64 {
    grid.iter()
        .enumerate()
        .filter(|(_, v)| **v)
        .map(|(idx, _)| idx as i64 + start_idx)
        .sum()
}

fn run_rules(grid: &[bool], rules: &[Rule]) -> (Vec<bool>, i64) {
    let pad = 3;
    let mut padded = vec![false; grid.len() + pad * 2];
    for (idx, v) in grid.iter().enumerate() {
        padded[idx + pad] = *v;
    }
    let mut new_grid = padded.clone();
    for idx in 2..(padded.len() - 2) {
        let slice = &padded[(idx - 2)..=(idx + 2)];
        assert!(slice.len() == 5);
        for rule in rules.iter() {
            if rule.pattern.iter().zip(slice).all(|(p, v)| p == v) {
                new_grid[idx] = rule.outcome;
                break;
            }
        }
    }
    (new_grid, pad as i64)
}

#[derive(Debug)]
struct Rule {
    pattern: Vec<bool>,
    outcome: bool,
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" => ").collect();
        Ok(Rule {
            pattern: parse_hashes(parts[0]),
            outcome: parts[1] == "#",
        })
    }
}

fn parse_hashes(s: &str) -> Vec<bool> {
    s.chars().map(|c| c == '#').collect()
}
