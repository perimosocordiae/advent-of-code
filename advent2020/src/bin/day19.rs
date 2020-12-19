use advent2020::*;
use std::collections::HashMap;

fn main() {
    let data = read_string("inputs/19.full");
    let (rules, messages) = parse_input(&data);
    println!("part1: {}", part1(&rules, &messages));
}

#[test]
fn part1_small() {
    let data = read_string("inputs/19.test");
    let (rules, messages) = parse_input(&data);
    assert_eq!(part1(&rules, &messages), 2);
}

fn part1(rules: &HashMap<usize, Rule>, messages: &str) -> usize {
    let start = rules.get(&0).unwrap();
    messages
        .lines()
        .filter(|msg| {
            let (b, extra) = matches_rule(&msg, &start, &rules);
            b && extra.is_empty()
        })
        .count()
}

fn matches_rule<'a>(
    msg: &'a str,
    start: &Rule,
    rules: &HashMap<usize, Rule>,
) -> (bool, &'a str) {
    if msg.is_empty() {
        return (false, msg);
    }
    if let Rule::Literal(ch) = start {
        return (msg.bytes().next().unwrap() == *ch, &msg[1..]);
    }
    if let Rule::Pattern(alts) = start {
        for seq in alts {
            let (b, extra) = matches_seq(&msg, &seq, &rules);
            if b {
                return (true, &extra);
            }
        }
    }
    (false, &msg)
}

fn matches_seq<'a>(
    msg: &'a str,
    seq: &[usize],
    rules: &HashMap<usize, Rule>,
) -> (bool, &'a str) {
    let mut rest = msg;
    for idx in seq {
        let r = rules.get(idx).unwrap();
        let (b, extra) = matches_rule(&rest, &r, &rules);
        if !b {
            return (false, &rest);
        }
        rest = extra;
    }
    (true, &rest)
}

#[derive(Debug)]
enum Rule {
    Literal(u8),
    Pattern(Vec<Vec<usize>>),
}

fn parse_input(data: &str) -> (HashMap<usize, Rule>, &str) {
    let mut chunk_iter = data.split("\n\n");
    let mut rules = HashMap::new();
    for line in chunk_iter.next().unwrap().lines() {
        let mut parts = line.split(": ");
        let idx: usize = parts.next().unwrap().parse().unwrap();
        let rhs = parts.next().unwrap();
        rules.insert(
            idx,
            if rhs.starts_with('"') {
                Rule::Literal(rhs.bytes().nth(1).unwrap())
            } else {
                Rule::Pattern(parse_pattern(&rhs))
            },
        );
    }
    let messages = chunk_iter.next().unwrap();
    (rules, messages)
}

fn parse_pattern(s: &str) -> Vec<Vec<usize>> {
    s.split(" | ")
        .map(|alt| {
            alt.split(' ')
                .map(|x| x.parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}
