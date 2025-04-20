mod bytecode;
mod error;
mod interpreter;
mod jit;
mod opcode;

use interpreter::runner::{OperationStream, Runner};

pub use error::{Error, Result};
use inkwell::context::Context;
use inkwell::OptimizationLevel;

fn main() -> Result<()> {
    interpreter_jump_run()?;
    interpreter_run()?;
    jit_example()?;

    Ok(())
}

fn interpreter_run() -> Result<()> {
    // 1 + 2 = 3
    let mut instructions = [0u64; 1024];
    instructions[0] = (0x0002) << 56 | (0x00000001) << 0; // STORE 1
    instructions[1] = (0x0002) << 56 | (0x00000002) << 0; // STORE 2
    instructions[2] = (0x0001) << 56 | (0x0BADF00D) << 0; // ADD

    let stream = OperationStream::new(instructions);

    let mut runner = Runner::new(stream);

    runner.evaluate_next_instruction()?;
    runner.evaluate_next_instruction()?;
    runner.evaluate_next_instruction()?;

    assert_eq!(runner.sp, 1);
    assert_eq!(runner.stack[runner.sp - 1], 3);

    Ok(())
}

fn interpreter_cmp_run() -> Result<()> {
    // TODO: Implement a test for the comparison operations
    Ok(())
}

fn interpreter_jump_run() -> Result<()> {
    // Test for jump operations
    let mut instructions = [0u64; 1024];

    // Setup: Store values and then test jumps
    instructions[0] = (0x0002) << 56 | (0x00000001) << 0; // STORE 1
    instructions[1] = (0x0002) << 56 | (0x00000000) << 0; // STORE 0

    // Test unconditional jump (OP_JMP)
    // Jump to instruction 5, skipping the next instruction
    instructions[2] = (opcode::OP_JMP as u64) << 56 | (5 - 2);

    // This should be skipped
    instructions[3] = 0x0; // shouldn't execute
    instructions[4] = 0x0; // shouldn't execute

    // Test conditional jump (OP_JMP_IF)
    // Store 5, then jump to 8 if top of stack is non-zero
    instructions[5] = (0x0002) << 56 | (0x00000005) << 0; // STORE 5
    instructions[6] = (opcode::OP_JMP_IF as u64) << 56 | (9 - 6);

    // This should be skipped
    instructions[7] = 0x0; // shouldn't execute
    instructions[8] = 0x0; // shouldn't execute

    // Test conditional jump (OP_JMP_IF_NOT)
    // Store 0, then jump to 13 if top of stack is zero
    instructions[9] = (0x0002) << 56 | (0x00000000) << 0; // STORE 0
    instructions[10] = (opcode::OP_JMP_IF_NOT as u64) << 56 | (13 - 10);

    // This should be skipped
    instructions[11] = 0x0; // shouldn't execute
    instructions[12] = 0x0; // shouldn't execute

    // Final value to confirm we reached the end
    instructions[13] = (0x0002) << 56 | (0x0000000A) << 0; // STORE 10

    let stream = OperationStream::new(instructions);
    let mut runner = Runner::new(stream);

    // Execute all instructions until we reach the end
    for _ in 0..8 {
        // We expect 8 instructions to be executed
        runner.evaluate_next_instruction()?;
    }

    // Check final stack state
    // We expect: [1, 0, 10] (top)
    assert_eq!(runner.sp, 3);
    assert_eq!(runner.stack[0], 1);
    assert_eq!(runner.stack[1], 0);
    assert_eq!(runner.stack[2], 10);

    // Ensure we're at instruction 14 (after executing the last instruction)
    assert_eq!(runner.stream.instruction_pointer, 14);

    Ok(())
}

fn jit_example() -> Result<()> {
    let context = Context::create();
    let module = context.create_module("example_funcs");
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;

    let codegen = jit::CodeGen::new(&context, module, context.create_builder(), execution_engine);
    let inc = codegen
        .jit_compile_inc()
        .ok_or_else(|| "Unable to JIT compile `increment`".to_string())?;

    unsafe {
        let x = 42;
        println!("inc({}) = {}", x, inc.call(x));
        assert_eq!(inc.call(x), x + 1);
    }
    Ok(())
}
