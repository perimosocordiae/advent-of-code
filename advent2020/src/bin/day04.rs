use advent2020::*;
use regex::Regex;
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

fn main() {
    let input = read_string("inputs/04.full");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[test]
fn part1_small() {
    let data = read_string("inputs/04.test");
    assert_eq!(part1(&data), 2);
}

#[test]
fn part2_small() {
    let data =
        "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f";
    assert_eq!(part2(&data), 1);
}

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .filter(|&chunk| is_valid_one(&chunk))
        .count()
}

fn is_valid_one(chunk: &str) -> bool {
    chunk
        .split_whitespace()
        .filter(|&s| !s.starts_with("cid:"))
        .count()
        == 7
}

fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .filter(|&chunk| is_valid_two(&chunk))
        .count()
}

fn parse_chunk(chunk: &str) -> HashMap<String, String> {
    let mut fields = HashMap::new();
    for part in chunk.split_whitespace() {
        let mut parts = part.split(':');
        fields.insert(
            parts.next().unwrap().to_string(),
            parts.next().unwrap().to_string(),
        );
    }
    fields
}

fn is_valid_two(chunk: &str) -> bool {
    lazy_static! {
        static ref RE_HCL: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        static ref RE_ECL: Regex =
            Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        static ref RE_PID: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    }
    let fields = parse_chunk(&chunk);
    check_year(fields.get("byr"), 1920, 2002)
        && check_year(fields.get("iyr"), 2010, 2020)
        && check_year(fields.get("eyr"), 2020, 2030)
        && check_hgt(fields.get("hgt"))
        && check_regex(fields.get("hcl"), &RE_HCL)
        && check_regex(fields.get("ecl"), &RE_ECL)
        && check_regex(fields.get("pid"), &RE_PID)
}

fn check_year(x: Option<&String>, lb: usize, ub: usize) -> bool {
    if let Some(s) = x {
        if let Ok(year) = s.parse::<usize>() {
            return year >= lb && year <= ub;
        }
    }
    false
}

fn check_regex(x: Option<&String>, re: &Regex) -> bool {
    if let Some(s) = x {
        return re.is_match(s);
    }
    false
}

fn check_hgt(x: Option<&String>) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    }
    if let Some(s) = x {
        if let Some(caps) = RE.captures(s) {
            if let Ok(h) = caps[1].parse::<usize>() {
                if &caps[2] == "cm" {
                    return h >= 150 && h <= 193;
                } else {
                    return h >= 59 && h <= 76;
                }
            }
        }
    }
    false
}
