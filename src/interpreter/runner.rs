#![allow(dead_code)]

use crate::bytecode::Instruction;
use crate::bytecode::Operation;
use crate::error::*;

type TrombValue = i32;

const STACK_SIZE: usize = 1024; // maybe should get it from environment, default should be 8Mb (as usual in Linux)

pub struct OperationStream<'a> {
    pub instructions: &'a [u64],
    pub instruction_pointer: usize,
}

impl<'a> OperationStream<'a> {
    pub fn new(instructions: &'a [u64]) -> Self {
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

pub struct Runner<'a> {
    pub stream: OperationStream<'a>,
    pub stack: [TrombValue; STACK_SIZE],
    pub sp: usize,
}

impl<'a> Runner<'a> {
    pub fn new(stream: OperationStream<'a>) -> Self {
        Self {
            stream,
            stack: [0; STACK_SIZE],
            sp: 0,
        }
    }

    pub fn evaluate_next_instruction(&mut self) -> Result<()> {
        use Operation::*;
        match self.stream.next_instruction()? {
            Add => {
                self.binary_op(|a, b| a + b);
            }
            PushLiteral { value } => {
                self.push(value);
            }
            Pop => {
                let _ = self.pop();
            }
            // Comparison
            Equal => {
                self.comparison(|a, b| a == b);
            }
            NotEqual => {
                self.comparison(|a, b| a != b);
            }
            LessThan => {
                self.comparison(|a, b| a < b);
            }
            GreaterThan => {
                self.comparison(|a, b| a > b);
            }
            LessThanOrEqual => {
                self.comparison(|a, b| a <= b);
            }
            GreaterThanOrEqual => {
                self.comparison(|a, b| a >= b);
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

    fn push(&mut self, value: TrombValue) {
        self.stack[self.sp] = value;
        self.sp += 1;
    }

    fn pop(&mut self) -> TrombValue {
        self.sp -= 1;
        self.stack[self.sp]
    }

    fn binary_op<F>(&mut self, op: F)
    where
        F: FnOnce(TrombValue, TrombValue) -> TrombValue,
    {
        let op1 = self.pop();
        let op2 = self.pop();
        self.push(op(op2, op1));
    }

    fn comparison<F>(&mut self, op: F)
    where
        F: FnOnce(TrombValue, TrombValue) -> bool,
    {
        self.binary_op(|a, b| op(a, b) as TrombValue);
    }
}
