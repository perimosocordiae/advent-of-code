use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

pub fn run() {
    let (ip, program) = setup("inputs/19.txt");
    println!("Part 1: {}", part1(ip, &program));
    println!("Part 2: {}", part2());
}

fn part1(ip: usize, program: &[Instruction]) -> usize {
    // run_decoded_program(911)
    let mut state = [0usize; 6];
    run_program(ip, &program, &mut state);
    state[0]
}

fn part2() -> usize {
    run_decoded_program(10_551_311)
}

fn run_decoded_program(r1: usize) -> usize {
    // Static analysis of the input program revealed that it's summing all divisors
    // of a certain integer, which varies from part1 to part2, in a really inefficient way.
    // This translation reduces the time complexity from O(n^2) to O(n).
    let mut r0 = 0;
    for r2 in 1..=r1 {
        if r1 % r2 == 0 {
            r0 += r2;
        }
    }
    r0
}

fn run_program(ip: usize, program: &[Instruction], mut state: &mut [usize; 6]) {
    let prog_size = program.len();
    while state[ip] < prog_size {
        let inst = &program[state[ip]];
        inst.execute(&mut state);
        state[ip] += 1;
    }
}

fn setup(path: &str) -> (usize, Vec<Instruction>) {
    let data = fs::read_to_string(path).unwrap();
    let mut lines = data.lines();
    let ip = lines.next().unwrap()[4..].parse().unwrap();
    let mut program = vec![];
    for line in lines {
        if let Ok(inst) = line.parse() {
            program.push(inst);
        }
    }
    (ip, program)
}

#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    a: usize,
    b: usize,
    c: usize,
}

impl Instruction {
    fn execute(&self, state: &mut [usize; 6]) {
        state[self.c] = match self.opcode {
            Opcode::AddR => state[self.a] + state[self.b],
            Opcode::AddI => state[self.a] + self.b,
            Opcode::MulR => state[self.a] * state[self.b],
            Opcode::MulI => state[self.a] * self.b,
            Opcode::BanR => state[self.a] & state[self.b],
            Opcode::BanI => state[self.a] & self.b,
            Opcode::BorR => state[self.a] | state[self.b],
            Opcode::BorI => state[self.a] | self.b,
            Opcode::SetR => state[self.a],
            Opcode::SetI => self.a,
            Opcode::GtIR => (self.a > state[self.b]) as usize,
            Opcode::GtRI => (state[self.a] > self.b) as usize,
            Opcode::GtRR => (state[self.a] > state[self.b]) as usize,
            Opcode::EqIR => (self.a == state[self.b]) as usize,
            Opcode::EqRI => (state[self.a] == self.b) as usize,
            Opcode::EqRR => (state[self.a] == state[self.b]) as usize,
        };
    }
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        Ok(Instruction {
            opcode: parts[0].parse().unwrap(),
            a: parts[1].parse()?,
            b: parts[2].parse()?,
            c: parts[3].parse()?,
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
