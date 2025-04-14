// use derive_more::derive::From;
// use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};

use crate::{bytecode::ArrayOpCode, Error};

// static STACK_SIZE: usize = 1024; // maybe should get it from environment, default should be 8Mb (as usual in Linux)
// static WORD_SIZE: usize = size_of::<u64>();

// #[derive(Debug)]
pub struct Runner {
    pub register: [u64; 256],
    pub instructions: ArrayOpCode,
    pub instruction_pointer: usize,
    // pub stack: *mut u64, // DWORD
}

impl Runner {
    pub fn new(instructions: ArrayOpCode) -> Result<Self, Error> {
        Ok(Self {
            register: [0; 256],
            instructions,
            instruction_pointer: 0,
        })
    }

    pub fn evaluate_next_instruction(&mut self) {
        let opcode = &self.instructions[self.instruction_pointer];
        match opcode {
            crate::bytecode::OpCode::Add { dest, src1, src2 } => {
                self.register[*dest as usize] =
                    self.register[*src1 as usize] + self.register[*src2 as usize];
            }
        };
        self.instruction_pointer += 1;
    }

    // pub fn new() -> Result<Self, Error> {
    //     let layout = Layout::from_size_align(STACK_SIZE / WORD_SIZE, WORD_SIZE)?;
    //     unsafe {
    //         let ptr = alloc(layout);
    //         if ptr.is_null() {
    //             handle_alloc_error(layout); // TODO
    //         }
    //         Ok(Self {
    //             register: [0; 256],
    //             stack: ptr as *mut u64,
    //         })
    //     }
    // }
}
