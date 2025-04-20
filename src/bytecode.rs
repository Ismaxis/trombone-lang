use crate::error::*;

pub type Offset = i32;
pub type Literal = i32;
pub type Immediate = i32;

pub struct Instruction(u64);

impl Instruction {
    pub fn from_u64(code: u64) -> Self {
        Self(code)
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

    #[cfg(test)]
    pub fn from_parts(opcode: u8, immediate: Immediate) -> Self {
        let code = ((opcode as u64) << 56) | ((immediate as u64) & 0xFFFFFFFF);
        Self(code)
    }

    #[cfg(test)]
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

pub enum Operation {
    Add,
    PushLiteral { value: Literal }, // pushing i64 literals requires three commands:  https://github.com/Ismaxis/trombone-lang/pull/4#discussion_r2051408913
    Pop,

    // Comparison
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,

    // Jump
    Jump { offset: Offset },
    JumpIf { offset: Offset },
    JumpIfNot { offset: Offset },
}

impl TryFrom<Instruction> for Operation {
    fn try_from(value: Instruction) -> Result<Self> {
        use crate::opcode::*;
        use Operation::*;
        let x = match value.extract_opcode() {
            1 => Add,
            2 => PushLiteral {
                value: value.extract_immediate(),
            },
            3 => Pop,

            // Comparison operations
            OP_EQ => Equal,
            OP_NE => NotEqual,
            OP_LT => LessThan,
            OP_GT => GreaterThan,
            OP_LE => LessThanOrEqual,
            OP_GE => GreaterThanOrEqual,

            // Jump operations
            OP_JMP => Jump {
                offset: value.extract_immediate(),
            },
            OP_JMP_IF => JumpIf {
                offset: value.extract_immediate(),
            },
            OP_JMP_IF_NOT => JumpIfNot {
                offset: value.extract_immediate(),
            },
            opcode => return Err(format!("unknown opcode: {:x}", opcode).into()),
        };
        Ok(x)
    }

    type Error = Error;
}
