use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
pub enum InstructionParseError<E> {
    Format,
    Op(String),
    Value(E),
}

impl<E> fmt::Display for InstructionParseError<E>
where
    E: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Format => {
                write!(f, "InstructionParseError::Format")
            }
            Self::Op(s) => {
                write!(f, "InstructionParseError::Op({})", s)
            }
            Self::Value(v) => {
                write!(f, "InstructionParseError::Value({})", v)
            }
        }
    }
}

impl<E> Error for InstructionParseError<E> where E: Error {}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl FromStr for Instruction {
    type Err = InstructionParseError<ParseIntError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split(' ');
        let op = splits.next().ok_or(Self::Err::Format)?;
        let arg = splits.next().ok_or(Self::Err::Format)?;
        let instruction = match op {
            "acc" => Instruction::Acc(arg.parse().map_err(Self::Err::Value)?),
            "jmp" => Instruction::Jmp(arg.parse().map_err(Self::Err::Value)?),
            "nop" => Instruction::Nop(arg.parse().map_err(Self::Err::Value)?),
            _ => return Err(Self::Err::Op(op.to_string())),
        };

        Ok(instruction)
    }
}

#[derive(Debug)]
pub struct Processor<'a> {
    pub instructions: &'a [Instruction],
    pub acc: i32,
    pub ip: usize,
}

impl<'a> Processor<'a> {
    pub fn new(instructions: &'a [Instruction]) -> Processor {
        if instructions.is_empty() {}

        Processor {
            instructions: instructions,
            acc: 0,
            ip: 0,
        }
    }

    pub fn reset(&mut self) {
        self.ip = 0;
        self.acc = 0;
    }

    pub fn execute(&mut self) -> Result<Option<i32>, Box<dyn Error + Send + Sync>> {
        if self.ip == self.instructions.len() {
            Ok(Some(self.acc))
        } else {
            Ok(self.instructions.get(self.ip).and_then(|ins| {
                match *ins {
                    Instruction::Acc(n) => {
                        self.acc += n;
                        self.ip += 1;
                    }
                    Instruction::Jmp(n) => {
                        self.ip = usize::try_from((self.ip as i32) + n).ok()?;
                    }
                    Instruction::Nop(_n) => {
                        self.ip += 1;
                    }
                }

                None
            }))
        }
    }
}
