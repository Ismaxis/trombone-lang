use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;

pub type IncrementFunc = unsafe extern "C" fn(i32) -> i32;

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(
        context: &'ctx Context,
        module: Module<'ctx>,
        builder: Builder<'ctx>,
        execution_engine: ExecutionEngine<'ctx>,
    ) -> Self {
        Self {
            context,
            module,
            builder,
            execution_engine,
        }
    }

    pub fn jit_compile_inc(&self) -> Option<JitFunction<IncrementFunc>> {
        let i32_type = self.context.i32_type();
        let fn_type = i32_type.fn_type(&[i32_type.into()], false);
        let function = self.module.add_function("increment", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);

        let x = function.get_nth_param(0)?.into_int_value();
        let inc = self
            .builder
            .build_int_add(x, i32_type.const_int(1, false), "increment")
            .ok()?;
        self.builder.build_return(Some(&inc)).ok()?;

        unsafe { self.execution_engine.get_function("increment").ok() }
    }

    #[allow(dead_code)]
    pub fn inspect_ir(&self) -> String {
        self.module.print_to_string().to_string()
    }
}
