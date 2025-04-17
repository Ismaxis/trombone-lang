mod bytecode;
mod error;
mod interpreter;
mod jit;

use interpreter::runner::{OperationStream, Runner};

pub use error::{Error, Result};
use inkwell::context::Context;
use inkwell::OptimizationLevel;

fn main() -> Result<()> {
    jit_example().and(interpreter_run())
}

fn interpreter_run() -> Result<()> {
    // 1 + 2 = 3
    let mut instructions = [0u64; 1024];
    instructions[0] = (0x0002) << 48 | (0x00000001) << 0; // STORE 1
    instructions[1] = (0x0002) << 48 | (0x00000002) << 0; // STORE 2
    instructions[2] = (0x0001) << 48 | (0x0BADF00D) << 0; // ADD

    let stream = OperationStream::new(instructions);

    let mut runner = Runner::new(stream);

    runner.evaluate_next_instruction()?;
    runner.evaluate_next_instruction()?;
    runner.evaluate_next_instruction()?;

    assert_eq!(runner.sp, 1);
    assert_eq!(runner.stack[runner.sp - 1], 3);

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
