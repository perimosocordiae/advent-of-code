use advent2020::*;
use std::str::FromStr;
use text_io::try_scan;

fn main() {
    let data = read_string("inputs/02.full");
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[test]
fn part1_small() {
    let data = read_string("inputs/02.test");
    assert_eq!(part1(&data), 2);
}

#[test]
fn part2_small() {
    let data = read_string("inputs/02.test");
    assert_eq!(part2(&data), 1);
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|p| p.parse::<PasswordAndPolicy>().unwrap())
        .filter(|p| p.is_valid_one())
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|p| p.parse::<PasswordAndPolicy>().unwrap())
        .filter(|p| p.is_valid_two())
        .count()
}

#[derive(Debug, Default)]
struct PasswordAndPolicy {
    password: String,
    policy: char,
    idx1: usize,
    idx2: usize,
}

impl PasswordAndPolicy {
    fn is_valid_one(&self) -> bool {
        let n = self.password.chars().filter(|&c| c == self.policy).count();
        (n <= self.idx2) && (n >= self.idx1)
    }

    fn is_valid_two(&self) -> bool {
        let a = self.policy == self.get_char(self.idx1);
        let b = self.policy == self.get_char(self.idx2);
        a != b
    }

    fn get_char(&self, idx: usize) -> char {
        self.password.chars().nth(idx - 1).unwrap()
    }
}

impl FromStr for PasswordAndPolicy {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut p = PasswordAndPolicy::default();
        try_scan!(s.bytes() => "{}-{} {}: {}", p.idx1, p.idx2, p.policy, p.password);
        Ok(p)
    }
}
