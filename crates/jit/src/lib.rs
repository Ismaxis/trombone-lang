use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::values::{BasicValueEnum, IntValue, PointerValue};
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
    ) -> Option<JitFunction<VmExecuteFunc>> /* TODO: result */ {
        let module_name = format!("block_module_{}", block_id);
        let module = self.context.create_module(&module_name);

        let state_ptr_type = self.context.ptr_type(inkwell::AddressSpace::default());
        let ret_type = state_ptr_type;

        let fn_type = ret_type.fn_type(&[state_ptr_type.into()], false);
        let function = module.add_function(&module_name, fn_type, None);
        let entry_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_block);

        let rsp = function.get_nth_param(0)?.into_pointer_value();

        let result = self.compile_operations(operations, rsp)?;
        self.builder.build_return(Some(&result)).ok()?;

        function.verify(true).then_some(())?;

        self.execution_engine.add_module(&module).ok()?;
        eprintln!("{}", module.print_to_string());

        unsafe { self.execution_engine.get_function(&module_name).ok() }
    }

    fn compile_operations(
        &self,
        operations: &[Operation],
        rsp: PointerValue<'ctx>,
    ) -> Option<BasicValueEnum<'ctx>> /* TODO: result */ {
        let i64_type = self.context.i64_type();
        let stack_ptr = self
            .builder
            .build_alloca(rsp.get_type(), "stack_ptr_var")
            .ok()?;
        self.builder.build_store(stack_ptr, rsp).ok()?;

        for op in operations {
            let current_stack_ptr = self.get_current_stack_pointer(rsp, stack_ptr)?;
            match op {
                Operation::PushLiteral { value } => {
                    let const_val = i64_type.const_int(*value as u64, false);
                    self.builder
                        .build_store(current_stack_ptr, const_val)
                        .ok()?;

                    self.update_stack_pointer(1, stack_ptr, current_stack_ptr);
                }
                Operation::Pop => {
                    self.update_stack_pointer(-1, stack_ptr, current_stack_ptr)?;
                }
                Operation::LocalCopy { variable_offset } => {
                    let offset = -*variable_offset - 1;
                    let variable_ptr =
                        self.ptr_with_offset(offset as i64, "variable_to_copy", current_stack_ptr)?;
                    let variable_value = self
                        .builder
                        .build_load(variable_ptr.get_type(), variable_ptr, "value_to_copy")
                        .ok()?;
                    self.builder
                        .build_store(current_stack_ptr, variable_value)
                        .ok()?;
                    self.update_stack_pointer(1, stack_ptr, current_stack_ptr)?;
                }
                Operation::LocalStore { variable_offset } => {
                    // TODO: optimize
                    let offset = -*variable_offset - 1;
                    let moved_stack_ptr =
                        self.update_stack_pointer(-1, stack_ptr, current_stack_ptr)?;
                    let value = self
                        .builder
                        .build_load(
                            moved_stack_ptr.get_type(),
                            moved_stack_ptr,
                            "value_to_store",
                        )
                        .ok()?;
                    let variable_ptr =
                        self.ptr_with_offset(offset as i64, "variable_to_update", moved_stack_ptr)?;
                    self.builder.build_store(variable_ptr, value).ok()?;
                }

                // Arithmetic
                // ...

                // Comparison
                Operation::Equal => {
                    let lhs = self.stack_get(-1, current_stack_ptr)?;
                    let rhs = self.stack_get(-2, current_stack_ptr)?;

                    assert!(lhs.is_int_value()); // TODO: return Err
                    assert!(rhs.is_int_value()); // TODO: return Err

                    let eq = self
                        .builder
                        .build_int_compare(
                            inkwell::IntPredicate::EQ,
                            lhs.into_int_value(),
                            rhs.into_int_value(),
                            "eq_result",
                        )
                        .ok()?;

                    self.stack_put(-2, current_stack_ptr, eq)?;
                    self.update_stack_pointer(-1, stack_ptr, current_stack_ptr)?;
                }
                // ...

                // break on non-supported operations
                _ => {
                    return None;
                }
            }
        }

        let final_stack_ptr = self
            .builder
            .build_load(rsp.get_type(), stack_ptr, "final_stack_ptr")
            .ok()?;
        Some(final_stack_ptr)
    }

    fn stack_put(
        &self,
        offset: i64,
        current_stack_ptr: PointerValue<'ctx>,
        value: IntValue<'_>,
    ) -> Option<()> /* TODO: result */ {
        self.builder
            .build_store(
                self.ptr_with_offset(offset, "store_stack_ptr", current_stack_ptr)?,
                value,
            )
            .ok()
            .map(|_| ())
    }

    fn stack_get(
        &self,
        offset: i64,
        current_stack_ptr: PointerValue<'ctx>,
    ) -> Option<BasicValueEnum<'ctx>> /* TODO: result */ {
        self.builder
            .build_load(
                self.context.i64_type(),
                self.ptr_with_offset(offset, "load_stack_ptr", current_stack_ptr)?,
                "value_on_stack",
            )
            .ok()
    }

    fn get_current_stack_pointer(
        &self,
        rsp: PointerValue<'ctx>,
        stack_ptr: PointerValue<'ctx>,
    ) -> Option<PointerValue<'ctx>> /* TODO: result */ {
        let current_stack_ptr = self
            .builder
            .build_load(rsp.get_type(), stack_ptr, "current_stack_ptr")
            .ok()?
            .into_pointer_value();
        Some(current_stack_ptr)
    }

    fn update_stack_pointer(
        &self,
        offset: i64,
        stack_ptr: PointerValue<'ctx>,
        current_stack_ptr: PointerValue<'ctx>,
    ) -> Option<PointerValue<'ctx>> /* TODO: result */ {
        let new_stack_ptr = self.ptr_with_offset(offset, "new_stack_ptr", current_stack_ptr)?;
        self.builder
            .build_store(stack_ptr, new_stack_ptr)
            .ok()
            .map(|_| new_stack_ptr)
    }

    fn ptr_with_offset(
        &self,
        offset: i64,
        name: &str,
        current_ptr: PointerValue<'ctx>,
    ) -> Option<PointerValue<'ctx>> /* TODO: result */ {
        let i64_type = self.context.i64_type();
        let offset_const = if offset < 0 {
            i64_type.const_int((-offset) as u64, true).const_neg()
        } else {
            i64_type.const_int(offset as u64, false)
        };
        unsafe {
            self.builder
                .build_in_bounds_gep(i64_type, current_ptr, &[offset_const], name)
        }
        .ok()
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
    use trombone_common::TrombValue;

    // https://stackoverflow.com/a/52843365/17826620
    #[test]
    fn test_jit_push_pop() {
        let context = Context::create();
        let codegen = init(&context);

        let mut stack: [TrombValue; 16] = [TrombValue::default(); 16];
        let stack_base = stack.as_mut_ptr();

        // Push
        let push_literal_jit = codegen
            .jit_compile_basic_block(0, &[Operation::PushLiteral { value: 42 }])
            .expect("Failed to compile push function");
        {
            let modified_stack_ptr = unsafe { push_literal_jit.call(stack_base) };

            assert_eq!(modified_stack_ptr, offset_ptr(stack_base, 1));
            assert_eq!(stack[..1], [42]);
            assert_eq!(stack[1..], [0; 15]);
        }

        // Pop
        let pop_jit = codegen
            .jit_compile_basic_block(1, &[Operation::Pop])
            .expect("Failed to compile pop function");
        {
            let modified_stack_ptr = unsafe { pop_jit.call(offset_ptr(stack_base, 1)) };

            assert_eq!(modified_stack_ptr, stack_base);
        }
    }

    #[test]
    fn test_jit_local_variables() {
        let context = Context::create();
        let codegen = init(&context);

        let mut stack: [TrombValue; 16] = [TrombValue::default(); 16];
        let stack_base = stack.as_mut_ptr();

        // LocalCopy
        let localcopy_jit = codegen
            .jit_compile_basic_block(0, &[Operation::LocalCopy { variable_offset: 7 }; 3])
            .expect("Failed to compile local_copy function");
        {
            stack[0] = 42;
            stack[1] = 54;
            stack[2] = 68;

            let modified_stack_ptr = unsafe { localcopy_jit.call(offset_ptr(stack_base, 8)) };

            assert_eq!(modified_stack_ptr, offset_ptr(stack_base, 11));
            assert_eq!(stack[..11], [42, 54, 68, 0, 0, 0, 0, 0, 42, 54, 68]);
        }

        // LocalStore
        let localstore_jit = codegen
            .jit_compile_basic_block(1, &[Operation::LocalStore { variable_offset: 7 }; 3])
            .expect("Failed to compile local_store function");
        {
            stack[8] = 11;
            stack[9] = 12;
            stack[10] = 13;

            let modified_stack_ptr = unsafe { localstore_jit.call(offset_ptr(stack_base, 11)) };

            assert_eq!(modified_stack_ptr, offset_ptr(stack_base, 8));
            assert_eq!(stack[..8], [11, 12, 13, 0, 0, 0, 0, 0]);
        }
    }

    #[test]
    fn test_jit_advanced_stack_operations() {
        let context = Context::create();
        let codegen = init(&context);

        let localcopy_jit_zero = codegen
            .jit_compile_basic_block(0, &[Operation::LocalCopy { variable_offset: 0 }])
            .expect("Failed to compile local_copy function");
        let localstore_jit_zero = codegen
            .jit_compile_basic_block(1, &[Operation::LocalStore { variable_offset: 0 }])
            .expect("Failed to compile local_store function");

        let mut stack: [TrombValue; 16] = [TrombValue::default(); 16];
        let stack_base = stack.as_mut_ptr();

        {
            stack[0] = 42;

            let modified_stack_ptr = unsafe { localcopy_jit_zero.call(offset_ptr(stack_base, 1)) };

            assert_eq!(modified_stack_ptr, offset_ptr(stack_base, 2));
            assert_eq!(stack[..2], [42, 42]);
        }
        {
            stack[2] = 1000;

            let modified_stack_ptr = unsafe { localstore_jit_zero.call(offset_ptr(stack_base, 3)) };

            assert_eq!(modified_stack_ptr, offset_ptr(stack_base, 2));
            assert_eq!(stack[..2], [42, 1000]);
        }
    }

    #[test]
    fn test_jit_cmp() {
        let context = Context::create();
        let codegen = init(&context);

        let mut stack: [TrombValue; 16] = [TrombValue::default(); 16];
        let stack_base = stack.as_mut_ptr();

        let eq_jit = codegen
            .jit_compile_basic_block(0, &[Operation::Equal])
            .expect("Failed to compile equal function");

        let tests: &[(&[TrombValue], &[TrombValue])] = &[(&[5, 5], &[1]), (&[5, 3], &[0])];

        for (initial_stack, final_stack) in tests {
            for (i, v) in initial_stack.iter().enumerate() {
                stack[i] = *v;
            }

            let modified_stack_ptr = unsafe { eq_jit.call(offset_ptr(stack_base, 2)) };

            assert_eq!(modified_stack_ptr, offset_ptr(stack_base, 1));
            assert_eq!(stack[..final_stack.len()], **final_stack);
        }
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

    fn offset_ptr(ptr: *mut u64, offset: i64) -> *mut u64 {
        (ptr as i64 + offset * std::mem::size_of::<u64>() as i64) as *mut u64
    }
}
