mod bytecode;
mod error;
mod interpreter;
mod jit;

use bytecode::OpCode;
use interpreter::runner::Runner;

pub use error::{Error, Result};
use inkwell::context::Context;
use inkwell::OptimizationLevel;

fn main() -> Result<()> {
    jit_example().and(interpreter_run())
}

fn interpreter_run() -> Result<()> {
    let instructions = vec![OpCode::Add {
        dest: 2,
        src1: 0,
        src2: 1,
    }];

    let mut runner = Runner::new(instructions)?;

    runner.register[0] = 21;
    runner.register[1] = 23;

    runner.evaluate_next_instruction();

    println!(
        "register[0] = {}, register[1] = {}, register[2] = {}",
        runner.register[0], runner.register[1], runner.register[2]
    );
    assert_eq!(runner.register[2], 21 + 23);

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
