use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl FromStr for Instruction {
    type Err = Box<dyn std::error::Error>;

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
pub struct Interpreter {
    pub instructions: Vec<Instruction>,
    pub ip: i32,
    pub acc: i32,
}

impl Interpreter {
    pub fn step(&mut self) -> bool {
        let idx: usize = self.ip as usize;
        // println!(
        //     "ip={}, acc={}, inst={:?}",
        //     self.ip, self.acc, self.instructions[idx]
        // );
        match self.instructions[idx] {
            Instruction::Nop(_) => self.ip += 1,
            Instruction::Acc(x) => {
                self.acc += x;
                self.ip += 1
            }
            Instruction::Jmp(x) => {
                self.ip += x;
            }
        }
        self.ip != self.instructions.len() as i32
    }
}
