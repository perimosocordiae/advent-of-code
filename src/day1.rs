use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn run() {
    let data = setup();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn setup() -> Vec<i32> {
    let f = File::open("inputs/01.txt").unwrap();
    BufReader::new(&f)
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect()
}

fn part1(data: &Vec<i32>) -> i32 {
    data.iter().sum()
}

fn part2(data: &Vec<i32>) -> i32 {
    let mut seen = HashMap::new();
    let mut sum: i32 = 0;
    // TODO: Use an infinite range here?
    for cycle in 0..1_000_000 {
        for x in data {
            sum += x;
            let old_cycle = seen.entry(sum).or_insert(cycle);
            if *old_cycle != cycle {
                return sum;
            }
        }
    }
    0
}
