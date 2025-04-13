mod error;
mod jit;

pub use error::{Error, Result};
use inkwell::context::Context;
use inkwell::OptimizationLevel;

fn main() -> Result<()> {
    jit_example()
}

fn jit_example() -> Result<()> {
    let context = Context::create();
    let module = context.create_module("example_funcs");
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::None)?;
    
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
