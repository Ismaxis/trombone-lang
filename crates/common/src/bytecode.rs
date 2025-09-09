use crate::{error::*, opcode::*};

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operation {
    PushLiteral { value: Literal }, // pushing i64 literals requires three commands:  https://github.com/Ismaxis/trombone-lang/pull/4#discussion_r2051408913
    Pop,
    LocalCopy { variable_offset: VariableOffset },
    LocalStore { variable_offset: VariableOffset },

    // Basic block
    BasicBlockStart { block_length: Offset },

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

    // Functions
    SetRetAddress { operands_count: Literal },
    Return,

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

    // Heap
    HeapAlloc,
    HeapPopPtr,
    HeapCopyPtr { variable_offset: VariableOffset },
    HeapLoad { variable_offset: VariableOffset },
    HeapStore { variable_offset: VariableOffset },

    // IO
    Read,
    Print,
}

impl Operation {
    // returns the difference of stack pointers before and after operation evaluation
    pub fn calc_stack_diff(&self) -> isize {
        let UNARY_OP = 0;
        let BINARY_OP = -1;
        match self {
            Operation::PushLiteral { .. } => 1,
            Operation::Pop => -1,
            Operation::LocalCopy { .. } => 1,
            Operation::LocalStore { .. } => -1,
            Operation::BasicBlockStart { .. } => 0,
            Operation::Neg => UNARY_OP,
            Operation::Not => UNARY_OP,
            Operation::Add => BINARY_OP,
            Operation::Sub => BINARY_OP,
            Operation::Mul => BINARY_OP,
            Operation::Div => BINARY_OP,
            Operation::Mod => BINARY_OP,
            Operation::And => BINARY_OP,
            Operation::Or => BINARY_OP,
            Operation::Xor => BINARY_OP,
            Operation::Lsh => BINARY_OP,
            Operation::Rsh => BINARY_OP,
            Operation::SetRetAddress { .. } => 0,
            Operation::Return => -1,
            Operation::Equal => BINARY_OP,
            Operation::NotEqual => BINARY_OP,
            Operation::LessThan => BINARY_OP,
            Operation::GreaterThan => BINARY_OP,
            Operation::LessThanOrEqual => BINARY_OP,
            Operation::GreaterThanOrEqual => BINARY_OP,
            Operation::Jump { .. } => 0,
            Operation::JumpIf { .. } => -1,
            Operation::JumpIfNot { .. } => -1,
            Operation::HeapAlloc => 0,
            Operation::HeapPopPtr => -1,
            Operation::HeapCopyPtr { .. } => 1,
            Operation::HeapLoad { .. } => 0,
            Operation::HeapStore { .. } => -2,
            Operation::Read => 1,
            Operation::Print => -1,
        }
    }
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

            // Basic block
            OP_BASICBLOCK_START => BasicBlockStart {
                block_length: value.extract_immediate(),
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

            // Function operations
            OP_SET_RET_ADDRESS => SetRetAddress {
                operands_count: value.extract_immediate(),
            },
            OP_RET => Return,

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

            // Heap operations
            OP_HEAP_ALLOC => HeapAlloc,
            OP_HEAP_POP_PTR => HeapPopPtr,
            OP_HEAP_COPY_PTR => HeapCopyPtr {
                variable_offset: value.extract_immediate(),
            },
            OP_HEAP_LOAD_PTR => HeapLoad {
                variable_offset: value.extract_immediate(),
            },
            OP_HEAP_STORE_PTR => HeapStore {
                variable_offset: value.extract_immediate(),
            },

            // IO operations
            OP_READ => Read,
            OP_PRINT => Print,

            // Unknown opcode
            opcode => return Err(format!("unknown opcode: {:x}", opcode).into()),
        };
        Ok(x)
    }

    type Error = Error;
}

impl From<Operation> for Instruction {
    fn from(value: Operation) -> Self {
        use Operation::*;
        match value {
            PushLiteral { value } => Instruction::from_parts(OP_PUSH, value),
            Pop => Instruction::from_parts(OP_POP, 0),
            LocalCopy { variable_offset } => {
                Instruction::from_parts(OP_LOCAL_COPY, variable_offset)
            }
            LocalStore { variable_offset } => {
                Instruction::from_parts(OP_LOCAL_STORE, variable_offset)
            }
            BasicBlockStart {
                block_length: block_type,
            } => Instruction::from_parts(OP_BASICBLOCK_START, block_type),
            Neg => Instruction::from_parts(OP_NEG, 0),
            Not => Instruction::from_parts(OP_NOT, 0),
            Add => Instruction::from_parts(OP_ADD, 0),
            Sub => Instruction::from_parts(OP_SUB, 0),
            Mul => Instruction::from_parts(OP_MUL, 0),
            Div => Instruction::from_parts(OP_DIV, 0),
            Mod => Instruction::from_parts(OP_MOD, 0),
            And => Instruction::from_parts(OP_AND, 0),
            Or => Instruction::from_parts(OP_OR, 0),
            Xor => Instruction::from_parts(OP_XOR, 0),
            Lsh => Instruction::from_parts(OP_LSH, 0),
            Rsh => Instruction::from_parts(OP_RSH, 0),
            SetRetAddress { operands_count } => {
                Instruction::from_parts(OP_SET_RET_ADDRESS, operands_count)
            }
            Return => Instruction::from_parts(OP_RET, 0),
            Equal => Instruction::from_parts(OP_EQ, 0),
            NotEqual => Instruction::from_parts(OP_NE, 0),
            LessThan => Instruction::from_parts(OP_LT, 0),
            GreaterThan => Instruction::from_parts(OP_GT, 0),
            LessThanOrEqual => Instruction::from_parts(OP_LE, 0),
            GreaterThanOrEqual => Instruction::from_parts(OP_GE, 0),
            Jump { offset } => Instruction::from_parts(OP_JMP, offset),
            JumpIf { offset } => Instruction::from_parts(OP_JMP_IF, offset),
            JumpIfNot { offset } => Instruction::from_parts(OP_JMP_IF_NOT, offset),
            HeapAlloc => Instruction::from_parts(OP_HEAP_ALLOC, 0),
            HeapPopPtr => Instruction::from_parts(OP_HEAP_POP_PTR, 0),
            HeapCopyPtr { variable_offset } => {
                Instruction::from_parts(OP_HEAP_COPY_PTR, variable_offset)
            }
            HeapLoad { variable_offset } => {
                Instruction::from_parts(OP_HEAP_LOAD_PTR, variable_offset)
            }
            HeapStore { variable_offset } => {
                Instruction::from_parts(OP_HEAP_STORE_PTR, variable_offset)
            }
            Read => Instruction::from_parts(OP_READ, 0),
            Print => Instruction::from_parts(OP_PRINT, 0),
        }
    }
}
