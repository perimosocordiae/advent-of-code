use advent2020::*;
use std::collections::HashMap;

#[macro_use]
extern crate scan_fmt;

fn main() {
    let notes = parse_notes("inputs/16.full");
    println!("part1: {}", part1(&notes));
    println!("part2: {}", part2(&notes, "departure "));
}

#[test]
fn part1_small() {
    let notes = parse_notes("inputs/16.test");
    assert_eq!(part1(&notes), 71);
}

fn part1(notes: &Notes) -> usize {
    notes
        .tickets
        .iter()
        .skip(1)
        .map(|t| sum_invalid_entries(t, &notes.rules))
        .sum()
}

#[test]
fn part2_small() {
    let notes = parse_notes("inputs/16.test2");
    assert_eq!(part2(&notes, "class"), 12);
    assert_eq!(part2(&notes, "row"), 11);
    assert_eq!(part2(&notes, "seat"), 13);
    assert_eq!(part2(&notes, ""), 11 * 12 * 13);
}

fn part2(notes: &Notes, prefix: &str) -> usize {
    // transpose valid tickets s.t. each position is on a row
    let mut valid_cols = vec![vec![]; notes.tickets[0].len()];
    for t in notes
        .tickets
        .iter()
        .skip(1)
        .filter(|t| sum_invalid_entries(t, &notes.rules) == 0)
    {
        for (i, x) in t.iter().enumerate() {
            valid_cols[i].push(x);
        }
    }
    // determine which positions work for which rules
    let mut assignments = HashMap::new();
    for (key, ranges) in notes.rules.iter() {
        for (i, nums) in valid_cols.iter().enumerate() {
            if nums
                .iter()
                .all(|x| ranges[0].contains(x) || ranges[1].contains(x))
            {
                assignments.entry(key).or_insert_with(Vec::new).push(i);
            }
        }
    }
    // assign one position per rule key
    let mut fixed_positions = HashMap::new();
    while !assignments.is_empty() {
        // positions are fixed when only one assignment is available
        for (key, positions) in assignments.iter() {
            if positions.len() == 1 {
                fixed_positions.insert(positions[0], key.to_string());
            }
        }
        // remove fixed positions from other rules' assignments
        for pos in fixed_positions.keys() {
            for positions in assignments.values_mut() {
                if let Some(idx) = positions.iter().position(|p| p == pos) {
                    positions.remove(idx);
                }
            }
        }
        // drop rules with no valid assignments (because they've been fixed)
        assignments.retain(|_, positions| !positions.is_empty());
    }
    // find the product of my ticket's values with the given rule prefix
    let my_ticket = &notes.tickets[0];
    fixed_positions
        .iter()
        .filter(|(_, key)| key.starts_with(prefix))
        .map(|(pos, _)| my_ticket[*pos])
        .product()
}

type RulesMap = HashMap<String, [std::ops::RangeInclusive<usize>; 2]>;

#[derive(Debug)]
struct Notes {
    rules: RulesMap,
    tickets: Vec<Vec<usize>>,
}

fn sum_invalid_entries(ticket: &[usize], rules: &RulesMap) -> usize {
    ticket.iter().filter(|&x| !any_rule_valid(*x, &rules)).sum()
}

fn any_rule_valid(x: usize, rules: &RulesMap) -> bool {
    rules
        .values()
        .any(|ranges| ranges[0].contains(&x) || ranges[1].contains(&x))
}

fn parse_notes(path: &str) -> Notes {
    let data = read_string(path);
    let mut chunks = data.split("\n\n");
    let rules = parse_rules(chunks.next().unwrap());
    let mut tickets = vec![];
    for chunk in chunks {
        for line in chunk.lines().skip(1) {
            tickets.push(parse_ticket(line));
        }
    }
    Notes { rules, tickets }
}

fn parse_ticket(ticket_data: &str) -> Vec<usize> {
    ticket_data.split(',').map(|x| x.parse().unwrap()).collect()
}

fn parse_rules(rules_data: &str) -> RulesMap {
    let mut out = HashMap::new();
    for line in rules_data.lines() {
        let mut parts = line.split(": ");
        let key = parts.next().unwrap().to_string();
        let (a, b, c, d) = scan_fmt!(
            parts.next().unwrap(),
            "{d}-{d} or {d}-{d}",
            usize,
            usize,
            usize,
            usize
        )
        .unwrap();
        out.insert(key, [a..=b, c..=d]);
    }
    out
}
