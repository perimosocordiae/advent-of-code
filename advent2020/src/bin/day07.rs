use advent2020::*;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[macro_use]
extern crate lazy_static;

fn main() {
    let data = read_string("inputs/07.full");
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[test]
fn part1_small() {
    let data = read_string("inputs/07.test");
    assert_eq!(part1(&data), 4);
}

fn part1(input: &str) -> usize {
    let rules = parse_rules_in_to_out(&input);
    let mut queue = vec!["shiny gold"];
    let mut seen = HashSet::new();
    while let Some(bag) = queue.pop() {
        if let Some(outer_bags) = rules.get(bag) {
            for outer in outer_bags {
                if !seen.contains(outer) {
                    queue.push(outer);
                    seen.insert(outer);
                }
            }
        }
    }
    seen.len()
}

fn parse_rules_in_to_out(input: &str) -> HashMap<String, Vec<String>> {
    lazy_static! {
        static ref RE_LHS: Regex =
            Regex::new(r"^(\w+ \w+) bags? contain ").unwrap();
        static ref RE_RHS: Regex = Regex::new(r"\d+ (\w+ \w+) bags?").unwrap();
    }
    let mut rules = HashMap::new();
    for line in input.lines() {
        let lhs_cap = RE_LHS.captures(line).unwrap();
        let outer_bag = &lhs_cap[1];
        for rhs_cap in RE_RHS.captures_iter(&line[lhs_cap[0].len()..]) {
            let inner_bag = rhs_cap[1].to_string();
            rules
                .entry(inner_bag)
                .or_insert_with(|| vec![])
                .push(outer_bag.to_string());
        }
    }
    rules
}

#[test]
fn part2_small() {
    let data = read_string("inputs/07.test");
    assert_eq!(part2(&data), 32);
}

#[test]
fn part2_secondary() {
    let data = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
    assert_eq!(part2(&data), 126);
}

fn part2(input: &str) -> i32 {
    let rules = parse_rules_out_to_in(&input);
    let mut memo = HashMap::new();
    count_bags("shiny gold", &rules, &mut memo) - 1
}

type OutToInRules = HashMap<String, Vec<(i32, String)>>;
type CountMap = HashMap<String, i32>;

fn count_bags(bag: &str, rules: &OutToInRules, mut memo: &mut CountMap) -> i32 {
    if let Some(count) = memo.get(bag) {
        return *count;
    }
    let mut c = 1;
    for (num, inner_bag) in rules.get(bag).unwrap() {
        c += num * count_bags(&inner_bag, &rules, &mut memo);
    }
    memo.insert(bag.to_string(), c);
    c
}

fn parse_rules_out_to_in(input: &str) -> OutToInRules {
    lazy_static! {
        static ref RE_LHS: Regex =
            Regex::new(r"^(\w+ \w+) bags? contain ").unwrap();
        static ref RE_RHS: Regex =
            Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();
    }
    let mut rules = HashMap::new();
    for line in input.lines() {
        let captures = RE_LHS.captures(line).unwrap();
        let outer_bag = captures[1].to_string();
        let rhs = &line[captures[0].len()..];
        let mut inner_bags = vec![];
        for cap in RE_RHS.captures_iter(rhs) {
            let num: i32 = cap[1].parse().unwrap();
            inner_bags.push((num, cap[2].to_string()));
        }
        rules.insert(outer_bag, inner_bags);
    }
    rules
}
