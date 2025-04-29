use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::values::{BasicValueEnum, PointerValue};
use trombone_common::bytecode::Operation;

type Rsp = *mut u64;
pub type VmExecuteFunc = unsafe extern "C" fn(Rsp) -> Rsp;

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

    pub fn jit_compile_basic_block(
        &self,
        block_id: usize,
        operations: &[Operation],
    ) -> Option<JitFunction<VmExecuteFunc>> {
        let module_name = format!("block_module_{}", block_id);
        let module = self.context.create_module(&module_name);

        let state_ptr_type = self.context.ptr_type(inkwell::AddressSpace::default());
        let ret_type = state_ptr_type;

        let fn_type = ret_type.fn_type(&[state_ptr_type.into()], false);
        let function = module.add_function(&module_name, fn_type, None);
        let entry_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_block);

        let rsp = function.get_nth_param(0)?.into_pointer_value();

        if let Some(result) = self.compile_operations(operations, rsp) {
            self.builder.build_return(Some(&result)).ok()?;
        } else {
            self.builder
                .build_return(Some(&ret_type.const_zero()))
                .ok()?;
        }

        function.verify(true).then(|| ())?;

        self.execution_engine.add_module(&module).ok()?;

        unsafe { self.execution_engine.get_function(&module_name).ok() }
    }

    fn compile_operations(
        &self,
        operations: &[Operation],
        rsp: PointerValue<'ctx>,
    ) -> Option<BasicValueEnum<'ctx>> {
        let i64_type = self.context.i64_type();
        let stack_ptr_alloca = self
            .builder
            .build_alloca(rsp.get_type(), "stack_ptr_var")
            .ok()?;
        self.builder.build_store(stack_ptr_alloca, rsp).ok()?;

        for op in operations {
            match op {
                Operation::PushLiteral { value } => {
                    let current_stack_ptr =
                        self.get_current_stack_pointer(rsp, stack_ptr_alloca)?;

                    let const_val = i64_type.const_int(*value as u64, false);
                    self.builder
                        .build_store(current_stack_ptr, const_val)
                        .ok()?;

                    self.update_stack_pointer(1, stack_ptr_alloca, current_stack_ptr);
                }
                Operation::Pop => {
                    let current_stack_ptr =
                        self.get_current_stack_pointer(rsp, stack_ptr_alloca)?;
                    self.update_stack_pointer(-1, stack_ptr_alloca, current_stack_ptr)?;
                }
                // break on non-supported operations
                _ => {
                    return None;
                }
            }
        }

        let final_stack_ptr = self
            .builder
            .build_load(rsp.get_type(), stack_ptr_alloca, "final_stack_ptr")
            .ok()?;
        Some(final_stack_ptr)
    }

    fn get_current_stack_pointer(
        &self,
        rsp: PointerValue<'ctx>,
        stack_ptr_alloca: PointerValue<'ctx>,
    ) -> Option<PointerValue<'_>> {
        let current_stack_ptr = self
            .builder
            .build_load(rsp.get_type(), stack_ptr_alloca, "current_stack_ptr")
            .ok()?
            .into_pointer_value();
        Some(current_stack_ptr)
    }

    fn update_stack_pointer(
        &self,
        offset: i64,
        stack_ptr_alloca: PointerValue<'ctx>,
        current_stack_ptr: PointerValue<'_>,
    ) -> Option<()> {
        let i64_type = self.context.i64_type();
        let offset_const = if offset < 0 {
            i64_type.const_int((-offset) as u64, true).const_neg()
        } else {
            i64_type.const_int(offset as u64, false)
        };
        let new_stack_ptr = unsafe {
            self.builder.build_in_bounds_gep(
                i64_type,
                current_stack_ptr,
                &[offset_const],
                "new_stack_ptr",
            )
        }
        .ok()?;
        self.builder
            .build_store(stack_ptr_alloca, new_stack_ptr)
            .ok()?;
        Some(())
    }

    #[allow(dead_code)]
    pub fn inspect_ir(&self) -> String {
        self.module.print_to_string().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::OptimizationLevel;

    // https://stackoverflow.com/a/52843365/17826620
    #[test]
    fn push_operation() {
        let context = Context::create();
        let codegen = init(&context);

        let push_literal_jit = codegen
            .jit_compile_basic_block(0, &[Operation::PushLiteral { value: 42 }])
            .expect("Failed to compile push function");

        let stack: [trombone_common::TrombValue; 16] = [trombone_common::TrombValue::default(); 16];
        let mut stack_ptr = stack.as_ptr() as *mut u64;

        unsafe {
            stack_ptr = push_literal_jit.call(stack_ptr);
        }
        assert_eq!(
            stack_ptr as usize - std::mem::size_of::<usize>(),
            stack.as_ptr() as usize
        );
        assert_eq!(stack[..1], [42]);
        assert_eq!(stack[1..], [0; 15]);
    }

    #[test]
    fn pop_operation() {
        let context = Context::create();
        let codegen = init(&context);

        let pop_jit = codegen
            .jit_compile_basic_block(0, &[Operation::Pop])
            .expect("Failed to compile pop function");

        let mut stack: [trombone_common::TrombValue; 16] =
            [trombone_common::TrombValue::default(); 16];
        stack[0] = 42;
        let mut stack_ptr = (stack.as_ptr() as usize + std::mem::size_of::<u64>()) as *mut u64;

        unsafe {
            stack_ptr = pop_jit.call(stack_ptr);
        }
        assert_eq!(stack_ptr as usize, stack.as_ptr() as usize);
    }

    #[test]
    fn test_jit_compile_multiple() {
        let context = Context::create();
        let codegen = init(&context);

        // First

        let push_jit = codegen
            .jit_compile_basic_block(0, &[Operation::PushLiteral { value: 42 }])
            .expect("Failed to compile push function");

        let stack: [trombone_common::TrombValue; 16] = [trombone_common::TrombValue::default(); 16];
        let mut stack_ptr = stack.as_ptr() as *mut u64;

        unsafe {
            stack_ptr = push_jit.call(stack_ptr);
        }
        assert_eq!(
            stack_ptr as usize - std::mem::size_of::<usize>(),
            stack.as_ptr() as usize
        );
        assert_eq!(stack[..1], [42]);
        assert_eq!(stack[1..], [0; 15]);

        // Second

        let pop_jit = codegen
            .jit_compile_basic_block(1, &[Operation::Pop])
            .expect("Failed to compile pop function");

        let mut stack: [trombone_common::TrombValue; 16] =
            [trombone_common::TrombValue::default(); 16];
        stack[0] = 42;
        let mut stack_ptr = (stack.as_ptr() as usize + std::mem::size_of::<u64>()) as *mut u64;

        unsafe {
            stack_ptr = pop_jit.call(stack_ptr);
        }
        assert_eq!(stack_ptr as usize, stack.as_ptr() as usize);
    }

    fn init(context: &Context) -> CodeGen {
        let module = context.create_module("unused_module");
        let builder = context.create_builder();
        let execution_engine = module
            .create_jit_execution_engine(OptimizationLevel::None)
            .expect("Failed to create JIT execution engine");

        let codegen = CodeGen::new(&context, module, builder, execution_engine);
        codegen
    }
}
