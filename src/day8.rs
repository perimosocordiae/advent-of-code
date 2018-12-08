use std::fs;

pub fn run() {
    let data = setup();
    println!("Part 1: {}", part1(&data));
    // println!("Part 2: {}", part2(&data));
}

fn setup() -> Vec<usize> {
    fs::read_to_string("inputs/08.txt")
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn part1(data: &[usize]) -> usize {
    data.iter().sum()
}
