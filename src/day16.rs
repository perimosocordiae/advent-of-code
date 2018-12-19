use core::slice::Iter;
use std::num::ParseIntError;
use std::str::FromStr;

pub fn run() {
    let example = SampleInstruction {
        before: [3, 2, 1, 1],
        after: [3, 2, 2, 1],
        inst: "9 2 1 2".parse().unwrap(),
    };
    let matches = example.matching_opcodes();
    println!("{:?}", matches);
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
            // println!("{:?}: {:?} vs {:?}", op, result, self.after);
            if result == self.after {
                matches.push(op);
            }
        }
        matches
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

#[derive(Debug, Clone, Copy)]
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
