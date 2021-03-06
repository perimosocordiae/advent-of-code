use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl FromStr for Instruction {
    type Err = scan_fmt::parse::ScanError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (inst, arg) = scan_fmt!(&s, "{} {}", String, i32)?;
        match inst.as_str() {
            "nop" => Ok(Instruction::Nop(arg)),
            "acc" => Ok(Instruction::Acc(arg)),
            "jmp" => Ok(Instruction::Jmp(arg)),
            _ => panic!("invalid bootcode instruction"),
        }
    }
}

#[derive(Debug)]
pub struct Interpreter<'a> {
    instructions: &'a [Instruction],
    pub ip: i32,
    pub acc: i32,
}

impl Interpreter<'_> {
    pub fn new(code: &[Instruction]) -> Interpreter {
        Interpreter {
            instructions: code,
            ip: 0,
            acc: 0,
        }
    }

    pub fn step(&mut self) -> bool {
        self.ip += match self.instructions[self.ip as usize] {
            Instruction::Nop(_) => 1,
            Instruction::Acc(x) => {
                self.acc += x;
                1
            }
            Instruction::Jmp(x) => x,
        };
        self.ip != self.instructions.len() as i32
    }
}
