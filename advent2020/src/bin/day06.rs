use advent2020::*;
use std::collections::HashMap;

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
    let mut answers: HashMap<char, usize> = HashMap::new();
    for line in group.lines() {
        for ch in line.chars() {
            *answers.entry(ch).or_insert(0) += 1;
        }
    }
    let num_lines = group.lines().count();
    answers
        .values()
        .filter(|&count| *count == num_lines)
        .count()
}
