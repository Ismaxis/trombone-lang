use crate::error::*;
use std::io::Write;
use trombone_common::bytecode::*;

pub struct InstructionWriter<W: Write> {
    writer: W,
}

impl<W: Write> InstructionWriter<W> {
    pub fn write(&mut self, op: Operation) -> Result<usize> {
        self.writer
            .write(Instruction::from(op).as_u64().to_le_bytes().as_slice())
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }
}

pub type InMemoryWriter = InstructionWriter<Vec<u8>>;
pub type FileWriter = InstructionWriter<std::fs::File>;
