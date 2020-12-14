use advent2020::*;
use std::collections::HashMap;

#[macro_use]
extern crate scan_fmt;

fn main() {
    let prog = parse_program("inputs/14.full");
    println!("part1: {}", part1(&prog));
}

#[test]
fn part1_small() {
    let prog = parse_program("inputs/14.test");
    assert_eq!(part1(&prog), 165);
}

fn part1(prog: &[ProgramBlock]) -> i64 {
    let mut ram = HashMap::<usize, i64>::new();
    for block in prog {
        for (addr, val) in block.assignments.iter() {
            ram.insert(*addr, block.update_value(*val));
        }
    }
    ram.values().sum()
}

#[derive(Debug)]
struct ProgramBlock {
    force_on: i64,
    force_off: i64,
    assignments: Vec<(usize, i64)>,
}

impl ProgramBlock {
    fn update_value(&self, val: i64) -> i64 {
        let mut result = val;
        result |= self.force_on;
        result &= !self.force_off;
        result
    }
}

fn parse_program(path: &str) -> Vec<ProgramBlock> {
    let data = read_string(path);
    let mut prog = vec![];
    for line in data.lines() {
        match &line[..4] {
            "mask" => {
                let (force_on, force_off) = parse_mask(&line[7..]);
                prog.push(ProgramBlock {
                    force_on,
                    force_off,
                    assignments: vec![],
                })
            }
            "mem[" => {
                let (addr, val) =
                    scan_fmt!(&line, "mem[{d}] = {d}", usize, i64).unwrap();
                prog.last_mut().unwrap().assignments.push((addr, val));
            }
            _ => panic!("Unknown program line: {:?}", line),
        }
    }
    prog
}

fn parse_mask(mask: &str) -> (i64, i64) {
    let mut force_on = 0;
    let mut force_off = 0;
    for (i, ch) in mask.chars().enumerate() {
        let shift = 35 - i;
        match ch {
            '1' => {
                force_on |= 1 << shift;
            }
            '0' => {
                force_off |= 1 << shift;
            }
            'X' => {}
            _ => panic!("unexpected mask char: {}", ch),
        }
    }
    (force_on, force_off)
}
