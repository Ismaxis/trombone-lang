use std::alloc::{GlobalAlloc, Layout};

use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::io::{Stdin, Stdout, Write};

use trombone_common::TrombValue;
use trombone_common::bytecode::Instruction;
use trombone_common::bytecode::Operation;
use trombone_common::bytecode::VariableOffset;
use trombone_common::error::*;
use trombone_jit::{CodeGenTrait, VmExecuteFunc};

use crate::control_block::ControlBlock;

const STACK_SIZE: usize = 1024; // maybe should get it from environment, default should be 8Mb (as usual in Linux)

pub trait OperationStream {
    fn next_instruction(&mut self) -> Result<Operation>;
    fn switch_frame(&mut self, offset: i32);
    fn get_instruction_pointer(&self) -> usize;
    fn get_instructions_len(&self) -> usize;
    fn get_next_n(&mut self, n: usize) -> Vec<Operation>;
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

impl OperationStream for ArrayOperationStream<'_> {
    fn next_instruction(&mut self) -> Result<Operation> {
        let ip = self.instruction_pointer;
        self.instruction_pointer += 1;
        Instruction::from_u64(self.instructions[ip]).try_into()
    }

    fn switch_frame(&mut self, offset: i32) {
        self.instruction_pointer = ((self.instruction_pointer as i64) + offset as i64) as usize;
    }

    fn get_instruction_pointer(&self) -> usize {
        self.instruction_pointer
    }

    fn get_instructions_len(&self) -> usize {
        self.instructions.len()
    }

    fn get_next_n(&mut self, n: usize) -> Vec<Operation> {
        self.instructions[self.instruction_pointer..]
            .iter()
            .take(n)
            .map(|&x| Instruction::from_u64(x).try_into().unwrap())
            .collect()
    }
}

pub struct Runner<'alloc, 'ctx, OpStream, IStream, OStream>
where
    OpStream: OperationStream,
    IStream: BufRead,
    OStream: Write,
{
    pub stream: OpStream,
    pub stack: [TrombValue; STACK_SIZE],
    pub sp: usize,

    pub allocator: &'alloc dyn GlobalAlloc,

    pub input: IStream,
    pub output: OStream,

    pub basic_block_stats: HashMap<usize, usize>,
    pub basic_block_jitted: HashMap<usize, VmExecuteFunc>,
    pub codegen: Option<(trombone_jit::CodeGen<'ctx>, Threshold)>,
}
type Threshold = usize;

impl<'alloc, 'ctx, OpStream> Runner<'alloc, 'ctx, OpStream, BufReader<Stdin>, Stdout>
where
    OpStream: OperationStream,
{
    pub fn new_with_defaults(stream: OpStream) -> Self {
        Self {
            stream,
            stack: [0; STACK_SIZE],
            sp: 0,
            input: BufReader::new(std::io::stdin()),
            output: std::io::stdout(),
            allocator: &std::alloc::System,
            basic_block_stats: HashMap::new(),
            basic_block_jitted: HashMap::new(),
            codegen: None,
        }
    }

    pub fn default_allocator() -> &'alloc dyn GlobalAlloc {
        &std::alloc::System
    }

    pub fn default_input() -> BufReader<Stdin> {
        BufReader::new(std::io::stdin())
    }

    pub fn default_output() -> Stdout {
        std::io::stdout()
    }
}

pub enum ReturnCode {
    Continue,
    Done,
}

impl<'alloc, 'ctx, OpStream, IStream, OStream> Runner<'alloc, 'ctx, OpStream, IStream, OStream>
where
    OpStream: OperationStream,
    IStream: BufRead,
    OStream: Write,
{
    pub fn new(
        stream: OpStream,
        input: IStream,
        output: OStream,
        allocator: &'alloc dyn GlobalAlloc,
    ) -> Self {
        Self {
            stream,
            stack: [0; STACK_SIZE],
            sp: 0,
            input,
            output,
            allocator,
            basic_block_stats: HashMap::new(),
            basic_block_jitted: HashMap::new(),
            codegen: None,
        }
    }

    pub fn set_codegen(&mut self, codegen: trombone_jit::CodeGen<'ctx>, threshold: Threshold) {
        self.codegen = Some((codegen, threshold));
    }

    pub fn evaluate_next_instruction(&mut self) -> Result<ReturnCode> {
        use Operation::*;
        match self.stream.next_instruction()? {
            // Stack operations
            PushLiteral { value } => self.push(value as TrombValue),
            Pop => {
                self.pop();
            }
            LocalCopy { variable_offset } => {
                let op = *self.get_variable(variable_offset);
                self.push(op);
            }
            LocalStore { variable_offset } => {
                let value = self.pop();
                *self.get_variable(variable_offset) = value;
            }

            // Basic block
            BasicBlockStart { block_length } => {
                let (codegen, threshold) = match self.codegen {
                    Some(ref codegen) => codegen,
                    None => return Ok(ReturnCode::Continue),
                };

                let current_ip = self.stream.get_instruction_pointer() - 1;
                // // If the block is already jitted, execute it
                if let Some(&compiled_func) = self
                    .basic_block_jitted
                    .get(&self.stream.get_instruction_pointer())
                {
                    let new_stack_ptr =
                        unsafe { compiled_func(self.stack.as_mut_ptr().offset(self.sp as isize)) };
                    if new_stack_ptr.is_null() {
                        return Err("JIT compiled function returned null pointer".into());
                    }

                    self.sp =
                        unsafe { new_stack_ptr.offset_from(self.stack.as_mut_ptr()) } as usize;
                    self.stream.switch_frame(block_length as i32);
                    return Ok(ReturnCode::Continue);
                }

                let cnt = self.basic_block_stats.entry(current_ip).or_insert(0);
                *cnt += 1;

                if *cnt == *threshold {
                    println!(
                        "Basic block at {} executed {} times, compiling...",
                        self.stream.get_instruction_pointer() - 1,
                        cnt
                    );

                    let ip = self.stream.get_instruction_pointer();
                    let operations = self.stream.get_next_n(block_length as usize);
                    let compiled_func = codegen.jit_compile_basic_block(ip, &operations).expect(
                        format!("JIT compilation failed for basic block at {}", ip).as_str(),
                    );

                    self.basic_block_jitted
                        .insert(ip, unsafe { compiled_func.as_raw() });
                }
            }

            // Arithmetic
            Neg => self.unary_op(|a| a.wrapping_neg()),
            Not => self.unary_op(|a| !a),
            Add => self.binary_op(|a, b| a.wrapping_add(b)),
            Sub => self.binary_op(|a, b| a.wrapping_sub(b)),
            Mul => self.binary_op(|a, b| a.wrapping_mul(b)),
            Div => self
                .try_binary_op(|a, b| a.checked_div(b).ok_or("Zero division encountered".into()))?,
            Mod => self
                .try_binary_op(|a, b| a.checked_rem(b).ok_or("Zero division encountered".into()))?,
            And => self.binary_op(|a, b| a & b),
            Or => self.binary_op(|a, b| a | b),
            Xor => self.binary_op(|a, b| a ^ b),
            Lsh => self.binary_op(|a, b| a.checked_shl(b as u32).unwrap_or(0)),
            Rsh => self.binary_op(|a, b| a.checked_shr(b as u32).unwrap_or(0)),

            PushRetAddress { operands_count } => {
                println!(
                    "ip: {}, opcount: {}",
                    self.stream.get_instruction_pointer(),
                    operands_count
                );
                self.push(
                    (self.stream.get_instruction_pointer() + operands_count as usize + 1)
                        as TrombValue,
                );
            }

            Return { return_value_size } => {
                if self.sp == 0 {
                    return Ok(ReturnCode::Done);
                }
                self.stack[self.sp - (return_value_size as usize + 1)..self.sp].rotate_left(1);
                let address = self.pop();
                self.stream
                    .switch_frame(address as i32 - self.stream.get_instruction_pointer() as i32);
            }

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

            // Heap
            HeapAlloc => {
                let size = self.pop() as usize;
                if size == 0 {
                    self.push(0);
                    return Ok(ReturnCode::Continue);
                }

                let ptr = self.allocate_heap_memory(size);
                if ptr.is_null() {
                    return Err("Heap allocation failed".into());
                }
                self.push(ptr as TrombValue);
            }
            HeapPopPtr => {
                let ptr = self.pop() as *mut TrombValue;
                if ptr.is_null() {
                    return Err("Null pointer dereference".into());
                }
                let control_block_ptr = ControlBlock::from_value_ptr(ptr);
                let control_block = Self::ptr_to_ref(control_block_ptr);
                if control_block.ref_count() == 0 {
                    unsafe {
                        self.allocator
                            .dealloc(control_block_ptr as *mut u8, (*control_block).layout())
                    };
                } else {
                    control_block.decrement_ref_count();
                }
            }
            HeapCopyPtr { variable_offset } => {
                let ptr = *self.get_variable(variable_offset);
                Self::ptr_to_ref(ControlBlock::from_value_ptr(ptr as *const TrombValue))
                    .increment_ref_count();
                self.push(ptr);
            }
            HeapLoad { variable_offset } => {
                // TODO: Maybe it is better to pass variable_offset ignoring offset values on stack?

                let ptr = self.get_pointer_from_variable(variable_offset);
                if ptr.is_null() {
                    return Err("Null pointer dereference".into());
                }

                let offset = self.pop();
                if offset < 0 {
                    return Err("Negative offset in heap load".into());
                }

                let ptr = unsafe { ptr.add(offset as usize) };
                let value = unsafe { *ptr };
                self.push(value);
            }
            HeapStore { variable_offset } => {
                // TODO: Maybe it is better to pass variable_offset ignoring offset and value values on stack?

                let ptr = self.get_pointer_from_variable(variable_offset);
                if ptr.is_null() {
                    return Err("Null pointer dereference".into());
                }

                let offset = self.pop();
                if offset < 0 {
                    return Err("Negative offset in heap store".into());
                }
                let value = self.pop();
                let ptr = unsafe { ptr.add(offset as usize) };
                unsafe {
                    *ptr = value;
                }
            }
            Read => {
                self.output.write_fmt(format_args!("> "))?;
                self.output.flush()?;
                let mut line = String::new();
                self.input.read_line(&mut line)?;
                if let Some(value) = atoi::atoi::<TrombValue>(line.as_bytes()) {
                    self.push(value);
                } else {
                    // TODO:
                    let _ = self.output.write("parsing error\n".as_bytes())?;
                }
            }
            Print => {
                let value = self.pop();
                self.output.write_fmt(format_args!("$$ {}\n", value))?;
            }
        }
        Ok(ReturnCode::Continue)
    }

    pub fn evaluate(&mut self) -> Result<()> {
        while self.stream.get_instruction_pointer() < self.stream.get_instructions_len() {
            // println!("IP: {}", self.stream.get_instruction_pointer());
            self.evaluate_next_instruction()?;
        }
        // println!("IP: {}", self.stream.get_instruction_pointer());

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

    fn get_pointer_from_variable(&mut self, variable_offset: i32) -> *mut TrombValue {
        let ptr = self.get_variable(variable_offset);
        *ptr as *mut TrombValue
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

    fn allocate_heap_memory(&self, len: usize) -> *mut TrombValue {
        if len == 0 {
            return std::ptr::null_mut();
        }
        let (layout, _offset) = Self::control_block_layout(len);
        let ptr = unsafe { self.allocator.alloc_zeroed(layout) };
        if ptr.is_null() {
            return std::ptr::null_mut();
        }

        let ptr = unsafe { ptr.add(std::mem::offset_of!(ControlBlock<[TrombValue; 1]>, value)) };
        let control_block = ControlBlock::from_value_ptr(ptr as *const TrombValue);
        // no ref_count initialization needed, ref_count is one less than the number of references
        unsafe { (*control_block).set_layout(layout) };
        ptr as *mut TrombValue
    }

    fn control_block_layout(len: usize) -> (Layout, usize) {
        let header = Layout::new::<usize>();
        let layout_field = Layout::new::<Layout>();
        let (header_layout, _layout_offset) = header.extend(layout_field).unwrap();

        let array = Layout::array::<TrombValue>(len).unwrap();
        let (full_layout, value_offset) = header_layout.extend(array).unwrap();
        (full_layout.pad_to_align(), value_offset)
    }

    fn ptr_to_ref<'a, T>(control_block_ptr: *mut T) -> &'a mut T {
        unsafe { &mut *control_block_ptr }
    }
}
