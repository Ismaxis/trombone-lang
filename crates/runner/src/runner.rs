#![allow(dead_code)]

use trombone_common::TrombValue;
use trombone_common::bytecode::Instruction;
use trombone_common::bytecode::Operation;
use trombone_common::bytecode::VariableOffset;
use trombone_common::error::*;

const STACK_SIZE: usize = 1024; // maybe should get it from environment, default should be 8Mb (as usual in Linux)

pub trait OperationStream {
    fn next_instruction(&mut self) -> Result<Operation>;
    fn switch_frame(&mut self, offset: i32);
}

pub struct ArrayOperationStream<'a> {
    pub instructions: &'a [u64],
    pub instruction_pointer: usize,
}

impl<'a> ArrayOperationStream<'a> {
    pub fn new(instructions: &'a [u64]) -> Self {
        Self {
            instructions,
            instruction_pointer: 0,
        }
    }
}

impl<'a> OperationStream for ArrayOperationStream<'a> {
    fn next_instruction(&mut self) -> Result<Operation> {
        let ip = self.instruction_pointer;
        self.instruction_pointer += 1;
        Instruction::from_u64(self.instructions[ip]).try_into()
    }

    fn switch_frame(&mut self, offset: i32) {
        self.instruction_pointer = ((self.instruction_pointer as i64) + offset as i64) as usize;
    }
}

pub struct Runner<OpStream: OperationStream> {
    pub stream: OpStream,
    pub stack: [TrombValue; STACK_SIZE],
    pub sp: usize,
}

impl<OpStream: OperationStream> Runner<OpStream> {
    pub fn new(stream: OpStream) -> Self {
        Self {
            stream,
            stack: [0; STACK_SIZE],
            sp: 0,
        }
    }

    pub fn evaluate_next_instruction(&mut self) -> Result<()> {
        use Operation::*;
        match self.stream.next_instruction()? {
            // Stack operations
            PushLiteral { value } => self.push(value as TrombValue),
            Pop => {
                self.pop();
            }
            LocalCopy {
                variable_offset: variable,
            } => {
                let op = *self.get_variable(variable);
                self.push(op);
            }
            LocalStore {
                variable_offset: variable,
            } => {
                let value = self.pop();
                *self.get_variable(variable) = value;
            }
            // Arithmetic
            Neg => self.unary_op(|a| -(a as i64) as TrombValue),
            Not => self.unary_op(|a| !a),
            Add => self.binary_op(|a, b| a + b),
            Sub => self.binary_op(|a, b| a - b),
            Mul => self.binary_op(|a, b| a * b),
            Div => self.try_binary_op(|a, b| {
                if b == 0 {
                    Err("Zero division encountered".into())
                } else {
                    Ok(a / b)
                }
            })?,
            Mod => self.try_binary_op(|a, b| {
                if b == 0 {
                    Err("Zero division encountered".into())
                } else {
                    Ok(a % b)
                }
            })?,
            And => self.binary_op(|a, b| a & b),
            Or => self.binary_op(|a, b| a | b),
            Xor => self.binary_op(|a, b| a ^ b),
            Lsh => self.binary_op(|a, b| a << b),
            Rsh => self.binary_op(|a, b| a >> b),

            // Comparison
            Equal => self.comparison(|a, b| a == b),
            NotEqual => self.comparison(|a, b| a != b),
            LessThan => self.comparison(|a, b| a < b),
            GreaterThan => self.comparison(|a, b| a > b),
            LessThanOrEqual => self.comparison(|a, b| a <= b),
            GreaterThanOrEqual => self.comparison(|a, b| a >= b),

            // Jump
            Jump { offset } => self.stream.switch_frame(offset - 1),
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

    fn get_variable(&mut self, variable: VariableOffset) -> &mut TrombValue {
        &mut self.stack[self.sp - 1 - variable as usize]
    }

    fn unary_op<F>(&mut self, op: F)
    where
        F: FnOnce(TrombValue) -> TrombValue,
    {
        let oper = self.pop();
        self.push(op(oper));
    }

    fn binary_op<F>(&mut self, op: F)
    where
        F: FnOnce(TrombValue, TrombValue) -> TrombValue,
    {
        let op1 = self.pop();
        let op2 = self.pop();
        self.push(op(op2, op1));
    }

    fn try_binary_op<F>(&mut self, op: F) -> Result<()>
    where
        F: FnOnce(TrombValue, TrombValue) -> Result<TrombValue>,
    {
        let op1 = self.pop();
        let op2 = self.pop();
        self.push(op(op2, op1)?);
        Ok(())
    }

    fn comparison<F>(&mut self, op: F)
    where
        F: FnOnce(TrombValue, TrombValue) -> bool,
    {
        self.binary_op(|a, b| op(a, b) as TrombValue);
    }
}
