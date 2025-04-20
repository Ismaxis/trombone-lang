use crate::error::*;

pub struct Instruction(u64);

impl Instruction {
    pub fn as_u64(&self) -> u64 {
        self.0
    }

    fn extract_word(&self, idx: usize) -> u8 {
        (self.0 >> (idx * 8)) as u8
    }

    pub fn extract_opcode(&self) -> u8 {
        self.extract_word(7)
    }

    pub fn extract_immediate(&self) -> i32 {
        (self.0 & 0xFFFFFFFF) as i32
    }

    pub fn from_u64(code: u64) -> Self {
        Self(code)
    }
}

pub type Offset = i32;
pub type Literal = i32;

pub enum Operation {
    Add,
    PushLiteral { value: Literal }, // pushing i64 literals requires three commands:  https://github.com/Ismaxis/trombone-lang/pull/4#discussion_r2051408913
    Jump { offset: Offset },
    Pop,
}

impl TryFrom<Instruction> for Operation {
    fn try_from(value: Instruction) -> Result<Self> {
        // TODO: add checks for instructions that not uses immediate
        let x = match value.as_u64() {
            _ if value.extract_opcode() == 1 => Operation::Add,
            _ if value.extract_opcode() == 2 => Operation::PushLiteral {
                value: value.extract_immediate(),
            },
            _ if value.extract_opcode() == 3 => Operation::Pop,
            _ if value.extract_opcode() == 4 => Operation::Jump {
                offset: value.extract_immediate(),
            },
            _ => return Err(format!("{:x} is incorrect opcode", value.extract_opcode(),).into()),
        };
        return Ok(x);
    }

    type Error = Error;
}
