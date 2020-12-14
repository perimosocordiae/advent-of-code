use advent2020::*;
use itertools::Itertools;
use std::collections::HashMap;

#[macro_use]
extern crate scan_fmt;

fn main() {
    let prog = parse_program("inputs/14.full");
    println!("part1: {}", part1(&prog));
    println!("part2: {}", part2(&prog));
}

#[test]
fn part1_small() {
    let prog = parse_program("inputs/14.test");
    assert_eq!(part1(&prog), 165);
}

fn part1(prog: &[ProgramBlock]) -> usize {
    let mut ram = HashMap::<usize, usize>::new();
    for block in prog {
        for (addr, val) in block.assignments.iter() {
            ram.insert(*addr, block.update_value(*val));
        }
    }
    ram.values().sum()
}

#[test]
fn part2_small() {
    let prog = parse_program("inputs/14.test2");
    assert_eq!(part2(&prog), 208);
}

fn part2(prog: &[ProgramBlock]) -> usize {
    let mut ram = HashMap::<usize, usize>::new();
    for block in prog {
        for (addr, val) in block.assignments.iter() {
            block.expand_address(*addr, |a| {
                ram.insert(a, *val);
            });
        }
    }
    ram.values().sum()
}

#[derive(Debug)]
struct ProgramBlock {
    force_on: usize,
    force_off: usize,
    floating_bits: Vec<usize>,
    assignments: Vec<(usize, usize)>,
}

impl ProgramBlock {
    fn update_value(&self, val: usize) -> usize {
        let mut result = val;
        result |= self.force_on;
        result &= !self.force_off;
        result
    }
    fn expand_address(
        &self,
        addr: usize,
        mut callback: impl FnMut(usize) -> (),
    ) {
        let mut result = addr;
        result |= self.force_on as usize;
        for comb in self
            .floating_bits
            .iter()
            .map(|_| [false, true].iter())
            .multi_cartesian_product()
        {
            for (f, &b) in self.floating_bits.iter().zip(comb) {
                if b {
                    result |= f;
                } else {
                    result &= !f;
                }
            }
            callback(result);
        }
    }
}

fn parse_program(path: &str) -> Vec<ProgramBlock> {
    let data = read_string(path);
    let mut prog = vec![];
    for line in data.lines() {
        match &line[..4] {
            "mask" => {
                prog.push(parse_mask(&line[7..]));
            }
            "mem[" => {
                let (addr, val) =
                    scan_fmt!(&line, "mem[{d}] = {d}", usize, usize).unwrap();
                prog.last_mut().unwrap().assignments.push((addr, val));
            }
            _ => panic!("Unknown program line: {:?}", line),
        }
    }
    prog
}

fn parse_mask(mask: &str) -> ProgramBlock {
    let mut force_on = 0;
    let mut force_off = 0;
    let mut floating_bits = vec![];
    for (i, ch) in mask.chars().enumerate() {
        let shifted: usize = 1 << (35 - i);
        match ch {
            '1' => {
                force_on |= shifted;
            }
            '0' => {
                force_off |= shifted;
            }
            'X' => {
                floating_bits.push(shifted);
            }
            _ => panic!("unexpected mask char: {}", ch),
        }
    }
    ProgramBlock {
        force_on,
        force_off,
        floating_bits,
        assignments: vec![],
    }
}
