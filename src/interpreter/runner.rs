#![allow(dead_code)]

use crate::bytecode::Instruction;
use crate::bytecode::Operation;
use crate::error::*;

type TrombValue = i32; // DWORD
static STACK_SIZE: usize = 1024; // maybe should get it from environment, default should be 8Mb (as usual in Linux)
static WORD_SIZE: usize = size_of::<TrombValue>();

pub struct OperationStream {
    pub instructions: [u64; 1024], // TODO better types
    pub instruction_pointer: usize,
}

impl OperationStream {
    pub fn next_instruction(&mut self) -> Result<Operation> {
        let ip = self.instruction_pointer;
        self.instruction_pointer += 1;
        Instruction::from_u64(self.instructions[ip]).try_into()
    }

    fn switch_frame(&mut self, offset: i32) {
        self.instruction_pointer = ((self.instruction_pointer as i64) + offset as i64) as usize;
    }

    pub fn new(instructions: [u64; 1024]) -> Self {
        Self {
            instructions,
            instruction_pointer: 0,
        }
    }
}

// #[derive(Debug)]
pub struct Runner {
    pub stream: OperationStream,
    pub stack: [TrombValue; STACK_SIZE],
    pub sp: usize,
}

impl Runner {
    pub fn evaluate_next_instruction(&mut self) -> Result<()> {
        let operation = self.stream.next_instruction()?;
        match operation {
            Operation::Add => {
                let op1 = self.stack[self.sp - 1];
                let op2 = self.stack[self.sp - 2];
                self.stack[self.sp - 2] = op1 + op2;
                self.sp -= 1;
            }
            Operation::Pop => {
                self.sp -= 1;
            }
            Operation::PushLiteral { value } => {
                self.stack[self.sp] = value as TrombValue;
                self.sp += 1;
            }
            Operation::Jump { offset } => {
                self.stream.switch_frame(offset);
            }
        }
        Ok(())
    }

    pub fn new(stream: OperationStream) -> Self {
        Self {
            stream,
            stack: [0; STACK_SIZE],
            sp: 0,
        }
    }
}
