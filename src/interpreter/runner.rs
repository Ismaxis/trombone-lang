#![allow(dead_code)]

use crate::bytecode::Instruction;
use crate::bytecode::Operation;
use crate::error::*;

type TrombValue = i32; // DWORD
const STACK_SIZE: usize = 1024; // maybe should get it from environment, default should be 8Mb (as usual in Linux)
const WORD_SIZE: usize = size_of::<TrombValue>();

pub struct OperationStream {
    pub instructions: [u64; 1024], // TODO better types
    pub instruction_pointer: usize,
}

impl OperationStream {
    pub fn new(instructions: [u64; 1024]) -> Self {
        Self {
            instructions,
            instruction_pointer: 0,
        }
    }

    fn next_instruction(&mut self) -> Result<Operation> {
        let ip = self.instruction_pointer;
        self.instruction_pointer += 1;
        Instruction::from_u64(self.instructions[ip]).try_into()
    }

    fn switch_frame(&mut self, offset: i32) {
        self.instruction_pointer = ((self.instruction_pointer as i64) + offset as i64) as usize;
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
        use Operation::*;
        match self.stream.next_instruction()? {
            Add => {
                let op1 = self.pop();
                let op2 = self.pop();
                self.push(op1 + op2);
            }
            PushLiteral { value } => {
                self.push(value);
            }
            Pop => {
                let _ = self.pop();
            }
            // Comparison
            Equal => {
                let op1 = self.pop();
                let op2 = self.pop();
                self.push(if op2 == op1 { 1 } else { 0 });
            }
            NotEqual => {
                let op1 = self.pop();
                let op2 = self.pop();
                self.push(if op2 != op1 { 1 } else { 0 });
            }
            LessThan => {
                let op1 = self.pop();
                let op2 = self.pop();
                self.push(if op2 < op1 { 1 } else { 0 });
            }
            GreaterThan => {
                let op1 = self.pop();
                let op2 = self.pop();
                self.push(if op2 > op1 { 1 } else { 0 });
            }
            LessThanOrEqual => {
                let op1 = self.pop();
                let op2 = self.pop();
                self.push(if op2 <= op1 { 1 } else { 0 });
            }
            GreaterThanOrEqual => {
                let op1 = self.pop();
                let op2 = self.pop();
                self.push(if op2 >= op1 { 1 } else { 0 });
            }
            // Jump
            Jump { offset } => {
                self.stream.switch_frame(offset - 1);
            }
            JumpIf { offset } => {
                if self.pop() != 0 {
                    self.stream.switch_frame(offset - 1);
                }
            }
            JumpIfNot { offset } => {
                if self.pop() == 0 {
                    self.stream.switch_frame(offset - 1);
                }
            }
        }
        Ok(())
    }

    pub fn push(&mut self, value: TrombValue) {
        self.stack[self.sp] = value;
        self.sp += 1;
    }

    fn pop(&mut self) -> TrombValue {
        self.sp -= 1;
        self.stack[self.sp]
    }

    pub fn new(stream: OperationStream) -> Self {
        Self {
            stream,
            stack: [0; STACK_SIZE],
            sp: 0,
        }
    }
}
