use advent2020::*;
use std::collections::HashSet;

fn main() {
    println!("part1: {:?}", part1("inputs/08.full"));
}

fn part1(path: &str) -> i32 {
    let data = read_bootcode(path);
    let mut int = bootcode::Interpreter {
        instructions: data,
        ip: 0,
        acc: 0,
    };
    let mut seen = HashSet::new();
    seen.insert(int.ip);
    while int.step() {
        if !seen.insert(int.ip) {
            break;
        }
    }
    int.acc
}