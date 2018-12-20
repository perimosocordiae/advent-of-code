use core::slice::Iter;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

pub fn run() {
    let (samples, program) = setup("inputs/16.txt");
    println!("Part 1: {}", part1(&samples));
    println!("Part 2: {}", part2(&samples, &program));
}

fn part1(samples: &[SampleInstruction]) -> usize {
    samples
        .iter()
        .map(|s| s.num_matching_opcodes())
        .filter(|&n| n >= 3)
        .count()
}

fn part2(samples: &[SampleInstruction], program: &[Instruction]) -> usize {
    let mut unknown_ids = HashMap::new();
    let mut known_ids = HashMap::new();
    for s in samples.iter() {
        let matches = s.matching_opcodes();
        if matches.len() == 1 {
            known_ids.insert(s.inst.opcode_id, matches[0]);
            continue;
        }
        let choices: HashSet<Opcode> = matches.into_iter().collect();
        unknown_ids.insert(s.inst.opcode_id, choices);
    }
    while !unknown_ids.is_empty() {
        for (id, choices) in unknown_ids.iter_mut() {
            for op in known_ids.values() {
                choices.remove(op);
            }
            if choices.len() == 1 {
                known_ids.insert(*id, choices.drain().next().unwrap());
            }
        }
        unknown_ids.retain(|_, v| v.len() > 0);
    }
    let mut state = [0usize; 4];
    for inst in program.iter() {
        let op = known_ids.get(&inst.opcode_id).unwrap();
        state = op.apply(inst, &state);
    }
    state[0]
}

fn setup(path: &str) -> (Vec<SampleInstruction>, Vec<Instruction>) {
    let data = fs::read_to_string(path).unwrap();
    let mut lines = data.lines();
    let mut samples = vec![];
    while let Some(line) = lines.next() {
        if !line.starts_with("Before: ") {
            break;
        }
        let before = parse_quad_array(line);
        let inst = lines.next().unwrap().parse().unwrap();
        let after = parse_quad_array(lines.next().unwrap());
        lines.next().unwrap(); // empty
        samples.push(SampleInstruction {
            before,
            after,
            inst,
        });
    }
    let mut program = vec![];
    while let Some(line) = lines.next() {
        if let Ok(inst) = line.parse() {
            program.push(inst);
        }
    }
    (samples, program)
}

fn parse_quad_array(s: &str) -> [usize; 4] {
    let a = s.find('[').unwrap() + 1;
    let b = s.rfind(']').unwrap();
    let parts: Vec<&str> = s[a..b].split(", ").collect();
    [
        parts[0].parse().unwrap(),
        parts[1].parse().unwrap(),
        parts[2].parse().unwrap(),
        parts[3].parse().unwrap(),
    ]
}

#[derive(Debug)]
struct SampleInstruction {
    before: [usize; 4],
    after: [usize; 4],
    inst: Instruction,
}

impl SampleInstruction {
    fn matching_opcodes(&self) -> Vec<Opcode> {
        let mut matches = vec![];
        for &op in Opcode::all_opcodes() {
            let result = op.apply(&self.inst, &self.before);
            if result == self.after {
                matches.push(op);
            }
        }
        matches
    }
    fn num_matching_opcodes(&self) -> usize {
        let mut count = 0usize;
        for &op in Opcode::all_opcodes() {
            let result = op.apply(&self.inst, &self.before);
            if result == self.after {
                count += 1;
            }
        }
        count
    }
}

#[derive(Debug)]
struct Instruction {
    opcode_id: u8,
    a: usize,
    b: usize,
    c: usize,
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        Ok(Instruction {
            opcode_id: parts[0].trim().parse()?,
            a: parts[1].trim().parse()?,
            b: parts[2].trim().parse()?,
            c: parts[3].trim().parse()?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Opcode {
    AddR,
    AddI,
    MulR,
    MulI,
    BanR,
    BanI,
    BorR,
    BorI,
    SetR,
    SetI,
    GtIR,
    GtRI,
    GtRR,
    EqIR,
    EqRI,
    EqRR,
}

impl Opcode {
    fn apply(self, inst: &Instruction, state: &[usize; 4]) -> [usize; 4] {
        let mut result = state.clone();
        result[inst.c] = match self {
            Opcode::AddR => state[inst.a] + state[inst.b],
            Opcode::AddI => state[inst.a] + inst.b,
            Opcode::MulR => state[inst.a] * state[inst.b],
            Opcode::MulI => state[inst.a] * inst.b,
            Opcode::BanR => state[inst.a] & state[inst.b],
            Opcode::BanI => state[inst.a] & inst.b,
            Opcode::BorR => state[inst.a] | state[inst.b],
            Opcode::BorI => state[inst.a] | inst.b,
            Opcode::SetR => state[inst.a],
            Opcode::SetI => inst.a,
            Opcode::GtIR => (inst.a > state[inst.b]) as usize,
            Opcode::GtRI => (state[inst.a] > inst.b) as usize,
            Opcode::GtRR => (state[inst.a] > state[inst.b]) as usize,
            Opcode::EqIR => (inst.a == state[inst.b]) as usize,
            Opcode::EqRI => (state[inst.a] == inst.b) as usize,
            Opcode::EqRR => (state[inst.a] == state[inst.b]) as usize,
        };
        result
    }
    fn all_opcodes() -> Iter<'static, Opcode> {
        static OPCODES: [Opcode; 16] = [
            Opcode::AddR,
            Opcode::AddI,
            Opcode::MulR,
            Opcode::MulI,
            Opcode::BanR,
            Opcode::BanI,
            Opcode::BorR,
            Opcode::BorI,
            Opcode::SetR,
            Opcode::SetI,
            Opcode::GtIR,
            Opcode::GtRI,
            Opcode::GtRR,
            Opcode::EqIR,
            Opcode::EqRI,
            Opcode::EqRR,
        ];
        OPCODES.into_iter()
    }
}

impl FromStr for Opcode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "addr" => Ok(Opcode::AddR),
            "addi" => Ok(Opcode::AddI),
            "mulr" => Ok(Opcode::MulR),
            "muli" => Ok(Opcode::MulI),
            "banr" => Ok(Opcode::BanR),
            "bani" => Ok(Opcode::BanI),
            "borr" => Ok(Opcode::BorR),
            "bori" => Ok(Opcode::BorI),
            "setr" => Ok(Opcode::SetR),
            "seti" => Ok(Opcode::SetI),
            "gtir" => Ok(Opcode::GtIR),
            "gtri" => Ok(Opcode::GtRI),
            "gtrr" => Ok(Opcode::GtRR),
            "eqir" => Ok(Opcode::EqIR),
            "eqri" => Ok(Opcode::EqRI),
            "eqrr" => Ok(Opcode::EqRR),
            _ => Err(()),
        }
    }
}
