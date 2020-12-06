use advent2020::*;
use std::collections::HashSet;

fn main() {
    let data = read_string("inputs/06.full");
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[test]
fn part1_small() {
    let data = read_string("inputs/06.test");
    assert_eq!(part1(&data), 11);
}

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|s: &str| decode_group_one(&s))
        .sum()
}

#[test]
fn part2_small() {
    let data = read_string("inputs/06.test");
    assert_eq!(part2(&data), 6);
}

fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|s: &str| decode_group_two(&s))
        .sum()
}

fn decode_group_one(group: &str) -> usize {
    let mut answers: Vec<char> = group
        .chars()
        .filter(|&ch| ch.is_ascii_alphabetic())
        .collect();
    answers.sort_unstable();
    answers.dedup();
    answers.len()
}

fn decode_group_two(group: &str) -> usize {
    let mut yesses = vec![];
    for line in group.lines() {
        let ans: HashSet<char> = line.chars().collect();
        yesses.push(ans);
    }
    yesses
        .iter()
        .fold(yesses[0].clone(), |acc, set| {
            acc.intersection(&set).cloned().collect()
        })
        .len()
}
