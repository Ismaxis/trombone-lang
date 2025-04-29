use inkwell::OptimizationLevel;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::values::{BasicValueEnum, PointerValue};
use trombone_common::bytecode::Operation;

pub type IncrementFunc = unsafe extern "C" fn(i32) -> i32;
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

    pub fn jit_compile_basic_block(
        &self,
        block_id: usize,
        operations: &[Operation],
    ) -> Option<JitFunction<VmExecuteFunc>> {
        // Define VM state pointer type (u64*)
        let state_ptr_type = self.context.ptr_type(inkwell::AddressSpace::default());

        // Return type is (u64*)
        let ret_type = state_ptr_type;

        // Create function signature: (u64*) -> (u64*)
        let fn_type = ret_type.fn_type(&[state_ptr_type.into()], false);

        // Create function with unique name based on block ID
        let fn_name = format!("block_{}", block_id);
        let function = self.module.add_function(&fn_name, fn_type, None);

        // Create entry basic block
        let entry_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_block);

        // Get the VM state pointer argument
        let rsp = function.get_nth_param(0)?.into_pointer_value();

        // Compile each operation in the block
        if let Some(result) = self.compile_operations(operations, rsp) {
            // Return the result
            self.builder.build_return(Some(&result)).ok()?;
        } else {
            // Default return if no operations or compilation failed
            self.builder
                .build_return(Some(&ret_type.const_zero()))
                .ok()?;
        }

        // Verify the function for correctness
        function.verify(true).then(|| ())?;

        // Get compiled function
        unsafe { self.execution_engine.get_function(&fn_name).ok() }
    }

    fn compile_operations(
        &self,
        operations: &[Operation],
        rsp: PointerValue<'ctx>,
    ) -> Option<BasicValueEnum<'ctx>> {
        let i64_type = self.context.i64_type();

        // Create a mutable pointer to track our stack position
        let stack_ptr_alloca = self
            .builder
            .build_alloca(rsp.get_type(), "stack_ptr_var")
            .ok()?;

        // Initialize it with the input stack pointer
        self.builder.build_store(stack_ptr_alloca, rsp).ok()?;

        for op in operations {
            match op {
                Operation::PushLiteral { value } => {
                    // 1. Load current stack pointer
                    let current_stack_ptr =
                        self.get_current_stack_pointer(rsp, stack_ptr_alloca)?;

                    // 2. Store value at current stack position
                    let const_val = i64_type.const_int(*value as u64, false);
                    self.builder
                        .build_store(current_stack_ptr, const_val)
                        .ok()?;

                    // 3. Advance stack pointer
                    let new_stack_ptr = unsafe {
                        self.builder.build_in_bounds_gep(
                            i64_type,
                            current_stack_ptr,
                            &[i64_type.const_int(1, false)],
                            "new_stack_ptr",
                        )
                    }
                    .ok()?;

                    // 4. Save updated stack pointer
                    self.builder
                        .build_store(stack_ptr_alloca, new_stack_ptr)
                        .ok()?;
                }
                // Operation::Add => {}
                // Operation::Sub => {}
                // Operation::LocalStore { variable_offset } => {}
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

    #[allow(dead_code)]
    pub fn inspect_ir(&self) -> String {
        self.module.print_to_string().to_string()
    }
}

#[cfg(test)]
#[test]
fn test_jit_compile_inc() {
    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .expect("Failed to create JIT execution engine");

    let codegen = CodeGen::new(&context, module, builder, execution_engine);

    // TEST

    let push_literal_jit = codegen
        .jit_compile_basic_block(0, &[Operation::PushLiteral { value: 42 }])
        .expect("Failed to compile increment function");

    let stack: [trombone_common::TrombValue; 16] = [trombone_common::TrombValue::default(); 16];
    let mut stack_ptr = stack.as_ptr() as *mut u64;

    assert_eq!(stack, [0; 16]);
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
