use crate::error::*;

pub type Offset = i32;
pub type Literal = i32;
pub type Immediate = i32;

pub struct Instruction(u64);

impl Instruction {
    pub fn from_u64(code: u64) -> Self {
        Self(code)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }

    fn extract_word(&self, idx: usize) -> u8 {
        (self.0 >> (idx * 8)) as u8
    }

    pub fn extract_opcode(&self) -> u8 {
        self.extract_word(7)
    }

    pub fn extract_immediate(&self) -> Immediate {
        (self.0 & 0xFFFFFFFF) as Immediate
    }
}

pub enum Operation {
    Add,
    PushLiteral { value: Literal }, // pushing i64 literals requires three commands:  https://github.com/Ismaxis/trombone-lang/pull/4#discussion_r2051408913
    Pop,
    Jump { offset: Offset },
    JumpIf { offset: Offset },
    JumpIfNot { offset: Offset },
}

impl TryFrom<Instruction> for Operation {
    fn try_from(value: Instruction) -> Result<Self> {
        // TODO: add checks for instructions that not uses immediate
        use crate::opcode::*;
        let x = match value.extract_opcode() {
            1 => Operation::Add,
            2 => Operation::PushLiteral {
                value: value.extract_immediate(),
            },
            3 => Operation::Pop,
            OP_JMP => Operation::Jump {
                offset: value.extract_immediate(),
            },
            OP_JMP_IF => Operation::JumpIf {
                offset: value.extract_immediate(),
            },
            OP_JMP_IF_NOT => Operation::JumpIfNot {
                offset: value.extract_immediate(),
            },
            opcode => return Err(format!("unknown opcode: {:x}", opcode).into()),
        };
        Ok(x)
    }

    type Error = Error;
}
