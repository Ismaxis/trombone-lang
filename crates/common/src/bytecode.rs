use crate::error::*;

pub type Offset = i32;
pub type Literal = i32;
pub type Immediate = i32;
pub type VariableOffset = i32;

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
        (self.0 & Self::IMMEDIATE_MASK) as Immediate
    }

    pub const OPCODE_SHIFT: i32 = 56;
    pub const IMMEDIATE_MASK: u64 = 0xFFFFFFFF;

    #[allow(dead_code)]
    pub fn from_parts(opcode: u8, immediate: Immediate) -> Self {
        let code =
            ((opcode as u64) << Self::OPCODE_SHIFT) | ((immediate as u64) & Self::IMMEDIATE_MASK);
        Self(code)
    }

    #[allow(dead_code)]
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    PushLiteral { value: Literal }, // pushing i64 literals requires three commands:  https://github.com/Ismaxis/trombone-lang/pull/4#discussion_r2051408913
    Pop,
    LocalCopy { variable_offset: Literal },
    LocalStore { variable_offset: Literal },

    // Arithmetic
    Neg,
    Not,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Xor,
    Lsh,
    Rsh,

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
            OP_PUSH => PushLiteral {
                value: value.extract_immediate(),
            },
            OP_POP => Pop,
            OP_LOCAL_COPY => LocalCopy {
                variable_offset: value.extract_immediate(),
            },
            OP_LOCAL_STORE => LocalStore {
                variable_offset: value.extract_immediate(),
            },

            // Arithmetic operations
            OP_NEG => Neg,
            OP_NOT => Not,
            OP_ADD => Add,
            OP_SUB => Sub,
            OP_MUL => Mul,
            OP_DIV => Div,
            OP_MOD => Mod,
            OP_AND => And,
            OP_OR => Or,
            OP_XOR => Xor,
            OP_LSH => Lsh,
            OP_RSH => Rsh,

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
