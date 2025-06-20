mod test;

pub use trombone_common::error::{Error, Result};

use byteorder::{ByteOrder, LittleEndian};
use clap::Parser;
use std::fs;
use trombone_common::{bytecode::Instruction, opcode};
use trombone_runner::runner::{self, ArrayOperationStream};

/// Virtual Machine for TromboneLang bytecode
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to input .trbc file
    path: String,
    //
    // TODO: debug flag, step by step execution
}

fn main() -> Result<()> {
    let args = Args::parse();

    let raw = fs::read(args.path).expect("can't read input file");
    assert_eq!(
        raw.len() % 8,
        0,
        "len of executable should be multiple of 8 bytes"
    );

    let mut instructions = vec![0u64; raw.len() / 8];
    LittleEndian::read_u64_into(&raw, instructions.as_mut());

    let stream = /* TODO: buffered stream */ ArrayOperationStream::new(instructions.as_ref());
    let mut runner = runner::Runner::new_with_defaults(stream);
    for _ in 0..instructions.len() {
        runner.evaluate_next_instruction()?;
    }
    println!("Execution completed!");
    println!("Stack: {:?}", &runner.stack[..runner.sp]);
    Ok(())
}

#[allow(dead_code)]
fn create_bytecode_file(path: &str) {
    let instructions = vec![
        Instruction::from_parts(opcode::OP_PUSH, 3).as_u64(),
        Instruction::from_parts(opcode::OP_PUSH, 2).as_u64(),
        Instruction::from_parts(opcode::OP_PUSH, 1).as_u64(),
        Instruction::from_parts(opcode::OP_ADD, 0x0).as_u64(),
        Instruction::from_parts(opcode::OP_ADD, 0x0).as_u64(),
    ];

    let mut raw = vec![0u8; instructions.len() * 8];
    LittleEndian::write_u64_into(&instructions, raw.as_mut());
    fs::write(path, raw).expect("can't write to file");
}
