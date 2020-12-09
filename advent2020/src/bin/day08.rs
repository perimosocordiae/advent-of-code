use advent2020::*;
use std::collections::HashSet;

fn main() {
    println!("part1: {}", part1("inputs/08.full"));
    println!("part2: {}", part2("inputs/08.full"));
}

#[test]
fn test_part1() {
    assert_eq!(part1("inputs/08.test"), 5);
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

#[test]
fn test_part2() {
    assert_eq!(part2("inputs/08.test"), 8);
}

fn part2(path: &str) -> i32 {
    let mut data = read_bootcode(path);
    for i in 0..data.len() {
        let flipped = flip_inst(data[i]);
        if flipped == data[i] {
            continue;
        }
        data[i] = flipped;
        if let Some(acc) = does_halt(&data) {
            return acc;
        }
        data[i] = flip_inst(flipped);
    }

    -1
}

fn flip_inst(inst: bootcode::Instruction) -> bootcode::Instruction {
    match &inst {
        bootcode::Instruction::Nop(x) => bootcode::Instruction::Jmp(*x),
        bootcode::Instruction::Jmp(x) => bootcode::Instruction::Nop(*x),
        x => *x,
    }
}

fn does_halt(code: &[bootcode::Instruction]) -> Option<i32> {
    let mut int = bootcode::Interpreter {
        instructions: code.to_vec(),
        ip: 0,
        acc: 0,
    };
    let mut seen = HashSet::new();
    seen.insert(int.ip);
    while int.step() {
        if !seen.insert(int.ip) {
            return None;
        }
    }
    Some(int.acc)
}
